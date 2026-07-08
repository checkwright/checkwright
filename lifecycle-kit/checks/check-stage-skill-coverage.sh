#!/usr/bin/env bash
# graph: couples=.claude/commands/*.md dir=one valve=none tier=precommit
# spec: lifecycle-kit/SPEC.md §check-stage-skill-coverage — the configured stage set and the skills dir cover each other: every stage has a skill, every enter-stage-invoking skill names a live stage
#
# usage: check-stage-skill-coverage.sh [skills-dir]
#   Defaults to LIFECYCLE_SKILLS_DIR (.claude/commands).
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../lib/stages.sh
source "$KIT/lib/stages.sh"

DIR="${1:-$LIFECYCLE_SKILLS_DIR}"
[[ -d "$DIR" ]] || { echo "check-stage-skill-coverage: skills dir not found: $DIR" >&2; exit 2; }

missing=(); orphan=()

for s in "${LIFECYCLE_STAGES[@]}"; do
    [[ -f "$DIR/$s.md" ]] || missing+=("$s (expected $DIR/$s.md)")
done

shopt -s nullglob
for f in "$DIR"/*.md; do
    mapfile -t names < <(grep -oE 'enter-stage\.sh[[:space:]]+[a-z][a-z-]*' "$f" \
        | sed -E 's/.*enter-stage\.sh[[:space:]]+//' | sort -u)
    for n in "${names[@]}"; do
        lifecycle_stage_known "$n" || orphan+=("$(basename "$f") invokes enter-stage.sh '$n', not a lifecycle stage")
    done
done
shopt -u nullglob

if [[ ${#missing[@]} -gt 0 || ${#orphan[@]} -gt 0 ]]; then
    echo "check-stage-skill-coverage: stage set (${LIFECYCLE_STAGES[*]}) and skills dir $DIR"
    echo "are out of sync — a stage with no skill cannot be entered; an orphan stage skill"
    echo "is a retired stage's dead entry point:"
    for m in "${missing[@]}"; do echo "  no skill for stage: $m"; done
    for o in "${orphan[@]}"; do echo "  orphan skill:       $o"; done
    echo "  help: add the missing <stage>.md skill, or retire the orphan skill / fix the"
    echo "        stage name it invokes. The stage set is LIFECYCLE_STAGES (lifecycle-stages.sh)."
    exit 1
fi

echo "STAGE-SKILL-COVERAGE: clean (${#LIFECYCLE_STAGES[@]} stage(s) each have a skill; every enter-stage-invoking skill in $DIR names a live stage)"
exit 0
