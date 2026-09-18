# SPEC amendment: windows-follow-up

`init` ends with a follow-up block of two commands. On every host both lines are
spelled `bash gate-sdk/bin/run-gates.sh …`. A native-Windows adopter typing into
PowerShell who runs them gets the WSL launcher, not a shell: that `bash` is the
system directory's homonym. The PowerShell twin `gate-sdk/bin/run-gates.ps1` has
shipped since `windows-bash-floor`. This amendment has `init` key the block on
the host it runs on. On a Windows host, both lines invoke the twin through
Windows PowerShell. Everywhere else the block does not change. The consumer
smoke's follow-up arm takes one precise change so it can read the Windows line,
and the PowerShell leg runs the printed line in place of its own spelling.

**Probed at authoring, 2026-09-18, at `59810bb3`:**

- `native/src/installer/init.rs:737-739` prints `next:` and the two bash lines
  unconditionally, under a directive naming the block a stated grammar.
- installer/SPEC.md §init states that grammar. The banner is `next:`. An
  indented line is a command. Everything from `#` on is commentary. The target
  is the first token containing a `/`, and the tokens before it are "the
  interpreter the line spells". So a `powershell … -File <path>` line already
  parses: its interpreter tokens are `powershell -NoProfile -ExecutionPolicy
  Bypass -File`, and its target is the script path.
- `installer/consumer-smoke/run-smoke.sh:562` requires the target to satisfy
  `-f` and `-x`. `git ls-files -s gate-sdk/bin/run-gates.ps1` shows mode
  `100644`, while `run-gates.sh` is `100755`. So an unchanged arm would red the
  Windows line on any host that honours the mode bit.
- The `install-smoke-windows` leg runs the consumer smoke through the bash
  bootstrap on a Windows host (`gates.yml` step "install smoke on a native
  Windows host"), so the arm reads the Windows block there. The
  `install-smoke-powershell` leg checks only that `^next:` is present
  (`gates.yml:850`), and its hooks step spells its own
  `pwsh -NoProfile -File gate-sdk/bin/run-gates.ps1 --install-hooks`
  (`gates.yml:926`).
- `docs/install.md:476-477`, the manual-vendoring path's step 4, spells only
  the bash opt-in line.

## What changes

### (1) `init` prints the follow-up block for the host it runs on {design-bearing}

On a Windows host (`cfg!(windows)`, the host the binary was built for), the two
lines are:

```text
  powershell -NoProfile -ExecutionPolicy Bypass -File gate-sdk/bin/run-gates.ps1 --install-hooks   # opt this clone into the generated pre-commit hook
  powershell -NoProfile -ExecutionPolicy Bypass -File gate-sdk/bin/run-gates.ps1       # the battery, green on what was just vendored
```

Every other host keeps today's two bash lines byte for byte. The banner, the
reasons and the grammar do not change. The interpreter prefix and the front-end
path are one per-host pair in `init.rs`, and both lines print from it, so the two
lines cannot disagree about the host.

The spelling has four parts, each with its own ground:

- **Keyed on the OS, not on which bootstrap ran.** A Windows adopter may type
  into PowerShell or into Git Bash. A `powershell …` line runs from both, since
  `powershell.exe` is on the Windows `PATH` that Git Bash inherits. A `bash …`
  line runs only from Git Bash. Keying on the bootstrap would need each
  bootstrap to pass a marker to the binary. That would add a sixth act to the
  five-step bootstrap that installer/SPEC.md §The install boundary rules to be
  the whole of what is written twice. So it is refused. WSL resolves to the
  Linux build and keeps the bash block, which is correct for WSL.
- **`powershell`, not `pwsh`.** Windows PowerShell 5.1 is the one PowerShell a
  stock Windows host ships. PowerShell 7 is an install. This line is what makes
  the parity arm's 5.1 run (gate-sdk/SPEC.md §run-gates) a precondition of this delta.
- **`-ExecutionPolicy Bypass`.** A Windows client's default execution policy
  refuses `-File` on a script. The flag is scoped to the one process it
  launches, and it changes no machine or user policy.
- **`-NoProfile`.** An adopter's profile is not part of the command.

**Verified at align, 2026-09-19, at `902e4e1e`:** no Windows host is reachable from this session, so the named command could not be run; Microsoft's own reference (`about_Execution_Policies`) is checked instead and states it directly: "If no execution policy is set in any scope, the effective execution policy is **Restricted**, which is the default for Windows clients," and `Restricted` "[p]revents running of all script files." That confirms the parenthetical without a live host, and is the ground the flag rests on.

**Not yet applied.** In installer/SPEC.md §init, after "prints a
**follow-up block**: the commands that finish the setup, one per line, each
carrying its reason beside it.", add: "The block is keyed on the host `init`
runs on. A Windows host is told to run the PowerShell front-end through Windows
PowerShell, a spelling that runs from PowerShell and from Git Bash alike. Every
other host is told to run the bash front-end."

**Not yet applied.** docs/install.md's manual-vendoring step 4 gains a second
sentence: "On native Windows, from PowerShell:
`powershell -NoProfile -ExecutionPolicy Bypass -File gate-sdk/bin/run-gates.ps1 --install-hooks`."
There is no `init` on that path, so the page is the only thing that can say it.

### (2) The follow-up arm requires an executable target only when the line names no interpreter {design-bearing}

The arm's second assertion becomes: the target resolves to a regular file inside
the consumer, and it is executable when no token precedes it. A line that spells
an interpreter hands the interpreter a file to read, so the mode bit is not part
of what the adopter was told. A line that runs its target directly does need the
bit, and it keeps that check. The flag probe does not change: it runs the line
as printed. On the `install-smoke-windows` leg that means running
`powershell … run-gates.ps1 --install-hooks--checkwright-smoke-unknown` and then
the printed line, from bash. That is the first adopter-shaped run of the twin
under 5.1.

**Inferred, cannot run before build:** `powershell` resolves on the smoke's `RUN_PATH` on the `install-smoke-windows` leg and the twin's refusal names the sentinel flag — the leg's first run of deltas 1 and 2 settles both.

**Not yet applied.** In installer/SPEC.md §The consumer smoke, the bullet
"**The target resolves and is executable** inside the consumer the arm was
handed." becomes: "**The target resolves** to a file inside the consumer the arm
was handed, **and is executable where the line spells no interpreter before
it**: an interpreter reads the file rather than executing it, so the mode bit is
not part of the instruction."

### (3) The PowerShell leg runs the line `init` printed {design-bearing}

In `install-smoke-powershell`, step 2 of the step "commit through the generated
hooks with no bash on PATH" stops using its own spelling. It parses the
follow-up block out of `init-out.txt` (the file the previous step wrote) under
the grammar in §init. It asserts that every command's target is
`gate-sdk/bin/run-gates.ps1`. Then it runs the one line whose flags include
`--install-hooks`, splitting it into tokens and invoking them with `&`, with
every bash already stripped from `PATH`. After that the step asserts what it
asserts today: `core.hooksPath` is set, a clean commit lands with the hook's
passed line, and a violation is refused by name. A bash-spelled block would red
the step by name, because no bash is left to resolve. The block-presence check
in the previous step stays as it is.

## Producers and consumers

- **The Windows follow-up block.** The producer is `init.rs` on a Windows build.
  The consumers are the adopter, the consumer smoke's follow-up arm (on
  `install-smoke-windows`) and the PowerShell leg's hooks step (delta 3). The
  Linux and macOS smoke legs keep reading the bash block, which is unchanged.
  `grep -rn 'next:' installer/consumer-smoke native/src .github/workflows`
  finds these readers (`run-smoke.sh:546`, `gates.yml:851`). Its other hits
  are the unrelated indented `next:` lines in `enter_stage.rs` and
  `commit_msg.rs`.
- **The per-host prefix pair.** It is read by the two `println!` lines in
  `init.rs` and nothing else.
- **The interpreter-conditional executable check.** It is read only by the arm
  itself. The arm's exit status feeds the legs that run the smoke, and they are
  unchanged.

## Existing sections updated

- installer/SPEC.md §init: host keying (delta 1).
- installer/SPEC.md §The consumer smoke: the follow-up arm's second assertion
  (delta 2).
- `native/src/installer/init.rs`, and its directive, which gains the host clause
  (delta 1).
- `installer/consumer-smoke/run-smoke.sh` `assert_followups`, and its directive
  (delta 2).
- `.github/workflows/gates.yml` `install-smoke-powershell`, the hooks step and
  its header comment (delta 3).
- docs/install.md, manual-vendoring step 4 (delta 1).
- `docs/installer/SPEC.md`: the generated mirror, regenerated (all deltas).

## Retired spellings

- None — the bash follow-up spelling survives on every non-Windows host, and the leg's own `pwsh -NoProfile -File` line is replaced at its one site rather than retired as a name.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The
      causal-completeness check holds for the host-keyed block and both readers.
- [ ] **Instruction surfaces: instruction only** — the printed reasons stay
      reasons, and no grounds are added to `init`'s output.
- [ ] **Merged with no information lost** — §init and §The consumer smoke are
      re-phrased, not appended to.
- [ ] **Both Windows legs green on the printed block** — the smoke's follow-up
      line on `install-smoke-windows` and the hooks step on
      `install-smoke-powershell`.
- [ ] **Amendment deleted** — this file removed on merge; none remain for
      installer (`ls installer/SPEC-*.md`).
- [ ] **Removals propagated** — nothing retired.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed.
