#!/usr/bin/env bash
# Direct unit test of check-graph's external-ref assertion (assertion H) via
# the hermetic --refs-only mode — the good/bad pair stays the themeless
# --amend-only case, so the emit-side ref allowlist needs its own test: a theme
# chrome link that matches no allowed prefix reds, the same link allowlisted via
# GATE_SDK_GRAPH_EXTERNAL_REFS clears, and the kit-seeded mermaid import is always
# allowed even under an empty knob. An empty GATE_SDK_CONFIG_FILE isolates the
# test from this repo's persistent config (which sets the knob). The chrome link
# is injected through a GATE_SDK_GRAPH_THEME_DIR part file: this test is not
# theme-neutral, so it carries the same migration the theme test does.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # gate-sdk/
CHECKS="$DIR/checks"

fails=0
tmp="$(mktemp -d)"; trap 'rm -rf "$tmp"' EXIT

: > "$tmp/empty-config.sh"        # neutralize the repo's gate-sdk-config.sh
EXT="https://third-party.example/widget.js"

mkdir -p "$tmp/theme"
printf '  <a href="%s">third-party</a>\n' "$EXT" > "$tmp/theme/footer.html"

run() {  # run() <config> <theme-dir> <knob> -> sets rc/out
    out="$(gate_env GATE_SDK_CONFIG_FILE="$1" GATE_SDK_GRAPH_THEME_DIR="$2" \
                    GATE_SDK_GRAPH_EXTERNAL_REFS="$3" \
           && gate_run check-graph "$CHECKS" --refs-only 2>&1)"
    rc=$?
}

# --- an un-allowlisted theme chrome link reds --------------------------------
run "$tmp/empty-config.sh" "$tmp/theme" ""
[[ "$rc" -eq 1 ]] \
    || { echo "  FAIL: un-allowlisted ref expected exit 1, got $rc: $out"; fails=$((fails + 1)); }
grep -qF -- "$EXT" <<<"$out" \
    || { echo "  FAIL: red output does not name the offending ref '$EXT': $out"; fails=$((fails + 1)); }

# --- the same link, allowlisted by the knob, clears --------------------------
run "$tmp/empty-config.sh" "$tmp/theme" "https://third-party.example"
[[ "$rc" -eq 0 ]] \
    || { echo "  FAIL: knob-allowlisted ref expected exit 0, got $rc: $out"; fails=$((fails + 1)); }

# --- the seeded mermaid import alone is allowed under an empty knob -----------
run "$tmp/empty-config.sh" "$tmp/absent" ""
[[ "$rc" -eq 0 ]] \
    || { echo "  FAIL: seeded-mermaid-only emit expected exit 0, got $rc: $out"; fails=$((fails + 1)); }

if [[ "$fails" -gt 0 ]]; then
    echo "check-graph-refs.test: $fails assertion(s) failed"
    exit 1
fi
echo "check-graph-refs.test: ok (un-allowlisted ref reds; knob clears; seeded mermaid always allowed)"
exit 0
