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
