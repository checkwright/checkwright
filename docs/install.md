---
title: Install
nav_id: install
nav_order: 4
nav_suffix: spec=installer/SPEC.md plugin=plugin/SPEC.md
---

# Install and upgrade

This page gets the kits into your repository, keeps them current, and takes them out again. Checkwright is vendored: `init` copies the kit source into your tree and commits it, so what governs your repository is committed and reviewable. The gate binary is the one compiled piece, and it is checked against a published digest before anything runs. The [footprint page](footprint.md) prices each kit in agent context.

## Requirements

An install needs a supported system and a few tools, and [What an install requires](requirements.md) lists both.

## Install

<!-- install-primary: tarball -->

Run the one line for your system from your repository's root, in a new project or an existing one with every change committed: `init` makes one commit and refuses a dirty worktree. The line downloads the newest Release tarball, checks it against its published digest and, where `gh` is signed in and the release is attested, against its build attestation, unpacks it outside your repository and runs `init`, with no runtime to install first. The recommended path starts with `demo`, to see the guarantee. Install with your line and run the commands `init` prints; `checkwright prove` then shows the guarantee on your tree ([Prove it on your tree](#prove-it-on-your-tree)).

Pick your route:

- **macOS or Linux:** [the one line and its steps](#macos-and-linux).
- **Windows:** [natively, in PowerShell](#windows).
- **Windows through Windows Subsystem for Linux (WSL):** the [macOS and Linux](#macos-and-linux) line, in your WSL shell. The install places the Linux gate binary and the pre-commit hook runs it, so commit to that repository from WSL too: Git for Windows cannot start the Linux binary.
- **With Node:** [`npx checkwright init`](#with-node).
- **In Claude Code or Codex:** [the plugin marketplace](#from-a-plugin-marketplace).
- **With another coding agent:** [a prompt to give it](#with-another-coding-agent).

### Try it first

`demo` in place of `init` runs the whole arc in a scratch repository of its own and removes it, without touching yours: it installs the `full` profile, runs the battery green, commits a task marked done with no evidence behind it and shows the claim caught, then withdraws the claim and runs green again. Each line below takes it as its verb; with Node, `npx checkwright demo`. On Windows, run the `PATH` block below first.

### macOS and Linux

**You need** the tools [Requirements](#requirements) lists for installing on Linux and macOS.

On macOS, stock bash is 3.2, below the floor. If your profile owes `bash` (Requirements), run this block, which installs Homebrew's bash and puts it first on your `PATH`, now and in `~/.zprofile`. If your login shell is bash, use `~/.bash_profile` instead.

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

**You need** the tools [Requirements](#requirements) lists for installing on Windows, where the minimum Windows version is stated too.

The pre-commit hook needs no shell: git starts the gate binary directly. Nor does the battery: run the binary with `--run`, or `gate-sdk/bin/run-gates.ps1` from PowerShell.

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

`npx checkwright init` runs the same `init` from the npm package, which carries npm's provenance attestation as the tarball carries a GitHub build attestation ([installer/SPEC.md](installer/SPEC.md#the-dependency-boundary)). The package's command is a shell script, so on Windows run the `PATH` block above first, which puts the `sh` npm starts it with on your `PATH`.

### From a plugin marketplace

In Claude Code, add Checkwright's marketplace and install its plugin:

```text
/plugin marketplace add checkwright/checkwright
/plugin install checkwright@checkwright
```

In Codex:

```text
codex plugin marketplace add checkwright/checkwright
codex plugin add checkwright@checkwright
```

The plugin installs no kit. Ask for its `install` skill in your repository: it chooses a profile with you and runs the same `init` this page documents. Then ask for its `adopt` skill: it fits the knobs to your layout, and works through the first reds with you until the battery is green or each has a disposition you accept. Any Agent Plugins client, Antigravity among them, loads the repository's `plugin/` directory by its own route ([plugin/README.md](plugin/README.md)).

### With another coding agent

Give your agent this prompt:

```text
Install Checkwright in this repository. Choose a profile with me from https://checkwright.dev/install.html#choosing-a-profile, then install it as https://checkwright.dev/install.html#install describes for this host, and run each command init prints. Then follow gate-sdk/templates/adopt.md. Then run checkwright prove from the repository root, through the then: line init printed, and report its last line.
```

The last step is the same walk the plugin's `adopt` skill runs.

### Choosing a profile

Choose a profile with `--profile`. If another toolkit writes your specs, pass `--recipe` with the toolkit's name, as [Spec toolkits](spec-toolkits.md) shows. With none, a first install takes `starter` and a re-run keeps the profile the install recorded:

- `starter` — the gate SDK on its own;
- `workflow` — adds the stage machine and the recorded test runs to it, the least on which `prove` catches a done claim;
- `delegation` — adds the kits for agent sessions;
- `prose` — adds canon-kit, for a repository of documents;
- `full` — everything.

Moving to a profile that contains yours only adds. Re-running it is safe: it reports files you have edited instead of overwriting them, unless you pass `--force`. `--dry-run` prints the plan and writes nothing.

**Choosing kits and gates.** `--with-kit` and `--without-kit` add a kit to your profile or take one out. `--with-gate` and `--without-gate` do the same for a gate. Each repeats. `checkwright.lock` records them, so a re-run keeps them until you pass new ones or `--no-selection`. `gate-sdk` stays, since it runs the others. To replace a gate with your own, put a gate of the same name in your gates directory. It runs instead of the kit's, and `init` never overwrites it.

### Prove it on your tree

`checkwright prove` makes the claim `demo` shows caught, a task marked done with no evidence behind it, on a scratch clone of your `HEAD`. It runs your own battery there before and after the claim and reports what your install enforces at commit and in CI, and what it leaves inactive. It writes nothing here. It asks nothing either, so a coding agent can run it unattended. The `then:` line `init` prints runs it through the binary `init` placed, and the [maintenance verbs](maintenance.md#managing) spell it for each route.

Its last line is the verdict, and its exit status says the same: `PROVE: held`, exit `0`, with the minutes since the install commit, or `PROVE: not held`, exit `1`, with the cause. `starter`, the default, keeps no task queue, so it is not held; on a tree with no task list either, a `help:` line names the profile that adds one.

## Going further

- **Vendoring by hand**, and what a compiled gate discloses: [Vendoring without the installer](installer/SPEC.md#vendoring-without-the-installer).
- **Reviewing the hook** before you install it: [Reviewing the pre-commit hook](installer/SPEC.md#reviewing-the-pre-commit-hook).
- **Running under an `AGENTS.md` harness**: [the adapter recipe](positioning.md#running-under-an-agentsmd-harness).
- **How the installer works**: [installer/SPEC.md](installer/SPEC.md).
- **What a commit costs**, measured: [the pre-commit figure](requirements.md#what-a-commit-costs).
- **Every step of the install**, run by hand: [Manual steps](manual-install.md).
- **Managing, uninstalling and upgrading**: [Maintenance](maintenance.md).

Back to the [kit map](index.md#the-kits) or [why Checkwright](methodology.md).
