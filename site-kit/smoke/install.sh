#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §Consumer smoke — site-kit consumer-smoke install (README.md §Install)
# cwd = scratch-consumer root; SMOKE_KIT_ROOT = the vendored site-kit copy.
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via run-consumer-smoke.sh}"
SDK="$SMOKE_KIT_ROOT/../gate-sdk"   # the vendored gate-sdk beside this kit

cat >> scripts/gates.list <<'EOF'
# site-kit
check-docs-cname-parity
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
bash "$SDK/checks/check-graph.sh" --emit > .workflow/CHECK-GRAPH.html
