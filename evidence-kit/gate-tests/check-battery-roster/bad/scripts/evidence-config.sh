# shellcheck shell=bash
# spec: evidence-kit/SPEC.md §check-battery-roster — fixture consumer config: the same three suites as good/, so the pair differs only in the runner doc
# shellcheck disable=SC2034  # consumed by evidence-kit/lib/evidence.sh after sourcing
EVIDENCE_KIT_SUITES=(alpha beta gamma)
EVIDENCE_KIT_RUN_alpha='bash bin/run-alpha.sh'
EVIDENCE_KIT_RUN_beta='env FIXTURE_VERBOSE=1 bash bin/run-beta.sh'
EVIDENCE_KIT_RUN_gamma='bash bin/run-gamma.sh --deep'
