# SPEC amendment: consumer-first-mover

The first port under a **non-kit root** — and the questions it answers are
`consumer-gate-port-disposition`'s, not this cohort's, which is why they are
authored here rather than inside the cohort amendment beside them. Every one of
the 13 consumer-declared gates inherits the answers; the three this iteration
ports are simply the first to need them.

**The destination is ruled and is not re-opened here.** Operator-ruled
2026-08-14: the 13 gates declared under the consumer's own gates directory become
subcommands of the existing multi-call binary, with the accepted cost that an
adopter's binary carries subcommands implementing another project's repo rules
which they can never register (`consumer-gate-port-disposition` records the
ruling, the two refused alternatives, and the cost). What this amendment owns is
the narrower thing that ruling deferred to design: **how the unregistrable
subcommands are declared** — assertion B's owner column, whose only two-column
value today is a kit directory basename, and the crate unit test behind it, which
asserts a path shape a consumer's gates directory cannot hold.

## What changes

### 1. The owner column's domain widens from *owning kit* to *declaring root*, with one sentinel for the consumer's own gates directory

`--list`'s second column keeps its shape and gains one value: **`-`**, meaning
*this subcommand is declared by the consumer's own gates directory and no kit
ships it*. The crate's dispatch registry carries `-` in the owner field for a
consumer-declared member, and `gates::names_with_owners` emits it unchanged.
**[design-bearing]**

**A sentinel rather than the gates directory's basename, and the reason is that a
basename is not an identity here.** `GATE_SDK_GATES_DIR` defaults to `scripts`
for *every* consumer (§Layout and configuration), so the same string would name
this repo's declaring root and every adopter's — the column would be reporting a
value that cannot distinguish the tree that owns the subcommand from the tree
that merely received the binary, which is the exact discrimination delta 2 needs
it for. A crate literal spelling one project's gates-directory name would also
ship that project's layout to everyone, the class §Layout and configuration
already rules for the target roster and CLAUDE.md §The provenance seam rules
generally. The sentinel is layout-independent, and it needs to be: the gate
resolves a consumer-declared descriptor through `gate_sdk_gates_dir` and already
globs that directory into its descriptor set, so nothing on the reading side
wants the name.

**`-` rather than `?`, because the sibling arm has already spent `?` on the other
meaning.** `--reads` prints `?` for a root the gate's author *cannot bound*, and
its reader counts it as undecidable rather than trusting it as empty
(§Meta-gate conservation for the binary substrate). A consumer-declared member's
owning kit is not undecidable — it is decided, and there is none. Two readings
that far apart must not share a spelling.

### 2. Assertion B's scope test gains one branch: a consumer-declared subcommand is in scope in the publishing tree, and only there

Today a subcommand is in scope iff its owner appears among the vendored kits'
directory basenames. The rule gains its second clause: **a subcommand owned by
the consumer sentinel is in scope iff the tree is a publishing tree** — the
predicate assertion F already computes, *the crate's tracked source is here*,
reused rather than spelled a second time and deliberately **source** rather than
directory presence for the reason stated there. Everything else about the
assertion is unchanged: the descriptor→subcommand direction stays unrestricted,
the `reference-only` allowance composes, out-of-scope subcommands are counted and
declared on the clean line. **[design-bearing]**

Both directions carry weight, and neither is the obvious one:

- **In the publishing tree it must be in scope, or the assertion goes dark for
  the whole tranche.** The tree carrying the crate source is the tree whose
  registry decides which subcommands exist, so it is the only tree where a
  stranded implementation can be created — a module and a registry entry landed
  while the descriptor is forgotten. Nothing else catches that: the
  descriptor→subcommand direction is the other direction, and assertion A stays
  quiet because a half-finished port leaves the member resolving to its surviving
  `.sh`. Ruling these members permanently out of scope was weighed and refused on
  exactly this: it would buy simplicity by ending, for 13 members, the one
  assertion assertion B exists for.
- **In an adopter it must be out of scope, or the equality is unsatisfiable.**
  §upgrade-smoke states the defining property of the consumer's own gates
  directory — a gate living solely there *"cannot appear in a vendored tree"* —
  while §Consumer payload has `pack-installer.sh` shipping the prebuilt binary.
  So an adopter holds the subcommand and can never hold a descriptor for it. That
  is neither dead code nor a half-finished port, which is precisely the category
  the vendor scoping was introduced for.

**Refused, and recorded because it is the first thing a later session reaches
for:** unioning the gates-directory basename into the vendored-kit name set.
Under it every adopter is in scope for all 13 (their gates directory carries the
same default name), and every adopter reds with a finding they cannot discharge —
reintroducing the unsatisfiable equality the scoping rule removed, and growing it
by the size of each tranche.

**The version-skew behavior is inherited rather than designed, and is stated
because it is load-bearing.** Against a **pre-sentinel gate** the sentinel is
simply a value absent from the vendored-kit names, so the existing code counts it
out of scope and prints the count — the same disposition this delta rules,
reached by the old code path. A newer binary in an older vendored tree therefore
does not red, and the bounded residual assertion B already states is not widened
by this delta.

### 3. The crate's owner unit test resolves the descriptor by root shape, and is renamed to say so

`every_registry_member_declares_the_kit_that_carries_its_descriptor` asserts
`<repo>/<owner>/checks/<name>.gate` for every registered subcommand. It gains a
branch — for the sentinel owner the descriptor is `<repo>/<gates-dir>/<name>.gate`,
with `<gates-dir>` read from `GATE_SDK_GATES_DIR` and defaulting to `scripts` —
and it is renamed `every_registry_member_declares_the_root_that_carries_its_descriptor`,
because a test whose name says *kit* while it asserts over two root shapes is a
name that will be read instead of the body. **[design-bearing]**

Reading a layout knob inside a `cargo test` is sanctioned **here and nowhere
else**, and the bound is stated rather than left to taste: these tests run only in
the publishing tree — `native/` ships no `checks/` and no `smoke/`, so it is not a
kit root and no consumer ever receives the crate source (§Meta-gate conservation
for the binary substrate, *Where that verification runs, and where it does not*).
The knob is gate-sdk's own documented spelling with its own documented default, so
the test learns a layout rather than a vocabulary, in the one place where knowing
this tree's layout is not a seam crossing. A gate **module** reaching for the same
knob would be a different thing entirely and is not licensed by this: a value a
module needs crosses the config bridge (§lib/gate.sh).

### 4. A consumer-declared member earns a conservation row on exactly the same terms as a kit-declared one, and §Meta-gate conservation says so in a sentence

The substrate-sensitive set is *a registry member whose expanded `couples=`
covers the declaration path of a registry member*. **Neither side of that test
mentions where a declaration lives**, and the section already contemplates the
consumer's gates directory — it excludes `scripts/queue-config.sh` on the ground
that it is not a *gate*, never on the ground that it is not in a kit. The ruling
is that **location is not a term of the derivation**, and it lands as prose in the
section because the table is entirely kit gates today and the natural reading of
that is a false exemption — an exemption whose cost is the silent end of whatever
assertions a consumer-declared meta-gate makes, which is the one failure mode the
section exists to prevent. **[design-bearing]**

The ruling has two live consequences, and they run in opposite directions:

- **As a subject** — a consumer-declared member whose couples cover a declaration
  path owes a row like any other. This tranche's three do not: their couples are
  the release-note corpus and two `.workflow/` files, none of which is a
  declaration path. That is a **measured verdict, not an exemption**, and it is
  re-derived at the cut rather than inherited, because assertion C derives the
  set at runtime and a manifest may have moved.
- **As an object** — a consumer-declared member's declaration path *changes* at
  its port, `<gates-dir>/<name>.sh` becoming `<gates-dir>/<name>.gate`, which can
  move **other** members into or out of the derived set. Delta 7 of the cohort
  amendment enumerates that delta for this tranche; the rule that it must be
  enumerated at every tranche is this one.

### 5. Three kit-root-scoped readers do not reach a consumer-declared member, and each verdict is recorded rather than left to be rediscovered per tranche

A gate whose corpus is `gate_kit_roots` cannot see the consumer's own gates
directory, and three of them matter to a port. Each is measured, not assumed, and
each verdict is written here because every later tranche member meets the same
three and would otherwise re-derive them. **[design-bearing]**

- **`check-gate-substrate-parity` assertion E's sibling half** — it asks, for every
  declared `.gate`, that no `<name>.<ext>` exist **under any kit root**, so an
  implementation dropped beside a consumer-declared descriptor is outside its
  reach. **That is its scope, not a hole**: the half enforces §Consumer payload's
  ruling, whose subject is what a consumer *receives*, and a file in this repo's
  own gates directory vendors nowhere. The crate-root half is untouched and stays
  live throughout.
- **`check-install-disposition`** — it sweeps kit roots only, so it imposes no
  `# install:` requirement on a consumer-declared gate in either substrate, and a
  consumer-declared descriptor owes no such header. This is the section's own
  recorded disposition (*substrate-blind by construction; a port moves nothing
  here*) holding as written.
- **`check-readme-roster`** — its corpus is kit READMEs against kit `checks/`
  contents, so no roster names a consumer-declared gate before or after a port and
  neither direction can fire. **The general fact this exposes is not this
  amendment's to fix and is not silently absorbed**: no consumer-declared gate has
  a roster reader at all, which is a standing coverage question over all 13 rather
  than a defect this port introduces. It is filed to the gap inbox, costed, so the
  tranche does not carry it as an unwritten assumption.

### 6. The tranche entry's terminal move is a demotion, not a Done move

On completion `consumer-gate-port-disposition` drops its `[spec:]` tag and
returns to the deferred section under `[design-pending]`. **[mechanical]**

Its deliverable is a corpus of 13 and this tranche delivers 3. A Done move would
assert a finished tranche with 10 members unported, and neither half of the
Done-move contract has a gate behind it (canon-kit/SPEC.md §Merging an
amendment), so the wrong terminal move reds nothing.

### 7. The gate's own tightening is declared, because it is a kit-shipped gate getting stricter

Delta 2 lands a new in-scope branch in `check-gate-substrate-parity`, a
**kit-shipped** gate: an adopter who ports their own gates into their own crate is
newly in scope for them. So the build stage appends the gate's bare name to
`.workflow/tightened-gates.txt` when the branch lands (gate-sdk/SPEC.md
§upgrade-smoke owns that surface's contract). **[mechanical]**

Stated because the cohort's three ported members are *consumer*-declared and
therefore owe that surface nothing — which makes "this cohort declares nothing"
the easy and wrong conclusion. The tightening is the gate's, not the members'.

## Producers and consumers

**This amendment introduces one new interface value (the owner sentinel), one new
assertion branch, and no new state, event, message or field.** The `--list`
protocol, the dispatch registry, the descriptor set and the conservation section
all exist and are unchanged in shape.

- **The owner sentinel `-`** (delta 1). **Producer:** the crate's dispatch
  registry, in the owner field of each consumer-declared member's entry — an entry
  a member cannot compile without, the same shape `--reads` and `--knobs` have —
  emitted by `gates::names_with_owners` through `--list`. Its enabling
  configuration is nothing: it is a compiled-in literal, so there is no deployed
  configuration that could fail to set it. **Consumers, both named and both
  live:** `check-gate-substrate-parity`'s assertion-B scope loop, which reads it
  at every battery run (delta 2), and the crate's own owner unit test, which reads
  it under `cargo test` (delta 3). No third consumer exists at landing.
  **Its red condition is delta 2's**, and the sentinel's own failure mode is
  covered there rather than here.
- **The publishing-tree branch** (delta 2). **Producer:** the gate, calling the
  `crate_source_here` helper it already defines for assertion F — so the producer
  is a call site added to a function with a live caller, not a new mechanism whose
  enabling config could be missing. **Consumer:** the assertion's own finding set
  and clean line; the operator reads *`subcommand nothing declares`* on red and
  the `in scope / out of scope / owner column` triple on green. **Every field has
  a named reader:** the clean line's out-of-scope count already has one — the
  operator, and the bespoke test's subset cases — and this delta adds no field to
  it, deliberately: a fourth counter naming consumer-declared members separately
  would be a number with no reader and is refused.
- **The renamed unit test** (delta 3). **Producer:** `cargo test`, run by
  `check-crate-arms` at commit time in this repo and in CI. **Consumer:** the
  contributor reading a failure. It runs **never in a consumer tree**, which is
  the property that makes the knob read in it sound.
- **The conservation-section sentence** (delta 4). **Producer:** the merge, into
  gate-sdk/SPEC.md. **Consumer:** the session cutting the next consumer tranche,
  and — machine-side — assertion C, whose behavior the sentence *describes*
  rather than changes. That is the point worth naming: the sentence adds no code,
  so its only failure mode is a later reader believing the opposite, which is
  exactly what an unwritten rule produces.

### Each reader's red condition, for the corpus this widens

This amendment **widens** rather than narrows — a value the owner column could not
previously take, and a branch that puts more subcommands in scope — so
canon-kit's point-5 narrowing analysis binds on the cohort amendment's deletions
rather than here. Two readers are still enumerated by red condition, because a
widening can red too:

- **`check-gate-substrate-parity` itself** — reds, under this delta, when the
  publishing tree carries a consumer-declared subcommand with no descriptor and no
  `reference-only` disposition. That is the assertion working; the failure mode to
  guard is the inverse, an adopter reddened by a descriptor they cannot write,
  which delta 2's out-of-scope clause forecloses and which the bespoke test's
  subset cases are the place to prove.
- **The crate owner unit test** — reds when a registry entry's declared root does
  not carry its descriptor. Under the sentinel branch it reds on a
  consumer-declared entry landed before its descriptor, which is the half-finished
  port arriving through the `cargo test` door rather than the battery door, one
  commit earlier.

## Existing sections updated

- **gate-sdk/SPEC.md §check-gate-substrate-parity** — owned by deltas 1, 2, 3, 5.
  Assertion B's owner paragraph gains the sentinel and its two-clause scope rule;
  the *crate unit test holds it to the tree* sentence is rewritten to the
  two-root-shape form rather than left stating the kit-only shape; the refused
  union and the pre-sentinel skew behavior land as the paragraph's recorded
  refusals. Assertion E's calibration paragraph gains delta 5's scope sentence.
  The bespoke test's case roster gains the consumer-declared cases delta 2 needs
  proved — a publishing tree with a consumer-declared subcommand and no
  descriptor, which must red, and the same roster in a non-publishing tree, which
  must run clean.
- **gate-sdk/SPEC.md §Meta-gate conservation for the binary substrate** — owned by
  deltas 3 and 4. The *substrate-sensitive set is derived* paragraph gains delta
  4's location-is-not-a-term sentence beside its existing
  `scripts/queue-config.sh` exclusion, which is where a reader already looks for
  the gates directory. *Where that verification runs, and where it does not* gains
  the owner test's renamed spelling and delta 3's bound on the knob read.
- **gate-sdk/SPEC.md §Layout and configuration** — owned by delta 1, one clause:
  `GATE_SDK_GATES_DIR`'s entry notes that the crate names no gates directory, the
  sentinel standing in its place, so a reader looking for where the crate learns
  this repo's layout finds the answer *nowhere* stated rather than absent.
- **gate-sdk/SPEC.md §upgrade-smoke** — owned by delta 7, if and only if the
  build session finds the declaration surface's contract does not already cover a
  gate tightened by an assertion branch; read the section before adding, since a
  restatement there is the defect rather than the fix.
- **`.workflow/tightened-gates.txt`** — owned by delta 7: one appended data line
  naming `check-gate-substrate-parity`.
- **`consumer-gate-port-disposition`'s queue entry** — owned by delta 6, at the
  terminal move.

## Definition of Done

- [ ] **Causal completeness** — the sentinel has two named consumers and no
      third; the publishing-tree branch's producer is a helper with a live caller
      rather than a new mechanism; no field is added to the clean line without a
      reader.
- [ ] **Both scope directions proved by a run, not by inspection** — a bespoke
      case where a publishing tree missing a consumer-declared descriptor **reds**,
      and one where a non-publishing tree carrying the same roster runs **clean**.
      The second is the one an adopter depends on and the one an over-tight
      predicate passes by accident.
- [ ] **The refused union is recorded** — unioning the gates-directory basename
      into the vendored-kit names, refused with the unsatisfiable-equality ground,
      in §check-gate-substrate-parity rather than only here.
- [ ] **Pre-sentinel skew is stated as inherited** — an older gate counts the
      sentinel out of scope by its existing code path; assertion B's bounded
      residual is unwidened.
- [ ] **The conservation ruling is a sentence in the section**, not a row: it says
      location is not a term of the derivation, and it does not add a table entry
      for a member the derivation does not select.
- [ ] **Provenance seam held** — no gates-directory name, and no consumer path
      literal, anywhere in `native/src/`; the one knob read is in a `cargo test`
      that never runs in a consumer tree.
- [ ] **The gate's own tightening is declared** — `check-gate-substrate-parity`
      appended to `.workflow/tightened-gates.txt`.
- [ ] **Merged with no information lost** — the assertion-B and conservation
      paragraphs rewritten in place, never appended to.
- [ ] **Amendment deleted** — this file removed on merge; none remain for gate-sdk
      (`ls gate-sdk/SPEC-*.md`), discharged at the iteration rather than at the
      commit while the cohort amendment is in flight.
- [ ] **Removals propagated** — grepped every spec for the unit test's old name
      and for the *owning kit* phrasing of the owner column; nothing dangles.
- [ ] **Terminal move is a demotion** — `consumer-gate-port-disposition` returns
      to deferred under `[design-pending]`.
- [ ] **Gaps filed** — cross-component gaps found during the work filed through
      the gap inbox.
