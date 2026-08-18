#!/usr/bin/env bash
# Behavioral test of check-battery-roster — the branches the one
# good/bad pair cannot hold: the three fail-closed misconfigurations (absent
# doc, doc with no marker block, empty suite roster), the normalization arm the
# pair does not reach (leading VAR=value assignments with no `env` token), and
# the suite whose EVIDENCE_KIT_RUN_<suite> is unset, which run-validate rejects
# and this gate therefore leaves alone rather than reporting as undocumented.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # evidence-kit/

fails=0
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

# case_run <name> <config-body> <doc-body|SKIP> <want> <expect>
case_run() {
    local name="$1" cfg="$2" doc="$3" want="$4" expect="$5"
    local d="$tmp/$name" out rc; mkdir -p "$d/scripts"
    printf '%b' "$cfg" >"$d/scripts/evidence-config.sh"
    [[ "$doc" == "SKIP" ]] || printf '%b' "$doc" >"$d/runner.md"
    out="$( cd "$d" && unset EVIDENCE_KIT_CONFIG_FILE \
        && gate_env GATE_SDK_GATES_DIR=scripts \
        && gate_run check-battery-roster "$DIR/checks" runner.md 2>&1 )"; rc=$?
    if [[ "$rc" -ne "$want" ]]; then
        echo "  FAIL: $name expected exit $want, got $rc: $out"; fails=$((fails + 1)); return
    fi
    if ! grep -qF -- "$expect" <<<"$out"; then
        echo "  FAIL: $name exit OK but output lacks '$expect': $out"; fails=$((fails + 1))
    fi
}

CFG1='EVIDENCE_KIT_SUITES=(alpha)\nEVIDENCE_KIT_RUN_alpha="bash bin/run-alpha.sh"\n'
BLOCK='<!-- battery-roster:begin -->\nbash bin/run-alpha.sh\n<!-- battery-roster:end -->\n'

# A — the configured runner doc does not exist: misconfiguration, not a clean.
case_run "no-doc" "$CFG1" SKIP 2 "runner doc not found"

# B — the doc exists but carries no marker block: a consumer keeping no register
#     opts out by not registering the gate, so an unmarked doc is exit 2 and
#     never a vacuous pass over a doc the gate cannot locate the register in.
case_run "no-markers" "$CFG1" 'nothing to see\n' 2 "marker block"

# C — an empty suite roster leaves nothing to hold the doc against.
case_run "empty-suites" 'EVIDENCE_KIT_SUITES=()\n' "$BLOCK" 2 "EVIDENCE_KIT_SUITES is empty"

# D — leading VAR=value assignments with no `env` token normalize away too: the
#     harness's environment is not what a contributor types, whichever spelling
#     the config uses.
case_run "bare-assignment-prefix" \
    'EVIDENCE_KIT_SUITES=(alpha)\nEVIDENCE_KIT_RUN_alpha="VERBOSE=1 bash bin/run-alpha.sh"\n' \
    "$BLOCK" 0 "BATTERY-ROSTER: clean"

# E — a suite with no EVIDENCE_KIT_RUN_<suite> has no documented invocation to
#     compare, and run-validate already exits 2 on it; reporting it here as an
#     undocumented suite would send the reader to the doc to fix a config bug.
case_run "suite-without-command" \
    'EVIDENCE_KIT_SUITES=(alpha orphan)\nEVIDENCE_KIT_RUN_alpha="bash bin/run-alpha.sh"\n' \
    "$BLOCK" 0 "BATTERY-ROSTER: clean"

if [[ "$fails" -gt 0 ]]; then
    echo "check-battery-roster.test: $fails assertion(s) failed"
    exit 1
fi
echo "check-battery-roster.test: ok (absent doc + unmarked doc + empty suite roster fail closed; bare assignment prefix normalized, command-less suite left alone)"
exit 0
