#!/usr/bin/env bash
# graph: couples=TASK-QUEUE.md,.workflow/WORKFLOW-STATE.txt dir=one valve=none tier=precommit
# spec: lifecycle-kit/SPEC.md §check-stage-entry — prior-stage invocation-stamp ordering + drain-entry queue-empty + audit-trigger signal
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../lib/stages.sh
source "$KIT/lib/stages.sh"

QUEUE="${1:-$LIFECYCLE_KIT_QUEUE_FILE}"
STATE="${2:-$LIFECYCLE_KIT_STATE_FILE}"

[[ -f "$QUEUE" ]] || { echo "check-stage-entry: file not found: $QUEUE" >&2; exit 2; }
[[ -f "$STATE" ]] || { echo "check-stage-entry: file not found: $STATE" >&2; exit 2; }

hdr="$(lifecycle_header "$QUEUE")"
if [[ -z "$hdr" ]]; then
    echo "STAGE-ENTRY: no '## Iteration:' header in $QUEUE"
    echo "  help: add '## Iteration: <name>' to $QUEUE"
    exit 1
fi

# spec: lifecycle-kit/SPEC.md §check-stage-entry — the two axes, two surfaces: the header names the iteration, the state file's last stamp is the entered stage. An empty cursor is unreachable by construction here — enter-stage hands this gate a temp state file that always carries the candidate stamp, and at pre-commit the entry commit stages that same stamp — so it stays a hard parse error rather than a disarm.
iter="$(lifecycle_header_iter "$hdr")"
stage="$(lifecycle_current_stage "$STATE")"
if [[ -z "$iter" ]]; then
    echo "STAGE-ENTRY: could not parse the iteration from: $hdr"
    echo "  help: header must read '## Iteration: <name>'"
    exit 1
fi
if [[ -z "$stage" ]]; then
    echo "STAGE-ENTRY: could not parse the entered stage — no stamp line in $STATE"
    echo "  help: the entered stage is the last '<iter> <stage> <session-id> <YYYY-MM-DD>' stamp; run the stage skill (it stamps as its first step)"
    exit 1
fi

if ! lifecycle_stage_known "$stage"; then
    echo "STAGE-ENTRY: cursor stage '$stage' is not a lifecycle stage (${LIFECYCLE_KIT_STAGES[*]})"
    echo "  help: the last stamp in $STATE must name one of the configured lifecycle stages"
    exit 1
fi
pred="${LIFECYCLE_KIT_PREDECESSOR[$stage]:-}"

errors=()
c_fired=0

# assertion A: the entered stage's mandatory-predecessor stamp exists for this iteration
if [[ -n "$pred" ]]; then
    found="$(awk -v it="$iter" -v pr="$pred" '
        /^---[[:space:]]*$/ { f = 1; next }
        f && NF { if ($1 == it && $2 == pr) { print "1"; exit } }
    ' "$STATE")"; st=$?
    fail_closed "$st" STAGE-ENTRY "awk predecessor-stamp scan"
    [[ "$found" == "1" ]] || errors+=("entering '$stage' but no '$iter $pred' stamp in $STATE — the mandatory predecessor stage was never invoked (run /$pred, or correct the entry stamp)")
fi

# assertion B: drain-entry queue-empty — [drain-exempt:] exempts at drain entry only; the drain successor's entry backstops with no exemption
b_mode=""
if [[ -n "$LIFECYCLE_KIT_DRAIN_STAGE" ]]; then
    if [[ "$stage" == "$LIFECYCLE_KIT_DRAIN_STAGE" ]]; then
        b_mode=drain
    else
        for s in "${!LIFECYCLE_KIT_PREDECESSOR[@]}"; do
            [[ "$s" == "$stage" && "${LIFECYCLE_KIT_PREDECESSOR[$s]}" == "$LIFECYCLE_KIT_DRAIN_STAGE" ]] && b_mode=successor
        done
    fi
fi
exempt_detail=""
if [[ -n "$b_mode" ]]; then
    secs=""
    for s in "${LIFECYCLE_KIT_ACTIVE_SECTIONS[@]}"; do secs+="${secs:+|}$s"; done
    scan="$(awk -v secre="^## ($secs)[[:space:]]*$" -v exempt="$([[ "$b_mode" == drain ]] && echo 1 || echo 0)" '
        $0 ~ secre { inq = 1; next }
        /^## /     { inq = 0 }
        !inq || $0 !~ /^- / { next }
        {
            if (match($0, /\[drain-exempt:[[:space:]]*[^]]*\]/)) {
                r = substr($0, RSTART, RLENGTH)
                sub(/^\[drain-exempt:[[:space:]]*/, "", r); sub(/\]$/, "", r)
                gsub(/[[:space:]]+$/, "", r)
                if (r == "")     printf "M\t%d\t%s\n", FNR, $0
                else if (exempt) printf "E\t%d\t%s\n", FNR, r
                else             printf "L\t%d\t%s\n", FNR, $0
            } else printf "L\t%d\t%s\n", FNR, $0
        }
    ' "$QUEUE")"; st=$?
    fail_closed "$st" STAGE-ENTRY "awk active-queue scan"
    leftover=""; malformed=""; n_exempt=0
    while IFS=$'\t' read -r kind ln text; do
        [[ -n "$kind" ]] || continue
        case "$kind" in
            L) leftover+=$'\n'"    $ln: $text" ;;
            M) malformed+=$'\n'"    $ln: $text" ;;
            E) n_exempt=$((n_exempt + 1)); exempt_detail+="${exempt_detail:+; }$ln: $text" ;;
        esac
    done <<< "$scan"
    if [[ -n "$leftover" ]]; then
        if [[ "$b_mode" == drain ]]; then
            errors+=("entering '$stage' but the active queue is non-empty (the prior stage is not drained):$leftover")
        else
            errors+=("entering '$stage' (the drain successor) but the active queue is non-empty — nothing may remain active past '$LIFECYCLE_KIT_DRAIN_STAGE', [drain-exempt:] included:$leftover")
        fi
    fi
    [[ -z "$malformed" ]] || errors+=("[drain-exempt:] with an empty reason is malformed (the reason is the audit trail):$malformed")
fi

# assertion C: audit-entry with a cross-component amendment signal and no audit stamp demands that stamp or a recorded waiver (lifecycle-kit/SPEC.md §check-stage-entry)
if [[ -n "$LIFECYCLE_KIT_AUDIT_STAGE" && "$stage" == "$LIFECYCLE_KIT_AUDIT_ENTRY_STAGE" ]]; then
    audit_stamp="$(awk -v it="$iter" -v au="$LIFECYCLE_KIT_AUDIT_STAGE" '
        /^---[[:space:]]*$/ { f = 1; next }
        f && NF { if ($1 == it && $2 == au) { print "1"; exit } }
    ' "$STATE")"; st=$?
    fail_closed "$st" STAGE-ENTRY "awk audit-stamp scan"

    if [[ "$audit_stamp" != "1" ]]; then
        # spec: lifecycle-kit/SPEC.md §check-stage-entry — templates/ paths excluded (a shipped stub is not a live amendment)
        declare -A roster=()
        while IFS= read -r sf; do
            [[ -n "$sf" ]] || continue
            sf="${sf#./}"; roster["${sf%/*}"]=1
        done < <(gate_find "." -name "$LIFECYCLE_KIT_ROSTER_BASENAME" -type f | grep -v '/templates/' || true)  # reads-couples-exempt: whole-tree audit-signal scan; re-fire is owned by the state-file couple every stage entry stamps

        declare -A amend_dirs=()
        amend_files=()
        while IFS= read -r af; do
            [[ -n "$af" ]] || continue
            af="${af#./}"; amend_dirs["${af%/*}"]=1; amend_files+=("$af")
        done < <(gate_find "." -name "$LIFECYCLE_KIT_AMENDMENT_GLOB" -type f | grep -v '/templates/' || true)  # reads-couples-exempt: whole-tree audit-signal scan; re-fire is owned by the state-file couple every stage entry stamps

        contract_alt=""
        for ct in "${LIFECYCLE_KIT_CONTRACT_TOKENS[@]}"; do
            contract_alt+="${contract_alt:+|}${ct//./\\.}"
        done

        signal_detail=""
        if [[ ${#amend_dirs[@]} -ge 2 ]]; then
            signal_detail="amendments span ${#amend_dirs[@]} component dirs: ${!amend_dirs[*]}"
        else
            for af in ${amend_files[@]+"${amend_files[@]}"}; do
                declare -A comps=(["${af%/*}"]=1)
                while IFS= read -r tok; do
                    [[ -n "$tok" ]] || continue
                    d="$tok"
                    for ct in "${LIFECYCLE_KIT_CONTRACT_TOKENS[@]}"; do
                        d="${d%/$ct}"; d="${d%/${ct%/}}"
                    done
                    [[ -n "${roster[$d]:-}" ]] && comps["$d"]=1
                done < <(grep -oE '[a-z0-9][a-z0-9/_-]*/('"$contract_alt"')' "$af" 2>/dev/null || true)
                if [[ ${#comps[@]} -ge 2 ]]; then
                    signal_detail="amendment $af references ${#comps[@]} components: ${!comps[*]}"
                    unset comps
                    break
                fi
                unset comps
            done
        fi

        if [[ -n "$signal_detail" ]]; then
            waiver="$(awk -v it="$iter" -v wv="$LIFECYCLE_KIT_WAIVER_TOKEN" '
                /^---[[:space:]]*$/ { f = 1; next }
                f && NF { if ($1 == it && $2 == wv) { print "1"; exit } }
            ' "$STATE")"; st=$?
            fail_closed "$st" STAGE-ENTRY "awk waiver scan"
            if [[ "$waiver" != "1" ]]; then
                c_fired=1
                errors+=("entering '$stage' with a cross-component amendment signal and no '$iter $LIFECYCLE_KIT_AUDIT_STAGE' stamp ($signal_detail) — the audit trigger (≥2 components' contracts) was not verified for this iteration")
            fi
        fi
    fi
fi

if [[ ${#errors[@]} -gt 0 ]]; then
    echo "STAGE-ENTRY: ${#errors[@]} prior-stage readiness issue(s) entering '$stage' of '$iter':"
    printf '  %s\n' "${errors[@]}"
    if [[ "$c_fired" == "1" ]]; then
        echo "  help: a cross-component $LIFECYCLE_KIT_AUDIT_ENTRY_STAGE entry must run /$LIFECYCLE_KIT_AUDIT_STAGE (stamps '$iter $LIFECYCLE_KIT_AUDIT_STAGE <session> <date>'), or — on an explicit user ruling, never self-issued by the entering session — record a deliberate waiver line '$iter $LIFECYCLE_KIT_WAIVER_TOKEN <session> <date>' in $STATE"
    else
        echo "  help: a stage entry re-verifies the prior stage's static exit — invoke the predecessor skill (it stamps $STATE) and drain the active queue before entering $LIFECYCLE_KIT_DRAIN_STAGE"
    fi
    exit 1
fi

if [[ -z "$pred" ]]; then
    detail="'$stage' has no mandatory predecessor"
elif [[ "$b_mode" == drain ]]; then
    detail="predecessor '$pred' stamped; active queue drained"
    [[ -z "$exempt_detail" ]] || detail+=" (drain-exempt residue — $exempt_detail)"
elif [[ "$b_mode" == successor ]]; then
    detail="predecessor '$pred' stamped; active queue empty (drain-successor backstop, no exemption)"
else
    detail="predecessor '$pred' stamped"
fi
echo "STAGE-ENTRY: clean ('$iter' / '$stage' — $detail)"
exit 0
