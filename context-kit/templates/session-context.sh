#!/usr/bin/env bash
# spec: context-kit/SPEC.md §The session-context hook — consumer-copy SessionStart hook (bash-guard pattern); assembles the session brief, every step guarded, never fails a session

set -uo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$REPO_ROOT" 2>/dev/null || exit 0

# spec: context-kit/SPEC.md §The session-context hook — consumer layout: vendored kit tools + governed queue file, retarget to yours [EDIT ME]
QUEUE_INDEX="queue-kit/bin/queue-index.sh"
CTX_BIN="context-kit/bin"
QUEUE_FILE="${GATE_SDK_QUEUE_FILE:-TASK-QUEUE.md}"
DRIFT_REPORT="${CONTEXT_KIT_DRIFT_REPORT:-}"

echo "── Session context (context-kit session-context hook) ──────────────────"
echo

# spec: context-kit/SPEC.md §The session-context hook — step 1 queue index
stage=""
if [[ -f "$QUEUE_FILE" ]]; then
    stage="$(awk '/^## Iteration:/ {
        if (match($0, /\[stage: *[a-z]+ *\]/)) {
            s = substr($0, RSTART, RLENGTH); gsub(/\[stage: *| *\]/, "", s)
            print s; exit
        }
    }' "$QUEUE_FILE" 2>/dev/null)"
fi
if [[ -f "$QUEUE_INDEX" ]]; then
    if [[ "$stage" == close || "$stage" == scope ]]; then
        bash "$QUEUE_INDEX" 2>/dev/null || echo "(queue-index unavailable)"
    else
        bash "$QUEUE_INDEX" --collapse-deferred 2>/dev/null || echo "(queue-index unavailable)"
    fi
    echo
fi

# spec: context-kit/SPEC.md §The session-context hook — step 2 dirty-surface pre-run; component detection + index command are layout assumptions [EDIT ME]
mapfile -t changed < <(
    git status --porcelain 2>/dev/null | awk '{ print $NF }' \
        | awk -F/ 'NF>1 { print $1 }' | sort -u \
        | while read -r d; do [[ -d "$d/src" ]] && echo "$d"; done
)
if [[ ${#changed[@]} -gt 0 && -f "$CTX_BIN/pub-index.sh" ]]; then
    echo "Uncommitted changes touch: ${changed[*]}"
    echo "Public API surface of those components (pub-index — read the file for bodies):"
    echo
    for c in "${changed[@]}"; do
        bash "$CTX_BIN/pub-index.sh" "$c/src/" 2>/dev/null || true
    done
    echo
fi

# spec: context-kit/SPEC.md §The session-context hook — step 3 drift line (drift-kit owns the report; the seam is this optional line)
if [[ -n "$DRIFT_REPORT" && -f "$DRIFT_REPORT" ]]; then
    drift_line="$(bash "$DRIFT_REPORT" --trend 2>/dev/null)" || true
    if [[ -n "$drift_line" ]]; then
        echo "$drift_line  (full: bash $DRIFT_REPORT)"
        echo
    fi
fi

# spec: context-kit/SPEC.md §The session-context hook — step 4 stage-conditioned nudges; which stages get which nudge is consumer judgment [EDIT ME]
case "$stage" in
    scope | align | build)
        echo "Delegation is the primary token lever and is pre-authorized here: send"
        echo "read-heavy cross-SPEC audits and mechanical rename/merge sweeps to a sub-agent"
        echo "without waiting to be asked — this standing licence satisfies the Agent tool's"
        echo "ask-first default (see your delegation protocol)."
        echo
        ;;
esac

# spec: context-kit/SPEC.md §The session-context hook — step 5 memory-off backstop; check-memory-off fires at commit, this surfaces pollution between commits
MEM_DIRS="${CONTEXT_KIT_MEMORY_DIRS:-$HOME/.claude/projects/$(printf '%s' "$REPO_ROOT" | tr '/.' '-')/memory}"
for _md in $MEM_DIRS; do
    [[ -d "$_md" ]] || continue
    if find "$_md" -type f ! -name .gitkeep -print -quit 2>/dev/null | grep -q .; then
        echo "⚠ harness memory dir holds content ($_md) — durable facts belong in a tracked surface (context-kit/SPEC.md §The memory-off doctrine), not per-session memory."
        echo
        break
    fi
done

# spec: context-kit/SPEC.md §The session-context hook — step 6 scratch sweep
TMP_DIR="${GATE_SDK_TMP_DIR:-.tmp}"
if [[ -d "$TMP_DIR" ]]; then
    swept="$(find "$TMP_DIR" -mindepth 1 ! -name .gitkeep -mmin +1440 -depth -print -delete 2>/dev/null | wc -l | tr -d ' ')"
    if [[ "${swept:-0}" -gt 0 ]]; then
        echo "Tidied $swept stale scratch path(s) from $TMP_DIR/."
        echo
    fi
fi

# spec: context-kit/SPEC.md §The session-context hook — step 7 index-reminder footer; list your actual index commands [EDIT ME]
cat <<EOF
Before opening source for a task, run the matching surface index first
(index, then read the one you need):
  • bash $CTX_BIN/pub-index.sh <component>/src/    — Rust public API surface
  • bash $CTX_BIN/md-index.sh <file.md>            — large markdown / SPEC outline
  • bash $CTX_BIN/md-section.sh <file.md> "<head>" — extract one section by heading
EOF
echo "────────────────────────────────────────────────────────────────────────"
