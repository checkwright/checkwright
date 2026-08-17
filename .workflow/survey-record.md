# contract: lifecycle-kit/SPEC.md §The survey record — carried surveys, one block per survey; boundary-truncated, cited only behind a passing witness.






















## 2026-08-17 scope — Which deferred entries reach LIFECYCLE_KIT_RECURRENCE_THRESHOLD, and what is each one's standing at this boundary?
- corpus: TASK-QUEUE.md
- oracle: grep -n 'recurrence:' TASK-QUEUE.md
- rev: 687cf956552e209004adabf705cd1b0b0c6dbfbf
- finding: Threshold is 2 (lifecycle-kit/lib/stages.sh:56; scripts/lifecycle-config.sh does not override). 13 declarations live; THREE reach it, and the set CHANGED at this boundary. (1) waiting-rule-fourth-firing-post-fix now stands at THREE dates (2026-08-06 2026-08-16 2026-08-17), the third stamped by this session's own post-close drain. Its own body rules the consequence: 'A third threshold recurrence routes to the operator, not to a third decline - two is where lead discretion ends.' So it is OPERATOR-class, not lead-declinable. (2) icebox-worklist-roadmap-blind stands at THREE (2026-08-15 2026-08-16 2026-08-17); re-measured live at this rev, queue-index.sh --icebox-candidates offers 4 rows and 0 eligible (plugin-marketplace, benchmark-ab-experiment, hosted-attestation-service roadmap-tagged; rendered-site-link-monitor on the named-event clause) - precision zero an EIGHTH consecutive time. Its deferral stands on carrier queue-index-non-gate-arm-port, filed 2026-08-17 at scope. (3) stage-stamp-ordering-unenforced stands at TWO (2026-08-07 2026-08-16), UNCHANGED since the lead's 2026-08-16 decline recorded in-entry, so re-escalating it without a new date is churn. recurrence-drain-input-widening, which reached the threshold for the first time last iteration, has LANDED (4bea9ceb) and is off the declaration set entirely.

## 2026-08-17 scope — Which port increment is takeable next, on the gate corpus and off it, and what does each buy beyond the port itself?
- corpus: scripts/gates.list queue-kit/bin/ scripts/gen-docs-mirror.sh drift-kit/bin/trajectory.sh TASK-QUEUE.md
- oracle: bash gate-sdk/bin/port-blockers.sh --group; wc -l on the non-gate candidates
- rev: 687cf956552e209004adabf705cd1b0b0c6dbfbf
- finding: TWO tracks are takeable and they are NOT interchangeable. GATE CORPUS: --group reports 104 scanned, 69 ported, 34 groups, 0 undecidable; the SIZE arm's group 1 is check-install-disposition + check-readme-roster (both c2=pair c3=precommit c7=clean, keyed libs=fail_closed,gate_kit_roots globs=*.gate,*.sh). That is the mechanical next gate cohort and it carries no dividend beyond the port. OFF-CORPUS (the non-gate arm): queue-kit/bin/queue-index.sh is 182 lines, carries ZERO '# graph:' manifests (re-verified: grep -c returns 0) and is therefore reachable by NO selector the port runs - --group scans registered gate members, freshness-emitter-port-cohort scans the six check-*-fresh emitters. Its callers, re-verified live, are context-kit/templates/session-context.sh, context-kit/bin/always-loaded.sh, queue-kit/bin/queue-counts.sh and .claude/commands/close.md - so unlike a ported emitter it has NO gate consumer to call it in-process, which is the entry's open design half. Taking it discharges queue-index-non-gate-arm-port AND unblocks icebox-worklist-roadmap-blind, whose 3-date recurrence declaration is the highest live count in the pool and whose deferral was re-grounded onto that carrier by the operator on 2026-08-17. The remaining freshness triple is gen-docs-mirror.sh 127 + trajectory.sh 242 + roadmap.sh 76 = 445 lines, design-ANSWERED (a ported emitter is a non-gate arm) and held by sequence alone; at 445 lines it is roughly twice the twelfth cohort's ~222 and does not fit one iteration beside anything else.

## 2026-08-17 scope — Does enter-stage.sh --simulate relay each refusal's recovery guidance, and does the SPEC's simulate roster match the refusals the code runs?
- corpus: lifecycle-kit/bin/enter-stage.sh lifecycle-kit/SPEC.md lifecycle-kit/templates/lead.md
- oracle: grep -n 'help:|sim_relay|sim == 1' lifecycle-kit/bin/enter-stage.sh; read SPEC lines 1452-1464 against the refusal set in the script
- rev: 687cf956552e209004adabf705cd1b0b0c6dbfbf
- finding: NO, on both halves, and the two are one defect. (1) EVERY help: line in enter-stage.sh - 210 and 226 (preflight), 246 (Lessons), 262 and 263 (gap inbox), 287 (BOUNDARY_REQUIRE) - sits AFTER the 'if sim == 1' branch's own exit 1, so --simulate prints the refusing check's findings and never a single recovery instruction. Five refusal classes, zero recovery under simulate. The designed consumer of simulate is the LEAD: lifecycle-kit/SPEC.md §templates/lead.md (line 2485) directs it to gate an expensive dispatch with --simulate rather than hand-deriving prior-stage completeness. ATTESTED COST at this very boundary, not predicted: the lead ran --simulate scope, saw the bullets without line 263's post-close recovery, and escalated to this session an unverified negative asking whether a pre-entry queue write by scope is sanctioned - a question line 263 answers verbatim ('disposition them here, in this entering session'). (2) SPEC-vs-code drift on the same subject: lifecycle-kit/SPEC.md:1452-1464 enumerates what --simulate runs and ends '...and the iteration-boundary Lessons check - then stops', omitting the gap-inbox refusal the code runs at line 252 and the BOUNDARY_REQUIRE check at 269 (the latter is covered obliquely at line 1434, the former nowhere). NOT already filed: enter-stage-arg-position-silent-drop is flag POSITION, enter-stage-simulate-no-write-fixture (icebox) is the no-write guarantee; neither touches output actionability.

## 2026-08-17 spec — Is post-close-ness decidable from git, i.e. is the gap-inbox truncation commensurable in commit order with the close stamp?
- corpus: .workflow/gap-inbox.md and .workflow/WORKFLOW-STATE.txt history over the last 4 closed iterations
- oracle: git log --oneline -- .workflow/gap-inbox.md; git log --oneline -- .workflow/WORKFLOW-STATE.txt; git log -1 --format=%ai on each pair
- rev: 2dbd8c057422d22f5d6bb377b5f0a9e541f8c451
- finding: NO in the way that matters. The truncation is NEVER in the close-stamp commit and is ALWAYS a later, separately authored one: f0f2060c->22c5b0a4 (+7h33m), 93fcf17f->4e194b8a (+3m41s), c9f73f7f->27176682 (+6m39s), 4c863623->b2adf007 (+12m01s). So 'truncated since the close stamp?' is TRUE of every normally closed iteration; the only case it distinguishes is a close that stamped and skipped its drain, where it answers 'close-skipped' and names a stage that is gone. Where close was genuinely skipped there is no close-stamp commit to anchor on, so the predicate degenerates to 'is there a close stamp?' -- the cursor read, available from the state file with no git. SECOND FINDING, same corpus: the current post-close recovery (enter-stage.sh:263) produces a PRE-STAMP queue write by a session that has entered no stage -- 28ea8128 'chore(scope): drain the post-close gap bullet as the ninth firing' precedes 687cf956 'chore(scope): stamp scope entry'; ce10d8c1 '...as attesting 7' precedes the prior iteration's stamp the same way. enter-stage.sh invokes git NOWHERE today (only .gitkeep in comments).

## 2026-08-17 spec — What is queue-index.sh's live caller set, and what does the non-gate arm class demand of the port?
- corpus: whole tree: *.sh *.md *.gate *.rs, unsuppressed grep for 'queue-index'; native/src/emit/; gate-sdk/bin/run-gates.sh
- oracle: grep -rn queue-index across the tree with stderr live; read native/src/emit/mod.rs; ./native/target/release/checkwright-gates --knobs check-queue-entry-budget
- rev: 2dbd8c057422d22f5d6bb377b5f0a9e541f8c451
- finding: CALLER SET CORRECTED. queue-kit/bin/queue-counts.sh is NOT a caller (the entry and the 2026-08-17 scope port-increment survey both list it) -- read end to end, its only relation is a spec: citation at line 24 that both tools count the same unit. scripts/session-context.sh IS a caller and was omitted: this repo's live copy of context-kit/templates/session-context.sh, byte-identical invocation logic. Live invoking set: context-kit/templates/session-context.sh, scripts/session-context.sh, context-kit/bin/always-loaded.sh (via CONTEXT_KIT_HOOK_CMD, overridden at scripts/context-config.sh:10), .claude/commands/close.md. lifecycle-kit/templates/stages/close.md carries NO reference. context-kit/smoke/install.sh:52-53 asserts the file's PRESENCE. --extent has no in-tree invoker at all; its caller is a session doing a queue edit (queue-kit/SPEC.md:639 rests on it). NON-GATE ARM: two families -- 6 hardcoded flags in main.rs, and the EMITTERS table (native/src/emit/mod.rs:17-65) keyed --emit-NAME. ONLY EMITTERS members get knob bridging: the table's third tuple element feeds --knobs, which gate-sdk/bin/run-gates.sh:19-38's --emit front-end resolves through gate_knob_env. A hardcoded flag gets NO config. EmitFn = fn(&[String]) -> Result<String,String>, and its own spec: comment sanctions mode-on-the-argv-tail. Ported members: footprint, close-surfaces, enforcement-map, value-rollup; no shim survived any of them (context-kit/SPEC.md:487-494). Bridged knobs are the SECTION-NAME knobs, not lib/queue.sh's derived regexes (queue-kit/lib/queue.sh:71-87 derives QUEUE_*_RE from QUEUE_KIT_ACTIVE_SECTIONS/_DEFERRED_SECTION/_ICEBOX_SECTION; the ported queue gates bridge exactly those). native/src/queue.rs is 392 lines and already carries the compiled parser. No .gate descriptor couples queue-index.
