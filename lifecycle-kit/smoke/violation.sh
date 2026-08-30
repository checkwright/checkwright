#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §Consumer smoke — lifecycle-kit consumer-smoke violation: renames the header's iteration out from under the stamps, reddening check-stage-evidence's name-axis (staleness) assertion
# no-port: gate-sdk/SPEC.md §Consumer smoke, The port disposition — legs 2 and 3 of the class ruling of 2026-08-30. Leg 2: this is an executable recipe by stated contract — the harness runs the file against the scratch tree and reads the expected gate name off its first stdout line — so a crate form crosses harder the recipe-into-derivation boundary §Consumer smoke declined to cross for its sibling install.sh. Leg 3: it vendors to an adopter with its kit but is executed by no adopter path — the SMOKE_KIT_ROOT entry-point guard refuses a bare invocation and the only callers in existence are this repo own validate suites — so it costs an adopter no interpreter dependency. Structural, not a sizing judgment, and stated rather than cited-by-example because the class had no precedent in either direction before that ruling.
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via run-consumer-smoke.sh}"

echo "check-stage-evidence"

sed -i 's/^## Iteration: .*$/## Iteration: renamed-out-of-band/' TASK-QUEUE.md
