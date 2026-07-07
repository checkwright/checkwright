# SPEC amendment: manifest-count

Sibling of SPEC-manifest-temporal.md, same tiering family: a pinned integer
that directly quantifies a *growing* governed collection ("six gates",
"seven meta-gates") in manifest prose is an un-gateable second source — the
count's owner is the collection itself (`gates.list`, a `checks/` dir, the
stages config), and a restated total drifts silently. The motivating find:
this repo landed one gate and left the same count in four disagreeing
copies (five/six/seven across two READMEs and a SPEC), caught only by
close-stage review. **Ban, don't validate**: a validating gate (the
platform's earlier approach) perpetuates the standing token cost
context-kit's brevity machinery rejects and needs FP-prone entity mapping;
a lexical tripwire eliminates the copy outright. Surfaced 2026-07-07.

## What changes

- **New gate `spec-kit/checks/check-manifest-count.sh`** (skeleton-derived,
  four contracts, `good/`+`bad/` fixtures). Invariant: no bare cardinal
  immediately quantifying a configured collection noun in governed manifest
  prose outside an exempt context.
  - **Surface** — the shared manifest set (`SPEC_KIT_MANIFEST_FILES` via
    the `lib/spec.sh` finder the sibling gate introduces); fenced blocks
    skipped; amendments excluded.
  - **Cardinal grammar** — digit sequences and the spelled words
    `two`…`twelve`, case-insensitive. `one` is deliberately outside the
    grammar: singleton and cardinality-rule idioms ("one owner per fact",
    "one iteration per kit") are invariants, not totals.
  - **Collection nouns** — `SPEC_KIT_COUNT_COLLECTIONS`, array; default =
    the collections the kits themselves govern and grow: `gates`,
    `meta-gates`, `checks`, `kits`, `stages`, `KPIs`. A consumer appends
    its own governed plurals; the noun list is the one place consumer
    vocabulary enters, and it enters as config.
  - **Exempt contexts**, mechanical first:
    - threshold/comparator context — the cardinal preceded (same line) by
      `≥ ≤ > <`, `at least`, `at most`, `up to`, `more than`, `fewer
      than`, or followed by a `per`-phrase; a bound is a rule, not a
      restated total;
    - partition idioms — `all but <cardinal>` (a classification claim,
      not a count);
    - `SPEC_KIT_COUNT_ALLOWED_PHRASES` — exact-phrase allowlist for
      *fixed* named sets a doc legitimately enumerates inline; default
      `("the four contracts")` — gate-sdk's own shipped vocabulary, the
      one fixed set any consumer's docs may cite. This repo's config adds
      its other fixed sets (e.g. "the three index tools") — fixed-set
      naming is consumer judgment, config not mechanism;
    - per-site `manifest-count-exempt: <reason>` for the residue.
  - Red on any unexempted `<cardinal> <collection-noun>` hit; fail-closed
    on an unreadable manifest file.
- **Calibration shares the sibling's FP corpus and procedure**: run the
  draft over this tree at build; disposition every hit — reword to cite
  the owning collection (preferred), extend config (a genuinely fixed
  set), or site-exempt with reason. The default noun list ships only as
  tuned against that corpus.
- **Tier: precommit**; `# graph:` manifest lists its couplings; registered
  in this repo's `scripts/gates.list`.

## Producers and consumers

- **Producer:** the generated pre-commit hook / `run-gates.sh` — reachable
  in every consumer whose `gates.list` names the gate; no enabling config
  beyond registration.
- **Consumer:** the committing operator/agent via the gate output contract.
- **Fields:** each hit is read at the gate's single scan transition (file,
  line, matched cardinal+noun in the violation message); the exempt
  patterns and per-site marker are read by the same scan to suppress. No
  new persistent state.
- **Interaction with the sibling gate:** shared manifest-set finder and
  per-site-marker matching in `lib/spec.sh`; independent marker grammars
  and knobs — either gate registers without the other.

## Existing sections updated

At merge into spec-kit/SPEC.md:
- new `### check-manifest-count` per-component contract section (invariant
  + calibration as above);
- §Content tiering's "Quantitative literals are code-owned" bullet gains
  the gate cite, and "Honest mechanizability" adds restated collection
  totals to the structural sub-rules that gate;
- §Layout and configuration gains the `SPEC_KIT_COUNT_COLLECTIONS` and
  `SPEC_KIT_COUNT_ALLOWED_PHRASES` knobs.

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
