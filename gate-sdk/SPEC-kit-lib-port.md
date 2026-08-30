# SPEC amendment: kit-lib-port

The port disposition of the **kit library class** — every kit's `lib/*.sh`. Cut
2 of 3 of the `port-declaration-cohort-and-windows-leg` declaration cohort,
sequenced second: heavier than either sibling by line count and lighter by
argument, since two members already declare on the ground.

**The deliverable is the class ruling and the declarations it licenses.** The
ruling is a partition, and the discriminator is not the one the class's name
suggests. What decides a member is whether it is the config bridge's **sole
resolver** for its kit's knobs — a property of the file's content, which does
not follow from its directory.

## What changes

### (1) `gate-sdk/SPEC.md §lib/gate.sh` states the rule it is already cited for

Four sites in this SPEC attribute a rule to §lib/gate.sh that §lib/gate.sh does
not state — *exactly one place a knob's value is computed* — and each cites it
as settled: §The port-candidate criteria's criterion 6, §Consumer smoke *The
port disposition*'s leg 1, §Meta-gate conservation's `check-knob-default-coupling`
row, and §check-core-files' bridged-root-set paragraph {design-bearing}. The rule is
real and the citations are correct about its content; what is missing is the
sentence in its named owner, which is the content-tiering defect of a fact whose
only statements are its citations. §lib/gate.sh states it, and the four keep
citing rather than restating.

The statement is the bridge's own mechanism read as a rule: `gate_command`
resolves a compiled member's declared knob by sourcing the owning kit's
`lib/*.sh` — the owner derived from the knob's `<KIT>_` prefix — and hands the
binary the resolved value, so the value is computed in the kit's shell library
and nowhere else, and the crate holds no default to drift from. The crate's
readers already implement the strict form: an absent bridge variable is an error
rather than a fallback, and an empty one is a resolved-empty set.

### (2) A class ruling, `gate-sdk/SPEC.md §lib/gate.sh` gains *The port disposition*

A kit library that is the config bridge's sole resolver for its kit's knobs is
permanently shell, because a crate-side resolver would be the second producer
criterion 6 refuses — and the refusal is not a preference but a **deletion this
tree already paid for** {design-bearing}. When the bridge landed, the crate's
prune-dir default and the unit test holding it equal to the shell default were
removed rather than extended, and §The port-candidate criteria records that
departure with a warning that a later reader must not restore the parity test as
a missing piece. Porting a sole resolver would re-create exactly that
duplication.

**The ground has a stated precedent rather than a precedent by example.**
`drift-kit/lib/drift.sh` declares on it in full — "the config bridge's **sole
resolver** for the `DRIFT_KIT_*` knobs … so a crate-side resolver would be the
second producer criterion 6 refuses" — and says why it stated the ground rather
than citing one: "the class of kit `lib/*.sh` files has never been swept, and a
cohort inherits a stated reason where it cannot inherit a precedent-by-example."
This ruling is that sweep. `gate-sdk/lib/consumer-smoke.sh` is the second, under
the smoke class ruling's leg 1.

**The discriminator is content, not directory, and the mechanism says so.** The
bridge sources `<kit>/lib/*.sh` — a flat glob — whenever any compiled member
declares a knob that kit owns. Two consequences fall out and both are load-bearing:

- **A file one directory deeper is never sourced at all.** `lib/pub-lang/*.sh`
  sits outside the bridge entirely, so the ground cannot reach it whatever else
  is true of it.
- **A file directly under `lib/` rides the glob whether or not it resolves
  anything.** Bridge membership is by position; being *the resolver* is not. A
  member sourced into the resolution subshell that computes no knob contributes
  nothing to the bridge and is not held by this ground — deleting it would leave
  the bridge sourcing one file fewer and resolving exactly the same values.

So the ruling reaches the kit libraries that carry their kit's knob defaults,
each of which is the only place those values can be computed: the sourced
loaders of canon-kit, context-kit, delegation-kit, doctrine-kit, evidence-kit,
lifecycle-kit, queue-kit and site-kit, and gate-sdk's own. Two of them —
canon-kit's and site-kit's — have no shell sourcer left in the tree at all, so
the bridge is the whole of their live role, which is the ground in its purest
form.

**`gate-sdk/lib/gate.sh` is the same ground held twice over, and it is stated
separately because the stronger case is easy to under-read.** It is not a client
of the bridge; it *is* the bridge — the machinery that sources the other eight
and derives which kit owns a knob. A crate-side form would have to either source
shell libraries from inside a binary or re-implement every kit's defaults, which
is the second producer squared. §gen-pre-commit already declares on precisely
this ground, operator-ratified, from the opposite direction: the hook generator
bakes a resolved knob and so cannot move either.

**`guard-kit/lib/guard.sh` is decided here, and the ground that decides it is
not this one — which is the finding, not an evasion.** The natural reading is
that the largest kit library is the heaviest instance of the sole-resolver
ground. It is measurably not: guard-kit ships no `checks/` directory and no
registered member, so no compiled member ever declares a `GUARD_KIT_` knob and
the bridge never sources this file. The ground that does reach it is guard-kit's
own: §Consumer rules rules that a consumer's project block/steer/allow rules
live in its copy of `templates/bash-guard.sh`, **composed from the
`lib/guard.sh` primitives**. The library is the API a consumer's own shell rules
are written against, so porting it deletes the extension point — which is
`native-gate-port-remaining-corpus`' ruling (1), *a cut narrows the port, never
an extension point*, on a seam whose interface is a set of shell functions. Its
declaration therefore cites `guard-kit/SPEC.md §The guard framework (lib/guard.sh)`
and that section states the ground, exactly as the smoke class ruling has each
reached kit's own section state its own.

**Why deciding it here does not average two grounds into one cut.** The composer
ruling constrains how a cut is *selected* — the owed files behind one stated
contract — and this cut is selected behind §lib/gate.sh's. What put this one
further file inside the cut is an operator ruling on the promoting entry, which
records that guard.sh's port-versus-declare disposition "is DECIDED by this cut
rather than separately takeable". Deciding it on the ground that actually holds,
in the section that holds it, is what keeps the two grounds *un*-averaged; folding
it under the bridge ground would be the averaging all three entries forbid, and
leaving it undecided would fail the ruling that put it here.

**What reopens it**, on §Consumer smoke *The port disposition*'s terms: the
sole-resolver face dissolves for a kit whose knobs stop crossing the bridge —
which is what happened to guard-kit and is why it is decided elsewhere — and it
dissolves generally if §lib/gate.sh ever admits a second bridge producer. The
extension-point face dissolves if §Consumer rules stops composing a consumer's
rules from this library's primitives.

**The honest limit: the members this ruling leaves owed are left owed on
purpose, and one of them the ruling positively says must port.** `lib/pub-lang/`'s
shipped extractors are the **bundled members** of a consumer-first plug-in
registry, and ruling (1)'s own words are that a seam's resolution, direct
execution and env contract survive while *only the bundled members move
in-crate* — the disposition drift-kit's KPI plugins already took. They are owed,
not undecided. What the other four meet is on delta (5)'s entry.

### (3) Ten `# no-port:` header declarations

Each declaring member gains one `# no-port: <cause>` header line naming the
ruling that makes it permanent {mechanical}. The nine sole resolvers cite delta
(2)'s subsection; `guard-kit/lib/guard.sh` cites guard-kit's own section on the
pattern above. No file gains any other field, none carries `# port-until:`, and
no `.gate` descriptor is touched.

### (4) The reached surfaces record the disposition in one sentence each

Each owning kit's library section records its own member's disposition and
points at the ruling: `canon-kit/SPEC.md §lib/spec.sh`,
`context-kit/SPEC.md §lib/context.sh`, `doctrine-kit/SPEC.md §lib/doctrine.sh`,
`evidence-kit/SPEC.md §lib/evidence.sh`, `lifecycle-kit/SPEC.md §lib/stages.sh`,
`queue-kit/SPEC.md §lib/queue.sh`, `site-kit/SPEC.md §lib/site.sh`, and —
delegation-kit having no library section — its
`§Layout and configuration`, beside the layout line that names the file
{design-bearing}. `guard-kit/SPEC.md §The guard framework (lib/guard.sh)` states
its own ground in full rather than pointing, because that ground is stated
nowhere else.

### (5) The owed residue is filed as one deferred entry

The port work this ruling leaves standing is filed rather than absorbed, per the
unit set's own boundary {mechanical}. One deferred entry owns `lib/toolfloor.sh`,
the two `lib/pub-lang/` extractors, `lib/declaration.sh`, `lib/inject.sh` and
`lib/test-hermetic.sh`, and carries what each actually meets:

- **`lib/pub-lang/rust.sh` and `ts.sh`** — bundled plug-in members that ruling
  (1) sends in-crate; their port waits on the resolver that finds them,
  `bin/pub-index.sh`, which is itself owed.
- **`lib/toolfloor.sh`** — a probe roster read on the **installer** path and by
  `check-install-toolchain`'s parity assertion, so its port is sequenced behind
  the installer's own two blocked readings rather than by anything in this class.
- **`lib/declaration.sh`** — already dual-implemented against
  `native/src/declaration.rs` under a standing parity lane, with one live shell
  sourcer. Criterion 6's *unless* clause admits the duplication and its stated
  test is whether the shell caller set empties; it has not, so the disposition is
  temporary rather than permanent.
- **`lib/inject.sh`** — three shell sourcers, each itself owed.
- **`lib/test-hermetic.sh`** — deliberately not declared. It computes a second
  default for a bridged knob, `GATE_SDK_NATIVE_BIN`, and that default differs
  from `§lib/gate.sh`'s by omitting the executable-suffix helper, so on a Windows
  host a bespoke test pins the knob to a path that cannot exist. Declaring a file
  that holds a second producer of a bridged knob's default would bless the
  duplication this ruling rests on refusing. The defect is filed to the gap inbox
  separately; the disposition waits on it.

## Producers and consumers

The only new state is a **port-disposition declaration** on ten tracked shell
files, plus one **prose rule relocated into its named owner** (delta 1). No new
field, tag, knob, interface, or knob value.

- **Producer** — the build session's declaring commit writes one `# no-port:`
  header line per member. Enabling path: the file being tracked, and nothing
  else (§The `# graph:` manifest, "no registration step").
- **Consumer 1 — §port-blockers' `--tree` arm**, reclassifying each row
  `owed` → `no-port`; the reader TRAJECTORY.md's completion predicate is stated
  over.
- **Consumer 2 — §check-gate-substrate-parity assertion G**, once the sibling
  amendment `SPEC-port-declaration-shape.md` widens its corpus to the tracked
  shell tree. Until that lands no gate reads any of these ten causes.
- **Consumer 3 — §check-comment-tier**, whose directive roster already carries
  both spellings over the whole governed tree; no widening needed.
- **Consumer 4 — canon-kit's measured-claim oracle**, transitively through the
  `tree-shell-owed` key and the resolved values the generated hooks bake, which
  is why the regeneration below is an update target.
- **Delta 1's consumer** is the reader of any of the four sites that cite the
  rule; its transition is following that citation to §lib/gate.sh and finding the
  rule stated. The citations themselves are unchanged, which is the point — a
  correct citation needs a statement at its target, not a copy at its source.

**No behavior changes and that is checkable rather than asserted.** A
`# no-port:` line is a header comment: it alters no resolution path, no sourcing
order, and no knob's value, and every declaring member's callers see the file
they saw before. The bridge's own sourcing is a glob over `lib/*.sh` and is
indifferent to a comment.

**This delta widens no corpus and narrows none**, so §The causal-completeness
check point 5's red-condition enumeration does not bind: every reader sees more
declarations and fewer owed rows, and no reader over this corpus reds on
*finding none*, asserts an exact count, or holds a coverage floor. The one count
that moves, `tree-shell-owed`, is cited by no governed sentence today.

## Existing sections updated

- `gate-sdk/SPEC.md §lib/gate.sh` — states the one-place rule it is cited for,
  and gains the *The port disposition* subsection (deltas 1 and 2).
- `guard-kit/SPEC.md §The guard framework (lib/guard.sh)` — states its own
  member's ground in full and records the disposition (deltas 2, 3 and 4).
- `guard-kit/SPEC.md §Consumer rules` — records that the primitives a consumer
  composes its rules from are permanently shell, which is what makes the
  placement contract stated there durable (delta 2).
- `canon-kit/SPEC.md §lib/spec.sh`, `context-kit/SPEC.md §lib/context.sh`,
  `doctrine-kit/SPEC.md §lib/doctrine.sh`, `evidence-kit/SPEC.md §lib/evidence.sh`,
  `lifecycle-kit/SPEC.md §lib/stages.sh`, `queue-kit/SPEC.md §lib/queue.sh`,
  `site-kit/SPEC.md §lib/site.sh`, `delegation-kit/SPEC.md §Layout and configuration`
  — each records its own member's disposition in one sentence (delta 4).
- `drift-kit/SPEC.md §lib/drift.sh` — records that its already-landed declaration
  is the class's stated precedent rather than a lone case (delta 2).
- `gate-sdk/SPEC.md §lib/declaration.sh`, `§lib/inject.sh`,
  `§lib/test-hermetic.sh` and `context-kit/SPEC.md §Layout and configuration`
  (for `lib/toolfloor.sh` and the `lib/pub-lang/` roster) — each records that its
  member is owed and names the entry that owns the work, so a later reader does
  not read the silence as an undecided class (delta 5).
- `TASK-QUEUE.md`, one new deferred entry owning the owed residue (delta 5).
- The generated projections this cut stales: the on-site SPEC mirror, and the
  generated `pre-commit`/`commit-msg` hooks together with `docs/check-graph.html`,
  which `docs/site-architecture.md` §Generated projections names as staling when
  "a script header gaining a `# no-port:` cause moves the `tree-shell-owed` key"
  (all deltas).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`), discharged at the iteration rather
      than at the commit, this iteration carrying sibling amendments.
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **The oracle moved by the roster, not by the trailer** — each declaring
      member reports `no-port` on `--tree` and every owed member of the class is
      unchanged, read as a per-file diff.
- [ ] **No knob's resolved value moved** — the generated hooks' baked
      `GATE_SDK_KNOB_*` assignments differ only where the `tree-shell-owed` key
      does, which is the one value a declaration is allowed to move.
- [ ] **The regeneration fan-out is discharged in the landing commit** — the
      generated hooks and the graph artifact.
