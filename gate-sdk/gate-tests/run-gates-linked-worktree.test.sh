#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §The harness-integration arm — a fail-open arm run inside a linked worktree with no binary of its own dispatches to the main checkout's binary: the fork ban, the workflow-state guard and the bash guard fire from the worktree; with both binaries absent the hook still declines at 0; a verdict-bearing arm still reports the binary absent at 2; and an absolute pin naming nothing takes no fallback
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd -P)"
# shellcheck source=../lib/gate.sh
source "$HERE/gate-sdk/lib/gate.sh"
PINNED="$GATE_SDK_NATIVE_BIN"
[[ -x "$PINNED" ]] || { echo "run-gates-linked-worktree.test: the gate binary $PINNED is absent — build it first"; exit 2; }

SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0
note() { echo "  FAIL [$1]: $2"; fails=$((fails + 1)); }

main="$SANDBOX/main"
mkdir -p "$main/gate-sdk" "$main/guard-kit"
cp -R "$HERE/gate-sdk/bin" "$HERE/gate-sdk/lib" "$main/gate-sdk/"
cp -R "$HERE/guard-kit/lib" "$main/guard-kit/"
cp "$HERE/guard-kit/templates/bash-guard.sh" "$main/bash-guard.sh"
mkdir -p "$main/.workflow"
: >"$main/.workflow/WORKFLOW-STATE.txt"
git -C "$main" init -q -b main
git -C "$main" -c user.email=t@example.invalid -c user.name=t add -A
git -C "$main" -c user.email=t@example.invalid -c user.name=t commit -qm seed
wt="$main/.claude/worktrees/agent-01"
git -C "$main" worktree add -q -b agent-branch "$wt" HEAD

default_bin="$main/native/target/release/checkwright-gates$(gate_exe_suffix)"
mkdir -p "${default_bin%/*}"
cp "$PINNED" "$default_bin"

in_wt() {  # $1=stdin  $2..=command — run from the worktree with the knob unset, so the default resolves
    local input="$1"
    shift
    ( cd "$wt" && unset GATE_SDK_NATIVE_BIN && "$@" <<<"$input" 2>&1 )
}

# 1. the fork ban fires from the worktree
out="$(in_wt '{"tool_input":{"subagent_type":"fork","prompt":"x"}}' bash gate-sdk/bin/run-gates.sh --hook agent-dispatch-guard)"; rc=$?
[[ "$rc" -eq 2 ]] || note fork-status "want exit 2 on a fork dispatch from a linked worktree, got $rc -- $out"
grep -qF 'a fork inherits' <<<"$out" || note fork-text "the fork-ban text is missing: $out"

# 2. the workflow-state guard fires from the worktree
out="$(in_wt '{"tool_input":{"file_path":".workflow/WORKFLOW-STATE.txt","content":"x"}}' bash gate-sdk/bin/run-gates.sh --hook workflow-state-guard)"; rc=$?
[[ "$rc" -eq 2 ]] || note state-status "want exit 2 on a state-file Write from a linked worktree, got $rc -- $out"

# 3. the bash guard runs its rules from the worktree rather than advising they did not run
out="$(in_wt '{"tool_input":{"command":"cd deploy && ls"}}' bash bash-guard.sh)"; rc=$?
[[ "$rc" -eq 2 ]] || note guard-status "want exit 2 from the bash guard on a blocked command, got $rc -- $out"
grep -qF 'rules did not run' <<<"$out" && note guard-advisory "the bash guard took the unreachable-binary advisory: $out"

# 5. a verdict-bearing arm links nothing onto a door path git does not ignore: exit 2, naming why
out="$(in_wt '' bash gate-sdk/bin/run-gates.sh --emit knob-roster)"; rc=$?
[[ "$rc" -eq 2 ]] || note emit-status "want exit 2 from --emit with an unignored door path, got $rc -- $out"
grep -qF 'is not gitignored here' <<<"$out" || note emit-text "--emit did not name the unignored door: $out"
grep -qF 'Build it' <<<"$out" && note emit-remedy "the worktree refusal told the session to build: $out"
[[ -e "$wt/native/target/release/checkwright-gates$(gate_exe_suffix)" ]] && note emit-linked "a binary was linked onto an unignored door path"

# 7. with the door path ignored and no crate source tracked, the main checkout's installed binary is linked and runs
printf 'native/target/\n' >"$wt/.gitignore"
out="$(in_wt '' bash gate-sdk/bin/run-gates.sh --emit knob-values GATE_SDK_NATIVE_BIN)"; rc=$?
[[ "$rc" -eq 0 ]] || note payload-status "want exit 0 through the linked main binary, got $rc -- $out"
grep -qF 'GATE_SDK_NATIVE_BIN' <<<"$out" || note payload-text "the linked binary did not answer: $out"
rm -rf "$wt/native" "$wt/.gitignore"

# 6. an absolute pin naming nothing takes no fallback: the hook declines at 0 on its own line
out="$( cd "$wt" && GATE_SDK_NATIVE_BIN="$SANDBOX/nowhere/gates" bash gate-sdk/bin/run-gates.sh --hook agent-dispatch-guard <<<'{"tool_input":{"subagent_type":"fork"}}' 2>&1 )"; rc=$?
[[ "$rc" -eq 0 ]] || note pin-status "want exit 0 on an absolute pin naming nothing, got $rc -- $out"
grep -qF "$SANDBOX/nowhere/gates is absent or not executable" <<<"$out" || note pin-text "the absent line does not name the pin: $out"

# 4. the control: with the main checkout's binary removed too, the hook declines at 0
rm -f "$default_bin"
out="$(in_wt '{"tool_input":{"subagent_type":"fork"}}' bash gate-sdk/bin/run-gates.sh --hook agent-dispatch-guard)"; rc=$?
[[ "$rc" -eq 0 ]] || note control-status "want exit 0 with both binaries absent, got $rc -- $out"
grep -qF 'is absent or not executable' <<<"$out" || note control-text "the absent-binary line is missing: $out"

git -C "$main" worktree remove --force "$wt"

# 8-9. a tree tracking the crate source: a main binary whose source stamp is the worktree's is linked
#      and runs, one line of skew refuses at 2 without linking, and neither builds
crate="$SANDBOX/crate"
mkdir -p "$crate/gate-sdk"
cp -R "$HERE/gate-sdk/bin" "$HERE/gate-sdk/lib" "$crate/gate-sdk/"
while IFS= read -r f; do
    mkdir -p "$crate/${f%/*}"
    cp "$HERE/$f" "$crate/$f"
done < <(git -C "$HERE" ls-files native)
printf 'target/\n' >"$crate/.gitignore"
git -C "$crate" init -q -b main
git -C "$crate" -c user.email=t@example.invalid -c user.name=t add -A
git -C "$crate" -c user.email=t@example.invalid -c user.name=t commit -qm seed
cwt="$crate/.claude/worktrees/agent-02"
git -C "$crate" worktree add -q -b agent-branch "$cwt" HEAD
crate_bin="$crate/native/target/release/checkwright-gates$(gate_exe_suffix)"
mkdir -p "${crate_bin%/*}"
cp "$PINNED" "$crate_bin"
door="$cwt/native/target/release/checkwright-gates$(gate_exe_suffix)"

out="$( cd "$cwt" && unset GATE_SDK_NATIVE_BIN && bash gate-sdk/bin/run-gates.sh --emit knob-values GATE_SDK_NATIVE_BIN 2>&1 )"; rc=$?
[[ "$rc" -eq 0 ]] || note stamp-match-status "want exit 0 through a stamp-matched main binary, got $rc -- $out"
[[ -x "$door" ]] || note stamp-match-link "the stamp-matched main binary was not linked at the door path"
[[ -e "$cwt/native/target/release/deps" ]] && note stamp-match-build "a build ran in the worktree"

rm -f "$door"
printf '// skew\n' >>"$cwt/native/src/main.rs"
out="$( cd "$cwt" && unset GATE_SDK_NATIVE_BIN && bash gate-sdk/bin/run-gates.sh --emit knob-values GATE_SDK_NATIVE_BIN 2>&1 )"; rc=$?
[[ "$rc" -eq 2 ]] || note skew-status "want exit 2 on a skewed crate, got $rc -- $out"
grep -qF "was not built from this worktree's crate source" <<<"$out" || note skew-text "the refusal did not name the skew: $out"
[[ -e "$door" ]] && note skew-link "a skewed main binary was linked"
[[ -e "$cwt/native/target/release/deps" ]] && note skew-build "a build ran in the worktree"
git -C "$crate" worktree remove --force "$cwt"

[[ "$fails" -eq 0 ]] || { echo "run-gates-linked-worktree.test: $fails assertion(s) failed"; exit 1; }
echo "run-gates-linked-worktree.test: clean (from a linked worktree with no binary the fork ban, the workflow-state guard and the bash guard fire through the main checkout's binary; a verdict arm links the main binary only onto a gitignored door and, where crate source is tracked, only on a matching source stamp, refusing at 2 with the reason otherwise and never building; an absolute pin naming nothing takes no fallback; with both absent the hook declines at 0)"
exit 0
