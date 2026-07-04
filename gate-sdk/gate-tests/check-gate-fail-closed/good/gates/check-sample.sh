#!/usr/bin/env bash
# Fixture gate (scanned as text, never executed): its awk capture handles the
# subprocess status, so check-gate-fail-closed must ACCEPT it.
set -uo pipefail
out="$(awk '{print}' "$FILE")"; st=$?
fail_closed "$st" check-sample awk
[[ -n "$out" ]] && exit 1
exit 0
