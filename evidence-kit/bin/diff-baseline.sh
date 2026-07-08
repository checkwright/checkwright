#!/usr/bin/env bash
# spec: evidence-kit/SPEC.md §bin/diff-baseline.sh — the situational runtime diff (not a precommit gate): parse the captured logs passed as arguments, diff each against the baseline's suite slice per-scenario, print findings. Reads the skip side-channel to demote self-skipped scenarios from pass.
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/evidence.sh
source "$KIT/lib/evidence.sh"

if [[ $# -lt 2 || $(($# % 2)) -ne 0 ]]; then
    echo "usage: diff-baseline.sh <suite> <logfile> [<suite> <logfile>...]" >&2
    exit 2
fi

mkdir -p "$EVIDENCE_KIT_TMP_DIR"
recoveries=0
rc=0
while [[ $# -gt 0 ]]; do
    suite="$1"; log="$2"; shift 2
    [[ -f "$log" ]] || { echo "diff-baseline: log not found: $log" >&2; exit 2; }
    parsed="$EVIDENCE_KIT_TMP_DIR/diff-$suite.parsed"
    ek_parse "$EVIDENCE_KIT_PARSER" "$suite" "$log" 0 >"$parsed"
    out="$(ek_diff "$EVIDENCE_KIT_BASELINE_FILE" "$suite" "$parsed" "$EVIDENCE_KIT_SKIP_FILE")" || rc=1
    if [[ -n "$out" ]]; then
        printf '%s\n' "$out"
        recoveries=$((recoveries + $(grep -c '^recovery ' <<<"$out" || true)))
    fi
done

if [[ "$rc" -ne 0 ]]; then
    echo "diff-baseline: NEW failures against $EVIDENCE_KIT_BASELINE_FILE (see 'new-failure' lines above)"
    exit 1
fi
echo "diff-baseline: clean ($recoveries unpromoted recovery finding(s); no new failure vs $EVIDENCE_KIT_BASELINE_FILE)"
exit 0
