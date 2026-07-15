# shellcheck shell=bash
# spec: gate-sdk/SPEC.md §lib/gate.sh — sourced library: values + adapters, never gate structure

# spec: gate-sdk/SPEC.md §Layout and configuration — auto-source the consumer config seam so a layout knob's override persists past the shell that set it; GATE_SDK_CONFIG_FILE wins, else <gates-dir>/gate-sdk-config.sh (GATE_SDK_GATES_DIR stays env-or-default — a config file cannot name its own directory)
_gate_sdk_config="${GATE_SDK_CONFIG_FILE:-}"
if [[ -n "$_gate_sdk_config" ]]; then
    [[ -f "$_gate_sdk_config" ]] || {
        echo "gate-sdk: GATE_SDK_CONFIG_FILE not found: $_gate_sdk_config" >&2
        exit 2
    }
    # shellcheck disable=SC1090  # consumer-supplied config, path is a knob
    source "$_gate_sdk_config"
else
    _gate_sdk_config="${GATE_SDK_GATES_DIR:-scripts}/gate-sdk-config.sh"
    if [[ -f "$_gate_sdk_config" ]]; then
        # shellcheck disable=SC1090  # consumer-supplied config, path is a knob
        source "$_gate_sdk_config"
    fi
fi
unset _gate_sdk_config

# spec: gate-sdk/SPEC.md §Fail-closed contract — non-zero capture status means the check could not run; exit 2, never a false clean
fail_closed() {
    if [[ "$1" -ne 0 ]]; then
        printf '%s: %s exited %s — the check could not run; treating as ' \
            "$2" "$3" "$1" >&2
        printf 'failure (not clean)\n' >&2
        exit 2
    fi
}

if [[ -n "${GATE_SDK_PRUNE_DIRS:-}" ]]; then
    read -r -a GATE_PRUNE_DIRS <<<"$GATE_SDK_PRUNE_DIRS"
else
    GATE_PRUNE_DIRS=(target .git node_modules .tmp gate-tests)
fi

gate_find() {
    local prune=() d
    for d in "${GATE_PRUNE_DIRS[@]}"; do prune+=(-name "$d" -o); done
    unset 'prune[${#prune[@]}-1]'
    find "$1" \( "${prune[@]}" \) -prune -o "${@:2}" -print
}

# shellcheck disable=SC2034  # consumed by sourcing gates, never within this lib
GATE_GREP_EXCLUDES=()
for _gpd in "${GATE_PRUNE_DIRS[@]}"; do GATE_GREP_EXCLUDES+=(--exclude-dir="$_gpd"); done
unset _gpd

gate_path_pruned() {
    local p="$1" d
    for d in "${GATE_PRUNE_DIRS[@]}"; do
        [[ "$p" == "$d/"* || "$p" == "./$d/"* || "$p" == */"$d"/* ]] && return 0
    done
    return 1
}

gate_sdk_root() {
    ( cd "${BASH_SOURCE[0]%/*}/.." && pwd )
}

gate_sdk_gates_dir() {
    printf '%s\n' "${GATE_SDK_GATES_DIR:-scripts}"
}

gates_list_members() {
    grep -Ev '^[[:space:]]*(#|$)' "$1" || true
}

gate_resolve() {
    local g="$1" d
    shift
    for d in "$@"; do
        if [[ -f "$d/$g.sh" ]]; then
            printf '%s\n' "$d/$g.sh"
            return 0
        fi
    done
    return 1
}

gate_kit_roots() {
    local d kit
    if [[ -n "${GATE_SDK_KIT_DIRS:-}" ]]; then
        for d in $GATE_SDK_KIT_DIRS; do printf '%s\n' "$d"; done
        return 0
    fi
    local sdk parent
    sdk="$(gate_sdk_root)"
    printf '%s\n' "$sdk"
    parent="${sdk%/*}"
    for d in "$parent"/*/; do
        kit="${d%/}"
        [[ "$kit" == "$sdk" ]] && continue
        [[ -d "$kit/checks" || -d "$kit/smoke" ]] || continue
        printf '%s\n' "$kit"
    done
    return 0
}

gate_check_dirs() {
    gate_sdk_gates_dir
    local k
    while IFS= read -r k; do
        printf '%s/checks\n' "$k"
    done < <(gate_kit_roots)
}

# spec: gate-sdk/SPEC.md §lib/gate.sh — the fixture-suite derivation shared by CI and evidence-kit's validate config: every dir with a gate-tests/ tree (the kit roots plus the gates dir), one tab-separated '<suite> <tests-dir> <checks-dir-or-empty>' row per suite in kit-roots-then-gates-dir order. suite = the dir basename with '-'→'_' (a valid var suffix + scenario name); checks-dir is the sibling checks/ when present, else empty so run-gate-tests falls back to consumer-first resolution. A new kit's gate-tests/ enrols with no hand-list to drift.
gate_fixture_suites() {
    local anchor base suite
    anchor="${GATE_SDK_ROOT:-$(gate_sdk_root)}"; anchor="${anchor%/*}"
    { gate_kit_roots_rel; gate_sdk_gates_dir; } | while IFS= read -r base; do
        base="${base%/}"
        [[ -d "$anchor/$base/gate-tests" ]] || continue
        suite="${base##*/}"; suite="${suite//-/_}"
        if [[ -d "$anchor/$base/checks" ]]; then
            printf '%s\t%s\t%s\n' "$suite" "$base/gate-tests" "$base/checks"
        else
            printf '%s\t%s\t\n' "$suite" "$base/gate-tests"
        fi
    done
}

# spec: gate-sdk/SPEC.md §lib/gate.sh — gate_kit_roots as repo-root-relative dirs (the anchor the couples globs share); absolute roots resolve against the kits' parent, relative roots (a GATE_SDK_KIT_DIRS override) pass through
gate_kit_roots_rel() {
    local anchor root
    anchor="${GATE_SDK_ROOT:-$(gate_sdk_root)}"; anchor="${anchor%/*}"
    while IFS= read -r root; do
        if [[ "$root" == /* ]]; then
            realpath --relative-to="$anchor" "$root" 2>/dev/null || printf '%s\n' "$root"
        else
            printf '%s\n' "$root"
        fi
    done < <(gate_kit_roots)
}

# spec: gate-sdk/SPEC.md §check-graph — expand each kit:<glob> token in a comma-joined couples/trigger field to <kit-root>/<glob> for every gate_kit_roots_rel member; non-kit tokens pass through verbatim. The single reader gen-pre-commit, check-graph, and the hook share, so emitter and checker cannot desync.
gate_expand_couples() {
    local field="$1"
    local -a roots=() parts=() out=()
    mapfile -t roots < <(gate_kit_roots_rel)
    IFS=',' read -ra parts <<<"$field"
    local tok r glob
    for tok in "${parts[@]}"; do
        if [[ "$tok" == kit:* ]]; then
            glob="${tok#kit:}"
            for r in "${roots[@]}"; do out+=("${r%/}/$glob"); done
        else
            out+=("$tok")
        fi
    done
    local IFS=','
    printf '%s\n' "${out[*]}"
}

# spec: gate-sdk/SPEC.md §The `# graph:` manifest — read one field from a resolved gate's `# graph:` line; the shared field reader gen-pre-commit and run-gates --for selection draw the manifest through (the couples-token expansion stays gate_expand_couples, the reader check-graph also shares). Emits the value, empty when the field is absent; never fails on a missing field.
gate_manifest_field() {
    local src="$1" key="$2" man kv
    man="$(grep -m1 '^# graph: ' "$src" 2>/dev/null || true)"
    for kv in ${man#\# graph: }; do
        [[ "$kv" == "$key="* ]] && { printf '%s' "${kv#"$key"=}"; return 0; }
    done
    return 0
}

# spec: gate-sdk/SPEC.md §run-gates — the path/glob matcher shared by run-gates --for selection and the emitted pre-commit hook: true when a path in the caller's staged_all array matches one of the given globs (bash glob, `*` spans '/'). gen-pre-commit emits this body verbatim into the hook's staged_matches; check-graph's freshness assertion holds the two in sync.
# shellcheck disable=SC2154  # staged_all is the caller's array: the hook's staged set, the selector's --for paths
gate_staged_matches() {
    local f pat
    for f in "${staged_all[@]}"; do
        for pat in "$@"; do
            # shellcheck disable=SC2053
            [[ "$f" == $pat ]] && return 0
        done
    done
    return 1
}

# spec: gate-sdk/SPEC.md §check-commit-msg — resolve the banned-pattern file set shared by check-commit-msg and check-tree-terms: explicit positional args win; otherwise GATE_SDK_MSG_PATTERN_FILES (tracked, must exist — fail-closed) plus GATE_SDK_MSG_PATTERN_FILES_LOCAL (gitignored, skipped when absent). Emits one existing readable file path per line; returns 2 when a required tracked file is missing.
gate_msg_pattern_files() {
    if [[ $# -gt 0 ]]; then
        printf '%s\n' "$@"
        return 0
    fi
    local f gd
    gd="$(gate_sdk_gates_dir)"
    for f in ${GATE_SDK_MSG_PATTERN_FILES:-$gd/msg-patterns.list}; do
        [[ -f "$f" ]] || { echo "gate_msg_pattern_files: required tracked pattern file missing: $f" >&2; return 2; }
        [[ -r "$f" ]] || { echo "gate_msg_pattern_files: pattern file not readable: $f" >&2; return 2; }
        printf '%s\n' "$f"
    done
    for f in ${GATE_SDK_MSG_PATTERN_FILES_LOCAL:-$gd/msg-patterns.local.list}; do
        [[ -f "$f" && -r "$f" ]] && printf '%s\n' "$f"
    done
    return 0
}

# spec: gate-sdk/SPEC.md §check-commit-subject — the single home of the commit-type roster (check-commit-subject's type alternation; trajectory.sh and kpi-task-split classify over the same tokens). Emits the space-separated roster on one line: GATE_SDK_COMMIT_TYPES when set, else the shipped default.
gate_commit_types() {
    printf '%s\n' "${GATE_SDK_COMMIT_TYPES:-feat fix refactor perf docs test build ci chore style}"
}

# spec: gate-sdk/SPEC.md §lib/gate.sh — the self-repo blob-link prefix `<identity>/blob/<ref>/`, shared by check-md-refs' resolver and the reference-link producers (the enforcement map) so an emitted link and the pass that validates it derive one identity. Identity comes from `git remote get-url origin`; the git@ and https remote forms normalize to one https identity, so no kit ships a repo name (the provenance seam holds). Empty output ⇒ no origin or an unrecognized remote form, and the caller skips the self-repo pass. The ref is the caller's policy arg, never a literal here.
gate_self_repo_prefix() {
    local ref="$1" origin id rest
    origin="$(git remote get-url origin 2>/dev/null)" || return 0
    [[ -n "$origin" ]] || return 0
    id="${origin%.git}"; id="${id%/}"
    case "$id" in
        git@*:*)  rest="${id#git@}"; id="https://${rest/:/\/}" ;;
        https://*|http://*) ;;
        *) return 0 ;;
    esac
    printf '%s/blob/%s/\n' "$id" "$ref"
}
