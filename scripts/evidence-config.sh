# shellcheck shell=bash
# spec: evidence-kit/SPEC.md §Layout and configuration — this repo's evidence-kit consumer config
# no-port: gate-sdk/SPEC.md §The config-seam port disposition — the class ruling of 2026-08-30, reached by ground rather than by scope: this is the seeded-copy side of evidence-kit's config seam, the file this repo actually edits and the more edited of the two, so porting it deletes the seam outright. Its suite roster and per-suite runner commands are this repo's own test topology — half of it derived from the gate-tests dirs on disk — and name tools no kit owns. The 2026-08-24 vocabulary ruling left it owed deliberately — suite names and runner commands, read as tooling layout — which answers "does this hold private vocabulary?" and never "is this an edit seam?"; that verdict is untouched here and the two grounds are cumulative. Structural, not a sizing judgment.
# shellcheck disable=SC2034  # every knob below is consumed by evidence-kit/lib/evidence.sh after sourcing

EVIDENCE_KIT_SUITES=(gates)
# spec: gate-sdk/SPEC.md §lib/gate.sh — derive the per-kit fixture suites from the gate-tests dirs on disk (gate_fixture_suites, in scope via evidence-kit → gate-sdk), so a new kit's fixtures enrol with no edit here; the suites below the loop have no gate-tests dir and stay hand-listed.
while IFS=$'\t' read -r _suite _tests _checks; do
    EVIDENCE_KIT_SUITES+=("$_suite")
    declare "EVIDENCE_KIT_RUN_$_suite=bash gate-sdk/bin/run-gate-tests.sh $_tests${_checks:+ $_checks}"
done < <(gate_fixture_suites)
unset _suite _tests _checks
EVIDENCE_KIT_SUITES+=(guard_tests demo installer_smoke consumer_smoke upgrade agents_md_smoke index_tests native_crate)

EVIDENCE_KIT_PARSER=exit-code

# spec: evidence-kit/SPEC.md §Layout and configuration — per-gate scenarios for the gates suite; the verbose run is what emits the per-gate tails the parser reads
EVIDENCE_KIT_PARSER_gates='bash gate-sdk/bin/run-gates.sh --emit parse-gates-log'
EVIDENCE_KIT_RUN_gates='env GATE_SDK_VERBOSE=1 bash gate-sdk/bin/run-gates.sh'

# spec: evidence-kit/SPEC.md §Layout and configuration — per-arm scenarios for the installer smoke; the arm roster is derived from the smoke's own headers, and the suite's fail-fast shape is what turns an early abort into a red on every arm behind it rather than a hidden one
EVIDENCE_KIT_PARSER_installer_smoke='bash gate-sdk/bin/run-gates.sh --emit parse-smoke-log installer/consumer-smoke/run-smoke.sh'

EVIDENCE_KIT_RUN_guard_tests='bash guard-kit/bin/run-guard-tests.sh'
EVIDENCE_KIT_RUN_demo='bash demo/run-demo.sh'
EVIDENCE_KIT_RUN_installer_smoke='bash installer/consumer-smoke/run-smoke.sh'
EVIDENCE_KIT_RUN_consumer_smoke='bash gate-sdk/bin/run-consumer-smoke.sh'
EVIDENCE_KIT_RUN_upgrade='bash gate-sdk/bin/run-gates.sh --upgrade-smoke'
EVIDENCE_KIT_RUN_agents_md_smoke='bash context-kit/smoke/agents-md.sh'
EVIDENCE_KIT_RUN_index_tests='bash context-kit/bin/run-index-tests.sh'
EVIDENCE_KIT_RUN_native_crate='cargo test --release --manifest-path native/Cargo.toml'
