# SPEC amendment: foreign-stage-binding

The host protocol leaves the audit stage's template for one file every stage that admits it points at, and a consumer binds a stage to a foreign adapter in config instead of in each dispatch prompt. It pairs two queue entries: the cross-vendor delegation entry's next stage-contract slice (deltas 1, 2 and 3) and the per-stage executor binding (deltas 4, 5 and 6).

## What changes

### (1) The host protocol is one file, stated once for any stage {design-bearing} {user-facing: operator direction 2026-10-08 on the cross-vendor delegation entry — a host protocol for any stage but the audit stage is the next slice}

The seven steps the audit stage's template carries move to `lifecycle-kit/templates/host-protocol.md`, worded for the stage the reading session entered rather than for one stage. A stage template that admits the protocol carries one line that points at the file and restates none of it. The file is load-triggered: a session reads it only when its dispatch names an adapter, so the stages that run on the master harness pay nothing for it.

The step that composes the prompt gains one clause. The host appends what its dispatch gave it that varies, and nothing read from a file the repository ignores. The clone already withholds ignored content from the foreign agent (delegation-kit/SPEC.md §The foreign-vendor run, **The tree**), and a host that pasted a private brief into the prompt would hand a vendor what the clone was built to keep back.

Replacement text for `lifecycle-kit/templates/host-protocol.md`, a new file. **Not yet applied.**

```text
The host protocol: your dispatch names a foreign adapter, so this stage's reading runs there and every write stays yours.

1. Enter, stamp and verify your tier as any session of your stage does.
2. Write `--emit stage-contract <your stage>` to a prompt file in the scratch dir and append what your dispatch gave you that varies. Append nothing read from a file the repository ignores.
3. Run it as an audit-mode `--foreign-run` on the named adapter, under delegation-kit's foreign-run bullet (delegation-kit/templates/agent-execution.md): the budget read first, backgrounded with its liveness record.
4. Check each finding before you build on it. Perform each proposed write you accept yourself, in your stage's own commits. Answer a question from the governed surfaces or escalate it, and return an answer with `--foreign-resume` where the adapter resumes.
5. Run yourself every command the report names as refused or as needing a built artifact, your stage's gates among them.
6. Record the run's verdict line in your journal and in the message of the commit that lands its findings.
7. A run that ends `FAILED` or `REFUSED` returns nothing to build on: execute the contract yourself, and where that needs a tier above your dispatch's, escalate a re-tier.
```

Step 5 widens the audit stage's "run this stage's consistency gate yourself": the frame already has the foreign reader name each command its sandbox refuses or that needs a built artifact (§The stage-contract arm), and outside the audit stage those are more than one gate.

### (2) Every shipped stage template but the build stage's points at it {design-bearing} {user-facing: operator direction 2026-10-08 on the cross-vendor delegation entry — the next slice; which stage a consumer sends foreign stays its own choice, delta 4}

The corpus is `ls lifecycle-kit/templates/stages`, six files. Each member's value:

- `align.md` — its seven-step block is replaced by the pointer line.
- `scope.md`, `spec.md`, `validate.md`, `close.md` — each gains the pointer line in its session ritual.
- `build.md` — takes no line.

The pointer line, identical in the five. **Not yet applied.**

```text
**Where your dispatch names a foreign adapter**, this stage's reading runs there and every write stays yours: follow lifecycle-kit/templates/host-protocol.md (lifecycle-kit/SPEC.md §The host protocol).
```

**A stage admits the protocol when its work separates into a reading and writes a session can land from a report.** The frame returns every write as a proposal carrying its full text, which the host checks and lands. Authoring, auditing, surveying and reviewing separate that way. The build stage's work does not: its writes are proved by running what they produce, a loop that needs the write path and a built artifact inside it, and a reader that returns code as report text gives the host the whole build to redo. That stage waits for the entry's remaining item, a stage whose writes a foreign agent performs.

**What a foreign reading cannot reach, at any admitted stage**, stated in §The host protocol:

- An oracle needing a built artifact, the gate binary first. The host runs it, so a stage that is mostly oracle-running returns mostly a command list, and a consumer weighs that before binding one.
- Content the repository ignores or has not committed. A stage whose reading rests on a private surface gets a reading without it, and the host supplies that part.
- A remote act: a push, a release, an authenticated read of a hosting service. The first two are writes and the third is the sandbox's to refuse; each is the host's.

**The kit instructs the set and recommends none.** Which admitted stage is worth sending is the consumer's, through delta 4's binding, on the arm's standing sentence that the reading worth sending is the dispatcher's judgment.

### (3) §The host protocol is a section of its own, and the arm's last bullet follows it {mechanical}

The two host paragraphs under §templates/stages/ move under a new `### The host protocol` heading placed after §The stage-contract arm, re-phrased for any admitting stage, carrying delta 2's admission rule and limits. Three sentences change with it:

- §templates/stages/ keeps one sentence citing the new section in place of the two paragraphs.
- §The stage-contract arm's bullet *It emits for any configured stage* ends "and the kit instructs every stage whose template points at the host protocol (§The host protocol)" in place of "and the kit instructs one, the audit stage". The arm's behaviour is unchanged: it still emits for any configured stage.
- The host paragraph's closing sentence, that a per-stage executor binding is not built, is replaced by a citation of delta 4's knob. **Where the executor-binding entry does not land in the same batch**, that sentence stays as it reads today and deltas 4, 5 and 6 are not merged.

### (4) `LIFECYCLE_KIT_STAGE_EXECUTOR` binds a stage to a foreign adapter {design-bearing} {user-facing: operator direction 2026-10-03 on the executor-binding entry — a per-stage executor binding as consumer config, a consumer-selectable set with the all-master-harness posture one member}

An indexed knob, one `<stage>=<adapter>` element per bound stage. A stage no element names runs on the master harness, and the empty default is the all-master-harness posture, so the shipped calibration is off and each consumer binds its own. The adapter is a name from `DELEGATION_KIT_FOREIGN_ADAPTERS`, whose argv already carries the vendor's program, model and effort (delegation-kit/SPEC.md §The foreign-vendor run), so this knob holds no model and no vendor.

**lifecycle-kit owns it, not delegation-kit's tier binding.** A stage is this kit's vocabulary, and a consumer vendoring delegation-kit alone has no stage to bind. The tier binding maps a class to a master-harness model and its three readers all read master-harness model ids; an adapter binds to no class. So the two bindings share a shape and no reader, and the queue entry's filing-time pointer at §The tier binding is corrected here rather than followed.

The table validator refuses, at exit 2 with every finding: an element with no `=`, a stage outside `LIFECYCLE_KIT_STAGES`, a stage bound twice, an empty adapter, and an adapter name outside `[a-z0-9-]`, the shape delegation-kit's validator holds an adapter name to. It does not read `DELEGATION_KIT_FOREIGN_ADAPTERS`: a kit's validator reads its own table, and an adapter no knob configures is already `--foreign-run`'s `FAILED` at exit 2 before anything is cloned. Being indexed, the knob has no environment spelling.

**A binding usually lives where its adapters do.** The kit ships no adapter, so a consumer's adapters sit in delegation-kit's gitignored overlay, and a tracked binding would name an adapter a fresh clone does not configure. The gitignored `lifecycle-config.local.knobs` overlay (gate-sdk/SPEC.md §The knob file) is its usual home. This repo binds no stage with this amendment: the knob lands unset, and binding one is the operator's choice.

### (5) `check-stage-skill-coverage` holds a bound stage to a surface that carries the protocol {design-bearing} {user-facing: operator direction 2026-10-08 on the executor-binding entry — only a stage with a host protocol can be bound foreign, so the member set follows the slice}

A **fourth direction**: every stage `LIFECYCLE_KIT_STAGE_EXECUTOR` binds has an executed surface carrying the citation `lifecycle-kit/SPEC.md §The host protocol`. A bound stage whose surface lacks it reds, naming the stage, the surface and the two remedies: unbind the stage, or carry the protocol's pointer.

The set of stages that may be bound is therefore derived from the templates and never listed. A roster knob of host-capable stages was weighed and refused: it would be a hand-kept copy of a fact the surfaces already state, drifting the first time a template gains or drops the line. The marker is a citation on the third direction's ground, a token `check-spec-pointer` resolves, so no sentence is matched; the executed surface and the whitespace collapse are that direction's, reused. A copy-and-specialize consumer admits a stage by carrying the same citation in its own skill, which is the act of adopting the protocol.

The direction is inert while the knob is empty, and the clean line says how many bound stages it read, so a tree binding none reads differently from one whose bindings passed. The member's declared knobs gain `LIFECYCLE_KIT_STAGE_EXECUTOR`. Its `# graph:` couples are unchanged: a knob-file edit lies outside them and the whole-tree battery backstops it, the limit the section already states for a stage-set edit.

**Honest limit.** The gate proves the surface points at the protocol, not that a session follows it or that the adapter is configured. And where the binding lives in the gitignored overlay, the direction reads it on that machine alone.

### (6) The lead reads the binding before it dispatches a stage {design-bearing} {user-facing: operator direction 2026-10-03 on the executor-binding entry — the pick leaves each dispatch prompt for config}

The read is `--emit knob-values LIFECYCLE_KIT_STAGE_EXECUTOR`, the generic knob read, so no arm is minted. Where an element names the stage about to be dispatched, the lead names that adapter in the dispatch prompt, the one channel a stage session already reads it from, and tiers the host by what its writes demand (§The host protocol). The stage session's side is unchanged: it follows the protocol because its prompt names an adapter, and never reads the knob.

**The binding is the default and the lead may withhold it for a cause.** The adapter's keyed verdict is read first with `--foreign-run <adapter> --budget`; on a `PAUSE` the lead dispatches the stage on the master harness and journals the line. A binding that always won would stall an iteration on another vendor's window, and the host's own fallback (step 7) would discover the pause one dispatch later at a tier chosen for a foreign reading. A dispatch may also name an adapter for a stage the knob leaves unbound, as today.

Replacement text for `lifecycle-kit/templates/lead.md`, one paragraph beside its stage-dispatch rule. **Not yet applied.**

```text
**Read the stage's executor before you dispatch it.** Run `--emit knob-values LIFECYCLE_KIT_STAGE_EXECUTOR`. Where an element binds the stage, read that adapter's `--foreign-run <adapter> --budget`: on `OK`, `RESET-OK`, `STALE` or `OFF`, name the adapter in the dispatch prompt and tier the host by what its writes demand; on `PAUSE`, dispatch the stage on this harness and journal the line.
```

delegation-kit's sentence that the dispatcher picks an adapter by name for the unit's class gains the citation that a stage's pick may be bound here. The standing dispatch policy in a consumer's stage-session definition already lists a foreign adapter among what a prompt varies, and is not edited.

## Producers and consumers

- **`templates/host-protocol.md`** — producer: this amendment. Consumer: a stage session whose dispatch names an adapter, reaching it through its stage template's pointer line. No gate reads the file's wording.
- **The pointer line** — producer: the five templates (delta 2). Consumers: the stage session, and `check-stage-skill-coverage`'s fourth direction, which reads its citation. `check-spec-pointer` resolves the citation against the heading delta 3 adds, so the heading and the lines land in one commit. `check-shim-restatement` compares a shim with the template it binds and reads no template against another, so five identical lines are not its subject. A stage template citing a slotless file under a kit's `templates/` is already green in the tree: `align.md` cites `delegation-kit/templates/agent-execution.md`, and `templates/frames/stage-contract.md` carries no slot.
- **`LIFECYCLE_KIT_STAGE_EXECUTOR`** — producer: a consumer's knob file or its overlay; no deployed configuration here sets it, by delta 4's ruling, and the gate's fixture pair and the validator's cases exercise it. Consumers: the table validator (shape), `check-stage-skill-coverage` (delta 5), the lead through `--emit knob-values` (delta 6). Each field has a reader: the stage keys the lead's lookup and the gate's surface resolution, and the adapter is the word the lead puts in the prompt and `--foreign-run` takes.
- **Roster-holding readers of a minted knob name** — the crate's knob table and its rendered roster, the kit's knob template, the kit SPEC's knob list, and the release declarations, each an update target below.
- **A narrowed corpus** — none. Delta 2 widens the set of templates carrying the protocol and delta 5 adds a red; neither prunes a reader's corpus.
- **Every member's value** — delta 2 names each of the six templates'.

## Existing sections updated

Produced by `git grep -n -l 'host protocol\|foreign adapter' -- ':!docs/' ':!TASK-QUEUE.md'` and `ls lifecycle-kit/templates/stages`.

- `lifecycle-kit/SPEC.md` §templates/stages/ — the two host paragraphs leave for the new section (delta 3); a new §The host protocol (deltas 1, 2 and 3); §The stage-contract arm — its last bullet and its closing *An arm, where a caller could concatenate* sentence, which names the binding as built (deltas 3 and 4); §templates/lead.md — the foreign-dispatch paragraph (delta 6); §check-stage-skill-coverage — the fourth direction, the direction count and the declared knobs (delta 5); §Layout and configuration — the knob bullet and the table validator's sentence (delta 4).
- `lifecycle-kit/templates/host-protocol.md` — new (delta 1).
- `lifecycle-kit/templates/stages/align.md`, `scope.md`, `spec.md`, `validate.md`, `close.md` (delta 2).
- `lifecycle-kit/templates/lead.md` (delta 6); `lifecycle-kit/templates/lifecycle-config.knobs` and `lifecycle-kit/README.md` where it lists knobs or the foreign reading (delta 4).
- `delegation-kit/SPEC.md` §The foreign-vendor run — the opening paragraph's "a stage's contract under lifecycle-kit's host protocol" citation repointed at the new section (delta 3), and the adapter-pick sentence under **What returns** (delta 6); §The tier binding — its closing sentence on the mechanical class gains nothing, and is listed as read and unchanged.
<!-- update-target-exempt: listed because the grep returns it; its standing-policy sentence already covers a prompt-named adapter and no delta changes it -->
- `.claude/agents/stage-session.md` — read and unchanged.
- `native/src/gates/mod.rs` — `check-stage-skill-coverage`'s declared-knob roster gains `LIFECYCLE_KIT_STAGE_EXECUTOR` (delta 5).
- `native/src/knobs/lifecycle_kit.rs` — the row and the validator's refusals (delta 4); `native/src/gates/stage_skill_coverage.rs` and `lifecycle-kit/gate-tests/check-stage-skill-coverage/` — the direction and a fixture pair extended with a bound stage whose surface carries the citation and one whose surface does not (delta 5).
- The crate's cases — the validator's five refusals (delta 4), and `--emit stage-contract` composing a document for each admitted stage handed to an audit-mode run under the stub adapter (delta 2).
- `.workflow/surface-ceiling.txt` — the rows of the four templates that gain the line and of the lead template re-stamped with `--emit always-loaded --ceiling` in the commit that grows each, and a row for the new protocol file where the ratchet governs it; the audit template shrinks and needs none (deltas 1, 2 and 6).
- `.workflow/release-declarations.md` — a row for the knob, the gate's new red and the template set (deltas 2, 4 and 5).
- `docs/lifecycle-kit/`, `docs/delegation-kit/` and every other generated projection of the surfaces above — regenerated, never hand-edited (all deltas).
- `TASK-QUEUE.md` — the cross-vendor delegation entry's remaining-work list and landed-slices paragraph, rewritten inside the entry cap in the demoting commit (deltas 1, 2 and 3).

## Retired spellings

- None — no delta retires a name; the audit template's block is replaced by a pointer and two SPEC paragraphs are re-homed under a new heading.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **No foreign run is owed by this amendment.** Acceptance is the crate's stub-driven cases and one read-only master-harness child handed a non-audit stage's composed document. A foreign agent executing a non-audit contract is unobserved and is filed as a gap rather than run without a grant. The one grant on the delegation entry is the audit stage's and is spent, or left, by that stage's host.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls lifecycle-kit/SPEC-*.md`), which the iteration's last lifecycle-kit batch discharges.
- [ ] **The delegation entry demoted, not done** — it outlives this amendment, a stage whose writes a foreign agent performs being unbuilt: `--queue demote heterogeneous-agent-delegation` in the commit merging deltas 1, 2 and 3, before the stage that drains the queue, its body brought inside the entry cap in that commit.
- [ ] **The executor-binding entry done** — `--queue done stage-executor-binding` in the commit merging deltas 4, 5 and 6, before the stage that drains the queue.
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
