#!/usr/bin/env bash
# graph: couples=TASK-QUEUE.md,.workflow/lesson-evidence.txt dir=one valve=none tier=precommit
# spec: lifecycle-kit/SPEC.md §check-lesson-disposition — every Lessons entry present at HEAD and gone from the worktree leaves a well-formed disposition stamp in the evidence file
#
# usage: check-lesson-disposition.sh [queue-head] [queue-worktree] [evidence-file]
#   bare compares git show HEAD:<queue> vs the worktree + LIFECYCLE_KIT_LESSON_EVIDENCE_FILE;
#   three explicit file args drive it hermetically (the check-task-conservation precedent).
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../lib/stages.sh
source "$KIT/lib/stages.sh"

cleanup=()
trap 'rm -f ${cleanup[@]+"${cleanup[@]}"}' EXIT

if [[ -n "${1:-}" ]]; then
    HEAD_FILE="$1"; WORK_FILE="${2:-}"; EVID_FILE="${3:-}"
    [[ -f "$HEAD_FILE" ]] || { echo "check-lesson-disposition: queue-head not found: $HEAD_FILE" >&2; exit 2; }
    [[ -f "$WORK_FILE" ]] || { echo "check-lesson-disposition: queue-worktree not found: $WORK_FILE" >&2; exit 2; }
    [[ -f "$EVID_FILE" ]] || { echo "check-lesson-disposition: evidence file not found: $EVID_FILE" >&2; exit 2; }
else
    QUEUE="$LIFECYCLE_KIT_QUEUE_FILE"
    git rev-parse --git-dir >/dev/null 2>&1 || {
        echo "LESSON-DISPOSITION: clean (no git repository — no HEAD baseline to compare)"; exit 0; }
    ht="$(mktemp)"; cleanup+=("$ht")
    if ! git show "HEAD:$QUEUE" >"$ht" 2>/dev/null; then
        echo "LESSON-DISPOSITION: clean ($QUEUE not at HEAD — no prior lessons to disposition)"; exit 0
    fi
    [[ -f "$QUEUE" ]] || { echo "check-lesson-disposition: worktree queue not found: $QUEUE" >&2; exit 2; }
    HEAD_FILE="$ht"; WORK_FILE="$QUEUE"; EVID_FILE="$LIFECYCLE_KIT_LESSON_EVIDENCE_FILE"
    [[ -f "$EVID_FILE" ]] || { echo "check-lesson-disposition: evidence file not found: $EVID_FILE" >&2; exit 2; }
fi

lessons_of() {  # queue-file -> one normalized Lessons lead line per top-level entry
    awk '
        /^## Lessons Learned[[:space:]]*$/ { inl = 1; next }
        /^## / { inl = 0 }
        inl && /^-[[:space:]]/ {
            line = $0
            sub(/^[[:space:]]*-[[:space:]]+/, "", line)
            sub(/[[:space:]]+$/, "", line)
            print line
        }
    ' "$1"
}

parsed="$(awk 'BEGIN { FS = " — " }
    /^[[:space:]]*#/ || /^[[:space:]]*$/ { next }
    {
        if (NF < 2 || $1 !~ /^[^ ]+ lesson (rule|task|harvest|discard) /) { print "BAD\t" $0; next }
        p = $2; for (i = 3; i <= NF; i++) p = p FS $i
        print "OK\t" p
    }
' "$EVID_FILE")"; st=$?
fail_closed "$st" LESSON-DISPOSITION awk

malformed=(); prefixes=()
while IFS=$'\t' read -r kind rest; do
    [[ -n "$kind" ]] || continue
    if [[ "$kind" == BAD ]]; then malformed+=("$rest"); else prefixes+=("$rest"); fi
done <<<"$parsed"

if [[ ${#malformed[@]} -gt 0 ]]; then
    echo "check-lesson-disposition: malformed disposition line(s) in $EVID_FILE"
    echo "(grammar: <iteration> lesson <rule <file>|task <slug>|harvest <tag>|discard <reason>> — <lead-line prefix>):"
    printf '  %s\n' "${malformed[@]}"
    echo "  help: rewrite each line to the grammar above; the ' — ' separates the disposition from the lead-line prefix it dispositions."
    exit 1
fi

declare -A in_work=()
while IFS= read -r l; do [[ -n "$l" ]] && in_work["$l"]=1; done < <(lessons_of "$WORK_FILE")

undispositioned=(); matched=0
while IFS= read -r entry; do
    [[ -n "$entry" ]] || continue
    [[ -n "${in_work[$entry]:-}" ]] && continue   # still present in the worktree — not a removal
    hit=0
    for p in ${prefixes[@]+"${prefixes[@]}"}; do
        [[ -n "$p" && "$entry" == "$p"* ]] && { hit=1; break; }
    done
    if [[ "$hit" == 1 ]]; then matched=$((matched + 1)); else undispositioned+=("$entry"); fi
done < <(lessons_of "$HEAD_FILE")

if [[ ${#undispositioned[@]} -gt 0 ]]; then
    echo "check-lesson-disposition: Lessons entr(y|ies) removed since HEAD with no disposition"
    echo "stamp in $EVID_FILE (a lesson cleared without a rule/task/harvest/discard record):"
    printf '  %s\n' "${undispositioned[@]}"
    echo "  help: stamp each removed lesson in $EVID_FILE — '<iteration> lesson <kind> <ref> — <lead-line prefix>' — or restore the entry."
    exit 1
fi

echo "LESSON-DISPOSITION: clean ($matched removed lesson(s) each matched a disposition stamp; grammar holds in $EVID_FILE)"
exit 0
