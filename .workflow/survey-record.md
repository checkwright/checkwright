# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.



































## 2026-08-24 scope — Which deferred entries have reached LIFECYCLE_KIT_RECURRENCE_THRESHOLD and so enter this scope's proposed unit set regardless of theme, and which of them are already promotion-committed to this boundary?
- corpus: TASK-QUEUE.md, all 24 recurrence: declarations
- oracle: awk over '^  recurrence: ' lines counting YYYY-MM-DD tokens per declaration, threshold 2 from lifecycle-kit/lib/stages.sh:56; then a read of each at-threshold entry's body for a promotion commitment
- rev: 81031fa23155d9934d8e7fb82cc6e40bc7815818
- finding: SEVEN entries are at or above 2: turn-end-chokepoint-and-wait-primitive (4 stamped), scratch-execution-control-is-bash-only (4 stamped), kfric-empty-log-ambiguity (2), close-entry-baseline-bootstrap-deadlock (2), agent-worktree-reclamation-unenforced (2 at read, stamped to 3 this session on a firing found at this boundary), delegation-provenance-floor (2), icebox-candidate-eligibility-unapplied (2). The judgment half a later stage would re-buy is that the DATE COUNT UNDERSTATES TWO of them by design: turn-end's declaration hit check-queue-wrap's 100-column ceiling and carries a 5th and 6th judged firing in prose only (true count 6, stamped 4); scratch-execution-control-is-bash-only likewise carries a 5th, 6th and 7th measurement in prose (true count 7, stamped 4). Every other count is exact. TWO of the seven are NOT discretionary: scratch-execution-control-is-bash-only carries 'PROMOTION COMMITTED AT THE NEXT BOUNDARY - OPERATOR RULING 2026-08-23, REVERSING THEIR OWN FOUR PRIOR DECLINES', and turn-end-chokepoint-and-wait-primitive carries 'SEQUENCED 2026-08-22 BY THE LEAD; THE CONDITION IS NOW PAID AND PROMOTION IS DUE AT THE NEXT BOUNDARY' plus 'the pair still promotes together; a boundary that promotes neither is the failure the unstampable dates cannot signal'. Its blocker subagent-stop-payload-background-tasks-read is Done and paid out as nothing collapsed, so the sequencing condition is discharged. THIS is that boundary. The other five have all been held repeatedly on surface grounds (delegation-provenance-floor 4 times, close-entry-baseline-bootstrap-deadlock 6 times), and no hold added a recurrence date because a decline is not a firing.

## 2026-08-24 scope — What does the rest of the queue converge on at this undirected boundary — which candidates carry the inbound weight, and does the port sequence directive's next member carry any?
- corpus: TASK-QUEUE.md, ~230 deferred entries plus 47 icebox one-liners
- oracle: bash queue-kit/bin/queue-edges.sh, ranked by inbound count; cross-read against TRAJECTORY.md's PRIORITY DIRECTIVE sequence and gate-sdk/bin/port-blockers.sh both arms
- rev: 81031fa23155d9934d8e7fb82cc6e40bc7815818
- finding: Top inbound: prose-filename-citation-liveness 5; delegation-provenance-floor 4; benchmark-ab-experiment 4; then powershell-installer-surface, platform-support-ci-matrix, native-gate-port-remaining-corpus, session-model-identity-verification, dispatch-cited-evidence-unverified and build-stage-tier-economics at 3 each. THE BOOTSTRAP PAIR IS THE INTERESTING READING and it is aggregation-blind in the naive direction: powershell-installer-surface carries 3 inbound while install-step-relocation carries only 1 — but that single edge is powershell-installer-surface itself saying 'install-step-relocation is what moves it there ... its size depends on that relocation landing first', so the pair's weight sits on the member with the LOWER count and reading the two separately inverts their order. TRAJECTORY.md's PRIORITY DIRECTIVE names exactly this pair as next in the port track, both prior members (battery-runner-port, shell-gate-tail-port) being done. Premise re-verified rather than carried: install-step-relocation's narrowed claim HOLDS at installer/lib/init.sh — line 313 still runs 'gen-pre-commit.sh --write' (297 lines of shell, the one genuinely un-relocated step) and line 317 reaches the graph emitter through 'run-gates.sh --emit graph', a thin dispatcher to the binary, exactly as the entry says. The two port oracles disagree by design and both were run: --group reads 106 scanned / 0 owed / 0 takeable (the BATTERY is finished), --tree reads 153 scanned / 0 declared no-port / 0 held / 153 OWED (the PORT is not). Only --tree's owed count is the completion predicate. The trap for a later stage: --group's clean zero is not a finished port, and the 153 includes every scripts/ config bridge, so it is a denominator rather than a work estimate.
