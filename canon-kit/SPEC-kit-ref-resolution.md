# SPEC amendment: kit-ref-resolution

This amendment makes two changes to how `check-kit-ref-liveness`
(§Layout and configuration) resolves a token. Both are in the same paragraph.

- **The gap inbox joins the queue's valve.** The inbox is design-ahead in the
  same way the queue is. A gap bullet proposing an unminted knob by its full
  prefixed name reds the battery, so the filer has to describe the knob instead
  of naming it.
- **The member-to-stem direction narrows to a declared family.** Today any token
  under a stem that kit source spells resolves, so a typo'd knob under a defined
  stem passes.

**The measurements this amendment rests on (2026-09-22).**

- **The motivating composed family is gone.** The entry cites
  `GATE_SDK_KNOB_<GATE>_<KNOB>` and proposes resolving a token against the
  binary's `--knobs` answer. `.workflow/release-declarations.md` records that
  the `--knobs` arm and every `GATE_SDK_KNOB_<NAME>` variable were removed. So
  the stronger candidate the entry weighs no longer exists, and neither does its
  built-binary trade.
- **The binary's own knob table already declares the composed families.**
  `native/src/knobs/mod.rs`'s `Kit` carries `families: &[Family]`, and
  evidence-kit's table declares `EVIDENCE_KIT_RUN_` and `EVIDENCE_KIT_PARSER_`.
  `grep -n "families:" native/src/knobs/*.rs` shows every other static kit
  declares none. The gate already runs inside that binary, since it unions
  `knobs::static_names()` into its defined set, so reading the families adds no
  dependency.
- **The live hole is stems that are not families.** The probe
  `git grep -h -o -E '(<every kit prefix>)_[A-Z0-9_]*_([^A-Z0-9_]|$)' -- ':!*.md' ':!*/gate-tests/*' ':!docs/'`
  finds four stems in kit source. Two are the declared families. The other two,
  `CANON_KIT_PROSE_TELL_` and `CANON_KIT_COMMENT_`, are wildcard mentions in a
  help string (`native/src/gates/prose_tells.rs`) and a comment
  (`native/src/gates/comment_tier.rs`). Either one makes a typo such as
  `CANON_KIT_PROSE_TELL_EMDASH_MAXX` resolve.
- **The narrowing reds nothing today.** A scratch probe took every kit-prefixed
  caps run in the gate's scanned corpus (`git grep` over the tracked tree minus
  the gate's valves). It checked each one against `--emit knob-roster`, the
  static locators and the exact names in kit source. The one token left over is `GATE_SDK_UPGRADE_`, a scanned stem.
  It resolves through the stem-to-member direction, which this amendment keeps.
- **The inbox is readable by knob.** `LIFECYCLE_KIT_GAP_INBOX_FILE` is a static
  row (default `.workflow/gap-inbox.md`), so `walk::knob_scalar` reads it the
  way the gate already reads `GATE_SDK_QUEUE_FILE`. That is true even in a tree
  that vendors no lifecycle-kit, where the default names a file that does not
  exist and so valves nothing.

**The limb chosen for the inbox, and the one refused.** The entry offers
joining the valve or stating a filer rule not to spell unminted names. The
valve is taken. The inbox is truncated at every close, so a dangling name
cannot outlive the boundary. The same property is why the queue is valved.
A filer rule would keep the red and make every filer pay the rewording the
attested instance paid. The survey record is also truncated at the boundary, but
it is **not** valved. Its fields record measurements of the live tree rather
than proposals, and no red has been attested there. A valve with no instance
is widening for its own sake.

## What changes

### (1) The gap inbox joins the queue in the valve {mechanical}

**Not yet applied.** In §Layout and configuration, replace the closing valve
list's "…, `SPEC-*.md` amendments, and the queue — so a rename cannot leave a
dangle without turning a gate red." with:

> …, `SPEC-*.md` amendments, and the two design-ahead records: the queue and the
> gap inbox (`LIFECYCLE_KIT_GAP_INBOX_FILE`), each read by its knob. Both name
> knobs and paths not yet minted, and the inbox cannot hold one past the close
> that truncates it. So a rename cannot leave a dangle without turning a gate
> red.

In `native/src/gates/kit_ref_liveness.rs`, the valve skips the tracked path the
knob resolves to, compared as a repo-relative path rather than by basename. The
queue keeps its basename test. The `good/` fixture gains an inbox file at the
knob's path that names an unminted kit knob and must pass.

### (2) A member resolves through a stem only when that stem is a declared family {mechanical}

**Not yet applied.** In §Layout and configuration, replace the sentences from
"**A knob resolves through a family stem in either direction**" through "…which
is the one shape that crosses the provenance seam here." with:

> **A knob resolves through a family stem in either direction, and the second
> direction is bounded by a declaration.** A scanned `<FAMILY>_` stem resolves
> when kit source defines any member under it. A scanned member resolves through
> a stem only when that stem is a **declared family**. For a static kit, that
> means a family its knob table declares, as evidence-kit's table declares
> `EVIDENCE_KIT_RUN_` for the suite it is running
> (evidence-kit/SPEC.md §Layout and configuration). For a prefix no static table
> owns, it means a stem its kit source spells. The member direction exists for a
> **dispatch-composed** name, whose full spelling is built at runtime and appears
> in no kit literal. Spelling the members out to satisfy an exact match would be
> a hardcoded roster of consumer knob names in a kit literal, which is the one
> shape that crosses the provenance seam here. A static kit's table is the
> complete answer for its prefix, though. A stem its source spells only as a
> wildcard in a help line or a comment is not a family, and resolving members
> through one would pass any misspelling under it.

In the gate, the member-to-stem branch of `knob_ok` requires that the stem be a
family prefix of the static kit owning it (a new `knobs` accessor over
`Kit::families`), or that `knobs::owner` places the stem in no static kit. The
`bad/` fixture gains `CANON_KIT_PROSE_TELL_EMDASH_MAXX`, which resolves today
and must red. The `good/` fixture's `EVIDENCE_KIT_RUN_SOME_SUITE` must still
pass. The Rust unit test beside `knob_ok` gains both cases.

**The limit this keeps, stated so a green run is not read as closing it.** A
typo'd tail under a declared family, such as a misspelled suite name, still
resolves. The tail is consumer vocabulary, and a kit cannot enumerate it.

## Producers and consumers

- **The inbox valve** (delta 1). Producer: `LIFECYCLE_KIT_GAP_INBOX_FILE`'s
  static row, set in every tree through its default, which makes it reachable.
  Consumer: the gate's file walk, which drops the path before any scan. The
  inbox's writer, `--emit file-gap`, is unchanged.
- **The declared-family bound** (delta 2). Producer: the static knob tables'
  `families` field, which already exists and has another reader in the knob
  resolution of `native/src/knobs/mod.rs`. Consumer: `knob_ok`'s member branch,
  read once per scanned token. A second reader of the field is the only new
  coupling. A family added to a table later is reached with no edit here.
- **Point 5**, bound by delta 2, which narrows the resolving set (not a corpus
  but the same monotonicity question). The gate's red condition is a scanned
  kit-prefixed token that resolves neither exactly nor through a permitted stem.
  The narrowing can only add reds, and the probe above measured zero on the
  current tree. Delta 1 narrows the scanned corpus by one file. The gate reds
  on a found dangle, never on finding none, and asserts no count or floor, so
  the narrowing can only remove findings.
- **Point 6** does not bind. No delta obliges every member of a corpus.

## Existing sections updated

Rosters from `git grep -n "kit-ref-liveness\|kit_ref_liveness" -- ':!docs/' ':!TASK-QUEUE.md'`
and `grep -rn "knob_ok\|stem" native/src/gates/*.rs`, run 2026-09-22.
`check-docs-cmd`'s own `knob_ok` resolves only the stem-to-member direction, so
it carries no copy of the hole and is not a target.

- canon-kit/SPEC.md §Layout and configuration, the `check-kit-ref-liveness` paragraph (deltas 1 and 2).
- `native/src/gates/kit_ref_liveness.rs`, the valve and `knob_ok` (deltas 1 and 2).
- `native/src/knobs/mod.rs`, the family-prefix accessor (delta 2).
- `scripts/gate-tests/check-kit-ref-liveness/good/` and `bad/` (deltas 1 and 2).
<!-- update-target-exempt: generated mirror, regenerated by its freshness gate's printed command -->
- `docs/canon-kit/SPEC.md`.

## Retired spellings

- None — no delta of this amendment removes a name.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The
      causal-completeness check holds for the valve and the bound.
- [ ] **Instruction surfaces: instruction only.** Not reached, since no template
      or shim changes.
- [ ] **Merged with no information lost.** The stem paragraph is re-phrased, not
      appended to.
- [ ] **Amendment deleted.** This file is removed on merge (`ls canon-kit/SPEC-*.md`).
- [ ] **Entries moved.** `gap-inbox-kit-ref-valve` and
      `kit-ref-liveness-stem-token-hole` move to Done in the merge commit, at a
      stage before the drain stage.
- [ ] **Removals propagated.** `check-amendment-retired-spelling` runs the block
      above.
- [ ] **Gaps filed.** Any cross-component gap build discovers is resolved that
      session.
