# TASK-QUEUE.md — Checkwright work queue

## Iteration: pre-adoption-grammar-break

  The lifecycle-kit gates read this header's iteration name and the stage
  cursor — the last stamp in `.workflow/WORKFLOW-STATE.txt`
  (lifecycle-kit/SPEC.md §The state machine); queue-kit formalizes the queue
  format itself and gates this file. One iteration per hardening or roadmap
  unit; [README.md](README.md) maps the kits.

---

## New Features

## Technical Debt

- **queue-selection-order-implicit** — `queue-kit/SPEC.md`
  documents section order as selection order, so the default section sequence
  silently makes `New Features` outrank `Technical Debt` in what scope picks
  first. Unlike the spec-tag requirement that `CANON_KIT_FEATURE_SECTIONS`
  places on `New Features` alone, which is principled and argued, this ordering
  policy is embedded in section sequence with no stated argument anywhere.
  Either state the argument or make selection order explicit rather than
  positional. Surfaced while ruling on whether the two active sections should
  collapse into one — they should not, but this rides along on their sequence.
  Debt: latent policy with no owner doc.
  **Rides `pre-adoption-grammar-break` as the rider, ruled 2026-07-29 at scope.**
  Same owner doc as the iteration's size-cap and icebox work, so the argument
  lands in a section the sibling amendments are already opening. If the
  resolution mints an explicit ordering declaration rather than stating the
  argument in prose, that is a governed name and the unit is misfiled here —
  escalate rather than minting it from the debt lane.
  Filed 2026-07-20 by lead while ruling on the active-section question;
  promoted 2026-07-29 at scope as this iteration's rider.

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

- **release-body-url-form** [design-pending] — *residue only: the two cheap
  deliverables are done (see below); what remains is the monitor-shaped half,
  promotable only together with `rendered-site-link-monitor`.* The `v0.6.0`
  GitHub Release body linked
  `https://checkwright.dev/posts/2026-07-18-checkwright-v0-6-0/` **with a trailing
  slash, which 404s** — the site serves posts without one. Confirmed 2026-07-19:
  the no-slash form returns 200, and `v0.4.0`/`v0.5.0` both use the correct form,
  so this is a one-off regression in the most recent release rather than a
  long-standing convention error. `v0.7.0` hit the same trap while being cut and
  was corrected before this entry was filed, which is the evidence that the trap
  is live rather than historical.
  **Two deliverables — both DONE 2026-07-19, operator-authorized out of stage**
  (the entry's own "pinning the form in the runbook is the cheap half and stands
  alone" is the authority for landing them without a scoped iteration):
  (a) the `v0.6.0` body is repaired — a one-line `gh release edit`, deliberately
  *not* done while closing `derivation-by-precedent` because editing a previously
  published release is outside that iteration's envelope and the envelope had
  already been widened once by ruling; re-verified 200 after the edit, and the
  note's prose is byte-identical apart from the URL. `v0.4.0`/`v0.5.0` re-checked
  and already correct, so no other published body carries the trap.
  (b) RELEASING.md step 5 now pins the no-slash form and makes opening the link a
  named verification; it previously said only that the body "points at the post's
  `https://checkwright.dev/` URL", and that ambiguity is what both slips came
  through.
  **Gap generalization — why no gate.** A release body lives on the host, not in
  the tree, so the battery cannot reach it; and the link is an external URL whose
  liveness reds on causes no commit produced, which site-kit/SPEC.md §The monitor
  boundary already rules out of gate shape on the low-false-positive contract. So
  the durable form is **monitor-shaped, not gate-shaped** — the same disposition
  and the same reasoning as `rendered-site-link-monitor`, over a different surface
  (links *in release bodies pointing at* the site, versus links *on* the rendered
  site). Promote the two together if either earns automation; pinning the form in
  the runbook is the cheap half and stands alone.
  **Cost while deferred:** one dead link in one published release body, plus a
  recurring per-release chance of repeating it until the runbook pins the form.
  Surfaced 2026-07-19 by the close-stage release step for `derivation-by-precedent`,
  verifying that the URL its own release body advertises actually resolves.
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
  wrong: artifact state. A lead checked that validate's commit had landed with
  complete evidence, the tree was clean, the battery green, and a simulated close
  entry cleared — then dispatched close into a still-running `run-validate`. Every
  check passed while the producer was mid-write, because `run-validate` commits
  its evidence and keeps going: **the terminal commit existing is fully compatible
  with the process still executing.** The lead dispatched on the *absence* of a
  completion notification rather than its arrival.
  **The generalizable trap, worth the entry on its own.** The lead read an
  operator's note about having just answered a stalled permission prompt as the
  stage being finished. It means the opposite: an approval prompt gates a command
  **starting**. Any signal about a prompt being answered is a start signal, never
  a completion one.
  **Deliverable:** a dispatch precondition in
  `lifecycle-kit/templates/lead.md` — stage N+1 is dispatched on stage N's agent
  **completion notification**, never on the presence of its commit or any
  tree-state check, because artifact state cannot distinguish "finished" from
  "still writing." It belongs beside the existing post-delegation verify
  discipline, which specifies what to check *after* a stage and is silent on how
  the lead knows the stage is over.
  **Assertability — checked, not assumed.** The precondition is **prose-only**.
  The notification is harness session state with no tracked artifact, so no
  battery gate can read it; the precedent is the sibling-dispatch clause, prose in
  the same template for the same reason. But the *negative* is assertable, and
  that is the pairing that matters: "did the lead get a notification?" is
  unreadable, while "is the producer still running?" is exactly what the lock
  sentinel under `validate-producer-liveness-unobservable` reads. The two are one
  unit — prose rule on the dispatch side, oracle on the artifact side.
  **Disposition of the incident as a set.** This rule is the proximate cause and
  the cheapest fix, so it goes first — but it is not sufficient alone, and the
  queue carries the reason: `validate-verb-collision-and-check-routing` establishes
  that a prose fix with no oracle recurs and is caught only by an operator.
  Shipping this without the sentinel repeats that pattern knowingly. Three real
  findings, not four: the dispatch decision (this entry), the observability gap
  (the sentinel), and the artifact churn (`evidence-row-upsert-order`), which is
  worth fixing on its own merits rather than as a symptom. A fourth reading — that
  `check-evidence-manifest` is grammar-only — is retracted; see the assertion-A
  note under `validate-producer-liveness-unobservable`.
  **Why `[design-pending]`:** it adds a precondition to a shipped template's dispatch
  contract and states a limit (prose-only, human-enforced) that
  lifecycle-kit/SPEC.md should own explicitly rather than leave implied.
  **Cost while deferred:** every multi-stage iteration with a live lead can
  re-run this race, and the cost lands on the next stage session, which fights a
  file changing underneath it. Debt: one template rule plus one SPEC limit; adds
  no governed name.
  Filed 2026-07-25 by close, from the lead's own account of the dispatch
  decision; the operator ruled stage sequencing the lead's accountability, which
  is why this is a lead rule rather than a stage-session or gate concern.

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
  **Cost while deferred:** compounding and silent — this recurs at **every
  amendment that measures the tree**, the failure mode is a canonical doc
  asserting a false number, and detection is by hand at align if at all.
  Filed 2026-07-26 by close (`release-path-hardening`), draining the
  stale-measured-count bullet; costed at roughly one small unit.

- **action-gh-repo-context** [design-pending] — a workflow job that invokes `gh`
  while carrying **neither a checkout nor a repo-context env** cannot resolve a
  target repository, and nothing catches it until a tag fires. This is not
  hypothetical: it took down `v0.17.0`'s `release` job on its first live run.
  `.github/workflows/publish.yml`'s release job downloads the packed artifact and
  deliberately checks out nothing (the job needs no source, and its own header
  argues that design), but set only `GH_TOKEN` and `TAG` — so `gh` fell back to
  resolving the repo from a git remote that was not there and died in 7s on
  `failed to run git: fatal: not a git repository`, before its first API call.
  No Release was created and no assets were attached.
  **Gap generalization — the class is real and ShellCheck is structurally blind
  to it.** `check-action-run-shell`, which this same iteration shipped, lints
  exactly this block and passes it: the shell is valid, the variables are quoted,
  the control flow is sound. The defect is **semantic** — an assumption about the
  runner's filesystem that no syntactic linter can hold. So the gate that exists
  is not the gate this needed, and that distinction is the entry's whole point.
  **Deliverable:** an assertion that a job whose `run:` blocks invoke `gh` either
  contains a checkout step, sets `GH_REPO` (job- or step-level), or passes
  `--repo` on every `gh` invocation. All three inputs are readable from the
  workflow text, and the trigger is narrow — only jobs that actually call `gh`
  arm it.
  **Why `[design-pending]`:** placement and reach are undecided. It reads the same
  Actions-shape surface as `check-action-run-shell` and so probably belongs in
  gate-sdk, but that makes it consumer-reachable — which means a fixture pair, a
  SPEC section, `gates.list` registration, three regenerated projections, and a
  Tightened-gates bullet in the next note. The `--repo`-on-every-call arm is also
  the fiddly one: proving *every* invocation carries it is a per-line read, and a
  job mixing prefixed and unprefixed calls is the false-negative to design out.
  **Cost while deferred:** the concrete instance is fixed (the release job now
  sets `GH_REPO`, bound by a comment to the no-checkout design), so what stays
  open is the class — any future `gh`-using job repeats it, and the failure mode
  is the worst-timed one available: green everywhere, red only at the tag, on the
  release path itself. Costed at roughly one small unit.
  Filed 2026-07-27 by close (`release-path-hardening`), from the v0.17.0 release
  job's first live failure; ruled a unit rather than a close-step patch, since
  landing a consumer-reachable gate after validate has passed is the failure mode
  this iteration exists to fix.

- **native-gate-binary-port** [design-pending] [roadmap: next/reliability] — a new gate substrate.
  roadmap-summary: The gate battery as one native binary: no GNU userland, sub-second runs.
  Port the battery off bash-plus-GNU-userland onto a single native compiled
  binary (Rust the lead candidate), because every structural pain the current
  stack carries is substrate-bound, not fixable in place. Scale is read off
  `gates.list` and `*/checks`, never restated here. The pains, each already
  live: the platform reality is Linux-only — stock macOS ships bash 3.2 and a
  BSD userland (docs/install.md §Requirements states it outright) and Windows is
  WSL-only (`platform-support-ci-matrix` carries that half); the
  bash/git/jq/awk/sort/shellcheck assortment has independent release lifecycles,
  with cross-version workarounds already in tree; battery wall-time has grown
  toward minutes, most of it process-spawn cost rather than work; shellcheck is
  a linter, not a compiler, and bash offers no type system at this size; and
  check source in the consumer's tree feeds the source-prediction anti-pattern —
  agents reading gate scripts to predict verdicts instead of running the oracle,
  a token sink the delegation rules fight behaviorally rather than structurally.
  **Deliverable:** one multi-call binary (busybox-style, one subcommand per
  check), with `gates.list` resolution dispatching per-entry to binary
  subcommand or script so the port lands cohort by cohort, slowest and
  meta-gates first; each ported gate's `good/`+`bad/` fixture pair is the
  mechanical parity oracle before its script retires; consumer-authored gates
  keep the shell escape hatch, so registry-plus-shadowing semantics survive
  verbatim; per-platform release artifacts (Linux, macOS, native Windows)
  with published checksums and publicly buildable source; git the sole
  runtime dependency (shelled out, not embedded).
  **Why `[design-pending]`:** the consumer-extensibility model is the design that
  decides everything else — script escape hatch as first-class vs a
  declarative check DSL vs native plugins — plus language choice (Rust vs
  Go), the dogfood question (this repo must run built artifacts or the
  opacity win is consumer-only, and the Rust source sits readable in-tree
  regardless), the trust inversion (an auditable script becoming an opaque
  binary in a pre-commit hook demands reproducible builds and checksums —
  opacity to agents is opacity to human adopters too), and the
  cross-compile/release pipeline including how `pack-installer` ships
  per-platform payloads. A multi-iteration track: cost the design spike
  separately from the port, since the spike decides whether the port is a
  cohort per iteration or worse.
  **Cost while deferred:** compounding on three axes — every new gate adds
  shell to the eventual port, battery duration grows with the roster and is
  a per-commit tax already, and the source-prediction token waste recurs per
  session. Bounded in kind: nothing breaks; the current substrate is correct
  on Linux and fully gated. Interacts with `platform-support-ci-matrix`
  (native binaries change what a platform leg must prove — the binary runs,
  not that a GNU floor is met) and with the front-door requirements story
  (one static binary is a stronger install claim than the utility list).
  Feature-shaped at triage: it adds governed names — the binary, its
  subcommand surface, the dispatch knob — on top of new mechanism.
  Filed 2026-07-28 by operator request, from a session assessing the shell
  substrate's structural limits against a native-binary port.

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
  Filed 2026-07-27 at align in front-door-readiness, while verifying the
  roadmap amendment's docs-home link; re-verified at close.

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

- root-page-render-coverage
- needs-spec-tag-rename
- deferred-queue-carry-cost
- templates-stages-taxonomy-realignment
- queue-index-title-tag-residue

## Lessons Learned
