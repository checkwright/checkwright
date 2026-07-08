#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §Consumer smoke — evidence-kit consumer-smoke install (README.md §Install)
# cwd = scratch-consumer root; SMOKE_KIT_ROOT = the vendored evidence-kit copy.
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via run-consumer-smoke.sh}"
SDK="$SMOKE_KIT_ROOT/../gate-sdk"   # the vendored gate-sdk beside this kit

cat >> scripts/gates.list <<'EOF'
# evidence-kit
check-evidence-baseline
check-evidence-manifest
EOF

mkdir -p .workflow
[[ -f .workflow/validate-baseline.txt ]] \
    || printf '# held-constant validate baseline: <suite> <scenario> <status> [<slug>]\n' > .workflow/validate-baseline.txt
[[ -f .workflow/validate-evidence.txt ]] \
    || printf '# contract: evidence-manifest v1\n' > .workflow/validate-evidence.txt

bash "$SDK/bin/gen-pre-commit.sh" --write >/dev/null
bash "$SDK/checks/check-graph.sh" --emit > .workflow/CHECK-GRAPH.html

# spec: evidence-kit/SPEC.md §bin/run-validate.sh — exercise run-validate end-to-end (advisory tool, no fixture pair): a one-suite exit-code run appends a clean evidence line.
es="$PWD/.tmp/run-validate-smoke"
rm -rf "$es"; mkdir -p "$es/.workflow" "$es/scripts" "$es/.tmp"
printf '# baseline\ngreen green pass\n' > "$es/.workflow/validate-baseline.txt"
printf '# contract: evidence-manifest v1\n' > "$es/.workflow/validate-evidence.txt"
cat > "$es/scripts/evidence-config.sh" <<'EOF'
EVIDENCE_KIT_SUITES=(green)
EVIDENCE_KIT_PARSER=exit-code
EVIDENCE_KIT_RUN_ID=smoke
EVIDENCE_KIT_RUN_green='true'
EOF
( cd "$es" && GATE_SDK_GATES_DIR=scripts bash "$SMOKE_KIT_ROOT/bin/run-validate.sh" >/dev/null )
grep -qE '^smoke green sha256=[0-9a-f]{64} pass=1 fail=0 ignore=0 verdict=clean ' \
    "$es/.workflow/validate-evidence.txt" \
    || { echo "smoke(run-validate): clean evidence line not appended" >&2; exit 1; }
rm -rf "$es"
