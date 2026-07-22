#!/usr/bin/env bash
# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the deterministic stamp half of a stage transition, mechanized (judgment stays in the skill)
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/stages.sh
source "$KIT/lib/stages.sh"

usage() { echo "usage: enter-stage.sh [--simulate] <stage>   (stage ∈ ${LIFECYCLE_KIT_STAGES[*]})" >&2; }

# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — --simulate: read-only preflight, every line prefixed 'enter-stage (simulate):' so a transcript can never read as a stamp
sim=0
if [[ "${1:-}" == "--simulate" ]]; then sim=1; shift; fi
sim_relay() { local l; while IFS= read -r l; do echo "enter-stage (simulate): $l"; done <<<"$1"; }

stage="${1:-}"
if [[ -z "$stage" ]]; then usage; exit 2; fi
if ! lifecycle_stage_known "$stage"; then
    echo "enter-stage: '$stage' is not a lifecycle stage (${LIFECYCLE_KIT_STAGES[*]})" >&2
    usage
    exit 2
fi

QUEUE="$LIFECYCLE_KIT_QUEUE_FILE"
STATE="$LIFECYCLE_KIT_STATE_FILE"
[[ -f "$QUEUE" ]] || { echo "enter-stage: queue file not found: $QUEUE" >&2; exit 2; }
[[ -f "$STATE" ]] || { echo "enter-stage: state file not found: $STATE" >&2; exit 2; }

hdr="$(lifecycle_header "$QUEUE")"
cur_iter="$(lifecycle_header_iter "$hdr")"
if [[ "$stage" == "$LIFECYCLE_KIT_FIRST_STAGE" ]]; then
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
    if [[ "$sim" == 1 ]]; then
        echo "enter-stage (simulate): '$stamp_line' is already the last stamp in $STATE — the real entry would be an idempotent no-op."
        exit 0
    fi
    echo "enter-stage: '$stamp_line' already stamped in $STATE — idempotent no-op, nothing written."
    exit 0
fi

tmpdir="${GATE_SDK_TMP_DIR:-.tmp}"
mkdir -p "$tmpdir"
tmpqueue="$tmpdir/enter-stage.queue.$$"
tmpstate="$tmpdir/enter-stage.state.$$"
trap 'rm -f "$tmpqueue" "$tmpstate"' EXIT
truncated=()

# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the pre-flight hand-off: the cursor is the last stamp, so the temp file carrying the candidate transition is the STATE file, not the queue. The boundary reset additionally renames the header (dropping any residual pre-upgrade [stage:] field), so the first stage passes a temp queue too; every later entry passes the live queue untouched — stage motion no longer writes it.
if [[ "$first" == 1 ]]; then
    awk '
        !d && /^## Iteration:/ { print "## Iteration: —"; d = 1; next }
        { print }
    ' "$QUEUE" > "$tmpqueue"
    header_only="$(awk '{ print } /^---[[:space:]]*$/ { exit }' "$STATE")"
    printf '%s\n\n%s\n' "$header_only" "$stamp_line" > "$tmpstate"
    pre_queue="$tmpqueue"
else
    cp "$STATE" "$tmpstate"
    printf '%s\n' "$stamp_line" >> "$tmpstate"
    pre_queue="$QUEUE"
fi

if ! preflight="$(bash "$KIT/checks/check-stage-entry.sh" "$pre_queue" "$tmpstate" 2>&1)"; then
    if [[ "$sim" == 1 ]]; then
        echo "enter-stage (simulate): check-stage-entry would refuse the entry to '$stage':" >&2
        sim_relay "$preflight" >&2
        exit 1
    fi
    echo "enter-stage: check-stage-entry refuses the entry to '$stage' — nothing written:" >&2
    printf '%s\n' "$preflight" >&2
    echo "  help: resolve the finding above, or (to override deliberately) perform the stamp by hand." >&2
    exit 1
fi

# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — LIFECYCLE_KIT_ENTRY_PREFLIGHT: each entry matching the entered stage runs after the built-in pre-flight with the same '<queue> <state>' argv the built-in gets — the candidate-carrying temp file is the state file, the queue passes through live; a non-zero exit refuses the entry, nothing written
for pf in ${LIFECYCLE_KIT_ENTRY_PREFLIGHT[@]+"${LIFECYCLE_KIT_ENTRY_PREFLIGHT[@]}"}; do
    [[ "${pf%%=*}" == "$stage" ]] || continue
    read -r -a pf_argv <<<"${pf#*=}"
    if ! pf_out="$("${pf_argv[@]}" "$pre_queue" "$tmpstate" 2>&1)"; then
        if [[ "$sim" == 1 ]]; then
            echo "enter-stage (simulate): LIFECYCLE_KIT_ENTRY_PREFLIGHT command for '$stage' would refuse the entry:" >&2
            sim_relay "$pf_out" >&2
            exit 1
        fi
        echo "enter-stage: LIFECYCLE_KIT_ENTRY_PREFLIGHT command for '$stage' refuses the entry — nothing written:" >&2
        printf '%s\n' "$pf_out" >&2
        echo "  help: resolve the finding above, or (to override deliberately) perform the stamp by hand." >&2
        exit 1
    fi
done

# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the iteration-boundary entry refuses on a non-empty ## Lessons Learned: an untriaged lesson must not cross into the next iteration (no [attend] injection may outlive its iteration), the same refusal contract as the check-stage-entry precondition
if [[ "$first" == 1 ]]; then
    lessons="$(awk '
        /^## Lessons Learned[[:space:]]*$/ { inl = 1; next }
        /^## / { inl = 0 }
        inl && /^-[[:space:]]/ { print }
    ' "$QUEUE")"
    if [[ -n "$lessons" ]]; then
        if [[ "$sim" == 1 ]]; then
            echo "enter-stage (simulate): iteration-boundary entry to '$stage' would be refused — ## Lessons Learned is non-empty:" >&2
            sim_relay "$lessons" >&2
            exit 1
        fi
        echo "enter-stage: iteration-boundary entry to '$stage' refused — ## Lessons Learned is non-empty; the close stage must disposition every lesson before the next iteration begins (nothing written):" >&2
        printf '%s\n' "$lessons" >&2
        echo "  help: run the close ritual's disposition step (rule/task/harvest/discard, stamping $LIFECYCLE_KIT_LESSON_EVIDENCE_FILE), clear the section, then re-run enter-stage $stage." >&2
        exit 1
    fi
fi

# spec: lifecycle-kit/SPEC.md §The committed gap inbox — the iteration-boundary entry refuses while the gap inbox holds bullets: a mid-iteration gap must be dispositioned by close's drain before the next iteration begins (the same refusal contract as the Lessons check), so no gap outlives its iteration untriaged.
if [[ "$first" == 1 && -f "$LIFECYCLE_KIT_GAP_INBOX_FILE" ]]; then
    gaps="$(awk '/^-[[:space:]]/ { print }' "$LIFECYCLE_KIT_GAP_INBOX_FILE")"
    if [[ -n "$gaps" ]]; then
        if [[ "$sim" == 1 ]]; then
            echo "enter-stage (simulate): iteration-boundary entry to '$stage' would be refused — $LIFECYCLE_KIT_GAP_INBOX_FILE holds untriaged gap bullets:" >&2
            sim_relay "$gaps" >&2
            exit 1
        fi
        echo "enter-stage: iteration-boundary entry to '$stage' refused — $LIFECYCLE_KIT_GAP_INBOX_FILE holds untriaged gap bullets; every gap is dispositioned before the next iteration begins (nothing written):" >&2
        printf '%s\n' "$gaps" >&2
        echo "  help: if the closing stage has not run yet, run its gap-drain step — disposition each bullet (promote to a deferred [needs-spec] entry, fix inline, or discard with cause in the commit message), then truncate the inbox to its header." >&2
        echo "  help: if it has already run, these bullets were filed after the drain and no stage is coming back for them: disposition them here, in this entering session — promote each directly to the deferred queue (or fix it), truncate the inbox to its header, commit, and re-run enter-stage $stage. Deleting a bullet without a disposition is not a drain." >&2
        exit 1
    fi
fi

# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — LIFECYCLE_KIT_BOUNDARY_REQUIRE: at the iteration boundary each member must carry a data line whose first token is the closing iteration's name, else the entry refuses (fail-closed on a missing file); a never-named (—) closing iteration has nothing to disposition and skips the check. Runs after the Lessons refusal and before the boundary truncation, the same refusal contract.
if [[ "$first" == 1 && "$cur_iter" != "—" ]]; then
    for br in ${LIFECYCLE_KIT_BOUNDARY_REQUIRE[@]+"${LIFECYCLE_KIT_BOUNDARY_REQUIRE[@]}"}; do
        req_msg=""
        if [[ ! -f "$br" ]]; then
            req_msg="required boundary-disposition file not found: $br"
        elif ! awk -v it="$cur_iter" '
            /^#/ || /^[[:space:]]*$/ { next }
            $1 == it { found = 1 }
            END { exit found ? 0 : 1 }
        ' "$br"; then
            req_msg="no disposition line naming the closing iteration '$cur_iter' in $br"
        fi
        [[ -z "$req_msg" ]] && continue
        if [[ "$sim" == 1 ]]; then
            echo "enter-stage (simulate): iteration-boundary entry to '$stage' would be refused — $req_msg" >&2
            exit 1
        fi
        echo "enter-stage: iteration-boundary entry to '$stage' refused — $req_msg (nothing written)." >&2
        echo "  help: the close stage must disposition the iteration at the release boundary, stamping a '<iteration> release <version|none> — <basis>' line into $br before the next iteration begins." >&2
        exit 1
    done
fi

if [[ "$sim" == 1 ]]; then
    echo "enter-stage (simulate): entry to '$stage' would proceed — no stamp, nothing written."
    exit 0
fi

if [[ "$first" == 1 ]]; then
    mv "$tmpstate" "$STATE"
    # spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the kit-owned lesson-evidence file resets as a built-in member (kit owns this surface); LIFECYCLE_KIT_BOUNDARY_TRUNCATE stays reserved for files the kit does not own
    for bt in "$LIFECYCLE_KIT_LESSON_EVIDENCE_FILE" ${LIFECYCLE_KIT_BOUNDARY_TRUNCATE[@]+"${LIFECYCLE_KIT_BOUNDARY_TRUNCATE[@]}"}; do
        [[ -f "$bt" ]] || continue
        bttmp="$tmpdir/boundary-truncate.$$"
        awk 'drop { next } /^#/ || /^[[:space:]]*$/ { print; next } { drop = 1 }' "$bt" > "$bttmp"
        mv "$bttmp" "$bt"
        truncated+=("$bt")
    done
    mv "$tmpqueue" "$QUEUE"
else
    # spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the live stamp is an append, never a rewrite of the pre-flight temp copy: a concurrent session's stamp landing between the copy and the write would be lost by a whole-file move
    printf '%s\n' "$stamp_line" >> "$STATE"
fi
trap - EXIT
rm -f "$tmpqueue" "$tmpstate"

if [[ "$first" == 1 ]]; then
    echo "enter-stage: iteration-boundary reset — stamped '$stamp_line'; header set to '## Iteration: —'."
    echo "  next: commit $QUEUE and $STATE together (the boundary reset writes both), hook enabled."
else
    echo "enter-stage: stamped '$stamp_line'; the cursor is now '$stage' (no queue write — stage motion never touches it)."
    echo "  next: commit $STATE, hook enabled."
fi
if [[ ${#truncated[@]} -gt 0 ]]; then
    echo "  note: boundary-truncated to the '# contract:' header: ${truncated[*]} — commit alongside the reset."
fi
