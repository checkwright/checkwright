#!/usr/bin/env bash
# Behavioral test of check-path-names over the arms the good/+bad/ pair cannot
# reach: each expect.txt line is an independent substring assertion, so the
# absence of a record under a directory hit and the record order live here, as
# do a cwd outside any work tree and a configuration naming no live term.
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # gate-sdk/
CHECKS="$DIR/checks"
CASES="$DIR/gate-tests/check-path-names"
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

# --- outside a work tree there is no tracked path set to be clean about: refused at 2.
mkdir -p "$SANDBOX/plain"
printf 'foo\n' > "$SANDBOX/plain/terms.list"
out="$( cd "$SANDBOX/plain" && gate_env GATE_SDK_DENY_TERMS=terms.list \
    && gate_run check-path-names "$CHECKS" 2>&1 )"; rc=$?
expect non-repo 2 'not a git repository' "$rc" "$out"

# --- a list holding only comments and blanks configures no term: clean, not applicable.
mkdir -p "$SANDBOX/repo"
git -C "$SANDBOX/repo" init -q
printf '# rule: CODENAME\n\n' > "$SANDBOX/repo/terms.list"
out="$( cd "$SANDBOX/repo" && gate_env GATE_SDK_DENY_TERMS=terms.list \
    && gate_run check-path-names "$CHECKS" 2>&1 )"; rc=$?
expect no-term 0 'not applicable: no deny term configured' "$rc" "$out"

# --- the bad pair's full record set, in order: one record at the directory hit, none for
# the term-named file under it, and nothing for the tracked list itself.
out="$( cd "$CASES/bad" && gate_env GATE_SDK_DENY_TERMS=terms.list \
    && gate_run check-path-names "$CHECKS" 2>&1 )"; rc=$?
got="$(grep -E '^tree/' <<<"$out")"
want="$(printf '%s\n' \
    'tree/ZePhYr.md: [CODENAME]' \
    'tree/foo: [NAMES-DENIED]' \
    'tree/foo-cli.sh: [NAMES-DENIED]' \
    'tree/foo.rs: [NAMES-DENIED]' \
    'tree/foo_x.txt: [NAMES-DENIED]')"
if [[ "$rc" -ne 1 ]]; then
    echo "  FAIL [record-set]: want exit 1, got $rc -- $out"; fails=$((fails + 1))
elif [[ "$got" != "$want" ]]; then
    echo "  FAIL [record-set]: records are not the shortest-prefix set in path order:"
    printf '    %s\n' "$got"
    fails=$((fails + 1))
fi

if [[ "$fails" -gt 0 ]]; then
    echo "check-path-names.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-path-names.test.sh: clean (the arms a case dir cannot reach: a cwd outside a work tree, a list with no live term, and the bad pair's exact record set — 3 cases)"
exit 0
