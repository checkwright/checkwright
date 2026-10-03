# TASK-QUEUE.md — Checkwright work queue

## Iteration: guard-ruleset-gate-neutrality-pass

  The lifecycle-kit gates read this header's iteration name and the stage cursor — the last stamp in `.workflow/WORKFLOW-STATE.txt` (lifecycle-kit/SPEC.md §The state machine); queue-kit formalizes the queue format itself and gates this file. One iteration per hardening or roadmap unit; [docs/kits.md](docs/kits.md) maps the kits.

---

## New Features

## Technical Debt

### guard-kit-ruleset-brevity

guard-kit/SPEC.md's §The generic ruleset (its preamble, §The rule roster and §Writing a consumer rule), under [spec-brevity-residue](#spec-brevity-residue)'s three moves (run-on structure, archaeology, restatement), about 22.5k of the file's 48.7k words by `wc -w` at this scope, the rule roster 21.0k of it; the last guard-kit section the moves have not passed.

**Deliverable:** the three moves over that section under `check-prose-bounds` and `check-provenance-seam`'s dated arm, every heading and rule name kept verbatim and every fact another surface cites into it kept, per a citation survey. Applied last, after [grant-slot-escaped-blank](#grant-slot-escaped-blank), whose rule `grant_path_slot` the roster cites at six sites.

**Cost while deferred:** paid by every session and adopter that reads the rule roster. Filed 2026-10-03 as a split at guard-ruleset-gate-neutrality-pass' scope. Part of the operator's selection of the unit set, direction 2026-10-03, lead-relayed (not a ruling).

## Deferred

### audit-trigger-mirror-component

[cost: iteration/low] [surface: lifecycle-kit]

`check-stage-entry`'s assertion C reads a generated SPEC mirror (`docs/<kit>/SPEC.md`) as a roster dir, so a single-kit iteration whose amendments name their mirror as a regenerate target reaches two components on that token alone and is refused build entry without an align stamp or a waiver. lifecycle-kit/SPEC.md §check-stage-entry states the behaviour and prices it under C's honest limit, so the over-demand is designed, not a bug.

**Deliverable:** a way for a consumer to declare a generated mirror as the projection of its source component, so the token resolves to the component it mirrors, specified in §check-stage-entry with a fixture over a mirror-naming single-kit amendment; or a ruling that the waiver stays the valve.

**Cost while deferred:** an align dispatch or an operator waiver ask per single-kit iteration whose amendments name their mirror. Filed 2026-10-03 to the gap inbox at guard-kit-write-side-pass' build; promoted at its close: →fix fails because the mirror's resolution is a grammar decision the SPEC settles the other way, →forward because no recorded ruling is reversed. Re-verified: the SPEC's roster-dir paragraph names this exact case. Owner lookup: `mirror`, `assertion C`, `audit-trigger` in this file — none; owner lifecycle-kit/SPEC.md §check-stage-entry.

### grant-slot-escaped-blank

[cost: event/low] [surface: guard-kit]

Rule `grant_path_slot` false-blocks a backslash-escaped blank inside a slot operand: `rm -rf .tmp/x\ y` under a `Bash(rm -rf .tmp/*)` grant blocks, naming `y` as a second operand outside the slot. The dequoted view (`native/src/guard/bash.rs`, `dequoted`) keeps `\ ` as a backslash and a blank rather than a held sentinel, so the shared word split cuts where the shell does not.

**Deliverable:** guard-kit/SPEC.md §The reader and its views states how the dequoted view holds an escaped blank, the reader follows it, and a decision-table row pairs the escaped-blank slot operand with a real second operand.

**Cost while deferred:** a false block on a rare spelling, in the safe direction. Filed 2026-10-03 to the gap inbox at guard-kit-write-side-pass' build (predates the iteration); promoted at its close: →fix fails because the dequoted view is shared by every rule declaring it, so the change is a reader contract with cross-rule reach; →forward because no ruling is owed. Re-verified: the hook blocks that command under this repo's grant. Owner lookup: `escaped`, `backslash`, `grant_path_slot` in this file — none; owner guard-kit/SPEC.md §The reader and its views.

### guard-kit-violation-recipe

[cost: once/low] [surface: guard-kit]

guard-kit registers `check-door-binding` but ships no `smoke/violation.sh`, which gate-sdk/SPEC.md §Consumer smoke makes owed wherever a battery-reddening violation is craftable; the nine other kits each ship one.

**Deliverable:** `guard-kit/smoke/violation.sh` planting a `check-door-binding` violation in the scratch consumer, run green by `--run-consumer-smoke`, and guard-kit/SPEC.md §Testing's smoke paragraph naming it.

**Cost while deferred:** the consumer smoke never proves guard-kit's one gate reds a scratch consumer. Filed 2026-10-03 to the gap inbox at guard-kit-write-side-pass' build; promoted at its close: →fix fails because a violation recipe is new mechanism; →forward because no ruling is owed. Re-verified: `ls */smoke/violation.sh` lists nine kits and not guard-kit. drift-kit/SPEC.md's stale *as guard-kit does* clause was fixed at the same close. Owner lookup: `violation.sh`, `violation recipe` in this file — none; owner guard-kit/SPEC.md §Testing.

### amendment-manifest-prose-span

[cost: event/low] [surface: gate-sdk]

`check-graph`'s amendment-manifest reader (`native/src/gates/graph.rs`, the inline-span loop beside `validate_amend_manifest`) takes every backticked span opening `# graph: ` on any `SPEC-*.md` line as a manifest, so a prose sentence naming a manifest key mid-paragraph reds assertion G for missing `dir=`/`valve=`/`tier=` and an empty `couples=`. An amendment author must rephrase around the token.

**Deliverable:** gate-sdk/SPEC.md §check-graph states which spans are manifests, so a mid-sentence mention is told apart from an embedded manifest, and the reader follows it, with a bad/good fixture pair over a mid-sentence mention.

**Cost while deferred:** a false red at amendment authoring for prose that names a manifest key. Filed 2026-10-02 to the gap inbox at consumer-smoke-driver-pass' spec; promoted at its close: →fix fails because which spans count is a grammar decision the SPEC does not settle, →forward because no ruling is owed. Re-verified: an untracked root `SPEC-zzprobe.md` carrying the mid-sentence span turns `check-graph` red on four AMEND-MANIFEST lines. Owner lookup: `# graph:`, `mid-line`, `amendment manifest`, `assertion G` in this file — none; owner gate-sdk/SPEC.md §check-graph.

### criterion-five-omit-stale

[cost: event/low] [surface: gate-sdk]

gate-sdk/SPEC.md's criterion 5 still prices a port against an omit-and-declare install outcome the installer retired: an unrostered or hasher-less host is now refused at the bootstrap, and the installer writes no `# omitted:` record (installer/SPEC.md §Hosts refused at the bootstrap, §The gate binary). The *ruled condition* paragraph, the residual's instrument (an artifact-free install whose `gates.list` declares the dispatched set at a non-zero count), the measurement order, the growth predicate, the criterion-5 bullet and exception class (b), §Binary dispatch's install-model sentence and the binary meta-gates' shared predicate all describe the retired outcome, contradicting the same SPEC's *omission branch* and *all-omitted install refuses* paragraphs. The cut records ruling *accept and declare* read as current.

**Deliverable:** criterion 5 restated against the bootstrap refusal, deciding what a cohort's binary-less price measures now that an uncovered host gets no install; the stale sentences rewritten or cut, and each per-cut record's criterion-5 ruling either kept as a record or re-pointed.

**Cost while deferred:** a porting session reads an instrument that no longer exists and a price the install model no longer charges. Filed 2026-10-03 to the gap inbox at consumer-smoke-driver-pass' build; promoted at its close: →fix fails because the stale reach spans a dozen paragraphs and what criterion 5 prices now is a design call, →forward because no ruling is reversed (the installer's own SPEC already retired the outcome). Re-verified: the binary-less leg asserts the refusal (installer/SPEC.md §The consumer smoke, *artifact-less refusal leg*), and `native/src/emit/installer_smoke/profiles.rs` fails any install carrying an omission record. Owner lookup: `criterion 5`, `omission`, `binary-less`, `accept and declare`, `residual`, `artifact-free`, `uncovered` in this file — none; owner gate-sdk/SPEC.md §Porting a gate to the binary substrate.

### required-sections-literal

[cost: event/low] [surface: queue-kit]

`QUEUE_KIT_REQUIRED_SECTIONS`'s default (`native/src/knobs/queue_kit.rs`, `REQUIRED_SECTIONS`) re-lists as literals the names the active, deferred and done section knobs own, plus `Lessons Learned`; only the icebox is composed in (`native/src/queue.rs`, `required_sections`). An adopter renaming a section through its own knob, or dropping the optional Lessons channel, must restate the whole required list, or `check-queue-sections` reds on the old name.

**Deliverable:** the default composed from the section knobs (Lessons Learned only while the lesson channel is on), with a fixture renaming the deferred section through its knob alone, queue-kit/SPEC.md §Layout and configuration updated, and a release declaration for the changed default.

**Cost while deferred:** a second copy of each section name a renaming adopter must keep in step; this repo already restates the list in `scripts/queue-config.knobs` to drop Lessons Learned. Filed 2026-10-01 to the gap inbox at lifecycle-queue-value-pass' build; promoted at its close: →fix fails because it changes a shipped knob's default, adopter-visible semantics no amendment settled, →forward because no ruling is owed. Re-verified: the literal list and the icebox-only composition read as filed. Owner lookup: `REQUIRED_SECTIONS`, `required list` in this file — none; owner queue-kit/SPEC.md §Layout and configuration.

### manifest-finder-untracked-walk

[cost: event/low] [surface: canon-kit]

canon-kit/SPEC.md §check-md-refs says the scan runs over tracked sources only, but the shared manifest-set finder (`native/src/spec.rs`, `manifest_files`) walks the disk, so an untracked `vendor/lib/README.md` in an adopter tree is scanned: a link to an absent file reds `check-md-refs`, and a 'Previously' line reds `check-manifest-temporal`. Every manifest-set gate shares the finder.

**Deliverable:** the finder takes the tracked set, or each member's SPEC states the walk — a per-member call, since some fixture cases run outside a git repository — with a fixture case carrying an untracked manifest.

**Cost while deferred:** an adopter's untracked vendored or generated READMEs red defect-class gates on content they never authored. Filed 2026-10-01 to the gap inbox at install-disposition-pass' spec; promoted at its close: →fix fails because the finder feeds every manifest-set member and the tracked-versus-walk choice is per member, →forward because the SPEC already rules md-refs' side. Re-verified: a nested probe repo with a committed `CLAUDE.md` and an untracked `vendor/lib/README.md` linking `absent.md` turns `check-md-refs` from clean to exit 1. Owner lookup: `manifest set`, `find_named`, `untracked.*README`, `tracked sources`, `tracked set` in this file — none; owner canon-kit/SPEC.md §The shared spec adapters.

### kept-gate-disposition-report

[cost: event/low] [surface: installer]

init and update print nothing for a gate whose install disposition moved off zero-config while an adopter-edited `gates.list` keeps it. init's kept-registry report covers only recipe `unregister.list` and `--without-gate` drops, so the members install-disposition-pass moved to `on-surface` stay registered silently in an edited registry.

**Deliverable:** a report of kept members the derivation no longer starts, which needs the lock to record the prior starting set, since a deliberately registered on-surface gate is otherwise indistinguishable.

**Cost while deferred:** an adopter who edited `gates.list` is not told which kept gates are now house rules they never chose; the release note's `--with-gate` line is the only notice. Filed 2026-10-01 to the gap inbox at install-disposition-pass' build on a lead decision; promoted at its close: →fix fails because the report needs a new lock field, adopter-visible mechanism no amendment settled, →forward because no ruling is owed. Re-verified: the report is fed only by the recipe and `--without-gate` drop sets. Owner lookup: `still.registered`, `kept members`, `kept-registry`, `edited gates.list`, `prior starting set` in this file — none; owner installer/SPEC.md §What init seeds.

### spec-pointer-bare-section-mark

[cost: event/low] [surface: canon-kit]

`check-spec-pointer`'s unqualified prose-citation form reads a bare `§N` (or `§2.1`) in adopter prose as a heading citation and reds "§heading in no governed file", under `## 3. Terms` as under `## Terms`, for documents that use `§` as a section-number mark. Operator direction 2026-10-01, lead-relayed (not a `/consult` ruling): state the limit, file the narrowing.

**Inferred, not run:** how many adopter documents carry a bare `§N` outside legal-style text.

**Deliverable:** a fragment opening with a digit, a bare section number, never fires in the unqualified form (native/src/gates/spec_pointer.rs, the fragment-opening rule), with a `good/` fixture line and canon-kit/SPEC.md §check-spec-pointer's carve-out sentence and honest limit updated.

**Cost while deferred:** a `prose` or `full` install over a document citing `§3` reds a defect-class gate on content that is not wrong, which the install-disposition amendment states as the gate's honest limit. Filed 2026-10-01 at install-disposition-pass's align: the probe reddened both heading shapes; →fix fails because narrowing changes the gate's asserted behavior, an envelope change.

### foreign-vendor-critique

[cost: event/low] [surface: delegation-kit]

a consumer's use of the foreign-vendor run (delegation-kit/SPEC.md §The foreign-vendor run) for critique alongside the deterministic gates. Operator direction 2026-09-30, lead-relayed, not a ruling; the uses are candidates. Two adapters sit in the gitignored local overlay, each pinning its model and effort in argv so a run never follows the vendor CLI's last-switched model: a review adapter for typical work, and an expert adapter for top-expertise work. Candidate uses: work review, for example at iteration close; strategic consultation, for example a SWOT or an architecture discussion.

**Why design-pending:** a close-stage review hook is new stage mechanism; each live run takes an operator grant, and a standing grant for a recurring review is the operator's to give; whether foreign adapters map onto `DELEGATION_KIT_TIER_MODEL`'s classes belongs to [heterogeneous-agent-delegation](#heterogeneous-agent-delegation)'s next slice. The foreign subscription's budget is small beside the primary harness's, so an exhausted foreign budget never blocks an iteration — a review degrades to skipped-with-notice — while a strategic consultation may wait for the limit to reset. Distinct from that entry, which owns transport and the adapter seam; this one is a consumer's use of it. Vendor model literals stay in consumer config (gate-sdk/SPEC.md §The provenance seam).

**Cost while deferred:** a second-vendor reading of the iteration's work is foregone, and the deterministic gates stay the only critique. Filed 2026-09-30 to the gap inbox by the lead; promoted at delegation-tier-binding's close: →fix fails because the hook is new mechanism, →forward because the direction is already given and only its per-run grants are the operator's. Owner lookup: `critique`, `foreign-run`, `review adapter` in this file — only heterogeneous-agent-delegation, whose subject is transport; owner delegation-kit/SPEC.md §The foreign-vendor run.

### spec-pointer-unstated-literal

[cost: event/low] [surface: native]

ported gate modules carry `spec:` comments citing a section for a literal awk or regex spelling, or a transcription note about the retired shell form, that the section states only as policy or not at all. `check-spec-pointer` checks only that the target resolves, so it cannot see this. Instances: `native/src/gates/gate_fail_closed.rs` (the `(awk|jq)` boundary regex, the per-FNR state machine), `native/src/gates/install_disposition.rs` (the `sub(/^# install:.../)` read, the `check-[a-z0-9]+` pattern) and `native/src/gates/assertion_strength.rs` (the awk hash-order note).

**Inferred, not run:** the class reaches past these three modules into the rest of the crate's ported gates.

**Deliverable:** a sweep of the crate's `spec:` comments against their cited sections, each unstated one retagged `comment-tier-exempt:` where the fact is local, deleted where it is port residue, or moved to the section where it is a contract.

**Cost while deferred:** a reader following such a pointer finds no support for the literal, and a later SPEC edit cannot tell the comment depends on it. Filed 2026-09-26 to the gap inbox by gate-sdk-meta-gate-brevity's citation survey; promoted 2026-09-26 at front-door-release's close: →fix fails because the class spans the crate and each comment needs a local-versus-contract call. The survey's other instance, `native/src/hook/stop_liveness.rs` citing §check-test-hermetic for per-case scratch roots, was fixed at the drain, as were eight in `native/src/emit/pack_installer.rs` citing §The packer (2026-09-29 drain), which shows the class reaching emit arms too. Folded in at preview-readiness' close, from `drift-kit-measurement-brevity`'s citation survey, all pre-dating this entry: the KPI members under `native/src/emit/kpi/` (`overhead.rs`, `gate_runtime.rs`, `gate_backlog.rs`, `task_split.rs`, `incident_recurrence.rs`, `queue_net_delta.rs`, `amendment_age.rs`) citing §Bundled KPIs, and `native/src/emit/stage_economics.rs` and `native/src/history.rs` citing §The stage-economics meter, for window sizes, rounding, age bands, tie-breaks, regexes and iteration orders neither section states. Owner lookup: `spec-pointer`, `comment-tier-exempt`, `shell form` in this file — none; owner canon-kit/SPEC.md §check-comment-tier.

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

foreign agents. Cross-vendor stage dispatch: a lead delegating a stage to a foreign coding agent. It cashes the public no-lock-in claim — governance enforced at the git/gate boundary, not by trusting the author; the verification substrate and the shared git-index/HEAD serialization are already agent-neutral. *Remaining, worst-first:* (1) the **escalation resume model**, plumbing of (2); (2) **dispatch transport** — today the harness `Agent`/`SendMessage`/task-notification; a foreign agent needs a transport-neutral "open / prompt / permission-request / resume" handoff over each vendor's **machine plane, never its TUI** (a screen relay yields frames not turn events and bets on the least-stable surface); (3) **budget oracle** — N vendor-keyed oracles, the credential-swap entries' seam, fed by the vendors' JSONL token-usage events; (4) **stage-contract expression** — the stage-skill prose is not vendor-neutral.

**First slice landed 2026-09-30 (delegation-tier-binding):** the foreign-CLI executor, delegation-kit/SPEC.md §The foreign-vendor run — `--foreign-run` runs a read-only audit or mechanical sweep in a scratch clone, returning a report and a patch, adapters as consumer config. Live acceptance, operator direction 2026-09-30 lead-relayed (not a ruling): one read-only audit through the codex CLI, `codex exec --sandbox read-only --ephemeral --color never -`, returned OK; the spec's "one vendor CLI installed" premise was wrong (two are), and the operator chose codex. **Next slice:** (2), with (1) riding it. **Horizon, operator direction 2026-09-30, lead-relayed (not a ruling):** `now` — delivery has started, and close moves a horizon on actual work.

**Seam ruling (on record):** generic mechanism only — transport, budget oracle and escalation channel are consumer-config seams; a kit literal naming a vendor crosses the provenance seam, the pattern the retired `prose-profile` ruled. Interacts with [hosted-attestation-service](#hosted-attestation-service), [plugin-harness-reach](#plugin-harness-reach) and the credential-swap entries.

**Demand attested (2026-07-23):** the operator holds three foreign-vendor subscriptions and wants read-heavy delegation routed to them for budget headroom. **Its citers** ([companion-toolkit-profile](#companion-toolkit-profile), the credential-swap entries) block on none of it.

**Attribution, operator direction 2026-09-30, lead-relayed (not a ruling):** foreign harnesses' commit trailers off and the method stated once in the README, held by construction (foreign work lands in the delegating session's commit); a harness setting change waits on operator confirmation.

**Design memory (2026-07-25, 2026-08-02):** a TUI relay buys no resume or token efficiency — both live in the vendor's session store, so interactive-vs-headless is rendering, not state; headless warm-resume by session id and JSONL turn events ship on the installed binaries probed. The machine profile (context-kit/SPEC.md §bin/env-probe, local-only) owns which CLIs and how.

**Cost while deferred:** stage-level work still bills one vendor's budget while three subscriptions are held, and this design memory ages against fast-moving CLIs. Surfaced 2026-07-17 in the release-in-lifecycle lead session (operator question).

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

**The gain is the ENCODING, not the list, and the entry says so because the format already has the list.** queue-kit/SPEC.md §The tag algebra, the `recurrence:` declaration paragraph, defines `recurrence: <slug> <YYYY-MM-DD> [<YYYY-MM-DD>…]`, multiple dates on one line today. **A second interaction dissolves with it.** queue-kit/SPEC.md §The tag algebra, the self-naming-slug paragraph, grounds the field partly in `check-queue-hygiene` rejecting exact-duplicate lines, naming same-day recurrence on two entries as "exactly the case the declaration exists to record". Under a minute-bearing stamp those two lines stop colliding at all, so one of that field's two stated grounds is retired by the encoding rather than argued against.

**The costs, probed rather than listed, because a reader meeting this cold should price it.** Date stamps span `recurrence:` and `ruled:` declarations, filed-prose provenance lines, gap-inbox bullets, survey-record headings and WORKFLOW-STATE stamps; the evidence manifest's trailing date field is OPTIONAL and so is not a cost, correcting the relayed picture. FOUR crate gates carry a date predicate (`stage_evidence.rs`, `stage_entry.rs`, `gap_inbox_neutrality.rs`, `evidence_manifest.rs`, the first two spelling their own `is_date`), and SEVEN shell tools stamp `date +%F` outside fixtures and smoke, none of which stamps a time today. `YY` also drops the century, a deliberate trade rather than an oversight to find later.

**Why design-pending:** the ruling fixes the DIRECTION and not the grammar. Open: which components take the new encoding and in what order, whether the change is a migration or a read-both-write-new window, and what each date-reading gate asserts across it — a wrong answer reds every governed surface at once.

**Cost while deferred:** low and bounded — every entry needing discrimination keeps buying it with text against the entry budget. Filed 2026-09-01 by close under CLAUDE.md §Housekeeping's operator-directed exception; it rides no cut and is no hotfix.

### spec-brevity-residue

[roadmap: now/adoption] [cost: session/high] [surface: delegation-kit] [roadmap-summary: Kit SPECs that state their contracts without run-ons, history or restatement.]

the per-SPEC remainder of `spec-tier-brevity-pass`'s three moves (run-on structure, archaeology, restatement), outside the five sections that entry landed. In the filing profile's order, by size: gate-sdk landed in eight slices, as `gate-sdk-framework-brevity`, `gate-sdk-remainder-brevity`, `gate-sdk-porting-brevity`, `gate-sdk-native-brevity`, `gate-sdk-runner-brevity`, `gate-sdk-meta-gate-brevity`, `gate-sdk-tooling-brevity` and `gate-sdk-tail-brevity`; lifecycle-kit, installer and guard-kit are finished or split out (the slices below); what remains starts at delegation-kit (43.9k), canon-kit (39.1k), queue-kit (26.5k), drift-kit (22.1k), context-kit (19.8k), evidence-kit (14.9k), site-kit (9.5k), then doctrine-kit's DOCTRINE.md and SPEC.md. Horizon `now`, operator direction 2026-09-29, lead-relayed (not a ruling): a slice has landed at each recent scope; the per-kit slices stay off the roadmap.

**Deliverable:** the three moves applied SPEC by SPEC in that order, under the gates the first slice landed, `check-prose-bounds` and `check-provenance-seam`'s dated arm; one SPEC, or a batch of the small ones, per iteration, and gate-sdk in slices, since no iteration passes it whole. Not a wholesale cut: a contract sentence stays.

**Split ten times, 2026-09-25 to 09-27 at scope, each on an operator direction lead-relayed (not a /consult ruling):** gate-sdk's framework, remainder, porting, native-contracts, runner, meta-gate, tooling and tail slices each left as their own debt entry, all since landed, as did the ninth and tenth, installer's contract and lifecycle-kit's template sections.

**Cost while deferred:** paid by every session that opens a section not yet passed and every adopter who reads one on the site. Filed 2026-09-25 at scope, split from spec-tier-brevity-pass on an operator direction, lead-relayed; the profile and sampled tables are in that entry's filing commit.

Each on an operator direction lead-relayed (not a /consult ruling): lifecycle-kit's sections above §Per-component contracts, its state-machine tool sections and its remaining sections left 2026-09-28 to 09-29 at the consult-inbox-front-brevity, lifecycle-machine-brevity and native-hook-customer-legs scopes, landing as `lifecycle-kit-front-brevity`, `lifecycle-kit-machine-brevity` and `lifecycle-kit-tail-brevity`; installer's install-surface sections (§The verbs through §The manifest) and its remaining sections other than §The consumer smoke left 2026-09-29 at the companion-technical-gates and companion-adoption-landing scopes, landing as `installer-install-brevity` and `installer-remainder-brevity`; and, the kit's other sections remaining: drift-kit's measurement sections left 2026-09-29 at preview-readiness' scope, landing as `drift-kit-measurement-brevity`; guard-kit's front sections, delegation-kit's tier sections and canon-kit's amendment-family sections left 2026-09-30 at the guard-kit-steering, delegation-tier-binding and canon-kit-value-pass scopes, landing as `guard-kit-front-brevity`, `delegation-kit-tier-brevity` and `canon-kit-amendment-brevity`; canon-kit's §Layout and configuration and seven gate sections left 2026-10-01 at install-disposition-pass' scope, landing as `canon-kit-gate-brevity`; context-kit's four feature-edited sections left 2026-10-01 at context-kit-value-pass' scope, landing as `context-kit-feature-brevity`; queue-kit's eleven gate sections left 2026-10-01 at lifecycle-queue-value-pass' scope, landing as `queue-kit-gate-brevity`; guard-kit's tool sections left 2026-10-01 at gate-sdk-value-pass' scope, landing as `guard-kit-tool-brevity`.

delegation-kit's §The delegation model and installer's §The consumer smoke left 2026-10-02 at the companion-tier-delegation-pass and consumer-smoke-driver-pass scopes, landing as `delegation-kit-model-brevity` and `installer-smoke-brevity`; guard-kit's remainder and §The generic ruleset left 2026-10-03 at the guard-kit-write-side-pass and guard-ruleset-gate-neutrality-pass scopes, landing as `guard-kit-remainder-brevity` and [guard-kit-ruleset-brevity](#guard-kit-ruleset-brevity).

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

### release-note-section-set-derivation

[cost: event/high] [surface: installer] [recurrence: 2026-09-27]

release notes serve human upgraders poorly, and their section set is crate literals. A note (installer/SPEC.md §The upgrade contract) has one human section, In brief, then declaration-bearing sections a mechanical consumer reconciles; the v0.26.0 note is 34,527 words against v0.25.0's 1,058. The section names are string literals in the release gates, the declaration parser and the upgrade smoke, so a rename is a crate edit, a new section is added by copying a call, and no gate asserts the gates' set equals the page's. Operator directions, 2026-09-27 (lead session): add sections for adopters with custom gates, whose reconcile differs; put a summary table at the top linking to the detail; reconsider the names, since "Tightened gates" also holds new gates, "Renamed knobs" also carries removals and new knobs have no section; and no hard code in gates. An operator question the same day asked where new platform support goes: this iteration's musl switch was declared as two Behavior changes bullets.

**Deliverable:** a note structure with a linked summary table, audience-keyed sections and a Platforms section derived from the diff of docs/install.md's gated platforms table between two releases, its section set one knob-owned roster every reader derives from. The lead tokens are machine-read over a historical corpus the upgrade smoke resolves at any FROM/TO, so a rename owes an alias window or a note-corpus migration.

**Cost while deferred:** every release ships a note a human must read whole to act on, custom-gate adopters are not told what applies to them, and a platform change hides in Behavior changes. Filed 2026-08-01 at close; iceboxed 2026-09-11 as dormant; re-filed 2026-09-27 to the gap inbox as two bullets (readability, a Platforms section) by platform-prerequisite-floors' lead, and returned at its close on a judged recurrence: →fix fails because the redesign renames machine-read tokens. Re-verified: `wc -w` over the two notes, and the literals in `native/src/gates/release_bump.rs`, `tightened_gates_grammar.rs`, `release_change_declared.rs`, `native/src/declaration.rs` and `native/src/emit/upgrade_smoke.rs`. `removed-knob-docs-cmd-valve`, done, is DISTINCT. Owner: installer/SPEC.md §The upgrade contract, RELEASING.md.

### site-video-poster-rule

[cost: event/low] [surface: site-kit]

the site has no rule for video, and the operator wants short intro and demo videos (direction 2026-09-27, the implementation delegated to the lead). Lead decision: host them on the reserved YouTube channel, a discovery channel fitting `catalog-then-plugin`'s distribution grounds that keeps video bytes out of every clone; a page shows a local poster image linking out, never an iframe player, so the site keeps zero third-party requests at page load.

**Deliverable:** a page-authoring rule in docs/site-architecture.md admitting only the poster-link form, a gate reding an `<iframe>` or a third-party `src=` in docs pages, and first homes on docs/spec-toolkits.md and the front door.

**Cost while deferred:** a video lands with no rule, and an embed would add the site's first third-party request. Filed 2026-09-27 to the gap inbox by companion-catalog-extension's lead; promoted 2026-09-27 at its close: →fix fails because the gate is new mechanism. Re-verified: `docs/_layouts/default.html` loads only local assets, and `check-docs-render-fidelity` lists `iframe` as a known tag without refusing it. Owner lookup: `video`, `iframe`, `YouTube` in this file — none; owner docs/site-architecture.md §Page-authoring rules.

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

**Deliverable:** a derived table whose columns answer an upgrader, above all whether a release needs action on upgrade (it carries Tightened gates or Renamed knobs entries); also version, date, bump class and per-section counts. The note composer writes the counts and the action flag as front-matter keys and a gate holds them equal to the note's sections; parsing sections in Liquid at render time is refused as fragile.

**Cost while deferred:** a reader skipping several versions opens each note to learn which need action. Filed 2026-09-29 to the gap inbox by companion-technical-gates' lead, at the operator's leave; promoted at its close: →fix fails because the keys are new grammar on the notes, →forward because no ruling is owed. Re-verified: the page's `<ul>` loop and 29 notes carrying `release:`. Owner lookup: `releases.md`, `front-matter` in this file — none; owner docs/site-architecture.md, with installer/SPEC.md §The upgrade contract for the keys.

### docs-chrome-page-repeat

[cost: event/low] [surface: docs]

`check-docs-page-repeat` reads page sources and never `docs/_layouts` or `docs/_includes`, so a chrome addition repeating a page's statement passes. Found at `homepage-license-duplicate`, where the footer's license line duplicated docs/index.md's License section; that unit's `check-license-line` widening holds the license instance only.

**Deliverable:** an arm prefixing the layout's and includes' literal text nodes (Liquid excluded) to every page's corpus, so a sentence of eight words or more or a link target stated in both reds; or a boundary note refusing it.

**Cost while deferred:** the next chrome addition can duplicate a page statement unseen until a reader finds it. Filed 2026-09-29 to the gap inbox by companion-adoption-landing's build; promoted at its close: →fix fails because the arm is new mechanism with a false-positive risk to calibrate, →forward because no ruling is owed. Re-verified: canon-kit/SPEC.md §check-docs-page-repeat reads each `CANON_KIT_PAGE_REPEAT_PAGES` page alone and deliberately asserts no repeat across pages, so the arm must weigh that boundary. Owner lookup: `page-repeat`, `_layouts`, `chrome` in this file — [site-video-poster-rule](#site-video-poster-rule), DISTINCT (embeds); owner canon-kit/SPEC.md §check-docs-page-repeat, with docs/site-architecture.md §Page-authoring rules. Surface also canon-kit.

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

### footprint-arm-publisher-page

[cost: event/low] [surface: context-kit]

context-kit's `--emit footprint` arm ships in the binary but prints this site's page: Jekyll front matter (`nav_parent: value`), the site's own title and regen command, and a kit roster derived from `*/SPEC.md` files the payload withholds. In an adopter's tree it measures nothing of theirs, yet context-kit/README.md lists it among the adopter tools. Its gate, `check-footprint-fresh`, was withheld at `small-kit-value-audit`, which audited gates and did not reach this non-gate arm, so this is DISTINCT from that unit.

**Deliverable:** one of two dispositions, chosen at spec: withhold the arm from the README and the payload docs as a publisher tool, or make the preamble and the roster knobs an adopter can set.

**Cost while deferred:** an adopter running a listed tool gets a page about Checkwright. Filed 2026-10-01 to the gap inbox at context-kit-value-pass' build; promoted 2026-10-01 at its close: →fix fails because the choice between the two dispositions is a design call and the second adds knobs, →forward because no ruling is owed. Re-verified: the arm's output opens with the front matter, and the README's tool list carries `--emit footprint`. Owner lookup: `footprint` in this file — none; owner context-kit/SPEC.md §bin/footprint.

### withheld-gate-literal-knobs

[cost: event/low] [surface: gate-sdk]

`check-consumer-value-literal` holds only the kit-shipped gates' crate modules. The gates withheld to `scripts/` keep their consumer-value literals: at gate-sdk-value-pass' spec census, over the 26 then withheld, 62 consumer values and 39 borderline rows, 23 of them `const DEFAULT_<NAME>` positional layout defaults. Operator direction 2026-09-27, kept on record for this half by the operator's choice of option C, 2026-10-02, lead-relayed (not a ruling): avoid any hard code in gates, because checkwright benefits from configurable gates itself.

**Deliverable:** the gate's corpus widened to every gate module, and each withheld gate's literal moved to a descriptor-declared knob (gate-sdk/SPEC.md §The declaration cohort, a knob no static kit's prefix owns) or valved with its ground.

**Cost while deferred:** no adopter meets these literals, since a withheld gate never ships. The cost is that this repo's own gates stay unconfigurable, and a new withheld gate's literal reds nowhere. Filed 2026-10-02 at gate-sdk-value-pass' spec on that direction. Owner lookup: `literal`, `withheld` in this file — `consumer-value-literal-gate`, DISTINCT (the kit-shipped half); owner gate-sdk/SPEC.md §The port-candidate criteria.

### notification-delivery-probe

[cost: event/low] [surface: lifecycle-kit]

the delivery rule under `lead-notification-wake-race`'s remedy, unprobed: does a completion notification queued during a supervisor's turn that makes no tool call wake its session, and does one tool call before the turn end drain it.

**Deliverable:** that probe, run by a lead with the operator present, since a dispatched session's turn end is its session end and cannot observe it; then lifecycle-kit/templates/lead.md §The lead model's wait clause and the agent-execution backgrounding bullet's bound confirmed or corrected against the result.

**Inferred, not run:** that a tool call before the turn end delivers the notification — two stalled transcripts show only an undrained queue at the stop, and one 2026-10-02 lead turn with tool calls received its notification inside the turn.

**Cost while deferred:** the lead's wait clause rests on an inferred mechanism; if it is wrong, a stall still costs hours of idle wall-clock until the operator wakes the lead. Filed 2026-10-02 as a split at companion-tier-delegation-pass' scope, operator direction lead-relayed (not a ruling).

### front-end-door-residue

[cost: event/low] [surface: native]

the binary still names or spawns the bash front end where an adopter meets it: the `--help` text (native/src/runner.rs `USAGE`, pinned across both front ends), the runner's `--only` steer for a gate passed as a gates dir, installer/demo.rs's walkthrough line, and `EVIDENCE_KIT_RUN_<suite>`'s default in native/src/knobs/evidence_kit.rs. Widening `GUARD_KIT_DOOR_ROOTS` from the gates, hook and emit trees to all of native/src reds exactly these.

**Deliverable:** each site respelled to the binary door, the knob default's working-directory change (the front end changes to the toplevel, the binary does not) settled in evidence-kit/SPEC.md, and the door roots widened to native/src.

**Cost while deferred:** an adopter reading `--help` or the demo meets a command their install may not carry. Filed 2026-10-02 to the gap inbox at gate-sdk-value-pass' build; promoted at its close: →fix fails because the knob default is a behaviour change across two front ends, →forward because no ruling is owed. Re-verified: grep finds all four sites. `hook-emit-remedy-door` scoped its fix to the hook and emit trees; this is DISTINCT, not a recurrence. Owner lookup: `run-gates.sh`, `DOOR_ROOTS` in this file — none; owner guard-kit/SPEC.md §check-door-binding, with gate-sdk/SPEC.md §run-gates.

### gate-sdk-layout-assumptions

[cost: event/low] [surface: gate-sdk]

four kept gate-sdk gates assume this repo's layout, found at gate-sdk-value-pass' gate audit:

- `check-gate-substrate-parity` defaults its conservation doc to `<GATE_SDK_ROOT>/SPEC.md`, which the payload withholds, so a bare run in an installed tree exits 2. The consumer smoke vendors by copy and masks it.
- `check-template-copy-parity` globs `<root>/*/templates/*.sh` rather than joining the kit roots, so under a subdirectory vendoring its corpus empties and it reads clean.
- `check-tree-terms` and `check-portability-floor` exempt their own pattern files by the shipped basename prefix, so a consumer whose pattern-file knob names another file reds on its own roster.
- `--emit enforcement-map` links each kit to `<kit>/index.md`, this site's docs layout, so an adopter's page ships one dead link per kit.

**Deliverable:** each gate made layout-neutral, with a fixture per change and gate-sdk/SPEC.md's gate sections updated, and a release declaration for each verdict change.

**Cost while deferred:** an adopter registering the parity gate meets a refusal it cannot discharge; a nested vendoring drifts template copies unchecked; a renamed pattern file reds; the enforcement page ships dead links. Filed 2026-10-02 to the gap inbox as four bullets at gate-sdk-value-pass' build; promoted at its close as one entry: →fix fails because each changes a shipped gate's verdict, →forward because no ruling is owed. Re-verified at gate_substrate_parity.rs's default and `GATE_SDK_PAYLOAD_WITHHOLD`, template_copy_parity.rs's glob, both `SELF_EXEMPT_PREFIX` constants and enforcement_map.rs's `kit_cell`. Owner lookup: `layout`, `kit roots`, `enforcement-map` in this file — [footprint-arm-publisher-page](#footprint-arm-publisher-page), DISTINCT (another emitter's page); owner gate-sdk/SPEC.md, each gate's section.

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

## Done

