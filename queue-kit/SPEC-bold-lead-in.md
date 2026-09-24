# SPEC amendment: bold-lead-in

queue-kit/SPEC.md §The queue-migrate arm starts a new paragraph at every continuation line that opens with a bold span. A wrapped sentence whose next line happens to begin with bold text is therefore cut in two mid-sentence. The arm has no way to tell a bold lead-in, such as `**Deliverable:**` opening a paragraph, from a bold phrase that a wrap placed at a line start.

**Measured at authoring (2026-09-24)**, by running `bash gate-sdk/bin/run-gates.sh --emit queue-migrate` on the pre-conversion queue (`git show 4e205fc2^:TASK-QUEUE.md`):

- **The corpus.** The file carries 275 continuation lines that open with `**`. This repo's conversion found 12 of them mid-sentence, where the join happened by hand at queue-kit-unwrap's close. They are lines 216, 414, 424, 764, 829, 946, 962, 998, 1375, 1451, 1477 and 1559 of that file, each preceded by a line ending in a word such as *is*, *so*, *and*, *a*, *the*, or a dash.
- **The rule in delta 1, replayed over the same lines** (an inline `python3` replay of the arm's paragraph loop, declaration lines lifted as the arm lifts them). It joins exactly those 12 and starts a paragraph at the other 263.
- **The case that needs the second clause.** Dropping it joins exactly one more line, line 20, where the paragraph before ends `blocked-by: markdown-hard-wrap-unowned-and-ungated`, a plain continuation with no sentence end. The line opens a new paragraph because its own bold span ends in `:`.
- **The cases that need the lifted line skipped.** At lines 1321 and 1540 the line before is a declaration, `recurrence:` and `not-icebox-eligible:`, which the arm lifts to the tag line. Each opens a paragraph because the text before the declaration ends in `.`.

## What changes

### (1) A bold span starts a paragraph only as a lead-in {design-bearing}

**Not yet applied.** In `native/src/emit/queue_migrate.rs` `convert`, a continuation line that opens with `**` starts a new paragraph only when either of these holds:

- the open paragraph is empty, or its text ends a sentence: its last character, read past any closing `*`, `` ` ``, `)`, `"` or `'`, is `.`, `:`, `?` or `!`;
- the line's own bold span ends in `.` or `:`, which is the shape of a label lead-in such as `**Deliverable:**` or `**Owner is context-kit, not gate-sdk.**`.

Otherwise the line joins the open paragraph with one space, as any other continuation does. The open paragraph is the text the arm has joined so far, so a declaration line lifted to the tag line never counts as its last line. A list-marker line still always starts a new paragraph, and so does a line after a blank one.

In queue-kit/SPEC.md §The queue-migrate arm, the fourth bullet becomes:

> - A continuation line that opens with a list marker starts a new paragraph. So does one that opens with a bold lead-in: a bold span whose own text ends in `.` or `:`, or any bold span when the paragraph before it ends a sentence (its last character, past closing emphasis, a backtick, a bracket or a quote, is `.`, `:`, `?` or `!`). Every other continuation, a bold phrase a wrap left at a line start included, joins its paragraph with one space, and its indentation is dropped.

### (2) The unit test carries the mid-sentence case {mechanical}

**Not yet applied.** The conversion test's queue (`a_bullet_queue_converts_to_the_heading_grammar`) gains an entry holding both kinds of bold line:

- a wrap that leaves `**bold phrase**` at a line start mid-sentence, which must come out joined;
- a bold line after a sentence end, which must still open a paragraph;
- a `**Label:**` line after a line with no sentence end, which must open one too.

The expected output holds all three, and the idempotence test runs over the same queue. The SPEC's "A unit test holds the conversion over a queue exercising every rule above" is then true of the new rule too, and needs no edit.

## Producers and consumers

- **The paragraph test.**
  - Producer: `convert`, on each continuation line of a task-section bullet entry.
  - Consumer: the rendered body. Only paragraph breaks move.
  - Postcondition reader: the arm's own postcondition compares live slugs, tags and the done set, and none of them reads paragraph breaks. So a fixed conversion passes it exactly as the broken one did. Checked by reading `postcondition` (`native/src/emit/queue_migrate.rs`), which compares the slug-and-tags pairs and then the done set.
- **Roster-holding readers.** None: no name is minted.
- **Point 5.** No corpus narrows.
- **Point 6.** Not obliged.

## Existing sections updated

Roster from `grep -n "bold lead-in" queue-kit/SPEC.md native/src/emit/queue_migrate.rs`, run 2026-09-24.

- `queue-kit/SPEC.md` §The queue-migrate arm, the fourth bullet (delta 1).
- `native/src/emit/queue_migrate.rs` `convert` (delta 1) and its tests (delta 2).
- `.workflow/release-declarations.md`, one Behavior changes bullet (delta 1): `**--emit queue-migrate**` no longer cuts a wrapped sentence at a line that begins with bold text; a bold span starts a paragraph only as a lead-in.
<!-- update-target-exempt: generated mirror, regenerated by its freshness gate's printed command -->
- `docs/queue-kit/SPEC.md`.

## Retired spellings

- None — no name is renamed or deleted.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The causal-completeness check holds for the paragraph test.
- [ ] **Instruction surfaces: instruction only.** Not reached.
- [ ] **Merged with no information lost.** The bullet is re-phrased, not appended to.
- [ ] **Amendment deleted.** This file is removed on merge (`ls queue-kit/SPEC-*.md`).
- [ ] **Entry moved.** `queue-migrate-bold-split` moves to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** Not reached.
- [ ] **Gaps filed.** Any cross-component gap found during the work is filed to the gap inbox.
