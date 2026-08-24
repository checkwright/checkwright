#!/usr/bin/env bash
# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the deterministic stamp half of a stage transition, mechanized (judgment stays in the skill)
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/stages.sh
source "$KIT/lib/stages.sh"
# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the kit's bin/ layer depends on gate-sdk, declared at load rather than inside the one arm that dispatches a gate: this tool resolves check-stage-evidence through gate_command instead of by script path, and writing a second dispatch resolver here is the duplicate the substrate exists to remove
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"

# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the unnamed-iteration placeholder, resolved once: the boundary reset's header rewrite and bootstrap stamp, the boundary-require skip, and --rename's refusal to write it all read this rather than repeating the glyph
UNNAMED="—"

usage() {
    printf 'usage: %s [--simulate] <stage>          (stage ∈ %s)\n' "$(basename "$0")" "${LIFECYCLE_KIT_STAGES[*]}"
    printf '       %s [--simulate] --rename <name>  (rename the iteration: queue header + column 1 of every stamp)\n' "$(basename "$0")"
    printf '       %s [-h|--help]\n' "$(basename "$0")"
}

# spec: gate-sdk/SPEC.md §The bin/-tool contract — the help half; this tool's positionals are membership-validated, so it owes usage on stdout at exit 0 but no leading-'-' refusal
if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
    usage
    exit 0
fi

# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — --simulate: read-only preflight, every line prefixed 'enter-stage (simulate):' so a transcript can never read as a stamp
sim=0
if [[ "${1:-}" == "--simulate" ]]; then sim=1; shift; fi
sim_relay() { local l; while IFS= read -r l; do echo "enter-stage (simulate): $l"; done <<<"$1"; }

# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the recovery relay: a refusal's help line is its actionable half, so it prints under --simulate too. The mode's designed consumer is the lead, which gates an expensive dispatch on it rather than hand-deriving prior-stage completeness; relaying the verdict while withholding the one line that resolves it is what sent a lead to escalate a question this tool already answered.
relay_help() {
    local l
    for l in "$@"; do
        if [[ "$sim" == 1 ]]; then
            echo "enter-stage (simulate):   help: $l" >&2
        else
            echo "  help: $l" >&2
        fi
    done
}

# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the two pre-flight arms refuse on the same terms and recover the same way, so the recovery is one string rather than two that must be kept in step
HELP_PREFLIGHT="resolve the finding above, or (to override deliberately) perform the stamp by hand."

QUEUE="$LIFECYCLE_KIT_QUEUE_FILE"
STATE="$LIFECYCLE_KIT_STATE_FILE"

# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — --rename: the two-surface iteration rename in one motion, no stamp appended and no stage token written, so the cursor is untouched and this is not stage motion
if [[ "${1:-}" == "--rename" ]]; then
    rn_name="${2:-}"
    if [[ $# -ne 2 ]]; then
        echo "enter-stage: --rename takes exactly one <name> — nothing written." >&2
        usage >&2
        exit 2
    fi
    if [[ -z "$rn_name" ]]; then
        echo "enter-stage: --rename <name> is empty — nothing written." >&2
        exit 2
    fi
    # spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the placeholder refusal precedes the slug grammar that would already reject it, so a session trying to un-name an iteration is told which writer owns that value rather than that its name is malformed
    if [[ "$rn_name" == "$UNNAMED" ]]; then
        echo "enter-stage: --rename must not write the unnamed placeholder '$UNNAMED' — only the iteration-boundary reset writes it; nothing written." >&2
        exit 2
    fi
    # spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the slug grammar is a refusal because column 1 is whitespace-delimited: a two-word name silently shifts every field of every stamp
    if [[ ! "$rn_name" =~ ^[a-z0-9][a-z0-9-]*$ ]]; then
        echo "enter-stage: --rename '$rn_name' is not a queue slug ([a-z0-9][a-z0-9-]*) — nothing written." >&2
        exit 2
    fi
    [[ -f "$QUEUE" ]] || { echo "enter-stage: queue file not found: $QUEUE" >&2; exit 2; }
    [[ -f "$STATE" ]] || { echo "enter-stage: state file not found: $STATE" >&2; exit 2; }

    rn_hdr="$(lifecycle_header "$QUEUE")"
    if [[ -z "$rn_hdr" ]]; then
        echo "enter-stage: no '## Iteration:' header in $QUEUE — nothing written." >&2
        exit 2
    fi
    rn_cur="$(lifecycle_header_iter "$rn_hdr")"
    rn_moves="$(awk -v n="$rn_name" '/^---[[:space:]]*$/ { f = 1; next } f && NF && $1 != n { c++ } END { print c + 0 }' "$STATE")"
    if [[ "$rn_cur" == "$rn_name" && "$rn_moves" == 0 ]]; then
        if [[ "$sim" == 1 ]]; then
            echo "enter-stage (simulate): the iteration is already named '$rn_name' in $QUEUE and in every stamp of $STATE — the real rename would be an idempotent no-op."
            exit 0
        fi
        echo "enter-stage: the iteration is already named '$rn_name' in $QUEUE and in every stamp of $STATE — idempotent no-op, nothing written."
        exit 0
    fi

    rn_tmpdir="${GATE_SDK_TMP_DIR:-.tmp}"
    mkdir -p "$rn_tmpdir"
    rn_tmpqueue="$rn_tmpdir/enter-stage.rename.queue.$$"
    rn_tmpstate="$rn_tmpdir/enter-stage.rename.state.$$"
    trap 'rm -f "$rn_tmpqueue" "$rn_tmpstate"' EXIT

    awk -v n="$rn_name" '
        !d && /^## Iteration:/ { print "## Iteration: " n; d = 1; next }
        { print }
    ' "$QUEUE" > "$rn_tmpqueue"
    # spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — every data line's column 1 is rewritten, not only the last: the first-stage entry truncates the state file, so every line below the separator belongs to the current iteration by construction — which is what check-stage-evidence asserts — and rewriting all of them heals a half-landed hand-rename
    awk -v n="$rn_name" '
        !f { print; if ($0 ~ /^---[[:space:]]*$/) f = 1; next }
        !NF { print; next }
        { $1 = n; print }
    ' "$STATE" > "$rn_tmpstate"

    # spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the columns-2-to-last witness: the writer proves it touched only the field it meant to, which is the content predicate a PreToolUse guard would have to reconstruct a pre-edit file to compute. It reads to $NF rather than to a pinned column because a field riding outside the witness could be dropped or corrupted with neither this tool nor its test noticing
    rn_fields() { awk '/^---[[:space:]]*$/ { f = 1; next } f && NF { s = ""; for (i = 2; i <= NF; i++) s = s (i > 2 ? " " : "") $i; print s }' "$1"; }
    if [[ "$(rn_fields "$STATE")" != "$(rn_fields "$rn_tmpstate")" ]]; then
        echo "enter-stage: the rename would alter columns 2 through NF (stage, session id, date, head) of $STATE — refusing, nothing written." >&2
        exit 2
    fi

    # spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the rename pre-flight names the gate and never a substrate: gate_command yields the shell script's one-element argv or the binary's, prefixed by its bridged knobs, and the two positional arguments ride unchanged because that argv is prefix-shaped. An argv the bridge refused to build is exit 2 — the verdict the dispatcher gives it — never a rename that proceeds unchecked.
    mapfile -t rn_argv < <(gate_command check-stage-evidence "$KIT/checks")
    if [[ ${#rn_argv[@]} -eq 0 ]]; then
        echo "enter-stage: check-stage-evidence could not be dispatched (see above) — the rename could not be pre-flighted; nothing written." >&2
        exit 2
    fi
    if ! rn_pre="$("${rn_argv[@]}" "$rn_tmpqueue" "$rn_tmpstate" 2>&1)"; then
        if [[ "$sim" == 1 ]]; then
            echo "enter-stage (simulate): check-stage-evidence would refuse the rename to '$rn_name':" >&2
            sim_relay "$rn_pre" >&2
            exit 1
        fi
        echo "enter-stage: check-stage-evidence refuses the rename to '$rn_name' — nothing written:" >&2
        printf '%s\n' "$rn_pre" >&2
        exit 1
    fi

    if [[ "$sim" == 1 ]]; then
        echo "enter-stage (simulate): --rename '$rn_name' would rewrite both surfaces — no write:"
        sim_relay "$QUEUE: '## Iteration: $rn_cur' -> '## Iteration: $rn_name'"
        sim_relay "$STATE: column 1 of $rn_moves stamp(s) -> '$rn_name'; columns 2 through NF proved unchanged"
        exit 0
    fi

    # spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the rename is a whole-file rewrite by construction (every data line's column 1 moves), which is why it is the writer's own mode rather than an append like the stamp path
    mv "$rn_tmpqueue" "$QUEUE"
    mv "$rn_tmpstate" "$STATE"
    trap - EXIT
    echo "enter-stage: renamed the iteration to '$rn_name' — header in $QUEUE, column 1 of $rn_moves stamp(s) in $STATE; columns 2 through NF proved unchanged."
    echo "  next: commit $QUEUE and $STATE together — the rename writes both, and check-stage-evidence requires them to agree."
    exit 0
fi

stage="${1:-}"
if [[ -z "$stage" ]]; then usage >&2; exit 2; fi
if ! lifecycle_stage_known "$stage"; then
    echo "enter-stage: '$stage' is not a lifecycle stage (${LIFECYCLE_KIT_STAGES[*]})" >&2
    usage >&2
    exit 2
fi

[[ -f "$QUEUE" ]] || { echo "enter-stage: queue file not found: $QUEUE" >&2; exit 2; }
[[ -f "$STATE" ]] || { echo "enter-stage: state file not found: $STATE" >&2; exit 2; }

hdr="$(lifecycle_header "$QUEUE")"
cur_iter="$(lifecycle_header_iter "$hdr")"
if [[ "$stage" == "$LIFECYCLE_KIT_FIRST_STAGE" ]]; then
    first=1
    stamp_iter="$UNNAMED"
else
    first=0
    stamp_iter="$cur_iter"
fi

if ! id="$(bash "$KIT/bin/session-id.sh")"; then
    echo "enter-stage: could not read the session id (see above) — nothing written." >&2
    exit 2
fi
today="$(date +%F)"
# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the sole production writer of <head>, read in the state file's own work tree at the instant of the append; 'none' where there is no work tree or no commit to name is a value, never an omission
head_at="$(git -C "$(dirname "$STATE")" rev-parse --short HEAD 2>/dev/null)"
[[ -n "$head_at" ]] || head_at="none"
stamp_line="$stamp_iter $stage $id $today $head_at"

last="$(awk '/^---[[:space:]]*$/ { f = 1; next } f && NF { last = $0 } END { print last }' "$STATE")"
read -r f_iter f_stage f_id _f_date f_head _ <<<"$last"
# spec: lifecycle-kit/SPEC.md §The state machine — the idempotence guard keys on the head too, so a re-entry after HEAD moved appends rather than reporting a no-op: re-running this tool IS the stated remedy for a stale recorded head, and a guard blind to the head would answer that remedy by writing nothing
if [[ "$f_iter" == "$stamp_iter" && "$f_stage" == "$stage" && "$f_id" == "$id" && "$f_head" == "$head_at" ]]; then
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
wiped=()

# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the pre-flight hand-off: the cursor is the last stamp, so the temp file carrying the candidate transition is the STATE file, not the queue. The boundary reset additionally renames the header (dropping any residual pre-upgrade [stage:] field), so the first stage passes a temp queue too; every later entry passes the live queue untouched — stage motion no longer writes it.
if [[ "$first" == 1 ]]; then
    awk -v u="$UNNAMED" '
        !d && /^## Iteration:/ { print "## Iteration: " u; d = 1; next }
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

# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the built-in pre-flight names the gate and never a substrate, the rename pre-flight's own resolution: an argv the bridge refused to build is exit 2, never an entry that proceeds unchecked.
mapfile -t pre_argv < <(gate_command check-stage-entry "$KIT/checks")
if [[ ${#pre_argv[@]} -eq 0 ]]; then
    echo "enter-stage: check-stage-entry could not be dispatched (see above) — the entry could not be pre-flighted; nothing written." >&2
    exit 2
fi
if ! preflight="$("${pre_argv[@]}" "$pre_queue" "$tmpstate" 2>&1)"; then
    if [[ "$sim" == 1 ]]; then
        echo "enter-stage (simulate): check-stage-entry would refuse the entry to '$stage':" >&2
        sim_relay "$preflight" >&2
    else
        echo "enter-stage: check-stage-entry refuses the entry to '$stage' — nothing written:" >&2
        printf '%s\n' "$preflight" >&2
    fi
    relay_help "$HELP_PREFLIGHT"
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
        else
            echo "enter-stage: LIFECYCLE_KIT_ENTRY_PREFLIGHT command for '$stage' refuses the entry — nothing written:" >&2
            printf '%s\n' "$pf_out" >&2
        fi
        relay_help "$HELP_PREFLIGHT"
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
        else
            echo "enter-stage: iteration-boundary entry to '$stage' refused — ## Lessons Learned is non-empty; the close stage must disposition every lesson before the next iteration begins (nothing written):" >&2
            printf '%s\n' "$lessons" >&2
        fi
        relay_help "run the close ritual's disposition step (rule/task/harvest/discard, stamping $LIFECYCLE_KIT_LESSON_EVIDENCE_FILE), clear the section, then re-run enter-stage $stage."
        exit 1
    fi
fi

# spec: lifecycle-kit/SPEC.md §The committed gap inbox — the iteration-boundary gap-inbox check: one detector (any bullet), two dispositions. Close-skipped refuses as it always did, because a skipped stage is recoverable by running it; post-close admits, because no stage of the closing iteration is coming back and a refusal there only pushes the queue write outside the state machine, ahead of any stamp.
if [[ "$first" == 1 && -f "$LIFECYCLE_KIT_GAP_INBOX_FILE" ]]; then
    gaps="$(awk '/^-[[:space:]]/ { print }' "$LIFECYCLE_KIT_GAP_INBOX_FILE")"
    if [[ -n "$gaps" ]]; then
        gap_n="$(grep -c '' <<<"$gaps")"
        # spec: lifecycle-kit/SPEC.md §The committed gap inbox — the discriminator, one cursor read shared with bin/file-gap.sh. A never-named closing iteration has no close to have skipped — the guard LIFECYCLE_KIT_BOUNDARY_REQUIRE applies one block down for the same reason — and a boundary with no cursor at all is that case too, which the predicate alone reports as not-reached, so both edges are named here rather than folded into it.
        if [[ "$cur_iter" == "$UNNAMED" ]] || lifecycle_closing_stage_reached "$STATE" \
            || [[ -z "$(lifecycle_current_stage "$STATE")" ]]; then
            if [[ "$sim" == 1 ]]; then
                echo "enter-stage (simulate): iteration-boundary entry to '$stage' would not refuse for the gap inbox — it would carry $gap_n bullet(s) from $LIFECYCLE_KIT_GAP_INBOX_FILE into '$stage''s own intake:" >&2
                sim_relay "$gaps" >&2
            else
                echo "enter-stage: $LIFECYCLE_KIT_GAP_INBOX_FILE holds $gap_n bullet(s) and no stage of the closing iteration is coming back for them — they do not refuse this entry; they are this iteration's '$stage' intake:" >&2
                printf '%s\n' "$gaps" >&2
            fi
            relay_help "disposition each bullet in this session, after the stamp: promote it to a queue entry, fix it inline, or discard it with cause in the commit message — then truncate $LIFECYCLE_KIT_GAP_INBOX_FILE to its header in the same commit. Deleting a bullet without a disposition is not a drain."
            relay_help "a promoted entry's provenance sentence carries the bullet's own date and names the iteration whose close generated it — the finding's disposition lands in this iteration's ledger, and saying so is what keeps that legible."
        else
            if [[ "$sim" == 1 ]]; then
                echo "enter-stage (simulate): iteration-boundary entry to '$stage' would be refused — $LIFECYCLE_KIT_GAP_INBOX_FILE holds $gap_n untriaged gap bullet(s) and the cursor never reached '${LIFECYCLE_KIT_STAGES[-1]}', the closing stage of '$cur_iter':" >&2
                sim_relay "$gaps" >&2
            else
                echo "enter-stage: iteration-boundary entry to '$stage' refused — $LIFECYCLE_KIT_GAP_INBOX_FILE holds $gap_n untriaged gap bullet(s) and the cursor never reached '${LIFECYCLE_KIT_STAGES[-1]}', the closing stage of '$cur_iter' (nothing written):" >&2
                printf '%s\n' "$gaps" >&2
            fi
            relay_help "run the closing stage's gap-drain step — disposition each bullet (promote to a deferred [design-pending] entry, fix inline, or discard with cause in the commit message), truncate the inbox to its header, then re-run enter-stage $stage."
            exit 1
        fi
    fi
fi

# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the iteration-boundary linked-worktree refusal, the same contract as the two above: at an iteration boundary no linked worktree should be live, an in-flight dispatch being something that must not straddle a boundary and everything else being residue. Read off 'git worktree list' and never off 'git status' — an ignored worktree leaves the status clean while it still stands, so a status-derived check reports success on exactly the state it exists to catch. The predicate is a property of the boundary rather than a path, so no knob names a residue directory: a kit default spelling one harness's layout would publish it.
if [[ "$first" == 1 && "$LIFECYCLE_KIT_BOUNDARY_WORKTREE_CHECK" == "1" ]] \
    && git rev-parse --git-dir &>/dev/null; then
    linked="$(git worktree list --porcelain 2>/dev/null \
        | awk '/^worktree / { if (n++) print substr($0, 10) }')"
    if [[ -n "$linked" ]]; then
        wt_n="$(grep -c '' <<<"$linked")"
        if [[ "$sim" == 1 ]]; then
            echo "enter-stage (simulate): iteration-boundary entry to '$stage' would be refused — $wt_n linked worktree(s) still stand:" >&2
            sim_relay "$linked" >&2
        else
            echo "enter-stage: iteration-boundary entry to '$stage' refused — $wt_n linked worktree(s) still stand (nothing written):" >&2
            printf '%s\n' "$linked" >&2
        fi
        relay_help "reap each path with 'git worktree remove <path>' (or --force where the child left it locked) and delete the branch ref it leaves behind — 'worktree remove' clears the directory only, so a reap that stops there accretes refs this check cannot see. Then re-run enter-stage $stage."
        relay_help "the harness's auto-clean of a read-only child's worktree is best-effort, so residue here is expected rather than evidence the child wrote; one 'git status --porcelain' inside a worktree tells a stray write from an unfired reclamation (delegation-kit/SPEC.md §The delegation model)."
        exit 1
    fi
fi

# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — LIFECYCLE_KIT_BOUNDARY_REQUIRE: at the iteration boundary each member must carry a data line whose first token is the closing iteration's name, else the entry refuses (fail-closed on a missing file); a never-named (—) closing iteration has nothing to disposition and skips the check. Runs after the Lessons refusal and before the boundary truncation, the same refusal contract.
if [[ "$first" == 1 && "$cur_iter" != "$UNNAMED" ]]; then
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
        else
            echo "enter-stage: iteration-boundary entry to '$stage' refused — $req_msg (nothing written)." >&2
        fi
        relay_help "the close stage must disposition the iteration at the release boundary, stamping a '<iteration> release <version|none> — <basis>' line into $br before the next iteration begins."
        exit 1
    done
fi

if [[ "$sim" == 1 ]]; then
    echo "enter-stage (simulate): entry to '$stage' would proceed — no stamp, nothing written."
    exit 0
fi

if [[ "$first" == 1 ]]; then
    mv "$tmpstate" "$STATE"
    # spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the two kit-owned surfaces (lesson evidence, survey record) reset as built-in members (the kit owns them); LIFECYCLE_KIT_BOUNDARY_TRUNCATE stays reserved for files the kit does not own, and a defaulted array is replaced rather than merged when a consumer assigns it, so a built-in shipped as a default member would lose its reset in every configuring consumer
    for bt in "$LIFECYCLE_KIT_LESSON_EVIDENCE_FILE" "$LIFECYCLE_KIT_SURVEY_RECORD_FILE" ${LIFECYCLE_KIT_BOUNDARY_TRUNCATE[@]+"${LIFECYCLE_KIT_BOUNDARY_TRUNCATE[@]}"}; do
        [[ -f "$bt" ]] || continue
        bttmp="$tmpdir/boundary-truncate.$$"
        # spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the header run stops at a markdown '## ' section heading as well as at the first data line: on a markdown surface whose blocks are '## ' headings (the survey record) a bare /^#/ predicate reads the first block's heading as part of the header and carries it across the boundary
        awk 'drop { next } /^[[:space:]]*$/ { print; next } /^#([^#]|$)/ { print; next } { drop = 1 }' "$bt" > "$bttmp"
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

# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the boundary scratch wipe, distinct from the truncate above (that rewrites a tracked file to its header; this deletes untracked scratch outright). Runs last so this run's own enter-stage.*.$$ temporaries are already gone and never candidates. '.gitkeep' is the kit invariant LIFECYCLE_KIT_BOUNDARY_PRESERVE cannot unset — a consumer that tracks its scratch dir's scaffolding must not have it deleted at the moment the boundary reset is committed.
if [[ "$first" == 1 && -d "$tmpdir" ]]; then
    wipe_args=(-mindepth 1 -depth ! -name .gitkeep)
    for bp in ${LIFECYCLE_KIT_BOUNDARY_PRESERVE[@]+"${LIFECYCLE_KIT_BOUNDARY_PRESERVE[@]}"}; do
        wipe_args+=(! -name "$bp")
    done
    # spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — stderr is suppressed because a preserved basename inside a doomed subdirectory makes that subdirectory's own delete fail; the failure is noise, never an abort
    mapfile -t wiped < <(find "$tmpdir" "${wipe_args[@]}" -print -delete 2>/dev/null)
fi

if [[ "$first" == 1 ]]; then
    echo "enter-stage: iteration-boundary reset — stamped '$stamp_line'; header set to '## Iteration: $UNNAMED'."
    echo "  next: commit $QUEUE and $STATE together (the boundary reset writes both), hook enabled."
else
    echo "enter-stage: stamped '$stamp_line'; the cursor is now '$stage' (no queue write — stage motion never touches it)."
    echo "  next: commit $STATE, hook enabled."
fi
if [[ ${#truncated[@]} -gt 0 ]]; then
    echo "  note: boundary-truncated to the '# contract:' header: ${truncated[*]} — commit alongside the reset."
fi
if [[ ${#wiped[@]} -gt 0 ]]; then
    echo "  note: boundary-wiped from $tmpdir: ${wiped[*]}"
fi

# spec: lifecycle-kit/SPEC.md §The survey record — the read trigger: the entry report prints the record's headings (the questions), never the findings, at the one moment a stage session is guaranteed to be looking. Findings stay behind the witness — printing them here would put a possibly-stale judgment into context ahead of the check that qualifies it. A non-empty record never refuses the boundary: unlike the gap inbox one line above, a survey owes nobody a disposition.
if [[ -f "$LIFECYCLE_KIT_SURVEY_RECORD_FILE" ]]; then
    mapfile -t survey_qs < <(awk '/^##[[:space:]]/ { print }' "$LIFECYCLE_KIT_SURVEY_RECORD_FILE")
    if [[ ${#survey_qs[@]} -gt 0 ]]; then
        echo "  note: $LIFECYCLE_KIT_SURVEY_RECORD_FILE carries ${#survey_qs[@]} survey(s) this iteration — before buying one of these again, run its witness (diff the corpus since its rev, re-run its oracle) and cite it if both hold:"
        printf '    %s\n' "${survey_qs[@]}"
    fi
fi
