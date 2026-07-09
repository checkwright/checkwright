# evidence-kit

A held-constant test baseline and a committed per-run evidence manifest for the
validate stage. A stage stamp proves a stage was invoked; this proves it
produced a green result. Together they let a reviewer — or an external
verifier — trust that the checks actually ran, not just that someone said so.

The manifest is a versioned wire contract an outside party can consume, which
is why the kit exists as a distinct surface: it is the seam a hosted attestation
service would verify. It ships the validate runner and baseline-diff tool, and
gates over baseline grammar, slug liveness, and manifest grammar.

## Install

Vendor the `evidence-kit/` directory into your repo, register its gates in
`gates.list`, and declare your validate suites in its external config.

## Quick start

```bash
bash gate-sdk/bin/run-gate-tests.sh evidence-kit/gate-tests evidence-kit/checks
```

## Contracts

The baseline grammar and the versioned manifest contract are defined in the
kit's `SPEC.md`; its `README.md` lists the mechanism. Back to the
[kit map](../index.md#the-kits).
