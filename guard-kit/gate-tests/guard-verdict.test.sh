#!/usr/bin/env bash
# spec: guard-kit/SPEC.md §guard-verdict — the arm prints the decision class and the hook's own streams for one command, asked in the decision table's sandbox or in the tree, of the running binary or the one --bin names, and files nothing as friction. Every case is a property of the seam — argv, stdin, a real child and the scratch dir — which the crate's unit tests cannot see; the grammar, the stdin trim and the rendering are pinned in the module's own tests.
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

fails=0
assert_rc()    { [[ "$2" -eq "$3" ]] || { echo "FAIL [$1]: expected exit $3, got $2"; fails=$((fails + 1)); }; }
assert_first() { [[ "$(head -n 1 <<<"$3")" == "$2" ]] || { echo "FAIL [$1]: expected first line '$2', got: $3"; fails=$((fails + 1)); }; }
assert_has()   { grep -qF -- "$2" <<<"$3" || { echo "FAIL [$1]: expected present: $2"; fails=$((fails + 1)); }; }

sandboxes=()
trap 'rm -rf "${sandboxes[@]}"' EXIT

# One case: a `git init` sandbox of its own, the scratch dir and the friction log pinned into it.
verdict() {  # $1=case  $2.. = the arm's argv; stdin passes through
    local sb; sb="$(mktemp -d)"; sandboxes+=("$sb")
    git -C "$sb" init -q
    mkdir -p "$sb/.tmp"
    printf 'seed\n' >"$sb/friction.log"
    SB="$sb"
    out="$(cd "$sb" && gate_env GATE_SDK_TMP_DIR="$sb/.tmp" GUARD_KIT_LOG="$sb/friction.log" && gate_arm_run --guard-verdict "${@:2}" 2>&1)"
    rc=$?
}

verdict sandbox-block --sandbox -- 'cd deploy && ls'
assert_rc sandbox-block "$rc" 0
assert_first sandbox-block block "$out"
assert_has sandbox-block "don't use 'cd' in a compound command" "$out"

verdict sandbox-allow --sandbox -- 'touch .tmp/a.txt'
assert_rc sandbox-allow "$rc" 0
assert_first sandbox-allow allow "$out"
assert_has sandbox-allow '"permissionDecision":"allow"' "$out"

verdict sandbox-fallthrough --sandbox -- 'git status'
assert_rc sandbox-fallthrough "$rc" 0
[[ "$out" == fallthrough ]] || { echo "FAIL [sandbox-fallthrough]: expected 'fallthrough' alone, got: $out"; fails=$((fails + 1)); }

verdict stdin --sandbox --stdin <<<'cd deploy && ls'
assert_rc stdin "$rc" 0
assert_first stdin block "$out"

verdict substitution --sandbox -- 'bash @ROOT@/scripts/check-foo.sh'
assert_rc substitution "$rc" 0
assert_first substitution rewrite "$out"

verdict bin --sandbox --bin "$GATE_SDK_NATIVE_BIN" -- 'cd deploy && ls'
assert_rc bin "$rc" 0
assert_first bin block "$out"
printf 'not a binary\n' >"$SB/plain"
verdict bin-not-executable --sandbox --bin "$SB/plain" -- 'git status'
assert_rc bin-not-executable "$rc" 2

verdict in-tree-no-friction -- 'git status'
assert_rc in-tree-no-friction "$rc" 0
assert_first in-tree-no-friction fallthrough "$out"
[[ "$(cat "$SB/friction.log")" == seed ]] || { echo "FAIL [in-tree-no-friction]: the pinned friction log changed"; fails=$((fails + 1)); }
left="$(find "$SB/.tmp" -mindepth 1 2>/dev/null)"
[[ -z "$left" ]] || { echo "FAIL [in-tree-no-friction]: the arm left a file under the scratch dir: $left"; fails=$((fails + 1)); }

for refused in none surplus dash stdin-operand; do
    case "$refused" in
        none) verdict "refuse-$refused" ;;
        surplus) verdict "refuse-$refused" a b ;;
        dash) verdict "refuse-$refused" -x -- a ;;
        stdin-operand) verdict "refuse-$refused" --stdin a </dev/null ;;
    esac
    assert_rc "refuse-$refused" "$rc" 2
    assert_has "refuse-$refused" "usage: checkwright-gates --guard-verdict" "$out"
done

[[ "$fails" -eq 0 ]] || { echo "guard-verdict.test: $fails assertion(s) failed"; exit 1; }
echo "guard-verdict.test: clean (block, allow, fall-through and rewrite answered in the sandbox, from an operand and from stdin; --bin asks the named binary and refuses a non-executable; an in-tree fall-through leaves the friction log and the scratch dir untouched; four grammar refusals print the usage at exit 2)"
exit 0
