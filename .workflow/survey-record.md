# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.














































## 2026-08-30 scope — Which stated contract owns the most owed files, i.e. what does the ruled stated-contract composer select as the next port cut?
- corpus: :(glob)**/*.sh
- oracle: bash gate-sdk/bin/run-gates.sh --emit port-blockers --tree
- rev: 18408035884daa77544dabc3e669680f41a108a9
- edges: none
- finding: Oracle at this rev: 139 scanned, 15 no-port, 1 held, 123 owed, 14214 owed lines. Every owed file declares its owning section on its own "# spec:" line, so the composer input is a one-command derivation over the oracle owed list (xargs grep -m1 on the spec line, grouped by the text before the em dash). Ranked by owed FILES: gate-sdk/SPEC.md §Consumer smoke 17 files/1572 lines; delegation-kit/SPEC.md §Testing 6/573; context-kit/SPEC.md §Testing 5/364; context-kit/SPEC.md §Index-first reading 5/285; evidence-kit/SPEC.md §Layout and configuration 4; delegation-kit/SPEC.md §The delegation model 4. Ranked by owed LINES the single-file contracts lead: guard-kit/SPEC.md §The guard framework 1/1243; installer/README.md §The consumer smoke 1/851; gate-sdk/SPEC.md §lib/gate.sh 1/709; lifecycle-kit/SPEC.md §bin/enter-stage.sh 1/617. FINDING: §Consumer smoke is the largest stated contract on BOTH axes, 14 percent of the owed file count in one section, and it is exactly the class kit-smoke-port-disposition-cohort says has zero precedent and needs one class ruling rather than a per-cut argument. SUB-FINDING: the smoke class is not one contract. Seventeen smoke-related files answer to gate-sdk §Consumer smoke while the context-kit trio and the delegation-kit pair answer to their own §Testing sections, so the cohort entry framing of all eleven kits shipping a smoke/install.sh crosses three contracts and a cut must be scoped against that. SECOND FINDING, independent of the composer: seven kit config templates (canon, context, delegation, evidence, guard, lifecycle, queue) are counted owed while drift-kit/templates/drift-config.sh carries a no-port declaration whose cause is structural - a kit config template IS the adopter config seam, so porting it deletes the seam - and each of those seven consumer copies under scripts/ is itself declared no-port. The operator-ruled completion predicate is therefore inflated by about seven files for a class already ruled structural, and no gate holds that declaration in lockstep across the class.

## 2026-08-30 scope — Which deferred entries does the rest of the queue converge on, and which of the port-track candidates has had its blocker discharged since it was last read?
- corpus: TASK-QUEUE.md
- oracle: bash queue-kit/bin/queue-edges.sh
- rev: 5dbadb97d33550188d0cb222e0b0ab5afc915188
- edges: platform-support-ci-matrix 9, prose-filename-citation-liveness 6, native-gate-port-remaining-corpus 6, powershell-installer-surface 5, waiter-loop-condition-predicate-gap 3, wait-loop-exemption-blind-behind-a-script-name 3, unqualified-section-citation-liveness 3, threshold-recurrence-routing-residency 3, session-model-identity-verification 3, scratch-citation-skill-surface-reach 3, retired-slug-live-pointer-citation 3, queue-entry-grammar-single-owner 3 (icebox), overlay-only-oracle-grants-uncommitted 3, guard-steer-grant-mismatch 3, guard-rule-number-not-citable-outside-kit 3, dispatch-claim-evidentiary-tier-unmarked 3, companion-toolkit-profile 3, build-stage-tier-economics 3, benchmark-ab-experiment 3, batch-split-stamp-ownership 3. Caveat: citation-liveness-family-convergence has ZERO inbound while being the corpus most prolific citer, so an inbound-only ranking misses rollup entries entirely.
- finding: Counts: Deferred 276, Icebox 49, both active sections empty. THREE OF THE TOP FOUR MOST-CITED LIVE SLUGS ARE PORT-TRACK, which is independent confirmation of where the queue mass sits. Retired-slug hubs still cited from live entries: check-spec-pointer 14 (shipped as a native gate), battery-runner-port 13 (Done), shell-gate-tail-port 12 (Done), guard-grant-review 5, scratch-execution-control-is-bash-only 5 (shipped as guard rule 23) - so several live entries argue from premises already settled. PRINCIPAL FINDING, re-verified first-hand rather than relayed: platform-support-ci-matrix, the single most-cited live entry at 9 inbound, is UNBLOCKED. Its one named remaining blocker (blocker 6, the MSYS path dialect, symptom: 10 of 10 gates FAILED every one unresolved) was routed out to msys-path-dialect-boundary-unmodelled, which owns the resolver fix and the contract while this entry owns only the observation. That owning entry is retired - shipped last iteration at 48cff8d3, whose message states the repair in native/src/walk.rs reproduced the observed all-gates-unresolved string character-for-character. The slug survives only as four citations, none a live bullet. SECOND FINDING, which changes the price the entry itself records: the install-smoke-windows job in .github/workflows/gates.yml runs unconditionally on every push to master under continue-on-error true, so the next observation rides the iteration ordinary close push at ZERO incremental push cost. The entry cost line saying each further round costs one push was written while rounds were being iterated within an iteration; one observation off the close push costs nothing. THIRD FINDING: the entry cannot drain in the iteration whose push produces its observation, which is the shape observation-predicate-entry-cannot-drain-in-its-own-iteration owns; check-stage-entry assertion B models exactly this residue as a drain-exempt lead-line tag.
