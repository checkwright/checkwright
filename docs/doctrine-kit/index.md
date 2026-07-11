# doctrine-kit

The experience-packaging rung. The delivery doctrine the other kits enforce
piecemeal — content-tiering, enforcement-first, de-literalization, the
always-loaded shape, load-trigger residency, widest-true-tier placement,
oracle-first — becomes one customer-deliverable rules file, `DOCTRINE.md`,
carried into a consumer's project by reference.

The deliverable — `DOCTRINE.md` — is referenced in place, never copy-installed:
a consumer's always-loaded agent file gains a one-line-per-rule digest and a
markdown link to the vendored doctrine, and re-vendoring the kit *is* the
doctrine upgrade. A copied doctrine drifts; a linked one cannot.

The installer — `install-doctrine.sh` — inserts or replaces that reference block
between fixed markers, so a re-run is idempotent and a harness-less consumer can
paste the block by hand.

The gate — `check-doctrine-registration` — holds the block present: it asserts
the configured agent file carries the markdown link, and fails closed when the
agent file is missing. What the digest says is the consumer's to edit; the gate
asserts only the link, so a consumer that rejects a rule trims its own digest.

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

The rule statements live in the kit's `DOCTRINE.md`; the installer contract, the
gate invariant, and every knob are defined in its `SPEC.md`. Back to the
[kit map](../index.md#the-kits).
