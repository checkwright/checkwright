# SPEC amendment: doctrine-single-source

## What changes

- `templates/agent-execution.md` becomes a binding-shim *template* in
  lifecycle-kit's grammar (lifecycle-kit/SPEC.md §check-skill-binding —
  the gate as specced accepts a template owned by any kit): its two
  HTML-comment consumer sections convert to named slots
  `*<shared-file-roster: ...>*` and `*<validate-battery: ...>*`, and its
  header's copy-instruction becomes a bind-instruction — the consumer
  creates `.claude/commands/agent-execution.md` as a shim naming this
  template and binding exactly those two slots. The gate's verdict logic
  accepts a template owned by any kit unchanged, but its `# graph:`
  couples name only the lifecycle skills-template dir — the couples
  widen to cover this template's path so an edit to it re-triggers the
  gate at commit (regenerated hook + graph artifact ride the landing
  commit).
- `templates/claude-md-agent-execution.md` **retires** (deleted). The
  consumer's CLAUDE.md block shrinks to the residency-earning minimum:
  the consumer's pre-authorization sentence (consumer judgment on what
  delegation is pre-approved) plus the full-protocol pointer
  (`/agent-execution`). Rationale, recorded in the SPEC: the residency
  rule — a rule is resident only when it has no load trigger — and every
  digest rule triggers at `Agent` dispatch, which already has a
  mechanical hook seam (the per-dispatch budget guard). The doctrine
  lives behind the trigger, not resident.
- The budget-guard template's block message names `/agent-execution`, so
  the mechanical seam surfaces the protocol at the exact load trigger.
  Honest limit, stated in the SPEC: the guard enforces budget
  mechanically, not protocol literacy — a session that dispatches without
  invoking the skill carries only the resident pointer.
- Consumer-side (this repo) rides the unit: `.claude/commands/agent-execution.md`
  converts to a binding shim (roster slot: the CLAUDE.md serialize list's
  members; battery slot: the full battery plus the touched kit's fixture
  runner); CLAUDE.md §Agent execution shrinks to the pre-authorization
  sentence plus the pointer, deleting every bullet the template owns.
  `scripts/agent-budget-guard.sh` re-copies from the guard template when
  its block message gains the pointer — no gate couples the pair, so the
  re-copy is named here (it also squares the copy's `# spec:` line,
  which differs from the template's).

## Producers and consumers

- Producer of the resident pointer: install time — the consumer pastes
  the shrunk CLAUDE.md block (the install docs section that today says
  "copy `claude-md-agent-execution.md`" changes to carry the two-line
  block inline). Producer of the mechanical pointer:
  `templates/agent-budget-guard.sh` on a blocking verdict.
- Consumer: the delegating session — the Skill tool loads the shim, the
  shim's bindings supply roster and battery, the template supplies the
  protocol. `check-skill-binding` enforces the slot pairing on the
  consumer side.
- Retired template: after deletion, every reference to
  `claude-md-agent-execution` is swept from specs, docs, and installers —
  its content has no surviving reader.

## Existing sections updated

- delegation-kit/SPEC.md §Two templates, one protocol — rewritten: one
  template, bound as a skill; the CLAUDE.md block is a pointer, never a
  digest (the section title itself changes; inbound references updated).
- delegation-kit/SPEC.md §Layout and configuration — the template roster
  drops the digest, and the consumer-install steps swap copy-the-digest
  for paste-the-two-line-block plus create-the-shim.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls delegation-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
