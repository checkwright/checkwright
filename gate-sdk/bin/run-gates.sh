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

cd "$(git rev-parse --show-toplevel 2>/dev/null)" || {
    echo "run-gates: not inside a git repository" >&2
    exit 2
}

# spec: gate-sdk/SPEC.md §run-gates — the one usage text, the stdout body of a help request and
# the stderr body of an unrecognized-option refusal, per §The bin/-tool contract
usage() {
    cat <<'EOF'
usage: run-gates.sh [gates-dir]                run every registered gate
       run-gates.sh --only <name> [<name>...]  run only the named registered gates
       run-gates.sh --for <path> [<path>...]   run only gates coupling to those paths
       run-gates.sh --emit <arm> [args...]     dispatch a ported non-gate emitter arm
       run-gates.sh --hook <member>            dispatch a harness hook member, payload on stdin
       run-gates.sh --statusline               render the harness status line, payload on stdin
       run-gates.sh --usage-poll               refresh the usage snapshot from its source
       run-gates.sh --usage-verdict [paths]    budget verdict: 0 OK/RESET-OK, 1 PAUSE, 2 STALE
       run-gates.sh --lesson-sink <tag>        route a lesson body on stdin to its sink
       run-gates.sh --upgrade-smoke            prove the FROM->TO kit upgrade in scratch
       run-gates.sh --install-lifecycle [file] install the lifecycle resident surfaces
       run-gates.sh --wait-probe <sub> [args]  the wait-primitive probe: 'sweep' is the reproducer
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
  --hook  dispatches the named harness hook member: the harness payload passes
          through on stdin, the hook-JSON envelope (where the member emits one)
          comes back on stdout, and the exit status is the harness's own
          allow/block signal. Where the binary is absent or its configuration
          cannot be resolved, this arm declines with a diagnostic on stderr and
          exit 0 rather than blocking every guarded tool call.
  --statusline  renders the status line for the harness's statusLine hook and
          rewrites the usage snapshot; declines like --hook when unavailable.
  --usage-poll  runs one poll cycle against the usage source and rewrites the
          snapshot. Its caller is a refresh command or a session rather than a
          gate on a tool call, so it refuses with exit 2 when unavailable.
  --usage-verdict  emits one budget verdict line on stdout from the usage
          snapshot: exit 0 OK / RESET-OK, 1 PAUSE, 2 STALE or unreadable
          (budget-unknown, which never blocks delegation). Two optional
          positionals override the snapshot and credentials paths for test
          injection; a path beginning with a dash is passed after `--`.
          Unavailable is exit 2, the same code an unreadable snapshot takes.
  --lesson-sink  reads a lesson body on stdin and runs the sink configured for
          <tag>, or appends to <workflow-dir>/<tag>-harvest.md when none is.
          The sink's exit status is this arm's, so a failing sink is visible to
          the close step that ran it; unavailable is exit 2 for the same reason.
  --upgrade-smoke  vendors every kit at GATE_SDK_UPGRADE_FROM into a scratch
          consumer, swaps them wholesale to GATE_SDK_UPGRADE_TO and asserts the
          sync is deterministic and the phase-B red set is declared. Takes no
          argument. Exit 0 clean with one UPGRADE-SMOKE line on stdout, 1 an
          upgrade finding, 2 a broken tag or environment; unavailable is 2.
  --install-lifecycle  writes the lifecycle registration block into the
          always-loaded agent file, the iteration-scoped merge attributes into
          .gitattributes, and the keep-ours merge driver into this clone's git
          config. The optional positional is the agent file to write into,
          overriding LIFECYCLE_KIT_AGENT_FILE. Idempotent; exit 2 when the agent
          file is absent or a marker pair is malformed, and unavailable is 2.
  --wait-probe  stands known-duration producers up and measures candidate wait
          forms against them, one trial line per run. The subcommand is an
          operand: produce, waiter, arm-local, record, report, sweep — the
          roster prints on stderr at exit 2 on misuse. Exit 0 on a completed
          subcommand, 1 for `report` with no trials recorded, 2 on misuse;
          unavailable is 2. Hand-invoked, wired into no tier, and `sweep`
          sleeps for its declared durations.
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
    exec env ${elems[@]+"${elems[@]}"} "$bin" "$arm" "$@"
}

FOR_PATHS=()
ONLY_NAMES=()
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
    # spec: gate-sdk/SPEC.md §The non-gate arm — the harness-integration arm: the member name is an
    # operand, never composed into the flag, and `exec` preserves stdin so the harness payload
    # reaches the arm untouched
    # spec: gate-sdk/SPEC.md §run-gates — the fail-open status: this arm's exit is the harness's
    # allow/block signal, so a tree with no binary for its platform declines rather than refusing
    # every guarded tool call
    --hook)
        shift
        HOOK_MEMBER="${1:-}"
        [[ -n "$HOOK_MEMBER" ]] || { echo "run-gates: --hook needs a member name" >&2; exit 2; }
        ARM_UNAVAILABLE_STATUS=0
        exec_arm --hook "$@"
        ;;
    # spec: gate-sdk/SPEC.md §The non-gate arm — the two harness-integration arms outside the hook
    # protocol. --statusline gates no tool call but the harness ignores its status, so it declines
    # the same way; --usage-poll keeps exit 2, its caller being a refresh command or a session
    --statusline)
        shift
        ARM_UNAVAILABLE_STATUS=0
        exec_arm --statusline "$@"
        ;;
    --usage-poll)
        shift
        exec_arm --usage-poll "$@"
        ;;
    # spec: delegation-kit/SPEC.md §usage-verdict — a bridged arm outside the `--emit-` family: a
    # three-state exit status an emitting arm collapses. Unavailable keeps exit 2 — the arm gates
    # no tool call and 2 is already this rule's own budget-unknown code
    --usage-verdict)
        shift
        exec_arm --usage-verdict "$@"
        ;;
    # spec: queue-kit/SPEC.md §The lesson-sink arm — a bridged arm outside the `--emit-` family,
    # because its contract is the sink's exit status and an emitting arm collapses it. It keeps
    # exit 2 when unavailable: its caller is a close-stage step whose failure must be visible
    --lesson-sink)
        shift
        exec_arm --lesson-sink "$@"
        ;;
    # spec: gate-sdk/SPEC.md §upgrade-smoke — a second bridged arm outside the `--emit-` family, on
    # the same ground: its contract is the 1-versus-2 split of its exit status. Unavailable is exit
    # 2 because its callers are a validate suite and the pre-release assertion, both of which read
    # the verdict
    --upgrade-smoke)
        shift
        exec_arm --upgrade-smoke "$@"
        ;;
    # spec: lifecycle-kit/SPEC.md §bin/install-lifecycle.sh — a third bridged arm outside the
    # `--emit-` family: its contract is an action with an exit status rather than a document.
    # Unavailable is exit 2 because its caller is an install step whose failure must be visible
    --install-lifecycle)
        shift
        exec_arm --install-lifecycle "$@"
        ;;
    # spec: delegation-kit/SPEC.md §bin/wait-probe — a bridged arm outside the `--emit-` family, on
    # its three-state exit status. Unavailable keeps exit 2 because the caller is a session reading
    # the verdict, and the subcommand stays an operand so the flag composes with nothing
    --wait-probe)
        shift
        exec_arm --wait-probe "$@"
        ;;
    --for)
        shift
        FOR_PATHS=("$@")
        [[ ${#FOR_PATHS[@]} -gt 0 ]] || { echo "run-gates: --for needs at least one path" >&2; exit 2; }
        set --
        ;;
    # spec: gate-sdk/SPEC.md §run-gates — a name beginning with '-' is an unrecognized option
    # wherever it stands in the list, so `--only --for` refuses at the name instead of taking it
    # for a gate and reporting it unregistered
    --only)
        shift
        ONLY_NAMES=("$@")
        [[ ${#ONLY_NAMES[@]} -gt 0 ]] || { echo "run-gates: --only needs at least one gate name" >&2; exit 2; }
        for only_name in "${ONLY_NAMES[@]}"; do
            case "$only_name" in -*) unrecognized_option "$only_name" ;; esac
        done
        set --
        ;;
    --)
        shift
        ;;
    -*)
        unrecognized_option "$1"
        ;;
esac

# spec: gate-sdk/SPEC.md §run-gates — the *effective* gates dir crosses as an explicit argument on every run rather than as an override of the bridged knob: the arm scopes its own declared-knob union by the registry there, and it keeps the configured value beside it so the `--only` steer can tell a positional from the default
GATES_DIR_ARG="${1-}"
GATES_DIR="${GATES_DIR_ARG:-$(gate_sdk_gates_dir)}"
LIST="$GATES_DIR/gates.list"

# spec: gate-sdk/SPEC.md §run-gates — the arm is the dispatcher wherever the binary is present, which is every covered platform; the shell loop below runs only where criterion 5's omit-and-declare branch left a consumer without one, and is the one duplication that branch's own contract requires
if [[ -x "$(gate_native_bin)" ]]; then
    RUN_ARGS=(--gates-dir "$GATES_DIR")
    [[ ${#ONLY_NAMES[@]} -gt 0 ]] && RUN_ARGS+=(--only "${ONLY_NAMES[@]}")
    [[ ${#FOR_PATHS[@]} -gt 0 ]]  && RUN_ARGS+=(--for "${FOR_PATHS[@]}")
    exec_arm --run "${RUN_ARGS[@]}"
fi

# spec: gate-sdk/SPEC.md §run-gates — the --only steer: a positional that is really a member of the
# default registry earns the remedy beside the refusal, never a run it did not ask for
if [[ ! -f "$LIST" ]]; then
    echo "run-gates: no registry at $LIST" >&2
    DEFAULT_LIST="$(gate_sdk_gates_dir)/gates.list"
    if [[ -n "$GATES_DIR_ARG" && -f "$DEFAULT_LIST" ]]; then
        mapfile -t DEFAULT_MEMBERS < <(gates_list_members "$DEFAULT_LIST")
        for m in ${DEFAULT_MEMBERS[@]+"${DEFAULT_MEMBERS[@]}"}; do
            [[ "$m" == "$GATES_DIR_ARG" ]] || continue
            printf "run-gates: '%s' is a gate registered in %s, not a gates dir — run it with: run-gates.sh --only %s\n" \
                "$GATES_DIR_ARG" "$DEFAULT_LIST" "$GATES_DIR_ARG" >&2
            break
        done
    fi
    exit 2
fi

mapfile -t MEMBERS < <(gates_list_members "$LIST")
[[ ${#MEMBERS[@]} -gt 0 ]] || { echo "run-gates: $LIST names no gates" >&2; exit 2; }

RESOLVE_DIRS=("$GATES_DIR")
while IFS= read -r k; do RESOLVE_DIRS+=("$k/checks"); done < <(gate_kit_roots)

# spec: gate-sdk/SPEC.md §run-gates — where both selectors land their result: RUN_LIST + RUN_ARGSTR (index-aligned; newline-joined staged-mode args under --for, empty for every member under --only, which names gates and has no paths to hand one)
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

# spec: gate-sdk/SPEC.md §run-gates — --only selection: set-shaped and registry-ordered, so two
# names give one transcript whichever way they were typed; an unregistered name is a refusal
# because a name is a claim about the registry, never a fact about the tree
select_only() {
    local n c found
    for n in "${ONLY_NAMES[@]}"; do
        found=0
        for c in "${MEMBERS[@]}"; do [[ "$c" == "$n" ]] && { found=1; break; }; done
        (( found )) || {
            echo "run-gates: --only: '$n' is not registered in $LIST" >&2
            exit 2
        }
    done
    for c in "${MEMBERS[@]}"; do
        for n in "${ONLY_NAMES[@]}"; do
            [[ "$c" == "$n" ]] || continue
            RUN_LIST+=("$c"); RUN_ARGSTR+=("")
            break
        done
    done
}

RUN_MEMBERS=("${MEMBERS[@]}")
if [[ ${#ONLY_NAMES[@]} -gt 0 ]]; then
    select_only
    RUN_MEMBERS=("${RUN_LIST[@]}")
elif [[ ${#FOR_PATHS[@]} -gt 0 ]]; then
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
# spec: gate-sdk/SPEC.md §run-gates — where gate_command's stderr is held apart from the argv its
# stdout carries; a scratch file of this dispatcher's own, rewritten per member and read only on a
# dispatch error, so nothing outside this loop has it as a reader
DISPATCH_ERR="$(mktemp)"
trap 'rm -f "$DISPATCH_ERR"' EXIT
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
