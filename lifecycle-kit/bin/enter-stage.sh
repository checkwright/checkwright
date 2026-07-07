#!/usr/bin/env bash
# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the deterministic flip+stamp half of a stage transition, mechanized (judgment stays in the skill)
#
# usage: enter-stage.sh <stage>
#   Performs the mechanical first step of a stage transition: append the
#   invocation stamp to the evidence file and flip the queue header's [stage:]
#   field, in one invocation (the flip+stamp ride together). The <stage>
#   argument must be one of LIFECYCLE_STAGES; anything else is a usage error.
#
#   Ordinary stage: read the iteration from the queue header, take the id from
#   session-id.sh (never an argument — the no-hand-picking rule rides into the
#   tool), append '<iteration> <stage> <id> <date>' to the state file, and flip
#   the header's [stage:] field to <stage>.
#   First stage (LIFECYCLE_FIRST_STAGE): iteration-boundary reset instead —
#   truncate the state file back to its header, stamp under the '—' sentinel,
#   and set the header to '## Iteration: —  [stage: <stage>]'.
#
#   Pre-flight, not enforcement: before mutating anything, run check-stage-entry
#   for the entered stage (a header-flipped temp queue + the real state file)
#   and refuse (exit 1, no writes) when it is red. The tool takes no --force, so
#   the easy path is the compliant one; an intended override is done by hand.
#   Idempotent: if the state file already ends with a stamp for the same
#   <iteration> <stage> <id>, report and exit 0 without appending.
#
#   Judgment stays in the skill (what the stage means, its exit condition, when
#   to enter it); committing the flip+stamp remains the skill's business too.
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/stages.sh
source "$KIT/lib/stages.sh"

usage() { echo "usage: enter-stage.sh <stage>   (stage ∈ ${LIFECYCLE_STAGES[*]})" >&2; }

stage="${1:-}"
if [[ -z "$stage" ]]; then usage; exit 2; fi
if ! lifecycle_stage_known "$stage"; then
    echo "enter-stage: '$stage' is not a lifecycle stage (${LIFECYCLE_STAGES[*]})" >&2
    usage
    exit 2
fi

QUEUE="$LIFECYCLE_QUEUE_FILE"
STATE="$LIFECYCLE_STATE_FILE"
[[ -f "$QUEUE" ]] || { echo "enter-stage: queue file not found: $QUEUE" >&2; exit 2; }
[[ -f "$STATE" ]] || { echo "enter-stage: state file not found: $STATE" >&2; exit 2; }

# Mode and the values that ride into the stamp/header. A malformed or absent
# header leaves cur_iter empty; the pre-flight (check-stage-entry) is the guard
# for that, not an early exit — the header content is the gate's to judge.
hdr="$(lifecycle_header "$QUEUE")"
cur_iter="$(lifecycle_header_iter "$hdr")"
if [[ "$stage" == "$LIFECYCLE_FIRST_STAGE" ]]; then
    first=1
    stamp_iter="—"
else
    first=0
    stamp_iter="$cur_iter"
fi

if ! id="$(bash "$KIT/bin/session-id.sh")"; then
    echo "enter-stage: could not read the session id (see above) — nothing written." >&2
    exit 2
fi
today="$(date +%F)"
stamp_line="$stamp_iter $stage $id $today"

# Idempotent re-entry: the state file's last data line already stamps this
# <iteration> <stage> <id>. A crashed-and-resumed session re-runs safely.
last="$(awk '/^---[[:space:]]*$/ { f = 1; next } f && NF { last = $0 } END { print last }' "$STATE")"
read -r f_iter f_stage f_id _ <<<"$last"
if [[ "$f_iter" == "$stamp_iter" && "$f_stage" == "$stage" && "$f_id" == "$id" ]]; then
    echo "enter-stage: '$stamp_line' already stamped in $STATE — idempotent no-op, nothing written."
    exit 0
fi

# Build the header-flipped queue once (it is both the pre-flight subject and,
# on success, the final file). A first-stage reset rewrites the whole header to
# the unnamed-iteration form; an ordinary flip swaps only the [stage:] token.
tmpdir="${GATE_SDK_TMP_DIR:-.tmp}"
mkdir -p "$tmpdir"
tmpqueue="$tmpdir/enter-stage.queue.$$"
trap 'rm -f "$tmpqueue"' EXIT

if [[ "$first" == 1 ]]; then
    awk -v nh="## Iteration: —  [stage: $stage]" '
        !d && /^## Iteration:/ { print nh; d = 1; next }
        { print }
    ' "$QUEUE" > "$tmpqueue"
else
    awk -v st="$stage" '
        !d && /^## Iteration:/ { sub(/\[stage:[^]]*\]/, "[stage: " st "]"); d = 1 }
        { print }
    ' "$QUEUE" > "$tmpqueue"
fi

# Pre-flight: evaluate the *entered* stage through check-stage-entry's existing
# positionals (the flipped temp queue + the real state file). The gate stays
# untouched; a red pre-flight refuses without writing.
if ! preflight="$(bash "$KIT/checks/check-stage-entry.sh" "$tmpqueue" "$STATE" 2>&1)"; then
    echo "enter-stage: check-stage-entry refuses the flip to '$stage' — nothing written:" >&2
    printf '%s\n' "$preflight" >&2
    echo "  help: resolve the finding above, or (to override deliberately) perform the stamp+flip by hand." >&2
    exit 1
fi

# Writes: the state stamp, then the header flip (mv the vetted temp queue into
# place). They must land in one commit — that is the skill's business.
if [[ "$first" == 1 ]]; then
    header_only="$(awk '{ print } /^---[[:space:]]*$/ { exit }' "$STATE")"
    printf '%s\n\n%s\n' "$header_only" "$stamp_line" > "$STATE"
else
    printf '%s\n' "$stamp_line" >> "$STATE"
fi
mv "$tmpqueue" "$QUEUE"
trap - EXIT

if [[ "$first" == 1 ]]; then
    echo "enter-stage: iteration-boundary reset — stamped '$stamp_line'; header set to '## Iteration: —  [stage: $stage]'."
else
    echo "enter-stage: stamped '$stamp_line'; header flipped to [stage: $stage]."
fi
echo "  next: commit $QUEUE and $STATE together (the flip+stamp ride in one commit)."
