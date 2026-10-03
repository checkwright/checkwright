# SPEC amendment: fan-width

**The read-only fan-out bound stays discipline, and the surfaces a dispatcher reads say so.** The template binds independent read-only units to `≤DELEGATION_KIT_FAN_WIDTH`-wide. `usage-verdict` reports the knob as its `width=` field at every dispatch, and `agent-budget-guard` blocks only on PAUSE. No rule of `agent-dispatch-guard` counts what is in flight. A close session dispatched three concurrent read-only audits under `width=2`, and nothing it read told it that nothing would stop it.

**A dispatch-guard width rule was weighed and refused, on three measured premises.**

- **The payload carries no in-flight set.** guard-kit/SPEC.md §The reader and its views records the measured `PreToolUse` top-level keys, and none enumerates running tasks. The harness's published hook contract, fetched at authoring, lists none for `PreToolUse` either. The live-children view §What `background_tasks` carries measured arrives on `SubagentStop`, a turn end, after the dispatch it would have had to refuse.
- **The attested overrun was one message.** The close session's three audit dispatches were three tool calls of a single assistant message, by the transcript's shared message id. A rule must therefore see same-message siblings to catch the one shape that fired.
- **The one record that could show them does not hold them yet.** The transcript is the only surface a `PreToolUse` hook can reach that names earlier dispatches, through the payload's `transcript_path`. A tool call's own record was absent from its session's transcript while the call ran, in one reading taken at authoring. So a transcript-counting rule would see wide fan-outs spread over several messages and miss the single-message burst. That is the shape guard-kit refused for a backgrounding rule reading only the `&` arm: built, it blocks the form nobody uses and passes the one that fires.

**A ledger the guard writes was refused too.** A hook appending each read-only dispatch to a scratch file would see same-message siblings. But it makes a verdict surface the owner of state it must also expire. A dispatch that never notifies would hold a slot forever, and §The delegation model already refuses a guard owning lifecycle (*The dispatch guard reaps nothing*). It also refuses a per-class state file as the heavier shape, at D2's resume.

## What changes

### (1) §The delegation model states that the width bound is unenforced, and why

The paragraph opening "The template's **Serialize on shared files; ≤`DELEGATION_KIT_FAN_WIDTH`-wide otherwise** rule caps an *unlocked* fan-out at read-only work." is replaced {mechanical}. **Not yet applied.**

> The template's **Serialize on shared files; ≤`DELEGATION_KIT_FAN_WIDTH`-wide otherwise** rule caps an *unlocked* fan-out at read-only work. The git index and HEAD are shared by every committing agent whatever its source files, so the cap is a correctness bound. A consumer may never configure it up for committing agents (the knob's derivation: §Layout and configuration).
>
> **The width bound is discipline, surfaced and never enforced.** `usage-verdict` reports it at every dispatch (§usage-verdict, the `width=<n>` field), and no guard rule counts what is in flight. A `PreToolUse` payload carries no in-flight set (guard-kit/SPEC.md §The reader and its views records its measured keys), and the harness's live-children view arrives only at `SubagentStop`. The transcript is the one reachable record of earlier dispatches, and it does not yet hold a call's own message while that call runs, in one reading. The attested overrun was three dispatches in one message, so a transcript-counting rule would miss the shape that fired. A guard-written ledger of dispatches would see it, but it makes a verdict surface own state it must expire, which *The dispatch guard reaps nothing* refuses.

### (2) §usage-verdict's width paragraph names what the field does not do

In the paragraph opening "**The `width=<n>` field.**", the sentence "It is the knob's mechanical reader." is replaced {mechanical}. **Not yet applied.**

> It is the knob's mechanical reader, and it reports the bound without enforcing it (§The delegation model).

§Layout and configuration's `DELEGATION_KIT_FAN_WIDTH` bullet closes on the same phrase. Its "the knob's mechanical reader" becomes "the knob's mechanical reader, which reports the bound and enforces nothing (§The delegation model)".

### (3) The template tells the dispatcher that the bound is its own to hold

delegation-kit/templates/agent-execution.md's **Serialize on shared files; ≤`DELEGATION_KIT_FAN_WIDTH`-wide otherwise** bullet: the sentence "Independent read-only units may run ≤`DELEGATION_KIT_FAN_WIDTH`-wide." is replaced {mechanical} {user-facing: the entry's deliverable, whose second branch, the bound stated as discipline the guard only surfaces, the entry leaves to spec; the unit selected by operator direction 2026-10-03; the wording is spec's calibration}. **Not yet applied.**

> Independent read-only units may run ≤`DELEGATION_KIT_FAN_WIDTH`-wide, counting every dispatch still in flight, those you send in one message included. No guard counts them: the budget check's `width=` field reports the bound and nothing refuses a dispatch past it.

The bullet's lead-in is unchanged, so `check-rule-citation`'s resolution of §The delegation model's citation of it holds.

## Producers and consumers

- **The unenforced-bound statement (deltas 1 and 3).** Producer: the template bullet, loaded at `/agent-execution` by every dispatching role. Consumer: the dispatcher sizing a wave, at the dispatch decision. The `width=` field it names is produced by `usage-verdict` and relayed by `agent-budget-guard` at every dispatch, so the bound and the statement reach the same reader at the same transition.
- **No new state, event, field or knob.** The deltas change prose only. `DELEGATION_KIT_FAN_WIDTH` keeps its readers: `native/src/hook/verdict.rs`, `native/src/knobs/delegation_kit.rs`'s table and validator, and `native/src/usage_tests.rs`' width cases (`git grep -n FAN_WIDTH -- ':!docs/'`).
- **The citation's reader.** `check-rule-citation` resolves the template-rule citations in §The delegation model, and delta 1 keeps the cited lead-in verbatim.

## Existing sections updated

- `delegation-kit/SPEC.md` §The delegation model (delta 1), §usage-verdict and §Layout and configuration's `DELEGATION_KIT_FAN_WIDTH` bullet (delta 2).
- `delegation-kit/templates/agent-execution.md`, the **Serialize on shared files; ≤`DELEGATION_KIT_FAN_WIDTH`-wide otherwise** bullet (delta 3).
- `docs/delegation-kit/SPEC.md`, the generated mirror (all deltas).

The roster came from `git grep -n 'FAN_WIDTH\|fan-width' -- ':!docs/' ':!TASK-QUEUE.md'` over the tracked tree, and from `grep -n 'Serialize on shared files' .claude/agents/*.md`, which finds one citer, `.claude/agents/edit-sweep.md`, naming the lead-in delta 3 keeps; no carrier restates the bullet.

## Retired spellings

- None — the amendment rewrites prose and renames nothing.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds; the deltas add no state, event, interface or obligation beyond the stated one.
- [ ] **Instruction surfaces: instruction only** — the template sentence carries the instruction; the refused rule's grounds sit in §The delegation model.
- [ ] **Merged with no information lost** — the merged §The delegation model reads as one document.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls delegation-kit/SPEC-*.md`).
- [ ] **Removals propagated** — nothing retired.
- [ ] **Gaps filed** — a cross-component gap found during the work filed with `--emit file-gap`.
- [ ] **Done move** — `fan-width-unenforced` moves to Done in the landing commit, a stage before the drain stage, with `--queue done`.
