# SPEC amendment: canon-cohort

The third gate cohort of the native port: **canon-kit's `spec_manifest_files`
family, ten gates**, ruled by the operator 2026-08-12 under
TRAJECTORY.md §PRIORITY DIRECTIVE — the port track's sequence. This amendment
designs that port. It does not restate the porting procedure
(§Porting a gate to the binary substrate), the conservation contract
(§Meta-gate conservation for the binary substrate), the criteria roster
(§The port-candidate criteria), or the directive's grounds.

**Ten and not eleven, and the arithmetic is a re-measurement rather than a
narrowing of the ruling.** What the operator ruled is the **selector** — the
`spec_manifest_files` family, chosen by §The first cohort's own rule as the
largest set of criteria-clearing gates sharing one corpus derivation. Eleven
scripts call that derivation, but `check-spec-pointer` **fails criterion 6**
(delta 10), so it was never inside the set that selector defines; the eleven was
scope's count of callers, not a count of the ruled cohort. Correcting it is the
same class of correction as the three below, and it is **held, not excluded** —
the sequencing §The port-candidate criteria prescribes and the queue-kit cohort's
own two holds already set as precedent. Operator-ruled 2026-08-12 at spec.

## Three premises this cohort was ruled on are wrong, and each changes the work

The cohort was selected on a scope-session survey and a queue entry. Both are
current as committed; three of their load-bearing claims do not survive a probe,
and correcting them here is what stops the build session from meeting each one at
implementation time — the failure mode criterion 7 exists to prevent, which the
queue-kit cohort then hit anyway through `check-roadmap-fresh`.

**(i) The cohort owes ONE shared mechanism, not two.** The queue entry states it
*"owes two Rust mechanisms `native/src` lacks — basename-glob matching beside
walk.rs's extension filter, and `gate_kit_roots`"*. The first is **already
built**. `native/src/walk.rs:167-179`'s `glob_files` is not a full-path matcher:
it splits each glob on `/` and matches component-wise through
`match_component`/`glob_here`/`bracket` (walk.rs:70-158), supporting `*`, `?`,
bracket classes with ranges, bash-faithful leading-dot handling, and **globstar**
(walk.rs:189-195). It is in production, called by
`native/src/gates/queue_slug_liveness.rs`. §The first cohort already records this
correctly — *"`check-queue-slug-liveness` needed the first and it is now built…
That cohort's remaining shared debt is the second alone"* — so the owner doc was
right and the queue entry is the stale copy. **The one owed mechanism is a Rust
`gate_kit_roots`**; a recursive grep for kit-root logic under `native/src`
returns nothing.

**(ii) Five of the eleven callers are substrate-sensitive, not none.** The survey
records *"all 11 clear criteria 4 and 7"*. Criterion 4 is the half that fails.
Assertion C's derivation makes a member substrate-sensitive when its expanded
`couples=` covers a **registry member's declaration path**, and thirteen gates
resolve under `scripts/`, so a `couples=scripts/*.sh` reaches declaration paths
just as `kit:*.sh` does. Read off the manifests: `check-spec-pointer`,
`check-docs-cmd`, `check-prose-enum` (all three carry both spellings),
`check-install-claim` and `check-payload-claim` are sensitive;
`check-manifest-count`, `check-manifest-temporal`, `check-knob-citation`,
`check-spec-fence-balance`, `check-tracking-claim` and `check-md-refs` are not.
One of the five — `check-spec-pointer` — is held by delta (10), so the gate
reports 30 dispositioned sensitive members at HEAD and **this port hands it four
more**.

**(iii) Three members shell out to a consumer-supplied command, which no
selection input mentions.** `check-install-claim` and `check-payload-claim` reach
`bash -c "$cmd"` through `spec_claim_vocabulary` (`canon-kit/lib/spec.sh:464`),
and `check-prose-enum` through `spec_enum_sets` (`spec.sh:448`). This is
**not** a criterion-7 blocker — `bash` is on `GATE_SDK_PROGRAM_FLOOR`
(`gate-sdk/lib/gate.sh:55-57`) — and saying so is the point: the mechanical screen
passes it, and the problem is elsewhere. A compiled gate that spawns `bash` to run
a consumer's shell command carries the interpreter back into the binary,
against TRAJECTORY.md §The objectives' sixth. Delta (7) is the answer; it is
named here rather than discovered at build.

**What the criteria do clear, verified rather than assumed.** All eleven callers
are registered in `scripts/gates.list` (criterion 1), all carry a `good/`+`bad/`
fixture pair with no `# no-fixture:` member among them (criterion 2), and all
declare `tier=precommit` (criterion 3) — the held member included, which is what
makes its hold a criterion-6 verdict rather than a general unfitness. So a green
`check-graph` after the port is end-to-end proof the manifest survived the
substrate change.

## The corpus derivation, and the two branches this repo never executes

The economy of a cohort is that its shared derivation ports once and is proved N
times. `spec_manifest_files` (`canon-kit/lib/spec.sh:208-230`) is that
derivation, and it has three branches:

| branch | condition | mechanism | Rust status |
|---|---|---|---|
| explicit globs | `CANON_KIT_MANIFEST_FILES` non-empty | `nullglob globstar` expansion, no subprocess | `walk::glob_files` — **built** |
| default walk | that knob empty | `spec_canonical_specs` + a `README.md` find + a `CLAUDE.md` find, each `gate_find`, the first two `/templates/`-filtered and kit-root pruned | needs `gate_kit_roots` — **owed** |
| prose surfaces | `CANON_KIT_PROSE_SURFACE_GLOBS` non-empty | glob expansion filtered by `_spec_slot_free` | `glob_files` + a fixed matcher — **built plus small work** |

**The sharpest consequence, and it is the one this cohort must design around:
this repo's own configuration executes neither of the two branches that carry the
owed mechanism.** `scripts/canon-config.sh` sets `CANON_KIT_MANIFEST_FILES` to
fifteen explicit globs (line 22) **and** `CANON_KIT_SCAN_KIT_ROOTS=1` (line 14).
The first routes every live invocation down the explicit-glob branch, so the
default walk never runs here. The second makes `_spec_prune_kit_roots` return at
its first line (`spec.sh:168`), so kit-root pruning is a no-op here even if the
walk did run. **A parity comparison over this repo's live tree therefore proves
nothing whatever about the one mechanism the cohort owes.**

That is the queue-kit cohort's lesson arriving one door over. §The first cohort
records it: *"A cohort is sized off what its members execute, never off what their
fixtures reach"* — there, a fixture pair steered the assertion off the live
emitter and a ported member's pair would have gone green over an arm with no
implementation. Here it is the **live tree** that steers off the arm, which is
worse, because the live tree is the oracle a session trusts most.

**`_spec_slot_free` is not the ERE problem it resembles.** It greps
`'\*<[a-z][a-z0-9-]*:|^CONSUMER BINDING'` (`spec.sh:205`). That pattern is a
**kit literal**, not consumer config, so the port hand-writes those two shapes and
owes no regex engine. This is exactly the distinction that made
`check-queue-prose-precondition` a held member: there the knob is consumer config
carrying an arbitrary ERE the gate *interprets*. Recorded so nobody sizes this
cohort against that one.

**A fidelity landmine inside the default branch.** Lines 217-219 are asymmetric:
`spec_canonical_specs` and the `README.md` find are both `/templates/`-filtered
and kit-root pruned, and the **`CLAUDE.md` find is neither** (`spec.sh:219`). A
port that regularizes the three into one loop changes the corpus. The asymmetry is
reproduced, not tidied.

## What changes

### The shared substrate

**(1) A Rust `gate_kit_roots`, inside the sanctioned walk implementation.**
[design-bearing] The shell function (`gate-sdk/lib/gate.sh:237-254`) returns the
`GATE_SDK_KIT_DIRS` override when set, else the gate-sdk root plus every sibling
directory holding a `checks/` or `smoke/` — that pair being the predicate that
makes a directory a kit. The Rust form takes the override across the existing
config bridge as an ordinary bridged knob, which discharges criterion 6 in its
strongest form (§The port-candidate criteria: for a bridged knob *"the duplication
is not machine-held, it is absent"*), and derives the fallback by the same
predicate. It lives in `walk.rs` beside `find_files`/`glob_files`, not in a gate
module, because it enumerates directories and unit test B reds a filesystem-walk
API named outside that module (`walk.rs:381-407`).

**(2) `_spec_prune_kit_roots`'s path-prefix semantics port with it.**
[design-bearing] Distinct work from (1) and easy to fold in by mistake:
`prune_dirs` (`walk.rs:10-20`) excludes by bare **directory name**, while this
excludes a file whose **absolute path falls under a discovered kit root** — and
only under a root *below the scan root*, since `spec.sh:183` skips a root that is
not a strict descendant, which is what keeps a kit's own fixture dir from pruning
itself. Its `CANON_KIT_SCAN_KIT_ROOTS` short-circuit is a bridged scalar.

**(3) The manifest-set derivation ports once, as a shared crate module.**
[design-bearing] All three branches, in one place the ten members call — the
same "ported once and proved N times" economy §The first cohort names, and the
same shape `native/src/queue.rs` already has for queue-kit. Porting it per gate is
the failure mode to refuse. The knob reads (`CANON_KIT_MANIFEST_FILES`,
`CANON_KIT_PROSE_SURFACE_GLOBS`, `CANON_KIT_SPEC_NAME`, `CANON_KIT_SCAN_KIT_ROOTS`,
`GATE_SDK_KIT_DIRS`) are declared in each member's registry tuple, so a knob the
bridge was never asked to carry cannot be read.

**(4) An edge-tree parity scenario for the branches this tree does not run.**
[design-bearing] Because of the configuration finding above, the acceptance oracle
for deltas (1)-(3) is **not** the live tree and **not** the stock fixture pairs.
The port stands up a throwaway tree with `CANON_KIT_MANIFEST_FILES` unset and
`CANON_KIT_SCAN_KIT_ROOTS` unset, and runs both implementations over it comparing
bytes and exit codes.

**Its composition is specified here rather than left to the build session**, because
this scenario is the sole oracle for the asymmetry above, and a tree that merely
"has a `templates/` skeleton" can pass while proving nothing about it. The three
finds differ across **two independent filters**, so the tree carries all six cells
— each of `SPEC.md` (whatever `CANON_KIT_SPEC_NAME` names), `README.md` and
`CLAUDE.md`, once under a `templates/` directory and once at the root of a vendored
kit:

| file | under `templates/` | at a vendored kit root |
|---|---|---|
| `SPEC.md` | filtered out | pruned out |
| `README.md` | filtered out | pruned out |
| `CLAUDE.md` | **kept** | **kept** |

**The two `CLAUDE.md` cells are load-bearing and are the easiest to omit.** They are
the only cells that distinguish the real asymmetry from a regularized
three-finds-in-one-loop implementation — which would drop both, and still pass a
tree built from the first two rows alone. That is exactly the regression the
fidelity landmine above names, so a scenario missing them leaves this amendment's
one prohibition ("the asymmetry is reproduced, not tidied") with no oracle behind
it. Plus the **ancestor-root case**: the scan root sitting *inside* a kit root,
which `spec.sh:183` requires never to prune.

The verdicts in the table are the shell's current behavior, written down to make
the cells legible; the oracle itself stays **differential**, so a divergence
surfaces as a byte difference between the two implementations rather than as a
hand-maintained expectation that could itself go stale.

This is the constructed-scenario form criterion 2 already sanctions for a
member whose state has no static representation, applied here to a *branch* rather
than a member. Without it the port ships a `gate_kit_roots` that nothing ever
executed.

### The ten members

**(5) Port the six criterion-4-clear members.** [design-bearing]
`check-manifest-count`, `check-manifest-temporal`, `check-knob-citation`,
`check-spec-fence-balance`, `check-tracking-claim`, `check-md-refs` — each becomes
a module under `native/src/gates/` and a 4-tuple in `REGISTRY`, its `.sh` deleted
and its `.gate` descriptor landed in one motion. Design-bearing: each rule is
re-implemented, and byte-identical parity over its fixture pair, the live tree and
the edge tree is the acceptance oracle. Two carry named sub-problems:

- **`check-spec-fence-balance` is criterion 6's own counter-example**
  (§The port-candidate criteria names it as the gate that made the criterion
  necessary, and records that `check-action-pinning` was selected instead). It is
  in this cohort, and the criterion is now **discharged by construction** rather
  than waived: its derivation is `spec_manifest_files`, whose knobs cross the
  bridge, so the value is computed in exactly one place. Stated so a later reader
  does not re-litigate a criterion that has since been answered.
- **`check-knob-citation` calls `gate_kit_roots` directly**, not only through the
  manifest derivation, so delta (1) has a second consumer inside the cohort and its
  Rust form is a shared function rather than a private helper.

**(6) `check-tracking-claim` and `check-md-refs` spawn git, and the spawn
discipline is not theirs to invent.** [design-bearing] Four callers spawn git —
these two plus `check-docs-cmd`, and `check-spec-pointer`, which delta (10) holds;
so **three ported members spawn git**. The crate's existing spawn site folds a
failed spawn and a non-zero exit into one `None`
(`native/src/gates/task_conservation.rs:10-16`), which is the live instance behind
the `gate-subprocess-fail-closed-unheld` debt riding this iteration. That entry
owns the mechanism; this delta owns the dependency: **the three ported members call
whatever the debt member lands**, and they port after it. Naming the ordering here
is what keeps three new spawn sites from being written against the defective shape
first and repaired second — and the held member is a fourth that will need it, so
the debt member's shape must not be sized to this cohort alone.

### The four substrate-sensitive members

**(7) The consumer-command spawn is redesigned, not transliterated.**
[design-bearing] `check-install-claim`, `check-payload-claim` and
`check-prose-enum` obtain their vocabulary by running a consumer-configured shell
command. Transliterating that into the crate makes the binary spawn `bash`, which
is the interpreter surface objective 6 shrinks. The design is the amendment's, and
the shape it should take is the one this repo already uses on the other side of
the seam: the command's **output** is the interface, not the command. So the
vocabulary crosses the existing config bridge as a resolved value — the kit's
shell library runs the consumer's emitter exactly as it already resolves every
other knob (`_gate_knob_value`, `gate-sdk/lib/gate.sh:110-142`), and the binary
receives data it does not have to spawn anything to get. That keeps the provenance
seam intact — the vocabulary stays consumer config and never becomes a kit literal
— and removes a `bash` dependency rather than compiling one in. The bridge's own
constraint binds and must be checked at build: it refuses an element containing a
tab or newline (`gate.sh:126-138`), which is a real bound on what an emitter may
emit and is the first thing to verify against `scripts/enum-sets.sh`,
`scripts/install-transports.sh` and `scripts/payload-claims.sh`.

**(8) `check-prose-enum` is the criterion-4 hazard in its real form, and it ports
last.** [design-bearing] Three of the four ported sensitive members are sensitive
only as **reverse triggers** — the conservation table records that their scanned
corpus is the governed-doc set and none reads a gate script's content
(§Meta-gate conservation for the binary substrate). `check-prose-enum` is the
exception the table itself already carved out: its enum derivation reads the queue
tag vocabulary out of another gate's own class table, so *"deleting that gate's
script broke the derivation"* once already. Porting it makes a gate whose input is
a gate's content into a gate that is itself gate content. It ports after deltas
(5)-(7), and its parity run must cover the case where the set it reads comes from
a ported member.

**(9) The conservation table is re-derived against the new sensitive set.**
[mechanical] Four new descriptors change which members assertion C's runtime
derivation selects. Each of the four already has a table row — `check-docs-cmd`,
`check-install-claim` and `check-payload-claim` under the reverse-trigger row and
`check-prose-enum` under its own — and each must now read correctly as describing
a `.gate`-declared gate rather than a shell script. `check-spec-pointer`'s row is
**not** touched by this delta, because delta (10) holds the member on shell; it is
touched by delta (10) instead, which records why.

**(10) `check-spec-pointer` is HELD on shell — criterion 6, sequencing not
exclusion.** [design-bearing] **Operator-ruled 2026-08-12 at spec.** The library
carries two corpus primitives: `spec_manifest_files`, which this cohort ports, and
`spec_comment_surface` (`spec.sh:264/266`), whose callers are `check-comment-tier`,
`check-deprecation-task`, `check-todo-task-liveness` — **and
`check-spec-pointer`**, the only member calling both. Porting it would require the
comment-surface derivation in Rust while three shell callers kept the shell one: a
duplication across substrates with **nothing holding the copies together**, since
this primitive's inputs are not all bridged knobs. That is criterion 6's red
condition rather than its discharge, and it is the same shape criterion 6 was
originally written against.

**The member was therefore never inside the ruled selector**, which is why the
cohort is ten. The ground is the one §The port-candidate criteria fixes in its
opening sentence and TRAJECTORY.md restates: a criterion a member fails **orders**
the work, and citing this hold as an eligibility screen inverts the rule. The
2026-08-09 directive ports the whole corpus; this member ports later, not never.

**What it owes, recorded so a later cohort inherits it rather than re-deriving
it.** `check-spec-pointer` ports with the **`spec_comment_surface` family taken
whole** — itself plus `check-comment-tier`, `check-deprecation-task` and
`check-todo-task-liveness` — so the second primitive ports once and is proved four
times, which is the same shared-derivation economy §The first cohort applies to
this cohort's own. Porting it alone re-imports work that cohort would pay once,
and porting it with the manifest family strands three shell callers against a
Rust twin. The hold is also filed on `cohort-held-members-port-prerequisites`,
beside the two queue-kit holds, because **a hold nobody can find later is an
exclusion**.

**One consequence to carry forward:** the conservation table's
`check-spec-pointer` row states that its corpus depends on the shared
comment-surface widening. That row is **unchanged** by this port and must be left
alone — the member stays a shell gate, and editing its row to describe a
`.gate`-declared gate would make the table describe a port that did not happen.

### Standing obligations

**(11) Parity is proved while both implementations exist.** [mechanical]
Assertion A forbids a `<name>.sh` and a `<name>.gate` in one resolve dir, so each
comparison runs with the shell gate still in place and the descriptor staged
elsewhere; the descriptor lands and the script is deleted in one motion. Per
member: the fixture pair, the live tree, and delta (4)'s edge tree.

**(12) The commit-time obligations this repo already carries.** [mechanical]
`bash gate-sdk/bin/build-native.sh` before each commit, plus the regenerated
projections per docs/site-architecture.md §Generated projections. Ten
descriptors and ten deleted scripts move the graph artifact, the enforcement
map, the footprint and value rollups and the docs mirror.

**(13) Two deferred entries advance toward closure.** [mechanical]
`gate-battery-parallel-execution` and `gate-battery-result-cache` each say the
port subsumes them. They are dispositioned at close, not built.

## Producers and consumers

**New interface: ten `.gate` descriptors under `canon-kit/checks/`.**
- *Producer* — this port; the descriptor's existence **is** the dispatch
  declaration (§The `.gate` descriptor).
- *Consumers* — `gate_resolve` (declaration path) and `gate_command` (argv),
  `gate-sdk/lib/gate.sh`. Enabling config is already emitted everywhere it must
  be: `GATE_SDK_NATIVE_BIN` carries a kit default, so no consumer sets anything
  for dispatch to resolve.
- *Every field has a named reader* — `# graph:` by `check-graph`,
  `gen-pre-commit.sh`, `enforcement-map.sh` and `footprint.sh`; `# spec:` by
  `check-spec-pointer`; `# install:` by `check-install-disposition`. No field is
  added to the closed roster, and no member takes `# no-fixture:` — all ten
  carry pairs.

**New interface: ten `REGISTRY` tuples in `native/src/gates/mod.rs`.**
- *Producer* — the crate at compile time; a member added without its declared read
  roots and knob reads fails to compile.
- *Consumers* — `lookup` (dispatch), `roots` (`--reads`, consumed by
  `check-reads-couples`), `knobs` (`--knobs`, consumed by `gate_command`'s bridge
  loop), `names` (`--list`, consumed by `check-gate-substrate-parity` assertion B).
- *Field readers at named transitions* — the knob-reads element is read by
  `gate_command` at argv construction, each name becoming one
  `GATE_SDK_KNOB_<NAME>=<tab-joined>` element; the read-roots element is read by
  `--reads` and held to executed behavior by unit test A at `cargo test`.

**New capability: a Rust `gate_kit_roots` and kit-root path pruning (deltas 1-2).**
- *Producer* — the manifest-set derivation, and `check-knob-citation`'s direct
  call.
- *Consumer* — the walk recorder, which must observe the roots this enumeration
  visits or unit test A's assertion over these members is vacuous. Unit test B
  binds: it lives inside the crate's single sanctioned walk implementation.
- *Enabling config actually emitted* — `GATE_SDK_KIT_DIRS` and
  `CANON_KIT_SCAN_KIT_ROOTS` cross the existing bridge. **The honest gap**: this
  repo sets the latter to `1`, which disables the mechanism, so the producer is
  reachable *in this tree only through delta (4)'s edge tree*. That is precisely
  point 1 of §The causal-completeness check — a named producer whose enabling
  config no deployed configuration sets is dead everywhere but tests — and the
  edge tree is what keeps it from being dead here.

**New interface: the bridged claim vocabularies (delta 7).**
- *Producer* — the kit's shell library, running the consumer's
  `CANON_KIT_ENUM_SETS_CMD` / `CANON_KIT_INSTALL_TRANSPORTS_CMD` /
  `CANON_KIT_PAYLOAD_CLAIMS_CMD` at knob-resolution time.
- *Consumer* — the three ported members, reading a resolved
  `GATE_SDK_KNOB_*` value; no `bash` spawn on the binary side.
- *Named reader at a named transition* — read once per invocation, when the member
  builds its vocabulary set, before its first corpus line is scanned.

**Red conditions of the readers this change touches** (§The causal-completeness
check, point 5 — the port **narrows** a corpus by deleting ten `.sh` files, and
"a narrower corpus can only remove violations" is the first argument this delta
reaches for and is false):

- `check-gate-output` — red on a **zero count** of a `: clean` / `help:` line.
  Non-monotone. All ten are fixtured, so each falls to the runtime arm rather
  than the source-grep, and the `check-task-conservation` branch is untouched.
- `check-gate-fixture-coverage` — red on a member with **neither** a pair **nor**
  an opt-out: a zero-count reader, cleared because every pair moves with its member.
- `check-gate-substrate-parity` assertion B — red on a descriptor with no
  subcommand **and** on a subcommand with no descriptor; a half-landed member reds
  either way. Assertion C — red on a substrate-sensitive member with **no
  disposition line**, a zero-count reader over a set this port **widens** by five;
  delta (9) is the discharge.
- `check-reads-couples` — red on a walk outside the declared couples; its shell
  parser finds nothing in a binary gate, so the `--reads` consumption is what keeps
  it from printing `clean` vacuously over ten members.
- `check-readme-roster` — red in **both** directions, so canon-kit's README moves
  from ten `.sh` names to ten `.gate` names rather than gaining them.
- `check-comment-tier`, `check-spec-pointer`, `check-todo-task-liveness`,
  `check-deprecation-task` — each loses ten shell files and gains ten descriptors
  plus ten Rust modules. The `*.gate` and `*.rs` arms already exist on the shared
  primitive (canon-kit/SPEC.md §lib/spec.sh), so these are monotone here and
  clearable by inspection. All four **stay shell** under delta (10)'s hold, which
  is what keeps this bullet a plain corpus shift rather than a member-level
  question: the comment-surface family is untouched by this port.
- `check-shellcheck` — loses ten files from its corpus; monotone, and the
  substrate equivalent is `cargo clippy`, which `check-crate-arms` brings to commit
  time this same iteration.
- `check-knob-default-coupling` — **deliberately not extended**, and it is a
  cohort-adjacent reader: it derives its knob prefixes from `gate_kit_roots`
  members and `native/` is not a kit root. Delta (1) gives the crate a
  `gate_kit_roots` but not a kit root, so this gate's verdict is unchanged. Stated
  because the coincidence of names invites the wrong edit.
- `check-gate-binary-fresh` — red on a stale binary once a member dispatches;
  already armed, and ten more dispatches do not change its predicate.
- `check-docs-cmd` — red on a doc fencing a path that no longer runs: real signal
  after ten `.sh` deletions, and the reason the docs mirror is in delta (12).
  It is also a cohort member, so it audits the consequences of its own port.
- `check-prose-enum` — red when a paragraph names ≥2 members of a declared set and
  omits one; any prose enumerating canon-kit's gate roster must move with the port.
  It is delta (8)'s member, so this reader and that delta are the same gate.

## Existing sections updated

- **gate-sdk/SPEC.md §The first cohort, and the rule that selects the next** —
  owned by the premise section and deltas (1)-(4). Its closing paragraph records
  Cohort A as owing one remaining shared mechanism; that mechanism is built here,
  so the paragraph is rewritten to record the debt discharged rather than left
  claiming it outstanding. Gains the by-corpus/by-kit reconciliation for this
  cohort — the operator ruled by *family*, which is the corpus axis the ordering
  rule names, so the two selectors agree here without the by-kit caveat the queue
  cohort needed. **Also gains the `check-spec-pointer` hold**, written in the same
  form the section already uses for the two queue-kit holds: the member, the
  criterion it fails, and the work it owes. That subsection is the canonical home
  for a held member, so this is where the hold survives the amendment's deletion.
- **gate-sdk/SPEC.md §The port-candidate criteria** — owned by premise (ii),
  premise (iii) and delta (5). Criterion 4 gains the reverse-trigger distinction:
  a member sensitive only because its `couples=` re-triggers it is not the
  self-referential-parity hazard the criterion is about, and conflating the two
  over-counts the blocked set. Criterion 6's `check-spec-fence-balance`
  counter-example is annotated as discharged by the bridge. Criterion 7 gains the
  consumer-command case — floor-clear yet objective-6-adverse — as a second worked
  example beside `check-action-run-shell`.
- **gate-sdk/SPEC.md §Meta-gate conservation for the binary substrate** — owned by
  delta (9): four rows re-read as describing `.gate`-declared gates. The
  `check-spec-pointer` row is **left unchanged** by delta (10), and the list says so
  rather than omitting it, because an untouched row and an overlooked row look
  identical in a diff.
- **gate-sdk/SPEC.md §lib/gate.sh** — owned by deltas (1) and (7): `gate_kit_roots`
  gains a binary-side equivalent, and the knob bridge gains the claim vocabularies
  as resolved values, with its tab/newline constraint stated as a bound on what a
  consumer emitter may emit.
- **canon-kit/SPEC.md §lib/spec.sh** — owned by deltas (2), (3) and (10): the
  manifest-set finder's three branches and the kit-root prune now have a Rust
  implementation, and the section states which of the two corpus primitives is
  which substrate — `spec_manifest_files` ported, `spec_comment_surface` still
  shell with all four of its callers. The `CLAUDE.md` asymmetry is written down
  rather than left implicit in the code.
- **canon-kit/SPEC.md, the ten per-gate contract sections** — each whose prose
  says the gate is a shell script or cites its `.sh` path; and
  **§check-prose-enum**, which owns the reads-a-gate's-content reasoning delta (8)
  extends.
- **canon-kit/README.md** — gate roster, ten names, `.sh` → `.gate`.
- **native/src/gates/mod.rs** — the registry, and the module-per-gate comment whose
  member count changes.
- **TASK-QUEUE.md `native-gate-port-remaining-corpus`** — owned by premise (i):
  the "two Rust mechanisms" line is one mechanism; and by the header ruling: the
  cohort is ten, `check-spec-pointer` held.
- **TASK-QUEUE.md `cohort-held-members-port-prerequisites`** — owned by delta (10):
  the entry that exists because *"what no queue entry carries is the work owed"*
  gains a third held member beside the two queue-kit ones. Landed at spec rather
  than deferred to the merge, since a hold recorded only in an amendment vanishes
  when the amendment is deleted.

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
