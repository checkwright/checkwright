#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §Consumer smoke — canon-kit consumer-smoke install (README.md §Install)
# cwd = scratch-consumer root; SMOKE_KIT_ROOT = the vendored canon-kit copy.
# no-port: gate-sdk/SPEC.md §Consumer smoke, The port disposition — legs 2 and 3 of the class ruling of 2026-08-30. Leg 2: this is an executable install recipe by stated contract, and check-install-disposition assertion B reads its body as text, so a crate table crosses harder the recipe-into-derivation boundary §Consumer smoke already declined to cross, and ADDS violations rather than removing them. Leg 3: it vendors to an adopter with its kit but is executed by no adopter path — the SMOKE_KIT_ROOT entry-point guard refuses a bare invocation and the only callers in existence are this repo own validate suites — so it costs an adopter no interpreter dependency. Structural, not a sizing judgment, and stated rather than cited-by-example because the class had no precedent in either direction before that ruling.
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via run-consumer-smoke.sh}"
SDK="$SMOKE_KIT_ROOT/../gate-sdk"   # the vendored gate-sdk beside this kit

cat >> scripts/gates.list <<'EOF'
# canon-kit (check-surface-duplication omitted — needs a glossary)
check-amendment-queue
check-amendment-update-target
check-spec-dod-singleton
check-spec-derivable-section
check-spec-embedded-source
check-comment-tier
check-deprecation-task
check-docs-cmd
check-docs-link-convention
check-install-claim
check-knob-citation
check-knob-default-coupling
check-manifest-count
check-manifest-temporal
check-md-refs
check-measured-claim
check-payload-claim
check-prose-enum
check-prose-tells
check-spec-fence-balance
check-spec-pointer
check-todo-task-liveness
check-tracking-claim
check-unmarked-claim
EOF

# smoke-unregistered: check-surface-duplication — the glossary topology it reads (CANON_KIT_GLOSSARY_FILE, default GLOSSARY.md) is optional and this tree ships none, so its exit 2 is uncorroborated only because the invoking repo lacks the same optional surface, not because the gate is broken

# spec: gate-sdk/SPEC.md §Consumer smoke — seed check-amendment-queue's surface (guarded; carries lifecycle-kit's inert header so the seed composes with the stage gates)
if [[ ! -f TASK-QUEUE.md ]]; then
    cat > TASK-QUEUE.md <<'EOF'
# TASK-QUEUE.md — smoke consumer work queue

## Iteration: —

---

## New Features

## Technical Debt

## Deferred

## Done
EOF
fi

bash "$SDK/bin/gen-pre-commit.sh" --write >/dev/null
bash "$SDK/bin/run-gates.sh" --emit graph > scripts/CHECK-GRAPH.html
