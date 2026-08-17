# SPEC amendment: second-batch

The **second budget batch** — two members, taken as two independent units.

`native-gate-port-remaining-corpus`'s increment for this iteration. The arm
itself is canonical at §The first cohort, and the rule that selects the next and
is not re-argued; this file owns the **cut**, the two adjudications it produced,
and nothing else the arm's record-only-findings rule does not admit.

**The arm's precondition holds, and it holds through an adjudication rather than
on the run's face.** `bash gate-sdk/bin/port-blockers.sh --group` at this rev
reports `104 member(s) scanned, 34 group(s) formed, 0 undecidable, 69 already
ported and excluded`. Exactly one group is non-singleton — group 1, pairing
`check-install-disposition` with `check-readme-roster` — and it is a **phantom
pair**: §The port-candidate criteria's exception class (a) rules
`check-install-disposition` permanently shell, and §Meta-gate conservation for
the binary substrate's own row for it says the same, so the group's portable half
is `check-readme-roster` alone. Every group is in fact a singleton, the size arm
is **exhausted**, and the increment belongs to the budget arm. The adjudication
is the one §The first cohort sanctions — *an advisory group is a finding the
selecting session adjudicates* — and the census behind it is filed at
`.workflow/survey-record.md`, cited here behind a passing witness (the corpus
diff since its rev is empty and the oracle's trailer is byte-identical) rather
than re-bought.

**k = 2**, cut at scope on this iteration's own budget, with its grounds recorded
in that same survey record. The two members are cut **here**, at authoring,
because §The first cohort names this stage the producer of a batch's roster and
forbids recording one in the SPEC.

**Ported count at authoring, from the oracle rather than a directory listing:**
`bash scripts/measured-claims.sh` emits `ported-gate-members` at 69 of 104.

## What changes

### 1. Two members port, each as an independent unit on the budget arm

`check-commit-subject` (44 shell lines) and `check-readme-roster` (108). Both are
registered in `scripts/gates.list`, both carry a `good/`+`bad/` fixture pair, and
neither has a `.gate` descriptor today — probed, not assumed — **design-bearing**,
because what makes this a delta is the composition ruling rather than either
port.

**No joint proof exists and none is claimed.** Each takes its own descriptor, its
own registry entry, its own parity run and its own deletion, and either may be
dropped without touching the other. They share no corpus derivation: one reads a
prospective commit message, the other every kit's `checks/` basenames and README
marker block. The batch is not one unit of work and is never merged, recorded or
argued as one.

**Why these two rather than the four the audit cleared.** `check-commit-subject`
is the cheapest zero-debt member on the whole remainder — no criterion carries a
cost and no still-shell gate shares a derivation with it. `check-readme-roster`
is the phantom pair's portable half, so its port dissolves the last
non-singleton group **structurally** — the same defect
`port-remainder-permanent-shell-inflation` closes from the tool's end in this
same iteration, closed here from the member's end.

**Corrected at align: the draft's citation to a prior disposition does not
resolve.** It read "§The first budget batch recorded it *READY with bounded
pre-work* and left it as 'the cheapest thing to leave for the next one'" —
grepped and read in full, §The first budget batch (gate-sdk/SPEC.md:2307-2360)
names six other members and never mentions `check-readme-roster`; neither phrase
occurs anywhere in the tree. `check-readme-roster`'s actual disposition is this
iteration's own finding, not a carried one: this SPEC's survey record (spec
stage, 2026-08-17) is where it was first probed and found to need criterion-4
fixture pre-work, which delta 5 discharges.

`check-root-tiering` (67 lines, no pre-work) and `check-core-files` (76) were
cleared and **not taken**: the budget is 2, neither retires a blocker later
members are queued behind, and `check-core-files` additionally owes a
`kit:<glob>` expander the crate does not carry — the cheapest thing to leave for
the next batch, on the same budget ground that left these two for this one.

### 2. Criterion 3 does not bar a `tier=commit-msg` member, and the emitter already carries it

`check-commit-subject` is the **first `commit-msg`-tier port**, and the doubt that
raises is adjudicated here rather than left for the next selector to re-buy —
**design-bearing**.

Criterion 3 reads **`tier=precommit`**, and its stated reason is that the member
*"lands in the generated hook, so a green `check-graph` after the port is
end-to-end proof the manifest survived the substrate change."* A `commit-msg`
member lands in the generated **`commit-msg`** hook, which `check-graph` holds
against `--emit-commit-msg` on identical terms. The criterion's **purpose** is
therefore met in full; the literal value is a proxy for *lands in a generated
hook `check-graph` holds*, and this is the first member to show the two coming
apart.

**The dispatch and the config bridge are already there, and this was probed
rather than assumed — the assumption ran the other way first.** The generated
`commit-msg` hook today emits `run_gate <name> <declaration>.sh "$msg_file"`,
which reads as a shell-only path with no `env` bridge, and on that reading the
port would owe a new emitter arm. It does not: the emitter builds that argv
through the same `command_rel` → `gate_command` path the pre-commit emitter uses,
and `gate_command` is what emits `env <bridged knobs> <binary> <name>` for a
`.gate` member. **One emitter, one resolver, both tiers.** The message-file
positional lands after the subcommand name and reaches the gate module's argv
slice unchanged.

Recorded because a member the criterion's literal text appears to bar, and does
not, reads as a rule being bent unless the reason is written down.

### 3. `check-commit-subject` ports

The subject line parses as `<type>(<scope>)?!?: <summary>` with `<type>` from the
shared roster, **or** matches one of the git-generated carve-outs
(`Merge`/`Revert` and the `fixup!`/`squash!` autosquash forms) — **mechanical**.

One knob crosses the bridge unrenamed: `GATE_SDK_COMMIT_TYPES`, resolved through
`gate_commit_types` in the kit's shell library, which stays the single place that
value is computed. Criterion 6 is discharged **by construction** on the bridged-knob
reading §The port-candidate criteria already rules: the crate holds no roster to
drift from.

**Two behaviors are load-bearing and must survive the port, stated because both
look like edge cases and neither is.** The **no-argument clean skip** — a bare
invocation prints clean and exits 0, because the prospective message is not a
whole-tree surface and the full battery runs the gate with no argument. And the
**carve-out alternation**, which is not tidying: git authors those subjects
itself, so a port that dropped them would red on commits no author wrote.

### 4. `check-readme-roster` ports

Every kit README's gate-roster marker block holds **name-set parity, both
directions**, with that kit's shipped `checks/` basenames across **both
declaration spellings** — **mechanical**.

A kit root with no `checks/` directory is skipped and counted, not red. A README
with no marker block is a finding of its own, distinct from a parity finding, and
the two carry different `help:` text — a distinction the port keeps rather than
merging into one message.

The `<!-- gate-roster:begin -->` / `<!-- gate-roster:end -->` markers stay **kit
literals inside the gate module**, not knobs, exactly as the shell holds them:
they are gate-sdk's own mechanism rather than a consumer's document vocabulary,
which is the same seam ruling that kept doctrine-kit's section headings as module
literals in the first budget batch. Its corpus arrives through the bridged
`GATE_SDK_KIT_DIRS` (**corrected at align** — the draft named `GATE_KIT_ROOTS`,
which is not a knob; `GATE_KIT_ROOTS_HERE`/`GATE_KIT_ROOTS_REL` are the derived
arrays `gate_kit_roots` resolves *from* `GATE_SDK_KIT_DIRS`, per this gate's own
§check-readme-roster section); criterion 6 is clean, the derivation being
`gate_kit_roots` plus two globs and a set comparison, with no shared corpus
helper to dual-hold.

### 5. `check-readme-roster` fails criterion 4, and the discharge is fixture pre-work

Its scanned corpus **is** every kit's `checks/` directory, and its own declaration
path lies inside it — criterion 4's predicate exactly — **design-bearing**,
because the discharge has to be executed in the right order or it proves nothing.

**Widen first, then port.** Criterion 4's discharge is a fixture corpus carrying
**every arm of the derivation being ported**, and this pair does not: probed
directly, every kit in both the `good/` and `bad/` trees ships only `.sh` files
under `checks/`, so the gate's `.gate` arm — which the shell already implements
correctly — is **exercised by no case at all**. The pre-work is fixture
construction with no design in it: a kit whose `checks/` holds a `.gate`
descriptor in both trees, and a **mixed** `.sh`+`.gate` kit, which is what proves
the union of the two globs rather than either alone.

**The live-tree arm is demoted from proof to smoke for this member**, on the terms
§The port-candidate criteria already sets for a gate-source auditor: assertion A
forbids a descriptor and a script coexisting in one resolve dir, so the
comparison necessarily runs on the pre-descriptor tree — a corpus this port then
changes. Its verdict is recorded as **no disagreement found on the pre-descriptor
tree**, never as parity proved. The edge-root arm keeps its own separate value.

**`check-commit-subject` clears criterion 4** — probed, not assumed: its
`couples=` names `gate-sdk/lib/gate.sh`, a **reverse trigger** on the roster's
source, and the corpus it scans as content is a single message file.

### 6. Every ported member's declaration and registration, per member

Each member's `.gate` descriptor carries its script's `# graph:`, `# install:`
and `# spec:` header lines **verbatim** — the manifest, the install disposition
and the spec pointer are properties of the gate, not of its substrate. Each takes
one registry tuple in the crate's gate table declaring its name, its `run`
function, its declared walk roots, the knobs it reads, and the kit root that
carries its descriptor — **mechanical**.

**No knob is renamed, dropped, or given a crate-side default.**

**`check-readme-roster`'s own SPEC section additionally takes the substrate edit
every other ported gate's section in this file already carries — corrected at
align, where the draft's "Existing sections updated" bullet claimed neither
ported section needed one.** §check-readme-roster's positional-form sentence
today names the `.sh` suffix explicitly
(`` `check-readme-roster.sh [root]` ``, gate-sdk/SPEC.md:7233-7234); every other
already-ported gate's section drops that suffix and gains an **"Its
implementation is a compiled subcommand, on §check-action-pinning's terms"**
sentence at port (§check-smoke-entry-guard and §check-template-registry-parity
are two of six precedents in this same file). This member's port takes that same
edit. `check-commit-subject`'s section names no invocation form today and needs
no such edit — the "neither section names its substrate" verdict holds for it
alone.

**Neither descriptor carries a `# no-port:` line**, and the sibling amendment in
flight for this component is what makes that worth saying: that field's domain is
the shell declaration path only, and a descriptor carrying one is a red under the
assertion it lands. A port **deletes** the question rather than answering it.

### 7. Each member's parity run, and the deletion that follows it

Fixture pair, live tree and edge roots, run **while both implementations still
exist** — the only order in which parity can be proved. The shell script is
deleted in the same unit that lands the descriptor, never left running beside it
— **mechanical**.

Neither member's parity run stands in for the other's: two runs, not one. And
`check-readme-roster`'s must follow delta 5's widening, or it proves the arm the
port is least sure of by not exercising it.

### 8. The substrate-sensitive set is re-derived at this batch's cut

A port **changes declaration paths**, which can move other members into or out of
the derived set, so assertion C's derivation is run fresh after both descriptors
land rather than inherited — **mechanical**.

Probed at authoring, with the expected outcome stated so a divergence is visible
as one: `check-readme-roster`'s `couples=` names `kit:checks/*`, so it is
substrate-sensitive and **already carries its row** (*retained, glob widened to
`*.sh` + `*.gate`*), which the port does not change — the row now describes a
ported member reading ported members' declaration paths, the shape
`check-value-rollup-fresh`'s row already has. `check-commit-subject`'s
`couples=` names `gate-sdk/lib/gate.sh`, which covers **no** registry member's
declaration path, so it earns no row. The verdict is still taken by running the
derivation, never read off this paragraph.

### 9. This SPEC gains §The second budget batch, and it records only deltas 2 and 5

Under the arm's record-only-findings rule the section carries delta 2's
criterion-3 adjudication and delta 5's criterion-4 ordering, and **not** a member
roster — membership is derivable from the tree and the count from
`scripts/measured-claims.sh` — **mechanical**.

**Audited at align: a k=2 batch where each recorded delta names its one subject
reads close to a roster, and is ruled distinct rather than assumed so.** Delta 2
and delta 5 are technical adjudications — a criterion satisfied in spirit despite
its literal wording, a criterion failed and discharged by fixture pre-work —
each argued on its own terms; a roster is a bare enumeration asserting only
membership, with no argument attached. That a two-member batch's two arguments
happen to name both members is a property of *this* batch's size, not a
membership list reintroduced under a different label — and membership stays
independently derivable from the tree with this section absent entirely, which a
roster's information could not be.

### 10. The binary is rebuilt and the full battery run before commit

`bash gate-sdk/bin/build-native.sh` plus `bash gate-sdk/bin/run-gates.sh`, and
neither discharges the other — **mechanical**.

## Producers and consumers

### Each ported member's descriptor and registry entry

- **Producer:** this batch's build, one member at a time. The descriptor is
  written into the member's existing `gate-sdk/checks/` directory; the registry
  tuple into the crate's gate table. **Enabling config actually emitted:** both
  members' knobs are resolved by `gate-sdk/lib/gate.sh` and crossed by the config
  bridge, which this repo's committed kit config sets today; not test-only, since
  both run in the live battery on every commit.
- **Consumers, named with the mechanism, across the whole component set:**
  `check-graph` reads the `# graph:` manifest off each descriptor and regenerates
  **both** hooks — the `commit-msg` hook as well as `pre-commit`, which is delta
  2's whole subject and is the one consumer a precommit-only batch never
  exercises; `check-spec-pointer` and `check-comment-tier` read the `.gate` and
  the `.rs` through the shared comment surface; **`check-readme-roster` itself**
  matches its `*.sh`+`*.gate` globs against each kit's README roster in both
  directions — it is a consumer of its own port and of its sibling's;
  `check-exec-bit` asserts each descriptor is **non**-executable;
  `check-gate-binary-fresh` reads declaration paths as a set;
  `check-gate-substrate-parity` holds assertion A and assertion C;
  `check-reads-couples` consumes the binary's `--reads` report;
  `check-install-disposition` reads `# install:` off the descriptor exactly as
  off the script; `check-gate-fixture-coverage`, `check-kit-enum`,
  `check-enforcement-fresh` and `check-value-rollup-fresh` read declaration paths
  as text, directly or through the footprint and enforcement-map emitters, so
  their generated projections are regenerated by this batch.
- **Every field has a named reader.** The registry tuple's five fields: the
  **name** by the binary's dispatch match; the **function pointer** by the same
  dispatch; the **declared walk roots** by the `--reads` arm and thence by
  `check-reads-couples`, and by the crate's meta-test comparing declared roots
  against roots the walk recorder observed over the member's fixture cases; the
  **declared knobs** by the `--knobs` arm and the config bridge; the **owner
  root** by the crate's meta-test asserting a descriptor exists at
  `<owner>/checks/<name>.gate`.

### The message-file positional, which is this batch's one new argv path

- **Producer:** git, which hands the prospective message path to the
  `commit-msg` hook as `$1`; the generated hook passes it as the argv tail of the
  member's resolved command.
- **Consumer:** the ported member's `run(args)` argv slice, at its first
  statement — the same slice a `.gate` member's positional arguments already
  arrive in.
- **Red condition, named rather than its subject:** an **absent** positional is
  not red and must not become one — it is the clean-skip branch of delta 3, which
  is how the whole-tree battery invokes the member. A **present but unreadable**
  path is exit 2, fail-closed, unchanged.

### This batch narrows a corpus, so every reader's red condition is named

Two `*.sh` files leave `gate-sdk/checks/`. Causal-completeness point 5 binds, and
*a narrower corpus can only remove violations* is false in four places:

- **`check-readme-roster`** — **not monotone, and it is one of the two members**,
  which is the sharpest instance of this rule in the batch. Its rule is a two-way
  roster match over `checks/` basenames, so deleting a `.sh` without landing its
  `.gate` in the same unit strands the gate-sdk README row in one direction and
  the roster entry in the other. Delete and descriptor land together, per member
  — **and the member whose port changes the corpus is the member doing the
  checking**, so its own port must be the one that keeps its own README row
  honest.
- **`check-settings-paths`** — **not monotone.** Its red condition is a committed
  permission grant naming a path that no longer exists, so the narrowing **adds**
  violations: every allow entry naming `gate-sdk/checks/check-commit-subject.sh`
  or `check-readme-roster.sh` is stranded by its port and re-pointed in the same
  unit.
- **`check-docs-cmd`** — **not monotone**, same shape: a doc fencing either
  deleted `.sh` path reds after the port. Two paths to grep, stderr **not**
  silenced — a `2>/dev/null` on a path grep reads a bad path as *no reader*.
- **`check-measured-claim`** — **not monotone.** Its red condition is a marked
  sentence disagreeing with its oracle, and this batch **moves the oracle**:
  `ported-gate-members` goes 69 → 71 as the members land, so every marked
  sentence stating a ported count moves with the batch.
- **`check-gate-fixture-coverage`** — monotone and clearable by inspection: both
  pairs stay under `gate-tests/`, which is pruned from every live-tree walk, and
  the descriptor inherits the coverage obligation the script carried.
- **`check-shellcheck`** — monotone: its corpus shrinks by two files with no
  shell left to lint. Retired for a ported member with cause, per the
  conservation table.
- **`check-gate-output`** — clearable, and the reason is the fixture pairs: for a
  fixtured member the output-contract assertion runs in `run-gate-tests.sh`
  against real output, so removing the source-grep target ends no assertion.
- **`check-graph` and both generated hooks** — regenerated from the descriptors'
  manifests; the red condition is staleness against the manifests, not a count,
  and a green `check-graph` after each port is the end-to-end proof criterion 3
  exists for — reaching the `commit-msg` hook for the first time, per delta 2.

## Existing sections updated

- **gate-sdk/SPEC.md §The first cohort, and the rule that selects the next** —
  gains `### The second budget batch` beneath it (delta 9), carrying deltas 2
  and 5 and nothing else.
- **gate-sdk/SPEC.md §The port-candidate criteria** — criterion 3's wording,
  which delta 2 shows is a proxy rather than a bar; and criterion 4's fixture
  discharge, where delta 5's instance belongs beside the ones already recorded.
- **gate-sdk/SPEC.md §Meta-gate conservation for the binary substrate** — delta
  8. No row is added unless the cut's re-derivation moves a member into the set;
  the re-derivation's verdict lands there either way, because *a member the
  section does not name is red*.
- **gate-sdk/SPEC.md §check-commit-subject** — takes no substrate change, and
  that is a verdict rather than an omission: probed, the section names no
  invocation form and no substrate today, because a gate's SPEC section owns its
  **rule** and the substrate is the descriptor's business.
- **gate-sdk/SPEC.md §check-readme-roster** — **corrected at align: this section
  DOES take a substrate edit**, delta 6's added paragraph — the `.sh` suffix
  drops from its positional-form sentence and it gains the compiled-subcommand
  sentence every other ported gate's section already carries. The draft's
  original bullet grouped this section with §check-commit-subject under "neither
  takes a substrate change"; that held only for the latter.
- **gate-sdk/README.md's gate roster** — held in both directions by the very
  member this batch ports.
- **The committed permission allow-list, and any doc fencing a deleted `.sh`
  path** — the two not-monotone readers named above, each owning its delta
  through the member whose script it names.
- **`native-gate-port-remaining-corpus`** — two edits, corrected at align to name
  both (the draft's bullet named only the first). Its dated `69 of 104 ported`
  oracle read (TASK-QUEUE.md:39-41) moves to 71 of 104 — **by hand, not by
  `check-measured-claim`**: that line explicitly disclaims durability ("never a
  count this line holds") and TASK-QUEUE.md carries no `<!-- measured: -->`
  marker at all (grepped), so no gate catches a stale copy of it; citing
  `check-measured-claim` as what forces this edit, as the draft did, overclaims
  what the gate covers (it does hold the real marked claim at
  `docs/install.md:193-194`, which is a separate edit and does move under that
  gate). Second, per this SPEC's DoD "Terminal move is a demotion, not a Done
  move": the entry drops its `[spec:]` tag and returns to `## Deferred` under
  `[design-pending]`, keeping its `[roadmap:]` tag.

## Definition of Done

- [ ] **Causal completeness** — each ported member's reader set is named with its
      mechanism, including the `commit-msg` hook consumer no earlier batch had;
      every registry-tuple field has a named reader at a named transition; the
      message-file positional's absent case is named as *clean*, not as red; the
      narrowing's four non-monotone readers are discharged rather than reasoned
      clear.
- [ ] **Order held on `check-readme-roster`** — the fixture widening of delta 5
      lands **before** its parity run, and its live-tree verdict is recorded as
      *no disagreement found on the pre-descriptor tree*.
- [ ] **Merged with no information lost** — deltas 2 and 5 land as
      `### The second budget batch`; delta 8's verdict lands in the conservation
      section. **No member roster is written into this SPEC.**
- [ ] **Amendment deleted** — this file removed on merge; `ls gate-sdk/SPEC-*.md`
      checked at the iteration horizon, since `SPEC-port-permanence.md` is a
      sibling in flight for this same component.
- [ ] **Removals propagated** — grepped every spec, README, doc and permission
      allow-list for both deleted `gate-sdk/checks/<gate>.sh` paths; nothing
      dangles. Stderr not silenced on any of those greps.
- [ ] **Gaps filed** — cross-component gaps filed through
      `bash lifecycle-kit/bin/file-gap.sh`; a build-time causal gap resolved that
      session rather than deferred.
- [ ] **Terminal move is a demotion, not a Done move** —
      `native-gate-port-remaining-corpus`'s deliverable is the whole corpus and
      this batch is one increment, so the entry drops its `[spec:]` tag and
      returns to the deferred section under `[design-pending]`, keeping its
      `[roadmap:]` tag and its place in the public projection.
