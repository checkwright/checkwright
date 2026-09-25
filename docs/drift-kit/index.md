---
title: drift-kit
nav_parent: kits
nav_child_order: 8
---

# drift-kit

Advisory drift reporting for stateless sessions. Not every consistency axis is cheap enough to block a commit — some are trends, not violations. drift-kit reports those: it collates pluggable KPIs from the other kits' governed surfaces under honest lead/lag labels, so a session sees where the tree is sliding without a false red.

It ships the drift report, a KPI plugin registry, a one-line trend summary the session-start hook injects, and the knowledge-friction loop that captures a re-derived fact the moment it costs a session time. It registers no gates.

## Install

The steps are the kit README's [Install](README.md#install) section. `checkwright init` does them for you — see the [install page](../install.md).

## Quick start

Run this arm as the kit [README](README.md#use) spells it, PowerShell included:

<!-- fence-runnable -->
```sh
gates="${GATE_SDK_NATIVE_BIN:-./scripts/checkwright-gates}"
"$gates" --emit drift-report   # the full advisory report
```

## Contracts

The KPI registry contract and the knowledge-friction loop are defined in the kit's [`SPEC.md`](SPEC.md#the-kpi-plugin-contract); its [`README.md`](README.md#drift-kit) lists the mechanism. Back to the [kit map](../index.md#the-kits).
