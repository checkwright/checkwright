#!/usr/bin/env bash
# Synthetic-kit-root cases the one-pair good/bad harness cannot hold: they need
# a per-case GATE_SDK_KIT_DIRS (run-gate-tests passes it to neither case) and a
# git tree whose gate-tests membership the gate reads via ls-files. The good/bad
# pair covers assertion A against the repo's own docs; these cover assertion B
# (a gate-tests-shipping kit absent from the runner doc), the B-owes-nothing
# rule for a kit without gate-tests, A's acceptance of both registry link forms
# (bare kit dir / a page under it), and the fail-closed missing-doc path.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # gate-sdk/
GATE="$DIR/checks/check-kit-registration.sh"
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0

# A two-kit synthetic tree: alpha-kit ships a tracked gate-tests/ file, beta-kit
# does not. Both are registered in README — alpha by the bare-dir link form,
# beta by the page form, so a run that stays clean proves assertion A accepts
# both. The runner doc names only alpha's fixture line — beta owes none (no
# gate-tests), alpha's presence is the pass.
mkdir -p "$SANDBOX/alpha-kit/gate-tests/check-x" "$SANDBOX/beta-kit/checks"
: >"$SANDBOX/alpha-kit/gate-tests/check-x/keep"
: >"$SANDBOX/beta-kit/checks/keep"
cat >"$SANDBOX/README.md" <<'EOF'
# registry
- [alpha-kit/](alpha-kit/)
- [beta-kit](beta-kit/index.md)
EOF
git -C "$SANDBOX" init -q
git -C "$SANDBOX" add -A
git -C "$SANDBOX" -c user.email=t@e -c user.name=t commit -qm init

run() {  # $1=label $2=want-rc $3=want-substring $4=runner-doc-body
    local label="$1" want="$2" sub="$3" body="$4" out rc
    printf '%s' "$body" >"$SANDBOX/RUNNER.md"
    out="$(cd "$SANDBOX" && env GATE_SDK_KIT_DIRS="alpha-kit beta-kit" \
        GATE_SDK_ROOT="$DIR" "$GATE" README.md RUNNER.md 2>&1)"; rc=$?
    if [[ "$rc" -ne "$want" ]]; then
        echo "  FAIL [$label]: want exit $want, got $rc -- $out"; fails=$((fails + 1)); return
    fi
    if [[ -n "$sub" ]] && ! grep -qF -- "$sub" <<<"$out"; then
        echo "  FAIL [$label]: exit $rc OK but output lacks '$sub':"; printf '    %s\n' "$out"
        fails=$((fails + 1))
    fi
}

# Runner doc names alpha's fixture line: alpha (ships gate-tests) satisfied,
# beta owes nothing -> clean, and only alpha counts toward the runner tally.
# Clean also means A took beta's page-form row: a bare-dir-only match would
# name beta here.
run "b-owed-satisfied" 0 "(2 kit root(s) each carry a registry row; 1 shipping gate-tests" \
    "run-gate-tests.sh alpha-kit/gate-tests alpha-kit/checks"

# Runner doc omits alpha's fixture line: a gate-tests-shipping kit fell out of
# the documented battery -> assertion B rejects, naming alpha-kit.
run "b-missing" 1 "alpha-kit" "no fixture-runner lines here"

# Fail-closed: a configured runner doc that does not exist is exit 2, not a pass.
out="$(cd "$SANDBOX" && env GATE_SDK_KIT_DIRS="alpha-kit beta-kit" GATE_SDK_ROOT="$DIR" \
    "$GATE" README.md no-such-doc.md 2>&1)"; rc=$?
if [[ "$rc" -ne 2 ]]; then
    echo "  FAIL [missing-doc]: want exit 2, got $rc -- $out"; fails=$((fails + 1))
fi

if [[ "$fails" -gt 0 ]]; then
    echo "check-kit-registration.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-kit-registration.test.sh: clean (assertion B satisfied/missing, B-owes-nothing, both A link forms, fail-closed missing-doc — 3 cases)"
exit 0
