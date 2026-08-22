# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.































## 2026-08-22 scope — Which deferred entries have reached LIFECYCLE_KIT_RECURRENCE_THRESHOLD (2), and what is the port track's takeable tier?
- corpus: TASK-QUEUE.md ## Deferred (all recurrence: declarations) plus the 106-member gate corpus
- oracle: grep -n 'recurrence:' TASK-QUEUE.md ; bash gate-sdk/bin/port-blockers.sh --group
- rev: e739e31a95869eab2a4c1d14e8992e36deefb928
- finding: Seven entries at/above threshold: session-mechanic-grants-uncommitted (4), turn-end-chokepoint-and-wait-primitive (4), entry-cap-displaces-mandated-writes (3), scratch-execution-control-is-bash-only (3, standing operator decline), close-entry-baseline-bootstrap-deadlock (2), delegation-provenance-floor (2), single-gate-run-config-bridge (2). Port: 98 ported, 3 permanently shell, 5 temporarily held, 5 owed, 0 takeable — every owed member is held behind cohort-held-members-port-prerequisites, so the budget arm cannot compose a cut.
