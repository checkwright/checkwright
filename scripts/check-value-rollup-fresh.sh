#!/usr/bin/env bash
# graph: couples=docs/value.md,scripts/gates.list,scripts/*.sh,kit:*.sh,scripts/kpis.list,.claude/settings.json,.github/workflows/*.yml,CLAUDE.md,kit:templates/*.md,lifecycle-kit/templates/skills/*.md dir=one valve=none tier=precommit
# spec: CLAUDE.md §Housekeeping — the value-rollup block in docs/value.md is the byte-fresh projection of gen-value-rollup.sh --emit
#
# usage: check-value-rollup-fresh.sh [projection-file] [emit-file]
#   bare: compare the value-rollup block in docs/value.md against `gen-value-rollup.sh --emit`.
#   two args: compare the block extracted from projection-file to a pre-baked emit-file (hermetic fixture).
set -uo pipefail

SDK="${GATE_SDK_ROOT:-"${BASH_SOURCE[0]%/*}/../gate-sdk"}"
# shellcheck source=../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"

PROJECTION="${1:-docs/value.md}"
EMIT_SRC="${2:-}"
BEGIN="<!-- value-rollup:begin -->"
END="<!-- value-rollup:end -->"

[[ -f "$PROJECTION" ]] || { echo "check-value-rollup-fresh: projection not found: $PROJECTION" >&2; exit 2; }

block="$(awk -v b="$BEGIN" -v e="$END" '
    $0 == b { inb = 1; next }
    $0 == e { inb = 0; next }
    inb     { print }
' "$PROJECTION")"; st=$?
fail_closed "$st" check-value-rollup-fresh awk
[[ -n "$block" ]] || { echo "check-value-rollup-fresh: no value-rollup marker block in $PROJECTION" >&2; exit 2; }

if [[ -n "$EMIT_SRC" ]]; then
    [[ -f "$EMIT_SRC" ]] || { echo "check-value-rollup-fresh: emit source not found: $EMIT_SRC" >&2; exit 2; }
    emitted="$(cat "$EMIT_SRC")"; st=$?
    fail_closed "$st" check-value-rollup-fresh cat
else
    GEN="${BASH_SOURCE[0]%/*}/gen-value-rollup.sh"
    [[ -x "$GEN" ]] || { echo "check-value-rollup-fresh: generator not found: $GEN" >&2; exit 2; }
    emitted="$(bash "$GEN" --emit)"; st=$?
    fail_closed "$st" check-value-rollup-fresh gen-value-rollup
fi

if [[ "$block" != "$emitted" ]]; then
    echo "check-value-rollup-fresh: the value-rollup block in $PROJECTION is stale vs gen-value-rollup.sh:"
    diff <(printf '%s\n' "$emitted") <(printf '%s\n' "$block") | head -20 || true
    echo "  help: regenerate — bash scripts/gen-value-rollup.sh"
    exit 1
fi
echo "VALUE-ROLLUP-FRESH: clean (the value-rollup block in $PROJECTION byte-matches gen-value-rollup.sh)"
exit 0
