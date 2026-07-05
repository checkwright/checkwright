#!/usr/bin/env bash
# gate-sdk consumer-smoke violation — drops a ShellCheck-dirty script into the
# gates dir so the self-lint meta-gate fires. First stdout line names the gate
# the harness expects to see FAIL.
set -euo pipefail

echo "check-shellcheck"

# SC2034 (unused variable) is a -S warning finding — check-shellcheck lints the
# gates dir, so this untracked script trips it; git clean -fd restores the tree.
cat > scripts/check-smoke-dirty.sh <<'EOF'
#!/usr/bin/env bash
set -uo pipefail
unused_var="this variable is never read"
echo "DIRTY: clean"
EOF
