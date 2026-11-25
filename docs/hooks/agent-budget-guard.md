---
title: The budget hook
nav_parent: install
nav_child_order: 8
since: v0.38.0
description: The agent-budget-guard hook blocks an agent dispatch while the usage verdict says pause, advises on every other verdict, and installs on its own, for one user, with no repository.
---

# The `agent-budget-guard` hook

This is one session hook you can run on its own, without adopting a kit: you install it for your user and add one entry to your coding agent's settings. This page tells you what it does and how to try it; each section links the specification that owns the detail. The other ways to install are on the [install page](../install.md).

## What it refuses

This hook runs before the agent's dispatch tool, `Agent` in Claude Code, and reads the usage verdict: whether the plan's five-hour or weekly window is past its pause threshold. A `PAUSE` verdict blocks the dispatch, naming the window that fired and the knob that raises its threshold. Every other verdict, `OK`, `RESET-OK` or `STALE`, lets the dispatch through with the verdict as added context. The verdict rule is [delegation-kit/SPEC.md §usage-verdict](../delegation-kit/SPEC.md#usage-verdict), and the guard's wiring is [§Layout and configuration](../delegation-kit/SPEC.md#layout-and-configuration).

## What it reads

- **The usage snapshot file**, which something you run keeps current, such as the statusline arm ([delegation-kit/SPEC.md §The statusline arm](../delegation-kit/SPEC.md#the-statusline-arm)).
- **The credentials file and the account config**, to tell a fresh login's lagging reading from a real one.
- **Its knob file**, named in the settings entry after `--knobs`. It reads no repository, so it runs from whatever directory the agent starts it in.

## How it exits

- **`PAUSE`**: exit 2, the verdict and its corrective on standard error.
- **Any other verdict**: exit 0, the verdict as context for the session.
- **A knob it cannot use**, such as a relative path: the verdict is `STALE`, naming the knob, and the dispatch goes through.
- **On trial**, every call appends one line to the observe log instead, and nothing is enforced.

## Its knobs

<!-- hook-knobs:begin -->

| Knob | Kit | Shape | Default |
| --- | --- | --- | --- |
| `DELEGATION_KIT_USAGE_FILE` | delegation-kit | scalar | (empty) |
| `DELEGATION_KIT_CRED_FILE` | delegation-kit | scalar | (empty) |
| `DELEGATION_KIT_ACCOUNT_CONFIG` | delegation-kit | scalar | (empty) |
| `DELEGATION_KIT_PAUSE_PCT` | delegation-kit | scalar | `80` |
| `DELEGATION_KIT_PAUSE_PCT_7D` | delegation-kit | scalar | `95` |
| `DELEGATION_KIT_STALE_AGE` | delegation-kit | scalar | `600` |
| `DELEGATION_KIT_LOGIN_WINDOW` | delegation-kit | scalar | `600` |
| `DELEGATION_KIT_LOGIN_SETTLE` | delegation-kit | scalar | `90` |
| `DELEGATION_KIT_REFRESH_CMD` | delegation-kit | indexed | (empty) |
| `DELEGATION_KIT_REFRESH_MIN_AGE` | delegation-kit | scalar | `60` |
| `DELEGATION_KIT_USAGE_HISTORY` | delegation-kit | scalar | (empty) |
| `DELEGATION_KIT_FAN_WIDTH` | delegation-kit | scalar | `2` |

<!-- hook-knobs:end -->

Run outside a repository, every path the verdict reads must be absolute. The usage, credential and account defaults derive from your home directory, so they are absolute wherever it is set. To raise a threshold, set its knob in the knob file.

## Installing it alone

On macOS and Linux:

```sh
curl -fsSL https://checkwright.dev/install.sh | sh -s -- init --user agent-budget-guard
```

On Windows, in PowerShell:

```powershell
& ([scriptblock]::Create((irm https://checkwright.dev/install.ps1))) init --user agent-budget-guard
```

It needs no repository and writes into none. It places the binary and the knob file in your per-user data directory and prints a `settings:` block holding the agent and version the entry needs, the settings file to merge it into, one JSON line running the placed binary on `PreToolUse` for the dispatch tool with `--hook agent-budget-guard --knobs <the knob file>`, and a `trial:` line. The install never writes your settings: merging the entry is yours to do. The directories and the knob file are [installer/SPEC.md §The user install](../installer/SPEC.md#the-user-install).

**Try it before it enforces.** Put `--observe` after `agent-budget-guard` in the entry's `args`, and take it out once the observe log shows the blocks you want.

## Removing it

Run the install line with `uninstall --user` in place of `init --user agent-budget-guard`, then delete the `PreToolUse` entry from your settings file yourself, as its `settings:` reminder says. What it removes and keeps is [installer/SPEC.md §uninstall](../installer/SPEC.md#uninstall).
