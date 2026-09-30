# SPEC amendment: tier-model-binding

**The tier a dispatch rides is written as a harness model alias at every place it is chosen, and no consumer setting chooses between following the newest model and pinning one.** `grep -n '^model:' .claude/agents/*.md` returns four aliases: `opus` for `stage-session` and `consult-session`, `sonnet` for `audit-sweep` and `edit-sweep`. `.claude/commands/lead.md` spells `model: opus` and three `model: sonnet` overrides in its ruling-config binding. `.claude/commands/economics.md` restates them. The kit side names no model, as the provenance seam requires: delegation-kit/templates/agent-execution.md tells a dispatcher to derive the class ladder from the live roster at dispatch time, and lifecycle-kit/templates/lead.md maps "class → live model" at dispatch time by pointing there. So the mapping lives in each dispatcher's head, and an alias resolves to whatever model the harness currently names by it. A roster move changes every dispatch's price and behaviour with no edit anywhere.

**The fix is one consumer binding per tier class, read by every place that chooses a tier.** The binding's values are the consumer's: an alias, which follows the newest model the harness names by it, or an exact model id, which stays until the consumer changes it. Following is the default this repository sets; pinning is the opt-in; an empty binding is off and leaves today's behaviour (Policy-as-choice).

**Measured at authoring.**

- The dispatch tool's per-dispatch `model` parameter is an **alias enum** on this harness: the authoring session's tool schema lists `sonnet`, `opus`, `haiku` and `fable` and nothing else. So an exact model id cannot ride a per-dispatch override. That refutes the entry's inferred premise that the harness takes an exact id wherever it takes an alias, and it shapes delta 4 and the pinning limit below.
- The agent-definition `model:` frontmatter takes an alias, an exact id or `inherit`, and the harness ignores a frontmatter key it does not recognize without reporting an error. Both are from the harness's sub-agent documentation, fetched at authoring; the second is quoted directly and the first came through a summarizer.
- A transcript's assistant records carry the model id each turn actually ran on (`message.model`, for example `claude-opus-5-5` in the authoring session's own transcript). This is the reader session-model-identity-verification builds on, and it is how a bypassed pin shows.

**Inferred, cannot run before build:** that the harness honours an exact id in a definition's `model:` field — the harness loads definitions at session start, so the probe is a dispatch from a session started after delta 6 lands, under a pinned value.

**The class vocabulary is the kit's; the models are the consumer's.** Three classes, ordered from highest to lowest:

- **judgment** — the tier trusted with design decisions: a design-bearing delta, scope, spec, close, a consultation.
- **routing** — the split-posture lead's own turns: dispatch, result ingestion and budget verdicts (lifecycle-kit/templates/lead.md §The lead model).
- **mechanical** — oracle-running, a read-heavy audit and a rename or merge sweep.

These are the tiers the protocol already names in prose. The spec stage's work-class labels map onto them: `design-bearing` to judgment and `mechanical` to mechanical. The order matters only to a floor check (session-model-identity-verification's), which reads "at or above".

**Two alternatives were weighed and refused.**

- **Three scalar knobs, one per class.** A scalar takes an environment override under its own name (gate-sdk/SPEC.md §The knob file). The binding feeds a freshness gate (delta 3), so a session exporting an override would red that gate against a tree nothing changed. An indexed knob takes no environment override.
- **Keeping `model:` hand-written and checking only that its value is some bound value.** Two classes bound to one alias make that check blind. Rebinding one of them then leaves every definition that meant the other class silently wrong. A definition has to say which class it is.

## What changes

### (1) `DELEGATION_KIT_TIER_MODEL` is the tier binding

delegation-kit gains the indexed knob `DELEGATION_KIT_TIER_MODEL`, each element `<class>=<model>`, default empty {design-bearing}. **Not yet applied.**

- **The row.** It joins `native/src/knobs/delegation_kit.rs`'s table. The table validator refuses, at exit 2 with every finding, an element with no `=`, a class outside `judgment`, `routing` and `mechanical`, a class bound twice, and an empty value, a value carrying whitespace or the value `inherit`. An `inherit` is a dispatcher's tier, not a model, and a definition states it directly (delta 2). It also refuses the two values that would match across every family: an all-digit value, which can only be a version token, and a value equal to the leading token of another bound value.
- **The matcher.** One crate function answers whether a running model id matches a bound value, for delta 4 and for session-model-identity-verification. It matches when the value equals the id, or equals one of the id's `-`-separated tokens after the first. So an alias matches every id the harness names by it, and an exact id matches only itself. The kit holds no alias list, because the harness's ids already carry their family as a token.
- **Why the leading token never matches.** The table validator sees the binding and no id, so it cannot tell a namespace spelling from an alias. The skip makes a namespace value match no id, which D6 blocks and the model verdict reads as `UNBOUND`, rather than match every id silently. The section below carries the grounds.
- **The section.** delegation-kit/SPEC.md gains `## The tier binding` before `## check-agent-tier-explicit`:

> ## The tier binding
>
> A consumer binds each **tier class** to a model once, and every place that chooses a tier reads the binding. The classes are kit vocabulary, highest first: **judgment** (design decisions), **routing** (a split-posture lead's own turns, lifecycle-kit/templates/lead.md §The lead model) and **mechanical** (oracle-running, read-heavy audits, rename and merge sweeps). A spec stage's `design-bearing` work class maps to judgment and its `mechanical` class to mechanical. `DELEGATION_KIT_TIER_MODEL` holds the binding, one `<class>=<model>` element per class, read by `--emit knob-values DELEGATION_KIT_TIER_MODEL`. An unbound class is one no element names. An empty binding is off: nothing below fires, and tiers stay each dispatcher's to name.
>
> **A value is an alias or an exact model id, and the choice is the consumer's.** An alias follows: the harness resolves it to the newest model it names by it, so a model upgrade arrives with no edit, and so do its price and its behaviour. An exact id pins: it stays until the consumer changes it. The kit ships no value, because the harness roster churns and the values are consumer config (gate-sdk/SPEC.md §The provenance seam). A running model **matches** a value when the value equals its id or equals one of the id's `-`-separated tokens after the first, so an alias matches every id in its family and an exact id matches only itself. The leading token is skipped because it is the namespace every id shares: a value equal to it would match every model and hold no tier. A binding that is all aliases does not show which spelling is the namespace, so the skip makes such a value match nothing, and every reader reports it. The table validator refuses the two spellings it can see: an all-digit value, a version token that crosses families, and a value equal to another bound value's leading token. **Honest limit:** a harness whose ids lead with their family reaches it by exact id only.
>
> **The binding is read at three places.** A definition's `model:` is generated from the class its `tier:` field declares (§check-agent-tier-explicit). A dispatch naming a model is held to a bound value (§The delegation model, D6). A session reads the class of the model it runs on (§model-verdict). **Honest limit on pinning:** this harness takes a per-dispatch `model` as an alias only, so a pinned class cannot ride a per-dispatch override. A consumer pinning a class it reaches by override dispatches it as a type whose definition carries the pin instead. Pinning an alias's own resolution is harness configuration (its default-model environment settings), outside this kit, on the reasoning §The delegation model applies to `worktree.baseRef`.
>
> **Following is a strength with a stated cost.** An upgrade under an alias changes price and behaviour silently. What surfaces it is a new id in the transcripts: drift-kit's `--price-coverage` names an id its price table cannot price (drift-kit/SPEC.md §The price-coverage arm), and the lead reads it before its first dispatch. Under a pin, a new id in a pinned class's transcripts means the pin was bypassed, and `--model-verdict` reports it as a mismatch.

The §Layout and configuration bullet, after `DELEGATION_KIT_REQUIRE_TIER`'s:

> - `DELEGATION_KIT_TIER_MODEL` — the tier binding (§The tier binding): indexed, one `<class>=<model>` element per class, the class one of `judgment`, `routing` and `mechanical`; default empty, which is off. The table validator refuses a malformed element, an unknown class, a class bound twice, an `inherit` value, an all-digit value and a value equal to another bound value's leading token. It is indexed so it takes no environment override: a freshness gate reads it, and a per-session override would red that gate against an unchanged tree.

`templates/delegation-config.knobs` gains a commented example block under a `spec:` line citing §The tier binding: `# DELEGATION_KIT_TIER_MODEL[] = judgment=<model>` and its two siblings. The kit names no model, so the placeholder stays a placeholder.

### (2) A definition declares its class, and its `model:` is generated

An agent definition declares its tier class in a frontmatter `tier:` field, and a new emitter arm writes its `model:` field from the binding {design-bearing}. **Not yet applied.**

- **The arm.** `bash gate-sdk/bin/run-gates.sh --emit agent-tiers [--write]` walks `DELEGATION_KIT_AGENT_DIR` as `check-agent-tier-explicit` does. For each definition whose first frontmatter block carries `tier: <class>`, the expected `model:` is that class's bound value. Bare, the arm prints one line per definition, `<path>: model: <current> -> <expected>` or `<path>: current`, and exits 0. `--write` rewrites the `model:` line inside the first frontmatter block, and inserts one after `tier:` where none exists. It touches no other byte. It writes nothing until every definition has resolved, so a definition naming an unbound class leaves the tree untouched and exits 2, naming it.
- **What it leaves alone.** A definition without `tier:` is left as it is, and so is every definition when the binding is empty, where the arm prints `binding off` and exits 0. Its declared knob roster is `DELEGATION_KIT_AGENT_DIR` and `DELEGATION_KIT_TIER_MODEL`.
- **Its registration.** It is an `--emit-` family member, since its exit grammar is the collapse, and it joins the arm table in `native/src/emit/mod.rs` and the fence-safe set by derivation. It writes only inside the tree and spawns nothing.

The harness ignores `tier:` (measured above), so the key costs a definition nothing on this harness. It is kit vocabulary in a harness file, the same footing `name:` and `model:` already have as the fields the kit reads.

### (3) `check-agent-tier-explicit` holds the generated field

The gate gains assertion B, which runs only where the binding is non-empty {design-bearing}. **Not yet applied.**

- **Assertion A.** Unchanged: every definition states `model:`.
- **Assertion B.** Every definition either states `model: inherit` with no `tier:`, or carries `tier:` naming a bound class with `model:` equal to what `--emit agent-tiers` would write. It reds on a missing `tier:` beside a non-`inherit` model, on an unbound class, and on a stale model line. Each finding prints `bash gate-sdk/bin/run-gates.sh --emit agent-tiers --write` as its fix.
- **The comparison.** The gate calls the arm's library function in process and compares whole-file bytes, so the arm and the gate cannot disagree about what `--write` produces.
- **The descriptor.** It gains `# projection: knob:DELEGATION_KIT_AGENT_DIR/*.md`. Its knob file already reaches its trigger through the derived couples (gate-sdk/SPEC.md §The `# graph:` manifest), so a binding edit re-runs it at commit.
- **The clean line.** It names the binding state: `binding off` where the knob is empty, and otherwise the count of definitions holding their class's model.

§check-agent-tier-explicit is re-phrased:

> ## check-agent-tier-explicit
>
> Every agent definition under `DELEGATION_KIT_AGENT_DIR` declares a `model:` field in its frontmatter (assertion A), and, where the tier binding is set, its `model:` is the bound value of the class its `tier:` field declares (assertion B). The gate is the oracle over the tracked half of the template's **Match the dispatched model and effort to the unit's shape** rule: the per-dispatch habit leaves no artifact, but a standing choice does.
>
> **Assertion A polices silence, not the choice.** An explicit `inherit` passes, since a type that should ride its dispatcher's tier is a legitimate answer. What reds is omission, the one state that looks like a neutral absence and is not one, because an omitted `model:` is the literal `inherit`.
>
> **Assertion B holds a generated copy.** The harness reads `model:` and cannot read the binding, so the field is a projection of the binding through the definition's class. `--emit agent-tiers --write` generates it, and B byte-compares each definition against that emitter. A definition stating `model: inherit` with no `tier:` passes B, on A's ground. B is off while the binding is empty.
>
> **Counted inertness.** A consumer with no such directory, or one holding no definitions, scans zero and reports a clean counted line. The scan set is the directory's contents.
>
> **Honest limit.** It holds the tracked surface only. A dispatch naming no agent type and no `model` inherits and leaves no artifact, which D5 reaches at the dispatch where it is switched on (§The delegation model).

The fixture pair grows:

- `good/` gains a case knob file (`scripts/delegation-config.knobs`, binding two classes) and a definition carrying `tier:` with its bound model.
- `bad/` gains a stale-model definition under the same binding, and one naming an unbound class.
- The existing omission case and the `inherit` case stay, so A's pair is unchanged.

`docs/site-architecture.md` §Generated projections and their freshness gates gains a roster row:

> **The agent-definition tiers** <!-- projection: check-agent-tier-explicit --> — each `.claude/agents/*.md` `model:` line is generated from its `tier:` and `DELEGATION_KIT_TIER_MODEL`: `bash gate-sdk/bin/run-gates.sh --emit agent-tiers --write` (`check-agent-tier-explicit` assertion B byte-gates it).

### (4) agent-dispatch-guard D6 holds a named model to the binding

The dispatch guard gains rule D6, **bound tier**: where the binding is non-empty and `tool_input.model` is present, a model that matches no bound value blocks {design-bearing}. **Not yet applied.**

- **The table.** A row after D5: `| D6 | bound tier | DELEGATION_KIT_TIER_MODEL is non-empty and tool_input.model matches no bound value | block |`.
- **The order.** D5 precedes D6. Their triggers are disjoint, an absent model against a named one, so the order sets only which message a reader meets first.
- **The paragraph.** It joins §The delegation model after **D5 makes the tier a choice**:

> **D6 holds a named tier to the binding.** Where the consumer binds its classes, a dispatch naming a `model` names one of the bound values, so a per-dispatch override chooses among the consumer's tiers rather than around them. Matching is §The tier binding's. The block message names the bound values, which are the consumer's config and not kit literals, and points at `--emit knob-values DELEGATION_KIT_TIER_MODEL`. A dispatch naming no model is D5's. An empty binding leaves D6 inert. Under a pinned class, the alias a per-dispatch parameter can carry matches no pinned id and blocks. That is the pin held rather than bypassed at the one chokepoint that sees it, and the consumer dispatches that class through its pinned type (§The tier binding, the honest limit on pinning).

- **The degradation table.** It gains a `DELEGATION_KIT_TIER_MODEL` row: resolves empty, D6 inert and silently so; fails its own read, an advisory naming D6 unenforced and the other rules unaffected. The unparseable-payload row and the validator-fault sentence name D6 beside D2, D4 and D5.
- **The cases.** `usage-tests/dispatch-guard-cases.tsv` gains a `bound:` type sentinel, which runs its row with the binding `judgment=big-model` and `mechanical=small`. It carries a firing and a non-firing case: a named model outside the binding blocks, and a named `small` falls through, as does a model id carrying `small` as a token after its first. The `armed:` rows run with the binding empty, so D6 stays inert there, and §Testing's roster of rules gains the D6 line.

### (5) The protocol templates read the binding

The dispatcher-facing templates stop deriving the ladder from memory and read the binding where one is set {mechanical}. **Not yet applied.**

- **delegation-kit/templates/agent-execution.md**, the **Match the dispatched model and effort to the unit's shape** bullet. The sentence "Derive the class ladder from the harness's **live model roster at dispatch time** — model families churn faster than kit text, so a baked model-name list in any doc is drift by construction." becomes: "Read the class ladder from the consumer's tier binding (`--emit knob-values DELEGATION_KIT_TIER_MODEL`, delegation-kit/SPEC.md §The tier binding) — judgment, routing, mechanical — and, where none is bound, from the harness's **live model roster at dispatch time**: a model-name list baked into any doc is drift by construction." The rest of the bullet stands.
- **lifecycle-kit/templates/lead.md** §Economics, *Tier each batch to its work class*. "Class → live model is mapped at dispatch time (agent-execution.md, same bullet)." becomes: "The class's model is the consumer's tier binding where it sets one, read at dispatch time (delegation-kit's `DELEGATION_KIT_TIER_MODEL`; agent-execution.md, same bullet)." The same bullet's "pins the cheaper tier with a `model` override on that batch's dispatch" becomes "dispatches that batch on the mechanical class: a `model` override naming the class's bound alias, or, where the class is pinned to an exact id, a type whose definition declares that class (delegation-kit/SPEC.md §The tier binding, the pinning limit)".
- **lifecycle-kit/templates/consult.md** step 7. "Name the tier in the dispatch, downward by default" becomes "Name the tier in the dispatch — the bound model of the class the work needs, where the consumer binds one — downward by default". consult-tier-declaration adds an unnumbered **First step** paragraph above the numbered steps. The two edits touch different passages, so whichever lands second applies its own delta to the text as the first left it.

### (6) This repository binds its classes and re-sources every choice

This repository sets its binding, declares each definition's class, and rewrites its tier prose to name classes rather than models {mechanical}. **Not yet applied.**

- **`scripts/delegation-config.knobs`** gains `DELEGATION_KIT_TIER_MODEL[] = judgment=opus`, `routing=opus` and `mechanical=sonnet`. `routing` binds the judgment model because the lead binding records that this repository's split is by session and not by tier, with the lead on the judgment model.
- **The four definitions** gain `tier:`: `stage-session` and `consult-session` judgment, `audit-sweep` and `edit-sweep` mechanical. `--emit agent-tiers --write` then leaves each `model:` as it is, which is the proof that the binding reproduces today's choice. hotfix-agent-definition adds a fifth definition. If it lands in the same batch as this delta or after it, its definition carries `tier: judgment` and a generated `model:`. If it lands before this delta, this delta's roster is re-derived by listing `.claude/agents/` at build and reaches that definition too.
- **The `## Tier` sections** of `audit-sweep.md` and `edit-sweep.md` become one sentence each: "Your `tier:` is `mechanical`, and your `model:` is generated from the repository's tier binding (delegation-kit/SPEC.md §The tier binding); an omitted field would be the literal `inherit`, silently buying the dispatcher's tier."
- **`.claude/commands/lead.md` ruling-config.**
  - "Lead and every stage — scope included — ride Opus via the agent's `model: opus` frontmatter default" becomes "Lead and every stage — scope included — ride the judgment class, the stage agent's generated `model:` default".
  - "Three stages depart from the Opus default" becomes "Three stages depart from the judgment-class default".
  - The three overrides, `validate`, `build`'s mechanical batches and `align`, become "the mechanical class's model, read from `--emit knob-values DELEGATION_KIT_TIER_MODEL` at dispatch". `build`'s "on the Opus judgment default" becomes "on the judgment-class default".
  - "scope, `spec`, and close stay on Opus" becomes "stay on the judgment class".
  - "Re-judge every tier when the harness model roster churns" becomes "Re-judge the binding when the harness model roster churns".
- **`.claude/commands/economics.md` posture.** The sentence beginning "As of this writing that binding puts the lead and the stages on Opus" becomes "The classes each stage rides are that binding's, and the models are `DELEGATION_KIT_TIER_MODEL`'s — so a build row may be either class, and which one is a fact about that iteration's batches rather than about the stage." That removes the model names the binding now owns.

### (7) Public documentation states alias tiering with its limit

The orchestration page and the delegation-kit README state the binding, the follow-or-pin choice and its honest limit {design-bearing}. **Not yet applied.**

- **`docs/orchestration.md`.** After the paragraph introducing the judgment and routing tiers, a paragraph: "Each tier is bound to a model once, in your delegation config. Bind a tier to a model alias and it follows the newest model the harness names by it: upgrades arrive with no edit, and so do their price and behaviour. The lead's first step names any model id your price table has not priced, which is how an upgrade shows. Bind a tier to an exact model id to pin it, and a new id in that tier's sessions means the pin was bypassed. The session's own model check reports it. The binding and its limits are [`delegation-kit/SPEC.md §The tier binding`](delegation-kit/SPEC.md#the-tier-binding)." Step 4's "on the tier your agent definition pins for it" becomes "on the tier your binding assigns its class".
- **`delegation-kit/README.md`.** The install steps gain an optional step after the budget-guard step: bind `DELEGATION_KIT_TIER_MODEL`, declare each agent definition's `tier:`, and run `--emit agent-tiers --write`.

### (8) The binding's crate tests

Crate tests hold the validator, the matcher and the arm, each with a firing and a non-firing case {design-bearing}. **Not yet applied.**

- **The validator**, in a test module `native/src/knobs/delegation_kit.rs` gains. One row per refusal in delta 1: no `=`, an unknown class, a class bound twice, an empty value, a value carrying whitespace, `inherit`, an all-digit value, and a value equal to another bound value's leading token. A binding refusing on several findings reports every one. A well-formed binding of three aliases and one of an alias beside an exact id pass.
- **The matcher**, in its own module's tests. An exact id matches only itself. An alias matches an id carrying it as a token after the first and misses an id without it. A value equal to an id's leading token matches no id.
- **The arm**, in `native/src/emit/agent_tiers.rs`'s test module, which gate-sdk/SPEC.md §The non-gate arm makes it owe. The bare report's two line shapes. `--write` rewriting a stale `model:`, inserting a missing one after `tier:`, and leaving every other byte alone. An unbound class exiting 2 with the tree untouched. A definition without `tier:` left alone. An empty binding printing `binding off`.

## Producers and consumers

- **`DELEGATION_KIT_TIER_MODEL`.** Producer: the consumer's knob file, which this repository sets in delta 6. Consumers:
  - `--emit agent-tiers` (delta 2), `check-agent-tier-explicit` assertion B (delta 3), D6 (delta 4), and a dispatcher reading `--emit knob-values` (delta 5);
  - `--model-verdict`, which session-model-identity-verification adds;
  - the roster-holding readers of a knob name: `native/src/knobs/delegation_kit.rs`'s table, `--emit knob-roster`, `check-knob-citation`, `check-knob-default-coupling` (the bullet's "default empty"), and the template's example line.
- **The class vocabulary.** Producer: §The tier binding. Consumers: the table validator, the `tier:` field, the lead template's mapping of work classes, and the model verdict's floor order.
- **`tier:`.** Producer: each definition's author. Consumers: `--emit agent-tiers` and assertion B. The harness ignores it.
- **`--emit agent-tiers`.** Producer: a session changing the binding or a `tier:` field, prompted by assertion B's fix line. Consumers: the definitions it writes, and the roster-holding readers of an arm name: the arm table and the fence-safe derivation in `native/src/emit/mod.rs`, and the projection roster that `check-projection-roster` holds.
- **D6.** Producer: the harness firing the dispatch guard on every `Agent` call, wired in this repository's `.claude/settings.json`. Consumer: the dispatching session, which reads the block message.

## Existing sections updated

- `delegation-kit/SPEC.md`: §The tier binding, new, and §Layout and configuration's bullet (delta 1); §check-agent-tier-explicit (deltas 2 and 3); §The delegation model, the D-table, the D6 paragraph and the degradation table, and §Testing's dispatch-guard roster (delta 4).
- `native/src/knobs/delegation_kit.rs`: the row and its validator (delta 1), and its test module (delta 8).
- `native/src/emit/mod.rs` and a new `native/src/emit/agent_tiers.rs`: the arm (delta 2), and its tests (delta 8).
- The matcher's module: the function (delta 1), and its tests (delta 8).
- `native/src/gates/agent_tier_explicit.rs`: assertion B (delta 3).
- `delegation-kit/checks/check-agent-tier-explicit.gate`: the projection line (delta 3).
- `delegation-kit/gate-tests/check-agent-tier-explicit/`: the grown pair (delta 3).
- `native/src/hook/` (the dispatch-guard member) and `delegation-kit/usage-tests/dispatch-guard-cases.tsv`: D6 and its cases (delta 4).
- `delegation-kit/templates/delegation-config.knobs`: the example (delta 1).
- `delegation-kit/templates/agent-execution.md` (delta 5).
- `lifecycle-kit/templates/lead.md` and `lifecycle-kit/templates/consult.md` (delta 5).
- `scripts/delegation-config.knobs`, `.claude/agents/*.md`, `.claude/commands/lead.md` and `.claude/commands/economics.md` (delta 6).
- `docs/site-architecture.md`: the projection row (delta 3).
- `docs/orchestration.md` and `delegation-kit/README.md` (delta 7).
- `docs/delegation-kit/SPEC.md` and `docs/delegation-kit/README.md`: the generated mirror (deltas 1, 2, 3, 4 and 7).
- `.workflow/surface-ceiling.txt` — the grown `delegation-kit/SPEC.md`, `delegation-kit/templates/agent-execution.md`, `lifecycle-kit/templates/lead.md`, `lifecycle-kit/templates/consult.md`, `docs/site-architecture.md` and `docs/orchestration.md` rows, and any agent-definition or command row that grows, re-stamped with `bash gate-sdk/bin/run-gates.sh --emit always-loaded --ceiling`, which `check-surface-ratchet` demands with the growth (deltas 1, 3, 4, 5, 6 and 7).

The roster came from `git grep -n -i -E "model roster|live roster|model: (opus|sonnet|inherit)|model override|cheaper tier"` over the tracked tree outside `docs/` and the queue, for the sites that choose a tier. `git grep -n 'DELEGATION_KIT_REQUIRE_TIER'` found the readers a delegation-kit knob reaches, and `git grep -n 'price-coverage'` the registration sites of an arm.

## Retired spellings

- None — no delta renames or removes a name; the model aliases leave the prose sites delta 6 rewrites but stay the binding's values.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for the knob, the class vocabulary, `tier:`, the arm, assertion B and D6.
- [ ] **Instruction surfaces: instruction only** — the agent definitions' `## Tier` sentences and the template edits carry no grounds.
- [ ] **Merged with no information lost** — §The tier binding reads as the one home of the classes, the follow-or-pin choice and the pinning limit, and §check-agent-tier-explicit as one gate with two assertions.
- [ ] **Amendment deleted** — this file removed on merge; `ls delegation-kit/SPEC-tier-model-binding.md` finds nothing.
- [ ] **Removals propagated** — `git grep -n -i -E 'opus|sonnet|haiku' -- .claude/commands` returns nothing.
- [ ] **The inferred premise** — a session started after delta 6 lands dispatches a definition under a temporarily pinned exact id and reads its transcript's `message.model`; the result is recorded in §The tier binding's pinning limit either way.
- [ ] **Gaps filed** — a cross-component gap found during the work filed with `--emit file-gap`.
- [ ] **The entry moves** — `tier-model-binding` moves to Done in the landing commit, a stage before the drain stage.
