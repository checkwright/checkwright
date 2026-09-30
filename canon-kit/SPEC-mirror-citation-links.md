# SPEC amendment: mirror-citation-links

**The on-site SPEC mirror renders every section citation as plain text.** `--emit docs-mirror` (`native/src/emit/docs_mirror.rs`) rewrites the targets of the links a source already carries and nothing else. The kit SPECs and the doctrine deliverable cite sections as prose, so a site reader finds each cited section by hand. A citation with no path is also held only for liveness (§check-spec-pointer), never pointed at a target a reader can follow.

**The mirror knows the target at emit time, so it renders the link.** check-spec-pointer's reader already parses the three citation forms and resolves each heading against a file. The mirror calls that reader, has the resolver return the heading it matched rather than a bare yes, and wraps the citation in a link to that heading's anchor. The target goes through the emitter's own `rewrite_target`, so a citation link and a hand link to one file render alike. Nothing is converted by hand and nothing can drift: the freshness gate compares the mirror with the emitter's output as it does today.

**A slug defect comes due with it, and delta 1 converges it on the owning section.** §check-md-refs says an anchor must match "the GitHub heading slug of a heading in the target file". `spec::anchor_slug` collapses a run of spaces into one hyphen. GitHub and the site's kramdown-GFM ids replace each space, so a heading carrying an em dash or a dropped `#` or `§` between spaces gets a double hyphen: `Content tiering — the star topology` has the id `content-tiering--the-star-topology`. So `check-md-refs` passes a link that 404s on the rendered page. The runner builds its published invariant URL through the same function (`native/src/runner.rs`). A mirror rendering anchors through it would publish broken ones for those headings. This is spec-determined debt that the unit cannot ship without, so it lands here, as delta 1.

**Measured at authoring.**

- The tracked `*/SPEC.md` hold 947 path-form citations, none in the link-then-`§` form, and 2207 bare ones outside fences and code spans. Of the bare ones, 1990 resolve in the citing file alone, 165 in the citing file and elsewhere, 50 in exactly one other file, 2 in several other files, and none nowhere. This was a scripted emulation of the reader; the gate is the oracle for liveness.
- doctrine-kit/DOCTRINE.md carries 17 `§` lines in the link-then-`§` form.
- Of 506 headings on the current mirror pages, 18 slug differently under the GitHub rule than under `anchor_slug`: 17 carry an em dash, and one is `The # graph: manifest`. The comparison ran kramdown 2.5.2 with kramdown-parser-gfm 1.1.0 and `auto_ids`.
- Two tracked links carry the collapsed slug of such a heading: `docs/ddd.md` (`canon-kit/SPEC.md#content-tiering-the-star-topology`) and `docs/orchestration.md` (`…#economics-batch-and-compact-where-it-pays`). The probe computed both slugs for every heading carrying ` — `, ` – `, ` # `, ` & ` or ` § ` over the tracked markdown less fixtures, and counted links to the collapsed form.
- The mirror pages are in no `check-citation-link`, `check-docs-page-repeat` or manifest-set glob of this repository's `scripts/canon-config.knobs`, and `check-docs-link-convention` walks them for shape. A `../<dir>/SPEC.md#anchor` link from one mirror page to another resolves inside `docs/` and satisfies its off-root rule.

## What changes

### (1) The anchor slug replaces each space

`native/src/spec.rs` `anchor_slug` {design-bearing}: after lowercasing and dropping every character outside `[a-z0-9 _-]`, each remaining space becomes one hyphen. Runs are kept rather than collapsed, and nothing is trimmed.

Its readers move together:

- `check-md-refs`' anchor resolution;
- `check-citation-link`'s arm B;
- the runner's published invariant fragment.

The two collapsed-slug links become `#content-tiering--the-star-topology` and `#economics--batch-and-compact-where-it-pays`. A unit test pins the three shapes: an em dash between spaces, a dropped `#`, and a leading dropped `§`. The fixture pairs of `check-md-refs` and `check-citation-link` are run, and a fixture whose anchor relied on the collapse is corrected.

canon-kit/SPEC.md §check-md-refs, first paragraph. **Not yet applied.** "must match the GitHub heading slug of a heading in the target file" becomes "must match the GitHub heading slug of a heading in the target file: lowercased, every character but a letter, digit, space, `-` or `_` dropped, and each space a hyphen, so an em dash between spaces leaves two".

### (2) The resolver returns the heading it matched

`native/src/gates/spec_pointer.rs` {design-bearing}: the heading resolution behind `present` gains a crate-visible form returning the matched heading. It returns the heading's whole text and the length of the fragment prefix it matched, or nothing. The prefix is the whole title, the qualifier-stripped title or the lead clause, by the rules §check-spec-pointer states.

`present` becomes a call to it, so the gate's verdict does not move. The path-less form's resolution is **the citing file first, then the one other manifest file holding the heading**. Several other files and none resolve to nothing. The gate's liveness verdict stays the union it is today, and the ordered form is the renderer's.

### (3) The mirror renders each resolved citation as a link

`native/src/emit/docs_mirror.rs` {design-bearing}: for a mirrored `SPEC.md` and for `doctrine-kit/DOCTRINE.md`, each paragraph's citations are read through `spec_pointer::paragraphs` and `sites`. Each one the resolver of delta 2 resolves is rewritten:

- **The link text** is the citation's own text, from its path (or from the `§` for a bare one) through the matched prefix, so the text after the heading stays prose. A link-then-`§` citation folds into one link whose text is the link's text, a space and the `§` span.
- **The target** is the cited file through `rewrite_target`, or a bare `#anchor` for the same file, with `#` and the matched heading's `anchor_slug` appended.
- **Line breaks inside the span are kept.**

What is left plain:

- a citation the resolver leaves unresolved;
- one already inside a link's text;
- one in a heading line, a code span, a fence or an HTML comment.

A mirrored `README.md` is left as it is, because its citations are hand links that `check-citation-link` already holds.

The unit tests cover:

- each form;
- the same-file and the one-other-file bare resolution;
- an ambiguous bare citation left plain;
- a lead-clause match whose anchor is the whole heading's slug;
- a citation broken by a wrap.

### (4) §The reference-link grammar states the rendering

canon-kit/SPEC.md §The reference-link grammar {mechanical}. **Not yet applied.** After the sentence ending "…so the relative shape a page uses is the same shape the source tree uses.", insert:

> **The emitter also renders each section citation in a mirrored SPEC or the doctrine deliverable as a link**, resolved through §check-spec-pointer's reader: the citation's own text becomes the link text, and the target is the cited file's mirrored or blob form with the heading's anchor. A citation with no path resolves to the citing file first, then to the one other governed file holding the heading. One held by several, or by none, stays plain, as does a citation already inside a link, so a README's hand links pass through unchanged. The honest limit is §check-spec-pointer's: a path-less citation meaning another file's section whose title the citing file shares links the citing file's own, and naming the path before the `§` pins it.

### (5) The projection row and the release declaration

`docs/site-architecture.md` §Generated projections and their freshness gates, the on-site SPEC mirror row {mechanical}: after its page list, the row gains "with every resolvable section citation in a SPEC or the doctrine rendered as a link (canon-kit/SPEC.md §The reference-link grammar)".

`.workflow/release-declarations.md` {mechanical}:

- under Tightened gates: "`check-md-refs` — an `#anchor` must now match the heading slug GitHub and kramdown render, each space a hyphen, so a link to a heading carrying an em dash (`Content tiering — the star topology`) needs the double hyphen (`#content-tiering--the-star-topology`); the single-hyphen form 404ed on the rendered page. Re-point each finding's anchor."
- under Tightened gates: "`check-citation-link` — arm B compares an anchor with that same slug. Re-point each finding's anchor."
- under Behavior changes: "**`--emit docs-mirror`** — a mirrored SPEC's and the doctrine's section citations render as links to their sections. Regenerate your mirror with `--emit docs-mirror --write`; nothing else to do."

## Producers and consumers

- **The rendered link.**
  - Producer: `--emit docs-mirror`, on `--write` and inside `check-docs-mirror-fresh`'s in-process comparison.
  - Enabling config: `CANON_KIT_MIRROR_ROOT`, which this repository leaves at `docs`.
  - Consumers: the site reader, and the freshness gate, which compares bytes and needs no change.
  - Each field is read where it is written: the link text by the reader, the target and the anchor by the browser.
- **The returned heading.**
  - Producer: the resolver of delta 2.
  - Consumers: `present`, which reads only whether it is there, and the renderer, which reads the text for the anchor and the prefix length for the link-text extent.
- **The slug.**
  - Producer: `anchor_slug`.
  - Consumers: `check-md-refs`, `check-citation-link` and the runner (`git grep -n anchor_slug -- native/src`).
  - Its narrowing reds `check-md-refs` on the two collapsed links, per violation, and delta 1 fixes both. No reader holds a count or a floor over anchors.
- **The mirror pages.** Their size grows by the link targets. `check-surface-ratchet` holds `docs/` pages under `.workflow/surface-ceiling.txt`, whose rows are re-stamped with `bash gate-sdk/bin/run-gates.sh --emit always-loaded --ceiling`.

## Existing sections updated

- `native/src/spec.rs`, and any `check-md-refs` or `check-citation-link` fixture relying on a collapsed slug: the slug (delta 1).
- `docs/ddd.md`: its collapsed anchor (delta 1).
- `docs/orchestration.md`: its collapsed anchor (delta 1).
- `canon-kit/SPEC.md` §check-md-refs (delta 1) and §The reference-link grammar (delta 4).
- `native/src/gates/spec_pointer.rs`: the resolver (delta 2).
- `native/src/emit/docs_mirror.rs` (delta 3).
- `docs/site-architecture.md`, the mirror row (delta 5).
- `.workflow/release-declarations.md` (delta 5).
- Every generated `docs/<dir>/SPEC.md` and `docs/doctrine-kit/DOCTRINE.md`, regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write`, and `.workflow/surface-ceiling.txt`'s rows for them (all deltas).
- `.workflow/prose-bound-ceiling.txt`: the `canon-kit/SPEC.md` row, re-stamped to the count `check-prose-bounds` prints if deltas 1 and 4 move it (deltas 1 and 4).

The roster came from `git grep -n anchor_slug -- native/src`, `grep -n 'fn ' native/src/emit/docs_mirror.rs native/src/gates/spec_pointer.rs`, the slug probe over the tracked markdown, and docs/site-architecture.md's mirror row.

## Retired spellings

- `content-tiering-the-star-topology` — the collapsed anchor, re-pointed to the double-hyphen form (delta 1).
- `economics-batch-and-compact-where-it-pays` — the collapsed anchor, re-pointed likewise (delta 1).

## Definition of Done

- [ ] **Causal completeness**: every point of canon-kit/SPEC.md §The causal-completeness check holds for the slug, the returned heading and the rendered link.
- [ ] **Instruction surfaces: instruction only**: the projection row states what the mirror carries, with the grounds in §The reference-link grammar.
- [ ] **Merged with no information lost**: §The reference-link grammar reads as one section, and §check-md-refs states the slug rule once.
- [ ] **Amendment deleted**: this file is removed on merge.
- [ ] **Removals propagated**: `check-amendment-retired-spelling` finds neither collapsed anchor outside this roster.
- [ ] **Gaps filed**: a cross-component gap found during the work is filed with `--emit file-gap`.
- [ ] **The entry moves**: `spec-mirror-citation-links` moves to Done in the landing commit, before the drain stage.
