# TASK-QUEUE.md — Checkwright work queue

## Iteration: —

  The lifecycle-kit gates read this header's iteration name and the stage
  cursor — the last stamp in `.workflow/WORKFLOW-STATE.txt`
  (lifecycle-kit/SPEC.md §The state machine); queue-kit formalizes the queue
  format itself and gates this file. One iteration per hardening or roadmap
  unit; [README.md](README.md) maps the kits.

---

## New Features

## Technical Debt


## Deferred

- **enter-stage-simulate-no-write-fixture** [needs-spec] — add a regression
  fixture asserting `enter-stage.sh --simulate <stage>` leaves the tree
  byte-identical after a *successful* (non-refused) boundary entry. The guard
  now present at `lifecycle-kit/bin/enter-stage.sh:168-171` (the `sim` exit ahead
  of every write) has no test pinning it, so a future refactor could silently
  re-introduce the success-path write. This is the second half of the now-closed
  enter-stage-simulate-writes entry (Done): the behavioral fix is verified
  present, and only its regression guard is missing. Consider also asserting the
  `(simulate)` marker prints on the success path so an honoured flag stays visible.
  Debt: adds a fixture to shipped mechanism, no governed name.
  **Cost while deferred:** low and non-rotting — the fix is present; the exposure
  is a future un-caught regression on a rarely-touched dry-run path.
  Filed 2026-07-20 by scope, the closed entry's own second ask (operator ruling).

- **stage-economics-truncation-durability** [needs-spec] —
  `spec-split-promotion-review` names this as its real blocker (in its own
  premise-correction) but no entry existed, so that dependency resolved to
  nothing; filing it makes the dependency real and pickable.
  **Premise to confirm FIRST — the entry's first open question.** The blocker as
  filed assumes `drift-kit/bin/stage-economics.sh` reads *only* the
  boundary-truncated live `WORKFLOW-STATE.txt`, losing every `spec`-stage row at
  each iteration reset. The tree partly contradicts that: `stage-economics.sh:108`
  already unions committed history via `git log -p -U0` on the state file (the
  arm whose comment says it "keeps a stamped-but-uncommitted stage visible"). So
  whether `spec`-stage economics rows are genuinely unrecoverable from
  `.metric/stage-economics-log.txt` is itself **unverified** — the spec pass must
  establish that before prescribing any fix, and "the meter is already durable and
  the missing rows have another cause" is a legitimate outcome that would retire
  this entry and unblock `spec-split-promotion-review` directly.
  Only if the loss is confirmed does a durability fix (read committed history for
  the truncated rows, or persist them before truncation) follow.
  Debt/analysis: converges the meter's read onto durable history; adds no governed
  name. **Cost while deferred:** `spec-split-promotion-review` stays blocked on an
  unconfirmed premise, and the backlog-aging review re-raises it each iteration.
  Filed 2026-07-20 by scope (operator ruling), from the undirected survey's
  premise re-verification.

- **price-table-age-kpi** [needs-spec] — an advisory drift-kit KPI reading the
  `priced-as-of:` date in `scripts/price-table.tsv`, so a stale price table
  surfaces as a trend signal instead of silently mispricing every economics read.
  **Why not a gate:** prices are a dated literal with no machine-readable feed,
  so a freshness *check* would have to fetch externally — which reds on causes no
  commit produced and breaks hermeticity, the shape
  `site-kit/SPEC.md §The monitor boundary` already rules out. An age KPI needs no
  network: it reads a date in-tree and reports, never blocking. Follow the
  established `kpi-deferred-age` / `defer 13d` age-KPI idiom, registering the name
  in `scripts/kpis.list`.
  **Concrete urgency:** the table records a known cliff — Sonnet 5 introductory
  pricing ends 2026-08-31, and on 2026-09-01 the Opus:Sonnet ratio moves from
  2.5x to ~1.67x. Every tier judgment made against the pre-cliff ratio needs
  re-reading after that date, and nothing in-tree would announce it.
  Feature: adds a governed KPI name to the registry. Filed 2026-07-20 by lead
  economics review during the `render-fidelity-leak-coverage` close.

- **guard-read-compound-carveout** [needs-spec] — tighten `guard_rule_cat_read` /
  `guard_rule_find_glob` so an all-reads compound does not slip their
  composition carve-out. Both rules deliberately exempt a `cat`/`find` that
  pipes, redirects, or feeds a consumer — correct, that is real composition. But
  a compound whose every segment is a bare read (`cat A; echo ===; cat B`) is not
  composition; it is two file reads batched through the shell to save a
  round-trip, which is exactly what the Read/Glob steer exists to prevent, and it
  currently falls through to a permission prompt. Observed 6x `cat` + 4x `find`
  in one iteration's friction log. Fix: treat a `;`-separated compound as
  steerable when every segment is itself a bare read, leaving genuine
  pipe/redirect composition untouched; ships with the good/bad fixture pair.
  Debt: refines an existing guard rule, adds no governed name. Filed 2026-07-20
  by tooling-friction triage during the `render-fidelity-leak-coverage` close.

- **stage-economics-attribution-honesty** [needs-spec] — converge
  `drift-kit/bin/stage-economics.sh` onto per-stage attribution that can carry a
  decision. The meter prices correctly; what it *attributes* is not trustworthy,
  in three independent ways, all verified against the tool this session.
  **(a) Over-count — one session, two stamps, counted in full twice.** The dedup
  key is `<iteration>/<stage>/<session>`, so two stage stamps from one session
  are two distinct keys, and each resolves the same transcript and sums its
  *entire* usage. Live evidence: `tooling-signal-honesty align-waived` and
  `tooling-signal-honesty build` (one session) print byte-identical rows, as do
  the early `lifecycle-kit scope`/`build` pair — the same figure billed to two
  stages. The trailing note ("attributed to its stamp's stage", singular) states
  the intended model; the key does not implement it. Direction: attribute one
  session's usage once, split or assigned, never duplicated.
  **(b) Under-count — unstamped continuation sessions are invisible.** The stamp
  is the stage's *first* step, and enumeration is stamp-driven, so a stage that
  continues in a new session (a credential swap mid-stage, or any resume) leaves
  that session unstamped, matching no stamp and never sought. The `unmatched`
  counter reports the inverse case (a stamp with no transcript) and is
  structurally blind to this one. `close` is the most exposed stage: it runs last,
  after build has drained the rate window — exactly when a swap happens. Sibling
  to `background-credential-swap-support` **(b)**, which is the same swap event
  corrupting a *different* surface (delegation-kit's `.metric/` usage-trend
  projection reading its tail unpartitioned); fix them coherently, not twice.
  **(c) The lead's burn belongs to no stage.** Under the split-lead posture the
  lead's dispatch, verification, and battery runs carry no stamp and appear in no
  row, so every per-stage total understates the iteration's true cost by the whole
  supervision line item. Direction: decide whether supervision is its own row or
  is apportioned — either is honest, silence is not.
  **Cost while deferred:** any tier decision read off these figures compares
  noisy numbers; (a) and (c) push in opposite directions, so the error does not
  even have a known sign. Debt/analysis: converges an existing meter onto honest
  accounting, adds no governed name. Filed 2026-07-20 by lead economics review
  during the `render-fidelity-leak-coverage` close.

- **gap-inbox-post-close-window** [needs-spec] — the gap inbox is drained by
  `close` and refuses the next iteration-boundary `scope` entry while non-empty
  (`enter-stage.sh`), which assumes every bullet is filed *during* an iteration.
  A bullet filed after that iteration's close has no drainer left in the state
  machine: the next `scope` is refused and only an out-of-band edit clears it.
  Reproduced live: `enter-stage.sh scope --simulate` refused on two post-close
  filings.
  **Not a gate** (lead ruling, open to spec): `close` legitimately writes the
  inbox when it drains, so the rule cannot be "block changes" without separating
  close-truncating from anyone-appending; `file-gap.sh` appends without
  committing, so a gate reds at whatever commit next carries the file, blaming
  an actor who did not file the bullet; and refusing capture does not dissolve a
  real finding — it pushes it back into session context, the deferred-capture
  antipattern the inbox exists to prevent. A close that files something it
  genuinely cannot disposition in-session would red on a correct action.
  **Preferred arm:** warn at the point of capture — `file-gap.sh` knows the
  current stamp, so it can tell the filer, while they can still act, that the
  bullet will block the next `scope` unless drained or promoted directly.
  **Second half:** `enter-stage`'s refusal text says only that "the close stage
  must drain every gap", which once close has finished describes a stage that is
  not coming back; it should name the promote-directly recovery. The existing
  refusal *did* detect this correctly — what failed was the message's
  actionability, which is why a second detector is the wrong fix.
  Debt: a real boundary hazard in shipped mechanism, not a new capability.
  Filed 2026-07-20 by lead, drained from the gap inbox.

- **queue-selection-order-implicit** [needs-spec] — `queue-kit/SPEC.md`
  documents section order as selection order, so the default section sequence
  silently makes `New Features` outrank `Technical Debt` in what scope picks
  first. Unlike the spec-tag requirement that `CANON_KIT_FEATURE_SECTIONS`
  places on `New Features` alone, which is principled and argued, this ordering
  policy is embedded in section sequence with no stated argument anywhere.
  Either state the argument or make selection order explicit rather than
  positional. Surfaced while ruling on whether the two active sections should
  collapse into one — they should not, but this rides along on their sequence.
  Debt: latent policy with no owner doc.
  Filed 2026-07-20 by lead while ruling on the active-section question.

- **close-triage-surface-roster** [needs-spec] — close's inbound triage
  surfaces are enumerated only as prose in a template placeholder
  (`lifecycle-kit/templates/skills/close.md`, the housekeeping step: deprecation
  scan, gate-runtime budget check, backlog-aging review, tooling-friction
  triage). Nothing derives that list and nothing notices a surface close never
  read. Today they are `gap-inbox.md`, `knowledge-friction.log`,
  `prompt-friction.log`, the queue's Lessons section, and `essay-harvest.md` —
  and **only the gap inbox has a structural forcing function**, the
  `enter-stage` boundary refusal. The other four depend on close remembering
  them, so adding a sixth inbox is the dangerous act: it fails silently and
  forever. Direction: derive the roster (derivation-first — a roster is never
  maintained by hand), and give each surface either a forcing function or an
  explicit advisory-only marking, so "close did not read this" is a
  distinguishable state rather than an invisible one.
  **Cost while deferred:** unbounded and undetectable — a surface silently
  skipped leaves no trace anywhere in the tree.
  Debt: converges existing mechanism onto a derived roster, adds no capability.
  Filed 2026-07-20 by lead, from a review of the workflow-directory surfaces.

- **workflow-file-format-convention** [needs-spec] — `.workflow/` carries three
  extensions with no stated rule for choosing one, and the apparent convention
  does not survive inspection. The tidy reading — `.txt` for gate-read line
  records, `.md` for prose, `.log` for tool-appended events — breaks on
  `gap-inbox.md` versus `knowledge-friction.log`: both are appended by a `bin/`
  affordance one dated record at a time, differing in drain semantics and merge
  attributes, neither of which the extension tracks. So the extension encodes
  nothing reliable, and a new file's extension is a coin flip.
  **Second defect, concrete:** `prompt-friction.log` carries no `# contract:`
  header, alone among the ten non-empty files, so nothing names its owner,
  grammar, or reclaim path. Direction: state the rule in the owning SPEC keyed
  on a real property (gated-vs-advisory, or drained-vs-accumulating), rename to
  match, and gate the contract-header requirement — the header is already
  universal in practice, which makes it cheap to enforce and cheap to lose.
  Debt: a naming convention with no owner doc plus one missing header.
  Filed 2026-07-20 by lead, same workflow-directory review.

- **friction-log-merge** [needs-spec] — `knowledge-friction.log` and
  `prompt-friction.log` are the same surface twice: each is appended by a
  capture affordance one line per event, each is triaged at the same close step,
  each is metered as an advisory KPI rather than gated, and they differ only in
  which *kind* of friction they record — which a type field carries. Merging
  them behind one log with a type column halves close's inbox count and gives
  the two frictions a single trend line, which is also the more honest read:
  they compete for the same triage attention and are currently ranked against
  each other by nothing.
  **Sequencing:** land after `close-triage-surface-roster`, which will show
  whether the merge actually reduces what close must remember or merely moves
  it. Merging first risks optimising a count that the roster work reframes.
  Debt: consolidates two shipped surfaces, adds no capability.
  Filed 2026-07-20 by lead; ranked third of the workflow-directory findings.

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
  credential-consuming smoke with no pin at all — reproduced 2026-07-21 (was:
  "no kit does today" as of 2026-07-19).
  Cost to close: roughly one iteration. Surfaced 2026-07-19 by the validate
  re-entry on `derivation-by-precedent`, downstream of that iteration's
  operator-authorized hermeticity fix rather than of its precedent-doctrine
  envelope.
  **Reproduced 2026-07-21** by the `lifecycle-rule-placement` validate
  re-entry: `delegation-kit/smoke/install.sh` pins `DELEGATION_KIT_CRED_FILE`
  for its live 95%-reading `usage-verdict.sh` assertion but never pins
  `DELEGATION_KIT_PAUSE_PCT`/`_7D`, so it inherits whatever the ambient
  session env carries. That session's env happened to carry
  `DELEGATION_KIT_PAUSE_PCT=100` / `_7D=100` (an operator override for an
  unrelated stale pre-login budget reading, later reverted in config but
  already exported into the live process — reverting the file cannot unexport
  it from inherited child env), which pushed the 95% reading's expected PAUSE
  (exit 1) to OK (exit 0) — turning demo, consumer_smoke, upgrade, and
  agents_md_smoke red, since all four share that installer. Controlled by
  re-running with both vars unset (no code change): all four suites passed,
  confirming the ambient-env leak, not a code regression, was the cause. The
  one-line fix: `smoke/install.sh` should `export DELEGATION_KIT_PAUSE_PCT`/
  `_7D` around its 95%-reading assertion the same way
  `delegation-kit/bin/run-usage-tests.sh` already pins
  `DELEGATION_KIT_PAUSE_PCT=0` around its own. Still build-routed tech-debt —
  not fixed at this validate re-entry.

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

- **build-stage-tier-economics** [needs-spec] — measure whether the `build`
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
  ruling-config tier by data, adds no governed name. Filed 2026-07-20 by lead
  ruling during the `render-fidelity-leak-coverage` spec, from an operator
  question.

- **upgrade-smoke-phase-a-regen-derivation** [needs-spec] —
  `gate-sdk/bin/upgrade-smoke.sh` phase A swaps in the tip-of-tree kits then
  regenerates a **hardcoded roster** of generated artifacts — pre-commit,
  check-graph, and (as of `cd5dd59`) install-doctrine's digest — plus a **literal
  `CLAUDE.md`** in the determinism whitelist. Both are hand-maintained copies of a
  fact the vendored kits already own: which artifacts each kit's installer
  regenerates. A kit that adds or renames a generated artifact (as doctrine-kit
  just did, reddening `check-doctrine-registration` in phase B until `cd5dd59`
  patched the roster) silently falls out of phase A until someone hand-edits the
  roster — the maintain-a-derivable antipattern derivation-first rules out.
  **Design direction:** derive the regen set from the vendored kits' own installers
  (each kit owns its generated-artifact targets), so phase A re-runs whatever the
  vendored kits install rather than a literal list, and the determinism whitelist
  follows from those targets rather than naming `CLAUDE.md` by hand. This is the
  interim flagged in `cd5dd59`'s message.
  Debt: converges an existing smoke onto a derived roster; adds no governed name.
  **Cost while deferred:** low but rot-prone — each new or renamed generated
  artifact in any vendorable kit re-opens the same phase-A staleness, caught only
  when a downstream phase-B gate reds and someone re-derives the cause (as
  `cd5dd59` did). Cost to close: roughly one iteration. Filed 2026-07-21 by build
  during `lifecycle-rule-placement`'s model correction.

## Done

## Lessons Learned
