# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-11-25 scope — Which deferred and icebox entries serve the orchestrator adopter's asks, how do they rank, and does anything supersede them?
- corpus: TASK-QUEUE.md gate-sdk/SPEC.md native/src/runner.rs
- oracle: bash gate-sdk/bin/run-gates.sh --emit queue-edges; grep -n -i 'adopter\|orchestrator\|outside engine' TASK-QUEUE.md; grep -n 'dispatch_all\|provenance_of' native/src/runner.rs
- rev: dd6fd2d8ed602ecaef677bd22f3a0d00d5ec25e3
- finding: Adopter-tied: verdict-key-before-dispatch (blocks the adopter enabling the store), verdict-fix-release-notify (release-tied), exit-status-site-unification (outside engine retries on 3), adopter-content-checks (Ask D, nine checks, one feature family). Icebox holds no adopter-requested entry. No deferred row is session- or iteration-class; no entry at the recurrence threshold (2); none of the four carries an inbound edge. Supersession: none for the store fix; exit-status-site-unification reshapes adopter-content-checks (new gates born on the unified exit rule). Ask D splits into three families: deny-list (text, names, strings, ghpin) on the redacted record with rule ids, since-base-ref (literals, migrations), code-shape (pipefail, script, exec-write).
- inferred: the adopter's priority among Ask D's nine checks and the script rule list are not on any tracked surface; asked through the lead
