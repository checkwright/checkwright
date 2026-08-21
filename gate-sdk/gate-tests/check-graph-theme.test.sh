#!/usr/bin/env bash
# Direct unit test of check-graph's theme seam (GATE_SDK_GRAPH_THEME_DIR and its
# three verbatim part files) — the good/bad pair stays the themeless case
# (--amend-only), so the emit-side theme seam needs its own test: a theme's
# injected markers provably land in the emission, an absent theme falls back
# byte-identically (no markers), both paths are deterministic (the byte-compare
# assertion E relies on it), the kit body survives theming, and the RETIRED
# sourced-function seam refuses loudly rather than silently dropping a theme.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # gate-sdk/
CHECKS="$DIR/checks"

fails=0
tmp="$(mktemp -d)"; trap 'rm -rf "$tmp"' EXIT

CSS_MARKER='THEME-CSS-MARKER-Xyzzy1'
HDR_MARKER='theme-header-marker-Xyzzy2'
FTR_MARKER='theme-footer-marker-Xyzzy3'

# the part-file contract: at most three optional files, each inlined byte verbatim
mkdir -p "$tmp/theme"
printf '    /* %s */\n' "$CSS_MARKER" > "$tmp/theme/theme.css"
printf '  <div id="%s"></div>\n' "$HDR_MARKER" > "$tmp/theme/header.html"
printf '  <div id="%s"></div>\n' "$FTR_MARKER" > "$tmp/theme/footer.html"

# spec: gate-sdk/SPEC.md §The non-gate arm — the emitter is a binary arm, so it is reached through
# the front-end that resolves its bridged knobs, never as a mode of the gate
emit() {  # emit() <theme-dir> -> the emission under that theme directory
    gate_env GATE_SDK_GRAPH_THEME_DIR="$1" && bash "$DIR/bin/run-gates.sh" --emit graph
}

themed="$(emit "$tmp/theme")"
bare="$(emit "$tmp/absent")"

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
themed2="$(emit "$tmp/theme")"
[[ "$themed" == "$themed2" ]] \
    || { echo "  FAIL: themed --emit is not byte-deterministic across runs"; fails=$((fails + 1)); }
bare2="$(emit "$tmp/absent")"
[[ "$bare" == "$bare2" ]] \
    || { echo "  FAIL: themeless --emit is not byte-deterministic across runs"; fails=$((fails + 1)); }

# --- the mermaid graph body survives theming (a self-loop edge, kit chrome) ----
for anchor in 'graph LR' 'class="mermaid viewport"'; do
    grep -qF -- "$anchor" <<<"$themed" \
        || { echo "  FAIL: themed --emit dropped kit body anchor '$anchor'"; fails=$((fails + 1)); }
done

# --- the retired sourced-function seam refuses, naming the migration ----------
# A themed consumer that silently lost its theme would produce an artifact the
# byte-compare cannot tell from a legitimate theme edit, so the failure would be
# invisible in a green battery — what the tripwire exists to refuse.
: > "$tmp/graph-theme.sh"
out="$(gate_env GATE_SDK_GRAPH_THEME="$tmp/graph-theme.sh" \
       && gate_run check-graph "$CHECKS" 2>&1)"; rc=$?
[[ "$rc" -eq 2 ]] \
    || { echo "  FAIL: the retired GATE_SDK_GRAPH_THEME seam expected exit 2, got $rc: $out"; fails=$((fails + 1)); }
grep -qF 'GATE_SDK_GRAPH_THEME_DIR' <<<"$out" \
    || { echo "  FAIL: the retired-seam refusal does not name the migration: $out"; fails=$((fails + 1)); }

# the same refusal from the file's mere presence in the gates dir, no env set
sb="$tmp/sandbox"; mkdir -p "$sb/scripts"
: > "$sb/scripts/graph-theme.sh"
out="$( cd "$sb" && gate_run check-graph "$CHECKS" 2>&1 )"; rc=$?
[[ "$rc" -eq 2 ]] \
    || { echo "  FAIL: a stale <gates-dir>/graph-theme.sh expected exit 2, got $rc: $out"; fails=$((fails + 1)); }

if [[ "$fails" -gt 0 ]]; then
    echo "check-graph-theme.test: $fails assertion(s) failed"
    exit 1
fi
echo "check-graph-theme.test: ok (part files land; absent-theme fallback; determinism; body preserved; retired seam refuses)"
exit 0
