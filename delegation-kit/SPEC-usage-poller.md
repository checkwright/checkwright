# SPEC amendment: usage-poller

## What changes

The snapshot's only shipped producer is the statusline hook, which fires on
the supervising session's own message flow — so a lead that delegates goes
blind exactly when it is static: a 6.7-hour delegated build ran with zero
snapshot updates (trend-log evidence, 2026-07-13), and every per-dispatch
budget verdict in that window read a stale file. The usage.txt contract is
producer-pluggable by design; this amendment ships the second reference
producer, **`templates/usage-poller.sh`** — a timer-driven poller independent
of message flow.

- **What it does.** One poll cycle per invocation: read the harness OAuth
  token, query the account usage source, map the payload onto the snapshot
  contract (the three mandatory lines plus whichever optional keys the source
  exposes), and atomically rewrite the snapshot (`tmp` + `mv`, same discipline
  as the statusline producer). No daemon, no loop: scheduling belongs to the
  consumer's timer (a cron line or systemd timer — the SPEC shows one example
  wiring), keeping the kit free of service management.
- **Credentials.** Read-only read of the harness credentials file the kit
  already knows (`DELEGATION_KIT_CRED_FILE`); the token goes into the request
  header and nowhere else — never logged, never echoed, never written to the
  snapshot. A missing or unreadable credentials file is a non-zero exit with
  a `help:` line, snapshot untouched.
- **Source stability (the named risk).** The usage endpoint is
  harness-account plumbing, not a published contract, so the poller is
  **fail-soft**: any fetch or parse failure exits non-zero *without touching
  the snapshot* — a stale snapshot is already the detected condition
  downstream (`usage-verdict`'s `updated_at` staleness check turns it into a
  STALE verdict, never a silent green). The endpoint URL is a knob,
  **`DELEGATION_KIT_USAGE_ENDPOINT`** (default: the shipped source), which is
  both the test seam (fixtures point it at a local stub) and the valve when
  the source moves.
- **Coexistence with the statusline producer.** Both write the whole snapshot
  atomically from the same account source; last-writer-wins is correct
  because the freshest write is the truest and `updated_at` arbitrates
  downstream. Nothing serializes them and nothing needs to.
- **Trend log unchanged.** `usage-verdict` stays the single append author of
  the sample log; the poller writes the snapshot only.

## Producers and consumers

- **The snapshot rewrite** — producer: `templates/usage-poller.sh` under a
  consumer-wired timer (the enabling config is the consumer's timer entry,
  named in the SPEC's wiring example — without it the producer is dead and
  the SPEC says so); consumer: `usage-verdict` at every verdict call, the
  statusline render at every refresh. All keys written are existing contract
  keys with existing named readers; the poller adds **no new key**.
- **`DELEGATION_KIT_USAGE_ENDPOINT`** — producer: consumer config (default in
  the template); reader: the poller's fetch step. Joins the knob roster in
  delegation-kit/SPEC.md §Layout and configuration.
- **The non-zero fail-soft exit** — consumer: the invoking timer's logging;
  the design explicitly does *not* route poller failures anywhere else — the
  snapshot's staleness is the in-band signal the existing readers already
  interpret.

## Existing sections updated

- delegation-kit/SPEC.md §The usage.txt contract: the producer paragraph
  ("`templates/statusline-usage.sh` is the reference producer") generalizes
  to two shipped producers with the push/poll distinction and the
  coexistence rule.
- delegation-kit/SPEC.md §Layout and configuration: knob roster gains
  `DELEGATION_KIT_USAGE_ENDPOINT`.
- delegation-kit/SPEC.md §Testing: the poller's stub-endpoint smoke joins the
  suite (happy path writes a contract-valid snapshot; fetch failure leaves a
  pre-seeded snapshot byte-identical).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls <component>/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
