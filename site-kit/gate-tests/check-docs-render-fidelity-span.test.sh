#!/usr/bin/env bash
# Direct unit test of check-docs-render-fidelity's span-corruption assertion
# (assertion 1), isolated from every other cause. The good/bad pair carries the
# severed span alongside the fence/heading case, so on that page the fence run
# alone would red the gate — which cannot distinguish a real widening from a
# vacuous one. These pages carry NO fence, NO surplus heading, and NO table: the
# only defect is a single-backtick span whose angle-bracket placeholder kramdown
# consumes as raw HTML (gettalong/kramdown#843), severing the span so it leaks a
# stray backtick AND a raw non-HTML-element tag. The pre-widening assertion
# matched only a `{3,} run and was blind to exactly this shape, so a red here is
# proof the widened assertion carries the class on its own. The doubled-backtick
# form of the same prose clears. The gate enumerates tracked pages via
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

cat > "$tmp/docs/leak.md" <<'MD'
---
title: Leak
---

# Heading

The consequence clause appends after the ` -> <verdict>` arrow, leaving it
disjoint from the invariant below.
MD

cat > "$tmp/docs/ok.md" <<'MD'
---
title: Ok
---

# Heading

The consequence clause appends after the `` -> <verdict> `` arrow, leaving it
disjoint from the invariant below.
MD

# --- the bad page: the severed span reds the span assertion only ---------------
git -C "$tmp" add docs/leak.md
out="$( cd "$tmp" && "$GATE" docs 2>&1 )"; rc=$?
[[ "$rc" -eq 1 ]] \
    || { echo "  FAIL: severed span expected exit 1, got $rc: $out"; fails=$((fails + 1)); }
grep -qF -- "leaked into rendered text" <<<"$out" \
    || { echo "  FAIL: red output lacks the span finding: $out"; fails=$((fails + 1)); }
grep -qF -- "fall short of" <<<"$out" \
    && { echo "  FAIL: span case unexpectedly reds the table assertion: $out"; fails=$((fails + 1)); }
grep -qF -- "rendered heading(s) exceed" <<<"$out" \
    && { echo "  FAIL: span case unexpectedly reds the heading assertion: $out"; fails=$((fails + 1)); }

# --- the good page: the doubled-backtick form renders faithfully, clean --------
git -C "$tmp" rm -q --cached docs/leak.md
rm -f "$tmp/docs/leak.md"
git -C "$tmp" add docs/ok.md
out="$( cd "$tmp" && "$GATE" docs 2>&1 )"; rc=$?
[[ "$rc" -eq 0 ]] \
    || { echo "  FAIL: faithful doubled-backtick span expected exit 0, got $rc: $out"; fails=$((fails + 1)); }

if [[ "$fails" -gt 0 ]]; then
    echo "check-docs-render-fidelity-span.test: $fails assertion(s) failed"
    exit 1
fi
echo "check-docs-render-fidelity-span.test: ok (severed span reds span-only; doubled-backtick form clears)"
exit 0
