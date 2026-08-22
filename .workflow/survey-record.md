# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.































## 2026-08-22 scope — Which deferred entries have reached LIFECYCLE_KIT_RECURRENCE_THRESHOLD (2), and what is the port track's takeable tier?
- corpus: TASK-QUEUE.md ## Deferred (all recurrence: declarations) plus the 106-member gate corpus
- oracle: grep -n 'recurrence:' TASK-QUEUE.md ; bash gate-sdk/bin/port-blockers.sh --group
- rev: e739e31a95869eab2a4c1d14e8992e36deefb928
- finding: Seven entries at/above threshold: session-mechanic-grants-uncommitted (4), turn-end-chokepoint-and-wait-primitive (4), entry-cap-displaces-mandated-writes (3), scratch-execution-control-is-bash-only (3, standing operator decline), close-entry-baseline-bootstrap-deadlock (2), delegation-provenance-floor (2), single-gate-run-config-bridge (2). Port: 98 ported, 3 permanently shell, 5 temporarily held, 5 owed, 0 takeable — every owed member is held behind cohort-held-members-port-prerequisites, so the budget arm cannot compose a cut.

## 2026-08-22 align — Which of the five # port-until: declarations already have their hold ground reachable in one hop from their # spec:-pointed section, and does an assertion-H-shaped ground-reachability check already exist anywhere in check-gate-substrate-parity.sh?
- corpus: gate-sdk/checks/check-tree-terms.sh,check-gate-assertions.sh,check-shellcheck.sh,check-action-run-shell.sh,check-gate-substrate-parity.sh,site-kit/checks/check-docs-render-fidelity.sh,gate-sdk/SPEC.md,site-kit/SPEC.md
- oracle: grep -rn '# port-until:' gate-sdk/checks site-kit/checks; per declaration, read its # spec: pointer's section for the literal 'port-until'; bash gate-sdk/bin/port-blockers.sh --group
- rev: 665ff0cb31248c7ee10a379a830ae32faa2562f4
- finding: 5 live declarations: check-tree-terms, check-gate-assertions, check-shellcheck, check-action-run-shell (gate-sdk), check-docs-render-fidelity (site-kit). check-gate-substrate-parity.sh carries assertions A-G (seven) today, no assertion-H-shaped ground-reachability check exists under any name (checked two ways: symbol grep plus check-gate-exemption-tasks/check-comment-tier read). Exactly 2 of 5 fail one-hop reachability today -- check-tree-terms (SPEC.md:9896-9911, no mention) and check-docs-render-fidelity (site-kit/SPEC.md, no mention anywhere) -- the other three (check-shellcheck, check-action-run-shell, check-gate-assertions) already name port-until in their pointed section. port-blockers --group trailer: 5 still owed, 0 takeable at this cut.
