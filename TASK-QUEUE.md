# TASK-QUEUE.md — Checkwright work queue

## Iteration: personal-install-residue-pass

  The lifecycle-kit gates read this header's iteration name and the stage cursor — the last stamp in `.workflow/WORKFLOW-STATE.txt` (lifecycle-kit/SPEC.md §The state machine); queue-kit formalizes the queue format itself and gates this file. One iteration per hardening or roadmap unit; [docs/kits.md](docs/kits.md) maps the kits.

  **Set record**

  **The set (operator direction 2026-10-25, lead-relayed, not a ruling; the iteration opened undirected):** option C, seven units, the install-hooks and personal-install residue plus one lead-posture rider. Debt promoted by scope: `hookspath-prior-physical`, `marker-block-final-newline`, `personal-exclude-crlf`, `link-refusal-junction-case` (the four bullets personal-install-hardening-pass' close carried in), and `smoke-git-config-isolation`. Features for spec to author and pair: `hookspath-chain`, `oracle-turn-done-marker`. `notification-delivery-probe` stays deferred. Stages scope, spec, build, validate, close; align joins if an authored amendment spans two component directories. Composition: bundled, drain 3 against inflow 1.4.

  **Order (scope, own authority):** `hookspath-prior-physical` lands before `hookspath-chain`, which records and runs the prior hooks directory and so needs it resolved physically. No other pairing among the set or the pool.

  **Push need (2026-10-25, inside the budget):** 2 against 2. One mid-iteration push after `link-refusal-junction-case`, `smoke-git-config-isolation` and `hookspath-chain` build, since a Windows leg is the first runner of each, then the closing push.

---

## New Features

### hookspath-chain

[spec: gate-sdk/SPEC.md]

`--install-hooks` refuses a clone already running hooks (landed as `hookspath-clobber`), so a personal install on a project whose tracked hook manager runs its hooks gets no gate hooks: calling `--git-hook` from that manager means editing the project's tracked files.

**Deliverable:** chain rather than refuse. Record the prior hooks directory, run its same-named hook ahead of the battery from the placed hooks, place a forwarding launcher for each other live hook, re-place them on `--refresh`, and restore the prior value at uninstall. Designed in the amendment, the native Windows path through `git hook run --to-stdin` included (git 2.36 introduced `git hook run`, above the 2.15 floor).

**Cost while deferred:** a personal adopter on such a project checks at the battery alone, never at commit. Filed 2026-10-25 by personal-install-hardening-pass' close, drained from the gap inbox. Re-verified there: a fresh clone whose global `core.hooksPath` names a directory holding a live `pre-commit` refuses `--install-hooks` at exit 2. Owner lookup: `chain`, `hooksPath` in this file and the disposed-findings record, none; owner gate-sdk/SPEC.md §install-hooks.

### oracle-turn-done-marker

[spec: SPEC-oracle-turn-done.md]

Under the split lead posture, a scope session resumed as the intent oracle after its stage ended appends its answer to its resume journal past the `DONE` line, so `DONE` is no longer the last line and lifecycle-kit/templates/lead.md's last-line completion check reads that session as unfinished.

**Deliverable:** a resumed turn that appends to its stage's journal re-stamps `DONE`, stated in the stage templates' last step and at lead.md's completion read; designed in the amendment.

**Cost while deferred:** a lead's completion read misjudges a finished oracle and falls back to asking it. Filed 2026-10-24 by personal-overlay-pass' lead after its close (seen when the oracle answered spec's workflow-profile question), drained at the next scope. Owner lookup: `DONE`, `oracle` in this file and the disposed-findings record, none; owner lifecycle-kit/SPEC.md §templates/lead.md.

## Technical Debt

### marker-block-final-newline

`install_block` and `uninstall_block` in `native/src/marker.rs` re-terminate an unterminated last line. (a) An exclude file holding `keep` with no newline, given personal installs in worktrees A then B and removals A then B, ends `keep\n`: A's removal keeps the newline B follows, so the writer meets gate-sdk/SPEC.md §lib/inject.sh's end-of-file condition while the personal install's pre-install-bytes promise fails for a non-LIFO removal. (b) Both rebuilds push `\n` after every kept line, so another block whose end marker is an unterminated last line gains a newline when this worktree's block is rewritten, against the promise that every other block stays byte-identical; the landed test compares only the extracted body.

**Deliverable:** carry the input's final-newline state through both rebuilds and record which separator an append wrote, with a byte-level case for each half.

**Cost while deferred:** a stray trailing newline in a hand-edited or multi-worktree exclude file. Filed 2026-10-25 by personal-install-hardening-pass' close (second-vendor review, read off source), drained from the gap inbox. Half (a) is read off source; its byte-level case runs it. Owner lookup: `newline`, `marker` in this file and the disposed-findings record, none; owner gate-sdk/SPEC.md §lib/inject.sh.

### personal-exclude-crlf

The personal install's exclude file carries two unstated assumptions. (a) Markers match as whole LF-split lines (`fresh::file_lines` keeps a CR), so a CRLF-converted exclude file hides an unkeyed block from mode detection and the half-open check, and an explicit `init --personal` appends a second block. (b) Two worktrees' runs read and rewrite the one shared exclude file with no lock, so concurrent runs can drop one worktree's block.

**Deliverable (operator direction 2026-10-25, lead-relayed, not a ruling):** tolerate a trailing CR on marker lines, and state the single-writer limit in installer/SPEC.md §The personal install. No lock file.

**Cost while deferred:** a CRLF-converted exclude file or two concurrent personal runs leave a duplicated or lost block. Filed 2026-10-25 by personal-install-hardening-pass' close (second-vendor review, read off source), drained from the gap inbox. Owner lookup: `CRLF`, `exclude file` in this file and the disposed-findings record, none; owner installer/SPEC.md §The personal install.

### link-refusal-junction-case

`init`'s link refusal has no native-Windows witness. The build inferred that `symlink_metadata` reads a junction as a link, but `link_below`'s unit test `a_destination_through_a_link_is_named_at_its_first_link` (`native/src/installer/mod.rs`) is `#[cfg(unix)]`, and the consumer smoke's personal arm plants no junction, so no Windows leg runs the junction case.

**Deliverable:** a Windows-gated unit case creating a junction (`mklink /J` through `cmd`, or `std::os::windows::fs::symlink_dir` where the runner allows it) under a kit directory and asserting `link_below` names it.

**Cost while deferred:** a junction under a kit root on native Windows may still receive the payload unrefused. Filed 2026-10-25 by personal-install-hardening-pass' close (read off the closing push's logs), drained from the gap inbox. Owner lookup: `junction`, `link_below` in this file and the disposed-findings record, none; owner installer/SPEC.md §init.

### smoke-git-config-isolation

The consumer smoke's scratch consumers read the host's global and system git config, which installer/SPEC.md §The consumer smoke leaves as it is by design. Since `hookspath-clobber`, a host whose global `core.hooksPath` or `init.templateDir` leaves a live hook in a fresh consumer reds every arm running `--install-hooks`.

**Deliverable:** the smoke's git children run under `GIT_CONFIG_GLOBAL` naming an empty file, and `GIT_CONFIG_NOSYSTEM` where a leg allows it (cf. `fence_run.rs`), with that section's sentence rewritten. Read the Windows legs before dropping the system file, whose Git for Windows defaults (`core.autocrlf`, `core.symlinks`) the arms may lean on (inferred).

**Cost while deferred:** a dev host so configured sees smoke reds; CI hosts are unaffected. Filed 2026-10-25 by personal-install-hardening-pass' close, drained from the gap inbox. Re-verified there: with the global file naming such a directory, `--install-hooks` in a fresh repository exits 2, and with `GIT_CONFIG_GLOBAL=/dev/null GIT_CONFIG_NOSYSTEM=1` it exits 0; no `GIT_CONFIG` name appears under `native/src/emit/installer_smoke/`. Owner lookup: `GIT_CONFIG`, `global git` in this file and the disposed-findings record, none; owner installer/SPEC.md §The consumer smoke.

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

### notification-delivery-probe

[cost: event/low] [surface: lifecycle-kit] [not-icebox-eligible: 2026-10-10 filed on the operator direction of 2026-10-02 and waits on a lead session with the operator present; evicting it would compress that direction away]

the delivery rule under `lead-notification-wake-race`'s remedy, unprobed: does a completion notification queued during a supervisor's turn that makes no tool call wake its session, and does one tool call before the turn end drain it.

**Deliverable:** that probe, run by a lead with the operator present, since a dispatched session's report is all its caller holds and no session reports on its own turn end (delegation-kit/SPEC.md §The delegation model); then lifecycle-kit/templates/lead.md §The lead model's wait clause and the agent-execution backgrounding bullet's bound confirmed or corrected against the result.

**Inferred, not run:** that a tool call before the turn end delivers the notification — two stalled transcripts show only an undrained queue at the stop, and one 2026-10-02 lead turn with tool calls received its notification inside the turn.

**Cost while deferred:** the lead's wait clause rests on an inferred mechanism; if it is wrong, a stall still costs hours of idle wall-clock until the operator wakes the lead. Filed 2026-10-02 as a split at companion-tier-delegation-pass' scope, operator direction lead-relayed (not a ruling).

### evaluator-root-agent-file

[cost: event/low] [surface: gate-sdk] [not-icebox-eligible: 2026-10-25 split on the operator direction of 2026-10-17 and waits on the operator's first selection-probe run; evicting it would compress that direction away]

surface (c) of `evaluator-agent-discovery`, split off at evaluator-discovery-pass' scope (operator direction 2026-10-17, lead-relayed, not a ruling): a root `AGENTS.md` for evaluators and adopters, since a cloning agent reads the maintainer-internal `CLAUDE.md`.

**Why split rather than built:** `check-root-tiering`'s built-in allowlist rejects a stray second agent file at root (context-kit/SPEC.md §The AGENTS.md agent-file adapter smoke asserts it), and an `AGENTS.md` is the always-loaded file of any agent that reads one, so an evaluator-facing file would also load into every such session working here, foreign adapters running stages included. **Inferred, not run:** which foreign adapters this repo dispatches read `AGENTS.md`.

**Deliverable:** design-pending, a feature: the file's audience and home and the rule `check-root-tiering` takes on it, or a refusal recorded.

**Cost while deferred:** a cloning agent's first read is maintainer-internal instructions; whether that costs a recommendation is unmeasured until drift-kit's `--selection-probe` runs (drift-kit/SPEC.md §The agent-selection probe). Run it before a scope weighs this entry, and again once its change publishes; its first run is the operator's. Filed 2026-10-17 by evaluator-discovery-pass' scope as a split remainder. Re-verified there: no root `AGENTS.md`. Owner lookup: the parent's, none; owner gate-sdk/SPEC.md §check-root-tiering.

### codex-plugin-hook-discovery

[cost: event/high] [surface: plugin]

`codex-cli` 0.162.1 and 0.163.0 (re-probed 2026-10-21 through the app server's `hooks/list`) discover no plugin hooks while the package root carries the portable `plugin.json` (with or without `extensions.com.openai` hooks or a `.codex-plugin/plugin.json` beside it); without that manifest `hooks/hooks.json` is discovered and the unchanged rendering blocks correctly. Likely upstream: Codex's build docs say `hooks/hooks.json` is discovered by default. Codex guard wiring is tier 1 (operator direction 2026-09-28).

**Deliverable:** on a Codex release discovering hooks beside the root manifest, re-run plugin/SPEC.md §The validation leg's Codex install, wire the guards, and update plugin/SPEC.md's Codex row and plugin/README.md's guard bullet. Its patch residue split off as `codex-patch-path-residue` at foreign-harness-write-path-pass' scope (operator direction 2026-10-23, lead-relayed, not a ruling), and landed there.

**Cost while deferred:** a Codex adopter gets the skills and no shell, wakeup or workflow-state guard. Filed 2026-10-18 by harness-reach-pass' close. Re-verified there: plugin/SPEC.md's Codex row and limits. Re-probed 2026-10-23: `codex-cli` on PATH is still 0.163.0. Owner lookup: `codex`, `apply_patch` in this file and the disposed-findings record, none; owner plugin/SPEC.md §The harnesses.

### codex-plugin-version-pin

[cost: event/low] [surface: plugin] [observed-by: publish]

Codex keys an installed plugin at version `1.0.0`, since neither manifest carries `version` (plugin/SPEC.md §The manifests) and Codex does not take the marketplace entry's, so whether a pin move reaches an installed copy through `codex plugin marketplace upgrade` is unrun.

**Deliverable:** run it at the next release's pin move, and fix the manifest stance if it does not reach.

**Cost while deferred:** a Codex user may stay on an old package after a release. Filed 2026-10-18 by harness-reach-pass' close. Re-verified there: plugin/SPEC.md's limit and no `version` key in `plugin/plugin.json`. Owner lookup: `codex`, `marketplace upgrade` in this file, none; owner plugin/SPEC.md §The manifests.

### cursor-muse-plugin-observed

[cost: event/low] [surface: plugin]

Cursor and Muse Code are installed on no build machine, so plugin/SPEC.md §The harnesses carries their rows read off their docs only: Cursor reads an Agent Plugin's skills and imports a project's Claude Code settings hooks by default (its `Bash` to `Shell` tool-name mapping unrun), and Muse Code installs Claude-format plugins with an undocumented hook payload.

**Deliverable:** install and run each on a machine that has it, turn its cells from read to run, and file any wiring defect.

**Cost while deferred:** two tier-2 harnesses' skill load and guard reach rest on vendor docs. Filed 2026-10-18 by harness-reach-pass' close. Re-verified there: neither CLI on the build machine's PATH. Owner lookup: `cursor`, `muse` in this file, none; owner plugin/SPEC.md §The harnesses.

### personal-workflow-export

[cost: event/low] [surface: installer]

the workflow half of `personal-adoption`: a developer runs the stage machine on a personal branch or fork of a project that does not adopt it, `init` as today, plus an export keeping commits that touch only lock-roster and workflow paths out of an upstream pull request. Untracked workflow state was refused on consult advice, since every stamp is committed and names its commit (lifecycle-kit/SPEC.md §Multi-operator semantics).

**Deliverable:** design-pending, a feature: the export arm's shape (a path-filtered branch rewrite or a patch series), how it reads the lock roster and the workflow paths without a kit literal naming a layout, and what it refuses.

**Cost while deferred:** the workflow half stays usable only by vendoring into a tree the developer does not own, or by stripping its commits by hand before an upstream pull request. Filed 2026-10-24 by personal-overlay-pass' scope, split from personal-adoption (operator direction 2026-10-24, lead-relayed, not a ruling). Owner lookup: `export`, `fork`, `personal branch` in this file, none beyond the parent; owner installer/SPEC.md.

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

- hookspath-prior-physical

