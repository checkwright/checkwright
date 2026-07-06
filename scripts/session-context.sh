#!/usr/bin/env bash
# context-kit session-context hook (consumer copy — context-kit/SPEC.md §The
# session-context hook). Wired as the harness SessionStart hook via
# templates/settings-sessionstart.json; it assembles the session brief. Every
# step is guarded and degrades silently — the hook never fails a session. This
# is a consumer copy (the bash-guard.sh pattern): edit the [EDIT ME] sections,
# which are layout judgment, not mechanism.

set -uo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$REPO_ROOT" 2>/dev/null || exit 0

# ── consumer layout ─────────────────────────────────────────────  [EDIT ME]
# Where the vendored kit tools live, plus the governed queue file. Retarget to
# your layout (these defaults are the checkwright monorepo's).
QUEUE_INDEX="queue-kit/bin/queue-index.sh"       # queue-kit's queue surface
CTX_BIN="context-kit/bin"                         # context-kit index tools
QUEUE_FILE="${GATE_SDK_QUEUE_FILE:-TASK-QUEUE.md}"
DRIFT_REPORT="${CONTEXT_KIT_DRIFT_REPORT:-}"      # e.g. drift-kit/bin/drift-report.sh
# ────────────────────────────────────────────────────────────────────────────

echo "── Session context (context-kit session-context hook) ──────────────────"
echo

# 1. Queue index — collapse Deferred except on the scope stage. Deferred is
#    unpickable and only scope (promotion) acts on it, so its full listing every
#    other session is pure recurring cost.
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
    if [[ "$stage" == scope ]]; then
        bash "$QUEUE_INDEX" 2>/dev/null || echo "(queue-index unavailable)"
    else
        bash "$QUEUE_INDEX" --collapse-deferred 2>/dev/null || echo "(queue-index unavailable)"
    fi
    echo
fi

# 2. Dirty-surface pre-run ───────────────────────────────────────  [EDIT ME]
#    For each component with uncommitted changes, pre-run its surface index so a
#    resumed session's editing surface is already in context. Default: pub-index
#    over top-level dirs that contain src/. Component detection and the index
#    command are layout assumptions — retarget them to your tree.
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
# ────────────────────────────────────────────────────────────────────────────

# 3. Drift line — one trend summary when a drift report exists (drift-kit is a
#    later extraction; this optional line is the seam). Silently absent otherwise.
if [[ -n "$DRIFT_REPORT" && -f "$DRIFT_REPORT" ]]; then
    drift_line="$(bash "$DRIFT_REPORT" --trend 2>/dev/null)" || true
    if [[ -n "$drift_line" ]]; then
        echo "$drift_line  (full: bash $DRIFT_REPORT)"
        echo
    fi
fi

# 4. Stage-conditioned nudges ────────────────────────────────────  [EDIT ME]
#    Short reminders keyed on the current stage. Which stages get which nudge is
#    consumer judgment; the platform's delegation nudge is the exemplar.
case "$stage" in
    align | build)
        echo "Delegation is the primary token lever — read-heavy cross-SPEC audits and"
        echo "mechanical rename/merge sweeps go to a sub-agent (delegation-kit/templates/"
        echo "agent-execution.md is the protocol)."
        echo
        ;;
esac
# ────────────────────────────────────────────────────────────────────────────

# 5. Scratch sweep — reclaim scratch older than a day, depth-first so stray
#    directories go too, never touching .gitkeep. Age-guarded so a concurrent
#    same-checkout session's in-flight scratch survives.
TMP_DIR="${GATE_SDK_TMP_DIR:-.tmp}"
if [[ -d "$TMP_DIR" ]]; then
    swept="$(find "$TMP_DIR" -mindepth 1 ! -name .gitkeep -mmin +1440 -depth -print -delete 2>/dev/null | wc -l | tr -d ' ')"
    if [[ "${swept:-0}" -gt 0 ]]; then
        echo "Tidied $swept stale scratch path(s) from $TMP_DIR/."
        echo
    fi
fi

# 6. Index-reminder footer ───────────────────────────────────────  [EDIT ME]
#    The "index first" ritual with your actual index commands listed.
cat <<EOF
Before opening source for a task, run the matching surface index first
(index, then read the one you need):
  • bash $CTX_BIN/pub-index.sh <component>/src/    — Rust public API surface
  • bash $CTX_BIN/md-index.sh <file.md>            — large markdown / SPEC outline
  • bash $CTX_BIN/md-section.sh <file.md> "<head>" — extract one section by heading
EOF
echo "────────────────────────────────────────────────────────────────────────"
