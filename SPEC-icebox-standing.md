# SPEC amendment: icebox-standing

**Nothing in this amendment is applied.** Every passage below is a proposal for the build stage to
land. Where replacement wording is given it is marked **Not yet applied** at the passage itself.

The entry this amendment discharges is `icebox-standing-ineligibility-unrecordable`. The closing
stage's eviction worklist (`--emit queue-index --icebox-candidates`) excludes a row on exactly three
causes: a `[roadmap:]` tag, a dated `recurrence:` line, and a named live slug. Nothing reads a
**standing** fact that an entry must stay out of the icebox, so an entry a ruling, direction or
decision already removed from the running is proposed again at every close. Each close then
re-reads its body to the bottom to rediscover the paragraph that says so.

## The fork, ruled

The entry left two candidates open: (a) a `not-icebox-eligible:` body declaration, and (b) the arm
reading a prose marker already in the body. This amendment takes **(a)**.

- **(a)'s recorded cost is not paid.** The entry priced (a) at one counted line against the
  per-entry cap. But `check-queue-entry-budget` discounts at most one line of each declaration
  grammar the queue format defines. The grammar below has that grammar's shape (a lead token, the
  entry's own slug, an ISO date), so (a) costs no counted line at all.
- **(b) has no stable input.** Five live instances were censused at this stage and filed in the
  survey record. They spell the marker five ways: `NOT icebox-eligible`, `NOT ICEBOX-ELIGIBLE`,
  `STANDING NOT-ICEBOX-ELIGIBLE`, `Why NOT icebox`, and a paragraph arguing from the cost line. A
  phrase list misses the sixth spelling. A matcher loose enough to catch it also hits prose
  *discussing* eligibility, this entry's own body included.
- **This is not the retired `ruled:` grammar coming back.** `check-queue-entry-budget` assertion D
  refuses `ruled:` because its only machine reader was the discount itself, and it restated
  provenance the body already carried. This declaration has a reader outside the gate that
  discounts it, the worklist arm. And it states a fact that arm cannot derive.

## What changes

### (1) The `not-icebox-eligible:` declaration joins the queue format

queue-kit/SPEC.md §The tag algebra gains a body-line declaration beside `recurrence:`
{design-bearing}. **Not yet applied.**

`not-icebox-eligible: <slug> <YYYY-MM-DD> <grounds>` is one indented body line. It carries the
entry's own slug, the date the standing fact was recorded, and a non-empty grounds clause. The
clause states the fact in brief, or points at the paragraph, ruling or decision that holds it.

- **Self-naming, by the rule every body-line declaration inherits.** The slug field keeps the line
  unique under `check-queue-hygiene`'s exact-duplicate axis, and it makes the line resolvable by one
  anchored grep.
- **At most one line per entry.** A second standing ground extends the grounds clause and keeps the
  first date. A line per ground would grow an entry linearly against the cap, the refusal §The tag
  algebra already records for a line per recurrence.
- **It declares a standing exclusion, not a promotion trigger.** The entry stays out of the icebox
  whatever its age and cost, until the fact is withdrawn.
- **Who writes it.** The session that records the standing fact on the entry, in the same commit
  as the grounds. The routine instance is a closing stage keeping an entry at its eviction review
  for a reason no trigger carries. Any session landing a direction or decision that an entry stays
  deferred writes it too.
- **Who removes it.** Whoever may revise the fact it records, by the classes of lifecycle-kit/SPEC.md
  §The steering vocabulary:
  - a direction, at a later scoping or authoring stage;
  - a decision, by the role that made it;
  - a ruling, only through the consult skill.

  A worklist row is never grounds for deleting the line.
- **Admitted as a body declaration, not a lead-line tag.** canon-kit/SPEC.md §The amendment
  lifecycle's attribute test decides it: the one machine reader scans entry bodies, and no lead-line
  reader consumes the line.
- **The honest limit.** Nothing stops a session declaring every entry ineligible. The grounds
  clause is mandatory, and the arm prints it on the worklist row. So the review happens on a line
  every close already reads. That review is the only check, and no gate judges grounds.

### (2) The eligibility rule gains a fourth exclusion

queue-kit/SPEC.md §The icebox tier, the **Eligibility** paragraph, gains a limb: the entry carries no
`not-icebox-eligible:` declaration {mechanical}. **Not yet applied.**

The sentence narrowing *what counts as live* stays as written. The declaration is an exclusion
standing beside the live-trigger limb, not a widening of what counts as live. So the measured
narrowing that paragraph records is untouched.

### (3) The worklist arm reads the declaration

`native/src/emit/queue_index.rs`'s `ineligibility()` gains the cause {mechanical}.

- **The test.** A body line whose trimmed text starts with `not-icebox-eligible:` excludes the row.
- **The printed cause.** `[standing] not-icebox-eligible <date> — <grounds>`, under the existing
  cause cap. A line with no date prints `(undated)`, the arm's existing rule that an absent input
  appears rather than vanishing.
- **The order.** The check runs after the `[roadmap:]` tag and before the dated `recurrence:`
  line. A cause someone wrote about this entry outranks a trigger the arm infers.

queue-kit/SPEC.md §The queue-index arm's cause list gains the member and its place in the order. A
unit test pins both. **Not yet applied.** The list today reads "the `[roadmap:]` tag, a dated
`recurrence:` declaration, or the first live slug named in file order".

### (4) The entry budget discounts the declaration

`native/src/gates/queue_entry_budget.rs`'s `DECLARATIONS` table gains `not-icebox-eligible:` beside
`recurrence:` {mechanical}. queue-kit/SPEC.md §check-queue-entry-budget assertion A names both where
it now reads "today `recurrence:` alone". **Not yet applied.**

The section already says a later grammar is discounted "by construction rather than by a further
edit here". The code keys the discount on a token table, so construction here is one table row. The
gate's `good/` fixture gains an entry at the cap that carries the declaration. That entry measures
clean only when the line is discounted.

### (5) The live instances take the declaration, and close's keep branch names it

Build reads each entry in this stage's census, filed in `.workflow/survey-record.md` {design-bearing}:

- `measured-marker-cannot-sit-mid-paragraph`
- `threshold-recurrence-routing-residency`
- `harness-project-dir-fold-dialect-unresolved`
- `manifest-family-couples-misses-the-consumer-widened-corpus`
- `stage-journal-path-unsourced-mid-stage`

Before relying on the census, build runs its witness. For each entry it judges which kind of
paragraph the entry carries:

- **A standing fact** takes the declaration. It is dated from the decision the paragraph records,
  and the paragraph is compressed to what the grounds clause does not already carry.
- **A trigger argument the arm already decides** takes nothing.
- **A ground resting only on a prose cost opener** is not a standing fact. The same goes for the
  retired machinery-class conjunction.

In this repo's close binding, `.claude/commands/close.md`, the backlog-eviction bullet has a keep
branch that reads "keep it in Deferred with the trigger that keeps it there". It gains: "or, where
no trigger keeps it, with the `not-icebox-eligible:` declaration recording why". **Not yet applied.**

## Producers and consumers

The declaration line is the one new interface. There is no new file, knob, arm, tag or event.

- **Producer.** A session records a standing fact on a deferred entry by hand, in the queue file. The
  closing stage's eviction review is the routine instance (delta 5's keep branch). **Enabling config:
  none.** The line needs no knob. Its reader is live wherever a closing stage runs the worklist, which
  this repo's close binding does at every close.
- **Consumers, each with the transition it reads at.**
  1. `queue-index --icebox-candidates` reads the line's **presence, date and grounds** at every
     worklist run, to exclude the row and print its cause (delta 3).
  2. **The closing stage session** reads the printed cause at the eviction review, where it rules the
     row (delta 5).
  3. `check-queue-entry-budget` reads the line's **shape** at every gate run, to discount it from the
     count (delta 4).
- **Every field has a named reader.**
  - **slug:** read by `check-queue-hygiene`, whose uniqueness it guarantees, and by the anchored
    grep §The tag algebra's self-naming rule exists for.
  - **date:** read by the closing stage at review, to weigh whether a later scoping or authoring
    stage has since revised the recorded fact.
  - **grounds:** read by the closing stage at review, on the row consumer 1 prints.
- **Red conditions, named (point 5).** Consumer 3 narrows the count. Assertion A reds only on
  exceeding the cap, so it is monotone in the count. Assertions B, C and D read line shape, cost-field
  presence and a retired lead token. The discount removes nothing from their scans, so none of them
  can flip. Consumer 1 is an advisory projection with no red. It moves a row from `•` to `✗` and never
  drops one.
- **Readers surveyed across the whole tree.** The tracked tree was grepped for `icebox-candidates`,
  `ineligib`, `not-icebox`, `dated re-filing` and `names live slug`, with no stderr suppressed. The
  readers found:
  - `native/src/emit/queue_index.rs`;
  - `queue-kit/SPEC.md` and its generated mirror;
  - `queue-kit/README.md`'s usage line;
  - context-kit/SPEC.md's session-context step, which routes eviction work to the arm and restates
    no cause list;
  - `.claude/commands/close.md`.

  No surface outside queue-kit restates the cause list.

## Existing sections updated

- `queue-kit/SPEC.md` §The tag algebra: the declaration, its grammar, writer, remover, admission and
  honest limit (delta 1).
- `queue-kit/SPEC.md` §The icebox tier: the Eligibility paragraph's fourth limb (delta 2).
- `queue-kit/SPEC.md` §The queue-index arm: the cause list and its order (delta 3).
- `queue-kit/SPEC.md` §check-queue-entry-budget: assertion A's discounted-grammar sentence (delta 4).
- `native/src/emit/queue_index.rs`: the cause and its unit test (delta 3).
- `native/src/gates/queue_entry_budget.rs` and `queue-kit/gate-tests/check-queue-entry-budget/`: the
  table row and the fixture entry at the cap (delta 4).
- `docs/queue-kit/SPEC.md`: the generated mirror, regenerated in the landing commit (deltas 1, 2, 3
  and 4).
- `.claude/commands/close.md`: the backlog-eviction bullet's keep branch (delta 5).
- `TASK-QUEUE.md`: the censused instances (delta 5).

## Retired spellings

- None — no delta retires a spelling. Delta 1 adds a declaration token, and deltas 2 to 4 add a
  member to three existing rosters without renaming any.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named, reachable producer and a
      named consumer; every new field has a named reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its proper canonical-spec
      section (not appended); the merged spec reads as one coherent document a reader who never saw
      the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component
      (`ls SPEC-*.md queue-kit/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings`
      above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked
      tree.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a
      build-time causal gap is resolved that session, not deferred).
- [ ] **The binary rebuilt** — `bash gate-sdk/bin/build-native.sh` in the commit that lands deltas 3
      and 4, and the crate's unit tests and the budget gate's fixture pair green.
