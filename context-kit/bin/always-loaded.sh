#!/usr/bin/env bash
# spec: context-kit/SPEC.md §The always-loaded meter — measures the standing
# per-session surface (configured surfaces + steady-state hook body) against a
# committed baseline. Never runs the session-context hook itself: that hook
# emits this meter's own line, so self-measurement would recurse.
#
#   always-loaded.sh                     one line: total, per-part split, delta vs baseline
#   always-loaded.sh --update-baseline   rewrite the baseline (a close-stage act)
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$REPO_ROOT" 2>/dev/null || { echo "always-loaded: cannot enter repo root" >&2; exit 2; }

UPDATE=0
[[ "${1:-}" == "--update-baseline" ]] && UPDATE=1

_ck_cfg="${CONTEXT_KIT_CONFIG_FILE:-${GATE_SDK_GATES_DIR:-scripts}/context-config.sh}"
if [[ -f "$_ck_cfg" ]]; then
    # shellcheck source=/dev/null  # consumer config path is resolved at runtime
    source "$_ck_cfg"
fi
unset _ck_cfg

declare -p CONTEXT_KIT_SURFACES >/dev/null 2>&1 || CONTEXT_KIT_SURFACES=("CLAUDE.md")
: "${CONTEXT_KIT_BASELINE_FILE:=${GATE_SDK_WORKFLOW_DIR:-.workflow}/always-loaded-baseline.txt}"

# spec: context-kit/SPEC.md §Layout and configuration — CONTEXT_KIT_HOOK_CMD
# default: queue-kit's collapsed queue index when the script resolves (consumer
# gates dir first, then a sibling queue-kit/bin); else empty. Unset means
# "derive"; set-but-empty is honored (surfaces only).
if [[ -z "${CONTEXT_KIT_HOOK_CMD+x}" ]]; then
    CONTEXT_KIT_HOOK_CMD=""
    for _qi in "${GATE_SDK_GATES_DIR:-scripts}/queue-index.sh" "$KIT/../queue-kit/bin/queue-index.sh"; do
        if [[ -f "$_qi" ]]; then
            CONTEXT_KIT_HOOK_CMD="bash $_qi --collapse-deferred"
            break
        fi
    done
    unset _qi
fi

surface=0
for f in "${CONTEXT_KIT_SURFACES[@]}"; do
    [[ -f "$f" ]] || continue
    n="$(wc -l < "$f" 2>/dev/null | tr -d ' ')"
    [[ "$n" =~ ^[0-9]+$ ]] && surface=$(( surface + n ))
done

hook=0
if [[ -n "$CONTEXT_KIT_HOOK_CMD" ]]; then
    hook_out="$(bash -c "$CONTEXT_KIT_HOOK_CMD" 2>/dev/null || true)"
    if [[ -n "$hook_out" ]]; then
        hook="$(printf '%s\n' "$hook_out" | wc -l | tr -d ' ')"
        [[ "$hook" =~ ^[0-9]+$ ]] || hook=0
    fi
fi

total=$(( surface + hook ))

# spec: context-kit/SPEC.md §The always-loaded meter — baseline data line is
# '<total> <surface> <commit> [extra…]'; surface is re-measured not read back,
# and extra trailing fields are preserved on update.
base_total=""; base_commit=""; base_extra=""
if [[ -f "$CONTEXT_KIT_BASELINE_FILE" ]]; then
    read -r base_total _ base_commit base_extra < <(
        grep -vE '^[[:space:]]*(#|$)' "$CONTEXT_KIT_BASELINE_FILE" 2>/dev/null | head -1)
fi

if [[ "$UPDATE" -eq 1 ]]; then
    commit="$(git rev-parse HEAD 2>/dev/null || echo unknown)"
    line="$total $surface $commit"
    [[ -n "$base_extra" ]] && line="$line $base_extra"
    {
        echo "# contract: context-kit/SPEC.md §The always-loaded meter"
        echo "$line"
    } > "$CONTEXT_KIT_BASELINE_FILE"
    echo "always-loaded baseline updated: ${total}l (surfaces $surface · hook $hook) @ ${commit:0:8}"
    exit 0
fi

line="always-loaded: ${total}l (surfaces $surface · hook $hook)"
if [[ "$base_total" =~ ^[0-9]+$ ]]; then
    delta=$(( total - base_total ))
    line="$line  $(printf '%+d' "$delta") since ${base_commit:0:8}"
fi
echo "$line"
exit 0
