# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.








































## 2026-08-26 scope — Which deferred entries have reached LIFECYCLE_KIT_RECURRENCE_THRESHOLD (2), and does the stage template's prescribed anchored-grep oracle find all of them?
- corpus: TASK-QUEUE.md's Deferred section, lines 17-8297, 304 entries
- oracle: grep -n 'recurrence:' TASK-QUEUE.md, UNANCHORED, then count ISO dates per hit and confirm the line number is below the Icebox header
- rev: 4b35bc295be2d12e8c1bb3501f20459646ecab92
- finding: SEVEN entries are at threshold: relayed-ruling-provenance-unrecorded, dead-queue-citation-report, kfric-empty-log-ambiguity, absorbed-duplicate-disposition, validate-baseline-suite-coverage, pack-installer-vendors-untracked-scratch, isolated-child-liveness-hook-displaces-its-report. 26 recurrence declarations exist in all. CRITICAL SECOND FINDING: the anchored grep the stage template prescribes would have MISSED the seventh. TASK-QUEUE.md:7934 reads 'recurrence: 2026-08-25 2026-08-26' with NO slug, violating queue-kit/SPEC.md's grammar, and no gate validates it — the only reader, native/src/gates/queue_entry_budget.rs is_recurrence(), is a budget-discount heuristic that accepts the slugless form. Use an UNANCHORED grep until a gate exists.
