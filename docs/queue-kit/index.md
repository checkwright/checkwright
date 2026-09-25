---
title: queue-kit
nav_parent: kits
nav_child_order: 3
---

# queue-kit

A git-native, agent-readable task tracker. The queue is a single Markdown file under version control, not an external service — so an agent session reads its work and its history from the same tree it commits to, with no credential and no network round-trip.

queue-kit formalizes that file: one slug namespace, a small tag algebra (blocked-by, spec, drain-exempt, roadmap, observed-by, cost, surface, cap-credit, recurrence, roadmap-summary, not-icebox-eligible) that encodes preconditions an agent can resolve mechanically, an index tool, a roadmap projector that renders the curated entries as a generated public page, and gates that hold the grammar a session selects work by.

## Install

The steps are the kit README's [Install](README.md#install) section. `checkwright init` does them for you — see the [install page](../install.md).

## Quick start

Run these arms as the kit [README](README.md#use) spells them, PowerShell included:

<!-- fence-runnable -->
```sh
gates="${GATE_SDK_NATIVE_BIN:-./scripts/checkwright-gates}"
"$gates" --emit queue-index    # list selectable work
"$gates" --run-gate-tests queue-kit/gate-tests queue-kit/checks
```

## Contracts

The queue format and the tag algebra are defined in the kit's [`SPEC.md`](SPEC.md#the-queue-format); its [`README.md`](README.md#queue-kit) lists the mechanism. Back to the [kit map](../index.md#the-kits).
