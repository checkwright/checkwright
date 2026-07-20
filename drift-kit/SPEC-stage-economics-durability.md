# SPEC amendment: stage-economics-truncation-durability

Makes `bin/stage-economics.sh` truncation-immune by reading the stage stamps
from committed history rather than the live state file alone. The sibling meter
`bin/trajectory.sh` already reconstructs the identical stamps this way and is
durable by construction; this amendment brings the second meter over the same
stamp surface onto the same technique.

## What changes

**The stamp source becomes the union of history and the live file.** Today the
join loop reads `< "$DRIFT_KIT_STATE_FILE"` (`bin/stage-economics.sh:158`), and
`/scope` truncates that file at every iteration boundary, so an iteration's
economics are capturable only in the window between its close and the next
scope. The meter instead collects stamps from two sources and joins over the
union:

1. **Committed history** — added lines in the state file's diff across history,
   the technique `bin/trajectory.sh:95` already uses:
   `git log --reverse --format='COMMIT %H' -p -U0 -- "$DRIFT_KIT_STATE_FILE"`,
   keeping `+`-prefixed lines that parse as the stamp grammar.
2. **The live file** — its current data lines, when it exists.

Union, not replacement and not fallback, and the choice is forced by what each
alternative loses:

- *Replacement* blinds the meter to the stamp of a stage that has stamped but
  not yet committed. Every stage session stamps as its first step and commits
  that stamp on its own, so the window is short but real — and it is exactly the
  window in which the in-flight stage's own economics would be read.
- *Fallback* (read history only when the live file is unreadable) reintroduces
  the loss for the common case: the live file is almost always present and
  almost always truncated, so the fallback arm would never fire on the path that
  needs it.

The union costs nothing structurally, because the existing `seen_triples` dedup
(`:132-133`) already collapses a stamp seen twice on the
`iteration/stage/session8` key. History is a superset of the live file except
for the uncommitted tail, so the union's only added rows are precisely the ones
replacement would have lost.

**No reconstruction-depth bound, and no knob for one.** The reconstruction reads
all available history. `bin/trajectory.sh` sets the in-kit precedent — it bounds
nothing and reads the state file's whole history — and for this meter the
effective bound is already self-enforcing from a different direction: a stamp
whose session transcript has aged out of `DRIFT_KIT_SESSIONS_DIR` takes the
existing unmatched-transcript path (`:136-139`) and costs one skipped row. A
depth knob would be a second bound with no reader, which §Definition of Done
rules out for a field and this amendment rules out for a knob on the same
grounds.

**The unmatched-stamp notice collapses to one summary line.** Unbounded history
turns the per-stamp `no transcript matched (skipped)` line into unbounded
output, so the meter counts unmatched stamps and emits a single line naming the
count instead of one line each. This is presentation calibration, not a contract
change — the skipped rows were never logged and still are not.

**The append-log needs no new idempotence — it already has it.** The open
question was whether a history-reading meter that can re-derive rows it already
logged needs a new mechanism. It does not: `log_line` (`:113-121`) already
strips any prior line matching the `<iteration> <stage> <model>` triple before
appending, and §The trend log already specifies that triple as the dedup key
read on append. Re-deriving a logged row replaces its line rather than
double-counting. This amendment adds no mechanism here and changes no grammar;
it records that the property the question asked for is already shipped and
already asserted, so no future reader re-derives it.

**One honest limit follows from re-derivation.** The log's `<date>` field is the
*measurement* date, not the stage's date, and re-deriving an old iteration
restamps that row to the day it was re-measured. Under the live-file meter the
two nearly coincided (a row could only be measured inside its own window); under
the history meter they can be far apart. The field's named reader is the
reading-age caveat, which stays correct — it ages the reading, which is what it
says — but the field can no longer be read as "when this stage ran". The stage's
own date is in the stamp, and the trajectory extractor is the surface that
renders it.

**Degradation widens.** The meter currently exits early with a notice when the
state file is absent (`:104-108`). That arm becomes wrong: history can carry
stamps for a state file that is absent from the working tree. The absent-file
case degrades to a notice and continues to the history read, and the 0-exit
"nothing to read" notice fires only when *both* sources yield no stamps. The
advisory contract is unchanged — exit is always 0, the meter never joins
`gates.list`, and a missing input is a notice rather than a failure.

## Producers and consumers

**New state: the history-derived stamp set.**

- *Producer* — a new `collect_stamps()` in `bin/stage-economics.sh`, invoked
  unconditionally on every run (its enabling config is the already-emitted
  `DRIFT_KIT_STATE_FILE`, which this repo's `scripts/drift-config.sh` and the kit
  default both set; no new config must be emitted anywhere for the producer to
  fire). It shells to `git log` in the repo root the script already `cd`s to
  (`:7-8`).
- *Consumer* — the existing join loop (`:129-158`), which changes only its input
  redirection: it reads `collect_stamps()` output instead of the state file
  directly. Its body, its `seen_triples` dedup, its transcript match, and its
  pricing are untouched.

**New fields: none.** The stamp grammar
`<iteration> <stage> <session-id> <date>` is lifecycle-kit's
(lifecycle-kit/SPEC.md §The state machine), and this meter stays a read-only
consumer of it — reading it from a second location changes nothing there. The
trend-log grammar gains no field, so there is no new field needing a named
reader.

**New knobs: none**, per the no-depth-bound ruling above.

**Unchanged consumers verified, not assumed.** `bin/trajectory.sh` reads the
same state file's history independently and is not touched. The
`/economics` skill chains `bin/overhead-meter.sh` → `bin/stage-economics.sh`
(§The `/economics` skill) and consumes the meter's stdout and the trend log; both
keep their grammar, so the skill needs no change. `kpi-amendment-age` and the
other bundled KPIs read neither this meter nor its log.

## Existing sections updated

- **§The stage-economics meter, input 1 (Stamps).** The sentence "Read from
  `DRIFT_KIT_STATE_FILE` (§Layout and configuration), defaulting to the same
  state-file path the trajectory extractor already reads" becomes the union
  contract: read from that path's *committed history and current content*, so a
  boundary truncation of the live file destroys no economics. State the
  union-over-replacement rationale in one line and the no-depth-bound ruling
  with its trajectory precedent.
- **§The stage-economics meter, Degradation.** Extend to the both-sources-empty
  condition replacing the absent-state-file early exit.
- **§The trend log.** Note that the dedup key is what makes re-derivation safe
  (it already is — this adds the *why now* to an existing statement, not a new
  rule), and correct the `date` field's reader description to say measurement
  date with the re-derivation limit.
- **§Layout and configuration, `DRIFT_KIT_STATE_FILE`.** "the WORKFLOW-STATE path
  the stage-economics join reads for stamps" becomes the path whose history and
  live content the join reads. Default unchanged.
- **§Testing.** The smoke already builds a hermetic fake-history repo for the
  trajectory extractor. Add the economics assertion to it: a stamp present only
  in history — truncated out of the live file — still prices, which is the whole
  claim of this amendment in one assertion.

## Seam

Generic mechanism throughout. The state path is already each consumer's config,
the git-history technique is layout-independent, and no rule content, model
roster, or product constant enters the kit — the price table remains the sole
consumer-config surface and is untouched.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls drift-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **The attested loss is recovered** — after the merge, a run of the meter
      prices the `spec` stage of an iteration whose live-file stamps were already
      truncated, which is the defect this unit was filed on.
