#!/usr/bin/env bash
# Behavioral test of bin/diff-baseline.sh's status handling — the one thing no
# gate fixture pair can hold, because the tool is a situational bin rather than a
# registered gate, and because what is under test is an argument grammar plus a
# refusal rather than a verdict over a tree.
#
# The defect it pins: the tool used to hand ek_parse a hardcoded 0, so an
# exit-code suite reported pass for every log it was ever handed and the tool
# cleared reds it could not see. A test asserting only the happy path would have
# passed against that, so case A is the load-bearing one.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # evidence-kit/
BIN="$DIR/bin/diff-baseline.sh"

fails=0
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT
mkdir -p "$tmp/empty" "$tmp/scratch"

printf '# fixture\ngreensuite greensuite pass\nredsuite redsuite fail some-live-slug\n' >"$tmp/base.txt"
printf 'whatever the suite happened to print\n' >"$tmp/log"

# Every run is hermetic: an empty gates dir keeps consumer config off the lookup
# path, and the baseline, skip file and scratch are the fixture's own.
_run() {
    env -u EVIDENCE_KIT_CONFIG_FILE \
        GATE_SDK_GATES_DIR="$tmp/empty" \
        EVIDENCE_KIT_PARSER=exit-code \
        EVIDENCE_KIT_BASELINE_FILE="$tmp/base.txt" \
        EVIDENCE_KIT_SKIP_FILE="$tmp/skip.txt" \
        EVIDENCE_KIT_TMP_DIR="$tmp/scratch" \
        bash "$BIN" "$@" 2>&1
    printf 'rc=%s\n' "$?"
}

# A — an exit-code suite with no status REFUSES. This is the fail-closed arm: the
#     tool cannot derive that suite's verdict from the log, so assuming success
#     would clear a red structurally invisible to it.
out="$(_run greensuite "$tmp/log")"
if ! grep -q 'parsed by exit code and no status was given' <<<"$out" || ! grep -qx 'rc=2' <<<"$out"; then
    echo "  FAIL: an exit-code suite with no status must refuse at exit 2, not assume 0: $out"; fails=$((fails + 1))
fi

# B — the same suite with a failing status reds against its baseline 'pass' row.
#     This is the assertion the hardcoded 0 made unreachable.
out="$(_run greensuite "$tmp/log" 1)"
if ! grep -qx 'new-failure greensuite greensuite' <<<"$out" || ! grep -qx 'rc=1' <<<"$out"; then
    echo "  FAIL: a failing status against a baseline 'pass' row must be a new-failure: $out"; fails=$((fails + 1))
fi

# C — a failing status against a baseline 'fail' row is clean. This is what lets a
#     CI leg be green against a recorded baseline rather than green simpliciter.
out="$(_run redsuite "$tmp/log" 1)"
if ! grep -q '^diff-baseline: clean' <<<"$out" || ! grep -qx 'rc=0' <<<"$out"; then
    echo "  FAIL: a failing status matching a baseline 'fail' row must stay clean: $out"; fails=$((fails + 1))
fi

# D — a passing status against a baseline 'fail' row is an unpromoted recovery:
#     reported, never red, so a fixed suite does not silently re-baseline itself.
out="$(_run redsuite "$tmp/log" 0)"
if ! grep -qx 'recovery redsuite redsuite' <<<"$out" || ! grep -qx 'rc=0' <<<"$out"; then
    echo "  FAIL: a passing status against a baseline 'fail' row must report a recovery at rc 0: $out"; fails=$((fails + 1))
fi

# E — the status is optional per group and a suite name can never be mistaken for
#     one, since a suite name suffixes EVIDENCE_KIT_RUN_<suite> and so is a shell
#     identifier. Two groups, the first carrying a status and the second not:
#     the second must reach its own refusal rather than swallowing a neighbour.
out="$(_run redsuite "$tmp/log" 1 greensuite "$tmp/log")"
if ! grep -q "suite 'greensuite' is parsed by exit code" <<<"$out" || ! grep -qx 'rc=2' <<<"$out"; then
    echo "  FAIL: group parsing must not let a statusless group borrow its neighbour's status: $out"; fails=$((fails + 1))
fi

# F — a log-parsing suite may still omit the status, so the pair form keeps
#     working for the consumers it was written for.
printf '#!/usr/bin/env bash\nprintf "greensuite pass\\n"\n' >"$tmp/stub.sh"
out="$(env -u EVIDENCE_KIT_CONFIG_FILE \
        GATE_SDK_GATES_DIR="$tmp/empty" \
        EVIDENCE_KIT_PARSER="bash $tmp/stub.sh" \
        EVIDENCE_KIT_BASELINE_FILE="$tmp/base.txt" \
        EVIDENCE_KIT_SKIP_FILE="$tmp/skip.txt" \
        EVIDENCE_KIT_TMP_DIR="$tmp/scratch" \
        bash "$BIN" greensuite "$tmp/log" 2>&1; printf 'rc=%s\n' "$?")"
if ! grep -q '^diff-baseline: clean' <<<"$out" || ! grep -qx 'rc=0' <<<"$out"; then
    echo "  FAIL: a log-parsing suite must still accept the statusless pair form: $out"; fails=$((fails + 1))
fi

if [[ "$fails" -gt 0 ]]; then
    echo "diff-baseline-status.test: $fails assertion(s) failed"
    exit 1
fi
echo "diff-baseline-status.test: ok (exit-code suite refuses without a status; status drives new-failure, baselined-fail and recovery; group parsing is unambiguous; the pair form survives for log-parsing suites)"
exit 0
