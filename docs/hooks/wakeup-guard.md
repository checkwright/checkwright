---
title: The wakeup hook
nav_parent: install
nav_child_order: 6
since: unreleased
description: The wakeup-guard hook denies every self-scheduled wakeup a coding agent tries to set and logs the attempt, and installs on its own, for one user, with no repository.
---

# The `wakeup-guard` hook

This is one session hook you can run on its own, without adopting a kit: you install it for your user and add one entry to your coding agent's settings. This page tells you what it does and how to try it; each section links the specification that owns the detail. The other ways to install are on the [install page](../install.md).

## What it refuses

A coding agent can schedule its own later wakeup, a stored prompt that fires in a later session as if you typed it. This hook runs before the agent's wake tools, `ScheduleWakeup` and `CronCreate` in Claude Code, and denies every such call. The session reads the denial. Its posture and the attempt log's line grammar are [guard-kit/SPEC.md §wakeup-guard](../guard-kit/SPEC.md#wakeup-guard).

## What it reads

- **The event's payload**, on standard input: the session, the tool and its input, which it copies into the attempt line.
- **Its knob file**, named in the settings entry after `--knobs`. It reads no repository, so it runs from whatever directory the agent starts it in.

## How it exits

- **Every wake call**: exit 2 with the denial on standard error. One line goes to the attempt log, `GUARD_KIT_WAKEUP_LOG`: the time, then the payload's fields.
- **A knob it cannot use**, such as a relative log path or a knob file it cannot read: the call is still denied but goes unlogged. The denial names what is at fault.
- **On trial**, every call appends one line to the observe log instead, and nothing is enforced.

The dispatch and the trial are [gate-sdk/SPEC.md §The harness-integration arm](../gate-sdk/SPEC.md#the-harness-integration-arm).

## Its knobs

<!-- hook-knobs:begin -->

| Knob | Kit | Shape | Default |
| --- | --- | --- | --- |
| `GUARD_KIT_WAKEUP_LOG` | guard-kit | scalar | `.workflow/wakeup-attempts.log` |

<!-- hook-knobs:end -->

The default is the in-repository one. Run outside a repository, the hook needs the log path absolute, so the install below writes an absolute path into the knob file. No knob turns the denial off: removing the settings entry does.

## Installing it alone

On macOS and Linux:

```sh
curl -fsSL https://checkwright.dev/install.sh | sh -s -- init --user wakeup-guard
```

On Windows, in PowerShell:

```powershell
& ([scriptblock]::Create((irm https://checkwright.dev/install.ps1))) init --user wakeup-guard
```

It needs no repository and writes into none. It places the binary and the knob file in your per-user data directory, makes the scratch directory under your per-user state directory, and prints a `settings:` block holding:

- the agent and version the entry needs, and the settings file to merge it into;
- one JSON line, the entry for the file's `hooks` object, which runs the placed binary on `PreToolUse` for the wake tools with `--hook wakeup-guard --knobs <the knob file>`;
- a `trial:` line, for the step below;
- an `attempt log:` line, naming the log in your per-user state directory.

The install never writes your settings: merging the entry is yours to do. `--dry-run` prints the plan and the block and writes nothing, and `update --user` places a newer binary and prints the block again. The directories and the knob file are [installer/SPEC.md §The user install](../installer/SPEC.md#the-user-install).

**Try it before it enforces.** Put `--observe` after `wakeup-guard` in the entry's `args`. The hook then allows every call and logs the decision it would have taken to the observe log. Once the log shows the denials you want, take `--observe` out.

**To schedule a wakeup deliberately**, remove the entry for that session; the denial is the default, not a capability removed.

## Removing it

Run the install line with `uninstall --user` in place of `init --user wakeup-guard`. It removes the binary, the user manifest and every knob file you left unedited, and keeps the state directory, the attempt log with it. It then prints a `settings:` reminder: delete the `PreToolUse` entry from your settings file yourself. What it removes and keeps is [installer/SPEC.md §uninstall](../installer/SPEC.md#uninstall).
