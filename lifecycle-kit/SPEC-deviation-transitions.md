# SPEC amendment: lifecycle-deviation-transitions

lifecycle-kit/SPEC.md gains a deviation-transitions section: the gate-legal
shapes for leaving the linear stage walk. This has teeth because
`check-stage-entry` and the flip+stamp protocol block an ad-hoc abandon
today; each escape hatch below is specified against the existing gates, and
the ruling is that **no new tooling lands** — every shape composes the
mechanism the SPEC already owns (`bin/enter-stage.sh`, the queue tags, the
amendment lifecycle), so a harness-less consumer keeps every hatch.

## What changes

`lifecycle-kit/SPEC.md` gains `### Deviation transitions` under §The state
machine, owning:

- **The demote ritual** (the shared step the other shapes compose): to take
  a promoted entry out of a live iteration, move it back to the deferred
  section restoring its design-pending tag, and delete its amendment file in
  the same commit. Git history preserves the design (a later scope
  re-promotes by resurrecting the file from history rather than re-deriving
  it); canon-kit's `check-amendment-queue` is the enforcement already on the
  books — a deferred entry still carrying a spec ref, or an orphaned
  amendment on disk, reds the commit. If the validate baseline carries a
  scenario keyed to the demoted entry, that scenario is re-scoped or removed
  in the same commit (`check-evidence-baseline`'s slug-liveness holds
  regardless, since a demoted entry stays live — this obligation is about
  coverage honesty, not the gate).
- **Abandon** — ending an iteration without close. The shape: disposition
  every active entry explicitly — demote it (ritual above) or carry it (it
  stays active with its amendment and the next iteration adopts it); sink or
  delete every Lessons entry under the existing disposition rules (the
  first-stage entry refuses a non-empty Lessons section, so this is already
  forced); then the next `enter-stage.sh scope` *is* the abandon — scope has
  no mandatory predecessor, so the flip is gate-legal from any stage, and
  the boundary reset drops the dead iteration's stamps exactly as it drops a
  closed one's (git history is the permanent audit trail — the existing
  §The state machine doctrine, not a new rule). The abandon commit's subject
  names the abandoned iteration; no stamp grammar changes.
- **Split mid-flight** — the iteration name never changes once set (every
  stamp already written carries it). Splitting is narrowing: demote the
  split-out subset via the ritual and drive the remaining queue through the
  remaining stages; the subset re-promotes at a later scope under its own
  iteration. A rename-in-place is barred because it would orphan the
  earlier stamps against `check-stage-evidence`'s header/stamp agreement.
- **Reopen after close** — barred as an in-place edit: stamps are
  append-only within an iteration and scope is the only reset, so there is
  no gate-legal way to continue a closed iteration's evidence file — and no
  history rewrite is sanctioned to fake one (doctrine-kit rule 16 territory).
  The sanctioned shape is a successor iteration: a post-close defect files
  as a debt entry and the follow-up iteration proceeds normally; the closed
  iteration's record stays immutable.

## Producers and consumers

- The section's producer is this unit's build session (a SPEC merge); its
  consumers are operator sessions executing a deviation and the scope skill
  (whose triage step gains nothing new — the demote ritual writes only
  surfaces whose gates already exist).
- No new state, no new stamp grammar, no new tag, no new tool flag: every
  step above is producer- and consumer-complete inside mechanisms the SPEC
  already specifies (`enter-stage.sh`'s boundary reset, canon-kit's
  amendment pairing, queue-kit's tag algebra).

## Existing sections updated

- `lifecycle-kit/SPEC.md` §The state machine — gains the pointer to the new
  subsection (the linear walk is the default, deviations are specified, not
  improvised).
- `lifecycle-kit/README.md` — one line naming the section, per the kit's
  README/SPEC tiering.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
