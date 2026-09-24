#!/usr/bin/env bash
# spec: guard-kit/SPEC.md §Testing — rule `worktree_confinement` from the one place it applies, a linked worktree's working tree, which the decision table's sandbox cannot be: the shell-guard member is driven from a scratch worktree this suite adds and removes, over a main checkout it builds
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

BIN="$GATE_SDK_NATIVE_BIN"
[[ -x "$BIN" ]] || { echo "worktree-confinement.test: the gate binary $BIN is absent — build it first"; exit 2; }
BIN="$(cd "$(dirname "$BIN")" && pwd -P)/$(basename "$BIN")"

SANDBOX="$(cd "$(mktemp -d)" && pwd -P)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0
checks=0

main="$SANDBOX/main"
mkdir -p "$main/docs"
printf 'foo\n' >"$main/docs/a.md"
printf '.tmp/\n.claude/worktrees/\n*.local.md\n' >"$main/.gitignore"
git -C "$main" init -q -b main
git -C "$main" -c user.email=t@example.invalid -c user.name=t add -A
git -C "$main" -c user.email=t@example.invalid -c user.name=t commit -qm seed
mkdir -p "$main/.tmp"
wt="$main/.claude/worktrees/agent-01"
git -C "$main" worktree add -q -b agent-branch "$wt" HEAD

# spec: guard-kit/SPEC.md §Testing — the measured isolated toolset carries no Glob or Grep, so the
# search tools are emptied and rules `find_glob` and `git_grep` leave the searches to rule `worktree_confinement`, as they do there
printf 'GUARD_KIT_SEARCH_TOOLS =\n' >"$SANDBOX/default.knobs"
printf 'GUARD_KIT_SEARCH_TOOLS =\nGUARD_KIT_WORKTREE_READS = off\n' >"$SANDBOX/off.knobs"
printf 'GUARD_KIT_SEARCH_TOOLS =\nGUARD_KIT_RO_BINS[] = grep\nGUARD_KIT_RO_BINS[] = awk\nGUARD_KIT_RO_FORMS[awk] = none\n' >"$SANDBOX/awk.knobs"

tool=Bash
payload() {  # the payload names the tool `tool` holds
    local c="$1"
    c="${c//\\/\\\\}"
    c="${c//\"/\\\"}"
    c="${c//$'\n'/\\n}"
    printf '{"tool_name":"%s","tool_input":{"command":"%s"}}' "$tool" "$c"
}

decide() {  # $1=knob file $2=command — prints block, allow, fallthrough or other:<rc>
    local out rc
    out="$(cd "$wt" && payload "$2" | GUARD_KIT_KNOB_FILE="$1" GATE_SDK_NATIVE_BIN="$BIN" GUARD_KIT_LOG="$SANDBOX/friction.log" "$BIN" --hook shell-guard 2>"$SANDBOX/err")"
    rc=$?
    if [[ "$rc" -eq 2 ]]; then
        echo block
    elif [[ "$rc" -ne 0 ]]; then
        echo "other:$rc"
    elif [[ "$out" == *'"permissionDecision":"allow"'* ]]; then
        echo allow
    elif [[ -z "$out" ]]; then
        echo fallthrough
    else
        echo "other:$out"
    fi
}

want() {  # $1=label $2=knob file $3=want $4=command [$5=block text]
    checks=$((checks + 1))
    local got
    got="$(decide "$2" "$4")"
    if [[ "$got" != "$3" ]]; then
        echo "  FAIL [$1]: '$4' gave $got, want $3 — $(cat "$SANDBOX/err")"
        fails=$((fails + 1))
    elif [[ -n "${5:-}" ]] && ! grep -qF -- "$5" "$SANDBOX/err"; then
        echo "  FAIL [$1]: '$4' blocked without '$5' — $(cat "$SANDBOX/err")"
        fails=$((fails + 1))
    fi
}

d="$SANDBOX/default.knobs"
refusal="reaches the main checkout"

# --- allowed: the journal by shell under the main scratch dir, reads of the main checkout, and a
#     read redirected into the session's own worktree
want journal-append    "$d" allow "printf 'x\n' >> $main/.tmp/journal.md"
want journal-heredoc   "$d" allow "cat >> $main/.tmp/journal.md <<'EOF'"$'\n'"a finding"$'\n'"EOF"
want grep-main         "$d" allow "grep -rn foo $main/docs"
want grep-head-main    "$d" allow "grep -rn foo $main/docs | head -5"
want find-name-main    "$d" allow "find $main/docs -name a.md"
want read-to-own       "$d" fallthrough "grep -rn foo $main/docs > found.txt"

# --- blocked: every write form over the main checkout, however it is spelled
want find-delete       "$d" block "find $main/docs -delete" "$refusal"
want find-fprint       "$d" block "find $main/docs -fprint found.txt" "$refusal"
want touch-main        "$d" block "touch $main/new.txt" "$refusal"
want traversal         "$d" block "touch ../../../new.txt" "$refusal"
want scratch-fold      "$d" block "touch $main/.tmp/../escape.txt" "$refusal"
want ignored-outside   "$d" block "printf 'x\n' >> $main/notes.local.md" "$refusal"
want git-c-amend       "$d" block "git -C $main commit --amend --no-edit" "$refusal"
want read-to-main      "$d" block "grep -rn foo $main/docs > $main/found.txt" "$refusal"

# --- fell through: a write inside the session's own worktree is not this rule's
want touch-own         "$d" fallthrough "touch inside.txt"

# --- the selector: under off, a search of the main checkout blocks and the journal is still granted
want off-grep          "$SANDBOX/off.knobs" block "grep -rn foo $main/docs" "$refusal"
want off-journal       "$SANDBOX/off.knobs" allow "printf 'x\n' >> $main/.tmp/journal.md"

# --- a program-bearing tool is never admitted, whatever the roster declares of it
want awk-declared      "$SANDBOX/awk.knobs" block "awk '{print}' $main/docs/a.md" "$refusal"

# --- rule `script_interpreter` reads the main checkout's scratch dir from here too, and steers the script to the
#     worktree's own scratch dir, since the runner refuses a main-checkout path from a worktree;
#     a script under the worktree's own scratch dir keeps the plain runner steer
want main-scratch-body "$d" block "bash $main/.tmp/x.sh" "write it under the worktree's own scratch dir ($wt/.tmp)"
want main-scratch-rel  "$d" block "bash ../../../.tmp/x.sh" "write it under the worktree's own scratch dir ($wt/.tmp)"
want own-scratch-body  "$d" block "bash .tmp/own.sh" "run a scratch script through the runner: '"

# --- the corrective interpolates the loaded roster rather than carrying a copy
checks=$((checks + 1))
decide "$d" "find $main/docs -delete" >/dev/null
grep -qF "(grep egrep fgrep" "$SANDBOX/err" || { echo "  FAIL [roster-named]: the refusal does not name the loaded roster — $(cat "$SANDBOX/err")"; fails=$((fails + 1)); }

# --- a PowerShell call meets the rule with no admitted read: a backslash is a separator, the
#     journal append under the main scratch dir falls through, and the corrective omits the read
tool=PowerShell
want ps-set-content    "$d" block "Set-Content -Path $main/new.txt -Value x" "$refusal"
want ps-backslash      "$d" block "Set-Content -Path ..\\..\\..\\new.txt -Value x" "$refusal"
want ps-journal        "$d" fallthrough "Add-Content -Path $main/.tmp/journal.md -Value x"
want ps-no-admission   "$d" block "grep -rn foo $main/docs" "$refusal"
checks=$((checks + 1))
! grep -qF "read-only pipeline" "$SANDBOX/err" || { echo "  FAIL [ps-no-admitted-read]: the PowerShell refusal offers the admitted read — $(cat "$SANDBOX/err")"; fails=$((fails + 1)); }
tool=Bash

# --- rules `git_mutation_under_producer`, `background_no_record` and `bounded_wait`'s arm (B) resolve the scratch dirs against the main checkout, the liveness
#     record's one home: a launch recording there is granted or passes, one recording into the own
#     worktree is refused naming the main home, and only a main-checkout record holds a git write
printf '{"permissions":{"allow":["Bash(make *)"]}}\n' >"$SANDBOX/settings.json"
printf 'GUARD_KIT_SEARCH_TOOLS =\nGUARD_KIT_SETTINGS = %s\n' "$SANDBOX/settings.json" >"$SANDBOX/rec.knobs"
r="$SANDBOX/rec.knobs"
launch="make build > $main/.tmp/b.log 2>&1 & echo \"pid=\$! run=b\" >"
want record-main       "$r" allow "$launch $main/.tmp/b.run; wait; rm -f $main/.tmp/b.run"
want record-main-rel   "$r" fallthrough "$launch ../../../.tmp/b.run"
want record-own        "$r" block "$launch .tmp/b.run; wait; rm -f .tmp/b.run" "> $main/.tmp/<key>.run"

sleep 60 &
live=$!
trap 'kill "$live" 2>/dev/null; rm -rf "$SANDBOX"' EXIT
printf 'pid=%s run=wt-live\n' "$live" >"$main/.tmp/wt-live.run"
want producer-main     "$r" block "git commit -m x" "wt-live"
rm -f "$main/.tmp/wt-live.run"
mkdir -p "$wt/.tmp"
printf 'pid=%s run=own-live\n' "$live" >"$wt/.tmp/own-live.run"
want producer-own      "$r" fallthrough "git commit -m x"

git -C "$main" worktree remove --force "$wt"

[[ "$fails" -eq 0 ]] || { echo "worktree-confinement.test: $fails of $checks assertion(s) failed"; exit 1; }
echo "worktree-confinement.test: ok ($checks assertions; from a linked worktree the journal append under the main scratch dir and read-only searches of the main checkout pass, every write naming the main checkout outside its scratch dir is refused with the loaded roster named, a PowerShell call meets the same refusal with no admitted read, a write in the own worktree is not the rule's, 'off' refuses the search and keeps the journal, a program-bearing tool is never admitted, and the liveness record resolves to the main checkout's scratch dir for the launch, its grant and the git-write hold)"
exit 0
