#!/usr/bin/env bash
# Behavioral test of check-reads-couples' .gate arm — the refusal the one good/bad
# pair cannot hold, because the pair models exit 0 and exit 1 and this arm is exit 2
# by the fail-closed contract. With no .gate member registered anywhere in the tree
# the arm has no live instance, so without this file it would be an untested branch
# rather than the counted zero the clean line reports.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../lib/test-hermetic.sh"

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
GATE="$ROOT/gate-sdk/checks/check-reads-couples.sh"

fails=0
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

# $1=label  $2=descriptor body  $3=want-rc  $4=want-substring
check_case() {
    local label="$1" body="$2" want="$3" substr="$4" dir out rc
    dir="$tmp/$label"
    mkdir -p "$dir"
    printf '%s\n' "$body" > "$dir/sandbox.gate"
    out="$( cd "$dir" && "$GATE" sandbox.gate 2>&1 )"; rc=$?
    if [[ "$rc" -ne "$want" ]]; then
        echo "  FAIL [$label]: want exit $want, got $rc -- $out"; fails=$((fails + 1)); return
    fi
    if [[ -n "$substr" ]] && ! grep -qF -- "$substr" <<<"$out"; then
        echo "  FAIL [$label]: exit $rc OK but output lacks '$substr': $out"; fails=$((fails + 1))
    fi
}

# A — the bare descriptor. Zero walks are findable in it, so a passing run would
# report clean over a corpus the parser never saw.
check_case bare \
    '# graph: couples=corpus/*.md dir=one valve=none tier=precommit' \
    2 'whose walks this'

# B — a descriptor claiming an exemption. The arm has no opt-out by design: the
# disposition table records this gate as retained and failing closed until a
# binary-side equivalent exists, and a port able to write its way past that in one
# sentence would end the assertion instead of replacing it.
check_case claimed_exemption \
    '# reads-couples-exempt: the walks are covered elsewhere' \
    2 'no descriptor-level exemption'

# C — the shell arm is unaffected: a .sh source with a covered walk still analyzes.
mkdir -p "$tmp/shell/corpus"
: > "$tmp/shell/corpus/a.md"
printf '%s\n' \
    '# graph: couples=corpus/*.md dir=one valve=none tier=precommit' \
    'gate_find "corpus" -name '"'"'*.md'"'"' -type f' > "$tmp/shell/sandbox-gate.sh"
( cd "$tmp/shell" && git init -q . && git add -A ) >/dev/null 2>&1
out="$( cd "$tmp/shell" && "$GATE" sandbox-gate.sh 2>&1 )"; rc=$?
if [[ "$rc" -ne 0 ]]; then
    echo "  FAIL [shell-arm]: want exit 0, got $rc -- $out"; fails=$((fails + 1))
elif ! grep -qF -- 'READS-COUPLES: clean' <<<"$out"; then
    echo "  FAIL [shell-arm]: exit 0 but not clean: $out"; fails=$((fails + 1))
fi

if [[ "$fails" -gt 0 ]]; then
    echo "check-reads-couples.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-reads-couples.test.sh: clean (3 cases)"
exit 0
