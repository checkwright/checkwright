#!/usr/bin/env bash
# spec: queue-kit/SPEC.md §The queue-counts arm — the seam a crate unit test cannot see: that the
# battery runner's --emit front-end resolves the arm at all, and that a set consumer knob actually
# reaches the rendering through the shell bridge. The discriminating case is the icebox-UNSET one,
# which a hardcoded implementation passes against this repo's own config and fails only here. The
# rendering itself — the derived section set, the top-level-entry unit, Done excluded, renamed
# sections coming back renamed — is pinned in the ported module's own #[cfg(test)] tests, where
# check-crate-arms runs them; duplicating it here would assert the same thing twice.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
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

- **feat-a** — do a thing.
- **feat-b** — do another.

## Technical Debt

## Deferred

- **defer-a** — later.

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

# --- a set consumer knob reaches the rendering through the bridge: the icebox section ---
# The arm cannot know this section name; it arrives only if the bridge carried it.
checks=$((checks + 1))
out="$( cd "$ROOT" && env QUEUE_KIT_ICEBOX_SECTION='Chill' \
    bash "$RUN_GATES" --emit queue-counts "$SANDBOX/TASK-QUEUE.md" 2>&1 )"
grep -qxF "$(printf 'Chill\t1')" <<<"$out" \
    || note bridge-icebox "a configured icebox section did not reach the arm through the bridge: $out"

# --- the discriminating case: an emptied icebox knob leaves the tier off the tally entirely ---
checks=$((checks + 1))
out="$( cd "$ROOT" && env QUEUE_KIT_ICEBOX_SECTION= \
    bash "$RUN_GATES" --emit queue-counts "$SANDBOX/TASK-QUEUE.md" 2>&1 )"
grep -q 'Chill' <<<"$out" \
    && note bridge-icebox-empty "an emptied icebox knob still tallied the tier: $out"

# --- a missing queue file is a refusal, never an empty clean answer ---
checks=$((checks + 1))
( cd "$ROOT" && bash "$RUN_GATES" --emit queue-counts "$SANDBOX/nope.md" >/dev/null 2>&1 )
[[ "$?" -eq 2 ]] || note missing-file "a missing queue file did not exit 2"

if [[ "$fails" -gt 0 ]]; then
    echo "queue-counts.test.sh: $fails case(s) failed"
    exit 1
fi
echo "queue-counts.test.sh: clean (the --emit front-end resolves the arm, a configured icebox section and an emptied one each reach the rendering through the bridge, and a missing file refuses; $checks checks)"
exit 0
