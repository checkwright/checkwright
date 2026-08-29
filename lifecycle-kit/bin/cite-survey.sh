#!/usr/bin/env bash
# spec: lifecycle-kit/SPEC.md §The survey record — the citation affordance: emits one record block as an inline-ready snippet, so carrying a finding onto a permanent surface is one command rather than a pointer that dies at the next boundary
# usage: cite-survey.sh "<heading-substring>"   (one, non-empty)
#   writes the matched block's heading and all five witness fields to stdout as
#   markdown for the author to paste; exit 2 on no match, an ambiguous match, or an empty record
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/stages.sh
source "$KIT/lib/stages.sh"

REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$REPO_ROOT" 2>/dev/null || exit 1

usage() {
    printf 'usage: %s [-h|--help] [--] "<heading-substring>"\n' "$(basename "$0")"
    printf '  emits the one matching block of %s as an inline-ready snippet\n' \
        "$LIFECYCLE_KIT_SURVEY_RECORD_FILE"
}

# spec: gate-sdk/SPEC.md §The bin/-tool contract — free-text positionals validate shape, not only arity: help is stdout at exit 0, an unrecognized leading '-' is refused, '--' ends option processing
if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
    usage
    exit 0
fi
if [[ "${1:-}" == "--" ]]; then
    shift
else
    for _cs_arg in "$@"; do
        [[ "$_cs_arg" == -* ]] || continue
        printf '%s: unrecognized option: %s — a substring beginning with "-" is passed after a "--" separator\n' \
            "$(basename "$0")" "$_cs_arg" >&2
        usage >&2
        exit 2
    done
fi

if [[ $# -ne 1 || -z "${1:-}" ]]; then
    usage >&2
    exit 2
fi

RECORD="$LIFECYCLE_KIT_SURVEY_RECORD_FILE"
if [[ ! -f "$RECORD" ]]; then
    printf 'cite-survey: no survey record at %s — nothing to cite.\n' "$RECORD" >&2
    exit 2
fi

mapfile -t _cs_headings < <(grep -n '^## ' "$RECORD" | grep -F -- "$1" || true)
if [[ "${#_cs_headings[@]}" -eq 0 ]]; then
    printf 'cite-survey: no block heading in %s contains: %s\n' "$RECORD" "$1" >&2
    printf 'cite-survey: the record carries these headings —\n' >&2
    grep '^## ' "$RECORD" >&2 || printf '  (none)\n' >&2
    exit 2
fi
# spec: lifecycle-kit/SPEC.md §The survey record — an ambiguous substring is a refusal rather than a first-match guess: the author asked for one finding and a silently-chosen sibling would be pasted onto a permanent surface as if it were the one they read
if [[ "${#_cs_headings[@]}" -gt 1 ]]; then
    printf 'cite-survey: %s block headings contain "%s" — narrow the substring:\n' "${#_cs_headings[@]}" "$1" >&2
    printf '  %s\n' "${_cs_headings[@]}" >&2
    exit 2
fi

_cs_start="${_cs_headings[0]%%:*}"

block="$(awk -v start="$_cs_start" '
    FNR == start { inb = 1; print; next }
    inb && /^## / { exit }
    inb && /^- (corpus|oracle|rev|edges|finding): / { print }
' "$RECORD")"; st=$?
[[ "$st" -eq 0 ]] || {
    printf 'cite-survey: awk exited %s reading %s — the block could not be extracted.\n' "$st" "$RECORD" >&2
    exit 2
}

heading="$(head -1 <<<"$block")"
heading="${heading#\#\# }"

printf '%s\n' "$block" | awk -v h="$heading" '
    NR == 1 { printf "**Carried survey — %s**\n", h; next }
    { print }
'
