#!/usr/bin/env bash
# Behavioral test of bin/queue-edges.sh: the citation grammar's four rules
# (live-set resolution, unresolved token is not an error, no self-citation,
# [blocked-by:] counts), the verbatim citing line, nearest-preceding-bullet
# attribution, --inbound's silence-means-no-edges contract, and the retired-target
# half — the retired block, the never-live floor beside it, and the declared
# no-repository degradation. queue-edges is a tool, not a gate, so it has no
# good/bad pair; this drives it directly. The repo's own corpus carries no live
# [blocked-by:] tag, so that path exists here or nowhere. Config is isolated via
# QUEUE_KIT_CONFIG_FILE so the repo's queue-config.sh does not leak in.
#
# The first sandbox is deliberately NOT a git repository: it is the no-repository
# arm, and it also keeps every pre-existing assertion reading the live block alone.
# The retired half needs history, so it gets a second sandbox with two revisions.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # queue-kit/
EDGES="$DIR/bin/queue-edges.sh"
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT
: >"$SANDBOX/empty-config.sh"
export QUEUE_KIT_CONFIG_FILE="$SANDBOX/empty-config.sh"

fails=0
checks=0

want() {   # $1=label $2=output $3=substring-that-must-be-present
    checks=$((checks + 1))
    grep -qF -- "$3" <<<"$2" || { echo "  FAIL [$1]: output lacks '$3':"; printf '    %s\n' "$2"; fails=$((fails + 1)); }
}
absent() { # $1=label $2=output $3=substring-that-must-be-absent
    checks=$((checks + 1))
    grep -qF -- "$3" <<<"$2" && { echo "  FAIL [$1]: output should not contain '$3':"; printf '    %s\n' "$2"; fails=$((fails + 1)); } || true
}
rc_is() {  # $1=label $2=actual-rc $3=expected-rc
    checks=$((checks + 1))
    [[ "$2" == "$3" ]] || { echo "  FAIL [$1]: exit $2, want $3"; fails=$((fails + 1)); }
}

cat >"$SANDBOX/TASK-QUEUE.md" <<'EOF'
# TASK-QUEUE.md

## Iteration: demo

## New Features

- **feat-a** — a feature citing `feat-b` on its lead line.
  **Relation to `feat-b`:** this one subsumes it entirely.
  It also mentions `landed-thing`, which is not a live slug, and `feat-a`.
  - **feat-a-sub** — a sub-task.
    It cites `def-a` in its own name.

- **feat-b** [blocked-by: def-a] — blocked, so it cites its blocker.

## Technical Debt

- **debt-a** — an entry nobody cites.
  It mentions `feat-b` once more.

## Deferred

- **def-a** — a deferred entry is a live target.

## Done

- done-slug

## Lessons Learned
EOF

Q="$SANDBOX/TASK-QUEUE.md"

# Default listing: grouped by target, in queue order, with per-target counts.
out="$(bash "$EDGES" "$Q")"
want "count-feat-b"  "$out" "feat-b (2 inbound)"
want "count-def-a"   "$out" "def-a (2 inbound)"

# The citing line is carried verbatim, which is what supplies the relation's kind.
want "verbatim-kind" "$out" "**Relation to \`feat-b\`:** this one subsumes it entirely."

# Rule: an unresolved token is not an edge and not an error (exit 0, no mention).
rc=0; bash "$EDGES" "$Q" >/dev/null 2>&1 || rc=$?
rc_is  "unresolved-rc"   "$rc" 0
absent "unresolved-token" "$out" "landed-thing ("

# Rule: self-citation is not an edge -- feat-a cites only feat-b and never itself.
absent "no-self-target" "$out" "feat-a ("

# Rule: a done slug is not live, so it is never a target.
absent "done-not-live" "$out" "done-slug ("

# Rule: [blocked-by: <slug>] is an edge -- feat-b's tag cites def-a.
out_defa="$(bash "$EDGES" --inbound def-a "$Q")"
want "blocked-by-edge" "$out_defa" "feat-b"
want "blocked-by-line" "$out_defa" "[blocked-by: def-a]"

# Attribution is to the nearest preceding slug bullet: the sub-task cites in its
# own name, not its parent's.
want   "subtask-cites"     "$out_defa" "feat-a-sub"
absent "parent-not-credited" "$out_defa" "  feat-a  "

# A lead line yields its [blocked-by:] tag alone, never its prose: feat-a's lead
# line names feat-b, but only the body citation and debt-a's are edges.
out_featb="$(bash "$EDGES" --inbound feat-b "$Q")"
want   "body-citation"  "$out_featb" "subsumes it entirely"
want   "other-citer"    "$out_featb" "debt-a"
absent "lead-line-prose" "$out_featb" "a feature citing"

# --inbound on a live slug with no inbound edges: empty output, exit 0. Silence
# means "no inbound edges" -- so a dead slug must exit 1 instead, never print
# nothing.
rc=0; out_none="$(bash "$EDGES" --inbound debt-a "$Q" 2>/dev/null)" || rc=$?
rc_is "live-no-edges-rc" "$rc" 0
checks=$((checks + 1))
[[ -z "$out_none" ]] || { echo "  FAIL [live-no-edges-empty]: want empty, got: $out_none"; fails=$((fails + 1)); }

rc=0; err="$(bash "$EDGES" --inbound no-such-slug "$Q" 2>&1 >/dev/null)" || rc=$?
rc_is "dead-slug-rc"  "$rc" 1
want  "dead-slug-msg" "$err" "not a live or retired slug: no-such-slug"

# The tool mutates nothing.
before="$(cksum <"$Q")"
bash "$EDGES" "$Q" >/dev/null 2>&1
checks=$((checks + 1))
[[ "$(cksum <"$Q")" == "$before" ]] || { echo "  FAIL [no-mutation]: the queue file changed"; fails=$((fails + 1)); }

# The declared no-repository degradation: this sandbox has no git history behind
# it, so the retired set is empty and the tool prints its live block alone --
# byte-for-byte what it printed before the retired half existed. Asserted rather
# than inferred, because "silent-safe" is only true if the silence is total.
absent "no-repo-no-retired-block" "$out" ", retired)"

# ---------------------------------------------------------------------------
# The retired half. A second sandbox, this one a git repository, whose queue file
# has two revisions: `gone-a` held a lead line in revision 1 and holds none in
# revision 2, which is the definition of retired. `landed-thing` never held one.
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
gout="$(bash "$EDGES" "$GQ")"

# A citation of a retired slug becomes an edge, marked as such and counted.
want "retired-target"   "$gout" "gone-a (2 inbound, retired)"
want "retired-edge"     "$gout" "Sequence against \`gone-a\` rather than duplicating it"

# The floor holds: a token that was never a slug stays off it. This is the whole
# discriminator -- without it the report is dominated by SHAs and ordinary words.
absent "never-live-floor" "$gout" "landed-thing ("

# A live target keeps today's unmarked line, and the retired block trails it --
# the ordering is what lets a reader stop at the live block and read no further.
want   "live-target-unmarked" "$gout" "feat-a (1 inbound)"
absent "live-target-unretired" "$gout" "feat-a (1 inbound, retired)"
live_at="$(grep -n 'feat-a (1 inbound)' <<<"$gout" | head -1 | cut -d: -f1)"
ret_at="$(grep -n 'gone-a (2 inbound, retired)' <<<"$gout" | head -1 | cut -d: -f1)"
checks=$((checks + 1))
[[ -n "$live_at" && -n "$ret_at" && "$ret_at" -gt "$live_at" ]] \
    || { echo "  FAIL [retired-block-trails]: live at '$live_at', retired at '$ret_at'"; fails=$((fails + 1)); }

# --inbound widens with it: a retired slug is addressable, a never-live token is
# still the caller error it was, so silence keeps meaning "no inbound edges".
rc=0; gout_r="$(bash "$EDGES" --inbound gone-a "$GQ")" || rc=$?
rc_is "retired-inbound-rc" "$rc" 0
want  "retired-inbound"    "$gout_r" "gone-a (2 inbound, retired)"

rc=0; gerr="$(bash "$EDGES" --inbound landed-thing "$GQ" 2>&1 >/dev/null)" || rc=$?
rc_is "never-live-inbound-rc"  "$rc" 1
want  "never-live-inbound-msg" "$gerr" "not a live or retired slug: landed-thing"

if [[ "$fails" -gt 0 ]]; then
    echo "queue-edges.test.sh: $fails case(s) failed"
    exit 1
fi
echo "queue-edges.test.sh: clean (grammar: live-set resolution, unresolved token, self-citation, blocked-by, done-not-live; verbatim line; sub-task attribution; lead-line prose excluded; --inbound empty vs dead slug; no mutation; retired target + marker, never-live floor, no-repository degradation, --inbound over both domains; $checks checks)"
exit 0
