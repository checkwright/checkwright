# SPEC amendment: template-governance

## What changes

**Ruling: finder discriminator, not a stated manual duty.** The escaping
class — slot-free kit-template markdown and other non-manifest governed
prose (`.claude/agents/*.md` agent definitions) — is mechanically decidable:
the routing unit shipped both poles as live examples
(`lifecycle-kit/templates/lead.md` slot-bearing, would false-positive;
`.claude/agents/stage-session.md` slot-free and citation-dense, passes
clean), so the enforcement-first false-positive carve-out does *not* apply
and a duty would under-deliver what a gate can decide.

**Mechanism (kit) / candidates (consumer) — the seam.** canon-kit's manifest
finder gains a derived inclusion step: a new consumer knob
`CANON_KIT_PROSE_SURFACE_GLOBS` (array of globs, default empty — the
provenance posture: which surfaces a consumer governs is their config)
names *candidate* files; the finder includes a candidate in the manifest
set **iff it is slot-free** — it contains no binding-slot span (the
`*<name: …>*` grammar lifecycle-kit/SPEC.md §templates/skills/ owns) and no
`CONSUMER BINDING` header. The discriminator is kit mechanism (it encodes
the slot grammar, which the kit owns); the candidate list is consumer
config. A slot-bearing candidate is silently excluded — its coverage remains
the purpose-built shim/binding gates.

**Coverage gained.** Included files join the manifest set wholesale, so the
two axes that actually escape today — §heading-fragment liveness
(`check-spec-pointer`'s prose citations) and command/knob citation
(`check-docs-cmd`) — plus the remaining manifest doc gates run on them.
Slot-free finished prose is expected to pass; a failure it surfaces is a
real defect, which is the point.

**This repo's config.** `scripts/canon-config.sh` sets the globs to
`*/templates/*.md` and `.claude/agents/*.md`. Stage-skill shims under
`.claude/commands/` stay out: they are consumer bindings governed by
`check-skill-binding`/`check-shim-restatement`, and their `## Bindings`
bodies are residue, not finished kit prose.

**Fixtures.** The finder change lands with a gate-test on one representative
manifest gate (`check-spec-pointer`): a good case whose slot-bearing
candidate carries a dangling citation (excluded, passes) and a bad case
whose slot-free candidate carries the same dangling citation (included,
fails). The implicit exclusion becomes an explicit, ruled surface.

## Producers and consumers

- **Producer:** the finder (canon-kit lib), at every gate run that resolves
  the manifest set; the knob is set in the consumer's canon config, which
  every manifest gate already sources.
- **Consumer:** every manifest-keyed gate — they read the widened set with
  no per-gate change.
- **Knob reader:** `CANON_KIT_PROSE_SURFACE_GLOBS` is read by the finder
  alone; documented in canon-kit/SPEC.md §Layout and configuration's knob
  roster with its default and the discriminator semantics.

## Existing sections updated

- `canon-kit/SPEC.md` — §Layout and configuration gains the knob and the
  discriminator contract; the finder's contract section states the
  slot-free rule and cites lifecycle-kit's slot grammar as the owner of the
  slot syntax.
- `scripts/canon-config.sh` — this repo's candidate globs.
- `canon-kit/gate-tests/` — the representative fixture pair.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section; the merged doc reads alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component.
- [ ] **Removals propagated** — nothing retired.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed
      as debt tasks.
