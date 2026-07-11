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
3. [Enforcement map](enforcement.md) — what each kit enforces, and how hard:
   the class of every check surface, emitted from the registries.
4. The [kit map](#the-kits) below — one page per kit, in reading order.
5. [Announcing Checkwright](posts/2026-07-09-announcing-checkwright.md) — the
   launch note.

## The kits

Read in this order: each kit assumes the machinery of the ones above it.

- [gate-sdk](gate-sdk/index.md) — the lint framework the other kits register into.
- [lifecycle-kit](lifecycle-kit/index.md) — the iteration stage state machine for
  stateless sessions.
- [queue-kit](queue-kit/index.md) — the git-native, agent-readable task tracker.
- [canon-kit](canon-kit/index.md) — spec discipline for agent-authored components.
- [guard-kit](guard-kit/index.md) — permission-friction tooling for agent sessions.
- [delegation-kit](delegation-kit/index.md) — safe delegated-agent execution under a
  token budget.
- [context-kit](context-kit/index.md) — token-economics-aware context management.
- [drift-kit](drift-kit/index.md) — advisory drift reporting across the governed
  surfaces.
- [evidence-kit](evidence-kit/index.md) — a held-constant test baseline and a
  committed per-run evidence manifest.
- [site-kit](site-kit/index.md) — deployment-truth governance for a repo-served
  docs site: a CNAME-host parity gate and a live-site monitor template.
- [doctrine-kit](doctrine-kit/index.md) — the experience-packaging rung: the
  cross-kit delivery doctrine, referenced by link into a consumer's
  always-loaded agent file.

The walkthrough and evidence pages join this map when their kits land.

## License

Checkwright is Apache-2.0. Adoption is the goal.
