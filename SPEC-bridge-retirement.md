# SPEC amendment: bridge-retirement

Queue entries: `config-seam-static-format`, its last increment, beside
`config-bridge-resolution-cost`, `config-seam-overrides-harness-pin` and `knob-shape-flip-undetected`,
each of which this cut closes. **gate-sdk migrates last and retires the config bridge**, in unit set
`config-seam-fourth-cut` (operator direction, 2026-09-14, lead-relayed, taken with a partial landing
named as a real risk). It lands after SPEC-knob-files-cut-4.md, which leaves gate-sdk the one bridged
kit. The grammar, the precedence and every static rule stand. This amendment moves gate-sdk's knobs into
the crate, removes the machinery that carried the rest, and re-grounds the dispositions that machinery
held up.

**The residue it discharges, measured at `692ad8c2`** (SPEC-knob-files-cut-4.md records the method). A
bare battery pays 120 to 124 ms resolving its knob union and 12 ms sourcing `lib/gate.sh`, against a
per-gate timing sum of 30 664 ms, so the cost `config-bridge-resolution-cost` carries is well under one
percent of a battery. Every harness hook routed through `run-gates.sh` pays the front-end's 17 ms over a
bare binary call, measured on `--emit knob-roster`. The case for this cut was never speed: it is the one
runtime script surface that is neither bash-and-PowerShell nor native.

**What the cut does not move, stated before any delta so no surface overclaims it.** Retiring the bridge
removes the one runtime surface that cannot be dual-implemented. It leaves bash in four places: the
front-end stub, which must locate the binary before any binary runs; the two generated git hooks;
guard-kit's hook, whose rules are bash by SPEC-knob-files-cut-4.md's ruling; and the gate-author and
test libraries sourced into bash callers. docs/install.md's `bash` requirement therefore stays, with its
reason restated (delta 12). gate-sdk/SPEC.md §The knob file's sentence calling this cut *the one that
moves docs/install.md's bash floor for a native Windows runtime* is corrected to that. The
native-Windows floor, a PowerShell twin of the front-end stub beside the surfaces that still need bash,
is its own Deferred entry, `native-windows-bash-floor`, filed with this amendment (operator direction,
2026-09-14, lead-relayed).

## The seam

- **Kit mechanism:** `native/src/knobs/gate_sdk.rs`, the crate's kit-root and gate-sdk-root derivations,
  the `--emit kit-roots` arm, the graph-vocabulary reader, the pre-binary accessors in `lib/gate.sh` and
  the unit test holding them to the crate, the installer's knob-file seam writer, and the reduced
  `lib/gate.sh`, `bin/run-gates.sh`, `bin/gen-pre-commit.sh` and `lib/test-hermetic.sh`.
- **Consumer config:** `<gates-dir>/gate-sdk-config.knobs` with its `.local.knobs` overlay, and
  `<gates-dir>/graph-vocab.knobs`, the graph vocabulary. Both stay the adopter's edit seam.
- **This repo's own:** `scripts/gate-sdk-config.knobs` carries its graph artifact path, its graph-theme
  hosts, its extra lint directories and its install-path corpus. `scripts/graph-vocab.knobs` carries its
  per-kit layer rules. The hosts and the layer vocabulary are this project's own coupling vocabulary and
  product constants, and they stay consumer values, never kit literals.
- **Private rule content:** none newly in reach.

## What changes

### (1) The crate derives the kit roots from a gate-sdk root locator {design-bearing}

The bridge carried three values the crate could not derive, because each was anchored at
`lib/gate.sh`'s own location: `GATE_KIT_ROOTS_HERE`, `GATE_KIT_ROOTS_REL` and `GATE_SDK_ROOT_HERE`. With
no shell library resolving them, the crate needs the anchor handed to it once.

gate-sdk/SPEC.md §Layout and configuration, the `GATE_SDK_ROOT` entry. **Not yet applied:**

> `GATE_SDK_ROOT` — the vendored gate-sdk root, a **locator** read from the environment, default
> `gate-sdk` relative to the working directory. It is not a knob a file can set, for the reason
> `GATE_SDK_GATES_DIR` is not: the binary needs it to find the kits whose knob files it reads.
> `bin/run-gates.sh` exports it on every exec, from its own location, spelled relative to the repo root
> when the root lies under it and absolute otherwise, so every front-end call carries the exact root. A
> call reaching the binary without the front-end, a generated git hook or a harness spawning the binary
> directly, takes the default, which is where `init` vendors gate-sdk. A consumer vendoring it anywhere
> else exports the locator for those calls, as it already exports `GATE_SDK_GATES_DIR` for a relocated
> gates directory.

The crate gains the derivations the shell library held:

- **`walk::sdk_root()`**, the locator's value.
- **`walk::kit_roots_abs()`**: the gate-sdk root, then every sibling under the root's parent holding a
  `checks/` or a `smoke/` directory, in name order, absolutized. `GATE_SDK_KIT_DIRS` (delta 2) replaces
  the set when it is non-empty. This is `_gate_kit_roots_derived` and `gate_kit_roots` ported, and the
  shell forms leave in delta 7.
- **`walk::kit_roots_rel()`**: the same roots relative to the root's parent, the anchor the couples
  globs share. The two spellings are computed together, so the index alignment the bridge could not
  guarantee is now true by construction.
- **`--emit kit-roots`**, a non-gate arm printing one root per line spelled relative to the working
  directory, for the shell callers that still need the set (delta 7).

`GATE_KIT_ROOTS_HERE` retires with no replacement name. It existed because a bridged value could not
carry an absolute path, and an in-process value can. Every reader of it moves to `kit_roots_abs()`.
Every reader of `GATE_SDK_ROOT_HERE` moves to `sdk_root()`, and `CONTEXT_KIT_HOOK_CMD`'s derived row
declares `GATE_SDK_ROOT` in its place.

**A spawning arm absolutizes the locators it hands a child in another working directory.**
§run-gate-tests already absolutizes `GATE_SDK_NATIVE_BIN` and pins `GATE_SDK_TMP_DIR` at the invoker's
root. It absolutizes `GATE_SDK_ROOT` the same way, and so does every arm spawning a battery or a gate in
a scratch tree, whose rosters build derives by grepping `native/src` for `run_merged_in`,
`run_in` and `current_dir`.

**Refused.** Deriving the root from the binary's own path. `init` places the binary in the gates
directory, and nothing relates that path to the kits. Also refused: baking the locators into the
generated hooks. A baked relative locator is right only for the tree that generated it, the default is
right for every conventional tree, and the hooks stay environment-free (delta 8).

### (2) gate-sdk goes static {mechanical}

gate-sdk gains `native/src/knobs/gate_sdk.rs` and joins `STATIC_KITS`, stem `gate-sdk`, locator
`GATE_SDK_KNOB_FILE`. Every name `gate-sdk/lib/gate.sh` defaults transcribes as it stands at this
amendment's commit, with delta 3's shapes:

- **Scalars:** `GATE_SDK_WORKFLOW_DIR` (`.workflow`), `GATE_SDK_TMP_DIR` (`.tmp`), `GATE_SDK_QUEUE_FILE`
  (`TASK-QUEUE.md`), `GATE_SDK_AGENT_FILE` (`CLAUDE.md`), `GATE_SDK_REGISTRY_DOC` and
  `GATE_SDK_RUNNER_DOC` (`README.md`), `GATE_SDK_ENFORCE_SCAN_DIR` (`.`), `GATE_SDK_GIT_EMAIL_FILE`,
  `GATE_SDK_GIT_REMOTES_FILE` and `GATE_SDK_GH_HOSTS_FILE` (empty), `GATE_SDK_GH_HOST` (`github.com`),
  `GATE_SDK_UPGRADE_REPO` and `GATE_SDK_UPGRADE_FROM` (empty), `GATE_SDK_UPGRADE_TO` (`HEAD`),
  `GATE_SDK_GRAPH_MAX_EDGES` (`100000`), `GATE_SDK_NATIVE_CRATE` (`native`) and
  `GATE_SDK_NATIVE_PUBLISH_WORKFLOW` (`.github/workflows/publish.yml`).
- **Derived from the gates-directory locator:** `GATE_SDK_HOOKS_DIR`, `GATE_SDK_ROOT_ALLOWLIST`,
  `GATE_SDK_CORE_FILES_FILE`, `GATE_SDK_IDENTITY_FILE`, `GATE_SDK_TESTS_DIR`, `GATE_SDK_GRAPH_ARTIFACT`,
  `GATE_SDK_GRAPH_THEME_DIR` and `GATE_SDK_GRAPH_VOCAB` (delta 4).
- **Derived from `GATE_SDK_NATIVE_CRATE`:** `GATE_SDK_NATIVE_SRC`, `GATE_SDK_NATIVE_TARGETS_FILE`,
  `GATE_SDK_NATIVE_RUNNERS_FILE` and `GATE_SDK_CARGO_TARGET_DIR`. The crate's trailing `/` is stripped
  where the row resolves, once, as `gate_native_crate` stripped it.
- **`GATE_SDK_NATIVE_BIN`**, derived: `native/target/release/checkwright-gates` followed by
  `std::env::consts::EXE_SUFFIX` (delta 5).
- **Indexed:** `GATE_SDK_PROGRAM_FLOOR`, with its library default.
- **Delta 3's whitespace scalars**, each with its library default.

**An empty value takes the default where the library's resolution did.** The library treated an empty
value as unset for most of these, through a `[[ -n … ]]` guard or a `${NAME:-…}` default, so an exported
empty value fell back to the default, where the static precedence keeps an exported empty value. `Row`
gains `empty_takes_default`, set on exactly those rows, and `knobs::resolve` answers the default for an
empty layered value on such a row. The build roster is `grep -n -e '\[\[ -n "\${GATE_SDK_' -e
'\${GATE_SDK_[A-Z_]*:-' gate-sdk/lib/gate.sh`, and a row the library guarded with `[[ -v … ]]` keeps an
exported empty value. A unit test holds each flagged row to the fallback.

**Environment-only names are refused in a file.** `GATE_SDK_GATES_DIR` and `GATE_SDK_ROOT` are
locators, and `GATE_SDK_JOBS` and `GATE_SDK_VERBOSE` are execution settings the runner reads from its
own environment (§run-gates). None is a row. `Kit` gains `env_only`, and a file line naming one is
refused with *set it in the environment*, where the generic refusal would only name the roster.

**`GATE_SDK_GRAPH_THEME` joins the retired-name table**, replacement `GATE_SDK_GRAPH_THEME_DIR`. The
static refusal of an exported retired name is the trap `native/src/gates/graph.rs` hand-writes today,
and that trap is deleted.

### (3) A whitespace-list knob keeps its grammar, and its resolved array spelling retires {design-bearing}

Ten of gate-sdk's consumer knobs are whitespace-separated scalars that `lib/gate.sh` split into nine
arrays under second names, `GATE_SDK_PRUNE_EXTRA_DIRS` appending to `GATE_SDK_PRUNE_DIRS`' array,
because one name meaning two grammars is the defect §lib/gate.sh's prefix rule exists against. The
second names were never consumer-facing. They existed to carry the split values across the tab-joined
wire.

- **Each keeps its consumer name and its scalar grammar**, split on whitespace by its reader:
  `GATE_SDK_PRUNE_DIRS`, `GATE_SDK_PRUNE_EXTRA_DIRS`, `GATE_SDK_EXEC_GLOBS`, `GATE_SDK_EXEC_PRUNE`,
  `GATE_SDK_LINT_EXTRA_DIRS`, `GATE_SDK_GRAPH_EXTERNAL_REFS`, `GATE_SDK_MSG_PATTERN_FILES`,
  `GATE_SDK_MSG_PATTERN_FILES_LOCAL`, `GATE_SDK_PORTABILITY_PATTERNS` and `GATE_SDK_PORTABILITY_PATHS`,
  with `GATE_SDK_KIT_DIRS` and `GATE_SDK_COMMIT_TYPES` beside them. So an exported override still works,
  which an indexed knob would not allow.
- **The nine resolved spellings retire:** `GATE_PRUNE_DIRS`, `GATE_EXEC_GLOBS`, `GATE_EXEC_PRUNE`,
  `GATE_LINT_EXTRA_DIRS`, `GATE_GRAPH_EXTERNAL_REFS`, `GATE_MSG_PATTERN_FILES`,
  `GATE_MSG_PATTERN_FILES_LOCAL`, `GATE_PORTABILITY_PATTERNS` and `GATE_PORTABILITY_PATHS`.
  `GATE_SDK_KIT_DIRS`' resolved spellings are delta 1's.
- **The composition moves to the reader, on cut 3's `_EXTRA` precedent.** `walk::prune_dirs()` is the
  resolved `GATE_SDK_PRUNE_DIRS` followed by `GATE_SDK_PRUNE_EXTRA_DIRS`, and every declarer of
  `GATE_PRUNE_DIRS` declares the two knobs instead. `GATE_SDK_EXEC_GLOBS`' default is derived, the three
  kit globs then `<gates-dir>/check-*.sh` and `<gates-dir>/kpi-*.sh`. The pattern-file and portability
  defaults derive from the gates directory.
- **The split is not a shell expansion.** The library expanded `GATE_SDK_MSG_PATTERN_FILES`,
  `GATE_SDK_MSG_PATTERN_FILES_LOCAL`, `GATE_SDK_PORTABILITY_PATTERNS`, `GATE_SDK_PORTABILITY_PATHS` and
  `GATE_SDK_KIT_DIRS` unquoted, so a glob in the value was expanded against the working directory. The
  reader splits on whitespace and expands nothing. No value in this tree carries a glob, and the release
  declaration names the change (delta 14).

**Refused.** Indexed knobs under new names (`GATE_SDK_PRUNE_DIRS[] = target`). It renames or regrammars
twelve consumer knobs, and it removes the environment override fixtures and adopters use today.

### (4) The graph vocabulary is a document the member reads {design-bearing}

`scripts/graph-vocab.sh` is consumer rule content the library sources so the bridge can carry six
unprefixed globals. With no bridge, it becomes what §lib/gate.sh already calls a configurable document:
a path the member reads.

gate-sdk/SPEC.md §check-graph, the vocabulary paragraph. **Not yet applied:**

> **The vocabulary file is `GATE_SDK_GRAPH_VOCAB`**, default `<gates-dir>/graph-vocab.knobs`, written in
> the knob-file grammar (§The knob file) and read by `check-graph` and `--emit graph` through one reader.
> It declares six names: `GRAPH_VOCAB`, `GRAPH_LEADING`, `GRAPH_LAGGING`, `GRAPH_LAYERS` and
> `GRAPH_LAYER_RULES` indexed, and `GRAPH_LAYER_DEFAULT` a scalar, default `surfaces`. An absent file
> leaves every indexed name empty, which disables the checks each drives, exactly as an absent sourced
> file did. The file is a document and not a kit's knob file: it takes no environment override and no
> local overlay, and a line naming any other name, or a form disagreeing with a name's shape, is refused
> with its file and line, exit 2. A `<gates-dir>/graph-vocab.sh` left beside it is refused at exit 2,
> naming the migration, for the reason every legacy shell config is.

`native/src/graph_vocab.rs` holds the reader over `knobfile::parse`. The couple from each reader's
descriptor to the vocabulary is `knob:GATE_SDK_GRAPH_VOCAB`, which expands to the resolved path. A unit
test covers the absent file, each shape refusal, an undeclared name and the legacy refusal.

**Refused.** Folding the six names into `gate-sdk-config.knobs` as `GATE_SDK_GRAPH_*` rows. It renames
every name a consumer's vocabulary carries, and it moves rule content into the file that holds layout,
where CLAUDE.md §The provenance seam names the separate vocabulary file as the pattern. Keeping the
unprefixed names is safe here because the member, not the prefix-owning loader, reads the file.

### (5) The pre-binary accessors, and the executable suffix {design-bearing}

A shell caller that must reach the binary, or build it, needs a few gate-sdk values before any binary
can answer: the binary's path, the crate root and the two platform rosters under it, and the
banned-pattern files `bin/build-native.sh` checks a fresh artifact against. These six are the **pre-binary
knobs**: `GATE_SDK_NATIVE_BIN`, `GATE_SDK_NATIVE_CRATE`, `GATE_SDK_NATIVE_TARGETS_FILE`,
`GATE_SDK_NATIVE_RUNNERS_FILE`, `GATE_SDK_MSG_PATTERN_FILES` and `GATE_SDK_MSG_PATTERN_FILES_LOCAL`.

gate-sdk/SPEC.md §lib/gate.sh, a new bullet. **Not yet applied:**

> **The pre-binary accessors read their knob in shell, and a crate test holds them to the table.**
> `gate_native_bin`, `gate_native_crate`, `gate_native_targets_file`, `gate_native_runners_file` and
> `gate_msg_pattern_files` resolve their knobs through `_gate_prebinary_knob`, which reads the scalar form
> alone, `NAME = value`, over the static precedence: the environment, the local overlay, the tracked
> file (`GATE_SDK_KNOB_FILE` or `<gates-dir>/gate-sdk-config.knobs`), and the default, which it derives
> as the table does. Every other shell read of a gate-sdk knob goes through `gate_knob_values`, which asks
> the binary. The accessors are a second holder of six values, admitted on criterion 6's *unless*
> clause, and the machine that holds them is `native/src/knobs/gate_sdk.rs`' unit test: it spawns the five
> accessors under an environment value, an overlay value, a tracked value, an empty value and nothing,
> and compares each answer with `knobs::resolve`. The accessor does not refuse a malformed line it skips.
> The binary refuses it at its first gate-sdk read, which every arm reaches.

**The executable suffix gets a compiled answer for the host and none for a target.**
`GATE_SDK_NATIVE_BIN`'s host default reads `std::env::consts::EXE_SUFFIX`, the standard library's
constant for the platform the binary was built for, which is the platform it runs on.
`gate_exe_suffix` stays the shell owner of the target form, for `bin/build-native.sh` and
`scripts/ci-build-artifact.sh`, and the accessor test above covers its host form through the default.
**`GATE_SDK_NATIVE_ARTIFACT_NAMES` retires.** `--pack-installer` takes each roster target's artifact
name by discovery instead: the target's artifact directory must hold exactly one regular file not named
`*.sha256`, beside its sidecar, and zero or several is a refusal naming the directory. That is the
bootstrap's own `select_artifact` rule (§lib/gate.sh, the `gate_exe_suffix` bullet), so the packer and
the bootstrap agree on the name without either spelling `.exe`.

**Refused.** A crate twin of `gate_exe_suffix`'s target predicate. It is a second holder where discovery
needs none. Also refused: making the six pre-binary knobs locators. Their values are consumer config the
installer writes into the file, and the binary reads them too.

### (6) The wire retires {design-bearing}

- **`knobs::wire`** loses its bridged branch. A name no static kit owns is refused: *X is not a
  statically owned knob*. `walk::knob_wire`'s *invoked without the config bridge* message goes with it.
- **`--knobs`, `knobs::bridged`, `bridged_inputs`, `reaches_bridged` and `Origin::Bridged` are deleted.**
  With no bridged name, the closure is empty for every member, so the arm has nothing to answer.
  The validator's skip for a row derived from a bridged input has no case, and the referent refusal and
  SPEC-knob-files-cut-4.md's declared-referent exception have none either, so both bullets are deleted.
  `Row.referents` is deleted with them.
- **`Origin::Placeholder` stays, for the two locators.** `--emit knob-roster` renders a locator input as
  `${GATE_SDK_GATES_DIR}` or `${GATE_SDK_ROOT}`, and the probing derivations still render their first
  candidate for it.
- **The registry's declared knobs stay** (`gates/mod.rs`, `emit/mod.rs`' arm table, `hook/mod.rs`),
  because `--knob-files`, `check-reads-couples` and `check-gate-substrate-parity` read them in process.
  `BRIDGED_ARMS` is renamed `ARMS`. `registry::EVERY_COUPLES_KNOB` stays a declaration, read by the
  knob-file derivation, and loses its bridge expansion. `registry::couples_knob_names` returns every
  `knob:` token's name.
- **`runner.rs`** stops building the knob union, stops stripping and re-applying it per child
  (`child_knobs`, and `drop_env` in `proc::dispatch`), and hands each member the invoking environment
  unchanged.
- **Tests.** Every `knobenv` write of `GATE_SDK_KNOB_<NAME>` becomes a write of the knob's own scalar name
  or a scratch knob file under the case's `<KIT>_KNOB_FILE`: `knobs/mod.rs`, `registry.rs`, `walk.rs`,
  `emit/enforcement_map.rs` (moved to the evidence knob file by SPEC-knob-files-cut-4.md),
  `emit/port_blockers.rs` and `emit/lesson_sink.rs`. The `GATE_SDK_KNOB_` string fixtures in
  `emit/run_gate_tests.rs` and `runner.rs` are deleted with the code they test. The test helper
  `walk::bridge_declared_knobs` is deleted, and its callers in `knobenv.rs`, `proc.rs`, `gates/mod.rs`
  and `walk.rs` read the table's defaults. `knobenv.rs` keeps its lock, whose subject is now every
  environment write a case makes.

### (7) lib/gate.sh without the bridge {design-bearing}

`gate-sdk/lib/gate.sh` keeps the gate-author and caller API and loses everything that resolved a value.

- **Deleted:** the config-seam source, every resolved global and guarded default, `_gate_knob_emit`,
  `_gate_knob_pairs`, `_gate_knob_prefix_emit`, `_gate_knob_kit_emit`, `_gate_knob_owning_kit`,
  `_gate_knob_owner_candidates`, `_gate_knob_owner_match`, `gate_knob_env`, `gate_knob_env_set`,
  `gate_knob_env_one`, `GATE_SDK_RESOLVING_KNOB`, `gate_static_knob`, `GATE_SDK_COUPLES_KNOB_SENTINEL`,
  `_gate_couples_knob_names`, `_gate_couples_knob_bridge`, `_gate_couples_static_knob_names`,
  `_gate_couples_knob_partition`, `_gate_kit_roots_derived`, `gate_kit_roots_rel` and its cache, and
  `gate_commit_types`, which has no caller and whose value now lives in the table. The graph-vocabulary
  source goes with delta 4.
- **`gate_command`** emits the one-element `<dir>/<name>.sh` or the two-element `<binary> <name>`, never
  an `env` prefix. §lib/gate.sh's *a caller needing the executable takes the first element that is
  neither `env` nor an assignment* rule goes, and so does §run-gate-tests' instance of it.
- **`gate_knob_values`** runs `"$(gate_native_bin)" --emit-knob-values "$@"` directly.
- **`gate_expand_couples_var`** reads every `knob:` token's members from one `gate_knob_values` call over
  the token set, once per process, held by name.
- **`gate_kit_roots`** reads `--emit kit-roots`, and `gate_check_dirs` composes over it unchanged.
  `gate_sdk_root` stays the library's own location, which is what `bin/run-gates.sh` exports.
- **`gate_find`, `GATE_GREP_EXCLUDES` and `gate_path_pruned`** take the prune set from one
  `gate_knob_values GATE_SDK_PRUNE_DIRS GATE_SDK_PRUNE_EXTRA_DIRS` read on first use, split on
  whitespace, the reader-side composition of delta 3.
- **The pre-binary accessors** are delta 5's.
- `gate_resolve`, `gate_manifest_field`, `gate_staged_matches`, `gates_list_members`,
  `gate_native_source_stamp`, `gate_native_targets`, `gate_native_runner`, `gate_exe_suffix`,
  `gate_authoring_tree`, `gate_native_module`, `gate_self_repo_prefix`, `gate_sdk_gates_dir` and
  `fail_closed` are unchanged.

The crate tests that spawn the library change with it: `registry.rs`' couples-sentinel test keeps its
literal half against the crate and drops the shell half; `knobs/mod.rs`' gates-directory default test
stays; `gates/mod.rs`' `resolve_gates_dir` helper stays; `main.rs`' source-stamp test stays.

### (8) The front-ends and the hermetic harness {design-bearing}

- **`bin/run-gates.sh`** resolves the repo root, sources `lib/gate.sh` for `gate_native_bin` and
  `gate_sdk_gates_dir`, resolves the gates-dir positional exactly as now, exports `GATE_SDK_ROOT`
  (delta 1), and `exec`s the binary with no knob environment. `ARM_UNAVAILABLE_STATUS` is unchanged.
- **`bin/gen-pre-commit.sh`** emits `run_gate <name> <binary> <name>` with no `env` prefix. Its
  `REL_DIRS` read `gate_kit_roots`. Its knob-token expansion is delta 7's. The two generated hooks carry
  no knob environment, so a knob edit changes a hook only through a trigger, which the derived knob-file
  couple already supplies.
- **`bin/run-consumer-smoke.sh`**'s unregistered-gate probe calls `gate_command`, now bridge-free, from
  the scratch consumer's working directory, where the binary reads that consumer's knob files. The probe
  still cannot delegate to `run-gates.sh --only`, whose subject is the registry.
- **`lib/test-hermetic.sh`** pins `GATE_SDK_KNOB_FILE` beside every other kit's, since gate-sdk's name
  no longer lands in a wire namespace, and pins `GATE_SDK_CONFIG_FILE` to the shared empty file as it
  pins every retired locator. It exports an absolute `GATE_SDK_ROOT` from its own anchor, beside the
  absolute `GATE_SDK_NATIVE_BIN` it already exports. `gate_run` executes `gate_command`'s argv, and
  `gate_arm_run` executes `<binary> <arm> <argv>`.

### (9) The installer writes the knob-file seam {mechanical}

- **`native/src/install.rs`' `seam_text`** writes `GATE_SDK_NATIVE_BIN = <dest>` into
  `<gates-dir>/gate-sdk-config.knobs`, keeping every other line and dropping any earlier line whose head
  is `GATE_SDK_NATIVE_BIN`. `init` (`native/src/installer/init.rs`) and `--install place-artifact` pass
  the `.knobs` path, and `native/src/installer/doctor.rs` reads the binary path from that line.
- **A left-behind `gate-sdk-config.sh` is not deleted.** installer/README.md §What init seeds rules that
  `init` never deletes a file a release stopped writing, and the loader's legacy refusal names the
  migration. So an adopter upgrading from an installed tree meets that refusal until the file is deleted,
  and the release declaration says so.
- **The Windows leg** in `.github/workflows/gates.yml` passes `--seam scripts/gate-sdk-config.knobs` and
  matches the new `own` record. `installer/consumer-smoke/run-smoke.sh` reads the seam's `.knobs` path at
  its four ownership checks, and parses the binary line in the knob grammar.
- installer/README.md §The gate binary, §doctor and §The consumer smoke name the `.knobs` seam.

### (10) The dispositions the bridge held {design-bearing}

Six `# no-port:` declarations and two class sections stand on the bridge. Each is re-read against the
ground it states.

- **gate-sdk/SPEC.md §The kit-library port disposition** has no member left: evidence-kit's library is
  deleted by SPEC-knob-files-cut-4.md, guard-kit's keeps only its extension-point ground, and
  `lib/gate.sh` is no longer the bridge. The section is deleted, and each citation to it is re-pointed at
  the ground its citing site actually rests on.
- **gate-sdk/SPEC.md §The config-seam port disposition** keeps its ground, stated for the knob files,
  and loses every shell member: the section shortens to the ground and the derivation.
- **`gate-sdk/lib/gate.sh`** declares on two grounds its section states: its API is shell functions
  sourced into bash callers (CI steps, the installer smoke, the author template, the front-ends and the
  test harness), and a binary arm cannot be sourced into bash; and its pre-binary accessors answer before
  any binary exists. That is `lib/test-hermetic.sh`'s first limb and `bin/build-native.sh`'s bootstrap
  cause, each already in the tree.
- **`gate-sdk/bin/run-gates.sh`** declares on the bootstrap cause: the front-end locates the binary it
  executes, which the binary cannot do for itself.
- **`gate-sdk/lib/test-hermetic.sh`** keeps its first limb and loses its second, which cited the bridge.
- **`gate-sdk/bin/gen-pre-commit.sh`**, **`gate-sdk/bin/run-consumer-smoke.sh`** and
  **`gate-sdk/lib/consumer-smoke.sh`** have no ground left: each declared on the bridge's single
  producer, and the crate is now that producer. They become **owed**, their `# no-port:` lines are
  deleted, and one Deferred entry per port is filed at landing. §Consumer smoke *The port disposition*
  loses leg 1, and its opening count and reopening condition follow. gen-pre-commit.sh's declaration
  carried an operator ratification, which no TRAJECTORY.md ruling records, and §gen-pre-commit named
  its reversal as one of two open options. **The operator reopened it** (operator direction,
  2026-09-14, lead-relayed, a direction and not a consult ruling), so the move to owed is that
  authority's own. gate-sdk/SPEC.md §gen-pre-commit, the paragraphs from *This generator does not
  port* through *The `# no-port:` declaration above closes neither of them*. **Not yet applied:**

  > **This generator is owed.** Its `no-port` declaration rested on criterion 6's single-producer rule:
  > the hook baked a knob the owning kit's shell library resolved, so a crate-side emitter would have
  > been the second producer. The crate is now every knob's one producer, and that ratification was
  > reopened by the authority that gave it, so the ground is gone and the declaration with it. The port
  > is the emit-arm path this section once declined for now, moving `--emit` into the binary, and its
  > own queue entry owns it. Until it lands, `check-graph` assertion D keeps spawning `bash
  > bin/gen-pre-commit.sh`, declared in its `--needs` element, and criterion 7 still clears the spawn
  > because `bash` is on the program floor.

  The date and channel stay in this amendment and the landing commit, never in the kit SPEC (CLAUDE.md
  §The provenance seam). The residue paragraphs on `--needs`, the absent-`bash` branch and the measured
  bound stay, re-read as the owed state's.

### (11) The rules that existed for two seams, and the three closed entries {design-bearing}

- **gate-sdk/SPEC.md §The knob file.** The unruled-shapes list is deleted: its last bullet is this cut.
  *While migration is in progress the two seams resolve in different orders*, *A bridged config reading
  a static knob is a transitional shape*, the reference refusal for a bridged referent, and the
  derived-default paragraph's bridged-input rules are deleted. The *environment beats the file* paragraph
  keeps its reason and drops *It does not dissolve it for a bridged kit*. The legacy-refusal paragraph
  gains gate-sdk's two locators. *The one knob the file cannot set is `GATE_SDK_GATES_DIR`* becomes
  delta 2's environment-only list, and the gates-directory literal's *until gate-sdk migrates* clause is
  deleted.
- **gate-sdk/SPEC.md §Layout and configuration.** The loader paragraph (*`lib/gate.sh` auto-sources the
  consumer config seam*) is replaced by the knob file and its two layers, and *Which of the file and the
  environment wins is the file's own choice* is deleted with the seam it described. Every roster entry's
  *resolved in `lib/gate.sh` rather than inline … the config bridge's undeclared-knob refusal* clause is
  deleted, keeping the knob's default and reader. `GATE_SDK_KNOB_<NAME>`'s entry is deleted.
- **gate-sdk/SPEC.md §lib/gate.sh.** *The array-knob config bridge* and every bullet under it are deleted:
  the owning-kit derivation, the memo rule, the `declare -p` visibility rule, the absolute-path rule, the
  resolving-knob set, the batch, the tab serialization, the keyed arm, the prefix family, the four
  refusals, the keyed arm's live fail-open history, `env` in the dispatch path, and the bridge's
  process-global serialization. The `knobenv` paragraph keeps its lock rule restated for every
  environment write. *So a knob has exactly one producer* stays, stated without a bridged case.
- **`knob-shape-flip-undetected` closes.** Its residue paragraph, *A reader taking a knob as an array
  when its consumer has since redeclared it `declare -A`*, is deleted with the keyed arm: no shell
  declaration reaches a reader, and a knob file refuses a line whose form disagrees with the declared
  shape.
- **`config-seam-overrides-harness-pin` closes.** §run-gate-tests' paragraph *The pin is not
  authoritative, and that is a known hole* is replaced by one sentence: the pin is an environment value,
  and a scalar's environment value outranks every file. A unit test sets `GATE_SDK_TMP_DIR` in a case's
  knob file and asserts the pinned value reaches the member.
- **`config-bridge-resolution-cost` closes.** No union is resolved and no library is sourced to launch a
  battery. The measurement above is recorded in §run-gates' front-end paragraph as the cost the cut
  removed.
- **§The port-candidate criteria, criterion 6**: *For a bridged knob the criterion is discharged by
  construction* becomes *for a static knob*, and delta 5's accessors are the clause's machine-held
  instance.

### (12) The prose sweep {mechanical}

Every governed surface that names the bridge is resolved. Each mention is deleted with its subject,
re-grounded where it states a ground, or rewritten where it describes a flow that survives without the
bridge. The roster is `git grep -n -i -e 'config bridge' -e 'bridged arm' -e 'bridged non-gate arm' -e
'bridged-arm table' -e 'GATE_SDK_KNOB_'`, run at build. `GATE_SDK_KNOB_FILE` is the one legitimate
survivor of the last pattern. The retired spellings below are its checksum.

Named because each states a rule rather than narrating the flow:

- gate-sdk/SPEC.md §The non-gate arm and §The harness-integration arm: *a bridged non-gate arm* and *the
  bridged-arm table* become *a non-gate arm* and *the arm table*, and the forced-family test is restated:
  a hardcoded top-level flag receives no row in the arm table, so `--knob-files`, the couples check and
  the substrate-parity check cannot see what it reads.
- gate-sdk/SPEC.md §run-gates: the front-end paragraph's residue list, the *one `gate_knob_env` call*,
  and the port-disposition paragraph follow delta 10.
- lifecycle-kit/SPEC.md §bin/enter-stage.sh, the *second caller resolving the binary and the bridged
  environment itself*, reduces to resolving the binary.
- docs/install.md's `bash` requirement line states the surfaces that still need bash, the front-end,
  the two git hooks and guard-kit's hook, in place of *the shell library every gate loads through*.
- CLAUDE.md §The provenance seam names `scripts/graph-vocab.knobs` as the pattern.

### (13) Configs, fixtures and regeneration {mechanical}

- `git mv scripts/gate-sdk-config.sh scripts/gate-sdk-config.knobs`, rewritten:
  `GATE_SDK_GRAPH_ARTIFACT = docs/check-graph.html`, `GATE_SDK_GRAPH_EXTERNAL_REFS = …`,
  `GATE_SDK_LINT_EXTRA_DIRS = …` and `GATE_SDK_PORTABILITY_PATHS = …`, each value verbatim. The `spec:`
  comments survive as `#` lines; the `# shellcheck` and `# no-port:` lines go.
- `git mv scripts/graph-vocab.sh scripts/graph-vocab.knobs`, one `GRAPH_LAYER_RULES[] =` line per rule,
  one `GRAPH_LAYERS[] =` line per layer, and `GRAPH_LAYER_DEFAULT = …`.
- `scripts/core-files.list` names the two `.knobs` files. `scripts/portability-patterns.list`' header
  comment names the `.knobs` corpus.
- **Fixtures and tests.** Every test writing a gate-sdk config or a graph vocabulary writes knob-file
  lines under `GATE_SDK_KNOB_FILE` or `GATE_SDK_GRAPH_VOCAB`. `gate-sdk/gate-tests/lib-gate.test.sh` loses
  its bridge cases and keeps the helper cases. `knob-family-parity.test.sh` is deleted, because both arms
  it compares are gone. `check-reads-couples.test.sh`, `run-dispatch-streams.test.sh`,
  `run-gate-tests.test.sh`, `check-graph-refs.test.sh`, `check-graph-tree.test.sh` and
  `run-arm-contract.test.sh`, whose knob-union section is deleted with the union, follow. The `args`
  comments naming `gate-sdk-config.sh` in `check-install-disposition`, `check-portability-floor`,
  `check-readme-roster`, `check-smoke-entry-guard` and `check-template-registry-parity` name the knob
  file. `native/src/emit/agents_md_smoke.rs`' scratch config names the knob file. The roster is
  `git grep -l -e GATE_SDK_CONFIG_FILE -e gate-sdk-config -e graph-vocab -e GATE_SDK_KNOB_`, run at build.
- Regenerate the pre-commit and commit-msg hooks, the graph, every kit SPEC mirror and the knob roster's
  readers.

### (14) The release declarations {mechanical}

`.workflow/release-declarations.md`. **Not yet applied:**

- Renamed knobs: `GATE_SDK_CONFIG_FILE` → `GATE_SDK_KNOB_FILE`. `GATE_SDK_NATIVE_ARTIFACT_NAMES` → ∅,
  never set by a consumer. The nine resolved `GATE_*` arrays and `GATE_KIT_ROOTS_HERE`,
  `GATE_KIT_ROOTS_REL` and `GATE_SDK_ROOT_HERE` → ∅, never consumer-authored.
- Behavior changes: **`<gates-dir>/gate-sdk-config.sh` and `<gates-dir>/graph-vocab.sh`** are replaced by
  `gate-sdk-config.knobs` and `graph-vocab.knobs`. A left-behind shell file is refused at exit 2,
  **including the `gate-sdk-config.sh` an earlier `init` wrote beside the binary**: delete it; `init` now
  writes the binary's path into the `.knobs` file. An exported gate-sdk scalar now outranks the file,
  where a bare assignment in the shell config used to win. A whitespace-list knob no longer expands a glob
  in its value. **The config bridge is removed**: `gate_knob_env`, `gate_knob_env_set`,
  `gate_knob_env_one`, `gate_static_knob` and `GATE_SDK_KNOB_<NAME>` no longer exist, `gate_command`
  never prefixes `env`, and `gate_fixture_suites`, `gate_kit_roots_rel` and `gate_commit_types` are gone.
  A binary call made without `run-gates.sh` reads `GATE_SDK_ROOT` from the environment, default
  `gate-sdk`.
- Tightened gates: every member reading a gate-sdk knob, the set build derives from
  `checkwright-gates --knob-files` and a per-gate refusal probe over a scratch tree holding a left-behind
  `gate-sdk-config.sh`, cut 3's method.

## Producers and consumers

- **`GATE_SDK_ROOT` and the kit-root derivations** (new interface). *Producer:* `bin/run-gates.sh`'s
  export, `lib/test-hermetic.sh`'s export, a spawning arm's absolutization, or the default.
  *Consumers:* `walk::sdk_root`, `walk::kit_roots_abs` and `walk::kit_roots_rel`, read by every member
  that declared the three retired names, by `registry::fixture_suites` and by `CONTEXT_KIT_HOOK_CMD`'s
  derivation; `--emit kit-roots`, read by `gate_kit_roots`.
- **`native/src/knobs/gate_sdk.rs`** (new state). *Producer:* the table. *Consumers:* `knobs::resolve` for
  every gate-sdk read, the two roster arms, the undeclared-name and environment-only refusals, and the
  pre-binary accessor test.
- **`Row.empty_takes_default` and `Kit.env_only`** (new fields). *Readers:* `knobs::resolve` at
  resolution, and `knobs::layer` at parse, respectively.
- **The graph-vocabulary reader** (changed interface). *Producer:* `graph_vocab::read` over the path.
  *Consumers:* `check-graph`'s vocabulary and leading-lagging assertions, and `--emit graph`'s layer
  grouping. Each declared name is read by the assertion or grouping it names.
- **`_gate_prebinary_knob`** (new interface). *Producer:* the knob file and the environment. *Consumers:*
  the five accessors, and through them the front-ends, CI, `bin/build-native.sh`,
  `scripts/ci-build-artifact.sh`, the installer smoke, the session-context hooks and the test harness.
- **The artifact-name discovery** (changed interface). *Producer:* the artifact directory's contents.
  *Consumer:* `--pack-installer`, which reads the discovered name to select the binary and its sidecar.
- **The knob-file seam line** (changed interface). *Producer:* `install::seam_text`. *Consumers:*
  `_gate_prebinary_knob`, `knobs::resolve` and `installer/doctor.rs`.
- **Red conditions (point 5).** Deltas 6, 7 and 10 narrow corpora and delete readers.
  - *`check-docs-cmd` assertion B's known-knob set* reds on a documented name absent from the set. The
    table now covers every gate-sdk row, but a documented retired name reds unless delta 12 removed its
    mention. Build runs the gate.
  - *`check-knob-default-coupling`* reds on a SPEC default disagreeing with its source. Every gate-sdk
    default moves source, from the library to the table. Build runs it.
  - *§port-blockers' `--tree` counts and every measured claim over them* read exact counts. Two shell
    configs leave the scanned set, and three files move from `no-port` to `owed`, so the owed count
    rises. `check-measured-claim` re-runs its oracles, and build runs the battery rather than inspecting.
  - *`check-graph` assertion D* reds on a stale hook, and every baked argv changes. Delta 13 regenerates.
  - *`check-reads-couples`* compares declarations with couples and holds each `knob:` token inside its
    member's declaration. Every declarer of a retired name declares its replacement, and build runs it.
  - *`check-gate-substrate-parity`* reads the registry, the kit roots and `GATE_SDK_ROOT_HERE`. It moves
    to the derivations, and build runs it.
  - *`check-test-hermetic`* reds on a bespoke test that does not source the harness first. No test's first
    line changes.
  - *`check-core-files`* reds on a listed path that is absent or untracked, so the two `.knobs` names
    and the moves land in one commit.
  - *`check-amendment-retired-spelling`* reds on a survivor outside the roster below, the sweep's checksum.
  - *The installer smoke and the Windows leg* red on a seam at the old path. Delta 9 moves every reader.
  - *The upgrade suite* reds on a phase-B red outside TO's declaration, which delta 14's Tightened-gates
    list must contain.

## Existing sections updated

- `gate-sdk/SPEC.md` — §Layout and configuration, §The knob file, §The config-seam port disposition,
  §The kit-library port disposition deleted, §The port-candidate criteria, §The non-gate arm, §The
  harness-integration arm, §Consumer smoke, §lib/gate.sh, §lib/test-hermetic.sh, §run-gates,
  §run-gate-tests, §gen-pre-commit, §check-graph, §Consumer payload (all deltas).
- `canon-kit/SPEC.md` — bridge mentions and the prune and kit-root names (deltas 1, 3 and 12).
- `context-kit/SPEC.md` — bridge mentions and `CONTEXT_KIT_HOOK_CMD`'s input (deltas 1 and 12).
- `delegation-kit/SPEC.md` — bridge mentions and the kit-root name (deltas 1 and 12).
- `doctrine-kit/SPEC.md` — bridge mention (delta 12).
- `drift-kit/SPEC.md` — bridge mentions, the arm-table name, the vocabulary file (deltas 4, 6 and 12).
- `evidence-kit/SPEC.md` — bridge mentions (delta 12).
- `guard-kit/SPEC.md` — bridge mentions, the arm-table name, the kit-root name (deltas 1, 6 and 12).
- `lifecycle-kit/SPEC.md` — bridge mentions, the second-caller paragraph (deltas 1, 3 and 12).
- `queue-kit/SPEC.md` — bridge mentions (delta 12).
- `site-kit/SPEC.md` — bridge mention (delta 12).
- `installer/README.md` — the seam, §doctor, the consumer smoke, bridge mentions (deltas 9 and 12).
- `docs/install.md` — the `bash` requirement line (delta 12).
- `CLAUDE.md` — the provenance seam's pattern name (deltas 4 and 12).
- `native/src/knobs/mod.rs` — `STATIC_KITS`, the wire, the closure, the origins, the new fields (deltas 2
  and 6).
- `native/src/knobs/gate_sdk.rs` — new table and the pre-binary accessor test (deltas 2 and 5).
- `native/src/knobs/context_kit.rs` — the root input (delta 1).
- `native/src/knobs/evidence_kit.rs` — the root and kit-root inputs SPEC-knob-files-cut-4.md gave it
  (delta 1).
- `native/src/graph_vocab.rs` — new reader (delta 4).
- `native/src/walk.rs` — the root derivations, the prune composition, the bridge readers and test helper
  (deltas 1, 3 and 6).
- `native/src/registry.rs` — the couples names and the sentinel test (deltas 6 and 7).
- `native/src/runner.rs` — the union (delta 6).
- `native/src/proc.rs` — `drop_env`, the test helper call (delta 6).
- `native/src/main.rs` — the `--knobs` arm (delta 6).
- `native/src/knobenv.rs` — the lock's charter, the helper call (delta 6).
- `native/src/gates/mod.rs` — declarations, the helper call (deltas 1, 3 and 6).
- `native/src/emit/mod.rs` — declarations, `ARMS`, the `--emit kit-roots` row (deltas 1, 3 and 6).
- `native/src/hook/mod.rs` — declarations and the `--knobs` sentinel comment (delta 6).
- `native/src/gates/graph.rs` — the vocabulary reader, the root, the retired-theme trap (deltas 1, 2 and 4).
- `native/src/emit/graph.rs` — the vocabulary reader, the roots (deltas 1 and 4).
- `native/src/gates/gate_substrate_parity.rs` — the root (delta 1).
- `native/src/gates/commit_msg.rs` — the pattern-file reader (delta 3).
- `native/src/gates/exec_bit.rs` — the two lists (delta 3).
- `native/src/gates/shellcheck.rs` — the extra dirs (delta 3).
- `native/src/gates/portability_floor.rs` — the two lists (delta 3).
- `native/src/gates/docs_nav_reachable.rs` — the prune set (delta 3).
- `native/src/gates/reads_couples.rs` — bridge comment (delta 12).
- `native/src/gates/root_tiering.rs` — the vocabulary pattern's name in a comment (delta 13).
- `native/src/emit/pack_installer.rs` — discovery, the roots (deltas 1 and 5).
- `native/src/emit/port_blockers.rs` — the roots, the prune set, test writers (deltas 1, 3 and 6).
- `native/src/emit/enter_stage.rs` — the roots, the prune set (deltas 1 and 3).
- `native/src/emit/enum_sets.rs` — the roots (delta 1).
- `native/src/emit/agents_md_smoke.rs` — the roots, the scratch config (deltas 1 and 13).
- `native/src/emit/demo.rs` — the roots (delta 1).
- `native/src/emit/drift_report.rs` — the roots (delta 1).
- `native/src/emit/install_hooks.rs` — the roots (delta 1).
- `native/src/emit/run_gate_tests.rs` — the roots, the root absolutization, fixtures (deltas 1 and 6).
- `native/src/emit/run_guard_tests.rs` — the roots (delta 1).
- `native/src/emit/run_index_tests.rs` — the roots (delta 1).
- `native/src/emit/upgrade_smoke.rs` — the roots, the root absolutization (delta 1).
- `native/src/emit/roadmap.rs` — bridge comment (delta 12).
- `native/src/emit/trajectory.rs` — bridge comment (delta 12).
- `native/src/emit/lesson_sink.rs` — test writers (delta 6).
- `native/src/install.rs` — `seam_text` (delta 9).
- `native/src/installer/init.rs` — the seam path (delta 9).
- `native/src/installer/doctor.rs` — the seam read (delta 9).
- `gate-sdk/lib/gate.sh` — delta 7's reduction, the accessors (deltas 3, 4, 5 and 7).
- `gate-sdk/lib/test-hermetic.sh` — pins and exports (delta 8).
- `gate-sdk/lib/consumer-smoke.sh` — the disposition (delta 10).
- `gate-sdk/bin/run-gates.sh` — the stub, the disposition (deltas 8 and 10).
- `gate-sdk/bin/run-consumer-smoke.sh` — the disposition (deltas 8 and 10).
- `gate-sdk/gate-tests/lib-gate.test.sh` — bridge cases (delta 13).
- `gate-sdk/gate-tests/knob-family-parity.test.sh` — deleted (delta 13).
- `gate-sdk/gate-tests/check-reads-couples.test.sh` — the bridge call (delta 13).
- `gate-sdk/gate-tests/run-dispatch-streams.test.sh` — the `--knobs` probe (delta 13).
- `gate-sdk/gate-tests/run-gate-tests.test.sh` — the `--knobs` probe (delta 13).
- `gate-sdk/gate-tests/check-graph-refs.test.sh` — config pin (delta 13).
- `gate-sdk/gate-tests/run-arm-contract.test.sh` — the scratch gate-sdk config its knob-union case
  writes (delta 13).
- `gate-sdk/gate-tests/check-graph-tree.test.sh` — vocabulary writer (deltas 4 and 13).
- `gate-sdk/gate-tests/check-install-disposition/good/args` — comment (delta 13).
- `gate-sdk/gate-tests/check-portability-floor/bad/args` — comment (delta 13).
- `gate-sdk/gate-tests/check-portability-floor/good/args` — comment (delta 13).
- `gate-sdk/gate-tests/check-readme-roster/good/args` — comment (delta 13).
- `gate-sdk/gate-tests/check-smoke-entry-guard/good/args` — comment (delta 13).
- `gate-sdk/gate-tests/check-template-registry-parity/good/args` — comment (delta 13).
- `context-kit/gate-tests/check-brevity.test.sh` — bridge comment (delta 12).
- `context-kit/gate-tests/check-surface-ratchet.test.sh` — bridge comment (delta 12).
- `guard-kit/gate-tests/scratch-run.test.sh` — bridge comment (delta 12).
- `queue-kit/gate-tests/roadmap.test.sh` — bridge comment (delta 12).
- `guard-kit/lib/guard.sh` — bridge mention in its header (delta 12).
- `evidence-kit/lib/evidence.sh` — deleted by SPEC-knob-files-cut-4.md before this lands (delta 12).
- `installer/consumer-smoke/run-smoke.sh` — the seam path (delta 9).
- `scripts/gate-sdk-config.sh` — moved to `.knobs` (delta 13).
- `scripts/graph-vocab.sh` — moved to `.knobs` (deltas 4 and 13).
- `scripts/context-config.knobs` — the prune-set name in a comment (delta 3).
- `scripts/core-files.list` — the two names (delta 13).
- `scripts/portability-patterns.list` — header comment (delta 13).
- `scripts/ci-build-artifact.sh` — the artifact-name comment (delta 5).
- `scripts/git-hooks/pre-commit` — regenerated (deltas 7 and 13).
- `scripts/git-hooks/commit-msg` — regenerated (deltas 7 and 13).
- `.github/workflows/gates.yml` — the Windows leg's seam (delta 9).
- `.workflow/release-declarations.md` — the three sections (delta 14).
- `.workflow/audit-roster.txt` — the bridge mention in a row (delta 12).
- `TASK-QUEUE.md` — the four paired entries' landing moves (delta 11).
- `docs/check-graph.html` — generated artifact, regenerated (deltas 4 and 13).
- `docs/gate-sdk/SPEC.md` — generated mirror, regenerated (all deltas).
- `docs/canon-kit/SPEC.md` — generated mirror, regenerated (deltas 1, 3 and 12).
- `docs/context-kit/SPEC.md` — generated mirror, regenerated (deltas 1 and 12).
- `docs/delegation-kit/SPEC.md` — generated mirror, regenerated (deltas 1 and 12).
- `docs/doctrine-kit/SPEC.md` — generated mirror, regenerated (delta 12).
- `docs/drift-kit/SPEC.md` — generated mirror, regenerated (deltas 4, 6 and 12).
- `docs/evidence-kit/SPEC.md` — generated mirror, regenerated (delta 12).
- `docs/guard-kit/SPEC.md` — generated mirror, regenerated (deltas 1, 6 and 12).
- `docs/lifecycle-kit/SPEC.md` — generated mirror, regenerated (deltas 1, 3 and 12).
- `docs/queue-kit/SPEC.md` — generated mirror, regenerated (delta 12).
- `docs/site-kit/SPEC.md` — generated mirror, regenerated (delta 12).
<!-- update-target-exempt: the survey record is boundary-truncated at the next scope, and its finding quotes the pre-cut file names as the measurement it was -->
- `.workflow/survey-record.md` — unchanged.
<!-- update-target-exempt: a dated release post is immutable history, and its mention of the knob it shipped stays -->
- `docs/posts/2026-07-17-checkwright-v0-2-0.md` — unchanged.
<!-- update-target-exempt: a dated release post is immutable history, and its mention of the helper it shipped stays -->
- `docs/posts/2026-08-01-checkwright-v0-21-0.md` — unchanged.
<!-- update-target-exempt: a dated release post is immutable history, and its mention of the bridge it shipped stays -->
- `docs/posts/2026-08-21-checkwright-v0-24-0.md` — unchanged.

## Retired spellings

- `config bridge` — the mechanism, removed (deltas 6, 7 and 12).
- `gate_knob_env` — deleted with `_set` and `_one` (delta 7).
- `_gate_knob_` — the bridge's private family, deleted (delta 7).
- `gate_static_knob` — the transitional helper, deleted (deltas 7 and 11).
- `GATE_SDK_RESOLVING_KNOB` — the resolving set, deleted (delta 7).
- `GATE_SDK_COUPLES_KNOB_SENTINEL` — the shell spelling of the sentinel, deleted (delta 7).
- `GATE_KIT_ROOTS_HERE` — retired, readers use `walk::kit_roots_abs` (delta 1).
- `GATE_KIT_ROOTS_REL` — the bridged name, derived by `walk::kit_roots_rel` (deltas 1 and 3).
- `GATE_SDK_ROOT_HERE` — retired, readers use `walk::sdk_root` (delta 1).
- `GATE_PRUNE_DIRS` — the resolved spelling of `GATE_SDK_PRUNE_DIRS` (delta 3).
- `GATE_EXEC_GLOBS` — the resolved spelling of `GATE_SDK_EXEC_GLOBS` (delta 3).
- `GATE_EXEC_PRUNE` — the resolved spelling of `GATE_SDK_EXEC_PRUNE` (delta 3).
- `GATE_LINT_EXTRA_DIRS` — the resolved spelling of `GATE_SDK_LINT_EXTRA_DIRS` (delta 3).
- `GATE_GRAPH_EXTERNAL_REFS` — the resolved spelling of `GATE_SDK_GRAPH_EXTERNAL_REFS` (delta 3).
- `GATE_MSG_PATTERN_FILES` — the resolved spelling of the two pattern-file knobs (delta 3).
- `GATE_PORTABILITY_PATTERNS` — the resolved spelling of `GATE_SDK_PORTABILITY_PATTERNS` (delta 3).
- `GATE_PORTABILITY_PATHS` — the resolved spelling of `GATE_SDK_PORTABILITY_PATHS` (delta 3).
- `GATE_SDK_NATIVE_ARTIFACT_NAMES` — replaced by discovery (delta 5).
- `gate-sdk-config.sh` — replaced by `gate-sdk-config.knobs` (deltas 9 and 13).
- `graph-vocab.sh` — replaced by `graph-vocab.knobs` (deltas 4 and 13).
- `GATE_SDK_CONFIG_FILE` — renamed `GATE_SDK_KNOB_FILE` (deltas 2 and 14).
- `bridge_declared_knobs` — the test helper, deleted (delta 6).
- `BRIDGED_ARMS` — renamed `ARMS` (delta 6).
- `Origin::Bridged` — deleted (delta 6).
- `gate_commit_types` — deleted, no caller (delta 7).
- `gate_kit_roots_rel` — deleted, no caller (delta 7).
- `--knobs` — the bridge's query arm, deleted (delta 6).

## Definition of Done

- [ ] **Causal completeness** — the locator, the root derivations, the empty-takes-default flag, the
      environment-only refusal, the vocabulary reader, the pre-binary accessors and the artifact discovery
      each have a unit test; the harness pin has its test.
- [ ] **Instruction surfaces: instruction only** — the refusal messages and the release declaration carry
      the remedy, not the grounds.
- [ ] **Merged with no information lost** — each surviving rule lands in its section; the deleted bridge
      prose goes because its subject does, and the refused alternatives stay in this file's history.
- [ ] **Amendment deleted**, and `config-seam-static-format` **moves to Done**, since no bridged kit is left
      and the corpus is finished; the three closed entries move to Done.
- [ ] **Removals propagated** — `check-amendment-retired-spelling` green.
- [ ] **Gaps filed** — one Deferred entry per port delta 10 makes owed, and any reader delta 12's or delta
      13's roster missed.
