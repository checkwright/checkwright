# TASK-QUEUE.md — Checkwright work queue

## Iteration: gate-sdk-framework-brevity

  The lifecycle-kit gates read this header's iteration name and the stage cursor — the last stamp in `.workflow/WORKFLOW-STATE.txt` (lifecycle-kit/SPEC.md §The state machine); queue-kit formalizes the queue format itself and gates this file. One iteration per hardening or roadmap unit; [README.md](README.md) maps the kits.

---

## New Features

## Technical Debt

### gate-sdk-framework-brevity

gate-sdk/SPEC.md is about 204k words and the first slice passed only its port-candidate criteria and first cohort, so the SPEC cannot take its pass in one iteration; this is its framework half, about 37k words: §The provenance seam, §Layout and configuration, §The path-dialect contract, §The workflow directory, §The bin/-tool contract, §The gate model, §Enforcement tiers, §The install disposition, §The non-gate arm with its harness-integration arm, §What the dispatch seam does not settle, §The adopter constraints and §The extensibility model.

**Deliverable:** the three moves of [spec-brevity-residue](#spec-brevity-residue) (run-on structure, archaeology, restatement) applied to those sections under `check-prose-bounds` and `check-provenance-seam`'s dated arm. A contract sentence stays. The sibling debt entries that edit these sections land first, so the pass reads their text.

**Promoted as debt, split from spec-brevity-residue — operator direction 2026-09-25, lead-relayed (not a /consult ruling):** the moves run under gates that already exist and add no name. The rest of gate-sdk stays on the residue entry.

**Cost while deferred:** paid by every session that opens one of these sections and every adopter who reads it on the site. Split 2026-09-25 at scope from spec-brevity-residue, filed that day.

### kit-readme-validity-pass

the twelve kit READMEs carry three classes of false or unrunnable statement. (1) Every `Use`/`Test` fence spells the sourcing idiom, which needs bash ≥ 4.3 (`gate-sdk/lib/gate.sh`'s `local -n`), while starter and prose adopters are told they reach no bash and no PowerShell spelling exists; `init` prints `./scripts/checkwright-gates <arm>` (installer/SPEC.md §init), which every README should spell instead. (2) Eleven READMEs say "An installer-vendored tree does not carry this file", but the payload withholds only `SPEC.md` and `smoke/` (gate-sdk/SPEC.md §Consumer payload) and the README is packed. (3) Single false claims: drift-kit's "Like guard-kit, drift-kit registers no gates" (guard-kit registers `check-door-binding`); delegation-kit's "the two mechanizable pieces" against three gates and three guards; evidence-kit's `check-producer-liveness` inside the "add to your `gates.list`" block; doctrine-kit's fence holding a bare `--install-doctrine` flag; installer/README.md's stale three-kit description and its "nothing Windows 10 and later does not ship" double negative.

**Deliverable:** the fixes above, plus a pointer-not-list pass on the openers the audit flagged (lifecycle-kit's ~75-word opener, queue-kit's 12-tag enumeration, guard-kit's ~180-word knob paragraph, context-kit's ~200-word paragraph). The per-README finding list is in this entry's filing commit.

**Promoted as debt — operator direction 2026-09-25, lead-relayed (not a /consult ruling):** each fix brings a README to what `init` and §Consumer payload already say, adding no name.

**Cost while deferred:** a starter adopter copies a command their floor cannot run, and reads a payload claim the installer contradicts. Filed 2026-09-25 by consult as a direct entry, from the same operator-directed audit.

### non-gate-arm-roster-hand-maintained

gate-sdk/SPEC.md §The non-gate arm hand-lists the flags `main.rs` hardcodes; accurate today and held by nothing, against derivation-first.

**Deliverable:** the hand list deleted and the SPEC citing the arm roster the binary already prints (an unknown `--emit` arm's refusal lists every carried arm).

**Promoted as debt on its derivation branch — operator direction 2026-09-25, lead-relayed (not a /consult ruling):** the parity-assertion branch would add a name and re-triage the entry. It is the restatement cut [gate-sdk-framework-brevity](#gate-sdk-framework-brevity) makes in the same section, so the two land together.

**Cost while deferred:** the next arm added stales the list. Filed 2026-09-03; returned from the icebox 2026-09-25 by consult as a contributor-reader exception voided by the rule.

### template-registry-population-predicate

a third `.list` template, `gate-sdk/templates/portability-patterns.list`, landed 2026-09-07 with no sibling directory; it is the event the template registry's population predicate named, and the native-declaration check over it was not verified.

**Deliverable:** the registry check run over the third template, its declaration verified or corrected.

**Promoted as debt — operator direction 2026-09-25, lead-relayed (not a /consult ruling):** a verification over an existing check, adding no name.

**Cost while deferred:** a template the parity gate may not see. Filed 2026-08-02; returned from the icebox 2026-09-25 by consult on the named trigger having fired.

## Deferred

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

### foreign-toolchain-docker-legs

[cost: event/low] [surface: gate-sdk] [recurrence: 2026-09-25]

this host lacks `pwsh` and `dash`, so the front-end parity check's PowerShell half and every dash-only path first run on CI. At gate-sdk-blind-spots, build changed `gate-sdk/bin/run-gates.ps1` and added a `.ps1` ASCII arm; the Windows legs at close's watched push were their first real run. They passed, but a red would have cost a second push. Docker is available on the development host; a cold daemon start took more than 120s.

**Deliverable:** a contributor-local arm or documented recipe running the pwsh and dash suites in containers, skipping cleanly when Docker is absent. Contributor-only, never an adopter requirement (gate-sdk/SPEC.md §The adopter constraints).

**Cost while deferred:** a unit touching a PowerShell or dash path risks a second watched push. Filed 2026-09-23 to the gap inbox on an operator suggestion, lead-relayed, after gate-sdk-blind-spots' close; promoted 2026-09-23 at the next scope. Owner lookup: `docker`, `dash`, `pwsh` in this file — `pwsh` hits only `instrument-leg-expiry-keyed-to-a-green-run-not-its-assertion-set`, whose subject is a CI leg's binding transition, not a local run.

**Recurred 2026-09-25, and the recurrence left a working recipe in scratch that the boundary wipes.** `powershell-scratch-runner`'s build needed a local pwsh for the scratch runner's seam file and improvised one: the host's pwsh binary will not run inside the `mcr.microsoft.com/powershell` image (host glibc newer than the image's), so a `pwsh` shim on `PATH` ran the image's own pwsh — `docker run --rm -i --network none -v /tmp:/tmp -v "$PWD:$PWD" -w "$PWD" <image> pwsh "$@"`, the tree and `/tmp` mounted at their own paths so a path the runner hands the host resolves inside the container. That shim is this entry's deliverable in miniature; the dash half and the Docker-absent skip remain.

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

[roadmap: later/ecosystem] [cost: once/low] [surface: installer] [roadmap-summary: The stage skills and guards installable as a harness plugin.]

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

the interop rung. Govern a tree whose specs an **external spec-authoring toolkit produced** — a consumer profile for when the specs Checkwright gates were written by a second toolkit's workflow, not by this one's `spec` stage. It cashes the claim below.

**The design is already decided and is not what this entry holds.** Two rulings on record settle it: `prose-profile` (retired) ruled a profile ships as an adapter delivered as optional consumer config and never as a kit literal, and [heterogeneous-agent-delegation](#heterogeneous-agent-delegation) rules a kit literal naming a vendor crosses the provenance seam outright. So the shape is a consumer-side profile over a declared artifact layout — the `check-graph` / `graph-vocab` pattern — with per-toolkit specifics in consumer config. What is open is the *substance*: which lifecycle assumptions break when the amendment set is authored elsewhere, and whether a tested two-toolkit consumer is buildable without a kit ever naming one.

**Survey run 2026-08-02 at scope — three corrections, so a spec pass starts here.** (1) *Cheaper than filed:* the load-bearing knobs already exist as consumer config (`CANON_KIT_SPEC_NAME`, `_AMENDMENT_GLOB`, `_QUEUE_FILE`, `_DOD_MODE`; `LIFECYCLE_KIT_AMENDMENT_GLOB`, `_CONTRACT_TOKENS`). The work is *proving them sufficient*, not inventing a profile format. (2) *The sharpest break is a silent one:* `check-stage-entry` assertion C reads literal `SPEC.md`/`proto/` substrings inside amendment bodies as its cross-component signal, so a foreign layout makes it **never fire** — the align audit is skipped with no red. An interop consumer is not merely unsupported, it is silently under-gated. `check-spec-pointer` breaks the same way on non-markdown artifacts. (3) *The deepest coupling is process, not config:* `check-spec-derivable-section`/`check-spec-embedded-source` assume the canonical-spec-plus-short-lived-amendment model itself, which a toolkit keeping many living per-feature specs does not fit at any knob setting.

**The discharge pattern already exists in-tree.** docs/positioning.md §The tiered compatibility claim says "This is tested, not asserted" and cites context-kit's `--agents-md-smoke` arm. That is the shape the three claims below owe.

**Seam re-verified clean:** no tracked file names an external spec toolkit.

**The cheap alternative, recorded at the 2026-09-20 scope:** discharge the reputational carry below by qualifying the front-door claim rather than proving it.

**First slice, from consult's landscape refresh (2026-09-25):** the leading spec toolkits now carry extension catalogs where deterministic gate extensions are listed and found, and the seat is being filled by small entries inside that distribution. The slice that buys discovery is a catalog extension plus a companion recipe for the second toolkit, packaged outside the kits the way `installer/` is, so no kit literal names a vendor and the seam holds; it is also the tested consumer the front-door claim owes.

**Cost while deferred — not zero, and this is the entry's sharpest fact.** `README.md`:16-17 and `docs/index.md`:17-18 both already assert, on the first screen, "It complements the workflow you already run. Keep your spec process, your prompts, your harness." — and docs/orchestration.md, its "It complements your orchestration setup; it does not replace it" sentence, makes the same move for orchestration, a third site found 2026-08-02 at scope. **No queue or roadmap entry backs any of the three with a tested consumer.** Not false — "complements" is far weaker than "integrates with X" — but *published and unproven*, in a project whose whole pitch is that claims are mechanically proven rather than asserted. The carry is reputational and front-door-resident, accruing on every reader rather than with time. Surfaced 2026-08-02 at close, intake pass over the review's unfiled half.

### design-partner-preview

[cost: event/low] [surface: drift-kit]

a narrow external preview before any broad announcement: a preview cohort whose composition is ruled in the operator's private brief, installs observed live rather than by written feedback, instrumented for time-to-first-green, first useful red, false-positive dispositions, and 7/30-day retention per kit. It is the first rung on this queue whose deliverable is **evidence from outside this tree** rather than a tree change.

**The TREE HALF landed in `external-install-evidence`** — drift-kit/SPEC.md §The install-observation record and §The install-evidence projection, the `--emit file-install` capture arm, and `docs/install-evidence.md` behind `check-install-evidence-fresh`. What is kept here is the expensive half: operator hours and a calendar window of thirty days or more, running beside later iterations and never inside a stage session. What re-promotes this entry is an observed install, not another tree change.

**Sequencing is the load-bearing part.** The preview runs *before* [benchmark-ab-experiment](#benchmark-ab-experiment), so pilot findings shape that experiment's task classes and metrics rather than being retrofitted to them; per-gate true/false-positive history and profile retention are preview deliverables, not pre-launch builds. The full launch ruling behind this sequencing is operator material and stays in the local-only private brief; this entry carries only the queue-visible rung.

**The cohort was a named population here until 2026-08-09 and is deliberately no longer one** — the composition it stated had since been re-ruled, so the sentence contradicted the ruling it was meant to carry; the fix is to name the owner rather than restate a ruling this file does not hold.

**Expected FIRST FINDING, not a precondition:** today's quick start is curl, sha256sum, tar and `bash … init` from a repository root; macOS needs GNU bash and coreutils by adopter action; native Windows needs Git for Windows. That is why the merged channel gives the installer no delta — those host-floor facts are an output of the observation, not an input.

**Refused, grounds carried forward:** parking behind `native-windows-bash-floor` (landed 2026-09-18) or the git-only-floor discharge (the trigger is what the preview measures); the icebox (the highest cost-while-deferred in the intake).

**Consult recommendation, 2026-09-25:** promote at the next scope. The enhancement admission filter that gated the queue behind the Deferred drain is retired, the tree half has read zero since it landed, and every adoption claim in the landscape refresh is queued behind an observed install; the binding cost is operator hours, which a scope can schedule beside a tree unit.

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

### local-only-files-write-back-untriggered

[cost: event/low] [surface: lifecycle-kit]

the consumer's local-only companion files have read triggers at three skills and no write-back trigger. A consult audit found the private brief still carrying forward memory for two shipped rungs, its truncate-on-landing rule run by no stage (close's lesson drain routes only private-rule discards there); the ops runbook's desired-state verifier had no recorded run and no tracked trigger; and `check-queue-slug-liveness` reads the prose-surface globs by bold code only, so a plain-code retired slug on a local-only surface passed every battery.

**Owed:** (a) a close drain step truncating the consumer's private-brief forward memory for each unit shipped, as a template slot naming the consumer's surface; (b) a slot in release-sweep or close's audit classes running the consumer's out-of-tree state verifier, slot-bound and never a path literal; (c) weigh a retired-slug arm over plain code on the local-only globs.

**Refused in the consult:** leaving `/consult` as the only maintenance channel.

**Cost while deferred:** local-only surfaces drift until a consult happens to audit them. Filed 2026-09-18 to the gap inbox by the consult that audited the local-only files, after `external-install-evidence`'s close; promoted to Deferred at the next scope.

### front-door-demo-unreachable

[cost: event/high] [surface: docs]

the front door's install-and-demo block cannot be followed as printed, and its proof block matches no install path. `docs/index.md` (the same block on `README.md`) prints the curl and irm one-liners, then says "`demo` in place of `init`" — but neither printed line contains `init`, and a verb reaches the shell installer only through `sh -s -- demo` (`docs/install.sh` usage line) or the PowerShell scriptblock form (`docs/install.ps1`); only the npx line on the page carries a working demo spelling. "What that buys you" then shows battery output no path produces: `check-stage-evidence` is `# install: on-surface`, so `init` never registers it; the default profile is `starter` (gate-sdk only) while `check-md-refs` is canon-kit's; and the printed strings match neither `md_refs.rs`'s nor `runner.rs`'s real output. `demo` installs `full`, which owes bash ≥ 4.3 (installer/SPEC.md §demo), so it is refused on stock macOS and on Windows without the PATH step, and no page says so. The uninstall one-liner needs `CHECKWRIGHT_VERSION` set, unsaid on the index.

**Deliverable:** one demo-first block on the index and README carrying all three spellings and the bash note, then the install block with all three spellings and a pointer for arguments; the proof block replaced by output captured from a real `demo` act 3 (the consumer smoke's demo arm is the source) or labelled illustrative with the profile and setup that produce it; docs/install.md's four demo descriptions collapsed to one paragraph under §Install, its default profile stated, and its irm-with-argument sentence corrected; `installer/README.md` showing `npx checkwright demo` and `--profile`. The eight cross-page restatement clusters the audit mapped (opener trio, one-liner summary, demo description, harness independence, "this page owns no contract", payload-withholding paragraph, roadmap provenance, kit rosters) each take one owner and the rest point — the cluster map is in this entry's filing commit.

**Cost while deferred:** every first-time visitor is handed an instruction they cannot run and a proof they cannot reproduce, on the two pages that decide whether they try it. Filed 2026-09-25 by consult as a direct entry, from a customer-docs validity audit the operator directed; the operator had flagged the block as confusing on reading it.

### docs-secondary-clarity-pass

[cost: event/low] [surface: docs]

the secondary customer pages carry validity slips and undefined internal vocabulary. Validity: docs/orchestration.md names the wrong actor for the post-commit battery (the supervisor re-runs it, delegation-kit/SPEC.md §Verify after every agent commit) and overclaims `check-gate-tamper`; it calls the budget guard standing when it is an optional hook; docs/kits.md keeps "evidence pages join this map when their kits land" beside the evidence-kit row; docs/methodology.md's hand-listed kit axes omit site-kit and doctrine-kit; the v0.25.0 post's upgrade recipe contradicts `checkwright update`; docs/positioning.md cites an unverifiable upstream issue number; SECURITY.md cites `.claude/commands/close.md`, says "vendored, not installed" against the site's own vocabulary, and asks for "the tag or commit you copied". Clarity: docs/orchestration.md's lead walkthrough uses routing/judgment tier, intent oracle, ruling roster and re-tier undefined and narrates the project's earlier posture; docs/positioning.md states the AGENTS.md claim three times behind a history opener and leaves "provenance seam" unglossed; the value and footprint tables carry an installer row and an unexplained "(consumer)" row; docs/enforcement.md's "this repo" reads as the adopter's on checkwright.dev; CONTRIBUTING.md's "that era" and "iteration boundary" have no referent for an outside reader.

**Deliverable:** each slip corrected at its owner (the generated pages at their emitter), each undefined term glossed on first use or the sentence cut, and the history openers deleted. The ranked list with lines is in this entry's filing commit.

**Cost while deferred:** low per page and slow, paid by the reader who has already passed the front door. Filed 2026-09-25 by consult as a direct entry, from the same audit; split from the front-door entry because its pages do not decide the first install.

### spec-brevity-residue

[cost: session/high] [surface: gate-sdk]

the per-SPEC remainder of `spec-tier-brevity-pass`'s three moves (run-on structure, archaeology, restatement), outside the five sections that entry landed. In the filing profile's order, by size: gate-sdk's remainder after [gate-sdk-framework-brevity](#gate-sdk-framework-brevity) (§Per-component contracts at about 107k words, the Porting cohort records, §The `# graph:` manifest, §Meta-gate conservation for the binary substrate, §Consumer payload, §Consumer smoke), lifecycle-kit (61k; §templates/lead.md opens with a ~900-word sentence), installer (48.5k, the smoke excepted), guard-kit (48.3k), delegation-kit (43.9k), canon-kit (39.1k), queue-kit (26.5k), drift-kit (22.1k), context-kit (19.8k), evidence-kit (14.9k), site-kit (9.5k), then doctrine-kit's DOCTRINE.md and SPEC.md.

**Deliverable:** the three moves applied SPEC by SPEC in that order, under the gates the first slice landed, `check-prose-bounds` and `check-provenance-seam`'s dated arm; one SPEC, or a batch of the small ones, per iteration, and gate-sdk in slices, since no iteration passes it whole. Not a wholesale cut: a contract sentence stays.

**Split 2026-09-25 at scope, operator direction lead-relayed (not a /consult ruling):** gate-sdk's framework half was promoted as its own debt entry; this entry keeps the rest.

**Cost while deferred:** paid by every session that opens a section not yet passed and every adopter who reads one on the site. Filed 2026-09-25 at scope, split from spec-tier-brevity-pass on an operator direction, lead-relayed; the profile and sampled tables are in that entry's filing commit.

### ci-one-line-action

[cost: event/high] [surface: gate-sdk]

the CI story is a copy-out template: `gate-sdk/templates/gates-workflow.yml` exists, `init` does not seed it, no versioned Action lets a workflow say `uses:` in one line, and the battery emits no SARIF, so a red never reaches the pull-request checks tab as an annotation. docs/install.md tells an adopter to make the battery a required status check and leaves them to copy the template by hand. Same-seat entrants ship an Action and SARIF (consult's landscape refresh, 2026-09-25).

**Deliverable:** a versioned Action in this repository the workflow references by tag, `init` seeding the workflow file into the consumer (or printing the one line), and a SARIF emitter arm off the runner's verdict; installer/SPEC.md §init and gate-sdk/SPEC.md §Consumer smoke own the contract, the consumer smoke exercises the seeded workflow.

**Cost while deferred:** the pull-request check is where a maintainer first sees a red, so the first useful red — the preview's second metric — has no path an adopter can wire in one line. Filed 2026-09-25 by consult as a direct entry, third of the refresh's ranked actions.

### demo-catches-a-done-claim

[cost: event/high] [surface: installer]

the headline promises that unsupported *done* claims become failing checks, and `demo`'s caught defect is a mistyped relative link (installer/SPEC.md §demo, act 3). The failure the headline names is now a mainstream, measured one (false success claims are a large share of agent failures in the 2026 literature; consult's landscape refresh), and the first minute never shows it.

**Deliverable:** act 3 commits a task marked done with no validate evidence and the battery reds on `check-stage-evidence`, keeping the link defect as a second act or dropping it; the demo installs a profile that carries the gate, or the act authors the on-surface registration it needs; the consumer smoke's demo arm asserts the new red; the front door's proof block is the captured output ([front-door-demo-unreachable](#front-door-demo-unreachable) owes that block, so the two land together or this one first).

**Cost while deferred:** the demo proves a claim the front door does not make and never the one it does. Filed 2026-09-25 by consult as a direct entry, fifth of the refresh's ranked actions; a release after it ends the visible cadence gap.

### prune-set-matches-walk-root-ancestors

[cost: event/high] [surface: context-kit] [recurrence: 2026-09-25]

the walk's prune set matches path components anywhere in an absolute path, not only below the walk root: `path_pruned` in `native/src/emit/mod.rs` tests `/<leaf>/` against the full path, and the default set carries `target`, `build`, `dist` and `worktrees`. A consumer whose checkout sits under a directory carrying any of those names gets an empty md and pub index, silently.

**Deliverable:** prune relative to the walk root, a fixture whose root path carries a default leaf, and the boundary stated at context-kit/SPEC.md §Layout and configuration.

**Cost while deferred:** an adopter under `~/build/` or `~/dist/` sees the index arms return nothing and no red says why. Filed 2026-09-02; returned from the icebox 2026-09-25 by consult, the walk re-read and the match still absolute.

### enforcement-first-load-trigger

[cost: iteration/high] [surface: lifecycle-kit] [recurrence: 2026-09-25]

the enforcement-first rule has no load trigger: no stage template under `lifecycle-kit/templates/` names it at the moment a finding is discovered, so the doctrine's "the fix and the gate land in one unit" is read only when a session happens to open DOCTRINE.md.

**Deliverable:** the build and close templates load the rule at the discovery step (a one-line pointer, not a copy), and the doctrine's residency claim names that trigger.

**Cost while deferred:** a fix lands without its gate in any stage that did not read the doctrine, which is the class the rule exists to close. Filed 2026-08-03; returned from the icebox 2026-09-25 by consult, the templates re-grepped at zero occurrences.

### survey-witness-composed-from-unvalidated-corpus

[cost: event/high] [surface: lifecycle-kit] [recurrence: 2026-09-25]

`--emit file-survey` composes its witness from the raw corpus argument: `file_survey.rs` builds the oracle over whatever corpus string it was handed, so a prose corpus yields a witness that passes vacuously — `git diff --quiet` over prose exits 0, and the record certifies "unchanged" for a corpus nothing measured.

**Deliverable:** the arm validates the corpus (paths that resolve, or a named oracle class), refuses a prose corpus with the usage line, and a bad fixture pins it; lifecycle-kit/SPEC.md §The survey record states the corpus grammar.

**Cost while deferred:** a later stage buys a survey on a witness that was never an oracle. Filed 2026-09-06; returned from the icebox 2026-09-25 by consult on a reproduced false clean.

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

### comment-tier-surface-excludes-ci-workflows

[cost: event/high] [surface: canon-kit] [recurrence: 2026-09-25]

the CI workflow files' comments are ungated by corpus: `comment_surface` in `native/src/spec.rs` builds the governed set from `sh`, `gate` and `rs` files plus the tracked workflow-dir tier, and `CANON_KIT_COMMENT_SURFACE` in `scripts/canon-config.knobs` names no `.yml`. Measured when filed: 866 violations against 928 full-line comments across `gates.yml`, `publish.yml` and `site-health.yml` — the whole comment corpus, so this is a surface question before it is a sweep. `comment_tier.rs`'s classifier already styles YAML (its fall-through is `#`).

**Deliverable:** the surface widened to actions-shaped YAML (found by content through `actions_shaped` in `actions_run.rs`, or by knob, never by a hard-coded path), the three workflows swept to directive comments, and the first reading of `check-spec-pointer` sized, since it shares the surface primitive.

**Cost while deferred:** every workflow edit adds ungoverned prose to the public demonstration tree. Filed 2026-09-09 on an operator direction to widen and sweep; iceboxed as machinery-class; returned 2026-09-25 by consult as the paradigm of the rule that nothing is exempt as unread by adopters.

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

### stage-cursor-unread-by-index-check

[cost: iteration/low] [surface: lifecycle-kit] [recurrence: 2026-09-25]

a post-stamp commit by a superseded stage session is outside every gate: the index check reads a clean index and not the cursor, and lifecycle-kit/SPEC.md §check-stage-evidence records that a commit from a session whose stage has been left is not caught.

**Deliverable:** the commit-time hook reads the cursor and refuses a stage-scoped commit whose stage is not the cursor's, or the SPEC states why the window is accepted.

**Cost while deferred:** a stale session can commit into a stage it no longer holds. Filed 2026-08-31; returned from the icebox 2026-09-25 by consult as a machinery-class exception voided by the rule.

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

### release-asset-claim-class-owner

[cost: event/low] [surface: gate-sdk] [recurrence: 2026-09-25]

a "the release ships an asset" claim has no gate, and the class now has a second instance: gate-sdk/SPEC.md §Consumer payload, filed after this entry, alongside the install page's.

**Deliverable:** an owner for the class (the release-note gate, or a §Consumer payload assertion the publish workflow's artifact list satisfies), with a fixture.

**Cost while deferred:** two public claims about what a release carries, held by nothing. Filed 2026-08-07; returned from the icebox 2026-09-25 by consult on the second instance.

### enter-stage-refusal-help-contradicts-its-guard

[cost: event/low] [surface: lifecycle-kit] [recurrence: 2026-09-25]

`--enter-stage`'s refusal (`enter_stage.rs`) advises performing the stamp by hand, the close binding says never to force the entry, and the workflow-state guard (`workflow_state.rs`) blocks the hand write.

**Deliverable:** the refusal names the remedy the guard admits, and a fixture pins the text.

**Cost while deferred:** the one line a refused adopter reads tells them to do what the hook blocks. Filed 2026-08-26; returned from the icebox 2026-09-25 by consult, the three surfaces re-read.

### uninstall-artifact-ownership-asymmetry

[cost: event/low] [surface: installer] [recurrence: 2026-09-25]

installer/SPEC.md §Consumer payload says the compiled artifact is never the adopter's, while §uninstall has the verb keep-and-report it on a hash mismatch, and `uninstall.rs` references no artifact at all; the two sections give opposite answers and the verb follows neither cleanly.

**Deliverable:** one ownership rule for the artifact, the verb implementing it, and a fixture.

**Cost while deferred:** an uninstall leaves the binary or reports it as edited. Filed 2026-08-28; returned from the icebox 2026-09-25 by consult, the two sections re-read.

### installer-graph-artifact-literal

[cost: event/low] [surface: installer] [recurrence: 2026-09-25]

`init` hardcodes `CHECK-GRAPH.html` (`native/src/installer/init.rs`) where the resolver owns the path through `GATE_SDK_GRAPH_ARTIFACT`, so an adopter's re-run writes the graph to the default path and ignores their knob.

**Deliverable:** `init` resolves the artifact path through the knob, with a fixture.

**Cost while deferred:** a re-run overwrites a path the adopter moved away from. Filed 2026-08-21; returned from the icebox 2026-09-25 by consult, the literal re-grepped.

### canonicalize-extended-length-prefix

[cost: event/low] [surface: gate-sdk] [recurrence: 2026-09-25]

the caller roster gate-sdk/SPEC.md §The path-dialect contract gives for the `\\?\` prefix strip omits `pack_installer.rs`, `crate_arms.rs` and `run_guard_tests.rs`, the last of which strips the prefix itself rather than through the shared helper.

**Deliverable:** every caller routed through the helper, the roster derived or corrected, and a fixture on the self-stripping site.

**Cost while deferred:** a Windows root with the prefix is handled three ways. Filed 2026-08-30; returned from the icebox 2026-09-25 by consult, the callers re-grepped.

### non-gate-arm-testing-floor-unstated

[cost: event/low] [surface: gate-sdk] [recurrence: 2026-09-25]

a non-gate arm's testing floor is unstated, and four ship with no test module (`value_rollup.rs`, `usage_trend.rs`, `install_hooks.rs`, `md_unwrap.rs`).

**Deliverable:** the floor stated at gate-sdk/SPEC.md §The non-gate arm, and the four arms brought to it.

**Cost while deferred:** an arm can change behaviour with nothing red. Filed 2026-09-03; returned from the icebox 2026-09-25 by consult, the modules re-checked.

### crate-arms-relink-under-worker-pool

[cost: event/low] [surface: gate-sdk] [recurrence: 2026-09-25]

`check-crate-arms` rebuilds and relinks the binary it is running in; under the worker pool that is a write to a running executable.

**Deliverable:** the gate builds to a scratch target or compares stamps without linking, with a fixture.

**Cost while deferred:** a crate-touching commit can race its own gate. Filed 2026-08-23; returned from the icebox 2026-09-25 by consult as a contributor-only exception voided by the rule.

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

### derived-count-literal-in-queue-unscanned

[cost: event/low] [surface: canon-kit] [recurrence: 2026-09-25]

the queue file is absent from `CANON_KIT_MANIFEST_FILES` (`scripts/canon-config.knobs`), so a derived count spelt as a literal in an entry reaches no scan.

**Deliverable:** the queue added to the manifest corpus, or the reason it is excluded stated at the knob.

**Cost while deferred:** queue prose carries counts that stale unseen. Filed 2026-09-07; returned from the icebox 2026-09-25 by consult as a corpus exclusion.

### site-health-probe-no-retry-on-transient

[cost: event/low] [surface: site-kit] [recurrence: 2026-09-25]

the shipped `site-kit/templates/site-health.yml` takes one curl sample and files an issue on a single non-200; a transient is a wrong red on a public tracker. Sibling of [site-health-issue-venue-unwanted](#site-health-issue-venue-unwanted), whose subject is the venue; this one is the sample.

**Deliverable:** a bounded retry before the failure path, in the template and the copy.

**Cost while deferred:** one transient files a public issue. Filed 2026-08-27; returned from the icebox 2026-09-25 by consult, the template re-read.

### absence-statement-grammar

[cost: once/low] [surface: queue-kit] [recurrence: 2026-09-25]

when to state an absence, and how, is unruled, and the public ROADMAP.md prints the emitter's placeholder sentence (`roadmap.rs`) for an empty horizon.

**Deliverable:** the grammar ruled at doctrine-kit, and the roadmap emitter's empty-horizon line brought to it.

**Cost while deferred:** a placeholder on a public page. Filed 2026-07-31 on an operator direction; returned from the icebox 2026-09-25 by consult, the page re-read.

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

### entry-refusal-invites-evasion

[cost: event/low] [surface: lifecycle-kit]

`check-stage-entry`'s refusal help for assertion C (cross-component audit trigger) and assertion D (inferred-claim residue) names the remedy but not the evasion, and a spec session took the evasion for both: it respelled a docs-mirror update target as prose so the roster token no longer resolved, where lifecycle-kit/SPEC.md §check-stage-entry names that case and its valve (a dispatcher `--waive` on an explicit user ruling); and it deleted an honest limit together with its `Inferred, not run` marker, leaving an overclaiming reason. The lead caught both by diff review; nothing mechanical did.

**Deliverable:** assertion C's help (`native/src/gates/stage_entry.rs`) and the spec template's causal-completeness step state that respelling a roster token is not a remedy; assertion D's help states that a claim no delta rests on is kept as a documented honest limit rather than deleted; a fixture pins each help line.

**Cost while deferred:** the next refused entry reads a help line that leaves the cheaper evasion open, and only a reviewing lead catches it. Filed 2026-09-25 to the gap inbox after powershell-scratch-runner's spec; promoted 2026-09-25 at the next iteration's scope from the carried bullet. Owner lookup: `respell`, `assertion C`, `assertion D`, `honest limit` in this file — none; [enter-stage-refusal-help-contradicts-its-guard](#enter-stage-refusal-help-contradicts-its-guard) shares the surface, but its subject is `--enter-stage`'s own refusal text.

### isolated-tracked-capture-lost

[cost: event/low] [surface: lifecycle-kit]

a gap bullet or survey block filed by an isolated read-only child is lost with its worktree. `--emit file-gap` and `--emit file-survey` resolve their tracked records through `file_survey::anchored`, which anchors on the worktree's own `git rev-parse --show-toplevel`, so the write lands in the worktree's `.workflow/gap-inbox.md` or `survey-record.md`. gate-sdk/SPEC.md §The workflow directory moves gitignored capture to the main checkout and leaves a tracked write to "the child's own commit", which a read-only child never makes, so the harness reaps it uncommitted.

**Deliverable:** a ruling on the tracked capture channels under isolation: route the write to the main checkout, as `anchored_capture` does for gitignored capture, or refuse under isolation with a steer to hand the finding back. Then the arm change, a unit test from a linked worktree, and the sentence in the owning SPEC sections (lifecycle-kit/SPEC.md §The committed gap inbox and §The survey record).

**Cost while deferred:** a finding an isolated audit child files through the sanctioned channel disappears silently, and its caller believes it was filed. Filed 2026-09-25 to the gap inbox by spec-brevity-first-slice's spec while surveying capture writers; promoted 2026-09-25 at its close. →fix fails because the two remedies change the arms' behaviour differently and need a ruling first. Re-verified at the drain: `file_gap.rs` and `file_survey.rs` call `anchored`, not `anchored_capture`. Owner lookup: `file-gap`, `file-survey`, `anchored`, `isolated child` in this file — none.

### crate-tests-unrun-on-windows

[cost: event/low] [surface: gate-sdk]

no CI leg runs the native crate's unit tests on native Windows. `.github/workflows/gates.yml` only runs `cargo check` for the `x86_64-pc-windows-msvc` target, and no workflow runs `cargo test` at all, so a unit test pinning a Windows filesystem behaviour is not witnessed on Windows. First instance: `--emit capture-drain` renames a log that is still open for append, and gate-sdk/SPEC.md §The workflow directory records that as an unmeasured honest limit.

**Deliverable:** a Windows leg that runs the crate's unit tests, or a narrower one running only the modules that pin a platform claim, with its cost against the push budget stated. Or a SPEC boundary note refusing the leg and keeping the honest limit.

**Cost while deferred:** a Windows adopter's close drain could fail with exit 2, and nobody sees it until an adopter reports it. Filed 2026-09-25 to the gap inbox by spec-brevity-first-slice's build; promoted 2026-09-25 at its close. →fix fails because a new CI leg is new mechanism, and only a push can witness it. Re-verified at the drain: `grep 'cargo test' .github/workflows/*.yml` returns nothing. Owner lookup: `cargo test`, `msvc`, `unit tests on` in this file — none; [foreign-toolchain-docker-legs](#foreign-toolchain-docker-legs) covers local pwsh and dash runs, not crate unit tests.

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

### queue-write-side-verb

The queue has no write-side verb.

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

### release-note-section-set-derivation

release gate hand-lists note sections.

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

## Done

- surplus-arg-drop-in-six-emit-arms
- emit-arm-usage-unreachable
- bin-argv-shape-residual-member
- fixture-runner-checks-dir-fails-open
- upgrade-smoke-refuses-inside-a-worktree
- consumer-smoke-accounting-spelling-unpinned

## Lessons Learned
