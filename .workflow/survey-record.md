# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.





























## 2026-08-22 scope — Which deferred entries carry a RECORDED operator ruling that is not yet built, and what surface do they write?
- corpus: TASK-QUEUE.md ## Deferred, all ~250 entries, read at 2026-08-22 scope
- oracle: grep -n 'RULED\|AUTHORIZED\|DIRECTION IS SUPPLIED\|operator 2026-08-' TASK-QUEUE.md, then read each hit's entry whole and check the ruling's discharge is still owed
- rev: 62aecf6aa07b6cc6850c82d619a5fab793222cea
- finding: FOUR, and all four write ONE surface (.claude/settings.json plus guard-kit config): settings-allow-intended-breadth-declaration (RULED BUILD IT 2026-08-20, knob glob-plus-reason beside GUARD_KIT_BREADTH_PROBES, committed file only); guard-grant-review (direction supplied 2026-08-20, narrow on security grounds only, UNBLOCKED, and TRAJECTORY.md names it as the entry discharging the allowlist ruling; 105 Bash grants in the committed file); session-mechanic-grants-uncommitted (disposition (a) GRANT ruled 2026-08-20, append form only, settings write is its own build work, threshold recurrence x3); subagent-stop-liveness-hook-wiring (AUTHORIZED 2026-08-20 for the logging-only hook variant, a settings hook registration no agent session may authorize alone, feeds turn-end-chokepoint-and-wait-primitive at threshold x3). Witness: re-run the grep and confirm each of the four still lacks a landing note.

## 2026-08-22 scope — Which port members are still owed, and is any takeable at this cut?
- corpus: the whole gates registry, 104 members
- oracle: bash gate-sdk/bin/port-blockers.sh --group (trailer), cross-read against '# port-until:' declarations in the tree
- rev: 62aecf6aa07b6cc6850c82d619a5fab793222cea
- finding: 5 owed, 0 TAKEABLE. 96 ported, 3 permanently shell (check-install-disposition, check-crate-arms, check-gate-substrate-parity), 5 temporarily held and every one declaring '# port-until: cohort-held-members-port-prerequisites': gate-sdk check-shellcheck, check-gate-assertions, check-action-run-shell, check-tree-terms, and site-kit check-docs-render-fidelity. Grounds are criterion 7 (shellcheck on PATH for two, 'paste' for check-gate-assertions, SITE_KIT_RENDERER's first element for check-docs-render-fidelity) and criterion 4 for check-tree-terms. So the budget arm cannot select and the port's next unit is the design entry, not a wider cut - which is also what native-gate-port-remaining-corpus itself says.
