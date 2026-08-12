#!/usr/bin/env bash
# Behavioral test of the config-driven paths the one-pair good/bad harness
# cannot hold with stock defaults: a consumer-appended collection noun and the
# CANON_KIT_COUNT_ALLOWED_PHRASES containment exemption. The allowlist ships
# empty, so only a config that both governs a noun and allowlists a phrase
# containing it exercises the exemption — which run-gate-tests passes to no
# fixture. The good/bad pair covers the cardinal+noun shapes (adjacent, wedged,
# noun-then-range, wrapped) and the mechanical exemptions (threshold, per,
# partition, partitive, inline-code, fence, per-site marker); these cases cover
# the knobs, plus the paragraph-join edges. Two reasons hold those here, and
# neither is expect.txt's old single-substring limit (lines are a conjunction
# now): they need their own scan root, which run-gate-tests gives no fixture
# beyond the one pair, and "the join never double-reports a same-line hit" is a
# negative — expect.txt asserts presence only, so absence has no pair spelling.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # canon-kit/
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0

cat >"$SANDBOX/cfg.sh" <<'EOF'
CANON_KIT_COUNT_COLLECTIONS=("contracts")
CANON_KIT_COUNT_ALLOWED_PHRASES=("the four contracts")
EOF

cat >"$SANDBOX/SPEC.md" <<'EOF'
# consumer — SPEC

The gate suite honors the four contracts, a fixed named set the doc may cite.

A growing collection is different: this kit ships six contracts now.
EOF

check_case() {  # $1=label  $2=want-rc  $3=want-substring  $4..=env assignments
    local label="$1" want="$2" sub="$3"; shift 3
    local out rc
    out="$(cd "$SANDBOX" && gate_env "$@" && gate_run check-manifest-count "$DIR/checks" 2>&1)"; rc=$?
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
check_case "governed-noun-trips" 1 "six contracts" CANON_KIT_CONFIG_FILE="$SANDBOX/cfg.sh"

out="$(cd "$SANDBOX" && gate_env CANON_KIT_CONFIG_FILE="$SANDBOX/cfg.sh" \
    && gate_run check-manifest-count "$DIR/checks" 2>&1)"
if grep -qF -- "four contracts" <<<"$out"; then
    echo "  FAIL [allowlist-exempt]: allowlisted 'the four contracts' was flagged:"
    printf '    %s\n' "$out"; fails=$((fails + 1))
fi

# Stock defaults do not govern "contracts", so the same manifest is clean —
# a non-governed count-noun is never a hit, allowlist or not.
check_case "ungoverned-noun-clean" 0 "MANIFEST-COUNT: clean"

# The paragraph-join window: a total split across a prose wrap is reported at the
# span's *first* physical line (the cardinal's), not the line the noun completes
# it on, and a per-line hit is never double-reported by the join.
mkdir -p "$SANDBOX/wrap"
cat >"$SANDBOX/wrap/SPEC.md" <<'EOF'
# wrap — SPEC

Prose that pins a total across the
wrap: this suite ships two
governed gates today.
EOF
out="$(cd "$SANDBOX/wrap" && gate_run check-manifest-count "$DIR/checks" 2>&1)"; rc=$?
if [[ "$rc" -ne 1 ]]; then
    echo "  FAIL [wrap-flagged]: want exit 1, got $rc -- $out"; fails=$((fails + 1))
elif ! grep -qF -- "SPEC.md:4  restated collection total: two" <<<"$out"; then
    echo "  FAIL [wrap-first-line]: wrapped total not reported at its first physical line:"
    printf '    %s\n' "$out"; fails=$((fails + 1))
elif [[ "$(grep -c 'restated collection total' <<<"$out")" -ne 1 ]]; then
    echo "  FAIL [wrap-single-report]: the join double-reported a span:"
    printf '    %s\n' "$out"; fails=$((fails + 1))
fi

# A blank line ends the paragraph, so a cardinal and a noun that merely abut
# across a paragraph break never join into a phantom total.
cat >"$SANDBOX/wrap/SPEC.md" <<'EOF'
# wrap — SPEC

The registry holds two

gates own their own totals, per `gates.list`.
EOF
check_case "paragraph-break-no-join" 0 "MANIFEST-COUNT: clean"

# The measured-marker discharge is a *window*, not a whole-file valve: the marker
# line and the claim below it are exempt, and the very next paragraph is not — a
# pair cannot spell that, because one file either trips or does not.
cat >"$SANDBOX/wrap/SPEC.md" <<'EOF'
# wrap — SPEC

<!-- measured: gate-total=7 -->
The registry holds 7 gates today, bound to an oracle rather than transcribed.

The registry holds 7 gates today, said again with nothing bound to it.
EOF
out="$(cd "$SANDBOX/wrap" && gate_run check-manifest-count "$DIR/checks" 2>&1)"; rc=$?
if [[ "$rc" -ne 1 ]]; then
    echo "  FAIL [measured-window]: want exit 1, got $rc -- $out"; fails=$((fails + 1))
elif [[ "$(grep -c 'restated collection total' <<<"$out")" -ne 1 ]]; then
    echo "  FAIL [measured-window]: the discharge did not bound itself to the marked claim:"
    printf '    %s\n' "$out"; fails=$((fails + 1))
elif ! grep -qF -- "SPEC.md:6  restated collection total: 7 gates" <<<"$out"; then
    echo "  FAIL [measured-window]: the unmarked restatement was not the one reported:"
    printf '    %s\n' "$out"; fails=$((fails + 1))
fi

if [[ "$fails" -gt 0 ]]; then
    echo "check-manifest-count.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-manifest-count.test.sh: clean (governed-noun trip + allowlist containment + ungoverned-noun clean + wrapped-total first-line report + paragraph-break non-join + measured-marker discharge window)"
exit 0
