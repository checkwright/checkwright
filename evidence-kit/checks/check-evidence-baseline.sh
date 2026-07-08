#!/usr/bin/env bash
# graph: couples=.workflow/validate-baseline.txt,TASK-QUEUE.md dir=one valve=none tier=precommit
# spec: evidence-kit/SPEC.md §check-evidence-baseline — held-constant baseline grammar, blocking-slug liveness, and (when scenario globs are configured) manifest↔disk set equality
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/evidence.sh
source "$KIT/lib/evidence.sh"

BASELINE="${1:-$EVIDENCE_KIT_BASELINE_FILE}"
QUEUE="${2:-$EVIDENCE_KIT_QUEUE_FILE}"

[[ -f "$BASELINE" ]] || {
    echo "EVIDENCE-BASELINE: baseline not found: $BASELINE"
    echo "  help: create $BASELINE with a comment header and one '<suite> <scenario> <status> [<slug>]' line per known scenario"
    exit 1
}

errors=()

declare -A slug_of=()
while IFS= read -r line; do
    read -r suite scenario status slug rest <<<"$line"
    if [[ -z "$suite" || -z "$scenario" || -z "$status" ]]; then
        errors+=("malformed line (want '<suite> <scenario> <status> [<slug>]'): $line"); continue
    fi
    if [[ -n "$rest" ]]; then
        errors+=("too many fields (a slug is a single token): $line"); continue
    fi
    case "$status" in
        pass)
            [[ -z "$slug" ]] || errors+=("a 'pass' scenario takes no blocking slug: $line")
            ;;
        fail|ignore)
            if [[ -z "$slug" ]]; then
                errors+=("a '$status' scenario requires a blocking slug (a live task or permanent marker): $line")
            else
                slug_of["$slug"]=1
            fi
            ;;
        *)
            errors+=("bad status '$status' (want pass|fail|ignore): $line")
            ;;
    esac
done < <(ek_data_lines "$BASELINE")

if [[ ${#slug_of[@]} -gt 0 ]]; then
    declare -A permanent=()
    for p in ${EVIDENCE_KIT_PERMANENT_SLUGS[@]+"${EVIDENCE_KIT_PERMANENT_SLUGS[@]}"}; do permanent["$p"]=1; done
    declare -A live=() done_only=()
    if [[ -f "$QUEUE" ]]; then
        while read -r qslug qsec; do
            if [[ "$qsec" == "Done" ]]; then
                [[ -n "${live[$qslug]:-}" ]] || done_only["$qslug"]=1
            else
                live["$qslug"]=1
                unset 'done_only[$qslug]'
            fi
        done < <(awk '
            /^##[[:space:]]/ { sec = $0; sub(/^##[[:space:]]*/, "", sec); next }
            /^-[[:space:]]+\*\*[^*]+\*\*/ {
                s = $0; sub(/^-[[:space:]]+\*\*/, "", s); sub(/\*\*.*$/, "", s)
                print s, sec
            }
        ' "$QUEUE")
    fi
    for slug in "${!slug_of[@]}"; do
        [[ -n "${permanent[$slug]:-}" ]] && continue
        [[ -n "${live[$slug]:-}" ]] && continue
        if [[ -n "${done_only[$slug]:-}" ]]; then
            errors+=("blocking slug '$slug' is a Done task — stale; promote the scenario or repoint the slug")
        else
            errors+=("blocking slug '$slug' resolves to no live task in $QUEUE and no permanent marker")
        fi
    done
fi

for suite in "${!EVIDENCE_KIT_SCENARIO_GLOBS[@]}"; do
    glob="${EVIDENCE_KIT_SCENARIO_GLOBS[$suite]}"
    declare -A on_disk=() in_base=()
    shopt -s nullglob
    for f in $glob; do on_disk["$(basename "$f")"]=1; done
    shopt -u nullglob
    while read -r bsuite bscen _; do
        [[ "$bsuite" == "$suite" ]] && in_base["$bscen"]=1
    done < <(ek_data_lines "$BASELINE")
    for s in "${!in_base[@]}"; do
        [[ -n "${on_disk[$s]:-}" ]] || errors+=("suite '$suite': baseline scenario '$s' matches no file under glob '$glob'")
    done
    for s in "${!on_disk[@]}"; do
        [[ -n "${in_base[$s]:-}" ]] || errors+=("suite '$suite': on-disk scenario '$s' (glob '$glob') has no baseline line")
    done
    unset on_disk in_base
done

if [[ ${#errors[@]} -gt 0 ]]; then
    echo "EVIDENCE-BASELINE: ${#errors[@]} issue(s) in $BASELINE:"
    printf '  %s\n' "${errors[@]}"
    echo "  help: each line is '<suite> <scenario> <status> [<slug>]'; a fail/ignore carries a live blocking slug; the baseline is edited by human commit only"
    exit 1
fi
echo "EVIDENCE-BASELINE: clean ($(ek_data_lines "$BASELINE" | grep -c . || true) scenario(s); grammar, slug liveness, and coverage hold in $BASELINE)"
exit 0
