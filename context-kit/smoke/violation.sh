#!/usr/bin/env bash
# spec: context-kit/SPEC.md §Testing — context-kit consumer-smoke violation:
# appends an over-budget bullet that cites a deeper doc to CLAUDE.md §"## Shared
# conventions" (the last section, so it lands inside), reddening check-brevity.
# First stdout line names the expected FAIL gate; git reset --hard restores the tree.
set -euo pipefail

echo "check-brevity"

cat >> CLAUDE.md <<'EOF'
- **Bloated:** this bullet runs on for well past the four-line
  budget across several continuation lines, and it openly cites
  a deeper doc at HANDBOOK §Somewhere, so the brevity gate must
  flag it as over budget while admitting that its detail already
  lives in that referenced section rather than here.
EOF
