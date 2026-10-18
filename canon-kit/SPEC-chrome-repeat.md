# SPEC amendment: chrome-repeat

Pairs `docs-chrome-page-repeat`. `check-docs-page-repeat` reads page sources and never the site chrome that renders around every page, so a page restating the chrome passes. The filed instance: `docs/index.md`'s License section read "Checkwright is [Apache-2.0](…/LICENSE)", linking the same target as the footer every page renders. `check-license-line`'s page walk now holds that one instance. This amendment adds an arm that reads the consumer's chrome files as part of every declared page.

**Weighed against the section's refusal of cross-page repeat.** §check-docs-page-repeat deliberately asserts no repeat across pages, because a restatement on another page is usually a paraphrase no scanner decides. Chrome is not another page. It renders inside every page, so a reader meets a page's restatement of it on that page. Exact repeats are what the gate already reads, and this arm reads them on the page the reader actually meets. The refusal stands for page-to-page repeats.

**The arm compares scheme targets, where arm A does not.** Chrome links are typically absolute: this repo's two literal chrome targets are `https://github.com/checkwright/checkwright` and its `blob/master/LICENSE`. The filed instance was such a link in a three-word sentence. A chrome arm on arm A's terms (relative targets, long sentences) would not have caught the case it was filed for.

**Generic mechanism, consumer chrome.** Which files are chrome is the consumer's layout, so it is a knob, default empty, which turns the arm off. A template expression is excluded by its delimiters, `{{ … }}` and `{% … %}`, the shared delimiters of Liquid and its relatives, so no engine is named.

Probe at spec: a scan over the chrome globs delta 3 sets, comparing against the 38 pages `CANON_KIT_PAGE_REPEAT_PAGES` expands to, found two literal chrome targets and one chrome sentence of eight words or more ("Checkwright — governed by the same kits it ships."). No page restates any of them, so the arm lands green here.

## What changes

### (1) `check-docs-page-repeat` gains arm C, chrome restated on a page {design-bearing} {user-facing: docs-chrome-page-repeat's Deliverable, "an arm prefixing the layout's and includes' literal text nodes (Liquid excluded) to every page's corpus, so a sentence of eight words or more or a link target stated in both reds"}

Replacement for `canon-kit/SPEC.md` §check-docs-page-repeat, its invariant sentence. **Not yet applied.**

> Invariant: no declared page states a relative link target, or a sentence of `CANON_KIT_PAGE_REPEAT_MIN_WORDS` words or more, twice, nor restates the site chrome that renders around it.

A bullet after the **Arm B** bullet, **not yet applied**:

> - **Arm C — chrome restated on a page.** Files matching `CANON_KIT_PAGE_REPEAT_CHROME`, expanded like the page globs, are read as part of every declared page, each occurrence there coming first. HTML comments, `script` and `style` elements and template expressions (`{{ … }}`, `{% … %}`) are dropped. A chrome **target** is a literal `href` value, and its **text** is what each line holds once tags are stripped, split and folded as arm B splits a page. A page link whose target equals a chrome target is a repeat, a scheme target included, since chrome links are commonly absolute. A page sentence of `CANON_KIT_PAGE_REPEAT_MIN_WORDS` words or more equal to a chrome sentence is a repeat too. A `mailto:` or pure `#anchor` target is out of scope, as in arm A. Repeats inside the chrome are not asserted.

The **Red** paragraph's `help:` sentence gains, **not yet applied**: "Chrome's is *the site chrome already states it on every page: drop the page's copy*." The finding names the chrome file and line as the first occurrence. The clean line counts chrome files. **Exit 2** also covers an unreadable chrome file.

The **Deliberately not asserted** paragraph's last sentence becomes, **not yet applied**:

> A restatement across pages is usually a paraphrase, which no scanner decides. Chrome is no other page, since it renders inside each one, so arm C reads it as part of every page. **Honest limits:** every chrome file joins every page whichever template a page renders through, so a consumer with several layouts names only the chrome all of them share. Character references are compared undecoded. A chrome statement shorter than the sentence floor and carrying no link, such as a three-word licence line, is not read.

`native/src/gates/docs_page_repeat.rs`: a `CHROME_KNOB` constant, a chrome reader yielding `(target, file, line)` and `(sentence, file, line)` sets through the extractions above, and arm C's comparison in `scan`. The page side reuses `line_link_targets` without arm A's scheme filter and arm B's sentence fold. `Arm::Chrome` carries the chrome locator in place of a first-occurrence line. The unit test module gains a case over an inline chrome string: a literal absolute target, a template-valued `href`, a sentence inside a `script` element, and a matching page link and sentence.

### (2) The knob, its declared reads and the fixtures {mechanical}

- `canon-kit/SPEC.md` §Layout and configuration gains, after `CANON_KIT_PAGE_REPEAT_MIN_WORDS`'s bullet, **not yet applied**: "`CANON_KIT_PAGE_REPEAT_CHROME` — the site-chrome template files `check-docs-page-repeat`'s arm C reads as part of every declared page, an array of globs, empty by default, which turns the arm off."
- `native/src/knobs/canon_kit.rs`: `Row::indexed("CANON_KIT_PAGE_REPEAT_CHROME", &[])`.
- `native/src/gates/mod.rs`, the member's row: a second root `(".", "glob:knob:CANON_KIT_PAGE_REPEAT_CHROME", "", "")` and the knob among its declared names.
- `canon-kit/checks/check-docs-page-repeat.gate`: `knob:CANON_KIT_PAGE_REPEAT_CHROME` joins `couples=`, and the `# spec:` gloss names the chrome arm.
- `canon-kit/gate-tests/check-docs-page-repeat/`: both fixture knob files set the chrome glob to a `site/_chrome/*.html`. The `bad/` chrome carries an absolute link and a long sentence that a page restates. The `good/` chrome carries both, plus a template-valued `href` and a `script` sentence that a page shares without a finding, and one valved restatement.
- `canon-kit/README.md`'s gate roster line names the chrome reach.

### (3) This repo declares its chrome {mechanical}

`scripts/canon-config.knobs`, after the `CANON_KIT_PAGE_REPEAT_PAGES` lines, **not yet applied**: `CANON_KIT_PAGE_REPEAT_CHROME[] = docs/_layouts/*.html` and `CANON_KIT_PAGE_REPEAT_CHROME[] = docs/_includes/*.html`, under a `# comment-tier-exempt:` line. Its reason: the site's one layout and its include render around every declared page.

`docs/page-authoring.md` §Page-authoring rules, the **One statement, one home** paragraph: one clause saying the site chrome counts as part of every page, so a page does not restate what the chrome says. **Not yet applied.**

## Producers and consumers

- **`CANON_KIT_PAGE_REPEAT_CHROME` (deltas 1 to 3).**
  - Producer: a consumer's knob file. This repo sets it (delta 3), so a deployed configuration enables the arm; the kit default leaves it off.
  - Consumer: the member, once per run, through the glob walk declared in its registry row.
  - Roster-holding readers, from `git grep -n "CANON_KIT_PAGE_REPEAT_PAGES"` over the tracked tree outside the posts and mirrors: §Layout and configuration, the defaults table, the registry row, the descriptor's `couples=` (which `check-reads-couples` holds to the row), and this repo's knob file (deltas 2 and 3). `canon-kit/templates/canon-config.knobs` names neither page-repeat knob, so it gains none.
- **Arm C's finding (delta 1).** Producer: the member. Consumer: the committing session through the output contract and `--run-gate-tests` through the pair. Fields: the page and line, which the session edits; the target or sentence, which it removes; the chrome locator, which it reads to confirm the chrome says it. Each has its reader.
- **Point 5:** no delta narrows a corpus.
- **Point 6:** delta 3 obliges each declared page to restate no chrome target or long sentence. The members are the 38 pages `CANON_KIT_PAGE_REPEAT_PAGES` expands to. Each satisfies it as it stands, per the probe above.

## Existing sections updated

The roster came from `git grep -n "PAGE_REPEAT\|page-repeat" -- ':!docs/posts' ':!docs/*-kit/*' ':!TASK-QUEUE.md'` over the tracked tree.

- `canon-kit/SPEC.md` §check-docs-page-repeat (delta 1).
- `native/src/gates/docs_page_repeat.rs` (delta 1).
- `canon-kit/SPEC.md` §Layout and configuration (delta 2).
- `native/src/knobs/canon_kit.rs` (delta 2).
- `native/src/gates/mod.rs`, the member's row (delta 2).
- `canon-kit/checks/check-docs-page-repeat.gate` (delta 2).
- `canon-kit/gate-tests/check-docs-page-repeat/` (delta 2).
- `canon-kit/README.md`, the gate roster (delta 2).
- `scripts/canon-config.knobs` (delta 3).
- `docs/page-authoring.md` §Page-authoring rules (delta 3).
- `.workflow/release-declarations.md`: `## Knob changes` for the new knob, and `## New and tightened gates` for `check-docs-page-repeat` tightened by arm C where the knob is set (deltas 1 and 2).
- `docs/canon-kit/SPEC.md` and `docs/canon-kit/README.md`, the generated mirrors, and `docs/check-graph.html` for the descriptor (all deltas).

## Retired spellings

- None — arms A and B keep their wording and help lines; the arm adds names and retires none.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls canon-kit/SPEC-*.md`) once the iteration's last canon-kit amendment merges.
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Native build and suites** — `bash gate-sdk/bin/build-native.sh`, the crate's unit tests, `--run-gate-tests canon-kit/gate-tests` and the full battery run green.
- [ ] **Done move** — the entry moves to Done in the commit that merges this amendment, before the drain stage.
