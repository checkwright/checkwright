#!/usr/bin/env bash
# Behavioral test of check-portability-floor over the arms the good/+bad/ pair
# cannot reach. Both cases pass a positional pattern file and set a non-empty
# corpus, so the two disabled-by-absent-config cleans are unreachable from a
# case: a case with no corpus would assert nothing at all, and one with no
# pattern file could not be told from one whose patterns simply do not match.
# The fail-closed arms are unreachable for the same structural reason — a case
# ships readable files by construction, and an unreadable one could not be
# tracked in a state a fixture could carry across the payload's transport.
#
# The record-order arm pins what a pair cannot: each expect.txt line is an
# independent substring assertion, so record *order* is assertable only here.
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # gate-sdk/
CHECKS="$DIR/checks"
CASES="$DIR/gate-tests/check-portability-floor"
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

# --- an unconfigured corpus disables the assertion and SAYS SO. The clean line
# must distinguish *nothing configured* from *nothing found*: an adopter who
# deletes their roster silently disables the gate, and this sentence is the whole
# bound on that degradation.
mkdir -p "$SANDBOX/repo"
git -C "$SANDBOX/repo" init -q
printf 'newest() { printf "%%s\\n" "$@" | sort -V; }\n' > "$SANDBOX/repo/verb"
printf 'sort -V\n' > "$SANDBOX/repo/patterns.list"
git -C "$SANDBOX/repo" add verb
out="$( cd "$SANDBOX/repo" \
    && gate_env GATE_SDK_PORTABILITY_PATHS= \
    && gate_run check-portability-floor "$CHECKS" patterns.list 2>&1 )"; rc=$?
expect empty-corpus 0 'no install-path corpus configured' "$rc" "$out"
if grep -qF 'none uses one of the' <<<"$out"; then
    echo "  FAIL [empty-corpus]: the disabled verdict must not read as a nothing-found one -- $out"
    fails=$((fails + 1))
fi

# --- an ABSENT pattern file disables the assertion too, on the same ground and
# with its own sentence: the kit cannot know a consumer's install path, so a
# fail-closed default would red every adopter's first commit.
out="$( cd "$SANDBOX/repo" \
    && gate_env GATE_SDK_PORTABILITY_PATHS=. GATE_SDK_PORTABILITY_PATTERNS="$SANDBOX/repo/absent.list" \
    && gate_run check-portability-floor "$CHECKS" 2>&1 )"; rc=$?
expect absent-pattern-file 0 '0 banned construct(s) configured' "$rc" "$out"

# --- the corpus is live under the same configuration: the *same* knob path that
# disabled the gate above finds the violation when it is set, so the clean above
# is a disabled gate rather than a broken one.
out="$( cd "$SANDBOX/repo" \
    && gate_env GATE_SDK_PORTABILITY_PATHS=. \
    && gate_run check-portability-floor "$CHECKS" patterns.list 2>&1 )"; rc=$?
expect knob-corpus-live 1 'verb:1:' "$rc" "$out"

# --- a GNU escape in a portability blocklist is refused BY NAME at compile,
# which is exit 2 rather than a scan that quietly matched nothing. This is the
# joke the gate has to survive rather than make.
printf '\\bsort\\b -V\n' > "$SANDBOX/repo/gnu.list"
out="$( cd "$SANDBOX/repo" \
    && gate_env GATE_SDK_PORTABILITY_PATHS=. \
    && gate_run check-portability-floor "$CHECKS" gnu.list 2>&1 )"; rc=$?
expect gnu-escape-refused 2 '\b' "$rc" "$out"

# --- an unreadable pattern file fails CLOSED where an absent one degrades. The
# two are different facts: absence is how a consumer declines the gate,
# unreadability is a machine that cannot answer. The arm skips-and-declares under
# an account that can read anything (root, or a host with no POSIX modes), which
# is the honest shape for an assertion whose precondition is a platform
# capability.
unreadable_arm=skipped
printf 'sort -V\n' > "$SANDBOX/repo/locked.list"
chmod 000 "$SANDBOX/repo/locked.list" 2>/dev/null
if ! head -c1 "$SANDBOX/repo/locked.list" >/dev/null 2>&1; then
    unreadable_arm=run
    out="$( cd "$SANDBOX/repo" \
        && gate_env GATE_SDK_PORTABILITY_PATHS=. \
        && gate_run check-portability-floor "$CHECKS" locked.list 2>&1 )"; rc=$?
    expect unreadable-pattern-file 2 'pattern file not readable' "$rc" "$out"
else
    echo "  SKIP [unreadable-pattern-file]: chmod 000 left the file readable — this account or filesystem does not enforce POSIX modes, so the arm's precondition is absent and it declares the skip rather than reporting a failure."
fi
chmod 644 "$SANDBOX/repo/locked.list" 2>/dev/null

# --- a banned construct inside a tracked BINARY corpus member is skipped and
# COUNTED, never scanned: a construct is a spelling a shell runs, and a match
# inside a compiled artifact names no line an adopter's machine executes. Proved
# by greenness plus the count, so a regression that started scanning it reds.
printf 'ELF\x00\x01 newest() { sort -V; }\x00more\n' > "$SANDBOX/repo/artifact"
git -C "$SANDBOX/repo" add artifact
out="$( cd "$SANDBOX/repo" \
    && gate_env GATE_SDK_PORTABILITY_PATHS=artifact \
    && gate_run check-portability-floor "$CHECKS" patterns.list 2>&1 )"; rc=$?
expect binary-skipped 0 '1 binary member(s) skipped' "$rc" "$out"

# --- record order over the tracked bad/ case: path order, then line order. Read
# off the pair so the corpus has one home. The corpus knob is set here rather
# than left to the case's own config seam, because gate_run resolves the bridge
# from the library this file sourced at the repo root and never re-enters the
# case dir to source it again — the case's seam is what the --run-gate-tests arm
# reads, and this driver is not that arm.
out="$( cd "$CASES/bad" \
    && gate_env GATE_SDK_PORTABILITY_PATHS=tree \
    && gate_run check-portability-floor "$CHECKS" patterns.list 2>&1 )"; rc=$?
got="$(grep -E '^tree/' <<<"$out")"
want="$(printf '%s\n' \
    'tree/empty-reason-verb:5:' \
    'tree/too-far-verb:5:' \
    'tree/undeclared-verb:3:')"
if [[ "$rc" -ne 1 ]]; then
    echo "  FAIL [record-order]: want exit 1, got $rc -- $out"; fails=$((fails + 1))
elif [[ "$(sed 's/^\([^:]*:[0-9]*:\).*/\1/' <<<"$got")" != "$want" ]]; then
    echo "  FAIL [record-order]: record sequence is not path-then-line order:"
    printf '    %s\n' "$got"
    fails=$((fails + 1))
fi

if [[ "$fails" -gt 0 ]]; then
    echo "check-portability-floor.test.sh: $fails case(s) failed"
    exit 1
fi
if [[ "$unreadable_arm" == run ]]; then
    echo "check-portability-floor.test.sh: clean (the arms a case dir cannot reach: an unconfigured corpus disabling the assertion in its own sentence, an absent pattern file doing the same, the same knob path finding the violation once set, a GNU escape refused by name at compile, an unreadable pattern file failing closed where an absent one degrades, a binary corpus member skipped-and-counted, and the bad pair's record order — 9 assertions over 7 cases)"
else
    echo "check-portability-floor.test.sh: clean (the arms a case dir cannot reach: an unconfigured corpus disabling the assertion in its own sentence, an absent pattern file doing the same, the same knob path finding the violation once set, a GNU escape refused by name at compile, a binary corpus member skipped-and-counted, and the bad pair's record order — 7 assertions over 6 cases; the unreadable-pattern-file arm declared a skip above, this account reading a mode-000 file)"
fi
exit 0
