# SPEC amendment: orchestration-headline

Docs-positioning unit over the public site (`docs/` — repo-root-governed, no
owning kit, hence this amendment's location). Prose only: no new page, no new
gate, no new knob. Three rulings.

## What changes

**1. Index hero carries the differentiator.** `docs/index.md`'s opening
section gains one paragraph naming verification-under-delegation as the
differentiator: Checkwright makes *done* mechanically decidable, so delegated
agent work can be trusted without reading all of it — linking
`orchestration.md` at hero altitude. The existing Positioning bullet stays
(the hero states the claim; the bullet remains the section's navigational
entry). The Start-here list is not renumbered — headline positioning is the
hero's job, not the reading order's.

**2. Multi-operator walkthrough as a section, not a page.**
`docs/orchestration.md` gains a scenario-form walkthrough section (working
title "Two operators, one governed tree"): two iterations proceeding
concurrently — each iteration owning one branch, the iteration-scoped state
surfaces resolving to the arriving branch at merge via the derived merge-driver
set, close-merges serializing at the integration branch — every invariant
cited to `lifecycle-kit/SPEC.md §Multi-operator semantics`, the section owning
narrative only. Ruled a *section*: a new page would add nav registration and
another governed surface for a story whose contract home already exists; the
walkthrough is reading matter for the page that already positions the layer.

**3. De-changelog §What is built.** The section's status vocabulary
("landed", built-vs-roadmap framing) is drift by construction — the tree is
the oracle, and a status stamp ages the moment it is written. Rewrite
present-tense as a description of what Checkwright's own coordination
*covers* (the iteration lead; multi-operator semantics), retitled accordingly
(working title "Checkwright's own coordination"); roadmap narration is
dropped — the public queue's deferred section is the roadmap surface, and the
page may point there once rather than narrate item status.

## Producers and consumers

Prose-only: no new state, event, or interface. The pages remain governed by
the existing docs gates (link/command resolution via `scripts/canon-config.sh`,
`check-docs-nav-reachable` — front matter unchanged, no nav change). Reader:
the site visitor; the contract owner cited downward stays
`lifecycle-kit/SPEC.md §Multi-operator semantics` and
`lifecycle-kit/SPEC.md §templates/lead.md`.

## Existing sections updated

- `docs/index.md` — hero section (ruling 1).
- `docs/orchestration.md` — new walkthrough section (ruling 2); §What is
  built rewritten and retitled (ruling 3); the page's closing "Where to go
  next" links stay valid.
- No SPEC changes; no docs-mirror regeneration is triggered (neither page is
  a mirrored kit doc).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition. (Prose-only: vacuously satisfied; the
      check is that every invariant stated is a citation, not a restatement.)
- [ ] **Merged with no information lost** — each addition integrated into its
      proper page section (not appended); each page reads as one coherent
      document.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the
      repo root (`ls SPEC-*.md`).
- [ ] **Removals propagated** — grepped the docs for the retired status
      vocabulary and the old section title; nothing dangles (including
      inbound anchors to the retitled section).
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
