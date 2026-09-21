# SPEC amendment: seam-content

The provenance seam bars two classes from a kit file (gate-sdk/SPEC.md §The
provenance seam). The sweep that has run, and canon-kit's `check-provenance-seam`,
hold only the second, **a publisher's provenance**. The first, **a consumer's rule
content**, has neither a discriminator a session can apply nor an oracle. Two
entries pair this amendment. One asks whether two instances in gate-sdk/SPEC.md are
private content. The other asks where a gate catching a kit SPEC quoting consumer
configuration draws its false-positive line. **This amendment rules the
discriminator, disposes of both instances by it, and adds the one shape an oracle
can hold without flooding.** That shape is a consumer-configured roster quoted in a
kit SPEC, and it becomes an arm of `check-provenance-seam`.

**The discriminator.** A kit literal is a consumer's rule content when it is true
only of one tree's configuration or practice. It is a value a second vendoring
consumer would read as the shape of their own. A value the kit itself ships is kit
content: its own descriptors, table defaults and templates. Quoting such a value in
the SPEC is a restatement, governed by de-literalization rather than by the seam.
The two rules then sort the two instances differently, and each instance's defence
is answered on its own ground.

- **Instance 1, the queue-practice block in §Porting a gate to the binary
  substrate, is private content.** It states which of *this* repo's queue entries
  hosts a port cut, the composer entry's column arithmetic against its own slug and
  `[roadmap:]` tag, and how the entry is promoted and demoted. That is one tree's
  queue practice written as gate-sdk mechanism. It also directs nothing any longer:
  the entry it describes is gone from the queue, and
  `--emit port-blockers --tree` prints `0 owed`. The one generic fact in it, that a
  lead line's tag capacity is per-entry arithmetic, is queue-kit mechanism and
  moves there. Its defence was that deleting it re-buys a `git log -S` at every
  cut. No cut remains to re-buy it, and the record is in history.
- **Instance 2, the six `couples=` values in §The first budget batch, is kit
  content.** They are values of kit-shipped descriptors. But the paragraph restates
  values the descriptors own, and this iteration's document-path unit changes those
  very values. So the sentence would go stale in the same iteration. Its defence
  was that it records one past derivation, and that de-literalizing it destroys the
  record. The record's *claim* is a negative: none of the six reaches a gate
  declaration path. That claim survives the de-literalization together with the
  oracle that re-derives it. What goes is the transcription of values, which
  history already holds.

**The false-positive boundary, measured (2026-09-21).** The probe drew every
override value from the consumer knob files and grepped each kit SPEC for it.
Matching any single override value is useless: about 94% of 72 candidate tokens had
a lawful reading. A value that is also a kit default, a generic noun or a kit's own
path is not a leak. Co-occurrence of two or more values of one knob in a paragraph
still flooded, with 144 hits. The hits were word-valued elements: the abbreviation
allow list, the queue horizons and agent meta paths. What discriminates is the
**shape of the attested leak**. The probe then counted a whole inline-code span
equal to an element of an indexed or keyed knob's consumer-set value, with a keyed
element spelled `<key>=<value>`, where the element carries a `/` or `=` and is
absent from every kit table and template. Two or more such spans of one knob in one
paragraph reds. That yields **zero hits** today over the eleven kit SPECs, and it
fires on the attested instance: `24f77de4:gate-sdk/SPEC.md` carries five
`LIFECYCLE_KIT_PREDECESSOR` pairs in one sentence.

## What changes

### (1) gate-sdk/SPEC.md §The provenance seam states the content-class discriminator {mechanical}

**Not yet applied.** After the paragraph **The seam decides the voice, never the
content.**, add:

> **What makes a literal consumer rule content is whose truth it is.** A value true
> only of one tree's configuration or practice is that tree's content, however it
> is voiced, and it leaves the kit: as optional config where the kit needs it,
> otherwise into the publisher's own record. A value the kit itself ships — its
> descriptors, its table defaults, its templates — is kit content, and quoting it is
> a restatement that de-literalization governs rather than the seam.

In the same section, the sentence naming `check-provenance-seam` changes to say the
gate holds the publisher-provenance class's lexical shapes *and* the content class's
roster shape (delta 4).

### (2) The queue-practice block leaves gate-sdk/SPEC.md {mechanical}

**Not yet applied.** Delete gate-sdk/SPEC.md §Porting a gate to the binary substrate
from the paragraph opening **Which queue entry a cut rides** through the paragraph
ending **Stated here so there is no fourth.** That is the hosting rule, the
dedicated-host case, the three-fact multi-cut resolution with its column arithmetic,
the blocker-versus-delivery paragraph and the reached-and-lost paragraph. The
paragraphs before and after stay. In queue-kit/SPEC.md §check-queue-wrap, after the
calibration paragraph, add:

> **A lead line's tag capacity is per-entry arithmetic, never a flat count.** A
> lead-line-scoped tag costs its bracketed width plus one space against the budget,
> an entry's fixed part is its slug lead plus every tag it carries permanently, and
> trailing prose reflows onto continuation lines — so how many refs one lead line
> holds is measured on the entry, never assumed.

`check-spec-pointer` is the oracle for inbound citations. Every citation of
§Porting a gate to the binary substrate targets a paragraph that stays, by
`grep -rn '§Porting a gate to the binary substrate' --include=*.md .`: CLAUDE.md,
installer/SPEC.md twice, and context-kit/SPEC.md. The heading itself stays.

### (3) §The first budget batch states its negative without the values {mechanical}

**Not yet applied.** In the paragraph **The substrate-sensitive set was re-derived at
the batch's cut**, replace the sentence enumerating the six values, which begins
"None of the six reaches a gate declaration path through its own `couples=`:", with:

> None of the six reaches a gate declaration path through its own `couples=`: each
> names a corpus outside `<kit>/checks/`, `native/src/gates/*.rs` and
> `native/src/*.rs`, which assertion C's derivation re-establishes whenever it is
> re-run.

### (4) `check-provenance-seam` gains a consumer-roster arm {design-bearing}

**Not yet applied.** It is added to canon-kit/SPEC.md §check-provenance-seam and to
`native/src/gates/provenance_seam.rs`.

- **Corpus.** The gate's existing one: every kit root's canonical spec, scanned only
  under `CANON_KIT_SCAN_KIT_ROOTS=1`, with fences skipped and paragraphs rejoined
  across wraps.
- **Candidates.** For every static indexed or keyed row whose layered value was set
  by the consumer (`Origin::is_set`), take the elements that are not in the row's
  static default. A keyed element is spelled `<key>=<value>`. Keep those carrying a
  `/` or `=`.
- **Red** when one paragraph carries two or more distinct candidates of one knob,
  each as a whole inline-code span. The finding names the file, the paragraph's
  first line, the knob and the matched spans, on the gate's existing channel and in
  its arm vocabulary as `consumer-roster`.
- **Reads.** The member's knob declaration reaches every static kit. So each kit's
  knob file joins its `couples=` as a derived couple (gate-sdk/SPEC.md §The `# graph:`
  manifest), and a consumer knob edit that creates a finding fires the gate, with no
  path literal in the descriptor.
- **Honest limits, in the section.** A singleton quote passes, as does a word-valued
  element, an unbackticked mention and a scalar knob's value. The singleton
  instances found while measuring are filed as one gap by this spec session, to be
  swept under delta 1's discriminator. They are canon-kit/SPEC.md §Layout and
  configuration quoting this repo's claim-command argv four times,
  gate-sdk/SPEC.md §The knob file quoting the measured-claims argv, and
  §Porting a gate to the binary substrate naming this repo's plugin files as knob
  values.
- **Fixture pair.** `bad/` carries a consumer knob file setting a keyed knob, and a
  kit SPEC quoting two of its pairs in one sentence. `good/` quotes one pair, and
  two pairs that equal the knob's default.

## Producers and consumers

- **The discriminator (delta 1).** It is prose, and its readers are the authoring
  session and the close-stage seam review.
- **The relocated arithmetic (delta 2).** Reader: a session adding a lead-line tag,
  which today meets the rule only in gate-sdk. No queue-kit gate reads prose, so no
  roster changes. The deleted block's one inbound reader, the port composer, is gone.
- **The arm (delta 4).** Producer: `check-provenance-seam`, registered at
  `precommit`. It is inert in every tree at the default `CANON_KIT_SCAN_KIT_ROOTS=0`,
  which is every adopter vendoring kits. Consumers: the battery and hook, through
  the gate's existing finding channel, and the fixture runner. No gate name, knob or
  directive is minted, so no gate-roster, knob roster or registration reader changes.
  **Release declaration:** build adds a `check-provenance-seam` bullet to the
  Tightened-gates section of `.workflow/release-declarations.md`. It reaches only a
  tree that authors kits, and its remedy is to name the knob rather than quote its
  configured roster.
- **Point 5.** Delta 2 removes paragraphs, and point 5 is checked against the
  readers of gate-sdk/SPEC.md. `check-spec-pointer` reds on a dangling `§`, and the
  heading stays. The block carries one HTML comment, a `manifest-temporal-exempt:`
  valve on its own mention of a removed shell path, and the valve goes with the
  mention it valves. It carries no measured-claim marker, and no reader asserts a
  minimum over it.
- **Point 6.** Delta 4's corpus is the eleven kit SPECs, and the probe above shows
  zero members red at landing.

## Existing sections updated

Rosters from `grep -rn '§Porting a gate to the binary substrate' --include=*.md .`,
the false-positive probe above, and reading gate-sdk/SPEC.md §The provenance seam and
§The first budget batch, queue-kit/SPEC.md §check-queue-wrap and canon-kit/SPEC.md
§check-provenance-seam.

- gate-sdk/SPEC.md §The provenance seam (delta 1).
- gate-sdk/SPEC.md §Porting a gate to the binary substrate and queue-kit/SPEC.md
  §check-queue-wrap (delta 2).
- gate-sdk/SPEC.md §The first budget batch (delta 3).
- canon-kit/SPEC.md §check-provenance-seam, `native/src/gates/provenance_seam.rs`,
  `canon-kit/gate-tests/check-provenance-seam/` and
  `.workflow/release-declarations.md` (delta 4).
<!-- update-target-exempt: generated mirrors, regenerated by their freshness gate's printed command -->
- `docs/gate-sdk/SPEC.md`, `docs/canon-kit/SPEC.md`, `docs/queue-kit/SPEC.md`.

## Retired spellings

- None — no name is removed; the deleted paragraphs name no mechanism of their own.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The
      causal-completeness check holds for the arm.
- [ ] **Instruction surfaces: instruction only.** Not reached.
- [ ] **Merged with no information lost.** The deleted block's one generic fact
      reaches queue-kit, and its practice is in history.
- [ ] **Amendment deleted.** This file is removed on merge (`ls SPEC-*.md`).
- [ ] **Entries moved.** `kit-spec-seam-content-half-unswept` and
      `kit-spec-consumer-config-literal` move to Done in the merge commit, at a stage
      before the drain stage.
- [ ] **Fails closed.** `bad/` reds on the quoted roster, and `good/` stays green on
      a singleton and on default-equal pairs.
- [ ] **Removals propagated.** `check-amendment-retired-spelling` runs the block
      above.
- [ ] **Gaps filed.** Any cross-component gap build discovers is resolved that
      session.
