# SPEC amendment: profile-keyed-install

The profile becomes a **key the install recipe is resolved against**, and the
profile set becomes a **bounded lattice derived from the payload** instead of a
three-name chain asserted separately on five surfaces.

This is slice (a) of the `prose-profile` rung and nothing more: **no fourth
profile ships here, and no prose gate cohort ships here.** What ships is the
seam a fourth profile needs, and the oracle that will hold it.

## What changes

**D1 — `installer/lib/common/profile.sh` gains the order, derived.**
`{design-bearing}`
Two new functions in the module that already owns profile resolution:

- `profile_order()` — every ordered pair `(a, b)` of distinct selectable
  profiles where `profile_kits(a)` is a subset of `profile_kits(b)`. Derived
  from set inclusion over the existing `profile_kits`; **no new row grammar in
  `profiles.list`** and no declared parent, because a declared order and the
  kit sets could disagree, and then two surfaces would assert the chain again —
  which is the defect this amendment exists to end.
- `profile_gates(root, profile)` — the sorted, de-duplicated union of
  `recipe_gates(kit, profile)` over `profile_kits(root, profile)`.

The profile set is a **lattice**, not a chain: profiles are partially ordered by
kit-set inclusion. Two profiles may be incomparable, and incomparability is
legitimate — a prose profile sits above the minimum and below the maximum while
comparing to `delegation` in neither direction.

**D2 — `recipe_gates` takes the profile as its second argument.**
`{mechanical}`
`recipe_gates(kit, profile)`. Every existing arm ignores the second argument and
is otherwise unchanged. **Corrected at align: `recipe_gates` has two call
sites, not one** — `plan_gates` in `installer/lib/init.sh`, which passes the
selected profile, and `installer/consumer-smoke/run-smoke.sh`'s per-kit
omitted-gate scan (its `want_omitted` loop), which calls `recipe_gates "$k"`
with no second argument at all. The second site is unaffected by this delta:
it reads the per-kit zero-config candidate set to find `.gate`-only members
with no `.sh` fallback, a query the profile does not vary today, and a missing
positional argument is simply an empty `$2` no arm reads. Build passes the
profile through this call site too when it wires the signature, for the same
discipline reason every call site of a two-argument function should agree on
its arity — but nothing in this tree depends on it doing so before the first
profile-varying arm lands. This is the seam: after it, a profile-varying
roster is a change to one arm, not a change to a signature and every caller
that reads it.

**D3 — `recipe_seed` and `recipe_config_seam_plan` stay profile-blind, and the
refusal is recorded so build does not "finish" it.** `{design-bearing}`
`recipe_config_seam_plan` derives its plan from the `templates/*-config.sh` a
kit ships; a profile cannot change which templates a kit ships without changing
the payload, so a profile argument there could never be read. `recipe_seed`'s
every seeded surface is owned by a kit the profile either vendors or does not,
so **the kit set is already a sufficient key for seeding** — verified against
each arm: the queue file (gated by `recipe_needs_queue`), the evidence
manifests, the workflow-state file, and the doctrine block all follow kit
membership alone. A parameter with no reader is removed
(canon-kit/SPEC.md §The causal-completeness check, point 4), so neither gets
one. A later profile that genuinely seeds differently changes those signatures
then, and pays only what it uses.

The asymmetry is the finding, and it is the reason this unit exists: **the kit
set is a sufficient key for what a profile *seeds*, and an insufficient key for
what a profile *registers*.**

**D4 — `installer/consumer-smoke/run-smoke.sh` asserts the lattice instead of
three literals.** `{design-bearing}`
Deleted: the `at most three profiles` cardinality bound, and the two `contains`
calls naming `starter` and `delegation` as literals. After this delta **no
profile name is a literal anywhere in the smoke** — only `PROFILE_DERIVED`,
which is a constant the module owns.

Four assertions replace them, each derived from the installed payload:

1. **Every named kit resolves in the payload.** Unchanged, kept.
2. **The order has exactly one minimum and exactly one maximum** — the lattice
   is bounded. A profile comparable to nothing, or a second incomparable
   maximum, is a red.
3. **The maximum is `PROFILE_DERIVED`.** The payload-derived profile is the top
   by construction; an authored roster naming a kit outside it would break
   assertion 1 first, and this states the shape directly rather than by
   consequence.
4. **Gate-roster monotonicity** — for every ordered pair `(a, b)` in
   `profile_order()`, `profile_gates(a)` is a subset of `profile_gates(b)`.

Assertion 4 is what D2's parameter is *for*. The adopter-facing promise is not
"a bigger profile vendors more directories" — it is "moving up only ever adds",
and what an adopter experiences is the battery, not the directory list. Kit-set
containment does not imply gate-set containment once a roster can vary by
profile, so this is the assertion that keeps the promise true.

The cardinality bound is deleted rather than raised. It existed only because the
smoke hand-enumerated the profiles it drove; a fourth profile is now admitted
exactly when it fits the bounded lattice, which is the contract the bound was
standing in for.

**D5 — `installer/profiles.list`'s header states the lattice.** `{design-bearing}`
Its `no fourth profile` clause is replaced by the bounded-lattice statement, and
the header records the split it already half-states: the **membership rows stay
hand-authored judgments** with their criterion beside them, while the **shape**
(order, bounds, monotonicity) is derived and asserted, never authored.

**D6 — `installer/README.md` §Profiles and §The consumer smoke.**
`{design-bearing}`
§Profiles gains the lattice as the contract and keeps the chain as what today's
three profiles happen to be. The adopter-facing promise is restated precisely
rather than weakened: *moving from a profile to one that contains it only ever
adds; profiles that contain neither the other are alternatives, not steps.*
§The consumer smoke's sentence "there are at most three profiles" is replaced by
the four assertions of D4.

**D7 — `docs/install.md` §Quick start is a deliberate no-op.** `{mechanical}`
Lines 254-257 describe today's three profiles as a chain. Today's three
profiles *are* a chain, so the page stays true and stays unchanged. Recorded as
a delta so a build session finds the no-op ruled rather than inventing a
rewrite; the page's prose changes when a fourth profile ships, not before.

**D8 — `.github/ISSUE_TEMPLATE/adoption-report.yml` is out of the lattice's
reach, and the ground is stated.** `{design-bearing}`
Its `profile` dropdown hardcodes the three names. A GitHub issue form is static
YAML with no execution, so it cannot derive them, and generating it would buy a
sixth generated projection plus a freshness gate for a **reporting** surface: a
stale option there mislabels an adoption report and mis-installs nothing. Ruled
out of scope with that ground rather than left as an unexplained omission.

## Producers and consumers

**`profile_order()`** — Producer: `installer/lib/common/profile.sh`, computed
per call from `profile_rows` (reading the packed `profiles.list`) and
`profile_payload_kits` (globbing the packed payload). Enabling config: none; both
inputs are present in every packed payload by construction, so the producer is
reachable on the real install path, not only under test. Consumer:
`installer/consumer-smoke/run-smoke.sh`, assertions 2-4, by direct call after it
sources the module out of the *installed* payload (its existing
invariant-against-what-an-adopter-receives discipline).

**`profile_gates(root, profile)`** — Producer: the same module, unioning
`recipe_gates(kit, profile)` over `profile_kits`. Consumers, **two, both live**:
`plan_gates` in `installer/lib/init.sh`, which stops unioning per kit inline and
calls this instead, so a fresh consumer's `gates.list` and the smoke's
monotonicity assertion are computed by one function rather than two; and
`run-smoke.sh` assertion 4. The `init` consumer is what keeps this off the
dead-everywhere-but-tests path.

**`recipe_gates`' second parameter, `profile`** — Producer: `installer/lib/init.sh`,
which holds the selected profile from its flag parse (`--profile`, else the
manifest's recorded `profile`, else `starter`) and passes it through
`plan_gates` into `profile_gates`. Named reader: `profile_gates`, at the
transition where a profile is resolved to a gate set — once per `init` run on
the real install path, and once per profile in the smoke's monotonicity pass.
**`recipe_gates`'s other call site — `run-smoke.sh`'s `want_omitted` scan —
reads no second argument and has no named reader for this field**, by D2's own
ground: the omitted-gate query does not vary by profile, so the field is
absent there rather than unread by an oversight.

**The honest limit, stated because a reader will otherwise overclaim
assertion 4.** Every `recipe_gates` arm ignores the profile today, so
`profile_gates(a)` is a subset of `profile_gates(b)` follows from
`profile_kits(a)` being a subset of `profile_kits(b)` by construction, and
assertion 4 cannot fail on this tree. Its worth is that it is **armed before the
first profile-varying arm lands**, so the iteration that ships a prose profile
meets the constraint at its own commit rather than discovering it in an adopter's
battery. An assertion that cannot fail yet is not the same as one that is
decorative — this one has a date at which it starts biting, and that date is the
next unit on this rung.

**No new field is added to any message or manifest.** `checkwright.lock`'s schema
is untouched: `profile` and `kits` already carry everything the lattice reads,
and the order is derived from the payload rather than recorded in the consumer's
tree.

## Existing sections updated

- **installer/README.md §Profiles** — D5, D6: the lattice becomes the contract,
  the chain becomes today's instance of it, and the containment promise is
  restated for comparable pairs.
- **installer/README.md §The consumer smoke** — D6: the "at most three profiles"
  sentence is replaced by D4's four assertions.
- **installer/README.md §What init seeds** — D2: its *honest limit* paragraph
  names `recipe_gates` in `lib/common/recipe.sh` as the whole of a fresh
  consumer's registry. Still true; the sentence updates to the two-argument
  form, and to `profile_gates` as the one function that unions it.
- **installer/profiles.list header** — D5.
- **docs/install.md §Quick start** — D7, a ruled no-op.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls installer/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles. Specifically: no surface still asserts a profile
      *count*, and no profile name survives as a literal in `run-smoke.sh`.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
