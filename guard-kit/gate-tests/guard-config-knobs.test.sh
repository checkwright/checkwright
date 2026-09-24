#!/usr/bin/env bash
# Direct test of the guard knobs whose effect a `decision <TAB> command` row cannot express. The
# decision table's runner feeds every row through one sandbox whose config it never varies, so a
# knob's non-default value has no row to live in; each case here drives the shell-guard member
# under a sandbox knob file selected through GUARD_KIT_KNOB_FILE, the consumer's own selector. Each
# case runs the whole ruleset, so a case another rule decides asserts the deciding rule's text.
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # guard-kit/
BIN="$GATE_SDK_NATIVE_BIN"
[[ -x "$BIN" ]] || { echo "guard-config-knobs.test: the gate binary $BIN is absent — build it first"; exit 2; }
BIN="$(cd "$(dirname "$BIN")" && pwd -P)/$(basename "$BIN")"

fails=0
checks=0
# spec: guard-kit/SPEC.md §The generic ruleset — a value carrying the letters SQ or DQ reads as a
# skeleton placeholder, so a random scratch name that spells one is drawn again
until tmp="$(cd "$(mktemp -d)" && pwd -P)"; [[ "$tmp" != *SQ* && "$tmp" != *DQ* ]]; do
    rm -rf "$tmp"
done
trap 'rm -rf "$tmp"' EXIT
mkdir -p "$tmp/cwd"
git -C "$tmp/cwd" init -q

payload() {
    local c="$1"
    c="${c//\\/\\\\}"
    c="${c//\"/\\\"}"
    printf '{"tool_name":"Bash","tool_input":{"command":"%s"}}' "$c"
}

# One member call under a knob file, from a scratch checkout: stdout to $tmp/out, stderr to
# $tmp/err, the exit status echoed — 2 = blocked, 0 = anything else.
member() {  # $1=config file $2=command
    (cd "$tmp/cwd" && payload "$2" | GUARD_KIT_KNOB_FILE="$1" GUARD_KIT_LOG="$tmp/friction.log" "$BIN" --hook shell-guard >"$tmp/out" 2>"$tmp/err")
    echo $?
}

want() {  # $1=label $2=config $3=command $4=want-rc [$5=text the block must carry]
    checks=$((checks + 1))
    local got; got="$(member "$2" "$3")"
    if [[ "$got" != "$4" ]]; then
        echo "  FAIL [$1]: '$3' under ${2##*/} gave rc=$got, want $4 — $(cat "$tmp/err")"
        fails=$((fails + 1))
    elif [[ -n "${5:-}" ]] && ! grep -qF -- "$5" "$tmp/err"; then
        echo "  FAIL [$1]: '$3' under ${2##*/} did not carry '$5': $(cat "$tmp/err")"
        fails=$((fails + 1))
    fi
}

grants() {  # $1=label $2=config $3=command $4=want (allow|withheld), the read-only pipeline grant
    checks=$((checks + 1))
    local got=withheld
    member "$2" "$3" >/dev/null
    grep -qF '"permissionDecision":"allow"' "$tmp/out" && got=allow
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
want "default-find"       "$tmp/defaults.knobs"        "find lib -type f" 2 "use the Glob tool"
want "default-git-grep"   "$tmp/defaults.knobs"        "git grep foo" 2 "use the Grep tool"

# --- an empty value leaves both rules inert: the tool a steer would name is not there to reach
want "empty-find"         "$tmp/no-search-tools.knobs" "find lib -type f" 0
want "empty-git-grep"     "$tmp/no-search-tools.knobs" "git grep foo" 0

# --- each rule reads its own member and never the other's
want "grep-only-find"     "$tmp/grep-only.knobs"       "find lib -type f" 0
want "grep-only-git-grep" "$tmp/grep-only.knobs"       "git grep foo" 2 "use the Grep tool"

# --- a firing corrective names the allowlisted bare fallback beside the tool, so a session whose
#     toolset lacks the tool is not steered at nothing
want "fallback-find"      "$tmp/defaults.knobs"        "find lib -type f" 2 "find <dir> -type f | sort"
want "fallback-git-grep"  "$tmp/defaults.knobs"        "git grep foo" 2 "grep -rn <pattern> <path>"

# --- rule `git_c_root`'s arm (d): a respelled path names the same file as the knob's resolved value and still
#     fires, since the generated hooks bake the text; a differing file falls through
printf 'GUARD_KIT_SETTINGS = %s/settings.json\n' "$tmp" >"$tmp/settings-path.knobs"
: >"$tmp/settings.json"
: >"$tmp/other.json"
k="$tmp/settings-path.knobs"
want "knob-echo-verbatim"   "$k" "GUARD_KIT_SETTINGS=$tmp/settings.json make build" 2 "Run it without the prefix: make build"
want "knob-echo-same-file"  "$k" "GUARD_KIT_SETTINGS=$tmp/./settings.json make build" 2 "check-graph"
want "knob-echo-other-file" "$k" "GUARD_KIT_SETTINGS=$tmp/other.json make build" 0

# --- rules `ro_pipeline` and `allowlist_chain` read an entry whose only '*' is a closing ' *' as granting its bare head, which
#     the harness does: a decorated bare head takes rule `allowlist_chain`'s steer and a read-only tail rule `ro_pipeline`'s
#     grant, an argument-carrying lead and the ':*' and two-star forms are not widened, and a
#     redirected emitter head is left to rule `emitter_write`
printf '{"permissions":{"allow":["Bash(make build *)","Bash(echo *)","Bash(make:*)","Bash(cargo * --x *)"]}}\n' >"$tmp/star.json"
printf 'GUARD_KIT_SETTINGS = %s/star.json\n' "$tmp" >"$tmp/star.knobs"
s="$tmp/star.knobs"
steer="bare — it's a statically allowlisted command"
want   "star-redirect"      "$s" "make build > .tmp/b.log" 2 "run 'make build' $steer"
want   "star-chain"         "$s" "make build; touch x" 2 "run 'make build' $steer"
want   "star-bare"          "$s" "make build" 0
want   "star-args"          "$s" "make build --jobs 2 > .tmp/b.log" 0
want   "colon-not-widened"  "$s" "make > .tmp/b.log" 0
want   "two-star-not-widened" "$s" "cargo > .tmp/b.log" 0
want   "emitter-redirect"   "$s" "echo > notes.md" 2 "don't write 'notes.md' through a redirect"
want   "emitter-chain"      "$s" "echo; touch x" 2 "run 'echo' $steer"
grants "star-ro-tail"       "$s" "make build | head -3" allow
grants "star-args-ro-tail"  "$s" "make build --jobs 2 | head -3" withheld

# --- a refused config blocks with the refusal's own text, and no rule runs
printf 'GUARD_KIT_SEARCH_TOOLS\n' >"$tmp/malformed.knobs"
want "missing-knob-file"   "$tmp/absent.knobs"    "cat README.md" 2 "GUARD_KIT_KNOB_FILE names $tmp/absent.knobs"
want "malformed-knob-file" "$tmp/malformed.knobs" "cat README.md" 2 "malformed.knobs:1"
checks=$((checks + 1))
grep -qF "Read tool" "$tmp/err" && { echo "  FAIL [no-rule-on-refusal]: a rule ran under a refused config: $(cat "$tmp/err")"; fails=$((fails + 1)); }

# --- the steer door is gate-sdk's command spelling of GATE_SDK_NATIVE_BIN: './'-prefixed when
#     relative, as written when absolute
door_case() {  # $1=label $2=GATE_SDK_NATIVE_BIN value
    checks=$((checks + 1))
    local spelled
    spelled="$(GATE_SDK_NATIVE_BIN="$2" bash -c 'source "$1/../gate-sdk/lib/gate.sh"; gate_native_bin_spelled' _ "$DIR")"
    (cd "$tmp/cwd" && payload "bash -c 'ls'" | GATE_SDK_NATIVE_BIN="$2" GUARD_KIT_KNOB_FILE="$tmp/defaults.knobs" GUARD_KIT_LOG="$tmp/friction.log" "$BIN" --hook shell-guard >/dev/null 2>"$tmp/err")
    grep -qF -- "'$spelled --scratch-run <script>'" "$tmp/err" || {
        echo "  FAIL [$1]: the steer does not name '$spelled': $(cat "$tmp/err")"
        fails=$((fails + 1))
    }
}
door_case "door-relative" "vendor/bin/checkwright-gates"
door_case "door-absolute" "/opt/checkwright/bin/checkwright-gates"

# --- the library's own load, while the library is live: an unreachable binary advises, runs no rule
#     and exits 0; a load list ahead of the binary fails open with an advise naming the name and the
#     rebuild; a refused config blocks with the refusal's own text
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
load "unreachable-binary" 0 '"additionalContext"' GATE_SDK_NATIVE_BIN="$tmp/no-such-binary"
load "lib-missing-knob-file"  2 "GUARD_KIT_KNOB_FILE names $tmp/absent.knobs" GUARD_KIT_KNOB_FILE="$tmp/absent.knobs"
load "lib-malformed-knob-file" 2 "malformed.knobs:1" GUARD_KIT_KNOB_FILE="$tmp/malformed.knobs"
cat >"$tmp/behind-bin" <<EOF
#!/usr/bin/env bash
case "\$1" in
    --emit-knob-values) echo "--emit-knob-values: GUARD_KIT_WORKTREE_READS is not a guard-kit knob" >&2; exit 2 ;;
    --emit-knob-roster) "$BIN" --emit-knob-roster | grep -v '^GUARD_KIT_WORKTREE_READS' ;;
    *) exec "$BIN" "\$@" ;;
esac
EOF
chmod +x "$tmp/behind-bin"
load "binary-behind-load"  0 "GUARD_KIT_WORKTREE_READS, which the gate binary" GATE_SDK_NATIVE_BIN="$tmp/behind-bin"
load "binary-behind-names-rebuild" 0 "build-native.sh" GATE_SDK_NATIVE_BIN="$tmp/behind-bin"
cat >"$tmp/refusing-bin" <<EOF
#!/usr/bin/env bash
case "\$1" in
    --emit-knob-values) echo "malformed.knobs:1: not a knob line" >&2; exit 2 ;;
    *) exec "$BIN" "\$@" ;;
esac
EOF
chmod +x "$tmp/refusing-bin"
load "declared-refusal-blocks" 2 "malformed.knobs:1" GATE_SDK_NATIVE_BIN="$tmp/refusing-bin"

if [[ "$fails" -gt 0 ]]; then
    echo "guard-config-knobs.test: $fails of $checks assertion(s) failed"
    exit 1
fi
echo "guard-config-knobs.test: ok ($checks assertions; GUARD_KIT_SEARCH_TOOLS fires rules \`find_glob\` and \`git_grep\` on its own member each and leaves them inert when empty, and each firing corrective names its bare fallback; GUARD_KIT_RO_FORMS grants an added roster member only once declared, and a consumer entry replaces the kit's declaration; rule `git_c_root`'s arm (d) fires on a respelled path naming the knob's resolved file and not on a differing one; the member blocks with the refusal on a missing or malformed knob file and runs no rule; the steer door is gate_native_bin_spelled's spelling; the library's load advises on an unreachable binary, blocks on a refused config, and fails open naming the rebuild when the binary does not declare a loaded name)"
exit 0
