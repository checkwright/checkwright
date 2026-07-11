# TASK-QUEUE.md — Checkwright work queue

## Iteration: craft-and-routing  [stage: align]

  The lifecycle-kit gates read the header above and
  `.workflow/WORKFLOW-STATE.txt` (lifecycle-kit/SPEC.md §The state machine);
  queue-kit formalizes the queue format itself and gates this file. One
  iteration per hardening or roadmap unit; [README.md](README.md) maps the
  kits.

---

## New Features

- **craft-extraction** [spec: SPEC-craft-extraction.md] — copy-first extraction of
  the generic craft share of the private consumer's handbook: doctrine-kit's
  engineering-craft register grows six working-style/git-hygiene rules (full
  statement/rationale/enforcement triple — the shape question is ruled in the
  amendment), guard-kit's generic ruleset gains the advisory git-rewrite rule, and
  delegation-kit gains the load-triggered dispatch/rename checklist template.
  Provenance: generalized only — product names, identity/key items stay private.

- **scope-session-routing** [spec: SPEC-scope-session-routing.md] — rung 1 of the
  lead-session design: a lifecycle-kit lead template (not a stage skill) dispatches
  stage sessions and answers their batched, decision-shaped escalations; sessions
  resume in place. Stamps stay authoritative — the lead writes no lifecycle state;
  ruling classes live in tracked agent-definition config; an optional guard-kit
  SendMessage template is the mechanical floor. Ownership ruled to lifecycle-kit
  (supersedes the companion-tool hypothesis); topology stays with
  multi-operator-semantics.

- **trajectory-close-freshness** [spec: SPEC-close-freshness.md] — close the
  close-blindness of the trajectory freshness gate: the extractor SPEC gains the
  close-coupling contract (regenerate the projection in the first commit after the
  close stamp lands), this repo's freshness gate widens its `trigger=` to the queue
  file so the Done-clearing commit runs it with the stamp in history, and the close
  binding names the regeneration step. No new scanner — the self-reference is
  inherent; CI stays the outer backstop.

- **docs-site-chrome** [spec: SPEC-docs-site-chrome.md] — the docs site gains its chrome:
  a custom Primer-based layout (header with logo + wordmark, left nav sidebar derived from
  front matter, client-side search over a Liquid-emitted index, light/dark/auto selector
  persisted under the `checkwright-theme` key) and the generated SPEC mirror
  (`scripts/gen-docs-mirror.sh` + `check-docs-mirror-fresh`) so reference reading stays
  on-site — rendered-document links go relative to the mirror, blob links narrow to source
  references (the docs-reference-routing supersession is ruled in the amendment). Logo moves
  to `docs/assets/logo/`; the nav-parity assertion joins `check-docs-kit-parity`.

- **check-graph-theme-parity** [spec: SPEC-graph-theme.md] [blocked-by: docs-site-chrome] —
  a theme-injection seam
  in check-graph's emitter (`GATE_SDK_GRAPH_THEME`, the graph-vocab pattern): three
  optional override functions (css/header/footer), kit default byte-identical when
  unset; this repo's `scripts/graph-theme.sh` supplies Primer-shaped tokens + site
  chrome (light+dark) so `docs/check-graph.html` reads as the same site. Artifact
  stays generated-only; self-containment and assertion-F href policing unchanged.

## Technical Debt

## Deferred

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

## Lessons Learned
