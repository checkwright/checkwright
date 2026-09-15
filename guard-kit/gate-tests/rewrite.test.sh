#!/usr/bin/env bash
# spec: guard-kit/SPEC.md §rewrite — the arm's SEAM through the front end, in a git sandbox holding a relocated workflow dir: a two-file literal sweep writes both and exits 0; an outside operand, a symlink, a path through .git and the relocated state file each exit 2 with every file byte-identical, a matching valid operand listed first included; an unrecognized option exits 2, and `--` keeps a `-`-leading <find> reachable. The matcher, report and write cases are pinned in the module's #[cfg(test)] tests.
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

cd "$(git rev-parse --show-toplevel 2>/dev/null || pwd)" || exit 2
ROOT="$(pwd -P)"
RUN="$ROOT/gate-sdk/bin/run-gates.sh"
[[ -x "$RUN" ]] || { echo "rewrite.test: front end not found: $RUN"; exit 2; }

sb="$(mktemp -d)"; outside="$(mktemp -d)"
trap 'rm -rf "$sb" "$outside"' EXIT
git -C "$sb" init -q || { echo "rewrite.test: cannot init the sandbox repo"; exit 2; }
mkdir -p "$sb/wf" "$sb/docs"
export GATE_SDK_WORKFLOW_DIR=wf

fails=0
assert_rc()  { [[ "$2" -eq "$3" ]] || { echo "FAIL [$1]: expected exit $3, got $2"; fails=$((fails + 1)); }; }
assert_has() { grep -qF -- "$2" <<<"$3" || { echo "FAIL [$1]: expected present: $2"; fails=$((fails + 1)); }; }
assert_body() { [[ "$(cat "$2")" == "$3" ]] || { echo "FAIL [$1]: $2 changed"; fails=$((fails + 1)); }; }
rewrite() { (cd "$sb" && bash "$RUN" --rewrite "$@" 2>&1); }

printf 'alpha one\n' > "$sb/a.md"
printf 'beta one\n' > "$sb/docs/b.md"
out="$(rewrite one two a.md docs/b.md)"; rc=$?
assert_rc   sweep "$rc" 0
assert_body sweep "$sb/a.md" 'alpha two'
assert_body sweep "$sb/docs/b.md" 'beta two'
assert_has  sweep 'rewrite: 2 replacement(s) in 2 of 2 file(s)' "$out"
assert_has  sweep '+beta two' "$out"

# Each refusal lists a matching valid operand first: exit 2 must mean nothing was written.
printf 'keep\n' > "$sb/valid.md"
printf 'keep\n' > "$outside/out.md"
out="$(rewrite keep lost valid.md "$outside/out.md")"; rc=$?
assert_rc   refuse-outside "$rc" 2
assert_has  refuse-outside 'outside the working directory' "$out"
assert_body refuse-outside "$sb/valid.md" keep
assert_body refuse-outside "$outside/out.md" keep

ln -s valid.md "$sb/link.md"
out="$(rewrite keep lost valid.md link.md)"; rc=$?
assert_rc   refuse-symlink "$rc" 2
assert_has  refuse-symlink 'a symlink' "$out"
assert_body refuse-symlink "$sb/valid.md" keep

git -C "$sb" config core.rewritetest keep
cp "$sb/.git/config" "$outside/config.before"
out="$(rewrite keep lost valid.md .git/config)"; rc=$?
assert_rc   refuse-git "$rc" 2
assert_has  refuse-git '.git' "$out"
assert_body refuse-git "$sb/valid.md" keep
cmp -s "$sb/.git/config" "$outside/config.before" || { echo "FAIL [refuse-git]: .git/config changed"; fails=$((fails + 1)); }

printf 'keep build\n' > "$sb/wf/WORKFLOW-STATE.txt"
out="$(rewrite keep lost valid.md ./wf/../wf/WORKFLOW-STATE.txt)"; rc=$?
assert_rc   refuse-state "$rc" 2
assert_has  refuse-state 'lifecycle state file' "$out"
assert_body refuse-state "$sb/valid.md" keep
assert_body refuse-state "$sb/wf/WORKFLOW-STATE.txt" 'keep build'

out="$(rewrite -x y valid.md)"; rc=$?
assert_rc   refuse-option "$rc" 2
assert_has  refuse-option 'unrecognized option: -x' "$out"

printf 'flag -x here\n' > "$sb/dash.md"
out="$(rewrite -- -x -y dash.md)"; rc=$?
assert_rc   dash-find "$rc" 0
assert_body dash-find "$sb/dash.md" 'flag -y here'

[[ "$fails" -eq 0 ]] || { echo "rewrite.test: $fails assertion(s) failed"; exit 1; }
echo "rewrite.test: clean (a two-file sweep writes both; outside, symlink, .git and state-file operands refused with nothing written; an unknown option refused; -- reaches a dash-led find)"
exit 0
