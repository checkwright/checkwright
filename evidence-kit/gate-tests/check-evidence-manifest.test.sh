#!/usr/bin/env bash
# Behavioral test of checks/check-evidence-manifest.sh — the lifecycle-coupled
# assertions the one good/bad pair (grammar) cannot hold: (C) a validate stamp
# demands ≥1 evidence line, disarmed while the header is still at validate and
# re-armed past it; (A) a close-entry header demands the full green block over
# every configured suite; and the no-lifecycle disarm.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # evidence-kit/
GATE="$DIR/checks/check-evidence-manifest.sh"
HASH=e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

fails=0
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

# case_run <name> <manifest-body> <header> <state-body> <suites> <want> <expect>
case_run() {
    local name="$1" man="$2" hdr="$3" state="$4" suites="$5" want="$6" expect="$7"
    local d="$tmp/$name" out rc; mkdir -p "$d/scripts"
    printf '# contract: evidence-manifest v1\n%b' "$man" >"$d/man.txt"
    printf '%s\n' "$hdr" >"$d/queue.md"
    printf '%b' "$state" >"$d/state.txt"
    [[ -n "$suites" ]] && printf 'EVIDENCE_KIT_SUITES=(%s)\n' "$suites" >"$d/scripts/evidence-config.sh"
    out="$( cd "$d" && env -u EVIDENCE_KIT_CONFIG_FILE GATE_SDK_GATES_DIR=scripts "$GATE" man.txt queue.md state.txt 2>&1 )"; rc=$?
    if [[ "$rc" -ne "$want" ]]; then
        echo "  FAIL: $name expected exit $want, got $rc: $out"; fails=$((fails + 1)); return
    fi
    if ! grep -qF -- "$expect" <<<"$out"; then
        echo "  FAIL: $name exit OK but output lacks '$expect': $out"; fails=$((fails + 1))
    fi
}

# A — validate stamp present, header past validate (close), no evidence line:
#     stamp-coupling (C) fires — validate ran and recorded nothing.
case_run "C-recorded-nothing" \
    '' '## Iteration: it  [stage: close]' \
    'h\n---\nit scope s1 2026-07-01\nit validate s2 2026-07-05\n' '' \
    1 "recorded nothing"

# B — same validate stamp, but header still AT validate: C is disarmed (the
#     entry flip legitimately precedes the suites), so a bare manifest is CLEAN.
case_run "C-disarmed-at-validate" \
    '' '## Iteration: it  [stage: validate]' \
    'h\n---\nit scope s1 2026-07-01\nit validate s2 2026-07-05\n' '' \
    0 "clean"

# C — close entry with one of two configured suites missing its clean line:
#     assertion A fires for the missing suite.
case_run "A-missing-suite" \
    "it unit sha256=$HASH pass=1 fail=0 ignore=0 verdict=clean 2026-07-06\n" \
    '## Iteration: it  [stage: close]' \
    'h\n---\nit validate s2 2026-07-05\n' 'unit extra' \
    1 "no clean evidence line"

# D — close entry with both suites clean and dated after the validate stamp is
#     CLEAN (the full green block).
case_run "A-green-block" \
    "it unit sha256=$HASH pass=1 fail=0 ignore=0 verdict=clean 2026-07-06\nit extra sha256=$HASH pass=2 fail=0 ignore=0 verdict=clean 2026-07-06\n" \
    '## Iteration: it  [stage: close]' \
    'h\n---\nit validate s2 2026-07-05\n' 'unit extra' \
    0 "clean"

# E — a clean line dated BEFORE the earliest validate stamp is stale evidence.
case_run "A-stale-date" \
    "it unit sha256=$HASH pass=1 fail=0 ignore=0 verdict=clean 2026-07-01\n" \
    '## Iteration: it  [stage: close]' \
    'h\n---\nit validate s2 2026-07-05\n' 'unit' \
    1 "before the earliest validate stamp"

# F — no lifecycle state file at all: A and C disarm, grammar alone governs.
_no_state() {
    local d="$tmp/nostate"; mkdir -p "$d"
    printf '# contract: evidence-manifest v1\n' >"$d/man.txt"
    printf '## Iteration: it  [stage: close]\n' >"$d/queue.md"
    "$GATE" "$d/man.txt" "$d/queue.md" "$d/absent-state.txt" 2>&1
}
if ! out="$(_no_state)" || ! grep -qF "close-entry/stamp-coupling disarmed" <<<"$out"; then
    echo "  FAIL: no-lifecycle disarm not reported clean: $out"; fails=$((fails + 1))
fi

if [[ "$fails" -gt 0 ]]; then
    echo "check-evidence-manifest.test: $fails assertion(s) failed"
    exit 1
fi
echo "check-evidence-manifest.test: ok (C recorded-nothing + A missing-suite + A stale-date rejected; C disarmed-at-validate + A green-block + no-lifecycle accepted)"
exit 0
