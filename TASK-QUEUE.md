# TASK-QUEUE.md — Checkwright work queue

## Iteration: render-fidelity-leak-coverage

  The lifecycle-kit gates read this header's iteration name and the stage
  cursor — the last stamp in `.workflow/WORKFLOW-STATE.txt`
  (lifecycle-kit/SPEC.md §The state machine); queue-kit formalizes the queue
  format itself and gates this file. One iteration per hardening or roadmap
  unit; [README.md](README.md) maps the kits.

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
  The experiment's measurement half — per-stage, per-model, price-weighted
  token burn off harness transcripts — is the stage-economics-report tool
  filed above; this rung consumes it rather than rebuilding it. Nearer use of
  that tool: verifying the split-lead posture's savings
  (lifecycle-kit/templates/lead.md §Economics). Surfaced 2026-07-15 by the
  per-stage budget analysis that motivated that posture.
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
  **The manual fix is demonstrably incomplete — an eighth site surfaced
  2026-07-19.** The `tooling-signal-honesty` close audit swept the class
  mechanically and found `DELEGATION_KIT_STALE_AGE` cited as bare `STALE_AGE` at
  delegation-kit/SPEC.md:200, a survivor the config-seam-hardening pass missed
  while correctly fixing seven neighbours in the same file (the §Layout roster two
  hundred lines below had the prefixed spelling right the whole time). Fixed at
  that close. This is the demand evidence the entry was waiting on: a hand sweep
  of one file missed one instance in eight, so the audit cadence is catching
  what review does not — and a ~20-line scan expressed the whole class, which is
  itself evidence the low-false-positive boundary is tighter than feared (one hit,
  zero false positives, across every kit SPEC in the tree).
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

- **stage-lag-disambiguation** [needs-spec] —
  narrow the session-context hook's accepted over-firing by distinguishing a
  first-of-stage session from a restarted predecessor session, using the session
  id the stage-cursor migration puts in the state file's last stamp.
  **The rule was never wrong, only wrongly argued.** context-kit's cursor-lag
  rule (context-kit/SPEC.md §The session-context hook) already survived the
  stage-cursor migration on its *lag* ground: the hook fires at session start,
  before the arriving skill's
  first step writes the cursor, so a first-of-stage session still reads the
  predecessor's value. What the migration retired is the rule's former stated
  **justification** — "no header value distinguishes [a first-of-stage session]
  from a restarted predecessor session" — and the over-firing cost it accepted
  on that basis. The header carried one value; the state file's last stamp
  carries `<iter> <stage> <session-id> <date>`, so the id distinguishes exactly
  the two cases the rule says are indistinguishable: ids match = restarted
  session of the stamped stage, ids differ = new session whose stage has not
  stamped yet. Found by the stage-cursor-extraction align audit; deliberately
  left out of that amendment's envelope (a behavior widening coupled to a
  mechanical migration muddies what its fixture proves).
  **Cost — larger than the comparison it looks like.** The session id *is*
  reachable from the hook payload, so the premise holds: the session-role
  signal already reads it (`scripts/session-context.sh`, spec at
  context-kit/SPEC.md §The session-context hook), and the stamp's 8-char id
  already matches the
  `${hook_sid:0:8}` comparison shape that code uses. But that read sits inside
  the `[[ -f "$ROLE_FILE" && ! -t 0 ]]` guard, and **stdin is consumable
  exactly once** — so the payload read must be hoisted to the unconditional
  path before a stage derivation can use it. (The stage-cursor migration added
  a *named-file* cursor read ahead of that guard, deliberately consuming no
  stdin, so it left this hoist untouched.) The hoist is the real work, and it
  lands on a hook whose contract is "never fails a session" and "signal absent =
  byte-identical output": a read that today happens only for lead-marked
  sessions would happen every fire. Note the audience inversion that makes the
  hoist unavoidable rather than incidental — the sessions this entry serves are
  stage sessions, which carry *no* role marker, i.e. precisely the case the
  current guard skips. Scope: the hoist plus its no-payload/`-t 0` fallbacks,
  the comparison, and a fixture pair proving both the first-of-stage and
  restarted-predecessor cases; both kit template and consumer copy.
  **Cost while deferred:** low and non-rotting — the over-firing is the
  documented accepted cost, not a defect, and the rule stays correct on its lag
  ground. Unblocked: `stage-cursor-extraction` has reached Done, so this entry
  is pickable.

- **heterogeneous-agent-delegation** [needs-spec] — cross-vendor stage dispatch:
  a Claude Code lead delegating a stage (e.g. `/build`) to a foreign coding agent
  (Codex, etc.), extending the homogeneous multi-agent / multi-operator model to a
  heterogeneous fleet. Cashes the public "no IDE/model/harness lock-in"
  positioning claim, and is the purest expression of the thesis — governance
  enforced at the git/gate boundary, not by trusting the author. It splits along a
  **two-substrate line**. *Already agent-neutral:* the verification substrate (git
  + the gate battery + the bash stamp state machine) does not care who
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

- **stage-economics-smoke-jq-arm-dormant** [needs-spec] — drift-kit's smoke
  asserts the jq-absent degradation of `bin/stage-economics.sh`, but the
  assertion never executes. `drift-kit/smoke/install.sh:171` branches on the
  **host's** jq (`command -v jq`), so the degradation arm at `:189` — "without
  jq must emit its degradation notice" — is reached only on a jq-less machine.
  Neither this machine nor CI is one (the gates workflow image carries jq), so
  the arm is **dormant on every runner that actually runs it**: a correct
  assertion that no run has ever evaluated. **This is a testability gap, not a
  defect** — the stage-economics-report validate verified the degradation by
  hand (jq masked → exit 0, `jq not found` notice, 0 rows logged), so the
  behavior is known-good; what is missing is the *automation* of that check.
  Fix direction: exercise the arm unconditionally rather than conditionally —
  a second tool run under a PATH sandbox with jq masked, asserted alongside the
  jq-present run, so both arms evaluate on every host. **Scope: drift-kit's
  smoke only.** Deliberately not generalized to a smoke-authoring rule — one
  instance is not evidence for a general rule, and gate-sdk's own jq consumers
  do not assert their degradation at all, so a blanket rule would manufacture
  work against a pattern nobody has shown to be wrong.
  **Cost while deferred:** low and non-rotting. The asserted behavior was
  hand-verified at ship, and the arm cannot *false-green* anything — it is
  skipped, not passed. The real cost is narrow: a future regression in the
  jq-absent path (say a reordering that emits the notice after an unguarded
  `jq` call) lands unnoticed, because the only mechanism watching that path
  never runs. Bounded by the degradation path being small and rarely touched.
  Filed 2026-07-18 by lead ruling at the stage-economics-report close.

- **new-initiative-filing-default** [needs-spec] — no always-loaded surface
  states the rule that a **new initiative raised mid-session** (an operator
  feature request, a design idea) is **filed as a Deferred queue entry by
  default, not started** — it enters delivery only by passing through `/scope` as
  the active iteration's unit. The rule is *implicit* in the iteration model and
  partially covered for one case: doctrine-kit's **Gap disposition** bullet ("a
  gap you defer is costed and filed") and close's gap→promote step cover a *gap
  found while working*, and scope's "a standing directive is a theme, not a unit
  list" bounds the scope survey — but none states the general default for an
  operator-raised initiative. This session is the evidence: the assistant first
  offered to *run* the `economics-budget-pct-decouple` decouple as a live
  iteration rather than file it, and the operator had to redirect.
  **Design question (why deferred, not an inline one-liner):** placement is a
  widest-true-tier ruling — CLAUDE.md's iteration-lifecycle section, doctrine-kit's
  Gap-disposition bullet (widen its wording), or lifecycle-kit's SPEC — and
  codifying "don't start work out of band" by an *out-of-band edit to an
  always-loaded governance surface* is self-contradictory, so it earns the
  deliberate scope pass it prescribes.
  **Cost while deferred:** low but recurring — each session re-litigates whether
  an operator ask is filed or started, the ambiguity this session already paid.
  Surfaced 2026-07-19 alongside `economics-budget-pct-decouple` at the operator's
  direction.

- **hermetic-bin-roster-config** [needs-spec] — `check-test-hermetic` assertion
  B catches *partial* credential pinning but not *absent* pinning. B arms only
  when the smoke script itself contains a `*_CRED_FILE=` assignment, so a smoke
  script that calls a credential-consuming own-kit bin and pins nothing at all
  is never flagged. The narrow trigger was deliberate and correct — an
  unconditional every-own-bin-call-must-pin rule false-positives across the
  credential-free kits — so the hole is a known limit of the trigger, not a bug
  in it.
  **Design question (why deferred, not a build fix):** closing it needs a
  per-kit roster of credential-consuming bins, and that roster is exactly the
  content the provenance seam keeps out of a kit — it is optional consumer
  config (the `check-graph` / `scripts/graph-vocab.sh` pattern), never a
  gate-sdk literal. So the deliverable is a config seam and its fall-open
  default, an align-shaped design pass rather than an assertion tweak.
  **Cost while deferred:** low and bounded — the gate still catches the partial
  case that actually regressed here; the uncovered case is a kit shipping a
  credential-consuming smoke with no pin at all, which no kit does today.
  Cost to close: roughly one iteration. Surfaced 2026-07-19 by the validate
  re-entry on `derivation-by-precedent`, downstream of that iteration's
  operator-authorized hermeticity fix rather than of its precedent-doctrine
  envelope.

- **release-body-url-form** [needs-spec] — *residue only: the two cheap
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
- **enforcement-first-behavioral-regressions** [needs-spec] — the always-loaded
  enforcement-first rule ("the fix and the gate that catches it land in one unit;
  removing the duplication outranks gating it") anchors its second clause — and
  every neighbouring doctrine example (content-tiering, de-literalization) — in
  the SSOT/duplication domain, so it under-cues the incident→gate reflex for a
  *behavioral* regression that has nothing to do with duplicated content. Design
  shape: tighten the always-loaded line (or the doctrine-kit section behind it) to
  name the generalize-to-class-then-gate reflex for runtime/behavioral defects,
  not only duplication — a `doctrine-kit/DOCTRINE.md` change, re-vendored to
  upgrade, so it sits outside an incident-fix commit's envelope. **Cost while
  deferred:** each behavioral bug fixed in a maintenance turn risks shipping
  without its paired low-FP gate until a reviewer prompts. Surfaced 2026-07-19 by
  the check-graph `maxEdges` fix (the coupling graph outgrew Mermaid's 500-edge
  render cap): the one-line render fix landed, but the paired render-cap gate —
  exactly the low-FP gate enforcement-first says to land in the same unit — was
  added only on explicit request.

- **spec-split-promotion-review** [needs-spec] — after the six-stage roster has
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
  **Cost while deferred:** one queue line; the backlog-aging review re-raises it
  every iteration until the data exists to run it. Filed 2026-07-19 by lead
  ruling at the `stage-posture-split-tuning` close — the split shipped on
  projected economics, and this is the loop that confirms or retires that
  projection with recorded data.

- **split-posture-waiver-writer** [needs-spec] — in the split-lead posture the
  lead issues stage rulings (e.g. an align waiver) but commits no lifecycle
  state by design (the lead stamps nothing; only stage sessions write), so a
  lead-issued **waiver stamp** has no clean writer. This iteration's
  `trajectory-stage-roster-hardcode align-waived cbc94da8 2026-07-19` line was
  carried into the *build* session's entry commit (`6b22f38`) as a piggyback,
  because no stage owns the waiver and the lead cannot commit it. The protocol
  worked here only because the build session was told to carry it; nothing in
  the stage skills or lifecycle SPEC prescribes *which* stage commit a pending
  waiver rides, nor guarantees the carry happens at all.
  **Design question (why deferred, not fixed inline):** the fix is a protocol
  ruling on the seam, not a code tweak — options include (a) codify in the
  next-stage skill/entry contract that a stage session carries any pending
  lead-issued waiver into its entry commit (formalize the piggyback that
  happened), (b) a lead-invokable waiver-writer that appends the stamp without
  the lead becoming a lifecycle-state writer (tension with the split posture's
  "lead writes none" invariant), or (c) route the waiver through the same
  committed-worklist channel the dispatch uses. Choosing among these is a
  split-posture-vs-state-machine seam ruling that belongs in
  lifecycle-kit's SPEC / the lead template, so it earns a scope pass.
  **Cost while deferred:** low and non-rotting — the waiver *did* land correctly
  here and `check-stage-evidence` accepts the line, so the machine is not
  broken. The cost is recurrence risk: every future split-posture waiver
  re-litigates which stage commit carries it, and a stage session that stamps
  its entry without carrying a pending waiver would leave the waiver
  uncommitted — living only in the message channel, which is transport, never a
  store (CLAUDE.md §How to escalate). Surfaced 2026-07-19 by the
  `trajectory-stage-roster-hardcode` close, filed by lead-dispatch instruction.

- **supervisor-verification-attestation** [needs-spec] — the resume-journal
  recovery contract now rests on an **unattested** supervisor duty. The
  `resume-journal-done-marker-compliance` amendment (this iteration) rescoped the
  DONE-absence clause so that on the ordinary path "the supervisor consumed the
  agent's return and ran its post-commit verification (§Validate after every agent
  commit)" *is* the recovery contract, making the `DONE` marker redundant. That
  promotion is correct — DONE was false-reading completed runs as interrupted —
  but it moves the load onto a step nothing checks: no mechanism records that the
  supervisor actually ran the verification before deleting the journal, so a
  skipped verification is indistinguishable from a performed one.
  **Deliberately out of the amendment's envelope:** the operator ruled plain (b),
  which excluded marker enforcement and gate mechanization by name — the amendment
  fixes the *reading* of the contract, and mechanizing the *check* is separate
  work, filed rather than smuggled in.
  **Design question (why [needs-spec], not a build unit):** the verification
  happens **after** the commit it verifies, so the pre-commit battery cannot reach
  it — a gate is the wrong shape on ordering grounds alone, the same
  wrong-shape finding as `rendered-site-link-monitor`'s. The open design is what
  an attestation would even be: a supervisor-written stamp (another
  self-asserted-completion marker, which lifecycle-kit/SPEC.md §The stamp protocol
  rules out as proving a claim rather than completion), a next-dispatch preflight
  that refuses until the prior batch's verification is on record, or an honest
  ruling that the duty stays unmechanized and the SPEC's existing
  **Honest limit** paragraph is widened to say so. Note `check-gate-tamper`
  already mechanizes one slice (agent gate-edit *shape*) and its own text concedes
  the by-eye review remains a supervisor duty — so the precedent in-kit is a
  partial floor plus a stated limit, not full mechanization.
  **Cost while deferred:** low today, structurally rising. Under a single
  attentive supervisor the verification does happen (this iteration's every batch
  was verified); the exposure grows with dispatch volume and with the split-lead
  posture, where the lead accepting a stage session's return is exactly the
  unattested step. Bounded by the delete being idempotent and the journal being
  scratch — a missed verification costs a late-caught regression, never lost work.
  Filed 2026-07-19 by the `tooling-signal-honesty` close, as the follow-up the
  plain-(b) ruling named.

- **assertion-strength-exit-header-reach** [needs-spec] —
  `check-assertion-strength` is armed by callee `# exit:` headers, and **two
  scripts in the tree declare one** (`delegation-kit/bin/usage-verdict.sh`,
  `delegation-kit/bin/usage-trend.sh`; the other two hits are the gate's own
  fixtures). Of those, `usage-trend.sh` declares its codes in prose with no
  uppercase token, so it yields an **empty** token→code map — leaving
  `usage-verdict.sh`'s `PAUSE`→1 / `STALE`→2 as the gate's entire live
  vocabulary, over 2 call sites out of 54 scanned scripts. Recorded as an
  honest limit in gate-sdk/SPEC.md §check-assertion-strength.
  **What the close sweep changes about the premise.** The limit reads as though
  the reach were inherently narrow; it is narrow **by adoption**. A count of the
  bin roster found **29 of 64 scripts exit with a code >1**, i.e. carry a
  three-valued contract a header could declare. That is a materially larger
  candidate surface than "one script" suggests.
  **Design question (why [needs-spec], and why the value is the open part):**
  the mechanism is trivial — add `# exit:` headers across the bin roster. The
  unresolved question is whether it buys reach or ceremony. The gate fires only
  when a guard's *failure message names a verdict token*, and for most of the 29
  the code >1 arm is a generic error exit (usage error, unreadable input) rather
  than a named verdict a caller relays, as `usage-verdict.sh`'s is. Headers
  declaring `2 USAGE` / `2 UNREADABLE` would arm the gate against a smoke guard
  claiming "fails on unreadable input" while checking only truthiness — a
  plausible real defect class, but one nobody has shown to have occurred. So the
  design pass must establish the value before prescribing the convention;
  concluding "not worth it" and widening the SPEC's honest limit to say *why* is
  a legitimate outcome of this entry.
  **Cost while deferred:** low and non-rotting — the gate is correct and clean
  over what it reaches, and the SPEC states the limit rather than overclaiming.
  The cost is that each reader re-derives the adoption-vs-inherent distinction,
  as this close did. Filed 2026-07-20 by the `verdict-reader-honesty` close, by
  lead instruction.

- **scope-iteration-cost-bundling-test** [needs-spec] — the `scope` stage
  contract does not carry the **economic composition test**: weigh whether a
  single sub-threshold unit justifies a whole iteration, and either bundle
  related-surface deferred entries or argue the unit is significant enough to
  stand alone. `scope.md` today covers feature-vs-debt triage, theme-not-unit-list,
  and premise re-verification, but has no line making that iteration-cost weighing
  a scope duty — which is why it had to be relayed by hand at this iteration's
  open. The lead template's economics section
  (`lifecycle-kit/templates/lead.md` §Economics) already carries the underlying
  cost principle, so the deliverable **single-sources it by citation** — a scope
  contract line pointing at that section, never restating the economics — placed
  at the widest tier true for all lifecycle-kit consumers.
  Debt, not feature: it converges `scope.md` onto a principle a sibling template
  already owns and adds no governed name. `[needs-spec]` is for the placement
  ruling only (the scope-skill template line vs a lifecycle-kit SPEC section — a
  widest-true-tier call, the same shape as `new-initiative-filing-default`), not
  the principle, which lead.md §Economics settles.
  **Cost while deferred:** low but recurring — each undirected scope re-litigates
  by hand whether a lone small unit earns an iteration, the ambiguity this
  iteration's open already paid. Filed 2026-07-20 by lead instruction at this
  iteration's `scope`, during the render-fidelity bundling ruling.

- **gate-spec-claim-assertion-parity** [needs-spec] — the generalization the
  `render-fidelity-inline-span-leak` entry flagged as a deliberately-unsettled
  `/spec` question, ruled here: a gate whose **SPEC prose names a failure class
  its assertions do not implement**. This iteration's defect is the instance —
  `check-docs-render-fidelity`'s section described the severed-span defense
  (`gettalong/kramdown#843`) while its assertion matched only a multi-backtick
  fence run, so the prose claimed a defense the code omitted.
  **Ruling — the general class is a human-audit class, not gateable.** The two
  in-kit precedents that look adjacent both key on a **structured token**:
  `check-assertion-strength` reads callee `# exit:` codes and
  `check-gate-assertion-strength` reads the runtime **failure message** string.
  A SPEC's claimed-defense is unstructured natural-language prose with no such
  token, so correlating prose intent to code behaviour is the by-eye
  faithful-artifact residue `check-gate-tamper` and `check-assertion-strength`
  already concede as a stated limit plus partial floor — not a deterministic,
  low-false-positive assertion. No gate is built for the general class.
  **Why `[needs-spec]` and not closed:** a *narrower structured sub-class* could
  be gateable via a **claimed-defense annotation** convention — an author tags a
  claimed failure class with a machine-readable marker and a gate correlates the
  marker to a present assertion. That is a genuine new-gate initiative, filed
  here per `new-initiative-filing-default` (a broader new-gate build is filed,
  never smuggled into this iteration's lean single-unit envelope), demand-gated.
  The open design is whether the annotation buys reach or ceremony — the same
  reach-vs-ceremony question as `assertion-strength-exit-header-reach`; the
  honest outcome may be to record the general class as a permanent human-audit
  limit in `gate-sdk/SPEC.md` near `check-assertion-strength` and build nothing,
  a widest-true-tier placement call in its own right (gate-sdk honest limit vs a
  meta-gate doctrine line).
  **Cost while deferred:** low and non-rotting — this iteration's instance is
  fixed and the general class stays a review tripwire; the recurrence cost is
  that a future gate can overclaim in prose versus its assertions and only human
  audit catches it. Debt/analysis, adds no governed name unless the annotation
  convention is built. Filed 2026-07-20 by the `render-fidelity-leak-coverage`
  spec, settling the flagged `/spec` question.

- **stage-routing-for-debt-with-design-rulings** [needs-spec] — the lifecycle
  contract does not document how a **debt** unit that carries genuine design
  rulings (not mere behavior-convergence) routes its design: through a `/spec`
  pass, or settled at `/scope`. `/spec` is nominally trigger-gated to feature
  units, yet this iteration ran `/spec` for a debt unit by lead ruling and it
  worked cleanly — the false-positive floor was real design work that scope
  would have carried badly.
  **Design question (why [needs-spec]):** a lifecycle-contract clarification —
  name the routing rule and place it at the widest true tier
  (`lifecycle-kit/templates/skills/scope.md` / `spec.md`, or lifecycle-kit's
  SPEC), the same shape as `scope-iteration-cost-bundling-test` and
  `new-initiative-filing-default`. Debt, adds no governed name.
  **Cost while deferred:** low but recurring — each debt unit carrying design
  re-litigates its routing by lead ruling, as this iteration did.
  Filed 2026-07-20 by lead ruling at this iteration's build, from a spec-stage
  process observation.

- **build-stage-tier-economics** [needs-spec] — measure whether the `build`
  stage downgrades from Opus to Sonnet net-positive rather than flipping on
  intuition; a ruling-config tier re-judgment (`.claude/agents/stage-session.md`
  / the lead template's ruling-config, which invites re-judging every tier).
  Grounding: `drift-kit/bin/stage-economics.sh` shows build is the largest token
  consumer of any stage (~100–175k output, 5–25M cache-read per run, versus
  close/validate ~7–49k output), so it is the highest-value tier to test — a
  bigger lever than the validate→Sonnet move already adopted.
  **Two design blockers:** (1) the decision metric is uninstrumented — the tool
  reports `cost=n/a` because `scripts/price-table.tsv` is absent (whether that
  file is intentionally a local/gitignored artifact or simply not yet created is
  itself part of the design question), so no price-weighted comparison is
  possible today; obtaining/placing a price table is the prerequisite. (2) the
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
  ruling-config tier by data, adds no governed name. Filed 2026-07-20 by lead
  ruling during the `render-fidelity-leak-coverage` spec, from an operator
  question.

## Done

- render-fidelity-inline-span-leak

## Lessons Learned
