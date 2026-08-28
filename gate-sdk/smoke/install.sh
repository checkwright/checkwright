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
bash "$SDK/bin/run-gates.sh" --emit graph > scripts/CHECK-GRAPH.html

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

# spec: gate-sdk/SPEC.md §run-gates — --only selects by name off argv and refuses an unregistered
# one; the registry still holds the failing stub, so a leg that goes green proves the selection
# narrowed rather than that the battery happened to pass
only_run() { env -u GATE_SDK_VERBOSE GATE_SDK_GATES_DIR="$q" GATE_SDK_TMP_DIR="$q/tmp" bash "$SDK/bin/run-gates.sh" "$@"; }

out="$(only_run --only check-smoke-pass)" || { echo "smoke(--only): selecting the passing member of a red registry went red" >&2; exit 1; }
grep -q 'All 1 gates passed' <<<"$out" || { echo "smoke(--only): summary N is not the selected count" >&2; exit 1; }
if grep -q 'SMOKE-FAIL: 1 stub finding' <<<"$out"; then echo "smoke(--only): ran a member that was not selected" >&2; exit 1; fi

if out="$(only_run --only check-smoke-absent 2>&1)"; then echo "smoke(--only): an unregistered name did not refuse" >&2; exit 1; fi
grep -q "check-smoke-absent" <<<"$out" || { echo "smoke(--only): refusal did not name the name" >&2; exit 1; }
grep -q "$q/gates.list" <<<"$out" || { echo "smoke(--only): refusal did not name the registry" >&2; exit 1; }
rm -rf "$q"

# spec: gate-sdk/SPEC.md §The bin/-tool contract — behavioural coverage of a bin/ tool's help and
# refusal, on the precedent that rule names: no gate reads it, so these two legs are what keeps
# `--help` from falling back through to the gates-dir positional
out="$(bash "$SDK/bin/run-gates.sh" --help 2>/dev/null)" || { echo "smoke(--help): a help request did not exit 0" >&2; exit 1; }
grep -q '^usage: run-gates.sh' <<<"$out" || { echo "smoke(--help): usage did not reach stdout" >&2; exit 1; }

if err="$(bash "$SDK/bin/run-gates.sh" --smoke-nope 2>&1 >/dev/null)"; then echo "smoke(refusal): an unrecognized option was not refused" >&2; exit 1; fi
grep -q 'unrecognized option: --smoke-nope' <<<"$err" || { echo "smoke(refusal): refusal did not name the option" >&2; exit 1; }
grep -q '^usage: run-gates.sh' <<<"$err" || { echo "smoke(refusal): usage did not reach stderr" >&2; exit 1; }

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

# spec: gate-sdk/SPEC.md §port-blockers — the tokenizer rules, exercised behaviourally because the
# arm owes no fixture pair and no in-crate test reaches the front-end path: a scan that truncates at
# a here-string, or steals a case-pattern close inside a substitution, loses the trailing requirement
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
# spec: gate-sdk/SPEC.md §The non-gate arm — the arm is reached through the one front-end that
# resolves a bridged arm's environment; --gates-dir is what scopes the arm's own declared-knob
# union, so the invocation asserted here is the one a caller actually makes
pb_run() { GATE_SDK_GATES_DIR="$pb" bash "$SDK/bin/run-gates.sh" --emit port-blockers --gates-dir "$pb" "$@"; }

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
# spec: gate-sdk/SPEC.md §port-blockers — the count is asserted exactly, against the planted declaration's own length rather than a transcribed number, because a lines=[0-9]+ shape match would pass a field counting the wrong file, which is the failure the column's whole value rests on
pb_decl_lines=$(( $(wc -l < "$pb/check-smoke-tokenizer.sh") ))
grep -Eq "check-smoke-tokenizer +lines=$pb_decl_lines +c2=" <<<"$out" || {
    echo "smoke(port-blockers): --group lost lines=$pb_decl_lines on the member row, or moved it out of the fixed-width run: $out" >&2; exit 1; }

if pb_run --nope >/dev/null 2>&1; then
    echo "smoke(port-blockers): an unrecognized argument was not refused" >&2; exit 1
fi
# spec: gate-sdk/SPEC.md §The bin/-tool contract — help before arity: -h/--help as the *first*
# argument prints usage at exit 0 whatever follows it, which the arm adopts rather than repairing.
# First is load-bearing: a --gates-dir ahead of it is an unrecognized argument, not a help request.
pb_help() { bash "$SDK/bin/run-gates.sh" --emit port-blockers "$@"; }
pb_help --help >/dev/null || { echo "smoke(port-blockers): --help did not exit 0" >&2; exit 1; }
pb_help --help --group >/dev/null || {
    echo "smoke(port-blockers): --help stopped winning over what follows it" >&2; exit 1; }
if pb_help --gates-dir "$pb" --help >/dev/null 2>&1; then
    echo "smoke(port-blockers): help was honoured from a position the contract does not give it" >&2
    exit 1
fi
# spec: gate-sdk/SPEC.md §port-blockers — the missing-registry refusal, structurally absent from
# every in-crate test: a registry arm handed a directory with no gates.list refuses rather than
# reporting an empty battery
mkdir -p "$pb/empty"
if err="$(GATE_SDK_GATES_DIR="$pb" bash "$SDK/bin/run-gates.sh" --emit port-blockers \
    --gates-dir "$pb/empty" 2>&1 >/dev/null)"; then
    echo "smoke(port-blockers): a missing registry did not refuse" >&2; exit 1
fi
grep -q 'registry not found' <<<"$err" || {
    echo "smoke(port-blockers): missing-registry refusal did not name its cause: $err" >&2; exit 1; }
rm -rf "$pb"

# spec: gate-sdk/SPEC.md §port-blockers — the tree arm's corpus rules and its three dispositions,
# exercised behaviourally on the terms §The bin/-tool contract states; asserted as *deltas* against a
# baseline run rather than as absolute counts, because the surrounding consumer tree's own shell
# corpus is not this assertion's subject and pinning it would break on every unrelated addition
# spec: gate-sdk/SPEC.md §port-blockers — the four counts are lifted by matching the trailer's whole
# grammar, so a trailer that changed shape fails here rather than silently yielding a wrong field
pbt_counts() {
    tail -1 | sed -nE 's/^port-blockers --tree: ([0-9]+) file\(s\) scanned, ([0-9]+) declared no-port, ([0-9]+) temporarily held, ([0-9]+) owed$/\1 \2 \3 \4/p'
}
pbt_tree() { bash "$SDK/bin/run-gates.sh" --emit port-blockers --tree; }
pbt_before="$(pbt_tree | pbt_counts)"
[[ -n "$pbt_before" ]] || { echo "smoke(port-blockers): --tree trailer did not match its specified grammar" >&2; exit 1; }
read -r pbt_n0 pbt_p0 pbt_h0 pbt_o0 <<<"$pbt_before"
mkdir -p pbtree/gate-tests
cat > pbtree/plain.sh <<'EOF'
#!/usr/bin/env bash
echo plain
EOF
cat > pbtree/permanent.sh <<'EOF'
#!/usr/bin/env bash
# no-port: the adoption bootstrap runs before any binary exists
echo permanent
EOF
cat > pbtree/held.sh <<'EOF'
#!/usr/bin/env bash
# port-until: check-smoke-blocker
echo held
EOF
# spec: gate-sdk/SPEC.md §The `# graph:` manifest — the three ill-formed declarations, each of
# which must leave its file owed: a bare hold naming no work, a cause with no payload, and both
# fields at once contradicting the pair's mutual exclusion
cat > pbtree/bare.sh <<'EOF'
#!/usr/bin/env bash
# port-until:
echo bare
EOF
cat > pbtree/empty-cause.sh <<'EOF'
#!/usr/bin/env bash
# no-port:
echo empty
EOF
cat > pbtree/both.sh <<'EOF'
#!/usr/bin/env bash
# no-port: a permanent cause
# port-until: check-smoke-blocker
echo both
EOF
cp pbtree/permanent.sh pbtree/excluded.test.sh
cp pbtree/permanent.sh pbtree/gate-tests/fixture.sh
git add pbtree
pbt="$(pbt_tree)"
read -r pbt_n1 pbt_p1 pbt_h1 pbt_o1 <<<"$(pbt_counts <<<"$pbt")"

for row in "pbtree/plain.sh	owed" "pbtree/permanent.sh	no-port" \
    "pbtree/held.sh	port-until:check-smoke-blocker" "pbtree/bare.sh	owed" \
    "pbtree/empty-cause.sh	owed" "pbtree/both.sh	owed"; do
    grep -q "^$row	lines=" <<<"$pbt" || {
        echo "smoke(port-blockers): --tree missed the row '$row': $pbt" >&2; exit 1; }
done
for excluded in pbtree/excluded.test.sh pbtree/gate-tests/fixture.sh; do
    if grep -q "^$excluded	" <<<"$pbt"; then
        echo "smoke(port-blockers): --tree scanned $excluded, which its corpus rules exclude" >&2; exit 1
    fi
done
# spec: gate-sdk/SPEC.md §port-blockers — the counts are asserted as exact deltas, so a trailer that
# merely moved would not pass: six files enter the corpus, one declares no-port, one is held, four
# are owed, and the two excluded files must move nothing at all
[[ $((pbt_n1 - pbt_n0)) -eq 6 && $((pbt_p1 - pbt_p0)) -eq 1 &&
    $((pbt_h1 - pbt_h0)) -eq 1 && $((pbt_o1 - pbt_o0)) -eq 4 ]] || {
    echo "smoke(port-blockers): --tree trailer deltas wrong — scanned +$((pbt_n1 - pbt_n0)) (want 6), no-port +$((pbt_p1 - pbt_p0)) (want 1), held +$((pbt_h1 - pbt_h0)) (want 1), owed +$((pbt_o1 - pbt_o0)) (want 4)" >&2
    exit 1
}
# spec: gate-sdk/SPEC.md §port-blockers — lines= is asserted against the planted file's own length
# rather than a shape match, on the ground the --group arm's own count assertion above records
pbt_held_lines=$(($(wc -l < pbtree/held.sh)))
grep -q "^pbtree/held.sh	port-until:check-smoke-blocker	lines=$pbt_held_lines$" <<<"$pbt" || {
    echo "smoke(port-blockers): --tree row lost lines=$pbt_held_lines: $pbt" >&2; exit 1; }
git rm -rqf pbtree

# spec: gate-sdk/SPEC.md §port-blockers — the non-repository refusal, the one place this arm
# diverges from the shared corpus rule it is built on: that rule degrades to an empty corpus, and a
# silently empty one would print the completion predicate where the arm must refuse
# spec: gate-sdk/SPEC.md §run-gates — reached by invoking the binary rather than the front-end, and
# that is the finding rather than a shortcut: run-gates.sh refuses a non-repository before it execs,
# so the front-end cannot reach this branch and a leg run through it would pass on the wrong refusal
pbt_bin="$( source "$SDK/lib/gate.sh" >/dev/null 2>&1; gate_native_bin )"
case "$pbt_bin" in /*) ;; *) pbt_bin="$PWD/$pbt_bin" ;; esac
nogit="$(mktemp -d)"
if out="$(cd "$nogit" && "$pbt_bin" --emit-port-blockers --tree 2>&1)"; then
    echo "smoke(port-blockers): --tree did not refuse outside a repository: $out" >&2
    rm -rf "$nogit"; exit 1
fi
grep -q 'not a git repository' <<<"$out" || {
    echo "smoke(port-blockers): non-repository refusal did not name its cause: $out" >&2
    rm -rf "$nogit"; exit 1; }
if grep -q ' owed' <<<"$out"; then
    echo "smoke(port-blockers): --tree printed a count over an empty corpus: $out" >&2
    rm -rf "$nogit"; exit 1
fi
rm -rf "$nogit"
