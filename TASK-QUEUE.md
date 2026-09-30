# TASK-QUEUE.md — Checkwright work queue

## Iteration: guard-kit-steering

  The lifecycle-kit gates read this header's iteration name and the stage cursor — the last stamp in `.workflow/WORKFLOW-STATE.txt` (lifecycle-kit/SPEC.md §The state machine); queue-kit formalizes the queue format itself and gates this file. One iteration per hardening or roadmap unit; [docs/kits.md](docs/kits.md) maps the kits.

---

## New Features

### gate-output-contributor-door

[spec: SPEC-gate-output-door.md] [recurrence: 2026-09-27]

a gate's printed finding or help line reaches an adopter's installed tree, yet about fourteen `println!`/`format!` sites under `native/src/gates/` still name `bash gate-sdk/bin/run-gates.sh` as the command to run (the install-lifecycle and install-doctrine remedies, the roadmap and value-rollup regenerators, the file-gap, file-survey and cite-survey arms, among them); an adopter's door is the binary `GATE_SDK_NATIVE_BIN` names. `check-door-binding` reads kit READMEs, templates, `lib/` and `bin/` and never a compiled gate's strings, so nothing reds.

**Recurred 2026-09-27, observed in a consumer:** in a scratch prose install of v0.26.0, `check-graph`'s red printed its hook and graph regenerators in the bash spelling (`native/src/gates/graph.rs`, three sites), while `./scripts/checkwright-gates --emit git-hooks --write` and `--emit graph` ran there. A starter or prose install owes no bash (installer/SPEC.md §Requirements), and run-gates.sh needs bash 4.3, which stock macOS lacks, so the remedy is unrunnable on such a host, not only misrouted.

**Deliverable:** each site re-pointed at the binary `GATE_SDK_NATIVE_BIN` names, or declared contributor-facing, and a check-door-binding assertion over gate-module output strings holding it.

**Cost while deferred:** an adopter following a red's remedy runs a path their tree lacks. Filed 2026-09-26 to the gap inbox by the done-claim-demo build (check-evidence-manifest's assertion-C remedy, fixed at the drain); promoted 2026-09-26 at its close: →fix fails because the holding assertion is new mechanism and some sites (the prose-bounds worklist, the smoke-entry guard) need a contributor-or-adopter call each. Re-verified at the drain: `git grep 'run-gates.sh' native/src/gates` over `println!`/`format!` lines returns fifteen sites before the fix. Owner lookup: `door`, `run-gates.sh`, `remedy` in this file — none live; owner guard-kit/SPEC.md §check-door-binding.

**Selected 2026-09-30 for guard-kit-steering, operator direction lead-relayed (not a ruling);** the spec stage authors and promotes it, after [guard-kit-value-audit](#guard-kit-value-audit) rules the gate. Re-verified: 16 `println!`/`format!` lines under `native/src/gates` name `run-gates.sh`.

### side-effect-free-read-arms

[spec: SPEC-side-effect-free-read-arms.md]

shell utilities an agent runs for read-only work can also write: `sed -i`, `find -delete` and `-exec`, awk's `system()`, `tee`, redirects. The operator's shape: block side-effect-capable utilities and steer to side-effect-free arms of the gate binary that can be allowlisted and advertised as their replacements. The seed is a read-only subset declared beside gate-sdk's fence-safe arm set, `FENCE_SAFE_ARMS`, which admits working-tree writes (gate-sdk/SPEC.md §The non-gate arm) and so cannot be the seed itself; corrected at spec. It succeeds guard-kit rule `worktree_confinement`'s admitted read for isolated children, whose declared-forms trust is that rule's stated honest limit.

**Deliverable:** standard utilities first — a steer to a side-effect-free standard spelling where one exists, a static program check admitting a side-effect-free `sed` or `awk` program, and a read-only arm set for the residue; no allowlist entry. New governed names, so it owes an amendment.

**Cost while deferred:** a read-only shell call keeps a write path the guard must judge per call, and an isolated child's read set stays the interim allowlist. Filed 2026-09-23 to the gap inbox by the lead on an operator direction; the operator recalls earlier discussion and no tracked record was found. Promoted 2026-09-23 at seam-and-stage-residue's close drain: an initiative with new names, never a drain fix. Owner lookup: `side-effect`, `FENCE_SAFE`, `dual-use`, `sed -i` in this file — none. Surface also delegation-kit and gate-sdk.

**Selected 2026-09-30 for guard-kit-steering, operator direction lead-relayed (not a ruling);** the spec stage authors and promotes it, the set's one unit reaching gate-sdk's arm set and delegation-kit's isolated-child allowlist. Clarified 2026-09-30, operator direction lead-relayed (not a ruling): a side-effect-capable read falls through to the harness's out-of-band decision, the auto-mode classifier with its false positives, and the unit takes those calls off that path.

## Technical Debt

### guard-kit-front-brevity

guard-kit/SPEC.md's front sections under [spec-brevity-residue](#spec-brevity-residue)'s three moves (run-on structure, archaeology, restatement): §The friction loop, §The shell guard, §Consumer rules and §The hook on native Windows, about 7.8k of the file's 48.2k words; guard-kit's other sections stay on the parent.

**Deliverable:** the three moves over those sections under `check-prose-bounds` and `check-provenance-seam`'s dated arm, every `##` heading kept verbatim and every fact another surface cites into them kept, per a citation survey. Applied after the iteration's guard-kit amendments merge, so no section is passed twice in one iteration.

**Cost while deferred:** paid by every session and adopter that reads an unpassed guard-kit section. Filed 2026-09-30 as a split at guard-kit-steering's scope, next in the parent's size order since installer's §The consumer smoke waits on [compiled-consumer-smoke-driver](#compiled-consumer-smoke-driver). Part of the operator's selection of the set, direction 2026-09-30, lead-relayed (not a ruling).

### guard-kit-value-audit

[gate-customer-value-audit](#gate-customer-value-audit)'s guard-kit slice: `check-door-binding`, the one gate guard-kit ships (`# install: zero-config`), audited for customer value and configurability under gate-sdk/SPEC.md §Consumer payload's rule.

**Deliverable:** the gate's verdict applied (made generic, kept out of the payload and the customer-OS legs, or kept) and the parent's audited tally updated. Ruled before [gate-output-contributor-door](#gate-output-contributor-door), whose new assertion lands on this gate.

**Cost while deferred:** the parent's cost, for this kit. Filed 2026-09-30 as a split at guard-kit-steering's scope; its landing moves the parent's roadmap row to `now` under the operator's horizon principle. Part of the operator's selection of the set, direction 2026-09-30, lead-relayed (not a ruling).

## Deferred

### shellcheck-extra-dirs-trigger

[cost: event/low] [surface: gate-sdk]

`check-shellcheck`'s generated-hook trigger never fires on a directory only `GATE_SDK_LINT_EXTRA_DIRS` adds: the knob is a `.words()` row, which `check-graph` refuses as a `knob:` couples token, so an edit there is linted by the full battery and CI but not at commit. gate-sdk/SPEC.md §check-shellcheck states the limit.

**Deliverable:** a manifest token that expands a word-list knob into trigger globs, or a boundary note ruling the late tier acceptable.

**Cost while deferred:** a consumer's lint findings in knob-added dirs arrive one tier late. Filed 2026-09-26 to the gap inbox by gate-sdk-tooling-brevity; promoted 2026-09-26 at capture-integrity-brevity's close: →fix fails because a new manifest token is a contract change to `check-graph`. Re-verified: `GATE_SDK_LINT_EXTRA_DIRS` is declared `.words()` in `native/src/knobs/gate_sdk.rs`, and the descriptor couples only `knob:GATE_SDK_GATES_DIR/*.sh,kit:*.sh`. Owner lookup: `LINT_EXTRA`, `word-list` in this file — none; owner gate-sdk/SPEC.md §check-shellcheck.

### seeded-ci-gates-on-surface

[cost: event/low] [surface: gate-sdk]

`init` seeds `.github/workflows/gates.yml`, but `check-action-pinning` and `check-action-permissions` stay `# install: on-surface`, so an adopter's own workflows are held by neither unless the adopter registers them. Keeping both on-surface was decided at front-door-release's spec. Moving them to zero-config would arm their tag-ref and undeclared-scope rules on every adopter's workflows at install, and on `update` for an existing tree.

**Deliverable:** both gates zero-config with a tightened-gate release note and the adopter allowed-red the upgrade contract requires (installer/SPEC.md §The upgrade contract), or a boundary note in gate-sdk/SPEC.md §check-action-pinning ruling on-surface permanent.

**Cost while deferred:** an adopter's hand-written workflows carry unpinned actions and undeclared token scopes with no red, paid at each adopter install. Filed 2026-09-26 to the gap inbox by front-door-release's spec; promoted 2026-09-26 at its close: →fix fails because the move changes what an adopter's install reds, which is user-facing semantics and owes a release note. Re-verified: both `.gate` files read `# install: on-surface`. Owner lookup: `action-pinning`, `action-permissions`, `zero-config` in this file — none; owner gate-sdk/SPEC.md §check-action-pinning. Surface also installer.

### spec-pointer-unstated-literal

[cost: event/low] [surface: native]

ported gate modules carry `spec:` comments citing a section for a literal awk or regex spelling, or a transcription note about the retired shell form, that the section states only as policy or not at all. `check-spec-pointer` checks only that the target resolves, so it cannot see this. Instances: `native/src/gates/gate_fail_closed.rs` (the `(awk|jq)` boundary regex, the per-FNR state machine), `native/src/gates/install_disposition.rs` (the `sub(/^# install:.../)` read, the `check-[a-z0-9]+` pattern) and `native/src/gates/assertion_strength.rs` (the awk hash-order note).

**Inferred, not run:** the class reaches past these three modules into the rest of the crate's ported gates.

**Deliverable:** a sweep of the crate's `spec:` comments against their cited sections, each unstated one retagged `comment-tier-exempt:` where the fact is local, deleted where it is port residue, or moved to the section where it is a contract.

**Cost while deferred:** a reader following such a pointer finds no support for the literal, and a later SPEC edit cannot tell the comment depends on it. Filed 2026-09-26 to the gap inbox by gate-sdk-meta-gate-brevity's citation survey; promoted 2026-09-26 at front-door-release's close: →fix fails because the class spans the crate and each comment needs a local-versus-contract call. The survey's other instance, `native/src/hook/stop_liveness.rs` citing §check-test-hermetic for per-case scratch roots, was fixed at the drain, as were eight in `native/src/emit/pack_installer.rs` citing §The packer (2026-09-29 drain), which shows the class reaching emit arms too. Folded in at preview-readiness' close, from `drift-kit-measurement-brevity`'s citation survey, all pre-dating this entry: the KPI members under `native/src/emit/kpi/` (`overhead.rs`, `gate_runtime.rs`, `gate_backlog.rs`, `task_split.rs`, `incident_recurrence.rs`, `queue_net_delta.rs`, `amendment_age.rs`) citing §Bundled KPIs, and `native/src/emit/stage_economics.rs` and `native/src/history.rs` citing §The stage-economics meter, for window sizes, rounding, age bands, tie-breaks, regexes and iteration orders neither section states. Owner lookup: `spec-pointer`, `comment-tier-exempt`, `shell form` in this file — none; owner canon-kit/SPEC.md §check-comment-tier.

### absence-statement-gate-arms

[cost: event/low] [surface: canon-kit]

doctrine-kit/DOCTRINE.md's Absence statements rule has an audit-roster class and no gate. Two arms are narrow enough to decide mechanically: (a) a `check-prose-tells` arm redding a section whose whole body is one negative-existential sentence, over a consumer-configured surface glob, off by default and never all markdown, exempting generated regions; (b) a placeholder-slug denylist in `check-task-names`, since a queue section holding only a `none` bullet or an indented None passes `check-task-names` and `check-queue-hygiene` today. The roster class calls the rule un-gateable as a whole; these arms take only the shapes that need no judgment of whether a reader must know the question was considered.

**Deliverable:** per arm, a knob, an assertion and a `good/`+`bad/` fixture pair, or a SPEC boundary note refusing it.

**Cost while deferred:** an absence sentence on a ledger is found only by the close audit sweep. No live instance exists: the queue's and the ruling record's empty sections are headings alone. Filed 2026-09-26 to the gap inbox by absence-statement-grammar's amendment, which costed the arms without building them; promoted 2026-09-26 at its close: →fix fails because each arm is new mechanism with a knob, and no live instance needs repair. Owner lookup: `absence`, `placeholder` in this file — none live; owner doctrine-kit/DOCTRINE.md Absence statements, with the arms in canon-kit/SPEC.md §check-prose-tells and queue-kit/SPEC.md §check-task-names. Surface also queue-kit.

### docs-liquid-literal-unseen

[cost: event/low] [surface: site-kit]

a balanced Liquid token a docs page means literally (`{{ x }}`, `{% x %}`) parses, so `check-docs-liquid-parse` passes it, and renders as something else, usually blank. `check-docs-render-fidelity` renders through kramdown without Liquid, so it cannot see the loss either (site-kit/SPEC.md §check-docs-liquid-parse states both halves). Probed 2026-09-25 at close: outside raw blocks, only `docs/releases.md` carries tokens, and those are meant as Liquid.

**Deliverable:** a render-side assertion (Liquid-render each page against an empty context and diff against the source outside raw blocks), or a token scan outside raw blocks and the Liquid a page owns, or a SPEC boundary note refusing both.

**Cost while deferred:** a page documenting a template or workflow expression outside a raw block loses its literal text on the live site, silently. Filed 2026-09-24 to the gap inbox by hosted-install-path's build; promoted 2026-09-25 at its close: →fix fails because the assertion is new mechanism, and no live page carries the defect to repair. Owner lookup: `liquid`, `raw block` in this file — only the landed `pages-liquid-break-undetected`, whose subject is a parse break, not a literal token; owner site-kit/SPEC.md §check-docs-liquid-parse.

### config-variant-battery-harness

[cost: event/high] [surface: gate-sdk]

nothing shipped lets a customer run the battery under a named config-seam variant and see what changes; the fixture pairs prove each gate's arm against fixed trees, and the smoke scripts are this repo's harness legs.

**Operator ruling at consult: file it costed.** Deliverable: a shipped, bridged arm that takes a scratch copy of the consumer's tree, applies a named variant of the config seam, runs the battery, and prints the per-gate verdict diff against baseline — so an adopter sees which gates a knob arms, disarms or reds before committing the knob.

**Why design-pending:** the variant's declaration form (an env file or a config-dir overlay), whether the scratch copy is a worktree or a copy that carries untracked content, and whether the diff or a full report is the product. Native, per the interpreter constraint (gate-sdk/SPEC.md §The adopter constraints).

**Refused:** repurposing the smoke scripts, which are install recipes read as text by the install-disposition gate and bound to this repo's harness; a shipped directory of shell tests, which widens the interpreter surface the adopter constraints shrink.

**Cost while deferred:** an adopter evaluating a knob edits the seam, commits, and learns from the next red; the preview cohort's false-positive dispositions have no cheap rehearsal. Filed 2026-09-11 by consult as a direct entry, the test gap the operator named there.

### benchmark-ab-experiment

[roadmap: later/adoption] [cost: once/low] [surface: drift-kit] [roadmap-summary: A controlled experiment measuring drift with and without governance.]

a controlled A/B trial.

**Cost while deferred:** zero — the self-referential drift-trajectory route already carries the claim this rung would upgrade, and the measurement half it consumes ships independently; what is foregone is an externally-comparable number the project does not currently claim. The controlled differential experiment: same model, same dependent-task series, two arms (ungoverned loop vs Checkwright-governed), drift *accumulation across the series* as the metric — a governance layer's effect, not a model leaderboard number. Metric axis: Drift-Bench's "satisfiable drift". Substrate/vocab primaries: seqBench (arXiv 2509.16866), Drift-Bench (arXiv 2602.02455 — real title "Diagnosing Cooperative Breakdowns in LLM Agents under Input Faults via Multi-Turn Interaction"; the "Decomposing Reasoning Into Failure Types" expansion is confabulated, do not repeat it), Lost-in-Conversation / FlowBench as prior art. Surfaced 2026-07-08 inside adoption-track; split out 2026-07-09 — the self-referential route (drift-trajectory) ships first and this rung upgrades the claim only if demand attests it. The experiment's measurement half — per-stage, per-model, price-weighted token burn off harness transcripts — is the stage-economics-report tool filed above; this rung consumes it rather than rebuilding it. Nearer use of that tool: verifying the split-lead posture's savings (lifecycle-kit/templates/lead.md §Economics). Surfaced 2026-07-15 by the per-stage budget analysis that motivated that posture.

### hosted-attestation-service

[roadmap: later/commercial] [cost: event/low] [surface: evidence-kit] [roadmap-summary: Gate runs verified by a neutral party no committing agent can touch.]

hosted attestation. The team/paid rung: gates verified server-side by a party the committing agents cannot touch — hosted gate runs as a neutral attestation, cross-repo drift dashboards, maintained rulesets. A service, not code: cloning the kits does not clone the neutrality or the ops. Demand-gated — this entry is the public roadmap marker, not a scaffold; hosting and sequencing decisions are on record in the operator's local brief, and multi-operator-semantics is its prerequisite mechanism. Surfaced 2026-07-07.

**Cost while deferred:** zero — this is a service rather than tree mechanism, so nothing rots; the residue is that gate runs stay self-attested, which binds only when a party the committing agents cannot touch is asked to trust them.

### heterogeneous-agent-delegation

[roadmap: later/ecosystem] [cost: iteration/low] [surface: delegation-kit] [roadmap-summary: Dispatch a stage to any vendor's coding agent, gated identically.]

foreign agents. Cross-vendor stage dispatch: a lead delegating a stage to a foreign coding agent. It cashes the public no-lock-in claim and is the purest expression of the thesis — governance enforced at the git/gate boundary, not by trusting the author. *Already agent-neutral:* the verification substrate (git, the gate battery, the bash stamp state machine) does not care who authored the diff, and the coordination primitive is the shared git-index/HEAD serialization. *Homogeneous today — the real work, worst-first:* (1) the **escalation resume model** collapses into (2) as a property of the chosen transport, per the 2026-07-25 amendment below; (2) **dispatch transport** — today the harness `Agent`/`SendMessage`/task-notification; a foreign agent needs a transport-neutral handoff. The adapter contract is "open / prompt / permission-request / resume" spoken over each vendor's structured **machine plane, never its TUI**: a screen-scrape relay is the adapter of last resort for a vendor shipping no machine interface at all — it yields rendered frames not turn events, answers dialogs by heuristic, and bets on the vendor's least-stable surface. (3) **budget oracle** — the verdict tool is Anthropic-OAuth-specific; a heterogeneous fleet has N vendor-keyed oracles, the same seam as the credential-swap entries, and the vendors' JSONL event streams carry the token-usage events a TUI path would scrape from a status bar. (4) **stage-contract expression** — the lifecycle machinery is neutral bash but the stage-skill prose is not.

**Seam ruling (on record):** generic mechanism only — transport, budget oracle, and escalation channel become consumer-config seams; a kit literal naming a vendor crosses the provenance seam and is ruled out, the pattern the retired `prose-profile` ruled. It extends the per-batch model-tiering lever across vendors, and interacts with [hosted-attestation-service](#hosted-attestation-service), the harness plugin package's reach ([plugin-harness-reach](#plugin-harness-reach)), and the credential-swap entries.

**Demand-gated — demand attested (2026-07-23):** the operator holds working foreign-vendor subscriptions and wants read-heavy delegation routed to them for budget headroom, and with three vendors live the N-keyed oracle seam is no longer hypothetical. First slice at promotion: a foreign-CLI executor for the already-pre-authorized read-heavy audit / mechanical-sweep class over a spawned non-interactive CLI process, one adapter per vendor as consumer config — not full stage dispatch.

**Its citers** ([companion-toolkit-profile](#companion-toolkit-profile), the credential-swap entries) block on none of it.

**Its scoping owes an attribution policy**, operator direction 2026-09-29, lead-relayed (not a ruling): each foreign harness's trailer adds a GitHub contributor per vendor. Candidates: each harness credits itself, or trailers off with the method stated once in the README (the lead's lean); a harness setting change waits on operator confirmation.

**Design-memory amendment (2026-07-25):** the TUI relay buys no session resume or token efficiency — both live in the vendor's session store (stateless APIs, the same on-disk transcript replayed against the same server-side prompt cache), so interactive-vs-headless is rendering, not state. Headless warm-resume by session id and JSONL turn events ship on the vendors probed, which makes (1) plumbing.

**Verification capability (2026-08-02):** those probes ran against **installed binaries**, so the executor is verifiable, not inferred from vendor docs — a change to the unit's risk under oracle-first: the executor ships with a smoke that invokes them. The machine profile (context-kit/SPEC.md §bin/env-probe, local-only) owns which CLIs and how.

**Cost while deferred:** the foregone lever is live — read-heavy audits and mechanical sweeps all bill against one vendor's budget while three subscriptions are held — and this design memory ages against fast-moving CLIs. Surfaced 2026-07-17 in the release-in-lifecycle lead session (operator question).

### background-credential-swap-support

[cost: event/high] [surface: delegation-kit]

first-class support for swapping the Anthropic OAuth credential out from under in-flight agents (to spread burn across accounts), which the budget oracle does not model today. Four components, worst-first; all delegation-kit SPEC+code, all demand-gated (no one swaps in background yet — this is the roadmap marker).

**(a) Detection.** usage-verdict's auth-change reroute fires only on CRED_FILE mtime, so an out-of-band / env-var / path token swap that does not rewrite that file bypasses it — the verdict trusts the prior account's snapshot and the poller re-fetches the stale file's token. Broaden the reroute to also fire when the live account identity (oauthAccount.accountUuid / subscriptionType) differs from the snapshot's `account=` / `tier=`, forcing a re-poll on any swap.

**(b) Evidence.** the `.metric/` trend samples already carry `account=` / `tier=`, but the wave-over-wave burn projection reads the tail **unpartitioned**, so a swap reads as a spurious used% drop that corrupts the projection and masks aggregate load. Segment usage analysis by `account=` and mark the swap boundary in the trend log so the evidence is per-account-honest.

**(c) Safety.** the budget guard's premise is one account = one rate window per wave; background rotation moves the wall in-flight agents bill against and lets rotation collectively exceed what any single account's 5h/7-day PAUSE would allow while each account stays individually under threshold. Add a cross-account aggregate view so supported swapping cannot silently blow past the true combined ceiling.

**(d) Signal-quality refinement (advisory, not a bug).** the post-login reroute (`DELEGATION_KIT_LOGIN_WINDOW`) is correctly advisory-only — STALE never blocks (delegation-kit/SPEC.md §usage-verdict, which also states the server lag the next point turns on), so this is signal quality, not a dispatch-blocking defect. Two points: the window default is 600s while the SPEC's own stated server-lag is "about a minute", a ~10x margin worth tightening; and it is a **blanket** time-window where an **account-keyed** check is sharper — trust `usage.txt` when its `account=` matches the current credential's account AND `updated_at > login_at`, with a short (~90s) settling floor for the server lag. That restores the true reading in ~1 min instead of 10 and stops 10 min of STALE samples polluting the trend log (`.metric/usage-history.log`) — which directly sharpens (b).

**Cost while deferred:** any background swap today silently corrupts the burn projection and can breach the combined budget ceiling with every account reading individually safe; and the login window over-STALEs by ~10x.

**Seam:** all four are generic delegation-kit mechanism — the account-id is already on the `usage.txt` contract; nothing consumer-specific is added. This is the budget-oracle prerequisite cluster heterogeneous-agent-delegation cross-references. Surfaced 2026-07-17 in the release-in-lifecycle session (kfric plus one operator-raised refinement).

### companion-toolkit-profile

[roadmap: now/ecosystem] [cost: event/high] [surface: lifecycle-kit] [roadmap-summary: Gate a tree whose specs another toolkit's workflow wrote.]

the interop rung's submission half. The build half landed at companion-catalog-extension: `companion/` holds the Spec Kit extension and both recipes, the consumer smoke's companion arm proves them, and docs/spec-toolkits.md is their landing page (companion/SPEC.md). Horizon `now`, operator direction 2026-09-29, lead-relayed (not a ruling): the recent iterations work it toward its submission.

**Deliverable:** the Spec Kit community-catalog submission, filed as the catalog's Extension Submission issue with its `download_url` naming the `checkwright-companion-<version>.zip` Release asset; the extension's README and the landing page then gain the catalog's install form.

**Gated on** a published tag carrying that asset, since the catalog installs from a tagged archive, and on [design-partner-preview](#design-partner-preview)'s observed install. **Also gated, operator direction 2026-09-29, lead-relayed (not a ruling),** on [companion-install-tier](#companion-install-tier) and [gate-customer-value-audit](#gate-customer-value-audit): the audit's verdicts decide which gates that tier exposes, so scope reads the two as one sequencing. **The four preconditions the operator set on 2026-09-27 landed at catalog-submission-preconditions:** `crate-tests-windows-flip`, since red jobs inside a green run read as ignored failures; `linux-glibc-artifacts`; `catalog-landing-docs-polish`; and `spec-toolkits-guarantee`.

**Four more prerequisites, operator direction 2026-09-29, lead-relayed (not a ruling):** three landed at companion-technical-gates (`install-gate-selection`, `companion-spec-to-code-gates`, `speckit-extension-full-profile`), and `adoption-prompt-templates` landed at companion-adoption-landing. Ground: Spec Kit and OpenSpec are technical toolkits, so a companion offering only document gates reads as near-useless, and the launch needs an early-adopter wow.

**Push need (2026-09-27, inside the budget):** the release tag push beside the closing push, since the submission needs the extension on a published tag; close's release policy decides the cut.

**Cost while deferred:** the extension installs only from a Release URL a reader must already hold, so a Spec Kit user browsing the catalog, where adopters find enforcement extensions, does not find it. Surfaced 2026-08-02 at close; demoted 2026-09-27 at companion-catalog-extension's build, on its amendment's Definition of Done; survey correction (2), a lifecycle stage machine over a foreign workflow, went to the gap inbox.

### design-partner-preview

[roadmap: now/adoption] [cost: event/low] [surface: drift-kit] [roadmap-summary: A small observed preview measuring first green, first useful red and retention on real installs.]

a narrow external preview before any broad announcement: a preview cohort whose composition is ruled in the operator's private brief, the first install observed live and the rest arriving through the observation record, instrumented for time-to-first-green, first useful red, false-positive dispositions, and 7/30-day retention per kit. It is the first rung on this queue whose deliverable is **evidence from outside this tree** rather than a tree change.

**The TREE HALF landed in `external-install-evidence`** — drift-kit/SPEC.md §The install-observation record and §The install-evidence projection, the `--emit file-install` capture arm, and `docs/install-evidence.md` behind `check-install-evidence-fresh`. What is kept here is the expensive half: operator hours and a calendar window of thirty days or more, running beside later iterations and never inside a stage session. What re-promotes this entry is an observed install, not another tree change.

**Sequencing is the load-bearing part.** The preview runs *before* [benchmark-ab-experiment](#benchmark-ab-experiment), so pilot findings shape that experiment's task classes and metrics rather than being retrofitted to them; per-gate true/false-positive history and profile retention are preview deliverables, not pre-launch builds. The full launch ruling behind this sequencing is operator material and stays in the local-only private brief; this entry carries only the queue-visible rung.

**Expected FIRST FINDING, not a precondition:** today's quick start is curl, sha256sum, tar and `bash … init` from a repository root; macOS needs GNU bash and coreutils by adopter action; native Windows needs Git for Windows. That is why the merged channel gives the installer no delta — those host-floor facts are an output of the observation, not an input.

**Refused, grounds carried forward:** parking behind `native-windows-bash-floor` (landed 2026-09-18) or the git-only-floor discharge (the trigger is what the preview measures); the icebox (the highest cost-while-deferred in the intake).

**Held deferred twice on 2026-09-26, operator direction lead-relayed (not a ruling),** until the repaired front door (`front-door-demo-unreachable`, `demo-catches-a-done-claim`) was published, which v0.26.0 did.

**Consult ruling, 2026-09-27, superseding the 2026-09-25 recommendation to promote next:** the cohort was re-ruled in the brief — one observed pre-submission install, then the installers the catalogs and the plugin marketplace send — so this entry runs *through* the two ecosystem units the ledger's `catalog-then-plugin` ruling sequenced first, not ahead of them; that ruling discharged 2026-09-29, the extension and the plugin package both published. The observed install is owed before the catalog submission; the rest re-promotes on the first observed install, as stated above.

**Readiness ahead of the observed install, operator direction 2026-09-29, lead-relayed (not a ruling):** a clean-container first-run rehearsal ran first, as `front-door-container-rehearsal` at preview-readiness, its runs filed to the author seat's record (drift-kit/SPEC.md §The install-observation record), never the published one. Operator-seat adoption on the operator's own projects is independent evidence and gates nothing; counting it toward the observed install was declined.

**Cost while deferred — still the highest of its intake, and now the more exposed half.** Every claim that would be strongest with external evidence still rests on internal dogfooding, and the channel that would carry it is built and reading zero: the published page states an honest `0` on every pass while the volume of unattested governed surface keeps growing. Deferring also silently defers [benchmark-ab-experiment](#benchmark-ab-experiment), since running that first would fix the wrong metrics. Surfaced 2026-08-02 at close, in the same intake pass, as the review's fourth-ranked item.

### external-gate-quality-evidence

[cost: event/low] [surface: drift-kit]

durable, published evidence of **gate quality as experienced outside this tree**: per-gate true/false-positive history, the disposition of each red a non-author hit, and whether a red changed behaviour or was worked around. The direct answer to the standing threat that a false positive converts the enforcement advantage into bypass and distrust — every blocking gate raises the stakes of a wrong red.

**Why it is not just a report.** The tree already publishes evidence projections, so the mechanism exists; what does not exist is a *population* to measure. A red in this repo is authored and dispositioned by the same party, which cannot distinguish a gate that is right from a gate whose author agrees with it.

**The record's FIELDS landed in `external-install-evidence`**, settling the open design question the deferred form carried: the collection surface is a new capture stream — the observation record's `red` line, with its closed `<verdict>`, `<disposition>` and `<behaviour>` sets — rather than a field on a disposition this tree already records, the disposition being measured being the *adopter's*. What is kept here is the evidence itself, which accrues only once observations exist; `docs/install-evidence.md` publishes zero reds today, and a population is what re-promotes this entry.

**The bundle with [design-partner-preview](#design-partner-preview) was not convenience.** This entry's own carry sentence rules the history unrecoverable — it cannot be retroactively collected, and starts accruing only once someone decides to record it — so a protocol landing without the gate-quality fields designed into it would have sent the first installs' reds somewhere this baseline can never read. There was one chance to fix the record's fields and it is spent.

**Cost while deferred — moderate, asymmetric, and no longer unrecoverable.** The gate-quality claim still rests on fixture pairs and a green battery, which prove a gate does what its author specified and say nothing about whether that was the right thing to specify. The irreversible half is discharged: the record exists, so the history starts accruing at the first observation rather than never. What remains is that the first externally-hit false positive is argued from anecdote until a population exists. Surfaced 2026-08-02 at close, in the same intake pass, as the last of the growth half.

### gate-authoring-sdk-surface

[roadmap: next/ecosystem] [cost: event/low] [surface: gate-sdk] [roadmap-summary: Author a gate in any language behind one substrate-neutral descriptor.]

a gate-authoring SDK. `.gate` as the substrate-neutral surface. **Operator-surfaced during `native-gate-dispatch-seam` build; filed so the framing outlives the session that saw it.** Horizon set 2026-08-02 on the operator's steer: this is ecosystem work on [companion-toolkit-profile](#companion-toolkit-profile)'s rung, not "make our own gates fast and opaque".

**The observation:** because the manifest lives outside the implementation, the graph, hook, and meta-gate layers never learn what implements a gate. That is not a Rust seam that happens to work — it is a **language-agnostic** one. A gate could be written in any language behind a descriptor, and slice 1 already avoided baking a language into the descriptor format, the resolution path, and `check-gate-substrate-parity` assertion D (whose comment-leader match is `#`, `//` and `/*` deliberately). `GATE_SDK_NATIVE_SRC` is a path knob, not a language knob, for the same reason.

**Why it is an SDK question and not a port question:** the port asks "how does *this* gate move"; this asks what a **third party** needs to author a gate on any substrate — the descriptor contract, the subcommand calling convention, the output contract, and the fixture-pair obligation, which are already the four things gate-sdk holds. The kit is most of an SDK already; what is missing is the statement that the substrate is a parameter.

**Boundary against the two questions already settled, so this one does not sprawl:** how a compiled gate *arrives* and what it *discloses* are both ruled and recorded (gate-sdk/SPEC.md §Consumer payload, which owns both). What stays open is what a gate *is* independent of substrate, and that is this entry alone. The distinction it supplies outlived those rulings and is why it was worth keeping — a descriptor discloses a gate's **shape** without its **predicate**, which is the line the disclosure ruling drew.

**Not started, and deliberately not widened into slice 1**: building it would have meant generalizing a seam with exactly one instance, which is the shape of a design that fits nothing later.

**Cost while deferred:** each further port hardens substrate-specific assumptions by habit rather than by ruling, and the cheapest moment to keep the seam neutral is before the second language exists — not after.

**One citer blocks on it:** [gate-tamper-exemption-reader-substrate](#gate-tamper-exemption-reader-substrate) is design-pending precisely on the ruling this entry holds, so deferring this entry holds that one too. Filed 2026-08-02 by build, on an operator ruling, during `native-gate-dispatch-seam`.

### gate-tamper-exemption-reader-substrate

[cost: event/low] [surface: gate-sdk]

`check-gate-tamper`'s exemption reader has no implementation-side equivalent. Split 2026-08-09 at scope by operator ruling from `gate-tamper-roster-native-reach` (since retired), when that entry narrowed to its meta-path-roster half and promoted; this is the exemption half, unchanged in substance. That entry was itself split 2026-08-02 from `native-gate-meta-layer-reach`, so this is the second narrowing of one original gap. `extract_exemptions()` parses a shell `# exception-list:` array literal, so a ported gate's Rust module can carry no exemption the gate is able to read.

**Why design-pending:** it wants the ruling [gate-authoring-sdk-surface](#gate-authoring-sdk-surface) holds — whether a meta-gate reads a substrate-neutral descriptor or learns each substrate — and that entry is horizon-set to ecosystem work, so this one waits.

**The coupling was checked at the split rather than inherited.** It is true of this half and was not true of the roster half: which paths a tamper roster covers is configuration, where how a meta-gate reads an exemption across substrates is exactly the substrate-neutrality question the SDK entry holds.

**Cost while deferred:** zero until a ported gate needs an exemption; no first-cohort member carries an exemption list, which gate-sdk/SPEC.md §Meta-gate conservation for the binary substrate records in its `check-gate-tamper` row. Filed 2026-08-02 at close from the gap inbox; found by build. Split out 2026-08-09 at scope.

### session-model-identity-verification

[cost: event/high] [surface: delegation-kit]

a session cannot report or verify the model tier it is running at. The session-context hook prints iteration, budget and drift; `drift-report` prints neither. Nothing surfaces the running model, so a session cannot state its own tier without a human hand-reading the harness transcript, and no stage can assert the tier it was dispatched at.

**Operator-proposed shape, recorded 2026-08-04:** a `usage-verdict`-shaped check — snapshot in, exit 0/1/2, fail-soft — reusing the `--emit-session-id` arm's projects-dir derivation, with the *tier expectation in consumer config* rather than a kit literal. Both halves of that placement are forced: a baked model-name ladder is drift by construction (delegation-kit's agent-execution rule keys tiering to capability, not to a name), and the provenance seam keeps product constants out of kit literals regardless.

**Feature-shaped, so it wants `/spec`, not a debt promotion.** It spans derivation lifecycle or context, verdict delegation, and a consumer config surface, and it introduces a new governed name. Cross-kit ownership plus a new name is the amendment threshold, and a scope that promotes this straight to build will be authoring the contract inside the build.

**Cost while deferred:** every tiering rule in the tree is unverifiable — including the two filed alongside this one. [consult-tier-declaration](#consult-tier-declaration) blocks on it outright, and the `Co-Authored-By` attribution defect has no derivable fix without it. Filed 2026-08-04 at close from the gap inbox; filed by the lead.

### consult-tier-declaration

[blocked-by: session-model-identity-verification] [cost: event/high] [surface: delegation-kit]

`/consult` governs the tier of what it dispatches and asserts nothing about its own. The skill landed this iteration to carry judgment-tier boundary questions, and its own amendment argues it is judgment-tier *by nature* — yet it declares no floor for the session running it and verifies nothing at entry. A consultation answered at a cheap tier is indistinguishable, in the record, from one answered at the tier the skill was built for.

**Operator direction 2026-08-04:** it must run on the top model and verify that at entry.

**The shape that keeps the seam intact:** the *skill* declares its own floor, the *kit* never spells a model name — the same split [session-model-identity-verification](#session-model-identity-verification) sets up, which is why this blocks on it rather than racing it. Without the mechanism this entry is a prose assertion of the kind that already failed twice this iteration.

**Cost while deferred:** the repo's one escalation-grade skill is silently downgradeable, and the failure is invisible in the artifact — a thin consultation reads as a short one. Filed 2026-08-04 at close from the gap inbox; filed by the lead on operator direction.

### intra-file-pendency-contradiction-scan

[cost: event/high] [surface: canon-kit]

one file can call the same slug landed in one section and pending in another, and nothing reads both. Found at close 2026-08-04 by the `capability-pendency-after-landing` audit: gate-sdk/SPEC.md said a second port "lands after `native-artifact-publish-path` and `native-artifact-install-path`" while, ninety lines later, the same file said criterion 5 is "implemented by `native-artifact-publish-path` and `native-artifact-install-path`". Both landed 2026-08-03. Two sections, two tenses, one file, one slug pair.

**Why the existing coverage did not catch it, which is the point.** The `capability-pendency-after-landing` roster class *did* run at that iteration's own close and missed it, because the class is scoped as a human sweep of governed prose against the tree — an unbounded read whose reach depends on which files the sweeper opens. The stale paragraph was written mid-iteration and never revisited after the same iteration's later commit discharged it, so the tree-comparison the class prescribes never reached it.

**Deliverable, and why it is narrower than the class it sits under:** a scan for one *decidable* shape — a governed file citing a slug in a landed construction ("implemented by", "built", "ships") and in a pending construction ("waits on", "lands after", "does not exist yet", "not yet") within the same file. It needs no tree comparison and no judgment about what is actually live: the contradiction is internal, so the file falsifies itself. That is what makes it gateable where its parent class is not.

**Why design-pending:** the construction vocabulary is the whole gate, and a literal phrase list in a kit is drift by construction plus a provenance-seam problem — the vocabulary is consumer editorial. It wants the `check-graph` / `graph-vocab.knobs` treatment, optional consumer config, which is a design call rather than a size one. Also open: whether a legitimate "X landed, Y still waits on it" sentence pair trips it, which decides whether the predicate is per-slug or per-slug-per-section.

**Cost while deferred:** the class stays a sweep whose reach is whoever runs it, and its one measured miss cost a full iteration of a governed SPEC contradicting itself in public — gate-sdk/SPEC.md is mirrored to the docs site, so the contradiction shipped. Filed 2026-08-04 at close; the instances it would have caught were fixed the same session.

### baseline-row-prose-coupling-gate

[cost: event/low] [surface: canon-kit]

governed prose asserts what `.workflow/validate-baseline.txt` holds, and nothing checks it against the file.

**The instance that bought this entry** was fixed at this close, not deferred: `gate-sdk/SPEC.md` claimed in two places that the baseline carried a held `installer_smoke fail` row. It was flipped to `pass` in `97683db2`, so a cohort pricing criterion 5 read a pointer to a mechanism it could not find, and the cheapest wrong conclusion was that the row had been dropped rather than earned out. Both sentences were re-worded at this close.

**Why it is gateable, unlike its neighbours.** The general class — prose making claims about machine surfaces — is the human-audit class [gate-spec-claim-assertion-parity](#gate-spec-claim-assertion-parity) already rules ungateable. This slice is not: a sentence naming `.workflow/validate-baseline.txt` and quoting a `<suite> <verdict>` pair is a decidable pattern, and the live file is a two-column lookup. The scanner reds when a quoted verdict disagrees with the row.

**Deliverable:** a canon-kit gate over governed prose citing that file, with the `good`/`bad` fixture pair, plus a ruling on the past-tense form — a sentence deliberately recording a *retired* row (both repaired sentences are now exactly that) must not red, so the predicate needs a tense or a citation convention to key on. That convention is the design question.

**A SECOND instance was authored 2026-08-24, at this close's eviction review.** Ruling the `installer_smoke` row's attribution put a claim about that file's slug column into two governed surfaces at once — `bridged-knob-case-tmp-dir-override-inert`'s body and evidence-kit/SPEC.md §Baseline manifest. That pair is no longer live: the entry's body left the queue with its Done move (2026-09-12), so the claim now sits on the SPEC section alone.

**No `recurrence:` date joins:** the entry names an unbuilt gate rather than a defect, so authoring a new instance of the class it would catch is the class recurring, not the finding re-firing.

**Cost while deferred:** low and slow, but it recurs on exactly the readers who most need the file — a cohort pricing criterion 5 reads the prose first. Filed 2026-08-14 by close, from its own gap-inbox drain and staleness review; kept in Deferred at the 2026-08-24 eviction review on the trigger above and on the live slug it names.

### site-health-issue-venue-unwanted

[cost: event/low] [surface: site-kit] [not-icebox-eligible: 2026-09-21 operator-ruled, cron-armed]

the site-health probe files issues on the public repo for failures the iteration lifecycle resolves anyway, and the operator does not want that venue.

**Operator-ruled 2026-08-25: the issue-filing path is unwanted.** The objection is to the **venue**, not to the probe — and a later session must not read it as the probe being wrong. Both firings were true positives on arm #6, the Release body missing its note URL: 2026-08-08 on `v0.22.0` and 2026-08-24 on `v0.25.0`, each cleared by the probe's own recovery path.

**Those dates sit in this prose deliberately.** The operator has since deleted both issues — probed here, `gh issue list --state all` returns nothing — so the tracker is empty, two dead run-log URLs are all that survives of the evidence, and the underlying defect's repair landed as `release-body-step-has-no-in-tree-witness` — the publish job now composes the Release body, unwitnessed until the first tagged publish run — rather than at any issue that resolves. The `site-health` label survives and is harmless: the workflow's label creation is idempotent and its open-issue lookup returns empty either way.

**The deletion is not the fix, and an empty tracker is not the problem going away.** The workflow is unchanged and still armed on its `17 6 * * *` cron, so the next arm-#6 failure opens a fresh issue. That is the whole reason this entry exists.

**The fork, unresolved, and it is two changes rather than one.** (1) Repo-copy only: delete the step and the `issues: write` scope from `.github/workflows/site-health.yml`. One file — but that file is pinned governed repo-meta in `scripts/core-files.list` and is a copy of `site-kit/templates/site-health.yml`, so this forks the template a repo governed by its own kits dogfoods. (2) Kit-level opt-in: the issue path becomes consumer config defaulting **off** under the `<KIT>_<KNOB>` convention, with this repo taking the default — template plus site-kit/SPEC.md plus `site-kit/smoke/install.sh`, which copies the template in.

**Option 2 is the recommendation and it is BLOCKED, operator-class.** The template header states the issue path as a standing design ruling — a failed probe opens or updates an issue and recovery self-clears — so making it opt-out reverses that ruling, which is the operator's to do and neither a stage's nor a lead's. Recorded rather than resolved.

**The replacement signal is the whole cost, and one half is now probed.** Candidate A, red run only: GitHub's documented scheduled-failure notification targets the last modifier of the **cron syntax** — not the last committer — and here that is `016d522a`, 2026-07-10, the operator, so the channel resolves to the right person today. The limit that cannot be probed from the tree is whether their notification settings deliver it. Candidate B, write the failure report to the run's job summary: visible in the Actions tab and files nothing, but the workflow writes no step summary today, so this is net-new work rather than a redirect.

**Cost while deferred:** tracker noise on a public repo, and nothing worse — the probe is accurate and self-clearing, so no outage goes unseen while this waits. Filed 2026-08-25 by scope, operator-directed and relayed through the lead; the tree read behind it was re-run here rather than taken on the relay.

### harness-project-dir-fold-dialect-unresolved

[cost: event/low] [surface: context-kit] [not-icebox-eligible: 2026-09-10 live CI-leg trigger]

the harness project-dir derivation `check-memory-off` and its two shell twins share folds a repo root's `/` and `.` to `-`, and under gate-sdk/SPEC.md §The path-dialect contract's per-substrate dialects the two substrates fold the *same* Windows checkout to two different names: the crate reads a drive-lettered root and yields one spelling, an MSYS shell reads the `/c/…` spelling and yields another. Only one can match the directory the harness itself creates, so on Windows at most one of the three sites is right and nothing here says which.

**The three sites, verified 2026-08-30:** `native/src/gates/memory_off.rs` `memory_dir_default()` (a char fold over the raw repo root) and `scripts/session-context.sh`, its `tr '/.' '-'` fold, / `context-kit/templates/session-context.sh`, the same fold.

**Why it promotes rather than fixing or iceboxing.** →fix fails on evidence, not on effort: the missing fact is *which spelling the harness uses on Windows*, an observation of another program on a host this tree has none of, and no command on a Linux box produces it — writing a fold without it would be inventing a Windows fact, which is what spec declined to do. →icebox fails because a live trigger exists and is dated: the Windows leg `platform-support-ci-matrix` shipped before retiring 2026-09-06 still runs on every push to master and is the run that can observe it, and the migration that just landed made every *other* producer dialect-correct, so these three are now the tree's recorded exception rather than part of a uniform unfixed background.

**Standing exclusion — `lead, own-authority` 2026-09-10 through the lead's message channel, at `host-resolution-fail-open-cut`'s scope:** the worklist reads the retired slug and not the live CI leg `install-smoke-pwsh-windows`, so it scores this entry false-eligible; [icebox-trigger-blind-to-retired-carrier](#icebox-trigger-blind-to-retired-carrier) owns that predicate defect, DECLINED as a rider then with the exposure accepted in writing.

**Owner is context-kit, not gate-sdk.** The rule's home is context-kit/SPEC.md §Layout and configuration; the dialect contract is gate-sdk's. It is that seam, not a migration defect.

**Pre-existing, not a regression** — the fold is already wrong on a backslash-spelled root today, so `msys-dialect-migration` discharged its whole deliverable without answering this.

**Cost while deferred:** low today and stepwise later — no Windows adopter exists pre-launch, so the wrong fold silently disables a memory check nobody is running; it becomes reader-visible the first time a Windows session opens, which is the same event that supplies the answer.

**Deliverable:** the observed harness spelling recorded as a fact with its witness, one fold that produces it on both substrates, and a fixture pinning the cross-substrate agreement. Filed 2026-08-30 by close, promoted from the gap inbox (spec filed it; the three sites carry a recorded `spec:` verdict naming the open question rather than an invented answer).

### record-stamp-encoding-compression

[cost: event/low] [surface: queue-kit] [recurrence: 2026-09-03] [not-icebox-eligible: 2026-09-22 operator-ruled direction of 2026-09-01; evicting it would demote that ruling]

buy discrimination in the queue's record stamps by RE-ENCODING them rather than by adding text, the deferred pool's per-entry budget being what makes added text the wrong trade.

**Operator-ruled 2026-09-01, and the ruling picked a route none of the three escalated options offered.** The escalation asked how to disambiguate two same-day recurrences and proposed, among others, an iteration slug beside the date. That was REFUSED: adding a field spends the budget the format is trying to protect. The worked example given is `YYYY-MM-DD` → a dashless `YYMMDDHHMM` — **the same ten columns, now carrying hour and minute** — which discriminates same-day instances outright and needs no slug. Array notation for multiple stamps is named as a further step, and the direction is stated to generalize to other task-record components rather than to `recurrence:` alone.

**The envelope is one knob now.** queue-kit-unwrap moved the entry cap to code points (`QUEUE_KIT_ENTRY_CAP`) and deregistered `check-queue-wrap` here, so a shorter stamp frees budget but no columns; same-day discrimination is untouched. Weighed at that close and kept: dropping it would re-scope an operator ruling.

**The column axis was witnessed three times before the wrap limit left.** 2026-09-01: `/spec` blocked — `native-gate-port-remaining-corpus`'s lead line could not hold two `spec:` refs under 100 columns. 2026-09-03: the wall forced minting a second host, `drift-kit-bin-port-residue`, for an encoding reason. 2026-09-04: four cuts wanted four refs against a base that held one; four per-cut entries taken instead.

**The gain is the ENCODING, not the list, and the entry says so because the format already has the list.** queue-kit/SPEC.md §The tag algebra, the `recurrence:` declaration paragraph, defines `recurrence: <slug> <YYYY-MM-DD> [<YYYY-MM-DD>…]`, multiple dates on one line today. **A second interaction dissolves with it.** queue-kit/SPEC.md §The tag algebra, the self-naming-slug paragraph, grounds the field partly in `check-queue-hygiene` rejecting exact-duplicate lines, naming same-day recurrence on two entries as "exactly the case the declaration exists to record". Under a minute-bearing stamp those two lines stop colliding at all, so one of that field's two stated grounds is retired by the encoding rather than argued against.

**The costs, probed rather than listed, because a reader meeting this cold should price it.** Date stamps span `recurrence:` and `ruled:` declarations, filed-prose provenance lines, gap-inbox bullets, survey-record headings and WORKFLOW-STATE stamps; the evidence manifest's trailing date field is OPTIONAL and so is not a cost, correcting the relayed picture. FOUR crate gates carry a date predicate (`stage_evidence.rs`, `stage_entry.rs`, `gap_inbox_neutrality.rs`, `evidence_manifest.rs`, the first two spelling their own `is_date`), and SEVEN shell tools stamp `date +%F` outside fixtures and smoke, none of which stamps a time today. `YY` also drops the century, a deliberate trade rather than an oversight to find later.

**Why design-pending:** the ruling fixes the DIRECTION and not the grammar. Open: which components take the new encoding and in what order, whether the change is a migration or a read-both-write-new window, and what each date-reading gate asserts across it — a wrong answer reds every governed surface at once.

**Cost while deferred:** low and bounded — every entry needing discrimination keeps buying it with text against the entry budget. Filed 2026-09-01 by close under CLAUDE.md §Housekeeping's operator-directed exception; it rides no cut and is no hotfix.

### spec-brevity-residue

[roadmap: now/adoption] [cost: session/high] [surface: installer] [roadmap-summary: Kit SPECs that state their contracts without run-ons, history or restatement.]

the per-SPEC remainder of `spec-tier-brevity-pass`'s three moves (run-on structure, archaeology, restatement), outside the five sections that entry landed. In the filing profile's order, by size: gate-sdk landed in eight slices, as `gate-sdk-framework-brevity`, `gate-sdk-remainder-brevity`, `gate-sdk-porting-brevity`, `gate-sdk-native-brevity`, `gate-sdk-runner-brevity`, `gate-sdk-meta-gate-brevity`, `gate-sdk-tooling-brevity` and `gate-sdk-tail-brevity`; lifecycle-kit is finished (`lifecycle-template-brevity` and the three slices below); installer is finished apart from §The consumer smoke (its contract, install-surface and remaining sections landed as `installer-contract-brevity`, `installer-install-brevity` and `installer-remainder-brevity`); what remains starts at guard-kit (48.3k), delegation-kit (43.9k), canon-kit (39.1k), queue-kit (26.5k), drift-kit (22.1k), context-kit (19.8k), evidence-kit (14.9k), site-kit (9.5k), then doctrine-kit's DOCTRINE.md and SPEC.md. Horizon `now`, operator direction 2026-09-29, lead-relayed (not a ruling): a slice has landed at each recent scope; the per-kit slices stay off the roadmap.

**Deliverable:** the three moves applied SPEC by SPEC in that order, under the gates the first slice landed, `check-prose-bounds` and `check-provenance-seam`'s dated arm; one SPEC, or a batch of the small ones, per iteration, and gate-sdk in slices, since no iteration passes it whole. Not a wholesale cut: a contract sentence stays.

**Split ten times, 2026-09-25 to 09-27 at scope, each on an operator direction lead-relayed (not a /consult ruling):** gate-sdk's framework, remainder, porting, native-contracts, runner, meta-gate, tooling and tail slices each left as their own debt entry, all since landed; the ninth, installer's contract sections, left ahead of lifecycle-kit because the platform-prerequisite-floors theme rewrites them; the tenth is lifecycle-kit's template sections.

**Cost while deferred:** paid by every session that opens a section not yet passed and every adopter who reads one on the site. Filed 2026-09-25 at scope, split from spec-tier-brevity-pass on an operator direction, lead-relayed; the profile and sampled tables are in that entry's filing commit.

lifecycle-kit's sections above §Per-component contracts left 2026-09-28 at consult-inbox-front-brevity's scope, on an operator direction lead-relayed (not a /consult ruling), and landed as `lifecycle-kit-front-brevity`.

lifecycle-kit's state-machine tool sections left 2026-09-28 at lifecycle-machine-brevity's scope, on an operator direction lead-relayed (not a /consult ruling), and landed as `lifecycle-kit-machine-brevity`.

lifecycle-kit's remaining sections left 2026-09-29 at native-hook-customer-legs' scope, on an operator direction lead-relayed (not a /consult ruling), and landed as `lifecycle-kit-tail-brevity`, which finishes lifecycle-kit.

installer's install-surface sections (§The verbs through §The manifest) left 2026-09-29 at companion-technical-gates' scope, on an operator direction lead-relayed (not a /consult ruling), and landed as `installer-install-brevity`.

installer's remaining sections other than §The consumer smoke left 2026-09-29 at companion-adoption-landing's scope, on an operator direction, 2026-09-29, lead-relayed (not a /consult ruling), and landed as `installer-remainder-brevity`, which finishes installer apart from the smoke.

drift-kit's measurement sections left 2026-09-29 at preview-readiness' scope, on an operator direction lead-relayed (not a /consult ruling), and landed as `drift-kit-measurement-brevity`; drift-kit's other sections remain.

guard-kit's front sections left 2026-09-30 at guard-kit-steering's scope, on an operator direction lead-relayed (not a /consult ruling), as [guard-kit-front-brevity](#guard-kit-front-brevity).

### prune-set-matches-walk-root-ancestors

[cost: event/high] [surface: context-kit] [recurrence: 2026-09-25]

the walk's prune set matches path components anywhere in an absolute path, not only below the walk root: `path_pruned` in `native/src/emit/mod.rs` tests `/<leaf>/` against the full path, and the default set carries `target`, `build`, `dist` and `worktrees`. A consumer whose checkout sits under a directory carrying any of those names gets an empty md and pub index, silently.

**Deliverable:** prune relative to the walk root, a fixture whose root path carries a default leaf, and the boundary stated at context-kit/SPEC.md §Layout and configuration.

**Cost while deferred:** an adopter under `~/build/` or `~/dist/` sees the index arms return nothing and no red says why. Filed 2026-09-02; returned from the icebox 2026-09-25 by consult, the walk re-read and the match still absolute.

### tarball-build-attestation

[cost: event/high] [surface: installer] [recurrence: 2026-09-25]

the primary install channel carries no build provenance: the Release tarball ships a digest, which proves transfer, and docs/install.md states the tarball "cannot" carry an attestation where the npm package does. A GitHub artifact attestation on the tarball is a workflow step, not a platform limit.

**Deliverable:** the publish workflow attests the tarball, the shell installers verify it where the verifier is present and say so where it is not, and the install page's claim is corrected; installer/SPEC.md §The dependency boundary owns the rule.

**Cost while deferred:** the channel most adopters take is the one with no provenance, on a project whose pitch is verified claims. Filed 2026-07-26; returned from the icebox 2026-09-25 by consult as a trust gap on a public claim.

### contributor-writeback-disposition

[cost: event/high] [surface: CONTRIBUTING.md] [recurrence: 2026-09-25]

CONTRIBUTING.md promises an inbound issue or pull request a disposition within one iteration, and the scope binding caps each lane at five per iteration; the sixth inbound item is promised what the machine cannot deliver. The "pre-launch, dormant" ground has lapsed: releases are public.

**Deliverable:** either the cap carried on the public promise or a lane that honours it, and the disposition record named; CONTRIBUTING.md and the scope binding agree.

**Cost while deferred:** the first contributor past the cap reads a promise the tree breaks. Filed 2026-07-31; returned from the icebox 2026-09-25 by consult, the promise and the cap re-read.

### nested-battery-env-inheritance-invisible

[cost: event/low] [surface: evidence-kit] [recurrence: 2026-09-25]

a smoke that re-runs the battery inside its sandbox inherits no evidence-kit scoping and reads clean: `installer/consumer-smoke/run-smoke.sh` re-executes batteries in three places with no `EVIDENCE_KIT` reference, so a scoped nested run can record `verdict=clean` for a battery the outer run never scoped.

**Deliverable:** the nested run inherits or refuses the scope, and a fixture pins the refusal.

**Cost while deferred:** a false clean in the evidence record. Filed 2026-08-18; returned from the icebox 2026-09-25 by consult, the smoke re-grepped.

### site-health-probe-no-retry-on-transient

[cost: event/low] [surface: site-kit] [recurrence: 2026-09-25]

the shipped `site-kit/templates/site-health.yml` takes one curl sample and files an issue on a single non-200; a transient is a wrong red on a public tracker. Sibling of [site-health-issue-venue-unwanted](#site-health-issue-venue-unwanted), whose subject is the venue; this one is the sample.

**Deliverable:** a bounded retry before the failure path, in the template and the copy.

**Cost while deferred:** one transient files a public issue. Filed 2026-08-27; returned from the icebox 2026-09-25 by consult, the template re-read.

### release-drain-ordering-contradiction

[cost: event/low] [surface: RELEASING.md] [recurrence: 2026-09-25]

RELEASING.md's step-4 opener bundles the drain and the close stamp as one commit, and the step's body separates them; a public runbook that contradicts itself.

**Deliverable:** the opener rewritten to the body's order.

**Cost while deferred:** a release session follows whichever half it reads first. Filed 2026-08-06; returned from the icebox 2026-09-25 by consult, the step re-read.

### queue-provenance-restates-git-history

[cost: once/low] [surface: TASK-QUEUE.md] [recurrence: 2026-09-25]

queue provenance prose restates what `git log` answers; the ruled sweep is small, ten route-phrase hits remaining when re-counted.

**Deliverable:** the ten sites cut to the fact the entry needs, and the writing rule at queue-kit/SPEC.md §The queue format.

**Cost while deferred:** low; paid by every reader of those entries. Filed 2026-09-09; returned from the icebox 2026-09-25 by consult, the count re-run.

### removed-knob-docs-cmd-valve

[cost: event/low] [surface: canon-kit]

a release note's Renamed-knobs removal bullet must lead with the removed knob backticked (installer/SPEC.md §The upgrade contract), and `check-docs-cmd` assertion B reds a backticked kit-prefixed knob no kit code carries, which a removed knob by definition is. The temporal-exempt path and marker valves do not reach B, so the only valve is the whole-doc `CANON_KIT_MDREF_EXCLUDE`, which also drops the note's md-refs and docs-cmd path checks.

**Deliverable:** a valve that admits a removed knob in its sanctioned release-note position, or B reading the removal bullet's grammar, and the v0.26.0 note taken back off the exclude.

**Cost while deferred:** every release that removes a knob buys a whole-doc exclude, and each excluded note loses its link and path checks for good. Filed 2026-09-26 to the gap inbox at front-door-release's close, which excluded the v0.26.0 note for three removed knobs; promoted 2026-09-26 at the next iteration's scope: →fix fails because the valve's shape is a design call on a shipped gate. Re-verified: `scripts/canon-config.knobs` carries the exclude for that note. Owner lookup: `docs-cmd`, `Renamed-knobs`, `MDREF_EXCLUDE` in this file — only the icebox's [docs-cmd-retired-path-blind-to-queue](#docs-cmd-retired-path-blind-to-queue), which is assertion C; owner canon-kit/SPEC.md §check-docs-cmd.

### crates-reservation-republish

[cost: event/low] [surface: installer]

the crates.io reservation page still carries the retired methodology description, because `reserve/crates/` was regenerated to the product statement but never republished. The republish is a registry write only the operator makes.

**Inferred, not run:** crates.io versions are immutable, so the new description ships only as a new version (`0.0.1`), not over `0.0.0`.

**Deliverable:** the reservation crate republished by the operator, and the live page reading the product statement.

**Cost while deferred:** a reader searching crates.io meets a second product description, the one `one-product-statement` removed everywhere else. Filed 2026-09-27 to the gap inbox by gate-sdk-tail-docs-standard's build; promoted 2026-09-27 at its close: →fix fails because the write is operator-only. Re-verified: the crates.io API returns the retired description at `max_version` 0.0.0. Owner lookup: `crates.io`, `reserve`, `republish` in this file — none; owner installer/SPEC.md §The dependency boundary.

### manual-operation-spend-channel

[cost: iteration/low] [surface: drift-kit]

no measurement channel attributes a session's spend to repeated manual operations, so the close economics pass cannot surface a tooling opportunity. The stage-economics log prices spend per stage and tier, the overhead meter measures always-loaded bytes, prompt-friction sees only Bash calls that prompt (so Edit and Write queue edits are invisible to it), and knowledge-friction is self-reported. `queue-write-side-verb`'s evidence came from an ad-hoc look at `.tmp/`. Operator expectation, 2026-09-27 (lead session): the economics analysis every close runs should surface such opportunities.

**Deliverable:** a channel attributing repeated manual operations (tool-call shapes, hand edits of one surface) to sessions, and a close economics read listing the top candidates.

**Cost while deferred:** tooling gaps surface only by chance and repeated manual work stays unpriced. Filed 2026-09-27 to the gap inbox by platform-prerequisite-floors' lead; promoted at its close: →fix fails because the channel is new mechanism. Re-verified: `.workflow/knowledge-friction.log` holds no line at this close. Owner lookup: `economics`, `tooling opportunit`, `repeated` in this file — [build-stage-tier-economics](#build-stage-tier-economics) and [queue-tier-label-correction-cost](#queue-tier-label-correction-cost), DISTINCT (tier pricing); owner drift-kit/SPEC.md §The stage-economics meter.

### custom-gate-substrates

[cost: event/high] [surface: gate-sdk]

an adopter writes a custom gate in shell only. The registry resolves a member as a `.sh` or a `.gate` declaration (gate-sdk/SPEC.md §lib/gate.sh), and a `.gate` descriptor dispatches into the published binary, which an install cannot extend. On native Windows an adopter therefore needs Git for Windows' bash to author a gate, although the installer and the PowerShell front end already run under PowerShell; operator direction, 2026-09-27 (lead session): customers may write gates in shell, but on Windows they should be able to write them in PowerShell. And there is no supported path to a Rust gate, while a fork that adds the subcommand and ships its own build can (docs/install.md §Writing your own Rust gates; operator direction, 2026-09-27, lead session). Custom Rust gates are wanted as a capability, with install prerequisites split between shipped native gates, custom shell gates and custom Rust gates (operator direction, 2026-09-27, lead session).

**Deliverable:** the registry resolving further substrates under the output, fail-closed, fixture-pair and self-lint contracts: `.ps1` resolution in the registry and the runner, which the hook's `--git-hook` arm dispatches through, with a PowerShell lint counterpart to `check-shellcheck`; a Rust path, as an adopter-built executable the registry dispatches or an extension crate; and the install page's prerequisites per substrate. Whether one executable-dispatch shape serves both is spec's.

**Cost while deferred:** a native-Windows adopter authoring a gate takes on a bash dependency and a second shell dialect, and one wanting a typed, testable gate must write shell or fork. Filed 2026-09-27 to the gap inbox as two bullets by platform-prerequisite-floors' lead, merged here at its close because both ask which substrates the registry resolves beyond `.sh` and `.gate`: →fix fails because each substrate is new mechanism. The Rust bullet's premise, that an adopter cannot write a Rust gate at all, is corrected to the relayed direction above. Re-verified: `registry::resolve` tries `sh` then `gate` per dir and nothing else. Owner lookup: `ps1`, `PowerShell`, `custom gate`, `Rust gate`, `consumer crate` in this file — none; owner gate-sdk/SPEC.md §lib/gate.sh and §The port-candidate criteria.

### doctor-shell-gate-bash

[cost: event/low] [surface: context-kit]

`doctor` does not owe `bash` for a consumer-registered shell gate. `bash`'s audience is derived over the kit roots and the anchor's fence-run corpus only (context-kit/SPEC.md §bin/env-probe), and a registered name with no `REGISTRY` row contributes nothing, so a native-Windows adopter who writes a shell gate without Git for Windows' bash reads `DOCTOR: clean` and meets the battery's exit 2 instead.

**Deliverable:** a fourth derivation arm owing `bash` where the anchor's `gates.list` registers a member resolving to a `.sh` declaration in the gates dir, with its fixture.

**Cost while deferred:** the requirement is stated on the install page only, and doctor's clean verdict misleads an adopter who skipped it. Filed 2026-09-27 to the gap inbox by platform-prerequisite-floors' build; promoted at its close: →fix fails because the arm widens doctor's verdict, adopter-visible semantics no amendment settled. Re-verified: the section's bash audience names three arms, two per kit root and one over the fence-run corpus, none reading a registered `.sh` member; docs/install.md's Windows section states the requirement. Owner lookup: `doctor`, `audience`, `env-probe` in this file — only [heterogeneous-agent-delegation](#heterogeneous-agent-delegation)'s CLI probe, DISTINCT; owner context-kit/SPEC.md §bin/env-probe. Related: [custom-gate-substrates](#custom-gate-substrates).

### release-note-section-set-derivation

[cost: event/high] [surface: installer] [recurrence: 2026-09-27]

release notes serve human upgraders poorly, and their section set is crate literals. A note (installer/SPEC.md §The upgrade contract) has one human section, In brief, then declaration-bearing sections a mechanical consumer reconciles; the v0.26.0 note is 34,527 words against v0.25.0's 1,058. The section names are string literals in the release gates, the declaration parser and the upgrade smoke, so a rename is a crate edit, a new section is added by copying a call, and no gate asserts the gates' set equals the page's. Operator directions, 2026-09-27 (lead session): add sections for adopters with custom gates, whose reconcile differs; put a summary table at the top linking to the detail; reconsider the names, since "Tightened gates" also holds new gates, "Renamed knobs" also carries removals and new knobs have no section; and no hard code in gates. An operator question the same day asked where new platform support goes: this iteration's musl switch was declared as two Behavior changes bullets.

**Deliverable:** a note structure with a linked summary table, audience-keyed sections and a Platforms section derived from the diff of docs/install.md's gated platforms table between two releases, its section set one knob-owned roster every reader derives from. The lead tokens are machine-read over a historical corpus the upgrade smoke resolves at any FROM/TO, so a rename owes an alias window or a note-corpus migration.

**Cost while deferred:** every release ships a note a human must read whole to act on, custom-gate adopters are not told what applies to them, and a platform change hides in Behavior changes. Filed 2026-08-01 at close; iceboxed 2026-09-11 as dormant; re-filed 2026-09-27 to the gap inbox as two bullets (readability, a Platforms section) by platform-prerequisite-floors' lead, and returned at its close on a judged recurrence: →fix fails because the redesign renames machine-read tokens. Re-verified: `wc -w` over the two notes, and the literals in `native/src/gates/release_bump.rs`, `tightened_gates_grammar.rs`, `release_change_declared.rs`, `native/src/declaration.rs` and `native/src/emit/upgrade_smoke.rs`. [removed-knob-docs-cmd-valve](#removed-knob-docs-cmd-valve) is DISTINCT. Owner: installer/SPEC.md §The upgrade contract, RELEASING.md.

### consumer-value-literal-gate

[cost: event/low] [surface: gate-sdk]

nothing enforces gate-sdk/SPEC.md §The port-candidate criteria's rule that every consumer value a member reads is a knob with one producer, never a crate literal. Instances surface by hand: `installer-graph-artifact-literal`, landed this iteration, and the release-note section names in [release-note-section-set-derivation](#release-note-section-set-derivation). Operator direction, 2026-09-27 (lead session): avoid any hard code in gates, because checkwright offers configurable gate templates to customers and benefits from them itself.

**Deliverable:** an audit sizing the consumer-value literal population in `native/src/gates`, then a gate or lint over it with a declared valve.

**Cost while deferred:** each literal publishes this repo's configuration as every adopter's mechanism, and instances surface only by chance. Filed 2026-09-27 to the gap inbox by platform-prerequisite-floors' lead; promoted at its close: →fix fails because the gate is new mechanism over an unsized corpus. Re-verified: the rule's sentence appears in gate-sdk/SPEC.md and its site mirror alone, with no gate section enforcing it. Owner lookup: `literal`, `hardcod` in this file's headings — [knob-default-accessor-singularity](#knob-default-accessor-singularity), DISTINCT (it bars re-spelling an existing knob's default; this bars a consumer value with no knob); owner gate-sdk/SPEC.md §The port-candidate criteria.

### spec-mirror-citation-links

[cost: event/low] [surface: canon-kit]

the kit SPECs' on-site mirrors render their section citations as plain text, so a reader finds each cited section by hand and a path-less citation stays liveness-gated only. The docs-mirror arm (`native/src/emit/docs_mirror.rs`) rewrites link targets and nothing else.

**Deliverable:** citation rendering at mirror time in that arm, each citation resolved through `check-spec-pointer`'s resolver into a relative link to the mirrored section, so every citation is reached with no hand conversion and no drift.

**Cost while deferred:** every site reader of a SPEC follows its citations by hand. Filed 2026-09-27 to the gap inbox at companion-catalog-extension's spec, split from `docs-ux-authoring-rules`, whose range was the hand-authored pages and READMEs; promoted 2026-09-27 at its close: →fix fails because the rendering is new mechanism in a shipped arm. Re-verified: the scope survey's oracle over the thirteen tracked `*/SPEC.md` counts 2,157 citation lines outside fences at the close, and the arm's `rewrite_line` rewrites `](` targets only. Owner lookup: `docs_mirror`, `mirror`, `citation` in this file — none; owner canon-kit/SPEC.md §The reference-link grammar.

### hotfix-agent-definition

[cost: event/low] [surface: delegation-kit]

no tracked agent definition exists for the operator-ruled hotfix path of the scope-gated intake rule. A lead dispatching one picks general-purpose and restates standing policy in the prompt (not a stage, no stage entry, no queue or state writes, one test-and-doc-complete commit, the battery and every reached kit suite, stop on a design question, the gap-inbox disposition): the policy-is-config tell (lifecycle-kit/templates/lead.md §Policy is config, not prose). `agent-dispatch-guard` confines an undeclared type to a worktree, where a crate-source commit is refused and `--emit file-gap` refuses, so the 2026-09-27 front-door hotfix needed a second dispatch to land.

**Deliverable:** a hotfix agent definition under `.claude/agents/` carrying that policy, declared in `DELEGATION_KIT_MUTATING_TYPES`, and the lead template naming the dispatch shape.

**Cost while deferred:** every hotfix costs a restated prompt and a second dispatch. Filed 2026-09-27 to the gap inbox by companion-catalog-extension's lead; promoted 2026-09-27 at its close: →fix fails because the definition is a new governed name. Owner lookup: `hotfix`, `agents/`, `MUTATING_TYPES` in this file — none; owner delegation-kit/SPEC.md. The worktree half landed as `worktree-crate-commit-red`: the hook greens in a worktree, and `check-crate-arms` refusing a crate-source commit there is now the stated contract.

### readme-spec-links-offsite

[cost: event/low] [surface: gate-sdk]

a kit README's citation of another kit's SPEC section links off-site to the GitHub blob, since `check-packed-links` reds a relative `../<kit>/SPEC.md` link (the payload withholds every SPEC) and the packer rewrites only a README's own SPEC link; so its docs mirror sends readers to GitHub where an on-site mirror of that SPEC exists.

**Deliverable:** the pack-time rewrite widened to `../<leaf>/SPEC.md[#frag]` for a packed leaf, giving on-site targets on the mirror and published ones in the payload.

**Cost while deferred:** about fourteen README citations leave the site. Filed 2026-09-27 to the gap inbox by companion-catalog-extension's docs-ux build; promoted 2026-09-27 at its close: →fix fails because the rewrite bound is a packer contract change. Re-verified: 14 blob-SPEC link lines across the twelve top-level READMEs. Owner lookup: `packed-links`, `blob`, `rewrite` in this file — none; owner gate-sdk/SPEC.md §check-packed-links. Related: [spec-mirror-citation-links](#spec-mirror-citation-links).

### citation-link-root-docs-range

[cost: event/low] [surface: canon-kit]

`check-citation-link`'s `CANON_KIT_CITATION_LINK_PAGES` binding (`docs/*.md`, `docs/*/index.md`, `*/README.md`, `README.md`) leaves out the other root docs GitHub renders; TRAJECTORY.md carries plain section citations, one written unlinked by the docs-ux build batch itself. With it, each ruling could be anchored by a header whose slug is its primary ruling name, so a citation links `TRAJECTORY.md#<name>` as queue slugs do; no tracked file cites that file by anchor today, citers using the ruling name the ruling-staleness probe sweeps.

**Deliverable:** the binding widened to GitHub-rendered root docs (TRAJECTORY.md, ROADMAP.md and the like), excluding agent-loaded surfaces such as CLAUDE.md where links grow the always-loaded meter; and the ruling-header option costed with it, never as its own unit (operator direction, 2026-09-27, lead session). Against the headers: the record format is lifecycle-kit's, a ruling may declare several names while a header carries one slug, and the file shrinks toward empty.

**Cost while deferred:** a root-doc reader hunts cited sections by hand. Filed 2026-09-27 to the gap inbox by companion-catalog-extension's lead, on an operator direction to file rather than widen mid-iteration, with an addendum; promoted 2026-09-27 at its close: →fix fails because the widening is a ruled deferral. Re-verified: `scripts/canon-config.knobs` binds the four globs, and TRAJECTORY.md lines 4, 12 and 18 carry `§`. Owner lookup: `citation-link`, `CITATION_LINK_PAGES`, `TRAJECTORY.md#` in this file — none; owner canon-kit/SPEC.md §check-citation-link, with lifecycle-kit/SPEC.md §The ruling-staleness probe.

### site-video-poster-rule

[cost: event/low] [surface: site-kit]

the site has no rule for video, and the operator wants short intro and demo videos (direction 2026-09-27, the implementation delegated to the lead). Lead decision: host them on the reserved YouTube channel, a discovery channel fitting `catalog-then-plugin`'s distribution grounds that keeps video bytes out of every clone; a page shows a local poster image linking out, never an iframe player, so the site keeps zero third-party requests at page load.

**Deliverable:** a page-authoring rule in docs/site-architecture.md admitting only the poster-link form, a gate reding an `<iframe>` or a third-party `src=` in docs pages, and first homes on docs/spec-toolkits.md and the front door.

**Cost while deferred:** a video lands with no rule, and an embed would add the site's first third-party request. Filed 2026-09-27 to the gap inbox by companion-catalog-extension's lead; promoted 2026-09-27 at its close: →fix fails because the gate is new mechanism. Re-verified: `docs/_layouts/default.html` loads only local assets, and `check-docs-render-fidelity` lists `iframe` as a known tag without refusing it. Owner lookup: `video`, `iframe`, `YouTube` in this file — none; owner docs/site-architecture.md §Page-authoring rules.

### intake-routing-test

[cost: event/low] [surface: doctrine-kit]

no guidance routes a mid-iteration operator request into the current iteration. doctrine-kit/DOCTRINE.md rule 11 (Scope-gated intake) names two entries, a Deferred filing through scope and an operator-ruled hotfix. Close's gap-inbox drain tries →fix first, a third entry rule 11 does not name, and lifecycle-kit/templates/lead.md has no route for an operator's "add X to this iteration". Observed 2026-09-28: a lead offered close's drain for a copy change, then on the operator's "add to this iteration" switched without a stated reason to an unsanctioned build batch writing its own queue entry, and the operator steered it back.

**Deliverable:** a routing test over {defer to scope, hotfix, gap fixed at close's drain, refuse to the next iteration} in the lead template, and rule 11 naming the drain route.

**Cost while deferred:** a correct route is abandoned on generic operator wording, each time a lead meets one. Filed 2026-09-28 to the gap inbox by plugin-marketplace-queue-verbs' lead; promoted at its close: →fix fails because rule 11 itself makes amending the doctrine a scoped unit. Re-verified: rule 11 names the two entries and no other; lead.md carries no intake route. Owner lookup: `intake`, `Scope-gated`, `hotfix` in this file — [hotfix-agent-definition](#hotfix-agent-definition), DISTINCT (the hotfix path's agent type, not the routing); owner doctrine-kit/DOCTRINE.md, with lifecycle-kit/templates/lead.md.

### plugin-harness-reach

[cost: event/low] [surface: plugin]

the harness plugin package reaches Claude Code only in its tested and guarded parts. plugin/SPEC.md reads the portable `plugin.json` and `skills/` as loading on any Agent Plugins 1.0 client off the standard alone, since only Claude Code's install is run; the guards ride `hooks/hooks.json`, outside the standard's portable core, so no other client runs them; the marketplace file is Claude Code's format. No queue entry names another harness. **Inferred, not run:** which harnesses adopt Agent Plugins 1.0. **Operator-supplied, unverified:** Meta's coding harness "seems to be named Muse Code". **Harness priority, operator direction, 2026-09-28:** tier 1 Claude Code and Codex; tier 2 Muse Code, Antigravity and Cursor.

**Deliverable:** (1) the skills load verified on the codex and Antigravity CLIs; (2) a survey of which harnesses read Agent Plugins 1.0 and their catalogs; (3) per-harness guard wiring where a harness has a hook surface.

**Cost while deferred:** an adopter on another harness gets unverified skills and no guards. Filed 2026-09-28 to the gap inbox by plugin-marketplace-queue-verbs' lead on an operator direction to file it for promotion; promoted at its close. Re-verified: plugin/SPEC.md states the portable core and that only Claude Code's install is run. Owner lookup: `Agent Plugins`, `codex`, `Cursor`, `Antigravity` in this file — none; owner plugin/SPEC.md.

### verify-workflow-decoupling

[cost: event/low] [surface: installer]

the operator states that verification and workflow are fully decoupled, each shippable without the other; the tree shows one direction only. Verification without the workflow holds: `installer/profiles.list`'s starter profile is gate-sdk alone and prose is gate-sdk plus canon-kit, and the companion recipes gate another toolkit's workflow. The workflow without verification is not shipped: every profile carrying lifecycle-kit carries gate-sdk, forced in because without it there is no runner, hook generator or registry; `--enter-stage` is an arm of the gate binary; the delegation profile's comment reads "Vendored is not yet enforced"; docs/kits.md orders the roster as "each kit assumes the machinery of the ones above it". The operator's stated aim (2026-09-28, lead session; an aim, not a /consult objective): a competitive offering that is loosely coupled, composable, configurable and efficient.

**Deliverable:** a ruling on whether the gate binary is verification or substrate both halves share; if the decoupling then holds, the front door and docs/kits.md state it, and a workflow-only profile is weighed.

**Cost while deferred:** the front door neither claims nor refutes a decoupling the operator believes in, and docs/kits.md reads as a dependency chain. Filed 2026-09-28 to the gap inbox by plugin-marketplace-queue-verbs' lead; promoted at its close: →fix fails because the classification is a design ruling with no direction behind it. Re-verified: the three profiles' rows and comments in `installer/profiles.list`, and docs/kits.md line 13. Owner lookup: `decoupl`, `profiles.list`, `workflow-only` in this file — none; owner installer/SPEC.md §Profiles, with docs/kits.md.

### plugin-front-matter-yaml

[cost: event/low] [surface: plugin]

`check-plugin-parity`'s front-matter reader (`front_matter` in `native/src/gates/plugin_parity.rs`) splits each line at its first `:` and strips one quote pair, so it admits a `SKILL.md` description that strict YAML rejects: an unquoted `: ` in agent-execution's description passed the local battery and the harness CLI and redded only CI's `skills-ref` step on the v0.28.0 stamp push. plugin/SPEC.md §The validation leg rules manifest validity the leg's, and nothing runs that leg's oracles before a push.

**Deliverable:** the leg's oracles runnable locally before a package change commits, or the reader refusing what a YAML plain scalar cannot carry, with a `bad/` fixture holding the `: ` case.

**Cost while deferred:** a front-matter slip is caught only by a push, which spends a hotfix from the budget. Filed 2026-09-28 to the gap inbox by plugin-marketplace-queue-verbs' close; promoted 2026-09-28 at the next iteration's scope: →fix fails because a stricter reader changes a shipped gate's verdict. Re-verified: the reader above, and `.github/workflows/gates.yml` installs `skills-ref` in CI only. Owner lookup: `front matter`, `skills-ref`, `yaml` in this file — only the icebox's template-copy-parity-yaml-widening, DISTINCT (template copies); owner plugin/SPEC.md §check-plugin-parity.

### positional-reference-rule

[cost: event/low] [surface: canon-kit]

positional references to a block go stale like restated counts, and nothing catches them. README.md said "The last line, `--run-demo`, is the adoption walkthrough" after the generated battery-roster block gained a later line; docs/install.md carried two more against its hand-authored recipes. All three were rewritten at companion-front-door-widening's close to name the referent. `check-manifest-count` gates cardinals only, and canon-kit/SPEC.md §check-amendment-retired-spelling records the renumber slice as undecidable, its durable fix being to name the referent rather than its position; no doctrine line states that for first/last/above/below prose. Operator question, 2026-09-28 (lead session): why are positional qualifiers allowed when restated counts are blocked.

**Deliverable:** doctrine-kit/DOCTRINE.md's De-literalization or Derivation-first rule naming positional references beside counts; a narrow gate weighed, a positional word citing a marked or generated block, since a general "last line" matcher would cry wolf.

**Cost while deferred:** the next block edit strands a positional sentence silently. Filed 2026-09-28 to the gap inbox by companion-front-door-widening's lead; the three instances fixed at its close, the rule and gate promoted: →fix fails because a doctrine rule change is a scoped unit (the reasoning [intake-routing-test](#intake-routing-test) records) and the gate is new mechanism. Re-verified: the battery-roster block ended at `--projection-witness`. Owner lookup: `positional`, `ordinal`, `renumber` in this file — only the icebox's doctrine-rule-number-citation-liveness, DISTINCT (numbered doctrine-rule citations); owner doctrine-kit/DOCTRINE.md, with canon-kit/SPEC.md for any gate arm.

### gate-customer-value-audit

[roadmap: next/adoption] [cost: iteration/high] [surface: gate-sdk] [roadmap-summary: Every installed gate useful to its adopter and configurable, and no gate shipped that serves only Checkwright's own development.]

nothing asks whether each shipped gate is useful and configurable for a customer. Operator direction, 2026-09-28 (lead session): "offer customers useful and configurable gates"; a gate benefiting only Checkwright's own development, such as a GitHub Pages check that is not configurable, "should either be abstracted or not be shipped and tested on customer OSes". The provenance seam bars project content from kits, but customer value is another question, and the install disposition (zero-config, on-surface, never) decides registration, not whether a gate belongs in the payload. Horizon `next`, operator direction 2026-09-29, lead-relayed (not a ruling): `now` if the next scope takes it, else at the first close landing a slice; the per-kit slices stay off the roadmap.

**Deliverable:** an audit over every shipped gate recording customer value and configurability, each gate then made generic, kept out of the payload and the customer-OS legs, or kept; site-kit's GitHub Pages gates first.

**Cost while deferred:** every push runs gates of no customer value on customer-OS legs, and every adopter installs them. Filed 2026-09-28 to the gap inbox by companion-front-door-widening's lead; promoted 2026-09-28 at the next iteration's scope: →fix fails because the audit is unsized. Owner lookup: `customer value`, `payload` in this file — [consumer-value-literal-gate](#consumer-value-literal-gate), DISTINCT (a hard-coded value inside a gate, not the gate's place in the payload). Owner gate-sdk/SPEC.md §Consumer payload.

**First slice landed 2026-09-29 at native-hook-customer-legs, operator direction lead-relayed (not a /consult ruling):** the payload rule at gate-sdk/SPEC.md §Consumer payload, and site-kit audited against it.

Audited: site-kit, 5 gates (5 kept, 2 made generic, 0 withheld); the other 118 shipped gates remain, kit by kit, under gate-sdk/SPEC.md §Consumer payload's rule.

guard-kit's slice left 2026-09-30 at guard-kit-steering's scope, on an operator direction lead-relayed (not a /consult ruling), as [guard-kit-value-audit](#guard-kit-value-audit).

**A precondition of [companion-toolkit-profile](#companion-toolkit-profile)'s submission, operator direction 2026-09-29, lead-relayed (not a ruling),** beside [companion-install-tier](#companion-install-tier), whose exposed kits this audit's verdicts decide, so those kits are its natural first slices.

### compiled-consumer-smoke-driver

[cost: iteration/high] [surface: installer]

the consumer smoke has two drivers. native-hook-customer-legs chose a PowerShell driver over the entry's five coverage classes, so installer/consumer-smoke/run-smoke.sh and run-smoke.ps1 both spell init, battery, hooks, upgrade and uninstall, and the arms installer/SPEC.md §The consumer smoke names as staying on the unix legs do not run on native Windows.

**Deliverable:** one compiled driver on every leg, replacing both scripts.

**Cost while deferred:** two drivers held in step by nobody, and Windows loses those arms. Filed 2026-09-29 to the gap inbox at native-hook-customer-legs' spec; promoted 2026-09-29 at its close: →fix fails because the driver is new mechanism, →forward because no ruling is owed. Re-verified: both scripts exist (2015 and 232 lines). Owner lookup: `consumer smoke`, `run-smoke.ps1`, `smoke driver` in this file — none; owner installer/SPEC.md §The consumer smoke.

### native-executable-git-hooks

[cost: event/high] [surface: gate-sdk]

the generated pre-commit and commit-msg hooks start through a shell on every OS: each is two lines of POSIX sh that exec the gate binary, and on Windows git needs Git for Windows' bundled sh to start it, emulated on Arm (the delta-6 probe, CI Windows x64 and arm64, 2026-09-29). Operator direction, 2026-09-29, lead-relayed (not a ruling): explore a hook git starts as a native executable, one shape on every OS, never a Windows-only exception.

**Deliverable:** (1) a probe of whether git's hook lookup starts a native executable directly on each OS (the Windows `.exe` lookup inferred from git's source, never run); (2) where a native hook lives, since a per-platform binary cannot be the tracked text hook scripts/git-hooks/ holds: hooks installed untracked with the generated-projection contract and its freshness gate re-pointed at the installer, or a tracked shim kept, which is the shell this removes; (3) the per-commit start cost per OS.

**Cost while deferred:** every commit on Windows starts an emulated-or-bundled shell. Filed 2026-09-29 to the gap inbox by native-hook-customer-legs' lead; promoted 2026-09-29 at its close: →fix fails because the probe is unrun and the home undecided, →forward because the direction is given. Re-verified: scripts/git-hooks/pre-commit opens `#!/bin/sh`. Not a recurrence of `native-hook-dispatch`, which removed the bash dependency. Owner lookup: `native executable`, `hook shim` in this file — none; owner gate-sdk/SPEC.md, with installer/SPEC.md for an untracked install.

### tier-model-binding

[cost: event/low] [surface: delegation-kit]

the tier a dispatch rides is written as a harness model alias wherever it is chosen (the `model:` field of the stage-session, consult-session, audit-sweep and edit-sweep agent definitions, the lead binding's per-stage overrides, the build batch tiering), and no consumer setting chooses between following the newest model and pinning one. On 2026-09-29 the alias dispatches of native-hook-customer-legs resolved to two newer model ids with no edit, per the subagent transcripts.

**Deliverable, operator direction 2026-09-29, lead-relayed (not a ruling):** (1) a consumer binding per tier class (judgment, routing, mechanical) to an alias, which follows the newest model, or an exact model id, which stays until changed; alias the default and pinning the opt-in, per Policy-as-choice; every place that chooses a tier reads the one binding. (2) Public documentation of alias tiering as a strength with its honest limit: an alias upgrade silently changes price and behaviour, so the claim ships with the price-coverage arm's detector (drift-kit/SPEC.md §The price-coverage arm), and under a pin a new id in the transcripts means the pin was bypassed. **Inferred, not run:** that the harness accepts an exact model id wherever it accepts an alias.

**Cost while deferred:** a model upgrade changes every dispatch's price and behaviour unannounced, and no consumer can opt out. Filed 2026-09-29 to the gap inbox by native-hook-customer-legs' lead; promoted 2026-09-29 at the next iteration's scope: →fix fails because the binding is a new knob. Re-verified: `model: opus` or `model: sonnet` in the four agent definitions. Owner lookup: `alias`, `model id`, `tier` in this file — [session-model-identity-verification](#session-model-identity-verification), DISTINCT (verifying the running tier, not choosing it); owner delegation-kit/SPEC.md, with delegation-kit/templates/agent-execution.md's live-roster rule.

### openspec-delta-base-agreement

[cost: event/low] [surface: companion]

an OpenSpec change delta that disagrees with its base spec passes the battery and OpenSpec's own validator, and is caught, if at all, only at archive. Measured on openspec 1.13.2 at companion-technical-gates' spec: `validate --strict` exits 0 on a MODIFIED or RENAMED delta naming an absent requirement and on an ADDED one naming an existing requirement, printing only an INFO line; `archive -y` refuses those three, but `--skip-specs` bypasses the refusal; a REMOVED delta naming an absent requirement passes validate silently and archive takes it as already removed.

**Deliverable:** a generic commit-time gate for heading-set delta agreement, its OpenSpec binding in the recipe, if it clears the value bar `toolkit-overlap-value-bar` landed (companion/SPEC.md §The two tiers); it is that bar's first candidate, since OpenSpec owns the check at archive and this gate would re-check it earlier.

**Cost while deferred:** an OpenSpec adopter's technical gates read nothing on a conventional task list, which names no paths, so the companion's code-facing reach there is `check-task-path-claim` alone. Filed 2026-09-29 at companion-technical-gates' spec on an operator direction lead-relayed (not a ruling). Owner lookup: `delta`, `archive`, `openspec` in this file — `companion-spec-to-code-gates`, DISTINCT (it ships the task gates and defers this one), and `toolkit-nav-hierarchy`, DISTINCT (nav labels); owner companion/SPEC.md.

### front-door-flag-operand

[cost: event/low] [surface: installer]

`check-front-door-verbs` reads an advertised verb and each flag after it, never a flag's value, so a recipe or profile name the pinned release lacks after a flag it carries reds nothing and fires no release trigger (installer/SPEC.md §The front door's verbs, *Honest limits*).

**Deliverable:** invariant B also reads the operand of `--recipe` and `--profile` against the pinned tag's recipes and `installer/profiles.list`, under the same pending admission.

**Cost while deferred:** a new toolkit page can advertise a recipe the one-liner refuses until an unforced release. Filed 2026-09-29 to the gap inbox at companion-technical-gates' spec; promoted at its close: →fix fails because the assertion is new mechanism on a shipped gate, →forward because no ruling is owed. Re-verified: the honest-limit sentence names a flag's value. Owner lookup: `operand`, `flag's value` in this file — none; owner installer/SPEC.md §The front door's verbs.

### install-smoke-sh-matrix

[cost: event/low] [surface: .github]

`.github/workflows/gates.yml` spells three unix install-smoke legs as hand-copied jobs, `install-smoke-sh-macos`, `install-smoke-sh-macos-intel` and `install-smoke-sh-linux-arm64`, while every other derived job group is a matrix. The two macOS legs differ only in triple, one `uname -m` probe line and log strings; the arm64 leg is the same shape less the macOS remedy step. The roster step already emits `unix_legs`, read only by `crate-tests-unix`.

**Deliverable:** one install-smoke-sh matrix over `unix_legs` less the baseline triple, the remedy step conditioned on the runner OS being macOS; `install-smoke-sh-linux` stays its own job for its baseline diff and foreign-host containers. **Inferred, not probed:** that nothing reds a declared triple with no smoke leg.

**Cost while deferred:** triplicated YAML held in step by nobody, and a new unix triple needs a hand-written smoke job. Filed 2026-09-29 to the gap inbox by companion-technical-gates' lead on an operator question; promoted at its close: →fix fails because the rewrite needs a push to witness, →forward because no ruling is owed. Re-verified: the three job keys and the `unix_legs` output in gates.yml. Owner lookup: `install-smoke-sh`, `unix_legs` in this file — none; owner installer/SPEC.md §The consumer smoke.

### releases-page-table

[cost: event/low] [surface: docs]

docs/releases.md renders its derived note list as a bare list of version links, which repeats the nav: the page carries `nav_children_key: release`, so the nav already lists every note. Operator question, 2026-09-29, lead-relayed: a stats table instead.

**Deliverable:** a derived table whose columns answer an upgrader, above all whether a release needs action on upgrade (it carries Tightened gates or Renamed knobs entries); also version, date, bump class and per-section counts. The note composer writes the counts and the action flag as front-matter keys and a gate holds them equal to the note's sections; parsing sections in Liquid at render time is refused as fragile.

**Cost while deferred:** a reader skipping several versions opens each note to learn which need action. Filed 2026-09-29 to the gap inbox by companion-technical-gates' lead, at the operator's leave; promoted at its close: →fix fails because the keys are new grammar on the notes, →forward because no ruling is owed. Re-verified: the page's `<ul>` loop and 29 notes carrying `release:`. Owner lookup: `releases.md`, `front-matter` in this file — none; owner docs/site-architecture.md, with installer/SPEC.md §The upgrade contract for the keys.

### consult-intake-narrowing

[cost: event/high] [surface: lifecycle-kit]

the consult inbox takes more than the operator's model of consult. Operator direction, 2026-09-29, lead-relayed (not a ruling): consult is for (a) reconsidering a ruling and (b) an expert opinion from a superior model on strategic direction, such as architecture. lifecycle-kit/SPEC.md §The consult inbox also admits an operator-class finding a session cannot relay live, a threshold entry declined twice, a misnamed direction question and a stage's operator signal; the catch-alls twice pulled in a question the operator had already answered as a direction (the operator-seat install item, re-classed at 30a5343b; the npm-approval item, discarded at 8ea1233c).

**Deliverable:** the producer list narrowed to (a) and (b), the other classes routed to the gap inbox or a live lead; templates/consult.md (the operator rules) reconciled with (b)'s advisory reading; whether (b) pins consult's tier, where [consult-tier-declaration](#consult-tier-declaration) asserts the tier consult dispatches; and whether a lead-dispatched consultation with no operator, which can only re-class or discard, still earns a dispatch.

**Cost while deferred:** each misfiled item costs a consultation to route it back out. Filed 2026-09-29 to the gap inbox by companion-adoption-landing's lead; promoted at its close: →fix fails because narrowing a producer set is a contract change to a SPEC section, →forward because the direction is given and filing it to consult is the misfile it names. Re-verified: §The consult inbox's Producers paragraph and opening list. Owner lookup: `consult inbox`, `intake` in this file — [consult-tier-declaration](#consult-tier-declaration), DISTINCT (the tier consult runs at, not what enters it); owner lifecycle-kit/SPEC.md §The consult inbox.

### user-facing-delta-unsurfaced

[cost: event/low] [surface: lifecycle-kit]

a user-facing choice can ride a `{mechanical}` delta and reach no operator: the payload-withholding amendment's delta 11 hid the SPEC mirrors from the site nav's kit suffix, and `nav-spec-suffix-restore` reversed it at companion-adoption-landing's build. The work-class tag measures the execution judgment a delta demands (lifecycle-kit/templates/stages/spec.md, Label every delta), not whether it changes what a reader meets.

**Deliverable:** a spec-stage rule that a delta changing user-facing semantics, the site's reach or labels included, is surfaced in the amendment's rulings, and align's check of it; or a boundary note refusing both.

**Cost while deferred:** the next such choice lands unreviewed and costs a later reversal unit. Filed 2026-09-29 to the gap inbox by companion-adoption-landing's build; promoted at its close: →fix fails because a stage rule is new kit mechanism, →forward because no ruling is owed. Re-verified: spec.md's Label every delta defines the tag by execution judgment. Owner lookup: `work-class`, `user-facing`, `mechanical` in this file — the icebox's amendment-work-class-label-placement, DISTINCT (where the tag sits, not what it misses); owner lifecycle-kit/templates/stages/spec.md.

### docs-chrome-page-repeat

[cost: event/low] [surface: docs]

`check-docs-page-repeat` reads page sources and never `docs/_layouts` or `docs/_includes`, so a chrome addition repeating a page's statement passes. Found at `homepage-license-duplicate`, where the footer's license line duplicated docs/index.md's License section; that unit's `check-license-line` widening holds the license instance only.

**Deliverable:** an arm prefixing the layout's and includes' literal text nodes (Liquid excluded) to every page's corpus, so a sentence of eight words or more or a link target stated in both reds; or a boundary note refusing it.

**Cost while deferred:** the next chrome addition can duplicate a page statement unseen until a reader finds it. Filed 2026-09-29 to the gap inbox by companion-adoption-landing's build; promoted at its close: →fix fails because the arm is new mechanism with a false-positive risk to calibrate, →forward because no ruling is owed. Re-verified: canon-kit/SPEC.md §check-docs-page-repeat reads each `CANON_KIT_PAGE_REPEAT_PAGES` page alone and deliberately asserts no repeat across pages, so the arm must weigh that boundary. Owner lookup: `page-repeat`, `_layouts`, `chrome` in this file — [site-video-poster-rule](#site-video-poster-rule), DISTINCT (embeds); owner canon-kit/SPEC.md §check-docs-page-repeat, with docs/site-architecture.md §Page-authoring rules. Surface also canon-kit.

### skill-binding-couples-drift

[cost: event/low] [surface: lifecycle-kit]

`check-skill-binding` couples each out-of-tree bound template by name, and nothing checks the list against the templates the shims bind: `drift-kit/templates/economics.md` was absent until companion-adoption-landing's close, and `gate-sdk/templates/adopt.md` was added by hand at its spec.

**Deliverable:** an assertion that every template a shim under `LIFECYCLE_KIT_SKILLS_DIR` binds matches a `couples=` member of the gate's own descriptor, with a fixture pair; or a boundary note in lifecycle-kit/SPEC.md §check-skill-binding refusing it.

**Cost while deferred:** a slot added to an uncoupled bound template fires nothing at commit and surfaces one tier late. Filed 2026-09-29 at companion-adoption-landing's close as the drain's gap generalization for the economics omission it fixed. Owner lookup: `skill-binding`, `couples` in this file — none; owner lifecycle-kit/SPEC.md §check-skill-binding.

### companion-install-tier

[roadmap: next/ecosystem] [cost: event/high] [surface: companion] [roadmap-summary: A companion install exposing every deterministic gate a spec toolkit does not already do.]

a companion install tier, operator direction 2026-09-29, lead-relayed (not a ruling). A Spec Kit or OpenSpec adopter installs a profile exposing every kit's deterministic gates except the kits whose job the toolkit already does (candidates, unmeasured: lifecycle-kit's stage machine, queue-kit's task queue), each exclusion recorded with the toolkit it defers to, so adopting an excluded kit later is the path to replacing that piece. Each toolkit's customers get a recipe for adopting the non-conflicting kits, in the toolkit's extension where one exists and at minimum on its docs page (OpenSpec has none today). Ground: the companion brings those toolkits' customers to deterministic checks, which complement the toolkit's own work.

**Deliverable:** the tier, its exclusion record and the per-toolkit recipes, under the companion arm's tested-line rule (companion/SPEC.md §Applying a recipe).

**Horizon, operator direction 2026-09-29, lead-relayed (not a ruling):** one outcome row, `now` if the next scope takes it, else `next` until the first close landing a slice moves it to `now`; its slices stay off the roadmap.

**Cost while deferred:** [companion-toolkit-profile](#companion-toolkit-profile)'s submission waits on it, and a companion adopter today chooses between document gates alone and every kit installed beside the toolkit's own workflow. Filed 2026-09-29 to the gap inbox by preview-readiness' lead; promoted 2026-09-30 at its close: →fix fails because a tier is new mechanism and new governed names, →forward because the direction is given. Re-verified: companion/SPEC.md §The two tiers has `prose` (canon-kit's document gates) and `full` (every kit), with no exclusion. Owner lookup: `exclusion`, `conflict`, `companion tier` in this file — none; owner companion/SPEC.md §The two tiers, with installer/SPEC.md §Profiles.

### roadmap-horizon-lag-detector

[cost: iteration/low] [surface: lifecycle-kit]

close's roadmap-motion read files a consult item only for a slug that left the projection and for a vacant first horizon, so an entry a ruling re-sequences, or that iterations work toward while it stays tagged `next`, fires neither. The operator's horizon principle, 2026-09-29, lead-relayed (not a ruling), gives the detector its criterion: a horizon reflects reality, `now` when work is planned as soon as possible, else `next`, and the first close landing a slice of an entry moves it to `now`. So an entry with a slice landed in the range (a Done split child, or a landed line in its body) still tagged `next`, or carrying no roadmap tag though its outcome is curated, is lagging, and close files the consult item.

**Deliverable:** that read in lifecycle-kit/templates/stages/close.md step 5, with its mechanical half (slice landed, horizon tag) on an arm rather than a judgment, or a boundary note in lifecycle-kit/SPEC.md §templates/stages/ refusing it.

**Inferred, not run:** rulings landed before the consult binding's re-tag surface existed had no retroactive horizon pass.

**Cost while deferred:** the public roadmap lags the queue until a consult happens to look. Instances: [companion-toolkit-profile](#companion-toolkit-profile) held `next` from 2026-08-02 through the companion iterations; [spec-brevity-residue](#spec-brevity-residue) and [gate-customer-value-audit](#gate-customer-value-audit) carried no tag with slices landed, both curated at the 2026-09-29 consult, the audit to `next` on the relayed conditional. Filed 2026-09-29 to the gap inbox by preview-readiness' lead, with an addendum carrying the criterion, merged here; promoted 2026-09-30 at its close: →fix fails because a close read is new stage mechanism, →forward because no ruling is owed. Re-verified: close.md step 5 names only the two triggers. Owner lookup: `horizon`, `lagging` in this file — none; owner lifecycle-kit/templates/stages/close.md, with queue-kit/SPEC.md §The roadmap arm.

### update-availability-notice

[cost: event/low] [surface: installer]

nothing tells an installed consumer a newer Checkwright release exists, so an upgrade happens only when the adopter thinks to run `update`. Operator direction 2026-09-29, lead-relayed (not a ruling): add one. The lead's proposed shape, for spec to challenge: an advisory line from the session-context hook (every profile and agent session) and the same reading in `doctor`; the probe `git ls-remote --tags` on the upstream (git is the one floor), compared with the manifest's recorded version, cached in gitignored scratch under a consumer-selectable interval set with off among it, bounded by a timeout, silent offline or on failure. The operator's seeded default is weekly; the probe discloses the adopter's IP to the host each interval, so `init` states it is on and how to turn it off.

**Deliverable:** that notice, or spec's challenge to its shape; the zero-code interim is a docs pointer to `npm outdated` and GitHub release watching.

**Cost while deferred:** an adopter runs a stale release unknowingly, paid at each release. Filed 2026-09-29 to the gap inbox by preview-readiness' lead; promoted 2026-09-30 at its close: →fix fails because the notice is new mechanism with a knob, →forward because the direction is given. Re-verified: no `ls-remote` probe or newer-release notice in installer/SPEC.md, context-kit/SPEC.md or `native/src`. Owner lookup: `update notice`, `newer release`, `outdated` in this file — none; owner installer/SPEC.md §The upgrade contract, with context-kit/SPEC.md for the hook. Surface also context-kit.

### front-door-rehearsal-rule

[cost: event/high] [surface: lifecycle-kit]

the front-door rehearsal belongs in the methodology, operator direction 2026-09-29, lead-relayed (not a ruling). `front-door-container-rehearsal` found two defects no smoke had caught, a hooked update or profile move refused by `check-gate-tamper` and `init`/`uninstall` hiding git's own failure output, because every smoke installs a tree-packed payload from the author's seat and never the published artifact from a clean one.

**Deliverable, three layers:** (1) a generic kit rule, before an audience-facing event rehearse the published front door from a clean seat and file what it finds, the container, platform and route set being consumer config (doctrine-kit or lifecycle-kit's release step, per the provenance seam); (2) here, a post-publish job in `.github/workflows/publish.yml` installing the just-published Release on clean runners (init, hooks on, a first red, an upgrade from the previous release, a hooked profile move), reaching the macOS and Windows runners a container cannot; (3) a manual agent-walked rehearsal of the routes CI cannot drive (the adoption prompt, the plugin marketplace, the Spec Kit extension), before audience events only, a catalog submission or a partner install, per the operator.

**Cost while deferred:** each audience event risks a front-door defect only a clean-seat install would show; the one-off left macOS, native Windows, arm64 Linux, WSL and the npx, plugin, PowerShell and agent-prompt routes unrehearsed. Filed 2026-09-29 to the gap inbox by preview-readiness' lead; promoted 2026-09-30 at its close: →fix fails because each layer is new mechanism, →forward because the direction is given. Re-verified: `publish.yml` runs roster, build, pack, npm and release and installs nothing after publishing. Owner lookup: `rehears`, `post-publish`, `clean seat` in this file — [design-partner-preview](#design-partner-preview), DISTINCT (the observed install this precedes); owner RELEASING.md for layer 2, the kit rule's home open. Surface also doctrine-kit.

### front-door-arm-route-blind

[cost: event/low] [surface: installer]

`check-front-door-verbs` reads an advertised flag only after a route, so a gate-binary arm a front-door page advertises in prose, as docs/install.md's "run the gate binary with `--measure-commit`" and `--emit env-probe` do, is never checked against the pinned release, and a `none` or `deferred:` disposition passes it. installer/SPEC.md §The front door's verbs states the limit ("A flag named apart from its route … is out of reach").

**Deliverable:** invariant B reads a gate-binary arm the page advertises outside a route, under the same pending admission, with a `bad/` fixture holding the prose form; or a boundary note keeping the limit.

**Cost while deferred:** a front-door page can advertise an arm the installed release refuses, unseen until an adopter runs it. Filed 2026-09-30 to the gap inbox at preview-readiness' close, where `--measure-commit` landed after v0.30.0 and a `deferred:v0.31.0` probe line left the gate green; promoted 2026-09-30 at the next iteration's scope: →fix fails because widening a shipped gate's read is new mechanism. Re-verified: docs/install.md advertises both arms in prose, and v0.31.0 carries `--measure-commit`, so the instance is discharged and the class stands. Owner lookup: `front-door-verbs`, `route` in this file — [front-door-flag-operand](#front-door-flag-operand), DISTINCT (a flag's value after a route, not an arm outside one); owner installer/SPEC.md §The front door's verbs.

### lessons-learned-channel-audit

[cost: iteration/low] [surface: queue-kit]

the Lessons Learned channel may be obsolete, operator direction 2026-09-30, lead session (not a ruling). TASK-QUEUE.md's section is empty; the last tagged lesson was written 2026-07-11 and the last harvested 2026-09-16. Still wired to it: close step 1 with `check-lesson-disposition`, the lesson-evidence file and its boundary truncate, queue-index's attend-tag attention block, the essay harvest through `--lesson-sink`, and validate's filing rule routing method observations there. Candidate replacements: the gap inbox, kfric, the survey record, the consult inbox and the lead journal. Possible losses: a door from method observation to durable rule, since the gap drain offers fix, promote or drop; and the essay harvest, if posts draw on it.

**Deliverable:** a function-to-replacement map and the essay harvest's reader confirmed; then, per Policy-as-choice, most likely the section made consumer-optional and off here rather than deleted, since queue-kit and lifecycle-kit ship it to adopters.

**Cost while deferred:** every close walks a lesson step and a disposition gate over an empty channel, and validate routes observations to a section nothing reads. Filed 2026-09-30 to the gap inbox by preview-readiness' lead; promoted 2026-09-30 at the next iteration's scope: →fix fails because the audit is unsized and its likely outcome is a new knob. Re-verified: the `## Lessons Learned` section holds no bullet. Owner lookup: `lesson`, `essay` in this file — only the icebox's reclaim-precondition-outside-the-tree, DISTINCT (the essay sink's reclaim); owner queue-kit/SPEC.md §The queue format, with lifecycle-kit/templates/stages/close.md. Surface also lifecycle-kit.

## Icebox

  Dormant entries, one line each: the cost field said the carry was low, no `[roadmap:]` commitment rides on it, and no named event is waiting to promote it. Still live work — a legal `[blocked-by:]` target, conserved on the way in and on the way back out. The removed body is recoverable from the evicting commit (queue-kit/SPEC.md §The icebox tier).

### disclaimer-beside-its-own-restatement

No gate reads a restatement-disclaimer phrase sitting beside a content clause that restates the rule it disclaims.

### residency-roster-template-reach-ungated

No oracle reads the resident-tier placement rule.

### fixture-suites-never-run-history-less

No leg runs fixtures history-less.

### lead-held-block-no-sanctioned-surface

No route records a lead-held block.

### survey-record-filed-after-the-fact

Order to the work goes wholly unread.

### icebox-drops-a-bought-census

No dormant home for a measured payload.

### inline-source-literal-ungateable

Fence-only oracle; no rename pending.

### turn-end-refusal-used-as-a-busy-wait

Sessions busy-wait via the stop hook.

### runtime-dir-two-tier-detector

No two-tier proof for file-pattern ignores.

### done-slug-commit-naming-gate

Done-moving commits need not name their slug.

### enter-stage-simulate-no-write-fixture

Guard present, unpinned by a fixture.

### stage-lag-disambiguation

Hook over-firing is accepted, not a defect.

### metric-dir-admission-unstated

Ad-hoc scripts persist in .metric/.

### hermetic-bin-roster-config

Pinning coverage needs a consumer roster seam.

### supervisor-verification-attestation

The verification duty is unattested.

### gate-spec-claim-assertion-parity

Ruled a human-audit class, not gateable.

### port-takeability-has-no-instrument

Takeability hand-read on the tree axis.

### scope-amendment-authoring-gate

Scope can do spec's job and stay green.

### evidence-journal-hash-chain

Tamper-evidence wanted only by a hosted rung.

### operator-authored-unit-set

The contract omits operator-authored unit sets.

### action-run-shell-scan-predicate

No consumer seam on a correct gate.

### scratch-execution-allowlist-bar

Each close re-derives this standing bar.

### gate-tamper-consumer-gate-coverage

A glob and a roster audit remain.

### upgrade-contract-rename-routing-unstated

One clause leans on it.

### md-refs-tree-link-resolution

Unreachable while one generator produces.

### recurrence-judgment-vs-declaration

The two share a noun, not a meaning.

### advisory-lane-draft-state-unswept

GitHub's notifications are the sweep.

### survey-record-extension-tier-hybrid

Paid only by a future workflow author.

### install-lifecycle-reversibility

A declined branch; only optionality owed.

### rendered-site-link-monitor

Rendered-site link rot waits on a launch crawl.

### kit-index-page-vocabulary-ungated

Index-page enums are ungated.

### context-pressure-signal

Compaction timing has no per-session signal.

### post-immutability-machine-read-carveout

Immutable prose, live machine read.

### path-pinned-allow-entry-oracle

No scanner reds a path-naming grant.

### economics-posture-binding-stale

A shim restates a ruling it should cite.

### align-context-draw-growth

Two falls read the draw as work-side.

### customer-facing-iteration-cadence

No tracked classifier for the bound.

### scan-prompts-truncation-quote-desync

Truncation inflates the scan only.

### template-out-of-tree-copy-obligation

Out-of-tree copies are unreachable.

### doctrine-rule-number-citation-liveness

A renumber stales citations.

### false-ground-citation-propagation

Nothing re-reads a ground once cited.

### spec-embedded-source-criterion-4-membership

Its port sizing stays unruled.

### lead-dispatch-simulate-optionality

Dispatch may skip the pre-flight.

### self-repo-prefix-normalisation-unheld

Two link-prefix holders, unheld.

### stage-cursor-rerun-stamp-gap

A skipped re-run stamp points the cursor back.

### interpreter-grant-redirect-residue

Seven redirected shapes stay ungranted.

### build-native-obligation-unconditional

A crate-free commit still rebuilds.

### port-blockers-library-mediated-scan

A library-mediated spawn reads clean.

### walk-entry-model-unstated

Walk drops symlinks unstated; tree has none.

### prune-set-convergence-question

Two kits' prune sets diverge, unruled.

### gap-inbox-slug-predicate-ground

Its anti-cycle premise died unreplaced.

### cited-object-token-sweep-corpus-narrower-than-the-class

Corpus unruled.

### worktree-lock-start-time-guard-untaken

Dormant until a consumer acts on it.

### friction-key-segment-selection-unruled

Which segment to key is unruled.

### post-build-instrument-edit-unowned

No stage owns a post-build tree edit.

### smoke-roster-guard-precedes-hand-off

Guard stricter than its stated reason.

### smoke-report-array-carrier-mangling-unexplained

Witness now needs design.

### root-doc-roster-registration-parity

Only one root-doc roster is enforced.

### self-revert-reminder-expectation

Self-revert reminder reads as injection.

### co-authored-by-trailer-attribution

Model trailer is a baked literal.

### guard-steer-grant-mismatch

Tree steers unpaired; the kit's are templated.

### reclaim-precondition-outside-the-tree

Essay-sink reclaim can never fire.

### amendment-dod-sibling-dependence

DoD items depend on unnamed siblings.

### recurrence-resolver-literal-match-only

Unspelled recurrences file as new.

### section-prose-outlives-its-entries

Section preambles outlive Clear-Done.

### unregistered-gate-fixture-coverage

Unregistered gates skip fixture duty.

### queue-tier-label-correction-cost

Fixing a label at the cap costs a trim.

### rejected-compound-commit-relabel

A bare retry mislabels staged work.

### survey-record-supersede-invisible

Superseded survey blocks look live.

### release-runbook-identity-diagnosis

Account check is prose, not a step.

### consult-rulings-outside-the-authority-roster

Consult readings lack a slot.

### ruling-record-prose-staleness-unreachable

Old rulings evade the probe.

### close-surface-reclaim-uncoupled-from-read

Reclaim may wipe unread rows.

### post-scope-admission-has-no-promotion-route

Late debt has no promoter.

### declaration-shape-outside-header-unreadable

Inert literals read as live.

### boundary-preserve-covers-names-not-lifetimes

Keep-list lists names only.

### validate-suite-wall-clock-unowned

Serial smoke suites cost ~16 minutes.

### enforcement-first-behavioral-regressions

Rule under-cues behavior gates.

### spec-split-promotion-review

Spec-stage default awaits an economics read.

### build-stage-tier-economics

Build tier set by intuition, not a priced A/B.

### supervision-overhead-unmeasured

Supervision burn priced; quality unread.

### gate-battery-result-cache

Battery reruns all gates on an unmoved tree.

### state-representation-integrity

Text-state invariants are gate-held only.

### rule-reach-before-merits

Merits argued before a rule's reach is set.

### template-copy-parity-yaml-widening

YAML template copies mirror by hand.

### template-spec-restatement-reach

No gate holds a SPEC off its template.

### amendment-deletion-content-completeness

Merges can drop rationale unheld.

### lead-line-parser-conformance

Eight lead-line holders; no conformance.

### breadth-declaration-stale-listing

Spent breadth declarations stay silent.

### breadth-declaration-committed-glob-home

Glob keep-rulings have no home.

### criterion-4-two-spellings-disagree

Criterion 4 reads two ways.

### substrate-parity-assertion-c-reach-unannounced

C can shrink unannounced.

### recurrence-threshold-counts-dates-not-incidences

Same-day firings merge.

### same-stage-journal-append-uncoordinated

Parallel appends share one file.

### survey-engagement-trigger-narrower-than-its-class

Trigger is scope-only.

### scratch-citation-introducer-form-reach

Copula pointers evade the scan.

### threshold-entry-escalation-travel-unruled

Rider or competitor, unruled.

### amendment-target-delta-correspondence-unverified

Orphans in both ways.

### dispatched-child-asserts-an-unverified-base

Children infer, not probe.

### hermetic-harness-export-masks-the-condition-under-test

Pins void arms.

### verbose-battery-idiom-steered-to-its-granted-spelling

Bare form prompts.

### exe-suffix-single-spelling-unenforced

Suffix owner claim has no gate.

### intra-stage-batch-stamp-unobserved

A skipped batch stamp goes unseen.

### boundary-sweep-github-write-skips-identity-step

Account check unenforced.

### wait-primitive-and-record-compose-to-false-completion

Waiters exit early.

### kfric-second-field-direction-inverted

Surface field names the owner.

### baseline-self-certification-unasserted

Self-served verdicts unasserted.

### pre-grammar-disposition-authority-ambiguity

Double-named rulers unread.

### kpi-cost-per-unit

No KPI prices cost per shipped unit.

### line-range-citation-stales-inside-its-own-iteration

Line ranges go stale.

### amendment-commit-shape-red-conditions

Prompt lacks a commit-shape class.

### amendment-correction-density

Correction density goes unmeasured.

### probe-evidence-sufficiency

guard-kit rule `pgrep_self_match` passes a probe that is no evidence.

### scratch-citation-skill-surface-reach

Skill files escape the pointer scan.

### throughput-and-wait-time-unmeasured

Wait and throughput are unmeasured.

### headroom-check-ordering-unruled

When to read cap headroom is unruled.

### close-red-push-ownership

No owner for a close blocked by a red push.

### expected-permission-mode-undeclared

No surface states the expected mode.

### scan-prompts-blocking-half-blind

The KPI cannot see blocked commands.

### handoff-premise-reverification-placement

Premise-check placement unruled.

### amendment-work-class-label-placement

Work-class tag placement unruled.

### tracked-to-untracked-pointer-scope

Untracked-target pointer scope unheld.

### born-native-flip-enforcement-gate

Born-native rule has no enforcing gate.

### msrv-move-clippy-arm-coupling

Floor moves surface unbudgeted lints.

### declaration-lib-refusal-output-leak

Refusal output mixes in good tokens.

### deferred-pool-identifier-restatement-sweep

The pool was never swept.

### fixture-assertion-liveness

Stale fixture expectations go uncaught.

### fixture-assertion-coverage-unmeasured

Driver coverage of arms unmeasured.

### survey-engagement-residue-untracked

Engagement residue rests on conduct.

### metric-dir-member-contract-unheld

The metric dir accretes leftovers.

### kit-bin-entry-point-unrostered

No roster maps bin tools to kits.

### gate-test-in-tree-invoker-ruling

Is a gate-test an in-tree caller?

### survey-oracle-liveness-unasserted

An oracle may name a wiped path.

### stage-completion-unattested

Entry stamps cannot show completion.

### deferred-entry-time-deixis-rot

Relative deixis rots in deferred bodies.

### iteration-scoping-clause-date-ambiguity

Date scoping names no iteration.

### assertion-strength-exit-header-reach

The gate now reaches no script.

### audit-depth-measure-degrades-under-fanout

Fan-out depth is self-reported.

### candidate-list-anchors-a-sweep-obligation

A relayed list anchors a sweep.

### spec-pointer-boundary-legality

Resolving targets may be illegal owners.

### ruled-line-retirement-provenance-census

Census of the retirement's cuts.

### amendment-census-claim-unrun

Amendment counts skip the shipped oracle.

### composition-test-scores-a-section-cut-as-one-unit

A cut scores as one.

### port-created-failure-mode-refusal-unruled

Port-made refusals unruled.

### removal-propagation-site-argued-out-of-scope

Found sites argued away.

### icebox-trigger-blind-to-retired-carrier

Blind to retired carriers.

### rationale-located-by-reading-not-by-grep

Grep misses paraphrases.

### smoke-whole-tree-precondition-unscoped

Any dirty path blocks the smoke.

### spec-internal-identifier-prefix-drift

SPECs cite internal names, not knobs.

### lint-scope-hook-trigger

extra lint dirs skip the commit hook, CI-only.

### knob-default-accessor-singularity

no gate bars re-spelling a knob default.

### docs-corpus-derivation-manifest-divergence

two docs gate corpora diverge.

### consumer-gate-roster-unread

no roster reads a consumer-declared gate.

### waiter-loop-condition-predicate-gap

waiting rule misses a lone waiter loop.

### prose-uniqueness-claim-unchecked

no gate checks a prose uniqueness claim.

### kit-spec-layout-tree-hand-maintained

kit SPEC layout trees are hand-kept.

### expansion-rule-backtick-blind

expansion rule misses backtick substitution.

### artifact-substitution-remedy-has-no-end-to-end-arm

untested end to end.

### readme-bin-roster-underived

no gate holds a kit README's bin/ tool roster.

### align-in-session-absorption-tier-unruled

operator-class; awaits a consult.

### push-account-selection-has-an-explicit-per-command-form

remedy probed, unranked.

### survey-locator-review-catches-after-loss

a gate here reopens a stated refusal.

### lead-agent-id-compaction-defense

each claim still wants its own probe.

### gap-capture-argv-prompt-friction

Capture arms take their prose as argv, so a filing carrying shell punctuation costs a permission decision; a `--from <path>` body arm is the candidate fix.

### docs-cmd-retired-path-blind-to-queue

`check-docs-cmd` assertion (C) misses a retired path cited from the queue, which is outside its manifest corpus and whose `<path>:<line>` token fails the path shape.

### local-only-files-write-back-untriggered

The consumer's local-only companion files (private brief, ops runbook) have read triggers but no write-back trigger, so shipped-unit forward memory and an out-of-tree state verifier drift until a consult audits them; owed are close and release-sweep template slots for both, and a retired-slug arm over plain code on the local-only globs.

### retired-citation-referent-rule

Whether a live queue entry should cite shipped mechanism by a stable anchor (a SPEC section, a gate name, a path) rather than a retired slug, and whether a check-queue-hygiene axis should hold that, is unruled; close's retired-block read corrects each instance inline meanwhile, as it did twice at preview-readiness' close.

## Done

- scratch-auto-allow-no-decoration-steer
- consumer-guard-rule-coverage
- plugin-guards-subdir-launch
- one-motion-commit-race-remains-open

## Lessons Learned
