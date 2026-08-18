#!/usr/bin/env bash
# Fixture gate (scanned as text, never executed): the held-port declaration's
# slug resolves only to Done, and the file carries no exemption array, so a
# trigger keyed on the array marker would miss it — must be REJECTED.
# graph: couples=docs/*.md dir=one valve=none tier=precommit
# port-until: retired-task
set -uo pipefail
echo "HELD: clean (stub)"
