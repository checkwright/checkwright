# SPEC amendment: grant-spend-mark

Pairs `consult-grant-spend-unmarked`. A consult-inbox item may carry a grant the operator gave through the lead, such as one outside-critique run, which a dispatched consultation spends on the item's own terms (§templates/consult.md). When the item stays after the spend, a ruling to reconsider waiting for the operator say, its bullet is unchanged. The spend survives only in the spending commit's message and the consultation's gitignored journal. The lead template already tells the lead to read a grant's spent state off its target's current state at relay time (`templates/lead.md`, *Relay, never assert*), and for an inbox item that state shows nothing. One lead re-dispatched such an item on a grant it inferred unspent, and the consultation bought an expert-tier read to rediscover the spend.

**The chosen form: a spend mark on the item's own line, written by the spending session.** It gives the lead's existing read a target that carries the fact, so the read needs no new step.

**The refused alternative: a lead step reading the item's history.** The spending commit need not touch the inbox. In the attested case it removed two other bullets and left this one byte-identical, so `git log` over the inbox finds the item's filing and not its spend. A history read would have to search commit messages for an item's prose, which is no stable key. **The other refused alternative: a second bullet recording the spend.** `--emit consult-count` counts column-0 `- ` lines, so a spend bullet would read as one more item owed.

## What changes

### (1) The consult inbox admits a spend mark on a staying item {design-bearing} {user-facing: consult-grant-spend-unmarked's Deliverable, the marker-or-lead-step form it leaves to this stage}

Replacement for §The consult inbox, the paragraph beginning **The surface.**, its first sentence, and a new paragraph after the paragraph beginning **Ruling on an item**. **Not yet applied.**

> **The surface.** `.workflow/consult-items.md` (knob `LIFECYCLE_KIT_CONSULT_INBOX_FILE`, §Layout and configuration) is committed and append-only, save the spend mark below, with the gap inbox's grammar: a `# contract:` header, then one `- <YYYY-MM-DD> — <prose>` bullet per item.

> **A grant an item carries is marked when it is spent and the item stays.** The session spending it appends `Grant spent <YYYY-MM-DD>: <what it paid for>.` to the end of the item's bullet line, in the commit that lands the spend's result, one mark per grant spent. The line stays one bullet, so the count is unchanged and the item keeps its filing date. The lead reads the mark when it relays the item's grant (§templates/lead.md), and a dispatched consultation reads it before spending. An item leaving the inbox takes its mark with it, so a mark only ever qualifies an item still owed. **Honest limit:** no gate reads the mark, so a spend left unmarked reads as unspent, as before. Under `merge=union`, a mark landing beside a concurrent drain of the same bullet resurrects it, the hazard any edit to a drained line already carries.

Replacement for §templates/consult.md, the last sentence of the paragraph beginning **A consultation may commission an outside critique**. **Not yet applied.**

> The exception is a critique an inbox item carries as an operator grant the lead relays, spent on the item's own terms and marked spent on the item's line when the item stays (§The consult inbox), since a grant travels in the dispatch prompt of the session that spends it (§The steering vocabulary).

### (2) The consult template marks a spend on a staying item {mechanical}

Replacement for `lifecycle-kit/templates/consult.md`, the paragraph beginning **Dispatched by a lead, with no operator in the session**, its first sentence. **Not yet applied.**

> **Dispatched by a lead, with no operator in the session**, take up consult-inbox items only and run no critique, except one an item carries as an operator grant the lead relays, which you spend on the item's own terms: read the item's line for a `Grant spent` mark first, and where the item stays after you spend it, append `Grant spent <YYYY-MM-DD>: <what it paid for>.` to its line in the commit landing the spend's result (lifecycle-kit/SPEC.md §The consult inbox).

Replacement for `lifecycle-kit/templates/consult.md`, step 2's last sentence. **Not yet applied.**

> An item the operator defers stays in the inbox; where you spent a grant it carries, mark the spend on its line as the dispatched paragraph below says.

### (3) The lead reads an inbox item's grant off its spend mark {mechanical}

Replacement for `lifecycle-kit/templates/lead.md`, the last sentence of the bullet beginning **A claim carries its tier**. **Not yet applied.**

> A grant also carries whether it is spent, read off its target's current state at relay time; an inbox item's grant is spent where the item's line carries a `Grant spent` mark for it (lifecycle-kit/SPEC.md §The consult inbox).

## Producers and consumers

- **The spend mark (delta 1).**
  - Producer: the session spending a grant an inbox item carries while the item stays. That is a lead-dispatched consultation (delta 2), or an operator-started one spending a grant on an item the operator then defers. No enabling config: the mark is grammar inside the bullet prose of the file `LIFECYCLE_KIT_CONSULT_INBOX_FILE` names.
  - Consumers: the lead, at relay time (delta 3); a dispatched consultation, before it spends (delta 2).
  - Readers of the inbox that must leave the mark alone, from `grep -rn "LIFECYCLE_KIT_CONSULT_INBOX_FILE" native/src --include=*.rs`: `emit/consult_count.rs` counts column-0 `- ` lines, so a mark inside a bullet line changes no count or date; `emit/file_consult.rs` appends whole bullets and never reads one; `gates/kit_ref_liveness.rs` skips the file; `gates/dispatch_entry.rs`, `stages.rs` and `gates/merge_attrs.rs` read its path alone. No grammar gate reads the consult inbox (§The consult inbox, its honest limit).
  - Fields: the date, read by the lead to place the spend against the item's later history; *what it paid for*, read by the lead to relay which grant is spent where an item carries more than one.
- **Point 5:** no corpus narrows. **No member corpus is obliged**, so point 6 adds nothing.

## Existing sections updated

The roster below came from `grep -n "grant\|stays\|append-only" lifecycle-kit/SPEC.md lifecycle-kit/templates/consult.md lifecycle-kit/templates/lead.md` over the tracked tree.

- `lifecycle-kit/SPEC.md` §The consult inbox and §templates/consult.md (delta 1).
- `lifecycle-kit/templates/consult.md`, step 2 and the dispatched paragraph (delta 2).
- `lifecycle-kit/templates/lead.md`, *Relay, never assert* (delta 3).
- `docs/lifecycle-kit/SPEC.md`, the generated on-site mirror (all deltas).

## Retired spellings

- None — no delta retires a spelling; the inbox keeps its grammar and the mark is a new sentence inside it.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Done move** — the entry moves to Done in the commit that merges this amendment, before the drain stage.
