#!/usr/bin/env bash
# A compliant bespoke test: sources the hermetic bootstrap as its first act.
set -uo pipefail
source "$(dirname "${BASH_SOURCE[0]}")/../../gate-sdk/lib/test-hermetic.sh"
echo ok
