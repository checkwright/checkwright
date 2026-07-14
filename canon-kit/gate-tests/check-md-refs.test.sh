#!/usr/bin/env bash
# Behavioral test of the self-repo blob-link pass the one-pair good/bad harness
# cannot hold: it runs each case with `cd casedir` inside *this* repo, so its
# origin is fixed and its self-repo links would break for a consumer whose
# origin differs. These cases build throwaway git repos with a controlled
# origin, so the origin-derived identity, the git@/https normalization, and the
# no-origin skip are exercised hermetically.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # canon-kit/
GATE="$DIR/checks/check-md-refs.sh"

fails=0

# Build a throwaway git repo with $1 as origin (empty ⇒ no origin) and a doc.md
# carrying the link line $2; target.md holds one "## Target Heading" section.
make_repo() {  # $1=origin-url  $2=doc-link-line -> echoes the sandbox path
    local origin="$1" link="$2" sb
    sb="$(mktemp -d)"
    git -C "$sb" init -q
    git -C "$sb" config user.email t@example.com
    git -C "$sb" config user.name test
    [[ -n "$origin" ]] && git -C "$sb" remote add origin "$origin"
    printf '# target\n\n## Target Heading\n' >"$sb/target.md"
    printf '# doc\n\n%s\n' "$link" >"$sb/doc.md"
    git -C "$sb" add -A
    printf '%s' "$sb"
}

check_case() {  # $1=label $2=origin $3=link $4=want-rc $5=want-substring $6..=env
    local label="$1" origin="$2" link="$3" want="$4" sub="$5"; shift 5
    local sb out rc
    sb="$(make_repo "$origin" "$link")"
    out="$(cd "$sb" && env "$@" "$GATE" doc.md 2>&1)"; rc=$?
    rm -rf "$sb"
    if [[ "$rc" -ne "$want" ]]; then
        echo "  FAIL [$label]: want exit $want, got $rc -- $out"; fails=$((fails + 1)); return
    fi
    if [[ -n "$sub" ]] && ! grep -qF -- "$sub" <<<"$out"; then
        echo "  FAIL [$label]: exit $rc OK but output lacks '$sub':"; printf '    %s\n' "$out"
        fails=$((fails + 1))
    fi
}

BLOB=https://github.com/acme/widget/blob/master

# git@ origin: a self-repo link to a tracked path + real anchor resolves.
check_case "gitat-good" "git@github.com:acme/widget.git" \
    "See [spec]($BLOB/target.md#target-heading)." 0 "1 self-repo reference link"

# https origin normalizes to the same identity — the same link resolves.
check_case "https-good" "https://github.com/acme/widget.git" \
    "See [spec]($BLOB/target.md#target-heading)." 0 "1 self-repo reference link"

# A self-repo link to a path no git tracks is a dangle, caught.
check_case "dangling-path" "git@github.com:acme/widget.git" \
    "See [spec]($BLOB/missing.md)." 1 "is not a git-tracked file"

# A tracked path with an anchor that slugs to no heading is caught.
check_case "bad-anchor" "git@github.com:acme/widget.git" \
    "See [spec]($BLOB/target.md#no-such-heading)." 1 "no heading in"

# No origin ⇒ the pass is skipped: the same dangling link is treated as an
# external URL and never resolved, so the doc is clean.
check_case "no-origin-skip" "" \
    "See [spec]($BLOB/missing.md)." 0 "0 self-repo reference link"

# The blob ref is a knob: a link on a different ref than the configured one is
# not a self-repo link (foreign identity), so it is skipped, not resolved.
check_case "ref-knob" "git@github.com:acme/widget.git" \
    "See [spec](https://github.com/acme/widget/blob/main/missing.md)." 0 \
    "0 self-repo reference link" CANON_KIT_DOCS_BLOB_REF=master

if [[ "$fails" -gt 0 ]]; then
    echo "check-md-refs.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-md-refs.test.sh: clean (self-repo pass: git@/https identity, dangle, bad anchor, no-origin skip, ref knob — 6 cases)"
exit 0
