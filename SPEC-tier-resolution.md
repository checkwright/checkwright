# SPEC amendment: tier-resolution

A tier class resolves to a model and an effort in one place, for the master harness and for a foreign vendor's harness alike. Deltas 1 through 6 are merged: the row grammar, the class roster, the foreign table, the executor's `--class`, the master readers' effort half and the consultation's class knob are canonical text, and each heading below points at where it landed. Deltas 7, 8 and 9 remain to apply: the templates that pass a class, the canonical passages that still say the two bindings do not meet, and this repo's own binding.

## What changes

### (1) A row binds a class to a model and an optional effort

Merged: delegation-kit/SPEC.md §The tier binding, **A row binds a model and an optional effort**. {design-bearing}

### (2) The class roster is a knob

Merged: delegation-kit/SPEC.md §The tier binding, **The roster is consumer config**, and §Layout and configuration, `DELEGATION_KIT_TIER_CLASSES`. {design-bearing}

### (3) A foreign harness has a class table of its own, in the same grammar

Merged: delegation-kit/SPEC.md §The tier binding, **A foreign harness has a table of its own**, and §Layout and configuration, `DELEGATION_KIT_FOREIGN_TIER_MODEL` and `DELEGATION_KIT_FOREIGN_HARNESS`. {design-bearing}

### (4) The executor resolves a run's class into the adapter's argv

Merged: delegation-kit/SPEC.md §The foreign-vendor run, **The executor resolves a run's class into the adapter's argv**, and §Resuming a session. {design-bearing}

### (5) The master harness's readers take the pair

Merged: delegation-kit/SPEC.md §check-agent-tier-explicit, §model-verdict and §The delegation model's D6 paragraph. {design-bearing}

**Inferred, cannot run before build:** that a definition carrying `effort: <level>` dispatches its child at that level — no definition in this tree carries the line until delta 9 lands, and a definition reaches only sessions started after it lands; the run is a dispatch of a tiered type from such a session, then `--model-verdict <the child's id>`

Build landed the verdict's `effort=` field and the generated line, and the field reads a live session's own effort. The run above was not reachable from the session that built them. The canonical text states the claim as read from the harness's documentation and not measured (delegation-kit/SPEC.md §check-agent-tier-explicit, **Honest limits**), and a run that returns restates that sentence.

### (6) A consultation's class is a binding

Merged: lifecycle-kit/SPEC.md §Layout and configuration, `LIFECYCLE_KIT_CONSULT_CLASS`, with its honest limit. {design-bearing}

One ground of this delta is not yet placed, because delta 8 places it in lifecycle-kit/SPEC.md §templates/consult.md:

> This is a knob and still not a template slot. The refusal of a slot stands on its own ground, a residue every adopter's shim would have to fill; a knob with a default asks nothing of an adopter who wants the default.

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

The sentences stating that no adapter binds to a class, and that the two bindings share no reader, are rewritten to what deltas 3, 4 and 6 made true. {mechanical}

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

### (9) This repo binds four classes, each with an effort

This repo's roster becomes `expert`, `judgment`, `mechanical`, `trivial`, each bound to a model and an effort, and its consultations ride `expert`. {mechanical}

The values are the consumer's and sit in its own config, never in a kit file. The binary that reads these rows knows the roster knob, the effort half and the consult-class knob, since deltas 1, 2, 5 and 6 are built.

- `scripts/delegation-config.knobs`: `DELEGATION_KIT_TIER_CLASSES` set to the four names, highest first as written above. `DELEGATION_KIT_TIER_MODEL` rows `expert=fable,high`, `judgment=opus,high`, `mechanical=sonnet,high` and `trivial=haiku,medium`. The `routing` row goes, with the comment that explains it: this repo splits its lead by session, so it names no routing class.
- `scripts/lifecycle-config.knobs`: `LIFECYCLE_KIT_CONSULT_CLASS = expert`.
- `.claude/agents/consult-session.md`: `tier: expert`, moved in the same commit as the knob, since nothing holds the two equal (lifecycle-kit/SPEC.md §Layout and configuration, the knob's honest limit).
- Every tiered definition regenerated with `--emit agent-tiers --write`, which rewrites `model:` where the class moved and inserts each `effort:` line.

Its oracle is `check-agent-tier-explicit` green over the regenerated definitions, and `--emit knob-values` on the three knobs returning the values above. A definition reaches only sessions started after it lands, so a live lead keeps dispatching the old lines until its session ends.

## Producers and consumers

- **`--class` (delta 7).** Its producers are a stage host under the host protocol, a consultation commissioning a critique, and any dispatcher following the foreign-run bullet. Its reader, the executor, is built (delegation-kit/SPEC.md §The foreign-vendor run).
- **`LIFECYCLE_KIT_CONSULT_CLASS`'s template readers (delta 7).** The consult template's first step and the lead's consultation dispatch, each through `--emit knob-values`. The knob and its validator are built.
- **Narrowed corpus, point 5.** None is narrowed.
- **Every member's satisfying value, point 6.** The built effort line obliges each tiered definition to carry its class's effort where the class binds one. The members are the consumer's definitions declaring `tier:`, enumerated by `git grep -n '^tier:' -- .claude/agents`: here `audit-sweep` and `edit-sweep` on `mechanical`, and `consult-session`, `hotfix-session` and `stage-session` on `judgment`. Each one's satisfying value is its class's bound pair under delta 9, which `--write` produces: `sonnet` and `high` for the two sweeps, `opus` and `high` for `hotfix-session` and `stage-session`, and `fable` and `high` for `consult-session` once its `tier:` reads `expert`.

## Existing sections updated

- `delegation-kit/SPEC.md` §The tier binding — the opening paragraph's last sentence (delta 8).
- `delegation-kit/SPEC.md` §The foreign-vendor run — the opening sentence, the returns bullet and the honest limit (delta 8).
- `delegation-kit/templates/agent-execution.md` — the two passages delta 7 carries.
- `lifecycle-kit/SPEC.md` §Layout and configuration — the executor knob's sub-bullets, and the critique adapter's "It holds a name and no model" ground (delta 8).
- `lifecycle-kit/SPEC.md` §The host protocol — the host passes the reading's class, and the commit-message sentence's verdict line now carries it (delta 7).
- `lifecycle-kit/SPEC.md` §templates/lead.md — the executor read names the reading's class, and the consultation dispatch its bound class (delta 7).
- `lifecycle-kit/SPEC.md` §templates/consult.md — the floor paragraph, the critique run and the honest limit (deltas 7 and 8).
- `lifecycle-kit/templates/host-protocol.md`, `lifecycle-kit/templates/lead.md`, `lifecycle-kit/templates/consult.md` — delta 7's replacement text.
- `.workflow/release-declarations.md` — the template entries name the class a foreign reading passes and the knob a consultation reads (delta 7).
- `scripts/delegation-config.knobs`, `scripts/lifecycle-config.knobs` and `.claude/agents/` — this repo's roster, rows, consult class and regenerated definitions (delta 9).
- `docs/delegation-kit/SPEC.md` and `docs/lifecycle-kit/SPEC.md` — the generated mirrors, stale the moment either source lands (deltas 7 and 8).

Passages that use `judgment` descriptively — a judgment-tier recommendation, the judgment stage rows of this repo's lead binding — are left as they are, since `judgment` remains a class and the default.

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
