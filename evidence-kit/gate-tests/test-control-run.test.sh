#!/usr/bin/env bash
# spec: evidence-kit/SPEC.md §Test controls — a control re-runs its suite under its assignments and records the witness after the suite's line; a control that witnesses no red exits 1 and still publishes the Statement, a malformed declaration exits 2 before the claim, and a re-run supersedes the suite's control lines.
#
# Run by the --run-gate-tests arm (any <tests-dir>/*.test.sh; must exit 0).
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"

DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"   # evidence-kit/
FE="$(cd "$DIR/../gate-sdk/bin" && pwd)/run-gates.sh"

fails=0
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

# mk_tree <dir> <suite-script-body> <control-value> — one suite, alpha, under the exit-code parser,
# so its one scenario is the suite's name; the tree carries one commit for a Statement's subject
mk_tree() {
    rm -rf "$1"; mkdir -p "$1/.workflow" "$1/scripts" "$1/.tmp"
    printf '# baseline\nalpha alpha pass\n' >"$1/.workflow/validate-baseline.txt"
    printf '# contract: evidence-manifest v2\n' >"$1/.workflow/validate-evidence.txt"
    printf '#!/usr/bin/env bash\nprintf "run\\n" >> .tmp/suite.log\n%s\n' "$2" >"$1/scripts/suite.sh"
    printf '%s\n' \
        "EVIDENCE_KIT_SUITES[] = alpha" \
        "EVIDENCE_KIT_RUN_ID = ctl-test" \
        "EVIDENCE_KIT_RUN_alpha = bash scripts/suite.sh" \
        "EVIDENCE_KIT_CONTROLS[] = guard" \
        "EVIDENCE_KIT_CONTROL_guard = $3" >"$1/scripts/evidence-config.knobs"
    ( cd "$1" && git init -q . && git -c user.name=t -c user.email=t@t commit -q --allow-empty -m base ) >/dev/null 2>&1
}
_rv() { ( cd "$1" && EVIDENCE_KIT_KNOB_FILE=scripts/evidence-config.knobs bash "$FE" --run-validate 2>&1 ); }
manifest_lines() { grep -v '^#' "$1/.workflow/validate-evidence.txt"; }

# A — a suite that fails when BREAK is set: the control witnesses red, its line follows the suite's,
#     its log and parse sit beside the suite's, and the run exits 0. A re-run leaves one control line.
#     The line's assign= is the SHA-256 of the declaration's one assignment, 'BREAK=1' and a newline.
BREAK_DIGEST=ed00581708d857f9a1d8afd4572cdf1ef80eb19cdd5721a5405edfb5f617fa56
mk_tree "$tmp/a" '[[ -z "${BREAK:-}" ]]' "alpha alpha BREAK=1"
out="$(_rv "$tmp/a")"; rc=$?
lines="$(manifest_lines "$tmp/a")"
if [[ "$rc" -ne 0 ]]; then
    echo "  FAIL: a control witnessed red must exit 0 (rc=$rc): $out"; fails=$((fails + 1))
fi
if ! grep -q '^ctl-test alpha sha256=' <<<"$(sed -n 1p <<<"$lines")" \
    || ! grep -q '^ctl-test alpha control=guard scenario=alpha witnessed=red ' <<<"$(sed -n 2p <<<"$lines")"; then
    echo "  FAIL: the control line does not follow its suite line witnessed red: $lines"; fails=$((fails + 1))
fi
if ! grep -q " assign=$BREAK_DIGEST\$" <<<"$(sed -n 2p <<<"$lines")"; then
    echo "  FAIL: the control line does not record its declaration's assignments digest: $lines"; fails=$((fails + 1))
fi
if [[ ! -e "$tmp/a/.tmp/validate-alpha.control-guard.log" ]] \
    || ! grep -q '^alpha fail' "$tmp/a/.tmp/validate-alpha.control-guard.parsed" 2>/dev/null; then
    echo "  FAIL: the control's log and parse are not beside the suite's"; fails=$((fails + 1))
fi
_rv "$tmp/a" >/dev/null; rc=$?
if [[ "$rc" -ne 0 ]] || [[ "$(manifest_lines "$tmp/a" | grep -c 'control=guard')" -ne 1 ]]; then
    echo "  FAIL: a re-run did not supersede the suite's control line (rc=$rc): $(manifest_lines "$tmp/a")"; fails=$((fails + 1))
fi

# B — a suite that ignores BREAK: the control witnesses green, the run exits 1 naming the control,
#     the line is still written, and the Statement still publishes.
mk_tree "$tmp/b" 'true' "alpha alpha BREAK=1"
printf 'EVIDENCE_KIT_STATEMENT_FILE = .tmp/statement.json\n' >>"$tmp/b/scripts/evidence-config.knobs"
out="$(_rv "$tmp/b")"; rc=$?
if [[ "$rc" -ne 1 ]] || [[ "$out" != *"control 'guard' witnessed green"* ]]; then
    echo "  FAIL: a control witnessing no red must exit 1 naming it (rc=$rc): $out"; fails=$((fails + 1))
fi
if ! manifest_lines "$tmp/b" | grep -q 'control=guard scenario=alpha witnessed=green' \
    || ! manifest_lines "$tmp/b" | grep -q 'verdict=clean'; then
    echo "  FAIL: the green witness was not recorded beside a clean suite line: $(manifest_lines "$tmp/b")"; fails=$((fails + 1))
fi
if [[ ! -s "$tmp/b/.tmp/statement.json" ]]; then
    echo "  FAIL: a run whose control witnessed no red did not publish its Statement"; fails=$((fails + 1))
fi

# C — a declaration with no assignment refuses at exit 2 before the claim: no suite runs, nothing is written.
mk_tree "$tmp/c" 'true' "alpha alpha"
before="$(cat "$tmp/c/.workflow/validate-evidence.txt")"
out="$(_rv "$tmp/c")"; rc=$?
if [[ "$rc" -ne 2 ]] || [[ "$out" != *"control 'guard' carries no <VAR>=<value> assignment"* ]]; then
    echo "  FAIL: a malformed declaration must refuse at exit 2 naming it (rc=$rc): $out"; fails=$((fails + 1))
fi
if [[ -e "$tmp/c/.tmp/suite.log" ]] || [[ "$(cat "$tmp/c/.workflow/validate-evidence.txt")" != "$before" ]]; then
    echo "  FAIL: a refused declaration still ran a suite or wrote the manifest"; fails=$((fails + 1))
fi

if [[ "$fails" -gt 0 ]]; then
    echo "test-control-run.test: $fails assertion(s) failed"
    exit 1
fi
echo "test-control-run.test: ok (red witness after its suite line at exit 0, carrying its assignments digest, and superseded on re-run; green witness exit 1 with the line and Statement written; malformed declaration exit 2 with nothing run)"
exit 0
