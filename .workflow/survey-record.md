# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.









## 2026-08-12 scope — Which kit is the cheapest coherent next native-port cohort, and what shared substrate does each remaining kit still owe?
- corpus: scripts/gates.list native/src canon-kit/lib/spec.sh gate-sdk/lib/gate.sh
- oracle: grep -c '^pub mod' native/src/gates/mod.rs
- rev: 4d6324b039a9d4f3dc3535d560b277ed44a20230
- finding: Verdict at survey time: 10 ported modules; scripts/gates.list carries 101 registered gates, so 91 remain — the queue entry's '2 of 100' is stale. Unported per kit: canon-kit 21, gate-sdk 30, lifecycle-kit 12, context-kit 4, evidence-kit 3, delegation-kit 3, site-kit 2, queue-kit 2 (both operator-held), doctrine-kit 1, root scripts check-* 13. Judgment: lifecycle-kit is the cheapest coherent block — 12 gates, mostly file-only, needing no new shared substrate; only check-close-surfaces, check-lesson-disposition and check-survey-record touch git or jq, and check-shim-restatement and check-stage-entry walk. canon-kit's 11-gate spec_manifest_files block stays blocked on two Rust mechanisms that do not exist yet: a basename-glob matcher beside walk.rs's find_files (fixed extension) and glob_files (full-path glob), and a Rust gate_kit_roots, which a recursive grep over native/src does not find. gate-sdk sequences last: roughly 12 of its 30 audit gate sources, criterion 4's self-referential parity. Criterion-7 blockers confirmed: jq is absent from GATE_SDK_PROGRAM_FLOOR so context-kit's check-memory-off and check-settings-pins are blocked; ruby/kramdown blocks site-kit's check-docs-render-fidelity; shellcheck blocks check-action-run-shell. Two kit-local gate scripts are unregistered in gates.list: canon-kit check-surface-duplication, evidence-kit check-producer-liveness. Per-gate classification below kit level was sampled by grep, not read line by line.
