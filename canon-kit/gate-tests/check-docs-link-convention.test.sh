#!/usr/bin/env bash
# Behavioral test of the off-root rule in canon-kit/checks/check-docs-link-convention.sh.
# The good/bad pair grows a case per direction, but its single expect string
# cannot pin three rules at once, and the off-root rule's whole design is its
# boundary predicate — each edge below is a ruled edge, so each is tested rather
# than left to the implementer's reading.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
GATE="$ROOT/canon-kit/checks/check-docs-link-convention.sh"

fails=0
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

# $1=case dir  $2=body of docs/page.md
scaffold() {
    mkdir -p "$1/docs"
    printf '# Outside\n' >"$1/OUTSIDE.md"
    mkdir -p "$1/outsidedir"
    printf '# Inside\n' >"$1/docs/other.md"
    printf '%s\n' "$2" >"$1/docs/page.md"
}

# $1=label $2=dir $3=want-rc $4=want-substring
check_case() {
    local out rc
    out="$(cd "$2" && "$GATE" docs 2>&1)"; rc=$?
    if [[ "$rc" -ne "$3" ]]; then
        echo "  FAIL [$1]: want exit $3, got $rc -- $out"; fails=$((fails + 1)); return
    fi
    if [[ -n "$4" ]] && ! grep -qF -- "$4" <<<"$out"; then
        echo "  FAIL [$1]: exit $rc OK but output lacks '$4': $out"; fails=$((fails + 1))
    fi
}

# A — the violation itself: a relative link resolving to an existing file outside
# the root. This is what 404s on a site served from the root alone.
scaffold "$tmp/a" 'See [outside](../OUTSIDE.md).'
check_case "off-root-file-reds" "$tmp/a" 1 "off-root relative link"

# B — resolving back under the root is silent. Keying on the '../' text instead of
# the resolved path would red the majority of correct links in a real corpus.
scaffold "$tmp/b" 'See [other](../docs/other.md).'
check_case "resolves-back-under-root-silent" "$tmp/b" 0 "DOCS-LINK-CONVENTION: clean"

# C — the blob form the rule demands cannot itself red the rule: it is an
# absolute URL, already outside this gate's scope.
scaffold "$tmp/c" 'See [outside](https://github.com/owner/repo/blob/master/OUTSIDE.md).'
check_case "blob-form-silent" "$tmp/c" 0 "DOCS-LINK-CONVENTION: clean"

# D — a pure anchor has no path to resolve.
scaffold "$tmp/d" 'See [a section](#somewhere).'
check_case "pure-anchor-silent" "$tmp/d" 0 "DOCS-LINK-CONVENTION: clean"

# E — anti-double-report: a relative target that resolves to nothing is
# check-md-refs' finding and must stay only its finding.
scaffold "$tmp/e" 'See [nothing](../NOSUCHFILE.md).'
check_case "nonexistent-target-not-ours" "$tmp/e" 0 "DOCS-LINK-CONVENTION: clean"

# F — a directory target outside the root satisfies the first rule's predicate
# first, is reported there once, and this rule does not double-report it.
scaffold "$tmp/f" 'See [a dir](../outsidedir).'
check_case "off-root-directory-is-rule-one" "$tmp/f" 1 "directory-target link"
out_f="$(cd "$tmp/f" && "$GATE" docs 2>&1)"
if grep -qF -- "off-root relative link" <<<"$out_f"; then
    echo "  FAIL [off-root-directory-not-double-reported]: rule three also fired: $out_f"
    fails=$((fails + 1))
fi

# G — the existing valve suppresses this rule as it does the other two; no second
# valve and no rule-specific escape.
scaffold "$tmp/g" '<!-- docs-link-exempt: deliberate, for the test -->
See [outside](../OUTSIDE.md).'
check_case "existing-valve-suppresses" "$tmp/g" 0 "DOCS-LINK-CONVENTION: clean"

# H — a generated mirror page is in scope deliberately: its conformance is a
# property of the generator, and a generator can regress.
scaffold "$tmp/h" '---
generated: true
---

See [outside](../OUTSIDE.md).'
check_case "generated-page-in-scope" "$tmp/h" 1 "off-root relative link"

if [[ "$fails" -gt 0 ]]; then
    echo "check-docs-link-convention.test: $fails assertion(s) failed"
    exit 1
fi
echo "check-docs-link-convention.test: ok (an off-root relative link to an existing target reds; a ../ link resolving back under the root, the blob form, a pure anchor, and a target that resolves to nothing are all silent; an off-root directory target stays the directory rule's single finding; the existing docs-link-exempt valve suppresses it; a generated mirror page is in scope)"
exit 0
