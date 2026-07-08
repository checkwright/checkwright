#!/usr/bin/env bash
# graph: couples=.workflow/validate-evidence.txt,TASK-QUEUE.md,.workflow/WORKFLOW-STATE.txt dir=one valve=none tier=precommit
# spec: evidence-kit/SPEC.md §check-evidence-manifest — (B) manifest grammar + current-iteration scoping; with lifecycle configured also (A) close-entry green block and (C) validate-stamp↔evidence coupling
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/evidence.sh
source "$KIT/lib/evidence.sh"

MANIFEST="${1:-$EVIDENCE_KIT_MANIFEST_FILE}"
QUEUE="${2:-$EVIDENCE_KIT_QUEUE_FILE}"
STATE="${3:-$EVIDENCE_KIT_STATE_FILE}"

[[ -f "$MANIFEST" ]] || {
    echo "EVIDENCE-MANIFEST: manifest not found: $MANIFEST"
    echo "  help: seed it with a '# contract: $EVIDENCE_MANIFEST_CONTRACT' header; run-validate appends one line per suite"
    exit 1
}

iter=""; stage=""
iter="$(ek_queue_iteration "$QUEUE" 2>/dev/null || true)"
stage="$(ek_queue_stage "$QUEUE" 2>/dev/null || true)"

# assertion B: every manifest line is the eight-field shape and carries the current iteration
grammar_errs=()
declare -A clean_suite_date=()
have_line_for_iter=0
while IFS= read -r line; do
    read -r f1 f2 f3 f4 f5 f6 f7 f8 rest <<<"$line"
    if [[ -z "$f8" || -n "$rest" ]]; then
        grammar_errs+=("malformed line (want '<iteration> <suite> sha256=… pass=… fail=… ignore=… verdict=… <date>'): $line"); continue
    fi
    ok=1
    [[ "$f3" =~ ^sha256=[0-9a-f]{64}$ ]] || { grammar_errs+=("bad sha256 field '$f3': $line"); ok=0; }
    [[ "$f4" =~ ^pass=[0-9]+$ ]]         || { grammar_errs+=("bad pass field '$f4': $line"); ok=0; }
    [[ "$f5" =~ ^fail=[0-9]+$ ]]         || { grammar_errs+=("bad fail field '$f5': $line"); ok=0; }
    [[ "$f6" =~ ^ignore=[0-9]+$ ]]       || { grammar_errs+=("bad ignore field '$f6': $line"); ok=0; }
    [[ "$f7" =~ ^verdict=(clean|new-failures)$ ]] || { grammar_errs+=("bad verdict field '$f7' (want verdict=clean|new-failures): $line"); ok=0; }
    [[ "$f8" =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2}$ ]]   || { grammar_errs+=("bad date '$f8': $line"); ok=0; }
    [[ "$ok" -eq 1 ]] || continue
    if [[ -n "$iter" && "$f1" != "$iter" ]]; then
        grammar_errs+=("foreign iteration '$f1' (current is '$iter') — the iteration-boundary truncation was skipped: $line"); continue
    fi
    have_line_for_iter=1
    if [[ "$f7" == "verdict=clean" ]]; then
        clean_suite_date["$f2"]="$f8"
    fi
done < <(ek_data_lines "$MANIFEST")

if [[ ${#grammar_errs[@]} -gt 0 ]]; then
    echo "EVIDENCE-MANIFEST: ${#grammar_errs[@]} grammar issue(s) in $MANIFEST:"
    printf '  %s\n' "${grammar_errs[@]}"
    echo "  help: every line is the eight-field '$EVIDENCE_MANIFEST_CONTRACT' shape and carries the current iteration; a foreign line means the iteration-boundary truncation (LIFECYCLE_BOUNDARY_TRUNCATE) did not clear the manifest"
    exit 1
fi

if [[ -z "$iter" || ! -f "$STATE" ]]; then
    echo "EVIDENCE-MANIFEST: clean (grammar holds in $MANIFEST; no lifecycle state — close-entry/stamp-coupling disarmed)"
    exit 0
fi

earliest_validate=""
have_validate=0
while read -r s1 s2 _ s4 _; do
    [[ "$s1" == "$iter" && "$s2" == "validate" ]] || continue
    have_validate=1
    if [[ -z "$earliest_validate" || "$s4" < "$earliest_validate" ]]; then earliest_validate="$s4"; fi
done < <(awk '/^---[[:space:]]*$/{f=1; next} f && NF {print}' "$STATE")

errors=()

# assertion C: a validate stamp demands ≥1 evidence line, re-armed only once the header has advanced past validate (the entry flip precedes the suites)
if [[ "$have_validate" -eq 1 && "$stage" != "validate" && "$have_line_for_iter" -eq 0 ]]; then
    errors+=("iteration '$iter' has a validate stamp but no evidence line — validate ran and recorded nothing (run evidence-kit/bin/run-validate.sh)")
fi

# assertion A: a close-entry header requires the full green block — every configured suite a clean line dated on/after the earliest validate stamp
if [[ "$stage" == "close" ]]; then
    for suite in ${EVIDENCE_KIT_SUITES[@]+"${EVIDENCE_KIT_SUITES[@]}"}; do
        d="${clean_suite_date[$suite]:-}"
        if [[ -z "$d" ]]; then
            errors+=("close entry: suite '$suite' has no clean evidence line for '$iter'")
        elif [[ -n "$earliest_validate" && "$d" < "$earliest_validate" ]]; then
            errors+=("close entry: suite '$suite' clean line is dated $d, before the earliest validate stamp $earliest_validate — stale evidence")
        fi
    done
fi

if [[ ${#errors[@]} -gt 0 ]]; then
    echo "EVIDENCE-MANIFEST: ${#errors[@]} issue(s) coupling $MANIFEST to $STATE:"
    printf '  %s\n' "${errors[@]}"
    echo "  help: record a run-validate evidence line per suite before flipping to close; the entry stamp proves invocation, the evidence line proves the green result"
    exit 1
fi
echo "EVIDENCE-MANIFEST: clean (grammar + close-entry/stamp-coupling hold for '$iter' [stage: ${stage:-?}] in $MANIFEST)"
exit 0
