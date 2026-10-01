# SPEC amendment: intake-routing

**A mid-iteration operator request is routed by a stated test, and the doctrine names every route it can take.** doctrine-kit/DOCTRINE.md rule 11 (Scope-gated intake) names two ways work enters: a costed Deferred filing that scope later selects, and an operator-ruled hotfix. The closing stage's gap-inbox drain tries →fix first (lifecycle-kit/templates/stages/close.md, step 2), which resolves a debt-shaped bullet inline in the closing session. That is debt-shaped work, which is no initiative and so sits outside the rule, though the rule does not say so. lifecycle-kit/templates/lead.md has a hotfix dispatch paragraph and no route for an operator's "add X to this iteration". With no stated test, a lead meeting generic wording abandons a correct route for an unsanctioned one: a stage batch it starts itself, writing its own queue entry.

The rule spans two components, doctrine-kit's statement and lifecycle-kit's lead template, so this amendment sits at the repository root.

**Read at authoring:**

- Rule 11's body and its digest trailer (`doctrine-kit/DOCTRINE.md`, rule 11 and the `*Digest:*` line under it). The digest is generated into `CLAUDE.md`'s doctrine block and held in per-rule lockstep by `check-doctrine-registration` (doctrine-kit/SPEC.md §check-doctrine-registration, assertions B, C and F). So a digest edit lands with the block's regeneration through `--install-doctrine`.
- doctrine-kit/SPEC.md's valve paragraph: a valve's wording is statement, and changing it re-rules the valve, which enters as a scoped unit. This amendment is that unit.
- `lead.md` §The escalation protocol carries the hotfix dispatch paragraph and the consult-dispatch paragraph. lifecycle-kit/SPEC.md §templates/lead.md carries their grounds.

## What changes

### (1) Rule 11 names the drain route

{design-bearing} **Not yet applied.** In doctrine-kit/DOCTRINE.md rule 11:

- Before "*Under agent work:*" the rule gains:

  > **The closing stage's drain is outside this rule, because it takes only debt and debt is no initiative.** It is not a valve: it admits no initiative. A gap filed mid-iteration goes to the gap inbox, whether it is a defect a session found or an operator's request for a change that converges on names the governed surfaces already carry. The closing stage's drain fixes it inline, in the session that drains it. A bullet needing a new name, a design ruling, or more than that session holds is promoted to a costed Deferred entry instead, which is this rule's default. The test that routes an operator's request among scope, the drain and the hotfix is the iteration lead's ([lifecycle-kit/templates/lead.md](../lifecycle-kit/templates/lead.md) §Mid-iteration intake).

- The `*Digest:*` line becomes: "a mid-session initiative is filed as a costed Deferred entry by default, never started; work enters only through scope or an operator-ruled hotfix of an impacting failure, minimal and test-and-doc-complete in one commit; a debt-shaped gap is no initiative and is fixed at close's drain."

`CLAUDE.md`'s doctrine block is regenerated in the same commit by the gate binary's `--install-doctrine` arm (doctrine-kit/README.md, the install step). The always-loaded surface grows by the digest's added clause. Where `check-surface-ratchet` reds on it, the growing commit re-stamps with `--emit always-loaded --ceiling`, per §The surface ratchet.

### (2) The lead template carries the routing test

{design-bearing} **Not yet applied.** lifecycle-kit/templates/lead.md gains a section after §The escalation protocol, before §Channel design:

> ## Mid-iteration intake
>
> **Route an operator's mid-iteration request by its shape, never by its wording.** Take the first route that fits. "Add it to this iteration" asks for a timing, which the routes already answer, and moves no request off its route.
>
> 1. **A question rather than work** — answer it, or relay it to the operator as you relay an escalation (§The escalation protocol).
> 2. **An impacting failure** — propose the hotfix with its cost against the iteration's (doctrine-kit/DOCTRINE.md, Scope-gated intake). On the operator's ruling, dispatch it as §The escalation protocol says; without one, take the first later route that fits.
> 3. **Debt** — a change converging on names the governed surfaces already carry, needing no design ruling: file it with `--emit file-gap` and tell the operator the closing stage's drain fixes it this iteration.
> 4. **Anything else** — a new name, a design ruling, or more than one drain session holds: file it the same way. The drain promotes it and the next scope ranks it. Where the operator wanted it in this iteration, say it cannot enter before the next, and write their wish into the bullet's prose as their direction for the next scope.
>
> Never start a request's work in a batch of your own, and never write its queue entry.

### (3) The lead template's grounds

{mechanical} **Not yet applied.** lifecycle-kit/SPEC.md §templates/lead.md gains, after the paragraph **An operator-ruled hotfix is a dispatch, never a track.**:

> **Mid-iteration intake is a routing test, because wording is not shape.** An operator asking for work mid-iteration names a timing, never a route, and a lead reading the timing as the route starts work no stage owns. The test's order is the doctrine's (doctrine-kit/DOCTRINE.md, Scope-gated intake): a question is answered where it stands, a failure is offered to the hotfix ruling, debt rides the drain that already fixes debt inline, and the rest is filed for scope. A refusal to take work into the current iteration is route 4 said aloud, not a fifth route: the lead files and carries the wish, and the next scope decides. The routes end where the lifecycle's capture channels already do: a question at a live lead, work at the gap inbox. So an item another intake turns away has a destination here without a route of its own. **Honest limit:** shape is judged by the lead, and no gate reads a request.

The drain paragraph near line 271 of lifecycle-kit/SPEC.md changes "a defect fixed in one commit adds nothing for scope to weigh" to "debt-shaped work, a defect or an operator's request for a change converging on names the governed surfaces already carry, fixed in one commit adds nothing for scope to weigh".

The section joins the template's whole-protocol list in the paragraph **The template owns the orchestration protocol whole.**, as "mid-iteration intake" after "the escalation protocol and its four-header block".

### (4) The site mirrors and the ratchet follow

{mechanical} `docs/doctrine-kit/DOCTRINE.md` and `docs/lifecycle-kit/SPEC.md` are regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write`, in the commits landing deltas 1 and 3. A governed surface this amendment grows (`CLAUDE.md`, and `lifecycle-kit/templates/lead.md` where `CONTEXT_KIT_RATCHET_PATHS` reaches it) is re-stamped in its growing commit, where `check-surface-ratchet` reds.

## Producers and consumers

- **The routing test (delta 2).** Producer: the lead template, loaded by a `/lead` session. Consumer: the lead, at an operator's mid-iteration request. Its routes write only through channels that already exist:
  - `--emit file-gap` for routes 3 and 4;
  - the escalation relay for route 1;
  - the hotfix dispatch for route 2.

  So no new interface, state or name is minted. The drain consuming a route-3 bullet is close's step 2, unchanged.
- **The doctrine's drain clause (delta 1).** Producer: rule 11's text. Consumers:
  - every session that reads the doctrine through `CLAUDE.md`'s digest or the link;
  - `check-doctrine-registration`, which reds on a digest bullet out of lockstep with its trailer by name or by text, a red delta 1's same-commit regeneration discharges;
  - the stage-rules emitter, which routes craft rules only, so rule 11 is outside its register.
- **Where the narrowed consult intake would send its classes.** The consult inbox currently also takes an operator-class finding a session cannot relay live, a misnamed direction question, and a stage's operator signal. Narrowing it to rulings and strategic opinions, the deferred entry `consult-intake-narrowing`, would route a question-shaped one to a live lead (route 1) and a work-shaped one to the gap inbox (routes 3 and 4). That unit is in no batch of this iteration, so this amendment states one act: the route set already holds both destinations, and that unit owes no route here.
- **Readers whose verdict moves, and their red conditions.**
  - `check-doctrine-registration` assertion F reds on a digest bullet whose text differs from its trailer. It is discharged by the regeneration.
  - `check-surface-ratchet` reds on a governed surface above its ceiling. It is discharged by the re-stamp.
  - `check-brevity` reads `CONTEXT_KIT_BREVITY_SECTIONS`, which in this repo names two `CLAUDE.md` sections and not the doctrine block (`scripts/context-config.knobs`), so it is unchanged.
  - No reader reds on finding none. Nothing narrows.

## Existing sections updated

Roster produced by `git grep -n 'Scope-gated intake\|one valve\|hotfix' -- '*.md' ':!TASK-QUEUE.md' ':!docs'`, with each hit read.

- `doctrine-kit/DOCTRINE.md` — rule 11 and its digest trailer (delta 1).
- `CLAUDE.md` — the regenerated doctrine block (delta 1).
- `lifecycle-kit/templates/lead.md` — the new §Mid-iteration intake (delta 2).
- `lifecycle-kit/SPEC.md` — §templates/lead.md, the grounds paragraph and the whole-protocol list (delta 3).
- `lifecycle-kit/SPEC.md` line 271 — the drain paragraph's "a defect fixed in one commit" generalizes to debt-shaped work, whether a defect or an operator's request (delta 3). The "A parallel hotfix track is refused" sentence near line 121 was read and is unchanged: the drain paragraph already says the drain is not the hotfix track.
- `docs/doctrine-kit/DOCTRINE.md`, `docs/lifecycle-kit/SPEC.md` — the regenerated mirrors (delta 4).
- `.workflow/surface-ceiling.txt` — the re-stamped rows, where the ratchet reds (deltas 1 and 4).

## Retired spellings

- None — no name leaves the tree.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the repository root (`ls SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
