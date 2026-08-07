#!/usr/bin/env bash
# Behavioral test of checks/check-survey-record.sh — the scenarios the one-pair
# good/bad harness cannot hold. The fixture pair drives the gate through its
# hermetic file argument, which asserts grammar only, because a fixture's rev
# names no commit in the tree the fixture was copied into. This file covers the
# other half: the bare (configured-record) mode in a real sandbox repo, where the
# rev-existence probe runs — a rev naming a real commit passes, a well-formed
# 40-hex rev naming nothing is the wrong-rev finding the shape check cannot make
# — plus the two inert shapes (no record at all, a header-only record).
#
# Run by run-gate-tests.sh (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # lifecycle-kit/
GATE="$DIR/checks/check-survey-record.sh"
SANDBOX="$(mktemp -d)"
trap 'rm -rf "$SANDBOX"' EXIT

fails=0

check_case() {  # $1=label  $2=dir  $3=want-rc  $4=want-substring
    local out rc
    out="$(cd "$2" && "$GATE" 2>&1)"; rc=$?
    if [[ "$rc" -ne "$3" ]]; then
        echo "  FAIL [$1]: want exit $3, got $rc -- $out"; fails=$((fails + 1)); return
    fi
    if [[ -n "$4" ]] && ! grep -qF -- "$4" <<<"$out"; then
        echo "  FAIL [$1]: exit $rc OK but output lacks '$4':"; printf '    %s\n' "$out"
        fails=$((fails + 1))
    fi
}

seed_repo() {  # $1=dir -> a one-commit git repo, printing its HEAD sha
    git -C "$1" init -q
    printf 'seed\n' >"$1/seed.txt"
    git -C "$1" add -A
    git -C "$1" -c user.email=t@t.invalid -c user.name=t commit -q -m base
    git -C "$1" rev-parse HEAD
}

write_record() {  # $1=dir  $2=rev
    mkdir -p "$1/.workflow"
    cat >"$1/.workflow/survey-record.md" <<EOF
# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys.

## 2026-01-02 scope — which gates meet every port criterion?
- corpus: checks/
- oracle: bash run-gates.sh
- rev: $2
- finding: four of them.
EOF
}

# --- bare mode, rev naming a real commit: the probe runs and passes ---
live="$SANDBOX/live"; mkdir -p "$live"
head_sha="$(seed_repo "$live")"
write_record "$live" "$head_sha"
check_case "rev-exists" "$live" 0 "1 rev(s) name a real commit"

# --- bare mode, well-formed 40-hex rev naming nothing: the wrong-rev finding ---
ghost="$SANDBOX/ghost"; mkdir -p "$ghost"
seed_repo "$ghost" >/dev/null
write_record "$ghost" "0123456789abcdef0123456789abcdef01234567"
check_case "rev-unknown" "$ghost" 1 "rev names no commit in this repository"

# --- an absent record is clean and counted inert: the surface is optional ---
empty="$SANDBOX/empty"; mkdir -p "$empty"
seed_repo "$empty" >/dev/null
check_case "no-record" "$empty" 0 "no survey filed this iteration"

# --- a header-only record (the shape the iteration boundary leaves behind) ---
hdr="$SANDBOX/header-only"; mkdir -p "$hdr/.workflow"
seed_repo "$hdr" >/dev/null
printf '# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys.\n' \
    >"$hdr/.workflow/survey-record.md"
check_case "header-only" "$hdr" 0 "clean (0 block(s)"

if [[ "$fails" -gt 0 ]]; then
    echo "check-survey-record.test.sh: $fails case(s) failed"
    exit 1
fi
echo "check-survey-record.test.sh: clean (rev-exists + rev-unknown probe arms, absent-record and header-only inert shapes, 4 cases)"
exit 0
