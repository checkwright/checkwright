#!/usr/bin/env bash
# spec: queue-kit/SPEC.md §The queue-edges arm — the seam a crate unit test cannot see: that the
# battery runner's --emit front-end resolves the arm at all, and the two halves that need a real
# sandbox and a real history — the no-repository degradation and the retired block. The citation
# grammar itself (live-set resolution, unresolved token, self-citation, [blocked-by:], verbatim
# line, nearest-preceding-bullet attribution, retired ordering) is pinned in the ported module's
# own #[cfg(test)] tests, where check-crate-arms runs them.
#
# The first sandbox is deliberately NOT a git repository: it is the no-repository arm. The retired
# half needs history, so it gets a second sandbox with two revisions.
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
edges() { ( cd "$ROOT" && bash "$RUN_GATES" --emit queue-edges "$@" 2>&1 ); }

cat >"$SANDBOX/TASK-QUEUE.md" <<'EOF'
# TASK-QUEUE.md

## Iteration: demo

## New Features

- **feat-a** — a feature citing `feat-b` on its lead line.
  **Relation to `feat-b`:** this one subsumes it entirely.

- **feat-b** [blocked-by: def-a] — blocked, so it cites its blocker.

## Technical Debt

## Deferred

- **def-a** — a deferred entry is a live target.

## Done

## Lessons Learned
EOF

Q="$SANDBOX/TASK-QUEUE.md"

# --- the front-end resolves the arm at all ---
checks=$((checks + 1))
out="$(edges "$Q")"; rc=$?
[[ "$rc" -eq 0 ]] || note resolve "the front-end did not resolve --emit queue-edges (exit $rc): $out"
grep -qF 'feat-b (1 inbound)' <<<"$out" \
    || note resolve-render "the arm resolved but aggregated nothing: $out"

# --- the declared no-repository degradation: this sandbox has no git history behind it, so the
# retired set is empty and the arm prints its live block alone — byte for byte what it printed
# before retired targets existed. Asserted rather than inferred, because "silent-safe" is only
# true if the silence is total.
checks=$((checks + 1))
grep -qF ', retired)' <<<"$out" \
    && note no-repo-no-retired-block "a sandbox outside a work tree still printed a retired block: $out"

# --- a set consumer knob reaches the rendering through the bridge: the deferred section name.
# Renamed away, def-a is no longer in a task section, so it is no longer a live target at all.
checks=$((checks + 1))
out="$( cd "$ROOT" && env QUEUE_KIT_DEFERRED_SECTION='Someday' \
    bash "$RUN_GATES" --emit queue-edges "$Q" 2>&1 )"
grep -qF 'def-a (' <<<"$out" \
    && note bridge-deferred "a renamed deferred section did not reach the arm through the bridge: $out"

# --- the arm mutates nothing ---
checks=$((checks + 1))
before="$(cksum <"$Q")"
edges "$Q" >/dev/null 2>&1
[[ "$(cksum <"$Q")" == "$before" ]] || note no-mutation "the queue file changed"

# ---------------------------------------------------------------------------
# The retired half. A second sandbox, this one a git repository, whose queue file has two
# revisions: `gone-a` held a lead line in revision 1 and holds none in revision 2, which is the
# definition of retired. `landed-thing` never held one.
GITBOX="$SANDBOX/repo"
mkdir -p "$GITBOX"
git -C "$GITBOX" init -q 2>/dev/null
gitc() { git -C "$GITBOX" -c user.name=t -c user.email=t@t -c commit.gpgsign=false "$@"; }

cat >"$GITBOX/TASK-QUEUE.md" <<'EOF'
# TASK-QUEUE.md

## Iteration: demo

## New Features

- **feat-a** — a live entry.

## Technical Debt

## Deferred

- **gone-a** — an entry that will be disposed of in the next revision.

## Done

## Lessons Learned
EOF
gitc add TASK-QUEUE.md >/dev/null 2>&1
gitc commit -q --no-verify -m r1 >/dev/null 2>&1

cat >"$GITBOX/TASK-QUEUE.md" <<'EOF'
# TASK-QUEUE.md

## Iteration: demo

## New Features

- **feat-a** — a live entry.
  Its body cites `gone-a`, disposed of one revision ago, and also
  `landed-thing`, a token that never held a lead line at all.

## Technical Debt

- **debt-a** — a second citer, so the retired count is not always one.
  Sequence against `gone-a` rather than duplicating it, and defer to `feat-a`.

## Deferred

## Done

- gone-a

## Lessons Learned
EOF
gitc add TASK-QUEUE.md >/dev/null 2>&1
gitc commit -q --no-verify -m r2 >/dev/null 2>&1

GQ="$GITBOX/TASK-QUEUE.md"
gout="$(edges "$GQ")"

# A citation of a retired slug becomes an edge, marked as such and counted from real history.
checks=$((checks + 1))
grep -qF 'gone-a (2 inbound, retired)' <<<"$gout" \
    || note retired-target "the history walk found no retired target: $gout"

# The floor holds: a token that was never a slug stays off it. This is the whole discriminator —
# without it the report is dominated by SHAs and ordinary words.
checks=$((checks + 1))
grep -qF 'landed-thing (' <<<"$gout" \
    && note never-live-floor "a token that never held a lead line became a target: $gout"

# The retired block trails the live one, which is what lets a reader stop at the live block.
checks=$((checks + 1))
live_at="$(grep -n 'feat-a (1 inbound)' <<<"$gout" | head -1 | cut -d: -f1)"
ret_at="$(grep -n 'gone-a (2 inbound, retired)' <<<"$gout" | head -1 | cut -d: -f1)"
[[ -n "$live_at" && -n "$ret_at" && "$ret_at" -gt "$live_at" ]] \
    || note retired-block-trails "live at '$live_at', retired at '$ret_at'"

# --inbound widens with it: a retired slug is addressable, a never-live token is still a caller
# error, so silence keeps meaning "no inbound edges". The refusal is exit 2 on this substrate —
# the emitter type returns a Result and the dispatcher maps every error arm to 2.
checks=$((checks + 1))
gout_r="$(edges --inbound gone-a "$GQ")"; rc=$?
[[ "$rc" -eq 0 ]] || note retired-inbound-rc "--inbound on a retired slug exited $rc"
grep -qF 'gone-a (2 inbound, retired)' <<<"$gout_r" \
    || note retired-inbound "--inbound did not reach the retired domain: $gout_r"

checks=$((checks + 1))
gerr="$(edges --inbound landed-thing "$GQ")"; rc=$?
[[ "$rc" -eq 2 ]] || note never-live-inbound-rc "a never-live --inbound slug exited $rc, want 2"
grep -qF 'not a live or retired slug: landed-thing' <<<"$gerr" \
    || note never-live-inbound-msg "the refusal did not name the slug: $gerr"

if [[ "$fails" -gt 0 ]]; then
    echo "queue-edges.test.sh: $fails case(s) failed"
    exit 1
fi
echo "queue-edges.test.sh: clean (the --emit front-end resolves the arm, a renamed deferred section reaches it through the bridge, the no-repository degradation is total, and a real two-revision history yields the retired target, the never-live floor, the trailing block and both --inbound domains; $checks checks)"
exit 0
