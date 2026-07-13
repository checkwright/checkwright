# TASK-QUEUE.md — Checkwright work queue

## Iteration: orchestration-residue  [stage: close]

  The lifecycle-kit gates read the header above and
  `.workflow/WORKFLOW-STATE.txt` (lifecycle-kit/SPEC.md §The state machine);
  queue-kit formalizes the queue format itself and gates this file. One
  iteration per hardening or roadmap unit; [README.md](README.md) maps the
  kits.

---

## New Features

## Technical Debt

## Deferred

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
  deprecation-lifecycle's. Extended 2026-07-12 (operator ask — does upgrade auto-clean the
  customer slot fills the new template absorbs): the shim upgrade path splits three ways, and
  only the third needs design here. (1) slot-set drift — a template that adds, drops, or
  renames a declared slot reddens the consumer's shim at `check-skill-binding` (it binds the
  template's exact slot set), so Phase-B names the orphaned or missing slot. (2) verbatim
  absorption — `check-shim-restatement` couples the stage-skill templates into its dedup corpus
  and re-fires when the new template lands, so a slot fill the template now covers by a 9-word
  run reddens automatically, naming the shared phrase. (3) semantic residual — a slot fill the
  new template now *means* to cover but the consumer worded differently passes both gates (the
  n-gram holds copy-shape only, the SPEC's stated honest limit) and is un-gateable; it belongs
  in this rung's phase-B disposition skill as a judgment step (surface the changed slots and
  shim slot values, judge redundancy) — the ungateable-class-audit-cadence pattern with the
  upgrade event as the cadence. So the smoke asserts (1) and (2) mechanically; the skill owns
  (3).
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
- **pub-index-language-plugins** [needs-spec] — context-kit's `pub-index.sh`
  hardcodes the Rust grammar and file glob; make the extractor pluggable —
  per-language extractors resolved registry-style, a consumer knob naming the
  enabled set, Rust demoted from the tool's identity to its shipped default
  extractor, and the session-context nudge line reworded to match. Surfaced
  2026-07-12 as site feedback on context-kit's README.
- **positioning-harness-emphasis** [needs-spec] — the harness-agnostic story is true but
  defensively framed: `docs/positioning.md` §The tiered compatibility claim already states Tier
  one (the gate battery is bare bash — runs under any harness, any CI, or no harness), but it
  sits as bullet one of a "tiered, not blanket" hedge while the surrounding pages lead
  Claude-Code-first. Foreground the no-harness-dependency enforcement core on `docs/index.md`
  and `docs/methodology.md`. Hard constraint (honesty/provenance seam): do not claim tested
  compatibility with AGENTS.md / `.cursorrules` that does not exist — this is an emphasis pass on
  the already-true claim; its truthful expansion is harness-adapter-agnostic. Surfaced 2026-07-13.
- **harness-adapter-agnostic** [needs-spec] — the truthful widening of Tier-two compatibility:
  today the lifecycle stage skills, the CLAUDE.md load convention, and the settings pins are
  Claude-Code-native (positioning.md §The tiered compatibility claim names other rules-file
  conventions an untested adaptation surface, not compatibility). Build the adapter — an
  AGENTS.md (and kin) load-convention shim plus a settings-pin port — so the multi-harness claim
  expands because it became true, never by overclaiming. Anti-drift shape to design: parity
  between the ported load surface and the Claude-Code one. Prerequisite for
  positioning-harness-emphasis to assert more than Tier one. Surfaced 2026-07-13.
- **footprint-drop-word-measure** [needs-spec] — the footprint table's compound cell carries
  three measures (`53l · 408w · ~669t`); the token figure is the budget unit but a labeled
  bytes/4 estimate, and exact lines are the repo's native editing currency — words are neither,
  so drop the word measure from `context-kit/bin/footprint.sh`'s cell (emitter, prose in the
  emitted page body, and the "exact counts" framing all adjust together). Machine extraction is
  unaffected either way — `scripts/gen-value-rollup.sh` parses the compound cell and reads only
  the token figure; re-check its awk survives the narrower cell. Ruled against splitting the
  cell into per-measure columns: the analyzing party is a script that already extracts, and
  column-scanning alone does not buy the width. Surfaced 2026-07-13.

## Done

## Lessons Learned
