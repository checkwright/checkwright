#!/usr/bin/env bash
# spec: guard-kit/SPEC.md §scratch-run — the runner echoes an in-scratch script's contents before executing it, passes the child's args and exit code through verbatim, and refuses (exit 2, nothing echoed, nothing executed) any target resolving outside GATE_SDK_TMP_DIR. Every case here is a property of the SEAM — the front end resolving the arm, the config bridge supplying GATE_SDK_TMP_DIR, and a real child process — which a crate unit test cannot see; the shebang classifier's own cases and the containment predicate's are pinned in the ported module's #[cfg(test)] tests, where check-crate-arms runs them.
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

cd "$(git rev-parse --show-toplevel 2>/dev/null || pwd)" || exit 2
ROOT="$(pwd -P)"
RUN="gate-sdk/bin/run-gates.sh"
[[ -x "$RUN" ]] || { echo "scratch-run.test: front end not found: $RUN"; exit 2; }

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
out="$(GATE_SDK_TMP_DIR="$scratch" bash "$RUN" --scratch-run "$scratch/probe.sh" alpha beta 2>&1)"; rc=$?
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
out="$(GATE_SDK_TMP_DIR="$scratch" bash "$RUN" --scratch-run "$outside/evil.sh" 2>&1)"; rc=$?
assert_rc     refuse-outside "$rc" 2
assert_has    refuse-outside "refusing" "$out"
assert_absent refuse-outside "touch" "$out"
[[ -e "$outside/EXECUTED" ]] && { echo "FAIL [refuse-outside]: the child ran"; fails=$((fails + 1)); }

# A path spelled under the scratch dir but resolving above it is the same
# refusal — the guard tests the resolved path, never the spelling.
out="$(GATE_SDK_TMP_DIR="$scratch" bash "$RUN" --scratch-run "$scratch/../$(basename "$outside")/evil.sh" 2>&1)"; rc=$?
assert_rc  refuse-traversal "$rc" 2
assert_has refuse-traversal "refusing" "$out"
[[ -e "$outside/EXECUTED" ]] && { echo "FAIL [refuse-traversal]: the child ran"; fails=$((fails + 1)); }

# A symlink planted INSIDE the scratch dir pointing out of it is the case a
# lexical `..`-normalizing compare would pass: the test reads the resolved path,
# so the containment control stays fail-closed through the port.
ln -s "$outside" "$scratch/escape"
out="$(GATE_SDK_TMP_DIR="$scratch" bash "$RUN" --scratch-run "$scratch/escape/evil.sh" 2>&1)"; rc=$?
assert_rc  refuse-symlink "$rc" 2
assert_has refuse-symlink "refusing" "$out"
[[ -e "$outside/EXECUTED" ]] && { echo "FAIL [refuse-symlink]: the child ran"; fails=$((fails + 1)); }

out="$(GATE_SDK_TMP_DIR="$scratch" bash "$RUN" --scratch-run "$scratch/absent.sh" 2>&1)"; rc=$?
assert_rc  missing-target "$rc" 2
assert_has missing-target "no such script" "$out"

out="$(GATE_SDK_TMP_DIR="$scratch" bash "$RUN" --scratch-run 2>&1)"; rc=$?
assert_rc  no-args "$rc" 2
assert_has no-args "usage" "$out"

# Scratch execution is bash-only, and the runner reads the file's own shebang
# rather than a roster: a target naming a non-bash interpreter is refused
# unexecuted, and the side-effect file proves it never ran.
printf '#!/usr/bin/env python3\nopen("%s/PYRAN", "w").close()\n' "$scratch" > "$scratch/py.py"
out="$(GATE_SDK_TMP_DIR="$scratch" bash "$RUN" --scratch-run "$scratch/py.py" 2>&1)"; rc=$?
assert_rc     refuse-shebang "$rc" 2
assert_has    refuse-shebang "bash-only" "$out"
assert_absent refuse-shebang 'open("' "$out"
[[ -e "$scratch/PYRAN" ]] && { echo "FAIL [refuse-shebang]: the child ran"; fails=$((fails + 1)); }

# The env-indirected spelling resolves to the same interpreter, and a bash
# shebang — direct or through env — still runs, which is what keeps every .sh
# the runner handles today working.
printf '#!/bin/sh\necho sh-ran\n' > "$scratch/posix.sh"
out="$(GATE_SDK_TMP_DIR="$scratch" bash "$RUN" --scratch-run "$scratch/posix.sh" 2>&1)"; rc=$?
assert_rc  allow-sh-shebang "$rc" 0
assert_has allow-sh-shebang "sh-ran" "$out"

# A target with no shebang at all is unaffected: nothing states an interpreter,
# so nothing contradicts the rule.
printf 'echo no-shebang-ran\n' > "$scratch/plain.sh"
out="$(GATE_SDK_TMP_DIR="$scratch" bash "$RUN" --scratch-run "$scratch/plain.sh" 2>&1)"; rc=$?
assert_rc  allow-no-shebang "$rc" 0
assert_has allow-no-shebang "no-shebang-ran" "$out"

[[ "$fails" -eq 0 ]] || { echo "scratch-run.test: $fails assertion(s) failed"; exit 1; }
echo "scratch-run.test: clean (echo precedes exec; args and exit code pass through; out-of-scratch, traversal and symlink-escape targets refused unexecuted; a non-bash shebang refused unexecuted)"
exit 0
