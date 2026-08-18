# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.
























## 2026-08-18 scope — Which deferred citation-liveness entries share one owning gate, and which sub-bundle do the entries themselves instruct a scope to cost together?
- corpus: TASK-QUEUE.md
- oracle: bash queue-kit/bin/queue-edges.sh
- rev: 73c4ca6b470eafcb61c724f99e6bdab981aa7815
- finding: Twelve deferred entries form the dangling-citation family; five name check-spec-pointer's prose extractor as the owning surface (prose-filename-citation-liveness, unqualified-section-citation-liveness, link-wrapped-section-citation-liveness, qualified-pointer-section-ownership, spec-pointer-self-section-citation), two name check-md-refs (prose-filename-citation-liveness again, md-refs-tree-link-resolution), two resolve to a listing rather than a gate (dead-queue-citation-report and done-slug-ownership-citation-report, which say taking either costs both), and three own separate mechanisms (doctrine-rule-number-citation-liveness, kit-ref-liveness-stem-token-hole, ruling-record-condition-staleness-probe). None carries a blocked-by tag. The bundle the entries themselves instruct is the first three plus spec-pointer-self-section-citation on one extractor pass: unqualified-section-citation-liveness says a promoting scope should cost them together and may find one predicate covers both, link-wrapped says cost all three together, and spec-pointer-self says to sequence it with them rather than building a second scanner. qualified-pointer-section-ownership shares the gate name and is excluded by its own text: it resolves successfully and is wrong anyway, a comprehension problem whose entry admits an honest not-buildable outcome. stale-identifier-after-retirement names prose-filename-citation-liveness its decisive neighbour, the only subsumption claim in the set. Feature-vs-debt splits inside the bundle: prose-filename-citation-liveness states itself a feature (mints a script name and a gates.list registration) while unqualified-section-citation-liveness states itself debt (an assertion inside check-spec-pointer, minting no name), so the bundle owes an amendment for its shipped-path arm whatever the other members land as. Stated cost the bundle carries: 171 live unqualified citations held by nothing.

## 2026-08-18 scope — Which deferred entries have reached the recurrence threshold, and which unported gate groups are takeable this cut?
- corpus: TASK-QUEUE.md scripts/gates.list */checks/*.gate native/src/gates/*.rs
- oracle: bash gate-sdk/bin/port-blockers.sh --group
- rev: 73c4ca6b470eafcb61c724f99e6bdab981aa7815
- finding: Recurrence: exactly one deferred entry reaches LIFECYCLE_KIT_RECURRENCE_THRESHOLD (2) — stage-stamp-ordering-unenforced, dates 2026-08-07 and 2026-08-16. Thirteen other entries carry a single-date recurrence declaration and are below threshold. Port: 104 members scanned, 71 already ported, 3 permanently shell, 30 groups formed and every one of them a SINGLETON, so the size arm is exhausted at this cut as it was at the last two. The budget arm's policy entry (port-tail-cohort-batching-policy) and the consumer-cohort entry (consumer-gate-port-disposition) have both left the queue through Done, so a hand-composed budget batch is the only gate-side increment available and the citations to those two slugs in TASK-QUEUE.md and TRAJECTORY.md are now dead. Separately rostered and NOT part of that 30: freshness-emitter-port-cohort's three remaining shell emitters, scripts/gen-docs-mirror.sh (127 lines), drift-kit/bin/trajectory.sh (242) and queue-kit/bin/roadmap.sh (76), 445 shell lines whose non-gate-arm design ruling is already merged and whose two Linux-side comparators, check-docs-mirror-fresh and check-trajectory-fresh, are already native. The third comparator, check-roadmap-fresh, is a held shell member on cohort-held-members-port-prerequisites whose hold ground is that it keeps queue_roadmap_entries on one shell adapter, so porting roadmap.sh without it splits an emitter/gate pair across substrates.
