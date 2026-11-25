#!/usr/bin/env bash
# spec: evidence-kit/SPEC.md §bin/run-validate.sh — the pre-hook runs once per suite, before it, with the suite as operand, and a failing hook aborts at exit 3 with no evidence appended.
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # evidence-kit/
FE="$(cd "$DIR/../gate-sdk/bin" && pwd)/run-gates.sh"

fails=0
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

# spec: evidence-kit/SPEC.md §bin/run-validate.sh — each scratch tree is its own git toplevel, crossing the absolute binary the preamble exports.
mk_tree() {
    rm -rf "$1"; mkdir -p "$1/.workflow" "$1/scripts" "$1/.tmp"
    ( cd "$1" && git init -q . ) >/dev/null 2>&1
    printf '# baseline\nalpha alpha pass\nbeta beta pass\n' >"$1/.workflow/validate-baseline.txt"
    printf '# contract: evidence-manifest v2\n' >"$1/.workflow/validate-evidence.txt"
    printf '#!/usr/bin/env bash\nprintf "%%s\\n" "$1" >> .tmp/hook.log\n' >"$1/scripts/hook.sh"
    # The suite exits non-zero unless the hook already ran for it, so ordering is observable as a verdict.
    printf '#!/usr/bin/env bash\nprintf "%%s\\n" "$1" >> .tmp/suite.log\ngrep -qx "$1" .tmp/hook.log\n' >"$1/scripts/suite.sh"
    printf '%s\n' \
        "EVIDENCE_KIT_SUITES[] = alpha" \
        "EVIDENCE_KIT_SUITES[] = beta" \
        "EVIDENCE_KIT_RUN_ID = hook-test" \
        "EVIDENCE_KIT_RUN_alpha = bash scripts/suite.sh alpha" \
        "EVIDENCE_KIT_RUN_beta = bash scripts/suite.sh beta" \
        "EVIDENCE_KIT_PRE_HOOK = $2" >"$1/scripts/evidence-config.knobs"
}
_rv() { ( cd "$1" && EVIDENCE_KIT_KNOB_FILE=scripts/evidence-config.knobs bash "$FE" --run-validate 2>&1 ); }

# A — a word-split hook runs per suite in roster order, each call handed its suite, each before that suite.
mk_tree "$tmp/a" "bash scripts/hook.sh"
out="$(_rv "$tmp/a")"; rc=$?
if [[ "$rc" -ne 0 ]]; then
    echo "  FAIL: a passing hook with suites that saw it must exit 0 (rc=$rc): $out"; fails=$((fails + 1))
fi
if [[ "$(cat "$tmp/a/.tmp/hook.log" 2>/dev/null)" != $'alpha\nbeta' ]]; then
    echo "  FAIL: the hook did not run once per suite in roster order with the suite as operand: $(cat "$tmp/a/.tmp/hook.log" 2>/dev/null)"; fails=$((fails + 1))
fi
if ! grep -q '^hook-test alpha .* verdict=clean ' "$tmp/a/.workflow/validate-evidence.txt" \
    || ! grep -q '^hook-test beta .* verdict=clean ' "$tmp/a/.workflow/validate-evidence.txt"; then
    echo "  FAIL: a suite ran before its hook, or no evidence was folded: $(cat "$tmp/a/.workflow/validate-evidence.txt")"; fails=$((fails + 1))
fi

# B — a failing hook aborts at the tool-error code before its suite runs, and no line reaches the manifest.
mk_tree "$tmp/b" "false"
out="$(_rv "$tmp/b")"; rc=$?
if [[ "$rc" -ne 3 ]] || [[ "$out" != *"pre-hook failed for suite 'alpha'"* ]]; then
    echo "  FAIL: a failing hook must abort at exit 3 naming its suite (rc=$rc): $out"; fails=$((fails + 1))
fi
if [[ -e "$tmp/b/.tmp/suite.log" ]]; then
    echo "  FAIL: a suite ran after its hook failed"; fails=$((fails + 1))
fi
if grep -q '^hook-test ' "$tmp/b/.workflow/validate-evidence.txt"; then
    echo "  FAIL: a run its hook refused still recorded evidence"; fails=$((fails + 1))
fi

# C — with a Statement configured, a hook that moves HEAD refuses at exit 2 before
# its suite runs, naming the suite, and neither the manifest nor the Statement is written.
mk_tree "$tmp/c" "bash scripts/commit.sh"
printf '#!/usr/bin/env bash\ngit -c user.name=t -c user.email=t@t commit -q --allow-empty -m "hook $1"\n' >"$tmp/c/scripts/commit.sh"
printf 'EVIDENCE_KIT_STATEMENT_FILE = .tmp/statement.json\n' >>"$tmp/c/scripts/evidence-config.knobs"
( cd "$tmp/c" && git -c user.name=t -c user.email=t@t commit -q --allow-empty -m base ) >/dev/null 2>&1
before="$(cat "$tmp/c/.workflow/validate-evidence.txt")"
out="$(_rv "$tmp/c")"; rc=$?
if [[ "$rc" -ne 2 ]] || [[ "$out" != *"'alpha'"* ]]; then
    echo "  FAIL: a hook that commits must refuse at exit 2 naming its suite (rc=$rc): $out"; fails=$((fails + 1))
fi
if [[ -e "$tmp/c/.tmp/suite.log" ]] || [[ -e "$tmp/c/.tmp/statement.json" ]] \
    || [[ "$(cat "$tmp/c/.workflow/validate-evidence.txt")" != "$before" ]]; then
    echo "  FAIL: a run refused for a moved HEAD still ran a suite or wrote a file"; fails=$((fails + 1))
fi

if [[ "$fails" -gt 0 ]]; then
    echo "pre-hook.test: $fails assertion(s) failed"
    exit 1
fi
echo "pre-hook.test: ok (per-suite hook in roster order with its operand ahead of the suite, failing hook exit 3 with no suite run and no evidence, a HEAD-moving hook under a Statement exit 2 with nothing written)"
exit 0
