# SPEC amendment: footprint-page

## What changes

The kits' measured context footprint becomes a published, generated docs
page — the adoption-cost evidence a consumer evaluates before vendoring, and
the concrete form of the token-economics positioning.

**Extractor-home ruling: context-kit.** The kit owns the metric (§The
always-loaded meter, §The consumer footprint). New `bin/footprint.sh`:

- **Measures two tiers per kit:** the always-loaded share (agent-file
  injected blocks the kit generates, session-hook output it contributes) and
  each load-triggered surface (skill/template markdown, loaded only on its
  trigger), reusing the always-loaded meter's counting for the first tier.
- **Numbers ruling:** line and word counts exact; token counts a *labeled
  estimate* — the page publishes the method (bytes/4 heuristic, stated
  inline as model-tokenizer-dependent) and never a false-precision number.
- **Attribution ruling: kit share only.** A kit's advertised cost is what
  the kit ships — templates, hooks, generated blocks. Consumer bindings,
  consumer config, and this repo's own CLAUDE.md residue are excluded, and
  the page states the exclusion so the number is honest.
- Emits the full page to stdout (`--emit` symmetry with the sibling
  emitters); totals plus per-kit split.

**Page + freshness.** `docs/footprint.md` is generated
(`bash context-kit/bin/footprint.sh --emit > docs/footprint.md`) and
byte-gated by new `check-footprint-fresh` (context-kit/checks/; the
`check-docs-mirror-fresh` posture). Registered in `gates.list` with a
`# graph:` manifest coupling the measured surfaces; skeleton-contract
fixture pair ships with it. Where the generated numbers would trip the
prose gates (bare cardinals, temporal phrasing), the page follows the
`docs/evidence-data.md` valve precedent — a generated, freshness-gated
projection is Derivation-first's sanctioned copy.

**Placement ruling: evidence framing.** The page rides the docs site's
evidence family (beside the trajectory page), with nav front matter, and
`docs/install.md` links it where vendoring cost is weighed. It complements —
never restates — §The consumer footprint's budget rule: the SPEC owns the
budget doctrine, the page owns the measured numbers.

## Producers and consumers

- **Producer:** `bin/footprint.sh`, run by the maintainer after a change to
  a measured surface; the freshness gate is what makes the re-run
  non-optional (a stale page reddens the battery).
- **Consumer:** evaluating adopters reading the docs site; the freshness
  gate as the page's second reader.
- **Fields/readers:** per-kit rows (adopter's per-kit vendoring decision),
  the method label (reader's calibration of the estimate), totals (the
  headline number `docs/install.md` links to). No unread field.
- **Existing integration prose:** context-kit/SPEC.md gains §bin/footprint
  and §check-footprint-fresh contracts; §The consumer footprint gains one
  pointing sentence to the published page.

## Existing sections updated

- `context-kit/SPEC.md` — new component contracts; §The consumer footprint
  pointer; §Layout and configuration if a knob emerges (none planned — the
  measured set derives from the kit roster).
- `docs/install.md` — adoption-cost link.
- `scripts/gates.list`, generated pre-commit hook, `docs/check-graph.html`,
  `docs/enforcement.md` — regenerated for the new gate.
- `docs/index.md` kit rows / nav — only as `check-docs-kit-parity` demands.

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
