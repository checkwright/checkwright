---
title: The dispatch hook
nav_parent: install
nav_child_order: 9
since: unreleased
description: The agent-dispatch-guard hook blocks an agent dispatch that forks, escapes its isolation or leaves its model tier unchosen, and installs on its own, for one user, with no repository.
---

# The `agent-dispatch-guard` hook

This is one session hook you can run on its own, without adopting a kit: you install it for your user and add one entry to your coding agent's settings. This page tells you what it does and how to try it; each section links the specification that owns the detail. The other ways to install are on the [install page](../install.md).

## What it refuses

This hook runs before the agent's dispatch tool, `Agent` in Claude Code, and holds the dispatch to its rules, in order:

- **D1** blocks a `fork` dispatch, always.
- **D2** blocks a type you declared read-only unless it runs in its own worktree.
- **D4** blocks a type you have not declared mutating unless it runs in its own worktree, once you declare any.
- **D5** blocks a dispatch naming no model whose type's definition states none, once you switch it on.
- **D6** blocks a named model outside the tiers you bound, once you bind any.
- **D3** advises a dispatch from inside a dispatched session.

Each block names the lawful alternatives. The rules and their order are [delegation-kit/SPEC.md §The delegation model](../delegation-kit/SPEC.md#the-delegation-model).

## What it reads

- **The event's payload**, on standard input: the dispatch's type, isolation and model, and whether the dispatcher is itself a dispatched session.
- **Its knob file**, named in the settings entry after `--knobs`: the type rosters, D5's switch and the agent-definition directory, and the tier binding. It reads no repository, so it runs from whatever directory the agent starts it in.

## How it exits

- **Allowed**: exit 0, with D3's advice as context where it fires.
- **Blocked**: exit 2, the rule and its alternatives on standard error.
- **A knob it cannot use**: the rule that knob serves goes unenforced on that call, and the advice says so.
- **On trial**, every call appends one line to the observe log instead, and nothing is enforced.

## Its knobs

<!-- hook-knobs:begin -->

| Knob | Kit | Shape | Default |
| --- | --- | --- | --- |
| `DELEGATION_KIT_READONLY_TYPES` | delegation-kit | indexed | (empty) |
| `DELEGATION_KIT_MUTATING_TYPES` | delegation-kit | indexed | (empty) |
| `DELEGATION_KIT_REQUIRE_TIER` | delegation-kit | scalar | `off` |
| `DELEGATION_KIT_AGENT_DIR` | delegation-kit | scalar | `.claude/agents` |
| `DELEGATION_KIT_TIER_MODEL` | delegation-kit | indexed | (empty) |
| `GATE_SDK_NATIVE_BIN` | gate-sdk | scalar | `native/target/release/checkwright-gates` |

<!-- hook-knobs:end -->

D2, D4, D5 and D6 do nothing until you configure them. To switch D5 on outside a repository, set `DELEGATION_KIT_AGENT_DIR` to an absolute directory in the knob file: a relative one, the default included, leaves D5 unenforced. A user-scope hook looks a type up in that one directory, never in the project the dispatch fires in ([delegation-kit/SPEC.md §Layout and configuration](../delegation-kit/SPEC.md#layout-and-configuration)).

## Installing it alone

On macOS and Linux:

```sh
curl -fsSL https://checkwright.dev/install.sh | sh -s -- init --user agent-dispatch-guard
```

On Windows, in PowerShell:

```powershell
& ([scriptblock]::Create((irm https://checkwright.dev/install.ps1))) init --user agent-dispatch-guard
```

It needs no repository and writes into none. It places the binary and the knob file in your per-user data directory and prints a `settings:` block holding the agent and version the entry needs, the settings file to merge it into, one JSON line running the placed binary on `PreToolUse` for the dispatch tool with `--hook agent-dispatch-guard --knobs <the knob file>`, and a `trial:` line. The install never writes your settings: merging the entry is yours to do. The directories and the knob file are [installer/SPEC.md §The user install](../installer/SPEC.md#the-user-install).

**Try it before it enforces.** Put `--observe` after `agent-dispatch-guard` in the entry's `args`, and take it out once the observe log shows the blocks you want.

## Removing it

Run the install line with `uninstall --user` in place of `init --user agent-dispatch-guard`, then delete the `PreToolUse` entry from your settings file yourself, as its `settings:` reminder says. What it removes and keeps is [installer/SPEC.md §uninstall](../installer/SPEC.md#uninstall).
