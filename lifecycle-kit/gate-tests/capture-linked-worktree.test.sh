#!/usr/bin/env bash
# spec: lifecycle-kit/SPEC.md §The committed gap inbox — the two tracked capture arms run from a real linked worktree of a sandbox repository: each refuses at exit 2 with the hand-back steer and leaves the record absent in both the worktree and the main checkout, and each files from the main checkout. The refusal's text is pinned in native/src/emit/file_survey.rs's own #[cfg(test)] tests; the crate may not add a linked worktree itself, so the real one lives here.
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0
note() { echo "  FAIL [$1]: $2"; fails=$((fails + 1)); }

main="$SANDBOX/main"
mkdir -p "$main"
printf 'seed\n' >"$main/seed.txt"
git -C "$main" init -q
git -C "$main" add -A
git -C "$main" -c user.email=t@t.invalid -c user.name=t -c commit.gpgsign=false commit -qm seed
wt="$SANDBOX/wt"
git -C "$main" worktree add -q -b agent-branch "$wt" HEAD

run_in() {  # $1=dir  $2.. = arm argv -> stdout+stderr, exit code preserved
    ( cd "$1" && gate_arm_run "${@:2}" 2>&1 )
}

# --- from the linked worktree: both arms refuse, steer the hand-back, and write nowhere ---
for arm in gap survey; do
    if [[ "$arm" == gap ]]; then
        out="$(run_in "$wt" --emit-file-gap "a gap an isolated child found")"; rc=$?
        rec=".workflow/gap-inbox.md"
    else
        out="$(run_in "$wt" --emit-file-survey "q" "seed.txt" "o" "none" "f")"; rc=$?
        rec=".workflow/survey-record.md"
    fi
    [[ "$rc" -eq 2 ]] || note "$arm-status" "want exit 2 from a linked worktree, got $rc -- $out"
    grep -qF "linked worktree of" <<<"$out" || note "$arm-names-main" "the refusal does not name the main checkout: $out"
    grep -qF "hands the finding back to its dispatcher" <<<"$out" || note "$arm-steer" "no hand-back steer: $out"
    [[ -e "$wt/$rec" ]] && note "$arm-wt-write" "the arm wrote $rec in the worktree on a refusal"
    [[ -e "$main/$rec" ]] && note "$arm-main-write" "the arm routed $rec into the main checkout"
done

# --- from the main checkout: both arms file as before ---
out="$(run_in "$main" --emit-file-gap "a gap the dispatcher files")"; rc=$?
[[ "$rc" -eq 0 ]] || note gap-main "want exit 0 from the main checkout, got $rc -- $out"
grep -qF -- "— a gap the dispatcher files" "$main/.workflow/gap-inbox.md" 2>/dev/null \
    || note gap-main-append "the main checkout's inbox lacks the bullet"
out="$(run_in "$main" --emit-file-survey "does the main checkout file" "seed.txt" "o" "none" "f")"; rc=$?
[[ "$rc" -eq 0 ]] || note survey-main "want exit 0 from the main checkout, got $rc -- $out"
grep -qF -- "— does the main checkout file" "$main/.workflow/survey-record.md" 2>/dev/null \
    || note survey-main-append "the main checkout's record lacks the block"

[[ "$fails" -eq 0 ]] || { echo "capture-linked-worktree.test: $fails assertion(s) failed"; exit 1; }
echo "capture-linked-worktree.test: clean (file-gap and file-survey refuse in a linked worktree with the hand-back steer and write the record in neither checkout, and both file from the main checkout)"
exit 0
