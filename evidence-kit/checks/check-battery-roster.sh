#!/usr/bin/env bash
# graph: couples=README.md,scripts/evidence-config.sh,gate-sdk/lib/gate.sh dir=bi valve=none tier=precommit
# install: on-surface
# spec: evidence-kit/SPEC.md §check-battery-roster — the runner doc's battery-roster block holds name-set parity with EVIDENCE_KIT_SUITES, both directions
#
# usage: check-battery-roster.sh [runner-doc]
#   bare: EVIDENCE_KIT_RUNNER_DOC (default README.md) against the git toplevel;
#   positional: taken as given, so a fixture tree can be pointed at.
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/evidence.sh
source "$KIT/lib/evidence.sh"

BEGIN="<!-- battery-roster:begin -->"
END="<!-- battery-roster:end -->"

DOC="${1:-}"
if [[ -z "$DOC" ]]; then
    top="$(git rev-parse --show-toplevel)" \
        || { echo "check-battery-roster: not a git repository and no runner-doc argument" >&2; exit 2; }
    DOC="$top/${EVIDENCE_KIT_RUNNER_DOC:-README.md}"
fi
[[ -f "$DOC" ]] || { echo "check-battery-roster: runner doc not found: $DOC" >&2; exit 2; }
[[ ${#EVIDENCE_KIT_SUITES[@]} -gt 0 ]] \
    || { echo "check-battery-roster: EVIDENCE_KIT_SUITES is empty — no suite roster to hold $DOC against" >&2; exit 2; }
grep -qF -- "$BEGIN" "$DOC" \
    || { echo "check-battery-roster: no '$BEGIN' marker block in $DOC" >&2; exit 2; }

# spec: evidence-kit/SPEC.md §check-battery-roster — a suite's documented invocation is EVIDENCE_KIT_RUN_<suite> minus a leading 'env' and its VAR=value assignments, the validate harness's environment being no part of what a contributor types
normalize_invocation() {
    local -a t=()
    read -r -a t <<<"${1-}"
    local out="" tok head=1
    for tok in ${t[@]+"${t[@]}"}; do
        if [[ "$head" -eq 1 ]]; then
            [[ "$tok" == "env" || "$tok" =~ ^[A-Za-z_][A-Za-z0-9_]*= ]] && continue
            head=0
        fi
        out+="${out:+ }$tok"
    done
    printf '%s\n' "$out"
}

roster="$(awk -v b="$BEGIN" -v e="$END" '
    { line = $0; sub(/#.*$/, "", line); gsub(/^[ \t]+|[ \t\r]+$/, "", line); gsub(/[ \t]+/, " ", line) }
    line == b { inb = 1; next }
    line == e { inb = 0; next }
    inb && line ~ /^[a-z][a-z0-9_-]* / { print FNR ":" line }
' "$DOC")"; st=$?
fail_closed "$st" check-battery-roster awk

declare -A roster_line=()
while IFS= read -r r; do
    [[ -n "$r" ]] || continue
    roster_line["${r#*:}"]="${r%%:*}"
done <<<"$roster"

declare -A suite_of=()
for s in "${EVIDENCE_KIT_SUITES[@]}"; do
    c="$(normalize_invocation "$(ek_suite_cmd "$s")")"
    [[ -n "$c" ]] && suite_of["$c"]="$s"
done

findings=()
# assertion A: every configured suite's documented invocation is a roster line
for c in "${!suite_of[@]}"; do
    [[ -n "${roster_line[$c]:-}" ]] \
        || findings+=("$DOC: suite '${suite_of[$c]}' is absent from the battery-roster block — no line reads '$c'")
done
# assertion B: every roster line resolves to a configured suite
for c in "${!roster_line[@]}"; do
    [[ -n "${suite_of[$c]:-}" ]] \
        || findings+=("$DOC:${roster_line[$c]}: roster line runs no configured suite: '$c'")
done

if [[ ${#findings[@]} -gt 0 ]]; then
    printf '%s\n' "check-battery-roster: the battery-roster block is out of parity with EVIDENCE_KIT_SUITES:"
    printf '  %s\n' "${findings[@]}" | sort
    echo "  help: keep the block in name-set parity with the configured suites — add the"
    echo "        missing suite's documented invocation, or drop the line whose command"
    echo "        runs no configured suite (evidence-kit/SPEC.md §check-battery-roster)."
    exit 1
fi

echo "BATTERY-ROSTER: clean (${#EVIDENCE_KIT_SUITES[@]} configured suite(s) in name-set parity with the battery-roster block in $DOC)"
exit 0
