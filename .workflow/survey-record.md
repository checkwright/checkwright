# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.


































## 2026-08-23 scope — Which deferred entries have reached LIFECYCLE_KIT_RECURRENCE_THRESHOLD and so enter the proposed unit set regardless of theme?
- corpus: TASK-QUEUE.md
- oracle: grep -c on '^  recurrence: ' lines in TASK-QUEUE.md, counting YYYY-MM-DD tokens per declaration; threshold 2 per lifecycle-kit/lib/stages.sh
- rev: ce87fae5324e928a0fa9be138a3b8ca58d790588
- finding: 21 declarations carry 32 dates. SEVEN entries are at or above the threshold of 2 and therefore enter this scope's proposed unit set regardless of directive: turn-end-chokepoint-and-wait-primitive (4 stamped, a 5th judged 2026-08-23 and unstampable because a 5th date is 107 columns against check-queue-wrap's 100 — recorded in the entry's prose this drain, so the count reads 4 and the firing count is 5); scratch-execution-control-is-bash-only (4); icebox-candidate-eligibility-unapplied (2); delegation-provenance-floor (2); agent-worktree-reclamation-unenforced (2); close-entry-baseline-bootstrap-deadlock (2); kfric-empty-log-ambiguity (2). The judgment half a later stage would re-buy: the oracle's date count UNDERSTATES turn-end by one for exactly the reason the ceiling exists, so a session re-running it and reading 4 must not conclude the entry is one firing from threshold — it is three past it. Every other count is exact.

## 2026-08-23 scope — What does the rest of the queue converge on for the port tail, and which entries unblock when shell-gate-tail-port lands?
- corpus: TASK-QUEUE.md
- oracle: bash queue-kit/bin/queue-edges.sh
- rev: ce87fae5324e928a0fa9be138a3b8ca58d790588
- finding: shell-gate-tail-port carries 6 inbound edges from 4 distinct citers, the most of any port-cluster entry: native-gate-port-remaining-corpus (names it second in the operator sequence), port-oracle-corpus-narrower-than-the-directive (names it as the owner of the six owed gates and the two unregistered ones), and TWO entries that are [blocked-by:] it outright — interpreter-floor-gawk-residue-empty and binary-less-dispatch-loop-retirement. Both unblock on its landing, so the promotion dividend is three entries, not one. port-oracle-corpus-narrower-than-the-directive has ZERO inbound and is the trap in the set: it is the only entry that owns the completion MEASUREMENT, so shell-gate-tail-port landing makes port-blockers.sh read 0 takeable over a 106-member gate census while roughly 14k non-test shell lines stand outside that corpus. Its low inbound count is aggregation-blind, not a low rank. criterion-4-two-spellings-disagree shares shell-gate-tail-port's SPEC section (gate-sdk/SPEC.md, The port-candidate criteria), which that unit already edits to retire exception class (a). turn-end-chokepoint-and-wait-primitive carries 3 inbound but is sequenced BEHIND subagent-stop-payload-background-tasks-read by a lead ruling of 2026-08-22, and that blocker has 1 inbound and costs one deliberate value read plus a privacy-ruling decision.
