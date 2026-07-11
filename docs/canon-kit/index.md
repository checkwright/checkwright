# canon-kit

Spec discipline for agent-authored components. When agents write both the code
and the prose that describes it, the two drift silently: the spec restates what
the code already says, the restatement rots, and no stateless session notices.
canon-kit removes the copy.

The kit keeps one canonical spec per component, carries changes as short-lived
amendment files that merge and delete, and enforces a content-tiering star
topology — one owner per fact, cite rather than restate. Its gates target the
copy-shaped failure modes directly.

## Install

Vendor the `canon-kit/` directory into your repo and register its gates in
`gates.list`. The manifest set, section names, and tiering knobs are external
configuration.

## Quick start

```bash
bash gate-sdk/bin/run-gate-tests.sh canon-kit/gate-tests canon-kit/checks
```

## Contracts

The spec model — canonical-plus-amendment lifecycle, the tiering topology, and
the anti-restatement doctrine — is defined in the kit's
[`SPEC.md`](https://github.com/checkwright/checkwright/blob/master/canon-kit/SPEC.md#the-spec-model);
its [`README.md`](https://github.com/checkwright/checkwright/blob/master/canon-kit/README.md)
lists the mechanism. Back to the [kit map](../index.md#the-kits).
