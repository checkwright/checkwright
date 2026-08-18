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
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0

mkdir -p "$SANDBOX/memory"   # clean memory dir — isolates the override axis
cat >"$SANDBOX/settings-pins.conf" <<'EOF'
.autoMemoryEnabled = false
EOF
# spec: context-kit/SPEC.md §check-memory-off — the local file's path is derived from the
# settings knob, and the knob's own set-ness contract makes an explicitly-set missing path a
# refusal, so the tracked sibling has to exist for the untracked one to be reachable at all
printf '{}\n' >"$SANDBOX/settings.json"

check_case() {  # $1=label  $2=want-rc  $3=want-substring  $4=settings.local.json body ("" = absent)
    local label="$1" want="$2" sub="$3" body="$4"
    rm -f "$SANDBOX/settings.local.json"
    [[ -n "$body" ]] && printf '%s\n' "$body" >"$SANDBOX/settings.local.json"
    local out rc
    # spec: context-kit/SPEC.md §check-memory-off — the three knobs are the only redirection the
    # ported member has, so the case sets them where it used to pass `--fixture <dir>`
    out="$(
        gate_env \
            CONTEXT_KIT_MEMORY_DIRS="$SANDBOX/memory" \
            CONTEXT_KIT_SETTINGS_FILE="$SANDBOX/settings.json" \
            CONTEXT_KIT_SETTINGS_PINS="$SANDBOX/settings-pins.conf"
        gate_run check-memory-off "$DIR/checks" 2>&1
    )"; rc=$?
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

# spec: context-kit/SPEC.md §check-memory-off — the null disposition, opposite to
# check-settings-pins' absent-pin refusal: a pinned key explicitly set to null sets no override
check_case "local-null-clean" 0 "MEMORY-OFF: clean" '{"autoMemoryEnabled": null}'

if [[ "$fails" -gt 0 ]]; then
    echo "check-memory-off.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-memory-off.test.sh: clean (no-local clean, unrelated-local clean, pinned-key override red, explicit-null clean, 4 cases)"
exit 0
