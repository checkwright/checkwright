#!/usr/bin/env bash
# spec: context-kit/SPEC.md §Testing — consumer-smoke violation: over-budget pointered bullet reddens check-brevity
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via run-consumer-smoke.sh}"

echo "check-brevity"

# spec: context-kit/SPEC.md §Testing — insert inside the budgeted section, never at EOF (a co-vendored trailing section would push an EOF bullet out of scope)
section="${CONTEXT_KIT_BREVITY_SECTION:-## Shared conventions}"
awk -v sec="$section" '
    { print }
    !ins && substr($0, 1, length(sec)) == sec {
        print ""
        print "- **Bloated:** this bullet runs on for well past the four-line"
        print "  budget across several continuation lines, and it openly cites"
        print "  a deeper doc at HANDBOOK §Somewhere, so the brevity gate must"
        print "  flag it as over budget while admitting that its detail already"
        print "  lives in that referenced section rather than here."
        ins = 1
    }
' CLAUDE.md > CLAUDE.md.tmp && mv CLAUDE.md.tmp CLAUDE.md
