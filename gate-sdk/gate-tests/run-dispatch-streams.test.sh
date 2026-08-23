#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §run-gates — end-to-end lock-in for the dispatch capture's stream
# separation, which is `gate_command`'s contract rather than the battery's since the runner moved
# into the binary: its stdout *is* the invocation argv, its stderr is diagnostic text, and a
# diagnostic written by a *successful* call must never become argv[0] and be exec'd.
# spec: gate-sdk/SPEC.md §run-gates — plus the arm's own tails over a hermetic scratch registry:
# the exact green phrase, and the dispatch-harness-error tail a `.gate` naming a subcommand the
# binary does not carry still earns. Deterministic by construction — a stand-in kit library that
# writes to stderr while still resolving its knob reproduces the shape without reproducing the
# SIGPIPE-disposition accident that first exposed it.
# Run by run-gate-tests.sh.
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
RUN="$ROOT/gate-sdk/bin/run-gates.sh"
[[ -x "$RUN" ]] || { echo "run-dispatch-streams.test: runner not found: $RUN"; exit 2; }

scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT

fails=0
assert_has()    { grep -qF -- "$2" <<<"$3" || { echo "FAIL [$1]: expected present: $2"; fails=$((fails + 1)); }; }
assert_absent() { grep -qF -- "$2" <<<"$3" && { echo "FAIL [$1]: expected absent: $2"; fails=$((fails + 1)); }; return 0; }

# A .gate member dispatching to a stand-in binary that answers --knobs, so the resolved argv is
# observable without the real binary carrying the subcommand.
printf '# graph: couples=cfg dir=one valve=none tier=precommit trigger=*\n' \
    > "$scratch/check-noisy.gate"
cat > "$scratch/bin" <<'BIN'
#!/usr/bin/env bash
[[ "${1:-}" == --knobs ]] && { printf 'PROBE_KIT_VALUE\n'; exit 0; }
exit 0
BIN
chmod +x "$scratch/bin"

# The stand-in kit whose library resolves the knob *and* writes to stderr while
# doing it: a gate_command that exits 0 with non-empty stderr, which is exactly
# the shape a merged capture turns into argv[0].
mkdir -p "$scratch/probe-kit/lib" "$scratch/probe-kit/checks"
cat > "$scratch/probe-kit/lib/probe.sh" <<'PROBE'
# shellcheck shell=bash
printf 'probe-kit: a diagnostic on stderr, exit status still 0\n' >&2
PROBE_KIT_VALUE=resolved
PROBE
printf 'check-noisy\n' > "$scratch/gates.list"

gate_command_argv() {
    GATE_SDK_GATES_DIR="$scratch" GATE_SDK_KIT_DIRS="$scratch/probe-kit" \
        GATE_SDK_NATIVE_BIN="$1" \
        bash -c 'source "$1/gate-sdk/lib/gate.sh"; gate_command check-noisy "$2"' \
        bash "$ROOT" "$scratch" 2>"$scratch/err.txt"
}

# Arm 1 — stderr on a successful resolution: argv comes from stdout alone, so the diagnostic the
# kit library wrote is not in the argv a caller would exec.
argv="$(gate_command_argv "$scratch/bin")"; rc=$?
err="$(<"$scratch/err.txt")"
assert_has    stdout-argv 'GATE_SDK_KNOB_PROBE_KIT_VALUE=resolved' "$argv"
assert_has    stdout-argv 'check-noisy'                            "$argv"
assert_absent stdout-argv 'a diagnostic on stderr'                 "$argv"
assert_has    stdout-argv 'a diagnostic on stderr'                 "$err"
[[ "$rc" -eq 0 ]] || { echo "FAIL [stdout-argv]: expected exit 0, got $rc"; fails=$((fails + 1)); }

# Arm 2 — the stderr the split must not discard: an absent binary is gate_command's exit 2, whose
# diagnostic body is stderr. Deleting the capture rather than splitting it would empty this report.
gate_command_argv "$scratch/absent-bin" >/dev/null; rc=$?
err="$(<"$scratch/err.txt")"
assert_has dispatch-err 'absent or not executable' "$err"
assert_has dispatch-err 'build-native.sh'          "$err"
[[ "$rc" -eq 2 ]] || { echo "FAIL [dispatch-err]: expected exit 2, got $rc"; fails=$((fails + 1)); }

# Arm 3 — the arm's own tails over a hermetic registry, run through the real binary: a shell member
# that passes earns the exact green phrase, and a `.gate` naming a subcommand the binary does not
# carry earns the dispatch-harness-error tail rather than a pass.
green="$scratch/green"
mkdir -p "$green"
printf 'check-ok\n' > "$green/gates.list"
cat > "$green/check-ok.sh" <<'OK'
#!/usr/bin/env bash
# graph: couples=cfg dir=one valve=none tier=precommit
echo "OK: clean"
OK
chmod +x "$green/check-ok.sh"

run_battery() {
    GATE_SDK_GATES_DIR="$1" GATE_SDK_TMP_DIR="$scratch/.tmp" \
        GATE_SDK_VERBOSE=1 bash "$RUN" 2>&1
}

out="$(run_battery "$green")"; rc=$?
assert_has    arm-green 'All 1 gates passed.' "$out"
assert_has    arm-green '  PASS: check-ok'    "$out"
assert_absent arm-green 'FAIL:'               "$out"
[[ "$rc" -eq 0 ]] || { echo "FAIL [arm-green]: expected exit 0, got $rc"; fails=$((fails + 1)); }

red="$scratch/red"
mkdir -p "$red"
printf 'check-noisy\n' > "$red/gates.list"
cp "$scratch/check-noisy.gate" "$red/check-noisy.gate"
out="$(run_battery "$red")"; rc=$?
assert_has arm-red '  FAIL: check-noisy (dispatch harness error, exit 2)' "$out"
assert_has arm-red 'no such gate subcommand'                              "$out"
assert_has arm-red '1 of 1 gates FAILED: check-noisy'                     "$out"
[[ "$rc" -eq 1 ]] || { echo "FAIL [arm-red]: expected exit 1, got $rc"; fails=$((fails + 1)); }

[[ "$fails" -eq 0 ]] || { echo "run-dispatch-streams.test: $fails assertion(s) failed"; exit 1; }
echo "run-dispatch-streams.test: clean (argv taken from stdout alone with stderr present on a successful resolution; the exit-2 diagnostic body still reaches the caller; the arm's green phrase and dispatch-harness-error tail hold over a hermetic registry)"
exit 0
