# SPEC amendment: the consumer smoke packs a real binary

Pairs with the queue entry `consumer-smoke-artifact-arm`.

**The ruling: the smoke builds.** `installer/consumer-smoke/run-smoke.sh` gains a
leg that compiles the crate for the host target, emits its digest sidecar, and
packs the result through `--artifacts`, so the gate binary's placement path —
target resolution, pre-write digest verification, the seam knob, and the artifact
lock record — is exercised by the suite instead of by a dated hand-verification.

## The open question, and why it is now answerable

The entry sat `[design-pending]` on one question and named it precisely: whether
the smoke **builds** a binary — "a `cargo` dependency the suite does not have
today, against the toolchain floor" — or **fabricates** a stand-in with a
matching digest, "cheap and hermetic, but then it exercises placement without
exercising the real producer."

Three grounds settle it, and the first is why this unit sits where it does in the
iteration rather than waiting another cycle.

**1. The dependency is already paid.** `native-gate-dogfood-ruling` rules that
this repo runs built artifacts, so `cargo` is a commit-time requirement in this
tree with a live descriptor. A suite that already cannot run its own battery
without a built binary acquires nothing by building one here. The entry's sole
stated objection to *builds* is discharged by its predecessor rather than by an
argument, which is what makes this unit an insertion into the iteration rather
than a re-cut of it.

**2. A host build satisfies the whole roster — verified, not assumed.**
`pack-installer.sh --artifacts` is strict in a way the entry does not record: it
iterates **every** roster target and exits 2 on the first one with no artifact
directory, with the help text *"a roster target no leg built is a broken payload,
not a narrower one."* Had the roster carried several targets, *builds* would have
meant cross-compilation and the ruling would plausibly have gone the other way.
It does not: `native/targets.list` declares exactly one target,
`x86_64-unknown-linux-gnu`. So a single host build on the CI runner and the
maintainer's box satisfies pack's all-targets demand with no cross-compilation
and no fabrication.

**3. Fabrication buys the weaker oracle, by the entry's own reasoning.** A
stand-in exercises placement without exercising the producer, which leaves the
one thing most likely to break — the real build leg's digest agreeing with what
`init` verifies before writing — still covered by nothing.

**The honest limit, recorded because ground 2 is a fact about today's roster
rather than a property of the design.** The roster is the single surface
asserting platform support and it is expected to grow. The moment it declares a
second target, a host build no longer satisfies `--artifacts` and this ruling's
ground 2 expires. The re-entry is named here so it is not re-derived under
pressure: either the smoke's build leg cross-compiles every declared target, or
its pack is steered at a narrowed roster through the existing
`GATE_SDK_NATIVE_TARGETS_FILE` knob (`gate-sdk/SPEC.md` §Layout and
configuration) so the smoke commits to the host target alone while the published
payload still commits to all of them. The second is cheaper and is the
recommendation; neither is built here, because a seam with one instance is the
shape of a design that fits nothing later.

**What this ruling does not touch: `pack-installer.sh` still never builds.** Its
own contract — *"the script never builds one, so a locally-built binary can never
substitute for a released one"* — is load-bearing and survives verbatim. The
**smoke** builds and hands pack a directory; pack's rule is about what the
publishing path may do, and nothing here gives it a build step.

## What changes

Each delta carries its `{mechanical | design-bearing}` work class.

### 1. The smoke's build leg {design-bearing}

Before the first pack, the smoke compiles the crate for the host target, writes
the binary and a `<binary>.sha256` sidecar beside it into a per-target directory
under its own scratch, and passes that directory to `--artifacts`.

Four things this leg must get right, each of which is a way it could pass while
proving nothing:

- **The layout is pack's, not the smoke's invention.** `--artifacts <dir>` is
  read as `<dir>/<target>/<binary>` plus `<dir>/<target>/<binary>.sha256`, where
  `<target>` is a roster line and `<binary>` is `GATE_SDK_NATIVE_BIN`'s basename.
  Both are resolved through the existing accessors (`gate_native_targets`,
  `gate_native_bin`) rather than spelled literally, so the smoke cannot drift
  from the roster or from a knob change.
- **The digest is computed once, by this leg.** Pack re-verifies it with
  `sha256sum -c` and refuses on a mismatch, so the sidecar is the leg's real
  output rather than decoration. Stated because a reader meeting
  §check-gate-substrate-parity assertion F's *each digest has one producer* rule
  will ask whether this is a second producer: it is not — assertion F's scanned
  surface is `GATE_SDK_NATIVE_PUBLISH_WORKFLOW`, and the smoke is outside it.
- **It writes nothing in-tree.** The smoke already refuses to run against a dirty
  worktree, and the crate's build output lands in gitignored `native/target/`, so
  the build leaves `git status` clean and the existing guard keeps its meaning.
  The artifact directory the leg assembles lives under the smoke's own scratch,
  never in the repo.
- **A build failure is `blocked`, not `fail`.** The smoke already separates an
  environment refusal from an assertion failure, and a machine that cannot
  compile the crate has not falsified the install path.

### 2. Both branches run, and the omission branch is the one at risk {design-bearing}

Today only the omission branch executes. The defect is that the placement branch
does not — but the fix must not invert it, because the omission branch is a real
adopter's path (`init`'s `substrate-unavailable` and `digest-unverifiable`
outcomes), not a degenerate case.

So the smoke asserts **both**: one leg installs from a pack carrying artifacts
and asserts placement, and one leg installs from an artifact-free pack and
asserts the omission record. The exact leg arrangement — which of the existing
three packs gains artifacts, whether the omission leg is a fourth — is build
calibration and is deliberately not fixed here; what is fixed is that neither
branch may end this unit unexercised.

**A sequencing constraint that binds this delta to the next unit and is stated
so a batch does not discover it.** With `native-gate-cohort-descriptors` landed,
the artifact-free leg vendors two `.gate` descriptors into a consumer with no
binary, where `check-gate-binary-fresh` and `check-gate-substrate-parity` both
exit 2 — verified by oracle. That leg therefore **cannot be green on descriptors
alone**; it is green again only once those two gates are taught to read a
declared omission, which is `native-gate-cohort-descriptors`' delta. This unit
lands first and its omission leg passes on today's zero-descriptor tree; the next
unit is what keeps it passing. Neither unit is complete without the other, and
the Definition of Done binds them to one iteration.

### 3. What the placement leg asserts {design-bearing}

The entry names four assertions that landed with `native-artifact-install-path`
and whose branch has never run. Each is named so the leg is written against a
list rather than against whatever the author remembers:

- **Target resolution** — `init` selects the host's triple from the payload's
  roster copy, and the three outcomes stay distinguishable (placed / declared but
  absent / never declared), which is the distinction `select_artifact`'s own
  `# spec:` line says collapsing is the defect.
- **Pre-write digest verification** — the published digest is checked *before*
  anything is written, and a tampered artifact refuses rather than warns.
- **The seam knob** — `GATE_SDK_NATIVE_BIN` is written into the consumer's config
  naming the placed binary, before the hook is generated, since the generator
  resolves a `.gate` member to it.
- **The artifact lock record** — the placed binary, its target and its digest are
  recorded in the consumer's lock, so an upgrade can tell what it is replacing.

### 4. The prose that turns with it {mechanical}

`installer/README.md` §The consumer smoke states what the suite proves; it
currently describes a suite whose artifact path is unexercised. It gains the
build leg and both branches. `installer/README.md` §The gate binary keeps its
contract unchanged — this unit adds no behavior to `init`, only an oracle over
behavior `init` already has.

## Producers and consumers

**The artifact directory (new interface, smoke-internal).**
*Producer:* the smoke's build leg (delta 1) — `cargo build --release
--manifest-path native/Cargo.toml` followed by a digest write, into a
per-target directory under the smoke's scratch. Reachable and not test-only in
the sense that matters: the leg runs on every invocation of a suite that is a
declared member of this repo's battery (README §This repo, governed), not behind
a flag someone must remember.
*Consumer:* `scripts/pack-installer.sh --artifacts`, which reads
`<dir>/<target>/<binary>` and `<dir>/<target>/<binary>.sha256`, re-verifies the
sidecar with `sha256sum -c`, and copies both into `payload/artifact/<target>/`
beside a verbatim copy of the roster.

**Every field has a named reader.** The directory carries exactly two files per
target and both already have readers in the shipped install path, which is why
no new field is introduced:

- `<binary>` — read by `pack-installer.sh` (copied into the payload) and then by
  `installer/lib/init.sh`'s `select_artifact`, which digests it and, on a match,
  writes it to `ARTIFACT_PATH` and records it in the lock.
- `<binary>.sha256` — read by `pack-installer.sh`'s `sha256sum -c` at pack time
  and by `select_artifact`'s `want`/`got` comparison at install time, which is
  the pre-write verification delta 3 asserts.

**No new configuration knob.** The leg reads `GATE_SDK_NATIVE_BIN`,
`GATE_SDK_NATIVE_CRATE` and `GATE_SDK_NATIVE_TARGETS_FILE`, all existing, all
defaulted in `gate-sdk/lib/gate.sh`, all already read by other callers. Stated
explicitly rather than left as an absence: a new knob here would be a second
spelling of the crate's location, which `gate_native_crate` exists to prevent.

**The omission branch (existing, unchanged by this unit).**
*Producer:* `select_artifact` setting `OMIT_REASON` to `substrate-unavailable` or
`digest-unverifiable`.
*Consumer:* `plan_gates`, which writes `# omitted: <name> <reason>` into the
consumer's `gates.list` for each member that `dispatches_to_binary`. Named here
because delta 2's omission leg asserts that record, and because
`native-gate-cohort-descriptors` adds a second consumer of it — the two binary
meta-gates, which today read no such record and are why that leg needs the next
unit to stay green.

## Existing sections updated

- **installer/README.md §The consumer smoke** — owned by delta 4. What the suite
  proves gains the build leg and the two branches.
- **installer/README.md §The gate binary** — owned by delta 3, for one sentence
  only: the placement path now has an automated oracle rather than a dated
  hand-verification. The behavior it specifies is unchanged.
- **Not updated here, and named so build does not adopt them as orphans:**
  `gate-sdk/SPEC.md` §check-gate-binary-fresh and §check-gate-substrate-parity
  are what delta 2's sequencing constraint depends on, and they turn in
  `native-gate-cohort-descriptors`. `gate-sdk/SPEC.md` §Consumer smoke governs
  the *other* harness (`run-consumer-smoke.sh`), which is also that unit's.

## Cross-component notice

This amendment changes the contract of **installer** (its consumer smoke gains a
build leg and an artifact-packing branch) and reads, without changing, gate-sdk's
native knobs and `scripts/pack-installer.sh`'s `--artifacts` contract. It also
carries a sequencing dependency on **gate-sdk** through delta 2. Whether that
clears `check-stage-entry` assertion C's two-component test is a judgment the
audit stage should make rather than one this amendment should assert for itself;
the `spec` session's next-stage recommendation names the audit stage on the
strength of its sibling amendments regardless.

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
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **Both branches are exercised, asserted by running the suite rather than
      by reading it.** The placement branch and the omission branch each run and
      each assert. The tell that this was got wrong: a suite that is green while
      one branch never executes is the exact defect this unit exists to close,
      and it is what the entry's *"a hand-verified path under a green suite reads
      as covered"* names.
- [ ] **The pack path still never builds** — `pack-installer.sh` gains no build
      step, so a locally-built binary still cannot substitute for a released one
      on the publishing path.
- [ ] **The smoke still writes nothing in-tree and still refuses a dirty
      worktree** — the build output is gitignored and the artifact directory is
      scratch-local, so the existing guard keeps its meaning.
- [ ] **Ships in the same iteration as the descriptors** — the omission leg
      cannot stay green once descriptors land until
      `native-gate-cohort-descriptors` teaches the two binary meta-gates to read
      a declared omission. The two units land in one iteration.
