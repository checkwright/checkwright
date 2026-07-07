#!/usr/bin/env bash
# spec: lifecycle-kit/SPEC.md §bin/session-id.sh — the per-stage stamp id, read from the newest transcript (not hand-picked)
set -uo pipefail

sessions_dir() {
    if [[ -n "${LIFECYCLE_SESSIONS_DIR:-}" ]]; then
        printf '%s\n' "$LIFECYCLE_SESSIONS_DIR"
        return 0
    fi
    local home slug
    home="${CLAUDE_CONFIG_DIR:-$HOME/.claude}"
    slug="$(pwd | sed 's/[^a-zA-Z0-9]/-/g')"
    printf '%s/projects/%s\n' "$home" "$slug"
}

dir="$(sessions_dir)"
[[ -d "$dir" ]] || {
    echo "session-id: sessions dir not found: $dir" >&2
    echo "  help: set LIFECYCLE_SESSIONS_DIR to the agent transcript directory for this tree." >&2
    exit 2
}

newest=""
shopt -s nullglob
for f in "$dir"/*.jsonl; do
    [[ -z "$newest" || "$f" -nt "$newest" ]] && newest="$f"
done
shopt -u nullglob

[[ -n "$newest" ]] || {
    echo "session-id: no transcript (*.jsonl) under $dir" >&2
    echo "  help: confirm this is the right sessions dir (LIFECYCLE_SESSIONS_DIR)." >&2
    exit 2
}

base="${newest##*/}"       # <uuid>.jsonl
base="${base%.jsonl}"      # <uuid>
printf '%s\n' "${base:0:8}"
