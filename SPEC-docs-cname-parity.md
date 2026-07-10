# SPEC amendment: docs-cname-parity

`docs/CNAME` becomes the single gated source of truth for the docs host
(queue: `docs-cname-parity`). Offline and decidable, so it fits the gate
contract its sibling `site-health-monitor` deliberately breaks. Stated
honestly: it would **not** have caught the 2026-07-10 cutover bug (tree
and CNAME agreed; DNS disagreed) — it prevents the *next* rename from
half-landing in the tree. Consumer gate: the host-alias set is this
repo's rule content, and `docs/` has no owning kit.

## What changes

New consumer gate `scripts/check-docs-cname-parity.sh`, registered in
`scripts/gates.list`, `precommit` tier, `# graph:` coupling `docs/CNAME`
to the tracked doc set. Invariant: with `H` = the host in `docs/CNAME`
(fail-closed if missing/empty/multi-line), no tracked file names a
**project host alias** other than `H` in a URL. The alias set is an
inline `# exception-list:`-style array in the gate — this repo's own
names, legitimately hardcoded in a consumer gate: `checkwright.dev`,
`www.checkwright.dev`, `checkwright.com`, `www.checkwright.com`,
`checkwright.github.io`. Any of these appearing where it isn't `H`
(e.g. a README link to the `.com` redirect, a lingering `github.io`
URL, a `www.` deep link) is red: the redirect host is reachable but is
never the *cited* docs URL, and after a future rename every stale alias
lights up at the first commit.

Scope: all tracked text files, with two path exemptions — `docs/posts/*`
(immutable published artifacts, the temporal-exempt precedent; their old
URLs stay honest history and the `.com` 301 keeps them alive) and the
gate's own fixtures (`*/gate-tests/*`). Bare-host mentions outside URLs
(prose naming the brand) are out of scope — the gate matches URL
occurrences, the shape that breaks. Optional file args point the fixture
pair at synthetic trees.

## Producers and consumers

- Producer: the generated pre-commit hook / `run-gates.sh` (hook +
  `.workflow/CHECK-GRAPH.html` regenerated at registration).
- Consumer: the committing operator via the standard output contract —
  each hit `file:line` plus the offending alias and the current `H`.
- `docs/CNAME` gains its named reader beyond GitHub Pages itself; a
  rename becomes a one-file edit whose fallout the gate enumerates.

## Existing sections updated

- None in kit specs. CLAUDE.md's docs bullet already names `docs/CNAME`
  as the Pages binding; the gate's `# spec:` header cites it.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — the gate's contract lives in its
      header directives; no canonical kit spec changes.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the
      repo root for this unit (`ls SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks.
- [ ] **Fixture pair** — bad: a `.com` URL and a `github.io` URL in a tracked
      doc against a `checkwright.dev` CNAME; good: apex-only URLs, a post
      under `docs/posts/`, the exempted fixture path.
