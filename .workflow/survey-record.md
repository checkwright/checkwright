# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-09-17 scope — Undirected scope ranking: which deferred entries lead the next unit set by cost class, roadmap, surface
- corpus: TASK-QUEUE.md
- oracle: bash gate-sdk/bin/run-gates.sh --emit queue-edges
- rev: 821b2353a1aa849059061b49fdfd46ab55a164d2
- finding: Shortlist premises held for close-differential-instruction-sweep (no differential step in close.md or its binding), spec-authoring-self-check-pass (no authoring-exit predicate re-run in spec.md), survey-inferred-claim-has-no-execution-obligation (lifecycle-kit SPEC survey record still permits carrying an inferred claim onward). lifecycle-kit carries 4 iteration/low rows, queue-kit 4. heterogeneous-agent-delegation is the only iteration-class roadmap row.
- inferred: batch-split-stamp-ownership cost no longer incurred, read off stamp-commit subjects since 2026-08-29 rather than the state file; kfric-obligation-residency partly carried by lead.md journal-disposal line

## 2026-09-17 spec — Which files form the instruction-surface sweep corpus, and where does the previous close's range base sit on this tree?
- corpus: */templates/*.md .claude/agents/*.md .workflow/WORKFLOW-STATE.txt
- oracle: git ls-files -- '*/templates/*.md' '.claude/agents/*.md' ':!*/gate-tests/*'
- rev: 9dbd27302d5d3b14e37b9ada0ec08f5bf87f2bb9
- finding: The pathspec without the gate-tests exclusion also matches a fixture guide.md; with it, git ls-files lists 20 files, the set the full instruction pass swept. The state file at the iteration-start commit a024f14f yields the previous close-entry head bea4dd49 (awk over stage==close, field 5); the previous close reviewed its audit roster at 58e9a8ed and ran its brevity pass at a024f14f, so the iteration-start range excludes that pass. check-stage-entry's simulated build entry proceeds with both new lifecycle-kit amendments on disk, so assertion C does not fire.
- inferred: none
