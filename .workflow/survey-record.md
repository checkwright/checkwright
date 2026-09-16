# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-09-16 scope — Do the two session-class deferred entries' measured premises still hold against the current tree?
- corpus: TASK-QUEUE.md .workflow/*.log .workflow/essay-harvest.md scripts/gates.list scripts/delegation-config.knobs
- oracle: bash gate-sdk/bin/run-gates.sh --emit queue-edges
- rev: 7a219118b8c8eb9491728b924f43566967f15a1f
- finding: BOTH HOLD; both entries' recorded figures are stale and must be re-measured at spec time. retired-block-admits-live-gate-name: live queue-edges prints 55 retired targets over 88 citing rows with check-spec-pointer carrying 15 (17 percent), against the entry's recorded 51/92/14 (15 percent); check-spec-pointer is registered at scripts/gates.list:103, so it is live mechanism, not disposed work. delegated-read-blind-to-gitignored-capture: the D2 mandate is live (DELEGATION_KIT_READONLY_TYPES[] = audit-sweep, scripts/delegation-config.knobs:4) and was attested this session by the guard refusing a non-worktree audit-sweep dispatch; three of the four named gitignored capture surfaces still carry content (prompt-friction.log 41, subagent-stop-liveness.log 13, essay-harvest.md 220) against the entry's 822/561/220, and knowledge-friction.log is now 0 against its recorded 18.
- inferred: none
