#!/usr/bin/env bash
# spec: lifecycle-kit/SPEC.md §bin/enter-stage.sh — the boundary scratch wipe end-to-end through a sandboxed enter-stage: the iteration-boundary entry deletes the scratch dir's members, keeps the .gitkeep kit invariant and every LIFECYCLE_KIT_BOUNDARY_PRESERVE basename at any depth, names the wiped set in its report, and a non-boundary entry touches no scratch at all
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # lifecycle-kit/
ENTER="$DIR/bin/enter-stage.sh"
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0
note() { echo "  FAIL [$1]: $2"; fails=$((fails + 1)); }

seed() {  # $1=sandbox subdir; writes a boundary-ready consumer with a populated scratch dir
    local sb="$1"
    mkdir -p "$sb/.workflow" "$sb/scratch/doomed-sub" "$sb/scratch/mixed-sub"
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
demo-iteration validate cccccccc 2026-06-03 none
demo-iteration close dddddddd 2026-06-04 none
EOF
    cat >"$sb/lifecycle-config.sh" <<'EOF'
# shellcheck shell=bash
LIFECYCLE_KIT_BOUNDARY_PRESERVE=(keep-me)
EOF
    : >"$sb/scratch/.gitkeep"
    printf 'live\n'   >"$sb/scratch/keep-me"
    printf 'stale\n'  >"$sb/scratch/doomed.log"
    printf 'stale\n'  >"$sb/scratch/doomed-sub/nested.txt"
    printf 'live\n'   >"$sb/scratch/mixed-sub/keep-me"
}

run_enter() {  # $1=sandbox subdir  $2=stage
    ( cd "$1" && env LIFECYCLE_KIT_CONFIG_FILE="$1/lifecycle-config.sh" \
                     GATE_SDK_TMP_DIR=scratch \
                     LIFECYCLE_KIT_SESSION_ID=deadbeef01 \
                     bash "$ENTER" "$2" 2>&1 )
}

# --- the boundary entry wipes, keeping the invariant and the keep-list ---
bnd="$SANDBOX/boundary"
seed "$bnd"
out="$(run_enter "$bnd" scope)"; rc=$?
[[ "$rc" -eq 0 ]] || note boundary-entry "want exit 0, got $rc -- $out"

[[ -f "$bnd/scratch/.gitkeep" ]]           || note keep-invariant ".gitkeep was deleted (kit invariant)"
[[ -f "$bnd/scratch/keep-me" ]]            || note keep-listed "the PRESERVE member was deleted"
[[ -f "$bnd/scratch/mixed-sub/keep-me" ]]  || note keep-nested "a PRESERVE basename below the top level was deleted"
[[ -e "$bnd/scratch/doomed.log" ]]         && note wipe-file "an unlisted scratch file survived"
[[ -e "$bnd/scratch/doomed-sub" ]]         && note wipe-dir "an all-unlisted subdirectory survived"
[[ -d "$bnd/scratch" ]]                    || note wipe-dir-self "the scratch dir itself was removed (members, not the dir)"

grep -qF 'boundary-wiped from scratch' <<<"$out" || note report "the boundary report names no wiped set: $out"
grep -qF 'doomed.log' <<<"$out"                  || note report-member "the report does not name the wiped file: $out"
grep -qF 'scratch/keep-me' <<<"$out"             && note report-kept "the report names a kept member as wiped: $out"

# --- a non-boundary entry touches no scratch ---
non="$SANDBOX/nonboundary"
seed "$non"
before="$(find "$non/scratch" | sort)"
out="$(run_enter "$non" build)"; rc=$?
[[ "$rc" -eq 0 ]] || note nonboundary-entry "want exit 0, got $rc -- $out"
after="$(find "$non/scratch" | sort)"
[[ "$before" == "$after" ]] || note nonboundary-scratch "a non-boundary entry changed the scratch dir"
grep -qF 'boundary-wiped' <<<"$out" && note nonboundary-report "a non-boundary entry reported a wipe: $out"

# --- an unset keep-list still spares the invariant ---
def="$SANDBOX/default-knob"
seed "$def"
: >"$def/lifecycle-config.sh"
out="$(run_enter "$def" scope)"; rc=$?
[[ "$rc" -eq 0 ]] || note default-entry "want exit 0, got $rc -- $out"
[[ -f "$def/scratch/.gitkeep" ]] || note default-invariant ".gitkeep was deleted under an empty keep-list"
[[ -e "$def/scratch/keep-me" ]]  && note default-wipe "an unset keep-list spared a member anyway"

[[ "$fails" -eq 0 ]] || { echo "boundary-scratch-wipe.test: $fails assertion(s) failed"; exit 1; }
echo "boundary-scratch-wipe.test: clean (boundary wipe keeps .gitkeep and every PRESERVE basename at any depth, reports the wiped set, and leaves non-boundary entries untouched)"
exit 0
