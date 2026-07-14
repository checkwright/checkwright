#!/usr/bin/env bash
# spec: drift-kit/SPEC.md §Bundled KPIs — kpi-overhead: governance + gate-output share over the overhead meter's log
set -uo pipefail

: "${DRIFT_KIT_TMP_DIR:=${GATE_SDK_TMP_DIR:-.tmp}}"
LOG="${DRIFT_KIT_OVERHEAD_LOG:-$DRIFT_KIT_TMP_DIR/overhead-log.txt}"
WINDOW=10   # sessions averaged: recent enough to track the trend, wide enough to damp per-session noise

na() { [[ "${1:-}" == "--trend" ]] && exit 0; printf 'lead\toverhead\tn/a (%s)\n' "$2"; exit 0; }

[[ -s "$LOG" ]] || na "${1:-}" "no measurement yet — run bin/overhead-meter.sh"

read -r n avgpct gatepct lastdate < <(
    tail -n "$WINDOW" "$LOG" | awk '
        {
            pct = 0; tot = 0; g = 0
            for (i = 1; i <= NF; i++) {
                if ($i ~ /^pct=/)   { split($i, a, "="); pct = a[2] }
                if ($i ~ /^total=/) { split($i, a, "="); tot = a[2] }
                if ($i ~ /^gate=/)  { split($i, a, "="); g   = a[2] }
            }
            sp += pct; st += tot; sg += g; n++; last = $1
        }
        END {
            ap = n  ? int((sp / n) + 0.5)       : 0
            gp = st ? int((sg * 100 / st) + 0.5) : 0
            printf "%d %d %d %s\n", n, ap, gp, last
        }
    '
)

if [[ "${1:-}" == "--trend" ]]; then
    printf 'ovh %d%%\n' "$avgpct"
    exit 0
fi

now_s="$(date +%s)"
last_s="$(date -d "$lastdate" +%s 2>/dev/null)" || last_s="$now_s"
days=$(( (now_s - last_s) / 86400 )); (( days < 0 )) && days=0

printf 'lead\toverhead (gov share)\t%d%% over %d session(s), as of %s (%dd; byte-proxy)\n' \
    "$avgpct" "$n" "$lastdate" "$days"
printf 'lead\toverhead (gate share)\t%d%% of volume is gate output (byte-proxy)\n' "$gatepct"
exit 0
