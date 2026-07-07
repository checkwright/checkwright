#!/usr/bin/env bash
# Behavioral test of the config-driven paths the one-pair good/bad harness
# cannot hold with stock defaults: a consumer-appended collection noun and the
# SPEC_KIT_COUNT_ALLOWED_PHRASES containment exemption. The default allowlist
# entry ("the four contracts") names a set whose noun is not governed by
# default, so only a config that governs that noun exercises the exemption —
# which run-gate-tests passes to no fixture. The good/bad pair covers the
# core cardinal+noun detection and the mechanical exemptions (threshold, per,
# partition, inline-code, fence, per-site marker); these cases cover the knobs.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # spec-kit/
GATE="$DIR/checks/check-manifest-count.sh"
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0

cat >"$SANDBOX/cfg.sh" <<'EOF'
SPEC_KIT_COUNT_COLLECTIONS=("contracts")
SPEC_KIT_COUNT_ALLOWED_PHRASES=("the four contracts")
EOF

cat >"$SANDBOX/SPEC.md" <<'EOF'
# consumer — SPEC

The gate suite honors the four contracts, a fixed named set the doc may cite.

A growing collection is different: this kit ships six contracts now.
EOF

check_case() {  # $1=label  $2=want-rc  $3=want-substring  $4..=env assignments
    local label="$1" want="$2" sub="$3"; shift 3
    local out rc
    out="$(cd "$SANDBOX" && env "$@" "$GATE" 2>&1)"; rc=$?
    if [[ "$rc" -ne "$want" ]]; then
        echo "  FAIL [$label]: want exit $want, got $rc -- $out"; fails=$((fails + 1)); return
    fi
    if [[ -n "$sub" ]] && ! grep -qF -- "$sub" <<<"$out"; then
        echo "  FAIL [$label]: exit $rc OK but output lacks '$sub':"; printf '    %s\n' "$out"
        fails=$((fails + 1))
    fi
}

# Consumer governs "contracts": the growing "six contracts" total trips, while
# the allowlisted fixed set "the four contracts" is exempt — so the run fails
# on the growing total and its message never names the allowlisted phrase.
check_case "governed-noun-trips" 1 "six contracts" SPEC_KIT_CONFIG_FILE="$SANDBOX/cfg.sh"

out="$(cd "$SANDBOX" && env SPEC_KIT_CONFIG_FILE="$SANDBOX/cfg.sh" "$GATE" 2>&1)"
if grep -qF -- "four contracts" <<<"$out"; then
    echo "  FAIL [allowlist-exempt]: allowlisted 'the four contracts' was flagged:"
    printf '    %s\n' "$out"; fails=$((fails + 1))
fi

# Stock defaults do not govern "contracts", so the same manifest is clean —
# a non-governed count-noun is never a hit, allowlist or not.
check_case "ungoverned-noun-clean" 0 "MANIFEST-COUNT: clean"

if [[ "$fails" -gt 0 ]]; then
    echo "check-manifest-count.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-manifest-count.test.sh: clean (governed-noun trip + allowlist containment + ungoverned-noun clean, 3 cases)"
exit 0
