# SPEC amendment: member-start

Two entries pair this amendment, because both reshape how `registry::invocation` starts a consumer member on native Windows. `custom-rust-gates` adds a second `# exec:` target class, a program the adopter builds, started directly. `windows-shell-gate-dispatch` starts a `.sh` declaration through bash there, since Windows starts no script by its path. Land delta 1 first: deltas 2 to 4 read the class it defines, and delta 5 edits the same function.

**The shape.** The classifier keeps its three answers and `Substrate::Exec(String)` stays one variant. The new class is a function of the exec value, asked by every class reader. A fourth variant is refused: `gate_output.rs`, `reads_couples.rs`, `gate_fixture_coverage.rs`, `gate_binary_fresh.rs`, `gate_substrate_parity.rs` and `port_blockers.rs` match the substrate non-exhaustively, so a new variant would fall silently into their shell or catch-all arm. Probe: `grep -rnE 'Substrate::(Exec|Shell|Compiled)|registry::substrate\(' native/src` over the crate.

## What changes

### (1) The executable target class {design-bearing} {user-facing: custom-rust-gates' Deliverable, an `# exec:` target class started directly}

An `# exec:` value whose final path component carries no `.` names an **executable**. The battery starts the file at the target path plus the host's executable suffix, which is `std::env::consts::EXE_SUFFIX`, the standard library's answer that §Layout and configuration already admits in the crate. It passes the member's own arguments and no host. So one tracked descriptor names one gate on every system: `# exec: bin/check-widget` starts `bin/check-widget` on Linux and macOS and the suffixed file on Windows. The class test lives in `native/src/substrate.rs` beside the PowerShell one and answers PowerShell, executable or no class. `registry::invocation`, assertion K (delta 2), §port-blockers and §check-gate-assertions (delta 4) all ask it.

- **The order in `invocation` changes.** The class is decided first, because the existence test reads the suffixed path for an executable. Today the test reads the raw target before any class is known (`native/src/registry.rs`, the `Exec` arm).
- **An executable that is not a file is refused at dispatch, exit 2, and so is one the host cannot execute.** Executability is `proc::is_executable`, the crate's one spelling. The message names the path and says the adopter's own build places it. A registered gate that cannot start is not clean.
- **A value ending in a separator is refused by `registry::exec_field`** as naming no file, beside the empty, rooted and climbing refusals. Its final component is empty and would otherwise read as extensionless.
- **A final component carrying any `.` is not an executable.** A dotfile is included, and so is `check.v2`. Each is PowerShell when it ends in `.ps1`, else of no admitted class, as today. That keeps the class test one character class, and a name the adopter controls costs nothing to choose.

The start needs no knob, so `GATE_SDK_POWERSHELL` stays the PowerShell class's alone. A member of either class still receives `GATE_SDK_NATIVE_BIN` (§The gate-authoring surface, the invocation bullet).

Replacement for §The gate-authoring surface, paragraph **The exec target classes**, through the paragraph naming `native/src/substrate.rs`. **Not yet applied.**

> **The exec target classes.** The field names a path relative to the descriptor's own directory, with no `..` segment, no root and no trailing separator. The class is read off the path's final component, and two are admitted:
>
> - **A PowerShell script**: the component ends in `.ps1`, matched without regard to ASCII case. *(The existing bullet's host, argument and policy text stands unchanged.)*
> - **An executable**: the component carries no `.`. It is started directly, with the member's own arguments, at the path plus the host's executable suffix (`std::env::consts::EXE_SUFFIX`), so one descriptor names one gate on every system. It needs no host and no knob. It is the class a gate written in Rust takes, or in any language that builds a program.
>
> `native/src/substrate.rs` owns both classes: the class test, the PowerShell argument list and host resolution, which a gate's dispatch, drift-kit's plugin start and guard-kit's scratch runner (guard-kit/SPEC.md §scratch-run) share, and the executable's suffixed path. A target of no admitted class is refused at dispatch, exit 2. So is a PowerShell target while `GATE_SDK_POWERSHELL` is empty, naming the knob and its two values, and an executable that is not an executable file, naming the build that places it. A registered gate that cannot start is not clean. A PowerShell host is resolved before the spawn through the crate's own program resolution, and an executable's path is already one, so the dispatch receives a path either way (§Fail-closed contract).

### (2) An executable target need not be tracked, and assertion K holds its class and placement {design-bearing}

An adopter's executable is a build output, so §check-gate-substrate-parity assertion K's tracked-file requirement binds the PowerShell class only. For an executable, K holds the placement, the field's shape and the class, and reads nothing about presence: whether the program is there to run is the dispatch's answer, exit 2 at run time. A tracked executable is admitted too, for an adopter who commits a program. K's existing unsuffixed `is_file()` read would test the wrong file on Windows, so it reads nothing for this class rather than the suffixed path: a K that reds an unbuilt gate would red every fresh clone before the adopter's build ran.

Replacement for §check-gate-substrate-parity, the **assertion K** bullet's sentence beginning `Its value names a tracked regular file` and its sentence beginning `K reds`, which the two sentences below take in order. The sentence between them, on a descriptor under a kit root, stands. **Not yet applied.**

> Its value names a file of an admitted target class, relative to that directory with no `..` segment, no root and no trailing separator (§The gate-authoring surface): a PowerShell target is a tracked regular file, and an executable is a build output K never looks for. K reds a misplaced field, a second field, a rooted, `..`-bearing, separator-ended or empty value, a target of no admitted class, and a PowerShell target untracked.

The clean line's segment `{exec_placed} exec descriptor(s) in the gates directory, each naming one tracked PowerShell target` (`native/src/gates/gate_substrate_parity.rs`) becomes `{exec_placed} exec descriptor(s) in the gates directory, each naming a target of an admitted class`, and the K finding `the one admitted target class is a PowerShell script (.ps1)` names both classes. Fixture: `good/` gains an executable-class descriptor whose target is absent, which K leaves alone. `bad/` gains a separator-ended value, and its `.txt` target stays the no-class case, since `.txt` carries a `.`. The crate unit test beside `an_exec_target_holds_only_once_tracked` gains an untracked executable that holds. The §check-gate-substrate-parity fixture paragraph names the new cases.

### (3) The battery never checks an adopter's executable against its source {design-bearing}

The entry leaves this to spec, and the ruling is no. The battery runs the file the path names. §check-gate-binary-fresh's predicate is unchanged and counts compiled members alone.

- **That gate works only because the kit knows the source.** Its stamp is git's identity for one known crate's tracked files, and the shipped binary carries the matching stamp because the build that bakes it is this tree's own. The kit knows no adopter's build: neither the source set, nor the toolchain, nor whether the program carries a stamp.
- **Two remedies are refused.** A `# build:` command the battery runs before the member would put the adopter's toolchain on every seat's commit path, and run build code on each commit that review never sees as a gate. A declared source glob compared with nothing the program carries is a self-declaration nothing verifies, the shape §The `# graph:` manifest refuses.

**Honest limit:** an adopter who edits a gate's source and does not rebuild runs the old program, as any tool on `PATH` would. The install page names the rebuild as the adopter's step (delta 7).

Replacement for §check-gate-binary-fresh, the sentence `An exec member's spawn runs its host, never a subcommand, so it dispatches to no binary.` **Not yet applied.**

> An exec member's spawn runs its PowerShell host or its own program, never a subcommand, so it dispatches to no binary, and an adopter's program is checked against no source (§The gate-authoring surface).

### (4) The other class readers {mechanical}

Each site that reads the exec class, from the probe `grep -rnE 'Substrate::Exec|exec_target|is_powershell_body|EXEC_NEEDS' native/src` over the crate:

- **§port-blockers** reports an exec member's requirement as `(exec host $GATE_SDK_POWERSHELL)` (`native/src/emit/port_blockers.rs`, `EXEC_NEEDS`). An executable-class member requires nothing the kit can name, so it reports no row and does not count as undecided. Today it would report a PowerShell host it does not use.
- **§check-gate-assertions** reads an exec member's markers from its target (`native/src/gates/gate_assertions.rs`). An executable is a build output whose markers live in source the descriptor does not name. So the member is counted on the clean line as **out of reach**, the arm a compiled member takes in a tree with no crate. Reading the built program would red "zero markers" on every executable gate whose contract a consumer SPEC enumerates.
- **Unchanged, and why:**
  - `check-psscriptanalyzer` filters on the PowerShell test.
  - `check-gate-fixture-coverage` and `check-gate-output` read any exec member alike, and both rightly refuse it a `# no-fixture:`.
  - `check-gate-tamper` adds the unsuffixed target to its gate-file set, and an untracked build output never reaches a diff.
  - `check-reads-couples` counts any exec member undecidable.
  - The fixture runner absolutizes and guards the argv's first element, which is the suffixed path.

Replacement for §The gate-authoring surface, **The answering interface stays per substrate**, the sentence beginning `Its reads are undecidable`. **Not yet applied.**

> Its reads are undecidable and counted as such (§check-reads-couples). A PowerShell target requires the host its class names, and an executable requires nothing the kit can name (§port-blockers).

Replacement for §port-blockers, the evidence-column clause beginning `or the declaration path and (exec host`. **Not yet applied.**

> or the declaration path and `(exec host $GATE_SDK_POWERSHELL)` for a PowerShell exec member. An executable exec member reports no row.

Replacement for §check-gate-assertions, the sentence beginning `An exec member's markers are read from its target`, through its closing `(§The gate-authoring surface).`. **Not yet applied.**

> A PowerShell exec member's markers are read from its target, their locality-class home, as a compiled member's are read from its module. An executable's live in source its descriptor does not name, so it is counted out of reach, as a compiled member is in a tree with no crate. The classifier tells the classes apart (§The gate-authoring surface).

### (5) A shell declaration on native Windows starts through bash {design-bearing} {user-facing: windows-shell-gate-dispatch's Deliverable, its first branch, the dispatch naming bash so docs/requirements.md §Writing your own shell gates and docs/install.md's Windows paragraph hold}

`registry::invocation`'s shell arm hands `proc::dispatch` the declaration path as argv[0] with no interpreter, and `proc::spawn_resolution` passes a path through untouched. A POSIX host starts it by its shebang and execute bit. Windows starts only an executable image, so the member fails as a dispatch harness error, though both pages say Git for Windows' bash runs it.

**Inferred, cannot run before build:** that native Windows refuses to start a `.sh` member today — no Windows leg dispatches one and this host is not Windows, so delta 6's Windows case, landed by build, is the first run of it.

- **On Windows the shell arm starts `bash` with the declaration as its first argument, and the member's own arguments after.** On every other host the arm is unchanged.
- **bash is resolved before the spawn, through the crate's program resolution with its system-directory rejection.** That is the `SYSTEM_DIR_HOMONYMS` rule §check-graph owns, so `System32`'s `bash.exe`, the WSL launcher, is never started. It is resolved there and not left to the spawn, because `proc::dispatch` spawns its program raw (`native/src/proc.rs`, the dispatch face). It is also resolved instead of `proc::resolve_floor_tool`, which falls back to the bare name, which Windows would search for in `System32` first.
- **A bash that resolves nowhere outside the system directory is a dispatch refusal, exit 2.** The message names Git for Windows' bash and docs/requirements.md §Writing your own shell gates.
- **The fixture runner takes the same start.** `gate_command` (§lib/gate.sh) answers a shell member with its path, and `--run-gate-tests` spawns that argv natively (`native/src/emit/run_gate_tests.rs`). So a one-element argv whose element ends in `.sh` is started through the shell arm's function, and a case and the battery start a shell member identically. The test is the `.sh` extension, `gate_resolve`'s one spelling of a shell declaration, and not `registry::substrate`, which answers shell for every path that does not end in `.gate`. An executable's argv (delta 1) is also one element, the suffixed path. Its value carries no `.`, so the path ends in the host's suffix at most and never in `.sh`, and it is spawned as it is.
- **Refused: having `gate_command` ask `--emit gate-argv` for a `.sh` declaration.** That would put the binary on the path of every shell caller of `gate_command`, a tree vendoring the library with no binary included. Its in-bash callers (`gate_run` in `lib/test-hermetic.sh`) already run a `.sh` by its path under Git for Windows' bash.

The shell arm's one function is called from both sites. Its unit test in `native/src/emit/gate_argv.rs`, which pins the shell argv as the declaration alone, splits by `cfg(windows)`, since `crate-tests-windows` runs it.

Replacement for §The gate-authoring surface, **One function starts every member**, the sentence beginning `Shell takes the declaration itself`. **Not yet applied.**

> Shell takes the declaration itself, or on native Windows `bash` with the declaration as its first argument, bash resolved outside the system directory before the spawn. Compiled takes the binary and the name once the binary carries a subcommand of that name. A PowerShell exec member takes its host with the class's arguments once the target is a file, and an executable takes its suffixed path once that is an executable file.

Replacement for §run-gate-tests, **The case runs whatever the member dispatches to**, the sentence beginning `The invocation resolves through`. **Not yet applied.**

> The invocation resolves through `gate_command` (§lib/gate.sh), so a case runs `<binary> <name>` for a compiled member, the host and its target or the suffixed program for an exec member, and the script for a shell one, which on native Windows the runner starts through the shell arm of `registry::invocation`.

### (6) The live dispatch cases {design-bearing}

`native/tests/exec_dispatch.rs`, whose scratch consumer runs `--run`, gains two members. Each Windows triple's `crate-tests-windows` leg runs `cargo test`, so both bind there with no workflow step.

- **An executable pair.** The test builds a passing and a failing program from source with the `rustc` its own toolchain carries, places each extensionless with the host's suffix, and registers both. It asserts `PASS`, `FAIL (exit 1)`, the invariant line, and the member's view of `GATE_SDK_NATIVE_BIN`, as the PowerShell case does. A third case leaves the target unbuilt and asserts the dispatch refusal at exit 2.
- **A shell pair.** It registers a passing and a failing `.sh` member and asserts the same lines under `--run`, then runs `--run-gate-tests` over a scratch pair dir for the passing member. On a Windows triple this is the native-Windows leg the entry owes, run under the runner image's Git for Windows bash. On every other host it holds the unchanged POSIX start.

`gate-sdk/gate-tests/lib-gate.test.sh` gains the executable argv case beside its PowerShell one: `gate_command` answers the suffixed path alone.

Replacement for §The gate-authoring surface, **The live dispatch is a crate case**. **Not yet applied.**

> **The live dispatch is a crate case.** An integration test builds a scratch consumer whose `gates.list` registers a passing and a failing member of each start this section admits, and runs `--run` over it.
>
> - **PowerShell** runs under each host that resolves. A Windows build runs it under `powershell` unconditionally, since every Windows host carries it, and under `pwsh` where it resolves. A seat with neither runs only the refusal case, and a local run under `--with-foreign-shells` reaches the `pwsh` arm.
> - **Executable** members are built from source by the test's own `rustc`, and an unbuilt one is the refusal case.
> - **Shell** members run under `--run` and under `--run-gate-tests`.
>
> It asserts `PASS`, `FAIL (exit 1)`, the invariant line and the member's view of `GATE_SDK_NATIVE_BIN`. The Windows legs run `cargo test`, so they bind every case with no workflow step, and the shell case there is the one run of a `.sh` member on native Windows.

### (7) The install page's prerequisites for a Rust gate {design-bearing} {user-facing: custom-rust-gates' Deliverable, the install page's prerequisites for a Rust gate}

Replacement for docs/requirements.md §Writing your own Rust gates, the whole body. **Not yet applied.**

> A Rust gate is a program you build. Its `.gate` descriptor in your gates directory carries the gate's `# graph:` line, its `# spec:` line, and `# exec: <path>`, naming the built program relative to that directory with no extension. The battery adds `.exe` on Windows, so one descriptor serves every system. It needs a Rust toolchain on every machine that runs the gate. The battery runs the program at that path and never builds it, so build it before the battery runs and rebuild it after each edit: the battery does not compare it with its source. A gate not yet built fails as a gate that could not run. It prints the lines your shell gates print, and reads a knob through `--emit knob-values <NAME>` on the binary its environment names. The same descriptor starts a program built in any language. [gate-sdk/SPEC.md](gate-sdk/SPEC.md#the-gate-authoring-surface) owns this.

The page is held by the docs gates, so the build runs them on the page. It names `.exe` because it is not a kit surface, whereas §lib/gate.sh's `gate_exe_suffix` rule bars any kit surface from spelling the suffix. The SPEC text above therefore says "the host's executable suffix".

## Producers and consumers

- **The executable class (delta 1).**
  - Producer: an adopter's descriptor carrying `# exec:` with an extensionless value, under the consumer gates directory, the one placement K admits.
  - Enabling config: none. Every consumer that writes such a descriptor reaches it, and no knob gates it.
  - Consumers of the class answer: `registry::invocation` (the start), assertion K (delta 2), §port-blockers' requirement row and §check-gate-assertions' out-of-reach count (delta 4).
  - Consumers of `invocation`'s answer are its existing callers, unchanged: the battery and git-hook arm, the stage-entry pre-flight, the install-hooks identity rung, the consumer smoke's probe and `--emit gate-argv`. Through `gate-argv`, `gate_command` and the fixture runner are consumers too.
  - Roster-holding readers: no knob, tag or file convention is minted. The class needs no roster entry. The program roster (`native/src/programs.rs`) is not reached, because an adopter's program is a `Program::consumer` under the existing `GATE_DECLARATION` ground. Probe: `grep -n GATE_DECLARATION native/src/programs.rs native/src/registry.rs`.
- **The dispatch refusal for an unbuilt executable (delta 1).**
  - Producer: `invocation`.
  - Consumers: the battery's `FAIL … (dispatch harness error, exit 2)` line and `gate-argv`'s exit 2, which `gate_command` and the fixture runner's `HARNESS:` line read. That path already carries the PowerShell host-off refusal.
- **The bash start on Windows (delta 5).**
  - Producer: `invocation`'s shell arm under `cfg(windows)`, and the fixture runner's one-element shell argv.
  - Enabling config: none. Every native-Windows battery registering a `.sh` member reaches it.
  - Consumer: `proc::dispatch` and the runner's spawn, each receiving a resolved bash path.
  - The program roster already carries `BASH`, whose audience is `toolfloor::DERIVED`. A new spawn site of an existing member mints no row.
  - Its refusal's consumer is the same dispatch-harness-error line.
- **No new field.** The class is read off the existing `# exec:` value, so point 4 adds no reader.
- **Point 5: no corpus narrows.** K's tracked requirement narrows by class (delta 2), and K's red condition stays named for each arm. A PowerShell target untracked still reds. An executable target never reds on presence. K holds no count or floor its narrowing could flip: its clean segment counts descriptors held in place, and an extensionless target still counts.
- **Point 6: no corpus-wide obligation is minted.**

## Existing sections updated

Rosters here came from the probes the deltas name, plus `git grep -n -F -e 'one admitted target class' -e 'tracked PowerShell target' -e 'There is no supported path today' -e 'One class is admitted' -e 'cannot author a compiled gate' -e 'never a compiled one'` and `git grep -n -E 'compiled gate|exec descriptor|exec member|# exec:'` over the tracked tree.

- `gate-sdk/SPEC.md` §The gate-authoring surface: the exec target classes (delta 1), the answering-interface sentence (delta 4), the one-start function (delta 5), the live dispatch case (delta 6).
- `gate-sdk/SPEC.md` §The port-candidate criteria, the paragraph opening **The default's domain is a tree that carries the crate**. It says a consumer's substrate is shell or PowerShell, "never a compiled one", and that an adopter has no supported path to a compiled gate. Both become: an adopter writes a compiled gate as an executable exec target (§The gate-authoring surface), and the fork route is no longer the only one (delta 1).
- `gate-sdk/SPEC.md` §Meta-gate conservation for the binary substrate, the `check-shellcheck` row's clause "an adopter cannot author a compiled gate" and the consequence drawn from it, that a vendoring consumer's gate family is shell or an exec member's PowerShell. They change together: an adopter's compiled gate is an executable exec target, which no shell lint reads, and the family is shell, a PowerShell exec member or an executable (delta 1).
- `gate-sdk/SPEC.md` §check-shellcheck, the paragraph **The gate ships to adopters for their own shell**. It says an adopter **cannot author a compiled gate**, and that a vendoring consumer's gate family is shell, or PowerShell behind an exec descriptor. It becomes the same pair as the row above (delta 1).
- `gate-sdk/SPEC.md` §The `# graph:` manifest, the paragraph **Its existence is the dispatch declaration**, whose last clause says assertion K holds the exec path to a tracked file. K holds a PowerShell target tracked, and an executable's placement and class (delta 2).
- `gate-sdk/SPEC.md` §The gate-authoring surface, the sentence beginning `It refuses two` that lists the field's refusals. It gains the separator-ended value (delta 1).
- `gate-sdk/SPEC.md` §lib/gate.sh, the `gate_command` bullet. An exec member's answer is the host, its arguments and the target for a PowerShell class, and the suffixed path alone for an executable (delta 1).
- `gate-sdk/SPEC.md` §port-blockers, the paragraph **A compiled member is read from its substrate**, its sentence saying an exec member's one requirement row is `?<TAB>GATE_SDK_POWERSHELL`. That holds for a PowerShell exec member, and an executable reports no row (delta 4).
- `gate-sdk/SPEC.md` §check-gate-substrate-parity: assertion K and its fixture paragraph (delta 2).
- `gate-sdk/SPEC.md` §check-gate-binary-fresh, the exec-member sentence (delta 3).
- `gate-sdk/SPEC.md` §port-blockers, the default arm's evidence column (delta 4).
- `gate-sdk/SPEC.md` §check-gate-assertions, the exec-member marker sentence (delta 4).
- `gate-sdk/SPEC.md` §run-gate-tests, the case-runs paragraph (delta 5).
- `docs/gate-sdk/SPEC.md`, the generated mirror, regenerated by the command its freshness gate prints (deltas 1, 2, 3, 4, 5 and 6).
- `docs/requirements.md` §Writing your own Rust gates (delta 7).
- `gate-sdk/README.md`, the paragraph naming the PowerShell descriptor as the non-shell route. It gains one clause, that a descriptor may name a program the adopter builds instead, plus its generated mirror `docs/gate-sdk/README.md` (delta 1).
- `native/src/substrate.rs`: the class test and the suffixed path (delta 1).
- `native/src/registry.rs`: `exec_field`'s separator refusal and `invocation`'s order and executable arm (delta 1). `invocation`'s shell arm (delta 5).
- `native/src/gates/gate_substrate_parity.rs`: K's class and tracked halves, its finding text, its clean segment and its unit test (delta 2).
- `gate-sdk/gate-tests/check-gate-substrate-parity/good/expect.txt` and both case trees (delta 2).
- `native/src/emit/port_blockers.rs` (delta 4).
- `native/src/gates/gate_assertions.rs` (delta 4).
- `native/src/emit/run_gate_tests.rs`: the shell start of a one-element argv (delta 5).
- `native/src/emit/gate_argv.rs`: unit-test cases for the executable argv (delta 1), and the `cfg(windows)` split of the shell argv (delta 5).
- `native/tests/exec_dispatch.rs` (delta 6).
- `gate-sdk/gate-tests/lib-gate.test.sh` (delta 6).
- `.workflow/release-declarations.md`, the next release's declaration of the new class and the Windows shell start, which §check-release-change-declared reads (deltas 1, 5 and 7).

## Retired spellings

- `one admitted target class` — K's no-class finding names both classes; survivors are `native/src/gates/gate_substrate_parity.rs`, rewritten (delta 1).
- `tracked PowerShell target` — K's clean segment; survivors are `native/src/gates/gate_substrate_parity.rs` and `gate-sdk/gate-tests/check-gate-substrate-parity/good/expect.txt`, both rewritten (delta 2).
- `There is no supported path today` — the Rust-gate page's withdrawn claim; survivor `docs/requirements.md`, rewritten (delta 7).
- `One class is admitted` — the exec target classes paragraph's count; survivors are `gate-sdk/SPEC.md` and its generated mirror `docs/gate-sdk/SPEC.md`, rewritten (delta 1).
- `cannot author a compiled gate` — the two sites saying an adopter has no compiled-gate route; survivors are `gate-sdk/SPEC.md` and its generated mirror `docs/gate-sdk/SPEC.md`, rewritten (delta 1).

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it.
- [ ] **The Windows legs read green** — the push the two entries share runs `crate-tests-windows` on each Windows triple, and delta 6's shell and executable cases pass there. Both entries move to Done at that remote verdict, by the build stage's remote-oracle rule, before the drain stage.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks.
