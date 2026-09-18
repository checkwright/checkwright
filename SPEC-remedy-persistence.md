# SPEC amendment: remedy-persistence

docs/install.md §Requirements has two remedy blocks, one for macOS and one for
Windows, and the install-smoke legs run each one verbatim. Both blocks fall short
of the adopter the same way: a leg greens on a `PATH` state that the adopter's
next terminal does not have.

- **macOS.** The block's `export PATH=` line lasts one shell. The page says the
  block is the whole remedy. But an adopter who runs it and opens a new terminal
  resolves BSD `sort` and `date` again, and `checkwright doctor` refuses. The CI
  legs never see this, because `$GITHUB_PATH` carries the entries across steps.
- **Windows.** The block holds only `choco install shellcheck -y`. The
  PowerShell leg first prepends Git for Windows' `usr\bin` and `bin` to `PATH`
  itself, and the page names no such step. So an adopter in PowerShell meets a
  `doctor` refusal the page gives no remedy for. The block also leaves out `jq`,
  as the probe below shows.

This amendment makes each block do the whole job. It puts the ordering into the
current shell and also persists it for the next one. It gives each leg a
next-shell measurement, so persistence is measured rather than only claimed. The
macOS profile file and the Windows persistence form are the adopter-facing
choices the two queue entries left to this stage. Both are ruled here.

**Probed at authoring, 2026-09-18, at `59810bb3`:**

- `docs/install.md:84-91` is the macOS block: `brew install bash coreutils
  shellcheck`, then one `export PATH=` line. `docs/install.md:222-228` is the
  Windows block: `choco install shellcheck -y`.
- Both macOS legs extract the block and `eval` it under bash. They then write
  the entries the block prepended to `$GITHUB_PATH`
  (`grep -n macos-remedy .github/workflows/gates.yml` prints lines 1238-1243
  and 1540-1545).
- `install-smoke-powershell` sets `$env:PATH = "C:\Program Files\Git\usr\bin;C:\Program Files\Git\bin;$env:PATH"`
  at `gates.yml:793`, ahead of its `windows-remedy` extraction.
  `install-smoke-windows` hands the block to `pwsh -NoProfile -Command`
  (`gates.yml:408`) from Git Bash, where the userland already resolves.
- **Git for Windows ships no `jq`.** In run 35390249229's
  `install-smoke-windows` "probe the runner" step, Git Bash resolves `sort`,
  `awk` and `sed` from `/usr/bin` and `jq` from
  `/c/ProgramData/Chocolatey/bin/jq`. The runner image preinstalls `jq`, and
  that is what hides the gap. So the page's sentence "the one floor member Git
  for Windows does not supply" is false: `jq` is a second one.
- The crate resolves `bash` and `sort` past the Windows system directory's
  homonyms (`SYSTEM_DIR_HOMONYMS` in `native/src/proc.rs`). So a `PATH` where
  the system directory comes before Git's userland still reaches Git's `bash`
  and `sort` for every spawn and every floor probe.
- `docs/site-architecture.md:311-324` rosters both blocks and their readers,
  and states that no gate holds either block and the binding legs are its
  enforcement.

## What changes

### (1) The macOS block persists its `PATH` ordering in `~/.zprofile` {design-bearing}

The block becomes three lines:

```sh
brew install bash coreutils shellcheck
echo "export PATH=\"$(brew --prefix)/opt/coreutils/libexec/gnubin:$(brew --prefix)/bin:\$PATH\"" >> ~/.zprofile
export PATH="$(brew --prefix)/opt/coreutils/libexec/gnubin:$(brew --prefix)/bin:$PATH"
```

- **`~/.zprofile`.** zsh is macOS's default login shell, and a new Terminal
  window is a login shell that reads that file. It is also where Homebrew's own
  installer tells the adopter to put its `shellenv` line. An adopter whose login
  shell is bash uses `~/.bash_profile` instead. The prose says so in one
  sentence, and the block does not branch.
- **The prefix is resolved when the line is appended, and `$PATH` when the
  profile is read.** So the appended line runs even where the new shell cannot
  find Homebrew, as on a Mac whose profile never ran `brew shellenv`.
- **The `export` line stays.** The profile covers the next shell and the
  `export` covers this one, so the block still finishes with a working shell.
- **Re-running it appends a duplicate line.** A repeated `PATH` entry is
  harmless, and a guard would cost a conditional in a block adopters type by
  hand, so the block carries none.

### (2) The Windows block installs `jq` and puts Git's userland on `PATH`, for this session and the next {design-bearing}

The block becomes:

```powershell
choco install shellcheck jq -y
$git = Split-Path (Split-Path (Split-Path ((git --exec-path) -replace '/', '\')))
[Environment]::SetEnvironmentVariable('Path', [Environment]::GetEnvironmentVariable('Path', 'User') + ";$git\usr\bin;$git\bin", 'User')
$env:PATH = "$git\usr\bin;$git\bin;$env:PATH"
```

- **`jq` joins `shellcheck`.** These are the two floor members Git for Windows
  does not supply.
- **Git's location is derived from `git --exec-path`, never spelled.** The
  install-smoke-powershell hooks step already derives it this way. A per-user
  Git install does not live under `C:\Program Files`.
- **Persisted by appending to the user `Path`, and prepended for the session.**
  Windows builds a new process's `PATH` as the machine value followed by the
  user value, so a user entry can never come before the system directory. That
  makes appending the benign form: `cmd`'s own `sort` and `find` stay first for
  the adopter's other tools. The crate's homonym resolution is what still
  reaches Git's `bash` and `sort` past them. The session line prepends, which is
  the ordering the leg measures today.
- **No administrator rights.** The user scope needs none. Chocolatey's install
  still does, as it does today.

**Inferred, not run:** Git for Windows' default installer choice puts only its `cmd` directory on `PATH`, so PowerShell resolves none of Git's userland — `$env:PATH -split ';' | Select-String 'Git'` in PowerShell on a stock Windows host after a default Git for Windows install.

**Inferred, cannot run before build:** `doctor` exits 0 in a process whose `PATH` is the machine value followed by the user value this block wrote, with nothing else prepended — delta 3's next-shell measurement on `install-smoke-powershell` is the run.

### (3) The legs run the blocks verbatim and measure the next shell {design-bearing}

- **Both macOS legs** keep their extraction, `eval` and `$GITHUB_PATH` step. They
  add one assertion after it: `env -i HOME="$HOME" /bin/zsh -l -c 'command -v sort'`
  must resolve under `…/coreutils/libexec/gnubin`. That is a login shell with
  no inherited `PATH`, which is what an adopter's next terminal is. A miss reds
  the step by name.
- **`install-smoke-powershell`** deletes its own `$env:PATH =` prepend at
  `gates.yml:793`. The block's session line takes its place, and the comment
  above it is rewritten to say so. Before `doctor` runs in the current session,
  the step runs `doctor` once more in a child `pwsh` whose `PATH` is set to
  `[Environment]::GetEnvironmentVariable('Path','Machine') + ';' + [Environment]::GetEnvironmentVariable('Path','User')`.
  That child is the next terminal. Its exit status is asserted with `Check`,
  and its output is printed on a miss.
- **`install-smoke-windows`** needs no change. It runs the whole block, and the
  block's `PATH` lines are harmless in its Git Bash session.

**Inferred, cannot run before build:** `env -i … /bin/zsh -l` on both macOS runner images reads `~/.zprofile` and resolves gnubin `sort` — the leg's first run of this delta.

### (4) The page's prose follows the blocks {mechanical}

**Not yet applied.** In docs/install.md §Requirements:

- "The remedy is the two commands below. They are the whole of it: this page
  names no other step for a Mac." becomes "The remedy is the block below, and
  it is the whole of it: this page names no other step for a Mac."
- The paragraph after the macOS block becomes: "The block's middle line writes
  the ordering into `~/.zprofile`, the profile a new zsh Terminal window reads.
  If your login shell is bash, append the same line to `~/.bash_profile`
  instead. The last line orders the shell you are in. The requirements below
  assert what `PATH` actually resolves, so a Mac carrying Homebrew coreutils
  that is not `PATH`-ordered reports below contract. That is correct, since BSD
  `sort` is what the gates would invoke. The two macOS install-smoke legs run
  this block verbatim and then open a fresh login shell, so both the ordering
  and its persistence are measured rather than suggested."
- "The Windows remedy block: the one floor member Git for Windows does not
  supply, installed from PowerShell." becomes "The Windows remedy block, typed
  into PowerShell. It installs the two floor members Git for Windows does not
  supply, `shellcheck` and `jq`. It then puts Git's own `usr\bin` and `bin` on
  `PATH`, for this session and, through your user `Path`, for every later one:
  Git's installer puts only its `cmd` directory there by default, and that
  directory holds no userland."
- One sentence after the Windows block: "These lines change your machine, not
  your repository, so `checkwright uninstall` does not reverse them, just as it
  does not uninstall a Homebrew package."

**Not yet applied.** In docs/site-architecture.md's remedy-blocks bullet, the
macOS readers' sentence gains ", then assert a fresh login shell resolves the
ordering", and the Windows readers' sentence gains "; the pwsh leg then runs
`doctor` under the `PATH` a new terminal would compose".

## Producers and consumers

- **The `~/.zprofile` line.** The producer is the adopter or the leg running
  the block. The consumer is the next login zsh. On CI the reader is delta 3's
  `env -i` assertion, and nothing in the payload reads the file.
- **The user `Path` entry.** The producer is the block. The consumer is every
  new Windows process, and on CI the reader is delta 3's child `doctor`. The
  `install-smoke-windows` run writes the same entry on its own runner, and
  nothing reads it there.
- **`jq` from Chocolatey.** The consumer is `doctor`'s floor verdict and the
  jq-using gates. It is already on the runner, so the leg's `choco` line is a
  no-op there, and the page is the delta that matters.

## Existing sections updated

- docs/install.md §Requirements: both blocks (deltas 1 and 2) and their prose
  (delta 4).
- `.github/workflows/gates.yml`: both macOS extraction steps (lines 1234 and
  1536), plus the `install-smoke-powershell` prepend, its comment block
  (lines 772-793) and a next-terminal `doctor` (delta 3).
- docs/site-architecture.md: the remedy-blocks bullet (delta 4).

## Retired spellings

- None — no name is retired; the leg's literal `C:\Program Files\Git` prepend is deleted at its one workflow site (`git grep -n 'Program Files\\Git\\usr'` prints `gates.yml:793` and three `proc.rs` test fixtures, which model a Windows `PATH` and stay).

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The
      causal-completeness check holds for both persisted states and their readers.
- [ ] **Instruction surfaces: instruction only** — the blocks carry commands
      and no commentary.
- [ ] **Merged with no information lost** — the page's prose is re-phrased, not
      appended to.
- [ ] **All three binding legs green**, with the next-shell assertions printed.
- [ ] **Amendment deleted** — this file removed on merge; no root `SPEC-*.md`
      remains for this unit.
- [ ] **Removals propagated** — nothing retired.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed.
