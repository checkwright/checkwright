#!/usr/bin/env bash
# spec: queue-kit/SPEC.md §The queue-counts arm — the seam a crate unit test cannot see: that the
# battery runner's --emit front-end resolves the arm at all, and that a set consumer knob actually
# reaches the rendering through it. The discriminating case is the icebox-UNSET one,
# which a hardcoded implementation passes against this repo's own config and fails only here. The
# rendering itself — the derived section set, the top-level-entry unit, Done excluded, renamed
# sections coming back renamed, and --by's value rule and ordering — is pinned in the ported
# module's own #[cfg(test)] tests, where check-crate-arms runs them; duplicating it here would
# assert the same thing twice.
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
RUN_GATES="$ROOT/gate-sdk/bin/run-gates.sh"
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0
checks=0
note() { echo "  FAIL [$1]: $2"; fails=$((fails + 1)); }

cat >"$SANDBOX/TASK-QUEUE.md" <<'EOF'
# TASK-QUEUE.md

## Iteration: demo

## New Features

- **feat-a** [cost: event/low] — do a thing.
- **feat-b** [design-pending] — do another.

## Technical Debt

## Deferred

- **defer-a** [cost: once/high] — later.

## Chill

- **chill-a** — much later.

## Done

- done-a

## Lessons Learned
EOF

# --- the front-end resolves the arm at all ---
checks=$((checks + 1))
out="$( cd "$ROOT" && bash "$RUN_GATES" --emit queue-counts "$SANDBOX/TASK-QUEUE.md" 2>&1 )"; rc=$?
[[ "$rc" -eq 0 ]] || note resolve "the front-end did not resolve --emit queue-counts (exit $rc): $out"
grep -qxF "$(printf 'New Features\t2')" <<<"$out" \
    || note resolve-render "the arm resolved but rendered no tally: $out"

# --- a set consumer knob reaches the rendering: the icebox section ---
# The arm cannot know this section name; it arrives only if the knob was read.
checks=$((checks + 1))
out="$( cd "$ROOT" && env QUEUE_KIT_ICEBOX_SECTION='Chill' \
    bash "$RUN_GATES" --emit queue-counts "$SANDBOX/TASK-QUEUE.md" 2>&1 )"
grep -qxF "$(printf 'Chill\t1')" <<<"$out" \
    || note knob-icebox "a configured icebox section did not reach the arm: $out"

# --- the discriminating case: an emptied icebox knob leaves the tier off the tally entirely ---
checks=$((checks + 1))
out="$( cd "$ROOT" && env QUEUE_KIT_ICEBOX_SECTION= \
    bash "$RUN_GATES" --emit queue-counts "$SANDBOX/TASK-QUEUE.md" 2>&1 )"
grep -q 'Chill' <<<"$out" \
    && note knob-icebox-empty "an emptied icebox knob still tallied the tier: $out"

# --- --by reaches the rendering through the front-end, keyed by the same consumer knob ---
# The flag rides the arm's own argv tail, so the seam worth holding here is that the front-end
# passes it through at all and that the partition still walks the configured section set.
checks=$((checks + 1))
out="$( cd "$ROOT" && env QUEUE_KIT_ICEBOX_SECTION='Chill' \
    bash "$RUN_GATES" --emit queue-counts --by cost "$SANDBOX/TASK-QUEUE.md" 2>&1 )"; rc=$?
[[ "$rc" -eq 0 ]] || note by-resolve "the front-end did not pass --by through (exit $rc): $out"
grep -qxF "$(printf 'New Features/event/low\t1')" <<<"$out" \
    || note by-field "a field tag's value did not key its partition: $out"
grep -qxF "$(printf 'New Features/(none)\t1')" <<<"$out" \
    || note by-none "an entry carrying neither shape did not appear under (none): $out"
grep -qxF "$(printf 'Chill/(none)\t1')" <<<"$out" \
    || note by-knob "the configured icebox section did not reach the partition: $out"
grep -q 'Technical Debt' <<<"$out" \
    && note by-empty-section "a section with no entries emitted a partition line: $out"

# --- the default partition is untouched, which is the in-process caller's whole contract ---
checks=$((checks + 1))
plain="$( cd "$ROOT" && bash "$RUN_GATES" --emit queue-counts "$SANDBOX/TASK-QUEUE.md" 2>&1 )"
grep -qxF "$(printf 'Technical Debt\t0')" <<<"$plain" \
    || note default-unchanged "the default rendering lost its zero row: $plain"

# --- a missing queue file is a refusal, never an empty clean answer ---
checks=$((checks + 1))
( cd "$ROOT" && bash "$RUN_GATES" --emit queue-counts "$SANDBOX/nope.md" >/dev/null 2>&1 )
[[ "$?" -eq 2 ]] || note missing-file "a missing queue file did not exit 2"

if [[ "$fails" -gt 0 ]]; then
    echo "queue-counts.test.sh: $fails case(s) failed"
    exit 1
fi
echo "queue-counts.test.sh: clean (the --emit front-end resolves the arm, a configured icebox section and an emptied one each reach the rendering, --by passes through to a compound key whose value rule and empty-section silence hold while the default partition is untouched, and a missing file refuses; $checks checks)"
exit 0
