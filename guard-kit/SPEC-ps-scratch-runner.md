# SPEC amendment: ps-scratch-runner

On a Windows host the harness's `PowerShell` tool can run a script whose body sits in the scratch dir (`& .tmp\x.ps1`, `pwsh -File .tmp/x.ps1`), and nothing steers it. So the body the approver saw at the permission decision need not be the body that runs, and nothing echoes it when it runs. Guard-kit rule `script_interpreter` has no PowerShell form (guard-kit/SPEC.md §The hook on native Windows, the honest limit), because the `--scratch-run` runner its corrective names runs bash bodies only. This amendment does three things:

- It gives the runner a **PowerShell path the consumer selects**. A knob names the PowerShell host, empty and off by default.
- It gives the rule its PowerShell form.
- It closes a window the runner has always had. The runner echoes the body and then hands its interpreter the **path**, and the interpreter reads the file again. From now on the runner executes the bytes it echoed, on both paths.

The consumer-selected shape and the fold of the re-read fix are operator directions (2026-09-25, lead-relayed), recorded on the queue entry.

**The component span, judged against the owners.** The runner, the rule, the knob and their tests are guard-kit's (§scratch-run, §The generic ruleset, §Layout and configuration, §Testing). The code lands in the crate: `native/src/emit/scratch_run.rs`, `native/src/guard/`, `native/src/knobs/guard_kit.rs`. The binding witness lands in the native-Windows leg of `.github/workflows/gates.yml`. No other kit's SPEC changes. gate-sdk's program-roster section says `pwsh` and `powershell` are contributor members spawned only by the front-end parity arm, and that stays true: the runner spawns the host the knob names as a consumer command grounded on that knob, never the roster members. So the unit is single-component and sited in guard-kit, as the PowerShell reader's amendment was before it merged into §The reader and its views.

**What was run at authoring** (2026-09-25, at `34bf0101`):

- **The window, against today's runner.** A body's first line rewrote, in place and with `dd … conv=notrunc`, a later line of its own file that sat about 100 KiB in, past bash's read buffer. `echo ORIGINAL` became `echo INJECTED`. `--scratch-run` on that file echoed `echo ORIGINAL` and printed `INJECTED`. The window is real, and delta 6's seam case reproduces it deterministically.
- **The PowerShell host CLI**, in `mcr.microsoft.com/powershell:latest` (pwsh 7.4.2, Linux, `--network none`):
  - `-NoLogo -NoProfile -NonInteractive -ExecutionPolicy Bypass -File <f> a 'b c' -Foo bar` bound `-Foo`, passed `a` and `b c` as `$args`, and passed `exit 3` through as rc 3. A `throw` exits 1.
  - `-ExecutionPolicy` is accepted on Linux, and `Get-ExecutionPolicy` reads `Unrestricted`.
  - `$PSCommandPath` and `$PSScriptRoot` name the file handed to `-File`.
  - `-f` and `-fi` are `-File`; `-nop`, `-nol`, `-noni`, `-ex` and `-ep` are accepted; a bare first word is `-File`; `-c` and `-com` are `-Command`.
  - `-File -`, `-Command -`, and a pipe into a bare `pwsh` each read commands from stdin.
  - An unrecognized option exits 64 with a usage line, so declining on one costs no real call.
  - `-wd <dir>` does not re-root the path given to `-File`.
  - pwsh 7 on Linux ran a non-`.ps1` file under `-File`.
- **The PowerShell forms that run a scratch body** in the same container: `.tmp/x.ps1` in command position, `& .tmp/x.ps1`, `. .tmp/x.ps1`, `& .\.tmp\x.ps1`, `Get-Content -Raw .tmp/throw.ps1 | Invoke-Expression`, and `pwsh .tmp/x.ps1`. `Invoke-Command -FilePath .tmp/x.ps1` refuses without a session, so it runs nothing locally.
- `git grep -n "bash-only"` over the tracked tree finds guard-kit/SPEC.md (§The generic ruleset's `script_interpreter` item and §scratch-run), `guard-kit/gate-tests/scratch-run.test.sh`, `native/src/emit/scratch_run.rs` and `native/src/guard/rules/reach.rs`, plus the generated mirror. `git grep -l "scratch-run\|scratch_run"` adds only citations that this unit leaves true: gate-sdk's path-dialect and non-gate-arm sections, context-kit's arm-spelling sentence, the close-triage template, and the `--rewrite` arm's reuse of `is_inside`.

Three premises are carried unrun:

**Inferred, not run:** Windows PowerShell 5.1 refuses a `-File` target without the `.ps1` extension, per Microsoft's documentation. Delta 2 keeps the extension, so the runner does not depend on it either way — `powershell -NoProfile -File .tmp/x.txt` on a Windows host

**Inferred, not run:** a machine or user Group Policy execution-policy scope outranks the process scope that `-ExecutionPolicy Bypass` sets, per Microsoft's documented precedence; no Windows host exists here — `powershell -NoProfile -Command "Get-ExecutionPolicy -List"` on a Windows host carrying such a policy

**Inferred, cannot run before build:** `guard-kit/gate-tests/scratch-run.test.sh` runs under Git Bash on the native-Windows leg once its paths are spelled for the binary (delta 6) — the file's PowerShell cases and path spelling are what delta 6 adds, and the leg in delta 7 is their first run

## What changes

### (1) The runner executes the bytes it echoed {design-bearing}

**Not yet applied.** In `native/src/emit/scratch_run.rs` and guard-kit/SPEC.md §scratch-run.

After the containment test passes and the body is read, and **before the echo**, the runner writes those bytes to a **snapshot**:

- The snapshot is a new file in the target's own resolved directory.
- Its name is derived from the target's basename, carries a token unique to the run, and keeps the target's extension. It begins with `.`, so a listing of the scratch dir does not show it as the target.
- It is created exclusively and never opened over an existing path. On unix its mode is owner read and write only.

The runner echoes the body, then a header naming the target and the snapshot:

`=== scratch-run: executing <target> as <snapshot> ===`

The first header is unchanged, so the seam case asserting `scratch-run: <target>` holds. The runner then hands the interpreter the **snapshot**, spelled as the target's own directory spelling joined to the snapshot's name, so a relative target stays relative. It passes `[args…]` verbatim. Once the child exits, whatever its exit code, the runner removes the snapshot. A failed removal is written to stderr and does not change the exit code, because the child's code is the contract. A snapshot the runner cannot create or write is refused before the echo at exit 2, naming the directory, so every refusal still prints no body.

The replacement text for §scratch-run, one paragraph after **Fail-closed on reach**:

> **The runner executes the bytes it echoed, never the path it was given.** An interpreter handed the target re-reads it, and bash reads a script in pieces as it runs, so a rewrite landing between the echo and the read ran a body the transcript never showed: the exact harm the echo compensates for. So the runner copies the bytes it read into a snapshot beside the target, created exclusively and named for this run with the target's extension kept, echoes those bytes, runs the interpreter on the snapshot, and removes it when the child exits. The snapshot sits in the target's directory so a body's references to its sibling files still resolve. Handing the body over as an argument or on stdin was refused: an argument meets the platforms' per-argument and command-line length limits, and a body on stdin is read by any command in it that reads stdin. **Honest limits.** `$0`, `BASH_SOURCE` and `$PSCommandPath` name the snapshot rather than the target. A runner killed mid-run leaves its snapshot behind, where the scratch dir's own disposal removes it (lifecycle-kit's iteration-boundary wipe, where that kit runs). The snapshot is as writable by the same user as any file, so the control holds against a concurrent session rewriting the shared scratch path and is no boundary against a process that targets the runner's own file.

§Testing's scratch-run paragraph gains the case in delta 6.

### (2) The PowerShell path {design-bearing}

**Not yet applied.** In `native/src/emit/scratch_run.rs` and guard-kit/SPEC.md §scratch-run.

**The interpreter comes from the file, and the extension decides it.**

- A target whose name ends in `.ps1`, matched without regard to ASCII case, is a **PowerShell body**. A shebang on it must name `pwsh` or `powershell`, read through the existing `/usr/bin/env` resolution, or be absent. Any other shebang is refused before the echo as a contradiction.
- Every other target is a **bash body**, as today. A shebang naming `pwsh` or `powershell` on one is refused before the echo, with the corrective to name the body `.ps1`.
- The extension decides because PowerShell itself dispatches a script on it (5.1 refuses a `-File` target without it). It is read off the file the runner already holds, so no roster is involved, and `GUARD_KIT_SCRIPT_INTERPRETERS` stays unread and undeclared by the arm.

**A PowerShell body needs a host the consumer named.** The arm's knob roster becomes `GATE_SDK_TMP_DIR` and `GUARD_KIT_SCRATCH_POWERSHELL` (delta 3). When the knob is empty, a PowerShell body is refused before the echo at exit 2. The message says the PowerShell path is off in this project, names the knob, and gives the bash alternative. When the knob is set:

- The runner checks that the named host resolves (`proc::on_path` for a bare name, a file test for a path). An unresolved host is refused before the echo, naming the knob and its value.
- After the echo it spawns the host as `Program::consumer("GUARD_KIT_SCRATCH_POWERSHELL", <value>)` with the argv `-NoLogo -NoProfile -NonInteractive -ExecutionPolicy Bypass -File <snapshot> [args…]`, and passes the exit code through.
- **Every flag carries a reason**, and the §scratch-run text states them:
  - `-NoProfile`: a profile is code the echo never shows.
  - `-NonInteractive`: a prompt would hang a call no one can answer.
  - `-ExecutionPolicy Bypass`: 5.1's default client policy refuses every script file. It sets the process scope only, which a Group Policy scope still outranks. On a non-Windows pwsh it is accepted and inert (measured).
  - `-NoLogo` keeps the banner out of the transcript.

**The replacement text for §scratch-run's policy paragraphs.** The four paragraphs from **Scratch execution is bash-only** through **What bash-only costs** are rewritten, not appended to:

> **Scratch execution runs in the harness's own shells: bash always, PowerShell where the consumer names a host.** The runner's interpreter set is the statement of that rule. Rule `script_interpreter` is the command-line half of the enforcement, and the runner's refusal of a body whose file states another interpreter is its own half.
>
> **Why the set is the harness's shells and nothing wider.** The runner is reached through the front-end grant (§The recommended allowlist), so each interpreter it runs is run on that grant with no settings edit. An interpreter the harness does not itself run as a shell tool would turn a grant for *run a reviewed body in the session's shell* into *run anything on a reviewed body*, and that stays refused. PowerShell is the harness's second shell tool. The echo covers a PowerShell body exactly as it covers a bash one, and admitting it adds no capability, because a bash body under the runner could already call `pwsh` inline. What the admission does move is the grant's reach, which is why it is the consumer's selection and off by default: a consumer that names a host widens what its front-end grant runs, in its own knob file rather than in a kit release.
>
> **What the set costs.** A session wanting a Python scratch script writes a shell script that invokes Python with the body inline, or does the work in a shell the control covers. The population of other-interpreter scratch runs is small, while the silent hole applied to every one of them.

The paragraph **The runner refuses a non-bash shebang** becomes the extension-and-shebang rule above. It keeps its closing argument that the guard has a command string and the runner has a file, and that this is why the arm does not read `GUARD_KIT_SCRIPT_INTERPRETERS`. **What deliberately does not become config: the policy** is rewritten:

> **What becomes config and what does not.** Which PowerShell host runs a PowerShell body, if any, is a consumer selection with off among its values (doctrine-kit/DOCTRINE.md, Policy-as-choice). The set itself, the echo, and the snapshot are rule: no knob admits an interpreter outside the harness's shells, and none turns the echo off.

The **Content-agnostic generic mechanism** paragraph's "no kit knob is added" becomes: the scratch dir comes from `GATE_SDK_TMP_DIR`, and the one kit knob is the PowerShell host.

### (3) The knob {mechanical}

**Not yet applied.**

- **The knob table.** `native/src/knobs/guard_kit.rs` gains `Row::scalar("GUARD_KIT_SCRATCH_POWERSHELL", "")`.
- **The validator** refuses a non-empty value whose program name, as `programs::name_of` reads it (the final path component, less the host's executable suffix), is neither `pwsh` nor `powershell`. The message is `GUARD_KIT_SCRATCH_POWERSHELL must be empty, or name pwsh or powershell (got '<value>')`. A refused value blocks every guarded call and refuses the runner, which is §The shell guard's loud posture for a refused knob read.
- **§Layout and configuration** gains a bullet after `GUARD_KIT_SCRIPT_INTERPRETERS`:

  > `GUARD_KIT_SCRATCH_POWERSHELL` — the PowerShell host the `--scratch-run` arm hands a `.ps1` body to, and the switch for rule `script_interpreter`'s PowerShell steer; default empty, which is off. A value is a program named `pwsh` or `powershell`, bare or as a path, and the validator refuses any other. Its readers are the arm (§scratch-run) and the rule (§The rule roster). A scalar selector on `GUARD_KIT_WORKTREE_READS`' ground: whether the runner's grant reaches PowerShell bodies is a calibration, and off leaves the command-line block standing.

- **The template.** `guard-kit/templates/guard-config.knobs` gains a commented example, `# GUARD_KIT_SCRATCH_POWERSHELL = pwsh`, beside the `GUARD_KIT_WORKTREE_READS` line.
- **The guard's knob slice.** `native/src/guard/host.rs` reads the knob into the host's fields.
- **This repo's own `scripts/guard-config.knobs`** sets nothing: this host carries no PowerShell, so the default holds.

### (4) Rule `script_interpreter` gains its PowerShell form and a PowerShell arm {design-bearing}

**Not yet applied.** In guard-kit/SPEC.md §The rule roster, item `script_interpreter`, and in `native/src/guard/rules/reach.rs` and `native/src/guard/rules/mod.rs`. The item gains `Shells \`bash\` and \`powershell\`.`, and its table row names `BOTH`. Its declarations are unchanged: the expansion decline reads through `ctx.expands`, which the `raw` declaration covers (§The generic ruleset, the expansion decline).

**A third arm, (c), the PowerShell body.** It is checked before arm (b), so a consumer that lists `pwsh` in `GUARD_KIT_SCRIPT_INTERPRETERS` still meets (c). It fires in either of two cases.

**Case 1, a PowerShell host taking its body from a scratch path**, under either reader. The head word is a PowerShell host: its final path component, split on `/` and, under PowerShell, on `\`, with a trailing `.exe` removed, is `pwsh` or `powershell`, compared without regard to case under PowerShell. The host's options are walked on its own CLI grammar, each option matched by a prefix of its name without regard to case:

- `-File` (from `-f`): the next word is the body. A `-` there means stdin.
- `-Command` (from `-c`) and `-EncodedCommand` (from `-e`): the body is carried in the command string. A `-` after `-Command` means stdin.
- Valued options skip their value: `-ExecutionPolicy` (also `-ep`), `-WorkingDirectory` (also `-wd`), `-WindowStyle`, `-OutputFormat`, `-InputFormat`, `-ConfigurationName`, `-ConfigurationFile`, `-CustomPipeName`, `-SettingsFile`, `-PSConsoleFile` and `-Version`.
- Switches are skipped: `-NoProfile`, `-NoLogo`, `-NonInteractive`, `-NoExit`, `-Interactive`, `-Login`, `-Sta`, `-Mta` and `-NoProfileLoadTime`.
- Any other option-shaped word **declines**. The host itself exits 64 on one (measured), so the decline loses no real run.
- The first bare word is the body. It is `-File`'s operand on pwsh and a command naming the file on Windows PowerShell, and either way the file is the body.
- No body word at all means stdin.

A stdin body is then resolved exactly as the bash walk resolves it, from a pipe producer's words.

**Case 2, a PowerShell invocation form**, under the PowerShell reader only. A segment's first word is:

- the call operator `&`, with the next word a scratch path;
- the dot-source operator `.` standing alone, with the next word a scratch path;
- a scratch path itself, in command position.

Or it is `Invoke-Expression` or its alias `iex` (without regard to case) in a segment that names a scratch path, or whose pipe producer does.

In both cases the scratch test is `scratch_home`'s. Under PowerShell each path word has `\` folded to `/` first, the rule `worktree_confinement` precedent, so `.\.tmp\x.ps1` reads as `./.tmp/x.ps1`.

**Arms (a) and (b) now read under PowerShell too.** `bash .tmp/x.sh` or `python .tmp/x.py` issued through the PowerShell tool takes the same verdict as through Bash.

**The steers.** Each keeps the current worktree variant (Home::Main) and the closing `!<command>` valve.

- **Arm (a)** is unchanged.
- **Arm (c) with `GUARD_KIT_SCRATCH_POWERSHELL` set** steers to the runner with the body kept as a `.ps1`: `'<runner> <script>.ps1 [args…]'`, which echoes the body and runs it under the named host.
- **Arm (c) with the knob empty** says the PowerShell scratch path is off in this project and names the knob. It steers to writing the body as a bash script under the scratch dir and running it through the runner. That call can be followed, because the hook's floor on a Windows host is Git for Windows' bash (§The hook on native Windows).
- **Arm (b)**'s message changes "scratch execution is bash-only" to "scratch execution runs in bash, or in PowerShell where the project names a host". The rest of its text is unchanged.

**The door under PowerShell.** A steer the PowerShell reader decides spells the runner as a PowerShell command: the call operator, then the door single-quoted with any `'` doubled, `& '<door>' --scratch-run`. A path carrying a blank does not run otherwise. §The shell guard's **The steer door** paragraph gains this sentence.

**What case 2 does not reach, stated in the item.**

- A scratch path inside a `-Command` string. The skeleton blanks quoted spans, the same bound the item already states for a bash `-c` body that opens a file.
- A form inside a script block or a subexpression, the reader's own limit.
- `Import-Module`, `Start-Process` launching a host with `-Wait` on a scratch body, and a body the session builds with `[scriptblock]::Create`.

Each falls through.

**Rule `bounded_wait`'s arm (B)** takes this rule's test as a predicate (`native/src/guard/rules/liveness.rs`). It is bash-only, so its only new refusal is a canonical recorded launch of a PowerShell host on a scratch body under Bash, which arm (c) would block unbackgrounded. That is the same predicate reasoning applied to a new arm. No edit is owed there beyond this paragraph's mention in §The rule roster.

### (5) The ruled classes and the native-Windows section {mechanical}

**Not yet applied.**

- **guard-kit/SPEC.md §The generic ruleset.** The two-part test's second half gains the vocabulary this rule reads: "…a roster of cmdlet names and their built-in aliases, **or the shell's own invocation grammar — the call and dot-source operators, a path in command position, and its hosts' command-line options** — which is shell-substrate knowledge this section's clause already admits." The fourth bullet of the ruled classes, "The harm rule whose PowerShell form needs mechanism the kit does not have (rule `script_interpreter`)…", is deleted, since no rule is left in that class.
- **guard-kit/SPEC.md §The hook on native Windows, *What a PowerShell call meets*.** The **honest limit** sentence becomes: "A PowerShell script run off a scratch-dir body meets rule `script_interpreter`, whose steer names the runner's PowerShell path where the project names a host (`GUARD_KIT_SCRATCH_POWERSHELL`) and a bash body under the runner where it names none (§scratch-run)."
- **The same section's *The oracle*** gains: "and the leg runs the scratch runner's seam cases under both of that host's PowerShells, so the PowerShell path's first run is on the substrate it serves."

### (6) The tests {mechanical}

**Not yet applied.**

- **`guard-kit/guard-tests/powershell-cases.tsv`** gains a `# rule \`script_interpreter\`` block, run under the table's default knobs, so the knob is empty:
  - `block`:
    - `& .tmp/x.ps1`
    - `.\.tmp\x.ps1`
    - `. .tmp/x.ps1`
    - `.tmp/x.ps1 a`
    - `pwsh -File .tmp/x.ps1`
    - `pwsh -nop -f .tmp\x.ps1`
    - `powershell -NoProfile -ExecutionPolicy Bypass .tmp/x.ps1`
    - `Get-Content -Raw .tmp/x.ps1 | iex`
    - `bash .tmp/x.sh`
    - `python3 .tmp/x.py`
  - `fallthrough`:
    - `& tools/build.ps1`
    - `pwsh -File tools/build.ps1`
    - `pwsh -Command 'Get-Date'`
    - `pwsh -Bogus .tmp/x.ps1` (the decline)
    - `& ./gate-sdk/bin/run-gates.sh --scratch-run .tmp/x.ps1` (the body is the runner)
- **`guard-kit/guard-tests/cases.tsv`** gains:
  - `block`: `pwsh -File .tmp/x.ps1`, `pwsh .tmp/x.ps1`, `powershell.exe -f .tmp/x.ps1`
  - `fallthrough`: `pwsh -File tools/x.ps1`, `pwsh -c 'Get-Date'`

  Every existing row whose command carries `pwsh` or `powershell` has its expected column re-derived under §Testing's non-monotone rule. `git grep -n "pwsh\|powershell" guard-kit/guard-tests/cases.tsv` returned none at authoring.
- **`guard-kit/gate-tests/guard-config-knobs.test.sh`**:
  - Under a knob file setting `GUARD_KIT_SCRATCH_POWERSHELL = pwsh`, a PowerShell `& .tmp/x.ps1` blocks with the runner steer spelled `& '<door>' --scratch-run`. The knob-off text is absent.
  - The same payload with no knob blocks with the knob-off steer naming the knob.
  - A knob file setting it to `python3` blocks every call with the validator's text.
- **`guard-kit/gate-tests/worktree-confinement.test.sh`** gains one case: a PowerShell `& <main>/.tmp/x.ps1` from the worktree blocks with the corrective naming the worktree's own scratch dir.
- **`guard-kit/gate-tests/scratch-run.test.sh`** gains the following cases.
  - **The window** (delta 1). It uses the authoring probe's body: a first line that rewrites a line past bash's read buffer in its own target. It asserts that the echoed line's output appears and the rewritten line's does not. It reds on today's runner.
  - **The snapshot.** A body printing `$0` shows a name other than the target, in the target's directory, and no `.`-prefixed snapshot remains in the scratch dir after a run exiting 0 or 7.
  - **The PowerShell path off.** With the knob empty, a `.ps1` is refused at exit 2, no body is printed, a side-effect file proves no child ran, and the message names the knob.
  - **The contradictions.** A `.ps1` carrying `#!/bin/bash` and a `.sh` carrying `#!/usr/bin/env pwsh` are each refused unexecuted.
  - **The host checks.** A knob naming an absent host is refused unexecuted, and `python3` is refused by the validator.
  - **Per PowerShell host on `PATH`**, `pwsh` and, where present, `powershell`, with `GUARD_KIT_SCRATCH_POWERSHELL` set in the environment:
    - echo precedes exec;
    - `alpha 'b c'` reach the script as two arguments;
    - `exit 7` passes through;
    - `$PSCommandPath` names the snapshot and not the target;
    - no snapshot remains.

    These cases print a skip line naming the missing host and pass when no PowerShell is on `PATH`, as on this host.
  - **The file runs under Git Bash.** Every path it hands the binary or a PowerShell host is spelled through `cygpath -m` when that tool is present (the liveness step's precedent). The existing `refuse-shebang` case asserts the new message text rather than `bash-only`. The file's closing clean line lists the new properties.
- **Local oracle before the push.** The PowerShell cases can be run on this host inside the `mcr.microsoft.com/powershell` image, whose pwsh was measured at authoring. Build runs them there before the mid-iteration push, through `--scratch-run` on a scratch wrapper that mounts the tree. That covers pwsh 7 on Linux. It does not cover Windows PowerShell 5.1, the Windows path dialect, or Git Bash, which only the leg in delta 7 measures.

### (7) The native-Windows witness {design-bearing}

**Not yet applied.** `.github/workflows/gates.yml`, job `install-smoke-sh-windows`, gains a binding step after `guard decision table under Git Bash`: **scratch runner under both PowerShells**, `shell: bash`, run on the artifact the normalize step placed. It runs `bash guard-kit/gate-tests/scratch-run.test.sh` with `GATE_SDK_NATIVE_BIN` set to the placed artifact. Git for Windows' bash then runs the bash path, the window case included, and the per-host cases run under both `pwsh` and Windows PowerShell 5.1, since the runner carries both. The step's comment names what it is the oracle for: the runner's two paths on the host the PowerShell path exists for.

The `gates` job's fixture-suites step on `ubuntu-latest` already runs this file. That runner carries `pwsh`, as its front-end parity step spawns it on every push, so the pwsh 7 cases also run there on the same push.

## Producers and consumers

- **The snapshot (delta 1).**
  - Producer: the arm, on every run it does not refuse.
  - Consumers: the interpreter, which reads it by path, and the runner, which removes it.
  - It carries no field. Its name is read by the executing header, which a transcript reader uses to map `$0` back to the target.
  - No roster-holding reader lists snapshot names. A stale snapshot is removed with the scratch dir's other disposable contents, so it needs no roster.
- **The executing header's `as <snapshot>` field (delta 1).** Its reader is the transcript reader, at the moment an error line names the snapshot's path. The seam case asserting the first header is unaffected.
- **The PowerShell path (delta 2).**
  - Producer: the arm, for a `.ps1` target with the knob set.
  - Enabling config: a consumer's knob file. This repo sets none, and the leg's and the seam file's environments set it, so the path is reachable in a deployed configuration the moment a consumer names a host.
  - Consumers: the named host, and the session through the passed-through exit code.
- **`GUARD_KIT_SCRATCH_POWERSHELL` (delta 3).**
  - Producer: the consumer's knob file or the environment.
  - Readers: the arm (delta 2), the rule's arm (c) steer choice (delta 4), and the validator (delta 3).
  - Roster-holding readers:
    - `--emit knob-roster`, which reads the table;
    - `check-knob-default-coupling`, which couples the table's empty default to the default the §Layout and configuration bullet states;
    - the program roster's consumer-ground scan (a unit test in `native/src/programs.rs`), which accepts `Program::consumer`'s ground only if it names a static knob, and this one does once the row lands;
    - `check-comment-tier`, which binds any new `// spec:` directive.
- **Arm (c) and the PowerShell form (delta 4).**
  - Producer: the member, on a Bash or PowerShell payload.
  - Consumers: the session, through the block and its steer, and rule `bounded_wait`'s arm (B) through the predicate.
  - `check-guard-registration` assertion C reads the item's shells clause against the table row, which is `bash` and `powershell` on both sides once delta 4 lands. Its arm D reads rule names, and no rule is added, renamed or reordered.
- **The witness step (delta 7).** Producer: the leg on every push. Consumer: the push watch, reading the `gates` workflow run.
- **Point 5.** The rule's firing set widens (arm (c), and the PowerShell reader), and its red condition is a block per call, monotone in the calls it fires on. The decision table is the instrument, and delta 6 re-derives every row that could flip. The runner's refusal set widens too. Its seam file asserts exit codes per case, never a count.
- **Point 6.** Delta 6's per-host cases oblige each PowerShell host present on the running host. The members at authoring are none on this host, `pwsh` on `ubuntu-latest`, and `pwsh` and `powershell` on the Windows runner. The satisfying values are the ones delta 6 names.

## Existing sections updated

Roster from `git grep -n "bash-only"`, `git grep -l "scratch-run\|scratch_run"`, `git grep -l "GUARD_KIT_SCRIPT_INTERPRETERS"`, `git grep -l "GUARD_KIT_WORKTREE_READS"`, a read of guard-kit/SPEC.md §The shell guard, §The hook on native Windows, §The generic ruleset, §The rule roster (`script_interpreter`, `shell_wrapper`), §scratch-run, §Layout and configuration and §Testing, and `.github/workflows/gates.yml`, run 2026-09-25.

- guard-kit/SPEC.md §scratch-run; `native/src/emit/scratch_run.rs` (deltas 1 and 2).
- `native/src/knobs/guard_kit.rs`, `native/src/guard/host.rs`, `guard-kit/templates/guard-config.knobs`, guard-kit/SPEC.md §Layout and configuration (delta 3).
- guard-kit/SPEC.md §The rule roster, item `script_interpreter`; guard-kit/SPEC.md §The shell guard, **The steer door**; `native/src/guard/rules/reach.rs`, `native/src/guard/rules/mod.rs` (delta 4).
- guard-kit/SPEC.md §The generic ruleset and §The hook on native Windows (delta 5).
- `guard-kit/guard-tests/powershell-cases.tsv`, `guard-kit/guard-tests/cases.tsv`, `guard-kit/gate-tests/scratch-run.test.sh`, `guard-kit/gate-tests/guard-config-knobs.test.sh`, `guard-kit/gate-tests/worktree-confinement.test.sh`, guard-kit/SPEC.md §Testing (delta 6).
- `.github/workflows/gates.yml`, `install-smoke-sh-windows` (delta 7).
- `.workflow/release-declarations.md`, §Behavior changes (deltas 1, 2 and 4), three bullets:
  - The `--scratch-run` arm executes a snapshot of the body it echoed, so `$0` names that snapshot.
  - `GUARD_KIT_SCRATCH_POWERSHELL`, default off, lets the arm run a `.ps1` under the named host.
  - Rule `script_interpreter` now applies to the `PowerShell` tool and to a PowerShell host under Bash, blocking a scratch-body run and steering by the knob.
- `.workflow/gap-inbox.md`: the two 2026-09-25 bullets on the runner's re-read window are drained at close as fixed by this unit (delta 1).
<!-- update-target-exempt: generated mirror, regenerated by its freshness gate's printed command, never hand-edited -->
- `docs/guard-kit/SPEC.md`.

## Retired spellings

- None — no governed name is renamed or removed. The prose phrase "bash-only" is rewritten where it states the old policy (deltas 2, 4 and 6), and it is no governed name.

## Definition of Done

- [ ] **Causal completeness.** Every point of SPEC §The causal-completeness check (canon-kit) holds for the snapshot, the PowerShell path, the knob, arm (c) and the witness step.
- [ ] **The window closed.** Delta 6's window case reds on the pre-change runner and greens after.
- [ ] **Run before pushed.** The PowerShell seam cases pass in the local pwsh container before the mid-iteration push.
- [ ] **Instruction surfaces: instruction only.** Each steer carries the spelling and the minimum reason to follow it. The grounds live in §scratch-run and the item.
- [ ] **Merged with no information lost.** Each SPEC edit re-phrases the passage it refines.
- [ ] **The witness is green and binding** on the `install-smoke-sh-windows` leg of the mid-iteration push, and again on the closing push.
- [ ] **Amendment deleted.** This file is removed on merge (`ls guard-kit/SPEC-*.md`).
- [ ] **Entry moved.** `ps-scratch-script-unsteered` moves to Done in the merge commit, at a stage before the drain stage, once the leg is observed green.
- [ ] **Removals propagated.** `check-amendment-retired-spelling` runs the block above.
- [ ] **Gaps filed.** Any cross-component gap found during the work is filed through `--emit file-gap`.
