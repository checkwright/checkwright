# TASK-QUEUE.md — Checkwright work queue

## Iteration: —

  The lifecycle-kit gates read this header's iteration name and the stage cursor — the last stamp in `.workflow/WORKFLOW-STATE.txt` (lifecycle-kit/SPEC.md §The state machine); queue-kit formalizes the queue format itself and gates this file. One iteration per hardening or roadmap unit; [docs/kits.md](docs/kits.md) maps the kits.

---

## New Features

## Technical Debt

## Deferred

### benchmark-ab-experiment

[roadmap: later/adoption] [cost: once/low] [surface: drift-kit] [roadmap-summary: A controlled experiment measuring drift with and without governance.]

a controlled A/B trial.

**Cost while deferred:** zero — the self-referential drift-trajectory route already carries the claim this rung would upgrade, and the measurement half it consumes ships independently; what is foregone is an externally-comparable number the project does not currently claim. The controlled differential experiment: same model, same dependent-task series, two arms (ungoverned loop vs Checkwright-governed), drift *accumulation across the series* as the metric — a governance layer's effect, not a model leaderboard number. Metric axis: Drift-Bench's "satisfiable drift". Substrate/vocab primaries: seqBench (arXiv 2509.16866), Drift-Bench (arXiv 2602.02455 — real title "Diagnosing Cooperative Breakdowns in LLM Agents under Input Faults via Multi-Turn Interaction"; the "Decomposing Reasoning Into Failure Types" expansion is confabulated, do not repeat it), Lost-in-Conversation / FlowBench as prior art. Surfaced 2026-07-08 inside adoption-track; split out 2026-07-09 — the self-referential route (drift-trajectory) ships first and this rung upgrades the claim only if demand attests it. The experiment's measurement half — per-stage, per-model, price-weighted token burn off harness transcripts — is the stage-economics meter (drift-kit/SPEC.md §The stage-economics meter); this rung consumes it rather than rebuilding it. Nearer use of that tool: verifying the split-lead posture's savings (lifecycle-kit/templates/lead.md §Economics). Surfaced 2026-07-15 by the per-stage budget analysis that motivated that posture.

### hosted-attestation-service

[roadmap: later/commercial] [cost: event/low] [surface: evidence-kit] [roadmap-summary: Gate runs verified by a neutral party no committing agent can touch.]

hosted attestation. The team/paid rung: gates verified server-side by a party the committing agents cannot touch — hosted gate runs as a neutral attestation, cross-repo drift dashboards, maintained rulesets. A service, not code: cloning the kits does not clone the neutrality or the ops. Demand-gated — this entry is the public roadmap marker, not a scaffold; hosting and sequencing decisions are on record in the operator's local brief, and its prerequisite mechanism has landed (lifecycle-kit/SPEC.md §Multi-operator semantics). Its single-repo statistics half split off as `iteration-statistics-view`, landed at site-health-statistics-pass (operator direction 2026-10-17, on consult advice, lead-relayed, not a ruling); the cross-repo dashboards stay here. Surfaced 2026-07-07.

**Cost while deferred:** zero — this is a service rather than tree mechanism, so nothing rots; the residue is that gate runs stay self-attested, which binds only when a party the committing agents cannot touch is asked to trust them.

### companion-toolkit-profile

[roadmap: now/ecosystem] [cost: event/high] [surface: lifecycle-kit] [roadmap-summary: Gate a tree whose specs another toolkit's workflow wrote.]

the interop rung's submission half. The build half landed at companion-catalog-extension: `companion/` holds the Spec Kit extension and both recipes, the consumer smoke's companion arm proves them, and docs/spec-toolkits.md is their landing page (companion/SPEC.md). Horizon `now`, operator direction 2026-09-29, lead-relayed (not a ruling): the recent iterations work it toward its submission.

**Deliverable:** the Spec Kit community-catalog submission, filed as the catalog's Extension Submission issue with its `download_url` naming the `checkwright-companion-<version>.zip` Release asset; the extension's README and the landing page then gain the catalog's install form.

**Gated on** a published tag carrying that asset, since the catalog installs from a tagged archive, and on [design-partner-preview](#design-partner-preview)'s observed install. **Also gated, operator direction 2026-09-29, lead-relayed (not a ruling),** on `gate-customer-value-audit`, which landed at gate-sdk-value-pass, and `companion-install-tier`, which landed at companion-tier-delegation-pass: the audit's verdicts decide which gates that tier exposes. **The four preconditions the operator set on 2026-09-27 landed at catalog-submission-preconditions:** `crate-tests-windows-flip`, since red jobs inside a green run read as ignored failures; `linux-glibc-artifacts`; `catalog-landing-docs-polish`; and `spec-toolkits-guarantee`.

**Four more prerequisites, operator direction 2026-09-29, lead-relayed (not a ruling):** three landed at companion-technical-gates (`install-gate-selection`, `companion-spec-to-code-gates`, `speckit-extension-full-profile`), and `adoption-prompt-templates` landed at companion-adoption-landing. Ground: Spec Kit and OpenSpec are technical toolkits, so a companion offering only document gates reads as near-useless, and the launch needs an early-adopter wow.

**Push need (2026-09-27, inside the budget):** the release tag push beside the closing push, since the submission needs the extension on a published tag; close's release policy decides the cut.

**Cost while deferred:** the extension installs only from a Release URL a reader must already hold, so a Spec Kit user browsing the catalog, where adopters find enforcement extensions, does not find it. Surfaced 2026-08-02 at close.

### design-partner-preview

[roadmap: now/adoption] [cost: event/low] [surface: drift-kit] [roadmap-summary: A small observed preview measuring first green, first useful red and retention on real installs.]

a narrow external preview before any broad announcement: a preview cohort whose composition is ruled in the operator's private brief, the first install observed live and the rest arriving through the observation record, instrumented for time-to-first-green, first useful red, false-positive dispositions, and 7/30-day retention per kit. It is the first rung on this queue whose deliverable is **evidence from outside this tree** rather than a tree change.

**The TREE HALF landed in `external-install-evidence`** — drift-kit/SPEC.md §The install-observation record and §The install-evidence projection, the `--emit file-install` capture arm, and `docs/install-evidence.md` behind `check-install-evidence-fresh`. What is kept here is the expensive half: operator hours and a calendar window of thirty days or more, running beside later iterations and never inside a stage session. What re-promotes this entry is an observed install, not another tree change.

**Sequencing is the load-bearing part.** The preview runs *before* [benchmark-ab-experiment](#benchmark-ab-experiment), so pilot findings shape that experiment's task classes and metrics rather than being retrofitted to them; per-gate true/false-positive history and profile retention are preview deliverables, not pre-launch builds. The launch ruling behind it stays in the private brief; this entry is the queue-visible rung.

**Expected FIRST FINDING, not a precondition** (facts restated 2026-10-11 to the current floor, operator direction, lead-relayed, not a ruling): today's quick start is a one-line install under POSIX sh or PowerShell; bash is owed only by a runnable fence or a registered shell gate; native Windows needs Git for Windows for `git` (docs/requirements.md). That is why the merged channel gives the installer no delta — those host-floor facts are an output of the observation, not an input.

**Refused, grounds carried forward:** parking behind `native-windows-bash-floor` (landed 2026-09-18) or the git-only-floor discharge (the trigger is what the preview measures); the icebox (the highest cost-while-deferred in the intake).

**Held deferred twice on 2026-09-26, operator direction lead-relayed (not a ruling),** until v0.26.0 published the repaired front door (`front-door-demo-unreachable`, `demo-catches-a-done-claim`).

**Consult ruling, 2026-09-27, superseding the 2026-09-25 recommendation to promote next:** the cohort was re-ruled in the brief — one observed pre-submission install, then the installers the catalogs and the plugin marketplace send — so this entry runs *through* the two ecosystem units the ledger's `catalog-then-plugin` ruling sequenced first, not ahead of them; that ruling discharged 2026-09-29, the extension and the plugin package both published. The observed install is owed before the catalog submission; the rest re-promotes on the first observed install, as stated above.

**Readiness ahead of the observed install, operator direction 2026-09-29, lead-relayed (not a ruling):** a clean-container first-run rehearsal ran first, as `front-door-container-rehearsal` at preview-readiness, its runs filed to the author seat's record, never the published one. Operator-seat adoption is independent evidence and gates nothing; counting it toward the observed install was declined.

**Comparison arm (operator direction 2026-10-17, lead-relayed, not a ruling):** where the metrics allow, beside a custom-CI baseline and one named same-seat entrant, on mistakes caught, setup effort, interruptions and retention.

**Cost while deferred — still the highest of its intake, and now the more exposed half.** Every claim that would be strongest with external evidence still rests on internal dogfooding, and the channel that would carry it is built and reading zero: the published page states an honest `0` on every pass while the volume of unattested governed surface keeps growing. Deferring also silently defers [benchmark-ab-experiment](#benchmark-ab-experiment), since running that first would fix the wrong metrics. Surfaced 2026-08-02 at close as the review's fourth-ranked item.

### external-gate-quality-evidence

[cost: event/low] [surface: drift-kit]

durable, published evidence of **gate quality as experienced outside this tree**: per-gate true/false-positive history, the disposition of each red a non-author hit, and whether a red changed behaviour or was worked around. The direct answer to the standing threat that a false positive converts the enforcement advantage into bypass and distrust — every blocking gate raises the stakes of a wrong red.

**Why it is not just a report.** The tree already publishes evidence projections, so the mechanism exists; what does not exist is a *population* to measure. A red in this repo is authored and dispositioned by the same party, which cannot distinguish a gate that is right from a gate whose author agrees with it.

**The record's FIELDS landed in `external-install-evidence`**, settling the open design question the deferred form carried: the collection surface is a new capture stream — the observation record's `red` line, with its closed `<verdict>`, `<disposition>` and `<behaviour>` sets — rather than a field on a disposition this tree already records, the disposition being measured being the *adopter's*. What is kept here is the evidence itself, which accrues only once observations exist; `docs/install-evidence.md` publishes zero reds today, and a population is what re-promotes this entry.

**The bundle with [design-partner-preview](#design-partner-preview) was not convenience.** This entry's own carry sentence rules the history unrecoverable — it cannot be retroactively collected, and starts accruing only once someone decides to record it — so a protocol landing without the gate-quality fields designed into it would have sent the first installs' reds somewhere this baseline can never read. There was one chance to fix the record's fields and it is spent.

**Cost while deferred — moderate, asymmetric, and no longer unrecoverable.** The gate-quality claim still rests on fixture pairs and a green battery, which prove a gate does what its author specified and say nothing about whether that was the right thing to specify. The irreversible half is discharged: the record exists, so the history starts accruing at the first observation rather than never. What remains is that the first externally-hit false positive is argued from anecdote until a population exists. Surfaced 2026-08-02 at close, in the same intake pass, as the last of the growth half.

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

### smoke-whole-tree-precondition-unscoped

[cost: event/low] [surface: installer] [recurrence: 2026-10-18]

the consumer smoke gates its whole suite on an unscoped clean-tree preflight (installer/SPEC.md §The consumer smoke), so a dirty path the payload never reads still costs the full `installer_smoke` suite; the packer's own refusal is already footprint-scoped (§The packer).

**Recurred 2026-10-18, at session-sweep-policy-pass' validate.** A completed `--run-validate` with one red suite folds `.workflow/validate-evidence.txt` (evidence-kit/SPEC.md §Evidence manifest: the fold follows the last suite, so a mid-roster abort writes nothing), and the rerun's preflight refused on that dirty manifest: one wasted validate run plus a manual restore. A 2026-10-16 bullet of this shape was discarded as tree-decided; that record is deleted, the honest limit stating the behaviour while rule-it-correct stays refused.

**Ruled `lead, own-authority` 2026-09-10, lead-relayed, not the operator's: leave it filed.** Narrowing the preflight (B) and ruling it correct (C) were both refused at build. B is no one-liner: the smoke's post-run clean assertion is sound only because the preflight established a clean tree, so the pair moves together, an envelope change against the merged `pack-step-dirty-tree-predicate-unscoped` amendment. C stays live: the smoke asserts over the tree it packs, so its precondition may be correct on independent grounds. The recurrence adds a route from the other owner, run-validate keeping its fold out of a path a later suite's preflight reads (evidence-kit/SPEC.md §Evidence manifest), the same envelope question.

**Cost while deferred:** each validate rerun after a red run, and each capture left uncommitted while a validate battery is pending, costs a full `installer_smoke` suite. Filed 2026-09-10 by close, draining the gap inbox; thawed 2026-10-18 by close on the recurrence. Owner lookup: `dirty`, `validate-evidence`, `run-validate` in this file and the disposed-findings record — the 2026-10-16 record, deleted as its ground fell; owner installer/SPEC.md §The consumer smoke.

### tarball-attestation-observed

[cost: event/low] [surface: installer] [observed-by: publish]

the observation half of `tarball-build-attestation`: the first Release cut after that entry landed carries an artifact attestation on its tarball, and a shell installer run against that Release verifies it where the verifier is present.

**Deliverable:** that observation, read off the `publish` run and one install from the published Release, with any defect it shows filed.

**Release (operator directions, lead-relayed, not rulings):** v0.32.0 stays deferred, closing push only, no tag and no publish, by a direction of 2026-10-12. **The bar for the cut, the same direction:** cut once no open finding is a wrong grant or a regression from v0.31.0; a fall-through hole rides as a declared limit in the note. Which open finding the bar blocks is read by the session that next weighs a cut, against its own runs. **At each close since 2026-10-13** the operator rested that reading on the iteration's second-vendor review returning clean, a failed run or a finding leaving the bar a judgment being asked, never skipped or met by a same-vendor reader; on 2026-10-14 the one granted run moved to validate, none at close. **Since 2026-10-14** a close promotes every verdict-moving guard change it would author rather than landing it, the close binding carrying the rule, so a finding the review returns waits for a later build and validate. Three closes read the bar unmet on findings the latest review returned; the last, `grant-redirect-scan-cr`, landed at guard-word-read-release-pass. **That close, by a direction of 2026-10-15,** runs one audit-mode second-vendor review over the iteration's range and cuts v0.32.0 on a clean return, closing push plus tag push; a finding or a failed run defers, a verdict-moving finding promoted. The run failed on the vendor's credit wall, so v0.32.0 stays deferred; foreign-build-budget-pass' close, granted no run, ran none and deferred it again. crate-name-reader-pass' validate spent its one granted run, not clean on one regression the operator ruled a hotfix for; its close deferred v0.32.0 again with no second run granted (operator direction 2026-10-15, lead-relayed), and gate-substrate-sdk-pass' close, given no new answer, deferred it under that direction. That cut's `publish` run produces this observation and [front-door-job-observed](#front-door-job-observed)'s.

**Three grants of 2026-10-16 spent (operator grants, lead-relayed):** gate-dispatch-hermetic-pass' close review returned two findings, carried to the next scope; tamper-reader-repair-pass' validate review returned three P2 findings, landed at its close; budget-feed-observation-pass' validate review returned four, the three that held fixed at its close. Each left v0.32.0 deferred and its tag push unspent; none was verdict-moving.

**Two grants of 2026-10-17 spent:** competitive-followup-pass' validate and close reviews each returned five findings, so v0.32.0 stayed deferred. That close withdrew `prove` from docs/install.md and docs/maintenance.md and the overlap section from docs/openspec.md, its line held in `companion/fixtures/openspec/overlap/line.md`; **the cut restores them** and deletes the held file.

**Grant of 2026-10-18 spent:** session-sweep-policy-pass' close review returned five P2 findings, so v0.32.0 stayed deferred; that close fixed all five, none verdict-moving.

**Cost while deferred:** the attestation step ships unobserved until a release exercises it. Filed 2026-10-04 as a split at scope, because a tag-triggered run cannot be produced by a mid-iteration push (lifecycle-kit/SPEC.md §The state machine). Owner lookup: `attest`, `publish`, `tarball` in this file — tarball-build-attestation, its produce half, and front-door-rehearsal-rule, DISTINCT (a clean-seat rehearsal job, not this one observation).

### plugin-harness-reach

[roadmap: next/ecosystem] [cost: event/low] [surface: plugin] [roadmap-summary: The plugin package verified on more harnesses, with guards wherever a harness has a hook surface.] [not-icebox-eligible: 2026-10-06 filed on the operator direction of 2026-09-28 its body carries; evicting it would compress that direction away]

the harness plugin package reaches Claude Code only in its tested and guarded parts. plugin/SPEC.md reads the portable `plugin.json` and `skills/` as loading on any Agent Plugins 1.0 client off the standard alone, since only Claude Code's install is run; the guards ride `hooks/hooks.json`, outside the standard's portable core, so no other client runs them; the marketplace file is Claude Code's format. No queue entry names another harness. **Inferred, not run:** which harnesses adopt Agent Plugins 1.0. **Operator-supplied, unverified:** Meta's coding harness "seems to be named Muse Code". **Harness priority, operator direction, 2026-09-28:** tier 1 Claude Code and Codex; tier 2 Muse Code, Antigravity and Cursor.

**Deliverable:** (1) the skills load verified on the codex and Antigravity CLIs; (2) a survey of which harnesses read Agent Plugins 1.0 and their catalogs; (3) per-harness guard wiring where a harness has a hook surface. **Read first (2026-10-17, competitive analysis, unprobed here):** Cursor imports Claude Code hook configuration from `.claude/settings*.json` by default, so one wiring may reach two harnesses.

**Cost while deferred:** an adopter on another harness gets unverified skills and no guards. Filed 2026-09-28 by plugin-marketplace-queue-verbs' lead on an operator direction. Re-verified: plugin/SPEC.md states the portable core and that only Claude Code's install is run. Owner lookup: `Agent Plugins`, `codex`, `Cursor`, `Antigravity` in this file — none; owner plugin/SPEC.md.

### verify-workflow-decoupling

[cost: event/low] [surface: installer] [not-icebox-eligible: 2026-10-06 holds the operator's stated aim of 2026-09-28 and the ruling it waits on; evicting it would compress the question away]

the operator states that verification and workflow are fully decoupled, each shippable without the other; the tree shows one direction only. Verification without the workflow holds: `installer/profiles.list`'s starter profile is gate-sdk alone and prose is gate-sdk plus canon-kit, and the companion recipes gate another toolkit's workflow. The workflow without verification is not shipped: every profile carrying lifecycle-kit carries gate-sdk, forced in because without it there is no runner, hook generator or registry; `--enter-stage` is an arm of the gate binary; the delegation profile's comment reads "Vendored is not yet enforced"; docs/kits.md orders the roster as "each kit assumes the machinery of the ones above it". The operator's stated aim (2026-09-28, lead session; an aim, not a /consult objective): a competitive offering that is loosely coupled, composable, configurable and efficient.

**Deliverable:** a ruling on whether the gate binary is verification or substrate both halves share; if the decoupling then holds, the front door and docs/kits.md state it, and a workflow-only profile is weighed.

**Cost while deferred:** the front door neither claims nor refutes a decoupling the operator believes in, and docs/kits.md reads as a dependency chain. Filed 2026-09-28 by plugin-marketplace-queue-verbs' lead. Re-verified: the three profiles' rows and comments in `installer/profiles.list`, and docs/kits.md line 13. Owner lookup: `decoupl`, `profiles.list`, `workflow-only` in this file — none; owner installer/SPEC.md §Profiles, with docs/kits.md.

### releases-page-table

[cost: event/low] [surface: docs] [not-icebox-eligible: 2026-10-07 holds the operator's question of 2026-09-29 and the table shape answering it; evicting it would compress the question away]

docs/releases.md renders its derived note list as a bare list of version links, which repeats the nav: the page carries `nav_children_key: release`, so the nav already lists every note. Operator question, 2026-09-29, lead-relayed: a stats table instead.

**Deliverable:** a derived table whose columns answer an upgrader, above all whether a release needs action on upgrade (its `gates` or `knobs` role section carries entries, roles per installer/SPEC.md §The upgrade contract); also version, date, bump class and per-section counts, which each note's In brief summary table already states. The note composer writes the counts and the action flag as front-matter keys and a gate holds them equal to the note's sections; parsing sections in Liquid at render time is refused as fragile.

**Cost while deferred:** a reader skipping several versions opens each note to learn which need action. Filed 2026-09-29 by companion-technical-gates' lead, at the operator's leave. Re-verified: the page's `<ul>` loop and 29 notes carrying `release:`. Owner lookup: `releases.md`, `front-matter` in this file — none; owner docs/site-architecture.md, with installer/SPEC.md §The upgrade contract for the keys.

### front-door-job-observed

[cost: event/low] [surface: lifecycle-kit] [observed-by: publish]

the observation half of `front-door-rehearsal-rule`'s second layer: the first Release cut after its post-publish job landed, `publish.yml`'s `front-door`, runs that job on the clean runners, and the run is read.

**Deliverable:** that observation, read off the `publish` run, with any defect the clean-seat installs show filed.

**Cost while deferred:** the post-publish job ships unobserved until a release exercises it. Filed 2026-10-10 as a split at scope, because a tag-triggered run cannot be produced by a mid-iteration push (lifecycle-kit/SPEC.md §The state machine). Owner lookup: `post-publish`, `publish`, `observed-by` in this file — [tarball-attestation-observed](#tarball-attestation-observed), DISTINCT (the attestation step's own first run, on the same release).

### notification-delivery-probe

[cost: event/low] [surface: lifecycle-kit] [not-icebox-eligible: 2026-10-10 filed on the operator direction of 2026-10-02 and waits on a lead session with the operator present; evicting it would compress that direction away]

the delivery rule under `lead-notification-wake-race`'s remedy, unprobed: does a completion notification queued during a supervisor's turn that makes no tool call wake its session, and does one tool call before the turn end drain it.

**Deliverable:** that probe, run by a lead with the operator present, since a dispatched session's report is all its caller holds and no session reports on its own turn end (delegation-kit/SPEC.md §The delegation model); then lifecycle-kit/templates/lead.md §The lead model's wait clause and the agent-execution backgrounding bullet's bound confirmed or corrected against the result.

**Inferred, not run:** that a tool call before the turn end delivers the notification — two stalled transcripts show only an undrained queue at the stop, and one 2026-10-02 lead turn with tool calls received its notification inside the turn.

**Cost while deferred:** the lead's wait clause rests on an inferred mechanism; if it is wrong, a stall still costs hours of idle wall-clock until the operator wakes the lead. Filed 2026-10-02 as a split at companion-tier-delegation-pass' scope, operator direction lead-relayed (not a ruling).

### hook-contract-harness-neutral

[cost: event/low] [surface: guard-kit]

kit prose carries the master harness's names where no knob owns them: the hook event names and matchers, the payload keys, the session and config-home variables (owner lifecycle-kit/SPEC.md §bin/session-id.sh), the dispatch tool's parameter names, and two of drift-kit's measurement commands. gate-sdk/SPEC.md §The adopter constraints declares the hook members' names the shipped binding, so nothing is misstated.

**Deliverable:** a harness-neutral statement of the hook contract behind a binding table, the kit SPECs citing it; feature-sized, since it restates wire contracts.

**Cost while deferred:** an adopter on another harness reads the hook-member sections as one vendor's protocol, which they are. Filed 2026-10-06 by doctrine-brevity-journal-arm-pass' build. Re-verified by grep over the kit SPECs for the event names, payload keys, variables and parameter names: 93 lines, delegation-kit 42, guard-kit 31, context-kit 8, lifecycle-kit 7, gate-sdk 3, drift-kit 1, evidence-kit 1. Owner lookup: `hook contract`, `binding table`, `harness-neutral`, `payload key` in this file and the disposed-findings record — none; [plugin-harness-reach](#plugin-harness-reach), DISTINCT (its per-harness guard wiring would bind against this contract); `harness-literal-catcher-gate`, DISTINCT (the catcher for sites already generic).

### grant-pwsh-word-read

[cost: event/high] [surface: guard-kit]

the half of `guard-rules-skeleton-word-read` split off at guard-word-read-release-pass' scope (operator direction, lead-relayed, not a ruling), so no grant widened ahead of the v0.32.0 bar. Three reads stay on the skeleton. The grants read a redirect target there, so a quoted `/dev/null` withholds the grant the bare spelling keeps: rule `ro_pipeline` falls through on `grep -rn foo src 2>"/dev/null"`, and rules `append_scratch`, `bounded_wait`, `bounded_write` and `background_no_record`'s third exemption read it the same way, read off their source and not run. Rule `ro_pipeline` reads its command word there too, and declines a quoted one silently: no variant-lane relation reaches a grant's decline, so it has no limit rows (the 9 once filed under it were block rows rule `allowlist_chain` took over at that iteration's build). And 9 rows in `guard-kit/guard-tests/variant-limits.tsv` are the PowerShell reader's: a backtick inside a command name falls through rules `rm_tracked`, `cd_compound`, `script_interpreter` and `background_no_record`, and a quoted or backticked redirect target or scratch body path through rules `scratch_redirect` and `script_interpreter`.

**Deliverable:** rule `abs_script`'s by-value read (`redirect_values` in `native/src/guard/rules/grants.rs`) given to each grant, rule `ro_pipeline`'s command word read off the word read with authored rows as its oracle, and the PowerShell reader's own word read, each lift deleting its limit rows, where it has any, in its own commit. Every grant lift widens a grant, so it lands in a build a validate's second-vendor review reads.

**Inferred, not run:** that a PowerShell host reads each derived `ps-` row as its base row's command; no host is installed on the machine that derived them — run the derived rows through `pwsh` on the x64 Windows leg

**Cost while deferred:** a PowerShell re-spelling evades two safety blocks, rules `rm_tracked` and `script_interpreter`, and a quoted `/dev/null` costs a prompt the bare spelling does not. Filed 2026-10-14 by guard-word-read-release-pass' scope as the split's remainder; the finding was filed 2026-10-13 by guard-spelling-oracle-pass' spec. Owner lookup: `word read`, `redirect_values`, `ps-`, `pwsh` in this file and the disposed-findings record — `guard-rules-skeleton-word-read`, DISTINCT (the bash block and steer half), `grant-redirect-scan-cr`, done at guard-word-read-release-pass, DISTINCT (the scan this read lands on); owner guard-kit/SPEC.md §The reader and its views.

### evaluator-root-agent-file

[cost: event/low] [surface: gate-sdk]

surface (c) of `evaluator-agent-discovery`, split off at evaluator-discovery-pass' scope (operator direction 2026-10-17, lead-relayed, not a ruling): a root `AGENTS.md` for evaluators and adopters, since a cloning agent reads the maintainer-internal `CLAUDE.md`.

**Why split rather than built:** `check-root-tiering`'s built-in allowlist rejects a stray second agent file at root (context-kit/SPEC.md §The AGENTS.md agent-file adapter smoke asserts it), and an `AGENTS.md` is the always-loaded file of any agent that reads one, so an evaluator-facing file would also load into every such session working here, foreign adapters running stages included. **Inferred, not run:** which foreign adapters this repo dispatches read `AGENTS.md`.

**Deliverable:** design-pending, a feature: the file's audience and home and the rule `check-root-tiering` takes on it, or a refusal recorded.

**Cost while deferred:** a cloning agent's first read is maintainer-internal instructions; whether that costs a recommendation is unmeasured until [agent-selection-probe](#agent-selection-probe) runs. Filed 2026-10-17 by evaluator-discovery-pass' scope as a split remainder. Re-verified there: no root `AGENTS.md`. Owner lookup: the parent's, none; owner gate-sdk/SPEC.md §check-root-tiering.

### agent-selection-probe

[cost: event/low] [surface: drift-kit]

surface (e) of `evaluator-agent-discovery`, split off at evaluator-discovery-pass' scope (operator direction 2026-10-17, lead-relayed, not a ruling): a repeatable cold agent-selection probe asking foreign agent CLIs and a fresh session, context-free, for a tool verifying coding-agent claims before merge, recording whether Checkwright appears.

**Why split rather than built:** each run buys foreign-CLI credit, which has already walled a granted second-vendor run, and an agent's answer moves on its own index and training clock rather than this tree's, so a run in the iteration landing the evaluator surfaces measures their absence.

**Deliverable:** design-pending, a feature: the prompt set, the agents asked, the record's home and form, and the cadence.

**Inferred, not run:** that an evaluating agent reading `CLAUDE.md` forms a worse impression; this probe measures it.

**Cost while deferred:** whether the evaluator surfaces change what an agent recommends stays unmeasured. Filed 2026-10-17 by evaluator-discovery-pass' scope as a split remainder. Owner lookup: the parent's, `llms`, `AGENTS.md`, `evaluator`, `keywords` in this file and the disposed-findings record, none; owner drift-kit/SPEC.md, beside [design-partner-preview](#design-partner-preview)'s observation record.

### shellcheck-gate-tests-derived

[cost: once/low] [surface: gate-sdk]

`check-shellcheck` reaches the bespoke `<tree>/gate-tests/*.test.sh` suites only through a hand-kept list of eleven directories in this repo's `GATE_SDK_LINT_EXTRA_DIRS`. A twelfth kit's suites join no lint until someone remembers to name them. The kit-root predicate derives `lib/`, `bin/`, `checks/`, `templates/` and `smoke/` and never `gate-tests/`, although `--run-gate-tests` spawns each direct `*.test.sh` as shell and the fixture pair ships to every adopter.

**Deliverable:** a feature. Either the derived set gains each kit root's and the gates dir's `gate-tests/` direct `*.sh`, with fixture coverage and the knob list shrunk back to the installer, demo and docs directories, or a refusal is recorded in gate-sdk/SPEC.md §check-shellcheck. Widening a shipped gate's corpus for every adopter is a kit-semantics call, so a close drain does not make it.

**Cost while deferred:** a new kit's suites lint nothing until the knob is edited, and the 129 linted today carried four warnings that went unseen. Filed 2026-10-18 by site-health-statistics-pass' close as the derivation residual of the drained unlinted-gate-tests bullet. Re-verified there: 129 direct suites across eleven directories, ShellCheck 0.11.0 `-S warning` found four warnings, and a probe file proved the widened knob reaches the corpus. Owner lookup: `shellcheck`, `LINT_EXTRA`, `gate-tests` lint in this file and the disposed-findings record found none; the owner is gate-sdk/SPEC.md §check-shellcheck.

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

### install-smoke-sh-matrix

Three unix install-smoke legs in gates.yml are hand-copied jobs where a matrix over the roster's unix legs would hold them in step.

### worktree-memory-dir-key

`check-memory-off` derives the memory dir from the repository toplevel, so a linked-worktree session's memory may land where no scan reads if the harness keys that dir on a canonical working-copy root, a keying read from its bundle and never observed.

### manual-ops-door-subkey

the manual-operation meter keys every arm called through the gate binary's door as one key, split across the door's two spellings, so it cannot name a repeated arm; the deliverable is a door-aware sub-key owned by drift-kit/SPEC.md §The manual-operation meter or guard-kit/SPEC.md §scan-prompts.

### step-title-cite-unresolved

a citation naming a numbered step by its bold title, in RELEASING.md's procedure or lifecycle-kit/templates/lead.md, resolves under no gate, since check-spec-pointer and check-citation-link resolve the heading alone, so a reworded title strands every citing site with the battery green; the deliverable is a title-resolution arm on check-spec-pointer or the cited steps made headings, owned by canon-kit/SPEC.md §check-spec-pointer.

### kit-spec-arm-span-unresolved

A retired gate-binary arm cited in a kit SPEC code span resolves under no gate, since check-front-door-verbs reads the front-door pages alone; widening its arm resolution to the kit SPECs, with the history valves, would catch it.

### toolchain-pin-trails-stable

The Rust pin trails stable by one minor, and the tried move to 1.99.0 reddened the x86-64 glibc floor check (2.34 measured against the declared 2.39), so it waits on a floor-row or floor-rule decision that the audit roster's toolchain-pin class re-surfaces at release prep.

## Done

