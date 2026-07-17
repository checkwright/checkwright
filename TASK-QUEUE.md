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

- **mid-iteration-gap-channel-seam** [needs-spec] — should mid-iteration
  gap-filing be routed off `TASK-QUEUE.md`, with only `/scope` (open) and
  `/close` (dispose) writing the queue, and if so through what channel? Surfaced
  live in the release-in-lifecycle lead session: with a build stage-session
  holding the shared git index, every mid-iteration queue write races it, so the
  lead filed gaps via kfric as a workaround — which raised whether that should be
  doctrine, and whether it serves the multi-operator goal.
  **Two separable decisions — do not conflate:** (1) *Restrict queue writes to
  scope/close.* Precondition: the `[stage:]` cursor lives in the queue header and
  is flipped at **every** stage entry, so "only scope/close touch the queue" is
  unreachable until the fast-mutating cursor is **extracted out of the queue
  body** (into `.workflow/WORKFLOW-STATE.txt`, where the stamp already lives, or a
  dedicated cursor file). That slow-backlog vs fast-cursor split is what actually
  reduces multi-operator queue contention, and is worth doing independent of the
  channel choice. (2) *Choice of diversion channel.* kfric is the **wrong**
  channel for a multi-operator goal: `.metric/` is gitignored + account-bearing
  (the provenance seam), a **per-clone local buffer** that fragments the backlog
  across operators until close triages and pushes. Multi-operator wants a
  **committed** append-only channel — a per-session gap file close drains, or a
  conflict-free queue "inbox" region. Keep kfric as the **narrow
  knowledge-friction sensor** it is (`kfric N` means "facts with no home");
  overloading it as the general backlog inbox dilutes that signal — the lead did
  exactly this dilution under index pressure this session, evidence of the
  pressure, not that kfric is the right permanent home.
  **Multi-operator finding:** kfric-as-primary does **not** enforce or help
  multi-operator — it slightly harms it (local fragmentation). What enables
  concurrent operators is decision (1) plus a committed channel from (2); the two
  are independent, pick each on its own merits.
  **Recommendation:** (a) extract the stage cursor from the queue body; (b) if
  diverting mid-iteration filing, use a committed channel, not kfric; (c) preserve
  kfric's narrow friction-sensor role. Surfaces: lifecycle-kit/SPEC.md (state
  machine / queue-ownership — the `[stage:]` cursor location), drift-kit/SPEC.md
  §The knowledge-friction loop (kfric contract and scope), delegation-kit
  shared-file-roster (`TASK-QUEUE.md` contention). Surfaced 2026-07-17 in the
  release-in-lifecycle lead session under live shared-index pressure.

- **heterogeneous-agent-delegation** [needs-spec] — cross-vendor stage dispatch:
  a Claude Code lead delegating a stage (e.g. `/build`) to a foreign coding agent
  (Codex, etc.), extending the homogeneous multi-agent / multi-operator model to a
  heterogeneous fleet. Cashes the public "no IDE/model/harness lock-in"
  positioning claim, and is the purest expression of the thesis — governance
  enforced at the git/gate boundary, not by trusting the author. It splits along a
  **two-substrate line**. *Already agent-neutral:* the verification substrate (git
  + the gate battery + the bash flip/stamp state machine) does not care who
  authored the diff — a foreign agent's commit is gated identically; and the
  concurrent-agent coordination primitive is the shared git-index/HEAD
  serialization, vendor-neutral already. *Homogeneous today — the real work,
  worst-first:* (1) the **escalation resume model** is the gating sub-problem —
  the lead's whole value is resuming a paused stage in place instead of
  cold-restarting; a foreign agent cannot be `SendMessage`-resumed, so a foreign
  stage either runs fully autonomously (no mid-stage escalation) or escalates
  through a committed/polled channel with cold restarts, forfeiting the lead's
  cost asymmetry — the part that is not plumbing. (2) **dispatch transport** —
  today the harness `Agent`/`SendMessage`/task-notification; a foreign agent needs
  a transport-neutral handoff (committed worklist, issue, spawned process). (3)
  **budget oracle** — `usage-verdict.sh` is Anthropic-OAuth-specific; a
  heterogeneous fleet has N vendor-keyed oracles (the same seam as this session's
  credential-swap / token-usage tasks). (4) **stage-contract expression** —
  `/build` is a Claude Code skill (markdown + tool bindings); the lifecycle
  machinery is already neutral bash but the skill prose is not, so the contract
  needs an agent-agnostic form.
  **Seam ruling (on record):** generic mechanism only — the dispatch transport,
  budget oracle, and escalation channel become consumer-config seams (harness-native
  one adapter, foreign-agent another). A kit literal naming a vendor crosses the
  provenance seam and is ruled out, same pattern as `prose-profile`. **Economic
  why:** extends the existing per-batch model-tiering lever (the split-lead
  posture's whole point) across vendors — route each stage to whichever vendor's
  model wins. **Prerequisite cluster:** interacts with `hosted-attestation-service`
  (its neutral-party angle; multi-operator-semantics its named prerequisite),
  `plugin-marketplace` (the harness-absorption hedge), and this session's
  credential-swap budget-oracle tasks. **Demand-gated:** promote when a concrete
  cross-vendor dispatch need attests; until then this is the roadmap marker.
  Surfaced 2026-07-17 in the release-in-lifecycle lead session (operator question
  on external-agent delegation).

- **intra-stage-batch-supervision** [needs-spec] — who supervises a stage that
  splits into multiple batches, and whether the machinery permits it. Two
  coupled halves surfaced this session.
  **(1) The doctrine.** lifecycle-kit/templates/lead.md §Economics says "batch
  dispatches by shared surface" but never states who supervises *intra-stage*
  batching. A multi-batch stage (e.g. build's enforcement cluster plus a
  mechanical docs cluster) should be N sibling stage-sessions the **lead**
  dispatches and validates, not one stage-session that sub-dispatches — the
  latter nests a redundant second supervisor at the same tier, hidden from the
  lead's budget/context accounting. Proposed default: **lead-owns-batching**;
  reserve stage sub-dispatch for the read-only fan-outs a stage genuinely needs
  (the delegation nudge already sanctions those).
  **(2) The enabling precondition — verify oracle-first.** enter-stage.sh
  same-stage re-entry semantics are undocumented: when the lead dispatches a
  second /build session into an already-`[stage: build]` header, is the flip a
  stamp-refresh (which *licenses* lead multi-session stage dispatch) or a
  refusal? The SPEC must state which, and the answer must be verified against
  `check-stage-entry` / enter-stage.sh before the doctrine relies on it — a
  refusal would make lead-owns-batching unreachable without a machinery change.
  **Cost while deferred:** the batching posture stays unstated, so a stage under
  budget pressure improvises (this session's lead felt exactly this pull), and
  the re-entry behavior is trusted by assumption rather than by oracle.
  Surfaces: lifecycle-kit/templates/lead.md §Economics, lifecycle-kit/SPEC.md
  §The state machine (re-entry semantics), the delegation-kit stage-vs-lead
  dispatch boundary. Surfaced 2026-07-17 in the release-in-lifecycle session
  (kfric).

- **background-credential-swap-support** [needs-spec] — first-class support for
  swapping the Anthropic OAuth credential out from under in-flight agents (to
  spread burn across accounts), which the budget oracle does not model today.
  Four components, worst-first; all delegation-kit SPEC+code, all demand-gated
  (no one swaps in background yet — this is the roadmap marker).
  **(a) Detection.** usage-verdict's auth-change reroute fires only on
  CRED_FILE mtime, so an out-of-band / env-var / path token swap that does not
  rewrite that file bypasses it — the verdict trusts the prior account's
  snapshot and the poller re-fetches the stale file's token. Broaden the reroute
  to also fire when the live account identity (oauthAccount.accountUuid /
  subscriptionType) differs from the snapshot's `account=` / `tier=`, forcing a
  re-poll on any swap.
  **(b) Evidence.** the `.metric/` trend samples already carry `account=` /
  `tier=`, but the wave-over-wave burn projection reads the tail
  **unpartitioned**, so a swap reads as a spurious used% drop that corrupts the
  projection and masks aggregate load. Segment usage analysis by `account=` and
  mark the swap boundary in the trend log so the evidence is per-account-honest.
  **(c) Safety.** the budget guard's premise is one account = one rate window
  per wave; background rotation moves the wall in-flight agents bill against and
  lets rotation collectively exceed what any single account's 5h/7-day PAUSE
  would allow while each account stays individually under threshold. Add a
  cross-account aggregate view so supported swapping cannot silently blow past
  the true combined ceiling.
  **(d) Signal-quality refinement (advisory, not a bug).** the post-login
  reroute (`DELEGATION_KIT_LOGIN_WINDOW`) is correctly advisory-only — STALE
  never blocks (delegation-kit/SPEC.md §The staleness contract, lines 62-64 and
  207-208), so this is signal quality, not a dispatch-blocking defect. Two
  points: the window default is 600s while the SPEC's own stated server-lag is
  "about a minute" (SPEC line 245), a ~10x margin worth tightening; and it is a
  **blanket** time-window where an **account-keyed** check is sharper — trust
  `usage.txt` when its `account=` matches the current credential's account AND
  `updated_at > login_at`, with a short (~90s) settling floor for the server
  lag. That restores the true reading in ~1 min instead of 10 and stops 10 min
  of STALE samples polluting the trend log (`.metric/usage-history.log`) — which
  directly sharpens (b).
  **Cost while deferred:** any background swap today silently corrupts the burn
  projection and can breach the combined budget ceiling with every account
  reading individually safe; and the login window over-STALEs by ~10x.
  **Seam:** all four are generic delegation-kit mechanism — the account-id is
  already on the `usage.txt` contract; nothing consumer-specific is added. This
  is the budget-oracle prerequisite cluster heterogeneous-agent-delegation
  cross-references. Surfaced 2026-07-17 in the release-in-lifecycle session
  (kfric plus one operator-raised refinement).

## Done

## Lessons Learned
