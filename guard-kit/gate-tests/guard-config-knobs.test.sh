#!/usr/bin/env bash
# Direct unit test of the lib/guard.sh knobs whose effect a `decision <TAB> command` row cannot
# express. The decision table's runner feeds every row through one sandbox whose config it never
# varies, so a knob's non-default value has no row to live in; each case here sources the library
# under a sandbox guard-config.sh selected through GUARD_KIT_CONFIG_FILE, the consumer's own
# selector, and asks one rule for its verdict.
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # guard-kit/

fails=0
checks=0
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

# guard_block exits 2, so each verdict is taken in a subshell that sources the library under the
# case's config: 2 = blocked, 0 = declined and fell through to the rules after it.
run_rule() {  # $1=config file $2=rule function $3=command
    (
        # shellcheck source=../lib/guard.sh
        GUARD_KIT_CONFIG_FILE="$1" source "$DIR/lib/guard.sh"
        "$2" "$3"
    )
}

want() {  # $1=label $2=config $3=rule $4=command $5=want-rc
    checks=$((checks + 1))
    local got
    run_rule "$2" "$3" "$4" >/dev/null 2>&1
    got=$?
    [[ "$got" == "$5" ]] || {
        echo "  FAIL [$1]: '$4' under ${2##*/} gave rc=$got, want $5"
        fails=$((fails + 1))
    }
}

: >"$tmp/defaults.sh"
printf 'GUARD_KIT_SEARCH_TOOLS=()\n' >"$tmp/no-search-tools.sh"
printf 'GUARD_KIT_SEARCH_TOOLS=(Grep)\n' >"$tmp/grep-only.sh"

# --- GUARD_KIT_SEARCH_TOOLS: the kit default names both tools, so both steers fire
want "default-find"       "$tmp/defaults.sh"        guard_rule_find_glob "find lib -type f" 2
want "default-git-grep"   "$tmp/defaults.sh"        guard_rule_git_grep  "git grep foo" 2

# --- an empty value leaves both rules inert: the tool a steer would name is not there to reach
want "empty-find"         "$tmp/no-search-tools.sh" guard_rule_find_glob "find lib -type f" 0
want "empty-git-grep"     "$tmp/no-search-tools.sh" guard_rule_git_grep  "git grep foo" 0

# --- each rule reads its own member and never the other's
want "grep-only-find"     "$tmp/grep-only.sh"       guard_rule_find_glob "find lib -type f" 0
want "grep-only-git-grep" "$tmp/grep-only.sh"       guard_rule_git_grep  "git grep foo" 2

# --- a firing corrective names the allowlisted bare fallback beside the tool, so a session whose
#     toolset lacks the tool is not steered at nothing
for pair in "guard_rule_find_glob|find lib -type f|find <dir> -type f | sort" \
    "guard_rule_git_grep|git grep foo|grep -rn <pattern> <path>"; do
    rule="${pair%%|*}"
    rest="${pair#*|}"
    command="${rest%%|*}"
    fallback="${rest#*|}"
    checks=$((checks + 1))
    out="$(run_rule "$tmp/defaults.sh" "$rule" "$command" 2>&1)"
    [[ "$out" == *"$fallback"* ]] || {
        echo "  FAIL [fallback:$rule]: the corrective did not name '$fallback': $out"
        fails=$((fails + 1))
    }
done

if [[ "$fails" -gt 0 ]]; then
    echo "guard-config-knobs.test: $fails of $checks assertion(s) failed"
    exit 1
fi
echo "guard-config-knobs.test: ok ($checks assertions; GUARD_KIT_SEARCH_TOOLS fires rules 9 and 11 on its own member each and leaves them inert when empty, and each firing corrective names its bare fallback)"
exit 0
