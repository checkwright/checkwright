# SPEC amendment: duplicate-text

Pairs `cross-file-duplicate-text`. An adopter asks for a gate holding that no run of N or more words is shared across a declared file set, the goal being the same text repeated across always-loaded files. This amendment adds `check-duplicate-text`, a canon-kit gate off until a consumer declares its file set. It reads the word stream `check-prose-bounds` assertion C already reads, lifted into the shared adapters so both gates read one stream, and it rewrites §check-docs-page-repeat's *a repeat across pages* not-asserted sentence to point at it.

**What the tree does today**, read from `canon-kit/SPEC.md`, `native/src/gates/prose_bounds.rs`, `native/src/gates/shim_restatement.rs` and `native/src/spec.rs`:

- **No gate holds a verbatim run across a consumer's file set.** §check-docs-page-repeat reads one page at a time and states *a repeat across pages* as deliberately not asserted. §check-prose-bounds assertion C reads one section of one file. `check-shim-restatement` reads binding shims against its own dedup corpus. `git grep -n -i "cross-file\|across files\|shared run" -- '*/SPEC.md' native/src` returns cross-file links and comments, no gate section asserting one.
- **The prose-aware word stream exists, private to one gate.** `check-prose-bounds` cuts each file into units (a blank-line block, each list item its own), holding out headings, table rows, generated `<!-- name:begin -->` regions and comment-only lines, while `spec::walk_prose` drops fenced code. Its `phrase_words` then folds case and drops code spans, link targets, path-shaped tokens and punctuation. The unit builder, `phrase_words`, `drop_link_targets`, `path_shaped` and `comment_only` are private functions of `native/src/gates/prose_bounds.rs`.
- **`check-shim-restatement`'s reading is the raw document.** Its `ngrams` takes every ASCII alphanumeric run in the file as a word and runs one stream across the whole document, fenced code, link targets, table rows and generated regions included. That suits its corpus, shims and the templates they bind, and would make every shared code block, link target and kit-generated block a finding here.
- **A front-matter block is read as prose by `check-prose-bounds`.** Neither `walk_prose` nor that gate's sink skips it, where `spec::front_matter_span` is the reading every page gate that skips one takes.

## What changes

### (1) The prose unit and phrase reader join the shared spec adapters {mechanical}

Move `check-prose-bounds`' unit reading and its phrase stream out of `native/src/gates/prose_bounds.rs` into `native/src/spec.rs`: the unit (its text, its first line and its byte-to-line map), the unit-cutting sink, the code-span collapse, `phrase_words` and its two helpers. `check-prose-bounds` calls them unchanged. The move changes no behaviour: its fixture pair, its unit tests and `check-prose-bounds.test.sh` pass unedited, and they are the oracle.

Replacement text for §The shared spec adapters' last bullet. **Not yet applied:**

> - **The prose unit, the sentence split and the phrase stream.** §check-prose-tells, §check-prose-bounds and §check-duplicate-text cut a block into units on one set of line tests: a list item opens a unit, and a heading, a table row, a generated-region marker or a line inside a region, and a comment-only line close one and are held out. A sentence closes at a run of `.`, `!` or `?` followed by whitespace or the span's end. The **phrase stream** is a unit's words with case folded and code spans, link targets, path-shaped tokens and punctuation dropped, so a shared citation or knob name is no shared prose. §check-prose-bounds assertion C and §check-duplicate-text read it, and each word carries its physical line.

### (2) A new gate, `check-duplicate-text` {design-bearing} {user-facing: scope's seam decision on the paired entry (2026-11-28, lead-delegated, not a ruling) settles one canon-kit gate, a consumer glob set and run length, and the empty default that turns it off; the lead's spec dispatch (2026-11-28, not a ruling) delegated the within-file question, the defaults, the matching rules and the output shape to this stage, settled here}

A new section, `### check-duplicate-text`, placed after §check-docs-page-repeat. **Not yet applied:**

> Invariant: no run of `CANON_KIT_DUPLICATE_TEXT_MIN_WORDS` words or more is stated in two declared files. A fact stated in two always-loaded files has two homes, and an edit to one leaves the other silently behind.
>
> - **Corpus:** files matching `CANON_KIT_DUPLICATE_TEXT_GLOBS` less those matching `CANON_KIT_DUPLICATE_TEXT_EXCLUDE`, each expanded through the pruning walk like every canon-kit glob knob, byte-sorted by path. An empty expansion is a clean `0 file(s)`. A file is read as markdown whatever its extension.
> - **The words** are the phrase stream (§The shared spec adapters): prose units, headings, table rows, fenced code, generated regions and comment-only lines held out, then case folded and code spans, link targets, path-shaped tokens and punctuation dropped. The front-matter span (§The shared spec adapters) is held out too. So a block a kit generates into several files is never read, and neither is a shared link target, command or knob name.
> - **A run** is a window of `CANON_KIT_DUPLICATE_TEXT_MIN_WORDS` consecutive words inside one unit. A window never crosses a unit boundary, since words either side of a held-out line or a paragraph break are no one statement. Windows overlapping in both units are one run, reported once at its full length.
> - **Two files, by default.** A run in two units of one file is a finding only under `CANON_KIT_DUPLICATE_TEXT_WITHIN_FILE=on`. At `off`, the default, a repeat within one file stays the within-page and within-section gates' (§check-docs-page-repeat, §check-prose-bounds assertion C). A run repeated inside one unit is never read.
> - **Valve:** `duplicate-text-exempt: <reason>`, in the shared exempt window (§The shared spec adapters). A valved line is out of the walk, so the valve exempts its copy whichever occurrence it is. The reason is mandatory: a valve without one exempts nothing, and the finding names it.
>
> **Red** is one finding per later occurrence, at the line its run starts, naming the first occurrence's file and line and quoting the run, cut after its first `CANON_KIT_DUPLICATE_TEXT_MIN_WORDS` words with its full word count. Files are compared in path order, so the first occurrence is the earliest file's earliest unit. The `help:` line is *state it once, in the file whose tier owns it, and point to that home from every other: a link or a §section citation*. Which file owns a fact is the consumer's tier contract, never the gate's, so the finding names both and decides neither. The clean line counts files, units and the run length. **Exit 3:** an unreadable file. **Exit 2:** a knob the table validator refuses (§Layout and configuration). `tier=precommit`, `install: zero-config`, armed by `CANON_KIT_DUPLICATE_TEXT_GLOBS`.
>
> **Honest limits.** A paraphrase shares no run and passes, and a run shorter than the bound passes, though both are the same defect. A multi-line HTML comment's inner lines are prose to the unit reader. A file in both this corpus and §check-docs-page-repeat's, under `on`, has a repeated sentence long enough for both reported once per gate.

Knob bullets for §Layout and configuration, after the `CANON_KIT_PAGE_LENGTH_*` bullet. **Not yet applied:**

> - `CANON_KIT_DUPLICATE_TEXT_GLOBS` / `CANON_KIT_DUPLICATE_TEXT_EXCLUDE` — arrays of globs, default empty: the files `check-duplicate-text` compares, less the excluded ones, such as a generated copy that restates its source by construction. `CANON_KIT_DUPLICATE_TEXT_MIN_WORDS` — default `12`, an integer of at least `3`: the shortest shared run. `CANON_KIT_DUPLICATE_TEXT_WITHIN_FILE` — `off` (default) or `on`: whether a run in two units of one file counts.

- **Why `12`.** Over this tree's specs, READMEs and agent file, sampled maximal shared runs exactly 8 to 10 words long are mostly stock phrasings of one vocabulary, *the next heading of the same or a* and *at exit 2 with every finding*, which two independent sections write alike. At 12 the sampled runs are restatements, knob-file boilerplate copied between kit specs. The floor of 3 refuses a width at which ordinary function-word pairs collide everywhere. `check-shim-restatement`'s default of 9 rests on a corpus whose citations are never dropped. This stream drops path-shaped tokens, so a citation contributes only its heading's words.
- **Why within-file is a choice, off by default.** Scope's seam decision leaves a repeat within one file to §check-docs-page-repeat. The adopter's first message also asked for one, and nothing else reads a long run repeated across sections of one file: the page gate compares whole sentences on declared pages, and assertion C stays within a section. So a consumer selects it, and the default keeps scope's reading.
- **Why an exclude knob beside the glob set.** A glob cannot subtract, and a generated mirror or agent index under the same glob restates its source verbatim by construction, so a consumer naming `**/*.md` could otherwise not hold its own prose without reding on every projection. The pruning walk prunes build directories, never a mirror.
- **Ruled out: sharing `check-shim-restatement`'s reader, or folding that gate in.** The entry's seam decision favoured lifting that gate's word reading. Its reader is the raw document, so it reads exactly what this gate must hold out. Converging it on the phrase stream would change that gate's normalization contract, which its own kit owns, and a shim's fenced text is part of what it binds. Folding moves another kit's gate into canon-kit. Both stay out, and `native/src/gates/shim_restatement.rs` is untouched.
- **Ruled out: a new arm of §check-docs-page-repeat.** That gate's unit is a page, its grain a whole sentence and its corpus docs pages. This gate's subject is a file set of any kind at a word-run grain, and one gate holding both would print one clean line for two subjects.

Declared by `checks/check-duplicate-text.gate`, `# install: zero-config`, `# armed-by: CANON_KIT_DUPLICATE_TEXT_GLOBS`, dispatching to the binary, with `knob:CANON_KIT_DUPLICATE_TEXT_GLOBS` in `couples=`. The pair: `good/` holds two files sharing a run one word below the bound, a shared fenced block, a shared link target, one generated region in both, a front-matter block in both, a valved shared run, and a run repeated in two units of one file under the default. `bad/` holds a run shared by two files and repeated in a third, a wrapped paragraph whose run starts on its second line, and a reasonless valve. `check-duplicate-text.test.sh` holds the within-file arm at `on`, the exclude knob, and the exit-3 unreadable file, which one pair at its default cannot express.

### (3) §check-docs-page-repeat's not-asserted paragraph points at the new gate {mechanical}

Replacement text for the paragraph's first three sentences. **Not yet applied:**

> **Deliberately not asserted: a page-grain link rule, or a repeat across pages.** A page citing several sections of one README downward is the anchored shape §check-docs-link-convention requires, so the grain is the target, fragment included. A verbatim run across pages is §check-duplicate-text's, over the pages a consumer declares there. A paraphrase, the usual cross-page restatement, is asserted by neither, since no scanner decides one.

The rest of the paragraph, from *Chrome is no other page*, is unchanged.

### (4) This site's page-authoring rule names the verbatim half's gate {mechanical}

Replacement text for `docs/page-authoring.md`'s *One statement, one home* paragraph, from *The across-page half is review's* to its end. **Not yet applied:**

> The across-page half is review's. `check-duplicate-text` asserts the verbatim part over a declared file set, and this site declares none: its verbatim repeats across pages are sibling-page chrome, which the rule admits, and a cross-page restatement is otherwise usually a paraphrase, which no scanner decides.

## Producers and consumers

- **`CANON_KIT_DUPLICATE_TEXT_GLOBS` and `CANON_KIT_DUPLICATE_TEXT_EXCLUDE`** (new indexed knobs) — producer: the consumer's knob file, default empty. Consumers: the gate's corpus walk; `check-graph` and the trigger index through the `knob:` couples token; `doctor` through `# armed-by:`.
- **`CANON_KIT_DUPLICATE_TEXT_MIN_WORDS`** (new scalar, default `12`) and **`CANON_KIT_DUPLICATE_TEXT_WITHIN_FILE`** (new scalar, default `off`) — producer: the knob file or the environment. Consumers: the table validator, which refuses a width below `3` or not an integer and a mode other than `off` or `on` at exit 2 with the knob named, and the gate's window and pairing rules.
- **The finding** (new event) — producer: the gate. Fields: the later file and line (read by the committing session to find the copy), the first file and line (to find the home candidate), the quoted run and its count (to judge it). Consumers: the battery and the pre-commit hook through exit 1, and the SARIF log through the runner.
- **The `duplicate-text-exempt:` valve** (new marker) — producer: a declared file's author. Consumer: the gate, through the shared exempt window, and its own reasonless-valve check. It sits in a markdown HTML comment, which `check-comment-tier`'s surfaces do not read, so that gate's site-valve roster does not gain it.
- **The shared phrase stream and unit reader** (new shared adapters, delta 1) — producer: `native/src/spec.rs`. Consumers: `check-prose-bounds` (its units and assertion C) and `check-duplicate-text` (delta 2), which also reads each word's physical line and the front-matter hold-out the stream gains for it. `check-prose-bounds` takes neither: it reports a unit's first line, and holding out front matter there would change its corpus.
- **The registry row** (new member) — producer: `native/src/gates/mod.rs`. Consumers: the direct form, `--run`, `--needs`, the descriptor's dispatch, `check-graph`, and the reads declaration `(".", "glob:knob:CANON_KIT_DUPLICATE_TEXT_GLOBS", "", "")` with every knob the member reads listed, the prune pair included, as `check-prose-bounds`' row lists its own.
- **Point 5 (narrowing).** Delta 1 narrows no corpus. Delta 2's exclude knob narrows only this gate, whose verdict is monotone in its violation set, and it asserts no count or floor. Deltas 3 and 4 change prose only.
- **Point 6 (every member).** The members are every reader of the claim that a cross-page repeat is unasserted. Probe: `git grep -n -i "repeat across pages\|across-page half\|cross-page restatement" -- ':!docs/posts' ':!TASK-QUEUE.md'`, which returns §check-docs-page-repeat's paragraph, its mirror, and `docs/page-authoring.md`'s paragraph. Delta 3 rewrites the first, its regeneration the second, delta 4 the third.

## Existing sections updated

Roster from `git grep -l "check-docs-page-length" -- ':!docs/posts'`, the analog new canon-kit gate's sites, joined with the point-6 probe above and `grep -n "phrase_words\|build_unit\|path_shaped\|drop_link_targets\|comment_only" native/src/gates/prose_bounds.rs`.

- `canon-kit/SPEC.md` — §The shared spec adapters' last bullet (delta 1); §Layout and configuration's knob bullet and the new §check-duplicate-text (delta 2); §check-docs-page-repeat's not-asserted paragraph (delta 3).
- `docs/canon-kit/SPEC.md` — the generated mirror, regenerated by `--emit docs-mirror --write` (all deltas).
- `native/src/spec.rs` — gains the unit reader, the phrase stream and the code-span collapse (delta 1), and each word's line and the front-matter hold-out (delta 2).
- `native/src/gates/prose_bounds.rs` — loses the moved functions and calls the shared ones; its unit tests on them move with them (delta 1).
- `native/src/gates/duplicate_text.rs` — new module; `native/src/gates/mod.rs` gains its `pub mod` and registry row (delta 2).
- `native/src/knobs/canon_kit.rs` — two indexed rows, two scalar rows, and `validate`'s width floor and mode refusals, each with a unit case (delta 2).
- `canon-kit/checks/check-duplicate-text.gate` — new descriptor (delta 2).
- `canon-kit/gate-tests/check-duplicate-text/good/` and `bad/`, each with a fixture `scripts/canon-config.knobs`, and `canon-kit/gate-tests/check-duplicate-text.test.sh` (delta 2).
- `canon-kit/SPEC.md` §Per-component contracts' bespoke unit-test roster gains `check-duplicate-text.test.sh` (delta 2).
- `canon-kit/README.md` and its mirror `docs/canon-kit/README.md` — a gate-roster line beside `check-docs-page-repeat`'s, *needs declared files (no long run of words stated in two of them)* (delta 2).
- `canon-kit/smoke/install.sh` — the gate joins the registration list (delta 2).
- `scripts/gates.list` — registered under the dogfooding rule and left unbound, as `check-rule-id-coverage` is, so it reads clean-skipped here. Its verbatim repeats across `docs/` are sibling-page chrome (delta 2).
- `docs/page-authoring.md` — the *One statement, one home* paragraph (delta 4).
- Generated projections the new member moves: `docs/enforcement.md`, `docs/check-graph.html`, `docs/llms-full.txt`, and any rollup or size figure reading them, each regenerated by the command its freshness gate prints on red (docs/generated-projections.md) (delta 2).
- `.workflow/surface-ceiling.txt` — the surface ratchet's row for `canon-kit/SPEC.md`, moved in the growing commit (delta 2).
- `.workflow/release-declarations.md` — *Knob changes* gains the four knobs and *Behavior changes* the gate (delta 2).

## Retired spellings

- None — the gate, its knobs and the shared adapters are additions; delta 1 moves functions without renaming them, and deltas 3 and 4 re-phrase prose that names no retired spelling.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for the four knobs, the finding, the valve, the shared adapters and the registry row.
- [ ] **Instruction surfaces: instruction only** — no template, agent definition or shim is touched.
- [ ] **Merged with no information lost** — the new section lands whole, and the adapter bullet and the two not-asserted passages re-phrase the text they refine; the merged spec reads as one document.
- [ ] **Behaviour preserved** — delta 1 lands with `check-prose-bounds`' fixture pair, unit tests and bespoke test passing unedited, and the battery's `check-prose-bounds` verdict on this tree unchanged.
- [ ] **Calibration re-measured** — the gate run over `CLAUDE.md`, `*/README.md`, `*/SPEC.md` and `README.md` at widths 8, 10 and 12 confirms the default's grounds in delta 2, or the default and its grounds move with what it returns.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls canon-kit/SPEC-*.md`).
- [ ] **Removals propagated** — none declared; `check-amendment-retired-spelling` stays clean.
- [ ] **Entry moved** — `cross-file-duplicate-text` to Done with `--queue done` in the build batch's merge commit, before the drain stage. Its oracle is local, the canon-kit fixture suite, the smoke and the battery, so no remote run gates the move.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks.
