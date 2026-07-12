# shellcheck shell=bash
# spec: gate-sdk/SPEC.md §lib/inject.sh — the marker-bounded insert/replace shared by every kit's agent-file injector; block-content generation stays with the caller

# spec: gate-sdk/SPEC.md §lib/inject.sh — inject_marker_block <file> <begin> <end>; inner block content on stdin. Replaces the span between an existing marker pair (inclusive) or appends a fresh block when absent; a begin marker without its end is malformed (exit 2); the file must exist (exit 2). Echoes the action taken (appended|replaced).
inject_marker_block() {
    (
        set -euo pipefail
        file="$1"; begin="$2"; end="$3"
        [[ -f "$file" ]] \
            || { echo "inject_marker_block: target file not found: $file" >&2; exit 2; }

        tmp="$(mktemp)"; block="$(mktemp)"
        trap 'rm -f "$tmp" "$block"' EXIT
        { printf '%s\n' "$begin"; cat; printf '%s\n' "$end"; } >"$block"

        if grep -qF -- "$begin" "$file"; then
            grep -qF -- "$end" "$file" \
                || { echo "inject_marker_block: begin marker present but end marker missing in $file — refusing to guess the block bounds" >&2; exit 2; }
            awk -v b="$begin" -v e="$end" -v blockfile="$block" '
                $0 == b { skip = 1; while ((getline line < blockfile) > 0) print line; close(blockfile); next }
                $0 == e { skip = 0; next }
                !skip { print }
            ' "$file" >"$tmp"
            action="replaced"
        else
            cp "$file" "$tmp"
            printf '\n' >>"$tmp"
            cat "$block" >>"$tmp"
            action="appended"
        fi

        cp "$tmp" "$file"
        printf '%s\n' "$action"
    )
}
