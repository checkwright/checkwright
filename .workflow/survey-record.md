# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.














































## 2026-08-30 scope — Which stated contract owns the most owed files, i.e. what does the ruled stated-contract composer select as the next port cut?
- corpus: :(glob)**/*.sh
- oracle: bash gate-sdk/bin/run-gates.sh --emit port-blockers --tree
- rev: 18408035884daa77544dabc3e669680f41a108a9
- edges: none
- finding: Oracle at this rev: 139 scanned, 15 no-port, 1 held, 123 owed, 14214 owed lines. Every owed file declares its owning section on its own "# spec:" line, so the composer input is a one-command derivation over the oracle owed list (xargs grep -m1 on the spec line, grouped by the text before the em dash). Ranked by owed FILES: gate-sdk/SPEC.md §Consumer smoke 17 files/1572 lines; delegation-kit/SPEC.md §Testing 6/573; context-kit/SPEC.md §Testing 5/364; context-kit/SPEC.md §Index-first reading 5/285; evidence-kit/SPEC.md §Layout and configuration 4; delegation-kit/SPEC.md §The delegation model 4. Ranked by owed LINES the single-file contracts lead: guard-kit/SPEC.md §The guard framework 1/1243; installer/README.md §The consumer smoke 1/851; gate-sdk/SPEC.md §lib/gate.sh 1/709; lifecycle-kit/SPEC.md §bin/enter-stage.sh 1/617. FINDING: §Consumer smoke is the largest stated contract on BOTH axes, 14 percent of the owed file count in one section, and it is exactly the class kit-smoke-port-disposition-cohort says has zero precedent and needs one class ruling rather than a per-cut argument. SUB-FINDING: the smoke class is not one contract. Seventeen smoke-related files answer to gate-sdk §Consumer smoke while the context-kit trio and the delegation-kit pair answer to their own §Testing sections, so the cohort entry framing of all eleven kits shipping a smoke/install.sh crosses three contracts and a cut must be scoped against that. SECOND FINDING, independent of the composer: seven kit config templates (canon, context, delegation, evidence, guard, lifecycle, queue) are counted owed while drift-kit/templates/drift-config.sh carries a no-port declaration whose cause is structural - a kit config template IS the adopter config seam, so porting it deletes the seam - and each of those seven consumer copies under scripts/ is itself declared no-port. The operator-ruled completion predicate is therefore inflated by about seven files for a class already ruled structural, and no gate holds that declaration in lockstep across the class.
