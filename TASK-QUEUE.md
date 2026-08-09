# TASK-QUEUE.md — Checkwright work queue

## Iteration: prose-profile-activation

  The lifecycle-kit gates read this header's iteration name and the stage
  cursor — the last stamp in `.workflow/WORKFLOW-STATE.txt`
  (lifecycle-kit/SPEC.md §The state machine); queue-kit formalizes the queue
  format itself and gates this file. One iteration per hardening or roadmap
  unit; [README.md](README.md) maps the kits.

---

## New Features

- **prose-profile** [spec: SPEC-prose.md] [roadmap: now/ecosystem] — the fourth profile.
  roadmap-summary: A profile for documentation repos, where there is no build to gate.
  The `prose` profile — gate-sdk plus canon-kit — and what a first prose adopter meets at
  install. TRAJECTORY.md §PRIORITY DIRECTIVE sequences this next and fixes where its coherence
  is measured: at the adopter's floor, install then value then uninstall.
  Two premises this entry rested on are falsified by measurement, and `SPEC-prose.md` carries
  both with their witnesses. The profile parameter on `recipe_gates` is **not** the lever: `full`
  is the payload-derived maximum, so every profile is contained in it and anything a profile
  registers `full` must register too — a gate armed for one profile alone is not expressible,
  and the additive half is a disposition correction instead. And the cohort is not where the
  value is: 16 of canon-kit's 18 `on-surface` gates already pass on a bare install tree, but 12
  pass vacuously, because the manifest set is READMEs, canonical specs and the agent file and
  never `docs/*.md`.
  Six deltas: the profile rows and their criterion; a prune fix so a consumer's manifest set
  stops governing its dependencies' READMEs; eleven canon-kit gates moving to `zero-config` and
  seven staying put with reasons; a prose-shaped consumer that takes a real red; the install
  prose that still describes three nesting profiles; and the prune fix's own oracle, since
  neither this tree's battery nor the consumer smoke can serve as one.
  The acceptance oracle is the consumer smoke run for every profile, never this tree's battery —
  the prune is a no-op here, since this repo sets `CANON_KIT_SCAN_KIT_ROOTS=1`.
  Surfaced 2026-07-16 in the launch triage that scoped launch-readiness-gate; promoted
  2026-08-09 at spec.
- **install-queue-template-unreachable** [spec: SPEC-queue-seed.md] — the queue seed's owner.
  `queue-kit/templates/TASK-QUEUE.md` is unreachable at install by construction, and re-verified
  so for every profile that exists: `full` reaches canon-kit first and `delegation` reaches
  lifecycle-kit first, and neither ships the template.
  `SPEC-queue-seed.md` refuses both closes this entry offered. Deleting the template is now
  clearly worse rather than "not obviously worse", and an explicit owner declaration is new
  mechanism where a derivation is available: a kit that ships the template has already declared
  itself the format's owner. The arm stops selecting by kit and the seed hoists out of the
  per-kit loop, so payload order decides nothing here.
  The fact that refuses both is one neither close anticipated: the inline skeleton an adopter
  actually receives omits `## Lessons Learned`, which `QUEUE_KIT_REQUIRED_SECTIONS` requires by
  default — and `check-queue-sections`, the fail-closed floor under every section-scoped
  scanner, is one of four queue-kit gates not registered at install, so nothing says so.
  Filed 2026-08-09 by close (`install-profile-seam`); promoted 2026-08-09 at spec.
- **init-lifecycle-agent-block-seeding** [spec: SPEC-agent-block.md] — the agent-block rule.
  This entry asked whether `init` should seed lifecycle-kit's agent-file block, and said deciding
  which of its two outcomes held preceded any estimate. `SPEC-agent-block.md` decides it: the
  exclusion was considered, and the seeding is correct as it stands.
  The rule it states — **a kit's agent-file block is seeded at install iff a gate registered at
  install reads it** — derives both halves. doctrine-kit is seeded because
  `check-doctrine-registration` is `zero-config`; lifecycle-kit is not because no lifecycle-kit
  gate is, so the block would be resident always-loaded instruction for a stage machine the
  adopter has not adopted.
  So the deliverable is the small one: state the rule, cite it from the two producers, and
  correct `installer/README.md`, which offers a true statement about two gates as the reason for
  a posture covering eleven. The declined branch is costed in the gap inbox — seeding the block
  would owe a `--remove` mode `install-lifecycle.sh` does not have.
  Filed 2026-08-08 by close; promoted 2026-08-09 at spec.
- **dispatch-worktree-reds-the-battery** [spec: SPEC-worktree-prune.md] — one word.
  recurrence: dispatch-worktree-reds-the-battery 2026-08-08
  A live agent worktree under `.claude/worktrees/` is a full second copy of the repo that every
  tree-walking gate descends into, so the battery — the oracle every commit here requires — is
  dark for the whole duration of a read-only fan-out. Attested four times, at build, close and
  boundary stages alike, and unavoidable: the dispatch guard mandates the shape that causes it.
  `SPEC-worktree-prune.md` refutes this entry's own stated blocker by measurement. With a real
  worktree live the battery is 3 of 100 red; adding the basename `worktrees` to
  `GATE_SDK_PRUNE_DIRS` makes it 100 of 100 with the worktree still live. The entry reasoned
  that the prune could not name `.claude/worktrees` without taking `.claude/commands` and
  `.claude/agents` too — true of the parent, but the prune matches the **leaf**, and `worktrees`
  selects that directory and nothing else.
  Pruning `.claude` also passes 100 of 100, which is the trap the amendment records: that
  variant loses coverage silently rather than reddening, because the governed `.claude` markdown
  surfaces are read by explicit globs no prune touches.
  The real blocker was never named here: the knob **replaces** the default set with no additive
  form, so consumer config cannot express "the default plus one" without maintaining a copy that
  drifts. Hence four deltas — the default gains the member, the SPEC gains the reason and the
  coverage caveat, `lib-gate.test.sh` gains the discriminating case, and a
  `GATE_SDK_PRUNE_EXTRA_DIRS` append knob makes the placement ruling honest.
  Filed 2026-08-07 by close, from two rejected commits during its own audit dispatches;
  promoted 2026-08-09 at spec.
- **statusline-queue-section-counts** [spec: SPEC-queue-counts.md] — counters, one surface.
  Operator request: `TASK-QUEUE.md` section counts in the statusline as compact single-letter
  counters, the deferred one explicitly wanted.
  `SPEC-queue-counts.md` finds the requested set is *derivable* rather than a list — features,
  debt, deferred and icebox are exactly queue-kit's task sections, the set `lib/queue.sh`
  already composes for `QUEUE_TASK_RE`. So the counter enumerates nothing and the seam
  constraint this entry insisted on is satisfied by construction, not by discipline.
  Two nearer homes are refused with reasons. A further `bin/queue-index.sh` mode is refused by
  the owning spec itself, which fixes that tool's modes and rejects folding jobs together.
  Sourcing `lib/queue.sh` into the statusline is refused on measurement: the lib **exits 2 at
  source time** on malformed config, which would take down the entire status bar for a fault in
  a component contributing four characters. So the counter is a new one-job
  `queue-kit/bin/queue-counts.sh` the statusline calls as a subprocess.
  This entry's "two surfaces" premise is falsified. `.claude/settings.json` points `statusLine`
  at the template itself and project settings outrank user settings, so inside this repo the
  operator's live statusline **is** the template: one edit delivers the ask. The real second
  surface is the user-level copy, which is out of tree, ungoverned, already drifted, and runs
  only where there is no queue to count — filed to the gap inbox, not fixed here.
  Surfaced 2026-07-31, operator request; promoted 2026-08-09 at spec.
- **workflow-state-direct-edit-guard** [spec: SPEC-state-guard.md] — the uncommitted window.
  An uncommitted hand-edit to `.workflow/WORKFLOW-STATE.txt` moves the stage cursor for a whole
  session, while every gate that would catch it fires only at commit. This entry's four findings
  stand; it was held for design on two unsettled points, and `SPEC-state-guard.md` resolves
  both from the governing specs rather than by preference.
  **Ownership**: guard-kit's SPEC has already broken this tie for its own consumers — the kit
  owning the *rule* ships the guard and rides `lib/guard.sh`, and guard-kit mechanism moves only
  where the lib lacks a primitive. So lifecycle-kit ships the guard as the framework's fourth
  consumer, and guard-kit moves by exactly one clause: `guard_read_command` reads
  `.tool_input.command`, and a Write/Edit call carries `.tool_input.file_path`.
  **The settings surface is not pinned**, contrary to this entry: the pin roster carries two
  memory keys and never reads `.hooks`. What governs the block is a derivation — the enforcement
  map reads `.hooks.PreToolUse` to emit its Guards section and the value rollup joins that page
  — so registering the hook reds two freshness gates until both projections regenerate. That is
  a build instruction where "pinned" would have said do-not-touch.
  Seven deltas, and the amendment is cross-component: guard-kit, lifecycle-kit, and the repo's
  harness settings plus two generated projections.
  Surfaced 2026-07-31 by the close-entry refusal the same day, where `enter-stage`'s own refusal
  text openly offered a deliberate hand-stamp and the session declined on judgment alone — which
  is what this request wants to stop relying on; promoted 2026-08-09 at spec.

## Technical Debt

## Deferred

- **recurrence-drain-input-widening** [design-pending] — a recurrence with no bullet is uncounted.
  **Operator-ruled 2026-08-04, at the close of `ruling-capture-contracts`.** The drain that
  stamps the `recurrence:` declaration takes gap-inbox bullets as its sole input, which the
  amendment made the mechanism's single auditable producer. The open question is whether it
  should also stamp a recurrence the closing stage resolves for itself.
  **Grounds, from the iteration that built the mechanism:** `amendment-landing-citation-assertions`
  genuinely re-occurred — a build session found a false citation in an amendment the audit
  stage had passed as zero-divergence, which is that entry's own assertion (2) case and what
  its cost field predicted. It reached close as dispatch prose rather than a bullet, so close
  recorded dated prose evidence and no declaration. Faithfully captured, aggregated nowhere —
  the exact failure the counter exists to end, one channel over.
  **Why it needed a ruling rather than a fix in place:** widening the input widens what a
  machine writer may stamp onto the queue, and the single-producer property is what makes the
  count auditable. Both halves were deliberate, so which one yields is not close's to decide.
  **That premise moved, and this entry now owns the question it opened.**
  `gap-resolver-mention-overcount` back-tested the sole-producer claim against history and found
  it already false: of the nine commits that ever added a `recurrence:` date, three stamped one
  from outside the drain with no bullet in the same commit. So the drain is the only *mechanized*
  producer, never the only one, and the auditability the single-producer property was protecting
  has been re-based on same-commit inspection rather than re-derivation. Whether a direct stamp
  by a session that observed a recurrence outside the capture channel should be sanctioned,
  forbidden, or mechanized is inherited here by name and deliberately left open there — it is
  this entry's question, and the widening it names is now the live half of it.
  **Cost while deferred:** a recurrence observed anywhere but the capture channel misses both
  `kpi-incident-recurrence` and scope's pre-emption threshold, so the theme keeps outranking
  it silently — with the counter shipped, that silence now reads as evidence of no recurrence.
  **Second attested instance, 2026-08-05 at the `install-claim-contract` close.** The validate
  session ended its turn with `run-validate.sh` still running — a live re-occurrence of
  `validate-producer-liveness-unobservable`, with `dispatched-session-waiting-rule-residency`
  as the residency gap underneath it — and it reached close through `bin/kfric.sh` rather than
  a gap bullet, so neither entry's count moved. Recorded here as prose rather than stamped,
  which is the deferral behaving exactly as written and is why this reads as evidence rather
  than as a workaround.
  **Third close attesting, 2026-08-09 (`install-profile-seam`), and the first carrying an
  aggregate: THREE recurrences judged out of channel, ZERO in it.**
  `spec-measured-count-gate` re-occurred five times in one iteration — its own cost field
  predicts exactly that ("detection is by hand at align if at all") — and arrived as **lead
  dispatch prose**. `survey-edge-aggregation-residue` was observed while performing a rostered
  close audit, and `dispatch-worktree-reds-the-battery` as a battery red during the close's own
  commit. All three were declined a stamp on this entry's open question and recorded as prose,
  per its own rule. So a drain reading only bullets would have reported a clean recurrence
  count for an iteration carrying three — not a shortfall in the count, but the count reporting
  the opposite of the truth. Three iterations running, every recurrence the mechanism has
  actually seen is one it cannot count, and they have now arrived by four distinct channels
  (dispatch prose, `bin/kfric.sh`, a rostered audit, a battery red) — which is the argument
  that the gap is the *sole-input* shape rather than any one missing channel.
  Filed by the lead on operator direction under the direct-filing exception; the operator ruled
  file-a-unit over both keep-as-is and next-iteration-priority.

- **powershell-installer-surface** [design-pending] — a native Windows install path.
  The installer is bash end to end, so native Windows is unreachable:
  `native-gate-binary-port` states the limit in its own words — "`init` hands to bash, so
  Windows stays blocked" — and `platform-support-ci-matrix` covers Windows only as a
  **WSL** CI leg, which is a Linux userland wearing a Windows badge rather than a native
  path. No entry claims this ground today.
  **Ordered by the operator's trajectory pivot 2026-08-03**, whose objective 2 is every
  major OS including Windows and whose objective 6 is a script-interpreter surface that is
  minimal and dual-implementable — bash for Linux and macOS, PowerShell for Windows. The
  objective set is recorded in TRAJECTORY.md.
  **What the vendoring ruling already fixes the shape to, so this designs less than it
  looks like:** the bootstrap's whole job becomes resolve the platform, place the matching
  prebuilt binary, invoke it. Everything conditional lives on the far side of that invoke,
  and `install-step-relocation` is what moves it there. So this is a port of a three-step
  bootstrap, not of the installer as it stands — and its size depends on that relocation
  landing first.
  **Why it is design-pending:** two live shapes with different maintenance costs — two
  hand-kept bootstraps held in parity by a smoke leg, versus one bootstrap generated from a
  single declaration. Neither is obviously right at three steps, and the choice binds every
  later step that cannot move into the binary.
  **Cost while deferred:** the pivot's OS-reach objective stays unmet on the one platform
  it names that no current path reaches, and every install-path change is authored
  bash-first, which is the habit that made this entry necessary.
  Filed 2026-08-03 by spec, on the operator's trajectory pivot.

- **install-step-relocation** [design-pending] — move the install's shell steps into the binary.
  `installer/lib/init.sh` runs two consumer-side shell steps after it writes:
  `gen-pre-commit.sh --write` and `check-graph.sh --emit`. Each is a pure function of
  tracked text, and each is a natural subcommand of a binary the vendoring ruling puts on
  disk before either of them runs.
  **Ordered by the operator's trajectory pivot 2026-08-03**, objective 6 — the
  script-interpreter surface shrinks to the unavoidable. This entry is most of what makes
  it shrink: with these two relocated the bootstrap is resolve-the-platform,
  place-the-binary, invoke, which is small enough to be written twice.
  `powershell-installer-surface` is the entry that pays if this one does not land first.
  **Why it is design-pending:** both steps are **generated projections** with freshness
  gates and regen commands rostered in docs/site-architecture.md, so relocating them moves
  a generator without moving its gate — and whether that gate then invokes a shell command
  or a subcommand is a contract question `gate-authoring-sdk-surface` touches.
  **Cost while deferred:** the bootstrap keeps two steps that cannot be written in
  PowerShell without duplicating real logic, so the Windows path stays expensive for as
  long as this stands.
  Filed 2026-08-03 by spec; the pivot's objective 6 is what surfaced it.

- **instruction-surface-bash-focus** [design-pending] — the always-loaded surfaces assume bash.
  `CLAUDE.md` and the instruction surfaces beside it are written around a shell battery:
  the gate-authoring conventions, the fixture idiom, the housekeeping rules and the
  delegation guidance all name bash mechanisms as the default case. Under the trajectory
  pivot the default case becomes a native binary behind a minimal dual-implementable
  bootstrap, so those surfaces teach a shape the project is leaving. No entry claims this
  ground today.
  **Ordered by the operator's trajectory pivot 2026-08-03**; the objective set is recorded
  in TRAJECTORY.md, and objective 6 is what names this rewrite as needed without starting
  it.
  **Why it is design-pending:** the rewrite is not a find-and-replace. An always-loaded
  surface is costed per session, so the question is which bash specifics stay resident
  because they are still the common case, which move behind a load trigger, and which are
  deleted — and that depends on how far the port has actually got, which makes the trigger
  a threshold rather than a date.
  **Cost while deferred:** every session is oriented by a surface describing the substrate
  the project is moving off, and the correction is paid per session in re-derivation rather
  than once in an edit.
  Filed 2026-08-03 by spec; the pivot names this rewrite and does not start it.

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
  Cross-vendor stage dispatch: a lead delegating a stage to a foreign coding agent,
  extending the homogeneous multi-agent model to a heterogeneous fleet. It cashes the
  public no-lock-in claim and is the purest expression of the thesis — governance enforced
  at the git/gate boundary, not by trusting the author. *Already agent-neutral:* the
  verification substrate (git, the gate battery, the bash stamp state machine) does not
  care who authored the diff, and the coordination primitive is the shared git-index/HEAD
  serialization. *Homogeneous today — the real work, worst-first:* (1) the **escalation
  resume model** collapses into (2) as a property of the chosen transport, per the 2026-07-25
  amendment below; (2) **dispatch transport** — today the harness
  `Agent`/`SendMessage`/task-notification; a foreign agent needs a transport-neutral
  handoff. The adapter contract is "open / prompt / permission-request / resume" spoken over
  each vendor's structured **machine plane, never its TUI**: a screen-scrape relay is the
  adapter of last resort for a vendor shipping no machine interface at all — it yields
  rendered frames not turn events, answers dialogs by heuristic, and bets on the vendor's
  least-stable surface. (3) **budget oracle** — the verdict tool is
  Anthropic-OAuth-specific; a heterogeneous fleet has N vendor-keyed oracles, the same seam
  as the credential-swap entries, and the vendors' JSONL event streams carry the token-usage
  events a TUI path would scrape from a status bar. (4) **stage-contract expression** — the
  lifecycle machinery is neutral bash but the stage-skill prose is not.
  **Seam ruling (on record):** generic mechanism only — transport, budget oracle, and
  escalation channel become consumer-config seams; a kit literal naming a vendor crosses
  the provenance seam and is ruled out, the `prose-profile` pattern. It extends the
  per-batch model-tiering lever across vendors, and interacts with
  `hosted-attestation-service`, `plugin-marketplace`, and the credential-swap entries.
  **Demand-gated — demand attested (2026-07-23):** the operator holds working
  foreign-vendor subscriptions and wants read-heavy delegation routed to them for budget
  headroom, and with three vendors live the N-keyed oracle seam is no longer hypothetical.
  First slice at promotion: a foreign-CLI executor for the already-pre-authorized
  read-heavy audit / mechanical-sweep class over a spawned non-interactive CLI process,
  one adapter per vendor as consumer config — not full stage dispatch.
  Promotion-eligible at the next scope session.
  **Design-memory amendment (2026-07-25):** the TUI-relay alternative was probed for
  session resume and token efficiency. Ruling: those benefits live in the vendor's session
  store, not the TUI — the APIs are stateless and both modes replay the same on-disk
  transcript against the same server-side prompt cache, so interactive-vs-headless is a
  rendering choice, not a state choice. Headless warm-resume by session id and JSONL turn
  events ship today on the vendors probed, which is what makes (1) plumbing.
  **Verification capability (2026-08-02):** those probes ran against **installed binaries**
  — the foreign CLIs are present on the development machine — so the executor is verifiable
  rather than inferred from vendor documentation. Under oracle-first that is a change to
  the unit's risk, not a convenience, and it sharpens the first slice: the executor ships
  with a smoke that actually invokes them, the shape every kit already uses. The machine
  profile (context-kit/SPEC.md §bin/env-probe, local-only) owns which CLIs and how.
  **Cost while deferred:** the foregone lever is live — read-heavy audits and mechanical
  sweeps all bill against one vendor's budget while three subscriptions are held — and
  this design memory ages against fast-moving CLIs.
  Surfaced 2026-07-17 in the release-in-lifecycle lead session (operator question).

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
  never blocks (delegation-kit/SPEC.md §usage-verdict, which also states the
  server lag the next point turns on), so this is signal quality, not a
  dispatch-blocking defect. Two
  points: the window default is 600s while the SPEC's own stated server-lag is
  "about a minute", a ~10x margin worth tightening; and it is a
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
  **Carry-over (1) discharged 2026-08-01 by the undirected scope survey:**
  `stage-economics-honesty` has landed, attribution fix included, so the rows are
  no longer mis-attributed and this entry is **pickable with nothing left
  blocking it**. Only carry-over (2) — the unset `≥N` — stands.
  **Cost while deferred:** one queue line; the backlog-aging review re-raises it
  every iteration until the data exists to run it. Filed 2026-07-19 by lead
  ruling at the `stage-posture-split-tuning` close — the split shipped on
  projected economics, and this is the loop that confirms or retires that
  projection with recorded data.

- **build-stage-tier-economics** [design-pending] — measure whether the `build`
  stage downgrades from Opus to Sonnet net-positive rather than flipping on
  intuition; a ruling-config tier re-judgment (`.claude/agents/stage-session.md`
  / the lead template's ruling-config, which invites re-judging every tier).
  Grounding, **corrected** against priced rows: the split-lead spread is build
  $2.59–10.96, close $5.83–7.81, validate-on-Sonnet $0.54–1.44, and neither
  reading it yields matches the original grounding. **Close is comparable to
  build, not an order of magnitude below it** — so close is a tier candidate in
  its own right, arguably ahead of build, and the premise that build is the
  single highest-value lever no longer holds. And the already-adopted
  validate→Sonnet downgrade **demonstrably works**: validate is the cheapest
  stage by a wide margin with no observed quality cost, which is the affirmative
  precedent this A/B is testing for build. Re-read the figures from the log
  rather than trusting them as transcribed, and read the `cr` column rather than
  `cost` — a falling per-token rate makes a growing draw read flat in dollars.
  The superseded token-only reading is in history.
  **One design blocker remains; the first two are answered.** Both the
  uninstrumented metric and the mis-attributed rows are **discharged** — a price
  table exists, the meter prices instead of reporting `cost=n/a`, and
  `stage-economics-honesty` landed the one-transcript-one-row attribution fix on
  2026-08-01, so this task is **unblocked** and self-labelled Debt below. What
  stands: the
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
  **Datum, `native-cohort-activation` (2026-08-07): the per-batch lever paid nothing.** Every
  delta of all four units was classed before dispatch; each unit carried at least one
  design-bearing delta, so no batch was downgradeable and all four rode Opus. That is evidence
  *for* the per-build-class rule above — the classification was cheap and correctly returned
  *no* — but it warns that a whole iteration can be design-bearing, so the lever's expected
  value depends on the mix a scope cuts, which the A/B must sample rather than assume.
  **Cost while deferred:** low and non-rotting — validate's adopted downgrade
  already banks the affordable half of this lever, and the prerequisite that
  made the rows provisional has now landed; the residue is that
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
  **Why deferred rather than built alongside the floors:** the repo runs three
  workflows (`.github/workflows/gates.yml`, `publish.yml`, `site-health.yml`), none a
  matrix, so this is new runner spend on macOS and WSL images — and **no macOS
  or WSL adopter exists** to attest the spend. Demand-gated on exactly that,
  like the other adoption rungs.
  **Re-verified 2026-08-02 at scope, and a second un-defer trigger named.** The
  workflow count moved (two → three); the substance is unchanged — still zero
  non-Linux legs, no `matrix:` key anywhere, and the adopter trigger unfired. The
  second trigger: **`native-gate-binary-port` reaching distribution** — refined the
  same day, because "prerequisite" first stated it too strongly. The two are
  *different jobs* sharing only runner spend: that entry needs binary build+smoke
  legs in `publish.yml`, this one specs install-smoke of the bash battery. So
  neither blocks the other's start, and the port's per-platform obligation attaches
  to its artifacts clause — the release boundary, not the first ported cohort.
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

- **spec-measured-count-gate** [design-pending] — a **measured count or extent claim
  authored into governed prose goes stale with no oracle**.
  **Eight instances. Three before 2026-08-09:** the align audit found
  `gate-sdk/SPEC-action-run-shell.md` B2's block count wrong twice over (five, then six;
  the tree carried eight), slated for `gate-sdk/SPEC.md` where a sibling unit falsified
  it on the next commit; and a third landed in a **binding shim**,
  `.claude/commands/close.md`'s release-policy slot deriving the bump off the note's "two
  sections" where the cited owner fixes three, read by a close while deriving a bump.
  Each was corrected by hand — the Enforcement-first shape the doctrine bars, the fix
  landing without the gate that catches the next one.
  **Gap generalization — the check class that should have caught them.** `check-prose-enum`
  reads governed-set membership, not numerals; `check-spec-derivable-section` reads
  fenced-dump density; `check-shim-restatement` holds *copy shape*, and a restatement that
  is **wrong** has diverged from its owner's wording, which is what makes it not a copy. So
  the scanner must range over SPEC sections and binding shims alike.
  **Widened 2026-08-09, and the widening answers both grounds this entry left open.**
  Five more instances in one iteration (`install-profile-seam`), split by whether a
  cardinal-based scanner reaches them. **Three do:** nine gate-name lists that were seven,
  a "sole call site" that was two, seven marker hits that were eight. **Two do not,
  carrying no cardinal at all:** a five-line finding generalized to a whole-file claim,
  and an audit stamping "the kit SPECs came back clean" over a SPEC holding two live
  counter-instances. Those are *extent* claims and they fail identically — stated wider
  than the measurement behind them.
  **Answered: the unreachable axis is the one that costs.** Seven of the eight instances
  were caught downstream by hand at no cost beyond the re-measurement. The eighth — an
  extent claim — was caught by **nobody** for two iterations and shipped a false sentence
  into a published SPEC and its public docs mirror. A cardinal-only trigger catches the
  free half and misses the paid half.
  **Answered: the cheap discharge is insufficient, closing the ground raised 2026-08-01
  at scope.** `canon-kit/checks/check-manifest-count.sh` bans bare cardinals over governed
  collection nouns and takes an extensible `CANON_KIT_COUNT_NOUNS` list, so a `couples=`
  widening (its globs never reach `.claude/commands/*.md`) plus a noun override looked
  like a discharge. It cannot be: banning cardinals cannot reach a claim carrying none.
  Worth taking for the cardinal axis; it does not close this entry.
  **Why `[design-pending]`:** the false-positive surface *is* the design, and the extent
  axis sharpens it rather than settling it. "Bare cardinal near a roster noun" over-matches
  legitimate prose ("the four contracts", "both halves"); "extent claim over a corpus" has
  no syntactic tell at all. Both instance sets argue for an opt-in `measured:`-style marker
  the author applies over a scanner inferring intent — a smaller gate bought with a larger
  authoring contract, and that trade is the unit's to rule.
  **Cost while deferred:** compounding and silent — this recurs at **every amendment that
  measures the tree**, and the failure mode is a canonical doc asserting a false number.
  Detection is by hand at align if at all, and that phrasing is itself a claim about the
  *caught* set, which is what a survivorship count cannot see past.
  The session-act half is `audit-class-corpus-attestation`, deliberately not folded in:
  that entry designs a **stamp** obliging a sweep to record the corpus it read, making an
  extent claim falsifiable when written; this designs a **scanner** over authored prose.
  Filed 2026-07-26 by close (`release-path-hardening`), draining the
  stale-measured-count bullet; costed at roughly one small unit.

- **gate-battery-parallel-execution** [design-pending] — `run-gates.sh` runs the battery
  serially: no `xargs`, no `&`, no `wait`. Measured after the spawn-hoist unit
  landed: 23718ms/90 gates, of which `check-shellcheck` alone is 5921ms. Spread
  across cores the remaining gates sum to well under that, so **`check-shellcheck`
  becomes the critical path** and the battery floors at roughly its cost —
  splitting its corpus across workers is what breaks the bound, not the scheduler.
  **Premise dated 2026-08-02 at scope, twice over.** `gates.list` now registers 94,
  and no timing artifact survives in the tree — `.metric/` holds economics and usage
  only — so the figures above are re-measured before they are built on, not trusted
  as transcribed. The critical-path *shape* is unaffected by the count.
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

- **contributor-writeback-disposition** [design-pending] — operator-directed: every
  GitHub boundary-sweep disposition must write back to the contributor, and the
  comment must be appreciative — explicitly **including** the discard cases. A
  contribution declined without visible thanks spends community goodwill the
  project cannot refund. Current state in this repo's scope shim (the kit template
  carries no GitHub sweep, so this is local policy, not a kit change): nine
  dispositions across three lanes, only three write back at all, and none specifies
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
  **Premise corrected 2026-08-01 at build:** `security-advisory-lane` landed, so
  the old exclusion ("an unswept third lane") is void and the design pass owes
  the Advisories lane an explicit ruling. Do not assume same-treatment — its
  dispositions are stated on the private advisory thread until publication, so
  write-back lands on a different surface and audience, and two of its four
  (*advisory-only*, *declined with cause*) are discard cases in this sense.
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
  **Third instance, 2026-08-02, and it widens the class past READMEs:** `CLAUDE.md`
  §This repo enumerates the generated projections while docs/site-architecture.md
  §Generated projections is the declared roster. So the class reaches the
  **always-loaded** surface, where a stale enumeration is paid by every session
  rather than by a reader who opens a README. Left unfixed on purpose: correcting
  the copy is the Enforcement-first shape this entry already names, and a
  `generated-projection` enum set is exactly the kind the survey above must cost
  first.
  **Re-read at the next close (2026-08-02) — the copy is short by two classes,
  and the second one is a drift event rather than an omission.** Beyond
  `ROADMAP.md`, which the Housekeeping bullet below re-attaches by hand, the
  enumeration omits the **trajectory projection** (`docs/evidence-data.md`)
  outright: it entered the site-architecture roster at `5cfc477`, while CLAUDE.md's
  paragraph was last touched at `965d208`, which predates it. The copy has
  therefore already drifted by the exact mechanism this entry predicts — a roster
  gaining a member the copy never heard about — with no oracle between the two
  events. That is the recurrence the enum-set survey was waiting on.
  **Cost while deferred:** low and non-rotting, but paid once per close — the
  staleness read is the only detector, so every roster is held by a session's
  attention rather than by a gate. Raised by the third instance: one of the
  drifting rosters sits on the surface every session loads, and has now drifted.
  Surfaced 2026-07-31 by close's top-level staleness review, which found the first
  two; filed rather than fixed because the enum-set survey is the work.

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
  whoever drains it.
  recurrence: gap-bullet-premise-verification 2026-08-07
  Two of the three bullets re-verified at this boundary were
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
  gate. **That line states the unit's size, not its class** — on the shipped
  path it mints a script name and a `scripts/gates.list` registration, so
  canon-kit/SPEC.md's new-names litmus makes it a **feature** owing an
  amendment. It promotes into a feature section; only the
  assertion-on-an-existing-gate variant would be debt.
  Filed 2026-07-31 at close as the gap-generalization owed by the inline fix
  above; the `check-md-refs` blind spot and the `AGENTS.md` false-positive case
  were both verified against source before filing.

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
  Size: one gate plus fixtures.
  **Feature-shaped — self-label corrected 2026-08-04 at close.** This line read
  "Debt" while naming the governed gate name it mints; canon-kit/SPEC.md §The
  amendment lifecycle's litmus makes that a feature, so promoting it authors an
  amendment.
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
  Size: one existing gate widened plus a pair registry.
  **Feature-shaped — self-label corrected 2026-08-04 at close**, on the same read as
  its `workflow-permissions-scope-oracle` sibling: the label read "Debt" while naming
  the consumer knob it mints, and canon-kit/SPEC.md §The amendment lifecycle's litmus
  makes any such name a feature, so promoting it authors an amendment.
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
  **The series, against the same unmoved baseline: 197 (+25) → 213 (+41) → 213 (+41).**
  The middle reading advanced 16 lines and looked like compounding drift. The third,
  at `native-cohort-activation`'s close, is the first that did **not** advance — and
  the mechanism is worth more than the number: the iteration added exactly one line to
  the surface and that close's pass recovered exactly that one line, both times from
  the same `native/` bullet restating what gate-sdk's SPEC owns. So a pass-shaped fix
  holds the line **only** against a pass-shaped increment. It recovered one of
  forty-one when the growth was unit-shaped, and one of one when it was not. The
  baseline stays unstamped through all three.
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
  **Practice confirmed correct once, 2026-08-01.** This iteration's two build
  batches each wrote their own stamp — two `build` lines with distinct session
  ids in `.workflow/WORKFLOW-STATE.txt` — so a per-batch trail is achievable
  under the instruction both templates already carry, and the fork below is a
  live choice rather than a repair. A second limit confirmed alongside it:
  `bin/enter-stage.sh --simulate` **cannot predict a distinct session's entry**,
  so it never answers whether a sibling batch will stamp — it gates *this*
  session's entry only, which bounds any lead-side pre-dispatch check.
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
  **Regressed 2026-08-02:** nine build batches, one stamp — the once-correct
  practice did not hold, and since economics attributes one row per transcript,
  the lead binding's per-batch tier lever is unmeasurable while the trail is not.
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

- **upgrade-contract-rename-routing-unstated** [design-pending] —
  `docs/install.md` §The upgrade contract never states that a **gate-name**
  rename or a **file/directory-convention** rename routes to the note's
  "Behavior changes" section. The section's bullet-lead definition structurally
  admits both, and the mapping paragraph covers a *consumer's* residue classes
  rather than this project's own renames, so the routing is inferred and no
  release note to date instantiates it.
  **What leans on it.** lifecycle-kit's knob-rename compat clause
  (`knob-rename-compat-threshold`, Delta 3) rests on that routing to establish
  that its knob-scoping leaves no rename class without a home — the check that
  licenses filing no gap for the widening question. That delta was softened at
  align to state the inference *as* an inference; this entry is what would make
  the reasoning stand on something stated.
  **Why `[design-pending]`:** stating it changes what the upgrade contract
  asserts to consumers — note-grammar semantics rather than wording, so a scope
  call rather than a close fix.
  **Cost while deferred:** low and non-rotting — the routing is admissible today
  so no note is wrong; the residue is a downstream clause leaning on an inference.
  Class: **feature** if it states a new routing obligation, debt if it only makes
  an admitted one explicit — canon-kit/SPEC.md's new-names litmus decides at
  promotion.
  Filed 2026-08-01 at close from the gap inbox; align filed it after softening
  the delta that leans on the routing.

- **release-note-section-set-derivation** [design-pending] — the release note's
  fixed-section set is held by `scripts/check-release-bump.sh` as parallel
  hardcoded calls, so a fifth section is added by copying a call rather than by
  extending one roster, and **no gate asserts that the gate's set equals the
  page's**.
  **The disease this is the residue of.** The section *count* was hand-maintained
  as a literal across `gate-sdk/bin/upgrade-smoke.sh`,
  `scripts/check-tightened-gates-grammar.sh`, `check-release-bump`'s spec
  comment, `gate-sdk/SPEC.md` and several `docs/install.md` sentences, while the
  set itself was separate hardcoded calls with no array and no length. Nothing
  derived the number and nothing reddened when it was wrong — the test "if this
  number were wrong tomorrow, does anything red?" answered no at every site. It
  had already rotted: adding one section took a by-hand three-to-four edit at
  every site, and align's census of those sites — itself a hand roster — omitted
  `upgrade-smoke.sh`, which build's re-run caught. **The census only had to exist
  because nothing derived it.** The prose half is closed: every count is gone and
  the prose now names two *classes*, with `docs/install.md` §The upgrade contract
  owning the roster by enumerating it.
  **A ruling that must not be re-derived.** The obvious consolidation — put the
  set in `gate-sdk/lib/declaration.sh` beside the shared container — is **ruled
  out**: those section names are consumer content, and a kit literal carrying one
  project's release-note vocabulary crosses the provenance seam (CLAUDE.md §The
  provenance seam), the same ground on which the note-parity gate stays out of
  gate-sdk. So the unit is consumer-side: one rostered set in this repo's
  `scripts/`, derived from or gated against the page.
  **Why `[design-pending]`:** the seam ruling fixes *where*, not *how* — derive
  the gate's set from the page at runtime, or hold both and gate the parity. The
  two differ in fail-closed behavior when the page is unreadable, which is the
  open call.
  **Cost while deferred:** a fifth section is added by copy, and the gate's set
  can silently diverge from the roster the page owns — the exact shape that
  survived a dedicated audit stage once already.
  Debt: converges a duplicated set onto one roster; adds no governed name unless
  the parity arm becomes its own gate, which the promoting scope call settles.
  Filed 2026-08-01 at close from the gap inbox, filed by this iteration's build
  on an operator-raised doctrine check.

- **md-refs-tree-link-resolution** [design-pending] —
  `scripts/gen-docs-mirror.sh` emits a `/tree/` form for off-root **directory**
  targets, and `check-md-refs`' self-repo pass recognizes only the
  `/blob/<ref>/` prefix — so a `/tree/` link falls through to the external-URL
  skip and is resolved by nothing.
  **Not a live defect:** the generator's output is correct by construction, so no
  `/tree/` link is wrong today. A coverage hole rather than a violation, and out
  of `docs-root-link-grammar`'s boundary because that rule fires on **relative**
  links while a `/tree/` link is absolute and already converted.
  **Why `[design-pending]`:** the likely fix is one more prefix arm in the
  self-repo pass, whose identity derivation and pinned ref
  (`CANON_KIT_DOCS_BLOB_REF`) already exist — but confirm first whether a
  `/tree/` target should resolve to a *directory* in-tree, since that is the one
  case `check-docs-link-convention`'s directory-target rule deliberately routes
  elsewhere. Two gates disagreeing about directory targets is the design
  question; the missing arm is not.
  **Cost while deferred:** low and non-rotting while the generator is the only
  producer; the hole opens the moment a `/tree/` link is hand-written.
  Debt: converges a resolver on a link form its own generator already emits,
  minting nothing.
  Filed 2026-08-01 at close from the gap inbox, filed by this iteration's build,
  recorded in `docs-root-link-grammar`'s Producers-and-consumers section rather
  than flagged-and-skipped.

- **stage-stamp-ordering-unenforced** [design-pending] — `check-stage-evidence`
  accepts a stage stamp that lands **after** commits already made under it, so
  the stamp proves invocation but not that it preceded the work it authorizes.
  recurrence: stage-stamp-ordering-unenforced 2026-08-07
  **Observed with the battery green throughout.** This iteration's build batch 1
  stamped `.workflow/WORKFLOW-STATE.txt` as its *third* commit, after two commits
  had already landed build-stage edits under that unstamped entry. Nothing caught
  it — not `check-stage-evidence`, not `check-stage-entry`, not the pre-commit
  hook. Batch 2 stamped first and **the difference was invisible to every gate**,
  which is the point: the ordering is session discipline with no oracle, while
  `lifecycle-kit/templates/stages/build.md` states the stamp as the "First step"
  and says to commit it on its own. The prescription exists; only enforcement is
  missing.
  **The mechanism, run rather than observed (folded in 2026-08-02 from the gap
  inbox; the text above states the symptom, this states the cause).** Both gates
  are path-coupled in the generated pre-commit hook — the couple set is
  `TASK-QUEUE.md` and `.workflow/WORKFLOW-STATE.txt` — so a work commit touching
  only kit sources never runs them at all. They are not lenient about ordering;
  they do not execute. The stamp commit is the first commit that runs them, and by
  then the work they were meant to gate is already in history. On the full-battery
  path they do run, but `check-stage-entry` reads point-in-time state only:
  assertion C scans the amendment files and stamp set **as they are on disk**, with
  no read of history, so a stamp that landed last is byte-identical to one that
  landed first. The consequence generalizes past ordering — **every entry-time
  assertion, assertion C's demand for a prior audit-stage stamp included, is
  satisfiable retroactively**, because the gate cannot distinguish "align ran
  before this build" from "align's stamp exists now". It also opens a **cheaper
  candidate than the history assertion below**: widen the two gates' couple set so a
  stage's own output surfaces re-fire them, turning a never-ran gate into a
  ran-and-lenient one first.
  **Candidate shape to weigh at design:** assert that the commit introducing a
  stage's stamp is not preceded, within the same stage window, by commits
  touching that stage's own output surfaces — decidable from git history, though
  the surface set is the hard part. A cheap approximation misfires on a
  same-stage re-entry, where a second session's stamp legitimately follows the
  first session's commits; that case is **confirmed real** this iteration, not
  hypothetical (see `batch-split-stamp-ownership`). A narrower and possibly
  sufficient form: assert only that the **first** stamp for an iteration+stage
  pair is not preceded by non-stamp commits since the prior stage's stamp.
  **Why `[design-pending]`:** it narrows how a shipped evidence contract is read,
  and the surface-set question has no cheap answer.
  **Cost while deferred:** the stamp protocol's central claim — that a stamp
  marks a boundary the work happened after — is unattested, and a violation is
  invisible in a fully green battery.
  Class: mints a gate name if the oracle lands, so canon-kit/SPEC.md's new-names
  litmus makes it a **feature** on that path; debt only if it lands as an
  assertion inside `check-stage-evidence`. The promoting scope call settles it.
  Filed 2026-08-01 at close from the gap inbox; build filed it against its own
  batch-1 stamp.

- **amendment-deletion-content-completeness** [design-pending] — a closed
  amendment can be **deleted with part of its content landing in no canonical
  spec**, and the merge rule has no completeness oracle at deletion time.
  **Observed, not hypothetical.** `SPEC-supply-chain-trust-baseline.md`'s
  causal-chain rationale (its A1) survives only in git history, so the next
  amendment on that surface **re-derived** it rather than inheriting it — the
  re-derivation is the cost, paid in full.
  **Why `[design-pending]`:** spec-over-precedent makes git history a
  non-canonical tier, so "recoverable from the deleting commit" is not an
  answer here the way it is for an icebox eviction — an evicted queue body is
  dormant work, whereas a merged rationale is *live doctrine* a later reader is
  entitled to read forward. The oracle is the hard part: deciding that every
  claim in a deleted amendment landed somewhere canonical means diffing prose
  meaning, not tokens. Cheaper shapes to weigh first — require the deleting
  commit to name each destination section per amendment section, or forbid
  deletion outright and require an explicit `merged-into:` field the gate can
  resolve.
  **Cost while deferred:** unbounded and silent — each deletion may drop
  rationale, and the loss is invisible until someone re-derives it, which is
  exactly what happened here and cost this iteration a spec cycle.
  Filed 2026-08-01 at close from the gap inbox.

- **advisory-lane-draft-state-unswept** [design-pending] — the Advisories lane
  probes `state=triage`, which is correctly the **undispositioned** set:
  accepting a report moves it to `draft`, so `triage` is exactly what nobody has
  ruled on yet. But a "fix under embargo" disposition is **ongoing rather than
  terminal-in-practice** — the advisory sits in `draft` until publication, and
  no lane looks at `draft`, so an embargoed fix that stalls is invisible to
  every later boundary sweep. The lane closes the intake hole and leaves a
  follow-through hole one state downstream.
  **Options at design:** add a second probe for `state=draft` with a
  re-raise-only disposition, or rule that the thread's notifications are
  sufficient and record that ruling.
  **Cost while deferred:** low and bounded — GitHub keeps notifying maintainers
  and the thread is the work record (the same honest limit the lane's own
  preamble states), so this is **unswept rather than unwatched**; the residue is
  that nothing systematically re-raises a stalled embargo.
  Surfaced 2026-08-01 at build while deriving the state literal from GitHub's
  documented enum (lead ruling R3); filed at close from the gap inbox.

- **path-pinned-allow-entry-oracle** [design-pending] — the **gap generalization
  owed** by close's ruling that a standing allow entry naming a *script path* is
  not content-pinned (guard-kit/SPEC.md §The close-stage triage step, landed
  this close). The criterion now exists in prose and the three instances were
  pruned, but **nothing reds the next one**: the shape reads as a specific
  literal command until a reader notices the target sits in a writable,
  gitignored dir, and the two mechanical reports
  (`compare-settings-allow`'s redundancy and breadth sets) both returned empty
  on exactly these entries, so neither detector reaches the class.
  **Missing check class:** a settings-allowlist scanner asserting that no
  `Bash(...)` allow entry names a path under `GATE_SDK_TMP_DIR` unless the
  executing command is `bin/scratch-run.sh` — mechanically decidable from the
  settings JSON plus the scratch-dir knob, and it is the same reader
  `check-settings-pins` already has.
  **Why `[design-pending]`:** the boundary is the work, not the scan. A grant
  naming a *tracked* script is content-pinned by review; one naming an
  untracked-but-stable path may be either; and the rule should plausibly reach
  the committed `settings.json` as well as the local overlay, which is a
  policy question about the consumer's own file. Whether this is a new gate or
  an assertion inside `check-settings-pins` also settles its class.
  **Cost while deferred:** low per instance and detectable at close, but the
  detector is a session's attention — the reason this was filed as a criterion
  question rather than a removal request is precisely that removing instances
  without an oracle re-arms.
  Filed 2026-08-01 at close, as the gap generalization for the inline fix.

- **price-table-roster-coverage-oracle** [design-pending] — **the missing check
  class behind a price-table row that was absent for roughly ten iterations.**
  The instance is fixed: the consumer price table carried no row for the model
  class every stage but `validate` had ridden since the
  `supply-chain-trust-baseline` iteration, so every Opus stage row and every
  supervision row across that span priced to `cost=n/a`; the row was
  transcribed and the rows re-priced at this close. The meter degraded exactly
  as specified (drift-kit/SPEC.md §The stage-economics meter, the
  incomplete-pricing caveat) and reported the caveat, so nothing was ever wrong
  with the tool — the table was stale in a dimension its freshness KPI cannot
  see. `kpi-price-table-age` reads only the two dating headers and never a row
  (drift-kit/SPEC.md §Bundled KPIs), so a table whose `priced-as-of:` and
  `prices-valid-through:` are both current reads healthy while the roster
  underneath it has fallen behind the models actually running. **Nothing reds
  the next one** — that is the whole of what remains here.
  **Missing check class:** an assertion that every model id appearing in the
  trend log has a row in the price table. The oracle question is where the
  observed roster is read *from*: the trend log lives under the gitignored
  `DRIFT_KIT_METRIC_DIR`, so a gate cannot read it on a fresh clone or in CI,
  and the transcripts it derives from are outside the tree entirely. That
  pushes the check toward a KPI (advisory, runs where the metric dir exists)
  rather than a gate — which is a placement ruling, not a scan.
  **Why `[design-pending]`:** the seam. A kit literal enumerating model ids
  would publish the consumer's model roster, the same provenance boundary that
  made the price table consumer config in the first place, so the check must
  derive the roster from consumer-side data and can assert nothing about which
  models *should* appear. Whether that is a drift-kit KPI beside
  `kpi-price-table-age`, an assertion inside the meter's own run, or a third
  dating header naming the priced roster is the open question.
  **Cost while deferred:** low *now* and re-arming. With the row transcribed the
  cost column is live again, so the carrying cost is no longer the blank field —
  it is that the next roster churn reproduces the same ten-iteration blind spot,
  and the churn is not under this repo's control. The detector in the meantime
  is a session noticing `cost=n/a` in a report, which is exactly how this one
  was found: late, and only because someone read the output closely.
  Surfaced 2026-08-01 by the `/economics` run at close, whose entire Opus-side
  cost column degraded; the row landed the same session, and the entry was
  re-scoped from the instance to the oracle it still owes.

- **economics-posture-binding-stale** [design-pending] — the `/economics`
  consumer shim binds its posture slot by **restating** the model posture
  ("every stage rides Opus") rather than citing the surface that owns it. That
  ruling has since moved: the lead shim now records a **Split** posture —
  `validate` dispatched with a `sonnet` override, `build` tiered per batch, the
  remaining stages on the Opus default — and the measured rows confirm the tree
  runs the split, not the restatement. So the report's verdict slot asks the
  reader to judge a posture the repo no longer runs, and the reader has to
  notice the contradiction from the data to avoid answering the wrong question.
  This is a de-literalization defect in a consumer binding: the binding's job is
  to name *where* the ruling lives so the report reads it live, and a binding
  that inlines the ruling's content is a second copy that drifts silently.
  Rewriting this one binding is mechanical; the class is what needs a design.
  **Missing check class:** an assertion that no consumer command shim restates a
  ruling another shim owns. Both shims are `.claude/commands/*.md` binding
  blocks, so the surface is small and already in the tree, but the predicate
  ("restates" versus "cites") is not mechanically decidable in general — the
  tractable form is narrower, something like: a binding whose prose names
  another shim as the owner must not also assert that ruling's content.
  **Why `[design-pending]`:** the decidable predicate is the work. A binding
  legitimately summarizes to give the template a usable slot value, so a check
  that reds on any overlap would red on correct bindings; one that reds only on
  exact restatement would have missed this instance, since the drift was a
  *changed* ruling rather than a copied sentence. Whether the durable fix is a
  check at all — versus a rule that a binding slot may only carry a pointer,
  enforced by the template's own slot grammar — is the open question, and it
  reaches lifecycle-kit's binding contract, not just this repo's shims.
  **Cost while deferred:** low per instance but silent, which is the bad shape.
  Nothing reds; the report simply answers a stale question, and the staleness is
  visible only to a reader who cross-checks the binding against the live ruling.
  It re-arms on every posture change, and posture is re-judged whenever the
  harness model roster churns.
  Surfaced 2026-08-01 by the `/economics` run at close, when the priced rows
  contradicted the posture the binding named; filed from the gap inbox.

- **align-context-draw-growth** [design-pending] — **align's cache-read draw has
  roughly doubled across the ten iterations of the current model era** (the
  trend is in `.metric/stage-economics-log.txt`; figures are read from there,
  not restated here), making it the second-largest `cr` draw in a recent
  iteration behind build. The **tier** question this started as is settled and
  is not what remains: align now rides the cheaper tier by the ruling recorded
  in `.claude/commands/lead.md`, taken on its work class — verification against
  an already-authored contract — with the spend only saying the tier was worth
  re-judging. What that ruling does **not** answer is why a stage whose output
  is a plan is reading more context every iteration.
  **The hypothesis:** align is carrying context its work does not need. If so
  the fix is context shaping — what the stage loads at entry, and whether its
  batches reset — and that saving is available **at either tier**, so the
  tier-down banked a fraction of it rather than resolving it.
  **Why `[design-pending]`:** nothing yet distinguishes the two readings, and
  they call for opposite work. Growth could be **load-side** (the stage's
  always-loaded set and skill body have grown, so every align session starts
  heavier) or **work-side** (align legitimately audits more surface as the tree
  grows, and the draw is honest). The first is a context-budget defect worth
  fixing; the second is the cost of a bigger repo and should be left alone.
  Telling them apart needs the entry draw separated from the accumulated draw —
  a measurement the meter does not currently make, since it sums a session
  rather than profiling it.
  **The tier-down makes this harder to see, deliberately noted:** a cheaper
  per-token rate makes a growing draw read flat in dollars, so this trend must
  be judged on the `cr` column and never on `cost`. The ruling in the lead
  binding carries that watch condition; this entry is its backlog half.
  **Cost while deferred:** low and slow-rotting — the draw grows a few million
  cache-read tokens per iteration, which the tier-down now prices at a fifth of
  what it did. The real carry is diagnostic, not monetary: the longer the trend
  runs unexplained, the harder it is to tell a load-side regression from
  ordinary repo growth, because there is no clean earlier baseline to compare
  against once both have moved.
  **The trend is not monotonic — checked 2026-08-07 on the `cr` column, and this
  discriminates between the two readings for the first time.** The draw fell
  sharply in the iteration after its peak, to below the era's median, on an
  iteration whose audited surface was smaller. A load-side regression cannot fall
  that way: the always-loaded set and the skill body do not shrink between
  iterations. So the peak reads as work-side, and "roughly doubled" in the lead
  line above is a trend claim the series no longer supports without qualification.
  Surfaced 2026-08-01 by the `/economics` run at close as the competing
  hypothesis behind an align tier question; the tier half was ruled the same
  session and this entry re-scoped to the half that is still open.

- **template-registry-population-predicate** [design-pending] — a **contingent**
  residual in `check-template-registry-parity`'s population predicate
  (gate-sdk/SPEC-template-registry-parity.md, closed): a `templates/<X>.list`
  enters the parity population iff a sibling `<X>/` directory exists. If a kit
  ever ships a `.list` template of *consumer rule content* whose basename
  happens to match one of its own directories of kit-shipped `.sh` artifacts,
  the predicate admits a private vocabulary into a parity check it can never
  satisfy. No such case exists: two `.list` templates exist tree-wide
  (`kpis.list`, `msg-patterns.list`), and the predicate is sound against both.
  Why this is filed rather than fixed: the direction is a **false red**, which
  announces itself the moment the colliding template lands, and the obvious
  pre-emptive fix — an exception naming the private-vocabulary template — is
  the kit literal the provenance seam refuses, which is precisely why the
  predicate is structural in the first place. The residual is the price of the
  seam-respecting design, not a defect in it.
  The one thing that is not free: the gate is `valve=none`, so a consumer who
  hits the collision has no suppression path short of renaming their own
  template. If this is ever worth work, the work is a valve rather than a
  widened predicate.
  **Cost while deferred:** near-zero and non-rotting — a self-announcing false
  red on a case no tree has produced. Not iceboxed only because a named event
  waits: the next `.list` template landing in any kit is the moment to re-read
  this entry.
  Surfaced 2026-08-01 by `kit-template-registry-completeness` while writing the
  predicate; flagged then as contingent, not a present defect, and re-verified
  against the shipped gate at close.

- **companion-toolkit-profile** [design-pending] [roadmap: next/ecosystem] — the interop rung.
  roadmap-summary: Gate a tree whose specs another toolkit's workflow wrote.
  Govern a tree whose specs an **external spec-authoring toolkit produced** — a
  consumer profile for when the specs Checkwright gates were written by a second
  toolkit's workflow, not by this one's `spec` stage. It cashes the claim below.
  **The design is already decided and is not what this entry holds.** Two
  rulings on record settle it: `prose-profile` rules that a profile ships as an
  adapter delivered as optional consumer config and never as a kit literal, and
  `heterogeneous-agent-delegation` rules that a kit literal naming a vendor
  crosses the provenance seam outright. So the shape is a consumer-side profile
  over a declared artifact layout — the `check-graph` / `graph-vocab` pattern —
  and any per-toolkit specifics stay in consumer config. What is open is the
  *substance* — which lifecycle assumptions break when the amendment set is authored
  elsewhere, and whether a tested two-toolkit consumer is buildable without a kit
  ever naming one.
  **Survey run 2026-08-02 at scope — three corrections, so a spec pass starts here.**
  (1) *Cheaper than filed:* the load-bearing knobs already exist as consumer config
  (`CANON_KIT_SPEC_NAME`, `_AMENDMENT_GLOB`, `_QUEUE_FILE`, `_DOD_MODE`;
  `LIFECYCLE_KIT_AMENDMENT_GLOB`, `_CONTRACT_TOKENS`). The work is *proving them
  sufficient*, not inventing a profile format. (2) *The sharpest break is a silent
  one:* `check-stage-entry` assertion C reads literal `SPEC.md`/`proto/` substrings
  inside amendment bodies as its cross-component signal, so a foreign layout makes it
  **never fire** — the align audit is skipped with no red. An interop consumer is
  not merely unsupported, it is silently under-gated. `check-spec-pointer` breaks the
  same way on non-markdown artifacts. (3) *The deepest coupling is process, not
  config:* `check-spec-derivable-section`/`check-spec-embedded-source` assume the
  canonical-spec-plus-short-lived-amendment model itself, which a toolkit keeping
  many living per-feature specs does not fit at any knob setting.
  **The discharge pattern already exists in-tree.** docs/positioning.md §The tiered
  compatibility claim says "This is tested, not asserted" and cites context-kit's
  `smoke/agents-md.sh`. That is the shape the three claims below owe.
  **Seam re-verified clean:** no tracked file names an external spec toolkit.
  **Intake provenance:** never declined or costed — the opportunities half of the
  same operator-commissioned external review whose *weaknesses* half was filed
  2026-07-23 as six launch-facing rungs, all since landed. That intake's filter was
  the review's "top pre-announcement gaps", so the growth half fell consciously
  outside a stated filter rather than being missed; the gap here is the absent
  intake record, not that session's judgment.
  **Cost while deferred — not zero, and this is the entry's sharpest fact.**
  `README.md`:16-17 and `docs/index.md`:17-18 both already assert, on the first
  screen, "It complements the workflow you already run. Keep your spec process, your
  prompts, your harness." — and docs/orchestration.md:22-23 makes the same move for
  orchestration ("It complements your orchestration setup; it does not replace it"),
  a third site found 2026-08-02 at scope. **No queue or roadmap entry backs any of
  the three with a tested consumer.** Not false — "complements" is far weaker than
  "integrates with X" — but *published and unproven*, in a project whose whole pitch
  is that claims are mechanically proven rather than asserted. The carry is
  reputational and front-door-resident, accruing on every reader rather than with
  time. Surfaced 2026-08-02 at close, intake pass over the review's unfiled half.

- **design-partner-preview** [design-pending] — a narrow external preview before
  any broad announcement: a narrow external preview cohort, its composition ruled
  in the operator's brief, installs observed live rather than by written feedback,
  instrumented for
  time-to-first-green, first useful red, false-positive dispositions, and 7/30-day
  retention per kit. It is the first rung on this queue whose deliverable is
  **evidence from outside this tree** rather than a tree change.
  **The cohort was a named population here until 2026-08-09 and is deliberately no
  longer one.** The composition it stated had since been re-ruled, so the sentence
  contradicted the ruling it was meant to carry; the fix is to name the owner rather
  than to re-state a ruling this file does not hold. The rung's own sequencing is
  unchanged, and it now depends on `prose-profile` shipping.
  **Sequencing is the load-bearing part.** The preview runs *before*
  `benchmark-ab-experiment`, so pilot findings shape that experiment's task
  classes and metrics rather than being retrofitted to them; per-gate
  true/false-positive history and profile retention are preview deliverables,
  not pre-launch builds. Broad announcement waits on the activation path, the
  trust baseline, and two externally observed defect stories. The full launch
  ruling behind this sequencing is operator material and stays in the local-only
  private brief; this entry carries only the queue-visible rung.
  **Intake provenance:** item 4 of the external review's own priority order, and
  like `companion-toolkit-profile` it fell outside the 2026-07-23 intake's
  "top pre-announcement gaps" filter rather than being judged and declined —
  consciously out of scope for that pass, with no record left behind.
  **Cost while deferred:** the highest of this intake, and it compounds. Every
  claim in the tree that would be strongest with external evidence — false
  positives, retention, time-to-first-value — stays supported by internal
  dogfooding alone, and no external install has ever been observed. That is not
  a rot risk that ages passively: each iteration adds governed surface whose
  quality is unattested outside, so the volume of unattested claim grows while
  the evidence stays at zero. Deferring also silently defers
  `benchmark-ab-experiment`, since running it first would fix the wrong metrics.
  Surfaced 2026-08-02 at close, in the same intake pass, as the review's own
  fourth-ranked priority.

- **external-gate-quality-evidence** [design-pending] — durable, published
  evidence of **gate quality as experienced outside this tree**: per-gate
  true/false-positive history, the disposition of each red a non-author hit, and
  whether a red changed behaviour or was worked around. The review's own
  suggested new initiative, and the direct answer to the standing threat that a
  false positive converts the enforcement advantage into bypass and distrust —
  every blocking gate raises the stakes of a wrong red.
  **Why it is not just a report.** The tree already publishes evidence
  projections, so the mechanism exists; what does not exist is a *population* to
  measure. A red in this repo is authored and dispositioned by the same party,
  which cannot distinguish a gate that is right from a gate whose author agrees
  with it. That makes this entry structurally downstream of
  `design-partner-preview` — not blocked by it in the tag sense, since a
  narrower internal cut is conceivable, but the honest version needs external
  reds. The open design question is which of the two it should be, and whether
  the collection surface is a per-gate field the disposition already records or
  a new capture stream.
  **Intake provenance:** the third unfiled item of the external review's growth
  half, outside the 2026-07-23 intake's stated pre-announcement filter rather
  than declined by it.
  **Cost while deferred:** moderate and asymmetric. The gate-quality claim is
  currently supported by fixture pairs and a green battery, which prove a gate
  does what its author specified and say nothing about whether that was the
  right thing to specify — exactly the gap this measures. The carry is that the
  first externally-hit false positive will be argued from anecdote, because no
  baseline exists to argue from. It does not rot, but it cannot be
  retroactively collected either: the history it wants starts accruing only once
  someone decides to record it.
  Surfaced 2026-08-02 at close, in the same intake pass, as the third and last
  of the growth half's unfiled items.

- **lead-line-parser-conformance** [design-pending] — eight independent holders
  re-implement queue-kit's bullet lead-line predicate, coupled only by SPEC
  prose, and no mechanism catches divergence. The census and the residue
  accounting are owned by gate-sdk/SPEC.md §check-gate-exemption-tasks and are
  deliberately not restated here; this entry owns the *mechanism*, which that
  section explicitly leaves to a different unit.
  **The re-implement-and-cite-from-both-ends rule is NOT being re-litigated.**
  queue-kit/SPEC.md §The queue format and the gate-sdk section above both state
  it, for two different reasons (a cross-kit cycle for drift-kit, a layering
  inversion for gate-sdk), and that ruling stands. What is missing is any oracle
  over agreement.
  **The hard constraint on any candidate, and the reason a naive one reds on
  wanted behavior:** the holders diverge on the section **span** deliberately —
  drift-kit's `kpi-deferred-age` resets on an unknown heading and so excludes the
  icebox tier, `check-gate-exemption-tasks` does not reset and so includes it,
  and **both behaviors are wanted**. So a conformance mechanism must scope to the
  **line predicate alone**.
  **Second divergence axis, verified at this close and not in the original
  filing:** `bin/queue-index.sh`'s default index-mode walk matches lead lines with
  a column-0 `/^-[[:space:]]/` anchor rather than the tolerant
  `^[[:space:]]*-[[:space:]]+` one, so indented sub-task bullets are invisible to
  it. That is *sanctioned* — the gate-sdk section rules the indent level the
  reader's choice and the predicate not — but it means a conformance mechanism
  must also decide which level it holds, or it reds on a holder exercising a
  sanctioned narrowing. A whole-holder text-identity assertion is therefore
  already known to be wrong.
  **Candidate shapes, none ruled:** a conformance test asserting every holder
  returns the same slug set for one fixture queue; a gate asserting the line
  predicate's text is identical across declared holders; or accept the residue and
  merely roster the holders so the next format change is findable. The third is
  also the form that would retire the hand-maintained census in the gate-sdk
  section, which is the Derivation-first half of the prize.
  **Why `[design-pending]`:** the holders span four kits with a layering ban
  between two of them, so where the shared fixture or the declared-holder roster
  *lives* is a cross-kit ruling before it is a script.
  **Cost while deferred:** a format change costs all eight edits and endangers
  the few that build sets — a set builder with a wrong predicate fails silently,
  in wrong membership, which is `check-gate-exemption-tasks`' own shipped defect
  class. Two of the holders are inline scans no function-name grep finds, which
  is how the census was twice under-stated before a whole-tree survey ran.
  Non-rotting while the format holds still; the exposure is entirely at the next
  format change.
  Filed 2026-08-02 at spec by the causal-completeness reader survey for
  `gate-exemption-live-slug-derivation`, explicitly scoped out of that amendment;
  promoted at close from the gap inbox, its census re-verified against the tree
  rather than re-derived from the filing.

- **unqualified-section-citation-liveness** [design-pending] — a bare `(§Heading)`
  citation in governed prose — the same-file form, with no `<path>.md` prefix —
  resolves to nothing and no gate reds. The two citation gates each miss it from
  a different side: `check-md-refs` resolves markdown *links* only, and
  `check-spec-pointer`'s prose extractor requires a `<path>.md §` prefix before it
  will look (the `match(s, /…\.md[[:space:]]*§/)` guard), so the qualified form is
  held and the unqualified one is not.
  **Instance, fixed at this close:** gate-sdk/SPEC.md cited `§check-fixture-pair`
  as the authority for "a registered gate still owes its `good/`+`bad/` pair"; no
  such section exists — the gate is `check-gate-fixture-coverage`, named correctly
  seven lines later in the same paragraph. A reader chasing the load-bearing half
  of that sentence landed nowhere.
  **The class is large and currently unverified:** a mechanical sweep of the kit
  SPECs plus `CLAUDE.md` and `DOCTRINE.md` counted 171 parenthesized unqualified
  citations, against which a crude same-file resolver flagged 11 — one genuine
  (the instance above) and the rest artifacts of the crude predicate. Those
  artifacts are the design: they are the false-positive classes a real gate must
  answer for.
  **Why `[design-pending]` — three named false-positive classes, all observed in
  that sweep.** (1) **Cross-file antecedent:** `site-kit/SPEC.md` cites
  `(§run-gates)`, a gate-sdk section, and gate-sdk/SPEC.md cites
  `(§check-spec-pointer)`, a canon-kit one — bare, with the owning path named
  earlier in the paragraph. `check-spec-pointer` already implements a blank-line
  paragraph join for the qualified form, so the antecedent rule has an
  implementation to extend rather than invent, but whether a bare citation should
  bind to the nearest prior path or be required to qualify itself is the ruling
  this entry owes. (2) **Prose tail:** `(§Bundled KPIs / §Layout and
  configuration)`, `(§run-gates owns the contract)` — where the heading name ends
  and prose resumes is not syntactic. (3) **Possessive and inflected forms:**
  `(§check-stage-entry's trigger-gated-stage calibration)`.
  **Shape:** an assertion inside `check-spec-pointer` rather than a new gate — it
  already owns heading resolution in both exact and prefix modes, already walks
  the manifest set, and already carries the paragraph join. Sibling to
  `prose-filename-citation-liveness`, which is the same family (a citation form
  falling between the two gates) from the other end: that one is a path with no
  heading, this one a heading with no path. A promoting scope should cost them
  together and may find one predicate covers both.
  **Cost while deferred:** 171 live unqualified citations across the governed doc
  set, held by nothing. Each merged amendment and each renamed section can strand
  one, and the only detector is a reader who follows the pointer — which is how
  the instance above was found, one iteration after the paragraph was rewritten.
  Debt: an assertion added to an existing gate and its fixture arm; adds no
  governed name — the new-names litmus does not fire, which is what separates it
  from `prose-filename-citation-liveness`' shipped-path variant.
  Filed 2026-08-02 at spec to the gap inbox as the dangling pointer; promoted at
  close as the gap-generalization that inline fix owed, with the coverage gap
  verified against both gates' source and the class sized by sweep.

- **scratch-run-steer-rule** [design-pending] — the sanctioned form for executing
  a scratch script exists, is allowlisted, and nothing steers anyone to it, so
  sessions reach for the direct path and pay a permission prompt every time.
  `guard-kit/bin/scratch-run.sh` is granted by the **committed** allowlist
  (`Bash(bash guard-kit/bin/scratch-run.sh *)`), while a direct `bash .tmp/x.sh`
  is granted by nothing and prompts on every distinct script name — by design,
  since a path under the gitignored scratch dir is rewritable by any session, and
  scratch-run's echo-at-execution is the compensating control that buys the grant
  (guard-kit/SPEC.md §scratch-run).
  **Measured, not asserted:** this iteration's friction log ranks direct
  `bash .tmp/*.sh` calls as roughly twenty prompts across seventeen distinct
  one-off script names — the single largest prompting class once the guard's own
  deliberate steers (`cat`, `grep`, `find`) are set aside, and every one of them
  avoidable by one word of command.
  **Shape:** a `scripts/bash-guard.sh` steer arm on a direct scratch-dir
  execution, pointing at the runner — the same shape the guard already uses to
  steer `cat` to Read, an absolute repo path to its relative form, and the
  harness scratchpad to `.tmp/`. The bar itself is **not** being lowered: the
  prompt on the direct form is the control, and the steer routes to the form that
  already paid for its grant rather than granting the direct one.
  **Why `[design-pending]`:** it needs the guard-kit decision-table arm and its
  fixture, and one real ruling — whether the steer fires on the scratch dir alone
  or on any `bash <path>.sh` that no allowlist entry covers, which is a much wider
  net and would collide with legitimate one-off tool invocations. Interacts with
  the iceboxed `scratch-execution-allowlist-bar`, which records that each close
  re-derives the standing bar; a steer that names the bar in its message would
  retire that re-derivation as a side effect.
  **Cost while deferred:** about twenty interruptions per iteration, paid by
  whichever session is doing measurement work, and it falls hardest on exactly
  the sessions that probe the tree most. Non-rotting, bounded, and invisible to
  every gate — the friction log is the only detector, and it is advisory.
  Debt: one guard arm plus its decision-table fixture; adds no governed name.
  Filed 2026-08-02 by close's tooling-friction triage, the ranked log read against
  the committed allowlist rather than the local overlay.

- **customer-facing-iteration-cadence** [design-pending] — operator-directed: formalize
  a cadence so internal iterations cannot starve customer-facing ones. The ask was "X
  internal iterations per one roadmap/customer-facing one, with high-priority
  exceptions"; the survey below re-shapes it twice before any design starts.
  **A rule of this job already exists and has no oracle — that is the real gap.** The
  operator's local brief carries a stricter standing bound (finish only work that cuts
  time-to-first-value, closes a trust/supply-chain gap, or produces external proof;
  defer the rest until five external installs exist), with an exception clause already
  requiring the public queue entry to carry the exception and its reason. It is
  private, prose-only, and unenforced. So the subject is *giving the standing rule an
  oracle*, not adding a second, weaker ratio beside it.
  **Starvation bound, not a fixed ratio.** A ratio misfires in both directions on this
  tree: it forces a customer-facing iteration when none is ready — two of the three
  `next` roadmap items are demand-gated on adopters who do not exist — and it makes
  "internal" a quota to pad. A bound (no more than N consecutive iterations without a
  customer-facing one; the N+1st is either customer-facing or files a costed, named
  exception) never forces unready work, and turns the exception from a loophole into a
  filed artifact. That is the anti-vacuity property, and the failure mode to design
  against is the one `vacuous-green-elimination` just named: an exception clause broad
  enough to swallow its own rule.
  **Why `[design-pending]` — the classification data exists on no tracked surface.**
  `WORKFLOW-STATE.txt` is truncated at every scope boundary; roadmap tags ride
  entries that pass through Done, which close clears; `.metric/` is gitignored and
  account-bearing, so no gate may read it. A cadence rule over today's tree is
  therefore prose-only — the weak deliverable enforcement-first refuses. It becomes
  gateable with one small addition, and the pattern is already shipped:
  `.workflow/audit-roster.txt` is a tracked `due: <event> — last: <iteration>` roster
  that close reads and stamps. A `last:` stamp plus a boundary-incremented counter
  makes the gate a comparison rather than a history reconstruction.
  **Seam:** bound, stamp and counter are generic mechanism; the "customer-facing"
  predicate is consumer config, the `graph-vocab.sh` pattern — a kit literal deciding
  what counts as customer-facing would publish a product judgment.
  **Cost while deferred:** low and non-rotting while an operator directs composition
  by hand, which is exactly what the directive that surfaced this did. Measured at
  filing: the last two closes (`shipped-roster-parity`, `vacuous-green-elimination`)
  were internal and the Deferred pool stands at 71 — a two-iteration run, not a
  chronic one, so the bound would be preventive rather than remedial.
  Filed 2026-08-02 by scope from supplemental operator intake during the unit-set
  survey; scope-gated intake, so filed costed rather than started.

- **gate-authoring-sdk-surface** [design-pending] [roadmap: next/ecosystem] — a gate-authoring SDK.
  `.gate` as the substrate-neutral surface. **Operator-surfaced during
  `native-gate-dispatch-seam` build; filed so the framing outlives the session that
  saw it.** Horizon set 2026-08-02 on the operator's steer: this is ecosystem work
  on `companion-toolkit-profile`'s rung, not "make our own gates fast and opaque".
  roadmap-summary: Author a gate in any language behind one substrate-neutral descriptor.
  **The observation:** because the manifest lives outside the implementation, the
  graph, hook, and meta-gate layers never learn what implements a gate. That is not
  a Rust seam that happens to work — it is a **language-agnostic** one. A gate could
  be written in any language behind a descriptor, and slice 1 already avoided baking
  a language into the descriptor format, the resolution path, and
  `check-gate-substrate-parity` assertion D (whose comment-leader match is `#`, `//`
  and `/*` deliberately). `GATE_SDK_NATIVE_SRC` is a path knob, not a language knob,
  for the same reason.
  **Why it is an SDK question and not a port question:** the port asks "how does
  *this* gate move"; this asks what a **third party** needs to author a gate on any
  substrate — the descriptor contract, the subcommand calling convention, the output
  contract, and the fixture-pair obligation, which are already the four things
  gate-sdk holds. The kit is most of an SDK already; what is missing is the statement
  that the substrate is a parameter.
  **Boundary against the two questions already settled, so this one does not sprawl:**
  how a compiled gate *arrives* and what it *discloses* are both ruled and recorded
  (TRAJECTORY.md §The closed rulings; gate-sdk/SPEC.md §Consumer payload). What stays
  open is what a gate *is* independent of substrate, and that is this entry alone. The
  distinction it supplies outlived those rulings and is why it was worth keeping — a
  descriptor discloses a gate's **shape** without its **predicate**, which is the line
  the disclosure ruling drew.
  **Not started, and deliberately not widened into slice 1**: building it would have
  meant generalizing a seam with exactly one instance, which is the shape of a design
  that fits nothing later.
  **Cost while deferred:** each further port hardens substrate-specific assumptions
  by habit rather than by ruling, and the cheapest moment to keep the seam neutral is
  before the second language exists — not after.
  Filed 2026-08-02 by build, on an operator ruling, during `native-gate-dispatch-seam`.

- **queue-entry-evidence-tier** [design-pending] — **not "how is cut detail recovered"
  — that is solved. The gap is that a reader cannot tell detail was ever cut.**
  `check-queue-entry-budget`'s own help names the recovery path
  (`git log -p -S'<slug>'`), which is tracked, public-safe and needs no session
  transcript. What no surface carries is the *signal*: a compressed entry looks
  identical to an entry that was always short, so a session with a question it cannot
  answer has no way to know an evicting commit holds the answer, nor which one.
  Reproduced 2026-08-02: one scope session pushed `native-gate-binary-port` over the
  cap three times and compressed it back, and nothing in the entry now records that
  the worked arguments exist upstream.
  **The budget is gated and the gate works — this is not ungated drift.** The cap is
  50 and it is measured on the entry's **raw extent**, blanks included (assertion A,
  `n = bound - o_start[i]`), not on content lines; content lines are counted only for
  the icebox shape rule. It red thrice on that entry this session and forced each
  compression, so the discipline held. What it cannot do is say where the cut went.
  **A design constraint that is not obvious from the queue's prose, verified in the
  gate's awk:** a nested sub-task is counted into every open parent
  (`for (i = 1; i <= nopen; i++) o_nb[i]++`), so **sub-tasks do not relieve a parent's
  budget — only a companion top-level entry does.** A mechanism that routed overflow
  into sub-tasks would be defeated by the gate it has to live under.
  **One more constraint:** the repo is public and CLAUDE.md forbids internal session
  references in tracked files, so no entry may point at a transcript. And under
  spec-over-precedent an evicting commit evidences *that* a thing was costed, never
  *that the costing is right* — so the signal must lead to a ruling surface, not stop
  at history.
  **Empirical support, measured 2026-08-02 by spec.** Recovery via `git log -p -S`
  **worked**: every ground dropped from this entry's own reproduction case came back
  intact from the filing diff. What failed was *signal* — nobody knew to look, and the
  arguments were supplied from memory twice the same day instead. That is evidence for
  the narrowing already applied here, not grounds to re-open the shape.
  **One candidate mechanism, evaluated at close 2026-08-02 and recorded unproven.**
  The gate cannot judge loss, but it may be able to detect the *shape of the
  sanctioned remedy*: a commit that shrinks a deferred entry past some delta while
  adding no linked entry in the same commit. Two known costs, both real: it needs
  history access no battery gate has today, and it false-positives exactly where a
  genuinely answered ground is deleted — which is the case the authorization ruling
  in queue-kit/SPEC.md §check-queue-entry-budget already governs. Recorded so a
  later session weighs it rather than re-invents it; it is not a design decision.
  **Cost while deferred:** compression stays silently lossy in practice — the bytes
  survive in git, and the knowledge that they are worth fetching does not.
  Filed 2026-08-02 by scope on operator intake. The operator previously proposed an
  evidence-submission mechanism and it was declined in favour of boxed entries; this
  narrower shape is the ground for revisiting that ruling.

- **gate-tamper-roster-native-reach** [design-pending] — `check-gate-tamper` does not
  reach a ported gate's implementation. Split 2026-08-02 at scope from
  `native-gate-meta-layer-reach` by operator ruling, when that entry narrowed to its
  `check-reads-couples` half; this is the tamper half, unchanged in substance.
  **The roster excludes `native/`,** so a commit editing a ported gate's
  implementation alongside any gate file is **refused**. Discovered at commit time
  during `native-gate-dispatch-seam` build: the crate and a gate widening could not
  land together. Slice 1 sequenced around it — implementation in one commit,
  descriptor in another — and the conservation table records it, but the constraint
  gets *worse per port*, because the natural unit "edit the gate's rule and its
  declaration together" is precisely what the roster forbids. Verified 2026-08-02 at
  scope: the roster is `scripts/delegation-config.sh`'s `DELEGATION_KIT_META_PATHS`
  unioned with the kit roots, and `native/` is in neither. Same gate, second hole:
  `extract_exemptions()` parses a shell `# exception-list:` array literal and has no
  implementation-side equivalent.
  **Why `[design-pending]`:** it wants the ruling `gate-authoring-sdk-surface` holds
  — whether a meta-gate reads a substrate-neutral descriptor or learns each
  substrate — and that entry is horizon-set to ecosystem work, so this one waits.
  **Why it did not ride the narrowing:** it is commit ergonomics, not a correctness
  block. gate-sdk/SPEC.md §Porting a gate to the binary substrate names only the
  `check-reads-couples` half as a second-port prerequisite; this half was sequenced
  around once already and can be again.
  **Cost while deferred:** zero today (no live `.gate` dispatch); from the second
  port on, the natural commit unit is forbidden and every port pays the same
  two-commit sequencing tax.
  Filed 2026-08-02 at close from the gap inbox; found by build. Split out 2026-08-02
  at scope.

- **gate-timing-baseline-comparability** [design-pending] — a baseline with no
  comparer decays into a number nobody can use. `battery-baseline-capture` shipped
  `.workflow/gate-timing-baseline.txt` this iteration — 94 per-gate rows × 3 runs,
  captured deliberately *before* any gate ports off bash. Its extension and tier
  are correct (checked projection, `# contract:` pointer header, a stated
  field-wise line grammar — audited at close under the roster's
  `workflow-surface-extension` class). What it has is **no reader**: nothing in the
  tree parses it, and `kpi-gate-runtime` reads a single live timings file rather
  than a baseline-versus-current pair, though `DRIFT_KIT_TIMINGS_FILE` shows the
  seam where a comparer would attach.
  **Why that is not merely "not yet" — and the premise has since expired, which
  raises it rather than settling it.** The comparison moment is the second port. It
  was argued here as far off, on the ground that it waited on the operator-gated
  first tag publishing binaries. **Both halves of that ground are discharged:** the
  tag was cut at `v0.22.0` (2026-08-07) and the second port is built and proved
  (gate-sdk/SPEC.md §What is retained, and where the second port stands — every
  member of the first cohort ships its rule as a compiled subcommand). So the moment this entry
  says the baseline "must be ruled before" has **arrived**, and the entry is now
  overdue rather than parked. Corrected in place at the 2026-08-09 close's
  premise-rot review; the re-disposition is scope's to make, not close's.
  Meanwhile the baseline's own validity conditions decay. Two already have,
  measured at close the same iteration the file landed: the header pins a
  `gates.list` sha256 that no longer matches (the 95th gate landed after capture),
  and the header's environment line pins a kernel, bash build and CPU count that a
  later comparison will almost certainly not reproduce. The pins are doing their
  job — they make the drift *visible* — but nothing reads them, so nothing will
  refuse an apples-to-oranges comparison when it is finally made.
  **Why `[design-pending]`:** the choice is between a comparer that normalizes
  (re-measuring the shared gate set on the current box, which makes the stored
  numbers advisory) and a freshness gate that invalidates the baseline on roster
  or environment change (cheap, but it deletes the only pre-port measurement the
  repo will ever be able to take). Those are opposite answers, not a size question.
  **Held out of the `native-port-unblocking` iteration 2026-08-02 by operator
  ruling — a trigger, not a dismissal.** That iteration unblocks a second port
  without landing one, and this entry's surfaces (`.workflow/`, drift-kit) share
  nothing with the gate-sdk SPEC surface the iteration amends, so it buys no
  amortization. It becomes urgent the moment a second port is *scheduled*: that is
  the comparison moment, and the baseline must be ruled before it arrives.
  **Staleness measured rather than asserted, recomputed 2026-08-02 at scope so the
  next reader need not:** the header pins gates.list sha256
  `bd6c1cc8f8b89154d03fd80d4609d2b29efe82b5971dded6017564366923d5c9`; the file today
  hashes `19074f2be101376afb8b490a7ad08cf3cef303b4e97e40bec6f3619247a7800e`. The
  registry stands at 95 entries against the baseline's 94 rows.
  **Cost while deferred:** the port's headline justification stays unmeasurable in
  the one tree that could measure it — the before-numbers exist and quietly stop
  being comparable to any after-numbers, with no surface saying so.
  Filed 2026-08-02 at close by the `workflow-surface-extension` roster audit.

- **meta-gate-conservation-record-reach** [design-pending] — the conservation record
  under-describes what `check-gate-tamper` refuses, on both halves.
  `gate-sdk/SPEC.md` §Meta-gate conservation records the `native/`-excluded roster as
  refusing "a commit editing a gate's Rust implementation alongside its descriptor",
  mitigated by "neither binds slice 1". Verified 2026-08-03 by the
  `native-gate-meta-layer-reach` build, which the gate blocked.
  **Both halves under-describe it.** The refused co-staging was the crate alongside a gate
  *script*, not a descriptor — the rule is every gate file, and the recorded wording names
  only the descriptor case a reader is least likely to hit. And it now binds *routinely*
  rather than not at all: every crate-plus-gate change of the shape this iteration opened
  must land as two commits.
  **Deliverable:** the wording correction, plus a decision on whether the two-commit split
  is the intended contract or a roster gap.
  **Why `[design-pending]`:** the decision is the entry. The split is arguably the right
  outcome — a meta-gate that let an implementation and its own guard land together would
  be conserving nothing — but that is not what the record predicts, so correcting the
  prose without ruling the contract only relocates the surprise.
  **Cost while deferred:** a reader planning a port budgets against a record that
  understates both the rule's reach and its frequency, and meets the two-commit tax at the
  pre-commit hook instead.
  Filed 2026-08-03 at close from the gap inbox; found by build batch 1.

- **couples-dynamic-root-resolution** [design-pending] — the skipped-and-counted bucket is
  where trigger-drift hides, and its dominant subset is decidable.
  Verified 2026-08-03: `check-reads-couples`' root resolver handles a bare quoted literal,
  a KIT-prefixed token and a REPO_ROOT-prefixed token, and nothing else — so a walk root
  held in a *variable* falls to skipped and is never checked.
  **The live instance is this repo's own gate.** `check-gate-substrate-parity` assertion D
  walks the crate source recursively, but the gate's `couples=` never named it, so an
  implementation edit that plants a manifest-class annotation does not re-fire the gate at
  pre-commit. That hole stood for an unknown period with no couples token to re-fire it —
  the skipped bucket doing nothing. Assertion E's `couples=` widening fixed the same defect
  class for the crate root; assertion E's new kit-root walk lands in the same bucket. The
  instance fix is one manifest token pair mirroring what `check-reads-couples` already
  couples, and it is deliberately **not** landed bare: enforcement-first binds the instance
  fix to the check that catches the class, so both land in this unit.
  **The decidable subset, with no false-positive surface:** an assignment whose value is a
  parameter expansion with a literal default — the universal kit idiom CLAUDE.md
  §Conventions declares. Following one such assignment in the same file would have caught
  assertion D.
  **Deliverable:** that resolver extension, the assertion-D couples fix as its pinning
  instance, and a named cadence for the residue.
  **Why `[design-pending]`:** the residue is the open design. A truly dynamic root stays
  undecidable, so per the enforcement-first false-positive carve-out it needs a named
  cadence rather than a bare counter — and today the count is printed and nothing reviews
  it. Whether that cadence is a roster class, a close step or a threshold is unruled.
  **Cost while deferred:** every gate whose walk root is a variable is uncoupled and
  silently under-triggered, and the repo's own substrate-parity gate is one of them — a
  green battery that never re-ran the assertion.
  Filed 2026-08-03 at close from the gap inbox, merging the assertion-D instance into its
  class; both found by build batch 2.

- **amendment-landing-citation-assertions** [design-pending] — nothing validates an
  amendment's landing citations, and align fixed five instances bare.
  Align corrected five amendment-citation defects with no mechanism named and none landed
  — the enforcement-first stop signal (doctrine-kit/DOCTRINE.md: a green instance fix is
  the stop signal to ask what check should have caught it). `check-amendment-queue` is the
  only registered amendment gate and asserts the bidirectional queue-amendment rule and
  spec-readiness only.
  **The class splits by decidability, and two thirds of it is gateable at zero FP.**
  *(1)* Every delta the amendment marks as landing carries an "Existing sections updated"
  entry — decidable from the amendment file alone, and it catches four of the five
  (vendoring deltas 3 and 9, meta-layer deltas 1 and 2). *(2)* Every citation's named file
  exists and contains the named heading — the same shape as the repo's existing `spec:`
  pointer resolution. *(3)* **Not gateable, high FP:** whether a delta prescribes an edit
  its target actually needs. Vendoring delta 11 prescribed a cargo-bullet treatment for
  `installer/README.md` §Requirements, which carries no such bullet at all. Per the
  false-positive carve-out this becomes a stated manual duty whose cadence event is the
  align stage — which already performs this tree-verification by hand.
  Also in the class: `gate-payload-disclosure-ruling`'s crate-path knob was credited to
  `native-gate-vendoring-model`, which declares no such knob.
  **Deliverable:** assertions (1) and (2) inside `check-amendment-queue` — extending beats
  adding, and its `couples=` already reach `TASK-QUEUE.md` and the amendment glob — plus
  (3) written into the align stage template as a named duty.
  **Why `[design-pending]`:** (3)'s placement is the open call. A duty stated in a stage
  template is only as good as the stage that reads it, and this iteration's own evidence
  (`enforcement-first-load-trigger`) is that a stage template is not a reliable carrier.
  **Cost while deferred:** an amendment can claim a delta landed, name a file that does not
  carry it, and merge green — after which the claim is deleted with the amendment and
  nothing records that it was false.
  **The deferred cost was paid 2026-08-04, exactly as written.** A build batch found a
  false citation in an amendment that align's audit had already passed as
  zero-divergence — assertion (2)'s case, and the second stage to miss it by hand.
  That align passes this class by eye is now measured, not assumed, which retires the
  argument that the manual duty under (3) can also carry (2).
  Filed 2026-08-03 at close from the gap inbox; found by align.

- **amendment-owner-position-citation** [design-pending] — prose may cite a merged
  amendment as *settled history*, never as a live *owner*; today it does both.
  recurrence: amendment-owner-position-citation 2026-08-06
  Amendments are deleted on merge (canon-kit/SPEC.md §Merging an amendment), so a citation
  naming one in owner position dangles the moment it merges — by construction, not by
  oversight. This is the inverse direction of the class
  `amendment-landing-citation-assertions` covers (amendment-to-spec landing citations) and
  of `amendment-deletion-content-completeness` (content that fails to land): here the
  canonical side is the one that goes stale.
  **Re-verified 2026-08-03 at close against the tree, and half the filing was already
  false.** The `gate-sdk/SPEC.md` instances the base bullet named are **gone** — build
  batch 5 repointed them to `TRAJECTORY.md` when the ruling record landed, and the slug now
  has zero occurrences in that file. The live residue is entirely inside `TASK-QUEUE.md`
  deferred bodies, and the sweep found **five**, not the two validate filed: three more
  naming `gate-payload-disclosure-ruling` and one naming `native-gate-meta-layer-reach`,
  all phrased in present tense against slugs that are now bare `## Done` lines.
  `check-spec-pointer` catches none — it matches path-form only, and all five are slug-form.
  **Recurred 2026-08-06 on a second surface that re-verification did not cover, falsifying its
  "entirely inside `TASK-QUEUE.md`" scoping.** Sibling amendments *inside one iteration* cite
  each other by path, so merge order decides which side dangles: `SPEC-verify-verb.md` cited
  `evidence-kit/SPEC-liveness-lock.md` and `lifecycle-kit/SPEC-dispatch-signal.md`, both
  merged away by earlier batches, leaving two dead pointers inside a governing input the last
  batch had to repoint by hand before it could reason from it. Nothing fired on either. This
  surface is path-form, so unlike the five slug-form instances it is in `check-spec-pointer`'s
  reach — and its cost lands on the session reading the amendment as governing input, not on a
  reader of the queue.
  **The design question is exactly where those five differ from two that are fine.** The
  tag algebra's unresolved-token rule *explicitly* sanctions naming landed work: "entries
  legitimately name a closed defect class, a shipped contract, a settled ruling, and that
  citation is valuable prose no gate may punish." Two live citations do it correctly — they
  say a merged slug *holds* a decision, past tense, settled. The five defective ones say a
  merged slug *rules* something, present tense, as though the reader could go read it. Both
  forms sit in the tree today, which is the calibration set.
  **Deliverable:** repoint the five to `TRAJECTORY.md`, then extend `check-amendment-queue`
  (its `couples=` already reach `TASK-QUEUE.md` and the amendment glob) with an
  owner-position assertion. Structural removal first, gate second.
  **Why `[design-pending]`:** owner position needs a decidable spelling that separates those
  two forms, and tense is not mechanically decidable. The tractable proxy is the
  `§`-heading form and the amendment path form; whether that catches enough of the
  present-tense class to be worth its false-negative surface is the open call, and getting
  it wrong permissively punishes the prose the tag algebra protects.
  **Cost while deferred:** five false statements stand in the queue, each pointing a future
  scope at a content-free Done line, and the next merged amendment adds more for free.
  Filed 2026-08-03 at close from the gap inbox, merging the validate-filed instances into
  the base class and re-verifying both against the tree; found by build batch 1 and
  validate, corrected and widened at close.

- **amendment-done-move-assertions** [design-pending] — a task can be marked done with its
  amendment left undeleted on disk, and the battery stays green.
  The amendment-merge Done-move contract is stated twice and gated nowhere, and the hole is
  exactly the one `check-amendment-queue`'s bidirectional rule exists to close.
  canon-kit/SPEC.md §Merging an amendment requires the completed entry to move to `## Done`
  *dropping* its spec-pointer tag; queue-kit/SPEC.md §The queue format makes a Done entry a
  bare slug line. Verified 2026-08-03 against the tree: nothing checks either.
  **Why it falls through.** `check-amendment-queue`'s awk classifies only the feature,
  active and deferred sections, so `## Done` falls to the unclassified bucket — while its
  every-amendment-has-an-entry assertion collects spec-pointer refs with a whole-file grep,
  so a tag left on a Done entry still *satisfies* that half. queue-kit's
  `check-queue-sections`, `check-tag-lead-line`, `check-task-conservation` and
  `check-queue-hygiene` all assert other things. Net effect: the silent-amendment-survival
  failure the bidirectional rule was built against.
  **Defect class:** a state-machine transition whose completion contract is prose-only on
  the terminal side.
  **Deliverable, gateable at zero FP inside an existing gate:** a done-section arm in
  `check-amendment-queue` — a configured done-section name beside the active-sections knob,
  then (i) no spec-pointer tag on a done entry and (ii) done entries are bare slug lines. The
  `couples=` already reach `TASK-QUEUE.md` and the amendment glob, so no new gate and no new
  trigger set; extending beats adding.
  **Why `[design-pending]`:** the done-section name is a second consumer knob for a section
  canon-kit does not otherwise know about, and whether canon-kit should learn queue-kit's
  section vocabulary at all — rather than queue-kit asserting the Done shape and canon-kit
  asserting only the tag — is the layering call.
  **Cost while deferred:** zero while every merge is hand-checked; the failure mode is
  silent, so its first instance is also its discovery.
  Filed 2026-08-03 at close from the gap inbox; found by build batch 3 while establishing
  which dispositions of a partially-landed amendment the oracles actually permit.

- **partitive-exemption-line-scope** [design-pending] — `check-manifest-count` is stricter
  than its own spec, and the gap is one line break wide.
  Verified 2026-08-03 by build batch 4: prose reading "appeared in 57 of" / "the 96 checks
  counted that day" reds as a restated collection total purely because the line broke
  between the partitive marker and the cardinal — the same sentence reflowed onto one line
  passes.
  **Root cause, decidable and narrow.** The partitive exemption in `canon-kit/lib/spec.sh`
  tests a *same-line* prefix regex against the text preceding the cardinal, so a partitive
  marker on the previous line is invisible to it — while canon-kit/SPEC.md
  §check-manifest-count states the exemption as a property of the *sentence*, a partitive
  marker on either side of the match, with no line-scoped qualifier. The gate being
  stricter than its own spec is the defect, not the reverse.
  **The cost is paid quietly and then paid wrongly.** It is invisible until it fires, and
  the remedy the help line offers is the exemption tag — which would bless a non-violation
  permanently and corrupt the exemption's meaning for every later reader. A low-FP-contract
  violation that converts into a corrupted exemption set.
  **Deliverable, and the machinery already exists:** `check-manifest-count.sh` already
  carries a wrapped-paragraph hook beside the line hook, so the paragraph-joined text is
  available at the match site. Test the partitive prefix against the joined paragraph
  rather than the raw line, or carry a one-line lookback for the prefix window. The fix
  belongs in `canon-kit/lib/spec.sh` where both gates read it — `check-prose-enum` shares
  the same adapter and inherits the same defect — not in either gate. Plus a `good/`
  fixture case pinning a wrapped partitive, the regression the current pair does not carry.
  **Why `[design-pending]`:** paragraph-joined and one-line-lookback are not the same
  contract. Joining widens the exemption to any partitive anywhere in the paragraph, a real
  false-negative surface; the lookback is narrower and arbitrary. The spec says sentence,
  and neither implements a sentence.
  **Cost while deferred:** every author whose partitive happens to wrap pays a red gate and
  is offered an exemption tag as the remedy, so each occurrence risks permanently corrupting
  the exemption set rather than merely costing a reflow.
  Filed 2026-08-03 at close from the gap inbox; found by build batch 4.

- **root-doc-roster-registration-parity** [design-pending] — a new tracked root doc must
  join three rosters and only one of them is enforced.
  Verified 2026-08-03 while landing `TRAJECTORY.md`: `scripts/root-allowlist.list`
  (`check-root-tiering` reds on any tracked root entry not covered),
  `scripts/core-files.list` (`check-core-files` only reds listed-but-missing, never
  missing-but-should-be-listed), and the manifest-files array in `scripts/canon-config.sh`
  (nothing reds on a non-member). The allowlist has exactly one reader,
  `check-root-tiering`, and no gate couples allowlist membership to either of the other two.
  **Net effect:** a root doc that joins only the allowlist is battery-green while being
  unpinned against silent deletion *and* outside the doc-gate corpus — so its links,
  commands, spec pointers, manifest counts and temporal narration are all ungoverned. The
  failure is invisible and grows with every reader the doc acquires.
  **Defect class:** a governed surface whose registration is split across N rosters with
  enforcement on only one.
  **A fourth axis, added at close from the same landing.** A root doc also owes an inbound
  orientation link from a landing page, and nothing enforces that either: `TRAJECTORY.md`
  was reachable only from a SPEC deep-link and the always-loaded housekeeping roster until
  close added the README pointer by hand. Same shape, same invisibility, and it is the
  axis a *reader* actually pays for.
  **Deliverable, an extension rather than a new gate:** `check-root-tiering` already reads
  the allowlist and its subject already *is* the root set, so add an assertion that every
  allowlisted tracked root markdown file is also a `core-files.list` line, a manifest
  member, and the target of at least one link from a declared landing-page set. Its
  `couples=` must widen to `scripts/core-files.list` and `scripts/canon-config.sh`, or the
  assertion will not re-fire when either roster changes.
  **Why `[design-pending]`:** it is not zero-FP without a stated exemption set, checked
  against today's tree rather than assumed. `TASK-QUEUE.md` is core-files-pinned but
  deliberately outside the manifest set (queue-kit owns its corpus), and the `SPEC-*.md`
  amendments are deliberately in neither (transient tier). So the deliverable is the
  assertion *plus* a two-member exemption declaration, and where that declaration lives —
  gate literal, consumer config, or a per-file tag — is the open call and touches the
  provenance seam.
  **Cost while deferred:** every root doc added from here forward is one forgotten roster
  line away from being ungoverned, silently, with the battery green.
  Filed 2026-08-03 at close from the gap inbox; found by build batch 5.

- **enforcement-first-load-trigger** [design-pending] — enforcement-first has no load
  trigger anywhere in the lifecycle, which is why findings this iteration landed bare and
  were triaged retroactively.
  Verified 2026-08-03: zero occurrences of the rule's name anywhere in
  `lifecycle-kit/templates/` — no stage skill, no lead template, carries it.
  **It exists in exactly two tiers and neither fires at the moment of discovery.** The
  always-loaded CLAUDE.md line is a *summary* — the fix and the gate land in one unit —
  that omits every part of the procedure which changes behaviour: the green-instance-fix
  stop signal, the structural-first ladder, the false-positive carve-out, the named-cadence
  requirement. The full procedure in `doctrine-kit/DOCTRINE.md` is load-triggered with
  nothing in the lifecycle loading it. Load-trigger residency says a rule is resident only
  when no stage, skill or tool loads it; this one is neither resident in useful form nor
  triggered, so it fires only when a session already remembers it.
  **Evidence it is placement and not context pressure:** the align stage — a fresh session
  with a narrow brief and its stage skill loaded — fixed five instances bare, identically
  to a saturated routing-tier lead.
  **Two mechanisms, ranked.** *(1) Structural and cheapest* — put the triage step in the
  stage ritual where findings are discovered, so it costs a paragraph rather than a
  dispatch. *(2) Mechanical and self-applying* — `lifecycle-kit/bin/file-gap.sh` takes free
  text, so a bullet saying only that something is broken passes; requiring the three fields
  the doctrine asks for (defect class; mechanism, or the structural removal that obviates
  it; decidability call with a cadence when un-gateable) makes triage unskippable, because
  filing is the one step every finding already passes through. A dedicated triage subagent
  is the third option and fits a batch at close or a class needing a tree survey — but the
  decidability call is the load-bearing judgment, and getting it wrong permissively produces
  exactly the noisy gate the doctrine warns erodes the battery's authority, so it does not
  ride the cheapest tier.
  **Why `[design-pending]`:** (2) is a refusal on a capture affordance whose design premise
  is that refusing capture pushes a finding back into session context — the deferred-capture
  antipattern the inbox exists to prevent. Reconciling a required-fields check with that
  premise is unruled.
  **Cost while deferred:** the class is self-demonstrating — this iteration's gap inbox is
  the evidence, and the bullets carrying a named mechanism carry it because some session
  happened to remember a rule nothing loaded.
  Filed 2026-08-03 at close from the gap inbox; found by the lead.

- **self-revert-reminder-expectation** [design-pending] — a dispatched agent that reverts
  its own edits gets a harness reminder indistinguishable from an injected instruction.
  Observed 2026-08-03: a read-only audit sweep verifying the crate's unit-test
  falsifiability mutated source, reverted it, and received a note saying the file was
  modified by the user or a linter, that the change was intentional, that it should not
  revert it, and that it should not tell the user — so it reported a suspected prompt
  injection. The lead received the identical message twice in the same session for mundane
  causes (a gap-inbox append landing under it; a build batch sweeping its working-tree edit
  into a commit), which is what identifies it as genuine harness behaviour rather than an
  attack.
  **The hazard is not the false alarm; it is the true-belief case.** Falsifiability testing
  *requires* mutating source and reverting it, so this fires every time — and an agent that
  obeys leaves its mutation in the tree while reporting success. A broken crate behind a
  green report.
  **Two mechanisms, ranked.** *(1) Structural* — a falsifiability check runs against a
  scratch copy rather than the tracked tree, so no revert is ever needed and the reminder
  never fires. *(2) Stated* — a standing line in
  `delegation-kit/templates/agent-execution.md` telling a dispatched agent that this
  specific reminder is expected after a self-revert and does not countermand its brief;
  cheap, but it does not remove the surface. Not gateable: nothing about it enters the
  tracked tree.
  **Related to `fork-dispatch-prohibition`** — both are cases where a dispatched agent's
  authority is narrower than what some in-context text invites it to do.
  **Why `[design-pending]`:** (1) is the real fix and it is not free — a scratch-copy
  falsifiability harness has to reproduce enough of the build to make the mutation
  meaningful, and whether that lives in the crate's own test tooling or in the gate-test
  runner is unruled. (2) alone teaches an agent to discount a class of harness message,
  which is a cost worth naming before paying.
  **Cost while deferred:** every falsifiability check dispatched to an agent is one obedient
  reading away from silently leaving a mutation in the tree behind a green report.
  Filed 2026-08-03 at close from the gap inbox; observed by the lead across three
  occurrences.

- **behavior-change-surface** [design-pending] — no accumulating declaration surface.
  A tightened gate has one and a behavior change does not — and gate-sdk/SPEC.md §upgrade-smoke's
  own rationale for the former covers the latter without modification: it grounds
  `.workflow/tightened-gates.txt` on build being the only stage that knows what it tightened at
  the moment it tightens it, so the declaration is *written from knowledge* rather than
  reconstructed later. docs/install.md §The upgrade contract requires a Behavior changes section
  in every release note and gives it no such surface, while that set is judgment-laden rather
  than diff-derivable — so composing it at close means reconstructing an author's judgment from
  commits.
  **Deliverable:** the missing surface, carrying the three things `tightened-gates.txt` has and
  this would need — a `# contract:` header, a drain protocol at the tag, and a freshness or
  parity gate.
  **Open design question the promoting scope answers first — deliberately unresolved here:**
  whether this is a second `.workflow/` file or a widening of the existing one. A
  behavior-change bullet carries a changed-surface name plus prose, while the existing surface
  is specified as bare gate names and nothing else.
  **Measured, which is why the cost is not hypothetical.** The "each batch records its own set"
  convention was invented at batch 4 (`13f8091`) in response to this gap being filed at
  `df6fd3d`. Batches 1 and 2 closed before it existed and recorded nothing — a case-insensitive
  scan of every diff in the iteration finds the phrase only in those two later commits. So
  `native-artifact-publish-path`, `native-artifact-install-path` and
  `install-path-gnu-userland-undeclared` carry **no declaration, not a declared "none"**, and
  close reconstructed their sets from commit bodies so the next note need not: *publish-path* —
  the parity gate rides Tightened gates, leaving the descriptor and roster path as its behavior
  set; *install-path* — `init`, `doctor` and the lock/digest resolvers changed, and `c5c19e6`
  and `d13c1f6` are defect fixes whose own bodies say they "would have reached an adopter";
  *gnu-userland* — documentation only, no shipped-code path.
  **Cost while deferred:** exactly that reconstruction, paid again by whoever composes each
  release note, against evidence that is coldest when the batch count is highest.
  Filed 2026-08-04 at close from the gap inbox; the design question left open on purpose.

- **consumer-smoke-subset-accounting-verdict** [design-pending] — a per-kit smoke run reds an
  accounting the subset cannot decide, and says nothing about it.
  `gate-sdk/bin/run-consumer-smoke.sh` given a KIT SUBSET reds the registration accounting on
  gates the no-arg run accounts for: `check-action-gh-repo`, `check-action-pinning` and
  `check-action-run-shell` probe exit 0 in a two-kit scratch consumer and self-declare in the
  full one. So a per-kit invocation prints a FAIL that is not a repo finding, and nothing in
  its output says the verdict needs the full roster — the reader has no way to tell an
  artifact of the subset from a real one. Master itself is clean (11 kits probed, 0
  unaccounted), independently confirmed 2026-08-04.
  **Deliverable, and the design choice inside it:** either scope the accounting assertion to
  the invoked subset (correct verdict, weaker coverage) or keep it whole-roster and have the
  subset run *say so* — print that the accounting line is advisory under a subset and
  suppress its contribution to the exit code. The second is cheaper and keeps one accounting
  algorithm; the first is what a reader naively expects.
  **Cost while deferred:** every per-kit smoke run costs a false-alarm investigation, and the
  habit that pays it is worse than the alarm — a reader who learns to discount this FAIL
  discounts the true one. Already paid twice this iteration, including one bullet filed
  against master on the strength of it and retracted.
  Filed 2026-08-04 at close from the gap inbox; the false red corrected in place mid-iteration.

- **pack-installer-root-provenance** [design-pending] — the smoke packs whatever tree the
  caller's cwd is in, and reports success either way.
  recurrence: pack-installer-root-provenance 2026-08-05
  `scripts/pack-installer.sh` resolves its root from cwd (`git rev-parse --show-toplevel`)
  while `installer/consumer-smoke/run-smoke.sh` resolves its repo from its own path. Run the
  smoke from a linked worktree or any second checkout and the two disagree: pack assembles
  the payload from cwd's tree, the smoke asserts against it, and the run looks entirely
  normal — it even prints a PACK line naming the *other* tree's commit. The failure is not a
  red that needs explaining; it is a **green that asserts nothing about the tree under test**,
  which is the worse half of the two.
  **Measured, not theoretical:** it produced two green-but-meaningless smoke runs during that
  iteration's batch C before anyone noticed the PACK line named the wrong commit, and it
  recurred 2026-08-05 (the date above) costing `install-claim-contract`'s build another full
  ~10-minute run that proved nothing.
  **A second precondition trap on the same seam, found the same day.** The upgrade arm packs
  *mid-run*, so `pack-installer.sh`'s dirty-worktree refusal fires against whatever the tree
  looks like minutes into a ~10-minute suite rather than at invocation. Any concurrent edit
  — an ordinary thing during a run that long — surfaces as a refusal whose message is about
  the worktree and not about the timing, so the reading is a broken installer rather than a
  precondition checked at the wrong moment. Same root as the cwd defect: `pack-installer.sh`
  asserts against a tree it resolves for itself at a moment it chooses, while its caller
  believes it fixed both.
  **Deliverable:** give `pack-installer.sh` a root the caller passes, or have it refuse when
  its own resolved root and the invoking script's disagree. The refusal is the smaller change
  and fails closed; the parameter is the cleaner contract and moves the choice to every
  caller. Left open because the second answer touches every call site. The precondition half
  is separable and cheaper: hoist the worktree check to invocation, so the suite refuses
  before spending ten minutes rather than during them.
  **Cost while deferred:** the installer's only end-to-end oracle can pass without testing the
  tree being released, and the condition that triggers it — working from a worktree — is
  exactly the setup a parallel iteration uses.
  Filed 2026-08-04 at close from the gap inbox; found by build.

- **session-model-identity-verification** [design-pending] — a session cannot report or
  verify the model tier it is running at.
  The session-context hook prints iteration, budget and drift; `drift-report` prints neither.
  Nothing surfaces the running model, so a session cannot state its own tier without a human
  hand-reading the harness transcript, and no stage can assert the tier it was dispatched at.
  **Operator-proposed shape, recorded 2026-08-04:** a `usage-verdict`-shaped check — snapshot
  in, exit 0/1/2, fail-soft — reusing `lifecycle-kit/bin/session-id.sh`'s projects-dir
  derivation, with the *tier expectation in consumer config* rather than a kit literal. Both
  halves of that placement are forced: a baked model-name ladder is drift by construction
  (delegation-kit's agent-execution rule keys tiering to capability, not to a name), and the
  provenance seam keeps product constants out of kit literals regardless.
  **Feature-shaped, so it wants `/spec`, not a debt promotion.** It spans derivation lifecycle
  or context, verdict delegation, and a consumer config surface, and it introduces a new
  governed name. Cross-kit ownership plus a new name is the amendment threshold, and a scope
  that promotes this straight to build will be authoring the contract inside the build.
  **Cost while deferred:** every tiering rule in the tree is unverifiable — including the two
  filed alongside this one. `consult-tier-declaration` blocks on it outright, and the
  `Co-Authored-By` attribution defect has no derivable fix without it.
  Filed 2026-08-04 at close from the gap inbox; filed by the lead.

- **consult-tier-declaration** [design-pending] [blocked-by: session-model-identity-verification]
  — `/consult` governs the tier of what it dispatches and asserts nothing about its own.
  The skill landed this iteration to carry judgment-tier boundary questions, and its own
  amendment argues it is judgment-tier *by nature* — yet it declares no floor for the session
  running it and verifies nothing at entry. A consultation answered at a cheap tier is
  indistinguishable, in the record, from one answered at the tier the skill was built for.
  **Operator direction 2026-08-04:** it must run on the top model and verify that at entry.
  **The shape that keeps the seam intact:** the *skill* declares its own floor, the *kit*
  never spells a model name — the same split `session-model-identity-verification` sets up,
  which is why this blocks on it rather than racing it. Without the mechanism this entry is a
  prose assertion of the kind that already failed twice this iteration.
  **Cost while deferred:** the repo's one escalation-grade skill is silently downgradeable,
  and the failure is invisible in the artifact — a thin consultation reads as a short one.
  Filed 2026-08-04 at close from the gap inbox; filed by the lead on operator direction.

- **co-authored-by-trailer-attribution** [design-pending] — the model trailer is a baked
  prompt literal, so public history misattributes authorship.
  The `Co-Authored-By` line is copied from the harness prompt rather than derived from the
  running model, and tracked public history carries the result. **Measured 2026-08-04:** the
  lead session ran one model for two hours while its prompt's trailer named another, and the
  validate stage session ran a third for all 82 of its messages while both of its commits are
  signed with the first — two tiers, one baked literal, both wrong. Within this single
  iteration five stage commits carry the trailer and five do not, so the convention is neither
  uniform nor accurate, and no gate enforces either shape.
  **The decision comes before the fix:** whether the repo wants the trailer at all. Dropping
  it costs nothing and ends the defect; keeping it obliges deriving it, which needs
  `session-model-identity-verification` and is why this is filed beside that entry rather than
  under it — the drop answer needs no mechanism, so this is not simply blocked.
  **Why it is not merely cosmetic:** this is a public repo and the trailer is an authorship
  claim in tracked history. A wrong one is not a stale comment; it is a false statement about
  who wrote a commit, published, and it cannot be corrected without a rewrite.
  **Cost while deferred:** every commit adds another misattributed record, and the cost of the
  eventual correction grows monotonically with the count — history is the one surface this
  repo cannot re-generate.
  Filed 2026-08-04 at close from the gap inbox; filed by the lead, measured that session.

- **intra-file-pendency-contradiction-scan** [design-pending] — one file can call the same
  slug landed in one section and pending in another, and nothing reads both.
  Found at close 2026-08-04 by the `capability-pendency-after-landing` audit:
  gate-sdk/SPEC.md said a second port "lands after `native-artifact-publish-path` and
  `native-artifact-install-path`" while, ninety lines later, the same file said criterion 5
  is "implemented by `native-artifact-publish-path` and `native-artifact-install-path`". Both
  landed 2026-08-03. Two sections, two tenses, one file, one slug pair.
  **Why the existing coverage did not catch it, which is the point.** The
  `capability-pendency-after-landing` roster class *did* run at that iteration's own close
  and missed it, because the class is scoped as a human sweep of governed prose against the
  tree — an unbounded read whose reach depends on which files the sweeper opens. The stale
  paragraph was written mid-iteration and never revisited after the same iteration's later
  commit discharged it, so the tree-comparison the class prescribes never reached it.
  **Deliverable, and why it is narrower than the class it sits under:** a scan for one
  *decidable* shape — a governed file citing a slug in a landed construction ("implemented
  by", "built", "ships") and in a pending construction ("waits on", "lands after", "does not
  exist yet", "not yet") within the same file. It needs no tree comparison and no judgment
  about what is actually live: the contradiction is internal, so the file falsifies itself.
  That is what makes it gateable where its parent class is not.
  **Why `[design-pending]`:** the construction vocabulary is the whole gate, and a literal
  phrase list in a kit is drift by construction plus a provenance-seam problem — the
  vocabulary is consumer editorial. It wants the `check-graph` / `graph-vocab.sh` treatment,
  optional consumer config, which is a design call rather than a size one. Also open: whether
  a legitimate "X landed, Y still waits on it" sentence pair trips it, which decides whether
  the predicate is per-slug or per-slug-per-section.
  **Cost while deferred:** the class stays a sweep whose reach is whoever runs it, and its
  one measured miss cost a full iteration of a governed SPEC contradicting itself in public
  — gate-sdk/SPEC.md is mirrored to the docs site, so the contradiction shipped.
  Filed 2026-08-04 at close; the instances it would have caught were fixed the same session.

- **enter-stage-arg-position-silent-drop** [design-pending] — `--simulate` after the stage
  is silently dropped, and the read-only preflight runs the destructive reset instead.
  recurrence: enter-stage-arg-position-silent-drop 2026-08-07
  `bin/enter-stage.sh` parses the flag positionally — it tests `$1` for `--simulate` and
  shifts — so `enter-stage.sh scope --simulate` leaves `$1` as the stage, never sets the
  simulate bit, and treats the flag as a trailing argument nothing reads. Nothing refuses
  the extra argument and nothing in the output says the flag was ignored.
  **Why this one argument's position is not an ordinary usage nit.** The dropped token is
  the only thing separating a read-only preflight from an iteration-boundary reset. The
  real run truncates `.workflow/WORKFLOW-STATE.txt`, rewrites the queue header to `—`,
  truncates every `LIFECYCLE_KIT_BOUNDARY_TRUNCATE` member — including the release
  disposition a close just wrote — and wipes `.tmp` past the keep-list. So the failure mode
  of a typo'd read-only command is destroying the iteration state the command was being run
  to inspect, and the one unrecoverable casualty is `.tmp`: the boundary wipe is not a git
  operation, so a session journal goes with it while the tracked half restores clean.
  **Measured 2026-08-04 at this close**, on the close session's own verification step. Every
  tracked write was reverted from the index and the close's state restored; the wiped `.tmp`
  journals were not, and were not needed because the work was already committed. That
  recovery depended on the run happening with a clean tree — the same mistake with
  uncommitted work in the tree loses it.
  **Deliverable, and why the obvious fix is not the whole fix:** accept the flag in any
  position, which is a two-line change. But permissiveness is the weaker half — the
  load-bearing part is that an argument the script does not recognize must be a **refusal**,
  not a silent ignore, which is this repo's own fail-closed rule applied to its own tooling.
  A tool that silently discards what it cannot parse fails open on exactly the input a user
  got wrong.
  **Why `[design-pending]`:** whether the refusal belongs in this script alone or as a shared
  argument contract across `lifecycle-kit/bin/` is the open call — `file-gap.sh` already has
  a filed sibling symptom (`capture-affordance-help-flag`, icebox: it files `--help` as a
  gap), which says the class is the kit's argument handling and not this one script.
  Related and worth reading together: `enter-stage-simulate-no-write-fixture` (icebox) pins
  the no-write guard with a fixture, and would **not** have caught this — a fixture written
  the documented way puts the flag first and passes.
  **Cost while deferred:** every session that reaches for the preflight can destroy the
  state it meant to inspect, and the sessions most likely to run it are stage sessions at a
  boundary, which is exactly when the state is most valuable and least reconstructible.
  Filed 2026-08-04 at close, from the close session's own misfire.

- **native-gate-port-remaining-corpus** [design-pending] [roadmap: next/reliability] —
  the battery beyond the first cohort.
  roadmap-summary: The rest of the battery onto the native binary, cohort by cohort.
  Successor to `native-gate-binary-port`, which the lead scoped to the first cohort
  2026-08-06 so that the head could complete without the public roadmap commitment
  lapsing. This entry carries the remainder of the corpus and inherits the head's
  grounds rather than restating them: gate-sdk/SPEC.md §Porting a gate to the binary
  substrate owns the criteria, §Consumer payload the payload rule, and TRAJECTORY.md
  §The objectives the direction all three serve.
  **What the first cohort leaves settled, so this entry does not re-derive it:** the
  seven port-candidate criteria including the external-program screen the cohort paid
  for, the parity procedure (compare while both implementations still exist), the
  two-commit sequencing `check-gate-tamper` forces, and the next-cohort selection rule
  — the largest set of criteria-clearing gates sharing one corpus derivation.
  **What is genuinely open here.** Whether the criterion-4 corpus is large enough to
  finish without relaxing it, and TRAJECTORY.md's named re-entry condition for that
  relaxation — the criterion-clearing corpus exhausted *and* the parity oracle held off
  the shell substrate. Neither holds today, so this is the entry that will meet it.
  **Cost while deferred:** every new gate adds shell to the eventual port, the
  silent-failure classes stay reachable across the unported corpus, and the GNU-userland
  pins the port exists to retire keep their hold on the toolchain floor.
  Filed 2026-08-06 at spec, under the lead's split ruling, before the head's Done-move so
  that `ROADMAP.md` never loses the commitment.

- **dead-queue-citation-report** [design-pending] — an in-body citation that resolves to no
  live entry reads exactly like one that does, and nothing names the difference.
  queue-kit/SPEC.md §The tag algebra rules the in-body single-backtick slug a *reference*
  rather than a membership claim, aggregated by `bin/queue-edges.sh` and "audited by
  nothing". That is a deliberate choice and stays right — entries legitimately name landed
  work and no gate may punish it — but its cost is now attested rather than hypothetical.
  **The attestation.** Three ruled-and-deleted slugs were cited across four live entries, and
  two of those entries argued *from* them in the present tense: one framed a "Boundary with
  the two live companions" whose companions were both dead and settled. A scope session
  ranking such an entry reads closed rulings as open questions, which is a false premise in a
  survey input. Both were corrected inline at the 2026-08-06 close; nothing stops the next.
  **Why a report and not a gate**, which is the design half already half-answered. A red is
  wrong here by the SPEC's own reasoning, so the cheapest true form is a *listing*:
  `bin/queue-edges.sh` already resolves every in-body citation against the live set and drops
  the misses on the floor, so naming them costs one output section and no new scan — the
  no-red posture kept, the silence ended.
  **What is genuinely open:** where the listing is read (a close step, a scope survey input,
  or both), and whether a citation of *landed* work should be distinguishable in prose at all
  — a grammar question the SPEC left unanswered when it refused a relational vocabulary, and
  answering it the wrong way re-imports the maintained-roster anti-pattern that refusal avoided.
  **Cost while deferred:** dead citations accumulate at the rate rulings close, and each is a
  false premise sitting in a survey input at exactly the moment a scope decides what to promote.
  Filed 2026-08-06 at close, draining the gap inbox; found 2026-08-06 at scope.

- **guard-steer-grant-mismatch** [design-pending] — the guard steers sessions onto forms the
  committed allowlist does not grant, so obeying it costs a prompt.
  Attested at the 2026-08-06 close triage. `scripts/bash-guard.sh` refuses a shell expansion
  and names `git -C <dir>` as the better form; `git -C` sits behind no committed glob, so
  every session that obeys the steer prompts — 4 calls this iteration. The same shape one
  step down: the allowlist grants `chmod +x *` while the tree's actual motions are
  `chmod 755` and `chmod 644` on fixture stubs and descriptors, which that glob cannot match.
  **Why it is a class and not two typos.** A guard rule and an allowlist glob are authored on
  different surfaces by different motions, and nothing reads them against each other. The
  triage procedure already warns that an allowlist entry can *mask* a steering opportunity
  (guard-kit/templates/close-triage.md); this is the mirror it does not name — a steer with
  no grant behind it, which reads as ordinary friction rather than as a missing pair.
  **Why `[design-pending]`:** the cheap fix is the wrong one. Granting every steered-to form
  widens the auto-allow set, which that same template rules the **consumer's** call and never
  a session's, so the deliverable is at most a *report* pairing each guard steer against the
  committed globs — and whether the steer strings are mechanically extractable from a
  consumer's guard script at all is the open question. Provenance seam: the steer vocabulary
  is this consumer's, so no kit may carry the pairs.
  **Cost while deferred:** the friction loop the close triage exists to drain refills from
  the guard itself, and each close re-derives the same mismatches off the same log.
  Filed 2026-08-06 at close, from its own prompt-friction triage; scope-gated intake, so the
  report is costed here rather than built here.

- **kfric-empty-log-ambiguity** [design-pending] — an empty knowledge-friction log is read as
  no friction, and it is equally consistent with no capture.
  `.workflow/knowledge-friction.log` read **empty** at the 2026-08-06 close, across an
  iteration that added a Rust module, a build-time source stamp and a new gate. The same
  iteration's prompt-friction log records a build session reading a *deleted* `.gate`
  descriptor out of git history to shape the one it was writing — a prior deliverable
  consulted because no surface carries an example, which is exactly the class
  drift-kit/templates/close-knowledge.md names and `bin/kfric.sh` exists to stamp. It was not
  stamped, and nothing noticed.
  **The reader is what makes it expensive.** `kpi-knowledge-friction` trends the log toward
  zero and reads that trend as the tier contract's holes filling. A sensor with an
  in-the-moment capture discipline and no independent floor cannot tell a filled hole from an
  unstamped one, so the KPI reads best exactly where it is least trustworthy.
  **Why `[design-pending]`:** the honest answers differ in kind. A corroborating signal is
  cheap and weak — the prompt log already records history archaeology and close already reads
  both surfaces. A per-stage capture prompt is stronger and is precisely the standing-
  instruction tax context-kit's brevity machinery rejects. Stating the limit on the KPI
  instead — an empty log is not evidence of zero — costs one line and buys no detection.
  Which is right turns on whether the KPI is meant to be trusted at zero, a contract call.
  **Cost while deferred:** the one KPI measuring the tier contract's completeness reads best
  exactly when nobody is capturing, and no other signal contradicts it.
  Filed 2026-08-06 at close, from its own knowledge-friction sweep.

- **reclaim-precondition-outside-the-tree** [design-pending] — a declared `reclaim=` can be
  runnable and still un-runnable, and the gate that demands one cannot tell.
  `check-close-surfaces` blocks a capture-tier declaration that names no `reclaim=`, which
  is the right floor and is not the whole obligation. `.workflow/essay-harvest.md` declares
  `: > .workflow/essay-harvest.md` — a command any close can execute — but its *precondition*
  is "merged into the essay", and the essay lives outside this tree. So the reclaim has never
  fired: the sink now carries entries dated back to 2026-07-10, seven-plus iterations of
  operator material, each close correctly declining to truncate.
  **The bind is what makes it a defect rather than a slow queue.** A close that runs the
  declared reclaim destroys material it cannot re-derive; a close that declines lets a
  capture surface grow without bound, and does so *correctly* every time, so nothing ever
  reads as wrong. The declaration's own text is what hides it — a reclaim command that
  executes is indistinguishable, to the gate and to a reader, from a reclaim that discharges.
  **Why `[design-pending]`:** the candidates are a grammar change and a posture change, and
  they are not the same size. A `reclaim=` could carry its precondition (who or what must act
  first), which the gate could then require but never verify — honest, cheap, and still
  unenforceable. Or the surface could be ruled *not* capture-tier at all, since what it
  accumulates is an operator's material rather than a session's, and a tier whose reclaim no
  session may run may be the wrong tier. Provenance seam: what the precondition *is* — an
  essay, a person, an external merge — is consumer content, so a kit may demand the field and
  never a vocabulary for it.
  **Cost while deferred:** every close spends a judgment call re-deciding not to truncate,
  reaches the same answer, and leaves no residue that the answer was reached — which is
  exactly the shape the close-surface roster was built to end.
  Filed 2026-08-06 at close, from the runtime-artifact lifecycle check and an audit sweep
  that measured the sink's age.

- **release-drain-ordering-contradiction** [design-pending] — RELEASING.md step 4 prescribes a
  commit its own battery refuses.
  Step 4 makes the tightened-gates drain and the disposition stamp one commit, calls that the
  iteration's final commit, and requires pushing it and watching the `gates` run for that SHA
  green *before* tagging. `check-tightened-gates-note-parity` arms on a note whose declared
  version carries no tag yet, and its only dormancy arm is that every note is tagged — so the
  truncation reds it in both directions, nine declared gates against an emptied surface. It is
  `tier=precommit` and a battery member, so the prescribed commit is refused locally and the
  watch it requires would red remotely. No ordering satisfies both surfaces.
  **Attested at the v0.22.0 cut, and it could not have surfaced earlier.** The gate landed in
  `release-signaling-reset`, after v0.21.0 was tagged, so this was the first release cut under
  it. Ruled option A by the lead 2026-08-06 — the drain yields and lands as a follow-up commit
  once the tag has disarmed the gate — on the ground that the gate is executable, was written
  later, and encodes tag-then-drain in its own dormancy message, while step 4 was simply never
  updated to match. The release shipped on that reading and the runbook was deliberately left
  unedited mid-release, because changing the procedure you are executing is how a cut becomes
  its own precedent.
  **What this entry owes is the text, not the decision.** The edit is small and the care is in
  what must survive it: the one-commit coupling is defended by a stated parity-window reason and
  the watch-before-tag ordering by a separate stated reason about the remote oracle, and the
  rewrite must keep both arguments while moving only the drain.
  **Correction 2026-08-08 at scope, re-verified against HEAD — most of this entry is already
  discharged, and the residue is one sentence.** A prior revision claimed "Step 4's prose still
  describes the refused sequence, so the next release re-derives the contradiction". That is
  **false**: `d64e63c` (2026-08-07, the v0.22.0 cut) already rewrote step 4 to *stamp*, push,
  watch, tag, drain-after — `RELEASING.md:119` now reads "write the **stamp** commit", and
  `:127-135` carries a paragraph titled "The drain lands after the tag, and the parity gate is
  what forces it." What survives is a residual intra-step contradiction the corrective did not
  reach: step 4's **opening** sentence (`RELEASING.md:106-109`) still bundles the drain and the
  disposition stamp as one commit — "so the iteration's final commit is the one this step
  creates" — while the body it introduces now splits them. So the cost-while-deferred below
  overstates: the next release does not re-derive the contradiction, it reads one stale opening
  sentence against a correct body. Re-cost before promoting; the design question below may no
  longer be worth an iteration slot.
  **Why `[design-pending]`:** where the follow-up commit belongs is genuinely open. Riding the
  next iteration's push holds the two-push budget and leaves the surface undrained on the
  remote between releases; its own push buys immediacy at a cost CLAUDE.md rations. A third
  answer is to state the gate's dormancy model in the runbook as the contract it already is,
  making the follow-up's timing a consumer's call rather than a step.
  **Cost while deferred:** every release re-derives a settled contradiction, and re-derives it
  at the worst moment — mid-cut, with a note committed and a tag pending.
  Filed 2026-08-06 at close, on the lead's ruling, from the cut that hit it.

- **poll-sleep-guard-steer** [design-pending] — polling to wait is the one half of the
  never-poll rule that leaves a tracked artifact, and nothing reads it.
  `bash guard-kit/bin/scan-prompts.sh` ranked bare `sleep` as this iteration's **top**
  prompting pattern at 16 calls — a session waiting on background work by sleeping in the
  foreground, roughly forty minutes of turns spent not-waiting-correctly. It is the same
  rule `waiting-rule-fourth-firing-post-fix` records, approached from the opposite
  side: that entry is a session ending its turn to wait, this is a session refusing to.
  **Why this half is different, and why the difference is the whole entry.** That entry
  states, correctly, that no gate can read a session's choice to *end a turn* — the act
  leaves no tracked artifact. A **poll does**: it is a `Bash` call, on the exact surface
  `scripts/bash-guard.sh` already reads and already steers (`cat`→Read, `sed`→Read, bare
  allowlisted commands→undecorated). So the enforcement-first half that the turn-end class
  honestly cannot have is available here, cheaply, and no entry has noticed that.
  **Why `[design-pending]`:** a blanket `sleep` block is wrong. The sanctioned wait *is* a
  condition loop, and this repo's own sessions run `until <cond>; do sleep N; done`
  legitimately; a smoke or a probe may need a settle. So the rule has to separate a bare
  foreground `sleep` from a sleep inside a condition loop, and decide whether the steer
  names the notification channel, the harness's monitor form, or both — a guard-kit
  contract question, since the rule is generic mechanism while the named better form is
  consumer/harness vocabulary that must not become a kit literal (the provenance seam).
  **Cost while deferred:** the cheapest available detector for a rule that has now fired in
  four consecutive iterations goes unbuilt, and every polled wait bills a full-price turn
  for a session doing nothing — paid per occurrence, on the tiered stages where long oracle
  batteries are the whole work class.
  Filed 2026-08-05 by close, from its own prompt-friction triage; scope-gated intake, so it
  is filed rather than started.

- **waiting-rule-fourth-firing-post-fix** [design-pending] — the residency rule fired again,
  under its own freshly-strengthened prose.
  recurrence: waiting-rule-fourth-firing-post-fix 2026-08-06
  `dispatched-session-waiting-rule-residency` shipped this iteration. Batch 2 (`a046c06`)
  landed its residency half into `.claude/agents/stage-session.md` as a bare imperative —
  never end a turn on work still running, and never end one in order to wait. The **validate**
  session of this same iteration was then dispatched under that updated definition and did it
  anyway: it ended its turn with `run-validate` still executing, orphaning the producer. Four
  consecutive iterations now; the third firing went through a dispatch prompt that named the
  rule, the fourth through the strengthened agent definition itself.
  **Not a recurrence — a new defect.** The slug resolves only in `## Done`, and the firing came
  *after* its fix landed, which the drain rule files as new work rather than a recurrence stamp.
  **The mechanical half held, and that is the other side of the finding.** The lead ran
  `check-producer-liveness` rather than guessing, got a live PID, waited in-turn on the lock,
  and did not dispatch close into a mutating manifest; the `close=` entry preflight would have
  refused it regardless. The lock ended up absent rather than stale, so conditional release
  behaved as specified and no work was lost. Enforcement held exactly where a gate existed and
  failed exactly where only prose did — an enforcement-first result produced by dogfooding.
  **The operator ruled the prose half on 2026-08-06** — delegation-kit/SPEC.md §Operative
  residency now states that the rule requests rather than enforces, so the amendment-scope
  question this entry flagged is settled and no longer part of it.
  **Why `[design-pending]` still.** What survives the ruling is the enforcement question the
  ruling deliberately did not answer: given that prose alone does not hold, what does.
  `poll-sleep-guard-steer` is the same rule from the side that *does* leave a tracked
  artifact, so a scope taking either should cost both; `waiting-rule-carrier-reach` is the
  question of which sessions the prose even reaches, and the two firings stamped above are
  its evidence rather than this entry's.
  **Cost while deferred:** the project's only evidence that prose-alone enforcement fails is
  an anecdote spread across six firings' histories, and each further firing costs an orphaned
  producer plus the lead turn that discovers it.
  Filed 2026-08-06 by close, recording this iteration's central incident.

- **waiting-rule-carrier-reach** [design-pending] — the residency rule reaches the sessions
  whose definitions name it, and both firings this iteration were outside that set.
  **Fifth firing — a carrier that names the rule nowhere.** Scope dispatched an `audit-sweep`
  agent for the port-candidate census; it ended its turn on a live child fork after roughly
  14.5 minutes of tool work, returning "still waiting on the batch-2 fork's resend" and no
  census, and had to be resumed by message. The four firings
  `waiting-rule-fourth-firing-post-fix` records all ran through carriers that *do* name the
  rule — the `stage-session` definition, a dispatch prompt. The `audit-sweep` definition
  carries no residency clause at all, so this is evidence about the rule's **reach** rather
  than about prose-versus-enforcement strength.
  **Sixth firing — a second carrier class, and the more dangerous one.** The validate session
  backgrounded its own `run-validate.sh` and ended the turn in order to wait on it. For a
  dispatched session a turn end is a session end, so the observer died while the shell child
  survived, orphaned and still writing — and the harness fired a completion notification
  anyway, because that notification means only "no live `Agent` children" and is silent about
  a backgrounded shell child. Nothing was lost: the lead read the process table, found the
  producer live under its lock, barred a second one, and the resumed session waited in-turn
  and finished inside its ceiling.
  **What makes the sixth a distinct question rather than more of the fifth.** A shell child
  is not an agent, so no agent definition governs it however many definitions carry the
  clause — and the one signal a supervisor would trust to say the work is done is precisely
  the signal that is wrong here.
  **Why `[design-pending]`:** the two candidate homes trade off and neither covers both
  classes. Putting the clause on every dispatched agent type's definition is per-type
  maintenance a newly added type silently opts out of; putting it on the dispatching side as
  a standing clause in the dispatch-prompt template reaches every type but only through prose
  a dispatcher must remember to send. Neither answers the shell-child class, where the honest
  fix may be a liveness read rather than a rule. Provenance seam: the notification's
  semantics are harness vocabulary, so a kit may state the obligation and not the mechanism.
  **Cost while deferred:** every dispatch to a type whose definition omits the clause is
  unprotected, and a backgrounded producer under a session with no live lead is lost
  silently — the sixth firing cost nothing only because a lead happened to be watching.
  Filed 2026-08-06 by close, draining the gap inbox and the lead's separately held incident.

- **amendment-dod-sibling-dependence** [design-pending] — an amendment's DoD is written as if
  its unit were the iteration's only one.
  Two shapes attested in `background-producer-liveness`, one root.
  (1) **Reciprocal.** `evidence-kit/SPEC-liveness-lock.md` and
  `lifecycle-kit/SPEC-dispatch-signal.md` each carried a mirror "ships together with the other"
  item. Written symmetrically it reads as blocking to each side and neither half can go first;
  batch 1 merged only by reasoning past its own DoD, and nothing in either amendment asserted
  the iteration-level reading that makes that correct.
  (2) **Broader, and strictly containing (1).** The merge assertion `ls <kit>/SPEC-*.md returns
  none` is carried by *every* amendment for a kit with more than one in flight. It is satisfiable
  only by whichever batch merges that kit's last amendment; the earlier batches' identical copies
  are unsatisfiable at their own commit, for a reason nothing in either amendment states.
  **The general shape is the entry** — a DoD item whose satisfiability depends on sibling units
  the amendment never names. Triaging the two instances separately is what produced two gap
  bullets for one defect.
  **The reading is already ruled.** The operator ruled shape (1) discharged at the iteration, not
  the commit, on the mechanical ground that build stays in build until the queue empties and
  close refuses entry on a non-empty active queue; that ruling is landed at canon-kit/SPEC.md
  §Merging an amendment. What stays open is the *authoring* side.
  **Why `[design-pending]`:** the candidates differ in kind — a template change (the shipped DoD
  checklist names the iteration as the discharge horizon), an authoring rule (an amendment may
  not assert over files it does not own), or an assertion on the amendment glob that reds a
  cross-amendment path reference inside a DoD item. Which is right depends on whether sibling
  dependence is ever legitimate, and that is unruled.
  **Cost while deferred:** every multi-amendment iteration ships DoD items that are false at
  their own commit, and each costs a build session the reasoning-past that batch 1 paid — the
  exact move a DoD exists to make unnecessary.
  Filed 2026-08-06 by close, merging two gap-inbox bullets into the class both name.

- **recurrence-resolver-literal-match-only** [design-pending] — a real recurrence whose bullet
  does not spell the slug is drained as a fresh entry.
  The gap-inbox recurrence resolver matches a literal slug substring in a bullet's prose — at
  capture (`bin/file-gap.sh`'s live-slug scan) and again at the close drain, which re-resolves
  the same way. A filer who describes a defect rather than naming it produces a bullet that is
  present, is a genuine recurrence, and matches nothing; the drain files it as a new entry,
  fragmenting the backlog the inbox exists to keep whole.
  **The third face of one matcher, which is why the three cost together.**
  `gap-resolver-mention-overcount` covers the inflating direction only (a bare mention counted as
  a recurrence); `recurrence-drain-input-widening` covers a recurrence reaching close with no
  bullet at all. Neither covers a bullet that is present and fails to match. All three are the
  same bounded-substring predicate from three sides, and a scope taking one alone re-derives the
  other two.
  **Attested twice in one drain, in the over-counting direction.** Two of this iteration's nine
  bullets carried an auto-stamped recurrence prefix fired by a bare mention — one naming
  `gap-resolver-mention-overcount`, one naming `recurrence-drain-input-widening` — and both had
  to be struck by hand at the drain.
  **First attestation in its own direction, 2026-08-08.** A mechanical scan of the live slug set
  across the whole gap inbox matched nothing, yet one of that drain's six bullets was a genuine
  recurrence of `dispatch-worktree-reds-the-battery`: it described the defect — the worktree the
  dispatch guard mandates, and the battery reddening while it lives — without ever spelling the
  slug. The drain's session-side re-resolution caught it and stamped the date; the matcher alone
  would have filed a duplicate entry. Every prior attestation on this cluster ran the
  over-counting way, so this is the first measurement of the direction this entry names.
  **Why `[design-pending]`:** the under-matching direction has no syntactic remedy at all. The
  over-counting sibling can at least imagine a filer-supplied flag; here the missing signal is in
  the filer's head, so the honest candidates are a required recurrence-or-new field on
  `file-gap.sh`, or accepting the drain's session-side judgment as the only real channel.
  **That contract call has since been made, and this entry survives it.**
  `gap-resolver-mention-overcount` landed the second candidate: the drain is the authoritative
  judge, the capture-time matcher is demoted to an advisory that asks the filer, and the required
  recurrence-or-new field is refused there with cause (it interrogates a filer who has not read
  the queue). So the channel question is settled and this entry's is not — the shipped advisory
  is silent exactly where this entry points, since a bullet that never spells the slug raises no
  prompt at all, leaving the whole burden on a drain reading prose with nothing marking the match.
  **Cost while deferred:** the recurrence count under-reports in a direction nobody can audit,
  and its two readers — scope's pre-emption threshold and `kpi-incident-recurrence` — see a
  backlog that looks less repetitive than it is.
  Filed 2026-08-06 by close from the gap inbox.

- **section-prose-outlives-its-entries** [design-pending] — Clear-Done removes the entries and
  leaves the prose that framed them.
  A scope-authored preamble under an active section describes the entries beneath it. Close's
  Clear-Done step moves those entries to `## Done` and touches nothing else — precedent
  `e3d893e`, `f8433ac` — so the preamble survives as prose naming a causal order that points at
  nothing. No gate reds: prose is not an entry, and no gate reads the relation between them.
  **Instance, fixed at this close.** `## New Features` kept `background-producer-liveness`'s
  four-unit causal-order preamble after all four entries reached `## Done`. Lead-ruled 2026-08-06
  at the end of build that close removes it explicitly; the removal rides this close's Clear-Done
  commit, and the class is what survives.
  **Why `[design-pending]`:** the tractable rule is not obvious. "An active section with no
  entries has no body" is decidable and would have caught this instance, but it over-fires on a
  section preamble that is a standing authoring rule rather than a gloss on the current entries,
  and `TASK-QUEUE.md` carries both kinds. Separating them needs either a marker on
  iteration-scoped prose or a ruling that active sections may not carry standing prose at all —
  a queue-format contract change.
  **Cost while deferred:** every iteration whose scope writes a section preamble leaves one
  behind at close, and each stranded preamble tells the next scope a causal story about entries
  that are gone.
  Filed 2026-08-06 by close from the gap inbox; instance fixed inline in the same commit.

- **amendment-roster-stale-by-construction** [design-pending] — the enumeration that makes a
  sweep mechanical is stale the moment a sibling batch lands.
  An amendment that enumerates a rename or sweep roster up front is doing the very thing that
  makes such a delta mechanical rather than design-bearing. The roster is authored at spec time
  and is stale by construction as soon as any sibling batch writes prose on the same files, and
  nothing reds.
  **Attested this iteration.** The verb-rename roster was authored at spec; an earlier build
  batch then added ~77 lines to `delegation-kit/SPEC.md` and ~46 to
  `lifecycle-kit/templates/lead.md` without pre-applying the rename. The sweeping batch
  re-derived against the tree only because the lead flagged it, finding five sites the roster
  missed — one in `docs/delegation-kit/index.md`, a hand-authored page
  `scripts/gen-docs-mirror.sh` does not write, so no mirror regen would ever have corrected it.
  The enumeration also undercounted its own stated four citation sites.
  **Shape:** either a freshness assertion that a roster is re-derived at sweep time, or a stated
  rule that an enumerated roster is a floor and never the answer. The instance went green, which
  is exactly the enforcement-first stop signal to ask what check should have caught it.
  **Why `[design-pending]`:** a freshness assertion needs a re-derivation command the amendment
  can name, and a rename roster's derivation is a grep whose pattern is the amendment's own
  content — so the gate would assert a grep against a grep. The floor-not-answer rule is cheap
  and unenforceable, and choosing between an unenforceable rule and an expensive gate is the
  design.
  **Cost while deferred:** every multi-batch iteration carrying a sweep unit ships a roster its
  own later batches falsify, and the only detector is a lead who happens to flag it — which is
  what happened here and is not a mechanism.
  Filed 2026-08-06 by close from the gap inbox; the sweep instance itself landed green.

- **unregistered-gate-fixture-coverage** [design-pending] — the fixture-pair contract is
  asserted over the registry, not over the gates.
  `check-gate-fixture-coverage` walks `gates.list` members, so a kit gate that ships deliberately
  unregistered carries no fixture obligation at all: evidence-kit's `check-producer-liveness` (an
  entry-preflight gate, kept out of `gates.list` because this repo's battery is what it locks
  against) and canon-kit's `check-surface-duplication` can lose or never gain their `good/`+`bad/`
  pair with nothing red. `run-gate-tests.sh` executes whatever pair directories exist, so an
  absent pair reads as zero coverage rather than as a harness error — the silent direction.
  **Found at the build that created the first entry-only gate**
  (`validate-producer-liveness-unobservable`), which is what turned an always-true assertion into
  a false one.
  **Shape:** widen the assertion from the registry to every kit `checks/` member, in both
  declaration spellings, keeping the existing no-fixture opt-out. The false-positive surface is
  bounded precisely because that opt-out already exists — a gate that genuinely owes no pair
  declares it.
  **Why `[design-pending]`:** "every kit `checks/` member" is a claim about what a kit root is,
  and gate-sdk/SPEC.md's kit predicate is the authority; widening a gate to walk kit roots
  couples it to that predicate, and the coupling wants stating before it ships.
  **Cost while deferred:** the four gate-sdk contracts bind registered gates only, and the
  exception set grows by one each time a gate is deliberately unregistered — which this iteration
  established as a supported pattern rather than an oddity.
  Filed 2026-08-06 by close from the gap inbox; found at build, shape supplied by the filer.

- **link-wrapped-section-citation-liveness** [design-pending] — a section citation inside a
  markdown link is invisible to both citation gates.
  `check-spec-pointer`'s prose extractor guards on a `.md` *immediately* followed by the section
  sign, so it looks only where the two are adjacent. The reference-link form
  `[path.md](path.md) §Heading` puts a `)` between them: the guard never fires, the heading is
  never resolved, and this holds even though the file is in the governed manifest and
  `check-md-refs` has already resolved the link half. Live at `CONTRIBUTING.md:65-66`.
  **Third member of a family the queue already carries**, and the one that makes the family a
  predicate question rather than three fixes: `prose-filename-citation-liveness` is a path with
  no heading, `unqualified-section-citation-liveness` is a heading with no path, and this is a
  path *and* a heading that the adjacency guard splits. That second entry already says one
  predicate may cover both of the others; this is the evidence that the guard's window, not the
  citation form, is the variable, so a promoting scope should cost all three together.
  **Why `[design-pending]`:** widening the guard to tolerate an intervening `)` is a
  two-character change and almost certainly wrong alone. The honest fix decides what the
  extractor's window *is* — nearest preceding `.md` in the paragraph, or the link target — and
  that decision is shared with the two siblings.
  **Cost while deferred:** a renamed heading strands this citation with nothing red, inside a
  governed repo-meta file whose whole purpose is telling a contributor where to go.
  Found at spec 2026-08-06 while sizing the verb rename's blast radius, verified against the
  gate's own extraction logic rather than inferred; filed 2026-08-06 by close from the gap inbox.

- **queue-tier-label-correction-cost** [design-pending] — a wrong debt/feature label is cheapest
  to leave wrong.
  Two of `background-producer-liveness`'s four units — the lead dispatch-precondition entry and
  the dispatched-session waiting-rule entry — self-labelled as debt adding no governed name,
  while both delivered exactly what canon-kit/SPEC.md's litmus counts as a feature: a contract
  another component must honor (a dispatch precondition the lead must honor; a kit SPEC sanction).
  Both bodies also fired the misfiling tell — a multi-paragraph design ruling inlined where no
  gate sees it.
  **Scope read the labels correctly and still did not fix them.** It routed all four units to the
  authoring stage on the corrected reading, which is the behavior the label exists to drive, and
  left the labels standing: both entries sat at `check-queue-entry-budget`'s line cap, so editing
  a label first required finding a compensating trim. The correction was strictly more expensive
  than the miscarriage it fixed.
  **Why `[design-pending]`:** the label is not mechanically derivable — the litmus turns on
  whether a body delivers a contract, the same semantic judgment `check-queue-entry-budget`'s
  compression rule already declines to gate. The tractable question is the *friction*, not the
  label: whether a label-only edit should be exempt from the cap (a one-token diff cannot regrow
  an entry), whether the cap should be measured below the lead line, or whether nothing changes
  and the cost is correct.
  **Cost while deferred:** every mislabeled entry that reaches the cap stays mislabeled, and the
  label's two readers — the authoring-stage routing rule and the new-names litmus — read a value
  its author already knew was wrong.
  Filed 2026-08-06 by close from the gap inbox; both instances reached `## Done`, so the class is
  all that survives.

- **exit-echo-decoration-guard-vs-habit** [design-pending] — an agent chains otherwise
  allowlisted read-only commands into one probe, and the whole probe prompts.
  recurrence: exit-echo-decoration-guard-vs-habit 2026-08-06
  Stage sessions join independent read-only calls — `grep`, `find`, `cat`, `ls`, `echo`, git
  subcommands — into one multi-statement `Bash` call, which resolves to no single allowlist
  glob and prompts every time; the trailing `; echo EXIT:$?` decoration is one shape of the
  same root. The `$?` is a shell *expansion*, and bash-guard's own banner states no allowlist
  entry can suppress an expansion, so guard-kit/SPEC.md §The triage criterion **cannot**
  resolve this to (a) allowlist. It resolves to either a **guard steer** that recognises the
  benign shape and rewrites toward the per-statement form, or a **habit change** for stage
  sessions. Both are guard-kit design decisions.
  **Evicted to the icebox 2026-08-05, recurred 2026-08-06 at dominant scale**, which is the
  icebox tier's own re-entry condition. `scan-prompts.sh` ranked 20 `grep`, 18 `find`, 9
  `cat`, 8 `ls`, 4 `echo` and a long tail of single leading tokens as prompting — roughly 78
  of this iteration's 106 prompting calls. The majority were verified by inspection to be
  several statements joined by a **literal newline**, never by `;` or `&&`, which is the only
  separator `guard_split_compound` segments on: every statement would match a glob alone, and
  the blob matches nothing.
  **Second contributor, newly identified:** correctly piped diagnostics fail too —
  `find … | xargs …` and `find … | sort` segment fine but `xargs` and `sort` are absent from
  `GUARD_KIT_RO_BINS`' default roster, and nothing grants a bare `find` or `xargs`. So the
  fix has at least two independent halves, which is new information the icebox line could not
  carry.
  **Why `[design-pending]`:** a steer has to recognise the benign multi-statement shape
  without widening the expansion-suppression hole the banner warns about, and the choice
  between a narrowly-shaped steer and a documented habit change is the design.
  **Cost while deferred:** the dominant prompting pattern in two consecutive iterations, paid
  once per call on exactly the read-heavy stages the delegation doctrine wants cheap. No gate
  reds and nothing degrades, which is why it iceboxed the first time — but the carry is no
  longer low.
  Filed 2026-07-25 by close, operator-reported; iceboxed 2026-08-05; evicted back to Deferred
  2026-08-06 by close on attested recurrence (queue-kit/SPEC.md §The icebox tier).

- **scan-prompts-truncation-quote-desync** [design-pending] — the friction log's own truncation
  can make `scan-prompts` misreport an already-allowlisted command as prompting.
  `guard_log_fallthrough`'s 500-character log truncation can land inside an unclosed
  double-quoted argument. `scan-prompts.sh`'s skeleton pass protects only *balanced* quoted
  spans, so a literal `;` later in the now-unprotected prose reads to
  `guard_split_compound` as a statement break, splitting one allowlisted command into
  segments that match nothing.
  **Verified rather than asserted.** `lifecycle-kit/bin/file-gap.sh`'s nine fall-through lines
  were isolated and re-scanned one at a time: the three reported as prompting are exactly the
  three whose truncated prose carries a bare `;` inside an unclosed quote, and
  `guard_allow_match` independently confirms the committed `bash lifecycle-kit/bin/file-gap.sh`
  glob covers the un-mangled argument. The calls almost certainly never prompted — only the
  log's analysis of itself is wrong.
  **Deliverable:** one of — make the skeleton pass tolerant of an unbalanced trailing quote
  (protect from the last unmatched opener through end of line); size the per-entry cut so a
  truncated quote is rare for prose-bearing callers; or stop truncating an advisory scratch
  file whose disk cost is trivial. Plus a fixture reproducing the shape, since
  `scan-prompts`' KPI claim (guard-kit/SPEC.md §scan-prompts) does not hold for it today.
  **Why `[design-pending]`:** the three candidates trade differently — the first needs a
  precise rule stated and pinned, the second only shrinks the window without closing it, and
  the third trades log bulk for correctness on a knob drift-kit's overhead reporting also reads.
  **Cost while deferred:** low, bounded, and false-positive-only — a genuinely uncovered
  command still prompts correctly. It inflates the close-triage worklist and drift-kit's
  prompting KPI on any iteration filing several long, punctuated gap descriptions, so the
  distortion lands hardest on the iterations that capture the most.
  Filed 2026-08-06 by close, from this iteration's prompt-friction triage.

- **ruling-record-condition-staleness-probe** [design-pending] — a ruling conditioned on an
  event nobody retires once the event fires.
  `TRAJECTORY.md` carries rulings whose text is conditioned on a future event — "at
  `<iteration>`'s close" is the surviving example, and "until the first binaries tag" was the
  other until the prune deleted every ruling carrying it — and nothing retires one when its
  condition is met. Today obsolescence rides the unit that discharges the ruling, whose own
  entry names the rows it removes on landing; that holds only where an author remembered to
  write the removal in, and nothing catches the ruling whose condition a unit met silently.
  **The enforceable half is a probe, never a prune — and the ground for that has moved.** It
  rested on retirement being operator-class; the operator's 2026-08-08 pruning directive
  (`trajectory-prune-on-completion`) authorizes the prune, so what keeps this entry to a probe
  is its own deliverable rather than an authority limit; detect and act stay complementary.
  The deliverable is a *staleness probe* over condition-bearing rulings that escalates, plus
  whatever declaration makes a condition machine-readable — a ruling stating its own discharge
  event is the design question, since a prose condition carries no syntactic tell. That
  declaration is this entry's alone: the pruning unit ships prose convention, not form.
  **Both attested conditions have now fired, which settles the recurrence and not the
  deliverable.** At the 2026-08-06 close one was unfired and one already missed — a companion
  ruling naming a specific iteration's close as the tag point, which passed without cutting
  the tag and no surface said so. The unfired one fired 2026-08-07 (v0.22.0), discharging
  every row conditioned on it at once. Two state changes, no probe watching either.
  **The class reaches the record's corrections, not only its conditions** — the surface
  written to record the missed condition went stale itself in under a day (relocated here
  from `TRAJECTORY.md` by `trajectory-prune-on-completion`, which deletes it there).
  **Widened 2026-08-09 by the first post-prune close: the inbound half, and it is the mirror
  of what `SPEC-ruling-record-prune` D9 refused.** D9 weighed slug-liveness *over* the record
  — the record citing dead slugs — and correctly ruled it a no-op. The other direction was
  never considered: **another governed surface citing a ruling the record no longer carries.**
  The pointer names the surviving *section*, so it resolves and the battery reports clean over
  it. Now that pruning is licensed, every prune manufactures this exposure. Three instances
  were verified individually at that close, each having survived a full green battery: two in
  `gate-sdk/SPEC.md` (§The port-candidate criteria criterion 5, and its "What stands between
  that port and an adopter" paragraph) and one in this entry's own body above; all fixed
  there. **No denominator is stated on purpose** — two sweeps of the citation corpus returned
  counts that do not reconcile under differing exclusion sets, which is precisely what
  `spec-measured-count-gate` names.
  **The design input worth more than the instances, because a probe would miss it.** A fourth
  instance — `gate-timing-baseline-comparability`, corrected at the same close — cites the
  deleted ruling **by its distinctive phrasing, naming no file at all**, so no probe scoped to
  file-name citations reaches it; and it was the highest-consequence of the four, a deferral
  argument resting on the expired premise rather than stale narration. So the condition
  vocabulary must be readable from the *citing* side, not only the record's.
  **Why `[design-pending]`:** the condition vocabulary is the whole design, and the
  escalation-only boundary rules out the cheap fix.
  **Cost while deferred:** a ruling read as live after its condition passed steers the very
  sessions that consult the record for what is settled, and the record's authority makes that
  steer expensive to unwind. The widening adds the sharper cost: the citing surfaces are
  *published* — a kit SPEC and its docs mirror — so the blast radius reaches adopters.
  Filed 2026-08-07 by close, draining the gap inbox.

- **stage-economics-log-key-session-collision** [design-pending] — two sessions in one
  (iteration, stage) overwrite each other in the trend log.
  `drift-kit/bin/stage-economics.sh`'s `log_line` dedups on the `<iteration> <stage> <model>`
  triple, so a batch-split stage — several sessions stamped into the same pair — appends twice
  and the second silently replaces the first, under-reporting that stage by a whole session.
  The session id is written as a same-line field, never into the key.
  **Found by exercising the meter against the real sessions directory**, not by reading it:
  one iteration's build stage had run as two sessions and only one row survived. The fan-out
  pass folds its own rows for exactly this reason; the stage rows still race.
  **Held out of the ruled cut deliberately** by the iteration that fixed the fan-out half, so
  this is the named remainder rather than a regression.
  **Re-measured 2026-08-08 at close, on a second independent instance.** That iteration's build
  ran four batches tiered per batch across two models, two sessions each; the log retained one
  row per model, so each retained row carries a single batch's draw as though it were the
  stage's — an under-report of roughly a third on both models, with nothing in the file marking
  that anything was dropped. Two refinements that instance adds to the diagnosis above: the
  per-session report on **stdout is correct**, so only the persisted rollup is lossy and it is
  the persisted rollup that trends; and **per-batch model tiering is precisely the shape that
  breaks the dedupe**, which was written for one session per stage per model, where re-running
  the collector must not double-count.
  **Why `[design-pending]`:** the honest closes differ in kind, and there are three. Folding
  stage rows the way the fan-out pass folds keeps the key and loses per-session detail; widening
  the key with the session id is a trend-log grammar change every existing reader inherits; and
  making the row an explicit aggregate the writer recomputes from every matching transcript
  keeps the grain but moves the work into the writer. Naive summing is ruled out on its own —
  the collector is re-runnable, so a sum double-counts on every re-run.
  **Cost while deferred:** every per-stage figure for a split stage under-reports, and the
  under-report is invisible in the log — a cross-stage comparison drawn from a split iteration
  is not signal, and nothing in the log says so. The Split posture makes splitting routine, so
  the defect's reach grows with the posture's adoption. Sharper under model tiering: the rollup
  is the surface a standing tiering watch reads, so a model looks cheaper exactly when its stage
  ran more batches.
  Filed 2026-08-07 by close, draining the gap inbox; found at build. **Merged 2026-08-08 at
  scope by lead ruling** with `stage-economics-log-multi-session-undercount` (filed 2026-08-08
  by close), which was the same defect re-found: identical `log_line` dedupe on the
  iteration/stage/model triple, verified against the source. That entry's premise that the work
  was unfiled is false; its measurement and its third design candidate are folded in above and
  the duplicate slug is dropped.

- **guard-advise-jq-dependency** [design-pending] — the advisory primitive goes silent in
  exactly the degradation it exists to report.
  `guard-kit/lib/guard.sh`'s `guard_advise` pipes into `jq` with no fallback, so with `jq` off
  PATH it writes an empty stdout and the advisory is lost, leaving only a command-not-found on
  stderr. Measured, not inferred. Every consumer routing a degraded arm to `guard_advise`
  therefore says nothing at the moment it most needs to.
  **The workaround is already local.** This repo's dispatch guard hand-writes its envelope to
  avoid the dependency, and the constraint is stated in guard-kit's SPEC, so no consumer here
  is silent today. What is unanswered is whether the *primitive* should carry an escaper-free
  fallback so no consumer has to know this.
  **Why `[design-pending]`:** the call is whether a degraded advisory beats a lost one, and
  whether a hand-rolled escaper in the lib is a cure worth its own risk.
  **Cost while deferred:** small and bounded here, since the one consumer that mattered worked
  around it; the cost lands on a downstream consumer that reads the primitive as safe and
  inherits the silence without the local knowledge that avoids it.
  Filed 2026-08-07 by close, draining the gap inbox; found at build while wiring the
  dispatch guard's degraded arm.

- **rejected-compound-commit-relabel** [design-pending] — a rejected stage-and-commit leaves
  the index staged and the bare retry mislabels the work.
  A hook rejection of a compound stage-and-commit invocation unstages nothing, so the natural
  retry — a bare commit — sweeps the staged deliverable under whatever message the retry
  carries. Observed 2026-08-07: a build batch had its compound call rejected on an expansion
  rule, retried bare, and shipped fifteen files of feature work under a one-line stamp
  message; the real stamp then landed as a second commit.
  **Nothing red fires, because nothing is wrong with the content.** The work was correct and
  gate-verified. The casualty is provenance, and it is unrecoverable once pushed: the
  never-rewrite policy and a shared index both forbid amending, so the misdescription is
  permanent in the history later sessions read for what happened.
  **Note the interaction that makes this likely rather than rare.** The shared-index rule
  advises staging and committing in one motion precisely to keep a foreign commit from landing
  between the two — which is the very shape most likely to be rejected by a guard.
  **Why `[design-pending]`:** the candidates sit on different surfaces. A commit-msg gate
  asserting that a stamp-shaped message touches only the stamp path catches this case and
  little else; guidance to reset the index after a rejected compound commit is cheaper and
  unenforced. Which is right depends on how wide the misdescription class really is.
  **Cost while deferred:** the history that later sessions and the economics read take for
  ground truth can misattribute a whole deliverable, and the error is silent, permanent, and
  likeliest on the commits a guard just fired on.
  Filed 2026-08-07 by close, draining the gap inbox; observed against this iteration's
  own build commits.

- **survey-record-supersede-invisible** [design-pending] — the entry trigger prints a
  superseded block beside its superseder with no visible difference.
  `.workflow/survey-record.md`'s stage-entry read trigger advertises every block for a
  question, so a superseded block and the block that superseded it print as two identical
  headings. A reader cannot tell which is live from the trigger output, so the protocol's own
  instruction — run the witness before buying this survey again — has to be run twice to find
  out, which is the re-derivation the surface exists to remove.
  **Observed on the surface's first real use, and again at this close.** A build session filed
  a survey, its own commits moved the corpus, it filed a superseding block, and the next stage
  entries each printed the pair.
  **Why `[design-pending]`:** printing only the newest block per question is cheaper but hides
  the history the record is append-only to keep; marking the superseded heading keeps both and
  needs a supersede relation the record does not declare today. Either way it stays
  append-only.
  **Cost while deferred:** small per entry and growing with every supersede in an iteration,
  and it lands on the reader rather than the writer — so the writer never sees it and the
  surface's own value claim decays unobserved.
  Filed 2026-08-07 by close, draining the gap inbox; found at build, re-observed at close.

- **probe-before-assertion-doctrine** [design-pending] — the iteration's recurring failure was
  asserting where a cheap probe was available; does that earn a durable rule?
  Every session this iteration that *probed* found something reasoning had missed, and every
  falsified claim was falsifiable by one command. Scope asserted twice and was falsified
  twice; spec corrected scope and then made a softer version of the same error, which align
  caught; build's meter batch found a row-erasure race only by running against the live
  sessions directory; the guard batch found that a whitespace `IFS` made one of its own rules
  never fire; the survey batch found a boundary-truncate bug by running a real entry. Close
  added two more: a gap bullet asserted a disambiguation was missing that had landed two days
  before it was filed, and a stamped release disposition asserted a tag that was never cut.
  **The question is the form, not the pattern.** The pattern is attested past argument. What
  is unsettled is whether it is already covered — `Oracle-first` covers the gate case,
  `gap-bullet-premise-verification` covers the gap-inbox case, and rule 14's inspectable-run
  discipline covers the spawned-component case. What none covers is the *design-time* claim
  about the tree or the harness that a grep would settle, made at scope or spec where no
  oracle is running.
  **Why `[design-pending]`:** a doctrine rule owes an *Enforced by* line and this one has no
  gate behind it, so it would ship as convention — which the doctrine does carry, but the
  call on whether a fourth near-neighbour earns its own always-loaded line is a
  governed-surface widening, not a close's to self-serve. The cheaper alternative worth
  beating: widen one of the three existing rules rather than mint a fourth.
  **Cost while deferred:** the cost is paid where it was measured — a false premise entering
  at scope is corrected at spec or align if it is lucky, and lands in the queue as established
  fact if it is not; the queue's own convention of dating premise corrections into bodies is
  the standing evidence that it is often not.
  Filed 2026-08-07 by close, as the iteration's candidate lesson; the observation came from
  the lead, the evidence and the framing from this drain.

- **survey-record-extension-tier-hybrid** [design-pending] — the record is machine-parsed like
  a `.txt` and read like an `.md`, and the convention does not resolve the hybrid.
  gate-sdk/SPEC.md §The workflow directory splits the directory by extension: `.txt` is a
  record file whose grammar a gate parses field-wise, `.md` is a prose surface machine-read
  only for emptiness or a bullet count. `.workflow/survey-record.md` does both.
  `check-survey-record` parses every block field-wise — exact key order, no stray line,
  non-empty `corpus` and `oracle`, and a `rev` that must be a 40-hex sha naming a real commit —
  which is materially deeper than the `.md` tier's stated bound and is almost word-for-word the
  `.txt` tier's test. Its content is nonetheless genuinely prose: a natural-language question
  meant to be searched, closing on a free-form `finding` a later session judges before citing.
  **Not a mis-typing so much as a tier the split does not have.** The tracking axis is fine and
  the `# contract:` header is correct; what is unresolved is where a record-plus-judgment file
  belongs when the convention offers only record or prose.
  **Why `[design-pending]`:** the honest options are renaming the surface, widening the `.md`
  tier's machine-read bound to admit a validating gate, or naming a third tier — and the last
  one costs every consumer that has internalised the two.
  **Cost while deferred:** low and non-rotting; it is paid by the next author adding a workflow
  member, who reads the convention, finds the shipped counter-example, and re-derives which
  half to follow.
  Filed 2026-08-07 by close, from the workflow-surface-extension audit its roster made due.

- **consumer-smoke-accounting-spelling-unpinned** [design-pending] — the accounting reads both
  gate spellings, and nothing holds it there.
  `run-consumer-smoke.sh`'s registration accounting now unions `check-*.sh` with `check-*.gate`
  (gate-sdk/SPEC.md §Consumer smoke — the registration accounting), but that widening is stated
  in prose and pinned by no fixture. The defect it prevents is the silent one: a ported gate
  dropping out of the accounting's universe leaves no probe, no declaration and no finding, so
  a regression reads as a clean run rather than a red.
  **Why `[design-pending]`:** the harness is a `bin/` tool rather than a registry member, which
  is exactly what `check-gate-assertions`' assertion C cannot reach, so pinning it means
  standing up a second whole-consumer scratch tree inside a fixture — the cost is the design
  question, not the assertion.
  **Cost while deferred:** the one accounting that proves a ported gate is still counted is
  itself uncounted, and its failure mode is invisible by construction.
  Trigger: the next unit touching that accounting, or a second gate cohort porting.
  Filed 2026-08-07 by close, promoting the lead's gap-inbox filing at `d790c10`.

- **release-asset-claim-class-owner** [design-pending] — a reader-facing claim about what a
  Release ships is maintained by hand, with no oracle.
  `docs/install.md`'s cargo bullet asserts what a tagged release publishes. The descriptors unit
  corrected the sentence and **ruled the gap accepted-ungated** rather than minting a gate:
  the class has exactly one instance, and a registry for one member is the move the payload-claim
  design refuses on its own axis. This entry carries that ruling's named trigger so it survives
  the gap inbox's truncation.
  **Why `[design-pending]`:** the open question is whether a release-asset claim is its own class
  or a second axis of `check-payload-claim`'s vocabulary — one asks what a Release *ships*, the
  other what a consumer *receives*, and collapsing them wrongly is harder to unwind than leaving
  them apart.
  **Cost while deferred:** one published claim about Release contents sits on the adoption path
  held only by hand — the exact shape that produced `payload-disclosure-claim-owner` one claim
  class over.
  Trigger: a **second** release-asset claim appearing on any governed surface, at which point
  both axes collapse into `check-payload-claim` rather than into a third gate.
  Filed 2026-08-07 by close, promoting spec's gap-inbox filing at `06e379c`.

- **build-stage-tightened-gates-write-pair** [design-pending] — the stage template names one of
  the two writes a tightened gate owes.
  `lifecycle-kit/templates/stages/build.md` instructs a build session to append the bare gate
  name to `.workflow/tightened-gates.txt`. That is half the obligation: the dated release note
  owes a matching bullet, and `check-tightened-gates-note-parity` holds the two against each
  other **in the same commit**. A session following the template literally stages an incomplete
  pair and reds its own commit.
  **Two independent build batches hit this in one iteration**, which is what makes it a template
  defect rather than a session lapse — the second batch had no way to learn from the first.
  **Why `[design-pending]`:** the deliverable is nameable (say both writes, cite the parity
  gate), so the open part is only placement — whether the note bullet belongs in the build
  template, in the release runbook the template would then have to load, or in the gate's own
  red output, which is the surface that already knows both halves. A scope may promote this
  straight to a task.
  **Cost while deferred:** every build stage that tightens a gate reds once and re-derives the
  second write from the gate's failure, which is the knowledge-friction loop paying for a
  sentence.
  Filed 2026-08-07 by close, carried from build batch 4.

- **cargo-grant-committed-vs-overlay** [design-pending] — a commit-time requirement is granted
  only by an untracked file.
  The dogfood ruling made `cargo build --release --manifest-path native/Cargo.toml` a
  **commit-time** requirement in this tree (CLAUDE.md §Housekeeping), so every session now runs
  it — yet `Bash(cargo build *)` and `Bash(cargo test *)` live in the gitignored
  `.claude/settings.local.json` and not in the committed allowlist. Measured this iteration:
  six calls granted only by the overlay. A fresh clone that installs the hooks therefore prompts
  on the command the hook effectively requires, and the grant that makes the tree workable is
  the one thing a clone does not receive.
  **Why `[design-pending]`:** widening the committed allowlist is the consumer's call, not a
  session's (guard-kit/SPEC.md §The triage criterion), and the honest options differ in breadth —
  a glob over all `cargo build`, an exact grant of the one manifest-pinned command, or routing
  the build through a tracked `bin/` script that is granted by path like every other kit tool.
  **Cost while deferred:** the friction lands hardest on a first-time contributor, who meets it
  before any of the tooling that would explain it.
  Filed 2026-08-07 by close, from its own prompt-friction triage.

- **align-checklist-fanout-calibration** [design-pending] — align converges at zero divergence
  while build finds the defects it should have caught.
  **Read this entry as calibration, not as the revert signal.** The standing align tiering ruling
  names one live revert trigger: *a missed spec defect surfacing as a build round-trip*. It did
  **not** fire this iteration — every defect was fixed in-envelope by the build session that
  found it, none round-tripped, and nothing here is grounds to revert the tier. Conflating the
  two is the specific misreading this entry exists to prevent.
  **What did happen, `native-cohort-activation`.** Align passed all four amendments at zero
  divergence. The four build batches then found **ten** real defects. Six were fan-out and roster
  misses of one shape: a section a change touched that the amendment's "Existing sections updated"
  roster failed to name, or a reader-facing surface still asserting a predicate the change
  retired — twice in the release note itself, which is the front door.
  **Most were catchable two ways**, and neither is a judgment call: grep the unit slug tree-wide,
  and read each delta against its own **DoD** rather than against its "what this keeps" list. The
  second is the sharper one — a "keeps" list is the author's claim about what a change does not
  touch, so align reading it as a boundary inherits the exact blind spot that produced the miss.
  **Why `[design-pending]`:** a mechanical fan-out check over an amendment's roster is a real
  candidate and is already filed as `amendment-update-target-coverage`, so the open question is
  the split — how much of this is a checklist edit to the align template, how much is a gate, and
  whether a "keeps" list should keep its standing as an input at all.
  **A second, separate failure class, recorded beside this one rather than folded into it.** The
  validate session — also Sonnet — ended its turn on work still running in order to wait for it,
  which its standing dispatch policy forbids outright. Corrected in-flight; it recovered fully,
  re-verifying the first run's actual state rather than assuming it. That is a **protocol** miss,
  not a judgment miss, and it belongs to the dispatch-policy surface rather than to align's
  checklist. Kept adjacent because both are tier-calibration evidence and the two get read
  together; kept distinct because a fix for either does nothing for the other.
  **Cost while deferred:** align keeps returning a clean verdict that build then falsifies, so
  the stage's signal value decays toward zero while its cost does not — and a zero-divergence
  pass is read as evidence the amendments were right.
  Filed 2026-08-07 by close, from the lead's per-batch tiering watch.

- **survey-edge-aggregation-residue** [design-pending] — the audit that asks whether a scope
  survey aggregated inbound edges has nothing to read.
  `templates/stages/scope.md` requires it plainly — aggregate a candidate's inbound edges
  before ranking it, because the promotion dividend lives in the total and in no single entry.
  The `survey-engagement` audit class then asks each close whether that pass ran, and declares
  its only residue to be "the survey's own reasoning".
  **Attested this close: that residue does not carry the claim.** Scope's reasoning landed in a
  commit message recording *conclusions* — three premise falsifications, and one entry's
  self-declared strongest ground engaged and answered rather than merely refuted, which is the
  counter-evidence half done properly. Nothing in any artifact says whether an edge sum was
  taken for any candidate, so the audit can neither confirm nor fault it. An audit that cannot
  return either verdict is not un-gateable, it is unanswerable.
  **One named alternative happened on its own, 2026-08-08.** The next iteration's scope landed
  its ranking survey in the survey record with `bin/queue-edges.sh` named as an oracle and the
  inbound totals written into the finding itself — per-candidate sums for the recurrence cluster
  and for the trajectory-sequence candidates. "Folding the sum into the existing survey record"
  is therefore a shipped instance rather than a hypothetical, and this close's `survey-engagement`
  audit was answerable and passed on it. What stays open is that nothing *required* it: the audit
  is unanswerable again the moment a survey omits the sums.
  **That moment arrived at the very next iteration, 2026-08-09 — the entry predicted its own
  recurrence and the prediction held one iteration later.** `install-profile-seam`'s scope ran
  three surveys; the ranking one ("Which Deferred entries cluster onto a shared surface and
  mechanism") censused 129 Deferred entries and produced the seven-cluster ranking the cut was
  argued from, naming `bin/queue-index.sh` as its oracle and **not** `queue-edges.sh`. Its
  counter-evidence work is entirely premise-falsification (it lists five stale premises); no
  inbound sum appears for any candidate. So `survey-engagement` was answerable in one
  iteration and unanswerable in the next, on the same roster line, with nothing changed but
  the survey — which is the strongest available argument that the requirement is real and the
  residue is not. **No `recurrence:` date is stamped, and the decline is deliberate:** this
  was observed by a rostered close audit rather than through the gap-inbox capture channel,
  and whether such an observation may stamp directly is the open question
  `recurrence-drain-input-widening` holds. Attested here as prose, consistently with the two
  other out-of-channel recurrences that close recorded the same way.
  **Why `[design-pending]`:** the cheap fix — have the survey cite its edge sums — risks
  becoming ceremony, a stage writing down that it did the thing rather than doing it. The
  honest alternatives are a survey artifact the sum lands in, folding the sum into the existing
  survey record, or accepting that this class is not auditable and retiring the roster line
  rather than restamping it each close.
  **Cost while deferred:** every close restamps an audit it did not actually perform, which is
  worse than a skipped audit — the roster reads as coverage.
  Filed 2026-08-07 by close, performing the `survey-engagement` audit its roster made due.

- **release-runbook-identity-diagnosis** [design-pending] — the runbook reads a 404 as a
  permission fault on a machine where it is an identity fault.
  `RELEASING.md` step 4 treats a 404 on a write to the repo as a permission signature and
  prescribes fixing the permission. That inference holds only where the host CLI is
  authenticated as exactly one account. Where it holds two and the active one lacks push, the
  same 404 appears with the permission model already correct — and the prescribed resolution
  points at granting push to the account the private brief rules must **not** hold the
  namespace, so following the runbook literally walks a session across the identity boundary
  the brief exists to protect.
  **Reachability, not merely wording.** Step 6 is unreachable from a session whose active
  account is the non-owning one. A close that defers its release never meets this; a close that
  cuts one meets it mid-cut, with a note committed and a tag pending.
  **Correction 2026-08-08 at scope, oracle-settled — the class may be real, but both of the
  concrete grounds this entry was filed on are false at HEAD.** (1) A prior revision offered the
  unreachability above as "the most likely reason the last tag's Release body was never
  written". The premise fails: `gh release view v0.22.0` returns a written body — a pointer to
  the release post — and all four assets attached, the tarball, its checksum, and the one-triple
  gate binary with its checksum. Step 6 was reached on the last cut. (2) The entry's scenario
  needs the *active* host-CLI account to be the non-owning one; `gh api user` resolves it to the
  namespace-owning identity, so the plural-identity 404 is **latent rather than armed**, and the
  runbook's inference is untested here rather than demonstrated wrong. Note also that
  `RELEASING.md:148-166` already carries the credential precondition, the permission-not-scope
  test (`gh api repos/<owner>/<repo> --jq .permissions`), the 404-signature clause, and the
  explicit rule "Resolve it by fixing the permission, **never by switching identity**" — so the
  runbook is substantially further along than this body describes. The residual worth keeping is
  narrow and unchanged in kind: where the active account is the non-owning one, "fix the
  permission" resolves to granting push to the account the private brief rules must not hold the
  namespace. **Re-ground this entry before `spec` authors against it** — its current evidence
  cannot carry a design.
  **Why `[design-pending]`:** the fix is not a sentence. A runbook diagnosing across a plural
  identity has to establish which account is active and whether that account owns the namespace
  *before* it reads any status code, and where that check belongs — a preflight step, step 4's
  own prose, or a probe the procedure runs — is open. Part of why the runbook never said it is
  that the desired account state is private-ops content and cannot land on a tracked surface,
  so the runbook can name the discriminator but not its expected value.
  **Cost while deferred:** the outward-facing half of the release path is blocked behind a
  diagnosis that is wrong here and reads as authoritative, and the fix it prescribes is the one
  action the identity boundary forbids.
  Filed 2026-08-08 by close, draining the gap inbox; found at scope. Sibling on the same step:
  `release-drain-ordering-contradiction` covers step 4's drain/tag ordering, this its diagnosis.

- **gate-binary-target-roster-widening** [design-pending] [roadmap: next/reliability] — the
  binary ships one triple, and no queue unit carries widening it.
  roadmap-summary: A prebuilt gate binary for every platform the project says it supports.
  `native/targets.list` declares exactly one target triple, so the last release published one
  binary and every adopter off that triple takes the omit-and-declare outcome. That is a
  supported result rather than a break — but it means the trajectory objective naming every
  major operating system has no filed unit behind it on the artifact axis.
  **Distinct from the two entries a reader reaches for first, which is why it is separate.**
  `powershell-installer-surface` covers the bash bootstrap reaching a second interpreter, and the
  shell support-matrix CI legs cover the test matrix; neither is about which binaries the publish
  step produces. The design work is also half done and unowned — `installer/README.md` already
  records this design's named re-entry and its two candidate shapes, so what is missing is the
  unit, not the thinking.
  **Why `[design-pending]`:** the two candidate shapes differ in what they cost the publish
  workflow and in whether cross-compilation or a matrix of runners carries it, and choosing
  between them is the unit's substance rather than a detail inside it.
  **Cost while deferred:** a published objective has no unit behind it on this axis, so the gap
  is invisible to the roadmap projection and to anyone reading that projection as a commitment.
  Filed 2026-08-08 by close, draining the gap inbox; found at scope. **Tagged
  `next/reliability` 2026-08-08 by operator ruling** at the `adopter-floor-integrity` scope,
  answering the call this entry deliberately left open: a published objective naming every
  major operating system with no filed unit behind it on the artifact axis is exactly the gap
  the roadmap projection exists to prevent, so the public commitment is taken deliberately.


- **amendment-refusal-acceptance-parity** [design-pending] — an amendment's refusal rationale can
  claim an acceptance criterion asserts something that criterion does not say.
  The `--dry-run` amendment refused a gate on the stated ground that the behavioral property was
  asserted directly instead, in D8, as *each mutating verb's* `--dry-run` leaving the tree object
  unchanged. D8's own ordered specification listed one verb, not each. Two sections of one
  governing document said different things about what that document accepts.
  **The contradiction, not the coverage, is what earns the entry.** A build session reading either
  section in isolation would have been correct and would have shipped a different acceptor — the
  wide reading builds three assertions, the narrow one builds one. Build read D8's explicit list
  and built to it, which is the conservative call and the right one, since widening asserted
  behavior is envelope-shaped. But which acceptor shipped turned on which section the builder
  happened to open.
  **It passed spec and align.** Neither stage caught a self-falsifying document, which is what
  makes this a methodology gap rather than one amendment's typo: the acceptance section is the
  contract, the refusal section argues *from* it, and nothing compares the two.
  **Gap generalization — the class that should have caught it.** The nearest sibling is
  `intra-file-pendency-contradiction-scan`, which scans one file for a slug claimed landed in one
  section and pending in another. Same family — a governed file falsifying itself, decidable with
  no tree comparison — on a different axis: that one is about tense, this about the scope of an
  acceptance claim. Cited rather than folded, because collapsing them would hand one gate two
  unrelated vocabularies.
  **Why `[design-pending]`:** the decidable predicate is not obvious. "A refusal section citing an
  acceptance item must quote it" is checkable but is ceremony; comparing a paraphrase against the
  criterion it paraphrases is a judgment no scanner makes. The honest middle — a refusal may cite
  an acceptance item by identifier and may not restate its content — is a change to the amendment
  format, which is doctrine rather than a gate.
  **Cost while deferred:** any amendment can argue a refusal from a claim about its own acceptance
  that no stage verifies, and the failure is silent — both readings pass every gate, and the
  divergence surfaces only as a coverage gap found after the amendment is deleted.
  **Closing this takes both halves, and the ruling folds the fix in here rather than into a unit
  of its own.** Lead-ruled 2026-08-08: a separate entry for the one-assertion fix would let
  the cheap half close on its own and quietly leave the expensive one — that the document cleared
  both spec and align carrying the contradiction — as nobody's. Enforcement-first, applied to a
  finding rather than a gate: the fix and what explains it land together. Closure therefore
  requires **both** (1) the `init --dry-run` assertion in the consumer smoke — one arm, because
  `update` execs `init`, so it covers both mutating paths the reversal arm does not — and (2)
  removal of the stated bound now standing in `installer/README.md` §The verbs and §The consumer
  smoke. Taking (1) without (2) leaves a published caveat naming a coverage limit that no longer
  holds.
  **The tree is honest today, only narrower than the refusal argued.** The capability-liveness
  sweep at this close found zero governed surfaces still carrying the wide claim, so nothing
  published overclaims; what is missing is the assertion, not a correction.
  Filed 2026-08-08 by close, draining the gap inbox; found at build. The coverage half was
  escalated as an envelope call and folded in here on the lead's ruling the same day.

- **spec-pointer-self-section-citation** [design-pending] — a bare `(§Heading)` self-citation is
  outside every citation gate's reach.
  `check-spec-pointer`'s prose pass fires only on a **tracked path immediately followed by the
  section mark**, so a citation naming a section of the file it already sits in — the ordinary
  intra-SPEC form — is never resolved against that file's headings. A dead one survives
  indefinitely and rides the docs mirror to the public site.
  **Found live, not hypothesized.** `delegation-kit/SPEC.md` cited `§The staleness contract`,
  a heading no file in the tree carries; the mirrored page published it and a queue entry had
  copied it. Repointed at this close, so the instance is gone and the class is not.
  **Why it looks cheaper than it is.** The candidate set is large and mostly correct, so the
  design question is the resolution rule rather than the scan: a self-citation resolves against
  the containing file's own headings, which is a different lookup from the path-qualified form
  the gate already does, and the two must not be conflated in one pass. It sits on the same
  extractor the deferred `check-spec-pointer` citation entries name — sequence it with them
  rather than building a second scanner.
  **Cost while deferred:** a dead intra-file pointer reads as a live one, and the mirror
  publishes it, so the failure is silent on both the governed and the public tier.
  Filed 2026-08-08 by close, generalizing the gap it drained.

- **readonly-dispatch-isolation-unbuyable** [design-pending] — the shape that makes a read-only
  claim is the shape that poisons the read.
  `scripts/agent-dispatch-guard.sh` refuses a `DELEGATION_KIT_READONLY_TYPES` dispatch without
  `isolation: worktree`, which is correct — a read-only claim is made by isolation, not by
  sentence. But the harness cuts that worktree at a **stale base**, not at HEAD, so the audit
  the isolation was bought for reads the pre-change tree. The two rules compose into: a
  declared read-only audit is either refused or unreliable, with no third option.
  **Measured this iteration, on both halves.** A worktree was observed 21 commits behind,
  predating the whole iteration; `/align` ran three isolated audit sweeps whose clean verdict
  could not be certified, and all three build batches had to re-verify their own read sites to
  buy the doubt out. Separately an in-flight worktree dirtied the tree and aborted the consumer
  smoke at preflight (exit 2). The dirty-tree half is fixed — the path is gitignored — and the
  stale-base half has only a protocol mitigation
  (`delegation-kit/templates/agent-execution.md`): the child verifies `git rev-parse HEAD` and
  reads targets with `git show <rev>:<path>` out of the shared object store.
  **Why `[design-pending]`:** the mitigation is unenforced prose on the *child*, and nothing
  holds a parent to putting it in the dispatch. The honest candidates are a guard that requires
  the rev-discipline clause in a read-only dispatch prompt, a kit-side wrapper that injects it,
  or a ruling that read-only fan-out reads only through the object store — the first two gate a
  prompt's text, which is the reach question the design owes.
  **Cost while deferred:** an isolated audit's verdict is uncertifiable, so its consumer either
  re-verifies the same sites (paying for the audit twice) or trusts a read that may predate the
  work under review — and this close could not dispatch its two due audits at all, running them
  in-session instead.
  Filed 2026-08-08 by close, from the knowledge-friction capture and the gap it drained.

- **deferred-release-declaration-accumulation** [design-pending] — only one of the note's three
  sections survives a deferred release.
  `.workflow/tightened-gates.txt` is the accumulating declaration surface for **Tightened
  gates** and is drained only at the tag, so that section batches correctly across any number of
  deferrals (RELEASING.md §The procedure step 1). **Behavior changes** and **Renamed knobs** have
  no such surface: RELEASING.md step 2 says the outstanding criteria are carried into the next
  qualifying note, and nothing carries them. A deferral therefore drops them unless a later
  session reconstructs them from `git log`.
  **Already live, twice.** `installer-lifecycle-verbs` deferred on a minor earned by behavior
  changes alone — its tightened-gates surface was empty — so its declaration exists only in the
  basis clause of its disposition line. This close defers again with five behavior changes and
  one tightened gate; the composed bullets are carried below so the next qualifying note
  inherits them from a committed surface rather than from session memory.
  **The carried Behavior-changes declarations, composed 2026-08-08.** One paragraph per bullet;
  each opens on the changed surface's name, which the composing session bolds as the lead token
  the grammar docs/install.md §The upgrade contract owns.
  *`lifecycle-kit/bin/file-gap.sh`* — the slug matcher no longer writes a verdict onto the
  bullet it files. Every filing gets one bullet shape; on a match the tool raises a stderr
  advisory *asking* the filer to say in the prose whether the bullet re-files the named entry or
  merely cites it. The recurrence judgment moved to close's gap-inbox drain, which reads every
  bullet and stamps the `recurrence:` declaration in the same commit that truncates the inbox.
  Reconcile if your close skill or tooling parsed the retired prefix form.
  *`kpi-incident-recurrence`* (drift-kit) — the counted series is **not comparable across this
  release**. Every `recurrence:` date before it was matcher-derived; every date after is a
  session's judgment read off a bullet's prose. Compare within a segment, never across the
  break. For this project the break is the release carrying `gap-resolver-mention-overcount`;
  for a consumer it is that consumer's own adoption of it.
  *`installer/lib/doctor.sh`* — `doctor` now walks only the consumer-audience subset of the
  toolchain roster, so a machine with no Rust toolchain installs. Reconcile if you copied the
  roster out or relied on `doctor` failing on a contributor-side tool.
  *`context-kit/lib/toolfloor.sh`* — a roster element gained a fourth positional field,
  `<name>[:<min-version>[:<impl-token>[:<audience>]]]`. The only declarable audience is
  `contributor`; empty or omitted means every audience, so existing elements are unchanged.
  Reconcile if you carry a local roster read by a consumer-side reader of your own.
  *`.github/workflows/site-health.yml`* — a release-channel arm was added: it reads the
  published Release list on the schedule and files a `site-health` issue naming every Release
  whose prerelease flag disagrees with its own version line. Reconcile if you copied the
  workflow out.
  Renamed knobs: None.
  **Why `[design-pending]`:** a second accumulating file is the obvious fix and is probably
  wrong — three surfaces to drain at one tag, two of them free prose a gate cannot hold to the
  note the way `check-tightened-gates-note-parity` holds the first. The honest alternatives are
  one declaration surface carrying all three sections, or making the deferral line's basis the
  declared carrier with a gate that reads it.
  **Cost while deferred:** each deferral loses its non-gate declarations to git history, and the
  next qualifying note ships a Behavior-changes section that under-declares by however many
  iterations batched into it — which is exactly the section a consumer reconciles by reading.
  Filed 2026-08-08 by close, from the release-note obligation its own drain could not discharge.

- **always-loaded-baseline-restamp-unforced** [design-pending] — the brevity pass reacts to a
  delta nothing keeps per-iteration.
  `--update-baseline` is specified as a close-stage act precisely because the pass must react to
  *growth since the iteration started* rather than to the level (context-kit/SPEC.md §The
  always-loaded meter). Nothing forces it, and `kpi-always-loaded` is advisory, so a close that
  skips the re-stamp silently converts the next close's delta into a cumulative one — and the
  reading degrades further with every close that then trusts it.
  **Measured at this close:** the baseline last moved 2026-07-19 and had gone unrefreshed
  through roughly fifteen iterations, reporting `+41` where this iteration's own always-loaded
  growth was zero. The inherited 40 lines were read against the brevity criterion here and kept
  — each is a one-line-plus-pointer resident rule whose trigger is genuinely every session — so
  the debt is the broken signal, not the surface.
  **Why `[design-pending]`:** the obvious fix, a gate asserting the baseline commit is recent,
  gates a *cadence* rather than a property and would red on any close that legitimately had
  nothing to re-stamp. The honest candidates are folding the re-stamp into the same commit the
  brevity pass already writes, or making the meter report both deltas — against the baseline and
  against the iteration's first stamp — so a stale baseline is visible in the reading instead of
  invisible behind it.
  **Cost while deferred:** a cumulative delta reads as this iteration's growth, so the pass
  either re-reads surface it already cleared or dismisses a real addition as inherited.
  Filed 2026-08-08 by close, from running the meter during its own brevity pass.

- **drift-baseline-unnamed-iteration** [design-pending] — every since-iteration-start KPI
  baselines on an ancient commit for the whole of scope, silently.
  `drift-kit/bin/drift-report.sh`'s `iteration_start()` reads the queue header's iteration name
  and runs `git log -S"<iteration> scope " -- .workflow/WORKFLOW-STATE.txt | tail -1`. While the
  header carries the unnamed-iteration sentinel — which is every scope stage, from the boundary
  reset until the stage names the iteration — the pickaxe matches the *sentinel* rather than a
  name, and `tail -1` then returns the **oldest** sentinel-bearing commit in the whole history
  instead of this iteration's.
  **Probed at filing, not inferred.** The report printed `[iteration start 718ab4e]`, a commit
  dated 2026-07-10 — four weeks stale — and the pickaxe's own tail confirms it resolves to the
  first sentinel-bearing commit rather than the newest. The numbers it produced that day:
  `queue net delta +111 (126 filed, 15 drained)` and `queue carry weight +3759 lines`, where the
  true delta for the last full iteration, measured directly off boundary commits
  (`bf641b4` 3805 lines to `dbc9c1b` 4082), is roughly **+280**.
  **Why `[design-pending]`:** reporting `n/a` while the sentinel is live is honest and goes
  blank at exactly the stage that wanted the reading; resolving the baseline from the newest
  boundary-reset commit instead of from the header name keeps the reading but changes what
  "iteration start" means for every KPI that reads it. Which one is right is not the reporter's
  call to make in passing.
  **Cost while deferred:** it fails silent behind a plausible sha and worsens with age, and the
  stage it misleads is the one stage that reads the trend to pick units — a scope session sees a
  month of cumulative drift presented as this iteration's growth. Distinct from
  `always-loaded-baseline-restamp-unforced`, which is the always-loaded meter's re-stamp cadence
  rather than this resolver.
  Filed 2026-08-08 at scope on the lead's ruling, from running the report during its own survey.

- **survey-citation-outlives-its-record** [design-pending] — a permanent surface cites a
  boundary-truncated one, so the citation dies on schedule.
  The `prose-profile` entry above ends "Full finding and its two-command witness:
  `.workflow/survey-record.md`". That record is per-iteration scratch, boundary-truncated by
  design (lifecycle-kit/SPEC.md §The survey record), and `enter-stage.sh scope` truncated it at
  the very next boundary — so the citation resolved to nothing **one iteration** after it was
  written, and the finding survives only in the evicting commit.
  A live instance of `dead-queue-citation-report`'s class, distinguished by cause and worth
  separating for it: the target was not deleted by mistake or by a rename, it was deleted **on
  schedule by its own surface's contract**, so no liveness scan over authored links would have
  predicted it and no author error produced it.
  **Why `[design-pending]`:** three honest closes, and they trade against each other. The citing
  entry inlines what it needs at filing — cheapest, and it loses the two-command witness that
  makes a carried survey re-usable. The citation becomes commit-pinned (`git show <rev>:...`) —
  keeps the witness, and pins a reader to a rev the tree has moved past. Or the record grows a
  promotion path for a block a permanent surface cites — which re-opens the per-iteration
  lifetime the record's whole design turns on.
  **Cost while deferred:** the expensive judgment half of a survey is precisely what the record
  exists to carry across a boundary, and a queue entry is the surface most likely to want it
  *after* one. Every such citation is a silent one-iteration fuse.
  Filed 2026-08-08 at scope on the lead's ruling, found by following the citation and hitting
  the truncation this session's own boundary reset had just performed.

- **guard-glyph-match-context-blind** [design-pending] — the guard matches its trigger glyphs
  inside quoted and heredoc bodies, so writing *about* the guard is refused by it.
  `scripts/bash-guard.sh` tests the whole command string, so a `$(...)`, a brace expansion or the
  repo-root absolute path appearing inside a **heredoc body** — journal prose, a queue entry
  being appended, a commit message — trips the same refusal as the executable form. The refusal
  is correct about the glyph and wrong about the command.
  **Measured this session, which is the whole of why it is filed rather than felt:** 8 refusals
  in roughly 25 bash calls. Two of the 8 were this class (prose inside a heredoc, once for a
  brace glyph and once for an absolute path quoted while describing an earlier refusal); two more
  were `exit-echo-decoration-guard-vs-habit` firing twice in one session.
  **Why `[design-pending]`:** quote- and heredoc-aware matching in a PreToolUse hook is a shell
  parser, and the cheap approximations are wrong in both directions — skipping everything after a
  `<<` opener blinds the guard to real commands in a heredoc-bearing call, while matching only
  outside single quotes still refuses a double-quoted mention.
  **Cost while deferred:** a round trip per refusal on every session that documents its own
  tooling, and it lands hardest on the sessions writing the queue and the journals. The wider
  cost is the one the SWOT names: false positives on a blocking guard are the mechanism by which
  enforcement converts into bypass and distrust.
  Filed 2026-08-08 at scope on the lead's ruling, from this session's own refusal count. It
  duplicates neither `guard-command-prefix-wrapper` (transparent prefixes, measured at ~32% of
  prompting calls) nor `exit-echo-decoration-guard-vs-habit` (decoration on an allowlisted
  command); this is the guard refusing a string that was never going to be executed.

- **audit-class-corpus-attestation** [design-pending] — an un-gateable-class audit stamps a
  **verdict**, not the corpus it read, so a false negative is indistinguishable from a clean tree.
  `.workflow/audit-roster.txt` rows carry `due:` and `last:` and nothing else, so the close that
  performs one records *that* it swept and reports its finding count in prose. "Came back clean"
  is unfalsifiable at the time it is written and un-re-runnable afterwards.
  **Measured harm, and it is not hypothetical — a two-for-two false negative on consecutive
  closes.** `capability-pendency-after-landing`'s trigger fired when `native-cohort-activation`
  cut v0.22.0 (`.workflow/release-disposition.txt`, `git show d64e63c0`), the first tag publishing
  gate binaries as Release assets. Both that close and the next swept the class and stamped it
  performed: `ad8d4a31` recorded "the kit SPECs came back clean" and `9b2aec20` recorded "zero
  findings". `gate-sdk/SPEC.md` carried the discharged-blocker claim in **two** places throughout
  (§The port-candidate criteria criterion 5, and the paragraph opening "What stands between that
  port and an adopter"). Both survived both sweeps and shipped to the published SPEC and its
  public docs mirror, where validate found one of them two iterations later.
  **The deliverable is a stamp grammar, not a scanner** — which is what makes it buildable against
  a class the roster's own text calls un-gateable. A row's `last:` gains the corpus command the
  sweep ran and the hit count it triaged, so the next close re-runs the predecessor's own oracle
  instead of re-inventing a corpus, and a sweep that read nothing cannot stamp a verdict. What
  stays un-gateable is the *judgment* on each hit; what stops being un-gateable is whether a
  corpus was read at all.
  **Why `[design-pending]`:** three classes on the roster have no single-command corpus (a
  capability's "instances" are the tree set a scanner cannot infer — the reason they are on this
  roster), so the unit must rule what a row with no derivable corpus stamps instead of a command,
  and an honest "these rows stamp a named surface list" may be the answer for some of them.
  **Cost while deferred:** compounding and paid at the boundary the roster exists to hold — every
  close re-derives each class's corpus from scratch, and the roster's whole value is a cadence
  claim it currently cannot evidence. The two closes above are the attested instances.
  Filed 2026-08-09 by close (`install-profile-seam`), draining the criterion-5 staleness bullet:
  the bullet's own finding is fixed in this close, and this entry is the mechanism half of it.
  It is the generalization `spec-measured-count-gate` names as the scope-claim axis it cannot
  reach, and is cross-referenced there rather than folded into it — that entry designs a scanner
  over authored prose, this one designs a stamp over a session act.

- **absorbed-duplicate-disposition** [design-pending] — the queue has **no third state between
  live and done** for a slug merged away as a duplicate, so an absorbed entry is counted as a
  shipped deliverable.
  `check-task-conservation` diffs the live slug set and its own help sanctions moving a renamed
  slug to Done, so an absorbed duplicate has nowhere else to go. But Done means completed:
  `stage-economics-log-multi-session-undercount` shipped nothing and its content survives under
  `stage-economics-log-key-session-collision`, which is untouched and still live. `kpi-task-split`
  reads Done, so this iteration's feat/debt split inherits a row nothing built.
  **Distinct from the icebox tier**, which is the deferred pool's exit and holds *unbuilt* work
  that stays promotable; an absorbed duplicate is not dormant, it is redundant, and putting it
  back in a live section would re-open the namespace collision the merge closed.
  **Why `[design-pending]`:** every cheap answer costs something the conservation gate is holding.
  A fourth section re-opens the section-order contract queue-kit/SPEC.md §The icebox tier already
  records as gate-unenforced; an `[absorbed-by: <slug>]` tag keeps one section and obliges the
  KPI readers to learn it; and letting the slug simply vanish is the sanctioned disappearance the
  icebox tier's design explicitly declined to invent.
  **Cost while deferred:** low per instance and silent — every merge inflates the delivered-unit
  count by one and no reader of the KPI can tell. It recurs whenever a scope survey merges a
  filed duplicate, which the deferred pool's intake asymmetry makes routine.
  Filed 2026-08-09 by close (`install-profile-seam`), draining the bullet this iteration's own
  scope session filed against its own economics merge. This close dispositions the standing
  instance as an absorbed duplicate rather than as a deliverable, in its Done-clearing commit.

- **dispatch-cited-evidence-unverified** [design-pending] — a dispatched sweep's **quoted
  evidence** is covered by no verification rule, and one returned an attributed quotation that
  exists in no revision of the file it named.
  delegation-kit's shipped verify-after-commit set covers what a sweep *writes*; nothing covers
  what it *cites*. The class matters more than a wrong line number or a stale fact because
  quotation-with-attribution is precisely the signal a reader uses to decide a claim has already
  been checked, so it defeats normal trust rather than merely being wrong.
  **Caught 2026-08-08 at spec only by re-reading every load-bearing fact by hand against the named
  file** — a property of that session, not of the mechanism.
  **Candidate rule, stated where agent-execution binds:** a dispatched sweep's quoted evidence is
  a pointer to verify at the source, never evidence in itself. **Why `[design-pending]`:** stated
  that way it is an authoring contract with no oracle, and the buyable half may be narrower — a
  dispatch that must quote can be required to return file:line for every quotation, which makes
  the check mechanical for the dispatcher even though no gate can run it.
  **Cost while deferred:** compounding, and it lands on exactly the dispatches delegation is
  pre-authorized for. Every read-heavy audit this repo runs returns prose the dispatcher is
  expected to act on without re-reading the corpus — which is the whole economic point of the
  dispatch, and the reason an unverifiable quotation inside one is worth more than it looks.
  Filed 2026-08-09 by close (`install-profile-seam`), draining the bullet spec filed under
  scope-gated intake. It is filed here rather than left in the survey record for the reason the
  bullet itself gives: the record is boundary-truncated, so the finding would be erased at the
  next scope boundary.

- **icebox-worklist-roadmap-blind** [design-pending] — `queue-index.sh --icebox-candidates`
  filters on age and cost class only, so it offers candidates the icebox tier's own eligibility
  rule categorically excludes.
  queue-kit/SPEC.md §The icebox tier states that a roadmap-tagged entry is not
  icebox-eligible — a hard rule, not a judgment. The worklist does not read the tag.
  **Measured this close: 3 of 3 offered candidates carried a roadmap tag**
  (`plugin-marketplace`, `benchmark-ab-experiment`, `hosted-attestation-service`), so the
  worklist's precision was zero and every row was resolved by re-deriving the same exclusion.
  **The tool is honest about being advisory** — its own `spec:` comment says the age filter
  "only bounds how much close must look at" — so this is not a correctness defect, and that is
  why it is filed rather than fixed in place. The question the unit owes is whether a
  *categorical* eligibility rule belongs in the bound at all, given the tier deliberately keeps
  judgment out of the tool.
  **Cost while deferred:** low, recurring, and paid by every close — three entries re-read and
  one rule re-derived per iteration, forever, against a one-predicate fix.
  Filed 2026-08-09 by close (`install-profile-seam`) from its own backlog-eviction step.

- **cat-read-steer-guard** [design-pending] — `cat <file>` is the top prompting pattern and the
  guard steers `sed` reads but not `cat` ones.
  **Measured this close by `scan-prompts.sh`: 29 of 81 prompting calls across 25 patterns** —
  more than a third of the friction, and 5x the next pattern. `scripts/bash-guard.sh` already
  carries the precedent shape for `sed` ("don't read a file through 'sed' — use the Read tool"),
  with a pipe carve-out, and `cat` has neither a guard rule nor an allowlist entry.
  **Allowlisting is the wrong disposition and the triage criterion says so:** the form to
  reinforce is the Read tool, so a `Bash(cat *)` grant would mask the steering opportunity rather
  than take it. **Why `[design-pending]`:** the carve-out is the design — `cat file |` feeding a
  filter is legitimate and must not be refused, and the sed rule's own text shows the boundary
  is expressible; what the unit owes is a decision-table row plus its fixture pair, which is
  build-shaped rather than close-shaped.
  **Cost while deferred:** low per instance, high in aggregate — a round trip on roughly a third
  of all prompting calls, paid by every session, and it trains the habit the harness's own
  guidance already discourages.
  Filed 2026-08-09 by close (`install-profile-seam`) from its prompt-friction triage.

- **qualified-pointer-section-ownership** [design-pending] — `check-spec-pointer` asserts a
  cited `§Heading` **exists**, never that it is the heading which *owns* the cited claim, so a
  fully-qualified pointer aimed at the wrong section resolves and reds nothing.
  **Self-witnessed 2026-08-09, which is why it is filed rather than theorised.** This close,
  correcting another entry's expired premise, cited `gate-sdk/SPEC.md §What the dispatch seam
  does not settle` for a claim owned by §What is retained, and where the second port stands.
  Both sections exist. The gate passed — "every target file tracked and named §heading present"
  is its own verdict text, and presence is the whole of what it checks. Caught only by reading
  the file to confirm the sentence was there.
  **It is the mechanism under this iteration's headline class.** The inbound half of
  `ruling-record-condition-staleness-probe` — a citation surviving the deletion of the ruling
  it names — and this are one defect seen from two sides: a pointer is verified against the
  section's *existence*, so nothing notices when the section stops carrying the claim, whether
  because the claim was pruned out of it or because the wrong section was named to begin with.
  **Distinct from the three sibling entries in this cluster**, which are all about citations
  that resolve to *nothing*: `unqualified-section-citation-liveness` (a bare `§Heading` with no
  path), `spec-pointer-self-section-citation` (the self-citing form), and
  `prose-filename-citation-liveness`. This one resolves successfully and is wrong anyway, which
  is the harder half — the reader's trust is higher precisely because the pointer works.
  **Why `[design-pending]`, and an honest "not buildable" is a permitted outcome.** Deciding
  whether a section *supports* a sentence is comprehension, not scanning. The only mechanical
  approximations are term-overlap heuristics between the citing sentence and the target
  section, whose false-positive surface is the same one `spec-measured-count-gate` is stuck on,
  and a term-overlap red on a correct citation is worse than silence on a wrong one.
  **Cost while deferred:** broad and unmeasured — `check-spec-pointer` reports 902 directive
  pointers and 247 prose citations, and the ownership of every one of them is
  unverified. The cost is not that they are wrong; it is that the gate's green is read as
  saying they are right. **Both figures re-measured 2026-08-09 at scope by re-running the gate:
  the directive count filed here was 1774, wrong by roughly a factor of two on the day it was
  written. Read the numbers off the oracle, never off this line.**
  Filed 2026-08-09 by close (`install-profile-seam`), from its own miswritten citation.

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
- **evidence-journal-hash-chain** [design-pending] — Tamper-evidence wanted only by a hosted rung.
- **md-section-near-miss-match** [design-pending] — Empty on a near miss; correct on an exact query.
- **amendment-update-target-coverage** [design-pending] — Align checks it by hand; no gate yet.
- **gap-inbox-commit-ownership** [design-pending] — Who commits a lead-filed bullet is unspecified.
- **operator-authored-unit-set** [design-pending] — The contract omits operator-authored unit sets.
- **tarball-build-attestation** [design-pending] — The checksum proves transfer only; docs agree.
- **action-run-shell-scan-predicate** [design-pending] — No consumer seam on a correct gate.
- **scratch-execution-allowlist-bar** [design-pending] — Each close re-derives this standing bar.
- **capture-affordance-help-flag** [design-pending] — file-gap.sh files --help as a gap.

## Done

## Lessons Learned
