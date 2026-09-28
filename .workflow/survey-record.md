# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.

## 2026-09-28 scope — Which deferred entries lead this iteration's unit set under the scope ranking, and what cites each shortlisted candidate?
- corpus: TASK-QUEUE.md
- oracle: bash gate-sdk/bin/run-gates.sh --emit queue-edges
- rev: 6bad618516e7d61b47aaa552b3336542434fdf14
- finding: Over the Deferred section, 67 entries after the inbox drain: spec-brevity-residue (session/high) leads; the lifecycle-kit surface holds five rows, one (companion-toolkit-profile) gated on an observed install; queue-edges shows consult-inbox cited by roadmap-horizon-motion-unowned and blocking consult-inbox-drain-trigger, and no inbound edge on tier-only-rank-out-unruled.
- inferred: The cost-class tiers and per-surface row counts were read off the Deferred entries' tag lines, not re-derived by a tool.

## 2026-09-28 scope — How many words does each top-level section of lifecycle-kit/SPEC.md carry, to size the next brevity slice?
- corpus: lifecycle-kit/SPEC.md
- oracle: awk '/^## /{if(h)print w, h; h=$0; w=0; next} {w+=NF} END{print w, h}' lifecycle-kit/SPEC.md
- rev: 6bad618516e7d61b47aaa552b3336542434fdf14
- finding: 61,601 words in all. Above Per-component contracts: The state machine 5,972; The steering vocabulary 442; Layout and configuration 3,419; Multi-operator semantics 814; The committed gap inbox 6,384; The survey record 3,632 with its grammar example; The close-surface roster 1,041; The audit roster 944; Testing 648 — about 23.3k. Per-component contracts 38,147, of which templates/stages/ 5,081 and templates/lead.md 2,053 already landed as lifecycle-template-brevity; bin/enter-stage.sh alone is 11,349.
- inferred: none

## 2026-09-28 spec — Which tracked surfaces read the gap inbox's knob, arm or union membership, so a sibling consult inbox owes them a row?
- corpus: native/src lifecycle-kit canon-kit/SPEC.md gate-sdk/SPEC.md CLAUDE.md .claude .gitattributes
- oracle: git grep -n -e file-gap -e file_gap -e GAP_INBOX_FILE -e union_set -e 'two tracked capture arms' -e 'design-ahead records'
- rev: 44fe5a846f5680b95a6cfebd88c8bf7063823c95
- finding: Readers: emit/mod.rs arm table, file_gap.rs, knobs/lifecycle_kit.rs, stages::union_set (merge_attrs.rs help, install_lifecycle.rs KNOBS, .gitattributes block), kit_ref_liveness.rs design-ahead valve (canon-kit/SPEC.md Layout and configuration), dispatch_entry.rs lead-capture skip, enter_stage.rs boundary check (gap inbox only), gap_inbox_neutrality.rs, gate-sdk/SPEC.md The workflow directory, capture-linked-worktree.test.sh, lifecycle-kit README Use block, lead.md capture paragraphs, close.md step 2, scope.md second step, CLAUDE.md gap-capture line. lifecycle-kit/SPEC-consult-inbox.md delta 7 carries the rows a consult inbox owes.
- inferred: none

## 2026-09-28 build — Which facts do other surfaces cite into lifecycle-kit/SPEC.md's sections above Per-component contracts, so a brevity pass over them must keep them?
- corpus: .
- oracle: git grep -n -e '§The state machine' -e '§The stamp protocol' -e '§Deviation transitions' -e '§The steering vocabulary' -e '§Multi-operator semantics' -e '§The committed gap inbox' -e '§The consult inbox' -e '§The survey record' -e '§The close-surface roster' -e '§The audit roster' -e 'lifecycle-kit/SPEC.md §Testing' -e 'lifecycle-kit/SPEC.md §Layout and configuration'
- rev: a2535dcf16cf61276a94ea2d4f4f415c8cc500b3
- finding: Must-keep facts beyond the headings: the journal path's one source is the entry report and no dispatch prompt names it (delegation-kit, agent-execution, stage-session agent); the approval-prompt misreading with its generality-and-salience test (lead.md); the four lead honest-limit conclusions (lead.md); the lead writes no stamp, queue write, evidence file or push; same-stage re-entry; the iteration-start and previous-close commit definitions with their no-commit cases (context-kit, evidence-kit, drift-kit, audit roster); an interstitial commit falls into the next range (drift-kit); the direction-landing rule (stage-session agent); the five steering classes, direction revisable and ruling not (queue-kit, TRAJECTORY, CLAUDE.md), and the open relayed-answer question; the cheap-capture/drain-re-verifies argument (drift-kit); the recurrence obligation and its same-commit mandated write (queue-kit, CLAUDE.md, context-kit); the ordered drain dispositions, owner lookup and inferred marker (stage templates); the gate_run reach rule and the seven enter-stage harnesses' second caller (gate-sdk criterion 7); the close-surface directive line byte-exact; the survey and audit-roster grammar blocks verbatim.
- inferred: The parsed-text constraints (the survey and audit-roster grammar blocks, the unfenced close-surface directive line) were read off native/src rather than exercised by a run; the post-pass battery is the run.
