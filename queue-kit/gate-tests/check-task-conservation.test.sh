#!/usr/bin/env bash
# spec: queue-kit/SPEC.md §check-task-conservation — the repository answer no case dir can carry: outside a repository there is no HEAD baseline and the gate is clean, while a repository git refuses, a nested one git skips included, exits 2 rather than reading as none.
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # queue-kit/
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0
expect() {  # $1=case $2=cwd $3=want-exit $4=want-line
    local out rc
    out="$(cd "$2" && gate_run check-task-conservation "$DIR/checks" 2>&1)"; rc=$?
    if [[ "$rc" -ne "$3" ]] || ! grep -qF -- "$4" <<<"$out"; then
        echo "  FAIL [$1]: want exit $3 carrying '$4', got $rc -- $out"; fails=$((fails + 1))
    fi
}

mkdir -p "$SANDBOX/none"
expect no-repository-clean "$SANDBOX/none" 0 "no git repository"

mkdir -p "$SANDBOX/refused"
printf 'gitdir: %s/absent\n' "$SANDBOX/refused" >"$SANDBOX/refused/.git"
expect refused-repository-exits-2 "$SANDBOX/refused" 2 "marks a repository"

git init -q "$SANDBOX/outer"
mkdir -p "$SANDBOX/outer/inner/.git"
printf 'garbage\n' >"$SANDBOX/outer/inner/.git/HEAD"
expect nested-skipped-exits-2 "$SANDBOX/outer/inner" 2 "marks a repository beneath it"

if [[ "$fails" -gt 0 ]]; then
    echo "check-task-conservation.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-task-conservation.test.sh: clean (no repository clean, a refused and a nested skipped repository exit 2, 3 cases)"
exit 0
