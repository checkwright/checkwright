# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.








































## 2026-08-26 scope — Which deferred entries have reached LIFECYCLE_KIT_RECURRENCE_THRESHOLD (2), and does the stage template's prescribed anchored-grep oracle find all of them?
- corpus: TASK-QUEUE.md's Deferred section, lines 17-8297, 304 entries
- oracle: grep -n 'recurrence:' TASK-QUEUE.md, UNANCHORED, then count ISO dates per hit and confirm the line number is below the Icebox header
- rev: 4b35bc295be2d12e8c1bb3501f20459646ecab92
- finding: SEVEN entries are at threshold: relayed-ruling-provenance-unrecorded, dead-queue-citation-report, kfric-empty-log-ambiguity, absorbed-duplicate-disposition, validate-baseline-suite-coverage, pack-installer-vendors-untracked-scratch, isolated-child-liveness-hook-displaces-its-report. 26 recurrence declarations exist in all. CRITICAL SECOND FINDING: the anchored grep the stage template prescribes would have MISSED the seventh. TASK-QUEUE.md:7934 reads 'recurrence: 2026-08-25 2026-08-26' with NO slug, violating queue-kit/SPEC.md's grammar, and no gate validates it — the only reader, native/src/gates/queue_entry_budget.rs is_recurrence(), is a budget-discount heuristic that accepts the slugless form. Use an UNANCHORED grep until a gate exists.

## 2026-08-26 scope — Does citation-liveness-family-convergence's 2026-08-25 survey still hold, and is it citable at this boundary?
- corpus: TASK-QUEUE.md Deferred + Icebox, scripts/gates.list, native/src/gates/ — the survey's own corpus, re-run as a delta against its rev pin 457148bd
- oracle: git diff --stat 457148bd..HEAD over the three corpus halves, then grep each of the 14 named members for liveness, then read the native/src/gates/mod.rs delta for gate additions or removals
- rev: 8a53b2a64105de92eb79c09ef31c61c0ab09e804
- finding: WITNESS HOLDS — the survey is CITABLE at this boundary, delta-checked rather than assumed. Both corpus halves MOVED since the pin (TASK-QUEUE.md +953 lines, three files under native/src/gates/), so a blind citation would have been out of contract. The delta: all FOURTEEN named members still resolve live (12 Deferred + 2 Icebox); the mod.rs delta is ONE env knob added to check-stage-evidence's roster (LIFECYCLE_KIT_PREFLIGHT_VALVE_FILE) with no gate added or removed; and none of the four gate-touch points (check-spec-pointer, check-queue-slug-liveness, check-docs-cmd, the guard rule-number island) changed. The 953-line queue churn removed no member. So the finding stands as written: 14 members collapse to 4 gate-touch points, size floor 8-10 new assertions plus 2 report outputs, all native-crate. Operator-ruled 2026-08-26 to stay DEFERRED in favour of the Windows CI leg; this record exists so the next boundary inherits a live witness instead of re-buying the sweep, which is what the entry's own cost line says re-buying costs.
