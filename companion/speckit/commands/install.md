---
description: "Install Checkwright's gates and apply the Spec Kit recipe"
---

## User Input

$ARGUMENTS

## Steps

Work from the repository root, in a clean worktree. The install makes one commit and refuses uncommitted changes, so ask the user to commit or stash them first.

**Install the release this extension was tested with.** Read `extension.version` in `.specify/extensions/checkwright/extension.yml` and put it in place of `X.Y.Z` in the line for the host. On macOS and Linux:

<!-- companion-install:begin -->

```sh
curl -fsSL https://checkwright.dev/install.sh | CHECKWRIGHT_VERSION=X.Y.Z sh -s -- init --profile prose --recipe speckit
```

<!-- companion-install:end -->

On Windows, in PowerShell:

```powershell
$env:CHECKWRIGHT_VERSION = 'X.Y.Z'; & ([scriptblock]::Create((irm https://checkwright.dev/install.ps1))) init --profile prose --recipe speckit
```

**Finish the setup.** Run each command `init` printed under `next:`.

**Report the result.** On a green battery, say so and name the commit. On a red one, list each failing gate with its finding and its `help:` line, and stop.

Never edit a file under a vendored kit directory, drop a gate, or commit with `--no-verify`.
