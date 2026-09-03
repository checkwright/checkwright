#!/usr/bin/env bash
# Behavioral test of this repo's WIRING of EVIDENCE_KIT_PARSER_<suite>: that the values
# scripts/evidence-config.sh actually configures resolve and answer through ek_parse. The crate's
# own #[cfg(test)] module holds each arm's grammar, so it cannot see whether the CONFIGURED value
# reaches it — which is how a port that deleted a knob value's target once left a seam degraded
# silently for 77 firings. This test drives the seam, never a hardcoded arm invocation, so arms
# C and D are negative controls proving it would notice.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
# spec: gate-sdk/SPEC.md §lib/test-hermetic.sh — the preamble above pins every kit's config file at
# an empty one so a KIT test runs on kit defaults; this is a CONSUMER-wiring test and the values
# under test live in this repo's consumer config, so that file IS the subject here
export EVIDENCE_KIT_CONFIG_FILE="$ROOT/scripts/evidence-config.sh"

fails=0
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

cd "$ROOT" || { echo "  FAIL: cannot reach the repo root"; exit 1; }
# shellcheck source=../../evidence-kit/lib/evidence.sh
source "$ROOT/evidence-kit/lib/evidence.sh"

check() {  # $1=name $2=got $3=want
    [[ "$2" == "$3" ]] || { echo "  FAIL: $1: got [$2] want [$3]"; fails=$((fails + 1)); }
}

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

# A — the CONFIGURED gates value resolves and maps the verbose log to one line per gate.
check "configured gates value" "$(ek_parse gates "$tmp/verbose.log" 1)" "check-alpha pass
check-beta fail
check-gamma fail"

# B — the CONFIGURED installer_smoke value resolves, including the driver path its leading
#     positional carries, and attributes the last arm reached off the absent completion marker.
cat >"$tmp/smoke.log" <<'EOF'
build (the host gate binary the main payload carries)
pack
install (from the tarball, --offline)
boom
EOF
check "configured installer_smoke value" "$(ek_parse installer_smoke "$tmp/smoke.log" 1)" "build pass
pack pass
install fail"

# C — negative control: a gates value pointing at a path that does not resolve must NOT produce
#     scenario lines. This is the arm a hardcoded-arm test cannot make, and the failure mode it
#     names is the one that shipped green before.
out="$(EVIDENCE_KIT_PARSER_gates='bash scripts/parse-gates-log.sh' ek_parse gates "$tmp/verbose.log" 1 2>/dev/null)"
[[ -z "$out" ]] || { echo "  FAIL: a dead gates parser value still produced lines: $out"; fails=$((fails + 1)); }

# D — negative control for the failure mode this wiring newly admits: an installer_smoke value
#     whose leading positional is missing, so the arm cannot derive a roster and refuses.
out="$(EVIDENCE_KIT_PARSER_installer_smoke='bash gate-sdk/bin/run-gates.sh --emit parse-smoke-log' \
    ek_parse installer_smoke "$tmp/smoke.log" 1 2>/dev/null)"
[[ -z "$out" ]] || { echo "  FAIL: a driver-less installer_smoke value still produced lines: $out"; fails=$((fails + 1)); }

if [[ "$fails" -gt 0 ]]; then
    echo "evidence-parser-values.test: $fails assertion(s) failed"
    exit 1
fi
echo "evidence-parser-values.test: ok (both configured EVIDENCE_KIT_PARSER_<suite> values resolve and answer through ek_parse; a dead value and a value missing its driver positional each produce nothing)"
exit 0
