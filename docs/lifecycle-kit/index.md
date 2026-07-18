---
title: lifecycle-kit
nav_parent: kits
nav_child_order: 2
---

# lifecycle-kit

An iteration stage state machine built for stateless agent sessions. Where a
human developer carries the state of a task in their head across a day,
successive agent sessions do not — so lifecycle-kit externalizes it: an
evidence-stamp file records which stage a unit of work is in — its last stamp
is the cursor — and proves each stage was actually entered.

Each stage is a skill a session invokes; entering a stage stamps that file in
one commit, and gates make skipping a stage fail the commit. The stages
themselves are configuration, not code.

## Install

Vendor the `lifecycle-kit/` directory into your repo and register its stage
gates in `gates.list`. The stage set, the header file, and the stamp file are
external configuration you point the kit at.

## Quick start

```bash
bash lifecycle-kit/bin/enter-stage.sh <stage>        # stamp the stage entry, committed
bash gate-sdk/bin/run-gate-tests.sh lifecycle-kit/gate-tests lifecycle-kit/checks
```

## Contracts

The state-machine contract — the stamp grammar, the stamp protocol,
and the gates that enforce it — lives in the kit's
[`SPEC.md`](SPEC.md#the-state-machine);
its [`README.md`](README.md#lifecycle-kit)
lists the mechanism. Back to the [kit map](../index.md#the-kits).
