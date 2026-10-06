#!/usr/bin/env bash
# spec: lifecycle-kit/SPEC.md §The journal arm — the arm end-to-end in a non-git sandbox: an operand and a standard input land in the journal of the caller's last-stamped stage and never the cursor's, a waiver line is no stage, every refusal exits 2 leaving each journal byte-identical, an absent journal and directory are created whatever the require switch, a retargeted pattern is followed, an absent state file is refused as an unstamped caller, standard input lands byte for byte, and from a subdirectory of a repository every reader of the derivation names one file
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0
note() { echo "  FAIL [$1]: $2"; fails=$((fails + 1)); }

seed() {  # $1=sandbox subdir  $2..=stamp lines
    local sb="$1"; shift
    mkdir -p "$sb/state"
    : >"$sb/lifecycle-config.knobs"
    { printf '# contract: lifecycle-kit/SPEC.md §check-stage-evidence\n\n---\n\n'; printf '%s\n' "$@"; } >"$sb/state/stamps.txt"
}

arm() {  # $1=sandbox  $2=caller id  $3..=the arm's argv; stdin is the caller's
    local sb="$1" id="$2"; shift 2
    ( cd "$sb" && gate_env LIFECYCLE_KIT_KNOB_FILE="$sb/lifecycle-config.knobs" \
                           LIFECYCLE_KIT_STATE_FILE=state/stamps.txt \
                           LIFECYCLE_KIT_STAGE_JOURNAL_PATTERN="${PATTERN:-scratch/<stage>-journal.md}" \
                           LIFECYCLE_KIT_STAGE_JOURNAL_REQUIRE="${REQUIRE:-0}" \
                           LIFECYCLE_KIT_SESSION_ID="$id" \
        && gate_arm_run --emit-journal "$@" 2>&1 )
}

journals() { ( cd "$1" && find . -path ./state -prune -o -type f ! -name lifecycle-config.knobs -print | sort | while read -r f; do echo "== $f"; cat "$f"; done ); }

# --- an operand lands last under the opener's heading, and stdout is the one line ---
sb="$SANDBOX/operand"; seed "$sb" 'demo scope aaaaaaaa 2026-06-01 none' 'demo build deadbeef 2026-06-02 none'
mkdir -p "$sb/scratch"
heading='# stage-journal build — demo deadbeef 2026-06-02 none'
printf '%s\n' "$heading" >"$sb/scratch/build-journal.md"
out="$(arm "$sb" deadbeef01 "a confirmed finding")"; rc=$?
[[ "$rc" -eq 0 ]] || note operand-status "want exit 0, got $rc -- $out"
[[ "$out" == "journal: scratch/build-journal.md build deadbeef" ]] || note operand-stdout "stdout is not the one line: $out"
[[ "$(cat "$sb/scratch/build-journal.md")" == "$heading"$'\n'"a confirmed finding" ]] \
    || note operand-body "the operand did not land verbatim under the heading: $(cat "$sb/scratch/build-journal.md")"

# --- a multi-line standard input lands whole, and DONE as the operand is the last line ---
out="$(printf 'line one\nline two\n' | arm "$sb" deadbeef01)"; rc=$?
[[ "$rc" -eq 0 ]] || note stdin-status "want exit 0 on standard input, got $rc -- $out"
[[ "$(tail -n 2 "$sb/scratch/build-journal.md")" == $'line one\nline two' ]] \
    || note stdin-body "the standard input did not land whole: $(cat "$sb/scratch/build-journal.md")"
out="$(arm "$sb" deadbeef01 DONE)"; rc=$?
[[ "$rc" -eq 0 && "$(tail -n 1 "$sb/scratch/build-journal.md")" == "DONE" ]] \
    || note done-last "DONE is not the file's last line (exit $rc): $(tail -n 2 "$sb/scratch/build-journal.md")"
[[ "$(tail -c 5 "$sb/scratch/build-journal.md" | od -An -c | tr -d ' \n')" == 'DONE\n' ]] \
    || note done-newline "the closing newline was doubled or dropped"

# --- the caller's own stage, never the cursor's ---
sb="$SANDBOX/superseded"; seed "$sb" 'demo build deadbeef 2026-06-02 none' 'demo validate cccccccc 2026-06-03 none'
out="$(arm "$sb" deadbeef01 "a late finding")"; rc=$?
[[ "$rc" -eq 0 && "$out" == "journal: scratch/build-journal.md build deadbeef" ]] \
    || note superseded "a caller the cursor left did not write its own stage's journal (exit $rc): $out"
[[ -e "$sb/scratch/validate-journal.md" ]] && note superseded-cursor "the arm wrote the cursor's journal"

# --- two stamps: the later one's stage; a waiver line carrying the id is no stage ---
sb="$SANDBOX/two-stamps"; seed "$sb" 'demo scope deadbeef 2026-06-01 none' 'demo build deadbeef 2026-06-02 none' 'demo align-waived deadbeef 2026-06-02 none'
out="$(arm "$sb" deadbeef01 "after two stamps")"; rc=$?
[[ "$rc" -eq 0 && "$out" == "journal: scratch/build-journal.md build deadbeef" ]] \
    || note two-stamps "the later stamp's stage was not taken, or the waiver line was read as one (exit $rc): $out"

# --- every refusal exits 2 and leaves every journal byte-identical ---
sb="$SANDBOX/refusals"; seed "$sb" 'demo build deadbeef 2026-06-02 none'
mkdir -p "$sb/scratch"; echo "kept" >"$sb/scratch/build-journal.md"
before="$(journals "$sb")"
refused() {  # $1=label  $2=caller id  $3..=argv
    local label="$1" id="$2" out rc; shift 2
    out="$(arm "$sb" "$id" "$@" </dev/null)"; rc=$?
    [[ "$rc" -eq 2 ]] || note "$label-status" "want exit 2, got $rc -- $out"
    [[ "$(journals "$sb")" == "$before" ]] || note "$label-nowrite" "a refused call wrote a journal"
    REFUSAL="$out"
}
refused unstamped 0badc0de01 "a finding"
grep -qF '0badc0de' <<<"$REFUSAL" || note unstamped-id "the refusal does not name the id: $REFUSAL"
grep -qF -- '--enter-stage <stage>' <<<"$REFUSAL" || note unstamped-remedy "the refusal does not name the remedy: $REFUSAL"
refused empty-operand deadbeef01 ""
refused empty-stdin deadbeef01
refused second-operand deadbeef01 one two
refused dash-led deadbeef01 -led
out="$(arm "$sb" deadbeef01 -- "-led text" </dev/null)"; rc=$?
[[ "$rc" -eq 0 && "$(tail -n 1 "$sb/scratch/build-journal.md")" == "-led text" ]] \
    || note separator "the separator did not admit a dash-led text (exit $rc): $out"

# --- an absent journal and directory are created, whatever the require switch ---
for req in 0 1; do
    sb="$SANDBOX/create-$req"; seed "$sb" 'demo build deadbeef 2026-06-02 none'
    out="$(REQUIRE="$req" arm "$sb" deadbeef01 "first line")"; rc=$?
    [[ "$rc" -eq 0 && "$(cat "$sb/scratch/build-journal.md" 2>/dev/null)" == "first line" ]] \
        || note "create-$req" "the journal and its directory were not created (exit $rc): $out"
done

# --- a retargeted pattern is followed ---
sb="$SANDBOX/pattern"; seed "$sb" 'demo build deadbeef 2026-06-02 none'
out="$(PATTERN='notes/j/<stage>.log' arm "$sb" deadbeef01 "retargeted")"; rc=$?
[[ "$rc" -eq 0 && "$out" == "journal: notes/j/build.log build deadbeef" && "$(cat "$sb/notes/j/build.log" 2>/dev/null)" == "retargeted" ]] \
    || note pattern "a retargeted pattern was not followed (exit $rc): $out"

# --- an absent state file is the unstamped refusal, never an I/O failure ---
sb="$SANDBOX/no-state"; seed "$sb" 'demo build deadbeef 2026-06-02 none'
rm "$sb/state/stamps.txt"
out="$(arm "$sb" deadbeef01 "a finding")"; rc=$?
[[ "$rc" -eq 2 ]] || note no-state-status "want exit 2 with the state file absent, got $rc -- $out"
grep -qF 'deadbeef' <<<"$out" && grep -qF -- '--enter-stage <stage>' <<<"$out" \
    || note no-state-refusal "the refusal does not name the id and the remedy: $out"
[[ -e "$sb/scratch" ]] && note no-state-nowrite "a refused call created the journal directory"

# --- a standard-input byte that is no UTF-8 lands as itself ---
sb="$SANDBOX/bytes"; seed "$sb" 'demo build deadbeef 2026-06-02 none'
out="$(printf 'a\377b\n' | arm "$sb" deadbeef01)"; rc=$?
[[ "$rc" -eq 0 && "$(od -An -b "$sb/scratch/build-journal.md" | tr -d ' \n')" == '141377142012' ]] \
    || note bytes "the byte was replaced or dropped (exit $rc): $(od -An -b "$sb/scratch/build-journal.md" 2>&1)"

# --- from a subdirectory of a repository the opener, the entry assertion and the arm name one file ---
sb="$SANDBOX/subdir"; seed "$sb" 'demo scope aaaaaaaa 2026-06-01 none' 'demo build bbbbbbbb 2026-06-02 none'
mkdir -p "$sb/sub" "$sb/scratch"
printf '# TASK-QUEUE.md\n\n## Iteration: demo\n\n---\n\n## New Features\n\n## Technical Debt\n\n## Done\n' >"$sb/TASK-QUEUE.md"
echo "the predecessor's finding" >"$sb/scratch/build-journal.md"
git -C "$sb" init -q
git -C "$sb" add -A
git -C "$sb" -c user.email=t@t.invalid -c user.name=t -c commit.gpgsign=false commit -qm seed
from_sub() {  # $1.. = the binary's argv, run from the subdirectory under absolute queue and state knobs
    ( cd "$sb/sub" && gate_env LIFECYCLE_KIT_KNOB_FILE="$sb/lifecycle-config.knobs" \
                               GATE_SDK_TMP_DIR=scratch \
                               LIFECYCLE_KIT_QUEUE_FILE="$sb/TASK-QUEUE.md" \
                               LIFECYCLE_KIT_STATE_FILE="$sb/state/stamps.txt" \
                               LIFECYCLE_KIT_STAGE_JOURNAL_PATTERN='scratch/<stage>-journal.md' \
                               LIFECYCLE_KIT_STAGE_JOURNAL_REQUIRE=1 \
                               LIFECYCLE_KIT_SESSION_ID=deadbeef01 \
        && gate_arm_run "$@" 2>&1 )
}
out="$(from_sub --enter-stage validate)"; rc=$?
[[ "$rc" -eq 0 ]] || note subdir-entry "the entry did not read the predecessor journal at the repository root (exit $rc): $out"
out="$(from_sub --emit-journal "from the subdirectory")"; rc=$?
[[ "$rc" -eq 0 ]] || note subdir-arm "the arm refused from a subdirectory (exit $rc): $out"
[[ "$(head -n 1 "$sb/scratch/validate-journal.md" 2>/dev/null)" == '# stage-journal validate — demo deadbeef '* \
   && "$(tail -n 1 "$sb/scratch/validate-journal.md" 2>/dev/null)" == "from the subdirectory" ]] \
    || note subdir-one-file "the opener and the arm did not write one file at the repository root: $(cat "$sb/scratch/validate-journal.md" 2>&1)"
[[ -e "$sb/sub/scratch/validate-journal.md" ]] && note subdir-second-file "a second journal was written under the working directory"

[[ "$fails" -eq 0 ]] || { echo "journal-arm.test: $fails assertion(s) failed"; exit 1; }
echo "journal-arm.test: clean (an operand and a standard input land in the caller's last-stamped stage's journal with the one stdout line, DONE stands last, the cursor's journal and a waiver line are not read, five refusals exit 2 writing nothing, the separator admits a dash-led text, an absent journal is created at either require value, a retargeted pattern is followed, an absent state file is the unstamped refusal, a non-UTF-8 byte lands as itself, and from a subdirectory the opener, the entry assertion and the arm name one file)"
exit 0
