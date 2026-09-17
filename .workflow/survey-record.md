# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-09-17 scope — Which deferred entries lead the undirected unit set (cost-class ranking, recurrence threshold, inbound edges)?
- corpus: TASK-QUEUE.md
- oracle: run-gates.sh --emit queue-edges; grep 'cost: (session|iteration)' over the Deferred lead lines
- rev: 04f8b5f162ecc72fd20ee785c38a6c3582cd7fe6
- finding: 132 Deferred; no session-class row; no recurrence at threshold 2; stage-economics-log-redates-rows + stage-economics-meter-has-no-feeding-obligation (drift-kit) lead as one design pass; alternates boundary-wipe-preserve-basename-reach + dod-parks-a-queue-transition-at-a-stage-that-cannot-perform-it (lifecycle-kit), inline-interpreter-substrate-census (guard-kit)
- inferred: impact ordering among the 8 iteration/high rows is judged from body prose, not measured

## 2026-09-17 spec — Does every stage-economics log row derive a stamp date, and how many closes has the log lapsed?
- corpus: .metric/stage-economics-log.txt; .workflow/WORKFLOW-STATE.txt (git log --reverse -p -U0 plus live)
- oracle: awk join: (iteration, stage minus +fanout) latest stamp date, supervision -> iteration latest; closed = iterations with a close stamp in history order, lag = closes after the last one named in log field 2
- rev: 813fde96dab26ece8e13037d2d29eb1da8685f31
- finding: 1695 rows, all derive a stamp date (1473 stage/fanout, 222 supervision), 1672 would move; 177 closed iterations, newest priced queue-arm-report-fidelity, lag 3
- inferred: none — both figures executed

## 2026-09-17 align — Does SPEC-stage-economics-feed.md wire cleanly against the current tree: program claims at their read sites, Existing-sections-updated roster completeness, and the generated-projection fan-out it declares exempt?
- corpus: drift-kit/SPEC-stage-economics-feed.md native/src/emit/stage_economics.rs native/src/emit/kpi/ native/src/emit/drift_report.rs drift-kit/SPEC.md drift-kit/README.md .claude/commands/close.md drift-kit/smoke/install.sh TASK-QUEUE.md docs/site-architecture.md
- oracle: none
- rev: 169ce750674e64d464fd018a6fb708f78581c537
- finding: ~20 concrete program claims (stamp union, in-place write, dedup replace, BUILTINS/Ctx location, kpis.list roster x3, close.md text, smoke assertions, SPEC section text, README paragraph, queue entries, retired-spelling gate, FANOUT_SUFFIX knob, context hook) all CONFIRMED but one: Ctx struct fields are declared in native/src/emit/kpi/mod.rs, not drift_report.rs as the roster stated - fixed. Two more defects found and fixed: the row-date consumer roster omitted kpi-stage-economics-lag as a 4th log reader (it reads field 2 directly for Priced, delta 1 vs delta 4 cross-reference now added); and the exempt generated-projections bullet over-included docs/footprint.md and the generated pre-commit hook, neither of which a KPI addition (vs a gate addition) stales per docs/site-architecture.md SThe KPI-roster fan-out and SThe new-gate fan-out - both removed from the exempt list. Feeding-paragraph placement clarified to close the section after the fan-out row paragraph rather than wedge between trend-log/supervision/fan-out. All fixes landed in the amendment; gate battery green after.
- inferred: none
