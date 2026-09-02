# SPEC amendment: stage-rules-cut

The port disposition of **doctrine-kit's one member behind §stage-rules** —
`bin/stage-rules.sh` (51 lines) — off the shell substrate as a bridged
`Arm::Emit` member of the gate binary. One of five stated-contract port cuts in
`declaration-install-and-stage-helper-cuts`, under the port-only run
(TRAJECTORY.md §PRIORITY DIRECTIVE), the unit set composed at scope and ruled by
the **operator on 2026-09-03 over the lead relay**.

**Measured at this HEAD rather than carried from the survey**: the port oracle's
`--tree` arm reads 112 files scanned, 64 declared `no-port`, 0 temporarily held,
**48 owed**, `doctrine-kit/bin/stage-rules.sh owed lines=51` among them.

**The member is not a gate** — §stage-rules calls it *the emitter*, a `bin/` tool
whose output is a pointer block, so no `good/`+`bad/` fixture pair is owed and no
`gates.list` row or binary-less residual roster moves.

**This cut exists because a ruling landed six hours before it.** Read on ruling
(1) alone — *a cut narrows the port, never an extension point* — this file looks
exempt: it is what `CONTEXT_KIT_STAGE_RULES` resolves to. The 2026-09-03 ruling
that **a consumer's plugin on a kit seam is owed, ruling (1) reaching the seam
alone** is what settles it the other way, and delta (2) is that ruling applied.

## What changes

### (1) The cut is a singleton behind one stated contract, and the section's owed set is all of it

`doctrine-kit/bin/stage-rules.sh` declares `doctrine-kit/SPEC.md §stage-rules` in
its own line-2 `# spec:` header, and **no other tracked non-test `.sh` declares
that section** — read off the oracle's own roster rather than inferred
{design-bearing}. The section's other named file, `lib/doctrine.sh`, declares
`no-port` under a different section and a different ground. So the owed set behind
§stage-rules is one file and the cut takes all of it; the 2026-09-03 *outer bound,
never a minimum* ruling has no sequenced member to leave behind here.

### (2) Ruling (4) is what licenses this cut, and the disposition is written to its terms rather than to ruling (1)'s

`CONTEXT_KIT_STAGE_RULES` names this file, so the ruling that has to be answered
first is (1) — where a kit ships a consumer-facing plugin or config seam, that
seam's resolution, direct execution and env contract survive and only bundled
members move in-crate {design-bearing}. **The 2026-09-03 ruling bounds it away
from this file, on its own cited surfaces**: §The config-seam port disposition
reaches only files *whose whole documented purpose is to be edited*, and this
file's whole documented purpose is to run. A file this repo names as a knob's
**value** is what the seam resolves, **not the seam**.

So the 2026-08-28 literal completion predicate governs unopposed and the file
takes a per-file port. **What survives untouched is the seam itself**:
`CONTEXT_KIT_STAGE_RULES` still exists, still resolves whatever a consumer sets,
still defaults to empty in the kit template, and the block is still silently
absent when unset — exactly as `DRIFT_KIT_KPI_DIRS` still resolves a consumer
plugin after the bundled KPIs went in-crate.

**Recorded because the opposite reading is the one a reader arrives with**: this
is doctrine-kit's own shipped tool sitting in *context-kit's* knob, which makes it
look more seam-like than the five `scripts/` files the ruling was written over. It
is not — the discriminator the ruling states is *is it edited or is it run*, and
being named in a knob's default is what ruling (4) calls half (i) of an exemplar's
cause, never the cause.

### (3) `--emit-stage-rules` is a bridged `Arm::Emit` member, and both halves of that are forced

The member's contract is a **document** — one pointer line per routed rule, on
stdout — and its only failures are usage and a doctrine file it could not read,
**both already exit 2** in the shell form {design-bearing}. `Arm::Emit` collapses
every error to 2 and every success to 0, which is this member's exit grammar
exactly, so the `--emit-` spelling is available here where §The non-gate arm
forbids it to an exit-status member. It is reached through the generic
`--emit <name>` front-end composer rather than a front-end branch of its own.

**Table membership is forced by the forced-family test.** The member resolves one
knob, `DOCTRINE_KIT_DOCTRINE_FILE`, and a hardcoded top-level flag would resolve a
platform default and silently ignore every consumer override. Its declared roster
is that one knob.

**The argv question is answered rather than assumed.** The tool takes one required
positional `<stage>` and one optional `[doctrine-file]`. The optional positional
is a **config redirection**, the shape §stage-rules describes as "the same
positional override the gate and installer do" — and gate-sdk's distinguishing
test sends a positional that redirects resolved config to the knob rather than to
argv. It is nevertheless **kept**, because the sibling surfaces it exists to match
(`install-doctrine.sh [agent-file [doctrine-file]]` and the gate) still take
theirs, and dropping it here alone would break the symmetry §stage-rules states in
one sentence with them. The arm reads the knob when the positional is absent, and
the positional when present — the shell precedence, unchanged.

### (4) The seam's env contract widens from a path to a command, and that is the cut's real design content

`CONTEXT_KIT_STAGE_RULES` is documented as "**path** to a stage→craft-rule pointer
emitter", and this repo's hook copy consumes it as one: `scripts/session-context.sh`
guards on `-f "$STAGE_RULES"` and then runs `bash "$STAGE_RULES" "$stage"`
{design-bearing}. **A compiled arm has no path to `-f` and is not a script `bash`
can run**, so the knob's contract cannot survive verbatim.

Ruling (4)'s own words say which way it moves: porting such a file "moves its
mechanism into the binary and re-points the value; **the knob still takes any
consumer command**". So the knob's contract becomes a **command**, the shape
`CANON_KIT_ENUM_SETS_CMD` already carries in this tree: the hook drops the `-f`
existence guard, and invokes the resolved command with the stage appended. This
repo's consumer copy re-points its default from
`doctrine-kit/bin/stage-rules.sh` to the front-end invocation of the arm; the kit
template's default stays **empty**, so a consumer that set nothing is unaffected
and the block stays silently absent for them.

**The honest limit, stated rather than reasoned away.** A consumer whose config
holds a bare script path is running a command of one word and keeps working; a
consumer whose config holds a path to a *non-executable* script — which is what
this repo's own default was, run under an explicit `bash` — stops working, because
the `bash` prefix the hook used to supply is gone. That is a real narrowing for
one shape of existing config, it is what makes the knob able to name a compiled
arm at all, and the migration is one line in a consumer's config. It rides
context-kit's own §Layout and configuration knob bullet, where a reader looking
for the contract will be.

**Whether the knob is renamed: no.** `CONTEXT_KIT_STAGE_RULES_CMD` would be the
tidier spelling and is refused. A rename costs every consumer a config edit for a
contract that widens rather than changes subject, and it strands the knob's
citations in two SPECs and a template; widening a knob's documented contract in
place is the change the tree already makes elsewhere for this reason.

### (5) The silent-degradation hazard is this cut's largest, and no reader reds on it

The hook's invocation is `rules_block="$(bash "$STAGE_RULES" "$stage" 2>/dev/null)" || true`
behind an `-f` guard {design-bearing}. **Every failure mode is swallowed**: a
missing file skips the block, a non-zero exit is discarded, and stderr goes
nowhere. So a port that ports the tool and leaves the knob stale produces a
session brief with the craft-rule block **silently missing**, and nothing in the
battery reds.

The red-condition sweep in §Producers and consumers below is where each reader is
enumerated; the finding is that **none of them reaches this**. Three consequences
are written into the cut rather than left to the build:

- The knob re-point lands in the **same commit** as the deletion, never a
  follow-up.
- The Definition of Done requires the block **observed rendering** from a real
  hook run, not read off the diff. This is the one assertion in the cut that
  cannot be replaced by a gate.
- The `2>/dev/null … || true` swallow is **preserved verbatim**. It is the
  documented behavior of an advisory step — "silently absent when the emitter is
  unset or the stage routes no rules" — and repairing it inside a port is fixing
  the rules the port carries. Its cost is filed as a gap rather than absorbed.

### (6) The craft-section heading moves in-crate as kit mechanism, and the provenance seam holds

The shell form pins `CRAFT_SECTION="## Engineering-craft rules"` with its own
`# spec:` comment declaring the heading "kit mechanism (the kit ships
`DOCTRINE.md`), never config" {design-bearing}. The literal moves into the crate
module unchanged and stays kit mechanism.

**The seam this cut must not cross is a different one and it is untouched.** What
crosses to a consumer is the *rule content* — the rule numbers, names and
`*Stages:*` routings — and all of it stays in `DOCTRINE.md`, read at run time. The
arm bakes **no stage vocabulary**: it compares each parsed `*Stages:*` token
against the `<stage>` argument its caller supplies, so a consumer's renamed stage
set produces no routing rather than wrong routing, and no kit literal spells any
project's stages. That is the property §stage-rules already states as its honest
limit, and the port preserves the limit rather than resolving it.

### (7) The graceful-unknown-stage behavior is preserved verbatim, because a Rust rewrite would not spell it

`stage-rules.sh <unknown>` exits **0 with empty output**, and §stage-rules calls
that graceful by design and states its ground: "a consumer with a renamed stage
set gets no routing rather than wrong routing" {design-bearing}. An absent craft
section is the same — empty output, exit 0. Only a missing doctrine file and a
missing `<stage>` are exit 2.

**This is an explicit instruction, not an omission to be inferred.** The
idiomatic Rust write of a stage lookup that matches nothing is an error or a
diagnostic, and either would close a stated honest limit inside a port. The
disposition is the same one §upgrade-smoke's worktree predicate took, and it is
the third member of that class.

### (8) Criterion 6 is discharged by construction, and criterion 2's oracle is a constructed scenario

**Criterion 6.** `doctrine-kit/lib/doctrine.sh` is permanently `# no-port:` as the
config bridge's sole resolver for the `DOCTRINE_KIT_*` knobs, and this member's one
knob crosses the bridge {mechanical}. So the value is computed in exactly one
place — the kit's shell library — and the crate holds no default to drift. The
duplication is not machine-held; it is **absent**, the qualification's strongest
form. The library keeps its other sourcers (the installer and the gate) and its
caller set does not empty, which is why the road here is the bridge rather than
deletion.

**The enumeration criterion 6 demands was done.** The member sources exactly one
library, `lib/doctrine.sh`, and reaches no other helper; the set to enumerate is
that one and its disposition is above.

**Criterion 2.** The member has no fixture pair and owes none. Its discharge is the
`# no-fixture:` road — *the same cases, both substrates, while both implementations
exist* {mechanical}. Concretely: the tree's own `DOCTRINE.md` driven through both
holders for **every** stage in the roster plus at least one unknown stage, compared
on stdout bytes and exit status; plus a scratch doctrine with **no craft section**;
plus a missing doctrine file; plus a missing `<stage>`. Bought once, at port time,
with that road's standing limit — it proves the two agree then and nothing
machine-held keeps them agreeing after, which is why the shell original is
**deleted** rather than left running beside the arm.

**Criterion 5, and its residual is narrow and real.** A vendored consumer on a host
the artifact roster does not cover loses the craft-rule pointer block outright,
where today a shell script gives it to them. It is advisory output in a session
brief rather than a gate, so the loss is a smaller brief and never a broken
battery — and it is *invisible*, by delta (5), which is what makes it worth
stating rather than filing under the class's usual terms. No `gates.list` row is
omitted and the binary-less residual roster does not move, this cut carrying no
gate.

### (9) Every surface the deletion stales moves in the landing commit, probed rather than assumed

{mechanical}

- **`scripts/session-context.sh`** — this repo's consumer hook copy: the
  `CONTEXT_KIT_STAGE_RULES` default re-pointed at the arm, the `-f` guard dropped,
  the invocation changed from `bash "$STAGE_RULES" "$stage"` to the resolved
  command with the stage appended. It is `# no-port:` and stays so.
- **`context-kit/templates/session-context.sh`** — the kit template, whose default
  is and stays **empty**; only its invocation shape moves, in lockstep with the
  consumer copy. `check-template-copy-parity` is the reader that makes editing one
  without the other red.
- **`context-kit/SPEC.md §The session-context hook`, step 8** — "when
  `CONTEXT_KIT_STAGE_RULES` names a **present emitter** (doctrine-kit's
  `stage-rules.sh`)" becomes a command, and the parenthetical names the arm.
- **`context-kit/SPEC.md §Layout and configuration`** — the knob bullet's
  "**path** to a stage→craft-rule pointer emitter" becomes the widened command
  contract, with delta (4)'s honest limit stated on it.
- **`doctrine-kit/README.md`** and **`docs/doctrine-kit/index.md`** — each carries
  the invocation as copy-pasteable text; `check-docs-cmd` assertion A is the reader
  that reds on a repo-relative `.sh` path in invocation position resolving to no
  tracked file, so these are forced rather than optional.
- **`gate-sdk/bin/run-gates.sh`'s `--help` text** — where a bridged arm's usage
  lives; the class rules that a per-arm help flag would be a second home for one
  sentence.
- **`.claude/settings.json`** — **no edit owed, probed rather than assumed.** No
  grant names `stage-rules.sh`, so the 2026-08-29 settings-grant carve-out is
  exercised **zero times** by this cut, and the ported invocation is already
  covered by the existing `run-gates.sh` glob grant. That ruling demands the count
  be probed; this is the probe.
- **No `.gate` descriptor is touched** — none couples this path, checked rather
  than assumed.

### (10) The regeneration fan-out this cut stales

{mechanical} Each is rostered with its trigger and regen command in
`docs/site-architecture.md` §Generated projections and discharged in the landing
commit:

- **The generated `pre-commit` and `commit-msg` hooks** — the baked
  `check-measured-claim` values carry `tree-shell-owed`, which this deletion moves
  (`bash gate-sdk/bin/gen-pre-commit.sh --write`).
- **`docs/check-graph.html`**, asserted fresh with them by `check-graph`.
- **`docs/doctrine-kit/SPEC.md`, `docs/doctrine-kit/README.md`,
  `docs/context-kit/SPEC.md`**, the generated on-site mirrors, on the section and
  README rewrites (`check-docs-mirror-fresh`).
- **The gate binary**, `bash gate-sdk/bin/build-native.sh`, held current by
  `check-gate-binary-fresh`. The staging order is stated because both it and the
  hooks derive through `git ls-files`: stage the deletion first, regenerate second.
- **`docs/footprint.md`** — its measured set is narrower than "any kit file" and
  contains nothing under `bin/`; whether this cut's `templates/` edit reaches it is
  **checked against context-kit/SPEC.md §bin/footprint in the landing commit**
  rather than assumed either way, that misreading being the standing mistake here.

### (11) The host is the composer entry, on the rule's own default branch

`native-gate-port-remaining-corpus` hosts this cut {design-bearing}. §Porting a
gate to the binary substrate rules that "the composer entry hosts a cut that has no
other host; an entry already waiting on this cut's subject is the better host
rather than the leftover one" — and a sweep of the whole queue finds **no entry
naming `stage-rules.sh` at all**, in any section. `doctrine-kit/SPEC.md
§stage-rules` names no queue entry and sequences itself behind nothing either. So
the default branch is not a leftover here; it is the only branch that applies.

**Two arithmetics are done here rather than met as a red at build.**

- **The lead line fits.** The entry's fixed lead-line part is 66 columns and
  `check-queue-wrap`'s floor is 100; a tag costs `9 + len(basename)`, and
  `SPEC-stage-rules-cut.md` is 22, for 97 columns.
- **The `ruled:` line is already there and costs nothing.** The entry carries
  `ruled: native-gate-port-remaining-corpus operator 2026-09-03 lead-relay`, landed
  with this iteration's three earlier rulings; the unit-set ruling this promotion
  records shares that exact authority, date and channel, so the declaration is
  idempotent and no line is added. Stated because the alternative — a second line
  with the same tuple — is what a session following the grammar mechanically would
  write, and this entry cannot afford it.

**The demotion re-prices the entry and the headroom is one line.** Its deliverable
is a corpus and this cut is not even one increment of it, so the terminal move is a
**demotion** to `## Deferred` under `[design-pending]`, which lands it back inside
`check-queue-entry-budget`'s per-entry cap where a Done move would not
(canon-kit/SPEC.md §Merging an amendment). Measured: 50 raw lines carrying five
`ruled:` lines against a discount of at most one line per grammar, so counted **49**
against `QUEUE_KIT_ENTRY_LINE_CAP` = 50 — **one line**. The entry takes no body
write from a cut by its own 2026-08-28 ruling, so one line is enough; any narrative
a build wants to add there is paid by compressing spent narrative in the same
commit that demotes it.

## Producers and consumers

The amendment introduces **one interface** — the `--emit-stage-rules` arm — and
**no new state, no new event and no new field.** It **widens one existing knob's
contract** (`CONTEXT_KIT_STAGE_RULES`, from a path to a command) and adds no knob.

- **Producer of the arm** — `native/src/emit/mod.rs`'s `BRIDGED_ARMS` table, one
  row carrying `--emit-stage-rules`, `Arm::Emit` and the one-knob roster
  `DOCTRINE_KIT_DOCTRINE_FILE`. The enabling config is the table row itself:
  `--knobs` publishes the roster and `gate_knob_env` resolves it by sourcing
  `doctrine-kit/lib/doctrine.sh`, so nothing must be configured per install and no
  default moves — the library already defines the knob, which is what makes the
  resolution succeed rather than refuse with exit 2.
- **Producer of the front-end reach** — `gate-sdk/bin/run-gates.sh`'s existing
  generic `--emit <name>` composer. No new front-end branch is added, which is the
  difference between this member and an `Arm::Run` one.
- **Consumer of the document** — context-kit's **session-context hook**, at
  session start, once per session, resolving `CONTEXT_KIT_STAGE_RULES` and
  appending the stage derived from `CONTEXT_KIT_STATE_FILE`'s last data line. It is
  the only consumer, it is not created here, and it is re-pointed at delta (4)'s
  widened contract.
- **Consumer of the widened knob** — the same hook, at the same transition, in both
  copies: `scripts/session-context.sh` (this repo's, whose default moves) and
  `context-kit/templates/session-context.sh` (the kit's, whose default stays
  empty). `check-template-copy-parity` reads them as a pair.
- **Consumer of the moved count** — `check-measured-claim`'s oracle, transitively
  through the `tree-shell-owed` key and the resolved values the generated hooks
  bake, which is why delta (10)'s regeneration is an update target.
- **Reader that stops existing** — none. Every field the shell form emitted (rule
  number, rule name, doctrine path) is emitted by the arm in the same line shape,
  and its one reader is the block the hook prints. No field is added, so the
  every-field-has-a-named-reader test has nothing new to answer for.

**One corpus is narrowed — the tracked non-test `*.sh` tree loses one file — so
each reader's RED CONDITION is enumerated rather than its subject**
(canon-kit/SPEC.md §The causal-completeness check, point 5). "A narrower corpus can
only remove violations" is false and is the first argument a narrowing delta
reaches for.

- **`check-docs-cmd` assertion A** — reds on a repo-relative `.sh` path in
  invocation position inside a fence in a governed doc that resolves to no tracked
  file. **Not monotone**: the deletion *adds* violations, at `doctrine-kit/README.md`
  and `docs/doctrine-kit/index.md`. Cleared by delta (9), in the same commit.
- **`check-docs-cmd` assertion B** and **`check-kit-ref-liveness` leg (b)** — each
  reds on a kit-prefixed ALL-CAPS token occurring in no tracked kit code, the
  second over every tracked file. **Clear**: this cut moves no knob default, and
  `DOCTRINE_KIT_DOCTRINE_FILE` keeps its home in `lib/doctrine.sh`, which the cut
  does not touch. This is the trap §upgrade-smoke's port met and it does not arise
  here, precisely because the surviving library is the bridge's resolver.
- **`check-template-copy-parity`** — reds when a kit template and its consumer copy
  diverge outside the declared gap. **Not monotone**: editing one hook copy without
  the other reds. Cleared by delta (9) editing both.
- **`check-knob-citation`** — reds on a knob mentioned in prose with no citation to
  its owning section, and on an owning section that does not state the knob.
  **Not monotone** under delta (4)'s contract rewrite. Cleared by editing
  `context-kit/SPEC.md §Layout and configuration`'s bullet, which is where the
  contract is stated.
- **`check-knob-default-coupling`** — reds on a literal knob default disagreeing
  across its sites or with its owning SPEC. **Not monotone**: this repo's consumer
  default changes and the kit template's does not, and the SPEC bullet states the
  kit default. Cleared by delta (9) moving all three together.
- **`check-graph`** — reds when a committed generated hook or the graph artifact
  differs byte-for-byte from its generator's output. **Not monotone**: the deletion
  moves the baked `tree-shell-owed` value. Cleared by delta (10).
- **`check-docs-mirror-fresh`** — reds on a `docs/` mirror not byte-equal to its kit
  source. **Not monotone** on the section rewrites. Cleared by delta (10).
- **`check-gate-binary-fresh`** — reds when the committed binary's source stamp
  disagrees with the crate's. **Not monotone**: the crate changes. Cleared by the
  rebuild in delta (10).
- **`check-spec-pointer`** — reds on a `# spec:` header naming a section that does
  not exist. **Clear**: the deleted file takes its pointer with it and the section
  it named survives, restated.
- **`check-settings-paths`** — reds on a literal repo-relative `.sh` grant that does
  not resolve. **Not monotone in general — a port is the event this gate exists for
  — but clear here**, probed in delta (9). The trap rides with it: the generated
  hook matches staged `ACMR` paths, so a *deleted* `.sh` never fires the trigger and
  only a whole-tree battery run would catch a stranded grant.
- **`check-measured-claim`** — reds when a `measured:` marker disagrees with its
  oracle or names a key the oracle does not emit. `tree-shell-owed` moves; whether a
  tracked marker binds it is **checked by scanning the markers** in the landing
  commit rather than assumed.
- **`check-shellcheck`, `check-exec-bit`, `check-comment-tier`,
  `check-path-dialect`, `check-tree-terms`, `check-gate-substrate-parity`,
  `check-gate-exemption-tasks`, `check-evidence-baseline`** — each verdict is
  **monotone** in the narrowing: removing one `.sh` can only remove findings, the
  shellcheck corpus is nowhere near empty, no `.gate` roster moves, the file
  declares no disposition field, and no validate suite's name changes. Cleared by
  inspection.
- **The reader delta (5) names, and this is the finding rather than a row** — **no
  gate reads whether the hook's craft-rule block still renders.** The invocation
  swallows stderr and discards a non-zero exit behind an existence guard, so a
  stale knob is green everywhere. This is why the Definition of Done carries an
  observed run.
- **On the addition side**, `check-comment-tier` and `check-path-dialect` reach the
  new crate module from the commit it lands in.

**Cross-component signal: this amendment's component set is three** — doctrine-kit
(the ported member and its section), context-kit (the seam's contract, its SPEC and
its template) and the consumer surfaces under `scripts/`. So `check-stage-entry`
assertion C fires on the amendment-files-span-two-components arm and the **align
stamp is demanded at the build stage's entry**. Stated here so the build session is
not the one that learns it.

## Existing sections updated

- `doctrine-kit/SPEC.md §stage-rules` — restated for the arm: the invocation form
  and its `--emit` front-end reach, the one-knob declared roster, the two positional
  arguments and why the optional one is kept, the craft-section heading as an
  in-crate literal, and every stated honest limit preserved — the unknown-stage
  empty output, the absent-craft-section empty output, and the deferred stage-remap
  knob. The `## stage-rules` **heading is unchanged**, its citations in
  `context-kit/SPEC.md` and both READMEs resolving against it (deltas 1, 2, 3, 6, 7
  and 8).
- `doctrine-kit/SPEC.md §lib/doctrine.sh` — no ruling changes; named because delta
  (8)'s criterion-6 discharge rests on its sole-resolver declaration and a reader
  checking that discharge arrives from there (delta 8).
- `context-kit/SPEC.md §The session-context hook`, step 8 — the emitter is a
  command rather than a present file, and the parenthetical names the arm; the
  silently-absent behavior and the drift-line seam precedent are unchanged (deltas
  4 and 5).
- `context-kit/SPEC.md §Layout and configuration`, the `CONTEXT_KIT_STAGE_RULES`
  bullet — the contract widens from a path to a command, the kit default stays
  empty, and delta (4)'s honest limit for a consumer holding a non-executable path
  is stated here, where a reader looking for the contract will be (delta 4).
- `scripts/session-context.sh` and `context-kit/templates/session-context.sh` — the
  invocation shape in both copies and the default in the consumer's, edited together
  because `check-template-copy-parity` reads them as a pair (deltas 4 and 9).
- `doctrine-kit/README.md` and `docs/doctrine-kit/index.md` — the copy-pasteable
  invocation, forced by `check-docs-cmd` assertion A rather than optional (delta 9).
- `gate-sdk/SPEC.md §The non-gate arm` — the bridged-arm roster gains
  `--emit-stage-rules`; no ruling changes, and the `Arm::Emit`-versus-`Arm::Run`
  test gains a member on the emitting side, which the class's worked instances are
  currently thin on (delta 3).
- `gate-sdk/bin/run-gates.sh`'s `--help` text — the arm's usage line, the class's
  stated home for it (deltas 3 and 9).
- `docs/site-architecture.md` — no ruling changes; named because delta (10)'s
  fan-out is read off it (delta 10).
- `TASK-QUEUE.md`, the `native-gate-port-remaining-corpus` entry — promoted into
  `## New Features` with `[design-pending]` swapped for this amendment's `[spec:]`
  ref at 97 columns, adding **no** `ruled:` line because the tuple it would declare
  is already on the entry. It **demotes** at build rather than reaching `## Done`,
  re-priced against a cap it clears by one line (delta 11).

<!-- update-target-exempt: the composer entry takes no BODY write from a cut by its own 2026-08-28 ruling — each closed cut's record lives in the contract section that cut selected, which is delta 1's section; the promotion above is a tag move, not a body write -->
- `TASK-QUEUE.md`, `native-gate-port-remaining-corpus`'s body — deliberately
  unwritten.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named reader
      at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls doctrine-kit/SPEC-*.md`), the none-remain half discharged at
      the **iteration** rather than at the commit, this iteration carrying sibling
      amendments.
- [ ] **Removals propagated** — grepped every spec, README, template, config file,
      settings file and doc for the deleted path; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt
      tasks, delta (5)'s swallowed-failure hazard included (a build-time causal gap
      is resolved that session, not deferred).
- [ ] **The oracle moved by the roster, not by the trailer** — the `--tree` arm
      lists no `doctrine-kit/bin/stage-rules.sh` row, taken as a per-file roster diff.
- [ ] **Parity was executed with both substrates alive** — every stage in the roster
      plus an unknown stage plus an absent craft section plus a missing doctrine file
      plus a missing `<stage>`, compared on stdout bytes and exit status, *before*
      the shell file was deleted.
- [ ] **The graceful limits still hold** — an unknown stage and an absent craft
      section each exit 0 with empty output from the arm, and §stage-rules' honest
      limit is unchanged and unclosed.
- [ ] **The block was OBSERVED rendering** — a real session-context hook run against
      the re-pointed knob prints the current stage's craft-rule block. Read off a
      run, never off the diff: no gate reds on a stale knob, which is delta (5).
- [ ] **Both hook copies moved together** — `check-template-copy-parity` is green,
      and the kit template's default is still empty.
- [ ] **The regeneration fan-out is discharged in the landing commit** — the two
      generated hooks, the graph artifact, the three `docs/` mirrors and the gate
      binary, staged before regenerated; `docs/footprint.md`'s trigger checked
      against its owning section rather than assumed.
- [ ] **The host entry demotes inside its cap** — `native-gate-port-remaining-corpus`
      returns to `## Deferred` under `[design-pending]`, and its counted size is at
      or under `QUEUE_KIT_ENTRY_LINE_CAP` **in the demoting commit**.
