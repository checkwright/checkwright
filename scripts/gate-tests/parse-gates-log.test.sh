#!/usr/bin/env bash
# Behavioral test of scripts/parse-gates-log.sh — the EVIDENCE_KIT_PARSER_gates
# adapter. Not a gate (it emits scenario lines, not a clean/violation verdict),
# so it carries a test rather than a fixture pair.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
PARSER="$ROOT/scripts/parse-gates-log.sh"

fails=0
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

# A — a verbose log maps to one line per gate, off the tails run-gates emits
#     under GATE_SDK_VERBOSE. Both FAIL tails (exit-N and unresolved) classify.
cat >"$tmp/verbose.log" <<'EOF'

===== check-alpha =====
  PASS: check-alpha

===== check-beta =====
BETA: 1 violation(s):
  help: fix it
  FAIL: check-beta (exit 1)

===== check-gamma =====
check-gamma listed in scripts/gates.list but resolves in none of: scripts
  FAIL: check-gamma (unresolved)

===== gates summary =====
2 of 3 gates FAILED: check-beta check-gamma
EOF
out="$(bash "$PARSER" "$tmp/verbose.log")"
want="check-alpha pass
check-beta fail
check-gamma fail"
if [[ "$out" != "$want" ]]; then
    echo "  FAIL: verbose log did not map to per-gate lines: $out"; fails=$((fails + 1))
fi

# B — an early-crashed battery (no per-gate tails) yields no output, which
#     run-validate's produced-no-result guard turns into a run failure. The
#     fail-closed reading: no scenarios is a broken run, not an empty diff.
printf 'run-gates: gates.list not found\n' >"$tmp/crash.log"
out="$(bash "$PARSER" "$tmp/crash.log")"
if [[ -n "$out" ]]; then
    echo "  FAIL: a log with no per-gate tails must yield no output, got: $out"; fails=$((fails + 1))
fi

# C — a missing log is a hard error, never a silent empty parse.
if bash "$PARSER" "$tmp/nope.log" >/dev/null 2>&1; then
    echo "  FAIL: a missing log did not fail closed"; fails=$((fails + 1))
fi

if [[ "$fails" -gt 0 ]]; then
    echo "parse-gates-log.test: $fails assertion(s) failed"
    exit 1
fi
echo "parse-gates-log.test: ok (verbose log -> per-gate lines incl. both FAIL tails; no-tail log -> no output; missing log fails closed)"
exit 0
