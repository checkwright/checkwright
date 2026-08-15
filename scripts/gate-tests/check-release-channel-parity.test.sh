#!/usr/bin/env bash
# Behavioral test of check-release-channel-parity — the arms the one
# good/bad pair cannot hold on its own. The pair proves the agreeing state and a
# tree violating both invariants at once (its expect.txt pins invariant B, so
# deleting B fails the fixture). This isolates each invariant so neither can pass
# on the other's finding, exercises the real newest-tag read rather than the
# explicit-version seam the fixtures use, covers the no-tag dormancy arm, and
# covers every fail-closed refusal.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
GATES_DIR="$ROOT/scripts"

fails=0
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

# $1=dir  $2=channel-declaration-body  $3=prerelease? (yes/no)
write_case() {
    mkdir -p "$1"
    printf '## Versioning\n\n%s\n' "$2" >"$1/install.md"
    local flag=""
    [[ "$3" == "yes" ]] && flag=" --prerelease"
    cat >"$1/publish.yml" <<EOF
jobs:
  release:
    steps:
      - run: |
          gh release create "\$TAG" --title "\$TAG"$flag --notes ""
EOF
}

# $1=label $2=dir $3=version-arg (empty => resolve from git) $4=want-rc $5=want-substring
check_case() {
    local out rc
    if [[ -n "$3" ]]; then
        out="$(cd "$2" && gate_run check-release-channel-parity "$GATES_DIR" install.md publish.yml "$3" 2>&1)"; rc=$?
    else
        out="$(cd "$2" && gate_run check-release-channel-parity "$GATES_DIR" install.md publish.yml 2>&1)"; rc=$?
    fi
    if [[ "$rc" -ne "$4" ]]; then
        echo "  FAIL [$1]: want exit $4, got $rc -- $out"; fails=$((fails + 1)); return
    fi
    if [[ -n "$5" ]] && ! grep -qF -- "$5" <<<"$out"; then
        echo "  FAIL [$1]: exit $rc OK but output lacks '$5': $out"; fails=$((fails + 1))
    fi
}

# A alone — the channel is right for the version line, so B agrees and only the
# publish posture is wrong. A gate that shipped B alone would pass this.
write_case "$tmp/a-only" 'Release channel: **preview**' no
check_case "invariant-A-alone" "$tmp/a-only" 0.5.0 1 "invariant A: channel 'preview' demands --prerelease"
check_case "invariant-A-alone-not-B" "$tmp/a-only" 0.5.0 1 "invariant A"

# B alone — the publish posture agrees with the declaration, so A passes and only
# the version line disagrees. A gate simplified back to two surfaces passes this,
# which is exactly why the arm is tested in isolation.
write_case "$tmp/b-only" 'Release channel: **stable**' no
check_case "invariant-B-alone" "$tmp/b-only" 0.5.0 1 "invariant B: version line v0.5.0 is 0.x, which demands channel 'preview'"

# B's other direction — a 1.x line under a declared preview channel is the stale
# declaration that would hold lifecycle-kit's compat window open past GA.
write_case "$tmp/b-stale" 'Release channel: **preview**' yes
check_case "invariant-B-stale-preview" "$tmp/b-stale" 1.0.0 1 "invariant B: version line v1.0.0 is 1.x or later, which demands channel 'stable'"

# Both agreeing at the stable end of the line.
write_case "$tmp/stable-ok" 'Release channel: **stable**' no
check_case "stable-line-clean" "$tmp/stable-ok" 1.2.3 0 "RELEASE-CHANNEL-PARITY: clean"

# The leading v is accepted on the version line, since that is the spelling
# `git for-each-ref` hands back.
check_case "version-v-prefix" "$tmp/stable-ok" v1.2.3 0 "RELEASE-CHANNEL-PARITY: clean"

# --- the real newest-tag read, and the dormancy it can produce ----------------
# The fixtures pass an explicit version so they stay stable against this
# repository's tag line; these two cases exercise the default path instead.
mkgit() {
    mkdir -p "$1"
    git -C "$1" init -q 2>/dev/null
    git -C "$1" config user.email t@example.invalid
    git -C "$1" config user.name t
    : >"$1/seed"
    git -C "$1" add seed
    git -C "$1" commit -qm seed
}

# No tags at all: invariant B is dormant, A still asserts, and the clean line
# says so rather than letting dormancy read as verification.
mkgit "$tmp/notags"
write_case "$tmp/notags" 'Release channel: **preview**' yes
check_case "no-tags-B-dormant" "$tmp/notags" "" 0 "invariant B dormant"

# One tag: the default path resolves it and B asserts against it for real.
mkgit "$tmp/tagged"
write_case "$tmp/tagged" 'Release channel: **stable**' no
git -C "$tmp/tagged" tag -a v0.4.0 -m v0.4.0
check_case "newest-tag-read-red" "$tmp/tagged" "" 1 "invariant B: version line v0.4.0 is 0.x"

# --- fail-closed refusals: exit 2, never a pass ------------------------------
write_case "$tmp/nodecl" 'This section declares no channel at all.' yes
check_case "no-declaration" "$tmp/nodecl" 0.5.0 2 "carries no 'Release channel:' declaration line"

write_case "$tmp/dupdecl" 'Release channel: **preview**

Release channel: **stable**' yes
check_case "duplicate-declaration" "$tmp/dupdecl" 0.5.0 2 "declaration lines; exactly one is admissible"

write_case "$tmp/badval" 'Release channel: **beta**' yes
check_case "unrecognized-channel" "$tmp/badval" 0.5.0 2 "unrecognized channel value"

write_case "$tmp/nostep" 'Release channel: **preview**' yes
printf 'jobs:\n  release:\n    steps:\n      - run: echo nothing\n' >"$tmp/nostep/publish.yml"
check_case "no-release-step" "$tmp/nostep" 0.5.0 2 "no recognizable Release-creating step"

write_case "$tmp/badver" 'Release channel: **preview**' yes
check_case "unparseable-version" "$tmp/badver" not-a-version 2 "does not parse as semver"

if [[ "$fails" -gt 0 ]]; then
    echo "check-release-channel-parity.test: $fails assertion(s) failed"
    exit 1
fi
echo "check-release-channel-parity.test: ok (invariants A and B each red in isolation, B in both directions; the real newest-tag read asserts and reports dormancy when no tag exists; a missing, duplicated, or unrecognized declaration, an unrecognizable Release step, and an unparseable version each fail closed)"
exit 0
