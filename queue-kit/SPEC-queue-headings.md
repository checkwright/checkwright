# SPEC amendment: queue-headings

**Merge order:** after the md-unwrap amendment, merged into canon-kit/SPEC.md §check-md-unwrapped and queue-kit/SPEC.md §check-queue-entry-budget. Delta 6 unwraps the queue with the `--emit md-unwrap` arm canon-kit/SPEC.md §check-md-unwrapped owns.

This amendment pairs three entries.

- **`queue-entry-shape-slugs-headings-links`** carries steps two to five of the operator's 2026-09-10 queue-shape sequence, whose order binds: ratchet slug length, make each task a third-level heading, make every cross-task reference a real link, and retire the bold-code typographic convention. The operator's direction on this iteration (2026-09-22, lead-relayed) restates the goal: records such as tasks become link-referenceable headings.
- **`icebox-eviction-line-budget-squeeze`** and **`lead-line-blocked-by-spec-tag-width-collision`** are **moot**. Per the operator's direction (2026-09-22, lead-relayed), when a strategic unit removes the need for tactical line-limit logic, the tactical entry is unnecessary work. They pair here only to exit at this merge, and the one residue that survives unwrapping is delta 7. Each is a width budget (`check-queue-wrap`'s column cap) colliding with a one-line grammar: the icebox's self-contained sentence, and a lead line carrying a spec tag beside long blocker tags. Delta 6 deregisters `check-queue-wrap` here when the queue unwraps, so in this repo neither collision has a budget left to collide with. For a consumer that keeps the wrap gate, delta 7 states the residue as an honest limit and names the ratchet as its relief. The ratchet is the squeeze entry's own second candidate ruling ("cap slug length at filing"), and it now costs no retroactive rename. With that, neither entry has anything left to author.

**The rulings this amendment makes, with their grounds.**

- **Keep kebab, cap by ratchet, 30 code points** (ruled lead own-authority 2026-09-10, on the operator's delegation, and carried here). The heading **is** the bare slug. GitHub's anchor for a bare kebab heading is the slug itself, since `spec::anchor_slug` is the identity on `[a-z0-9-]`. So heading, anchor and `[blocked-by:]` target are one string in three roles. A ratchet binds a new or renamed slug and grandfathers the rest: 240 of 310 live slugs exceed 30, and a hard cap would force renames onto strings that are about to become anchors. The knob takes `off`, because a record-shape limit is consumer policy (the operator's direction on caps, 2026-09-22, lead-relayed).
- **Every bracketed tag sits on one tag line directly under the heading, and the three body declarations become tags there** (operator direction, 2026-09-22, lead-relayed: "I would move bracketed tags under the heading, but keep them in a single line, so they add less discomfort to the human reader"). The **tag line** is the first line under `### <slug>` when that line holds only bracketed tags. It is the entry's **lead line** in every existing lead-line contract, so none of those contracts is renamed, and the prose follows in its own paragraphs. `recurrence:`, `roadmap-summary:` and `not-icebox-eligible:` convert to `[recurrence: <date> …]`, `[roadmap-summary: <sentence>]` and `[not-icebox-eligible: <date> <grounds>]`. Each member was checked for fitness as a tag:
  - **The self-naming slug field is dropped.** It existed for two reasons: to make a declaration line unique against `check-queue-hygiene`'s duplicate axis, and to make it resolvable by one anchored grep. The heading directly above now supplies the slug. The duplicate axis exempts a tag line (delta 3), because two entries may legitimately carry identical board tags.
  - **canon-kit's attribute test is met rather than failed.** A lead-line attribute is a field tag, and each declaration's readers now read the tag line, so the test's example ("which is why `recurrence:` is one") is re-phrased (delta 2).
  - **The one real limit is the value grammar.** A tag value cannot hold `[` or `]`, so a `roadmap-summary` sentence cannot carry a markdown link, and `not-icebox-eligible` grounds cannot quote a tag. At authoring none of the 6 summaries or 3 standing declarations contains a bracket (`grep -n "roadmap-summary: \|not-icebox-eligible: " TASK-QUEUE.md`). This is stated as an honest limit, not escalated, because no member is unfit.
  - **`check-tag-lead-line` stays.** It now holds that no tag sits anywhere but the tag line, which guards placement at authoring rather than reflow.
- **Done stays a bare `- <slug>` bullet, not a heading** (operator direction, 2026-09-22, lead-relayed). The closing stage clears Done every iteration, so a Done anchor would last one close. The done grammar and its five readers are untouched, and the one required rewrite happens at the Done move: a link to a slug leaving the live set becomes a backticked retired citation, the form §The tag algebra already gives retired references.
- **A reference to a live entry is a link, and the queue gate demands it.** The entry's point was that `check-md-refs` validates links but nothing demands that a reference *be* one. `TASK-QUEUE.md` is outside `CANON_KIT_MANIFEST_FILES`, so `check-md-refs` does not scan it. The queue's own gate therefore holds both halves (delta 5). This reverses one refusal in §The tag algebra, "Whether a citation of landed work should be distinguishable in prose at all — no". That refusal's ground was a hand-maintained marker with no reader and a rewrite at every disposition. Under this grammar the reader is the resolving gate plus any human clicking, and the rewrite is forced, mechanical and made once, at the Done move. The operator's sequence asked for links, and the refusal is re-phrased to the retired half it still holds.
- **The deferred section's `###` subsections are retired.** `###` is now an entry, so a presentation subsection would read as a malformed entry. This repo has none, and the shipped template's `### Someday` goes.

**Measured at authoring (2026-09-22).**

- 310 live slugs, 240 over 30 code points, longest 64. `grep -c '^### ' TASK-QUEUE.md` returns 0. Of the in-body single-backtick tokens, 44 name a live slug other than their own entry's, spread over 26 entries (probe: a live-slug set built from task-section bold leads, then every single-backtick kebab token in those sections tested against it).
- Configured prose surfaces (`docs/*.md`, `*.local.md`) carry no tracked bold-code membership claim. `git grep -o -E '\*\*`[a-z0-9][a-z0-9-]*`\*\*' -- 'docs/*.md'` finds only mirrors and a post, which are outside the gate's glob. `BRIEF.local.md` carries 4, which is local and outside this amendment's reach.
- `native/src/emit/queue_index.rs` reads a `### ` line as a deferred subcategory for its tally, and `queue-kit/templates/TASK-QUEUE.md` ships `### Someday`.
- The reader roster below comes from a sweep of `native/src` for `is_top_level_bullet`, `bullet_slug`, `first_bold_slug`, `is_bullet`, `live_slugs` and `backtick_slugs`, and for independent `- **` scans outside `native/src/queue.rs`. It found nine independent holders beyond the shared module.

## What changes

### (1) The slug-length ratchet: `QUEUE_KIT_SLUG_MAX` {mechanical}

**Not yet applied.** This lands first, before any heading exists, so no grandfathered slug is renamed into an anchor. Add to §Layout and configuration:

> - `QUEUE_KIT_SLUG_MAX` — default `30`, or `off`: the code-point ceiling on a slug that enters the live set. A slug live at `HEAD` is grandfathered at any length. A rename is a new slug, so it is bound.

`check-task-conservation` gains **assertion B**: every live slug in the worktree that is absent from `HEAD`'s live and done sets, both through the shared adapters, is at most `QUEUE_KIT_SLUG_MAX` code points. The finding names the slug, its length and the ceiling. No `HEAD` means clean, as for assertion A. §check-task-conservation states B beside A. The member's constructed scenario gains a new-slug-over-ceiling case and a grandfathered-long-slug case, since the gate stays `no-fixture:` for the reason it already records. The knob row goes in `native/src/knobs/queue_kit.rs`, and its declaration in `native/src/gates/mod.rs`.

### (2) §The queue format: entries are headings {design-bearing}

**Not yet applied.** Replace the paragraph "An entry is a column-0 `- **slug** — prose…` bullet…" and the kit's opening sentence ("sections are queues, bullets are tasks, bold kebab-case slugs are the task handles") with:

> An **entry** in a task section is a third-level heading whose whole text is its slug, `### <slug>`. Its **lead line**, also called its tag line, is the first line after the heading when that line consists only of bracketed tags separated by spaces. It carries every tag the entry has, and an entry with no tags has none. Its body is the paragraphs that follow, one line each, up to the next heading of the same or a shallower level. The first body paragraph is the entry's summary. A **sub-task** is a `#### <slug>` heading inside its parent's extent, with the same grammar one level down. Slugs match `[a-z0-9][a-z0-9-]*` in one global unique namespace across active, deferred, icebox and sub-tasks. A slug is the task's stable handle and its heading anchor, and `[blocked-by:]`, a same-file link and the done line all carry it verbatim. A heading in a task section that is not a valid slug is malformed, which retires the deferred section's `###` presentation subsections. The **done section** keeps its bare `- <slug>` bullets, one line per exit. Lessons keep their bullets, and any other section is outside the grammar.

And the kit's opening sentence becomes "sections are queues, `###` headings are tasks, and each heading's kebab-case slug is the task's handle and anchor". In the deferred-section bullet, "`###` subsections are presentation, not semantics" is deleted. §The icebox tier's grammar block becomes:

```
### <slug>

<one sentence: what it is, and why it is dormant>
```

Its "No `###` subsections and no sub-tasks" bullet becomes "No sub-tasks, no tag line and one sentence of body". An eviction drops every tag, and the `[roadmap:]` exclusion holds unchanged, because an icebox entry has no tag line to carry `[roadmap-summary:]`. Its remaining lead-line wording refers to that sentence.

In §The tag algebra:

- "every tag sits on its bullet's **lead line**" becomes "every tag sits on its entry's tag line, directly under the heading, or on a lesson bullet's own line".
- The `roadmap-summary:`, `recurrence:` and `not-icebox-eligible:` paragraphs are re-phrased as three **field tags** on the tag line. `[roadmap-summary: <sentence>]` is the one sentence the public page prints. `[recurrence: <YYYY-MM-DD>, <YYYY-MM-DD>, …]` is an **array**: one tag per entry, holding every re-filing date, comma-separated and appended in order. A second recurrence appends to it and never adds a second tag (operator direction, 2026-09-22, lead-relayed, restating the array notation already named as a further step in `record-stamp-encoding-compression`'s operator ruling of 2026-09-01). Its count is the number of elements, which the scope pre-emption threshold reads. Its newest date is the last element, which the recurrence age limb reads (queue-kit/SPEC.md §The icebox tier). `[not-icebox-eligible: <YYYY-MM-DD> <grounds>]` holds a date and non-empty grounds. Each keeps its producers, its at-most-one-per-entry rule and its readers.
- The **self-naming slug paragraph** is replaced by: the heading above the tag line names the entry, and `check-queue-hygiene`'s duplicate axis does not read tag lines, so a tag needs no slug field.
- A new **value grammar** sentence: a free-text tag value (`roadmap-summary`, `not-icebox-eligible` grounds, `drain-exempt` and `precondition-ok` reasons) holds no `[` or `]`. So a summary cannot carry a markdown link.
- The "declaration, not a tag" arguments for `recurrence:` and `roadmap-summary:`, and the `close-surface:` comparison, are deleted, since the lead-line reader test now puts them on the tag line.
- The retired-`ruled:` paragraph keeps its refusal, re-phrased as "no `[ruled:]` tag".

Each surface that names the three as declarations re-phrases to the tag form. These are the roster from `git grep -n -E "recurrence:|roadmap-summary:|not-icebox-eligible:"` less docs, the queue and fixtures:
- `CLAUDE.md`'s recurrence-stamping bullet, and `.claude/commands/close.md`'s standing-exclusion clause.
- `lifecycle-kit/templates/stages/close.md` step 7's "append the bullet's date to that entry's recurrence declaration (creating it …)", which becomes "append the date to the entry's `[recurrence:]` array (creating the tag …)".
- `lifecycle-kit/templates/stages/scope.md`'s threshold count ("the number of dates on the entry's `[recurrence:]` array").
- lifecycle-kit/SPEC.md §Layout and configuration's `LIFECYCLE_KIT_RECURRENCE_THRESHOLD` bullet, and §The committed gap inbox's recurrence paragraphs.
- drift-kit/SPEC.md §Bundled KPIs `kpi-incident-recurrence`, `drift-kit/README.md`, the `recurrence:` comment in `drift-kit/templates/kpis.list` and `scripts/kpis.list`, and the smoke queue in `drift-kit/smoke/install.sh`.
- installer/SPEC.md's comparison with the `roadmap-summary:` shape.
- `queue-kit/checks/check-roadmap-fresh.gate`'s `# spec:` line.
- Root `README.md`'s kit table, queue-kit's cell: its tag-algebra list ("blocked-by/spec/drain-exempt/roadmap/observed-by/cost/surface/precondition-ok") gains `recurrence`, `roadmap-summary` and `not-icebox-eligible`, now first-class tags under §The tag algebra rather than body declarations the summary omitted them as.

Each binding keeps its one clause and gains no grounds.

canon-kit/SPEC.md §The amendment lifecycle's attribute paragraph, "What no lead-line reader consumes stays a body declaration, which is why `recurrence:` is one: its readers scan a line of its own", becomes "What no lead-line reader consumes stays prose".

§check-queue-entry-budget's declaration discount is retired: with no declaration lines left, the size is the whole extent, the tag line included. A recurrence tag costs one date per re-filing, under 1% of the cap. The `QUEUE_KIT_ENTRY_CAP` default of `4300cp` admits every entry measured with every line counted. Assertion (D) ("no body line led by `ruled:`") becomes "no `[ruled:` tag".

### (3) The shared adapters and every in-crate reader re-key onto the heading grammar {design-bearing}

**Not yet applied.** `native/src/queue.rs`, per §The shared queue adapters: `is_top_level_bullet` and `is_bullet`/`indent` give way to `entry_heading(line) -> Option<(level, slug)>` (level 3 for an entry, level 4 for a sub-task), and `bullet_slug`, `first_bold_slug` and `strip_bullet_lead` to that function and a `lead_line` helper. The extent is heading-to-next-heading of the same or a shallower level. `live_slugs` collects entry headings in the task sections. `done_slugs` and `bare_bullet_slug` are unchanged. `is_declaration` is deleted, and `field_tags` reads the three former declarations off the tag line. `roadmap_entries` reads `[roadmap:]` and `[roadmap-summary:]` off the tag line. `DeferMarks` still reads `Surfaced`/`Filed` from body lines. Readers of the former declarations: `native/src/emit/queue_index.rs` `ineligibility()` (both the standing and the recurrence causes, the latter reading the newest date against the age cutoff per queue-kit/SPEC.md §The queue-index arm), `native/src/gates/queue_entry_budget.rs` (the discount is deleted and assertion D reads the tag), `native/src/gates/roadmap_fresh.rs` assertion C, and drift-kit's `native/src/emit/kpi/incident_recurrence.rs`, which re-implements the pairing of heading and tag line and cites §The tag algebra. A new `link_slugs(line)` yields the same-file `](#<slug>)` fragments beside `backtick_slugs`.

Callers, each moved onto those adapters: `native/src/emit/queue_index.rs` (index, `--extent`, `--icebox-candidates` and title rendering; the `### ` subcategory tally is deleted), `native/src/emit/queue_counts.rs`, `native/src/emit/queue_edges.rs`, `native/src/emit/entry_history.rs`, `native/src/emit/roadmap.rs`, `native/src/gates/task_names.rs`, `native/src/gates/queue_entry_budget.rs`, `native/src/gates/task_conservation.rs`, `native/src/gates/deferred_board_tags.rs`, `native/src/gates/tag_lead_line.rs`, `native/src/gates/roadmap_fresh.rs`, `native/src/gates/queue_wrap.rs` and `native/src/gates/queue_slug_liveness.rs`. `native/src/gates/queue_prose_precondition.rs` drops its own `is_top_level_entry` for the shared adapter.

§check-queue-hygiene's duplicate axis skips a tag line, because two entries may carry identical tags and the heading above disambiguates them. Its no-column-0-prose axis becomes: a column-0 line is a heading, a bullet, `---`, a `QUEUE_KIT_PROSE_LEADS` token, or a line inside an entry's extent. Prose between a section heading and its first entry is still banned, and indented lines are still never flagged. §check-queue-entry-budget assertion B becomes "every icebox entry is its heading and exactly one lead line". The extent sentence in assertion A reads the heading extent. §check-task-names reads headings, and a task-section heading that is not a slug is a violation. §check-tag-lead-line's scanned surface is the entry lead lines plus lesson bullets. §check-deferred-board-tags reads the lead line. §check-queue-wrap's lead-line discount reads the lead line. §check-task-conservation's done paragraph keeps its rule with the new shape: an entry carried into the done section as its `### <slug>` heading and body matches neither the done grammar nor the live one, lands in no set, and reds as a lost task, so dispositioning to done is still a rewrite to `- <slug>`. `.claude/commands/build.md`'s "never the active-section `- **slug** — prose` shape" becomes "never the entry's heading and body".

### (4) The independent holders re-implement the heading predicate {mechanical}

**Not yet applied.** Each kit that cannot depend on queue-kit re-implements the entry predicate, and both ends cite §The queue format. Each member moves to "`### <slug>`, or `#### <slug>` for a sub-task, in a task section". Its value is its own unit test and fixture pair going green on a heading-form queue:

- canon-kit: `native/src/spec.rs` `queue_slugs` (read by `todo_task_liveness.rs` and `deprecation_task.rs`), `native/src/gates/amendment_queue.rs` (the `[spec:]` tag is read off the lead line under a heading, not off a `- ` bullet), and `native/src/gates/provenance_seam.rs` `lead_slugs` (heading slugs plus bare done slugs). canon-kit/SPEC.md §check-provenance-seam's queue-slug arm, "bullet whose lead is a bold slug or a bare slug", becomes "entry heading or bare done slug". §The shared spec adapters' description of that same `queue_slugs` walk, "a bold lead-in bullet in an active or design-pending section … and a done slug for a bare-slug bullet outside them", becomes "an entry heading in an active or design-pending section … and a done slug for a bare-slug bullet outside them".
- gate-sdk: `native/src/gates/gate_exemption_tasks.rs`. In gate-sdk/SPEC.md §check-gate-exemption-tasks, the live-slug predicate sentence becomes the heading predicate. The "four independent holders" paragraph is re-derived from this amendment's probe, and its count becomes the holders that build a live-slug set: `spec.rs` `queue_slugs`, the crate's queue module, drift-kit's `kpi-queue-net-delta` pool, lifecycle-kit's gap-capture resolver, `check-provenance-seam`'s queue-slug arm, evidence-kit's baseline liveness, and this gate.
- evidence-kit: `native/src/gates/evidence_baseline.rs` `queue_entries`.
- drift-kit: `native/src/emit/kpi/queue_net_delta.rs` `bold_lead_slug` and `native/src/emit/kpi/deferred_age.rs` `is_entry_lead`. `kpi/task_split.rs` reads the unchanged done grammar and stays.
- lifecycle-kit: `native/src/gates/stage_entry.rs` (`active_bullets`, `active_entry_lines`) and `native/src/emit/file_gap.rs` (`entry_slug`, `live_slug`). Without this edit, that resolver would silently match nothing. lifecycle-kit/SPEC.md §The committed gap inbox's live-set sentence becomes "every entry heading in the queue file". Its Lessons exclusion by name drops, since a lesson bullet is not a heading and so falls out by grammar, as a done bullet already does. §check-stage-entry assertion D's extent sentence reads "from its entry heading to the line before the next heading of the same or a shallower level, or a `---` rule".

### (5) References are links, and the bold-code claim retires {design-bearing}

**Not yet applied.**

- **In the queue.** §The tag algebra's citation paragraph becomes: a reference from one entry to another **live** entry is a same-file link, `[<slug>](#<slug>)`. A reference to a retired slug (done or departed) is a single-backticked token, the retired citation. `check-task-names` gains **assertion R**, which reds two things: a same-file link whose slug-shaped fragment names no entry heading (a dangling reference, whose remedy is backticks), and a single-backticked token in a task-section body that names a live slug other than its own entry's (an unlinked reference, whose remedy is the link). A link inside a fence and a token inside a bracketed tag are not read. The refusal "Whether a citation of landed work should be distinguishable in prose at all — no" is re-phrased to what it still refuses: a marker an author maintains by hand. The link form is forced by assertion R at exactly one move.
- **The Done move rewrites.** §The icebox tier's exits and §check-task-names' fan-out paragraph ("A Done move fans out…") gain: the commit that moves a slug to Done rewrites every `[<slug>](#<slug>)` in the file to `` `<slug>` ``, and the binary's `--rewrite` does it in one call. `.claude/commands/build.md`'s Done-move sentence and `lifecycle-kit/templates/stages/close.md`'s disposition step say so in one clause each. Each binding keeps its pointer to §The queue format, which delta 2 re-phrases. `.claude/commands/close.md`'s "rewritten to a bare `- <slug>` line" is unchanged.
- **Edges.** `native/src/emit/queue_edges.rs` reads link fragments as live edges and backticked tokens as retired edges. §The queue-edges arm says so.
- **On prose surfaces.** §check-queue-slug-liveness assertion A's claim grammar becomes a markdown link whose target resolves to the queue file and whose fragment is slug-shaped. The fragment must name a live entry. The bold-code form is no longer read, and §The tag algebra's bold-code paragraph is replaced by the link form. Assertion C (the status parenthetical) reads a link or a backticked token followed by `(<status>)`. Assertions A and C switch to the adapters' `link_slugs`, and the fixture pair's claims become links.
- **The roadmap projection** emits each item as `- [<slug>](<queue-path-relative-to-the-page>#<slug>) *(<track>)* — <summary>`, replacing the bold-code form, so the public page links to the record. §The roadmap arm states the line, `check-roadmap-fresh` byte-compares it as before, and `check-md-refs` (which scans `ROADMAP.md`) resolves each anchor.

### (6) `--emit queue-migrate`, and this repo's conversion {mechanical}

**Not yet applied.** A consumer upgrading queue-kit meets a grammar change, so the kit ships its migration. Add to §The queue-index arm's neighbours a non-gate arm, `--emit queue-migrate [--write] <file>`. The arm converts as follows.

- Each task-section bullet entry becomes `### <slug>`, and an indented bold sub-task becomes `#### <slug>`.
- Every bracketed tag from the lead line goes onto one tag line directly under the heading. Each `recurrence:`, `roadmap-summary:` and `not-icebox-eligible:` body line joins that tag line as its tag. The self-naming slug is dropped, and the recurrence dates become the comma-separated array.
- The lead line's remaining prose opens the body.
- A continuation line that opens with a bold lead-in starts a new paragraph. Every other continuation joins its paragraph with one space, and its indentation is dropped.
- Done bullets, lessons, headings and preambles pass through unchanged.

`--write` rewrites in place, checked. A declaration value holding `[` or `]` cannot become a tag, so the arm refuses the file at exit 2 and names the line. Its postcondition, held by a unit test over the queue-kit fixture trees, is that the shared adapters read the same live slugs, the same tags (the former declarations included) and the same done set before and after.

This repo's conversion, in one commit after deltas 2 to 5 land in code:

- `--emit queue-migrate --write TASK-QUEUE.md`, then `--emit md-unwrap --write TASK-QUEUE.md` for the preambles.
- The live-slug citations become links (44 at authoring; the conversion re-runs the probe above rather than trusting that frozen count, since this file's own authoring already moved it, and takes whatever it currently finds). Each member's value is `[<slug>](#<slug>)`, or backticks where its slug has left the live set since.
- `TASK-QUEUE.md` is removed from `CANON_KIT_UNWRAP_EXCLUDE` in `scripts/canon-config.knobs`, and `check-queue-wrap` is removed from `scripts/gates.list`, with the pre-commit hook regenerated.
- `queue-kit/templates/TASK-QUEUE.md` is migrated (its `### Someday` deleted), and it stays battery-clean as written.
- The queue-shaped fixtures are migrated with the arm: the pairs of `check-task-names`, `check-tag-lead-line`, `check-roadmap-fresh`, `check-queue-slug-liveness`, `check-queue-prose-precondition`, `check-queue-hygiene`, `check-queue-entry-budget` and `check-deferred-board-tags`; gate-sdk's `check-gate-exemption-tasks`; evidence-kit's `check-evidence-baseline`; canon-kit's `check-provenance-seam`, `check-deprecation-task`, `check-todo-task-liveness` and `check-amendment-queue`; lifecycle-kit's `check-scratch-citation`, `check-lesson-disposition` and `check-stage-entry`; and the inline queues in their `.test.sh` scripts. Each pair's `expect.txt` moves only where a reported line number moved. `check-queue-wrap`'s own pair stays in the bullet form it tests, because the kit still ships that gate.

### (7) The one residue of the two moot entries that survives unwrapping: a consumer that keeps the wrap gate {mechanical}

**Not yet applied.** §check-queue-wrap gains:

> **Two grammar collisions are this gate's, and only a consumer registering it meets them.** An icebox entry is one lead line, which a long slug and the tier's self-contained sentence can overrun. An active lead line carrying a spec tag and long blocker tags can overrun the budget before any prose. The relief is `QUEUE_KIT_SLUG_MAX`, which bounds every new slug's share of the line. A consumer keeping unwrapped markdown under `check-md-unwrapped` has no width budget and neither collision.

In §The tag algebra's `[drain-exempt:]` bullet, "while `check-queue-wrap` caps its width … keep it a keyword" is prefixed "Where `check-queue-wrap` is registered,".

## Producers and consumers

- **The ratchet** (delta 1). It is produced by assertion B on every commit touching the queue: `tier=precommit`, registered here, with a default that is live. Its consumer is the filing or renaming session. Each finding field (slug, length, ceiling) is what that session needs to shorten the slug.
- **The heading grammar** (deltas 2 to 4). Its producers are every session writing the queue, and the migration arm. Its consumers are the roster in deltas 3 and 4, each named with its predicate. Every roster-holding reader, meaning each set builder there, is listed (point 2).
- **Assertion R and the link form** (delta 5). Produced by assertion R on every queue commit. Its consumers are the writing session and the Done-moving session, whose rewrite the bindings name. On prose surfaces, the claim's producer is the page author and its consumer is assertion A.
- **The roadmap link** (delta 5). Produced by the roadmap arm. Consumed by the public page reader, with `check-md-refs` as its liveness.
- **The three former declarations as tags** (delta 2). Their producers are unchanged: the filing session for `[roadmap-summary:]`, a session judging a recurrence (close's drain among them) for `[recurrence:]`, and a session recording a standing fact for `[not-icebox-eligible:]`. Their readers are named in delta 3, and the recurrence array is read twice. Its **count** is read by the scope pre-emption threshold and `kpi-incident-recurrence`. Its **last element** is read by the icebox age limb. Every field has a reader. The dropped slug field lost both of its readers, the hygiene duplicate axis and the one-grep resolution, which is why it goes.
- **The migration arm** (delta 6). Its consumers are this repo's conversion and a consumer's upgrade. Its postcondition is its test.
- **Point 5, narrowing.** Retiring the deferred `###` subsections narrows the grammar. The only reader of a subsection was `queue_index.rs`'s tally, which is deleted with it, and that reader asserted no count. The `Lessons` exclusion in the gap resolver drops because the grammar now excludes lessons. Its red condition (none; it prompts) is unchanged. Deregistering `check-queue-wrap` here removes a red condition deliberately, and no other reader counts on it, since this repo's entry cap is in code points (queue-kit/SPEC.md §check-queue-entry-budget).
- **Point 6.** Delta 5's obligation over the 44 citations and delta 6's over the fixture corpus are enumerated by the named probes, with each member's value named.

## Existing sections updated

Rosters from the reader sweep named above, `git grep -n -E "\- \*\*<slug>\*\*|- <slug>|bare-slug line|column-0 .?- " -- '*.md' ':!docs/' ':!TASK-QUEUE.md' ':!*/gate-tests/*'`, and `git grep -n -E "bold-code|\*\*\`<slug>\`\*\*|### Someday|### "` over `queue-kit/`, run 2026-09-22.

- `queue-kit/SPEC.md` — the opening paragraph, §The queue format, §The icebox tier, §The tag algebra, §Layout and configuration, §The shared queue adapters, §The queue-index arm, §The queue-edges arm, §The roadmap arm, §check-roadmap-fresh, §check-queue-hygiene, §check-queue-entry-budget, §check-deferred-board-tags, §check-queue-wrap, §check-tag-lead-line, §check-task-names, §check-task-conservation, §check-queue-prose-precondition, §check-queue-slug-liveness and §templates/ (deltas 1, 2, 3, 5, 6 and 7).
- `queue-kit/README.md` — its task-handle sentence and its tag-reflow sentence (delta 2).
- `native/src/queue.rs` (delta 3).
- `native/src/emit/queue_index.rs`, `queue_counts.rs`, `entry_history.rs` and the other in-crate callers delta 3 names (delta 3).
- `native/src/knobs/queue_kit.rs` and `native/src/gates/mod.rs` — the ratchet knob (delta 1).
- `native/src/gates/task_conservation.rs` and its constructed scenario (delta 1).
- `canon-kit/SPEC.md` — §check-provenance-seam's queue-slug arm and §The shared spec adapters' `queue_slugs` description (delta 4).
- `native/src/spec.rs`, `native/src/gates/amendment_queue.rs` and `native/src/gates/provenance_seam.rs` (delta 4).
- `gate-sdk/SPEC.md` — §check-gate-exemption-tasks, its predicate sentence and holder-count paragraph (delta 4).
- `native/src/gates/gate_exemption_tasks.rs` (delta 4).
- `native/src/gates/evidence_baseline.rs` (delta 4).
- `native/src/emit/kpi/queue_net_delta.rs` and `native/src/emit/kpi/deferred_age.rs` (delta 4).
- `lifecycle-kit/SPEC.md` — §The committed gap inbox's live-set sentence and §check-stage-entry assertion D's extent sentence (delta 4).
- `native/src/emit/file_gap.rs` — its `entry_slug` predicate and comment (delta 4).
- `native/src/gates/stage_entry.rs` (delta 4).
- `native/src/emit/queue_edges.rs`, `native/src/gates/queue_slug_liveness.rs`, `native/src/emit/roadmap.rs` and `native/src/gates/task_names.rs` (delta 5).
- `.claude/commands/build.md` — the Done-move sentence (deltas 3 and 5).
- `lifecycle-kit/templates/stages/close.md` — its disposition step (delta 5) and its recurrence-append step (delta 2).
- `ROADMAP.md` — its generated block, regenerated by the roadmap arm (delta 5).
- `canon-kit/SPEC.md` — §The amendment lifecycle's attribute paragraph (delta 2).
- `native/src/emit/kpi/incident_recurrence.rs`, `native/src/gates/roadmap_fresh.rs` and the declaration discount in `native/src/gates/queue_entry_budget.rs` (deltas 2 and 3).
- `CLAUDE.md` — the recurrence-stamping bullet (delta 2).
- `.claude/commands/close.md` — its standing-exclusion clause (delta 2).
- `lifecycle-kit/templates/stages/scope.md` — the threshold count (delta 2).
- `lifecycle-kit/SPEC.md` — the `LIFECYCLE_KIT_RECURRENCE_THRESHOLD` bullet and §The committed gap inbox's recurrence paragraphs (delta 2).
- `drift-kit/SPEC.md` — §Bundled KPIs, `kpi-incident-recurrence` (delta 2).
- `drift-kit/README.md`, `drift-kit/templates/kpis.list`, `scripts/kpis.list` and `drift-kit/smoke/install.sh` (delta 2).
- `installer/SPEC.md` — the `roadmap-summary:` comparison (delta 2).
- `queue-kit/checks/check-roadmap-fresh.gate` — its `# spec:` line (delta 2).
- `native/src/emit/queue_migrate.rs` and `native/src/emit/mod.rs` (delta 6).
- `TASK-QUEUE.md` — the conversion (delta 6).
- `queue-kit/templates/TASK-QUEUE.md` (delta 6).
- The queue-shaped fixture trees and `.test.sh` scripts delta 6 names (delta 6).
- `scripts/canon-config.knobs`, `scripts/gates.list` and `scripts/git-hooks/pre-commit` (delta 6).
<!-- update-target-exempt: generated mirror, regenerated by its freshness gate's printed command -->
- `docs/lifecycle-kit/SPEC.md`.
<!-- update-target-exempt: generated mirror, regenerated by its freshness gate's printed command -->
- `docs/queue-kit/SPEC.md`.
<!-- update-target-exempt: generated projections, regenerated by their freshness gates' printed commands -->
- `docs/queue-kit/README.md`, the other touched kits' mirrors, `docs/enforcement.md` and `docs/check-graph.html`.

## Retired spellings

- `- **<slug>**` — the task-section bullet entry's grammar spelling, replaced by the `### <slug>` heading (delta 2). The bold-code membership claim retires with delta 5 as well. Its SPEC spelling is a double-backtick span this block's token grammar cannot carry, so that re-phrase is held by delta 5's roster.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The causal-completeness check holds for the ratchet, the heading grammar, assertion R, the link claim and the migration arm.
- [ ] **Instruction surfaces: instruction only.** The two bindings gain one clause each, with no grounds.
- [ ] **Merged with no information lost.** Each grammar passage is re-phrased, not appended to. The migration loses no slug, tag, declaration or prose, and its postcondition test holds that.
- [ ] **Order held.** Delta 1 lands before any heading exists. Delta 6's conversion lands after deltas 2 to 5, and after the md-unwrap amendment merged.
- [ ] **Amendment deleted.** This file is removed on merge (`ls queue-kit/SPEC-*.md`).
- [ ] **Entries moved.** `queue-entry-shape-slugs-headings-links`, `icebox-eviction-line-budget-squeeze` and `lead-line-blocked-by-spec-tag-width-collision` move to Done in the merge commit, at a stage before the drain stage, and their links rewrite per delta 5.
- [ ] **Removals propagated.** `check-amendment-retired-spelling` runs the block above.
- [ ] **Gaps filed.** Any cross-component gap found during the work is filed to the gap inbox.
