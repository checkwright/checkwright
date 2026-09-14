# SPEC amendment: knob-files-cut-2

Queue entry: `config-seam-static-format`, its second increment. It leads unit set
`config-seam-second-cut`: **cut 2 is queue-kit plus lifecycle-kit** — operator direction,
2026-09-14, lead-relayed. The format, the precedence order, the legacy refusals and the
`--emit knob-roster` arm are cut 1's and stand (gate-sdk/SPEC.md §The knob file); this amendment
adds only what the two kits need beyond them.

Scope's shape probe recorded three questions on the entry, and this amendment rules each (deltas
2, 3 and 7). Authoring found three more that the probe did not reach, and each is ruled here too:
the shell expander's `knob:` tokens (delta 4), this repo's drift-kit config sourcing
lifecycle-kit's (delta 5), and the loader validation both libraries perform (delta 6).

## The seam

- **Kit mechanism:** the two kits' defaults tables and validators under `native/src/knobs/`, the
  bridged-input declaration and its `--knobs` closure, the `--emit knob-values` arm, the
  required-sections composition in `native/src/queue.rs`, the two comment-only config templates,
  and init's widened seeding derivation.
- **Consumer config:** each `<gates-dir>/queue-config.knobs` and `lifecycle-config.knobs` with its
  gitignored `.local.knobs` overlay. They stay the adopter's edit seam, so gate-sdk/SPEC.md §The
  config-seam port disposition's ground holds, and only the substrate changes.
- **This repo's own:** `scripts/queue-config.knobs` (the lesson tag, roadmap vocabulary and icebox
  tier) and `scripts/lifecycle-config.knobs` (the six-stage roster, the ruling record, the
  preflight commands and the lock-reason pattern). Both are consumer values and never become kit
  literals.
- **Private rule content:** the one private value in reach is the lesson sink command, which
  already lives in the gitignored overlay (queue-kit/SPEC.md §The lesson-sink arm). It moves to
  `scripts/queue-config.local.knobs` and stays untracked.

## What changes

### (1) The two kits go static {mechanical}

queue-kit and lifecycle-kit gain defaults tables, `native/src/knobs/queue_kit.rs` and
`native/src/knobs/lifecycle_kit.rs`, and join `STATIC_KITS`. Every knob, shape and default
transcribes from its library as it stands at this amendment's commit, with the exceptions deltas
2, 3 and 6 rule. Both libraries stop resolving anything and are deleted by delta 8.

Every live value in both consumer configs is already expressible in the line grammar, and none
needs quoting. Three cases are named because they look hard:

- **`LIFECYCLE_KIT_PREDECESSOR`** becomes one `LIFECYCLE_KIT_PREDECESSOR[<stage>] = <predecessor>`
  line per pair.
- **An `LIFECYCLE_KIT_ENTRY_PREFLIGHT` element** is `<stage>=<command>`. The first `=` on the line
  ends the head, so `LIFECYCLE_KIT_ENTRY_PREFLIGHT[] = close=gate-sdk/bin/run-gates.sh …` carries
  the element verbatim.
- **`QUEUE_KIT_LESSON_SINKS`** is keyed, and its value is a shell string. That is the lesson-sink
  arm's own contract, since a configured entry runs as `bash -c "<value>"` (queue-kit/SPEC.md §The
  lesson-sink arm). So the value crosses verbatim and execution is unchanged. gate-sdk/SPEC.md §The
  knob file's command-knob bullet governs `*_CMD` knobs, and this knob is not one.

Names prefix-owned by lifecycle-kit but read straight off the process environment
(`LIFECYCLE_KIT_SESSION_ID` and its siblings, lifecycle-kit/SPEC.md §bin/session-id.sh) get **no
row**. A knob file must not be able to set a stamp-id override, because every session would then
stamp one id. Their readers keep the environment read. The rationale comments at
`native/src/emit/session_id.rs` and the `--emit-session-id` row in `native/src/emit/mod.rs` are
re-grounded: *not a row in lifecycle-kit's static table*, where they currently say *not defined in
`lib/stages.sh`*.

### (2) A static default derived from a bridged knob {design-bearing}

Seven defaults derive from gate-sdk layout knobs that stay bridged until gate-sdk's cut:
`QUEUE_KIT_QUEUE_FILE` and `LIFECYCLE_KIT_QUEUE_FILE` from `GATE_SDK_QUEUE_FILE`, four lifecycle
file knobs from `GATE_SDK_WORKFLOW_DIR`, and `LIFECYCLE_KIT_STAGE_JOURNAL_PATTERN` from
`GATE_SDK_TMP_DIR`. `LIFECYCLE_KIT_PERMANENT_SURFACE_GLOBS` reaches `GATE_SDK_QUEUE_FILE` through
`LIFECYCLE_KIT_QUEUE_FILE`. A derived default today is a function of static siblings only, because
the resolver it is handed resolves only statically owned names.

gate-sdk/SPEC.md §The knob file, after the sentence that moves derived values to the defaults
table. **Not yet applied:**

> A derived default may read a **bridged** knob as well as a static sibling. Its row declares every
> name its derivation reads, so the set is known without running it. `--knobs <member>` adds the
> bridged names reachable through the declarations of the static knobs the member reads,
> transitively through static siblings. So the bridge carries a derived default's inputs by
> construction, and a member reading a derived knob never meets an absent input. The derivation
> reads a bridged input through the one reader function (§lib/gate.sh), so an absent input is the
> reader's ordinary refusal and never a silent fallback. `--emit knob-roster` renders a bridged input as
> `${NAME}`, because the arm takes no knob and gate-sdk's default for that input is not the
> crate's to spell.

Mechanism:

- `knobs::Row` gains an `inputs` list. The `Resolve` closure handed to a derivation falls through
  to `knobs::wire` for a name no static kit owns.
- `knobs::bridged` becomes the closure: the member's non-static names, plus the bridged inputs
  reachable from its static names.
- A unit test runs every derived row with a recording resolver and asserts that the names asked
  are a subset of the declared `inputs`. The closure's soundness rests on that declaration, so
  the test is what keeps it honest.

**Refused, with grounds (they stay here and in git history):** reading the gate-sdk input from the
environment with a crate literal behind it, the `GATE_SDK_GATES_DIR` locator shape. It silently
splits the two kits from gate-sdk for any consumer who relocates the workflow directory in
`gate-sdk-config.sh`. Replacing the derivation with literals has the same split and breaks
derivation-first. Migrating the three gate-sdk layout knobs now contradicts §The knob file's
*gate-sdk migrates last* rule and the prefix-ownership rule, since a kit is static whole.

### (3) The icebox joins the required set in the reader, not in the knob {design-bearing}

The shell loader appends a configured `QUEUE_KIT_ICEBOX_SECTION` to `QUEUE_KIT_REQUIRED_SECTIONS`
**after** the consumer config is sourced, so the append survives a consumer who set the list. A
static knob is replaced whole by its file, and a derived default fires only at the default, so
neither form can carry that.

`native/src/queue.rs` gains `required_sections()`: the resolved `QUEUE_KIT_REQUIRED_SECTIONS`,
followed by a non-empty `QUEUE_KIT_ICEBOX_SECTION` unless the list already names it. It is the
same one-composition rule `QUEUE_TASK_SECTIONS` follows (queue-kit/SPEC.md, the adapter section).
`check-queue-sections` reads it, and that member declares `QUEUE_KIT_ICEBOX_SECTION`. The knob's
value becomes exactly what the consumer wrote, and the roster prints that.

Refused: a derived default holding the append, because a consumer setting the list would lose it.
That is the fail-open queue-kit/SPEC.md §Layout and configuration rules against. Also refused: a
post-resolution transform row kind in the table, a new table concept for one composition a
shared adapter already covers.

### (4) The shell couples expander resolves a static `knob:` token {design-bearing}

`gate_expand_couples_var` in `lib/gate.sh` resolves a `knob:<NAME>` token from its
`GATE_SDK_KNOB_<NAME>` bridge variable, and `_gate_couples_knob_names` drops statically owned
names from the set it bridges. A `knob:` token naming a static knob therefore reds at exit 2 with
*names a knob the config bridge could not carry*. Cut 1 moved no `knob:`-named knob, so it never
met this. Cut 2 moves two: `check-scratch-citation`'s `knob:LIFECYCLE_KIT_PERMANENT_SURFACE_GLOBS`
and `check-queue-slug-liveness`'s `knob:QUEUE_KIT_PROSE_SURFACE_GLOBS`. Hook generation and
`run-gates --for` would both fail-close. The crate-side expander (`registry::expand_couples`)
already reads through `knobs::wire` and is unaffected.

gate-sdk/SPEC.md §The non-gate arm gains **`--emit knob-values <NAME>...`**. **Not yet applied:**

> It prints the **resolved** value of each named static knob in the invoking tree, in the roster's
> line grammar: `<NAME><TAB><shape><TAB><element>`, one line per element, and one line with an
> empty third field for an empty collection. A name no static kit owns, or one its kit does not
> declare, is exit 2. It is a bridged arm whose declared set is the bridged-input closure of its
> argv names (§The knob file), so a derived default resolves exactly as it does for a member.

In the shell expander, a static `knob:` token takes its members from that arm, called once per
process for the whole static token set. **Both shell consumers of the expander take this path**:
`bin/gen-pre-commit.sh` bakes the hook's trigger through `gate_expand_couples_var` (its trigger
expansion), and `run-gates --for` selects through the same function, so the generated hook and the
runtime selector cannot diverge on a static token. The arm is the only producer of the value: it
reads the crate's static resolution (`knobs::resolve`), and no bash code parses a knob file or
recomputes a default, which a second producer would be (gate-sdk/SPEC.md §The port-candidate
criteria, criterion 6). The token is never exported as a `GATE_SDK_KNOB_` variable,
which keeps *a static name is never bridged* literal. The arm's two readers are the shell expander
and delta 5's config reference.

Refused: dropping the two `knob:` tokens, because each is its gate's trigger over a consumer-owned
corpus and §The `# graph:` manifest refuses a literal in its place. Also refused: widening
`--emit knob-roster` to take names, since one arm answering *defaults* bare and *resolved values*
with arguments gives two meanings to one spelling.

### (5) This repo's drift-kit config reads the lifecycle stage roster through the binary {design-bearing}

**Option A of four, lead decision, 2026-09-14** — escalated at spec as a cross-component change to
drift-kit's adopter wiring that the unit-set premise did not foresee; ruled inside the operator's
cut-2 direction, since it adds no behaviour and keeps existing readers of the two kits working.

`scripts/drift-config.sh` sources `scripts/lifecycle-config.sh` and copies `LIFECYCLE_KIT_STAGES`
into `DRIFT_KIT_STAGES`, and drift-kit/SPEC.md §Layout and configuration documents that wiring as
the adopter's way to follow a wider roster. drift-kit stays bridged, so once lifecycle-kit is
static there is no shell file left to source. The alternatives are a legacy-refused `.sh` or a
knob file bash cannot parse.

This repo's config reads the value through delta 4's arm instead, via a `lib/gate.sh` helper,
`gate_static_knob <NAME> <array-outvar>`. The helper runs inside the bridge subshell that sources
`lib/drift.sh` (and therefore the config), and it refuses non-zero with the name on stderr. The
config calls it only when `DRIFT_KIT_STAGES` is in `GATE_SDK_RESOLVING_KNOB`, the gating
`GATE_SDK_RESOLVING_KNOB` already sanctions for a knob whose value costs a subprocess
(§lib/gate.sh). So a drift batch that never asks for the roster pays no spawn. drift-kit/SPEC.md's
`DRIFT_KIT_STAGES` bullet re-points its adopter wiring at the helper.

`gate_static_knob` computes nothing: it runs delta 4's arm and copies its third column into the
array, so the crate stays the value's one producer (criterion 6). It refuses a non-`indexed` shape
when asked for an array.

**The helper is transitional, and its removal is declared now.** It lives in `lib/gate.sh`, so it
dies with the bridge at gate-sdk's cut, which gate-sdk/SPEC.md §The knob file already names as the
cut retiring `lib/gate.sh` and its front-ends; that section's gate-sdk bullet gains
`gate_static_knob` by name, so the retirement is a declared deprecation rather than a discovery.
The stopgap itself ends earlier, at drift-kit's own cut, when drift's config becomes a knob file
and needs the reference form. That cut is already costed and filed: it is one of the remaining
increments of `config-seam-static-format`, whose entry names the knob reference as a shape a
remaining kit waits on, and §The knob file's selection rule names drift-kit as needing it.

This is a **bridged config reading a static knob**, a transitional shape that exists only while
the two seams coexist. It is not the knob-file reference form, which §The knob file still leaves to
the first cut whose *knob file* needs one: drift-kit's own cut rules that form and retires this
call.

Refused: a literal `DRIFT_KIT_STAGES` in this repo's config, which reintroduces the roster drift
the wiring exists to remove; nothing would compare the two. Pulling drift-kit into this cut is a
cut-boundary change, and its config needs the reference form this cut does not rule.

### (6) Load-time validation moves to the tables {design-bearing}

Both libraries validate on every load and exit 2 with every finding listed. queue-kit checks
emptiness, positive integers, the icebox not naming the deferred section and the roadmap
vocabulary as a pair. lifecycle-kit checks the stage-machine relations, the `0|1` switches, the
journal pattern's `<stage>` placeholder and the lock-reason pattern's compilation and capture
group. The contract to preserve is *a malformed config gates nothing*
(queue-kit/SPEC.md §Layout and configuration, lifecycle-kit/SPEC.md §Layout and configuration).

`knobs::Kit` gains an optional validator. It runs once per process per kit, at the kit's first
resolution, and collects every finding before refusing with exit 2 under the kit's existing
`malformed … config` lead line.

- **A row whose default derives from a bridged input is validated only when its value comes from
  the environment or a file.** The validator runs at the kit's first read, for a member that may
  not carry that input. A unit test holds every kit default valid under the validator, which is
  what makes skipping those rows at their default sound.
- **The lock-reason pattern still compiles under bash's own `[[ =~ ]]`,** spawned by the validator.
  lifecycle-kit/SPEC.md grounds the check in *the engine the consumer will actually run against*,
  and the crate's matcher for that pattern is itself a `bash -c` spawn (`native/src/emit/enter_stage.rs`).
- The *probe in a condition context* clause of lifecycle-kit/SPEC.md binds a shell sourcer under
  `set -e`, and no shell sourcer remains. It is deleted with the library, together with the smoke
  case written for it: `lifecycle-kit/smoke/install.sh` keeps a configured pattern, now as a knob
  file, and asserts `--install-lifecycle` still exits 0.

### (7) The first config templates ship as knob files {design-bearing}

Both kits ship a config template. `templates/queue-config.sh` and `templates/lifecycle-config.sh`
become `templates/queue-config.knobs` and `templates/lifecycle-config.knobs`. Each is comment-only:
one `#` pointer line to its kit's knob roster, with no override set, which is the starting point
§check-template-copy-parity already excludes on its *the consumer customizes it* ground. A
comment-only file is valid in the grammar. The `# no-port:` header goes with the shell substrate.

- **Init's seeding derivation widens.** installer/README.md §What init seeds reads *whatever
  `templates/*-config.sh` or `templates/*-config.knobs` it ships*, and `config_seam_plan` in
  `native/src/installer/recipe.rs` matches both suffixes. A seeded knob file is claimed before it
  is written, on the same path as a shell seam. The paragraph's *no such kit ships a config
  template yet* sentence is deleted.
- **`check-template-copy-parity`** excludes `*-config.knobs` by suffix beside `*-config.sh`, on the
  same ground.
- **`installer/consumer-smoke/run-smoke.sh`'s seam arm** edits `scripts/queue-config.knobs` in place
  of the `.sh`.
- **The upgrade consequence is the existing rule applied and needs no new mechanism.** `init` never
  deletes a file a release stopped shipping (installer/README.md §init), so an adopter who installed
  a release carrying `queue-config.sh` or `lifecycle-config.sh` keeps that copy. The reader's
  existence refusal (§The knob file) then fires naming the migration, whether or not the copy was
  edited. The release declaration (delta 11) states the remedy.

### (8) Delete both libraries and their parity machinery {mechanical}

- Delete `queue-kit/lib/queue.sh` and `lifecycle-kit/lib/stages.sh`. Every derived global, helper
  and renderer they hold already has its compiled counterpart in `native/src/queue.rs` and
  `native/src/stages.rs`, and the only in-tree shell callers are the two parity suites below. The
  four dead renderers lifecycle-kit/SPEC.md records as *removed in one motion by the cut that next
  takes this section's contract* go too.
- Delete `queue-kit/gate-tests/queue-lib-parity.test.sh` and
  `lifecycle-kit/gate-tests/stages-lib-parity.test.sh`, the `--queue-parity` and
  `--stages-lib-parity` arms in `native/src/main.rs`, and whatever `queue::parity_report` and the
  stages parity helper leave without a reader, found by the compiler rather than by reading. With
  one substrate there is nothing to compare, so criterion 6's machine-held disposition becomes its
  duplication-absent one (gate-sdk/SPEC.md §The port-candidate criteria).
- `scripts/queue-config.sh` and `scripts/lifecycle-config.sh` → `git mv` to `.knobs`, rewritten to
  the grammar, keeping each provenance comment as `#` lines. The local overlay entry in
  `.gitignore` becomes `scripts/queue-config.local.knobs`. Every fixture-tree config under both
  kits' `gate-tests/` takes the same move. Every `*.test.sh` and smoke writing a config file sets
  `<KIT>_KNOB_FILE` where it set `<KIT>_CONFIG_FILE`, and its array values become `NAME[] =` lines.
  The roster is `git grep -l -e QUEUE_KIT_CONFIG_FILE -e LIFECYCLE_KIT_CONFIG_FILE -e queue-config
  -e lifecycle-config`, run at build.
- `gate-sdk/gate-tests/run-arm-contract.test.sh` uses `QUEUE_KIT_REQUIRED_SECTIONS` as its example
  of a bridged array carrying a tab. It takes an array knob of a kit that is still bridged.
- Retarget the descriptors coupling a deleted file: `check-lifecycle-registration` and
  `check-merge-attrs` (`lifecycle-kit/lib/stages.sh`) and `check-scratch-citation`
  (`kit:lib/stages.sh`) couple `native/src/knobs/lifecycle_kit.rs` and
  `scripts/lifecycle-config.knobs`. `check-roadmap-fresh` couples `scripts/queue-config.knobs` for
  `scripts/queue-config.sh`. `check-graph` gains `scripts/*-config.knobs`, because a knob-file edit
  that changes a `knob:` expansion must re-trigger hook freshness, the way `scripts/*.sh` does for a
  shell config.
- The help lines naming a config file (`native/src/gates/queue_sections.rs`,
  `native/src/gates/stage_skill_coverage.rs`) name the `.knobs` file. This repo's
  `.claude/commands/scope.md` and `close.md` re-point likewise.
- `native/src/installer/recipe.rs`'s inline queue skeleton renders its headings from
  `knobs::queue_kit`'s `QUEUE_KIT_REQUIRED_SECTIONS` default rather than a second literal. The
  prefix-marked `Iteration:` renders as `## Iteration: —`, the skeleton's existing spelling. The
  default now lives in the crate, so the copy can be derived rather than kept.
- Regenerate the pre-commit hook, whose baked argv loses every `QUEUE_KIT_` and `LIFECYCLE_KIT_`
  assignment and gains delta 2's bridged inputs, and the kit SPEC mirrors.

### (9) The hermetic harness pins a static kit's knob file {design-bearing}

`lib/test-hermetic.sh` promises a bespoke suite runs *on kit defaults, never the invoker's cwd
config*, and it keeps that promise by pinning every `<KIT>_CONFIG_FILE` at one empty file. For a
static kit that pin only disarms the legacy refusal, and the reader still reads
`<gates-dir>/<stem>-config.knobs` under the suite's cwd. So a suite run from a tree carrying one
reads it. This cut moves many suites onto static kits, so the loop also pins every
`<KIT>_KNOB_FILE` at one empty file, which is valid in the grammar. A suite supplying its own
config sets `<KIT>_KNOB_FILE`, as it set `<KIT>_CONFIG_FILE` before. gate-sdk/SPEC.md
§lib/test-hermetic.sh states both pins. The local overlay is not pinned, and the section records
that as the remaining limit.

### (10) The kits' own SPEC sections {design-bearing}

- **queue-kit/SPEC.md and lifecycle-kit/SPEC.md §Layout and configuration.** Each states that the
  kit's knobs are static, as site-kit/SPEC.md §Layout and configuration does: the knob file,
  `<KIT>_KNOB_FILE`, the `.local.knobs` overlay and a pointer to gate-sdk/SPEC.md §The knob file for
  grammar, precedence and refusals. The *permanently shell* sentences and the `# no-port:` cause go.
  The rosters stay where they are, with each derived default restated in the roster's `${NAME}`
  spelling (delta 2). `QUEUE_KIT_REQUIRED_SECTIONS` names the reader-side composition (delta 3).
  The loader validation paragraph names the table validator (delta 6).
- **§lib/queue.sh becomes §The shared queue adapters, and §lib/stages.sh becomes §The stage-machine
  adapters.** Each describes its `native/src/*.rs` module as the sole holder. The *split is
  permanent* argument, the parity-harness paragraphs and the dead-renderer record are deleted,
  because what they argued about no longer exists. What survives is each adapter's contract: the
  section regexes and their empty-icebox guard, `QUEUE_TASK_SECTIONS`, the lead-line grammar,
  `roadmap_entries`, the cursor, the closing-stage predicate, the journal derivation and mark, the
  supersede and union sets, and the registration and merge-attribute renderers. The
  *bracketed-literal because awk `-v` eats a backslash* paragraph is deleted, since its reader was
  awk. The `// spec:` citations to both old headings (`native/src/queue.rs`, `native/src/stages.rs`,
  `native/src/main.rs`, `native/src/gates/`, `native/src/emit/file_gap.rs`) follow the rename, and
  so do the `.gitignore` comment and gate-sdk/SPEC.md's citations.
- **The local overlay** (queue-kit/SPEC.md §The lesson-sink arm and the adapter section) is
  `queue-config.local.knobs`, in the gates directory even when the tracked file is relocated. The
  shell loader put the overlay beside a relocated config, so this is a behavior change and delta 11
  declares it.
- **queue-kit/README.md and lifecycle-kit/README.md** re-point their config-copy steps.
- **delegation-kit/SPEC.md**, the statusline arm's paragraph on where the queue counter's four knobs
  resolve, and the queue-index arm's front-end paragraph it cites: both argue that a consumer
  override reaches the reader because the front end sources the same library. The four are now
  static, so the override reaches the reader through the crate's own resolution, and no bridge
  carries them. The malformed-config degradation holds through delta 6's validator.

### (11) gate-sdk's standing rules, and the release declarations {mechanical}

- gate-sdk/SPEC.md §The knob file: `<stem>` examples gain `queue` and `lifecycle`, the
  derived-default paragraph from delta 2 lands, and the *gate-sdk migrates last* bullet names
  `gate_static_knob` among what that cut retires (delta 5).
- gate-sdk/SPEC.md §The config-seam port disposition and §The kit-library port disposition: the two
  kits join the static set, which *has fired* covers. No new ruling.
- gate-sdk/SPEC.md §The non-gate arm: the `knob-values` arm (delta 4). The `--emit-knob-roster`
  paragraph's *A derived default renders with every sibling at its own default* gains the bridged
  `${NAME}` rendering.
- gate-sdk/SPEC.md §lib/gate.sh: the single-producer paragraph's `--knobs` sentence states the
  closure (delta 2), and the sentinel-expansion text states that static `knob:` members come from
  the arm (delta 4). drift-kit/SPEC.md §Layout and configuration: delta 5.
- `.workflow/release-declarations.md`. **Not yet applied:**
  - Renamed knobs: `QUEUE_KIT_CONFIG_FILE` → `QUEUE_KIT_KNOB_FILE`, `LIFECYCLE_KIT_CONFIG_FILE` →
    `LIFECYCLE_KIT_KNOB_FILE`.
  - Behavior changes: **`<gates-dir>/queue-config.sh`, `<gates-dir>/lifecycle-config.sh`**, with
    queue-kit's `.local.sh` overlay, are replaced by `.knobs` files in the knob-file line grammar,
    and both libraries are deleted, with defaults printed by `run-gates.sh --emit knob-roster`. A
    left-behind shell config is refused at exit 2, **including the copy an earlier `init` seeded
    that you never edited**: delete it, or rewrite what you set into the `.knobs` file `init` now
    seeds. For both kits an exported scalar knob now outranks the file. queue-kit's `.local`
    overlay now sits in the gates directory even when the tracked file is relocated. A `<gates-dir>/drift-config.sh` (or any
    bridged config) that sources `lifecycle-config.sh` to copy a value — the wiring drift-kit's
    SPEC documented for `DRIFT_KIT_STAGES` — breaks at upgrade: replace the `source` line and the
    copy with `gate_static_knob LIFECYCLE_KIT_STAGES DRIFT_KIT_STAGES`. `gate_static_knob` is
    transitional and is removed when the shell config bridge is.

## Producers and consumers

- **The two defaults tables and validators** (new state). *Producer:* `native/src/knobs/{queue,lifecycle}_kit.rs`.
  *Consumers:* `knobs::resolve` for every `QUEUE_KIT_*` and `LIFECYCLE_KIT_*` read,
  `--emit knob-roster`, the undeclared-name refusal, and the validator at first resolution.
- **`Row.inputs`** (new field). *Readers:* `knobs::bridged`'s closure at `--knobs` time, and the
  recording-resolver unit test. It is populated on every derived row and on no other row.
- **The `--knobs` closure** (changed interface). *Consumers:* `gate_command` (the argv now carries
  the bridged inputs), `gen-pre-commit`'s baked argv, `runner.rs`'s `child_knobs`, and the
  `knob-values` arm's own dispatch. Enabling config: the default layout itself, since no config is
  needed for the derived rows to fire.
- **`--emit knob-values`** (new interface). *Producer:* the crate, dispatched through `gate_knob_env`
  like any bridged arm. *Consumers:* `gate_expand_couples_var` at hook generation and
  `run-gates --for` (delta 4), and `gate_static_knob` in this repo's `scripts/drift-config.sh`
  (delta 5). Column one is read by both to key the value, column two only by `gate_static_knob` to
  refuse a non-indexed read into an array, and column three by both.
- **`gate_static_knob`** (new helper). *Producer:* `lib/gate.sh`. *Consumer:* `scripts/drift-config.sh`,
  the one live reference. This repo sets the enabling config.
- **`queue::required_sections()`** (new adapter). *Consumer:* `check-queue-sections`.
- **The `.knobs` templates** (new state). *Producer:* the kits. *Consumers:* `config_seam_plan` at
  `init`, and `check-template-copy-parity`'s suffix exclusion.
- **Red conditions (point 5).** Deltas 8 and 3 narrow corpora, and delta 2 widens `--knobs`.
  - *`check-docs-cmd` assertion B's known-knob set* reds on a documented name absent from the
    set. The libraries leave kit-root source, which can **add** reds, so it is not monotone. The
    roster union covers declared rows. The env-only `LIFECYCLE_KIT_SESSION_*` names are known
    through their kit-root mentions in `lifecycle-kit/smoke/install.sh`. Build runs the gate, and
    a red there means a mention outlived its only source.
  - *`check-knob-default-coupling`* reds on disagreement and on a SPEC stating no default. Both
    are monotone. The migrated scalars move to the roster idiom, and a `${NAME}` render classifies
    as computed and is skipped-and-counted exactly as the shell deferral was. The skipped count
    moves, and nothing asserts it.
  - *§port-blockers' `--tree` no-port count and every measured claim over it* read exact counts.
    Two libraries and four shell configs (two consumer, two templates) leave the scanned set, and
    `check-measured-claim` re-runs its oracles. Build runs the battery rather than inspecting.
  - *`check-graph` assertion D* compares the regenerated hook byte for byte and reds on staleness.
    Every member reading a derived knob gains argv elements, so delta 8 regenerates.
    `check-reads-couples` and `check-gate-substrate-parity` read `--knobs` against `couples=` and
    against the registry's declared `KNOBS`. A widened answer is a superset, so an assertion
    requiring `knob:` tokens to be declared stays green. An assertion equating `--knobs` with the
    registry constant would red; build runs both gates and states which held.
  - *`check-gate-fixture-coverage`* derives its set from arms and descriptors. Deleting
    `--queue-parity` and `--stages-lib-parity` removes members, not fixtures.
  - *`check-template-copy-parity`* pairs templates with copies. The `.sh` exclusion drops two
    members and the `.knobs` exclusion adds two, so the net finding set is unchanged by inspection.
    Build runs it.

## Existing sections updated

- `gate-sdk/SPEC.md` — §The knob file (deltas 2, 5 and 11); §The non-gate arm (deltas 4 and 11);
  §lib/gate.sh (deltas 2, 4 and 11); §The config-seam port disposition and §The kit-library port
  disposition (delta 11); §lib/test-hermetic.sh (delta 9); §check-template-copy-parity (delta 7).
- `queue-kit/SPEC.md` — §Layout and configuration, §lib/queue.sh renamed, §The lesson-sink arm,
  §The queue-index arm, §check-queue-sections (deltas 3, 6 and 10).
- `lifecycle-kit/SPEC.md` — §Layout and configuration, §lib/stages.sh renamed (deltas 6 and 10).
- `drift-kit/SPEC.md` — §Layout and configuration, `DRIFT_KIT_STAGES` (delta 5).
- `delegation-kit/SPEC.md` — the statusline arm's paragraph on where the queue counter's knobs
  resolve (delta 10).
- `installer/README.md` — §What init seeds (delta 7).
- `queue-kit/README.md` — config step (delta 10).
- `lifecycle-kit/README.md` — config step (delta 10).
- `native/src/knobs/mod.rs` — `inputs`, closure, validator hook, `knob-values` (deltas 2, 4 and 6).
- `native/src/knobs/queue_kit.rs` — new table and validator (deltas 1 and 6).
- `native/src/knobs/lifecycle_kit.rs` — new table and validator (deltas 1 and 6).
- `native/src/queue.rs` — `required_sections()` (delta 3); parity report removed and citations
  renamed (deltas 8 and 10).
- `native/src/stages.rs` — parity helper removed and citations renamed (deltas 8 and 10).
- `native/src/main.rs` — parity arms removed (delta 8).
- `native/src/gates/queue_sections.rs` — composition (delta 3); help line (delta 8).
- `native/src/gates/mod.rs` — declared knob (delta 3); citation renamed (delta 10).
- `native/src/gates/stage_skill_coverage.rs` — help line (delta 8).
- `native/src/gates/stage_evidence.rs` — citation renamed (delta 10).
- `native/src/emit/file_gap.rs` — citation renamed (delta 10).
- `native/src/emit/session_id.rs` — re-grounded comment (delta 1).
- `native/src/emit/mod.rs` — re-grounded comment (delta 1); `knob-values` row (delta 4).
- `native/src/installer/recipe.rs` — seeding suffixes (delta 7); skeleton derivation (delta 8).
- `native/src/gates/template_copy_parity.rs` — suffix exclusion (delta 7).
- `gate-sdk/lib/gate.sh` — static `knob:` expansion and `gate_static_knob` (deltas 4 and 5).
- `gate-sdk/lib/test-hermetic.sh` — the knob-file pin (delta 9).
- `queue-kit/lib/queue.sh` — deleted (delta 8).
- `lifecycle-kit/lib/stages.sh` — deleted (delta 8).
- `queue-kit/gate-tests/queue-lib-parity.test.sh` — deleted (delta 8).
- `lifecycle-kit/gate-tests/stages-lib-parity.test.sh` — deleted (delta 8).
- `queue-kit/templates/queue-config.sh` — replaced by the `.knobs` template (delta 7).
- `lifecycle-kit/templates/lifecycle-config.sh` — replaced by the `.knobs` template (delta 7).
- `scripts/queue-config.sh` — moved to `.knobs` (delta 8).
- `scripts/lifecycle-config.sh` — moved to `.knobs` (delta 8).
- `scripts/drift-config.sh` — reads the roster through the helper (delta 5).
- `queue-kit/gate-tests/` — config writers (delta 8).
- `lifecycle-kit/gate-tests/` — config writers (delta 8).
- `lifecycle-kit/smoke/install.sh` — config writers and the lock-pattern case (deltas 6 and 8).
- `gate-sdk/gate-tests/run-arm-contract.test.sh` — the bridged tab example (delta 8).
- `installer/consumer-smoke/run-smoke.sh` — the seam arm's edited path (delta 7).
- `lifecycle-kit/checks/check-lifecycle-registration.gate` — couples (delta 8).
- `lifecycle-kit/checks/check-merge-attrs.gate` — couples (delta 8).
- `lifecycle-kit/checks/check-scratch-citation.gate` — couples (delta 8).
- `queue-kit/checks/check-roadmap-fresh.gate` — couples (delta 8).
- `gate-sdk/checks/check-graph.gate` — couples (delta 8).
- `.gitignore` — the overlay entry and its comment (deltas 8 and 10).
- `.claude/commands/scope.md` — config path (delta 8).
- `.claude/commands/close.md` — config paths (delta 8).
- `scripts/git-hooks/pre-commit` — regenerated (delta 8).
- `.workflow/release-declarations.md` — Renamed knobs and Behavior changes (delta 11).
- `docs/check-graph.html` — generated artifact, regenerated (delta 8).
- `docs/gate-sdk/SPEC.md` — generated mirror, regenerated (all deltas).
- `docs/queue-kit/SPEC.md` — generated mirror, regenerated (all deltas).
- `docs/lifecycle-kit/SPEC.md` — generated mirror, regenerated (all deltas).
- `docs/drift-kit/SPEC.md` — generated mirror, regenerated (delta 5).
- `docs/delegation-kit/SPEC.md` — generated mirror, regenerated (delta 10).
- `docs/queue-kit/README.md` — generated mirror, regenerated (delta 10).
- `docs/lifecycle-kit/README.md` — generated mirror, regenerated (delta 10).
<!-- update-target-exempt: a dated release post is immutable history, and its mention of the library it shipped stays -->
- `docs/posts/2026-08-21-checkwright-v0-24-0.md` — unchanged.

## Retired spellings

- `queue-kit/lib/queue.sh` — deleted (delta 8).
- `lifecycle-kit/lib/stages.sh` — deleted (delta 8).
- `queue-lib-parity.test.sh` — deleted (delta 8).
- `stages-lib-parity.test.sh` — deleted (delta 8).
- `--queue-parity` — arm deleted (delta 8).
- `--stages-lib-parity` — arm deleted (delta 8).
- `queue-config.sh` — replaced by `queue-config.knobs` (deltas 7 and 8).
- `lifecycle-config.sh` — replaced by `lifecycle-config.knobs` (deltas 7 and 8).
- `queue-config.local.sh` — replaced by `queue-config.local.knobs` (delta 10).
- `QUEUE_KIT_CONFIG_FILE` — renamed `QUEUE_KIT_KNOB_FILE` (delta 11).
- `LIFECYCLE_KIT_CONFIG_FILE` — renamed `LIFECYCLE_KIT_KNOB_FILE` (delta 11).
- `§lib/queue.sh` — section renamed §The shared queue adapters (delta 10).
- `§lib/stages.sh` — section renamed §The stage-machine adapters (delta 10).

## Definition of Done

- [ ] **Causal completeness** — every new state and interface above has a reachable producer and a
      named consumer, and `Row.inputs` is held by its unit test.
- [ ] **Instruction surfaces: instruction only** — the refusal messages and the release declaration
      carry the remedy, not the grounds.
- [ ] **Merged with no information lost** — each surviving adapter contract lands in its renamed
      section. Deleted argument prose (the permanent split, the parity harnesses, the awk escaping)
      is gone because its subject is, and the refused alternatives stay in this file's history.
- [ ] **Amendment deleted**, and the entry is **demoted, not moved to Done** (corpus increment),
      restoring its position and board tags from this promotion's diff.
- [ ] **Removals propagated** — `check-amendment-retired-spelling` green.
- [ ] **Gaps filed** — any reader delta 8's grep roster missed, and whichever assertion the
      `--knobs` widening reddened, if it lands as a gap.
