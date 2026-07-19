# SPEC amendment: release-note-section-taxonomy

## What changes

The release note's three fixed sections (Tightened gates / Renamed knobs /
Behavior changes) are the consumer's reconciliation checklist for residue the
wholesale-sync + battery cannot mechanically surface. docs/install.md §The
upgrade contract names **four** residue classes (shadowed gates, copied-out
templates, own-config knob renames, depended-on behavior) but the note carries
**three** sections, so the section set is imperfectly aligned to its own stated
model at two edges. This amendment reconciles those two edges *without* adding a
section or touching `check-release-bump`.

**Confirmed principled (no change).** "Tightened gates" (not "gate changes") is
correct: only a new/stricter gate can red a clean tree, so it alone is the
mechanical allowed-red set; a relaxed/removed gate reds nobody and is off the
worklist by design. That narrowness stays.

**Edge 1 — knob removal / orphaned config.** `old → ∅` orphans consumer config
exactly as a rename does, is the **same residue class** as a rename ("knob
renames in your own config"), yet is not itself a rename and may red no gate.
**Ruling: express a removal as `old → ∅` under the existing "Renamed knobs"
section** — same residue class, same section, no new section name. This needs
**no `check-release-bump` change**: the gate counts bullets per fixed section
(it never parses the `→` arrow), so an `old → ∅` bullet counts as a
renamed-knob bullet and correctly floors the release to minor — a config
orphaning is phase-B work. The slight "Renamed knobs contains a removal"
misnomer is the accepted cost of keeping the section set (and the gate's fixed
section names) stable; retitling the section would break the gate's
`section_bullets "…" "Renamed knobs"` match and is deliberately declined.

**Edge 2 — four classes, three sections.** "Templates you have copied out" is a
named residue class with no dedicated section, folded implicitly into Behavior
changes. **Ruling: keep it behavior-folded, and make the mapping explicit** — a
copied-out template that changed *is* depended-on behavior diverging from the
consumer's copy, so Behavior changes is its correct home. The defect is only
that the folding is implicit; docs/install.md §The upgrade contract states the
four-classes → three-sections mapping so the note grammar is aligned to its own
model on the page, not by author inference. No new section, no gate change.

**Envelope.** Documentation-and-convention only: the reconciliation of the
section taxonomy to its four-class model. No new section, no
`check-release-bump` change, no change to the "None is a valid, stated body"
rule. Complementary to and non-overlapping with the sibling
`release-note-chrome-ownership` unit (that owns the *fixed chrome*; this owns
the *variable-section taxonomy*).

## Producers and consumers

- **Producer** — the release-note author (the closing session running
  RELEASING.md step 1), who now has an explicit home for a knob removal
  (`old → ∅` under Renamed knobs) and an explicit statement that copied-out
  template residue goes under Behavior changes.
- **Consumer** — the upgrading consumer reconciling residue against the note's
  sections (the checklist reader), and `check-release-bump` as the mechanical
  floor. The gate's behavior is **unchanged**: it already counts Renamed-knobs
  bullets, and an `old → ∅` bullet is one such bullet, so a removal floors to
  minor with no code edit. No field, section, or knob is added, so there is no
  new reader to name beyond these.

## Existing sections updated

- **docs/install.md §The upgrade contract** — under "Renamed knobs", state that
  a knob *removal* is expressed `old → ∅` (same residue class as a rename);
  add the four-residue-classes → three-sections mapping (shadowed gates →
  Tightened gates; own-config knob renames *and removals* → Renamed knobs;
  copied-out templates *and* depended-on behavior → Behavior changes) so the
  section set is visibly aligned to its stated model.
- **RELEASING.md step 1** — its pointer to docs/install.md §The upgrade
  contract for the section grammar is unchanged; no edit required unless a
  one-line "a knob removal is `old → ∅`" reminder reads more clearly inline
  (additive, optional).
- **scripts/check-release-bump.sh** — **no change**; the ruling is chosen
  specifically so the bullet-count floor already covers the `old → ∅` form.

No wire contract, no fenced source to embed.

## Definition of Done

- [ ] **Causal completeness** — the two ruled edges name their producer (the
      note author) and consumers (the reconciling upgrader + the unchanged
      `check-release-bump`); no new field or section, so no unread reader.
- [ ] **Merged with no information lost** — the `old → ∅` convention and the
      four-to-three mapping land in docs/install.md §The upgrade contract; the
      page reads as one coherent grammar a note author can follow alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the
      repo root for this unit (`ls SPEC-*.md`).
- [ ] **Removals propagated** — confirm no surface claims a "Removed knobs"
      section exists (the ruling deliberately adds none); the gate's fixed
      section names are untouched.
- [ ] **Gaps filed** — any cross-component gap found during the work filed as a
      debt task.
