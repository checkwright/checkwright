---
title: Install
nav_id: install
nav_order: 3
nav_suffix: spec=installer/SPEC.md plugin=plugin/SPEC.md
---

# Install and upgrade

This page gets the kits into your repository, keeps them current, and takes them out again. Checkwright is vendored: `init` copies the kit source into your tree and commits it, so what governs your repository is committed and reviewable. The gate binary is the one compiled piece, and it is checked against a published digest before anything runs. The [footprint page](footprint.md) prices each kit in agent context.

## Requirements

An install needs a supported system and a few tools, and [What an install requires](requirements.md) lists both.

## Install

<!-- install-primary: tarball -->

Run the one line for your system from your repository's root, in a new project or an existing one with every change committed: `init` makes one commit and refuses a dirty worktree. The line downloads the newest Release tarball, checks it against its published digest and, where `gh` is signed in and the release is attested, against its build attestation, unpacks it outside your repository and runs `init`, with no runtime to install first. A step-by-step form is linked under each line.

Pick your route:

- **macOS or Linux:** [the one line and its steps](#macos-and-linux).
- **Windows:** [natively, in PowerShell](#windows).
- **Windows through Windows Subsystem for Linux (WSL):** the [macOS and Linux](#macos-and-linux) line, in your WSL shell. The install places the Linux gate binary and the pre-commit hook runs it, so commit to that repository from WSL too: Git for Windows cannot start the Linux binary.
- **With Node:** [`npx checkwright init`](#with-node).
- **In Claude Code:** [the plugin marketplace](#from-a-plugin-marketplace).
- **With another coding agent:** [a prompt to give it](#with-another-coding-agent).

### Try it first

`demo` in place of `init` runs the whole arc in a scratch repository of its own and removes it, without touching yours: it installs the `full` profile, runs the battery green, commits a task marked done with no evidence behind it and shows the claim caught, then withdraws the claim and runs green again. Its spellings are `sh -s -- demo` on the macOS and Linux line or the script-block form with `demo` on Windows; with Node, `npx checkwright demo`. `full` owes `bash` 4.3 or later ([Requirements](#requirements)). On stock macOS run the Homebrew bash block below first; on Windows, the `PATH` block.

### macOS and Linux

**You need** the tools [Requirements](#requirements) lists for installing on Linux and macOS.

On macOS, stock bash is 3.2, below the floor. If your profile owes `bash` (Requirements), run this block, which installs Homebrew's bash and puts it first on your `PATH`, now and in `~/.zprofile`. If your login shell is bash, use `~/.bash_profile` instead. The floor's grounds are in [installer/SPEC.md](installer/SPEC.md#requirements) and context-kit's [env-probe](context-kit/SPEC.md#binenv-probe).

<!-- macos-remedy:begin -->

```sh
brew install bash \
  && echo "export PATH=\"$(brew --prefix)/bin:\$PATH\"" >> ~/.zprofile \
  && export PATH="$(brew --prefix)/bin:$PATH"
```

<!-- macos-remedy:end -->

Then, from your repository root:

```sh
curl -fsSL https://checkwright.dev/install.sh | sh
```

The script is `docs/install.sh` in this repository; read it at <https://checkwright.dev/install.sh> before you pipe it. To pass arguments, name the verb first, since the line runs `init` only when given none: end the line with `sh -s -- init --profile prose`, `sh -s -- demo` or `sh -s -- uninstall`.

The same install, one step at a time: [Installing step by step on macOS and Linux](manual-install.md#macos-and-linux).

### Windows

This is the native Windows route. Under WSL, take [macOS and Linux](#macos-and-linux).

**You need** the tools [Requirements](#requirements) lists for installing on Windows, where the minimum Windows version is stated too. Git for Windows' bash is also what runs any shell gate you write ([Writing your own shell gates](requirements.md#writing-your-own-shell-gates)).

The pre-commit hook needs no shell: it is the gate binary under the hook's name, and git starts it directly. The battery needs none either: run the gate binary with `--run`, or `gate-sdk/bin/run-gates.ps1` from PowerShell.

Run this block in PowerShell first. It puts Git's `usr\bin` and `bin` on your `PATH`, for this session and every later one:

<!-- windows-remedy:begin -->

```powershell
. {
  $exec = if (Get-Command git -ErrorAction SilentlyContinue) { git --exec-path }
  if (-not $exec -or $LASTEXITCODE -ne 0) { throw 'git --exec-path named no directory: install Git for Windows, then run this block again' }
  $git = Split-Path (Split-Path (Split-Path ($exec -replace '/', '\')))
  [Environment]::SetEnvironmentVariable('Path', [Environment]::GetEnvironmentVariable('Path', 'User') + ";$git\usr\bin;$git\bin", 'User')
  $env:PATH = "$git\usr\bin;$git\bin;$env:PATH"
}
```

<!-- windows-remedy:end -->

Then, in PowerShell, from your repository root:

```powershell
irm https://checkwright.dev/install.ps1 | iex
```

The script is `docs/install.ps1` in this repository; read it at <https://checkwright.dev/install.ps1> before you pipe it. To pass arguments, run it as a script block, the verb first: `& ([scriptblock]::Create((irm https://checkwright.dev/install.ps1))) init --profile prose`, or `demo`, or `uninstall`.

To run each step yourself on Windows: [Installing step by step](manual-install.md#windows).

### With Node

`npx checkwright init` runs the same `init` from the npm package, which carries npm's provenance attestation as the tarball carries a GitHub build attestation ([installer/SPEC.md](installer/SPEC.md#the-dependency-boundary)). `init` verifies the gate binary with `sha256sum` or `shasum`, and refuses without one. It needs Node 8.2 or later ([Requirements](#requirements)).

### From a plugin marketplace

In Claude Code, add Checkwright's marketplace and install its plugin:

```text
/plugin marketplace add checkwright/checkwright
/plugin install checkwright@checkwright
```

The plugin installs no kit: it registers Checkwright's lifecycle skills and its guards. Ask for its `install` skill in your repository: it chooses a profile with you and runs the same `init` this page documents. Then ask for its `adopt` skill: it fits the knobs to your layout, and works through the first reds with you until the battery is green or each has a disposition you accept. Any Agent Plugins client loads the repository's `plugin/` directory by its own install route ([plugin/README.md](plugin/README.md)).

### With another coding agent

Give your agent this prompt:

```text
Install Checkwright in this repository. Choose a profile with me from https://checkwright.dev/install.html#choosing-a-profile, then install it as https://checkwright.dev/install.html#install describes for this host, and run each command init prints. Then follow gate-sdk/templates/adopt.md.
```

The last step is the same walk the plugin's `adopt` skill runs.

### Choosing a profile

`init` vendors the kits, writes a `gates.list` and the config files they read, records the install in `checkwright.lock`, and makes one commit. It ends by printing the commands that finish the setup; run them.

Choose a profile with `--profile`. If another toolkit writes your specs, pass `--recipe` with the toolkit's name, as [Spec toolkits](spec-toolkits.md) shows. With none, a first install takes `starter` and a re-run keeps the profile the install recorded:

- `starter` — the gate SDK on its own;
- `delegation` — adds the kits for agent sessions;
- `prose` — adds canon-kit, for a repository of documents;
- `full` — everything.

Moving to a profile that contains yours only adds. `init` refuses outside a git work tree or in a repository git refuses to read (the refusal names the command that prints why), on a dirty worktree (`--no-commit` stages instead of committing), or when `checkwright doctor` finds a missing tool. Re-running it is safe: it reports files you have edited instead of overwriting them, unless you pass `--force`. `--dry-run` prints the plan and writes nothing.

**Choosing kits and gates.** `--with-kit` and `--without-kit` add a kit to your profile or take one out. `--with-gate` and `--without-gate` do the same for a gate. Each repeats. `checkwright.lock` records them, so a re-run keeps them until you pass new ones or `--no-selection`. `gate-sdk` stays, since it runs the others. To replace a gate with your own, put a gate of the same name in your gates directory. It runs instead of the kit's, and `init` never overwrites it.

## Going further

- **Vendoring by hand**, and what a compiled gate discloses: [Vendoring without the installer](installer/SPEC.md#vendoring-without-the-installer).
- **Reviewing the hook** before you install it: [Reviewing the pre-commit hook](installer/SPEC.md#reviewing-the-pre-commit-hook).
- **Running under an `AGENTS.md` harness**: [the adapter recipe](positioning.md#running-under-an-agentsmd-harness).
- **How the installer works**: [installer/SPEC.md](installer/SPEC.md).
- **What a commit costs**, measured: [the pre-commit figure](requirements.md#what-a-commit-costs).
- **Every step of the install**, run by hand: [Manual steps](manual-install.md).
- **Managing, uninstalling and upgrading**: [Maintenance](maintenance.md).

Back to the [kit map](index.md#the-kits) or [why Checkwright](methodology.md).
