# TASK-QUEUE.md — Checkwright work queue

## Iteration: public-positioning  [stage: build]

  The lifecycle-kit gates read the header above and
  `.workflow/WORKFLOW-STATE.txt` (lifecycle-kit/SPEC.md §The state machine);
  queue-kit formalizes the queue format itself and gates this file. One
  iteration per kit through extraction, then per hardening or roadmap unit;
  [README.md](README.md) records the extraction order.

---

## New Features

## Technical Debt

- **extraction-desemantics** — sweep the extraction narrative from public
  surfaces: README drops the kit table's Status column and the
  extracted-in-order framing; docs/index.md + docs/methodology.md provenance
  lines go; kit README/SPEC "Extracted from the governance meta-layer…"
  intros and "defaults are the extracted platform's" reword to name the
  single-operator default layout without the origin story; lifecycle
  phrasing simplifies to "one iteration per hardening or roadmap unit"
  (extraction completed; the per-kit clause is history). The seam *rule*
  stays: CLAUDE.md keeps the private-rule-content boundary (config, never
  kit literals), reframed as a provenance/privacy rule rather than an
  extraction story. docs/posts/ are immutable — leave them untouched.
- **contributing-support-model** — replace CONTRIBUTING.md §Bandwidth with
  a support-model section: community support is best-effort with no
  response SLA; paid support, consultancy, and training are offered — name
  a contact route, not an entity. Fixture-first doctrine unchanged.
- **docs-domain-cutover** — move the docs site to checkwright.dev:
  first step dig-verifies the apex already resolves to GitHub Pages and
  stops red if it does not (DNS is a user-side step); then flip docs/CNAME,
  update the docs URL in README.md and any page that names it, and confirm
  Pages re-issues the cert. The retiring docs.vasyltretiakov.dev 301 is
  DNS-side, not in-repo.
- **readme-ci-badge** — add the gates workflow status badge to the top of
  README.md, linking to the workflow's Actions page.
- **docs-kit-page-links** — the docs pages must read in the GitHub tree
  view too (README's "same pages served in-repo" promise): kit-map rows
  link `<kit>/index.md` explicitly (a bare `<kit>/` opens a folder — tree
  view renders README.md, not index.md), and each kit page's back-link
  targets `../index.md#the-kits`, not the page top. Ruling: the per-kit
  dirs stay — stable per-kit URLs with room for extra pages beat one
  bundled index; jekyll-relative-links maps the .md links on the rendered
  site. Rides with it: relax check-kit-registration's row match from the
  literal `](<kit>/)` to the `](<kit>/` prefix, fixture pair updated to
  prove both link forms.

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
  (a launch-comms prerequisite) is what starts this rung's clock.
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
- **docs-link-convention-gate** [needs-spec] — a docs-scoped gate holding
  the link conventions docs-kit-page-links settles: no directory-target
  relative link in docs/ living pages (name the file), kit-page back-links
  carry their section anchor. Cheap and decidable, but let the convention
  prove stable first — promote once the sweep has landed and held through
  an iteration. Surfaced 2026-07-09 in public-positioning scope.
- **launch-comms** [needs-spec] — the promotion arc, sequenced after
  public-positioning lands, the checkwright.dev cutover is live, and a
  first release tag exists: LinkedIn profile update + announcement post;
  a methodology essay on the writing project's Substack cross-posted to
  Dev.to and LinkedIn; one well-timed Show HN / Lobsters submission once
  the demo and docs withstand that traffic; conference CfP targets picked
  by lead time. In-repo residue only (docs/posts/ entries, the tag);
  the campaign itself is operator work. Surfaced 2026-07-09.
- **trajectory-closed-row-freeze** [needs-spec] — a closed iteration's
  published-evidence row must be immutable, but `trajectory.sh` anchors only
  the gate-count column at the close commit; feat/debt and amendment-latency
  run to HEAD for the *last* iteration, so any post-close commit mutates a
  closed row and reds `check-trajectory-fresh` until a regen. Freeze a closed
  iteration's whole range at its close boundary (align the feat/debt/amendment
  windows with the gate-count precedent), so post-close hotfixes never disturb
  a published row. Design note: decide the owner of post-close, pre-next-scope
  commits (currently the last iteration; freezing leaves them unowned, which
  under-counts a cross-boundary amendment — an acceptable edge or not).
  Surfaced 2026-07-09 fixing the adoption-track CI backstop.

## Done

## Lessons Learned
