---
title: Kit Reference
nav_id: kits
nav_order: 7
---

# Kit Reference

One page per kit.

## The kits

Start with gate-sdk. Its binary is the runner, the registry, the git hooks and the commands the other kits call, so every install carries it. The rest follow in reading order, not dependency order: each installs beside gate-sdk alone.

- [gate-sdk](gate-sdk/index.md) — the lint framework the other kits register into.
- [lifecycle-kit](lifecycle-kit/index.md) — the iteration stage state machine for stateless sessions.
- [queue-kit](queue-kit/index.md) — the git-native, agent-readable task tracker.
- [canon-kit](canon-kit/index.md) — spec discipline for agent-authored components.
- [guard-kit](guard-kit/index.md) — permission-friction tooling for agent sessions.
- [delegation-kit](delegation-kit/index.md) — safe delegated-agent execution under a token budget.
- [context-kit](context-kit/index.md) — token-economics-aware context management.
- [drift-kit](drift-kit/index.md) — advisory drift reporting across the governed surfaces.
- [evidence-kit](evidence-kit/index.md) — a held-constant test baseline and a committed per-run evidence manifest.
- [site-kit](site-kit/index.md) — deployment-truth governance for a repo-served docs site: a CNAME-host parity gate, a rendered-page fidelity gate, and a live-site monitor template.
- [doctrine-kit](doctrine-kit/index.md) — the experience-packaging rung: the cross-kit delivery doctrine, referenced by link into a consumer's always-loaded agent file.
