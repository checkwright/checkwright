# Announcing Checkwright

*2026-07-09*

Checkwright is a coding-agent-assisted delivery methodology, packaged as
installable kits. It is open source under Apache-2.0, and it governs its own
repository with the same machinery it ships to you.

## The bet

When coding agents do the writing, discipline stops holding on its own.
Conventions that live in prose depend on a reader who remembers them, and a
stateless agent session is a reader who does not. The result is silent drift:
specs that no longer match their code, lists that fall out of sync with their
registries, stages of work quietly skipped.

Checkwright's bet is that most of this is mechanically decidable, and what is
mechanically decidable should be enforced by a gate that blocks the commit —
leaving only the genuinely semantic judgment to a human or agent. Discipline you
have to remember becomes discipline the tree refuses to break.

## What ships

A set of kits, each owning one axis and vendored into your repository whole: a
self-testing lint framework, an evidence-stamped iteration lifecycle for
stateless sessions, a git-native task queue, spec-consistency discipline,
permission-friction tooling, safe delegated execution under a token budget,
token-economics-aware context management, advisory drift reporting, and a
verifiable evidence baseline. Adopt the kits that pay for themselves.

The [methodology essay](../methodology.md) makes the full argument; the
[install guide](../install.md) shows the vendoring and upgrade flow; the
[kit map](../index.md) is the reading order.

## Where it is going

The kits are the open, permissive core. The road ahead runs through
server-side attestation — checks verified by a party your agents cannot touch —
and cross-repository drift dashboards. Those are for when adoption attests the
demand. For now: clone it, vendor a kit, and let the gates hold the line.
