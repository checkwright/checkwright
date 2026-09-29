---
name: install
description: Installs Checkwright into this repository with the installer's init, after choosing a profile with the user. Run it only when the user asks for it.
---

Install Checkwright into this repository.

1. Choose a profile with the user, from the installer's [Choosing a profile](https://github.com/checkwright/checkwright/blob/master/installer/README.md#choosing-a-profile) section.
2. Confirm the worktree is clean. If it is not, say so, and stop.
3. Run the one-line install for this host, with the chosen profile in place of `<profile>`:
   - macOS and Linux: `curl -fsSL https://checkwright.dev/install.sh | sh -s -- init --profile <profile>`
   - Windows, in PowerShell: `& ([scriptblock]::Create((irm https://checkwright.dev/install.ps1))) init --profile <profile>`
4. Run each command in the `next:` block `init` prints, in order.
5. Then run the `adopt` skill, which fits the knobs to this repository and triages the first reds.
