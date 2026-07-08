# SPEC amendment: kit-registration

## What changes

A new gate, `check-kit-registration`, closes the prose-registry gap
`check-kit-enum` leaves open: that gate guards gate-coupling hand-lists, so a
landed kit can silently fall out of the human-facing registry docs. Invariant:
every kit root `gate_kit_roots` enumerates is registered in the consumer's
docs. Two assertions:

- **(A) registry row** — the registry doc carries a markdown link whose
  target is the kit root (`](<kit>/)` for each root, repo-root-relative);
  a landed kit missing from the public kit table is red.
- **(B) fixture-runner line** — every kit root with tracked `gate-tests/`
  files has a line in the agent-instructions doc naming `<kit>/gate-tests`
  (the fixture-runner invocation); a kit whose fixtures never entered the
  documented battery is red. A kit without `gate-tests/` (guard-kit,
  drift-kit) owes nothing under B.

Config, platform-pattern shape (`<KIT>_<KNOB>`, values as defaults):

- `GATE_SDK_REGISTRY_DOC` (default `README.md`) — assertion A's doc.
- `GATE_SDK_RUNNER_DOC` (default `CLAUDE.md`) — assertion B's doc.

Fail-closed: a configured doc that does not exist is a misconfiguration
(exit 2, like `check-kit-enum`'s missing registry), not a pass. A consumer
keeping no prose registry opts out by not registering the gate in its
`gates.list` — no empty-knob valve.

The gate retires the manual "does the kit table still reflect the kit set?"
staleness check from the close ritual; the close skill's checklist narrows
that line to the un-gated remainder (row descriptions, per-kit READMEs)
when this merges.

## Producers and consumers

- Producer: the pre-commit hook and `run-gates.sh`, once the gate's name is
  added to `scripts/gates.list` (this repo registers it at land time).
- Consumer: the committing session — findings name the unregistered kit root
  and the doc it is missing from.
- Inputs read: `gate_kit_roots_rel` (roster), `git ls-files <kit>/gate-tests/`
  (assertion B's precondition), the two configured docs. No new state or
  message fields.

## Existing sections updated

- gate-sdk SPEC §Per-component contracts gains the `check-kit-registration`
  contract section (this delta).
- gate-sdk SPEC §Layout and configuration knob table gains the two knobs.
- `.claude/commands/close.md` step 5: kit-table membership and the
  fixture-runner lines become gate-held — reword the step to the un-gated
  remainder (row descriptions, CLAUDE.md prose, per-kit READMEs), citing
  the gate. lifecycle-kit's close skill template carries only a generic
  staleness line — no edit owed.

Ships with a `good/`+`bad/` fixture pair (synthetic kit roots via
`GATE_SDK_KIT_DIRS`, per the four gate contracts) and a `# graph:` manifest
line coupling `README.md,CLAUDE.md,kit:gate-tests/*`; regenerate the
pre-commit hook and CHECK-GRAPH artifact on land.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
