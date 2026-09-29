---
description: "Install Checkwright's gates, prose or every kit, and apply the Spec Kit recipe"
argument-hint: "prose (the default) or full"
---

## User Input

$ARGUMENTS

## Steps

Work from the repository root, in a clean worktree. The install makes one commit and refuses uncommitted changes, so ask the user to commit or stash them first.

**Choose the profile.** If the user input asks for `full`, install `full`; otherwise install `prose`. Before a `full` install, tell the user what it adds and costs: every kit's gates, `bash` 4.3 or later, which stock macOS lacks, and Checkwright's own task queue, a doctrine block in the agent file and evidence files under `.workflow/`, beside Spec Kit's own. If `init` refuses on a missing tool, report its `help:` line and stop.

**Install the release this extension was tested with.** Read `extension.version` in `.specify/extensions/checkwright/extension.yml` and put it in place of `X.Y.Z` in the line for the profile and the host. For `prose` on macOS and Linux:

<!-- companion-install:begin -->

```sh
curl -fsSL https://checkwright.dev/install.sh | CHECKWRIGHT_VERSION=X.Y.Z sh -s -- init --profile prose --recipe speckit
```

<!-- companion-install:end -->

For `full` on macOS and Linux:

<!-- companion-full:begin -->

```sh
curl -fsSL https://checkwright.dev/install.sh | CHECKWRIGHT_VERSION=X.Y.Z sh -s -- init --profile full --recipe speckit
```

<!-- companion-full:end -->

On Windows, in PowerShell, for `prose`:

```powershell
$env:CHECKWRIGHT_VERSION = 'X.Y.Z'; & ([scriptblock]::Create((irm https://checkwright.dev/install.ps1))) init --profile prose --recipe speckit
```

For `full`:

```powershell
$env:CHECKWRIGHT_VERSION = 'X.Y.Z'; & ([scriptblock]::Create((irm https://checkwright.dev/install.ps1))) init --profile full --recipe speckit
```

**Finish the setup.** Run each command `init` printed under `next:`.

**Report the result.** On a green battery, say so and name the commit. On a red one, list each failing gate with its finding and its `help:` line, and stop.

Never edit a file under a vendored kit directory, drop a gate, or commit with `--no-verify`.
