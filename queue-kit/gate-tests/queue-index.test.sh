#!/usr/bin/env bash
# spec: queue-kit/SPEC.md §The queue-index arm — the seam a crate unit test cannot see: that the
# battery runner's --emit front-end resolves the arm at all, and that a set consumer knob actually
# reaches the rendering through the shell bridge. The rendering itself is pinned in the ported
# module's own #[cfg(test)] tests, where check-crate-arms runs them; duplicating it here would
# assert the same thing twice and hold neither end of this seam.
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
- **feat-b** [attend-not-a-tag] — another.

## Technical Debt

## Deferred

- **def-a** — a deferred entry.

## Cold Storage

- **ice-a** — an iceboxed entry.
- **ice-b** — a second iceboxed entry.

## Done

## Lessons Learned

- **l1** [attend] — first attention point
- **l2** [attend] — second attention point
EOF

emit() { ( cd "$ROOT" && bash "$RUN_GATES" --emit queue-index "$@" 2>&1 ); }

# --- the front-end resolves the arm at all ---
checks=$((checks + 1))
out="$(emit "$SANDBOX/TASK-QUEUE.md")"; rc=$?
[[ "$rc" -eq 0 ]] || note resolve "the front-end did not resolve --emit queue-index (exit $rc): $out"
grep -qF 'Active (pick the first •):' <<<"$out" \
    || note resolve-render "the arm resolved but rendered no index: $out"

# --- a set consumer knob reaches the rendering through the bridge: the icebox section ---
# The arm cannot know this section name; it arrives only if the bridge carried it.
checks=$((checks + 1))
out="$( cd "$ROOT" && env QUEUE_KIT_ICEBOX_SECTION='Cold Storage' \
    bash "$RUN_GATES" --emit queue-index "$SANDBOX/TASK-QUEUE.md" 2>&1 )"
grep -qF 'Cold Storage: 2 entries' <<<"$out" \
    || note bridge-icebox "a configured icebox section did not reach the arm through the bridge: $out"

checks=$((checks + 1))
out="$( cd "$ROOT" && env QUEUE_KIT_ICEBOX_SECTION= \
    bash "$RUN_GATES" --emit queue-index "$SANDBOX/TASK-QUEUE.md" 2>&1 )"
grep -qF 'entries' <<<"$out" \
    && note bridge-icebox-empty "an empty icebox knob still printed a tally: $out"

# --- a set consumer knob reaches the rendering through the bridge: the attend cap ---
checks=$((checks + 1))
out="$( cd "$ROOT" && env QUEUE_KIT_ATTEND_CAP=1 \
    bash "$RUN_GATES" --emit queue-index "$SANDBOX/TASK-QUEUE.md" 2>&1 )"
grep -qF '(+1 more [attend])' <<<"$out" \
    || note bridge-cap "a lowered attend cap did not reach the arm through the bridge: $out"

# --- the other two modes reach the arm through the same front-end ---
checks=$((checks + 1))
out="$(emit --extent def-a "$SANDBOX/TASK-QUEUE.md")"
[[ "$out" =~ ^[0-9]+\ [0-9]+$ ]] \
    || note bridge-extent "--extent did not reach the arm through the front-end: $out"

checks=$((checks + 1))
out="$(emit --icebox-candidates "$SANDBOX/TASK-QUEUE.md")"; rc=$?
[[ "$rc" -eq 0 ]] || note bridge-candidates "--icebox-candidates did not reach the arm (exit $rc): $out"

if [[ "$fails" -gt 0 ]]; then
    echo "queue-index.test.sh: $fails case(s) failed"
    exit 1
fi
echo "queue-index.test.sh: clean (the --emit front-end resolves the arm, and a configured icebox section, an emptied one and a lowered attend cap each reach the rendering through the bridge; $checks checks)"
exit 0
