---
title: Footprint
nav_parent: value
nav_child_order: 2
---

# Context footprint

What vendoring Checkwright costs a consumer's context budget, measured per kit
and split by when the cost is paid. Every number here is generated from the
tracked kit surfaces by `context-kit/bin/footprint.sh` and held current by a
freshness gate, so the page cannot drift from what the kits actually ship.

## What is measured

Each kit's footprint splits by when its cost lands in a session:

- **Always-loaded** — the fixed block a kit injects into the consumer's
  always-loaded agent file, so it rides every session's context. Measured as the
  content a kit generates between its own `begin`/`end` markers in the configured
  surface files.
- **Load-triggered** — the kit's shipped skill and template markdown, pulled
  into context only when its trigger fires. Measured over the markdown the kit
  ships under its templates directory.

Line counts are exact. The token column is a labeled estimate — a
bytes-over-four heuristic, marked with a leading `~` because the true count is
model-tokenizer-dependent; read it as an order of magnitude, never a precise
figure.

## What is excluded

The figures are kit-share only — what a kit itself ships. A consumer's own
bindings (the skill shims that point at a vendored template), consumer
configuration, the reference SPEC and README pages a reader opens on demand, and
the session hook's dynamic body (which is consumer state, not fixed kit text) are
all left out, so each number reflects the kit's advertised cost rather than a
host repository's residue.

## Per-kit footprint

| kit | always-loaded | load-triggered |
| --- | --- | --- |
| canon-kit | — | 53l · ~669t |
| context-kit | — | 30l · ~493t |
| delegation-kit | — | 192l · ~3137t |
| doctrine-kit | 16l · ~310t | — |
| drift-kit | — | 64l · ~1024t |
| evidence-kit | — | — |
| gate-sdk | — | — |
| guard-kit | — | 31l · ~486t |
| lifecycle-kit | 5l · ~74t | 684l · ~10585t |
| queue-kit | — | 43l · ~378t |
| site-kit | — | — |
| **total** | 21l · ~384t | 1097l · ~16774t |
