#!/usr/bin/env bash
# Behavioral test of bin/queue-counts.sh: the emitted section set is the
# *configured* task sections in configured order — not a fixed four — and the
# count is the top-level entry, the unit the queue-index arm lists. The
# icebox-unset case is the discriminating one: a hardcoded implementation passes
# a run against this repo's own config and fails only here. queue-counts is a
# tool, not a gate, so it has no good/bad pair; this drives it directly.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # queue-kit/
BIN="$DIR/bin/queue-counts.sh"
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT
: >"$SANDBOX/empty-config.sh"
export QUEUE_KIT_CONFIG_FILE="$SANDBOX/empty-config.sh"

fails=0
checks=0

eq() {     # $1=label $2=got $3=want
    checks=$((checks + 1))
    [[ "$2" == "$3" ]] || { echo "  FAIL [$1]: got '$2', want '$3'"; fails=$((fails + 1)); }
}

cat >"$SANDBOX/TASK-QUEUE.md" <<'EOF'
# TASK-QUEUE.md

## Iteration: demo

## New Features

- **feat-a** — do a thing.
  - **not-an-entry** — an indented bullet is body, not a second entry.
- **feat-b** — do another.

## Technical Debt

## Deferred

- **defer-a** — later.

## Chill

- **chill-a** — much later.

## Done

- done-a
- **done-b** — a Done entry shaped like an active one, to prove Done is out.

## Lessons Learned

- **l1** [attend] — a lesson is not a task section.
EOF

# (a) with an icebox configured: the four configured task sections, in order,
#     with Done and Lessons excluded and the indented bullet not counted.
out="$(QUEUE_KIT_ICEBOX_SECTION=Chill bash "$BIN" "$SANDBOX/TASK-QUEUE.md")"
eq "icebox-set" "$(tr '\t' '=' <<<"$out" | paste -sd, -)" \
   'New Features=2,Technical Debt=0,Deferred=1,Chill=1'

# (b) the discriminating case: no icebox configured -> three lines, and the
#     section that is no longer a task section contributes nothing.
out="$(bash "$BIN" "$SANDBOX/TASK-QUEUE.md")"
eq "icebox-unset" "$(tr '\t' '=' <<<"$out" | paste -sd, -)" \
   'New Features=2,Technical Debt=0,Deferred=1'

# (c) renamed sections come back renamed — the tool resolves, never enumerates.
cat >"$SANDBOX/renamed-config.sh" <<'EOF'
QUEUE_KIT_ACTIVE_SECTIONS=("Work")
QUEUE_KIT_DEFERRED_SECTION="Someday"
QUEUE_KIT_ICEBOX_SECTION=""
EOF
cat >"$SANDBOX/renamed.md" <<'EOF'
## Work

- **w1** — one.

## Someday

- **s1** — two.
- **s2** — three.

## Done

- **d1** — not counted.
EOF
out="$(QUEUE_KIT_CONFIG_FILE="$SANDBOX/renamed-config.sh" bash "$BIN" "$SANDBOX/renamed.md")"
eq "renamed" "$(tr '\t' '=' <<<"$out" | paste -sd, -)" 'Work=1,Someday=2'

# (d) over a queue whose bullets are all top-level, the total agrees with
#     queue_live_slugs — the two readers must not disagree about one queue's size.
#     The fixture is the renamed one deliberately: the main fixture's indented
#     decoy is an entry to queue_live_slugs' grammar and a body bullet to this
#     tool's, a latent divergence between two *existing* readers that this test
#     must not silently adopt as agreement.
lib_total="$(QUEUE_KIT_CONFIG_FILE="$SANDBOX/renamed-config.sh" bash -c \
    'source "$1/lib/queue.sh"; queue_live_slugs "$2" | wc -l' _ "$DIR" "$SANDBOX/renamed.md")"
counts_total="$(QUEUE_KIT_CONFIG_FILE="$SANDBOX/renamed-config.sh" bash "$BIN" "$SANDBOX/renamed.md" \
    | awk -F'\t' '{ n += $2 } END { print n + 0 }')"
eq "agrees-with-queue_live_slugs" "$counts_total" "$lib_total"

# (e) a missing queue file is exit 2, never an empty clean answer.
checks=$((checks + 1))
( bash "$BIN" "$SANDBOX/nope.md" >/dev/null 2>&1 )
[[ "$?" -eq 2 ]] || { echo "  FAIL [missing-file]: expected exit 2"; fails=$((fails + 1)); }

if [[ "$fails" -gt 0 ]]; then
    echo "queue-counts.test: $fails of $checks assertion(s) failed"
    exit 1
fi
echo "queue-counts.test: ok ($checks assertions; configured task sections resolved, icebox derived, Done excluded, entry unit agrees with queue_live_slugs)"
exit 0
