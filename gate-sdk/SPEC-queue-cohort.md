# SPEC amendment: queue-cohort

The first **real** gate cohort of the native port: **queue-kit, taken as a whole
kit** — all ten of its registered gates — ruled by the operator 2026-08-11 under
TRAJECTORY.md §PRIORITY DIRECTIVE. This amendment designs that port. It does not
restate the porting procedure (§Porting a gate to the binary substrate), the
conservation contract (§Meta-gate conservation for the binary substrate), or the
directive's grounds.

## The cohort, and why by-kit does not contradict the ordering rule

§The first cohort, and the rule that selects the next sets the ordering axis as
*"the largest set of criteria-clearing gates sharing one corpus derivation"* and
warns in the same paragraph that *"selecting by kit, by profile, or by whatever
is easiest next all re-import work this cohort only paid once."* The operator's
ruling selects by kit. **The two select the same set here**, and recording why is
what keeps the ordering rule usable rather than quietly overridden: queue-kit's
gates are the kit whose corpus derivation *is* the queue file.

Read off the ten `# graph:` manifests directly: **eight couple `TASK-QUEUE.md`
alone**; `check-queue-slug-liveness` couples
`TASK-QUEUE.md,ROADMAP.md,scripts/queue-config.sh` and carries an explicit
`trigger=` field; `check-roadmap-fresh` couples `TASK-QUEUE.md,scripts/*.sh`,
where the glob is a **reverse trigger** — the conservation table already records
that shape for this family (*"names `scripts/*.sh` in `couples=` only so that a
script change re-runs it"*). All ten are `dir=one valve=none tier=precommit`, so
**criterion 3 is clear across the cohort** and a green `check-graph` after the
port is end-to-end proof the manifest survived the substrate change.

The kit boundary and the corpus boundary therefore coincide, so the by-kit ruling
buys the by-corpus economy rather than trading it away. A later selector must not
read this as licence to take a kit whose gates share nothing.

*Correction, recorded because a carried block says otherwise.* The scope survey
attributes `ROADMAP.md,scripts/queue-config.sh` to `check-roadmap-fresh`. Probed
at HEAD, that couples line is **`check-queue-slug-liveness`'s**. These two are
also the cohort's odd members on the axes below, which is exactly how the swap
survives a reading — a later session citing that block takes this paragraph over
it.

## The two independent nine-of-ten splits

The figure carried into this iteration — **9 of queue-kit's 10 gates are
walk-free, only `check-queue-slug-liveness` walks** — is correct and is the
*walk* axis. Probed first-hand at HEAD, there is a **second, orthogonal
nine-of-ten**, and it lands on a **different member**:

| axis | the nine | the one |
|---|---|---|
| corpus shape (criterion 6) | walk-free, fixed-path reads | `check-queue-slug-liveness` walks `QUEUE_KIT_PROSE_SURFACE_GLOBS` (queue-kit/checks/check-queue-slug-liveness.sh:21) |
| fixture pair (criterion 2) | `good/`+`bad/` under `queue-kit/gate-tests/` | `check-task-conservation` — the kit's sole `# no-fixture:` member (queue-kit/checks/check-task-conservation.sh:4) |

Evidence: `find queue-kit/gate-tests -maxdepth 2 -type d -name good -o -name bad`
returns pairs for exactly nine names, and `grep -rn '^# no-fixture:'
queue-kit/checks/` returns exactly one line. All ten are registered in
`scripts/gates.list` (lines 55-64), so criterion 1 is clear across the cohort.

**The consequence is the cohort's real shape: eight cheap, two designed.** Eight
members clear every criterion the existing substrate answers and port on the
bridge alone. Two carry one named engineering problem each, they are *different*
problems, and neither is discharged by the other's work. Reading "9 of 10" on one
axis as the cohort's difficulty is the mis-sizing this section exists to prevent.

## What changes

### The cheap eight

**(1) Port the eight criteria-clearing members.** [design-bearing]
`check-queue-entry-budget`, `check-queue-hygiene`, `check-queue-prose-precondition`,
`check-queue-sections`, `check-queue-wrap`, `check-roadmap-fresh`,
`check-tag-lead-line`, `check-task-names` — each becomes a module under
`native/src/gates/` and a 4-tuple in `REGISTRY`, its `.sh` deleted and its
`.gate` descriptor landed in one motion. Design-bearing rather than mechanical:
each rule is re-implemented, and byte-identical parity over its fixture pair is
the acceptance oracle, not a formality.

**(2) The shared queue library ports once — and it carries more than helpers.**
[design-bearing] `queue-kit/lib/queue.sh` is 193 lines and the cohort's members
source it. Three distinct things live in it, and only the first is what "shared
library" suggests:

- **Four helpers** — `queue_alt` (:68), `queue_live_slugs` (:91),
  `queue_done_slugs` (:102), `queue_roadmap_entries` (:116). Re-implemented
  **once**, as a Rust module the cohort's gates share — the "ported once and
  proved N times" economy §The first cohort names. Porting them per-gate is the
  failure mode to refuse.
- **Derived globals** — `QUEUE_ACTIVE_RE`, `QUEUE_DEFERRED_RE`,
  `QUEUE_ICEBOX_RE`, `QUEUE_DONE_RE`, `QUEUE_TASK_SECTIONS` (:71-82), computed
  from the knobs at source time. These are **not knobs**, so the bridge does not
  carry them; the Rust side derives them from the bridged knob values by the same
  rules. Naming them matters because a port that bridges only the knobs and looks
  for the regexes will not find them declared anywhere.
- **A fail-closed config validation** (:164-192) that `exit 2`s with *"queue-kit:
  malformed queue config — the gates cannot run"*, covering emptiness, integer
  shape, an icebox/deferred name collision, and a **cross-knob** rule that
  `QUEUE_KIT_HORIZONS` and `QUEUE_KIT_TRACKS` are configured as a pair.

**The validation survives the port at no cost, and the reason is worth
recording.** `_gate_knob_value` resolves each declared knob by **sourcing the
owning kit's `lib/*.sh` in a subshell** (gate-sdk/lib/gate.sh:104-113), so
`queue.sh` is sourced at dispatch and its validation block runs there. Its
`exit 2` propagates out of the subshell, through `_gate_knob_value`'s non-zero
return, into `gate_command`'s `|| exit 2` — the §Fail-closed contract, preserved
without re-implementing a line of it. This is criterion 6's *"the duplication is
not machine-held, it is absent"* argument extending from knob **defaults** to
knob **validation**, which is what makes a thirteen-knob cohort tractable at all.

**Its honest limit, stated rather than discovered.** That only holds for a member
declaring **at least one** knob; a member declaring none never causes the library
to be sourced and would silently skip validation. Every one of the ten reads at
least `QUEUE_KIT_QUEUE_FILE`, so the limit is not live in this cohort — but it is
a property of the bridge rather than of this cohort, and the next port meets it
again.

**(3) Knob bridge, no new substrate — and the count is larger than carried.**
[design-bearing] Probed at HEAD (`grep -ohE 'QUEUE_KIT_[A-Z_]+' queue-kit/checks/*.sh`),
the cohort's gates name **thirteen** knobs, not the eight the scope survey lists:

- **Seven scalars** — `QUEUE_KIT_QUEUE_FILE`, `QUEUE_KIT_DEFERRED_SECTION`,
  `QUEUE_KIT_WRAP_BUDGET`, `QUEUE_KIT_ENTRY_LINE_CAP`,
  `QUEUE_KIT_PRECONDITION_REGEX`, `QUEUE_KIT_ROADMAP_FILE`,
  `QUEUE_KIT_ROADMAP_MARKER`.
- **Six arrays** — `QUEUE_KIT_REQUIRED_SECTIONS`, `QUEUE_KIT_PROSE_LEADS`,
  `QUEUE_KIT_PROSE_SURFACE_GLOBS`, `QUEUE_KIT_LESSON_TAGS`,
  `QUEUE_KIT_HORIZONS`, `QUEUE_KIT_TRACKS`.

The survey omits `ROADMAP_FILE`, `ROADMAP_MARKER`, `PROSE_SURFACE_GLOBS`,
`HORIZONS` and `TRACKS` — the roadmap family, the same family its couples
attribution swapped. Sourcing the library reaches five more
(`QUEUE_KIT_ACTIVE_SECTIONS`, `QUEUE_KIT_DONE_SECTION`,
`QUEUE_KIT_ICEBOX_SECTION`, `QUEUE_KIT_ICEBOX_AGE_DAYS`,
`QUEUE_KIT_ATTEND_CAP`), which the validation above reads whether or not a member
declares them.

Every one of the thirteen is carried by the existing config bridge — the
substrate that landed last iteration and ported zero gates by ruling, and
discharging criterion 6 for these arrays is exactly what it was built for. Each
member declares its knob reads in its registry tuple's fourth element, so a knob
the bridge was never asked to carry cannot be read.

**One knob shape the bridge cannot carry, found here and clear here.**
`QUEUE_KIT_LESSON_SINKS` is a `declare -A` associative array (queue-kit/lib/queue.sh:56).
The bridge's wire format is **tab-joined elements with no key channel**
(§lib/gate.sh), so an associative array has no serialization in it. **No cohort
member reads it** — it is `bin/lesson-sink.sh`'s — so this cohort is unaffected.
It is recorded because it is a property of the bridge that the next kit's port
will meet, and meeting it at implementation time rather than at design time is
the expensive order.

**(4) `gates.list` is unchanged.** [mechanical] A port swaps a member's
declaration spelling, never its name, so the registry is untouched — which is
what makes the `check-graph` proof above a proof of the *manifest* rather than of
a rewritten registration.

### `check-queue-slug-liveness` — the walker

**(5) Basename-glob list matching over a bridged glob array.** [design-bearing]
The member walks `QUEUE_KIT_PROSE_SURFACE_GLOBS` (this repo's config sets
`docs/*.md` and `*.local.md`). `walk.rs` today filters by extension; this needs
list matching of a **basename glob**. Two commitments are inherited rather than
re-decided: the matcher is **`**`-capable (globstar semantics)**, already
committed under criterion 6 against the `shopt -s globstar` the shell side
enables; and the bridge transports the glob **strings** and interprets nothing,
so the matcher lives on the reader's side.

This is one of the two shared mechanisms §The first cohort records **Cohort A**
(the canon-kit `spec_manifest_files` callers) as still owing. Building it here
means Cohort A's remaining debt after this iteration is the other one alone — a
Rust `gate_kit_roots` for kit-root pruning. Recorded so the next selector reads
Cohort A's cost correctly rather than re-deriving it.

The member's walk root is declared in its registry tuple's third element and held
to executed behavior by crate unit test A. The declaration is a concrete root or
`?`, never a guess.

### `check-task-conservation` — the fixture-less member

**(6) Parity is proved by a constructed scenario, not a fixture pair.**
[design-bearing] Its `# no-fixture:` reason is structural and stays true after the
port: the rule diffs HEAD against the worktree, and a committed fixture has
HEAD == worktree, so the loss case has no static representation. Criterion 2's
demand — *"parity between substrates is proved by running both against the same
cases, never asserted"* — is therefore met by a **scratch-clone scenario**: a
throwaway repo in which an entry is deleted in the worktree without committing,
both implementations run against it, and their bytes compared. That still
satisfies the criterion's actual requirement (same cases, both substrates, while
both implementations exist) without pretending a fixture pair is available.

**(7) The conservation table's `check-gate-output` row must change, and it is a
prerequisite rather than a follow-up.** [design-bearing] This is the port's
sharpest consequence and it is invisible from the cohort's own gates.

`check-gate-output` resolves each `gates.list` member's declaration path and
branches on `# no-fixture:` (gate-sdk/checks/check-gate-output.sh:37): a fixtured
member is asserted on real output by the runner, and a `# no-fixture:` member is
**source-grepped** for its `: clean` and `help:` lines (:41, :44). The
conservation table records this precisely — *"source-grep retained for the one
member outside it… The remaining member, `check-task-conservation`… has no case
for a runtime assertion to reach, so the source-grep stays its only oracle"*
(§Meta-gate conservation for the binary substrate). **That one member is a cohort
member.** After the port its resolved declaration path is the `.gate` descriptor,
and a descriptor carries the `# graph:` manifest, its `# spec:` pointer and an
optional `# no-fixture:` opt-out — *"Nothing else"* (§The `.gate` descriptor). So:

- Keep `# no-fixture:` on the descriptor → the source-grep arm runs against a
  file that **cannot** contain an `echo`/`printf` line → `check-gate-output` goes
  **red**.
- Drop `# no-fixture:` → the member falls to the runtime arm, where **no fixture
  case exists for the runner to assert over** → its output-contract coverage
  silently becomes **zero**. That is the exact vacuity the conservation table
  exists to close, arriving through the table's own remaining exception.

Both branches are wrong, so the port owes a design. The repair: for a
`.gate`-dispatched `# no-fixture:` member, `check-gate-output`'s source-grep
corpus follows the rule to where it now lives — the **implementation module** —
resolved by the crate's existing name convention, which is specified once rather
than left as a second registry that could drift. This is the same "corpus
extended to the Rust module" shape the table already uses for
`check-comment-tier`, `check-gate-assertions` and `check-todo-task-liveness`; what
is new is that this reader needs the mapping **per member** rather than a widened
glob, and that is the piece to specify.

Sequencing follows: `check-task-conservation` ports **last within the cohort**,
after delta (7) lands, on the same principle §The first cohort applies to
criterion-4 and criterion-7 gates — *"Neither is ported and patched later; both
are designed, then ported."*

### Conservation and parity obligations across the cohort

**(8) Disposition coverage is re-derived, not assumed.** [design-bearing]
`check-gate-substrate-parity` assertion C derives the substrate-sensitive set at
runtime — a member whose expanded `couples=` covers a registry member's
declaration path — and reds any member without a disposition line. Ten new
descriptors change which members that derivation selects, so the table is
re-checked against the derivation after the port rather than assumed complete.
Two rows are known to be in scope already: `check-gate-output` by delta (7), and
`check-queue-slug-liveness`, which is itself a table row (*"Survive unchanged —
reverse triggers"*) **and** a cohort member — its row's reasoning stands, since
what it scans is the governed-doc set rather than a gate's content, but the row
now describes a `.gate`-declared gate and must read correctly as one.

**(9) Parity is proved while both implementations exist.** [mechanical] Assertion
A forbids a `<name>.sh` and a `<name>.gate` in one resolve dir, so the comparison
runs with the shell gate still in place and the descriptor staged elsewhere; the
descriptor lands and the script is deleted in one motion. Per member: the fixture
pair (or delta (6)'s scenario), the live tree, and an edge tree.

**(10) The commit-time obligations this repo already carries.** [mechanical]
`bash gate-sdk/bin/build-native.sh` before each commit, and the generated
projections regenerated per docs/site-architecture.md §Generated projections. Ten
descriptors and ten deleted scripts move the footprint and value rollup, the
graph artifact, the enforcement map and the docs mirror.

**(11) Two deferred entries become closure candidates as this lands.**
[mechanical] `gate-battery-parallel-execution` and `gate-battery-result-cache`
each say in their own text that the port subsumes them. They are dispositioned at
close, not built.

## Producers and consumers

**New interface: ten `.gate` descriptors under `queue-kit/checks/`.**
- *Producer* — this port; the descriptor's existence **is** the dispatch
  declaration, with no dispatch field and no mapping table (§The `.gate`
  descriptor).
- *Consumers* — `gate_resolve` (declaration path, for every manifest reader) and
  `gate_command` (invocation argv), gate-sdk/lib/gate.sh. Enabling config is
  already emitted everywhere it must be: `GATE_SDK_NATIVE_BIN` carries a kit
  default, so no consumer must set anything for dispatch to resolve.
- *Every field has a named reader* — `# graph:` by `check-graph`,
  `gen-pre-commit.sh`, `enforcement-map.sh` and `footprint.sh`; `# spec:` by
  `check-spec-pointer`; `# install:` by `check-install-disposition`;
  `# no-fixture:` (on `check-task-conservation`'s descriptor only) by
  `check-gate-fixture-coverage` **and**, per delta (7), by `check-gate-output`.
  No field is added to the closed roster.

**New interface: ten `REGISTRY` tuples in `native/src/gates/mod.rs`.**
- *Producer* — the crate, at compile time; a member added without its declared
  read roots and knob reads fails to compile.
- *Consumers* — `lookup` (dispatch), `roots` (`--reads`, consumed by
  `check-reads-couples`), `knobs` (`--knobs`, consumed by `gate_command`'s bridge
  loop at gate-sdk/lib/gate.sh:156-167), `names` (`--list`, consumed by
  `check-gate-substrate-parity` assertion B).
- *Field readers at named transitions* — the **knob-reads** element is read by
  `gate_command` at argv construction, where each name becomes one
  `GATE_SDK_KNOB_<NAME>=<tab-joined>` element; the **read-roots** element is read
  by `--reads` and held to behavior by unit test A at `cargo test`.

**New interface: the shared queue-library Rust module (delta 2).**
- *Producer* — the cohort's own port.
- *Consumer* — each cohort member that calls a queue helper; there is no
  external caller and no `--` arm exposing it, so it adds nothing to the binary's
  flag surface and cannot leak into `--list` as a phantom subcommand.

**New capability: basename-glob list matching in `walk.rs` (delta 5).**
- *Producer* — `check-queue-slug-liveness`'s corpus derivation, driven by the
  bridged `QUEUE_KIT_PROSE_SURFACE_GLOBS`.
- *Consumer* — the walk recorder, which must observe the roots this walk visits or
  unit test A's assertion over the member is vacuous. Unit test B's rule binds:
  the matching lives inside the crate's single sanctioned walk implementation, not
  in the gate module, or the walk becomes invisible to the recorder.

**Red conditions of the readers this change touches** (§The causal-completeness
check, point 5 — the port **narrows** a corpus by deleting ten `.sh` files, and
"a narrower corpus can only remove violations" is the first argument this delta
reaches for and is false):
- `check-gate-output` — red on a **zero count** of a matching `: clean` / `help:`
  line. Non-monotone: deleting `check-task-conservation.sh` *adds* a violation.
  This is delta (7), and it is the attested instance of exactly the shape point 5
  names.
- `check-gate-fixture-coverage` — red on a member with **neither** a pair **nor**
  a `# no-fixture:` opt-out: another zero-count reader, cleared only if the
  descriptor carries the opt-out forward.
- `check-gate-substrate-parity` assertion B — red on a descriptor with no
  subcommand **and** on a subcommand with no descriptor; both directions, so a
  half-landed member reds either way.
- `check-gate-substrate-parity` assertion C — red on a substrate-sensitive member
  with **no disposition line**: a zero-count reader over a set the port *widens*.
- `check-reads-couples` — red on a walk outside the declared couples, and its
  shell parser finds nothing in a binary gate; the `--reads` consumption is what
  keeps it from printing `clean` vacuously over the ported members.
- `check-readme-roster` — red in **both** directions, so queue-kit's README must
  move from ten `.sh` names to ten `.gate` names rather than gaining them.
- `check-shellcheck`, `check-comment-tier`, `check-spec-pointer`,
  `check-todo-task-liveness`, `check-deprecation-task` — each loses ten shell
  files from its corpus and gains ten descriptors plus ten Rust modules; the
  comment-surface arms for `*.gate` and `*.rs` already exist, so these are
  monotone here and clearable by inspection.
- `check-gate-binary-fresh` — red on a stale binary once a member dispatches;
  already armed, and ten more dispatches do not change its predicate.
- `check-docs-cmd` — red on a doc fencing a path that no longer runs: real signal
  after ten `.sh` deletions, and the reason the docs mirror is in delta (10).

## Existing sections updated

- **gate-sdk/SPEC.md §The first cohort, and the rule that selects the next** —
  owned by the reconciliation section above. Records that the by-kit ruling and
  the shared-corpus ordering rule select the same set here, and why that is not a
  precedent for taking any kit. Also updates its statement of Cohort A's
  outstanding shared mechanisms, since delta (5) discharges one of the two.
- **gate-sdk/SPEC.md §Meta-gate conservation for the binary substrate** — owned
  by deltas (7) and (8). The `check-gate-output` row is rewritten: its "one
  member outside the fixtured corpus" is a ported member, and the row states the
  implementation-module corpus rather than a source-grep over a descriptor. The
  `check-queue-slug-liveness` row is re-read as describing a `.gate`-declared
  gate.
- **gate-sdk/SPEC.md §The port-candidate criteria** — owned by the two-splits
  section. Criterion 2 gains the constructed-scenario form as the parity oracle
  for a `# no-fixture:` member, which criterion 2 as written does not contemplate.
- **gate-sdk/SPEC.md §check-gate-output** — owned by delta (7): the per-member
  corpus resolution for a `.gate`-dispatched `# no-fixture:` member.
- **queue-kit/SPEC.md** — every per-gate contract section whose prose says the
  gate is a shell script or cites its `.sh` path; and
  **§check-task-conservation**, which owns the `# no-fixture:` reasoning delta (6)
  extends to parity.
- **queue-kit/README.md** — gate roster, ten names, `.sh` → `.gate`.
- **native/src/gates/mod.rs** — the registry, and the module-per-gate comment
  whose "one module per ported gate" claim now spans twelve members.

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
