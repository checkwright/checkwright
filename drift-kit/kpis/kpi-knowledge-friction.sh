#!/usr/bin/env bash
# spec: drift-kit/SPEC.md §The knowledge-friction loop — kpi-knowledge-friction: re-derivations logged this iteration
#
# Lag by construction: only what a session noticed and logged is counted, so the
# count lower-bounds the real re-derivation rate. Reads the knowledge-friction
# log (gitignored per-iteration scratch, cleared at close-stage triage); degrades
# to n/a when the log is absent (the surface the capture convention feeds).
set -uo pipefail

LOG="${DRIFT_KIT_KNOWLEDGE_LOG:-${GATE_SDK_WORKFLOW_DIR:-.workflow}/knowledge-friction.log}"

if [[ ! -f "$LOG" ]]; then
    [[ "${1:-}" == "--trend" ]] && exit 0
    printf 'lag\tknowledge friction\tn/a (no knowledge-friction log)\n'
    exit 0
fi

count="$(grep -cE '[^[:space:]]' "$LOG")" || count=0
[[ "$count" =~ ^[0-9]+$ ]] || count=0

if [[ "${1:-}" == "--trend" ]]; then
    printf 'kfric %d\n' "$count"
    exit 0
fi
printf 'lag\tknowledge friction\t%d re-derivation(s) logged this iteration (lower bound)\n' "$count"
exit 0
