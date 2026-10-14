#!/usr/bin/env bash
# Direct test of the shell guard's consumer stage, of --guard-json, the toolkit a consumer rule
# command calls, and of --run-guard-tests' consumer lane, its derivation and its limits-table read. Each stage case drives the shell-guard member from a scratch checkout under a
# sandbox knob file naming a scratch rule command, so the command's exit, stdout and stderr are the
# case's input and the member's relay of them is what is asserted.
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

BIN="$GATE_SDK_NATIVE_BIN"
[[ -x "$BIN" ]] || { echo "consumer-rules.test: the gate binary $BIN is absent — build it first"; exit 2; }
BIN="$(cd "$(dirname "$BIN")" && pwd -P)/$(basename "$BIN")"

fails=0
checks=0
tmp="$(cd "$(mktemp -d)" && pwd -P)"
trap 'rm -rf "$tmp"' EXIT
mkdir -p "$tmp/cwd"
git -C "$tmp/cwd" init -q

fail() {  # $1=label $2=what
    echo "  FAIL [$1]: $2"
    fails=$((fails + 1))
}

eq() {  # $1=label $2=got $3=want
    checks=$((checks + 1))
    [[ "$2" == "$3" ]] || fail "$1" "got '$2', want '$3'"
}

has() {  # $1=label $2=file $3=text it must carry
    checks=$((checks + 1))
    grep -qF -- "$3" "$2" || fail "$1" "$(basename "$2") lacks '$3': $(cat "$2")"
}

lacks() {  # $1=label $2=file $3=text it must not carry
    checks=$((checks + 1))
    ! grep -qF -- "$3" "$2" || fail "$1" "$(basename "$2") carries '$3': $(cat "$2")"
}

payload() {  # $1=command [$2=tool name]
    local c="$1"
    c="${c//\\/\\\\}"
    c="${c//\"/\\\"}"
    c="${c//$'\n'/\\n}"
    printf '{"tool_name":"%s","tool_input":{"command":"%s"}}' "${2:-Bash}" "$c"
}

# The scratch rule command: its first argument selects what it does with the call.
cat >"$tmp/rule.sh" <<'EOF'
#!/usr/bin/env bash
out="$2"
case "$1" in
    block) cat >"$out/seen"; echo "consumer: blocked by the project rule" >&2; exit 2 ;;
    allow) cat >"$out/seen"; "$3" --guard-json allow "consumer grant" ;;
    empty) cat >"$out/seen"; exit 0 ;;
    exit1) exit 1 ;;
    garbage) cat >/dev/null; echo "not json" ;;
    array) cat >/dev/null; echo "[1]" ;;
    env) cat >/dev/null; pwd -P >"$out/pwd"; printf '%s' "${CONSUMER_PROBE:-}" >"$out/env" ;;
esac
EOF

knobs() {  # $1=mode — a knob file naming the rule command in that mode
    printf 'GUARD_KIT_CONSUMER_RULES_CMD[] = bash\nGUARD_KIT_CONSUMER_RULES_CMD[] = %s\nGUARD_KIT_CONSUMER_RULES_CMD[] = %s\nGUARD_KIT_CONSUMER_RULES_CMD[] = %s\nGUARD_KIT_CONSUMER_RULES_CMD[] = %s\n' \
        "$tmp/rule.sh" "$1" "$tmp" "$BIN" >"$tmp/$1.knobs"
    printf '%s' "$tmp/$1.knobs"
}

# One member call under a knob file, from the scratch checkout: the payload as given, stdout to
# $tmp/out, stderr to $tmp/err, the exit status echoed.
member() {  # $1=knob file $2=payload
    rm -f "$tmp/seen" "$tmp/friction.log"
    (cd "$tmp/cwd" && printf '%s' "$2" | GUARD_KIT_KNOB_FILE="$1" GUARD_KIT_LOG="$tmp/friction.log" "$BIN" --hook shell-guard >"$tmp/out" 2>"$tmp/err")
    echo $?
}

generic_block="cd /tmp && ls"
generic_allow="grep foo a.md | head -n 5"
fallthrough="make build"

# --- a block is relayed with the command's own stderr, verbatim, and the generic ruleset never runs;
#     the command reads the payload bytes exactly as the harness sent them
sent="$(payload "$generic_block")"$'\r\n'
eq "block-status" "$(member "$(knobs block)" "$sent")" 2
eq "block-stderr" "$(cat "$tmp/err")" "consumer: blocked by the project rule"
eq "block-stdout" "$(cat "$tmp/out")" ""
checks=$((checks + 1))
cmp -s "$tmp/seen" <(printf '%s' "$sent") || fail "block-payload-verbatim" "the command read $(od -c "$tmp/seen" | head -3)"

# --- an allow envelope is relayed verbatim where the generic ruleset would have blocked the call,
#     and a consumer decision logs no fall-through
eq "allow-status" "$(member "$(knobs allow)" "$(payload "$generic_block")")" 0
eq "allow-stdout" "$(cat "$tmp/out")" "$("$BIN" --guard-json allow "consumer grant")"
checks=$((checks + 1))
[[ ! -e "$tmp/friction.log" ]] || fail "allow-no-fallthrough" "a fall-through was logged: $(cat "$tmp/friction.log")"

# --- an empty result is no decision: the generic ruleset decides, and a call it leaves falls through
eq "empty-block-status" "$(member "$(knobs empty)" "$(payload "$generic_block")")" 2
has "empty-generic-block" "$tmp/err" "shell-guard: "
eq "empty-fallthrough-status" "$(member "$(knobs empty)" "$(payload "$fallthrough")")" 0
eq "empty-fallthrough-stdout" "$(cat "$tmp/out")" ""
eq "empty-fallthrough-logged" "$(cat "$tmp/friction.log" 2>/dev/null)" "$fallthrough"

# --- a non-zero-non-2 exit is a fault: the ruleset runs, and its answer carries the fault — appended
#     to a block, as additionalContext on an allow, as an advise when nothing decided (still logged)
eq "exit1-block-status" "$(member "$(knobs exit1)" "$(payload "$generic_block")")" 2
has "exit1-block-generic" "$tmp/err" "shell-guard: "
has "exit1-block-fault" "$tmp/err" "exited 1"
has "exit1-block-names" "$tmp/err" "$tmp/rule.sh exit1"
eq "exit1-allow-status" "$(member "$(knobs exit1)" "$(payload "$generic_allow")")" 0
has "exit1-allow-decision" "$tmp/out" '"permissionDecision":"allow"'
eq "exit1-allow-context" "$("$BIN" --guard-json field-or-empty .hookSpecificOutput.additionalContext <"$tmp/out" | grep -c 'exited 1')" 1
eq "exit1-fallthrough-status" "$(member "$(knobs exit1)" "$(payload "$fallthrough")")" 0
eq "exit1-fallthrough-advise" "$("$BIN" --guard-json field-or-empty .hookSpecificOutput.additionalContext <"$tmp/out" | grep -c 'exited 1')" 1
eq "exit1-fallthrough-logged" "$(cat "$tmp/friction.log" 2>/dev/null)" "$fallthrough"

# --- stdout that does not parse, or parses as no object, is the same fault
eq "garbage-status" "$(member "$(knobs garbage)" "$(payload "$generic_block")")" 2
has "garbage-generic" "$tmp/err" "shell-guard: "
has "garbage-fault" "$tmp/err" "not a JSON object"
eq "array-status" "$(member "$(knobs array)" "$(payload "$fallthrough")")" 0
has "array-fault" "$tmp/out" "not a JSON object"

# --- a command that cannot start is a fault, never a silent pass
printf 'GUARD_KIT_CONSUMER_RULES_CMD[] = %s\n' "$tmp/no-such-command" >"$tmp/unstarted.knobs"
eq "unstarted-status" "$(member "$tmp/unstarted.knobs" "$(payload "$generic_block")")" 2
has "unstarted-fault" "$tmp/err" "could not run"

# --- the empty knob runs no stage
: >"$tmp/none.knobs"
eq "none-status" "$(member "$tmp/none.knobs" "$(payload "$generic_block")")" 2
has "none-generic" "$tmp/err" "shell-guard: "
checks=$((checks + 1))
[[ ! -e "$tmp/seen" ]] || fail "none-no-stage" "a consumer command ran"

# --- the stage runs whatever the tool, and a tool no reader serves is otherwise left alone
eq "tool-block-status" "$(member "$(knobs block)" "$(payload "ls" Read)")" 2
eq "tool-empty-status" "$(member "$(knobs empty)" "$(payload "$generic_block" Read)")" 0
eq "tool-empty-stdout" "$(cat "$tmp/out")" ""

# --- the command inherits the working directory and the environment
(cd "$tmp/cwd" && payload ls | CONSUMER_PROBE=inherited GUARD_KIT_KNOB_FILE="$(knobs env)" GUARD_KIT_LOG="$tmp/friction.log" "$BIN" --hook shell-guard >/dev/null 2>&1)
eq "env-cwd" "$(cat "$tmp/pwd" 2>/dev/null)" "$tmp/cwd"
eq "env-var" "$(cat "$tmp/env" 2>/dev/null)" "inherited"

# --- --guard-json's field read hands back a value's own bytes, a CR included
crlf='{"tool_name":"Bash","tool_input":{"command":"cat <<EOF\r\nline\r\nEOF","run_in_background":false}}'
eq "field-crlf" "$(printf '%s' "$crlf" | "$BIN" --guard-json field .tool_input.command)" "$(printf 'cat <<EOF\r\nline\r\nEOF')"
eq "field-false" "$(printf '%s' "$crlf" | "$BIN" --guard-json field .tool_input.run_in_background)" "false"
eq "field-or-empty-false" "$(printf '%s' "$crlf" | "$BIN" --guard-json field-or-empty .tool_input.run_in_background)" ""
eq "field-garbage" "$(printf 'not json' | "$BIN" --guard-json field .tool_input.command)" ""

# --- view: the named view of the command, heredoc bodies and quoted strings inert
view() {  # $1=command [$2=tool] $3..=the view's words
    local c="$1" t="$2"
    shift 2
    payload "$c" "$t" | "$BIN" --guard-json view "$@"
}
eq "view-heredoc" "$(view $'cat <<EOF\ngit commit --no-verify\nEOF' Bash sq dq hd)" "$(printf 'cat <<EOF\nHD\nEOF')"
eq "view-quoted" "$(view 'git commit -m "x --no-verify"' Bash sq dq hd)" "git commit -m DQ"
eq "view-one-operand" "$(view 'git commit -m "x"' Bash 'sq dq hd')" "git commit -m DQ"
eq "view-raw" "$(view 'echo "a"' Bash raw)" 'echo "a"'
eq "view-non-bash" "$(view 'echo "a"' Read sq dq hd | wc -c | tr -d ' ')" 0
eq "view-powershell" "$(view $'@\'\ncd x\n\'@ | git commit -F -' PowerShell sq dq hd)" "HD | git commit -F -"
eq "view-powershell-quotes" "$(view "rm 'it''s' \"a\`\"b\"" PowerShell sq dq hd)" "rm SQ DQ"
eq "view-garbage" "$(printf 'not json' | "$BIN" --guard-json view sq dq hd | wc -c | tr -d ' ')" 0
payload x | "$BIN" --guard-json view body >/dev/null 2>&1
eq "view-body-usage" "$?" 2
payload x | "$BIN" --guard-json view dq >/dev/null 2>&1
eq "view-unknown-usage" "$?" 2

# --- words: the word read's values, one line a segment, the words tab-separated
words() {  # $1=command [$2=tool]
    payload "$1" "${2:-Bash}" | "$BIN" --guard-json words
}
tab=$'\t'
eq "words-quoted" "$(words "git 'clean' -fd")" "git${tab}clean${tab}-fd"
eq "words-escaped" "$(words 'git \clean -fd')" "git${tab}clean${tab}-fd"
eq "words-split" "$(words "git cl'ea'n -fd")" "git${tab}clean${tab}-fd"
eq "words-segments" "$(words 'git commit -m x; echo --no-verify')" "$(printf 'git\tcommit\t-m\tx\necho\t--no-verify')"
eq "words-tab" "$(printf '%s' '{"tool_name":"Bash","tool_input":{"command":"echo '\''a\tb'\''"}}' | "$BIN" --guard-json words)" 'echo'"${tab}"'a\tb'
eq "words-backslash" "$(words "echo 'a\\b'")" 'echo'"${tab}"'a\\b'
eq "words-powershell" "$(words 'git `clean -X' PowerShell)" "git${tab}clean${tab}-X"
eq "words-heredoc" "$(words $'git commit -F - <<\'MSG\'\nx\nMSG' | wc -c | tr -d ' ')" 0
eq "words-non-bash" "$(words 'echo "a"' Read | wc -c | tr -d ' ')" 0
payload x | "$BIN" --guard-json words sq >/dev/null 2>&1
eq "words-operand-usage" "$?" 2

# --- a raw control byte could forge the reader's own marks: the member blocks naming it, before any
#     rule and with no fall-through, and view prints nothing
forged='{"tool_name":"Bash","tool_input":{"command":"make build \u0001 x"}}'
eq "control-byte-status" "$(member "$tmp/none.knobs" "$forged")" 2
has "control-byte-named" "$tmp/err" "0x01"
has "control-byte-escape" "$tmp/err" "printf '\\x01'"
checks=$((checks + 1))
[[ ! -e "$tmp/friction.log" ]] || fail "control-byte-no-fallthrough" "a fall-through was logged: $(cat "$tmp/friction.log")"
eq "control-byte-view" "$(printf '%s' "$forged" | "$BIN" --guard-json view sq dq hd | wc -c | tr -d ' ')" 0

# --- the consumer lane of --run-guard-tests: the consumer's rows against its own command, spawned
#     from the arm's working directory with the running binary exported; the kit tables are one
#     inert row each, so the lane is what is measured
cat >"$tmp/lane-rule.sh" <<'EOF'
#!/usr/bin/env bash
cmd="$("$GATE_SDK_NATIVE_BIN" --guard-json field .tool_input.command)" || exit 3
case "$cmd" in
    "at $(pwd -P)") exit 2 ;;
    forbidden*) echo "lane: forbidden" >&2; exit 2 ;;
esac
exit 0
EOF
kit_rows="$tmp/kit-rows.tsv"
printf 'fallthrough\tmake build\n' >"$kit_rows"
printf 'fallthrough\tfalse\tmake build\n' >"$tmp/kit-bg-rows.tsv"
: >"$tmp/no-limits.tsv"
lane() {  # $1=knob file [$2=limits table, default an empty one] — the arm from the scratch checkout, output to $tmp/out, the exit status echoed
    (cd "$tmp/cwd" && env -u GATE_SDK_NATIVE_BIN GUARD_KIT_KNOB_FILE="$1" GATE_SDK_KIT_DIRS="$GATE_SDK_ROOT/../guard-kit" \
        "$BIN" --run-guard-tests "$kit_rows" "$tmp/kit-bg-rows.tsv" "$kit_rows" "$tmp/kit-bg-rows.tsv" "${2:-$tmp/no-limits.tsv}" >"$tmp/out" 2>&1)
    echo $?
}
printf 'GUARD_KIT_CONSUMER_RULES_CMD[] = bash\nGUARD_KIT_CONSUMER_RULES_CMD[] = %s\nGUARD_KIT_CONSUMER_CASES = lane.tsv\n' "$tmp/lane-rule.sh" >"$tmp/lane.knobs"
printf 'block\tforbidden x\nfallthrough\tmake build\nblock\tat @ROOT@\n' >"$tmp/cwd/lane.tsv"
eq "lane-green-status" "$(lane "$tmp/lane.knobs")" 0
has "lane-green-count" "$tmp/out" "3 consumer case(s) from lane.tsv"
has "lane-green-derived" "$tmp/out" "36 derived row(s) of the four tables under 0 limit row(s)"
lacks "lane-green-underived" "$tmp/out" "derived re-spelling(s)"
printf 'fallthrough\tforbidden x\nblock\tmake build\n' >"$tmp/cwd/lane.tsv"
eq "lane-mismatch-status" "$(lane "$tmp/lane.knobs")" 1
has "lane-mismatch-unblocked" "$tmp/out" "want 'fallthrough', got 'block' -- [consumer bash $tmp/lane-rule.sh] forbidden x"
has "lane-mismatch-blocked" "$tmp/out" "want 'block', got 'fallthrough' -- [consumer bash $tmp/lane-rule.sh] make build"
printf 'GUARD_KIT_CONSUMER_RULES_CMD[] = bash\nGUARD_KIT_CONSUMER_RULES_CMD[] = %s\n' "$tmp/lane-rule.sh" >"$tmp/lane-off.knobs"
eq "lane-off-status" "$(lane "$tmp/lane-off.knobs")" 0
lacks "lane-off-count" "$tmp/out" "consumer case"
printf 'GUARD_KIT_CONSUMER_CASES = lane.tsv\n' >"$tmp/lane-nocmd.knobs"
eq "lane-nocmd-status" "$(lane "$tmp/lane-nocmd.knobs")" 2
printf 'GUARD_KIT_CONSUMER_RULES_CMD[] = bash\nGUARD_KIT_CONSUMER_RULES_CMD[] = %s\nGUARD_KIT_CONSUMER_CASES = no-such.tsv\n' "$tmp/lane-rule.sh" >"$tmp/lane-nofile.knobs"
eq "lane-nofile-status" "$(lane "$tmp/lane-nofile.knobs")" 2

# --- the consumer lane's derivation: off where GUARD_KIT_CONSUMER_VARIANT_LIMITS is empty (above); a
#     named table greens the re-spellings the consumer's command declines, reds one it does not
#     declare and a row nothing needs, and the knob with no cases table or no readable table is a
#     harness precondition
derive_knobs() {  # $1=name $2=limits path, relative to the scratch checkout — a knob file naming it, its path echoed
    printf 'GUARD_KIT_CONSUMER_RULES_CMD[] = bash\nGUARD_KIT_CONSUMER_RULES_CMD[] = %s\nGUARD_KIT_CONSUMER_CASES = lane.tsv\nGUARD_KIT_CONSUMER_VARIANT_LIMITS = %s\n' \
        "$tmp/lane-rule.sh" "$2" >"$tmp/$1.knobs"
    printf '%s' "$tmp/$1.knobs"
}
printf '# rule `cd_compound` — a section opener, its name the consumer'"'"'s own\nblock\tforbidden x\nfallthrough\tmake build\n' >"$tmp/cwd/lane.tsv"
printf 'fallthrough\tcd_compound\t%s\n' sq-lead dq-lead esc-lead part-lead wrap-command >"$tmp/cwd/lane-limits.tsv"
eq "derive-green-status" "$(lane "$(derive_knobs derive lane-limits.tsv)")" 0
has "derive-green-count" "$tmp/out" "2 consumer case(s) from lane.tsv with 14 derived re-spelling(s) under 5 limit row(s) from lane-limits.tsv"
: >"$tmp/cwd/lane-no-limits.tsv"
eq "derive-undeclared-status" "$(lane "$(derive_knobs undeclared lane-no-limits.tsv)")" 1
has "derive-undeclared-named" "$tmp/out" "want 'block', got 'fallthrough' -- [consumer bash $tmp/lane-rule.sh] [variant sq-lead of rule cd_compound, no limit row] 'forbidden' x"
printf 'fallthrough\tcd_compound\t%s\n' sq-lead dq-lead esc-lead part-lead wrap-command sq-arg >"$tmp/cwd/lane-stale-limits.tsv"
eq "derive-stale-status" "$(lane "$(derive_knobs stale lane-stale-limits.tsv)")" 1
has "derive-stale-named" "$tmp/out" "limit row 'cd_compound sq-arg' admits a re-spelling that no longer fails"
printf 'GUARD_KIT_CONSUMER_RULES_CMD[] = bash\nGUARD_KIT_CONSUMER_RULES_CMD[] = %s\nGUARD_KIT_CONSUMER_VARIANT_LIMITS = lane-limits.tsv\n' "$tmp/lane-rule.sh" >"$tmp/derive-nocases.knobs"
eq "derive-nocases-status" "$(lane "$tmp/derive-nocases.knobs")" 2
has "derive-nocases-named" "$tmp/out" "GUARD_KIT_CONSUMER_VARIANT_LIMITS"
eq "derive-nofile-status" "$(lane "$(derive_knobs nofile no-such-limits.tsv)")" 2
has "derive-nofile-named" "$tmp/out" "GUARD_KIT_CONSUMER_VARIANT_LIMITS"
printf 'block\tcd_compound\tsq-lead\n' >"$tmp/cwd/lane-bad-limits.tsv"
eq "derive-unreadable-status" "$(lane "$(derive_knobs unreadable lane-bad-limits.tsv)")" 2
has "derive-unreadable-named" "$tmp/out" "GUARD_KIT_CONSUMER_VARIANT_LIMITS"

# --- the variant lane reads the fifth positional: a limit row nothing needs is a mismatch, and a
#     table it cannot read is a harness precondition
limits() {  # $1=name $2..=rows — a limits table of those rows, its path echoed
    local f="$tmp/$1.tsv"
    shift
    printf '%b\n' "$@" >"$f"
    printf '%s' "$f"
}
eq "limits-stale-status" "$(lane "$tmp/lane-off.knobs" "$(limits stale 'fallthrough\tcd_compound\tsq-lead')")" 1
has "limits-stale-named" "$tmp/out" "limit row 'cd_compound sq-lead' admits a re-spelling that no longer fails"
eq "limits-floor-status" "$(lane "$tmp/lane-off.knobs" "$(limits floor 'block\tcd_compound\tsq-lead')")" 2
eq "limits-transform-status" "$(lane "$tmp/lane-off.knobs" "$(limits transform 'allow\tcd_compound\tsq')")" 2
eq "limits-twice-status" "$(lane "$tmp/lane-off.knobs" "$(limits twice 'allow\tcd_compound\tsq-lead' 'fallthrough\tcd_compound\tsq-lead')")" 2
eq "limits-missing-status" "$(lane "$tmp/lane-off.knobs" "$tmp/no-such-limits.tsv")" 2

if [[ "$fails" -gt 0 ]]; then
    echo "consumer-rules.test: $fails of $checks assertion(s) failed"
    exit 1
fi
echo "consumer-rules.test: ok ($checks assertions; the consumer command reads the payload bytes as received with the working directory and environment inherited, a block and a decision are relayed verbatim ahead of the generic ruleset, an empty result falls through to it, a fault — non-zero-non-2 exit, unparseable or non-object stdout, a command that cannot start — rides the ruleset's own answer and never silences it, an empty knob runs no stage; --guard-json field reads bytes verbatim, view prints the named view through the tool's reader, nothing for a tool no reader serves, and words prints the word read's values a segment a line, nothing where the read answers nothing, and refuses an operand; a raw control byte is blocked by name before any rule; --run-guard-tests' consumer lane drives the consumer's rows against its command from the arm's working directory, reds a mismatch either way, is silent where no table is named, and refuses a table with no command or no file at exit 2, and derives its rows' re-spellings only under a named limits table, held from both sides and refused at exit 2 with no cases table or no readable table; its variant lane counts the four tables' derived rows, reds a limit row nothing needs and refuses a limits table it cannot read at exit 2)"
exit 0
