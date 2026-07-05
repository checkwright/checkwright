#!/usr/bin/env bash
# graph: couples=TASK-QUEUE.md,.workflow/WORKFLOW-STATE.txt dir=one valve=none tier=precommit
# spec: lifecycle-kit/SPEC.md §check-stage-evidence — the header's current stage carries a matching skill-invocation stamp; stamp grammar + staleness
#
# usage: check-stage-evidence.sh [queue-file [state-file]]
#   Defaults: the configured queue and workflow-state files, resolved from the
#   cwd (the pre-commit hook runs at the repo root; a fixture case dir carries
#   its own copies).
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/stages.sh
source "$KIT/lib/stages.sh"

QUEUE="${1:-$LIFECYCLE_QUEUE_FILE}"
STATE="${2:-$LIFECYCLE_STATE_FILE}"

hdr="$(lifecycle_header "$QUEUE")"
if [[ -z "$hdr" ]]; then
    echo "STAGE-EVIDENCE: no '## Iteration:' header in $QUEUE"
    echo "  help: add '## Iteration: <name>  [stage: <stage>]' to $QUEUE"
    exit 1
fi

iter="$(lifecycle_header_iter "$hdr")"
stage="$(lifecycle_header_stage "$hdr")"
if [[ -z "$iter" || -z "$stage" ]]; then
    echo "STAGE-EVIDENCE: could not parse iteration/stage from: $hdr"
    echo "  help: header must read '## Iteration: <name>  [stage: <stage>]'"
    exit 1
fi

if [[ "$iter" == "—" && "$stage" != "$LIFECYCLE_FIRST_STAGE" ]]; then
    echo "STAGE-EVIDENCE: iteration is still unnamed ('—') at stage '$stage' — /$LIFECYCLE_FIRST_STAGE must name the iteration (header + stamp) before advancing past $LIFECYCLE_FIRST_STAGE"
    echo "  help: set '## Iteration: <name>  [stage: $stage]' and rewrite the matching $STATE stamp's '—' to <name>"
    exit 1
fi

[[ -f "$STATE" ]] || {
    echo "STAGE-EVIDENCE: $STATE is missing"
    echo "  help: create it — prose header, a '---' separator, then one '<iter> <stage> <session-id> <YYYY-MM-DD>' stamp per stage-skill invocation"
    exit 1
}

errors=()
found=0
# Cross-stage session distinctness: a session id is a context boundary, so two
# *different* stages of the current iteration may not share one — a stage flip
# demands a fresh session. Same-stage re-entries (a multi-session build) may
# share or rotate ids freely; waiver-token stamps are exempt (not a stage, so
# they never enter this map). Keyed session-id → the one stage that owns it.
declare -A stage_of_sid=()
while IFS= read -r line; do
    read -r f1 f2 f3 f4 rest <<<"$line"
    if [[ -z "$f4" || -n "$rest" ]]; then
        errors+=("malformed stamp (want '<iter> <stage> <session-id> <YYYY-MM-DD>'): $line"); continue
    fi
    if ! lifecycle_stage_known "$f2" && [[ -z "$LIFECYCLE_WAIVER_TOKEN" || "$f2" != "$LIFECYCLE_WAIVER_TOKEN" ]]; then
        errors+=("bad stage '$f2' (not a lifecycle stage: ${LIFECYCLE_STAGES[*]}${LIFECYCLE_WAIVER_TOKEN:+, or the waiver token $LIFECYCLE_WAIVER_TOKEN}): $line"); continue
    fi
    [[ "$f4" =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2}$ ]] || { errors+=("bad date '$f4': $line"); continue; }
    [[ "$f1" == "$iter" || ( "$f1" == "—" && "$iter" == "—" ) ]] \
        || errors+=("stamp iteration '$f1' is neither current ('$iter') nor a legal '—' bootstrap (allowed only while the header is unnamed) — stale; /$LIFECYCLE_FIRST_STAGE truncates at the iteration boundary and renames its bootstrap stamp on naming: $line")
    [[ "$f1" == "$iter" && "$f2" == "$stage" ]] && found=1
    if [[ "$f1" == "$iter" ]] && lifecycle_stage_known "$f2"; then
        if [[ -n "${stage_of_sid[$f3]:-}" && "${stage_of_sid[$f3]}" != "$f2" ]]; then
            errors+=("session id '$f3' is shared by stages '${stage_of_sid[$f3]}' and '$f2' of '$iter' — a stage flip is a context boundary and needs a fresh session (same-stage re-entries may share or rotate freely; waiver stamps are exempt): $line")
        else
            stage_of_sid["$f3"]="$f2"
        fi
    fi
done < <(awk '/^---[[:space:]]*$/{f=1; next} f && NF {print}' "$STATE")

[[ "$found" -eq 1 ]] || errors+=("no stamp for the current '$iter $stage' — run /$stage in this session (it stamps as its first step), or append the stamp, before advancing/committing the stage line")

if [[ ${#errors[@]} -gt 0 ]]; then
    echo "STAGE-EVIDENCE: ${#errors[@]} issue(s) in $STATE:"
    printf '  %s\n' "${errors[@]}"
    echo "  help: run /\$stage in this session (it stamps $STATE as its first step), or append the '<iter> <stage> <session> <date>' stamp"
    exit 1
fi
echo "STAGE-EVIDENCE: clean ('$iter' / '$stage' stamped; all stamps well-formed, current, and stage-distinct in session id)"
exit 0
