---
title: lifecycle-kit
nav_parent: kits
nav_child_order: 2
---

# lifecycle-kit

An iteration stage state machine built for stateless agent sessions. Where a human developer carries the state of a task in their head across a day, successive agent sessions do not — so lifecycle-kit externalizes it: an evidence-stamp file records which stage a unit of work is in — its last stamp is the cursor — and proves each stage was actually entered.

Each stage is a skill a session invokes; entering a stage stamps that file in one commit, and gates make skipping a stage fail the commit. The stages themselves are configuration, not code.

## Install

The steps are the kit README's [Install](README.md#install) section. `checkwright init` does them for you — see the [install page](../install.md).

## Quick start

Run these arms as the kit [README](README.md#use) spells them, PowerShell included:

```sh
gates="${GATE_SDK_NATIVE_BIN:-./scripts/checkwright-gates}"
"$gates" --enter-stage <stage>          # stamp a stage entry (the transition itself)
"$gates" --run-gate-tests lifecycle-kit/gate-tests lifecycle-kit/checks
```

## Contracts

The state-machine contract — the stamp grammar, the stamp protocol, and the gates that enforce it — lives in the kit's [`SPEC.md`](SPEC.md#the-state-machine); its [`README.md`](README.md#lifecycle-kit) lists the mechanism. Back to the [kit map](../index.md#the-kits).
