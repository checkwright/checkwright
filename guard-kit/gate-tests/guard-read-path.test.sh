#!/usr/bin/env bash
# Direct unit test of lib/guard.sh's guard_read_path — the file-path counterpart
# of guard_read_command. The absent-field case is the discriminating one: an
# implementation returning success on a missing file_path passes the happy path
# and wedges every call its matcher covers, because a consumer reads the
# non-zero return as "not my call, fall through".
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # guard-kit/
# shellcheck source=../lib/guard.sh
source "$DIR/lib/guard.sh"

fails=0
checks=0

eq() {  # $1=label $2=got $3=want
    checks=$((checks + 1))
    [[ "$2" == "$3" ]] || { echo "  FAIL [$1]: got '$2', want '$3'"; fails=$((fails + 1)); }
}

# a Write payload: the path comes back verbatim, status 0
got="$(guard_read_path <<<'{"tool_name":"Write","tool_input":{"file_path":".workflow/WORKFLOW-STATE.txt","content":"x"}}')"; rc=$?
eq "write-path"   "$got" ".workflow/WORKFLOW-STATE.txt"
eq "write-status" "$rc"  "0"

# an Edit payload carries the same field — the accessor is tool-agnostic
got="$(guard_read_path <<<'{"tool_name":"Edit","tool_input":{"file_path":"/abs/x.md","old_string":"a","new_string":"b"}}')"
eq "edit-path" "$got" "/abs/x.md"

# the discriminating case: no file_path -> non-zero and no output
got="$(guard_read_path <<<'{"tool_name":"Bash","tool_input":{"command":"ls"}}')"; rc=$?
eq "absent-status" "$rc"  "1"
eq "absent-output" "$got" ""

# an unparseable payload is the same fall-through, never a block
got="$(guard_read_path <<<'not json at all')"; rc=$?
eq "garbage-status" "$rc"  "1"
eq "garbage-output" "$got" ""

# an empty file_path is absent, not a path
got="$(guard_read_path <<<'{"tool_input":{"file_path":""}}')"; rc=$?
eq "empty-status" "$rc" "1"

# the binary writes LF on every host, so an allow entry reads back as its own text: one whose JSON
# ends in a "\r" keeps it and so is no `Bash(...)` entry, and no other entry gains one
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT
printf '{"permissions":{"allow":["Bash(git status)\\r","Bash(ls)","Read"]}}\n' >"$tmp/settings.json"
got="$(GUARD_KIT_SETTINGS="$tmp/settings.json" _guard_allow_inners)"
eq "allow-inners-verbatim" "$got" "ls"
printf '{"permissions":{"allow":["Bash(git status)","Bash(ls)"]}}\n' >"$tmp/plain.json"
got="$(GUARD_KIT_SETTINGS="$tmp/plain.json" _guard_allow_inners)"
eq "allow-inners-lf" "$got" "$(printf 'git status\nls')"
[[ "$got" != *$'\r'* ]] || eq "allow-inners-no-cr" "a CR" "none"

# the three substitution readers hand the ruleset a payload's own bytes: a JSON "\r\n" reads back
# verbatim, and a payload carrying no CR reads back with none
crlf='{"tool_input":{"command":"cat <<EOF\r\nline\r\nEOF\r\n","file_path":"a\r\nb\r\n","run_in_background":"true\r\n"}}'
eq "command-crlf" "$(guard_read_command <<<"$crlf")" "$(printf 'cat <<EOF\r\nline\r\nEOF\r')"
eq "path-crlf" "$(guard_read_path <<<"$crlf")" "$(printf 'a\r\nb\r')"
GUARD_INPUT="$crlf"
eq "field-crlf" "$(guard_input_field '.tool_input.run_in_background')" "$(printf 'true\r')"
unset GUARD_INPUT
lf='{"tool_input":{"command":"cat <<EOF\nline\nEOF\n","file_path":"a\nb","run_in_background":true}}'
for got in "$(guard_read_command <<<"$lf")" "$(guard_read_path <<<"$lf")" "$(GUARD_INPUT="$lf" guard_input_field '.tool_input.run_in_background')"; do
    [[ "$got" != *$'\r'* ]] || eq "reader-added-cr" "a CR" "none"
done
eq "command-lf" "$(guard_read_command <<<"$lf")" "$(printf 'cat <<EOF\nline\nEOF')"
GUARD_INPUT="$lf"
eq "field-bool" "$(guard_input_field '.tool_input.run_in_background')" "true"
eq "field-object" "$(guard_input_field '.tool_input')" ""
eq "field-filter" "$(guard_input_field '.tool_input.command | length')" ""
unset GUARD_INPUT

# the unreachable-binary advisory is a fixed literal, so it is parsed here: the binary it cannot
# reach at hook time is the parser this test has
adv="$(GATE_SDK_NATIVE_BIN=/nonexistent/checkwright-gates bash -c 'source "$1"' _ "$DIR/lib/guard.sh" </dev/null)"; rc=$?
eq "unreachable-status" "$rc" "0"
ctx="$(printf '%s' "$adv" | "$_guard_bin" --guard-json field '.hookSpecificOutput.additionalContext')"
[[ "$ctx" == "guard-kit's rules did not run on this call"* ]] || eq "unreachable-literal-parses" "$adv" "a JSON advisory"
eq "unreachable-event" "$(printf '%s' "$adv" | "$_guard_bin" --guard-json field '.hookSpecificOutput.hookEventName')" "PreToolUse"

if [[ "$fails" -gt 0 ]]; then
    echo "guard-read-path.test: $fails of $checks assertion(s) failed"
    exit 1
fi
echo "guard-read-path.test: ok ($checks assertions; path extraction plus the absent/unparseable/empty fall-through contract, every reader's payload bytes read back verbatim, and the unreachable-binary advisory parsed as JSON)"
exit 0
