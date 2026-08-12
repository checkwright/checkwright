# SPEC amendment: measured-claim

A new canon-kit gate, `check-measured-claim`, ends the class where **a measured
count or extent claim authored into governed prose goes stale with no oracle**.
Thirteen instances are recorded on the queue entry across five iterations; this
amendment designs the mechanism, not the history, and does not restate it.

## The trade the entry hands this unit, and the ruling

The entry states the fork precisely: *"Both instance sets argue for an opt-in
`measured:`-style marker the author applies over a scanner inferring intent — a
smaller gate bought with a larger authoring contract, and that trade is the
unit's to rule."*

**Ruled: the marker.** Three grounds, in order of strength.

1. **A scanner cannot reach the half that costs.** The entry's own accounting is
   decisive: eleven of thirteen instances were caught downstream by hand at no
   cost beyond re-measurement, and the twelfth — an *extent* claim, carrying no
   cardinal at all — *"was caught by nobody for two iterations and shipped a false
   sentence into a published SPEC and its docs mirror."* An inferring scanner
   triggers on a numeral. The claims that cost the most have no numeral. Buying
   the scanner buys the free half.
2. **The tractable slice of the cardinal axis is already gated, and a second
   inferring scanner would duplicate it.** `check-manifest-count` bans a bare
   cardinal quantifying a governed collection noun. Its known limit is stated on
   the entry — *"banning cardinals cannot reach a claim carrying none"* — but
   within its own slice it works. A second scanner inferring intent from prose
   would overlap it where it already holds and over-match everywhere else ("the
   four contracts", "both halves").
3. **A marker has no false-positive surface by construction**, which matters
   because the entry names that surface as *the* design problem, and because the
   sibling entry stuck on the same surface — `qualified-pointer-section-ownership`
   — is deliberately **not** folded in here and stays deferred. That entry needs
   comprehension (does this section *support* this sentence). This one does not,
   once the author declares what was measured.

**But a bare declaration is not enough, and this is where the design goes past the
entry.** A marker that merely says "this number was measured" records an intent no
machine can check, and this tree already has three markers shipped with **zero
live uses** (`manifest-count-exempt:`, `spec-embedded-source-exempt:`,
`scratch-citation-exempt:` — the last already filed as its own defect). A fourth
would be dead mechanism on arrival. So the marker names an **oracle**, and the
gate re-runs it. That converts a transcribed literal into a generated,
freshness-gated copy, which is what the derivation-first rule asks for: *derive
the derivable, never maintain it; a needed copy is generated and freshness-gated.*

## What changes

**(1) The marker: a full-line HTML comment binding a claim to an oracle key and
its measured value.** [design-bearing]

```
<!-- measured: <key>=<value> -->
```

placed on the line **immediately above** the claim it binds, matching the
line-or-line-above placement convention every canon-kit marker already uses.

**It joins an existing family of two rather than inventing a form**, and the
family is exact: `<!-- install-primary: <transport-id> -->` (§check-install-claim)
and `<!-- payload-discloses: <claim-id> -->` (§check-payload-claim). Both are
full-line HTML comments binding a prose claim to a machine-readable id, and both
pair that id with a **consumer-owned vocabulary** loaded through a `*_CMD` knob —
which is the same pairing delta (2) reaches for, so this marker and its oracle are
one shape the kit already ships twice, not two new ones. The stated reason
transfers unchanged: the marker is *"a marker rather than a visible sentence,
because the reader-facing form of this claim already exists as prose and must stay
prose; the marker is the tier beside it, not a replacement."*

This is deliberately **not** the `-exempt:` family, which is the other marker
vocabulary in the tree: those are suppression valves that make a gate look away,
and this is an *attachment* that gives a gate something to check. Delta (5) is
where that distinction pays.

**(2) The oracle: a consumer-owned emitter behind a `*_CMD` knob.**
[design-bearing] `CANON_KIT_MEASURED_CLAIMS_CMD` names a command emitting one
`<key>\t<value>` line per measurable fact. This is the third instance of a shape
canon-kit already ships twice over — `CANON_KIT_ENUM_SETS_CMD`,
`CANON_KIT_INSTALL_TRANSPORTS_CMD` and `CANON_KIT_PAYLOAD_CLAIMS_CMD` — and it is
what holds the provenance seam: **the marker grammar and the comparison are kit
mechanism; every key, every oracle command and every measured fact is consumer
config.** A kit literal enumerating what a project measures would publish that
project's vocabulary, which is the seam's whole point. Unset knob means the gate
has no oracle and reports clean, the same inactive-by-default posture the sibling
`*_CMD` gates take.

**(3) Three arms, and the third is what keeps the marker honest against its own
prose.** [design-bearing]

- **A — the oracle disagrees.** Red when the emitter's current value for `<key>`
  differs from the marker's `<value>`. This is the whole point: the number in the
  document is now checked against the tree.
- **B — the key is unknown.** Fail closed, exit 2, when `<key>` is absent from the
  emitter's roster. A marker naming a key nobody emits is a claim with no oracle
  wearing the costume of one, which is worse than an unmarked claim.
- **C — the marker drifted off its own sentence.** When `<value>` is a bare
  cardinal, red unless that cardinal appears as a token in the bound claim's
  paragraph. Without arm C the marker and the prose can disagree while the marker
  and the oracle agree — the gate would go green over a false sentence, which is
  the exact failure being closed.

Arm C is also where the *authoring contract* the entry priced shows up: a bound
claim carrying **more than one** distinct cardinal is ambiguous, and the gate
fail-closes on it (exit 2) rather than guessing which one it holds. The remedy is
in the help line — split the sentence, or move the marker to the clause that
carries the measurement. That is the "larger authoring contract" bought here, and
it is bounded: it applies only to sentences an author chose to mark.

**(4) Extent claims are covered by arms A and B alone, and that is the design, not
a gap.** [design-bearing] An extent claim carries no cardinal, so arm C does not
apply and arm A does the work — with `<value>` being whatever the author declares
the extent to be and the emitter recomputes: a corpus size, a sorted membership
list, a digest over the swept set. The audit that stamped *"the kit SPECs came
back clean"* over a SPEC holding two counter-instances would have carried a key
whose oracle emits the counter-instance count, and would have reddened when it
moved off zero. This is the axis no scanner reaches, and it costs nothing extra:
the same three arms, with a set-valued rather than integer-valued oracle.

The session-act half stays out of scope, deliberately and by the entry's own
words: `audit-class-corpus-attestation` designs a **stamp** obliging a sweep to
record the corpus it read. This designs a check over **authored prose**. A claim
nobody marks is uncaught here, and delta (5) is the only place that gap is
narrowed.

**(5) The marker becomes `check-manifest-count`'s sanctioned discharge, which is
what gives it live users on day one.** [design-bearing] Today that gate's ban is
discharged by rewording, by an allow-list phrase, or by a
`manifest-count-exempt:` tag with **zero** live uses in the tree. A fourth route
is added and it is the good one: a cardinal carrying a `measured:` marker
satisfies the ban, because the objection the ban encodes — a transcribed total
with no owner — is answered by an oracle rather than suppressed. This wires the
two gates into one story (ban the unowned cardinal, offer the oracle), and it
converts the exempt tag's role from "deliberately unchecked" to a genuine last
resort. It is also the only arm of this design that makes marking *pressured*
rather than voluntary, so it is what keeps the coverage limit in delta (4) from
widening over time.

**(6) The corpus is its own glob surface, not the manifest set.** [design-bearing]
`CANON_KIT_MEASURED_SURFACE_GLOBS` (array, default empty) is the scanned surface.
The entry requires it: one of the thirteen landed in `.claude/commands/close.md`,
a **binding shim**, and *"the scanner must range over SPEC sections and binding
shims alike"*.

**Neither existing surface reaches a shim, and the two exclude it for different
reasons — one of which is not written down.** Stated precisely, because the
distinction is the whole argument for a third knob:

- `CANON_KIT_MANIFEST_FILES` (`scripts/canon-config.sh:22-38`) excludes shims **by
  omission**. The array simply does not list `.claude/commands/`, and its comment
  says nothing about them. No documented rationale exists — this amendment does not
  manufacture one, and a reader should not infer a deliberate ruling from a silent
  absence. What would create one is a sentence on that array's own comment; whether
  it is worth writing is that knob's question, not this gate's.
- `CANON_KIT_PROSE_SURFACE_GLOBS` (`scripts/canon-config.sh:75`) excludes them **by
  a documented decision**, and the reason is the load-bearing one: *"the stage-skill
  shims under `.claude/commands/` stay out (they are consumer bindings, governed by
  check-skill-binding/check-shim-restatement)"*. That is an argument about which
  gate **owns** a shim, not a claim that a shim carries no governed prose.

The second is what licenses a third surface rather than a widening of either. A
shim is excluded from the prose surface because other gates govern its *copy
shape* — and `check-shim-restatement` holds copy shape, which the queue entry
already records is exactly what a **wrong** restatement escapes, since a
restatement that is wrong has diverged from its owner's wording and is therefore
not a copy. So the ownership that keeps shims out of the prose surface is
ownership this gate's rule is not covered by. Reusing either existing surface
would under-scan by exactly the instance that motivated widening the class. This
repo sets the new knob to the manifest globs plus `.claude/commands/*.md`.

**(7) The gate is born native, and this is the first one. Operator-ruled
2026-08-12 at spec, with the criterion-5 cost below in view and accepted.**
[design-bearing]
TRAJECTORY.md §The objectives' sixth shrinks the interpreter surface to the
unavoidable, and `native-gate-port-remaining-corpus` records that *"every gate
landed meanwhile adds shell to the eventual port"* — so a new shell gate is debt
created knowingly. Landing it as a Rust module plus a `.gate` descriptor avoids
that, and three things make it cheap rather than ambitious:

- **It needs no unbuilt substrate.** Delta (6)'s glob corpus is served by
  `native/src/walk.rs`'s `glob_files`, already in production. Consequently this
  gate has **no dependency on the `spec_manifest_files` cohort porting**, which is
  the reason the corpus was designed as its own surface rather than a union with
  the manifest set.
- **Criterion 2 does not bind.** The port criteria govern *ports*; a gate with one
  implementation has no second substrate to prove parity against. Its oracle is
  its `good/`+`bad/` fixture pair, exactly like any new shell gate's.
- **Criterion 4 is clear.** Its corpus is governed prose, and its `couples=` names
  no registry member's declaration path, so it is outside assertion C's derivation
  and takes no conservation-table row.

**The honest cost, accepted rather than merely disclosed:** by criterion 5 a
`.gate`-declared member is omitted from the `gates.list` of a consumer whose host
the release publishes no artifact for, so on an uncovered platform this gate does
not run where a shell gate would have. That is the port's standing cost, already
accepted for ten members whose shell forms were deleted; it adds no new class of
cost, and it is why the omit-and-declare path exists. **The operator ruled with
this cost stated, so it is a paid price and not an open risk** — a later reader
must not re-open the ruling on it. What stays open is only its reach: this settles
*this* gate, and a reader weighing a second born-native gate weighs the cost again
rather than citing this one as blanket precedent.

**(8) Registration and the new-gate fan-out.** [mechanical]
`check-measured-claim` joins `scripts/gates.list`, canon-kit's README gate-roster
block, and a `### check-measured-claim` section in canon-kit/SPEC.md, with a
`good/`+`bad/` pair at `canon-kit/gate-tests/check-measured-claim/`. Its
`# graph:` manifest is `dir=one valve=none tier=precommit` — the invariant is
restorable inside the commit that perturbs it — coupling the measured surface
globs and the emitter script. The regen set and its ordering constraint are
docs/site-architecture.md §Generated projections' to state and this delta's to
execute.

**(9) The consumer emitter.** [design-bearing] `scripts/measured-claims.sh`,
mirroring `scripts/enum-sets.sh`, emits this repo's keys. It is authored with the
first real markers rather than empty, since an emitter with no keys and a gate
with no markers is two dead mechanisms attesting to each other.

## Producers and consumers

**New interface: the `measured:` marker.**
- *Producer* — an author, at the moment of writing a measured claim; and delta
  (5)'s pressure, which makes it the sanctioned way to keep a cardinal that
  `check-manifest-count` would otherwise ban.
- *Consumer* — `check-measured-claim`, which parses it out of the scanned surface
  and resolves `<key>` against the emitter's roster.
- *Every field has a named reader* — `<key>` is read by arm B against the emitter
  roster; `<value>` is read by arm A against the emitter's value and by arm C
  against the bound paragraph. There is no third field, and neither of the two is
  read at only one arm.

**New knob: `CANON_KIT_MEASURED_CLAIMS_CMD` (scalar, default empty).**
- *Producer* — the consumer's `canon-config.sh`. **Enabling config actually
  emitted:** this repo sets it to `bash scripts/measured-claims.sh` in delta (9),
  so the producer is live here and not test-only — the point
  §The causal-completeness check makes first.
- *Consumer* — the gate, once per invocation, through `spec_claim_vocabulary`'s
  existing command-running adapter (`canon-kit/lib/spec.sh:462-478`) or its
  binary-side equivalent; unset means no oracle and a clean, inactive gate.

**New knob: `CANON_KIT_MEASURED_SURFACE_GLOBS` (array, default empty).**
- *Producer* — the consumer's `canon-config.sh`; this repo's value is the manifest
  globs plus `.claude/commands/*.md`.
- *Consumer* — the gate's corpus derivation, crossing the config bridge as a
  bridged array and read by `walk::glob_files` on the binary side. Empty means an
  empty corpus and a clean gate, which is the same inactive-by-default posture as
  the knob above; the two are set together or not at all.

**New interface: the `key\tvalue` emitter protocol.**
- *Producer* — the consumer's emitter script.
- *Consumer* — the gate, at knob-resolution time.
- *Constraint with a named reader*: the config bridge refuses an element
  containing a tab or a newline (`gate-sdk/lib/gate.sh:126-138`). Since tab is this
  protocol's own field separator, the emitter's **values** may not contain one —
  a real bound on a set-valued extent oracle, which therefore joins its members
  with something else. Checked at build against the emitter, not assumed.

**Red conditions of the readers this change touches.** This delta **widens** every
corpus it touches (a new gate, a new SPEC section, a new emitter, new markers) and
narrows none, so no reader here is exposed to the non-monotone case
§The causal-completeness check point 5 binds on. Named anyway, because "it only
adds" is the first argument this delta reaches for:

- `check-manifest-count` — red on a bare cardinal over a governed collection.
  Delta (5) **adds a discharge**, which strictly shrinks its violation set; its
  own fixture pair and `check-manifest-count.test.sh` must gain the new route or
  the discharge is untested.
- `check-gate-output` — red on a **zero count** of a `: clean` / `help:` emission.
  A born-native member's declaration path is a descriptor, which by the closed
  field roster cannot hold those strings, so the resolution follows the rule to
  the implementation module — the branch §check-gate-output already owns for a
  `.gate`-dispatched member. This is the one reader where born-native changes the
  answer, and it is why delta (7) is design-bearing.
- `check-gate-fixture-coverage` — red on a member with neither a pair nor an
  opt-out: a zero-count reader, cleared by delta (8)'s pair.
- `check-gate-substrate-parity` — assertion B reds on a descriptor with no
  subcommand and on a subcommand with no descriptor, so the module and the
  descriptor land together. Assertion C is not reached: delta (7) records why this
  member is outside the substrate-sensitive derivation, and a conservation-table
  row must **not** be added for it.
- `check-reads-couples` — red on a walk outside the declared couples; the member
  declares its glob-surface root in its registry tuple and answers `--reads`.
- `check-readme-roster` — red in both directions; canon-kit's README gains the
  name in the same commit.
- `check-knob-citation` — red on a kit knob stated with its value outside the
  owning kit's SPEC. Both new knobs' values belong in canon-kit/SPEC.md
  §Layout and configuration and nowhere else.
- `check-docs-cmd` — red on a governed doc fencing a path that does not resolve;
  `scripts/measured-claims.sh` must exist before any doc fences it.
- `check-graph`, `check-enforcement-fresh`, `check-footprint-fresh`,
  `check-value-rollup-fresh`, `check-docs-mirror-fresh` — each red on a stale
  projection, which a new registered gate makes stale by construction.
- `check-gate-binary-fresh` — a born-native member makes the binary load-bearing
  exactly as a ported one does; already armed, predicate unchanged.
- **`check-measured-claim` over its own SPEC section** — the section will describe
  the marker and will therefore contain marker-shaped text. canon-kit's siblings
  solve this with a self-referential exemption (`prose-enum-exempt:` sits in
  canon-kit/SPEC.md for exactly this reason); the same treatment applies here, and
  naming it now stops it being discovered as a red at landing time.

## Existing sections updated

- **canon-kit/SPEC.md, new `### check-measured-claim`** — the invariant, the three
  arms, the marker grammar, the ambiguity fail-close, the extent-claim coverage
  and its limit, and delta (7)'s born-native ruling with its honest cost.
- **canon-kit/SPEC.md §check-manifest-count** — owned by delta (5): the discharge
  roster gains the marker, and the section's own statement of what its ban cannot
  reach now points at the gate that does.
- **canon-kit/SPEC.md §Content tiering — the star topology** — owned by deltas (1)
  and (2). Its *"Quantitative literals are code-owned"* bullet currently offers one
  remedy, citing the owning source. A marked-and-oracled literal is a second, and
  it is the case the bullet's own "a literal stays verbatim only when load-bearing,
  and then only gate-coupled" clause describes without yet having a mechanism for.
- **canon-kit/SPEC.md §Layout and configuration** — owned by deltas (2) and (6):
  the two new knobs, their shapes and their defaults.
- **canon-kit/SPEC.md §lib/spec.sh** — owned by delta (2) if the emitter is read
  through `spec_claim_vocabulary`: that adapter gains a third caller, and the
  section names its callers.
- **canon-kit/README.md** — the gate-roster block gains the name.
- **gate-sdk/SPEC.md §The port-candidate criteria** — owned by delta (7): the
  criteria are stated over ports, and a born-native gate is the first member they
  do not describe. One paragraph fixing which criteria bind on a gate with no
  shell form (5 does, 2 and 4 do not) keeps a later reader from applying the parity
  criterion to a member that has no second substrate.
- **scripts/canon-config.sh, scripts/gates.list, scripts/measured-claims.sh** —
  owned by deltas (2), (6), (8) and (9).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls canon-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
