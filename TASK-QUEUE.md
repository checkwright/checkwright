# TASK-QUEUE.md — Checkwright work queue

## Iteration: close-loop-hardening  [stage: build]

  The lifecycle-kit gates read the header above and
  `.workflow/WORKFLOW-STATE.txt` (lifecycle-kit/SPEC.md §The state machine);
  queue-kit formalizes the queue format itself and gates this file. One
  iteration per hardening or roadmap unit; [README.md](README.md) maps the
  kits.

---

## New Features

- **site-health-monitor** [spec: SPEC-site-health-monitor.md] — own cron
  workflow, deliberately not a gate: probes apex, www, http→https, the
  alternate-domain redirect, and cert expiry against the live site,
  failing to an opened/updated site-health issue, never a red merge; no
  README badge.

## Technical Debt

- **todo-task-liveness** — the one admitted TODO directive is never
  liveness-checked: check-comment-tier classifies TODO(task: <slug>) as a
  directive but nothing resolves the slug, so a TODO bound to a completed,
  cleared task passes forever — the staleness class its siblings already
  guard (a blocked-by tag is flagged stale on a done slug; exemption
  until: refs validate against live tasks). Converge: every TODO(task:) site on a
  governed source resolves to a live queue slug, stale-flagged otherwise.
  Latent today — no such site exists in the tree yet; land the check
  before the first one does. Same neighborhood, small judgment at fix
  time: whether a narrow trailing-comment scan for bare TODO/FIXME/HACK
  markers is worth it (trailing comments are out of scope by documented
  ruling, but these markers are rarely legitimate trailing content); if it
  grows into a real design question, file it separately rather than rule
  it here.
- **count-scan-wrap-blindness** —
  the shared count adapter matches within one physical line, so a prose wrap
  splitting cardinal from noun evades both restated-total gates: spec-kit's own
  §lib/spec.sh carried "two comment / gates" across a line break, invisible to
  check-manifest-count while its adjacent twin on one line reds. Candidate: a
  paragraph-joined scan window reporting the first physical line of the span.

## Deferred

- **ddd-positioning-docs** [needs-spec] — docs page plus example consumer
  config positioning Checkwright for DDD ubiquitous-language enforcement
  (vocabulary via the check-graph/graph-vocab pattern, comment-tier
  directives); mechanism kits stay DDD-neutral — the coupling lives in docs
  and examples only; natural landing slot is alongside drift-kit (kit 7).
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
  lead becomes a second, ungated source of delegation policy.
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
  the neutrality or the ops. Separate private repo, created only when
  adoption demand shows up — this entry is the public roadmap marker, not a
  scaffold; natural landing post adoption-track, and multi-operator-semantics
  is its prerequisite mechanism. Surfaced 2026-07-07.
- **site-kit-extraction** [needs-spec] — the docs/site governance cluster
  keeps landing as root amendments and consumer gates (docs-site, the
  link-shape and CNAME-parity gates, the site-health workflow): extract the
  generic mechanism into kit form when demand attests it — a second consumer
  vendoring the kits, or the next docs-scoped consumer gate landing. The seam
  is pre-cut for the lift: conventions, host aliases, and sinks are already
  consumer config, so extraction is mechanical. Likely a split rather than
  one kit: link shape joins spec-kit beside check-md-refs (same governed doc
  set and scan machinery); the deployment-truth pieces (CNAME parity, the
  health-workflow template) are the site-kit. Surfaced 2026-07-10 at
  close-loop-hardening scope, reading the root-amendment accumulation as the
  same copy-first extraction signal the kits themselves came from.
- **memory-off-enforcement** [needs-spec] — the no-per-user-memory stance
  graduates from a CLAUDE.md rule to public doctrine plus enforcement,
  split by the tree-vs-deployment seam: a hermetic settings-parity gate
  pinning the tracked harness config (.claude/settings.json) against a
  desired-state file (the identity.conf pattern), and a
  check-identity-class local gate scanning the harness's per-project
  memory dir and settings.local.json — CI-neutral, clean where the surface
  is absent, the fail-open-on-absent caveat stated honestly, and the paths
  as config knobs (the harness layout moves; the plugin-marketplace ruling
  applies); a session-context warning line backstops harness-layout drift.
  Doctrine and mechanism land in context-kit — memory is an ungated
  always-loaded surface its meter cannot read — with docs/methodology.md
  citing downward. The replacement routes stay the star topology: durable
  facts to doc owners via knowledge-friction, iteration-scoped attention
  to the lesson channels, private context to the local brief. Blast radius
  is bounded (gates hold the tree regardless of a polluted session), so
  the build is a lightweight gate pair, not machinery. Evidence: memory
  measured as a drift surface and its layer deprecated on a larger private
  consumer of this lifecycle; that consumer's memory-folder scan is the
  local half's prior art. Surfaced 2026-07-10 in build.
- **deprecation-lifecycle** [needs-spec] — deprecation with teeth, for
  consumers: the scan itself stays consumer toolchain (clippy/ESLint-class
  linters already inventory deprecation markers, and drift-kit's Out of
  scope pre-rules toolchain-shaped scans as consumer plugins — the marker
  roster is consumer config, a consumer's language is never a kit
  literal); the kit ships the governance coupling no linter has. Three
  pieces: queue-binding — a deprecation marker on a governed surface
  resolves to a live decommission task, the TODO(task:) analog with
  todo-task-liveness' contract; a release-boundary sweep as a skill step —
  at a major, walk the inventory and force a disposition per entry
  (decommission, re-justify carrying the task forward, or un-deprecate;
  the lesson-disposition contract shape at a release boundary),
  upgrade-path's phase-B sibling; and a deprecated-surface example plugin
  on drift-kit's KPI contract so the backlog trends between majors instead
  of surprising at one. Demand-gated: attested practice on a larger
  private consumer of this lifecycle, which is also the anticipated first
  consumer and the prior-art source. Surfaced 2026-07-10 in build.
- **delegation-fan-width** [needs-spec] — the ≤2-wide fan-out bound is a
  subscription-economics number restated as a kit literal in three prose
  sites (delegation-kit SPEC rule 3, CLAUDE.md, the agent-execution
  skill): the invariant is bound-the-in-flight-loss-to-what-the-window-
  can-absorb, and 2 is that invariant at a Pro-class window — a Max-class
  window yields more, and an API-billed operator has no mid-flight wall
  at all, so the loss-bounding rationale vanishes there and spend rate
  plus provider rate limits replace it. Ship the number as a
  delegation-kit knob, default 2, the three prose sites citing the knob;
  width governs read-only fan-outs only — committing agents serialize or
  take a worktree regardless, which is correctness and never configurable
  up. Design questions for scope: whether the Agent budget guard enforces
  width mechanically (it fires per-dispatch and could count in-flight
  dispatches) or the knob stays advisory prose; and the upgrade path —
  usage-verdict already knows pct and reset horizon and could derive a
  per-wave suggested width from last-wave burn, staying
  billing-model-agnostic by consuming the verdict rather than window
  semantics (the pluggable usage source is the precedent: an API
  consumer's verdict carries different axes). Name the supervision
  ceiling in the docs: attention over N concurrent reports does not
  scale with budget. Surfaced 2026-07-10 in build.
- **commit-subject-grammar** [needs-spec] — check-commit-msg is a leak
  guard only (banned patterns); nothing asserts subject shape, yet the
  prefix is load-bearing in two mechanisms no gate backs: trajectory.sh's
  feat/debt column classifies commit subjects, so a mistyped prefix
  silently drifts the published evidence rows, and the closed-row freeze
  leans on docs/chore filings sitting outside that harvest — a property
  held by subject-class convention alone. A subject that does not parse is
  an unread write to a governed projection, not a style nit. Shape: a
  sibling assertion in gate-sdk's commit-msg tier (the hook plumbing
  exists) — subject matches type(scope)?: with the type set a knob whose
  default aligns with kpi-task-split's classification vocabulary; the
  design tension to rule is one vocabulary with two readers (share the
  list vs restate it across gate-sdk and drift-kit). Carve-outs: merge
  commits, reverts, fixup!/squash! autosquash subjects. Surfaced
  2026-07-10 in build, asking whether the convention was already enforced.
- **launch-comms** [needs-spec] — the promotion arc, sequenced after
  public-positioning lands, the checkwright.dev cutover is live, and a
  first release tag exists: LinkedIn profile update + announcement post;
  a methodology essay on the writing project's Substack cross-posted to
  Dev.to and LinkedIn; one well-timed Show HN / Lobsters submission once
  the demo and docs withstand that traffic; conference CfP targets picked
  by lead time. In-repo residue only (docs/posts/ entries, the tag);
  the campaign itself is operator work. Surfaced 2026-07-09.

## Done

- docs-cname-parity
- docs-link-convention-gate
- manifest-count-shapes
- comment-count-drift
- trajectory-closed-row-freeze
- lesson-disposition-traceability
- lesson-pub-harvest
- lesson-context-tag

## Lessons Learned
