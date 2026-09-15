# SPEC amendment: git-hooks-port

Queue entries: `gen-pre-commit-port`, which leads unit set `owed-port-tail` (operator direction,
2026-09-15, lead-relayed), and the rider `gate-command-status-conflation-third-caller`, one of whose two
surviving call sites this port deletes. The rider's other site is SPEC-consumer-smoke-port.md's, and the
entry closes when both have landed.

**What moves.** `gate-sdk/bin/gen-pre-commit.sh` emits the two generated git hooks from the per-gate
`# graph:` manifests. It becomes a binary arm with the same output, `check-graph` assertion D reads that
emission in process instead of spawning `bash`, every caller moves to the arm, and the script is deleted.
The hooks stay bash files that git runs. Only their generator changes substrate, and §gen-pre-commit's
refusal of a two-line exec shim stands untouched.

**What the port leaves alone.** The front-end stub `bin/run-gates.sh` still needs bash, and every
caller that is itself shell reaches the arm through it. That floor belongs to `native-windows-bash-floor`,
and this port does not move it.

## The seam

- **Kit mechanism:** the hook emitter module and its `--emit git-hooks` arm, `check-graph`'s in-process
  assertion D, the reduced `lib/gate.sh`, the caller moves across the crate and the kit `smoke/` recipes,
  and the tree test's `gen=manual` cases.
- **Consumer config:** unchanged. `GATE_SDK_HOOKS_DIR`, `GATE_SDK_NATIVE_BIN` and `GATE_SDK_KIT_DIRS`
  keep their meaning and defaults. A consumer's action is the new regeneration command (delta 7).
- **This repo's own:** `scripts/gate-sdk-config.knobs`' `GATE_SDK_PORTABILITY_PATHS` corpus, the
  committed hooks, and one permission allowlist entry (delta 3).
- **Private rule content:** none in reach.

## What changes

### (1) The hook emitter moves into the crate as `--emit git-hooks` {design-bearing}

A new module, `native/src/emit/git_hooks.rs`, owns the emission. It exposes two functions, both
parameterized by the repository root and the resolved gates dir:

- `pre_commit(root, gates_dir) -> Result<String, String>`: the pre-commit hook text.
- `commit_msg(root, gates_dir) -> Result<Option<String>, String>`: the commit-msg hook text, or `None`
  when no registered member is `tier=commit-msg`. That `None` is the one statement of the conditional
  both `--write` and assertion D read (delta 2), where today the script and the gate each compute it.

The arm is an `Arm::Emit` row, `--emit-git-hooks`, taking exactly one operand:

- `pre-commit` prints the pre-commit hook.
- `commit-msg` prints the commit-msg hook, and refuses with exit 2 when `commit_msg` returns `None`,
  naming the absent tier.
- `--write` writes `<hooks-dir>/pre-commit` always and `<hooks-dir>/commit-msg` only when `commit_msg`
  returns `Some`. It creates the hooks dir, sets each written file's executable bit where the host has
  one, and prints one `git-hooks: wrote <path>` line per file.
- No operand, or any other operand, prints `usage: --emit git-hooks pre-commit|commit-msg|--write` and
  exits 2.

The family and the operand shape are forced rather than chosen. The generator declares no exit 1, so
the {0, 2} collapse an emitting arm makes discards nothing. §The non-gate arm rules that a member's own
subcommand word is an operand, never composed into the flag. And `--emit docs-mirror --write` is the
existing precedent for a document arm that writes. The name is `git-hooks`, the hooks dir's own default
basename, because a bare `hooks` would read as the harness-integration `--hook` family.

The arm derives the root with `git rev-parse --show-toplevel` and refuses with exit 2 outside a
repository, as the script did. Its knob roster is `GATE_SDK_HOOKS_DIR`, `GATE_SDK_NATIVE_BIN`,
`GATE_SDK_KIT_DIRS` and the `registry::EVERY_COUPLES_KNOB` sentinel. The crate's arm-knob test holds
that roster to what the module reads, and the build takes that test's verdict over this list.

**The emission is the script's, byte for byte, and each rule has a crate source already in the tree:**

- **Members:** `registry::members` over `<gates-dir>/gates.list`, deduplicated in first-seen order, then
  filtered by `tier=`.
- **Check dirs:** the gates dir first, then each `walk::kit_roots_rel()` root's `checks/`, spelled
  relative to the repository root.
- **Manifest fields:** `registry::manifest_line` and `registry::manifest_fields` over the member's
  resolved declaration. A member that resolves nowhere reads every field empty.
- **Trigger:** `trigger=`, else `couples=`, expanded by `registry::expand_couples`. Unless the trigger is
  `*`, the member's derived knob files from `registry::knob_files` over the same check dirs are appended.
- **Invocation:** `registry::resolve` over the check dirs. A `.gate` declaration emits
  `<GATE_SDK_NATIVE_BIN> <name>` with the knob's value as resolved. A `.sh` declaration emits its
  resolved path. A member resolving nowhere emits `<gates-dir>/<name>.sh`, the script's fallback, kept so
  a registry naming an absent member still yields a hook whose `run_gate` line fails loudly at commit.
- **Quoting:** each argv element is emitted verbatim when it is non-empty and made only of
  `[A-Za-z0-9_./:=+,@%-]`. Anything else becomes bash ANSI-C `$'…'`, with backslash, single quote, tab
  and newline escaped as the script's `quote_elem` does.
- **Blocks:** the unconditional `run_gate` line for a `*` trigger, the `mapfile … git diff --cached`
  staged block for `mode=staged`, and the `staged_matches` block otherwise, each character for character.
- **`gen=manual`:** the current hook's `# >>> manual: <name>` / `# <<< manual: <name>` regions are read
  from `<hooks-dir>/pre-commit` before emission. A member with a stored region re-emits it verbatim, and
  one without re-emits the TODO placeholder line.
- **Matcher body:** the lines strictly between `gate_staged_matches() {` and the next line that is
  exactly `}` in `<gate-sdk-root>/lib/gate.sh`, read as text. An absent function is exit 2 naming it. No
  `bash` is spawned to read it.
- **Header and tail:** the two hooks' fixed text, carried as literals in the module. The one change is
  the regeneration command both headers name, which becomes
  `bash gate-sdk/bin/run-gates.sh --emit git-hooks --write` (delta 7 regenerates the committed hooks).

**The rider's shape does not survive the move, and here is why.** The script's `command_rel` read
`gate_command` through a process substitution. An absent binary (`gate_command`'s exit 2) therefore
arrived as an empty argv, the dead `|| return 1` guard never fired, and the caller fell back to emitting
`<gates-dir>/<name>.sh` for a `.gate` member. The emitter is now the binary itself, and a `.gate`
declaration always emits the binary's argv. The one remaining failure is resolution, which returns found
or not found, so no harness-error status exists to be conflated.

### (2) `check-graph` assertion D reads the emission in process {design-bearing}

`native/src/gates/graph.rs` drops `generator_emit`, the generator-path resolution and its
`gen-pre-commit.sh not found` error. Assertion D calls `git_hooks::pre_commit` and `git_hooks::commit_msg`
over the root it runs at, and it compares each committed hook against the returned text exactly as it
compares today. `has_msg_gate` is replaced by the `Option` delta 1 returns, so the two readers of that
conditional become one. An emission `Err` is the existing check-could-not-run branch, carrying the
emitter's message where it carried the child's account. Every remedy line names
`bash gate-sdk/bin/run-gates.sh --emit git-hooks --write`.

**`check-graph`'s declared requirement is re-measured, not assumed.** The `("bash", "")` element and its
comment in `native/src/gates/mod.rs` leave with the spawn. The replacement set is what the in-process
path spawns, taken by the spawn census §gen-pre-commit already names as this member's method: `git` if
the emitter core or the gate derives the root through `git rev-parse`, and nothing new otherwise. The
build takes the census, not this sentence. Unit test A's observation stays vacuous for this member for
the reason the section gives, and `check-graph-tree.test.sh` stays the behavioural oracle.

### (3) Every caller moves to the arm {mechanical}

- `native/src/installer/init.rs` — the vendored hook step runs
  `gate-sdk/bin/run-gates.sh --emit git-hooks --write` through `run_vendored`, beside the graph step
  already spelled that way.
- `native/src/emit/agents_md_smoke.rs` — its `--write` step, the same command.
- `native/src/emit/upgrade_smoke.rs` — phase A's regeneration, the same command under the exported
  `GATE_SDK_ROOT`, and its failure message becomes `phase A hook regeneration failed at TO`.
- `native/src/emit/install_hooks.rs` — the no-hooks-dir refusal names the new command.
- Every kit's `smoke/install.sh` (gate-sdk, canon-kit, context-kit, delegation-kit, doctrine-kit,
  evidence-kit, lifecycle-kit, queue-kit, site-kit) — `bash "$SDK/bin/run-gates.sh" --emit git-hooks
  --write`. **The recipes keep a shell spelling, and it is the front-end's.** A `smoke/` recipe is bash by
  §Consumer smoke's port disposition, and the front-end is its one door to the binary. A retained
  `gen-pre-commit.sh` that execs the arm is refused, because it would be a second spelling of the arm
  roster, the thing §The non-gate arm refuses a front-end case for.
- `gate-sdk/gate-tests/check-graph-tree.test.sh` — `regen()` calls the arm through the front-end under
  the sandbox's hermetic knobs.
- `scripts/gate-sdk-config.knobs` — `GATE_SDK_PORTABILITY_PATHS` drops `gate-sdk/bin/gen-pre-commit.sh`,
  and its comment drops the clause naming it. `init` now leaves behind no shell the install path runs
  except `lib/gate.sh`.
- `.claude/settings.json` — the `Bash(bash gate-sdk/bin/gen-pre-commit.sh --write)` allow entry is
  stale. A permission-settings edit is applied on the operator's behalf and never by hand (CLAUDE.md
  §Housekeeping), so the build prepares the one-line removal as a diff for the lead to relay. Only
  prepared, never applied, by the build.

### (4) The script, `--knob-files` and three callerless helpers retire {design-bearing}

- **`gate-sdk/bin/gen-pre-commit.sh` is deleted.**
- **`lib/gate.sh` loses `gate_manifest_field`, `gate_expand_couples_var` and `gate_expand_couples`.**
  The script was the only caller of the first two, and the third already had none. That census was taken
  with a tracked-tree grep, and delta 1 of SPEC-roster-rederivation.md has the build re-take it. With the
  shell expander gone, §The `# graph:` manifest's roster of independent `kit:`/`knob:` prefix readers
  drops from four to three, and hook emission reads `registry::expand_couples`, the reader
  `check-graph` and `--for` already share.
- **`gate_staged_matches` stays.** It has two live readers: the emitter splices its body into every
  hook (delta 1), and `runner.rs`'s standing cross-substrate comparison feeds it the canned glob corpus
  (§The port-candidate criteria, criterion 6's second worked instance). Its header comment names the
  emitter where it names `gen-pre-commit`.
- **The `--knob-files` top-level flag retires.** The script was its only caller, and §The non-gate arm
  requires a named caller or the arm is dead weight. It leaves `TOP_LEVEL_FLAGS`, its `knob_files`
  function and its dispatch in `native/src/main.rs`. The derivation behind it, `registry::knob_files`,
  stays with two callers: the emitter and `check-graph`'s manifest loop. Every comment and SPEC sentence
  that uses the flag's name for the derivation ("a hardcoded flag would hide its reads from
  `--knob-files`") is reworded to name the derivation, which is what it always meant. Those sites are
  about fifteen crate comments and the prose in gate-sdk, context-kit, delegation-kit, drift-kit,
  guard-kit and queue-kit SPECs, by `git grep -c -- --knob-files`.

### (5) The `gen=manual` round-trip gets its first executed oracle {design-bearing}

No registered member uses `gen=manual`, and no test reaches the round-trip: the one `gen=manual` in a
fixture is an amendment-body example that never reaches hook emission. The port is where the round-trip
could break silently, so `check-graph-tree.test.sh` gains two cases over a sandbox member declaring
`gen=manual`. **Placeholder:** with no region in the current hook, `--write` emits the sentinels around
the TODO line and `check-graph` is clean. **Round-trip:** a hand-filled region survives a second
`--write` byte for byte, and `check-graph` is clean. A third, **stale**, case edits the committed region
without regenerating and asserts `check-graph` stays clean. The manual region is the consumer's text,
and assertion D compares against an emission that carries it back.

**The byte-parity proof is taken once, before the deletion.** On this tree and on the tree test's
sandbox, the script's `--emit` and `--emit-commit-msg` output is diffed against the arm's `pre-commit`
and `commit-msg` output. The only permitted difference is the header's regeneration-command line, and
the build's commit message records the diff. After the deletion the standing oracles are assertion D's
freshness comparison and the tree test.

### (6) The SPEC sections that described the shell generator {design-bearing}

gate-sdk/SPEC.md §gen-pre-commit keeps its heading. §bin/enter-stage.sh and §run-gate-tests are the
precedent for a section outliving its script under its old name, and every `§gen-pre-commit` citation
in the tree stays live. Its body changes as follows. **Not yet applied:**

- The opening paragraph names the arm and its three operands where it names `--emit`,
  `--emit-commit-msg` and `--write`. The sentence on a `tier=commit-msg` member porting "with no new
  emitter arm" becomes: both hooks resolve every member's invocation through one registry resolution
  in the emitter.
- **Deleted:** the paragraphs that exist only for the spawn residue: **This generator is owed**,
  **The residue's disposition**, **What the declaration does not cover**, **And the measurement is what
  the declaration rests on** and **The absent-`bash` branch was run rather than assumed**. The residue
  they dispose of is gone. Their one surviving rule, that a member's declared requirements are taken by
  spawn census and never read off a green test, is §The `# graph:` manifest's already, and delta 2
  cites it there.
- **Assertion E is not one of the two arms** becomes: assertions D and E both compute their emissions
  in process.
- **The hook carries no knob environment** replaces "resolves argv through `gate_command` at generation
  time" with "resolves each member's invocation through the registry at generation time".
- The quoting paragraph keeps its rule and replaces its ground. The hook is bash, so an element that is
  not shell-inert takes bash's ANSI-C form, and the emitter renders it with a fixed escape set rather
  than any shell's own quoting, which is what keeps committed hooks byte-identical across clones.
- The closing sentence on derived knob files reads them in process through `registry::knob_files` over
  the emitter's check dirs, and no longer through `--knob-files`.
- A new paragraph states delta 1's matcher-body read and the `commit_msg` `Option` as the one statement
  of the commit-msg conditional.

gate-sdk/SPEC.md §check-graph, **The interpreter assertion D spawns is resolved, not named**, and its
continuation through **A generator that could not run reports why it could not run**: assertion D
spawns no interpreter after delta 2, so these paragraphs lose their attested instance, while the rule
they carry (the homonym roster resolved by the owner inside `proc::run*`) stays live for every other
bare-name `bash` spawn. **Not yet applied:** the paragraphs are re-grounded on a surviving spawn. The
installer's `run_vendored`, which spawns `bash` for the front-end on every `init`, is the nearest one,
and the build names whichever instance its census confirms. The Windows record (five rounds on the WSL
launcher, the empty-cause widening) stays as history of the rule, stated without assertion D as its
present subject.

gate-sdk/SPEC.md, the other sections naming the generator, each rewritten to the arm. **Not yet
applied:** §The `# graph:` manifest (the four-reader roster, per delta 4), §Reading a `couples=` field's
reach, §The port-candidate criteria (criterion 7's attested instance at assertion D, and the owed-file
narrative), §lib/gate.sh (the three retired helpers, the `--knob-files` sentences, and `gate_command`'s
caller list), §The non-gate arm (the roster gains `--emit-git-hooks` and loses `--knob-files`),
§run-gates (the matcher paragraph's "`gen-pre-commit` emits verbatim", and "draw the `# graph:` fields
through `gate_manifest_field` + `gate_expand_couples_var`", which becomes the registry readers),
§upgrade-smoke ("since `gen-pre-commit` expands every `couples=knob:` token"), §port-blockers (the
`--needs`/spawn-census example), §install-hooks and §Consumer payload wherever they name the script.

installer/README.md, **A `behind-invoke` step may spawn `bash`, and one does**. **Not yet applied:** the
step still spawns `bash`, now for the front-end stub rather than a shell generator. The paragraph says
so, drops "not compiled until that port lands", and keeps its point that the bootstrap spawns nothing.

### (7) Regeneration, prose, projections and release declarations {mechanical}

- Regenerate both committed hooks with the new command. Their headers change, and the hooks also bake
  `check-measured-claim`'s resolved values, whose `tree-shell-owed` key moves when an owed file leaves
  (docs/site-architecture.md §Generated projections and their freshness gates). Then regenerate the
  graph artifact, the SPEC mirror, and every projection whose freshness gate reds.
- Prose naming the script, rewritten to the arm: `gate-sdk/README.md` (tool roster and regeneration
  line), each kit README's install-block regeneration bullet (canon-kit, context-kit, delegation-kit,
  doctrine-kit, evidence-kit, lifecycle-kit, queue-kit, site-kit), `docs/install.md` §Reviewing the
  pre-commit hook before you install it, `docs/site-architecture.md` (the graph-artifact row and every
  other row naming it), `lifecycle-kit/SPEC.md`, `canon-kit/SPEC.md` and `drift-kit/SPEC.md`.
- `.workflow/release-declarations.md` bullets. **gate-sdk:** `bin/gen-pre-commit.sh` is deleted and the
  hooks regenerate with `bash gate-sdk/bin/run-gates.sh --emit git-hooks --write`, so a consumer's own
  scripts or CI calling the old path must switch. `lib/gate.sh` no longer defines `gate_manifest_field`,
  `gate_expand_couples_var` or `gate_expand_couples`, so a consumer shell gate calling one must stop.
  The binary's `--knob-files` flag is gone. **Every kit:** the README install block's regeneration
  line changed. **installer:** `init` generates the hooks through the arm, and nothing changes for an
  adopter.

## Producers and consumers

- **`--emit-git-hooks` (new arm)** — producer: the binary, reached as `run-gates.sh --emit git-hooks`.
  Consumers, each by spawn of the front-end or by the binary's own dispatch: `init` at install, the nine
  kit `smoke/install.sh` recipes under `--run-consumer-smoke`, `--upgrade-smoke` and
  `--agents-md-smoke`, the upgrade and agents-md arms' own regen steps, the tree test, and a session
  regenerating after a manifest edit on `check-graph`'s remedy line. Enabling config: none. Every
  deployed consumer has `GATE_SDK_HOOKS_DIR`'s default.
- **`git_hooks::pre_commit` / `commit_msg` (new in-crate interface)** — producer: the module. Consumers:
  the arm, and `check-graph` assertion D on every battery, hook and CI run. The `Option` field's
  readers are `--write` (whether to write `commit-msg`), the `commit-msg` operand (whether to refuse),
  and assertion D (whether a committed `commit-msg` is required). All three read it at the same
  transition, one emission.
- **The `git-hooks: wrote <path>` lines** — reader: a human at the terminal. `init` captures merged output
  only to print on failure, `--upgrade-smoke` discards it, and the kit recipes let it through. No program
  parses it, so the prefix change is safe by inspection. The build confirms this with its re-derivation.
- **Retired producers.** The script's stdout, read by assertion D and by nothing else once the callers
  in delta 3 move. `--knob-files`' stdout, read by the script alone. The three `lib/gate.sh` helpers,
  called by the script alone.
- **Narrowing, point 5.** Deleting the script narrows three corpora. For each, the red condition:
  - **`GATE_SDK_PORTABILITY_PATHS`** (`check-portability-floor`) reds (exit 1) on an undeclared
    banned construct in a scanned file, and exits 2 only on an unreadable member or pattern file. An
    empty corpus disables the gate cleanly and names the absence. The corpus keeps `installer/bin` and
    `lib/gate.sh`, so it neither empties nor gains a violation. The deleted file's one
    `# portability-declared:` site (its `realpath --relative-to`) leaves with it, and the emitter
    computes the relative spelling in Rust.

How the rosters here and below were derived, per SPEC-roster-rederivation.md's delta 2: `git grep -c
"gen-pre-commit"` and `git grep -c -- "--knob-files"` over the tracked tree without stderr suppression,
a delegated read-only survey of the crate's arm table, check-graph's spawn and `lib/gate.sh`'s caller
sets, and `check-amendment-retired-spelling`'s survivor report, which named the comment sites in
`registry.rs` and `core_files.rs` that the grep for the script's name could not reach. This is a floor,
and the build re-derives it.
  - **`port-blockers --tree`'s owed count** has no red threshold of its own. Its reader
    `check-measured-claim` reds on a `measured:` literal disagreeing with the re-run key, so any prose
    literal quoting the owed count reds and is re-stamped. That is a caught change, not a silent one.
  - **The hook's `run_gate` roster** is unchanged, since the member set is the registry's.

## Existing sections updated

- `native/src/emit/git_hooks.rs` — the new emitter module (delta 1).
- `native/src/emit/mod.rs` — the arm-table row, and the comment naming `--knob-files` for the
  derivation (deltas 1 and 4).
- `native/src/gates/graph.rs` — assertion D in process and its remedy lines (delta 2).
- `native/src/gates/mod.rs` — `check-graph`'s requirement element (delta 2).
- `native/src/main.rs` — `--knob-files` retires (delta 4).
- `native/src/installer/init.rs` — the vendored hook step (delta 3).
- `native/src/emit/agents_md_smoke.rs` — its regeneration step (delta 3).
- `native/src/emit/upgrade_smoke.rs` — phase A's regeneration and its message (delta 3).
- `native/src/emit/install_hooks.rs` — the no-hooks-dir refusal (delta 3).
- `native/src/registry.rs` — the comment naming `gate_manifest_field` (delta 4).
- `native/src/gates/core_files.rs` — the comment naming `gate_expand_couples` (delta 4).
- `native/src/doctrine.rs` — the `--knob-files` comment (delta 4).
- `native/src/emit/cite_survey.rs` — the `--knob-files` comment (delta 4).
- `native/src/emit/diff_baseline.rs` — the `--knob-files` comment (delta 4).
- `native/src/emit/file_gap.rs` — the `--knob-files` comment (delta 4).
- `native/src/emit/file_survey.rs` — the `--knob-files` comment (delta 4).
- `native/src/emit/install_lifecycle.rs` — the `--knob-files` comment (delta 4).
- `native/src/emit/kfric.rs` — the `--knob-files` comment (delta 4).
- `native/src/emit/overhead_meter.rs` — the `--knob-files` comment (delta 4).
- `native/src/emit/port_blockers.rs` — the `--knob-files` comment (delta 4).
- `native/src/emit/run_gate_tests.rs` — the `--knob-files` comment (delta 4).
- `native/src/emit/run_validate.rs` — the `--knob-files` comment (delta 4).
- `native/src/emit/stage_rules.rs` — the `--knob-files` comment (delta 4).
- `native/src/hook/statusline.rs` — the `--knob-files` comment (delta 4).
- `gate-sdk/bin/gen-pre-commit.sh` — deleted (delta 4).
- `gate-sdk/lib/gate.sh` — three helpers deleted, and `gate_staged_matches`' comment (delta 4).
- `gate-sdk/smoke/install.sh` — the regeneration call (delta 3).
- `canon-kit/smoke/install.sh` — the regeneration call (delta 3).
- `context-kit/smoke/install.sh` — the regeneration call (delta 3).
- `delegation-kit/smoke/install.sh` — the regeneration call (delta 3).
- `doctrine-kit/smoke/install.sh` — the regeneration call (delta 3).
- `evidence-kit/smoke/install.sh` — the regeneration call (delta 3).
- `lifecycle-kit/smoke/install.sh` — the regeneration call (delta 3).
- `queue-kit/smoke/install.sh` — the regeneration call (delta 3).
- `site-kit/smoke/install.sh` — the regeneration call (delta 3).
- `gate-sdk/gate-tests/check-graph-tree.test.sh` — the arm call and the `gen=manual` cases (deltas 3
  and 5).
- `scripts/gate-sdk-config.knobs` — the portability corpus (delta 3).
- `.claude/settings.json` — the stale allow entry, as a prepared diff only (delta 3).
- `gate-sdk/SPEC.md` — §gen-pre-commit, §check-graph and the sections delta 6 names (deltas 2, 4 and 6).
- `installer/README.md` — the behind-invoke paragraph (delta 6).
- `context-kit/SPEC.md` — the `--knob-files` sentence (delta 4).
- `delegation-kit/SPEC.md` — the `--knob-files` sentence (delta 4).
- `drift-kit/SPEC.md` — the `--knob-files` sentence and the generator mention (deltas 4 and 7).
- `guard-kit/SPEC.md` — the `--knob-files` sentences (delta 4).
- `queue-kit/SPEC.md` — the `--knob-files` sentences (delta 4).
- `lifecycle-kit/SPEC.md` — the generator mentions (delta 7).
- `canon-kit/SPEC.md` — the generator mention (delta 7).
- `gate-sdk/README.md` — the tool roster and the regeneration line (delta 7).
- `canon-kit/README.md` — the regeneration line (delta 7).
- `context-kit/README.md` — the regeneration line (delta 7).
- `delegation-kit/README.md` — the regeneration line (delta 7).
- `doctrine-kit/README.md` — the regeneration line (delta 7).
- `evidence-kit/README.md` — the regeneration line (delta 7).
- `lifecycle-kit/README.md` — the regeneration line (delta 7).
- `queue-kit/README.md` — the regeneration line (delta 7).
- `site-kit/README.md` — the regeneration line (delta 7).
- `docs/install.md` — §Reviewing the pre-commit hook before you install it (delta 7).
- `docs/site-architecture.md` — the graph-artifact row and every row naming the generator (delta 7).
- `scripts/git-hooks/pre-commit` — regenerated (delta 7).
- `scripts/git-hooks/commit-msg` — regenerated (delta 7).
- `docs/check-graph.html` — regenerated (delta 7).
- `docs/gate-sdk/SPEC.md` — the SPEC mirror, regenerated (all deltas).
- `docs/gate-sdk/README.md` — the README mirror, regenerated (delta 7).
- `docs/canon-kit/README.md` — the README mirror, regenerated (delta 7).
- `docs/context-kit/README.md` — the README mirror, regenerated (delta 7).
- `docs/context-kit/SPEC.md` — the SPEC mirror, regenerated (delta 4).
- `docs/delegation-kit/README.md` — the README mirror, regenerated (delta 7).
- `docs/delegation-kit/SPEC.md` — the SPEC mirror, regenerated (delta 4).
- `docs/doctrine-kit/README.md` — the README mirror, regenerated (delta 7).
- `docs/drift-kit/SPEC.md` — the SPEC mirror, regenerated (deltas 4 and 7).
- `docs/evidence-kit/README.md` — the README mirror, regenerated (delta 7).
- `docs/guard-kit/SPEC.md` — the SPEC mirror, regenerated (delta 4).
- `docs/lifecycle-kit/README.md` — the README mirror, regenerated (delta 7).
- `docs/lifecycle-kit/SPEC.md` — the SPEC mirror, regenerated (delta 7).
- `docs/queue-kit/README.md` — the README mirror, regenerated (delta 7).
- `docs/queue-kit/SPEC.md` — the SPEC mirror, regenerated (delta 4).
- `docs/site-kit/README.md` — the README mirror, regenerated (delta 7).
- `docs/canon-kit/SPEC.md` — the SPEC mirror, regenerated (delta 7).
- `.workflow/release-declarations.md` — the bullets (delta 7).
- `docs/posts/2026-07-19-checkwright-v0-7-0.md` — a published release note, deliberately unchanged: it
  records what that release shipped (delta 4).
- `docs/posts/2026-07-19-checkwright-v0-8-0.md` — a published release note, deliberately unchanged
  (delta 4).
- `docs/posts/2026-07-20-checkwright-v0-10-0.md` — a published release note, deliberately unchanged
  (delta 4).

## Retired spellings

- `gen-pre-commit.sh` — the deleted generator's filename, every spelling of its path included (delta 4).
- `--emit-commit-msg` — the script's commit-msg mode, replaced by the arm's `commit-msg` operand
  (deltas 1 and 6).
- `gate_manifest_field` — deleted from `lib/gate.sh` (delta 4).
- `gate_expand_couples` — deleted from `lib/gate.sh`, and it covers `gate_expand_couples_var` as a
  substring (delta 4).
- `--knob-files` — the retired top-level flag (delta 4).

## Definition of Done

- [ ] **Causal completeness** — the arm, the in-process interface and its `Option` each have named
      consumers, and every retired producer's readers moved or retired with it.
- [ ] **Byte parity** — the pre-deletion diff (delta 5) shows only the header command line, recorded in
      the commit message.
- [ ] **`gen=manual` covered** — the tree test's cases pass (delta 5).
- [ ] **Declared requirements re-measured** — `check-graph`'s requirement set taken by spawn census
      (delta 2).
- [ ] **Owed count** — `bash gate-sdk/bin/run-gates.sh --emit port-blockers --tree` no longer lists
      `gate-sdk/bin/gen-pre-commit.sh`.
- [ ] **Instruction surfaces: instruction only** — the kit README and recipe edits carry the command,
      not the grounds.
- [ ] **Merged with no information lost** — §gen-pre-commit reads whole without this file.
- [ ] **Amendment deleted** — this file removed on merge.
- [ ] **Roster re-derived** — per SPEC-roster-rederivation.md delta 1, with missed sites named in the
      commit.
- [ ] **Removals propagated** — `check-amendment-retired-spelling` green.
- [ ] **Settings diff prepared** — the allowlist removal handed to the lead, never applied by hand.
- [ ] **Gaps filed** — through the gap inbox.
