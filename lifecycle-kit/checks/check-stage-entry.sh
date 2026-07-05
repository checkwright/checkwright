#!/usr/bin/env bash
# graph: couples=TASK-QUEUE.md,.workflow/WORKFLOW-STATE.txt dir=one valve=none tier=precommit
# spec: lifecycle-kit/SPEC.md §check-stage-entry — prior-stage invocation-stamp ordering + drain-entry queue-empty + audit-trigger signal
#
# usage: check-stage-entry.sh [queue-file [state-file]]
#   Defaults: the configured queue and workflow-state files, resolved from the
#   cwd (the pre-commit hook runs at the repo root; a fixture case dir carries
#   its own copies).
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../lib/stages.sh
source "$KIT/lib/stages.sh"

QUEUE="${1:-$LIFECYCLE_QUEUE_FILE}"
STATE="${2:-$LIFECYCLE_STATE_FILE}"

[[ -f "$QUEUE" ]] || { echo "check-stage-entry: file not found: $QUEUE" >&2; exit 2; }
[[ -f "$STATE" ]] || { echo "check-stage-entry: file not found: $STATE" >&2; exit 2; }

hdr="$(lifecycle_header "$QUEUE")"
if [[ -z "$hdr" ]]; then
    echo "STAGE-ENTRY: no '## Iteration:' header in $QUEUE"
    echo "  help: add '## Iteration: <name>  [stage: <stage>]' to $QUEUE"
    exit 1
fi

iter="$(lifecycle_header_iter "$hdr")"
stage="$(lifecycle_header_stage "$hdr")"
if [[ -z "$iter" || -z "$stage" ]]; then
    echo "STAGE-ENTRY: could not parse iteration/stage from: $hdr"
    echo "  help: header must read '## Iteration: <name>  [stage: <stage>]'"
    exit 1
fi

if ! lifecycle_stage_known "$stage"; then
    echo "STAGE-ENTRY: header stage '$stage' is not a lifecycle stage (${LIFECYCLE_STAGES[*]})"
    echo "  help: set [stage:] to one of the configured lifecycle stages"
    exit 1
fi
pred="${LIFECYCLE_PREDECESSOR[$stage]:-}"

errors=()
c_fired=0

# assertion A: the entered stage's mandatory-predecessor stamp exists for this iteration
if [[ -n "$pred" ]]; then
    found="$(awk -v it="$iter" -v pr="$pred" '
        /^---[[:space:]]*$/ { f = 1; next }
        f && NF { if ($1 == it && $2 == pr) { print "1"; exit } }
    ' "$STATE")"; st=$?
    fail_closed "$st" STAGE-ENTRY "awk predecessor-stamp scan"
    [[ "$found" == "1" ]] || errors+=("entering '$stage' but no '$iter $pred' stamp in $STATE — the mandatory predecessor stage was never invoked (run /$pred, or correct the [stage:] flip)")
fi

# assertion B: a drain-stage header requires an empty active queue
if [[ -n "$LIFECYCLE_DRAIN_STAGE" && "$stage" == "$LIFECYCLE_DRAIN_STAGE" ]]; then
    secs=""
    for s in "${LIFECYCLE_ACTIVE_SECTIONS[@]}"; do secs+="${secs:+|}$s"; done
    leftover="$(awk -v secre="^## ($secs)[[:space:]]*$" '
        $0 ~ secre { inq = 1; next }
        /^## /     { inq = 0 }
        inq && /^- / { printf "    %d: %s\n", FNR, $0 }
    ' "$QUEUE")"; st=$?
    fail_closed "$st" STAGE-ENTRY "awk active-queue scan"
    [[ -z "$leftover" ]] || errors+=("entering '$stage' but the active queue is non-empty (the prior stage is not drained):"$'\n'"$leftover")
fi

# assertion C: audit-entry with a cross-component amendment signal and no audit stamp demands that stamp or a recorded waiver (lifecycle-kit/SPEC.md §check-stage-entry)
if [[ -n "$LIFECYCLE_AUDIT_STAGE" && "$stage" == "$LIFECYCLE_AUDIT_ENTRY_STAGE" ]]; then
    audit_stamp="$(awk -v it="$iter" -v au="$LIFECYCLE_AUDIT_STAGE" '
        /^---[[:space:]]*$/ { f = 1; next }
        f && NF { if ($1 == it && $2 == au) { print "1"; exit } }
    ' "$STATE")"; st=$?
    fail_closed "$st" STAGE-ENTRY "awk audit-stamp scan"

    if [[ "$audit_stamp" != "1" ]]; then
        # A roster/amendment file under a templates/ directory is a copyable
        # stub, not governed content — the same rationale spec-kit's finders
        # apply (spec-kit/lib/spec.sh spec_amendments). Excluding it keeps a
        # shipped SPEC-amendment.md template from reading as a live amendment
        # in a second component dir and false-firing the cross-component signal.
        declare -A roster=()
        while IFS= read -r sf; do
            [[ -n "$sf" ]] || continue
            sf="${sf#./}"; roster["${sf%/*}"]=1
        done < <(gate_find "." -name "$LIFECYCLE_ROSTER_BASENAME" -type f | grep -v '/templates/' || true)

        declare -A amend_dirs=()
        amend_files=()
        while IFS= read -r af; do
            [[ -n "$af" ]] || continue
            af="${af#./}"; amend_dirs["${af%/*}"]=1; amend_files+=("$af")
        done < <(gate_find "." -name "$LIFECYCLE_AMENDMENT_GLOB" -type f | grep -v '/templates/' || true)

        contract_alt=""
        for ct in "${LIFECYCLE_CONTRACT_TOKENS[@]}"; do
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
                    for ct in "${LIFECYCLE_CONTRACT_TOKENS[@]}"; do
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
            waiver="$(awk -v it="$iter" -v wv="$LIFECYCLE_WAIVER_TOKEN" '
                /^---[[:space:]]*$/ { f = 1; next }
                f && NF { if ($1 == it && $2 == wv) { print "1"; exit } }
            ' "$STATE")"; st=$?
            fail_closed "$st" STAGE-ENTRY "awk waiver scan"
            if [[ "$waiver" != "1" ]]; then
                c_fired=1
                errors+=("entering '$stage' with a cross-component amendment signal and no '$iter $LIFECYCLE_AUDIT_STAGE' stamp ($signal_detail) — the audit trigger (≥2 components' contracts) was not verified for this iteration")
            fi
        fi
    fi
fi

if [[ ${#errors[@]} -gt 0 ]]; then
    echo "STAGE-ENTRY: ${#errors[@]} prior-stage readiness issue(s) for header '$iter [stage: $stage]':"
    printf '  %s\n' "${errors[@]}"
    if [[ "$c_fired" == "1" ]]; then
        echo "  help: a cross-component $LIFECYCLE_AUDIT_ENTRY_STAGE entry must run /$LIFECYCLE_AUDIT_STAGE (stamps '$iter $LIFECYCLE_AUDIT_STAGE <session> <date>'), or — on an explicit user ruling, never self-issued by the entering session — record a deliberate waiver line '$iter $LIFECYCLE_WAIVER_TOKEN <session> <date>' in $STATE"
    else
        echo "  help: a stage flip re-verifies the prior stage's static exit — invoke the predecessor skill (it stamps $STATE) and drain the active queue before flipping to $LIFECYCLE_DRAIN_STAGE"
    fi
    exit 1
fi

if [[ -z "$pred" ]]; then
    detail="'$stage' has no mandatory predecessor"
elif [[ "$stage" == "$LIFECYCLE_DRAIN_STAGE" ]]; then
    detail="predecessor '$pred' stamped; active queue empty"
else
    detail="predecessor '$pred' stamped"
fi
echo "STAGE-ENTRY: clean ('$iter' / '$stage' — $detail)"
exit 0
