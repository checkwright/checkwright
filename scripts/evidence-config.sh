# shellcheck shell=bash
# spec: evidence-kit/SPEC.md §Layout and configuration — this repo's evidence-kit consumer config
# shellcheck disable=SC2034  # every knob below is consumed by evidence-kit/lib/evidence.sh after sourcing

# spec: gate-sdk/SPEC.md §lib/gate.sh — derive the per-kit fixture suites from the gate-tests dirs on disk (gate_fixture_suites, in scope via evidence-kit → gate-sdk), so a new kit's fixtures enrol with no edit here; the suites below the loop have no gate-tests dir and stay hand-listed.
EVIDENCE_KIT_SUITES=(gates)
while IFS=$'\t' read -r _suite _tests _checks; do
    EVIDENCE_KIT_SUITES+=("$_suite")
    declare "EVIDENCE_KIT_RUN_$_suite=bash gate-sdk/bin/run-gate-tests.sh $_tests${_checks:+ $_checks}"
done < <(gate_fixture_suites)
unset _suite _tests _checks
EVIDENCE_KIT_SUITES+=(guard_tests usage_tests budget_guard_tests trend_tests demo consumer_smoke agents_md_smoke index_tests)

EVIDENCE_KIT_PARSER=exit-code

EVIDENCE_KIT_RUN_gates='bash gate-sdk/bin/run-gates.sh'
EVIDENCE_KIT_RUN_guard_tests='bash guard-kit/bin/run-guard-tests.sh'
EVIDENCE_KIT_RUN_usage_tests='bash delegation-kit/bin/run-usage-tests.sh'
EVIDENCE_KIT_RUN_budget_guard_tests='bash delegation-kit/bin/run-budget-guard-tests.sh'
EVIDENCE_KIT_RUN_trend_tests='bash delegation-kit/bin/run-trend-tests.sh'
EVIDENCE_KIT_RUN_demo='bash demo/run-demo.sh'
EVIDENCE_KIT_RUN_consumer_smoke='bash gate-sdk/bin/run-consumer-smoke.sh'
EVIDENCE_KIT_RUN_agents_md_smoke='bash context-kit/smoke/agents-md.sh'
EVIDENCE_KIT_RUN_index_tests='bash context-kit/bin/run-index-tests.sh'
