#!/usr/bin/env bash
# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the predecessor-journal assertion end-to-end through a sandboxed enter-stage: the derived path is a function of the stage, the default REQUIRE=0 asserts nothing, REQUIRE=1 passes on a written journal and refuses on an absent or empty one writing nothing, --simulate relays the refusal, the escape clears it, the first stage of an iteration is never asserted against, a second session of one stage asserts the first session's journal at the same path, and a pattern with no <stage> placeholder is refused as config
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

# --- the DEFAULT asserts nothing: no journal anywhere, and the entry proceeds ---
sb="$SANDBOX/default-off"; seed "$sb"
out="$( cd "$sb" && env LIFECYCLE_KIT_CONFIG_FILE="$sb/lifecycle-config.sh" \
                        GATE_SDK_TMP_DIR=scratch LIFECYCLE_KIT_SESSION_ID=deadbeef01 \
                        bash "$ENTER" validate 2>&1 )"; rc=$?
[[ "$rc" -eq 0 ]] || note default-off "the default REQUIRE refused an entry with no journal: $out"

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
echo "boundary-stage-journal.test: clean (the default asserts nothing; REQUIRE=1 refuses an absent and an empty predecessor journal writing nothing, --simulate relays it, the escape clears it, the first stage is exempt, a same-stage second session asserts the first's file, the pattern really expands, and two malformed configs are refused)"
exit 0
