# doctrine-kit

The experience-packaging rung: the cross-kit delivery doctrine the other kits
enforce piecemeal, stated once in a customer-deliverable rules file and carried
into a consumer's project by reference.

The deliverable — [`DOCTRINE.md`](DOCTRINE.md) — carries each rule as its
statement, why it holds under coding-agent work, and a pointer to the kit
mechanism that enforces it. It is referenced in place, never copy-installed: a
consumer's always-loaded agent file gains a one-line-per-rule digest and a
markdown link to the vendored doctrine, and re-vendoring the kit *is* the
doctrine upgrade. See [SPEC.md](SPEC.md#the-doctrine-deliverable) for why that
reference-not-copy boundary is the mechanism.

The installer — `bin/install-doctrine.sh` — inserts or replaces the reference
block between fixed markers, idempotently. The gate —
`check-doctrine-registration` — holds the block present: it asserts the
always-loaded file carries the markdown link and fails closed when that file is
missing.

The doctrine ships the rule *statements* only. Each kit's SPEC owns its
mechanism and knob rosters, cited from the doctrine and never restated — so no
private rule content crosses the provenance seam.

## Install

Vendor the kit beside [gate-sdk](../gate-sdk/) (required), then:

1. Install the reference block — with your always-loaded agent file present:

   ```
   bash doctrine-kit/bin/install-doctrine.sh
   ```

   It writes (or updates) the `## Delivery doctrine` block in `CLAUDE.md`,
   linking `doctrine-kit/DOCTRINE.md`. A harness-less consumer can paste the
   marker block by hand instead — the installer is only its generator. Point
   `DOCTRINE_KIT_AGENT_FILE` / `DOCTRINE_KIT_DOCTRINE_FILE` at your own paths if
   they differ from the defaults.

2. Register the gate — add to your `gates.list`:

   ```
   check-doctrine-registration
   ```

   Regenerate the hook + graph artifacts: `bash gate-sdk/bin/gen-pre-commit.sh --write`.

3. Edit the digest — trim any rule your project does not keep resident. The gate
   asserts only the link, so the block body is yours to shape.

## Test

```bash
bash gate-sdk/bin/run-gate-tests.sh doctrine-kit/gate-tests doctrine-kit/checks
```
