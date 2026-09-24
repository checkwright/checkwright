#!/usr/bin/env bash
# Direct test of the shell guard's consumer stage and of --guard-json, the toolkit a consumer rule
# command calls. Each stage case drives the shell-guard member from a scratch checkout under a
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
eq "view-garbage" "$(printf 'not json' | "$BIN" --guard-json view sq dq hd | wc -c | tr -d ' ')" 0
payload x | "$BIN" --guard-json view body >/dev/null 2>&1
eq "view-body-usage" "$?" 2
payload x | "$BIN" --guard-json view dq >/dev/null 2>&1
eq "view-unknown-usage" "$?" 2

# --- a raw control byte could forge the reader's own marks: the member blocks naming it, before any
#     rule and with no fall-through, and view prints nothing
forged='{"tool_name":"Bash","tool_input":{"command":"make build \u0001 x"}}'
eq "control-byte-status" "$(member "$tmp/none.knobs" "$forged")" 2
has "control-byte-named" "$tmp/err" "0x01"
has "control-byte-escape" "$tmp/err" "printf '\\x01'"
checks=$((checks + 1))
[[ ! -e "$tmp/friction.log" ]] || fail "control-byte-no-fallthrough" "a fall-through was logged: $(cat "$tmp/friction.log")"
eq "control-byte-view" "$(printf '%s' "$forged" | "$BIN" --guard-json view sq dq hd | wc -c | tr -d ' ')" 0

if [[ "$fails" -gt 0 ]]; then
    echo "consumer-rules.test: $fails of $checks assertion(s) failed"
    exit 1
fi
echo "consumer-rules.test: ok ($checks assertions; the consumer command reads the payload bytes as received with the working directory and environment inherited, a block and a decision are relayed verbatim ahead of the generic ruleset, an empty result falls through to it, a fault — non-zero-non-2 exit, unparseable or non-object stdout, a command that cannot start — rides the ruleset's own answer and never silences it, an empty knob runs no stage; --guard-json field reads bytes verbatim and view prints the named view through the tool's reader, nothing for a tool no reader serves; a raw control byte is blocked by name before any rule)"
exit 0
