#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §enforcement-map — the kit→surface→class map emitted from the class registries; the emitted page owns the enforcement-class taxonomy
# usage: enforcement-map.sh [--emit]
#   bare: an advisory header plus the page; --emit: the page only, for the committed docs/enforcement.md projection.
#   advisory by construction: exit is always 0, never joins gates.list; the freshness gate (check-enforcement-fresh) blocks a stale emission.
set -uo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"

# spec: gate-sdk/SPEC.md §enforcement-map — every registry defaults to this repo's layout and reads through the owning kit's knob, so a consumer relocates each surface without touching the emitter
GATES_DIR="$(gate_sdk_gates_dir)"
LIST="$GATES_DIR/gates.list"
: "${DRIFT_KIT_KPIS_FILE:=$GATES_DIR/kpis.list}"
: "${CONTEXT_KIT_SETTINGS_FILE:=.claude/settings.json}"
: "${EVIDENCE_KIT_CONFIG_FILE:=$GATES_DIR/evidence-config.sh}"
: "${GATE_SDK_ENFORCE_SCAN_DIR:=.}"

emit=0
[[ "${1:-}" == "--emit" ]] && emit=1

# spec: gate-sdk/SPEC.md §enforcement-map — attribute a path to its owning kit label by the first kit segment (paths arrive absolute from gate_kit_roots or relative from a hook command); a surface under no kit groups as (consumer)
attribute_kit() {
    local seg
    local IFS=/
    local -a segs
    read -ra segs <<< "${1#./}"
    for seg in "${segs[@]}"; do
        case "$seg" in
            gate-sdk|*-kit) printf '%s\n' "$seg"; return 0 ;;
        esac
    done
    printf '%s\n' "(consumer)"
}

# spec: gate-sdk/SPEC.md §enforcement-map — the first slash-bearing token of a hook/suite command is the enforcing script, whose dir attributes the row
command_path() {
    local tok
    for tok in $1; do
        [[ "$tok" == */* ]] && { printf '%s\n' "$tok"; return 0; }
    done
    printf '%s\n' "$1"
}

mapfile -t CHECK_DIRS < <(gate_check_dirs)
mapfile -t KIT_ROOTS < <(gate_kit_roots)

# spec: gate-sdk/SPEC.md §enforcement-map — Blocking gates: gates.list membership, tier read from each gate's `# graph:` line, kit from the resolution dir; grouped by tier (precommit, commit-msg, align-only), gates.list order within a tier
gate_rows() {
    [[ -f "$LIST" ]] || return 0
    local -a pre=() msg=() align=()
    local c src tier kit row
    while IFS= read -r c; do
        src="$(gate_resolve "$c" "${CHECK_DIRS[@]}" 2>/dev/null || true)"
        [[ -n "$src" ]] || continue
        tier="$(sed -n 's/^# graph:.*[[:space:]]tier=\([a-z-]*\).*/\1/p' "$src" | head -1)"
        [[ -n "$tier" ]] || tier="?"
        kit="$(attribute_kit "$src")"
        row="| $kit | $c | $tier |"
        case "$tier" in
            precommit)   pre+=("$row") ;;
            commit-msg)  msg+=("$row") ;;
            *)           align+=("$row") ;;
        esac
    done < <(gates_list_members "$LIST")
    [[ ${#pre[@]} -eq 0 && ${#msg[@]} -eq 0 && ${#align[@]} -eq 0 ]] && return 0
    printf '## Blocking gates\n\n'
    printf '| kit | gate | tier |\n| --- | --- | --- |\n'
    printf '%s\n' "${pre[@]+"${pre[@]}"}" "${msg[@]+"${msg[@]}"}" "${align[@]+"${align[@]}"}"
    printf '\n'
}

# spec: gate-sdk/SPEC.md §enforcement-map — Advisory KPIs: the drift-kit KPI registry, kit from where each plugin resolves (scripts/ then each kit's kpis/)
kpi_rows() {
    [[ -f "$DRIFT_KIT_KPIS_FILE" ]] || return 0
    local -a rows=()
    local name src d
    while IFS= read -r name; do
        src=""
        [[ -f "$GATES_DIR/$name.sh" ]] && src="$GATES_DIR/$name.sh"
        if [[ -z "$src" ]]; then
            for d in "${KIT_ROOTS[@]}"; do
                [[ -f "$d/kpis/$name.sh" ]] && { src="$d/kpis/$name.sh"; break; }
            done
        fi
        [[ -n "$src" ]] || src="$GATES_DIR/$name.sh"
        rows+=("| $(attribute_kit "$src") | $name |")
    done < <(gates_list_members "$DRIFT_KIT_KPIS_FILE")
    [[ ${#rows[@]} -eq 0 ]] && return 0
    printf '## Advisory KPIs\n\n'
    printf '| kit | KPI |\n| --- | --- |\n'
    printf '%s\n' "${rows[@]}"
    printf '\n'
}

# spec: gate-sdk/SPEC.md §enforcement-map — Guards and Session warnings: PreToolUse / SessionStart command hooks in the tracked harness settings file, kit from the enforcing script's dir
hook_rows() {
    local kind="$1" jqpath="$2"
    command -v jq >/dev/null 2>&1 || return 0
    [[ -f "$CONTEXT_KIT_SETTINGS_FILE" ]] || return 0
    jq -e . "$CONTEXT_KIT_SETTINGS_FILE" >/dev/null 2>&1 || return 0
    local -a rows=()
    local line cmd matcher path
    while IFS=$'\t' read -r cmd matcher; do
        [[ -n "$cmd" ]] || continue
        path="$(command_path "$cmd")"
        if [[ "$kind" == guard ]]; then
            rows+=("| $(attribute_kit "$path") | $path | $matcher |")
        else
            rows+=("| $(attribute_kit "$path") | $path |")
        fi
    done < <(jq -r "$jqpath" "$CONTEXT_KIT_SETTINGS_FILE" 2>/dev/null)
    [[ ${#rows[@]} -eq 0 ]] && return 0
    if [[ "$kind" == guard ]]; then
        printf '## Guards\n\n'
        printf '| kit | surface | intercepts |\n| --- | --- | --- |\n'
    else
        printf '## Session warnings\n\n'
        printf '| kit | surface |\n| --- | --- |\n'
    fi
    printf '%s\n' "${rows[@]}"
    printf '\n'
}

# spec: gate-sdk/SPEC.md §enforcement-map — Validate suites: evidence-kit's suite registry (the config's suite list + per-suite run command), kit from the run command's script
suite_rows() {
    [[ -f "$EVIDENCE_KIT_CONFIG_FILE" ]] || return 0
    local -a suites=()
    local cmd var
    # shellcheck disable=SC1090  # consumer-supplied evidence config, path is config
    source "$EVIDENCE_KIT_CONFIG_FILE"
    declare -p EVIDENCE_KIT_SUITES &>/dev/null || return 0
    suites=("${EVIDENCE_KIT_SUITES[@]}")
    [[ ${#suites[@]} -eq 0 ]] && return 0
    printf '## Validate suites\n\n'
    printf '| kit | suite |\n| --- | --- |\n'
    local s
    for s in "${suites[@]}"; do
        var="EVIDENCE_KIT_RUN_$s"
        cmd="${!var-}"
        printf '| %s | %s |\n' "$(attribute_kit "$(command_path "$cmd")")" "$s"
    done
    printf '\n'
}

# spec: gate-sdk/SPEC.md §enforcement-map — Monitors: the one class with no parseable registry, so a non-gate surface declares itself with a line-start `# enforce: class=monitor <free-text>` marker the emitter greps; a marker is dormant in a template/fixture (an inert copy-source) and activates only where a consumer copies the file into a live path, so the walk prunes templates/ (grep -v, the sibling-finder idiom) atop the gate-tests exclusion GATE_GREP_EXCLUDES already carries; sorted for a stable projection
monitor_rows() {
    local -a rows=()
    local line file rest surface
    while IFS= read -r line; do
        [[ -n "$line" ]] || continue
        file="${line%%:*}"; rest="${line#*:}"; rest="${rest#*:}"
        surface="$(printf '%s' "$rest" | sed -E 's/^[[:space:]]*#[[:space:]]*enforce:[[:space:]]+class=monitor[[:space:]]+//; s/[[:space:]]+$//')"
        rows+=("| $(attribute_kit "$file") | $surface |")
    done < <(grep -rHnI "${GATE_GREP_EXCLUDES[@]}" -E '^[[:space:]]*#[[:space:]]*enforce:[[:space:]]+class=monitor[[:space:]]' "$GATE_SDK_ENFORCE_SCAN_DIR" 2>/dev/null | grep -v '/templates/' | sort)
    [[ ${#rows[@]} -eq 0 ]] && return 0
    printf '## Monitors\n\n'
    printf '| kit | surface |\n| --- | --- |\n'
    printf '%s\n' "${rows[@]}"
    printf '\n'
}

emit_page() {
    cat <<'PREAMBLE'
# Enforcement map

_Generated by `bash gate-sdk/bin/enforcement-map.sh --emit`; do not hand-edit —
`check-enforcement-fresh` byte-compares this page against the emitter._

Every governed surface in this repo is held by one enforcement class, ordered
here from hardest to softest. A **blocking gate** fails the commit (or, at the
`align-only` tier, the consistency audit) — the pre-commit hook is its local
reach, the CI workflow its server-side backstop. An **advisory KPI** never
blocks; it reports a drift trend into the session-context line. A **guard**
intercepts a tool call before it runs. A **session warning** surfaces context
when a session opens. A **validate suite** holds a test baseline that a per-run
evidence manifest attests. A **monitor** watches deployment truth rather than
tree truth, so it reds a scheduled run, never a merge.

The rows below derive from the class registries — the gate registry, the KPI
registry, the harness settings hooks, the evidence-suite config, and the
`# enforce:` markers a non-gate surface declares itself with — so this map
cannot drift from what actually runs. A registry a consumer has not adopted
leaves its section absent.

PREAMBLE
    gate_rows
    kpi_rows
    hook_rows guard '.hooks.PreToolUse // [] | .[] | (.matcher // "*") as $m | (.hooks // [])[] | select(.type == "command") | "\(.command)\t\($m)"'
    hook_rows warning '.hooks.SessionStart // [] | .[] | (.hooks // [])[] | select(.type == "command") | "\(.command)\t"'
    suite_rows
    monitor_rows
}

if [[ "$emit" -eq 0 ]]; then
    echo "=== Enforcement map (advisory — regenerate the committed page with --emit) ==="
    echo
fi
emit_page
exit 0
