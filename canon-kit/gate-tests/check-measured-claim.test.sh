#!/usr/bin/env bash
# Behavioral test of the arms the one-pair good/bad harness cannot hold: every
# fail-closed exit. A pair asserts one exit code per case dir and the crate's
# read-set unit test asserts no member exits 2 on its own fixtures, so arm B (an
# unemitted key), the ambiguity fail-close and a malformed marker have no pair
# spelling. The inactive-by-default posture is here for the same reason: it needs
# a config the pair's own canon-config.sh cannot also be.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # canon-kit/
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0

cat >"$SANDBOX/cfg.sh" <<'EOF'
CANON_KIT_MEASURED_CLAIMS_CMD='printf "gate-total\t7\n"'
CANON_KIT_MEASURED_SURFACE_GLOBS=("*.md")
EOF

cat >"$SANDBOX/off.sh" <<'EOF'
CANON_KIT_MEASURED_SURFACE_GLOBS=("*.md")
EOF

check_case() {  # $1=label  $2=want-rc  $3=want-substring  $4=config
    local label="$1" want="$2" sub="$3" cfg="$4"
    local out rc
    out="$(cd "$SANDBOX" && gate_env CANON_KIT_CONFIG_FILE="$SANDBOX/$cfg" \
        && gate_run check-measured-claim "$DIR/checks" 2>&1)"; rc=$?
    if [[ "$rc" -ne "$want" ]]; then
        echo "  FAIL [$label]: want exit $want, got $rc -- $out"; fails=$((fails + 1)); return
    fi
    if [[ -n "$sub" ]] && ! grep -qF -- "$sub" <<<"$out"; then
        echo "  FAIL [$label]: exit $rc OK but output lacks '$sub':"; printf '    %s\n' "$out"
        fails=$((fails + 1))
    fi
}

# Arm B: a marker naming a key the emitter does not carry is a claim with no
# oracle wearing the costume of one, so it fails closed rather than reading as
# an agreement nobody could have checked.
cat >"$SANDBOX/SPEC.md" <<'EOF'
# consumer — SPEC

<!-- measured: gate-tally=7 -->
The registry holds 7 gates today.
EOF
check_case "unknown-key-fails-closed" 2 "key 'gate-tally' is absent from the emitter's roster" cfg.sh

# The authoring contract arm C prices: a bound claim carrying more than one
# distinct cardinal is ambiguous, and the gate refuses to guess which one the
# marker holds.
cat >"$SANDBOX/SPEC.md" <<'EOF'
# consumer — SPEC

<!-- measured: gate-total=7 -->
The registry holds 7 gates across 3 kits today.
EOF
check_case "ambiguous-claim-fails-closed" 2 "2 distinct cardinals" cfg.sh

# A marker that does not parse is an authoring error, never a site to skip: a
# skipped malformed marker is an unchecked claim that looks checked.
cat >"$SANDBOX/SPEC.md" <<'EOF'
# consumer — SPEC

<!-- measured: gate-total -->
The registry holds 7 gates today.
EOF
check_case "malformed-marker-fails-closed" 2 "marker does not parse" cfg.sh

# Inactive by default: with no oracle command the gate has nothing to check a
# marker against, and reports clean rather than reddening every marker in a
# tree that never configured it.
cat >"$SANDBOX/SPEC.md" <<'EOF'
# consumer — SPEC

<!-- measured: gate-total=7 -->
The registry holds 7 gates today.
EOF
check_case "no-oracle-is-clean" 0 "MEASURED-CLAIM: clean" off.sh

if [[ "$fails" -gt 0 ]]; then
    echo "check-measured-claim.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-measured-claim.test.sh: clean (unknown key + ambiguous claim + malformed marker all fail closed; an unset oracle is inactive)"
exit 0
