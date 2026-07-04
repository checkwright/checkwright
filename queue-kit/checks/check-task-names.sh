#!/usr/bin/env bash
# graph: couples=TASK-QUEUE.md dir=one valve=none tier=precommit
# spec: queue-kit/SPEC.md §check-task-names — task entries lead with a unique kebab slug, done entries are bare slugs, every blocked-by resolves to a live task
#
# usage: check-task-names.sh [queue-file]
#   Defaults to the configured queue file (QUEUE_KIT_QUEUE_FILE).
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../lib/queue.sh
source "$KIT/lib/queue.sh"

FILE="${1:-$QUEUE_KIT_QUEUE_FILE}"
[[ -f "$FILE" ]] || { echo "check-task-names: file not found: $FILE" >&2; exit 2; }

out="$(awk -v taskre="$QUEUE_TASK_RE" -v donere="$QUEUE_DONE_RE" -v sectre="$QUEUE_SECTION_RE" '
    $0 ~ taskre { sec = "task"; next }
    $0 ~ donere { sec = "done"; next }
    $0 ~ sectre { sec = "other"; next }

    sec == "task" && $0 ~ /^[[:space:]]*-[[:space:]]/ {
        indent = match($0, /[^[:space:]]/) - 1
        isbold = ($0 ~ /^[[:space:]]*-[[:space:]]+\*\*/)
        if (indent == 0 || isbold) {
            if ($0 ~ /^[[:space:]]*-[[:space:]]+\*\*[a-z0-9][a-z0-9-]*\*\*/) {
                match($0, /\*\*[a-z0-9][a-z0-9-]*\*\*/)
                slug = substr($0, RSTART + 2, RLENGTH - 4)
                if (slug in live)
                    printf "dup-slug\t%d\t%s (first seen at line %d)\n", FNR, slug, liveln[slug]
                else { live[slug] = 1; liveln[slug] = FNR }
            } else if (isbold) {
                if (match($0, /\*\*[^*]*\*\*/))
                    printf "invalid-slug\t%d\t%s\n", FNR, substr($0, RSTART + 2, RLENGTH - 4)
                else
                    printf "invalid-slug\t%d\t%s\n", FNR, "(unparsable bold lead-in)"
            } else {
                printf "missing-slug\t%d\t%s\n", FNR, $0
            }
        }
        s = $0
        while (match(s, /\[blocked-by:[[:space:]]*[a-z0-9][a-z0-9-]*/)) {
            ref = substr(s, RSTART, RLENGTH); sub(/\[blocked-by:[[:space:]]*/, "", ref)
            nb++; brefs[nb] = ref; breln[nb] = FNR
            s = substr(s, RSTART + RLENGTH)
        }
        next
    }

    sec == "done" && $0 ~ /^[[:space:]]*-[[:space:]]/ {
        if ($0 ~ /^[[:space:]]*-[[:space:]]+[a-z0-9][a-z0-9-]*[[:space:]]*$/) {
            d = $0; sub(/^[[:space:]]*-[[:space:]]+/, "", d); sub(/[[:space:]]*$/, "", d)
            done[d] = 1
        } else {
            printf "bad-done\t%d\t%s\n", FNR, $0
        }
        next
    }

    END {
        for (i = 1; i <= nb; i++) {
            r = brefs[i]
            if (r in live) continue
            else if (r in done) printf "stale-blocker\t%d\t%s\n", breln[i], r
            else                printf "unresolved-blocker\t%d\t%s\n", breln[i], r
        }
        printf "count\t%d\t%d\n", length(live), length(done)
    }
' "$FILE")"; st=$?
fail_closed "$st" TASK-NAMES awk

missing=(); invalid=(); dup=(); baddone=(); unresolved=(); stale=(); nlive=0; ndone=0
while IFS=$'\t' read -r class a b; do
    case "$class" in
        missing-slug)        missing+=("$FILE:$a: $b") ;;
        invalid-slug)        invalid+=("$FILE:$a: $b") ;;
        dup-slug)            dup+=("$FILE:$a: $b") ;;
        bad-done)            baddone+=("$FILE:$a: $b") ;;
        unresolved-blocker)  unresolved+=("$FILE:$a: [blocked-by: $b]") ;;
        stale-blocker)       stale+=("$FILE:$a: [blocked-by: $b]") ;;
        count)               nlive="$a"; ndone="$b" ;;
    esac
done <<< "$out"

if (( ${#missing[@]} + ${#invalid[@]} + ${#dup[@]} + ${#baddone[@]} + ${#unresolved[@]} + ${#stale[@]} > 0 )); then
    sep=0
    emit() {  # emit <header-line...> then indented findings then help; blank-separate blocks
        (( sep )) && echo ""; sep=1
    }
    if (( ${#missing[@]} > 0 )); then emit
        echo "check-task-names: task entry without a bold kebab-case slug:"
        printf '  %s\n' "${missing[@]}"
        echo "  help: lead the entry with a slug — '- **the-slug** — <prose>'."
    fi
    if (( ${#invalid[@]} > 0 )); then emit
        echo "check-task-names: task entry whose bold lead-in is not a valid slug:"
        printf '  %s\n' "${invalid[@]}"
        echo "  help: a slug matches [a-z0-9][a-z0-9-]* (lowercase kebab-case); for a"
        echo "        non-task note, use a plain or italic indented bullet instead."
    fi
    if (( ${#dup[@]} > 0 )); then emit
        echo "check-task-names: duplicate slug (active + deferred + sub-tasks are one namespace):"
        printf '  %s\n' "${dup[@]}"
        echo "  help: rename one — a slug is a task's stable handle for its whole life."
    fi
    if (( ${#baddone[@]} > 0 )); then emit
        echo "check-task-names: done entry that is not a bare slug:"
        printf '  %s\n' "${baddone[@]}"
        echo "  help: a done entry is the bare slug only — '- the-slug'; the story lives in git."
    fi
    if (( ${#unresolved[@]} > 0 )); then emit
        echo "check-task-names: blocked-by pointing at no live task:"
        printf '  %s\n' "${unresolved[@]}"
        echo "  help: name a live task (active or deferred); fix the slug or add the blocker."
    fi
    if (( ${#stale[@]} > 0 )); then emit
        echo "check-task-names: stale blocked-by pointing at a completed (done) task:"
        printf '  %s\n' "${stale[@]}"
        echo "  help: the blocker is done — remove the now-stale [blocked-by:] tag (it alone"
        echo "        keeps the entry unpickable)."
    fi
    exit 1
fi

echo "TASK-NAMES: clean ($nlive live slug(s) unique, $ndone done, all blockers resolve to live tasks in $FILE)"
exit 0
