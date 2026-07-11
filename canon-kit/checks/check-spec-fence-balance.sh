#!/usr/bin/env bash
# graph: couples=*SPEC*.md,*README.md,CLAUDE.md,TASK-QUEUE.md dir=one valve=none tier=precommit
# spec: canon-kit/SPEC.md §check-spec-fence-balance — every governed markdown file has an even fence-delimiter count, so the fence-skipping parsers never desync and fail open
#
# usage: check-spec-fence-balance.sh [file...]
#   Defaults to the manifest set (lib/spec.sh) plus the configured queue file.
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SDK="${GATE_SDK_ROOT:-$KIT/../gate-sdk}"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
# shellcheck source=../lib/spec.sh
source "$KIT/lib/spec.sh"

if [[ $# -gt 0 ]]; then
    files=("$@")
else
    mapfile -t files < <(spec_manifest_files ".")
    [[ -f "$CANON_KIT_QUEUE_FILE" ]] && files+=("$CANON_KIT_QUEUE_FILE")
fi

bad=(); scanned=0
for f in "${files[@]}"; do
    [[ -f "$f" ]] || continue
    scanned=$((scanned + 1))
    n="$(grep -cE '^[[:space:]]*```' "$f")"; gst=$?
    [[ "$gst" -ge 2 ]] && { echo "check-spec-fence-balance: grep failed on $f (exit $gst)" >&2; exit 2; }
    (( n % 2 == 0 )) || bad+=("$f ($n fence delimiters — odd)")
done

if [[ ${#bad[@]} -gt 0 ]]; then
    echo "check-spec-fence-balance: markdown file(s) with an odd fence-delimiter count —"
    echo "the fence-skipping parsers (embedded-source, tag-lead-line, the queue scanners)"
    echo "toggle a fence flag; an odd count desyncs it and the rest of the file fails open:"
    printf '  %s\n' "${bad[@]}"
    echo "  help: close the unbalanced code fence, or delete the stray delimiter line."
    exit 1
fi

echo "SPEC-FENCE-BALANCE: clean ($scanned governed markdown file(s), all even fence counts)"
exit 0
