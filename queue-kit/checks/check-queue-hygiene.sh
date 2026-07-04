#!/usr/bin/env bash
# graph: couples=TASK-QUEUE.md dir=one valve=none tier=precommit
# spec: queue-kit/SPEC.md §check-queue-hygiene — the queue holds only tasks, tags, and structure: no HTML comments, no duplicate lines, no column-0 prose
#
# usage: check-queue-hygiene.sh [queue-file]
#   Defaults to the configured queue file (QUEUE_KIT_QUEUE_FILE).
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../lib/queue.sh
source "$KIT/lib/queue.sh"

FILE="${1:-$QUEUE_KIT_QUEUE_FILE}"
[[ -f "$FILE" ]] || { echo "check-queue-hygiene: file not found: $FILE" >&2; exit 2; }

out="$(awk -v leads="$(printf '%s\n' "${QUEUE_KIT_PROSE_LEADS[@]+"${QUEUE_KIT_PROSE_LEADS[@]}"}")" '
    BEGIN { nl = split(leads, L, "\n") }
    {
        if ($0 ~ /<!--/ || $0 ~ /-->/)
            printf "html\t%d\t%s\n", FNR, $0

        if ($0 ~ /^[^[:space:]]/) {                 # column-0, non-blank line
            ok = 0
            if      ($0 ~ /^#/)                ok = 1   # heading
            else if ($0 ~ /^-[[:space:]]/)    ok = 1   # bullet
            else if ($0 ~ /^---[[:space:]]*$/) ok = 1   # rule
            else {
                for (i = 1; i <= nl; i++)
                    if (L[i] != "" && index($0, L[i]) == 1) { ok = 1; break }
            }
            if (!ok) printf "prose\t%d\t%s\n", FNR, $0
        }

        if ($0 !~ /^[[:space:]]*$/ && $0 !~ /^---[[:space:]]*$/) {
            if ($0 in seen)
                printf "dup\t%d\t%s (first seen at line %d)\n", FNR, $0, seen[$0]
            else
                seen[$0] = FNR
        }
    }
' "$FILE")"; st=$?
fail_closed "$st" QUEUE-HYGIENE awk

html=(); prose=(); dup=()
while IFS=$'\t' read -r class ln text; do
    [[ -n "$class" ]] || continue
    case "$class" in
        html)  html+=("$FILE:$ln: $text") ;;
        prose) prose+=("$FILE:$ln: $text") ;;
        dup)   dup+=("$FILE:$ln: $text") ;;
    esac
done <<< "$out"

if [[ ${#html[@]} -gt 0 || ${#prose[@]} -gt 0 || ${#dup[@]} -gt 0 ]]; then
    if [[ ${#html[@]} -gt 0 ]]; then
        echo "check-queue-hygiene: HTML comment(s) in the queue (provenance belongs in git history):"
        printf '  %s\n' "${html[@]}"
        echo "  help: delete the comment; record the why in the commit message, not the queue."
    fi
    if [[ ${#dup[@]} -gt 0 ]]; then
        [[ ${#html[@]} -gt 0 ]] && echo ""
        echo "check-queue-hygiene: exact-duplicate line(s) (copy-paste artifact):"
        printf '  %s\n' "${dup[@]}"
        echo "  help: remove the duplicate; if two tasks genuinely share wording, differentiate them."
    fi
    if [[ ${#prose[@]} -gt 0 ]]; then
        { [[ ${#html[@]} -gt 0 ]] || [[ ${#dup[@]} -gt 0 ]]; } && echo ""
        echo "check-queue-hygiene: column-0 prose (every column-0 line must be a heading,"
        echo "a '- ' bullet, '---', or a configured QUEUE_KIT_PROSE_LEADS lead):"
        printf '  %s\n' "${prose[@]}"
        echo "  help: indent the prose to a continuation line under its bullet, or (for a"
        echo "        recurring protocol lead) add its token to QUEUE_KIT_PROSE_LEADS."
    fi
    exit 1
fi

echo "QUEUE-HYGIENE: clean (no HTML comments, no duplicate lines, no column-0 prose in $FILE)"
exit 0
