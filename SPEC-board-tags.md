# SPEC amendment: board-tags

**Nothing in this amendment is applied.** Every passage below is a proposal for the build stage to
land. Where replacement wording is given it is marked **Not yet applied** at the passage itself.

This amendment serves three entries, which point at it together:

- `deferred-cost-class-opener-vocabulary` is the lead entry: a closed cost-class tag.
- `deferred-surface-tag-for-bundling` adds a closed surface tag, riding this amendment by operator
  direction.
- `scope-ranking-blind-to-deferral-cost-and-impact` is the ranking mechanism that reads both tags.

**Why the board cannot rank today.** The session board is the `queue-index` arm's index rendering.
The session hook prints the deferred listing in full at scope and close sessions, and a tally
elsewhere. That listing strips every tag from a lead line, and the cost field is free prose in the
body. So scope's cost-first ranking reads every deferred body: 166 entries at this commit, roughly
500 KB. The eviction worklist, meanwhile, matches a prose opener that only about a sixth of the pool
opens with, so it under-selects silently. Nothing machine-readable says which entries share a
surface. Scope's third rank tier is therefore re-clustered by hand from full-body reads at every
boundary.

## The value sets, decided

The direction left the value set to spec: it "must serve both readers …, so the values encode
recurrence class before magnitude".

### Cost — `[cost: <recurrence>/<magnitude>]`

`<recurrence>` says when the cost is paid:

- `session` — by every session that loads or runs the affected surface. Examples: an always-loaded
  token cost, a hook, a battery that reds at every commit.
- `iteration` — at least once by every iteration's stage walk. Examples: a re-read at every scope
  or close, a wait at every close, a leg on every push.
- `event` — when a named event other than a stage walk occurs. Examples: a release, an adopter's
  install, a touch of the affected surface, an exposure being exercised.
- `once` — a fixed cost that does not grow while the entry is deferred.

`<magnitude>` is `high` or `low`, judged against the entry's own cost prose.

**The two readers partition this set, and each reads only its own partition.**

- **Scope's first rank tier** is `session/*` and `iteration/*`. Within it, `session` ranks before
  `iteration` and `high` before `low`.
- **The icebox's low class** is `event/low` and `once/low`.

A first-tier value is never icebox-eligible. That is the answer already recorded on
`scope-ranking-blind-to-deferral-cost-and-impact`.

**Why one tag with two slash-joined fields.** Scope's first tier reads recurrence alone. No reader
reads magnitude alone. The icebox reads both. So the pair is one attribute, and splitting it would
mint a tag no reader reads by itself. The roadmap tag's two slash-joined fields set the precedent.

**Why kit-fixed values rather than consumer-configured ones.** Both partitions are kit mechanism:
queue-kit owns the icebox eligibility rule, and lifecycle-kit's scope template owns the rank tiers. A
configured value set would need a configured partition, which moves the kit's eligibility rule into
config. The values name recurrence classes rather than a project's vocabulary, so the provenance seam
is not crossed.

**The prose openers the worklist matched retire as its input.**

| Old opener | Maps to |
|---|---|
| `low`, `zero`, `cosmetic` | `*/low` |
| `bounded` | `once/*` |

### Surface — `[surface: <entry>]`

`<entry>` names one top-level entry of the repository root: a kit root, an owned root directory such
as `native` or `installer`, or a root file such as `CLAUDE.md`. The value is valid exactly when that
entry exists in the root directory listing.

The direction also settled three points, and this amendment does not re-derive them:

- **The axis is surface, not kit.**
- **The value set comes from the tree.**
- **A multi-surface entry declares its primary surface only.**

**This derivation is the least-mechanism reading of the direction, not its letter.** The direction
says "the kit roots plus the rostered root surfaces, derived from the tree". This repo has one roster
of root entries: gate-sdk's `check-root-tiering` allowlist, which is consumer config
(`scripts/root-allowlist.list` here). The tag does not read that roster, for three reasons:

- **The coupling buys nothing.** A queue-kit gate depending on a gate-sdk consumer file adds a
  coupling for a set the tree already yields.
- **The two sets coincide.** Wherever that gate is green, its roster matches the tree's root listing.
- **Any extra is harmless.** A directory listing also admits untracked root scratch, which no filer
  names as a surface.

No knob is added, and the consumer's own root is its configuration. A kit literal would presume a
layout.

## What changes

### (1) The cost tag joins the tag algebra

queue-kit/SPEC.md §The tag algebra gains `[cost: <recurrence>/<magnitude>]`, with the value set and
partition above {design-bearing}. **Not yet applied.**

- **Admitted by the attribute test** in canon-kit/SPEC.md §The amendment lifecycle, because its
  readers (the board and the eviction worklist) scan lead lines alone.
- **It carries the class, and the `Cost while deferred` field carries the prose.** That field stays
  required under `check-queue-entry-budget` assertion C. It is the prose the class summarises, and
  where scope reads impact for a shortlist.
- **§The queue format's sentence on that field** gains "its class rides the lead line's `[cost:]`
  tag".

### (2) The surface tag joins the tag algebra

queue-kit/SPEC.md §The tag algebra gains `[surface: <entry>]`, with the derivation above and the
primary-surface rule {design-bearing}. **Not yet applied.**

**The lead's shared-surface batching is deliberately not a reader.** `lead.md` §Economics cuts
batches from promoted entries whose amendments name the files they edit, and a root entry is
coarser than that.

### (3) Both tags are lead-line-governed

`native/src/gates/tag_lead_line.rs`'s `CLASSES` table gains `cost:` and `surface:` {mechanical}.
queue-kit/SPEC.md §check-tag-lead-line's governed-set sentence names both, and so does the `# spec:`
parenthesis in `queue-kit/checks/check-tag-lead-line.gate`.

`--emit-enum-sets` reads that table, so the task-tag enum set widens with no emitter edit.
`check-prose-enum` then reds any prose that enumerates the task-tag set without the new members.
Build runs it and repairs each site it names.

The gate's `good/`+`bad/` pair gains a lead-line and a reflowed instance of each tag.

### (4) The owed gate: `check-deferred-board-tags`

queue-kit/SPEC.md §The icebox tier has recorded "the gate that holds it is owed" since the class
opener became the contract, and this delta discharges it for the tag {design-bearing}.

**Build.** The gate is born native: a Rust module, a `.gate` descriptor and a `good/`+`bad/` fixture
pair. It is registered in `scripts/gates.list` and queue-kit/README.md's gate roster. Its `# graph:`
manifest drives the regenerated pre-commit hook and `docs/check-graph.html`. It is declared on
`.workflow/tightened-gates.txt` as gate-sdk/SPEC.md §upgrade-smoke rules for a landed gate.
queue-kit/SPEC.md gains its own section. **Not yet applied.**

**Assertions.**

- **(A) Presence and validity.** Every top-level deferred entry carries exactly one `[cost:]` tag,
  whose value parses in the closed grammar. It also carries exactly one `[surface:]` tag, naming an
  existing top-level entry of the repository root.
- **(B) Absent from the active sections.** A promotion drops both tags, as it drops
  `[design-pending]` (delta 9), so this assertion is the checksum on that move.
- **Not applied to the icebox.** An eviction drops both tags (delta 9), and a one-line entry has no
  reader for either.
- **Fail-closed.** An unreadable queue file or repository root exits 2.
- **Opt-in by registration.** It is a separate gate rather than a widened assertion on an existing
  one. A consumer that registers it meets it on upgrade; one that does not sees no change.

**Delta 5 leans on assertion A.** "Exactly one of each, in a closed grammar" is what bounds the
width that delta discounts.

### (5) `check-queue-wrap` discounts the two tags on a deferred lead line

**The conflict, measured and filed in the survey record at 98ae2633.** Deferred lead-line heads — the
slug plus the tags an entry already carries — reach 99 columns. About 48 added columns push most lead
lines past the 100-column floor: 143 of 169 at 98ae2633, and 140 of 166 at 927dc297 by the intent
oracle's recount. No value spelling fits every entry while both tags must sit on the lead line.

**Settled by lead decision, grounded in the intent oracle's answer** {design-bearing}: option B,
narrowed. When `check-queue-wrap` measures a top-level lead line in the deferred section, it removes
exactly the `[cost: …]` and `[surface: …]` tokens, each with one adjacent space, and measures what
remains. No other governed tag is discounted. `[blocked-by:]` may repeat, so discounting every
governed tag would let a lead line grow without bound. The two tags here are bounded: delta 4
asserts exactly one of each in a closed grammar.

**The resulting maximum lead-line width** is:

- `QUEUE_KIT_WRAP_BUDGET`,
- **plus 23**: the widest cost tag, `[cost: iteration/high]`, and its space,
- **plus 12 and the longest top-level root entry name**: `[surface: ]` and its space.

In this repo that is 153 columns, with `CODE_OF_CONDUCT.md` as the longest permanent root entry.
queue-kit/SPEC.md §check-queue-wrap states the formula rather than this repo's figure.
**Not yet applied.**

**Refused, with grounds.**

- **Raising `QUEUE_KIT_WRAP_BUDGET` for the whole file.** It loosens every line, prose included,
  and weakens the entry cap the wrap gate underwrites (below).
- **Moving the surface tag into the body.** It defeats the reason the tag rides the board.
- **Shorter spellings.** No spelling fits every entry.

§check-queue-wrap also gains two statements. **Not yet applied.**

- **The coupling it underwrites.** `check-queue-entry-budget` bounds an entry by counting its
  **lines**, and a line cap bounds an entry's size only while lines are width-bounded. So the wrap
  gate is that cap's denominator, which is a stronger ground than the column-0 runaway it states
  today. The discount keeps the denominator: it widens only the one lead line, by a bounded amount,
  and every prose line still wraps at the budget.
- **Inheritance.** Any later gate measuring the width or wrapping of queue lines inherits this
  discount for these two tags.

The kit SPEC states that rule undated and names no queue slug. The instance lives on this repo's
queue instead: build adds one sentence to `markdown-hard-wrap-unowned-and-ungated`, the entry that
owns the future hard-wrap gate. The sentence says that gate inherits the two-tag discount. **Not yet
applied.**

`queue_wrap.rs` gains a unit test for each case:

- a tagged lead line over the raw budget but under it once discounted, which passes;
- a prose continuation line over the budget, which still reds;
- a tagged active-section line, which is not discounted.

The gate's fixture pair gains the passing case.

### (6) The board prints both tags, and the worklist reads the class instead of the opener

`native/src/emit/queue_index.rs` changes in two places {design-bearing}. queue-kit/SPEC.md §The
queue-index arm is updated to match. **Not yet applied.**

- **The index rendering.** It re-echoes `[cost: …]` and `[surface: …]` after each deferred row's
  title, the way it re-echoes `[blocked-by:]` and `[drain-exempt:]`. The session hook prints the full
  listing only at scope and close sessions, so the added tokens are paid there alone.
- **`--icebox-candidates`.**
  - **Selection.** The cost limb admits a row whose class is `event/low` or `once/low`.
  - **An entry with no tag** is listed as `(unclassed)`, on the arm's rule that an absent input
    appears rather than vanishing.
  - **The printout.** The class is printed where the opener was.
  - **Retired.** The opener parse goes: `opener()`, `low_class()` and `OPENER_CAP` are deleted.

### (7) The icebox eligibility rule reads the tag, and the no-re-authoring rule moves with it

queue-kit/SPEC.md §The icebox tier changes in three places {design-bearing}. **Not yet applied.**

- **The Eligibility paragraph.** "its cost field opens in the low class" becomes "its `[cost:]` class
  is in the low class".
- **The class-word paragraph retires.** It begins "The cost field's opening token is the class word".
- **The opener rule becomes a class rule.** "That opener is not re-authored to unblock an eviction"
  becomes "that class is not re-declared to unblock an eviction". Its grounds and both refused
  alternatives carry over. The alternative that named "the owed opener gate" now names delta 4's
  gate, which likewise enforces shape and never a cost's truth.

### (8) Scope ranks from the board

lifecycle-kit/templates/stages/scope.md's ranking paragraph ("**Rank the pool by what deferral costs
and what landing buys**") changes in four places {design-bearing}. **Not yet applied.**

- **The first tier** is the board rows whose `[cost:]` class is `session` or `iteration`. Scope reads
  bodies for that shortlist only.
- **The third tier** fills from rows whose `[surface:]` value matches the lead unit's.
- **The join-now test gains its re-spend criterion.** A first-tier entry joins the iteration being
  bounded only where joining re-spends nothing. That means both of these hold:
  - its surface is one the unit set already carries;
  - it triggers no stage the set does not already walk. A feature joining a debt-only set triggers
    the authoring stage, and a second component can trigger the audit stage.

  Otherwise it leads the next iteration's set, which its class arranges at that scope without a
  record.
- **The closing sentence changes.** "an entry states both in its own cost field, which is what this
  ranking reads" becomes: the class rides the tag, and impact rides the field's prose.

### (9) Filing, promotion, eviction and demotion carry the tags

Each move that writes a lead line now handles the two tags {mechanical}. **Not yet applied.**

- **Filing.** lifecycle-kit/templates/stages/close.md step 2's →promote files the entry with both
  tags. Every other filing session does the same, and delta 4 holds them to it.
- **Promotion.** canon-kit/SPEC.md §The amendment lifecycle: promotion drops both tags along with
  `[design-pending]`.
- **Demotion.** canon-kit/SPEC.md §Merging an amendment, step 4: demotion restores both tags from the
  promoting commit's diff, the same diff it already recovers the position from.
- **Eviction.** queue-kit/SPEC.md §The icebox tier's grammar bullets and `.claude/commands/close.md`'s
  backlog-eviction bullet: eviction drops both tags.

### (10) The `[drain-exempt:]` width sentence names what the discount does not cover

In queue-kit/SPEC.md §The tag algebra, the `[drain-exempt:]` bullet reads "`check-tag-lead-line` pins
the tag to the lead line while `check-queue-wrap` caps its width" {mechanical}. It gains a clause: the
lead-line discount covers only `[cost:]` and `[surface:]`, never this tag, whose free-text reason is
unbounded. **Not yet applied.**

### (11) The release note's Behavior changes lines

No accumulating surface carries a behavior change between iterations; the deferred entry
`behavior-change-surface` records that gap {mechanical}. So build writes these lines into the landing
commit's message under `Behavior change:`, for the next release note's Behavior changes section
(RELEASING.md, under docs/install.md §The upgrade contract's grammar). **Not yet applied.**

- **`check-queue-wrap`.** A deferred lead line's `[cost:]` and `[surface:]` tags no longer count
  toward the wrap budget. The gate reds less and never more, so nothing an adopter's tree passes
  today starts failing.
- **`queue-index --icebox-candidates`.** It reads the `[cost:]` class instead of the cost field's
  opening word, and an entry without the tag is listed `(unclassed)`. An adopter who has not adopted
  the tag sees a longer worklist, never a shorter one.
- **`queue-index` index rendering.** It re-echoes both tags on deferred rows.
- **`check-deferred-board-tags`.** It ships, and is opt-in by registration.
- **Copied-out templates.** The scope template ranks from the two tags, and the close template files
  entries with them.

### (12) The reclassification sweep

Every top-level deferred entry takes both tags {design-bearing}. There are 166 at this commit.

- **The cost class** comes from the entry's own `Cost while deferred` prose. An opener reading `low`
  whose prose describes a per-iteration cost is classed by the prose.
- **The surface** is the entry's primary named surface.

The sweep lands in the commit that registers delta 4's gate and lands deltas 3 and 5. Until all
three are in, `check-deferred-board-tags` reds on untagged entries and `check-queue-wrap` reds on
tagged ones. A classification fan-out is sanctioned, provided each row is verified against its body
before it lands.

## Producers and consumers

**The new interfaces** are:

- the `[cost:]` tag and the `[surface:]` tag;
- `check-deferred-board-tags`'s verdict;
- the wrap discount;
- the board's tag re-echo;
- the worklist's class column.

**Producers.**

- **Both tags** are written by hand on the lead line by the filing session:
  - scope's triage;
  - close's drain →promote;
  - an operator-directed filing;
  - a session returning an icebox entry on recurrence.

  Delta 12's sweep writes them for the existing pool. **Enabling config: none** for the tags.
- **The gate's verdict** is live because this repo's `scripts/gates.list` registers the gate (delta
  4), and the regenerated hook and CI run the battery.
- **The discount, the re-echo and the class column** are live as soon as the binary is rebuilt. They
  read no knob that is not already read.

**Consumers, each with the transition it reads at.**

1. `check-tag-lead-line` reads each tag's **placement** at every gate run (delta 3).
2. `check-deferred-board-tags` reads **presence, value grammar and surface existence** in the
   deferred section, and **absence** in the active sections, at every gate run (delta 4).
3. `check-queue-wrap` reads each tag's **token extent** on a deferred lead line at every gate run, to
   discount it (delta 5).
4. The `queue-index` index rendering reads **both values** and re-echoes them. The **scope session**
   reads them on the board the session hook prints, at ranking (deltas 6 and 8).
5. `queue-index --icebox-candidates` reads the **cost value** at every worklist run. The **closing
   stage** reads the printed class at the eviction review (deltas 6 and 7).

**Every field has a named reader.**

- **Recurrence** is read by scope, for first-tier membership and order (consumer 4), and by the
  worklist, to exclude `session` and `iteration` (consumer 5).
- **Magnitude** is read by the worklist, for the low class (consumer 5), and by scope, for order
  within the first tier (consumer 4).
- **The surface value** is read by scope, at the third-tier fill and the join-now criterion
  (consumer 4), and by consumer 2, for validity.

**Red conditions, named (point 5).**

- **`check-queue-wrap` narrows its measured width.** It reds only on exceeding the budget, so it is
  monotone and clears by inspection.
- **`check-prose-enum` sees a widened set.** It reds a prose enumeration that now lacks members. That
  is not monotone, so build runs it (delta 3).
- **`check-tag-lead-line`'s widened set** reds a body line carrying a bracketed `[cost:` or `[surface:`
  on an entry whose lead line lacks it. At authoring, the one bracketed instance was in
  `deferred-cost-class-opener-vocabulary`'s own body, and this stage's promotion commit unbrackets
  it. A grep of the queue found no other.
- **`check-deferred-board-tags` is a new red on absence.** That is why delta 12 lands in the same
  commit.
- **`check-queue-entry-budget` is untouched.** Its assertion C still asserts the prose field.

**Readers surveyed across the whole tree.** The tracked tree was grepped for `Cost while deferred`,
`cost field`, `cost-class`, `class word`, `opener` and `icebox-candidates`, with no stderr
suppressed. The readers are:

- `native/src/emit/queue_index.rs`, the only reader of the opener;
- `native/src/gates/queue_entry_budget.rs`, which checks presence only and is unchanged;
- queue-kit/SPEC.md, queue-kit/README.md's usage line and lifecycle-kit/templates/stages/scope.md;
- context-kit/SPEC.md's session-context step, which routes to the arm and restates no rule;
- `.claude/commands/close.md`.

The two fixture texts that spell the field (`native/src/emit/file_gap.rs`,
`lifecycle-kit/gate-tests/file-gap-recurrence.test.sh`) are gap-bullet inputs, not readers. The
release posts under `docs/posts/` are history.

**Edges to this iteration's other amendments.**

- `SPEC-icebox-standing.md` delta 3 edits the same `ineligibility()` and `flush()` path as delta 6
  here.
- `SPEC-icebox-standing.md` delta 5 judges instances resting on a prose cost opener, which delta 6
  retires as an input.

Neither edge crosses a value.

## Existing sections updated

- `queue-kit/SPEC.md` §The queue format: the cost field's class rides the tag (delta 1).
- `queue-kit/SPEC.md` §The tag algebra: both tags (deltas 1 and 2), and the `[drain-exempt:]` width
  sentence (delta 10).
- `queue-kit/SPEC.md` §The icebox tier: the eligibility limb, the retired class-word paragraph, the
  no-re-declaration rule, and eviction dropping the tags (deltas 7 and 9).
- `queue-kit/SPEC.md` §The queue-index arm: the re-echo and the worklist's class column (delta 6).
- `queue-kit/SPEC.md` §check-tag-lead-line: the governed set (delta 3).
- `queue-kit/SPEC.md` §check-queue-wrap: the discount, its bound, the entry-cap coupling and the
  inheritance rule (delta 5).
- `queue-kit/SPEC.md`: the new `check-deferred-board-tags` section (delta 4).
- `canon-kit/SPEC.md` §The amendment lifecycle and §Merging an amendment: promotion drops the tags and
  demotion restores them (delta 9).
- `lifecycle-kit/templates/stages/scope.md`: the ranking paragraph (delta 8).
- `lifecycle-kit/templates/stages/close.md`: step 2's →promote (delta 9).
- `.claude/commands/close.md`: the backlog-eviction bullet (delta 9).
- `native/src/gates/tag_lead_line.rs` and its fixture pair (delta 3).
- `native/src/gates/queue_wrap.rs` and its fixture pair (delta 5).
- `native/src/emit/queue_index.rs` (delta 6).
- The new native gate module, `native/src/gates/mod.rs` registration,
  `queue-kit/checks/check-deferred-board-tags.gate` and its fixture pair (delta 4).
- `queue-kit/checks/check-tag-lead-line.gate`: the `# spec:` parenthesis (delta 3).
- `queue-kit/README.md`, `scripts/gates.list`, `scripts/git-hooks/pre-commit`,
  `docs/check-graph.html` and `.workflow/tightened-gates.txt` (delta 4).
- `docs/queue-kit/SPEC.md` and `docs/canon-kit/SPEC.md`: the generated mirrors (deltas 1, 2, 3, 4, 5,
  6, 7, 9 and 10).
- `TASK-QUEUE.md`: the sweep (delta 12), and the inheritance sentence on
  `markdown-hard-wrap-unowned-and-ungated` (delta 5).

## Retired spellings

- `OPENER_CAP` — the worklist's opener truncation constant (delta 6).
- `low_class` — the prose low-class matcher (delta 6).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named, reachable producer and a
      named consumer; every new field has a named reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its proper canonical-spec
      section (not appended); the merged spec reads as one coherent document a reader who never saw
      the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component
      (`ls SPEC-*.md queue-kit/SPEC-*.md canon-kit/SPEC-*.md lifecycle-kit/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings`
      above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked
      tree.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a
      build-time causal gap is resolved that session, not deferred).
- [ ] **The binary rebuilt** — `bash gate-sdk/bin/build-native.sh` in every commit touching
      `native/`, with the crate's unit tests and the three touched gates' fixture pairs green.
- [ ] **Behavior changes carried** — delta 11's lines sit in the landing commit message.
