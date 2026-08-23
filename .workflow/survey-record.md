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

## 2026-08-23 spec — What is the full non-test shell census outside the gate corpus — the remainder the PRIORITY DIRECTIVE's completion predicate bounds and no oracle counts?
- corpus: git ls-files '*.sh' over the tracked tree, 404 files / 27650 lines
- oracle: git ls-files '*.sh' with wc -l, bucketed disjointly and reconciled on both axes; the two queue-carried figures re-run as 'git ls-files *.sh | grep -v .test.sh$' and again minus gate-tests/ paths
- rev: 36165dfbf27409051784c09c3c044ee23774a441
- finding: Both figures the entry carries are CONFIRMED byte-exact: minus *.test.sh = 17949 lines; minus gate-tests/ too = 16923 lines. 244 of 404 files sit under a gate-tests/ path and all 88 *.test.sh are among them. Subtracting the two buckets a port entry owns — 6 registered shell gates (1148 lines) and 2 kit-shipped unregistered gates (181 lines) — leaves 152 files / 15594 lines that NO queue entry owns and NO oracle counts. Disjoint decomposition, reconciling exactly to 404 files / 27650 lines: kit bin/ 42 files/5883 lines; kit lib/ 18/3397; */smoke/ 21/1962; installer/ 11/2074; scripts/ 25/968; */templates/ 20/613; drift-kit/kpis/ 13/552; demo/ 1/96; context-kit/index-tests/ 1/49. native/ and .claude/ hold zero shell. The judgment a later stage would re-buy: the 15594 figure is the DENOMINATOR of the directive's predicate, not a work estimate — most of those files will declare or be deleted rather than port, and SPEC-port-oracle.md deliberately takes no disposition on any of them.

## 2026-08-23 spec — For each of shell-gate-tail-port's eight members: what external program does it spawn, at what line, with what install disposition and tier, and does it carry a fixture pair?
- corpus: the eight members' declaration paths plus the kit libraries they source
- oracle: grep of the '# graph:'/'# install:'/'# spec:' headers, a read of each body's spawn sites, bash gate-sdk/bin/port-blockers.sh both arms, and a caller grep for ek_pid_alive
- rev: 36165dfbf27409051784c09c3c044ee23774a441
- finding: FIVE members spawn an off-floor program, not four. check-shellcheck (zero-config, precommit, 56 lines) needs shellcheck at :27,:46. check-action-run-shell (on-surface, precommit, 222) needs shellcheck at :17,:193, plus an inline awk YAML extractor at :169. check-crate-arms (never, precommit, 78) needs cargo at :28,:53,:60 AND rustc at :42 — two programs, where criterion 7's prose names cargo alone. check-docs-render-fidelity (on-surface, precommit, 241) needs the renderer, KNOB-DERIVED via SITE_KIT_RENDERER_BATCH at :32 and SITE_KIT_RENDERER at :42, default is a ruby kramdown one-liner. check-producer-liveness (never, align-only, 75, UNREGISTERED) needs ps, reached through ek_pid_alive at evidence-kit/lib/evidence.sh:117-122 (kill -0 builtin first, ps -p fallback). The other three spawn only floor programs: check-install-disposition (zero-config, precommit, 92, c7=clean), check-gate-substrate-parity (on-surface, precommit, 459, spawns the gate binary itself via a BIN variable --list at :223), check-surface-duplication (on-surface, align-only, 106, UNREGISTERED, dir=bi). All eight carry a good/+bad/ fixture pair; none carries a no-fixture, no-port or port-until header — those two port fields have ZERO declarations tree-wide. Install dispositions: exactly TWO are zero-config (check-shellcheck, check-install-disposition, both gate-sdk), which is what bounds criterion 5's predicted residual growth at two. The judgment a later stage would re-buy: ek_pid_alive keeps a LIVE shell caller after the port at evidence-kit/bin/run-validate.sh:52, so that port CREATES a criterion-6 dual implementation discharged by a standing comparison, never by deletion.
