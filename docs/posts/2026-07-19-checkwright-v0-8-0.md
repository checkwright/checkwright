---
release: v0.8.0
---

# Checkwright v0.8.0

*2026-07-19*

Checkwright is the verification layer under agent orchestration, and this
release splits amendment authoring out of the scoping stage into its own
optional stage. Scoping a unit set and authoring the amendment that specifies
one are different work — one bounds, the other generates — and carrying both in
a single session means the authoring context rides along through every scoping
decision. `spec` is the stage that separates them. It is **opt-in**: the kit
default roster is unchanged, so an upgrade that ignores this release entirely
keeps behaving exactly as it did.

## Tightened gates

None.

## Renamed knobs

None.

## Behavior changes

- **lifecycle-kit/templates/skills/spec.md** — a new optional
  **amendment-authoring** stage template: the generative half of design, split
  out from `scope` (the ontology being that scope bounds the units, the
  authoring stage authors the amendments, the audit stage independently verifies
  them). It is a full stage — it invokes `enter-stage.sh` and stamps — and is
  **trigger-gated exactly like the audit stage**: it runs only when an iteration
  promotes a feature to author, it *appends* rather than resets the evidence file,
  and it takes `scope` as its predecessor without being named any stage's
  mandatory predecessor. The **kit default roster does not bind it**; a consumer
  activates it through `LIFECYCLE_KIT_STAGES` and `LIFECYCLE_KIT_PREDECESSOR`. If
  you do not opt in, nothing in your tree moves.
- **lifecycle-kit/templates/skills/scope.md** — scope's conditional authoring
  step no longer carries the authoring how-to inline; it points at `spec.md`,
  which now single-sources that content (causal completeness and canon-kit's
  bidirectional queue pairing). A default-roster `scope` still authors amendments
  exactly as before and simply reads the how-to there, so this is a
  single-sourcing move rather than a change in what scope does. If you have
  copied `scope.md` out, your copy still holds — but it now duplicates prose the
  kit owns in one place.
- **lifecycle-kit/templates/lead.md** — gains a **tier-each-stage-to-its-work-class**
  rule for dispatchers running stages as separate sessions: a stage whose work is
  mechanical oracle-running (running a fixed battery and reporting its verdict)
  is tier-downgradeable to a cheaper model via a `model` override on that stage's
  dispatch, while a stage whose work is generative or verificational judgment —
  bounding a unit set, authoring an amendment, cross-spec audit, implementation —
  is not, because the judgment is exactly what the tier buys. Which stages fall on
  each side stays the consumer's binding; the rule supplies only the work-class
  test, and it asks you to re-judge every assignment when the harness model roster
  churns.
- **canon-kit and the stage-referring templates** — prose and one gate *message*
  are stage-neutralized: `check-amendment-queue`'s error text now reads
  "spec-writing is an authoring-stage activity" rather than naming the scope stage,
  and the same neutralization lands in canon-kit's SPEC and README, lifecycle-kit's
  `align.md`, and delegation-kit's dispatch checklists. **No assertion changed** —
  every gate accepts and rejects exactly what it did before. The wording is what
  moved, so that the rules read correctly on a roster that splits an authoring
  stage out and on one that does not.

## Upgrading

Sync the vendored kit directories wholesale at `v0.8.0`, then regenerate the
generated artifacts — the pre-commit hook
(`bash gate-sdk/bin/gen-pre-commit.sh --write`) and the graph projection
(`bash gate-sdk/checks/check-graph.sh --emit > docs/check-graph.html`) — and
run the full battery.

**No allowed reds.** This release tightens no gate, so a clean upgrade turns
nothing red. If you want the new stage, activate it after the sync by adding
`spec` to `LIFECYCLE_KIT_STAGES` and giving it a `scope` predecessor in
`LIFECYCLE_KIT_PREDECESSOR` — set the roster and the predecessor map together,
since a roster member absent from the predecessor map fails config-load
validation. Leave both alone and you keep the five-stage default.

The behavior changes above are declared for reading, not a mechanical scan. If
a gate reds that this note does not name, the upgrade smoke was supposed to
catch it first —
[open an issue](https://github.com/checkwright/checkwright/issues), because
that is a defect in the release rather than work for you.
