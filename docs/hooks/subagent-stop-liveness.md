---
title: The turn-end hook
nav_parent: install
nav_child_order: 4
since: v0.36.0
description: The subagent-stop-liveness hook keeps a dispatched agent session from ending its turn while work it launched is still writing, and installs on its own, for one user, with no repository.
---

# The `subagent-stop-liveness` hook

This is one session hook you can run on its own, without adopting a kit: you install it for your user and add one entry to your coding agent's settings. This page tells you what it does and how to try it; each section links the specification that owns the detail. The other ways to install are on the [install page](../install.md).

## What it refuses

A coding agent can dispatch a session to do a piece of work. That session can start a command in the background and end its turn while the command still runs, so its report reaches its caller while the command keeps writing files nobody is watching. This hook runs when a dispatched session ends its turn, the `SubagentStop` event in Claude Code, and refuses the turn end while either of these holds:

- a launch record in its scratch directory names a process that is still running;
- the agent's task list shows a background shell command the session started, still running.

The refused session reads why, and what it may do: wait on the work's own output, or delete a record whose process has exited. The main session's turn end is never refused. Its readings, its wait modes and its limits are [delegation-kit/SPEC.md §The turn-end liveness hook](../delegation-kit/SPEC.md#the-turn-end-liveness-hook).

## What it reads

- **The event's payload**, on standard input: which session stopped, whether the agent is already continuing after a refusal, its list of running tasks, and the session's transcript, which it reads only to tell the tasks the session launched from its siblings'.
- **Launch records**: files named `<key>.run` in the scratch directory, each one line `pid=<n> run=<key>`. Whatever launches long-running work writes one, your work engine or a session following the waiting rule, and deletes it once the process has exited. A record whose line does not parse refuses too, until it is fixed or deleted. The line's grammar is [evidence-kit/SPEC.md §The producer-liveness lock](../evidence-kit/SPEC.md#the-producer-liveness-lock).
- **Its knob file**, named in the settings entry after `--knobs`. It reads no repository, so it runs from whatever directory the agent starts it in.

## How it exits

- **Allowed**: exit 0, with nothing on standard output.
- **Refused**: exit 2, the reason on standard error, which the agent shows the session as the reason its turn did not end.
- **A knob it cannot use**, such as a relative path: it allows the turn end and prints a notice for you, naming the knob, so an unguarded turn end is never silent.
- **Every firing** appends one line to the stop log, `DELEGATION_KIT_STOP_LOG`: the time, the event, the session, the reading, the decision, and the names of the payload's keys, never their values.
- **On trial**, every call appends one line to the observe log instead, and nothing is enforced.

The dispatch, the notice and the trial are [gate-sdk/SPEC.md §The harness-integration arm](../gate-sdk/SPEC.md#the-harness-integration-arm).

## Its knobs

<!-- hook-knobs:begin -->

| Knob | Kit | Shape | Default |
| --- | --- | --- | --- |
| `DELEGATION_KIT_STOP_LOG` | delegation-kit | scalar | `.workflow/subagent-stop-liveness.log` |
| `DELEGATION_KIT_STOP_WAIT` | delegation-kit | scalar | `refuse` |
| `DELEGATION_KIT_LIVENESS_CMD` | delegation-kit | indexed | (empty) |
| `GATE_SDK_TMP_DIR` | gate-sdk | scalar | `.tmp` |

<!-- hook-knobs:end -->

The defaults are the in-repository ones. Run outside a repository, the hook needs both paths absolute and declines on a relative one, so the install below writes absolute paths into the knob file. `DELEGATION_KIT_STOP_WAIT` takes `refuse` or `resume-wait`, the second for an agent that gives a session a new turn when its own background command completes. `DELEGATION_KIT_LIVENESS_CMD` replaces the process check, which by default is the installed binary itself. No knob turns the refusal off: removing the settings entry does.

## Installing it alone

On macOS and Linux:

```sh
curl -fsSL https://checkwright.dev/install.sh | sh -s -- init --user subagent-stop-liveness
```

On Windows, in PowerShell:

```powershell
& ([scriptblock]::Create((irm https://checkwright.dev/install.ps1))) init --user subagent-stop-liveness
```

It needs no repository and writes into none. It places the binary and the knob file in your per-user data directory, makes the scratch directory under your per-user state directory, and prints a `settings:` block holding:

- the agent and version the entry needs, Claude Code 2.1.139 or later, and the settings file to merge it into, `~/.claude/settings.json` or the one under `CLAUDE_CONFIG_DIR`;
- one JSON line, the entry for the file's `hooks` object, which runs the placed binary on `SubagentStop` with `--hook subagent-stop-liveness --knobs <the knob file>`;
- a `trial:` line, for the step below;
- a `launch records:` line, naming the scratch directory.

The install never writes your settings: merging the entry is yours to do. `--dry-run` prints the plan and the block and writes nothing, and `update --user` places a newer binary and prints the block again. The directories and the knob file are [installer/SPEC.md §The user install](../installer/SPEC.md#the-user-install), and the record the install keeps of itself is [§The user manifest](../installer/SPEC.md#the-user-manifest).

**Try it before it enforces.** Put `--observe` after `subagent-stop-liveness` in the entry's `args`. The hook then allows every turn end and logs the decision it would have taken, one `would=` line per call, to the observe log `GATE_SDK_HOOK_OBSERVE_LOG` names, which sits in the scratch directory unless you set it ([gate-sdk/SPEC.md §Layout and configuration](../gate-sdk/SPEC.md#layout-and-configuration)). Once the log shows the refusals you want, take `--observe` out.

**Point your launch records at the scratch directory**, or set `GATE_SDK_TMP_DIR` in the knob file to the directory your engine writes them to. A later install keeps a knob file you edited.

To check the release before anything runs, download and verify it by hand, the attestation included, as [Installing step by step](../manual-install.md) shows, then run `init --user subagent-stop-liveness` from the unpacked package in place of `init`.

## Removing it

Run the install line with `uninstall --user` in place of `init --user subagent-stop-liveness`. It removes the binary, the user manifest and every knob file you left unedited, and keeps one you edited unless you pass `--force`. It keeps the state directory, since the logs and launch records there are your evidence. It then prints a `settings:` reminder: delete the `SubagentStop` entry from your settings file yourself. What it removes and keeps is [installer/SPEC.md §uninstall](../installer/SPEC.md#uninstall).
