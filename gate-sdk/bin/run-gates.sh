#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §run-gates — the battery front-end, reduced to its residue: resolve the repo root, locate the binary, export the gate-sdk root locator, exec the binary. The argument grammar, the usage text, the selectors, the dispatch and the output contract are all the binary's `--run` arm's.
# no-port: gate-sdk/SPEC.md §run-gates, The front-end's port disposition — this is the residue that stub cut left, and it stays shell on a per-file bootstrap cause: the front-end locates the binary it executes, which the binary cannot do for itself. It serves a harness shim and a pre-build clone rather than an adopter door now, and neither audience leaves the binary able to answer for itself. Structural, not a sizing judgment.
#
# usage: run-gates.sh [gates-dir] | --only <name>... [-- <arg>...] | --for <path>... | --emit <arm> [args...] | --pack-installer [--version <semver>] [--out <dir>] [--artifacts <dir>] [--root <dir>] | -h | --help
#        every arm, its refusals and the knobs print from the tool itself: run-gates.sh --help
#   timings → $GATE_SDK_TMP_DIR/gate-timings.txt (default .tmp/); a measurement, never committed, written by an unfiltered run
set -uo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"

# spec: gate-sdk/SPEC.md §run-gates — the crosser's repository mark, held here because the crate runs after this stub's cd: a non-empty GIT_DIR, or a .git entry at the physical working directory or above, the ascent stopping where GIT_CEILING_DIRECTORIES stops git's own, which ignores a relative entry
_run_gates_marked() {
    [[ -n "${GIT_DIR:-}" ]] && return 0
    local sep=: d c n=0
    local -a ceil=()
    case "${OSTYPE:-}" in msys* | cygwin* | win32*) sep=';' ;; esac
    IFS="$sep" read -r -a ceil <<<"${GIT_CEILING_DIRECTORIES:-}"
    d="$(pwd -P)"
    while :; do
        if ((n > 0)); then
            for c in "${ceil[@]}"; do
                gate_path_rooted "$c" && [[ "$d" == "$(cd "$c" 2>/dev/null && pwd -P)" ]] && return 1
            done
        fi
        [[ -e "$d/.git" || -L "$d/.git" ]] && return 0
        [[ -z "$d" || "$d" == / ]] && return 1
        d="$(dirname "$d")"
        n=$((n + 1))
    done
}

# spec: gate-sdk/SPEC.md §run-gates — the one piece of per-arm knowledge the stub keeps, the fail-open set on one declaration line check-front-end-fail-open holds to the crate's FAIL_OPEN_ARMS: the unavailable status is read on precisely the paths where no binary runs, so it cannot be asked of the binary that would report it
ARM_UNAVAILABLE_STATUS=2
FAIL_OPEN_ARMS='--hook --statusline'
for arm in $FAIL_OPEN_ARMS; do
    if [[ "${1-}" == "$arm" ]]; then
        ARM_UNAVAILABLE_STATUS=0
    fi
done

# spec: gate-sdk/SPEC.md §The harness-integration arm — a tree whose repository git refuses or skips runs no binary, so a fail-open arm declines there as on an absent binary
_run_gates_refuse() {  # <line> <leading arm>
    echo "$1" >&2
    if [[ "$2" == --hook ]]; then
        printf '%s\n' '{"systemMessage":"run-gates: git refuses or skips the repository this session stands in, so every hook guard in this tree is off and each guarded call is allowed. git status prints its reason, or git --git-dir=<that .git> status for a skipped one"}'
    fi
    exit "$ARM_UNAVAILABLE_STATUS"
}

# spec: gate-sdk/SPEC.md §run-gates — the refusal is the stub's own line alone. The one lookup splits as the crate splits it (§The crate's crosser), and a failed one hands `cd` a path that is never a directory rather than an empty one: bash before 5.3 takes `cd ""` as a silent success
# spec: gate-sdk/SPEC.md §The path-dialect contract — recorded verdict: the toplevel is only opened, by the mark test and the cd, never compared or composed as a string
if answer="$(git rev-parse --show-toplevel --show-prefix 2>/dev/null)" && [[ -n "$answer" ]]; then
    top="${answer%%$'\n'*}"
    prefix=''
    [[ "$answer" == *$'\n'* ]] && prefix="${answer#*$'\n'}"
    if gate_skipped_mark "$top" "$prefix" >/dev/null; then
        _run_gates_refuse "run-gates: git skipped the repository a .git entry marks between here and the toplevel it answered, and answered the enclosing one; git --git-dir=<that .git> status prints its reason" "${1-}"
    fi
else
    top=/dev/null
fi
cd "$top" 2>/dev/null || {
    if _run_gates_marked; then
        _run_gates_refuse "run-gates: git refuses the repository marked here (GIT_DIR, or a .git entry here or above); git status prints its reason" "${1-}"
    fi
    _run_gates_refuse "run-gates: not inside a git repository" "${1-}"
}

# spec: gate-sdk/SPEC.md §Layout and configuration — the gate-sdk root locator, exported from this front-end's own location so every arm reaches the exact root
if [[ "$SDK" == "$PWD"/* ]]; then
    export GATE_SDK_ROOT="${SDK#"$PWD"/}"
else
    export GATE_SDK_ROOT="$SDK"
fi

# spec: gate-sdk/SPEC.md §run-gates — the binary located and exec'd with the environment this process already carries. $ARM_UNAVAILABLE_STATUS is the status a *dispatch* failure exits — 2 for every arm whose verdict a battery or a session reads, 0 for a harness-integration arm gating a user action, which §The non-gate arm rules must decline rather than wedge the session
exec_arm() {
    local bin why=''
    if [[ "$ARM_UNAVAILABLE_STATUS" -eq 0 ]]; then
        bin="$(gate_harness_bin)"
    else
        { IFS= read -r bin; IFS= read -r why; } < <(gate_verdict_bin)
    fi
    if [[ ! -x "$bin" ]]; then
        printf 'run-gates: %s dispatches to the native binary, but %s is absent or not ' "$1" "$bin" >&2
        if [[ -n "$why" ]]; then
            printf 'executable — it could not run. In this linked worktree %s; run it in the main checkout rather than building here\n' "$why" >&2
        else
            printf 'executable — it could not run. Build it: bash gate-sdk/bin/build-native.sh\n' >&2
        fi
        # spec: gate-sdk/SPEC.md §The harness-integration arm — exit-0 stderr reaches the harness's debug log alone, so the fail-open --hook decline speaks through the one envelope every hook event accepts
        if [[ "$1" == --hook ]]; then
            printf '%s\n' '{"systemMessage":"run-gates: the gate binary is absent or not executable, so every hook guard in this tree is off and each guarded call is allowed. Build it: bash gate-sdk/bin/build-native.sh"}'
        fi
        exit "$ARM_UNAVAILABLE_STATUS"
    fi
    exec "$bin" "$@"
}

# spec: gate-sdk/SPEC.md §run-gates — the residual argv grammar, and the whole of it: the *gates-dir positional* is the one token the crate cannot tell from a gate name, so the front-end resolves it and spells it `--gates-dir`. Every other form of the battery's own grammar — the two selectors, the help request, the `--` escape and every refusal — travels to the `--run` arm untouched, and every other leading token is an arm name the crate's own parser normalizes
case "${1-}" in
    -h | --help)
        set -- --run "$@"
        ;;
    --only | --for)
        set -- --run --gates-dir "$(gate_sdk_gates_dir)" "$@"
        ;;
    --)
        shift
        set -- --run --gates-dir "${1:-$(gate_sdk_gates_dir)}"
        ;;
    -*)
        ;;
    '')
        set -- --run --gates-dir "$(gate_sdk_gates_dir)"
        ;;
    *)
        set -- --run --gates-dir "$1"
        ;;
esac

exec_arm "$@"
