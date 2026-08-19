# shellcheck shell=bash
# spec: evidence-kit/SPEC.md §check-evidence-manifest — the case reaches the rule through the
# three path knobs rather than through the positionals, so the pair is a parity oracle for the
# default-derivation branch the production battery actually takes; the positional arm keeps its
# own coverage in check-evidence-manifest.test.sh.
# shellcheck disable=SC2034  # consumed by evidence-kit/lib/evidence.sh after sourcing
EVIDENCE_KIT_MANIFEST_FILE="validate-evidence.txt"
EVIDENCE_KIT_QUEUE_FILE="TASK-QUEUE.md"
EVIDENCE_KIT_STATE_FILE="WORKFLOW-STATE.txt"
