# SPEC amendment: stage-routing

## What changes

The engineering-craft rules reach the stage sessions where they apply,
instead of waiting behind a deliberate link-follow. The constraint held
throughout: a *stage* is itself a load trigger, so stage-scoped surfacing is
consistent with Load-trigger residency **provided it ships pointers — rule
name plus link — never the prose**, and only the stage-relevant subset.

**Mapping ruling: the rule owns its stage tag (derived mapping, option 3).**
Each craft rule in `DOCTRINE.md` gains a machine-parsable trailer line,
`*Stages:* <stage>[, <stage>…]` or `*Stages:* —` for none, naming the kit's
default stage vocabulary. Single source: the mapping lives on the rule, so a
re-vendored DOCTRINE.md carries its own routing and no consumer-side
stage↔rule table exists to drift.

**Emitter.** New `bin/stage-rules.sh <stage>` (doctrine-kit): scans
DOCTRINE.md's craft block for rules whose `*Stages:*` line names the given
stage and emits one pointer line per hit (rule number + name + the doctrine
path). Unknown stage names yield empty output — graceful by design, which is
also the reshaped-machine posture: the tags name the kit-default stages, and
a consumer with a renamed stage set gets no routing rather than wrong
routing (stated honest limit; a remap knob is deferred until such a consumer
exists).

**Surfacing seam.** context-kit's session-context hook template gains an
optional step (the drift-line seam precedent): the hook already derives the
current stage from the queue header; if the doctrine emitter is present it
emits the stage's pointer block. This repo's `scripts/session-context.sh`
copy adopts the step (dogfood). Per-session cost: a few pointer lines, only
on stages that have routed rules. Sibling-slot note:
`context-kit/SPEC-env-profile.md` adds its own optional step to the same
numbered hook list this iteration; the two are order-independent — each
appends after the list's tail at its own merge time, and whichever merges
second appends again, renumbering nothing.

**Parity gate.** `check-doctrine-registration` extends: every craft rule
carries exactly one `*Stages:*` trailer matching the grammar (comma list of
lowercase tokens, or `—`) — so a re-vendored DOCTRINE.md that adds an
untagged craft rule reddens instead of silently dropping out of the routing.
Stage-name *validity* is deliberately unasserted (doctrine-kit does not
depend on lifecycle-kit's config; the emitter's empty-output posture covers
it). Fixture pair updated.

**Guards half ruled out for now.** The point-of-use `guard_advise` extension
(rules like config-edits-are-merges pushed at the triggering action) is
deferred: guard-kit's seam is Bash-only today (PreToolUse matchers: Bash,
Agent — verified), and the triggering actions ride the harness Write/Edit
tools the guard never sees. A Write-seam guard is a new mechanism with one
tentative consumer — demand-gated; the derived surfacing already reaches
every build session. The history-rewrite advisory (Bash-visible) stays as
is.

**Tag seeding (build worklist).** Initial stage tags, judged per rule:
spec-invariant test naming → build, validate; test-from-real-runtime →
build, validate; inspectable-run → build, validate; rename-full-sweep →
build; config-edits-are-merges → build; volatile-state-before-rewrite →
build, close; cross-repo governance → build; qualifier-dropping naming →
scope, build; reuse-co-located-data → scope, build; resolver-flagged-key →
build. Numbers follow the gap-disposition renumber (11–20), which lands
first.

## Producers and consumers

- **Producer:** DOCTRINE.md's `*Stages:*` lines (authored with the rule,
  held by the extended registration gate); the emitter derives, the hook
  step triggers at SessionStart.
- **Consumer:** the stage session reading its context brief; the pointer's
  named reader is the session deciding to follow the doctrine link before
  the matching action.
- **Existing integration prose updated:** doctrine-kit/SPEC.md gains the
  emitter contract and the tag grammar under its DOCTRINE.md/registration
  sections; context-kit/SPEC.md §The session-context hook gains the optional
  step (the drift-line precedent paragraph is the model).

## Existing sections updated

- `doctrine-kit/DOCTRINE.md` — trailer line on every craft rule.
- `doctrine-kit/SPEC.md` — tag grammar, emitter contract, extended
  registration-gate contract.
- `context-kit/SPEC.md` §The session-context hook — optional routing step.
- `context-kit/templates/session-context.sh` + this repo's
  `scripts/session-context.sh` — the step itself.
- `doctrine-kit/gate-tests/check-doctrine-registration/` — fixtures gain the
  trailer assertions.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section; the merged doc reads alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component.
- [ ] **Removals propagated** — nothing retired.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed
      as debt tasks.
