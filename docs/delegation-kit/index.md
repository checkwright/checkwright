---
title: delegation-kit
nav_parent: kits
nav_child_order: 6
---

# delegation-kit

Safe delegated-agent execution for budget-bounded sessions. Delegating work to sub-agents is the primary way a supervisor session stays within its token budget — but a sub-agent that shares the git index, weakens a gate to get past it, or reports a false pass turns a saving into a hazard. delegation-kit is the protocol that makes delegation safe.

It supplies the supervisor rules (serialize on the shared index, one commit per unit, a resume journal, verify after every agent commit), a trustworthy budget verdict, and a commit-shape gate that blocks the attested gate-weakening shapes.

## Install

The steps are the kit README's [Install](README.md#install) section. `checkwright init` does them for you — see the [install page](../install.md).

## Quick start

Run this arm with the gate binary `GATE_SDK_NATIVE_BIN` names:

```bash
. "${GATE_SDK_ROOT:-gate-sdk}/lib/gate.sh" && gates="$(gate_native_bin_spelled)"
"$gates" --usage-verdict        # one budget verdict: OK / PAUSE / STALE
```

## Contracts

The supervisor protocol and the tamper-gate contract are defined in the kit's [`SPEC.md`](SPEC.md#the-delegation-model); its [`README.md`](README.md#delegation-kit) lists the mechanism. Back to the [kit map](../index.md#the-kits).
