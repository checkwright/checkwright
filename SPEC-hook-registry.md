# SPEC amendment: hook-registry

A hook registration in the tracked harness settings file is read by two readers, and neither reads it whole.

- **`check-settings-paths` resolves no hook at all.** Its subject is `permissions.allow[]` alone (`native/src/gates/settings_paths.rs`, `allow_entries`). A hook whose `command` names a renamed or deleted script reds nowhere. The harness treats any exit status other than 2 as a non-blocking error, so a hook whose script is gone lets every call through and the behaviour it guards stops without a red or a symptom. This happened at `native-shell-guard` build: `scripts/bash-guard.sh` was deleted before its registration was swapped to `--hook shell-guard`, and the shell guard failed open for every session in between.
- **The enforcement map reads two events and credits every member row to gate-sdk.** `hook_sections` (`native/src/emit/enforcement_map.rs`) reads `/hooks/PreToolUse` and `/hooks/SessionStart` only, so the `SubagentStop` registration is not on the page. It takes the first slash-bearing token of a command as the surface, so every `--hook <member>` row reads `gate-sdk/bin/run-gates.sh` and is attributed to gate-sdk. Today the four Guards rows of `docs/enforcement.md` cannot be told apart.

**The ruling.** Widen `check-settings-paths` rather than minting a second gate. The canon-kit new-names litmus is met either way; the extra name buys nothing. The settings file, the fail-closed posture, the fixture mode and the command-token walk are all this gate's already, and a second gate would read the same file on the same terms. A `--hook` registration has two things to resolve, the front end and the member name, and the gate resolves both. The member half is a commit-time catch for a failure that is loud at run time rather than silent: an unknown member exits 2 and blocks every guarded call. The enforcement map reads every event under `hooks`, and credits a member row to the kit that owns the member, as recorded on the member table.

**Measured at authoring (2026-09-24):**

- **The live registrations.** `python3 -c` over `.claude/settings.json`'s `hooks` shows three events: `PreToolUse` (three groups, four commands), `SessionStart` (one) and `SubagentStop` (one). Every command token resolves (`ls scripts/session-context.sh gate-sdk/bin/run-gates.sh`). Every `--hook` operand is a `HOOKS` member (`native/src/hook/mod.rs`). The widened gate registers green, so no prune has to land first.
- **The owners.** `head -2 native/src/hook/*.rs` shows each member module's leading `spec:` binding. The agent-budget-guard, agent-dispatch-guard and subagent-stop-liveness modules bind to delegation-kit. The escalation-guard, shell-guard and wakeup-guard modules bind to guard-kit. The workflow-state-guard module binds to lifecycle-kit.
- **The event order.** `native/Cargo.toml` builds `serde_json` without `preserve_order`, so an object's keys iterate sorted.
- **The placeholder.** The harness's hooks reference (`https://code.claude.com/docs/en/hooks.md`, *Reference scripts by path*) documents `${CLAUDE_PROJECT_DIR}` as the project root for a hook's command. It also documents an exec form in which `args` carries the argv.

## What changes

### (1) check-settings-paths resolves every hook command's path {design-bearing}

**Not yet applied.** In context-kit/SPEC.md §check-settings-paths, the first paragraph's invariant sentence becomes:

> holds the second invariant over the same tracked file: every path the file tells the harness to run resolves in the working tree. That is each `permissions.allow[]` entry whose command token is a **literal** repo-relative `.sh` path, and each `type: command` hook under `hooks`, whatever its event, whose command token is a literal repo-relative path.

The class paragraph gains, after its first sentence:

> A dead hook costs more than a dead grant. The harness runs the command, any status but 2 is a non-blocking error to it, and so a hook whose script is gone lets every call through while the rule it carried stops with no red anywhere. Deleting a script before swapping its registration opens exactly that window.

The extraction-predicate paragraph gains, after its `*`-twin sentence:

> **A hook's candidate is scoped differently, on the same walk.** Its tokens are `command` split on ASCII whitespace, or, in the exec form, `command` followed by each `args` element verbatim. The env and interpreter skip is the one above. The candidate is taken when it carries a `/`, whatever its extension. The runtime-created ground that scopes a grant to `.sh` does not transfer: a hook names what the harness executes, never a path it creates. A leading project-root placeholder (`${CLAUDE_PROJECT_DIR}` or `$CLAUDE_PROJECT_DIR`, quoted or bare, then `/`) is stripped, and the rest resolves repo-relative, because that is the root the harness substitutes. A candidate carrying any other `$` names a root outside the tree, such as a plugin's, and is skipped and counted. A hook running its script through an interpreter other than `bash`/`sh` (`node`, `pwsh -File`) puts the script in argument position, where the walk does not look: this is a stated limit, not a scope.

The dispositions paragraph's finding sentence becomes:

> exit 1 lists each violating grant verbatim, and each violating hook as `hooks.<Event>[<i>].hooks[<j>]: <command>`, beside the path that did not resolve, so the reader can repoint or drop the entry without re-deriving which token was read.

Its clean-line sentence becomes:

> The clean line reports the **checked counts**: grants, hooks, and hooks skipped for a placeholder. That is what distinguishes a predicate that scoped to its subject from one that vacuously matched nothing.

The fixture paragraph becomes:

> The pair pins the scoping, not merely the verdict. `good/` carries every skipped grant shape — pattern tokens, bare non-path commands, non-`.sh` paths — alongside resolving literals in all three extraction shapes (bare, `env`-prefixed, trailing-flag). It also carries hooks under two events: a bare script, an `env`-prefixed one, a placeholder-prefixed one, an exec-form `args` one, a `--hook` registration naming a member the binary carries, a `$`-rooted token and a non-`command` hook type. Its expectation pins **both checked counts**. The pattern-expansion defect above passes an exit-code-only fixture, because an expanded pattern resolves by construction, and is visible only in the count. `bad/` carries a dead grant path in the bare, `*`-twin and `env`-prefixed shapes, a dead hook script, and a `--hook` operand the binary does not carry, each beside a resolving one, so a broken extraction arm shows as a missing finding rather than a still-red exit.

### (2) A `--hook` registration's member resolves against the member table {design-bearing}

**Not yet applied.** In context-kit/SPEC.md §check-settings-paths, after the new hook-candidate paragraph of delta 1:

> **A hook naming gate-sdk's front end names a member too, and both resolve.** Where a hook's command token has the front end's file name (`run-gates.sh` or `run-gates.ps1`) and the next token is `--hook`, the token after it must be a member the gate binary carries (gate-sdk/SPEC.md §The harness-integration arm). A missing operand is a finding too. The member roster is read in process from the binary's own table, so the gate and the dispatch it predicts cannot disagree. An unknown member is loud at run time rather than silent: the arm exits 2 and blocks every call the hook guards. The gate catches it at the commit instead of in a wedged session. The finding reads `— no such hook member: <name>`, and its `help:` line prints the roster.

In gate-sdk/SPEC.md §The harness-integration arm, the member-table paragraph's list of what the table is (see delta 3) names this reader. The registration grammar has one parser beside the table in `native/src/hook/mod.rs`. It reads a command's tokens, finds the front end as its command token and returns the `--hook` operand. The gate and the enforcement map (delta 4) both call it, so the two readers of a registration cannot parse it two ways.

### (3) The member table carries each member's owning kit {mechanical}

**Not yet applied.** In gate-sdk/SPEC.md §The harness-integration arm, the member-table paragraph's first two sentences become:

> **The member table is the single roster.** `native/src/hook/mod.rs` owns a `HOOKS` table keyed by member name. Each row carries the member's function, its declared knob slice and its **owning kit**, the kit whose SPEC states the member's rule, with one module per member beside it. A unit test holds each row's owner to the kit named by its member module's leading `spec:` binding, so the owner is checked against the module rather than transcribed. The table is what the arm dispatches on, what the `--hook` row's `EVERY_HOOK_KNOB` sentinel stands for, what the unknown-member refusal prints, what `check-settings-paths` resolves a registration's member against, and what the enforcement map credits a member row to. It is derived once and never transcribed.

The owner is a table column and not a derivation, and the alternatives were measured. Knob prefixes do not name a member's kit. The escalation guard declares no knob, and the workflow-state guard declares a gate-sdk knob for a lifecycle-kit rule. A runtime read of the module's comment is not available to the shipped binary.

### (4) The enforcement map reads every hook event and credits each member to its owner {design-bearing}

**Not yet applied.** In gate-sdk/SPEC.md §enforcement-map, the registry paragraph's guards clause becomes:

> **session warnings** come from the `SessionStart` command hooks, and **guards** from the command hooks of every other event, in the tracked harness settings file (`CONTEXT_KIT_SETTINGS_FILE`, parsed with the crate's own JSON reader). Events are sorted by name and hooks keep registration order within one. A guard row carries its event beside its *intercepts* cell. That cell carries the harness matcher verbatim, or `*` where the event takes none, so its `|` alternation is escaped on the way into the table — a case a freshness gate can never report, because it compares the emitter against the page and both would carry the same broken row. A hook that registers a member through gate-sdk's front end is rowed under the member's name and credited to the kit the member table names as its owner (§The harness-integration arm). Any other hook is rowed and credited by its command path. An operand the table does not carry keeps the path attribution, and `check-settings-paths` is the gate that reds it. Every event is read, never a roster of events, because a registration the page cannot see is a guard the map says does not exist.

In the emitter's taxonomy (`class_roster`, `native/src/emit/enforcement_map.rs`), the guard bullet becomes:

> A **guard** intercepts a harness event — a tool call before it runs, a turn end before it completes — and can refuse it.

The Guards table's columns become `kit | surface | event | intercepts`, and the Session warnings table is unchanged.

## Producers and consumers

- **The hook findings (deltas 1, 2).**
  - Producer: `check-settings-paths`, `precommit`. It is already registered, so its generated-hook trigger on the settings file and its whole-tree battery run are the paths that reach it. Its module imports the member table, so its derived module cut reaches `native/src/hook/mod.rs` with no hand-written coupling (gate-sdk/SPEC.md §The harness-integration arm, the cut).
  - Consumer: the committing session, through the output contract. CI reads it through the battery.
- **The registration parser (delta 2).** It has two named callers, the gate and the emitter. It is a function and not an arm, so there is no knob and no field.
- **The owner column (delta 3).**
  - Readers: the enforcement map's member rows (delta 4) and the new unit test.
  - The value rollup reads the map's per-kit counts through `enforcement_map::measure` (`native/src/emit/value_rollup.rs`), so its Guards column moves from gate-sdk to three kits. That is a regenerated projection and not a new reader.
- **The event column (delta 4).**
  - Reader: the page, and the value rollup's section titles. The section titles are unchanged.
  - `check-enforcement-fresh` byte-compares the page. Its `good/` case's staged page carries the taxonomy line delta 4 rewrites, so the case is regenerated.
- **Point 5.** No corpus narrows. Both readers widen. `check-settings-paths` reds on a violation and never on a zero count, and its clean-line count is pinned by the fixture, which delta 1 rewrites.
- **Point 6.** The obliged members are enumerable, and each one's value is measured above.
  - Every live hook registration's command token resolves. Each `--hook` operand is a table member.
  - Every table member's owner is named above: agent-budget-guard, agent-dispatch-guard and subagent-stop-liveness → delegation-kit; escalation-guard, shell-guard and wakeup-guard → guard-kit; workflow-state-guard → lifecycle-kit.

## Existing sections updated

The roster comes from `git grep -n "hook_sections\|HOOKS\b\|literal_script_path\|intercepts a tool call\|surface | intercepts\|SessionStart" -- '*.rs' '*.md'` and the generated-projection roster in `docs/site-architecture.md`, both run 2026-09-24.

- `context-kit/SPEC.md` §check-settings-paths (deltas 1 and 2).
- `native/src/gates/settings_paths.rs` (deltas 1 and 2).
- `context-kit/gate-tests/check-settings-paths/good/` and `bad/` — settings, fixture scripts and `expect.txt` (deltas 1 and 2).
- `gate-sdk/SPEC.md` §The harness-integration arm (deltas 2 and 3).
- `native/src/hook/mod.rs` and the member modules beside it (deltas 2 and 3).
- `gate-sdk/SPEC.md` §enforcement-map (delta 4).
- `native/src/emit/enforcement_map.rs` (delta 4).
- `gate-sdk/gate-tests/check-enforcement-fresh/good/docs/enforcement.md`, regenerated (delta 4).
- `docs/enforcement.md` and `docs/value.md`'s rollup block, regenerated by their gates' printed commands (deltas 3 and 4).
- `.workflow/surface-ceiling.txt`, re-baselined if `check-surface-ratchet` reds on the page's growth (delta 4).
- `.workflow/release-declarations.md` (deltas 1, 2 and 4). Under Tightened gates: `check-settings-paths` now resolves every hook command's path and every `--hook` member. Under Behavior changes: the enforcement map rows every hook event and credits a member row to its owning kit.
<!-- update-target-exempt: generated mirror, regenerated by its freshness gate's printed command -->
- `docs/context-kit/SPEC.md` and `docs/gate-sdk/SPEC.md`.

## Retired spellings

- `A **guard** intercepts a tool call before it runs` — the taxonomy line delta 4 rewrites. Its survivors are the emitter and the two pages it generates (delta 4).

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The causal-completeness check holds for the hook findings, the registration parser, the owner column and the event column.
- [ ] **Instruction surfaces: instruction only.** No instruction surface is edited.
- [ ] **Merged with no information lost.** Each SPEC edit re-phrases the passage it refines, and the ruling's grounds move into §check-settings-paths and §enforcement-map.
- [ ] **Amendment deleted.** This file is removed on merge (`ls SPEC-*.md`).
- [ ] **Entries moved.** `settings-hook-command-path-gate` and `enforcement-map-member-owner` move to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** `check-amendment-retired-spelling` runs the declared spelling over the tracked tree.
- [ ] **Gaps filed.** Any cross-component gap found during the work is filed to the gap inbox.
