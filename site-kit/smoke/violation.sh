#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §Consumer smoke — site-kit consumer-smoke violation: configures an alias, then cites it in a ://URL, reddening check-docs-cname-parity
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via run-consumer-smoke.sh}"

echo "check-docs-cname-parity"

cat > scripts/site-config.sh <<'EOF'
# shellcheck shell=bash
# shellcheck disable=SC2034  # sourced by site-kit/lib/site.sh
SITE_KIT_ALIASES=(alt.example)
EOF

cat > NOTES.md <<'EOF'
# Notes

The old site lived at https://alt.example/ — a configured alias other than the
apex, which cname-parity must reject.
EOF
git add NOTES.md
