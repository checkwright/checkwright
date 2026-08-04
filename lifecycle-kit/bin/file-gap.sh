#!/usr/bin/env bash
# spec: lifecycle-kit/SPEC.md §The committed gap inbox — the capture affordance; stamps the bullet grammar, no caller-side redirect (the kfric.sh pattern)
# usage: file-gap.sh "<gap prose>"   (required, non-empty)
#   appends one dated bullet '- <YYYY-MM-DD> — <gap prose>' to the committed gap inbox; exit 2 on misuse
#   a prose naming a live queue slug gains the recurrence marker: '- <date> — recurrence of `<slug>`: <prose>'
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/stages.sh
source "$KIT/lib/stages.sh"

REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$REPO_ROOT" 2>/dev/null || exit 1

usage() {
    printf 'usage: %s "<gap prose>"\n' "$(basename "$0")" >&2
}

if [[ $# -ne 1 || -z "${1:-}" ]]; then
    usage
    exit 2
fi

INBOX="$LIFECYCLE_KIT_GAP_INBOX_FILE"
mkdir -p "$(dirname "$INBOX")" 2>/dev/null || true
# spec: lifecycle-kit/SPEC.md §The committed gap inbox — seed the contract header when the inbox does not yet exist (a fresh consumer's first filing); close's drain truncates back to this header
[[ -f "$INBOX" ]] \
    || printf '# contract: lifecycle-kit/SPEC.md §The committed gap inbox — append-only mid-iteration gap capture, close-drained; one bullet per gap below.\n' > "$INBOX"

# spec: lifecycle-kit/SPEC.md §The committed gap inbox — resolve the prose against the live slug set: every column-0 entry bullet outside the fixed-spelling Lessons section, which is active + deferred + configured icebox with done excluded by grammar (a bare-slug line) and Lessons by name (a lesson may be written in the entry shape); longest match wins, and this awk is lifecycle-kit's own rather than queue-kit's queue_live_slugs, which would close a cross-kit cycle
_fg_slug=""
if [[ -f "$LIFECYCLE_KIT_QUEUE_FILE" ]]; then
    _fg_slug="$(awk -v prose="$1" '
        function bounded(hay, needle,   pos, off, before, after) {
            off = 0
            while ((pos = index(substr(hay, off + 1), needle)) > 0) {
                pos += off
                before = (pos == 1) ? "" : substr(hay, pos - 1, 1)
                after = substr(hay, pos + length(needle), 1)
                if (before !~ /[a-z0-9-]/ && after !~ /[a-z0-9-]/) return 1
                off = pos
            }
            return 0
        }
        /^## Lessons Learned[[:space:]]*$/ { inl = 1; next }
        /^## / { inl = 0 }
        inl { next }
        /^-[[:space:]]+\*\*[a-z0-9][a-z0-9-]*\*\*[[:space:]]/ {
            match($0, /\*\*[a-z0-9][a-z0-9-]*\*\*/)
            live[substr($0, RSTART + 2, RLENGTH - 4)] = 1
        }
        END {
            hay = tolower(prose); best = ""
            for (s in live)
                if (length(s) > length(best) && bounded(hay, s)) best = s
            if (best != "") print best
        }
    ' "$LIFECYCLE_KIT_QUEUE_FILE" 2>/dev/null)"
fi

if [[ -n "$_fg_slug" ]]; then
    line="- $(date +%F) — recurrence of \`$_fg_slug\`: $1"
else
    line="- $(date +%F) — $1"
fi
printf '%s\n' "$line" >> "$INBOX"
printf 'file-gap: %s\n' "$line"

if [[ -n "$_fg_slug" ]]; then
    printf 'file-gap: this finding is already filed under `%s` — recorded as a recurrence; the closing stage'"'"'s drain stamps the date onto that entry.\n' \
        "$_fg_slug" >&2
fi

# spec: lifecycle-kit/SPEC.md §The committed gap inbox — warn at the point of
#   capture, while the filer can still act: after the iteration's last stage
#   stamps there is no drainer left in the machine.
_fg_stage="$(lifecycle_current_stage)"
if [[ "$_fg_stage" == "${LIFECYCLE_KIT_STAGES[-1]}" ]]; then
    printf 'file-gap: WARNING — the cursor is at %s, the last stage of the iteration. Disposition this bullet before the iteration ends: once that stage has finished, none is left to drain it, and the next %s entry refuses until an entering session promotes it directly.\n' \
        "$_fg_stage" "$LIFECYCLE_KIT_FIRST_STAGE" >&2
else
    printf 'file-gap: this bullet blocks the next %s entry until close drains it.\n' \
        "$LIFECYCLE_KIT_FIRST_STAGE" >&2
fi
