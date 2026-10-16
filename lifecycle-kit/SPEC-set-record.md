# SPEC amendment: set-record

Pairs `set-record-lost-at-done-move`. A record that rules the promoted unit set as a whole — the ruling party's direction on the set, a grant spanning it, a push no one entry's landing spends — has no home in the queue grammar, so the scope stage lands it on one entry's body, and the other entries cite that entry. The `done` verb removes the body and rewrites each citing link to the retired citation (queue-kit/SPEC.md §The queue verbs), so the record is gone from the tree while later stages still need it.

**The trace the design rests on**, read off this repo's history: one iteration's scope landed the set's direction and its push need on the lead entry, and two sibling entries cited it as the record. The first build batch moved the lead entry to Done in the commit landing its fix, and the verb turned both citations into backticked slugs pointing at nothing. The second batch re-landed both records as indented paragraphs under the iteration header, outside every entry, and the close removed them by hand. This amendment names that place and gives it a writer and a lifetime.

**The refused alternative: a Done move refusing while a live entry cites the body it removes.** A citing link cannot tell a record pointer from a relation: an owner-lookup line or a "one of them" reference reads the same, so the refusal would fire on ordinary citations and turn the verb's mechanical rewrite into a manual one. It also gives the record no home: the session relocates it onto a sibling entry, and the last entry's Done move, before the drain stage, still removes a record the closing stage reads. A home outside every entry, with a lifetime the boundary ends, removes the loss instead of detecting one shape of it.

## What changes

### (1) The set record: a block under the iteration header, written by the stage that bounds the set {design-bearing} {user-facing: set-record-lost-at-done-move's Deliverable, its first form, the form left to the authoring stage by the operator direction of 2026-10-16}

The queue file's iteration header gains a companion block, the **set record**. It opens at a line under the header whose text, indentation trimmed, is exactly `**Set record**`, and runs to the line before the next `---` rule or `## ` heading. It is the header region's last block, so any preamble the consumer keeps under the header sits above the opening line. Every line of it is indented, the shape the queue hygiene gate already requires of prose outside an entry. Each record is one paragraph opening with a bold label carrying its date and class inline, the stamp protocol's form: a set-level push takes the existing `**Push need (<date>, <grant>):**` spelling, and any other record names what it rules.

- **Writer.** The first stage writes it in the commit that promotes the set, and a later stage session landing an answer that rules the whole set lands it here (§The stamp protocol). An entry's body never carries a set-wide record.
- **Lifetime.** No queue verb moves or removes it, so it outlives every Done move, and the next iteration boundary drops it (delta 2). Between a close and the next boundary it still sits under the closed iteration's name, so it never reads as the next set's.
- **Readers.** The sessions that spend what it rules: the lead at batch-cut and before a dispatch that spends a grant, and a pushing session reading its push line (deltas 3 and 4). They read it with the queue, as they read the header line.

Replacement for §The state machine, the header-line bullet. **Not yet applied.**

> - **The header line**, at the top of the consumer's queue file (default `TASK-QUEUE.md`), carrying the *slow* axis — the iteration name, and nothing else:
>
>   ```
>   ## Iteration: <name>
>   ```
>
>   Below it sits the iteration's **set record**, the home of every record that rules the promoted set as a whole (below).

New paragraph after §The state machine's paragraph beginning **The header carries no stage field**. **Not yet applied.**

> **The set record holds what rules the whole unit set**: the ruling party's direction on the set, a grant spanning it, a push no one entry's landing spends. An entry's body leaves the tree at its Done move, which can come before the record's last reader, so a set-wide record never rides an entry. The block opens at a line reading `**Set record**`, indentation trimmed, and runs to the next `---` rule or `## ` heading, the header region's last block; its lines are indented, as all prose outside an entry is, and each record is one paragraph whose bold label carries its date and class (§The stamp protocol). The first stage writes it with the promotion, and a later stage session landing an answer that rules the whole set lands it here. No queue verb moves or removes it, so it outlives every Done move, and the iteration boundary drops it (§The iteration boundary). Its readers are the sessions that spend what it rules, the lead at batch-cut and dispatch and a pushing session. **The honest limit:** whether a record rules the set or one entry is semantic, so a set-wide record written on an entry's body reds nowhere and is lost at that entry's Done move as before.

### (2) The iteration boundary drops the set record {design-bearing} {user-facing: set-record-lost-at-done-move's Deliverable, its first form, the form left to the authoring stage by the operator direction of 2026-10-16}

The boundary reset's queue rewrite, the `rewrite_header` call on the first-stage path of `native/src/emit/enter_stage.rs`, also removes the set record: the opening line through the line before the next `---` rule or `## ` heading, and the blank lines directly above the opening line. The `--rename` path keeps the block, since naming happens inside the iteration that owns it. A queue with no opening line under the header is rewritten as today. The pre-flight hand-off already passes the rewritten queue as the temp queue, so `check-stage-entry` reads the queue without the block. The reset's report line names the drop where it made one.

Owed with it: crate cases in `enter_stage.rs` for a block ended by `---`, a block ended by `## `, a block below a consumer preamble that stays, a queue with none, and `--rename` leaving it in place; and one scenario in the boundary harnesses of `lifecycle-kit/gate-tests/` running a real boundary entry over a queue carrying a set record.

Replacement for §The iteration boundary, the first sentence of its first paragraph. **Not yet applied.**

> **The reset truncates the state file and every `LIFECYCLE_KIT_BOUNDARY_TRUNCATE` member back to its contract header**, restarts the queue header at the unnamed-iteration form and drops the closing iteration's set record (§The state machine).

### (3) The scope template lands set-wide records in the set record {mechanical}

Replacement for `lifecycle-kit/templates/stages/scope.md`, the last sentence of the paragraph beginning **Count the set's pushes before it is ruled**. **Not yet applied.**

> Ruling the set rules its need: on each push-needing entry, land `**Push need (<date>, <grant>):** <the push it spends and why>`, where `<grant>` is `inside the budget` or the party that ruled the overrun. A push no one entry's landing spends, and every other record ruling the set as a whole — the ruling on the set, a grant spanning it — lands in the set record under the iteration header, never on an entry (lifecycle-kit/SPEC.md §The state machine).

Replacement for §The scope template, the paragraph beginning **A unit set's push need is counted**, its next-to-last sentence pair. **Not yet applied.**

> Ruling the set rules the need, so an overrun costs no second interrupt. The ruling lands on each push-needing entry, because the landing session reads its entry; the closing push takes none, because every count includes it, and a push no one entry's landing spends lands in the set record, which outlives the entries' Done moves (§The state machine).

### (4) The build template and the lead template read the set record {mechanical}

Replacement for `lifecycle-kit/templates/stages/build.md`, the last sentence of the paragraph beginning **Run the system; don't reason about it**. **Not yet applied.**

> A push no such line names, on the entry or in the set record under the iteration header, changes the set's push need: escalate it before spending it.

Replacement for `lifecycle-kit/templates/lead.md`, the parenthetical in the first sentence of the paragraph beginning **An answer that amounts to a design ruling**. **Not yet applied.**

> (the amendment, the queue entry, or the set record for an answer ruling the whole set)

## Producers and consumers

- **The set record (delta 1).**
  - Producer: the first stage's promoting commit, instructed by the scope template (delta 3), and a stage session landing a set-wide answer (§The stamp protocol). No enabling config: the block is grammar on the queue file `LIFECYCLE_KIT_QUEUE_FILE` names.
  - Consumers: the lead, at batch-cut and before a grant-spending dispatch (delta 4); a pushing session, through the build template's push read (delta 4); the boundary reset, which drops it (delta 2).
  - Readers of the region that must leave it alone, from `grep -rn "Iteration:" native/src --include=*.rs`: `evidence.rs`, `stages.rs`, `hook/statusline.rs`, `emit/queue_index.rs`, `emit/enter_stage.rs` and `installer/demo.rs` each read the one header line and nothing below it. The queue gates read the region only on the hygiene axes; an indented line passes the column-0 prose axis, and `--queue done` rewrites a live link anywhere in the file, the set record included, to the retired citation, which is the right act for a record citing a landed entry. No native reader of a `**Push need` line exists (`grep -rn "Push need" native/src`); its readers are the three templates `grep -rln "Push need" lifecycle-kit/templates` returns.
  - Fields: the opening line, read by the boundary reset; each record's label, date and class, read by the spending session to know what was ruled and by whom.
- **The drop (delta 2).**
  - Producer: `--enter-stage` at the first stage, on every boundary.
  - Consumer: the queue file at the boundary commit, and `check-stage-entry` through the pre-flight's temp queue. The stamp-commit purity assertion already exempts the queue on the boundary path (§check-stage-evidence), so the drop rides the exemption the header rewrite uses.
  - Point 5: the drop narrows the queue. No reader asserts a count or a non-empty match over the region; `check-queue-sections` asserts the `Iteration:` heading's presence, which the drop never touches.
- **No member corpus is obliged**, so point 6 adds nothing.

## Existing sections updated

The roster below came from `grep -n "Push need\|header line\|unnamed-iteration form" lifecycle-kit/SPEC.md lifecycle-kit/templates/*.md lifecycle-kit/templates/stages/*.md` and `grep -n "rewrite_header" native/src/emit/enter_stage.rs` over the tracked tree.

- `lifecycle-kit/SPEC.md` §The state machine, the header-line bullet and a new paragraph (delta 1).
- `lifecycle-kit/SPEC.md` §The iteration boundary, its first sentence (delta 2).
- `native/src/emit/enter_stage.rs`, the first-stage path's queue rewrite and its crate cases (delta 2).
- `lifecycle-kit/gate-tests/`, one boundary scenario (delta 2).
- `lifecycle-kit/templates/stages/scope.md`, the push-count paragraph (delta 3).
- `lifecycle-kit/SPEC.md` §The scope template, the push-need ground (delta 3).
- `lifecycle-kit/templates/stages/build.md`, the remote-oracle push sentence (delta 4).
- `lifecycle-kit/templates/lead.md`, the design-ruling landing paragraph (delta 4).
- `docs/lifecycle-kit/SPEC.md`, the generated on-site mirror (all deltas).

## Retired spellings

- None — no delta retires a spelling; the header line, its unnamed form and the push-need label keep their names.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Done move** — the entry moves to Done in the commit that merges this amendment, before the drain stage.
