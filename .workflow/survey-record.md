# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.












































## 2026-08-28 scope — Does the 2026-08-27 port decomposition still hold at this boundary, and does its adopter-facing qualifier survive a probe?
- corpus: every tracked non-test .sh port-blockers.sh --tree walks (154 files), plus installer/profiles.list, installer/lib/common/profile.sh, scripts/pack-installer.sh, gate-sdk/lib/gate.sh
- oracle: bash gate-sdk/bin/port-blockers.sh --tree and bash gate-sdk/bin/port-blockers.sh --group, plus a read of the profile resolver
- rev: 62195fcb169d571b9c3c7664e4ed0a94937738c5
- finding: WITNESS HOLDS, ZERO DELTA: --tree reports 154 scanned, 12 no-port, 1 held, 141 owed, identical to rev d3361aa6, and an independent re-derivation matched the recorded by-kit split on all 14 kits. --group reports 107 scanned, 0 groups, 0 owed, 0 takeable, confirming the composer does not reach the tree remainder. TWO CORRECTIONS to the recorded block. (1) Its segment buckets sum to 146 not 141: scripts/ owed is 16 and the loose remainder is 19; the by-kit line is exact. Roles: bin 41 (5501 lines), lib 27 (5206), smoke 21 (2099), templates 20 (648), drift-kit/kpis 13 (558), scripts 16 (828), loose 3 (996). (2) Cut 2's premise that the instantiated scripts/ configs are declared no-port holds for only 6 of 10; scripts/context-config.sh, scripts/delegation-config.sh, scripts/drift-config.sh and scripts/evidence-config.sh carry no declaration and are owed, so cut 2 is 8 kit templates plus 4 undeclared scripts configs = 12 files, and the kit-template half is not covered by the 2026-08-24 provenance-seam cause at all because kit templates ride the payload while that cause is private vocabulary in a non-packed directory. THE QUALIFIER IS FALSIFIED. The recorded block says installer/profiles.list carries drift-kit in NO shipping profile and names three rosters. That reads the hand-authored rows alone and misses the fourth, derived profile. installer/lib/common/profile.sh line 9 sets PROFILE_DERIVED=full, lines 31 to 39 emit full from profile_names alongside every row profile, lines 41 to 46 resolve profile_kits full to profile_payload_kits, and lines 11 to 19 define that as every directory under payload/. scripts/pack-installer.sh lines 124 to 132 fill payload/ from gate_kit_roots_rel, and gate-sdk/lib/gate.sh lines 519 to 531 make any sibling carrying checks/ or smoke/ a kit root; drift-kit/smoke/ exists. installer/lib/init.sh lines 31 and 81 to 82 print and validate full as selectable, and docs/install.md line 274 tells adopters full is everything. So checkwright init --profile full vendors drift-kit and its 13 KPI plugins ARE adopter-facing. CONSEQUENCE: the contributor-side carve-out set shrinks from 46 files to the 33-file test and verification harness; the KPI cut moves onto the completion claim's critical path rather than off it; and the composer fork the 2026-08-27 lead ruling was left open on collapses, because largest-and-cleanest and adopter-facing now select the SAME cut. The claim was asserted independently twice, by that survey and by a fresh sweep this boundary, both by reading rows only, which is why it is recorded here with the resolver line numbers rather than as a conclusion.

## 2026-08-28 scope — Which live deferred entries does the rest of the queue converge on by inbound citation sum at this boundary, and what does the retired-target block say?
- corpus: TASK-QUEUE.md ## Deferred plus ## Icebox, every inbound citation resolved live-or-retired
- oracle: bash queue-kit/bin/queue-edges.sh
- rev: 62195fcb169d571b9c3c7664e4ed0a94937738c5
- finding: TOP LIVE TARGETS BY INBOUND SUM: platform-support-ci-matrix 10, prose-filename-citation-liveness 6, powershell-installer-surface 6, overlay-only-oracle-grants-uncommitted 4, then a band at 3 (unqualified-section-citation-liveness, session-model-identity-verification, scratch-citation-skill-surface-reach, retired-slug-live-pointer-citation, queue-entry-grammar-single-owner, native-gate-port-remaining-corpus, guard-steer-grant-mismatch, dispatch-claim-evidentiary-tier-unmarked, build-stage-tier-economics, benchmark-ab-experiment) and roughly forty at 2. powershell-installer-surface fell from 7 to 6 since the 2026-08-27 run, the retired gate-binary-target-roster-widening accounting for it. WHERE THE SUM CHANGES A RANKING, unchanged from the last run and still the only place: the citation-liveness family sums to about 18 inbound across members that individually rank low, and citation-liveness-family-convergence is where that dividend is already taken and legible. RETIRED-TARGET READ, an input and not a footnote: the heaviest retired targets are battery-runner-port 14 and check-spec-pointer 14, both shipped, so an entry arguing from either argues from disposed work; gap-resolver-mention-overcount 4, artifact-digest-mismatch-remedy-inert 4 and delegation-provenance-floor 4 follow. NO CANDIDATE WAS REFUSED ON A SUM this pass. The pass was owed and performed: this boundary is undirected, so the open question survey-edge-aggregation-residue carries, whether the pass is owed per ranked candidate when an operator supplies the unit set, does not fire here.
