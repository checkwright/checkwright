#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §Consumer smoke — canon-kit consumer-smoke install (README.md §Install)
# cwd = scratch-consumer root; SMOKE_KIT_ROOT = the vendored canon-kit copy.
# no-port: gate-sdk/SPEC.md §Consumer smoke, The port disposition — legs 2 and 3.
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via --run-consumer-smoke on the gate binary}"
SDK="$SMOKE_KIT_ROOT/../gate-sdk"   # the vendored gate-sdk beside this kit

cat >> scripts/gates.list <<'EOF'
# canon-kit (check-surface-duplication omitted — needs a glossary)
check-amendment-queue
check-amendment-update-target
check-amendment-retired-spelling
check-spec-dod-singleton
check-spec-derivable-section
check-spec-embedded-source
check-citation-link
check-comment-tier
check-deprecation-task
check-docs-cmd
check-docs-link-convention
check-docs-page-length
check-docs-page-repeat
check-docs-restatement-parity
check-fence-command-head
check-fence-paste-unit
check-fence-run
check-install-claim
check-knob-citation
check-knob-default-coupling
check-manifest-count
check-manifest-temporal
check-md-refs
check-md-unwrapped
check-measured-claim
check-payload-claim
check-pendency-contradiction
check-prose-bounds
check-prose-enum
check-prose-tells
check-provenance-seam
check-spec-fence-balance
check-spec-pointer
check-task-label-resolution
check-task-path-claim
check-todo-task-liveness
check-tracking-claim
check-unmarked-claim
# spec: gate-sdk/SPEC.md §check-gate-substrate-parity — assertion I's own declaration grammar,
# a sibling of the `# smoke-unregistered:` line below on a second roster (§The install disposition)
# unregistered: check-surface-duplication — this tree declares no glossary, for the provenance-seam reason, and the member exits 2 without one
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

# spec: gate-sdk/SPEC.md §Consumer smoke — seed check-docs-link-convention's surface (guarded, so it composes with site-kit's docs/ in any order)
if [[ ! -f docs/index.md ]]; then
    mkdir -p docs
    printf '# Smoke consumer\n' > docs/index.md
fi

bash "$SDK/bin/run-gates.sh" --emit git-hooks --write >/dev/null
bash "$SDK/bin/run-gates.sh" --emit graph > scripts/CHECK-GRAPH.html
