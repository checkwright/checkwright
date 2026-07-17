---
title: Site architecture
---

# The docs/ site architecture

`docs/` is the public GitHub-Pages site (served from `docs/` on master via its
`CNAME`), repo-root-governed with no owning kit. This page is the load-triggered
home for the site's chrome, page-authoring rules, generated projections, and docs
gate roster — the mechanism CLAUDE.md §Housekeeping names by pointer rather than
carries inline. It is off-nav by design (`scripts/docs-offnav.list`): maintainer
governance reached by citation, not a reader-nav destination.

## Site chrome and the nav contract

The site chrome — the Jekyll layout, client-side search, and theme selector —
lives in `docs/_config.yml`, `docs/_layouts/`, `docs/_includes/`, and
`docs/assets/`. The nav is Liquid over front matter: `nav_order` / `nav_parent`
place a page; a page's `nav_id` parents its `nav_child_order`-sorted children,
each child carrying derived suffix links to its `generated:`-marked directory
siblings. A nav page naming a `nav_children_key` instead derives its children from
the site pages carrying that key, path-descending, each labeled by the key's
value (the release notes under the Releases page). `check-docs-nav-reachable`
holds every docs page to a `title:` front-matter block and reachability from the
rendered nav (a nav slot, a relative-link walk seeded from the nav set, or the
generated-sibling suffix rule), with `scripts/docs-offnav.list` the allowlist for
pages off-nav by design. `check-docs-render-fidelity` holds the rendered Liquid
against the front-matter facts it projects. The kit registry lives on
`docs/kits.md` (the Kit Reference page); `check-docs-kit-parity` holds every kit's
row there and the nav child block (`nav_parent: kits` + `nav_child_order`) on
every `docs/<kit>/index.md`.

## Page-authoring rules

A page's `title:` is its terse nav label; its opening H1 carries the descriptive
full form (nav stays scannable, the page reads whole). Living pages are governed
prose under the anti-restatement doctrine (cite downward, never restate a SPEC's
invariant); dated `docs/posts/` are immutable, temporal-exempt but still
link/command-resolved (`scripts/canon-config.sh`). A page off-nav by design joins
`scripts/docs-offnav.list` — an embedded data fragment no link targets, or
maintainer governance like this page reached only by citation.

## Generated projections and their freshness gates

Several docs surfaces are generated and byte-gated for freshness; each gate's red
output names its own regen command, so the command need not stay resident to be
recoverable:

- **The on-site SPEC mirror** (`docs/<kit>/SPEC.md`, `docs/<kit>/README.md`,
  `docs/doctrine-kit/DOCTRINE.md`) — regenerate after editing any kit
  SPEC/README/DOCTRINE: `bash scripts/gen-docs-mirror.sh --write`
  (`check-docs-mirror-fresh` byte-gates it).
- **The value rollup** — `docs/value.md` is hand-framed prose around one
  generated marker block that joins the enforcement-map's per-kit class counts to
  the footprint's per-kit token cost. Regenerate on any change either emitter
  reports: `bash scripts/gen-value-rollup.sh` (`check-value-rollup-fresh`
  byte-gates the block, the byte-fresh projection of `gen-value-rollup.sh
  --emit`). The join reads the two emitters live (never the committed detail
  pages, so a stale page cannot poison the rollup); the class taxonomy and its
  hardest-to-softest column order are owned by the enforcement page and derived
  from its `##` section headings; the cost columns are the footprint's per-kit
  token figure, and the totals row reuses the footprint's pre-summed token totals
  rather than re-summing; the kit axis follows the footprint roster, then any
  enforcement-only label (a surface under no kit) groups as `(consumer)`. It is a
  consumer docs ruling, not kit mechanism — the join axis and column choice live
  here, never in a kit. `docs/value.md` holds the nav slot; `docs/enforcement.md`
  and `docs/footprint.md` persist as its off-nav drill-downs, link-reachable from
  it.
- **The install-toolchain parity contract** — `docs/install.md`'s Requirements
  section holds the toolchain list to env-probe's probe set:
  `check-install-toolchain` asserts name-set parity between its
  `<!-- toolchain:begin -->` bullets and `context-kit/bin/env-probe.sh`'s
  `PROBE_SET` array both directions (names derivable, purpose clauses hand prose),
  so a probe-set edit reds the docs list without an emitter handshake.
