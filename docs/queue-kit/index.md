---
title: queue-kit
nav_parent: kits
nav_child_order: 3
---

# queue-kit

A git-native, agent-readable task tracker. The queue is a single Markdown file
under version control, not an external service — so an agent session reads its
work and its history from the same tree it commits to, with no credential and
no network round-trip.

queue-kit formalizes that file: one slug namespace, a small tag algebra
(blocked-by, needs-spec, spec) that encodes preconditions an agent can resolve
mechanically, an index tool, and gates that hold the grammar a session selects
work by.

## Install

Vendor the `queue-kit/` directory into your repo, register its gates in
`gates.list`, and keep your queue in the tracked file the kit points at.

## Quick start

```bash
bash queue-kit/bin/queue-index.sh                    # list selectable work
bash gate-sdk/bin/run-gate-tests.sh queue-kit/gate-tests queue-kit/checks
```

## Contracts

The queue format and the tag algebra are defined in the kit's
[`SPEC.md`](SPEC.md#the-queue-format);
its [`README.md`](README.md#queue-kit)
lists the mechanism. Back to the [kit map](../index.md#the-kits).
