#!/usr/bin/env bash
# A leaky bespoke test: no bootstrap source, no hermetic-exempt marker — it
# inherits the invoker's cwd config and can green wrongly.
set -uo pipefail
echo ok
