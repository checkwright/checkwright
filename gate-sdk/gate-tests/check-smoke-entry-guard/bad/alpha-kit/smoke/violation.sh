#!/usr/bin/env bash
set -euo pipefail
echo "check-alpha-one"
cat > scripts/check-smoke-dirty.sh <<'EOF'
#!/usr/bin/env bash
echo "DIRTY"
EOF
