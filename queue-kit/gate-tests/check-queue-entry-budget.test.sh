#!/usr/bin/env bash
# spec: queue-kit/SPEC.md §check-queue-entry-budget — the cap's selectable measure, which the
# good/bad pair (one knob setting per case dir) cannot hold: (1) the shipped default reds an entry
# over it, in code points; (2) `<n>lines` measures counted lines; (3) `off` disables assertion A
# while the others still run; (4) credits `off`, and a credit in another unit than the cap's, each
# red the credit; (5) a malformed value and the retired knob name each refuse at exit 2.
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
CHECKS="$ROOT/queue-kit/checks"
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT
mkdir -p "$SANDBOX/scripts"

fails=0
checks=0
note() { echo "  FAIL [$1]: $2"; fails=$((fails + 1)); }

# $1 = the deferred entries
queue() {
    cat >"$SANDBOX/TASK-QUEUE.md" <<EOF
# TASK-QUEUE.md

## Iteration: seeded

## New Features

## Technical Debt

## Deferred

$1

## Done

## Lessons Learned
EOF
}

# $1 = the knob file body; the rest = NAME=VALUE environment
run() {
    local knobs="$1"
    shift
    printf '%s\n' "$knobs" >"$SANDBOX/scripts/queue-config.knobs"
    ( cd "$SANDBOX" && gate_env "QUEUE_KIT_KNOB_FILE=$SANDBOX/scripts/queue-config.knobs" "$@" \
        && gate_run check-queue-entry-budget "$CHECKS" 2>&1 )
}

BIG="$(printf 'ground %.0s' $(seq 1 700))"
LARGE="- **big-idea** — an entry far past the shipped cap.
  $BIG
  **Cost while deferred:** low. Filed 2026-01-01 by scope."
SMALL="- **small-idea** — an extent over a three-line cap.
  a second line
  a third line
  **Cost while deferred:** low. Filed 2026-01-01 by scope."

# --- (1) the shipped default is a code-point cap, and an entry past it reds -----------
checks=$((checks + 1))
queue "$LARGE"
out="$(run '')"; rc=$?
[[ "$rc" -eq 1 ]] || note default-exit "an entry past the default cap exited $rc rather than 1: $out"
grep -qE 'big-idea — [0-9]+cp \(cap 4300cp\)' <<<"$out" \
    || note default-unit "the default cap did not measure in code points: $out"

# --- (2) `<n>lines` measures counted lines: the extent, its trailing blank line included --
checks=$((checks + 1))
queue "$SMALL"
out="$(run 'QUEUE_KIT_ENTRY_CAP = 3lines')"; rc=$?
[[ "$rc" -eq 1 ]] || note lines-exit "a five-line extent under a three-line cap exited $rc: $out"
grep -qF 'small-idea — 5lines (cap 3lines)' <<<"$out" \
    || note lines-unit "the lines unit did not count lines: $out"

# --- (3) `off` disables assertion A and nothing else --------------------------------------
checks=$((checks + 1))
queue "$LARGE"
out="$(run 'QUEUE_KIT_ENTRY_CAP = off')"; rc=$?
[[ "$rc" -eq 0 ]] || note off-exit "an entry under an off cap exited $rc rather than 0: $out"
grep -qF 'size cap off' <<<"$out" || note off-line "the clean line did not name the off cap: $out"
queue "- **uncosted-idea** — no cost field. Filed 2026-01-01 by scope."
out="$(run 'QUEUE_KIT_ENTRY_CAP = off')"; rc=$?
[[ "$rc" -eq 1 ]] || note off-others "assertion C stopped running under an off cap (exit $rc): $out"

# --- (4) a credit reds when credits are off, and when its unit is not the cap's -----------
checks=$((checks + 1))
queue "- **credited-idea** [cap-credit: +2lines 2026-01-02 lead a measured ground] — over the cap.
  a second line
  a third line
  **Cost while deferred:** low. Filed 2026-01-01 by scope."
out="$(run $'QUEUE_KIT_ENTRY_CAP = 3lines\nQUEUE_KIT_ENTRY_CREDIT_MAX = off')"; rc=$?
[[ "$rc" -eq 1 ]] || note credits-off-exit "a credit under credits off exited $rc: $out"
grep -qF 'credits are off' <<<"$out" || note credits-off "the credit was not refused as off: $out"
out="$(run $'QUEUE_KIT_ENTRY_CAP = 3lines\nQUEUE_KIT_ENTRY_CREDIT_MAX = 5lines')"; rc=$?
[[ "$rc" -eq 0 ]] || note credit-stands "a granted, bounded, needed lines credit did not stand (exit $rc): $out"
out="$(run $'QUEUE_KIT_ENTRY_CAP = 300cp\nQUEUE_KIT_ENTRY_CREDIT_MAX = 100cp')"; rc=$?
[[ "$rc" -eq 1 ]] || note mismatch-exit "a lines credit under a cp cap exited $rc: $out"
grep -qF 'credit in lines but the cap is in cp' <<<"$out" \
    || note mismatch "the unit mismatch was not named: $out"

# --- (5) a malformed value and the retired name refuse at exit 2 --------------------------
checks=$((checks + 1))
queue "$SMALL"
out="$(run 'QUEUE_KIT_ENTRY_CAP = 50')"; rc=$?
[[ "$rc" -eq 2 ]] || note malformed "a unitless cap exited $rc rather than 2: $out"
grep -qF 'QUEUE_KIT_ENTRY_CAP' <<<"$out" || note malformed-name "the refusal did not name the knob: $out"
out="$(run 'QUEUE_KIT_ENTRY_LINE_CAP = 50')"; rc=$?
[[ "$rc" -eq 2 ]] || note retired "the retired knob name exited $rc rather than 2: $out"
grep -qF 'QUEUE_KIT_ENTRY_CAP' <<<"$out" || note retired-successor "the refusal did not name the successor: $out"

if [[ "$fails" -gt 0 ]]; then
    echo "check-queue-entry-budget.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-queue-entry-budget.test.sh: clean (the default cap measures code points, lines counts lines, off disables the size assertion alone, a credit reds under credits off and across units and stands when granted in the cap's unit, and a malformed or retired knob refuses at exit 2; $checks checks)"
exit 0
