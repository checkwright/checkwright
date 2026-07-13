#!/usr/bin/env bash
# spec: delegation-kit/SPEC.md §The usage.txt contract — consumer-copy timer-driven poll producer: one poll cycle per invocation, atomic snapshot rewrite, fail-soft; requires curl + jq
set -uo pipefail

fail() {
    echo "usage-poller: $1" >&2
    echo "  help: $2" >&2
    exit 1
}

USAGE_FILE="${DELEGATION_KIT_USAGE_FILE:-${CLAUDE_CONFIG_DIR:-$HOME/.claude}/usage.txt}"
CRED_FILE="${DELEGATION_KIT_CRED_FILE:-${USAGE_FILE%/*}/.credentials.json}"
ACCOUNT_CONFIG="${DELEGATION_KIT_ACCOUNT_CONFIG:-$HOME/.claude.json}"
ENDPOINT="${DELEGATION_KIT_USAGE_ENDPOINT:-https://api.anthropic.com/api/oauth/usage}"

command -v curl >/dev/null 2>&1 || fail "curl not found" \
    "install curl; the poller fetches the usage source over HTTPS (file:// for a test stub)."
command -v jq >/dev/null 2>&1 || fail "jq not found" \
    "install jq; the poller maps the source payload onto the snapshot contract."

[ -r "$CRED_FILE" ] || fail "credentials file unreadable: $CRED_FILE" \
    "point DELEGATION_KIT_CRED_FILE at the harness credentials file; the snapshot is untouched."
TOKEN="$(jq -r '.claudeAiOauth.accessToken // empty' "$CRED_FILE" 2>/dev/null)"
[ -n "$TOKEN" ] || fail "no OAuth token in $CRED_FILE" \
    "log the harness in to refresh the credentials file; the snapshot is untouched."

payload="$(curl -fsS --max-time 30 \
    -H "Authorization: Bearer $TOKEN" \
    -H "anthropic-beta: oauth-2025-04-20" \
    "$ENDPOINT" 2>/dev/null)" || fail "fetch failed: $ENDPOINT" \
    "the snapshot is untouched — usage-verdict reads its staleness as STALE, never a silent green; if the source moved, set DELEGATION_KIT_USAGE_ENDPOINT."

epoch_of() {
    local v="$1"
    [ -n "$v" ] || return 1
    [[ "$v" =~ ^[0-9]+$ ]] && { printf '%s\n' "$v"; return 0; }
    date -d "$v" +%s 2>/dev/null
}

FIVE_H="$(jq -r '.five_hour.utilization // .rate_limits.five_hour.used_percentage // empty' <<<"$payload" 2>/dev/null)"
FIVE_H_RESETS_RAW="$(jq -r '.five_hour.resets_at // .rate_limits.five_hour.resets_at // empty' <<<"$payload" 2>/dev/null)"
[[ "$FIVE_H" =~ ^[0-9.]+$ ]] || fail "unparseable payload: no five-hour utilization at $ENDPOINT" \
    "the snapshot is untouched; the source shape moved — adjust the copy or DELEGATION_KIT_USAGE_ENDPOINT."
FIVE_H_RESETS="$(epoch_of "$FIVE_H_RESETS_RAW")" && [[ "$FIVE_H_RESETS" =~ ^[0-9]+$ ]] || \
    fail "unparseable payload: no five-hour reset epoch at $ENDPOINT" \
    "the snapshot is untouched; the source shape moved — adjust the copy or DELEGATION_KIT_USAGE_ENDPOINT."

SEVEN_D="$(jq -r '.seven_day.utilization // .rate_limits.seven_day.used_percentage // empty' <<<"$payload" 2>/dev/null)"
SEVEN_D_RESETS_RAW="$(jq -r '.seven_day.resets_at // .rate_limits.seven_day.resets_at // empty' <<<"$payload" 2>/dev/null)"
SEVEN_D_RESETS=""
[ -n "$SEVEN_D_RESETS_RAW" ] && SEVEN_D_RESETS="$(epoch_of "$SEVEN_D_RESETS_RAW")"
[[ "$SEVEN_D" =~ ^[0-9.]+$ ]] || SEVEN_D=""
[[ "$SEVEN_D_RESETS" =~ ^[0-9]+$ ]] || SEVEN_D_RESETS=""

TIER=""; ACCOUNT=""
TIER="$(jq -r '.claudeAiOauth.subscriptionType // empty' "$CRED_FILE" 2>/dev/null)"
[ -r "$ACCOUNT_CONFIG" ] && ACCOUNT="$(jq -r '.oauthAccount.accountUuid // empty' "$ACCOUNT_CONFIG" 2>/dev/null)"

{
    printf 'five_hour_used_pct=%s\n'  "$FIVE_H"
    printf 'five_hour_resets_at=%s\n' "$FIVE_H_RESETS"
    printf 'updated_at=%s\n'          "$(date +%s)"
    [ -n "$SEVEN_D" ]        && printf 'seven_day_used_pct=%s\n'  "$SEVEN_D"
    [ -n "$SEVEN_D_RESETS" ] && printf 'seven_day_resets_at=%s\n' "$SEVEN_D_RESETS"
    [ -n "$ACCOUNT" ]        && printf 'account=%s\n'             "$ACCOUNT"
    [ -n "$TIER" ]           && printf 'tier=%s\n'                "$TIER"
} > "${USAGE_FILE}.tmp" && mv "${USAGE_FILE}.tmp" "$USAGE_FILE"
