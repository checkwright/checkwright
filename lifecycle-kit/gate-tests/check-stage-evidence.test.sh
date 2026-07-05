#!/usr/bin/env bash
# Behavioral test of checks/check-stage-evidence.sh — the sentinel-scoping
# guards the one-pair good/bad harness cannot hold ('—' legal only at the
# first stage; a '—' stamp legal only while the header is also unnamed), plus
# the waiver-token grammar allowance. The regression lives in the interplay of
# the header guard and the per-stamp loop allowance, so the cases drive the
# whole gate on crafted input via its $1/$2 argument mode.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # lifecycle-kit/
GATE="$DIR/checks/check-stage-evidence.sh"

fails=0
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

# case <name> <header-line> <stamp-lines> <want-exit> <expect-substring>
case_run() {
    local name="$1" hdr="$2" stamp="$3" want="$4" expect="$5" out rc
    printf '%s\n' "$hdr" >"$tmp/TASK-QUEUE.md"
    printf 'header prose\n---\n%b' "$stamp" >"$tmp/WORKFLOW-STATE.txt"
    out="$("$GATE" "$tmp/TASK-QUEUE.md" "$tmp/WORKFLOW-STATE.txt" 2>&1)"; rc=$?
    if [[ "$rc" -ne "$want" ]]; then
        echo "  FAIL: $name expected exit $want, got $rc: $out"; fails=$((fails + 1)); return
    fi
    if ! grep -qF -- "$expect" <<<"$out"; then
        echo "  FAIL: $name exit OK but output lacks '$expect': $out"; fails=$((fails + 1))
    fi
}

# A — an unnamed iteration past the first stage must FAIL (without the header
#     guard a '—' header matched a '— validate' stamp and passed clean).
case_run "unnamed-past-first-stage" \
    '## Iteration: —  [stage: validate]' \
    '— validate s1 2026-06-12\n' \
    1 "still unnamed ('—') at stage 'validate'"

# B — a named header with a leftover '—' bootstrap stamp is stale and must FAIL.
case_run "stale-bootstrap-under-named" \
    '## Iteration: demo-iteration  [stage: scope]' \
    '— scope s1 2026-06-12\n' \
    1 "legal '—' bootstrap"

# C — the legitimate pre-naming first-stage bootstrap is CLEAN.
case_run "unnamed-first-stage-bootstrap" \
    '## Iteration: —  [stage: scope]' \
    '— scope s1 2026-06-12\n' \
    0 "clean"

# D — a properly named iteration with a matching stamp at a later stage is CLEAN.
case_run "named-at-validate" \
    '## Iteration: demo-iteration  [stage: validate]' \
    'demo-iteration scope s1 2026-06-12\ndemo-iteration validate s2 2026-06-12\n' \
    0 "clean"

# E — a waiver line (check-stage-entry assertion C's recorded waiver) is a
#     well-formed stamp token: the grammar accepts it, and it never satisfies
#     the current-stage match (here the build stamp does), so the header is CLEAN.
#     It also shares its id with build here — a waiver stamp is exempt from the
#     stage-distinctness pass, so that reuse does not fire.
case_run "waiver-token-accepted" \
    '## Iteration: demo-iteration  [stage: build]' \
    'demo-iteration scope s1 2026-06-12\ndemo-iteration align-waived s3 2026-06-12\ndemo-iteration build s3 2026-06-12\n' \
    0 "clean"

# F — two distinct stages sharing one session id must FAIL: a stage flip is a
#     context boundary, so scope and build cannot both be session s1.
case_run "shared-session-across-stages" \
    '## Iteration: demo-iteration  [stage: build]' \
    'demo-iteration scope s1 2026-06-12\ndemo-iteration build s1 2026-06-13\n' \
    1 "is shared by stages"

# G — a multi-session build (same stage, two different ids) is CLEAN: same-stage
#     re-entries may rotate the id freely.
case_run "same-stage-multi-session" \
    '## Iteration: demo-iteration  [stage: build]' \
    'demo-iteration scope s1 2026-06-12\ndemo-iteration build s2 2026-06-13\ndemo-iteration build s3 2026-06-14\n' \
    0 "clean"

if [[ "$fails" -gt 0 ]]; then
    echo "check-stage-evidence.test: $fails assertion(s) failed"
    exit 1
fi
echo "check-stage-evidence.test: ok (unnamed past first stage + stale bootstrap + shared-session-across-stages rejected; bootstrap + named later stage + waiver token + multi-session build accepted)"
exit 0
