# TASK-QUEUE.md — Checkwright work queue

## Iteration: footprint-and-parity  [stage: scope]

  The lifecycle-kit gates read the header above and
  `.workflow/WORKFLOW-STATE.txt` (lifecycle-kit/SPEC.md §The state machine);
  queue-kit formalizes the queue format itself and gates this file. One
  iteration per hardening or roadmap unit; [README.md](README.md) maps the
  kits.

---

## New Features

- **runner-doc-off-resident** [spec: SPEC-runner-doc.md] — move the fixture-runner
  registration signal off the always-loaded surface: `GATE_SDK_RUNNER_DOC` defaults to the
  README, whose "This repo, governed" section gains the battery block CLAUDE.md sheds; the
  CONTRIBUTING pointer becomes true; hook + graph artifacts regenerate with the manifest edit.
- **consumer-footprint-budget** [spec: SPEC-consumer-footprint.md] — context-kit/SPEC.md
  gains §The consumer footprint: the one-pointer-line budget rule, the per-kit resident-ask
  roster by citation, and the floor-holder ruling (meter + baseline ship in the consumer
  install; the hold is advisory by design — attribution makes a level gate noisy).
- **delegation-rules-parity** [spec: SPEC-rules-parity.md] — the template owns the protocol
  rules (bold lead-ins as stable names); the SPEC drops the numbered roster, keeps rationale
  and mechanism contracts citing rules by name; new gate check-rule-citation holds forward
  name-citation liveness with the standard fixture pair.
- **doctrine-rule-lockstep** [spec: SPEC-doctrine-digest.md] — extend
  check-doctrine-registration to per-rule bidirectional digest coverage (methodology rule
  name ↔ digest bold lead-in) with fail-closed digest-section resolution via
  `DOCTRINE_KIT_DIGEST_SECTION`; gate-economy weighed in the amendment: owed, because
  re-vendor upgrades stale the digest by construction.
- **lifecycle-knob-prefix** [spec: SPEC-knob-prefix.md] — rename the bare `LIFECYCLE_` knob
  roster to `LIFECYCLE_KIT_`, including the config seam
  (`LIFECYCLE_KIT_CONFIG_FILE` / `lifecycle-config.sh`); compat ruling: hard rename, no shim
  — pre-first-tag renames are compat-free, stated as precedent in the merged SPEC.

## Technical Debt

## Deferred

- **craft-extraction** [needs-spec] — the reusable share of the private consumer's handbook
  is generic engineering craft this repo never extracted: agent working-style habits
  (config-edit hygiene, cross-repo governance reads, resolver-gate fork-not-verdict), git
  operation hygiene (verify HEAD before amend, re-stage after soft reset, fresh `-F` message
  files), and the dispatch/rename checklists (importer survey, collision check, dispatch
  brief, sweep verification). Triage ruling 2026-07-11 at scope: this repo's CLAUDE.md has
  nothing left to move (already pointer-shaped; the provenance seam and kit conventions are
  kit-author content), so the unit is a copy-first extraction from the consumer handbook —
  dispatch/rename checklists toward delegation-kit's template surface, working-style and git
  hygiene toward doctrine-kit's engineering-craft register; design question to rule at spec:
  whether the habit roster earns full doctrine-rule shape (statement/rationale/enforcement
  triple) or a lighter kit surface, and per the seam, attested-failure prose generalizes only
  with product names stripped (identity/key items stay private).

- **scope-session-routing** [needs-spec] — iteration-ambiguity routing across
  sessions: a build/align session forwards a question to the still-live scope
  session and relays the reply back; design it atop the harness's native
  agent-to-agent messaging (SendMessage/subagents) rather than bespoke
  plumbing — the substrate is moving fast and bespoke plumbing would be
  obsoleted; likely a companion tool or repo, not a kit; triage post kit 7.
  Extended 2026-07-07: the full ambition is the scope session as the
  iteration's *lead* — dispatching and supervising the other stage sessions,
  not only answering their questions; Q&A routing is that design's first
  rung. Design tension to rule: the lifecycle is built for stateless sessions
  with evidence stamps as the hand-off, so a live supervisor must leave the
  stamps authoritative — orchestration convenience must not become a second,
  ungated source of iteration state. Boundary note 2026-07-10: this rung is
  communication, not topology — its sessions share one clone and one
  lifecycle state by design; contributor-level branch/worktree strategy is
  multi-operator-semantics' question, and that rung is upstream of any
  team-flavored version of this one. Delegation-parameter note 2026-07-10:
  the lead owns model/effort for the stage sessions it dispatches (whoever
  invokes the dispatch selects; stage shape predicts effort, and the lead
  holds the iteration-wide picture), but sub-agent dispatch inside a stage
  session stays with that session under the resident delegation protocol
  and per-dispatch budget guard — the session's context, not the lead's,
  holds what selection needs. The stamps-authoritative constraint
  generalizes to policy: a lead's delegation preferences land in the
  tracked surfaces sessions already read (agent-definition frontmatter,
  delegation config), never ad-hoc per-session instructions — else the
  lead becomes a second, ungated source of delegation policy. Interaction
  model note 2026-07-10: this rung is *orchestration* (a blocked session
  pauses on its question and resumes in place when the answer arrives —
  the SendMessage substrate), not *delegation* (fire-and-forget: a
  delegated agent that hits a question surfaces it, terminates, and is
  re-dispatched fresh — the resident agent-execution protocol, correct
  for its bounded read/sweep units). The restart-vs-resume cost asymmetry
  is this rung's reason to exist: without routing, a stage session
  blocked on an iteration-ambiguity question forfeits its accumulated
  working state to a terminate-and-redispatch. Economics note, measured
  2026-07-10: subscription cache-forgiveness is NOT documented (API-side
  cache reads are ~0.1x, not free; the window expense of read-heavy
  sweeps is attested in /agent-execution), and the prompt-cache TTL is
  ~5 minutes — so a sporadically-questioned lead pays a full context
  re-warm per cold question. Design accordingly: batch questions rather
  than forward singly, keep the lead's resident context lean (stamps
  stay authoritative, so the lead need not hold what tracked surfaces
  hold), and verify spend via usage-verdict, never assumed forgiveness.
  Chatter-suppression note 2026-07-10: a background session's narration
  never reaches the lead — only pushes cross — so verbosity control is
  channel design: journal for routine findings (pull), SendMessage only
  for the escalation class (the supervisor-owns ruling list, named in
  agent-definition config per the policy-as-config ruling), escalations
  decision-shaped (question/options/recommendation/evidence, budgeted —
  which is also what makes batching natural), with a guard-kit
  SendMessage rule as the mechanical floor (prompts request, guards
  enforce). NOTE at sixth ruling: this entry is past the queue-entry
  altitude — its scope session should open by graduating these notes
  into the amendment rather than extending them.
- **plugin-marketplace** [needs-spec] — harness plugin/marketplace packaging
  of the stage skills and guards; anti-drift gate shape: manifest ↔ shipped
  surface parity. Design against the live manifest format at promotion — the
  plugin substrate moves fast (the scope-session-routing ruling applies).
  Surfaced 2026-07-09 in adoption-track's split; evidence artifact retained:
  upstream Claude Code issue #75214 (project config can't lift the Task
  ask-first default), surfaced dogfooding the delegation nudge 2026-07-07.
- **benchmark-ab-experiment** [needs-spec] — the controlled differential
  experiment: same model, same dependent-task series, two arms (ungoverned
  loop vs Checkwright-governed), drift *accumulation across the series* as
  the metric — a governance layer's effect, not a model leaderboard number.
  Metric axis: Drift-Bench's "satisfiable drift". Substrate/vocab primaries:
  seqBench (arXiv 2509.16866), Drift-Bench (arXiv 2602.02455 — real title
  "Diagnosing Cooperative Breakdowns in LLM Agents under Input Faults via
  Multi-Turn Interaction"; the "Decomposing Reasoning Into Failure Types"
  expansion is confabulated, do not repeat it), Lost-in-Conversation /
  FlowBench as prior art. Surfaced 2026-07-08 inside adoption-track; split
  out 2026-07-09 — the self-referential route (drift-trajectory) ships
  first and this rung upgrades the claim only if demand attests it.
- **upgrade-path** [needs-spec] — the upgrade demo/tooling rung: an upgrade
  smoke that vendors tag N into a scratch consumer, re-vendors N+1 over it,
  and asserts phase-A determinism (nothing changes outside kit dirs plus
  generated artifacts) and that the battery's red set matches the release
  note's tightened-gates declaration; optionally a thin installer CLI
  wrapping the git-native vendor copy (registries stay namespace
  reservations, never a dependency channel). The two-phase upgrade
  contract landed with docs-site (docs/install.md §The upgrade contract); this
  rung is buildable once a second tag exists. Surfaced 2026-07-09. Noted at
  public-positioning scope: no tag exists at all yet — the first release tag
  (a launch-comms prerequisite) is what starts this rung's clock. Ruled
  2026-07-10 at close-loop-hardening scope: the phase-B disposition ritual
  (read the tightened-gates declaration, disposition each new red as
  fix-the-tree vs exempt-with-cause, never weaken the gate) ships as a kit
  skill template beside the smoke — the smoke stays bare bash so a
  harness-less consumer keeps the upgrade path, the skill narrates it
  (lifecycle-kit's templates/skills precedent); harness packaging stays
  with plugin-marketplace. Extended 2026-07-10: this repo's own
  release-prep decommission sweep rides this rung — a deprecated-surface
  inventory beside the tightened-gates declaration, a release-time sweep,
  not a standing gate; the consumer-facing mechanism (queue-bound
  deprecation, the boundary-sweep skill step, the backlog KPI) is
  deprecation-lifecycle's.
- **multi-operator-semantics** [needs-spec] — the lifecycle's state surfaces
  assume one operator: WORKFLOW-STATE stamps, the TASK-QUEUE stage header,
  the per-iteration scratch logs (prompt-friction, knowledge-friction), and
  the committed baselines all carry single-writer semantics. Define merge and
  conflict behavior — concurrent stage sessions, branch-per-iteration vs
  shared master, who may flip the header — before any team pilot; the kits'
  team-readiness rung. Surfaced 2026-07-07. Boundary note 2026-07-10, the
  three-altitude split: sub-agents within one session are already ruled
  (agent-execution's serialize-or-worktree rules, reusable prior art here
  since a contributor's worktree looks the same to git as an agent's);
  sessions within one iteration are scope-session-routing; this rung owns
  the contributor altitude alone — the only one where branch/worktree
  topology is a design decision, because the state surfaces are
  single-writer. Fork contributors are out of scope: an outside PR never
  flips the header or stamps state, it only has to pass the battery in CI
  (the branch-protection/ruleset story), so multi-operator means a second
  operator of the methodology, not a drive-by contributor.
- **hosted-attestation-service** [needs-spec] — the team/paid rung: gates
  verified server-side by a party the committing agents cannot touch —
  hosted gate runs as a neutral attestation, cross-repo drift dashboards,
  maintained rulesets. A service, not code: cloning the kits does not clone
  the neutrality or the ops. Demand-gated — this entry is the public
  roadmap marker, not a scaffold; hosting and sequencing decisions are on
  record in the operator's local brief, and multi-operator-semantics
  is its prerequisite mechanism. Surfaced 2026-07-07.
- **launch-comms** [needs-spec] — the promotion arc, sequenced after
  public-positioning lands, the checkwright.dev cutover is live, and a
  first release tag exists. In-repo residue only (docs/posts/ entries, the
  tag); the campaign itself — channels, venues, timing — is operator work,
  planned in the operator's local brief rather than here. Surfaced
  2026-07-09.
## Done

## Lessons Learned
