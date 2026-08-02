#!/usr/bin/env bash
# Direct unit test of gate-sdk/bin/run-gate-tests.sh's expect.txt semantics —
# every non-blank line of an expect file is a separate assertion, and a case
# fails when any one of them is absent. A fixture pair cannot cover this: the
# runner is a bin/ tool, never a gates.list member, so it owes no good/+bad/
# pair, and a pair here would sit outside check-gate-fixture-coverage's registry
# authority set and be audited by nothing.
#
# The self-invocation is bounded: each inner run is handed a scratch TESTS_DIR
# holding fixture dirs and no *.test.sh, so it runs pairs and returns. Do not
# let the scratch tree acquire one.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

ROOT="$(git rev-parse --show-toplevel 2>/dev/null || pwd)"
RUN="$ROOT/gate-sdk/bin/run-gate-tests.sh"
[[ -x "$RUN" ]] || { echo "run-gate-tests.test: runner not found: $RUN"; exit 2; }

scratch="$(mktemp -d)"
trap 'rm -rf "$scratch"' EXIT

mkdir -p "$scratch/checks"
# The stub gate replays a case dir's own say/rc files, so each case dictates the
# output and exit status run_case sees.
{ printf '#!/usr/bin/env bash\n'
  printf 'cat ./say\n'
  printf 'exit "$(cat ./rc)"\n'
} > "$scratch/checks/stub-gate.sh"
chmod +x "$scratch/checks/stub-gate.sh"

# mk_tree <name> <good-say> <good-expect> <bad-say> <bad-expect>; an empty
# good-expect ships no good/expect.txt (the optional side of the asymmetry).
# Each say gets the output-contract line its exit code owes (§Output contract,
# asserted at runtime by run_case) appended, so these rows keep testing the
# expect.txt conjunction rather than doubling as output-contract cases.
mk_tree() {
    local name="$1" good_say="$2" good_expect="$3" bad_say="$4" bad_expect="$5"
    local dir="$scratch/$name/stub-gate"
    mkdir -p "$dir/good" "$dir/bad"
    printf '%s\nSTUB-GATE: clean (stub)\n' "$good_say" > "$dir/good/say"
    printf '0\n' > "$dir/good/rc"
    [[ -n "$good_expect" ]] && printf '%s\n' "$good_expect" > "$dir/good/expect.txt"
    printf '%s\n  help: stub remedy\n' "$bad_say" > "$dir/bad/say"
    printf '1\n' > "$dir/bad/rc"
    printf '%s\n' "$bad_expect" > "$dir/bad/expect.txt"
    echo "$scratch/$name"
}

run_tree() { bash "$RUN" "$1" "$scratch/checks" 2>&1; }

fails=0
assert_has()    { grep -qF -- "$2" <<<"$3" || { echo "FAIL [$1]: expected present: $2"; fails=$((fails + 1)); }; }
assert_absent() { grep -qF -- "$2" <<<"$3" && { echo "FAIL [$1]: expected absent: $2"; fails=$((fails + 1)); }; return 0; }
assert_rc()     { [[ "$2" -eq "$3" ]] || { echo "FAIL [$1]: expected exit $3, got $2"; fails=$((fails + 1)); }; }

# Row 1 — the unit: two expect lines, the gate prints only the first. Under the
# disjunction this passed; the second line must now be reported missing.
t="$(mk_tree first-only 'good ok' '' 'alpha fired' $'alpha fired\nbeta fired')"
out="$(run_tree "$t")"; rc=$?
assert_rc  first-only "$rc" 1
assert_has first-only 'missing: beta fired' "$out"
assert_absent first-only 'missing: alpha fired' "$out"

# Row 3 — the symmetric half: only the second line prints. A fix that checked
# just the first line would pass row 1 and leave the defect live here.
t="$(mk_tree second-only 'good ok' '' 'beta fired' $'alpha fired\nbeta fired')"
out="$(run_tree "$t")"; rc=$?
assert_rc  second-only "$rc" 1
assert_has second-only 'missing: alpha fired' "$out"
assert_absent second-only 'missing: beta fired' "$out"

# Row 2 — both lines print: the conjunction is satisfied, order irrelevant (the
# gate prints them in the reverse of the expect file's order).
t="$(mk_tree both 'good ok' '' $'beta fired\nalpha fired' $'alpha fired\nbeta fired')"
out="$(run_tree "$t")"; rc=$?
assert_rc  both "$rc" 0
assert_has both 'GATE-TESTS: clean' "$out"

# Row 4 — the single-line case is untouched: the tightening must not regress the
# shape nearly every tracked expect file uses.
t="$(mk_tree single 'good ok' '' 'alpha fired' 'alpha fired')"
out="$(run_tree "$t")"; rc=$?
assert_rc  single "$rc" 0
assert_has single 'GATE-TESTS: clean' "$out"

# Row 5 — a blank line between two real ones asserts nothing: it is a separator,
# and reporting it as a pin would name an empty missing line.
t="$(mk_tree blank 'good ok' '' $'alpha fired\nbeta fired' $'alpha fired\n\nbeta fired')"
out="$(run_tree "$t")"; rc=$?
assert_rc  blank "$rc" 0
assert_has blank 'GATE-TESTS: clean' "$out"
assert_absent blank 'missing: ' "$out"

# Row 6 — good/expect.txt reads identically: optional to supply, conjunctive
# once supplied.
t="$(mk_tree good-side 'gamma ok' $'gamma ok\ndelta ok' 'alpha fired' 'alpha fired')"
out="$(run_tree "$t")"; rc=$?
assert_rc  good-side "$rc" 1
assert_has good-side 'missing: delta ok' "$out"

# Every missing line is named in one run: an author fixing one pin per re-run is
# the same under-assertion defect at a smaller scale.
t="$(mk_tree all-missing 'good ok' '' 'alpha fired' $'alpha fired\nbeta fired\ngamma fired')"
out="$(run_tree "$t")"; rc=$?
assert_rc  all-missing "$rc" 1
assert_has all-missing 'missing: beta fired' "$out"
assert_has all-missing 'missing: gamma fired' "$out"

# The output contract, asserted at runtime rather than by grepping a gate's
# source (§Output contract). A clean case that satisfies every expect line but
# emits no canonical clean line is still a failure — this is the assertion that
# survives a gate's source becoming a compiled subcommand there is nothing to
# grep.
mk_no_clean_line() {
    local dir="$scratch/no-clean/stub-gate"
    mkdir -p "$dir/good" "$dir/bad"
    printf 'gamma ok\n' > "$dir/good/say"; printf '0\n' > "$dir/good/rc"
    printf 'gamma ok\n' > "$dir/good/expect.txt"
    printf 'alpha fired\n  help: stub remedy\n' > "$dir/bad/say"; printf '1\n' > "$dir/bad/rc"
    printf 'alpha fired\n' > "$dir/bad/expect.txt"
    echo "$scratch/no-clean"
}
out="$(run_tree "$(mk_no_clean_line)")"; rc=$?
assert_rc  no-clean "$rc" 1
assert_has no-clean "emitted no '<NAME>: clean" "$out"

# The symmetric half: a violating case with no help: remedy line.
mk_no_help_line() {
    local dir="$scratch/no-help/stub-gate"
    mkdir -p "$dir/good" "$dir/bad"
    printf 'STUB-GATE: clean (stub)\n' > "$dir/good/say"; printf '0\n' > "$dir/good/rc"
    printf 'alpha fired\n' > "$dir/bad/say"; printf '1\n' > "$dir/bad/rc"
    printf 'alpha fired\n' > "$dir/bad/expect.txt"
    echo "$scratch/no-help"
}
out="$(run_tree "$(mk_no_help_line)")"; rc=$?
assert_rc  no-help "$rc" 1
assert_has no-help "no 'help:' remedy line" "$out"

[[ "$fails" -eq 0 ]] || { echo "run-gate-tests.test: $fails assertion(s) failed"; exit 1; }
echo "run-gate-tests.test: clean (expect.txt is a per-line conjunction, order-independent, blanks inert, all missing lines named; the output contract is asserted at runtime on both cases)"
exit 0
