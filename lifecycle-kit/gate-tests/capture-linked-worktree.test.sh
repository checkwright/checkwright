#!/usr/bin/env bash
# spec: lifecycle-kit/SPEC.md §The committed gap inbox — the three tracked capture arms run from a real linked worktree of a sandbox repository: each refuses at exit 2 with the hand-back steer and leaves the record absent in both the worktree and the main checkout, and each files from the main checkout. The refusal's text is pinned in native/src/emit/file_survey.rs's own #[cfg(test)] tests; the crate may not add a linked worktree itself, so the real one lives here.
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

# --- from the linked worktree: every arm refuses, steers the hand-back, and writes nowhere ---
for arm in gap consult survey; do
    if [[ "$arm" == gap ]]; then
        out="$(run_in "$wt" --emit-file-gap "a gap an isolated child found")"; rc=$?
        rec=".workflow/gap-inbox.md"
    elif [[ "$arm" == consult ]]; then
        out="$(run_in "$wt" --emit-file-consult "an item an isolated child found")"; rc=$?
        rec=".workflow/consult-items.md"
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

# --- from the main checkout: every arm files ---
out="$(run_in "$main" --emit-file-gap "a gap the dispatcher files")"; rc=$?
[[ "$rc" -eq 0 ]] || note gap-main "want exit 0 from the main checkout, got $rc -- $out"
grep -qF -- "— a gap the dispatcher files" "$main/.workflow/gap-inbox.md" 2>/dev/null \
    || note gap-main-append "the main checkout's inbox lacks the bullet"
out="$(run_in "$main" --emit-file-consult "an item the dispatcher files")"; rc=$?
[[ "$rc" -eq 0 ]] || note consult-main "want exit 0 from the main checkout, got $rc -- $out"
grep -qF -- "— an item the dispatcher files" "$main/.workflow/consult-items.md" 2>/dev/null \
    || note consult-main-append "the main checkout's consult inbox lacks the bullet"
grep -qxF -- "# contract: lifecycle-kit/SPEC.md §The consult inbox — append-only capture of items owed to the consult skill, consult-drained; one bullet per item below." "$main/.workflow/consult-items.md" 2>/dev/null \
    || note consult-main-header "a fresh consult inbox was not seeded with its contract header"
out="$(run_in "$main" --emit-file-survey "does the main checkout file" "seed.txt" "o" "none" "f")"; rc=$?
[[ "$rc" -eq 0 ]] || note survey-main "want exit 0 from the main checkout, got $rc -- $out"
grep -qF -- "— does the main checkout file" "$main/.workflow/survey-record.md" 2>/dev/null \
    || note survey-main-append "the main checkout's record lacks the block"

[[ "$fails" -eq 0 ]] || { echo "capture-linked-worktree.test: $fails assertion(s) failed"; exit 1; }
echo "capture-linked-worktree.test: clean (file-gap, file-consult and file-survey refuse in a linked worktree with the hand-back steer and write the record in neither checkout, and each files from the main checkout)"
exit 0
