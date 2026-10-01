# SPEC amendment: anchored-wirings

**The SessionStart and statusLine wirings are anchored at the harness's project directory, as the guard hooks already are.** context-kit's SessionStart template spells `bash scripts/session-context.sh`. delegation-kit's README points the `statusLine` at `bash gate-sdk/bin/run-gates.sh --statusline`. Both commands are repository-relative. A hook or statusLine command runs in the session's current directory, so after a session `cd`s into a subdirectory each one exits 127. The status bar blanks, and a resume or compact re-fire loses its brief. guard-kit/SPEC.md §The shell guard already anchors the three guard hooks at `${CLAUDE_PROJECT_DIR}` and owns the grounds.

**Measured at authoring.** The installed harness's statusLine executor (version 2.1.286, read in its bundled source) runs the configured command through the same spawner as every hook command. That spawner sets `CLAUDE_PROJECT_DIR` in the child's environment, and its working directory is the session's current one. So the variable reaches a statusLine command exactly as it reaches a hook. This answers the queue entry's open premise by reading the program. A live statusLine run would need this repo's settings changed, which is the operator's act, and was not taken.

**Read at authoring.** Each reader of a wiring's command string, and how it takes the anchored spelling:

- `check-settings-paths` reads `hooks.*[].hooks[]` commands (not `statusLine`). It strips a leading `${CLAUDE_PROJECT_DIR}` (quoted or bare) before its existence check (`strip_project_root`, `native/src/gates/settings_paths.rs`), so the anchored SessionStart command still resolves.
- The bash-audience derivation's settings arm (`spawns_bash_in_settings`, `native/src/toolfloor.rs`) still finds `bash` as the first word.
- `check-door-binding` exempts `--statusline` and `--hook` wherever they sit (guard-kit/SPEC.md §check-door-binding).
- The enforcement-map emitter rows each SessionStart command by its first `/`-bearing token, verbatim (`native/src/emit/enforcement_map.rs`). So `docs/enforcement.md`'s session-warnings row changes when this repo's own SessionStart spelling does.
- No installer arm writes an adopter's settings (guard-kit/SPEC.md §check-door-binding), and `plugin/hooks/hooks.json` carries neither wiring.

## What changes

### (1) The kit wirings spell the anchored commands

{mechanical} Each surface takes the guard hooks' spelling, brace form inside double quotes:

- `context-kit/templates/settings-sessionstart.json` — the command becomes `bash \"${CLAUDE_PROJECT_DIR}/scripts/session-context.sh\"`. Its `//` comment keeps naming the gates-dir default `scripts/`, which the path still assumes.
- delegation-kit/README.md, step 3 — "point your harness `statusLine` at `bash gate-sdk/bin/run-gates.sh --statusline`" becomes "point your harness `statusLine` at `bash "${CLAUDE_PROJECT_DIR}/gate-sdk/bin/run-gates.sh" --statusline`".

### (2) The owning sections state that both wirings take the anchor

{mechanical} **Not yet applied.**

guard-kit/SPEC.md §The shell guard: the anchor paragraph's opening, "**The path is anchored at the harness's project directory**, because a hook runs in the session's current directory, which the session's own `cd` moves.", becomes:

> **The path is anchored at the harness's project directory**, because a hook or statusLine command runs in the session's current directory, which the session's own `cd` moves. The harness spawns both through one command runner that sets the variable. So every kit wiring takes this anchor — the guard hooks here, context-kit's SessionStart template and delegation-kit's statusLine step.

context-kit/SPEC.md §The session-context hook (template): the opening sentence "wired as the harness's session-start hook via `templates/settings-sessionstart.json`" gains ", its command anchored at the project directory (guard-kit/SPEC.md §The shell guard)".

### (3) The site mirrors follow

{mechanical} `docs/delegation-kit/README.md`, `docs/guard-kit/SPEC.md` and `docs/context-kit/SPEC.md` are regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write`, in the commits landing deltas 1 and 2.

### (4) This repo's own settings diff, prepared for the operator

{mechanical} **Not yet applied — the operator's to apply** (CLAUDE.md §Housekeeping, a settings edit is applied on the operator's behalf, and a delegated session only prepares the diff). Build relays this diff to the lead and does not write `.claude/settings.json`:

- `statusLine.command`: `bash gate-sdk/bin/run-gates.sh --statusline` → `bash "${CLAUDE_PROJECT_DIR}/gate-sdk/bin/run-gates.sh" --statusline`
- the `SessionStart` hook's `command`: `bash scripts/session-context.sh` → `bash "${CLAUDE_PROJECT_DIR}/scripts/session-context.sh"`

`docs/enforcement.md` is regenerated in the commit that applies it, with the command `check-enforcement-fresh` prints on red. Its session-warnings row changes with the SessionStart spelling. The entry's Done move rests on deltas 1 to 3 and the prepared diff, never on the operator's apply, because preparing the diff is the deliverable.

## Producers and consumers

- **The anchored command.** The producer is the harness, which spawns each wiring with `CLAUDE_PROJECT_DIR` set (measured above). The enabling configuration is an adopter merging the template or following the README step, and this repo once the operator applies delta 4.
  - The consumer is `bash`, which opens the absolute path. The front end and the hook script each `cd` to the repository toplevel before reading anything (`gate-sdk/bin/run-gates.sh`, `context-kit/templates/session-context.sh`), so anchoring the command path alone suffices.
  - **Honest limit**, guard-kit's own: on native Windows the variable arrives in a Windows spelling. That the host's bash opens the path it forms is inferred and not run.
- **Readers whose verdict moves, and their red conditions.**
  - `check-settings-paths` reds on a command path that does not exist after the project-dir strip. The anchored spelling strips to the same path, so it is unchanged.
  - `check-enforcement-fresh` reds on any byte diff between `docs/enforcement.md` and the emitter, so it moves only with delta 4, in that same commit.
  - No reader reds on finding none. The change re-spells two commands and neither narrows nor widens a corpus.
- **No new knob, state, event or interface.**

## Existing sections updated

Roster produced by `git grep -n 'statusLine\|SessionStart\|session-context.sh\|--statusline' -- . ':!TASK-QUEUE.md'`, with each hit read.

- `context-kit/templates/settings-sessionstart.json` — the command (delta 1).
- `delegation-kit/README.md` — step 3 (delta 1).
- `guard-kit/SPEC.md` — §The shell guard, the anchor paragraph (delta 2).
- `context-kit/SPEC.md` — §The session-context hook (template), the opening sentence (delta 2).
- `docs/delegation-kit/README.md`, `docs/guard-kit/SPEC.md`, `docs/context-kit/SPEC.md` — the regenerated mirrors (delta 3).
- `.claude/settings.json`, `docs/enforcement.md` — the operator's apply and its regeneration (delta 4).
<!-- update-target-exempt: a good-case fixture of a sweep that exempts the statusline arm wherever it sits; its relative spelling is the case, not a wiring -->
- `guard-kit/gate-tests/check-door-binding/good/alpha-kit/templates/settings-hooks.json` — left relative.

## Retired spellings

- None — the commands are re-spelled at their wiring sites, and the relative form survives as a fixture case and as the prose a reader types from the repository root.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls context-kit/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
