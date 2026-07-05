#!/usr/bin/env bash
# spec: delegation-kit/SPEC.md §usage-gate — trustworthy budget verdict from usage.txt
#
# Extracted from the governance meta-layer of a private production platform; the
# single-operator source path and thresholds are config knobs (lib/delegation.sh),
# platform values as defaults. Positional $1/$2 override the snapshot/credentials
# path for test injection.
#
#   usage-gate.sh [usage-file [credentials-file]]
# Exit: 0 OK / RESET-OK, 1 PAUSE, 2 STALE or unreadable (fail-closed).
set -euo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/delegation.sh
source "$KIT/lib/delegation.sh"

USAGE_FILE="${1:-$DELEGATION_KIT_USAGE_FILE}"
CRED_FILE="${2:-$DELEGATION_KIT_CRED_FILE}"
PAUSE_PCT="$DELEGATION_KIT_PAUSE_PCT"
STALE_AGE="$DELEGATION_KIT_STALE_AGE"
LOGIN_WINDOW="$DELEGATION_KIT_LOGIN_WINDOW"

if [[ ! -r "$USAGE_FILE" ]]; then
  echo "usage-gate: cannot read $USAGE_FILE -> STALE"
  exit 2
fi

pct="" ; resets_at="" ; updated_at=""
while IFS='=' read -r key val; do
  case "$key" in
    five_hour_used_pct)  pct="$val" ;;
    five_hour_resets_at) resets_at="$val" ;;
    updated_at)          updated_at="$val" ;;
  esac
done < "$USAGE_FILE"

if [[ -z "$pct" || -z "$resets_at" || -z "$updated_at" ]]; then
  echo "usage-gate: missing key(s) in $USAGE_FILE (pct='$pct' resets_at='$resets_at' updated_at='$updated_at') -> STALE"
  exit 2
fi

if [[ ! "$pct" =~ ^[0-9]+(\.[0-9]+)?$ ]]; then
  echo "usage-gate: non-numeric five_hour_used_pct='$pct' in $USAGE_FILE -> STALE"
  exit 2
fi

now=$(date +%s)
age=$(( now - updated_at ))
resets_in=$(( resets_at - now ))

if (( resets_in <= 0 )); then
  echo "used=${pct}% age=${age}s resets_in=${resets_in}s -> RESET-OK (window rolled over ${resets_in#-}s ago; pct is from the dead window, re-read for the live value)"
  exit 0
fi

if (( age > STALE_AGE )); then
  echo "used=${pct}% age=${age}s resets_in=${resets_in}s -> STALE (reading older than ${STALE_AGE}s; pct may lag reality — re-read or refresh before trusting)"
  exit 2
fi

if awk -v p="$pct" -v t="$PAUSE_PCT" 'BEGIN { exit !(p > t) }'; then
  if [[ -r "$CRED_FILE" ]]; then
    cred_mtime=$(stat -c %Y "$CRED_FILE" 2>/dev/null || echo 0)
    cred_age=$(( now - cred_mtime ))
    if (( cred_mtime > 0 && cred_age >= 0 && cred_age < LOGIN_WINDOW )); then
      echo "used=${pct}% age=${age}s resets_in=${resets_in}s -> STALE (auth changed ${cred_age}s ago — a /login starts a fresh 5h window the server-fed pct lags; re-read before trusting this ${pct}%)"
      exit 2
    fi
  fi
  echo "used=${pct}% age=${age}s resets_in=${resets_in}s -> PAUSE (over ${PAUSE_PCT}% of the live 5h window)"
  exit 1
fi

echo "used=${pct}% age=${age}s resets_in=${resets_in}s -> OK"
exit 0
