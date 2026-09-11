# SPEC amendment: moot-sweep

**Nothing in this amendment is applied.** Every passage below is a proposal for the build stage to
land. Where replacement wording is given it is marked **Not yet applied** at the passage itself.

The entry this amendment discharges is `landing-moots-live-entries-undetected`. The exit already
exists: queue-kit/SPEC.md §The icebox tier sanctions moving an entry that a landed unit has mooted to
the done section. What is missing is detection. No stage step asks which live entries a landing
mooted, so obsolete entries linger until a whole-pool triage reads them one by one.

The last triage found three this way:
- `meta-gate-conservation-record-reach`: a landed gate-sdk row and a consumer-config declaration
  answered both of its halves.
- `propose-once-clause-leaks-into-the-proposal-step`: its referent paragraph was deleted when the
  run holding it retired.
- `same-day-recurrence-date-multiplicity`: an operator decision that re-encoded the stamp superseded
  it.

## Where the step sits, decided

The step sits at the **closing stage, not build**. The entry left either one open. Four grounds
decide it:

- **Close sees every exit together.** It holds the iteration's whole exit set and whole commit range
  at once. A build batch sees only its own landing, so a moot two batches produce jointly is visible
  to neither.
- **The exit set is still in one place.** The done section holds the iteration's exits until close
  clears it, so the step reads them where they already sit.
- **Close already writes the deferred pool.** Its gap drain and its eviction both do. Build's queue
  authority is the Done move for its own units, and a build step moving *other* entries would widen
  it.
- **A build step repeats per batch.** Each batch would sweep the same pool again. Close sweeps it
  once.

## What changes

### (1) Close retires what the iteration's landings mooted, before it clears Done

`lifecycle-kit/templates/stages/close.md` step 5 is replaced {design-bearing}. It currently reads
"**Clear Done.**" **Not yet applied:**

> 5. **Retire what this iteration's landings mooted, then clear Done.** The inputs are the done
> section's slugs and the iteration's commit range. The range runs from the head recorded on the
> first-stage stamp (the first data line of `.workflow/WORKFLOW-STATE.txt`) to HEAD. Derive the
> candidates rather than recalling them:
> (a) the live entries citing a done slug, which are the retired-block rows for those slugs in the
> inbound-citation output step 2 already produced;
> (b) the live entries naming a path the range deleted (`git diff --name-only --diff-filter=D`);
> (c) the live entries citing a markdown heading the range removed from a file it kept
> (`git diff --diff-filter=M` over `*.md`, its removed heading lines).
> Read each candidate against the landing it cites. An entry whose question the landing answered,
> or whose referent it removed, moves to the done section as a bare slug in this stage's queue
> commit, and that commit's message names, per moved slug, the landing that mooted it. A
> `[roadmap:]` entry is outside every exit (queue-kit/SPEC.md §The icebox tier) and stays. A
> candidate still open stays too, with any stale pointer in it corrected inline, the way step 2's
> retired-citation read corrects one. Before moving an entry, relocate anything in its body worth
> more than the entry to the surface that owns it, because the done line keeps only the slug. Run
> the battery before committing. A red naming a moved slug is a pointer the move stranded, and it is
> corrected in that same commit. Then clear Done.

Candidate (a) costs no second run. Step 2 already invokes the arm and reads its retired block, where
every done slug appears as a retired target with its citing entries. The two steps read the same
rows for different questions: step 2 asks whether a citing line is a stale pointer, and this step
asks whether the citing *entry* is still open. Candidate (b) excludes nothing the range deleted.
Candidate (c) deliberately reads only files the range modified, because the headings of a deleted
amendment file are template scaffolding that no entry cites.

**The honest limit.** The candidate set reaches a referent removed at path or heading granularity,
and entries citing landed work by slug. Two cases fall outside it. The first is a referent edited
away below a heading, such as a clause or a paragraph (the second witness above). The second is a
supersession landed outside the range, such as a consult between iterations (the third witness).
The pool-wide triage stays the backstop for both. Whether an entry is mooted is semantic, so no gate
judges the step. The battery judges only that the move stranded no pointer.

### (2) queue-kit's done exit names where detection happens

queue-kit/SPEC.md §The icebox tier, the bullet on conserved moves, ends "or for an entry a landed unit
or a closed ruling has mooted" {mechanical}. It gains a sentence. **Not yet applied:**

> A landing's moots are looked for at the closing stage's moot sweep
> (lifecycle-kit/templates/stages/close.md, step 5), over the iteration's exits and commit range. A
> closed ruling's moots are found by the session that reads the ruling, because no range contains
> them.

## Producers and consumers

The one new interface is a stage step, plus a per-slug clause in the close commit message. It adds
no file, knob, arm or tag.

- **Producer.** The closing stage session runs step 5 every iteration, from inputs that already
  exist:
  - the done section, written by build's Done moves;
  - the retired block of the inbound-citation arm, which close's step 2 runs;
  - `git diff` over a range whose base `--enter-stage` records in the first-stage stamp at the
    iteration boundary.

  **Enabling config: none** beyond the queue-kit arm close already runs.
- **Consumers, each with the transition it reads at.**
  1. `check-task-conservation` reads each moved slug at the queue commit, as a conserved move.
  2. `check-task-names` reds on a `[blocked-by:]` tag naming a moved slug.
  3. `check-queue-slug-liveness` reds on a bold-code membership claim naming one, on a configured prose
     surface.
  4. `check-todo-task-liveness` and `check-gate-exemption-tasks` red on a code TODO or a gate
     exemption naming one.

  Consumers 2 to 4 each read at the step's battery run, before the commit.
  5. drift-kit's `qnet` reads each move as a slug drained from the live pool, at the close commit's
     stated figure.
  6. A later session recovering why an entry left reads the commit-message clause, through the
     pickaxe recovery queue-kit/SPEC.md §The icebox tier names.
- **Every field has a named reader.** The commit-message clause has two parts, the moved slug and the
  landing that mooted it. Consumer 6 reads both.
- **Red conditions, named (point 5).** The step **narrows** the live slug set, so each reader's red
  condition is enumerated rather than cleared by inspection:
  - **Consumers 2, 3 and 4 are not monotone under this narrowing.** Each reds on a reference to a
    slug that *left* the live set. A move to Done adds their violations rather than removing any.
    That is why the step runs the battery before committing and treats a red naming a moved slug as
    a pointer to correct in the same commit.
  - **Consumer 1** reds on a slug that vanishes. A move to Done keeps the slug visible on a bare
    line, so the step cannot flip it.
  - **Consumer 5** has no red.
- **Readers surveyed across the whole tree.** The survey searched `native/src/gates` for every
  reader of the live slug set, with no stderr suppressed. It found the five gates above, plus
  `check-task-conservation`'s membership diff. lifecycle-kit/SPEC.md carries no summary of close's
  step list (a grep for `Clear Done` and `moot` finds none), so no integration prose there restates
  step 5. The close binding's trajectory-projection bullet names "the template's Clear-Done step".
  That step still clears Done, so the bullet stays true and owes no edit.

## Existing sections updated

- `lifecycle-kit/templates/stages/close.md`: step 5 (delta 1).
- `queue-kit/SPEC.md` §The icebox tier: the done exit's detection sentence (delta 2).
- `docs/queue-kit/SPEC.md`: the generated mirror, regenerated in the landing commit (delta 2).

## Retired spellings

- None — no delta retires a spelling. Step 5's bold title grows rather than being renamed. The one
  surface citing it, the close binding's "Clear-Done step", names the half that stays.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named, reachable producer and a
      named consumer; every new field has a named reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its proper canonical-spec
      section (not appended); the merged spec reads as one coherent document a reader who never saw
      the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component
      (`ls SPEC-*.md lifecycle-kit/SPEC-*.md queue-kit/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings`
      above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked
      tree.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a
      build-time causal gap is resolved that session, not deferred).
