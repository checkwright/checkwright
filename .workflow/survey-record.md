# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.



## 2026-08-08 scope — Which deferred entries are at or over the recurrence threshold, and which candidates carry the heaviest inbound-citation totals?
- corpus: TASK-QUEUE.md — 142 entries, ~125 of them Deferred
- oracle: grep -n 'recurrence:' TASK-QUEUE.md (8 declarations, threshold 2 = two dates) and bash queue-kit/bin/queue-edges.sh
- rev: bf641b4ff16a78cc53dec6f10bf66e5f8cc5f313
- finding: Exactly one entry is at threshold: gap-resolver-mention-overcount, two dates 2026-08-06 and 2026-08-07 — every other declaration carries a single date. On inbound totals the recurrence cluster dominates the small-surface candidates: recurrence-drain-input-widening 3, gap-resolver-mention-overcount 3, recurrence-resolver-literal-match-only 1, and the three cite each other as one bounded-substring predicate seen from three sides. Among trajectory-sequence candidates installer-lifecycle-verbs has 1 inbound, prose-profile 2, companion-toolkit-profile 2, benchmark-ab-experiment 3.

## 2026-08-08 scope — Does installer-lifecycle-verbs' filed premise still hold, and what does a spec pass inherit unsettled?
- corpus: installer/ — lib/, README.md, and the entry filed 2026-07-26
- oracle: ls installer/lib/ plus the manifest field table and the doctor section of installer/README.md
- rev: 9b4d12fd696e2907eb9129da9748014e0576be79
- finding: HOLDS: checkwright.lock's files field records each written path at the content hash init last wrote there, so uninstall (remove only manifest-recorded files) and diff (hash drift) are both buildable off data that already exists; installer/lib/ holds init.sh and doctor.sh only, so all three verbs are genuinely absent. STALE: the entry predates doctor and never mentions it. doctor already reports installed release, commit, profile and kit set and re-verifies the gate binary digest in place, but nothing per-file — so diff's ground is unowned and the question a spec pass hits immediately is whether diff is a fourth verb or a widening of doctor. installer/README.md's doctor section closes with the seam that argues they are the same shape: doctor writes nothing so it has no --dry-run, and every verb that does write has one. Not settled here — spec's call.

## 2026-08-08 spec — Which init-written paths reach the manifest's files[] roster, and which manifest fields does an existing reader tolerate as absent?
- corpus: installer/lib/init.sh, doctor.sh, common/lock.sh — every record() call site and every lock_field/lock_own_file reader
- oracle: grep -n 'record "' installer/lib/init.sh and grep -rn 'lock_own_file|lock_field|lock_schema_ok' --include=*.sh installer/
- rev: ccade2ff3d3d5e5db1e8055018304279d5ae0f32
- finding: files[] MEMBERS: every vendored kit file, the seeded gates.list, every config seam and msg-patterns.list, the seeded queue/agent/evidence/workflow surfaces, the three generated projections (pre-commit, CHECK-GRAPH.html, commit-msg), and the gate binary at ARTIFACT_PATH — which also has the separate artifact key, so it needs no special case in a roster walk. NOT a member: checkwright.lock itself, because init appends it to WRITTEN only after emitting the manifest, so it never records itself and any removal path must dispose of it explicitly. ABSENCE-TOLERANT FIELDS, verified in source: version (the downgrade test is guarded by [[ -n "$PRIOR_VERSION" && ... ]]), profile (falls through to the starter default at init.sh:86), kits (only narrows lock_own_file's fixture-exclusion predicate, whose sole readers are doctor.sh x2 and run-smoke.sh x2, and both call sites need an artifact key or a gates.list). This is what licenses a narrowed residual manifest carrying schema + files alone.
