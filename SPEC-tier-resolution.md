# SPEC amendment: tier-resolution

A tier class resolves to a model and an effort in one place, for the master harness and for a foreign vendor's harness alike. Today the binding (delegation-kit/SPEC.md §The tier binding) maps a class to a master-harness model alone, effort is chosen at each dispatch, and a foreign adapter carries its model and effort as literal argv words and binds to no class. A renamed model, or a class moved to another effort, is edited at every adapter and remembered by every dispatcher.

The two harnesses take one design: a class table whose rows bind a class to a model and an optional effort, one grammar, one parser, one resolver. The master harness's table is the existing knob. A foreign harness's table is a second knob in the same grammar, because of where each lives and never because the design differs: a knob named in a file is replaced whole (gate-sdk/SPEC.md §The knob file), a vendor's rows usually sit in the gitignored overlay beside the adapters, and master rows in that overlay would replace the tracked ones a freshness gate reads.

Measured for this amendment:

- Every assistant record in a master-harness transcript carries a top-level `effort` field beside `message.model`: over one lead session's subagent transcripts, each record read `high` or `medium`, the two values dispatched.
- `--model-verdict --expect judgment` over a transcript whose model matches no bound value prints `class=none … -> UNBOUND` at exit 1.
- The harness's subagent documentation gives a definition a frontmatter `effort:` field, a per-dispatch `effort` parameter that overrides it, and an environment variable that overrides both. Read from the documentation page, not from a dispatch.

## What changes

### (1) A row binds a class to a model and an optional effort

An element of `DELEGATION_KIT_TIER_MODEL` becomes `<class>=<model>[,<effort>]`, the effort being whatever its harness calls a reasoning level. {design-bearing} {user-facing: the entry's envelope direction — one resolution point, class to model plus effort}

The element still splits at its first `=`. The value splits at its first `,`: the model before it and the effort after. A value with no comma binds a model and no effort, so every existing binding stays valid and means what it meant. The model keeps every refusal it has. The effort is refused when empty, when it carries whitespace and when it carries a second comma. The kit holds no effort vocabulary and no order over efforts: the levels are a harness's own and churn with its roster, the ground on which the kit ships no model value (gate-sdk/SPEC.md §The provenance seam).

A class with no effort leaves effort where it is today, with the dispatcher. The one read a dispatcher makes is unchanged, `--emit knob-values DELEGATION_KIT_TIER_MODEL`, and now returns both halves.

**A new arm is refused.** A resolver arm printing a class's pair would be a second spelling of that read, and its one caller already holds the row.

### (2) The class roster is a knob

`DELEGATION_KIT_TIER_CLASSES` holds the classes, highest first: indexed, default `judgment`, `routing`, `mechanical`. {design-bearing} {user-facing: the entry's envelope direction — the class roster is consumer config with the kit's three as its default}

Every reader that took the compiled three takes the roster: the binding's validator, `--model-verdict`'s `--expect` check and its ranking, and both tables' unknown-class refusal. The table validator refuses a name outside `[a-z0-9-]`, a name given twice, and a binding row whose class the roster lacks. Being indexed it has no environment spelling, on the binding's own ground.

**`routing` stays, as a default member and no longer a constant.** Its reader is the split posture (lifecycle-kit/templates/lead.md §The lead model), which a consumer may or may not adopt. A consumer that splits its lead by session binds `routing` to its judgment model or drops it from its roster.

**Honest limit.** The shipped templates name `judgment` and `mechanical`. A roster without one of them leaves the template step that names it reading `UNKNOWN`, tier unverified. The validator does not refuse such a roster, because a consumer who forks the templates may rename every class.

### (3) A foreign harness has a class table of its own, in the same grammar

`DELEGATION_KIT_FOREIGN_TIER_MODEL` holds a foreign harness's rows, `<harness>/<class>=<model>[,<effort>]`, and `DELEGATION_KIT_FOREIGN_HARNESS` names each class-taking adapter's harness, `<adapter>=<harness>`. {design-bearing} {user-facing: the operator's refinement on the entry — the two harnesses are modelled alike}

A **harness** is a name in `[a-z0-9-]` for one vendor's model roster. Several adapters of one vendor differ in mode, sandbox and session handling and share a harness, so a model move is one row.

- `DELEGATION_KIT_FOREIGN_TIER_MODEL` — indexed, default empty. The value after the `=` is delta 1's, read by the same parser. The table validator refuses an element that does not split, a harness or class outside its grammar, a class the roster lacks, a harness and class bound twice, and delta 1's effort refusals. The master table's matching refusals, the all-digit value and the leading-token value, are not applied: no reader matches a running model against a foreign row. A row whose harness no adapter names is not refused, since the rows may be tracked while the adapters sit in an overlay a fresh clone lacks.
- `DELEGATION_KIT_FOREIGN_HARNESS` — indexed, default empty, at most one element per adapter. The validator refuses a malformed element, an adapter `DELEGATION_KIT_FOREIGN_ADAPTERS` does not configure, and a second element for one adapter.

Both are indexed and take no environment spelling. Neither ships a value: a harness names a vendor and a row names its model.

### (4) The executor resolves a run's class into the adapter's argv

`--foreign-run` takes `--class <class>`, and an adapter word may carry `@MODEL@` and `@EFFORT@`, substituted with the pair the adapter's harness binds to that class. {design-bearing} {user-facing: the entry's envelope direction — a foreign adapter resolves its model and effort from a class through the same read}

**An adapter takes a class when `DELEGATION_KIT_FOREIGN_HARNESS` names it.** The validator holds the two facts together: a class-taking adapter's open form carries `@MODEL@`, and an adapter no element names carries neither token in its open or its resume form. `@EFFORT@` is optional. The tokens substitute inside a word as `@PROMPT_FILE@` does, so a vendor's `key=value` option takes one.

- **On a class-taking adapter `--class` is required.** A missing one, and a class the roster lacks, are shape refusals at exit 2 with the usage. A class the harness leaves unbound, or `@EFFORT@` under a row with no effort, is `FAILED (class: <why>)` with `budget=-`, among the pre-spawn checks that precede the budget read, so nothing is cloned.
- **On any other adapter `--class` is accepted and applies nothing**, and the line says so. A host therefore always passes its reading's class and needs no second read to learn whether the adapter takes one.
- **The verdict line gains `class=<class|->` after `mode=`**, on `--foreign-run` and `--foreign-resume` alike: the class whose pair was substituted, and `-` where none was. Its reader is the dispatcher, and the commit message that records a foreign reading (lifecycle-kit/SPEC.md §The host protocol).
- **`--budget` beside `--class` is a shape refusal**, as it is beside `--mode`.

**A session keeps the pair it opened with.** `session.txt` gains `class=<class|-> model=<model|-> effort=<effort|->` after `mode=`, and a resume substitutes those recorded values into the resume form. A binding edited while a session is open reaches the next open. Re-resolving at each turn is refused, since it would change a kept conversation's model with no line saying so; refusing the resume on a changed binding is refused too, since it strands a session over an edit that need not concern it.

**What the class says and does not say.** The line's `class=` is what the executor passed. The kit parses no vendor stream, so whether the vendor ran that model at that effort stays unverified, on the ground §The foreign-vendor run gives for reading a report as bytes.

The cases are crate tests under a stub adapter that echoes its argv: both tokens substituted inside a word; a class-taking adapter without `--class`; an unbound class and an effortless row under `@EFFORT@` failing before any clone; `--class` on a plain adapter printing `class=-`; a resumed turn carrying the opened pair after the binding changed; and each validator refusal of deltas 3 and 4.

### (5) The master harness's readers take the pair

The definition generator writes a class's effort, and the tier verdict reports the effort a session runs at. {design-bearing} {user-facing: the entry's envelope direction — one resolution point, class to model plus effort}

- **`--emit agent-tiers` and `check-agent-tier-explicit` assertion B.** Where a definition's class binds an effort, the expected frontmatter carries `effort: <effort>` on the line after `model:`, generated and byte-gated as `model:` is: `--write` rewrites that line or inserts it. Where the class binds none, an `effort:` line is left as written, the footing a definition without `tier:` has. The bare arm's per-definition line reports the pair.
- **`--model-verdict`.** The line gains `effort=<effort|->` after `id=`, read from the top-level `effort` field of the record that gives the model, and `-` where the record carries none. The verdict and its exit are decided by the model alone, as today. With `--expect`, where the expected class binds an effort, an effort was read and the two differ, an `OK` line carries the consequence clause `— running effort <e> is not the class's bound <b>`. It stays `OK` at exit 0: the kit holds no order over efforts, so it cannot call a different one lower.
- **D6 is unchanged** and reads the model half of the master table alone. A dispatch's `effort` is not held to the binding: an unnamed effort is the definition's or the dispatcher's, which the guard cannot see, so a check on the named case would only teach a dispatcher to leave it unnamed.

**Honest limits.** A harness environment variable that forces every session's effort overrides a definition's line, the shape §The delegation model refuses for the subagent-model override; the verdict's clause is what surfaces it. A definition's line reaches only sessions started after it lands, as its `model:` does.

**Inferred, not run:** that a definition carrying `effort: <level>` dispatches its child at that level — from a session started after the line lands, dispatch the type and run `--model-verdict <the child's id>`, reading its `effort=` field

The cases: the fixture pair of `check-agent-tier-explicit` gains a good definition holding its class's effort and a bad one with a stale `effort:` line; crate tests for the verdict's `effort=` field, its `-`, and the clause.

### (6) A consultation's class is a binding

`LIFECYCLE_KIT_CONSULT_CLASS` names the class a consultation runs on: a scalar, default `judgment`. {design-bearing} {user-facing: the operator's two-part direction on the entry — a consultation rides a class of the consumer's choosing, which the kit lets it bind}

The consult template's first step reads it with `--emit knob-values LIFECYCLE_KIT_CONSULT_CLASS` and passes it to `--model-verdict --expect`. The lead dispatches a consultation on that class. The table validator refuses an empty value and one outside `[a-z0-9-]`, and does not read delegation-kit's roster, on the ground `LIFECYCLE_KIT_STAGE_EXECUTOR` gives for an adapter name: a kit's validator reads its own table, and a class the roster lacks is `--model-verdict`'s refusal at exit 2, which the template already reads as unverified and states.

This is a knob and still not a template slot. The refusal of a slot stands on its own ground, a residue every adopter's shim would have to fill; a knob with a default asks nothing of an adopter who wants the default.

**Honest limit.** A consumer's dispatched-consultation definition declares its own `tier:`, and nothing holds it equal to this knob. A consumer that moves one moves the other.

### (7) A foreign reading names its class

A stage host and a consultation pass the class their reading rides to `--foreign-run`. {mechanical} {user-facing: the entry's envelope direction — a foreign adapter resolves its model and effort from a class through the same read}

Replacement text for `lifecycle-kit/templates/host-protocol.md` step 3. **Not yet applied.**

> 3. Run it as an audit-mode `--foreign-run` on the named adapter with `--class <the class your dispatch names for the reading, else your own tier class>`, under delegation-kit's foreign-run bullet (delegation-kit/templates/agent-execution.md): the budget read first, backgrounded with its liveness record.

Replacement text for `lifecycle-kit/templates/lead.md`, **Read the stage's executor before you dispatch it**, its clause from "name the adapter" to "demand". **Not yet applied.**

> name the adapter in the dispatch prompt with the class the reading rides, and tier the host by what its writes demand

Replacement text for `lifecycle-kit/templates/consult.md`, the critique paragraph's run sentence. **Not yet applied.**

> Run it as an audit-mode `--foreign-run` on that adapter, or the one the operator names, with `--class <your consultation's class>`, under delegation-kit's foreign-run bullet (delegation-kit/templates/agent-execution.md): the budget read first, backgrounded with its liveness record.

Replacement text for `lifecycle-kit/templates/consult.md`, the first step's opening two sentences. **Not yet applied.**

> **First step — verify your tier.** A consultation runs on the class `--emit knob-values LIFECYCLE_KIT_CONSULT_CLASS` names. Run `--model-verdict --expect <that class>` on the gate binary `GATE_SDK_NATIVE_BIN` names (delegation-kit/SPEC.md §model-verdict; without delegation-kit, skip this).

Replacement text for `lifecycle-kit/templates/lead.md`, **Dispatch a consultation for the consult inbox only while no stage session is live**, the words "on the judgment class". **Not yet applied.**

> on the class `LIFECYCLE_KIT_CONSULT_CLASS` names

Replacement text for `delegation-kit/templates/agent-execution.md`, **Match the dispatched model and effort to the unit's shape**, the sentence that reads the class ladder. **Not yet applied.**

> Read the classes from the consumer's roster and each class's model and effort from its tier binding (`--emit knob-values DELEGATION_KIT_TIER_CLASSES` and `DELEGATION_KIT_TIER_MODEL`, delegation-kit/SPEC.md §The tier binding), name both on the dispatch where the class binds both, and, where none is bound, read the harness's **live model roster at dispatch time**: a model-name list baked into any doc is drift by construction.

And in the foreign-run bullet of the same file, after "instead of a dispatch (delegation-kit/SPEC.md §The foreign-vendor run)." **Not yet applied.**

> Name the unit's class with `--class`: an adapter that takes one runs the model and effort its harness binds to it, and the verdict line's `class=` says whether it did.

### (8) The canonical passages that said the two bindings do not meet

The sentences stating that no adapter binds to a class, and that the two bindings share no reader, are rewritten to what deltas 3, 4 and 6 make true. {mechanical}

Replacement text for delegation-kit/SPEC.md §The foreign-vendor run, the third bullet of **What returns, and how it lands**. **Not yet applied.**

> - An adapter's model and effort come from the class its run names, through the row its harness binds to that class (§The tier binding), or sit in its argv as literal words where the adapter takes no class. Either way the value is an alias or a pinned id on that section's follow-or-pin reading. The dispatcher picks an adapter by name and a class for the unit, and a stage's adapter may be bound in lifecycle-kit's executor knob (lifecycle-kit/SPEC.md §The host protocol).

Replacement text for the same section's honest limit **Nothing verifies the reader's class.** **Not yet applied.**

> - **Nothing verifies the reader's model.** The verdict line's `class=` is what the executor passed. The tier verdict reads the host's transcript, and the kit parses no vendor stream.

Replacement text for the same section's opening sentence. **Not yet applied.**

> A read-only audit or a mechanical sweep may run on another vendor's coding agent instead of a dispatch, at the tier class the run names (§The tier binding).

Replacement text for §The tier binding's opening paragraph, its last sentence, "The mechanical class may also run on another vendor's agent through a configured adapter". **Not yet applied.**

> A foreign vendor's harness binds the same classes in a table of its own, and a foreign run names the class it rides (§The foreign-vendor run).

Replacement text for lifecycle-kit/SPEC.md §Layout and configuration, the first two sub-bullets of `LIFECYCLE_KIT_STAGE_EXECUTOR`. **Not yet applied.**

>   - **The adapter is a name from `DELEGATION_KIT_FOREIGN_ADAPTERS`.** Its model and effort are delegation-kit's to resolve, from the class the reading names (delegation-kit/SPEC.md §The foreign-vendor run). So this knob holds no model and no vendor.
>   - **This kit owns it, not delegation-kit's tier binding.** A stage is this kit's vocabulary, and a consumer vendoring delegation-kit alone has no stage to bind. This knob picks the adapter and the binding gives that adapter's class its model, so the two meet at one reader, the executor a host runs.

Replacement text for lifecycle-kit/SPEC.md §templates/consult.md, the honest limit **Nothing verifies the critic's class.** **Not yet applied.**

> - **Nothing verifies the critic's model.** The run names the consultation's class and the executor passes that class's pair, which no vendor stream is parsed to confirm (delegation-kit/SPEC.md §The foreign-vendor run).

The same section's paragraph **The template declares its own floor and checks it at entry** names the floor as the class `LIFECYCLE_KIT_CONSULT_CLASS` holds, default `judgment`, and carries delta 6's knob-not-slot ground in place of "It is a class rather than a slot…" through "…the binding already offers."

## Producers and consumers

- **A row's effort (delta 1).** Producer: the consumer's knob file. Consumers: `--emit agent-tiers` and assertion B, which generate and hold a definition's `effort:` line; `--model-verdict`, which compares it under `--expect`; a dispatcher's `--emit knob-values` read; and, on a foreign row, the executor's `@EFFORT@` substitution. A class binding none has no effort reader, and none is populated.
- **`DELEGATION_KIT_TIER_CLASSES` (delta 2).** Producer: the kit's default, or the consumer's knob file. Consumers: both tables' validators, `--model-verdict`, and `--foreign-run`'s `--class` check. Roster-holding readers a minted knob lands on: delegation-kit's knob table and its validator in the crate, `--emit knob-roster`, delegation-kit/SPEC.md §Layout and configuration, `templates/delegation-config.knobs`, and each arm's declared-knob list that the crate's arm-knob test holds to what the module reads.
- **`DELEGATION_KIT_FOREIGN_TIER_MODEL` and `DELEGATION_KIT_FOREIGN_HARNESS` (delta 3).** Producer: the consumer's knob file, usually the gitignored overlay. Consumer: the executor, at `--foreign-run`'s pre-spawn checks, and the validator. The same roster-holding readers as above, and `--foreign-run`'s and `--foreign-resume`'s declared knobs.
- **`--class`, `class=`, and the session's three fields (delta 4).** Producer of `--class`: a stage host under the host protocol, a consultation commissioning a critique, and any dispatcher following the foreign-run bullet (delta 7). `class=` on the line is read by the dispatcher and copied into a host's journal and landing commit message. `session.txt`'s `class`, `model` and `effort` are each read at one transition, the resume's substitution, and `class` again for the resume line. On an adapter taking no class all three are `-` and nothing reads them.
- **`effort:` in a definition (delta 5).** Producer: `--emit agent-tiers --write`. Consumers: the harness at dispatch, and assertion B. **`effort=` on the verdict line.** Producer: the transcript's own field, written by the harness on every assistant record. Consumer: the session that ran the arm, and its caller reading the clause.
- **`LIFECYCLE_KIT_CONSULT_CLASS` (delta 6).** Producer: the default, or a consumer's knob file or environment. Consumers: the consult template's first step and the lead's consultation dispatch, each through `--emit knob-values`, and lifecycle-kit's validator. Roster-holding readers: lifecycle-kit's knob table, `--emit knob-roster`, lifecycle-kit/SPEC.md §Layout and configuration and the kit's knob-file template.
- **Narrowed corpus, point 5.** None is narrowed. The roster knob widens what a class may be, and its default is the set compiled today.
- **Every member's satisfying value, point 6.** Delta 5 obliges each tiered definition to carry its class's effort where the class binds one. The members are the consumer's definitions declaring `tier:`, enumerated by `git grep -n '^tier:' -- .claude/agents`: here `audit-sweep` and `edit-sweep` on `mechanical`, and `consult-session`, `hotfix-session` and `stage-session` on `judgment`. Each one's satisfying value is its class's bound effort, which `--write` produces; while this repo's rows bind no effort the obligation is empty for all five. The kit's two fixture trees are the other members, and delta 5 names their cases.

## Existing sections updated

- `delegation-kit/SPEC.md` §The tier binding — the element grammar, the roster knob, `routing`'s standing, the foreign table and the three-places paragraph (deltas 1, 2, 3 and 8).
- `delegation-kit/SPEC.md` §check-agent-tier-explicit — assertion B's generated copy gains the effort line and the arm's printed pair (delta 5).
- `delegation-kit/SPEC.md` §model-verdict — the line, **Which model**'s record read, the consequence-clause list and the declared knobs (deltas 2 and 5).
- `delegation-kit/SPEC.md` §The delegation model — D6's paragraph says it reads the model half alone (delta 5).
- `delegation-kit/SPEC.md` §The foreign-vendor run — the opening, the adapter paragraph's "and model", the verdict line and its exits, the cleanup step's `session.txt` grammar, the returns bullet, the honest limit and the declared knobs (deltas 4 and 8).
- `delegation-kit/SPEC.md` §Resuming a session — the resume line and the recorded pair's substitution (delta 4).
- `delegation-kit/SPEC.md` §Layout and configuration — the two tables' rows, the three new knobs, and the adapter and resume knobs' token lists (deltas 1, 2, 3 and 4).
- `delegation-kit/SPEC.md` §Testing — the cases deltas 4 and 5 name.
- `delegation-kit/README.md` — the bind-your-tiers step names the roster knob and the effort half (deltas 1 and 2).
- `delegation-kit/templates/delegation-config.knobs` — the binding's comment and example rows, and the three new knobs (deltas 1, 2 and 3).
- `delegation-kit/templates/agent-execution.md` — the two passages delta 7 carries.
- `delegation-kit/gate-tests/check-agent-tier-explicit/` — the effort cases (delta 5).
- `lifecycle-kit/SPEC.md` §Layout and configuration — the executor knob's sub-bullets, the critique adapter's "It holds a name and no model" ground, and the new knob (deltas 6 and 8).
- `lifecycle-kit/SPEC.md` §The host protocol — the host passes the reading's class, and the commit-message sentence's verdict line now carries it (delta 7).
- `lifecycle-kit/SPEC.md` §templates/lead.md — the executor read names the reading's class, and the consultation dispatch its bound class (deltas 6 and 7).
- `lifecycle-kit/SPEC.md` §templates/consult.md — the floor paragraph, the critique run and the honest limit (deltas 6, 7 and 8).
- `lifecycle-kit/templates/host-protocol.md`, `lifecycle-kit/templates/lead.md`, `lifecycle-kit/templates/consult.md` — delta 7's replacement text.
- `native/src/tier.rs` — the row parser, the roster in place of the compiled classes, the foreign table and its resolver (deltas 1, 2 and 3).
- `native/src/knobs/delegation_kit.rs` and `native/src/knobs/lifecycle_kit.rs` — the four knobs and their refusals (deltas 2, 3, 4 and 6).
- `native/src/emit/agent_tiers.rs` and `native/src/gates/agent_tier_explicit.rs` — the effort line (delta 5).
- `native/src/hook/model_verdict.rs` — the roster, the effort read and the clause (deltas 2 and 5).
- `native/src/hook/dispatch.rs` — D6 reads the model half of a row (delta 1).
- `native/src/emit/foreign_run.rs` — `--class`, the tokens, the line and the session record (delta 4).
- `native/src/runner.rs` — the usage lines and help paragraphs of `--foreign-run` and `--model-verdict` (deltas 4 and 5).
- `docs/generated-projections.md` — the agent-definition tiers row names the effort line (delta 5).
- `.workflow/release-declarations.md` — the tier-binding and foreign-run entries, both unreleased, gain the grammar, the knobs, `--class` and the line fields (deltas 1, 2, 3, 4, 5 and 6).
- `scripts/delegation-config.knobs` and `.claude/agents/` — this repo's rows and its generated definitions, changed only where this repo binds an effort or a class (deltas 1 and 5).
- `docs/delegation-kit/SPEC.md`, `docs/delegation-kit/README.md` and `docs/lifecycle-kit/SPEC.md` — the generated mirrors, stale the moment any of them lands (all deltas).

Produced by `git grep -l` over the tracked tree for `TIER_MODEL`, `model-verdict`, `FOREIGN_ADAPTERS`, `FOREIGN_RESUME` and `STAGE_EXECUTOR`, by `git grep -n 'tier::\|CLASSES' -- native/src` for the crate's readers of the class vocabulary, and by `git grep -n 'judgment\|routing' -- lifecycle-kit/templates delegation-kit/templates` read for passages that name a class as a dispatch target. Passages that use `judgment` descriptively — a judgment-tier recommendation, the judgment stage rows of this repo's lead binding — are left as they are, since `judgment` remains a class and the default.

This repo's gitignored adapter overlay is outside the tracked tree and outside this roster. Moving its adapters onto classes is the operator's own edit: a harness element per adapter, the model and effort words replaced by the two tokens, and the rows that carry today's values.

## Retired spellings

- None — no delta retires a spelling. Every knob, arm, class and line field keeps its name, and a model-only row stays valid.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the repo root (`ls SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
