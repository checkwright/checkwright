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
check-gate-substrate-parity
check-crate-arms
check-install-disposition
check-graph
check-commit-msg
check-tree-terms
check-assertion-strength
check-commit-subject
check-core-files
check-exec-bit
check-hook-exec-bit
check-identity
check-kit-enum
check-readme-roster
check-reads-couples
check-smoke-entry-guard
check-template-copy-parity
check-template-registry-parity
check-test-hermetic
check-workflow-tiering
EOF

# smoke-unregistered: check-root-tiering — its subject is the consumer-curated root manifest GATE_SDK_ROOT_ALLOWLIST (default scripts/root-allowlist.list), which no kit install can author: the vendored root set is per-adoption and gate-sdk installs first, before that set exists
# spec: gate-sdk/SPEC.md §Consumer smoke — check-gate-binary-fresh is deliberately NOT registered here and deliberately carries no declaration: its subject is the crate the binary was built from, GATE_SDK_NATIVE_CRATE is kept outside every kit root by design, so the probe derives the exemption every run and a written reason would be the inversion the accounting refuses

# spec: gate-sdk/SPEC.md §Consumer smoke — ship the tracked default pattern list; the local companion is absent, exercising the fresh-clone path
cp "$SDK/templates/msg-patterns.list" scripts/msg-patterns.list

bash "$SDK/bin/gen-pre-commit.sh" --write >/dev/null
bash "$SDK/checks/check-graph.sh" --emit > scripts/CHECK-GRAPH.html

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

# spec: gate-sdk/SPEC.md §port-blockers — the tokenizer rules, exercised behaviourally because
# the tool is a bin/ tool and owed no fixture pair: a scan that truncates at a here-string, or
# steals a case-pattern close inside a substitution, reports the trailing requirement as absent
pb="$(mktemp -d)"
printf '%s\n' 'check-smoke-tokenizer' > "$pb/gates.list"
cat > "$pb/check-smoke-tokenizer.sh" <<'EOF'
#!/usr/bin/env bash
# graph: couples=scripts/gates.list dir=one valve=none tier=precommit
set -uo pipefail
if [[ -n "$(pb_alpha_prog)" ]]; then
    pb_bravo_prog
fi
v="$( case "$1" in a) pb_charlie_prog ;; *) pb_delta_prog ;; esac )"
read -r w <<<"$v"
for f in ./*.sh; do pb_tail_prog "$f" "$w"; done
fail_closed 0 SMOKE-TOKENIZER probe
EOF
pb_run() { GATE_SDK_GATES_DIR="$pb" bash "$SDK/bin/port-blockers.sh" "$@"; }

out="$(pb_run)" || { echo "smoke(port-blockers): default arm exited non-zero" >&2; exit 1; }
for prog in pb_bravo_prog pb_charlie_prog pb_delta_prog pb_tail_prog; do
    grep -q "	$prog	" <<<"$out" || {
        echo "smoke(port-blockers): scan lost $prog — the tokenizer truncated: $out" >&2; exit 1; }
done
if grep -q 'pb_alpha_prog' <<<"$out"; then
    echo "smoke(port-blockers): reported a word inside [[ ]], which is not command position: $out" >&2; exit 1
fi

out="$(pb_run --group)" || { echo "smoke(port-blockers): --group exited non-zero" >&2; exit 1; }
grep -q '1 group(s) formed, 0 undecidable' <<<"$out" || {
    echo "smoke(port-blockers): --group did not key a fully-scannable member: $out" >&2; exit 1; }
grep -q 'libs=fail_closed globs=\*.sh' <<<"$out" || {
    echo "smoke(port-blockers): --group key lost a factor: $out" >&2; exit 1; }

if pb_run --nope >/dev/null 2>&1; then
    echo "smoke(port-blockers): an unrecognized argument was not refused" >&2; exit 1
fi
pb_run --help >/dev/null || { echo "smoke(port-blockers): --help did not exit 0" >&2; exit 1; }
rm -rf "$pb"
