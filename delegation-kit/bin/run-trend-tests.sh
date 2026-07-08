#!/usr/bin/env bash
# spec: delegation-kit/SPEC.md §Testing — assertion runner for usage-trend over a fixture history
#
#   usage: run-trend-tests.sh [history-fixture]
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
TREND="$KIT/bin/usage-trend.sh"
HIST="${1:-$KIT/usage-tests/trend-history.log}"

[[ -x "$TREND" ]] || { echo "run-trend-tests: missing or non-executable $TREND" >&2; exit 2; }
[[ -f "$HIST" ]]  || { echo "run-trend-tests: history fixture not found: $HIST" >&2; exit 2; }

SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0
assert_has() {  # <label> <needle> <<< haystack
    local label="$1" needle="$2" hay; hay="$(cat)"
    grep -qF -- "$needle" <<<"$hay" || { echo "  FAIL [$label]: output missing '$needle'"; fails=$((fails + 1)); }
}
assert_count() {  # <label> <pattern> <want> <<< haystack
    local label="$1" pat="$2" want="$3" got; got="$(grep -cE -- "$pat" || true)"
    [[ "$got" -eq "$want" ]] || { echo "  FAIL [$label]: want $want match(es) of /$pat/, got $got"; fails=$((fails + 1)); }
}

# spec: delegation-kit/SPEC.md §Testing — hermetic sandbox run, fixture as the positional arg
out="$( cd "$SANDBOX" && bash "$TREND" "$HIST" )"; rc=$?
[[ "$rc" -eq 0 ]] || { echo "  FAIL [exit]: want 0, got $rc"; fails=$((fails + 1)); }

assert_has   "per-account grouping: acctA heads its own block" "account acctA" <<<"$out"
assert_has   "per-account grouping: acctB heads its own block" "account acctB" <<<"$out"

assert_count "5h segments: 3 acctA reset-boundary windows + 2 acctB login-split" '^  \[5h\]' 5 <<<"$out"
assert_count "7d segments: 1 reunited acctA week + 2 acctB login-split"          '^  \[7d\]' 3 <<<"$out"

assert_has   "spike-then-correction excluded, not averaged: 90->40 pair dropped, ends stay 10->25" "reset@20000 tier=pro: 10.0%->25.0%" <<<"$out"
assert_has   "spike-then-correction: the pair is flagged suspect" "2 suspect" <<<"$out"

assert_has   "weekly reunion across the switch-back: one acctA week spanning" "[7d] reset@600000 tier=pro: 30.0%->70.0%" <<<"$out"
assert_has   "weekly reunion: all 7 acctA samples in one segment" "7 sample(s)" <<<"$out"

assert_has   "token delta on the weekly report" "tokens: +1200 in / +410 out" <<<"$out"
assert_has   "weekly headroom on the report" "weekly headroom:" <<<"$out"

assert_has   "PAUSE onset annotated where the pause first landed" "first PAUSE onset at epoch 33600" <<<"$out"

( cd "$SANDBOX" && bash "$TREND" ) >/dev/null 2>&1 && { echo "  FAIL [unset knob]: want exit 2"; fails=$((fails + 1)); }
rc=$?; [[ "$rc" -eq 2 ]] || { echo "  FAIL [unset knob]: want exit 2, got $rc"; fails=$((fails + 1)); }
( cd "$SANDBOX" && bash "$TREND" "$SANDBOX/nope.log" ) >/dev/null 2>&1 && { echo "  FAIL [missing history]: want exit 2"; fails=$((fails + 1)); }
rc=$?; [[ "$rc" -eq 2 ]] || { echo "  FAIL [missing history]: want exit 2, got $rc"; fails=$((fails + 1)); }

if [[ "$fails" -gt 0 ]]; then
    echo "run-trend-tests: $fails assertion(s) failed"
    exit 1
fi
echo "run-trend-tests: ok (segmentation across reset/login/account boundaries, weekly reunion, spike flagging, and fail-closed knobs)"
exit 0
