# SPEC amendment: claude-md-housekeeping-residency

CLAUDE.md §Housekeeping is 72 of the file's 200 lines — 36% of the surface
every session pays for — and the only section carrying mechanism inline
rather than one line plus a pointer. Operator ruling on record (2026-07-17,
this iteration's scope): the mechanism moves to a **docs/-local architecture
doc** — option (a); widening site-kit was rejected, so the "docs/ is
repo-root-governed, no owning kit" ruling stands — and the same unit sweeps
the *rest* of §Housekeeping for load-trigger residency, not just the docs
bullet. Repo-root amendment: the governed surface is CLAUDE.md + docs/, no
owning kit (the shipped template's sanctioned home for exactly this case).

## What changes

**1. New file `docs/site-architecture.md`** — the load-triggered owner of
the docs-chrome mechanism that currently sits always-loaded. It receives,
integrated not concatenated:

- The site chrome contract: Jekyll layout, the nav Liquid contract and its
  front-matter keys (`nav_order`, `nav_parent`, `nav_id`,
  `nav_child_order`, `nav_children_key` and the release-notes derivation),
  client-side search, theme selector, and the chrome file roster
  (`docs/_config.yml`, `docs/_layouts/`, `docs/_includes/`,
  `docs/assets/`).
- Page-authoring rules: `title:` as terse nav label vs the opening H1's
  descriptive full form; living pages vs immutable dated `docs/posts/`;
  off-nav allowlisting (`scripts/docs-offnav.list`).
- The generated projections and their regen commands + freshness gates: the
  SPEC mirror (`gen-docs-mirror.sh` / `check-docs-mirror-fresh`), the value
  rollup (`gen-value-rollup.sh` / `check-value-rollup-fresh`), and the docs
  gate roster (`check-docs-kit-parity`, `check-docs-nav-reachable`,
  `check-docs-render-fidelity`), plus the install-toolchain parity contract
  (`check-install-toolchain` ↔ env-probe's `PROBE_SET`).
- Residency safety, stated in the doc: every regen-command move out of the
  always-loaded tier is covered oracle-first — each freshness gate's red
  output already names its regen command (`help: regenerate — …`), verified
  before this design; the gate teaches what residency used to.

Placement: off-nav by design — it joins `scripts/docs-offnav.list` (the
allowlist that exists for exactly this: governance prose for maintainers,
reachable by citation, not a reader-nav destination). It carries `title:`
front matter like every docs page and resolves under the doc gates
(`check-md-refs`, spec-manifest membership so its links and commands stay
live).

**2. CLAUDE.md §Housekeeping rewritten to the shape the file's own
§Conventions models** — one line per fact, mechanism behind a pointer:

- The `docs/` bullet collapses to ≤3 lines: what docs/ is, the
  no-owning-kit ruling, and the pointer to `docs/site-architecture.md` for
  chrome, authoring rules, regen commands, and the gate roster. The
  `docs/value.md` and `docs/install.md`-toolchain sub-bullets move whole
  into the new doc.
- The remaining bullets sweep for the same principle (operator ruling:
  whole-section sweep): CONTRIBUTING/RELEASING, `demo/`, and the
  local-files bullet each compress toward one line + pointer where their
  mechanism already has an owner (RELEASING.md, gate-sdk §Consumer smoke /
  evidence-kit suite registration, each kit's SPEC §Layout). The
  knowledge-friction bullet and the memory-off bullet stay resident — both
  are any-session facts with no load trigger (the widest-true-tier and
  load-trigger-residency tests both hold for them).
- The RELEASING.md sentence is co-owned this iteration: the
  release-in-iteration-lifecycle amendment changes the fact ("resident only
  at close's release step"); this unit owns the section's final shape.
  Build sequences the two edits; neither restates the other's content.

**3. No new gate.** What enforces §Housekeeping does not regrow mechanism:
the close skill's brevity pass is the standing cadence and `kpi-always-loaded`
trends the measured surface — this amendment names them as the watch and
accepts the honest limit (a judgment cadence, not a scanner). If the sweep
finds a gateable sub-class, it files as its own deferred entry per gap
disposition rather than growing this unit.

## Producers and consumers

- **`docs/site-architecture.md`** — producer: this unit authors it once
  from the content leaving CLAUDE.md (a move, not a copy: §Housekeeping's
  pointer replaces the prose in the same commit, so no two-sources window
  survives the unit). Consumers, each by a named mechanism: sessions
  touching `docs/` load it through the CLAUDE.md pointer (the load
  trigger); `check-md-refs` + the spec manifest hold its links and commands
  resolvable; `scripts/docs-offnav.list` membership is read by
  `check-docs-nav-reachable` (the off-nav sanction's mechanical reader).
- **The §Housekeeping pointer line** — producer: the same commit; consumer:
  every session (always-loaded), now paying one line where it paid the
  mechanism block.

## Existing sections updated

- CLAUDE.md §Housekeeping — the rewrite above (the section keeps its name
  and its always-loaded tier; only its content tier changes: facts stay,
  mechanism leaves).
- `scripts/docs-offnav.list` — one added path.
- The spec manifest needs no edit: `CANON_KIT_MANIFEST_FILES` already
  carries the `docs/*.md` glob (verified), so the new doc's links and
  commands resolve under the doc gates from its first commit.
- External references to §Housekeeping split by whether the cited *fact*
  stays or moves (align audit, 2026-07-17 — the scope grep scanned `.md`
  surfaces but missed the `spec:` pointers in gate/script source):
  - **Citations by section name whose fact stays** survive unedited — e.g.
    `.claude/commands/scope.md`'s evidence-reset binding cites "CLAUDE.md
    §Housekeeping" for the `.tmp/`/`.metric/` split; the section keeps its
    name and that fact.
  - **`spec:` pointers restating a fact that moves whole to
    `docs/site-architecture.md` must repoint** — `check-spec-pointer`
    validates only that the named §heading *exists*, not that it still owns
    the restated fact, so a stale owner stays green (silent drift). Two hard
    cases, each repointed to `docs/site-architecture.md §<heading>` in the
    same commit that moves the sub-bullet: `scripts/check-value-rollup-fresh.sh`
    (its em-dash tail restates the value-rollup projection) and
    `scripts/check-install-toolchain.sh` (restates the toolchain↔`PROBE_SET`
    parity). Build therefore authors `site-architecture.md` with named
    headings the two pointers can resolve to. The softer §Housekeeping
    citations (`demo/run-demo.sh`, `scripts/bash-guard.sh`) point at facts
    that stay in the compressed section or already carry a kit owner — build
    repoints or keeps each by where its fact lands.

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
