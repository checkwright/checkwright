#!/usr/bin/env bash
# Direct unit test of lib/guard.sh's guard_read_path — the file-path counterpart
# of guard_read_command. The absent-field case is the discriminating one: an
# implementation returning success on a missing file_path passes the happy path
# and wedges every call its matcher covers, because a consumer reads the
# non-zero return as "not my call, fall through".
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
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

if [[ "$fails" -gt 0 ]]; then
    echo "guard-read-path.test: $fails of $checks assertion(s) failed"
    exit 1
fi
echo "guard-read-path.test: ok ($checks assertions; path extraction plus the absent/unparseable/empty fall-through contract)"
exit 0
