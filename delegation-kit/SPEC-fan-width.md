# SPEC amendment: fan-width

## What changes

The ≤2-wide fan-out bound stops being a kit literal restated in three prose
sites and becomes a knob:

- **`DELEGATION_KIT_FAN_WIDTH`** — new config knob, default `2`, validated
  a positive integer by the lib loader. It bounds **read-only fan-outs
  only**: committing agents serialize or take a worktree regardless — that
  is correctness, never configurable up. The default is the loss-bounding
  invariant (bound the in-flight loss to what the window can absorb) at a
  Pro-class subscription window; a Max-class window supports more, and an
  API-billed operator has no mid-flight wall — there, spend rate and
  provider rate limits replace the rationale. The SPEC states this
  derivation beside the knob so a consumer retunes from the invariant, not
  the number.
- **`usage-verdict.sh` carries the width** — the verdict line gains a
  `width=<n>` field on every verdict (OK/PAUSE/STALE), read from the knob.
  This is the knob's mechanical reader: the budget check already runs
  before each dispatch, so the bound surfaces at exactly the decision
  point, and the agent-budget-guard's advisory text relays it on PAUSE.
  The `usage-tests` suite extends to assert the field.
- **The prose sites cite the knob** — delegation-kit SPEC rule 3, the kit's
  two templates (`templates/agent-execution.md`,
  `templates/claude-md-agent-execution.md` — the kit-owned sources the
  consumer copies derive from; unswept they keep shipping the bare literal),
  `CLAUDE.md`'s agent-execution digest, and the `/agent-execution` skill
  replace the bare literal with the knob name (default noted once, in the
  SPEC).
- **The supervision ceiling is named** in the SPEC beside the knob:
  operator attention over N concurrent reports does not scale with budget —
  a bigger window raises the affordable width, not the reviewable one.

**Ruled out (this iteration): mechanical width enforcement.** The Agent
budget guard fires per-dispatch and sees one tool call, not the in-flight
fleet — counting live dispatches means bespoke plumbing into harness
session state, on a substrate moving fast enough that the plumbing would be
obsoleted (the same ruling that shaped scope-session-routing). The knob is
advisory prose backed by the verdict field. The upgrade path — usage-verdict
deriving a per-wave *suggested* width from last-wave burn, staying
billing-model-agnostic by consuming the verdict rather than window
semantics — is recorded here as the design direction and not built.

## Producers and consumers

- **`DELEGATION_KIT_FAN_WIDTH`** — producer: consumer config
  (`DELEGATION_KIT_CONFIG_FILE`) or env, else the lib default `2`;
  consumers: `usage-verdict.sh` (emits `width=<n>`) and the lib validation
  block (rejects non-positive values).
- **`width=<n>` verdict field** — producer: `usage-verdict.sh`; readers:
  the supervisor running the per-dispatch budget check (sizes the wave
  before dispatching) and the agent-budget-guard advisory on PAUSE. No
  other field changes; exit-code semantics (0 OK / 1 PAUSE / 2 STALE) are
  untouched, so existing callers keep working unread.

## Existing sections updated

- `delegation-kit/SPEC.md` rule 3 (and the rule-8 budget text where it
  names the fan-out): the literal `≤2-wide` becomes "≤`DELEGATION_KIT_FAN_WIDTH`-wide
  (default 2)"; the knob joins the config-surface table; the derivation and
  supervision-ceiling paragraphs land beside it.
- `delegation-kit/SPEC.md` §usage-verdict contract: the verdict-line format
  gains the `width=` field.
- `delegation-kit/templates/agent-execution.md` and
  `templates/claude-md-agent-execution.md`: every restatement of the literal
  (including the loss-bounding rationale line) becomes the knob citation,
  so a fresh consumer copy inherits the knob, not the number.
- `CLAUDE.md` §Agent execution and `.claude/commands/agent-execution.md`:
  the literal is replaced by the knob citation — prose sites cite, the SPEC
  owns.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls delegation-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec, skill, and kit template
      for the bare `≤2-wide` literal; nothing restates the number outside
      the SPEC's default note.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
