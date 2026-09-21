---
title: drift-kit
nav_parent: kits
nav_child_order: 8
---

# drift-kit

Advisory drift reporting for stateless sessions. Not every consistency axis is
cheap enough to block a commit — some are trends, not violations. drift-kit
reports those: it collates pluggable KPIs from the other kits' governed
surfaces under honest lead/lag labels, so a session sees where the tree is
sliding without a false red.

It ships the drift report, a KPI plugin registry, a one-line trend summary the
session-start hook injects, and the knowledge-friction loop that captures a
re-derived fact the moment it costs a session time. It registers no gates.

## Install

Vendor the `drift-kit/` directory into your repo, register your KPIs in its
plugin list, and wire the trend summary into your session-start brief.

## Quick start

Run this arm with the gate binary `GATE_SDK_NATIVE_BIN` names:

```bash
. "${GATE_SDK_ROOT:-gate-sdk}/lib/gate.sh" && gates="$(gate_native_bin_spelled)"
"$gates" --emit drift-report   # the full advisory report
```

## Contracts

The KPI registry contract and the knowledge-friction loop are defined in the
kit's
[`SPEC.md`](SPEC.md#the-kpi-plugin-contract);
its [`README.md`](README.md#drift-kit)
lists the mechanism. Back to the
[kit map](../index.md#the-kits).
