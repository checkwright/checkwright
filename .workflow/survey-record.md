# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.




















## 2026-08-16 scope — Which port increment does gate-sdk/SPEC.md §The first cohort's precedence rule select next, and what does the remaining corpus look like?
- corpus: scripts/gates.list */checks/ native/src/
- oracle: bash gate-sdk/bin/port-blockers.sh --group
- rev: 720b09bfdfaaaa6c0f5bbd3217e0c65a9a4a2572
- finding: 39 of 104 registered members remain unported, 3 of them permanently shell (check-install-disposition, check-gate-substrate-parity, check-crate-arms), so 36 are still owed. The tail-batching iteration exhausted the large groups: --group forms 37 groups, 35 of them singletons. The two 2-member groups are group 1 (check-close-surfaces + check-queue-prose-precondition, key libs=fail_closed globs=-) and group 2 (check-install-disposition + check-readme-roster), and group 2 is effectively a singleton because its larger member is permanent-shell. So the SIZE arm selects group 1 with two members. The BLOCKER-RETIRING override outranks it and is live: the associative-array bridge is one wire-format change gating two members in two kits — check-stage-entry reads LIFECYCLE_KIT_PREDECESSOR and check-evidence-baseline reads EVIDENCE_KIT_SCENARIO_GLOBS, both declare -A knobs read BY KEY, and the config bridge's wire (GATE_SDK_KNOB_<NAME>=<tab-joined>, gate-sdk/lib/gate.sh) carries no key channel. Both claims probed live at this rev, not taken from the queue entry. The BUDGET arm is unreachable: --group reports takeable groups. Second-order finding: native/src/queue.rs already carries the shared queue parser and queue_entry_budget.rs already computes per-entry extents, so any queue-surface work rides machinery the port has already paid for.

## 2026-08-16 scope — Which deferred entries have reached LIFECYCLE_KIT_RECURRENCE_THRESHOLD, and which of them have already been ruled at that count?
- corpus: TASK-QUEUE.md
- oracle: grep -n 'recurrence:' TASK-QUEUE.md
- rev: 720b09bfdfaaaa6c0f5bbd3217e0c65a9a4a2572
- finding: Threshold is 2 (lifecycle-kit/lib/stages.sh default; scripts/lifecycle-config.sh does not override). 19 recurrence: declarations exist; exactly four carry two dates and therefore reach the threshold. Three were ruled at that same count by the lead on 2026-08-16 and carry their grounds in-entry: stage-stamp-ordering-unenforced (the cheap couple-set widening and the history assertion are one design fork, not two units), waiting-rule-fourth-firing-post-fix (nothing buildable survives the 2026-08-06 operator ruling; its design half turn-end-chokepoint-and-wait-primitive is a settings probe, and the entry itself records that a THIRD threshold recurrence routes to the operator rather than to a third decline), and icebox-worklist-roadmap-blind (the fix lands in queue-kit/bin/queue-index.sh, a 182-line shell bin tool the 2026-08-09 priority directive commits to deleting, so it should ride that tool's port rather than be patched in bash). The fourth, entry-headroom-unexposed, newly reached the threshold at the 2026-08-16 close (commit 2730c20b stamped its second date) and has never been escalated at threshold. Its own text names two closes, and their tiers differ: a --headroom mode on queue-index.sh mints a name (feature) while having check-queue-entry-budget print per-entry headroom in its clean output mints none (debt) and lands native, since that gate is already a .gate. Live probe at this rev: queue-index.sh --extent entry-headroom-unexposed returns 4497 4546, i.e. the entry itself sits at exactly the 50-line cap with zero headroom.
