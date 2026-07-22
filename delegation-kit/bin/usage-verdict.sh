#!/usr/bin/env bash
# spec: delegation-kit/SPEC.md §usage-verdict — trustworthy budget verdict from usage.txt
# usage: usage-verdict.sh [usage-file [credentials-file]]   ($1/$2 override paths for test injection)
#   exit: 0 OK / RESET-OK, 1 PAUSE, 2 STALE or unreadable (budget unknown, never blocks)
set -euo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/delegation.sh
source "$KIT/lib/delegation.sh"

USAGE_FILE="${1:-$DELEGATION_KIT_USAGE_FILE}"
CRED_FILE="${2:-$DELEGATION_KIT_CRED_FILE}"
PAUSE_PCT="$DELEGATION_KIT_PAUSE_PCT"
PAUSE_PCT_7D="$DELEGATION_KIT_PAUSE_PCT_7D"
STALE_AGE="$DELEGATION_KIT_STALE_AGE"
LOGIN_WINDOW="$DELEGATION_KIT_LOGIN_WINDOW"
HISTORY="$DELEGATION_KIT_USAGE_HISTORY"
WIDTH="$DELEGATION_KIT_FAN_WIDTH"
REFRESH_CMD="$DELEGATION_KIT_REFRESH_CMD"
REFRESH_MIN_AGE="$DELEGATION_KIT_REFRESH_MIN_AGE"

# spec: delegation-kit/SPEC.md §usage-verdict — demand-driven refresh: the verdict call is the trigger, short-circuited under REFRESH_MIN_AGE so the render path cannot hammer the source; fail-soft, the snapshot's own staleness judges the result
if [[ -n "$REFRESH_CMD" ]]; then
  refresh=1
  if [[ -r "$USAGE_FILE" ]]; then
    snap_updated="$(awk -F= '$1 == "updated_at" { print $2; exit }' "$USAGE_FILE" 2>/dev/null || true)"
    if [[ "$snap_updated" =~ ^[0-9]+$ ]] && (( $(date +%s) - snap_updated < REFRESH_MIN_AGE )); then
      refresh=0
    fi
  fi
  if (( refresh )); then
    bash -c "$REFRESH_CMD" >/dev/null 2>&1 || true
  fi
fi

if [[ ! -r "$USAGE_FILE" ]]; then
  echo "usage-verdict: cannot read $USAGE_FILE width=${WIDTH} -> STALE (never blocks delegation — re-read or refresh before trusting the number)"
  exit 2
fi

pct="" ; resets_at="" ; updated_at=""
pct_7d="" ; resets_7d="" ; account="" ; tier="" ; tokens_in="" ; tokens_out=""
while IFS='=' read -r key val; do
  case "$key" in
    five_hour_used_pct)   pct="$val" ;;
    five_hour_resets_at)  resets_at="$val" ;;
    updated_at)           updated_at="$val" ;;
    seven_day_used_pct)   pct_7d="$val" ;;
    seven_day_resets_at)  resets_7d="$val" ;;
    account)              account="$val" ;;
    tier)                 tier="$val" ;;
    tokens_in)            tokens_in="$val" ;;
    tokens_out)           tokens_out="$val" ;;
  esac
done < "$USAGE_FILE"

if [[ -z "$pct" || -z "$resets_at" || -z "$updated_at" ]]; then
  echo "usage-verdict: missing key(s) in $USAGE_FILE (pct='$pct' resets_at='$resets_at' updated_at='$updated_at') width=${WIDTH} -> STALE (never blocks delegation — re-read or refresh before trusting the number)"
  exit 2
fi

if [[ ! "$pct" =~ ^[0-9]+(\.[0-9]+)?$ ]]; then
  echo "usage-verdict: non-numeric five_hour_used_pct='$pct' in $USAGE_FILE width=${WIDTH} -> STALE (never blocks delegation — re-read or refresh before trusting the number)"
  exit 2
fi

now=$(date +%s)
age=$(( now - updated_at ))
resets_in=$(( resets_at - now ))

# spec: delegation-kit/SPEC.md §usage-verdict — login_at dates the last auth event; read once for the reroute and the sample line
login_at=0
if [[ -r "$CRED_FILE" ]]; then
  login_at=$(stat -c %Y "$CRED_FILE" 2>/dev/null || echo 0)
fi

# spec: delegation-kit/SPEC.md §The usage.txt contract — the sample line is the trend log's wire contract: raw values verbatim, optional keys omitted (never empty) when their source is absent
append_sample() {
  local verdict="$1" line
  [[ -n "$HISTORY" ]] || return 0
  line="updated_at=${updated_at} pct=${pct} resets_at=${resets_at} verdict=${verdict} login_at=${login_at}"
  [[ -n "$account" ]]                            && line+=" account=${account}"
  [[ -n "$tier" ]]                               && line+=" tier=${tier}"
  [[ -n "$pct_7d" && -n "$resets_7d" ]]          && line+=" pct_7d=${pct_7d} resets_7d=${resets_7d}"
  [[ -n "$tokens_in" && -n "$tokens_out" ]]      && line+=" tokens_in=${tokens_in} tokens_out=${tokens_out}"
  mkdir -p "$(dirname "$HISTORY")" 2>/dev/null || true
  printf '%s\n' "$line" >> "$HISTORY"
}

if (( resets_in <= 0 )); then
  append_sample RESET-OK
  echo "used=${pct}% age=${age}s resets_in=${resets_in}s width=${WIDTH} -> RESET-OK (window rolled over ${resets_in#-}s ago; pct is from the dead window, re-read for the live value)"
  exit 0
fi

if (( age > STALE_AGE )); then
  append_sample STALE
  echo "used=${pct}% age=${age}s resets_in=${resets_in}s width=${WIDTH} -> STALE (reading older than ${STALE_AGE}s; pct may lag reality; never blocks delegation — re-read or refresh before trusting the number)"
  exit 2
fi

# spec: delegation-kit/SPEC.md §usage-verdict — two pause axes judged independently; the weekly axis arms only when both seven_day keys are present and its window is live
pause_5h=0
awk -v p="$pct" -v t="$PAUSE_PCT" 'BEGIN { exit !(p >= t) }' && pause_5h=1

pause_7d=0
if [[ -n "$pct_7d" && -n "$resets_7d" && "$pct_7d" =~ ^[0-9]+(\.[0-9]+)?$ && "$resets_7d" =~ ^-?[0-9]+$ ]]; then
  if (( resets_7d - now > 0 )); then
    awk -v p="$pct_7d" -v t="$PAUSE_PCT_7D" 'BEGIN { exit !(p >= t) }' && pause_7d=1
  fi
fi

if (( pause_5h || pause_7d )); then
  append_sample PAUSE
  if (( pause_7d )); then
    echo "used=${pct}% (7d ${pct_7d}%) age=${age}s resets_in=${resets_in}s width=${WIDTH} -> PAUSE (7-day window; at or over ${PAUSE_PCT_7D}% of the live weekly window — remediation is days, not hours)"
  else
    echo "used=${pct}% age=${age}s resets_in=${resets_in}s width=${WIDTH} -> PAUSE (5h window; at or over ${PAUSE_PCT}% of the live 5h window)"
  fi
  exit 1
fi

# spec: delegation-kit/SPEC.md §usage-verdict — the reroute follows the axis compares and may suppress only the non-blocking outcome: a lagging under-threshold pct still cannot print a fresh-looking OK, while an over-threshold reading inside the window still blocks
cred_age=$(( now - login_at ))
if (( login_at > 0 && cred_age >= 0 && cred_age < LOGIN_WINDOW )); then
  append_sample STALE
  echo "used=${pct}% age=${age}s resets_in=${resets_in}s width=${WIDTH} -> STALE (auth changed ${cred_age}s ago; a /login starts fresh windows the server-fed pct lags; never blocks delegation — re-read or refresh before trusting the number)"
  exit 2
fi

append_sample OK
echo "used=${pct}% age=${age}s resets_in=${resets_in}s width=${WIDTH} -> OK"
exit 0
