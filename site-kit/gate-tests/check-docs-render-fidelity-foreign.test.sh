#!/usr/bin/env bash
# Direct unit test of check-docs-render-fidelity's foreign-content exemption
# within the span-corruption assertion (assertion 1b). The known-HTML-element
# set omitted the SVG and MathML roots, so a page carrying legitimate inline
# <svg> red the gate — a false positive on correct markup. Adding the roots
# alone does not clear it: the child vocabularies (<circle>, <mi>) are not HTML
# element names either, so the exemption is scoped to the subtree by open/close
# depth instead of granted to a list of names.
#
# The second half is what makes the first non-vacuous: a bare <path> outside any
# <svg> must still red. A blanket widening (dumping the SVG vocabulary into the
# allowlist) would pass the good page and silently lose exactly the placeholder
# tokens — <path>, <use>, <set>, <text> — this gate exists to catch. Both halves
# together prove the exemption narrowed the false-positive surface without
# weakening the true assertion. The gate enumerates tracked pages via
# git ls-files, so the fixture is a throwaway git repo.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # site-kit/
GATE="$DIR/checks/check-docs-render-fidelity.sh"

fails=0
tmp="$(mktemp -d)"; trap 'rm -rf "$tmp"' EXIT

(
    cd "$tmp" || exit 1
    git init -q
    git config user.email fixture@example.invalid
    git config user.name fixture
    mkdir -p docs
) || { echo "  FAIL: could not init fixture repo"; exit 1; }

# Legitimate inline foreign content: an SVG subtree whose children are SVG
# vocabulary, and a MathML subtree. Each root is kept on one line — kramdown
# severs a foreign subtree split across a line break, which is a genuine render
# divergence this gate is right to red, so putting one here would test the
# renderer's bug rather than the exemption. Two <svg> roots in sequence exercise
# the depth tracking closing back to zero between them.
cat > "$tmp/docs/ok.md" <<'MD'
---
title: Ok
---

# Heading

A diagram sits inline: <svg width="20"><circle cx="10" r="8" /><path d="M0 0" /></svg> and
a second <svg width="20"><g><rect width="4" /></g></svg> follows, then a formula
<math><mrow><mi>x</mi><mo>+</mo><mn>1</mn></mrow></math> ends the paragraph.
MD

# A placeholder token of a name that IS SVG vocabulary, but outside any <svg>.
cat > "$tmp/docs/leak.md" <<'MD'
---
title: Leak
---

# Heading

The rule appends after the ` -> <path>` arrow, leaving it disjoint from the
invariant below.
MD

# --- the good page: legitimate inline svg/math clears --------------------------
git -C "$tmp" add docs/ok.md
out="$( cd "$tmp" && "$GATE" docs 2>&1 )"; rc=$?
[[ "$rc" -eq 0 ]] \
    || { echo "  FAIL: legitimate inline svg/math expected exit 0, got $rc: $out"; fails=$((fails + 1)); }

# --- the bad page: a bare <path> outside <svg> still reds ----------------------
git -C "$tmp" rm -q --cached docs/ok.md
rm -f "$tmp/docs/ok.md"
git -C "$tmp" add docs/leak.md
out="$( cd "$tmp" && "$GATE" docs 2>&1 )"; rc=$?
[[ "$rc" -eq 1 ]] \
    || { echo "  FAIL: bare <path> placeholder expected exit 1, got $rc: $out"; fails=$((fails + 1)); }
grep -qF -- "leaked into rendered text" <<<"$out" \
    || { echo "  FAIL: red output lacks the span finding: $out"; fails=$((fails + 1)); }

if [[ "$fails" -gt 0 ]]; then
    echo "check-docs-render-fidelity-foreign.test: $fails assertion(s) failed"
    exit 1
fi
echo "check-docs-render-fidelity-foreign.test: ok (inline svg/math clears; bare <path> outside svg still reds)"
exit 0
