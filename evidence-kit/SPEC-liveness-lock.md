# SPEC amendment: producer-liveness lock

Pairs with the queue entry `validate-producer-liveness-unobservable`.

A stage session can report its stage done while its own oracle is still
executing, and nothing in the lifecycle can see it. The gap is **liveness
alone**: `check-evidence-manifest`'s assertion A already asserts suite-roster
completeness at a close cursor, and `run-validate` already builds each manifest
revision in a temp file and `mv`s it into place, so neither coverage nor torn
reads are at issue. What no surface carries is the fact that a producer is
*still running*, and a file read at an instant cannot carry it.

## The paired half, and why shipping this one alone is the failure

This gate is the **oracle half** of a two-part fix.
`lifecycle-kit/SPEC-dispatch-signal.md` is the other: the prose rule that a lead
dispatches stage N+1 on stage N's completion notification, never on artifact
state. **The two are one unit — prose rule on the dispatch side, oracle on the
artifact side** — and shipping this half alone is the named failure to avoid: it
installs a detector for the consequence while leaving the decision that causes it
ungoverned. The Definition of Done below binds them.

A third amendment completes the causal story:
`delegation-kit/SPEC-residency.md` is what keeps the completion notification
truthful in the first place, since a dispatched session that ends its turn on
still-running work emits a signal that lies. The entry-side reader
(`check-producer-liveness`, at stage entry and inheriting into `--simulate`) is
the detector for the residual case where the signal is wrong anyway — which is
precisely why it is worth building even though the two prose rules exist. Delta
6's writer-side refusal is not a second detector for that same case: close never
invokes `run-validate`, so there is nothing there for it to refuse. It is
adjacent coverage for a distinct hazard the residency/dispatch-signal chain does
not name — a second or out-of-band `run-validate` invocation racing the
manifest — named explicitly where it is ruled (§The ruling record).

## The cross-kit ruling (why this entry stayed design-pending)

**evidence-kit owns the lock at both ends. lifecycle-kit contributes only the
hook it already ships.**

The entry left open "which kit holds the lock", calling it a cross-kit ruling
rather than an implementation choice. It is ruled here on three grounds:

1. The lock is a property of the **producer's run**, and evidence-kit owns the
   producer (`bin/run-validate.sh`) and the scratch directory it would live under
   (`EVIDENCE_KIT_TMP_DIR`).
2. The reader need not live in lifecycle-kit. `LIFECYCLE_KIT_ENTRY_PREFLIGHT` is
   already a generic per-stage hook that names no evidence surface in the kit —
   evidence-kit/SPEC.md §lifecycle-kit integration states the seam, and
   lifecycle-kit/SPEC.md §bin/enter-stage.sh states it from the other side ("a
   downstream kit whose gate is the real precondition for a stage wires itself
   here"). A second evidence-kit gate on that roster therefore adds **no new
   cross-kit dependency at all**; the consumer wires it, exactly as it already
   wires the manifest gate.
3. The rejected alternative is worse in a way worth recording: a lock held by
   lifecycle-kit would force lifecycle-kit to know `EVIDENCE_KIT_TMP_DIR` — a
   downward dependency from the lifecycle kit onto one specific producer kit —
   and it would generalize wrongly, since lifecycle-kit would then have to model
   every possible producer's lock rather than one hook that any producer's gate
   can hang from.

**The reader is a new gate, not a fourth assertion on `check-evidence-manifest`.**
That gate's `# spec:` header scopes it to manifest *content* — the close-entry
green block, the grammar, the stamp coupling. Liveness is a different class (the
entry's own words: "Neither is a coverage hole; both are liveness holes"), and a
separate gate earns its own `good/`+`bad/` fixture pair under gate-sdk's
fixture-pair contract instead of widening an existing gate's charter.

## The stale-lock policy (the second reason it stayed design-pending)

**The lock records the owning PID, and it is held if and only if that PID is
alive.** A leaked lock is therefore self-invalidating — the same shape this repo
already relies on for the session-role marker, whose id match means "a stale
marker self-invalidates" (lifecycle-kit/templates/lead.md).

An **age-based TTL is rejected outright.** A long validate run outlives any
honest TTL, and a long run is precisely the case the sentinel exists for: a TTL
tuned short enough to reclaim a crashed run promptly is guaranteed to declare a
healthy long run dead, which restores the false-green this amendment removes.

**PID reuse is a named, accepted residual.** A recycled PID yields a false
*held* reading, which refuses a stage entry that could have proceeded. That
direction is fail-closed and costs one file deletion to clear, whereas the defect
being removed is a false *free* reading that costs the next session an evidence
file changing underneath it. Reading `/proc` to confirm the process identity is
rejected as well: it is unportable, and the trajectory's OS-reach objective makes
a Linux-only predicate a cost rather than a refinement.

The same false-*held* direction reaches a second reader once delta 6 ships:
`run-validate`'s own writer-side refusal reads the identical PID-liveness
predicate, so a recycled PID makes the *writer* over-refuse (declining to start)
by the same mechanism that makes the entry-side reader over-refuse. Both
manifestations share one cause, one direction (fail-closed, never false-green),
and one clearance (delete the file); this is a restated instance of the residual
above, not a second one to weigh separately.

The **reclaim path** the runtime-artifact lifecycle rule demands
(lifecycle-kit/templates/stages/close.md step 6 — "a write-path needs a paired
reclaim-path") is named in three layers, and the amendment asserts all three:

1. `trap … EXIT` in `run-validate`, which covers every exit path.
2. The reader's PID-liveness predicate, which makes a leaked file inert.
3. The `.tmp/` boundary wipe at the next scope entry, which removes the file.

No `close-surface:` declaration is owed: `check-close-surfaces` requires one for
capture-tier members of the **workflow directory**, and the lock lives under
`EVIDENCE_KIT_TMP_DIR` (the scratch tier). That is exactly the artifact
close.md's step 6 calls "the artifact *outside* that directory", whose reclaim
stays a judgment — discharged by the three layers above.

## What changes

Each delta carries its `{mechanical | design-bearing}` work class.

1. **`EVIDENCE_KIT_LOCK_FILE` knob** — {mechanical}
   Declared in `evidence-kit/lib/evidence.sh` beside the existing roster, using
   the same guarded-assignment idiom the file already uses; default
   `run-validate.lock` under `EVIDENCE_KIT_TMP_DIR`. Its value is stated in
   `evidence-kit/SPEC.md` §Layout and configuration and nowhere else
   (`check-knob-citation`), and the literal in the loader must equal what that
   SPEC states (`check-knob-default-coupling`).

2. **Lock claim and release in `bin/run-validate.sh`** — {design-bearing}
   The claim publishes the owning PID and the run key to `EVIDENCE_KIT_LOCK_FILE`.
   Placement is asserted, not left to the implementer: **after** the three
   preflight guards and the `mkdir -p "$EVIDENCE_KIT_TMP_DIR"` (a run that
   refuses to start must not claim), and **before** the batch file is created (so
   no evidence work happens outside the lock's cover).

   **The claim is atomic create-exclusive, never check-then-write.** See §The
   atomicity ruling below for why this is required rather than careful. The
   asserted property is two-part: the claim **succeeds for exactly one producer**,
   and the lock is **fully populated or absent**, never half-written. The
   sanctioned form is the one this file already uses to publish the manifest —
   build the record in a temp file under the same scratch dir, then `ln` it into
   place, which fails if the target exists. `mkdir` and a `set -C` redirect are
   atomic on the first half only: each leaves a window where the lock exists and
   its record does not, which the reader would have to parse around.

   **The release is conditional — remove only if the lock is still ours.** The
   `trap … EXIT` compares the recorded PID against `$$` and removes nothing on a
   mismatch. An unconditional `rm -f` is wrong for a reason worth stating,
   because it reproduces this unit's own defect inside the unit's own mechanism:
   the lock is a single-holder artifact, so with an unconditional claim *and* an
   unconditional release, whichever producer exits first deletes the survivor's
   lock, after which the preflight reads green with a producer still live.
   Atomic claim does **not** make this redundant, and the residual case is
   concrete, though it is not the one an `EXIT`-trap race first suggests: an
   `EXIT` trap runs synchronously as part of a process's own exit and is fully
   complete before that process reads as dead to `kill -0` (the sole exception,
   SIGKILL, skips the trap entirely rather than deferring it — verified by
   probe, `.tmp/align-trap-probe.sh`: a process inside its own `EXIT` trap still
   reads **live**). So a producer this design's own dead-PID reclaim has
   correctly identified as stale cannot later run its trap — either the trap
   already ran (and the producer released its own lock before any staleness was
   observable) or it never runs at all. The real residual case is a lock removed
   by any path *other than* delta 6's dead-PID reclaim: an operator manually
   deleting an apparently-stuck lock, or a future code path. There, producer A
   is still alive, unaware its lock was removed; producer B claims the freed
   slot; A eventually exits and, with an unconditional release, deletes B's live
   lock. Atomicity is what makes "still ours" *answerable* — a record that
   cannot be overwritten is a record whose PID can be trusted — and conditional
   release is what acts on the answer, for exactly this out-of-band-removal
   case rather than for the reclaim path the design itself controls.

   A trap is required rather than a tail line, and the reason is countable: the
   script has nine exit paths — six `exit 2` guards, one `exit 1` parse failure,
   two `fail_closed` sites — plus its terminal exit, so a tail-line release would
   leak the lock on every failure path, which is the population that matters most
   (a crashed run is exactly when a stale lock appears). The idiom precedent is
   `lifecycle-kit/bin/enter-stage.sh`, which claims temp files under the same
   scratch dir with `trap … EXIT` and disarms the trap once ownership transfers.
   Design-bearing because the placement, the atomicity and the release predicate
   are each load-bearing, and because the script installs no trap today, so the
   claim introduces the file's first one.

3. **New gate `check-producer-liveness`** — {design-bearing}
   An evidence-kit gate. It reads `EVIDENCE_KIT_LOCK_FILE`; it is green when the
   file is absent or names a dead PID, and red when it names a live one, printing
   the blocking run key so the operator can tell "wait" from "reclaim". Copied
   from `gate-sdk/templates/check-skeleton.sh` and shipping the `good/`+`bad/`
   fixture pair, per gate-sdk's four gate contracts. Design-bearing: the
   liveness predicate and the fixture pair for a *process-liveness* gate are both
   non-obvious (a fixture cannot portably hold a live PID, so the bad case must
   be built around a PID the fixture itself owns).

4. **Consumer wiring** — {mechanical}
   `scripts/lifecycle-config.sh` gains the gate on `LIFECYCLE_KIT_ENTRY_PREFLIGHT`
   at **two** stage keys, `validate=` and `close=`, beside the existing manifest
   entry. `close=` is the case the entry filed. `validate=` is added because the
   preflight roster is an exact per-stage match and a second validate batch
   entering while a first batch's `run-validate` is live is the same hazard with
   a worse outcome — two producers folding the same manifest. This is consumer
   config, not asserted kit behavior: a consumer whose producer runs at another
   stage wires its own keys.

5. **evidence-kit/SPEC.md sections** — {design-bearing}
   The knob roster, the writer contract, a new gate section, the seam paragraph,
   and the causal roster, each named under §Existing sections updated below.

6. **Writer-side refusal in `bin/run-validate.sh`** — {design-bearing}
   `run-validate` refuses to start while a live lock is held, exiting non-zero
   with the blocking run key. **Operator-ruled 2026-08-06**; see §The ruling
   record below for the refused alternatives and their grounds.
   The refusal predicate is the same PID-liveness test the reader uses, and it
   falls out of the atomic claim rather than being a second mechanism: the claim
   either succeeds — in which case no live holder existed — or fails, and the
   failure branch is the refusal.
   **The stale-reclaim protocol, bounded and asserted:** on a failed claim, read
   the existing lock; a **live** PID refuses immediately; a **dead** PID is
   reclaimed by removing the lock and retrying the claim **exactly once**. A
   second failure refuses rather than looping — it means another producer won the
   reclaim race, so its lock is live and refusing is the correct answer. This
   resolves the two-contender stale case without an unbounded retry: both may
   remove and relink, exactly one `ln` succeeds, and the loser's re-read finds a
   live PID.
   Design-bearing: the refusal predicate, the bounded reclaim, and the exit code
   are all new asserted behavior on a shipped surface.

## Producers and consumers

**New state: the lock file.**

- **Producer** — `bin/run-validate.sh`, at the claim point in delta 2. Its
  enabling config is `EVIDENCE_KIT_LOCK_FILE`, which carries a default in the
  kit loader, so it resolves in every deployed configuration rather than only
  under a test harness. The producer is reachable on the ordinary path: the
  validate stage skill runs `run-validate`, which is the only writer of the
  evidence manifest.
- **Consumer, first** — `check-producer-liveness`, invoked by
  `lifecycle-kit/bin/enter-stage.sh`'s preflight loop with the argv that loop
  already passes (`<queue> <state>`), at the `validate` and `close` stage keys
  the consumer wires. The mechanism is the existing hook; nothing new couples the
  two kits.
- **Consumer, second — `enter-stage.sh --simulate <stage>`, and it is the one
  worth naming explicitly.** The read-only preflight mode runs *every matching*
  `LIFECYCLE_KIT_ENTRY_PREFLIGHT` entry, so it inherits this gate with no extra
  wiring. That matters beyond bookkeeping: a lead gating an expensive dispatch
  with `--simulate` is today blind to a live producer — the incident behind
  `lead-dispatch-requires-completion-notification` is precisely a simulated close
  entry clearing mid-write — and once this lock ships, that same command reports
  the live run instead of clearing. The reader was found by surveying the
  component set rather than the obvious call path, and it is the highest-value
  consumer of the artifact.

  **It does not make the paired prose rule redundant**, and the boundary must be
  stated or the pair looks over-built: `--simulate` remains an instantaneous
  read, so a producer that starts a second later is still unseen, and the gate
  covers only producers that claim this lock. A lead that dispatches on artifact
  state is still dispatching on artifact state — it merely has one more artifact.
  The prose rule governs what the lead waits *for*; this gate narrows what
  survives being wrong about it.

**Every field has a named reader.**

- `pid` — read at **three** named transitions, all by the same liveness
  predicate:
  1. `check-producer-liveness`, at the stage-entry transition (and, inheriting
     it, at `--simulate`). It is the whole verdict.
  2. `run-validate` itself, at the **run-start** transition, on a failed atomic
     claim — the refusal predicate of delta 6.
  3. `run-validate`'s `EXIT` trap, at the **release** transition, compared
     against `$$` to answer "is this still ours" — the release predicate of
     delta 2.
- `run key` — read when a refusal line is composed, at two transitions: by
  `check-producer-liveness` at stage entry, and by `run-validate` at run start
  (delta 6). Its reader is the operator standing at a refusal, who must choose
  between waiting for the named run and reclaiming a lock whose owner is gone; a
  refusal that cannot name the run leaves that choice unserved.
- A **start timestamp was considered and removed.** It has no named reader once
  the stale policy is PID-liveness rather than age, and the causal-completeness
  rule removes a field with no reader rather than carrying it for plausibility.

**No existing field changes**, and the evidence line's grammar is untouched — a
liveness lock is a separate artifact precisely so the manifest's contract, which
`check-evidence-manifest` asserts and the forward attestation payload consumes,
stays fixed.

## Existing sections updated

- `evidence-kit/SPEC.md` §Layout and configuration — the knob roster gains
  `EVIDENCE_KIT_LOCK_FILE` with its default. Owned by delta 1.
- `evidence-kit/SPEC.md` §bin/run-validate.sh — the writer contract gains the
  claim/release pair, the placement assertion, the atomicity property, and the
  refusal-plus-bounded-reclaim behavior. The section currently describes the
  spine as guards → per-suite loop → single fold, and describes the tool's own
  exit as non-zero only when a suite records `new-failures`; both statements
  change, since the lock now bounds the spine and a held lock is a second
  non-zero exit. Owned by deltas 2 and 6.
- `evidence-kit/SPEC.md` — a new §check-producer-liveness section beside the
  existing gate sections. Owned by delta 3.
- `evidence-kit/SPEC.md` §lifecycle-kit integration — this section currently
  describes `LIFECYCLE_KIT_ENTRY_PREFLIGHT` as running *this gate* (the manifest
  gate) as a close-entry preflight. It becomes two gates at three stage keys, and
  the singular framing is updated rather than appended to. Owned by delta 4.
- `evidence-kit/SPEC.md` §Producers and consumers — gains the lock row in the
  same shape as the existing evidence-line and suite-roster rows. Owned by
  delta 5.
- `lifecycle-kit/SPEC.md` §bin/enter-stage.sh — **prose only, no contract
  change.** Its sentence naming evidence-kit's manifest gate as *the* example of
  a downstream kit wiring itself into the preflight now has a second example.
  Recorded here so build does not read it as a lifecycle-kit change: the hook's
  grammar, argv and refusal semantics are unchanged, and no lifecycle-kit knob
  moves. Owned by delta 4.

## The ruling record

**Operator-closed 2026-08-06: writer-side refusal is taken** (delta 6). The
deciding argument is that a lock the producer itself does not check is not a
mutex: a session can run `run-validate` without entering a stage, so the
entry-side red alone leaves two producers able to race the manifest with every
stage entry green.

This is a **deliberate widening, not a correction of an under-filing.** The queue
entry's original scope — the entry-side preflight red — was congruent with the
defect it named, which was the *observability* gap. Delta 6 adds prevention on
top of detection because the ruling judged detection insufficient, and the record
says so plainly rather than letting a later reader infer the original filing was
sloppy.

Two alternatives were weighed and **refused**, recorded with their grounds:

- **Entry-side red plus a stated limit, with the unguarded path filed as a new
  gap.** Refused because the unguarded path is not a completeness worry to be
  filed and forgotten — it is *documented behavior in this repo*.
  `delegation-kit/SPEC-verify-verb.md` records a lead moving to re-run
  `run-validate` to "verify" the validate stage, operator-caught, in a defect
  class that fired in two consecutive iterations. Filing a gap against a path
  something already walks defers a known live hazard.
- **Entry-side red plus a stated limit alone.** Refused on enforcement-first: the
  fix and the check that catches it land in one unit, and a stated limit is
  neither.

## The atomicity ruling

**The lead raised a check-then-write race and asked for it to be ruled against
the source rather than accepted. Ruled: the race is real, and taking delta 6 is
what introduces it.**

Delta 2 as originally written was an *unconditional* write with no predicate, so
no time-of-check window existed — there was no check. Delta 6 adds one, and a
naive read-then-claim would let producer A and producer B both observe a clear
lock and both claim, B's record overwriting A's. That reintroduces precisely the
two-producer case delta 6 was taken to close, and it would silently defeat
conditional release as well, since "still ours" cannot be answered from a record
that another producer overwrote.

Hence the atomic create-exclusive asserted in delta 2. The claim's success *is*
the check, so there is no interval between them to lose. The verification behind
this ruling is that `bin/run-validate.sh` already publishes the manifest by
building a temp file and moving it into place, so an atomic-publish idiom is
house-consistent here rather than newly imported; the lock's variant differs only
in using a link, which fails on an existing target instead of replacing it.

The reader gains a guarantee from the same property, which is why it is asserted
as two-part: because the record is published whole, `check-producer-liveness`
never has to parse a partially written lock or decide what an empty one means.

Nothing here widens the envelope further — the atomicity, the bounded reclaim and
the release predicate are all mechanics inside delta 2 and delta 6's stated
intent, so they are settled here rather than escalated.

## Out of the envelope, stated so build does not drift into it

Neither the recurrence counter's input (`recurrence-drain-input-widening`) nor
the recurrence resolver's discriminator (`gap-resolver-mention-overcount`) is
touched by this amendment, though both are filed against the same machinery. This
gate's red is not a recurrence declaration and must not stamp one.

## Definition of Done

- [ ] **Causal completeness** — the lock has a named, reachable producer and a
      named consumer; `pid` and `run key` each have a named reader at a named
      transition.
- [ ] **The pair ships together** — this amendment and
      `lifecycle-kit/SPEC-dispatch-signal.md` are both merged, or neither is.
      Landing this oracle while the dispatch rule it backs stays unstated is the
      failure the set exists to end.
- [ ] **Merged with no information lost** — the cross-kit ruling and the
      stale-lock rationale land in `evidence-kit/SPEC.md`'s prose, not as an
      appendix; the merged spec reads as one document.
- [ ] **Knob gates green** — the default is stated in `evidence-kit/SPEC.md`
      alone (`check-knob-citation`) and matches the loader literal
      (`check-knob-default-coupling`).
- [ ] **Gate contracts green** — `good/`+`bad/` fixture pair, fail-closed
      behavior, output shape, self-lint, per gate-sdk's four contracts.
- [ ] **The claim is atomic** — create-exclusive, not read-then-write, and the
      record is published whole. A claim that reads before writing has
      reintroduced the two-producer case delta 6 was taken to close, and has
      defeated the release predicate with it.
- [ ] **The release is conditional** — the `EXIT` trap removes the lock only when
      the recorded PID is still ours; an unconditional `rm -f` reproduces this
      unit's own defect inside its own mechanism.
- [ ] **The writer refuses a live lock** — `run-validate` exits non-zero and
      names the blocking run key when the atomic claim fails against a *live*
      PID, refusing immediately with no reclaim attempt. This is delta 6's
      headline behavior — distinct from the bounded-reclaim path the next bullet
      tests, and a build that only implements reclaim has not implemented the
      refusal the operator ruling actually took.
- [ ] **Reclaim is bounded** — a dead-PID lock is reclaimed with exactly one
      retry, and a second failure refuses rather than looping.
- [ ] **Amendment deleted** — this file removed on merge; `ls evidence-kit/SPEC-*.md`
      returns none.
- [ ] **Removals propagated** — grepped every spec for names this change retired;
      nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks.
