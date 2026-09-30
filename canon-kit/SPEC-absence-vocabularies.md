# SPEC amendment: absence-vocabularies

**Assertion G's two phrase lists become consumer-selectable vocabularies.** `check-prose-tells` assertion G (§check-prose-tells) bakes its placeholder tokens and its negative-existential openers into the gate source, so a consumer whose absence idiom is another (`nil`, `tbd`, `no known issues`, a non-English token) meets this repository's English list with no seam. doctrine-kit's Policy-as-choice rule reaches it: G's contract, *a section whose whole body is an absence statement*, stays true under another list. queue-kit's sibling arm already takes its list as `QUEUE_KIT_PLACEHOLDER_TOKENS`.

**Measured at authoring.**

- The two lists are `ABSENCE_PLACEHOLDERS` (`none`, `n/a`, `nothing`, `-`, `—`) and `ABSENCE_OPENERS` (`none`, `nothing`, `there is no`, `there are no`, `not applicable`, `n/a`) at `native/src/gates/prose_tells.rs:345` and `:349`. A third opener, *No* followed by a word, is a separate code arm in `opens_absence` (`:376-378`); a unit test holds that a bare `No` opens nothing (`:691`).
- `CANON_KIT_PROSE_TELL_ABSENCE_GLOBS` binds which files G reads, never which phrases.
- The seven bundled vocabularies with a matching `_EXTRA` are rostered at §Layout and configuration's *A bundled vocabulary is extended, never restated* paragraph, each unioned by `spec::vocabulary` (`native/src/spec.rs:641`).

## What changes

### (1) G's two lists become four knob rows

canon-kit's `native/src/knobs/canon_kit.rs` gains four indexed rows beside `CANON_KIT_PROSE_TELL_ABSENCE_GLOBS` {design-bearing}:

- `CANON_KIT_PROSE_TELL_ABSENCE_TOKENS`, default `none`, `n/a`, `nothing`, `-`, `—`;
- `CANON_KIT_PROSE_TELL_ABSENCE_TOKENS_EXTRA`, default empty;
- `CANON_KIT_PROSE_TELL_ABSENCE_OPENERS`, default `none`, `nothing`, `there is no`, `there are no`, `not applicable`, `n/a`, `no …`;
- `CANON_KIT_PROSE_TELL_ABSENCE_OPENERS_EXTRA`, default empty.

`native/src/gates/prose_tells.rs` reads each pair once per run through `spec::vocabulary`, so the effective set is the base followed by the extra. The two constants are deleted, and so is the `no ` arm of `opens_absence`: it becomes the default member `no …`.

**The opener grammar.** A member matches a sentence that opens with it, compared case-insensitively, when the character after it is no letter or digit. A member ending in a space and `…` matches only when a word follows the part before the `…`: `no …` is the bundled spelling of *No and a word*, so `No changes.` opens an absence and `No-op builds are fast.` and a bare `No` do not. A consumer writes the same shape for another language (`kein …`). The token grammar is unchanged: a body reduced to a lone member, compared case-insensitively.

**Empty is off, per half.** A pair whose base and extra both resolve empty turns that half of G off: no placeholder body, or no one-sentence body, is a finding. Unlike the temporal and authority marker sets, an empty set here is a designed state rather than malformed config, since G is itself opt-in and a consumer may want one shape without the other.

The member's declared knob list in `native/src/gates/mod.rs` (the `check-prose-tells` row, beside `CANON_KIT_PROSE_TELL_ABSENCE_GLOBS`) gains the four names.

At the defaults, the verdict on every input is today's: the default sets are today's lists, and the `no …` member reproduces the removed arm. The fixture pair proves the seam rather than a new verdict:

- `bad/scripts/canon-config.knobs` sets `CANON_KIT_PROSE_TELL_ABSENCE_TOKENS_EXTRA[] = tbd`, and `bad/absence.md` gains a section whose whole body is `TBD`. `bad/expect.txt` pins its `[G]` placeholder finding.
- `good/scripts/canon-config.knobs` sets `CANON_KIT_PROSE_TELL_ABSENCE_OPENERS` to one member, `there is no`. `good/absence.md` gains a section whose one sentence opens `Nothing`, which is clean under that replaced base.

A crate unit test holds the `…` suffix: `no …` matches `No changes.` and not `No-op builds are fast.` or `No`.

### (2) §check-prose-tells and §Layout and configuration state the vocabularies

canon-kit/SPEC.md states the four knobs {mechanical}. **Not yet applied.**

In §check-prose-tells, G's bullet reads:

> - **G. Absence section** — over `CANON_KIT_PROSE_TELL_ABSENCE_GLOBS` alone: a section, a heading of any level to the next, whose whole body is a placeholder token (`CANON_KIT_PROSE_TELL_ABSENCE_TOKENS`) or one sentence opening with a negative-existential opener (`CANON_KIT_PROSE_TELL_ABSENCE_OPENERS`), each merged with its `_EXTRA` (§Layout and configuration). An opener ending in ` …` needs a word after it, so the bundled `no …` reads *No changes* and not a bare *No*. An empty set turns its half off. Generated regions and comment-only lines are held out of the body, and a body of two sentences is clean, its second read as the reason. The surface is the consumer's: bind the hand prose whose sections show their emptiness by the heading alone. Leave out a checklist or declaration grammar whose `None` token a reader needs (an amendment's `## Retired spellings`, a work queue). G mechanizes the decidable shape of doctrine-kit's Absence statements rule.

In §Layout and configuration, the `CANON_KIT_PROSE_TELL_ABSENCE_GLOBS` bullet is followed by:

> - `CANON_KIT_PROSE_TELL_ABSENCE_TOKENS` and `CANON_KIT_PROSE_TELL_ABSENCE_OPENERS` — arrays, each defaulting to a bundled English set (`none`, `n/a`, a dash, …; `none`, `there is no`, `no …`, …; `--emit knob-roster` prints both whole): the lone tokens and the sentence openers assertion G reads as an absence (§check-prose-tells). Empty turns that half of G off.

The *A bundled vocabulary is extended, never restated* paragraph names the two bases in its roster and their extras, `CANON_KIT_PROSE_TELL_ABSENCE_TOKENS_EXTRA` and `CANON_KIT_PROSE_TELL_ABSENCE_OPENERS_EXTRA`, in its extras roster. Its closing sentence, *A temporal or authority marker set whose base and extra are both empty is malformed config*, stays as written: it names the two sets it binds, and these two are not among them.

### (3) The site mirror follows

`docs/canon-kit/SPEC.md` is regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` in the commit that lands delta 2 {mechanical}.

## Producers and consumers

- **The four knob rows.** Producer: canon-kit's knob table, a consumer's `canon-config.knobs` lines, or its gitignored overlay. Enabling configuration: every tree registering `check-prose-tells` with `CANON_KIT_PROSE_TELL_ABSENCE_GLOBS` set, this repository's among them (`scripts/canon-config.knobs:148-155`). Consumer: `check-prose-tells` assertion G alone, read once per run through `spec::vocabulary`. No field is read elsewhere.
- **Roster-holding readers of the new names.**
  - `check-knob-citation` reds a cited knob token no kit table declares; the rows land in the same commit as the SPEC text that cites them (deltas 1 and 2 ride one build commit).
  - `check-knob-default-coupling` couples a default a SPEC states to the table. Delta 2's text states no default literally beyond an elided illustration and names `--emit knob-roster` for the whole set, the precedent `QUEUE_KIT_PLACEHOLDER_TOKENS`' bullet sets.
  - The member's declared knob list is the admissibility roster `check-graph` reads for `knob:` tokens. `canon-kit/checks/check-prose-tells.gate`'s `couples=` gains nothing: the four rows name phrases, not files, and a static knob's file is a derived couple (gate-sdk/SPEC.md §The `# graph:` manifest).
- **The opener grammar.** Producer: the knob value. Consumer: `opens_absence`, at G's per-section judgment. The `…` suffix is read there and nowhere else.
- **No new state, event or interface** beyond the knobs.

## Existing sections updated

Roster produced by `grep -rn 'ABSENCE_PLACEHOLDERS\|ABSENCE_OPENERS\|PROSE_TELL_ABSENCE' --include='*.rs' --include='*.md' --include='*.knobs' --include='*.gate' .` over the tracked tree at authoring, and reading §Layout and configuration's vocabulary paragraph.

- `canon-kit/SPEC.md` — §check-prose-tells G's bullet; §Layout and configuration's absence bullet and the bundled-vocabulary roster (delta 2).
- `native/src/knobs/canon_kit.rs` — the four rows (delta 1).
- `native/src/gates/prose_tells.rs` — the constants and the `no ` arm replaced by the knob reads and the `…` grammar; the unit test at `:691` rewritten to the member (delta 1).
- `native/src/gates/mod.rs` — `check-prose-tells`' declared knob list (delta 1).
- `canon-kit/gate-tests/check-prose-tells/` — both cases' `scripts/canon-config.knobs`, `absence.md` and `bad/expect.txt` (delta 1).
- `docs/canon-kit/SPEC.md` — the regenerated mirror (delta 3).
- `TASK-QUEUE.md` — the paired entry's body names both retired constants, and the entry's Done move at merge drops it (delta 1).

## Retired spellings

- `ABSENCE_PLACEHOLDERS` — the baked token constant, replaced by `CANON_KIT_PROSE_TELL_ABSENCE_TOKENS` (delta 1).
- `ABSENCE_OPENERS` — the baked opener constant, replaced by `CANON_KIT_PROSE_TELL_ABSENCE_OPENERS` (delta 1).

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls canon-kit/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
