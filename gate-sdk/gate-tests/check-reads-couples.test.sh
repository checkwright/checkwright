#!/usr/bin/env bash
# Behavioral test of check-reads-couples' .gate arm — the consumption path and the two
# refusals that survive it. The one good/bad pair cannot hold this arm: the pair models
# exit 0 and exit 1 against a shell source, while this arm needs a binary to answer
# `--reads` and its refusals are exit 2 by the fail-closed contract. With no .gate member
# registered anywhere in the tree the arm has no live instance, so without this file it
# would be an untested branch rather than the counted zero the clean line reports.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../lib/test-hermetic.sh"

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
GATE="$ROOT/gate-sdk/checks/check-reads-couples.sh"
REAL_BIN="$ROOT/native/target/release/checkwright-gates"

fails=0
cases=0
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

MANIFEST='# graph: couples=corpus/*.md dir=one valve=none tier=precommit'

# Writes a stub binary that answers `--reads` with the given lines and refuses anything
# else — the shape the real binary has for a subcommand it carries.
make_stub() {
    local path="$1" r; shift
    {
        echo '#!/usr/bin/env bash'
        echo '[[ "${1:-}" == --reads ]] || exit 2'
        for r in "$@"; do printf "echo '%s'\n" "$r"; done
        echo 'exit 0'
    } > "$path"
    chmod +x "$path"
}

# $1=label $2=want-rc $3=want-substring $4=binary-path  (case dir prepared by the caller)
run_case() {
    local label="$1" want="$2" substr="$3" bin="$4" out rc
    cases=$((cases + 1))
    out="$( cd "$tmp/$label" && GATE_SDK_NATIVE_BIN="$bin" "$GATE" sandbox.gate 2>&1 )"; rc=$?
    if [[ "$rc" -ne "$want" ]]; then
        echo "  FAIL [$label]: want exit $want, got $rc -- $out"; fails=$((fails + 1)); return
    fi
    if [[ -n "$substr" ]] && ! grep -qF -- "$substr" <<<"$out"; then
        echo "  FAIL [$label]: exit $rc OK but output lacks '$substr': $out"; fails=$((fails + 1))
    fi
}

# Builds a case dir holding a descriptor, a tracked corpus, and a stub reporting $3...
make_case() {
    local label="$1" descriptor="$2"; shift 2
    local dir="$tmp/$label"
    mkdir -p "$dir/corpus/sub"
    printf '%s\n' "$descriptor" > "$dir/sandbox.gate"
    : > "$dir/corpus/top.md"
    : > "$dir/corpus/sub/deep.md"
    make_stub "$dir/stub-bin" "$@"
    ( cd "$dir" && git init -q . && git add -A ) >/dev/null 2>&1
}

# A — the reported root's tracked reads are covered. A couple naming both levels covers a
# recursive walk; this is the clean consumption path the refusal used to make unreachable.
make_case covered \
    '# graph: couples=corpus/*.md,corpus/sub/*.md dir=one valve=none tier=precommit' corpus
run_case covered 0 '1 resolvable walk(s) covered' "$tmp/covered/stub-bin"

# B — the same reported root, a couple that stops one level short. Globs never cross '/',
# so the deeper tracked file is uncovered and the finding must name it.
make_case uncovered "$MANIFEST" corpus
run_case uncovered 1 "corpus/sub/deep.md" "$tmp/uncovered/stub-bin"

# C — a root the gate cannot bound statically. It is counted, never assumed empty: the
# clean line reports it in the same skip counter the shell arm's unresolvable roots use.
make_case unbounded "$MANIFEST" '?'
run_case unbounded 0 '1 undecidable walk(s) skipped-and-counted' "$tmp/unbounded/stub-bin"

# D — a descriptor claiming the removed opt-out. There is deliberately no descriptor-level
# exemption, so the uncovered read is still a finding: the line buys nothing.
make_case claimed_exemption \
    "$MANIFEST
# reads-couples-exempt: the walks are covered elsewhere" corpus
run_case claimed_exemption 1 "corpus/sub/deep.md" "$tmp/claimed_exemption/stub-bin"

# E — surviving refusal one: a .gate member registered with no binary to ask. Exit 2, the
# same fail-closed shape check-gate-substrate-parity assertion B uses for this condition.
make_case no_binary "$MANIFEST" corpus
run_case no_binary 2 'absent' "$tmp/no_binary/nonexistent-bin"

# F — surviving refusal two: the binary is there but cannot answer for this member. A
# non-zero --reads must never read as "reads nothing".
make_case reads_fails "$MANIFEST" corpus
{ echo '#!/usr/bin/env bash'; echo 'echo "no such gate subcommand" >&2'; echo 'exit 3'; } \
    > "$tmp/reads_fails/broken-bin"
chmod +x "$tmp/reads_fails/broken-bin"
run_case reads_fails 2 'exited 3' "$tmp/reads_fails/broken-bin"

# G — the shell arm is unaffected: a .sh source with a covered walk still analyzes, so a
# refusal above cannot be passing for a parse failure.
mkdir -p "$tmp/shell/corpus"
: > "$tmp/shell/corpus/a.md"
printf '%s\n' \
    "$MANIFEST" \
    'gate_find "corpus" -name '"'"'*.md'"'"' -type f' > "$tmp/shell/sandbox-gate.sh"
( cd "$tmp/shell" && git init -q . && git add -A ) >/dev/null 2>&1
cases=$((cases + 1))
out="$( cd "$tmp/shell" && "$GATE" sandbox-gate.sh 2>&1 )"; rc=$?
if [[ "$rc" -ne 0 ]]; then
    echo "  FAIL [shell-arm]: want exit 0, got $rc -- $out"; fails=$((fails + 1))
elif ! grep -qF -- 'READS-COUPLES: clean' <<<"$out"; then
    echo "  FAIL [shell-arm]: exit 0 but not clean: $out"; fails=$((fails + 1))
fi

# H — the same consumption path against the *real* binary rather than a stub, so the
# reported grammar is the one the substrate actually emits. Skipped-and-named when the
# binary is absent: this file is hermetic and never builds, and a consumer tree has no
# crate to build from.
real_status="skipped (binary not built)"
if [[ -x "$REAL_BIN" ]]; then
    mkdir -p "$tmp/real"
    printf '%s\n' "$MANIFEST" > "$tmp/real/check-action-pinning.gate"
    cases=$((cases + 1))
    out="$( cd "$tmp/real" && GATE_SDK_NATIVE_BIN="$REAL_BIN" \
        "$GATE" check-action-pinning.gate 2>&1 )"; rc=$?
    real_status="run"
    if [[ "$rc" -ne 0 ]]; then
        echo "  FAIL [real-binary]: want exit 0, got $rc -- $out"; fails=$((fails + 1))
    elif ! grep -qF -- '1 undecidable walk(s) skipped-and-counted' <<<"$out"; then
        echo "  FAIL [real-binary]: exit 0 but the reported '?' was not counted: $out"
        fails=$((fails + 1))
    fi
fi

if [[ "$fails" -gt 0 ]]; then
    echo "check-reads-couples.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-reads-couples.test.sh: clean ($cases cases; real-binary case $real_status)"
exit 0
