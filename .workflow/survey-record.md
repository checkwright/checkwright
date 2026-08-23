# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.
































## 2026-08-23 scope — What does the eighth port cut compose from, and what remains unported after it?
- corpus: gate-sdk/checks gate-sdk/SPEC.md TASK-QUEUE.md scripts/gates.list
- oracle: bash gate-sdk/bin/port-blockers.sh --group
- rev: 2f4a34596d406ab6bb77249fd7d67774034f6266
- finding: Trailer read 2026-08-23: 106 scanned, 2 groups, 0 undecidable, 98 ported, 3 permanently shell, 3 temporarily held; 5 owed, 2 takeable. The takeable tier is exactly check-gate-assertions (lines=148, c2=pair, c3=align-only, c7=paste) and check-tree-terms (lines=66, c2=pair, c3=precommit, c7=clean), in two groups of one — so the size arm stays exhausted and the budget arm composes, per gate-sdk/SPEC.md §The first cohort, and the rule that selects the next. Both were retired from the held tier on 2026-08-22 with their prices relocated to their own SPEC sections, so neither carries a # port-until: declaration and neither needs a fresh hold adjudication: §check-gate-assertions prices paste -sd, - as a comma join over a sorted set (criterion 7 class (ii), verdict identical either side) and names the GNU-awk 3-arg match() as a capture-API re-expression rather than a hold; §check-tree-terms prices the criterion-4 bind as a fixture widening to be done BEFORE the port, its walk over git ls-files putting every registry declaration inside its own scanned corpus. Judgment: taking both empties the takeable tier, after which every unported member is behind a named sub-project — cohort-held-members-port-prerequisites (line 486) holds the three, each blocked on a shellcheck-free lint or a renderer-free render. Two entries name the port as their closure route (gate-battery-parallel-execution, gate-battery-result-cache, cited at TASK-QUEUE.md:51). Port-adjacent rider candidates surveyed and found NOT shovel-ready: born-native-flip-enforcement-gate (consumer-safety of the shape is unbuilt design work, TASK-QUEUE.md:5756) and graph-port-bash-spawn-residue (build refused it 2026-08-21; cost recorded as low and bounded).

## 2026-08-23 align — Do the leak-guard-and-assertion-meta-gate-port iteration's four SPEC amendments' claims about the tree hold, and are their self-authored counts and Existing-sections-updated rosters internally consistent?
- corpus: gate-sdk/SPEC-eighth-cut.md guard-kit/SPEC-journal-append.md queue-kit/SPEC-entry-split.md delegation-kit/SPEC-wait-primitive.md, plus every tree surface each cites: gate-sdk/checks/*.sh native/src/gates/*.rs gate-sdk/SPEC.md canon-kit/SPEC.md guard-kit/lib/guard.sh guard-kit/SPEC.md guard-kit/guard-tests/cases.tsv queue-kit/SPEC.md queue-kit/bin/queue-edges.sh delegation-kit/SPEC.md delegation-kit/templates/agent-execution.md lifecycle-kit/SPEC.md TASK-QUEUE.md TRAJECTORY.md .claude/settings.json .claude/agents/*.md .gitignore docs/ddd.md docs/install.md docs/site-architecture.md
- oracle: none - a one-time audit of four pre-merge amendments that delete at merge; not a re-runnable witness, four parallel read-only audit-sweep agents (isolation worktree) did the reading, one per amendment
- rev: 680d93d7de3e6b5cda16e96dcee69f66bddad758
- finding: Self-audit (delta count vs headings, every Existing-sections-updated bullet citing a delta) passed clean on all four amendments. Tree-audit found and this align session fixed six real divergences. gate-sdk/SPEC-eighth-cut.md named the graph artifact scripts/CHECK-GRAPH.html where the tracked/configured one is docs/check-graph.html (scripts/gate-sdk-config.sh, docs/site-architecture.md) - fixed. guard-kit/SPEC-journal-append.md misattributed the GUARD_KIT_RO_BINS honest-limit precedent to rule 18, it is rule 17's text today (SPEC.md:1166 labels the knob rule 17) - fixed. queue-kit/SPEC-entry-split.md's lifecycle-kit/templates/lead.md bullet neither restated the split recipe nor cited its owner, satisfying neither branch of the lead's Q4 ruling - added an explicit queue-kit/SPEC.md section check-queue-entry-budget pointer. delegation-kit/SPEC-wait-primitive.md's scratch-root paragraph claimed writes nothing in-tree while defaulting under .tmp (in-tree, gitignored) and cited demo/run-demo.sh plus scripts/pack-installer.sh as precedent when both actually default to real system temp - rewritten to ground the choice in the key.run liveness-record mandate instead. Its Producers-and-consumers section claimed an existing .gitignore capture-tier pattern already covers the new evidence file - false, every .workflow capture-tier member is an individual explicit line, a new line is required (the amendment's own roster bullet already planned this correctly) - fixed. Its branch-3 justification cited the fifteenth firing as evidence for event-stream reliability, but that firing count is the queue entry's first-half chokepoint-frequency evidence, a different question from the second-half four-dead-waiters measurement branch 3 actually rests on - corrected the citation. Also confirmed and left untouched: gate-sdk criterion-4's two-spellings gap stays filed-not-reconciled per lead ruling, no amendment reaches into rewriting the criterion itself; check-comment-tier's self-audit precedent for check-gate-assertions checks out; delegation-kit's carrier-surface roster undercounted this repo's agent definitions as one file where there are two, stage-session.md and audit-sweep.md - fixed in two places so a build session does not edit only one. Full battery reconfirmed 106/106 green both before and after the fixes.
