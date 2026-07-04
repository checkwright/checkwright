#!/usr/bin/env bash
# graph: couples=scripts/*.sh,gate-sdk/*.sh,TASK-QUEUE.md dir=one valve=none tier=precommit
# spec: gate-sdk/SPEC.md §check-gate-exemption-tasks — every exception-list element carries a live until: task or permanent: reason
#
# usage: check-gate-exemption-tasks.sh [queue-file [dir...]]
#   queue-file defaults to $GATE_SDK_QUEUE_FILE (default: TASK-QUEUE.md); dirs
#   default to the consumer gates dir plus the kit's checks/. A missing queue
#   file simply yields zero live slugs, so every '# until:' reads stale.
set -uo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"

QUEUE="${1:-${GATE_SDK_QUEUE_FILE:-TASK-QUEUE.md}}"
if [[ $# -gt 1 ]]; then
    DIRS=("${@:2}")
else
    DIRS=("$(gate_sdk_gates_dir)" "$SDK/checks")
fi

declare -A IS_LIVE=()
while IFS= read -r slug; do
    [[ -n "$slug" ]] && IS_LIVE["$slug"]=1
done < <(awk '
    /^## New Features/   { s=1; next }
    /^## Technical Debt/ { s=1; next }
    /^## Deferred/       { s=1; next }
    /^## Done/           { s=0 }
    /^## Lessons Learned/{ s=0 }
    s {
        while (match($0, /\*\*[a-z0-9][a-z0-9-]*\*\*/)) {
            print substr($0, RSTART+2, RLENGTH-4)
            $0 = substr($0, RSTART+RLENGTH)
        }
    }
' "$QUEUE" 2>/dev/null)

parse_elements() {
    awk -v hl="$2" '
        function strip(x){ t=x; gsub(/#.*/,"",t); gsub(/"[^"]*"/,"",t); gsub(/\047[^\047]*\047/,"",t); return t }
        NR<=hl { next }
        asgn==0 && /^[[:space:]]*[A-Za-z_][A-Za-z0-9_]*=\(/ {
            asgn=NR
            open=$0; sub(/^[^(]*\(/,"",open); sub(/#.*/,"",open); gsub(/[)[:space:]]/,"",open)
            if (length(open)>0) print NR "\topen-line\t"
        }
        asgn==0 { next }
        {
            if (NR > asgn) {
                noc=$0; sub(/#.*/,"",noc)
                if (noc ~ /[^[:space:]()]/) {
                    type="none"; payload=""
                    if (match($0, /#[[:space:]]*until:[[:space:]]*[a-z0-9][a-z0-9-]*/)) {
                        type="until"; payload=substr($0, RSTART, RLENGTH)
                        sub(/.*until:[[:space:]]*/, "", payload)
                    } else if ($0 ~ /#[[:space:]]*permanent:/) {
                        type="permanent"
                    }
                    print NR "\t" type "\t" payload
                }
            }
            s2=strip($0); o=gsub(/\(/,"x",s2); c=gsub(/\)/,"x",s2)
            if (o>0) started=1
            bal += o - c
            if (started && bal<=0) exit
        }
    ' "$1"
}

errors=()
arrays=0
scan_files=()
shopt -s nullglob
for d in "${DIRS[@]}"; do
    scan_files+=("$d"/*.sh)
done
shopt -u nullglob

if [[ ${#scan_files[@]} -gt 0 ]]; then
    while IFS= read -r hdr; do
        [[ -z "$hdr" ]] && continue
        file="${hdr%%:*}"; rest="${hdr#*:}"; lineno="${rest%%:*}"
        arrays=$(( arrays + 1 ))
        while IFS=$'\t' read -r el etype slug; do
            [[ -z "$el" ]] && continue
            case "$etype" in
                permanent) : ;;
                until)
                    if [[ -z "${IS_LIVE[$slug]:-}" ]]; then
                        errors+=("$file:$el — # until: $slug does not resolve to a live task (moved to Done, or missing from $QUEUE)")
                    fi ;;
                open-line)
                    errors+=("$file:$el — exemption element(s) on the array's opening '=(' line cannot carry a per-element disposition; put each element on its own line with a # until:/# permanent: comment") ;;
                *) errors+=("$file:$el — exemption element carries neither # until: <slug> nor # permanent: <reason>") ;;
            esac
        done < <(parse_elements "$file" "$lineno")
    done < <(grep -nE '^[[:space:]]*# exception-list:' "${scan_files[@]}" /dev/null 2>/dev/null)
fi

if [[ ${#errors[@]} -gt 0 ]]; then
    echo "GATE-EXEMPTION-TASKS: ${#errors[@]} violation(s):"
    printf '  %s\n' "${errors[@]}"
    echo "  help: annotate each exemption element '# until: <live-slug>' or '# permanent: <reason>'"
    exit 1
fi
echo "GATE-EXEMPTION-TASKS: clean ($arrays exemption array(s); every element declares until-with-live-task or permanent-with-reason)"
exit 0
