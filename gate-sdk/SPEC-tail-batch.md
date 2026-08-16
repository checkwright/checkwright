# SPEC amendment: tail-batch

The **first budget batch** — six members, taken as six independent units, plus
the one shared primitive two of them are queued behind.

This is the first application of the budget arm authored beside this file in
`SPEC-budget-arm.md`, which owns the arm itself; this file owns the batch. The
arm's precondition holds on a live run: `bash gate-sdk/bin/port-blockers.sh
--group` reports 37 groups over the 45-member remainder — 34 singletons, two
pairs and one 7-member group — and **no group is takeable**. The 7-member key
`libs=fail_closed globs=-` is ruled out (§The canonical-spec
`spec_canonical_specs` cohort, operator-ruled 2026-08-14); one pair holds
`check-install-disposition`, which §Meta-gate conservation for the binary
substrate keeps on shell permanently; the other holds
`check-docs-render-fidelity` on criterion 7. So the size arm selects nothing and
the batch is composed by budget, exactly as the arm provides.

**The six were selected by a criteria audit over the eleven members of the three
smallest-key groups**, recorded in `.workflow/survey-record.md`. Two more members
of that audit are *READY with bounded pre-work* — `check-queue-prose-precondition`
and `check-readme-roster` — and are deliberately outside this batch: the budget
is spent, and a member with pre-work is the cheapest thing to leave for the next
one. Three are not near-term at all and stay where the audit put them.

**Ported count at authoring, from the oracle rather than a directory listing:**
`bash scripts/measured-claims.sh` emits `ported-gate-members 59` of 104.

## What changes

### 1. Six members port, each as an independent unit on the budget arm

`check-hook-exec-bit` (46 shell lines), `check-agent-tier-explicit` (62),
`check-rule-citation` (77), `check-workflow-tiering` (85), `check-brevity` (104)
and `check-doctrine-registration` (246). All six are registered in
`scripts/gates.list`, all six carry a `good/`+`bad/` fixture pair, all six are
`tier=precommit`, and none has a `.gate` descriptor today (probed, not assumed)
— **design-bearing**, because what makes this delta a delta is the composition
ruling rather than any one port.

**No joint proof exists and none is claimed.** Each member takes its own
descriptor, its own registry entry, its own parity run and its own deletion, and
any member may be dropped without touching the other five. The batch is not one
unit of work and is never merged, recorded or argued as one.

### 2. A heading-bounded section walker lands in the crate as a shared primitive

The crate has **no** heading-bounded section walker today, and two of the six
gates hand-roll the identical awk one — **design-bearing**.

The primitive: enter a section at the first heading whose text matches a
**caller-supplied** name at any level 1–6, and close it at the next heading whose
level is **less than or equal to** the opening level; yield the bounded line
range, or nothing when the named section is absent. `check-doctrine-registration`
needs a second application of the same rule — a per-rule trailer walk *inside* a
bounded range — which is the primitive applied to the range at the next level
down, so this is one primitive rather than two.

**Why an existing module is not generalized in place, stated so a merging
session does not re-derive it.** `native/src/declaration.rs`'s section reader is
the closest analogue and is a **different rule**: it matches `##` only, resolves
the section name by literal prefix rather than a caller-supplied one, and closes
on the next `##` regardless of nesting — so a `###` subheading does not close it,
the opposite of same-or-shallower semantics. Generalizing it in place would
change the declaration grammar's own semantics under its existing callers.
`native/src/spec.rs`'s prose walker is a paragraph accumulator with no section
concept, and `native/src/queue.rs`'s matchers are fixed at `## ` with no level
tracking. A private, unexported level-computing helper exists inside one gate
module and is single-purpose plumbing rather than a callable primitive. So the
primitive lands as its own module, `native/src/section.rs`.

**The seam holds at the argument boundary.** The crate ships the walker and no
section vocabulary: every section name arrives as an argument, from
`CONTEXT_KIT_BREVITY_SECTION` and `DOCTRINE_KIT_DIGEST_SECTION` through the
config bridge, and from doctrine-kit's own two kit literals inside its gate
module — exactly where the shell holds them today. A section name compiled into
`section.rs` would be one project's document structure shipped as everyone's
mechanism.

### 3. The two walker-riding members are sequenced, not cohorted

`check-brevity` lands the primitive and `check-doctrine-registration` rides it,
and stating that this is **sequencing** rather than a rejoined cohort is the
adjudication this batch owes a later selector — **design-bearing**.

What they share is a **parsing primitive**, not a corpus derivation, and the
cohort axis keys on the latter: §The first cohort composes a cohort so that
*"the walk is ported once and proved N times, and the parity comparison is over
one corpus shape rather than N"*. These two walk different corpora —
`CLAUDE.md` against `CLAUDE.md` plus `doctrine-kit/DOCTRINE.md` — so there is no
single corpus shape to compare over and no proof either can lend the other.
Each keeps its own parity run.

The sequencing is soft in both directions, which is what keeps delta 1's
drop-any-member property true: drop `check-doctrine-registration` and the
primitive lands with one caller, which is still correct; drop `check-brevity`
and the primitive's landing moves into `check-doctrine-registration`'s own unit.

**This is also a finding about the tail rather than about these two gates**, and
it is why §The first budget batch exists at all under the arm's
record-only-findings rule: a missing shared primitive is invisible to
`port-blockers.sh --group`, whose key is shell libraries and globs, and both of
these gates share **no** shell library — each duplicates the walker inline. A
later selector must not read `--group`'s silence as evidence that no shared
primitive is owed.

### 4. `check-hook-exec-bit` ports

A pure scan of `git ls-files -s` over the hooks directory asserting index mode
`100755`; one knob, `GATE_SDK_HOOKS_DIR`; the spawn goes through the crate's
single sanctioned subprocess wrapper so the fail-closed property is structural —
**mechanical**.

**One sequencing note, because its corpus moves during this batch.** Its scanned
corpus is `scripts/git-hooks/*` — the *generated* pre-commit hook, which every
member's descriptor landing regenerates. Its **fixture pair is the proof** and
its live-tree arm is smoke, on the terms §The port-candidate criteria already
sets for a member whose live corpus the port itself changes; run its live arm
before the batch's other descriptors land, or record its verdict as *no
disagreement found* rather than as parity proved.

### 5. `check-agent-tier-explicit` ports

Every `*.md` under the agent directory declares a `model:` field in its
frontmatter — an explicit `inherit` passes, only omission reds; knob
`DELEGATION_KIT_AGENT_DIR`; its shell `gate_path_pruned` call becomes the
crate's pruned walk under the bridged `GATE_PRUNE_DIRS` — **mechanical**.

### 6. `check-rule-citation` ports

Every `**<name>** rule` citation inside delegation-kit/SPEC.md §The delegation
model resolves to a bold lead-in in `templates/agent-execution.md`, forward
direction only — an orphan template bullet stays unflagged, and the port
preserves that asymmetry rather than tidying it; the two file paths are
positional arguments with hardcoded defaults and no env knob, which the registry
entry declares as an empty knob list — **mechanical**.

### 7. `check-workflow-tiering` ports, both assertions

**(A)** every `.workflow/*` member is git-tracked or git-ignored, none of
neither; **(B)** every tracked non-directory member's first line is
`# contract: <payload>` whose payload is an owner-path-plus-section pointer or a
format-name-plus-version. Knob `GATE_SDK_WORKFLOW_DIR`; both git reads go
through the sanctioned spawn wrapper, and the payload forms are EREs handled by
`native/src/ere.rs` — **mechanical**.

### 8. `check-brevity` ports, and lands the primitive

Within the heading-bounded section named by `CONTEXT_KIT_BREVITY_SECTION`, a
`- **name**` bullet is a violation iff its line span exceeds
`CONTEXT_KIT_BREVITY_BUDGET` **and** its body matches
`CONTEXT_KIT_BREVITY_POINTER_RE` **and** it carries no `brevity-exempt` marker
on its own first line or the line above — a three-way conjunction the port must
keep intact, since dropping any conjunct turns a calibrated gate into a length
police — **design-bearing**, because it is the delta that lands delta 2's
primitive and proves it.

Four knobs cross the bridge unrenamed: `CONTEXT_KIT_BREVITY_FILE`, `_BUDGET`,
`_SECTION`, `_POINTER_RE`. The pointer expression is an ERE and goes to
`native/src/ere.rs`.

### 9. `check-doctrine-registration` ports, and rides the primitive

The dearest member of the batch at 246 shell lines and three walkers, all three
the same rule — **design-bearing**.

What it asserts: the always-loaded agent file markdown-links the doctrine file;
its digest section covers every doctrine methodology rule 1:1 with no orphan
digest bullet and with declared `doctrine-digest-trim:` waivers honored;
**assertion D**, every craft rule carries exactly one well-formed `*Stages:*`
trailer; **assertion E**, every methodology rule carries exactly one non-empty
`*Digest:*` trailer. Knobs `DOCTRINE_KIT_AGENT_FILE`, `_DOCTRINE_FILE`,
`_DIGEST_SECTION` cross the bridge unrenamed; the two doctrine section headings
stay **kit literals inside the gate module**, not knobs, exactly as the shell
holds them and for the reason its own `spec:` line already gives.

Its three walkers — two section walks and one trailer walk — all collapse onto
delta 2's primitive. That collapse is the member's whole cost story: it is dear
because of the walkers, and the primitive is what makes it takeable at all.

### 10. Every ported member's declaration and registration, per member

Each member's `.gate` descriptor carries its script's `# graph:`, `# install:`
and `# spec:` header lines **verbatim** — the manifest, the install disposition
and the spec pointer are properties of the gate, not of its substrate. Each
takes one registry tuple in `native/src/gates/mod.rs` declaring its name, its
`run` function, the walk roots it declares and the knobs it reads, plus the kit
root that carries its descriptor — **mechanical**.

**No knob is renamed, dropped, or given a crate-side default.** Each keeps its
`<KIT>_<KNOB>` spelling and resolves through the config bridge, leaving the
kit's shell library as the single place a knob's value is computed
(§Meta-gate conservation for the binary substrate, the
`check-knob-default-coupling` row).

### 11. Each member's parity run, and the deletion that follows it

Fixture pair, live tree and edge roots, run **while both implementations still
exist** — the only order in which parity can be proved, since
§check-gate-substrate-parity assertion A forbids a descriptor and a script
coexisting in one resolve dir. The shell script is deleted in the same unit that
lands the descriptor, never left running beside it — **mechanical**.

No member's parity run stands in for another's, which is delta 1 restated where
a build session will actually be tempted: six runs, not one.

### 12. The substrate-sensitive set is re-derived at this batch's cut

A port **changes declaration paths** — `<kit>/checks/<name>.sh` becoming
`<name>.gate` — which can move other members into or out of the derived set, so
the verdict is taken by running assertion C's derivation at the cut rather than
inherited from any earlier cohort's table reading. Every derived member takes
exactly one disposition row, and a member the section does not name is red —
**mechanical**.

Criterion 4 clears for all six, probed rather than assumed: no member's
declaration path lies inside the corpus it scans as content.

### 13. This SPEC gains §The first budget batch, and it records only the findings

Under the arm's record-only-findings rule, the section carries delta 2's missing
primitive and delta 3's sequencing adjudication, and **not** a member roster —
membership is derivable from the tree and the count from
`scripts/measured-claims.sh` — **mechanical**.

### 14. The binary is rebuilt and the full battery run before commit

`bash gate-sdk/bin/build-native.sh` plus `bash gate-sdk/bin/run-gates.sh`, and
neither discharges the other — **mechanical**.

## Producers and consumers

### The section-walk primitive

- **Producer:** `native/src/section.rs`, called from the two gate modules of
  deltas 8 and 9. **Enabling config actually emitted:** the section names arrive
  as arguments — from `CONTEXT_KIT_BREVITY_SECTION` and
  `DOCTRINE_KIT_DIGEST_SECTION` through the config bridge, which this repo's kit
  configs set today, and from doctrine-kit's two module-level literals. Not
  test-only: both gates run in the live battery on every commit.
- **Consumer:** `gates::brevity` and `gates::doctrine_registration`, by direct
  call. The primitive has exactly two consumers at landing and that is stated
  rather than left implied — a primitive with one caller is a helper, and if
  delta 9 is dropped this one becomes that, which delta 3 already provides for.
- **Every field has a named reader:** the primitive returns a line range and
  nothing else. The range's readers are the two gate modules, each at the point
  where it begins its bullet or trailer scan. There is no absent-section field,
  by design: absence is the empty return, and each caller decides its own
  fail-closed behavior for it exactly as its shell original does.

### Each ported member's descriptor and registry entry

- **Producer:** this batch's build, one member at a time. The descriptor is
  written into the member's existing kit `checks/` directory; the registry tuple
  into `native/src/gates/mod.rs`.
- **Consumers, named with the mechanism, across the whole component set:**
  `check-graph` reads the `# graph:` manifest off the descriptor and regenerates
  the pre-commit hook; `check-spec-pointer` and `check-comment-tier` read both
  the `.gate` and the `.rs` through the shared comment surface;
  `check-readme-roster` matches its `*.sh`+`*.gate` glob against each kit's
  README roster in both directions; `check-exec-bit` asserts the descriptor is
  **non**-executable; `check-gate-binary-fresh` reads declaration paths as a set
  to decide whether the binary is load-bearing;
  `check-gate-substrate-parity` holds assertion A (no descriptor beside a
  script in one dir) and assertion C (the conservation rows of delta 12);
  `check-reads-couples` consumes the binary's `--reads` report into its coverage
  assertion; `check-install-disposition` reads `# install:` off the descriptor
  exactly as off the script; `check-gate-fixture-coverage`,
  `check-kit-enum`, `check-enforcement-fresh` and `check-value-rollup-fresh`
  read declaration paths as text, directly or through the footprint and
  enforcement-map emitters, so their generated projections are regenerated by
  this batch.
- **Every field has a named reader.** The registry tuple's five fields: the
  **name** is read by `main.rs`'s dispatch match; the **function pointer** by the
  same dispatch; the **declared walk roots** by the `--reads` arm and thence by
  `check-reads-couples` assertion A, and by the crate's own meta-test comparing
  declared roots against roots the walk recorder observed over the member's
  fixture cases; the **declared knobs** by the `--knobs` arm and the config
  bridge; the **owner root** by the crate's meta-test asserting a descriptor
  exists at `<owner>/checks/<name>.gate`.

### This batch narrows a corpus, so every reader's red condition is named

Six `*.sh` files leave three kits' `checks/` directories. Causal-completeness
point 5 binds, and *"a narrower corpus can only remove violations"* is false
here in four places:

- `check-settings-paths` — **not monotone.** Its red condition is a committed
  permission grant naming a path that no longer exists, so the narrowing
  **adds** violations: every allow entry naming one of the six
  `checks/<gate>.sh` paths is stranded by its port and must be re-pointed in the
  same unit. This gate reds *because* of a port rather than falling silent
  after one, which is the mechanism working.
- `check-docs-cmd` — **not monotone**, same shape. Its red condition is a doc
  fencing a command path that does not resolve, so any doc fencing one of the
  six deleted `.sh` paths reds after the port. Six paths to grep, and the
  grep's stderr is not silenced: a `2>/dev/null` on a path grep reads a bad path
  as *no reader*.
- `check-readme-roster` — **not monotone.** Its rule is a two-way roster match,
  so deleting a `.sh` without landing its `.gate` in the same unit strands the
  kit README row in one direction and the roster entry in the other. Delete and
  descriptor land together, per member.
- `check-measured-claim` — **not monotone.** Its red condition is a marked
  sentence disagreeing with its oracle, and this batch **moves the oracle's
  value**: `ported-gate-members` goes from 59 toward 65 as members land. Every
  marked sentence stating a ported count moves with the batch, including the one
  in `native-gate-port-remaining-corpus`.
- `check-gate-fixture-coverage` — monotone and clearable by inspection: its red
  condition is a registry member with no fixture pair, all six pairs stay in
  place under `gate-tests/`, which is pruned from every live-tree walk, and the
  descriptor inherits the coverage obligation the script carried.
- `check-shellcheck` — monotone: its red condition is a lint finding, and its
  corpus shrinks by six files with no shell left to lint. Retired for a ported
  member with cause, per the conservation table.
- `check-gate-output` — clearable, and the reason is the fixture pairs: for a
  fixtured member the output-contract assertion runs in `run-gate-tests.sh`
  against real output, so removing the source-grep target ends no assertion. All
  six carry pairs.
- `check-graph` and the generated pre-commit hook — the hook is regenerated from
  the descriptors' manifests; its red condition is staleness against the
  manifests, not a count, and a green `check-graph` after each port is the
  end-to-end proof criterion 3 exists for.

## Existing sections updated

- **gate-sdk/SPEC.md §The first cohort, and the rule that selects the next** —
  gains `### The first budget batch` beneath it (delta 13), carrying deltas 2
  and 3 and nothing else.
- **gate-sdk/SPEC.md §Meta-gate conservation for the binary substrate** — delta
  12. No row is added unless the cut's re-derivation moves a member into the
  derived set; the re-derivation's verdict lands in that section either way,
  because *"a member the section does not name is red"*.
- **The six members' own SPEC sections take no substrate change, and that is a
  verdict rather than an omission.** Probed across gate-sdk/SPEC.md,
  delegation-kit/SPEC.md, context-kit/SPEC.md and doctrine-kit/SPEC.md: none of
  the six sections names its substrate, because a gate's SPEC section owns its
  **rule** and the substrate is the descriptor's business. A merging session
  that edits one of them for substrate is adding a fact the port did not change.
- **Each touched kit's README roster** — gate-sdk, delegation-kit, context-kit,
  doctrine-kit; held in both directions by `check-readme-roster`.
- **The committed permission allow-list, and any doc fencing a deleted `.sh`
  path** — the two not-monotone readers named above; each owns its delta through
  the member whose script it names.
- **`native-gate-port-remaining-corpus`** — its dated ported-count read moves
  with the batch, per the `check-measured-claim` reader above.

## Definition of Done

- [ ] **Causal completeness** — the primitive's two consumers and each ported
      member's reader set are named above; each registry-tuple field has a named
      reader at a named transition; the narrowing's four non-monotone readers are
      discharged rather than reasoned clear.
- [ ] **Merged with no information lost** — deltas 2 and 3 land as
      `### The first budget batch`; delta 12's verdict lands in the conservation
      section. No member roster is written into this SPEC.
- [ ] **Amendment deleted** — this file removed on merge; `ls gate-sdk/SPEC-*.md`
      checked at the iteration horizon, since `SPEC-budget-arm.md` is a sibling
      in flight for this same component.
- [ ] **Removals propagated** — grepped every spec, README, doc and permission
      allow-list for each of the six deleted `checks/<gate>.sh` paths; nothing
      dangles. Stderr not silenced on any of those greps.
- [ ] **Gaps filed** — cross-component gaps filed through
      `bash lifecycle-kit/bin/file-gap.sh`; a build-time causal gap resolved that
      session rather than deferred.
- [ ] **Terminal move is a demotion, not a Done move** —
      `native-gate-port-remaining-corpus`'s deliverable is the whole corpus and
      this batch is one increment, so the entry drops its `[spec:]` tag and
      returns to the deferred section under `[design-pending]`, keeping its
      `[roadmap:]` tag and its place in the public projection.
