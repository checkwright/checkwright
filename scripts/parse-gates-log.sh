#!/usr/bin/env bash
# spec: evidence-kit/SPEC.md §Layout and configuration — this repo's EVIDENCE_KIT_PARSER_gates adapter: the verbose run-gates log maps to one scenario per registered gate, so an existing gate turning red diffs as a new failure even while a sibling is legitimately held red
set -uo pipefail

LOG="${1:-}"
[[ -n "$LOG" && -f "$LOG" ]] || {
    echo "parse-gates-log: log not found: $LOG" >&2
    exit 2
}

# comment-tier-exempt: run-gates prints these tails only on failure or under GATE_SDK_VERBOSE, so EVIDENCE_KIT_RUN_gates sets it; a log with no tails yields no output, which run-validate's produced-no-result guard reads as the run failure it is
awk '
    /^  PASS: / { print $2, "pass" }
    /^  FAIL: / { print $2, "fail" }
' "$LOG"
