#!/usr/bin/env bash
# CONSUMER COPY — a minimal usage.txt producer (delegation-kit/SPEC.md §usage-verdict,
# The usage.txt contract). Wire it as your harness statusLine command: it reads
# the rate-limit JSON on stdin and atomically writes the three-line snapshot that
# usage-verdict reads. Any producer honoring the contract works — this is the floor,
# not the UX (the source platform's gauge bars and iteration display stayed
# behind). Requires jq.
input=$(cat)

FIVE_H=$(echo "$input" | jq -r '.rate_limits.five_hour.used_percentage // empty')
FIVE_H_RESETS=$(echo "$input" | jq -r '.rate_limits.five_hour.resets_at // empty')

USAGE_FILE="${DELEGATION_KIT_USAGE_FILE:-${CLAUDE_CONFIG_DIR:-$HOME/.claude}/usage.txt}"
{
  printf 'five_hour_used_pct=%s\n'  "${FIVE_H:-}"
  printf 'five_hour_resets_at=%s\n' "${FIVE_H_RESETS:-}"
  printf 'updated_at=%s\n'          "$(date +%s)"
} > "${USAGE_FILE}.tmp" && mv "${USAGE_FILE}.tmp" "$USAGE_FILE"

# A statusLine command must print the status line — emit a minimal one; replace
# with your own UX.
printf '5h %s%%\n' "${FIVE_H:-?}"
