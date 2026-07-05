#!/usr/bin/env bash
# Behavioral test of the vendored-kit-root prune the one-pair good/bad harness
# cannot hold: it needs a per-case GATE_SDK_KIT_DIRS + SPEC_KIT_SCAN_KIT_ROOTS,
# which run-gate-tests passes to neither. The good/bad pair covers the core
# DoD-count logic (exactly-one / doubled heading); these two cases cover the
# finder's kit-root scoping — a DoD-less vendored kit SPEC.md is pruned by
# default (so exactly-one holds on a vendored tree) and re-included by the knob.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # spec-kit/
GATE="$DIR/checks/check-spec-dod-singleton.sh"
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

# Default (SPEC_KIT_SCAN_KIT_ROOTS unset ⇒ 0): the vendored kit root is pruned,
# so only the consumer spec is scanned — exactly-one holds, clean.
check_case "prune-default" 0 "1 SPEC.md scanned" GATE_SDK_KIT_DIRS=vendored-kit

# Knob on: the kit root is re-included, so its DoD-less SPEC.md now trips
# exactly-one — the flag a first-party corpus (this repo) accepts by choice.
check_case "scan-kit-roots" 1 "vendored-kit/SPEC.md" \
    GATE_SDK_KIT_DIRS=vendored-kit SPEC_KIT_SCAN_KIT_ROOTS=1

if [[ "$fails" -gt 0 ]]; then
    echo "check-spec-dod-singleton.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-spec-dod-singleton.test.sh: clean (kit-root prune default + SPEC_KIT_SCAN_KIT_ROOTS re-include, 2 cases)"
exit 0
