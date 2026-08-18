#!/usr/bin/env bash
# Fixture gate (scanned as text, never executed): a held-port declaration whose
# slug is live, and deliberately carrying no exemption array — the header-field
# arm must reach it anyway — must be ACCEPTED.
# graph: couples=docs/*.md dir=one valve=none tier=precommit
# port-until: fix-sample-surface
set -uo pipefail
echo "HELD: clean (stub)"
