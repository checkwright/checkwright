# TASK-QUEUE.md — Checkwright work queue

## Iteration: close-loop-hardening  [stage: close]

  The lifecycle-kit gates read the header above and
  `.workflow/WORKFLOW-STATE.txt` (lifecycle-kit/SPEC.md §The state machine);
  queue-kit formalizes the queue format itself and gates this file. One
  iteration per hardening or roadmap unit; [README.md](README.md) maps the
  kits.

---

## New Features

## Technical Debt

## Deferred

- **ddd-positioning-docs** [needs-spec] — docs page plus example consumer
  config positioning Checkwright for DDD ubiquitous-language enforcement
  (vocabulary via the check-graph/graph-vocab pattern, comment-tier
  directives); mechanism kits stay DDD-neutral — the coupling lives in docs
  and examples only; natural landing slot is alongside drift-kit (kit 7).
- **orchestration-positioning-docs** [needs-spec] — the sibling positioning
  page: Checkwright as the verification layer under agent orchestration —
  coordination primitives answer who/when, the gates+stamps+tamper battery
  answers whether the work is right without reading all of it; facilitator
  today, prerequisite for unattended orchestration at scale. Grounded in
  the mechanisms that already exist (validate-after-agent-commit,
  check-gate-tamper, evidence stamps, budget guard) and honest about the
  boundary: the coordination rungs themselves are deferred
  (scope-session-routing, multi-operator-semantics) — the page claims the
  trust layer, never the orchestration layer, and cites those rungs as the
  roadmap rather than hiding them. Angle preserved as prose in the
  orchestration-trust-framing lesson (essay-tagged for harvest); the docs
  page and the essay share the argument, not the text. Like ddd-positioning-docs,
  coupling lives in docs only — no kit gains orchestration vocabulary.
  Surfaced 2026-07-10, asking whether Checkwright is an orchestration
  prerequisite.
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
- **enforcement-map** [needs-spec] — an emitted kit-first map of every
  check surface: kit → governed surfaces → enforcement class, where the
  class spectrum is wider than the gate graph knows — blocking gates by
  tier (the graph manifests already carry precommit/commit-msg/align-only,
  machine-readable today), advisory KPIs (drift-kit plugins, not gates by
  ruling), guards (guard-kit PreToolUse steering), session-context
  warnings, validate suites, and the monitor class (site-health:
  deployment, not tree). Never hand-maintained — a hand-written table is
  the restated projection the copy-gates exist to catch; extend the
  check-graph emitter (or a sibling) to group by owning kit, with the
  non-gate classes declaring their class somewhere parseable (the real
  design work: today only gates carry machine-readable manifests). Emitted
  at docs/ as an adoption page — what each kit enforces and how hard is
  the evaluating adopter's first question, today spread across the kit
  SPECs — freshness-gated by the proven byte-compare pattern (check-graph
  and check-trajectory-fresh precedents). The page becomes the owning home
  for the enforcement-class taxonomy itself, which no single surface owns
  today. Surfaced 2026-07-10 in build, asking whether such a mapping
  exists. Counts toward site-kit-extraction's docs-cluster criterion.
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
- **find-glob-steer** [needs-spec] — a guard-kit rule steering listing-only
  `find` to the Glob tool, the same shape as the existing `sed`-read steer
  (a better form exists, and it returns paths registered for a later Read).
  Scope is the design work: fire only when the invocation carries no action
  predicate (`-exec`, `-execdir`, `-delete`, `-ok`) and is not piped into a
  consumer — a blanket steer would block the legitimate uses, and "is this
  a bare listing" is precisely the logic a permission glob cannot express.
  Ships with decision-table rows for both sides of that line. Evidence: 7
  fall-throughs in close-loop-hardening, every one a plain listing under a
  kit directory (`find <dir> -type f`, `find <kit> -name '*.test.sh'`).
  Surfaced 2026-07-10 at close, tooling-friction triage.
- **prose-enum-drift** [needs-spec] — no gate holds a prose enumeration of a
  governed literal set. `check-kit-enum` owns the doctrine (a hand list of
  two-or-more members must name every member; the fix is a glob token, not a
  longer list) but reads only `# graph:` couples tokens in gate manifests,
  never prose. Attested 2026-07-10 at close: the README queue-kit row read
  "the blocked-by/needs-spec/spec tag algebra" and silently went incomplete
  when the attend tag and the harvest tags landed. Note the evasion shape —
  `check-manifest-count` would have caught "the tag algebra's three tags"
  (a bare cardinal over a governed collection); the row dodged it by
  *enumerating instead of counting*. Same restatement, different surface
  form, no scanner: the `count-scan-wrap-blindness` lesson at a new axis.
  Buildable because the sets are machine-readable (queue-kit/SPEC.md §The
  tag algebra bullets, `QUEUE_KIT_LESSON_TAGS`). Design tension to rule:
  a legitimate subset citation exists ("e.g. `[blocked-by]`"), so the gate
  needs the wedge/exempt escape `check-manifest-count` already carries, and
  the governed sets must declare themselves somewhere parseable rather than
  become a gate literal (the provenance seam).
- **lesson-sink-config** [needs-spec] — the `[essay]` harvest sink is a
  tracked literal, so the harvested material dead-ends in a staging file the
  operator must remember to drain. queue-kit/SPEC.md already rules that a
  tag's *sink* is consumer rule content, but this consumer's close skill
  hardcodes `.workflow/essay-harvest.md` — and cannot do otherwise, because
  `.claude/commands/close.md` is tracked and naming the downstream repo
  there would publish a private path. Ship the sink as a knob read from an
  untracked local config, default `.workflow/essay-harvest.md`: the
  `scripts/msg-patterns.local.list` precedent exactly (gitignored value,
  tracked name, "tracking it would itself be the leak"). Then a harvested
  lesson body appends straight to the operator's downstream backlog and the
  staging file disappears. Design questions for scope: whether the knob is a
  path or a command (a command lets the sink reformat per its own backlog
  grammar), and whether a missing local config is fail-open to the default
  sink or a red close. Sequenced before `launch-comms` — every close until
  then accumulates essay material in a file with a manual reclaim path.
  Surfaced 2026-07-10 at close.
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
