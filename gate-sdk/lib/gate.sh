# shellcheck shell=bash
# spec: gate-sdk/SPEC.md §lib/gate.sh — sourced library: values + adapters, never gate structure

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
