#!/usr/bin/env bash
# CONSUMER COPY — a full-UX usage.txt producer (delegation-kit/SPEC.md §usage-verdict,
# §The usage.txt contract). Wire it as your harness statusLine command (or copy it
# into your gates dir and wire that): it reads the harness rate-limit JSON on stdin,
# atomically writes the snapshot usage-verdict reads, AND renders a status bar —
# model/effort, a context gauge, the 5h and 7d rate windows with reset countdowns,
# and the iteration@stage readout. The snapshot write is the contract; the bar is
# reference UX a consumer may restyle. Requires jq.
input=$(cat)

MODEL=$(echo "$input" | jq -r '.model.display_name // empty')
MODEL=${MODEL%% *}; MODEL=${MODEL,,}; MODEL=${MODEL//[^a-z0-9]/}
EFFORT=$(echo "$input" | jq -r '.effort.level // empty')
CTX=$(echo "$input" | jq -r '.context_window.used_percentage // 0' | cut -d. -f1)
FIVE_H=$(echo "$input" | jq -r '.rate_limits.five_hour.used_percentage // empty')
FIVE_H_RESETS=$(echo "$input" | jq -r '.rate_limits.five_hour.resets_at // empty')
# The weekly pair arms usage-verdict's second pause axis; account/tier feed usage-trend's
# per-account/tier segmentation. tokens_in/tokens_out stay defined in the contract for
# third-party producers but ship no producer here — this payload carries no cumulative
# token count (the dead-producer rule).
SEVEN_D=$(echo "$input" | jq -r '.rate_limits.seven_day.used_percentage // empty')
SEVEN_D_RESETS=$(echo "$input" | jq -r '.rate_limits.seven_day.resets_at // empty')

# A 10-cell gauge: the pct centered in the bar, filled cells colored green/amber/red
# by threshold, the remainder dim. Pure ANSI — no external asset (§Self-contained).
_gauge() {
  local pct=${1%%.*} width=10 filled bg fg=$'\033[1;38;5;231m' lbl left right out="" i
  pct=${pct:-0}
  [ "$pct" -lt 0 ] && pct=0
  [ "$pct" -gt 100 ] && pct=100
  filled=$(( pct * width / 100 ))
  if   [ "$pct" -ge 80 ]; then bg=124
  elif [ "$pct" -ge 50 ]; then bg=136
  else                         bg=28
  fi
  lbl="${pct}%"
  left=$(( (width - ${#lbl}) / 2 )); right=$(( width - ${#lbl} - left ))
  printf -v lbl '%*s%s%*s' "$left" '' "$lbl" "$right" ''
  out="$fg"
  for (( i = 0; i < width; i++ )); do
    if [ "$i" -lt "$filled" ]; then out+=$'\033[48;5;'"${bg}m${lbl:i:1}"
    else                            out+=$'\033[48;5;238m'"${lbl:i:1}"; fi
  done
  printf '%s\033[0m' "$out"
}

_remaining() {
  local resets_at="$1" now remaining days hours minutes
  { [ -z "$resets_at" ] || [ "$resets_at" = "null" ]; } && return
  now=$(date +%s); remaining=$(( resets_at - now ))
  [ "$remaining" -le 0 ] && return
  days=$(( remaining / 86400 )); hours=$(( (remaining % 86400) / 3600 )); minutes=$(( (remaining % 3600) / 60 ))
  if [ "$days" -gt 0 ]; then printf '%dd%dh' "$days" "$hours"
  else                      printf '%dh%dm' "$hours" "$minutes"; fi
}

# iteration@stage from the queue header (queue-kit's format — generic mechanism).
ITER=""; STAGE=""
ROOT=$(git rev-parse --show-toplevel 2>/dev/null)
if [ -n "$ROOT" ] && [ -f "$ROOT/TASK-QUEUE.md" ]; then
  HEADER=$(grep -m1 '^## Iteration:' "$ROOT/TASK-QUEUE.md")
  ITER=$(printf '%s' "$HEADER" | sed -E 's/^## Iteration:[[:space:]]*//; s/[[:space:]]*\[stage:.*$//')
  STAGE=$(printf '%s' "$HEADER" | sed -E 's/^.*\[stage:[[:space:]]*//; s/[[:space:]]*\].*$//')
fi

USAGE_FILE="${DELEGATION_KIT_USAGE_FILE:-${CLAUDE_CONFIG_DIR:-$HOME/.claude}/usage.txt}"
# account/tier live in the local account config, not the stdin payload: the account
# UUID identifies whom a login switch is to (usage-trend groups per account); the
# subscription tier is the denominator behind the percentages.
CRED_FILE="${DELEGATION_KIT_CRED_FILE:-${USAGE_FILE%/*}/.credentials.json}"
ACCOUNT_CONFIG="${DELEGATION_KIT_ACCOUNT_CONFIG:-$HOME/.claude.json}"
TIER=""; ACCOUNT=""
[ -r "$CRED_FILE" ]      && TIER=$(jq -r '.claudeAiOauth.subscriptionType // empty' "$CRED_FILE" 2>/dev/null)
[ -r "$ACCOUNT_CONFIG" ] && ACCOUNT=$(jq -r '.oauthAccount.accountUuid // empty' "$ACCOUNT_CONFIG" 2>/dev/null)

{
  printf 'five_hour_used_pct=%s\n'  "${FIVE_H:-}"
  printf 'five_hour_resets_at=%s\n' "${FIVE_H_RESETS:-}"
  printf 'updated_at=%s\n'          "$(date +%s)"
  [ -n "$SEVEN_D" ]        && printf 'seven_day_used_pct=%s\n'  "$SEVEN_D"
  [ -n "$SEVEN_D_RESETS" ] && printf 'seven_day_resets_at=%s\n' "$SEVEN_D_RESETS"
  [ -n "$ACCOUNT" ]        && printf 'account=%s\n'             "$ACCOUNT"
  [ -n "$TIER" ]           && printf 'tier=%s\n'                "$TIER"
} > "${USAGE_FILE}.tmp" && mv "${USAGE_FILE}.tmp" "$USAGE_FILE"

# Optional dense sampling: statusLine fires far more often than the per-session /
# per-dispatch usage-verdict calls, so a consumer wanting a denser trend history can
# append a sample every render (usage-verdict logs one line when DELEGATION_KIT_USAGE_HISTORY
# is set, whatever the verdict) — the verdict path is the single append author:
#   [ -n "${DELEGATION_KIT_USAGE_HISTORY:-}" ] && bash delegation-kit/bin/usage-verdict.sh >/dev/null 2>&1 || true

SB="[${MODEL:-?}${EFFORT:+-$EFFORT}]·ctx $(_gauge "$CTX")"
SB="$SB·5h $(_gauge "${FIVE_H:-0}")"; [ -n "$FIVE_H_RESETS" ] && SB="$SB $(_remaining "$FIVE_H_RESETS")"
[ -n "$SEVEN_D" ] && { SB="$SB·7d $(_gauge "$SEVEN_D")"; [ -n "$SEVEN_D_RESETS" ] && SB="$SB $(_remaining "$SEVEN_D_RESETS")"; }
[ -n "$ITER" ] && SB="$SB·⟳ ${ITER}@${STAGE}"
printf '%s\n' "$SB"
