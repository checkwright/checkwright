#!/usr/bin/env bash
# Direct unit test of the lib/guard.sh knobs whose effect a `decision <TAB> command` row cannot
# express. The decision table's runner feeds every row through one sandbox whose config it never
# varies, so a knob's non-default value has no row to live in; each case here sources the library
# under a sandbox knob file selected through GUARD_KIT_KNOB_FILE, the consumer's own
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
        GUARD_KIT_KNOB_FILE="$1" source "$DIR/lib/guard.sh"
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

grants() {  # $1=label $2=config $3=command $4=want (allow|withheld), asked of rule 18
    checks=$((checks + 1))
    local out got=withheld
    out="$(run_rule "$2" guard_rule_ro_pipeline "$3" 2>/dev/null)"
    [[ "$out" == *'"permissionDecision":"allow"'* ]] && got=allow
    [[ "$got" == "$4" ]] || {
        echo "  FAIL [$1]: '$3' under ${2##*/} was $got, want $4"
        fails=$((fails + 1))
    }
}

: >"$tmp/defaults.knobs"
printf 'GUARD_KIT_SEARCH_TOOLS =\n' >"$tmp/no-search-tools.knobs"
printf 'GUARD_KIT_SEARCH_TOOLS[] = Grep\n' >"$tmp/grep-only.knobs"
printf 'GUARD_KIT_RO_BINS[] = grep\nGUARD_KIT_RO_BINS[] = md5sum\nGUARD_KIT_RO_FORMS[md5sum] = none\n' >"$tmp/ro-declared.knobs"
printf 'GUARD_KIT_RO_BINS[] = grep\nGUARD_KIT_RO_BINS[] = md5sum\n' >"$tmp/ro-undeclared.knobs"
printf 'GUARD_KIT_RO_BINS[] = grep\nGUARD_KIT_RO_BINS[] = md5sum\nGUARD_KIT_RO_FORMS[md5sum] =\n' >"$tmp/ro-empty.knobs"
printf 'GUARD_KIT_RO_FORMS[head] = -n\n' >"$tmp/ro-override.knobs"

# --- GUARD_KIT_RO_FORMS: a roster member a consumer adds is granted only once it is declared, since
#     an undeclared member is withheld, and an empty declaration is undeclared rather than 'none'
grants "ro-declared"      "$tmp/ro-declared.knobs" "grep foo a.md | md5sum" allow
grants "ro-undeclared"    "$tmp/ro-undeclared.knobs" "grep foo a.md | md5sum" withheld
grants "ro-empty"         "$tmp/ro-empty.knobs"    "grep foo a.md | md5sum" withheld

# --- a consumer entry for a default member replaces the kit's declaration rather than adding to it
grants "ro-kit-head"      "$tmp/defaults.knobs"    "grep foo a.md | head -n 5" allow
grants "ro-override-head" "$tmp/ro-override.knobs" "grep foo a.md | head -n 5" withheld

# --- GUARD_KIT_SEARCH_TOOLS: the kit default names both tools, so both steers fire
want "default-find"       "$tmp/defaults.knobs"        guard_rule_find_glob "find lib -type f" 2
want "default-git-grep"   "$tmp/defaults.knobs"        guard_rule_git_grep  "git grep foo" 2

# --- an empty value leaves both rules inert: the tool a steer would name is not there to reach
want "empty-find"         "$tmp/no-search-tools.knobs" guard_rule_find_glob "find lib -type f" 0
want "empty-git-grep"     "$tmp/no-search-tools.knobs" guard_rule_git_grep  "git grep foo" 0

# --- each rule reads its own member and never the other's
want "grep-only-find"     "$tmp/grep-only.knobs"       guard_rule_find_glob "find lib -type f" 0
want "grep-only-git-grep" "$tmp/grep-only.knobs"       guard_rule_git_grep  "git grep foo" 2

# --- a firing corrective names the allowlisted bare fallback beside the tool, so a session whose
#     toolset lacks the tool is not steered at nothing
for pair in "guard_rule_find_glob|find lib -type f|find <dir> -type f | sort" \
    "guard_rule_git_grep|git grep foo|grep -rn <pattern> <path>"; do
    rule="${pair%%|*}"
    rest="${pair#*|}"
    command="${rest%%|*}"
    fallback="${rest#*|}"
    checks=$((checks + 1))
    out="$(run_rule "$tmp/defaults.knobs" "$rule" "$command" 2>&1)"
    [[ "$out" == *"$fallback"* ]] || {
        echo "  FAIL [fallback:$rule]: the corrective did not name '$fallback': $out"
        fails=$((fails + 1))
    }
done

# --- the load's two failure answers: an unreachable binary advises, runs no rule and exits 0; a
#     refused config blocks with the refusal's own text
load() {  # $1=label $2=want-rc $3=want-substring $4.. = NAME=VALUE environment for the sourcing
    local label="$1" want_rc="$2" want_text="$3" out got
    shift 3
    checks=$((checks + 1))
    out="$(env "$@" bash -c 'source "$1/lib/guard.sh"; guard_rule_cat_file "cat README.md"; echo ran' _ "$DIR" 2>&1)"
    got=$?
    [[ "$got" == "$want_rc" && "$out" == *"$want_text"* && "$out" != *ran ]] || {
        echo "  FAIL [$label]: rc=$got, want $want_rc carrying '$want_text' with no rule run: $out"
        fails=$((fails + 1))
    }
}
printf 'GUARD_KIT_SEARCH_TOOLS\n' >"$tmp/malformed.knobs"
load "unreachable-binary" 0 '"additionalContext"' GATE_SDK_NATIVE_BIN="$tmp/no-such-binary"
load "missing-knob-file"  2 "GUARD_KIT_KNOB_FILE names $tmp/absent.knobs" GUARD_KIT_KNOB_FILE="$tmp/absent.knobs"
load "malformed-knob-file" 2 "malformed.knobs:1" GUARD_KIT_KNOB_FILE="$tmp/malformed.knobs"

if [[ "$fails" -gt 0 ]]; then
    echo "guard-config-knobs.test: $fails of $checks assertion(s) failed"
    exit 1
fi
echo "guard-config-knobs.test: ok ($checks assertions; GUARD_KIT_SEARCH_TOOLS fires rules 9 and 11 on its own member each and leaves them inert when empty, and each firing corrective names its bare fallback; GUARD_KIT_RO_FORMS grants an added roster member only once declared, and a consumer entry replaces the kit's declaration; the knob load advises and runs no rule on an unreachable binary, and blocks with the refusal on a missing or malformed knob file)"
exit 0
