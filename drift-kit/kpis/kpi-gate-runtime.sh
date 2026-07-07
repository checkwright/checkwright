#!/usr/bin/env bash
# spec: drift-kit/SPEC.md §Bundled KPIs — kpi-gate-runtime: full-battery runtime from the runner's timings file
#
# Lead. Reads $DRIFT_KIT_TIMINGS_FILE (the runner's per-gate + TOTAL lines):
# total, the three slowest gates, and the file's reading age. A measurement,
# not live state, so the age caveat rides the value. Degrades to n/a when no
# timings file exists yet.
set -uo pipefail

TIMINGS="${DRIFT_KIT_TIMINGS_FILE:-${GATE_SDK_TMP_DIR:-.tmp}/gate-timings.txt}"

na() { [[ "${1:-}" == "--trend" ]] && exit 0; printf 'lead\tgate runtime\tn/a (%s)\n' "$2"; exit 0; }

[[ -s "$TIMINGS" ]] || na "${1:-}" "no timings file — run the battery"

total="$(awk '$1=="TOTAL"{print $2; exit}' "$TIMINGS" 2>/dev/null)"
[[ "$total" =~ ^[0-9]+$ ]] || na "${1:-}" "no TOTAL line"

now="$(date +%s)"
mtime="$(date -r "$TIMINGS" +%s 2>/dev/null)" || mtime="$now"
secs=$(( now - mtime )); (( secs < 0 )) && secs=0
if   (( secs < 90 ));    then age="${secs}s ago"
elif (( secs < 5400 ));  then age="$(( secs / 60 ))m ago"
elif (( secs < 172800 ));then age="$(( secs / 3600 ))h ago"
else                          age="$(( secs / 86400 ))d ago"
fi

if [[ "${1:-}" == "--trend" ]]; then
    printf 'gates %dms\n' "$total"
    exit 0
fi

slowest="$(awk '$1!="TOTAL" && $2 ~ /^[0-9]+$/ {print $2"\t"$1}' "$TIMINGS" 2>/dev/null \
    | sort -rn | head -3 | awk -F'\t' '{printf "%s%s %sms", (NR>1?", ":""), $2, $1}')"
printf 'lead\tgate runtime\ttotal %dms; slowest %s (read %s)\n' "$total" "$slowest" "$age"
exit 0
