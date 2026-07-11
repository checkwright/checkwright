# SPEC amendment: scope-session-routing

Rung 1 of the lead-session design: iteration-ambiguity Q&A routing between a
live scope session (the lead) and the stage sessions it dispatches. This
amendment graduates the queue entry's accumulated note series into one
design and rules its open questions.

Ownership ruling: **lifecycle-kit**, superseding the entry's earlier
companion-tool/companion-repo hypothesis — a dated premise re-verified
against the current substrate: the harness natively supports background
dispatch with resume-in-place messaging, so no bespoke plumbing remains to
house; what remains is protocol prose and policy config, exactly the
templates shape lifecycle-kit already owns, and a companion repo would
rebuild the vendoring/upgrade path the kits have. A new orchestration-kit
is likewise ruled out for this rung: a kit earns its root by owning
mechanism, and rung 1's decomposes entirely into existing charges —
dispatch safety is delegation-kit's (inherited by citation), the
escalation-shape guard is guard-kit's (the framework owner; the
wakeup-guard precedent), and the stamps-authoritative constraint is a rule
about lifecycle-kit's own state machine that lands in its SPEC wherever
the template lives — leaving the kit one template plus pointers, a
directory rather than a remit (contrast doctrine-kit, minted with an
installer, a gate, and an eviction from canon-kit's out-of-scope). The
lead is also lifecycle-shaped, not generically orchestration-shaped: it
dispatches stage sessions, so a consumer without the stage machine has
nothing for it to dispatch. Reopen trigger, recorded: extraction to its
own kit is owed when a later rung grows orchestration *mechanism* with a
non-lifecycle consumer — not when the orchestration prose grows. The interaction-model
boundary stands: this is *orchestration* (a blocked session pauses on its
question and resumes in place), not *delegation* (fire-and-forget
redispatch) — delegation-kit's agent-execution protocol is unchanged and
stays correct for bounded read/sweep units, and sub-agent dispatch inside a
stage session stays with that session under the resident protocol and
per-dispatch budget guard. Topology is out of scope: lead and stage
sessions share one clone and one lifecycle state; contributor-level
branch/worktree strategy is multi-operator-semantics' question.

## What changes

- **New template `lifecycle-kit/templates/lead.md`** plus a consumer binding
  shim (this repo: `.claude/commands/lead.md`), the established
  template-plus-shim shape. It is not a stage skill: it stamps nothing and
  joins no stage roster.
- **The lead model (rung 1 scope).** The scope session may stay live as the
  iteration's lead. It dispatches a stage session as a background agent
  whose prompt is that stage's ordinary skill invocation; the stage session
  executes its stage skill unchanged — enter-stage flip+stamp, commits, and
  every other state write happen in the stage session. The lead's authority
  is dispatch and answers, nothing else. Dispatch mechanics inherit
  delegation-kit's safety rules unchanged (background + notification, the
  per-dispatch budget guard, validate after any agent commit).
- **The escalation protocol.** A stage session that hits a question inside
  the lead's ruling classes ends its turn with a decision-shaped escalation
  block — **Question / Options / Recommendation / Evidence** — batching
  every open question into that one turn-end rather than forwarding singly.
  The lead answers by messaging the paused session, which resumes in place
  with its working state intact — the restart-vs-resume cost asymmetry this
  rung exists to close.
- **Channel design.** Routine narration and findings go to the resume
  journal (pull — delegation-kit's journal mechanics apply unchanged); the
  message channel carries only the escalation classes. Verbosity control is
  channel design, not prompt pleading.
- **Policy as config.** The ruling-class roster — what must escalate versus
  what the stage session decides alone — lives in the tracked
  agent-definition frontmatter the dispatch names, never in ad-hoc
  per-dispatch prose; otherwise the lead becomes a second, ungated source
  of delegation policy.
- **The stamps-authoritative invariant** (the design's load-bearing rule):
  the lead writes no lifecycle state — no WORKFLOW-STATE stamps, no queue
  header flips, no evidence files. Every stamp originates in the stage
  session via enter-stage.sh. An answer that amounts to a design ruling is
  landed by the stage session in the governed surface it belongs to (the
  amendment, the queue entry) before the session acts on it: the message
  thread is transport, never a store, so a lead crash or transcript loss
  costs nothing the tracked surfaces do not already hold.
- **Economics.** The prompt cache's short TTL means a sporadically
  questioned lead pays a full context re-warm per cold question. Hence:
  batch escalations (the decision shape makes batching natural), keep the
  lead's resident context lean (stamps are authoritative, so the lead holds
  pointers, not state), and verify spend with delegation-kit's usage-verdict
  rather than assumed forgiveness.
- **Mechanical floor: an optional SendMessage guard template in guard-kit**
  (the guard-kit/SPEC.md §wakeup-guard (template) precedent — same
  framework, tool-targeted matcher, separate opt-in hook). Registered on
  the message tool in the *stage* session, it advises when an outbound
  escalation lacks the decision-shape headers. The four-header shape
  grammar is kit mechanism; the ruling-class roster stays consumer config.
  Prompts request, guards enforce.

## Producers and consumers

- Escalation block: produced by the stage session's turn-end message and
  returned to the lead as the dispatch result; consumed by the lead at its
  answer transition. Fields and their readers there: Question (what is
  blocked), Options (the choice set), Recommendation (the default the lead
  can rubber-stamp), Evidence (what the session verified). No field is read
  anywhere else; a block missing the shape is the guard template's advisory
  case.
- Answer: produced by the lead's message to the paused session; consumed by
  the resuming stage session, which lands any ruling content in the
  governed surface before acting (the invariant above).
- Ruling-class roster: produced in the consumer's agent-definition
  frontmatter; consumed by the stage session (escalate-vs-proceed decision)
  and by the guard template (its matcher config).
- Guard template: produced by opt-in hook registration in consumer
  settings; consumed by the stage session as an advisory. Enabling config
  is the consumer's settings file — absent registration, the template is
  inert prose, which is the intended default.
- Deliberately produced nowhere: new stamps, state files, or queue surfaces
  — the invariant is that this unit adds no iteration state.

## Existing sections updated

- lifecycle-kit/SPEC.md §The state machine — gains the lead carve-out
  paragraph: an optional live lead never becomes a second state source; the
  flip+stamp protocol is unchanged and remains the only iteration state.
- lifecycle-kit/SPEC.md §Per-component contracts — gains the lead
  template's subsection beside the stage-skill templates.
- delegation-kit/SPEC.md §The delegation model — one boundary sentence:
  resume-in-place orchestration is lifecycle-kit's lead template; this
  protocol stays fire-and-forget.
- guard-kit/SPEC.md §wakeup-guard (template) — gains the sibling
  SendMessage guard template subsection; guard-kit/SPEC.md §Testing — its
  usage-test row.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
