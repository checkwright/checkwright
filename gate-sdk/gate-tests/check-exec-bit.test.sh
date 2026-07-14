#!/usr/bin/env bash
# Behavioral test of checks/check-exec-bit.sh — the live `git ls-files` path the
# hermetic dump fixtures (argument mode) cannot exercise. Builds a temp git repo,
# stages a KPI plugin at index mode 100644 (red), re-stages it 100755 (green) —
# the exact silent-degradation the gate exists to catch.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # gate-sdk/
GATE="$DIR/checks/check-exec-bit.sh"
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0
repo="$SANDBOX/repo"
mkdir -p "$repo/somekit/kpis"
git -C "$repo" init -q
printf '#!/usr/bin/env bash\necho hi\n' >"$repo/somekit/kpis/kpi-x.sh"
git -C "$repo" add somekit/kpis/kpi-x.sh

# --- index mode 100644: red (a by-path-invoked plugin that will fail to run) ---
git -C "$repo" update-index --chmod=-x somekit/kpis/kpi-x.sh
out="$( cd "$repo" && "$GATE" 2>&1 )"; rc=$?
if [[ "$rc" -ne 1 ]]; then
    echo "  FAIL [644-red]: want exit 1, got $rc -- $out"; fails=$((fails + 1))
elif ! grep -qF 'not committed executable' <<<"$out"; then
    echo "  FAIL [644-red]: exit 1 but output lacks the finding: $out"; fails=$((fails + 1))
fi

# --- index mode 100755: green ---
git -C "$repo" update-index --chmod=+x somekit/kpis/kpi-x.sh
out="$( cd "$repo" && "$GATE" 2>&1 )"; rc=$?
if [[ "$rc" -ne 0 ]]; then
    echo "  FAIL [755-green]: want exit 0, got $rc -- $out"; fails=$((fails + 1))
elif ! grep -qF 'EXEC-BIT: clean' <<<"$out"; then
    echo "  FAIL [755-green]: exit 0 but output lacks the clean line: $out"; fails=$((fails + 1))
fi

if [[ "$fails" -gt 0 ]]; then
    echo "check-exec-bit.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-exec-bit.test.sh: clean (live git ls-files path: 100644 KPI reds, 100755 greens, 2 cases)"
exit 0
