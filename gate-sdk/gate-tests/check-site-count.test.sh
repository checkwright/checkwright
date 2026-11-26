#!/usr/bin/env bash
# Behavioral test of check-site-count over what the good/+bad/ pair cannot reach: an empty
# configuration, the baseline's exit-2 and exit-3 classes, a malformed rules file, the test-code
# classifier's knob and its Rust test module, and a tree with no git listing.
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # gate-sdk/
CHECKS="$DIR/checks"
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0
cases=0

expect() {  # expect <label> <want-rc> <substring> <got-rc> <output>
    cases=$((cases + 1))
    if [[ "$4" -ne "$2" ]]; then
        echo "  FAIL [$1]: want exit $2, got $4 -- $5"; fails=$((fails + 1))
    elif ! grep -qF -- "$3" <<<"$5"; then
        echo "  FAIL [$1]: exit $2 but output lacks '$3': $5"; fails=$((fails + 1))
    fi
}

# --- no rules file, or one holding no block: clean, not applicable.
mkdir -p "$SANDBOX/empty"
out="$( cd "$SANDBOX/empty" && gate_run check-site-count "$CHECKS" 2>&1 )"; rc=$?
expect not-applicable 0 'not applicable: no site rule configured' "$rc" "$out"
printf '# no rule yet\n' > "$SANDBOX/empty/rules.list"
out="$( cd "$SANDBOX/empty" && gate_env GATE_SDK_SITE_RULES=rules.list \
    && gate_run check-site-count "$CHECKS" 2>&1 )"; rc=$?
expect no-block 0 'not applicable: no site rule configured' "$rc" "$out"

# --- a rules file the gate cannot read is 3; a malformed one is 2 naming its line.
printf 'rule A site-count\n\xff\n' > "$SANDBOX/empty/bytes.list"
out="$( cd "$SANDBOX/empty" && gate_env GATE_SDK_SITE_RULES=bytes.list \
    && gate_run check-site-count "$CHECKS" 2>&1 )"; rc=$?
expect rules-unreadable 3 'bytes.list' "$rc" "$out"
printf 'rule A site-count\n  site x\n' > "$SANDBOX/empty/no-baseline.list"
out="$( cd "$SANDBOX/empty" && gate_env GATE_SDK_SITE_RULES=no-baseline.list \
    && gate_run check-site-count "$CHECKS" 2>&1 )"; rc=$?
expect rules-missing-key 2 "no-baseline.list:1: rule A lacks its 'baseline' key" "$rc" "$out"

# --- outside a work tree there is no tracked listing, which is refused, never clean.
printf 'rule A site-count\n  site x\n  baseline sites.tsv\n' > "$SANDBOX/empty/rules.list"
out="$( cd "$SANDBOX/empty" && gate_env GATE_SDK_SITE_RULES=rules.list \
    && gate_run check-site-count "$CHECKS" 2>&1 )"; rc=$?
expect no-work-tree 2 'not a git repository' "$rc" "$out"

# --- a tracked tree: the baseline's classes, then the test-code classifier.
mkdir -p "$SANDBOX/tree/src" "$SANDBOX/tree/tests"
printf 'rule CLI site-count\n  site ^tool \n  baseline sites.tsv\n  paths src tests\n' > "$SANDBOX/tree/rules.list"
printf 'tool a\n#[cfg(test)]\ntool b\n' > "$SANDBOX/tree/src/lib.rs"
printf 'tool c\n' > "$SANDBOX/tree/tests/run.sh"
git -C "$SANDBOX/tree" init -q
git -C "$SANDBOX/tree" add src/lib.rs tests/run.sh
run_tree() {
    ( cd "$SANDBOX/tree" && gate_env GATE_SDK_SITE_RULES=rules.list "$@" \
        && gate_run check-site-count "$CHECKS" 2>&1 )
}

out="$(run_tree)"; rc=$?
expect baseline-absent 2 'sites.tsv' "$rc" "$out"
printf '1 src/lib.rs\n' > "$SANDBOX/tree/sites.tsv"
out="$(run_tree)"; rc=$?
expect baseline-no-tab 3 'sites.tsv:1:' "$rc" "$out"
printf '1\tsrc/lib.rs\n1\tsrc/lib.rs\n' > "$SANDBOX/tree/sites.tsv"
out="$(run_tree)"; rc=$?
expect baseline-duplicate 3 'sites.tsv:2: src/lib.rs is given a second row' "$rc" "$out"

# The Rust test module and the tests/ directory are test code under the default globs.
printf '1\tsrc/lib.rs\n' > "$SANDBOX/tree/sites.tsv"
out="$(run_tree)"; rc=$?
expect test-code-skipped 0 'clean (1 file(s) read against 1 rule(s) in rules.list)' "$rc" "$out"
# An emptied knob names no test path, so the tests/ file is read; the Rust module is still cut.
out="$(run_tree GATE_SDK_TEST_GLOBS=)"; rc=$?
expect test-globs-empty 1 'tests/run.sh: [CLI] 1/0' "$rc" "$out"

# --- an argument is a usage error.
out="$( cd "$SANDBOX/empty" && gate_run check-site-count "$CHECKS" extra 2>&1 )"; rc=$?
expect usage 2 'takes no arguments' "$rc" "$out"

if [[ "$fails" -gt 0 ]]; then
    echo "check-site-count.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-site-count.test.sh: clean (configuration, the rules file and baseline exit classes, and the test-code classifier — $cases cases)"
exit 0
