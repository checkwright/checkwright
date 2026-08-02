# SPEC amendment: gate-payload-disclosure-ruling

Queue entry: **`gate-payload-disclosure-ruling`**. Companion to
**`native-gate-vendoring-model`** (how a compiled gate arrives) and
**`native-gate-meta-layer-reach`** (the meta-layer's binary-side reach). This one
rules what a compiled gate **discloses** to the consumer it judges.

The iteration's envelope is **unblock, not port**. This amendment lands a ruling
and the assertion that holds it; it lands no port.

## The objectives this ruling serves

**Ruled 2026-08-03 by the operator as a project-trajectory pivot**, recorded in
full at **`native-gate-vendoring-model`** §The objectives this ruling serves —
the install and dependency model owns them, and this amendment points rather
than restates. The one that decides everything here is objective 3:

> **Opacity is a goal, not a side effect.** Withholding a gate's implementation
> source is wanted: it favours *execution* of a gate over *analysis* of it by
> the coding agents the gate exists to hold.

## The ruling

**The payload withholds the predicate.** A gate on the binary substrate reaches a
consumer as its `.gate` descriptor, its `# spec:` pointer and the SPEC section
that pointer binds to, its `good/`+`bad/` fixture pair, and a prebuilt,
digest-verified binary. **Its implementation source does not ship.**

This is the *shape-without-predicate* option the question was filed with — a
consumer receives everything needed to run a gate, act on its verdict, and
verify it behaves as specified, and does not receive the rule's text.

**What the objective is actually buying, stated precisely so it is not
oversold.** The beneficiary is the failure mode where a coding agent, told to
make a battery green, reads the gate that is blocking it and edits its way around
the predicate rather than fixing the defect. Withholding the source removes the
cheapest path to that. It does **not** make the rule secret: a binary is
reverse-engineerable, the fixture pair discloses shape, and the SPEC section
states the invariant on purpose. The claim is *raised cost of analysis relative
to execution*, never confidentiality, and no governed surface may state it as the
latter.

**The prior ruling in this file — that the payload discloses the predicate and
opacity is permanently refused — is void.** It was authored against the
constraint set the substrate seam was built under, where opacity was explicitly
not claimed and the install model handed the consumer the source anyway. Both
premises are gone. Recorded rather than deleted so the reversal is legible as a
reversal.

## What changes

### 1. §Consumer payload — a new section stating what a gate ships {design-bearing}

gate-sdk/SPEC.md gains a **§Consumer payload** section carrying the ruling, the
four shipped artifacts, and the raised-cost-not-confidentiality limit above. It
is the surface a later port reads before arguing, and its absence is what let the
question stay a range instead of a value.

Its reach is stated: it rules what a **gate** discloses. It does not reach
whether *this* repo runs built artifacts — that stays
**`native-gate-dogfood-ruling`**, which asks the same lever from the other end
and which the pivot makes sharper rather than answers.

### 2. What opacity does not extend to, and why each exclusion is load-bearing {design-bearing}

A withholding rule with no stated boundary grows until the tool is unusable. Four
things ship, each because withholding it would break something the product needs:

- **The `.gate` descriptor**, because its manifest readers must work with no
  build and no execution — `installer/lib/init.sh` runs `gen-pre-commit.sh
  --write` in the consumer tree (§The `# graph:` manifest).
- **The `# spec:` pointer and its SPEC section**, because a gate that goes red
  without an explicable invariant is an unactionable block, and an unactionable
  block is how a blocking gate turns into a bypassed one.
- **The gate's own output and help text**, because the remedy line is the
  product. A gate that says only *no* is worse than no gate.
- **The fixture pair** (delta 3).

### 3. The fixture pair is shipping-side, and is now the consumer's whole verification oracle {design-bearing}

This answers the contract question the entry names as one that must not be
discovered late: whether the `good/`+`bad/` pair is development-side or
shipping-side is unstated today.

**It is shipping-side**, and the pivot promotes it from a convenience to the
load-bearing one. It vendors already — measured, not intended:
`installer/lib/init.sh`:127-133 enumerates a kit's payload with an unfiltered
`find . -type f`, so the pair arrives whole. Under the prior ruling that was a
bonus on top of readable source. Under this one it is **the only thing a
consumer can independently check**: source is withheld, so the pair plus
`gate-sdk/bin/run-gate-tests.sh` is the entire answer to *does this binary do
what its SPEC section says*. `gate_command`'s substrate-blind dispatch
(`gate-sdk/bin/run-gate-tests.sh`:32) is what makes it work across substrates, so
no new mechanism is needed — only the statement that the property is a contract
and may not be dropped to slim a payload.

It is also the consumer-side answer to the honest limit
**`native-gate-meta-layer-reach`** records: that amendment's crate-side
verification of a gate's declared read-set runs where cargo runs, which under
**`native-gate-vendoring-model`** is never a consumer tree.

**This is the one place the ruling pulls against its own objective, and it is
escalated rather than decided.** A `good/`+`bad/` pair shows an agent what passes
and what fails, which is analysis material. The recommendation on record is to
ship it — an agent that games a gate from its fixtures can game it from its help
text and its SPEC section, both of which must ship for the tool to be usable, and
withholding it leaves a consumer with nothing verifiable at all. But that trade
is the operator's, because it spends against objective 3 directly.

### 4. The obligation opacity buys is named, and lands on an existing entry {design-bearing}

A consumer who cannot read the gate has only the publisher's word for it, so the
integrity story is the whole of what replaced reading the source.
**`native-gate-vendoring-model`** delta 6 ships the achievable floor — a
published per-target digest, verified before the artifact is written. What that
floor does *not* provide is a reproducible build, and the queue already holds
that ground as **`tarball-build-attestation`**, whose one line says the checksum
proves transfer only and the docs agree.

This amendment adds one thing to that: the pivot changes what the entry is worth.
While sources shipped, a build attestation was a supply-chain nicety. With
sources withheld it is the consumer's only remaining basis for trust, so its
priority is now a function of this ruling rather than of general hygiene.
Re-costing it is a queue decision and is escalated, not taken here.

### 5. Opacity is held by structure, and the assertion says so {design-bearing}

The mechanizable residue of this ruling is **not** the gate the prior version of
this amendment proposed. That gate — a NUL-byte scan enforcing *no artifact
ships* — enforced the old ruling, under which shipping a binary was the
violation. Under this one shipping a binary is the point, so the gate would
assert against a rule that no longer exists. It is dropped, with that reason on
record rather than carried on momentum.

What replaces it is derived from what the new ruling can actually be violated by:
**a ported gate's implementation source reaching the vendoring set.** The
vendoring set is exactly the kit roots (`gate_kit_roots_rel`), so the violation
has two shapes and both are structural and cheaply checkable.
`check-gate-substrate-parity` — already the substrate auditor, already deriving
the descriptor set, and deliberately still shell so it never depends on the
substrate it audits — gains **assertion E**:

- **No implementation sibling.** For every `<name>.gate` under a kit root, no
  file named `<name>` with a recognized implementation extension exists anywhere
  under any kit root. This catches the natural mistake: putting a ported gate's
  Rust beside its descriptor, where it would vendor.
- **The crate root is outside every kit root.** `GATE_SDK_NATIVE_CRATE` resolves
  to no path under any `gate_kit_roots` member. This is the non-vacuous half: it
  reds the moment somebody relocates the crate to give it a ride into the
  payload, which is the one edit that would silently un-do the whole ruling.

Folding into the existing gate rather than shipping a new one is deliberate: the
assertion's corpus is the descriptor set that gate already derives, and a
separate gate would duplicate that derivation to add nothing.

### 6. The install and site surfaces state what a gate discloses {design-bearing}

`docs/install.md` states what an install writes and what it requires, and says
nothing about what a gate discloses. It gains a short statement naming the ruling
and pointing at §Consumer payload — the widest-true tier for a reader deciding
whether to adopt a tool whose rules they will not be able to read.

Two constraints on that prose, both from delta 1's limit: it states *raised cost
of analysis*, never confidentiality or secrecy; and it states *verified against a
published digest*, never *reproducible*. canon-kit's `check-install-claim` holds
the primary-install-path claim across governed install sections and is the reader
that must stay green over the edit.

## Producers and consumers

**The ruling itself** introduces no runtime state; its producer is the SPEC
section and its consumers are the sessions and the assertion below.

**Assertion E's inputs.**
Producer: the tracked tree — the `.gate` descriptor set under
`gate_kit_roots_rel` members, which `check-gate-substrate-parity` already
enumerates, plus the resolved `GATE_SDK_NATIVE_CRATE` path (the knob
**`native-gate-vendoring-model`** introduces; this assertion is its second
reader, which is what keeps the crate root from acquiring a second spelling
here). Consumer: the gate itself, at its existing run. Its `couples=` gains
nothing — the kit roots and the crate root are already in its trigger set — and
that is worth verifying at build rather than assuming, because an assertion that
never re-fires is an assertion that never runs.

**The fixture pair as a shipping artifact.**
Producer: the kit's `gate-tests/<name>/{good,bad}/` tree, written when the gate
is authored and required by `check-gate-fixture-coverage`. Consumer, in the
**consumer** tree rather than this one: `gate-sdk/bin/run-gate-tests.sh`,
resolving the member through `gate_command` and therefore running the consumer's
own installed binary against the shipped cases. That consumer is new — the pair
has always vendored, and this amendment names the reader that makes the vendoring
a contract instead of an accident.

**No new fields.** The descriptor gains none: it "carries no field that lacks a
reader, reserving nothing against a future reader" (§The `# graph:` manifest),
and a disclosure ruling that added a descriptor field would be declaring in data
what the payload's composition already states.

## Existing sections updated

Owned by this amendment, each named with the delta that claims it:

- gate-sdk/SPEC.md §Consumer payload — new; the ruling's home (delta 1), its
  boundary (delta 2), and the integrity obligation it points at (delta 4).
- gate-sdk/SPEC.md §Fixture-pair discipline — the shipping-side statement and its
  consumer-tree reader (delta 3).
- gate-sdk/SPEC.md §check-gate-substrate-parity — assertion E (delta 5).
- gate-sdk/SPEC.md §Porting a gate to the binary substrate — the port's
  justification, where opacity moves from a ground not claimed to a ground ruled
  (delta 1).
- gate-sdk/SPEC.md §What the dispatch seam does not settle — the "Opacity is not
  claimed" paragraph becomes a pointer to the ruling rather than a statement of
  the present condition (delta 1).
- `docs/install.md` — the disclosure statement and its two prose constraints
  (delta 6).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **No surface claims secrecy.** Every governed statement of this ruling says
      raised cost of analysis, and none says confidential, secret, or
      reproducible.
