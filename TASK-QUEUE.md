# TASK-QUEUE.md — Checkwright work queue

## Iteration: config-seam-hardening  [stage: close]

  The lifecycle-kit gates read the header above and
  `.workflow/WORKFLOW-STATE.txt` (lifecycle-kit/SPEC.md §The state machine);
  queue-kit formalizes the queue format itself and gates this file. One
  iteration per hardening or roadmap unit; [README.md](README.md) maps the
  kits.

---

## New Features

## Technical Debt

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

- **release-in-iteration-lifecycle** [needs-spec] — releasing is deliberately
  *outside* the iteration lifecycle today: the five stages tag nothing,
  `RELEASING.md` is a separate runbook, and CLAUDE.md holds it load-triggered
  ("resident only at a release"). So an iteration landing phase-B work with no
  version bump is the machinery working as designed, not an oversight. The
  operator's directive is to change that design: **release should be
  incorporated into the iteration methodology — each iteration meeting the
  criteria should bump the version accordingly.**
  **Observed cost of today's shape:** `v0.1.0` was tagged 2026-07-14; 141
  commits have landed since, roughly 38 feat/fix, including many new-or-stricter
  gates. `docs/posts/` holds exactly two entries (the announcement and the
  v0.1.0 note). Every one of those iterations met the minor criteria in
  `docs/install.md` §Versioning and none bumped — the drift is **systemic, not
  an oversight**, because nothing in the lifecycle asks.
  **Why it bites specifically:** the upgrade contract's phase B
  (docs/install.md §The upgrade contract) makes the red set a consumer's
  migration worklist and the release note the source of intent behind it. With
  no note, a consumer syncing past v0.1.0 gets reds with no declaration — the
  contract's second half is unhonorable until a note exists. That is a promise
  already published, not a nice-to-have.
  **Open design questions** (this entry carries the question, not a design):
  does close own the bump, or does a new stage? The criteria are *derived from
  the release note* (§Versioning reads the bump off the note's two sections), so
  an iteration-time bump needs the note authored in-iteration — which reorders
  RELEASING.md's steps 1–3 and touches release-sweep's release-boundary
  contract. Does every qualifying iteration tag, or does it accrue a pending
  bump that the release boundary consumes? What happens when an iteration's note
  sections are both "None"?
  **Seam:** lifecycle-kit is a shipped kit — the mechanism ships generic, this
  repo's release policy stays consumer config.
  A **`v0.2.0` release is being cut immediately** for the existing backlog,
  separately from this entry: that is the backlog fix, this entry is the
  forward-looking methodology change. Provenance: operator directive during
  config-seam-hardening close, 2026-07-17, prompted by the unreleased-backlog
  gap.
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

- **claude-md-housekeeping-residency** [needs-spec] — CLAUDE.md §Housekeeping is
  72 of the file's 200 lines — 36% of the always-loaded surface every session
  pays for — and it is the only section carrying **mechanism inline** rather than
  one line plus a pointer. Its `docs/` bullet is the bulk: Jekyll layout, the nav
  Liquid contract and its front-matter keys (`nav_order`, `nav_parent`, `nav_id`,
  `nav_child_order`, `nav_children_key`), the mirror/rollup regen commands, and
  the four docs gates. Every line of it is load-triggered by principle — only a
  session touching `docs/` needs any of it — so it fails
  **Load-trigger residency**, while the same file's §Conventions and §Delivery
  doctrine already model the shape it should take.
  **Cost while deferred:** the standing per-session tax is paid by every session
  regardless of whether it opens `docs/`, and §Housekeeping is where new
  mechanism keeps landing (the +13-line always-loaded growth this iteration
  window), so the share grows without a forcing function.
  **Why it needs spec, not just an edit:** the block has nowhere to point. This
  same file declares `docs/` "repo-root-governed, no owning kit", so the
  de-residency move needs an owner decided first — a `docs/`-local architecture
  doc, a widened site-kit (which would overturn the no-owning-kit ruling on
  record), or a split where the gate roster stays and the chrome leaves. Naming
  that owner is the design question. Related but distinct from the
  `always-loaded` KPI, which measures the surface and does not judge residency.
  Surfaced 2026-07-17 by the config-seam-hardening close brevity pass.

## Done

## Lessons Learned
