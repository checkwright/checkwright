# SPEC amendment: declined-target

A build session can decline an update target an amendment rostered. It does this
by reporting in its commit message that it found nothing to update. The build
template's test for "nothing to update" is whether a passage exists to edit. That
test can be literally true while the target's purpose goes unmet. A later
confirmation also reads the cause on the declining session's terms, so it cannot
catch the mismatch. This amendment changes what a declination has to state: the
obligation the citing delta places on the target, and the passage that already
meets it.

**The measured case.** One iteration's two build batches declined three targets.
Each declination was probed and confirmed, and one was a real hole in a roster.
`queue-kit/README.md` lists the arms that read the queue file, and it had no
entry-history line. The stated cause, "no entry-history line to widen", was true.
The right act was to add the line, and that iteration's close did.

**The ruling: neither reading the queue entry offered.** The entry offered two
readings. One was that the authoring roster over-reaches, fixed by narrowing at
authoring. The other was that the target surfaces have holes, fixed by a coverage
assertion on each surface. The datum fits neither:

- **Not over-reach.** The roster was right. The target needed a write, and the
  roster named it.
- **Not a surface-coverage gap to gate.** A per-surface coverage assertion is a
  new oracle for every roster-shaped surface. That is the order-larger
  deliverable, and one datum cannot justify it.

What failed is the declination test. It asks whether there is a passage to edit,
where it should ask whether the surface already says what the delta needs it to
say. An absent passage answers the first question with "nothing to update" and
the second with "a missed site". This is a template-level change, so no new name
is minted. It fits the entry's `iteration/low` cost.

**Probed at authoring, 2026-09-18.** `lifecycle-kit/templates/stages/build.md:91-93`
reads "A rostered target the re-derivation finds nothing to update in is reported
in the commit message as a finding, naming the target; never write an edit to
make it non-vacuous, and never drop it silently." That test is by edit-site, and
it states no obligation. `grep -n -i "declin" lifecycle-kit/templates/stages/validate.md
lifecycle-kit/templates/stages/close.md` returns only close's recurrence-decline
lines, so no stage template re-reads a declined target. The tree does not already
do what this amendment asks.

## What changes

### (1) The build template's declination test becomes the delta's obligation {design-bearing}

**Not yet applied.** This text replaces the last sentence of the paragraph
"**An amendment's roster is a floor, never the reach.**" in
`lifecycle-kit/templates/stages/build.md` (the sentence quoted above):

> Decline a rostered target only when the surface already says what its citing
> delta needs from it. Name the target in the commit message, with that need and
> the passage that already meets it. A missing passage never declines a target.
> A surface that lacks what the delta needs is a missed site, and you land it in
> this unit. Never write an edit a met need does not call for, and never drop a
> target silently.

The instruction carries no grounds. Delta 2 places them. The unit appends one
bullet to the release declaration surface, `.workflow/release-declarations.md`,
because a kit template's behaviour changes (the same template's *Declare what a
vendoring consumer will meet*).

### (2) The template's ground paragraph states why the test is by obligation {design-bearing}

**Not yet applied.** This text replaces the closing sentences of the
lifecycle-kit/SPEC.md §templates/stages/ paragraph that opens "The `build`
template treats an amendment's roster as a floor it re-derives", from "**The same
re-derivation meets the over-count:**" to the paragraph's end:

> **The same re-derivation meets the over-count, and it is judged by the
> delta's need, never by an edit site.** A rostered target with nothing to
> update tempts a fabricated edit or a silent drop, so the empty result is a
> stated finding. The finding's terms are the citing delta's need and the
> passage that meets it, because the other terms can be true while the target's
> purpose goes unmet. A roster of queue-reading arms that lacked one arm's line
> was declined on "no line to widen", confirmed on those terms, and later
> repaired by adding the line. Any later reader who confirms a declination now
> confirms it against the author's need. **The honest limit:** no gate reads a
> commit message's declination, on the same ground as the re-derivation itself,
> so a declination stated on edit-site terms is caught only by the reader who
> re-reads it. Two heavier fixes were weighed and refused on one measured
> instance: narrowing rosters at authoring, when the roster was right, and a
> coverage assertion on every roster-shaped surface, a new oracle per surface.

### (3) canon-kit's non-vacuity paragraph cites the new test {mechanical}

**Not yet applied.** In canon-kit/SPEC.md §check-amendment-update-target, the
paragraph "**Deliberately not asserted either: a rostered target's
non-vacuity.**" closes with "What remains is a claim the merging session checks:
the build stage re-derives the roster and reports an empty target as a finding
in its commit (lifecycle-kit/SPEC.md §templates/stages/), which is what separates
a vacuous bullet from a skipped one." That sentence becomes:

> What remains is a claim the merging session checks: the build stage re-derives
> the roster and declines a target only by naming the passage that already meets
> its delta's need (lifecycle-kit/SPEC.md §templates/stages/). That separates a
> met target from a skipped one.

### (4) The generated mirrors {mechanical}

`docs/lifecycle-kit/SPEC.md` and `docs/canon-kit/SPEC.md` are regenerated after
deltas 2 and 3 land. Each mirror's freshness gate prints its own regeneration
command.

## Producers and consumers

- **The declination statement** (delta 1). Producer: the build session that
  merges an amendment, at the merge commit, whenever its re-derivation finds a
  rostered target already met. This is reachable in every consumer that runs the
  build template, with no config. Consumer: any later reader of the commit
  message. Today those are the validate and close sessions and the lead, reading
  by hand. No field goes unread. The target names what was declined, the need
  is what a reader checks, and the passage is where they check it. No
  roster-holding reader exists, because no gate parses commit messages for
  declinations. The honest limit in delta 2 says so.
- **No new state, event, knob or name.** The unit changes the terms of an
  existing finding, so no roster gains a row.

## Existing sections updated

Roster probe: `git grep -n -i "nothing to update\|non-vacuous\|empty target\|over-count" -- '*.md'`
over the tracked tree, with `docs/` mirrors and `TASK-QUEUE.md` excluded, then
reading each hit's paragraph.

- `lifecycle-kit/templates/stages/build.md`, the roster-floor paragraph (delta 1).
- `.workflow/release-declarations.md`, the template's behaviour change (delta 1).
- `lifecycle-kit/SPEC.md` §templates/stages/, the build roster-floor ground
  paragraph (delta 2).
- `canon-kit/SPEC.md` §check-amendment-update-target, the non-vacuity paragraph
  (delta 3).
- `docs/lifecycle-kit/SPEC.md` and `docs/canon-kit/SPEC.md` (delta 4).

## Retired spellings

- None — no delta retires a spelling a reader cites. The replaced sentences are
  prose, and no other surface quotes them.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The
      causal-completeness check holds for each new state, event, interface and
      obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a
      template, agent definition or shim carries no grounds; a delta places them
      (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the
      canonical-spec text it refines rather than appending to it; the merged spec
      reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls lifecycle-kit/SPEC-*.md`) once `SPEC-resident-pricing.md`
      has merged too.
- [ ] **Removals propagated** — `## Retired spellings` above is accurate, and
      `check-amendment-retired-spelling` is green.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
