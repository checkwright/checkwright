# SPEC amendment: born-native

**New gates are born native by default; shell is the exception, and it needs a
stated cause.** Operator-ruled 2026-08-14 and re-affirmed the same day on
corrected criterion-5 evidence (TRAJECTORY.md §The closed rulings). This
amendment lands the flip at gate-sdk/SPEC.md §The port-candidate criteria, where
a born-native gate is today *"a design ruling rather than a default"*.

**The amendment's other half is the exception criterion, and it is inside this
unit rather than an add-on.** *"Shell by exception with cause"* is unimplementable
without a criterion that says which causes count: with none, every gate can argue
itself out and the flip is a preference. Authoring it is delta 3.

**What the ruling acts on, which is why it is worth a unit at all.** It is the
only measure taken so far against the port's *denominator* rather than its
remainder: under the prior default every gate landing while the port runs added
shell the port then owed, at a delivery rate of two to three ports per iteration
against 62 remaining. The cost it takes is criterion 5's, in full: a
`.gate`-declared member is omitted from the `gates.list` of a consumer whose host
`native/targets.list` carries no artifact for, that roster ships one target, and
the uncovered set is therefore every macOS adopter today. The ruling was
re-affirmed against that corrected figure. The residue — the *rate* at which the
omitted set now grows — is costed and filed as
`born-native-omission-accumulation` and is not closed here.

## What changes

### 1. The default flips, and the observation behind the old default survives it

§The port-candidate criteria's born-native passage keeps criterion 2's reasoning
and loses the conclusion drawn from it: a born-native gate's `good/`+`bad/` pair
really is its whole oracle, and that really is weaker than a ported member's
parity run — and it is now the accepted price rather than the reason to ship
shell. **[design-bearing]**

The trade reversed because its other side changed, and the merge should say so
rather than presenting the new default as self-evident: under the 2026-08-09
directive the corpus ports in full, so shipping a new gate in shell no longer
avoids the compiled implementation — it defers it, adds a parity run that would
not otherwise be owed, and grows the denominator the directive is racing. The
weaker oracle is paid once; the deferred port is paid again at every cohort cut
that has to sequence it.

### 2. The default's domain is a tree that carries the crate

The default binds gate authoring **where the authoring tree carries the crate the
gate would compile into** (`GATE_SDK_NATIVE_CRATE`). Where it does not — every
vendoring consumer — shell is not an exception but the only substrate, and the
consumer-facing authoring path is unchanged. **[design-bearing]**

This is not a narrowing of the ruling; it is the only reading a consumer can
execute. A consumer receives no gate implementation source (TRAJECTORY.md §The
objectives, and §Consumer payload's opacity ruling), and `native/` is not a kit —
it ships no `checks/` and no `smoke/`, so `gate_kit_roots` never selects it and
`init` never vendors it. An adopter therefore cannot author a compiled gate at
all, and a spec sentence telling them to would be false on its face.

The domain decides which surfaces the flip reaches, which is the practical work
this delta does: **publisher-facing surfaces change** (the always-loaded
authoring line, delta 5) and **consumer-facing ones do not** (delta 6).

### 3. The exception criterion: three closed classes, each with a stated cause

Shell is taken only under one of three classes, and the gate's own SPEC section
states which class and why. A fourth class is an amendment, not a judgment call.
**[design-bearing]**

- **(a) The gate audits the dispatch relation.** A gate whose assertion is about
  whether a gate declares itself, or whether a descriptor and a subcommand agree,
  stays shell — a compiled form could pass *itself* with a broken binary, which
  is a false green. This class is **not new**: §Meta-gate conservation for the
  binary substrate already rules it (*"a gate whose assertion is about the
  dispatch mechanism stays shell; a gate that merely reads declaration paths as
  content may port"*) and `check-gate-substrate-parity` and
  `check-install-disposition` are its live members, each held shell in that
  section's own rows on exactly this ground. `check-crate-arms` is **not** a
  member of this class, checked rather than assumed: that same section states it
  sits *outside* the conservation table's derivation entirely (its `couples=`
  covers no registry member's declaration path), and its permanent-shell ground
  is instead criteria 4 and 7 — a gate running `cargo test` over the crate cannot
  live inside the artifact it tests, and its rule invokes `cargo` — recorded at
  §The port-candidate criteria's worked-example passage as *"a different case"*
  from the dispatch-relation one. Two permanently-shell gates, two distinct
  causes; naming both here rather than folding the second into class (a) is what
  keeps the class's own ground exact. The exception criterion adopts the
  existing dispatch-relation ruling rather than inventing a parallel one.
  **Permanent**; the cause is the class.
- **(b) The gate's subject is a platform the target roster does not cover.**
  Criterion 5 omits a `.gate` member on exactly the platforms `native/targets.list`
  carries no artifact for. A gate whose findings arise *on* those platforms would
  therefore be omitted precisely where it is the only reader — the one case where
  born-native does not merely weaken the oracle but deletes it at the point of
  use. **Ends when the target lands**, so the cause names the target, and the
  gate ports under the ordinary criteria once `platform-support-ci-matrix` widens
  the roster.
- **(c) The rule needs substrate the crate does not carry, and building it is not
  this unit's work.** Criteria 6 and 7 bind on a born-native gate unchanged, and
  the existing text says such a gate is *"designed to clear them at authoring
  time … or it is not born native"* — which under a default reads as a licence
  unless it is bounded. The bound: the cause **names the missing substrate and
  the entry that owns it**, and the gate lands shell inside the port corpus with
  its blocker already declared. That is strictly better than the pre-flip state,
  where the same gate landed shell and its blocker had to be re-derived by
  `port-blockers.sh` at some later cohort cut. **Temporary by construction** — it
  expires when the substrate lands.

**The cause is recorded in the gate's own SPEC section**, beside the rule it
governs, on the same terms every other design ruling is recorded — not in a
central roster, which would be a maintained list of the residue that derivation-
first refuses and that rots at every port. A shell gate landing with no such
sentence is the defect the rule names, and delta 4 is honest about what does and
does not catch it.

**What the classes deliberately exclude**, because an exception criterion is
defined by its refusals: *the rule is easier to write in shell*, *the author is
faster in bash*, *the corpus is small*, and *it is only a temporary gate* are
none of them causes. Nor is *"criterion 5 makes it a real subtraction"* on its
own — that is true of **every** born-native gate, so admitting it as a cause
would swallow the rule the same day it landed. Class (b) is the sharpened form of
that argument and is the only form of it that survives.

### 4. The rule is procedural, and the ground for not gating it is stated rather than left as a gap

No gate enforces the flip, and the reason is that **no discriminator exists while
the port runs**: a newly authored shell gate is indistinguishable from one of the
60 members still awaiting their port, so any assertion over "shell gates" reddens
the whole residue. **[design-bearing]**

Two enforceable shapes were weighed and both are refused *for now*, with what
would change that named:

- **A per-gate `# substrate: shell — <cause>` header, asserted over every shell
  member.** Buildable today and it is the shape that would work — but it demands
  ~60 retrospective declarations, which is a costed sweep unit of its own, and
  every one of them is deleted again as its member ports. It becomes cheap when
  the residue is small, which is the condition to revisit it under.
- **A baseline roster of today's shell members that a gate diffs against.** A
  maintained roster, which derivation-first refuses, and it rots at every cohort
  cut.

So the rule is held by this spec section and by the always-loaded authoring line
(delta 5), and the enforcement gap is **filed rather than flagged-and-skipped**,
per the gap-disposition rule. Enforcement-first is not being waived: it ranks a
gate above discipline where a gate is available, and here the gate is available
only at a price the flip does not justify paying today.

### 5. The always-loaded authoring line changes

CLAUDE.md's gate-authoring line — *"New gates copy
`gate-sdk/templates/check-skeleton.sh` and ship with a `good/`+`bad/` fixture
pair"* — becomes the flipped default with its exception pointer, in one line, per
the always-loaded shape rule. **[mechanical]**

This is the delta that makes the unit cross-component: the ruling's contract
lands in gate-sdk/SPEC.md and the always-loaded surface at the repo root carries
the instruction a session actually reads at authoring time. A flip recorded only
in the SPEC would be a rule nobody loads.

### 6. The consumer-facing authoring path is deliberately unchanged, and that is recorded

`gate-sdk/templates/check-skeleton.sh`, gate-sdk/README.md's *"Write your first
gate by copying …"*, and the kit-landing checklist's fixture-pair demand stay
exactly as they are. **[mechanical]**

Recorded as a delta rather than left as an omission because a merge sweep
propagating the flip across every surface that mentions gate authoring is the
predictable error, and its result would be a kit telling an adopter to write a
Rust gate they can neither compile nor ship. Delta 2 is the ground.

### 7. What a born-native gate owes is unchanged, and the flip is not a discount

The criteria that bind on a born-native member — 1 and 3 unchanged, 2 not binding
for want of a second substrate, 4 by its own predicate, 5 hardest, 6 and 7
unchanged — are already enumerated at §The port-candidate criteria and this
amendment does not restate or relax them. **[mechanical]**

Two of them are worth naming as *more* load-bearing under a default, not less:
the `good/`+`bad/` pair is now the gate's entire oracle in the ordinary case
rather than the exceptional one, and the `--reads` declaration plus assertion C's
conservation row are owed at authoring time rather than at a port. A born-native
gate that ships without them ships less coverage than the shell gate it replaced,
which is the outcome the flip must not produce.

### 8. The residue is owned elsewhere and is not closed here

`born-native-omission-accumulation` carries the accumulation *rate* the flip
creates and names three candidate instruments, one of which is *"nothing beyond
the exception rule the flip's own amendment must state"*. **[mechanical]**

That entry is judged against the merged spec at its own scope, and this amendment
does not judge it: the exception criterion is deliberately about *which gates take
shell*, never about *how far the omitted set may grow*, and reading delta 3 as
closing the entry would retire a costed residue by assertion.

## Producers and consumers

**This amendment introduces one new interface — the exception class and its
stated cause — and no new state, event, message, field or configuration knob.**
`GATE_SDK_NATIVE_CRATE` is cited by delta 2 as the predicate of the default's
domain and is not modified.

- **The exception class and cause** (delta 3). **Producer:** the session
  authoring a new gate, writing one sentence into that gate's own SPEC section —
  reachable because that section is written in the same unit as the gate, which
  the kit-landing checklist already requires. **Consumers**, all three named
  because a cause with no reader would be ceremony:
  1. the **reviewing session** reading the gate's SPEC section, which is where
     every other design ruling for that gate already lives;
  2. the **port-track selector**, for which a class-(c) cause is a named blocker
     it would otherwise re-derive from `bash gate-sdk/bin/port-blockers.sh` — the
     cause states the missing substrate and the owning entry, which the derived
     roster cannot;
  3. the **session that lands the missing substrate or the missing target**, for
     which a class-(b) or class-(c) cause is the list of gates that become
     portable with it.
- **The flipped default** (deltas 1, 5). **Producer:** gate-sdk/SPEC.md §The
  port-candidate criteria, the contract surface; and CLAUDE.md's authoring line,
  the always-loaded surface a session actually loads. **Consumer:** every future
  gate-authoring session in a crate-carrying tree. The two surfaces are one tier
  apart by design — the contract behind the pointer, one line resident.
- **No consumer exists in a vendoring tree** (delta 2), and that is a positive
  finding rather than a missing one: the default's domain excludes them, so a
  reader in a vendored kit correctly finds the shell path unchanged.

**The growth the flip produces has a named reader, and it is a measurement rather
than a person.** Every born-native gate adds a member to the set an
artifact-free install omits, which `installer_smoke`'s binary-less leg derives
and asserts complete at a non-zero count (§The port-candidate criteria,
criterion 5). That leg is therefore the instrument the accumulation is visible
in from the day the flip lands, and it is what
`born-native-omission-accumulation` will reason over. Naming it here is what
keeps the flip from being a change with no observer.

**This delta narrows no corpus.** It changes which substrate a *future* gate is
authored on and deletes no file, no glob and no member, so causal-completeness
point 5's red-condition enumeration has no subject in this unit. Stated
explicitly rather than omitted, because the point applies to the sibling
amendment in this iteration and its absence here should read as a verdict.

## Existing sections updated

- **gate-sdk/SPEC.md §The port-candidate criteria** — owned by deltas 1, 3 and 7.
  Its born-native passage keeps its five-bullet criterion reading intact and
  gains the flipped default, the domain predicate and the three exception
  classes; criterion 2's bullet loses only its closing clause. The exception
  criterion lands **here**, beside the criteria it is stated in terms of, rather
  than in a new section.
- **gate-sdk/SPEC.md §When a gate earns its place** — owned by delta 1. It rules
  *whether* a gate is built and is one pointer sentence away from *what substrate
  it is built on*; it gains that pointer and not a second copy of the rule.
- **CLAUDE.md §This repo is governed by its own kits** — owned by delta 5. The
  gate-authoring line changes; the fixture-pair half of it is untouched, and
  under delta 7 it matters more.
- **TRAJECTORY.md §The closed rulings** — owned by delta 3. The born-native entry
  states the mechanism *"is owed to that same section by the amendment this
  ruling opens"*; on merge it is no longer owed, and the sentence becomes a
  pointer to the delivered criterion. The ruling text itself, including the
  re-affirmation, is not rewritten.
- **`docs/` mirror** — owned by deltas 1, 3 and 5, mechanically: the generated
  projection of gate-sdk/SPEC.md and CLAUDE.md, both changed above, regenerates
  with the merge, per its rostered regen command.
- **Deliberately not updated:** `gate-sdk/templates/check-skeleton.sh`,
  gate-sdk/README.md's first-gate walkthrough, and the kit-landing checklist
  (delta 6).

## Definition of Done

- [ ] **Causal completeness** — the exception cause has a named producer (the
      authoring session, in the gate's own SPEC section) and three named
      consumers; the flipped default has both its contract surface and its
      always-loaded surface; the omitted-set growth has its named instrument.
      No corpus is narrowed, stated as a verdict.
- [ ] **The exception criterion is closed and non-swallowing** — three classes,
      each with a cause form; the refused causes enumerated, including the
      criterion-5 argument that would swallow the rule.
- [ ] **The domain is stated wherever the default is** — no surface asserts the
      native default without the crate-carrying predicate, and no consumer-facing
      authoring path is flipped.
- [ ] **The enforcement gap is filed, not flagged** — the two refused gate shapes
      recorded with the condition that would revisit them, and an inbox entry
      filed for the disposition.
- [ ] **Merged with no information lost** — criterion 2's observation survives the
      flip; the merged §The port-candidate criteria reads as one document to a
      reader who never saw this amendment.
- [ ] **Amendment deleted** — this file removed on merge; none remain for
      gate-sdk (`ls gate-sdk/SPEC-*.md`), discharged at the iteration rather than
      at the commit while `SPEC-json-cohort.md` is in flight.
- [ ] **Removals propagated** — grepped every spec and the always-loaded surface
      for *"a design ruling rather than a default"* and for authoring lines that
      assume shell; nothing contradicts the flip and nothing tells a consumer to
      write Rust.
- [ ] **The residue is not closed by this unit** —
      `born-native-omission-accumulation` stands, judged at its own scope.
- [ ] **Gaps filed** — cross-component gaps found during the work filed as debt
      tasks.
