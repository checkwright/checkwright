# doctrine-kit

The experience-packaging rung. The delivery doctrine the other kits enforce
piecemeal becomes one customer-deliverable rules file, `DOCTRINE.md`, carried
into a consumer's project by reference. It holds two registers:
methodology-maintenance rules, which keep the methodology's own surfaces honest,
and engineering-craft rules, which govern how the work built under it is
written.

The deliverable — `DOCTRINE.md` — is referenced in place, never copy-installed:
a consumer's always-loaded agent file gains a one-line-per-rule digest and a
markdown link to the vendored doctrine, and re-vendoring the kit *is* the
doctrine upgrade. A copied doctrine drifts; a linked one cannot.

The installer — `install-doctrine.sh` — inserts or replaces that reference block
between fixed markers, so a re-run is idempotent and a harness-less consumer can
paste the block by hand.

The gate — `check-doctrine-registration` — holds the block honest: it asserts
the configured agent file carries the markdown link *and* keeps its
methodology-rule digest in per-rule lockstep with the doctrine, fail-closed when
a scanned file or heading is missing. A rule the consumer does not keep resident
is dropped by a declared trim marker beside the digest, never a silent deletion —
so a re-vendor that changes the rule set surfaces at the next commit.

The doctrine ships the rule statements only. Each kit's SPEC owns its mechanism
and knob rosters, cited from the doctrine and never restated — so no private
rule content crosses the provenance seam.

## Install

Vendor the `doctrine-kit/` directory into your repo, run
`bash doctrine-kit/bin/install-doctrine.sh` to write the reference block into
your always-loaded agent file, and register `check-doctrine-registration` in
`gates.list`. Point `DOCTRINE_KIT_AGENT_FILE` / `DOCTRINE_KIT_DOCTRINE_FILE` at
your own paths if they differ from the defaults.

## Quick start

```bash
bash gate-sdk/bin/run-gate-tests.sh doctrine-kit/gate-tests doctrine-kit/checks
```

## Contracts

The rule statements live in the kit's
[`DOCTRINE.md`](https://github.com/checkwright/checkwright/blob/master/doctrine-kit/DOCTRINE.md);
the installer contract, the gate invariant, and every knob are defined in its
[`SPEC.md`](https://github.com/checkwright/checkwright/blob/master/doctrine-kit/SPEC.md#check-doctrine-registration).
Back to the [kit map](../index.md#the-kits).
