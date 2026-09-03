#!/usr/bin/env bash
# spec: lifecycle-kit/SPEC.md §The committed gap inbox — the end-to-end half of the capture arm's contract, the seam a crate unit test cannot see: the bridged knobs reach the arm through the front-end, every filing writes the one plain '- <date> — <prose>' bullet with no verdict interposed, a live entry's slug in the prose asks the filer on stderr, and NO FILING TOUCHES THE QUEUE FILE — the invariant the inbox exists to hold and the one with no other holder. The grammar cases (done, Lessons, sub-task, hyphen-embedded near-miss, longest-match, icebox, a denying prose) are pinned in native/src/emit/file_gap.rs's own #[cfg(test)] tests, where check-crate-arms runs them.
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk" && pwd)"
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0
note() { echo "  FAIL [$1]: $2"; fails=$((fails + 1)); }

cat >"$SANDBOX/TASK-QUEUE.md" <<'EOF'
# TASK-QUEUE.md

## Iteration: demo-iteration

---

## New Features

- **fork-dispatch** — an active entry whose slug is a hyphen-prefix of a deferred one.

## Technical Debt

## Deferred

- **fork-dispatch-prohibition** [design-pending] — the deferred entry.
  Cost while deferred: recovery is re-paid each time.

## Icebox

## Done

- landed-and-fixed

## Lessons Learned
EOF

DATE="$(date +%F)"
OUT=""; ERR=""

filed() {   # $1=prose -> sets OUT (stdout) and ERR (stderr) of one filing into a fresh inbox
    local prose="$1" inbox="$SANDBOX/inbox-$RANDOM.md"
    OUT="$(env LIFECYCLE_KIT_GAP_INBOX_FILE="$inbox" \
            LIFECYCLE_KIT_QUEUE_FILE="$SANDBOX/TASK-QUEUE.md" \
            bash "$SDK/bin/run-gates.sh" --emit file-gap "$prose" 2>"$SANDBOX/err.txt")"
    ERR="$(cat "$SANDBOX/err.txt")"
    LAST_INBOX="$inbox"
}

# the advisory's distinguishing phrase — every filing's stderr also carries the
# unconditional drain-window warning, so silence is asserted against this alone
ASKS='names live entry'

plain() {   # $1=arm, $2=the prose the bullet must carry verbatim after the date
    grep -qxF -- "file-gap: - $DATE — $2" <<<"$OUT" \
        || note "$1-plain" "the bullet is not the plain '- <date> — <prose>' shape: $OUT"
    grep -q -- '^file-gap: - [0-9-]* — recurrence of' <<<"$OUT" \
        && note "$1-verdict" "the tool interposed a recurrence verdict into the bullet: $OUT"
}

qbefore="$(cat "$SANDBOX/TASK-QUEUE.md")"

# --- a live slug asks the filer, and the bullet it accompanies stays plain ---
p='the fork-dispatch-prohibition failure mode fired again inside scope'
filed "$p"
plain live-match "$p"
grep -qF "$ASKS \`fork-dispatch-prohibition\`" <<<"$ERR" \
    || note live-ask "no point-of-capture advisory naming the live slug: $ERR"
grep -qF 'RE-FILES' <<<"$ERR" \
    || note live-question "the advisory does not ask the filer to state the claim: $ERR"

# --- the SET consumer knobs reach the arm through the bridge, which a crate test cannot show ---
grep -qxF -- "- $DATE — $p" "$LAST_INBOX" \
    || note knob-reach "a set LIFECYCLE_KIT_GAP_INBOX_FILE did not reach the arm through the bridge"
head -1 "$LAST_INBOX" | grep -qF '# contract: lifecycle-kit/SPEC.md §The committed gap inbox' \
    || note seeded-header "a fresh inbox was not seeded with the contract header: $(head -1 "$LAST_INBOX")"

# --- a done slug is a new defect, not a recurrence: the resolver reaches the queue and stays silent ---
p='landed-and-fixed broke again after its fix landed'
filed "$p"
plain done-slug "$p"
grep -qF "$ASKS" <<<"$ERR" && note done-ask "a done-section slug raised the recurrence advisory: $ERR"

# --- no filing ever writes the queue ---
[[ "$qbefore" == "$(cat "$SANDBOX/TASK-QUEUE.md")" ]] \
    || note no-queue-write "the arm modified the queue file — the constraint the gap inbox exists to hold"

[[ "$fails" -eq 0 ]] || { echo "file-gap-recurrence.test: $fails assertion(s) failed"; exit 1; }
echo "file-gap-recurrence.test: clean (the bridged knobs reach the arm, every filing writes the plain bullet and no verdict, a fresh inbox is seeded with the contract header, a live slug asks the filer while a done slug stays silent, and no filing writes the queue)"
exit 0
