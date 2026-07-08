#!/usr/bin/env bash
# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the deterministic flip+stamp half of a stage transition, mechanized (judgment stays in the skill)
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

last="$(awk '/^---[[:space:]]*$/ { f = 1; next } f && NF { last = $0 } END { print last }' "$STATE")"
read -r f_iter f_stage f_id _ <<<"$last"
if [[ "$f_iter" == "$stamp_iter" && "$f_stage" == "$stage" && "$f_id" == "$id" ]]; then
    echo "enter-stage: '$stamp_line' already stamped in $STATE — idempotent no-op, nothing written."
    exit 0
fi

tmpdir="${GATE_SDK_TMP_DIR:-.tmp}"
mkdir -p "$tmpdir"
tmpqueue="$tmpdir/enter-stage.queue.$$"
trap 'rm -f "$tmpqueue"' EXIT
truncated=()

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

if ! preflight="$(bash "$KIT/checks/check-stage-entry.sh" "$tmpqueue" "$STATE" 2>&1)"; then
    echo "enter-stage: check-stage-entry refuses the flip to '$stage' — nothing written:" >&2
    printf '%s\n' "$preflight" >&2
    echo "  help: resolve the finding above, or (to override deliberately) perform the stamp+flip by hand." >&2
    exit 1
fi

# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — LIFECYCLE_ENTRY_PREFLIGHT: each entry matching the entered stage runs after the built-in pre-flight with the flipped temp queue + state file appended; a non-zero exit refuses the flip, nothing written
for pf in ${LIFECYCLE_ENTRY_PREFLIGHT[@]+"${LIFECYCLE_ENTRY_PREFLIGHT[@]}"}; do
    [[ "${pf%%=*}" == "$stage" ]] || continue
    read -r -a pf_argv <<<"${pf#*=}"
    if ! pf_out="$("${pf_argv[@]}" "$tmpqueue" "$STATE" 2>&1)"; then
        echo "enter-stage: LIFECYCLE_ENTRY_PREFLIGHT command for '$stage' refuses the flip — nothing written:" >&2
        printf '%s\n' "$pf_out" >&2
        echo "  help: resolve the finding above, or (to override deliberately) perform the stamp+flip by hand." >&2
        exit 1
    fi
done

if [[ "$first" == 1 ]]; then
    header_only="$(awk '{ print } /^---[[:space:]]*$/ { exit }' "$STATE")"
    printf '%s\n\n%s\n' "$header_only" "$stamp_line" > "$STATE"
    # spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — LIFECYCLE_BOUNDARY_TRUNCATE: reset each listed file to its leading '# contract:' header at the iteration boundary
    for bt in ${LIFECYCLE_BOUNDARY_TRUNCATE[@]+"${LIFECYCLE_BOUNDARY_TRUNCATE[@]}"}; do
        [[ -f "$bt" ]] || continue
        bttmp="$tmpdir/boundary-truncate.$$"
        awk 'drop { next } /^#/ || /^[[:space:]]*$/ { print; next } { drop = 1 }' "$bt" > "$bttmp"
        mv "$bttmp" "$bt"
        truncated+=("$bt")
    done
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
if [[ ${#truncated[@]} -gt 0 ]]; then
    echo "  note: boundary-truncated to the '# contract:' header: ${truncated[*]} — commit alongside the reset."
fi
