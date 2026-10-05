# TASK-QUEUE.md — Checkwright work queue

## Iteration: context-kit-tail-publisher-pass

  The lifecycle-kit gates read this header's iteration name and the stage cursor — the last stamp in `.workflow/WORKFLOW-STATE.txt` (lifecycle-kit/SPEC.md §The state machine); queue-kit formalizes the queue format itself and gates this file. One iteration per hardening or roadmap unit; [docs/kits.md](docs/kits.md) maps the kits.

---

## New Features

## Technical Debt

## Deferred

### foreign-project-critique

[cost: event/low] [surface: delegation-kit]

an on-demand critique of the project's strategic weaknesses, run on the top foreign tier (a consumer's expert adapter on `--foreign-run`), then a reflection pass on this harness's top tier that turns its findings into queue and SWOT updates. Operator direction 2026-10-03, lead-relayed (not a ruling). The close-time second-vendor review (`.workflow/audit-roster.txt`) reads one iteration's range and is distinct.

**Deliverable:** the on-demand trigger, the critique prompt, the reflection pass's contract and the SWOT's home, model literals in consumer config. The spec question is the reflection's write path, since work enters only through scope (doctrine-kit/DOCTRINE.md, Scope-gated intake).

**Cost while deferred:** nil while the operator holds no foreign credit to run it; after, strategic weaknesses surface only through the operator's own reading. Filed 2026-10-03 to the gap inbox by delegation-transport-pass' lead; promoted at its close: →fix fails because trigger, reflection and SWOT surface are new mechanism no amendment settled, →forward because the direction is given. Re-verified: no tracked prose names a SWOT or an on-demand foreign critique. Owner lookup: `SWOT`, `critique`, `strategic`, `reflection` in this file — foreign-vendor-critique, DISTINCT (its close review landed this iteration); owner delegation-kit/SPEC.md §The foreign-vendor run, the SWOT's home open.

### stage-executor-binding

[cost: event/low] [surface: delegation-kit]

a per-stage executor binding as consumer config: a consumer picks the harness and model for each stage, say every stage on the master harness and align on a foreign coding agent at a chosen model and effort, offered as a consumer-selectable set with today's all-master-harness posture one member. It extends `DELEGATION_KIT_TIER_MODEL`'s per-class binding to a per-stage executor. Operator direction 2026-10-03, lead-relayed (not a ruling).

**Gated on** [heterogeneous-agent-delegation](#heterogeneous-agent-delegation)'s item (4), stage-contract expression, without which no stage runs on a foreign agent.

**Deliverable:** the stage-to-executor knob, its validator and the lead's dispatch-time read, in delegation-kit/SPEC.md §The tier binding, with fixtures.

**Cost while deferred:** nil while item (4) is open and the operator holds no foreign credit; after, a consumer has no declarative way to put a stage on a foreign agent. Filed 2026-10-03 to the gap inbox by delegation-transport-pass' lead; promoted at its close: →fix fails because the knob is a new name and its prerequisite is unlanded, →forward because the direction is given. Re-verified: the knob roster carries no stage- or executor-keyed delegation-kit knob. Owner lookup: `per-stage`, `executor`, `TIER_MODEL` in this file — heterogeneous-agent-delegation, DISTINCT (item (4) lets a stage run foreign; this chooses which); owner delegation-kit/SPEC.md §The tier binding.

### guard-quoted-operand-words

[cost: event/low] [surface: guard-kit]

Two rules read a command word without its quoting. Rule `rm_tracked`'s bash arm tests the `sq dq hd` skeleton's words (`native/src/guard/rules/reach.rs`, `rm_tracked_reach`), so any quoted operand reaches the tracked test as a mark: `rm 'README.md'` and `rm "README.md"` pass while `rm README.md` blocks, against guard-kit/SPEC.md's rule entry, which blocks an `rm` naming a tracked path. Rule `find_exec`'s steer re-quotes dequoted words (`native/src/guard/rules/tools.rs`, `shell_word`), and the dequoted view keeps an escaping backslash, indistinguishable there from a literal one inside single quotes: `-exec cat a\ b '{}' \;` is steered to `cat 'a\ b'`, a different file.

**Deliverable:** each word read with its quoting known: the skeleton's held word beside the dequoted one, or a reader-owned word that carries it. The tracked test takes a quoted operand literally (`:(literal)`) and an unquoted one as the glob the shell expands. Both steers print the shell's own word. Each rule gets a decision-table row (a quoted tracked operand, a blank-bearing name, a single-quoted regex backslash), and guard-kit/SPEC.md §The reader and its views states the contract.

**Cost while deferred:** a quoted `rm` of a tracked path deletes unstaged, outside the block's steer; a `find_exec` steer naming an escaped blank cannot be followed as printed. Filed 2026-10-03 to the gap inbox as two bullets at guard-ruleset-gate-neutrality-pass' spec; promoted together at its close: →fix fails because an exact word needs per-word quoting provenance that no rule consumes yet, and stripping backslashes, as rule `grant_path_slot` does, breaks the common single-quoted regex steer. That makes it a reader contract with cross-rule reach, the escaped-blank precedent's ground. →forward fails because no ruling is owed. Re-verified by hook probe: both steers as quoted above, and the `rm_tracked` premise widened from blank-bearing names to every quoted operand. Owner lookup: `shell_word`, `quoted operand`, `rm_tracked`, `exact word` in this file — none; owner guard-kit/SPEC.md §The reader and its views.

### shared-tree-stash-steer

[cost: event/low] [surface: guard-kit]

A build session ran `git stash` / `git stash pop` on the shared checkout to rebuild a pre-change binary for a before/after probe. Stash rewrites the shared index and worktree under any concurrent session. Rule `git_mutation_under_producer` blocks `stash` only while a live producer record exists (guard-kit/SPEC.md, its act set), and no rule, agent-execution bullet or stage template steers a before/after probe to a throwaway worktree.

**Deliverable:** a steer from a worktree-rewriting `git stash` on the main checkout to `git worktree add` under the scratch dir, either as a guard-kit block with its decision-table rows or as a delegation-kit/templates/agent-execution.md bullet; choosing between them is the unit's spec question.

**Cost while deferred:** a concurrent session's uncommitted edits or staged paths can vanish or be re-applied under it. Lead-observed once, with the tree intact; the frequency is **inferred, not run**. Filed 2026-10-03 to the gap inbox by the lead at guard-ruleset-gate-neutrality-pass' build; promoted at its close: →fix fails because both shapes add adopter-visible semantics that no amendment settled, a new block verdict or a new standing instruction. →forward fails because no ruling is owed. Re-verified: `stash` sits only in the live-producer act set, and no template text names stash or a before/after probe. Owner lookup: `stash`, `before/after`, `throwaway` in this file — none; owner guard-kit/SPEC.md §The rule roster.

### audit-trigger-mirror-component

[cost: iteration/low] [surface: lifecycle-kit]

`check-stage-entry`'s assertion C reads a generated SPEC mirror (`docs/<kit>/SPEC.md`) as a roster dir, so a single-kit iteration whose amendments name their mirror as a regenerate target reaches two components on that token alone and is refused build entry without an align stamp or a waiver. lifecycle-kit/SPEC.md §check-stage-entry states the behaviour and prices it under C's honest limit, so the over-demand is designed, not a bug.

**Deliverable:** a way for a consumer to declare a generated mirror as the projection of its source component, so the token resolves to the component it mirrors, specified in §check-stage-entry with a fixture over a mirror-naming single-kit amendment; or a ruling that the waiver stays the valve.

**Cost while deferred:** an align dispatch or an operator waiver ask per single-kit iteration whose amendments name their mirror. Filed 2026-10-03 to the gap inbox at guard-kit-write-side-pass' build; promoted at its close: →fix fails because the mirror's resolution is a grammar decision the SPEC settles the other way, →forward because no recorded ruling is reversed. Re-verified: the SPEC's roster-dir paragraph names this exact case. Owner lookup: `mirror`, `assertion C`, `audit-trigger` in this file — none; owner lifecycle-kit/SPEC.md §check-stage-entry.

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

[cost: iteration/low] [surface: delegation-kit] [roadmap: now/ecosystem] [roadmap-summary: Dispatch a stage to any vendor's coding agent, gated identically.]

foreign agents. Cross-vendor stage dispatch: a lead delegating a stage to a foreign coding agent. It cashes the public no-lock-in claim — governance enforced at the git/gate boundary, not by trusting the author; the verification substrate and the shared git-index/HEAD serialization are already agent-neutral. *Remaining, worst-first:* (3) **budget oracle** — N vendor-keyed oracles beside the account-keyed verdict (delegation-kit/SPEC.md §usage-verdict), fed by the vendors' JSONL token-usage events; (4) **stage-contract expression** — the stage-skill prose is not vendor-neutral, and a foreign stage session has no stamp path, since it commits nothing and has no transcript the stamp protocol reads.

**Slices landed**, each in delegation-kit/SPEC.md §The foreign-vendor run and each accepted by one granted live codex audit: the foreign-CLI executor, `--foreign-run` (delegation-tier-binding, 2026-09-30); then (2) the dispatch transport with (1) the escalation resume model riding it, `--foreign-resume` continuing a vendor-held session in its kept clone, a permission request travelling as an escalation in the report, and no adapter binding to a tier class (delegation-transport-pass, 2026-10-03). **Next slice:** (3) the budget oracle or (4) stage-contract expression. **Horizon, operator direction 2026-09-30, lead-relayed (not a ruling):** `now` — delivery has started, and close moves a horizon on actual work.

**Seam ruling (on record):** generic mechanism only — transport, budget oracle and escalation channel are consumer-config seams; a kit literal naming a vendor crosses the provenance seam, the pattern the retired `prose-profile` ruled. Interacts with [hosted-attestation-service](#hosted-attestation-service) and [plugin-harness-reach](#plugin-harness-reach).

**Demand attested (2026-07-23):** the operator holds three foreign-vendor subscriptions and wants read-heavy delegation routed to them for budget headroom. **Its citer** [companion-toolkit-profile](#companion-toolkit-profile) blocks on none of it.

**Attribution, operator direction 2026-09-30, lead-relayed (not a ruling):** foreign harnesses' commit trailers off and the method stated in one place, the README, held by construction (foreign work lands in the delegating session's commit); a harness setting change takes the operator's confirmation.

**Design memory (2026-07-25, 2026-08-02):** JSONL turn events ship on the installed binaries probed, the budget oracle's likely feed. The machine profile (context-kit/SPEC.md §bin/env-probe, local-only) owns which CLIs and how.

**Cost while deferred:** stage-level work still bills one vendor's budget while three subscriptions are held, foreign spend goes unbudgeted, and this design memory ages against fast-moving CLIs. Surfaced 2026-07-17 in the release-in-lifecycle lead session (operator question).

### companion-toolkit-profile

[roadmap: now/ecosystem] [cost: event/high] [surface: lifecycle-kit] [roadmap-summary: Gate a tree whose specs another toolkit's workflow wrote.]

the interop rung's submission half. The build half landed at companion-catalog-extension: `companion/` holds the Spec Kit extension and both recipes, the consumer smoke's companion arm proves them, and docs/spec-toolkits.md is their landing page (companion/SPEC.md). Horizon `now`, operator direction 2026-09-29, lead-relayed (not a ruling): the recent iterations work it toward its submission.

**Deliverable:** the Spec Kit community-catalog submission, filed as the catalog's Extension Submission issue with its `download_url` naming the `checkwright-companion-<version>.zip` Release asset; the extension's README and the landing page then gain the catalog's install form.

**Gated on** a published tag carrying that asset, since the catalog installs from a tagged archive, and on [design-partner-preview](#design-partner-preview)'s observed install. **Also gated, operator direction 2026-09-29, lead-relayed (not a ruling),** on `gate-customer-value-audit`, which landed at gate-sdk-value-pass, and `companion-install-tier`, which landed at companion-tier-delegation-pass: the audit's verdicts decide which gates that tier exposes. **The four preconditions the operator set on 2026-09-27 landed at catalog-submission-preconditions:** `crate-tests-windows-flip`, since red jobs inside a green run read as ignored failures; `linux-glibc-artifacts`; `catalog-landing-docs-polish`; and `spec-toolkits-guarantee`.

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

### record-stamp-encoding-compression

[cost: event/low] [surface: queue-kit] [recurrence: 2026-09-03] [not-icebox-eligible: 2026-09-22 operator-ruled direction of 2026-09-01; evicting it would demote that ruling]

buy discrimination in the queue's record stamps by RE-ENCODING them rather than by adding text, the deferred pool's per-entry budget being what makes added text the wrong trade.

**Operator-ruled 2026-09-01, and the ruling picked a route none of the three escalated options offered.** The escalation asked how to disambiguate two same-day recurrences and proposed, among others, an iteration slug beside the date. That was REFUSED: adding a field spends the budget the format is trying to protect. The worked example given is `YYYY-MM-DD` → a dashless `YYMMDDHHMM` — **the same ten columns, now carrying hour and minute** — which discriminates same-day instances outright and needs no slug. Array notation for multiple stamps is named as a further step, and the direction is stated to generalize to other task-record components rather than to `recurrence:` alone.

**The envelope is one knob now.** queue-kit-unwrap moved the entry cap to code points (`QUEUE_KIT_ENTRY_CAP`) and deregistered `check-queue-wrap` here, so a shorter stamp frees budget but no columns; same-day discrimination is untouched. Weighed at that close and kept: dropping it would re-scope an operator ruling.

**The column axis was witnessed three times before the wrap limit left.** 2026-09-01: `/spec` blocked — `native-gate-port-remaining-corpus`'s lead line could not hold two `spec:` refs under 100 columns. 2026-09-03: the wall forced minting a second host, `drift-kit-bin-port-residue`, for an encoding reason. 2026-09-04: four cuts wanted four refs against a base that held one; four per-cut entries taken instead.

**The gain is the ENCODING, not the list:** the recurrence field is already an array (queue-kit/SPEC.md §The tag algebra), and the duplicate axis exempts tag lines (§check-queue-hygiene), so only same-day discrimination is left to buy.

**The costs, probed rather than listed, because a reader meeting this cold should price it.** Date stamps span `recurrence:` and `ruled:` declarations, filed-prose provenance lines, gap-inbox bullets, survey-record headings and WORKFLOW-STATE stamps; the evidence manifest's trailing date field is OPTIONAL and so is not a cost, correcting the relayed picture. FOUR crate gates carry a date predicate (`stage_evidence.rs`, `stage_entry.rs`, `gap_inbox_neutrality.rs`, `evidence_manifest.rs`, the first two spelling their own `is_date`), and SEVEN shell tools stamp `date +%F` outside fixtures and smoke, none of which stamps a time today. `YY` also drops the century, a deliberate trade rather than an oversight to find later.

**Why design-pending:** the ruling fixes the DIRECTION and not the grammar. Open: which components take the new encoding and in what order, whether the change is a migration or a read-both-write-new window, and what each date-reading gate asserts across it — a wrong answer reds every governed surface at once.

**Cost while deferred:** low and bounded — every entry needing discrimination keeps buying it with text against the entry budget. Filed 2026-09-01 by close under CLAUDE.md §Housekeeping's operator-directed exception; it rides no cut and is no hotfix.

### spec-brevity-residue

[roadmap: now/adoption] [cost: session/high] [surface: evidence-kit] [roadmap-summary: Kit SPECs that state their contracts without run-ons, history or restatement.]

the per-SPEC remainder of `spec-tier-brevity-pass`'s three moves (run-on structure, archaeology, restatement), outside the five sections that entry landed. In the filing profile's order: gate-sdk landed in eight slices (below); lifecycle-kit, installer, guard-kit, delegation-kit, canon-kit, queue-kit, drift-kit and context-kit are finished or split out (the slices below); what remains starts at evidence-kit, site-kit, then doctrine-kit's DOCTRINE.md and SPEC.md; `.workflow/prose-bound-ceiling.txt` holds each file's live finding count. Horizon `now`, operator direction 2026-09-29, lead-relayed (not a ruling): a slice has landed at each recent scope; the per-kit slices stay off the roadmap.

**Deliverable:** the three moves applied SPEC by SPEC in that order, under the gates the first slice landed, `check-prose-bounds` and `check-provenance-seam`'s dated arm; one SPEC, or a batch of the small ones, per iteration, and gate-sdk in slices, since no iteration passes it whole. Not a wholesale cut: a contract sentence stays.

**Split ten times, 2026-09-25 to 09-27 at scope, each on an operator direction lead-relayed (not a /consult ruling):** gate-sdk's framework, remainder, porting, native-contracts, runner, meta-gate, tooling and tail slices each left as their own debt entry, all since landed, as did the ninth and tenth, installer's contract and lifecycle-kit's template sections.

**Cost while deferred:** paid by every session that opens a section not yet passed and every adopter who reads one on the site. Filed 2026-09-25 at scope, split from spec-tier-brevity-pass on an operator direction, lead-relayed; the profile and sampled tables are in that entry's filing commit.

Each on an operator direction lead-relayed (not a /consult ruling): lifecycle-kit's sections above §Per-component contracts, its state-machine tool sections and its remaining sections left 2026-09-28 to 09-29 at the consult-inbox-front-brevity, lifecycle-machine-brevity and native-hook-customer-legs scopes as `lifecycle-kit-front-brevity`, `lifecycle-kit-machine-brevity` and `lifecycle-kit-tail-brevity`; installer's install-surface sections (§The verbs through §The manifest) and its remaining sections other than §The consumer smoke left 2026-09-29 at the companion-technical-gates and companion-adoption-landing scopes as `installer-install-brevity` and `installer-remainder-brevity`; drift-kit's measurement sections left 2026-09-29 at preview-readiness' scope as `drift-kit-measurement-brevity`; guard-kit's front sections, delegation-kit's tier sections and canon-kit's amendment-family sections left 2026-09-30 at the guard-kit-steering, delegation-tier-binding and canon-kit-value-pass scopes as `guard-kit-front-brevity`, `delegation-kit-tier-brevity` and `canon-kit-amendment-brevity`; canon-kit's §Layout and configuration and seven gate sections left 2026-10-01 at install-disposition-pass' scope as `canon-kit-gate-brevity`; context-kit's four feature-edited sections left 2026-10-01 at context-kit-value-pass' scope as `context-kit-feature-brevity`; queue-kit's eleven gate sections left 2026-10-01 at lifecycle-queue-value-pass' scope as `queue-kit-gate-brevity`; guard-kit's tool sections left 2026-10-01 at gate-sdk-value-pass' scope as `guard-kit-tool-brevity`.

delegation-kit's §The delegation model, installer's §The consumer smoke, guard-kit's remainder and §The generic ruleset, and delegation-kit's §The turn-end liveness hook left 2026-10-02 to 10-03 at successive scopes as `delegation-kit-model-brevity`, `installer-smoke-brevity`, `guard-kit-remainder-brevity`, `guard-kit-ruleset-brevity` and `delegation-liveness-brevity`; delegation-kit's rest left 2026-10-04 as `delegation-kit-tail-brevity`; canon-kit's claim-gate sections and rest left 2026-10-04 at the next two scopes as `canon-kit-claim-brevity` and `canon-kit-tail-brevity`; queue-kit's format and arms halves at the two after as `queue-kit-format-brevity` and `queue-kit-arms-brevity`; drift-kit's and context-kit's rests at the next two as `drift-kit-tail-brevity` and `context-kit-tail-brevity`.

### tarball-attestation-observed

[cost: event/low] [surface: installer] [observed-by: publish]

the observation half of `tarball-build-attestation`: the first Release cut after that entry lands carries an artifact attestation on its tarball, and a shell installer run against that Release verifies it where the verifier is present.

**Deliverable:** that observation, read off the `publish` run and one install from the published Release, with any defect it shows filed.

**Cost while deferred:** the attestation step ships unobserved until a release exercises it. Filed 2026-10-04 as a split at scope, because a tag-triggered run cannot be produced by a mid-iteration push (lifecycle-kit/SPEC.md §The state machine). Owner lookup: `attest`, `publish`, `tarball` in this file — tarball-build-attestation, its produce half, and front-door-rehearsal-rule, DISTINCT (a clean-seat rehearsal job, not this one observation).

### contributor-writeback-disposition

[cost: event/high] [surface: CONTRIBUTING.md] [recurrence: 2026-09-25]

CONTRIBUTING.md promises an inbound issue or pull request a disposition within one iteration, and the scope binding caps each lane at five per iteration; the sixth inbound item is promised what the machine cannot deliver. The "pre-launch, dormant" ground has lapsed: releases are public.

**Deliverable:** either the cap carried on the public promise or a lane that honours it, and the disposition record named; CONTRIBUTING.md and the scope binding agree.

**Cost while deferred:** the first contributor past the cap reads a promise the tree breaks. Filed 2026-07-31; returned from the icebox 2026-09-25 by consult, the promise and the cap re-read.

### site-health-probe-no-retry-on-transient

[cost: event/low] [surface: site-kit] [recurrence: 2026-09-25]

the shipped `site-kit/templates/site-health.yml` takes one curl sample and files an issue on a single non-200; a transient is a wrong red on a public tracker. Sibling of [site-health-issue-venue-unwanted](#site-health-issue-venue-unwanted), whose subject is the venue; this one is the sample.

**Deliverable:** a bounded retry before the failure path, in the template and the copy.

**Cost while deferred:** one transient files a public issue. Filed 2026-08-27; returned from the icebox 2026-09-25 by consult, the template re-read.

### release-drain-ordering-contradiction

[cost: event/low] [surface: RELEASING.md] [recurrence: 2026-09-25] [not-icebox-eligible: 2026-10-03 returned from the icebox by consult 2026-09-25; evicting it would reverse that consult]

RELEASING.md's step-4 opener bundles the drain and the close stamp as one commit, and the step's body separates them; a public runbook that contradicts itself.

**Deliverable:** the opener rewritten to the body's order.

**Cost while deferred:** a release session follows whichever half it reads first. Filed 2026-08-06; returned from the icebox 2026-09-25 by consult, the step re-read.

### queue-provenance-restates-git-history

[cost: once/low] [surface: TASK-QUEUE.md] [recurrence: 2026-09-25] [not-icebox-eligible: 2026-10-03 returned from the icebox by consult 2026-09-25; evicting it would reverse that consult]

queue provenance prose restates what `git log` answers; the ruled sweep is small, ten route-phrase hits remaining when re-counted.

**Deliverable:** the ten sites cut to the fact the entry needs, and the writing rule at queue-kit/SPEC.md §The queue format.

**Cost while deferred:** low; paid by every reader of those entries. Filed 2026-09-09; returned from the icebox 2026-09-25 by consult, the count re-run.

### custom-gate-substrates

[cost: event/high] [surface: gate-sdk]

an adopter writes a custom gate in shell only. The registry resolves a member as a `.sh` or a `.gate` declaration (gate-sdk/SPEC.md §lib/gate.sh), and a `.gate` descriptor dispatches into the published binary, which an install cannot extend. On native Windows an adopter therefore needs Git for Windows' bash to author a gate, although the installer and the PowerShell front end already run under PowerShell; operator direction, 2026-09-27 (lead session): customers may write gates in shell, but on Windows they should be able to write them in PowerShell. And there is no supported path to a Rust gate, while a fork that adds the subcommand and ships its own build can (docs/install.md §Writing your own Rust gates; operator direction, 2026-09-27, lead session). Custom Rust gates are wanted as a capability, with install prerequisites split between shipped native gates, custom shell gates and custom Rust gates (operator direction, 2026-09-27, lead session).

**Deliverable:** the registry resolving further substrates under the output, fail-closed, fixture-pair and self-lint contracts: `.ps1` resolution in the registry and the runner, which the hook's `--git-hook` arm dispatches through, with a PowerShell lint counterpart to `check-shellcheck`; a Rust path, as an adopter-built executable the registry dispatches or an extension crate; and the install page's prerequisites per substrate. Whether one executable-dispatch shape serves both is spec's.

**Cost while deferred:** a native-Windows adopter authoring a gate takes on a bash dependency and a second shell dialect, and one wanting a typed, testable gate must write shell or fork. Filed 2026-09-27 to the gap inbox as two bullets by platform-prerequisite-floors' lead, merged here at its close because both ask which substrates the registry resolves beyond `.sh` and `.gate`: →fix fails because each substrate is new mechanism. The Rust bullet's premise, that an adopter cannot write a Rust gate at all, is corrected to the relayed direction above. Re-verified: `registry::resolve` tries `sh` then `gate` per dir and nothing else. Owner lookup: `ps1`, `PowerShell`, `custom gate`, `Rust gate`, `consumer crate` in this file — none; owner gate-sdk/SPEC.md §lib/gate.sh and §The port-candidate criteria.

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

### native-executable-git-hooks

[cost: event/high] [surface: gate-sdk]

the generated pre-commit and commit-msg hooks start through a shell on every OS: each is two lines of POSIX sh that exec the gate binary, and on Windows git needs Git for Windows' bundled sh to start it, emulated on Arm (the delta-6 probe, CI Windows x64 and arm64, 2026-09-29). Operator direction, 2026-09-29, lead-relayed (not a ruling): explore a hook git starts as a native executable, one shape on every OS, never a Windows-only exception.

**Deliverable:** (1) a probe of whether git's hook lookup starts a native executable directly on each OS (the Windows `.exe` lookup inferred from git's source, never run); (2) where a native hook lives, since a per-platform binary cannot be the tracked text hook scripts/git-hooks/ holds: hooks installed untracked with the generated-projection contract and its freshness gate re-pointed at the installer, or a tracked shim kept, which is the shell this removes; (3) the per-commit start cost per OS.

**Cost while deferred:** every commit on Windows starts an emulated-or-bundled shell. Filed 2026-09-29 to the gap inbox by native-hook-customer-legs' lead; promoted 2026-09-29 at its close: →fix fails because the probe is unrun and the home undecided, →forward because the direction is given. Re-verified: scripts/git-hooks/pre-commit opens `#!/bin/sh`. Not a recurrence of `native-hook-dispatch`, which removed the bash dependency. Owner lookup: `native executable`, `hook shim` in this file — none; owner gate-sdk/SPEC.md, with installer/SPEC.md for an untracked install.

### openspec-delta-base-agreement

[cost: event/low] [surface: companion]

an OpenSpec change delta that disagrees with its base spec passes the battery and OpenSpec's own validator, and is caught, if at all, only at archive. Measured on openspec 1.13.2 at companion-technical-gates' spec: `validate --strict` exits 0 on a MODIFIED or RENAMED delta naming an absent requirement and on an ADDED one naming an existing requirement, printing only an INFO line; `archive -y` refuses those three, but `--skip-specs` bypasses the refusal; a REMOVED delta naming an absent requirement passes validate silently and archive takes it as already removed.

**Deliverable:** a generic commit-time gate for heading-set delta agreement, its OpenSpec binding in the recipe, if it clears the value bar `toolkit-overlap-value-bar` landed (companion/SPEC.md §The tiers); it is that bar's first candidate, since OpenSpec owns the check at archive and this gate would re-check it earlier.

**Cost while deferred:** an OpenSpec adopter's technical gates read nothing on a conventional task list, which names no paths, so the companion's code-facing reach there is `check-task-path-claim` alone. Filed 2026-09-29 at companion-technical-gates' spec on an operator direction lead-relayed (not a ruling). Owner lookup: `delta`, `archive`, `openspec` in this file — `companion-spec-to-code-gates`, DISTINCT (it ships the task gates and defers this one), and `toolkit-nav-hierarchy`, DISTINCT (nav labels); owner companion/SPEC.md.

### install-smoke-sh-matrix

[cost: event/low] [surface: .github]

`.github/workflows/gates.yml` spells three unix install-smoke legs as hand-copied jobs, `install-smoke-sh-macos`, `install-smoke-sh-macos-intel` and `install-smoke-sh-linux-arm64`, while every other derived job group is a matrix. The two macOS legs differ only in triple, one `uname -m` probe line and log strings; the arm64 leg is the same shape less the macOS remedy step. The roster step already emits `unix_legs`, read only by `crate-tests-unix`.

**Deliverable:** one install-smoke-sh matrix over `unix_legs` less the baseline triple, the remedy step conditioned on the runner OS being macOS; `install-smoke-sh-linux` stays its own job for its baseline diff and foreign-host containers. **Inferred, not probed:** that nothing reds a declared triple with no smoke leg.

**Cost while deferred:** triplicated YAML held in step by nobody, and a new unix triple needs a hand-written smoke job. Filed 2026-09-29 to the gap inbox by companion-technical-gates' lead on an operator question; promoted at its close: →fix fails because the rewrite needs a push to witness, →forward because no ruling is owed. Re-verified: the three job keys and the `unix_legs` output in gates.yml. Owner lookup: `install-smoke-sh`, `unix_legs` in this file — none; owner installer/SPEC.md §The consumer smoke.

### releases-page-table

[cost: event/low] [surface: docs]

docs/releases.md renders its derived note list as a bare list of version links, which repeats the nav: the page carries `nav_children_key: release`, so the nav already lists every note. Operator question, 2026-09-29, lead-relayed: a stats table instead.

**Deliverable:** a derived table whose columns answer an upgrader, above all whether a release needs action on upgrade (its `gates` or `knobs` role section carries entries, roles per installer/SPEC.md §The upgrade contract); also version, date, bump class and per-section counts, which each note's In brief summary table already states. The note composer writes the counts and the action flag as front-matter keys and a gate holds them equal to the note's sections; parsing sections in Liquid at render time is refused as fragile.

**Cost while deferred:** a reader skipping several versions opens each note to learn which need action. Filed 2026-09-29 to the gap inbox by companion-technical-gates' lead, at the operator's leave; promoted at its close: →fix fails because the keys are new grammar on the notes, →forward because no ruling is owed. Re-verified: the page's `<ul>` loop and 29 notes carrying `release:`. Owner lookup: `releases.md`, `front-matter` in this file — none; owner docs/site-architecture.md, with installer/SPEC.md §The upgrade contract for the keys.

### docs-chrome-page-repeat

[cost: event/low] [surface: docs]

`check-docs-page-repeat` reads page sources and never `docs/_layouts` or `docs/_includes`, so a chrome addition repeating a page's statement passes. Found at `homepage-license-duplicate`, where the footer's license line duplicated docs/index.md's License section; that unit's `check-license-line` widening holds the license instance only.

**Deliverable:** an arm prefixing the layout's and includes' literal text nodes (Liquid excluded) to every page's corpus, so a sentence of eight words or more or a link target stated in both reds; or a boundary note refusing it.

**Cost while deferred:** the next chrome addition can duplicate a page statement unseen until a reader finds it. Filed 2026-09-29 to the gap inbox by companion-adoption-landing's build; promoted at its close: →fix fails because the arm is new mechanism with a false-positive risk to calibrate, →forward because no ruling is owed. Re-verified: canon-kit/SPEC.md §check-docs-page-repeat reads each `CANON_KIT_PAGE_REPEAT_PAGES` page alone and deliberately asserts no repeat across pages, so the arm must weigh that boundary. Owner lookup: `page-repeat`, `_layouts`, `chrome` in this file — [site-video-poster-rule](#site-video-poster-rule), DISTINCT (embeds); owner canon-kit/SPEC.md §check-docs-page-repeat, with docs/site-architecture.md §Page-authoring rules. Surface also canon-kit.

### front-door-rehearsal-rule

[cost: event/high] [surface: lifecycle-kit]

the front-door rehearsal belongs in the methodology, operator direction 2026-09-29, lead-relayed (not a ruling). `front-door-container-rehearsal` found two defects no smoke had caught, a hooked update or profile move refused by `check-gate-tamper` and `init`/`uninstall` hiding git's own failure output, because every smoke installs a tree-packed payload from the author's seat and never the published artifact from a clean one.

**Deliverable, three layers:** (1) a generic kit rule, before an audience-facing event rehearse the published front door from a clean seat and file what it finds, the container, platform and route set being consumer config (doctrine-kit or lifecycle-kit's release step, per the provenance seam); (2) here, a post-publish job in `.github/workflows/publish.yml` installing the just-published Release on clean runners (init, hooks on, a first red, an upgrade from the previous release, a hooked profile move), reaching the macOS and Windows runners a container cannot; (3) a manual agent-walked rehearsal of the routes CI cannot drive (the adoption prompt, the plugin marketplace, the Spec Kit extension), before audience events only, a catalog submission or a partner install, per the operator.

**Cost while deferred:** each audience event risks a front-door defect only a clean-seat install would show; the one-off left macOS, native Windows, arm64 Linux, WSL and the npx, plugin, PowerShell and agent-prompt routes unrehearsed. Filed 2026-09-29 to the gap inbox by preview-readiness' lead; promoted 2026-09-30 at its close: →fix fails because each layer is new mechanism, →forward because the direction is given. Re-verified: `publish.yml` runs roster, build, pack, npm and release and installs nothing after publishing. Owner lookup: `rehears`, `post-publish`, `clean seat` in this file — [design-partner-preview](#design-partner-preview), DISTINCT (the observed install this precedes); owner RELEASING.md for layer 2, the kit rule's home open. Surface also doctrine-kit.

### kit-prose-harness-coupling

[cost: event/high] [surface: gate-sdk]

kit prose couples to the master harness. The kit SPECs and READMEs (guard-kit, lifecycle-kit, delegation-kit, context-kit, plugin) name Claude Code's `CLAUDE_PROJECT_DIR` as the hook anchor, so a customer running another master harness reads a contract bound to one vendor. Operator direction 2026-09-30, lead session (not a ruling): the Claude binding belongs only on the Claude adapter surfaces (guard-kit/templates/settings-hooks.json, the plugin, and the settings and plugin-parity readers of those files); generic prose names the harness's project-dir variable and gives Claude Code's as one binding.

**Deliverable:** that sweep, extended to every other Claude-only name in kit prose.

**Inferred, not run:** the per-SPEC framing the bullet names.

**Cost while deferred:** an adopter on another harness reads every hook contract as bound to Claude Code. Filed 2026-09-30 to the gap inbox by guard-kit-steering's lead; promoted at its close: →fix fails because the sweep spans five kits, →forward because the direction is given. Owner lookup: `CLAUDE_PROJECT_DIR`, `master harness`, `Codex` in this file — [plugin-harness-reach](#plugin-harness-reach), DISTINCT (the plugin package on other harnesses), and [heterogeneous-agent-delegation](#heterogeneous-agent-delegation), DISTINCT (a master harness delegating to other vendors, where this is the master harness swapped); owner gate-sdk/SPEC.md §The adopter constraints.

### release-step-number-cites

[cost: event/low] [surface: RELEASING.md]

RELEASING.md's numbered procedure steps are cited by position, the shape doctrine-kit/DOCTRINE.md Derivation-first names: inside RELEASING.md itself, and from outside it as "RELEASING.md step N" in gate-sdk/SPEC.md, installer/SPEC.md (three sites), docs/site-architecture.md, .claude/commands/close.md, `native/src/gates/install_pin.rs`'s two printed remedies and scripts/guard-config.knobs. Every number is correct today, since the list was re-linked, not reordered.

**Deliverable:** each citation names the step by its bold title or a heading, never its number.

**Cost while deferred:** a step inserted or reordered in RELEASING.md mis-points about fifteen citations, several in shipped kit SPECs and a gate's printed remedy. Filed 2026-09-30 to the gap inbox by canon-kit-value-pass' close positional-reference sweep, outside its range corpus; promoted 2026-09-30 at the next iteration's scope: →fix fails because the sweep reaches a gate's printed remedy, owing the native build and the battery. Re-verified: a grep for `RELEASING.md step <n>` hits every site named above. Owner lookup: `positional`, `step number`, `RELEASING.md step` in this file — none; owner RELEASING.md, with doctrine-kit/DOCTRINE.md Derivation-first. Surface also installer.

### worktree-memory-dir-key

[cost: event/low] [surface: context-kit]

`check-memory-off` derives the memory dir from the repository toplevel, which in a linked worktree is the worktree's own path, while the installed harness bundle keys its per-project memory dir on a canonical working-copy root (its default path reads a canonical-root lookup before the raw path). A linked-worktree session's memory may therefore land under the main checkout's slug, or under one no scan reads.

**Deliverable:** an authenticated probe of where a linked-worktree session's memory dir lands, then the derivation in context-kit/SPEC.md §Layout and configuration matched to it, or a stated reason the worktree's own slug is right.

**Inferred, not run:** the harness's canonical-root keying, read from its bundle; an unauthenticated run writes no memory dir, so the key was never observed.

**Cost while deferred:** a worktree session's memory could accrete where `check-memory-off` never scans. Filed 2026-10-01 to the gap inbox at context-kit-value-pass' spec; promoted 2026-10-01 at its close: →fix fails because the harness behaviour is unobserved and the probe needs an authenticated session, →forward because no ruling is owed. Re-verified: the gate's default derives from the repository toplevel (`git rev-parse --show-toplevel`). Owner lookup: `memory`, `worktree`, `canonical` in this file — none; owner context-kit/SPEC.md §check-memory-off.

### notification-delivery-probe

[cost: event/low] [surface: lifecycle-kit]

the delivery rule under `lead-notification-wake-race`'s remedy, unprobed: does a completion notification queued during a supervisor's turn that makes no tool call wake its session, and does one tool call before the turn end drain it.

**Deliverable:** that probe, run by a lead with the operator present, since a dispatched session's turn end is its session end and cannot observe it; then lifecycle-kit/templates/lead.md §The lead model's wait clause and the agent-execution backgrounding bullet's bound confirmed or corrected against the result.

**Inferred, not run:** that a tool call before the turn end delivers the notification — two stalled transcripts show only an undrained queue at the stop, and one 2026-10-02 lead turn with tool calls received its notification inside the turn.

**Cost while deferred:** the lead's wait clause rests on an inferred mechanism; if it is wrong, a stall still costs hours of idle wall-clock until the operator wakes the lead. Filed 2026-10-02 as a split at companion-tier-delegation-pass' scope, operator direction lead-relayed (not a ruling).

### release-section-collision

[cost: event/low] [surface: gate-sdk]

`GATE_SDK_RELEASE_SECTION_ALIASES`'s validator (`refusals` in `native/src/release_sections.rs`) reds an alias naming a heading the roster already gives, on the ground that the section would read under two roles, but admits two roster roles given one heading and one alias heading given to two roles, which fall to the same ground. gate-sdk/SPEC.md's entry for the knob lists the narrower refusal set.

**Deliverable:** the refusal widened to every heading reached under two roles, across the roster and the aliases, a validator test per case, and the knob entry's refusal list updated.

**Cost while deferred:** a consumer giving two roles one heading gets a note whose section the upgrade contract reads under either role, unrefused. Filed 2026-10-04 to the gap inbox by installer-trust-pass' close, its second-vendor review; promoted 2026-10-04 at the next iteration's scope: →fix fails because a widened refusal reds configurations accepted today, →forward because no ruling is owed. Re-verified: `refusals` checks a repeated role, and an alias against the roster's headings only. Owner lookup: `RELEASE_SECTION`, `two roles`, `alias` in this file — none; owner gate-sdk/SPEC.md §Layout and configuration.

### install-page-split

[cost: event/low] [surface: docs]

docs/install.md runs 320 lines and the operator finds it too long to read: they want it split into sub-pages, and a page-authoring rule, gate-held where possible, that flags an over-long docs page so they need not spot one (operator direction 2026-10-05, lead-relayed, not a ruling). The 2026-09-21 direction set about 150 prose lines around the gated blocks and sanctioned a split without requiring one. `check-surface-ratchet` holds the page at a ceiling row, which stops growth and names no length at which a page splits.

**Deliverable:** the page split into sub-pages, every reader of its marked install blocks re-pointed (the install-smoke legs, and the parity gates whose default is that path); a length rule in docs/site-architecture.md §Page-authoring rules, with a site-kit gate where the rule is mechanical.

**Cost while deferred:** every install reader meets a 320-line page. Filed 2026-10-05 to the gap inbox by drift-kit-tail-crosser-pass' lead, after scope's intake drain; promoted at its close: →fix fails because the split moves load-bearing readers and the length gate is new mechanism, →forward because the direction is given. Re-verified: `wc -l` reads 320 (314 at filing), the ceiling row 21308cp, and site-architecture.md states no length rule. Owner lookup: `install.md`, `page-authoring`, `split` in this file — site-video-poster-rule, DISTINCT (video embeds); docs-code-block-copy-wrap, landed this iteration, DISTINCT (code-block copy and wrap); owner docs/site-architecture.md §Page-authoring rules.

### windows-cfg-msrv-lint-local

[cost: event/low] [surface: gate-sdk]

crate code under `cfg(not(unix))` is linted at the MSRV only by `native-artifacts`' Windows clippy legs: `check-crate-arms` lints the host target, and the `gates` job's Windows step is `cargo check`, which carries no `incompatible_msrv` lint. An `io::Error::other` (Rust 1.74, MSRV 1.71) in `front_end_parity.rs` reached a push at drift-kit-tail-crosser-pass and cost a hotfix push (gates run 37279844257).

**Deliverable:** a contributor-side catch before the push: an opt-in cross-target clippy in `check-crate-arms` where the Windows target is installed, or a pre-push tool. Which, and how it degrades without `rustup`, is spec's: gate-sdk/SPEC.md §check-crate-arms rules the compile-only Windows check no gate, since it needs `rustup` and network.

**Cost while deferred:** a red push and a hotfix push per such slip. Filed 2026-10-05 to the gap inbox at that iteration's build; promoted at its close: →fix fails because a local cross-target lint is new mechanism, →forward because no ruling is owed. Re-verified: the `gates` job runs `cargo check --target x86_64-pc-windows-msvc`, and the only clippy on a Windows target is `native-artifacts`'. One premise fell: the bullet's candidate catcher, a cross-target clippy in the `gates` job, still lands on the push and so saves no red push. Owner lookup: `MSRV`, `clippy`, `cfg(not(unix))` in this file — msrv-move-clippy-arm-coupling (Icebox), DISTINCT (a floor move un-suppressing lints); owner gate-sdk/SPEC.md §check-crate-arms.

### liveness-windows-misread

[cost: event/low] [surface: evidence-kit]

`crate-tests-windows` x86_64's producer-liveness step misread row (b) once (gates run 37279844257): the record named pid 2688, pid-only ground truth read it gone immediately before and after the gate's read, and the gate read it held and exited 1; the next run was green with that code unchanged. evidence-kit/SPEC.md §The producer-liveness lock accepts PID reuse as a fail-closed residual, and the step already brackets the gate's read to admit it, so whatever held 2688 lived only inside that read.

**Inferred, not run:** that the holder was the gate's own process, issued the recycled pid at spawn; `native/src/gates/producer_liveness.rs` does not exclude the reader's own pid.

**Deliverable:** the holder identified, by a re-run logging the gate's own pid beside the record's; then the gate excluding its own pid from a held reading, stated in that section, or the step's verdict admitting the window.

**Cost while deferred:** an intermittent red on an unrelated push, spending a hotfix push to re-run. Filed 2026-10-05 to the gap inbox at drift-kit-tail-crosser-pass' build; promoted at its close: →fix fails because the cause is unobserved and either remedy changes a contract or a witness, →forward because no ruling is owed. Re-verified off the run's log: rows (a), (bp) and (c) read held then free, and (b) read gone around the gate with the gate at 1. One premise narrowed: the bullet's reuse inside the read window is a holder both brackets missed. Owner lookup: `liveness`, `pid`, `reuse` in this file — none; owner evidence-kit/SPEC.md §The producer-liveness lock.

### journal-append-arm

[cost: event/low] [surface: delegation-kit] [recurrence: 2026-10-05]

resume-journal appends are the top hand shape after the gate door in `--emit manual-ops`' first rankings: `printf >>` 47 calls in 6 sessions and `cat >>` 24 in 5 at drift-kit-tail-crosser-pass, every stage. Each spells the path `--enter-stage` printed, and the shell guard splits the write from any other command, so each append is its own call.

**Deliverable:** an append arm writing its operand or stdin to the journal the session's own stamp names, so no session spells the path; homed in delegation-kit/SPEC.md §Resume journal — agent writes, scratch reset sweeps, with the agent-execution template's journal bullet citing it.

**Cost while deferred:** a tool call and a spelled path per journal line, in every stage session. Filed 2026-10-05 to the gap inbox at that iteration's build, off the meter's first run; promoted at its close: →fix fails because the arm is a new governed name, →forward because no ruling is owed. Re-verified: the meter at close ranks `printf >>` second and `cat >>` sixth; the filer's delegation-transport-pass count (`cat >>` 57 in 9) is carried, not re-run. Owner lookup: `journal`, `append` in this file — none; owner that section.

### manual-ops-door-subkey

[cost: event/low] [surface: drift-kit]

`--emit manual-ops` keys a shell call by guard-kit's ranking key, which sub-keys only its multi-command set, so every arm called through the gate binary folds into one key, and two spellings of the binary split it: `./native/target/release/checkwright-gates` 61 calls in 5 sessions and `native/target/release/checkwright-gates` 24 in 1 at drift-kit-tail-crosser-pass, rewrite, scratch-run and emit arms alike. The meter cannot say which arm is repeated. That close ignored both spellings as the door, as `bash gate-sdk/bin/run-gates.sh` already was; the per-arm signal is this entry's.

**Deliverable:** a door-aware sub-key, the door plus its arm word, one key across the door's spellings, in drift-kit/SPEC.md §The manual-operation meter or guard-kit/SPEC.md §scan-prompts; which owner is spec's.

**Cost while deferred:** a repeated arm, the strongest tooling candidate the meter could name, stays invisible. Filed 2026-10-05 to the gap inbox at that iteration's build; promoted at its close: →fix fails because the key is shared with the prompt-friction ranking, so a change reaches two kits, →forward because no ruling is owed. Re-verified: the meter at close ranks the first spelling first and the second seventh; the split across spellings is new at the drain. Owner lookup: `ranking key`, `sub-key`, `manual-op` in this file — none; owner drift-kit/SPEC.md §The manual-operation meter.

### fence-toggle-list-item

[cost: event/low] [surface: canon-kit]

the toggle fence parsers keyed on `spec::is_fence_line` (`native/src/spec.rs`: `docs_mirror`, `manifest_temporal`, `spec_pointer`, `citation_link`, `prose_tells`, `task_path_claim` and the rest, 17 modules) and `check-spec-fence-balance` read a list-item fence opener (`- ```sh`) as no delimiter, while `fence_opening` (`native/src/gates/fence_command_head.rs`) and `check-fence-run` read it as kramdown renders it. An odd count of list-item fences in a manifest-set file reds the balance gate with a misleading "close the unbalanced fence"; an even count passes it and inverts every toggle parser's span between them, so those gates scan fence bodies as prose and skip the prose between.

**Deliverable:** one fence-line reader taking the list-item opener and its indented closer, shared by every toggle parser and the balance gate, with a list-item fence case in each affected gate's fixtures; canon-kit/SPEC.md §The shared spec adapters and §check-spec-fence-balance state the shape.

**Cost while deferred:** latent while no governed doc carries a list-item fence; the first adopter writing one meets a false balance red or silently unscanned prose. Filed 2026-10-05 to the gap inbox at context-kit-tail-publisher-pass' build; promoted at its close: →fix fails because the reader is shared by 17 gate modules whose fixtures each owe a case, and its directive binds it to the awk driver's fence shape, so a change is a cross-gate reader contract; →forward because no ruling is owed. Re-verified: `is_fence_line` strips leading blanks only, and `fence_opening` strips a list-item marker too. Owner lookup: `list-item`, `is_fence_line`, `fence balance` in this file — fence-reader-list-item, landed this iteration, DISTINCT (its deliverable named the `fences_of` readers alone); owner canon-kit/SPEC.md §check-spec-fence-balance.

### trajectory-limits-unstated

[cost: event/low] [surface: drift-kit]

drift-kit/SPEC.md §The published-evidence extractor says the extractor "states plainly that no controlled ungoverned baseline exists", and that the knowledge-friction exclusion is "stated as a limitation on the framing page". `native/src/emit/trajectory.rs` prints neither: its `--human` header is two lines naming neither limit, and no framing page exists, since `docs/evidence-data.md` is the bare table.

**Deliverable:** the two limits stated where a reader of the published evidence meets them, either in the arm's output or on a framing page around the committed projection, or the SPEC's two claims narrowed to what the arm prints; which one is spec's.

**Cost while deferred:** a reader trusting the SPEC believes the published evidence carries its own caveats, and it carries none. Filed 2026-10-05 to the gap inbox at context-kit-tail-publisher-pass' build; promoted at its close: →fix fails because choosing between the arm stating the limits and the SPEC dropping the claim changes asserted behaviour, and the self-referential framing it qualifies is on record; →forward because no ruling is owed for the first branch. Re-verified: the `--human` branch of `emit` pushes the two header lines alone, and no `docs/` page outside the SPEC mirrors names an ungoverned baseline. Owner lookup: `ungoverned`, `framing page`, `trajectory` in this file — benchmark-ab-experiment, DISTINCT (the controlled experiment itself); owner drift-kit/SPEC.md §The published-evidence extractor.

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

### docs-liquid-literal-unseen

A balanced Liquid token a docs page means literally parses and renders blank, and neither check-docs-liquid-parse nor check-docs-render-fidelity sees the loss; no live page carries it, and the fix is a render-side assertion or a token scan outside raw blocks (site-kit/SPEC.md §check-docs-liquid-parse).

### spec-pointer-unstated-literal

Ported gate and emit modules carry spec: comments citing a section for a literal or a port note the section does not state, so a reader following the pointer finds no support for it; a crate-wide sweep would retag, delete or relocate each, and a new instance returns the entry.

### site-video-poster-rule

No rule holds a docs video to a local poster linking out, so a first embed adds a third-party request.

## Done

