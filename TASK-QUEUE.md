# TASK-QUEUE.md — Checkwright work queue

## Iteration: craft-and-routing  [stage: build]

  The lifecycle-kit gates read the header above and
  `.workflow/WORKFLOW-STATE.txt` (lifecycle-kit/SPEC.md §The state machine);
  queue-kit formalizes the queue format itself and gates this file. One
  iteration per hardening or roadmap unit; [README.md](README.md) maps the
  kits.

---

## New Features

- **check-graph-theme-parity** [spec: SPEC-graph-theme.md] —
  a theme-injection seam
  in check-graph's emitter (`GATE_SDK_GRAPH_THEME`, the graph-vocab pattern): three
  optional override functions (css/header/footer), kit default byte-identical when
  unset; this repo's `scripts/graph-theme.sh` supplies Primer-shaped tokens + site
  chrome (light+dark) so `docs/check-graph.html` reads as the same site. Artifact
  stays generated-only; self-containment and assertion-F href policing unchanged.

## Technical Debt

## Deferred

- **template-doc-governance** [needs-spec] — kit template markdown escapes the
  manifest doc gates (link resolution, anti-restatement, command/knob citation):
  their file set is `CANON_KIT_MANIFEST_FILES`, which carries no `templates/`
  glob, so `check-kit-ref-liveness`, `check-tree-terms`, and the purpose-built
  `check-skill-binding`/`check-rule-citation` are a template's only coverage.
  Intentional for slot-bearing binding templates — unfilled `*<…>*` slots and
  CONSUMER BINDING headers would false-positive the finished-prose gates — but a
  slot-free reach-through like `delegation-kit/templates/dispatch-checklists.md`
  escapes link/restatement governance it would pass cleanly; its sibling links
  and SPEC citations hold only by authoring care. Design question to rule at
  scope: a finder discriminator (govern a template iff it has no `*<…>*` slot /
  CONSUMER BINDING header) vs a stated manual duty (the enforcement-first
  false-positive carve-out). The exclusion is implicit — `scripts/canon-config.sh`
  never rules it. Surfaced 2026-07-12 at build, craft-extraction's slot-free
  template. Extended 2026-07-12 at build (scope-session-routing): a sibling
  escaping class, `.claude/agents/*.md` agent-definitions, is neither a template
  nor a manifest doc yet the most citation-dense surface the routing work added
  (`stage-session.md` cites `§Session ritual`, `§Delivery doctrine`,
  `§The state machine`) — widen the design question past `templates/` to any
  non-manifest governed-prose surface. The narrowed residual after the tree-wide
  gates: §heading-fragment liveness (`check-spec-pointer`) and command/knob
  citation (`check-docs-cmd`), which key on the manifest set, are the only axes
  that actually escape; kit-path liveness and banned-terms already cover these
  files. The routing unit shipped both discriminator poles as live examples:
  `lifecycle-kit/templates/lead.md` is the slot-bearing case a blanket
  `templates/` glob would false-positive, `stage-session.md` the slot-free case
  that would pass clean — the pair the finder-vs-manual-duty ruling decides
  against.
- **gap-disposition-doctrine** [needs-spec] — a methodology-maintenance rule for
  the *defer* path: when a session surfaces a gap it will not fix this session,
  it costs the remedy (works out how it could be solved, not just names it) and
  either files that analysis or enriches the standing deferred entry — a bare
  flag-and-skip is the defect. This is the `drift-kit/SPEC.md` knowledge-friction
  loop one altitude up: that loop captures re-derived *facts with no owner*; this
  captures *design/coverage gaps* (the template-doc-governance escape is the
  archetype — mentioned once, it re-pays its own discovery every session). It
  complements, not duplicates, three existing `doctrine-kit/DOCTRINE.md` rules:
  Spec-over-precedent already calls a gap "a doc gap to capture"; Enforcement-first
  names the defect-and-mechanism for a fix being *landed*; neither covers a gap
  being *deferred*. Design questions to rule at scope: (1) new methodology rule vs
  an extension of Spec-over-precedent's capture clause / the knowledge-friction
  loop's convention. (2) Placement ripple if it is a new methodology rule — the
  methodology rules are numbered ahead of the engineering-craft rules, and at
  least one craft rule is cited *by number* from code (the git-rewrite guard's
  advisory in `guard-kit/lib/guard.sh`), so inserting a methodology rule renumbers
  the craft block and dangles that citation; rule whether to renumber-and-fix, to
  migrate craft citations to name-based first, or to place the new rule so no
  renumber is forced. The `CLAUDE.md` doctrine digest is held in per-rule lockstep
  by `check-doctrine-registration`, so the digest line lands in the same unit. (3)
  Enforcement: likely a stated manual duty, not a gate — "did the session cost the
  fix" is not machine-decidable, so the enforcement-first false-positive carve-out
  applies (Spec-over-precedent's "judgment with a capture mechanism, not a gate"
  is the precedent). Surfaced 2026-07-12 at build (scope-session-routing), operator
  ask after the template-doc-governance gap was flagged-and-skipped before being
  enriched on a follow-up prompt — the rule makes the enrich-on-discovery the
  default, not the prompted exception.
- **footprint-page** [needs-spec] — publish the kits' measured context footprint as a
  generated docs page: the always-loaded surfaces and each load-triggered skill/template,
  line/word counts exact and token counts as a labeled estimate (tokenizers are
  model-specific — publish the method, never a false-precision number), total plus per-kit
  split. The adoption-cost evidence a consumer evaluates before vendoring — the
  token-economics positioning made concrete, and the honest complement to context-kit's
  consumer-footprint budget rule (context-kit/SPEC.md §The consumer footprint). Derivation
  discipline: generated + freshness-gated (the evidence-data.md pattern), never
  hand-maintained counts. Design questions to rule at spec: extractor home (extend
  context-kit's always-loaded meter, which owns the metric, vs a drift-kit-style
  extractor beside trajectory.sh); per-kit attribution (a kit's templates and hooks vs
  consumer bindings and consumer config — only the kit share is the advertised cost);
  whether the page rides docs/ evidence framing or the install page. Filed 2026-07-11 at
  scope (operator ask).
- **local-env-profile** [needs-spec] — a kit-owned local machine/environment
  profile so an agent session adapts its guidance to the box it runs on: package
  manager, toolchain versions, shell, absent tools. Mechanism (generic, kit): a
  `bin/env-probe.sh` deriving the profile from `uname` / `/etc/os-release` /
  package-manager + toolchain detection — derivation-first, never hand-maintained.
  Content (consumer-local, gitignored): the probed profile plus the hand-authored
  gotchas a probe cannot know (the OPS.local.md "no `dig`/`host`; use `getent`/DoH"
  class) — `BRIEF.local.md`-class private context, so the machine's facts never
  land in the public tree. Surfacing: context-kit's session-context hook
  (context-kit/SPEC.md §The session-context hook template) gains an Environment
  line, so every session sees it at no resident always-loaded cost. The harness
  already injects a weak version (`Platform: linux`, an OS-version string); the
  value-add is a richer, tool-consumable, harness-independent profile a gate or
  script can read, not just prose in the prompt. Design questions to rule at
  scope: (1) owning kit — context-kit (owns what a session loads) vs a new
  concern. (2) live-probe each session (always fresh, hook latency) vs a cached
  gitignored projection with a freshness nudge (the evidence-data.md pattern) —
  env changes rarely, so a cached probe may win. (3) reconciliation with
  context-kit/SPEC.md §The memory-off doctrine: this is derived, explicit local
  config, not silently auto-accumulated memory, so it is `BRIEF.local.md`-class
  rather than banned harness memory — the SPEC must draw that line. (4)
  enforcement is likely a stated install step, not a gate (env truth is not
  cheaply machine-verifiable and a probe is already the derivation), so the
  enforcement-first false-positive carve-out applies. (5) install-procedure
  placement — a "seed your env profile" step in context-kit's install. Surfaced
  2026-07-12 at build (docs-site-chrome), operator ask — re-derived the Gentoo
  toolchain (portage/eselect, gem user-install path, no `dig`/`host`) while
  standing up a local Jekyll build to verify the docs site.
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
  2026-07-09. First-tag residue noted 2026-07-11 at scope: the README gains a
  release-version badge sourced from the GitHub tag, never from the registry
  placeholders — `reserve/` is a namespace reservation, not a channel, and a
  registry-sourced badge would advertise a dead install path.
## Done

- docs-site-chrome
- craft-extraction
- scope-session-routing
- trajectory-close-freshness

## Lessons Learned
