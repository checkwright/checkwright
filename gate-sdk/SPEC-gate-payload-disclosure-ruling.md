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

That reverses the direction the substrate seam was argued under, where opacity
was explicitly not claimed, and it voids the ruling below.

## The ruling

> **Void, being re-authored.** This ruling was authored before the objective
> above was visible. It is retained only until the re-authoring lands in this
> same file.

**The payload discloses the predicate. Opacity is refused as a goal, and the
refusal is permanent rather than provisional.**

A gate on the binary substrate reaches a consumer as four tracked text
artifacts — its `.gate` descriptor, its Rust implementation, its `good/`+`bad/`
fixture pair, and the SPEC section its `# spec:` pointer binds to — plus a binary
that consumer built from the source it received. No compiled artifact is shipped.

Three grounds, in the order that decides:

1. **The install model already settles it.** Under
   **`native-gate-vendoring-model`** a consumer that runs a compiled gate builds
   it from vendored crate source. There is no state of the world in which a
   consumer has the binary and not the source, so a disclosure ruling that
   withheld the predicate would be a rule with no mechanism behind it. This is
   the coupling that made the two entries inseparable, resolved in the direction
   the install model forces rather than the direction the port was argued on.
2. **A governance tool a consumer cannot audit is a trust regression.** Every
   blocking gate raises the cost of a wrong red, and the answer this project
   gives is that the rule is readable, fixtured, and specified. Withholding the
   rule keeps the blocking and removes the answer.
3. **Withholding raises an obligation this project does not carry.** A consumer
   who cannot read the gate has only the build's attestation, so a withheld
   predicate demands a reproducible build and a per-artifact provenance record.
   What exists today is a sha256 digest on the Release asset and npm's publish
   provenance on the package — integrity for the *package*, and neither is a
   reproducibility claim about a gate binary.

**The port's justification is restated to match.** The substrate buys the
conservation contract, the type-level elimination of the fail-closed defect class
(§Meta-gate conservation for the binary substrate's `check-gate-fail-closed`
row), and execution cost. It does not buy concealment, and no later session may
re-argue a port on an opacity ground this ruling refuses.

## What changes

### 1. §Consumer payload — a new section stating what a gate ships {design-bearing}

gate-sdk/SPEC.md gains a **§Consumer payload** section carrying the ruling above,
its three grounds, and the four artifacts. It is the surface a later port reads
before arguing, and its absence is what let the question stay a range instead of
a value.

The section states the reach honestly: it rules what a **gate** discloses. It
does not rule what a consumer's own tree must disclose, and it does not reach the
dogfood question — whether *this* repo runs built artifacts stays
**`native-gate-dogfood-ruling`**, which asks the same lever from the other end and
is not answered here.

### 2. The fixture pair is shipping-side, and is the consumer's acceptance test {design-bearing}

This answers the contract question the entry names as one that must not be
discovered late: the `good/`+`bad/` pair is a gate-sdk contract
(§Fixture-pair discipline) held by meta-gates, and whether it is development-side
or shipping-side is unstated.

**It is shipping-side.** `gate-tests/` vendors with its kit — measured, not
intended: `installer/lib/init.sh`:127-133 enumerates a kit's payload with an
unfiltered `find . -type f`, so the pair arrives whole. That is elevated from an
artifact of the copy loop to a stated contract, because it is what gives the
build-from-source model an acceptance test: a consumer runs
`gate-sdk/bin/run-gate-tests.sh` and their locally built binary is proved against
the same cases this repo proves it against. `gate_command`'s substrate-blind
dispatch (`gate-sdk/bin/run-gate-tests.sh`:32) is what makes the pair a parity
oracle across substrates rather than a shell-only one, so no new mechanism is
needed — only the statement that the property is load-bearing and may not be
dropped to slim a payload.

This is also the consumer's answer to the honest limit
**`native-gate-meta-layer-reach`** records: the crate-side verification of a
gate's declared read-set runs where cargo runs, and a consumer's assurance is the
one it has for every kit rule — the fixture pair, which it can run itself.

### 3. No artifact ships, and that is asserted rather than trusted {design-bearing}

The install loop is unfiltered and binary-safe (`init.sh`:127-133; `claim`'s
hash check is `git hash-object`), so a compiled artifact placed under a kit root
would vendor silently, with no code change and no review signal. A ruling whose
violation is invisible is not enforced.

A new gate, **`check-payload-text`**, holds it. Invariant: every tracked file
under a kit root, and under the substrate crate root, is text. Mechanism: a NUL
byte scan over the tracked set — no `file(1)` dependency, no extension roster to
maintain. Fixture pair: a `good/` tree of text, a `bad/` tree carrying one file
with a NUL byte. It ships with a `# graph:` manifest coupling the kit roots and
the crate root at `tier=precommit`, registers in `scripts/gates.list`, and joins
its kit README roster like any gate.

Rejected alternative: asserting inside `scripts/pack-installer.sh` at pack time.
Refused because a violation would then be caught only at release, by whoever runs
the pack, on a surface no fixture covers — while the tracked tree is where the
artifact actually lands and where a per-commit gate can refuse it.

### 4. The precondition for any future artifact transport is recorded {design-bearing}

The ruling refuses shipping an artifact **today**; it does not forbid one
forever. Recorded so a later session inherits the terms rather than re-deriving
them: an artifact transport, if one is ever added, carries three things before it
ships — a reproducible build a third party can re-run to the same bytes, a
per-artifact provenance record naming what built it from which source, and a
published digest. And the source ships regardless: this ruling is about what is
**added** to a payload, never about what may be removed from one.

### 5. The install and site surfaces state the disclosure {design-bearing}

`docs/install.md` states what an install writes and what it requires; it carries
no statement about what a gate discloses, and that is a genuine gap rather than a
contradiction. It gains one sentence naming the ruling and pointing at
§Consumer payload — the widest-true tier for a reader deciding whether to adopt.
canon-kit's `check-install-claim` holds the primary-install-path claim across
governed install sections and is the reader that must stay green over the edit.

## Producers and consumers

**The ruling itself** introduces no runtime state; its producer is the SPEC
section and its consumers are the sessions and the gates below.

**`check-payload-text`.**
Producer: the tracked tree — `git ls-files` under each `gate_kit_roots_rel`
member and under `GATE_SDK_NATIVE_CRATE` (the knob **`native-gate-vendoring-model`**
introduces; this gate is its second reader, which is what keeps the crate root
from acquiring a second spelling here). Consumer: `gate-sdk/bin/run-gates.sh` via
`scripts/gates.list` registration, and the generated pre-commit hook via its
`tier=precommit` manifest. Its enabling config is registration itself, which the
kit-landing checklist (§Consumer smoke) requires and `check-kit-registration`
holds. It emits no state; its verdict is its exit code and output line, which
`check-gate-output` and the fixture runner assert against.

**The fixture pair as a shipping artifact.**
Producer: the kit's `gate-tests/<name>/{good,bad}/` tree, written by whoever
authors the gate and required by `check-gate-fixture-coverage`. Consumer, in the
**consumer** tree rather than this one: `gate-sdk/bin/run-gate-tests.sh`, which
resolves the member through `gate_command` and therefore runs the consumer's own
built binary against the shipped cases. That consumer is new — the pair has
always vendored, and this amendment names the reader that makes the vendoring a
contract instead of an accident.

**No new fields.** The descriptor gains none: it "carries no field that lacks a
reader, reserving nothing against a future reader" (§The `# graph:` manifest),
and a disclosure ruling that added a descriptor field would be declaring in data
what the payload's contents already state.

## Existing sections updated

Owned by this amendment, each named with the delta that claims it:

- gate-sdk/SPEC.md §Consumer payload — new; the ruling's home (delta 1).
- gate-sdk/SPEC.md §Fixture-pair discipline — the shipping-side statement and its
  consumer-tree reader (delta 2).
- gate-sdk/SPEC.md §check-payload-text — new gate section (delta 3).
- gate-sdk/SPEC.md §Porting a gate to the binary substrate — the port's
  justification loses the opacity ground and names §Consumer payload (the
  ruling); §What the dispatch seam does not settle's "Opacity is not claimed"
  paragraph becomes a pointer to the ruling rather than a statement of the
  present condition (delta 1).
- gate-sdk/SPEC.md §Layout and configuration — `check-payload-text`'s knobs, if
  it takes any beyond the crate-root knob it shares (delta 3).
- `README.md` gate roster for gate-sdk — the new gate's row (delta 3);
  `check-readme-roster` is the reader that reds without it.
- `docs/install.md` — the disclosure sentence (delta 5).

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
- [ ] **The new gate ships its fixture pair**, registers in `gates.list`, and
      passes the four gate-sdk contracts (output, fail-closed, fixture-pair,
      self-lint).
