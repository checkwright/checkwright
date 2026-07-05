#!/usr/bin/env bash
# spec: lifecycle-kit/SPEC.md §bin/session-id.sh — the per-stage stamp id, read from the newest transcript (not hand-picked)
#
# usage: session-id.sh
#   Prints the canonical session id for a WORKFLOW-STATE stamp: the first 8 hex
#   chars of the most recently written agent-session transcript under the
#   sessions dir. The id rotates per session (including across a context clear),
#   which is exactly the per-stage provenance the stamp needs. The stage skills
#   read it from here so the stamped id is observed, never guessed.
#
#   Sessions dir (default): the agent's per-project transcript directory,
#   derived from the config home ($CLAUDE_CONFIG_DIR, else ~/.claude) and the
#   cwd (each non-alphanumeric char mapped to '-'), i.e.
#   <config-home>/projects/<cwd-slug>. Override with LIFECYCLE_SESSIONS_DIR.
#
#   Newest-file selection is the documented single-operator assumption: one
#   live session per project tree. Exit 2 if the dir or a transcript is absent.
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

# Newest *.jsonl by mtime via bash -nt (no ls parsing; transcript names are
# UUIDs, so no whitespace concerns).
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
