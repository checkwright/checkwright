# SPEC amendment: the first ported gate cohort

Pairs with the queue entry `native-gate-binary-port`.

The seam ships, both halves of the delivery path ship, and neither prerequisite a
second port once had is outstanding (gate-sdk/SPEC.md §What is retained, and what
a second port must do first). What has not happened is a port. This amendment
rules the **first cohort** — which gates, why those, what the port must prove
before a shell script retires, and the rule that selects the next cohort.

**Scope, ruled by the iteration lead 2026-08-06.** This unit is the first cohort,
not the whole battery. The head entry completes when the cohort lands and a
successor entry carries the remaining corpus with the public roadmap commitment
intact. That is a split rather than a demotion: the whole-battery objective is
recorded in TRAJECTORY.md §The objectives — the pivot itself — and not in the
queue entry, so ending the entry at cohort scope reverses no recorded ruling.
TRAJECTORY.md's PRIORITY DIRECTIVE sequences toward *per-profile coherence, not
whole-corpus completion*, and its step 3 is "a first ported cohort".

## The cohort

**Two members: `check-action-pinning` and `check-action-gh-repo`.**

They are one cohort rather than two coincidences, and that is the whole reason
for the pairing. gate-sdk/SPEC.md §check-action-gh-repo states it outright: *"The
walk is §check-action-pinning's — `gate_find` for `*.yml` / `*.yaml` from the
scan root, shared prune set, no roster and no new knob."* One corpus derivation,
byte-identical in both scripts (`SCANROOT="${1:-.}"`, then `gate_find "$SCANROOT"
-type f \( -name '*.yml' -o -name '*.yaml' \)`), zero knobs read by either, one
declared walk root (`?`) already carried by the crate's registry for exactly this
shape. Porting them together means the substrate work is done **once** and proved
twice.

`check-action-pinning` is not a new implementation. The crate already carries its
rule as the `reference-only` disposition, kept live precisely so the substrate
would stay exercised until a port needed it. This cohort is what that disposition
was held for: the port is **adding its descriptor**, not writing its module.

### How each criterion is satisfied

All six hold for both members, checked rather than assumed:

1. **Registered** — both in `scripts/gates.list`.
2. **Fixture pair** — both carry `gate-tests/<name>/{good,bad}`.
3. **`tier=precommit`** — both, so a green `check-graph` after the port is
   end-to-end proof the manifest survived the substrate change.
4. **Not substrate-sensitive** — their `couples=` covers
   `.github/workflows/*.yml`, `.github/workflows/*.yaml`,
   `.github/ISSUE_TEMPLATE/*.yml`, `docs/_config.yml` and
   `kit:templates/*.y[a]ml`. None of those is the declaration path of a registry
   member, so neither gate audits gate sources and the parity proof is not
   self-referential.
5. **Vendored form stays runnable** — satisfied project-wide: the payload carries
   a prebuilt per-target binary, digest-verified before it is written, and both
   mechanism halves are built.
6. **Corpus derivation self-contained** — with the one qualification below.

**Criterion 6 and `gate_find`, answered by the criterion's own clause.** Both
gates derive their corpus through `gate_find`, a shared-library call, which reads
as indirection until the criterion is read in full: *"unless the duplication the
port creates is machine-held."* It is. The crate's single sanctioned walk
implementation is the binary-side counterpart, and its prune-dir default is held
against `gate-sdk/lib/gate.sh`'s by an **executed** unit test — the disposition
gate-sdk/SPEC.md §Meta-gate conservation records in the
`check-knob-default-coupling` row, chosen there for this exact reason. The
counter-example criterion 6 was written against, `check-spec-fence-balance`, is a
*config-driven* derivation with **nothing** holding the two copies together. The
distinction is not the presence of a shared call; it is whether a machine
notices when the two sides diverge.

### The two exclusions, and their grounds

**`check-memory-off` — excluded.** Scope left this boundary case open, noting its
`CONTEXT_KIT_MEMORY_DIRS` override word-splits into multiple globs — criterion
6's shape with a different intent. That is a real failure of criterion 6 but it is
the weaker one, and resting on it would leave the stronger one unrecorded. The
decisive ground is **criterion 2**: the gate's `--fixture <dir>` arm is a
different code path from its live arm, and the live arm's corpus is not in the
tree at all — it is the harness memory directory under `HOME`, named by folding
the git toplevel's `/` and `.` to `-`, reached through two nested unquoted
expansions. Criterion 2 makes the `good/`+`bad/` pair *the* parity oracle between
substrates. A gate whose fixture arm bypasses the derivation being ported has no
parity oracle for the part that matters, which is the self-referential-parity
hazard criterion 4 exists to prevent arriving through a different door. It also
sources a consumer config file and reads four knobs.

**`check-action-run-shell` — excluded, and it is the near miss worth recording.**
It sits in the same corpus family as both cohort members, shares their walk, and
reads no knob, so every mechanical screen puts it in. It is out on a fact none of
those screens sees: **it requires `shellcheck` on `PATH`**, refusing when it is
absent and invoking it per extracted block. Porting it would put an **external
tool into a compiled gate's runtime** — against TRAJECTORY.md objective 1, which
collapses the dependency floor to git, and objective 5, which admits an adopter
who will not install a toolchain. Its rule is also a hand-rolled YAML
block-scalar extractor written as an inline awk program whose output is *fed to*
shellcheck, so a port must either ship the shellcheck dependency or drop the
assertion. Neither is acceptable in a first cohort. It is recorded here rather
than silently skipped because the next selector will hit it again: **a gate's
external-tool requirement is a seventh screen the six criteria do not carry**,
and this is the entry that says so.

### What the exclusions add to the criteria

The port-candidate criteria gain a seventh, stated as the others are — with what
it cost:

7. **Its rule invokes no external program the payload does not carry.** *Found at
   first-cohort selection.* `check-action-run-shell` clears all six and is still
   unportable, because a compiled gate shelling out to `shellcheck` moves a
   toolchain requirement from this repo's contributors onto every adopter — the
   dependency floor objective 1 exists to collapse. git is the one sanctioned
   exception, because it is the floor.

## What changes

Each delta carries its `{mechanical | design-bearing}` work class.

### 1. `check-action-gh-repo`'s rule as a crate module {design-bearing}

A new module beside `action_pinning.rs`, registered in the crate's `REGISTRY`
with its declared walk root `?` — the honest answer for a scan root taken from
the gate's own first argument, and the same declaration the reference module
already carries. An entry added without its roots fails to compile, and unit test
A holds a `?` to its arity rather than to nothing.

The rule to reimplement is the invariant gate-sdk/SPEC.md §check-action-gh-repo
already states, unchanged: a job whose `run:` bodies invoke `gh` establishes a
repository context — a checkout step ordered before the first such invocation, or
`GH_REPO` set at workflow, job or step level, or every detected invocation
carrying `--repo`. The Actions-shape split is kept exactly as that section rules
it: a top-level `jobs:` key makes a file the subject, a `runs:`-shaped composite
action is **skipped and counted**, a file matching neither is outside the scan.

Design-bearing, and this is the delta that carries the cohort's real judgment. The
shell form is a GNU-awk program — the gate's own header declares it *"Requires GNU
awk (3-arg match)"* — so the port must reproduce a job-scoped, order-sensitive
walk over YAML with the crate's hand-rolled parsing and no parser dependency. The
correctness ground for the whole port is exactly this class, and retiring that
awk pin is one of the seven `context-kit/lib/toolfloor.sh` pins the port was
argued on.

**No crate dependency is added.** The rule needs byte-level line parsing and
ordering, which is what `action_pinning.rs` already demonstrates. A YAML crate
would be the artifact's first dependency, and footprint is a first-class cost
under objective 4.

### 2. The two `.gate` descriptors {mechanical}

`gate-sdk/checks/check-action-pinning.gate` and
`gate-sdk/checks/check-action-gh-repo.gate`, each non-executable, carrying only
its `# graph:` manifest and its `# spec:` pointer — the closed field roster.
Each descriptor's arrival **deletes the `.sh` beside it**: `check-gate-substrate-
parity` assertion A reds a dir carrying both, on purpose, so the two cannot
coexist even briefly. The manifest lines move across verbatim; the substrate
change must not be an occasion to edit a coupling.

Mechanical: the manifests already exist and the assertion battery is the oracle.

### 3. The parity proof, and why it must precede delta 2 {design-bearing}

Each gate's fixture pair is the parity oracle, **executed** and not merely present
— `check-gate-fixture-coverage` asserts existence only. Assertion A forbids the
descriptor and the script from coexisting, so parity cannot be proved by running
both through the registry. The order is therefore fixed and is part of this
delta:

1. Build the binary. Invoke the subcommand **directly** against the gate's
   `good/` and `bad/` case dirs.
2. Compare its output against the shell gate's output on the same cases, and on
   the live tree, **byte for byte** — the standard slice 1 met and the one this
   cohort must meet again.
3. Only then land the descriptor and delete the script.

Design-bearing because a weaker reading of "green before the script retires" —
run the fixture suite after the swap and see it pass — proves the new
implementation against itself and proves nothing about parity.

### 4. Sequencing forced by `check-gate-tamper` {mechanical}

`check-gate-tamper`'s meta-path roster does not contain `native/`, so **a commit
editing a gate's Rust implementation alongside any gate file is refused**. This is
a known limit, recorded in gate-sdk/SPEC.md §Meta-gate conservation's
`check-gate-tamper` row and carried as the live entry
`gate-tamper-roster-native-reach`; slice 1 met it by landing its Rust module in a
commit separate from its descriptor.

**This cohort does the same: delta 1 commits alone, delta 2 commits alone.** Named
as a delta rather than left to a build session to rediscover at a refused commit,
which is where slice 1 found it.

### 5. §Meta-gate conservation, the reference-only table {design-bearing}

`check-action-pinning`'s row currently reads **Reference-only** — *"its live port
was reverted; the gate is shell again and no descriptor dispatches here."* Delta 2
makes that false. The row is removed, and the table is left **empty with its
meaning stated**: no implementation is currently held ahead of a live port, which
is the table's healthy state and not a defect. The paragraph above it keeps its
job unchanged — a subcommand with no descriptor and no row still reds under
assertion B.

The two reasons that row gave for keeping the module both survive, and stating
that is the point of writing this out. The crate no longer risks going green over
nothing, because two live members now exist for `cargo test` and the `native_crate`
evidence suite to assert over. And the read-declaration unit tests keep a
non-empty registry to run against — with two members rather than one, so test A's
coverage strengthens rather than lapsing.

### 6. What the cohort settles, and what it arms {design-bearing}

Three consequences follow from a live descriptor, and each is stated because a
reader would otherwise have to derive it:

- **The dogfood question is settled by this cohort, not by argument.**
  gate-sdk/SPEC.md §What the dispatch seam does not settle says exactly that:
  *"porting one gate decided it — the dogfood question is settled by the first
  live port, whenever that lands."* A `tier=precommit` descriptor puts
  `gate_command` on the pre-commit path, `gate_command` is fail-closed on an
  absent binary, so this repo builds and runs the binary at commit time and
  **`cargo` is a commit-time requirement again**. That section is rewritten to
  record the settlement rather than the open question. `native-gate-dogfood-ruling`
  is a live deferred entry whose disposition belongs to close, so it is named as
  an owed filing here and not edited from this stage.
- **The freshness hole arms.** `native-binary-freshness-ungated` costs zero while
  no gate dispatches to the binary and becomes a live vacuous green from the first
  commit of the second port. That is this cohort. `gate-sdk/SPEC-binary-freshness.md`
  is the oracle and the two units ship in one iteration.
- **The disclosure claim becomes false.** `installer/README.md` §init and three
  sentences on `docs/install.md` assert that everything governing a consumer's
  tree is readable source. `canon-kit/SPEC-payload-claim.md` corrects them, and
  the same one-iteration binding applies.

### 7. The successor entry {mechanical}

`native-gate-port-remaining-corpus`, filed to the deferred section carrying
`[roadmap: next/reliability]`, the head's `roadmap-summary:` line, a
`Cost while deferred` field and `[design-pending]`.

**It is filed before the head entry's Done-move, not after.** `ROADMAP.md` is a
projection of the queue's **live** task sections, so the head moving to Done
deletes the public roadmap line; filing the successor first means the commitment
never lapses. Verified against the oracle rather than assumed: `check-roadmap-fresh`
requires every `[roadmap:]`-tagged entry to carry **exactly one**
`roadmap-summary:` declaration, so the successor cannot be a one-line filing.
`ROADMAP.md` is regenerated in the same commit.

### 8. The rule that selects the next cohort {design-bearing}

Recorded so the next selector runs a rule rather than a survey. **The next cohort
is the largest set of criteria-clearing gates sharing one corpus derivation.**
Shared derivation is the axis because it is what made this cohort cheap: the walk
is ported once and proved N times, and the parity comparison is over one corpus
shape rather than N. Selecting by kit, by profile, or by "whatever is easiest
next" all re-import work this cohort only paid once.

Two standing bounds on that rule, neither of which the criteria carry:

- **Criterion 4 is not relaxed.** TRAJECTORY.md refuses that for the first cohort
  with a named re-entry condition — the criterion-clearing corpus exhausted *and*
  the parity oracle held off the shell substrate. Neither holds.
- **The seventh criterion binds.** A gate whose rule shells out to anything but
  git is out until its dependency is designed away, not ported and patched later.

## Producers and consumers

**The two `.gate` descriptors (new state).**
*Producer:* delta 2, landing them in `gate-sdk/checks/`. Not test-only and not
conditional on config: they are tracked files in the resolve dir every reader
already walks.
*Consumers,* each named because a port that records nothing silently ends the
assertions its readers make:
`gate_resolve` (declaration path, for every manifest reader);
`gate_command` (invocation argv — two lines, the binary path then the bare gate
name, fail-closed on an absent or non-executable binary);
`gen-pre-commit.sh` (the generated hook, consumer-side, with no build and no
execution — which is why the manifest must stay in the descriptor);
`check-graph` (the hook and the graph artifact, byte-fresh);
`check-gate-substrate-parity` assertions A, B and E;
`check-spec-pointer` and `check-comment-tier` through
`canon-kit/lib/spec.sh`'s `.gate` arm;
`check-readme-roster`, `check-exec-bit` (a descriptor must be **non**-executable),
and `check-gate-fixture-coverage`.
*Every field has a named reader:* `# graph:` by the manifest readers above,
`# spec:` by `check-spec-pointer`. Neither descriptor carries `# no-fixture:` —
both gates have fixture pairs — and no other field exists to add.

**The `check-action-gh-repo` subcommand (new interface).**
*Producer:* delta 1's registry entry. `--list` reports it; the registry is the one
place a subcommand can come from, so nothing else can mint one.
*Consumer:* `gate_command`, by process invocation with the bare gate name;
`check-gate-substrate-parity` assertion B, which now equates a two-member
descriptor set with a two-member `--list` roster in **both** directions — the
first time in this repo's history that assertion runs non-vacuously on the
descriptor side.
*Its declared walk root* (`?`) is read by the `--reads` arm and consumed by
`check-reads-couples` in place of its refusal, and by crate unit test A, which
runs the member over its own fixture cases with recording on and holds the `?` to
exactly one observed root.

**The retired shell scripts (removed state).**
*Producers of the removal:* delta 2. *Readers that must not break:*
`check-shellcheck` loses two files from its derived corpus, correctly — no shell
exists to lint. `check-docs-cmd` reds on any governed doc still fencing a deleted
`.sh` path, which is real signal and is swept as part of delta 2.
`check-gate-fail-closed` and `check-gate-output`'s source-grep lose their subject
for these two members by the dispositions already recorded.

## Existing sections updated

- **gate-sdk/SPEC.md §The port-candidate criteria** — owned by the exclusions
  section above. Gains criterion 7 with what it cost, in the same paid-for form
  criteria 5 and 6 take.
- **gate-sdk/SPEC.md §Porting a gate to the binary substrate, §What is retained,
  and what a second port must do first** — owned by deltas 1-3. That subsection is
  written for a tree with no live port; the cohort ends that state. It records the
  cohort as the second port, keeps the retained roster, and drops the
  waiting-on-nothing framing that only made sense before one landed.
- **gate-sdk/SPEC.md §What the dispatch seam does not settle** — owned by delta 6.
  Its dogfooding paragraph is rewritten from an open question to a settlement, on
  its own stated terms.
- **gate-sdk/SPEC.md §Meta-gate conservation for the binary substrate** — owned by
  delta 5. The reference-only table loses its one row and gains the statement of
  what an empty table means.
- **gate-sdk/SPEC.md §check-action-pinning and §check-action-gh-repo** — owned by
  deltas 1-2. Each records that its implementation is a compiled subcommand and
  its declaration a descriptor; the invariants themselves are unchanged, and
  saying so explicitly is what keeps a substrate change from reading as a rule
  change.
- **`native/src/gates/mod.rs`'s registry and `native/README`-side roster** — owned
  by delta 1.
- **CLAUDE.md §Housekeeping** — owned by deltas 1-2. The `native/` bullet states
  "**No gate is ported today:** the seam ships, the one live port was reverted".
  That becomes false at delta 2 and is corrected in the same unit.
- **docs/install.md §Requirements and the toolchain roster** — owned by delta 6,
  only if the dogfood settlement changes what a *contributor* to this repo needs.
  The consumer-facing floor is unchanged, and the amendment says so rather than
  leaving a reader to infer it.

## Cross-component notice

This amendment changes **gate-sdk**'s contracts (a criterion, four spec sections,
two gate sections), **native**'s (a new module, a registry entry), and the root
manifest, and it is bound to `canon-kit`'s and `installer`'s surfaces through
delta 6. It is a cross-component amendment on `check-stage-entry` assertion C's
own test — as are the two amendments already landed this iteration — so the audit
stage is owed before build entry, and `spec` recommends `align` as the next stage.

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
- [ ] **Gaps filed** — including the two this amendment already names:
      `native-gate-dogfood-ruling`'s disposition, and the tamper-roster
      accommodation if `gate-tamper-roster-native-reach` has not landed.
- [ ] **Parity proved by execution, before the swap** — both members byte-identical
      across substrates on their fixture pairs and on the live tree, with the
      comparison run while both implementations still exist.
- [ ] **Two commits, not one** — Rust modules and descriptors land separately, or
      `check-gate-tamper` refuses the commit.
- [ ] **The crate gains no dependency** — this cohort's rules are hand-rolled
      parsing, as `action_pinning.rs` already is.
- [ ] **The successor entry is filed before the head's Done-move**, carrying
      `[roadmap: next/reliability]` and exactly one `roadmap-summary:`, with
      `ROADMAP.md` regenerated — so the public commitment never lapses.
- [ ] **Ships with its two siblings** — the freshness oracle and the disclosure
      correction land in this iteration, because this cohort is what arms the one
      and falsifies the other.
