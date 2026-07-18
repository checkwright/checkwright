# TASK-QUEUE.md — Checkwright work queue

## Iteration: stage-economics-report

  The lifecycle-kit gates read this header's iteration name and the stage
  cursor — the last stamp in `.workflow/WORKFLOW-STATE.txt`
  (lifecycle-kit/SPEC.md §The state machine); queue-kit formalizes the queue
  format itself and gates this file. One iteration per hardening or roadmap
  unit; [README.md](README.md) maps the kits.

---

## New Features

- **stage-economics-report** [spec: SPEC-stage-economics.md] — a tracked
  drift-kit tool `bin/stage-economics.sh` pricing lifecycle spend by stage ×
  model × iteration (WORKFLOW-STATE stamps ⋈ transcripts ⋈ a consumer-config
  price table), plus the `/economics` close-cadence skill chaining
  overhead-meter → stage-economics → usage-trend into one post-iteration
  narrative. Answers the one question no built-in surface prices — real spend by
  lifecycle stage × model × iteration — and keeps the cache-read burn lever
  visible close-over-close (the dig that motivated it: cache-read of accumulated
  context is the dominant burn, not model choice). Design, causal completeness,
  and the provenance seam (the price table is consumer config, never a kit
  literal roster) are in the amendment. Cross-component read-only: the tool
  consumes lifecycle-kit's stamp contract and delegation-kit's usage surface
  without changing either, so build entry's assertion C will demand the audit
  stamp or a recorded waiver. Supersedes benchmark-ab-experiment's
  stage-burn-meter measurement half (that rung consumes this tool rather than
  rebuilding it). Surfaced 2026-07-18 by the budget-token usage analysis run on
  the `.metric/` prototype scripts.

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

- **release-note-chrome-ownership** [needs-spec] — the release note's fixed
  chrome — the opening reserved-phrasing sentence and the whole closing
  "Upgrading" paragraph (the "mechanical allowed-red set" / "open an issue …
  a defect in the release rather than work for you" boilerplate) — has **no
  owning surface**. RELEASING.md step 1 owns only the three *variable* sections
  (grammar in docs/install.md §The upgrade contract, floor-gated by
  `check-release-bump`) and names the reserved opener phrase, but hands the
  author no skeleton for the fixed chrome. So each note's chrome propagates by
  **copying the previous post** — the derivation-by-precedent the
  de-literalization / derivation-first doctrine rules out, and it passes every
  content gate byte-identically to a derived note (a correctness gate cannot see
  the derivation path). Found 2026-07-18 when the operator eyeballed the v0.6.0
  note and named the smell; the always-loaded lead validation (battery + git
  log/status) verifies the artifact, never how it was derived, so nothing caught
  it. Fix options: (a) a spec-owned note skeleton (a `docs/posts/` template with
  slot markers for the three sections, named by RELEASING.md step 1 and
  freshness-gated so the boilerplate cannot drift — the derivation-first move
  that turns an ungated maintained-copy into a gated generated one); (b) inline
  the canonical closing paragraph into RELEASING.md as the single source plus an
  explicit "author from grammar + skeleton, never copy a prior post."
  **Cost while deferred:** low, non-rotting — notes are dated immutable
  artifacts and the load-bearing *structure* is already gated; the cost is the
  per-release imitation and its doctrine-smell, one more copied note each
  release until owned.

- **kfric-trigger-prior-artifact-consultation** [needs-spec] — the knowledge-
  friction trigger roster (drift-kit/SPEC.md §The knowledge-friction loop) cues
  capture on "an implementation, a gate's source, a commit message" — a
  *non-owning surface* — but does **not** name **consulting a prior/sibling
  deliverable to derive a new one** (reading the last release note to author the
  next; copying a prior SPEC's structure). That is the same doctrine failure as
  re-deriving off an implementation, but the roster's examples do not cue it, so
  precedent-imitation self-reports nowhere and stays invisible to every content
  gate. This is the methodology-level generalization of
  `release-note-chrome-ownership`: that entry owns one artifact's chrome; this
  entry makes precedent-imitation *recognizable* everywhere. Fix: broaden the
  §The knowledge-friction loop capture roster (and the always-loaded CLAUDE.md
  kfric bullet if its wording narrows the cue) to name prior-artifact / sibling-
  deliverable consultation as a stampable non-owning surface. **Watch the seam:**
  the loop's §Seam already routes *work-shaped* findings to the gap inbox, not
  kfric — a "this artifact should be owned/generated" conclusion is work-shaped
  and belongs in the gap inbox; kfric stays the sensor for the *fact* re-derived.
  The broadening is a cue for the fact-channel, not a licence to overload it.
  Found 2026-07-18 alongside the chrome finding. **Cost while deferred:** the
  highest-leverage of this cluster while unfixed — derivation-by-precedent
  remains structurally invisible across all authored artifacts, caught only by a
  human eye, the exact failure mode the methodology exists to remove.

- **release-note-section-taxonomy** [needs-spec] — the three release-note
  sections (Tightened gates / Renamed knobs / Behavior changes) are the
  *consumer's reconciliation checklist for residue wholesale-sync + battery
  cannot mechanically surface* (docs/install.md §The upgrade contract names four
  residue classes: shadowed gates, copied-out templates, own-config knob
  renames, depended-on behavior). Under that lens the current narrowness is
  mostly principled — "Tightened gates" (not "gate changes") is correct because
  only new/stricter gates can red a clean tree; a relaxed/removed gate reds
  nobody and is off the allowed-red worklist by design — but two real edges are
  under-covered: **(1) knob *removal* / orphaned config.** `old → ∅` silently
  orphans consumer config exactly as a rename does, but is not a rename and may
  red no gate — no clean section home. **(2) four residue classes, three
  sections.** "templates you have copied out" is a *named* residue class with no
  dedicated section, folded implicitly into Behavior changes — the section set
  is imperfectly aligned to its own stated model. Scope: review the section
  taxonomy against §The upgrade contract's four residue classes; decide whether
  knob-removal earns explicit treatment (a Renamed knobs `old → ∅` form, or a
  Removed knobs line) and whether copied-out-template residue earns a home or is
  deliberately behavior-folded — then reconcile the grammar owner and
  `check-release-bump`. Surfaced 2026-07-18 in the operator's v0.6.0 note review.
  **Cost while deferred:** low and non-rotting — a knob removal today lands under
  Behavior changes by author judgment (reconciled by reading, not silently
  dropped), so the residue is stated; the cost is a checklist axis that is
  sharper in principle than in the current section set.

- **resume-journal-done-marker-compliance** [needs-spec] — stage-session agents
  complete without appending the spec-mandated resume-journal `DONE` marker
  (2 of 3 this iteration: align, validate), so the
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
  **Cost while deferred:** low but real — recovery cannot trust journal presence
  alone while the invariant is silently false; a genuinely interrupted
  lead-dispatched stage and a cleanly-completed one are indistinguishable by the
  marker the SPEC says distinguishes them.

## Done

## Lessons Learned
