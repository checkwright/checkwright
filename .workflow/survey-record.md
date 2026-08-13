# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.












## 2026-08-13 scope — Which deferred entries do the rest of the queue converge on, and which reach the recurrence threshold?
- corpus: TASK-QUEUE.md
- oracle: bash queue-kit/bin/queue-edges.sh
- rev: f13beb1f4d0cef713a35e0705857ac33afc808f8
- finding: Two clusters carry the aggregate weight, and neither is visible from any single entry. GUARD/PROMPT-FRICTION on one shared surface (scripts/bash-guard.sh + guard-kit/lib/guard.sh): exit-echo-decoration-guard-vs-habit (3 inbound; recurrence dates 2026-08-06 2026-08-13, the only entry in the queue at threshold 2), guard-glyph-match-context-blind (recurrence 2026-08-13, widened that day past prose to live commands), guard-command-prefix-wrapper (~32 percent of prompting calls, the largest single class), poll-sleep-guard-steer, guard-steer-grant-mismatch, guard-advise-jq-dependency, scan-prompts-truncation-quote-desync. CITATION-LIVENESS: prose-filename-citation-liveness (4 inbound, the highest non-launch count), unqualified-section-citation-liveness (2), link-wrapped-section-citation-liveness, qualified-pointer-section-ownership, spec-pointer-self-section-citation, ruling-record-condition-staleness-probe, md-section-near-miss-match, kit-ref-liveness-stem-token-hole, doctrine-rule-number-citation-liveness; qualified-pointer-section-ownership states the family is one predicate question rather than N fixes. Launch-gated entries score high on inbound but are demand-gated: benchmark-ab-experiment 5, hosted-attestation-service 3, plugin-marketplace 3 — all three are also the standing icebox candidates.

## 2026-08-13 scope — Which registered gates are still shell, and which external programs block them?
- corpus: scripts/gates.list */checks
- oracle: bash gate-sdk/bin/port-blockers.sh
- rev: f13beb1f4d0cef713a35e0705857ac33afc808f8
- finding: 103 registered members scanned; 26 carry a .gate descriptor and 77 are still shell, confirming native-gate-port-remaining-corpus's stated 77-of-103 against the tree rather than inheriting it. Real external-program blockers among the still-shell set are only six members over four programs: shellcheck (check-action-run-shell, check-shellcheck), cargo (check-crate-arms), ruby (check-docs-render-fidelity), jq (check-installer-no-deps, check-memory-off, check-settings-pins). The remaining undecidables are command-position variables the report cannot resolve (check-docs-kit-parity's WRAPPED, check-gate-binary-fresh's BIN) plus check-gate-substrate-parity's awk-internal tokens, which are false positives of the scan rather than program requirements. Separately verified still-shell on disk: all six members the ERE engine unblocked per cohort-held-members-port-prerequisites — check-queue-prose-precondition, check-spec-derivable-section, check-deprecation-task, check-tree-terms, check-commit-msg, check-brevity.
