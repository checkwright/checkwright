#!/usr/bin/env bash
# Direct unit test of check-graph.sh's external-ref assertion (assertion H) via
# the hermetic --refs-only mode — the good/bad pair stays the themeless
# --amend-only case, so the emit-side ref allowlist needs its own test: a theme
# chrome link that matches no allowed prefix reds, the same link allowlisted via
# GATE_SDK_GRAPH_EXTERNAL_REFS clears, and the kit-seeded mermaid import is always
# allowed even under an empty knob. An empty GATE_SDK_CONFIG_FILE isolates the
# test from this repo's persistent config (which sets the knob).
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # gate-sdk/
GATE="$DIR/checks/check-graph.sh"

fails=0
tmp="$(mktemp -d)"; trap 'rm -rf "$tmp"' EXIT

: > "$tmp/empty-config.sh"        # neutralize the repo's gate-sdk-config.sh
EXT="https://third-party.example/widget.js"

cat > "$tmp/theme.sh" <<THEME
graph_theme_footer() { echo "  <a href=\"$EXT\">third-party</a>"; }
THEME

run() {  # run() <config> <theme> <knob> -> sets rc/out
    out="$(GATE_SDK_CONFIG_FILE="$1" GATE_SDK_GRAPH_THEME="$2" \
           GATE_SDK_GRAPH_EXTERNAL_REFS="$3" "$GATE" --refs-only 2>&1)"
    rc=$?
}

# --- an un-allowlisted theme chrome link reds --------------------------------
run "$tmp/empty-config.sh" "$tmp/theme.sh" ""
[[ "$rc" -eq 1 ]] \
    || { echo "  FAIL: un-allowlisted ref expected exit 1, got $rc: $out"; fails=$((fails + 1)); }
grep -qF -- "$EXT" <<<"$out" \
    || { echo "  FAIL: red output does not name the offending ref '$EXT': $out"; fails=$((fails + 1)); }

# --- the same link, allowlisted by the knob, clears --------------------------
run "$tmp/empty-config.sh" "$tmp/theme.sh" "https://third-party.example"
[[ "$rc" -eq 0 ]] \
    || { echo "  FAIL: knob-allowlisted ref expected exit 0, got $rc: $out"; fails=$((fails + 1)); }

# --- the seeded mermaid import alone is allowed under an empty knob -----------
run "$tmp/empty-config.sh" "$tmp/absent.sh" ""
[[ "$rc" -eq 0 ]] \
    || { echo "  FAIL: seeded-mermaid-only emit expected exit 0, got $rc: $out"; fails=$((fails + 1)); }

if [[ "$fails" -gt 0 ]]; then
    echo "check-graph-refs.test: $fails assertion(s) failed"
    exit 1
fi
echo "check-graph-refs.test: ok (un-allowlisted ref reds; knob clears; seeded mermaid always allowed)"
exit 0
