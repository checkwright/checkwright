# TASK-QUEUE.md — Checkwright work queue

## Iteration: doctrine-kit  [stage: align]

  The lifecycle-kit gates read the header above and
  `.workflow/WORKFLOW-STATE.txt` (lifecycle-kit/SPEC.md §The state machine);
  queue-kit formalizes the queue format itself and gates this file. One
  iteration per hardening or roadmap unit; [README.md](README.md) maps the
  kits.

---

## New Features

- **doctrine-kit** [spec: SPEC-doctrine-kit.md] [blocked-by: fixture-suite-derivation]
  — the experience-packaging rung: `doctrine-kit/DOCTRINE.md` carries the
  generic delivery rules (referenced in place, re-vendor is the upgrade),
  `install-doctrine.sh` inserts the always-loaded reference block,
  `check-doctrine-registration` holds it present; CLAUDE.md §Single source
  of truth converts to consumer-by-reference; the companion-corpus sweep
  extends the roster — mechanism crosses the seam, rule content never.
- **read-command-guard-steer** [spec: guard-kit/SPEC-read-steer.md] — three
  steer rules beside `guard_rule_sed_file`: bare `cat` → Read, print-only
  `find` → Glob, working-tree `git grep` → Grep; piped/heredoc `cat`,
  mutating `find`, and history-searching `git grep` pass; fixtures per the
  decision-table contract.

## Technical Debt

- **gate-tamper-rename-policy** — ruled at scope: fold `docs/` and `.github/`
  into `DELEGATION_KIT_META_PATHS` in scripts/delegation-config.sh — public
  docs and CI are governance surfaces in this repo, and the mandatory
  `docs/enforcement.md` regen was forcing every new-gate commit through a
  one-off `--no-verify`; covers the rename and new-gate cases alike.
  Consumer config only — the gate carve-out and standing-`--no-verify`
  options are rejected; delegation-kit mechanism is untouched.
- **fixture-suite-derivation** [blocked-by: gate-tamper-rename-policy] —
  ruled: derive where executable, cite where prose — the CI workflow and
  scripts/evidence-config.sh loop over the `*/gate-tests` dirs instead of
  hand-enumerating suites; the agent-execution validate set cites the
  derivation; the CLAUDE.md battery block stays enumerated
  (check-kit-registration already holds it per-kit). doctrine-kit lands
  behind it as the derivation's first exercised pickup.

## Deferred

- **knob-literal-citation-gate** [needs-spec] — the enforcement half of the
  align knob-defaults finding: the instance is fixed (CLAUDE.md cites knob
  names; each kit's SPEC owns its roster and default values) and this gate
  forbids the slab growing back — no governed surface outside a kit's own
  SPEC states a `<KIT>_` knob's default value. Calibrate on the low-FP
  triad (knob token + default/value marker + non-owning surface); a bare
  number in prose stays a tripwire, not a gate.
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
- **delegation-doctrine-single-source** [needs-spec] — collapse the
  agent-execution doctrine to one source, the delegation-kit template:
  CLAUDE.md §Agent execution shrinks to a resident pointer plus at most the
  pre-authorization sentence, the claude-md-agent-execution.md digest
  template shrinks with it or retires, and the consumer /agent-execution
  skill becomes a binding shim over templates/agent-execution.md (the
  skill-template-binding grammar; check-skill-binding as specced does not
  restrict which kit owns the named template — sequenced after
  skill-template-binding lands). Rests on the residency rule from the
  convention-hardening align review: CLAUDE.md earns a rule's residency
  only when the rule has no load trigger — anything triggered by a stage,
  a skill, or a tool call lives in its owned doc behind a pointer, and the
  Agent budget guard shows dispatch already has a mechanical hook seam.
  Surfaced 2026-07-11. Superseded note (doctrine-kit scope): the residency
  rule's owned home is doctrine-kit's DOCTRINE.md, not context-kit's SPEC.
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
- **stage-shim-dedup** [needs-spec] — closure of the doctrine-kit align
  finding on the stage binding shims: the close shim paraphrases
  guard-kit/templates/close-triage.md and drift-kit/templates/close-knowledge.md
  instead of consuming them by reference — a fork of kit-owned procedure, a
  strict subset when filed but a stranded enhancement the day one lands
  locally — and the build/scope shims restate always-loaded CLAUDE.md facts
  (generated-hook regen, public-repo hygiene, the provenance seam,
  red-gate-never-bypassed) their reader provably has loaded. Fix is
  pointer-form: the housekeeping binding names the two kit templates and
  binds only consumer residue; the restatements trim to citations; the
  build shim's pre-commit battery subset becomes a citation of the CLAUDE.md
  battery block plus the touched kit's suite. Enforcement half, same unit:
  a duplication tripwire — a long shared word n-gram between a binding shim
  and an always-loaded surface or a kit template fails the commit; calibrate
  the n on the current corpus. Genericity itself stays semantic (no scanner
  buildable — the n-gram holds the copy shape, judgment holds the tier).
  Same residency/pointer family as delegation-doctrine-single-source, but
  sequenced independently of it. Surfaced 2026-07-11 at doctrine-kit align.
- **gate-run-for-path** [needs-spec] — the affordance half of the doctrine
  roster's oracle-first rule: expose the generated hook's path→gates
  selection as an agent-callable entry point (`run-gates.sh --for <path>`
  or a sibling gate-sdk bin tool), reusing the `# graph:` couples reader
  gen-pre-commit and check-graph already share, so selector and hook cannot
  desync. The mid-edit loop becomes edit → run coupled gates → read help —
  strictly cheaper than reading gate source to predict a verdict, the
  run-don't-predict equilibrium the oracle-first rule names. Surfaced
  2026-07-11 at doctrine-kit align.
## Done

## Lessons Learned
