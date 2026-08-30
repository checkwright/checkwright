#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §Consumer smoke — site-kit consumer-smoke install (README.md §Install)
# cwd = scratch-consumer root; SMOKE_KIT_ROOT = the vendored site-kit copy.
# no-port: gate-sdk/SPEC.md §Consumer smoke, The port disposition — legs 2 and 3 of the class ruling of 2026-08-30. Leg 2: this is an executable install recipe by stated contract, and check-install-disposition assertion B reads its body as text, so a crate table crosses harder the recipe-into-derivation boundary §Consumer smoke already declined to cross, and ADDS violations rather than removing them. Leg 3: it vendors to an adopter with its kit but is executed by no adopter path — the SMOKE_KIT_ROOT entry-point guard refuses a bare invocation and the only callers in existence are this repo own validate suites — so it costs an adopter no interpreter dependency. Structural, not a sizing judgment, and stated rather than cited-by-example because the class had no precedent in either direction before that ruling.
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via run-consumer-smoke.sh}"
SDK="$SMOKE_KIT_ROOT/../gate-sdk"   # the vendored gate-sdk beside this kit

cat >> scripts/gates.list <<'EOF'
# site-kit
check-docs-cname-parity
# spec: gate-sdk/SPEC.md §Consumer smoke — gate-sdk gates registered by the kit whose
# install writes their subject: the site-health workflow copied in below is the only
# Actions-shaped surface any install writes, so these three lint installed content
# rather than passing vacuously.
check-action-pinning
check-action-run-shell
check-action-gh-repo
check-action-permissions
check-docs-render-fidelity
EOF

# spec: gate-sdk/SPEC.md §Consumer smoke — the gated source of truth for the
# docs host (default SITE_KIT_CNAME); with SITE_KIT_ALIASES unset the gate holds
# on defaults, the assertion no fixture suite makes.
mkdir -p docs
echo "apex.example" > docs/CNAME

# spec: gate-sdk/SPEC.md §Consumer smoke — install the site-health template
# verbatim as governed surface, so a template regression against any vendored
# kit's gate reddens the battery (starter-template conformance).
mkdir -p .github/workflows
cp "$SMOKE_KIT_ROOT/templates/site-health.yml" .github/workflows/site-health.yml

bash "$SDK/bin/gen-pre-commit.sh" --write >/dev/null
bash "$SDK/bin/run-gates.sh" --emit graph > scripts/CHECK-GRAPH.html
