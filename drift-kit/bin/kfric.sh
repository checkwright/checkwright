#!/usr/bin/env bash
# spec: drift-kit/SPEC.md §The knowledge-friction loop — the capture affordance; stamps the grammar, no caller-side redirect
# usage: kfric.sh "<fact re-derived>" "<surface it was read from>"   (both required, non-empty)
#   appends one line '<date> <fact> ← <surface>' to the knowledge-friction log; exit 2 on misuse
set -uo pipefail

cd "$(git rev-parse --show-toplevel 2>/dev/null || pwd)" 2>/dev/null || exit 1

# spec: drift-kit/SPEC.md §Layout and configuration
_ds_cfg="${DRIFT_KIT_CONFIG_FILE:-}"
if [[ -n "$_ds_cfg" ]]; then
    [[ -f "$_ds_cfg" ]] || {
        echo "drift-kit: DRIFT_KIT_CONFIG_FILE not found: $_ds_cfg" >&2
        exit 2
    }
    # shellcheck source=/dev/null  # consumer config path is resolved at runtime
    source "$_ds_cfg"
else
    _ds_cfg="${GATE_SDK_GATES_DIR:-scripts}/drift-config.sh"
    if [[ -f "$_ds_cfg" ]]; then
        # shellcheck source=/dev/null  # consumer config path is resolved at runtime
        source "$_ds_cfg"
    fi
fi
unset _ds_cfg

: "${DRIFT_KIT_KNOWLEDGE_LOG:=${GATE_SDK_WORKFLOW_DIR:-.workflow}/knowledge-friction.log}"

usage() {
    printf 'usage: %s [-h|--help] [--] "<fact re-derived>" "<surface it was read from>"\n' "$(basename "$0")"
    printf '  appends one dated line to %s; "--" files a field beginning with "-"\n' \
        "$DRIFT_KIT_KNOWLEDGE_LOG"
}

# spec: gate-sdk/SPEC.md §The bin/-tool contract — free-text positionals validate shape, not only arity: the refusal scans every positional, since arity alone leaves a flag safe in no slot but the first
if [[ "${1:-}" == "-h" || "${1:-}" == "--help" ]]; then
    usage
    exit 0
fi
if [[ "${1:-}" == "--" ]]; then
    shift
else
    for _kf_arg in "$@"; do
        [[ "$_kf_arg" == -* ]] || continue
        printf '%s: unrecognized option: %s — a field beginning with "-" is filed after a "--" separator\n' \
            "$(basename "$0")" "$_kf_arg" >&2
        usage >&2
        exit 2
    done
fi

if [[ $# -ne 2 || -z "$1" || -z "$2" ]]; then
    usage >&2
    exit 2
fi

mkdir -p "$(dirname "$DRIFT_KIT_KNOWLEDGE_LOG")" 2>/dev/null || true
line="$(date +%F) $1 ← $2"
printf '%s\n' "$line" >>"$DRIFT_KIT_KNOWLEDGE_LOG"
printf 'kfric: %s\n' "$line"
