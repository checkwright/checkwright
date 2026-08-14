#!/usr/bin/env bash
# Behavioral test of the vendored-kit-root prune the one-pair good/bad harness
# cannot hold: it needs a per-case GATE_SDK_KIT_DIRS + CANON_KIT_SCAN_KIT_ROOTS,
# which run-gate-tests passes to neither. The good/bad pair covers the core
# DoD-count logic (exactly-one / doubled heading); these two cases cover the
# finder's kit-root scoping — a DoD-less vendored kit SPEC.md is pruned by
# default (so exactly-one holds on a vendored tree) and re-included by the knob.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # canon-kit/
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0

# A consumer spec (exactly-one DoD — passes on its own) beside a vendored kit
# whose SPEC.md carries no DoD (a reference-spec corpus, as the kits are).
mkdir -p "$SANDBOX/vendored-kit"
cat >"$SANDBOX/SPEC.md" <<'EOF'
# consumer — SPEC

## Definition of Done

- [ ] the one thing is done
EOF
cat >"$SANDBOX/vendored-kit/SPEC.md" <<'EOF'
# vendored-kit — SPEC

Documents a dependency's contract; carries no Definition-of-Done checklist.
EOF

check_case() {  # $1=label  $2=want-rc  $3=want-substring  $4=cwd  $5=root-arg  $6..=env assignments
    local label="$1" want="$2" sub="$3" cwd="$4" rootarg="$5"; shift 5
    local out rc
    # spec: gate-sdk/SPEC.md §run-gate-tests — the gate is named, never spelled as a script
    #   path: gate_run resolves whichever substrate declares it, under this case's own env
    out="$(cd "$cwd" && gate_env "$@" && gate_run check-spec-dod-singleton "$DIR/checks" ${rootarg:+"$rootarg"} 2>&1)"; rc=$?
    if [[ "$rc" -ne "$want" ]]; then
        echo "  FAIL [$label]: want exit $want, got $rc -- $out"; fails=$((fails + 1)); return
    fi
    if [[ -n "$sub" ]] && ! grep -qF -- "$sub" <<<"$out"; then
        echo "  FAIL [$label]: exit $rc OK but output lacks '$sub':"; printf '    %s\n' "$out"
        fails=$((fails + 1))
    fi
}

# Default (CANON_KIT_SCAN_KIT_ROOTS unset ⇒ 0): the vendored kit root is pruned,
# so only the consumer spec is scanned — exactly-one holds, clean.
check_case "prune-default" 0 "1 SPEC.md scanned" "$SANDBOX" "" GATE_SDK_KIT_DIRS=vendored-kit

# Knob on: the kit root is re-included, so its DoD-less SPEC.md now trips
# exactly-one — the flag a first-party corpus (this repo) accepts by choice.
check_case "scan-kit-roots" 1 "vendored-kit/SPEC.md" "$SANDBOX" "" \
    GATE_SDK_KIT_DIRS=vendored-kit CANON_KIT_SCAN_KIT_ROOTS=1

# A scan root that climbs: the same tree reached as '..' from a subdirectory. The prune is a
# prefix test, so it is correct only while the scan root, the kit roots and the walked files
# are all spelled the same way — normalising some and not others prunes nothing at all, a
# silent widening of the corpus rather than a red. The eighth cohort's edge-root parity run
# found exactly that asymmetry (gate-sdk/SPEC.md §The canonical-spec `spec_canonical_specs`
# cohort). What this case holds is the *compiled* prune, since the member now dispatches to
# the binary; it was verified to red under an asymmetric break and to stay green under a
# symmetric one, which is the defect shape it can and cannot see. The shell twin the surviving
# `_spec_prune_kit_roots` callers use is covered by nothing — `spec-prune-normalisation-shell-oracle`.
mkdir -p "$SANDBOX/sub"
check_case "prune-through-dotdot" 0 "1 SPEC.md scanned" "$SANDBOX/sub" ".." \
    GATE_SDK_KIT_DIRS="$SANDBOX/vendored-kit"

if [[ "$fails" -gt 0 ]]; then
    echo "check-spec-dod-singleton.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-spec-dod-singleton.test.sh: clean (kit-root prune default + CANON_KIT_SCAN_KIT_ROOTS re-include + '..' scan-root normalisation, 3 cases)"
exit 0
