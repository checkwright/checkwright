# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.





































## 2026-08-24 scope — Which Deferred entries sit on the turn-end / agent-dispatch / producer-liveness control set, so an iteration taking the subagent-stop exit-2 defect can bundle a shared-surface batch?
- corpus: TASK-QUEUE.md '## Deferred', 243 entries, read across the scope commits below the rev field (line numbers current at HEAD); plus scripts/bash-guard.sh, guard-kit/lib/guard.sh, delegation-kit/SPEC.md, evidence-kit/checks/check-producer-liveness.gate
- oracle: grep -n '^- \*\*' over the Deferred span for the entry index; two vocabulary sweeps (liveness|subagent-stop|worktree|isolation|turn-end|scratch|fail-closed|gate-exec|resume journal|dispatch, then background|notification|producer|kill -0|pgrep); queue-edges.sh --inbound per shortlisted slug; ~40 raw hits, 22 entries read in full, 14 kept
- rev: 90dfe576c92ff1fa72055aea5d1c37bc78296738
- finding: FOURTEEN entries are on-surface, all [design-pending], NONE roadmap-tagged. Four coherent groups, cheapest shared context first. (1) THE GUARD WAIT/.run QUAD, one function and one span walk: backgrounded-shell-child-run-record-unenforced 6717 + wait-loop-exemption-blind-behind-a-script-name 6847 (the queue's own text calls these the false-negative/false-positive pair of ONE text-shaped predicate) + waiter-loop-condition-predicate-gap 5104 + wait-loop-grant-lost-its-carrier 6809 (recurrence 1 date). Re-verified in the parent rather than relayed: guard.sh:785 calls guard_advise, not guard_block, so the .run record is advisory and rule 14's git-write block is a no-op against a producer that never announced itself. (2) THE WORKTREE PAIR, hard sequencing edge: worktree-reclamation-cause-falsification 7771 is the SELF-DECLARED UPSTREAM INPUT to agent-worktree-boundary-disposition 5823, which cannot be designed without it; 5823's open question is inventing a worktree liveness signal by analogy to the .run record plus the stage-entry gate. (3) THE gate-exec.sh PAIR, one 22-line file: kit-knob-consumer-adapter-convention 7180 asks whether that adapter shape becomes a named convention while single-gate-front-end-form-unruled 7393 asks whether the port retires the file, so deciding either alone risks blessing the opposite future; 7393's grant horn is operator-class and unlandable by a stage session. (4) THE JOURNAL/PROVENANCE PAIR on delegation-kit/SPEC.md SS-Resume journal: stage-journal-contract-unoracled 6435 + delegation-provenance-floor 4443. Also on-surface and standalone: subagent-stop-agent-id-attribution-doubt 7128 (a one-bullet SPEC edit, but settling its premise needs an operator-class raw-payload read), dispatch-unreadable-target-fallback 3984 (the only worktree-CHILD contract entry), lead-dispatch-simulate-optionality 5705 and stage-completion-unattested 6010 (both kit-template envelope changes in lead.md, below the line). REJECTED WITH CAUSE as topic-only, not surface: dispatch-cited-evidence-unverified, self-revert-reminder-expectation, lead-state-durable-home, settings-hook-command-path-gate, gate-file-coverage-closure, unregistered-gate-fixture-coverage, boundary-wipe-preserve-basename-reach, bridged-knob-case-tmp-dir-override-inert, bespoke-test-path-knob-pinning, port-blockers-library-mediated-scan, throughput-and-wait-time-unmeasured, overhead-meter-measures-the-lead, relayed-rule-role-scope-unchecked, dispatch-claim-evidentiary-tier-unmarked, lead-specifies-constraint-not-mechanism, precondition-gate-negation-false-positive. THE INBOUND SUM, and it needs reading with care: delegation-provenance-floor carries 4 inbound, the cohort's highest, but ALL FOUR citing lines are 'DISTINCT from' disambiguations rather than convergences, so the sum measures a crowded class rather than a promotion dividend. Every other cohort member carries 0 or 1.
