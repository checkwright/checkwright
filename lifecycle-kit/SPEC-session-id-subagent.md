# SPEC amendment: session-id-subagent

## What changes

`bin/session-id.sh` gains a derivation order that survives lead-dispatched
stage sessions (templates/lead.md), replacing the newest-top-level-transcript
scan as the only source. The observed defect class, two halves:

- A dispatched stage session's transcript nests under the *lead* session's
  transcript directory (`<sessions-dir>/<lead-uuid>/subagents/`), so the
  top-level glob derives the lead's own id and `check-stage-evidence` reds on
  the cross-stage collision.
- The documented per-dispatch remedy (point `LIFECYCLE_KIT_SESSIONS_DIR` at
  that `subagents/` dir) derives from basenames of the form
  `agent-<17hex>.jsonl`, so the first-8-chars cut yields `agent-aX` — six of
  the eight distinctness characters are a constant prefix (the prior
  orchestrated iteration stamped `agent-a4`/`agent-a9`/`agent-a0`/`agent-a3`),
  leaving ~one hex digit of cross-stage distinctness.

New derivation order, first hit wins, every path ending in the same
normalization (strip a leading `agent-` token if present, then take the first
8 characters):

1. **`LIFECYCLE_KIT_SESSION_ID`** — harness-neutral consumer override; a
   consumer whose harness exposes a session identity by any means wires it
   here (config-via-env, the kit knob shape).
2. **`CLAUDE_CODE_SESSION_ID`** — the shipped default source: this harness
   exports the session's full transcript uuid into every Bash tool
   environment, which identifies the *current* session directly instead of
   inferring it from file mtimes. Harness-specific by nature, named in the
   SPEC as the shipped default with knob 1 as the portability valve.
3. **Newest-transcript scan, widened** — the existing glob plus
   `<dir>/*/subagents/*.jsonl`, so a dispatched session with neither env var
   still resolves without a per-dispatch override. Newest-wins stays the
   documented single-operator assumption.

The per-dispatch `LIFECYCLE_KIT_SESSIONS_DIR` override remedy is retired from
prose (the knob itself stays — it is the sessions-dir layout knob, not a
collision remedy).

**Probe resolved (align, 2026-07-13):** a dispatched stage session observed
`CLAUDE_CODE_SESSION_ID` carrying the *lead's* uuid with
`CLAUDE_CODE_CHILD_SESSION=1` set — the contingency branch holds. So source 2
is **skipped when `CLAUDE_CODE_CHILD_SESSION` is set** and the widened glob
(source 3) carries dispatched sessions alone; no further build-time probe is
needed. In that case the lead uuid source 2 rejected still narrows source 3:
scan `<dir>/<that-uuid>/subagents/*.jsonl` alone, excluding the lead's own
top-level transcript — which is concurrently written and can out-mtime the
dispatched session's — from the newest-wins candidate set.

## Producers and consumers

- **The derived id** — producer: `session-id.sh` per the order above;
  consumer: `bin/enter-stage.sh` (its only caller), which stamps it into
  `.workflow/WORKFLOW-STATE.txt`; read at the `check-stage-evidence`
  well-formedness and cross-stage-distinctness transitions.
- **`LIFECYCLE_KIT_SESSION_ID`** — producer: consumer config or a dispatch
  environment; reader: `session-id.sh` step 1. Joins the knob roster in
  lifecycle-kit/SPEC.md §Layout and configuration.
- **`CLAUDE_CODE_SESSION_ID`** — producer: the harness process environment
  (nothing this kit emits); reader: `session-id.sh` step 2.
- No new files, no new stamps, no wire-format change — the stamp grammar and
  `check-stage-evidence` are untouched; only id *derivation* changes.

## Existing sections updated

- lifecycle-kit/SPEC.md §bin/session-id.sh: the derivation description is
  rewritten around the three-source order and the `agent-` normalization; the
  lead-dispatch collision paragraph and its `LIFECYCLE_KIT_SESSIONS_DIR`
  remedy sentence are replaced by the resolved behavior.
- lifecycle-kit/SPEC.md §Layout and configuration: knob roster gains
  `LIFECYCLE_KIT_SESSION_ID` (default unset).
- templates/lead.md: no edit — it never named the override; dispatch-prompt
  residency is SPEC-dispatch-policy.md's surface.
- lifecycle-kit smoke: covers env-first, `agent-` strip, and the widened-glob
  fallback (a synthetic sessions tree with a nested `subagents/` transcript).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls <component>/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
