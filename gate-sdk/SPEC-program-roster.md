# SPEC amendment: program-roster

Nothing machine-reads the set of programs the gate binary spawns. A registry
member's set is declared in its `gates::REGISTRY` row and held to behaviour by unit
test A. The rest of the binary has no such declaration: the non-gate arms, the
hooks, the installer's `doctor` and the shared modules they reach. gate-sdk/SPEC.md
§The non-gate arm records those sets in prose and closes on "making arm
requirements machine-readable is open work". context-kit/SPEC.md §bin/env-probe
states the consequence: `PROBE_SET`, `GATE_SDK_PROGRAM_FLOOR` and the spawned set
sit in no checked relation, so every floor census is bought again by grep. A
literal-only grep also misses every spawn whose program is a variable.

This amendment makes the spawned set a **type**. One module, `native/src/programs.rs`,
declares every program the crate may spawn as a constant of a type only that
module can construct. `proc`'s spawning faces accept only that type. Commands a
consumer names enter through one named constructor, `Program::consumer`, which
carries the knob or declaration that named them. The compiler then holds the
roster to be the binary's whole spawn set, variable-program sites included. A unit
test holds each roster member against `GATE_SDK_PROGRAM_FLOOR`, `PROBE_SET` or the
contributor audience.

**Direction, not ruling.** The operator directed this shape (option B), through
the lead, on 2026-09-18. The queue entry's candidate was a requirement element on
each `ARMS` row, held by the spawn recorder. That was set aside for two reasons,
both probed:

- **It could not be held.** Unit test A holds a registry member's `needs` by
  running the member over its `gate-tests/<name>/{good,bad}/` fixture cases with
  the recorder on (`native/src/gates/mod.rs:2378`). An `ARMS` row has no fixture
  corpus, and running the roughly 65 arms is not a unit test: `--pack-installer`
  runs `npm pack` and `--upgrade-smoke` builds the crate. An arm declaration
  nothing runs is the shape gate-sdk/SPEC.md refuses:
  - §The `# graph:` manifest, "**Two fields are refused rather than merely
    absent**" (lines 1984-1993): "A `# needs:` line declaring the member's
    external programs … earns the identical refusal on the identical ground" —
    "a self-declaration whose would-be reader could not verify what it read".
  - §check-reads-couples (lines 15443-15444): "an unrecorded prune declaration is
    refused, because without that reader the form is a self-certified narrowing,
    which is the unbound self-declaration this section exists to refuse".
- **It could not reach the whole binary.** Spawns also sit in `hook/`,
  `installer/` and shared modules that no `ARMS` row owns.

Per-arm attribution is dropped: the arms' sets stay prose in §The non-gate arm,
and the roster is their machine-held union. The intent oracle (scope) confirmed
that the entry's purpose is the machine-held floor census, and that dropping
per-arm attribution departs from no intent.

**Probed at authoring, 2026-09-18.** The census below comes from a read-only
survey of every call outside `proc.rs` to `proc`'s spawn, presence and resolution
faces, in shipped scope (a `#[cfg(test)]` item dropped, the rule `proc.rs`'s
`shipped_scope` applies).

- It found 171 string-literal spawn sites: `git` 122, `bash` 27, `date` 7,
  `mktemp` 6, `cp` 2, `jq` 2, and one each of `ps`, `curl`, `uname`, `tar` and
  `npm`.
- Three consts carry literal programs: `shellcheck` (`gates/action_run_shell.rs:9`,
  `gates/shellcheck.rs:8`), `cargo` and `rustc` (`gates/crate_arms.rs:9-10`).
- About 24 variable-program sites, each traced to its source in delta 3's table.
- `rustc` warns `dead_code` on an unused `pub const` in a binary crate. Checked
  with a two-const probe compiled by `rustc --edition 2021`, which delta 1 relies
  on.

The tree does not already do this: `grep -rn "struct Program\|mod programs"
native/src` returns nothing.

**Build order.** Build lands this amendment **after** the pid-liveness change, now
merged into gate-sdk/SPEC.md §Fail-closed contract. That change leaves `ps`
spawned only under `cfg(not(unix))`, so the roster's
`PS` member is `cfg(not(unix))` too (delta 1).

## What changes

**Batching.** Deltas 1, 2 and 3 compile only together, because the type, the
faces that take it and the call sites that pass it are one change. They land in
**one build batch and one commit**, although delta 3 is tagged mechanical. Build's
tiering may give delta 3's sweep a cheaper tier within that batch: a sub-dispatch
under the design-bearing session, which lands the commit. It may not split the
sweep into a batch of its own. Delta 4 depends on delta 1 and may follow in the
same batch or the next. Deltas 5 and 6 follow delta 4.

### (1) `native/src/programs.rs`, the program roster {design-bearing}

A new module declared in `main.rs`, holding:

- **`pub struct Program`**, with private fields, so no module outside `programs.rs`
  can construct one. The compiler holds that property, so no routing test is
  needed. A value has one of two identities:
  - a **roster member**: a `&'static str` name, plus an audience, which is
    `toolfloor`'s audience field value set, empty or `contributor`
    (context-kit/SPEC.md §bin/env-probe, *The audience axis*);
  - a **consumer command**: the command string, plus a `&'static str`
    **ground** naming what named it.

  Either identity may carry a resolved invocation path.
- **The roster**, written through one `macro_rules!` that emits, for each row,
  `pub const <ID>: Program`, and emits once `#[cfg(test)] pub const ALL: &[Program]`
  listing every row. So the slice is complete by construction. Staleness is held
  by the compiler: outside `cfg(test)`, a row no shipped code names is a
  `dead_code` finding, which `check-crate-arms`' deny-warnings clippy reds.
  The rows and each member's audience (causal-completeness point 6, per member):

  | const | name | audience | on |
  |---|---|---|---|
  | `GIT` | `git` | empty | floor, `PROBE_SET` |
  | `BASH` | `bash` | empty | floor, `PROBE_SET` |
  | `DATE` | `date` | empty | floor |
  | `MKTEMP` | `mktemp` | empty | floor |
  | `CP` | `cp` | empty | floor |
  | `AWK` | `awk` | empty | floor, `PROBE_SET` |
  | `SORT` | `sort` | empty | floor, `PROBE_SET` |
  | `JQ` | `jq` | empty | `PROBE_SET` |
  | `SHELLCHECK` | `shellcheck` | empty | `PROBE_SET` |
  | `PS` (`cfg(not(unix))`) | `ps` | empty | floor, via delta 5 |
  | `CURL` | `curl` | empty | `PROBE_SET`, via delta 5 |
  | `CARGO` | `cargo` | `contributor` | `PROBE_SET` (`::contributor`) |
  | `RUSTC` | `rustc` | `contributor` | — |
  | `TAR` | `tar` | `contributor` | — (the installer packer) |
  | `NPM` | `npm` | `contributor` | — (the installer packer) |
  | `UNAME` | `uname` | `contributor` | — (the env-probe arm, a contributor-side reader per context-kit/SPEC.md §bin/env-probe) |
  | `CHECKWRIGHT_GATES` | the crate's own binary name | empty | the payload itself |

  `AWK` and `SORT` are spawned only through the `PROBE_SET` walk (doctor,
  env-probe, `toolfloor::floor_met`). The walk names them through `by_name`,
  below, which keeps both rows live.
- **`Program::consumer(ground: &'static str, command: impl Into<String>) -> Program`**,
  the **one** constructor for a program named outside the crate's source. The
  ground is a knob name, or `programs::GATE_DECLARATION`, the ground for a gate
  dispatch argv's head, whether a `.sh` declaration or `gate_command`'s answer.
- **`Program::at(self, path: impl Into<String>) -> Program`**, which retargets a
  value to a resolved invocation path and keeps its identity. It is how
  `resolve_floor_tool`'s result and `current_exe()` / `GATE_SDK_NATIVE_BIN`
  become spawnable without minting a second identity.
- **`programs::by_name(name: &str) -> Option<Program>`**, the lookup a
  `PROBE_SET` walk uses to turn an element's name into its roster member. It
  returns an existing member and constructs nothing new. A `None` at a shipped
  caller is a fail-closed error, and delta 4's assertion C makes it unreachable.
- Accessors: `name()` returns the roster name, or the command's final path
  component under the recorder's existing `name_of` rule. `invocation()` returns
  the path to spawn. `ground()` returns `Option<&'static str>`.

### (2) `proc`'s faces take `&Program` {design-bearing}

`native/src/proc.rs`: every spawning face takes `program: &Program` in place of
`program: &str`, spawning `program.invocation()` through the existing
`spawn_target` funnel. The faces are `run`, `run_merged`, `run_merged_in`,
`run_bounded`, `run_bounded_capture`, `run_with_env`, `run_with_env_in`,
`run_with_stdin`, `piped`, `run_streamed`, `run_to`, `run_to_env`, `run_to_in`
and `run_stdout_in`.

- `dispatch` takes its argv head as a `Program` and the tail as today.
- The resolvers that feed a spawn also take and return the type:
  - `on_path(&Program) -> bool`;
  - `resolve_floor_tool(&Program) -> Program`, both `cfg` forms;
  - `resolve_interpreter(&Program) -> Result<Program, String>`.
- **`which` stays `&str`.** It is a presence probe whose callers include
  `env_probe`'s package-manager detection (`PM_CANDIDATES`), which is never
  spawned. A name probed and never run is not a requirement.
- **The recorder is unchanged in what it notes:** `name_of` of the invoked
  string. Unit test A's observed sets therefore do not move.
- **Error text** names `program.name()` and, for a consumer command, its ground
  (`cannot run <cmd> (named by <ground>): …`). This makes the ground field read
  at runtime as well as by delta 4's assertion D (point 4).

The routing test `no_module_outside_proc_constructs_a_subprocess_itself` stays as
it is.

### (3) The call-site sweep {mechanical}

Every caller of a delta-2 face passes a `Program`. It rides delta 2's batch and
commit (see *Batching*). Each site's value, by class:

- **Literal program → roster const.** Every string-literal site in the census
  above, for example `proc::run("git", …)` → `proc::run(&programs::GIT, …)`. The
  three literal consts are deleted for their roster members: `PROGRAM` in
  `action_run_shell.rs` and `shellcheck.rs` → `programs::SHELLCHECK`, and
  `CARGO`/`RUSTC` in `crate_arms.rs` → `programs::CARGO`/`programs::RUSTC`.
  `crate_arms::version_of` takes `&Program`. Its test's nonexistent program
  becomes `Program::consumer("test", "checkwright-no-such-program-exists")`
  inside `#[cfg(test)]`, which delta 4's assertion D does not scan.
- **Resolved roster member → `.at`, through the delta-2 resolvers:**
  - `toolfloor.rs:124-125` (`SORT`);
  - `installer/doctor.rs:31-36` and `emit/env_probe.rs:33-34,49`, through
    `by_name(<PROBE_SET element name>)`.
- **The payload → `programs::CHECKWRIGHT_GATES.at(<path>)`:**
  - `runner.rs:610`'s `self_exe` head;
  - `emit/enter_stage.rs:1522`'s `.gate` branch;
  - `emit/install_hooks.rs:146`;
  - `hook/stop_liveness.rs`' empty-knob default (`:196`);
  - `gates/gate_binary_fresh.rs:111` (`GATE_SDK_NATIVE_BIN`);
  - `emit/run_consumer_smoke.rs:361`'s `.gate` branch (`GATE_SDK_NATIVE_BIN`).
- **Consumer command → `Program::consumer(<ground>, …)`**, ground per site:

  | site | ground |
  |---|---|
  | `evidence.rs:124` | `"EVIDENCE_KIT_PARSER"` (the per-suite family resolves to it) |
  | `spec.rs:615` | the `knob` parameter's value. `command_lines` takes `knob: &'static str`, and its callers pass `CANON_KIT_ENUM_SETS_CMD`, `CANON_KIT_MEASURED_CLAIMS_CMD`, `CANON_KIT_INSTALL_TRANSPORTS_CMD`, `CANON_KIT_PAYLOAD_CLAIMS_CMD` and `CANON_KIT_CLAIM_CLASSES_CMD` |
  | `hook/verdict.rs:271` | `"DELEGATION_KIT_REFRESH_CMD"` |
  | `hook/stop_liveness.rs:209,232` (override set) | `"DELEGATION_KIT_LIVENESS_CMD"` |
  | `emit/always_loaded.rs:117` | `"CONTEXT_KIT_HOOK_CMD"` |
  | `emit/run_validate.rs:354` | `"EVIDENCE_KIT_PRE_HOOK"` at the `:152` caller, `"EVIDENCE_KIT_RUN_*"` at `:162`. `spawn` takes the ground as a parameter |
  | `emit/drift_report.rs:119` | `"DRIFT_KIT_KPIS_FILE"` (the plugin is a line of that file) |
  | `emit/enter_stage.rs:1548` | `"LIFECYCLE_KIT_ENTRY_PREFLIGHT"` |
  | `gates/docs_render_fidelity.rs:45` | `"SITE_KIT_RENDERER"` or `"SITE_KIT_RENDERER_BATCH"`, passed by `spawn_filter`'s callers |
  | `runner.rs:610` (`.sh` branch), `emit/enter_stage.rs:1522` (`.sh` branch), `emit/install_hooks.rs:157`, `emit/run_consumer_smoke.rs:361` (`.sh` branch), `emit/run_gate_tests.rs:330`, `emit/agents_md_smoke.rs:518` | `programs::GATE_DECLARATION` |

  The last row's `gate_command` sites (`run_gate_tests`, `agents_md_smoke`) take
  the ground whether the answer is a `.sh` path or the native bin, because the
  site cannot tell which. The parity test exempts consumer commands either way,
  so the label costs no verdict.

Run `bash gate-sdk/bin/build-native.sh`, then the battery.

### (4) The roster's unit tests {design-bearing}

In `programs.rs`'s tests, each a finding that names every offender before it
panics:

- **A, floor parity.** Every `ALL` member whose audience is empty is in one of:
  - `GATE_SDK_PROGRAM_FLOOR`'s **default**, read from the knob table's row
    (`native/src/knobs/gate_sdk.rs`), never from the environment, so a
    consumer's override cannot green it;
  - the name set of `toolfloor::PROBE_SET` elements whose audience is empty;
  - the payload name `env!("CARGO_PKG_NAME")`.

  A `contributor` member is admitted without either set. This is the entry's
  "declared contributor-side", held on the audience value set context-kit
  already owns rather than on a third list.
- **B, audience agreement.** Where a roster member is also a `PROBE_SET` element,
  their audiences are equal. So `CARGO` cannot be `contributor` on one surface
  and adopter-side on the other.
- **C, `PROBE_SET` ⊆ roster.** Every `PROBE_SET` element's name resolves through
  `by_name`. That makes the walk's lookup total and keeps a probed member from
  going unspawnable.
- **D, grounds are named.** Over the shipped scope of every module (the scanner
  and `shipped_scope` the routing test already uses), each `Program::consumer(`
  call's first argument is one of:
  - a string literal the crate's knob tables declare;
  - `programs::GATE_DECLARATION`;
  - a `&'static str` parameter. Where a site forwards the ground this way (`spec.rs`
    `command_lines`, `run_validate.rs` `spawn`, `docs_render_fidelity.rs`
    `spawn_filter`), the parameter's own callers are held to the literal form by
    the same scan.

  **Honest limit:** the scan reads call syntax, so a ground built at runtime
  would escape it. `&'static str` rules out a formatted string, and the rest is
  this assertion's floor.

### (5) Member dispositions the parity test forces {mechanical}

- **`ps` joins `GATE_SDK_PROGRAM_FLOOR`'s default** (`native/src/knobs/gate_sdk.rs:137`),
  alphabetical between `printf` and `pwd`. It is POSIX-mandated and present on
  busybox, macOS, every Linux and the MSYS userland of the one platform class that
  still spawns it (evidence-kit/SPEC.md §check-producer-liveness already rests the
  absent-`ps` cost on that). `port-blockers` then stops counting it as an
  off-floor requirement of `check-producer-liveness` on non-unix. That is the
  floor's own reading, and `ek_pid_alive`'s refusal on an absent `ps` stays,
  because floor membership never waived a refusal.
- **`curl` joins `PROBE_SET`, unconstrained and adopter-side.** This follows
  the `jq` and `shellcheck` precedent: programs only some kits spawn, which
  `doctor` still requires of every adopter. It was the operator's direction
  (2026-09-18, lead-relayed via AskUserQuestion; a direction, not a ruling) over
  the floor default, which is POSIX and coreutils only, and over a new audience
  value. `curl` is spawned by the usage poller (`hook/poll.rs:36,109`), an
  adopter-side hook that refuses by name when it is absent. Three things change:
  - `native/src/toolfloor.rs`'s `PROBE_SET` gains the bare element `curl`, after
    `jq`.
  - `docs/install.md` §Requirements' `toolchain` block gains a `curl` bullet,
    because `check-install-toolchain` holds that block and `PROBE_SET` equal
    element for element, in both directions. The bullet says the usage poller
    fetches its source with it, and that `init` refuses a machine without it,
    as the `shellcheck` bullet says.
  - The fixtures change only if they read the live roster. Its fixture pair
    passes its own `probe.sh` through `args`, so it does not.

  **The adopter-facing consequence is deliberate:** the installer's `doctor`,
  whose exit status is `init`'s last precondition, now refuses a host with no
  `curl` on `PATH`. So `init` refuses rather than half-installs, exactly as for a
  missing `jq` or `shellcheck` (installer/SPEC.md §doctor). The consumer smoke,
  install-smoke and doctor-running CI legs run on hosted runners that ship `curl`
  on Linux, macOS and Windows. The first pushed run after this lands confirms
  this rather than assuming it.
- `rustc`, `tar`, `npm` and `uname` take the `contributor` audience on the roster
  (delta 1) and change no other surface.

Regenerate whatever the knob-default edit stales. The knob-file projection and
the docs knob table print their own commands on red.

### (6) The SPEC passages the roster rewrites {design-bearing}

**Not yet applied**, each passage below:

- **gate-sdk/SPEC.md**, a new `### The program roster` after `### Fail-closed
  contract` and before `### Fixture-pair discipline`:

  > ### The program roster
  >
  > Every program the binary may spawn is a constant in `native/src/programs.rs`,
  > of a type no other module can construct, and `proc`'s spawning faces take
  > nothing else. So the roster *is* the binary's spawn set, held by the compiler
  > rather than by a census. A command a consumer names — a knob's argv, a gate's
  > `.sh` declaration — enters through `Program::consumer`, which carries the knob
  > or declaration that named it, and is the consumer's requirement rather than
  > the payload's. Each roster member carries `toolfloor`'s audience field.
  > Unit tests hold four relations: an adopter-side member is on
  > `GATE_SDK_PROGRAM_FLOOR`'s default, on `PROBE_SET`, or is the payload itself;
  > a member on both roster and `PROBE_SET` has one audience; every `PROBE_SET`
  > element is a roster member; and every consumer command's ground names a knob
  > or the gate-declaration ground. A row nothing spawns is dead code, which the
  > crate's deny-warnings lint reds. The roster records the **union**; which arm
  > or member spawns a program is recorded per member in `REGISTRY` for gates and
  > in prose for arms (§The non-gate arm). A per-arm declaration nothing runs is
  > the self-declaration §The `# graph:` manifest refuses. `which` stays a
  > name-typed presence probe, since a name probed and never run is no
  > requirement.

- **gate-sdk/SPEC.md §The non-gate arm**, the paragraph "**`--needs` answers
  about registry members only, and a non-gate arm is not one.**" (line 3037).
  Its sentence "An arm's spawned programs are therefore recorded in prose and
  nowhere a machine reads" becomes "An arm's own set is therefore recorded in
  prose; the union of every arm's set, and the rest of the binary's, is §The
  program roster". The closing lines 3145-3148 ("`grep -rn 'proc::'
  native/src/emit/` is the derivation; … making arm requirements machine-readable
  is open work.") become:

  > `native/src/programs.rs` is the machine-held union of these sets, and the
  > per-arm attribution above stays prose: no fixture corpus runs an arm, so a
  > per-arm declaration would be one nothing holds.

- **gate-sdk/SPEC.md §Fail-closed contract**, the spawn-recorder paragraph ("That
  routing test is also what makes `--needs` trustworthy …", line 1567) gains one
  clause after "notes the name of the program it is about to spawn": "— the
  `Program` it is handed, named as `name_of` names its invocation, so §The
  program roster changes nothing unit test A observes".
- **context-kit/SPEC.md §bin/env-probe**, "**The roster is what doctor verifies,
  not the set the binary spawns.**" (lines 401-409), is replaced by:

  > **The roster is what doctor verifies, not the set the binary spawns;
  > gate-sdk/SPEC.md §The program roster is that set, and a unit test holds the
  > two in relation.** A spawned program with an empty audience is on this
  > roster, on `GATE_SDK_PROGRAM_FLOOR`'s default, or is the payload itself;
  > `date`, `mktemp`, `cp` and `ps` rest on the floor's assumption that the host
  > carries them (gate-sdk/SPEC.md §lib/gate.sh). A contributor-side program —
  > `uname` (this arm), `tar` and `npm` (the installer packer), `rustc` —
  > carries the `contributor` audience on the program roster and need not be on
  > either. Every element of this roster is a program-roster member, so the walk
  > never probes a program the binary cannot name.

  `curl` (delta 5) is unconstrained, so the constrained-member list after that
  paragraph does not change. Its `PROBE_SET` line and its `docs/install.md`
  bullet are the whole of its record.

### (7) Regenerate the projections the SPEC edits stale {mechanical}

At least `docs/gate-sdk/SPEC.md` and `docs/context-kit/SPEC.md`, plus whatever the
delta-5 knob default and a `PROBE_SET` edit stale. Each freshness gate prints its
regen command on red (docs/site-architecture.md §Generated projections and their
freshness gates).

## Producers and consumers

- **`Program`, the roster and its constructors** (delta 1). The producers are
  `programs.rs` and, at runtime, `Program::consumer` and `Program::at` at the
  sites of delta 3's table. The consumers are `proc`'s faces (delta 2), which
  spawn `invocation()`; the error text, which reads `name()` and `ground()`; and
  delta 4's tests, which read `ALL`, the audiences and the grounds. Point 4: the
  name is read by the recorder and the tests, the audience by assertions A and
  B, the ground by assertion D and the error text, and the invocation by the
  spawn.
- **`by_name`** (delta 1). The producer is the roster. The consumers are the
  `PROBE_SET` walks in `doctor`, `env_probe` and `toolfloor`, and assertion C.
- **Roster-holding readers of the new names** (point 2):
  - `main.rs`'s module list (a `mod programs;` line);
  - the routing test's module scan, which skips only `proc.rs` and test-only
    modules, and which `programs.rs` passes because it builds no `Command`;
  - `check-comment-tier` over the new module's directives.
  - A new `###` heading is read by the mirror generator (delta 7) and by
    §-citation resolvers, which the delta-6 citations satisfy by naming the
    heading exactly.
- **The knob default** (delta 5). Its reader is `port-blockers`' floor filter
  (`emit/port_blockers.rs:226`), and the knob table's derived projections.
- **Point 5 (narrowing).** Delta 5 widens the floor default. `port-blockers`
  counts off-floor requirements, and a wider floor can only lower that count. No
  reader holds a minimum on it. Outside the docs, `git grep -ln
  GATE_SDK_PROGRAM_FLOOR` finds only the knob table and `port_blockers.rs` as code
  that reads it; the other three hits (`enum_sets.rs`, `upgrade_smoke.rs`,
  `proc.rs`) are comments. Delta 2's `which` exclusion
  narrows nothing, because `which` was never a spawn face.
- **Point 6.** Delta 1's table names each roster member's audience and set.
  Delta 3's table names each variable site's value. `curl`'s value is decided in
  delta 5.
- **`curl` on `PROBE_SET`** (delta 5). Its readers:
  - `doctor`, which now refuses a host without it;
  - the env-probe arm, which reports it;
  - `check-install-toolchain`, which needs the matching `docs/install.md` bullet;
  - assertions A and C, which it satisfies.

## Existing sections updated

- `gate-sdk/SPEC.md` §Fail-closed contract, the new §The program roster, and §The
  non-gate arm (delta 6).
- `context-kit/SPEC.md` §bin/env-probe (delta 6).
- `native/src/programs.rs` (new) and `native/src/main.rs` (delta 1).
- `native/src/proc.rs` (delta 2).
- `native/src/gates/action_run_shell.rs` — `PROGRAM` const deleted (delta 3).
- `native/src/gates/shellcheck.rs` — `PROGRAM` const deleted (delta 3).
- `native/src/gates/crate_arms.rs` — `CARGO`/`RUSTC` consts deleted (delta 3).
- `native/src/` — every other module in delta 3's census (delta 3).
- `native/src/knobs/gate_sdk.rs` (delta 5).
- `native/src/toolfloor.rs` — `PROBE_SET` gains `curl` (delta 5).
- `docs/install.md` — §Requirements' `toolchain` block gains the `curl` bullet
  (delta 5).
- `docs/gate-sdk/SPEC.md` and `docs/context-kit/SPEC.md` (delta 7).

## Retired spellings

- None — no delta retires a spelling a reader cites: the three deleted consts
  (`PROGRAM`, `CARGO`, `RUSTC`) are private, their names recur as unrelated
  identifiers across the crate, and no SPEC names them.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The
      causal-completeness check holds for each new state, event, interface and
      obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a
      template, agent definition or shim carries no grounds; a delta places them
      (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the
      canonical-spec text it refines rather than appending to it; the merged spec
      reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — `## Retired spellings` above is accurate, and
      `check-amendment-retired-spelling` is green.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [x] **`curl` disposed** — recorded in delta 5 at spec: `PROBE_SET`, adopter-side,
      on the operator's direction relayed by the lead on 2026-09-18.
- [ ] **The type, the faces and the sweep land in one commit** — deltas 1-3,
      with `bash gate-sdk/bin/build-native.sh` and the battery green at it; the
      tests land with delta 4 (the Enforcement-first rule).
- [ ] **The entry moves to Done before the drain stage** — at the batch that
      merges this amendment.
