# SPEC amendment: violation-recipe

**guard-kit ships a `smoke/violation.sh` that plants a front-end door in the vendored kit's `README.md`, so `--run-consumer-smoke` proves the kit's one gate, `check-door-binding`, reds a scratch consumer.** gate-sdk/SPEC.md §Consumer smoke owes the recipe wherever a battery-reddening violation is craftable. guard-kit registers `check-door-binding` (its `smoke/install.sh` appends it to the scratch `scripts/gates.list`) and ships none, so the harness reports guard-kit as install coverage only. The violation is craftable: assertion A reds on a kit root's `README.md` line naming the front end as a command to run, the shape the gate's own `bad/` fixture holds (`alpha-kit/README.md:3`).

**Measured at authoring.** Appending the line delta 1 plants to this repo's `guard-kit/README.md` turns `bash gate-sdk/bin/run-gates.sh --only check-door-binding` red. Over the full battery it also reds `check-docs-mirror-fresh` and `check-license-line`, both registered from this repo's `scripts/` and withheld from the payload, so neither runs in the scratch consumer. The tree was restored after the probe.

## What changes

### (1) `guard-kit/smoke/violation.sh` plants one assertion-A door {mechanical}

A new file, shaped as queue-kit's recipe: the `# spec:` and `# no-port:` header pair, the entry-point guard, the expected gate's name as the first stdout line, then one appended line on a file the scratch baseline tracks, so the harness's restore reverses it.

```bash
#!/usr/bin/env bash
# spec: gate-sdk/SPEC.md §Consumer smoke — guard-kit consumer-smoke violation: a front-end door in the kit README reddens check-door-binding
# no-port: gate-sdk/SPEC.md §Consumer smoke, The port disposition — legs 2 and 3.
set -euo pipefail
: "${SMOKE_KIT_ROOT:?run via --run-consumer-smoke on the gate binary}"

echo "check-door-binding"

printf '\n%s\n' 'Regenerate the hooks: `bash gate-sdk/bin/run-gates.sh --emit git-hooks --write`.' >> "$SMOKE_KIT_ROOT/README.md"
```

Under guard-kit's shipped defaults, with no consumer config: assertions A and B read every kit root's `README.md` and its `templates/`, `lib/` and `bin/`, while `GUARD_KIT_DOOR_ROOTS` is empty, so C is inert. The planted line names the front end after its interpreter word and its arm, `--emit`, is outside the fail-open set, so it is a door. The recipe's own text sits under `smoke/`, which the gate prunes, so the recipe is not itself a finding in this repo. It runs unchanged in guard-kit's self-sufficiency repeat, since the README is vendored with the kit.

**Inferred, cannot run before build:** the end-to-end red in the scratch consumer, and that no other gate the scratch registers reads a trailing README line outside a roster marker block — `--run-consumer-smoke` runs only once the recipe exists, and build runs it.

### (2) §Testing names the recipe {mechanical}

guard-kit/SPEC.md §Testing. **Not yet applied.** After the bullet list closing the `smoke/install.sh` paragraph, before the `gate-tests/compare-settings-allow.test.sh` paragraph, add:

> `smoke/violation.sh` plants one `check-door-binding` violation: a line naming the front end as a command to run, appended to the vendored kit's `README.md`, which assertion A reads under the shipped defaults. Assertions B and C are held by the gate's fixture pair.

### (3) The layout block lists the file {mechanical}

guard-kit/SPEC.md, the kit layout code block. **Not yet applied.** Under the line `  smoke/install.sh`, add `  smoke/violation.sh`.

### (4) The site mirror follows {mechanical}

`docs/guard-kit/SPEC.md` is regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` in the commit landing deltas 2 and 3.

## Producers and consumers

- **The planted door.** Producer: `smoke/violation.sh`, run by `--run-consumer-smoke` in the union pass and again in guard-kit's self-sufficiency repeat. Enabling configuration: guard-kit's own `smoke/install.sh` registers the gate. Consumer: the harness's red-phase assertions (a non-zero battery exit, a `FAIL: check-door-binding` line, a SARIF result carrying that rule id), then its `git reset --hard` and `git clean` restore.
- **The first stdout line** is read by the harness's fire step alone.
- **Roster-holding readers of the new file**, each by derivation over the kit roots, so none is edited: `check-smoke-entry-guard` (the recipe carries the guard), `check-test-hermetic`, `check-install-disposition`, and `.claude/settings.json`'s glob grant for `*/smoke/violation.sh`. `docs/check-graph.html` already carries a `guard-kit/smoke/violation.sh` node through a `couples=` glob. The consumer smoke's success line counts fired violations per run, so its count rises by one with no edit.
- **No new state, knob, event or interface.** No delta is user-facing: `smoke/` is withheld from the payload (gate-sdk/SPEC.md §Consumer payload), so an adopter meets no new output, exit, default or verdict.

## Existing sections updated

Rosters by `git grep -n "smoke/violation.sh"` and `git grep -n "violations fired\|kits installed"` over the tracked tree, and `ls */smoke/violation.sh` (nine kits, not guard-kit).

- `guard-kit/smoke/violation.sh` — new (delta 1).
- `guard-kit/SPEC.md` — §Testing and the kit layout block (deltas 2 and 3).
- `docs/guard-kit/SPEC.md` — the regenerated mirror (delta 4).

## Retired spellings

- None — the change adds a recipe and its prose; it retires no name.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — the recipe carries commands, not grounds.
- [ ] **Merged with no information lost** — §Testing reads as one section naming both recipes.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls guard-kit/SPEC-*.md`).
- [ ] **Removals propagated** — `check-amendment-retired-spelling` green over the declaration above.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks.
- [ ] **Run green** — `bash gate-sdk/bin/build-native.sh`, the full battery, and `--run-consumer-smoke` with guard-kit's recipe firing, in the merging batch.
- [ ] **The entry is done** — `guard-kit-violation-recipe` moves to Done in the merging commit, before the drain stage.
