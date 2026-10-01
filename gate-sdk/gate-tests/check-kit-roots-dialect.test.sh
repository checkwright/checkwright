#!/usr/bin/env bash
# Behavioral test of check-kit-roots-dialect — the hook environment the fixture
# pair cannot carry. Under a partial-path commit git exports a relative
# GIT_INDEX_FILE to the pre-commit hook; the gate's vendoring children must not
# inherit it, since inside a vendoring it names no index. The good tree is run
# with such a value set and must stay clean.
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # gate-sdk/
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0
tree="$DIR/gate-tests/check-kit-roots-dialect/good/tree"

out="$( cd "$SANDBOX" && GATE_SDK_TMP_DIR="$SANDBOX/tmp" GIT_INDEX_FILE=no-such-dir/next-index-1.lock \
    gate_run check-kit-roots-dialect "$DIR/checks" "$tree" 2>&1 )"; rc=$?
if [[ "$rc" -ne 0 ]]; then
    echo "  FAIL [relative-index]: want exit 0, got $rc -- $out"; fails=$((fails + 1))
elif ! grep -qF 'KIT-ROOTS-DIALECT: clean' <<<"$out"; then
    echo "  FAIL [relative-index]: exit 0 but output lacks the clean line: $out"; fails=$((fails + 1))
fi

if [[ "$fails" -gt 0 ]]; then
    echo "check-kit-roots-dialect.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-kit-roots-dialect.test.sh: clean (a relative GIT_INDEX_FILE the hook exports reaches no vendoring child, 1 case)"
exit 0
