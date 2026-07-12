#!/usr/bin/env bash
# Direct unit test of check-graph.sh's theme-injection seam (GATE_SDK_GRAPH_THEME
# and the graph_theme_css/header/footer overrides) — the good/bad pair stays the
# themeless case (--amend-only), so the emit-side theme seam needs its own test:
# a theme file's injected markers provably land in --emit, an absent theme falls
# back byte-identically (no markers), and both paths are deterministic (the
# byte-compare assertion E relies on).
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # gate-sdk/
GATE="$DIR/checks/check-graph.sh"

fails=0
tmp="$(mktemp -d)"; trap 'rm -rf "$tmp"' EXIT

CSS_MARKER='THEME-CSS-MARKER-Xyzzy1'
HDR_MARKER='theme-header-marker-Xyzzy2'
FTR_MARKER='theme-footer-marker-Xyzzy3'

cat > "$tmp/theme.sh" <<THEME
graph_theme_css()    { echo "    /* $CSS_MARKER */"; }
graph_theme_header() { echo "  <div id=\"$HDR_MARKER\"></div>"; }
graph_theme_footer() { echo "  <div id=\"$FTR_MARKER\"></div>"; }
THEME

themed="$(GATE_SDK_GRAPH_THEME="$tmp/theme.sh" "$GATE" --emit)"
bare="$(GATE_SDK_GRAPH_THEME="$tmp/absent.sh" "$GATE" --emit)"

# --- the injected markers provably land in the themed emission -----------------
for m in "$CSS_MARKER" "$HDR_MARKER" "$FTR_MARKER"; do
    grep -qF -- "$m" <<<"$themed" \
        || { echo "  FAIL: themed --emit lacks injected marker '$m'"; fails=$((fails + 1)); }
done

# --- an absent theme falls back to the kit default (no markers leak) -----------
for m in "$CSS_MARKER" "$HDR_MARKER" "$FTR_MARKER"; do
    grep -qF -- "$m" <<<"$bare" \
        && { echo "  FAIL: themeless --emit unexpectedly carries marker '$m'"; fails=$((fails + 1)); }
done

# --- both paths are deterministic (assertion E's byte-compare depends on it) ---
themed2="$(GATE_SDK_GRAPH_THEME="$tmp/theme.sh" "$GATE" --emit)"
[[ "$themed" == "$themed2" ]] \
    || { echo "  FAIL: themed --emit is not byte-deterministic across runs"; fails=$((fails + 1)); }
bare2="$(GATE_SDK_GRAPH_THEME="$tmp/absent.sh" "$GATE" --emit)"
[[ "$bare" == "$bare2" ]] \
    || { echo "  FAIL: themeless --emit is not byte-deterministic across runs"; fails=$((fails + 1)); }

# --- the mermaid graph body survives theming (a self-loop edge, kit chrome) ----
for anchor in 'graph LR' 'class="mermaid viewport"'; do
    grep -qF -- "$anchor" <<<"$themed" \
        || { echo "  FAIL: themed --emit dropped kit body anchor '$anchor'"; fails=$((fails + 1)); }
done

if [[ "$fails" -gt 0 ]]; then
    echo "check-graph-theme.test: $fails assertion(s) failed"
    exit 1
fi
echo "check-graph-theme.test: ok (markers land; absent-theme fallback; determinism; body preserved)"
exit 0
