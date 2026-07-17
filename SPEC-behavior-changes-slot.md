# SPEC amendment: the release note's Behavior changes slot

Root-level amendment (the release-note grammar is repo-root-governed —
`docs/install.md` owns it, `scripts/check-release-bump.sh` gates it; no kit
surface changes). Closes the grammar gap where a shipped behavior change that
is not a battery gate has no fixed slot in the note: `v0.2.0` produced two
such changes and `v0.4.0` recurred with two more, all landing in Upgrading
prose that no mechanical reader visits and no gate holds. Surfaced 2026-07-17
authoring the `v0.2.0` note; recurrence re-verified against the `v0.4.0` note
at promotion.

## What changes

### 1. A third fixed section: Behavior changes

`docs/install.md` §The upgrade contract's note grammar gains one section
beside the existing two:

- **Behavior changes** — one bullet per shipped change that alters what the
  kits *do* without landing or tightening a battery gate: a fail-closed
  convergence in a shared library, a runner's semantics, a skill or template
  behavior, a default's effect. The bullet's lead token is the changed
  surface's name (the script, knob, template, or file), bolded like the other
  sections' lead tokens; the rest of the bullet states what moved and what, if
  anything, the consumer reconciles.
- The empty state is stated, never omitted — `None — no behavior changed
  outside the gate battery.` — the same stated-silence convention the two
  existing sections carry, so an absent declaration is a grammar violation
  rather than an ambiguous omission.

### 2. The mechanical reader: check-release-bump

The named-reader requirement is satisfied by the gate that already reads the
note mechanically, at two transitions:

- **Presence** — `scripts/check-release-bump.sh` extends its fixed-section
  assertion to the third section (absent section: exit 2, the existing
  contract for the other two).
- **Bump floor** — the floor derivation includes it: a non-empty Behavior
  changes section sets the **minor** floor. Rationale: patch means
  blind-upgrade-safe (nothing to reconcile), and a behavior change is
  precisely what breaks that promise — the same reasoning that puts the other
  two sections on the floor. `docs/install.md` §Versioning's patch/minor
  definitions re-word from "both sections"/"either section" to range over the
  three.

### 3. Ruled out (recorded so build does not relitigate)

- **Widening the allowed-red set past the battery** — `upgrade-smoke`'s
  containment assertion is defined over battery reds; a non-gate change
  cannot red the battery, so a wider set is unverifiable by construction. The
  smoke does not read the new section. §The upgrade contract states this
  honest limit explicitly: Behavior-changes bullets are *declared for the
  human upgrader*, not smoke-asserted — the section fixes where they live and
  the gate fixes that they are stated; it does not make them executable.
- **Prose-only (blessing the status quo)** — the recurrence record (two
  changes in `v0.2.0`, two in `v0.4.0`) shows the class is routine, and an
  unenforced prose slot leaves the next release's non-gate change to author
  judgment — the filed defect, not a fix.

### 4. Migration: backfill, not grandfathering

The shipped notes are living doc pages (git history is the record), so the
existing notes are backfilled rather than grammar-exempted: `v0.2.0` and
`v0.4.0` move their already-written non-gate declarations from Upgrading prose
into the new fixed section (relocation, no new claims); `v0.1.0` and `v0.3.0`
gain the stated `None` line if nothing qualifies. This keeps
`check-release-bump`'s presence assertion unconditional — no
newest-note-only carve-out, no grammar epoch to track. Retro floor check:
every backfilled non-empty note already rode a minor, so the extended floor
reds nothing.

## Producers and consumers

- **The section body.** Producer: the release author at RELEASING.md step 1
  (the in-iteration note authoring step — its checklist names the three
  sections after this change, so the producer's enabling instruction ships
  with the grammar). Consumers: the human upgrader (the note's primary
  audience, now with a fixed location instead of scanning prose);
  `check-release-bump` at the two transitions above.
- **The `None` line.** Reader: `check-release-bump`'s section-bullet count
  (zero bullets = floor-neutral), and the human upgrader reading silence as a
  decision. No other new field is introduced; `upgrade-smoke` and the upgrade
  skill are deliberate non-readers, stated as such.

## Existing sections updated (at merge)

- `docs/install.md` §The upgrade contract — the section roster (three fixed
  names), the stated-None convention sentence, the honest-limit sentence on
  smoke coverage; "two sections under fixed names" wording.
- `docs/install.md` §Versioning — patch/minor floor definitions ("its two
  fixed sections" → the three; the gate-summary paragraph naming what
  `check-release-bump` reds).
- `RELEASING.md` step 1 (authoring: the three sections) and step 2 (the bump
  is "read off the note's" sections — count updated).
- `scripts/check-release-bump.sh` — `# spec:` header lines cite the updated
  grammar; the two-section literals gain the third.
- `docs/posts/` — the four existing notes backfilled per §4.
- No kit surface changes: `gate-sdk/bin/upgrade-smoke.sh` and
  `gate-sdk/SPEC.md` §upgrade-smoke restate no section roster and stay
  untouched.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged doc reads as one
      coherent grammar a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the
      root (`ls SPEC-*.md`).
- [ ] **Removals propagated** — grepped docs and scripts for the two-section
      cardinals and wording this change retires; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
