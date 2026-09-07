# SPEC amendment: platform-coverage

**The born-native flip attaches criterion 5's omission to every future gate, and nothing measures
the pile.** This amendment mints the instrument: `check-install-platforms`, a born-native gate
that holds the install page's platform declaration and `native/targets.list` in lockstep, refuses
an uncaused hold, and prints the omitted-member count per uncovered platform on every run — clean
as well as red.

**This is the costed residue of a closed ruling, not a challenge to it.** `TRAJECTORY.md`:277-291
records the 2026-08-14 flip to native-by-default and states, in the same breath, that "the ruling
does not widen the roster; the residue is costed as `born-native-omission-accumulation`". This
amendment discharges that costing. Nothing here re-opens the flip, and nothing here widens the
roster — the sibling unit `gate-binary-roster-covers-supported-platforms` owns that and this gate
is what will hold its result.

## The instrument choice, which is the ruling this amendment owes

The queue entry left three options open: **a measurement** (a count of omitted members per
uncovered target, so the pile is visible), **a bound** (a ceiling on how far the omitted set may
grow before widening is forced), or **nothing** beyond the exception rule the flip's own amendment
states. Ruled at spec 2026-09-08, in-envelope: **a bound, made reddenable, with the measurement
riding it.**

**Why not "nothing".** The entry's own words for what is missing are that "nothing would redden if
it grew without bound". An exception rule governs whether a *new gate* may be shell; it says
nothing about the aggregate, and it cannot, because the aggregate is a property of the roster and
the exception rule never reads the roster.

**Why not a measurement alone.** A count with no verdict is a number in a log that no run acts on.
This tree's own history on the point is `native/targets.list`'s: a fact recorded in a header and
read by nothing drifted for weeks while its owner retired out from under it.

**Why not a raw ceiling.** A policy number — "widening is forced at N omitted members" — is a
literal that de-literalization refuses: it would have to be maintained against a battery that
grows every iteration, and it answers the wrong question. The pile is not bad because it is large;
it is bad because a supported platform is uncovered.

**So the bound is structural rather than numeric: zero uncovered supported platforms, and no
uncaused hold.** Every platform the install documentation declares supported is either in the
roster or explicitly held with the run that would join it. The count then rides along as what a
reader needs to weigh a hold, not as the thing being bounded.

**It is green on landing, and that is a design fact rather than a temporary state.** Under the
operator's 2026-09-07 ruling the roster is still one line when this gate arrives, and macOS is a
documented supported platform. A gate whose bound were "in the roster" would red at its own
landing commit and would have to be held off, which is how an instrument becomes a thing waited
out. The **held** state is what makes the bound satisfiable the day it lands while still being a
real bound: a hold is a first-class, cause-bearing outcome that a reader can act on, and it cannot
be granted silently because arm C refuses an empty cause.

## What changes

### (1) `check-install-platforms` — a born-native gate holding the declaration and the roster in lockstep

A new gate, Rust module plus `.gate` descriptor, registered in `scripts/gates.list`
{design-bearing}.

It is **born native** with no exception argued, per CLAUDE.md's standing rule, and it is a
**repo-root** gate rather than a kit member — the same placement its sibling
`check-install-toolchain` takes, and for the same reason: its corpus is this repo's install page
and this repo's roster, so a kit would be shipping a gate whose subject no consumer has.

Its two operands are positional with repo-layout defaults, exactly as `check-install-toolchain`'s
are (`native/src/gates/install_toolchain.rs`:134-135): the install page, and the target roster. **No
new knob is minted**, and the reason is not laziness — it is the ruling that section already
records for its sibling, that a knob whose only reader would be `check-knob-citation` is not
configuration.

Four arms and one output line:

- **A — a `joined` declaration must be a live roster line.** Red when the block declares a triple
  `joined` and `native/targets.list` carries no such live line. This is the half that catches a
  declaration flipped ahead of the roster write.
- **B — a live roster line must be a `joined` declaration.** Red when the roster carries a triple
  the block does not declare `joined`. This arm is the mechanization of `gate-sdk/SPEC.md`
  §Consumer payload's **first** bound — a roster line "may not exceed what the project's own
  install documentation already states" — which today is held by prose alone and by whoever
  remembers to read it.
- **C — a `held` declaration carries a cause and is absent from the roster.** Red on an empty
  precondition, and red on a triple declared `held` that is nonetheless in the roster. The first
  half is what stops a hold from being granted silently; the second is B's converse and catches a
  roster write whose declaration flip was forgotten.
- **D — the measurement.** For each `held` triple, the count of `.gate`-declared members that
  would be **omitted** on it under criterion 5, printed on the clean line as well as the red one.
  This is the entry's "count of omitted members per uncovered target", and it is on the clean line
  deliberately: a number that appears only when something is already broken is not an instrument,
  it is a post-mortem.

**Fail-closed (exit 2)**, mirroring its sibling: an install page that is not a file, a roster that
is not a file, a page carrying no marker block, a marker block carrying no declaration, and a file
the reader cannot read. Arm D's count is **not** fail-closed on a checks directory it cannot walk
— it reports the walk failure and still runs A, B and C, because a measurement that could not be
taken must not suppress three verdicts that do not depend on it.

**Fixture pair** at `scripts/gate-tests/check-install-platforms/{good,bad}/`, per the shipped-gate
contract, with `bad/` carrying one violation per arm.

### (2) The `# graph:` manifest and the pre-commit projection

The descriptor declares its couples — the install page, the roster, its own module and the shared
reader — at `precommit` tier, and the generated pre-commit hook is regenerated in the same unit
{mechanical}.

The hook is generated and never hand-edited; the fan-out a new gate stales is rostered in
`docs/site-architecture.md` §Generated projections and their freshness gates, and each freshness
gate prints its own command on red. This delta is the roster's work, not a judgement call.

### (3) The accumulation is given an owner on the surface that costed it

`gate-sdk/SPEC.md` §The port-candidate criteria, criterion 5, gains the pointer to arm D
{design-bearing}.

Criterion 5 already prices the omission "per member and paid per cohort" and names the aggregate —
"a cohort's aggregate cost is the **binary-less residual**" — and already has one instrument for it:
`installer_smoke`'s binary-less leg, which asserts a cohort's own residual against the tree at that
cohort's landing, event-triggered and scoped to that one batch. **What it has never had is a
standing one** — a count that rides every battery invocation rather than firing only when a cohort
lands, and that reads per **held platform** rather than per port batch. Arm D is that: it prints on
the clean line as well as the red one, keyed to the declaration block's `held` triples rather than to
a landing cohort, so a pile that grows between cohorts is visible before the next port batch's own
smoke would catch it. The criterion cites arm D rather than restating the count, so the one place a
porting session meets this cost is the place that says where the number comes from.

### (4) The parity contract joins the roster of what holds the install page

`docs/site-architecture.md` §Generated projections and their freshness gates gains the
install-platforms parity contract beside the install-toolchain one {mechanical}.

That section is where a reader looks to find what holds a marker block on that page, and a second
block held by a second gate with no row there is exactly the drift the roster exists to prevent.

## Producers and consumers

**New state — the gate's verdict and its per-platform omitted count (deltas 1 and 3).**

- **Producer:** `check-install-platforms`, dispatched through the gate binary's subcommand arm and
  run by `gate-sdk/bin/run-gates.sh` from `scripts/gates.list` at every battery invocation, and by
  the generated pre-commit hook at `precommit` tier (delta 2). Its enabling configuration is the
  registry line and the descriptor, both landing in this unit — there is no configuration a
  deployment could fail to set, which is the property criterion 1 of the causal-completeness check
  asks for.
- **Consumers:** the battery's summary and exit status; the pre-commit hook, which blocks the
  commit; and a reader of the clean line, for whom arm D's count is the whole point.
- **Fields and their named readers.** The declaration's **triple** is read by arms A, B and C at
  the comparison against the roster, and by arm D as the key its count is reported under. Its
  **state** selects which of A and C applies. Its **precondition** is read by arm C, which reds on
  an empty one, and by a human deciding whether a hold is still honest — the transition being the
  next scope intake that reads this entry's successor. Arm D's **count** is read at the clean line
  and at any red, and by criterion 5's prose (delta 3), which is where a porting session is told
  what the number means. No field is introduced that no reader consumes.

**The declaration block itself is produced by the sibling amendment**, `SPEC-platform-evidence.md`
delta 1, and is hand-authored on `docs/install.md` §Requirements. That is a real cross-amendment
dependency and it is stated rather than assumed: **this gate cannot land before that block
exists**, because arm A's corpus would be absent and the gate would exit 2 at its own landing
commit. The two are same-surface joined units of one iteration and the ordering is
declaration-then-gate.

**Existing integration prose updated:** criterion 5 (delta 3) describes the prior flow — an
omission that is recorded per member and priced per cohort with no instrument that computes the
cohort figure — and it is updated here rather than left to describe a shape the tree no longer has.

**No corpus is narrowed by any delta.** The battery's member set **grows** by one, and the install
page's governed content grows by one block; the causal-completeness check's point 5 therefore does
not bind. Stated explicitly because a new gate invites the reflex of clearing readers by
inspection, and here there is nothing to clear: no existing gate's corpus, glob or file set is
touched by any delta.

## Existing sections updated

- `gate-sdk/SPEC.md` §The port-candidate criteria, criterion 5 — its aggregate-cost paragraph,
  which names the binary-less residual and, beyond the per-cohort instrument it already cites, has
  never had a standing, per-platform one (delta 3).
- `docs/site-architecture.md` §Generated projections and their freshness gates — the parity-contract
  roster, which today carries the install-toolchain contract alone (delta 4), and the generated-hook
  fan-out a new gate stales (delta 2).
- `scripts/gates.list` — the registry the battery resolves members from (delta 1).
- `gate-sdk/SPEC.md` §Consumer payload, its two-bound paragraph — bound (i) gains the arm that
  enforces it, so the paragraph names arm B rather than leaving the bound to prose (delta 1).

<!-- update-target-exempt: native/targets.list's header is rewritten by the sibling amendment SPEC-platform-evidence.md delta 6, which owns the roster surface; this amendment only reads it, and citing a delta here for a file another unit rewrites would put two owners on one edit -->
- `native/targets.list` — read, not written, by this unit.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
