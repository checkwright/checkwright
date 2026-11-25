#!/usr/bin/env bash
# spec: evidence-kit/SPEC.md §verify-gate-run — each EVIDENCE_KIT_VERIFY_REQUIRE word reds its own case and holds on a run of HEAD with every suite clean; EVIDENCE_KIT_TREE_UNTRACKED=count reads an untracked path as modified unless it is ignored or the run's own write, a link judged where it sits; --json prints the result as one document on exit 0 and 1 and the error document on exit 2 or 3, and the import arm refuses the option.
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

# spec: evidence-kit/SPEC.md §bin/run-validate.sh — each scratch tree is its own git toplevel, crossing the absolute binary the preamble exports; the tmp dir is left unignored so only the run-write exclusion keeps it out of a count.
mk_tree() {
    rm -rf "$1"; mkdir -p "$1/.workflow" "$1/scripts" "$1/out"
    ( cd "$1" && git init -q . && git config user.name t && git config user.email t@t ) >/dev/null 2>&1
    printf '# baseline\nok ok pass\nbad bad pass\n' >"$1/.workflow/validate-baseline.txt"
    printf '# contract: evidence-manifest v2\n' >"$1/.workflow/validate-evidence.txt"
    printf 'ignored.txt\n' >"$1/.gitignore"
    printf '%s\n' "EVIDENCE_KIT_SUITES[] = $2" \
        "EVIDENCE_KIT_RUN_ID = strict" \
        "EVIDENCE_KIT_RUN_ok = true" \
        "EVIDENCE_KIT_RUN_bad = false" \
        "EVIDENCE_KIT_STATEMENT_FILE = out/statement.json" >"$1/scripts/evidence-config.knobs"
    ( cd "$1" && git add -A && git commit -q -m base ) >/dev/null 2>&1
}
_fe() { local d="$1"; shift; ( cd "$d" && EVIDENCE_KIT_KNOB_FILE=scripts/evidence-config.knobs bash "$FE" "$@" 2>&1 ); }
_commit() { ( cd "$1" && git add -A && git commit -q --allow-empty -m "${2:-next}" ) >/dev/null 2>&1; }

# A — head: clean on a Statement of HEAD, red one commit later, and the default selection reds neither.
mk_tree "$tmp/a" ok
_fe "$tmp/a" --run-validate >/dev/null
out="$(EVIDENCE_KIT_VERIFY_REQUIRE="head" _fe "$tmp/a" --verify-gate-run out/statement.json)"; rc=$?
[[ "$rc" -eq 0 && "$out" == *"clean"*"ahead=0"* ]] || fail "head must hold on a Statement of HEAD (rc=$rc): $out"
cp "$tmp/a/out/statement.json" "$tmp/a-statement.json"
_commit "$tmp/a"
out="$(EVIDENCE_KIT_VERIFY_REQUIRE="head" _fe "$tmp/a" --verify-gate-run "$tmp/a-statement.json")"; rc=$?
[[ "$rc" -eq 1 && "$out" == *"is 1 commit(s) behind HEAD (EVIDENCE_KIT_VERIFY_REQUIRE: head)"* && "$out" == *"ahead=1"* ]] \
    || fail "head must red a Statement one commit behind HEAD, naming the requirement (rc=$rc): $out"
out="$(_fe "$tmp/a" --verify-gate-run "$tmp/a-statement.json")"; rc=$?
[[ "$rc" -eq 0 ]] || fail "the empty default must not red a Statement behind HEAD (rc=$rc): $out"

# A' — --json: the document's top-level verdict and ahead on a clean and on a contradicted Statement,
# the error document on an unreadable Statement, and the import arm keeps refusing the option.
out="$(_fe "$tmp/a" --verify-gate-run --json "$tmp/a-statement.json")"; rc=$?
[[ "$rc" -eq 0 && "$out" == '{"ahead":1,'* && "$out" == *'],"verdict":"clean"}' ]] \
    || fail "--json must print a clean document at ahead 1 (rc=$rc): $out"
out="$(EVIDENCE_KIT_VERIFY_REQUIRE="head" _fe "$tmp/a" --verify-gate-run "$tmp/a-statement.json" --json)"; rc=$?
[[ "$rc" -eq 1 && "$out" == '{"ahead":1,'* && "$out" == *'],"verdict":"contradiction"}' ]] \
    || fail "--json must print a contradiction document, the option after the operand (rc=$rc): $out"
out="$( cd "$tmp/a" && EVIDENCE_KIT_KNOB_FILE=scripts/evidence-config.knobs bash "$FE" --verify-gate-run --json "$tmp/absent.json" 2>/dev/null )"; rc=$?
[[ "$rc" -eq 3 && "$out" == '{"contract":"1.1.0","error":"input"}' ]] \
    || fail "--json must print the error document on an unreadable Statement at exit 3 (rc=$rc): $out"
out="$(EVIDENCE_KIT_IMPORTED_RUNS=accept _fe "$tmp/a" --import-gate-run --json "$tmp/a-statement.json")"; rc=$?
[[ "$rc" -eq 2 && "$out" == *"at most one operand (got: --json "* ]] || fail "--import-gate-run must keep refusing --json (rc=$rc): $out"

# B — verdict: clean on a passing suite, red on a failing one, and the default selection reds neither.
out="$(EVIDENCE_KIT_VERIFY_REQUIRE=verdict _fe "$tmp/a" --verify-gate-run "$tmp/a-statement.json")"; rc=$?
[[ "$rc" -eq 0 ]] || fail "verdict must hold on a passing suite (rc=$rc): $out"
mk_tree "$tmp/b" bad
_fe "$tmp/b" --run-validate >/dev/null; rc=$?
[[ "$rc" -eq 1 && -s "$tmp/b/out/statement.json" ]] || fail "a failing suite must exit 1 and still write its Statement (rc=$rc)"
out="$(EVIDENCE_KIT_VERIFY_REQUIRE=verdict _fe "$tmp/b" --verify-gate-run out/statement.json)"; rc=$?
[[ "$rc" -eq 1 && "$out" == *"suite 'bad' verdict: new-failures (EVIDENCE_KIT_VERIFY_REQUIRE: verdict)"* ]] \
    || fail "verdict must red a failing suite, naming the requirement (rc=$rc): $out"
out="$(_fe "$tmp/b" --verify-gate-run out/statement.json)"; rc=$?
[[ "$rc" -eq 0 ]] || fail "the empty default must not red a failing suite's Statement (rc=$rc): $out"
out="$(EVIDENCE_KIT_VERIFY_REQUIRE=ahead _fe "$tmp/b" --verify-gate-run out/statement.json)"; rc=$?
[[ "$rc" -eq 2 && "$out" == *"EVIDENCE_KIT_VERIFY_REQUIRE word 'ahead'"* ]] || fail "an unknown requirement word must refuse at exit 2 (rc=$rc): $out"

# C — count: clean with only an ignored file and the run's own writes untracked, modified with another
# untracked file; the default ignore reads that same tree clean.
mk_tree "$tmp/c" ok
: >"$tmp/c/ignored.txt"
EVIDENCE_KIT_TREE_UNTRACKED=count _fe "$tmp/c" --run-validate >/dev/null
grep -q '^strict ok .* tree=clean ' "$tmp/c/.workflow/validate-evidence.txt" \
    || fail "count must read clean with only an ignored file and the tmp dir untracked: $(cat "$tmp/c/.workflow/validate-evidence.txt")"
_commit "$tmp/c"
git -C "$tmp/c" rm -q --cached out/statement.json >/dev/null 2>&1; _commit "$tmp/c"
[[ -f "$tmp/c/out/statement.json" ]] || fail "the Statement under test is missing"
EVIDENCE_KIT_TREE_UNTRACKED=count _fe "$tmp/c" --run-validate >/dev/null
grep -q '^strict ok .* tree=clean ' "$tmp/c/.workflow/validate-evidence.txt" \
    || fail "count must not read the untracked Statement it wrote: $(cat "$tmp/c/.workflow/validate-evidence.txt")"
_commit "$tmp/c"
git -C "$tmp/c" rm -q --cached out/statement.json >/dev/null 2>&1; _commit "$tmp/c"
: >"$tmp/c/stray.txt"
EVIDENCE_KIT_TREE_UNTRACKED=count _fe "$tmp/c" --run-validate >/dev/null
grep -q '^strict ok .* tree=modified ' "$tmp/c/.workflow/validate-evidence.txt" \
    || fail "count must read modified with another untracked file: $(cat "$tmp/c/.workflow/validate-evidence.txt")"
out="$(_fe "$tmp/c" --verify-gate-run out/statement.json)"; rc=$?
[[ "$rc" -eq 1 && "$out" == *"suite 'ok' tree: modified"* ]] || fail "verify must red the Statement count produced (rc=$rc): $out"
_commit "$tmp/c"
git -C "$tmp/c" rm -q --cached out/statement.json >/dev/null 2>&1; _commit "$tmp/c"
: >"$tmp/c/stray2.txt"
_fe "$tmp/c" --run-validate >/dev/null
grep -q '^strict ok .* tree=clean ' "$tmp/c/.workflow/validate-evidence.txt" \
    || fail "the default ignore must read an untracked file clean: $(cat "$tmp/c/.workflow/validate-evidence.txt")"
out="$(EVIDENCE_KIT_TREE_UNTRACKED=all _fe "$tmp/c" --run-validate)"; rc=$?
[[ "$rc" -eq 2 && "$out" == *"EVIDENCE_KIT_TREE_UNTRACKED is 'all'"* ]] || fail "an unknown untracked selection must refuse at exit 2 (rc=$rc): $out"

# D — count judges a link where it sits: an untracked symlink outside the tmp dir whose target lies
# inside it is no write of the run's. Git Bash copies on a plain `ln -s`, so a copy is a skip.
mk_tree "$tmp/d" ok
mkdir -p "$tmp/d/.tmp/inside"
MSYS=winsymlinks:nativestrict ln -s .tmp/inside "$tmp/d/escape" 2>/dev/null
if [[ -L "$tmp/d/escape" ]]; then
    EVIDENCE_KIT_TREE_UNTRACKED=count _fe "$tmp/d" --run-validate >/dev/null
    grep -q '^strict ok .* tree=modified ' "$tmp/d/.workflow/validate-evidence.txt" \
        || fail "count must read an untracked link into the tmp dir as modified: $(cat "$tmp/d/.workflow/validate-evidence.txt")"
else
    echo "verify-gate-run-strict.test: skip [D]: this host made no symlink"
fi

if [[ "$fails" -gt 0 ]]; then
    echo "verify-gate-run-strict.test: $fails assertion(s) failed"
    exit 1
fi
echo "verify-gate-run-strict.test: ok (head clean at the subject and red one commit later, verdict clean on a passing suite and red on a failing one, count clean with only ignored and run-written untracked files and modified with another or with a link into the tmp dir, unknown words refused at exit 2, --json a document on exit 0 and 1 and the error document on an unreadable Statement)"
exit 0
