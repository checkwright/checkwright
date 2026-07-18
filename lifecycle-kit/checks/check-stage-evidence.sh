#!/usr/bin/env bash
# graph: couples=TASK-QUEUE.md,.workflow/WORKFLOW-STATE.txt dir=one valve=none tier=precommit
# spec: lifecycle-kit/SPEC.md §check-stage-evidence — stamp grammar + name-axis agreement (staleness) between the header and every stamp; cross-stage session-id distinctness
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/stages.sh
source "$KIT/lib/stages.sh"

QUEUE="${1:-$LIFECYCLE_KIT_QUEUE_FILE}"
STATE="${2:-$LIFECYCLE_KIT_STATE_FILE}"

hdr="$(lifecycle_header "$QUEUE")"
if [[ -z "$hdr" ]]; then
    echo "STAGE-EVIDENCE: no '## Iteration:' header in $QUEUE"
    echo "  help: add '## Iteration: <name>' to $QUEUE"
    exit 1
fi

iter="$(lifecycle_header_iter "$hdr")"
if [[ -z "$iter" ]]; then
    echo "STAGE-EVIDENCE: could not parse the iteration from: $hdr"
    echo "  help: header must read '## Iteration: <name>'"
    exit 1
fi

# spec: lifecycle-kit/SPEC.md §check-stage-evidence — the cursor is read before the unnamed-iteration guard because that guard consumes both axes; a state file present but carrying no stamp is this gate's no-cursor fallback and reds with the missing-file message's sibling — an unstamped file is exactly the condition this gate exists to reject, so it must not go vacuous once the stage axis moved off the header.
[[ -f "$STATE" ]] || {
    echo "STAGE-EVIDENCE: $STATE is missing"
    echo "  help: create it — prose header, a '---' separator, then one '<iter> <stage> <session-id> <YYYY-MM-DD>' stamp per stage-skill invocation"
    exit 1
}
stage="$(lifecycle_current_stage "$STATE")"
if [[ -z "$stage" ]]; then
    echo "STAGE-EVIDENCE: $STATE carries no stamp — there is no current stage to attest"
    echo "  help: run the stage skill (it stamps as its first step), or append the '<iter> <stage> <session-id> <YYYY-MM-DD>' stamp below the '---' separator"
    exit 1
fi

if [[ "$iter" == "—" && "$stage" != "$LIFECYCLE_KIT_FIRST_STAGE" ]]; then
    echo "STAGE-EVIDENCE: iteration is still unnamed ('—') at stage '$stage' — /$LIFECYCLE_KIT_FIRST_STAGE must name the iteration (header + stamp) before advancing past $LIFECYCLE_KIT_FIRST_STAGE"
    echo "  help: set '## Iteration: <name>' and rewrite the matching $STATE stamp's '—' to <name>"
    exit 1
fi

errors=()
# spec: lifecycle-kit/SPEC.md §check-stage-evidence — cross-stage session-id distinctness (a stage flip is a context boundary)
declare -A stage_of_sid=()
while IFS= read -r line; do
    read -r f1 f2 f3 f4 rest <<<"$line"
    if [[ -z "$f4" || -n "$rest" ]]; then
        errors+=("malformed stamp (want '<iter> <stage> <session-id> <YYYY-MM-DD>'): $line"); continue
    fi
    if ! lifecycle_stage_known "$f2" && [[ -z "$LIFECYCLE_KIT_WAIVER_TOKEN" || "$f2" != "$LIFECYCLE_KIT_WAIVER_TOKEN" ]]; then
        errors+=("bad stage '$f2' (not a lifecycle stage: ${LIFECYCLE_KIT_STAGES[*]}${LIFECYCLE_KIT_WAIVER_TOKEN:+, or the waiver token $LIFECYCLE_KIT_WAIVER_TOKEN}): $line"); continue
    fi
    [[ "$f4" =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2}$ ]] || { errors+=("bad date '$f4': $line"); continue; }
    [[ "$f1" == "$iter" || ( "$f1" == "—" && "$iter" == "—" ) ]] \
        || errors+=("stamp iteration '$f1' is neither current ('$iter') nor a legal '—' bootstrap (allowed only while the header is unnamed) — stale; /$LIFECYCLE_KIT_FIRST_STAGE truncates at the iteration boundary and renames its bootstrap stamp on naming: $line")
    # spec: lifecycle-kit/SPEC.md §check-stage-evidence — the distinctness map runs only at the 'stage' posture; 'iteration' skips this check alone, attribution still rides the stamps
    if [[ "$LIFECYCLE_KIT_SESSION_BOUNDARY" == stage && "$f1" == "$iter" ]] && lifecycle_stage_known "$f2"; then
        if [[ -n "${stage_of_sid[$f3]:-}" && "${stage_of_sid[$f3]}" != "$f2" ]]; then
            errors+=("session id '$f3' is shared by stages '${stage_of_sid[$f3]}' and '$f2' of '$iter' — a stage flip is a context boundary and needs a fresh session (same-stage re-entries may share or rotate freely; waiver stamps are exempt; the 'iteration' posture of LIFECYCLE_KIT_SESSION_BOUNDARY relaxes this check): $line")
        else
            stage_of_sid["$f3"]="$f2"
        fi
    fi
done < <(awk '/^---[[:space:]]*$/{f=1; next} f && NF {print}' "$STATE")

if [[ ${#errors[@]} -gt 0 ]]; then
    echo "STAGE-EVIDENCE: ${#errors[@]} issue(s) in $STATE:"
    printf '  %s\n' "${errors[@]}"
    echo "  help: run /\$stage in this session (it stamps $STATE as its first step), or append the '<iter> <stage> <session> <date>' stamp"
    exit 1
fi
if [[ "$LIFECYCLE_KIT_SESSION_BOUNDARY" == stage ]]; then
    echo "STAGE-EVIDENCE: clean ('$iter' / '$stage' stamped; all stamps well-formed, current, and stage-distinct in session id)"
else
    echo "STAGE-EVIDENCE: clean ('$iter' / '$stage' stamped; all stamps well-formed and current; cross-stage distinctness relaxed by the 'iteration' session boundary)"
fi
exit 0
