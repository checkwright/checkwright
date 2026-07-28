---
title: Home
nav_order: 1
---

# Checkwright

**Verification for coding-agent delivery.** Checkwright is the verification
layer under agent orchestration: spec drift, skipped stages, and unsupported
*done* claims become failing checks before a merge, instead of review findings
after one.

It is for the maintainer of a repository coding agents write most of, who has to
answer at merge time whether the work is actually done and cannot answer it by
reading every diff.

**It complements the workflow you already run.** Keep your spec process, your
prompts, your harness. Add Checkwright where a claim has to be mechanically
proven rather than asserted: the instructions shape, the gates enforce. Why that
split is the whole design is the layer model on
[Where Checkwright sits](positioning.md).

One command runs the entire arc against a throwaway consumer repo, installing
nothing:

```bash
bash demo/run-demo.sh
```

It vendors the kits into a fresh git repo, passes the battery clean, introduces
a defect and shows the gate that blocks it, then drops the defect and goes green
again.

## What that buys you

**Before.** A session finishes a task and marks it done; the evidence is the
session's own say-so. A page keeps citing a spec section that a rename moved out
from under it. Both commits go in green, and the next stateless session reads
both as ground truth.

**After.** Neither commit lands:

```text
===== check-md-refs =====
check-md-refs: dangling reference in the governed doc set
  docs/guide.md:71 -> SPEC.md §Retry budget — no such section
FAIL: check-md-refs
===== check-stage-evidence =====
check-stage-evidence: a task reached Done with no validate stamp this iteration
FAIL: check-stage-evidence
```

Nothing there is a review opinion. Each finding is cheap and mechanically
decidable, which is what earns it the right to block a commit rather than open a
thread; the semantic residue stays with the human or the agent, undiluted.

That is what **verification under delegation** means, and it is the prerequisite
for scaling agent [orchestration](orchestration.md) past the point where a human
reads every hop: coordination is only worth parallelizing once each coordinated
result is checkable.

The enforcement core carries no harness dependency. The gate battery is bare
bash, so it runs under any coding-agent harness, any CI, or none. Only the
always-loaded convention adapts, riding whichever agent file your harness reads
by configuration rather than a port, per the
[tiered compatibility claim](positioning.md#the-tiered-compatibility-claim).

These pages orient and sequence. They own no contracts — each contract lives in
the kit that enforces it, and a page here cites downward rather than restating
an invariant.

## Start here

1. [Why Checkwright](methodology.md) — the delivery-methodology essay: what
   goes wrong when agents write, and the shape of the remedy.
2. [Install](install.md) — vendoring the kits into your repo and the
   upgrade contract.
3. [Value](value.md) — what each kit enforces set against what it costs your
   context budget, joined from the registries; drills down to the
   [enforcement map](enforcement.md) and the [footprint](footprint.md).
4. [Coupling graph](check-graph.html) — which content surfaces each gate binds
   together, emitted from the per-gate manifests.
5. The [Kit Reference](kits.md) — one page per kit, in reading order.
6. [Announcing Checkwright](posts/2026-07-09-announcing-checkwright.md) — the
   launch note.

## The kits

One page per kit, in reading order — each kit assumes the machinery of the ones
above it. The full map, with a one-line gloss per kit, lives on the
[Kit Reference](kits.md) page.

## Positioning

Where Checkwright sits against practices you may already run — one page per
angle, positioning only, each owning no contract and citing the enforcing kit
downward.

- [Where Checkwright sits](positioning.md) — the layer model: Checkwright as
  layer-4 content beneath a closed harness prompt, plus its tiered
  harness-compatibility claim and memory-off position.
- [Domain-driven design](ddd.md) — Checkwright as the enforcement layer for a
  ubiquitous language: banned synonyms, comment and naming directives, and one
  home per definition.
- [Agent orchestration](orchestration.md) — Checkwright as the verification
  layer beneath a coordination framework: the gates, budget guard, stage
  stamps, and evidence manifest that make delegated work checkable.

## License

Checkwright is Apache-2.0. Adoption is the goal.
