#!/usr/bin/env bash
# Behavioral test of the second red condition the good/bad pair cannot express:
# an untracked settings.local.json that overrides a pinned key. The pair fixes
# the memory-dir axis (empty vs polluted); this holds the local-override axis —
# a clean dir plus a local file that re-enables what the tracked pin disabled.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # context-kit/
GATE="$DIR/checks/check-memory-off.sh"
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0

mkdir -p "$SANDBOX/memory"   # clean memory dir — isolates the override axis
cat >"$SANDBOX/settings-pins.conf" <<'EOF'
.autoMemoryEnabled = false
EOF

check_case() {  # $1=label  $2=want-rc  $3=want-substring  $4=settings.local.json body ("" = absent)
    local label="$1" want="$2" sub="$3" body="$4"
    rm -f "$SANDBOX/settings.local.json"
    [[ -n "$body" ]] && printf '%s\n' "$body" >"$SANDBOX/settings.local.json"
    local out rc
    out="$(CONTEXT_KIT_CONFIG_FILE=/dev/null "$GATE" --fixture "$SANDBOX" 2>&1)"; rc=$?
    if [[ "$rc" -ne "$want" ]]; then
        echo "  FAIL [$label]: want exit $want, got $rc -- $out"; fails=$((fails + 1)); return
    fi
    if [[ -n "$sub" ]] && ! grep -qF -- "$sub" <<<"$out"; then
        echo "  FAIL [$label]: exit $rc OK but output lacks '$sub':"; printf '    %s\n' "$out"
        fails=$((fails + 1))
    fi
}

# No local file: the clean dir alone is a clean posture.
check_case "no-local-file-clean" 0 "MEMORY-OFF: clean" ""

# Local file present but not touching the pinned key: still clean.
check_case "local-file-unrelated-clean" 0 "MEMORY-OFF: clean" '{"spinnerTipsEnabled": false}'

# Local file re-enables memory contrary to the pin: the override the hermetic
# gate cannot see, caught here.
check_case "local-override-red" 1 "local settings override" '{"autoMemoryEnabled": true}'

if [[ "$fails" -gt 0 ]]; then
    echo "check-memory-off.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-memory-off.test.sh: clean (no-local clean, unrelated-local clean, pinned-key override red, 3 cases)"
exit 0
