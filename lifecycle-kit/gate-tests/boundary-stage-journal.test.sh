#!/usr/bin/env bash
# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the journal opener and the predecessor assertion end-to-end through a sandboxed enter-stage: the derived path is a function of the stage, the default REQUIRE=0 opens nothing and asserts nothing, REQUIRE=1 opens the journal at the stamp and refuses on an absent, empty or opened-but-unwritten predecessor journal writing no stamp, --simulate relays the refusal and writes no journal, the escape clears it, the opener appends rather than overwrites and runs after the boundary wipe, the first stage of an iteration is never asserted against, a second session of one stage asserts the first session's journal at the same path, and a pattern with no <stage> placeholder is refused as config
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # lifecycle-kit/
ENTER="$DIR/bin/enter-stage.sh"
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0
note() { echo "  FAIL [$1]: $2"; fails=$((fails + 1)); }

seed() {  # $1=sandbox subdir; a consumer whose cursor is 'build', predecessor of nothing but itself
    local sb="$1"
    mkdir -p "$sb/.workflow" "$sb/scratch"
    cat >"$sb/TASK-QUEUE.md" <<'EOF'
# TASK-QUEUE.md

## Iteration: demo-iteration

---

## New Features

## Technical Debt

## Done
EOF
    cat >"$sb/.workflow/WORKFLOW-STATE.txt" <<'EOF'
# contract: lifecycle-kit/SPEC.md §check-stage-evidence

---

demo-iteration scope aaaaaaaa 2026-06-01 none
demo-iteration build bbbbbbbb 2026-06-02 none
EOF
    : >"$sb/lifecycle-config.sh"
}

run_enter() {  # $1=sandbox  $2=REQUIRE  $3...=enter-stage argv
    local sb="$1" req="$2"; shift 2
    ( cd "$sb" && env LIFECYCLE_KIT_CONFIG_FILE="$sb/lifecycle-config.sh" \
                      GATE_SDK_TMP_DIR=scratch \
                      LIFECYCLE_KIT_SESSION_ID=deadbeef01 \
                      LIFECYCLE_KIT_STAGE_JOURNAL_REQUIRE="$req" \
                      bash "$ENTER" "$@" 2>&1 )
}

state_of() { cat "$1/.workflow/WORKFLOW-STATE.txt"; }

# --- the DEFAULT asserts nothing and opens nothing: a consumer who has not taken the feature gains no file ---
sb="$SANDBOX/default-off"; seed "$sb"
out="$( cd "$sb" && env LIFECYCLE_KIT_CONFIG_FILE="$sb/lifecycle-config.sh" \
                        GATE_SDK_TMP_DIR=scratch LIFECYCLE_KIT_SESSION_ID=deadbeef01 \
                        bash "$ENTER" validate 2>&1 )"; rc=$?
[[ "$rc" -eq 0 ]] || note default-off "the default REQUIRE refused an entry with no journal: $out"
[[ -e "$sb/scratch/validate-journal.md" ]] && note default-off-opener "the opener wrote a journal at the kit default REQUIRE=0"

# --- REQUIRE=1, predecessor journal ABSENT: refuse, name the derived path, write nothing ---
sb="$SANDBOX/absent"; seed "$sb"
before="$(state_of "$sb")"
out="$(run_enter "$sb" 1 validate)"; rc=$?
[[ "$rc" -eq 1 ]] || note absent-status "want exit 1 with the predecessor journal absent, got $rc -- $out"
grep -qF 'scratch/build-journal.md' <<<"$out" || note absent-path "the refusal does not name the derived path: $out"
grep -qF "predecessor stage 'build'" <<<"$out" || note absent-pred "the refusal does not name the predecessor: $out"
grep -qF 'does not exist' <<<"$out"            || note absent-why "the refusal does not say why: $out"
[[ "$(state_of "$sb")" == "$before" ]]         || note absent-nowrite "a refused entry wrote to the state file"

# --- --simulate relays the same refusal and writes nothing ---
out="$(run_enter "$sb" 1 --simulate validate)"; rc=$?
[[ "$rc" -eq 1 ]] || note sim-status "want exit 1 under --simulate, got $rc -- $out"
grep -qF 'enter-stage (simulate)' <<<"$out"    || note sim-prefix "the simulate transcript is unprefixed: $out"
grep -qF 'scratch/build-journal.md' <<<"$out"  || note sim-path "the simulate refusal does not name the path: $out"
[[ "$(state_of "$sb")" == "$before" ]]         || note sim-nowrite "--simulate wrote to the state file"

# --- REQUIRE=1, predecessor journal EMPTY: refused too, and told apart from absent ---
sb="$SANDBOX/empty"; seed "$sb"
mkdir -p "$sb/scratch"; : >"$sb/scratch/build-journal.md"
out="$(run_enter "$sb" 1 validate)"; rc=$?
[[ "$rc" -eq 1 ]] || note empty-status "an empty predecessor journal did not refuse: $out"
grep -qF 'is empty' <<<"$out"        || note empty-why "an empty journal was not reported as empty: $out"
grep -qF 'does not exist' <<<"$out"  && note empty-conflate "an empty journal was reported as absent: $out"

# --- the ESCAPE clears it: writing the file is all the refusal asks for ---
echo "the predecessor left none" > "$sb/scratch/build-journal.md"
out="$(run_enter "$sb" 1 validate)"; rc=$?
[[ "$rc" -eq 0 ]] || note escape "a written journal still refused: $out"
grep -qF 'demo-iteration validate' "$sb/.workflow/WORKFLOW-STATE.txt" \
    || note escape-stamp "the cleared entry wrote no stamp: $out"

# --- and that admitted entry OPENED its own journal and reported the path ---
[[ -s "$sb/scratch/validate-journal.md" ]] \
    || note opener-writes "the admitted entry opened no journal for the stage it entered: $out"
grep -qF 'scratch/validate-journal.md' <<<"$out" \
    || note opener-reports "the entry report does not name the journal it opened: $out"
grep -qE '^# stage-journal validate — demo-iteration [0-9a-f]+ [0-9-]+ ' "$sb/scratch/validate-journal.md" \
    || note opener-fields "the opening line does not carry the stamp's own fields: $(cat "$sb/scratch/validate-journal.md")"

# --- REQUIRE=1, predecessor journal OPENED BUT UNWRITTEN: the vacuity delta (3) closes ---
sb="$SANDBOX/unwritten"; seed "$sb"
out="$(run_enter "$sb" 1 build)"; rc=$?   # a second 'build' session, opening build's own journal
[[ "$rc" -eq 1 ]] || note unwritten-seed "the seeding entry was not refused as expected: $out"
mkdir -p "$sb/scratch"; echo "the predecessor left none" > "$sb/scratch/build-journal.md"
out="$(run_enter "$sb" 1 build)"; rc=$?
[[ "$rc" -eq 0 ]] || note unwritten-seed2 "the stand-in did not clear the same-stage entry: $out"
out="$( cd "$sb" && env LIFECYCLE_KIT_CONFIG_FILE="$sb/lifecycle-config.sh" \
                        GATE_SDK_TMP_DIR=scratch LIFECYCLE_KIT_SESSION_ID=deadbeef02 \
                        LIFECYCLE_KIT_STAGE_JOURNAL_REQUIRE=1 \
                        bash "$ENTER" validate 2>&1 )"; rc=$?
[[ "$rc" -eq 0 ]] || note unwritten-setup "the validate entry that opens the skeleton was refused: $out"
before="$(state_of "$sb")"
[[ -s "$sb/scratch/validate-journal.md" ]] \
    || note unwritten-nonempty "the opened skeleton is empty — the old non-emptiness predicate would not have passed on it"
out="$(run_enter "$sb" 1 close)"; rc=$?
[[ "$rc" -eq 1 ]] || note unwritten-status "an opened-but-unwritten predecessor journal did not refuse (exit $rc): $out"
grep -qF 'never written into' <<<"$out" || note unwritten-why "the refusal does not name it as unwritten: $out"
grep -qF 'is empty' <<<"$out"           && note unwritten-conflate "an opened journal was reported as empty: $out"
[[ "$(state_of "$sb")" == "$before" ]]  || note unwritten-nowrite "a refused entry wrote to the state file"

# --- the ESCAPE still works from the new state: append the stand-in and re-enter ---
echo "validate left none" >> "$sb/scratch/validate-journal.md"
out="$(run_enter "$sb" 1 close)"; rc=$?
[[ "$rc" -eq 0 ]] || note unwritten-escape "an appended stand-in did not clear the unwritten refusal: $out"

# --- the opener APPENDS and never overwrites: a second session keeps the first's content ---
sb="$SANDBOX/append"; seed "$sb"
mkdir -p "$sb/scratch"; echo "session one wrote this" > "$sb/scratch/build-journal.md"
out="$(run_enter "$sb" 1 build)"; rc=$?
[[ "$rc" -eq 0 ]] || note append-entry "the second same-stage session was refused: $out"
grep -qF 'session one wrote this' "$sb/scratch/build-journal.md" \
    || note append-overwrite "the opener overwrote the first session's journal"
[[ "$(grep -c '^# stage-journal ' "$sb/scratch/build-journal.md")" -eq 1 ]] \
    || note append-heading "the second session did not append exactly one heading naming itself"

# --- --simulate writes NO journal: the scratch dir is byte-identical across the run ---
sb="$SANDBOX/sim-nowrite"; seed "$sb"
mkdir -p "$sb/scratch"; echo "written" > "$sb/scratch/build-journal.md"
before_ls="$(find "$sb/scratch" | sort)"
before_body="$(cat "$sb/scratch/build-journal.md")"
out="$(run_enter "$sb" 1 --simulate validate)"; rc=$?
[[ "$rc" -eq 0 ]] || note sim-open-status "the simulated entry did not clear: $out"
[[ "$(find "$sb/scratch" | sort)" == "$before_ls" ]] \
    || note sim-open-nowrite "--simulate opened a journal — the read-only probe performed a state write"
[[ "$(cat "$sb/scratch/build-journal.md")" == "$before_body" ]] \
    || note sim-open-append "--simulate appended to an existing journal"

# --- the opener runs AFTER the boundary wipe: the first stage's skeleton survives it ---
sb="$SANDBOX/wipe"; seed "$sb"
mkdir -p "$sb/scratch"; echo "stale" > "$sb/scratch/leftover.md"
out="$(run_enter "$sb" 1 scope)"; rc=$?
[[ "$rc" -eq 0 ]] || note wipe-status "the iteration-boundary entry was refused: $out"
[[ -e "$sb/scratch/leftover.md" ]] && note wipe-ran "the boundary wipe did not run"
[[ -s "$sb/scratch/scope-journal.md" ]] \
    || note wipe-order "the boundary wipe deleted the skeleton — the opener ran before it"

# --- the FIRST stage of an iteration is never asserted against, journal or no ---
sb="$SANDBOX/first"; seed "$sb"
out="$(run_enter "$sb" 1 --simulate scope)"; rc=$?
grep -qF 'no resume journal' <<<"$out" && note first-stage "the boundary entry asserted a predecessor journal: $out"

# --- a SECOND session of one stage asserts the first session's journal, same derived path ---
sb="$SANDBOX/same-stage"; seed "$sb"
out="$(run_enter "$sb" 1 build)"; rc=$?
[[ "$rc" -eq 1 ]] || note same-stage-status "a second session of 'build' asserted nothing: $out"
grep -qF 'scratch/build-journal.md' <<<"$out" || note same-stage-path "the same-stage assertion names the wrong path: $out"
mkdir -p "$sb/scratch"; echo "session one" > "$sb/scratch/build-journal.md"
out="$(run_enter "$sb" 1 build)"; rc=$?
[[ "$rc" -eq 0 ]] || note same-stage-pass "the first session's journal did not satisfy the second's entry: $out"

# --- the derivation is a real expansion, not a hardcoded name ---
sb="$SANDBOX/pattern"; seed "$sb"
mkdir -p "$sb/scratch/j"; echo x > "$sb/scratch/j/build.log"
out="$( cd "$sb" && env LIFECYCLE_KIT_CONFIG_FILE="$sb/lifecycle-config.sh" \
                        GATE_SDK_TMP_DIR=scratch LIFECYCLE_KIT_SESSION_ID=deadbeef01 \
                        LIFECYCLE_KIT_STAGE_JOURNAL_REQUIRE=1 \
                        LIFECYCLE_KIT_STAGE_JOURNAL_PATTERN='scratch/j/<stage>.log' \
                        bash "$ENTER" validate 2>&1 )"; rc=$?
[[ "$rc" -eq 0 ]] || note pattern "a configured pattern did not resolve to the file it names: $out"

# --- a pattern with no <stage> placeholder is a config refusal, never a wrong-file assertion ---
out="$( cd "$sb" && env LIFECYCLE_KIT_CONFIG_FILE="$sb/lifecycle-config.sh" \
                        GATE_SDK_TMP_DIR=scratch LIFECYCLE_KIT_SESSION_ID=deadbeef01 \
                        LIFECYCLE_KIT_STAGE_JOURNAL_PATTERN='scratch/j/one.log' \
                        bash "$ENTER" --simulate validate 2>&1 )"; rc=$?
[[ "$rc" -eq 2 ]] || note no-placeholder "a pattern with no <stage> placeholder was not refused as config (exit $rc): $out"

# --- and so is a REQUIRE that is neither 0 nor 1 ---
out="$(run_enter "$sb" yes --simulate validate)"; rc=$?
[[ "$rc" -eq 2 ]] || note bad-require "a non-boolean REQUIRE was not refused as config (exit $rc): $out"

[[ "$fails" -eq 0 ]] || { echo "boundary-stage-journal.test: $fails assertion(s) failed"; exit 1; }
echo "boundary-stage-journal.test: clean (the default opens and asserts nothing; REQUIRE=1 opens the entering stage's journal and refuses an absent, empty or opened-but-unwritten predecessor writing nothing, --simulate relays the refusal and writes no journal, both escapes clear it, the opener appends and survives the boundary wipe, the first stage is exempt, a same-stage second session asserts the first's file, the pattern really expands, and two malformed configs are refused)"
exit 0
