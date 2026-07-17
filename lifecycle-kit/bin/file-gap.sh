#!/usr/bin/env bash
# spec: lifecycle-kit/SPEC.md §The committed gap inbox — the capture affordance; stamps the bullet grammar, no caller-side redirect (the kfric.sh pattern)
# usage: file-gap.sh "<gap prose>"   (required, non-empty)
#   appends one dated bullet '- <YYYY-MM-DD> — <gap prose>' to the committed gap inbox; exit 2 on misuse
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

line="- $(date +%F) — $1"
printf '%s\n' "$line" >> "$INBOX"
printf 'file-gap: %s\n' "$line"
