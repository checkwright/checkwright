#!/usr/bin/env bash
# spec: context-kit/SPEC.md §Testing — consumer-smoke violation: over-budget pointered bullet reddens check-brevity
set -euo pipefail

echo "check-brevity"

cat >> CLAUDE.md <<'EOF'
- **Bloated:** this bullet runs on for well past the four-line
  budget across several continuation lines, and it openly cites
  a deeper doc at HANDBOOK §Somewhere, so the brevity gate must
  flag it as over budget while admitting that its detail already
  lives in that referenced section rather than here.
EOF
