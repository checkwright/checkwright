#!/usr/bin/env bash
# Fixture gate (scanned as text, never executed): every exemption element
# carries a valid disposition — must be ACCEPTED.
set -uo pipefail

# exception-list: surfaces excused from the sample scan
EXEMPT=(
    "surface-a"   # until: fix-sample-surface
    "surface-b"   # permanent: generated file, never hand-edited
)
echo "SAMPLE: clean (${#EXEMPT[@]} exemptions)"
