# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.



## 2026-08-08 scope — Which deferred entries are at or over the recurrence threshold, and which candidates carry the heaviest inbound-citation totals?
- corpus: TASK-QUEUE.md — 142 entries, ~125 of them Deferred
- oracle: grep -n 'recurrence:' TASK-QUEUE.md (8 declarations, threshold 2 = two dates) and bash queue-kit/bin/queue-edges.sh
- rev: bf641b4ff16a78cc53dec6f10bf66e5f8cc5f313
- finding: Exactly one entry is at threshold: gap-resolver-mention-overcount, two dates 2026-08-06 and 2026-08-07 — every other declaration carries a single date. On inbound totals the recurrence cluster dominates the small-surface candidates: recurrence-drain-input-widening 3, gap-resolver-mention-overcount 3, recurrence-resolver-literal-match-only 1, and the three cite each other as one bounded-substring predicate seen from three sides. Among trajectory-sequence candidates installer-lifecycle-verbs has 1 inbound, prose-profile 2, companion-toolkit-profile 2, benchmark-ab-experiment 3.
