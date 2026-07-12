# SPEC amendment: identifier-deliteralization

## What changes

**Rule ruling: extend De-literalization.** The rule's duty widens from values
to *source identifiers in prose*: prose names the public contract — the knob,
the command, the entry point another surface calls — never the source's
internal identifier roster or its step-by-step algorithm. A SPEC subsection
that inventories internal helpers or narrates branch conditions is the same
defect as a copied knob value: a second source of a fact the code owns,
stale at the next refactor. The WHY/invariant/public-contract stays SPEC
prose; the WHAT/how lives in the source behind a pointer.

**Enforcement ruling: stated authoring rule, no gate.** Every existing
anti-restatement gate targets a literal (values, counts, verbatim copies,
code dumps under banned headings); "prose names a source identifier" cannot
be decided cleanly because a SPEC legitimately names public functions *as
contracts* — the Enforcement-first high-false-positive carve-out applies.
Per that carve-out's cadence extension (the audit-cadence amendment), the
class joins the audit roster as its seed member
(`internal-identifier-restatement`), due on heavy-SPEC contract edits and at
release prep.

**Sweep worklist (the debt half of the unit).** Lean the verified leaks,
bounded to these sections — not tree-wide:

- `gate-sdk/SPEC.md` §lib/gate.sh — cut the function inventory down to
  internal helpers; keep the sourcing contract and the public entry points a
  gate author calls.
- `gate-sdk/SPEC.md` §check-graph — cut the letter-labeled assertion
  conditions mirroring the check's branches; keep the invariant and the
  manifest grammar (that grammar is an authoring contract).
- `canon-kit/SPEC.md` §lib/spec.sh — cut the internal awk identifiers and
  walk narration; keep the finder contract consumers configure.
- `canon-kit/SPEC.md` §check-comment-tier — the directive-token set is an
  *authoring vocabulary* (writers must use it), so the tokens stay; cut any
  narration of how the check walks.
- `canon-kit/SPEC.md` §check-manifest-count — same judgment: the cardinal
  grammar authors write against stays; the exemption regex internals go
  behind a pointer.
- `lifecycle-kit/SPEC.md` §bin/enter-stage.sh — cut the write-sequence steps;
  keep the atomicity invariant and the knob roster (knobs are contract).

The sweep judgment line: an identifier stays in prose iff a consumer outside
the file must type or configure it; it goes behind a pointer iff only the
source's own maintainer meets it.

## Producers and consumers

- **Producer:** SPEC authors, at writing time; the extended rule text is the
  trigger (loaded via the doctrine link; the De-literalization digest bullet
  in CLAUDE.md already points there — its one-line wording is unchanged).
- **Consumer:** the audit roster's close-stage review (the recurring check
  that the class has not regrown); the swept SPEC sections' readers, who now
  meet a pointer to the source instead of a stale inventory.
- No new machine surface: no state, file, field, or knob.

## Existing sections updated

- `doctrine-kit/DOCTRINE.md` — De-literalization gains the identifier clause;
  its *Enforced by* line states the manual-duty + audit-roster channel beside
  the existing bare-cardinal gate citation.
- The six SPEC sections above — leaned in place.
- `.workflow/audit-roster.txt` — seed member lands via the audit-cadence
  unit; this unit's build verifies the entry exists.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section; the merged doc reads alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component.
- [ ] **Removals propagated** — grepped the tree for §-citations into the
      leaned sections; every citation still resolves to a live heading.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed
      as debt tasks.
