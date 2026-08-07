# SPEC amendment: the first cohort's descriptors go live

Pairs with the queue entry `native-gate-cohort-descriptors`.

The two first-cohort rules ship as compiled subcommands today and dispatch to
nothing: their descriptors are held. This amendment lands them, and lands the
one correction that landing them turns out to require — a descriptor on disk is
**not** what makes the binary load-bearing, and treating it as such reds every
tree that legitimately has no binary.

## The correction this unit could not have shipped without

The held-descriptor design rests on a premise recorded in three places — that a
vendored `.gate` with no binary behind it reds a consumer's battery, and that
cutting the first binaries tag is what clears it. The first half is true. **The
second half is false, and it was verified by oracle at spec rather than
reasoned:** with two descriptors on disk and no binary,

    check-gate-binary-fresh: ... but 2 .gate descriptor(s) dispatch to it —
      the check could not run; treating as failure (not clean)        rc=2
    check-gate-substrate-parity: ... identical text                   rc=2

Both derive their descriptor set by globbing `*.gate` across the resolve dirs
(`check-gate-binary-fresh.sh`:27) and read no registry, so **no registry edit can
silence them**. Three trees are therefore red on a vendored descriptor no tag
will ever give them a binary for:

- **`gate-sdk/bin/run-consumer-smoke.sh`**, which vendors kit roots by `cp -R`
  and which §Consumer smoke rules *installs no gate binary by design*. Its red
  never dissolves.
- **`installer/consumer-smoke/run-smoke.sh`**'s artifact-free leg
  (`consumer-smoke-artifact-arm`, this iteration's unit 2).
- **A real adopter on `init`'s own omit path.** `select_artifact` sets
  `OMIT_REASON` to `substrate-unavailable` (host target outside the roster) or
  `digest-unverifiable` (no hasher). Both drop the ported members from
  `gates.list` while the descriptors still vendor — nothing filters `.gate` in
  `init.sh` or `pack-installer.sh`. The battery then exits 2 on every run,
  defeating that design's own stated promise that *"a re-run on a machine that
  has since gained a hasher converts it back into a live member with no hand
  edit"*: the battery is **red** until then, not merely narrower.

§Consumer smoke's explanation of the smoke red is also imprecise and is corrected
below: it attributes the red to `gate_command`, but the ported gates are not in
the smoke registry at all, so `gate_command` is never called for them. The red
arrives through the two registered meta-gates.

## The predicate, corrected

**A `.gate` file on disk is a declaration. A *registered member resolving to*
a `.gate` file is a dispatch.** Only the second makes the binary load-bearing,
and only the second can run stale.

This is not a new rule — it is §check-gate-binary-fresh's own zero-descriptor
reasoning applied at the right granularity. That section grounds its clean report
on *"No gate dispatches to the binary, so nothing can run stale."* That sentence
is exactly right; what was wrong was reading "a descriptor exists" as "a gate
dispatches". A member `gates.list` does not carry — whether never registered, or
registered and then commented out by `init`'s `# omitted:` record, which the
runner already strips from the live set — dispatches to nothing.

The correction needs **no new field, no marker file and no new knob**: the
omission record `init` already writes removes the member from the live registry,
so the same predicate covers the omit path, the unregistered-in-smoke case, and
the fully-registered case, with nothing to keep in sync.

**What it deliberately does not weaken.** §check-gate-substrate-parity assertion
B's *roster half* — the binary's `--list` against the descriptor set — stays
gated on the binary merely being readable, descriptor count and registry
irrelevant. That split was the correction the reverted port paid for, and it is
the half that catches a stranded implementation. Only the **fail-closed-on-absent-binary**
arm moves onto the corrected predicate. Stated because collapsing the two is the
obvious simplification and it would re-open the exact hole the revert closed.

**And in this repo it changes nothing, which is the point.** Both members are
registered in `scripts/gates.list`, so both are load-bearing here, both gates
stay fail-closed, and `cargo` is a commit-time requirement — which is
`native-gate-dogfood-ruling`'s settlement arriving in the tree rather than being
softened by this delta.

## What changes

Each delta carries its `{mechanical | design-bearing}` work class.

### 1. The two descriptors, and the two deletions {mechanical}

`gate-sdk/checks/check-action-pinning.gate` and
`gate-sdk/checks/check-action-gh-repo.gate`, each carrying the closed field
roster (§The `# graph:` manifest): the `# graph:` manifest **verbatim** from the
`.sh` it replaces, and a `# spec:` pointer. Non-executable —
§check-exec-bit asserts it. Both `.sh` files are deleted **in the same commit**,
because assertion A reds on a dir carrying both.

Two constraints on that commit, both already recorded and both easy to trip:

- **It may not also touch `native/`.** `check-gate-tamper`'s meta-path roster
  (`DELEGATION_KIT_META_PATHS`) does not contain `native/`, so a commit editing a
  gate's Rust implementation alongside its descriptor is refused.
- **The binary must be rebuilt before that commit, and `cargo test` does not do
  it.** Measured at spec: a full `cargo test --release` ran 15/15 green and
  `check-gate-binary-fresh` was still red at rc=1 immediately after, because the
  test harness is a different artifact from the one `GATE_SDK_NATIVE_BIN` names.
  Only `cargo build --release --manifest-path native/Cargo.toml` clears it. The
  rebuild writes to gitignored `native/target/`, so it does not violate the
  constraint above.

Mechanical: the manifest is transcribed, the deletions are forced, and the
verification is a battery run.

### 2. The load-bearing predicate {design-bearing}

`check-gate-binary-fresh` and `check-gate-substrate-parity`'s fail-closed arm
resolve the live registry and treat the binary as load-bearing only where a
**registered** member resolves to a `.gate`. With none, both report clean and say
so — naming the descriptor count *and* the dispatching count, so a reader can
tell "no descriptors" from "descriptors nothing dispatches to", which are
different tree states that must not share a report line.

`check-gate-binary-fresh` gains the registry as an input it does not read today;
`check-gate-substrate-parity` already reads `gates.list` and needs no new input.
Both keep their existing positional arguments so the fixture pairs stay hermetic.

Design-bearing: the predicate boundary **is** the assertion's contract, and the
failure mode of getting it wrong is silent — too loose and a stale binary passes
unnoticed, too tight and every omit-path adopter reds. The `bad/` fixture is the
near miss: a registered member resolving to a descriptor with no binary, which
must still exit 2.

### 3. The oracle configurations, relocated {design-bearing}

Two sections assign coverage to trees this unit moves off their configuration,
and a coverage claim that is quietly false is worse than none:

- §check-gate-substrate-parity assigns *no descriptors, binary present* to this
  repo's battery and *no descriptors, no binary* to the consumer smoke.
- §check-gate-binary-fresh assigns *zero descriptors* to this repo's battery.

After this unit, this repo is *descriptors registered + binary*, and both smoke
trees are *descriptors present, none dispatching*. Both sections are rewritten to
the configurations that actually exist afterwards, and the two no-descriptor
configurations move into the `good/`+`bad/` fixture corpus, where they are
reachable regardless of what any live tree happens to be. The new
descriptors-present-but-none-dispatching configuration gains a fixture case too —
it is delta 2's whole subject and would otherwise be proved only by the smoke
runs.

### 4. `DELEGATION_KIT_GATE_FILES` gains the `.gate` spelling {mechanical}

`delegation-kit/lib/delegation.sh`'s kit default is `check-*.sh` only. This repo
is covered — `scripts/delegation-config.sh` already carries `*/checks/*.gate` —
but a **consumer on the kit default** would receive a ported gate whose edits
escape `check-gate-tamper`'s isolation rule. The conservation table already
mandates this widening as the disposition that comes due at a port; this is the
port, so it comes due. Mechanical: two globs and a fixture run.

### 5. The prose that turns with the tree {design-bearing}

Each of these is present-tense about a tree state this unit creates, which is why
none of them turned in unit 1:

- **§Meta-gate conservation's reference-only table is emptied** — both rows go,
  since each implementation is now dispatched to. What an **empty** table means
  must be stated rather than left as an absent table: the disposition still
  exists and a subcommand no descriptor dispatches to is still red; there simply
  is no such subcommand today. An empty table read as "the rule lapsed" is the
  vacuity the whole section exists to refuse.
- **§What the dispatch seam does not settle** — the factual half unit 1
  deliberately left standing (*"no gate dispatches to the binary, and `cargo` is
  not required to commit"*) is now false and turns here.
- **§Consumer smoke** — corrected on the mechanism, per the finding above: the
  ported gates are not in the smoke registry, so the red it describes arrives
  through the two registered meta-gates, not through `gate_command`. And with
  delta 2 the harness is green, so the sentence bounding what it proves is
  restated rather than deleted — it still installs no binary; what changed is
  that it no longer needs one.
- **§check-action-pinning and §check-action-gh-repo** — each names its
  implementation substrate; both are now descriptors.
- **§The first cohort** — *"the descriptors are held"* and *"the port is
  therefore proved but not live"* both turn, along with the sentence listing the
  three consequences that *wait* with them.
- **CLAUDE.md §Housekeeping** — the *"no gate is ported today"* clause, and the
  sentence describing `check-gate-binary-fresh` as dormant.
- **CONTRIBUTING.md** — the local-run bullet grounds `cargo` on a fixture runner
  building the crate. That is now true-but-insufficient: a contributor who runs
  the whole declared battery still has a stale gate binary, because `cargo test`
  is not `cargo build --release`. The build step is named as its own obligation,
  citing the rule unit 1 lands.

### 6. `docs/install.md` §Requirements' cargo bullet, entire {design-bearing}

The bullet is corrected as **one unit**, not sentence by sentence, and the reason
is structural rather than convenient. Its whole shape is a contrast —
*contributors need cargo, installers never do*. This unit falsifies the first
half: a commit now requires the binary. Three lines later the same bullet asserts
*"every tagged release now publishes a prebuilt binary for each platform in the
declared target roster"*, which is false today — the newest tag and Release are
both v0.21.0 and no tagged release has ever published a binary — and which the
first binaries tag does **not** repair, since the quantifier ranges over every
tagged release and cutting v0.22.0 moves it from zero-of-22 to one-of-23.
Correcting only the first half would leave the bullet asserting both that a
commit requires the binary and that every release already ships one: still false,
and now incoherent.

The corrected bullet must say what is true after this unit — that `cargo` is a
commit-time requirement for a contributor and has no install-time role — and
must state the release-asset fact without a universal quantifier the tag cannot
satisfy. It stays inside the `toolchain:begin`/`end` block, whose
`name:min:impl` triple is unaffected: `check-install-toolchain` holds the
backticked name and the parenthetical only, verified by reading the gate's awk.

**The gap disposition this delta owes, stated rather than left to silence.**
That last fact is the point: the corrected sentence lands on a governed,
published surface with **no oracle over it** — precisely the shape that produced
`payload-disclosure-claim-owner` in the first place, one claim class over. The
disposition ruled here is **accepted ungated, and filed rather than absorbed**: a
release-asset claim class has exactly one known instance, and minting a second
claim gate to hold one sentence would repeat the registry-for-one-member move
that unit 4's amendment already refuses on its own axis. What makes that
acceptable rather than a repeat of the original defect is that the instance is
*filed with its cost* as a deferred entry naming the trigger — a second
release-asset claim appearing on a governed surface — at which point the two
collapse into `check-payload-claim`'s vocabulary rather than into a third gate.
Silence here is what the doctrine forbids; a costed, triggered deferral is not
silence.

### 7. The generated-projection fan-out {mechanical}

Absent from the entry's deliverable list and recovered here so a batch does not
discover it at a red commit. Deleting two `.sh` and adding two `.gate` changes
per-kit line counts, and the SPEC edits above change a mirrored surface:

- `docs/footprint.md` and `docs/value.md`'s rollup block — regenerated **after**
  `git add`, never before, since an unstaged change is invisible to the emitter.
- the on-site SPEC mirror, for every kit SPEC this amendment edits.
- `docs/check-graph.html` and the generated pre-commit hook, both of which read
  declaration paths.
- `docs/enforcement.md` does **not** move: it projects the class registry and
  both members keep `tier=precommit`. Named because the fan-out roster invites
  regenerating it anyway, and a needless regen is indistinguishable from a
  required one at review.

Each freshness gate prints its own regen command on red
(docs/site-architecture.md §Generated projections).

## Producers and consumers

**The `.gate` descriptor (new interface — the declaration going live).**
*Producer:* delta 1, as tracked files in `gate-sdk/checks/`. Reachable and not
test-only: they vendor to every consumer with the kit root.
*Consumers:* `gate_resolve` (declaration path) and `gate_command` (invocation
argv, `<binary> <name>`); the manifest readers — `check-graph`,
`enforcement-map.sh`, `footprint.sh`, `gen-pre-commit.sh` — which read `# graph:`
as text and need no build; `check-spec-pointer` for the `# spec:` line;
`check-exec-bit` for the non-executable assertion; `check-readme-roster`, whose
glob already covers `*.gate`; and `canon-kit/lib/spec.sh`'s comment surface,
whose `*.gate` arm is already in place. Each was verified present at spec rather
than assumed, because a port that silently ends an assertion is the failure mode
§Meta-gate conservation exists to prevent.

**The live-registry membership signal (new *reader* of an existing surface).**
*Producer:* `scripts/gates.list` in this repo; the consumer's `gates.list` as
written by `installer/lib/init.sh`'s `plan_gates`, which emits either a bare
member line or a `# omitted: <name> <reason>` comment when `OMIT_REASON` is set
and the member `dispatches_to_binary`. The producer's enabling config is
therefore already emitted on every real install path — the omit branch is not a
test-only configuration, it is what an adopter outside the target roster
receives.
*Consumers:* `check-gate-binary-fresh` (new reader) and
`check-gate-substrate-parity`'s fail-closed arm (existing reader, new use), both
per delta 2.
*Every field has a named reader:* the registry line carries one field, the member
name. It is read at the load-bearing decision — is any registered member's
declaration a `.gate` — and in each gate's clean and violation report, which
names the dispatching count beside the descriptor count so the two tree states
stay distinguishable. No field is added to the registry grammar; the `# omitted:`
comment is consumed exactly as the runner already consumes it, by being stripped
from the live set.

**No new configuration knob.** Both gates keep `GATE_SDK_NATIVE_BIN` and
`GATE_SDK_NATIVE_CRATE` and their existing positional arguments. Stated as a
verified absence: a knob selecting the registry would be a second spelling of
`GATE_SDK_GATES_DIR`.

## Existing sections updated

- **gate-sdk/SPEC.md §check-gate-binary-fresh** — owned by deltas 2 and 3. The
  trigger coupling is restated on the corrected predicate, and the coverage split
  is reassigned.
- **gate-sdk/SPEC.md §check-gate-substrate-parity** — owned by deltas 2 and 3.
  Assertion B's fail-closed arm moves onto the predicate; the roster half's
  binary-readable trigger is explicitly preserved, with the reason.
- **gate-sdk/SPEC.md §Meta-gate conservation for the binary substrate** — owned
  by delta 5 (the reference-only table emptied and what that means) and delta 4
  (the `check-gate-tamper` row's limit is discharged rather than merely noted).
- **gate-sdk/SPEC.md §Consumer smoke** — owned by delta 5, on both the mechanism
  correction and the bound restatement.
- **gate-sdk/SPEC.md §The first cohort, and the rule that selects the next**, and
  **§What the dispatch seam does not settle** — owned by delta 5.
- **gate-sdk/SPEC.md §check-action-pinning, §check-action-gh-repo** — owned by
  delta 5.
- **gate-sdk/SPEC.md §The `# graph:` manifest** — owned by delta 1, only where it
  describes the descriptor as a format no live gate uses yet.
- **delegation-kit/SPEC.md §Layout and configuration** — owned by delta 4, the
  `DELEGATION_KIT_GATE_FILES` default's documented value.
- **CLAUDE.md §Housekeeping, CONTRIBUTING.md, docs/install.md §Requirements** —
  owned by deltas 5 and 6.
- **docs/site-architecture.md §Generated projections** — owned by delta 7 only in
  the sense that this unit runs the fan-out that section rosters. No new row.

## Cross-component notice

This amendment changes the contracts of **gate-sdk** (two gates' predicates, the
conservation table, the smoke bound, the cohort record), **delegation-kit** (the
gate-file roster default), and this repo's root-governed **CLAUDE.md**,
**CONTRIBUTING.md** and **docs/** surface. That is comfortably a cross-component
amendment on `check-stage-entry` assertion C's test, and the audit stage is owed
before build entry — with delta 2's predicate as the thing most worth an
independent read, since it is the one delta that loosens an assertion.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`). The none-remain half is discharged at
      the iteration: `native-gate-dogfood-ruling` carries a sibling gate-sdk
      amendment, so only the batch merging the last of them can satisfy it.
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles. The named risk: any surface citing
      `check-action-{pinning,gh-repo}.sh` as a path. Verified at spec that only
      the two files' own `# usage:` lines do, so `check-docs-cmd` will not red on
      the deletion — re-verify rather than inherit.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **Every no-binary tree is green, asserted by running each one.**
      `gate-sdk/bin/run-consumer-smoke.sh`, `installer/consumer-smoke/run-smoke.sh`
      (both legs), and a simulated omit-path install. This is the DoD item the
      unit exists for, and it is discharged by running the harnesses, never by
      reasoning about the predicate.
- [ ] **This repo stays fail-closed.** With both members registered and the
      binary removed, `check-gate-binary-fresh` and `check-gate-substrate-parity`
      must still exit 2. A delta 2 that turned this repo green would have
      softened the dogfood ruling instead of implementing it, and it is the one
      way delta 2 can pass its own fixtures and still be wrong.
- [ ] **The roster half is untouched** — assertion B's `--list` comparison still
      runs whenever the binary is readable, descriptor count and registry
      irrelevant.
- [ ] **The release-asset claim's disposition is filed, not merely stated** — the
      deferred entry exists, carries its cost, and names the trigger that
      collapses it into `check-payload-claim`'s vocabulary.
- [ ] **Ships in the same iteration as its neighbours** — unit 2's omission leg
      depends on delta 2, and unit 4's corrected prose depends on these
      descriptors existing.
