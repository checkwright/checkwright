# TASK-QUEUE.md — Checkwright work queue

## Iteration: release-in-lifecycle  [stage: build]

  The lifecycle-kit gates read the header above and
  `.workflow/WORKFLOW-STATE.txt` (lifecycle-kit/SPEC.md §The state machine);
  queue-kit formalizes the queue format itself and gates this file. One
  iteration per hardening or roadmap unit; [README.md](README.md) maps the
  kits.

---

## New Features

- **release-in-iteration-lifecycle** [spec: SPEC-release-in-iteration-lifecycle.md] —
  releasing joins the iteration lifecycle: the close template gains a
  release-disposition step + `release-policy` slot (every close executes the
  consumer's release procedure or stamps an explicit `none`), enforced by a new
  `LIFECYCLE_KIT_BOUNDARY_REQUIRE` boundary require-check in `enter-stage.sh`
  (default empty); this repo binds the slot to RELEASING.md's reordered
  per-iteration procedure with `.workflow/release-disposition.txt` as evidence.
  Operator rulings in the amendment: tag per qualifying iteration; both-None
  tags nothing and stamps `none`. Surfaced by operator directive during
  config-seam-hardening close, 2026-07-17, prompted by the unreleased-backlog
  gap (141 unreleased commits post-v0.1.0; the v0.2.0 cut fixed the backlog,
  this entry is the methodology change).
- **claude-md-housekeeping-residency** [spec: SPEC-claude-md-housekeeping-residency.md] —
  CLAUDE.md §Housekeeping (72 of 200 always-loaded lines, the only section
  carrying mechanism inline) sheds its mechanism to a new load-triggered
  `docs/site-architecture.md` (off-nav by design; the no-owning-kit ruling
  stands per operator ruling in the amendment), the docs bullet collapsing to a
  pointer and the whole section swept to one-line-plus-pointer shape;
  regen-command residency is oracle-safe (each freshness gate's red names its
  regen command — verified). Surfaced 2026-07-17 by the config-seam-hardening
  close brevity pass.

## Technical Debt

- **release-major-criteria-pre-1-0-tension** — order docs/install.md
  §Versioning's Major bullet and Pre-1.0 qualifier explicitly, per the operator
  ruling at release-in-lifecycle scope (2026-07-17): the Major criterion is
  absolute — a decommission earns a major even pre-1.0 — and the qualifier
  scopes to breaking changes *other than* decommissions riding minors. Keeps
  release-sweep's no-marker-rides-past-the-major constraint anchored while 0.x.
  Doc-ordering fix, no new names. Surfaced 2026-07-17 deriving the v0.2.0 bump.

## Deferred

- **rendered-site-link-monitor** [needs-spec] — durable coverage for the
  reader-facing link liveness of the rendered checkwright.dev site. Internal
  and external link rot recurs, and the tree-side reference gates
  (check-md-refs, check-docs-nav-reachable, check-docs-render-fidelity) plus
  the site-health.yml deployment probe cover render and deployment truth but
  not the rendered-site external-URL crawl a reader actually hits. A hermetic
  gate is ruled out on record: site-kit/SPEC.md §The monitor boundary —
  external-link liveness reds on causes no commit produced (DNS, a moved
  target, an incident), breaking the low-false-positive contract. So the
  durable form is a **monitor**, a scheduled crawl step extending site-kit's
  site-health.yml, signalling through an issue and its own red run, never a
  blocked merge. Demand-gated like the other rungs: promote when the one-time
  launch crawl (launch-readiness-gate validate) shows recurrence worth
  automating. Surfaced 2026-07-16 in the launch triage that scoped
  launch-readiness-gate.
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
  Includes the experiment's measurement half: a **stage-burn meter** landing
  in drift-kit's bin/ on the overhead-meter pattern (sessions-dir resolution,
  config via env, advisory exit-0) — per-stage, per-model token burn read off
  harness transcripts and price-weighted, replacing the local-only prototype
  scripts parked in `.metric/`. Nearer use: verifying the split-lead posture's
  savings (lifecycle-kit/templates/lead.md §Economics). Surfaced 2026-07-15
  by the per-stage budget analysis that motivated that posture.
- **prose-profile** [needs-spec] — the non-code universality rung: a third
  consumer shaped as a prose/documentation repo (no build, no test suite)
  stress-tests whether the kits govern non-code work. Core dilution is ruled
  out on record — if pursued, this is an adapter/profile delivered as
  optional consumer config, never a kit literal (the provenance seam).
  Demand-gated: it attests only when a non-code consumer actually vendors a
  kit and hits friction; until then this entry is the roadmap marker. Seeds:
  gate-sdk, guard-kit, context-kit, drift-kit, and canon-kit's
  one-owner/coupling core are workflow-agnostic today; lifecycle-kit's stage
  semantics, evidence-kit's test baseline, and canon-kit's spec framing are
  software-coupled — the abstraction axis is "code + spec" artifacts
  generalizing to "governed surface". `check-prose-tells` (the
  launch-readiness-gate build) is the first concretely prose-shaped kit
  mechanism and the natural profile seed. Surfaced 2026-07-16 in the same
  launch triage that scoped launch-readiness-gate.
- **hosted-attestation-service** [needs-spec] — the team/paid rung: gates
  verified server-side by a party the committing agents cannot touch —
  hosted gate runs as a neutral attestation, cross-repo drift dashboards,
  maintained rulesets. A service, not code: cloning the kits does not clone
  the neutrality or the ops. Demand-gated — this entry is the public
  roadmap marker, not a scaffold; hosting and sequencing decisions are on
  record in the operator's local brief, and multi-operator-semantics
  is its prerequisite mechanism. Surfaced 2026-07-07.
- **smoke-violation-fail-open** [needs-spec] — the consumer-smoke
  `violation.sh` scripts fail **open**: run outside their entry point they
  mutate the invoking repo. The asymmetry is at the read sites —
  `smoke/install.sh` opens with `: "${SMOKE_KIT_ROOT:?run via
  run-consumer-smoke.sh}"` and refuses; `smoke/violation.sh` carries no guard
  and proceeds. Blast radius is the whole roster: nine kits ship an unguarded
  violation script — gate-sdk, lifecycle-kit, queue-kit, evidence-kit,
  delegation-kit, context-kit, doctrine-kit, site-kit, canon-kit — and
  delegation-kit's goes further, `git add`-ing into the index a concurrent
  session shares. Observed, not theorized: a bare invocation wrote
  `scripts/check-smoke-gate.sh` and `product/app.txt` into the real tree and
  staged both; the pre-commit foreign-path check caught it and nothing reached
  a commit.
  **Cost while deferred:** every bare `violation.sh` invocation stays a live
  contamination hazard whose only backstop fires *after* the mutation, and the
  class stays ungated — a new kit inherits the gap silently.
  The missing check class is **smoke-script-fails-open**: a kit script that
  mutates the invoking repo when run outside its entry point. Ruled shape (b),
  on record: the guard **plus** a gate-sdk §Consumer smoke contract sentence
  and a meta-gate asserting it across the roster, the way the fixture-pair and
  fail-closed contracts already are — `install.sh`'s guard is the shipped
  precedent. Enforcement-first: the fix and the gate that catches it land in
  one unit, and a contract sentence plus a meta-gate is what removes the
  duplication of the guard's claim across nine hand-copied sites. Guarding the
  nine alone was rejected — it leaves the class ungated, so kit #10 ships
  unguarded; flagging without filing was rejected under the gap-disposition
  doctrine, and this entry is that filing. Seam: the guard and the contract are
  kit mechanism; nothing consumer-specific belongs in either. Surfaced
  2026-07-16 dogfooding a bare `violation.sh` invocation during
  config-seam-hardening build.

- **spec-internal-identifier-prefix-drift** [needs-spec] — SPEC prose naming a
  script's **internal** variable spelling where the public knob is the contract
  name. Found by the config-seam-hardening close audit of the
  `internal-identifier-restatement` roster class, and fixed there: seven sites
  in delegation-kit/SPEC.md named `PAUSE_PCT`, `PAUSE_PCT_7D`, `LOGIN_WINDOW`,
  `REFRESH_CMD`, `REFRESH_MIN_AGE` — each of which exists in
  `bin/usage-verdict.sh` only as a local assigned straight from its
  `DELEGATION_KIT_`-prefixed env knob. The same doc's §Layout roster names the
  prefixed spelling correctly, so the drift was prose-vs-roster *within one
  file*.
  **Cost while deferred:** the fix is a rename away from rotting — renaming the
  local in the script silently falsifies the prose, and only the roster class's
  audit cadence catches it, at iteration granularity.
  The parent class is on the audit roster precisely because it is **un-gateable**
  (public contract names are legitimate citations). This entry is the narrower
  sub-class that does look gateable: a backtick-quoted `^[A-Z][A-Z0-9_]*$` token
  in a kit SPEC that appears in that kit's source *only* as a local assigned from
  a `<KIT>_`-prefixed env var must be cited by its prefixed spelling. The
  prefixed counterpart's existence is what bounds the false-positive surface — an
  internal constant with no public counterpart never fires. Needs spec because
  that boundary is the whole design: proving it is tight enough for a
  low-false-positive gate is the open work, and if it is not, the honest outcome
  is to record that here and leave the class to the audit cadence. Seam: the gate
  is generic mechanism; the `<KIT>_` prefix is already each consumer's config.

- **upgrade-note-nongate-change-slot** [needs-spec] — the release note's grammar
  has no slot for a behavior change that is not a battery gate, so the two most
  consequential changes in `v0.2.0` could not be declared where a mechanical
  reader looks. `docs/install.md` §The upgrade contract fixes two sections:
  **Tightened gates** ("one bullet per gate that landed new or got stricter, the
  gate name the bullet's lead token" — read mechanically as the release's
  allowed-red set) and **Renamed knobs**. Neither can express: (a) the
  config-seam fail-closed convergence, where `gate-sdk/lib/gate.sh` is sourced by
  every gate so a set-but-missing `<KIT>_CONFIG_FILE` reds the whole battery at
  once — no single gate name to lead a bullet with, and enumerating the roster
  would be a lie about what moved; or (b) `ek_diff`'s fail-closed convergence,
  which reds `bin/run-validate.sh` — not a battery gate, so it has no place in a
  set defined as the battery's allowed reds. Both went to the note's Upgrading
  prose, which is the honest call under the grammar (over-declaring is safe for
  the upgrade smoke but pollutes the contract) and is why this is a grammar gap
  rather than an authoring defect.
  **Cost while deferred:** a consumer reading the two sections mechanically —
  which the contract invites, since the lead tokens are specified as machine-read
  — misses every non-gate change, and `gate-sdk/bin/upgrade-smoke.sh` cannot
  assert containment for them because its assertion is defined over the battery's
  red set. The prose carrying them is unenforced: no gate holds it, so the next
  release's non-gate change lands in prose or nowhere by the author's judgment
  alone. The class is not rare — `v0.2.0` produced two in one release.
  **The design question** (unanswered): a third fixed section (a "Behavior
  changes" body with its own lead-token grammar and a `None` default, parseable
  the way the first two are), versus widening the allowed-red set's definition
  past the battery to any shipped runner, versus ruling the prose sufficient and
  saying so on the page so the silence is a decision rather than an omission. If
  a third section, it needs a reader before it is worth a grammar: a field with
  no named reader should not exist, and today upgrade-smoke is the only
  mechanical reader in sight. Seam: the note grammar is this repo's doc; the
  upgrade smoke that reads it is gate-sdk mechanism, so a grammar change lands on
  both sides of the seam and must keep the kit half generic. Surfaced 2026-07-17
  authoring the `v0.2.0` note.

## Done

- lead-task-selection-seam

## Lessons Learned
