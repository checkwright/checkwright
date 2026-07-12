# SPEC amendment: value-rollup-page

A docs-site ruling with no owning kit, so this amendment lives at the repo
root. It consolidates the enforcement map and the footprint into one
nav-visible "these benefits at this token cost" page and rules the three
forks the queue entry named. Its nav slot is assigned by
SPEC-docs-nav-ia.md's target layout.

## What changes

- `docs/value.md` — the value page: front matter `title: Value`, the
  top-level nav slot from SPEC-docs-nav-ia.md. Hand-authored framing prose
  (the value proposition: what the gates hold, at what context cost) around
  one marker-bounded generated block — the env-probe marker precedent
  (`value-rollup:begin` / `value-rollup:end` HTML comments), because the
  framing prose is hand content and the table is derived, and neither may
  own the other's tier.
- **Fork 1, the benefit metric** — the generated block is a per-kit table
  joining on the kit axis: per-kit gate counts *split by enforcement class
  as the enforcement map defines them* (the emitted enforcement page owns
  the class taxonomy — gate-sdk/SPEC.md §enforcement-map), joined to the
  footprint's per-kit always-loaded and load-triggered token cost
  (context-kit/SPEC.md §bin/footprint), with a totals row that totals
  within each column. Never one flattened tally: a blocking gate and an
  advisory surface are different promises and stay different columns. This
  absorbs the per-kit-gate-count idea — the tally is the benefit column,
  not a footprint add-on.
- **Fork 2, generation** — `scripts/gen-value-rollup.sh` is the joining
  emitter: it invokes the two existing emitters (`enforcement-map.sh
  --emit`, `footprint.sh --emit`), derives the join from their emitted
  tables, and rewrites the marker block in place. It is a consumer script,
  not kit mechanism: the join axis and column choice are this repo's docs
  ruling. Freshness is `scripts/check-value-rollup-fresh.sh` — byte-compares
  the committed marker block against a fresh emission, the
  `check-enforcement-fresh` pattern — registered in `scripts/gates.list`,
  precommit tier, `good/`+`bad/` fixtures under `scripts/gate-tests/`, a
  `# graph:` manifest coupling the page to both emitters' registries. Never
  a hand-stitched page (derivation-first).
- **Fork 3, survival of the detail** — the per-gate enforcement detail does
  not collapse into the page. `docs/enforcement.md` and `docs/footprint.md`
  persist as drill-down references linked from `docs/value.md`; the
  footprint page gives up its top-level `nav_order` (title front matter
  stays), so both are link-reachable under SPEC-docs-nav-ia.md's
  reachability gate without holding nav slots. No number is duplicated
  across surfaces by hand: the only figures on the value page are the ones
  its emitter derives.

## Producers and consumers

- The marker block: produced by `gen-value-rollup.sh` (run by the landing
  session and on any change to what the two source emitters report);
  consumed by page readers and byte-checked by `check-value-rollup-fresh`.
- The join inputs: the existing emitters, invoked live — the joiner never
  parses the committed pages, so a stale committed artifact cannot poison
  the join (each committed page has its own freshness gate already).
- `docs/index.md`'s pointer row and the nav include read the page like any
  other; no new knobs, no new front-matter vocabulary.

## Existing sections updated

- `docs/footprint.md` front matter — `nav_order` dropped (generated page:
  the change lands in `context-kit/bin/footprint.sh --emit` and the page is
  regenerated in the same commit, `check-footprint-fresh`).
- `docs/index.md` — the enforcement-map row in its numbered list re-points
  through the value page (the drill-down chain becomes value → detail).

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
