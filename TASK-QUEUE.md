# TASK-QUEUE.md — Checkwright work queue

## Iteration: supply-chain-trust-baseline

  The lifecycle-kit gates read this header's iteration name and the stage
  cursor — the last stamp in `.workflow/WORKFLOW-STATE.txt`
  (lifecycle-kit/SPEC.md §The state machine); queue-kit formalizes the queue
  format itself and gates this file. One iteration per hardening or roadmap
  unit; [README.md](README.md) maps the kits.

---

## New Features

## Technical Debt

## Deferred

- **runtime-dir-two-tier-detector** [needs-spec] — `check-tracking-claim`'s
  `is two-tier` predicate is rule-provable only for a directory the ignore rules
  match *whole* (a `dir/` pattern) that also carries a force-added tracked member:
  `git check-ignore --no-index <dir>` matches the whole-dir rule, and `git
  ls-files <dir>` finds the tracked member. A directory whose ignored members are
  matched by **file patterns** — the directory itself matching no rule, e.g.
  `.workflow/` under `.workflow/*.log` + specific-file ignores — has no rule-based
  two-tier proof: `check-ignore --no-index .workflow/` is a no-match, so the gate
  reads it as one-tier and the claim reds. The gate fix (this iteration) moved the
  ignored side from a presence read (`ls-files --others --ignored`, which listed
  only files that exist and so made `.workflow/`'s claim verify locally but red in
  a fileless CI checkout) to the rule-based `check-ignore --no-index`; that fix is
  correct and deterministic, but it cannot prove the file-pattern two-tier shape,
  so `.workflow/`'s two-tier fact was **unbound from the gate** in CLAUDE.md and is
  now carried as prose citing gate-sdk/SPEC.md §The workflow directory.
  **Deliverable:** a rules-based detector that proves two-tier for the
  file-pattern shape and re-binds `.workflow/ is two-tier` — probing candidate
  member paths under the directory via `check-ignore --no-index` (which resolves
  on non-existent paths), or reading the ignore patterns that target inside the
  directory. **Why `[needs-spec]`:** enumerating "would-be-ignored members" of a
  directory without the files present is the open design — a probe-path approach
  needs a principled candidate set, and a pattern-reading approach re-implements a
  slice of gitignore matching (negations, nested `.gitignore`, precedence), whose
  false-positive floor is the whole question. The honest outcome may be that the
  file-pattern two-tier shape stays a prose-only description and the predicate's
  reach is documented as bounded to whole-dir patterns.
  **Cost while deferred:** low and non-rotting — the gate is correct and
  deterministic over what it reaches (whole-dir gitignored, force-added two-tier),
  `.tmp/`/`.metric/` stay gated, and `.workflow/`'s two-tier fact is stated
  accurately in prose; the residue is one true directory-tiering fact the battery
  cannot mechanically hold. Filed 2026-07-22 by close, operator-blessed, from the
  remote-oracle red on this iteration's release commit (the gate shipped
  presence-based and failed its first CI run).

- **done-slug-commit-naming-gate** [needs-spec] — `kpi-task-split` reads a Done
  slug's feature/debt class off the commit its message names, via
  `git log -1 --grep=<slug>`. Nothing requires a landing commit to name its
  slug, so a correctly-typed `fix` commit that omits it leaves the row
  **unclassified** and the KPI reports `0f/0d`. Reproduced this iteration on all
  four units: `1dac2f9`, `9c5aeb2`, `a740a7b` are each typed `fix(delegation)`
  and each moves its slug(s) into Done **in the same commit**, yet none names a
  slug in its message — so the newest matching commit for every one of the four
  is the `chore(scope)` opener, and the split read `0f / 0d of 4 done (4
  unclassified)` for an iteration that was unambiguously four debt units.
  **Gap generalization — this class is gateable, and precisely.** The commit-msg
  hook holds both inputs the rule needs: a commit whose diff **adds a slug line
  to the queue's Done section** must name that slug in its message. Both halves
  are mechanical, the trigger is narrow (only Done-moving commits arm it), and
  there is no judgment in the match — the low-false-positive contract looks
  satisfiable without a new vocabulary. The queue-section parse already exists in
  `kpi-task-split`'s awk and in queue-kit's gates.
  **Open design (why `[needs-spec]`, not a build unit):** whether the rule binds
  the *moving* commit or merely requires *some* commit in the range to name the
  slug — a build may legitimately land the fix in one commit and move the queue
  line in another, and the strict form would then red on a correct sequence. That
  choice decides the gate's shape and its false-positive surface, and it is a
  queue-kit-vs-gate-sdk placement call besides (queue-format knowledge versus
  commit-message mechanism).
  **Cost while deferred:** low per-iteration but silently compounding — the
  headline lead KPI reads `n/a`-shaped noise whenever the convention lapses, and
  the lapse is invisible until close reads the report. `trajectory.sh` is
  unaffected (it splits range commits by subject and recorded this iteration's
  debt correctly), so the published evidence stays honest; what degrades is the
  live dashboard the lead steers by. Bounded: nothing breaks, no gate reds.
  Debt: converges a KPI onto a stated convention; adds no governed name unless
  the gate lands. Filed 2026-07-22 by close, from this iteration's own
  unclassified split.

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
  worst-first:* (1) the **escalation resume model** — the lead's whole value is
  resuming a paused stage in place instead of cold-restarting. **Re-ranked by
  the 2026-07-25 amendment below:** the founding premise ("a foreign agent
  cannot be resumed headless, so escalation means cold restarts") is stale —
  headless warm-resume exists per vendor, so (1) collapses into (2) as a
  property of the chosen transport, plumbing after all. (2) **dispatch
  transport** — today the harness `Agent`/`SendMessage`/task-notification; a
  foreign agent needs a transport-neutral handoff (committed worklist, issue,
  spawned process). The adapter contract is "open / prompt /
  permission-request / resume" spoken over each vendor's structured machine
  plane, never its TUI: a screen-scrape relay (tmux send-keys/capture-pane)
  is adapter-of-last-resort for a vendor shipping no machine interface at
  all — it yields rendered frames instead of turn events, answers dialogs by
  heuristic, and bets on the vendor's least-stable surface. (3) **budget
  oracle** — `usage-verdict.sh` is Anthropic-OAuth-specific; a heterogeneous
  fleet has N vendor-keyed oracles (the same seam as this session's
  credential-swap / token-usage tasks); the vendors' JSONL event streams
  carry token-usage events — raw oracle material the TUI path would have to
  scrape from a status bar. (4) **stage-contract expression** —
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
  credential-swap budget-oracle tasks. **Demand-gated — demand attested
  (2026-07-23):** the operator holds working foreign-vendor subscriptions
  (Codex; Gemini Enterprise via the Antigravity CLI) and wants read-heavy
  delegation routed to them for token-budget headroom — exactly the
  per-batch vendor-routing lever named above, and with three vendors live
  the N-keyed budget-oracle seam (3) is no longer hypothetical. First slice
  at promotion: not full stage dispatch but a foreign-CLI executor for the
  already-pre-authorized read-heavy audit / mechanical-sweep class (dispatch
  transport: a spawned non-interactive CLI process — `codex exec`-shaped,
  one adapter per vendor as consumer config; the escalation-resume
  sub-problem (1) is moot for autonomous read-only work, so it stays
  unblocked-on). Promotion-eligible at the next scope session.
  **Design-memory amendment (2026-07-25, verified against the installed
  CLIs):** the operator probed the interactive-TUI-relay alternative (drive
  vendor TUIs under tmux, relay via send-keys/capture-pane) for session
  resume and token efficiency. Ruling: those benefits live in the vendor's
  session store, not the TUI — the APIs are stateless, and both modes replay
  the same on-disk transcript against the same server-side prompt cache, so
  interactive-vs-headless is a rendering choice, not a state choice. Verified
  locally: Codex ships `codex exec resume <session-id> "<prompt>"` (headless
  warm-resume by id, `--json` JSONL turn events) plus `mcp-server` (stdio)
  and an experimental `app-server`; Claude Code pairs `-p --resume` with
  bidirectional stream-json. Gemini CLI speaks ACP (unverified — not
  installed here). First slice unchanged; escalation-with-warm-resume lands
  as a cheap second increment (`exec resume`) on the same adapter, not a new
  architecture.
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
  **Remaining scope is the roster design only — the one-line leak was split out
  and fixed separately.** The concrete `delegation-kit/smoke/install.sh`
  threshold leak this entry recorded is now its own promoted debt unit,
  `delegation-smoke-threshold-pin` (2026-07-21, operator ruling): it needed no
  config seam, only the export the sibling test script already uses. What stays
  here is the part that genuinely earns a design pass — the per-kit roster of
  credential-consuming bins and its fall-open default. Do not re-file the pin.
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

- **supervision-overhead-unmeasured** [needs-spec] — the `supervision` row is
  now the iteration's third-largest line and has never been examined. Re-derived
  at this close: supervision **$6.9870 of a $37.26 iteration total, 18.8%** —
  larger than every stage but build 1 ($9.3255), and larger than scope ($5.3941),
  spec ($5.2158), and align ($3.2266) individually. It went unexamined because
  until Delta B landed *this iteration* it was **not a distinct row at all** —
  the lead's burn was attributed nowhere. Note the scale comparison that makes
  this the priority: `stage-tiering-unit-is-the-batch` (above) fights over ~$2 of
  build-tier spread; this line is more than three times that.
  **First experiment, to run as recorded here:** run the **next** iteration's
  lead session on Sonnet and measure it against this iteration's Opus baseline of
  $6.9870. The `supervision` row makes that a clean A/B, which is exactly what
  Delta B was built to enable. It could not be tried mid-iteration — a session
  cannot re-tier itself — so the binding change necessarily applies to a future
  lead session, and the baseline above is the number it is measured against.
  **Window declined 2026-07-22 for `budget-oracle-honesty` — deliberately, not
  missed (operator ruling at that iteration's scope).** The experiment was live
  and expiring: a lead session cannot re-tier itself, so running it meant
  restarting the already-Opus lead at iteration open. Declined on the asymmetry
  between the two axes — the cost side now has 17 priced Opus supervision rows
  and the quality side has **no read at all**, and this entry's own risk
  paragraph says a bad ruling would score as a *saving*. Running a cost-only A/B
  on the tier that makes rulings would produce a number that cannot be
  interpreted. This iteration's Opus supervision cost therefore lands as another
  baseline row, and the experiment stays blocked on the quality-read design this
  entry already owes rather than on an absent opportunity.
  **Baseline premise corrected 2026-07-22 by the scope survey — larger than
  filed.** This entry frames the A/B against the single $6.9870 figure. In fact
  `.metric/stage-economics-log.txt` carries **17 priced Opus `supervision` rows**
  spanning ~15 iterations, $3.2016 to $19.1464. The comparison is therefore
  against a distribution, not a point, which both strengthens the eventual A/B
  and sharpens the caveat below: a single Sonnet run must be read against that
  spread, and a spread that wide may swallow the effect entirely.
  **Window declined again 2026-07-22 for `workflow-surface-tiering` (operator
  ruling at that iteration's scope), on the same grounds as the first
  declination — the quality read this entry owes still does not exist, and a
  cost-only A/B on the tier that makes rulings returns a number that cannot be
  interpreted. Second consecutive declination; that iteration's Opus supervision
  cost lands as another baseline row.**
  **Premise sharpened at that scope — the spread argument cuts harder than this
  entry states, and it re-ranks the entry's own open work.** Re-counted:
  `.metric/stage-economics-log.txt` now holds **24 `supervision` rows**, the
  priced Opus ones still running $3.2016 to $19.1464 — a **6x** spread. Read
  against that, a single Sonnet lead session is not merely underpowered at the
  margin; it is uninterpretable in principle, because any plausible tier effect
  sits well inside the existing variance. So the experiment as filed would
  return an unusable number **even if the quality read existed**. The
  consequence is a re-ordering this entry does not currently carry: it reads as
  though the quality read were the sole blocker and the cost read were ready,
  when in fact **both legs are blocked**. The cost leg needs either many
  repeated runs (n≫1, at one lead session per iteration) or a
  variance-controlled comparison that normalizes for iteration size — the
  obvious candidate being cost per unit delivered rather than cost per
  iteration. Design that normalization *before* the quality read: an
  uninterpretable cost axis makes the quality axis moot, and the reverse does
  not hold. Landed here so the next scope reads the corrected case rather than
  re-deriving it from the log.
  **A measurement caveat on the baseline itself:** supervision is the only row
  still growing while close runs, so any figure quoted for it is a snapshot. The
  lead's own mid-close read was $6.4552 (17.6% of $36.72); this close's read is
  $6.9870. The A/B must therefore compare rows read at the *same* lifecycle
  point, not two convenience snapshots.
  **The risk this experiment carries, which is a different class from
  validate's.** Supervision is where **rulings** happen. This iteration's
  highest-judgment act was a supervision-axis ruling — the intent oracle's
  finding that age *inverts* at a cliff, which redirected the whole
  `price-table-age-kpi` unit and is the reason it shipped an expiry header rather
  than an age-only KPI. Downgrading the tier that makes rulings is not the
  already-vindicated validate downgrade, whose rows are mechanical
  oracle-running; the failure mode is not a bigger bill but a **bad ruling that
  costs a rebuild**, which the cost row would score as a *saving*. So the
  experiment needs a **quality read alongside the cost read**, and the design
  question this entry owes is what that read is — a rebuild count, an escalation
  correctness sample, or an honest ruling that the axis is unmeasurable at n=1.
  **What cannot be harvested by delegation.** Supervision splits internally the
  same way `build` did — mechanical routing/verification versus genuine rulings —
  so the batch-tiering answer looks transferable. It is not: the verification
  half is **not delegable away from the supervising session**, because the
  supervisor re-running the battery and diffing every agent commit *is* the
  protocol (`delegation-kit/templates/agent-execution.md`, "Validate after every
  agent commit"). A supervision split can therefore be tiered but not delegated,
  which narrows the available levers to the tier question this experiment tests.
  **Why the figures are usable at all:** same provenance as the entry above —
  this is the first iteration whose meter output is trustworthy, being the
  iteration that fixed the meter's attribution, and the `supervision` row exists
  at all only because of that fix.
  Debt/analysis: measures an unexamined cost line and may re-tier a lead binding;
  adds no governed name.
  **Cost while deferred:** the largest unexamined line in the iteration budget,
  paid every iteration, with no evidence either way about whether it is bought or
  wasted. Bounded and non-rotting — nothing breaks, and the row now accumulates
  per-iteration baselines whether or not the experiment runs.
  Filed 2026-07-22 by close, from the same lead-side economics review.

- **launch-activation-cli** [needs-spec] — the five-minute activation path: an
  installer CLI on the reserved npm name (`npx checkwright init` / `doctor` /
  `update` / `diff` / `uninstall`, `--dry-run`) that vendors pinned kit source
  into the consumer repo and commits a lock/manifest recording profile, kit
  versions, and generated files — distribution convenience without giving up
  auditable, committed vendoring. At most three progressive profiles
  (starter / delegation / full) so the first experience is not all eleven
  kits at once; `doctor` probes the bash/GNU/jq/ShellCheck floors before any
  partial install. Acceptance shape: a clean Linux repo reaches first green
  in under five minutes with one command and no manual copying; re-init is
  idempotent; uninstall removes only manifest-recorded files; consumer smoke
  covers install/update/uninstall per profile. Sequencing: owns the
  install-ownership contract that `plugin-marketplace` must package against —
  a marketplace package without an ownership/upgrade manifest is a second
  install model. Promotion revisits the thin-installer demand-gating ruling
  (gate-sdk/SPEC.md §upgrade-smoke): the external review names
  time-to-first-value the top adoption weakness — a second attestation
  beside the anticipated second consumer.
  **Cost while deferred:** every prospective adopter pays the manual
  vendor-and-wire path, the largest single drop-off risk at announcement;
  non-rotting otherwise. Surfaced 2026-07-23 in an external
  product/positioning review (operator-commissioned; artifact local-only).

- **front-door-outcome-rewrite** [needs-spec] — rewrite the README/docs first
  screen around one job: a literal category line (verification for
  coding-agent delivery), the outcome (spec drift, skipped stages, and
  unsupported done claims become failing checks before merge), the target
  user, one command, one before/after example — the eleven-kit table and
  architecture prose move below the quick start, `demo/run-demo.sh` gets
  linked from README and the docs home (today the working demo is linked
  from neither), and the repo gains likely discovery topics. External-review
  finding: the front door describes architecture before outcome, inviting
  mis-categorization as a heavyweight methodology; the complement-not-compete
  stance (keep your spec workflow; add Checkwright where a claim must be
  mechanically proven) moves near the top. Consumes RELEASING.md's reserved
  launch-copy phrasing rather than forking it.
  **Premise corrected 2026-07-25 by the undirected scope survey — one
  sub-deliverable is already done, the other is confirmed live.** "The repo
  gains likely discovery topics" is stale: four topics are already set
  (`agent-governance`, `code-quality`, `git-hooks`, `linting`), so the residue
  is topic *quality* — none names agents, LLMs, spec-driven development, or a
  harness — not their absence. The demo-invisibility half stands and is
  confirmed: `run-demo`/`demo/` appears **zero times** in both `README.md` and
  `docs/index.md`, so the working walkthrough is linked from neither front
  door. The first screen's shape is confirmed too — badges, prose premise, then
  the eleven-kit table at README.md:24, architecture before outcome.
  **Cost while deferred:** the current first screen filters out exactly the
  reader the launch targets; zero until announcement, then compounding.
  Surfaced 2026-07-23 in the same external review.

- **platform-support-contract** [needs-spec] — make portability a tested
  contract instead of a layered explanation: a support matrix (Linux /
  macOS / Windows-WSL, exact tool floors) with a CI install-smoke leg per
  supported platform or an explicit experimental label; resolve the standing
  contradiction — docs/install.md declares no minimum versions are pinned
  while gate-sdk/README.md requires bash 4+ and GNU userland, neither of
  which stock macOS ships; the first support table distinguishes engine
  portability from full harness experience. The `doctor` probe belongs to
  `launch-activation-cli`; this entry owns the matrix and the CI legs.
  **Cost while deferred:** the install.md / gate-sdk floor contradiction is
  live public doc drift today; the rest costs nothing until macOS/WSL
  adopters exist, then becomes the dominant support load.
  Surfaced 2026-07-23 in the same external review (its portability finding).

- **preview-release-cadence** [needs-spec] — reset release signaling for a
  pre-1.0 audience: declare a preview/alpha channel, batch internal
  iterations into a slower external cadence (weekly-class) so consumers stop
  reading thirteen-releases-in-nine-days as churn, publish
  checksum-verifiable release assets instead of bare tag pointers, and keep
  a 30-second human changelog beside the migration detail. Separates
  internal iteration completion from public version publication — a
  RELEASING.md policy change more than a mechanism.
  **Premise corrected 2026-07-25 by the undirected scope survey — the count is
  larger than filed and the rhythm held.** This entry says
  "thirteen-releases-in-nine-days"; the tree now carries **14 tags in 12 days**,
  `v0.1.0` (2026-07-14) through `v0.14.0` (2026-07-25). The finding is not a
  one-off burst that has since settled — the cadence continued at roughly a
  release a day across the whole window, which strengthens rather than dates
  the signaling argument.
  **Now also carries `supply-chain-trust-baseline`'s carved-out deliverable:
  checksum-verifiable release assets.** That entry gated them on "once cadence
  stabilizes" and its promoted iteration carved them out on exactly that
  ground (2026-07-25, operator-approved), so the convention lands here, with
  the cadence it depends on, rather than being pinned to a rhythm this entry
  may change.
  **Cost while deferred:** zero pre-announcement; at announcement the
  release history itself signals instability to exactly the risk-averse
  teams the trust story targets — and the checksummed-asset gap inherited
  above stays open for the same window.
  Surfaced 2026-07-23 in the same external review (its release-signaling
  finding).

- **public-roadmap-projection** [needs-spec] — a generated Now/Next/Later
  `ROADMAP.md` projection off this queue (derivation-first: curated horizon
  markers on entries, a projection script, a freshness gate — never a
  hand-maintained copy), three-to-five items per horizon, labeled
  adoption / reliability / ecosystem / commercial; plus low-friction issue
  forms for install failures, doc problems, and adoption reports beside the
  fixture-first gate-defect template — the current funnel routes exactly
  the pre-launch usability signal away from issues.
  **Cost while deferred:** outsiders cannot read direction from a
  30-entry deferred section, and pre-launch usability feedback has no
  low-friction inlet. Surfaced 2026-07-23 in the same external review.

- **scope-amendment-authoring-gate** [needs-spec] — on a roster carrying a
  dedicated authoring stage, nothing stops the **scope** stage from doing
  `spec`'s job: `4e10265` authored both of this iteration's amendment files and
  landed both amendment-ref promotions in one `chore(scope)` commit, and every gate
  stayed green — `check-amendment-queue`'s bidirectional rule is satisfied by a
  paired amendment *whenever* it was written, and the stamp protocol only checks
  that a stage's predecessor stamped, never that a stage's **output** was produced
  under its own cursor. The stage evidence then records an authoring stage that
  authored nothing, which is exactly the auditability the state machine exists to
  provide.
  **Gap generalization — the inputs are already at the hook.** The pre-commit
  hook can read the stage cursor (last stamp in `.workflow/WORKFLOW-STATE.txt`)
  and the staged diff; the roster and its predecessor map are in
  `LIFECYCLE_KIT_STAGES` / `LIFECYCLE_KIT_PREDECESSOR`. Narrow rule: when the
  roster contains `spec` and the cursor is `scope`, a staged diff that **adds** an
  amendment file (`<kit>/SPEC-*.md`) or **adds** an amendment-ref tag (canon-kit's
  spec-ref form) to a queue entry is the violation. Trigger is narrow, both
  halves are mechanical, and the match carries no judgment.
  **Open design (why `[needs-spec]`, not a build unit):** three choices decide the
  gate's shape. (1) **Scope** — the narrow scope/spec rule, or its general form: a
  stage-scoped write-surface table where each stage declares the surfaces it may
  write and a commit under stage X touching stage Y's surface reds. The general
  form is worth far more and is a much larger design (every stage's surface set
  must be enumerable and correct, and stages legitimately share surfaces). (2)
  **Placement** — lifecycle-kit owns the cursor and the roster; canon-kit owns
  amendment files and the bidirectional rule. The rule needs both, so ownership is
  a real seam call, not a coin flip. (3) **Deliberate-early valve** — an operator
  may sanction early authoring (this iteration's amendments were kept, not
  reverted), so the gate needs a legible override, and a bypass-by-`--no-verify`
  is not one.
  Known false-positive surface to specify against: close's merge step **deletes**
  amendment files and drops their refs under a `close` cursor (deletions and
  removals must not arm the rule), and a debt-only iteration skips `spec`
  entirely, so scope authoring nothing is the normal case there.
  **Cost:** a real check plus a `good/`+`bad/` fixture pair, its `# graph:`
  manifest, the regenerated pre-commit hook and graph artifact, and a
  `gates.list` row — a full gate landing, not a one-liner; the general form
  multiplies that by the write-surface table it would have to enumerate and keep
  true. Feature by the litmus: it adds a governed name.
  **Cost while deferred:** low per occurrence, and the failure is silent — the
  amendments produced this way were sound, so nothing is wrong downstream; what
  degrades is the stage evidence's truthfulness, and it degrades invisibly
  (a green battery is not evidence against it). Recurred once so far.
  Filed 2026-07-24 by spec, operator-ruled, from this iteration's own
  scope/spec conflation.

- **exit-echo-decoration-guard-vs-habit** [needs-spec] — stage-session (sonnet)
  agents decorate an otherwise-allowlisted command with a trailing `; echo
  EXIT:$?` (or a leading `echo EXIT:$?;`) to read an exit status the harness
  **already reports**. The `$?` is a shell *expansion*, and bash-guard's own
  banner states no allowlist entry can suppress an expansion — so the triage
  criterion (guard-kit/SPEC.md §The triage criterion) **cannot** resolve this to
  (a) allowlist. It resolves to either a **guard rewrite/steer** that recognises
  the benign decoration and steers to the bare form, or a **habit-change** note
  for stage-sessions to drop it. Both are guard-kit design decisions.
  Reproduced this iteration: 7 fall-throughs in `.workflow/prompt-friction.log`
  containing `EXIT:` (validate-stage, operator-reported), the residue
  `scan-prompts.sh` folds into its `echo` pattern rank.
  **Open design (why `[needs-spec]`):** a steer must strip a benign trailing/leading
  `echo <literal>$?` around an allowlisted command *without* widening the very
  expansion-suppression hole the banner warns against — a general "strip trailing
  echo" rule has a safety surface that is the whole question, so the choice
  between a narrowly-shaped steer and a documented habit-change (no new mechanism)
  is the design, plus the steer-vs-note call itself. **Sibling in the same
  decision surface:** the broader compound exploratory-read fall-throughs
  (`grep`/`ls`/`git status`/`find` chained with `;`, the dominant `scan-prompts`
  ranks this iteration) share one root — stage-sessions decorate/chain benign
  commands — and this iteration already landed the `cat`/`find` compound
  read-steers and per-segment matching; widening the steer set to the other
  read verbs is the same guard-rewrite-vs-habit call, so scope should weigh them
  together.
  **Cost while deferred:** low and non-rotting — pure recurring prompt friction,
  no gate reds, no correctness impact; the harness surfaces exit status already,
  so every decorated call re-hits one avoidable prompt and nothing downstream
  degrades. Bounded. Debt: a guard steer adds no governed name; a habit-change
  note adds none either. Filed 2026-07-25 by close, operator-reported, from this
  iteration's validate-stage permission friction.

- **evidence-journal-hash-chain** [needs-spec] — tamper-evidence for the
  evidence trail itself, kit-side and infrastructure-free: each evidence
  record (a stage stamp, a gate-run evidence line) carries the hash of its
  predecessor, making the journal an append-only chain whose retroactive
  edits — a rewritten verdict, a silently dropped red run — break the chain
  and are detectable by a standalone offline verifier, no service involved.
  This is the first cryptographic rung under **hosted-attestation-service**
  and useful alone before it: a chained journal is exactly the record a
  neutral party later countersigns or logs, and today's integrity story
  (git history plus content hashes) proves nothing about omission.
  **Open design (why `[needs-spec]`):** which surfaces chain (the
  WORKFLOW-STATE stamp stream spans tracked projections and gitignored
  local capture; gate evidence is per-iteration), the chain's scope and
  reset boundary (per-iteration vs continuous), where the genesis hash
  anchors, and the record envelope — a DSSE/in-toto-compatible statement
  shape would make the chained record double as the attestation wire
  format, a contract decision that outlives the journal and should be
  taken deliberately, not defaulted.
  **Cost while deferred:** zero mechanism rots and nothing regresses — the
  gap is a missing guarantee, not drift, and it only bites when a third
  party is asked to trust the record, which is the hosted rung's
  precondition anyway. Bounded. Surfaced 2026-07-25 in the operator's
  attestation-direction review.

- **templates-stages-taxonomy-realignment** [needs-spec] — the tree scatters a class the
  SPEC names as one. `lead.md`, `release-sweep.md`, and `upgrade.md` are each
  classified **"boundary skill, not a stage"** (lifecycle-kit/SPEC.md:1375,
  :1391, :1417; lead cites "the release-sweep precedent" at :157), yet the
  layout puts two of them (`release-sweep`, `upgrade`) inside
  `templates/skills/` beside the stages and the third (`lead`) alone at
  `templates/` root. The realignment: move `release-sweep.md` + `upgrade.md`
  up to `templates/` root beside `lead.md` (the boundary-skill class), and
  rename `templates/skills/` → `templates/stages/` so the directory becomes
  precisely the stage-class templates (`scope`, `spec`, `align`, `build`,
  `validate`, `close`). Result: two pure adoption globs (root = boundary
  skills, `stages/` = stages) mirroring the SPEC's own stage/boundary axis,
  in place of today's mixed `skills/` bag plus a lone root file.
  **Why deferred, not done:** the payoff is purely cosmetic and the cost is a
  breaking rename of a **consumer-facing adoption path**. No correctness or
  gate value — `check-stage-skill-coverage` reads the configured roster
  (`LIFECYCLE_KIT_STAGES`), not the directory, and the roster can't be derived
  from `stages/` anyway (`spec.md` sits in the dir but is out of the default
  roster, and the roster is ordered and consumer-configurable), so there is no
  derivation-first win hiding here. The path is published surface: the binding
  shims hardcode it (`.claude/commands/*.md` → "Execute the template at
  `lifecycle-kit/templates/skills/<stage>.md`"), ~15 SPEC `§templates/skills/`
  refs point at it (including the binding-shim grammar section the boundary
  skills themselves cite), README's adoption line names it, and any vendored
  consumer adopted `templates/skills/`. Renaming it is a breaking change under
  the kit's own upgrade contract — it needs a deprecation marker and a release
  note, exactly the machinery `upgrade.md` narrates.
  **Cost while deferred:** zero — nothing rots, no gate gaps, no drift; the
  residue is one layout that groups by a different axis than the SPEC's stated
  one, with the SPEC prose carrying the true taxonomy correctly meanwhile. The
  disposition to hold: land it folded into the **next major that already
  breaks the adoption path** for another reason, so the break amortizes rather
  than standing as its own breaking release. Filed 2026-07-25 by operator
  request, from a session tracing why only lifecycle-kit carries a
  `templates/skills/` subdirectory.

- **needs-spec-tag-rename** [needs-spec] — the design-pending tag's *name*
  asserts a falsehood over part of the set it governs. `spec` is feature-bound
  everywhere in the system: `spec:` pairs with an authored amendment, the
  `/spec` stage authors amendments, and `check-amendment-queue` makes `spec:`
  **feature-only** ("a `spec:`-tagged entry in an active non-feature section
  … is misfiled there — it belongs in a feature section"). But `[needs-spec]`
  is mandatory on **every** Deferred entry, and Deferred is triaged into
  feature-vs-debt only at scope — debt always promotes, features only where
  scope authors. A deferred **debt** item promotes with its `[needs-spec]`
  **deleted, never converted to `spec:`** (debt carries neither tag in the
  active queue), so for the debt fraction the token names a spec that will
  never exist. The tag's own owner doc already defines it correctly as a
  "**design-pending marker**" (queue-kit/SPEC.md:59) — section-wide and
  honest; it is the *token* `needs-spec` that leaks the feature implication the
  definition does not carry. A name that contradicts its own definition, on the
  naming-doctrine tree.
  **Deliverable:** rename the pending-pole token to a design-state-honest
  spelling — candidate `[design-pending]` (doc-aligned, verbatim from
  queue-kit/SPEC.md:59); alternatives `[triage-pending]` / `[unscoped]` if
  "design" still over-implies for debt (the one fact true of every deferred
  entry is that scope has not triaged it). The `spec:` ready-pole stays — it
  is correctly feature-bound. Breaking the `[needs-spec]`/`spec:` lexical
  pairing is **intended**: the pending→`spec:` path is not guaranteed (debt
  exits the pool with no tag), so the tokens must not imply it is. Touches
  queue-kit (`check-tag-lead-line`, README, `TASK-QUEUE.md` template),
  canon-kit (`check-amendment-queue` + SPEC), lifecycle-kit (close's gap-drain,
  `enter-stage.sh` help text), the fixtures, docs, and a migration of every
  existing Deferred entry. **Not a decommission:** the pending↔ready
  cross-check is load-bearing (forbidden-in-active catches a stale tag on a
  botched promotion; `spec:`-in-Deferred forces promotion) — rename preserves
  every guard clause verbatim, only the token string changes. **Why
  `[needs-spec]`:** the exact replacement token is the open call, and whether to
  narrow the tag's *meaning* to match a renamed token or keep it section-wide
  under a truer name is the design the spec settles.
  **Cost while deferred:** low and non-rotting, rising toward launch — the
  grammar is internally consistent and gated; the residue is one token whose
  name every reader must mentally correct for the debt fraction, and a breaking
  rename that is near-free today (no external adopter) but prices in with the
  first one. Best landed before launch, or folded into the same major as
  `templates-stages-taxonomy-realignment` (both are pre-adoption grammar
  breaks). Filed 2026-07-25 by operator request, from the same session, on the
  observation that `[needs-spec]` implies a feature while deferred triage is a
  scope-stage decision.

- **md-section-near-miss-match** [needs-spec] — `context-kit/bin/md-section.sh`
  silently returns empty (exit 0, no output) on a near-miss heading query instead
  of matching or failing loudly. This session it was queried with `wakeup-guard`
  and `The knowledge-friction loop` while the actual headings were
  `wakeup-guard (template)` and a differently-worded line, and the tool matched
  only on exact/leading text — reproduced twice. The silent-empty result
  mis-signals "no such section" and pushes the caller to read the gate/mechanism
  **source** as a fallback, which is the source-prediction anti-pattern
  (oracle-first / the knowledge-friction lesson this iteration gave a doc home in
  guard-kit/SPEC.md §escalation-guard). **Design direction:** prefix/substring
  heading match, or fail loudly with a `did you mean <closest heading>`
  suggestion, never exit 0 empty. **Optional adjunct:** an advisory guard nudging
  a non-gate-authoring session that Reads a check's `.sh` toward the SPEC + running
  the gate (advisory-only, the honest ceiling for a judgment boundary).
  **Why `[needs-spec]`:** the match-relaxation has a real false-positive surface —
  a prefix/substring match can silently resolve to the *wrong* section, so
  choosing among prefix-only, substring, and fail-loud-with-suggestion is the open
  design, and the advisory-guard adjunct is a second guard-kit-vs-context-kit
  placement question. Not a close-stage inline fix: changing a shared tool's match
  semantics wants a fixture pair and belongs in a scoped unit, not a drive-by.
  **Cost while deferred:** low and non-rotting — the tool is correct on an exact
  query; the exposure is a caller silently mis-reading a near-miss as "absent" and
  source-reading as fallback, the exact loop the doctrine names. Filed 2026-07-25
  by close, drained from the committed gap inbox (`c5c595c`).

- **validate-verb-collision-and-check-routing** [needs-spec] — two coupled
  defects with one root: the delegation discipline verb collides with the
  `/validate` stage noun, and that collision misroutes the lead's
  post-delegation check onto the evidence producer.
  **(1) Verb collision.** The lifecycle stage `/validate` (runs `run-validate`,
  the broad checksuite) shares its term with the delegation discipline "validate
  after every agent commit" (the lead re-runs the gate battery to verify a
  sub-agent's self-report). The verb is performed right *before* the stage, so
  completing the lead-side verify reads as completing the stage — this iteration a
  lead conflated them and nearly skipped the `/validate` stage, jumping build to
  close. **Cheapest true fix:** rename the delegation discipline **verb**
  (validate → verify or re-check) in `delegation-kit/templates/agent-execution.md`
  and the lead binding, leaving the load-bearing stage name (`/validate`,
  `run-validate`, the state machine, gates, docs) untouched. Renaming the stage is
  rejected as invasive for a confusion the verb rename fully resolves.
  **(2) Check-routing gap the collision causes.** The post-delegation-check
  binding (`validate-battery`: re-run `run-gates` + the touched kit's
  `run-gate-tests`) carves out no case for when the *delegated stage is `/validate`
  itself*. There the naive "re-run the battery to check it" is **wrong**:
  `/validate` is the evidence producer, so re-running `run-validate` mutates or
  duplicates the committed execution evidence rather than verifying it. The correct
  lead-side check for the validate stage is to **read** its committed evidence
  manifest (`.workflow/validate-evidence.txt`), never to re-run. This session a
  lead acting on the vague default moved to re-run `run-validate` to "verify" the
  validate stage twice; the operator caught both. The binding must distinguish
  **work-producing stages** (spec, build → re-run the battery, safe and
  idempotent) from the **evidence-producing stage** (`/validate` → read the
  committed manifest, never re-run). The two defects travel together because the
  verb rename is what stops the check discipline mis-attaching to the evidence
  producer.
  **Why `[needs-spec]`:** a prose rename plus a binding-semantics change across
  `delegation-kit`'s dispatch template and the lead binding, with a
  grep-propagation pass — a shipped-kit surface change that wants a scoped unit,
  not a close drive-by; and the work-vs-evidence carve-out is a delegation-kit SPEC
  ruling, not a wording tweak. **Cost while deferred:** low but recurrent — every
  delegated `/validate` re-litigates whether the lead re-runs or reads, risking
  corrupted execution evidence, and the verb keeps inviting the skip-the-stage
  conflation. Filed 2026-07-25 by close, draining the committed gap inbox
  (`0eec298`) merged with the lead's post-dispatch third triage item (the
  check-routing half).

- **core-files-kit-coverage-derived** [needs-spec] — `scripts/core-files.list`
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
  **Why `[needs-spec]`:** the derived set's boundary is the open design, not the
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

- **amendment-update-target-coverage** [needs-spec] — an entry under an
  amendment's `## Existing sections updated` heading can name no owning delta,
  and nothing catches it. `supply-chain-trust-baseline` shipped two such entries
  of eight: the CLAUDE.md §Housekeeping enumeration and the gate-sdk/README.md
  roster row. Both were adopted correctly by a build batch reading intent, so
  nothing shipped wrong. The exposure is that an unclaimed target is
  indistinguishable from one a batch may skip, leaving the batch to decide on
  its own authority which it is.
  **Deliverable — a canon-kit gate asserting every entry under that heading
  cites a delta the same amendment defines.** The authoring-side rule landed at
  this close (canon-kit/templates/SPEC-amendment.md) and align now carries the
  manual verification (lifecycle-kit/templates/skills/align.md); this entry is
  the assertion those two currently stand in for.
  **Why `[needs-spec]`:** the delta-letter grammar is convention, not contract.
  `**A1.` through `**D1.` is how amendments have happened to number their
  deltas; no surface pins it, and a gate cannot match a grammar the template
  never specifies. The design call is whether canon-kit fixes that numbering
  (and reds a non-conforming amendment) or weakens the assertion to "cites some
  bold token defined above" — a proxy thin enough to fail gate-sdk/SPEC.md
  §When a gate earns its place.
  **Cost while deferred:** bounded. The failure mode is an edit adopted by
  judgment rather than one dropped, and the align duty now names it.
  Debt: one gate over an existing governed structure; adds no governed name.
  Filed 2026-07-25 by close, from the `supply-chain-trust-baseline` lesson
  triage.

- **gap-inbox-commit-ownership** [needs-spec] — the gap inbox has an unowned
  commit seam and an under-advertised entry point, both hit this iteration by a
  lead filing a bullet mid-iteration. Two halves, one unit.
  **(1) Who commits a lead-filed bullet is unspecified.** lifecycle-kit/SPEC.md
  §The committed gap inbox calls the channel committed and names any
  mid-iteration session, lead or stage, a producer; lifecycle-kit/templates/lead.md
  holds that every lifecycle-state write happens in a stage session, and the
  lead has no stage session of its own to carry the bullet. `bin/file-gap.sh`
  warns about the cursor consequence and says nothing about the commit
  obligation, so the producer derives whether to commit alone, ride a concurrent
  stage session's commit, or wait for a clean checkpoint.
  **(2) The always-loaded tier advertises only the narrower channel.** CLAUDE.md
  §Housekeeping names `kfric.sh` as the any-session capture affordance and never
  mentions `file-gap.sh`, so the work-shaped channel is visible only after
  loading lifecycle-kit/SPEC.md. That channel carries the harder consequence: a
  bullet blocks the next scope entry. The asymmetry is standing pressure toward
  the kfric overload drift-kit/SPEC.md §The knowledge-friction loop warns
  against.
  **Deliverable:** the SPEC names the commit owner, `file-gap.sh` says so at the
  point of capture, and one always-loaded line pairs the two channels by their
  seam.
  **Why `[needs-spec]`:** the ownership rule is the open call, not the wording.
  "The producer commits it alone" is one candidate and it collides with the
  shared-index discipline whenever a stage session holds a dirty index; the
  alternative defers the bullet to the next clean checkpoint and widens the
  window in which a filed gap is uncommitted.
  **Cost while deferred:** low, and it recurs per lead-filed bullet.
  Debt: docs plus one stderr string; adds no governed name.
  Filed 2026-07-25 by close, from the `supply-chain-trust-baseline` gap-inbox
  drain (both halves one unit, as the bullet filed them).

- **security-advisory-lane** [needs-spec] — `SECURITY.md` (shipped this
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
  **Why `[needs-spec]` — the lane owes three decisions, and copying the Issues
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

- **evidence-row-upsert-order** [needs-spec] — `run-gates.sh` rewrites its
  `gates` row in `.workflow/validate-evidence.txt` on **every** battery run, not
  only during validate, and the rewrite is a delete-then-append rather than an
  in-place update. The row's content is unchanged (same sha256, same counts,
  same date), so no evidence is corrupted; what the run produces is a
  reordering, and therefore a dirty tree.
  **Why it costs anything:** the battery is the repo's standing oracle, so any
  session told to verify before committing dirties a committed stage record as a
  side effect of verifying. This close hit it twice — once at entry, on a tree
  the lead had just reported clean after its own verification run, and once
  mid-session. Each hit costs a session the same diff read and the same judgment
  (churn or evidence?), and the wrong call in either direction is cheap to make:
  committing noise into a stage record, or discarding a row that mattered.
  **Deliverable:** an order-stable upsert, so a re-run of an unchanged suite
  leaves the file byte-identical.
  **Why `[needs-spec]`:** the open question is whether a non-validate battery
  run should write the file at all. Order-stable upsert fixes the symptom and
  keeps the writer's current reach; scoping the write to the validate stage is
  the narrower claim and asks what the row means when a close-stage run produces
  it. Those settle differently, and evidence-kit/SPEC.md owns the manifest
  contract that decides which is right.
  **Cost while deferred:** low per hit, once or twice per session that runs the
  battery. No gate catches it because nothing is wrong with the file's content.
  Debt: converges a writer onto a stable ordering; adds no governed name.
  Filed 2026-07-25 by close, from the session's own tree-state handling.

## Done

- supply-chain-trust-baseline

## Lessons Learned
