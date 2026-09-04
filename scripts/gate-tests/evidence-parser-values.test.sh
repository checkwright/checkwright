#!/usr/bin/env bash
# Behavioral test of this repo's WIRING of EVIDENCE_KIT_PARSER_<suite>: that the values
# scripts/evidence-config.sh actually configures resolve and answer through the compiled parser
# dispatch. The crate's own #[cfg(test)] module holds each arm's grammar, so it cannot see whether
# the CONFIGURED value reaches it — which is how a port that deleted a knob value's target once
# left a seam degraded silently for 77 firings. This test drives the seam, never a hardcoded arm
# invocation, so arms C and D are negative controls proving it would notice.
#
# spec: evidence-kit/SPEC.md §lib/evidence.sh — the dispatch is the compiled twin since the
# 2026-09-04 diff cut retired the shell adapters, so the seam is driven through the front end's
# `--diff-baseline` arm. The scenarios are read out of its FINDINGS against a fixture baseline:
# a scenario the configured value failed to produce reds as an absent baseline row, which is what
# makes a dead value observable rather than merely quiet.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
FE="$ROOT/gate-sdk/bin/run-gates.sh"
CONFIG="$ROOT/scripts/evidence-config.sh"

fails=0
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT
mkdir -p "$tmp/scratch"

cd "$ROOT" || { echo "  FAIL: cannot reach the repo root"; exit 1; }

# spec: gate-sdk/SPEC.md §lib/test-hermetic.sh — the preamble above pins every kit's config file at
# an empty one so a KIT test runs on kit defaults; this is a CONSUMER-wiring test and the values
# under test live in this repo's consumer config, so that file IS the subject here. A negative
# control edits a COPY of it rather than the environment, because the config is sourced after the
# environment and would overwrite an env override — which is the seam, not a test detail.
# The findings are STDOUT and a parser's own diagnostic is stderr, so the two are held apart:
# what a dead value proves is the absence of scenario lines, and folding its complaint into the
# compared stream would make the control pass on the complaint instead.
_diff() {   # $1 = config file, $2.. = argv groups; stderr lands in $tmp/err
    env EVIDENCE_KIT_CONFIG_FILE="$1" \
        EVIDENCE_KIT_BASELINE_FILE="$tmp/base.txt" \
        EVIDENCE_KIT_SKIP_FILE="$tmp/no-skips.txt" \
        EVIDENCE_KIT_TMP_DIR="$tmp/scratch" \
        bash "$FE" --diff-baseline "${@:2}" 2>"$tmp/err"
    printf 'rc=%s\n' "$?"
}

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

cat >"$tmp/smoke.log" <<'EOF'
build (the host gate binary the main payload carries)
pack
install (from the tarball, --offline)
boom
EOF

# The baseline names the scenario each configured value must produce as `pass`, so a value that
# produced nothing reds it as absent while a value that produced it stays silent; the scenarios it
# must produce as `fail` carry no row, so the fail-closed rule surfaces them as new failures.
printf '# fixture\ngates check-alpha pass\ninstaller_smoke build pass\ninstaller_smoke pack pass\n' \
    >"$tmp/base.txt"

# A — the CONFIGURED gates value resolves and maps the verbose log to one scenario per gate.
check "configured gates value" "$(_diff "$CONFIG" gates "$tmp/verbose.log" 1)" \
"new-failure gates check-beta
new-failure gates check-gamma
diff-baseline: NEW failures against $tmp/base.txt (see 'new-failure' lines above)
rc=1"

# B — the CONFIGURED installer_smoke value resolves, including the driver path its leading
#     positional carries, and attributes the last arm reached off the absent completion marker.
check "configured installer_smoke value" "$(_diff "$CONFIG" installer_smoke "$tmp/smoke.log" 1)" \
"new-failure installer_smoke install
diff-baseline: NEW failures against $tmp/base.txt (see 'new-failure' lines above)
rc=1"

# C — negative control: a gates value pointing at a path that does not resolve must produce NO
#     scenario lines, so the baselined `check-alpha pass` reds as absent and neither `fail`
#     scenario appears. This is the arm a hardcoded-arm test cannot make, and the failure mode it
#     names is the one that shipped green before.
sed "s#^EVIDENCE_KIT_PARSER_gates=.*#EVIDENCE_KIT_PARSER_gates='bash scripts/parse-gates-log.sh'#" \
    "$CONFIG" >"$tmp/dead-gates.sh"
check "a dead gates value produces nothing" "$(_diff "$tmp/dead-gates.sh" gates "$tmp/verbose.log" 1)" \
"new-failure gates check-alpha
diff-baseline: NEW failures against $tmp/base.txt (see 'new-failure' lines above)
rc=1"
grep -q 'parse-gates-log.sh' "$tmp/err" \
    || { echo "  FAIL: the dead gates value produced no diagnostic naming the path that did not resolve"; fails=$((fails + 1)); }

# D — negative control for the failure mode this wiring newly admits: an installer_smoke value
#     whose leading positional is missing, so the arm cannot derive a roster and refuses.
sed "s#^EVIDENCE_KIT_PARSER_installer_smoke=.*#EVIDENCE_KIT_PARSER_installer_smoke='bash gate-sdk/bin/run-gates.sh --emit parse-smoke-log'#" \
    "$CONFIG" >"$tmp/driverless.sh"
check "a driver-less installer_smoke value produces nothing" \
    "$(_diff "$tmp/driverless.sh" installer_smoke "$tmp/smoke.log" 1)" \
"new-failure installer_smoke build
new-failure installer_smoke pack
diff-baseline: NEW failures against $tmp/base.txt (see 'new-failure' lines above)
rc=1"
grep -q 'the driver is the consumer' "$tmp/err" \
    || { echo "  FAIL: the driver-less value produced no refusal naming the missing positional"; fails=$((fails + 1)); }

if [[ "$fails" -gt 0 ]]; then
    echo "evidence-parser-values.test: $fails assertion(s) failed"
    exit 1
fi
echo "evidence-parser-values.test: ok (both configured EVIDENCE_KIT_PARSER_<suite> values resolve and answer through the compiled dispatch; a dead value and a value missing its driver positional each produce nothing)"
exit 0
