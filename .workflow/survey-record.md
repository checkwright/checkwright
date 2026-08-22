# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.
































## 2026-08-23 scope — What does the eighth port cut compose from, and what remains unported after it?
- corpus: gate-sdk/checks gate-sdk/SPEC.md TASK-QUEUE.md scripts/gates.list
- oracle: bash gate-sdk/bin/port-blockers.sh --group
- rev: 2f4a34596d406ab6bb77249fd7d67774034f6266
- finding: Trailer read 2026-08-23: 106 scanned, 2 groups, 0 undecidable, 98 ported, 3 permanently shell, 3 temporarily held; 5 owed, 2 takeable. The takeable tier is exactly check-gate-assertions (lines=148, c2=pair, c3=align-only, c7=paste) and check-tree-terms (lines=66, c2=pair, c3=precommit, c7=clean), in two groups of one — so the size arm stays exhausted and the budget arm composes, per gate-sdk/SPEC.md §The first cohort, and the rule that selects the next. Both were retired from the held tier on 2026-08-22 with their prices relocated to their own SPEC sections, so neither carries a # port-until: declaration and neither needs a fresh hold adjudication: §check-gate-assertions prices paste -sd, - as a comma join over a sorted set (criterion 7 class (ii), verdict identical either side) and names the GNU-awk 3-arg match() as a capture-API re-expression rather than a hold; §check-tree-terms prices the criterion-4 bind as a fixture widening to be done BEFORE the port, its walk over git ls-files putting every registry declaration inside its own scanned corpus. Judgment: taking both empties the takeable tier, after which every unported member is behind a named sub-project — cohort-held-members-port-prerequisites (line 486) holds the three, each blocked on a shellcheck-free lint or a renderer-free render. Two entries name the port as their closure route (gate-battery-parallel-execution, gate-battery-result-cache, cited at TASK-QUEUE.md:51). Port-adjacent rider candidates surveyed and found NOT shovel-ready: born-native-flip-enforcement-gate (consumer-safety of the shape is unbuilt design work, TASK-QUEUE.md:5756) and graph-port-bash-spawn-residue (build refused it 2026-08-21; cost recorded as low and bounded).
