# TASK-QUEUE.md — Checkwright work queue

## Iteration: close-loop-hardening  [stage: build]

  The lifecycle-kit gates read the header above and
  `.workflow/WORKFLOW-STATE.txt` (lifecycle-kit/SPEC.md §The state machine);
  queue-kit formalizes the queue format itself and gates this file. One
  iteration per hardening or roadmap unit; [README.md](README.md) maps the
  kits.

---

## New Features

- **manifest-count-shapes** [spec: spec-kit/SPEC-count-shapes.md] —
  check-manifest-count widens to the wedged-modifier and noun-then-range
  shapes with a partitive carve-out; `rules` joins the default collection
  nouns; the dead allowed-phrases default is retired; the count grammar
  factors into lib/spec.sh for the comment-count sibling.
- **comment-count-drift** [blocked-by: manifest-count-shapes] [spec: SPEC-comment-count.md] —
  check-comment-tier gains the
  count-shape override: a count in a full-line comment is flagged even
  inside a blessed directive window, sharing the count grammar and noun
  vocabulary; the source-coupled numeral scan is rejected as FP-heavy.
- **trajectory-closed-row-freeze** [spec: SPEC-closed-row-freeze.md] —
  every range-scoped trajectory column freezes at the close boundary:
  iteration N harvests (close(N-1), close(N)], interstitial commits belong
  to the next row, and post-close queue filings and hotfixes alike leave
  every published row byte-identical.
- **lesson-disposition-traceability** [spec: SPEC-lesson-channels.md] —
  a commit shrinking Lessons Learned must stamp a per-entry disposition
  (rule/task/harvest/discard) into the lesson-evidence file;
  check-lesson-disposition holds the fail-closed contract, hermetic via
  override args.
- **lesson-pub-harvest** [spec: SPEC-lesson-channels.md] — the outbound
  channel: consumer-named harvest tags on a lesson lead line route the
  entry's body to a consumer-configured sink at close triage; this repo
  wires an essay tag into a gitignored harvest feeding the launch-comms
  methodology essay.
- **lesson-context-tag** [spec: SPEC-lesson-channels.md] — the inbound
  channel: a fixed attend tag on a lesson lead line makes queue-index emit
  the lead line into every later session of the same iteration, capped;
  enter-stage scope refuses on a non-empty Lessons section, so the
  injection mechanically dies at the iteration boundary.
- **docs-link-convention-gate** [spec: SPEC-docs-link-convention.md] —
  consumer gate over the docs pages: no directory-target relative link
  (name the file), kit-page back-links carry their section anchor; the
  shape gate beside check-md-refs' resolution.
- **docs-cname-parity** [spec: SPEC-docs-cname-parity.md] — consumer gate:
  docs/CNAME is the single source of truth for the docs host; a tracked
  URL naming a project host alias other than the CNAME host reds; posts
  and fixtures exempt.
- **site-health-monitor** [spec: SPEC-site-health-monitor.md] — own cron
  workflow, deliberately not a gate: probes apex, www, http→https, the
  alternate-domain redirect, and cert expiry against the live site,
  failing to an opened/updated site-health issue, never a red merge; no
  README badge.

## Technical Debt

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
  ungated source of iteration state.
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
  with plugin-marketplace.
- **multi-operator-semantics** [needs-spec] — the lifecycle's state surfaces
  assume one operator: WORKFLOW-STATE stamps, the TASK-QUEUE stage header,
  the per-iteration scratch logs (prompt-friction, knowledge-friction), and
  the committed baselines all carry single-writer semantics. Define merge and
  conflict behavior — concurrent stage sessions, branch-per-iteration vs
  shared master, who may flip the header — before any team pilot; the kits'
  team-readiness rung. Surfaced 2026-07-07.
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
- **launch-comms** [needs-spec] — the promotion arc, sequenced after
  public-positioning lands, the checkwright.dev cutover is live, and a
  first release tag exists: LinkedIn profile update + announcement post;
  a methodology essay on the writing project's Substack cross-posted to
  Dev.to and LinkedIn; one well-timed Show HN / Lobsters submission once
  the demo and docs withstand that traffic; conference CfP targets picked
  by lead time. In-repo residue only (docs/posts/ entries, the tag);
  the campaign itself is operator work. Surfaced 2026-07-09.

## Done

## Lessons Learned
