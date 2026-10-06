#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §Consumer smoke — site-kit consumer-smoke install (README.md §Install)
# cwd = scratch-consumer root; SMOKE_KIT_ROOT = the vendored site-kit copy.
# no-port: gate-sdk/SPEC.md §Consumer smoke, The port disposition — legs 2 and 3.
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via --run-consumer-smoke on the gate binary}"
SDK="$SMOKE_KIT_ROOT/../gate-sdk"   # the vendored gate-sdk beside this kit

cat >> scripts/gates.list <<'EOF'
# site-kit
check-docs-cname-parity
check-docs-highlight-coverage
check-docs-render-fidelity
check-docs-collapsible
check-docs-liquid-parse
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

bash "$SDK/bin/run-gates.sh" --emit graph > scripts/CHECK-GRAPH.html
