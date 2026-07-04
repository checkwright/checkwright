#!/usr/bin/env bash
# Fixture gate (scanned as text, never executed): one exemption's `# until:`
# resolves only to Done, and another carries no disposition — must be REJECTED.
set -uo pipefail

# exception-list: surfaces excused from the sample scan
EXEMPT=(
    "surface-a"   # until: retired-task
    "surface-b"
)
echo "SAMPLE: clean (${#EXEMPT[@]} exemptions)"
