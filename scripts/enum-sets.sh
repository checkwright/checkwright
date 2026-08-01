#!/usr/bin/env bash
# spec: canon-kit/SPEC.md §check-prose-enum — this repo's enum-set emitter: the queue tag vocabulary plus four derived roster families over the kit tree. One <set-name><TAB><member> line per member, every member read from the tree or from the gate that owns it, never restated. The tags partition by role into two sets: the task/selection tags list together in prose, the Lessons channel tags list together — a paragraph naming one role's tags is not enumerating the other.
set -uo pipefail

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# spec: canon-kit/SPEC.md §check-prose-enum — the tag vocabulary is queue-kit's own parse surface: the class table check-tag-lead-line derives both its match literal and its arr[] key from — the gate's one quoted-literal split() — read from the gate rather than re-listed here, so a rename cannot leave the two spellings disagreeing
mapfile -t _splits < <(grep -oE 'split\("[^"]+"' "$REPO/queue-kit/checks/check-tag-lead-line.sh")
(( ${#_splits[@]} == 1 )) || { echo "enum-sets: check-tag-lead-line.sh holds ${#_splits[@]} quoted-literal split() calls, not the one class table this derivation reads; anchor the read on the class table rather than on position" >&2; exit 2; }
_tagtable="$(printf '%s\n' "${_splits[0]}" | sed -E 's/^split\("//; s/"$//')"
# shellcheck disable=SC2086  # the table is a space-separated token list; word splitting is the parse
mapfile -t alltags < <(printf '%s\n' $_tagtable | sed -E 's/[]:]$//' | sort -u)
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

# spec: canon-kit/SPEC.md §check-prose-enum — the kit-root anchor for every derived family is gate-sdk's own kit-root derivation, so the sets cannot enumerate a different tree than the battery runs on
# shellcheck source=../gate-sdk/lib/gate.sh
source "$REPO/gate-sdk/lib/gate.sh"
mapfile -t kits < <(cd "$REPO" && gate_kit_roots_rel)
(( ${#kits[@]} > 0 )) || { echo "enum-sets: gate_kit_roots_rel enumerated no kit roots" >&2; exit 2; }

# spec: canon-kit/SPEC.md §check-prose-enum — a member is a file basename, never a path: the basename matches prose spelling the file kit-relative, repo-relative or bare, because the matcher's word boundary accepts the leading slash
emit() { local sname="$1"; shift; local m; for m in "$@"; do printf '%s\t%s\n' "$sname" "$m"; done; }

for kit in "${kits[@]}"; do
    # spec: canon-kit/SPEC.md §check-prose-enum — a lib/ that tracks no top-level *.sh is a layout this derivation can no longer read, so it fail-closes rather than emitting the silently empty set
    if [[ -d "$REPO/$kit/lib" ]]; then
        mapfile -t libs < <(cd "$REPO" && git ls-files "$kit/lib" | grep -E "^$kit/lib/[^/]+\.sh\$")
        (( ${#libs[@]} > 0 )) || { echo "enum-sets: $kit/lib tracks no top-level *.sh" >&2; exit 2; }
        emit "$kit-lib" "${libs[@]##*/}"
    fi
    # spec: canon-kit/SPEC.md §check-prose-enum — a gate-tests/ holding only good/bad fixture directories ships no bespoke unit test; that empty set is a measured normal state, not a broken derivation
    if [[ -d "$REPO/$kit/gate-tests" ]]; then
        mapfile -t tests < <(cd "$REPO" && git ls-files "$kit/gate-tests" | grep -E "^$kit/gate-tests/[^/]+\.test\.sh\$")
        (( ${#tests[@]} > 0 )) && emit "$kit-gate-test" "${tests[@]##*/}"
    fi
done
