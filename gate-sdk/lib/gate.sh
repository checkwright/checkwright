# shellcheck shell=bash
# spec: gate-sdk/SPEC.md §lib/gate.sh — sourced library: values + adapters, never gate structure
#
# Extracted from the governance meta-layer of a private production platform;
# product-specific constants stayed behind — this file carries only the
# generic mechanism.

# fail_closed <status> <gate-name> <tool-name>
# Call right after capturing a parser's output: a non-zero subprocess status
# means the check could not run, and "could not run" must never read as clean.
fail_closed() {
    if [[ "$1" -ne 0 ]]; then
        printf '%s: %s exited %s — the check could not run; treating as ' \
            "$2" "$3" "$1" >&2
        printf 'failure (not clean)\n' >&2
        exit 2
    fi
}

# The whole-tree walk exclusion set. Override with GATE_SDK_PRUNE_DIRS
# (space-separated) — the defaults are the platform's originals.
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

# ---- registry + resolution (gate-sdk additions over the platform lib) -------

# gate_sdk_root — absolute path of the kit (the directory holding lib/, bin/,
# checks/, gate-tests/). Derived from this file's own location so it works from
# any cwd, including a fixture case dir.
gate_sdk_root() {
    ( cd "${BASH_SOURCE[0]%/*}/.." && pwd )
}

# gate_sdk_gates_dir — the consumer repo's gates directory (holds gates.list,
# the repo's own check-*.sh, and gate-tests/). Env-overridable; the default is
# the platform's original layout.
gate_sdk_gates_dir() {
    printf '%s\n' "${GATE_SDK_GATES_DIR:-scripts}"
}

# gates_list_members <gates.list> — print member names, one per line
# (comments and blank lines stripped). grep exits 1 on an all-comment file;
# callers treat an empty member set as the error, so status is not consulted.
gates_list_members() {
    grep -Ev '^[[:space:]]*(#|$)' "$1" || true
}

# gate_resolve <gate-name> <dir>... — print the first <dir>/<gate>.sh that
# exists; return 1 if the gate resolves nowhere.
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

# gate_kit_roots — the vendored kit roots, one per line, in resolution order:
# this kit (gate-sdk) first, then its sibling kits — any directory beside it
# holding a checks/ OR a smoke/ — sorted by name. A gateless kit (hooks/tools
# only, no checks/) is still discovered by its smoke/, so its smoke/install.sh
# runs under run-consumer-smoke.sh and its lib/ and bin/ are swept by
# check-shellcheck. Override with GATE_SDK_KIT_DIRS (space-separated paths,
# resolved from the caller's cwd — the entry points cd to the repo root first).
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

# gate_check_dirs — the registry member resolution path, one dir per line:
# the consumer gates dir, then each vendored kit's checks/.
gate_check_dirs() {
    gate_sdk_gates_dir
    local k
    while IFS= read -r k; do
        printf '%s/checks\n' "$k"
    done < <(gate_kit_roots)
}
