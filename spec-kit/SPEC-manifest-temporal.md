# SPEC amendment: manifest-temporal

A tiering-family gate mechanizing the temporal share of the narration
judgment: a manifest states current behavior only — history is derivable
from git — and a "formerly…" line is standing context cost documenting the
*old* cost, taxing every session that reads it. Today that judgment is
manual (context-kit's close-stage brevity pass calls it "the semantic
residue check-brevity cannot decide"); its lexical share is cheap to gate.
Surfaced 2026-07-07.

## What changes

- **New gate `spec-kit/checks/check-manifest-temporal.sh`**
  (skeleton-derived, four contracts, `good/`+`bad/` fixtures). Invariant:
  no temporal-narration marker in governed manifest prose outside an
  exempt site.
  - **Manifest set** — a new shared surface notion, `SPEC_KIT_MANIFEST_FILES`
    (array of globs): default = the canonical specs (`SPEC_KIT_SPEC_NAME`,
    honoring `SPEC_KIT_SCAN_KIT_ROOTS`), `README.md` at any depth, and
    `CLAUDE.md`. Amendments (`SPEC_KIT_AMENDMENT_GLOB`) are excluded by
    construction — a transition artifact describes change; that is its
    nature, not narration. Enumeration lands as a `lib/spec.sh` shared
    finder so the sibling count gate reads the identical set (one adapter,
    no drift axis).
  - **Marker set** — `SPEC_KIT_TEMPORAL_MARKERS`, default (generic English,
    mechanism not rule content): `previously`, `formerly`, `renamed from`,
    `no longer`, `used to be`, `was retired|removed|renamed|replaced`.
    Bare `used to` is deliberately excluded (collides with instrumental
    "used to build/filter" — the platform's calibration note carries over).
    Case-insensitive; fenced code blocks are skipped (a gate-output example
    may quote a marker).
  - **Exemptions**, two valves:
    - per-site `manifest-temporal-exempt: <reason>` (the
      `spec-embedded-source-exempt:` pattern) for a line that is
      legitimately about the past;
    - `SPEC_KIT_TEMPORAL_EXEMPT_SECTIONS` — array of heading names whose
      whole section is exempt; default empty. This repo's config sets
      `What stayed on the platform` — deliberate extraction provenance is
      *this consumer's* convention, not kit mechanism (and at the
      `SPEC_KIT_SCAN_KIT_ROOTS=0` default a vendoring consumer never scans
      the kit SPECs that carry those sections).
  - Red on any unexempted marker hit; fail-closed on an unreadable
    manifest file.
- **Calibration is the crux, and the FP corpus is this repo**: at build,
  run the draft gate over this tree and disposition every hit — reword
  (preferred: narration is standing cost), section-exempt via config
  (provenance), or site-exempt with reason. The marker set ships only as
  tuned against that corpus.
- **Tier: precommit**; `# graph:` manifest lists its couplings; registered
  in this repo's `scripts/gates.list`.

## Producers and consumers

- **Producer:** the generated pre-commit hook / `run-gates.sh` — reachable
  in every consumer whose `gates.list` names the gate; no enabling config
  beyond registration (the default manifest set is non-empty wherever a
  README exists).
- **Consumer:** the committing operator/agent via the gate output contract.
- **Fields:** each marker hit is read at the gate's single scan transition
  (file, line, matched marker in the violation message); the exempt-section
  and per-site markers are read by the same scan to suppress. No new
  persistent state.
- **Interaction with context-kit's close-stage brevity pass:** the pass
  keeps the semantic narration judgment (is this sentence *about* the
  past?) on the always-loaded surface; this gate takes the lexical share on
  the manifest set. context-kit/SPEC.md §The close-stage brevity pass and
  `templates/close-brevity.md` step 3 gain a one-line cite naming the gate
  as the grown mechanical floor.
- **Interaction with drift-kit:** the platform's by-eye narration-marker
  KPI is superseded by this gate and stays unextracted (recorded in
  drift-kit/SPEC.md §What stayed on the platform); no drift-kit code reads
  this gate.

## Existing sections updated

At merge into spec-kit/SPEC.md:
- new `### check-manifest-temporal` per-component contract section
  (invariant + calibration as above);
- §Content tiering's "Honest mechanizability" bullet adds temporal
  narration to the list of structural sub-rules that gate;
- §Layout and configuration gains the `SPEC_KIT_MANIFEST_FILES`,
  `SPEC_KIT_TEMPORAL_MARKERS`, and `SPEC_KIT_TEMPORAL_EXEMPT_SECTIONS`
  knobs.

In context-kit (cross-kit, cite-only): the two one-line cites named above —
no mechanism moves.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls spec-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
