#!/usr/bin/env bash
# spec: drift-kit/SPEC.md §The knowledge-friction loop — the capture affordance; stamps the grammar, no caller-side redirect
# usage: kfric.sh "<fact re-derived>" "<surface it was read from>"   (both required, non-empty)
#   appends one line '<date> <fact> ← <surface>' to the knowledge-friction log; exit 2 on misuse
set -uo pipefail

REPO_ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
cd "$REPO_ROOT" 2>/dev/null || exit 1

# spec: drift-kit/SPEC.md §Layout and configuration
_ds_cfg="${DRIFT_KIT_CONFIG_FILE:-${GATE_SDK_GATES_DIR:-scripts}/drift-config.sh}"
if [[ -f "$_ds_cfg" ]]; then
    # shellcheck source=/dev/null  # consumer config path is resolved at runtime
    source "$_ds_cfg"
fi
unset _ds_cfg

: "${DRIFT_KIT_KNOWLEDGE_LOG:=${GATE_SDK_WORKFLOW_DIR:-.workflow}/knowledge-friction.log}"

usage() {
    printf 'usage: %s "<fact re-derived>" "<surface it was read from>"\n' "$(basename "$0")" >&2
}

if [[ $# -ne 2 || -z "$1" || -z "$2" ]]; then
    usage
    exit 2
fi

mkdir -p "$(dirname "$DRIFT_KIT_KNOWLEDGE_LOG")" 2>/dev/null || true
line="$(date +%F) $1 ← $2"
printf '%s\n' "$line" >>"$DRIFT_KIT_KNOWLEDGE_LOG"
printf 'kfric: %s\n' "$line"
