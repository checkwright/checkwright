# SPEC amendment: prose-bounds

The SPEC tier is hard to read as a specification, and its size is mostly structure rather than filler. A sampled classification found contract sentences the plurality class in every sampled paragraph. The cost comes from three shapes, one move each:

- **Run-on structure.** Sentences chained on colons and semicolons, and paragraphs of several hundred words. The move re-punctuates and splits.
- **Archaeology.** Dated instrument runs, round-by-round debugging logs and the narrative of how a rule was reached. The move sends them to the commits that carry them and keeps the undated rule and its grounds.
- **Restatement.** One rule re-derived on every bullet of a roster. The move collapses it to one lead-in.

This amendment lands a gate for each move and applies the moves to the first five sections. It also bounds the agent file's paragraphs, because one bound serves both corpora. The rest of the tier is `spec-brevity-residue`, run under these gates.

**The component span.** The length and repetition gate is canon-kit's, beside `check-prose-tells`, and its code lands in the crate. The archaeology gate is a new arm of canon-kit's `check-provenance-seam`, and the rule it enforces is gate-sdk/SPEC.md §The provenance seam's, which this amendment rewrites. context-kit's §The brevity gate states where an agent file's paragraphs are bounded. The five sections belong to gate-sdk, installer, delegation-kit, canon-kit and guard-kit. So the unit spans several components and sits at the repo root.

**What was run at authoring** (at `3cd24370`), with a scratch model of the gate's unit rule: a unit is a blank-line block, a list item is its own unit, fences, tables and headings are held out, an inline code span is one word, and a sentence ends at a run of `.`, `!` or `?` followed by whitespace.

- **The tier at 45 words a sentence and 200 a unit** (`*/SPEC.md`, `doctrine-kit/DOCTRINE.md`, `CLAUDE.md`): 16,820 sentences, 3,475 over the bound; 4,960 units, 514 over it.
- **The five sections at the same bounds**, long sentences then long units: §The port-candidate criteria 111 and 23; §The first cohort 77 and 9; §The consumer smoke 126 and 13; delegation-kit §Testing 30 and 6; canon-kit §Layout and configuration 22 and 4; guard-kit §Layout and configuration 9 and 1.
- **The agent file** has no unit over 200 words and two sentences over 45 (54 and 50 words).
- **Repetition**: a six-word phrase, code spans and path tokens dropped, recurring in three or more units of one section occurs 201 times across 29 sections of the tier. It hits canon-kit §Layout and configuration's per-bullet provenance-seam clause and gate-sdk §The port-candidate criteria's retirement-record narration of deleted shell forms. The agent file has none.
- **Dates**: `git grep -n -E` for an ISO date over the seam corpus finds 19 lines. They are 12 in gate-sdk/SPEC.md (one in §The first cohort), 3 in installer/SPEC.md (all in §The consumer smoke), 2 in delegation-kit/SPEC.md, and one each in lifecycle-kit/SPEC.md (a specimen) and site-kit/SPEC.md. canon-kit's `check-provenance-seam` `good/` fixture carries a lawful dated measurement.
- **The environment override reaches an empty value**, as gate-sdk/SPEC.md §The knob file states ("an exported empty value included"). `CANON_KIT_PROSE_TELL_EMDASH_MAX= bash gate-sdk/bin/run-gates.sh --only check-prose-tells` refused `got ''` at exit 2, so the file's value did not apply. The worklist spelling in delta 1 therefore needs no new operand.

## What changes

### (1) `check-prose-bounds` {design-bearing}

**Applied.** The four unit line tests (list item, heading, table row, generated-region marker) moved to `native/src/spec.rs` with the split, so both gates cut a unit by one rule. C lists overlapping phrase windows as the one longer phrase they cover. A new canon-kit gate: `canon-kit/checks/check-prose-bounds.gate`, `native/src/gates/prose_bounds.rs`, its row in `native/src/gates/mod.rs`, and a `good/`+`bad/` fixture pair under `canon-kit/gate-tests/check-prose-bounds/`, with `check-prose-bounds.test.sh` for the arms a pair cannot spell. It is born native. The descriptor takes `check-prose-tells`' shape: `tier=precommit`, `# install: on-surface`, and couples `knob:CANON_KIT_PROSE_BOUND_GLOBS` and `knob:CANON_KIT_PROSE_BOUND_CEILING_FILE`.

The new section, after §check-prose-tells in canon-kit/SPEC.md:

> ### check-prose-bounds
>
> Invariant: governed prose keeps its sentences and paragraphs within a bound and states a repeated rule once. The corpus is the files `CANON_KIT_PROSE_BOUND_GLOBS` matches; empty scans nothing and passes. Which prose is governed, and at what bound, is the consumer's calibration, so every bound has an `off`.
>
> **The unit** is §check-prose-tells': a blank-line block, with each list item and its continuation lines a unit of its own, so a bullet run is never one paragraph. Headings, table rows, fenced blocks, generated regions and comment-only lines are held out. A word is a whitespace-separated token, and an inline code span counts as one word. A sentence ends at a run of `.`, `!` or `?` followed by whitespace or the unit's end, the split §check-prose-tells assertion E uses. A colon or semicolon ends nothing, because a chain of them is the run-on this gate exists for.
>
> - **A. Sentence length** — a sentence over `CANON_KIT_PROSE_BOUND_SENTENCE_MAX` words.
> - **B. Unit length** — a unit over `CANON_KIT_PROSE_BOUND_PARAGRAPH_MAX` words.
> - **C. Repeated phrase** — a run of `CANON_KIT_PROSE_BOUND_REPEAT_WORDS` words recurring in at least `CANON_KIT_PROSE_BOUND_REPEAT_MIN` units of one section. Case is folded, and code spans, link targets, path-shaped tokens and punctuation are dropped first, so a shared citation or knob name does not count as prose. A section is the span from one heading to the next heading of any level. A rule stated on every bullet of a roster is the shape: state it once, above the roster.
>
> A finding names its file, line, assertion, the measured figure and its section. C reports one finding per section, at the first unit carrying a repeated phrase, and lists the phrases.
>
> **The ceiling.** A tree adopting the gate over existing prose sets `CANON_KIT_PROSE_BOUND_CEILING_FILE`. It holds a `# contract:` header, then one `<n> <path>` row per governed file with findings, sorted by path. With a ceiling, a file's findings count against its row and must equal it. More findings than the row is red: shorten the prose. Fewer is red too, and the message prints the lowered row to write, so a pass that removes findings records it in the same commit and the slack cannot hide a regression. A governed file with findings and no row is red. A row naming a file outside the corpus is ignored, so narrowing the corpus never reds. With the knob empty every finding is red. The gate never writes the file.
>
> The worklist behind a ceiling is the gate run with the knob emptied in the environment: `CANON_KIT_PROSE_BOUND_CEILING_FILE= bash gate-sdk/bin/run-gates.sh --only check-prose-bounds`.
>
> **Valve**: `<!-- prose-bound-exempt: <reason> -->` on the flagged line or the one above, riding the shared exempt window with a mandatory reason. An exempt finding leaves the count too.
>
> **Fail-closed (exit 2):** a threshold the knob table refuses; a ceiling file named but absent or unreadable; a row that does not parse, or a second row for one path.
>
> **Honest limits.** A paraphrased restatement shares no phrase and passes C. A word count is not reading time. Under a ceiling, a commit that removes one finding and adds another in the same file keeps the count and passes. Undated narrative of how a rule was reached passes all three assertions; the dated half is §check-provenance-seam's.
>
> Producer: the generated pre-commit hook and `run-gates.sh`, on a commit touching a governed file or the ceiling file. Consumer: the committing session through the output contract. Each field is read at the one scan and nothing persists. `good/` holds a bounded agent file and a bounded spec with a valved long sentence; `bad/` trips A, B and C, one of them in an agent file's paragraph section. `check-prose-bounds.test.sh` holds the ceiling arms: equal, above, below, missing row, stray row, duplicate row and the absent file.

The shared sentence split moves from `native/src/gates/prose_tells.rs` into `native/src/spec.rs`, so the two gates cut a sentence by one rule, and §The shared spec adapters gains its bullet:

> - **The sentence split** that §check-prose-tells assertion E and §check-prose-bounds share: a run of `.`, `!` or `?` followed by whitespace or the span's end.

canon-kit/SPEC.md §Per-component contracts' bespoke unit-test roster gains `check-prose-bounds.test.sh`.

### (2) The knobs {mechanical}

**Applied.** `native/src/knobs/canon_kit.rs` gains six rows and their validation:

- `CANON_KIT_PROSE_BOUND_GLOBS` — array, default empty.
- `CANON_KIT_PROSE_BOUND_SENTENCE_MAX` — default `45`; a positive integer or `off`.
- `CANON_KIT_PROSE_BOUND_PARAGRAPH_MAX` — default `200`; a positive integer or `off`.
- `CANON_KIT_PROSE_BOUND_REPEAT_WORDS` — default `6`; an integer of at least `3`, or `off`.
- `CANON_KIT_PROSE_BOUND_REPEAT_MIN` — default `3`; an integer of at least `2`.
- `CANON_KIT_PROSE_BOUND_CEILING_FILE` — scalar, default empty.

A refused value exits 2 with the knob named, the table's standing rule. canon-kit/SPEC.md §Layout and configuration gains one bullet, written in the collapsed form delta 9 gives that section:

> - `CANON_KIT_PROSE_BOUND_GLOBS`, the governed prose; `CANON_KIT_PROSE_BOUND_SENTENCE_MAX` (default `45` words) and `CANON_KIT_PROSE_BOUND_PARAGRAPH_MAX` (default `200`); `CANON_KIT_PROSE_BOUND_REPEAT_WORDS` (default `6`, at least `3`) and `CANON_KIT_PROSE_BOUND_REPEAT_MIN` (default `3` units, at least `2`); `CANON_KIT_PROSE_BOUND_CEILING_FILE`, empty for no ceiling. Every bound but the unit floor takes `off`. Read by §check-prose-bounds alone.

The corpus-knob paragraph's list of knobs that widen a gate's trigger gains `CANON_KIT_PROSE_BOUND_GLOBS`.

### (3) The dated arm of `check-provenance-seam` {design-bearing}

**Applied.** The `dated` findings inside deltas 6 and 7's sections (one in §The first cohort, three in §The consumer smoke) had their dates dropped, ground kept, when the arm landed, because the arm reds per finding and the battery stays green at every commit; deltas 6 and 7 still own those sections' moves. The dated-attribution bullet's "a date with no marker passes" sentence was a missed site, rewritten to hand that sentence to the new arm. In `native/src/gates/provenance_seam.rs` and canon-kit/SPEC.md §check-provenance-seam. A kit SPEC now carries no date at all, outside a fence.

**The rule.** gate-sdk/SPEC.md §The provenance seam's second paragraph (the one opening "The seam decides the voice, never the content.") keeps its first two sentences and replaces the rest with:

> A kit SPEC carries no date. A measurement that grounds a rule is stated undated, as the ground: the figure and the run that produced it are the landing commit's. A measurement a reader must be able to trust as current carries a `measured:` marker, which an oracle keeps true (canon-kit/SPEC.md §check-measured-claim). A dated figure is neither: it reads as live and is true only of one tree on one day. A specimen spells its date as a placeholder (`YYYY-MM-DD`), or sits in a fence. A change to data a kit writes into a consumer's tree names the release that carried it, never a landing date.

**The arm.** `dated` — an ISO `YYYY-MM-DD` date with no digit abutting it, in a sentence the dated-attribution arm does not already report. Both provenance sets take it, the kit SPECs and the declared seam surfaces. canon-kit/SPEC.md §check-provenance-seam changes to match:

- the **Arms** list gains `dated` after `dated-attribution`;
- the corpus paragraph's list of provenance arms gains `dated`;
- the honest limit "a dated landing or incident label with no attribution passes" is deleted;
- the fixture sentence changes: `good/`'s dated measurement moves to `bad/` as the `dated` trip, and `good/`'s specimen spells its date `YYYY-MM-DD`.

In the same section's `CANON_KIT_SEAM_AUTHORITY_MARKERS` bullet (§Layout and configuration), "adding one reds a dated measurement and a specimen that are not provenance" becomes "adding one reds role vocabulary that is not provenance".

**The sites outside the five sections**, each rewritten undated as the rule above says. The probe is `git grep -n -E '(^|[^0-9])20[0-9]{2}-[01][0-9]-[0-3][0-9]([^0-9]|$)' -- '*/SPEC.md' ':!docs/*' ':!*/gate-tests/*'`, and each site's replacement drops the date and keeps the ground:

- gate-sdk/SPEC.md, one line each in §Porting to Rust does not retire dialect exposure, §The third budget batch, §The settings cohort, and the crate's first dependency, §The declaration cohort, §The port disposition, §upgrade-smoke, §build-native, §check-crate-arms, §check-path-dialect, §check-assertion-strength and §check-graph;
- delegation-kit/SPEC.md, two lines in §The turn-end liveness hook;
- lifecycle-kit/SPEC.md §The committed gap inbox, the specimen `*verified 2026-08-30: …*`, respelled `*verified YYYY-MM-DD: …*`;
- site-kit/SPEC.md §check-docs-render-fidelity, the battery-cycle measurement.

The `dated` findings inside the five sections are deltas 6 and 7's.

### (4) An agent file's paragraphs take the same bound {mechanical}

**Applied.** context-kit/SPEC.md §The brevity gate's closing sentence, "Prose outside any bullet — the paragraph sections of an agent file — is outside the gate's grammar.", becomes:

> Prose outside any bullet, an agent file's paragraph sections, is outside this gate's grammar. Its length is bounded by canon-kit's `check-prose-bounds`, where a consumer lists the agent file in that gate's corpus (canon-kit/SPEC.md §check-prose-bounds).

This repo lists `CLAUDE.md` in the corpus (delta 5). CLAUDE.md's two sentences over the bound are split, and no word of their rules is dropped. The agent-file fixture is delta 1's.

### (5) This repo's binding {mechanical}

**Applied.**

- `scripts/canon-config.knobs` sets `CANON_KIT_PROSE_BOUND_GLOBS` to `*/SPEC.md`, `doctrine-kit/DOCTRINE.md` and `CLAUDE.md`, and `CANON_KIT_PROSE_BOUND_CEILING_FILE` to `.workflow/prose-bound-ceiling.txt`. It binds the four bounds at their defaults in its policy-calibration block, as it binds the prose-tell thresholds.
- `.workflow/prose-bound-ceiling.txt` is tracked, headed `# contract: canon-kit/SPEC.md §check-prose-bounds — <findings> <path>`. It is written last, by delta 10.
- `scripts/gates.list` registers `check-prose-bounds` beside `check-prose-tells`.

### (6) The three moves in gate-sdk §The port-candidate criteria and §The first cohort {design-bearing}

**Not yet applied.** Both sections of gate-sdk/SPEC.md take the three moves. Afterwards neither section has a `check-prose-bounds` finding or a `check-provenance-seam` finding.

- **Keep** every criterion, its test and its engineering ground, and every rule the cohort selection states.
- **Send to history** the dated oracle runs, the per-member accounts of how a port went, and the retirement-record narration of shell forms since deleted. The commits that landed them already carry them. This absorbs `port-archaeology-restatement-residue`, retired to Done at this amendment's pairing.
- **Collapse** a clause repeated on each criterion or cohort member to one lead-in.

Headings keep their text, so every `§` citation into them still resolves. Before editing, the build lists each section's normative sentences in its journal, and after editing it confirms that each survives, here or in the section that already owns it.

### (7) The three moves in installer §The consumer smoke {design-bearing}

**Not yet applied.** installer/SPEC.md §The consumer smoke keeps what the smoke asserts, how it runs and what a red means. The round-by-round debugging log and the dated cost measurement go to history. The same acceptance holds as in delta 6: no finding from either gate in the section, and every normative sentence accounted for. installer/SPEC.md is a declared seam surface, so its three dated lines are `dated` findings until this delta lands.

### (8) The three moves in delegation-kit §Testing {design-bearing}

**Not yet applied.** delegation-kit/SPEC.md §Testing keeps each test's subject and its reason. Its four narrative paragraphs become statements of what each test pins. The acceptance is delta 6's.

### (9) The three moves in canon-kit's and guard-kit's §Layout and configuration {design-bearing}

**Not yet applied.** In each section, a clause stated on each knob bullet becomes one lead-in above the roster. In canon-kit that clause is the provenance-seam ground several bullets restate in their own words ("so no spelling ships as a kit literal", "per the provenance seam it never lands as a kit literal"), and in guard-kit it is the repeated "default empty, in which case …". Each bullet keeps its knob, shape, default and reader. The acceptance is delta 6's. Delta 2's bullet lands in the collapsed form.

### (10) Stamp the ceiling {mechanical}

**Applied over the unmoved sections; re-stamped after deltas 6 to 9.** After deltas 3 to 9, run the worklist spelling from delta 1 and write one `<n> <path>` row per governed file with findings into `.workflow/prose-bound-ceiling.txt`. The five sections contribute none. `check-prose-bounds` is then green at the rows, and every later edit that moves a count moves its row. The sibling amendment `SPEC-capture-drain.md` merges into governed SPECs too, as the capture-home rule did (gate-sdk/SPEC.md §The workflow directory). A sibling merged before this delta has its count absorbed by the stamp. One merged after moves the rows it changes in its merge commit, and the gate refuses that commit until it does.

## Producers and consumers

- **`check-prose-bounds`** — produced by the pre-commit hook and `run-gates.sh` on a governed file or the ceiling file. The committing session reads each finding once. Its knobs are read by this gate alone (delta 2). Roster-holding readers of the new gate name, found with `git grep -l check-prose-tells -- ':!docs/*'`: `scripts/gates.list`, the generated `scripts/git-hooks/pre-commit`, `canon-kit/smoke/install.sh`'s registry heredoc (gate-sdk/SPEC.md §check-gate-substrate-parity assertion J), `canon-kit/README.md`'s gate list, `native/src/gates/mod.rs` and `native/src/knobs/canon_kit.rs`. `.workflow/validate-baseline.txt` gains a `gates check-prose-bounds pass` row; a missing row costs classification only (evidence-kit/SPEC.md §Baseline manifest). `.workflow/release-declarations.md` gains the gate and its knobs as a consumer-visible addition.
- **The ceiling file** — its only writer is a session editing it by hand, prompted by the gate's printed row; its one reader is the gate. `check-workflow-tiering` holds its header.
- **The `dated` arm** — produced by `check-provenance-seam`'s existing triggers; the finding names the arm, as every arm's does. Its red condition is any ISO date outside a fence in the two sets. The count reader it could flip is none: the gate reds per finding and asserts no count.
- **The worklist** — the gate run with the ceiling emptied. `spec-brevity-residue` reads it.
- **Narrowing check (point 5).** No delta narrows a corpus. The prose moves remove findings from gates that red per finding, and `check-measured-claim` arm C and `check-manifest-count` read markers and cardinals the moves leave in place or delete with their sentence. The build runs the full battery after each section.

## Existing sections updated

- canon-kit/SPEC.md §check-prose-bounds (new), §The shared spec adapters and §Per-component contracts (delta 1)
- canon-kit/SPEC.md §Layout and configuration (deltas 2, 3 and 9)
- canon-kit/SPEC.md §check-provenance-seam (delta 3)
- gate-sdk/SPEC.md §The provenance seam (delta 3)
- gate-sdk/SPEC.md, delegation-kit/SPEC.md, lifecycle-kit/SPEC.md and site-kit/SPEC.md at the dated sites (delta 3)
- context-kit/SPEC.md §The brevity gate (delta 4)
- `CLAUDE.md`, its two long sentences (delta 4)
- `scripts/canon-config.knobs`, `scripts/gates.list` (delta 5)
- gate-sdk/SPEC.md §The port-candidate criteria and §The first cohort, and the rule that selects the next (delta 6)
- installer/SPEC.md §The consumer smoke (delta 7)
- delegation-kit/SPEC.md §Testing (delta 8)
- guard-kit/SPEC.md §Layout and configuration (delta 9)
- `.workflow/prose-bound-ceiling.txt` (deltas 5 and 10)
- `canon-kit/gate-tests/check-provenance-seam/` and `check-provenance-seam.test.sh` (delta 3)
- the generated projections: the `docs/` SPEC mirrors, the pre-commit hook, `docs/enforcement.md` and the coupling graph (all deltas)

## Retired spellings

- None — no delta retires a name; the seam's dated-measurement allowance is prose, and its sentences are rewritten in place (delta 3).

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for the gate, its knobs, the ceiling file and the `dated` arm.
- [ ] **Instruction surfaces: instruction only** — no template, agent definition or shim gains grounds.
- [ ] **Merged with no information lost** — each delta re-phrases the text it refines, and every normative sentence of the five sections is accounted for in the build journal.
- [ ] **The gates are green on the tree** — `check-prose-bounds` at its ceiling rows, with no finding in the five sections, and `check-provenance-seam` with no `dated` finding anywhere.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the root (`ls SPEC-*.md`).
- [ ] **Removals propagated** — `## Retired spellings` holds, and `check-amendment-retired-spelling` runs it.
- [ ] **The entries move** — both entries this file pairs move to Done before the drain stage, their deliverables finished: the gates, the five sections and the agent-file bound.
- [ ] **Gaps filed** — any cross-component gap found during the work filed through the gap inbox.
