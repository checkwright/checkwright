#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §run-gates — end-to-end lock-in for the dispatch
# capture's stream separation: `gate_command`'s stdout *is* the invocation argv,
# its stderr is diagnostic text, and a diagnostic written by a *successful* call
# must never become argv[0] and be exec'd. Both arms run over a hermetic scratch
# registry and are deterministic by construction — a stand-in kit library that
# writes to stderr while still resolving its knob reproduces the shape without
# reproducing the SIGPIPE-disposition accident that first exposed it.
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

# A .gate member dispatching to a stand-in binary that answers --knobs, then
# reports the bridged value so the argv is observed end to end.
printf '# graph: couples=cfg dir=one valve=none tier=precommit trigger=*\n' \
    > "$scratch/check-noisy.gate"
cat > "$scratch/bin" <<'BIN'
#!/usr/bin/env bash
[[ "${1:-}" == --knobs ]] && { printf 'PROBE_KIT_VALUE\n'; exit 0; }
printf 'check-noisy ran knob=[%s]\n' "${GATE_SDK_KNOB_PROBE_KIT_VALUE:-}"
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

run_battery() {
    GATE_SDK_GATES_DIR="$scratch" GATE_SDK_KIT_DIRS="$scratch/probe-kit" \
        GATE_SDK_NATIVE_BIN="$1" GATE_SDK_TMP_DIR="$scratch/.tmp" \
        GATE_SDK_VERBOSE=1 bash "$RUN" 2>&1
}

# Arm 1 — stderr on a successful dispatch: argv comes from stdout alone, so the
# gate runs with its bridged knob and the diagnostic is not executed.
out="$(run_battery "$scratch/bin")"; rc=$?
assert_has    stdout-argv 'check-noisy ran knob=[resolved]' "$out"
assert_has    stdout-argv 'All 1 gates passed.'             "$out"
assert_absent stdout-argv 'exit 127'                        "$out"
assert_absent stdout-argv 'FAIL: check-noisy'               "$out"
[[ "$rc" -eq 0 ]] || { echo "FAIL [stdout-argv]: expected exit 0, got $rc"; fails=$((fails + 1)); }

# Arm 2 — the stderr the split must not discard: an absent binary is
# gate_command's exit 2, whose diagnostic body is stderr. Deleting the capture
# rather than splitting it would silently empty this report.
out="$(run_battery "$scratch/absent-bin")"; rc=$?
assert_has dispatch-err 'dispatch harness error, exit 2' "$out"
assert_has dispatch-err 'absent or not executable'       "$out"
assert_has dispatch-err 'build-native.sh'                "$out"
[[ "$rc" -eq 1 ]] || { echo "FAIL [dispatch-err]: expected exit 1, got $rc"; fails=$((fails + 1)); }

[[ "$fails" -eq 0 ]] || { echo "run-dispatch-streams.test: $fails assertion(s) failed"; exit 1; }
echo "run-dispatch-streams.test: clean (argv taken from stdout alone with stderr present on a successful dispatch; the exit-2 diagnostic body still reaches the report)"
exit 0
