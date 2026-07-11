#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §run-gates — aggregate gate runner; --for scopes it to gates coupling to given paths
#
# usage: run-gates.sh [gates-dir]              run every registered gate
#        run-gates.sh --for <path>...          run only gates coupling to those paths
#   timings → $GATE_SDK_TMP_DIR/gate-timings.txt (default .tmp/); a measurement, never committed
set -uo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"

REPO_ROOT="$(git rev-parse --show-toplevel)" || {
    echo "run-gates: not inside a git repository" >&2
    exit 2
}
cd "$REPO_ROOT" || exit 2

FOR_PATHS=()
if [[ "${1:-}" == --for ]]; then
    shift
    FOR_PATHS=("$@")
    [[ ${#FOR_PATHS[@]} -gt 0 ]] || { echo "run-gates: --for needs at least one path" >&2; exit 2; }
    set --
fi

GATES_DIR="${1:-$(gate_sdk_gates_dir)}"
LIST="$GATES_DIR/gates.list"
[[ -f "$LIST" ]] || { echo "run-gates: no registry at $LIST" >&2; exit 2; }

mapfile -t MEMBERS < <(gates_list_members "$LIST")
[[ ${#MEMBERS[@]} -gt 0 ]] || { echo "run-gates: $LIST names no gates" >&2; exit 2; }

RESOLVE_DIRS=("$GATES_DIR")
while IFS= read -r k; do RESOLVE_DIRS+=("$k/checks"); done < <(gate_kit_roots)

# spec: gate-sdk/SPEC.md §run-gates — --for selection: hook-identical match per member; RUN_LIST + RUN_ARGSTR (index-aligned, newline-joined staged-mode args) carry the result
RUN_LIST=()
RUN_ARGSTR=()

pathspec_matches() {
    local p="$1" g; shift
    for g in "$@"; do
        # shellcheck disable=SC2053
        { [[ "$p" == $g ]] || [[ "$p" == $g/* ]]; } && return 0
    done
    return 1
}

select_for() {
    local c src couples trigger mode p astr
    local -a globs matched staged_all
    local -A path_covered=()
    local covered
    for p in "${FOR_PATHS[@]}"; do path_covered["$p"]=0; done
    for c in "${MEMBERS[@]}"; do
        src="$(gate_resolve "$c" "${RESOLVE_DIRS[@]}")" || {
            echo "run-gates: --for cannot resolve '$c' in: ${RESOLVE_DIRS[*]}" >&2
            exit 2
        }
        couples="$(gate_manifest_field "$src" couples)"
        trigger="$(gate_manifest_field "$src" trigger)"; trigger="${trigger:-$couples}"
        trigger="$(gate_expand_couples "$trigger")"
        mode="$(gate_manifest_field "$src" mode)"
        IFS=',' read -ra globs <<<"$trigger"
        if [[ "$trigger" == '*' ]]; then
            for p in "${FOR_PATHS[@]}"; do path_covered["$p"]=1; done
            RUN_LIST+=("$c"); RUN_ARGSTR+=("")
            continue
        fi
        if [[ "$mode" == staged ]]; then
            matched=()
            for p in "${FOR_PATHS[@]}"; do
                if pathspec_matches "$p" "${globs[@]}"; then matched+=("$p"); path_covered["$p"]=1; fi
            done
            if [[ ${#matched[@]} -gt 0 ]]; then
                printf -v astr '%s\n' "${matched[@]}"; astr="${astr%$'\n'}"
                RUN_LIST+=("$c"); RUN_ARGSTR+=("$astr")
            fi
            continue
        fi
        covered=0
        for p in "${FOR_PATHS[@]}"; do
            # shellcheck disable=SC2034  # gate_staged_matches (sourced) reads staged_all
            staged_all=("$p")
            if gate_staged_matches "${globs[@]}"; then covered=1; path_covered["$p"]=1; fi
        done
        (( covered )) && { RUN_LIST+=("$c"); RUN_ARGSTR+=(""); }
    done
    for p in "${FOR_PATHS[@]}"; do
        (( path_covered["$p"] )) || echo "run-gates: no registered gate couples to $p"
    done
}

RUN_MEMBERS=("${MEMBERS[@]}")
if [[ ${#FOR_PATHS[@]} -gt 0 ]]; then
    select_for
    RUN_MEMBERS=("${RUN_LIST[@]}")
    if [[ ${#RUN_MEMBERS[@]} -eq 0 ]]; then
        printf '\n===== gates summary =====\nno coupled gate for the given path(s); nothing to run.\n'
        exit 0
    fi
fi

failed=()
TIMINGS="${GATE_SDK_TMP_DIR:-.tmp}/gate-timings.txt"
mkdir -p "$(dirname "$TIMINGS")" && : > "$TIMINGS"
total_ms=0
for i in "${!RUN_MEMBERS[@]}"; do
    c="${RUN_MEMBERS[$i]}"
    args=()
    if [[ ${#FOR_PATHS[@]} -gt 0 && -n "${RUN_ARGSTR[$i]}" ]]; then
        mapfile -t args <<<"${RUN_ARGSTR[$i]}"
    fi
    printf '\n===== %s =====\n' "$c"
    start_ns=$(date +%s%N)
    if gate_path="$(gate_resolve "$c" "${RESOLVE_DIRS[@]}")"; then
        if "$gate_path" "${args[@]}"; then
            printf '  PASS: %s\n' "$c"
        else
            rc=$?
            printf '  FAIL: %s (exit %d)\n' "$c" "$rc"
            failed+=("$c")
        fi
    else
        printf '  FAIL: %s (listed in %s but resolves in none of: %s)\n' \
            "$c" "$LIST" "${RESOLVE_DIRS[*]}"
        failed+=("$c")
    fi
    elapsed_ms=$(( ($(date +%s%N) - start_ns) / 1000000 ))
    printf '%s %d\n' "$c" "$elapsed_ms" >> "$TIMINGS"
    total_ms=$(( total_ms + elapsed_ms ))
done
printf 'TOTAL %d\n' "$total_ms" >> "$TIMINGS"

printf '\n===== gates summary =====\n'
if [[ ${#failed[@]} -eq 0 ]]; then
    printf 'All %d gates passed.\n' "${#RUN_MEMBERS[@]}"
    exit 0
fi
printf '%d of %d gates FAILED: %s\n' "${#failed[@]}" "${#RUN_MEMBERS[@]}" "${failed[*]}"
exit 1
