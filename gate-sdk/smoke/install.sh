#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §Consumer smoke — gate-sdk consumer-smoke install (README.md §Quick start)
# cwd = scratch-consumer root; SMOKE_KIT_ROOT = the vendored gate-sdk copy.
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via run-consumer-smoke.sh}"
SDK="$SMOKE_KIT_ROOT"   # gate-sdk installs itself; its tools live here

mkdir -p scripts .workflow

cat > scripts/gates.list <<'EOF'
# Consumer-smoke gate registry (gate-sdk meta-gates; kits append below).
check-shellcheck
check-gate-output
check-gate-fail-closed
check-gate-fixture-coverage
check-gate-exemption-tasks
check-gate-assertions
check-graph
check-commit-msg
check-tree-terms
EOF

# spec: gate-sdk/SPEC.md §Consumer smoke — ship the tracked default pattern list; the local companion is absent, exercising the fresh-clone path
cp "$SDK/templates/msg-patterns.list" scripts/msg-patterns.list

bash "$SDK/bin/gen-pre-commit.sh" --write >/dev/null
bash "$SDK/checks/check-graph.sh" --emit > .workflow/CHECK-GRAPH.html

# spec: gate-sdk/SPEC.md §run-gates — quiet green, loud red: green is one summary line, red prints verbatim, GATE_SDK_VERBOSE restores the banner roll
q="$PWD/.tmp/quiet-smoke"
rm -rf "$q"; mkdir -p "$q"
cat > "$q/check-smoke-pass.sh" <<'EOF'
#!/usr/bin/env bash
echo "SMOKE-PASS: clean (stub)"
EOF
cat > "$q/check-smoke-fail.sh" <<'EOF'
#!/usr/bin/env bash
echo "SMOKE-FAIL: 1 stub finding"
exit 1
EOF
chmod +x "$q"/check-smoke-pass.sh "$q"/check-smoke-fail.sh
printf 'check-smoke-pass\n' > "$q/gates.list"

qg_run() { env -u GATE_SDK_VERBOSE GATE_SDK_TMP_DIR="$q/tmp" bash "$SDK/bin/run-gates.sh" "$q"; }

out="$(qg_run)" || { echo "smoke(quiet): stub green battery went red" >&2; exit 1; }
grep -q 'All 1 gates passed' <<<"$out" || { echo "smoke(quiet): green summary line missing" >&2; exit 1; }
if grep -q 'SMOKE-PASS: clean' <<<"$out"; then echo "smoke(quiet): green run printed gate output" >&2; exit 1; fi
if grep -q '===== check-smoke-pass =====' <<<"$out"; then echo "smoke(quiet): green run printed a banner" >&2; exit 1; fi

out="$(GATE_SDK_TMP_DIR="$q/tmp" GATE_SDK_VERBOSE=1 bash "$SDK/bin/run-gates.sh" "$q")" || { echo "smoke(quiet): verbose green battery went red" >&2; exit 1; }
grep -q '===== check-smoke-pass =====' <<<"$out" || { echo "smoke(quiet): verbose run lost the banner" >&2; exit 1; }
grep -q 'SMOKE-PASS: clean' <<<"$out" || { echo "smoke(quiet): verbose run lost the gate output" >&2; exit 1; }

printf 'check-smoke-pass\ncheck-smoke-fail\n' > "$q/gates.list"
if out="$(qg_run)"; then echo "smoke(quiet): red battery exited green" >&2; exit 1; fi
grep -q '===== check-smoke-fail =====' <<<"$out" || { echo "smoke(quiet): red run lost the failing banner" >&2; exit 1; }
grep -q 'SMOKE-FAIL: 1 stub finding' <<<"$out" || { echo "smoke(quiet): red output not verbatim" >&2; exit 1; }
if grep -q 'SMOKE-PASS: clean' <<<"$out"; then echo "smoke(quiet): red run printed the passing gate's output" >&2; exit 1; fi
rm -rf "$q"

# spec: gate-sdk/SPEC.md §gen-pre-commit — the emitted hook's capture wrapper: green is one summary line, a red gate's output reprints verbatim
cat > scripts/smoke-hook-probe.sh <<'EOF'
#!/usr/bin/env bash
set -uo pipefail
echo "probe"
EOF
git add scripts/smoke-hook-probe.sh
out="$(env -u GATE_SDK_VERBOSE bash scripts/git-hooks/pre-commit)" || { echo "smoke(hook): green hook run failed" >&2; exit 1; }
grep -qE 'pre-commit: [0-9]+ gate\(s\) passed\.' <<<"$out" || { echo "smoke(hook): green hook summary missing" >&2; exit 1; }
if grep -q ': clean' <<<"$out"; then echo "smoke(hook): green hook printed gate output" >&2; exit 1; fi

cat > scripts/smoke-hook-probe.sh <<'EOF'
#!/usr/bin/env bash
set -uo pipefail
unused_var="never read"
echo "probe"
EOF
git add scripts/smoke-hook-probe.sh
if out="$(env -u GATE_SDK_VERBOSE bash scripts/git-hooks/pre-commit 2>&1)"; then echo "smoke(hook): red hook run passed" >&2; exit 1; fi
grep -q 'pre-commit: check-shellcheck failed' <<<"$out" || { echo "smoke(hook): red hook did not name the gate" >&2; exit 1; }
grep -q 'unused_var' <<<"$out" || { echo "smoke(hook): red hook output not verbatim" >&2; exit 1; }
git reset -q -- scripts/smoke-hook-probe.sh
rm scripts/smoke-hook-probe.sh
