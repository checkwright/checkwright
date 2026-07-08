# shellcheck shell=bash
# spec: evidence-kit/SPEC.md §Layout and configuration — this repo's evidence-kit consumer config
# shellcheck disable=SC2034  # every knob below is consumed by evidence-kit/lib/evidence.sh after sourcing

EVIDENCE_KIT_SUITES=(
    gates
    gatesdk_fixtures
    lifecycle_fixtures
    queue_fixtures
    spec_fixtures
    delegation_fixtures
    context_fixtures
    evidence_fixtures
    guard_tests
)
EVIDENCE_KIT_PARSER=exit-code

EVIDENCE_KIT_RUN_gates='bash gate-sdk/bin/run-gates.sh'
EVIDENCE_KIT_RUN_gatesdk_fixtures='bash gate-sdk/bin/run-gate-tests.sh gate-sdk/gate-tests gate-sdk/checks'
EVIDENCE_KIT_RUN_lifecycle_fixtures='bash gate-sdk/bin/run-gate-tests.sh lifecycle-kit/gate-tests lifecycle-kit/checks'
EVIDENCE_KIT_RUN_queue_fixtures='bash gate-sdk/bin/run-gate-tests.sh queue-kit/gate-tests queue-kit/checks'
EVIDENCE_KIT_RUN_spec_fixtures='bash gate-sdk/bin/run-gate-tests.sh spec-kit/gate-tests spec-kit/checks'
EVIDENCE_KIT_RUN_delegation_fixtures='bash gate-sdk/bin/run-gate-tests.sh delegation-kit/gate-tests delegation-kit/checks'
EVIDENCE_KIT_RUN_context_fixtures='bash gate-sdk/bin/run-gate-tests.sh context-kit/gate-tests context-kit/checks'
EVIDENCE_KIT_RUN_evidence_fixtures='bash gate-sdk/bin/run-gate-tests.sh evidence-kit/gate-tests evidence-kit/checks'
EVIDENCE_KIT_RUN_guard_tests='bash guard-kit/bin/run-guard-tests.sh'
