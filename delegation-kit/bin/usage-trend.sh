#!/usr/bin/env bash
# spec: delegation-kit/SPEC.md §Trend reporter — footprint evolution from the usage-history log
# usage: usage-trend.sh [history-file]   ($1 overrides the configured path, for test injection)
#   exit: 0 report emitted, 2 knob unset or history missing/unreadable (never 1 — it renders no verdict; usage-verdict stays the sole pause authority)
set -uo pipefail

KIT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=../lib/delegation.sh
source "$KIT/lib/delegation.sh"

HISTORY="${1:-$DELEGATION_KIT_USAGE_HISTORY}"
PAUSE_PCT_7D="$DELEGATION_KIT_PAUSE_PCT_7D"

if [[ -z "$HISTORY" ]]; then
  echo "usage-trend: DELEGATION_KIT_USAGE_HISTORY unset — no history to report (enable sampling first)" >&2
  exit 2
fi
if [[ ! -r "$HISTORY" ]]; then
  echo "usage-trend: cannot read history $HISTORY" >&2
  exit 2
fi

# spec: delegation-kit/SPEC.md §Trend reporter — one axis record per sample (5h always, weekly when both seven_day keys ride), sorted into segments downstream
records="$(awk '
  /^[[:space:]]*$/ { next }
  {
    delete kv
    for (i = 1; i <= NF; i++) {
      eq = index($i, "=")
      if (eq > 0) kv[substr($i, 1, eq - 1)] = substr($i, eq + 1)
    }
    acct = ("account" in kv) ? kv["account"] : "-"
    tier = ("tier" in kv) ? kv["tier"] : "-"
    login = ("login_at" in kv) ? kv["login_at"] : 0
    verdict = ("verdict" in kv) ? kv["verdict"] : "-"
    tin = ("tokens_in" in kv) ? kv["tokens_in"] : "-"
    tout = ("tokens_out" in kv) ? kv["tokens_out"] : "-"
    if (("pct" in kv) && ("resets_at" in kv) && ("updated_at" in kv))
      printf "5h\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n", acct, tier, login, kv["resets_at"], kv["updated_at"], kv["pct"], verdict, tin, tout
    if (("pct_7d" in kv) && ("resets_7d" in kv) && ("updated_at" in kv))
      printf "7d\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\t%s\n", acct, tier, login, kv["resets_7d"], kv["updated_at"], kv["pct_7d"], verdict, tin, tout
  }
' "$HISTORY" | LC_ALL=C sort -t"$(printf '\t')" -k2,2 -k1,1 -k3,3 -k4,4n -k5,5n -k6,6n)"

if [[ -z "$records" ]]; then
  echo "usage-trend: no parseable samples in $HISTORY (0 segments)"
  exit 0
fi

printf '%s\n' "$records" | awk -F'\t' -v pause7="$PAUSE_PCT_7D" '
  function med3(a, b, c,   x, y, z, t) {
    x = a; y = b; z = c
    if (x > y) { t = x; x = y; y = t }
    if (y > z) { t = y; y = z; z = t }
    if (x > y) { t = x; x = y; y = t }
    return y
  }
  function flush(   i, j, n, sp, runmax, susp, nsusp, fi, li, hours, rate, first, last, hdr, hh, tdelta, ratefmt) {
    n = cnt
    if (n == 0) return
    for (i = 1; i <= n; i++) {
      if (n == 1 || i == 1 || i == n) sp[i] = p[i]
      else sp[i] = med3(p[i-1], p[i], p[i+1])
    }
    nsusp = 0; runmax = p[1]
    for (i = 1; i <= n; i++) susp[i] = 0
    for (i = 2; i <= n; i++) {
      if (p[i] < runmax) {
        susp[i] = 1
        for (j = 1; j < i; j++) if (p[j] > p[i]) susp[j] = 1
      }
      if (p[i] > runmax) runmax = p[i]
    }
    fi = 0; li = 0
    for (i = 1; i <= n; i++) if (!susp[i]) { if (fi == 0) fi = i; li = i; }
    for (i = 1; i <= n; i++) nsusp += susp[i]
    if (fi == 0) { fi = 1; li = n }   # every sample suspect — fall back to the raw span so the segment still reports
    first = sp[fi]; last = sp[li]
    hours = (u[li] - u[fi]) / 3600.0
    if (hours > 0) { rate = (last - first) / hours; ratefmt = sprintf("%+.2f%%/h", rate) }
    else { rate = 0; ratefmt = "n/a (single reading)" }

    if (seg_acct != last_acct) {
      if (seg_acct == "-") printf "\naccount: (unstamped)\n"
      else printf "\naccount %s\n", seg_acct
      last_acct = seg_acct
    }
    printf "  [%s] reset@%s tier=%s: %.1f%%->%.1f%% over %.2fh, %s, %d sample(s), %d suspect\n",
      seg_axis, seg_reset, seg_tier, first, last, hours, ratefmt, n, nsusp
    if (tin[li] != "-" && tin[fi] != "-") {
      tdelta = tin[li] - tin[fi]
      printf "      tokens: +%d in / +%d out over the segment\n", tdelta, (tout[li] - tout[fi])
    }
    if (seg_axis == "7d") {
      hdr = pause7 - last
      if (rate > 0) { hh = hdr / rate; printf "      weekly headroom: %.1f%% to the %s%% ceiling (~%.1fh at current rate)\n", hdr, pause7, hh }
      else printf "      weekly headroom: %.1f%% to the %s%% ceiling (rate flat/negative — no depletion trend)\n", hdr, pause7
    }
    if (firstpause != "") { printf "      first PAUSE onset at epoch %s\n", firstpause }
    cnt = 0; firstpause = ""
  }
  {
    key = $1 "|" $2 "|" $3 "|" $4 "|" $5
    if (NR == 1) { curkey = key }
    if (key != curkey) { flush(); curkey = key }
    seg_axis = $1; seg_acct = $2; seg_tier = $3; seg_login = $4; seg_reset = $5
    cnt++
    u[cnt] = $6; p[cnt] = $7
    tin[cnt] = $9; tout[cnt] = $10
    if ($8 == "PAUSE" && firstpause == "") firstpause = $6
  }
  BEGIN { last_acct = "\0"; cnt = 0; firstpause = "" }
  END { flush() }
' | {
  segs="$(cat)"
  n_samples="$(printf '%s\n' "$records" | grep -c '')"
  echo "usage-trend: $n_samples axis-record(s) across the 5h/weekly segments"
  printf '%s\n' "$segs"
}
exit 0
