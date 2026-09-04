#!/usr/bin/env bash
# Behavioral test of the --diff-baseline arm's status handling and argv shape — the one thing no
# gate fixture pair can hold, because the tool is a situational arm rather than a
# registered gate, and because what is under test is an argument grammar plus a
# refusal rather than a verdict over a tree.
#
# spec: evidence-kit/SPEC.md §bin/diff-baseline.sh — driven through the front end since the
# 2026-09-04 port, which is also where the `-h`/`--help` arm now lives; cases G, H and I are the
# three behaviours the port ADDS, so they are asserted rather than carried over.
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
FE="$(cd "$DIR/../gate-sdk/bin" && pwd)/run-gates.sh"

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
        bash "$FE" --diff-baseline "$@" 2>&1
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
        bash "$FE" --diff-baseline greensuite "$tmp/log" 2>&1; printf 'rc=%s\n' "$?")"
if ! grep -q '^diff-baseline: clean' <<<"$out" || ! grep -qx 'rc=0' <<<"$out"; then
    echo "  FAIL: a log-parsing suite must still accept the statusless pair form: $out"; fails=$((fails + 1))
fi

# G — the front end owns the help arm: usage on STDOUT at exit 0, and it names this member. The
#     shell form had no help branch at all, so this is an addition rather than a preserved shape.
out="$(bash "$FE" --help 2>/dev/null; printf 'rc=%s\n' "$?")"
if ! grep -q -- '--diff-baseline' <<<"$out" || ! grep -qx 'rc=0' <<<"$out"; then
    echo "  FAIL: the front-end help arm must print usage naming --diff-baseline on stdout at exit 0: $out"; fails=$((fails + 1))
fi

# H — a positional beginning with a dash is REFUSED by name. The defect this closes is the sharpest
#     one the port fixes: the shell form absorbed `--help` as a suite name, matched no baseline row,
#     and printed `clean` at exit 0 — a wrong verdict read by a CI check mark rather than a session.
out="$(_run --help "$tmp/log")"
if ! grep -q 'unrecognized option: --help' <<<"$out" || ! grep -qx 'rc=2' <<<"$out"; then
    echo "  FAIL: a dash-led positional must be refused by name at exit 2, never absorbed as a suite: $out"; fails=$((fails + 1))
fi
if grep -q '^diff-baseline: clean' <<<"$out"; then
    echo "  FAIL: a mistyped invocation reported CLEAN — the exact false green the shape refusal exists against: $out"; fails=$((fails + 1))
fi

# I — `--` ends option processing, which is what keeps the refusal a fix rather than a capability
#     loss: a suite legitimately named with a leading dash is still reachable as free text.
#     The escape's product is that the token reaches the group parser AS A SUITE NAME, so the
#     assertion is the finding line naming it; the verdict that follows is the fail-closed rule's.
out="$(_run -- -dashsuite "$tmp/log" 1)"
if grep -q 'unrecognized option' <<<"$out" || ! grep -qx 'new-failure -dashsuite -dashsuite' <<<"$out"; then
    echo "  FAIL: the -- separator must admit a dash-led suite name as free text: $out"; fails=$((fails + 1))
fi

if [[ "$fails" -gt 0 ]]; then
    echo "diff-baseline-status.test: $fails assertion(s) failed"
    exit 1
fi
echo "diff-baseline-status.test: ok (exit-code suite refuses without a status; status drives new-failure, baselined-fail and recovery; group parsing is unambiguous; the pair form survives for log-parsing suites; the help arm is the front end's, a dash-led positional is refused by name, and -- admits one as free text)"
exit 0
