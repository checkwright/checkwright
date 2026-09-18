# SPEC amendment: windows-front-end

A native-Windows adopter cannot reach the battery without bash today. The
front-end `gate-sdk/bin/run-gates.sh` is bash. Both generated git hooks are
bash. guard-kit's `PreToolUse` hook is bash too. On a stock Windows host, a bare
`bash` typed in PowerShell reaches the WSL launcher in the system directory
rather than a shell. This amendment covers the first two of those three sites.
It adds a PowerShell twin of the front-end, held to the bash stub by an executed
comparison, and it rules how the hooks run on native Windows. guard-kit's hook
is a separate decision and stays Deferred (`guard-hook-windows-substrate`). Two
Windows debt items share the surface: the Windows build's two `dead_code`
warnings, and the Windows legs' copied `shellcheck` route.

**Probed at authoring, 2026-09-18, at `ef6ceb71`:**

- `gate-sdk/bin/run-gates.sh` is 66 lines. It sources `lib/gate.sh` for
  `gate_native_bin` and `gate_sdk_gates_dir`, and keeps the two-name
  `--hook`/`--statusline` test (read at HEAD).
- `scripts/git-hooks/pre-commit` and `commit-msg` open with
  `#!/usr/bin/env bash`. Outside `run_gate`'s argv they call bash builtins,
  `git`, and the binary path, and nothing else. Probe:
  `grep -v '^run_gate \|^#\|^$' scripts/git-hooks/pre-commit`.
- Every command in `.claude/settings.json` runs through `bash`. Probe:
  `grep -o '"command": "[^"]*"' .claude/settings.json`.
- `proc::resolve_interpreter` has no caller. The Windows spawn funnel
  (`spawn_target` → `spawn_resolution`) already applies `SYSTEM_DIR_HOMONYMS`'
  `Refuse` face to every spawn, so gate-sdk/SPEC.md's sentence saying the funnel
  "selects between" `resolve_interpreter` and `resolve_floor_tool` is stale.
  Probe: `grep -rn resolve_interpreter native/src gate-sdk/SPEC.md`.
- `WAIT_BODY`'s one non-test reader is `cfg(unix)` (`native/src/emit/wait_probe.rs:249`).
- The Windows legs install `shellcheck` at two sites: `install-smoke-windows`'
  bash step and `install-smoke-powershell`'s pwsh step
  (`grep -n 'choco install' .github/workflows/gates.yml`). The page's route is
  prose in the `shellcheck` bullet of docs/install.md §Requirements.
- This host has no PowerShell (`which pwsh powershell` finds neither), so
  nothing in delta 2 can run here. Its oracle runs on the CI legs named below.

## What changes

### (1) A PowerShell twin of the front-end, `gate-sdk/bin/run-gates.ps1` {design-bearing}

`gate-sdk/bin/run-gates.ps1` does the same five-step residue as the bash stub,
in the same order:

1. Resolve the repository root (`git rev-parse --show-toplevel`) and change
   directory to it, or refuse at exit 2 with the stub's own
   `run-gates: not inside a git repository`.
2. Export `GATE_SDK_ROOT` the way the stub does: relative when the gate-sdk root
   lies under the repository root, absolute otherwise.
3. Resolve the gates-dir positional with the stub's residual argv grammar
   (§run-gates), case for case.
4. Locate the binary. It reads `GATE_SDK_NATIVE_BIN` over the pre-binary
   precedence that `lib/gate.sh`'s `_gate_prebinary_knob` implements: the
   environment, then `<gates-dir>/gate-sdk-config.local.knobs`, then
   `GATE_SDK_KNOB_FILE` or `<gates-dir>/gate-sdk-config.knobs`, then the default.
   An empty value takes the default. The default is
   `native/target/release/checkwright-gates`, plus `.exe` on a Windows host.
   `<gates-dir>` is `GATE_SDK_GATES_DIR` or `scripts`.
5. Run the binary with the argv, inheriting all three streams, and exit with its
   status. PowerShell has no `exec`, so a child process whose status becomes the
   twin's own is the equivalent.

The absent-binary branch keeps the stub's message and the two-name test.
`--hook` and `--statusline` take `0`; every other leading token takes `2`.

- **Host class.** The twin runs under Windows PowerShell 5.1 and PowerShell 7,
  the hosts `installer/bin/checkwright.ps1` already serves. It also pins that
  bootstrap's two argument-passing settings, on the same ground: tokens are
  forwarded unrewritten, and a native non-zero status stays a status rather than
  becoming an exception.
- **It sources nothing.** `lib/gate.sh` is bash, so the twin re-implements the
  three accessors it needs: `_gate_prebinary_knob` for one knob,
  `gate_sdk_gates_dir`, and the host half of `gate_exe_suffix`. That makes it a
  second holder of a contract, and delta 2 is the standing comparison §run-gates
  requires of every second holder.
- **`# no-port:`** takes the stub's cause, a per-file bootstrap: the front-end
  locates the binary it runs. Its `# spec:` header cites §run-gates.
- **Callers.** Anyone on a native-Windows host who has no bash on `PATH`. The
  bash stub stays the front-end every existing caller, settings grant and printed
  command uses. No caller moves in this unit.

**Not yet applied.** Replace the sentence of gate-sdk/SPEC.md §run-gates that
opens "`bin/run-gates.sh` is the **front-end**" (currently lines 10108-10113) with:

> The **front-end** is `bin/run-gates.sh` and its PowerShell twin
> `bin/run-gates.ps1`, and after the stub cut each is a residue: resolve the
> repo root, locate the binary over `GATE_SDK_NATIVE_BIN`'s pre-binary
> precedence, resolve the **gates-dir positional**, export the `GATE_SDK_ROOT`
> locator (§Layout and configuration), and run the binary with no knob
> environment. The bash stub `exec`s it. The twin runs it as a child and exits
> with its status. The twin exists for a native-Windows host with no bash on
> `PATH`, whose bare `bash` reaches the WSL launcher. It sources nothing, so it is
> a second holder of the stub's contract, held by the executed comparison below.

**Also not yet applied.** In the paragraph "The front-end's port disposition is
landed", the sentence "That stub declares `# no-port:` on a per-file bootstrap
cause" becomes "Both halves declare `# no-port:` on a per-file bootstrap cause".
"It is the whole of what stays shell on the front-end's path to the binary"
becomes "They are the whole of what stays script on the front-end's path to the
binary". In the `ARM_UNAVAILABLE_STATUS` paragraph, "The stub holds it as a
two-name test" becomes "Each half holds it as the same two-name test".

gate-sdk/README.md's `bin/run-gates.sh` bullet gains one clause naming the twin
and the host it is for. The build appends a release-declaration bullet: gate-sdk
ships a new file in `bin/`.

### (2) The front-end parity oracle, `--run-front-end-parity` {design-bearing}

A new non-gate arm of the binary, `Arm::Run` (§The non-gate arm). It runs both
front-ends over one fixed case corpus and compares, per case, the exit status,
stdout and stderr, byte for byte after one normalization: CRLF becomes LF,
because PowerShell's console writer ends lines with CRLF on Windows and that is
not a contract difference. Exit codes:

- 0: every case is identical.
- 1: some case diverges. The arm prints the case, both transcripts and the first
  differing line.
- 2: the check could not run. That covers no PowerShell resolving, no bash
  resolving, and a sandbox that cannot be built. It is never a clean result.

Each case runs in a fresh scratch git repository that vendors the tree's own
`gate-sdk/bin/` and `gate-sdk/lib/`, with one environment given to both halves.
Where a case needs a runnable binary, `GATE_SDK_NATIVE_BIN` names
`current_exe()`, driven on arms whose output depends only on argv, environment
and stdin. The corpus (point 6: every member named with its satisfying value,
identical transcripts):

- outside a repository: exit 2 and the not-inside-a-repository line;
- binary absent, leading token `--emit`: exit 2 and the build-remedy lines;
- binary absent, leading `--hook`: exit 0 and the same lines;
- binary absent, leading `--statusline`: exit 0 and the same lines;
- `GATE_SDK_NATIVE_BIN` resolved from each precedence tier in turn: the
  environment, the local overlay, the tracked knob file, a `GATE_SDK_KNOB_FILE`
  override, the default, and an empty value taking the default. Each case is
  observed through the absent-binary message, which prints the resolved path;
- each residual-grammar form: `-h`, `--help`, `--only <name>`, `--for <path>`,
  `--`, `-- <dir>`, no argument, a bare positional, and a leading `--emit <arm>`.
  Each is observed through the binary's own output for the argv it received;
- stdin forwarded: `--hook <name>` with a JSON payload on stdin, observed through
  the hook arm's output.

**Where it runs.** The arm is not a battery member, because the battery's host
may have no PowerShell. It runs as a step in two legs of
`.github/workflows/gates.yml`:

- `install-smoke-windows`. It is binding, and its host carries Git-for-Windows
  bash and PowerShell, so it measures the twin on the platform it exists for.
- The `gates` job on `ubuntu-latest`, so a divergence also reds on the leg every
  push runs first.

`pwsh` joins the program roster (§The program roster) with the `contributor`
audience. No adopter spawns it.

**Honest limit, stated in §run-gates at merge.** No local battery reaches the
comparison on a host without PowerShell, so a divergence reds at push, not at
commit. That is the same trade the macOS remedy block and the installer's
PowerShell half accept.

**Inferred, cannot run before build:** that `ubuntu-latest` ships `pwsh`
preinstalled. If it does not, the `gates` step is dropped and
`install-smoke-windows` alone carries the oracle. Neither this host nor any
recorded run has probed it.
**Inferred, cannot run before build:** that a native child run from `pwsh -File`
inherits the script's stdin when no pipeline input is bound. The stdin case is
what settles it, and the twin reads `[Console]::In` explicitly if it does not.

**Not yet applied.** §run-gates gains, after the twin sentence of delta 1:

> **The twin is held by `--run-front-end-parity`**, an executed comparison of
> both halves' transcripts over a fixed corpus: each exit path, each precedence
> tier of `GATE_SDK_NATIVE_BIN`, each residual-grammar form, and forwarded
> stdin. Transcripts must be byte-identical after CRLF becomes LF. It runs on the
> binding Windows install-smoke leg and on the `gates` job, never in the battery,
> whose host may carry no PowerShell. So a divergence reds at push.

§The program roster's member list gains `pwsh` (contributor), and
`native/src/programs.rs` gains the row.

### (3) The generated hooks on native Windows: run by git's own shell, no second implementation {design-bearing}

The two generated hooks stay bash and gain no PowerShell twin. On a native
Windows host, git runs a hook itself, under the shell Git for Windows bundles,
whatever `PATH` carries. The hooks call nothing but bash builtins, `git` and the
binary. So they add no member to a native-Windows adopter's floor beyond git,
which that floor already requires. A twin would be a second emission of every
trigger block with nothing to hold it equal, and it would buy nothing git does
not already supply.

**The oracle.** `install-smoke-powershell` gains one step after its `init`
assertions, in the scratch consumer:

1. Remove from `PATH` every directory that holds a `bash.exe`, so that no bash
   the step could resolve is reachable.
2. Wire the hooks with `pwsh -File gate-sdk/bin/run-gates.ps1 --install-hooks`.
3. Commit a clean change from `pwsh`. Assert that the commit lands and that the
   pre-commit hook's `gate(s) passed` line printed.
4. Commit a change carrying a violation a registered precommit gate refuses.
   Assert that the commit is refused and names that gate.

**Inferred, cannot run before build:** that Git for Windows runs a
`#!/usr/bin/env bash` hook with no bash on `PATH`, resolving the shebang inside
its own install root. Step 1 is the proof, which is why it strips `PATH` rather
than trusting it. A MinGit host, whose distribution may omit bash, is not
claimed. If the step shows git cannot run the hook, this delta stops and goes
back to the lead: the invocation form would then need a design this amendment
does not hold.

**Not yet applied.** §gen-pre-commit gains, after "The emission is deterministic
… byte-stable.":

> **The hooks are bash and stay one implementation on every platform.** On native
> Windows, git runs them under the shell Git for Windows bundles, whatever
> `PATH` carries, and they call nothing but bash builtins, `git` and the binary.
> So they add nothing to that host's floor beyond git. `install-smoke-powershell`
> holds this by committing with every bash stripped from `PATH`.

§install-hooks's opening sentence gains the twin spelling beside the bash one:
`pwsh -File gate-sdk/bin/run-gates.ps1 --install-hooks` on a host without bash on
`PATH`.

### (4) Retire `resolve_interpreter`, fence `WAIT_BODY`, and hold the class {mechanical}

- Delete `proc::resolve_interpreter`. `spawn_target`'s Windows funnel already
  applies `SYSTEM_DIR_HOMONYMS`' `Refuse` face to every spawn, so the function is
  a second spelling of that face with no caller.
- Put `WAIT_BODY` behind `#[cfg(unix)]`, beside its one non-test reader. Its
  tests are `cfg(unix)` too, or already sit under a unix-only module; build reads
  which.
- Every `native-artifacts` leg in `.github/workflows/gates.yml` gains a step
  after the artifact build: `cargo clippy --release --all-targets` at
  `-D warnings`, the lint half of `check-crate-arms`, run on that leg's own
  target. The artifact build stays `scripts/ci-build-artifact.sh`, unchanged,
  so what CI builds is still what a release publishes. A warning then reds the
  leg and never a release artifact. Of the targets, only the Windows leg compiles
  `cfg(not(unix))` code, so that is where the class gets held.

**Inferred, cannot run before build:** that each `native-artifacts` runner's
Rust toolchain carries `clippy`. The first pushed run shows it. If a runner's
toolchain lacks it, the step adds the component with `rustup` first.

**Not yet applied.** gate-sdk/SPEC.md, the sentence opening "The two
dispositions are implemented by `proc::resolve_interpreter` and
`proc::resolve_floor_tool`" (currently lines 14980-14985), becomes:

> The funnel, `spawn_target`, applies the roster's disposition to every spawn.
> `proc::resolve_floor_tool` also keeps its own callers, because it is a
> **reporting** resolver whose value is rendered in doctor's banner and in the
> env-probe emitter rather than only spawned. That is the one identity the
> funnel cannot absorb.

At currently line 2063, "a member spawning `resolve_interpreter("bash")`'s path
declares `bash`" becomes "a member whose spawn the funnel resolved to a path
declares `bash`". The comment at `native/src/proc.rs:234` that names
`resolve_interpreter`'s face is re-worded to name the funnel.

### (5) A Windows remedy block on the install page, run by both Windows legs {design-bearing}

docs/install.md §Requirements gains a marker block, `windows-remedy:begin` /
`windows-remedy:end`, placed after the `shellcheck` bullet's native-Windows
clause. It holds one `powershell` fence whose single line is the Chocolatey
route, in its non-interactive form:

```powershell
choco install shellcheck -y
```

The `shellcheck` bullet's "On native Windows the source is Chocolatey (`choco
install shellcheck`)…" becomes a pointer to the block, and the bullet keeps its
"measured rather than suggested" claim.

- **Both Windows legs run the block verbatim, under PowerShell**, because that
  is the shell a native-Windows adopter types into.
  - `install-smoke-powershell` extracts the fence body in PowerShell and runs
    it.
  - `install-smoke-windows` is a bash leg. It extracts the body with the same awk
    program the macOS legs use and runs it with `pwsh -NoProfile -Command`.
  - An empty extraction reds either leg by name, as the macOS legs refuse. Each
    leg's own `choco install shellcheck` line is deleted.
  - Each leg keeps its post-install reporting: where `shellcheck` resolves, and
    the loud line when it did not land.
- **`--no-progress` leaves with the copies.** It only quietened the CI log, and
  a leg that adds a flag the page lacks is exactly the drift this block exists
  to end.
- **No gate holds the block**, on the macOS row's own ground. It is hand-authored
  and has no emitter, and the binding legs are its enforcement.

**Not yet applied.** docs/site-architecture.md's "The macOS remedy block" bullet
(currently lines 311-332) becomes "The remedy blocks". It is re-phrased so one
bullet carries both: two hand-authored marker blocks, each read by its
platform's install-smoke legs and run verbatim, with an empty extraction red.
The macOS block persists the `PATH` entries it prepended. The Windows block runs
under PowerShell in both of its legs. The bullet keeps its two refused shapes and
its honest limit, which now covers both blocks.

The comment above `install-smoke-windows`' shellcheck step, which closes with
"the gap inbox carries that gap", is rewritten to say that the step runs the
page's block.

### (6) The install page's bash claims say what native Windows now needs {mechanical}

**Not yet applied.** docs/install.md §Requirements:

- The `bash` toolchain bullet's "the `run-gates.sh` front-end that locates the
  gate binary, both generated git hooks and guard-kit's hook are written in
  bash; nothing in the battery runs without it" becomes:

  > guard-kit's hook and the shipped session templates are written in bash, and
  > so are both generated git hooks, which git runs under its own shell. On
  > native Windows, run the battery from PowerShell through
  > `gate-sdk/bin/run-gates.ps1`, the twin of `run-gates.sh`. Git for Windows'
  > bundled bash is what serves guard-kit's hook there.

- The Requirements opening's "Git for Windows supplies the bash and the GNU
  userland they name" stays. It is still true, and delta 5's block names the one
  member Git for Windows does not supply.

The floor roster is unchanged. `bash` stays on `GATE_SDK_PROGRAM_FLOOR`, because
guard-kit's hook and the templates still need it. This unit narrows where bash
is required. It does not drop bash from the floor.

## Producers and consumers

- **`run-gates.ps1` (delta 1).** Producer: a person or a process on a native
  Windows host that runs `pwsh -File gate-sdk/bin/run-gates.ps1 …`. Its
  deployed callers at landing are delta 3's leg step and the delta 2 arm; no
  settings file or printed command names it yet. Consumers:
  - the binary, by argv, environment and inherited streams;
  - the vendoring path: `init` enumerates gate-sdk's payload without
    filtering, so the file ships with no roster edit (installer/SPEC.md);
  - roster-holding readers of kit files. Probe at build: add the file and run
    the full battery. A roster reader that lacks it reds (oracle-first). None is
    known at authoring: `grep -rln '\.ps1' native/src` names only
    `install_platforms.rs`, which reads the installer's bootstrap pair and not
    kit `bin/`.
- **`--run-front-end-parity` (delta 2).** Producers: the two workflow steps
  named in delta 2. Consumers:
  - its exit status, read by those steps;
  - the arm table and `check-crate-arms`' arm reach (§The non-gate arm). Arm
    registration is roster-held, so the arm's registration is an update target.
  Fields: the case name, both transcripts and the first differing line. A human
  reads each one on a red, and no program parses them.
- **The `pwsh` program-roster member (delta 2).** Producer: the parity arm's
  spawn. Consumer: `programs.rs`' four unit-test relations. The audience is
  `contributor`, so the relation requiring an adopter-side member to be on
  `GATE_SDK_PROGRAM_FLOOR` or `PROBE_SET` does not bind.
- **The hooks-under-git obligation (delta 3).** Producer: `git commit` on a
  Windows host. Consumer: `install-smoke-powershell`'s new step, which is
  binding.
- **`windows-remedy` markers (delta 5).** Producer: the page author. Consumers:
  the two Windows legs' extraction steps. `check-docs-render-fidelity` and
  every other docs gate already carry an HTML-comment marker pair on this page
  (`macos-remedy`), so a second pair adds no new shape. Probe at build: the
  battery.
- **The clippy step (delta 4).** Producer: each `native-artifacts` leg.
  Consumer: that leg's status.

## Existing sections updated

- `gate-sdk/SPEC.md` §run-gates: the front-end sentence at lines 10108-10113,
  the port-disposition paragraph at 10159-10175, and the `ARM_UNAVAILABLE_STATUS`
  paragraph from 10198 (deltas 1 and 2).
- `gate-sdk/SPEC.md` §The program roster: the `pwsh` member (delta 2).
- `gate-sdk/SPEC.md` §gen-pre-commit and §install-hooks (delta 3).
- `gate-sdk/SPEC.md` at lines 2063 and 14980-14985, the `resolve_interpreter`
  citations. Probe: `grep -n resolve_interpreter gate-sdk/SPEC.md` (delta 4).
- `gate-sdk/README.md`, the `bin/run-gates.sh` bullet (delta 1).
- `gate-sdk/bin/run-gates.ps1`, the new file (delta 1).
- `native/src/programs.rs` and the arm table's module, for the new arm and member
  (delta 2).
- `native/src/proc.rs`, which deletes `resolve_interpreter` and re-words the
  comment at line 234 (delta 4).
- `native/src/emit/wait_probe.rs`, the `cfg(unix)` fence (delta 4).
- `.github/workflows/gates.yml`, in five places:
  - the `gates` job's parity step (delta 2);
  - `install-smoke-windows`' parity step (delta 2);
  - its shellcheck step and that step's comment block at lines 376-401 (delta
    5);
  - `install-smoke-powershell`'s hooks step (delta 3) and its shellcheck lines
    at 764-769 (delta 5);
  - every `native-artifacts` leg's clippy step (delta 4).
- `docs/install.md` §Requirements: the `shellcheck` and `bash` bullets and the
  new block (deltas 5 and 6).
- `docs/site-architecture.md`, the remedy-block bullet (delta 5).
- `.workflow/release-declarations.md`: the new shipped file and arm (deltas 1
  and 2).
- `docs/gate-sdk/SPEC.md`, the generated mirror (all deltas).

## Retired spellings

- `resolve_interpreter` — the dead Windows interpreter resolver, deleted with its
  SPEC and comment citations (delta 4).

## Definition of Done

- [ ] **Causal completeness**: every point of canon-kit/SPEC.md §The
      causal-completeness check holds for each new state, event, interface and
      obligation.
- [ ] **Instruction surfaces: instruction only**: replacement text for a
      template, agent definition or shim carries no grounds, and a delta places
      them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost**: each addition re-phrases the
      canonical-spec text it refines rather than appending to it, and the merged
      spec reads as one document a reader who never saw the amendment can use
      alone.
- [ ] **Amendment deleted**: this file is removed on merge, and none remain for
      the component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated**: `## Retired spellings` above is accurate, and
      `check-amendment-retired-spelling` is green.
- [ ] **The oracle is green**: the pushed `gates` run shows
      `--run-front-end-parity` exiting 0 on `install-smoke-windows` (and on the
      `gates` job, unless delta 2's inferred marker dropped it). It also shows
      `install-smoke-powershell`'s hooks step and both Windows legs' remedy
      steps green, and every `native-artifacts` leg's clippy step green with no
      `dead_code` warning in the Windows build. The run id is cited in the
      validate evidence.
- [ ] **Entries moved before the drain stage**: `native-windows-bash-floor`
      leaves the queue as Done. Its guard-kit third is the separate
      `guard-hook-windows-substrate` entry, so this slice finishes the entry
      rather than one increment of it. `windows-shellcheck-step-copies-page-route`
      and `windows-build-dead-code-warnings-unheld` move to Done as well.
- [ ] **Gaps filed**: cross-component gaps discovered during the work are filed
      as debt tasks. A build-time causal gap is resolved that session, not
      deferred.
