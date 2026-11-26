#!/usr/bin/env bash
# Behavioral test of check-exec-write's refusals and unparseable spans, which a good/+bad/ pair
# cannot hold: a pair case exits 0 or 1. Each case runs in a sandbox repository.
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

repo="$SANDBOX/repo"
mkdir -p "$repo/src"
git -C "$repo" init -q
printf 'pub fn f() {}\n' > "$repo/src/lib.rs"
printf 'pub fn h() {}\n' > "$repo/src/helper.rs"
git -C "$repo" add src

guard() {  # guard <helpers> [gate args…] — the gate in the sandbox over src
    local helpers="$1"
    shift
    ( cd "$repo" && gate_env GATE_SDK_EXEC_WRITE_PATHS=src GATE_SDK_EXEC_WRITE_HELPERS="$helpers" \
        && gate_run check-exec-write "$CHECKS" "$@" 2>&1 )
}

# --- the paths knob empty: clean, not applicable.
out="$( cd "$repo" && gate_run check-exec-write "$CHECKS" 2>&1 )"; rc=$?
expect not-applicable 0 'not applicable: no Rust path configured' "$rc" "$out"
out="$(guard src/helper.rs)"; rc=$?
expect clean 0 'clean (1 Rust file(s) read, 1 helper(s) exempt)' "$rc" "$out"

# --- a helper outside the tree, absent, or absolute, and an operand, are refused at 2.
out="$(guard ../src/helper.rs)"; rc=$?
expect helper-dotdot 2 "helper '../src/helper.rs' holds '..'" "$rc" "$out"
out="$(guard src/absent.rs)"; rc=$?
expect helper-absent 2 "helper 'src/absent.rs' names no file" "$rc" "$out"
out="$(guard "$repo/src/helper.rs")"; rc=$?
expect helper-absolute 2 'is absolute' "$rc" "$out"
out="$(guard src/helper.rs src/lib.rs)"; rc=$?
expect usage 2 'takes no arguments' "$rc" "$out"

# --- an unterminated raw string and an unbalanced call are unparseable spans at 3.
printf 'pub fn f() {\n    let s = r#"open";\n}\n' > "$repo/src/lib.rs"
out="$(guard src/helper.rs)"; rc=$?
expect raw-unterminated 3 'src/lib.rs:2: [LINT-PARSE] unparseable span' "$rc" "$out"
printf 'pub fn f() {\n    g(a,\n}\n' > "$repo/src/lib.rs"
out="$(guard src/helper.rs)"; rc=$?
expect call-unbalanced 3 'src/lib.rs:3: [LINT-PARSE] unparseable span' "$rc" "$out"

if [[ "$fails" -gt 0 ]]; then
    echo "check-exec-write.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-exec-write.test.sh: clean (the helper refusals and the unparseable spans — $cases cases)"
exit 0
