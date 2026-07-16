# SPEC amendment: drain-stage-active-residue

<!--
  Delta artifact for check-stage-entry assertion B's active-residue model.
  Owning component: lifecycle-kit (placed here); it also amends queue-kit's
  tag algebra and canon-kit's check-amendment-queue. Paired queue entry:
  **drain-stage-active-residue** [spec: SPEC-drain-stage-active-residue.md].
  Merged into the canonical specs and deleted when the work completes.
-->

## What changes

1. **Case-2 re-decision — ruled: mis-filing, not a gate gap.** Applying the
   operator reframe on record (feature-versus-debt triage is scope work):
   a task surfaced mid-iteration files as Deferred `[needs-spec]` and a
   later scope promotes it. The tag is not "false" on ruled-but-unpromoted
   debt, because `[needs-spec]` means awaiting scope's triage/promotion and
   all deferred work is design-pending by definition (canon-kit/SPEC.md
   §The amendment lifecycle). A designed-debt item therefore never
   legitimately sits in an active section mid-iteration, and assertion B
   models it correctly today. No exemption ships for case 2.

2. **Coupled gate (enforcement-first — lands with the fix):**
   `check-amendment-queue` gains a clause — a `[spec:]`-tagged entry in an
   active non-feature section (Technical Debt under the default
   `CANON_KIT_FEATURE_SECTIONS`/`CANON_KIT_ACTIVE_SECTIONS` split) is
   misfiled: a spec-ready entry belongs in the feature section. New error
   class in the existing single awk pass (the `sec == "active"` branch
   already exists); canon-kit/SPEC.md §check-amendment-queue invariant
   updated; a bad-fixture case added.

   **Knob coupling (noted at align).** This clause reads canon-kit's
   `CANON_KIT_ACTIVE_SECTIONS`, while part 3's assertion B reads
   lifecycle-kit's own `LIFECYCLE_KIT_ACTIVE_SECTIONS` — two independent knobs
   whose defaults coincide (`("New Features" "Technical Debt")`). The two halves
   of this amendment compose only while they agree; a consumer retargeting one
   alone splits them with no gate to notice. Recorded as a known coupling, not
   unified here (kit independence is deliberate and outranks the convenience).

3. **Case-1 model — the `[drain-exempt: <reason>]` tag.** A
   validate-spanning feature (its build half shipped; its validate half
   *is* validate work — launch-readiness-gate the attested first case)
   legitimately persists in the active queue into the drain stage. New
   queue tag, split per the `[needs-spec]`/`[spec:]` precedent: syntax in
   queue-kit (§The tag algebra bullet; `check-tag-lead-line`; `queue-index.sh`
   display), placement semantics in lifecycle-kit.

   **Correction (align, verified at the read site):** there is no governed-tag
   *roster* to join. `check-tag-lead-line`'s `classes()` hard-codes four literal
   awk arms (`blocked-by`, `spec`, `needs-spec`, `attend`); only *lesson* tags
   are configurable (`QUEUE_KIT_LESSON_TAGS`). `[drain-exempt:]` lands as a
   fifth hard-coded arm, and belongs there because its reader is genuinely
   lead-line-scoped: assertion B scans `^- ` lines only.

   Membership in `classes()` tracks **reader semantics, not the tag algebra** —
   a tag earns an arm when its reader scans lead lines alone. `[precondition-ok:]`
   is the contrast that fixes the rule: it is a governed task tag with no arm,
   correctly, because its only reader is reflow-tolerant by construction —
   `check-queue-prose-precondition.sh` sets `hasblock=1` from both the lead-line
   branch (:36) and the continuation branch (:42), so the tag is honored
   wherever a reflow puts it and has no lead-line requirement to enforce.
   `check-stage-entry` assertion B skips an active entry whose lead line
   carries the tag with a non-empty reason (empty reason = malformed,
   still red). **Backstop:** at entry to the drain stage's successor
   (derived from the `LIFECYCLE_KIT_PREDECESSOR` map — the stage whose
   predecessor is `LIFECYCLE_KIT_DRAIN_STAGE`; `close` in the default
   roster — never hard-coded), assertion B runs with **no** exemption:
   nothing may remain active, tagged or not. The drain contract becomes:
   untagged entries drain by drain-stage entry; tagged entries drain by
   successor entry.

   **Successor resolution (specified at align — the map is not injective).**
   `LIFECYCLE_KIT_PREDECESSOR` is many-to-one and the default roster already
   proves it: `align` and `build` both name `scope`. So "the stage whose
   predecessor is the drain stage" is a one-to-many query and needs a rule:
   - **Multiple matches** — every matching stage is a backstop entry; the
     no-exemption assertion runs at each. A tagged entry must therefore drain
     by whichever successor is entered first, which is the bound the backstop
     exists to impose.
   - **Zero matches** (a roster whose drain stage is terminal) — **fail-closed**:
     the exemption is refused outright, because a `[drain-exempt:]` with no
     reachable backstop is a permanent exemption, and a silently-never-running
     backstop is the one failure this design cannot absorb. Consistent with the
     gate framework's `fail_closed` posture.
   `lib/stages.sh`'s roster validator (which today checks only that the map's
   keys and values are known stages) gains the corresponding assertion.

4. **Costs 2 and 3 discharge.** With case 1 modeled, entering validate with
   legitimate residue needs no by-hand override, so the full battery no
   longer holds red for the stage and the whole-suite baseline no longer
   masks intra-validate regressions *for this cause*. This iteration's
   validate promotes the standing baseline row
   (`gates gates fail drain-stage-active-residue` in
   `.workflow/validate-baseline.txt`) back to `pass` — a human commit, per
   evidence-kit's held-constant contract. Residual per-gate baseline
   granularity (a defense against *any* future held-red suite masking a
   fresh red): **ruled (lead ruling, lifecycle-machinery scope session) —
   deferred as its own costed entry**, `per-gate-validate-baseline`
   `[needs-spec]`, filed with this amendment's promotion commit. The
   deferral preserves the operator's one-iteration bundling envelope
   ("split if the envelope grows" governs splitting the bundle, not
   absorbing adjacent scope into it); nothing is blocked by waiting — the
   assertion-B fix removes the standing driver, and the implementation
   path is already consumer-side (an `EVIDENCE_KIT_PARSER` consumer
   command mapping the gates log to per-gate scenario lines plus per-gate
   baseline rows — no kit change expected). Per gap-disposition doctrine
   the deferred entry itself carries the residual risk and that path.

## Producers and consumers

- `[drain-exempt: <reason>]` — producer: the scope session promoting a
  drain-spanning feature writes it on the entry's lead line (or a stage
  session lands it mid-iteration on a lead/operator ruling, in the queue —
  the governed surface — before acting). Consumers: `check-stage-entry`
  assertion B (exemption read at drain-stage entry; no-exemption refusal at
  successor entry), `check-tag-lead-line` (lead-line placement),
  `queue-index.sh` (display). The `<reason>` field's readers: assertion B
  asserts it non-empty and echoes it in its refusal/clean detail (the
  machine read); the audit trail is the semantic reader — the
  `[precondition-ok: <reason>]` precedent. Removal: the entry moves to Done
  when its spanning half completes, taking the tag with it (Done entries
  drop lifecycle tags per the existing merge ritual).
- The `check-amendment-queue` clause — producer: any commit touching the
  queue (precommit tier, existing `# graph:` couple). Consumer: the
  committer, via the gate's refusal text naming the misfiled entry.
- The baseline-row promotion — producer: the validate stage session's
  deliberate commit; consumer: `bin/diff-baseline.sh` / `run-validate.sh`
  per the unchanged evidence-kit contract.

## Existing sections updated

- lifecycle-kit/SPEC.md §check-stage-entry — assertion B prose: the
  exemption, the reason grammar, the successor-entry backstop, and the
  case-2 ruling's one-line rationale (deferred-filing is the model for
  ruled-but-unpromoted work).
- queue-kit/SPEC.md §The tag algebra — the new tag's bullet (syntax here,
  placement semantics in lifecycle-kit — the `[needs-spec]` split
  restated by citation, not copy).
- canon-kit/SPEC.md §check-amendment-queue — invariant clause (a) extended
  with the misfiled-spec-ready case.
- Fixtures: lifecycle-kit `gate-tests/check-stage-entry` (tagged residue at
  drain entry green; untagged red; tagged residue at successor entry red;
  empty reason red); canon-kit `gate-tests/check-amendment-queue/bad` (a
  `[spec:]` entry under Technical Debt); queue-kit tag-lead-line coverage
  for the new tag.
- `.workflow/validate-baseline.txt` — the row promotion at validate.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Cost-3 deferral standing** — the `per-gate-validate-baseline`
      deferred entry stays live (filed at promotion), carrying the residual
      risk and the consumer-side implementation path.
- [ ] **Merged with no information lost** — each addition integrated into
      its proper canonical-spec section (not appended).
- [ ] **Amendment deleted** — this file removed on merge; none remain for
      the component (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed
      as debt tasks.
- [ ] **Baseline honest** — the `gates` row promoted to `pass` in validate
      by deliberate commit; slug liveness green at every intermediate state.
