#!/usr/bin/env bash
# Behavioral coverage the one good/bad pair cannot hold: the knob's owning SPEC is
# resolved by its prefix, not the citing file's kit (a widget-kit source citing a
# GATE_SDK_ knob couples to gate-sdk's SPEC); a default the owning SPEC never states
# fires "stated nowhere"; and the three skip classes — a descriptively-stated
# default, a guarded array, an empty fallback, and a deferral expression whose tail
# the SPEC states — are counted, not flagged. A GATE_SDK_KIT_DIRS override makes two
# sandbox kits the whole roster, deterministic and independent of the real tree.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # canon-kit/
GATE="$DIR/checks/check-knob-default-coupling.sh"
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0
mkdir -p "$SANDBOX/gate-sdk" "$SANDBOX/widget-kit/lib"

run_gate() {  # echoes gate output; roster is the two sandbox kits
    ( cd "$SANDBOX" && env GATE_SDK_KIT_DIRS="gate-sdk widget-kit" GATE_SDK_ROOT="$DIR/../gate-sdk" "$GATE" 2>&1 )
}

check_case() {  # $1=label  $2=want-rc  $3=want-substring
    local label="$1" want="$2" sub="$3" out rc
    out="$(run_gate)"; rc=$?
    if [[ "$rc" -ne "$want" ]]; then
        echo "  FAIL [$label]: want exit $want, got $rc -- $out"; fails=$((fails + 1)); return
    fi
    if [[ -n "$sub" ]] && ! grep -qF -- "$sub" <<<"$out"; then
        echo "  FAIL [$label]: exit $rc OK but output lacks '$sub':"; printf '    %s\n' "$out"
        fails=$((fails + 1))
    fi
}

# gate-sdk owns GATE_SDK_QUEUE_FILE and states its default; widget-kit source cites
# it (cross-kit) and defines its own knobs the three skip classes cover. All clean.
cat >"$SANDBOX/gate-sdk/SPEC.md" <<'EOF'
# gate-sdk — SPEC

- `GATE_SDK_QUEUE_FILE` — default `TASK-QUEUE.md`.
EOF
cat >"$SANDBOX/widget-kit/SPEC.md" <<'EOF'
# widget-kit — SPEC

- `WIDGET_KIT_TRIGGER_REGEX` — default = the shipped phrase set.
- `WIDGET_KIT_INHERITED` — default `${GATE_SDK_QUEUE_FILE:-TASK-QUEUE.md}`.
EOF
cat >"$SANDBOX/widget-kit/lib/widget.sh" <<'EOF'
#!/usr/bin/env bash
: "${GATE_SDK_QUEUE_FILE:-TASK-QUEUE.md}"                 # owning SPEC is gate-sdk's
: "${WIDGET_KIT_TRIGGER_REGEX:-a|b(c|d)}"                 # descriptively stated — skip
: "${WIDGET_KIT_INHERITED:-TASK-QUEUE.md}"                # deferral tail the SPEC states — clean
: "${WIDGET_KIT_PRUNE:-}"                                 # empty fallback (set -u guard) — skip
[[ -v WIDGET_KIT_SECTIONS ]] || WIDGET_KIT_SECTIONS=(a b) # array default — skip
EOF
check_case "cross-kit-owner-and-skips-clean" 0 "KNOB-DEFAULT-COUPLING: clean"

# A knob whose owning SPEC states no default at all fires "stated nowhere".
cat >"$SANDBOX/widget-kit/lib/widget.sh" <<'EOF'
#!/usr/bin/env bash
: "${WIDGET_KIT_UNDOCUMENTED:-value.md}"
EOF
check_case "absent-default-fires" 1 "stated nowhere in the owning SPEC"

if [[ "$fails" -gt 0 ]]; then
    echo "check-knob-default-coupling.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-knob-default-coupling.test.sh: clean (cross-kit owning SPEC + descriptive/array/empty/deferral skips + absent-default fires)"
exit 0
