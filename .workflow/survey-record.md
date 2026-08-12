# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.










## 2026-08-12 scope — Which remaining shell gates share one corpus derivation, which clear the port-candidate criteria, and how large is the ERE engine's true beneficiary set?
- corpus: */checks/check-*.sh scripts/check-*.sh scripts/gates.list native/src/ */SPEC.md
- oracle: bash gate-sdk/bin/port-blockers.sh
- rev: 767bdcae8adc5912e199d691d3cdba0cf00d4121
- finding: 85 remaining gates partition by shared derivation as: gate-registry 10, gate_kit_roots 10, canon lib/spec.sh 11 (manifest 4 already compiled, comment-surface 4, canonical/amendments 4), git ls-files 8, emitter-freshness 6, queue+workflow-state 5, docs-md 3, docs/posts 3, skills-dir 3, workflows 3, fixed .workflow file 3, singletons 20. RANKED NEXT COHORTS: (1) the gate_kit_roots core of five — check-kit-registration, check-smoke-entry-guard, check-test-hermetic, check-assertion-strength, check-template-registry-parity — owes near-zero new mechanism because walk::kit_roots and kit_roots_rel are already bridged in native/src/walk.rs, and gate-sdk/SPEC.md pre-clears these members against criterion 4 by name, so no hold can be discovered mid-port; (2) emitter-freshness four — check-footprint-fresh, check-trajectory-fresh, check-roadmap-fresh, check-docs-mirror-fresh — cheapest in the tree, no walk or glob or engine, but CONTINGENT on the criterion-7 ruling below, and all six members carry the fixture trap where the EMIT_SRC positional arm steers the assertion off the live emitter; (3) canonical/amendments as a rider, only two members clearing since the other two are align-only and lose the check-graph proof. The gate-registry group of ten is the criterion-4 wall and must not be sequenced. check-docs-render-fidelity needs ruby plus kramdown and is not portable without redefining the rule. ERE ENGINE: the beneficiary set is nine, not the four on record — add check-tree-terms and check-commit-msg, which run grep -EnHf over consumer-authored POSIX EREs with no kit-authored ceiling, plus check-spec-derivable-section and check-deprecation-task, plus check-brevity marginally; check-comment-tier needs NO engine because join_alt bracket-escapes every knob element to a literal. A SECOND shipped-kit-default regex trap exists and no surface names it: the derivable-pointer knob's default is an alternation, so the engine is load-bearing for a consumer overriding nothing. CONTRADICTION, unresolved by the specs: port-blockers.sh reports none of the six freshness gates because bash is on the program floor, while gate-sdk/SPEC.md rules check-roadmap-fresh a criterion-7 hold on exactly that bash-emitter shape. HONEST LIMIT: criterion-4 verdicts are read off the conservation table plus each gate's assertion target and were NOT machine-checked against the runtime derivation; sizing signals are judgement, not measurement.
