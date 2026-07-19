#!/usr/bin/env bash
# Direct unit test of check-graph.sh's render-cap assertion (assertion I) via the
# hermetic --cap-only mode — the good/bad pair stays the themeless --amend-only
# case, so the emit-side cap needs its own test: an emitted graph with more edges
# than the page's declared maxEdges reds (Mermaid would paint an error graphic
# rather than the diagram), the cap set to exactly the edge count clears (Mermaid
# fails only when edges *exceed* the cap), and the GATE_SDK_GRAPH_MAX_EDGES knob
# drives the emitted cap. The boundary here mirrors Mermaid's own rule, verified
# against the renderer: edges==cap renders, edges>cap throws.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # gate-sdk/
GATE="$DIR/checks/check-graph.sh"

fails=0

# the emitted edge count the assertion measures against, from the real registry
edge_n="$(grep -cE '(<-->|-->)\|"' <<<"$("$GATE" --emit)")"
[[ "$edge_n" -gt 1 ]] \
    || { echo "  FAIL: expected the emitted graph to carry edges, counted $edge_n"; fails=$((fails + 1)); }

run() {  # run() <knob> -> sets rc/out
    out="$(GATE_SDK_GRAPH_MAX_EDGES="$1" "$GATE" --cap-only 2>&1)"; rc=$?
}

# --- a cap below the edge count reds, naming the count and the cap -------------
run "$((edge_n - 1))"
[[ "$rc" -eq 1 ]] \
    || { echo "  FAIL: cap below edge count expected exit 1, got $rc: $out"; fails=$((fails + 1)); }
grep -qF "$edge_n edges" <<<"$out" \
    || { echo "  FAIL: red output does not name the edge count '$edge_n': $out"; fails=$((fails + 1)); }

# --- cap == edge count clears (Mermaid fails only when edges exceed the cap) ---
run "$edge_n"
[[ "$rc" -eq 0 ]] \
    || { echo "  FAIL: cap equal to edge count expected exit 0, got $rc: $out"; fails=$((fails + 1)); }

# --- a generous cap clears -----------------------------------------------------
run "$((edge_n + 1000))"
[[ "$rc" -eq 0 ]] \
    || { echo "  FAIL: cap above edge count expected exit 0, got $rc: $out"; fails=$((fails + 1)); }

if [[ "$fails" -gt 0 ]]; then
    echo "check-graph-cap.test: $fails assertion(s) failed"
    exit 1
fi
echo "check-graph-cap.test: ok (over-cap reds; edges==cap clears; knob drives the emitted cap)"
exit 0
