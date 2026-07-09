# Checkwright

Checkwright is a coding-agent-assisted delivery methodology, packaged as
installable kits. It mechanizes the discipline that prose alone cannot hold when
stateless agent sessions do the writing: every cheap, mechanically-decidable
consistency axis becomes a gate that blocks the commit, so the human (or agent)
residue is the irreducibly semantic judgment alone.

These pages orient and sequence. They own no contracts — each contract lives in
the kit that enforces it, and a page here cites downward rather than restating
an invariant.

## Start here

1. [Why Checkwright](methodology.md) — the delivery-methodology essay: what
   goes wrong when agents write, and the shape of the remedy.
2. [Install](install.md) — vendoring the kits into your repo and the
   upgrade contract.
3. The kit map below — one page per kit, in reading order.
4. [Announcing Checkwright](posts/2026-07-09-announcing-checkwright.md) — the
   launch note.

## The kits

Read in this order: each kit assumes the machinery of the ones above it.

- [gate-sdk](gate-sdk/) — the lint framework the other kits register into.
- [lifecycle-kit](lifecycle-kit/) — the iteration stage state machine for
  stateless sessions.
- [queue-kit](queue-kit/) — the git-native, agent-readable task tracker.
- [spec-kit](spec-kit/) — spec discipline for agent-authored components.
- [guard-kit](guard-kit/) — permission-friction tooling for agent sessions.
- [delegation-kit](delegation-kit/) — safe delegated-agent execution under a
  token budget.
- [context-kit](context-kit/) — token-economics-aware context management.
- [drift-kit](drift-kit/) — advisory drift reporting across the governed
  surfaces.
- [evidence-kit](evidence-kit/) — a held-constant test baseline and a
  committed per-run evidence manifest.

The walkthrough and evidence pages join this map when their kits land.

## License

Checkwright is Apache-2.0. Adoption is the goal.
