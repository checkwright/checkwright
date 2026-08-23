#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §run-gates — the battery front-end: it owns the argument grammar and the bridged environment, and execs the binary's `--run` arm, which owns the registry walk, the selection, the dispatch and the output contract
#
# usage: run-gates.sh [gates-dir] | --only <name>... | --for <path>... | --emit <arm> [args...] | -h | --help
#        every arm, its refusals and the knobs print from the tool itself: run-gates.sh --help
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

# spec: gate-sdk/SPEC.md §run-gates — the one usage text, the stdout body of a help request and
# the stderr body of an unrecognized-option refusal, per §The bin/-tool contract
usage() {
    cat <<'EOF'
usage: run-gates.sh [gates-dir]                run every registered gate
       run-gates.sh --only <name> [<name>...]  run only the named registered gates
       run-gates.sh --for <path> [<path>...]   run only gates coupling to those paths
       run-gates.sh --emit <arm> [args...]     dispatch a ported non-gate emitter arm
       run-gates.sh -h | --help                this text, on stdout, exit 0

  --only  runs the named members in registry order whatever order they were
          typed; duplicates collapse, and an unregistered name is a refusal.
          The [gates-dir] positional is unavailable in this form — point
          GATE_SDK_GATES_DIR at another registry instead.
  --for   selects by coupling: every gate whose effective trigger matches one
          of the given repo-relative paths, exactly as the generated hook
          would. A path no gate couples to is a note, not a failure.
  --emit  dispatches the named non-gate arm of the native binary, handing it
          every remaining argument.
  --      ends option processing, so a gates-dir spelled with a leading dash
          is still reachable.

The battery itself is the binary's `--run` arm; this script resolves its
bridged environment and execs it.

GATE_SDK_VERBOSE (any non-empty value) restores the per-gate banner roll the
quiet-green output contract suppresses; GATE_SDK_JOBS sets the worker count
(default: the machine's parallelism; 1 restores a serial run). Per-gate timings
land in $GATE_SDK_TMP_DIR/gate-timings.txt (default .tmp/).
EOF
}

unrecognized_option() {
    printf 'run-gates: unrecognized option: %s\n' "$1" >&2
    usage >&2
    exit 2
}

# spec: gate-sdk/SPEC.md §run-gates — the bridged environment for one arm, resolved and exec'd: the front-end's whole job beyond argv, and the shape `--emit` already had
exec_arm() {
    local arm="$1"
    shift
    local bin env_out
    local -a elems=()
    bin="$(gate_native_bin)"
    if [[ ! -x "$bin" ]]; then
        printf 'run-gates: %s dispatches to the native binary, but %s is absent or not ' "$arm" "$bin" >&2
        printf 'executable — it could not run. Build it: bash gate-sdk/bin/build-native.sh\n' >&2
        exit 2
    fi
    env_out="$(gate_knob_env "$arm")" || exit 2
    [[ -n "$env_out" ]] && mapfile -t elems <<<"$env_out"
    exec env ${elems[@]+"${elems[@]}"} "$bin" "$arm" "$@"
}

RUN_ARGS=()
# spec: gate-sdk/SPEC.md §run-gates — the argument grammar: every arm is decided off the first
# argument alone, so the option arms are exclusive and a leading '-' that names none of them is a
# refusal rather than a gates-dir
case "${1-}" in
    -h | --help)
        usage
        exit 0
        ;;
    # spec: gate-sdk/SPEC.md §The non-gate arm — the emitter front-end: a ported arm receives no
    # configuration, so a caller already sourcing this library resolves its bridged knobs and invokes
    # it. The arm name is an operand here and a suffix in the crate, governed separately.
    --emit)
        shift
        EMIT_ARM_NAME="${1:-}"
        [[ -n "$EMIT_ARM_NAME" ]] || { echo "run-gates: --emit needs an arm name" >&2; exit 2; }
        shift
        exec_arm "--emit-$EMIT_ARM_NAME" "$@"
        ;;
    --for)
        shift
        [[ $# -gt 0 ]] || { echo "run-gates: --for needs at least one path" >&2; exit 2; }
        RUN_ARGS=(--for "$@")
        set --
        ;;
    # spec: gate-sdk/SPEC.md §run-gates — a name beginning with '-' is an unrecognized option
    # wherever it stands in the list, so `--only --for` refuses at the name instead of taking it
    # for a gate and reporting it unregistered
    --only)
        shift
        [[ $# -gt 0 ]] || { echo "run-gates: --only needs at least one gate name" >&2; exit 2; }
        for only_name in "$@"; do
            case "$only_name" in -*) unrecognized_option "$only_name" ;; esac
        done
        RUN_ARGS=(--only "$@")
        set --
        ;;
    --)
        shift
        ;;
    -*)
        unrecognized_option "$1"
        ;;
esac

# spec: gate-sdk/SPEC.md §run-gates — the [gates-dir] positional crosses as an explicit argument rather than as an override of the bridged knob, so the arm still holds the configured value the `--only` steer resolves its default registry through
GATES_DIR_ARG="${1-}"
[[ -n "$GATES_DIR_ARG" ]] && RUN_ARGS=(--gates-dir "$GATES_DIR_ARG" ${RUN_ARGS[@]+"${RUN_ARGS[@]}"})

exec_arm --run ${RUN_ARGS[@]+"${RUN_ARGS[@]}"}
