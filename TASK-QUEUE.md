# TASK-QUEUE.md — Checkwright work queue

## Iteration: front-door-readiness

  The lifecycle-kit gates read this header's iteration name and the stage
  cursor — the last stamp in `.workflow/WORKFLOW-STATE.txt`
  (lifecycle-kit/SPEC.md §The state machine); queue-kit formalizes the queue
  format itself and gates this file. One iteration per hardening or roadmap
  unit; [README.md](README.md) maps the kits.

---

## New Features

- **primary-install-path-claim** [spec: SPEC-primary-install-path-claim.md] [roadmap: now/adoption]
  roadmap-summary: One documented install path, with a gate holding every page to it.
  **no surface owns which install path is primary**, so two documents drifted
  apart with nothing to catch it: README.md's Quick start led with `npx
  checkwright init` — the command `v0.16.0` announced and which never worked,
  npm serving only the `0.0.1` reservation placeholder — while docs/install.md
  states the tarball is primary. The repo's most-read surface contradicted its
  own install page and advertised a broken command, through a release and into
  the next one. **Deliverable:** an `install-primary:` declaration owning the
  claim in docs/install.md §Quick start, plus a consistency gate holding every
  governed surface's install section to it. The registry-reachability half is
  ruled out of hermetic reach and the leading-vs-mentioning predicate is
  settled, both in canon-kit/SPEC-primary-install-path-claim.md.
  **Cost while deferred:** demonstrated, not hypothetical — it drifted at two
  consecutive releases, and the failure mode is the worst-placed one a project
  has: the front door telling a first-time reader to run something that does
  not work. Filed 2026-07-26 by close (`release-path-hardening`); promoted
  2026-07-27 by spec (`front-door-readiness`).

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
- **plugin-marketplace** [needs-spec] [roadmap: later/ecosystem] — harness plugin packaging.
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
  Surfaced 2026-07-09 in adoption-track's split; evidence artifact retained:
  upstream Claude Code issue #75214 (project config can't lift the Task
  ask-first default), surfaced dogfooding the delegation nudge 2026-07-07.
- **benchmark-ab-experiment** [needs-spec] [roadmap: later/adoption] — a controlled A/B experiment.
  roadmap-summary: A controlled experiment measuring drift with and without governance.
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
- **prose-profile** [needs-spec] [roadmap: later/ecosystem] — a profile for non-code repos.
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
- **hosted-attestation-service** [needs-spec] [roadmap: later/commercial] — a neutral attestation.
  roadmap-summary: Gate runs verified by a neutral party no committing agent can touch.
  The team/paid rung: gates
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

- **heterogeneous-agent-delegation** [needs-spec] [roadmap: later/ecosystem] — cross-vendor stages.
  roadmap-summary: Dispatch a stage to any vendor's coding agent, gated identically.
  Cross-vendor stage dispatch:
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

- **preview-release-cadence** [needs-spec] [roadmap: next/adoption] — a declared preview channel.
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
  manifest (`.workflow/validate-evidence.txt`), never to re-run. The binding must
  distinguish **work-producing stages** (spec, build → re-run the battery, safe
  and idempotent) from the **evidence-producing stage** (`/validate` → read the
  committed manifest, never re-run). The two defects travel together because the
  verb rename is what stops the check discipline mis-attaching to the evidence
  producer.
  **Recorded instances, both caught by the operator and by nothing else.**
  *First (filing iteration):* a lead acting on the vague default moved to re-run
  `run-validate` to "verify" the validate stage, twice. *Second
  (`supply-chain-trust-baseline`):* a lead ran the full `run-gates` battery after
  the validate stage's commit, as its standing "verify independently after every
  agent commit" discipline — precisely the case this part names as wrong.
  **What the second instance sharpens.** The two re-runs are not equally harmful,
  and the binding must say which is which. `run-gates` writes nothing under
  `.workflow` (see `evidence-row-upsert-order`, whose premise was corrected on
  exactly this point), so re-running it is inert on the evidence and merely
  wasted work; `run-validate` is the sole writer, so re-running *it* is the one
  that mutates committed execution evidence. The reason a lead reaches for
  `run-gates` here is that it is the generic post-delegation check and carries no
  carve-out — the routing defect is real for both, the corruption risk only for
  the second. Part (2)'s wording should split them rather than attributing the
  mutation to any re-run.
  **Disposition after recurrence — unchanged, cost raised, insufficiency named.**
  The recurrence does not make a different fix correct: the verb rename plus the
  work-vs-evidence carve-out is still the cheapest true fix, and nothing in the
  second instance argues for renaming the stage. What it does establish is that
  the prose fix is **not sufficient on its own** — it removes the ambiguity but
  installs no oracle, and a lead reaching for the generic check still gets no red.
  Gap generalization, per the close ritual: the check class that would catch this
  is a gate over a lead's *tool invocations*, and no scanner is buildable — a
  lead's choice of command leaves no tracked artifact for the battery to read.
  The nearest buildable proxy is the liveness sentinel filed under
  `validate-producer-liveness-unobservable`, which covers only the concurrent
  case, not a sequential re-run. So the unit ships the prose fix knowing
  detection stays human, and that limit belongs in the binding it lands.
  **Why `[needs-spec]`:** a prose rename plus a binding-semantics change across
  `delegation-kit`'s dispatch template and the lead binding, with a
  grep-propagation pass — a shipped-kit surface change that wants a scoped unit,
  not a close drive-by; and the work-vs-evidence carve-out is a delegation-kit SPEC
  ruling, not a wording tweak. **Cost while deferred:** raised from "low but
  recurrent" — it has now fired in two consecutive iterations, each time detected
  only by an operator catch, so the in-band cost is unbounded until someone
  happens to look: every delegated `/validate` re-litigates whether the lead
  re-runs or reads, and the verb keeps inviting the skip-the-stage conflation.
  Filed 2026-07-25 by close, draining the committed gap inbox (`0eec298`) merged
  with the lead's post-dispatch third triage item (the check-routing half);
  second instance added the same day by close.

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

- **evidence-row-upsert-order** [needs-spec] — `evidence-kit/bin/run-validate.sh`
  upserts each suite's row in `.workflow/validate-evidence.txt` as a
  delete-then-append: `awk` filters the `(iteration, suite)` line out into a
  temp, the new line is appended, the temp is `mv`'d over the manifest. So a
  re-run **relocates** the row to the end rather than updating it in place. The
  content is unchanged (same sha256, same counts, same date), so no evidence is
  corrupted; what a re-run produces is a reordering.
  **Correction to this entry's original premise, 2026-07-25.** It was filed
  claiming any battery run — not just validate's — rewrites the `gates` row.
  That is false. `gate-sdk/bin/run-gates.sh` contains no `.workflow` reference
  at all and writes nothing there; `run-validate.sh` is the sole writer.
  Re-verified by running the bare battery on a clean tree: 81/81 green, `git
  status` unchanged. The clause claiming this cost a close entry a false
  clean-tree report falls with the premise — the battery was never the writer,
  so a lead's post-delegation battery run never dirtied the manifest.
  **What actually caused the churn.** The validate session's `run-validate` was
  still executing in the background when close was dispatched, and its writes
  raced the close commits. Fingerprint in the committed manifests: `gates` is
  first in `EVIDENCE_KIT_SUITES` (`scripts/evidence-config.sh`) and first in the
  prior iteration's manifest at `7692335`, but **last** in this iteration's at
  `ae70eae`, both carrying all 21 suites clean. One relocated row with no
  content change is a second pass that had upserted only its first suite — a
  live producer, not a property of the battery. The concurrency itself is filed
  separately as `validate-producer-liveness-unobservable`; this entry keeps only
  the ordering defect.
  **Why the ordering is still worth fixing on its own merits.** A relocating
  upsert makes the manifest's line order non-deterministic, so a repeated or
  concurrent write surfaces as a diff with no content behind it, and every
  session reading that diff pays the same churn-or-evidence judgment — cheap to
  get wrong either way: committing noise into a stage record, or discarding a
  row that mattered. Under a stable order the race above would have been
  invisible in the tree; it is the ordering that turned a benign re-run into a
  session-visible defect.
  **Deliverable:** an order-stable upsert, so re-running an unchanged suite
  leaves the file byte-identical.
  **Why `[needs-spec]`:** the writer's `# spec:` line declares the current
  semantics deliberately — "a re-run supersedes this iteration's prior line for
  the suite, then appends" — so recency order may be the intended contract
  rather than an accident, and an in-place update changes it. Whether the
  manifest's line order is contractual (and if so, which order) is
  evidence-kit/SPEC.md §Evidence manifest's ruling, not a patch to the script.
  **Cost while deferred:** low per hit, and rarer than first filed — it needs a
  repeated or concurrent run-validate, not any battery run. No gate catches it
  because nothing is wrong with the file's content. Debt: converges a writer
  onto a stable ordering; adds no governed name.
  Filed 2026-07-25 by close; premise corrected the same day by close after
  re-deriving the writer set.

- **local-overlay-git-blanket-grant** [needs-spec] — the local permission overlay
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
  **Why `[needs-spec]`:** the general question is whether guard-kit should ship a
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

- **validate-producer-liveness-unobservable** [needs-spec] — a stage session can
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
  **Why `[needs-spec]`:** it adds a runtime artifact needing a named reclaim path
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

- **lead-dispatch-requires-completion-notification** [needs-spec] — the lead has
  no stated precondition for dispatching stage N+1, and the one it improvised is
  wrong: artifact state. This iteration the lead verified that validate's commit
  had landed with complete evidence, the tree was clean, the battery green, and
  `enter-stage.sh --simulate close` cleared — then dispatched close on that basis,
  into a still-running `run-validate`. Every one of those checks passed while the
  producer was mid-write, because `run-validate` commits its evidence and keeps
  going: **the terminal commit existing is fully compatible with the process still
  executing.** The lead dispatched on the *absence* of a completion notification
  rather than its arrival.
  **The generalizable trap, worth the entry on its own.** The operator mentioned
  having "approved it only now upon return," referring to a permission prompt the
  validate session had stalled on. The lead read that as the stall resolving and
  therefore the stage being finished. It means the opposite: an approval prompt
  gates a command **starting**, so that message was positive evidence the battery
  had just *begun*. Any signal about a prompt being answered is a start signal,
  never a completion one.
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
  **Disposition of the incident as a set, which differs from the lead's read.**
  The lead proposed this rule as the actual fix with the others as symptoms it
  prevents. Half right. It is the proximate cause and the cheapest fix, so it goes
  first. It is not sufficient alone, and this queue already carries the reason:
  `validate-verb-collision-and-check-routing` established this same iteration that
  a prose fix with no oracle recurs and is caught only by an operator. Shipping
  this rule without the sentinel repeats that pattern knowingly. Also retracted
  here: the incident was briefly read as `check-evidence-manifest` being
  grammar-only. It is not — see the assertion-A note under
  `validate-producer-liveness-unobservable`. Three real findings, not four:
  the dispatch decision (this entry), the observability gap (the sentinel), and
  the artifact churn (`evidence-row-upsert-order`), which is worth fixing on its
  own merits and is not merely a symptom.
  **Why `[needs-spec]`:** it adds a precondition to a shipped template's dispatch
  contract and states a limit (prose-only, human-enforced) that
  lifecycle-kit/SPEC.md should own explicitly rather than leave implied.
  **Cost while deferred:** every multi-stage iteration with a live lead can
  re-run this race, and the cost lands on the next stage session, which fights a
  file changing underneath it. Debt: one template rule plus one SPEC limit; adds
  no governed name.
  Filed 2026-07-25 by close, from the lead's own account of the dispatch
  decision; the operator ruled stage sequencing the lead's accountability, which
  is why this is a lead rule rather than a stage-session or gate concern.

- **platform-support-ci-matrix** [needs-spec] [roadmap: next/reliability] — a CI leg per platform.
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

- **gate-tamper-consumer-gate-coverage** [needs-spec] — this repo's gates
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
  **Deliverable:** restore the dropped glob —
  `("*/checks/*.sh" "scripts/check-*.sh" "gate-sdk/lib/gate.sh" "gate-sdk/bin/run-gate-tests.sh")`
  — with a fixture arm proving that a `scripts/` gate edit co-staged with
  product code is blocked, which no existing arm covers. Check the same
  replace-not-extend hazard on `DELEGATION_KIT_META_PATHS` while there; that one
  re-declares every default it overrides, so it is correct today by hand rather
  than by mechanism. One doc correction rides along: delegation-kit/SPEC.md's
  knob roster says this repo's config *widens* `GATE_FILES` — what a reader
  would assume, and the opposite of what the mechanism does. The SPEC should
  state the replace-not-extend semantics where it states the default.
  **Why deferred rather than fixed in the batch that found it:** the cost is not
  the one-line array. Widening coverage may red-line commit shapes this repo's
  own history relies on — a consumer gate landing beside the product change it
  gates is exactly the shape the assertion rejects — so the fix needs a survey
  of recent gate-touching commits before it lands, and a survey is not a
  build-batch side errand. Scope-gated intake.
  **Cost while deferred:** the assertion this repo advertises does not hold
  where it matters most. An agent can co-stage a weakening of
  `scripts/check-install-toolchain.sh` — or any consumer gate — with the product
  change that gate would have caught, and the tamper gate stays silent; the
  kit's own default covers precisely that case. Non-rotting, but the exposure is
  permanent until the glob returns.
  Filed 2026-07-26 by build, on the lead's ruling during the
  `platform-support-contract` floor batch, from a probe of `check-gate-tamper`'s
  resolved runtime config.

- **lint-scope-hook-trigger** [needs-spec] — `GATE_SDK_LINT_EXTRA_DIRS` widens
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

- **installer-lifecycle-verbs** [needs-spec] [roadmap: next/adoption] — update, diff, uninstall.
  roadmap-summary: update, diff and uninstall, so an install can be managed after init.
  The installer's second phase:
  `update`, `diff`, and `uninstall`, the verbs that manage an install after
  `init` has made one. Phase 1 (`init`, `doctor`, `--dry-run`, the manifest, the
  profiles, the packaging, and the consumer smoke) shipped in `activation-path`;
  this phase was ruled a separate build unit at promotion and is filed here on
  the lead's ruling at that iteration's merge, so it survives the amendment that
  governed it.
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
     today and is spelled `npx checkwright@<newer> init`. `init.sh`'s re-run
     refusal gates on a version difference, but `sort -V` narrows it to the
     downgrade direction only (it dies solely when the *package* is the older
     of the two); an upgrade falls straight through, the profile is re-read from
     the lock, and `claim()` rewrites only files whose hashes still match the
     manifest while reporting and preserving anything the adopter edited. So
     `--force` is not the upgrade path and never needed to be. What it costs to
     have no `update` verb is that nobody guesses `init` for an upgrade.
  3. **The cross-version upgrade is implemented but not smoke-covered.**
     `installer/consumer-smoke/run-smoke.sh` packs one `$VERSION` and asserts
     the same-version re-run leaves the tree unchanged — idempotence, not a
     version bump. The falls-through-and-re-applies path therefore has no
     automated exercise; the first adopter moving off v0.16.0 runs it first.
     Closing this is cheap and does not need the verbs: pack twice at two
     stamped versions and assert the bump re-applies while preserving a
     deliberately-edited file.
  `--dry-run` already covers most of what a deferred `diff` would add: it
  prints the file plan and the manifest that would be written. Non-rotting, and
  bounded by the manifest being correct and complete meanwhile — the data an
  upgrade needs is recorded from day one, which is why deferring costs
  capability rather than fidelity.
  Filed 2026-07-26 by build on the lead's ruling at the `activation-installer`
  merge, from scope the amendment governed and its deletion would otherwise
  have dropped.

- **installer-smoke-manifest-write-collision** [needs-spec] — `installer_smoke`
  (new this iteration in `EVIDENCE_KIT_SUITES`) sits 18th of 22 suites in
  `scripts/evidence-config.sh`'s roster, behind `gates` and every kit fixture
  suite, `guard_tests`, `usage_tests`, `budget_guard_tests`, `trend_tests`, and
  `demo`. `evidence-kit/bin/run-validate.sh` upserts each suite's row into the
  tracked `.workflow/validate-evidence.txt` manifest as it goes, so by the time
  `installer_smoke`'s turn arrives 17 prior suites have already dirtied that
  tracked file. `installer/consumer-smoke/run-smoke.sh` (and the
  `scripts/pack-installer.sh` it drives) both hard-refuse a dirty `git status
  --porcelain`, so a full `bash evidence-kit/bin/run-validate.sh` run
  deterministically reds `installer_smoke` on every invocation — not a race,
  a guaranteed collision between the roster's order and the spine's own
  write mechanism.
  **Confirmed this session.** The full-battery run for `activation-path
  validate` logged `installer_smoke -> new-failures`, log reading "the
  worktree is dirty". Stashing `.workflow/validate-evidence.txt`'s in-flight
  diff and re-running `installer/consumer-smoke/run-smoke.sh` alone on the
  now-clean tree passed all three profiles (starter/delegation/full) cleanly
  — matching the lead's independently reported clean run. The suite itself
  is sound; the codified spine's own write order is what breaks it.
  **Interim mitigation landed this session:** `installer_smoke` was reordered
  strictly first in `EVIDENCE_KIT_SUITES` (ahead of `gates`), with a
  load-bearing comment in `scripts/evidence-config.sh` explaining why the
  position matters. Verified before landing: grepped every configured
  suite's run command for a `git status`/porcelain precondition —
  `installer_smoke` (via `scripts/pack-installer.sh`) is the *only* one that
  requires this repo's own tree be clean; this session's own prior run is
  corroborating evidence, since the other 21 suites passed on a
  progressively dirtied tree. Also confirmed no gate, evidence-kit contract,
  or parser assumption pins `gates` (or anything else) to roster position 1
  — it was only `scripts/evidence-config.sh`'s own hand-chosen first entry.
  A **baseline hold was tried and reverted the same session.** Holding
  `installer_smoke installer_smoke fail <slug>` constant in
  `.workflow/validate-baseline.txt` was the first disposition attempted; it
  was wrong and has been dropped. `installer_smoke` carries no
  `EVIDENCE_KIT_PARSER_installer_smoke`, so it falls to the default
  `exit-code` parser, whose entire parsed result is one row. Any nonzero
  exit — a dirty tree, a broken profile, a pack failure, a genuine `init`
  regression — produces the identical row, and a held-constant baseline
  entry classifies all of them `clean` alike. The hold did not preserve a
  known red; it blinded the suite to every other failure mode. That is why
  holding it constant is not a viable disposition on its own, and why the
  reorder — imperfect as an interim step — is the mitigation that landed
  instead: it lets the suite's real result stand rather than papering over
  it.
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
  **Why `[needs-spec]`:** candidate (a) changes the writer contract
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

- **installer-upgrade-smoke-arm** [needs-spec] — `installer/consumer-smoke/run-smoke.sh`
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

- **gate-file-coverage-closure** [needs-spec] — the missing check class behind a
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
  **Why `[needs-spec]`:** it makes coverage-completeness a delegation-kit
  contract, which is a SPEC assertion, not just a new script.
  **Cost while deferred:** the config is correct today but unheld — the next
  consumer-resident gate, or the next kit-glob edit, can reopen the identical
  hole with nothing to catch it. Exactly the replace-vs-extend footgun the SPEC
  now documents but does not enforce.
  Filed 2026-07-26 by close (`activation-path`), generalizing the
  knowledge-friction captures that surfaced the replace-vs-extend semantics.

- **kit-owned-install-recipe** [needs-spec] — a kit's **zero-config gate
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
  **Why `[needs-spec]`:** it adds a kit-root structural predicate (every kit
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

- **operator-authored-unit-set** [needs-spec] — lifecycle-kit/SPEC.md's
  standing-scope-directive clause models exactly one shape: a **lead-authored**
  menu, "a theme bounding scope's survey and never a slug list". This iteration
  ran the other shape — an **operator-authored slug list** naming all four units
  before scope opened. That is legitimate in substance (the operator owns what
  the project builds) but unmodeled in the contract, and the clause's own stated
  hazard — a pre-authored menu pre-empting scope's premise re-verification — is
  precisely the hazard the operator case carries, with nothing written to
  address it.
  **Deliverable:** model the operator-authored unit set explicitly in the
  standing-scope-directive clause, and restate the premise-re-verification
  obligation as **scope's own regardless of who authored the list** — the
  re-verification is not a check on the author's authority, it is scope's
  contract with the tree.
  **Why `[needs-spec]`:** it amends a lifecycle-kit SPEC clause that governs
  stage-entry semantics for every consumer, and the honest form may be a
  generalization (any pre-authored unit set, author-agnostic) rather than a
  second enumerated case — that choice is the design.
  **Cost while deferred:** low but recurring — every operator-directed iteration
  runs a shape the contract does not describe, and scope's re-verification rests
  on convention rather than on a written obligation. Nothing reds; the residue is
  a governing clause narrower than the practice it governs.
  Filed 2026-07-26 by close (`release-path-hardening`), draining the gap inbox;
  surfaced by that iteration's scope and routed to spec by the lead for filing,
  deliberately not fixed in-iteration.

- **tarball-build-attestation** [needs-spec] — `release-tarball-delivery-channel`
  ships the Release tarball beside a `.sha256`, and that checksum proves
  **transfer integrity only**: checksum and tarball share one origin and one TLS
  session, so a consumer who fetches both from a compromised release fetches a
  matched pair. The **build-provenance** property — this artifact was built by
  this workflow from this commit — stays exclusive to the npm channel, which has
  `--provenance`. The tarball is now the primary channel, so the stronger
  property lives on the secondary one.
  **Deliverable:** a signed build attestation on the tarball channel —
  `actions/attest-build-provenance` at publish time, verified by
  `gh attestation verify` at install time — plus the install-path documentation
  that makes verification a step a Node-free adopter can actually run.
  **Why `[needs-spec]`:** it adds a fourth pinned `uses:` ref to `publish.yml`
  (a supply-chain-surface change the supply-chain-trust-baseline contract
  governs), and the install-side half is an open question: an attestation nobody
  verifies buys nothing, but `gh attestation verify` reintroduces a toolchain
  dependency on the channel whose whole premise is needing none.
  **Cost while deferred:** bounded and non-rotting — the checksum is honest about
  what it proves and the docs claim no more. The residue is a primary channel
  with a weaker integrity story than the secondary one, which inverts as adoption
  moves to the tarball. Costed at roughly one small unit.
  Filed 2026-07-26 by close (`release-path-hardening`) off the gap inbox's
  tarball-checksum bullet; ruled outside the shipping unit's envelope at align.

- **spec-measured-count-gate** [needs-spec] — a **measured count authored into a
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
  **Why `[needs-spec]`:** the false-positive surface *is* the design. "Bare
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

- **action-run-shell-scan-predicate** [needs-spec] —
  `check-action-run-shell`'s scan predicate is a **kit literal with no consumer
  seam**. The align ruling narrowed extraction to files carrying a top-level
  `jobs:` or `runs:` key, which is right for what gate-sdk claims, but it leaves
  a consumer whose shell genuinely lives in another CI dialect with no way to opt
  that surface in — CircleCI's `- run:` is the concrete case, and the extractor
  handles it correctly, so the reach is a policy choice, not a capability limit.
  **Deliverable:** a consumer-configurable scan predicate on the gate-sdk
  config-via-env pattern, defaulting to the Actions shape so **no consumer
  inherits a wider reach by accident**. The shape is the
  `check-graph` / `scripts/graph-vocab.sh` optional-consumer-config pattern the
  provenance seam already prescribes, so the precedent exists.
  **Why `[needs-spec]`:** it adds a knob to a gate that has none, and
  `gate-sdk/SPEC.md`'s no-new-knob claim for this gate would need re-deriving —
  a contract edit, not a script patch.
  **Cost while deferred:** low — the gate is correct and complete over what it
  claims, and the narrowing is documented. The residue is a consumer with
  non-Actions CI shell getting no coverage and no opt-in, which surfaces only
  when such a consumer adopts. Costed at roughly one small unit.
  Filed 2026-07-26 by close (`release-path-hardening`) from the scan-predicate
  bullet; deliberately out of that unit's envelope at align.

- **scratch-execution-allowlist-bar** [needs-spec] — the prompt-friction triage
  keeps surfacing one dominant class that must **never** be allowlisted, and
  nothing records why. This close's scan ranked `bash .tmp/<script>.sh` first by
  a wide margin (24 calls across 13 one-off script names, against 7 for the next
  pattern), and it will rank first at every close: writing a throwaway probe into
  `.tmp/` and running it is the repo's *sanctioned* scratch form — `bash-guard`
  actively steers sessions toward it, and `.claude/settings.json` already grants
  `rm .tmp/*`.
  **Why the obvious triage answer is wrong.** By guard-kit's criterion the
  pattern looks like a textbook allowlist candidate (safe, static, glob-matched,
  and already the form to reinforce). It is not. `.tmp/` content is by
  construction **agent-authored**, so the permission prompt on `bash .tmp/*.sh`
  is the single point at which a human sees agent-written code before it
  executes. Granting it would convert "every dangerous command prompts" into
  "write it to scratch and it does not" — a general bypass of every other guard
  in the settings, earned by one line.
  **Deliverable (enforcement-first, and this class is gateable):** a guard-kit
  assertion that reds a **committed** allowlist entry granting bare execution
  (`bash`, `sh`, `source`, `.`) of a path under an agent-writable scratch
  directory, plus the one-line rule in guard-kit/SPEC.md §The triage criterion
  that the gate enforces. Both halves land together, per Enforcement-first.
  **Why `[needs-spec]`:** the scratch-directory set is the design. `.tmp/` is
  this repo's binding, not a kit literal, so the gate needs the consumer seam the
  provenance seam prescribes (the `check-graph` / `graph-vocab.sh` pattern) — and
  the honest predicate may be wider than scratch (any agent-writable path,
  including a gitignored generated directory), which changes both the knob and
  the false-positive surface.
  **Cost while deferred:** low per close, but it is pure re-derivation — every
  close re-reaches the same conclusion from scratch about the same top-ranked
  pattern, and a close that reasons less carefully files the grant instead. The
  standing bar is currently held by nothing but each session's judgment.
  Filed 2026-07-26 by close (`release-path-hardening`), from its own
  prompt-friction triage; the local overlay carried no redundant entries to
  prune this iteration.

- **action-gh-repo-context** [needs-spec] — a workflow job that invokes `gh`
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
  **Why `[needs-spec]`:** placement and reach are undecided. It reads the same
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

- **native-gate-binary-port** [needs-spec] [roadmap: next/reliability] — replace the gate substrate.
  roadmap-summary: The gate battery as one native binary: no GNU userland, sub-second runs.
  Port the battery off bash-plus-GNU-userland onto a single native compiled
  binary (Rust the lead candidate), because every structural pain the current
  stack carries is substrate-bound, not fixable in place. The inventory today:
  76 check scripts, ~14.3k lines of shell across `*/checks`, `*/bin`, and
  `scripts/`, 85 registry entries. The pains, each already live: the platform
  reality is Linux-only — stock macOS ships bash 3.2 and a BSD userland
  (docs/install.md §Requirements states it outright) and Windows is WSL-only
  (`platform-support-ci-matrix` carries that half); the
  bash/git/jq/awk/sort/shellcheck assortment has independent release
  lifecycles, with cross-version workarounds already in tree; battery
  wall-time has grown from tens of seconds toward minutes on some batteries,
  most of it process-spawn cost rather than work; shellcheck is a linter, not
  a compiler, and bash offers no type system under 14k lines; and check
  source in the consumer's tree feeds the source-prediction anti-pattern —
  agents reading gate scripts to predict verdicts instead of running the
  oracle, a recurring token sink the delegation rules fight behaviorally
  rather than structurally.
  **Deliverable:** one multi-call binary (busybox-style, one subcommand per
  check), with `gates.list` resolution dispatching per-entry to binary
  subcommand or script so the port lands cohort by cohort, slowest and
  meta-gates first; each ported gate's `good/`+`bad/` fixture pair is the
  mechanical parity oracle before its script retires; consumer-authored gates
  keep the shell escape hatch, so registry-plus-shadowing semantics survive
  verbatim; per-platform release artifacts (Linux, macOS, native Windows)
  with published checksums and publicly buildable source; git the sole
  runtime dependency (shelled out, not embedded). Expected wins: the battery
  in the sub-second class, native macOS/Windows with no GNU-toolchain floor,
  the cross-utility version matrix gone, a real unit-test harness under the
  fixtures, and vendored binaries giving an agent nothing to read.
  **Why `[needs-spec]`:** the consumer-extensibility model is the design that
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

- **deferred-queue-carry-cost** [needs-spec] — the deferred section has grown
  to 56 entries and ~2,000 lines (~36 lines per entry), the arithmetic of an
  intake asymmetry the doctrine itself creates: gap disposition plus
  scope-gated intake make filing mandatory and cheap while the only exit is
  building the entry — demand-gated promotion exists, but no symmetric
  eviction. 21 entries self-describe their deferred cost as low and
  non-rotting — decision records carried as work items — and the tally the
  session banner steers by counts them identically with actionable debt.
  **Deliverable:** one carry-reduction unit: (a) an icebox tier for entries
  with low, non-rotting cost and no live promotion trigger — one line each,
  the full narrative left to the filing commit (derivation-first) — with
  demand-gated eviction symmetric to promotion (close triages entries past a
  defer-age threshold; an icebox entry re-filed on real recurrence is the
  mechanism working, not churn); (b) a queue-kit gate capping deferred-entry
  line count so the compression sticks (enforcement-first); (c) a
  filed-minus-closed net-delta KPI beside the existing defer-age drift
  metric; (d) the one-time triage sweep that seeds the icebox and cheaply
  *rules* on the `[needs-spec]` entries whose own text already admits a
  wontfix outcome (close as a one-line boundary note in the owning SPEC).
  **Why `[needs-spec]`:** the icebox's placement is the open design — a new
  queue section versus a separate file decides how check-task-conservation,
  check-queue-sections, and the slug-liveness grammar treat an iceboxed slug,
  and whether eviction is a conserved move or a sanctioned disappearance; the
  entry-size floor and the eviction age threshold are policy values needing
  an owner doc. A second open design rides the size cap: where filing-time
  design capital lives once the entry cannot carry it. Ruled at filing: the
  entry keeps the open questions (canon-kit's own few-line budget — its
  misfiling tell already names a long entry an amendment inlined where
  check-amendment-queue cannot see it); the filing commit body keeps the
  narrative and cost analysis, recoverable at authoring via
  `git log --grep=<slug>`, so nothing is discarded, only made non-resident;
  the owning SPEC keeps durable one-line facts; no ungoverned detail-file
  convention. Undecided, and the deciding lens is token economics: whether a
  governed `draft:` tag state between design-pending and spec-ready — a live
  pre-amendment file a deferred entry may point at — buys authoring a better
  starting point than the commit body does (pre-derived thinking reused, even
  slightly rotted, versus re-derived), against its anchoring risk on a moved
  codebase and the pairing/orphan gate semantics it needs — a new lifecycle
  state, not a tag reuse, since check-amendment-queue rules that a deferred
  entry carrying a `spec:` ref must be promoted. Decide at spec on triage
  evidence: entries whose discarded derivation would be genuinely expensive
  to re-derive. The size-cap gate's spec also mints the owner section for
  the entry-body field roster — the fields every entry carries (deliverable,
  open design, cost while deferred, filed-by) are today precedent-only,
  derived by imitating neighboring entries (kfric-stamped 2026-07-28), and
  the gate must enumerate them anyway.
  **Cost while deferred:** compounding with every filing — the queue is read
  by every scope session and curated by every close, and the intake rate
  (27 entries filed 07-25/26 alone) currently outruns closure with no
  counter-pressure; the filing threshold does not scale with queue size.
  Bounded: nothing reds; the cost is carry weight and a steering tally that
  overstates actionable debt. Debt-shaped at triage: converges the queue onto
  existing doctrine; the new governed names are the icebox tier, its gate,
  and the KPI.
  Filed 2026-07-28 by operator request, from a session assessing whether the
  queue's 30-to-56 deferred growth is healthy.

## Done

- front-door-outcome-rewrite
- public-roadmap-projection

## Lessons Learned
