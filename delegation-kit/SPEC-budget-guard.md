# SPEC amendment: budget-guard

The dispatch charter requires a fresh `usage-verdict.sh` reading before
every Agent dispatch and forbids eyeballing the raw percentage — yet the
norm relies on the agent choosing to run the tool, and a session quoted
~5% from memory while the live verdict read 29%. This amendment removes
the choice: a PreToolUse hook on the Agent tool surfaces the live verdict
at the exact decision point. Surfaced 2026-07-08.

## What changes

- **New template `delegation-kit/templates/agent-budget-guard.sh`** — a
  PreToolUse hook registered with matcher `Agent`, composing guard-kit's
  `lib/guard.sh` primitives. It runs `bin/usage-verdict.sh` and routes on
  the exit code:
  - **PAUSE (1)** → `guard_block`, self-describing: the verdict line plus
    the corrective form (wait for the window reset, or the user re-runs
    with the budget knob deliberately raised). This is guard-kit's one
    sanctioned fail-closed shape — the hook *matcher* proves the tool
    identity, so a deny is safe.
  - **STALE (2)** → `guard_advise`: budget unknown is decision-relevant;
    the advisory carries the verdict output (which already names the
    re-read/refresh corrective). Never a block — a consumer with no
    snapshot producer must not be wedged out of delegation.
  - **OK / RESET-OK (0)** → `guard_advise` injecting the verdict line as
    `additionalContext`, so the fresh reading rides in context at every
    dispatch and a memory-quoted percentage can never be the acting source.
- **Rulings recorded** (the three the queue entry demanded):
  - *Per-dispatch freshness, not SessionStart-only* — the observed failure
    was mid-session staleness; a session outlives any start-of-session
    reading. The hook fires at the decision point by construction.
  - *delegation-kit owns it* — the kit owns the verdict, its thresholds,
    and the dispatch protocol the hook enforces; guard-kit supplies only
    the framework primitives (its second consumer; cite-only there, no
    guard-kit mechanism moves).
  - *Block on PAUSE only* — PAUSE is reachable only through a fresh,
    readable, over-threshold snapshot, so blocking cannot wedge a consumer
    without a producer (they route to STALE → advise). Registration itself
    stays the opt-in valve: a consumer that wants pure advice simply does
    not register the hook.
- **Consumer instantiation in this repo:** copy to
  `scripts/agent-budget-guard.sh`, register under PreToolUse matcher
  `Agent` in `.claude/settings.json` (beside the existing Bash guard). The
  consumer session brief (`scripts/session-context.sh`, this repo's copy of
  the context-kit template) additionally prints the verdict line at
  SessionStart for planning-time visibility — a consumer-side edit; the
  context-kit template stays uncoupled from delegation-kit.
- **Testing:** decision-table cases land beside the verdict tests in
  `delegation-kit/usage-tests/`, driving the template with injected
  snapshot fixtures (OK → allow+advise, PAUSE → block, STALE → advise,
  unreadable → advise) and asserting the hook-protocol output, the
  guard-tests pattern. The gate contracts do not apply — a hook speaks
  exit-2 + hook JSON, not the gate output contract.

## Producers and consumers

- **Producer:** the harness fires the hook on every Agent tool call once
  the consumer registers it in settings (this repo's registration is part
  of this task — the enabling config is set, not test-only). The verdict's
  own producer chain (statusline → `usage.txt`) is unchanged.
- **Consumer:** the dispatching agent, by two mechanisms — the
  `additionalContext` advisory on allow, the stderr message on block. The
  supervising user consumes the block by ruling (raise the knob or wait);
  the agent never overrides.
- **Fields:** the verdict line (pct/age/resets_in + verdict word) is read
  by the agent at the dispatch-decision transition; exit code read by the
  guard's single routing transition. No new persistent state, no new keys
  on the `usage.txt` contract.

## Existing sections updated

At merge into delegation-kit/SPEC.md:

- §The delegation model: the budget-check step names the hook as its
  mechanical enforcement (the manual pre-dispatch run remains the planning
  tool).
- §Layout and configuration: templates roster gains
  `agent-budget-guard.sh`; registration recipe added beside the existing
  template docs.
- §Testing: gains the decision-table paragraph above.

Cross-kit, cite-only: guard-kit/SPEC.md §The guard framework gains a
one-line cite naming delegation-kit's budget guard as a framework consumer;
this repo's CLAUDE.md budget-check bullet gains "(enforced per-dispatch by
the Agent budget guard)".

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
