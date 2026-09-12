#!/usr/bin/env bash
# spec: queue-kit/SPEC.md §check-queue-entry-budget — the entry-history arm's seeded history: the
# two properties a crate unit test cannot reach, because both are properties of a git history and
# not of a queue file. (1) The walk's BOUND — it stops at the entry's filing commit and not at the
# repository root. (2) The two stated limits — a renamed slug reads as filed at its rename, and a
# commit that compresses one part of an entry while growing another does not appear at all — each
# recorded as behaviour rather than asserted as a claim. The count's single spelling is held here
# too, against check-queue-entry-budget's own headroom line at the same commit, which is the one
# assertion that needs both substrates in the same sandbox.
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
CHECKS="$ROOT/queue-kit/checks"
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0
checks=0
note() { echo "  FAIL [$1]: $2"; fails=$((fails + 1)); }

git -C "$SANDBOX" init -q
git -C "$SANDBOX" config user.email seed@example.invalid
git -C "$SANDBOX" config user.name Seed

queue() {  # $1 = the def-target body lines, $2 = the renamed entry's slug
    cat >"$SANDBOX/TASK-QUEUE.md" <<EOF
# TASK-QUEUE.md

## Iteration: seeded

## New Features

## Technical Debt

## Deferred

- **def-other** [cost: event/low] [surface: none] — an unrelated entry.
  **Cost while deferred** nothing at all.
$1
- **$2** [cost: event/low] [surface: none] — the renamed entry.
  **Cost while deferred** nothing at all.
  one body line

## Done

## Lessons Learned
EOF
}

commit() { git -C "$SANDBOX" add TASK-QUEUE.md && git -C "$SANDBOX" commit -q -m "$1"; }

TARGET_AT_FILING='
- **def-target** [cost: event/low] [surface: none] — the measured entry.
  **Cost while deferred** the grounds it carries.
  ground one
  ground two
  ground three
  ground four'
TARGET_GROWN="$TARGET_AT_FILING
  ground five
  ground six"
TARGET_NET_ZERO='
- **def-target** [cost: event/low] [surface: none] — the measured entry.
  **Cost while deferred** the grounds it carries.
  answer one, replacing two grounds
  a second answer line
  ground three
  ground four
  ground five
  ground six'
TARGET_COMPRESSED='
- **def-target** [cost: event/low] [surface: none] — the measured entry.
  **Cost while deferred** the grounds it carries.
  answered: all six grounds, relocated to `def-other`
  ground six'

# --- the seeded history, oldest first ------------------------------------------------
queue '' def-renamed-old;              commit "seed the queue with no target entry"
queue '' def-renamed-old;              printf '\n' >>"$SANDBOX/TASK-QUEUE.md"
commit "a second pre-filing commit, so a walk to the root is distinguishable"
queue "$TARGET_AT_FILING" def-renamed-old;  commit "file def-target"
queue "$TARGET_GROWN" def-renamed-old;      commit "grow def-target by two grounds"
queue "$TARGET_NET_ZERO" def-renamed-new;   commit "net zero on def-target; rename the other entry"
queue "$TARGET_COMPRESSED" def-renamed-new; commit "compress def-target by answering"

sha() { git -C "$SANDBOX" rev-parse --short=8 "$1"; }
FILING="$(sha HEAD~3)"
NETZERO="$(sha HEAD~1)"
GROWTH="$(sha HEAD~2)"
HEADC="$(sha HEAD)"

arm() { ( cd "$SANDBOX" && gate_arm_run --emit-entry-history "$@" 2>&1 ); }

# --- the walk's bound: it stops at the filing commit, not at the root -----------------
checks=$((checks + 1))
out="$(arm def-target)"; rc=$?
[[ "$rc" -eq 0 ]] || note bound-exit "the arm did not exit 0 on a live entry (exit $rc): $out"
grep -qF "filing commit $FILING" <<<"$out" \
    || note bound-filing "the walk did not stop at the filing commit $FILING: $out"
grep -qF "4 commit(s) walked" <<<"$out" \
    || note bound-count "the walk did not stop after the four commits from filing to HEAD: $out"

# --- the decrease is reported, with its before, its after and its subject -------------
checks=$((checks + 1))
grep -qE "^  $HEADC  8 -> 4  compress def-target by answering\$" <<<"$out" \
    || note row "the compressing commit's row is not the reported one: $out"

# --- limit: a commit that grows and one that nets out do not appear -------------------
checks=$((checks + 1))
grep -qF "$GROWTH" <<<"$out" && note growth "a growing commit was reported as a fall: $out"
grep -qF "$NETZERO" <<<"$out" \
    && note net-zero "a commit that compressed one part while growing another was reported: $out"
[[ "$(grep -cE '^  [0-9a-f]{8}  ' <<<"$out")" -eq 1 ]] \
    || note row-count "exactly one commit fell in the seeded history; the report disagrees: $out"

# --- limit: a renamed slug reads as filed at its rename -------------------------------
checks=$((checks + 1))
ren="$(arm def-renamed-new)"; rc=$?
[[ "$rc" -eq 0 ]] || note rename-exit "the arm did not exit 0 on the renamed slug (exit $rc): $ren"
grep -qF "filing commit $NETZERO" <<<"$ren" \
    || note rename "the renamed slug did not read as filed at its rename $NETZERO: $ren"
grep -qF "2 commit(s) walked" <<<"$ren" \
    || note rename-count "the renamed slug's walk did not stop at the rename: $ren"

# --- one spelling of the count: the arm's `after` is the cap's own count --------------
# The gate prints headroom, which is cap minus that count, so the two are one subtraction apart.
checks=$((checks + 1))
after="$(grep -E "^  $HEADC  " <<<"$out" | awk '{print $4}')"
head_out="$( cd "$SANDBOX" && gate_env QUEUE_KIT_ENTRY_LINE_CAP=50 \
    && gate_run check-queue-entry-budget "$CHECKS" "$SANDBOX/TASK-QUEUE.md" 2>&1 )"
headroom="$(grep -E '^  def-target: [0-9]+ lines of headroom' <<<"$head_out" | awk '{print $2}')"
if [[ -z "$after" || -z "$headroom" ]]; then
    note one-spelling-read "could not read both numbers (after='$after' headroom='$headroom'): $head_out"
elif [[ "$after" -ne $((50 - headroom)) ]]; then
    note one-spelling "the arm's count $after is not the cap's count $((50 - headroom))"
fi

# --- the exit contract is two-valued: 0 with a report, 2 on a usage error, never 1 ----
checks=$((checks + 1))
for bad in "" no-such-slug-at-all; do
    if [[ -z "$bad" ]]; then out2="$(arm)"; else out2="$(arm "$bad")"; fi
    rc=$?
    [[ "$rc" -eq 2 ]] || note usage "a usage error exited $rc rather than 2: $out2"
done

if [[ "$fails" -gt 0 ]]; then
    echo "entry-history.test.sh: $fails case(s) failed"
    exit 1
fi
echo "entry-history.test.sh: clean (the walk stops at the filing commit and not at the root, a growing and a net-zero commit are both absent from the report, a renamed slug reads as filed at its rename, the arm's count is check-queue-entry-budget's own, and the exit contract is 0-or-2 with no 1; $checks checks)"
exit 0
