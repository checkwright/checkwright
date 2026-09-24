# SPEC amendment: ps-producer-record

A producer the harness backgrounds through its `PowerShell` tool writes no liveness record, so guard-kit rule `git_mutation_under_producer` cannot see it and a tracked-tree mutation beside it goes unguarded. Guard-kit rule `background_no_record` refuses an unrecorded launch on bash only (guard-kit/SPEC.md §The hook on native Windows, the honest limit). Its PowerShell form was held back for one unmeasured fact. A PowerShell launch names a **Windows** process id, and on a native Windows build the liveness predicate asks Git Bash's `kill -0` builtin and then `ps -p` (`native/src/evidence.rs`, the non-unix `signal_zero`). Both of those resolve MSYS pids. This amendment measures that fact on a Windows runner, builds whichever predicate the measurement calls for, and then builds the rule's PowerShell form.

**The component span, judged against the owners.** The rule and its tables are guard-kit's. The predicate and the record grammar are evidence-kit's (evidence-kit/SPEC.md §The producer-liveness lock). The route the predicate takes on a non-unix build is stated in gate-sdk/SPEC.md §Fail-closed contract, and its FFI admission in §The settings cohort, and the crate's first dependency. evidence-kit's and gate-sdk's text is edited on **both** branches of delta 2: on the green branch it gains a measured scope, and on the red branch a new leg. So this unit spans guard-kit, evidence-kit and gate-sdk, plus one parenthetical in delegation-kit (delta 6) and the Windows leg in `.github/workflows/gates.yml`. It is sited at the repo root for that reason.

**What was run at authoring** (2026-09-24, at `8488e867`):

- `printf 'pid=1 run=probe\r\n'` into a record, then `bash gate-sdk/bin/run-gates.sh --only check-producer-liveness -- <record>`: **exit 2**, `carries no readable 'pid=<n> run=<key>' record`. A CRLF-terminated record is corrupt under the grammar, because the run key then ends in `\r`. So a record PowerShell writes with its default line ending is unreadable, and the canonical spelling in delta 3 writes a bare LF.
- The Cygwin user guide's `ps` and `kill` pages, and the Cygwin commit that stopped `kill(1)` taking a raw Windows pid: `kill` and `ps -p` take Cygwin pids, and a non-Cygwin process appears only under `ps -W`. That is documentation, not a measurement on Git for Windows' MSYS runtime:

**Inferred, cannot run before build:** that the predicate reads a live Windows process id of a process not started from Git Bash as gone, a false free — no Windows host exists here, and delta 1's step is the measurement.

## What changes

### (1) The measurement: a reporting step on the native-Windows leg {design-bearing}

**Not yet applied.** Every round costs a push and there is no local oracle, so the step has to be right the first time. `install-smoke-sh-windows` gains a step after `guard decision table under Git Bash`: **producer liveness over Windows process ids (reporting)**, run under `shell: bash` on the artifact the normalize step placed.

The step makes a fresh record directory under `$SMOKE_TMP`. For each row it reads a verdict with `bash gate-sdk/bin/run-gates.sh --only check-producer-liveness -- <record>` and a **ground truth** with `pwsh -NoProfile -Command "if (Get-Process -Id <n> -ErrorAction SilentlyContinue) { 'alive' } else { 'gone' }"`. It reads both while the process runs and again after it is stopped (`Stop-Process -Id <n>` and a wait on the ground truth reading `gone`). The rows:

- **(a) An MSYS pid.** `sleep 300 &` under Git Bash, recorded `pid=$! run=msys`. This is the shipped path, never before witnessed on this host.
- **(b) A pwsh self-record.** A backgrounded `pwsh -NoProfile -Command '<delta 3 record write>; Start-Sleep 300'`. The record holds that pwsh's own `$PID`, written by delta 3's canonical spelling. The step waits for the record file before reading.
- **(b′) The same row under `powershell.exe`**, Windows PowerShell 5.1.
- **(c) A detached child.** `pwsh -NoProfile -Command '(Start-Process pwsh -ArgumentList "-NoProfile","-Command","Start-Sleep 300" -PassThru).Id'`, recorded by `printf` with a bare LF.
- **(d) PowerShell's default writes.** A record written with plain `Set-Content` and one written with `>`, each under pwsh and under 5.1, naming a live pid. The expected reading is exit 2 (corrupt), because of CRLF, and on 5.1's `>` also UTF-16.
- **(e) A job's lifetime.** `pwsh -NoProfile -Command 'Start-Job { Set-Content -NoNewline job.pid "$PID"; Start-Sleep 300 } | Out-Null; Start-Sleep 10'`. After that pwsh exits, the step reads the ground truth for the pid in `job.pid`.

Each row prints `<row> truth=<alive|gone> gate=<exit>` at both readings. The step then prints one verdict line. It says `predicate answers Windows pids` when rows (b), (b′) and (c) read exit 1 while alive and exit 0 once gone, and `predicate misreads Windows pids` otherwise. It prints row (a)'s and row (e)'s outcomes on lines of their own. It exits 0 whatever it finds.

This step lands and is pushed before deltas 2 to 6 are cut. Its run is read with `gh run view <id> --log`, and deltas 2 and 3 take the branch the verdict line names. The debt unit `install-ps1-octet-under-ps51` also needs a probe on a Windows run, so both probes can ride one push.

A wrong row (a) is a defect in the shipped bash path on Windows, outside this unit. Build files it through `--emit file-gap` and escalates it to the lead before cutting delta 2.

### (2) The predicate answers a Windows process id {design-bearing}

**Not yet applied.** Two acts, and delta 1's verdict line picks one.

**If it reads `predicate answers Windows pids`**, no code changes. The measured scope lands in the owners' prose:

- In evidence-kit/SPEC.md §The producer-liveness lock, the sentence "On a non-unix build it reaches the shell's `kill -0` builtin, whose exit status conflates the two, so `ps -p` runs as the fallback" gains: "and the two legs answer a Windows process id from a PowerShell launch as they answer an MSYS one, which the native-Windows leg's liveness step witnesses".
- In gate-sdk/SPEC.md §Fail-closed contract, "On a non-unix build the pids are an MSYS shell's, which only that shell's builtin resolves" becomes "On a non-unix build the pids are an MSYS shell's or, from a PowerShell launch, Windows's, and the builtin with `ps -p` behind it answers both".

**If it reads `predicate misreads Windows pids`**, the non-unix predicate gains a **native leg**, run first. It is compiled under `cfg(windows)` in `native/src/evidence.rs`:

- `OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, FALSE, pid)`. When the process opens, an exit code of `STILL_ACTIVE` from `GetExitCodeProcess` means **held**, and any other exit code means this leg reads gone. The opened process is closed with `CloseHandle` before return.
- When the open fails, an error of `ERROR_ACCESS_DENIED` from `GetLastError` means **held**, which is the `EPERM` reading carried over. Any other error means this leg reads gone.
- When this leg reads gone, the existing legs run unchanged (`kill -0`, then `ps -p` with its absent-`ps` refusal), because an MSYS pid is not a Windows pid. A pid is **held when any leg says held.**

The three functions are declared in one `extern "system"` block against `kernel32`. That adds no crate, since std already links `kernel32` on this target. The `unsafe` calls take a `u32` pid, a stack-local `u32` exit code and a handle the leg closes itself. It spawns nothing, so the non-unix row's declared set (`bash`, `ps`) is unchanged.

The prose edits on this branch:

- **evidence-kit/SPEC.md §The producer-liveness lock.** The predicate paragraph states the native leg first, then the two MSYS legs behind it, and the any-leg-held rule. The absent-`ps` paragraph says `ps` is reached only when the native leg and `kill -0` both read gone. The PID-reuse paragraph gains one case with the same direction and clearance: an MSYS pid that numerically matches a live Windows process reads held.
- **evidence-kit/SPEC.md §check-producer-liveness,** the non-unix wrapper paragraph: the native leg spawns nothing, so the declared set is unchanged.
- **gate-sdk/SPEC.md §Fail-closed contract,** the per-platform route paragraph: on Windows the route opens with the native leg, and the builtin and `ps -p` answer the MSYS namespace behind it.
- **gate-sdk/SPEC.md §The settings cohort, and the crate's first dependency,** the `libc` paragraph. Its closing sentence ("the native Windows build resolves pids in a shell's namespace, where the crate's `kill` would read the wrong processes") gains: "and the Windows leg that answers a Windows pid is three `kernel32` functions declared in-crate, which admits no dependency".
- The `// spec:` directive above `pid_alive` names the native leg.

A `#[cfg(windows)]` crate test asserts the running process's own id reads held and `2147483646` reads gone. `cargo check --target x86_64-pc-windows-msvc` in the `gates` job compile-checks the leg on every push. Delta 1's step, made binding by delta 5, is its runtime oracle.

### (3) Rule `background_no_record` gains its PowerShell form {design-bearing}

**Not yet applied.** In guard-kit/SPEC.md §The rule roster, the item gains "Shells `bash` and `powershell`.", and its row in `native/src/guard/rules/mod.rs` names both. The bash reading is unchanged. The item gains a paragraph for the PowerShell reading, whose terms are these:

- **The backgrounding forms.**
  - The **harness form**: `.tool_input.run_in_background` is `true` on the PowerShell payload, the same field the bash arm reads.
  - The **shell form**: a segment of the PowerShell reader's compound split whose first word is `Start-Process`, `saps` or `start`, matched without regard to case. The segment must carry no `-Wait` parameter (by any prefix from `-wa`) and no `-WhatIf` (by any prefix from `-wh`). Such a launch starts a process that outlives the call.
  - A **job** is not a backgrounding form: `Start-Job`, `Start-ThreadJob`, or PowerShell 7's trailing `&`. A job belongs to its session and is stopped when the call's process exits. This is **conditional on delta 1's row (e)**. If row (e) reads the job's process `alive` after its session exited, the shell form also names `Start-Job` and its alias `sajb`, and a segment ending in a bare `&`. Delta 4 then gains their rows.
- **Exemption (1), the call writes a record.** It holds only for the canonical record write: a `Set-Content` segment carrying `-NoNewline` (or any prefix of it from `-n`, its shortest unambiguous one), whose `-Path` or `-LiteralPath` or first positional word is a literal `.run` path under a `GUARD_KIT_SCRATCH_DIRS` member (`\` folded to `/`), and whose `-Value` word is an expandable string ending in `` `n ``. A redirect or a plain `Set-Content` does not count, because it writes a CRLF record (and on 5.1 a `>` writes UTF-16), and the grammar reads either as corrupt. Rule `git_mutation_under_producer` declines on a corrupt record, so the mutation it exists to hold would pass.
- **Exemption (2), the call is a wait loop.** The whole `sq dq hd` skeleton, trimmed, is one loop statement: `while (<cond>) { <sleep> }`, or `do { <sleep> } while (<cond>)`, or `do { <sleep> } until (<cond>)`. `<sleep>` is a single statement led by `Start-Sleep` or `sleep`, and the keywords are matched without regard to case. The bash exemption's ground holds unchanged: a waiter's own record could falsify a condition that reads the record set.
- **Exemption (3), the read-only pipeline, has no PowerShell reading.** Its test is a roster of bash utilities, and a PowerShell counterpart is unmeasured (§The generic ruleset, the ruled classes). So a backgrounded read-only PowerShell call takes the record like any launch, at the cost of one write.
- **The `$` decline is narrowed.** Every record-writing spelling carries `$PID` or a `.Id` in its value by construction. So under PowerShell the rule does not decline on a `$` in the command at large (§The generic ruleset, the expansion decline). It declines only on a `$` inside a record write's path word, whose target it cannot resolve.
- **The corrective.** It names the canonical harness-form spelling:

  `Set-Content -NoNewline -Path <scratch-dir>/<key>.run -Value "pid=$PID run=<key>`n"; try { <command> } finally { Remove-Item <scratch-dir>/<key>.run }`

  It says that `$PID` is the process running the command, so the record reads held exactly while the command runs. It says the `-NoNewline` and the trailing `` `n `` are what make the record parse. It says a `Start-Process` launch is given `-Wait` inside that spelling rather than left detached, and that an inline wait loop owes no record. It also says no guard grant applies under PowerShell, so the spelling costs one permission decision.
- **The honest limit, stated in both directions.** `finally` removes the record on a normal exit and on a terminating error. A call killed mid-run leaves it behind. If each call's PowerShell process ends with the call, the record then names a dead pid and is inert. If the harness keeps one PowerShell host across calls, the record names a live pid and reads held until it is deleted. That is the fail-closed direction, cleared by the rule's own corrective. Nothing in this unit measures which process model the harness uses. The spelling is chosen because neither answer can make it read free while the command runs.

Rule `git_mutation_under_producer` needs no edit: it already applies to PowerShell and reads any record through the shared predicate.

### (4) The decision tables {mechanical}

**Not yet applied.**

- **A fifth table,** `guard-kit/guard-tests/powershell-background-cases.tsv`. It takes `background-cases.tsv`'s grammar and is fed as PowerShell payloads. `native/src/emit/run_guard_tests.rs` reads it as positional 3 and keeps it on disk. Its rows:
  - `true`, `pwsh -File tools/build.ps1` → block;
  - `true`, the canonical spelling around that command → fallthrough;
  - `true`, the same without `-NoNewline` → block;
  - `true`, a record written with `>` → block;
  - `true`, the canonical write to `tracked.run` outside the scratch dir → block;
  - `true`, the canonical write whose path carries `$k` → fallthrough;
  - `true`, `while (Test-Path .tmp/m) { Start-Sleep 5 }` → fallthrough;
  - `true`, `while (Test-Path .tmp/m) { Remove-Item x; Start-Sleep 5 }` → block;
  - `false`, `pwsh -File tools/build.ps1` → fallthrough.
- **`powershell-cases.tsv` gains the shell form:**
  - `Start-Process pwsh -ArgumentList x` and `start notepad` → block;
  - `Start-Process -Wait pwsh`, `Start-Process -wa pwsh` and `Start-Process pwsh -WhatIf` → fallthrough.
  - Every existing row carrying `Start-Process`, `start` or `saps` has its expected column re-derived under §Testing's non-monotone rule.
- **guard-kit/SPEC.md §Testing:** a paragraph for the fifth table on the third table's precedent, which differs from the third only in the payload's `tool_name`. The payload bullet's "`Bash` on every row but the fourth table's and `PowerShell` on those" becomes "…but the fourth and fifth tables' …". §Layout and configuration's tree lists the file.

### (5) The witness turns binding {mechanical}

**Not yet applied,** in the batch that lands deltas 2 and 3. Delta 1's step drops `(reporting)` from its name and asserts four things:

- rows (b), (b′) and (c) read exit 1 while the ground truth is `alive` and exit 0 once it is `gone`;
- row (a) reads the same way;
- row (d) reads exit 2;
- row (e) reads whichever way delta 3's conditional took.

On a misread it exits 1. Its comment names what it is the oracle for: this host's predicate over both pid namespaces, and the record delta 3's corrective spells.

guard-kit/SPEC.md §The hook on native Windows, **The oracle**, gains: "and the leg's liveness step reads a record naming a Windows process id through the predicate on this host, so the PowerShell form of rule `background_no_record` steers to a record something can judge".

### (6) The prose retires the rule's half of the limit {mechanical}

**Not yet applied.**

- **guard-kit/SPEC.md §The hook on native Windows,** *What a PowerShell call meets*: the honest limit keeps rule `script_interpreter` alone. Its `background_no_record` sentence is deleted, since a PowerShell launch now meets that rule.
- **guard-kit/SPEC.md §The generic ruleset,** the ruled classes' last bullet: "The harm rules whose PowerShell form needs mechanism the kit does not have (rules `background_no_record` and `script_interpreter`)" becomes "The harm rule whose PowerShell form needs mechanism the kit does not have (rule `script_interpreter`)".
- **delegation-kit/SPEC.md §The delegation model,** the sentence "guard-kit rule `bounded_wait` grants the one canonical recorded launch of an allowlisted command, so the compliant spelling costs no permission decision" gains "under the Bash tool; a PowerShell launch meets the refusal and takes no grant (guard-kit/SPEC.md §The generic ruleset)".

## Producers and consumers

- **The liveness step (deltas 1 and 5).**
  - Producer: the `install-smoke-sh-windows` job on every push. Its enabling config is the workflow on master.
  - Consumers: build, which reads the verdict line to cut deltas 2 and 3, and after delta 5 the job's status.
  - Every printed field has a reader. `truth` and `gate` make up the verdict. Row (a)'s line routes the escalation, and row (e)'s line routes delta 3's conditional.
- **The native leg (delta 2, red branch).**
  - Producer: `pid_alive` on a Windows build.
  - Consumers: every caller of the shared predicate, all unchanged: `check-producer-liveness` in both modes, guard-kit rule `git_mutation_under_producer`, the `SubagentStop` turn-end hook through set mode (delegation-kit/SPEC.md §The turn-end liveness hook), the `--run-validate` writer and `--enter-stage`'s worktree classifier.
- **The PowerShell form (delta 3).**
  - Producer: the member, on a PowerShell payload.
  - Consumers: the session, through the block and its corrective; rule `git_mutation_under_producer`, through the record the corrective writes.
  - Roster readers:
    - `check-guard-registration` assertion C, which checks that the item's shells equal the table's (`bash` and `powershell` on both sides once delta 3 lands);
    - the `--run-guard-tests` table list, which gains the fifth table;
    - `check-comment-tier`, where any new `// spec:` directive binds.
  - No rule is added or reordered, so assertion B is unchanged.
- **The canonical PowerShell record write.**
  - Producer: a session following the corrective.
  - Consumers: the three record readers through `lock_read`, which requires a bare LF.
  - The two fields are `pid`, read by the predicate, and `run`, printed by the gate. Both are the grammar's own, and nothing is added.
- **Point 5.** The `$` decline narrows the rule's decline set under PowerShell only. The rule's red condition is a block per call, monotone in the calls it fires on, and the decision table is the instrument for it (§Testing).
- **Point 6.** Delta 5 obliges each row of the step. Each row's satisfying value is named in delta 5.

## Existing sections updated

Roster from `git grep -n "background_no_record"`, `git grep -n 'pid=\$!'`, `git grep -n "kill -0\|ps -p" -- gate-sdk/SPEC.md delegation-kit/SPEC.md lifecycle-kit/SPEC.md installer/SPEC.md`, `grep -n "signal_zero" -r native/src` and a read of guard-kit/SPEC.md §The hook on native Windows, §The generic ruleset and §Testing, run 2026-09-24.

- `.github/workflows/gates.yml`, `install-smoke-sh-windows` (deltas 1 and 5).
- evidence-kit/SPEC.md §The producer-liveness lock and §check-producer-liveness; gate-sdk/SPEC.md §Fail-closed contract and §The settings cohort, and the crate's first dependency; `native/src/evidence.rs` (delta 2).
- guard-kit/SPEC.md §The rule roster, item `background_no_record`; `native/src/guard/rules/liveness.rs`, `native/src/guard/rules/mod.rs` (delta 3).
- `guard-kit/guard-tests/powershell-background-cases.tsv`, `guard-kit/guard-tests/powershell-cases.tsv`, `native/src/emit/run_guard_tests.rs`, guard-kit/SPEC.md §Testing and §Layout and configuration (delta 4).
- guard-kit/SPEC.md §The hook on native Windows, **The oracle** (delta 5).
- guard-kit/SPEC.md §The hook on native Windows and §The generic ruleset; delegation-kit/SPEC.md §The delegation model (delta 6).
- `TASK-QUEUE.md`: `ps-scratch-script-unsteered`'s owner-lookup link to this entry's anchor is repointed to guard-kit/SPEC.md §The generic ruleset in the commit that moves this entry to Done (delta 6).
- `.workflow/release-declarations.md` (delta 3), a Behavior changes bullet: `guard-kit` rule `background_no_record` now applies to the `PowerShell` tool. A PowerShell call backgrounded by the harness, or a detached `Start-Process`, is blocked unless it writes the LF-terminated record the corrective spells; a plain `Set-Content` or `>` record reads as corrupt.
<!-- update-target-exempt: generated mirrors, regenerated by their freshness gate's printed command, never hand-edited -->
- `docs/guard-kit/SPEC.md`, `docs/evidence-kit/SPEC.md`, `docs/gate-sdk/SPEC.md`, `docs/delegation-kit/SPEC.md`.

## Retired spellings

- None — no governed name is renamed or removed. Delta 6 deletes one prose sentence about rule `background_no_record`, whose name stays in use.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The causal-completeness check holds for the step, the native leg (if taken), the PowerShell form and the record write.
- [ ] **Measured before built.** Delta 1's run is read, and the branch taken by deltas 2 and 3 is named in the landing commit's body with the run id.
- [ ] **Instruction surfaces: instruction only.** The corrective carries the spelling and the minimum reason to follow it. The grounds live in the item's paragraph.
- [ ] **Merged with no information lost.** Each SPEC edit re-phrases the passage it refines.
- [ ] **The witness is green and binding** on the push that lands deltas 2 to 6.
- [ ] **Amendment deleted.** This file is removed on merge (`ls SPEC-*.md`).
- [ ] **Entry moved.** `ps-producer-liveness-record` moves to Done in the merge commit, at a stage before the drain stage, once the binding run is observed green.
- [ ] **Removals propagated.** `check-amendment-retired-spelling` runs the block above.
- [ ] **Gaps filed.** Row (a)'s defect if it reds, and any cross-component gap found during the work.
