#!/usr/bin/env bash
# spec: evidence-kit/SPEC.md §bin/diff-baseline.sh — the situational runtime diff (not a precommit gate): parse the captured logs passed as arguments, diff each against the baseline's suite slice per-scenario, print findings. Reads the skip side-channel to demote self-skipped scenarios from pass. Each group is <suite> <logfile> [<status>], and an exit-code suite must carry its status or the tool refuses rather than assuming success.
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/evidence.sh
source "$KIT/lib/evidence.sh"

usage() {
    echo "usage: diff-baseline.sh <suite> <logfile> [<status>] [<suite> <logfile> [<status>]...]" >&2
    echo "  <status> is the suite command's own exit status. A suite whose parser reads the" >&2
    echo "  log may omit it; an 'exit-code' suite may not, because the status is its verdict." >&2
    exit 2
}

[[ $# -ge 2 ]] || usage

mkdir -p "$EVIDENCE_KIT_TMP_DIR"
recoveries=0
rc=0
while [[ $# -gt 0 ]]; do
    [[ $# -ge 2 ]] || usage
    suite="$1"; log="$2"; shift 2
    # spec: evidence-kit/SPEC.md §bin/diff-baseline.sh — a suite name suffixes EVIDENCE_KIT_RUN_<suite>, so it is a shell identifier and cannot be all digits; that is what makes the optional status a total disambiguation rather than a heuristic
    status=""
    if [[ $# -gt 0 && "$1" =~ ^[0-9]+$ ]]; then status="$1"; shift; fi
    [[ -f "$log" ]] || { echo "diff-baseline: log not found: $log" >&2; exit 2; }
    if [[ -z "$status" ]]; then
        # spec: evidence-kit/SPEC.md §bin/diff-baseline.sh — refuse rather than assume success: an exit-code suite parsed against an assumed 0 reports pass for every log it is ever handed, so the tool would clear a red it structurally cannot see
        if [[ "$(ek_parser_for "$suite")" == exit-code ]]; then
            echo "diff-baseline: suite '$suite' is parsed by exit code and no status was given." >&2
            echo "  help: pass the suite's own exit status as a third argument —" >&2
            echo "        diff-baseline.sh $suite $log <status>" >&2
            echo "        Without it this tool cannot observe a failure in that suite at all." >&2
            exit 2
        fi
        status=0
    fi
    parsed="$EVIDENCE_KIT_TMP_DIR/diff-$suite.parsed"
    ek_parse "$suite" "$log" "$status" >"$parsed"
    out="$(ek_diff "$EVIDENCE_KIT_BASELINE_FILE" "$suite" "$parsed" "$EVIDENCE_KIT_SKIP_FILE")" || rc=1
    if [[ -n "$out" ]]; then
        printf '%s\n' "$out"
        recoveries=$((recoveries + $(grep -c '^recovery ' <<<"$out" || true)))
    fi
done

if [[ "$rc" -ne 0 ]]; then
    echo "diff-baseline: NEW failures against $EVIDENCE_KIT_BASELINE_FILE (see 'new-failure' lines above)"
    exit 1
fi
echo "diff-baseline: clean ($recoveries unpromoted recovery finding(s); no new failure vs $EVIDENCE_KIT_BASELINE_FILE)"
exit 0
