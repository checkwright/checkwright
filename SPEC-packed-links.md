# SPEC amendment: packed-links

A vendored kit README links a `SPEC.md` its installed tree does not carry: the
payload withholds each kit's `SPEC.md` (`GATE_SDK_PAYLOAD_WITHHOLD`, default
`SPEC.md smoke`), and the README's link stays relative. Probed,
`git grep -n '](SPEC.md' -- '*-kit/README.md' 'gate-sdk/README.md'` returns **18
links across the 11 kit READMEs**, four of them anchored (doctrine-kit:12,37,78;
site-kit:26). `git grep -n '](\.\./[a-z-]*/SPEC\.md'` over the same corpus returns
**zero**, and no reference-style link exists — so **the fix is own-SPEC-only**;
there is no cross-kit link target to widen to.

**Why the tracked link stays relative, restated from the entry because it is what
bounds the fix.** Re-targeting the links at the published location would put the
publisher's host in eleven kit files — the kit literal the `<KIT>_<KNOB>`
convention exists to prevent — and would break canon-kit/SPEC.md §The
reference-link grammar's one-to-one mirror topology, `docs/<kit>/README.md` being
a byte projection of the source that `check-docs-mirror-fresh` enforces.
Confirmed: `docs/<kit>/SPEC.md` exists for all eleven kits, so the docs-site link
resolves and only the **installer-vendored** tree dangles. That is why the
resolution belongs at pack time and nowhere else.

**Two premises of the entry's candidate, probed and corrected.**

- The entry says the resolution works *"the way a shipped `# spec:` pointer
  already resolves"*. The **grammar** is reusable; the **function** is not.
  `runner.rs:522-545`'s `resolved_location` requires the exact shape
  `<dir>/SPEC.md` — one non-empty segment, no further `/` — and a bare `SPEC.md`,
  which is every README link's shape, falls through to its in-tree branch. The
  packer must construct `<base>/<leaf>/SPEC` from the kit's own directory name.
- The packer is not a pure copier. `stamp()` (`pack_installer.rs:600-631`) already
  rewrites `{asm}/package.json` during packing. A README rewrite is a **second
  instance of an existing precedent**, not a new class — which is the answer to
  the objection that pack-time mutation is itself the risk.

**And one finding that changes what this unit owes.** No gate compares packed kit
content to tracked content: `verify_sidecar()` digests only the prebuilt
artifacts, and the consumer smoke vendors by direct filesystem copy
(`native/src/emit/csmoke.rs:126-165`), bypassing the packer entirely. So the
rewrite breaks no existing check — and equally, **nothing would catch it
regressing**. A rewrite without a reader of its output is a change nobody can
observe, so delta 3 is not optional garnish; it is the first thing in the battery
to look inside a packed README at all.

It is a root-level amendment because it spans gate-sdk (the payload contract and
the base-URL knob), `native/` (the packer) and `installer/` (the packer's owning
SPEC and the smoke).

**The tree does not already do this.** `git grep -n "SPEC_BASE_URL"
native/src/emit/pack_installer.rs` returns `:19` and `:215-216` — the stamp's read
only; nothing touches a payload README's bytes.

## What changes

**Batching.** Deltas 1 and 2 land together: the sequencing move and the rewrite
it enables are one edit to `pack()`'s body. Delta 3 lands with them, because a
rewrite whose output nothing reads is unobservable. Delta 4 is documentation of
what 1 to 3 did and rides with them.

### (1) `spec_base_url` is resolved before the kit loop {mechanical}

**Not yet applied.** `walk::knob_scalar("GATE_SDK_SPEC_BASE_URL")`, read today at
`pack_installer.rs:215` — after the kit loop at `:196-204` and only for `stamp()`
— moves above the loop, and `stamp()` takes the already-resolved value. Nothing
else changes: the knob's default stays empty, and an empty value still means
*resolve in the tree*.

Mechanical: it is a statement move with one call site, and the existing packer
tests are its oracle.

### (2) A packed kit README's own-SPEC link resolves to the published target {design-bearing}

**Not yet applied.** Inside the kit loop, immediately after `pack_tracked` at
`:202` succeeds, the packer rewrites `{asm}/payload/{leaf}/README.md` in place:
every markdown link whose target is exactly `SPEC.md`, or `SPEC.md#<fragment>`,
becomes `<base>/<leaf>/SPEC` or `<base>/<leaf>/SPEC#<fragment>`.

- **`<leaf>` is the loop's own `leaf`** — the kit's directory name, already bound
  at `:201`. The packer never parses the kit name out of the link.
- **The fragment passes through unchanged.** A README link's fragment is already
  a markdown heading slug, which is the same value `crate::spec::anchor_slug`
  would produce from a `§<heading>` pointer. `anchor_slug` exists to convert a
  *heading* to a slug; a fragment is not a heading, and running it through the
  converter would be a second, lossy normalization of an already-normalized
  token. Stated because the corpus has four anchored links and silence here would
  leave the next reader guessing.
- **An empty `GATE_SDK_SPEC_BASE_URL` rewrites nothing**, matching the knob's
  documented *resolve in the tree* meaning. A publisher who sets no base ships
  the relative link and the sentence beside it, which is today's behaviour
  exactly — so this delta cannot regress an unconfigured packer.
- **The tracked README and its docs mirror are never touched.** The rewrite
  operates on the extracted copy under `{asm}/payload/`, which is why
  `check-docs-mirror-fresh`'s byte-freshness invariant and canon-kit/SPEC.md §The
  reference-link grammar's mirror topology both survive untouched.

**The sentence already on every README stays.** All eleven carry *"An
installer-vendored tree does not carry this file. The payload withholds each
kit's `SPEC.md` and its `smoke/`, publishing the specification at the location
`GATE_SDK_SPEC_BASE_URL` names instead"* — verified present in all eleven, none
missing. It is now the explanation of a link that **works** rather than the
consolation for one that does not, and delta 4 re-words it on that footing rather
than deleting it: a reader who sets no base still meets the relative link.

### (3) A gate reads the packed README {design-bearing}

**Not yet applied.** `check-packed-links` (gate-sdk, native substrate, with the
`good/`+`bad/` fixture pair the four contracts require) asserts, over a packed
payload the gate produces itself:

- **A.** With a non-empty `GATE_SDK_SPEC_BASE_URL`, no README under
  `payload/<kit>/` carries a link whose target is a path the payload withholds.
  The withheld set is read from `GATE_SDK_PAYLOAD_WITHHOLD`, never re-listed, so
  adding a member to the withhold list extends the assertion with no gate edit.
- **B.** With an empty base, every packed README is byte-identical to its tracked
  source — the direction that proves the rewrite is conditional rather than
  unconditional, which assertion A alone cannot show.

**Point 5 — the red condition.** A names the kit, the README line and the
withheld target and exits 2; B names the first differing byte offset and exits 2.
The clean line prints the packed-kit count and the rewritten-link count, so a run
that packed nothing, or rewrote nothing because the base was silently empty, is
visible on green rather than indistinguishable from a clean one.

**Why this gate and not an extension of the consumer smoke.** The consumer smoke
deliberately vendors by copy (`csmoke.rs`'s own comment says so), and changing
that to exercise the packer would re-point a surface whose copy-vendoring is its
contract. A gate that packs its own fixture is the narrower change.

### (4) The payload contract states what the packer resolves {mechanical}

**Not yet applied.** gate-sdk/SPEC.md §Consumer payload's
`GATE_SDK_SPEC_BASE_URL` paragraph — which today states only the `# spec:`
pointer resolution and the stamp — gains the README-link resolution as the second
thing the base governs, with the empty-base behaviour and the
tracked-file-untouched bound. installer/SPEC.md §The packer names the rewrite
beside the `stamp()` rewrite it already describes, so the packer's two mutations
are one roster rather than one documented and one discovered.

The eleven READMEs' withholding sentence is re-worded once, in one wording, to
name the published link rather than only the published specification.

Mechanical: the wording is settled by deltas 2 and 3 and the sweep is eleven
identical replacements with `check-docs-mirror-fresh` as its oracle.

## Producers and consumers

- **The resolved `spec_base_url` (delta 1).** Producer: `walk::knob_scalar`.
  Consumers: `stamp()`, unchanged; and the README rewrite of delta 2. Its
  enabling configuration is the publisher's own knob file — set in this repo's
  `scripts/gate-sdk-config.knobs`, not test-only — and its empty default is a
  live, deployed state (any adopter re-packing with no base), which is why delta
  2 gives it a defined behaviour rather than treating it as unreachable.
- **The rewritten packed README (delta 2).** Producer: the packer's kit loop.
  Consumers: the adopter reading a vendored README, at click time; and
  `check-packed-links`, at battery time. Before this amendment the first consumer
  existed and the second did not, which is why the defect survived.
- **`check-packed-links` (delta 3).** Producer: its `.gate` descriptor in
  gate-sdk's `checks/`, registered in `gates.list`. Consumers: the battery, the
  generated pre-commit hook through its `# graph:` manifest (coupling
  `pack_installer.rs`, `GATE_SDK_PAYLOAD_WITHHOLD` and every kit README), and its
  fixture pair.
- **The stated contract (delta 4).** Readers: a publisher choosing a base, an
  adopter meeting a link, and `check-payload-claim`, which holds one governed doc
  as the single declarer of what the vendored payload discloses — so the added
  paragraph lands in the section that gate already points at, not a second one.

**Point 6 — every member of the README corpus has a satisfying value.** The
corpus is the eleven kit READMEs the packer's own loop enumerates. Ten carry one
to four own-SPEC links each and are rewritten. `gate-sdk/README.md` carries three
(`:34`, `:58`, `:102`) and is rewritten identically — it is packed through the
same loop and its own `SPEC.md` is withheld by the same default, so it is not an
exception. No member is narrowed past.

## Existing sections updated

Rosters produced by `git grep -n '](SPEC.md' -- '*-kit/README.md'` plus the same
over `gate-sdk/README.md` (18 hits, 11 files), `git grep -n
'](\.\./[a-z-]*/SPEC\.md'` over that corpus (zero hits), `grep -rn SPEC_BASE_URL
native/src/` (eight sites), and by reading `native/src/emit/pack_installer.rs` in
full, gate-sdk/SPEC.md §Consumer payload and §Layout and configuration,
installer/SPEC.md §The packer, and canon-kit/SPEC.md §The reference-link grammar.

- `native/src/emit/pack_installer.rs` — the `spec_base_url` read's position, the
  kit loop, and the new rewrite function (deltas 1 and 2).
- `gate-sdk/SPEC.md` §Consumer payload: the `GATE_SDK_SPEC_BASE_URL` paragraph
  (delta 4).
- `installer/SPEC.md` §The packer: the packer's mutation roster (delta 4).
- `scripts/gates.list`, a new `gate-sdk/checks/check-packed-links.gate`, its
  native module and its `good/`+`bad/` fixture pair; gate-sdk/README.md's gate
  roster block (delta 3).
- The eleven kit READMEs' withholding sentence (delta 4).
- `.workflow/release-declarations.md` §Behavior changes, appended by the landing
  session: one bullet that a packed kit README's own-SPEC link now resolves to
  the published location where a base is set (delta 2).
<!-- update-target-exempt: generated mirrors and projections, regenerated by their freshness gates' printed commands, never hand-edited -->
- `docs/gate-sdk/SPEC.md`, `docs/installer/SPEC.md`, every `docs/<kit>/README.md`,
  `docs/enforcement.md`, `scripts/CHECK-GRAPH.html`, `scripts/git-hooks/pre-commit`.

## Retired spellings

- None — no name is removed. The links' *targets* change in the packed copy only,
  the tracked spelling `](SPEC.md)` is deliberately preserved (delta 2's
  tracked-file-untouched bound), and no identifier, knob or file is renamed.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The
      causal-completeness check holds for each new state, event, interface and
      obligation.
- [ ] **Instruction surfaces: instruction only** — the READMEs' re-worded
      sentence carries no grounds; delta 4 places them in gate-sdk/SPEC.md.
- [ ] **Merged with no information lost** — each addition re-phrases the
      canonical-spec text it refines rather than appending to it.
- [ ] **Amendment deleted** — this file removed on merge (`ls SPEC-*.md`).
- [ ] **Entry moved** — `vendored-kit-readme-spec-link-dangles` moves to Done in
      the merge commit, a stage before the drain stage.
- [ ] **The tracked tree is unchanged in shape** — `check-docs-mirror-fresh` and
      `check-md-refs` stay green with no regeneration beyond delta 4's sentence.
- [ ] **The rewrite is observable** — `check-packed-links` reds on a packed
      README still linking a withheld target, and on an unconditional rewrite.
- [ ] **Removals propagated** — `check-amendment-retired-spelling` runs the block
      above against the tracked tree.
- [ ] **Gaps filed** — any cross-component gap build discovers is resolved that
      session.
