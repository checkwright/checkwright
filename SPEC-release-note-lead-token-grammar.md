# SPEC amendment: release-note-lead-token-grammar

The release note's Tightened-gates lead-token grammar and the parser that
consumes it disagree, so a bolded declaration compiles to an empty allowed-red
set without saying so. This amendment gives the lead token one canonical
spelling, gives the grammar one implementation, and makes a declaration that
fails to parse loud rather than empty.

A **root-level amendment**: the ruling has no single owning component. The
grammar is owned by a docs page, the parser by gate-sdk, and the corpus
assertion by this repo's own gates.

## What changes

### Delta 1 — one canonical spelling for the Tightened-gates lead token {design-bearing}

The lead token of a **Tightened-gates** bullet is a **backticked, unbolded** bare
gate name. docs/install.md §The upgrade contract owns that rule, stated in the
Tightened-gates paragraph that already declares the section machine-readable.

Backticks rather than bold because this token is a code identifier — a gate
name — and this tree spells identifiers in backticks everywhere else. Bold is a
rendering choice carrying no semantics, and it collides with the one token a
machine reads.

**The rule reaches this section and no other, and the corpus is what decides
that.** The Behavior-changes paragraph currently claims its lead tokens are
"bolded like the other sections' lead tokens" — a cross-section uniformity claim
that is false about Tightened gates the moment this delta lands, so the clause is
rewritten to stop asserting it. What replaces it is *not* the canonical spelling:
an audit of every shipped Behavior-changes bullet finds lead tokens that are
legitimately prose phrases rather than identifiers — `**stage cursor**`,
`**same-stage re-entry**`, `**scope boundary scratch wipe**`, `**Release
assets**` — and backticking those would be a prose rewrite, not a normalization.
That section keeps its own spelling; only the false uniformity claim goes.

**Renamed knobs is left unruled, deliberately.** Every shipped note declares it
`None`, so there is no corpus to normalize and no reader to protect. Legislating
a spelling over an empty set would be a rule with no attested drift axis, which
is the trivially-true shape gate-sdk/SPEC.md §When a gate earns its place bars.

**Honest limit, stated where the rule is.** Only Tightened gates is gated (delta
4), because it is the only section a machine parses — exactly parallel to that
page's existing honest limit on Behavior-changes bullets being declared for
reading rather than smoke-asserted.

### Delta 2 — the parser refuses a silently-empty declaration {design-bearing}

The failure this unit exists to kill is not the spelling. It is that a non-`none`
Tightened-gates section yielding zero tokens becomes "no allowed reds"
**silently**, disarming the one assertion the upgrade contract calls mechanical.

New invariant, independent of any spelling: a Tightened-gates section resolves to
either an **explicit empty set** (a `None` body) or a **non-empty token set**. A
non-`none` section that yields no tokens is a fail — exit 1 from the upgrade
smoke, naming the note and the lines that did not parse.

This is the arm that closes the class permanently. No future markup variant — an
italicized token, a linked one, a prose paragraph where a bullet list belongs —
can disarm the assertion any more; the worst it can do is red it.

### Delta 3 — one implementation of the grammar, two callers {design-bearing}

`gate-sdk/lib/` gains a helper that, given a note path and a section name,
reports the trichotomy delta 2 defines: explicit-`None`, a lead-token list, or
unparsed-and-not-`None`. `bin/upgrade-smoke.sh` calls it in place of its inline
`sed` expression, and delta 4's gate is its second caller.

gate-sdk/SPEC.md §check-action-run-shell states the standing rule this satisfies:
an inline extractor earns a `lib/` helper at a second consumer. There was none
before; there is one now. The rule is not decoration here — this unit exists
*because* two statements of the grammar disagreed, and landing a fix that leaves
three statements would file the same defect forward.

**Two container arms, one token predicate.**
`SPEC-upgrade-smoke-note-resolution.md` adds a second surface the same allowed-red
set is read from — a `.workflow/` declaration file of bare gate names, written
during the iteration. The helper therefore carries two *container* arms (a
markdown section's bullet lead tokens; a record file's data lines) over **one**
token predicate, and the container is the only thing that differs between them.
That is what keeps the two surfaces from re-opening this unit's own defect from a
new direction: the declaration file is deliberately markup-free, so the spelling
question does not arise on it at all, and close's composition step is the one
place a bare declared name becomes a backticked note bullet — an output this
amendment's delta 1 governs and delta 4 then holds.

### Delta 4 — a consumer gate holds the corpus {design-bearing}

A new gate in this repo's `scripts/` asserts, over every note under
`docs/posts/`, that its Tightened-gates section satisfies delta 2's trichotomy
under delta 1's grammar. It sits beside `check-release-bump`, which already
reads the same corpus and whose `# graph:` manifest already couples
`docs/posts/*.md` and `docs/install.md`.

**A consumer gate, not kit mechanism.** No kit ships a release note; the corpus
is this repo's own release record and the gate names live in this repo's
`gates.list`. What gate-sdk contributes is the grammar's implementation (delta
3), which is mechanism — a lead-token extractor over a named markdown section,
carrying no section name and no gate name of its own, both supplied by the
caller. The seam falls exactly there: the parsing is kit, the parsed content is
the consumer's.

**The invariant is parseability, corpus-wide, with no version cutoff.**
Registry membership — asserting each token names a live `gates.list` member — is
deliberately excluded: a gate renamed or retired since a note shipped would make
membership false about history without the record being wrong. Parseability is a
genuine corpus-wide invariant, and the reason it is genuine rather than tidy is
that `GATE_SDK_UPGRADE_FROM` and `GATE_SDK_UPGRADE_TO` make any historical pair a
supported run: any note in the corpus may be the one the smoke resolves, so every
note's declaration must stay machine-readable indefinitely.

### Delta 5 — the corpus is repaired to the canonical spelling {mechanical}

**The repair is scoped to the Tightened-gates section, and that scoping is
load-bearing rather than incidental.** A bolded-lead-token sweep run corpus-wide
would reach Behavior-changes bullets, whose lead tokens are legitimately prose
phrases (delta 1); normalizing those would be a prose rewrite of published posts
and would falsify this delta's whole warrant, which is that no claim changes. The
sweep is section-scoped or it is wrong.

Within that section the audit at authoring found 12 of the 22 non-`none` bullets
unreadable by the parser: eight bolded (`v0.4.0`, `v0.5.0`, `v0.7.0`, `v0.10.0`
with four, `v0.17.0`) and four bold-and-backticked (`v0.13.0` with three,
`v0.15.0`). Every lead token there is a bare gate name — no paths, no links, no
multi-token leads — so the repair strips the bold emphasis, leaves or adds the
backticks, and changes no claim any note makes.

The repair set is **derived by running delta 4's gate**, never transcribed from
this paragraph into the fix; the figures above are dated evidence that the repair
is a markup sweep rather than a rewrite, not a worklist to work from.

### The immutability question, ruled {design-bearing}

docs/site-architecture.md §Page-authoring rules calls dated `docs/posts/`
immutable, so delta 5 owes its authority rather than assuming it. That rule's own
structure supplies it. Posts are immutable **as prose** — exempt from the
anti-restatement and temporal-narration doctrines, not maintained against SPEC
drift — "but still link/command-resolved": the mechanically-held elements of a
post are repaired when they break, which is why a rotted link in a shipped note
is fixed rather than preserved.

A Tightened-gates lead token is such an element. docs/install.md declares the
section machine-readable — a mechanical consumer reads these lead tokens as the
release's allowed-red set — so a section that is *not* machine-readable is a
**defect in the shipped artifact**, not a record of what was said. Delta 5 is
repair in the class the rule already admits.

**The governance sentence is deliberately not amended.** Widening
§Page-authoring rules to name machine-read elements generally was offered and
declined, so the carve-out stays stated as it is and this reading stays local to
this amendment. The consequence is recorded honestly: the tension between
docs/install.md declaring the section machine-readable and
docs/site-architecture.md calling the posts immutable remains unrecorded on
either governed surface, and is filed rather than resolved here.

**Ruled out: widening the parser to strip bold.** It leaves the corpus untouched,
which is its whole appeal, and it fails on two counts. It blesses four spellings
(bare, backticked, bolded, bold-and-backticked) with no canonical one, so the
next variant reopens the class instead of closing it — leniency moves the
boundary rather than removing it. And it leaves delta 4's gate with no honest
predicate: either it carries a version cutoff below which the strict grammar does
not apply, which is a drift-bearing moving literal in a gate, or it gates nothing
but leniency, which returns the canonical spelling to convention and fails this
unit's deliverable outright.

## Producers and consumers

- **The lead-token grammar** — produced (stated) by docs/install.md §The upgrade
  contract. Consumed by the release session authoring a note, by the delta-3
  helper as its implementation, and by the delta-4 gate as its predicate. One
  statement, three readers, which is the condition that was violated.
- **The helper's trichotomy verdict** — produced by the `gate-sdk/lib/` helper.
  Consumed at two named transitions: by `bin/upgrade-smoke.sh` at its
  declaration-resolve step (for the containment assertion and delta 2's refusal),
  and by the delta-4 gate at each note it walks. Both readers named; neither
  verdict arm is unread — `None` drives the empty allowed set, a token list
  drives the containment subset check, and the unparsed arm drives the refusal.
- **The new gate's verdict** — produced by the gate under the battery; consumed
  by the committing author through the generated pre-commit hook and by CI's
  required check. Its `# graph:` manifest is consumed by `gen-pre-commit.sh` and
  `check-graph.sh`, the two generated readers every registered gate has.
- **No new knob.** The helper takes the note path and the section name as
  arguments, so no configuration surface is added on either side of the seam.

## Existing sections updated

- **docs/install.md §The upgrade contract** — the Tightened-gates paragraph gains
  the canonical spelling (delta 1) and the trichotomy (delta 2), and names the new
  gate where the section's mechanical character is claimed (delta 4); the
  Behavior-changes paragraph loses its cross-section uniformity clause without
  gaining a spelling rule of its own (delta 1).
- **gate-sdk/SPEC.md §upgrade-smoke** — the sentence describing the note as
  "parsed for the bullet lead tokens docs/install.md owns" gains the helper and
  the refusal (deltas 2-3).
- **gate-sdk/SPEC.md, the `lib/` roster** — the new helper's contract, including
  its two container arms over one token predicate (delta 3).
- **docs/site-architecture.md §Page-authoring rules — deliberately unchanged.**
  Widening its immutability carve-out was offered and declined; the repair
  proceeds on the rule as written. Named here so a build session does not amend
  the governance surface on this amendment's authority.
- **`scripts/gates.list`** and the generated projections the new gate moves —
  each freshness gate names its own regen command on red, and the fan-out is
  owned by docs/site-architecture.md §Generated projections and their freshness
  gates.
- **RELEASING.md §The procedure step 1 — deliberately unchanged.** It already
  cites docs/install.md as the grammar's owner and must keep citing rather than
  restate the canonical spelling; a second statement of the grammar in the
  runbook is the defect this unit is closing, one surface over. Named here so the
  build session does not helpfully add one.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the
      repo root (`ls SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
