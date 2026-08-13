# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.













## 2026-08-13 scope — Which cohort should the next native-port iteration take, and what does the remaining 67-gate corpus group into?
- corpus: scripts/gates.list (103 registered members), the */checks/ tree, gate-sdk/SPEC.md cohort sections, canon-kit/SPEC.md lib/spec.sh
- oracle: bash gate-sdk/checks/check-gate-substrate-parity.sh; bash gate-sdk/bin/port-blockers.sh; git ls-files '*/checks/*.sh' '*/checks/*.gate'
- rev: 8ae0e1ee4bf6dea11715d305c4886305c8056f17
- finding: 103 members, 36 dispatching, 67 remain. Remaining shell by dir: gate-sdk 26, scripts/ 13, canon-kit 12, evidence-kit 4, context-kit 4, delegation-kit 3, site-kit 2, queue-kit 2, lifecycle-kit 2, doctrine-kit 1. Criterion-7 external-program members are only six: shellcheck (check-action-run-shell, check-shellcheck), cargo (check-crate-arms), ruby (check-docs-render-fidelity), jq (check-installer-no-deps, check-memory-off, check-settings-pins). TWO BLOCKER-RETIRING CANDIDATES DOMINATE. (a) The spec_comment_surface family: exactly four callers, all still shell — check-comment-tier (with_templates), check-spec-pointer, check-deprecation-task, check-todo-task-liveness — one shared corpus primitive whose file set spans *.sh, *.gate and *.rs (canon-kit/SPEC.md 'The governed comment surface'); the *.rs arm is why all four fail criterion 4 (self-referential parity), already recorded on cohort-held-members-port-prerequisites. (b) The ERE dividend: the POSIX ERE matcher landed at native/src/ere.rs and is owed by nine members, of which the ERE cohort consumed only three (check-install-claim, check-payload-claim, check-manifest-temporal, all now .gate) — so roughly six members carry a paid blocker, spread across canon-kit, gate-sdk and queue-kit rather than sharing one derivation. Shell gates still interpreting a consumer pattern knob: check-comment-tier (screened out of the ERE roster), check-deprecation-task, check-spec-derivable-section, check-tree-terms, check-queue-prose-precondition. Also verified: normal_diff is defined once, at native/src/gates/lifecycle_registration.rs:14, and seven gates need it at their own port (six freshness gates plus check-gate-tamper); five bin/ scripts still source queue-kit/lib/queue.sh; both lifecycle gate-test runners still hold a script path (check-stage-entry.test.sh:18, check-close-surfaces.test.sh:15). NOT SETTLED: a full grouping of gate-sdk's 26 and scripts/ 13 by shared corpus derivation — the delegated sweep did not return, so no clean-group rival from those two blocks has been sized.
