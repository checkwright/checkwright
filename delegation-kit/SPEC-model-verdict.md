# SPEC amendment: model-verdict

**A session cannot report or verify the model it runs on.** The session-context hook prints the iteration, the budget verdict and drift, and nothing prints the running model to the session itself. So every tiering rule in the tree is unverifiable from inside the session it binds. A stage session dispatched on the mechanical class cannot tell whether it got it. `/consult`, which the operator directed must run on the top model, cannot check that it does. A pin the consumer set cannot be seen holding or failing.

**The shape is the operator's, recorded on the entry: a `usage-verdict`-shaped check.** It reads a snapshot, exits 0, 1 or 2, fails soft, and reuses the session-id derivation's transcript lookup, with the tier expectation in consumer config and never in a kit literal. The consumer config is tier-model-binding's `DELEGATION_KIT_TIER_MODEL`, so this unit lands after that one's delta 1, in its batch or a later one. The verdict has nothing to compare against without it, and it is not built against a placeholder.

**Measured at authoring.**

- A transcript's assistant records carry the model id each turn ran on, at `message.model`. The authoring session's own transcript carried `claude-opus-5-5` on every such record.
- The newest record exists while a tool call runs. The tail of the transcript held an assistant record while the probing command executed, so a session asking about itself finds its own current turn.
- drift-kit's meter already reads the field (`usage_by_model` in `native/src/emit/stage_economics.rs`). It skips a record whose usage is absent or null, keeps the last record per message id, and folds the rest into per-model sums, so it names no newest record. The all-zero test that excludes the harness's synthetic records is `--price-coverage`'s, applied to an id's summed counts (`used_ids` in `native/src/emit/price_coverage.rs`).
- The top-level interface already renders the model through this kit's `--statusline` arm, which reads the harness's statusline payload. `SessionStart`'s payload carries a `model` field only optionally, and no other hook event carries one (harness hook documentation, fetched at authoring). So the running model reaches a session only through its transcript, and a hook-side route would reach top-level sessions alone, where the statusline already shows it. context-kit's hook is therefore not changed.

**Two alternatives were weighed and refused.**

- **Stamping the model into `--enter-stage`'s record.** That would change the stamp grammar of lifecycle-kit's state machine for a fact a stage session can ask for at the moment it needs it. It would also still leave `/consult`, which stamps nothing, uncovered.
- **Refusing the delegated case, as the overhead meter does.** The meter must never measure a sibling's transcript, because its output is a logged cost. A verdict prints the transcript key it read. Its caller can check that key against its own stamp id, or pass its id as the operand, which the stage-session route below does. So a verdict can take the newest transcript where a meter cannot.

## What changes

### (1) `--model-verdict` reads the running model and judges it against the binding

delegation-kit gains the compiled arm `bash gate-sdk/bin/run-gates.sh --model-verdict [--expect <class>] [<transcript.jsonl | session8>]` {design-bearing}. **Not yet applied.** delegation-kit/SPEC.md gains `## model-verdict` after §The usage.txt contract's parent section, before `## Trend reporter`:

> ## model-verdict
>
> Emits one verdict line naming the model the session runs on and, with `--expect <class>`, whether that model meets the class (§The tier binding).
>
> **Which transcript.** A `.jsonl` operand is read as given. An eight-character operand names a transcript through the shared inverse lookup, whose normalization strips a leading `agent-` from each candidate (drift-kit/SPEC.md §The stage-economics meter). Bare, the arm takes the delegation-aware derivation's pick (lifecycle-kit/SPEC.md §bin/session-id.sh): a top-level session's own transcript, and for a dispatched session the newest transcript under its lead's `subagents/`. The sessions dir is `DELEGATION_KIT_SESSIONS_DIR`, empty meaning derived, as drift-kit's knob is. **Honest limit:** a sibling session writing at the same moment can out-date a dispatched caller's own transcript, so the line prints the key it read, and a caller holding its own id passes it.
>
> **Which model.** The newest assistant record whose usage is not all zero gives `message.model`. The record is read by the meter's own per-record reader, and all-zero is the test `--price-coverage` applies to an id, here applied to one record, since the harness's synthetic records carry zero usage. A model switched mid-session is read as the model now running.
>
> **The verdict.** The id's classes are the bound classes whose value it matches (§The tier binding). Without `--expect`, the line is a reading and exits 0. With it, the id meets class `E` when it matches `E` or a class ranked above it. The ranking is judgment, then routing, then mechanical, so a check at a floor passes a stronger model.
>
> | verdict | when | exit |
> | --- | --- | --- |
> | `READ` | no `--expect`, and a model id was read | 0 |
> | `OK` | the id meets the expected class | 0 |
> | `BELOW` | the id matches only classes ranked under the expected one | 1 |
> | `UNBOUND` | the id matches no bound value | 1 |
> | `UNKNOWN` | no transcript, no counting record, the binding empty under `--expect`, or the expected class unbound | 2 |
>
> `UNBOUND` is a mismatch rather than an unknown, since the model is known and the binding does not name it. Under a pinned class it is the pin bypassed. Under aliases it is a model outside the consumer's tiers, a forced subagent model for one.
>
> **The line** is `model-verdict: id=<id> session=<key> class=<classes|none> expect=<class|-> -> <VERDICT>`, followed by its consequence, on §usage-verdict's verdict-string contract: reading, epistemic status, consequence.
>
> - `OK` and `READ` carry no consequence clause.
> - `BELOW` and `UNBOUND` carry `— the session is not on its expected tier; stop before work that needs it and have it re-dispatched at that tier`.
> - `UNKNOWN` carries `— tier unverified, not refused; the caller's rule says whether to proceed`.
>
> **Exit 2 is fail-soft on refusing work**, as `usage-verdict`'s STALE is. A binding or a transcript that cannot be read is no evidence of the wrong tier, so no caller refuses on it alone.
>
> **The arm** is an `Arm::Run` row: its 1 is the signal its callers grade, so it cannot be an `--emit-` member (§usage-verdict states the same forcing). It spawns nothing and writes nothing. Its declared knobs are `DELEGATION_KIT_TIER_MODEL` and `DELEGATION_KIT_SESSIONS_DIR`, and the harness's session variables arrive as inputs, as they do for the meter. A dash-led operand naming no option is a refusal at exit 2, and `--` ends option processing.
>
> **Callers.** The consult skill at its entry (lifecycle-kit/templates/consult.md), a stage session whose dispatch names its class, and any session asked what it runs on.

The §Layout and configuration bullet, after `DELEGATION_KIT_TIER_MODEL`'s:

> - `DELEGATION_KIT_SESSIONS_DIR` — the transcript directory `--model-verdict` reads (§model-verdict); default empty, which derives `<config-home>/projects/<cwd-slug>` through the crate's one derivation (lifecycle-kit/SPEC.md §bin/session-id.sh), the same emptiness drift-kit's `DRIFT_KIT_SESSIONS_DIR` takes. A kit resolves its own knob and hands the answer to the shared module. So this knob, drift-kit's and lifecycle-kit's environment-only `LIFECYCLE_KIT_SESSIONS_DIR` are three names for one default.

Registration:

- the knob row joins `native/src/knobs/delegation_kit.rs`;
- the arm row joins `native/src/emit/mod.rs`'s table, and its usage line joins `native/src/runner.rs`'s help;
- the per-record reader is factored out of `usage_by_model`, which then calls it, and the arm calls the same function, so the two agree on what a record's model and usage are. The arm adds only the all-zero test;
- the matcher is tier-model-binding delta 1's.

### (2) The verdict's crate tests cover every row

A crate test module drives the arm's library function over transcripts it writes, with a firing and a non-firing case per verdict row {design-bearing}. **Not yet applied.**

- `READ`: a bare read of an unbound id.
- `OK`: an exact-id match, an alias-token match, and a stronger class under a floor.
- `BELOW`: a mechanical-only match against `--expect judgment`.
- `UNBOUND`: an id matching nothing.
- `UNKNOWN`: an empty transcript; a transcript whose only assistant record has all-zero usage; the binding empty under `--expect`; an unbound expected class.
- The operand forms: a path, and an eight-character key naming an `agent-` transcript.
- The newest-record rule: a transcript switching model mid-file reads the later model.

The binding is written into each case's sandbox knob file under the hermetic contract §Testing states for the budget verdict, with the `DELEGATION_KIT_*` namespace stripped. §Testing gains a paragraph naming the module and these rows.

### (3) This repository's stage sessions verify the class they were dispatched at

The lead names the class in every stage dispatch prompt, and a stage session checks it after its stamp {mechanical}. **Not yet applied.**

- **`.claude/commands/lead.md`**, the ruling-config binding. After the tiering bullets, a sentence: "Every stage dispatch prompt names the class it rides (`tier: <class>`), since the class varies by batch."
- **`.claude/agents/stage-session.md`** §Standing dispatch policy gains a bullet:

> - **Verify your tier.** When your dispatch names a class, run `bash gate-sdk/bin/run-gates.sh --model-verdict --expect <class> <your stamp id>` right after your entry stamp. On exit 1 escalate to the lead before any other work: a session on the wrong tier is re-dispatched, never resumed (lifecycle-kit/templates/lead.md §Economics, *Tier each batch*). On exit 2 record the line in your journal and proceed (delegation-kit/SPEC.md §model-verdict).

## Producers and consumers

- **The verdict line and exit status.** Producer: the arm, run by its callers.
  - The consult skill's entry reads the status: exit 1 stops, exit 2 proceeds with the line stated. consult-tier-declaration adds it.
  - A stage session reads it after its stamp: exit 1 escalates to the lead, exit 2 is journalled (delta 3).
  - A person or session asking reads the line.
- **The line's fields.**
  - `id` is what a caller escalates with.
  - `session` is what a caller checks against its own id.
  - `class` and `expect` are what the escalation's Evidence cites.
  - The verdict token and the consequence are what every caller routes on.
- **`DELEGATION_KIT_SESSIONS_DIR`.** Producer: the consumer's knob file, where a consumer's harness home is non-standard. This repository leaves it at the derived default. Consumers: the arm, and the roster-holding readers of a knob name: `native/src/knobs/delegation_kit.rs`'s table, `--emit knob-roster`, `check-knob-citation` and `check-knob-default-coupling`.
- **The `tier: <class>` prompt line.** Producer: the lead, per this repository's binding (delta 3). Consumer: the dispatched stage session, at its first step after the stamp.
- **The arm name.** Its roster-holding readers are the arm table in `native/src/emit/mod.rs` and the help text in `native/src/runner.rs`. The fence-safe derivation there admits the `--emit-` family alone, so it does not reach this `Arm::Run` row.

## Existing sections updated

- `delegation-kit/SPEC.md`: §model-verdict, new, and §Layout and configuration's bullet (delta 1); §Testing (delta 2).
- `native/src/knobs/delegation_kit.rs`, `native/src/emit/mod.rs`, `native/src/runner.rs` and a new verdict module under `native/src/hook/`, beside `verdict.rs` (delta 1).
- `native/src/emit/stage_economics.rs`: the per-record reader factored out of `usage_by_model` (delta 1).
- The verdict module's tests (delta 2).
- `.claude/commands/lead.md` and `.claude/agents/stage-session.md` (delta 3).
- `delegation-kit/README.md`: the verdict's one-line entry beside `--usage-verdict`'s (delta 1).
- `docs/delegation-kit/SPEC.md` and `docs/delegation-kit/README.md`: the generated mirror (deltas 1 and 2).
- `.workflow/surface-ceiling.txt` — the grown `delegation-kit/SPEC.md`, `.claude/commands/lead.md` and `.claude/agents/stage-session.md` rows re-stamped with `bash gate-sdk/bin/run-gates.sh --emit always-loaded --ceiling`, which `check-surface-ratchet` demands with the growth (all deltas).

The roster came from `git grep -l 'usage-verdict'` over the tracked tree, for the sites a delegation-kit `Arm::Run` verdict is registered or cited at, and from `git grep -n 'DRIFT_KIT_SESSIONS_DIR'` for the sessions-dir knob's pattern.

## Retired spellings

- None — the amendment adds an arm and a knob and renames nothing.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for the arm, its line, its knob and the prompt line.
- [ ] **Instruction surfaces: instruction only** — the stage-session bullet carries the command, the two exits' acts and a pointer, and no grounds.
- [ ] **Merged with no information lost** — §model-verdict reads as `usage-verdict`'s sibling. The two refused alternatives land in it as its grounds, and its honest limit sits beside the delegated pick.
- [ ] **Amendment deleted** — this file removed on merge; `ls delegation-kit/SPEC-model-verdict.md` finds nothing.
- [ ] **Removals propagated** — nothing retired.
- [ ] **Gaps filed** — a cross-component gap found during the work filed with `--emit file-gap`.
- [ ] **The entry moves** — `session-model-identity-verification` moves to Done in the landing commit, a stage before the drain stage.
