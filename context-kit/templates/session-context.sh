#!/usr/bin/env bash
# spec: context-kit/SPEC.md §The session-context hook (template) — consumer-copy SessionStart hook (bash-guard pattern); assembles the session brief, every step guarded, never fails a session
# no-port: gate-sdk/SPEC.md §The harness-template port disposition — the class ruling of 2026-08-30, reached by ground rather than by scope: this file carries an [EDIT ME] gap at every layout-judgment step (tool paths, the dirty-surface pre-run, the stage-conditioned nudges, the index footer), and context-kit/README.md tells an adopter to edit them as layout judgment rather than mechanism. The gaps ARE the extension point, so porting the file leaves an adopter nothing to fill. Structural, not a sizing judgment.

set -uo pipefail

cd "$(git rev-parse --show-toplevel 2>/dev/null || pwd)" 2>/dev/null || exit 0
REPO_ROOT="$(pwd -P)"

# spec: context-kit/SPEC.md §The session-context hook — consumer layout: vendored kit tools + governed queue file, retarget to yours [EDIT ME]. The queue index and the three index arms are reached through the battery runner's --emit front-end rather than by tool path: the front-end sources the shell library and supplies the bridged environment, so a consumer's section and cap overrides reach the arm (gate-sdk/SPEC.md §The non-gate arm).
RUN_GATES="gate-sdk/bin/run-gates.sh"
NATIVE_BIN="$(bash -c 'source gate-sdk/lib/gate.sh; gate_native_bin' 2>/dev/null)"
DRIFT_ARM="${CONTEXT_KIT_DRIFT_REPORT:-}"
STAGE_RULES="${CONTEXT_KIT_STAGE_RULES:-}"
STATE_FILE="${CONTEXT_KIT_STATE_FILE:-${GATE_SDK_WORKFLOW_DIR:-.workflow}/WORKFLOW-STATE.txt}"

echo "── Session context (context-kit session-context hook) ──────────────────"
echo

# spec: context-kit/SPEC.md §The session-context hook — step 1 queue index
# spec: context-kit/SPEC.md §The session-context hook — the stage cursor is the state file's last data line; read from a named file, never stdin, which the session-role signal below consumes exactly once. An absent file or one with no data line yields empty, which falls to the branch every non-close, non-scope stage already takes.
stage=""
if [[ -f "$STATE_FILE" ]]; then
    stage="$(awk '/^---[[:space:]]*$/ { f = 1; next } f && NF { l = $2 } END { print l }' "$STATE_FILE" 2>/dev/null)"
fi
if [[ -f "$RUN_GATES" ]]; then
    if [[ "$stage" == close || "$stage" == scope ]]; then
        bash "$RUN_GATES" --emit queue-index 2>/dev/null || echo "(queue-index unavailable)"
    else
        bash "$RUN_GATES" --emit queue-index --collapse-deferred 2>/dev/null || echo "(queue-index unavailable)"
    fi
    echo
fi

# spec: context-kit/SPEC.md §The session-context hook — step 2 dirty-surface pre-run; component detection + index command are layout assumptions [EDIT ME]
mapfile -t changed < <(
    git status --porcelain 2>/dev/null | awk '{ print $NF }' \
        | awk -F/ 'NF>1 { print $1 }' | sort -u \
        | while read -r d; do [[ -d "$d/src" ]] && echo "$d"; done
)
# spec: context-kit/SPEC.md §The session-context hook — the public-surface block guards on the gate binary, not on a script path: the index tools are arms of it now, and `exec_arm` exits 2 with a diagnostic this call site swallows, so a guard taken *after* the header would print the header and nothing under it on every host the artifact roster does not cover. Read the binary first and the block is absent rather than empty — the way the deleted `-f` guard degraded. The lookup runs in a subshell because the kit library exits 2 on a malformed config, and this hook never fails a session.
if [[ ${#changed[@]} -gt 0 && -n "$NATIVE_BIN" && -x "$NATIVE_BIN" ]]; then
    echo "Uncommitted changes touch: ${changed[*]}"
    echo "Public API surface of those components (pub-index — read the file for bodies):"
    echo
    for c in "${changed[@]}"; do
        bash "$RUN_GATES" --emit pub-index "$c/src/" 2>/dev/null || true
    done
    echo
fi

# spec: context-kit/SPEC.md §The session-context hook — step 3 drift line (drift-kit owns the report; the seam is this optional line). The knob names an **arm** of the battery runner's --emit front-end, not a script path: a `-f` test on an arm name is a test nothing can pass, so the guard is a non-empty name plus the front-end's own presence.
if [[ -n "$DRIFT_ARM" && -f "$RUN_GATES" ]]; then
    drift_line="$(bash "$RUN_GATES" --emit "$DRIFT_ARM" --trend 2>/dev/null)" || true
    if [[ -n "$drift_line" ]]; then
        echo "$drift_line  (full: bash $RUN_GATES --emit $DRIFT_ARM)"
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

# spec: context-kit/SPEC.md §The session-context hook — step 4 stage-conditioned nudges; which stages get which nudge is consumer judgment [EDIT ME]; suppressed for a lead (executor-facing)
if [[ "$role" != lead ]]; then
    case "$stage" in
        scope | align | build)
            echo "Delegation is the primary token lever and is pre-authorized here: send"
            echo "read-heavy cross-SPEC audits and mechanical rename/merge sweeps to a sub-agent"
            echo "without waiting to be asked — this standing licence satisfies the Agent tool's"
            echo "ask-first default (see your delegation protocol)."
            echo
            ;;
    esac
fi

# spec: context-kit/SPEC.md §The session-context hook — step 5 memory-off backstop; check-memory-off fires at commit, this surfaces pollution between commits
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
  • bash $RUN_GATES --emit pub-index <component>/src/    — public API surface (ships rust, ts)
  • bash $RUN_GATES --emit md-index <file.md>            — large markdown / SPEC outline
  • bash $RUN_GATES --emit md-section <file.md> "<head>" — extract one section by heading
EOF

# spec: context-kit/SPEC.md §The session-context hook — step 8 stage-routed craft-rule pointers; doctrine-kit owns the emitter, the seam is this optional block (drift-line precedent); suppressed for a lead (executor-facing)
if [[ "$role" != lead && -n "$stage" && -n "$STAGE_RULES" ]]; then
    rules_block="$(bash -c "$STAGE_RULES \"\$1\"" stage-rules "$stage" 2>/dev/null)" || true
    if [[ -n "$rules_block" ]]; then
        echo
        echo "Craft rules for the $stage stage — follow the doctrine link before the matching action:"
        echo "$rules_block"
    fi
fi

# spec: context-kit/SPEC.md §The session-context hook — step 9 env profile; consumer-local machine profile re-probed then emitted verbatim when present (env-profile seam, drift-line precedent)
ENV_PROFILE_FILE="${CONTEXT_KIT_ENV_PROFILE_FILE:-ENV.local.md}"
if [[ -f "$ENV_PROFILE_FILE" ]]; then
    # spec: context-kit/SPEC.md §The session-context hook — per-session auto-refresh: re-probe inside the file-present guard (never auto-seeds), output suppressed; reached through the --emit front-end like every other arm, so retarget $RUN_GATES to your layout [EDIT ME]
    [[ -f "$RUN_GATES" ]] && bash "$RUN_GATES" --emit env-probe >/dev/null 2>&1 || true
    echo
    echo "Local env profile ($ENV_PROFILE_FILE) — adapt commands to this box:"
    cat "$ENV_PROFILE_FILE" 2>/dev/null || true
fi
echo "────────────────────────────────────────────────────────────────────────"
