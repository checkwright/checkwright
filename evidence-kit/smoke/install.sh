#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §Consumer smoke — evidence-kit consumer-smoke install (README.md §Install)
# cwd = scratch-consumer root; SMOKE_KIT_ROOT = the vendored evidence-kit copy.
# no-port: gate-sdk/SPEC.md §Consumer smoke, The port disposition — legs 2 and 3 of the class ruling of 2026-08-30. Leg 2: this is an executable install recipe by stated contract, and check-install-disposition assertion B reads its body as text, so a crate table crosses harder the recipe-into-derivation boundary §Consumer smoke already declined to cross, and ADDS violations rather than removing them. Leg 3: it vendors to an adopter with its kit but is executed by no adopter path — the SMOKE_KIT_ROOT entry-point guard refuses a bare invocation and the only callers in existence are this repo own validate suites — so it costs an adopter no interpreter dependency. Structural, not a sizing judgment, and stated rather than cited-by-example because the class had no precedent in either direction before that ruling.
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via run-consumer-smoke.sh}"
SDK="$SMOKE_KIT_ROOT/../gate-sdk"   # the vendored gate-sdk beside this kit

cat >> scripts/gates.list <<'EOF'
# evidence-kit
check-evidence-baseline
check-evidence-manifest
EOF

# smoke-unregistered: check-producer-liveness — entry-preflight only, never gates.list-registered: its subject is a producer-in-flight transition, not tree state, and a consumer battery that includes itself (this repo's does) would red every run against the lock run-validate just claimed (evidence-kit/SPEC.md §check-producer-liveness)

mkdir -p .workflow
[[ -f .workflow/validate-baseline.txt ]] \
    || printf '# contract: evidence-kit/SPEC.md §Baseline manifest — held-constant validate baseline: <suite> <scenario> <status> [<slug>]\n' > .workflow/validate-baseline.txt
[[ -f .workflow/validate-evidence.txt ]] \
    || printf '# contract: evidence-manifest v1\n' > .workflow/validate-evidence.txt

bash "$SDK/bin/gen-pre-commit.sh" --write >/dev/null
bash "$SDK/bin/run-gates.sh" --emit graph > scripts/CHECK-GRAPH.html

# spec: evidence-kit/SPEC.md §bin/run-validate.sh — exercise the validate spine end-to-end (advisory tool, no fixture pair): a one-suite exit-code run appends a clean evidence line.
es="$PWD/.tmp/run-validate-smoke"
rm -rf "$es"; mkdir -p "$es/.workflow" "$es/scripts" "$es/.tmp"
printf '# baseline\ngreen green pass\nuntouched_tree untouched_tree pass\nmulti a pass\nmulti b pass\n' > "$es/.workflow/validate-baseline.txt"
printf '# contract: evidence-manifest v1\n' > "$es/.workflow/validate-evidence.txt"
printf '#!/usr/bin/env bash\nprintf "a pass\\nb pass\\n"\n' > "$es/scripts/multi-parser.sh"
# spec: evidence-kit/SPEC.md §Evidence manifest — a suite standing in for one whose precondition is a clean worktree: it reds if the manifest already carries a data line at its turn, which is what an earlier suite's row means. Second in the roster on purpose — first, it would pass under either writer
printf '#!/usr/bin/env bash\ngrep -qEv "^[[:space:]]*(#|$)" .workflow/validate-evidence.txt \\\n    && { echo "the spine wrote the tracked manifest before this suite ran" >&2; exit 1; }\nexit 0\n' \
    > "$es/scripts/untouched-tree.sh"
cat > "$es/scripts/evidence-config.sh" <<'EOF'
EVIDENCE_KIT_SUITES=(green untouched_tree multi)
EVIDENCE_KIT_PARSER=exit-code
EVIDENCE_KIT_RUN_ID=smoke
EVIDENCE_KIT_RUN_green='true'
EVIDENCE_KIT_RUN_untouched_tree='bash scripts/untouched-tree.sh'
EVIDENCE_KIT_RUN_multi='true'
EVIDENCE_KIT_PARSER_multi='bash scripts/multi-parser.sh'
EOF
# spec: evidence-kit/SPEC.md §bin/run-validate.sh — the spine is the bridged `--run-validate` arm, so this scratch tree becomes its own git toplevel (the front end refuses outside a repository and resolves every relative knob against the toplevel it lands on) and the binary this consumer was given crosses absolute, its repo-relative default naming nothing inside the new toplevel.
( cd "$es" && git init -q . ) >/dev/null 2>&1
spine_bin="$PWD/$( ( source "$SDK/lib/gate.sh"; gate_native_bin ) )"
( cd "$es" && GATE_SDK_GATES_DIR=scripts GATE_SDK_NATIVE_BIN="$spine_bin" \
    bash "$SDK/bin/run-gates.sh" --run-validate >/dev/null )
grep -qE '^smoke green sha256=[0-9a-f]{64} pass=1 fail=0 ignore=0 verdict=clean ' \
    "$es/.workflow/validate-evidence.txt" \
    || { echo "smoke(run-validate): clean evidence line not appended" >&2; exit 1; }
# spec: evidence-kit/SPEC.md §Layout and configuration — the per-suite parser override reaches the spine: 'multi' counts its two scenarios while its sibling stays on the global exit-code adapter (one scenario, asserted above)
grep -qE '^smoke multi sha256=[0-9a-f]{64} pass=2 fail=0 ignore=0 verdict=clean ' \
    "$es/.workflow/validate-evidence.txt" \
    || { echo "smoke(run-validate): per-suite parser override did not reach the spine" >&2; exit 1; }
# spec: evidence-kit/SPEC.md §Evidence manifest — the single fold, asserted from the suite's own vantage rather than from the finished file: a spine writing per suite reddens this one, so its clean row is the property
grep -qE '^smoke untouched_tree sha256=[0-9a-f]{64} pass=1 fail=0 ignore=0 verdict=clean ' \
    "$es/.workflow/validate-evidence.txt" \
    || { echo "smoke(run-validate): the spine wrote the manifest before the roster finished" >&2; exit 1; }
# spec: evidence-kit/SPEC.md §Evidence manifest — the rows land in configured-suite order, so a repeat run relocates nothing
diff <(grep -v '^#' "$es/.workflow/validate-evidence.txt" | awk '{print $2}') \
     <(printf 'green\nuntouched_tree\nmulti\n') >/dev/null \
    || { echo "smoke(run-validate): the folded rows are not in configured-suite order" >&2; exit 1; }
rm -rf "$es"
