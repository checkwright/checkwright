#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §The bin/-tool contract — the `--emit-` dispatcher's argument refusal, as a
# dispatched member meets it: a token a member's declared grammar does not name is exit 2 before the
# member runs, and every refusal prints the member's usage block exactly once. A crate unit test holds
# the dispatch function; this holds the binary a caller actually reaches.
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT

fails=0
checks=0
note() { echo "  FAIL [$1]: $2"; fails=$((fails + 1)); }

# run <row> <want-rc> <arm> [args...] — stdout and stderr land in $scratch/<row>.out / .err
run() {
    local row="$1" want="$2"
    shift 2
    checks=$((checks + 1))
    gate_arm_run "$@" >"$scratch/$row.out" 2>"$scratch/$row.err"
    local rc=$?
    [[ "$rc" -eq "$want" ]] || note "$row" "exited $rc rather than $want: $(cat "$scratch/$row.err")"
}
has() { grep -qF -- "$3" "$scratch/$1.$2" || note "$1" "$2 lacks: $3"; }
usage_once() {
    local n
    n="$(grep -c '^usage:\|: usage:' "$scratch/$1.err")"
    [[ "$n" -eq 1 ]] || note "$1" "the refusal printed $n usage lines rather than one"
}

# A member taking no argument refuses a surplus one rather than dropping it at exit 0.
run surplus 2 --emit-session-id bogus-surplus
has surplus err "unrecognized argument: bogus-surplus"
has surplus err "usage: --emit session-id   (it takes no argument)"
[[ -s "$scratch/surplus.out" ]] && note surplus "a refused invocation printed a document"

# A member reading one flag refuses a misspelling of it, naming the flag it does read.
run misspelt 2 --emit-value-rollup --writ
has misspelt err "unrecognized argument: --writ"
has misspelt err "usage: --emit value-rollup [--write]"

# `--help` is no per-arm help flag: it is the member's refusal, and the refusal carries the usage.
run help 2 --emit-file-survey --help
has help err "unrecognized option: --help"
has help err "usage: --emit file-survey"
usage_once help

# A member whose own refusal already spells its usage is not handed it twice.
run spelled 2 --emit-md-section
usage_once spelled

# The negative control: the declared-empty grammar admits the bare invocation it declares.
run bare 0 --emit-kit-roots
[[ -s "$scratch/bare.out" ]] || note bare "the bare invocation printed no kit root"

if [[ "$fails" -gt 0 ]]; then
    echo "emit-arm-shape.test.sh: $fails case(s) failed"
    exit 1
fi
echo "emit-arm-shape.test.sh: clean (a surplus and a misspelt argument are refused before the member runs, --help is a refusal, every refusal prints its usage once, and a bare invocation of an argument-free member still runs; $checks invocations)"
exit 0
