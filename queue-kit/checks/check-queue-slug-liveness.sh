#!/usr/bin/env bash
# graph: couples=TASK-QUEUE.md,scripts/*.sh dir=one valve=none tier=precommit
# spec: queue-kit/SPEC.md §check-queue-slug-liveness — every slug-shaped bold-code token in a configured prose surface resolves against the queue's live slug set
#
# usage: check-queue-slug-liveness.sh [scan-root]   (default '.')
#   Scans QUEUE_KIT_PROSE_SURFACE_GLOBS (empty default: no-op) under the root.
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../lib/queue.sh
source "$KIT/lib/queue.sh"

ROOT="${1:-.}"
[[ -d "$ROOT" ]] || { echo "check-queue-slug-liveness: not a directory: $ROOT" >&2; exit 2; }

files=()
if [[ ${#QUEUE_KIT_PROSE_SURFACE_GLOBS[@]} -gt 0 ]]; then
    shopt -s nullglob globstar
    for g in "${QUEUE_KIT_PROSE_SURFACE_GLOBS[@]}"; do
        for f in "$ROOT"/$g; do [[ -f "$f" ]] && files+=("$f"); done
    done
    shopt -u nullglob globstar
fi

if [[ ${#files[@]} -eq 0 ]]; then
    echo "QUEUE-SLUG-LIVENESS: clean (no prose surface configured in QUEUE_KIT_PROSE_SURFACE_GLOBS — nothing to resolve)"
    exit 0
fi

QUEUE="$QUEUE_KIT_QUEUE_FILE"
[[ -f "$QUEUE" ]] || QUEUE="$ROOT/$QUEUE_KIT_QUEUE_FILE"
[[ -f "$QUEUE" ]] || { echo "check-queue-slug-liveness: queue file not found: $QUEUE_KIT_QUEUE_FILE" >&2; exit 2; }

live_str="$(queue_live_slugs "$QUEUE" | tr '\n' ' ')"; st=${PIPESTATUS[0]}
fail_closed "$st" QUEUE-SLUG-LIVENESS queue_live_slugs

out="$(awk -v live="$live_str" '
    BEGIN { n = split(live, a, " "); for (i = 1; i <= n; i++) L[a[i]] = 1 }
    {
        s = $0
        while (match(s, /\*\*`[a-z0-9][a-z0-9-]*`\*\*/)) {
            tok = substr(s, RSTART + 3, RLENGTH - 6)
            if (!(tok in L)) printf "%s:%d:%s\n", FILENAME, FNR, tok
            s = substr(s, RSTART + RLENGTH)
        }
    }
' "${files[@]}")"; st=$?
fail_closed "$st" QUEUE-SLUG-LIVENESS awk

if [[ -n "$out" ]]; then
    echo "check-queue-slug-liveness: bold-code token claims queue membership but names no live task:"
    while IFS= read -r line; do printf '  %s\n' "$line"; done <<< "$out"
    echo "  help: a **\`slug\`** token claims the slug is a live queue task. If the task"
    echo "        landed, drop the bold-code form and cite its owning SPEC; otherwise fix"
    echo "        the slug or restore the task to the queue."
    exit 1
fi

echo "QUEUE-SLUG-LIVENESS: clean (${#files[@]} prose surface(s) scanned; every slug-shaped bold-code token resolves to a live task in $QUEUE)"
exit 0
