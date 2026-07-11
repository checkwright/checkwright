# SPEC amendment: ddd-positioning

A docs-site ruling with no owning kit (the `docs/` surface is
repo-root-governed), so this amendment lives at the repo root — the
SPEC-docs-cname-parity precedent.

## What changes

- `docs/ddd.md` — a living positioning page: Checkwright as the enforcement
  layer for a DDD **ubiquitous language**. The argument the page owns:
  a shared vocabulary is exactly the kind of discipline prose cannot hold
  when stateless agent sessions do the writing — synonyms creep in one
  plausible word at a time — and vocabulary consistency is mechanically
  decidable, so it belongs to gates. The page positions; it owns no
  contract.
- The mechanism mapping, cited downward (the anti-restatement doctrine —
  each cited section keeps owning its invariant): banned-synonym
  enforcement via `check-tree-terms` and its consumer pattern files (the
  canonical term is used because its rivals cannot be committed);
  comment/naming directives via canon-kit's `check-comment-tier`; single
  ownership of each definition via canon-kit's content-tiering doctrine;
  and the `check-graph`/`graph-vocab.sh` pattern as the shape every
  vocabulary takes here — **consumer config, never a kit literal**. That
  last point is the page's honesty clause and the provenance seam in
  public form: the kits stay DDD-neutral, no kit gains domain vocabulary,
  and the coupling to DDD lives in this page and its example only.
- The example: a fenced consumer-config sketch for a **fictional**
  cargo-shipping domain (the classic DDD example domain — stated as
  fictional in the page, so no reader mistakes it for a shipped file or a
  real consumer): a pattern file banning the non-canonical synonyms
  (`parcel`, `shipment` where `cargo` is canonical) wired through
  `check-tree-terms`'s pattern-file argument. Embedded in the page, not
  shipped as a file — an `examples/` directory would be a new tree
  convention this unit does not need.
- `docs/index.md` — gains a `## Positioning` section between the kit map
  and the license, created by this unit with the DDD row; the sibling
  orchestration page (SPEC-orchestration-positioning.md) adds the second
  row. Governance is automatic: `docs/*.md` is already in
  `CANON_KIT_MANIFEST_FILES` (`scripts/canon-config.sh`), so the page's
  links and commands resolve under the doc gates from the first commit.

## Producers and consumers

- Producer: the build session authors the page; GitHub Pages serves it
  from `docs/` on master (the existing site mechanism — no new publishing
  config).
- Consumers: readers arriving from `docs/index.md`'s Positioning section;
  the doc gates (link/command resolution, temporal-narration, CNAME
  parity) consume it as they do every living page. The fenced example's
  reader is the evaluating adopter — it must be copy-adaptable against
  the cited `check-tree-terms` usage line without consulting kit source.
- No new fields, states, or config: the page cites existing knobs and
  gates by name and adds none.

## Existing sections updated

- `docs/index.md` — the new Positioning section (this unit creates it);
  no kit-map row changes (`check-docs-kit-parity` governs kit rows only,
  and this page is not a kit page).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls SPEC-*.md` at the repo root).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
