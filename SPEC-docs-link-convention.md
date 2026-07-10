# SPEC amendment: docs-link-convention

The link conventions the docs-kit-page-links sweep settled have now held
through an iteration, so they graduate from convention to gate (queue:
`docs-link-convention-gate`). `docs/` is repo-root-governed with no owning
kit, so this lands as a **consumer gate**, not kit mechanism — a second
consumer wanting it would motivate promotion into spec-kit beside
`check-md-refs`, which is out of scope here.

## What changes

New consumer gate `scripts/check-docs-link-convention.sh`, registered in
`scripts/gates.list`, `precommit` tier, `# graph:` coupling the `docs/`
page set. Two invariants over the docs pages (living pages and posts —
posts are temporal-exempt but stay link-governed, per
`scripts/spec-config.sh`):

1. **No directory-target relative link** — a relative link resolving to a
   directory renders differently on GitHub Pages than in the repo tree;
   name the file (`kit/index.md`, not `kit/`).
2. **Kit-page back-links carry a section anchor** — a link from a
   `docs/<kit>/index.md` page to that kit's `README.md`/`SPEC.md` names
   its `#section`: the docs tier owns orientation and *cites downward*,
   and an anchorless back-link is a "go read the whole spec" non-citation.

Division of labour with `check-md-refs` (which already resolves these
links): md-refs owns *resolution*, this gate owns *shape* — neither
subsumes the other. Per-site valve: `docs-link-exempt: <reason>` on the
line or the one above (HTML-comment form in Markdown), the standard
exemption grammar. Fail-closed on an unreadable page; `good/`+`bad/`
fixture pair under `scripts/gate-tests/check-docs-link-convention/`,
pointed at synthetic pages via an optional root argument (the
`check-docs-kit-parity` pattern).

## Producers and consumers

- Producer: the generated pre-commit hook / `run-gates.sh` (regenerate the
  hook and `.workflow/CHECK-GRAPH.html` when the gate registers).
- Consumer: the committing operator via the standard output contract —
  each hit `file:line` plus which convention it broke, read at the single
  scan transition.
- The docs page set is derived from the tree (`docs/**/*.md`), not a new
  knob — the gate is consumer-scoped, so this repo's layout is hardcoded
  legitimately.

## Existing sections updated

- CLAUDE.md Housekeeping (docs governance bullet) already states the
  citation doctrine; it gains nothing — the gate enforces, the doctrine
  stays where it is. `docs/index.md` needs no row (kit rows only).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — the gate's contract lives in its
      header directives (`# spec:` pointing at the convention's owning prose);
      no canonical kit spec changes.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the
      repo root for this unit (`ls SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks.
- [ ] **Fixture pair** — bad: a directory-target link and an anchorless kit
      back-link; good: file-target links, anchored back-links, an exempted
      site.
