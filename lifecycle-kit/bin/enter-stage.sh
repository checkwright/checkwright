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

# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the worktree liveness predicate is evidence-kit's ek_pid_alive, sourced only when the lock-reason pattern is configured: an unconfigured consumer classifies nothing and owes no second kit, so the knob buys the dependency rather than vendoring lifecycle-kit doing so. A configured pattern with the library unreachable is exit 2 rather than an everything-unclassified boundary, the same fail-closed direction lib/stages.sh takes on a malformed pattern.
if [[ -n "$LIFECYCLE_KIT_WORKTREE_LOCK_PID_RE" ]]; then
    EK="${EVIDENCE_KIT_ROOT:-$KIT/../evidence-kit}"
    if [[ ! -r "$EK/lib/evidence.sh" ]]; then
        echo "enter-stage: LIFECYCLE_KIT_WORKTREE_LOCK_PID_RE is set but evidence-kit's liveness predicate is unreachable at $EK/lib/evidence.sh — nothing written." >&2
        exit 2
    fi
    # shellcheck source=../../evidence-kit/lib/evidence.sh
    source "$EK/lib/evidence.sh"
fi

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
VALVE="$LIFECYCLE_KIT_PREFLIGHT_VALVE_FILE"

# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the valve ledger's two fail-closed shapes, refused together so one pass names every malformed line: a data line under four fields, and a state token that is neither 'armed' nor 'used'. Both silent branches are wrong on a ledger that cannot be parsed — admitting hides a malformed arming, refusing hides a valid one — so neither is taken.
valve_parse() {
    awk '
        /^#/ || /^[[:space:]]*$/ { next }
        NF < 4 { printf "  line %d carries %d field(s), fewer than the four <iteration> <stage> armed|used <reason...> requires: %s\n", NR, NF, $0; bad = 1; next }
        $3 != "armed" && $3 != "used" { printf "  line %d carries state token %s, which is neither armed nor used: %s\n", NR, $3, $0; bad = 1 }
        END { exit bad ? 1 : 0 }
    ' "$1"
}

# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the match: the FIRST armed line whose iteration and stage both equal the entering entry's, printed as '<line-number> <prior-used-count> <reason>' tab-separated. The reason is rebuilt field by field rather than by a bracketed-quantifier strip, because that quantifier is not portable across every awk a consumer runs.
valve_lookup() {
    awk -v it="$2" -v st="$3" '
        /^#/ || /^[[:space:]]*$/ { next }
        $1 == it && $3 == "used" { used++ }
        !hit && $1 == it && $2 == st && $3 == "armed" {
            hit = NR
            for (i = 4; i <= NF; i++) reason = reason (i > 4 ? " " : "") $i
        }
        END { if (!hit) exit 1; printf "%d\t%d\t%s\n", hit, used + 0, reason }
    ' "$1"
}

# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the consumption rewrites field 3 of exactly one line rather than substituting the token's text, because an iteration or stage name may itself contain the token and a text substitution would rewrite the wrong field on that line
valve_consume() {
    local vc_tmp="$tmpdir/preflight-valve.$$"
    awk -v n="$2" 'NR == n { $3 = "used" } { print }' "$1" > "$vc_tmp" && mv "$vc_tmp" "$1"
}

valve_state=unqueried
valve_line=""
valve_used=0
valve_reason=""
valve_report=()

# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the ledger is read only where the question "is it armed?" is actually asked, which is a LIFECYCLE_KIT_ENTRY_PREFLIGHT refusal. Parsing it on every entry would let a malformed ledger wedge entries that never needed a valve, which is a wider refusal than the fail-closed arm was ruled for.
valve_query() {
    local vq_bad vq_hit
    [[ "$valve_state" == unqueried ]] || return 0
    valve_state=none
    [[ -n "$VALVE" && -f "$VALVE" ]] || return 0
    if ! vq_bad="$(valve_parse "$VALVE")"; then
        echo "enter-stage: the pre-flight valve ledger $VALVE cannot be parsed, so whether this entry is armed is unanswerable — nothing written:" >&2
        printf '%s\n' "$vq_bad" >&2
        exit 2
    fi
    if vq_hit="$(valve_lookup "$VALVE" "$cur_iter" "$stage")"; then
        IFS=$'\t' read -r valve_line valve_used valve_reason <<<"$vq_hit"
        valve_state=armed
    fi
}

# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the admission report is buffered and emitted with the entry's own report rather than at the moment of the match: a stage may wire several pre-flight commands and the boundary refusals run after the loop, so an entry that matched a valve line can still refuse, and an entry that refuses must never have printed that it was admitted or spent a line saying so.
valve_emit() {
    local vr
    [[ ${#valve_report[@]} -gt 0 ]] || return 0
    for vr in "${valve_report[@]}"; do
        if [[ "$sim" == 1 ]]; then sim_relay "$vr"; else printf '%s\n' "$vr"; fi
    done
}

# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the linked-worktree scan: one porcelain parse classified live/orphaned/unclassified, shared by the boundary refusal and the mid-iteration advisory so the two cannot disagree about what a path is. Prints '<class>\t<pid>\t<path>\t<head>' per linked worktree; the main checkout is skipped.
worktree_scan() {
    git rev-parse --git-dir &>/dev/null || return 0
    local path head lockflag reason pid class
    while IFS=$'\t' read -r path head lockflag reason; do
        [[ -n "$path" ]] || continue
        pid=""
        if [[ -z "$LIFECYCLE_KIT_WORKTREE_LOCK_PID_RE" ]]; then
            class=unclassified
        elif [[ "$lockflag" == 0 ]]; then
            class=orphaned
        elif [[ "$reason" =~ $LIFECYCLE_KIT_WORKTREE_LOCK_PID_RE ]]; then
            pid="${BASH_REMATCH[1]}"
            if ek_pid_alive "$pid"; then class=live; else class=orphaned; fi
        else
            class=unclassified
        fi
        printf '%s\t%s\t%s\t%s\n' "$class" "${pid:--}" "$path" "$head"
    done < <(git worktree list --porcelain 2>/dev/null | awk '
        function flush() {
            if (path != "" && n++) printf "%s\t%s\t%s\t%s\n", path, head, locked, reason
            path = ""; head = ""; locked = 0; reason = ""
        }
        /^worktree /              { flush(); path = substr($0, 10); next }
        /^HEAD /                  { head = substr($0, 6); next }
        /^locked([[:space:]]|$)/  { locked = 1; reason = (length($0) > 7 ? substr($0, 8) : ""); next }
        END { flush() }
    ')
}

# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the loss report: the two facts that decide whether removing a residue worktree loses anything, read mechanically at the moment of refusal rather than left for the session to re-derive per path with a hand-run 'git status'. Two git reads, no vendor vocabulary.
worktree_loss() {
    local p="$1" h="$2" dirty commits
    if [[ ! -d "$p" ]]; then
        echo "directory already gone — prunable residue"
        return 0
    fi
    if [[ -n "$(git -C "$p" status --porcelain 2>/dev/null)" ]]; then dirty="dirty"; else dirty="clean"; fi
    commits="$(git rev-list --count "$h" '^HEAD' 2>/dev/null)"
    [[ "$commits" =~ ^[0-9]+$ ]] || commits="?"
    if [[ "$dirty" == "clean" && "$commits" == "0" ]]; then
        echo "clean, no commit unreachable from HEAD — removal is lossless"
    else
        echo "$dirty, $commits commit(s) unreachable from HEAD"
    fi
}

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
        # spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the valve reaches this arm and no other: LIFECYCLE_KIT_ENTRY_PREFLIGHT is the consumer-wired precondition, and a consumer-wired precondition is the only one whose deadlock a consumer can reach at all
        valve_query
        if [[ "$valve_state" == armed ]]; then
            valve_report+=("the pre-flight valve admitted this entry past a refusing LIFECYCLE_KIT_ENTRY_PREFLIGHT command for '$stage' — the findings it would have refused on:")
            valve_report+=("$(printf '  %s\n' "$pf_out")")
            continue
        fi
        if [[ "$sim" == 1 ]]; then
            echo "enter-stage (simulate): LIFECYCLE_KIT_ENTRY_PREFLIGHT command for '$stage' would refuse the entry:" >&2
            sim_relay "$pf_out" >&2
        else
            echo "enter-stage: LIFECYCLE_KIT_ENTRY_PREFLIGHT command for '$stage' refuses the entry — nothing written:" >&2
            printf '%s\n' "$pf_out" >&2
        fi
        relay_help "$HELP_PREFLIGHT"
        # spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the refusal names the configured ledger and its state, so a typo'd path cannot masquerade as a never-armed valve; the cause rides the same line because a valve stated without the one deadlock it is for reads as a generic bypass
        if [[ -n "$VALVE" ]]; then
            if [[ -f "$VALVE" ]]; then
                valve_why="carries no 'armed' line for '$cur_iter $stage'"
            else
                valve_why="does not exist (header-only is its resting state, so this is 'not armed' rather than an error — check the path if you meant to arm it)"
            fi
            relay_help "or, for the one cause the pre-flight valve is sanctioned for — a stage whose entry pre-flight is refused by a precondition only a later stage can clear — append '$cur_iter $stage armed <reason>' to the valve ledger $VALVE, which $valve_why, and re-run enter-stage $stage. Reaching for it twice in one iteration is the failure rather than a supported mode, and the admitted entry prints the count that makes the second reach visible."
        fi
        exit 1
    fi
done

# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the predecessor-journal assertion, the same refusal contract as the boundary-precondition family: exit 1, the expected path printed, nothing written. Non-boundary only, because the first stage of an iteration has no predecessor journal by construction — this very entry's scratch wipe is the journal's named reclaim path. The predecessor is the cursor, the last stamp's stage, so a second session of one stage asserts the first session's journal at the same derived path.
if [[ "$first" == 0 && "$LIFECYCLE_KIT_STAGE_JOURNAL_REQUIRE" == "1" ]]; then
    pred_stage="$(lifecycle_current_stage "$STATE")"
    if [[ -n "$pred_stage" ]]; then
        pred_journal="$(lifecycle_stage_journal "$pred_stage")"
        pred_why=""
        if [[ ! -f "$pred_journal" ]]; then
            pred_why="does not exist"
        elif [[ ! -s "$pred_journal" ]]; then
            pred_why="is empty"
        fi
        if [[ -n "$pred_why" ]]; then
            if [[ "$sim" == 1 ]]; then
                echo "enter-stage (simulate): entry to '$stage' would be refused — the predecessor stage '$pred_stage' left no resume journal: $pred_journal $pred_why." >&2
            else
                echo "enter-stage: entry to '$stage' refused — the predecessor stage '$pred_stage' left no resume journal: $pred_journal $pred_why (nothing written)." >&2
            fi
            relay_help "write $pred_journal yourself, stating plainly that '$pred_stage' left none, then re-run enter-stage $stage. The assertion is evadable by design and this is the escape: what it buys is that the absence becomes deliberate and written instead of silent and unnoticed, at the one moment someone is looking."
            exit 1
        fi
    fi
fi

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

# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the iteration-boundary linked-worktree refusal, the same contract as the two above: at an iteration boundary no linked worktree should be live, an in-flight dispatch being something that must not straddle a boundary and everything else being residue. Read off 'git worktree list' and never off 'git status' — an ignored worktree leaves the status clean while it still stands, so a status-derived check reports success on exactly the state it exists to catch. Both classes refuse: a live one because an in-flight dispatch must not straddle the boundary, an orphaned one because residue must be cleared before it is crossed; what the class changes is the remedy named, since '--force' is wrong advice for a holder that is still working.
if [[ "$first" == 1 && "$LIFECYCLE_KIT_BOUNDARY_WORKTREE_CHECK" == "1" ]] \
    && git rev-parse --git-dir &>/dev/null; then
    mapfile -t wt_rows < <(worktree_scan)
    if [[ ${#wt_rows[@]} -gt 0 ]]; then
        wt_lines=()
        wt_live=0
        wt_orphaned=0
        wt_unclassified=0
        for wt_row in "${wt_rows[@]}"; do
            IFS=$'\t' read -r wt_class wt_pid wt_path wt_head <<<"$wt_row"
            case "$wt_class" in
                live)
                    wt_live=$((wt_live + 1))
                    wt_lines+=("live         $wt_path — held by pid $wt_pid")
                    ;;
                orphaned)
                    wt_orphaned=$((wt_orphaned + 1))
                    wt_lines+=("orphaned     $wt_path — $(worktree_loss "$wt_path" "$wt_head")")
                    ;;
                *)
                    wt_unclassified=$((wt_unclassified + 1))
                    wt_lines+=("unclassified $wt_path — $(worktree_loss "$wt_path" "$wt_head")")
                    ;;
            esac
        done
        if [[ "$sim" == 1 ]]; then
            echo "enter-stage (simulate): iteration-boundary entry to '$stage' would be refused — ${#wt_rows[@]} linked worktree(s) still stand:" >&2
            sim_relay "$(printf '%s\n' "${wt_lines[@]}")" >&2
        else
            echo "enter-stage: iteration-boundary entry to '$stage' refused — ${#wt_rows[@]} linked worktree(s) still stand (nothing written):" >&2
            printf '%s\n' "${wt_lines[@]}" >&2
        fi
        [[ "$wt_live" -gt 0 ]] && relay_help "a live worktree's holder is still working: wait for the named pid to return, then re-run enter-stage $stage. Do not remove it and do not force it — the reap advice below is for the other classes."
        [[ "$wt_orphaned" -gt 0 ]] && relay_help "an orphaned worktree's holder is gone, so its lock states a fact that has become false: reap it with 'git worktree remove --force --force <path>' — git requires --force TWICE to remove a LOCKED worktree, once being enough only for an unlocked dirty one — and delete the branch ref it leaves behind, since 'worktree remove' clears the directory only and a reap that stops there accretes refs this check cannot see."
        [[ "$wt_unclassified" -gt 0 ]] && relay_help "reap each path with 'git worktree remove <path>' (or --force where the child left it locked) and delete the branch ref it leaves behind — 'worktree remove' clears the directory only, so a reap that stops there accretes refs this check cannot see. Then re-run enter-stage $stage."
        exit 1
    fi
fi

# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the mid-iteration worktree advisory: the same scan away from the boundary, orphaned paths only and never a refusal. It closes the boundary refusal's stated within-iteration blind spot, and it is safe only because the class exists — an unclassified report here would name every in-flight dispatch, which mid-iteration is the normal state.
if [[ "$first" == 0 && "$LIFECYCLE_KIT_BOUNDARY_WORKTREE_CHECK" == "1" ]] \
    && git rev-parse --git-dir &>/dev/null; then
    adv_lines=()
    while IFS=$'\t' read -r wt_class wt_pid wt_path wt_head; do
        [[ "$wt_class" == "orphaned" ]] || continue
        adv_lines+=("orphaned     $wt_path — $(worktree_loss "$wt_path" "$wt_head")")
    done < <(worktree_scan)
    if [[ ${#adv_lines[@]} -gt 0 ]]; then
        if [[ "$sim" == 1 ]]; then
            echo "enter-stage (simulate): ${#adv_lines[@]} orphaned worktree(s) stand — advisory, this entry would not refuse:" >&2
            sim_relay "$(printf '%s\n' "${adv_lines[@]}")" >&2
        else
            echo "enter-stage: ${#adv_lines[@]} orphaned worktree(s) stand — advisory, this entry is not refused:" >&2
            printf '%s\n' "${adv_lines[@]}" >&2
        fi
        relay_help "the holder of each is gone: reap with 'git worktree remove --force --force <path>' and delete the branch ref it leaves behind. Left standing they refuse the next iteration boundary."
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
    # spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — --simulate reports the would-be admission and leaves the ledger byte-identical: the real entry would proceed, so the mode's verdict is exit 0, and the line it names is the one a real entry would consume
    if [[ ${#valve_report[@]} -gt 0 ]]; then
        valve_emit
        sim_relay "the valve line at $VALVE:$valve_line would be consumed (state 'armed' -> 'used'); its reason: $valve_reason"
        sim_relay "this iteration already carries $valve_used used valve line(s) — no write, the ledger is untouched."
    fi
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

# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the consumption rides the write, not the match: this tool writes no ledger line ever, it rewrites the state token of exactly one line the arming session already wrote, so the ledger's line set stays the arming session's alone and this tool can only narrow what is admissible
if [[ ${#valve_report[@]} -gt 0 ]]; then
    valve_consume "$VALVE" "$valve_line"
fi

# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the boundary scratch wipe, distinct from the truncate above (that rewrites a tracked file to its header; this deletes untracked scratch outright). Runs last so this run's own enter-stage.*.$$ temporaries are already gone and never candidates. '.gitkeep' is the kit invariant LIFECYCLE_KIT_BOUNDARY_PRESERVE cannot unset — a consumer that tracks its scratch dir's scaffolding must not have it deleted at the moment the boundary reset is committed.
if [[ "$first" == 1 && -d "$tmpdir" ]]; then
    wipe_args=(-mindepth 1 -depth ! -name .gitkeep)
    for bp in ${LIFECYCLE_KIT_BOUNDARY_PRESERVE[@]+"${LIFECYCLE_KIT_BOUNDARY_PRESERVE[@]}"}; do
        wipe_args+=(! -name "$bp")
    done
    # spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — stderr is suppressed because a preserved basename inside a doomed subdirectory makes that subdirectory's own delete fail; the failure is noise, never an abort
    mapfile -t wiped < <(find "$tmpdir" "${wipe_args[@]}" -print -delete 2>/dev/null)
fi

if [[ ${#valve_report[@]} -gt 0 ]]; then
    valve_emit
    echo "  valve reason: $valve_reason"
    # spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the prior-use count is delta 4's whole mechanism: nothing prohibits the second reach, the count announces it to the session taking it, in its own transcript, at the one moment someone is looking
    echo "  note: this iteration carried $valve_used used valve line(s) before this one — reaching for the valve twice in one iteration is the failure rather than a supported mode."
    echo "  next: commit $VALVE with the stamp, and file the blocking task this reason names before the closing stage ends."
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
