# SPEC amendment: value-arm

`installer_smoke`'s value arm is baseline-held `fail`
(`.workflow/validate-baseline.txt`: `installer_smoke … fail
port-criterion-aggregate-cost-blindness`). This amendment rules the fix. It does
not restate §The consumer smoke's arm list, §Profiles' lattice contract, or
§The gate binary's selection model.

## The measurement, taken live rather than read off the baseline row

Run to completion at `143988ad`, all four profiles, from the main checkout:

| Profile | Battery | Omitted-and-declared | Planted defect |
|---|---|---|---|
| `starter` | 10 gates | 0 | **green** (not caught) |
| `delegation` | 16 gates | 5 | **green** |
| `prose` | 19 gates | 6 | **green** |
| `full` | 25 gates | 11 | **green** |

```
INSTALLER-SMOKE: FAIL — no profile's battery caught the planted prose defect —
the install is green, idempotent and reversible, and worth nothing on a document
```

**The failing assertion is the first sentence of the value claim, not the
second, and the queue entry has this one degree off.** The entry frames the hole
as "no profile *below the maximum*" catches the class. The live verdict is
stronger: **`full` does not catch it either.** An adopter who takes every kit in
the payload gets no markdown-link governance at all when their host has no
artifact. That difference decides the design, because it removes one of the three
candidate answers outright — there is no shell gate anywhere in the payload, at
any profile, that reds on a broken relative link in adopter markdown, so
"redesign the value assertion off binary-dispatched gates" has nothing to
redesign onto.

**The omitted counts are unchanged** from the entry's 2026-08-12 measurement
(0/5/6/11), so the entry's numbers are current and only its framing moved.

**Why `prose` is the profile that matters here.** Its roster criterion
(`installer/profiles.list`) promises *"link, claim, staleness and pointer
governance over every README.md at any depth"*, and the arm plants its defect in
`docs/README.md` for exactly that reason: `spec_manifest_files`' default walk is
`gate_find "$root" -name 'README.md' -type f` (`canon-kit/lib/spec.sh:221`), a
README at any depth. The gate that reds is `check-md-refs`, which the canon-kit
cohort made a `.gate`. So the profile whose entire stated purpose is prose
governance is the profile the hole empties.

## The premise that turns out to be the actual defect

§The port-candidate criteria, criterion 5 rules that a cohort's binary-less
residual is *"measured, never reasoned — the oracle exists and is
`installer_smoke`'s value arm"*.

**The value arm is not an oracle for the residual, and treating it as one is
what let this land.** The arm asserts a claim about **the product an adopter
installs**: some profile below the maximum catches a real defect. It reads as a
residual oracle only because `scripts/pack-installer.sh` packs **no artifact** in
the smoke's main loop — visible in the run's own first line, `0 prebuilt gate
binary/binaries` — so every profile in the loop happens to be an
uncovered-platform install. That is an accident of the harness, borrowed as a
contract. The consequence is precisely the failure recorded: a cohort satisfied
criterion 5 per member, the aggregate emptied a class, and the assertion that
noticed was the one about the *product*, which then reads as broken rather than
as reporting.

Both claims are worth holding. They are two claims and they need two arms.

## What changes

**(1) The main loop packs a real gate binary.** [design-bearing] Every profile's
battery becomes the battery an adopter on a covered platform actually receives,
which is what the value claim has always been about. This costs no new
dependency: the smoke's preflight **already requires `cargo` and `rustc`**,
because the artifact arm builds the binary it packs
(`installer/consumer-smoke/run-smoke.sh:19`), and that arm already proves build,
pack, digest verification and placement end to end. The main loop reuses that
machinery rather than inventing a second one.

**A host-built artifact is a harness stand-in and must be labelled one.**
§Consumer payload rules that the payload carries a prebuilt binary per declared
target, *"built by the release and never from a working tree"*. The smoke builds
from the working tree because it has no Release to draw on; the artifact arm
already takes that liberty under the same reasoning. The amendment states it
where the main loop takes it too, so a later reader does not read delta (1) as
the payload rule being quietly relaxed.

**(2) A named binary-less leg replaces what the main loop was measuring by
accident.** [design-bearing] Deleting the uncovered-platform case along with the
accident would destroy the only instrument criterion 5 has. So one profile —
`prose`, the one whose criterion the hole empties — is additionally installed
**with no artifact packed**, and that leg asserts **disclosure rather than
catching**:

- every member the profile loses is recorded `# omitted: <name> <reason>` in the
  consumer's `gates.list`, which `run-smoke.sh:232` already asserts equals the set
  this payload dispatches to a binary;
- and the omitted count is **non-zero**, so the leg cannot pass vacuously on a
  payload that dispatches nothing — the anti-vacuity half, which is the assertion
  the present arm does not carry and the reason a per-member reading passed a
  cohort that emptied a class.

The leg deliberately does **not** assert that the defect goes uncaught. Pinning a
missing capability as expected behavior would make the hole permanent the moment
it is fixed.

**(3) Criterion 5 gets its own oracle, named rather than borrowed.**
[design-bearing] The residual a cohort must record is the **omitted roster and
its count**, which delta (2)'s leg derives per profile and prints, and which
`run-smoke.sh:232` holds complete against the payload's dispatch set. Criterion
5's text is corrected to cite that, so a cohort records *the roster it grew*
rather than a pass/fail it never owned. This also closes the criterion's own
stated honest limit — *"Nothing forces a future cohort to take the measurement:
the obligation is prose on the porting session"* — one notch: the count is now
machine-derived and machine-asserted-complete, and what stays prose is only the
cohort's decision about whether the grown roster is acceptable.

**(4) The release blocker is discharged by the fix, not by a new gate.**
[mechanical] The entry rules this blocking at **release-sweep** rather than at
close, so that a publish cannot ship the hole unnoticed. Delta (1) retires the
red, and `installer_smoke` is already a validate suite that release-sweep's
own preflight reads — so no new mechanism is owed and none is added. Stated
because the entry's blocking clause invites one.

**(5) The baseline row retires with the entry.** [mechanical] The
`installer_smoke` row moves from `fail` to `pass` and drops its slug when the
suite goes green. `check-evidence-baseline` requires a `fail` row's slug to
resolve to a live queue task, so the row and the entry's Done move happen
together or the gate reds — close's step, not build's.

**(6) The sibling half stays split and is not re-merged.** [mechanical] Half (1)
of this entry promoted 2026-08-12 as `port-criterion-cohort-cost-form`, which
gives criterion 5 a form that can price a cohort. Delta (3) touches the same
criterion from the measurement side. The two are compatible — one is the shape of
the price, the other is where the number comes from — and a build session must not
fold them, because the other half is not in this iteration.

## Producers and consumers

**Changed behavior: the main loop's packed payload carries an artifact (delta 1).**
- *Producer* — `run-smoke.sh`'s pack step, building the crate for the host target
  before invoking `scripts/pack-installer.sh`, the same sequence the artifact arm
  runs today.
- *Consumers* — `init`'s `select_artifact()` and its binary-write block, which
  resolve the host to a target, verify the artifact against its digest and place
  it; then `gate_command` in each scratch consumer, dispatching every `.gate`
  member through the placed path.
- *Named reader at a named transition* — the placed binary is read by each
  profile's battery at `assert_install`, and by the value arm's two battery runs
  at `assert_value`. No field is added to the manifest: §The gate binary already
  records the artifact as an ordinary `files` entry with `artifact.target` and
  `artifact.digest`, and `run-smoke.sh:215-227` already asserts that whole
  placement branch — it simply never ran in the main loop.
- *Enabling config actually emitted* — none is added. The seam line
  `GATE_SDK_NATIVE_BIN` is written by `init` on the placement branch, which is the
  branch this delta makes the main loop take.

**New state: `VALUE_VERDICT` under a no-artifact payload (delta 2).**
- *Producer* — the binary-less `prose` leg, one battery run.
- *Consumer* — the leg's own disclosure assertions; it does **not** feed
  `VALUE_RED`, and this is the field-with-a-reader question answered explicitly:
  a verdict recorded into the aggregate claim would re-create the conflation
  delta (3) is removing.
- *Every field has a named reader* — the leg reads the omitted roster (from the
  consumer's `gates.list`) and its count. Both are read at one transition, the
  leg's assertion block, immediately after `init`. Nothing else is recorded.

**Red conditions of the readers this change touches.** §The causal-completeness
check point 5 binds because delta (1) **widens** a consumer's gate set and delta
(2) **narrows** one, and a narrowing is where the non-monotone readers live.

- **The aggregate value assertion** (`run-smoke.sh:323-328`) — reds on an empty
  `VALUE_RED` (a **zero count**, non-monotone) and again when every catching
  profile equals `PROFILE_DERIVED`. Delta (1) is what clears the first; the second
  is cleared by `prose` catching the defect, which is the mechanism verified above
  (`docs/README.md` is in the default walk, `check-md-refs` reds on the mistyped
  link). **If `prose` still fails to catch it with a binary placed, the finding is
  a different and larger one** and the build session stops rather than widening the
  assertion — stated because "make the assertion pass" is the wrong repair here.
- **The omission-completeness assertion** (`:232`) — reds when the registry's
  omitted set differs from the set this payload dispatches. Under delta (1) the
  main loop takes the *placement* branch instead, where `:226` reds if anything is
  omitted at all; under delta (2) the leg takes the omission branch. Both branches
  are already written and already asserted; the delta changes which arm reaches
  which, not what either asserts.
- **The per-profile battery assertion** (`:135`) — reds when the freshly-installed
  battery is not green. Delta (1) adds dispatched members to every profile's
  battery, so a member that is red in a fresh consumer now surfaces here rather
  than being omitted past. That is the delta's intended reach and not a
  regression, but it is the assertion most likely to red first when this lands.
- **The manifest agreement assertion** (`:154`) — reds on any recorded entry
  disagreeing with the tree, and on a **zero** checked count (`:155`,
  non-monotone). The artifact adds entries rather than removing them, so it is
  monotone here and clearable by inspection.
- **The gate-roster monotonicity assertion** (`:108`) — reds when a smaller
  profile's gate set is not contained in a larger's. Delta (1) adds dispatched
  members to **every** profile from the same payload, so containment is preserved;
  delta (2) changes no roster, because omission is applied per install rather than
  per profile.
- **`check-evidence-baseline`** — reds on a `fail` row whose slug resolves to no
  live queue task. Delta (5) is the discharge, at close.

## Existing sections updated

- **installer/README.md §The consumer smoke** — owned by deltas (1)-(3): the arm
  list gains the packed artifact in the main loop and the binary-less `prose` leg,
  and the "value claim asserted over the loop" paragraph gains the distinction
  between the **coverage** claim (what an adopter on a covered platform gets) and
  the **disclosure** claim (what an adopter on an uncovered one is told). Its
  existing rule that neither sentence names a gate or a profile survives
  unchanged — the leg names a profile because it is a *scoping* choice about which
  install to run, not a derivation of which profiles catch what.
- **installer/README.md §The gate binary** — owned by delta (1): the main loop now
  drives the placement branch, so the section's account of which arm exercises
  which outcome is corrected, and the host-built stand-in is labelled where the
  payload rule is cited.
- **gate-sdk/SPEC.md §The port-candidate criteria, criterion 5** — owned by delta
  (3): the residual's oracle becomes the omitted roster and its completeness
  assertion rather than the value arm's verdict, and the paragraph that calls the
  value arm the oracle is rewritten. Its "N members each individually runnable is
  not a discharge" clause is untouched and is what delta (3) gives an instrument.
  The honest-limit paragraph is narrowed rather than deleted.
- **gate-sdk/SPEC.md §Consumer payload** — owned by delta (1): one sentence saying
  the smoke's host-built artifact is a harness stand-in, so the "never from a
  working tree" rule reads as the payload rule it is rather than as one the smoke
  breaks.
- **`.workflow/validate-baseline.txt`** — owned by delta (5), at close.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls installer/SPEC-*.md`), discharged at the iteration rather
      than at the commit while sibling amendments are in flight.
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
