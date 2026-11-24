#!/usr/bin/env bash
# spec: evidence-kit/SPEC.md §bin/run-validate.sh — the git-notes pair: a run stores its Statement as a note on the subject with no file configured, the read knob verifies it with no operand, the store leaves the worktree untouched, a second run's note replaces the first, and a failing store exits 2 naming the published manifest.
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # evidence-kit/
FE="$(cd "$DIR/../gate-sdk/bin" && pwd)/run-gates.sh"

fails=0
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT
fail() { echo "  FAIL: $*"; fails=$((fails + 1)); }

t="$tmp/tree"
mkdir -p "$t/.workflow" "$t/scripts"
( cd "$t" && git init -q . && git config user.name t && git config user.email t@t ) >/dev/null 2>&1
printf '# baseline\nok ok pass\n' >"$t/.workflow/validate-baseline.txt"
printf '# contract: evidence-manifest v2\n' >"$t/.workflow/validate-evidence.txt"
printf '.tmp/\n' >"$t/.gitignore"
printf '%s\n' "EVIDENCE_KIT_SUITES[] = ok" "EVIDENCE_KIT_RUN_ID = store" "EVIDENCE_KIT_RUN_ok = true" >"$t/scripts/base.knobs"
{
    cat "$t/scripts/base.knobs"
    printf 'EVIDENCE_KIT_STATEMENT_WRITE_CMD[] = %s\n' git notes --ref=gate-run add -f -F -
    printf 'EVIDENCE_KIT_STATEMENT_CMD[] = %s\n' git notes --ref=gate-run show HEAD
} >"$t/scripts/notes.knobs"
{
    cat "$t/scripts/base.knobs"
    printf 'EVIDENCE_KIT_STATEMENT_WRITE_CMD[] = %s\n' git no-such-subcommand
} >"$t/scripts/broken.knobs"
( cd "$t" && git add -A && git commit -q -m base ) >/dev/null 2>&1
_fe() { local knobs="$1"; shift; ( cd "$t" && EVIDENCE_KIT_KNOB_FILE="scripts/$knobs" bash "$FE" "$@" 2>&1 ); }

# A — the store: a note on the subject, no file written, and nothing in the worktree but the fold.
out="$(_fe notes.knobs --run-validate)"; rc=$?
[[ "$rc" -eq 0 && "$out" == *"Statement stored by EVIDENCE_KIT_STATEMENT_WRITE_CMD"* ]] || fail "the run must store its Statement (rc=$rc): $out"
git -C "$t" notes --ref=gate-run show HEAD 2>/dev/null | grep -q '"predicateType":"https://checkwright.dev/gate-run/v1"' \
    || fail "no gate-run Statement stored as the subject's note"
status="$(git -C "$t" status --porcelain --untracked-files=all)"
[[ "$status" == " M .workflow/validate-evidence.txt" ]] || fail "the store must leave the worktree untouched but for the fold: $status"

# B — the read half: the verify arm judges the note with no operand.
out="$(_fe notes.knobs --verify-gate-run)"; rc=$?
[[ "$rc" -eq 0 && "$out" == *"clean"*"ahead=0"* ]] || fail "the verify arm must read the note clean through EVIDENCE_KIT_STATEMENT_CMD (rc=$rc): $out"

# C — a second run at the same subject replaces the note: still one note, and one more notes commit.
out="$(_fe notes.knobs --run-validate)"; rc=$?
[[ "$rc" -eq 0 ]] || fail "a second run must replace the note (rc=$rc): $out"
notes="$(git -C "$t" notes --ref=gate-run list | wc -l | tr -d ' ')"
commits="$(git -C "$t" rev-list --count refs/notes/gate-run)"
[[ "$notes" == 1 && "$commits" == 2 ]] || fail "a second run must replace the first note (notes=$notes, notes commits=$commits)"

# D — a failing store: exit 2, naming the manifest as published.
out="$(_fe broken.knobs --run-validate)"; rc=$?
[[ "$rc" -eq 2 && "$out" == *"EVIDENCE_KIT_STATEMENT_WRITE_CMD exited"*"the manifest"*"published, the store is not"* ]] \
    || fail "a failing store must exit 2 naming the published manifest (rc=$rc): $out"

if [[ "$fails" -gt 0 ]]; then
    echo "statement-store.test: $fails assertion(s) failed"
    exit 1
fi
echo "statement-store.test: ok (the run stores a note on its subject with no file written and the worktree untouched but for the fold, the verify arm reads it clean with no operand, a second run replaces the note, a failing store exits 2 naming the published manifest)"
exit 0
