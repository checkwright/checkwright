# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-09-25 scope — Which CI legs still carry a hand-carried continue-on-error posture, and can a held producer red a binding consumer?
- corpus: .github/workflows docs/install.md
- oracle: grep -n continue-on-error .github/workflows/*.yml; grep -n '(joined)\|(held)' docs/install.md
- rev: 6492a7ed011d58eab71c4580704962b0e5381d2b
- finding: Every continue-on-error in the workflows is derived (per-target roster index, pwsh_legs, matrix.held); none is hand-carried. All six declared targets are joined, so no held producer exists at HEAD. install-smoke-sh-macos is the one consumer leg whose binding posture is hard-coded rather than read from the index. A failed continue-on-error matrix leg satisfies needs: run 34200226768's held darwin native-artifacts legs failed and install-smoke-macos still ran (measured at spec, gh run view).
- inferred: none

## 2026-09-25 scope — Which deferred entries pass the enhancement admission filter, and how do they rank?
- corpus: TASK-QUEUE.md
- oracle: bash gate-sdk/bin/run-gates.sh --emit queue-edges
- rev: 6492a7ed011d58eab71c4580704962b0e5381d2b
- finding: No session- or iteration-class entry passes the filter (heterogeneous-agent-delegation, the one iteration class, is excluded). Filter-exempt debt at event/high: the three .github CI-leg entries (instrument-leg-expiry, held-ci-leg, binding-intel-leg) and push-need-uncounted-at-scope. Trust-arm candidates: ps-scratch-script-unsteered, side-effect-free-read-arms (guard-kit). Roadmap rungs are demand-gated or filter-held. No entry reaches the recurrence threshold of 2. Inbound edges fix no rank change: the highest in-degree (companion-toolkit-profile, heterogeneous-agent-delegation, session-model-identity-verification, benchmark-ab-experiment: 2 each) are filter-held or blocked.
- inferred: none
