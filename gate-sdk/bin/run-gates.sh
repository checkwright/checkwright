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

# spec: gate-sdk/SPEC.md §The non-gate arm — the emitter front-end: a ported arm receives no
# configuration, so a caller already sourcing this library resolves its bridged knobs and invokes
# it. The arm name is an operand here and a suffix in the crate, governed separately.
if [[ "${1:-}" == --emit ]]; then
    shift
    EMIT_ARM_NAME="${1:-}"
    [[ -n "$EMIT_ARM_NAME" ]] || { echo "run-gates: --emit needs an arm name" >&2; exit 2; }
    EMIT_ARM="--emit-$EMIT_ARM_NAME"
    shift
    EMIT_ARGS=("$@")
    EMIT_BIN="$(gate_native_bin)"
    if [[ ! -x "$EMIT_BIN" ]]; then
        printf 'run-gates: --emit dispatches to the native binary, but %s is absent or not ' "$EMIT_BIN" >&2
        printf 'executable — the projection could not be emitted. Build it: bash gate-sdk/bin/build-native.sh\n' >&2
        exit 2
    fi
    EMIT_ENV="$(gate_knob_env "$EMIT_ARM")" || exit 2
    EMIT_ELEMS=()
    [[ -n "$EMIT_ENV" ]] && mapfile -t EMIT_ELEMS <<<"$EMIT_ENV"
    exec env ${EMIT_ELEMS[@]+"${EMIT_ELEMS[@]}"} "$EMIT_BIN" "$EMIT_ARM" ${EMIT_ARGS[@]+"${EMIT_ARGS[@]}"}
fi

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
        gate_expand_couples_var trigger "$trigger"
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
# spec: gate-sdk/SPEC.md §run-gates — where gate_command's stderr is held apart from
# the argv its stdout carries; rewritten per member, read only on a dispatch error
DISPATCH_ERR="${GATE_SDK_TMP_DIR:-.tmp}/gate-dispatch-stderr.txt"
total_ms=0
VERBOSE="${GATE_SDK_VERBOSE:-}"
for i in "${!RUN_MEMBERS[@]}"; do
    c="${RUN_MEMBERS[$i]}"
    args=()
    if [[ ${#FOR_PATHS[@]} -gt 0 && -n "${RUN_ARGSTR[$i]}" ]]; then
        mapfile -t args <<<"${RUN_ARGSTR[$i]}"
    fi
    start_ns=$(date +%s%N)
    out=""; ok=1
    # spec: gate-sdk/SPEC.md §lib/gate.sh — execute the invocation argv, keeping
    # gate_command's exit 2 (dispatch harness error) distinct from its exit 1
    # (nothing declares this member)
    # spec: gate-sdk/SPEC.md §run-gates — the two streams are captured apart, never
    # merged: argv is stdout alone, so a diagnostic any successful call writes to
    # stderr cannot become argv[0] and be exec'd
    argv_out="$(gate_command "$c" "${RESOLVE_DIRS[@]}" 2>"$DISPATCH_ERR")"; cstatus=$?
    argv=()
    [[ -n "$argv_out" && "$cstatus" -eq 0 ]] && mapfile -t argv <<<"$argv_out"
    if [[ "$cstatus" -eq 0 && ${#argv[@]} -gt 0 ]]; then
        if out="$("${argv[@]}" "${args[@]}" 2>&1)"; then
            tail="  PASS: $c"
        else
            rc=$?; ok=0; tail="  FAIL: $c (exit $rc)"; failed+=("$c")
        fi
    elif [[ "$cstatus" -eq 2 ]]; then
        ok=0
        out="$(<"$DISPATCH_ERR")"
        tail="  FAIL: $c (dispatch harness error, exit 2)"
        failed+=("$c")
    else
        ok=0
        out="$c listed in $LIST but resolves in none of: ${RESOLVE_DIRS[*]}"
        tail="  FAIL: $c (unresolved)"
        failed+=("$c")
    fi
    if (( ! ok )) || [[ -n "$VERBOSE" ]]; then
        printf '\n===== %s =====\n' "$c"
        [[ -n "$out" ]] && printf '%s\n' "$out"
        printf '%s\n' "$tail"
    fi
    elapsed_ms=$(( ($(date +%s%N) - start_ns) / 1000000 ))
    printf '%s %d\n' "$c" "$elapsed_ms" >> "$TIMINGS"
    total_ms=$(( total_ms + elapsed_ms ))
done
printf 'TOTAL %d\n' "$total_ms" >> "$TIMINGS"

printf '\n===== gates summary =====\n'
# spec: gate-sdk/SPEC.md §run-gates — a declared omission is what keeps `All N gates passed.` honest as the roster-collapse tripwire: an omitted member shrinks N legitimately, so it is reported on its own line beside the summary. Separate is load-bearing rather than tidy — the consumer smokes match the green phrase against this output, so the line must not carry it
report_omissions() {
    local reason count
    while read -r count reason; do
        [[ -n "$reason" ]] || continue
        case "$reason" in
            substrate-unavailable)
                printf '%d gate(s) omitted (%s): no prebuilt gate binary is published for this platform.\n' \
                    "$count" "$reason" ;;
            digest-unverifiable)
                printf '%d gate(s) omitted (%s): install sha256sum or shasum, then re-run checkwright init.\n' \
                    "$count" "$reason" ;;
            *)
                printf '%d gate(s) omitted (%s).\n' "$count" "$reason" ;;
        esac
    done < <(awk '$1 == "#" && $2 == "omitted:" { print $4 }' "$LIST" | sort | uniq -c)
}
report_omissions
if [[ ${#failed[@]} -eq 0 ]]; then
    printf 'All %d gates passed.\n' "${#RUN_MEMBERS[@]}"
    exit 0
fi
printf '%d of %d gates FAILED: %s\n' "${#failed[@]}" "${#RUN_MEMBERS[@]}" "${failed[*]}"
exit 1
