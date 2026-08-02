# shellcheck shell=bash
# spec: evidence-kit/SPEC.md §check-battery-roster — fixture consumer config: three suites, one env-prefixed, so the pair exercises the invocation normalization the parity compare runs on
# shellcheck disable=SC2034  # consumed by evidence-kit/lib/evidence.sh after sourcing
EVIDENCE_KIT_SUITES=(alpha beta gamma)
EVIDENCE_KIT_RUN_alpha='bash bin/run-alpha.sh'
EVIDENCE_KIT_RUN_beta='env FIXTURE_VERBOSE=1 bash bin/run-beta.sh'
EVIDENCE_KIT_RUN_gamma='bash bin/run-gamma.sh --deep'
