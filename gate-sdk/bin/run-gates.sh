#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §run-gates — the battery front-end, reduced to its residue: resolve the repo root, resolve one bridged environment, exec the binary. The argument grammar, the usage text, the selectors, the dispatch and the output contract are all the binary's `--run` arm's.
# no-port: gate-sdk/SPEC.md §run-gates, The front-end's port disposition — this is the residue that stub cut left, and it stays shell on the config-seam cause gate-sdk/SPEC.md §The config-seam port disposition rules for lib/gate.sh itself: gate_knob_env is the config bridge's bash producer, the single place a knob's value is computed, so the one call below cannot be made from inside the binary it is resolving the environment for. An existing cause reached one step further, never a new class.
#
# usage: run-gates.sh [gates-dir] | --only <name>... [-- <arg>...] | --for <path>... | --emit <arm> [args...] | -h | --help
#        every arm, its refusals and the knobs print from the tool itself: run-gates.sh --help
#   timings → $GATE_SDK_TMP_DIR/gate-timings.txt (default .tmp/); a measurement, never committed
set -uo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"

cd "$(git rev-parse --show-toplevel 2>/dev/null)" || {
    echo "run-gates: not inside a git repository" >&2
    exit 2
}

# spec: gate-sdk/SPEC.md §run-gates — the bridged environment for one arm, resolved and exec'd: the front-end's whole job beyond argv, and the shape `--emit` already had. $ARM_UNAVAILABLE_STATUS is the status a *dispatch* failure exits — 2 for every arm whose verdict a battery or a session reads, 0 for a harness-integration arm gating a user action, which §The non-gate arm rules must decline rather than wedge the session
ARM_UNAVAILABLE_STATUS=2
exec_arm() {
    local arm="$1"
    shift
    local bin env_out
    local -a elems=()
    bin="$(gate_native_bin)"
    if [[ ! -x "$bin" ]]; then
        printf 'run-gates: %s dispatches to the native binary, but %s is absent or not ' "$arm" "$bin" >&2
        printf 'executable — it could not run. Build it: bash gate-sdk/bin/build-native.sh\n' >&2
        exit "$ARM_UNAVAILABLE_STATUS"
    fi
    env_out="$(gate_knob_env "$arm" "$@")" || exit "$ARM_UNAVAILABLE_STATUS"
    [[ -n "$env_out" ]] && mapfile -t elems <<<"$env_out"
    # spec: gate-sdk/SPEC.md §run-gates — `env` adds the bridged elements to the environment this
    # process already carries rather than replacing it, which is how the two unbridged reporting
    # knobs, GATE_SDK_VERBOSE and GATE_SDK_JOBS, reach the arm at all
    exec env ${elems[@]+"${elems[@]}"} "$bin" "$arm" "$@"
}

# spec: gate-sdk/SPEC.md §run-gates — the one piece of per-arm knowledge the stub keeps, and it is a two-name test rather than a table: the unavailable status is read on precisely the path where the binary is absent, so it is the one property that cannot be asked of the binary that would report it
case "${1-}" in
    --hook | --statusline) ARM_UNAVAILABLE_STATUS=0 ;;
esac

# spec: gate-sdk/SPEC.md §run-gates — the residual argv grammar, and the whole of it: the *gates-dir positional* is the one token the crate cannot tell from a gate name, so the front-end resolves it and spells it `--gates-dir`, which is also what scopes the arm's declared-knob union to that registry. Every other form of the battery's own grammar — the two selectors, the help request, the `--` escape and every refusal — travels to the `--run` arm untouched, and every other leading token is an arm name the crate's own parser normalizes
case "${1-}" in
    # spec: gate-sdk/SPEC.md §run-gates — a help request carries no gates dir, and that is
    # correctness rather than thrift: the positional is what scopes the declared-knob union, so a
    # help request carrying one would fail wherever any registered member's knobs cannot resolve
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
