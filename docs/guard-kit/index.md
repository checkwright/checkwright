# guard-kit

Permission-friction tooling for agent sessions. An agent that stops to ask
permission for every routine command wastes turns and trains its supervisor to
rubber-stamp; one that is allowed too much is unsafe. guard-kit steers the
middle: a pre-tool guard that blocks, redirects, rewrites, or auto-allows a
call by rule.

It ships a harness-generic ruleset, a scanner for where prompts originate,
tracked-versus-local allowlist curation, an optional wakeup guard, and a
close-stage step that triages the friction a session actually hit. It registers
no gates.

## Install

Vendor the `guard-kit/` directory into your repo and wire its guard as your
harness's pre-tool hook. The ruleset and allowlists are external configuration.

## Quick start

```bash
bash guard-kit/bin/run-guard-tests.sh                # exercise the decision table
```

## Contracts

The guard decision model and the friction-triage step are defined in the kit's
`SPEC.md`; its `README.md` lists the mechanism. Back to the
[kit map](../index.md#the-kits).
