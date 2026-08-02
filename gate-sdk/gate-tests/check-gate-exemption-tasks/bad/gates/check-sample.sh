#!/usr/bin/env bash
# Fixture gate (scanned as text, never executed): one exemption's `# until:`
# resolves only to Done, one carries no disposition, and one points at a bold
# token in body prose that was never a task — all three must be REJECTED.
set -uo pipefail

# exception-list: surfaces excused from the sample scan
EXEMPT=(
    "surface-a"   # until: retired-task
    "surface-b"
    "surface-c"   # until: emphasis
)
echo "SAMPLE: clean (${#EXEMPT[@]} exemptions)"
