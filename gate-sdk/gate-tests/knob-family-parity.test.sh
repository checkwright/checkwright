#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §lib/gate.sh — the two bridge arms serialize one variable identically: a knob named outright and the same knob reached through its prefix family must cross as the same element, for every shape a knob can have. The associative case is the one that drifted — `${map[*]}` yields values with the keys destroyed — and it drifted silently because the family form's two live members carry no keyed knob, so this holds the arms against each other rather than against either one's output.
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"
# shellcheck source=../lib/gate.sh
source "$(dirname "${BASH_SOURCE[0]}")/../lib/gate.sh"

fails=0
note() { echo "  FAIL [$1]: $2"; fails=$((fails + 1)); }

# The subject variables are declared here rather than in a kit library: both arms resolve from
# whatever scope they are called in, which in production is the owning kit's sourced subshell.
declare -A PARITYKIT_MAP=([zeta]=last [alpha]=first [mid]=middle)
PARITYKIT_ARRAY=(one two three)
PARITYKIT_SCALAR=solo
declare -A PARITYKIT_EMPTYMAP=()
PARITYKIT_EMPTYARRAY=()

family="$(_gate_knob_prefix_emit PARITYKIT_ 0)" || note family-arm "the family arm refused a well-formed set"

for name in PARITYKIT_MAP PARITYKIT_ARRAY PARITYKIT_SCALAR PARITYKIT_EMPTYMAP PARITYKIT_EMPTYARRAY; do
    named="$(_gate_knob_emit "$name" parity-probe 0 /nonexistent-kit)" \
        || { note "named-arm[$name]" "the named arm refused a well-formed knob"; continue; }
    got="$(grep -F "GATE_SDK_KNOB_$name=" <<<"$family")"
    [[ "$got" == "$named" ]] \
        || note "parity[$name]" "the family arm and the named arm disagree
    named:  $named
    family: $got"
done

# The associative case stated as its own assertion too, so a regression reports the destroyed
# keys by name rather than only as a diff between two arms that could both go wrong together.
grep -qF 'GATE_SDK_KNOB_PARITYKIT_MAP=alpha=first' <<<"$family" \
    || note keys-destroyed "the family arm dropped a keyed knob's keys: $(grep -F PARITYKIT_MAP <<<"$family")"

[[ "$fails" -eq 0 ]] || { echo "knob-family-parity.test: $fails assertion(s) failed"; exit 1; }
echo "knob-family-parity.test: clean (the prefix-family arm and the named arm serialize a keyed, an indexed, a scalar and both empty knobs identically, and a keyed knob crosses as sorted <key>=<value> pairs)"
exit 0
