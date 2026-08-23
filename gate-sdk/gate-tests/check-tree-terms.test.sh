#!/usr/bin/env bash
# Behavioral test of check-tree-terms over the arms the good/+bad/ pair cannot
# reach. A case dir is not its own repository — `git ls-files` inside one returns
# the outer repo's index scoped to that subdir — so the non-repository arm is
# structurally unreachable from a case, and both cases pass a positional
# pattern-file, which short-circuits the whole env-knob resolution path. The
# fourth arm pins what a pair cannot: each expect.txt line is an independent
# substring assertion, so record *order* is assertable only here.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # gate-sdk/
CHECKS="$DIR/checks"
CASES="$DIR/gate-tests/check-tree-terms"
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0

expect() {  # expect <label> <want-rc> <substring> <got-rc> <output>
    if [[ "$4" -ne "$2" ]]; then
        echo "  FAIL [$1]: want exit $2, got $4 -- $5"; fails=$((fails + 1))
    elif ! grep -qF -- "$3" <<<"$5"; then
        echo "  FAIL [$1]: exit $2 but output lacks '$3': $5"; fails=$((fails + 1))
    fi
}

# --- a non-repository cwd: fail-closed, never a clean report over zero files ---
mkdir -p "$SANDBOX/outside"
printf 'a-pattern\n' > "$SANDBOX/outside/patterns.list"
out="$( cd "$SANDBOX/outside" && gate_run check-tree-terms "$CHECKS" . patterns.list 2>&1 )"; rc=$?
expect non-repository 2 'not a git repository' "$rc" "$out"

# --- the env-knob resolution path, reached only with no positional pattern-file ---
mkdir -p "$SANDBOX/repo"
git -C "$SANDBOX/repo" init -q
printf 'nothing banned here\n' > "$SANDBOX/repo/a.txt"
git -C "$SANDBOX/repo" add a.txt
out="$( cd "$SANDBOX/repo" \
    && gate_env GATE_SDK_MSG_PATTERN_FILES="$SANDBOX/repo/absent.list" \
    && gate_run check-tree-terms "$CHECKS" . 2>&1 )"; rc=$?
expect missing-pattern-file 2 'required tracked pattern file missing' "$rc" "$out"

# --- an empty pattern set leaves the tree unchecked: clean, not fail-closed.
# The fail-closed obligation is on a missing file, never on an empty one.
printf '# only a comment\n\n' > "$SANDBOX/repo/empty.list"
out="$( cd "$SANDBOX/repo" \
    && gate_env GATE_SDK_MSG_PATTERN_FILES="$SANDBOX/repo/empty.list" \
    && gate_run check-tree-terms "$CHECKS" . 2>&1 )"; rc=$?
expect empty-pattern-set 0 '0 banned pattern(s) configured; tree unchecked' "$rc" "$out"

# --- multi-record order over the tracked bad/ case: path order, then line order,
# one record per matching *line* however many patterns hit it, and no dedup of
# two identical lines. Read off the pair so the corpus has one home.
out="$( cd "$CASES/bad" && gate_run check-tree-terms "$CHECKS" tree patterns.list 2>&1 )"; rc=$?
got="$(grep -E '^tree/' <<<"$out")"
want="$(printf '%s\n' \
    'tree/leak.txt:1:' \
    'tree/multi.txt:1:' \
    'tree/multi.txt:3:' \
    'tree/multi.txt:4:' \
    'tree/second-leak.txt:2:')"
if [[ "$rc" -ne 1 ]]; then
    echo "  FAIL [record-order]: want exit 1, got $rc -- $out"; fails=$((fails + 1))
elif [[ "$(sed 's/^\([^:]*:[0-9]*:\).*/\1/' <<<"$got")" != "$want" ]]; then
    echo "  FAIL [record-order]: record sequence is not path-then-line order:"
    printf '    %s\n' "$got"
    fails=$((fails + 1))
fi

if [[ "$fails" -gt 0 ]]; then
    echo "check-tree-terms.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-tree-terms.test.sh: clean (the arms a case dir cannot reach: a non-repository cwd, the env-knob resolution path through a missing required pattern file, an empty pattern set leaving the tree unchecked, and the bad pair's record order — 4 assertions over 4 cases)"
exit 0
