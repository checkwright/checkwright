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
2b. **The other three readers' violation modes differ from evidence-kit's, and
   only evidence-kit's is silent.** Term 2 orders evidence-kit first because
   its disarm is invisible. Measured at the read sites (align, empirically —
   not inferred), a retired-header-first violation degrades the others thus:
   `session-context` yields empty and falls to its existing `else` branch
   (benign, invisible); `drift-report`'s name axis is unaffected;
   **statusline's `STAGE` sed emits the entire header string** — a non-matching
   `s/^.*\[stage:[[:space:]]*//` strips nothing, so the status line renders
   `## Iteration: <name>` where a stage token belongs. Loud and cosmetic, so it
   does not need term 2's ordering guarantee — but it is the one mode a
   reviewer would read as a rendering bug rather than a missed migration, so
   the statusline migration should not be the batch's last commit.

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
   step; nothing flips. Mid-iteration the queue file is written only for real
   work-state transitions: promotion + naming (scope), the Done move riding
   each amendment-merge commit, and close's dispositions. Stage motion never
   touches it.

   **What the entry re-fire keeps, and what it costs (audited at align).** The
   entry commit still stages the evidence file, so every gate whose `# graph:`
   couples name `.workflow/WORKFLOW-STATE.txt` — `check-stage-entry`,
   `check-stage-evidence`, `check-evidence-manifest`, `check-trajectory-fresh`,
   plus `check-deprecation-task` and `check-todo-task-liveness` via their
   `.workflow/*.txt` glob — re-fires at entry exactly as before. That is the
   half of the enforcement property that survives verbatim.

   The **13 queue-only-coupled gates lose their per-stage-entry re-fire** —
   queue-kit's eight, `check-gate-exemption-tasks`, `check-evidence-baseline`,
   `check-spec-fence-balance`, `check-amendment-queue`,
   `check-lesson-disposition`. Today the flip stages the queue at every entry,
   so all 13 run once per stage whether or not anything they gate changed.
   This is an accepted, costed loss, not a preserved property:
   - Each of the 13 couples at least one *other* input that still fires it on
     the change it actually gates (a queue-only gate reads only the queue, so
     an unwritten queue makes its re-run a no-op; the cross-coupled ones —
     slug-liveness on `scripts/*.sh`, amendment-queue on `SPEC-*.md`,
     lesson-disposition on `lesson-evidence.txt` — fire from the other side).
   - What is genuinely given up is the *periodic sweep*: the guarantee that
     each of the 13 runs at least once per stage regardless of the coupled
     paths, which caught drift introduced out of band (a `--no-verify` commit,
     an edit outside every coupled glob). The full battery at validate is the
     surviving sweep, and it is the stronger one — it runs every gate, not the
     queue-coupled subset. The per-stage-entry re-fire was incidental to the
     flip, never designed as the sweep.

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
- **The no-cursor window (empty or absent state file) — every derivation
  states its fallback.** The header was *always* present and always carried a
  stage, so no reader had an empty case; the state file has two, and one is
  reachable in normal operation: `/scope` truncates at the iteration boundary,
  leaving the file with its prose preamble and `---` but **no data line**
  until the boundary stamp lands. A file absent entirely is the second
  (unvendored or pre-upgrade consumer). Both yield "no cursor", and each
  derivation names its behavior rather than inheriting whatever its parser
  happens to emit:
  - `lifecycle_current_stage` returns empty with status 0 for both cases —
    "no cursor" is a legitimate state, not an error. Its two callers decide:
    `check-stage-entry` reads the preflight temp file, which always carries
    the candidate stamp, so empty there is unreachable-by-construction and
    stays a hard error with the existing "could not parse" shape;
    `check-stage-evidence` keeps its own `[[ -f "$STATE" ]]` guard and its
    current missing-file message.
  - **evidence-kit** — `ek_state_stage` returns non-zero on both, preserving
    `ek_queue_stage`'s current contract, so `check-evidence-manifest:29`'s
    `|| true` keeps yielding an empty `stage`. This is the *same* disarm term 2
    warns about, but here it is correct and already handled: the gate's
    `[[ -z "$iter" || ! -f "$STATE" ]]` early-out (:64) already declares "no
    lifecycle state — close-entry/stamp-coupling disarmed" and exits clean.
    The fixture required by the first Definition-of-Done item must cover this
    window too, or it proves A/C fire only in the easy case.
  - **context-kit** — empty `stage` falls to the existing `else` branch
    (`--collapse-deferred`), which is already the behavior for every non-close,
    non-scope stage. No new branch; the hook must not fail a session.
  - **delegation-kit** — the statusline renders an empty `STAGE` as it renders
    a missing queue today; it must not print a partial parse (see the
    ordering note below).

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
  **The vacuity is conditional and the condition must survive the narrowing**
  (align): the `found` clause (`check-stage-evidence.sh:55`) can only fail
  where the *staleness* assertion (:53-54, "stamp iteration is neither current
  nor a legal '—' bootstrap") has already fired — that assertion is what
  forces `/scope` to rewrite its bootstrap stamp's `—` to the iteration name
  once the header is named. Drop `found`; **keep staleness**, which is now the
  sole enforcer of header/stamp name-axis agreement and therefore *is* the
  narrowed assertion, not a bystander to it. Also unchanged and still
  stage-consuming: the unnamed-iteration guard (:28-32), which reads the name
  axis from the header and the stage axis from the new cursor — it keeps
  working only because the two axes stay independently sourced.
- §templates/skills/ — the skills' first-step prose (stamp, not flip+stamp);
  scope.md's naming step (header edit becomes name-only).
- §templates/lead.md — dispatch prose citing the flip.
- lifecycle-kit/README.md, docs/orchestration.md — flip wording, header shape.
- evidence-kit/SPEC.md §lib/evidence.sh + §check-evidence-manifest — the
  reader swap (envelope term 2 order).
- context-kit/SPEC.md §The session-context hook + knob roster —
  `CONTEXT_KIT_STATE_FILE`, stage-routing source. **Two named rules in that
  section cite the header and must be re-stated, not just re-worded:**
  - **§The header-lag rule** — the lag itself survives unchanged (the hook
    still runs before the arriving skill's first step, which now stamps
    instead of flipping, so a first-of-stage session still reads the
    *predecessor's* value). But the rule's stated *justification* goes stale:
    it argues from "no header value distinguishes [a first-of-stage session]
    from a restarted predecessor session", and the state file's last stamp
    carries a **session id**, so that limit no longer binds. Re-state the rule
    on its surviving ground — the hook reads a cursor written after it fires —
    and rename it (the "header" is no longer the lagging surface). Whether to
    *exploit* the session-id to narrow the accepted over-firing is an envelope
    question, deliberately **not** settled here: see the deferred note below.
  - **§The session-role signal** — its opening clause ("keys every
    stage-conditioned injection off the queue header's `[stage:]`") re-points
    at the state file. The rule's substance is untouched; the marker-file
    mechanism and its session-id scoping are independent of the cursor source.

  **Deferred (filed, not silently dropped):** the last stamp's session id
  makes the header-lag rule's over-firing cost avoidable — the hook could
  compare its own payload session id against the last stamp's and distinguish
  "restarted session of the stamped stage" (ids match) from "new session whose
  stage has not stamped yet" (ids differ), serving the first-of-stage session
  without over-firing to the others. This widens the hook's asserted behavior
  and is out of this amendment's four-reader-migration envelope. It is
  recorded here as the design memory; promoting it needs its own unit.
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
      proves it, not inspection). The fixture covers the **no-cursor window**
      as well (state file truncated to preamble + `---`, no data line): A and C
      must disarm *through the gate's declared early-out* (:64), not by an
      empty `stage` slipping past live assertions — an all-green fixture that
      never exercises the empty case proves only the easy half.
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
