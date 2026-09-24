---
title: guard-kit
nav_parent: kits
nav_child_order: 5
---

# guard-kit

Permission-friction tooling for agent sessions. An agent that stops to ask permission for every routine command wastes turns and trains its supervisor to rubber-stamp; one that is allowed too much is unsafe. guard-kit steers the middle: a pre-tool guard that blocks, redirects, rewrites, or auto-allows a call by rule.

It ships a harness-generic ruleset, a scanner for where prompts originate, tracked-versus-local allowlist curation, an optional wakeup guard, and a close-stage step that triages the friction a session actually hit. It registers no gates.

## Install

The steps are the kit README's [Install](README.md#install) section. `checkwright init` does them for you — see the [install page](../install.md).

## Quick start

Run this arm with the gate binary `GATE_SDK_NATIVE_BIN` names:

```bash
. "${GATE_SDK_ROOT:-gate-sdk}/lib/gate.sh" && gates="$(gate_native_bin_spelled)"
"$gates" --run-guard-tests    # decision-table over the generic ruleset
```

## Contracts

The guard decision model and the friction-triage step are defined in the kit's [`SPEC.md`](SPEC.md#the-shell-guard); its [`README.md`](README.md#guard-kit) lists the mechanism. Back to the [kit map](../index.md#the-kits).
