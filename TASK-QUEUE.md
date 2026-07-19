# TASK-QUEUE.md — Checkwright work queue

## Iteration: derivation-by-precedent

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

- **resume-journal-done-marker-compliance** [needs-spec] — stage-session agents
  complete without appending the spec-mandated resume-journal `DONE` marker
  (2 of 3 on stage-economics-report: align, validate; then 2 of 2 on
  derivation-by-precedent: build, validate — 4 of 5 across two iterations, a
  pattern rather than a fluke, and the recurrence is what lifts this from
  anecdote to a standing false invariant), so the
  delegation-kit/SPEC.md §Resume journal invariant "a journal present without
  `DONE` = interrupted, resume from it" **false-reads a completed run as
  interrupted**. Found 2026-07-18 checking the journals during the operator's
  journal-lifecycle question; the on-going close session did not capture it
  (its kfric triage drained only the session-context stdin entry), so promoted
  here per the operator's standing directive. Fix options: (a) tighten
  `.claude/agents/stage-session.md` to make the `DONE` append non-skippable
  (the marker is the recovery contract, enforce it); (b) a spec ruling that
  **under a live lead** the return-message plus the lead's post-commit oracle-
  verification *is* the recovery contract, making `DONE` redundant for
  lead-dispatched agents — and scope the SPEC's "present-without-DONE =
  interrupted" clause to lead-less runs. **Note the channel seam** (the same one
  drift-kit/SPEC.md §The knowledge-friction loop draws): this is a *work-shaped*
  finding and by that seam belonged in the gap inbox, not the kfric log where it
  was first stamped — same Deferred destination, and the mis-stamp is itself a
  data point for `kfric-trigger-prior-artifact-consultation`'s seam-clarity.
  **Cost while deferred:** low but real, and now recurring — recovery cannot
  trust journal presence alone while the invariant is silently false; a
  genuinely interrupted lead-dispatched stage and a cleanly-completed one are
  indistinguishable by the marker the SPEC says distinguishes them. Each
  omission costs a supervisor either a re-run of finished work or the deletion
  of an interrupted unit's only recovery record; both iterations so far were
  rescued by the lead verifying completeness out of band, which is precisely
  the manual step the marker exists to retire.

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

- **economics-budget-pct-decouple** [needs-spec] — drift-kit's `/economics`
  narrative chains three cost surfaces, the third being delegation-kit's
  `usage-trend` (`drift-kit/templates/economics.md` step 3) — the subscription
  **budget-%** rate-window footprint. Budget-% is account-wide: it is confounded
  by overlapping sessions and by a second operator on the same account, so it is
  the wrong instrument for **per-iteration cost attribution**. `stage-economics`
  already prices per-transcript, per-stage, per-model token draw (the token SSOT,
  immune to that confound), so the narrative carries a confounded advisory number
  beside a clean one. **Scope ruled with operator 2026-07-19: economics-only** —
  drop `usage-trend` from the `/economics` chain; `stage-economics` becomes the
  sole cost surface. **Keep every delegation-kit budget surface**:
  `usage-verdict`, `agent-budget-guard`, and `usage-trend` itself all stand — the
  budget-% is *correct* for its real job, the pre-dispatch safety throttle
  against a shared-account cap, where cross-session / cross-operator overlap is
  the feature not the bug (usage-trend also still serves delegation planning —
  weekly headroom). Only its role as an economics *cost* surface is removed.
  **Edit surface:** `drift-kit/templates/economics.md` (drop step 3 + its
  narrative bullet; chain becomes overhead-meter → stage-economics),
  `drift-kit/SPEC.md` §The `/economics` skill (two-tool chain, drop the
  usage-trend caller clause), `drift-kit/README.md` (reword the one-liner), and
  the regenerated `docs/drift-kit/README.md` projection
  (`check-docs-render-fidelity` gates its freshness); then the gate battery plus
  drift-kit fixtures.
  **Cost while deferred:** low, non-rotting — no data corruption (stage-economics
  is already the headline and is unaffected); the cost is a confounded budget-%
  line a reader could over-trust as this iteration's cost. Surfaced 2026-07-19 by
  the operator question on whether `/economics` measures cost purely from session
  transcripts.

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

- **smoke-exit-code-assertion-honesty** [needs-spec] — `delegation-kit/smoke/install.sh:21`
  asserts less than its message claims. The `if` guard fails only on exit 0, so
  it accepts PAUSE (exit 1) and STALE (exit 2) alike, while the error text says
  "usage-verdict did not PAUSE on a live 95% reading". Exit semantics confirmed
  in `delegation-kit/bin/usage-verdict.sh` (0 OK/RESET-OK, 1 PAUSE, 2
  STALE-or-unreadable). Pre-existing and unchanged by the hermeticity fix: the
  cred pin landed there makes a STALE far less likely but does not make the
  assertion honest, so a future STALE regression would still pass this smoke
  silently.
  **Design question (why deferred, not fixed at close):** the fix itself is a
  one-line condition change plus a smoke re-run, well under an hour — but the
  class is not one script. An assertion whose guard is weaker than its message
  is a gateable shape, and filing it as a build unit lets the fix and the
  scanner that catches the class land together per Enforcement-first, rather
  than landing a silent one-line correction in a close stage after validate has
  already signed the tree.
  **Cost while deferred:** low — one smoke assertion is weaker than it reads,
  in a kit whose other assertions are honest; the risk is a masked STALE
  regression, not a false green today. Surfaced 2026-07-19 by the validate
  re-entry on `derivation-by-precedent`, the same
  operator-authorized-hermeticity-fix thread as `hermetic-bin-roster-config`.

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
- **stage-posture-split-tuning** [needs-spec] — the `/economics` read on
  `derivation-by-precedent` found cache-read is ~95% of iteration token burn
  (cr 94.8% of 31.2M tokens; output 1.1%), so the cost levers are per-stage
  model class and capping context accumulation — not template trimming, which
  is load-triggered and already terse. Two moves the SPEC must rule:
  (a) **validate → lower tier.** validate's rows are mechanical oracle-running
  (out=7,764 on cr=759k for one session) — run the battery, report — with low
  generative judgment. A cheaper class likely serves; the lead ruling-config
  pins every stage to Opus (lifecycle-kit/templates/lead.md §Economics). align
  is explicitly *out of scope* for downgrade: its low output (35k) is
  cross-spec *verification* that prevents build struggles, not glue — the
  judgment justifies the tier.
  (b) **scope amendment-authoring split.** scope's cr (5.3M) is driven by
  causal-completeness amendment authoring after a whole-corpus sweep; the seam
  is the pre-promotion ruling ("the proposed unit set is escalated for ruling
  before promotion"), splitting the exploratory half from the generative half
  so the sweep context stops being re-read. Leading design direction: give the
  amendment-authoring half its *own lifecycle stage* rather than a second scope
  session. scope carries the unconditional iteration-boundary reset, so a second
  `enter-stage.sh scope` would re-truncate WORKFLOW-STATE; a dedicated stage
  stamps and appends like align/build/validate/close, leaving the reset uniquely
  scope's and untouched. It also sharpens the ontology (scope = bound the units;
  new stage = author the amendments; align = independently verify them) and is
  naturally trigger-gated like align — skipped for a debt-only iteration with no
  amendment. Blast radius is wide but shallow: every roster mirror (the CLAUDE.md
  stage block, lead dispatch, enforcement map, docs), all held in lockstep by
  `check-lifecycle-registration`. Alternative considered and not preferred:
  teach enter-stage.sh to reset only on the unnamed boundary and append on
  re-entry — narrower blast radius (one tool) but adds a conditional to the reset
  (lifecycle-kit/SPEC.md §check-stage-entry / §bin/enter-stage.sh). A roster
  change, its own iteration — not a tuning tweak. build-session splitting, by
  contrast, is already directed and needs no spec change.
  **Cost while deferred:** every iteration pays the top class on validate's
  mechanical pass and re-reads scope's full sweep context through the
  amendment-authoring turns. **Surfaced** 2026-07-19 by `/economics` on
  `derivation-by-precedent`.

## Done

## Lessons Learned
