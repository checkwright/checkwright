# SPEC amendment: docs-site-chrome

The docs site rides the bare GitHub-Pages Primer default — no `_config.yml`,
no layouts — so every page is an unbranded column with no navigation, and
every SPEC reference exits the site for a github.com blob view, stranding the
reader outside the chrome. This unit ships the site chrome (left nav, search,
logo, theme selector) and the generated SPEC mirror that keeps reference
reading on-site. The docs tree is repo-root-governed with no owning kit, so
this amendment lives at the repo root; kit mechanism is untouched except one
prose narrowing in canon-kit noted below.

Supersession ruled here: the docs-reference-routing ruling ("point, don't
generate" — reference reading happens on the github.com tree) narrows. It was
taken when the alternative was a hand-maintained copy; the derivation-first
rule sanctions a generated, freshness-gated copy, and the navigation dead-end
is operator-attested. Post-merge: *rendered-document* references (SPEC,
README, DOCTRINE) go relative to the on-site mirror; self-repo blob links
remain the grammar for *source* references (scripts, gate bodies, config) —
files the site should not render.

## What changes

- **`docs/_config.yml` + `docs/_layouts/default.html` (+ `docs/_includes/`)**
  — a custom layout keeping the Primer stylesheet and adding: a header with
  the logo mark + wordmark linking home, the theme selector, a left sidebar
  (nav + search box), the content column, and a footer. All pages render
  through it, including the mirror.
- **Nav derives from front matter, never a hand roster.** The sidebar is
  Liquid over `site.pages` filtered on a `nav_order` front-matter key (pages
  without it — the dated posts, which are immutable and stay untouched — do
  not appear; they remain reachable from the index). A kit's mirrored pages
  nest under its kit entry via the generator-stamped front matter.
  Enforcement of the silent-drop gap: `scripts/check-docs-kit-parity.sh`
  (consumer gate) gains the assertion that every `docs/*/index.md` carries
  the nav front-matter block — a kit page absent from the nav is red, not
  invisible.
- **Search** — a Liquid template emits `search.json` over the site's pages at
  Pages build; a small vendored `docs/assets/search.js` (no CDN, no external
  service) filters it client-side from the sidebar box. Build-time output is
  not committed and not gated — the tree holds only the template.
- **Theme selector** — light/dark/auto in the header; `prefers-color-scheme`
  is the auto default, the explicit choice persists in the localStorage key
  `checkwright-theme` stamped as `data-theme` on the root element. The color
  tokens are CSS custom properties over the Primer palette, defined once in
  the layout. **This is the chrome contract the graph-theme unit consumes**:
  `scripts/graph-theme.sh` emits header/footer fragments and tokens matching
  this layout, including honoring the same localStorage key — so the queue
  entry for check-graph-theme-parity takes `[blocked-by: docs-site-chrome]`.
- **Logo** — `assets/logo/*` moves to `docs/assets/logo/` (the tree's single
  served home; nothing tracked references the old path). The header uses the
  mark; the mono mark is the dark-scheme variant if the sheet's tokens say
  so.
- **The SPEC mirror.** `scripts/gen-docs-mirror.sh` emits each kit's
  `SPEC.md`, `README.md`, and `doctrine-kit/DOCTRINE.md` into
  `docs/<kit>/` with generator-stamped front matter. Link rewriting rule: a
  relative link that resolves to another mirrored page stays relative (the
  mirror preserves the kits' cross-citation topology); any other relative
  link — a source file, a directory — is rewritten to the self-repo
  blob/tree grammar so nothing on the served site dead-ends in a 404.
  Mirrored pages are generated copies outside the governed manifest set (the
  existing `CANON_KIT_MANIFEST_FILES` globs already exclude them; the
  freshness gate, not the doc gates, is their honesty mechanism) and carry a
  generated-do-not-edit banner in an HTML comment.
- **`scripts/check-docs-mirror-fresh.sh`** (new consumer gate, registered in
  `gates.list`) — byte-compares the mirror against the generator's emission,
  the check-trajectory-fresh pattern. `# graph:` couples the mirror files to
  the kit SPEC/README/DOCTRINE sources; fixture pair supplies a synthetic
  source tree. Hook + graph + enforcement-map artifacts regenerate in the
  same unit.
- **Kit docs pages flip their reference blocks** from blob links to relative
  mirror links, anchors kept — `check-docs-link-convention`'s anchored
  back-link rule now applies to them and holds.

## Producers and consumers

- Mirror pages: produced by `gen-docs-mirror.sh`, run by whoever edits a kit
  doc (the freshness gate reds the commit otherwise — that gate is the
  producer's enabling trigger); consumed by the Pages build (rendering), the
  freshness gate's byte-compare, and site readers following the flipped
  reference links.
- Nav front matter: produced by page authors (living pages) and the
  generator (mirror pages); read by the layout's Liquid at build and by the
  extended parity gate at commit.
- `search.json`: produced by the Pages build from the Liquid template;
  read by `search.js` at query time. No committed artifact, no gate.
- `checkwright-theme` key: produced by the selector; read at page load by
  the layout script and by the graph page's injected chrome (the graph-theme
  unit's consumer worklist names this contract).
- The narrowed reference grammar: produced by this ruling in canon-kit's
  prose; consumed by docs-page authors choosing link form and by
  `check-md-refs`' existing passes (relative for mirror links, self-repo
  blob for source links — both already mechanized, no gate change).

## Existing sections updated

- canon-kit/SPEC.md §check-md-refs — the reference-link grammar paragraph
  narrows to source references (rendered documents cite the mirror
  relatively); the self-repo pass mechanism is unchanged.
- CLAUDE.md §Housekeeping, the docs bullet — gains the mirror's
  one-line regeneration pointer beside the existing generated-projection
  commands.
- docs kit pages' reference blocks — flipped as above.
- gate-sdk/SPEC-graph-theme.md (sibling amendment, pre-merge) — its consumer
  worklist derives tokens/fragments from this layout and the
  `checkwright-theme` key.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
