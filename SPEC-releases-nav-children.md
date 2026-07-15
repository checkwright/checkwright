# SPEC amendment: releases-nav-children

Repo-root docs-chrome ruling (docs/ is repo-root-governed, no owning kit):
surface each release note as a second-level nav child of the Releases page,
derivation-first. Merge targets: CLAUDE.md §Housekeeping (the nav grammar
sentence), `docs/_includes/nav.html`, `docs/releases.md`,
`scripts/check-docs-nav-reachable.sh`, `scripts/docs-offnav.list`,
`RELEASING.md`.

## What changes

- **New front-matter key: `nav_children_key`.** A top-level nav page may name
  a front-matter key (the Releases page sets `nav_children_key: release`).
  The nav include gains a generic derived-children branch beside the existing
  `nav_id`/`nav_parent` branch: the page's nav children are the site pages
  whose front matter carries the named key, sorted by path descending
  (newest first — the same query and order `docs/releases.md`'s body already
  runs), each labeled by its key *value* (e.g. `v0.1.0`). Never per-child
  `nav_parent` stamps: dated posts are immutable, and the child set derives
  from front matter the notes already carry for the upgrade tooling.
- **Gate model extension.** `check-docs-nav-reachable` models the rendered
  nav in shell; it learns the derived-children rule: a page carrying a key
  that some nav-slotted page names in `nav_children_key` *holds a nav slot* —
  it is reachable, and the key value is its rendered label, so the `title:`
  floor is satisfied without a `title:` block (an immutable post cannot gain
  one). The gate's fixture pair grows a firing/non-firing case for the rule.
- **Allowlist shrinks.** The v0.1.0 note leaves `scripts/docs-offnav.list`
  (it now holds a nav slot); the header comment's release-note rationale is
  rewritten — release notes are on-nav by derivation now. The front-matter-less
  announcement post (no `release:` key) stays allowlisted.
- **Release runbook step removed.** RELEASING.md's note-authoring step (§The
  procedure step 2) loses its "Add the post's path to `scripts/docs-offnav.list`"
  sentence: a note joins the nav by its `release:` key with no further step —
  the allowlist stops growing per release (it stays for the pages off-nav by
  design, e.g. the front-matter-less announcement post).
- **Verification is the local render.** Liquid's dynamic-key lookup
  (`p[key]` under `where_exp`, or a fallback for-loop filter if `where_exp`
  refuses a variable key) is confirmed against the local Jekyll render
  (ENV.local.md's GEM_HOME invocation) before commit — the gate models the
  nav, it does not render it, so only the render proves the include.

## Producers and consumers

- **`nav_children_key` on a nav page** — producer: the page's tracked front
  matter (`docs/releases.md`, emitted in-tree — no runtime config).
  Consumers: `docs/_includes/nav.html`'s derived-children branch at render
  time, and `check-docs-nav-reachable`'s model at pre-commit. Two readers,
  both named; the gate's model and the include must agree, which the gate's
  fixture pair pins.
- **The named key's value on each child page** (`release: vX.Y.Z`) — readers:
  the nav branch (as the child's label and link), the gate (as the
  title-floor satisfier), plus the pre-existing readers (the releases-page
  body loop, the upgrade tooling) which are untouched.
- No other new state, field, or interface.

## Existing sections updated

- CLAUDE.md §Housekeeping's nav-grammar parenthesis (Liquid over
  `nav_order`/`nav_parent` front matter …) gains the derived-children clause
  — the gate's `# spec:` lines cite that sentence, so model and prose move
  together.
- `scripts/docs-offnav.list` header comment (above).
- RELEASING.md §The procedure step 2, the note-authoring step (above).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper home; CLAUDE.md's nav sentence reads whole without this file.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the
      repo root (`ls SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired (the runbook's allowlist step, the allowlist line); nothing
      dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed
      as debt tasks (a build-time causal gap is resolved that session, not
      deferred).
