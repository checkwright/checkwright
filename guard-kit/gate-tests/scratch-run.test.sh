#!/usr/bin/env bash
# spec: guard-kit/SPEC.md §scratch-run — the runner echoes an in-scratch script's contents before executing it, passes the child's args and exit code through verbatim, and refuses (exit 2, nothing echoed, nothing executed) any target resolving outside GATE_SDK_TMP_DIR
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$ROOT" || exit 2
RUN="guard-kit/bin/scratch-run.sh"
[[ -x "$RUN" ]] || { echo "scratch-run.test: runner not found: $RUN"; exit 2; }

scratch="$(mktemp -d)"; outside="$(mktemp -d)"
trap 'rm -rf "$scratch" "$outside"' EXIT

fails=0
assert_has()    { grep -qF -- "$2" <<<"$3" || { echo "FAIL [$1]: expected present: $2"; fails=$((fails + 1)); }; }
assert_absent() { grep -qF -- "$2" <<<"$3" && { echo "FAIL [$1]: expected absent: $2"; fails=$((fails + 1)); }; return 0; }
assert_rc()     { [[ "$2" -eq "$3" ]] || { echo "FAIL [$1]: expected exit $3, got $2"; fails=$((fails + 1)); }; }

# The echoed body is what makes an allowlisted (prompt-free) execution
# self-documenting, so the assertion is on the script's own text, not just a
# header: a runner that printed only the path would satisfy a weaker check.
printf '#!/usr/bin/env bash\necho "ran with: $*"\nexit 7\n' > "$scratch/probe.sh"
out="$(GATE_SDK_TMP_DIR="$scratch" bash "$RUN" "$scratch/probe.sh" alpha beta 2>&1)"; rc=$?
assert_rc  echo-then-exec "$rc" 7
assert_has echo-then-exec 'echo "ran with: $*"' "$out"
assert_has echo-then-exec "scratch-run: $scratch/probe.sh" "$out"
assert_has echo-then-exec 'ran with: alpha beta' "$out"

# Ordering is the evidence property: the contents must appear before the run's
# own output, else the transcript reads the effect ahead of the cause.
body_line="$(grep -n 'echo "ran with' <<<"$out" | head -1 | cut -d: -f1)"
ran_line="$(grep -n '^ran with: alpha beta$' <<<"$out" | head -1 | cut -d: -f1)"
[[ -n "$body_line" && -n "$ran_line" && "$body_line" -lt "$ran_line" ]] || {
    echo "FAIL [echo-before-exec]: script body did not precede its output"; fails=$((fails + 1)); }

# Fail-closed: an out-of-scratch target is refused before any echo or
# execution — the side-effect file proves the child never ran.
printf '#!/usr/bin/env bash\ntouch "%s/EXECUTED"\n' "$outside" > "$outside/evil.sh"
out="$(GATE_SDK_TMP_DIR="$scratch" bash "$RUN" "$outside/evil.sh" 2>&1)"; rc=$?
assert_rc     refuse-outside "$rc" 2
assert_has    refuse-outside "refusing" "$out"
assert_absent refuse-outside "touch" "$out"
[[ -e "$outside/EXECUTED" ]] && { echo "FAIL [refuse-outside]: the child ran"; fails=$((fails + 1)); }

# A path spelled under the scratch dir but resolving above it is the same
# refusal — the guard tests the resolved path, never the spelling.
out="$(GATE_SDK_TMP_DIR="$scratch" bash "$RUN" "$scratch/../$(basename "$outside")/evil.sh" 2>&1)"; rc=$?
assert_rc  refuse-traversal "$rc" 2
assert_has refuse-traversal "refusing" "$out"
[[ -e "$outside/EXECUTED" ]] && { echo "FAIL [refuse-traversal]: the child ran"; fails=$((fails + 1)); }

out="$(GATE_SDK_TMP_DIR="$scratch" bash "$RUN" "$scratch/absent.sh" 2>&1)"; rc=$?
assert_rc  missing-target "$rc" 2
assert_has missing-target "no such script" "$out"

out="$(GATE_SDK_TMP_DIR="$scratch" bash "$RUN" 2>&1)"; rc=$?
assert_rc  no-args "$rc" 2
assert_has no-args "usage" "$out"

[[ "$fails" -eq 0 ]] || { echo "scratch-run.test: $fails assertion(s) failed"; exit 1; }
echo "scratch-run.test: clean (echo precedes exec; args and exit code pass through; out-of-scratch and traversal targets refused unexecuted)"
exit 0
