#!/usr/bin/env bash
# spec: lifecycle-kit/SPEC.md §The committed gap inbox — file-gap.sh's capture-time slug resolution: a live entry's slug in the prose stamps the recurrence marker and warns on stderr, a done slug and a Lessons-only slug do not, the longest live slug wins, a hyphen-embedded near-miss is not a match, and no run touches the queue file
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

filed() {   # $1=prose -> prints "<stdout>|<stderr>" of one filing into a fresh inbox
    local prose="$1" inbox="$SANDBOX/inbox-$RANDOM.md" o e
    o="$(cd "$SANDBOX" && env LIFECYCLE_KIT_GAP_INBOX_FILE="$inbox" \
            LIFECYCLE_KIT_QUEUE_FILE="$SANDBOX/TASK-QUEUE.md" \
            bash "$TOOL" "$prose" 2>"$SANDBOX/err.txt")"
    e="$(cat "$SANDBOX/err.txt")"
    printf '%s|%s\n' "$o" "$e"
}

qbefore="$(cat "$SANDBOX/TASK-QUEUE.md")"

# --- a live deferred slug matches, and the longest match wins over its prefix ---
out="$(filed 'the fork-dispatch-prohibition failure mode fired again inside scope')"
grep -qF -- "- $DATE — recurrence of \`fork-dispatch-prohibition\`: " <<<"$out" \
    || note live-match "no recurrence marker for a live deferred slug: $out"
grep -qF 'already filed under `fork-dispatch-prohibition`' <<<"$out" \
    || note live-warn "no point-of-capture stderr warning: $out"

# --- an icebox entry is live too ---
out="$(filed 'iced-entry regressed')"
grep -qF 'recurrence of `iced-entry`' <<<"$out" || note icebox "an icebox entry did not resolve as live: $out"

# --- a done slug is a new defect, not a recurrence ---
out="$(filed 'landed-and-fixed broke again after its fix landed')"
grep -qF 'recurrence of' <<<"$out" && note done-slug "a done-section slug resolved as a recurrence: $out"
grep -qF -- "- $DATE — landed-and-fixed broke again" <<<"$out" \
    || note done-plain "the plain bullet grammar was not written for a done slug: $out"

# --- a Lessons entry written in the entry shape is not a queue entry ---
out="$(filed 'lesson-shaped-slug came up once more')"
grep -qF 'recurrence of' <<<"$out" && note lesson "a Lessons lead line resolved as a live queue entry: $out"

# --- a sub-task is out of scope: the rule and its threshold are entry-scoped ---
out="$(filed 'nested-subtask surfaced again')"
grep -qF 'recurrence of' <<<"$out" && note subtask "an indented sub-task resolved as an entry: $out"

# --- a hyphen-embedded near-miss is not a match ---
out="$(filed 'the fork-dispatching helper is slow')"
grep -qF 'recurrence of' <<<"$out" && note boundary "a slug embedded in a longer hyphenated token matched: $out"

# --- no filing ever writes the queue ---
[[ "$qbefore" == "$(cat "$SANDBOX/TASK-QUEUE.md")" ]] \
    || note no-queue-write "file-gap.sh modified the queue file — the constraint the gap inbox exists to hold"

[[ "$fails" -eq 0 ]] || { echo "file-gap-recurrence.test: $fails assertion(s) failed"; exit 1; }
echo "file-gap-recurrence.test: clean (live-slug resolution stamps the marker and warns, done/Lessons/sub-task slugs do not, longest match wins, and no filing writes the queue)"
exit 0
