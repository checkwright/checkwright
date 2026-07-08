# SPEC amendment: docs-cmd

## What changes

A new gate, `check-docs-cmd`, the command/knob analog of `check-md-refs`:
prose that quotes an invocation or a config knob drifts silently when the
script is renamed or the knob retired — a broken link is caught, a broken
`bash <path>` line is not. Invariant: every repo-path invocation and every
kit-prefixed env knob named in the governed doc set resolves against the
tree. Two assertions:

- **(A) command paths** — inside fenced code blocks, any word matching a
  repo-relative script path (`<dir>/…/<name>.sh`, and the direct
  `bash <path>` form) must be a tracked file (`git ls-files`); a doc
  quoting a command that no longer exists is red.
- **(B) env knobs** — any backticked or fenced ALL-CAPS name carrying a
  known kit prefix (the prefix roster derived from `gate_kit_roots`, e.g.
  `GATE_SDK_`, `SPEC_KIT_`, `EVIDENCE_KIT_`) must occur in that kit's
  tracked sources; a doc naming a retired or misspelled knob is red.
  Names with no kit prefix are out of scope (no false positives on
  generic shell vars).

Governed doc set: the same surface `check-md-refs` governs (shared config,
no second knob) — the gate is a sibling in the same family and follows its
calibration section's shape. Deliberately narrow: prose outside fences and
backticks is never scanned; a hypothetical-example path is written without
a fence or with the existing per-file exemption valve the md-refs family
provides (build resolves the valve's exact spelling against
`check-md-refs`' implementation).

Fail-closed per the four gate contracts; ships with a `good/`+`bad/`
fixture pair under `spec-kit/gate-tests/check-docs-cmd/` and a `# graph:`
manifest line coupling the governed doc set to `scripts/*.sh` and each
kit's `bin/`+`checks/`; regenerate the pre-commit hook and CHECK-GRAPH
artifact on land.

## Producers and consumers

- Producer: the pre-commit hook and `run-gates.sh`, once registered in
  `scripts/gates.list` (this repo registers it at land time).
- Consumer: the committing session — findings name the doc line and the
  unresolvable path or knob.
- Inputs read: the governed doc set (via spec-kit's existing finders),
  `git ls-files` (assertion A), kit sources under `gate_kit_roots`
  (assertion B). No new state or message fields.

## Existing sections updated

- spec-kit SPEC §Per-component contracts gains the `check-docs-cmd`
  contract section (this delta).
- spec-kit SPEC §check-md-refs notes the sibling split: links there,
  commands/knobs here, one shared governed set.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls spec-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
