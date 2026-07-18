# SPEC amendment: stage-cursor extraction (the four-reader migration)

The fast-mutating `[stage:]` cursor leaves the queue header: the evidence
file's last stamp becomes the cursor's single source and the header flip dies.
Re-promotes the design demoted from mid-iteration-gap-channel-seam during the
concurrency-hardening align audit (operator ruling, Option C; the demotion
commit's parent preserves the original text) — demoted precisely because the
`[stage:]` field proved a **cross-kit interface**, not a lifecycle-internal
copy, so this amendment's envelope is the four-reader migration, ordered.
Premises re-verified against the tree 2026-07-18 at promotion; unit-set ruling
by the operator the same day.

## Envelope terms

1. **The grammar narrows.** The queue header becomes `## Iteration: <name>` —
   the `[stage:]` field is retired from queue-kit's header grammar. The
   cursor's single source becomes the **last data line** of the evidence file
   (`.workflow/WORKFLOW-STATE.txt`): its `<stage>` token already records every
   stage entry, so the header field was a second copy of a derivable fact,
   bought at the price of one queue write per stage entry (derivation-first;
   the header/stamp agreement assertion existed only to hold the copy in
   sync).
2. **Ordering constraint (silent-gate-disarm) — binding, not advisory.** The
   evidence-kit reader migrates **before** the `[stage:]` field retires from
   any live or fixture header. `ek_queue_stage` feeds
   `check-evidence-manifest` assertions A (`stage==close`) and C
   (`stage!=validate`); with the field retired first, the helper returns
   empty and both assertions void **with no red** — the gate disarms without
   signalling. Build lands the evidence-kit migration (with its fixtures) as
   its first commit; the field retires only after that commit is green.
3. **Four readers migrate to self-contained last-stamp derivations** — no
   reader gains a lifecycle-kit dependency; each derives the stage from the
   state file it already can name (config-via-env, gate-sdk defaults):
   - **evidence-kit** — `ek_queue_stage` (lib/evidence.sh) is replaced by a
     state-file reader `ek_state_stage` (last data line's `<stage>` token,
     `EVIDENCE_KIT_STATE_FILE` — already on the kit's knob roster) preserving
     the existing self-contained-reader seam comment;
     `check-evidence-manifest` switches to it. `ek_queue_iteration` stays on
     the queue (the name axis remains the header's).
   - **context-kit** — the session-context hook's stage routing (the kit
     template and this repo's `scripts/session-context.sh` copy) reads the
     state file's last stamp; new knob `CONTEXT_KIT_STATE_FILE` (default
     `${GATE_SDK_WORKFLOW_DIR:-.workflow}/WORKFLOW-STATE.txt`) joins the kit's
     roster beside `CONTEXT_KIT_DRIFT_REPORT`/`CONTEXT_KIT_STAGE_RULES`.
   - **delegation-kit** — `templates/statusline-usage.sh` derives `STAGE`
     from the state file's last stamp (same repo-root-relative hardcode style
     as its existing `TASK-QUEUE.md` read — a consumer-owned template, no new
     knob); `ITER` keeps the header read.
   - **queue-kit** — grammar owner: SPEC prose and `templates/TASK-QUEUE.md`
     narrow the header to `## Iteration: <name>`; `[stage:]` leaves the
     grammar. No queue-kit code parses the field (verified at promotion:
     `bin/`+`checks/` grep-clean; only gate-test fixture headers carry it).
4. **The flip dies; the protocol renames.** §The flip+stamp protocol becomes
   **§The stamp protocol**: the arriving stage's skill *stamps* as its first
   step; nothing flips. The enforcement property is preserved — the entry
   commit still stages the evidence file, and every queue/state-coupled gate
   re-fires on it via the graph couples, so the prior stage's
   machine-expressible exit is still re-verified at entry. Mid-iteration the
   queue file is written only for real work-state transitions: promotion +
   naming (scope), the Done move riding each amendment-merge commit, and
   close's dispositions. Stage motion never touches it.

## Producers and consumers

- **Cursor (last stamp line).** Producer: `bin/enter-stage.sh` appending the
  stamp (unchanged writer, now sole). Lifecycle-internal consumers: a new
  `lib/stages.sh` helper `lifecycle_current_stage <state-file>` (the
  last-data-line read `enter-stage.sh` already performs inline, hoisted so
  every lifecycle reader shares one derivation) — used by
  `checks/check-stage-entry.sh` (derives the entered stage; today it parses
  the header) and `checks/check-stage-evidence.sh` (name-axis agreement only,
  below). `lifecycle_header_stage` is retired with no remaining reader.
  Cross-kit consumers: the three self-contained derivations of envelope
  term 3 (each kit's own reader, its own path knob — deliberately *not*
  `lifecycle_current_stage`, which would cross the no-lifecycle-dependency
  seams).
- **Preflight hand-off.** `enter-stage.sh` today hands `check-stage-entry` a
  temp *queue* with the flipped header; post-change it hands a temp *evidence
  file* with the candidate stamp appended (queue passed through unchanged).
  Same refuse-before-write contract, including the
  `LIFECYCLE_KIT_ENTRY_PREFLIGHT` hook, whose commands receive the same
  `<queue> <state>` argv — the temp file swaps sides.
- **Residual `[stage:]` healing.** Producer: the first-stage boundary reset
  rewrites the header and drops a residual pre-upgrade `[stage:]` field.
  Consumers: the header name-axis parsers (`lifecycle_header_iter`,
  `ek_queue_iteration`, statusline's `ITER` strip, drift-report's awk
  terminator — all already strip-or-terminate on an optional trailing
  bracketed field), so a vendored consumer upgrades mid-iteration without a
  red and the field disappears at its next iteration boundary.

## Existing sections updated (at merge)

- lifecycle-kit/SPEC.md §The state machine — the two governed surfaces: the
  header carries the slow axis (iteration name) only; the evidence file
  carries stamps *and is the cursor*; "every flip and stamp" wording.
- §The flip+stamp protocol → §The stamp protocol (rename + body).
- §Deviation transitions — "header/stamp agreement" citation in the split
  ritual re-words to the name-axis rule.
- §Multi-operator semantics — queue-contention prose updated (stage motion no
  longer writes the queue; the queue leaves the per-stage write set).
- §lib/stages.sh — helper roster: `lifecycle_current_stage` in,
  `lifecycle_header_stage` out; `lifecycle_registration_block`'s emitted text
  ("its flip+stamp protocol") re-worded, which cascades to the consumer
  registration block (CLAUDE.md, in lockstep via
  `check-lifecycle-registration`).
- §bin/enter-stage.sh — non-first entries write only the evidence file; the
  preflight temp-file swap; residual-field healing at the boundary reset.
- §check-stage-entry, §check-stage-evidence — cursor source; the evidence
  gate's agreement assertion narrows to the name axis (the "current stage has
  a matching stamp" clause is vacuous once the last stamp *is* the stage).
- §templates/skills/ — the skills' first-step prose (stamp, not flip+stamp);
  scope.md's naming step (header edit becomes name-only).
- §templates/lead.md — dispatch prose citing the flip.
- lifecycle-kit/README.md, docs/orchestration.md — flip wording, header shape.
- evidence-kit/SPEC.md §lib/evidence.sh + §check-evidence-manifest — the
  reader swap (envelope term 2 order).
- context-kit/SPEC.md §The session-context hook + knob roster —
  `CONTEXT_KIT_STATE_FILE`, stage-routing source.
- delegation-kit/SPEC.md — statusline stage-source sentence.
- queue-kit/SPEC.md + templates/TASK-QUEUE.md — header grammar narrowing.
- drift-kit/bin/drift-report.sh — name-axis awk keeps working unchanged (the
  `[stage:` terminator simply never fires once the field is gone); the dead
  terminator literal is removed in the sweep so no `[stage:` token dangles.
- Fixture/smoke sweep: every fixture, smoke script, and gate-test sandbox
  carrying a `[stage:]` header rewrites to the new grammar + a state-file
  stamp where the scenario needs a cursor — lifecycle-kit (gate-tests +
  smoke), evidence-kit (check-evidence-manifest.test.sh), queue-kit
  (queue-index.test.sh), canon-kit (check-deprecation-task.test.sh + smoke),
  drift-kit (smoke), site-kit (render-fidelity tree content). docs/ kit
  mirrors regenerate.

## Definition of Done

- [ ] **Ordering honored** — the evidence-kit reader commit precedes any
      commit retiring a `[stage:]` header; `check-evidence-manifest`
      assertions A and C demonstrably still fire post-migration (fixture
      proves it, not inspection).
- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as
      one coherent document a reader who never saw the amendment can use
      alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec, template, doc, fixture,
      and smoke script for `[stage:`, `flip+stamp`, and
      `lifecycle_header_stage`; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed
      as debt tasks (a build-time causal gap is resolved that session, not
      deferred).
