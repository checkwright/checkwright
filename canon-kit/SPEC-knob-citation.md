# SPEC amendment: knob-citation

## What changes

- New gate `canon-kit/checks/check-knob-citation.sh` — the enforcement
  half of the knob-defaults SSOT rule: on the governed manifest set
  (`CANON_KIT_MANIFEST_FILES`), a kit knob may not be stated *with its
  value* outside the kit's own SPEC.md. The knob-token vocabulary is
  derived, never listed: a SCREAMING_SNAKE token fires only when its
  prefix matches a form derived from a kit in `gate_kit_roots` — two
  forms per root: the dir name uppercased with hyphens to underscores
  (`delegation-kit` → `DELEGATION_KIT_`, and the sdk's own dir mapping
  the same way), and for a `-kit`-suffixed dir the suffix-dropped form
  too (`lifecycle-kit` → `LIFECYCLE_`: lifecycle-kit's SPEC-owned roster
  carries the short prefix, `LIFECYCLE_KIT_STAGES_FILE` its lone
  long-form knob, so the long form alone leaves that roster ungoverned).
  Both forms derive mechanically, so the gate still ships no term list
  and the provenance seam is untouched. The short form widens the token
  space (`QUEUE_`, `CANON_`, …); the value-marker leg of the triad below
  is what holds the false-positive rate, verified in calibration at
  build.
- Value-statement triad — all three required to fire, the low-FP
  calibration the filing named:
  1. a derived-prefix knob token;
  2. a same-line value marker — `=` appended directly to the token, or
     the word "default" in the clause together with a literal (a
     backticked value, a number, or a quoted string);
  3. the surface is not the owning kit's SPEC.md.
- Bare knob names stay legal everywhere — prose cites the name and points
  at the owning roster; that is the fixed instance's shape and the gate
  must never flag it. A bare number with no knob token in reach never
  fires: that stays a tripwire for human judgment, not a gate rule.
- Code and config files are out of scope — code owns values; the gate
  scans the prose manifest only.
- Skeleton copy-edit with a `good/`+`bad/` fixture pair; `# graph:`
  couples the manifest set and the kit SPECs; `tier=precommit`.

## Producers and consumers

- Producer: the gate, via the battery and the regenerated pre-commit hook
  (hook + graph artifact ride the landing commit).
- Consumer: the committing session — a red run names file:line, the knob
  token, and the owning SPEC the value belongs in, so the fix is a move,
  not a hunt.

## Existing sections updated

- canon-kit/SPEC.md gains §check-knob-citation beside the other
  prose-surface gates. The doctrine home stays doctrine-kit's
  de-literalization rule — the section cites it, never restates it.

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
