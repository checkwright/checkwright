#!/usr/bin/env bash
# Fixture gate (scanned as text, never executed): a NAKED awk capture with no
# status check — a crash would false-green as clean. check-gate-fail-closed must
# REJECT it.
set -uo pipefail
out="$(awk '{print}' "$FILE")"
[[ -n "$out" ]] && exit 1
exit 0
