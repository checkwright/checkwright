#!/usr/bin/env bash
# Direct unit test of check-commit-subject.sh — the config path (a
# consumer-widened roster) and the git-generated carve-outs, neither of which a
# single good/bad fixture pair can exercise: the pair runs one subject each,
# while the roster is env-selected and the carve-outs are a set.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # gate-sdk/
GATE="$DIR/checks/check-commit-subject.sh"

fails=0
tmp="$(mktemp -d)"; trap 'rm -rf "$tmp"' EXIT

# Run the gate on a one-line subject; echo the exit code.
run_subject() {
    printf '%s\n' "$1" > "$tmp/msg.txt"
    "$GATE" "$tmp/msg.txt" >/dev/null 2>&1
    echo $?
}

# --- config path: a type valid only under a widened roster --------------------
rc="$(run_subject 'epic: land the milestone')"
[[ "$rc" -eq 1 ]] || { echo "  FAIL: 'epic:' should fail the default roster, exited $rc"; fails=$((fails + 1)); }

rc="$(GATE_SDK_COMMIT_TYPES='feat fix epic' run_subject 'epic: land the milestone')"
[[ "$rc" -eq 0 ]] || { echo "  FAIL: 'epic:' should pass a roster that includes it, exited $rc"; fails=$((fails + 1)); }

# A default-roster type must still fail against a narrowed roster that omits it.
rc="$(GATE_SDK_COMMIT_TYPES='fix' run_subject 'feat: add a thing')"
[[ "$rc" -eq 1 ]] || { echo "  FAIL: 'feat:' should fail a roster of just 'fix', exited $rc"; fails=$((fails + 1)); }

# --- git-generated carve-outs (all pass regardless of roster) -----------------
for sub in 'Merge branch feature into master' \
           'Revert "feat: add a thing"' \
           'fixup! feat: add a thing' \
           'squash! feat: add a thing'; do
    rc="$(run_subject "$sub")"
    [[ "$rc" -eq 0 ]] || { echo "  FAIL: carve-out '$sub' should pass, exited $rc"; fails=$((fails + 1)); }
done

# --- valid conventional shapes (scope token, break marker) --------------------
for sub in 'docs: fix a typo' \
           'feat(gate-sdk): add a gate' \
           'refactor(a.b/c-d): rename' \
           'fix!: breaking change'; do
    rc="$(run_subject "$sub")"
    [[ "$rc" -eq 0 ]] || { echo "  FAIL: valid subject '$sub' should pass, exited $rc"; fails=$((fails + 1)); }
done

# --- rejections: missing colon, empty summary, non-roster prefix --------------
for sub in 'feat add a thing' \
           'feat: ' \
           'chore(scope) no colon'; do
    rc="$(run_subject "$sub")"
    [[ "$rc" -eq 1 ]] || { echo "  FAIL: malformed subject '$sub' should fail, exited $rc"; fails=$((fails + 1)); }
done

# --- edge behavior mirroring check-commit-msg ---------------------------------
"$GATE" >/dev/null 2>&1; rc=$?
[[ "$rc" -eq 0 ]] || { echo "  FAIL: no-arg run should clean-skip (exit 0), exited $rc"; fails=$((fails + 1)); }

"$GATE" "$tmp/does-not-exist.txt" >/dev/null 2>&1; rc=$?
[[ "$rc" -eq 2 ]] || { echo "  FAIL: missing message file should fail-closed (exit 2), exited $rc"; fails=$((fails + 1)); }

if [[ "$fails" -gt 0 ]]; then
    echo "check-commit-subject.test: $fails assertion(s) failed"
    exit 1
fi
echo "check-commit-subject.test: ok (config roster; carve-outs; valid shapes; rejections; edge behavior)"
exit 0
