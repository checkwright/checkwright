---
title: Footprint
nav_parent: value
nav_child_order: 2
---
<!-- door-contributor: a generated projection whose only door is the emitter command behind its own numbers, read by the contributor who regenerates it -->

# Context footprint

What vendoring Checkwright costs a consumer's context budget, measured per kit and split by when the cost is paid. Every number here is generated from the tracked kit surfaces by `bash gate-sdk/bin/run-gates.sh --emit footprint` and held current by a freshness gate, so the page cannot drift from what the kits actually ship.

## What is measured

Each kit's footprint splits by when its cost lands in a session:

- **Always-loaded** — the fixed block a kit injects into the consumer's always-loaded agent file, so it rides every session's context. Measured as the content a kit generates between its own `begin`/`end` markers in the configured surface files.
- **Load-triggered** — the kit's shipped skill and template markdown, pulled into context only when its trigger fires. Measured over the markdown the kit ships under its templates directory.

Code-point counts (`cp`) are exact. The token column is a labeled estimate — a bytes-over-four heuristic, marked with a leading `~` because the true count is model-tokenizer-dependent; read it as an order of magnitude, never a precise figure.

## What is excluded

The figures are kit-share only — what a kit itself ships. A consumer's own bindings (the skill shims that point at a vendored template), consumer configuration, the reference SPEC and README pages a reader opens on demand, and the session hook's dynamic body (which is consumer state, not fixed kit text) are all left out, so each number reflects the kit's advertised cost rather than a host repository's residue.

## Per-kit footprint

| kit | always-loaded | load-triggered |
| --- | --- | --- |
| canon-kit | — | 5785cp · ~1531t |
| context-kit | — | 2351cp · ~594t |
| delegation-kit | — | 40153cp · ~10112t |
| doctrine-kit | 1819cp · ~463t | — |
| drift-kit | — | 6368cp · ~1610t |
| evidence-kit | — | — |
| gate-sdk | — | — |
| guard-kit | — | 4436cp · ~1124t |
| installer | — | — |
| lifecycle-kit | 301cp · ~76t | 119699cp · ~30189t |
| queue-kit | — | 1392cp · ~359t |
| site-kit | — | — |
| **total** | 2120cp · ~539t | 180184cp · ~45521t |
