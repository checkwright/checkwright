# SPEC amendment: docs-nav-suffixes

## What changes

The rendered nav is one child level deep, so each kit's mirrored ground-truth
pages (`docs/<kit>/SPEC.md`, `docs/<kit>/README.md`,
`docs/doctrine-kit/DOCTRINE.md`) sit two hops from anywhere — kit index, then
its Contracts links. Each kit's nav child gains **compact sibling suffix
links** (`queue-kit [readme|spec]`), putting the mirrors one click from every
page without a second nav level.

- **Derived, never annotated.** `docs/_includes/nav.html` discovers a child's
  suffix set from what already exists: the site pages sharing the child
  page's directory, excluding the child itself, that carry the mirror front
  matter (`generated: true` — the marker `scripts/gen-docs-mirror.sh` already
  stamps). Labels are the pages' lowercased `title:` values (`readme`,
  `spec`, `doctrine`), title-sorted for a stable order. No new front-matter
  key, no per-kit maintenance: a new mirrored page appears in the nav the
  commit its mirror lands.
- **`current` extends to suffix pages.** When the open page *is* a suffix
  page, its suffix link carries the `current` class and the owning child
  `<li>` carries it too — today a SPEC page highlights nothing, so the
  reader loses their nav position exactly on the ground-truth pages.
- **Styling.** A `nav-suffix` class in the site's inline style block
  (`docs/_layouts/default.html` — the site has no separate stylesheet;
  `docs/assets/` holds only the logo and search script), visually
  subordinate (smaller, muted, bracketed) so the
  child label stays the scannable unit — the nav-label terseness rule
  (CLAUDE.md §Housekeeping) is unaffected.
- **The gate residual is paid down in the same unit.**
  `scripts/check-docs-nav-reachable.sh` models the include (front-matter
  slots + a link walk); every nav.html feature it does not model widens the
  gate-vs-include divergence stamped as residual at the prior close. The
  suffix rule joins the gate's reachability seeding: a page whose front
  matter marks it generated and whose directory-sibling `index.md` is
  nav-reachable is itself reachable. Today the mirrors ride the link walk
  incidentally (a kit index dropping its Contracts section would red them
  despite the nav still reaching them); encoding the rule makes the gate
  model the include's actual features instead of their side effects. Ruled
  against the alternative (leave the gate alone, divergence grows by one
  feature): the encode is a few lines against a stable discovery rule, and
  derivation-first favors the gate deriving the same set the include does.

## Producers and consumers

- **Suffix links** — producer: the nav.html Liquid loop at render time, from
  page dirs + the `generated: true` front matter; consumer: the site reader
  on every page. The discovery inputs' own producer is
  `scripts/gen-docs-mirror.sh` (already emits the front matter;
  `check-docs-mirror-fresh` keeps it existing).
- **`current`-on-suffix** — producer: the same loop comparing the suffix
  page's URL to `page.url`; consumer: the stylesheet's existing `current`
  styling plus the new `nav-suffix` rules.
- **The gate's sibling rule** — producer: the `check-docs-nav-reachable.sh`
  seeding edit; consumer: the gate's own reachability verdict; its fixture
  pair gains a generated-sibling case both directions (reachable via
  sibling-index; orphaned generated page with no nav-reachable sibling index
  stays red).
- No new front-matter keys, no new knobs, no new files — the CSS lines join
  the layout's existing inline style block.

## Existing sections updated

- CLAUDE.md §Housekeeping (the docs-chrome paragraph): the nav description
  ("a page's `nav_id` parenting its `nav_child_order`-sorted children")
  gains the suffix-link clause — one line, the mechanism stays in nav.html.
- `scripts/docs-offnav.list`: unchanged — no page changes off-nav status;
  the mirrors become structurally reachable rather than walk-reachable.

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
