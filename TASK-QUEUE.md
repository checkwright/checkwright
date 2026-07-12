# TASK-QUEUE.md — Checkwright work queue

## Iteration: craft-and-routing  [stage: validate]

  The lifecycle-kit gates read the header above and
  `.workflow/WORKFLOW-STATE.txt` (lifecycle-kit/SPEC.md §The state machine);
  queue-kit formalizes the queue format itself and gates this file. One
  iteration per hardening or roadmap unit; [README.md](README.md) maps the
  kits.

---

## New Features

## Technical Debt

## Deferred

- **craft-rule-stage-routing** [needs-spec] — promote the engineering-craft rules
  (DOCTRINE.md 10–19) into the stage sessions where they apply, instead of the
  passive behind-link reference tier they are today: a craft rule reaches a session
  only on a deliberate link-follow or the single point-of-use guard that pushes
  rule 15 (guard-kit/lib/guard.sh's git-history-rewrite advisory), so the rest
  (rename-is-a-full-surface-sweep, config-edits-are-merges, spec-invariant test
  naming, inspectable-run discipline, reuse-co-located-data) reach a build session
  only if the author reaches for them. The doctrine tension to resolve at scope:
  this must not re-introduce the resident token tax the Always-loaded shape /
  Load-trigger residency rules deliberately avoided — the resolution is that a
  *stage* is itself a load trigger, so stage-scoped surfacing is consistent with
  Load-trigger residency provided it ships pointers (rule name + link), never the
  prose, and only the subset relevant to that stage. Mechanism options to rule at
  scope (cost the remedy): (1) stage-skill citation — each stage skill template
  names its relevant craft-rule pointers at entry; simplest, but a hand-maintained
  stage↔rule mapping drifts against a re-vendored DOCTRINE.md. (2) point-of-use
  guards — extend the rule-15 `guard_advise` pattern to the mechanically-triggerable
  rules (a rename touching multiple surfaces, a config-file rewrite); lowest tax,
  content pushed only at the trigger, but covers only rules with a detectable
  action. (3) a derived stage↔rule mapping — each craft rule declares its own stage
  relevance (a tag on the rule), and the session-context hook or the stage skill
  derives and surfaces the relevant subset; derivation-first, single-source (the
  rule owns its stage tag), generated surfacing. Likely a blend: (3)'s derived
  mapping for the surfacing plus (2)'s guards for the action-triggerable subset. If
  a mapping exists, a parity gate holds it against the rule set — the
  check-doctrine-registration precedent, so a re-vendored DOCTRINE.md that adds a
  craft rule cannot silently drop out of the routing. Complements
  ungateable-class-audit-cadence (that plans audits for what a gate cannot decide;
  this pushes craft guidance to where it applies) and builds on the rule-15
  point-of-use precedent; the Always-loaded shape rule is the constraint it must
  respect. Surfaced 2026-07-12 at validate (operator take, after tracing craft-rule
  residency — only rule 15 has an active push to a stage session).
- **spec-contract-leaning-sweep** [needs-spec] — lean the `## Per-component
  contracts` subsections of the heavy SPECs (gate-sdk/SPEC.md, canon-kit/SPEC.md)
  that restate the source's function/variable roster and step-by-step algorithm —
  the mechanical WHAT the code owns — keeping the invariant, the design rationale,
  and the public-API contract citations while cutting the exhaustive
  internal-identifier inventories and branch-condition narration, pointing to the
  source as owner (the de-literalization move the kit already makes for knob
  values). Verified leaks: gate-sdk/SPEC.md §lib/gate.sh (a function inventory down
  to internal helpers `gate_expand_couples` / `gate_manifest_field` /
  `gate_staged_matches`), canon-kit/SPEC.md §lib/spec.sh (internal awk identifiers
  `SPEC_COUNT_CARDINAL_RE` / `spec_count_awk_lib` / `sk_count_hit` plus walk
  narration), lifecycle-kit/SPEC.md §bin/enter-stage.sh (write-sequence steps + the
  knob roster the script reads), gate-sdk/SPEC.md §check-graph (letter-labeled
  assertion conditions mirroring the check's branches), canon-kit/SPEC.md
  §check-comment-tier (the directive-token set) and §check-manifest-count (the
  cardinal grammar + exemption regex). Why un-gated: every existing
  anti-restatement gate targets a *literal* — check-manifest-count (values/counts),
  check-knob-citation (knob values), check-spec-embedded-source (verbatim copy),
  check-spec-derivable-section (code dumps under banned headings); a prose
  function-inventory is none of those and escapes — the delta between the codified
  de-literalization-of-values rule and the broader WHY-to-SPEC / WHAT-to-code
  aspiration. Design question to rule at scope: extend the de-literalization duty
  to *source identifiers in prose* (name the public contract, never the internal
  roster/algorithm) as a stated authoring rule — no gate, per the Enforcement-first
  high-false-positive carve-out (a SPEC legitimately names public function names as
  contracts, so "prose names a source identifier" cannot be decided cleanly). Scope
  bounded to the two heavy SPECs' lib/* and check-* subsections, not tree-wide;
  this sweep is the first instance the ungateable-class-audit-cadence rule below
  would schedule. Surfaced 2026-07-12 at validate (operator ask, audit of the
  WHY/WHAT SPEC split).
- **ungateable-class-audit-cadence** [needs-spec] — a methodology-maintenance rule
  for the class a gate cannot watch: Enforcement-first's false-positive carve-out
  sends an unformalizable / high-FP defect class to "a stated manual duty", but a
  duty with no cadence is a duty no session performs — it rots exactly like the
  deferred gap the gap-disposition-doctrine rule catches. Propose: when a class is
  un-gateable, plan a *recurring audit* (a named periodic sweep on a lifecycle hook
  — a close-stage roster review, or an every-N-iterations counter) rather than
  leaving the duty cadence-less, so the mechanically-undecidable residual still
  gets a review channel. Relationship to existing rules: extends Enforcement-first's
  carve-out (adds cadence to "stated manual duty"); complements
  gap-disposition-doctrine (that rule costs the remedy when a gap is *deferred*;
  this schedules the un-gateable one no gate can ever close); complements
  Oracle-first (the gate is the mechanically decidable, the audit is its residual).
  Design questions to rule at scope: (1) new numbered methodology rule vs an
  extension of Enforcement-first's carve-out clause — the gap-disposition-doctrine
  placement ripple applies (a new numbered rule renumbers the engineering-craft
  block a by-number code citation in guard-kit/lib/guard.sh depends on; prefer
  extending the clause, or migrate that citation to name-based first). (2) the
  CLAUDE.md doctrine digest is held in per-rule lockstep by
  check-doctrine-registration, so a new rule lands its digest bullet in the same
  unit. (3) where the cadence lives — a close-stage skill step, an iteration
  counter, or a standalone audit skill — and whether the audit roster is a tracked
  derived artifact (un-gateable classes + last-audit stamp, the evidence-data.md
  pattern) so "which audits are due" is derived, not remembered. (4) the rule's own
  enforcement is a stated duty (audit cadence is not machine-decidable) — the
  carve-out is self-applying. Surfaced 2026-07-12 at validate (operator ask, after
  the spec-contract-leaning-sweep gap proved un-gateable).
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
- **faithful-artifact-verification** [needs-spec] — a build/verify discipline for a
  change whose real output is a *deployed or generated artifact* (a rendered site, a
  compiled binary, a published package), not just the tracked tree: the gate battery
  lints the tree and can be all-green over an artifact that is broken, so the artifact
  itself is exercised before the task is called done. Three sharpenings the
  docs-site-chrome work paid for: (1) reachable (the build stage's run-the-system
  guidance) includes *cheap to stand up* — an absent-but-installable runtime is not
  unreachable; the Jekyll gem was one `bundle install` away, and building the site is
  what caught a 404 in every kit page (flipped reference links stayed `.md` against
  `.html` pages) that all 60 gates missed. (2) A local replica must be *faithful to the
  deployment*, version- and plugin-matched: local Jekyll 4.4 raised SCSS deprecation
  warnings that do not exist on GitHub Pages' Jekyll 3.10, and its differing plugin
  defaults are what first hid then exposed the link bug — the `github-pages` gem (the
  pinned deployment toolchain) is the honest oracle; a newer local one both invents
  failures and masks real ones. (3) Gate-green is not artifact-correct: name the
  artifact surface a change carries and verify it, never infer it from a clean tree.
  Complements Oracle-first (which is about running *gates*, not the end artifact) and
  the /verify skill (exercise end-to-end). Design questions to rule at scope: (1) home
  — extend the build stage's run-the-system guidance and/or the Oracle-first rule vs a
  new methodology rule (the gap-disposition-doctrine placement-ripple applies: a new
  numbered rule renumbers the craft block a code citation depends on, so prefer
  extending an existing rule). (2) whether the faithful-replica recipe (pin the
  deployment toolchain, e.g. a vendored `github-pages` bundle kept out of the tree so
  it cannot race the real build) belongs in the /verify skill as a bootstrap step. (3)
  enforcement is a stated duty, not a gate — "did the session exercise the artifact
  faithfully" is not machine-decidable, so the enforcement-first carve-out applies.
  Surfaced 2026-07-12 at build (docs-site-chrome), operator ask after a local Jekyll
  build caught a deployment bug the green gate battery did not.
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

- check-graph-theme-parity
- docs-site-chrome
- craft-extraction
- scope-session-routing
- trajectory-close-freshness

## Lessons Learned
