# SPEC amendment: this repo runs built artifacts

Pairs with the queue entry `native-gate-dogfood-ruling`.

**The ruling: yes. This repo runs built artifacts.** When a `.gate` descriptor is
live, `cargo` is a commit-time requirement in this tree, the battery depends on a
compiled toolchain at every commit, and the port's opacity ground becomes
measurable in the one tree that exercises it daily.

## Why this is a spec pass and not a build decision

gate-sdk/SPEC.md §What the dispatch seam does not settle already records that
*"the dogfood question is settled by the first live port, whenever that lands,
and not by argument beforehand."* Read alone that sentence invites the
conclusion that landing the descriptors is the whole answer and no amendment is
owed. It is not, and the distinction is the reason this unit exists and is
ordered first.

That sentence names what **determines** the answer — the question is not a free
preference, it is entailed by whether a descriptor goes live. What it does not
license is settling the answer **in code first**. TRAJECTORY.md's
`init-claim-stickiness` ruling is exactly on point: a change to what a contract
*means* is "a contract change rather than a repair, and settling it in code
first would be settling it by implementation," and its sequencing half — the
spec pass comes before any code — is part of the ruling rather than a
preference attached to it. Making a compiled toolchain a precondition of this
repo's own battery at every commit is a contract change by that test.

So both hold: the port determines the answer, and the answer is written down
before the port lands. That is the whole content of this iteration's forced
ordering, and it is recorded here so the next reader meeting those two sentences
does not read them as a contradiction and pick one.

## The trade, stated in both directions

Recorded because the queue entry states it and because a ruling whose cost is
not written down gets reopened by whoever first pays the cost.

**What it costs.** The battery depends on a compiled toolchain at every commit;
`cargo` moves from a build-and-CI floor to a commit-time one for this tree; and
a fresh clone cannot commit until it has built the crate once.

**What it buys.** The alternative — running from source here — keeps that
toolchain optional and keeps the port's headline benefit unmeasurable in the one
tree that exercises it daily, which is the entry's own framing: *"this repo must
run built artifacts or the opacity win is consumer-only, and the Rust source sits
readable in-tree regardless."*

**And the alternative is not actually available, which is the strongest ground
and the one the entry does not carry.** A descriptor and its shell script cannot
coexist in one resolve dir (§check-gate-substrate-parity assertion A), and the
kit tree this repo runs its gates from *is* the tree that vendors to consumers.
There is no arrangement in which a consumer dispatches to the binary while this
repo runs the shell script. Ruling *no* is therefore not "this repo dogfoods
differently" — it is refusing the port outright, which contradicts the PRIORITY
DIRECTIVE. The ruling is forced by mechanism, and saying so is worth more than
re-arguing the preference.

## What changes

Each delta carries its `{mechanical | design-bearing}` work class. Every
sentence this amendment lands is written as a **rule**, true at its own commit,
never as a claim about how many descriptors exist today — the present-tense tree
turns belong to `native-gate-cohort-descriptors` and land with the descriptors
themselves. This is the same window discipline the payload-claim amendment
already carries, applied here for the same reason.

### 1. §What the dispatch seam does not settle — the dogfood paragraph is retired {design-bearing}

The paragraph opening *"Dogfooding is open again, and the revert is what
reopened it"* is replaced by the settlement. What the replacement must carry:

- **The ruling**, in one sentence: this repo runs built artifacts.
- **The conditional that makes it true today.** The mechanism is stated as a
  rule — *with a live descriptor, `gate_command` puts the binary on the
  pre-commit path and `cargo` is a commit-time requirement here* — so the
  sentence is true before and after the descriptors land, and stops being a
  statement about the current count.
- **The trade**, both directions, from the section above.
- **The forced-by-mechanism ground**, which is the part no surface currently
  holds and which stops the ruling being re-argued as a preference.
- **What the section keeps.** Its remaining paragraphs (vendoring, opacity, the
  extensibility model, the language-agnostic reading, the manifest SSOT split)
  are untouched. The heading stays correct: the seam still does not settle those.

The existing text's factual half — that with no `.gate` member nothing dispatches
to the binary and `cargo` is not required to commit — is **not** a casualty of
this delta. It is a true statement about the current tree and it is
`native-gate-cohort-descriptors` that falsifies it, which is why it turns there
and not here.

### 2. context-kit/SPEC.md — the `cargo` floor's character sharpens {design-bearing}

`cargo:1.56`'s roster entry calls it "a **contributor/build** floor, not a
runtime one". After this ruling that is true but no longer precise: for a tree
carrying a live descriptor it is a **commit-time** floor, which is a stronger
claim than "build" and is the one a contributor needs.

The entry is re-worded to carry both tiers rather than replacing one with the
other, because both are real and they are real for different readers: it is a
commit-time floor where a descriptor is live (this repo, once the cohort lands),
and a contributor/build floor otherwise (a consumer tree, which receives a
prebuilt binary and never a crate).

**Two things in that entry must survive untouched, named so a rewrite does not
take them with it.** The floor still tracks *the edition the crate declares*,
never whatever rustc a box carries — that is the section's own rule against
aspirational pins. And *"Runtime is unaffected: git remains the sole runtime
dependency of a ported gate, shelled out rather than embedded"* is a closed
ruling (TRAJECTORY.md §The closed rulings) and no re-wording may soften it. The
change is to the **contributor-side** tier only.

`PROBE_SET` itself does not change: `cargo:1.56` is already a member at the
already-correct floor. Verified rather than assumed — the roster carries
`cargo:1.56` today, so `scripts/check-install-toolchain.sh`'s `name:min:impl`
parity with `docs/install.md` is undisturbed by this unit.

### 3. The build-step obligation the fixture battery does not discharge {design-bearing}

Measured this session rather than reasoned, and it is the non-obvious half of
the ruling: **`cargo test` does not refresh the gate binary.**

A full `cargo test --release --manifest-path native/Cargo.toml` ran 15/15 green
— including `source_stamp_agrees_with_the_shell_library`, which proves the two
substrates compute the source stamp identically — and `check-gate-binary-fresh`
was **still red at rc=1** immediately afterwards, reporting a baked stamp of
`d3d7c79c…` against a tree hashing to `9a23ce43…`. The test harness is a
different artifact from `native/target/release/checkwright-gates`, which
`GATE_SDK_NATIVE_BIN` names and which the gate reads. Only
`cargo build --release --manifest-path native/Cargo.toml` cleared it.

This matters because README's fixture-runner battery already carries a
`cargo test` line, so a contributor who runs the whole declared battery still
has a stale gate binary and no surface says so. The obligation is therefore
**named as its own step** wherever the contributor routine is stated, rather than
left to be inferred from the presence of a cargo line.

The reader-facing half of this — CONTRIBUTING.md's local-run bullet, which today
grounds `cargo` on a fixture runner building the crate — turns in
`native-gate-cohort-descriptors` with the rest of the present-tense prose, since
it is only insufficient once a descriptor is live. What lands **here** is the
rule it will cite: a tree with a live descriptor must build the binary before it
commits, and running the crate's tests is not that step.

## Producers and consumers

This amendment introduces **no new state, event, interface, field, or config
knob**. It rules a question and re-words three prose surfaces, so the
causal-completeness check applies to the ruling's mechanism rather than to a new
message. Stated explicitly because "no new fields" is a conclusion to verify,
not an absence to assume — and every field a reader might expect here already
exists and already has a named reader:

**The mechanism the ruling turns on (existing, unchanged).**
*Producer:* a `.gate` descriptor in a resolve dir — landed by
`native-gate-cohort-descriptors`, not by this unit. Its enabling config is
`GATE_SDK_NATIVE_BIN` (default `native/target/release/checkwright-gates`,
`gate-sdk/lib/gate.sh`:113), which is set for every reader by the library every
gate sources, so the producer's config is emitted everywhere it must be rather
than in tests only.
*Consumers:* `gate_command` (`gate-sdk/lib/gate.sh`:88-109), which resolves a
`.gate` member to `<binary> <name>` and **exits 2** when the binary is absent —
this is the code path that makes `cargo` commit-time, and it is reached from the
generated pre-commit hook because both cohort members are `tier=precommit`;
`check-gate-binary-fresh`, which arms at ≥1 descriptor; and
`check-gate-substrate-parity` assertion B.

**The `cargo:1.56` roster element (existing, unchanged).**
*Producer:* `context-kit/lib/toolfloor.sh`'s `PROBE_SET`.
*Consumers:* `context-kit/bin/env-probe.sh` (the local profile) and
`scripts/check-install-toolchain.sh`, which reads the element as a
`name:min:impl` triple and holds it in parity with `docs/install.md`'s toolchain
block. Delta 2 changes **prose only**, on both sides of a parity gate that reads
neither prose body — verified by reading the gate, whose awk extracts the
bullet's backticked name and its parenthetical alone.

**No reader is added and none is removed**, which is the whole causal claim this
amendment makes.

## Existing sections updated

- **gate-sdk/SPEC.md §What the dispatch seam does not settle** — owned by delta
  1. The dogfooding paragraph is replaced by the settlement; the section's other
  paragraphs are untouched and the heading stays true.
- **context-kit/SPEC.md, the `cargo:1.56` roster entry** — owned by delta 2. The
  contributor-side tier sharpens to commit-time-where-a-descriptor-is-live; the
  edition-tracking rule and the runtime-unaffected clause survive verbatim.
- **gate-sdk/SPEC.md §check-gate-binary-fresh** — owned by delta 3, and only for
  the one sentence its measurement falsifies by omission: the section explains
  the rebuild obligation and prints the rebuild command on red, but nothing
  states that the crate's own test run does not discharge it. One sentence,
  cited from delta 3's finding.
- **Not updated here, and named so build does not adopt them as orphans:**
  `docs/install.md` §Requirements' cargo bullet, CLAUDE.md §Housekeeping's "no
  gate is ported today" clause, and CONTRIBUTING.md's local-run bullet all turn
  in `native-gate-cohort-descriptors`. Each is present-tense about the tree, so
  turning it here would assert a state that does not exist yet — the precise
  error the 2026-08-06 demotion of `payload-disclosure-claim-owner` was made to
  correct.

## Cross-component notice

This amendment changes the contracts of **gate-sdk** (§What the dispatch seam
does not settle, §check-gate-binary-fresh) and **context-kit** (the toolchain
floor's contributor tier). That is two components, so it meets
`check-stage-entry` assertion C's cross-component test and the audit stage is
owed before build entry.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition. (This amendment introduces none; the
      §Producers and consumers section discharges it by verification rather than
      by assertion.)
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`). Discharged at the iteration rather
      than at the commit: `native-gate-cohort-descriptors` carries a sibling
      gate-sdk amendment, so only the batch merging the last of them can satisfy
      the none-remain half.
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **No sentence lands that is false until the descriptors land.** Every
      statement this unit writes is a rule or a conditional, never a count and
      never a present-tense claim about the tree. The tell to grep for at review:
      a sentence asserting that a gate *does* dispatch to the binary, or that
      `cargo` *is* required to commit, belongs to
      `native-gate-cohort-descriptors` and is a defect here.
- [ ] **The runtime bound is not softened** — no re-wording of the `cargo` floor
      entry weakens *git remains the sole runtime dependency of a ported gate,
      shelled out rather than embedded*, which is a closed ruling
      (TRAJECTORY.md §The closed rulings).
- [ ] **Ships in the same iteration as the descriptors** — this ruling and
      `native-gate-cohort-descriptors` land in one iteration, so no release
      carries a ruling whose tree state never arrived.
