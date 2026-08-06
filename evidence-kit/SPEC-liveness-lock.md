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
still-running work emits a signal that lies. This lock is the detector for the
residual case where the signal is wrong anyway — which is precisely why it is
worth building even though the two prose rules exist.

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
   The claim writes the owning PID and the run key to `EVIDENCE_KIT_LOCK_FILE`.
   Placement is asserted, not left to the implementer: **after** the three
   preflight guards and the `mkdir -p "$EVIDENCE_KIT_TMP_DIR"` (a run that
   refuses to start must not claim), and **before** the batch file is created (so
   no evidence work happens outside the lock's cover).
   The release is `trap 'rm -f …' EXIT`, installed at claim time. A trap is
   required rather than a tail line, and the reason is countable: the script has
   nine exit paths — six `exit 2` guards, one `exit 1` parse failure, two
   `fail_closed` sites — plus its terminal exit, so a tail-line release would
   leak the lock on every failure path, which is the population that matters most
   (a crashed run is exactly when a stale lock appears). The idiom precedent is
   `lifecycle-kit/bin/enter-stage.sh`, which claims temp files under the same
   scratch dir with `trap … EXIT` and disarms the trap once ownership transfers.
   Design-bearing because the placement is load-bearing and because the script
   installs no trap today, so the claim introduces the file's first one.

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

## Producers and consumers

**New state: the lock file.**

- **Producer** — `bin/run-validate.sh`, at the claim point in delta 2. Its
  enabling config is `EVIDENCE_KIT_LOCK_FILE`, which carries a default in the
  kit loader, so it resolves in every deployed configuration rather than only
  under a test harness. The producer is reachable on the ordinary path: the
  validate stage skill runs `run-validate`, which is the only writer of the
  evidence manifest.
- **Consumer** — `check-producer-liveness`, invoked by
  `lifecycle-kit/bin/enter-stage.sh`'s preflight loop with the argv that loop
  already passes (`<queue> <state>`), at the `validate` and `close` stage keys
  the consumer wires. The mechanism is the existing hook; nothing new couples the
  two kits.

**Every field has a named reader.**

- `pid` — read by `check-producer-liveness`'s liveness predicate, at the
  stage-entry transition. It is the whole verdict.
- `run key` — read by `check-producer-liveness` when it composes its refusal
  line, at the same transition. Its reader is the operator standing at a refused
  entry, who must choose between waiting for the named run and reclaiming a lock
  whose owner is gone; a refusal that cannot name the run leaves that choice
  unserved.
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
  claim/release pair and states the placement assertion. The section currently
  describes the spine as guards → per-suite loop → single fold; the lock bounds
  that spine and the prose must say where. Owned by delta 2.
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

## Considered and deferred

**Writer-side refusal — `run-validate` refusing to start while a live lock is
held.** This is the natural mutex reading, and it closes a hole the preflight
cannot: a session can run `run-validate` without entering a stage, so two
producers can still race the manifest with every stage entry green. It is
recorded here rather than asserted above because it **widens `run-validate`'s
asserted behavior** — a new refusal on a shipped surface — beyond the deliverable
the queue entry filed, which names only the entry-side red. Escalated to the
lead; if taken, it becomes a sixth delta {design-bearing} and a second named
consumer of the `pid` field, at the run-start transition.

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
- [ ] **Amendment deleted** — this file removed on merge; `ls evidence-kit/SPEC-*.md`
      returns none.
- [ ] **Removals propagated** — grepped every spec for names this change retired;
      nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks.
