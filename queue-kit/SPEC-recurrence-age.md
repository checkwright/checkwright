# SPEC amendment: recurrence-age

§The icebox tier counts a dated `recurrence:` line among an entry's live promotion triggers and gives that trigger no age. So one recurrence holds a low-cost entry out of the tier for as long as the entry lives, even after later re-measurements fail to reproduce it. The tier already ages the entry itself (`QUEUE_KIT_ICEBOX_AGE_DAYS` against the defer date). It never ages the recurrence date.

**The ruling: an age limb on the newest recurrence date, read against the tier's existing age knob.** The entry offered an age limb or a stated refusal. The refusal was weighed and turned down. Its only ground would be that a recurrence proves the finding is live. But a recurrence proves it was live *on that date*, and the tier's own age limb already holds that evidence goes stale after a set window: an entry younger than the knob "has not had the chance to recur". A recurrence older than the knob with no newer one is the same evidence read from the other end. The finding had the window to recur again and did not. The limb reuses `QUEUE_KIT_ICEBOX_AGE_DAYS` and mints no knob. Both limbs answer one question, whether the entry has had the window to recur, so a second number could only disagree with the first.

**The round trip is unchanged.** An entry evicted after its recurrence ages out returns to the deferred section on the next real recurrence. That return stamps a fresh date, which is live again under this limb. That is the conserved route §The icebox tier already has, and it is the safety valve here too.

**What does not age.** The limb decides icebox eligibility and nothing else. lifecycle-kit's scope pre-emption rule (`LIFECYCLE_KIT_RECURRENCE_THRESHOLD`) and drift-kit's `kpi-incident-recurrence` count dates as history, and a recurrence stays a recurrence however old it is. Neither reader changes.

**Measured at authoring (2026-09-22).** `grep -c "^  recurrence: " TASK-QUEUE.md` finds 10 declarations. The cause is decided in `native/src/emit/queue_index.rs` `ineligibility()`, which returns the `[recurrence]` class for any `recurrence:` body line that carries a date (`has_date`) and reads no cutoff. The cutoff is computed once per run by the arm from `QUEUE_KIT_ICEBOX_AGE_DAYS` (this repo sets `7`), and `ineligibility()` is not handed it today.

## What changes

### (1) §The icebox tier: a recurrence is a live trigger only while its newest date is inside the age window {design-bearing}

**Not yet applied.** In the eligibility paragraph, replace "a dated `recurrence:` line" in the live-trigger list with "a `recurrence:` line whose newest date is no older than `QUEUE_KIT_ICEBOX_AGE_DAYS`". Then add after the sentence ending "the round trip the tier already conserves.":

> A recurrence ages on the same knob as the entry. It shows the finding was live on its date, and once the window has passed with no newer date the finding had its chance to recur and did not, which is the dormancy evidence the age limb reads. A later recurrence returns the entry with a fresh date. Only eligibility ages the date. The scope pre-emption count and the recurrence KPI read every date as history.

In the self-citing-set paragraph, "a dated `recurrence:` line" becomes "a `recurrence:` line inside the age window".

### (2) The worklist decides the recurrence cause against the cutoff {mechanical}

**Not yet applied.** `native/src/emit/queue_index.rs`: `ineligibility()` takes the cutoff date the arm already computes and returns the `[recurrence]` cause only when the line's **newest** ISO date is at or after the cutoff. The newest date is the last one on the line, since dates are appended in order (§The tag algebra). A line whose dates are all older falls through to the live-slug test, as an undated line does today. The cause text becomes `[recurrence] re-filed <newest-date> — live trigger`, so the row shows the date the window is read from. That is a field with a reader: the closing stage checks the date when it rules on the row. Unit tests: a line whose newest date is inside the window excludes the entry; a line whose dates are all outside it does not; a line with an old date followed by a new one excludes it.

§The queue-index arm's cause-order sentence changes "a dated `recurrence:` declaration" to "a `recurrence:` declaration whose newest date is inside the age window, printed with that date".

### (3) The knob's roster line names its second reader {mechanical}

**Not yet applied.** In §Layout and configuration, the `QUEUE_KIT_ICEBOX_AGE_DAYS` bullet's "the defer-date age filter for the `queue-index` arm's `--icebox-candidates`, and nothing else" becomes "the age window for the `queue-index` arm's `--icebox-candidates`: the defer-date filter and the recurrence-trigger limb (§The icebox tier), and nothing else".

## Producers and consumers

- **The aged trigger** (deltas 1 and 2). It is produced at each `--icebox-candidates` run from the knob every configured icebox already sets. This repo sets `7` and the kit default is `30`, so it is live in every deployed configuration with a tier. Its consumer is the closing stage's eviction, which reads the worklist row. A row whose recurrence has aged now shows `•` or the next cause, and the stage still rules against the recovered body (§The icebox tier).
- **The printed newest date** (delta 2). It is read by the closing stage when a `[recurrence]` row holds an entry out, so it can see when the window closes.
- **Point 5 (narrowing).** This narrows the set of live triggers. The worklist is advisory and has no red condition. No gate reads the trigger: `check-queue-entry-budget` discounts the declaration line by grammar, whatever its age, and `check-roadmap-fresh` does not read it. So no reader flips.
- **Point 6.** Not reached; no member obligation is added.

## Existing sections updated

Roster from `git grep -n -E "\[recurrence\]|dated .?recurrence|recurrence:. line|ICEBOX_AGE_DAYS" -- ':!docs/' ':!TASK-QUEUE.md'`, run 2026-09-22.

- queue-kit/SPEC.md §The icebox tier, the eligibility and self-citing-set paragraphs (delta 1).
- queue-kit/SPEC.md §The queue-index arm, the cause-order sentence (delta 2).
- queue-kit/SPEC.md §Layout and configuration, the `QUEUE_KIT_ICEBOX_AGE_DAYS` bullet (delta 3).
- `native/src/emit/queue_index.rs`, `ineligibility()`, its caller and its unit tests (delta 2).
<!-- update-target-exempt: generated mirror, regenerated by its freshness gate's printed command -->
- `docs/queue-kit/SPEC.md`.

## Retired spellings

- None — no delta retires a name; the cause class `[recurrence]` keeps its token.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The causal-completeness check holds for the age limb and the printed date.
- [ ] **Instruction surfaces: instruction only.** Not reached; no template or shim changes.
- [ ] **Merged with no information lost.** The eligibility paragraph is re-phrased, not appended to.
- [ ] **Amendment deleted.** This file is removed on merge (`ls queue-kit/SPEC-*.md`).
- [ ] **Entry moved.** `recurrence-line-never-ages` moves to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** Not reached; the block above declares none.
- [ ] **Gaps filed.** Any cross-component gap found during the work is filed to the gap inbox.
