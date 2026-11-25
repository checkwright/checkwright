# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-11-25 scope — Which deferred and icebox entries serve the orchestrator adopter's asks, how do they rank, and does anything supersede them?
- corpus: TASK-QUEUE.md gate-sdk/SPEC.md native/src/runner.rs
- oracle: bash gate-sdk/bin/run-gates.sh --emit queue-edges; grep -n -i 'adopter\|orchestrator\|outside engine' TASK-QUEUE.md; grep -n 'dispatch_all\|provenance_of' native/src/runner.rs
- rev: dd6fd2d8ed602ecaef677bd22f3a0d00d5ec25e3
- finding: Adopter-tied: verdict-key-before-dispatch (blocks the adopter enabling the store), verdict-fix-release-notify (release-tied), exit-status-site-unification (outside engine retries on 3), adopter-content-checks (Ask D, nine checks, one feature family). Icebox holds no adopter-requested entry. No deferred row is session- or iteration-class; no entry at the recurrence threshold (2); none of the four carries an inbound edge. Supersession: none for the store fix; exit-status-site-unification reshapes adopter-content-checks (new gates born on the unified exit rule). Ask D splits into three families: deny-list (text, names, strings, ghpin) on the redacted record with rule ids, since-base-ref (literals, migrations), code-shape (pipefail, script, exec-write).
- inferred: the adopter's priority among Ask D's nine checks and the script rule list are not on any tracked surface; asked through the lead

## 2026-11-25 build — Which native read sites of a configured file map absent or unreadable onto the wrong side of the exit cut?
- corpus: native/src '*/checks/*.sh' '*/gate-tests/*.test.sh'
- oracle: git grep -n 'read_to_string' -- native/src
- rev: 26e6fd5981cd45c8ee1e0d973ecc80142a9e2ac8
- finding: Direct sites split three ways: absent mapped to 3, unreadable mapped to 2, and swallowed reads; String funnels (knob loader, spec finders, graph registry, projection roster, fresh::read_captured) carry the read error untyped to callers that map one kind.
- inferred: the knob-loader funnel reach (561 walk::knob_* call sites) is counted by grep, not traced per caller
