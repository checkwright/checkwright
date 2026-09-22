# SPEC amendment: md-unwrap

This repo hard-wraps its markdown, and nothing owns the convention or gates it. The wrap is bimodal: `TRAJECTORY.md` alone mixes lines at 79 to 81 columns with lines at 92 to 100, and `CLAUDE.md` carries unwrapped one-line bullets beside four-line wrapped ones. The wrap is also load-bearing, and no manifest says so. Every size measure that counts **lines** is only a size measure while lines are width-bounded, and `check-queue-wrap` is the only thing bounding width, over the queue file alone. The operator's direction on this unit (2026-09-22, lead-relayed) is that tracked markdown carries no artificial line cap, and that a tool needing a line-capped form gets one generated from the uncapped tracked file. Under enforcement-first the unwrap and its oracle land in one unit or neither does.

This amendment does four things.

1. It ships the oracle: a canon-kit gate that reds a paragraph broken across lines, and a canon-kit arm that performs the join. They share one block scanner, so the gate reds exactly the lines the arm would join.
2. It re-units every line-counting size measure to **code points**, a measure that reflow does not move. This is the precondition for dropping any width cap.
3. It makes the queue's per-entry cap a **selectable** measure, with `off` as a valid choice.
4. It rewrites this repo's tracked markdown, **except the queue file**. The queue's grammar is line-scoped by design, so the queue unwraps inside `queue-kit/SPEC-queue-headings.md`'s conversion, which rewrites every entry anyway, and `check-queue-wrap` stays registered on it until then.

**Merge order.** This amendment merges before `queue-kit/SPEC-queue-headings.md`, whose entry is blocked on this one's.

**The kit posture: both conventions stay supported, and this repo binds one.** A kit ships generic mechanism. `check-queue-wrap` keeps shipping for a consumer that wraps, `check-md-unwrapped` ships for one that does not, and each consumer registers the one it wants, as the entry specified. The same posture governs the record cap, per the operator's directions (2026-09-22, lead-relayed): the kit may support several capping approaches, some consumers will want no cap at all, and this project adopts exactly one. The cap knob therefore takes a unit, and `off` is one of its values (delta 3). This repo binds code points. A lead-grantable per-record exception is **not** in this amendment. It would reverse §check-queue-entry-budget's stated refusal of a conditional cap ("Why the cap is not widened for exceptional content"), which is wider than this unit's envelope, and it is escalated to the lead.

**Why code points.** The unit was ruled lead own-authority on 2026-09-10 and stands. `cplen` in `queue_wrap.rs` is the in-tree precedent. Bytes penalise the em dashes and section marks this prose is full of. Words would need a tokenizer decision. A code-point count of an entry's text, with line breaks counted as one and indentation trimmed, differs between the wrapped and unwrapped forms by essentially nothing. So the measure survives this very rewrite and any later reflow, which a line count cannot.

**Measured at authoring (2026-09-22).**

- **The entry's six named arms are half wrong, and the real set is different.** A read of each module shows that `overhead_meter.rs` sums bytes of session transcripts, `scan_prompts.rs` ranks logged shell commands, and `port_blockers.rs` counts `wc -l` over tracked `.sh` files. None of the three measures markdown by lines, and none changes here. The markdown line-as-size measures are `native/src/emit/always_loaded.rs` (the meter and the ratchet's size), `native/src/emit/footprint.rs` (both tiers), `native/src/gates/brevity.rs` (a bullet's span), and `native/src/gates/queue_entry_budget.rs`. context-kit/SPEC.md §The brevity gate states that the brevity gate, the meter and the ratchet measure "one quantity", so they re-unit together. `native/src/emit/queue_index.rs` prints an entry's line extent in the worklist, for display only. `md_index.rs` counts lines for navigation and stays.
- **Width readers.** `check-queue-wrap` measures code points over the queue. `check-audit-roster` caps bytes per line over `.workflow/audit-roster.txt`, a `key: value` record file and not markdown, so it is outside this amendment.
- **No `.metric/` series records a line count.** `grep -l loaded .metric/*` finds only the stage-economics log, which names an iteration. The history that does break is tracked: `.workflow/always-loaded-baseline.txt` (`184 176 7314a4da…`, in lines) and `.workflow/surface-ceiling.txt` (`<lines> <path>` rows). The meter's baseline delta and the ratchet's verdict would read a phantom fall on the first run after the re-unit. Delta 4 records the discontinuity in the row format itself.
- **Corpus.** `git ls-files '*.md'` counts 437 files. 289 of them are gate-test fixtures (`*/gate-tests/*`, including `scripts/gate-tests/`), 25 are shipped templates and 13 are `.claude/` agent and command files. The generated projections are the SPEC and README mirrors under `docs/<dir>/`, `docs/footprint.md`, `docs/enforcement.md`, `docs/value.md`'s rollup block, `docs/evidence-data.md`, `docs/install-evidence.md` and `ROADMAP.md`'s marker block (docs/site-architecture.md §Generated projections and their freshness gates).
- **Rendering hazards are nil.** No tracked markdown line ends in two spaces. The 20 trailing-backslash lines are all shell continuations inside fences. Four files carry a multi-line HTML comment: `.github/pull_request_template.md`, `canon-kit/SPEC.md`, its mirror, and `canon-kit/templates/SPEC-amendment.md`.
- **Line-led declarations outside the queue already stand alone.** `git grep -n -B1 -A1 -E "^(close-surface|door-contributor):" -- '*.md' ':!docs/' ':!*/gate-tests/*'` shows every `close-surface:` line bounded by blank lines or a fence, so none is a paragraph continuation the join could absorb.
- **No markdown line-number cite exists outside the queue** (`git grep -n -E "[A-Za-z0-9_./-]+\.md:[0-9]+"` over the tracked tree less the queue, fixtures and Rust test strings finds two example outputs in `README.md` and `docs/index.md`). The queue's eight cites are `queue-kit/SPEC-queue-citations.md`'s.
- **Calibration of the two re-united caps.** The probe measured each top-level Deferred entry of the queue at HEAD two ways: its non-blank lines less one declaration line per grammar, and delta 3's code-point size over the same lines. The pool has 63 entries at a median of 85 code points per counted line. The four largest (48 or 49 counted lines) measure 3963 to 4211 code points. The same code-point rule over each bullet of `CLAUDE.md`'s two governed brevity sections gives 264 to 327 for the four-line bullets carrying a `§`, and at most 272 for the one-line ones.

## What changes

### (1) canon-kit: `check-md-unwrapped` and `--emit md-unwrap` {design-bearing}

**Not yet applied.** Add a canon-kit section `### check-md-unwrapped` after §check-md-refs:

> Invariant: in the governed markdown set, no paragraph is broken across physical lines. Each paragraph, list-item paragraph and block-quote paragraph is one line, however long. A consumer that wraps registers queue-kit's `check-queue-wrap` instead, and a consumer that does neither registers neither. Which convention a tree keeps is its own choice, and this gate is the oracle for one of them.
>
> **The block scanner.** A line is a **soft break**, and red, when it is non-blank and continues the paragraph the previous line opened. That is, it opens no block, and the previous line is paragraph text in the same container. A line **opens a block** when it is an ATX heading, a fence delimiter, a table row (`|`-led), a thematic break or setext underline (`---`, `***`, `___`, `===`), a list-item marker (`-`, `*`, `+`, or `<digits>.`/`<digits>)` followed by a space, at any indent), a block-quote marker `>` whose quoted content itself opens a block, an HTML block start (`<` followed by a tag name, `!--` or `/`), or a link-reference definition (`[label]:`). The interiors of fences, HTML blocks (a comment runs to its `-->`, and any other HTML block runs to a blank line) and a leading `---`-delimited front-matter block are outside the grammar. A line after one ending in a hard break (two spaces, or a backslash outside code) is deliberate and exempt. Inside a block quote, the comparison is made on the quoted content. The scanner is a CommonMark subset: an indented code block is read only where no list is open, and lazy continuation inside nested containers is joined onto the innermost open paragraph. Anything the subset does not model is an honest limit, and the fixture pair pins what it does model.
>
> **Corpus.** Tracked files matching `CANON_KIT_UNWRAP_GLOBS` less those matching `CANON_KIT_UNWRAP_EXCLUDE`, with git pathspec semantics and both default empty. An empty include set is a clean skip whose clean line names the unset knob, because which files a tree keeps unwrapped is its own editorial choice and a kit literal would presume a layout. Gate-test fixtures are the canonical exclusion: a fixture's line shape is the input under test.
>
> **Findings** name file, line and the first code points of the offending line, and the help text prints the arm's command. **Fail-closed:** an unreadable file or an unresolved knob exits 2.
>
> **`--emit md-unwrap [--write] <file>…`** is the gate's remedy and shares its scanner. Each soft break is joined onto its predecessor with one space, and the continuation's indentation, or its `>` prefix inside a quote, is dropped. The arm prints the rewritten text, or with `--write` rewrites each file in place, checked, naming a failed path at exit 2. It is a non-gate arm (gate-sdk/SPEC.md §The non-gate arm). Its postcondition is the gate's clean verdict on its own output, and a unit test holds it over both fixture trees. A join never crosses a blank line, a block start or an exempt interior, so it changes no rendering the subset models.
>
> **A tool that needs a line-capped form** reads one generated from the uncapped tracked file, rostered and freshness-gated as every generated projection is. It is never a hand-wrapped tracked copy. No such reader exists in this repo today, so the rule has no instance here.
>
> **The width-straddling readers stay.** `check-manifest-count`'s wrapped-paragraph arm, `check-unmarked-claim`'s flattened paragraph, `check-spec-pointer`'s prose-citation join and `check-scratch-citation`'s paragraph accumulator each cross a line break so that a wrapped token still matches. Over an unwrapped corpus each is the identity, and each is still correct for a consumer that wraps. Removing them is refused, because a kit gate may not assume its consumer's convention.

In §Layout and configuration, add `CANON_KIT_UNWRAP_GLOBS` and `CANON_KIT_UNWRAP_EXCLUDE` (arrays, default empty) to the knob roster. They join the corpus-knob list, and its count moves by two. Add the gate to the governed-gate roster wherever the section lists canon-kit's gates. Wiring: a `canon-kit/checks/check-md-unwrapped.gate` descriptor (`couples=` both knobs, `tier=precommit`), `native/src/gates/md_unwrapped.rs` and the scanner as a shared function the arm calls from `native/src/emit/md_unwrap.rs`, both registered in their `mod.rs`, knob rows in `native/src/knobs/canon_kit.rs`, and a `good/`+`bad/` fixture pair. `bad/` holds a wrapped paragraph, a wrapped list item and a wrapped quote. `good/` holds every block start above, a fence and a multi-line HTML comment, a hard break, front matter and a table. In §check-md-refs, "(a space, or a line break — the scan is per-line)" becomes "(a space)", because a line break inside a paragraph is what this gate reds.

### (2) The unwrap-aware wording in kit SPECs that assumed this repo wraps {mechanical}

**Not yet applied.** Kit mechanism that reads across a wrap stays, as delta 1 rules. What changes is text that states *this repo's* wrap as an authoring target or remedy:

- queue-kit/SPEC.md §check-queue-wrap: "(default 100 columns; the authoring target is ~80)" becomes "(default 100 columns)", and the section opens by naming itself as the wrapping consumer's gate, with `check-md-unwrapped` as the alternative (canon-kit/SPEC.md §check-md-unwrapped).
- queue-kit/SPEC.md §check-queue-hygiene's wrapped-fragment paragraph is prefixed "Under a wrapped queue," because an unwrapped paragraph cannot share a fragment line.
- context-kit/SPEC.md §The brevity gate's first honest limit is replaced by delta 4's text.

### (3) queue-kit: a selectable per-entry cap, `QUEUE_KIT_ENTRY_CAP`, retiring `QUEUE_KIT_ENTRY_LINE_CAP` {design-bearing}

**Not yet applied.** In §Layout and configuration, replace the `QUEUE_KIT_ENTRY_LINE_CAP` bullet with:

> - `QUEUE_KIT_ENTRY_CAP` — default `4250cp`: the per-entry size cap `check-queue-entry-budget` assertion A holds over the deferred section, as a count and a unit. `<n>cp` measures code points, `<n>lines` measures counted lines, and `off` disables assertion A while assertions B to E still run. A value outside that grammar is malformed config (exit 2). `lines` is a sound measure only where every line is width-bounded, so choose it beside `check-queue-wrap` and never beside `check-md-unwrapped`.

Add the retired pair (`QUEUE_KIT_ENTRY_LINE_CAP`, `QUEUE_KIT_ENTRY_CAP`) to `native/src/knobs/queue_kit.rs`'s `retired` table, so a consumer file still setting the old name exits 2 naming the successor (gate-sdk/SPEC.md §The knob file). Validate the new grammar in the table validator.

In §check-queue-entry-budget, assertion (A) becomes:

> - **(A) Size.** No deferred entry exceeds `QUEUE_KIT_ENTRY_CAP` in its unit. An entry's **extent** is the range the `queue-index` arm's `--extent` yields, so the range the gate measures is the range an eviction deletes. Its **size** is measured over that extent less **at most one line of each declaration grammar the queue format defines** (§The tag algebra), and nothing else is discounted. In code points, each remaining non-blank line counts its code points with leading and trailing whitespace trimmed, plus one for each break between two of them, so a reflow of the same text moves the size by nothing. In lines, the size is the count of remaining lines. A sub-task is measured as its own entry too and claims its own discount. An inferred-marker line is counted, being content rather than a declaration.

The calibration paragraph becomes:

> Calibration: `4250cp` is the former default of 50 lines times the deferred pool's median of 85 code points per counted line (2026-09-22), so the re-unit moved the unit and not the bound. The cap's job is to keep compression from regrowing, not to force the initial cut, and it is set at the tail of a real pool. It is a stated policy with a stated purpose, not a derived value, which is why it is a knob. **The unit is the consumer's choice and so is having a cap at all**, because which measure bounds a record is policy rather than mechanism. The kit ships the measures and this repo binds one. The scan is line-local: the cost field's bold lead-in must sit on one line, the same reason a tag must sit on its lead line (§check-tag-lead-line).

Every other mention of "counted lines" or "N lines" as the cap's unit in the section moves to "size in the configured unit": the clean-path headroom rows, the `entry-history` rows' two counts, the calibration of the declaration discount ("fixed-shape and width-bounded" becomes "fixed-shape"), and *Why the cap is not widened*. The discount stays **one declaration line per grammar**, measured in the configured unit. `native/src/gates/queue_entry_budget.rs` parses the knob into a unit and a count, measures in it, prints headroom and findings with the unit suffix, and skips (A) under `off`. `native/src/emit/entry_history.rs` reports in the same unit because it calls the same walk. `native/src/emit/queue_index.rs` prints an entry's size in the configured unit in place of `{:>4}l`. §The queue-index arm's "carrying the entry's line count" becomes "carrying the entry's size in the cap's unit", and under `off` it prints code points. §The tag algebra's "Its own ceiling is `check-queue-wrap`'s budget" paragraph on `recurrence:` drops the width clause: an unwrapped line has no width ceiling, and the at-most-one-line rule is what bounds the declaration.

§check-queue-wrap's **coupling paragraph** ("The coupling it underwrites is the stronger ground…") is replaced by:

> **The coupling it underwrites is conditional.** Where `QUEUE_KIT_ENTRY_CAP` is in `lines`, this gate is that cap's denominator, and deregistering it unbounds the cap. Under `cp` or `off` the entry cap is width-independent, and this gate is the wrapping convention's own tripwire and nothing more.

The fixture pair of `check-queue-entry-budget` moves to the new knob. `bad/` keeps an over-cap entry at the default, and a `.test.sh` case pins `lines` and `off`. This repo's `scripts/queue-config.knobs` sets nothing, so the default binds here.

### (4) context-kit: the meter, the ratchet, the brevity gate and the footprint measure code points {design-bearing}

**Not yet applied.**

- §The always-loaded meter: "the summed line count" becomes "the summed code-point count", measured with the same count delta 3 defines (trimmed non-blank lines plus one per break), over the surfaces and over the hook body's stdout. The baseline row becomes `<total>cp <surface>cp <baseline-commit>`. **A row whose figures carry no `cp` suffix is a line-unit row from before the re-unit.** The bare reading then prints `baseline in retired line unit — re-stamp with --update-baseline` in place of a delta, and exits 0. That discontinuity is recorded where the reader meets it, never as a phantom fall. `--growth` reports net code points.
- §The surface ratchet: "size = newline count, the meter's measure" becomes "size = the meter's code-point count". The ceiling rows become `<n>cp <path>`, and a row without the suffix exits 2, naming `--emit always-loaded --ceiling` as the re-stamp.
- §The brevity gate: the per-bullet budget is in code points. `CONTEXT_KIT_BREVITY_BUDGET` is retired for `CONTEXT_KIT_BREVITY_CAP`, a positive integer of code points, default `330`. Every governed bullet the four-line budget admitted with a pointer measures at most 327 code points (2026-09-22), so the re-unit keeps the bound. The first honest limit is replaced by: "**The unit is the code point.** A bullet is measured the way the meter measures a surface, so joining or wrapping its lines moves nothing, and the apparatus still measures one quantity." The finding line reports code points.
- §bin/footprint: the Numbers ruling becomes "Code-point counts are exact". The cell reads `<n>cp · ~<t>t`, and both tiers count with the meter's measure.
- drift-kit/SPEC.md §Bundled KPIs, `kpi-always-loaded`: the trend fragment reads `loaded <n>cp`.

Wiring: `native/src/emit/always_loaded.rs` (the measure, the row grammar and the legacy-row branch), `native/src/gates/surface_ratchet.rs`, `native/src/gates/brevity.rs`, `native/src/emit/footprint.rs` (the `Tally` field and `cell`), `native/src/emit/kpi/always_loaded.rs`, and `native/src/knobs/context_kit.rs` (the new row and the retired pair). The fixture pairs and `.test.sh` scripts of `check-brevity` and `check-surface-ratchet` move to code points, and the ratchet's test adds a legacy row reddening at exit 2. `context-kit/templates/close-brevity.md` step 1's "grew net" gains "code points". **In the re-unit commit**, `.workflow/always-loaded-baseline.txt` is re-expressed at its own recorded commit (the surface half from that commit's blobs, and the hook half measured now, since a past hook body is not recoverable, as the stale mark already concedes), and `.workflow/surface-ceiling.txt` is re-stamped with `--ceiling`. `docs/footprint.md` and `docs/value.md` regenerate.

### (5) This repo registers the gate and rewrites its markdown, the queue file excepted {mechanical}

**Not yet applied.** This lands after deltas 3 and 4, so the re-united measures read the rewrite as the near-zero size change it is.

- `scripts/canon-config.knobs` sets `CANON_KIT_UNWRAP_GLOBS[] = *.md` and `CANON_KIT_UNWRAP_EXCLUDE[]` to `*/gate-tests/*` and `TASK-QUEUE.md`. The queue entry is removed by `queue-kit/SPEC-queue-headings.md` when the queue converts. `scripts/gates.list` registers `check-md-unwrapped`, and the pre-commit hook regenerates.
- `--emit md-unwrap --write` runs over every file the corpus resolves. Each generated projection is then regenerated by its printed command, and **each generator that emits prose emits each paragraph on one line**. The members are the emitters behind `docs/footprint.md`, `docs/enforcement.md`, `docs/value.md`'s rollup block, `docs/evidence-data.md`, `docs/install-evidence.md`, `ROADMAP.md`'s block and the docs mirror (a byte copy, satisfied by its sources). Each emitter's value is that the gate is clean over its regenerated page.
- **Reds the rewrite can surface, owned in the same commit.** Per-line scanners see whole paragraphs for the first time. A link that was split across a wrap is scanned by `check-md-refs` for the first time. A count or claim that straddled a wrap is read whole by the per-line arms. Any finding this surfaces is a real defect the wrap was hiding, and it is fixed, never exempted. An inline `…-exempt` marker now covers its whole paragraph, a widening this amendment accepts: it can only remove findings, and each marker's reason still names its subject.
- The full battery and every kit's fixture suite run green on the rewrite commit. The rewrite touches every kit's SPEC.

## Producers and consumers

- **The unwrap gate** (delta 1). It is produced on every commit touching the governed set: `tier=precommit`, and the corpus knobs are set by this repo (delta 5), so it is live here. Its consumer is the committing session, which runs the arm the help text prints. The findings' fields (file, line, head) are each read by that session to locate the break. The arm's consumers are that session and a consumer adopting the gate, for whom it is the migration path. The two corpus knobs each have a reader, the gate, and the arm reads its operands only.
- **The capped-form rule** (delta 1). It has no producer today, which the section says. It is a rule for a future reader and mints no mechanism, so point 1 is met by saying there is no instance.
- **`QUEUE_KIT_ENTRY_CAP`** (delta 3). It is produced by its default here. Its consumers are assertion A, the headroom print, `entry-history` (through the same walk) and the `queue-index` worklist's size column. Each reads both the unit and the count. The retired name's reader is the knob loader's retired table.
- **The `cp` row suffix** (delta 4). It is produced by `--update-baseline` and `--ceiling`. Its consumers are the meter (legacy detection), the ratchet (legacy refusal) and the staleness mark, which compares the row's surface figure with the surfaces' size at the iteration-start commit, now both in code points.
- **`CONTEXT_KIT_BREVITY_CAP`** (delta 4). It is produced by its default here, since `scripts/context-config.knobs` sets no budget. Its consumer is `check-brevity`.
- **Point 5, narrowing.** Delta 5's exclusion of gate-test fixtures and of the queue narrows nothing any reader counts on: the unwrap gate is new, and no reader asserts a count, a minimum or coverage over its corpus. Under `off`, assertion A reads nothing, and no other reader depends on it: B to E and `entry-history` measure independently. Re-uniting a measure is not a narrowing. Each cap's red condition is named at its re-calibrated value, and on the measured pool that value admits every entry and bullet the old one admitted (delta 3's four largest entries sit at 3963 to 4211, under `4250cp`, and delta 4's largest pointer-carrying bullet is 327, under `330`).
- **Point 6.** Delta 5 obliges every file in the resolved corpus to be clean, and the arm's postcondition is that value for each. The generator members are named with theirs.

## Existing sections updated

Rosters from the module reads above, `git grep -n -E "ENTRY_LINE_CAP|BREVITY_BUDGET|line count|newline count|counted lines" -- '*.md' '*.rs' ':!docs/' ':!TASK-QUEUE.md'`, `git grep -n "check-queue-wrap" -- ':!docs/'`, and docs/site-architecture.md §Generated projections and their freshness gates, run 2026-09-22.

- `canon-kit/SPEC.md` — the new §check-md-unwrapped, §Layout and configuration's knob roster and corpus-knob count, §check-md-refs's link-escape sentence (delta 1), and §The amendment lifecycle's merge step 4, "measured per-entry", which names the cap (delta 3).
- `canon-kit/checks/check-md-unwrapped.gate` (delta 1).
- `native/src/gates/md_unwrapped.rs` (delta 1).
- `native/src/emit/md_unwrap.rs` and `native/src/emit/mod.rs` (delta 1).
- `native/src/gates/mod.rs` — the new member (delta 1), and the knob declarations naming `QUEUE_KIT_ENTRY_LINE_CAP` and `CONTEXT_KIT_BREVITY_BUDGET` (deltas 3 and 4).
- `native/src/knobs/canon_kit.rs` (delta 1).
- `canon-kit/gate-tests/check-md-unwrapped/` (delta 1).
- `canon-kit/README.md`, its gate roster (delta 1).
- `canon-kit/smoke/install.sh` (delta 1).
- `queue-kit/SPEC.md` — §check-queue-wrap and §check-queue-hygiene (deltas 2 and 3); §The queue format's sentence "which also caps the entry's total length", §Layout and configuration, §check-queue-entry-budget, §The queue-index arm and §The tag algebra's `recurrence:` ceiling sentence (delta 3).
- `native/src/gates/queue_entry_budget.rs` (delta 3).
- `native/src/emit/entry_history.rs` (delta 3).
- `native/src/emit/queue_index.rs` (delta 3).
- `native/src/knobs/queue_kit.rs` — the new row, the grammar check and the retired pair (delta 3).
- `queue-kit/gate-tests/check-queue-entry-budget/` — both fixture knob files set the old name (delta 3).
- `queue-kit/gate-tests/entry-history.test.sh` (delta 3).
- `lifecycle-kit/SPEC.md` — §Layout and configuration, the two knob bullets citing "`QUEUE_KIT_ENTRY_LINE_CAP`'s posture" as their calibration precedent (delta 3).
- `context-kit/SPEC.md` — §The always-loaded meter, §The surface ratchet, §The brevity gate (its one-budget paragraph included), §bin/footprint, §The consumer footprint where it reads the meter's unit, and §Layout and configuration, whose knob bullet and malformed-config sentence name the budget (deltas 2 and 4).
- `drift-kit/SPEC.md` — §Bundled KPIs, `kpi-always-loaded` (delta 4).
- `native/src/emit/always_loaded.rs` (delta 4).
- `native/src/gates/surface_ratchet.rs` (delta 4).
- `native/src/gates/brevity.rs` (delta 4).
- `native/src/emit/footprint.rs` (delta 4).
- `native/src/emit/kpi/always_loaded.rs` (delta 4).
- `native/src/knobs/context_kit.rs` — the new row and the retired pair (delta 4).
- `context-kit/gate-tests/check-brevity/` and `context-kit/gate-tests/check-brevity.test.sh` (delta 4).
- `context-kit/gate-tests/check-surface-ratchet/` and `context-kit/gate-tests/check-surface-ratchet.test.sh` (delta 4).
- `context-kit/templates/close-brevity.md` (delta 4).
- `.workflow/always-loaded-baseline.txt` (delta 4).
- `.workflow/surface-ceiling.txt` (delta 4).
- `scripts/canon-config.knobs` (delta 5).
- `scripts/gates.list` and `scripts/git-hooks/pre-commit` (delta 5).
- Every tracked `*.md` the corpus resolves, every kit SPEC included (delta 5).
<!-- update-target-exempt: dated release post, history left as written; it names the retired knob as it stood on its date -->
- `docs/posts/2026-07-31-checkwright-v0-18-0.md`.
<!-- update-target-exempt: generated mirror, regenerated by its freshness gate's printed command -->
- `docs/queue-kit/SPEC.md`.
<!-- update-target-exempt: generated mirror, regenerated by its freshness gate's printed command -->
- `docs/context-kit/SPEC.md`.
<!-- update-target-exempt: generated mirror, regenerated by its freshness gate's printed command -->
- `docs/lifecycle-kit/SPEC.md`.
<!-- update-target-exempt: generated projections, regenerated by their freshness gates' printed commands -->
- `docs/canon-kit/SPEC.md`, the other `docs/<dir>/` mirrors, `docs/footprint.md`, `docs/value.md`, `docs/enforcement.md`, `docs/check-graph.html`, `docs/evidence-data.md`, `docs/install-evidence.md` and `ROADMAP.md`'s block.

## Retired spellings

- `QUEUE_KIT_ENTRY_LINE_CAP` — the line-only cap knob, replaced by the unit-bearing `QUEUE_KIT_ENTRY_CAP` (delta 3).
- `CONTEXT_KIT_BREVITY_BUDGET` — the line budget, replaced by `CONTEXT_KIT_BREVITY_CAP` in code points (delta 4).

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The causal-completeness check holds for the gate, the arm, both corpus knobs, both caps and the row suffix.
- [ ] **Instruction surfaces: instruction only.** `close-brevity.md` gains a unit word, with no grounds.
- [ ] **Merged with no information lost.** Each re-united passage is re-phrased, not appended to. The rewrite joins lines and deletes no word, and the arm's postcondition test holds that.
- [ ] **Order held.** Deltas 3 and 4 land before delta 5. This amendment merges before `queue-kit/SPEC-queue-headings.md`.
- [ ] **Amendment deleted.** This file is removed on merge (`ls SPEC-*.md`).
- [ ] **Entries moved.** `markdown-hard-wrap-unowned-and-ungated` and `entry-line-cap-has-no-line-axis-relief` move to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** `check-amendment-retired-spelling` runs the block above over the tracked tree.
- [ ] **Gaps filed.** Any cross-component gap found during the work is filed to the gap inbox.
