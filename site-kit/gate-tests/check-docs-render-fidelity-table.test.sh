#!/usr/bin/env bash
# Direct unit test of check-docs-render-fidelity's table-leakage assertion — the
# good/bad pair stays the fence/heading case (the real historical Install bug),
# so the table assertion needs its own case: a GFM table whose last row abuts a
# non-blank marker line (the value-page rollup incident, 2026-07-13) collapses to
# literal-pipe paragraph text and reds ONLY the table assertion (green on fence
# and heading, proving the new assertion carries the class the two missed); the
# same table with a trailing blank line renders and clears. The gate enumerates
# tracked pages via git ls-files, so the fixture is a throwaway git repo.
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

Intro paragraph.

| Kit | Cost |
| --- | --- |
| alpha | 12 |
<!-- rollup:end -->

Trailing paragraph.
MD

cat > "$tmp/docs/ok.md" <<'MD'
---
title: Ok
---

# Heading

Intro paragraph.

| Kit | Cost |
| --- | --- |
| alpha | 12 |

<!-- rollup:end -->

Trailing paragraph.
MD

# --- the bad page: the table collapses, red on the table assertion only --------
git -C "$tmp" add docs/leak.md
out="$( cd "$tmp" && "$GATE" docs 2>&1 )"; rc=$?
[[ "$rc" -eq 1 ]] \
    || { echo "  FAIL: collapsed table expected exit 1, got $rc: $out"; fails=$((fails + 1)); }
grep -qF -- "fall short of" <<<"$out" \
    || { echo "  FAIL: red output lacks the table finding: $out"; fails=$((fails + 1)); }
grep -qF -- "leaked into rendered text" <<<"$out" \
    && { echo "  FAIL: table case unexpectedly reds the fence assertion: $out"; fails=$((fails + 1)); }
grep -qF -- "rendered heading(s) exceed" <<<"$out" \
    && { echo "  FAIL: table case unexpectedly reds the heading assertion: $out"; fails=$((fails + 1)); }

# --- the good page: a trailing blank line renders the table, clean -------------
git -C "$tmp" rm -q --cached docs/leak.md
rm -f "$tmp/docs/leak.md"
git -C "$tmp" add docs/ok.md
out="$( cd "$tmp" && "$GATE" docs 2>&1 )"; rc=$?
[[ "$rc" -eq 0 ]] \
    || { echo "  FAIL: well-formed table expected exit 0, got $rc: $out"; fails=$((fails + 1)); }

if [[ "$fails" -gt 0 ]]; then
    echo "check-docs-render-fidelity-table.test: $fails assertion(s) failed"
    exit 1
fi
echo "check-docs-render-fidelity-table.test: ok (collapsed table reds table-only; trailing blank clears)"
exit 0
