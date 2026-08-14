#!/usr/bin/env bash
# Behavioral test of the red condition the good/bad pair cannot express: the pair
# fixes the holds-vs-mismatch axis (exit 0 vs exit 1), and this holds the
# **refusal** axis — a pin outside the documented path grammar exits 2 naming the
# pin, the knob it came from and the construct, never a silent clean verdict.
# The narrowing is only a guard if it is loud, so the loudness is what is pinned.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # context-kit/
SDK="$DIR/../gate-sdk"
# shellcheck source=../../gate-sdk/lib/gate.sh
source "$SDK/lib/gate.sh"
BIN="$(gate_native_bin)"
[[ -x "$BIN" ]] || { echo "check-settings-pins.test.sh: gate binary absent at $BIN — build it: bash gate-sdk/bin/build-native.sh" >&2; exit 1; }

SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT
printf '{"a":{"b":1},"n":null,"arr":[10,20],"f":1.0}\n' >"$SANDBOX/settings.json"

fails=0

check_case() {  # $1=label  $2=want-rc  $3=want-substring  $4=pins-file body
    local label="$1" want="$2" sub="$3" body="$4" out rc
    printf '%s\n' "$body" >"$SANDBOX/settings-pins.conf"
    out="$("$BIN" check-settings-pins --fixture "$SANDBOX" 2>&1)"; rc=$?
    if [[ "$rc" -ne "$want" ]]; then
        echo "  FAIL [$label]: want exit $want, got $rc -- $out"; fails=$((fails + 1)); return
    fi
    if [[ -n "$sub" ]] && ! grep -qF -- "$sub" <<<"$out"; then
        echo "  FAIL [$label]: exit $rc OK but output lacks '$sub':"; printf '    %s\n' "$out"
        fails=$((fails + 1))
    fi
}

# Every out-of-subset construct the grammar refuses, each naming itself.
check_case "filter-pipe-refused"   2 "not a jq filter"        '.a | .b = 1'
check_case "error-suppression-refused" 2 "not a jq filter"    '.a? = 1'
check_case "iteration-refused"     2 "not followed by an identifier" '.[] = 1'
check_case "slice-refused"         2 "unterminated"           '.a[1:2] = 1'
check_case "array-literal-refused" 2 "jq array literal"       '["a"] = 1'
check_case "function-refused"      2 "opens with '.'"         'map(.x) = 1'

# The refusal names the knob, so its reader is sent to the right file.
check_case "refusal-names-the-knob" 2 "CONTEXT_KIT_SETTINGS_PINS" '.a | .b = 1'

# jq's indexing type rules are preserved: a field step on an array is an error,
# classified as a malformed pin exactly as the shell's non-zero jq status was.
check_case "field-step-on-array-is-malformed" 2 "cannot index array" '.arr.k = 1'
check_case "index-step-on-object-is-malformed" 2 "cannot index object" '.a[0] = 1'

# In-subset pins still work, so the refusals above are not a blanket rejection.
check_case "dotted-path-holds"     0 "SETTINGS-PINS: clean" '.a.b = 1'
check_case "quoted-key-holds"      0 "SETTINGS-PINS: clean" '."a"."b" = 1'
check_case "negative-index-holds"  0 "SETTINGS-PINS: clean" '.arr[-1] = 20'

# Numbers compare by f64, not by the parsed value's variant: 1.0 == 1.
check_case "number-variants-compare-equal" 0 "SETTINGS-PINS: clean" '.f = 1'

# The absent branch conflates a missing key with an explicit null, deliberately.
check_case "explicit-null-reads-as-absent" 2 "no such key" '.n = 1'

if [[ "$fails" -gt 0 ]]; then
    echo "check-settings-pins.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-settings-pins.test.sh: clean (6 grammar refusals, knob named, 2 jq type-rule errors, 3 in-subset holds, f64 equality, null conflation — 14 cases)"
exit 0
