# SPEC amendment: tool-port

The port disposition of **queue-kit's three remaining `bin/` tools** —
`lesson-sink.sh` (28 lines), `queue-counts.sh` (35) and `queue-edges.sh` (114) —
onto the binary substrate as non-gate arms. This is the iteration's port cut
under the port-only run (TRAJECTORY.md §PRIORITY DIRECTIVE), composed at scope
and ruled Option B by the operator on 2026-08-31, relayed by the lead.

**Measured at this HEAD rather than carried from the survey**: the port oracle's
`--tree` arm reads 125 files scanned, 64 declared `no-port`, 0 temporarily held,
**61 owed**. The three members above are queue-kit's entire owed column, and
every other queue-kit file already declares `no-port` — the library, both smoke
scripts and the config template. The cut therefore takes a kit root to **zero
owed**, which no cut before it has done, and moves the completion predicate from
61 to 58.

**No ported member is a gate**, so the eleven-gate roster the Windows consumer
smoke reports is unchanged. The round-7 reading its sibling amendment
(`gate-sdk/SPEC-generator-cause.md`) buys is nonetheless **not diff-free** — it
comes off a binary that also carries these arms — and the entry recording that
finding already says so.

## What changes

### (1) The three members are one cut, and the ground is stated because the composer demands it

`native-gate-port-remaining-corpus`'s composer ruling of 2026-08-28 selects a cut
**by stated contract** — the owed files behind one specification section — and
refuses size- and kit-ordered composers, on the ground that a cut assembled for
convenience *averages* grounds its members do not share {design-bearing}. The
three members share one ground and it is not their kit:

- **They are one class with one section that already ruled it.** Each is "a
  tool, not a gate (no `# graph:` manifest)" in its own words, and
  §bin/queue-edges.sh states its own membership — "following the queue-index
  precedent". §The queue-index arm is the section that ported the **first**
  member of this class and settled every question the remaining three raise:
  which knobs cross the bridge, that the derived shell regexes "were never a
  configuration surface" and so do not cross, that the modes ride the arm's own
  argv tail, and that stdout grammar is byte-preserved while an error path's
  exit code collapses to 2. That section is the one this cut's ruling lands in.
- **They resolve against primitives already in the crate.** `native/src/queue.rs`
  is queue-kit/lib/queue.sh's Rust counterpart and already carries `Sections`,
  `live_slugs`, `first_bold_slug`, `heading_name`, `bullet_slug` and
  `is_section_line`. `Sections::task_sections()` composes active ∪ deferred ∪
  configured-icebox — **the same composition** `QUEUE_TASK_SECTIONS` is built
  from in the shell library, verified line by line rather than assumed. So the
  cut adds no derivation; it adds readers of one that exists.

**The whole-column fact is a consequence, not the selector — ruled well-formed
2026-08-31 by the lead on own authority, against the composer read directly.**
That queue-kit's owed column happens to fit one cut is how the *candidate* was
found; it is not why the cut is well-formed, and the distinction is load-bearing
rather than pedantic because **the coincidence cuts the wrong way**. The framing
that reached the operator for Option B was kit-wise — "the only kit whose whole
owed column fits one cut" — and a record that states that fact as the *ground*
reads as exactly the kit-ordered composer the 2026-08-28 ruling refuses, with no
way for a later reader to tell the coincidence from the reason. So the ground is
stated as what it is: **§The queue-index arm's stated contract**, which settles
the front-end requirement, the bridged-knob rule, the
derived-regexes-were-never-a-configuration-surface finding, the argv-tail modes,
and byte-preserved stdout with error paths collapsing to exit 2 — every question
the other three members raise. Membership is stated in the members' own sections
rather than inferred from the kit root. The test would still be met if a fourth
queue-kit tool were owed, or if one of these three sat in another kit; and the
kit reaching zero owed is a **result** of taking the class, recorded above as a
measurement rather than offered here as a reason.

### (2) `--emit-queue-counts` — the section tally, in the `--emit-` family

`bin/queue-counts.sh` becomes an `Arm::Emit` member of `BRIDGED_ARMS` spelled
`--emit-queue-counts`, reachable through the shipped front-end as
`run-gates.sh --emit queue-counts [queue-file]` with no front-end change
{mechanical}. Its declared reads are `QUEUE_KIT_QUEUE_FILE`,
`QUEUE_KIT_ACTIVE_SECTIONS`, `QUEUE_KIT_DEFERRED_SECTION` and
`QUEUE_KIT_ICEBOX_SECTION` — the four `Sections::active_and_deferred()` resolves,
and `QUEUE_KIT_DONE_SECTION` deliberately not, because Done is not a task
section and the arm must not acquire a read it does not make.

The stdout grammar is byte-preserved: one `<section-name><TAB><count>` line per
task section, in configured order, counting the **top-level entry bullet** and
not lines or nested bullets. The optional `[queue-file]` positional ports
unchanged, on §The non-gate arm's distinguishing test — it is an argument the
rule itself consumes, falling back to a knob, the shape `check-amendment-queue`
and `check-evidence-manifest` already carry through their ports. The `-h/--help`
arm does not port: usage for a bridged arm lives in `run-gates.sh`'s own help and
in queue-kit/README.md, which is where the class already keeps it, and a per-arm
help flag would be a second home for one sentence.

### (3) `--emit-queue-edges` — the citation aggregator, git walk included

`bin/queue-edges.sh` becomes `--emit-queue-edges`, an `Arm::Emit` taking the same
`[--inbound <slug>] [queue-file]` argv tail off the emitter type's argv slice —
the mechanism §The queue-index arm already uses for its three modes
{design-bearing}. Its declared reads are delta (2)'s four knobs. Four properties
are the contract and each survives explicitly, because each is a place a
reimplementation would quietly differ:

- **The retired set is derived from history, and its degradation stays silent-safe.**
  The arm walks `git log -p --format= -- <basename>` through `proc::run`, the
  crate's one spawn site, and declares `git` in the requirement element `--needs`
  prints. Absent git, a file outside a work tree, and a file git has no history
  for all still yield the empty set and today's output exactly.
- **Citation attribution is to the nearest preceding slug bullet**, with the lead
  line yielding its `[blocked-by:]` tag alone and never its prose.
- **A token in neither the live nor the retired set is silently not an edge**,
  never a complaint — so silence means "no inbound edges" and nothing else.
- **Retired targets sort alphabetically** after the queue-ordered live block,
  because a retired slug has no queue position to order by.

**One observable moves, and the ruling that moves it is inherited rather than
retaken.** A `--inbound` slug that is neither live nor retired exits **1** today
and will exit **2**, because `EmitFn` returns a `Result` and the dispatcher maps
every error arm to 2. That is precisely §The queue-index arm's `--extent`
finding — "preserving the old code would mean widening the class's return
contract for one mode's error path; no caller reads it … so the widening is not
taken" — and this arm's callers are the same kind: stage steps in
`lifecycle-kit/templates/stages/scope.md` and `close.md`, read by a session.
Every mode's stdout grammar is byte-preserved.

### (4) `--lesson-sink <tag>` — an `Arm::Run`, and the spelling is forced

`bin/lesson-sink.sh` becomes a **top-level `Arm::Run` member** spelled
`--lesson-sink`, beside `--statusline` and `--usage-poll`, with a matching
three-line case in `gate-sdk/bin/run-gates.sh` {design-bearing}. The `--emit-`
family is **refused** for it, and the refusal is a contract question rather than
a naming preference: §bin/lesson-sink.sh rules that "the command's exit status
becomes the tool's, so a failing sink is a red close step and harvest material is
never half-routed to a silent fallback", and `Arm::Emit` collapses every error to
exit 2 and every success to 0. An emitting arm therefore *cannot* carry this
member's stated contract. `Arm::Run` returns the code, which is the whole reason
the bridged-arm table is keyed by flag rather than by family.

`ARM_UNAVAILABLE_STATUS` for the new case is **2**, matching `--usage-poll` and
not `--hook`/`--statusline`: this arm gates no tool call and its caller is a
close-stage step whose failure must be visible, which is the discriminator
§The non-gate arm already states for that status.

Three behaviours are named because the port moves each of them:

- **The seam survives; only the resolver moves in-crate.** `QUEUE_KIT_LESSON_SINKS`
  stays the adopter's configuration and a configured entry still runs as a
  **command** — `bash -c "<value>"` with the body on its stdin — so a sink may
  still reformat into a downstream grammar and a private sink value still lives
  in the `queue-config.local.sh` overlay. This is the composer's ruling (1)
  applied: the cut narrows the port, not the extension point. The map crosses the
  bridge through `walk::knob_map`, the keyed arm, and gate-sdk/SPEC.md already
  records that this specific knob "becomes portable now" on exactly that arm —
  the one blocker this member had, retired before the cut was composed and named
  in the spec so a later selector would not re-derive it. `bash` joins `--needs`.
- **`exec` becomes spawn-and-report.** The shell form `exec`s, so the sink's
  status *is* the tool's by process replacement. The arm spawns through
  `proc::run_with_stdin` and returns the child's code, with a signal-killed sink
  reporting `128 + n` through `proc.rs`'s existing `exit_code` spelling — the
  same one `gate-sdk/SPEC-generator-cause.md` delta (2) reaches for, so the two
  amendments must not mint two spellings of it.
- **stdin is buffered rather than streamed.** The shell form hands the child an
  inherited descriptor; the arm reads the body to completion first. A lesson body
  is one queue entry's prose, so the bound is the queue's own per-entry cap, and
  this is recorded rather than absorbed because it is the one place the port
  changes what an arbitrarily large input would do.

The unconfigured-tag fallback stays **open** — append to
`<workflow-dir>/<tag>-harvest.md` — preserving the fresh-clone close path and the
staging file's documented reclaim path. One tightening rides with it: the shell
form spells its own `${GATE_SDK_WORKFLOW_DIR:-.workflow}` default, while the arm
resolves the knob through the bridge, where unset is an error. The knob has a
shipped default, so no configured consumer moves; an adopter who deleted it from
their config gets a refusal instead of a silent write to `.workflow`, which is
the better failure and is stated so it is not read as a regression.

### (5) The statusline stops spawning a shell, and the subprocess contract retires with its own ground

`native/src/hook/statusline.rs` spawns `bash <root>/queue-kit/bin/queue-counts.sh`
today. The cut deletes that script, so this caller must move, and **the shape it
moves to is a design decision two governed sentences currently forbid**
{design-bearing}. **Ruled 2026-08-31 by the lead on own authority**: it calls the
counts rendering **in process** and maps an `Err` to an empty counter group, and
both sentences retire with their ground. The ruling is envelope-class and
reverses nothing recorded — neither sentence carries a `ruled:` marker or an
authority attribution; they are SPEC design clauses with stated grounds, and
retiring a clause whose stated ground the port has removed is what a port cut
does.

The two sentences and why both retire together:

- delegation-kit/SPEC.md §The statusline arm: "**It is a subprocess call, and
  that is the contract, not an implementation detail.** The counter's library
  exits 2 on a malformed queue config; calling it in-process would take the whole
  status bar down."
- queue-kit/SPEC.md §bin/queue-counts.sh: "**It is invoked as a subprocess, never
  sourced.** `lib/queue.sh` exits 2 at source time on a missing
  `QUEUE_KIT_CONFIG_FILE` and on any malformed-config assertion."

**Both state the same ground and the ground is a bash fact.** The hazard is a
*sourced shell library* calling `exit` in the caller's own process. The ported
rendering resolves its knobs through `walk::knob_scalar`/`knob_array`, which
return `Result` and cannot exit; a malformed queue config becomes an `Err` the
caller absorbs. The ruling comes back **uniform across both sections**, and the
uniformity is the finding: the ground is either right for both or right for
neither, and it is a property of the substrate rather than of the arm.

**Everything the sentences protect is preserved.** An unresolvable queue file, a
malformed config and empty output all still drop the counter group and change
nothing else about the bar, matching the two degradations the arm already
performs. The isolation argument's remaining force is already spent in any case:
`hook/statusline.rs` imports `crate::emit::kpi` and renders it in process, so the
bar has never been isolated from crate code — it was isolated from *shell*, and
that is what the port removes.

**One consequence is a declared-reads change and it is not a hidden cost.**
`--statusline`'s roster gains delta (2)'s four `QUEUE_KIT_*` knobs, because the
subprocess used to resolve its own and an in-process reader resolves through the
bridge. This *improves* on the state it replaces — knob resolution stops
happening twice by two mechanisms and goes through the single producer criterion
6 names. The property §The statusline arm actually cares about is untouched: the
section vocabulary still arrives as returned names, so there is still nothing
here for a later editor to hardcode.

**Where those four knobs resolve on the in-process path, and what a consumer
override does — stated because §The queue-index arm's *The front-end is not
optional dressing* does not reach this caller.** That paragraph settles the
front-end requirement for a caller reaching an arm *as* an arm, and names its own
hazard: a caller that "invoked the binary directly would resolve platform
defaults and silently ignore every consumer override", the worked instance being
an arm that cannot see `QUEUE_KIT_ICEBOX_SECTION` and "silently drops the tally
in every consumer that configures a tier". An in-process call from a hook module
is neither the front end nor a direct binary invoke, so it sits outside what that
paragraph settled and owes its own answer:

- **They resolve from the bridged environment `--statusline`'s own exec already
  carries.** The harness invokes `bash gate-sdk/bin/run-gates.sh --statusline`
  (`.claude/settings.json`), so the front end sources the shell library, resolves
  the arm's declared roster and execs the binary with
  `GATE_SDK_KNOB_<NAME>=…` in the environment. Adding the four names to
  `--statusline`'s roster is therefore not bookkeeping — it *is* the mechanism
  that puts them there, and it is the whole of what the in-process reader needs.
  A consumer override set in `<gates-dir>/queue-config.sh` reaches this reader by
  exactly the path it reached the subprocess: the front end sources the same
  library either way, and the override is resolved before the exec in both.
- **The silent-override-ignored failure this path could have had is structurally
  unavailable, and the reason is the crate's own rule rather than care.**
  `walk::knob_scalar` reads `GATE_SDK_KNOB_<NAME>` and returns `Err` when it is
  absent — "the crate holds no default" — so an unbridged or partially-bridged
  invocation cannot substitute a platform default for a consumer's value. It has
  no default to substitute. The reader errors, `Err` maps to an empty counter
  group, and the group **vanishes** rather than rendering a wrong tally.
- **So the residual failure is total and visible, never partial and wrong.** A
  roster widened to three of the four knobs, or a caller invoking
  `checkwright-gates --statusline` directly and bypassing the front end, drops
  the whole counter group and changes nothing else about the bar — which is one
  of the degradations §The statusline arm already specifies. The failure mode the
  front-end paragraph exists to prevent is a consumer's configured tier silently
  missing from a rendered tally; on this path that outcome has no spelling.

### (6) The two bespoke tests narrow to the seam a crate test cannot see

`queue-kit/gate-tests/queue-counts.test.sh` and `queue-edges.test.sh` drive the
shell tools directly today. Each narrows to the shape
`queue-kit/gate-tests/queue-index.test.sh` already took at its own port
{design-bearing}: it holds "the seam a crate unit test cannot see — that the
battery runner's `--emit` front-end resolves the arm at all, and that a set
consumer knob actually reaches the rendering through the shell bridge", while
"the rendering itself is pinned in the ported module's own `#[cfg(test)]` tests,
where `check-crate-arms` runs them". So each test keeps its **discriminating**
case — for counts, the icebox-unset case a hardcoded implementation would pass
against this repo's own config and fail only there; for edges, the
no-repository degradation and the retired-target half, both of which need a real
sandbox and a real history — and hands the grammar cases to the ported modules.

**This cut does not settle `gate-test-in-tree-invoker-ruling`, and does not need
to.** That entry asks whether a gate-test counts as an in-tree invoker for the
named-caller requirement. Every arm here has a caller that is not a test:
`--emit-queue-counts` has the statusline's in-process reader and the invocation
queue-kit/README.md documents for a session; `--emit-queue-edges` has two stage
steps; `--lesson-sink` has the close skill. Settling a live `[design-pending]`
fork inside a port cut is non-port design work and the composer refuses it, so
the fork is named here and left standing.

### (7) The path-bearing surfaces move in the same commit as the delete

Every surface naming a deleted path is repointed in the deleting commit
{mechanical}, the count probed rather than assumed:

- `.claude/settings.json` — three grants name two of the deleted paths
  (`queue-edges.sh` bare and with arguments, `lesson-sink.sh` with arguments).
  Removing a grant whose target a ruled port cut deletes is **outside** the
  2026-08-22 bar under the operator's 2026-08-29 settings-grant carve-out, and
  the removal lands in the same commit as the delete — the window that carve-out
  exists to close. Replacements naming the new invocations land with them, and
  `check-settings-paths` is the oracle on both halves.
- `.claude/commands/close.md` and `lifecycle-kit/templates/stages/scope.md` and
  `close.md` — the three stage/skill surfaces that invoke a deleted path.
- `queue-kit/README.md`'s command roster (four lines), and its docs mirror.
- `docs/site-architecture.md`'s standing-instance sentence, which names
  `bin/queue-edges.sh` as a tool with no stored projection. The **ruling**
  survives the port on the stored-projection ground alone, exactly as the
  `queue-index` ruling did; only the name changes.

### (8) `queue-lib-dead-derivation`'s premise narrows, and its verdict does not

That entry names `bin/queue-counts.sh`, `bin/queue-edges.sh` and
`queue_live_slugs` as the three live readers of `QUEUE_TASK_RE` and
`QUEUE_SECTION_RE`, and rests its **partial**-death verdict on them
{mechanical}. This cut deletes two of the three. The entry is corrected in place
to name the one surviving reader; its verdict is unchanged, because one live
reader is still a live reader and a blanket section deletion is still wrong.
Recorded as a delta rather than left for a later scope to rediscover, since an
entry whose stated premise a landed cut falsified reads as settled work.

## Producers and consumers

The amendment introduces **three interfaces** — one bridged flag each — and **no
new state, no new event, no new field, and no new knob**. Every knob named is
already shipped and already read; what changes is which process reads it.

- **Producer** — `native/src/emit/mod.rs`'s `BRIDGED_ARMS` table, one row per
  arm, plus one `run-gates.sh` case for `--lesson-sink`. The enabling config is
  the table row itself: `--knobs` publishes each member's declared roster and
  `gate_command` resolves it, so an arm's environment is emitted by the same
  bridge every other member uses and nothing must be configured per install.
  `--emit-queue-counts` and `--emit-queue-edges` need **no** front-end change,
  because `run-gates.sh` composes `--emit-<name>` from its `--emit <name>`
  operand — the property §The non-gate arm calls load-bearing rather than house
  style, and the reason the two emitting members take that spelling.
- **Consumer of `--emit-queue-counts`** — two, at two transitions. The
  **statusline arm** calls the rendering in process at each statusline fire
  (delta 5), and a **session** invokes the arm through the front-end at the
  command queue-kit/README.md documents. A session reaching a mode through the
  front-end counts exactly as a stage step does, which §The non-gate arm states.
- **Consumer of `--emit-queue-edges`** — the **scope stage**, which buys the
  inbound-edge sum for a promotion decision (lifecycle-kit/SPEC.md, and
  `templates/stages/scope.md`), and the **close stage**, which reads its retired
  block (`templates/stages/close.md`). Both are stage steps invoking the
  front-end.
- **Consumer of `--lesson-sink`** — the **close skill**, which streams a tagged
  entry's body through it at the lessons harvest (`.claude/commands/close.md`;
  the contract is §The lessons harvest). Its exit status is read by the close
  step itself: a failing sink is a red close step, which is delta (4)'s whole
  reason for the `Arm::Run` spelling.
- **Consumer of the declared-reads change** — `--knobs`, read by `gate_command`
  and baked into the generated hooks. `--statusline`'s roster gaining four names
  (delta 5) is visible there and nowhere else, and it is a **named reader at a
  named transition**: the front-end resolves the roster before the exec.

**Every arm has a caller that is not a test**, enumerated above rather than
asserted, which is the requirement §The non-gate arm's third property states and
the reason delta (6) can leave `gate-test-in-tree-invoker-ruling` standing.

**No corpus is narrowed and none is widened**, so §The causal-completeness
check's red-condition enumeration does not bind. The three arms read the same
queue file over the same section set as the three scripts did; the port oracle's
`--tree` corpus loses three rows by deletion, which is the completion predicate
moving in its intended direction and is read by no gate that asserts a count.
The one figure that moves — the owed count, 61 → 58 — is cited by no governed
sentence today, checked by scanning the `measured:` markers on the governed
surfaces rather than assumed.

**Cross-component signal: this amendment's component set is four** — queue-kit,
gate-sdk, delegation-kit and lifecycle-kit — so `check-stage-entry` assertion C
fires and the **align stamp is demanded at the build stage's entry**. Stated here
so the build session is not the one that learns it.

## Existing sections updated

- `queue-kit/SPEC.md §The queue-index arm` — gains the class ruling this cut
  rests on: that its own port settled the questions the three remaining tools
  raise, and that they take the same dispositions. Its *The front-end is not
  optional dressing* paragraph gains one sentence marking an **in-process call
  from a hook module** as outside what it settles, and pointing at the section
  that answers that path — so a later reader meeting the paragraph does not read
  its silence as coverage (deltas 1 and 5).
- `queue-kit/SPEC.md §bin/queue-counts.sh` — renamed to its arm and rewritten for
  the new home: the derived section set, the top-level-entry counting unit and
  the "why a second tool rather than a fourth mode" refusal all survive; the
  *It is invoked as a subprocess, never sourced* paragraph retires with its
  ground (deltas 2 and 5).
- `queue-kit/SPEC.md §bin/queue-edges.sh` — renamed to its arm; the four contract
  properties, the inbound-only refusal and the no-repository degradation survive,
  and the exit-1 → exit-2 move is recorded on §The queue-index arm's own terms
  (delta 3).
- `queue-kit/SPEC.md §bin/lesson-sink.sh` — renamed to its arm; the `Arm::Run`
  spelling and its exit-status ground, the buffered stdin, and the bridged
  workflow-dir knob are recorded beside the seam and the fail-open default that
  do not change (delta 4).
- `queue-kit/SPEC.md §lib/queue.sh` — its note that `bin/queue-edges.sh`'s
  history walk is the lead-line grammar's second reader now names the arm
  (delta 3).
- `gate-sdk/SPEC.md §The non-gate arm` — the class roster gains the three
  members, and the `--lesson-sink` row carries the one sentence the class does
  not yet hold: that a member whose stated contract is its child's exit status
  cannot take the `--emit-` spelling (deltas 2, 3 and 4).
- `gate-sdk/SPEC.md §run-gates` — the front-end's arm grammar gains the
  `--lesson-sink` case and its `ARM_UNAVAILABLE_STATUS` (delta 4).
- `gate-sdk/SPEC.md`, the keyed-map arm's `QUEUE_KIT_LESSON_SINKS` paragraph —
  its "becomes portable now" forecast is discharged and says so (delta 4).
- `delegation-kit/SPEC.md §The statusline arm` — the counter group's subprocess
  paragraph is replaced by the in-process reading and its preserved
  degradations; the arm's declared reads are stated, together with **where they
  resolve** and what a consumer override does on that path, which is the
  condition the 2026-08-31 lead ruling attached (delta 5).
- `lifecycle-kit/SPEC.md` and `templates/stages/scope.md` and `close.md` — the
  three surfaces citing the edge tool by path (deltas 3 and 7).
- `docs/site-architecture.md` — the standing-instance sentence naming the edge
  tool, whose ruling survives the port unchanged (delta 7).
- `queue-kit/README.md`'s command roster (delta 7).
- `TASK-QUEUE.md`, the `native-gate-port-remaining-corpus` entry — promoted with
  `[design-pending]` swapped for this amendment's `[spec:]` ref; it demotes at
  build and never reaches `## Done`, which its own body already rules (all deltas).
- `TASK-QUEUE.md`, the `queue-lib-dead-derivation` entry — its reader roster
  corrected to the one survivor (delta 8).
- The generated projections this cut stales — the on-site SPEC mirrors, the
  generated `pre-commit`/`commit-msg` hooks and `docs/check-graph.html` (a
  declared-reads change moves baked argv), and the gate binary itself. All are
  rostered with their triggers and regen commands in `docs/site-architecture.md`
  §Generated projections (all deltas).

<!-- update-target-exempt: the composer entry takes no write from a cut by its own 2026-08-28 ruling — each closed cut's record lives in the contract section that cut selected, which is delta 1's section -->
- `TASK-QUEUE.md`, `native-gate-port-remaining-corpus`'s body — deliberately
  unwritten beyond the tag swap.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls queue-kit/SPEC-*.md`), discharged at the iteration rather
      than at the commit, this iteration carrying a sibling amendment.
- [ ] **Removals propagated** — grepped every spec, skill, template, README and
      settings file for the three deleted paths; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **The oracle moved, and by the roster rather than by a number** — the
      `--tree` arm lists no `queue-kit/bin/` row at all and every other member of
      that kit reads `no-port`, taken as a per-file roster diff and not as a
      trailer delta.
- [ ] **The regeneration fan-out is discharged in the landing commit** — the
      generated hooks, the graph artifact, the SPEC mirrors and the gate binary.
- [ ] **Both amendments' shared file is sequenced** — `native/src/proc.rs` is
      touched by this cut and by `gate-sdk/SPEC-generator-cause.md`, and the
      `exit_code` spelling is reused rather than duplicated.
