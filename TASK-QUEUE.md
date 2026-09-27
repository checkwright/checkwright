# TASK-QUEUE.md — Checkwright work queue

## Iteration: catalog-submission-preconditions

  The lifecycle-kit gates read this header's iteration name and the stage cursor — the last stamp in `.workflow/WORKFLOW-STATE.txt` (lifecycle-kit/SPEC.md §The state machine); queue-kit formalizes the queue format itself and gates this file. One iteration per hardening or roadmap unit; [docs/kits.md](docs/kits.md) maps the kits.

---

## New Features

### linux-glibc-artifacts

[spec: SPEC-linux-glibc.md]

the next release would publish no glibc Linux gate binary: platform-prerequisite-floors replaced the two `*-linux-gnu` lines of `native/targets.list` with the `*-linux-musl` pair, over the hybrid that would publish both. The operator wants glibc artifacts published beside musl before the next tag (operator direction 2026-09-27, lead-relayed at companion-catalog-extension's close), and v0.27.0 is held on it.

**Deliverable:** the gnu triples back in `native/targets.list` and `native/runners.list` beside musl, joined under that file's predicate; the installer's host-to-artifact choice between them on a Linux host; docs/install.md's platforms table and the release declarations to match.

**Cost while deferred:** docs/spec-toolkits.md, live and linked from the front door, sends adopters to Release assets no tag yet carries, since v0.26.0 predates `companion/`, and that tag waits on this. Filed 2026-09-27 to the gap inbox at companion-catalog-extension's close; promoted 2026-09-27 at the next iteration's scope. Re-verified: `native/targets.list` lists only the musl Linux triples, and `gh release view v0.26.0` lists only the gnu Linux archives. Owner lookup: `glibc`, `linux-gnu`, `targets.list` in this file — `install-platform-release-gap`, [musl-dev-binary](#musl-dev-binary) and [musl-smoke-build-wrapper](#musl-smoke-build-wrapper), DISTINCT (the page-to-gate binding; the dev binary; the smoke's hand-off); owner gate-sdk/SPEC.md §Consumer payload. Surface also gate-sdk.

**Taken for /spec 2026-09-27 at catalog-submission-preconditions' scope, operator direction lead-relayed (not a /consult ruling):** the amendment settles the host-to-artifact choice and the declaration rows in one, with `install-platform-release-gap`'s gate reading the table it settles.

**Push need (2026-09-27, overrun granted by operator direction, lead-relayed):** the mid-iteration observation push, shared with `windows-fresh-fixture-stub`, that joins the gnu lines under the file's predicate; then the release tag beside the closing push, since v0.27.0 is held on this.

## Technical Debt

### crate-tests-windows-flip

[observed-by: gates]

`crate-tests-windows` in `.github/workflows/gates.yml` carries a hard-coded `continue-on-error: true`; once one run is green on both triples it moves to the `matrix.held` expression `install-smoke-pwsh-windows` reads, and the reports-until-green sentences leave gate-sdk/SPEC.md §check-crate-arms and the capture-drain limit in §The workflow directory.

**Deliverable:** that flip and those deletions, in one commit, after `windows-fresh-fixture-stub` lands.

**Promoted as debt 2026-09-27 at catalog-submission-preconditions' scope, operator direction lead-relayed (not a /consult ruling):** the flip converges the job on the `matrix.held` convention and adds no name. Precondition (1) of the catalog submission, on the operator's ground that red jobs inside a green run read as ignored failures.

**Push need (2026-09-27, overrun granted by operator direction, lead-relayed):** the mid-iteration observation push `windows-fresh-fixture-stub` spends supplies the green run this reads; the flip rides the closing push.

**Cost while deferred:** a Windows crate-test failure passes every run as a warning only close reads. Filed 2026-09-26 to the gap inbox by native-contracts-brevity's build as the follow-up crate-tests-unrun-on-windows' amendment ordered; promoted 2026-09-26 at its close: →fix fails because the job has no green run yet. Re-verified 2026-09-27 at scope: `gates.yml` reads `continue-on-error: true`, and run 36338295147 still fails the registry pair on both triples.

### spec-toolkits-guarantee

docs/spec-toolkits.md says four defect classes "fail at commit and in CI" and never states what that guarantees: the pre-commit hook is skippable with `--no-verify`, so the guarantee is CI run as a required status check under branch protection, and the page does not say so. It also leaves unsaid that the four classes are document hygiene, not spec-to-code conformance. The operator found the page unconvincing (operator direction 2026-09-27, lead session).

**Deliverable:** the page states the guarantee and its setup (the seeded CI workflow as a required status check), names the four classes as document hygiene and not conformance, and keeps its limit that the extension's commands and hook are agent-followed. Where `foreign-spec-lifecycle-unowned` lands a lifecycle binding in the same set, the page states what that adds and for which profile.

**Promoted as debt 2026-09-27 at catalog-submission-preconditions' scope, operator direction lead-relayed (not a /consult ruling):** a page edit stating facts the specs already carry. Precondition (4) of the catalog submission.

**Cost while deferred:** the catalog's one-shot exposure lands on a page whose claim a skeptical reader cannot place. Filed 2026-09-27 to the gap inbox by the lead after companion-catalog-extension's close. Re-verified: the page carries no `required`, `--no-verify` or branch-protection sentence. Owner lookup: `spec-toolkits`, `guarantee` in this file — [companion-toolkit-profile](#companion-toolkit-profile), DISTINCT (the submission this gates); owner companion/SPEC.md.

### catalog-landing-docs-polish

the pages a Spec Kit catalog visitor reaches are not yet polished for that one-shot exposure: the extension's `repository` and `homepage` (README.md, docs/index.md), its README (companion/speckit/README.md), and the landing page with the install page it routes to (docs/spec-toolkits.md, docs/install.md). The `catalog-then-plugin` ruling puts their polish before the submission; the page-authoring rules and the one-product statement have landed, so what remains is the polish itself (operator direction 2026-09-27, lead session).

**Deliverable:** a catalog visitor's read through those five pages against docs/site-architecture.md §Page-authoring rules, each finding fixed on the page or filed costed. It runs last in the set, after the platforms table, the guarantee statement and the lifecycle binding have changed those pages.

**Promoted as debt 2026-09-27 at catalog-submission-preconditions' scope, operator direction lead-relayed (not a /consult ruling):** convergence on the landed page-authoring rules, adding no name; bounded to the pages the extension manifest and the landing page link. Precondition (3) of the catalog submission.

**Cost while deferred:** the listing sends its visitors to pages no reader-journey pass has read. Filed 2026-09-27 to the gap inbox by the lead after companion-catalog-extension's close. Owner lookup: `polish`, `landing page` in this file — [companion-toolkit-profile](#companion-toolkit-profile), DISTINCT (the submission); owner docs/site-architecture.md §Page-authoring rules.

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

**Cost while deferred:** a reader following such a pointer finds no support for the literal, and a later SPEC edit cannot tell the comment depends on it. Filed 2026-09-26 to the gap inbox by gate-sdk-meta-gate-brevity's citation survey; promoted 2026-09-26 at front-door-release's close: →fix fails because the class spans the crate and each comment needs a local-versus-contract call. The survey's other instance, `native/src/hook/stop_liveness.rs` citing §check-test-hermetic for per-case scratch roots, was fixed at the drain. Owner lookup: `spec-pointer`, `comment-tier-exempt`, `shell form` in this file — none; owner canon-kit/SPEC.md §check-comment-tier.

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

### side-effect-free-read-arms

[cost: event/high] [surface: guard-kit]

shell utilities an agent runs for read-only work can also write: `sed -i`, `find -delete` and `-exec`, awk's `system()`, `tee`, redirects. The operator's shape: block side-effect-capable utilities and steer to side-effect-free arms of the gate binary that can be allowlisted and advertised as their replacements. The seed is gate-sdk's fence-safe arm set, `FENCE_SAFE_ARMS` (gate-sdk/SPEC.md, arms that reach no network and write nowhere). It succeeds guard-kit rule `worktree_confinement`'s interim read-only Bash allowlist for isolated children, whose advertised set and refusal message are worded to point at these tools later.

**Deliverable:** the arm roster that replaces each blocked read, the guard rules that steer to it, and the allowlist entries. New governed names, so it owes an amendment.

**Cost while deferred:** a read-only shell call keeps a write path the guard must judge per call, and an isolated child's read set stays the interim allowlist. Filed 2026-09-23 to the gap inbox by the lead on an operator direction; the operator recalls earlier discussion and no tracked record was found. Promoted 2026-09-23 at seam-and-stage-residue's close drain: an initiative with new names, never a drain fix. Owner lookup: `side-effect`, `FENCE_SAFE`, `dual-use`, `sed -i` in this file — none. Surface also delegation-kit and gate-sdk.

### retired-citation-referent-rule

[cost: event/low] [surface: queue-kit]

live entries cite retired slugs in prose, and nothing marks them the same way ('(landed)', '(retired)', 'Done <date>'). After the queue-headings amendment, a retired citation is a backticked token that assertion R keeps apart from a live link (queue-kit/SPEC.md §The tag algebra), but it still has no anchor. The open question is the operator's: should a live entry cite the shipped mechanism by a stable anchor (a SPEC §, a gate name, a path) rather than the retired slug? And if so, should a `check-queue-hygiene` axis hold that, given that the queue-edges arm already computes the retired set? A provenance citation ("filed during X") names the slug correctly and must stay legal. This is inferred, not measured.

**Deliverable:** that ruling, and either the axis or a stated reason the close-stage retired-block read is enough.

**Cost while deferred:** a retired citation's referent can be reached only through history. Filed 2026-09-22 to the gap inbox by the lead on an operator question. The one-off sweep it asked for ran at queue-kit-unwrap's close: it found 28 retired rows plus one name-live row, most already marking retirement, and corrected three inline. Promoted for the rule half. Owner: queue-kit/SPEC.md §The tag algebra and §The queue-edges arm; neither rules on the referent.

### disclaimer-beside-its-own-restatement

[cost: event/low] [surface: canon-kit]

a surface that disclaims carrying a rule ("stated there and not restated here") in the same sentence that carries it is asserted by nothing, and the disclaimer tells every sweep the copy is not one.

**Attested once, fixed inline:** README.md §This repo, governed restated the commit-time fixture-suite selection rule beside exactly that disclaimer, so correcting CLAUDE.md stranded README's copy; `5e0e10f8` de-literalized it. What survives is the class, not the instance.

**Why reachable when general restatement is not:** the predicate is a disclaimer phrase co-located with a content clause — does the sentence around it name the rule's substance rather than only its owner. `check-surface-duplication` and `check-shim-restatement` hold restatement for their own corpora; neither reads a disclaimer.

**Deliverable:** a gate, or an assertion joining an existing restatement gate, over that shape. A feature by the new-names litmus, so it owes an amendment.

**Cost while deferred:** each such disclaimer is a licence a later reader trusts, and the copy beside it rots silently. Filed 2026-09-20 to the gap inbox by the close of `adopter-floor-door-remainder`; promoted 2026-09-21 at the next scope's intake, so the record is late and says so. Owner lookup ran over `disclaim`, `not restated here`, `restatement` and the two gates above and found no owner.

### config-variant-battery-harness

[cost: event/high] [surface: gate-sdk]

nothing shipped lets a customer run the battery under a named config-seam variant and see what changes; the fixture pairs prove each gate's arm against fixed trees, and the smoke scripts are this repo's harness legs.

**Operator ruling at consult: file it costed.** Deliverable: a shipped, bridged arm that takes a scratch copy of the consumer's tree, applies a named variant of the config seam, runs the battery, and prints the per-gate verdict diff against baseline — so an adopter sees which gates a knob arms, disarms or reds before committing the knob.

**Why design-pending:** the variant's declaration form (an env file or a config-dir overlay), whether the scratch copy is a worktree or a copy that carries untracked content, and whether the diff or a full report is the product. Native, per the interpreter constraint (gate-sdk/SPEC.md §The adopter constraints).

**Refused:** repurposing the smoke scripts, which are install recipes read as text by the install-disposition gate and bound to this repo's harness; a shipped directory of shell tests, which widens the interpreter surface the adopter constraints shrink.

**Cost while deferred:** an adopter evaluating a knob edits the seam, commits, and learns from the next red; the preview cohort's false-positive dispositions have no cheap rehearsal. Filed 2026-09-11 by consult as a direct entry, the test gap the operator named there.

### plugin-marketplace

[roadmap: next/ecosystem] [cost: once/low] [surface: installer] [roadmap-summary: The stage skills and guards installable as a harness plugin.]

harness plugin packaging. Harness plugin/marketplace packaging of the stage skills and guards; anti-drift gate shape: manifest ↔ shipped surface parity. Design against the live manifest format at promotion — the plugin substrate moves fast (the scope-session-routing ruling applies).

**The install-ownership contract this must package against already exists:** `checkwright.lock`, written by the installer's `init` and specified at installer/SPEC.md §The manifest — its schema owner is `native/src/installer/lock.rs`. A marketplace package that installs kits without writing that manifest would be a second install model with no upgrade or uninstall story, which is the sequencing risk this entry has always flagged; the named contract replaces the re-derivation it used to imply. The upgrade/uninstall story itself has shipped as the installer's `update` and `uninstall` verbs, specified at installer/SPEC.md §update and §uninstall — sequence against those rather than duplicating them.

**Negative result — the tarball channel's economics do not transfer here.** The retired `release-tarball-delivery-channel` was cheap for a structural reason that is absent from this rung: `.github/workflows/publish.yml`'s `pack` job already assembles and stamps one tarball and uploads it as the run's artifact, so a new channel is a sibling job that `needs: pack` and consumes that artifact. A marketplace package cannot consume it. Its unit of delivery is the harness's own plugin manifest format, not a packed npm assembly, and its subject is the stage skills and guards rather than the eleven-kit tree — so it shares neither the assembly nor the artifact. Recorded because the reflex at promotion will be to cost this by analogy from the tarball's sibling-job cheapness and arrive at the wrong number.

**Open question a promoting scope answers first — deliberately undecided here.** Whether the marketplace package vendors kits at all, or merely registers the skills and delegates all vendoring to the installer's `init`. Under the second answer it stops being a distribution channel and becomes a **discovery surface**, and `checkwright.lock` ceases to be a contract it must *honour* and becomes one it must not *violate* — the materially cheaper answer, and the one that dissolves most of the sequencing risk above. It is not settled here because it is downstream of this entry's standing ruling that the plugin substrate moves fast and the design must be made against the live manifest format at promotion; deciding it now would be deciding it against a format that will have moved. Recorded 2026-07-26 by close (`activation-path`).

**The format has settled, and it answers the question.** A cross-vendor plugin packaging standard reached 1.0 (consult's landscape refresh, 2026-09-25), adopted by several harnesses with org-managed enable/block lists, and it excludes hooks from its portable core. So the package is a discovery surface registering the skills, vendoring stays with `init`, and the gates stay in the binary outside every harness — the cheaper answer above, now with a stable target.

**Cost while deferred:** zero mechanism rots — the install-ownership contract this must package against is already written and maintained by the installer's `init`; what is foregone is a discovery surface, and the plugin substrate's motion means a design taken early would be retaken at promotion anyway. Surfaced 2026-07-09 in adoption-track's split; evidence artifact retained: upstream Claude Code issue #75214 (project config can't lift the Task ask-first default), surfaced dogfooding the delegation nudge 2026-07-07.

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

foreign agents. Cross-vendor stage dispatch: a lead delegating a stage to a foreign coding agent, extending the homogeneous multi-agent model to a heterogeneous fleet. It cashes the public no-lock-in claim and is the purest expression of the thesis — governance enforced at the git/gate boundary, not by trusting the author. *Already agent-neutral:* the verification substrate (git, the gate battery, the bash stamp state machine) does not care who authored the diff, and the coordination primitive is the shared git-index/HEAD serialization. *Homogeneous today — the real work, worst-first:* (1) the **escalation resume model** collapses into (2) as a property of the chosen transport, per the 2026-07-25 amendment below; (2) **dispatch transport** — today the harness `Agent`/`SendMessage`/task-notification; a foreign agent needs a transport-neutral handoff. The adapter contract is "open / prompt / permission-request / resume" spoken over each vendor's structured **machine plane, never its TUI**: a screen-scrape relay is the adapter of last resort for a vendor shipping no machine interface at all — it yields rendered frames not turn events, answers dialogs by heuristic, and bets on the vendor's least-stable surface. (3) **budget oracle** — the verdict tool is Anthropic-OAuth-specific; a heterogeneous fleet has N vendor-keyed oracles, the same seam as the credential-swap entries, and the vendors' JSONL event streams carry the token-usage events a TUI path would scrape from a status bar. (4) **stage-contract expression** — the lifecycle machinery is neutral bash but the stage-skill prose is not.

**Seam ruling (on record):** generic mechanism only — transport, budget oracle, and escalation channel become consumer-config seams; a kit literal naming a vendor crosses the provenance seam and is ruled out, the pattern the retired `prose-profile` ruled. It extends the per-batch model-tiering lever across vendors, and interacts with [hosted-attestation-service](#hosted-attestation-service), [plugin-marketplace](#plugin-marketplace), and the credential-swap entries.

**Demand-gated — demand attested (2026-07-23):** the operator holds working foreign-vendor subscriptions and wants read-heavy delegation routed to them for budget headroom, and with three vendors live the N-keyed oracle seam is no longer hypothetical. First slice at promotion: a foreign-CLI executor for the already-pre-authorized read-heavy audit / mechanical-sweep class over a spawned non-interactive CLI process, one adapter per vendor as consumer config — not full stage dispatch.

**Its citers** ([companion-toolkit-profile](#companion-toolkit-profile), the credential-swap entries) block on none of it.

**Design-memory amendment (2026-07-25):** the TUI relay buys no session resume or token efficiency — both live in the vendor's session store (stateless APIs, the same on-disk transcript replayed against the same server-side prompt cache), so interactive-vs-headless is rendering, not state. Headless warm-resume by session id and JSONL turn events ship on the vendors probed, which makes (1) plumbing.

**Verification capability (2026-08-02):** those probes ran against **installed binaries** (the foreign CLIs are on the development machine), so the executor is verifiable, not inferred from vendor docs — a change to the unit's risk under oracle-first: the executor ships with a smoke that invokes them. The machine profile (context-kit/SPEC.md §bin/env-probe, local-only) owns which CLIs and how.

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

[roadmap: next/ecosystem] [cost: event/high] [surface: lifecycle-kit] [roadmap-summary: Gate a tree whose specs another toolkit's workflow wrote.]

the interop rung's submission half. The build half landed at companion-catalog-extension: `companion/` holds the Spec Kit extension and both recipes, the consumer smoke's companion arm proves them, and docs/spec-toolkits.md is their landing page (companion/SPEC.md).

**Deliverable:** the Spec Kit community-catalog submission, filed as the catalog's Extension Submission issue with its `download_url` naming the `checkwright-companion-<version>.zip` Release asset; the extension's README and the landing page then gain the catalog's install form.

**Gated on** a published tag carrying that asset, since the catalog installs from a tagged archive, and on [design-partner-preview](#design-partner-preview)'s observed install. **Ranked behind four preconditions the operator set on 2026-09-27:** [crate-tests-windows-flip](#crate-tests-windows-flip), since red jobs inside a green run read as ignored failures; [linux-glibc-artifacts](#linux-glibc-artifacts); [catalog-landing-docs-polish](#catalog-landing-docs-polish); and [spec-toolkits-guarantee](#spec-toolkits-guarantee).

**Push need (2026-09-27, inside the budget):** the release tag push beside the closing push, since the submission needs the extension on a published tag; close's release policy decides the cut.

**Cost while deferred:** the extension installs only from a Release URL a reader must already hold, so a Spec Kit user browsing the catalog, where adopters find enforcement extensions, does not find it. Surfaced 2026-08-02 at close; demoted 2026-09-27 at companion-catalog-extension's build, on its amendment's Definition of Done; survey correction (2), a lifecycle stage machine over a foreign workflow, went to the gap inbox.

### design-partner-preview

[roadmap: now/adoption] [cost: event/low] [surface: drift-kit] [roadmap-summary: A small observed preview measuring first green, first useful red and retention on real installs.]

a narrow external preview before any broad announcement: a preview cohort whose composition is ruled in the operator's private brief, the first install observed live and the rest arriving through the observation record, instrumented for time-to-first-green, first useful red, false-positive dispositions, and 7/30-day retention per kit. It is the first rung on this queue whose deliverable is **evidence from outside this tree** rather than a tree change.

**The TREE HALF landed in `external-install-evidence`** — drift-kit/SPEC.md §The install-observation record and §The install-evidence projection, the `--emit file-install` capture arm, and `docs/install-evidence.md` behind `check-install-evidence-fresh`. What is kept here is the expensive half: operator hours and a calendar window of thirty days or more, running beside later iterations and never inside a stage session. What re-promotes this entry is an observed install, not another tree change.

**Sequencing is the load-bearing part.** The preview runs *before* [benchmark-ab-experiment](#benchmark-ab-experiment), so pilot findings shape that experiment's task classes and metrics rather than being retrofitted to them; per-gate true/false-positive history and profile retention are preview deliverables, not pre-launch builds. The full launch ruling behind this sequencing is operator material and stays in the local-only private brief; this entry carries only the queue-visible rung.

**The cohort was a named population here until 2026-08-09 and is deliberately no longer one** — the composition it stated had since been re-ruled, so the sentence contradicted the ruling it was meant to carry; the fix is to name the owner rather than restate a ruling this file does not hold.

**Expected FIRST FINDING, not a precondition:** today's quick start is curl, sha256sum, tar and `bash … init` from a repository root; macOS needs GNU bash and coreutils by adopter action; native Windows needs Git for Windows. That is why the merged channel gives the installer no delta — those host-floor facts are an output of the observation, not an input.

**Refused, grounds carried forward:** parking behind `native-windows-bash-floor` (landed 2026-09-18) or the git-only-floor discharge (the trigger is what the preview measures); the icebox (the highest cost-while-deferred in the intake).

**Held deferred 2026-09-26 at scope, operator direction lead-relayed (not a ruling):** re-offered at the next scope, after `front-door-demo-unreachable` and `demo-catches-a-done-claim` repair the front door the cohort would meet. Both landed 2026-09-26 on master, but the published release lacks the `demo` verb, so an installing cohort meets the repaired front door only from the next tag. **Held again 2026-09-26 at front-door-release's scope, operator direction lead-relayed (not a ruling):** deferred until v0.26.0, which this iteration's close cuts on the same direction, publishes; it then re-promotes on the first observed install, as stated above.

**Consult ruling, 2026-09-27, superseding the 2026-09-25 recommendation to promote next:** the cohort was re-ruled in the brief — one observed pre-submission install, then the installers the catalogs and the plugin marketplace send — so this entry runs *through* the two ecosystem units the ledger's `catalog-then-plugin` ruling sequences first, not ahead of them. The observed install is owed before the catalog submission; the rest re-promotes on the first observed install, as stated above.

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

**Standing exclusion — `lead, own-authority` 2026-09-10 through the lead's message channel, at `host-resolution-fail-open-cut`'s scope:** the worklist reads the retired slug and not the live CI leg `install-smoke-sh-windows`, so it scores this entry false-eligible; [icebox-trigger-blind-to-retired-carrier](#icebox-trigger-blind-to-retired-carrier) owns that predicate defect, DECLINED as a rider then with the exposure accepted in writing.

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

[cost: session/high] [surface: lifecycle-kit]

the per-SPEC remainder of `spec-tier-brevity-pass`'s three moves (run-on structure, archaeology, restatement), outside the five sections that entry landed. In the filing profile's order, by size: gate-sdk landed in eight slices, as `gate-sdk-framework-brevity`, `gate-sdk-remainder-brevity`, `gate-sdk-porting-brevity`, `gate-sdk-native-brevity`, `gate-sdk-runner-brevity`, `gate-sdk-meta-gate-brevity`, `gate-sdk-tooling-brevity` and `gate-sdk-tail-brevity`; then lifecycle-kit (61k; its §templates/stages/ and §templates/lead.md landed as `lifecycle-template-brevity`), installer (48.5k, the smoke excepted; its contract sections landed as `installer-contract-brevity`), guard-kit (48.3k), delegation-kit (43.9k), canon-kit (39.1k), queue-kit (26.5k), drift-kit (22.1k), context-kit (19.8k), evidence-kit (14.9k), site-kit (9.5k), then doctrine-kit's DOCTRINE.md and SPEC.md.

**Deliverable:** the three moves applied SPEC by SPEC in that order, under the gates the first slice landed, `check-prose-bounds` and `check-provenance-seam`'s dated arm; one SPEC, or a batch of the small ones, per iteration, and gate-sdk in slices, since no iteration passes it whole. Not a wholesale cut: a contract sentence stays.

**Split ten times, 2026-09-25 to 09-27 at scope, each on an operator direction lead-relayed (not a /consult ruling):** gate-sdk's framework, remainder, porting, native-contracts, runner, meta-gate, tooling and tail slices each left as their own debt entry, all since landed; the ninth, installer's contract sections, left ahead of lifecycle-kit because the platform-prerequisite-floors theme rewrites them; the tenth is lifecycle-kit's template sections.

**Cost while deferred:** paid by every session that opens a section not yet passed and every adopter who reads one on the site. Filed 2026-09-25 at scope, split from spec-tier-brevity-pass on an operator direction, lead-relayed; the profile and sampled tables are in that entry's filing commit.

### prune-set-matches-walk-root-ancestors

[cost: event/high] [surface: context-kit] [recurrence: 2026-09-25]

the walk's prune set matches path components anywhere in an absolute path, not only below the walk root: `path_pruned` in `native/src/emit/mod.rs` tests `/<leaf>/` against the full path, and the default set carries `target`, `build`, `dist` and `worktrees`. A consumer whose checkout sits under a directory carrying any of those names gets an empty md and pub index, silently.

**Deliverable:** prune relative to the walk root, a fixture whose root path carries a default leaf, and the boundary stated at context-kit/SPEC.md §Layout and configuration.

**Cost while deferred:** an adopter under `~/build/` or `~/dist/` sees the index arms return nothing and no red says why. Filed 2026-09-02; returned from the icebox 2026-09-25 by consult, the walk re-read and the match still absolute.

### consumer-guard-rule-coverage

[cost: event/high] [surface: guard-kit] [recurrence: 2026-09-25]

consumer-only guard rules are untested: the three destructive rules in `scripts/guard-rules.sh` (`--no-verify`, the harness temp path, `git clean -x`) have no test case, and guard-kit ships no lane in which a consumer tests its own rules.

**Deliverable:** a consumer-rule test lane in the guard test runner (`--run-guard-tests` over a consumer rules file with its cases), this repo's three rules covered, and the lane named at guard-kit/SPEC.md §The generic ruleset.

**Cost while deferred:** the demonstration tree's most destructive guards are the ones nothing verifies, and an adopter writing a rule has no way to prove it fires. Filed 2026-08-13; returned from the icebox 2026-09-25 by consult, the case files re-checked at zero.

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

### gate-tests-suite-identity-in-evidence

[cost: iteration/low] [surface: evidence-kit] [recurrence: 2026-09-25]

the evidence manifest's digest covers the suite's log bytes only (`run_validate.rs`), and the success line names no kit, so two suites with identical output share one hash while evidence-kit/SPEC.md §Baseline manifest says the digest "pins which run".

**Deliverable:** the suite identity (kit and runner arguments) folded into the digest or carried beside it, and the SPEC's claim brought to what the digest proves.

**Cost while deferred:** the attestation payload the paid rung would countersign cannot distinguish two suites. Filed 2026-08-01; returned from the icebox 2026-09-25 by consult, the hash input re-read.

### recurrence-declaration-grammar-ungated

[cost: iteration/low] [surface: queue-kit] [recurrence: 2026-09-25]

no gate checks a `[recurrence:]` array's date shape: `recurrence_dates` in `native/src/queue.rs` drops a token that is not a date silently, so a mistyped stamp undercounts the scope pre-emption threshold and the icebox age limb.

**Deliverable:** a `check-queue-hygiene` axis refusing a malformed recurrence token, with a bad fixture.

**Cost while deferred:** a counted rule fires late on a typo nothing reports. Filed 2026-08-26; returned from the icebox 2026-09-25 by consult, the parser re-read.

### one-motion-commit-race-remains-open

[cost: session/low] [surface: CLAUDE.md] [recurrence: 2026-09-25]

CLAUDE.md offers "stage and commit in one motion" as the shared-index remedy, and the filing measured that `git add … && git commit` still races a concurrent stage; `git commit -o <paths>` (the only-paths form) closes it and is steered nowhere.

**Deliverable:** the always-loaded line names the only-paths form, the guard steers `add`-then-`commit` to it, and the delegation protocol's shared-index bullet agrees.

**Cost while deferred:** every session reads a remedy that does not close the race it is offered for. Filed 2026-08-27; returned from the icebox 2026-09-25 by consult, the line re-read.

### nested-battery-env-inheritance-invisible

[cost: event/low] [surface: evidence-kit] [recurrence: 2026-09-25]

a smoke that re-runs the battery inside its sandbox inherits no evidence-kit scoping and reads clean: `installer/consumer-smoke/run-smoke.sh` re-executes batteries in three places with no `EVIDENCE_KIT` reference, so a scoped nested run can record `verdict=clean` for a battery the outer run never scoped.

**Deliverable:** the nested run inherits or refuses the scope, and a fixture pins the refusal.

**Cost while deferred:** a false clean in the evidence record. Filed 2026-08-18; returned from the icebox 2026-09-25 by consult, the smoke re-grepped.

### battery-timing-file-overwritten-by-only-run

[cost: event/low] [surface: drift-kit] [recurrence: 2026-09-25]

the runner writes `gate-timings.txt` from whatever subset ran (`runner.rs`), so a `--only` run overwrites the battery's timing file, and `kpi-gate-runtime` reports the subset as the battery total with no partial-sample check.

**Deliverable:** the runner marks a filtered run, the KPI reads the mark, and a fixture pins the refusal to sum a subset.

**Cost while deferred:** a confident wrong number on an evidence page. Filed 2026-09-07; returned from the icebox 2026-09-25 by consult, both sites re-read.

### gate-timing-baseline-comparability

[cost: once/low] [surface: drift-kit] [recurrence: 2026-09-25]

`.workflow/gate-timing-baseline.txt` has no reader in `native/src` or `scripts`; its named trigger, a second substrate port, has fired.

**Deliverable:** a comparer arm, or the file retired from the workflow directory with its declaration.

**Cost while deferred:** a tracked baseline nothing compares against. Filed 2026-08-02; returned from the icebox 2026-09-25 by consult on the fired trigger.

### site-health-probe-no-retry-on-transient

[cost: event/low] [surface: site-kit] [recurrence: 2026-09-25]

the shipped `site-kit/templates/site-health.yml` takes one curl sample and files an issue on a single non-200; a transient is a wrong red on a public tracker. Sibling of [site-health-issue-venue-unwanted](#site-health-issue-venue-unwanted), whose subject is the venue; this one is the sample.

**Deliverable:** a bounded retry before the failure path, in the template and the copy.

**Cost while deferred:** one transient files a public issue. Filed 2026-08-27; returned from the icebox 2026-09-25 by consult, the template re-read.

### scratch-auto-allow-no-decoration-steer

[cost: event/low] [surface: guard-kit] [recurrence: 2026-09-25]

the guard declines a chained scratch append (`grants.rs`) and steers only allowlisted leads to the decorated form, so the most frequent write the protocol asks for costs a permission decision with no steer for everyone else.

**Deliverable:** the decline names the granted spelling, with a fixture.

**Cost while deferred:** a permission prompt per journal line. Filed 2026-09-04; returned from the icebox 2026-09-25 by consult, the grant re-read.

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

### roadmap-horizon-motion-unowned

[cost: event/low] [surface: lifecycle-kit]

a roadmap horizon moves only when a `/consult` happens to re-tag it. The consult binding (`.claude/commands/consult.md`) reads ROADMAP.md only as a projection and names no tag reconciliation; the roadmap arm prints an empty horizon as information (queue-kit/SPEC.md §The roadmap arm); scope reads the tag only as a ranking input; and no drain retires a tagged entry. `check-roadmap-fresh` compares the projection with the tags, never the tags with direction, so it stays green throughout. Attested: the 2026-09-25 consult refresh ranked three entries and tagged none, and now/ stood empty until an operator-directed re-tag at non-gate-arm-contract's close.

**Deliverable:** close raises a consult-owed signal on a vacant now/ horizon or a landed tagged entry, feeding [consult-inbox](#consult-inbox), and the consult binding reconciles tags on any direction change.

**Cost while deferred:** the public roadmap lags direction until an operator-started consult notices. Filed 2026-09-26 to the gap inbox by the lead on an operator question; promoted 2026-09-26 at non-gate-arm-contract's close: →fix fails because the signal is new mechanism on two surfaces. Re-verified at the drain: the consult binding's one roadmap hit is the projection sentence, and `--emit roadmap` printed now/ empty before the re-tag. Owner lookup: `horizon`, `consult-owed` in this file — no entry about motion.

### consult-inbox

[cost: event/high] [surface: lifecycle-kit]

nothing queues work owed to `/consult`. Its only automatic trigger is `--emit ruling-staleness`, which the scope and close bindings read and the consult binding does not, so an operator-class reversal a stage finds, a threshold entry declined twice, a direction question misnamed as a ruling, or a vacant roadmap now/ horizon is relayed live or lost.

**Deliverable, on an operator direction (2026-09-26, lead session, not a ruling):** a separate committed inbox drained only by `/consult`, each item ruled, re-classed back to the queue, or discarded with cause; the consult binding's entry-reading adds it after TRAJECTORY.md; the session-context hook surfaces its count, advisory and never blocking a stage. Items are public-safe; one needing private context points at a private-brief section rather than restating it. It stays apart from the gap inbox because the drain owners differ: close cannot rule a consult item, so a mixed inbox would stall close's drain. Open for /spec: age escalation, and whether close forwards consult-class gap bullets. First consumer: [roadmap-horizon-motion-unowned](#roadmap-horizon-motion-unowned)'s signal.

**Cost while deferred:** a consult-owed item survives only if a live session relays it. Filed 2026-09-26 to the gap inbox by the lead; promoted 2026-09-26 at non-gate-arm-contract's close: →fix fails because a new committed surface with new governed names owes an amendment. Re-verified at the drain: `ruling-staleness` appears in the scope binding and the close template, not in either consult surface. Owner lookup: `consult inbox`, `consult-inbox` in this file — none.

### consult-inbox-drain-trigger

[cost: event/low] [surface: delegation-kit] [blocked-by: consult-inbox]

the consult inbox needs a drain trigger and a status cue, on an operator direction (2026-09-26, lead session, not a ruling). (a) A status-line counter for the consult inbox, and optionally the gap inbox, through delegation-kit's statusline counter group, which takes its counts from the queue-counts arm today (`native/src/hook/statusline.rs`); the inbox path comes from a knob, since delegation-kit may not name a lifecycle-kit path. (b) The two consults split by trigger: an operator-started consult stays interactive, and the lead dispatches one for inbox items only while no stage session is live, because a consult writes TRAJECTORY.md and the queue. The dispatched consult re-classes or discards an item alone, with cause, and escalates a ruling-class item to the lead as a four-header block for the operator. Batching thresholds (count or age) are consumer-bound, off by default.

**Deliverable:** both halves, with the open question stated in lifecycle-kit/SPEC.md §The steering vocabulary rather than assumed: whether an operator answer relayed through the lead into a dispatched consult is a ruling.

**Cost while deferred:** the inbox, once it lands, is drained only when an operator remembers it. Filed 2026-09-26 to the gap inbox by the lead, extending the consult-inbox bullet; promoted 2026-09-26 at non-gate-arm-contract's close: →fix fails on the same amendment ground. Re-verified at the drain: `statusline.rs` reads `queue_counts::emit`. Owner lookup: `statusline counter`, `counter group` in this file — none.

### gate-output-contributor-door

[cost: event/low] [surface: guard-kit] [recurrence: 2026-09-27]

a gate's printed finding or help line reaches an adopter's installed tree, yet about fourteen `println!`/`format!` sites under `native/src/gates/` still name `bash gate-sdk/bin/run-gates.sh` as the command to run (the install-lifecycle and install-doctrine remedies, the roadmap and value-rollup regenerators, the file-gap, file-survey and cite-survey arms, among them); an adopter's door is the binary `GATE_SDK_NATIVE_BIN` names. `check-door-binding` reads kit READMEs, templates, `lib/` and `bin/` and never a compiled gate's strings, so nothing reds.

**Recurred 2026-09-27, observed in a consumer:** in a scratch prose install of v0.26.0, `check-graph`'s red printed its hook and graph regenerators in the bash spelling (`native/src/gates/graph.rs`, three sites), while `./scripts/checkwright-gates --emit git-hooks --write` and `--emit graph` ran there. A starter or prose install owes no bash (installer/SPEC.md §Requirements), and run-gates.sh needs bash 4.3, which stock macOS lacks, so the remedy is unrunnable on such a host, not only misrouted.

**Deliverable:** each site re-pointed at the binary `GATE_SDK_NATIVE_BIN` names, or declared contributor-facing, and a check-door-binding assertion over gate-module output strings holding it.

**Cost while deferred:** an adopter following a red's remedy runs a path their tree lacks. Filed 2026-09-26 to the gap inbox by the done-claim-demo build (check-evidence-manifest's assertion-C remedy, fixed at the drain); promoted 2026-09-26 at its close: →fix fails because the holding assertion is new mechanism and some sites (the prose-bounds worklist, the smoke-entry guard) need a contributor-or-adopter call each. Re-verified at the drain: `git grep 'run-gates.sh' native/src/gates` over `println!`/`format!` lines returns fifteen sites before the fix. Owner lookup: `door`, `run-gates.sh`, `remedy` in this file — none live; owner guard-kit/SPEC.md §check-door-binding.

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

### tier-only-rank-out-unruled

[cost: iteration/low] [surface: lifecycle-kit]

the audit roster's survey-engagement class does not say whether a scope survey that ranks a body-read entry out on the scope template's cost-tier order alone (session before iteration, high before low) has engaged the entry's self-declared strongest ground under limb (a). The class calls an entry ranked out on a blanket call alone a finding, while the template's rank order makes a tier sentence a sufficient rank-out. gate-sdk-tail-docs-standard's close met the case on heterogeneous-agent-delegation, whose demand-attested and live-lever grounds its survey left unengaged: the delegated reader called it a hit, and the close declined it on the predecessor sweep's precedent.

**Deliverable:** the reading ruled and stated once, in the class's scope line and in lifecycle-kit/templates/stages/scope.md's counter-evidence paragraph: either a tier-only rank-out counts as engagement, or a body-read entry's strongest ground is weighed in the survey record before it ranks out.

**Cost while deferred:** each close's survey-engagement sweep re-judges tier-only rank-outs and can reach opposite verdicts. Filed 2026-09-27 to the gap inbox by gate-sdk-tail-docs-standard's close; promoted 2026-09-27 at the next iteration's scope: →fix fails because the specs leave the reading to precedent, and a ruling settles it. Re-verified: `.workflow/audit-roster.txt`'s survey-engagement row states the blanket-call finding, and its `declined:` field records the tier-ground decline. Owner lookup: `survey-engagement`, `rank-out`, `cost tier` in this file — survey-engagement-residue-untracked and survey-engagement-trigger-narrower-than-its-class, both DISTINCT (gitignored residue; non-scope surveys); owner lifecycle-kit/templates/stages/scope.md.

### truncation-reclaim-residue

[cost: iteration/low] [surface: delegation-kit]

delegation-kit/SPEC.md declares a truncation reclaim, `reclaim=: > <log>`, for both of its advisory close surfaces, `.workflow/subagent-stop-liveness.log` and `.workflow/wait-primitive-evidence.txt`, while gate-sdk/SPEC.md §The workflow directory rules that a capture log a close reads before draining drains by rotation through `--emit capture-drain`, never by truncation: a truncate after the read erases every line appended between the two. guard-kit's and drift-kit's capture logs already declare the rotation.

**Deliverable:** both declarations moved to the `capture-drain` reclaim, and the close's read taken off the drain file.

**Cost while deferred:** every close's reclaim of those logs can erase lines a live session appended after the read, and the truncation spelling compounded with other calls is what the harness classifier denied as audit-log tampering. Filed 2026-09-27 to the gap inbox by gate-sdk-tail-docs-standard's close; promoted 2026-09-27 at the next iteration's scope: →fix fails because the move changes the close's read surface for two logs. Re-verified: `grep 'close-surface:'` over the kit SPECs shows the two truncation reclaims beside three `capture-drain` ones. Owner lookup: `reclaim`, `capture-drain`, `truncat` in this file — only the icebox's [close-surface-reclaim-uncoupled-from-read](#close-surface-reclaim-uncoupled-from-read), DISTINCT (whether the row was read, not the reclaim's form); owner delegation-kit/SPEC.md.

### truncation-compound-unsteered

[cost: event/low] [surface: guard-kit]

shell-guard splits a compounded emitter write out of a compound (rule `emitter_write` arm (a)) but lets a `: >` truncation compound through, so a call chaining an exact allow entry such as `: > .workflow/subagent-stop-liveness.log` with other allowlisted calls matches no single entry and is decided out of band, with no steer. Rule `allowlist_chain` reads only the leading statement, and `:` is no `GUARD_KIT_APPEND_BINS` member.

**Deliverable:** a compounded statement that alone matches an exact committed allow entry steered to its own call, in whichever rule owns the shape, with a `good/`+`bad/` fixture pair.

**Cost while deferred:** a session compounding a granted truncation meets an out-of-band decision, which a classifier may deny. Filed 2026-09-27 to the gap inbox by gate-sdk-tail-docs-standard's close; promoted 2026-09-27 at the next iteration's scope: →fix fails because choosing the owning rule is a design call on a shipped guard. Re-verified: a hook payload `grep -c x TASK-QUEUE.md; : > .workflow/subagent-stop-liveness.log` exits 0 with no steer, while the same compound with `echo hi >` is steered. Owner lookup: `compound`, `truncat` in this file — only the icebox's rejected-compound-commit-relabel, DISTINCT (a commit retry); owner guard-kit/SPEC.md §The generic ruleset.

### queue-write-side-verb

[cost: iteration/low] [surface: queue-kit] [recurrence: 2026-09-27]

the queue has gates and read arms but no write commands, so every restructure (promote, Done move, icebox, de-icebox, defer, split, recurrence stamp) is a hand edit the gates check only after the fact, and an entry's history (filed, promoted, iceboxed, returned) is reconstructed by hand from git. Filed 2026-08-12 on five throwaway queue scripts measured in one iteration; iceboxed 2026-09-11 as machinery-class. The lead measured 1293 commits touching TASK-QUEUE.md in the month to 2026-09-27, reconstructed entry histories by hand about six times in one session, and saw a scope session misread one. Operator direction, 2026-09-27 (lead session): such commands would be beneficial.

**Deliverable:** queue-kit write commands for that operation set, with the queue gates as their post-check, and a `queue-history <slug>` read printing each transition's date and commit.

**Cost while deferred:** every queue restructure is a hand edit, and each history question costs several git calls with a misread risk. Re-filed 2026-09-27 to the gap inbox by platform-prerequisite-floors' lead, widened from one write verb to the operation set and a history read; returned from the icebox at its close on a judged recurrence: →fix fails because the commands are new governed names. Re-verified: `git log --since=2026-08-27 -- TASK-QUEUE.md` lists 1291 commits at the close, and the evicting commit bd3dab89 holds the original body. Owner: queue-kit/SPEC.md §Per-component contracts.

### manual-operation-spend-channel

[cost: iteration/low] [surface: drift-kit]

no measurement channel attributes a session's spend to repeated manual operations, so the close economics pass cannot surface a tooling opportunity. The stage-economics log prices spend per stage and tier, the overhead meter measures always-loaded bytes, prompt-friction sees only Bash calls that prompt (so Edit and Write queue edits are invisible to it), and knowledge-friction is self-reported. [queue-write-side-verb](#queue-write-side-verb)'s evidence came from an ad-hoc look at `.tmp/`. Operator expectation, 2026-09-27 (lead session): the economics analysis every close runs should surface such opportunities.

**Deliverable:** a channel attributing repeated manual operations (tool-call shapes, hand edits of one surface) to sessions, and a close economics read listing the top candidates.

**Cost while deferred:** tooling gaps surface only by chance and repeated manual work stays unpriced. Filed 2026-09-27 to the gap inbox by platform-prerequisite-floors' lead; promoted at its close: →fix fails because the channel is new mechanism. Re-verified: `.workflow/knowledge-friction.log` holds no line at this close. Owner lookup: `economics`, `tooling opportunit`, `repeated` in this file — [build-stage-tier-economics](#build-stage-tier-economics) and [queue-tier-label-correction-cost](#queue-tier-label-correction-cost), DISTINCT (tier pricing); owner drift-kit/SPEC.md §The stage-economics meter.

### custom-gate-substrates

[cost: event/high] [surface: gate-sdk]

an adopter writes a custom gate in shell only. The registry resolves a member as a `.sh` or a `.gate` declaration (gate-sdk/SPEC.md §lib/gate.sh), and a `.gate` descriptor dispatches into the published binary, which an install cannot extend. On native Windows an adopter therefore needs Git for Windows' bash to author a gate, although the installer and the PowerShell front end already run under PowerShell; operator direction, 2026-09-27 (lead session): customers may write gates in shell, but on Windows they should be able to write them in PowerShell. And there is no supported path to a Rust gate, while a fork that adds the subcommand and ships its own build can (docs/install.md §Writing your own Rust gates; operator direction, 2026-09-27, lead session). Custom Rust gates are wanted as a capability, with install prerequisites split between shipped native gates, custom shell gates and custom Rust gates (operator direction, 2026-09-27, lead session).

**Deliverable:** the registry resolving further substrates under the output, fail-closed, fixture-pair and self-lint contracts: `.ps1` resolution in the registry, the runner and the generated hook, with a PowerShell lint counterpart to `check-shellcheck`; a Rust path, as an adopter-built executable the registry dispatches or an extension crate; and the install page's prerequisites per substrate. Whether one executable-dispatch shape serves both is spec's.

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

### lead-clause-heading-family

[cost: event/low] [surface: canon-kit]

`check-spec-pointer`'s lead-clause admission holds no citation of a heading family whose titles share a lead clause: over an OpenSpec spec, where every requirement heading reads `Requirement: <name>`, a prose citation `§Requirement: Account lockout` resolves through the lead clause `Requirement` although no such requirement exists. A linked citation is held by `check-md-refs`' anchor check, so the loss is the bare-prose form only.

**Deliverable:** the lead-clause rule calibrated (for instance, admitting it only when the full fragment matches no heading's prefix and the lead clause is unique in the file), with a fixture site in each case of the pair.

**Cost while deferred:** a companion adopter's bare-prose requirement citation dangles unseen. Filed 2026-09-27 to the gap inbox at companion-catalog-extension's spec, reproduced in a scratch OpenSpec tree; promoted 2026-09-27 at its close: →fix fails because the calibration is a verdict change on a shipped gate with more than one candidate rule. Re-verified: `Heading::prefix_of` in `native/src/gates/spec_pointer.rs` admits a boundary-anchored prefix of `lead_clause`, which is `Requirement` for such a heading. Owner lookup: `lead clause`, `spec-pointer`, `OpenSpec` in this file — none; owner canon-kit/SPEC.md §check-spec-pointer.

### worktree-crate-commit-red

[cost: event/low] [surface: gate-sdk]

a session editing the crate in a linked worktree cannot land its commit there. `cargo test --release` reds on `every_registry_member_declares_the_roots_it_walks` and `_the_programs_it_spawns`, which run `check-crate-arms` over its fixtures, and that gate refuses at exit 2 in a linked worktree with no recorded green stamp. And the generated pre-commit hook reds `check-gate-binary-fresh` with `git could not hash the tracked source under native`, while `run-gates.sh --run` over the same staged tree passed that gate a minute earlier.

**Inferred, not run:** the hook's red comes from the index and git-dir variables git exports into a hook meeting `git -C native ls-files` in the freshness stamp.

**Deliverable:** the crate's test arm and the hook green in a linked worktree, or the refusal stated as the contract with the tests pinning it.

**Cost while deferred:** a worktree session's crate commit rests on a main-checkout re-run. Filed 2026-09-27 to the gap inbox as two bullets by the front-door hotfix; promoted 2026-09-27 at companion-catalog-extension's close: →fix fails because the hook cause is unprobed and the test's shape is a contract call. Re-verified: `native/src/gates/crate_arms.rs` refuses in a linked worktree lacking a stamp. Owner lookup: `linked worktree`, `gate-binary-fresh`, `crate-arms` in this file — `windows-fresh-fixture-stub`, DISTINCT (the same test pair, red on Windows for a stub it cannot start); owner gate-sdk/SPEC.md §check-crate-arms.

### piped-install-argument-witness

[cost: event/low] [surface: installer]

no CI leg runs an argument form docs/install.md tells an adopter to type through the piped bootstrap: `.github/workflows/gates.yml` pipes `install.sh` and `install.ps1` with no arguments only, and profiles are exercised only as `init --profile <p>` against the bootstrap directly, which is how the flags-first examples shipped. `check-front-door-verbs` now reds a flag-led route statically, but nothing executes the page's argument examples.

**Deliverable:** a step on each leg piping the served `docs/install.sh` and `docs/install.ps1` with `init --profile starter` beside the bare line.

**Cost while deferred:** a broken argument form reaches adopters with every leg green. Filed 2026-09-27 to the gap inbox by the front-door hotfix; promoted 2026-09-27 at companion-catalog-extension's close: →fix fails because the PowerShell 5.1 script-block leg cannot be witnessed on this host and a red would spend a hotfix push from an allocated budget. Re-verified: the one-liner step runs `curl … | … sh` with no argument. Owner lookup: `piped`, `one-liner`, `irm` in this file — none; owner installer/SPEC.md §Requirements.

### hotfix-agent-definition

[cost: event/low] [surface: delegation-kit]

no tracked agent definition exists for the operator-ruled hotfix path of the scope-gated intake rule. A lead dispatching one picks general-purpose and restates standing policy in the prompt (not a stage, no stage entry, no queue or state writes, one test-and-doc-complete commit, the battery and every reached kit suite, stop on a design question, the gap-inbox disposition): the policy-is-config tell (lifecycle-kit/templates/lead.md §Policy is config, not prose). `agent-dispatch-guard` confines an undeclared type to a worktree, where this repo's hook reds and `--emit file-gap` refuses, so the 2026-09-27 front-door hotfix needed a second dispatch to land.

**Deliverable:** a hotfix agent definition under `.claude/agents/` carrying that policy, declared in `DELEGATION_KIT_MUTATING_TYPES`, and the lead template naming the dispatch shape.

**Cost while deferred:** every hotfix costs a restated prompt and a second dispatch. Filed 2026-09-27 to the gap inbox by companion-catalog-extension's lead; promoted 2026-09-27 at its close: →fix fails because the definition is a new governed name. Owner lookup: `hotfix`, `agents/`, `MUTATING_TYPES` in this file — none; owner delegation-kit/SPEC.md, with [worktree-crate-commit-red](#worktree-crate-commit-red) as the worktree half.

### musl-smoke-build-wrapper

[cost: event/low] [surface: installer]

the consumer smoke's build leg needs a musl standard library: since the musl join, a Linux host maps to `x86_64-unknown-linux-musl`, and a host with a system toolchain and no rustup cannot build it (E0463). The route that works: run the tracked `scripts/ci-build-artifact.sh x86_64-unknown-linux-musl <dir>` inside `docker rust:latest` over a git clone of the tree (not `git archive`, since `build.rs` stamps from git), passing `safe.directory` and chowning the output back; then run the smoke on the host with `INSTALLER_SMOKE_ARTIFACTS_DIR=<dir>`, reusing an artifact only while `native/` has no diff since its build commit and its sha256 matches the sidecar. Only the docker wrapper was scratch, swept at the boundary.

**Deliverable:** a tracked maintainer wrapper yielding that hand-off directory, named in installer/SPEC.md §The consumer smoke.

**Cost while deferred:** each maintainer without a musl target re-derives the route before a local validate. Filed 2026-09-27 to the gap inbox by the front-door hotfix, with an addendum on the recovered route (re-proved at companion-catalog-extension's build, installer smoke 20/20); promoted 2026-09-27 at its close: →fix fails because the wrapper is a new tracked script. Re-verified: this host's sysroot carries only the gnu std, `command -v rustup` finds nothing, and the section names no docker route. Owner lookup: `musl`, `ARTIFACTS_DIR`, `docker` in this file — `install-platform-release-gap` and [musl-dev-binary](#musl-dev-binary), DISTINCT (publishing; the dev binary); owner installer/SPEC.md §The consumer smoke.

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

### musl-dev-binary

[cost: event/high] [surface: gate-sdk]

a maintainer's Linux dev binary could be the static musl build, so the battery and the generated hooks run the bytes adopters install. Operator direction, 2026-09-27 (lead session): filed as a costed idea, not work.

**Inferred, not run:** the prior iteration's spec measurement — battery 139/139 with the musl binary at the canonical path, 15.41s against 14.47–14.89s for gnu (about +4%); crate tests 1196 pass, 4.99s against 4.58s.

**Deliverable:** `build-native.sh` and `check-gate-binary-fresh` building and stamping a musl dev binary, macOS and Windows staying on their native runners.

**Cost while deferred:** the dogfood battery runs a binary adopters on Linux never receive; adopting it, every native rebuild needs docker or a host musl target. Filed 2026-09-27 to the gap inbox by companion-catalog-extension's lead; promoted 2026-09-27 at its close: →fix fails because the operator filed it as an idea, not work. Re-verified: this host's sysroot carries no musl std. Owner lookup: `musl`, `build-native`, `dev binary` in this file — [musl-smoke-build-wrapper](#musl-smoke-build-wrapper), DISTINCT (the smoke's hand-off only); owner gate-sdk/SPEC.md §Porting a gate to the binary substrate.

### site-video-poster-rule

[cost: event/low] [surface: site-kit]

the site has no rule for video, and the operator wants short intro and demo videos (direction 2026-09-27, the implementation delegated to the lead). Lead decision: host them on the reserved YouTube channel, a discovery channel fitting `catalog-then-plugin`'s distribution grounds that keeps video bytes out of every clone; a page shows a local poster image linking out, never an iframe player, so the site keeps zero third-party requests at page load.

**Deliverable:** a page-authoring rule in docs/site-architecture.md admitting only the poster-link form, a gate reding an `<iframe>` or a third-party `src=` in docs pages, and first homes on docs/spec-toolkits.md and the front door.

**Cost while deferred:** a video lands with no rule, and an embed would add the site's first third-party request. Filed 2026-09-27 to the gap inbox by companion-catalog-extension's lead; promoted 2026-09-27 at its close: →fix fails because the gate is new mechanism. Re-verified: `docs/_layouts/default.html` loads only local assets, and `check-docs-render-fidelity` lists `iframe` as a known tag without refusing it. Owner lookup: `video`, `iframe`, `YouTube` in this file — none; owner docs/site-architecture.md §Page-authoring rules.

## Icebox

  Dormant entries, one line each: the cost field said the carry was low, no `[roadmap:]` commitment rides on it, and no named event is waiting to promote it. Still live work — a legal `[blocked-by:]` target, conserved on the way in and on the way back out. The removed body is recoverable from the evicting commit (queue-kit/SPEC.md §The icebox tier).

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

### price-table-roster-coverage-oracle

An unpriced model id reds nothing.

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

## Done

- install-platform-release-gap
- consumer-scratch-unignored
- foreign-spec-lifecycle-unowned
- windows-fresh-fixture-stub

## Lessons Learned
