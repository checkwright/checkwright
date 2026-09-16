#!/usr/bin/env bash
# Behavioral test of check-audit-roster — the scenarios the one-pair good/bad harness cannot hold.
# The fixture pair drives the gate through its hermetic file argument, which asserts the grammar,
# the cap and class uniqueness, because a fixture's iteration names nothing in the host's state
# file. This file covers the bare (configured-roster) mode in a sandbox: assertion D against a
# real queue header and state file, a past iteration's last left unchecked, and the three inert
# shapes (an empty knob, an absent roster, a header-only roster).
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # lifecycle-kit/
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0

check_case() {  # $1=label  $2=dir  $3=roster knob value  $4=want-rc  $5=want-substring
    local out rc
    out="$(cd "$2" && LIFECYCLE_KIT_AUDIT_ROSTER_FILE="$3" gate_run check-audit-roster "$DIR/checks" 2>&1)"; rc=$?
    if [[ "$rc" -ne "$4" ]]; then
        echo "  FAIL [$1]: want exit $4, got $rc -- $out"; fails=$((fails + 1)); return
    fi
    if [[ -n "$5" ]] && ! grep -qF -- "$5" <<<"$out"; then
        echo "  FAIL [$1]: exit $rc OK but output lacks '$5':"; printf '    %s\n' "$out"
        fails=$((fails + 1))
    fi
}

seed_tree() {  # $1=dir  $2=stamped stage -> a queue naming iteration 'live-cut' and one stamp
    mkdir -p "$1/.workflow"
    printf '# queue\n\n## Iteration: live-cut\n' >"$1/TASK-QUEUE.md"
    printf 'header\n---\nlive-cut %s sid 2026-01-02 abcdef1\n' "$2" >"$1/.workflow/WORKFLOW-STATE.txt"
}

write_roster() {  # $1=dir  $2=last value
    cat >"$1/roster.txt" <<EOF
# contract: lifecycle-kit/SPEC.md §The audit roster

class: stale-identifier
scope: a path cited as live after it was deleted
due: an iteration that deletes a tracked path
last: $2
corpus: git grep -n retired
hits: 1
declined: none
EOF
}

# --- D holds: the current iteration's last names a stage the state file stamped ---
held="$SANDBOX/held"; mkdir -p "$held"
seed_tree "$held" close
write_roster "$held" "live-cut close"
check_case "stamp-holds" "$held" roster.txt 0 "1 current-iteration last stamp(s) checked"

# --- D reds: the named stage never ran this iteration (the pre-stamp claiming close) ---
claimed="$SANDBOX/claimed"; mkdir -p "$claimed"
seed_tree "$claimed" build
write_roster "$claimed" "live-cut close"
check_case "stamp-missing" "$claimed" roster.txt 1 "the named stage never ran this iteration"

# --- a non-close stage that did stamp is admitted: close reads it as a pre-stamp ---
pre="$SANDBOX/pre-stamp"; mkdir -p "$pre"
seed_tree "$pre" build
write_roster "$pre" "live-cut build"
check_case "pre-stamp-admitted" "$pre" roster.txt 0 "1 current-iteration last stamp(s) checked"

# --- a past iteration's last is not checked: the boundary truncated its stamps ---
past="$SANDBOX/past"; mkdir -p "$past"
seed_tree "$past" build
write_roster "$past" "earlier-cut close"
check_case "past-iteration" "$past" roster.txt 0 "0 current-iteration last stamp(s) checked"

# --- the inert shapes ---
inert="$SANDBOX/inert"; mkdir -p "$inert"
seed_tree "$inert" build
check_case "empty-knob" "$inert" "" 0 "no roster configured"
check_case "absent-roster" "$inert" roster.txt 0 "no roster at roster.txt"
printf '# contract: lifecycle-kit/SPEC.md §The audit roster\n' >"$inert/header.txt"
check_case "header-only" "$inert" header.txt 0 "no class block"

if [[ "$fails" -gt 0 ]]; then
    echo "check-audit-roster.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-audit-roster.test.sh: clean (assertion D held, missing, admitting a stamped pre-stamp and skipping a past iteration; empty-knob, absent and header-only inert shapes, 7 cases)"
exit 0
