#!/usr/bin/env bash
# spec: delegation-kit/SPEC.md §The usage.txt contract — consumer-copy statusLine producer: writes the snapshot usage-verdict reads and renders the reference status bar; requires jq
input=$(cat)

MODEL=$(echo "$input" | jq -r '.model.display_name // empty')
MODEL=${MODEL%% *}; MODEL=${MODEL,,}; MODEL=${MODEL//[^a-z0-9]/}
EFFORT=$(echo "$input" | jq -r '.effort.level // empty')
CTX=$(echo "$input" | jq -r '.context_window.used_percentage // 0' | cut -d. -f1)
FIVE_H=$(echo "$input" | jq -r '.rate_limits.five_hour.used_percentage // empty')
FIVE_H_RESETS=$(echo "$input" | jq -r '.rate_limits.five_hour.resets_at // empty')
SEVEN_D=$(echo "$input" | jq -r '.rate_limits.seven_day.used_percentage // empty')
SEVEN_D_RESETS=$(echo "$input" | jq -r '.rate_limits.seven_day.resets_at // empty')

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

# spec: delegation-kit/SPEC.md §The statusline template — the section vocabulary is the counter tool's, never this template's: each label is the initial of a name the tool returned, widened to two characters for every counter as soon as two returned names share one, so a colliding pair stays readable and the group stays aligned
_counters() {
  local name n labels=() nums=() dup=0 out="" i j w=1
  while IFS=$'\t' read -r name n; do
    [ -n "$name" ] && { labels+=("$name"); nums+=("$n"); }
  done <<< "$1"
  [ "${#labels[@]}" -gt 0 ] || return
  for (( i = 0; i < ${#labels[@]}; i++ )); do
    for (( j = i + 1; j < ${#labels[@]}; j++ )); do
      [ "${labels[i]:0:1}" = "${labels[j]:0:1}" ] && dup=1
    done
  done
  [ "$dup" -eq 1 ] && w=2
  for (( i = 0; i < ${#labels[@]}; i++ )); do out+="${out:+ }${labels[i]:0:$w}${nums[i]}"; done
  printf '%s' "$out"
}

ITER=""; STAGE=""; COUNTS=""
ROOT=$( { cd "$(git rev-parse --show-toplevel 2>/dev/null)" && pwd -P; } 2>/dev/null )
if [ -n "$ROOT" ] && [ -f "$ROOT/TASK-QUEUE.md" ]; then
  HEADER=$(grep -m1 '^## Iteration:' "$ROOT/TASK-QUEUE.md")
  ITER=$(printf '%s' "$HEADER" | sed -E 's/^## Iteration:[[:space:]]*//; s/[[:space:]]*\[stage:.*$//')
fi
# spec: delegation-kit/SPEC.md §The statusline template — the stage is the state file's last data line, so an absent file or one with no data line leaves STAGE empty and the render drops the @stage suffix rather than printing a partial parse
if [ -n "$ROOT" ] && [ -f "$ROOT/.workflow/WORKFLOW-STATE.txt" ]; then
  STAGE=$(awk '/^---[[:space:]]*$/ { f = 1; next } f && NF { l = $2 } END { print l }' "$ROOT/.workflow/WORKFLOW-STATE.txt")
fi
# spec: delegation-kit/SPEC.md §The statusline template — a subprocess and never a source: the counter's library exits 2 at source time on a malformed queue config, which in this shell would take the whole status bar down for a component worth four characters
if [ -n "$ROOT" ] && [ -x "$ROOT/queue-kit/bin/queue-counts.sh" ]; then
  COUNTS=$(bash "$ROOT/queue-kit/bin/queue-counts.sh" 2>/dev/null) || COUNTS=""
fi

USAGE_FILE="${DELEGATION_KIT_USAGE_FILE:-${CLAUDE_CONFIG_DIR:-$HOME/.claude}/usage.txt}"
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

# spec: delegation-kit/SPEC.md §The usage.txt contract — optional dense sampling drives usage-verdict from the render path (the single append author); a consumer opts in with DELEGATION_KIT_USAGE_HISTORY set

SB="[${MODEL:-?}${EFFORT:+-$EFFORT}]·ctx $(_gauge "$CTX")"
SB="$SB·5h $(_gauge "${FIVE_H:-0}")"; [ -n "$FIVE_H_RESETS" ] && SB="$SB $(_remaining "$FIVE_H_RESETS")"
[ -n "$SEVEN_D" ] && { SB="$SB·7d $(_gauge "$SEVEN_D")"; [ -n "$SEVEN_D_RESETS" ] && SB="$SB $(_remaining "$SEVEN_D_RESETS")"; }
[ -n "$ITER" ] && SB="$SB·⟳ ${ITER}${STAGE:+@$STAGE}"
[ -n "$COUNTS" ] && { GROUP=$(_counters "$COUNTS"); [ -n "$GROUP" ] && SB="$SB·$GROUP"; }
printf '%s\n' "$SB"
