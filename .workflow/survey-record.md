# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.











































## 2026-08-27 scope — What cohesive cuts does the 141-file owed port remainder decompose into, now that the registry composer provably does not reach it?
- corpus: every tracked non-test .sh port-blockers.sh --tree walks (154 files, 12 no-port, 1 held, 141 owed)
- oracle: bash gate-sdk/bin/port-blockers.sh --tree, piped through an awk decomposition on path segment 2
- rev: d3361aa6e20cf2d27f20b10b0908431f21421ca4
- finding: By second path segment the owed 141 are: bin 41, lib 27, smoke 21, templates 20, kpis 13, and 24 loose scripts/ members. By kit: drift-kit 21, scripts 16, delegation-kit 16, context-kit 16, gate-sdk 14, lifecycle-kit 11, installer 11, guard-kit 10, queue-kit 7, evidence-kit 6, doctrine-kit 5, canon-kit 4, site-kit 3, demo 1. TWO CUTS ARE COHESIVE AND THE REST ARE NOT. (1) drift-kit/kpis/ is 13 files and 558 lines behind ONE stated contract, drift-kit/SPEC.md section The KPI plugin contract, which specifies a plugin as an executable kpi-<name>.sh resolved through the registry and invoked directly rather than through an interpreter word. A native KPI is a binary subcommand, not a file, so the cut is not mechanical: the plugin-discovery contract is what the port amends, and every one of the 13 lands behind that one amendment. Largest single-surface family in the corpus, and drift-kit is the largest owed kit precisely because of it. (2) Eight two-line */templates/*-config.sh files are counted owed while their INSTANTIATED forms under scripts/ are declared no-port on the 2026-08-24 provenance-seam config-and-vocabulary ruling, visible at scripts/lifecycle-config.sh line 3 and its siblings. Same content class, opposite disposition, purely because the tree arm walks kit templates too. That is a declaration cohort rather than a port cohort and costs eight comment lines. NOT COHESIVE: bin at 41 spans every kit and shares no contract; smoke at 21 is the test harness the 2026-08-23 tail ruling explicitly excludes from the completion claim; lib at 27 is per-kit internals with no common seam. THE READING THIS SUPPORTS: native-gate-port-remaining-corpus is blocked on a composer, not on work, and a cohort-by-stated-contract composer is derivable from this corpus while a size-ordered or kit-ordered one is not. QUALIFIER PROBED AT THE SAME SESSION AND IT BINDS CUT (1)'s PRIORITY, NOT ITS COHESION: installer/profiles.list carries drift-kit in NO shipping profile — the three rosters are starter (gate-sdk), delegation (eight kits, drift-kit absent) and prose (gate-sdk, canon-kit) — so the 13 KPI plugins ship to no adopter today and fall under the 2026-08-23 tail ruling's contributor-side clause, which ports opportunistically and never as a gate on the claim. They are still counted owed by --tree, because the predicate is every tracked non-test .sh. So the cohort is the largest and cleanest cut available AND is not on the adopter-facing critical path, and a composer ruling has to say which of those two facts it is selecting on. That is the choice, stated rather than resolved here. RULED 2026-08-27, lead, own-authority: THE COMPOSER QUESTION IS LEFT OPEN and the port yields this boundary, on this survey's own reading that both cohesive cuts advance the owed count while neither advances the completion claim. So a later dispatch inherits a decomposition and not a composer. What it should do first is re-run the witness above and diff the corpus at this rev; if both hold, the two cuts and the three non-cuts stand as recorded and only the selection rule is owed. The entry that carries the composer is native-gate-port-remaining-corpus, which now points here by name rather than restating any of this.

## 2026-08-27 scope — Which live deferred entries does the rest of the queue converge on, by inbound citation sum, and does any candidate's weight live in the total rather than in its own body?
- corpus: TASK-QUEUE.md ## Deferred + ## Icebox, every inbound citation resolved live-or-retired
- oracle: bash queue-kit/bin/queue-edges.sh
- rev: d3361aa6e20cf2d27f20b10b0908431f21421ca4
- finding: TOP LIVE TARGETS BY INBOUND SUM: platform-support-ci-matrix 10, powershell-installer-surface 7, prose-filename-citation-liveness 6, overlay-only-oracle-grants-uncommitted 4, then a long tail at 3 (unqualified-section-citation-liveness, session-model-identity-verification, scratch-citation-skill-surface-reach, retired-slug-live-pointer-citation, queue-entry-grammar-single-owner, native-gate-port-remaining-corpus, guard-steer-grant-mismatch, dispatch-claim-evidentiary-tier-unmarked, build-stage-tier-economics, benchmark-ab-experiment) and roughly thirty at 2. THE ONE PLACE THE SUM CHANGES A RANKING: six of the tail-and-mid entries are one family and no single one of them reads as iteration-sized — prose-filename-citation-liveness 6, unqualified-section-citation-liveness 3, retired-slug-live-pointer-citation 3, spec-pointer-self-section-citation 2, stale-identifier-after-retirement 2, qualified-pointer-section-ownership 2, summing to 18 inbound edges across a family whose members are individually small. citation-liveness-family-convergence is the entry that already took that sum and it is where the dividend is legible; read alone, every member of it ranks low. That is the aggregation failure this pass exists to catch, and it is present in this corpus. RETIRED-TARGET READ, which is an input and not a footnote: the two heaviest retired targets are battery-runner-port (14 inbound) and check-spec-pointer (14 inbound), both shipped, so an entry arguing from either is arguing from disposed work; dead-queue-citation-report (2 inbound, retired) is the convergence entry's own already-disclosed stale member and both of its citations correctly self-label it retired; waiter-predicate-self-match (1 inbound, retired) is cited by waiter-loop-condition-predicate-gap and by a carried gap bullet that asserts it is LIVE, which is false at this rev — the work landed at 05af5200. NO CANDIDATE WAS REFUSED ON A SUM this pass; the sum promoted one bundle and falsified one carried premise.

## 2026-08-27 align — Does installer/SPEC-cross-version-reversal.md's every tree/prose claim (line citations, row counts, config channels, queue state) hold against the live tree, and is it internally self-consistent
- corpus: the amendment file plus every file/line it cites: installer/consumer-smoke/run-smoke.sh, installer/README.md sect The consumer smoke, scripts/parse-installer-smoke-log.sh, scripts/evidence-config.sh, native/src/gates/evidence_baseline.rs, .workflow/validate-baseline.txt, TASK-QUEUE.md, evidence-kit/SPEC.md sect Baseline manifest
- oracle: manual line-by-line citation verification (grep + Read against each cited line number/variable/config key) plus a self-consistency re-read of the amendment's own framing sentences against what its deltas actually mandate
- rev: 2babeccc0ea76b68bd7400169e8eaf4ad661230e
- finding: 27+ citations verified accurate. 3 defects found and fixed in place, all sharing one wrong premise (9 existing installer_smoke scenarios instead of the actual 12, confirmed both at HEAD and at the amendment's own authoring commit a35e2a06): the 'gains a tenth row' claim (x2, should be thirteenth), and 'the nine existing arm scenarios' claim (should be twelve). Also fixed an unsupported 'and one moves' framing claim in delta 2 that neither the delta's own prose nor the Existing-sections-updated roster backs — neither paragraph relocates, one is edited in place and one stays untouched. The two limits the amendment states explicitly (pack-from-one-worktree, baseline-row-starts-ignore-until-binary-less-dispatch-loop-retirement-lands) both still hold and were left alone.
