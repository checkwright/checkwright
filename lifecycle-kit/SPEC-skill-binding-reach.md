# SPEC amendment: skill-binding-reach

**`check-skill-binding` couples every kit's templates by derivation, and the assertion that would hold a hand list against the shims is refused because the list goes.** The gate's descriptor names the skills dir, the stage-template dir, the four boundary templates and three out-of-tree bound templates one path at a time (`lifecycle-kit/checks/check-skill-binding.gate`). Nothing holds that list against the templates the shims bind, so a template a later shim binds fires nothing at commit until someone extends the list by hand. The entry offers an assertion over the list or a boundary note refusing it. This amendment removes the list, which outranks gating it (doctrine-kit/DOCTRINE.md, Enforcement-first), and records the refusal of the assertion with the residue it leaves.

**Read at authoring:**

- The shims' directives, by `grep -rn "Execute the template at" .claude/commands/`: twelve bound templates, every one under a kit's `templates/` directory (lifecycle-kit's stage templates, `lead.md`, `release-sweep.md` and `consult.md`; delegation-kit's `agent-execution.md`; gate-sdk's `adopt.md`; drift-kit's `economics.md`).
- `kit:templates/*.md` already reaches each of them: `check-shim-restatement` couples that token (lifecycle-kit/SPEC.md §check-shim-restatement), and `GATE_SDK_VERBOSE=1 bash gate-sdk/bin/run-gates.sh --for drift-kit/templates/economics.md` runs it. So does `--for canon-kit/templates/SPEC-amendment.md`, a kit template no shim binds.
- The crate declares one `?` dynamic root for the gate (`native/src/gates/mod.rs`, the `check-skill-binding` registry row), the skills-dir walk. The couple change moves no declared root.
- A literal kit path in a kit-shipped couple matches nothing in a tree that vendors the kits under another root, while `kit:` expands against the consumer's own kit roots (gate-sdk/SPEC.md §The `# graph:` manifest). The derived couple is therefore also right for an adopter whose kit roots differ from this tree's.

## What changes

### (1) The descriptor couples every kit's templates

The `couples=` field of `lifecycle-kit/checks/check-skill-binding.gate` becomes `knob:LIFECYCLE_KIT_SKILLS_DIR/*.md,kit:templates/*.md`, with every other manifest field unchanged. {mechanical} The eight literal template couples go: the stage-template dir glob, `lead.md`, `release-sweep.md`, `upgrade.md`, `consult.md`, `delegation-kit/templates/agent-execution.md`, `gate-sdk/templates/adopt.md` and `drift-kit/templates/economics.md`. `kit:templates/*.md` reaches every one, since the field's `*` crosses `/`.

### (2) The section states the derived couple and refuses the assertion

lifecycle-kit/SPEC.md §check-skill-binding's coupling paragraph is rewritten. {design-bearing} **Not yet applied.** The paragraph opening "The `# graph:` couples at `tier=precommit` the skills dir, the stage-template dir, each boundary skill" through "…a consumer that does bind it inherits the manifest." becomes:

> The `# graph:` couples at `tier=precommit` the skills dir and every kit's templates (`kit:templates/*.md`, whose `*` crosses `/`, gate-sdk/SPEC.md §Reading a `couples=` field's reach), so a binding changed in a shim or a slot added to any kit template fires the gate. The couple is derived, not listed: a by-name list of bound templates is a second copy of what the shims' directives say, and a template a later shim binds would fire nothing until the list caught up. A kit template no shim binds fires the gate too, an over-trigger a cheap gate pays without cost to anyone. **Refused: asserting that every bound template matches a couple.** Under the derived couple its one finding would be a template outside every kit's `templates/`, which a kit-shipped descriptor cannot name for a consumer anyway. **Honest limit:** a shim binding such a template fires this gate only in the whole-tree battery.

The paragraph's last sentence, on the fixture pair and `gate-tests/check-skill-binding.test.sh`, stays as it stands.

### (3) The generated projections follow

`docs/check-graph.html` is regenerated with `bash gate-sdk/bin/run-gates.sh --emit graph > docs/check-graph.html` in delta 1's commit. {mechanical} `docs/lifecycle-kit/SPEC.md` is regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` in delta 2's commit.

## Producers and consumers

- **The derived couple (delta 1).** Producer: the descriptor's `couples=` field, expanded by the crate's one expander (`registry::expand_couples`). Its consumers are unchanged in kind:
  - the generated pre-commit hook and `--for`, through `runner::staged_matches`, which now select the gate for a staged `.md` under any kit root's `templates/`;
  - `check-graph`, which reds on a graph artifact that no longer matches the manifests until delta 3 regenerates `docs/check-graph.html`;
  - `check-kit-enum`, which reds on a couples set literally naming two or more kit roots under one glob suffix without naming every root carrying it. The literal list named its kits under distinct basenames, and the token leaves nothing literal to check, so it stays clean.
- **No corpus narrows.** The gate's read set is the shims and the templates their directives name, and delta 1 changes only which commits trigger it. The trigger set widens. No reader's red condition turns on the trigger set's size.
- **No new knob, state, event, name or interface.**

## Existing sections updated

Roster produced by `git grep -n "check-skill-binding" -- ':!TASK-QUEUE.md' ':!docs/posts'` and `git grep -n "economics.md\|by name" -- lifecycle-kit/SPEC.md lifecycle-kit/checks`, with each hit read.

- `lifecycle-kit/checks/check-skill-binding.gate` — the `couples=` field (delta 1).
- `docs/check-graph.html` — regenerated (delta 1's commit, delta 3).
- `lifecycle-kit/SPEC.md` — §check-skill-binding, the coupling paragraph (delta 2).
- `docs/lifecycle-kit/SPEC.md` — the regenerated mirror (deltas 2 and 3).

The other hits need no edit. §check-stage-skill-coverage says its manifest couples the stage-template dir "as `check-skill-binding` does", which stays true under the kit-wide token. §check-shim-restatement's coupling paragraph describes its own couple.

## Retired spellings

- `coupled **by name**` — the claim that the boundary skills are coupled one path at a time, retired by delta 2.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls lifecycle-kit/SPEC-*.md`), discharged at the iteration where sibling amendments for the component are in flight.
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **The entry moves** — `skill-binding-couples-drift` moves to Done with `--queue done` in the commit deleting this file, at its build batch, before the drain stage.
