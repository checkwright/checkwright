#!/usr/bin/env bash
# Behavioral test of templates/workflow-state-guard.sh: it blocks a Write/Edit
# whose target resolves to the lifecycle state file and falls through on
# everything else. The path-equality cases are the point — a textual comparison
# passes the first and fails the rest, and that is exactly how this guard is
# defeated in the field.
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # lifecycle-kit/
GUARD="$DIR/templates/workflow-state-guard.sh"
LIB="$DIR/../guard-kit/lib/guard.sh"
command -v jq >/dev/null 2>&1 || { echo "workflow-state-guard.test: jq not found on PATH" >&2; exit 2; }

SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT
mkdir -p "$SANDBOX/.workflow" "$SANDBOX/docs"
printf '# contract\n\n---\n\niter build abc 2026-01-01\n' >"$SANDBOX/.workflow/WORKFLOW-STATE.txt"
printf 'ordinary\n' >"$SANDBOX/docs/page.md"
# a symlinked route to the same directory: same file, different spelling
ln -s .workflow "$SANDBOX/wf-link"

fails=0
checks=0

verdict() {   # $1=json-payload -> "block" | "pass" | "advise"
    local out rc
    out="$(cd "$SANDBOX" && printf '%s' "$1" | GUARD_KIT_LIB="$LIB" bash "$GUARD" 2>/dev/null)"; rc=$?
    if [[ "$rc" -eq 2 ]]; then echo block; return; fi
    if grep -q '"additionalContext"' <<<"$out"; then echo advise; return; fi
    if [[ "$rc" -eq 0 ]]; then echo pass; return; fi
    echo "exit$rc"
}

want() {   # $1=label $2=file_path $3=expected-verdict
    local got
    checks=$((checks + 1))
    got="$(verdict "$(jq -nc --arg p "$2" '{tool_name:"Write",tool_input:{file_path:$p,content:"x"}}')")"
    [[ "$got" == "$3" ]] || { echo "  FAIL [$1]: '$2' -> $got, want $3"; fails=$((fails + 1)); }
}

# the plain spelling
want "relative"        ".workflow/WORKFLOW-STATE.txt"            block
# the three spellings a textual comparison misses
want "dot-slash"       "./.workflow/WORKFLOW-STATE.txt"          block
want "absolute"        "$SANDBOX/.workflow/WORKFLOW-STATE.txt"   block
want "through-symlink" "wf-link/WORKFLOW-STATE.txt"              block
want "dot-dot"         ".workflow/../.workflow/WORKFLOW-STATE.txt" block

# ordinary files are untouched — the guard must not become a general write block
want "ordinary-file"   "docs/page.md"                            pass
want "sibling-in-dir"  ".workflow/gap-inbox.md"                  pass
# a name that merely contains the target's is not the target
want "prefix-collision" ".workflow/WORKFLOW-STATE.txt.bak"       pass

# a call with no file_path falls through rather than blocking
checks=$((checks + 1))
got="$(verdict '{"tool_name":"Bash","tool_input":{"command":"ls"}}')"
[[ "$got" == pass ]] || { echo "  FAIL [no-file_path]: got $got, want pass"; fails=$((fails + 1)); }

# an Edit payload is judged the same way as a Write — the matcher covers both
checks=$((checks + 1))
got="$(verdict "$(jq -nc --arg p ".workflow/WORKFLOW-STATE.txt" \
    '{tool_name:"Edit",tool_input:{file_path:$p,old_string:"a",new_string:"b"}}')")"
[[ "$got" == block ]] || { echo "  FAIL [edit-payload]: got $got, want block"; fails=$((fails + 1)); }

# the relocation knob is honored: a consumer who moved the workflow dir gets a
# guard that follows it, and the old path stops being the state file
checks=$((checks + 1))
mkdir -p "$SANDBOX/wf2"; printf 'x\n' >"$SANDBOX/wf2/WORKFLOW-STATE.txt"
out="$(cd "$SANDBOX" && jq -nc '{tool_name:"Write",tool_input:{file_path:"wf2/WORKFLOW-STATE.txt",content:"x"}}' \
    | GATE_SDK_WORKFLOW_DIR=wf2 GUARD_KIT_LIB="$LIB" bash "$GUARD" 2>/dev/null)"; rc=$?
[[ "$rc" -eq 2 ]] || { echo "  FAIL [relocated-dir]: got exit $rc, want 2 ($out)"; fails=$((fails + 1)); }

# fail-open-but-loud: with the rule unenforceable the call is allowed and the
# advisory names the rule, rather than passing silently or wedging every write
checks=$((checks + 1))
# PATH is emptied of jq and readlink; bash is invoked by its absolute path so the
# stripped PATH cannot be mistaken for the interpreter itself going missing.
mkdir -p "$SANDBOX/nojq"
BASH_ABS="$(command -v bash)"
out="$(cd "$SANDBOX" && jq -nc '{tool_name:"Write",tool_input:{file_path:".workflow/WORKFLOW-STATE.txt",content:"x"}}' \
    | PATH="$SANDBOX/nojq" GUARD_KIT_LIB="$LIB" "$BASH_ABS" "$GUARD" 2>/dev/null)"; rc=$?
{ [[ "$rc" -eq 0 ]] && grep -q '"additionalContext"' <<<"$out"; } \
    || { echo "  FAIL [jq-absent]: want exit 0 with an advisory, got exit $rc: $out"; fails=$((fails + 1)); }

# an absent guard-kit lib is a clean fall-through, never a wedge
checks=$((checks + 1))
( cd "$SANDBOX" && printf '%s' '{"tool_input":{"file_path":".workflow/WORKFLOW-STATE.txt"}}' \
    | GUARD_KIT_LIB="$SANDBOX/nope.sh" bash "$GUARD" >/dev/null 2>&1 )
[[ "$?" -eq 0 ]] || { echo "  FAIL [absent-lib]: expected a fall-through exit 0"; fails=$((fails + 1)); }

if [[ "$fails" -gt 0 ]]; then
    echo "workflow-state-guard.test: $fails of $checks assertion(s) failed"
    exit 1
fi
echo "workflow-state-guard.test: ok ($checks assertions; resolved path equality across four spellings, Write and Edit, the relocation knob, and both degradation postures)"
exit 0
