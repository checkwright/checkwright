# TASK-QUEUE.md — Checkwright work queue

## Iteration: —  [stage: scope]

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
- **docs-render-fidelity-gate** [needs-spec] — GitHub Pages renders through
  kramdown-GFM, which diverges from github.com's cmark: consecutive fenced
  blocks inside one list item corrupt the page (the second fence prints
  literally, a `# contract:` skeleton line becomes a heading), so a
  source-green tree shipped garbled lifecycle-kit and evidence-kit Install
  sections — no gate exercises the rendered artifact. Remedy shape: a
  site-kit gate rendering each docs page with the pinned Pages parser and
  asserting no fence or heading leakage — the faithful-artifact-verification
  class, mechanizable for this artifact. Surfaced 2026-07-12 as site
  feedback; both READMEs restructured to indented code blocks same day, fix
  verified against a local kramdown-GFM render of every docs page.
- **knob-default-source-coupling** [needs-spec] — no gate couples a kit knob's
  default as stated in its owning SPEC to the `:-` fallback in the source that
  actually supplies it, so they drift silently. `check-knob-citation` only bars a
  knob value *outside* the owning SPEC; nothing asserts the in-SPEC default equals
  the source fallback. Surfaced 2026-07-12 verifying `GATE_SDK_GRAPH_ARTIFACT`
  against `gate-sdk/checks/check-graph.sh` during this iteration's identifier
  de-literalization work. Remedy shape: a canon-kit gate extracting each
  `${KNOB:-…}` fallback from kit source and asserting the owning SPEC states the
  same default — the value class of that sweep's identifier de-literalization,
  mechanizable where the identifier class was not. The graph-artifact knob's
  default home was reviewed the same day and kept site-neutral (a docs-site path
  would presume a consumer's publishing convention this repo sets only as an
  override, and the dual-couple manifest exists for exactly that), so the gate
  pins the existing default rather than a new one.
- **harness-layer-positioning** [needs-spec] — a docs positioning section
  ("Where checkwright sits") stating the layer model: checkwright is layer-4
  content (harness-loaded instructions + gates running outside it) that shapes
  and audits behavior layers 1-3 produce, subordinate to a closed layer-3
  harness prompt — issue #75214 (project config can't lift the Task ask-first
  default) is the worked ceiling, already retained under plugin-marketplace.
  Folded with the harness-compatibility statement: a tiered honest claim, not
  blanket compat — the gate battery (gate-sdk, the check-* suite) is bare bash
  and runs under any harness or CI, while the lifecycle skills, the CLAUDE.md
  load convention, and the settings-pin hooks are Claude-Code-native and
  adapter-shaped for AGENTS.md/.cursorrules/Cursor/Codex/OpenCode. Include the
  public memory-off position (durable guidance in tracked manifests, not
  per-user harness memory; check-memory-off enforces), generalized across
  harness memory conventions. Surfaced 2026-07-12 (operator positioning asks
  a+e); confirmed no harness-compat statement exists in docs today.
- **lifecycle-deviation-transitions** [needs-spec] — lifecycle-kit/SPEC.md
  gains a non-linear-transitions section: how to abandon an iteration, split
  one mid-flight, and reopen after close. Has teeth because check-stage-entry
  and the flip+stamp protocol block an ad-hoc abandon today, so each escape
  hatch needs a gate-legal shape (what the header flips to, what stamps, what
  the evidence/baseline obligation is). Surfaced 2026-07-12 (operator ask c);
  confirmed absent from lifecycle-kit/SPEC.md and README.md.
- **model-effort-guidance** [needs-spec] — a principle-level note (not a
  stage-to-model-id roster, which would violate de-literalization and age with
  pricing): scope and align are the reasoning-dense stages — spend model/effort
  budget there; build is the mechanical stage, while validate (regression
  catch) and close (triage/disposition) stay judgment-heavy. Effort-tier, not
  product-named; orthogonal to the existing delegation-kit token lever.
  Surfaced 2026-07-12 (operator ask b). Lowest-priority doc rung.
- **competitive-positioning** [needs-spec] — a single design-philosophy
  contrast paragraph in docs (gates-as-provenance-seam + lifecycle state
  machine vs rules-file / harness-memory approaches), explicitly not a scored
  feature matrix — a maintained comparison table rots on every competitor
  release, the maintain-the-derivable anti-pattern the doctrine bars. Surfaced
  2026-07-12 (operator ask d). Lowest-priority; positioning prose only.
- **os-support-statement** [needs-spec] — docs/install.md gains a
  requirements/prerequisites section, absent today: Checkwright is Unix-first
  (Linux, macOS) and Windows runs via WSL, not natively, because the gate
  battery and hooks are bash plus a coreutils toolchain (the set context-kit's
  env-probe detects — bash, git, awk, jq, python3, shellcheck) with no
  native-Windows shell path. Design tension is derive-vs-restate: env-probe
  already owns that roster (ENV.local.md's generated block), so a hand-kept
  prose list drifts — decide a generated requirements fragment or a single-owner
  list over a restated one, and keep specific min-versions probe-owned, not
  baked. Shape as a tiered honest claim like harness-layer-positioning's (the
  bare-bash battery runs under any POSIX bash + coreutils; the Claude-Code-native
  pieces are the narrower tier), and co-locate the two rungs in one
  positioning/onboarding pass if scoped together. Surfaced 2026-07-12 (operator
  ask).
- **value-rollup-page** [needs-spec] — consolidate the enforcement map and the
  footprint into one nav-visible "these benefits at this token cost" page, led by
  a generated per-kit rollup that joins the two on the kit axis (kit → gates
  enforced → always-loaded + load-triggered cost) with a totals row as the value
  proposition. Absorbs the per-kit-gate-count idea: the gate tally is the benefit
  column, not a footprint add-on. Three spec forks: (1) benefit metric — count by
  enforcement class (blocking gate vs advisory KPI vs guard), never one flattened
  tally that equates a blocker with an advisory; (2) generation — the rollup joins
  two emitters (enforcement-map.sh, footprint.sh), so it needs a joining emitter
  plus its own freshness gate, never a hand-stitched page (derivation-first); (3)
  survival — whether the ~60-row per-gate enforcement detail collapses into the
  page as a drill-down section or stays a linked reference, with no number
  duplicated across surfaces. Clusters with the positioning rungs
  (harness-layer-positioning, competitive-positioning, os-support-statement) as
  one positioning/onboarding docs pass. Surfaced 2026-07-12 (operator ask).
- **docs-nav-ia** [needs-spec] — left-nav information architecture, two parts.
  (a) Grouping: the per-kit entries sit flat at the top level (nav_order in the
  tens), a long run under the guide pages; group them under one "Kit Reference"
  parent so the top level stays short. The nav include supports one nesting level
  only (a nav_id parent with nav_parent children, no grandchildren), so the spec
  fork is flatten — Kit Reference to the kit index pages, each kit's
  README/SPEC/DOCTRINE dropping off the nav tree (still reached from the kit index
  page and client-side search) — versus extending nav.html for a deeper tree;
  flatten preferred. check-docs-kit-parity gates the nav front-matter block on
  every docs/<kit>/index.md, so the regroup moves that gate and its expected block
  in lockstep. (b) Reachability gate: assert every docs page resolves from the nav
  tree except an explicit off-nav allowlist for embedded data fragments
  (evidence-data.md is one) — the silent-orphan failure mode already caught
  enforcement.md (no front matter, unreachable from nav), so Enforcement-first
  over a one-off nav_order add. Shares the nav surface with value-rollup-page
  (which folds enforcement + footprint into one entry); scope the two together to
  avoid churning the nav twice. Surfaced 2026-07-12 (operator ask).

## Done

## Lessons Learned
