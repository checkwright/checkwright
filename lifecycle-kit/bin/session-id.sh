#!/usr/bin/env bash
# spec: lifecycle-kit/SPEC.md §bin/session-id.sh — the per-stage stamp id via the env-first derivation order (not hand-picked)
set -uo pipefail

sessions_dir() {
    if [[ -n "${LIFECYCLE_KIT_SESSIONS_DIR:-}" ]]; then
        printf '%s\n' "$LIFECYCLE_KIT_SESSIONS_DIR"
        return 0
    fi
    local home slug
    home="${CLAUDE_CONFIG_DIR:-$HOME/.claude}"
    slug="$(pwd | sed 's/[^a-zA-Z0-9]/-/g')"
    printf '%s/projects/%s\n' "$home" "$slug"
}

normalize() {                  # strip a leading agent- token, then take the first 8 chars
    local id="${1#agent-}"
    printf '%s\n' "${id:0:8}"
}

if [[ -n "${LIFECYCLE_KIT_SESSION_ID:-}" ]]; then
    normalize "$LIFECYCLE_KIT_SESSION_ID"
    exit 0
fi

if [[ -z "${CLAUDE_CODE_CHILD_SESSION:-}" && -n "${CLAUDE_CODE_SESSION_ID:-}" ]]; then
    normalize "$CLAUDE_CODE_SESSION_ID"
    exit 0
fi

dir="$(sessions_dir)"
[[ -d "$dir" ]] || {
    echo "session-id: sessions dir not found: $dir" >&2
    echo "  help: set LIFECYCLE_KIT_SESSIONS_DIR to the agent transcript directory for this tree." >&2
    exit 2
}

newest=""
pick() {                       # advance $newest across a (possibly empty) glob
    local f
    for f in "$@"; do
        [[ -e "$f" ]] || continue
        [[ -z "$newest" || "$f" -nt "$newest" ]] && newest="$f"
    done
}

shopt -s nullglob
if [[ -n "${CLAUDE_CODE_CHILD_SESSION:-}" && -n "${CLAUDE_CODE_SESSION_ID:-}" ]]; then
    pick "$dir/${CLAUDE_CODE_SESSION_ID}/subagents"/*.jsonl   # the lead's own subagents/ dir alone
else
    pick "$dir"/*.jsonl
    pick "$dir"/*/subagents/*.jsonl
fi
shopt -u nullglob

[[ -n "$newest" ]] || {
    if [[ -n "${CLAUDE_CODE_CHILD_SESSION:-}" && -n "${CLAUDE_CODE_SESSION_ID:-}" ]]; then
        # spec: lifecycle-kit/SPEC.md §bin/session-id.sh — empty narrowed scan + a top-level
        # transcript for the env uuid marks the flag spurious: the uuid names a live top-level
        # session, not a lead, so fall back to it rather than exiting 2.
        if [[ -e "$dir/${CLAUDE_CODE_SESSION_ID}.jsonl" ]]; then
            normalize "$CLAUDE_CODE_SESSION_ID"
            exit 0
        fi
        echo "session-id: no subagent transcript under $dir/${CLAUDE_CODE_SESSION_ID}/subagents and no top-level $dir/${CLAUDE_CODE_SESSION_ID}.jsonl" >&2
    else
        echo "session-id: no transcript (*.jsonl) under $dir" >&2
    fi
    echo "  help: confirm this is the right sessions dir (LIFECYCLE_KIT_SESSIONS_DIR)." >&2
    exit 2
}

base="${newest##*/}"       # <uuid>.jsonl or agent-<hex>.jsonl
base="${base%.jsonl}"
normalize "$base"
