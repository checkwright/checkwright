#!/usr/bin/env bash
# spec: evidence-kit/SPEC.md §Layout and configuration — this repo's EVIDENCE_KIT_PARSER_installer_smoke adapter: one scenario per arm of installer/consumer-smoke/run-smoke.sh, the arm roster derived from that script's own top-level headers rather than listed here, and an arm the run never reached emitted as nothing at all so the baseline's absent-scenario rule judges it
set -uo pipefail

LOG="${1:-}"
SMOKE=installer/consumer-smoke/run-smoke.sh
[[ -n "$LOG" && -f "$LOG" ]] || {
    echo "parse-installer-smoke-log: log not found: $LOG" >&2
    exit 2
}
[[ -f "$SMOKE" ]] || {
    echo "parse-installer-smoke-log: smoke script not found: $SMOKE — the arm roster is derived from it" >&2
    exit 2
}

# spec: evidence-kit/SPEC.md §Layout and configuration — an arm header is a top-level `printf '<literal>\n'` with no redirect; the arm's name is the literal up to its parenthetical, which is what makes a header carrying an interpolated profile still name one stable scenario
arms=()
while IFS= read -r line; do
    [[ "$line" == printf\ \'* && "$line" != *">"* ]] || continue
    body="${line#printf \'}"
    name="${body%%\\n*}"
    [[ "$name" != "$body" ]] || continue
    name="${name%% (*}"
    case "$name" in ''|%*|INSTALLER-SMOKE*) continue ;; esac
    arms+=("$name")
done <"$SMOKE"

if [[ ${#arms[@]} -eq 0 ]]; then
    echo "parse-installer-smoke-log: no arm header derived from $SMOKE — the parser cannot judge this run" >&2
    exit 2
fi

reached=()
clean=0
while IFS= read -r line; do
    [[ "$line" == "INSTALLER-SMOKE: clean"* ]] && clean=1
    for a in "${arms[@]}"; do
        [[ "$line" == "$a" || "$line" == "$a ("* ]] || continue
        for r in ${reached[@]+"${reached[@]}"}; do [[ "$r" == "$a" ]] && continue 2; done
        reached+=("$a")
        break
    done
done <"$LOG"

# spec: evidence-kit/SPEC.md §Layout and configuration — the smoke aborts at its first failure, so every arm but the last one reached is proved by the arm that followed it; the last is proved by the run's own clean line and is `fail` without it
n=${#reached[@]}
for i in "${!reached[@]}"; do
    status=pass
    [[ "$i" -eq "$((n - 1))" && "$clean" -eq 0 ]] && status=fail
    printf '%s %s\n' "${reached[$i]// /-}" "$status"
done
