#!/usr/bin/env bash
# spec: canon-kit/SPEC.md §check-prose-enum — this repo's enum-set emitter: the queue tag vocabulary, derived (not restated) from queue-kit's own lead-line tag parser plus the configured lesson tags. One <set-name><TAB><member> line per member. Two sets, because the tags partition by role: the task/selection tags list together in prose, the Lessons channel tags list together — a paragraph naming one role's tags is not enumerating the other.
set -uo pipefail

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# spec: canon-kit/SPEC.md §check-prose-enum — the tag vocabulary is queue-kit's own parse surface: the arr["<tag>"] rows check-tag-lead-line keys on, read from the gate rather than re-listed here
mapfile -t alltags < <(grep -oE 'arr\["[a-z][a-z-]*"\]' "$REPO/queue-kit/checks/check-tag-lead-line.sh" \
    | sed -E 's/arr\["([a-z-]+)"\]/\1/' | sort -u)
[[ ${#alltags[@]} -gt 0 ]] || { echo "enum-sets: no tags parsed from check-tag-lead-line.sh" >&2; exit 2; }

# spec: canon-kit/SPEC.md §check-prose-enum — the Lessons channel: [attend] (queue-kit/SPEC.md §The Lessons Learned channel) plus this repo's configured harvest tags; the rest are task/selection tags
QUEUE_KIT_LESSON_TAGS=()
# shellcheck source=./queue-config.sh
source "$REPO/scripts/queue-config.sh"
lessons=(attend "${QUEUE_KIT_LESSON_TAGS[@]+"${QUEUE_KIT_LESSON_TAGS[@]}"}")

is_lesson() { local t="$1" l; for l in "${lessons[@]}"; do [[ "$l" == "$t" ]] && return 0; done; return 1; }

for t in "${alltags[@]}"; do
    is_lesson "$t" || printf 'queue-task-tag\t%s\n' "$t"
done
for t in "${lessons[@]}"; do
    printf 'queue-lessons-tag\t%s\n' "$t"
done
