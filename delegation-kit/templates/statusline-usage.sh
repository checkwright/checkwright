#!/usr/bin/env bash
# CONSUMER COPY — a minimal usage.txt producer (delegation-kit/SPEC.md §usage-verdict,
# The usage.txt contract). Wire it as your harness statusLine command: it reads
# the rate-limit JSON on stdin and atomically writes the snapshot that usage-verdict
# reads. Any producer honoring the contract works — this is the floor, not the UX
# (the source platform's gauge bars and iteration display stayed behind). Requires jq.
input=$(cat)

FIVE_H=$(echo "$input" | jq -r '.rate_limits.five_hour.used_percentage // empty')
FIVE_H_RESETS=$(echo "$input" | jq -r '.rate_limits.five_hour.resets_at // empty')
# Optional keys, written only when their source exposes a value (never written empty).
# The weekly pair arms usage-verdict's second pause axis; account/tier are read by
# usage-trend for per-account/tier segmentation. tokens_in/tokens_out are defined in
# the contract for third-party producers but shipped by no live producer here — the
# harness statusLine payload carries no cumulative token counts (the dead-producer rule).
SEVEN_D=$(echo "$input" | jq -r '.rate_limits.seven_day.used_percentage // empty')
SEVEN_D_RESETS=$(echo "$input" | jq -r '.rate_limits.seven_day.resets_at // empty')

USAGE_FILE="${DELEGATION_KIT_USAGE_FILE:-${CLAUDE_CONFIG_DIR:-$HOME/.claude}/usage.txt}"
# account/tier live in the local account config, not the stdin payload: the account
# UUID identifies whom a login switch is to (usage-trend groups per account), the
# subscription tier is the denominator behind the percentages.
CRED_FILE="${DELEGATION_KIT_CRED_FILE:-${USAGE_FILE%/*}/.credentials.json}"
ACCOUNT_CONFIG="${DELEGATION_KIT_ACCOUNT_CONFIG:-$HOME/.claude.json}"
TIER=""; ACCOUNT=""
[[ -r "$CRED_FILE" ]]      && TIER=$(jq -r '.claudeAiOauth.subscriptionType // empty' "$CRED_FILE" 2>/dev/null)
[[ -r "$ACCOUNT_CONFIG" ]] && ACCOUNT=$(jq -r '.oauthAccount.accountUuid // empty' "$ACCOUNT_CONFIG" 2>/dev/null)

{
  printf 'five_hour_used_pct=%s\n'  "${FIVE_H:-}"
  printf 'five_hour_resets_at=%s\n' "${FIVE_H_RESETS:-}"
  printf 'updated_at=%s\n'          "$(date +%s)"
  [[ -n "$SEVEN_D" ]]        && printf 'seven_day_used_pct=%s\n'  "$SEVEN_D"
  [[ -n "$SEVEN_D_RESETS" ]] && printf 'seven_day_resets_at=%s\n' "$SEVEN_D_RESETS"
  [[ -n "$ACCOUNT" ]]        && printf 'account=%s\n'             "$ACCOUNT"
  [[ -n "$TIER" ]]           && printf 'tier=%s\n'                "$TIER"
} > "${USAGE_FILE}.tmp" && mv "${USAGE_FILE}.tmp" "$USAGE_FILE"

# Optional dense sampling: this statusLine fires far more often than the per-session /
# per-dispatch usage-verdict calls, so a consumer wanting a denser trend history can
# append a sample every render by invoking usage-verdict here (it appends one line when
# DELEGATION_KIT_USAGE_HISTORY is set, whatever the verdict) — the verdict path is the
# single append author, so no snapshot key names need to be duplicated:
#   [ -n "${DELEGATION_KIT_USAGE_HISTORY:-}" ] && bash delegation-kit/bin/usage-verdict.sh >/dev/null 2>&1 || true

# A statusLine command must print the status line — emit a minimal one; replace
# with your own UX.
printf '5h %s%%\n' "${FIVE_H:-?}"
