#!/usr/bin/env bash
# Behavioral test of check-close-surfaces over real git repos — the
# two assertions the good/bad pair cannot reach. A fixture case dir cannot ship
# a gitignored member: `git check-ignore` never reports a tracked path, and a
# fixture file must be tracked to survive a clone. So the capture-tier half —
# assertion A (an undeclared capture surface) and assertion C (a declared one
# with no reclaim command, inside or outside the workflow dir) — is exercised in sandbox repos here, the
# check-merge-attrs / check-exec-bit precedent. The pair covers assertion B.
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # lifecycle-kit/
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0

check_case() {  # $1=label  $2=dir  $3=want-rc  $4=want-substring
    local out rc
    out="$(gate_run check-close-surfaces "$DIR/checks" "$2" 2>&1)"; rc=$?
    if [[ "$rc" -ne "$3" ]]; then
        echo "  FAIL [$1]: want exit $3, got $rc -- $out"; fails=$((fails + 1)); return
    fi
    if [[ -n "$4" ]] && ! grep -qF -- "$4" <<<"$out"; then
        echo "  FAIL [$1]: exit $rc OK but output lacks '$4':"; printf '    %s\n' "$out"
        fails=$((fails + 1))
    fi
}

# lib/test-hermetic.sh pins LIFECYCLE_KIT_KNOB_FILE to an empty file, so these
# cases run on the kit's default declaration glob; the good/bad pair is what
# exercises a consumer-set LIFECYCLE_KIT_CLOSE_SURFACE_GLOBS.
seed_repo() {  # $1=dir  $2=declaration body (may be empty)
    mkdir -p "$1/.workflow" "$1/owners"
    git -C "$1" init -q
    printf '.workflow/*.log\n' >"$1/.gitignore"
    printf 'a capture line\n' >"$1/.workflow/capture.log"
    printf '# The owner\n\n%s\n' "$2" >"$1/owners/SPEC.md"
    git -C "$1" add -A
    git -C "$1" -c user.email=t@t.invalid -c user.name=t commit -q -m base
}

# --- assertion A: a gitignored workflow member nobody declared ---
undecl="$SANDBOX/undeclared"
seed_repo "$undecl" ""
check_case "undeclared-capture" "$undecl" 1 "with no 'close-surface:' declaration"

# --- assertion C: the capture surface is declared, but names no reclaim ---
noreclaim="$SANDBOX/noreclaim"
seed_repo "$noreclaim" "close-surface: .workflow/capture.log advisory"
check_case "capture-without-reclaim" "$noreclaim" 1 "names no reclaim= command"

# --- both satisfied: declared, moded, reclaimed ---
ok="$SANDBOX/ok"
seed_repo "$ok" "close-surface: .workflow/capture.log advisory reclaim=: > .workflow/capture.log"
check_case "capture-declared-and-reclaimed" "$ok" 0 "CLOSE-SURFACES: clean"

# --- assertion C reads <tracking>: a gitignored declared path outside the workflow dir, no reclaim ---
outside="$SANDBOX/outside"
seed_repo "$outside" $'close-surface: .workflow/capture.log advisory reclaim=: > .workflow/capture.log\nclose-surface: notes/scratch.log advisory'
printf 'notes/\n' >>"$outside/.gitignore"
mkdir -p "$outside/notes"; printf 'x\n' >"$outside/notes/scratch.log"
check_case "ignored-outside-workflow-without-reclaim" "$outside" 1 "'close-surface: notes/scratch.log' is capture-tier"

# --- a declared log's drain companions fold into its row; a drain beside no declared log does not ---
drained="$SANDBOX/drained"
seed_repo "$drained" "close-surface: .workflow/capture.log advisory reclaim=: > .workflow/capture.log"
printf '.workflow/*.drain\n.workflow/*.drain.part\n' >>"$drained/.gitignore"
printf 'a drained line\n' >"$drained/.workflow/capture.log.drain"
printf 'a crashed line\n' >"$drained/.workflow/capture.log.drain.part"
check_case "declared-log-with-drain" "$drained" 0 "CLOSE-SURFACES: clean"
printf 'x\n' >"$drained/.workflow/stray.log.drain"
check_case "drain-of-undeclared-log" "$drained" 1 ".workflow/stray.log.drain"

if [[ "$fails" -gt 0 ]]; then
    echo "check-close-surfaces.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-close-surfaces.test.sh: clean (undeclared capture, declared-without-reclaim, the satisfied case, a gitignored declared path outside the workflow dir, a declared log's drain folded into its row, and a drain beside no declared log reported, 6 cases)"
exit 0
