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

### Delta 1 — one canonical spelling, owned by docs/install.md {design-bearing}

The lead token of a bullet in each of the three fixed sections is a
**backticked, unbolded** bare name. docs/install.md §The upgrade contract owns
that rule; its Behavior-changes paragraph currently prescribes the opposite —
lead tokens "bolded like the other sections' lead tokens" — which is both false
of two of the three sections and, for Tightened gates, the one spelling the
parser cannot read. That clause is replaced by the canonical spelling, stated
once for all three sections.

Backticks rather than bold because the lead token is a code identifier in every
section: a gate name under Tightened gates, a knob name under Renamed knobs, a
script/knob/template/file name under Behavior changes. This tree spells
identifiers in backticks everywhere else, and bold is a rendering choice that
carries no semantics while colliding with the one token a machine reads.

**Honest limit, stated where the rule is.** Only the Tightened-gates section is
gated (delta 4) — it is the only one a machine parses. The other two sections'
spelling is a convention docs/install.md states and review holds, exactly
parallel to that page's existing honest limit on Behavior-changes bullets being
declared for reading rather than smoke-asserted.

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

The audit at authoring found 12 of the 22 non-`none` Tightened-gates bullets
unreadable by the parser: eight bolded (`v0.4.0`, `v0.5.0`, `v0.7.0`, `v0.10.0`
with four, `v0.17.0`) and four bold-and-backticked (`v0.13.0` with three,
`v0.15.0`). Every lead token in the corpus is a bare gate name — no paths, no
links, no multi-token leads — so the repair strips the bold emphasis, leaves or
adds the backticks, and changes no claim any note makes.

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

A Tightened-gates lead token is a third mechanically-held element. docs/install.md
declares the section machine-readable — a mechanical consumer reads these lead
tokens as the release's allowed-red set — so a section that is *not*
machine-readable is a **defect in the shipped artifact**, not a record of what
was said. Delta 5 is repair in the class the rule already admits, and
§Page-authoring rules gains the lead token as its third named member so the next
reader does not have to re-derive this.

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

- **docs/install.md §The upgrade contract** — the Behavior-changes bolding clause
  and the canonical spelling for all three sections (delta 1); the
  Tightened-gates paragraph gaining the trichotomy (delta 2); the new gate named
  where the section's mechanical character is claimed (delta 4).
- **gate-sdk/SPEC.md §upgrade-smoke** — the sentence describing the note as
  "parsed for the bullet lead tokens docs/install.md owns" gains the helper and
  the refusal (deltas 2-3).
- **gate-sdk/SPEC.md, the `lib/` roster** — the new helper's contract (delta 3).
- **docs/site-architecture.md §Page-authoring rules** — the lead token as the
  third mechanically-held member of the post-repair class (the immutability
  ruling).
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
