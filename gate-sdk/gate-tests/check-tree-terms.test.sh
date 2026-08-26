#!/usr/bin/env bash
# Behavioral test of check-tree-terms over the arms the good/+bad/ pair cannot
# reach. A case dir is not its own repository — `git ls-files` inside one returns
# the outer repo's index scoped to that subdir — so the non-repository arm is
# structurally unreachable from a case, and both cases pass a positional
# pattern-file, which short-circuits the whole env-knob resolution path. The
# record-order arm pins what a pair cannot: each expect.txt line is an independent
# substring assertion, so record *order* is assertable only here.
#
# The dangling-symlink arm is here for a different reason, and stating it is what
# stops the next reader moving it back into good/tree/. A fixture SHIPS
# (gate-sdk/SPEC.md §Consumer payload), so it is payload content and bound by what
# the payload's transport can carry — and `tar` cannot create a dangling symlink
# on a native Windows host, so the tracked link aborted the vendor mid-kit. The
# link is constructed at run time here instead. It must stay DANGLING: the module
# filters with `is_file()`, which follows the link, so a resolvable one is scanned
# rather than skipped and the arm would assert the opposite of what it exists for.
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

# --- a banned shape inside a tracked binary: a path-only record, and still red.
# Dead on this tree (no tracked binaries) and live in a consumer's, which is why
# it is asserted here rather than discovered there; the fixture pair stays at the
# three-way split its own measurement fixed. The banned shape is composed, not
# spelled: this file's directory is pruned, but the composition is the same rule
# the module itself carries and reads better held in one place.
printf 'PNG\x00\x01 clone into /%s/bob/x and more\x00bytes\n' home > "$SANDBOX/repo/blob.bin"
printf '/(home|Users)/[A-Za-z0-9._-]+\n' > "$SANDBOX/repo/banned.list"
git -C "$SANDBOX/repo" add blob.bin
out="$( cd "$SANDBOX/repo" && gate_run check-tree-terms "$CHECKS" . banned.list 2>&1 )"; rc=$?
expect binary-record 1 'blob.bin' "$rc" "$out"
if grep -qE '^blob\.bin:[0-9]+:' <<<"$out"; then
    echo "  FAIL [binary-record]: a binary match must not carry :lineno:line -- $out"
    fails=$((fails + 1))
fi

# --- a tracked dangling symlink is skipped, not scanned, proved by GREENNESS:
# its own blob content is a banned shape, so a run that scanned it would be red.
# The arm skips-and-declares where `ln -s` fails, creating a symlink needing a
# privilege an ordinary Windows account may not hold — the honest shape for an
# assertion whose *precondition* is a platform capability.
link_arm=skipped
mkdir -p "$SANDBOX/linkrepo"
git -C "$SANDBOX/linkrepo" init -q
printf 'nothing banned here\n' > "$SANDBOX/linkrepo/plain.txt"
printf '/(home|Users)/[A-Za-z0-9._-]+\n' > "$SANDBOX/linkrepo/banned.list"
if ln -s '/home/nobody/absent' "$SANDBOX/linkrepo/dangling-link" 2>/dev/null; then
    link_arm=run
    git -C "$SANDBOX/linkrepo" add plain.txt dangling-link
    # the plant is worthless unless it is TRACKED and a symlink in the index: an
    # unstaged plant leaves the arm running over nothing while the case still passes
    if [[ "$(git -C "$SANDBOX/linkrepo" ls-files -s -- dangling-link | cut -d' ' -f1)" != "120000" ]]; then
        echo "  FAIL [dangling-symlink]: the plant is not a tracked symlink, so the arm asserts nothing"
        fails=$((fails + 1))
    fi
    out="$( cd "$SANDBOX/linkrepo" && gate_run check-tree-terms "$CHECKS" . banned.list 2>&1 )"; rc=$?
    expect dangling-symlink 0 '1 tracked file(s) scanned' "$rc" "$out"
else
    echo "  SKIP [dangling-symlink]: ln -s failed — creating a symlink needs a privilege this account may not hold; the arm's precondition is a platform capability, so it declares the skip rather than reporting a failure."
fi

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
if [[ "$link_arm" == run ]]; then
    echo "check-tree-terms.test.sh: clean (the arms a case dir cannot reach or may not ship: a non-repository cwd, the env-knob resolution path through a missing required pattern file, an empty pattern set leaving the tree unchecked, a banned shape inside a tracked binary yielding a path-only record, a tracked dangling symlink skipped rather than scanned, and the bad pair's record order — 8 assertions over 6 cases)"
else
    echo "check-tree-terms.test.sh: clean (the arms a case dir cannot reach: a non-repository cwd, the env-knob resolution path through a missing required pattern file, an empty pattern set leaving the tree unchecked, a banned shape inside a tracked binary yielding a path-only record, and the bad pair's record order — 6 assertions over 5 cases; the dangling-symlink arm declared a skip above, this host refusing ln -s)"
fi
exit 0
