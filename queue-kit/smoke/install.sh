#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §Consumer smoke — queue-kit consumer-smoke install (README.md §Install)
# cwd = scratch-consumer root; SMOKE_KIT_ROOT = the vendored queue-kit copy.
# no-port: gate-sdk/SPEC.md §Consumer smoke, The port disposition — legs 2 and 3 of the class ruling of 2026-08-30. Leg 2: this is an executable install recipe by stated contract, and check-install-disposition assertion B reads its body as text, so a crate table crosses harder the recipe-into-derivation boundary §Consumer smoke already declined to cross, and ADDS violations rather than removing them. Leg 3: it vendors to an adopter with its kit but is executed by no adopter path — the SMOKE_KIT_ROOT entry-point guard refuses a bare invocation and the only callers in existence are this repo own validate suites — so it costs an adopter no interpreter dependency. Structural, not a sizing judgment, and stated rather than cited-by-example because the class had no precedent in either direction before that ruling.
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via run-consumer-smoke.sh}"
SDK="$SMOKE_KIT_ROOT/../gate-sdk"   # the vendored gate-sdk beside this kit

cat >> scripts/gates.list <<'EOF'
# queue-kit
check-queue-hygiene
check-queue-wrap
check-tag-lead-line
check-task-names
check-task-conservation
check-queue-prose-precondition
check-roadmap-fresh
check-queue-entry-budget
check-queue-sections
check-queue-slug-liveness
EOF

cp "$SMOKE_KIT_ROOT/templates/TASK-QUEUE.md" TASK-QUEUE.md

bash "$SDK/bin/gen-pre-commit.sh" --write >/dev/null
bash "$SDK/bin/run-gates.sh" --emit graph > scripts/CHECK-GRAPH.html
