# delegation-kit

Safe delegated-agent execution for budget-bounded sessions. Delegating work to
sub-agents is the primary way a supervisor session stays within its token
budget — but a sub-agent that shares the git index, weakens a gate to get
past it, or reports a false pass turns a saving into a hazard. delegation-kit
is the protocol that makes delegation safe.

It supplies the supervisor rules (serialize on the shared index, one commit per
unit, a resume journal, validate after every agent commit), a trustworthy
budget verdict, and a commit-shape gate that blocks the attested
gate-weakening shapes.

## Install

Vendor the `delegation-kit/` directory into your repo, register its gate in
`gates.list`, and point the budget tool at your usage source.

## Quick start

```bash
bash delegation-kit/bin/usage-verdict.sh             # one budget verdict: OK / PAUSE / STALE
```

## Contracts

The supervisor protocol and the tamper-gate contract are defined in the kit's
`SPEC.md`; its `README.md` lists the mechanism. Back to the
[kit map](../index.md).
