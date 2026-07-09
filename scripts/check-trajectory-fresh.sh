#!/usr/bin/env bash
# graph: couples=docs/evidence-data.md,.workflow/WORKFLOW-STATE.txt,.workflow/validate-evidence.txt,scripts/gates.list dir=one valve=none tier=precommit
# spec: drift-kit/SPEC.md §The published-evidence extractor — docs/evidence-data.md is the byte-fresh projection of trajectory.sh --emit
#
# usage: check-trajectory-fresh.sh [projection-file] [emit-file]
#   bare: compare docs/evidence-data.md against `trajectory.sh --emit`.
#   two args: compare projection-file to a pre-baked emit-file (hermetic fixture).
set -uo pipefail

SDK="${GATE_SDK_ROOT:-"${BASH_SOURCE[0]%/*}/../gate-sdk"}"
# shellcheck source=../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"

PROJECTION="${1:-docs/evidence-data.md}"
EMIT_SRC="${2:-}"

[[ -f "$PROJECTION" ]] || { echo "check-trajectory-fresh: projection not found: $PROJECTION" >&2; exit 2; }

if [[ -n "$EMIT_SRC" ]]; then
    [[ -f "$EMIT_SRC" ]] || { echo "check-trajectory-fresh: emit source not found: $EMIT_SRC" >&2; exit 2; }
    emitted="$(cat "$EMIT_SRC")"; st=$?
    fail_closed "$st" check-trajectory-fresh cat
else
    TRAJ="${BASH_SOURCE[0]%/*}/../drift-kit/bin/trajectory.sh"
    [[ -x "$TRAJ" ]] || { echo "check-trajectory-fresh: extractor not found: $TRAJ" >&2; exit 2; }
    emitted="$(bash "$TRAJ" --emit)"; st=$?
    fail_closed "$st" check-trajectory-fresh trajectory
fi

if [[ "$emitted" != "$(cat "$PROJECTION")" ]]; then
    echo "check-trajectory-fresh: $PROJECTION is stale vs the trajectory extractor:"
    diff <(printf '%s\n' "$emitted") "$PROJECTION" | head -20 || true
    echo "  help: regenerate — bash drift-kit/bin/trajectory.sh --emit > docs/evidence-data.md"
    exit 1
fi
echo "TRAJECTORY-FRESH: clean ($PROJECTION byte-matches the trajectory extractor)"
exit 0
