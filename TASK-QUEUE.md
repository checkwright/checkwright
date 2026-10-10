# TASK-QUEUE.md — Checkwright work queue

## Iteration: front-door-readiness-pass

  The lifecycle-kit gates read this header's iteration name and the stage cursor — the last stamp in `.workflow/WORKFLOW-STATE.txt` (lifecycle-kit/SPEC.md §The state machine); queue-kit formalizes the queue format itself and gates this file. One iteration per hardening or roadmap unit; [docs/kits.md](docs/kits.md) maps the kits.

---

## New Features

## Technical Debt

### windows-hook-legs-unexecuted

four behaviors windows-shell-floor-pass landed run on no Windows leg. gate-sdk/gate-tests/native-git-hooks.test.sh skips its replaced-launcher forwarding assertion on a host with an executable suffix. The Windows hook witness in `.github/workflows/gates.yml` exercises `pre-commit` alone, leaving `commit-msg`, the absent-binary refusal, the self-name refusal and the linked-worktree case to Ubuntu. The session-context hook member's configured-command spawn has no Windows end-to-end run. The stop-restore witness checks `ErrorActionPreference` after a successful install block only, never a failing block nor the fetch fence's `ProgressPreference`.

**Deliverable:** each run on a Windows leg, or a stated reason one cannot be.

**Read at promotion, 2026-10-10:** the last three hold. The Windows witness steps name `pre-commit.exe` alone, no step of the workflow starts a hook member, and the stop-restore witness reads the error preference after a block that exited 0, while no step asserts the progress preference.

**Push need (2026-10-10, inside the budget):** one mid-iteration push, since a new Windows assertion runs on the remote leg alone; `action-walk-yaml-shapes` and the front-door job's workflow text ride the same push.

**Unit set (2026-10-10, operator direction, lead-relayed, not a ruling):** taken into this iteration.

**Rider (2026-10-10, operator direction, lead-relayed, not a ruling):** a Windows-leg witness for `scripts/ci-front-door.ps1` joins `gates.yml` and rides this unit's push. It must have the file parsed and executed by a PowerShell host on a Windows leg on every push; the mechanism is build's.

**Cost while deferred:** a Windows-only regression in any of the four ships green. Filed 2026-10-07 off windows-shell-floor-pass' close second-vendor review, carried to the next scope's intake; the first held there by reading the test. Owner lookup: `Windows leg`, `ErrorActionPreference`, `stop-restore`, `witness` in this file and the disposed-findings record — [windows-kpi-plugin-start](#windows-kpi-plugin-start), DISTINCT (a plugin Windows cannot start); owner gate-sdk/SPEC.md §git-hook, with context-kit/SPEC.md §The session-context hook.

## Deferred

### guard-quoted-operand-words

[cost: event/low] [surface: guard-kit]

Two rules read a command word without its quoting. Rule `rm_tracked`'s bash arm tests the `sq dq hd` skeleton's words (`native/src/guard/rules/reach.rs`, `rm_tracked_reach`), so any quoted operand reaches the tracked test as a mark: `rm 'README.md'` and `rm "README.md"` pass while `rm README.md` blocks, against guard-kit/SPEC.md's rule entry, which blocks an `rm` naming a tracked path. Rule `find_exec`'s steer re-quotes dequoted words (`native/src/guard/rules/tools.rs`, `shell_word`), and the dequoted view keeps an escaping backslash, indistinguishable there from a literal one inside single quotes: `-exec cat a\ b '{}' \;` is steered to `cat 'a\ b'`, a different file.

**Deliverable:** each word read with its quoting known: the skeleton's held word beside the dequoted one, or a reader-owned word that carries it. The tracked test takes a quoted operand literally (`:(literal)`) and an unquoted one as the glob the shell expands. Both steers print the shell's own word. Each rule gets a decision-table row (a quoted tracked operand, a blank-bearing name, a single-quoted regex backslash), and guard-kit/SPEC.md §The reader and its views states the contract.

**Cost while deferred:** a quoted `rm` of a tracked path deletes unstaged, outside the block's steer; a `find_exec` steer naming an escaped blank cannot be followed as printed. Filed 2026-10-03 by guard-ruleset-gate-neutrality-pass' spec. An exact word needs per-word quoting provenance that no rule consumes yet, and stripping backslashes, as rule `grant_path_slot` does, breaks the common single-quoted regex steer. That makes it a reader contract with cross-rule reach, the escaped-blank precedent's ground. Re-verified by hook probe: both steers as quoted above, and the `rm_tracked` premise widened from blank-bearing names to every quoted operand. Owner lookup: `shell_word`, `quoted operand`, `rm_tracked`, `exact word` in this file — none; owner guard-kit/SPEC.md §The reader and its views.

### shared-tree-stash-steer

[cost: event/low] [surface: guard-kit]

A build session ran `git stash` / `git stash pop` on the shared checkout to rebuild a pre-change binary for a before/after probe. Stash rewrites the shared index and worktree under any concurrent session. Rule `git_mutation_under_producer` blocks `stash` only while a live producer record exists (guard-kit/SPEC.md, its act set), and no rule, agent-execution bullet or stage template steers a before/after probe to a throwaway worktree.

**Deliverable:** a steer from a worktree-rewriting `git stash` on the main checkout to `git worktree add` under the scratch dir, either as a guard-kit block with its decision-table rows or as a delegation-kit/templates/agent-execution.md bullet; choosing between them is the unit's spec question.

**Cost while deferred:** a concurrent session's uncommitted edits or staged paths can vanish or be re-applied under it. Lead-observed once, with the tree intact; the frequency is **inferred, not run**. Filed 2026-10-03 by the lead at guard-ruleset-gate-neutrality-pass' build. Re-verified: `stash` sits only in the live-producer act set, and no template text names stash or a before/after probe. Owner lookup: `stash`, `before/after`, `throwaway` in this file — none; owner guard-kit/SPEC.md §The rule roster.

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

foreign agents. A lead delegating a stage to a foreign coding agent, cashing the public no-lock-in claim: governance enforced at the git/gate boundary, not by trusting the author. *Remaining:* (4) **stage-contract expression**, past its two slices below: a stage whose writes a foreign agent performs. Held at scope 2026-10-09 (operator direction, lead-relayed, not a ruling); not ruled out.

**Slices landed**, each in delegation-kit/SPEC.md §The foreign-vendor run, each accepted by one granted live codex audit: the foreign-CLI executor, `--foreign-run` (delegation-tier-binding, 2026-09-30); then (2) the dispatch transport with (1) the escalation resume model riding it, `--foreign-resume`, that section's Resuming a session (delegation-transport-pass, 2026-10-03). Then (3) the budget oracle, delegation-kit/SPEC.md §The keyed verdict (delegation-wait-journal-pass, 2026-10-07; operator direction, lead-relayed, not a ruling). No foreign run was granted: accepted by stub-driven crate cases and one producer read of a vendor's on-disk session store through a scratch binding, so a live turn refreshing the feed is unobserved, as is whether a turn in a vendor's non-persisting mode, two of this repo's three adapters, leaves a usage event on disk; one granted run of each kind settles both. **This repo's binding (operator direction, 2026-10-07, lead-relayed, not a ruling):** inline in the gitignored `.local` overlay alone, its adapters reading one snapshot under the metric dir from an inline producer. **Horizon, operator direction 2026-09-30, lead-relayed (not a ruling):** `now` — delivery has started, and close moves a horizon on actual work.

**Item (4)'s two slices landed** (both 2026-10-08; operator direction each, lead-relayed, not a ruling). First (foreign-stage-contract-pass), lifecycle-kit/SPEC.md §The stage-contract arm: the audit stage's reading runs foreign, its host performing every write. Second (foreign-stage-binding-pass), its §The host protocol: one protocol file every shipped stage template but the build stage's points at. **Grant spent** (one live foreign run, operator grant 2026-10-08, lead-relayed) by the second slice's own audit stage, whose landing commit carries the run's `OK` line. The agent wrote nothing, kept the frame's five parts, ran the contract's read-only commands unrefused, and left the battery and successor-entry read to the host, its clone holding no gate binary. **Second grant spent** (operator grant 2026-10-09, lead-relayed: judgment-class model, medium effort) by tier-resolution-pass' spec stage, a non-audit contract; its amendment's landing commit carries the `OK` line. It wrote nothing, returned each amendment as a full-text proposal, and named each command needing the binary and each reading lacking ignored content.

**Seam ruling (on record):** generic mechanism only — transport, budget oracle and escalation channel are consumer-config seams; a kit literal naming a vendor crosses the provenance seam, the pattern the retired `prose-profile` ruled. Interacts with [hosted-attestation-service](#hosted-attestation-service) and [plugin-harness-reach](#plugin-harness-reach).

**Demand attested (2026-07-23):** the operator holds three foreign-vendor subscriptions and wants read-heavy delegation routed to them for budget headroom. **Its citer** [companion-toolkit-profile](#companion-toolkit-profile) blocks on none of it.

**Attribution, operator direction 2026-09-30, lead-relayed (not a ruling):** foreign harnesses' commit trailers off and the method stated in one place, the README, held by construction (foreign work lands in the delegating session's commit); a harness setting change takes the operator's confirmation.

**Design memory (2026-07-25, 2026-08-02):** JSONL turn events ship on the installed binaries probed. The machine profile (context-kit/SPEC.md §bin/env-probe, local-only) owns which CLIs and how.

**Cost while deferred:** stage-level work still bills one vendor's budget, and the design memory ages against fast-moving CLIs. Surfaced 2026-07-17 in the release-in-lifecycle lead session (operator question).

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

`check-gate-tamper`'s exemption reader has no implementation-side equivalent. Split 2026-08-09 at scope by operator ruling from an entry that kept its meta-path-roster half; this is the exemption half, unchanged in substance. `extract_exemptions()` parses a shell `# exception-list:` array literal, so a ported gate's Rust module can carry no exemption the gate is able to read.

**Why design-pending:** it wants the ruling [gate-authoring-sdk-surface](#gate-authoring-sdk-surface) holds — whether a meta-gate reads a substrate-neutral descriptor or learns each substrate — and that entry is horizon-set to ecosystem work, so this one waits.

**The coupling was checked at the split rather than inherited.** It is true of this half and was not true of the roster half: which paths a tamper roster covers is configuration, where how a meta-gate reads an exemption across substrates is exactly the substrate-neutrality question the SDK entry holds.

**Cost while deferred:** zero until a ported gate needs an exemption; no first-cohort member carries an exemption list, which gate-sdk/SPEC.md §Meta-gate conservation for the binary substrate records in its `check-gate-tamper` row. Filed 2026-08-02 at close; found by build.

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

### tarball-attestation-observed

[cost: event/low] [surface: installer] [observed-by: publish]

the observation half of `tarball-build-attestation`: the first Release cut after that entry landed carries an artifact attestation on its tarball, and a shell installer run against that Release verifies it where the verifier is present.

**Deliverable:** that observation, read off the `publish` run and one install from the published Release, with any defect it shows filed.

**Cost while deferred:** the attestation step ships unobserved until a release exercises it. Filed 2026-10-04 as a split at scope, because a tag-triggered run cannot be produced by a mid-iteration push (lifecycle-kit/SPEC.md §The state machine). Owner lookup: `attest`, `publish`, `tarball` in this file — tarball-build-attestation, its produce half, and front-door-rehearsal-rule, DISTINCT (a clean-seat rehearsal job, not this one observation).

### custom-gate-substrates

[cost: event/high] [surface: gate-sdk]

an adopter writes a custom gate in shell only. The registry resolves a member as a `.sh` or a `.gate` declaration (gate-sdk/SPEC.md §lib/gate.sh), and a `.gate` descriptor dispatches into the published binary, which an install cannot extend. On native Windows an adopter therefore needs Git for Windows' bash to author a gate, although the installer and the PowerShell front end already run under PowerShell; operator direction, 2026-09-27 (lead session): customers may write gates in shell, but on Windows they should be able to write them in PowerShell. And there is no supported path to a Rust gate, while a fork that adds the subcommand and ships its own build can (docs/requirements.md §Writing your own Rust gates; operator direction, 2026-09-27, lead session). Custom Rust gates are wanted as a capability, with install prerequisites split between shipped native gates, custom shell gates and custom Rust gates (operator direction, 2026-09-27, lead session).

**Deliverable:** the registry resolving further substrates under the output, fail-closed, fixture-pair and self-lint contracts: `.ps1` resolution in the registry and the runner, which the hook's `--git-hook` arm dispatches through, with a PowerShell lint counterpart to `check-shellcheck`; a Rust path, as an adopter-built executable the registry dispatches or an extension crate; and the install page's prerequisites per substrate. Whether one executable-dispatch shape serves both is spec's.

**The hook-wiring rewire build folded in here on 2026-10-07 is [exec-form-hook-registration](#exec-form-hook-registration)**, its own entry by operator direction of that date: it shares this entry's end, the last bash a native-Windows adopter owes, and not its mechanism.

**Cost while deferred:** a native-Windows adopter authoring a gate takes on a bash dependency and a second shell dialect, and one wanting a typed, testable gate must write shell or fork. Filed 2026-09-27 by platform-prerequisite-floors' lead. Re-verified: `registry::resolve` tries `sh` then `gate` per dir and nothing else. Owner lookup: `ps1`, `PowerShell`, `custom gate`, `Rust gate`, `consumer crate` in this file — none; owner gate-sdk/SPEC.md §lib/gate.sh and §The port-candidate criteria.

### plugin-harness-reach

[cost: event/low] [surface: plugin] [not-icebox-eligible: 2026-10-06 filed on the operator direction of 2026-09-28 its body carries; evicting it would compress that direction away]

the harness plugin package reaches Claude Code only in its tested and guarded parts. plugin/SPEC.md reads the portable `plugin.json` and `skills/` as loading on any Agent Plugins 1.0 client off the standard alone, since only Claude Code's install is run; the guards ride `hooks/hooks.json`, outside the standard's portable core, so no other client runs them; the marketplace file is Claude Code's format. No queue entry names another harness. **Inferred, not run:** which harnesses adopt Agent Plugins 1.0. **Operator-supplied, unverified:** Meta's coding harness "seems to be named Muse Code". **Harness priority, operator direction, 2026-09-28:** tier 1 Claude Code and Codex; tier 2 Muse Code, Antigravity and Cursor.

**Deliverable:** (1) the skills load verified on the codex and Antigravity CLIs; (2) a survey of which harnesses read Agent Plugins 1.0 and their catalogs; (3) per-harness guard wiring where a harness has a hook surface.

**Cost while deferred:** an adopter on another harness gets unverified skills and no guards. Filed 2026-09-28 by plugin-marketplace-queue-verbs' lead on an operator direction. Re-verified: plugin/SPEC.md states the portable core and that only Claude Code's install is run. Owner lookup: `Agent Plugins`, `codex`, `Cursor`, `Antigravity` in this file — none; owner plugin/SPEC.md.

### verify-workflow-decoupling

[cost: event/low] [surface: installer] [not-icebox-eligible: 2026-10-06 holds the operator's stated aim of 2026-09-28 and the ruling it waits on; evicting it would compress the question away]

the operator states that verification and workflow are fully decoupled, each shippable without the other; the tree shows one direction only. Verification without the workflow holds: `installer/profiles.list`'s starter profile is gate-sdk alone and prose is gate-sdk plus canon-kit, and the companion recipes gate another toolkit's workflow. The workflow without verification is not shipped: every profile carrying lifecycle-kit carries gate-sdk, forced in because without it there is no runner, hook generator or registry; `--enter-stage` is an arm of the gate binary; the delegation profile's comment reads "Vendored is not yet enforced"; docs/kits.md orders the roster as "each kit assumes the machinery of the ones above it". The operator's stated aim (2026-09-28, lead session; an aim, not a /consult objective): a competitive offering that is loosely coupled, composable, configurable and efficient.

**Deliverable:** a ruling on whether the gate binary is verification or substrate both halves share; if the decoupling then holds, the front door and docs/kits.md state it, and a workflow-only profile is weighed.

**Cost while deferred:** the front door neither claims nor refutes a decoupling the operator believes in, and docs/kits.md reads as a dependency chain. Filed 2026-09-28 by plugin-marketplace-queue-verbs' lead. Re-verified: the three profiles' rows and comments in `installer/profiles.list`, and docs/kits.md line 13. Owner lookup: `decoupl`, `profiles.list`, `workflow-only` in this file — none; owner installer/SPEC.md §Profiles, with docs/kits.md.

### plugin-front-matter-yaml

[cost: event/low] [surface: plugin]

`check-plugin-parity`'s front-matter reader (`front_matter` in `native/src/gates/plugin_parity.rs`) splits each line at its first `:` and strips one quote pair, so it admits a `SKILL.md` description that strict YAML rejects: an unquoted `: ` in agent-execution's description passed the local battery and the harness CLI and redded only CI's `skills-ref` step on the v0.28.0 stamp push. plugin/SPEC.md §The validation leg rules manifest validity the leg's, and nothing runs that leg's oracles before a push.

**Deliverable:** the leg's oracles runnable locally before a package change commits, or the reader refusing what a YAML plain scalar cannot carry, with a `bad/` fixture holding the `: ` case.

**Cost while deferred:** a front-matter slip is caught only by a push, which spends a hotfix from the budget. Filed 2026-09-28 by plugin-marketplace-queue-verbs' close. Re-verified: the reader above, and `.github/workflows/gates.yml` installs `skills-ref` in CI only. Owner lookup: `front matter`, `skills-ref`, `yaml` in this file — only the icebox's template-copy-parity-yaml-widening, DISTINCT (template copies); owner plugin/SPEC.md §check-plugin-parity.

### openspec-delta-base-agreement

[cost: event/low] [surface: companion] [not-icebox-eligible: 2026-10-07 filed on the operator direction of 2026-09-29 and carries the measurement its gate would rest on; evicting it would compress both away]

an OpenSpec change delta that disagrees with its base spec passes the battery and OpenSpec's own validator, and is caught, if at all, only at archive. Measured on openspec 1.13.2 at companion-technical-gates' spec: `validate --strict` exits 0 on a MODIFIED or RENAMED delta naming an absent requirement and on an ADDED one naming an existing requirement, printing only an INFO line; `archive -y` refuses those three, but `--skip-specs` bypasses the refusal; a REMOVED delta naming an absent requirement passes validate silently and archive takes it as already removed.

**Deliverable:** a generic commit-time gate for heading-set delta agreement, its OpenSpec binding in the recipe, if it clears the value bar `toolkit-overlap-value-bar` landed (companion/SPEC.md §The tiers); it is that bar's first candidate, since OpenSpec owns the check at archive and this gate would re-check it earlier.

**Cost while deferred:** an OpenSpec adopter's technical gates read nothing on a conventional task list, which names no paths, so the companion's code-facing reach there is `check-task-path-claim` alone. Filed 2026-09-29 at companion-technical-gates' spec on an operator direction lead-relayed (not a ruling). Owner lookup: `delta`, `archive`, `openspec` in this file — `companion-spec-to-code-gates`, DISTINCT (it ships the task gates and defers this one), and `toolkit-nav-hierarchy`, DISTINCT (nav labels); owner companion/SPEC.md.

### releases-page-table

[cost: event/low] [surface: docs] [not-icebox-eligible: 2026-10-07 holds the operator's question of 2026-09-29 and the table shape answering it; evicting it would compress the question away]

docs/releases.md renders its derived note list as a bare list of version links, which repeats the nav: the page carries `nav_children_key: release`, so the nav already lists every note. Operator question, 2026-09-29, lead-relayed: a stats table instead.

**Deliverable:** a derived table whose columns answer an upgrader, above all whether a release needs action on upgrade (its `gates` or `knobs` role section carries entries, roles per installer/SPEC.md §The upgrade contract); also version, date, bump class and per-section counts, which each note's In brief summary table already states. The note composer writes the counts and the action flag as front-matter keys and a gate holds them equal to the note's sections; parsing sections in Liquid at render time is refused as fragile.

**Cost while deferred:** a reader skipping several versions opens each note to learn which need action. Filed 2026-09-29 by companion-technical-gates' lead, at the operator's leave. Re-verified: the page's `<ul>` loop and 29 notes carrying `release:`. Owner lookup: `releases.md`, `front-matter` in this file — none; owner docs/site-architecture.md, with installer/SPEC.md §The upgrade contract for the keys.

### docs-chrome-page-repeat

[cost: event/low] [surface: docs]

`check-docs-page-repeat` reads page sources and never `docs/_layouts` or `docs/_includes`, so a chrome addition repeating a page's statement passes. Found at `homepage-license-duplicate`, where the footer's license line duplicated docs/index.md's License section; that unit's `check-license-line` widening holds the license instance only.

**Deliverable:** an arm prefixing the layout's and includes' literal text nodes (Liquid excluded) to every page's corpus, so a sentence of eight words or more or a link target stated in both reds; or a boundary note refusing it.

**Cost while deferred:** the next chrome addition can duplicate a page statement unseen until a reader finds it. Filed 2026-09-29 by companion-adoption-landing's build. Re-verified: canon-kit/SPEC.md §check-docs-page-repeat reads each `CANON_KIT_PAGE_REPEAT_PAGES` page alone and deliberately asserts no repeat across pages, so the arm must weigh that boundary. Owner lookup: `page-repeat`, `_layouts`, `chrome` in this file — [site-video-poster-rule](#site-video-poster-rule), DISTINCT (embeds); owner canon-kit/SPEC.md §check-docs-page-repeat, with docs/page-authoring.md §Page-authoring rules. Surface also canon-kit.

### front-door-job-observed

[cost: event/low] [surface: lifecycle-kit] [observed-by: publish]

the observation half of `front-door-rehearsal-rule`'s second layer: the first Release cut after its post-publish job lands runs that job on the clean runners, and the run is read.

**Deliverable:** that observation, read off the `publish` run, with any defect the clean-seat installs show filed.

**Cost while deferred:** the post-publish job ships unobserved until a release exercises it. Filed 2026-10-10 as a split at scope, because a tag-triggered run cannot be produced by a mid-iteration push (lifecycle-kit/SPEC.md §The state machine). Owner lookup: `post-publish`, `publish`, `observed-by` in this file — [tarball-attestation-observed](#tarball-attestation-observed), DISTINCT (the attestation step's own first run, on the same release).

### notification-delivery-probe

[cost: event/low] [surface: lifecycle-kit]

the delivery rule under `lead-notification-wake-race`'s remedy, unprobed: does a completion notification queued during a supervisor's turn that makes no tool call wake its session, and does one tool call before the turn end drain it.

**Deliverable:** that probe, run by a lead with the operator present, since a dispatched session's report is all its caller holds and no session reports on its own turn end (delegation-kit/SPEC.md §The delegation model); then lifecycle-kit/templates/lead.md §The lead model's wait clause and the agent-execution backgrounding bullet's bound confirmed or corrected against the result.

**Inferred, not run:** that a tool call before the turn end delivers the notification — two stalled transcripts show only an undrained queue at the stop, and one 2026-10-02 lead turn with tool calls received its notification inside the turn.

**Cost while deferred:** the lead's wait clause rests on an inferred mechanism; if it is wrong, a stall still costs hours of idle wall-clock until the operator wakes the lead. Filed 2026-10-02 as a split at companion-tier-delegation-pass' scope, operator direction lead-relayed (not a ruling).

### manual-ops-door-subkey

[cost: event/low] [surface: drift-kit]

`--emit manual-ops` keys a shell call by guard-kit's ranking key, which sub-keys only its multi-command set, so every arm called through the gate binary folds into one key, and two spellings of the binary split it: `./native/target/release/checkwright-gates` 61 calls in 5 sessions and `native/target/release/checkwright-gates` 24 in 1 at drift-kit-tail-crosser-pass, rewrite, scratch-run and emit arms alike. The meter cannot say which arm is repeated. That close ignored both spellings as the door, as `bash gate-sdk/bin/run-gates.sh` already was; the per-arm signal is this entry's.

**Deliverable:** a door-aware sub-key, the door plus its arm word, one key across the door's spellings, in drift-kit/SPEC.md §The manual-operation meter or guard-kit/SPEC.md §scan-prompts; which owner is spec's.

**Cost while deferred:** a repeated arm, the strongest tooling candidate the meter could name, stays invisible. Filed 2026-10-05 by that iteration's build. Re-verified: the meter at close ranks the first spelling first and the second seventh; the split across spellings is new at the drain. Owner lookup: `ranking key`, `sub-key`, `manual-op` in this file — none; owner drift-kit/SPEC.md §The manual-operation meter.

### trajectory-limits-unstated

[cost: event/low] [surface: drift-kit]

drift-kit/SPEC.md §The published-evidence extractor says the extractor "states plainly that no controlled ungoverned baseline exists", and that the knowledge-friction exclusion is "stated as a limitation on the framing page". `native/src/emit/trajectory.rs` prints neither: its `--human` header is two lines naming neither limit, and no framing page exists, since `docs/evidence-data.md` is the bare table.

**Deliverable:** the two limits stated where a reader of the published evidence meets them, either in the arm's output or on a framing page around the committed projection, or the SPEC's two claims narrowed to what the arm prints; which one is spec's.

**Cost while deferred:** a reader trusting the SPEC believes the published evidence carries its own caveats, and it carries none. Filed 2026-10-05 by context-kit-tail-publisher-pass' build. Re-verified: the `--human` branch of `emit` pushes the two header lines alone, and no `docs/` page outside the SPEC mirrors names an ungoverned baseline. Owner lookup: `ungoverned`, `framing page`, `trajectory` in this file — benchmark-ab-experiment, DISTINCT (the controlled experiment itself); owner drift-kit/SPEC.md §The published-evidence extractor.

### site-health-probe-unexecuted

[cost: event/low] [surface: site-kit]

site-kit/templates/site-health.yml's probe step is linted by the `check-action-*` gates and copied by the kit's smoke, and no tracked oracle executes its bash before a consumer's scheduled run.

**Deliverable:** a tracked site-kit test that extracts the probe step's run block and runs it under PATH stubs for `curl`, `gh` and `sleep`, over a healthy set, one and two transient failures, a persistent failure and `force_fail`.

**Cost while deferred:** a regression in the resample loop or an arm's zero case ships to every copier unseen. Filed 2026-10-06 by site-fence-release-cites-pass' build. Re-verified: no tracked `.sh` names `site-health` but site-kit/smoke/install.sh, which copies the template and runs none of it. Owner lookup: `site-health`, `resampl`, `PATH stub` in this file and the disposed-findings record — [site-health-issue-venue-unwanted](#site-health-issue-venue-unwanted), DISTINCT (the issue step's venue); owner site-kit/SPEC.md §templates/site-health.yml.

**Inferred, not run:** that the loop passed those five cases under a scratch harness at its build, which no tracked file carries.

### step-title-cite-unresolved

[cost: event/low] [surface: canon-kit]

a citation naming a numbered step by its bold title (RELEASING.md §The procedure, *Author the release-note post — in-iteration*, and the same shape citing lifecycle-kit/templates/lead.md) resolves under no gate: `check-spec-pointer` and `check-citation-link` resolve the heading alone, so a reworded step title leaves every citing site stale with the battery green.

**Deliverable:** a holder for the title, either a title-resolution arm on `check-spec-pointer` or the cited steps made headings; which one is spec's, and so is whether a prefix resolves: .github/workflows/publish.yml cites *Watch the publish workflow* for a title that runs on to *both channels*, and .claude/agents/stage-session.md cites *Tier each batch* for a longer one (read 2026-10-06 at this entry's filing close, off its second-vendor review).

**Cost while deferred:** one reworded title in RELEASING.md mis-names its citations, a gate's printed remedy among them. Filed 2026-10-06 by site-fence-release-cites-pass' build. Re-verified by grep: eight lines cite a step of the two files by title. Owner lookup: `bold title`, `step title`, `title-resolution` in this file and the disposed-findings record — none; release-step-number-cites, landed this iteration, DISTINCT (it moved the citations off the step's number, and this is the title's own drift, so no recurrence).

**Inferred, not run:** that neither gate reads past the heading; no fixture rewording a cited title was run.

### site-health-resample-knobs

[cost: event/low] [surface: site-kit]

site-kit/templates/site-health.yml's resample loop sets its attempt count and its pause as literals in the probe step's run body, outside the step's env block that holds the template's other knobs, and site-kit/SPEC.md §templates/site-health.yml says the template owns both as literals. doctrine-kit/DOCTRINE.md Policy-as-choice asks a calibration to ship as a consumer-selectable set, off among it.

**Deliverable:** both lifted into the probe step's env block, one attempt meaning off, the SPEC section stating the set; a feature, since each is a new name on the template's config surface.

**Cost while deferred:** a copier wanting another bound, or no resample, edits the probe body. Filed 2026-10-06 by site-fence-release-cites-pass' close audit. Re-verified: the run body assigns `attempts=3` beside a pause, and the SPEC's resample paragraph states the ownership. Owner lookup: `site-health`, `resampl`, `attempts`, `pause` in this file and the disposed-findings record — [site-health-probe-unexecuted](#site-health-probe-unexecuted), DISTINCT (the loop's missing oracle, which this reshapes: its cases would read the knobs); [site-health-issue-venue-unwanted](#site-health-issue-venue-unwanted), DISTINCT (the issue step's venue).

### hook-contract-harness-neutral

[cost: event/low] [surface: guard-kit]

kit prose carries the master harness's names where no knob owns them: the hook event names and matchers, the payload keys, the session and config-home variables (owner lifecycle-kit/SPEC.md §bin/session-id.sh), the dispatch tool's parameter names, and two of drift-kit's measurement commands. gate-sdk/SPEC.md §The adopter constraints declares the hook members' names the shipped binding, so nothing is misstated.

**Deliverable:** a harness-neutral statement of the hook contract behind a binding table, the kit SPECs citing it; feature-sized, since it restates wire contracts.

**Cost while deferred:** an adopter on another harness reads the hook-member sections as one vendor's protocol, which they are. Filed 2026-10-06 by doctrine-brevity-journal-arm-pass' build. Re-verified by grep over the kit SPECs for the event names, payload keys, variables and parameter names: 93 lines, delegation-kit 42, guard-kit 31, context-kit 8, lifecycle-kit 7, gate-sdk 3, drift-kit 1, evidence-kit 1. Owner lookup: `hook contract`, `binding table`, `harness-neutral`, `payload key` in this file and the disposed-findings record — none; [plugin-harness-reach](#plugin-harness-reach), DISTINCT (its per-harness guard wiring would bind against this contract); `harness-literal-catcher-gate`, DISTINCT (the catcher for sites already generic).

### windows-kpi-plugin-start

[cost: event/low] [surface: drift-kit]

a consumer KPI plugin cannot start on native Windows. The drift report starts a resolved tier-1 or tier-2 `kpi-<name>.sh` path as a program, with no interpreter word, and Windows starts no `.sh`. drift-kit/SPEC.md §The extensibility contract's promise is unreachable there and the row degrades to its plugin-failed read.

**Deliverable:** a plugin dispatch a native-Windows consumer can meet. Its shape is spec's, and a ruling on [custom-gate-substrates](#custom-gate-substrates)' dispatch shape bears on it.

**Inferred, not run:** the row degrades on native Windows — a drift report over a tree registering one plugin, on a Windows leg

**Cost while deferred:** a native-Windows consumer's own KPI never reports. Filed 2026-10-06 by windows-shell-floor-pass' spec, operator direction lead-relayed (not a ruling). Re-verified by reading the report's plugin arm and the process wrapper: the path is spawned as given. Owner lookup: `DRIFT_KIT_KPIS_FILE`, `kpi-`, `plugin failed`, `extensibility` in this file and the disposed-findings record — none; [custom-gate-substrates](#custom-gate-substrates), DISTINCT (gate authoring off bash).

### post-spec-cite-no-valve

[cost: event/low] [surface: canon-kit]

a dated post citing a kit SPEC section by heading reds `check-spec-pointer` when a later unit deletes that section. The other canon gates hold a post immutable and this one carries no valve for it, so an amendment rostering such a post as left standing cannot be followed. Met at native-executable-git-hooks, whose landing repointed the v0.30.0 post's one citation.

**Deliverable:** one rule for a post's section citation, either a valve on the gate's prose-citation pass or a stated duty to repoint; which is spec's, against canon-kit/SPEC.md §check-spec-pointer.

**Cost while deferred:** each section deletion a post cites edits a surface the tree calls immutable. Filed 2026-10-06 by windows-shell-floor-pass' build. Re-verified: the landing commit's diff carries a one-line edit to that post. Owner lookup: `docs/posts`, `check-spec-pointer`, `immutable` in this file and the disposed-findings record — [step-title-cite-unresolved](#step-title-cite-unresolved), DISTINCT (a step title no gate resolves).

### exec-form-hook-registration

[cost: event/low] [surface: gate-sdk]

the hook-wiring rewire windows-kit-bash-files did not land: context-kit and guard-kit still owe bash on native Windows through their settings templates, whose hook commands lead with a `bash` word. Split 2026-10-07 at scope from [custom-gate-substrates](#custom-gate-substrates), where build had folded it (operator direction, 2026-10-07, lead-relayed, not a ruling: its own entry).

**Deliverable:** every hook registration a kit ships takes the exec form: `command` naming the gate binary under the project-directory placeholder, with no executable suffix, and `args` carrying `--hook` and the member. The registration parser reads the binary as a command token, and `check-settings-paths` resolves a suffix-less candidate. The plugin's wiring keeps the shell form, since it must exit 0 in a repository that never installed the kits. The refused-repository decline moves into the `--hook` arm. gate-sdk/SPEC.md §The adopter constraints' first script-door class narrows to a value the harness can only run through a shell. The bash row then names a runnable fence and a registered shell gate alone.

- **Waits on** a native Windows host, which no workflow leg is. Operator-run there 2026-10-07 (Windows 11, harness 2.1.292, two sessions, both transcripts read by the lead): the exec form started the binary with its `args` and the placeholder substituted, and trailing shell words in `args` made no file; a block refused the call and an advisory reached the session; with the binary renamed away the call proceeded on a failed direct spawn of the suffix-less path, which shows no shell stands between; and the suffix-less path started the file carrying the suffix. Unrun: no bash on `PATH`, held on the operator's report with no transcript line showing the `PATH`, where the harness still found Git's bash for its own shell tool; a host with no Git for Windows bash at all; the linked-worktree placeholder on Windows; same-session arming.
- **Run on Linux, harness 2.1.292.** The same four, and: with the binary renamed away no notice reached the model channel; in a linked worktree the placeholder named the main checkout; a `hooks` edit armed in the running session; a call a tool refuses on its own input never reaches the hook.
- **Hazard.** The binary given no argument exits 2, so a harness that dropped `args` would block every call the matcher takes. A witness rides a matcher no session uses and a scratch copy of the binary.

**Cost while deferred:** a native-Windows adopter whose profile carries context-kit or guard-kit needs Git for Windows' bash for the hook wiring. Filed 2026-10-07 by windows-shell-floor-pass' build as a fold, its open question carried by that close to this scope's intake. Owner lookup at that close: hook wiring, settings templates, the exec form and PowerShell in the icebox — no other owner; [hook-contract-harness-neutral](#hook-contract-harness-neutral), DISTINCT (the contract's prose, not the registration's form); owner gate-sdk/SPEC.md §The adopter constraints.

### session-sweep-horizon-baked

[cost: event/low] [surface: context-kit]

the session-context hook member bakes its scratch sweep's age horizon: it removes scratch older than one day on a constant (`STALE_SECS` in `native/src/hook/session_context.rs`), with no context-kit knob and no off value, where doctrine-kit/DOCTRINE.md Policy-as-choice has a kit ship a calibration as a selectable set with off among it. The port carried the shell template's own literal.

**Deliverable:** the horizon as a context-kit knob with an off value, its row in context-kit/SPEC.md §Layout and configuration and §The session-context hook stating the set; a feature, since it adds a knob.

**Cost while deferred:** a consumer wanting another horizon, or no sweep, has no setting. Filed 2026-10-07 off windows-shell-floor-pass' close audit, carried to the next scope's intake. Re-verified: the constant, and no `CONTEXT_KIT_` knob in the SPEC names a sweep. Owner lookup: `age horizon`, `scratch sweep`, `STALE_SECS` in this file and the disposed-findings record — none; [site-health-resample-knobs](#site-health-resample-knobs), DISTINCT (the same class on site-kit's template).

### kit-spec-arm-span-unresolved

[cost: event/low] [surface: installer]

a retired gate-binary arm named in a kit SPEC's code span resolves under no gate. lifecycle-kit/SPEC.md cited the hook-emit arm as a live precedent for a whole iteration after `native-executable-git-hooks` deleted it, its amendment's own delta having named the repoint, and the battery stayed green: `check-front-door-verbs` resolves an arm span against the binary's arm table on the front-door pages alone. Found twice at windows-shell-floor-pass' close, by its second-vendor review and its audit sweep, and fixed there.

**Deliverable:** the same arm resolution over the kit SPECs' code spans, with the history valves; a feature, since it widens a gate's corpus.

**Cost while deferred:** a kit SPEC can cite an arm the binary no longer carries, unread until a reader runs it. Filed 2026-10-07 off windows-shell-floor-pass' close, carried to the next scope's intake. Owner lookup: `arm span`, `front-door-verbs`, `retired arm` in this file and the disposed-findings record — none; owner installer/SPEC.md §check-front-door-verbs.

### wait-exemption-body-unbounded

[cost: event/low] [surface: guard-kit]

guard-kit rule `background_no_record`'s bash exemption (2) exempts any backgrounded call carrying a `do … done` span, whatever its body runs, so a backgrounded loop that does work is a producer launched with no record: `until [ -f .tmp/marker ]; do touch .tmp/marker; sleep 5; done` falls through, pinned as a row of guard-kit/guard-tests/background-cases.tsv. The PowerShell exemption holds the body to one sleep statement, and rule `wait_no_producer` declines on the same body.

**Deliverable:** the bash exemption held to a body of `sleep` statements, each decision table's loop-carrying rows re-derived, and guard-kit/SPEC.md §The generic ruleset stating it; a behavior change, since it widens a block.

**Cost while deferred:** rule `git_mutation_under_producer` has no record to read for such a loop, so a commit under it passes. Filed 2026-10-07 by delegation-wait-journal-pass' build.

### inline-python-steer-split

[cost: event/low] [surface: guard-kit]

rule `sed_file`'s inline-python arm took one of two like calls in delegation-wait-journal-pass' close session. A `python3 -` heredoc rewriting four tracked files through `open().write`, followed by a grep, ran; the next, rewriting one file and followed by a build call, was blocked with the steer to `--rewrite`.

**Deliverable:** the two shapes as decision-table rows, and the rule or its stated reach in guard-kit/SPEC.md corrected to agree.

**Inferred, not run:** that a computed-text construct in the first body told them apart, the discriminator the rule entry states — neither body was replayed through the hook

**Cost while deferred:** the steer toward the count-asserting rewrite arm is skipped by a shape nobody has named. Filed 2026-10-08 by delegation-wait-journal-pass' close, after its drain, and promoted at the next scope's intake. Re-verified: the rule entry blocks a literal rewrite carrying no computed-text construct and says it leans toward passing. Owner lookup: `python`, `--rewrite`, `heredoc`, `sed_file` in this file and the disposed-findings record — none; owner guard-kit/SPEC.md §The generic ruleset.

### agent-effort-line-unmeasured

[cost: event/low] [surface: delegation-kit]

that an agent definition's effort line sets its child's effort (delegation-kit/SPEC.md §check-agent-tier-explicit, Honest limits) is unmeasured. tier-resolution-pass' close session ran under a definition stating one effort and read that effort from `--model-verdict`, but its dispatch named none and its dispatcher read the same value, so inheritance explains the reading as well as the definition does. Every tracked definition binds the same effort, so no session of this tree's present binding can tell the two apart.

**Deliverable:** one dispatch of a definition whose effort differs from its dispatcher's running effort, `--model-verdict` read in the child, and the Honest-limits sentence rewritten to the result.

**Cost while deferred:** a class's bound effort is trusted on the harness's documentation, so a harness that ignored the line would run every class at the dispatcher's effort with the generated line and its freshness gate both green. Filed 2026-10-09 by tier-resolution-pass' close after its drain, and promoted at the next scope's intake. Re-verified there by grep: the five tracked definitions state one effort. Owner lookup: `effort`, `model-verdict`, `Honest limits` in this file and the disposed-findings record — none; owner delegation-kit/SPEC.md §check-agent-tier-explicit.

### git-name-read-line-form

[cost: event/low] [surface: native]

git's name-listing reads outside the tracked-set listing still take the line form, which quotes a name carrying a special byte: the always-loaded growth diff (`--name-only`), the packer's and the stage entry's `status --porcelain` reads, the upgrade smoke's staged read, the packer's `ls-tree --name-only` reads and the source stamp's two shell holders among them. `listing-line-form-readers` closed the tracked-set listing alone, and its holder sees none of these.

**Deliverable:** a census of the crate's name-bearing git reads by form, each one that reads a name moved to the NUL form through the shared reader or stated as an emptiness test that reads none, and the holder widened to the argvs it then covers.

**Inferred, not run:** that each site misreads or misses a name git quotes — stage a member whose name carries a double quote and run each reader over it

**Cost while deferred:** a staged or dirty member under a name git quotes is misread or missed by each such reader. Filed 2026-10-09 by crate-reader-hardening-pass' spec and promoted at its close. Re-verified by `git grep -n -E 'name-only|name-status|porcelain|ls-tree' -- native/src`: the named sites carry no `-z`, and most `porcelain` hits under the installer smoke are emptiness tests. Owner lookup: `name-only`, `name-status`, `porcelain` in this file and the disposed-findings record — none; `listing-line-form-readers`, done, DISTINCT (the tracked-set listing); owner gate-sdk/SPEC.md §Fail-closed contract.

### nul-listing-lossy-decode

[cost: event/low] [surface: native]

ten listing readers already on the NUL form decode a name lossily through an argv of their own: `check-md-refs`, `check-spec-pointer`, `check-fence-command-head`, `check-docs-cmd`, `check-md-unwrapped`, the hook's staged read, the consumer-smoke and projection-witness listings, `--emit close-surfaces` and uninstall. A member whose name is not UTF-8 is matched or opened there under a replacement-character spelling, while the shared reader `tracked-name-lossy-decode` landed refuses it.

**Deliverable:** each of the ten on the shared reader's refusing path, or its lossy read stated with the ground that keeps it, and one holder for the set.

**Inferred, not run:** that each of the ten opens or matches a non-UTF-8 name under a replacement spelling — commit a member whose name carries a lone 0xff byte and run each reader over the tree

**Cost while deferred:** the tree gives two answers on one name, a refusal from the shared reader and a silent misspelling from these. Filed 2026-10-09 by crate-reader-hardening-pass' spec and promoted at its close. Re-verified by `git grep -c from_utf8_lossy -- native/src`: each named module carries the call. Owner lookup: `lossy`, `from_utf8_lossy`, `non-UTF-8` in this file and the disposed-findings record — [worktree-memory-dir-key](#worktree-memory-dir-key), DISTINCT (a memory directory's key); `tracked-name-lossy-decode`, done, DISTINCT (the shared reader alone); owner gate-sdk/SPEC.md §Fail-closed contract.

### tier-model-shape-unvalidated

[cost: event/low] [surface: delegation-kit]

the tier tables' validator holds a model to no shape beyond carrying no whitespace, and `--emit agent-tiers --write` puts it bare after a definition's `model:` key. `tier-value-validator-gaps` closed the effort's hole and did not scope the model's shape.

**Deliverable:** a model shape the validator refuses outside of, admitting a vendor's bracketed and dotted ids, with its unit rows and the sentence in delegation-kit/SPEC.md §check-agent-tier-explicit.

**Inferred, not run:** that a model opening on a hash is written as a YAML comment, leaving the definition's model empty while assertion B compares bytes and passes — bind such a model on a scratch definition directory and run `--emit agent-tiers --write`, then the gate

**Cost while deferred:** a binding typo of that shape ships a definition with no model under a green gate. Filed 2026-10-09 by crate-reader-hardening-pass' first build batch and promoted at its close. Re-verified by reading `native/src/knobs/delegation_kit.rs`: its refusal rows hold the pair's form and whitespace alone. Owner lookup: `agent-tiers`, `tier table`, `YAML comment` in this file and the disposed-findings record — none; owner delegation-kit/SPEC.md §check-agent-tier-explicit.

### unremapped-rebuild-reds-smoke

[cost: event/low] [surface: installer]

the crate suite's `cargo test --release` rebuilds the release binary without `bin/build-native.sh`'s path-remap flags, so the builder's cargo registry path is left in it. The next validate run's installer smoke stages that binary and reds on `check-tree-terms` until `bin/build-native.sh` is rerun. The smoke's staging step refuses a binary whose source stamp is stale (`native/src/emit/installer_smoke/staging.rs`) and reads nothing of the remap.

**Deliverable:** one of three, spec's to choose: the crate suite built with the remap, the staging step refusing an unremapped binary with the rebuild steer it already prints for a stale one, or the smoke building through `bin/build-native.sh`.

**Inferred, not run:** that a plain `cargo test --release` is what strips the remap — run it under `native/`, then the installer smoke, and read `check-tree-terms`

**Cost while deferred:** a wasted validate round, about 17 minutes, on any seat whose binary was last built by a plain cargo build. Filed 2026-10-09 by crate-reader-hardening-pass' validate, which saw the red and the clean rerun, and promoted at its close. Re-verified by reading the staging step. Owner lookup: `remap`, `installer_smoke`, `check-tree-terms`, `cargo test` in this file and the disposed-findings record — none; [validate-suite-wall-clock-unowned](#validate-suite-wall-clock-unowned) and [build-native-obligation-unconditional](#build-native-obligation-unconditional), both DISTINCT (the suites' serial cost, a rebuild owed by a crate-free commit); owner installer/SPEC.md §The consumer smoke.

### front-matter-dotted-closer

[cost: event/low] [surface: canon-kit]

the shared front-matter reader (`front_matter_span` in `native/src/spec.rs`) takes a line of three hyphens as a block's only closer. Jekyll also closes a block on a line of three dots, so a page closed that way is read as carrying no block and its metadata is scanned as body by the six gates on the reader, where canon-kit/SPEC.md §The shared spec adapters calls the reading Jekyll's.

**Deliverable:** the closer set stated in that section and read by the reader, with a unit row for the dotted form.

**Inferred, not run:** that a gate on the reader gains a finding or a long measure on such a page — close a scratch page's block on three dots and run the six gates over it

**Cost while deferred:** a page closing its front matter on dots can gain a false finding or be measured long. Filed 2026-10-09 off crate-reader-hardening-pass' close second-vendor review, which ran Jekyll 4.4.1 on such a page and got the body, and carried to the next scope's intake. Re-verified there: the reader's closing rule compares a trimmed line with three hyphens alone. Owner lookup: `three dots`, `closer`, `front matter` in this file and the disposed-findings record — [plugin-front-matter-yaml](#plugin-front-matter-yaml), DISTINCT (the plugin gate's own reader); `front-matter-reader-siblings`, done, DISTINCT (the shared reader and the unclosed opener); owner canon-kit/SPEC.md §The shared spec adapters.

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

### install-smoke-sh-matrix

Three unix install-smoke legs in gates.yml are hand-copied jobs where a matrix over the roster's unix legs would hold them in step.

### worktree-memory-dir-key

`check-memory-off` derives the memory dir from the repository toplevel, so a linked-worktree session's memory may land where no scan reads if the harness keys that dir on a canonical working-copy root, a keying read from its bundle and never observed.

## Done

- front-door-rehearsal-rule
- contributor-writeback-disposition
- inherited-stdout-child-status
- hermetic-preamble-overlay-read
- hook-staging-fixed-sibling
- action-walk-yaml-shapes

