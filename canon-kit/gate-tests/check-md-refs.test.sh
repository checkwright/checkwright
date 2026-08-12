#!/usr/bin/env bash
# Two behaviors the one-pair good/bad harness cannot hold, both of this gate's
# and both riding here because run-gate-tests resolves exactly one good/ and one
# bad/ case dir per gate and this gate's pair is spent on link resolution.
#
# 1. The self-repo blob-link pass. The pair runs each case with `cd casedir`
#    inside *this* repo, so its origin is fixed and its self-repo links would
#    break for a consumer whose origin differs. These cases build throwaway git
#    repos with a controlled origin, so the origin-derived identity, the
#    git@/https normalization, and the no-origin skip are exercised hermetically.
#
# 2. The vendored-kit-root prune on the manifest set's README half. Its two
#    cases differ only in CANON_KIT_SCAN_KIT_ROOTS over the *same* tree, which
#    the pair's one-tree-per-verdict shape cannot express even with a slot free.
#    This is the prune's only executable oracle: this repo sets the knob so its
#    battery is a no-op on it, and a consumer's knob is 0 either way so no
#    consumer-smoke run tells a knob-gated prune from an unconditional one.
#    check-spec-dod-singleton.test.sh carries the same prune's canonical-spec
#    half; these cases are its mirror on the README finder.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # canon-kit/

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
    out="$(cd "$sb" && gate_env "$@" && gate_run check-md-refs "$DIR/checks" doc.md 2>&1)"; rc=$?
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

# A consumer whose own README links a doc it ships, beside a vendored kit root
# whose README carries a link that dangles on this tree — canon-kit's own
# `../queue-kit/` link on a profile that vendors canon-kit without queue-kit is
# the measured instance. The gate runs with no file arguments, so the manifest
# set finder is what decides whether that README is scanned at all.
PRUNE_SB="$(mktemp -d)"
trap 'rm -rf "$PRUNE_SB"' EXIT
mkdir -p "$PRUNE_SB/docs" "$PRUNE_SB/vendored-kit"
git -C "$PRUNE_SB" init -q
git -C "$PRUNE_SB" config user.email t@example.com
git -C "$PRUNE_SB" config user.name test
printf '# consumer\n\nRead [the guide](docs/guide.md).\n' >"$PRUNE_SB/README.md"
printf '# guide\n\nProse.\n' >"$PRUNE_SB/docs/guide.md"
printf '# vendored-kit\n\nSee [the queue kit](../queue-kit/README.md).\n' >"$PRUNE_SB/vendored-kit/README.md"
git -C "$PRUNE_SB" add -A

prune_case() {  # $1=label  $2=want-rc  $3=want-substring  $4..=env assignments
    local label="$1" want="$2" sub="$3"; shift 3
    local out rc
    out="$(cd "$PRUNE_SB" && gate_env "$@" && gate_run check-md-refs "$DIR/checks" 2>&1)"; rc=$?
    if [[ "$rc" -ne "$want" ]]; then
        echo "  FAIL [$label]: want exit $want, got $rc -- $out"; fails=$((fails + 1)); return
    fi
    if [[ -n "$sub" ]] && ! grep -qF -- "$sub" <<<"$out"; then
        echo "  FAIL [$label]: exit $rc OK but output lacks '$sub':"; printf '    %s\n' "$out"
        fails=$((fails + 1))
    fi
}

# Default (CANON_KIT_SCAN_KIT_ROOTS unset ⇒ 0): the kit root's README is out of
# the manifest set, so the consumer answers for its own doc and nothing else.
# The doc count is the assertion — a clean exit alone would also be produced by
# a corpus that scanned the kit README and happened to resolve its link.
prune_case "prune-default" 0 "clean (1 doc(s)" GATE_SDK_KIT_DIRS=vendored-kit

# Knob on: the kit README is first-party content again, so its dangling link is
# the consumer's to answer for. A prune applied unconditionally — the plausible
# wrong implementation — goes clean here where a red is expected.
prune_case "scan-kit-roots" 1 "vendored-kit/README.md" \
    GATE_SDK_KIT_DIRS=vendored-kit CANON_KIT_SCAN_KIT_ROOTS=1

if [[ "$fails" -gt 0 ]]; then
    echo "check-md-refs.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-md-refs.test.sh: clean (self-repo pass: git@/https identity, dangle, bad anchor, no-origin skip, ref knob; README-finder kit-root prune: default + knob re-include — 8 cases)"
exit 0
