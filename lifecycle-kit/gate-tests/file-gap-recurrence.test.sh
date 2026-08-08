#!/usr/bin/env bash
# spec: lifecycle-kit/SPEC.md §The committed gap inbox — file-gap.sh's capture-time slug resolution raises an advisory and never a verdict: every filing writes the one plain '- <date> — <prose>' bullet, a live entry's slug in the prose asks the filer on stderr, a done slug and a Lessons-only slug and a sub-task and a hyphen-embedded near-miss ask nothing, the longest live slug is the one the advisory names, prose that denies the recurrence in words is left exactly as written, and no run touches the queue file
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # lifecycle-kit/
TOOL="$DIR/bin/file-gap.sh"
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
  - **nested-subtask** — a sub-task, indented, deliberately out of the entry scan.

## Icebox

- **iced-entry** — one line, still live work.

## Done

- landed-and-fixed

## Lessons Learned

- **lesson-shaped-slug** [attend] — a lesson written in the entry shape.
EOF

DATE="$(date +%F)"
OUT=""; ERR=""

filed() {   # $1=prose -> sets OUT (stdout) and ERR (stderr) of one filing into a fresh inbox
    local prose="$1" inbox="$SANDBOX/inbox-$RANDOM.md"
    OUT="$(cd "$SANDBOX" && env LIFECYCLE_KIT_GAP_INBOX_FILE="$inbox" \
            LIFECYCLE_KIT_QUEUE_FILE="$SANDBOX/TASK-QUEUE.md" \
            bash "$TOOL" "$prose" 2>"$SANDBOX/err.txt")"
    ERR="$(cat "$SANDBOX/err.txt")"
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

# --- a live deferred slug asks, the bullet stays plain, and the longest match wins ---
p='the fork-dispatch-prohibition failure mode fired again inside scope'
filed "$p"
plain live-match "$p"
grep -qF "$ASKS \`fork-dispatch-prohibition\`" <<<"$ERR" \
    || note live-ask "no point-of-capture advisory naming the longest live slug: $ERR"
grep -qF 'RE-FILES' <<<"$ERR" \
    || note live-question "the advisory does not ask the filer to state the claim: $ERR"

# --- an icebox entry is live too ---
p='iced-entry regressed'
filed "$p"
plain icebox "$p"
grep -qF "$ASKS \`iced-entry\`" <<<"$ERR" || note icebox-ask "an icebox entry did not resolve as live: $ERR"

# --- a done slug is a new defect, not a recurrence: no advisory at all ---
p='landed-and-fixed broke again after its fix landed'
filed "$p"
plain done-slug "$p"
grep -qF "$ASKS" <<<"$ERR" && note done-ask "a done-section slug raised the recurrence advisory: $ERR"

# --- a Lessons entry written in the entry shape is not a queue entry ---
p='lesson-shaped-slug came up once more'
filed "$p"
plain lesson "$p"
grep -qF "$ASKS" <<<"$ERR" && note lesson-ask "a Lessons lead line resolved as a live queue entry: $ERR"

# --- a sub-task is out of scope: the rule and its threshold are entry-scoped ---
p='nested-subtask surfaced again'
filed "$p"
plain subtask "$p"
grep -qF "$ASKS" <<<"$ERR" && note subtask-ask "an indented sub-task resolved as an entry: $ERR"

# --- a hyphen-embedded near-miss is not a match ---
p='the fork-dispatching helper is slow'
filed "$p"
plain boundary "$p"
grep -qF "$ASKS" <<<"$ERR" && note boundary-ask "a slug embedded in a longer hyphenated token matched: $ERR"

# --- the live instance, frozen: prose that denies the recurrence in words is left as written ---
p='the harness grows unbounded and nothing prunes it. NOT a recurrence of `fork-dispatch-prohibition` — the filer ruled this must be a separate cross-referencing entry.'
filed "$p"
plain denial "$p"
grep -qF "$ASKS \`fork-dispatch-prohibition\`" <<<"$ERR" \
    || note denial-ask "the advisory should still ask on a denying bullet — asking is not asserting: $ERR"

# --- no filing ever writes the queue ---
[[ "$qbefore" == "$(cat "$SANDBOX/TASK-QUEUE.md")" ]] \
    || note no-queue-write "file-gap.sh modified the queue file — the constraint the gap inbox exists to hold"

[[ "$fails" -eq 0 ]] || { echo "file-gap-recurrence.test: $fails assertion(s) failed"; exit 1; }
echo "file-gap-recurrence.test: clean (every filing writes the plain bullet and no verdict, a live slug asks the filer while done/Lessons/sub-task/near-miss slugs stay silent, the longest live slug is the one named, a denying prose survives verbatim and is still asked about, and no filing writes the queue)"
exit 0
