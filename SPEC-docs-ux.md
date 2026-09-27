# SPEC amendment: docs-ux

The site has no authoring rules for page names, section names, citation form or collapsible regions, so a page improves only when someone happens to notice it. The first measured defect is citation form: rendered pages and the kit READMEs cite sections as bare text, such as docs/install.md's *(§With Node)*, which a reader must find by hand. A path-less citation is also held only to liveness: `check-spec-pointer` asserts that some governed file carries the heading, so a citation aimed at the wrong file passes. This amendment states the four rules in docs/site-architecture.md, gates each rule's decidable half, and converts the citations on the pages it governs.

One queue entry pairs it: [docs-ux-authoring-rules](TASK-QUEUE.md#docs-ux-authoring-rules).

**The rulings.**

- **Four rules, one home.** Page names, section names, link-form citations and collapsible regions join docs/site-architecture.md §Page-authoring rules beside *One statement, one home*. The rules name their gates; the gates' contracts live in their kits' SPECs.
- **A citation is a link, and the link carries the anchor.** A link names its file, so `check-md-refs` resolves both file and section, which closes the wrong-file hole a path-less citation leaves open. The link text keeps the path and the `§` heading, so a reader and a model both see the target before following it. A later mention of a section already linked on the page stays plain text, as `check-docs-page-repeat` requires. The citation gate admits it, so the two gates compose.
- **The citation gate is canon-kit's, and the collapsible gate is site-kit's.** The citation grammar and the reference-link grammar are canon-kit's, and the gate reuses `check-spec-pointer`'s citation reader rather than a second lexer. `markdown="1"` is a fact about the Pages parser, which is site-kit's subject.
- **Range.** Converted by hand: the hand-authored `docs/` pages and the kit READMEs, `installer/README.md` among them. Dated posts are immutable and stay as published. The generated pages are out: the SPEC and README mirrors regenerate from their sources, and `docs/enforcement.md`'s citations are already links. The front door's `README.md` is converted by [companion-toolkit-profile](TASK-QUEUE.md#companion-toolkit-profile), whose landing-page polish owns that page.
- **The kit SPECs are out of range, and the gap is filed.** Their site mirrors carry about 2,129 citation lines outside fences. A link rendering at mirror time, resolving each citation through `check-spec-pointer`'s resolver, is the derivation that would reach them. It was filed to the gap inbox with this amendment's pairing commit, costed by that census, rather than hand-converted here.
- **Naming rules are review's, except uniqueness.** A heading that names its section in the reader's words is a judgment. That a heading is unique on its page is already held: docs pages and READMEs are in the manifest set, where `check-spec-pointer` reds two headings sharing a title in one file.
- **The seam.**
  - Kit mechanism: `check-citation-link` and its corpus knob, shipped by canon-kit and off by default; `check-docs-collapsible`, shipped by site-kit.
  - This repository's own: the corpus binding, the rules' text in docs/site-architecture.md, and the converted pages.
  - No private rule content is involved.

**Refused.**

- **Converting the kit SPECs by hand.** About 2,129 lines, most of them in contract prose a brevity pass is already rewriting; a derivation at mirror time reaches them all and cannot drift.
- **Requiring every mention to be a link.** It collides with `check-docs-page-repeat`'s link-once rule and adds a link a reader has already been offered.
- **Asserting the link target's file against the citation's path.** A README's `SPEC.md` citation targets `SPEC.md`, a docs page's targets the mirror, and an off-site one targets a blob URL, so agreement needs path resolution per target shape. The anchor is where a renamed section leaves a stale link, and arm B holds that.
- **Folding the collapsible check into `check-docs-render-fidelity`.** That gate needs the renderer and fails closed without it; a source-level check needs nothing.

**Measured at authoring.**

- **The render.** Under the site renderer (`SITE_KIT_RENDERER`'s default kramdown GFM program), `<details>` without `markdown="1"` prints its body's markdown raw, and with it the body renders parsed. `check-docs-render-fidelity` reds the unmarked region only when its body carries a code span or a fence, since a raw backtick is its symptom. A prose-only unmarked region, with bold text, a link and a list, rendered raw and passed clean. Probed in a scratch repository holding one page of each shape.
- **The citation census.** Lines carrying `§` outside fences, over `docs/*.md`, `docs/*/index.md` and the twelve top-level `*/README.md` files:
  - docs pages: `docs/site-architecture.md` 16, `docs/orchestration.md` 8, `docs/install.md` 7, `docs/enforcement.md` 6 (already links), `docs/ddd.md` 5, `docs/positioning.md` 3, `docs/releases.md` 1;
  - READMEs: drift-kit 11, lifecycle-kit 10, delegation-kit 6, gate-sdk 4, guard-kit 4, installer 4, evidence-kit 3, context-kit 2, queue-kit 2, site-kit 2, canon-kit 1, doctrine-kit 1;
  - kit SPECs, out of range: 2,129.
- **The forms.** Seven shapes occur: a whole code span `` `path §Heading` ``, a bare `path §Heading`, `` `path` §Heading ``, a link followed by `§` (`[SPEC.md](SPEC.md) §The state machine`), a path-less same-page `§Requirements`, a `SPEC §Heading` with no extension, and a possessive `that file's §Heading`. Every form converts to the link shape delta 1 states.
- **The existing collapsed regions.** `<details` opens two regions, both on docs/install.md, each carrying `markdown="1"` and a `<summary>` and holding no heading; docs/site-architecture.md names the element in prose only.

## What changes

### (1) The page-authoring rules {design-bearing}

**Not yet applied.** docs/site-architecture.md §Page-authoring rules gains four paragraphs after **One statement, one home.**, and its first paragraph's opening sentence stays as the title rule. The text below is the content; the build may tighten the wording without dropping a clause.

> **Page names.** A page's file name is its URL: the subject in lowercase hyphenated words, stable once published. Rename a page only when its name misleads, and move every inbound link with it. `check-md-refs` finds the in-tree links; an outside link to the old name breaks, and a misleading name is worth that price.
>
> **Section names.** A heading names what its section answers, in the reader's words — a noun phrase or a task, such as *Choosing a profile* — and uses the implementation's name only where that name is what the reader types. It is sentence case, with no trailing punctuation and no link or `§` in it. A heading is an address as well as a label, since a citation links its anchor. So it is unique on its page, which `check-spec-pointer`'s one-title-per-file rule holds over every page and README, and a rename moves its inbound links in the same commit, which `check-md-refs` reds if missed.
>
> **Section citations are links.** Cite a section with a link whose text is the citation and whose target carries the section's anchor: `` [`installer/SPEC.md` §Requirements](installer/SPEC.md#requirements) ``. The target follows canon-kit/SPEC.md §The reference-link grammar: the on-site mirror for a kit's SPEC or README, a self-repo blob link for any other file off the site, and a bare `#anchor` for a section of the same page, whose link text is the heading alone. A later mention of a section the page already linked is plain text, by the rule above. `check-citation-link` holds the form over the pages `CANON_KIT_CITATION_LINK_PAGES` names, and `check-md-refs` resolves each target and anchor.
>
> **Collapsible regions.** Collapse depth a reader may skip, such as the step-by-step form under a one-line command or a long reference table. Never collapse the primary path, a step every reader runs, or a section a link targets. A region opens `<details markdown="1">`, then a `<summary>` naming what opens, and holds no heading; without `markdown="1"` the Pages parser prints the region's markdown raw. `check-docs-collapsible` holds that form on every page under the docs root. What collapses is review's.
>
> **A page found breaking a rule is redone.** The unit that finds it rebuilds the page to the rule, or files it with its cost; it is never annotated around.

The same section's parser paragraph gains one clause naming `check-docs-collapsible` beside `check-docs-render-fidelity`, since the raw-region class is the one render-fidelity misses on a prose-only body.

### (2) The citation gate: `check-citation-link` {design-bearing}

**Not yet applied.** canon-kit/SPEC.md gains a section after §check-docs-page-repeat:

> ### check-citation-link
>
> Invariant: on a declared page, every section citation sits in a link's text, and a link whose text carries a citation targets the section it names.
>
> - **Corpus:** files matching `CANON_KIT_CITATION_LINK_PAGES`, an array of globs expanded like every canon-kit glob knob, default empty. An empty expansion is a clean `0 page(s)`.
> - **A citation** is what §check-spec-pointer's prose-citation pass reads — its three forms, its placeholder rule and its code-span rule — through the same reader, so the two gates cannot disagree on what a citation is. Fenced blocks, HTML comments and the front-matter block are skipped.
> - **Arm A — an unlinked citation.** A citation whose `§` lies outside every link's text is a finding. A link followed by `§` is one, since its heading is not linked. **Admitted:** a later mention, where the text at the citation's start begins with the text of a link earlier on the same page, compared with backticks dropped and whitespace folded. That is the plain-text later mention §check-docs-page-repeat requires.
> - **Arm B — a link that misses its section.** A link whose text carries a citation must carry a `#anchor`, and the anchor must be the heading slug of the text after the `§`, or begin with that slug and a `-`. The second form admits a citation naming a heading by its lead clause, as §check-spec-pointer admits it. The slug is §check-md-refs' heading slug. Whether the anchor exists is §check-md-refs' finding and is not repeated here.
> - **Valve:** `citation-link-exempt: <reason>` on the citation's line or the one above, in the shared exempt window (§The shared spec adapters). The reason is mandatory, and a valve without one exempts nothing.
>
> **Red** is one finding per citation, naming the page, the line and the citation. Arm A's `help:` line is *make the citation a link to its section: `[<path> §<heading>](<target>#<anchor>)`*. Arm B's is *point the link at the section its text names*. The clean line counts pages, linked citations and admitted later mentions. **Exit 2:** an unreadable page. `tier=precommit`, `install: zero-config`, armed by `CANON_KIT_CITATION_LINK_PAGES`.
>
> **Deliberately not asserted: the link target's file against the citation's path.** A README targets `SPEC.md`, a docs page the mirror, and an off-site citation a blob URL, so agreement needs a resolution per target shape; the anchor is where a renamed section leaves a stale link.

canon-kit/SPEC.md §Layout and configuration gains the knob row after the two page-repeat rows:

> - `CANON_KIT_CITATION_LINK_PAGES` — the pages `check-citation-link` holds, an array of globs, empty by default.

The implementation:

- `native/src/gates/citation_link.rs` holds the rule and calls the prose-citation reader `native/src/gates/spec_pointer.rs` uses, factored into a shared function if it is not one already; the gate table in `native/src/gates/mod.rs` registers it.
- `native/src/knobs/canon_kit.rs` declares the knob.
- `canon-kit/checks/check-citation-link.gate` carries `# graph: couples=knob:CANON_KIT_CITATION_LINK_PAGES dir=one valve=none tier=precommit`, `# install: zero-config`, `# armed-by: CANON_KIT_CITATION_LINK_PAGES` and the `# spec:` line.
- Its `good/`+`bad/` pair: `bad/` carries a bare path citation, a whole-code-span citation, a link followed by `§`, a path-less citation, a link with no anchor and a link whose anchor names another section. `good/` carries a linked citation in each target shape, a lead-clause anchor, an admitted later mention, a `§<heading>` placeholder, a `§` quoted in a code span, a fenced citation and a valved citation.
- The module's unit tests hold the later-mention comparison, the slug agreement and the exit-2 path.
- canon-kit/README.md's gate roster and canon-kit/smoke/install.sh register it.

`.workflow/release-declarations.md`'s Tightened gates section gains `` - `check-citation-link` `` with its one-line intent. It arms only where a consumer sets the corpus knob.

### (3) The collapsible gate: `check-docs-collapsible` {design-bearing}

**Not yet applied.** site-kit/SPEC.md gains a section after §check-docs-liquid-parse:

> ## check-docs-collapsible
>
> `checks/check-docs-collapsible.gate` (`precommit`, binary-dispatched, `install: on-surface`), its rule in `native/src/gates/docs_collapsible.rs`. Invariant: every collapsible region on a tracked page under `SITE_KIT_DOCS_DIR` renders its body as markdown, names what it holds, and hides no heading.
>
> - **Corpus:** §check-docs-render-fidelity's page set, the same `git ls-files` enumeration with underscore-prefixed segments excluded.
> - **A region** opens at a line whose first non-blank text is `<details` and closes at the matching `</details>`, counted by depth so a nested region is its own region. Fenced blocks and HTML comments are skipped.
> - **Assertions**, one finding each:
>   - the opening tag carries `markdown="1"` or `markdown="block"`, since without it the Pages parser passes the body through raw;
>   - the next non-blank line holds a `<summary>` element with non-empty text;
>   - no ATX or setext heading sits inside the region, since a heading a link targets must not be hidden;
>   - every region closes before the page ends.
>
> **Red** names the page, the opening line and the assertion; the `help:` line names the region's form. **Clean** counts pages and regions; a page with none is clean. **Exit 2:** not a git repository, a docs dir not found, or an unreadable page, in that order, the refusal order §check-docs-render-fidelity states. **No valve:** a raw body and an unlabelled region have no legitimate case, and a region that must hold a heading is a section to leave open. **No renderer** is needed; the check reads source only, so it runs where Ruby is absent.
>
> **Honest limit:** the gate holds the form, not the choice. Whether a region hides the primary path is review's.

site-kit/SPEC.md §Layout and configuration's registration sentence names it: it registers where a docs site built by the Pages parser exists, as `check-docs-render-fidelity` does.

The implementation:

- `native/src/gates/docs_collapsible.rs` holds the rule; `native/src/gates/mod.rs` registers it.
- `site-kit/checks/check-docs-collapsible.gate` couples render-fidelity's page set and carries the `# spec:` line.
- Its `good/`+`bad/` pair: `bad/` carries a region without `markdown="1"`, one without a summary, one holding a heading and one left open. `good/` carries a marked region with a fence and a table inside, a nested region, a `<details` inside a fence and a page with none.
- site-kit/README.md's gate roster and site-kit/smoke/install.sh register it.

`.workflow/release-declarations.md`'s Tightened gates section gains `` - `check-docs-collapsible` `` with its one-line intent.

### (4) This tree binds the corpus and converts its pages {mechanical}

**Not yet applied.**

- `scripts/canon-config.knobs` binds `CANON_KIT_CITATION_LINK_PAGES` to `docs/*.md`, `docs/*/index.md` and `*/README.md`. The globs miss `docs/posts/`, the generated mirrors (`docs/<dir>/SPEC.md`, `docs/<dir>/README.md`, `docs/doctrine-kit/DOCTRINE.md`) and `reserve/`. [companion-toolkit-profile](TASK-QUEUE.md#companion-toolkit-profile) adds `README.md` when it converts the front door.
- `scripts/gates.list` registers `check-citation-link` beside `check-docs-page-repeat`, and `check-docs-collapsible` beside `check-docs-render-fidelity`.
- Every citation outside a fence on the bound pages becomes a link in delta 1's shape, the census above being the worklist: `docs/site-architecture.md`, `docs/orchestration.md`, `docs/install.md`, `docs/ddd.md`, `docs/positioning.md`, `docs/releases.md` and the twelve READMEs. A same-page citation becomes `[<heading>](#<anchor>)`. A citation of a file the site does not serve (`CLAUDE.md`, `TRAJECTORY.md`, a root page, a source file) takes the self-repo blob form, which `check-docs-link-convention`'s off-root rule already demands. A second mention of one section on a page becomes plain text, which `check-docs-page-repeat` demands.
- A README link resolves on two surfaces, the kit directory and its generated mirror under `docs/<dir>/`, and the mirror keeps the cross-citation topology, so a relative target such as `../gate-sdk/SPEC.md#consumer-payload` holds on both.

The battery with both gates registered is the oracle for every page. `check-md-refs`, `check-docs-link-convention` and `check-docs-page-repeat` hold the converted links' resolution, shape and count.

## Producers and consumers

- **`CANON_KIT_CITATION_LINK_PAGES`.**
  - Producer: `scripts/canon-config.knobs` sets it.
  - Consumer: `check-citation-link` at every commit, since the gate is precommit-tier and registered.
  - Roster-holding readers of the minted name: `check-knob-citation` bars stating its value outside canon-kit/SPEC.md; `check-docs-cmd` (B) reds a backticked knob name no kit code carries, so delta 1's text lands in the commit that declares the knob or after it; `check-knob-default-coupling` reads the empty default.
  - Each member's value: the three globs delta 4 names.
- **`citation-link-exempt:`**, a new comment directive. It rides as an HTML comment on a page, never a full-line comment on the comment surface, so `check-comment-tier` owes it no row; the converted pages need none.
- **The two gates' findings.** Consumer: the committing session, through the output contract on the generated pre-commit hook, `run-gates.sh` and CI, and the `--run-gate-tests` arm through each fixture pair. The new-gate fan-out below regenerates the projections the gate roster feeds.
- **The link text's fragment** (arm B) is read at commit by `check-citation-link` and by `check-spec-pointer`'s citation pass, which resolves the heading in the cited file; the anchor is read by `check-md-refs`.
- **Red conditions.**
  - `check-citation-link`: an unlinked citation, a citation link without its anchor, an unreadable page. An empty corpus is clean and counted.
  - `check-docs-collapsible`: a finding per failed assertion; it reds on no count, so a page without regions is clean.
- **Each member's value (delta 4).** Every census line becomes a link or an admitted later mention; a line whose `§` the reader's placeholder rule excludes, if the build finds one, needs nothing and is named in the landing commit.

## Existing sections updated

Roster probes, over the tracked tree: `git grep -n` for `§Page-authoring rules`, `check-docs-page-repeat` and `check-docs-render-fidelity`; the census command above; and the new-gate fan-out docs/site-architecture.md §Generated projections and their freshness gates lists.

- `docs/site-architecture.md` — §Page-authoring rules (delta 1); its own citations (delta 4).
- `canon-kit/SPEC.md` — the new §check-citation-link and §Layout and configuration's knob row (delta 2).
- `native/src/gates/citation_link.rs`, `native/src/gates/spec_pointer.rs`, `native/src/gates/mod.rs`, `native/src/knobs/canon_kit.rs`, `canon-kit/checks/check-citation-link.gate`, `canon-kit/gate-tests/check-citation-link/`, `canon-kit/README.md`, `canon-kit/smoke/install.sh` (delta 2).
- `site-kit/SPEC.md` — the new §check-docs-collapsible and §Layout and configuration's registration sentence (delta 3).
- `native/src/gates/docs_collapsible.rs`, `site-kit/checks/check-docs-collapsible.gate`, `site-kit/gate-tests/check-docs-collapsible/`, `site-kit/README.md`, `site-kit/smoke/install.sh` (delta 3).
- `.workflow/release-declarations.md` — two Tightened gates bullets (deltas 2 and 3).
- `scripts/canon-config.knobs`, `scripts/gates.list` (delta 4).
- `docs/orchestration.md`, `docs/install.md`, `docs/ddd.md`, `docs/positioning.md`, `docs/releases.md` (delta 4).
- `canon-kit/README.md`, `context-kit/README.md`, `delegation-kit/README.md`, `doctrine-kit/README.md`, `drift-kit/README.md`, `evidence-kit/README.md`, `gate-sdk/README.md`, `guard-kit/README.md`, `installer/README.md`, `lifecycle-kit/README.md`, `queue-kit/README.md`, `site-kit/README.md` (delta 4).
- `docs/canon-kit/SPEC.md`, `docs/site-kit/SPEC.md` and every `docs/<dir>/README.md` mirror — regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` (all deltas).
- `docs/enforcement.md`, `docs/value.md`, `docs/check-graph.html`, `scripts/git-hooks/pre-commit`, `.workflow/surface-ceiling.txt` — the new-gate fan-out, each regenerated by the command its gate prints (deltas 2 and 3).

## Retired spellings

- None — the converted citations change form and name no retired path, knob or token.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; no root-level amendment remains for this unit (`ls SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Run** — the full battery green with both gates registered; canon-kit's and site-kit's fixture suites green; `check-docs-render-fidelity` green over every converted docs page. The entry moves to Done before the drain stage (`LIFECYCLE_KIT_DRAIN_STAGE`).
