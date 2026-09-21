#!/usr/bin/env bash
set -euo pipefail
members=(c a b)
printf '%s\n' "${members[@]}" | sort
