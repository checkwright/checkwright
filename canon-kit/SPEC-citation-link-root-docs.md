# SPEC amendment: citation-link-root-docs

**The root documents GitHub renders are read like the READMEs beside them, and their citations are unlinked.** `check-citation-link` holds the pages `CANON_KIT_CITATION_LINK_PAGES` names, and this repository binds the site pages, the front door and the top-level READMEs. The other root documents a reader opens on GitHub carry plain `§` citations, so a reader hunts each cited section by hand. Those documents are RELEASING.md, CONTRIBUTING.md, SECURITY.md, CODE_OF_CONDUCT.md, TRAJECTORY.md and ROADMAP.md.

**Out of range: the agent-loaded working surfaces.** CLAUDE.md is always loaded, so every link's target and anchor bytes grow the always-loaded meter on every session. TASK-QUEUE.md is the stage sessions' working file, read and machine-edited at every stage. Neither is read chiefly on GitHub.

**One kit change comes with the widening.** ROADMAP.md is the first bound page carrying a generated region, the `roadmap:begin`…`roadmap:end` block its emitter writes. `check-citation-link` scans a generated region like hand prose, and a finding there is unfixable at the page, since the repair is to the emitter's source. `check-prose-tells` already holds generated regions out on that ground (§check-prose-tells), through the shared marker test (§The shared spec adapters), and this gate takes the same holdout.

**Measured at authoring.** `bash gate-sdk/bin/run-gates.sh --only check-citation-link` over a copy of `scripts/canon-config.knobs` with the six documents added reds 31 arm-A findings: RELEASING.md 19, CONTRIBUTING.md 5, SECURITY.md 3, TRAJECTORY.md 3, ROADMAP.md 1 and CODE_OF_CONDUCT.md none. There are no arm-B findings. None of the cited headings carries an em dash (`grep -n` over the cited files' headings), so no anchor depends on `spec-mirror-citation-links`' slug delta. ROADMAP.md's one finding is its hand-authored intro, and its generated block carries no `§`.

**Costed with it, and refused: a heading per ruling in TRAJECTORY.md.** A header would let a citation link `TRAJECTORY.md#<name>` as queue slugs are linked. Against it:

- The record format is lifecycle-kit's (§The ruling-staleness probe reads `ruling:` and `discharge:` lead lines).
- A ruling declares one or more names, while a heading's anchor carries one slug.
- `check-spec-pointer`'s one-title-per-file rule would hold every ruling name unique as a title.
- A heading standing alone as a paragraph is an edge for the probe's forward-phrase pass.
- The file shrinks toward empty and holds no ruling now.
- No tracked file links `TRAJECTORY.md#` (`git grep -n 'TRAJECTORY.md#'` finds only the queue entry).

The citers use the ruling name the probe sweeps, which a header would not replace.

## What changes

### (1) check-citation-link holds generated regions out

`native/src/gates/citation_link.rs` {mechanical}: `prose_only` blanks the lines from a `<!-- name:begin -->` marker line through its `<!-- name:end -->`, markers included, using `spec::is_gen_marker`, the test `check-prose-tells` uses. The `good/` fixture gains a generated region holding an unlinked citation, which stays clean.

canon-kit/SPEC.md §check-citation-link, the citation bullet's last sentence. **Not yet applied.**

> Fenced blocks, HTML comments, the front-matter block and generated `<!-- name:begin -->`…`<!-- name:end -->` regions are skipped, the last because a region is its emitter's to fix (§check-prose-tells).

### (2) The binding reaches the GitHub-rendered root documents

`scripts/canon-config.knobs` {mechanical}: `CANON_KIT_CITATION_LINK_PAGES` gains `CONTRIBUTING.md`, `SECURITY.md`, `CODE_OF_CONDUCT.md`, `RELEASING.md`, `TRAJECTORY.md` and `ROADMAP.md`. The comment above the binding becomes:

> `# comment-tier-exempt: the hand-authored pages whose section citations are links (docs/site-architecture.md §Page-authoring rules): the site pages, the front door, the top-level READMEs and the other root documents GitHub renders; CLAUDE.md and the queue stay out as agent-loaded working surfaces, where links grow every session's read, and the dated posts and generated mirrors stay out`

The descriptor's `knob:CANON_KIT_CITATION_LINK_PAGES` token expands to the new globs, so the trigger widens with no descriptor edit.

### (3) The 31 citations become links

RELEASING.md, CONTRIBUTING.md, SECURITY.md, TRAJECTORY.md and ROADMAP.md {mechanical}: each finding the widened run reports becomes a link in the form the gate's help line prints. The target is repo-relative from the root, and a same-file section takes a bare `#anchor`. A later mention of a section linked earlier on the page stays plain text, which arm A admits.

A citation inside a quoted literal takes the valve instead: RELEASING.md's quotation of a `# contract:` header gets `<!-- citation-link-exempt: quotes the contract header's literal text -->`. Linking there would falsify the quote.

The TRAJECTORY.md edits touch its contract prose and no ruling, the any-session act of correcting a fact that directs nothing different afterwards.

### (4) The release declaration

`.workflow/release-declarations.md` {mechanical}, under Behavior changes:

> - **canon-kit `check-citation-link`** — a generated `<!-- name:begin -->`…`<!-- name:end -->` region on a declared page is skipped, as `check-prose-tells` skips it, since its fix is the emitter's. Nothing to do.

## Producers and consumers

- **The holdout.**
  - Producer: `check-citation-link`'s page read, per page, per run.
  - Enabling config: any bound page carrying a marker pair. This repository's ROADMAP.md does once delta 2 lands.
  - Consumer: the committing session through the output contract. The region's lines never reach arms A and B.
- **The widened corpus.** Delta 2 widens a corpus and narrows none. The gate reds per citation and holds no count or floor, so no reader's verdict turns on the corpus shrinking.
  - The six documents are already in the manifest set. `check-md-refs` therefore already resolves each new link's target and anchor, and `check-spec-pointer` already resolves each citation.
  - `check-docs-page-repeat` binds its own pages knob, which delta 2 does not touch, so a link added here is never its finding.
  - `check-surface-ratchet` holds `.workflow/surface-ceiling.txt` rows for grown always-loaded surfaces. None of the six is one (`grep -n` over that file).

## Existing sections updated

- `native/src/gates/citation_link.rs` and `canon-kit/gate-tests/check-citation-link/good/`: the holdout (delta 1).
- `canon-kit/SPEC.md` §check-citation-link (delta 1).
- `docs/canon-kit/SPEC.md`: the generated mirror, regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` (delta 1).
- `scripts/canon-config.knobs`: the binding and its comment (delta 2).
- `RELEASING.md`, `CONTRIBUTING.md`, `SECURITY.md`, `TRAJECTORY.md`, `ROADMAP.md`: the conversions (delta 3). ROADMAP.md's edit is to the hand-authored intro, outside the marker block `check-roadmap-fresh` compares.
- `.workflow/release-declarations.md`: the Behavior changes bullet (delta 4).

The roster came from the widened scratch run above, from `git grep -n CITATION_LINK_PAGES` over the tracked tree, and from `grep -n 'prose_only\|is_gen_marker' native/src/gates/citation_link.rs native/src/spec.rs`.

## Retired spellings

- None — the amendment widens a binding and adds a holdout, renaming nothing.

## Definition of Done

- [ ] **Causal completeness**: every point of canon-kit/SPEC.md §The causal-completeness check holds for the holdout and the widened corpus.
- [ ] **Instruction surfaces: instruction only**: the knob comment states the range and its ground, nothing else.
- [ ] **Merged with no information lost**: §check-citation-link reads as one section.
- [ ] **Amendment deleted**: this file is removed on merge.
- [ ] **Removals propagated**: nothing retired.
- [ ] **Gaps filed**: a cross-component gap found during the work is filed with `--emit file-gap`.
- [ ] **The entry moves**: `citation-link-root-docs-range` moves to Done in the landing commit, before the drain stage.
