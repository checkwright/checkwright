#!/usr/bin/env bash
# graph: couples=scripts/*.sh,gate-sdk/*.sh,lifecycle-kit/*.sh dir=one valve=none tier=precommit
# spec: gate-sdk/SPEC.md §check-gate-fail-closed — every awk/jq capture in the gate family handles subprocess exit status
#
# usage: check-gate-fail-closed.sh [dir...]
#   Scans check-*.sh under each given dir. Default: the consumer gates dir plus
#   each vendored kit's checks/.
set -uo pipefail

SDK="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/gate.sh
source "$SDK/lib/gate.sh"

if [[ $# -gt 0 ]]; then
    DIRS=("$@")
else
    mapfile -t DIRS < <(gate_check_dirs)
fi
gates=()
shopt -s nullglob
for d in "${DIRS[@]}"; do
    gates+=("$d"/check-*.sh)
done
shopt -u nullglob
[[ ${#gates[@]} -gt 0 ]] || { echo "check-gate-fail-closed: no check-*.sh gates found under: ${DIRS[*]}" >&2; exit 2; }

out="$(awk '
    BEGIN { Q = sprintf("%c", 39) }
    function isparser(line) { return (line ~ /(^|[^A-Za-z])(awk|jq)[[:space:]]/) }
    function sat(line) {
        if (line ~ /fail_closed/)         return 1
        if (line ~ /=\$\?/)               return 1
        if (line ~ /\|\|[[:space:]]*\{/)  return 1
        if (line ~ /fail-closed-exempt/)  return 1
        return 0
    }
    function nquote(line,   n, t) { t = line; n = gsub(Q, "", t); return n }
    function close_if(line) {
        if (line ~ /\)"/ && qc % 2 == 0) {
            inblk = 0
            if (hasparser && !heredoc && !satisfied)
                printf "  %s:%d: awk/jq capture branches on output without a fail-closed status check:\n      %s\n", FILENAME, blkstart, opentext
        }
    }
    function scan(line) {
        qc += nquote(line)
        if (isparser(line)) hasparser = 1
        if (sat(line))      satisfied = 1
        if (line ~ /<<</)   heredoc = 1
        close_if(line)
    }
    FNR == 1 { inblk = 0; pend = 0; qc = 0 }
    !inblk && /^[[:space:]]*#/ { if ($0 ~ /fail-closed-exempt/) pend = 1; next }
    !inblk && /^[[:space:]]*$/ { pend = 0; next }
    !inblk && /[A-Za-z_][A-Za-z0-9_]*="\$\(/ && $0 !~ /="\$\(\(/ {
        inblk = 1; blkstart = FNR; opentext = $0
        hasparser = 0; satisfied = 0; heredoc = 0; qc = 0
        if (pend) satisfied = 1
        pend = 0
        scan($0)
        next
    }
    inblk { scan($0); next }
    { pend = 0 }
' "${gates[@]}")"; st=$?
fail_closed "$st" check-gate-fail-closed awk

if [[ -n "$out" ]]; then
    echo "check-gate-fail-closed: gate(s) capture a parser's output but branch on"
    echo "emptiness without checking the subprocess status (gate-sdk/SPEC.md"
    echo "§Fail-closed contract — a crash would false-green as 'clean'):"
    echo ""
    echo "$out"
    echo ""
    echo "  help: capture the status and route it through the shared helper —"
    echo "        out=\"\$(awk '…' \"\$FILE\")\"; st=\$?"
    echo "        fail_closed \"\$st\" check-<name> awk"
    echo "  (source gate-sdk/lib/gate.sh near the top), or annotate a"
    echo "  genuinely-safe capture with '# fail-closed-exempt: <reason>'."
    exit 1
fi

echo "GATE-FAIL-CLOSED: clean (every awk/jq capture in the gate family handles subprocess status)"
exit 0
