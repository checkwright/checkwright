#!/usr/bin/env bash
# spec: context-kit/SPEC.md §The session-context hook (template) — consumer copy assembling the per-session brief; every step guarded, never fails a session
# no-port: CLAUDE.md §The provenance seam (never cross it) — the class ruling of 2026-08-30 at gate-sdk/SPEC.md §The harness-template port disposition, reached by ground rather than by scope: this copy is the filled instance of its template's [EDIT ME] gaps, so every value it holds beyond the template is this repo's own layout judgment. It also takes its template's disposition under gate-sdk/SPEC.md §check-template-copy-parity's bidirectional parity, so the ground here stands whatever becomes of the template's. Structural, not a sizing judgment.

set -uo pipefail

cd "$(git rev-parse --show-toplevel 2>/dev/null || pwd)" 2>/dev/null || exit 0
REPO_ROOT="$(pwd -P)"

NATIVE_BIN=""                                     # the gate binary: the queue surface, the index arms and the verdict
# shellcheck source=/dev/null
[[ -f gate-sdk/lib/gate.sh ]] && source gate-sdk/lib/gate.sh
declare -F gate_native_bin_spelled >/dev/null && NATIVE_BIN="$(gate_native_bin_spelled)"
DRIFT_ARM="${CONTEXT_KIT_DRIFT_REPORT:-drift-report}"  # drift-kit trend line: an --emit arm name
STAGE_RULES="${CONTEXT_KIT_STAGE_RULES:-bash gate-sdk/bin/run-gates.sh --emit stage-rules}"  # doctrine-kit craft-rule router: a command, not a path
STATE_FILE="${CONTEXT_KIT_STATE_FILE:-${GATE_SDK_WORKFLOW_DIR:-.workflow}/WORKFLOW-STATE.txt}"  # lifecycle stage cursor

echo "── Session context (context-kit session-context hook) ──────────────────"
echo

# spec: context-kit/SPEC.md §The session-context hook — the stage cursor is the state file's last data line; read from a named file, never stdin, which the session-role signal below consumes exactly once. An absent file or one with no data line yields empty, which falls to the existing non-close/non-scope branch.
stage=""
if [[ -f "$STATE_FILE" ]]; then
    marker=""
    while IFS= read -r line || [[ -n "$line" ]]; do
        if [[ "$line" =~ ^---[[:space:]]*$ ]]; then marker=1; stage=""; continue; fi
        read -r f1 f2 _ <<< "$line"
        [[ -n "$marker" && -n "${f1:-}" ]] && stage="${f2:-}"
    done 2>/dev/null < "$STATE_FILE"
fi
if [[ -x "$NATIVE_BIN" ]]; then
    if [[ "$stage" == close || "$stage" == scope ]]; then
        "$NATIVE_BIN" --emit queue-index 2>/dev/null || echo "(queue-index unavailable)"
    else
        "$NATIVE_BIN" --emit queue-index --collapse-deferred 2>/dev/null || echo "(queue-index unavailable)"
    fi
    echo
fi

mapfile -t changed < <(
    git status --porcelain 2>/dev/null \
        | while read -r l; do p="${l##* }"; [[ "$p" == */* && -d "${p%%/*}/src" ]] && echo "${p%%/*}"; done \
        | sort -u
)
# spec: context-kit/SPEC.md §The session-context hook — the public-surface block guards on the gate binary: the index tools are arms of it, and a missing binary would print the header and nothing under it on every host the artifact roster does not cover. Read the binary first and the block is absent rather than empty — the way the deleted `-f` guard degraded.
if [[ ${#changed[@]} -gt 0 && -x "$NATIVE_BIN" ]]; then
    echo "Uncommitted changes touch: ${changed[*]}"
    echo "Public API surface of those components (pub-index — read the file for bodies):"
    echo
    for c in "${changed[@]}"; do
        "$NATIVE_BIN" --emit pub-index "$c/src/" 2>/dev/null || true
    done
    echo
fi

# spec: context-kit/SPEC.md §The session-context hook — the knob names an arm of the gate binary, not a script path; a `-f` test on an arm name passes for nothing, which is how this line would have vanished with no red anywhere.
if [[ -n "$DRIFT_ARM" && -x "$NATIVE_BIN" ]]; then
    drift_line="$("$NATIVE_BIN" --emit "$DRIFT_ARM" --trend 2>/dev/null)" || true
    if [[ -n "$drift_line" ]]; then
        echo "$drift_line  (full: $NATIVE_BIN --emit $DRIFT_ARM)"
        echo
    fi
fi

# spec: delegation-kit/SPEC.md §usage-verdict — the verdict is an arm, so the brief dispatches
# it to the binary rather than testing for a path the port deleted
if [[ -x "$NATIVE_BIN" ]]; then
    budget_line="$("$NATIVE_BIN" --usage-verdict 2>/dev/null)" || true
    if [[ -n "$budget_line" ]]; then
        echo "Budget (enforced per-dispatch by the Agent budget guard): $budget_line"
        echo
    fi
fi

# spec: context-kit/SPEC.md §The session-context hook — session-role signal: lead iff the marker's id matches this fire's payload session id (the payload, never CLAUDE_CODE_SESSION_ID — a subagent inherits its parent's); guarded, signal absent = byte-identical output
role=""
ROLE_FILE="${CONTEXT_KIT_SESSION_ROLE_FILE:-${GATE_SDK_TMP_DIR:-.tmp}/session-role}"
if [[ -f "$ROLE_FILE" && ! -t 0 ]]; then
    payload="$(cat 2>/dev/null)" || true
    hook_sid="$(grep -oE '"session_id"[[:space:]]*:[[:space:]]*"[^"]*"' <<<"${payload:-}" 2>/dev/null | head -1 | sed -E 's/.*"([^"]*)"$/\1/')" || true
    read -r m_role m_sid _ < "$ROLE_FILE" 2>/dev/null || true
    if [[ "${m_role:-}" == "lead" && -n "${m_sid:-}" && "${hook_sid:0:8}" == "${m_sid:-}" ]]; then
        role=lead
    fi
fi

# spec: context-kit/SPEC.md §The session-context hook — step 4 suppressed for a lead (executor-facing)
if [[ "$role" != lead ]]; then
    case "$stage" in
        scope | align | build)
            echo "Delegation is the primary token lever and is pre-authorized here: send"
            echo "read-heavy cross-SPEC audits and mechanical rename/merge sweeps to a sub-agent"
            echo "without waiting to be asked — this standing licence satisfies the Agent tool's"
            echo "ask-first default (/agent-execution is the full protocol)."
            echo
            ;;
    esac
fi

# spec: gate-sdk/SPEC.md §The path-dialect contract — this fold compares against a harness-owned directory name, the one place the per-substrate dialects would meet; which spelling the harness uses on Windows is not decidable from this tree, so the producer above is normalized like any other and this open question is recorded rather than guessed (gap filed)
MEM_DIRS="${CONTEXT_KIT_MEMORY_DIRS:-$HOME/.claude/projects/$(printf '%s' "$REPO_ROOT" | tr '/.' '-')/memory}"
for _md in $MEM_DIRS; do
    [[ -d "$_md" ]] || continue
    if find "$_md" -type f ! -name .gitkeep -print -quit 2>/dev/null | grep -q .; then
        echo "⚠ harness memory dir holds content ($_md) — durable facts belong in a tracked surface (context-kit/SPEC.md §The memory-off doctrine), not per-session memory."
        echo
        break
    fi
done

TMP_DIR="${GATE_SDK_TMP_DIR:-.tmp}"
if [[ -d "$TMP_DIR" ]]; then
    swept="$(find "$TMP_DIR" -mindepth 1 ! -name .gitkeep -mmin +1440 -depth -print -delete 2>/dev/null | wc -l | tr -d ' ')"
    if [[ "${swept:-0}" -gt 0 ]]; then
        echo "Tidied $swept stale scratch path(s) from $TMP_DIR/."
        echo
    fi
fi

cat <<EOF
Before opening source for a task, run the matching surface index first
(index, then read the one you need):
  • $NATIVE_BIN --emit pub-index <component>/src/    — public API surface (ships rust, ts)
  • $NATIVE_BIN --emit md-index <file.md>            — large markdown / SPEC outline
  • $NATIVE_BIN --emit md-section <file.md> "<head>" — extract one section by heading
EOF

# spec: context-kit/SPEC.md §The session-context hook — step 8 suppressed for a lead (executor-facing)
if [[ "$role" != lead && -n "$stage" && -n "$STAGE_RULES" ]]; then
    rules_rc=0
    rules_block="$(bash -c "$STAGE_RULES \"\$1\"" stage-rules "$stage" 2>/dev/null)" || rules_rc=$?
    # spec: context-kit/SPEC.md §The session-context hook — step 8 reports a configured command that fails rather than dropping the block silently
    if [[ "$rules_rc" -ne 0 ]]; then
        echo
        echo "⚠ craft rules unavailable: the CONTEXT_KIT_STAGE_RULES command exited $rules_rc — fix the knob (context-kit/SPEC.md §Layout and configuration)."
    elif [[ -n "$rules_block" ]]; then
        echo
        echo "Craft rules for the $stage stage — follow the doctrine link before the matching action:"
        echo "$rules_block"
    fi
fi

ENV_PROFILE_FILE="${CONTEXT_KIT_ENV_PROFILE_FILE:-ENV.local.md}"   # context-kit env-probe profile
if [[ -f "$ENV_PROFILE_FILE" ]]; then
    # spec: context-kit/SPEC.md §The session-context hook — step 9 per-session auto-refresh: re-probe before emitting, inside the file-present guard (never auto-seeds), output suppressed so it never pollutes the brief
    [[ -x "$NATIVE_BIN" ]] && "$NATIVE_BIN" --emit env-probe >/dev/null 2>&1 || true
    echo
    echo "Local env profile ($ENV_PROFILE_FILE) — adapt commands to this box:"
    cat "$ENV_PROFILE_FILE" 2>/dev/null || true
fi
echo "────────────────────────────────────────────────────────────────────────"
