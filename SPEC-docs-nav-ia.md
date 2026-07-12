# SPEC amendment: docs-nav-ia

A docs-site ruling with no owning kit (the `docs/` surface is
repo-root-governed), so this amendment lives at the repo root — the
SPEC-ddd-positioning precedent. It owns the target left-nav layout; the
sibling docs amendments (SPEC-value-rollup.md, SPEC-harness-positioning.md)
cite their nav slots from here rather than re-ruling them.

## What changes

**Part (a) — the Kit Reference regroup.** The per-kit pages leave the nav's
top level and nest under one parent, and the flatten fork is taken: the nav
include renders one nesting level only, so the kit mirror pages drop off the
nav tree rather than the include growing a deeper walker.

- `docs/kits.md` — the Kit Reference landing page, created by this unit:
  front matter `title: Kit Reference`, `nav_id: kits`, and the top-level
  `nav_order` slot from the target layout below. Its body takes over the kit
  registry list from `docs/index.md` (which keeps a one-line pointer to it —
  content-tiering: the registry lives on one surface). The registry-doc
  argument of `scripts/check-docs-kit-parity.sh` re-points at `docs/kits.md`
  so the wrapped kit-registration invariant (gate-sdk/SPEC.md
  §check-kit-registration) follows the table.
- Every `docs/<kit>/index.md` front-matter block becomes `title` +
  `nav_parent: kits` + `nav_child_order: <n>` — the child slot, preserving
  the kits' relative order from their prior `nav_order` values. `nav_order`
  and `nav_id` are dropped: the page is no longer top-level, and nothing can
  render beneath it under the one-level include.
- `scripts/gen-docs-mirror.sh` stops emitting `nav_parent`/`nav_child_order`
  on the mirror pages (`docs/<kit>/SPEC.md`, `README.md`, `DOCTRINE.md`) —
  dead front matter once their parent is itself a child. Mirror pages are
  reached from the kit index page's body links and client-side search. The
  mirror is regenerated in the same commit (`check-docs-mirror-fresh`).
- `scripts/check-docs-kit-parity.sh` — `has_nav_block` re-targets the block
  this amendment defines (`nav_parent: kits` + `nav_child_order`) in lockstep
  with the regroup; its fixture pair moves with it.

**Target top-level layout** (owned here): Home, Why Checkwright,
Orchestration, Domain-driven design, Where Checkwright sits
(SPEC-harness-positioning.md), Install, Value (SPEC-value-rollup.md),
Kit Reference. The Footprint page leaves the top level; its disposition is
SPEC-value-rollup.md's ruling.

**Part (b) — the reachability gate.** `scripts/check-docs-nav-reachable.sh`,
a consumer gate (the nav semantics live in this repo's chrome,
`docs/_includes/nav.html`, so the gate is consumer-local like
`check-docs-kit-parity`, not kit mechanism). Two assertions over every
tracked `docs/**/*.md` outside underscore-prefixed dirs:

1. **Front-matter floor** — the page opens with a front-matter block carrying
   `title:`, or is allowlisted as an embedded fragment. This is the defect
   class the enforcement map shipped with: a page with no front matter
   renders without the site layout and joins no nav.
2. **Reachability** — the page is in the rendered-nav set (`nav_order`
   top-level, or `nav_parent` resolving to a top-level page's `nav_id` —
   mirroring the include's one-level semantics; a change to the include's
   semantics moves this gate in lockstep), or is reached by a breadth-first
   walk over relative markdown links seeded from that set, or is allowlisted.

The allowlist is `scripts/docs-offnav.list` (one repo-relative path per
line, `#` comments), seeded with `docs/evidence-data.md` — an embedded data
fragment no link targets by design. Registered in `scripts/gates.list`,
precommit tier, with a `good/`+`bad/` fixture pair under
`scripts/gate-tests/` and a `# graph:` manifest coupling `docs/` and the
allowlist.

**The enforcement page's front matter** — `gate-sdk/bin/enforcement-map.sh
--emit` gains a front-matter header (`title: Enforcement map`) so the emitted
page passes assertion 1 without hand-editing a generated artifact;
`docs/enforcement.md` is regenerated in the same commit
(`check-enforcement-fresh`).

## Producers and consumers

- New gate: produced by this unit under `scripts/`, invoked by the generated
  pre-commit hook via the registry (regenerate via `gen-pre-commit.sh` and
  the graph artifact, per CLAUDE.md); its readers are committers and the
  fixture runner.
- The allowlist: hand-maintained by whoever lands an off-nav fragment; read
  only by the new gate.
- The nav front matter: produced by page authors and `gen-docs-mirror.sh`;
  read by `docs/_includes/nav.html` and the new gate (same semantics, stated
  coupling above).
- No new fields beyond the two front-matter keys already in the chrome's
  vocabulary; no new knobs.

## Existing sections updated

- CLAUDE.md §Housekeeping — the sentence naming `check-docs-kit-parity`'s
  expected nav block updates to the new block shape and registry page.
- `docs/index.md` — kit registry list replaced by a pointer to `docs/kits.md`.
- `scripts/check-docs-kit-parity.sh` header prose — registry doc becomes
  `docs/kits.md`.

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
