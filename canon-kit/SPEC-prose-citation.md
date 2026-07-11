# SPEC amendment: prose-citation

## What changes

- `check-spec-pointer` gains a **prose-citation pass** — closing the gap
  where a free-prose citation (`<path>.md §<Heading>` inline in a
  sentence) resolves neither file nor heading while the structured
  `spec:`/`contract:` directive and the markdown-link anchor are both
  verdict-checked. Scope of the new pass: the governed manifest set
  (`CANON_KIT_MANIFEST_FILES`). Rule: any prose occurrence of a tracked
  `.md` path immediately followed by `§<heading fragment>` must name a
  heading the target file carries, resolved by the existing
  `heading_present` helper (same trailing-qualifier tolerance the
  directive pass already has — one heading-resolution path, two callers).
- Low-FP carve-outs, applied in scan order:
  - fenced code blocks are skipped (a quoted example is not a citation);
  - a `§` with no tracked `.md` path directly before it on the line is
    skipped — the directive pass's deliberate em-dash prose-tail carve-out
    is unchanged, and non-citation `§` use never fires;
  - a cited path that is not a tracked file is out of this pass's scope —
    path liveness stays with the gates that own it (`check-md-refs`,
    `check-kit-ref-liveness`); this pass rules only on headings of
    resolvable files, which is what keeps the false-positive rate at the
    directive pass's level.
- The clean line's parenthetical extends with the prose-citation count;
  the fixture pair extends with a good and a bad prose-citation case.

## Producers and consumers

- Producer: `check-spec-pointer`, already registered and hook-coupled;
  its `# graph:` manifest widens to the manifest file set — the couples
  name shell and workflow-state surfaces only, no markdown (regenerated
  hook + graph artifact ride the landing commit).
- Consumer: the committing session — a red run names file:line, the cited
  path, and the missing heading, same shape as the directive pass.

## Existing sections updated

- canon-kit/SPEC.md §check-spec-pointer: the invariant sentence widens to
  both passes, and the carve-out paragraph gains the distinction between
  a directive's em-dash prose tail and a free-prose citation.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls canon-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
