#!/usr/bin/env bash
# spec: context-kit/SPEC.md §Testing — consumer-smoke violation: over-budget pointered bullet reddens check-brevity
# no-port: gate-sdk/SPEC.md §Consumer smoke, The port disposition — legs 2 and 3 of the class ruling of 2026-08-30, which reaches this file by its GROUND rather than by its scope: that ruling stated-contract cut covers the recipes answering to §Consumer smoke and this one answers to context-kit/SPEC.md §Testing, but both legs hold of it identically. Leg 2: an executable recipe by stated contract — the harness runs it against the scratch tree and reads the expected gate name off its first stdout line. Leg 3: it vendors with the kit but is executed by no adopter path — the SMOKE_KIT_ROOT entry-point guard refuses a bare invocation and the only callers are this repo own validate suites. Structural, not a sizing judgment.
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
