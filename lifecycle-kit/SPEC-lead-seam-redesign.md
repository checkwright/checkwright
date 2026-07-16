# SPEC amendment: lead-seam-redesign

<!--
  Delta artifact for the lead↔machinery seam redesign. Owning component:
  lifecycle-kit (placed here); it also amends context-kit's session-context
  hook. Paired queue entry: **lead-seam-redesign** [spec: SPEC-lead-seam-redesign.md].
  Merged into the canonical specs and deleted when the work completes.
-->

## What changes

Four coupled deltas so the lead trusts the machinery instead of re-deriving
prior-stage completeness by hand:

1. **`bin/enter-stage.sh --simulate <stage>`** — a read-only preflight mode.
   It runs everything a real entry runs up to the write: config load and
   stage validation, header parse, session-id derivation, the idempotence
   probe (a would-be no-op is reported as such and exits 0), the temp
   flipped-queue build, `check-stage-entry`, every matching
   `LIFECYCLE_KIT_ENTRY_PREFLIGHT` entry, and the iteration-boundary
   Lessons check — then stops. No stamp, no header flip, no boundary
   truncation; the temp queue is removed. Every output line is prefixed
   `enter-stage (simulate):` so a transcript can never read as a stamp.
   Exit 0 = the real entry would proceed (or no-op); exit 1 = it would
   refuse, with the refusing check's output verbatim; exit 2 = usage/config
   error, as today. Not a gate — exercised in `smoke/` beside the existing
   enter-stage coverage.

2. **Lead-template ruling (`templates/lead.md`)** — the lead never
   hand-derives prior-stage completeness from WORKFLOW-STATE or git log: it
   dispatches and trusts enter-stage's fail-closed refusal (relayed in the
   stage session's report), or runs `--simulate` first when it wants to
   gate an expensive dispatch cheaply (oracle-first made concrete).
   Lead-does-stamping is ruled out: it breaks §Stamps are authoritative,
   and under the `stage` posture of `LIFECYCLE_KIT_SESSION_BOUNDARY` a
   lead stamp is exactly the self-reported skip `check-stage-evidence`
   catches. Second ruling (the lead-durability gap): a ruling whose acting
   session is not imminent is filed to a durable governed surface (a queue
   entry, an amendment) in the moment it is made — the lead's context and
   the message thread are transport, never a store, and "a ruling lands in
   the stage session that acts on it" holds only when that session exists.

3. **Session-role signal for the session-context hook** — context-kit's
   hook keys every stage-conditioned injection off the queue header's
   `[stage:]` with no signal for whether the reading session is a lead, a
   stage session, or a manual run, so a lead draws executor-facing craft
   rules at every hook fire (startup, and each compact/resume re-fire).
   New signal — **ruled (lead ruling, lifecycle-machinery scope session): a
   marker file written by /lead, session-id-scoped.** /lead's first step
   records `lead <session-id.sh value>` in a gitignored scratch marker
   (`CONTEXT_KIT_SESSION_ROLE_FILE`, default
   `${GATE_SDK_TMP_DIR:-.tmp}/session-role`); the hook — which receives
   the harness session id on stdin, whose 8-char prefix equals what
   `session-id.sh` computes **in a top-level session** (the parity attested,
   and scoped, in context-kit/SPEC.md §The session-context hook, "Ruled out —
   lifecycle stamp-id injection") — treats the session as `lead` only when the
   marker's id matches its own, so a concurrent or later **top-level** session
   never bleeds and a stale marker self-invalidates when the id rotates.

   **Why top-level scoping is sufficient (audited, align/lifecycle-machinery).**
   Both producer and consumer are top-level by construction: /lead runs in the
   top-level session, and `SessionStart` does not fire for Task-spawned
   subagents — the brief reaches a top-level transcript as a hook attachment and
   reaches a stage session not at all. So the only sessions the hook fires in
   are leads and manual runs, and the identity match discriminates exactly
   those. The bleed it prevents is a *manual run in the same tree while a lead
   marker sits in scratch*, not a lead versus its own stage sessions.

   **Named assumption (the match's soundness rests on it):** a stage session
   shares its lead's `CLAUDE_CODE_SESSION_ID`, so were the harness ever to fire
   `SessionStart` in subagents, every stage session would match the marker and
   read as `lead` — inverting the suppression onto exactly its intended
   audience. Build wires the read against the hook's own payload, never against
   the env var, and this assumption is revisited if subagent hook-fire lands.

   **Known, accepted limits (not defects):** the initial startup fire
   precedes /lead by construction, so it renders as today — a bounded,
   one-per-lead-session cost, against the per-compact/resume recurrence
   that is the actual waste. The marker also ages out with the hook's
   day-horizon scratch sweep; a lead session older than a day degrades to
   absent-signal behavior, the same posture as before this change. Finally the
   producer inherits `session-id.sh`'s `CLAUDE_CODE_SESSION_ID` dependency:
   unset, that script falls through to a newest-transcript scan that in a lead
   with live subagents returns an `agent-` prefix the hook's payload can never
   match, and the signal silently no-ops to today's behavior. This is the same
   degradation shape the launch-env-var alternative was rejected for, in a
   strictly better reliability class — harness-set, not operator-remembered —
   and it is recorded rather than guarded: the failure costs a suppression, not
   a correctness property.

   Rejected alternatives (recorded so build does not re-litigate): a
   launch-env var — costs an operator ritual per lead launch in
   perpetuity, and a forgotten export degrades silently to today's
   behavior with no signal that it happened; both-producers-with-
   precedence — two producers plus a precedence rule to spec, gate, and
   keep consistent, for a gap one hook fire wide. The recorded producer
   sketch ("set by /lead") is honored as closely as the harness permits.

   Consumer behavior: when the signal marks the session
   `lead`, the hook suppresses the two executor-facing steps — step 4
   (stage-conditioned nudges) and step 8 (stage-routed craft-rule
   pointers) — and emits everything else unchanged: the queue index, drift
   line, budget line, memory backstop, scratch sweep, index footer, and
   env profile all serve a lead too. Signal absent ⇒ byte-identical to
   today; stage sessions and manual runs never see a difference. The read
   is guarded like every hook step — the hook never fails a session.

4. **`LIFECYCLE_KIT_SESSION_BOUNDARY` unchanged — ruled.** The knob stays
   on the session-span/evidence axis (`stage` | `iteration`); no rename, no
   new value. Manual-versus-lead is the driver/role axis and rides the
   part-3 signal. This lands the queue entry's recorded lean as the ruling;
   the rejected alternative (overloading the boundary knob with role
   values) is recorded here so it is not re-derived.

## Producers and consumers

- `--simulate`: producer — the lead session invoking it before a dispatch
  (templates/lead.md names the call; docs/orchestration.md's lead sequence
  reaches it through that template). Consumer — the lead's dispatch
  decision; the flag's output is read in the invoking turn and stored
  nowhere. No new state, file, or field.
- Lead-template rulings: producer/consumer are prose — templates/lead.md is
  the owner; lifecycle-kit/SPEC.md §templates/lead.md's summary sentence is
  the only other surface, updated in this amendment's merge.
- Role signal: producer — /lead's first step writes the marker
  (templates/lead.md carries the step; the enabling path knob has a kit
  default, so every deployment that runs /lead produces it — not
  test-only). Consumer — the consumer-copy hook
  (`scripts/session-context.sh`) and the kit template
  (`context-kit/templates/session-context.sh`), at steps 4 and 8. Fields:
  the marker carries `role` (read by the hook's suppression branch) and
  `session` (read by the hook's identity match) — both read at the same
  hook-fire transition. No other reader; no field is populated that the
  hook does not read.
- Part 4 introduces nothing — it is a recorded non-change.

## Existing sections updated

- lifecycle-kit/SPEC.md §bin/enter-stage.sh — gains the `--simulate`
  contract (read-only, output prefix, exit semantics).
- lifecycle-kit/SPEC.md §templates/lead.md — summary gains the
  no-hand-derivation and durability rulings.
- lifecycle-kit/templates/lead.md — part-2 prose; `--simulate` named where
  the lead model describes dispatch gating.
- context-kit/SPEC.md §The session-context hook — the role-signal
  conditioning on steps 4 and 8, the marker grammar, and the accepted
  startup-fire limit; §Layout and configuration gains
  `CONTEXT_KIT_SESSION_ROLE_FILE`. Its "Ruled out — lifecycle stamp-id
  injection" parity attestation is **narrowed to the top-level case** (landed
  at align: the unqualified claim is false for a subagent, which is handed its
  parent's `CLAUDE_CODE_SESSION_ID` while `session-id.sh` derives its own
  transcript id).
- context-kit/templates/session-context.sh and scripts/session-context.sh
  (consumer copy) — the guarded role read.
- docs/orchestration.md §Running an iteration under a lead — one line on
  how a lead run carries the signal (cites lead.md, never restates).
- CLAUDE.md — **no delta** (load-trigger residency: the signal loads with
  /lead and the hook; nothing always-loaded changes).
- delegation-kit — **no delta** (dispatch mechanics unchanged; its SPEC was
  consulted for lead economics only).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Role-signal ruling merged** — the marker grammar, the
      startup-fire accepted limit, and the rejected alternatives land in
      context-kit/SPEC.md §The session-context hook at merge.
- [ ] **Merged with no information lost** — each addition integrated into
      its proper canonical-spec section (not appended).
- [ ] **Amendment deleted** — this file removed on merge; none remain for
      the component (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed
      as debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **Smoke coverage** — `--simulate` exercised in lifecycle-kit `smoke/`
      (would-pass, would-refuse, would-no-op; asserts nothing written).
