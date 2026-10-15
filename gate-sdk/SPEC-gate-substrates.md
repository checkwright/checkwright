# SPEC amendment: gate-substrates

Two entries pair this amendment. `gate-authoring-sdk-surface` asks what a gate *is* independent of the language implementing it, and whether a meta-gate reads a substrate-neutral declaration or learns each substrate. `custom-gate-substrates` asks for a gate an adopter writes in PowerShell, so a native-Windows adopter need not write bash. The second is the first one's first instance, so they are designed together: the statement that the substrate is a parameter, and the one dispatch shape that makes PowerShell its second value.

**The shape: a descriptor naming its implementation.** A `.gate` descriptor today dispatches to a subcommand of the published binary, which an install cannot extend. This amendment adds one descriptor field, `# exec: <path>`, naming the file that implements the gate. Its first target class is a PowerShell script, started by a host the consumer names. The descriptor stays the declaration every reader already reads: the graph, the hook selection, the install and projection fields, the `# spec:` pointer. So no declaration reader learns a new spelling, and the implementation keeps only its locality-class lines (§The `# graph:` manifest's partition), as a compiled gate's Rust module does.

**Refused: a third declaration spelling, `<name>.ps1`, carrying its own manifest.** It is the shape the entry named first, and it reaches every reader that globs or resolves a declaration. By `git grep` over the crate and the kit trees that is 26 `registry::resolve` callers, seven private copies of the `.sh`-then-`.gate` resolution, and some twenty corpus globs. Several of those would read a `.ps1` as shell, scanning it with the shell command-word scanner or the shell walk parser. A field reaches the same readers through the spelling they already resolve. The tamper entry gets its answer from the same choice: a meta-gate reads the declaration and never learns a substrate's syntax (delta 1).

**What stays out.** Starting an adopter-built executable directly, the Rust path, is split to `custom-rust-gates`: it widens the target class this amendment defines, with naming and freshness questions of its own. A self-declaring `.sh` gate is unchanged and stays first-class (§The extensibility model).

## What changes

### (1) The gate-authoring surface {design-bearing} {user-facing: gate-authoring-sdk-surface's roadmap summary, "Author a gate in any language behind one substrate-neutral descriptor", on the operator's horizon steer of 2026-08-02 recorded on the entry}

A new section of gate-sdk/SPEC.md, placed after §The extensibility model. It replaces §What the dispatch seam does not settle's paragraph **Visible and not built.**, which becomes one sentence: *The gate-authoring surface is ruled at §The gate-authoring surface.* **Not yet applied.**

> ## The gate-authoring surface
>
> **A gate is its declaration, its invocation, its output and its fixture pair, and none of the four names a language.** A third party authoring a gate on any substrate meets exactly these:
>
> - **The declaration** is a text file every reader greps with no build and no execution: a `.sh` script's header block, or a `.gate` descriptor (§The `# graph:` manifest). Its manifest-class fields are the same in either.
> - **The invocation** is the same for every substrate. A member runs with the work-tree toplevel as its working directory, the arguments its `gates.list` line gives (a `mode=staged` member also receives its staged paths), stdin empty, stdout and stderr captured together, and `TMPDIR` set to a scratch directory of its own. It inherits the environment. A member that is not compiled into the binary also receives `GATE_SDK_NATIVE_BIN` set to the running binary, absolute, so a member in any language reads a knob value through `--emit knob-values <NAME>`. That includes a `GATE_LOCAL_` knob its own descriptor declares (§The knob file), without a library to source.
> - **The output** is §Output contract's, exit codes included, asserted at run time by the fixture runner on every substrate.
> - **The fixture pair** is §Fixture-pair discipline's, run through the same dispatch the battery uses.
>
> **The substrate is a parameter, and it has one reader.** `registry::Substrate` classifies a resolved declaration as **shell** (a `.sh` declaration, its own implementation), **compiled** (a `.gate` descriptor with no `# exec:` field, dispatching to the binary's subcommand of its name), or **exec** (a `.gate` descriptor carrying `# exec:`, dispatching to the file the field names). Every site asking what implements a member asks the classifier. A file's extension answers only what kind of file it is: whether a `.gate` dispatches to the binary is the classifier's answer, never `ends_with(".gate")`.
>
> **Meta-gates read the declaration and never learn a substrate.** A fact a meta-gate reads without a build, such as a coupling, a disposition or an exemption list, is manifest-class and lives in the declaration. A meta-gate that parsed each substrate's syntax would owe a reader per language, and the first language it lacked would be a gate it could not hold. The two contracts that cannot be read off a declaration stay per substrate, each with its stated reach:
>
> - **Fail-closed.** §check-gate-fail-closed reads shell statically. The compiled substrate closes the defect by construction (§Fail-closed contract). An exec member's is its author's, held only where its fixture pair's exit codes reach.
> - **Self-lint.** §check-shellcheck lints shell, `cargo clippy` the crate, and §check-psscriptanalyzer PowerShell.
>
> **The answering interface stays per substrate.** `--reads` and `--needs` answer for the compiled substrate by flag and for shell by parse (§The `# graph:` manifest). An exec member has neither. Its reads are undecidable and counted as such (§check-reads-couples), and its one requirement is the host its target class names (§port-blockers). Requiring a foreign substrate to answer `--reads` by argv convention is refused: nothing outside the crate's unit tests could verify the answer, so it would be the self-declaration §The `# graph:` manifest refuses.
>
> **The exec target classes.** The field names a path relative to the descriptor's own directory, with no `..` segment and no root, to a tracked regular file. The class is read off the target's extension. One class is admitted:
>
> - **A PowerShell script**, `.ps1` matched without regard to ASCII case. It is started by the host `GATE_SDK_POWERSHELL` names, with the arguments `-NoLogo -NoProfile -NonInteractive -ExecutionPolicy Bypass -File <target>` followed by the member's own. `-NoLogo` keeps a banner out of the output the contract reads, `-NoProfile` keeps unreviewed profile code out of the verdict, `-NonInteractive` turns a prompt into a failure instead of a hang, and `-ExecutionPolicy Bypass` sets the policy of the host's own process, so a machine policy refusing script files does not refuse the gate. A non-Windows `pwsh` accepts that flag inertly. **Honest limit:** a policy Group Policy sets outranks the process scope, so a host under one can still refuse the script, and the member's exit is then the host's.
>
> A target of any other class is refused at dispatch, exit 2. An empty `GATE_SDK_POWERSHELL` refuses a PowerShell target at dispatch, exit 2, naming the knob and its two values, since a registered gate that cannot start is not clean. The host is resolved before the spawn through the crate's own program resolution, so the dispatch receives a path (§Fail-closed contract).
>
> **Where an exec descriptor may sit.** Under the consumer's gates directory only, because a kit's gates reach a consumer compiled and digest-verified (§Consumer payload), and an implementation file inside a kit root is what §check-gate-substrate-parity assertion E refuses.
>
> **Which host, if any, is the consumer's selection, with off among its values** (doctrine-kit/DOCTRINE.md, Policy-as-choice). `GATE_SDK_POWERSHELL` is empty, `pwsh` or `powershell`, by name or as a path whose final component is one of the two. Windows PowerShell 5.1 is on every Windows host and `pwsh` is the cross-platform one, so a team spanning both sets the tracked value its members share and a seat overrides it in its local overlay. The same knob starts a drift-kit KPI plugin written in PowerShell (drift-kit/SPEC.md §The extensibility contract), so one value arms PowerShell for the consumer.

Replacement for §The extensibility model's opening line and its first paragraph. **Not yet applied.**

> How a consumer extends the battery: four shapes, two kept and two refused, each with its ground.
>
> **The shell escape hatch stays first-class.** A consumer authors their own gates as shell in their own resolve dir. Resolution stays consumer-first and `.sh` beats `.gate` within a dir (§lib/gate.sh), so a consumer shadowing a ported kit gate with their own script still wins. Withholding a gate's predicate does not narrow this. It is a choice made on the consumer's own machine about the consumer's own code, and it puts no interpreter into the shipped payload's dependency floor (§The adopter constraints). Forbidding it would strand every consumer-authored gate and buy nothing.
>
> **A descriptor naming its implementation is the second kept shape.** A consumer's `.gate` descriptor carrying `# exec:` dispatches to a file in a target class §The gate-authoring surface admits, so a gate is written in a language other than shell behind the declaration every reader already reads.

### (2) The substrate classifier and the shared invocation {design-bearing} {user-facing: custom-gate-substrates' Deliverable — the knob an adopter sets to arm PowerShell}

- **`registry::Substrate` and `registry::substrate(decl)`** in `native/src/registry.rs`. The function reads a `.gate` descriptor's `# exec:` lines and classifies the declaration. It refuses two `# exec:` lines in one descriptor, an empty value, a rooted value and a value with a `..` segment, naming the descriptor. A refusal reaches every caller as an exit 2 through the member or arm asking.
- **`registry::invocation(name, decl, self_exe)`** returns the program and leading arguments for a resolved member, or a refusal. Shell returns the declaration as the program. Compiled returns the binary and the name, after the `gates::declared` check the call sites hold today. Exec checks that the target is a file and returns the host and the target class's arguments. **Its four callers drop their own `ends_with(".gate")` branch:** `runner.rs`' `dispatch_one`, `emit/enter_stage.rs`' pre-flight dispatch, `emit/install_hooks.rs`' identity-shadow dispatch, and `emit/run_consumer_smoke.rs`' unregistered-member probe. The battery, the hook arm (which dispatches through `dispatch_one`), the stage-entry pre-flight and the identity check therefore start an exec member identically.
- **The exec child's environment.** `dispatch_one` adds `GATE_SDK_NATIVE_BIN=<self_exe>` for a shell or exec member's spawn. A compiled member is the binary itself and needs none.
- **`native/src/substrate.rs`**, a new module, owns the PowerShell target class. It holds the extension test, the host argument list and the host resolution, and `registry::invocation` and drift-kit's plugin start (drift-kit/SPEC-kpi-powershell.md) both call it. `emit/scratch_run.rs` takes the argument list and the extension test from it, and keeps its own knob and refusal order (delta 11).
- **The knob.** `GATE_SDK_POWERSHELL`, a scalar row in gate-sdk's table, default empty, validated empty or with a final component of `pwsh` or `powershell`. The validator is the one `GUARD_KIT_SCRATCH_POWERSHELL` uses today, moved to `substrate.rs`, so the two knobs accept the same set.

Replacement for §Layout and configuration's knob list: one row added beside `GATE_SDK_LINT_EXTRA_DIRS`. **Not yet applied.**

> - `GATE_SDK_POWERSHELL` — the PowerShell host that starts a gate's `# exec:` target and a PowerShell KPI plugin: empty (off, the default), `pwsh` or `powershell`, by name or path (§The gate-authoring surface).
> - `GATE_SDK_PSSCRIPTANALYZER_EXCLUDE` — a whitespace list of PSScriptAnalyzer rule names `check-psscriptanalyzer` excludes, default empty (§check-psscriptanalyzer).

### (3) The descriptor field roster gains `# exec:` {design-bearing} {user-facing: custom-gate-substrates' Deliverable, on the operator direction of 2026-09-27 that a customer on Windows can write a gate in PowerShell}

Replacement for §The `# graph:` manifest's paragraphs **The `.gate` descriptor.** through the field roster's closing sentence. **Not yet applied.**

> **The `.gate` descriptor.** A gate whose implementation is not its own declaration keeps a **non-executable** declaration file `<dir>/<name>.gate`. It is data, never sourced and never run.
>
> **Its existence is the dispatch declaration.** Without an `# exec:` field it dispatches to the binary's subcommand named for the gate, so no mapping table or second registry can drift from what it names. With one, it dispatches to the file the field names (§The gate-authoring surface), and that path is held to a tracked file by §check-gate-substrate-parity assertion K.
>
> **The field roster is closed, and every field on it has a named reader:**
>
> - `# graph:`, required — the manifest readers below.
> - `# spec:` — canon-kit's `check-spec-pointer`.
> - `# install:` — §check-install-disposition; `# armed-by:` — `doctor` and §check-install-disposition; `# projection:` — §check-projection-roster. All three are ruled at §The install disposition, in a descriptor on the same terms as in a script.
> - `# no-fixture:`, optional — §check-gate-fixture-coverage.
> - `# exec:`, optional, consumer descriptors only — `registry::substrate`, and §check-gate-substrate-parity assertion K.
>
> The descriptor carries no field without a reader and reserves nothing for a future one.

Replacement for §The `# graph:` manifest's paragraph **The layer above the descriptor is substrate-blind.** **Not yet applied.**

> **The layer above the descriptor is substrate-blind.** The graph, hook and meta-gate layers read a gate's declaration without knowing what implements it, and the one question about the implementation, which substrate it is, has one reader (§The gate-authoring surface). A descriptor discloses a gate's shape without its predicate, which §Consumer payload relies on.

Replacement for the last sentence of §The `# graph:` manifest's paragraph **The answering interface is substrate-neutral**: *Whether a later authoring SDK moves this contract to a substrate-neutral surface is that SDK's question.* becomes *An exec member answers neither flag, and §The gate-authoring surface rules why no argv convention is asked of it.* **Not yet applied.**

`native/src/gates/comment_tier.rs`' built-in directive roster, `SHELL_COLON`, gains `exec:`. Without it every descriptor carrying the field reds `check-comment-tier`, whose surface this repo's `CANON_KIT_COMMENT_SURFACE` sets to `**/*.gate`.

### (4) The fixture runner and `gate_command` reach an exec member through `--emit gate-argv` {design-bearing} {user-facing: custom-gate-substrates' Deliverable — the fixture-pair contract it names, met for a PowerShell gate}

`gate_command` (§lib/gate.sh) prints a member's argv, and the fixture runner and four shell callers execute it. A shell parse of the `# exec:` field would be a second holder of the field's grammar and of the host argument list. So for a `.gate` declaration `gate_command` asks the binary. It keeps its absent-binary check, then prints what a new arm, `--emit gate-argv <declaration>`, prints: `registry::invocation`'s answer, one element per line, with the binary's own path where a compiled member's first element is. The arm's refusal reaches `gate_command`'s caller as the existing harness-error status, never `return 1`.

- **The arm.** `native/src/emit/gate_argv.rs`, an `Arm::Emit` row declaring `GATE_SDK_POWERSHELL`, whose named caller is `gate_command`. Exit 0 prints the argv; exit 2 is a refusal.
- **The fixture runner.** `run_gate_tests.rs` applies its executable guard and its absolutization to the first element when that element carries a path separator, and resolves a bare host name through the crate's program resolution instead. It absolutizes an exec member's target element before the `cd` into the case, as it does a script's first element.

Replacement for §lib/gate.sh's `gate_command` bullet, its first sentence. **Not yet applied.**

> - `gate_command` returns the **invocation argv**, one element per line: the one-element `<dir>/<name>.sh`, or for a `.gate` declaration what `--emit gate-argv` answers, `<binary> <name>` for a compiled member and the host, its arguments and the target for an exec member (§The gate-authoring surface). It never adds an `env` prefix, since a member reads its knobs itself.

Replacement for §run-gate-tests' paragraph **The case runs whatever the member dispatches to**, its first sentence. **Not yet applied.**

> **The case runs whatever the member dispatches to**. The invocation resolves through `gate_command` (§lib/gate.sh), so a case runs `<binary> <name>` for a compiled member, the host and its target for an exec member, and the script for a shell one, and the executable guard applies to the resolved argv's first element where that element is a path.

**Honest limit, stated in §run-gate-tests:** the fixture runner still resolves each case through a `bash` spawn sourcing `lib/gate.sh`. A native-Windows adopter running a PowerShell gate's fixture pair therefore uses Git for Windows' bash, which an install there already requires. The battery and the hooks start the gate with no shell.

### (5) Each meta-gate's reading of an exec member {design-bearing} {user-facing: custom-gate-substrates' Deliverable — the output, fail-closed, fixture-pair and self-lint contracts it names}

The roster is every crate site that branches on a declaration's spelling, by `grep -rn 'ends_with(".gate")\|strip_suffix(".gate")' native/src`, together with the corpus readers the substrate-reader survey named. Each site's satisfying value:

- **`check-gate-substrate-parity`.**
  - Assertion B's descriptor set, and assertion I's reverse direction ("a registration that dispatches to the binary"), take compiled members only, through the classifier. An exec descriptor names no subcommand by design.
  - **New assertion K, exec placement.** Every `# exec:` field sits in a descriptor directly under the consumer gates directory, is the descriptor's only one, and names, relative to that directory with no `..` segment and no root, a tracked regular file of an admitted target class. A descriptor under a kit root carrying the field reds, naming §The gate-authoring surface.
  - Assertions A, C, D, E, F, G, H and J are unchanged. E reads kit roots, where K forbids the field, and an exec target in the gates directory is outside E's vendoring set.
  - Its pair gains `bad/` cases for K's four reds and a `good/` exec descriptor. Its descriptor's `couples=` reaches a kit descriptor through `kit:checks/*` but a consumer one through nothing, since its gates-directory tokens are `gates.list` and `*.sh`. So it gains `knob:GATE_SDK_GATES_DIR/*.gate` and `knob:GATE_SDK_GATES_DIR/*.ps1`, and a commit adding, moving or deleting an exec field or its target fires K.
- **`check-gate-binary-fresh`** (`gate_binary_fresh.rs:88`): a member makes the binary load-bearing only when the classifier answers compiled. An exec member's spawn runs the host, never a subcommand.
- **`check-reads-couples`** (`reads_couples.rs:471`): an exec member is skipped and counted on the clean line as an exec member declaring no reads, where a compiled descriptor with no roots row stays exit 2.
- **`check-gate-assertions`** (`gate_assertions.rs:272`): the `# assertion` lines of an exec member are read from its target, its locality-class home, as a compiled member's are read from its module.
- **`check-gate-output`** (`gate_output.rs:146`): its static corpus is reached only by a `# no-fixture:` member, which an exec member cannot be (next bullet). The site asks the classifier so a compiled-only branch never reads an exec target as a module.
- **`check-gate-fixture-coverage`**: an exec member owes a `good/`+`bad/` pair, and `# no-fixture:` on its descriptor is a finding. The run-time oracle is the only output-contract and fail-closed check its substrate has.
- **`port-blockers`** (`port_blockers.rs:548`): an exec member leaves the owed partition, as a compiled one does. Its per-member requirement row is `?<TAB>GATE_SDK_POWERSHELL`, the knob form, so the host it needs is reported as that knob's command word.
- **`check-gate-tamper`** (delegation-kit/SPEC.md §Verify after every agent commit): the derived gate-file set gains each registered exec member's resolved target, so an edit to a PowerShell gate's implementation is held to meta-layer isolation as its descriptor is.
- **Unchanged, each by its own predicate.** `check-exec-bit` holds a `.gate` at 100644 and reads no target, and a target needs no execute bit since the host starts it. `check-gate-fail-closed` globs `check-*.sh`. `check-install-disposition`, `check-readme-roster`, the installer's recipe and selection derivations and the consumer smoke's descriptor count read kit `checks/`, where K forbids the field. `toolfloor`'s registered-shell-gate arm reads `.sh` members, and the host is a consumer-named command, the consumer's requirement under §The program roster, so the probe roster gains no row. `check-graph`'s `knob:` admissibility treats an exec member as it treats a shell one, with no registry row. `doctor`, `check-gate-exemption-tasks` and every manifest reader read the descriptor as today.

Replacement for delegation-kit/SPEC.md §Verify after every agent commit, assertion A's last sentence. **Not yet applied.**

> A gate file is a path a `DELEGATION_KIT_GATE_FILES` glob matches, **or** a registered member's resolved declaration or, for a descriptor carrying `# exec:`, its target, **or** the gate-sdk library.

### (6) `check-psscriptanalyzer`, the PowerShell self-lint {design-bearing} {user-facing: custom-gate-substrates' Deliverable, "a PowerShell lint counterpart to `check-shellcheck`"}

A new section of gate-sdk/SPEC.md after §check-shellcheck, and a new member: `gate-sdk/checks/check-psscriptanalyzer.gate`, its module `native/src/gates/psscriptanalyzer.rs`, its pair under `gate-sdk/gate-tests/check-psscriptanalyzer/`. **Not yet applied.**

> ### check-psscriptanalyzer
>
> `checks/check-psscriptanalyzer.gate` (`precommit`, binary-dispatched, `on-surface`, `# armed-by: GATE_SDK_POWERSHELL`).
>
> Invariant: every `*.ps1` directly under the consumer gates dir, under each vendored kit's `lib/`, `bin/`, `checks/`, `templates/` and `smoke/`, and under each directory named in `GATE_SDK_LINT_EXTRA_DIRS`, and every registered exec member's PowerShell target, passes PSScriptAnalyzer at severity Warning and above. The corpus is §check-shellcheck's, read for the PowerShell extension, plus the targets.
>
> - **Disarmed while `GATE_SDK_POWERSHELL` is empty**: the gate prints its clean line saying the PowerShell host is off and asserts nothing. A consumer who writes no PowerShell sets no host and meets no PowerShell requirement.
> - **Armed**, it starts the host once, with `-NoLogo -NoProfile -NonInteractive -Command` and a script that is a constant of the module, handing the targets in an environment variable, one path per line, and the excluded rules in a second. No path or rule name is composed into the command text, which is where an injection would live. The script exits 3 where the PSScriptAnalyzer module is not installed for that host, which the gate refuses at exit 2 with the install remedy. Severity is Warning, Error and ParseError, so a script the analyser cannot parse reds. Each finding prints as `<path>:<line>: <rule> <message>`.
> - **A rule the consumer contradicts tree-wide is excluded by name, and nothing else is.** `GATE_SDK_PSSCRIPTANALYZER_EXCLUDE`, a whitespace-list scalar defaulting empty, names rules the gate passes as `-ExcludeRule`. It exists for a rule whose premise a constraint the consumer holds on every script denies, where a per-site suppression would repeat one ruling on every file. The worked instance is `PSUseBOMForUnicodeEncodedFile`: a tree that keeps its PowerShell ASCII outside comments and BOM-less, because Windows PowerShell 5.1 and a shebang-reading shim each break on the alternative (installer/SPEC.md §Requirements, the install blocks), excludes it. Any other finding is fixed, or suppressed at its site.
> - **Exit 2** also for a host that does not resolve, probed before the targets are globbed, and for an empty target set while armed, on §check-shellcheck's ground that a derivation selecting nothing has verified nothing.
> - **Its rule is an invocation of the analyser**, criterion 7's wrapper class (§The port-candidate criteria), so the host is a declared requirement the payload does not carry. Its `needs` row is `?<TAB>GATE_SDK_POWERSHELL`.
> - **The verdict is a function of the analyser's release**, as ShellCheck's is. The CI verdict's reference is the version the consumer's workflow pins, installed from the upstream release asset and refused unless its digest matches. In this tree `.github/workflows/gates.yml` owns that pin, and it is the newest release the `pwsh` that `--with-foreign-shells` provisions can load, so a contributor's local run reads against the same analyser.
> - **A false positive** is silenced at the site with PowerShell's `SuppressMessageAttribute` and its justification argument, never with a settings file widening the gate away.
>
> **The pair runs on every host the fixture suites run on**, because each case pins `GATE_SDK_POWERSHELL` in its own knob file to a stub host in the case dir. The stub's final component is `pwsh`, and it answers as the analyser would. The pair therefore proves the wrapper: its exit mapping, its finding lines, its `help:` line, its disarmed clean line and its missing-module refusal. The analyser's own judgement is proved by the armed battery run over this tree on CI.

Its descriptor's manifest:

```text
# graph: couples=knob:GATE_SDK_GATES_DIR/*.ps1,kit:*.ps1,knob:GATE_SDK_LINT_EXTRA_DIRS/*.ps1,knob:GATE_SDK_GATES_DIR/*.gate dir=one valve=none tier=precommit
```

The last token fires the gate when a descriptor gains or moves an `# exec:` target. `gate-sdk/README.md`'s gate roster block gains `check-psscriptanalyzer` after `check-shellcheck`, which `check-readme-roster` holds. This repo registers it in `scripts/gates.list`, where it stays disarmed on a seat with no host and is armed on CI by delta 8.

### (7) The install page states each substrate's prerequisites {mechanical} {user-facing: custom-gate-substrates' Deliverable, "the install page's prerequisites per substrate"}

Replacement for `docs/requirements.md` §Writing your own shell gates' fourth sentence, *On native Windows that bash is Git for Windows' bash: a gate cannot be written in PowerShell today.* **Not yet applied.**

> On native Windows that bash is Git for Windows' bash, or write the gate in PowerShell instead.

A new section of `docs/requirements.md` between §Writing your own shell gates and §Writing your own Rust gates. **Not yet applied.**

> ## Writing your own PowerShell gates
>
> A PowerShell gate is two files in your gates directory: a `.gate` descriptor carrying the gate's `# graph:` line, its `# spec:` line and `# exec: <script>.ps1`, and the script it names. It needs a PowerShell host, which you name by setting `GATE_SDK_POWERSHELL` in `gate-sdk-config.knobs`: `powershell` for Windows PowerShell 5.1, which every Windows host carries, or `pwsh` for PowerShell 7 on any system. The gate runs as your shell gates do and prints the same lines; it reads a knob through `--emit knob-values <NAME>` on the binary its environment names. `check-psscriptanalyzer` lints it once you register it, and then the PSScriptAnalyzer module binds you. Running its fixture pair with `--run-gate-tests` also uses Git for Windows' bash. [gate-sdk/SPEC.md](gate-sdk/SPEC.md#the-gate-authoring-surface) owns this.

The first sentence of `docs/install.md`'s Windows **You need** paragraph's second sentence becomes: *Git for Windows' bash is also what runs any shell gate you write, and a gate you write in PowerShell needs none ([Writing your own PowerShell gates](requirements.md#writing-your-own-powershell-gates)).* **Not yet applied.**

`gate-sdk/README.md`'s Quick start paragraph on the first gate gains one sentence after its first: *To write it in PowerShell, write a `scripts/check-<area>.gate` descriptor whose `# exec:` line names `scripts/check-<area>.ps1`, and set `GATE_SDK_POWERSHELL` ([SPEC.md §The gate-authoring surface](SPEC.md#the-gate-authoring-surface)).* **Not yet applied.**

### (8) CI arms the lint and pins the analyser {mechanical}

`.github/workflows/gates.yml`'s `gates` job pins PSScriptAnalyzer **1.23.0** and the SHA-256 of its release asset `PSScriptAnalyzer.1.23.0.nupkg`, `b9969345f78251112fb75bdddc789be8689a343472a2e09762303aa03287afab`, in the workflow's `env` beside the ShellCheck pin. A step downloads that asset, refuses it on a digest mismatch, and expands it into a module directory before the battery step. The battery step sets `GATE_SDK_POWERSHELL: pwsh`, which arms `check-psscriptanalyzer` over this tree's `.ps1` files. No other job arms it.

- **Why 1.23.0.** Releases 1.24.0 and 1.25.0 refuse to load under a `pwsh` older than 7.4.6, and the image `GATE_SDK_FOREIGN_PWSH_IMAGE` pins carries 7.4.2, so 1.23.0 is the newest a local run can reproduce. The release page publishes a digest for 1.25.0 only, so the 1.23.0 value is the one computed from the downloaded asset.
- **The findings it raises today.** The corpus reaches four files: `gate-sdk/bin/run-gates.ps1`, `installer/bin/checkwright.ps1` and `docs/install.ps1` through `GATE_SDK_LINT_EXTRA_DIRS`, and `scripts/ci-front-door.ps1` directly under the gates directory. Run at Warning and above under 1.23.0 in that image, they raise 20 findings. Four are `PSUseBOMForUnicodeEncodedFile`, one per file, which this repo excludes with `GATE_SDK_PSSCRIPTANALYZER_EXCLUDE = PSUseBOMForUnicodeEncodedFile` in `scripts/gate-sdk-config.knobs`, its cause on a `comment-tier-exempt:` line. The other 16 are three empty catch blocks in `run-gates.ps1`, three assignments to the preference variable `PSNativeCommandArgumentPassing` read as unused, seven `Write-Host` calls in `ci-front-door.ps1`, two plural cmdlet nouns and one state-changing function without `ShouldProcess`. Build fixes each or suppresses it at its site with a justification, before the commit registering the gate, and re-runs the analyser the same way to green. An edit to `run-gates.ps1` re-runs `--run-front-end-parity` under `--with-foreign-shells`, and one to either bootstrap or `docs/install.ps1` keeps `check-portability-floor`'s ASCII arm green and is carried by the mid-iteration push's install-smoke legs, the oracles those files already have.
- **Inferred, cannot run before build:** that the battery step's host imports the pinned module rather than a copy the runner image already carries — the step does not exist until build writes it, and its first run is the mid-iteration push.

The local run is the image's `pwsh` in `docker run` over a read-only mount of the tree, with the pinned asset expanded onto its `PSModulePath`.

### (9) The crate cases and the bespoke test {mechanical}

- **`registry.rs`**: `substrate` classifies each of the three; refuses two fields, an empty value, a rooted value and a `..` segment; `invocation` returns each substrate's argv, and refuses an absent target, a non-PowerShell target and an empty host.
- **`substrate.rs`**: the extension test without regard to case; the validator's accept and refuse sets; the argument list, held equal to `scratch_run`'s use of it.
- **A live dispatch case** builds a scratch consumer whose `gates.list` registers an exec member with a passing and a failing target, and runs `--run` over it under each host that resolves. It asserts `PASS`, `FAIL (exit 1)`, the invariant line and the member's view of `GATE_SDK_NATIVE_BIN`. **On a Windows build the case runs under `powershell` unconditionally**, since every Windows host carries it, and under `pwsh` where it resolves. So the Windows legs bind it with no workflow step, and a seat with neither host on another platform runs only the refusal cases. A local run under `--with-foreign-shells` reaches the `pwsh` arm.
- **`gate_argv.rs`**: the arm's three answers and its refusal.
- **`gate-sdk/gate-tests/lib-gate.test.sh`** gains `gate_command` rows for an exec member: the argv it prints, and its harness-error status on the arm's refusal.
- **`check-gate-substrate-parity`'s and `check-psscriptanalyzer`'s pairs** (deltas 5 and 6).

### (10) The release declarations {mechanical}

`.workflow/release-declarations.md` gains three bullets:

- §New and tightened gates: **`check-psscriptanalyzer`** — new, `on-surface`: lints your PowerShell gates and scripts with PSScriptAnalyzer once `GATE_SDK_POWERSHELL` names a host. Nothing to do until you write PowerShell. `check-gate-substrate-parity` gains assertion K, which reds a misplaced or dangling `# exec:` field, and `check-gate-fixture-coverage` reds `# no-fixture:` on a descriptor carrying one.
- §Knob changes: **`GATE_SDK_POWERSHELL`** — new, empty by default: the PowerShell host that starts a gate's `# exec:` target and a PowerShell KPI plugin, `pwsh` or `powershell`. **`GATE_SDK_PSSCRIPTANALYZER_EXCLUDE`** — new, empty by default: PSScriptAnalyzer rules `check-psscriptanalyzer` skips, for a rule your tree contradicts on every script.
- §Gate-authoring changes: **a gate in PowerShell** — a `.gate` descriptor in your gates directory may carry `# exec: <script>.ps1`, and the battery and the hooks then start that script with the host `GATE_SDK_POWERSHELL` names. A member that is not compiled into the binary receives `GATE_SDK_NATIVE_BIN` naming the running binary.

### (11) The scratch runner shares the PowerShell host's arguments {mechanical}

`emit/scratch_run.rs` takes `HOST_FLAGS`, `is_powershell_body` and the knob validator from `substrate.rs`; its behaviour does not move. Replacement for guard-kit/SPEC.md §scratch-run's sentence beginning *The command is `-NoLogo -NoProfile -NonInteractive -ExecutionPolicy Bypass -File <snapshot> [args…]`, each flag for a reason:* and its four bullets. **Not yet applied.**

> The command is the PowerShell target class's argument list with the snapshot as the file, `[args…]` after it, each flag for the reason gate-sdk/SPEC.md §The gate-authoring surface gives. `-NoProfile` matters most here, since a profile is code the echo never shows.

The honest limit on Group Policy that follows stays, and now also cites gate-sdk/SPEC.md §The gate-authoring surface, which states it for a gate.

## Producers and consumers

- **The `# exec:` field.** Producer: the adopter, in a consumer descriptor. Consumers: `registry::substrate`, by file read at every classification, and assertion K. Its one value, the target path, is read by `registry::invocation` at dispatch, by assertion K for placement and class, by `check-gate-tamper`'s derived set, by `check-gate-assertions` and by `check-psscriptanalyzer`'s corpus.
- **`registry::Substrate`.** Producer: `registry::substrate`. Consumers: `registry::invocation` and every meta-gate site in delta 5, by call.
- **The exec spawn.** Producer: `registry::invocation`, called by `dispatch_one`, which serves `--run`, `--only`, `--for` and the git-hook arm, and by the stage-entry pre-flight, the identity shadow and the consumer smoke's probe. Enabling config: a consumer descriptor carrying the field and a non-empty `GATE_SDK_POWERSHELL`. This repo sets neither for dispatch, so its live producers are the crate cases (delta 9) and a consumer's tree. Consumer: the target script, by argv and environment. Its two fields: the target path, read by the host's `-File`, and `GATE_SDK_NATIVE_BIN`, read by the script when it reads a knob.
- **`GATE_SDK_POWERSHELL`.** Producer: the consumer's knob file or local overlay; CI's `gates` job in this repo (delta 8). Consumers: `registry::invocation`, `--emit gate-argv`, `check-psscriptanalyzer`, and drift-kit's plugin start (drift-kit/SPEC-kpi-powershell.md). Each declares it in its registry row or arm row.
- **`GATE_SDK_PSSCRIPTANALYZER_EXCLUDE`.** Producer: the consumer's knob file; this repo's `scripts/gate-sdk-config.knobs` (delta 8). Consumer: `check-psscriptanalyzer`, which declares it and hands each word to the analyser as an excluded rule. It narrows the gate's findings, never its corpus, so the empty-target refusal is untouched.
- **`--emit gate-argv`.** Producer: the arm. Consumer: `gate_command`, by stdout, at each fixture case's resolution and at each shell caller's.
- **Roster-holding readers of the new names.**
  - The descriptor field roster in §The `# graph:` manifest (delta 3), and `comment_tier.rs`' `SHELL_COLON` (delta 3).
  - The knob table in `native/src/knobs/gate_sdk.rs` and §Layout and configuration (delta 2); `--emit knob-roster` derives from the table.
  - `scripts/gates.list` and `gate-sdk/README.md`'s roster block for the new gate (delta 6). `check-kit-registration` and `check-readme-roster` hold those.
  - The `ARMS` table and its test-module census for `gate_argv.rs` (delta 4); `runner.rs`' `USAGE` names the `--emit <arm>` family generically, so it gains nothing.
  - `REGISTRY` in `native/src/gates/mod.rs` for the new gate's row.
  - §Meta-gate conservation for the binary substrate's member rows: assertion C derives which members owe a row from the couples reach, so build runs C after registering the gate and adds the row it names, if any.
- **The seam.** The field, the classifier, the target class, the host arguments, the arm and the lint wrapper are kit mechanism. Which host, the analyser's pinned version, and every exec target are consumer config or consumer content. No private rule content is involved.

## Existing sections updated

- `gate-sdk/SPEC.md` §The gate-authoring surface, new; §What the dispatch seam does not settle; §The extensibility model (delta 1).
- `gate-sdk/SPEC.md` §Layout and configuration — the `GATE_SDK_POWERSHELL` row (delta 2).
- `gate-sdk/SPEC.md` §The `# graph:` manifest — the descriptor paragraphs, the field roster, the substrate-blind paragraph and the answering-interface sentence (delta 3).
- `gate-sdk/SPEC.md` §lib/gate.sh — the `gate_command` bullet (delta 4).
- `gate-sdk/SPEC.md` §run-gate-tests — the dispatch paragraph and the honest limit (delta 4).
- `gate-sdk/SPEC.md` §check-gate-substrate-parity — assertions B, I and K, and the pair (delta 5).
- `gate-sdk/SPEC.md` §check-gate-binary-fresh, §check-reads-couples, §check-gate-assertions, §check-gate-output, §check-gate-fixture-coverage, §port-blockers — each one's exec reading (delta 5).
- `gate-sdk/SPEC.md` §check-psscriptanalyzer, new (delta 6).
- `delegation-kit/SPEC.md` §Verify after every agent commit — assertion A's gate-file sentence (delta 5).
- `guard-kit/SPEC.md` §scratch-run — the host arguments (delta 11).
- `native/src/registry.rs`, `native/src/substrate.rs`, `native/src/knobs/gate_sdk.rs`, `native/src/knobs/guard_kit.rs` — the classifier, the invocation, the module and the knob (delta 2).
- `native/src/runner.rs`, `native/src/emit/enter_stage.rs`, `native/src/emit/install_hooks.rs`, `native/src/emit/run_consumer_smoke.rs` — each dispatch through `registry::invocation` (delta 2).
- `native/src/gates/comment_tier.rs` — `SHELL_COLON` (delta 3).
- `native/src/emit/gate_argv.rs`, `native/src/emit/mod.rs`, `gate-sdk/lib/gate.sh`, `native/src/emit/run_gate_tests.rs` — the arm and its callers (delta 4).
- `native/src/gates/gate_substrate_parity.rs`, `gate_binary_fresh.rs`, `reads_couples.rs`, `gate_assertions.rs`, `gate_output.rs`, `gate_fixture_coverage.rs`, `gate_tamper.rs`, `native/src/emit/port_blockers.rs` — the classifier's readers (delta 5).
- `gate-sdk/checks/check-gate-substrate-parity.gate` and `gate-sdk/gate-tests/check-gate-substrate-parity/` — the two couples tokens and the K cases (delta 5).
- `gate-sdk/checks/check-psscriptanalyzer.gate`, `native/src/gates/psscriptanalyzer.rs`, `native/src/gates/mod.rs`, `gate-sdk/gate-tests/check-psscriptanalyzer/`, `scripts/gates.list`, `gate-sdk/README.md` — the new gate (delta 6).
- `docs/requirements.md`, `docs/install.md`, `gate-sdk/README.md` — the prerequisites and the authoring pointers (delta 7).
- `.github/workflows/gates.yml` — the analyser pin and the arming (delta 8).
- `scripts/gate-sdk-config.knobs` — this repo's rule exclusion (delta 8).
- `gate-sdk/bin/run-gates.ps1`, `installer/bin/checkwright.ps1`, `docs/install.ps1`, `scripts/ci-front-door.ps1` — the 16 findings fixed or suppressed (delta 8).
- `native/src/emit/scratch_run.rs` — the shared host arguments (delta 11).
- `gate-sdk/gate-tests/lib-gate.test.sh` — the `gate_command` rows (delta 9).
- `.workflow/release-declarations.md` — three bullets (delta 10).
- `docs/gate-sdk/SPEC.md`, `docs/gate-sdk/README.md`, `docs/guard-kit/SPEC.md`, `docs/delegation-kit/SPEC.md` — generated mirrors, stale once a delta lands (all deltas).

## Retired spellings

- None — the amendment adds a field, a class, a knob, an arm and a gate, and rewrites the sentences that said a descriptor has no dispatch field and a gate cannot be written in PowerShell without retiring a name.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls gate-sdk/SPEC-*.md`), discharged at the iteration.
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Entries moved** — `gate-authoring-sdk-surface` and `custom-gate-substrates` move to Done with `--queue done` once their remote oracle is read: the mid-iteration push's Windows `crate-tests-windows` legs and the `gates` job's armed lint, green. Each moves ahead of the drain stage.
