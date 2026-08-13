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

## 2026-08-13 scope — Which set is the next native port cohort under the largest-shared-derivation rule?
- corpus: scripts/gates.list */checks */lib gate-sdk/SPEC.md TASK-QUEUE.md
- oracle: bash gate-sdk/bin/port-blockers.sh
- rev: 8c20b2e0233c9472893ce7f657f1028ed0963248
- finding: lifecycle-kit taken as a near-whole kit is the largest criteria-clearing set sharing one corpus derivation now available: 11 of its 12 still-shell members source lifecycle-kit/lib/stages.sh (only check-close-surfaces does not, and its derivation over .workflow plus kit SPEC declarations is separate — size it apart). Every one of the 12 manifests reads dir=one valve=none tier=precommit, so criterion 3 clears kit-wide exactly as it did for queue-kit; no member appears in the blocker report, so criterion 7 clears; no couples= glob covers a gate declaration path, so criterion 4 clears; all carry good/+bad/ fixture dirs. ONE MEMBER IS HELD: check-stage-entry reads LIFECYCLE_KIT_PREDECESSOR by key (a declare -A at scripts/lifecycle-config.sh:22, read at check-stage-entry.sh:46 and 67-68), which gate-sdk/SPEC.md's bridge section rules unportable until the wire format grows keys. Net 10 clean. Rival groups sized and rejected: evidence-kit shares lib/evidence.sh at 3 members but check-evidence-baseline reads EVIDENCE_KIT_SCENARIO_GLOBS by key (the same declare -A gap, second instance), netting 2; the gate-sdk leak-guard pair check-tree-terms plus check-commit-msg shares gate_msg_pattern_files byte-for-byte and is the first cohort's exact shape, but check-tree-terms greps content over the whole git ls-files tree including gate declarations and native/src/*.rs, which reads as a criterion-4 failure no surface records — assertion C does not select it, since its couples= is scripts/msg-patterns.list, so the two facts are independent as the spec says; delegation-kit's check-gate-tamper carries the not-in-tree live-corpus hazard that excluded check-memory-off from the first cohort. check-spec-derivable-section (canon-kit) rides along at zero marginal derivation cost: both its primitives, spec_manifest_files and the ERE matcher, are already compiled. The generated-projection freshness family is confirmed NOT a cohort: each of the six emitters maps 1:1 to one gate, so no blocker-retiring exception applies.
