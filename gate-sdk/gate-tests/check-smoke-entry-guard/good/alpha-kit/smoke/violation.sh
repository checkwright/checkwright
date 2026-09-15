#!/usr/bin/env bash
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via run-gates.sh --run-consumer-smoke}"
echo "check-alpha-one"
