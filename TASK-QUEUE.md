# TASK-QUEUE.md — Checkwright work queue

## Iteration: release-signaling-reset

  The lifecycle-kit gates read this header's iteration name and the stage
  cursor — the last stamp in `.workflow/WORKFLOW-STATE.txt`
  (lifecycle-kit/SPEC.md §The state machine); queue-kit formalizes the queue
  format itself and gates this file. One iteration per hardening or roadmap
  unit; [README.md](README.md) maps the kits.

---

## New Features

## Technical Debt

- **npm-publish-approval-posture-unrecorded** — the `npm-publish` environment's
  two weakening settings are written down nowhere, so the actual strength of the
  control `RELEASING.md` step 5 names is knowable only by querying the repo.
  **Settings VERIFIED 2026-08-01 at spec — every filed premise holds, and the
  design below now stands on established fact.** The entry demanded the first
  session to act on it run the environment read before anything else; it ran
  (`gh api repos/:owner/:repo/environments/npm-publish`) and returns: exactly one
  `required_reviewers` protection rule, carrying `prevent_self_review: false` and
  a single reviewer — the maintainer account, which is also the account that
  pushes release tags — beside `can_admins_bypass: true` and a null
  `deployment_branch_policy`. The environment's `updated_at` still equals its
  `created_at`, so nothing has been touched since it was made. This iteration
  falsified two inherited premises at their central claim; this one survived the
  same test, and the record now says so rather than leaving it unestablished.
  **This entry is the narrowed residue of a filed claim that is false, and the
  correction is the point.** Close filed the `v0.18.0` publish as having run "in
  12s with no approval requested" — the approval environment failing to gate an
  irreversible public publish. It gated. The `npm` job declares
  `environment: npm-publish` at job level; that run's deployment sat `waiting`
  for 77 seconds before `queued`; the run's approvals API records one `approved`
  entry whose user id matches the triggering actor; and the operator confirms
  they approved it. The "12s" was the job's post-approval runtime, not the wall
  clock from the tag push. `can_admins_bypass` did not fire — a bypass leaves no
  approval entry, and one exists. **Step 5's wording is accurate as written:**
  it claims a hold, never independent review, so the overclaim reading that
  motivated the filing does not survive either.
  **What survives verification, and only this.** (i) `can_admins_bypass: true`
  leaves a standing path on which the stated hold would not hold. It is
  available to the same single account either way, so its real cost is not an
  unauthorized publish but a publish landing with **no approval record** — and
  that record is the entire output of the control for a solo operator.
  (ii) Neither setting's posture is recorded, so a reader cannot tell deliberate
  from accidental, and a well-meant hardening pass setting
  `prevent_self_review: true` would **deadlock the channel**: the project is
  single-maintainer by standing ruling, so no second reviewer exists to approve
  a release.
  **Scope ruling 2026-08-01 — this is what unblocked promotion.** A solo-operator
  approval environment is a **confirmation step that produces an audit record**,
  never independent review: with one maintainer, independent review is
  unavailable by construction, and claiming it would be the overclaim this entry
  exists to avoid. Two consequences bound the unit. (1) The deliverable is to
  **record** the posture — `OPS.local.md`'s desired state, plus whatever
  `RELEASING.md` step 5 owes — so a later reader can tell deliberate from
  accidental. (2) `prevent_self_review: true` is ruled **out permanently**, not
  deferred: it deadlocks the channel against a single maintainer, and that is a
  property of the project's standing structure rather than of its current size.
  Whether to flip `can_admins_bypass` is **the operator's and is not this unit's
  to decide** — it is a live repo-settings change on a supply-chain surface; the
  ruling establishes only that its cost is a publish with no approval record,
  never an unauthorized publish.
  Debt: records deliberate repo-settings desired state; adds no governed name.
  Filed 2026-07-31 by close after the `v0.18.0` release, on a premise the
  operator contradicted at the source; re-verified against the run and narrowed
  to this residue at the next scope entry. Promoted 2026-08-01 by operator ruling
  at that scope's unit-set escalation.

## Deferred

- **rendered-site-link-monitor** [design-pending] — durable coverage for the
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
  **Cost while deferred:** low and non-rotting — the tree-side reference gates
  and the deployment probe still hold render and deployment truth; the residue
  is that external link rot on the rendered site is found by a reader rather
  than by a scheduled crawl.
- **plugin-marketplace** [design-pending] [roadmap: later/ecosystem] — harness plugin packaging.
  roadmap-summary: The stage skills and guards installable as a harness plugin.
  Harness plugin/marketplace packaging
  of the stage skills and guards; anti-drift gate shape: manifest ↔ shipped
  surface parity. Design against the live manifest format at promotion — the
  plugin substrate moves fast (the scope-session-routing ruling applies).
  **The install-ownership contract this must package against already exists:**
  `checkwright.lock`, written by the installer's `init` and specified at
  installer/README.md §The manifest — its schema owner is
  `installer/lib/common/lock.sh`. A marketplace package that installs kits
  without writing that manifest would be a second install model with no upgrade
  or uninstall story, which is the sequencing risk this entry has always
  flagged; the named contract replaces the re-derivation it used to imply.
  The upgrade/uninstall story itself is `installer-lifecycle-verbs` below —
  sequence against it rather than duplicating it.
  **Negative result — the tarball channel's economics do not transfer here.**
  `release-tarball-delivery-channel` is cheap for a structural reason that is
  absent from this rung: `.github/workflows/publish.yml`'s `pack` job already
  assembles and stamps one tarball and uploads it as the run's artifact, so a
  new channel is a sibling job that `needs: pack` and consumes that artifact.
  A marketplace package cannot consume it. Its unit of delivery is the
  harness's own plugin manifest format, not a packed npm assembly, and its
  subject is the stage skills and guards rather than the eleven-kit tree — so
  it shares neither the assembly nor the artifact. Recorded because the reflex
  at promotion will be to cost this by analogy from the tarball's sibling-job
  cheapness and arrive at the wrong number.
  **Open question a promoting scope answers first — deliberately undecided
  here.** Whether the marketplace package vendors kits at all, or merely
  registers the skills and delegates all vendoring to the installer's `init`.
  Under the second answer it stops being a distribution channel and becomes a
  **discovery surface**, and `checkwright.lock` ceases to be a contract it must
  *honour* and becomes one it must not *violate* — the materially cheaper
  answer, and the one that dissolves most of the sequencing risk above. It is
  not settled here because it is downstream of this entry's standing ruling
  that the plugin substrate moves fast and the design must be made against the
  live manifest format at promotion; deciding it now would be deciding it
  against a format that will have moved. Recorded 2026-07-26 by close
  (`activation-path`).
  **Cost while deferred:** zero mechanism rots — the install-ownership contract
  this must package against is already written and maintained by the installer's
  `init`; what is foregone is a discovery surface, and the plugin substrate's
  motion means a design taken early would be retaken at promotion anyway.
  Surfaced 2026-07-09 in adoption-track's split; evidence artifact retained:
  upstream Claude Code issue #75214 (project config can't lift the Task
  ask-first default), surfaced dogfooding the delegation nudge 2026-07-07.
- **benchmark-ab-experiment** [design-pending] [roadmap: later/adoption] — a controlled A/B trial.
  roadmap-summary: A controlled experiment measuring drift with and without governance.
  **Cost while deferred:** zero — the self-referential drift-trajectory route
  already carries the claim this rung would upgrade, and the measurement half it
  consumes ships independently; what is foregone is an externally-comparable
  number the project does not currently claim.
  The controlled differential
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
  The experiment's measurement half — per-stage, per-model, price-weighted
  token burn off harness transcripts — is the stage-economics-report tool
  filed above; this rung consumes it rather than rebuilding it. Nearer use of
  that tool: verifying the split-lead posture's savings
  (lifecycle-kit/templates/lead.md §Economics). Surfaced 2026-07-15 by the
  per-stage budget analysis that motivated that posture.
- **prose-profile** [design-pending] [roadmap: later/ecosystem] — a profile for non-code repos.
  roadmap-summary: A profile for documentation repos, where there is no build to gate.
  The non-code universality rung: a third
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
  **Cost while deferred:** zero — the kits make no non-code claim to falsify,
  core dilution is ruled out on record, and the named seeds stay accurate until
  a non-code consumer vendors a kit and reports friction.
- **hosted-attestation-service** [design-pending] [roadmap: later/commercial] — hosted attestation.
  roadmap-summary: Gate runs verified by a neutral party no committing agent can touch.
  The team/paid rung: gates
  verified server-side by a party the committing agents cannot touch —
  hosted gate runs as a neutral attestation, cross-repo drift dashboards,
  maintained rulesets. A service, not code: cloning the kits does not clone
  the neutrality or the ops. Demand-gated — this entry is the public
  roadmap marker, not a scaffold; hosting and sequencing decisions are on
  record in the operator's local brief, and multi-operator-semantics
  is its prerequisite mechanism. Surfaced 2026-07-07.
  **Cost while deferred:** zero — this is a service rather than tree mechanism,
  so nothing rots; the residue is that gate runs stay self-attested, which binds
  only when a party the committing agents cannot touch is asked to trust them.
- **spec-internal-identifier-prefix-drift** [design-pending] — SPEC prose naming a
  script's **internal** variable spelling where the public knob is the contract
  name. Found by the config-seam-hardening close audit of the
  `internal-identifier-restatement` roster class, and fixed there: seven sites in
  delegation-kit/SPEC.md named locals of `bin/usage-verdict.sh` that exist only as
  assignments from their `DELEGATION_KIT_`-prefixed env knobs, while the same
  doc's §Layout roster had the prefixed spelling right — prose-vs-roster drift
  *within one file*.
  **Cost while deferred:** the fix is a rename away from rotting — renaming the
  local in the script silently falsifies the prose, and only the roster class's
  audit cadence catches it, at iteration granularity.
  **The manual fix is demonstrably incomplete — an eighth site surfaced
  2026-07-19** and was fixed at that close. That is the demand evidence the entry
  was waiting on: a hand sweep of one file missed one instance in eight, so the
  audit cadence catches what review does not, and a ~20-line scan expressed the
  whole class.
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
  **Premise updated 2026-07-20 by the `verdict-reader-honesty` close audit — two
  corrections, both of which the next scope would otherwise re-derive.**
  (1) *The class is currently clean.* The eighth site was fixed at this
  iteration's scope, and a fresh mechanical sweep of every kit SPEC found zero
  genuine hits. A gate built for this class today would land **greenfield** — it
  proves the boundary rather than fixing a backlog, so the demand argument now
  rests entirely on recurrence, not on outstanding drift.
  (2) *The false-positive bound above is looser than stated.* The sweep produced
  one hit — `GRAPH_VOCAB` at gate-sdk/SPEC.md:883 — which is **correct prose**:
  it names the array a consumer declares inside `<gates-dir>/graph-vocab.sh`,
  a public contract in its own right, while `GATE_SDK_GRAPH_VOCAB` is the
  *separate* knob naming that file's path. So a `<KIT>_`-prefixed counterpart
  can exist and denote a **different contract**, which the "prefixed counterpart
  bounds the false positives" claim does not anticipate. The gate must therefore
  also establish that the bare token and the prefixed one denote the *same*
  contract — the discriminator the design pass actually owes.

- **heterogeneous-agent-delegation** [design-pending] [roadmap: later/ecosystem] — foreign agents.
  roadmap-summary: Dispatch a stage to any vendor's coding agent, gated identically.
  Cross-vendor stage dispatch: a lead delegating a stage to a foreign coding
  agent, extending the homogeneous multi-agent model to a heterogeneous fleet.
  It cashes the public no-lock-in claim and is the purest expression of the
  thesis — governance enforced at the git/gate boundary, not by trusting the
  author. *Already agent-neutral:* the verification substrate (git, the gate
  battery, the bash stamp state machine) does not care who authored the diff,
  and the concurrent-agent coordination primitive is the shared git-index/HEAD
  serialization. *Homogeneous today — the real work, worst-first:*
  (1) the **escalation resume model** collapses into (2) as a property of the
  chosen transport, per the 2026-07-25 amendment below; (2) **dispatch
  transport** — today the harness `Agent`/`SendMessage`/task-notification; a
  foreign agent needs a transport-neutral handoff. The adapter contract is
  "open / prompt / permission-request / resume" spoken over each vendor's
  structured **machine plane, never its TUI**: a screen-scrape relay is
  adapter-of-last-resort for a vendor shipping no machine interface at all —
  it yields rendered frames instead of turn events, answers dialogs by
  heuristic, and bets on the vendor's least-stable surface. (3) **budget
  oracle** — the verdict tool is Anthropic-OAuth-specific; a heterogeneous fleet
  has N vendor-keyed oracles, the same seam as the credential-swap entries, and
  the vendors' JSONL event streams carry the token-usage events a TUI path would
  scrape from a status bar. (4) **stage-contract expression** — the lifecycle
  machinery is neutral bash but the stage-skill prose is not.
  **Seam ruling (on record):** generic mechanism only — transport, budget oracle,
  and escalation channel become consumer-config seams; a kit literal naming a
  vendor crosses the provenance seam and is ruled out, the `prose-profile`
  pattern. It extends the per-batch model-tiering lever across vendors, and
  interacts with `hosted-attestation-service`, `plugin-marketplace`, and the
  credential-swap budget-oracle entries.
  **Demand-gated — demand attested (2026-07-23):** the operator holds working
  foreign-vendor subscriptions and wants read-heavy delegation routed to them for
  budget headroom, and with three vendors live the N-keyed oracle seam is no
  longer hypothetical. First slice at promotion: a foreign-CLI executor for the
  already-pre-authorized read-heavy audit / mechanical-sweep class over a spawned
  non-interactive CLI process, one adapter per vendor as consumer config — not
  full stage dispatch. Promotion-eligible at the next scope session.
  **Design-memory amendment (2026-07-25, verified against the installed CLIs):**
  the TUI-relay alternative was probed for session resume and token efficiency.
  Ruling: those benefits live in the vendor's session store, not the TUI — the
  APIs are stateless and both modes replay the same on-disk transcript against
  the same server-side prompt cache, so interactive-vs-headless is a rendering
  choice, not a state choice. Headless warm-resume by session id and JSONL turn
  events ship today on the vendors probed, which is what makes (1) plumbing.
  **Cost while deferred:** the foregone lever is live — read-heavy audits and
  mechanical sweeps all bill against one vendor's budget while three
  subscriptions are held — and this design memory ages against fast-moving CLIs.
  Surfaced 2026-07-17 in the release-in-lifecycle lead session (operator question
  on external-agent delegation).

- **background-credential-swap-support** [design-pending] — first-class support for
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

- **enforcement-first-behavioral-regressions** [design-pending] — the always-loaded
  enforcement-first rule ("the fix and the gate that catches it land in one unit;
  removing the duplication outranks gating it") anchors its second clause — and
  every neighbouring doctrine example (content-tiering, de-literalization) — in
  the SSOT/duplication domain, so it under-cues the incident→gate reflex for a
  *behavioral* regression that has nothing to do with duplicated content. Design
  shape: tighten the always-loaded line (or the doctrine-kit section behind it) to
  name the generalize-to-class-then-gate reflex for runtime/behavioral defects,
  not only duplication — a `doctrine-kit/DOCTRINE.md` change, re-vendored to
  upgrade, so it sits outside an incident-fix commit's envelope.
  **Cost while deferred:** each behavioral bug fixed in a maintenance turn risks
  shipping without its paired low-FP gate until a reviewer prompts.
  Surfaced 2026-07-19 by
  the check-graph `maxEdges` fix (the coupling graph outgrew Mermaid's 500-edge
  render cap): the one-line render fix landed, but the paired render-cap gate —
  exactly the low-FP gate enforcement-first says to land in the same unit — was
  added only on explicit request.

- **spec-split-promotion-review** [design-pending] — after the six-stage roster has
  run **≥N iterations with `spec`-stage economics actually recorded on the
  trajectory**, re-run the `/economics` read; if the split shows the projected
  cache/context win, **promote the `spec` stage to the kit default** via a new
  amendment (else keep it consumer-config or revert). The `stage-posture-split`
  amendment shipped the stage as optional, demand-gated consumer config
  precisely so this promotion is an evidence-gated follow-up rather than a
  default flip on projection alone.
  **Precondition — hard dependency on `trajectory-stage-roster-hardcode`:** until
  that lands, `drift-kit/bin/trajectory.sh` silently drops every `spec` stamp
  (hardcoded 5-stage roster), so **no `spec`-stage economics data is recorded on
  the trajectory at all** — the re-run this task prescribes has nothing to read.
  This task cannot start before that fix.
  **Premise corrected 2026-07-20 by the undirected scope survey — the named
  dependency has landed and this entry is still blocked, by a different one.**
  `trajectory-stage-roster-hardcode` reached Done, and the trajectory table now
  renders `sp` for three iterations — so this entry reads as unblocked and
  pickable. It is not. The trajectory and the *economics* are two surfaces, and
  only the first recovered: `.metric/stage-economics-log.txt` holds **zero `spec`
  rows**, because `bin/stage-economics.sh` reads the boundary-truncated live state
  file rather than committed history. The precondition this entry actually needs
  is *recorded spec-stage economics*, and that is blocked on
  **`stage-economics-truncation-durability`** — the real blocker, named here so no
  future scope re-derives it or promotes a review with nothing to read.
  **Premise corrected again 2026-07-21 by the scope survey — that blocker has
  dissolved and this entry is now genuinely pickable.** The "zero `spec` rows"
  reading above was taken before the meter's history union had been exercised;
  `.metric/stage-economics-log.txt` in fact holds five `spec` rows covering five
  distinct iterations, all recovered *after* their boundary truncations. So the
  data this entry prescribes reading **exists**, and the named blocker is being
  retired inside `stage-economics-honesty` rather than repaired. Two carry-overs
  before the re-run is worth anything: (1) wait for that iteration to land, since
  the figures this review reads are exactly the ones
  `stage-economics-attribution-honesty` is correcting — re-running against
  mis-attributed rows would settle a tier question on noise; (2) the `≥N
  iterations` threshold this entry leaves unset still needs a value, and five
  recorded iterations is the number now on the table.
  **Cost while deferred:** one queue line; the backlog-aging review re-raises it
  every iteration until the data exists to run it. Filed 2026-07-19 by lead
  ruling at the `stage-posture-split-tuning` close — the split shipped on
  projected economics, and this is the loop that confirms or retires that
  projection with recorded data.

- **build-stage-tier-economics** [design-pending] — measure whether the `build`
  stage downgrades from Opus to Sonnet net-positive rather than flipping on
  intuition; a ruling-config tier re-judgment (`.claude/agents/stage-session.md`
  / the lead template's ruling-config, which invites re-judging every tier).
  Grounding, **corrected** against priced rows (the earlier token-only reading
  below predates both the price table and the split-lead posture, and overstated
  build's lead): under the current split-lead posture the priced spread is build
  $2.59–10.96, close $5.83–7.81, validate-on-Sonnet $0.54–1.44. Two readings
  follow, and neither matches the original grounding. **Close is comparable to
  build, not an order of magnitude below it** — so close is a tier candidate in
  its own right, arguably ahead of build, and the premise that build is the
  single highest-value lever no longer holds. And the already-adopted
  validate→Sonnet downgrade **demonstrably works**: validate is the cheapest
  stage by a wide margin with no observed quality cost, which is the affirmative
  precedent this A/B is testing for build. Treat every figure here as provisional
  until the prerequisite below lands — they carry the same attribution defects.
  The superseded token-only reading: build ~100–175k output, 5–25M cache-read per
  run versus close/validate ~7–49k output.
  **Two design blockers:** (1) ~~the decision metric is uninstrumented~~ —
  **resolved**: a price table now exists and the meter prices instead of
  reporting `cost=n/a`. It is replaced by a sharper blocker: the figures are
  priced but **mis-attributed**, so `stage-economics-attribution-honesty` is
  now this task's hard prerequisite. Running the A/B on today's rows would
  compare two noisy numbers — one session bearing two stamps is billed to both
  stages in full, unstamped continuation sessions vanish, and the lead's own
  supervision burn lands in no row at all. Sequence that task first. (2) the
  metric must be **net delivered-work cost** — price-weighted tokens + rework
  round-trips + the supervisor's by-eye gate-diff burden + escalation load
  shifted onto the Opus lead — not single-pass token price; a cheaper builder
  that fails the battery or emits a subtly-wrong gate the supervisor must catch
  can invert the saving.
  **Design direction:** a deliberate A/B on representative *low-judgment* builds
  (convergence on an already-authored contract), holding the unit class
  comparable; high-judgment builds that write gate assertions or touch the
  provenance seam likely stay Opus regardless, so the honest outcome may be a
  per-build-class tier rule, not a blanket flip. Sibling to
  `spec-split-promotion-review` (evidence-gated tier promotion) and
  `benchmark-ab-experiment` (which holds model constant and varies governance —
  this holds stage constant and varies model). Debt/analysis: settles a
  ruling-config tier by data, adds no governed name.
  **Cost while deferred:** low and non-rotting — validate's adopted downgrade
  already banks the affordable half of this lever, and every figure here is
  provisional until the prerequisite corrects the rows; the residue is that
  build's and close's tiers stay set by intuition rather than by a priced A/B.
  Filed 2026-07-20 by lead
  ruling during the `render-fidelity-leak-coverage` spec, from an operator
  question.

- **supervision-overhead-unmeasured** [design-pending] — supervision is roughly a
  fifth of an iteration's priced burn and has never been examined. It went
  unexamined because it was not a distinct row at all until the attribution fix
  landed; the figures live in `.metric/stage-economics-log.txt` and are read from
  there rather than restated here. The obvious experiment — run a lead session on
  Sonnet against the Opus baseline — has been **declined twice at scope**
  (2026-07-22, both times on the same grounds), and the premise has since
  sharpened enough to re-rank the entry's own open work.
  **Both legs are blocked, which this entry used to obscure.** Re-counted at that
  scope: 24 `supervision` rows, the priced Opus ones spanning a **6x** range. Read
  against that spread a single Sonnet lead session is not merely underpowered, it
  is uninterpretable in principle — any plausible tier effect sits well inside the
  existing variance — so the experiment as filed returns an unusable number *even
  if the quality read existed*. The cost leg therefore needs either many repeated
  runs (n≫1, at one lead session per iteration) or a variance-controlled
  comparison normalizing for iteration size, the obvious candidate being **cost
  per unit delivered** rather than cost per iteration. **Design that
  normalization before the quality read:** an uninterpretable cost axis makes the
  quality axis moot, and the reverse does not hold. A further constraint on any
  comparison: supervision is the only row still growing while close runs, so rows
  must be read at the *same* lifecycle point, never at two convenience snapshots.
  **The risk is a different class from validate's.** Supervision is where
  **rulings** happen, so the failure mode is not a bigger bill but a **bad ruling
  that costs a rebuild** — which the cost row would score as a *saving*. That is
  why a quality read is owed at all, and the design question this entry carries is
  what that read is: a rebuild count, an escalation-correctness sample, or an
  honest ruling that the axis is unmeasurable at n=1. It is also why the two
  declinations were correct rather than missed windows: the cost side has many
  priced Opus rows and the quality side has no read at all.
  **What cannot be harvested by delegation.** Supervision splits internally the
  way `build` does — mechanical routing/verification versus genuine rulings — so
  the batch-tiering answer looks transferable. It is not: the verification half is
  **not delegable away from the supervising session**, because the supervisor
  re-running the battery and diffing every agent commit *is* the protocol
  (`delegation-kit/templates/agent-execution.md`). A supervision split can be
  tiered but not delegated, which narrows the levers to the tier question.
  Debt/analysis: measures an unexamined cost line and may re-tier a lead binding;
  adds no governed name.
  **Cost while deferred:** the largest unexamined line in the iteration budget,
  paid every iteration, with no evidence either way about whether it is bought or
  wasted. Bounded and non-rotting — nothing breaks, and the row now accumulates
  per-iteration baselines whether or not the experiment runs.
  Filed 2026-07-22 by close, from the same lead-side economics review.

- **preview-release-cadence** [design-pending] [roadmap: next/adoption] — a preview channel.
  roadmap-summary: A declared preview channel and a slower, calmer release cadence.
  Reset release signaling for a
  pre-1.0 audience: declare a preview/alpha channel, batch internal
  iterations into a slower external cadence (weekly-class) so consumers stop
  reading thirteen-releases-in-nine-days as churn, publish
  checksum-verifiable release assets instead of bare tag pointers, and keep
  a 30-second human changelog beside the migration detail. Separates
  internal iteration completion from public version publication — a
  RELEASING.md policy change more than a mechanism.
  **Premise corrected 2026-07-25 by the undirected scope survey, re-counted
  2026-07-26 and again 2026-07-27 — the count is larger than filed and the
  rhythm held.** This entry says "thirteen-releases-in-nine-days"; the tree now
  carries **17 tags in 14 days**, `v0.1.0` (2026-07-14) through `v0.17.0`
  (2026-07-27). The finding is not a one-off burst that has since settled — the
  cadence continued at roughly a release a day across the whole window, and the
  count has risen at every re-read (13 → 14 → 15 → 17), which strengthens rather
  than dates the signaling argument.
  **The inherited checksum-verifiable-asset deliverable is SATISFIED — corrected
  2026-07-27 by scope.** This entry absorbed `supply-chain-trust-baseline`'s
  carved-out "publish checksum-verifiable release assets instead of bare tag
  pointers" (2026-07-25, operator-approved) on the ground that the convention
  should land with the cadence it depends on. `release-tarball-delivery-channel`
  shipped it independently: `v0.17.0` is the first release carrying assets —
  `checkwright-0.17.0.tgz` beside its `.sha256` — where `v0.16.0` carried none.
  The deliverable is struck; what this entry still owns is the **signaling** half
  alone (a declared preview channel, a slower external cadence batching internal
  iterations, and a 30-second human changelog beside the migration detail). The
  residual integrity question — that a checksum sharing one origin with its
  tarball proves transfer integrity only — is `tarball-build-attestation`'s, not
  this entry's.
  **Cost while deferred:** zero pre-announcement; at announcement the
  release history itself signals instability to exactly the risk-averse
  teams the trust story targets. The inherited checksummed-asset gap no longer
  contributes — it is closed, per the correction above.
  Surfaced 2026-07-23 in the same external review (its release-signaling
  finding).

- **validate-verb-collision-and-check-routing** [design-pending] — two coupled
  defects with one root: the delegation discipline verb collides with the
  `/validate` stage noun, and that collision misroutes the lead's
  post-delegation check onto the evidence producer.
  **(1) Verb collision.** The lifecycle stage `/validate` shares its term with the
  delegation discipline "validate after every agent commit". The verb is performed
  right *before* the stage, so completing the lead-side verify reads as completing
  the stage — a lead conflated them and nearly skipped `/validate`, jumping build
  to close. **Cheapest true fix:** rename the delegation discipline **verb**
  (validate → verify or re-check) in `delegation-kit/templates/agent-execution.md`
  and the lead binding, leaving the load-bearing stage name untouched. Renaming
  the stage is rejected as invasive for a confusion the verb rename resolves.
  **(2) Check-routing gap the collision causes.** The post-delegation-check
  binding carves out no case for when the *delegated stage is `/validate`
  itself*. There the naive "re-run the battery to check it" is **wrong**:
  `/validate` is the evidence producer, so re-running `run-validate` mutates or
  duplicates the committed execution evidence rather than verifying it. The
  correct lead-side check is to **read** its committed evidence manifest, never to
  re-run. The binding must distinguish **work-producing stages** (re-run the
  battery, safe and idempotent) from the **evidence-producing stage** (read the
  committed manifest). The two defects travel together because the verb rename is
  what stops the check discipline mis-attaching to the evidence producer.
  **Fired in two consecutive iterations, operator-caught both times** — once
  moving to re-run `run-validate` to "verify" the validate stage, once running the
  full `run-gates` battery after the validate stage's commit. **The two re-runs
  are not equally harmful, and part (2)'s wording must split them:** `run-gates`
  writes nothing under `.workflow`, so re-running it is inert on the evidence and
  merely wasted work, while `run-validate` is the sole writer. The routing defect
  is real for both, the corruption risk only for the second.
  **Disposition after recurrence — unchanged, cost raised, insufficiency named.**
  The verb rename plus the work-vs-evidence carve-out is still the cheapest true
  fix. What recurrence establishes is that the prose fix is **not sufficient on
  its own**: it removes the ambiguity but installs no oracle. The check class that
  would catch this is a gate over a lead's *tool invocations*, and no scanner is
  buildable — a lead's choice of command leaves no tracked artifact to read. The
  nearest buildable proxy is `validate-producer-liveness-unobservable`, which
  covers the concurrent case, not a sequential re-run. So the unit ships the prose
  fix knowing detection stays human, and that limit belongs in the binding.
  **Why `[design-pending]`:** a prose rename plus a binding-semantics change
  across `delegation-kit`'s dispatch template and the lead binding, with a
  grep-propagation pass, and the work-vs-evidence carve-out is a SPEC ruling
  rather than a wording tweak.
  **Cost while deferred:** raised from "low but recurrent" — it has fired in two
  consecutive iterations, detected only by an operator catch both times, so the
  in-band cost is unbounded until someone happens to look.
  Filed 2026-07-25 by close, draining the committed gap inbox (`0eec298`) merged
  with the lead's post-dispatch third triage item; second instance added the same
  day by close.

- **core-files-kit-coverage-derived** [design-pending] — `scripts/core-files.list`
  carries a block headed "One SPEC.md per kit (each kit's canonical contract)"
  that lists **9 of the 11** kit SPECs: `site-kit/SPEC.md` and
  `doctrine-kit/SPEC.md` are absent, as is `doctrine-kit/DOCTRINE.md` (which
  `CANON_KIT_MANIFEST_FILES` does carry). The block is a hand-maintained roster
  of a **derivable** set — `gate_kit_roots` × `SPEC.md` — which is the
  maintain-a-derivable antipattern derivation-first rules out; and
  `check-core-files` asserts only that *listed* paths exist, never that the
  derivable set is covered, so the omission is invisible to a green battery.
  **Deliverable — the derived coverage assertion, not the three-line backfill.**
  Backfilling the two missing names re-arms the identical drift for kit twelve:
  that is the defect, not the fix. Two in-kit precedents give the shape, both
  bidirectional parity over a derived set — `check-readme-roster` (each kit
  README's roster block ↔ that kit's `checks/` basenames) and `check-kit-enum`
  (a `couples=` set naming two or more kit roots must name every root carrying
  the suffix). This entry is the same move over the core-file manifest: every
  `gate_kit_roots` member's canonical SPEC is pinned by derivation, not by a
  line someone remembered to add.
  **Why `[design-pending]`:** the derived set's boundary is the open design, not the
  assertion. Kit SPECs are uniform, but `doctrine-kit/DOCTRINE.md` is a per-kit
  *deliverable* with no counterpart in the other ten, and the same manifest pins
  surfaces no derivation reaches (workflow instances, generated projections, the
  validate baseline). The rule must therefore state which slice it owns and
  leave the remainder an honest hand list, or it over-reaches into a roster that
  is legitimately hand-held. Placement is a second call: a new assertion inside
  `check-core-files` versus a sibling gate.
  **Cost while deferred — the derivation-first defect is confirmed, the safety
  hole is not.** Confirmed: the block's stated per-kit invariant is false today
  and degrades silently as kits land. Unconfirmed: whether an unpinned kit SPEC
  is genuinely deletable without a red anywhere. `scripts/gen-docs-mirror.sh
  --list` derives all eleven kits' SPEC and README plus DOCTRINE.md, so a
  deleted kit SPEC would plausibly surface through `check-docs-mirror-fresh` or
  `check-docs-kit-parity` — not destructively verified here, so whoever picks
  this up should establish the real exposure before costing the gate against it.
  Debt: converges an existing manifest onto a derived roster; adds no governed
  name unless the assertion lands.
  Filed 2026-07-25 by align, from the `supply-chain-trust-baseline` cross-spec
  audit — the amendment's A3 edits this same file, so the gap is adjacent to its
  envelope without being inside it. Lead ruling, scope-gated intake.

- **security-advisory-lane** [design-pending] — `SECURITY.md` (shipped this
  iteration) directs reporters to GitHub private vulnerability reporting, and
  nothing on our side is named as the reader. Advisories are a surface distinct
  from issues and PRs, returned by neither `gh issue list` nor `gh pr list`.
  The scope skill's GitHub boundary sweep (.claude/commands/scope.md §The GitHub
  boundary sweep) has exactly two lanes, Issues and PRs, while asserting nothing
  lives triaged-but-unqueued anywhere else — so the assertion is now false for
  the one lane whose items are the most time-critical.
  SPEC-supply-chain-trust-baseline.md §causal chains named the route's producer
  and called the external reporter its consumer, but the reporter is the sender.
  **Honest limit:** advisories are not unread. GitHub notifies maintainers, so
  this is an unswept lane rather than a black hole, which is what kept it out of
  the in-flight unit.
  **Why `[design-pending]` — the lane owes three decisions, and copying the Issues
  lane settles none of them.**
  *Disposition grammar under a public queue:* advisories are private and
  TASK-QUEUE.md is public, so a promoted entry citing an unfixed vulnerability
  publishes it. The lane needs its own dispositions (fix under embargo,
  advisory-only, decline with cause) and a rule for what a public entry may say
  before the advisory is published.
  *Response latency:* scope runs at iteration boundaries, so a sweep-lane duty
  makes acknowledgement latency equal to the time until the next scope entry.
  That is systematic disposition, not speed.
  *Whether scope-gated intake gains an exception:* what delivers a fast critical
  response is an interrupt, a rule that a critical advisory preempts the running
  iteration. CLAUDE.md §Delivery doctrine has no severity carve-out, and a
  security interrupt is the one case that plausibly earns one.
  **Cost while deferred:** until the interrupt path exists, `SECURITY.md`'s
  current windows must not be tightened. Promising a response the machinery
  cannot deliver is the overclaim shape this iteration existed to remove, one
  level up from the gate overclaim it did remove.
  Debt: small-to-medium; the constraint is the design, not the code.
  Filed 2026-07-25 by close, draining the `supply-chain-trust-baseline` gap
  inbox (two bullets merged at triage, costed once).

- **local-overlay-git-blanket-grant** [design-pending] — the local permission overlay
  carries `Bash(git *)`, a single glob granting **every** git subcommand without a
  prompt. The destructive ones ride it: `git reset --hard`, `git clean -fd`, `git
  push --force`, `git rm -r`. `bash-guard`'s project rules cover only two narrow
  cases (`git commit --no-verify`, `git clean -x/-X`), so everything else in that
  set is auto-allowed by the blanket rather than by a judgment about it.
  **Why it survived this long.** `compare-settings-allow.sh` reports it
  non-redundant and therefore never flags it — correctly, since the committed set
  holds no glob that covers it. The tool answers "is this entry redundant?", and
  the question this entry needs is "is this entry *too broad?*", which nothing
  asks. That is the gap, not the entry itself.
  **Deliverable:** replace the blanket with the write-side verbs the workflow
  actually uses (`git checkout <path>`, `git push origin master`, `git tag -a`,
  `git add`, `git commit`, `git stash`), leaving destructive git to prompt. The
  read-only verbs no longer need it — `git status|log|show|diff|rev-parse|
  ls-remote|tag -l|check-ignore` were promoted to the committed allowlist this
  close, which is what makes the narrowing affordable.
  **Why `[design-pending]`:** the general question is whether guard-kit should ship a
  *breadth* criterion beside its redundancy one — a check that reds a local glob
  whose match set includes a known-destructive command — and if so, where the
  destructive set is owned without becoming a maintained roster. That is a
  guard-kit/SPEC.md ruling; narrowing this one overlay is the instance, not the
  fix.
  **Cost while deferred:** an unprompted destructive git command is a
  low-probability, high-cost event, and the overlay is local-only so the exposure
  is one machine rather than every consumer. Debt: narrows a local grant and adds
  one gate class; adds no governed name to a shipped surface.
  **Why close did not just narrow it here:** the triage step's mandate is
  promote-and-prune, and re-shaping a grant model mid-release — with a tag and a
  push still to run in this same session — is bad sequencing regardless of
  authority. The promotion and the dead-entry prune landed; this is the costed
  remainder.
  Filed 2026-07-25 by close, from the tooling-friction triage.

- **validate-producer-liveness-unobservable** [design-pending] — a stage session can
  report its stage done while its own oracle is still executing, and nothing in
  the lifecycle can see it. This iteration the validate session's `run-validate`
  was still running in the background when the lead dispatched close; its writes
  landed on `.workflow/validate-evidence.txt` underneath the close session's
  commits (the churn recorded under `evidence-row-upsert-order`).
  **What did not catch it, and why.** The lead's post-dispatch check is `git
  status` / `git log`, both blind to a live process. The close-entry preflight
  (`LIFECYCLE_KIT_ENTRY_PREFLIGHT`, `scripts/lifecycle-config.sh`) is blind for a
  different reason: it reads a file at an instant, and the file can change a
  second later. Neither is a coverage hole; both are liveness holes.
  **What the preflight already asserts — recorded so this entry is not filed on
  a false premise, as its sibling above was.** `check-evidence-manifest` is not
  grammar-only. Assertion A, armed at a `close` cursor, walks
  `EVIDENCE_KIT_SUITES` and errors on any configured suite with no clean line for
  the iteration and on any clean line predating the earliest validate stamp — so
  suite-roster completeness *is* asserted, and already derived from the
  configured roster rather than a maintained list. A truncated manifest reds at
  the close entry today. Nor is a torn read reachable: `run-validate` builds each
  revision in a temp file and `mv`s it over the manifest, so a row is relocated,
  never briefly absent. The gap is liveness alone.
  **Deliverable:** a liveness sentinel — `run-validate` claims a lock under
  `EVIDENCE_KIT_TMP_DIR` for the duration of a run and releases it on exit; the
  close-entry preflight reds on a held lock, so a stage entry cannot be stamped
  while the prior stage's producer is still writing.
  **Why `[design-pending]`:** it adds a runtime artifact needing a named reclaim path
  (the runtime-artifact lifecycle rule), a stale-lock policy for a crashed run,
  and a knob; and evidence-kit/SPEC.md owns the writer contract while
  lifecycle-kit owns the preflight, so which kit holds the lock is a
  cross-kit ruling, not an implementation choice.
  **Cost while deferred:** silent at the moment it happens and paid entirely by
  the next session, which fights a file changing underneath it and cannot
  distinguish churn from evidence. Rare — it needs a backgrounded producer — but
  it cost this iteration three restores once dispatch overlapped, and the only
  detector was the operator. Debt: one runtime artifact plus one preflight
  assertion; adds no governed name to a shipped surface.
  Filed 2026-07-25 by close, from the operator's observation of the live
  `run-validate` and the row-relocation fingerprint at `ae70eae`.

- **lead-dispatch-requires-completion-notification** [design-pending] — the lead has
  no stated precondition for dispatching stage N+1, and the one it improvised is
  wrong: artifact state. A lead checked validate's commit had landed with complete
  evidence, the tree clean, the battery green, and a simulated close entry
  cleared — then dispatched close into a still-running `run-validate`. Every check
  passed mid-write because `run-validate` commits its evidence and keeps going:
  **the terminal commit existing is fully compatible with the process still
  executing.** The lead dispatched on the *absence* of a completion notification.
  **The generalizable trap, worth the entry on its own.** The lead read an
  operator's note about having just answered a stalled permission prompt as the
  stage being finished. It means the opposite — an approval prompt gates a command
  **starting**, so any prompt-answered signal is a start signal, never a
  completion one.
  **Deliverable:** a dispatch precondition in `lifecycle-kit/templates/lead.md` —
  stage N+1 is dispatched on stage N's agent **completion notification**, never on
  its commit or any tree-state check, because artifact state cannot distinguish
  "finished" from "still writing." It belongs beside the post-delegation verify
  discipline, which says what to check *after* a stage and is silent on how the
  lead knows the stage is over.
  **Not already covered by the adjacent paragraph — re-verified 2026-08-01 at
  scope.** `templates/lead.md`:62-67 (never hand-derive prior-stage completeness;
  trust `enter-stage.sh`'s refusal or `--simulate`) is a **gating** rule, predates
  the incident, and is silent on **liveness** — the simulated entry cleared
  mid-write, which is the proof.
  **Assertability — checked, not assumed.** The precondition is **prose-only**:
  the notification is harness session state with no tracked artifact, so no
  battery gate can read it (precedent: the sibling-dispatch clause, prose in the
  same template for the same reason). But the *negative* is assertable, which is
  the pairing that matters — "is the producer still running?" is exactly what the
  lock sentinel under `validate-producer-liveness-unobservable` reads. The two are
  one unit: prose rule on the dispatch side, oracle on the artifact side.
  **Disposition of the incident as a set.** This rule is the proximate cause and
  cheapest fix, so it goes first — but not alone:
  `validate-verb-collision-and-check-routing` establishes that a prose fix with no
  oracle recurs and is caught only by an operator. Three real findings, not four —
  the dispatch decision (this entry), the observability gap (the sentinel), and
  the artifact churn (`evidence-row-upsert-order`), worth fixing on its own
  merits. A fourth reading, that `check-evidence-manifest` is grammar-only, is
  retracted; see the assertion-A note under the sentinel entry.
  **Why `[design-pending]`:** it adds a precondition to a shipped template's
  dispatch contract and states a limit (prose-only, human-enforced) that
  lifecycle-kit/SPEC.md should own explicitly rather than imply.
  **Cost while deferred:** every multi-stage iteration with a live lead can
  re-run this race, and the cost lands on the next stage session, which fights a
  file changing underneath it. Debt: one template rule plus one SPEC limit; adds
  no governed name.
  Filed 2026-07-25 by close, from the lead's own account; the operator ruled stage
  sequencing the lead's accountability, which is why this is a lead rule rather
  than a stage-session or gate concern.

- **platform-support-ci-matrix** [design-pending] [roadmap: next/reliability] — a leg per platform.
  roadmap-summary: A CI install-smoke leg per supported platform, or an honest label.
  The per-platform half carved out
  of `platform-support-contract` when that entry was scoped down to the floor
  contract for the `activation-path` iteration: a CI install-smoke leg per
  supported platform (Linux / macOS / Windows-WSL), or an explicit experimental
  label where no leg exists. The floor contract states *what* the toolchain
  floors are; this entry is the mechanism that proves a platform actually meets
  them, and the support matrix's per-platform rows are only as honest as the
  legs behind them.
  **Why deferred rather than built alongside the floors:** the repo runs two
  workflows (`.github/workflows/gates.yml`, `site-health.yml`), neither a
  matrix, so this is new runner spend on macOS and WSL images — and **no macOS
  or WSL adopter exists** to attest the spend. Demand-gated on exactly that,
  like the other adoption rungs.
  **The un-defer trigger, carried so the reason is not re-derived:** the gap
  that motivates the legs is now *stated* rather than contradicted —
  `platform-support-contract` landed the floor contract, so `docs/install.md`
  §Requirements says outright that stock macOS ships bash 3.2 and a BSD
  userland and that a GNU toolchain is an adopter action there. That statement
  is the whole of what the floor contract can do; nothing proves whether the
  battery actually runs on a Mac so prepared until a leg runs there, and the
  page's macOS support claim rests on reasoning until one does. So this entry
  un-defers on the first
  of: a macOS or WSL adopter appearing, or the support matrix wanting to promote
  a platform from experimental to supported.
  **Cost while deferred:** the support matrix's non-Linux rows rest on
  reasoning rather than on a green run, so a macOS claim is unfalsified in both
  directions; zero until a non-Linux adopter exists, then it becomes the
  dominant support load, which is the same shape the parent entry recorded.
  Bounded and non-rotting — the floor contract lands the falsifiable half.
  Debt: CI configuration over an already-stated contract; adds no governed name
  unless a platform label becomes a gated surface.
  Filed 2026-07-26 by scope, operator ruling at the `activation-path` unit-set
  escalation, split from `platform-support-contract`.

- **gate-tamper-consumer-gate-coverage** [design-pending] — this repo's gates
  are outside `check-gate-tamper` entirely, and the override that did it looks
  like an oversight rather than a decision. `delegation-kit/lib/delegation.sh`
  defaults `DELEGATION_KIT_GATE_FILES` to `<gates-dir>/check-*.sh` plus the
  runner and library under that dir, and the declaration is
  `declare -p … || <default>` — a consumer array **replaces** it rather than
  extending it. `scripts/delegation-config.sh` declares
  `("*/checks/*.sh" "gate-sdk/lib/gate.sh" "gate-sdk/bin/run-gate-tests.sh")`,
  which reaches every kit-shipped gate and drops `scripts/check-*.sh`, the very
  case the default existed for. The globs were almost certainly meant to be
  additive: the comment above them explains the `*/checks/*.sh` reach and says
  nothing about giving anything up.
  **The coverage half has landed — re-verified 2026-07-31 at build against
  `scripts/delegation-config.sh`.** The array now carries `scripts/check-*.sh`
  beside the other three globs, with the replace-not-extend hazard written into
  the comment above it, and delegation-kit/SPEC.md was corrected at the same
  close. What this entry still owes is the *mechanism*, not the value.
  **Deliverable, as it now stands:** a fixture arm proving that a `scripts/` gate
  edit co-staged with product code is blocked, which no existing arm covers; and
  the same replace-not-extend audit on `DELEGATION_KIT_META_PATHS`, which
  re-declares every default it overrides and so is correct by hand rather than by
  mechanism. The open design question is whether a consumer array should extend
  rather than replace — a kit-wide declaration-semantics ruling, not a config
  edit.
  **Why deferred rather than fixed in the batch that found it:** widening
  coverage may red-line commit shapes this repo's own history relies on — a
  consumer gate landing beside the product change it gates is exactly the shape
  the assertion rejects — so the fix needs a survey of recent gate-touching
  commits before it lands. Scope-gated intake.
  **Cost while deferred:** low, and lower than this entry claimed before the
  correction above — the coverage hole is closed, so what remains is that the
  correct value is held by hand: nothing pins the glob, no fixture arm would red
  if it were dropped again, and `META_PATHS` carries the same unpinned shape.
  Filed 2026-07-26 by build, on the lead's ruling during the
  `platform-support-contract` floor batch, from a probe of `check-gate-tamper`'s
  resolved runtime config.

- **lint-scope-hook-trigger** [design-pending] — `GATE_SDK_LINT_EXTRA_DIRS` widens
  what `check-shellcheck` scans but cannot widen when the generated hook fires
  it. The hook's trigger is expanded from the gate's `# graph:` couples
  (`scripts/*.sh,kit:*.sh`), and `kit:` resolves to kit roots — a set no
  consumer knob joins — so a directory only the knob names is linted by the
  full battery and by CI, never at commit time. gate-sdk/SPEC.md
  §check-shellcheck now states this limit; this entry owns the fix.
  **Deliverable:** a consumer-expandable `lint:` token beside the existing
  `kit:` token in `gate_expand_couples`, expanded at hook-generation time from
  `GATE_SDK_LINT_EXTRA_DIRS`, so a consumer's added lint directories reach the
  generated hook without any kit literal naming them — the same shape the
  `check-graph` / `graph-vocab.sh` pattern already uses to keep consumer
  vocabulary out of kit source.
  **Why deferred rather than taken with the knob:** it changes gate-sdk/SPEC.md
  §The `# graph:` manifest — a gate-sdk contract the `activation-installer`
  amendment's §Existing sections updated does not list, so it sits outside that
  envelope. Scope-gated intake, not a batch side errand.
  **Two cheaper-looking options are already dead; do not re-derive them.**
  Widening the gate's `couples=` with entries like `kit:lib/*.sh` is a no-op:
  the hook matches with a pattern comparison in which `*` spans `/`, so
  `<kit>/*.sh` already reaches every subdirectory of every kit — measured, and
  the reason the kit half of the originally-reported gap turned out not to
  exist. Setting `trigger=*` works but costs the full gate on every commit:
  `check-shellcheck` runs about five seconds, the second-slowest gate in the
  battery behind `check-docs-render-fidelity`.
  **Cost while deferred:** consumer-named lint directories — this repo's
  installer scripts and its runnable walkthrough — are covered one tier later
  than every other script in the tree. CI catches a lint regression there; the
  commit does not, so the feedback arrives after the push rather than before
  it. Non-rotting and bounded to directories the consumer chose to add.
  Filed 2026-07-26 by build, on the lead's ruling during the `activation-path`
  doctor + manifest batch, after measuring both the glob behavior and the
  gate's runtime.

- **installer-lifecycle-verbs** [design-pending] [roadmap: next/adoption] — update, diff, uninstall.
  roadmap-summary: update, diff and uninstall, so an install can be managed after init.
  The installer's second phase: `update`, `diff`, and `uninstall`, the verbs that
  manage an install after `init` has made one. Phase 1 shipped in
  `activation-path`; this phase was ruled a separate build unit at promotion and
  is filed here so it survives the amendment that governed it.
  **The manifest is already the contract, and that is what makes this additive.**
  All six `checkwright.lock` fields — `schema`, `version`, `commit`, `profile`,
  `kits`, `files` — carry a phase-1 reader today (installer/README.md §The
  manifest), and **no field exists whose only reader arrives in this phase**. A
  seventh field proposed for `update`'s benefit alone was deferred with it. So
  these verbs are second readers of a schema that already exists rather than a
  schema revision, and a `/spec` pass should not re-derive that off
  `installer/lib/common/lock.sh`.
  **Acceptance shape:** `uninstall` removes only manifest-recorded files —
  never a file the adopter wrote, which is exactly what the per-file hash the
  manifest already records is for; `diff` reports drift between the recorded
  hashes and the tree; and `installer/consumer-smoke/run-smoke.sh` covers
  install → update → uninstall per profile, extending the suite it already runs
  rather than standing up a second one. Every mutating verb carries `--dry-run`,
  the rule installer/README.md already states.
  **Couples to `plugin-marketplace`, in both directions.** That entry must
  package against `checkwright.lock` as its install-ownership contract, and the
  sequencing risk it flags is a second install model with **no upgrade or
  uninstall story**. This entry is that story. A marketplace design that lands
  first would either wait on these verbs or duplicate them.
  **Cost while deferred — corrected 2026-07-26 against `installer/lib/init.sh`;
  the prior text overstated it, claiming no upgrade path at all.** Verified
  against the source, the deferred cost is three narrower things:
  1. **`uninstall` is the one real capability gap.** Backing out means deleting
     files by hand against the manifest. Mechanical, since `files` records every
     path with its hash — but manual, with no verb.
  2. **Discoverability, not capability, on upgrade.** Upgrade *is* supported
     today and is spelled `npx checkwright@<newer> init`: the re-run refusal
     narrows to the downgrade direction only, an upgrade falls straight through,
     the profile is re-read from the lock, and only files whose hashes still
     match the manifest are rewritten — anything the adopter edited is reported
     and preserved. So `--force` is not the upgrade path and never needed to be.
     What it costs to have no `update` verb is that nobody guesses `init`.
  3. **The cross-version upgrade is implemented but not smoke-covered** — carved
     out as `installer-upgrade-smoke-arm`, which does not need these verbs.
  `--dry-run` already covers most of what a deferred `diff` would add: it
  prints the file plan and the manifest that would be written. Non-rotting, and
  bounded by the manifest being correct and complete meanwhile — the data an
  upgrade needs is recorded from day one, which is why deferring costs
  capability rather than fidelity.
  Filed 2026-07-26 by build on the lead's ruling at the `activation-installer`
  merge, from scope the amendment governed and its deletion would otherwise
  have dropped.

- **installer-smoke-manifest-write-collision** [design-pending] — a guaranteed
  collision between the validate roster's order and the spine's own write
  mechanism. `run-validate.sh` upserts each suite's row into the tracked
  `.workflow/validate-evidence.txt` manifest as it goes, while `installer_smoke`
  (via `scripts/pack-installer.sh`) hard-refuses a dirty `git status --porcelain`
  — so with the suite anywhere but first, every full run deterministically reds
  it. Not a race. Reproduced at `activation-path validate`: the suite logged
  "the worktree is dirty", and re-running it alone on a clean tree passed all
  three profiles.
  **Interim mitigation landed:** `installer_smoke` reordered strictly first in
  `EVIDENCE_KIT_SUITES`, with a load-bearing comment on why position matters.
  Two facts verified before landing, and they are what the durable fix rests on:
  `installer_smoke` is the **only** configured suite requiring this repo's own
  tree to be clean, and **nothing pins any suite to roster position 1**.
  **A baseline hold was tried and reverted the same session** — the negative
  result worth keeping. `installer_smoke` carries no parser override, so it falls
  to the default `exit-code` parser whose entire result is one row: a dirty tree,
  a broken profile, a pack failure and a genuine `init` regression all produce
  the identical row, and a held-constant baseline classifies every one of them
  `clean`. The hold did not preserve a known red, it blinded the suite to every
  other failure mode.
  **Deliverable — the durable fix, not yet built.** The reorder is
  positional and fragile: it re-breaks silently the moment a second
  clean-tree-requiring suite lands anywhere but first, and nothing asserts
  the ordering, so the failure mode is silent regression, not a red. The
  durable fix is one of: (a) have `run-validate.sh` batch its manifest
  writes to scratch and fold them into the tracked file only after every
  suite has run, removing the write-order dependency entirely, or (b) an
  assertion (a gate, or a `run-validate.sh` precondition) that reds when a
  clean-tree-requiring suite is not first — a scanner for "requires a clean
  tree" would need each suite to declare that need rather than inferring it
  from grepping its script, which is itself an open design question.
  **Why `[design-pending]`:** candidate (a) changes the writer contract
  `evidence-row-upsert-order` already covers ("a re-run supersedes this
  iteration's prior line for the suite, then appends") — an
  `evidence-kit/SPEC.md` ruling, not a script patch, and the two entries
  likely converge on one fix. Candidate (b) needs a declaration mechanism
  this repo doesn't have yet (no suite currently states its own
  preconditions in a machine-readable way).
  **Cost while deferred — corrected.** The reorder closes the deterministic
  every-run red. What remains open is the silent-regression exposure: a
  second clean-tree-requiring suite landing anywhere but first would
  reintroduce the identical collision with no gate to catch it, discoverable
  only by a session noticing the red and re-deriving this same diagnosis.
  Debt: converges `run-validate.sh`'s writer onto an order-independent
  mechanism, or adds a declared-precondition gate; no governed name yet.
  Filed 2026-07-26 by validate (`activation-path`), from the full evidence
  battery run.

- **installer-upgrade-smoke-arm** [design-pending] — `installer/consumer-smoke/run-smoke.sh`
  packs a single `$VERSION` and asserts the same-version re-run leaves the tree
  unchanged (idempotence). The **cross-version** upgrade path — `init.sh`'s
  version check falling through in the upgrade direction, the profile re-read
  from the lock, `claim()` re-applying while preserving adopter-edited files —
  therefore ships in v0.16.0 with no automated exercise. It is implemented and
  was read against the source, not run.
  **Deliverable:** pack twice at two stamped versions in the scratch consumer;
  assert the bump re-applies the recorded profile, leaves a deliberately-edited
  file untouched, reports it as changed, and updates the lock's `version`.
  Extends the suite that already exists rather than adding one.
  **Scope note:** it asserts behavior installer/README.md §init and §The
  manifest already specify — no contract question, just uncovered ground, so a
  spec pass should be short.
  **Cost while deferred:** the first adopter to move off v0.16.0 is the first
  execution of that path. Carved out of `installer-lifecycle-verbs` because it
  needs none of the verbs and is much cheaper than they are.
  Filed 2026-07-26 by close (`activation-path`), correcting a false
  no-upgrade-path premise against `installer/lib/init.sh`.

- **gate-file-coverage-closure** [design-pending] — the missing check class behind a
  hole this close fixed inline: nothing asserts that every gate script in the
  tree is matched by some `DELEGATION_KIT_GATE_FILES` glob, so a gate can sit
  outside `check-gate-tamper`'s assertion-A coverage silently. It did: this
  repo's consumer config declares the array, which **replaces** the kit default
  rather than extending it (`delegation.sh` guards it with `declare -p … ||`),
  and the declaration named only `*/checks/*.sh` — leaving all nine
  `scripts/check-*.sh` consumer-resident gates uncovered. Close restated the
  default's glob in `scripts/delegation-config.sh` and corrected
  delegation-kit/SPEC.md §Layout and configuration, which had described the
  knob as *widening* the default — the inverse of the mechanism.
  **The gate:** enumerate gate scripts (the `gates.list` registry resolved to
  files), and red any whose path no `DELEGATION_KIT_GATE_FILES` glob matches.
  Cheap and mechanically decidable — the coverage set and the glob set are both
  already in hand at gate time. Needs a `gate-sdk` fixture pair and a home
  (delegation-kit, since it reads that kit's knob).
  **Why `[design-pending]`:** it makes coverage-completeness a delegation-kit
  contract, which is a SPEC assertion, not just a new script.
  **Cost while deferred:** the config is correct today but unheld — the next
  consumer-resident gate, or the next kit-glob edit, can reopen the identical
  hole with nothing to catch it. Exactly the replace-vs-extend footgun the SPEC
  now documents but does not enforce.
  Filed 2026-07-26 by close (`activation-path`), generalizing the
  knowledge-friction captures that surfaced the replace-vs-extend semantics.

- **kit-owned-install-recipe** [design-pending] — a kit's **zero-config gate
  subset** (the gates it can register in a fresh consumer with no
  adopter-authored surface) is encoded twice, and no surface owns it: once in
  each kit's `smoke/install.sh` for its scratch consumer, and once in
  `installer/lib/common/recipe.sh`'s per-kit case arm for a real adopter.
  Verified on the tree at filing: 11 kits ship `smoke/install.sh`, 9 carry a
  `checks/` roster, and `recipe.sh` restates those rosters as literal gate-name
  lists. The installer's copy was not derived from any spec — it was produced
  by reading eleven smoke installs and probing a scratch battery, because
  naive registration of every `checks/check-*.sh` reds 29 of 74 gates.
  **The exposure, stated precisely.** The consumer smoke catches a roster that
  names a gate which fails to *resolve*. It cannot catch the live direction of
  drift: a kit that **adds** a zero-config gate the installer never learns
  about. That gate then ships to adopters unregistered and silent, and nothing
  reds.
  **Costed fix (carried from the gap bullet, unchanged).** Push a
  `bin/install-<kit>.sh` down into each kit as the product form of its README
  §Install; have `smoke/install.sh` delegate to it and the installer's `init`
  call it; add a gate asserting every kit root ships one. Roughly nine new
  scripts, eleven smoke rewrites, a `gate-sdk/SPEC.md` contract section, and a
  gate with a fixture pair. This is the De-literalization and Derivation-first
  fix in one: the roster stops being prose-and-literal in two places and
  becomes a thing each kit owns and the installer calls.
  **Why `[design-pending]`:** it adds a kit-root structural predicate (every kit
  ships an install entry point), which is a `gate-sdk/SPEC.md` contract change
  across all eleven kits, not a script patch.
  **Cost while deferred:** the two rosters drift open-loop — every kit that
  gains a zero-config gate widens the gap silently, and the installer's copy is
  re-derivable only by repeating the eleven-smoke read and the scratch probe
  that produced it.
  Filed 2026-07-26 by close (`activation-path`), draining the gap inbox; the
  gap was ruled outside the `activation-installer` amendment's envelope as
  cross-component across all eleven kits. The same finding was independently
  captured as a knowledge-friction re-derivation against
  `installer/lib/common/recipe.sh`; both converge here.

- **spec-measured-count-gate** [design-pending] — a **measured count authored into a
  canonical SPEC section goes stale with no oracle**. Reproduced twice in one
  iteration: the align audit found `gate-sdk/SPEC-action-run-shell.md` B2's block
  count wrong twice over (five, then six; the tree carried eight) and, worse,
  slated for `gate-sdk/SPEC.md` — where this same iteration's sibling unit
  falsifies it on the next commit. It was corrected by hand (the canonical
  section now carries the **derivation**, never the count), which is exactly the
  shape CLAUDE.md's Enforcement-first rule bars: the fix landed without the gate
  that catches the next one.
  **Gap generalization — the check class that should have caught it.** Neither
  existing gate covers it: `check-prose-enum` reads governed-set membership, not
  numerals; `check-spec-derivable-section` reads fenced-dump density. The
  candidate is narrow — a **bare cardinal qualifying a derived roster** in a SPEC
  section whose subject is a `gate_find`-derived set — and it is filed here
  rather than left as prose, per the Gap-disposition rule.
  **Why `[design-pending]`:** the false-positive surface *is* the design. "Bare
  cardinal near a roster noun" over-matches legitimate prose (a contract that
  genuinely fixes a count — "the four contracts", "both halves"), so the gate
  needs a principled trigger, and the honest outcome may be an opt-in
  `measured:`-style marker the author applies rather than a scanner that infers
  intent.
  **Widened 2026-08-01: the surface set is not SPEC sections alone.** A third
  instance landed in a **binding shim** — `.claude/commands/close.md`'s
  release-policy slot said to derive the bump off the note's "two sections" and
  called a no-bump iteration "both-None", where the cited owner fixes three; a
  close read it against a three-section note at the moment it was deriving a
  bump. `check-shim-restatement` cannot reach it by construction: it holds *copy
  shape* (an n-gram shared with the corpus), and a restatement that is **wrong**
  has diverged from its owner's wording, which is precisely what makes it not a
  copy. So the scanner this entry designs must range over binding shims too, and
  the instance argues for the opt-in-marker outcome over the inferring one.
  Fixed at source by deleting the cardinal rather than correcting it, so the trap
  is disarmed rather than re-armed.
  **Cost while deferred:** compounding and silent — this recurs at **every
  amendment that measures the tree**, the failure mode is a canonical doc
  asserting a false number, and detection is by hand at align if at all.
  Filed 2026-07-26 by close (`release-path-hardening`), draining the
  stale-measured-count bullet; costed at roughly one small unit.

- **native-gate-binary-port** [design-pending] [roadmap: next/reliability] — a new gate substrate.
  roadmap-summary: The gate battery as one native binary: real parsers, no GNU userland.
  Port the battery off bash-plus-GNU-userland onto one native compiled binary
  (Rust the lead candidate); scale is read off `gates.list` and `*/checks`.
  Standing pains: Linux-only reach (docs/install.md §Requirements;
  `platform-support-ci-matrix` carries that half); utilities on independent
  lifecycles (the declared bash floor is already wrong — gap inbox 2026-08-01);
  a compiler replacing contracts the sdk enforces by discipline plus meta-gates;
  and the sharpest — **an agent cannot read a binary**, so oracle-first becomes
  structural instead of doctrine it can route around (consumer-side; this repo keeps its source).
  **Wall-time is mis-costed here in both directions.** The entry claimed speed
  until scope measured it 2026-08-01; that correction over-charged the port with a
  third-party floor mostly its own input — `check-shellcheck` is 5921ms of a
  23718ms/90-gate battery, linting each kit's own bash, so a port deletes it.
  In-bash: **`gate-battery-parallel-execution`**, **`gate-battery-result-cache`**.
  **Correctness evidence (2026-08-01, operator-directed).**
  `release-step-verification` produced eleven defects, sorted by whether a port
  removes them. **Four are removed:** the opt-in shell error model (a probe under
  `set +e` turned a failed `gh` call into an empty list and a zero exit, reporting
  green forever from its own data source dying); the regex dialect split (a word
  boundary in GNU grep is a backspace in POSIX awk); textual parameter expansion
  leaving artifacts in derived values; and the heaviest, hand-rolled parsers
  standing in for real ones — live instance
  `action-run-shell-yaml-anchor-fail-open`, an anchored `run:` body going
  silently unlinted. A real parser handles anchors by construction, and several
  gates share that shape. **Seven survive unchanged, and they are the ceiling:**
  CI-checkout and API-pagination semantics, evidence that does not identify what
  it certifies (`gate-tests-suite-identity-in-evidence`), presence asserted where
  resolution was needed, an ambiguous census, a SPEC claiming coverage the wiring
  lacks (`smoke-battery-workflow-gate-coverage`), and a required permission with
  no oracle (`workflow-permissions-scope-oracle`). **Consequence for scope:**
  justify the port on *silent success* and real parsers replacing regex
  approximation — never on speed, never on the vaguer claim that shell is
  error-prone, and not as closing `vacuous-assertion-count-discipline`, whose
  dominant class is assertion *design* and ports intact. Landing the port then
  relaxing is the failure mode to design against.
  **Deliverable:** one multi-call binary, a subcommand per check; `gates.list` dispatching
  per-entry to subcommand or script so it lands cohort by cohort, slowest and meta-gates first;
  each gate's fixture pair is the parity oracle before its script retires; consumer gates keep
  the shell escape hatch; checksummed per-platform artifacts, buildable source, needing only git.
  **Why `[design-pending]`:** the consumer-extensibility model decides everything
  else — escape hatch vs declarative check DSL vs native plugins — plus language
  choice, the dogfood question, and the trust inversion (opacity to agents is
  opacity to human adopters too, so reproducible builds and checksums are owed).
  Distribution is the hard part: kits vendor as text, zero build step. Cost apart.
  **Cost while deferred:** every new gate adds shell to the eventual port, the
  silent-failure classes above stay reachable, source-prediction waste recurs per
  session. Nothing breaks — correct on Linux, fully gated. Feature-shaped: it
  adds governed names. Filed 2026-07-28 by operator request.

- **gate-battery-parallel-execution** [design-pending] — `run-gates.sh` runs the battery
  serially: no `xargs`, no `&`, no `wait`. Measured after the spawn-hoist unit
  landed: 23718ms/90 gates, of which `check-shellcheck` alone is 5921ms. Spread
  across cores the remaining gates sum to well under that, so **`check-shellcheck`
  becomes the critical path** and the battery floors at roughly its cost —
  splitting its corpus across workers is what breaks the bound, not the scheduler.
  **The deliverable is the concurrency contract, not the scheduler.** Per-gate
  scratch isolation (`.tmp/` is one shared dir today); the timings file as a
  contended writer; deterministic output ordering under interleaved completion,
  so a red reads the same way twice; and an ordering constraint between gates
  that regenerate projections (`check-graph`, `check-enforcement-fresh`, the
  rollups) and gates that read them.
  **Adjacent lever, same measurement:** several gates each walk the whole tracked
  file tree independently. One shared walk feeding many readers is a structural
  win per-gate hoisting cannot reach, and is what one binary does natively.
  **Relation to `native-gate-binary-port`:** the port subsumes this — a native
  binary gets worker threads nearly free — so a port-first ruling closes this
  entry rather than duplicating it. Filed anyway because it pays if the port
  slips, and the isolation contract is owed either way.
  **Cost while deferred:** every validate and every close pays the serial battery,
  and each new gate lengthens it — the cost grows with the roster and is paid by
  every session, not once. Nothing is incorrect while deferred.
  Debt: no governed name. Filed 2026-08-01 by the lead at operator direction,
  from the battery profile measured during `delegation-reach-and-gate-cost`.

- **gate-battery-result-cache** [design-pending] — re-run the battery over an unchanged tree and
  every gate redoes its work. This iteration ran it well past half a dozen times,
  nearly always over trees where most gates' coupled inputs had not moved.
  **The cache key is already derivable, which is the whole case.** Every gate
  declares its inputs in its `# graph: couples=` manifest — the same source
  `gen-pre-commit.sh` projects the hook from — so a key is a content hash over
  the expanded couples plus the gate's own source. Nothing new to maintain, and
  `check-graph` already gates the manifest's freshness.
  **Opening question, to be answered rather than assumed:**
  **`docs-renderer-batch-contract`** ruled a content-hash cache *out* inside its
  own scope 2026-08-01. Whether that reasoning generalises from one renderer to
  the battery is this entry's first work — inherit it or overturn it on evidence,
  never ignore it.
  **The risk is invalidation, not speed.** A gate whose real inputs exceed its
  declared couples would be skipped while stale — a false green, the exact
  failure class the battery exists to prevent. `check-reads-couples` is the
  existing oracle for that gap and would become load-bearing rather than
  advisory, so its own coverage is a precondition, not a detail.
  **Relation to `native-gate-binary-port`:** the port makes this ordinary
  engineering instead of shell bookkeeping; a port-first ruling closes it.
  **Cost while deferred:** repeated full-battery runs inside one session are the
  dominant waste — a stage that validates after each commit re-runs every gate
  over a tree it just proved. Nothing is incorrect while deferred.
  Debt: no governed name. Filed 2026-08-01 by the lead at operator direction.

- **state-representation-integrity** [design-pending] — repo state lives in text
  files whose invariants — slug uniqueness, cross-entry `blocked-by` and `spec`
  targets resolving, one evidence record per suite — are properties of the
  readers, not of the format. Each is enforced by a gate that greps, so an
  invariant with no gate is silently unenforced rather than merely unchecked.
  **Measured evidence (2026-08-01, operator-directed), to be weighed at a future
  scope entry and opening nothing now.** Asked whether an embedded database should
  replace the text surfaces. Against `release-step-verification`'s eleven defects,
  a store with constraints prevents **two**: the interrupted validate run leaving
  a half-written evidence file (a transaction rolls it back), and
  `gate-tests-suite-identity-in-evidence` (a primary key makes an unidentifiable
  record unrepresentable). The other nine are CI-checkout and API-pagination
  semantics, assertion design, and missing oracles — untouched by storage. The
  counterweight belongs in the entry too: `check-task-conservation` *did* catch a
  deleted slug, as a grep over text.
  **Diffability is load-bearing here, not incidental.** `TASK-QUEUE.md` and the
  gap inbox are reviewed in diffs, merged across concurrent sessions by
  `merge=union`, and carry paragraphs of human reasoning. A binary store ends
  `git diff`, `git blame`, line-level review and union-merge; prose in a TEXT
  column buys no integrity over the part that matters most.
  **So split by shape, never convert wholesale.** Machine-written append-only
  records — the evidence manifest, `WORKFLOW-STATE.txt`, `tightened-gates.txt`,
  `gates.list` — are tables pretending to be files, but their concrete need is
  atomic append and self-identifying records, a far smaller fix than a database.
  Human-authored documents stay diffable text.
  **Deliverable, and it converges with `native-gate-binary-port`:** both reduce to
  *parse once into a typed model and validate there* — one parser and a schema
  replacing N hand-rolled greps. A store is one way to reach that model, a real
  deserializer over typed structs another. Cost them together or the same work is
  counted twice. The middle path is this repo's derivation-first rule one level
  further: keep text canonical and generate a freshness-gated queryable index, as
  `check-graph`, `ROADMAP.md` and the enforcement map already do, so gates run
  uniqueness and referential-integrity queries against the projection. That keeps
  diffs, centralises parsing, and trades *prevention* for after-the-fact
  validation — which is the bargain the gate model already makes.
  **The strongest argument for a store is one nobody has made yet:** there is no
  concurrency primitive. Shared-index contention, `merge=union` as a workaround,
  and a live session's journal clobbered mid-iteration are symptoms of its
  absence. Weigh it apart from the integrity case.
  **Cost while deferred:** gates keep hand-rolling parsers over state surfaces and
  integrity stays gate-enforced rather than structural — caught at commit where a
  gate exists, silent where none does. Nothing breaks today.
  Filed 2026-08-01 by operator request at close, on evidence the same iteration
  produced; a sibling rather than an addition because `check-queue-entry-budget`
  refused the combined body.

- **docs-root-link-grammar** [design-pending] — a hand-authored `docs/` page that
  links a path *outside* `docs/` with a bare relative link resolves on disk but
  404s on the rendered site, which is served from `docs/` as its root.
  `check-md-refs` resolves the target on disk and stays green, so nothing
  catches it; `jekyll-relative-links` cannot rewrite it either, because the
  target is outside the Jekyll source and so has no built URL to rewrite to.
  Verified live at close: `docs/orchestration.md` lines 90 and 108 both link
  `../lifecycle-kit/templates/lead.md`, two reader-facing broken links standing
  today. The generated mirror already emits the self-repo blob grammar for
  exactly this reason (`scripts/gen-docs-mirror.sh` rewrites source and
  directory links), and `docs/index.md` reaches `ROADMAP.md` that way by hand —
  so the correct form is established and only the hand-authored path is
  unguarded.
  **Deliverable:** a gate asserting that a link from a `docs/` page to a target
  outside `docs/` uses the blob form, plus — under enforcement-first, in the
  same unit — the sweep of the existing violations.
  **Why `[design-pending]`:** the boundary predicate needs care. Relative links
  *within* `docs/` are correct and must stay silent; the mirror's own pages are
  generated and already conform; and the rule must not fire on anchors or
  absolute URLs. Whether this is a new gate or an assertion inside
  `check-md-refs` (which already resolves every link and knows the target path)
  is the open call — folding it in reuses the traversal, but gives a
  disk-resolution gate a site-topology opinion.
  **Cost while deferred:** two reader-facing broken links stand on the rendered
  site, and every hand-authored docs page added meanwhile can add another
  silently. Not covered by `rendered-site-link-monitor`, whose scope is
  external-URL rot and which rules a hermetic gate out on false-positive
  grounds — this class is intra-repo and decidable from the tree alone, so that
  ruling does not transfer.
  **Ruled into `release-signaling-reset` as a deliberate second surface
  (2026-08-01, operator ruling at that scope's unit-set escalation).** The
  iteration's other four units share the release surface; this one is `docs/` +
  site-kit. The amortization dilution was named at the escalation and accepted
  knowingly, because these are the only currently-broken reader-facing artifacts
  in the queue — re-verified live at that scope: `docs/orchestration.md`:90 and
  :108. Fixing the two links **without** the gate was explicitly rejected, on
  enforcement-first. Recorded so the next reader does not read an off-surface
  unit in a single-surface iteration as an accident.
  Filed 2026-07-27 at align in front-door-readiness, while verifying the
  roadmap amendment's docs-home link; re-verified at close.

- **gate-exemption-live-slug-derivation** [design-pending] — an exemption can
  resolve green against no task at all. `check-gate-exemption-tasks` resolves an
  `# until: <slug>` marker against a live-slug set it derives by scanning the
  wrong thing, twice over.
  **(1) Section span.** The scan opens on the active and deferred headings and
  closes on Done / Lessons Learned with no reset on an unknown heading, so any
  section added between them is silently swept in as live. The icebox tier
  landed this iteration relies on exactly that accident for its placement.
  **(2) Token scan — strictly worse than (1).** Inside that span the awk reads
  every bold-emphasis token on every line rather than bullet lead lines, so the
  live set is 109 tokens against `queue_live_slugs`' 61 real slugs. Spurious
  members include `scope`, `debt`, `monitor`, `never`, `minor`, `resolved`,
  `source`, `output` — an `# until: scope` exemption resolves green today,
  against no task, with no icebox involved.
  **Why `[design-pending]`:** the two halves are one fix and the fix is a seam
  question — gate-sdk needs a real slug parse it can own **without depending on
  queue-kit for the section set**, which the provenance seam forbids. Candidates
  are a gate-sdk-owned live-section knob, an explicit heading reset, or a
  lead-line predicate; which one keeps the gate consumer-agnostic is the open
  call.
  **Cost while deferred:** low and non-rotting, but live now rather than
  contingent on the icebox — the token half is exploitable at HEAD, and it fails
  **open** (an exemption that should red resolves green) rather than loud.
  Surfaced 2026-07-29 at spec while surveying readers for the icebox tier; the
  token-scan half added 2026-07-31 at align. Drained from the gap inbox by close,
  which triaged the two bullets as one unit on the align bullet's instruction.

- **kit-index-page-vocabulary-ungated** [design-pending] — a kit index page
  carries governed vocabulary under no content gate. `docs/queue-kit/index.md`
  enumerates the task tags but is hand-authored, outside the docs mirror, and
  reached by no content assertion:
  `check-docs-kit-parity` and `check-docs-nav-reachable` read front matter and
  nav only. It survived this iteration's `[needs-spec]` → `[design-pending]`
  rename solely by accident — it happens to enumerate all five task tags and
  sits in `CANON_KIT_MANIFEST_FILES`, so `check-prose-enum` reaches it by
  completeness rather than by design. A page enumerating four would have drifted
  silently.
  **Why `[design-pending]`:** the deliverable is not "gate this page" but a
  ruling on what a kit index page *is* — a mirrored projection, a hand-authored
  surface carrying a declared enum obligation, or prose that must not enumerate
  at all. Each answer puts the fix in a different kit and a different gate.
  **Cost while deferred:** low and non-rotting today, but every kit index page is
  the same shape, so the exposure is per-page and grows with the docs site.
  Surfaced 2026-07-31 at build while tracing what the tag rename reached; drained
  from the gap inbox by close.

- **knob-rename-compat-threshold** [design-pending] — lifecycle-kit/SPEC.md's
  knob-rename compat precedent makes a rename compat-free "before the first
  release tag … because no external consumer can have vendored the kit yet (the
  first tag is a launch-comms prerequisite)". The tag was a proxy for that stated
  premise and the two have come apart: 17 tags have shipped, launch has not
  happened, and no external install has ever been observed — so the premise still
  holds while the threshold reads as long crossed. It misfires on the next real
  knob rename, demanding a deprecation window that protects nobody.
  **Why `[design-pending]` — two open questions the fix must settle.** (i) What
  observable ends the compat-free window in place of the first tag; the declared
  `preview-release-cadence` rung is a nominee, not a ruling. (ii) Whether the
  corrected clause generalizes beyond knobs: it is knob-scoped today — headed for
  knobs, sitting above the `LIFECYCLE_KIT_*` roster, cited nowhere else — and that
  scoping is exactly what kept it off this iteration's two renames.
  **Cost while deferred:** zero today and non-rotting, since the clause governs
  knob renames only and none is queued — but it bites the first time one is, and
  it bites as a **false obligation** rather than a missing one, which is the
  harder kind to notice.
  Surfaced 2026-07-31 at spec, escalated and ruled during
  `pre-adoption-grammar-break`; drained from the gap inbox by close.

- **rule-reach-before-merits** [design-pending] — the iteration's recurring
  failure mode, in both its forms, with no durable home. Six times a question
  that presented as a merits call turned instead on the governing rule's **scope**.
  **Form one — a rule invoked that does not reach the unit (five instances).** At
  scope the operator's pre-launch bar was the Enhancement decision rule and three
  of four units never reached it; at spec the knob-rename compat precedent reached
  neither rename; at align the release note's Renamed-knobs section proved scoped
  to own-config knobs twice over and so took neither rename either. In none was an
  exception owed or taken — and logging an exception where the rule never reached
  corrupts the record for the next reader.
  **Form two — a governing mechanism that DID reach and went unread (one
  instance).** Three sessions designed new mechanism for the close-entry refusal —
  an early tag, a rescoped preflight, a new `verdict=` state, a smoke redesign —
  before anyone read evidence-kit's held-constant-red baseline idiom, which already
  covered the case exactly and needed no new mechanism at all.
  They are one failure mode seen from two sides: **establish what a rule governs
  before arguing where its line falls, and read the governing mechanism before
  designing its replacement.**
  **Why `[design-pending]`:** placement is the whole question. As stated it is a
  delivery-doctrine rule — `doctrine-kit/DOCTRINE.md`, re-vendored, so it owes a
  release-note bullet — with an always-loaded one-liner above it; but form two sits
  close enough to the existing Oracle-first and Spec-over-precedent rules that it
  may belong as a clause on one of them rather than as a new rule. Minting a rule
  that restates a neighbour is the exact defect the doctrine's own content-tiering
  rule forbids, so the split is the design.
  **A third candidate rides the same placement question, and only that question.**
  Twice this iteration an oracle's *satisfaction* was read as the spec: the
  upgrade smoke's demand for one Tightened-gates bullet where the contract wanted
  three, and the budget guard's `OK` read as a dispatch decision. Generalized —
  *an oracle's pass is a floor, not the specification* — that is a doctrine rule
  sitting beside Oracle-first, and it is named here rather than filed separately
  because whoever settles new-rule-versus-clause for the two forms above settles
  it for this one in the same motion. The concrete half is already landed in
  delegation-kit/SPEC.md at the guard's contract; only the doctrine tier is open.
  **Cost while deferred:** the pattern recurred six times in one iteration and is
  paid in rework rather than in tree state — a wrong scope read spends a session's
  design effort on a rule that was never in play. Non-rotting; nothing degrades.
  Surfaced across scope, spec, align, build and validate of
  `pre-adoption-grammar-break`; drained from the gap inbox by close, which merged
  the original three-instance lesson with its inverted-form correction.

- **absence-statement-grammar** [design-pending] — operator-directed: prefer
  omission over a sentence that restates absence, and a bare token (`None`, or a
  dash) where a statement of absence *is* required. Live instance:
  `queue-kit/bin/roadmap.sh`'s empty-horizon placeholder emits a full sentence
  saying nothing is queued, asserted by its own gate test, where an empty section
  says the same thing.
  **The rule must not be blanket, and that is the design.** docs/install.md's
  upgrade contract deliberately requires `None.` stated-never-omitted on the three
  release-note sections, because there it is a checklist the reader must know was
  considered — and that same passage already forbids a clause that only restates
  the heading's own negation. Formulation to land: **state absence only where the
  reader must know it was considered; omit where the structure already shows it;
  when stated, a token, never a sentence.**
  **Why `[design-pending]`:** it touches `doctrine-kit/DOCTRINE.md` (re-vendored,
  so it owes a release-note bullet) plus one always-loaded `CLAUDE.md` line, and
  changes `roadmap.sh`'s placeholder and its fixture — a doctrine ruling and a
  behavior change in one unit, and the doctrine half must be worded so it does not
  falsify the install-doc carve-out it sits above.
  **Cost while deferred:** low and non-rotting but recurrent — agents keep landing
  on the wrong side of a distinction the tree makes in practice and states nowhere.
  Surfaced 2026-07-31, operator-directed; drained from the gap inbox by close.

- **statusline-queue-section-counts** [design-pending] — operator request: surface
  `TASK-QUEUE.md` section counts in the statusline in a compact layout — features,
  debt, deferred, icebox as single-letter counters, the deferred counter explicitly
  wanted. Needed for the operator's own statusline at minimum; extending
  `delegation-kit/templates/statusline-usage.sh` for consumers is welcome if the
  mechanism carries. That template already opens the queue (for the iteration name)
  and the lifecycle state file (for the stage), so counts add no new file access
  and no new dependency.
  **The design question that must not be assumed away:** section names are consumer
  config — `QUEUE_KIT_ICEBOX_SECTION` and its canon-kit and drift-kit counterparts
  — so the counter must resolve **configured** section names. Hardcoding this
  repo's four headings would put one consumer's layout into a kit literal, which is
  the provenance seam.
  **Two surfaces, separate edits:** the kit template is a consumer-copy producer,
  so the operator's live statusline is a copy and will not update from a template
  change.
  **Cost while deferred:** low and non-rotting; the residue is that queue shape is
  invisible at a glance and re-derived by running the queue index or opening the
  file.
  Surfaced 2026-07-31, operator request; drained from the gap inbox by close.

- **contributor-writeback-disposition** [design-pending] — operator-directed: every
  GitHub boundary-sweep disposition must write back to the contributor, and the
  comment must be appreciative — explicitly **including** the discard cases. A
  contribution declined without visible thanks spends community goodwill the
  project cannot refund. Current state in this repo's scope shim (the kit template
  carries no GitHub sweep, so this is local policy, not a kit change): five
  dispositions across two lanes, only three write back at all, and none specifies
  tone, so the discard cases read as bare rejection.
  **(1) Promotion writes back nothing.** An issue we accept, queue, and work leaves
  its reporter with silence — the worst case, because it is where we have most to
  say. The clause also states no **fate** for a promoted issue: left open it
  re-enters the per-lane cap at every boundary, and the sweep cannot distinguish
  already-promoted from never-triaged, because the "Surfaced by GitHub issue #N"
  citation lives on the queue entry rather than on the issue.
  **(2) The cap falsifies any blanket promise.** The sweep caps each lane at five
  items per boundary, so item six receives no disposition and no comment at all,
  not even a decline; with one iteration running at a time the long tail waits in
  silence across boundaries, and a contributor never reached cannot distinguish
  queued from ignored. Either an acknowledgment pass cheap enough to run uncapped
  while the analysis stays capped, or a promise worded to what the cap can keep.
  **Do not fix the tone requirement without settling this**, or the result is a
  stated commitment the mechanism cannot honour.
  **(3) The decline needs a taxonomy, not a template.** The operator-directed shape
  — thank, acknowledge the contribution lacks general applicability, suggest
  implementing it locally — is credible here because gates resolve consumer-first
  with kit shadowing and the provenance seam already mandates that non-general
  content become optional consumer config. But it is honest **only** when the
  reason really is "correct, but not general"; applied to wrong, duplicate, or
  seam-crossing contributions it is a form letter, which reads as less respectful
  than a short honest no. Minimum taxonomy: not-general, already-covered,
  incorrect, seam-violating — plus, in the PR lane, right-idea-no-fixture, whose
  honest closing is an invitation to resubmit rather than a decline.
  **(4) Content tiering.** `CONTRIBUTING.md` is where a *contributor* reads what
  happens to their contribution; the scope shim is where the *scope session* reads
  the ritual. Different readers, so both may carry a line without it being
  restatement — but that split must be ruled deliberately rather than duplicated by
  accident.
  Distinct from `security-advisory-lane`, an unswept third lane; this is a
  write-back defect in a lane already swept.
  **Cost while deferred:** low in tree terms and non-rotting, but it accrues
  against people rather than code, and the repo is pre-launch — the first external
  contributors meet whichever behaviour is in force then.
  Surfaced 2026-07-31, operator-directed across three gap-inbox bullets; drained
  and merged into one unit by close.

- **queue-index-blocked-by-assertions** [design-pending] — coverage hole in
  `queue-kit/gate-tests/queue-index.test.sh`, pre-existing: the blocked-by tag's
  re-echo and the ready/blocked marker (bullet vs cross) are asserted nowhere,
  although the sibling drain-exempt echo is. Surfaced while fixing the title
  rendering this iteration, because that fix restructured the very expression
  concatenating the blocked-by suffix onto the title — the hole sat directly under
  the change.
  Verified by running rather than by reading that nothing regressed: a fixture
  carrying a blocked entry, one whose lead line is all tag, and one carrying both
  rendered byte-identically before and after the fix apart from the intended
  separator change. But the assertion that would have caught a regression does not
  exist, and the marker derivation is likewise unasserted.
  **Cost while deferred:** low and non-rotting — two `want()` lines close it — and
  the residue is only that a future edit to the title-and-tag concatenation can
  regress the echo silently.
  Surfaced 2026-07-31 at build; not folded into the fix commit because the unit's
  commit had already landed under a HEAD moved by a concurrent session, so landing
  it meant either a second commit on a closed unit or a rewrite of shared history.
  Drained from the gap inbox by close.

- **context-pressure-signal** [design-pending] — operator request: a
  usage-verdict-style context-pressure signal that **suggests** compaction, so the
  decision stops depending on a lead's guess. The source needs no estimation: the
  harness hands the context window's used percentage to the statusline, and
  `delegation-kit/templates/statusline-usage.sh` already reads and renders it as
  the ctx gauge, then drops it. The pipeline exists too — the statusline is the
  `usage.txt` producer and `bin/usage-verdict.sh` its consumer — so the work is
  roughly to add a context-used key to the payload and give it a verdict.
  Explicitly **not** transcript parsing off `drift-kit/bin/overhead-meter.sh`:
  bytes are not tokens, and the transcript outlives the window across a compact.
  **Three constraints the design must settle.** (i) Only the statusline can produce
  it — the usage poller reads an account endpoint for rate limits, and context
  usage is a per-session client-side number no endpoint knows, so the key is
  statusline-only and absent under the poller producer, which the payload's
  optional-keys rule accommodates but the SPEC must state. (ii) **The real
  problem** — `usage.txt` is one global file while context usage is per session.
  Rate limits are per-account so sharing is correct for them; this repo runs a lead
  plus concurrent stage sessions each rendering over the same path, so whoever
  rendered last wins and a lead could read a stage session's number and act on it.
  Sub-agents that render no statusline contribute nothing at all. Keying by session
  id or writing per-session is the call. (iii) It suggests and never blocks —
  unlike the budget guard, where a false negative kills a dispatch, a false
  positive here costs context that did not need to be lost. Shape is a **hook, not
  a gate**, on the agent-budget-guard precedent.
  **Ownership is a genuine fork:** delegation-kit owns the pipeline and the
  statusline template; context-kit owns session context and the session brief.
  **Cost while deferred:** low and non-rotting, but the residue is that compaction
  timing stays a judgment call made by the one party that cannot see the number.
  Surfaced 2026-07-31, operator request superseding the lead's own first sketch;
  drained from the gap inbox by close.

- **workflow-state-direct-edit-guard** [design-pending] — operator question, worth a
  unit: can a direct edit to the lifecycle state file that bypasses
  `lifecycle-kit/bin/enter-stage.sh` be blocked? Findings, verified rather than
  assumed.
  **(1) Content-based detection is impossible and should not be attempted** — a
  hand-written stamp is byte-identical to an `enter-stage` one by design, and any
  marker `enter-stage` adds is equally writable by whoever is hand-editing.
  **(2) Much of the bypass is already closed**, which was not obvious:
  `check-evidence-manifest`, `check-stage-entry` and `check-stage-evidence` are all
  registered gates, and the manifest check reads the cursor out of the state file
  itself — so a hand-stamped `close` entry reds at the offender's first commit on
  the same assertion that refused the entry. The entry preflight and the pre-commit
  gate are the same code reading the same cursor.
  **(3) The missing lever** is a `PreToolUse` matcher on Write|Edit denying the
  state file; today `PreToolUse` matches only Bash and Agent, and guard-kit's
  bash-guard is the established precedent for exactly this shape.
  **(4) The actual hole is neither** — the cursor is the **worktree** file's last
  stamp and every reader reads the working tree, while gates fire only at commit.
  An uncommitted hand-edit moves the cursor immediately for the whole session, and
  a session that never commits its stamp is never caught at all. That window is
  ungated by construction and a Write/Edit guard is the only thing that reaches it.
  Residual and unclosable in-repo: `--no-verify`, and a human editing outside the
  agent tooling.
  **Why `[design-pending]`:** which kit owns a Write/Edit-side guard is unsettled —
  guard-kit owns the tool-call guard mechanism while lifecycle-kit owns the state
  file — and the settings hooks are a pinned surface, so the unit changes one.
  **Cost while deferred:** low today and non-rotting, but it is the integrity floor
  under every lifecycle claim the repo makes, so the exposure is reputational
  rather than mechanical.
  Surfaced 2026-07-31 by the close-entry refusal the same day, where `enter-stage`'s
  own refusal text openly offered a deliberate hand-stamp and the session declined
  on the close ritual's rule — i.e. the current control is the agent's judgment,
  which is what this request wants to stop relying on. Drained by close.

- **readme-roster-enum-coverage** [design-pending] — a kit README enumerating a
  **derivable** set is outside every parity gate, so it drifts silently while the
  battery stays green. `check-readme-roster` holds one roster per README — the
  `checks/` basenames — and nothing else; `check-prose-enum` holds only the sets
  `scripts/enum-sets.sh` declares, which today is the queue tag vocabulary and
  nothing else.
  **Two instances, both found by close's step-5 staleness read rather than by an
  oracle.** (1) drift-kit/README.md enumerates the bundled lead KPIs one-for-one
  and omitted `kpi-queue-net-delta`, shipped this iteration and registered in
  `scripts/kpis.list` — a registry that is exactly an enum-set source. (2)
  queue-kit/README.md's `## Use` block enumerates `bin/queue-index.sh`'s
  invocations and omitted `--icebox-candidates`, while queue-kit/SPEC.md states
  the tool's three modes outright; the README is the only invocation surface a
  reader gets. Both were corrected by hand at that close, which is the
  Enforcement-first shape the doctrine bars — the fix landed without the gate.
  **Why `[design-pending]`:** an enum set is cheap to declare and expensive to
  land, because declaring one obliges **every** prose enumeration of that set,
  tree-wide, to be complete. The unit owes a survey of what a `drift-kpi` set
  would red before it is declared, plus a ruling on whether a tool's *modes* are
  an enum set at all or want a different parity shape — the modes live in a
  SPEC sentence and an argument parser, neither of which is a registry file.
  The count half of this class is `spec-measured-count-gate`'s, not this
  entry's: a bare cardinal qualifying a roster is a different scanner from a
  membership check.
  **Cost while deferred:** low and non-rotting, but it is paid once per close —
  the staleness read is the only detector, so every README roster is held by a
  session's attention rather than by a gate.
  Surfaced 2026-07-31 by close's top-level staleness review, which found both
  instances; filed rather than fixed because the enum-set survey is the work.

- **close-generated-finding-route** [design-pending] — the gap inbox is drained
  **once, at close**, and close is the stage that generates findings by design —
  the audit sweeps, the lesson disposition, the staleness read, the release
  disposition. Anything close generates, or that close's own failure generates,
  necessarily postdates its drain, so a close-generated finding has no drainer
  inside its own iteration. This is not a filer's timing error to work around; it
  is untimeable.
  `bin/file-gap.sh` already warns that a filing made while the cursor sits at the
  last configured stage may have no stage left to drain it, but it frames that as
  the filer's problem. The design implication is stronger: the drain-once shape
  has its blind spot exactly where the lifecycle is most productive of findings,
  and the sanctioned fallback — the next scope entry refuses until the entering
  session promotes the bullets itself — silently pushes one iteration's record
  across the boundary into the next one's.
  **Evidence it is live rather than theoretical:** this iteration. The drain
  landed claiming 24 bullets dispositioned; five more findings arrived after it,
  and close re-drained.
  **Why `[design-pending]` — three candidates, three different failure modes.** A
  second drain pass late in close, after its own findings have settled, can loop.
  A re-arm refusing close's completion while the inbox is non-empty can deadlock
  close against its own filings. An explicit rule routing close-generated findings
  straight to the queue or to Lessons, never through the inbox, loses the inbox's
  one uniform capture channel and reintroduces the mid-iteration index contention
  the inbox exists to prevent. Picking among them is the work.
  **Cost while deferred:** low and non-rotting in tree terms, but it is paid every
  iteration that closes productively, and it is paid as a **record** defect — the
  findings survive, in the wrong iteration's ledger.
  Surfaced 2026-07-31 by the lead, deliberately **not** filed to the inbox on the
  ground that filing it there would compound the defect it names; drained by close
  in the same pass as the inbox.

- **gap-bullet-premise-verification** [design-pending] — a gap-inbox bullet
  asserts mechanism under no verification bar, and a false one is paid by
  whoever drains it. Two of the three bullets re-verified at this boundary were
  false at their central claim, both filed by the same close: one asserted the
  npm approval environment did not gate the `v0.18.0` publish (it gated — the
  deployment held 77 seconds and an approval is recorded), the other that
  `check-graph` prints no regeneration command on a red verdict (it prints both,
  with resolved knob paths). Each was falsified by a single command.
  **Why this is not `close-generated-finding-route`'s ground.** That entry owns
  *when* a close-generated finding gets drained; this one owns whether its
  factual claims were ever checked. A correctly-routed bullet carrying a false
  mechanism lands in the queue as a false premise — and this queue's own
  convention of dating premise corrections into entry bodies is the evidence
  that such a premise can then survive iterations before anyone re-derives it.
  **Why `[design-pending]`:** the affordance must not become a checkpoint. The
  inbox exists because deferred capture is no capture, so a verification bar
  that slows filing would trade a known failure mode for the one the inbox was
  built to prevent. Candidate shapes, with different costs: `bin/file-gap.sh`
  prompting for the command that establishes the claim, a grammar separating
  observation from inferred mechanism, or leaving capture untouched and making
  re-verification an explicit named step of the drain. The third adds no capture
  friction at all and is the one to beat.
  **Cost while deferred:** paid by the draining session, which either re-derives
  the mechanism or promotes a false premise into the queue where it reads as
  established; two instances in one boundary, from one close. Low and
  non-rotting — nothing in the tree degrades while it sits.
  Debt: a filing-or-drain discipline over an existing affordance; adds no
  governed name unless a grammar lands.
  Filed 2026-07-31 at scope from its own gap-inbox disposition; the lead ruled
  the observation durable but outside this iteration's ruled unit set.

- **always-loaded-regen-block-residency** [design-pending] — `CLAUDE.md`'s
  generated-hook paragraph carries two regeneration commands in the
  always-loaded tier, and the stated reason for leaving them there does not
  survive checking. Close's brevity pass trimmed only the enforcement-map half,
  on the ground that `check-enforcement-fresh` prints its regen command on red
  while `check-graph` does not.
  **That premise is false — verified at this scope, do not re-derive it.**
  `check-graph` prints **both** commands verbatim on exactly the reds where
  regenerating is the remedy: its hook-freshness assertion emits `regenerate:
  bash gate-sdk/bin/gen-pre-commit.sh --write` for a missing or stale hook, and
  its artifact assertion emits `regenerate: bash gate-sdk/checks/check-graph.sh
  --emit > <artifact>` for a missing or stale graph artifact — each with the
  *resolved* knob path rather than a literal, which is strictly better than
  `check-enforcement-fresh`'s single hardcoded help line. What `check-graph`
  omits them from is its generic `help:` footer, which covers manifest and
  parity reds where regeneration is not the remedy at all. So no echo line is
  missing and no gate change is owed.
  **A second instance, found 2026-08-01 by close's brevity pass.** The
  `ROADMAP.md` housekeeping bullet has the same shape — a prohibition (never
  hand-edit the marker block) plus a regen command (`roadmap.sh --write`) that
  `check-roadmap-fresh` already prints on red. It is deliberately left in place:
  trimming it would settle this entry's open question by precedent instead of by
  ruling. Whatever this unit decides applies to both bullets.
  **The residual question, and why it is still a unit.** The always-loaded block
  is a *forward* instruction — never hand-edit the generated hook; edit the
  manifest and regenerate — read before any red, not a recovery line read after
  one. Whether that earns always-loaded residency is the open call, and it is
  load-trigger residency applied to a surface no gate settles: the recovery path
  is already covered by the gate, so what remains resident is the prohibition
  plus the workflow, and the honest outcome may be the prohibition alone with
  the commands behind a load trigger.
  **Cost while deferred:** low and non-rotting, but charged against a tier
  trending the wrong way — the always-loaded KPI read 198 lines at this
  boundary, +26 across the closing iteration — and the falsified premise above
  was on course to be paid a second time by whoever picked the bullet up.
  Debt: a residency ruling over existing prose; adds no governed name.
  Filed 2026-07-31 by close as cheap-and-declined scope creep; promoted at the
  next scope entry from the post-close gap inbox, its premise falsified against
  `check-graph`'s source in the same pass.

- **post-immutability-machine-read-carveout** [design-pending] — the post
  immutability rule and the machine-readable-note rule are stated on two pages
  and neither records how they compose. `docs/site-architecture.md`
  §Page-authoring rules calls dated `docs/posts/` immutable, "temporal-exempt
  but still link/command-resolved"; `docs/install.md` §The upgrade contract
  declares a shipped note's Tightened-gates lead tokens a machine-read allowed-
  red set with one canonical spelling and no version cutoff. A shipped post
  whose tokens are mis-spelled is therefore both immutable prose and a wrong
  machine input, and no surface says which wins.
  **The working reading, and its status.** `release-assertion-honesty` repaired
  12 shipped bullets on the reading that the immutability carve-out already
  admits mechanically-held elements — a post is immutable *as prose* while its
  links, commands, and now its machine-read tokens stay resolved. The operator
  declined to widen the governance sentence, so that reading survives only in a
  transition-artifact amendment that is deleted at merge and in this entry.
  **Why `[design-pending]`:** the call is how wide the carve-out is stated —
  enumerate the mechanically-held element classes, or state the principle
  (immutable as prose, live as machine input) and let the classes derive.
  **Cost while deferred:** low and non-rotting today; nothing degrades while it
  sits. The failure mode is a governance sentence that under-describes its own
  carve-out, so the next session facing a shipped-post repair either re-derives
  the reading from scratch or refuses a correct repair on immutability grounds.
  That under-description is now demonstrable rather than predicted: close's
  staleness sweep flagged the sentence independently, on the evidence that its
  carve-out enumerates exactly two mechanically-held classes (links, commands)
  while a third now exists — `check-tightened-gates-grammar` runs at
  `tier=precommit` over `docs/posts/*.md` with no version floor, and seven
  already-shipped notes were edited to satisfy it.
  Debt: a governance sentence widened on one page; adds no governed name.
  Filed 2026-07-31 at spec; promoted at close from the gap inbox, its cost
  re-read at close against the sweep that found the same sentence unprompted.

- **tightened-gates-declaration-note-parity** [design-pending] — nothing asserts
  that a release note's composed Tightened-gates set equals the
  `.workflow/tightened-gates.txt` declaration surface it was composed from.
  `upgrade-smoke-note-resolution` made build append declared gate names to that
  tracked record and close transcribe them into the note's bullets and drain the
  file, both **by hand**, so a dropped or added name between the two surfaces is
  caught only by review — which is exactly what RELEASING.md step 1 states as
  its honest limit.
  Both sides are machine-readable by construction: bare names one per line on
  the surface, backticked unbolded bullet lead tokens in the note under
  `release-note-lead-token-grammar`'s canonical spelling, with
  `check-tightened-gates-grammar`'s parser already extracting the note side. So
  the parity assertion is mechanical, and the parser it needs already ships.
  **Why `[design-pending]`:** the assertion only holds at the release commit —
  before the tag the surface accumulates and no note exists; after the drain the
  surface is header-only and the note is the sole record. Which commit the gate
  binds on, and how it identifies the note under composition, is the open call.
  **Cost while deferred — now realized, not hypothetical.** The filing costed
  this as "low until the first release that ships a non-empty declared set".
  That release is `v0.19.0`, composed this close from a one-name surface, so the
  window is open from here on. A transcription slip either licenses an
  undeclared red or declares a gate that never tightened, and the upgrade smoke
  reads the note rather than the surface, so it cannot catch either.
  Debt: one gate plus its fixture pair over two existing surfaces.
  Filed 2026-07-31 at spec from the honest limit stated in the amendment's
  producer delta; promoted at close, its cost re-read against the release it
  now rides.

- **prose-filename-citation-liveness** [design-pending] — a bare `<name>.md`
  filename cited in governed prose can name no tracked file and nothing reds.
  `check-md-refs` resolves markdown *links* only; `check-spec-pointer` resolves
  `spec:`/`contract:` directives and free-prose `<path>.md §<heading>`
  citations, so a citation with **no** `§heading` and no link syntax falls
  between them.
  **Instance, fixed at this close:** `site-kit/SPEC.md` and its `docs/` mirror
  cited `SPEC-os-support.md` as owning `docs/install.md`'s Requirements ruling.
  The amendment is deleted, so the pointer dangled. Close deleted the
  parenthetical rather than repointing it — the same sentence already names
  `docs/install.md`'s Requirements prose as the stater, and that page carries
  the ruling. Same class as the seven citations the `release-assertion-honesty`
  build sweep cleaned, which is what makes this recurring rather than a one-off:
  every merged amendment deletes a file that governed prose may still name.
  **Why `[design-pending]`:** the predicate needs care, and a naive one reds
  correct prose. `docs/install.md` names `AGENTS.md` four times as a harness
  convention file that is deliberately untracked here — a legitimate citation of
  a filename with no in-tree target. Candidate narrowings: bind only the
  `SPEC-*.md` amendment-naming convention (narrow, decidable, covers the whole
  observed class), or extend `check-spec-pointer`'s free-prose extractor to the
  headingless form behind an exclusion roster. Which, and whether it is a new
  gate or an assertion inside an existing one, is the open call.
  **Cost while deferred:** one dangling citation per merged amendment that
  governed prose names, found only by a hand sweep somebody remembers to run;
  the citations read as live pointers until then.
  Debt: one gate plus its fixture pair, or an assertion added to an existing
  gate.
  Filed 2026-07-31 at close as the gap-generalization owed by the inline fix
  above; the `check-md-refs` blind spot and the `AGENTS.md` false-positive case
  were both verified against source before filing.

- **vacuous-assertion-count-discipline** [design-pending] — an assertion that
  passes without asserting now has three attested instances in this tree and no
  craft rule names it. In all three the tell was a **count**, never an exit code:
  the run was green and the set it ranged over was empty.
  **The three, all from `release-assertion-honesty`, all inside the fix for this
  very class** — which is the evidence that it recurs, not the irony: (i) a
  `None` predicate that silently matched nothing once it moved from `grep -E` to
  awk, because `\b` is a word boundary in ERE and a backspace in awk's regex
  (verified at this close: `awk '/\bNone\b/'` matches nothing on a literal
  `None` that `grep -cE '\bNone\b'` counts 1); (ii) a helper arm that would have
  shipped with no fixture to exercise it; (iii) a verification probe that
  mutated a gate the consumer never registers, so it reported a 0-wide red set
  against a non-empty declaration and read as a pass.
  **Where the concept already lives, partially.** `gate-sdk/SPEC.md` §run-gates
  names the *vacuous-pass tripwire* — a gate's clean line carries its scanned
  count, and `GATE_SDK_VERBOSE` is the reading that surfaces it — and
  `doctrine-kit/DOCTRINE.md` rule 22 carries the runner half. What is unnamed is
  the rule for a verification the *session itself* writes: a probe, a mutation,
  a predicate under edit. The tripwire exists for the battery and nowhere else.
  **Deliverable:** a craft rule in `doctrine-kit/DOCTRINE.md` §Engineering-craft
  rules — a verification whose subject is a set reports that set's size, because
  an exit code cannot distinguish "asserted and held" from "asserted nothing";
  and a mutation probe is valid only against a target the assertion actually
  ranges over. Filed rather than landed at close deliberately: rule 11 licenses
  no self-exemption, so amending this doctrine is a scoped unit.
  **Gap generalization, per instance.** (i) is fixture-shaped — the missing
  coverage is a good/bad pair over the `None` body, which the existing
  fixture-pair contract already requires, so this is enforcement that went
  unexercised rather than enforcement that is absent. (ii) is per-arm fixture
  coverage, already carried as the iceboxed
  `stage-economics-smoke-jq-arm-dormant`; a branch-coverage assertion over a
  check's fixture corpus is its buildable form. (iii) is **not** gateable:
  whether a session's mutation probe targeted something its assertion ranges
  over is a property of a session act that leaves no residue in the tree.
  **Why `[design-pending]`:** whether this ships as one craft rule, or as a rule
  plus the branch-coverage gate (ii) wants, is the open call — and a rule whose
  *Enforced by* line reads "judgment" must state the two-of-three split above
  honestly rather than promise a scanner.
  **Cost while deferred:** low and non-rotting in the tree; charged instead
  against every future assertion-shaped unit, where the failure mode is a green
  that means nothing and is indistinguishable from a green that means something.
  Debt: one doctrine rule, optionally one gate; adds no governed name unless the
  gate lands.
  Filed 2026-07-31 at close from the lead's dispatch, weighed as evidence and
  ruled **task**-shaped rather than lesson-shaped: the deliverable and its
  done-state are both nameable now, which is the litmus.

- **lead-iteration-open-authorization** [design-pending] — the iteration lead may
  open an iteration on its own inference, and opening one is the operator's call.
  lifecycle-kit/templates/lead.md §Opening an iteration requires only the
  operator's standing **directive** — a theme bounding scope's survey — and
  explicitly permits proceeding without even that ("absent a directive, the lead
  dispatches scope undirected"). The directive slot presupposes the decision to
  open has already been made and is silent on who makes it; re-verified at this
  close, the file contains no authorization word at all, and
  `lifecycle-kit/bin/enter-stage.sh` carries no floor either — its first-stage
  branch differs only in stamping `—` for the iteration name.
  **The live instance.** The operator said "fix all" about four outstanding
  release defects, two of them edits to governed surfaces; the lead read that as
  authorization to open an iteration, reset the boundary, dispatch scope, and
  stamp — and the operator then stated plainly that they had not asked for one
  and that opening one requires their explicit approval. The inference was
  defensible on the *content* (governed-surface edits do route through the
  lifecycle) and wrong on the *scale*, which was the operator's to choose.
  **Deliverable:** state in lead.md that opening an iteration requires explicit
  operator authorization, distinct from and prior to the standing directive, and
  that an ambiguous instruction to fix filed work is not that authorization — the
  lead asks. **Why `[design-pending]`:** whether a mechanical floor is buildable
  is the open question. Prose requests and guards enforce, so the shape worth
  costing is an `enter-stage.sh` first-stage precondition; against it, the
  authorization is a fact about a conversation, and every encoding of it
  (a flag, a stamped operator line) is forgeable by the same session it binds.
  **Cost while deferred:** a lead can spend an iteration's full opening cost on a
  scale the operator never chose, and the only detector is the operator noticing
  after the stamp has landed. Recurs per iteration opened under ambiguity.
  Debt: one prose rule, optionally one precondition; adds no governed name
  unless the floor lands.
  Filed 2026-08-01 at close from the gap inbox, operator-directed, from a live
  instance in this iteration's own opening.

- **smoke-battery-workflow-gate-coverage** [design-pending] — the consumer smoke
  installs workflow templates but no smoke gate lints their Actions shape, so a
  template regression reaches a consumer unopposed. The scratch battery is the
  union of what each kit's `smoke/install.sh` appends to `gates.list`, and
  `check-action-pinning`, `check-action-run-shell`, `check-action-gh-repo` and
  `check-enforcement-fresh` are in **no** kit's smoke `gates.list`. Re-verified at
  this close: `site-kit/smoke/install.sh` genuinely does copy
  `templates/site-health.yml` verbatim into the scratch tree, so the install half
  is real — but of the registered smoke gates only `check-tree-terms` and
  `check-docs-cname-parity` read the installed file at all, and both only scan it
  for terms and host aliases. Nothing lints its run bodies, its action pins, or
  its job shape. The SPEC sentence that inferred coverage from the install was
  narrowed to what the smoke actually exercises in the same close that filed this
  (site-kit/SPEC.md §templates/site-health.yml), so the tree no longer claims the
  coverage — this entry is the coverage itself, still absent.
  **Why `[design-pending]`:** registering four gates into a smoke `gates.list` is
  the obvious move and may be the wrong one — the smoke's purpose is proving a
  *vendored kit installs and runs*, not re-running the host battery inside it, and
  a smoke that grows toward the full roster stops being a smoke. The design
  question is which predicate earns a scratch-battery slot: plausibly only the
  gates that read a surface the install *writes*.
  **Cost while deferred:** charged against every edit to a shipped workflow
  template; a template shipping a defective `run:` body is caught by this repo's
  own battery over its own copy, and by nothing over the template.
  Debt: smoke wiring over existing gates; adds no governed name.
  Filed 2026-08-01 at close from the gap inbox, filed at spec while surveying the
  readers of site-health.yml.

- **action-run-shell-yaml-anchor-fail-open** [design-pending] — the gate whose
  whole job is linting workflow bash silently drops a `run:` body written behind a
  YAML anchor. gate-sdk/SPEC.md states `check-action-run-shell` refuses "a YAML
  anchor or alias as the `run:` value, since no anchor resolution is attempted";
  the implementation's refusal arms cover only the folded scalar, the explicit
  indent indicator, the alias sigil, and the unbalanced GitHub expression. There
  is no anchor arm, so `run: &anchor |` matches none of them, is non-empty, and
  falls through to the plain-scalar counter — its body lines are never linted.
  Re-verified at this close by execution, three ways over one identical body: the
  literal `run: |` form reds (SC2034 + SC2154), the alias form refuses loudly, and
  the anchor form exits 0 reporting "0 run: block(s) linted, 1 plain-scalar run:
  value(s) skipped". A fail-open of the same vacuous-pass class
  `vacuous-assertion-count-discipline` tracks, and the skipped-count line is
  exactly the diagnostic that made it visible.
  **Deliverable:** the missing code arm plus a `bad/` fixture over an anchored
  `run:` value. Explicitly **not** a narrowing of the SPEC line — the SPEC states
  the correct contract and narrowing it would bless the fail-open.
  **Cost while deferred:** a workflow author who anchors a `run:` value gets a
  green gate over unlinted bash, and nothing distinguishes that from a clean pass.
  No instance is in tree today, so the charge is latent rather than accruing.
  Debt: one refusal arm and one fixture in an existing gate; adds no governed name.
  Filed 2026-08-01 at close from the gap inbox, surfaced at this iteration's align
  audit while verifying an amendment claim about the gate's fail-closed set.

- **workflow-permissions-scope-oracle** [design-pending] — no gate parses a
  workflow `permissions:` block, so every scope a workflow needs is landed on
  reading alone. Re-verified exhaustively at this close: no check script matches
  `permissions:` at all, and every `permissions` hit across the tree is `jq` over
  the harness `settings.json`, an unrelated concept. The failure mode is concrete
  and this iteration supplied it: a `permissions:` block is an **allowlist**, an
  undeclared scope makes the read come back as an HTTP 404, and a 404 on a read is
  indistinguishable from an absent resource — the site-health release-body arm
  needs `contents: read` and would have reported "no such Release" without it.
  On a public repository the omission stays invisible until a private-repo
  consumer copies the workflow.
  **Precedent, stated precisely** (the gap's own framing overstated it):
  gate-sdk/SPEC.md declares **GitHub-expression injection** a non-goal and defers
  it to "a dedicated workflow-security linter". It does *not* rule the whole
  workflow-security category out of scope, so that line is supporting precedent
  for keeping the gate narrow, never a standing exclusion to argue against.
  **Deliverable:** a gate far narrower than a security linter — a workflow job
  whose `run:` bodies invoke `gh`, or that checks out, must declare the scopes
  those calls consume, starting with `contents: read`. Tree-local, hermetic,
  cheap: one gate, a `good/`+`bad/` fixture pair, a `gates.list` row, a graph
  manifest.
  **Cost while deferred:** charged against every workflow that gains an API call;
  the detector today is a 404 read as a missing object, which is the exact
  misreading `release-credential-precondition-scope-vs-permission` was filed for.
  Debt: one gate plus fixtures; adds one governed gate name when it lands.
  Filed 2026-08-01 at close from the gap inbox, confirmed at this iteration's
  align audit against all nine gates that read workflow YAML.

- **template-copy-parity-yaml-widening** [design-pending] — a kit `.yml` template
  and this repository's copy of it are mirrored by hand and a missed half is caught
  by nothing. Re-verified at this close: `check-template-copy-parity` globs
  `*/templates/*.sh` against the gates dir — three pairs in this tree, none of them
  YAML — and no other gate compares two hand-maintained copies of anything. Every
  other parity or freshness gate compares a **generated** projection against its
  source, which is a different problem. The duplication cannot be removed, because
  kit-template-plus-consumer-copy *is* the distribution model, so enforcement-first
  prefers a remedy that is unavailable here and gating is the fallback — and the
  fallback is absent.
  **Why byte parity is the wrong assertion.** The two `site-health.yml` files
  diverge today across eight hunks, and the divergence is largely *correct*:
  `ALT_DOMAIN` is `alt.example.com` in the template and this repo's own host in the
  copy, and the template's "copy verbatim, then set or delete this arm" adoption
  comments have no place in an instance. The shape that fits is the declared-
  divergence contract `check-template-copy-parity` already implements for `.sh`,
  widened from that extension to a registered template/copy pair of any extension —
  which also means the copy side stops being hard-wired to the gates dir.
  **Cost while deferred:** charged against every future edit to either copy. This
  iteration widened the exposure rather than creating it — the release-body arm
  adds a second hand-mirrored block to the same pair, and its own amendment stated
  outright that nothing catches a missed half.
  Debt: one existing gate widened plus a pair registry; adds one consumer knob.
  Filed 2026-08-01 at close from the gap inbox, confirmed at this iteration's align
  audit against the full gate roster.

- **gate-tests-suite-identity-in-evidence** [design-pending] — the validate
  manifest's per-suite hash cannot tell two suites apart, so it certifies that
  *some* suite ran clean rather than that *this* one did. evidence-kit's
  `run-validate.sh` hashes each suite's captured stdout+stderr as the manifest's
  identity check, but the hash carries no suite name or fingerprint of its own — it
  is only as discriminating as the wrapped tool's output, and
  `gate-sdk/bin/run-gate-tests.sh` ends with `GATE-TESTS: clean ($pairs pairs,
  $unit unit tests)`, naming neither the kit nor the files it ran. Two kits with
  matching counts therefore produce byte-identical logs and identical hashes.
  Confirmed live this iteration and re-verified at close: `evidence_kit` and
  `site_kit` both carry
  `sha256=1065526da8cb11a7fd6176adfde255374b9bcb95a8240a6591ea47fa43367c6d` in the
  committed evidence, reproduced across independent re-runs. Verified
  genuine-by-construction rather than a mis-wire — the two suites resolve distinct
  commands against their own correct kit directories and genuinely have matching
  counts (2 pairs, 3 unit tests each).
  **Why `[design-pending]`:** two fixes with different reach. Having
  `run-gate-tests.sh` name its kit in the success line sharpens the hash for free
  and helps every log reader, but it fixes only this producer; hashing something
  suite-identifying (the invoked command) alongside the log bytes fixes the
  manifest for every wrapped tool, including ones this repo does not own. They are
  not exclusive and the second is the one that generalizes.
  **Cost while deferred:** the manifest is weak evidence exactly where it is
  load-bearing — it cannot separate "suite X's correct result" from "some other
  suite emitting the same generic message", so a silently-skipped or mis-pointed
  suite reads as certified. Charged per validate run.
  Debt: a success-line change and/or a hash-input change; adds no governed name.
  Filed 2026-08-01 at close from the gap inbox, filed by this iteration's validate.

- **always-loaded-brevity-reach** [design-pending] — `check-brevity` guards the
  tidiest section of the always-loaded surface while the section that actually
  grows is outside its reach, so the tier ratchets with no oracle. The gate's
  target is a single named section (`CONTEXT_KIT_BREVITY_SECTION`, this repo's
  conventions block, 17 lines); §Housekeeping is 59 of `CLAUDE.md`'s 189 lines —
  31% of the surface — and no gate reads it. Measured at this close: the
  always-loaded meter reads 197 lines against a committed baseline of 172,
  **+25 (+14.5%)**, and the baseline stamp dates to `v0.8.0`, twelve releases
  back. Every close's brevity pass is therefore the only detector, which is a
  session's attention standing in for a gate — the shape enforcement-first exists
  to refuse.
  **Deliberately not reset.** Re-stamping the baseline at this close would clear
  the drift signal by blessing the growth, which is the one thing the baseline is
  for. It stays where it is until the ratchet is actually addressed.
  **What this close found and left.** Four bullets carry a doc pointer and then
  restate what is behind it — the `.tmp/`/`.metric/`/`.workflow/` bullet (13
  lines, mostly mechanism the owning SPECs own, with only the `*.local.md` roster
  genuinely resident-worthy), the `installer/` bullet (8 lines naming
  `installer/README.md` as layout owner then carrying packing mechanics and a
  scratch knob), the `demo/` bullet (compresses a `README.md` paragraph), and
  `reserve/` (2 lines guarding a mistake no session is near). Each needs content
  *relocated*, not merely deleted, which is unit-shaped rather than pass-shaped —
  filed here rather than done under release pressure.
  **Why `[design-pending]`:** pointing the gate at the whole file needs a budget
  model it does not have. A single whole-file cap either strangles a legitimately
  dense file or is set so loose it never fires; per-section budgets need a section
  roster that is itself maintained. The candidate worth costing first is a
  **ratchet** assertion rather than a cap — red when the surface grows against its
  committed baseline without the baseline being deliberately re-stamped — which
  needs no budget at all and makes the growth visible where it happens.
  **Cost while deferred:** compounding directly in the tier the whole methodology
  is trying to hold down, and paid by every session in this repo and every
  consumer that vendors context-kit. The detector is a close-stage read, so it is
  as reliable as the attention of whoever runs it.
  Debt: one gate widened or one assertion added; adds one knob if a budget lands.
  Filed 2026-08-01 by close's brevity pass, which measured the ratchet it could
  not close.

- **batch-split-stamp-ownership** [design-pending] — who stamps the per-session
  audit trail when a live lead splits one stage across several batch sessions is
  unowned. This iteration recorded **one** `build` line in
  `.workflow/WORKFLOW-STATE.txt` for five batch sessions.
  **Premise corrected 2026-08-01 by the undirected scope survey — the filed
  diagnosis was wrong and the next reader would otherwise re-derive it.** The
  entry said "the two surfaces disagree in effect", with
  `lifecycle-kit/templates/stages/build.md`'s "Every session still stamps"
  paragraph against a lead that "tells batches 2..n not to re-stamp". **The
  surfaces agree.** build.md:45 does say every build session re-runs
  `enter-stage.sh build`; `lifecycle-kit/templates/lead.md`:226-230 says batches
  are "N sibling stage sessions … each entering through `enter-stage.sh` as a
  same-stage re-entry … each leaving its own stamp and the cursor staying put" —
  the same instruction, not its opposite, and present since 2026-07-17, two weeks
  before the divergence. So the gap is **practice against instruction**, not
  instruction against instruction: the batches simply did not run the entry.
  Nor is the id the obstacle — dispatched stage sessions do resolve distinct
  session ids (this survey's own stamp differs from its lead's), so
  `bin/session-id.sh` was never the blocker.
  **Not a defect today.** Nothing gates on the missing lines, and the stage
  cursor is the *last* stamp, so it is correct either way — this is doctrine
  drift, not breakage.
  **The ruling the entry wants,** restated against the corrected diagnosis: is a
  per-session trail worth restoring at all — the evidence file's stated contract
  is one line per stage-skill invocation, which a batch session *is*, so the fork
  is stamping per batch or narrowing the contract to per stage. If per batch, the
  second half is what makes an instruction both templates already carry actually
  bind, since prose alone demonstrably did not.
  **Why `[design-pending]`:** it either narrows a shipped stamp contract and
  `check-stage-evidence`'s reading of it, or adds an oracle where two templates
  now rely on a dispatched session's compliance; and it recurs on every batched
  stage, not just build.
  **Cost while deferred:** the trail silently under-reports session count on
  every batched stage, so the evidence file cannot answer "how many sessions did
  this stage take" — an economics question the drift KPIs would otherwise want.
  Debt: one contract narrowed, or one oracle added; adds no governed name unless
  the oracle lands.
  Filed 2026-08-01 at close from the gap inbox, filed by this iteration's build.

- **gate-spawn-hoist-residual** [design-pending] — `gate-battery-spawn-hoists`
  closed most of its worklist and left a named remainder, recorded so the next
  pass starts from measurement rather than from this list.
  **Unmeasured, same shape:** `check-comment-tier`, `check-trajectory-fresh`,
  `check-docs-cmd`, and `check-docs-nav-reachable` all carry the in-gate
  fork/exec-per-item shape the landed hoists removed elsewhere, but were neither
  measured nor hoisted (the pass was time-boxed after the near-miss below).
  **Attempted and reverted:** `check-spec-pointer`'s `heading_present`. A
  bash-extglob port of its strippar/isprefix awk match logic was byte-identical
  to the awk original on small inputs and **pathologically slow** on this tree's
  real prose fragments (several thousand characters) — bash's extglob `%%`
  suffix match has none of awk's linear-time regex guarantees, turning a 2.7s
  gate into 25s+. Only the tracked-file-set caching landed (safe: no
  size-dependent blowup).
  **Deliverable:** per-target-file batching of the frag queries that keeps the
  match logic inside **one awk process per unique file** — not a bash port. The
  reverted attempt is the standing evidence that a bash port is the wrong shape.
  **Why `[design-pending]`:** the worklist above is already demonstrably stale
  twice over, so the unit begins by re-measuring against the post-hoist battery
  (23706ms/90 gates, `check-shellcheck` now the largest line at ~5.9s) and may
  find the remaining four are not worth the change.
  **Cost while deferred:** bounded and non-rotting — the gates are correct, just
  slower than they need to be, and the battery is already 42% faster than it was.
  Debt: awk-batching inside up to five gates; adds no governed name.
  Filed 2026-08-01 at close from the gap inbox, by the batch that time-boxed it.

- **template-spec-restatement-reach** [design-pending] — the gap generalization
  behind this close's resume-journal fix: `check-shim-restatement` holds
  **binding shims** (`.claude/commands/*.md`) to an n-gram-disjointness contract
  against a dedup corpus, and nothing holds the far heavier **template ↔ owning
  SPEC** pair to anything. Attested cost: `delegation-kit/SPEC.md` §Resume
  journal and `templates/agent-execution.md`'s resume-journal bullet drifted into
  near sentence-for-sentence restatement, and one build unit wrote the retention
  resolution and the `DONE`-as-last-line clause into **both** in parallel because
  no oracle held them to one owner. Close deduplicated them by hand; nothing
  stops the next lifetime edit doing the same thing again.
  **The mechanism already exists.** `lifecycle-kit/checks/check-shim-restatement.sh`
  is the whole implementation — normalize, emit every N-word window, intersect
  against a corpus index. The unit is a second (surface, corpus) pairing of that
  same machine, not new code.
  **Why `[design-pending]`, and the honest objection.** The shim contract is
  *bind consumer residue, cite kit-owned procedure* — a shim legitimately
  restates nothing. A SPEC legitimately restates its template's **contract
  clauses**, because a consumer reading the SPEC alone must learn what the kit
  requires. So a naive port reds on correct text, and the design question is what
  the exemption is: an explicit contract block the gate skips, a higher `N` for
  this pairing, or a direction-sensitive rule (the SPEC may cite the template,
  never the reverse). Answering that is the unit; porting the gate is an
  afternoon.
  **Cost while deferred:** paid per lifetime edit to any kit whose SPEC and
  template both discuss the same rule, and paid as a *silent* two-surface edit —
  the failure mode is a one-surface edit nobody catches, i.e. exactly the
  contradiction that build unit had to resolve.
  Debt: one gate widened or forked plus its fixture pair; adds one knob if the
  exemption is configurable.
  Filed 2026-08-01 by close, generalizing the gap it fixed inline.

- **guard-command-prefix-wrapper** [design-pending] — a transparent prefix
  displaces the token the guard matches on, so an already-allowlisted read-only
  command prompts anyway. Two shapes, one mechanism, measured at this close's
  prompt-friction triage as **35 of 108 prompting calls (~32%) — the largest
  class by a wide margin**: `time bash <allowlisted-script>` and
  `/usr/bin/time -f '<fmt>' bash <allowlisted-script>` (31), where the leading
  token is `time` rather than `bash`; and `git -c core.pager=cat <subcommand>`
  (4+), where the read-only-subcommand recognition reads `-c` instead of
  `status`/`log`/`tag`.
  **Why a guard rule and not an allowlist entry** — the triage criterion's own
  test. `Bash(time *)` would grant *anything* under `time`, which is the guard
  defeated rather than configured; the decision needs logic no static glob can
  express (strip the wrapper, then re-test the *wrapped* command against the
  committed allowlist with the matcher the guard already has).
  **Deliverable:** a generic-ruleset rule in `guard-kit/lib/guard.sh` that
  strips a recognized transparent prefix and re-tests via `guard_allow_match`,
  plus a `guard-tests/cases.tsv` pair and its SPEC rule entry. `scan-prompts.sh`
  already has half of it — its `strip_prefix()` handles `sudo `/`timeout ` for
  ranking purposes only, so the live hook and the scanner disagree about what is
  covered, which is its own small defect.
  **Why `[design-pending]`:** the wrapper roster is the design question. `time`
  and `git -c` are safe because they are transparent, but `env VAR=v <cmd>`,
  `nice`, `nohup`, and `xargs` are not uniformly so (`env` can replace `PATH`),
  so the rule needs a stated closed roster and a reason for its boundary rather
  than a "strip anything that looks like a wrapper" heuristic.
  **Cost while deferred:** roughly a third of all permission prompts in an
  iteration, all of them on commands the operator already blessed — the pure
  interruption cost this loop exists to retire, and it grows with profiling work
  (this iteration was a performance iteration, which is why `time` dominated).
  Debt: one guard rule plus a fixture pair and one roster; adds no governed name
  to a shipped surface.
  Filed 2026-08-01 by close's prompt-friction triage.

## Icebox

  Dormant entries, one line each: the cost field said the carry was low, no
  `[roadmap:]` commitment rides on it, and no named event is waiting to
  promote it. Still live work — a legal `[blocked-by:]` target, conserved on
  the way in and on the way back out. The removed body is recoverable from
  the evicting commit (`git log -p -S'<slug>' -- TASK-QUEUE.md`).

- **runtime-dir-two-tier-detector** [design-pending] — No two-tier proof for file-pattern ignores.
- **done-slug-commit-naming-gate** [design-pending] — Done-moving commits need not name their slug.
- **enter-stage-simulate-no-write-fixture** [design-pending] — Guard present, unpinned by a fixture.
- **stage-lag-disambiguation** [design-pending] — Hook over-firing is accepted, not a defect.
- **stage-economics-smoke-jq-arm-dormant** [design-pending] — Its jq-absent arm never runs anywhere.
- **hermetic-bin-roster-config** [design-pending] — Pinning coverage needs a consumer roster seam.
- **split-posture-waiver-writer** [design-pending] — A lead-issued waiver stamp has no writer.
- **supervisor-verification-attestation** [design-pending] — The verification duty is unattested.
- **assertion-strength-exit-header-reach** [design-pending] — Wider header reach may be ceremony.
- **gate-spec-claim-assertion-parity** [design-pending] — Ruled a human-audit class, not gateable.
- **upgrade-smoke-phase-a-regen-derivation** [design-pending] — A hand-held regen roster; rot-prone.
- **scope-amendment-authoring-gate** [design-pending] — Scope can do spec's job and stay green.
- **exit-echo-decoration-guard-vs-habit** [design-pending] — Prompt friction only, nothing degrades.
- **evidence-journal-hash-chain** [design-pending] — Tamper-evidence wanted only by a hosted rung.
- **md-section-near-miss-match** [design-pending] — Empty on a near miss; correct on an exact query.
- **amendment-update-target-coverage** [design-pending] — Align checks it by hand; no gate yet.
- **gap-inbox-commit-ownership** [design-pending] — Who commits a lead-filed bullet is unspecified.
- **evidence-row-upsert-order** [design-pending] — A re-run relocates the row; no content is wrong.
- **operator-authored-unit-set** [design-pending] — The contract omits operator-authored unit sets.
- **tarball-build-attestation** [design-pending] — The checksum proves transfer only; docs agree.
- **action-run-shell-scan-predicate** [design-pending] — No consumer seam on a correct gate.
- **scratch-execution-allowlist-bar** [design-pending] — Each close re-derives this standing bar.
- **capture-affordance-help-flag** [design-pending] — file-gap.sh files --help as a gap.

## Done

## Lessons Learned
