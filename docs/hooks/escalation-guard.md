---
title: The escalation hook
nav_parent: install
nav_child_order: 7
since: v0.38.0
description: The escalation-guard hook advises a dispatched session whose message to its lead lacks the decision-shape headers, never blocks, and installs on its own, for one user, with no repository.
---

# The `escalation-guard` hook

This is one session hook you can run on its own, without adopting a kit: you install it for your user and add one entry to your coding agent's settings. This page tells you what it does and how to try it; each section links the specification that owns the detail. The other ways to install are on the [install page](../install.md).

## What it advises

A dispatched session escalates to the session that dispatched it with a message addressed to `main`. This hook runs before the agent's message tool, `SendMessage` in Claude Code, and advises the session when such a message lacks any of the headers **Question / Options / Recommendation / Evidence**. It never blocks: a message to any other recipient, or one carrying every header, passes untouched. The header grammar is [guard-kit/SPEC.md §escalation-guard](../guard-kit/SPEC.md#escalation-guard).

## What it reads

- **The event's payload**, on standard input: the message's recipient and its text.
- **Its knob file**, named in the settings entry after `--knobs`. It declares no knob of its own and reads no repository, so it runs from whatever directory the agent starts it in.

## How it exits

- **Every call**: exit 0. A headerless message to the lead gets the advice as added context for the session; any other message gets nothing.
- **A knob file it cannot use**: it allows the call and prints a notice naming `--knobs` and the fault.
- **On trial**, every call appends one line to the observe log instead.

The dispatch and the trial are [gate-sdk/SPEC.md §The harness-integration arm](../gate-sdk/SPEC.md#the-harness-integration-arm).

## Its knobs

<!-- hook-knobs:begin -->

This hook declares no knob of its own.

<!-- hook-knobs:end -->

The knob file the install writes carries the scratch directory alone, where the observe log derives.

## Installing it alone

On macOS and Linux:

```sh
curl -fsSL https://checkwright.dev/install.sh | sh -s -- init --user escalation-guard
```

On Windows, in PowerShell:

```powershell
& ([scriptblock]::Create((irm https://checkwright.dev/install.ps1))) init --user escalation-guard
```

It needs no repository and writes into none. It places the binary and the knob file in your per-user data directory and prints a `settings:` block holding the agent and version the entry needs, the settings file to merge it into, one JSON line running the placed binary on `PreToolUse` for the message tool with `--hook escalation-guard --knobs <the knob file>`, and a `trial:` line. The install never writes your settings: merging the entry is yours to do. The directories and the knob file are [installer/SPEC.md §The user install](../installer/SPEC.md#the-user-install).

## Removing it

Run the install line with `uninstall --user` in place of `init --user escalation-guard`, then delete the `PreToolUse` entry from your settings file yourself, as its `settings:` reminder says. What it removes and keeps is [installer/SPEC.md §uninstall](../installer/SPEC.md#uninstall).
