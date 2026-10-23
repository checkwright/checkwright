# SPEC amendment: patch-paths

Pairs `codex-patch-path-residue`. Codex's patch tool matches the rendering's file-write group but carries the patch as the tool input's command, with no written path. `workflow-state-guard` reads only the written path and allows on an empty one (`native/src/hook/workflow_state.rs`, `run` and `superseded_stage`). So once Codex discovers the plugin's hooks, a Codex session could hand-write the stage state file. This amendment gives the `--hook` arm one reader of a write call's targets. The reader takes the written path where the tool input carries one, and otherwise each path a patch's file headers name. `workflow-state-guard`'s rules then read the paths the reader returns. It is designed with `SPEC-antigravity-binding.md`, whose adapter hands an Antigravity write tool's path on as the written path, so this reader serves all three harnesses and the two amendments land in either order.

**What was read at spec.**

- **The patch grammar** of `codex-cli` 0.163.0, read from the strings of its own binary (`strings -n 6` on the installed release). The tool's Lark grammar is:
  - `start: begin_patch environment_id? hunk+ end_patch`;
  - `begin_patch: "*** Begin Patch" LF` and `end_patch: "*** End Patch" LF?`;
  - `environment_id: "*** Environment ID: " filename LF`;
  - `hunk: add_hunk | delete_hunk | update_hunk`;
  - `add_hunk: "*** Add File: " filename LF add_line+`, `delete_hunk: "*** Delete File: " filename LF` and `update_hunk: "*** Update File: " filename LF change_move? change?`;
  - `change_move: "*** Move to: " filename LF`;
  - `filename: /(.+)/`.

  The same binary carries the refusal `hook returned updatedInput without string field \`command\``, consistent with the patch arriving as the tool input's `command`.
- **The payload** is the plugin SPEC's run record (plugin/SPEC.md §The guards): the patch tool matches `Write|Edit` and carries the patch text, never a written path. No hook payload was captured at spec.

**Inferred, cannot run before build:** that Codex resolves a relative patch filename against the session's working directory, the payload's `cwd` — the live check delta 4 names needs a guard that reads patch headers, which build lands.

**Inferred, cannot run before build:** that the patch tool's hook payload carries `cwd`, which the shipped binding's common-payload row lists and no member reads yet — the same live check captures that payload, and it waits on the same build.

The plugin's own hooks cannot carry that check, since `codex-cli` 0.163.0 discovers none beside the root manifest (plugin/SPEC.md §The guards). So the build's oracle is the crate cases below, with a project-level hook as the live check.

**Shapes refused.**

- **Keying on the tool name.** The reader keys on the command's first non-blank line being `*** Begin Patch`, never on a tool name. The rendering's matcher already scopes the call to the file-write tools, and the patch tool's hook-facing name was never captured, so a name test would rest on a spelling nobody has read.
- **A guard of its own for patches.** The two rules already state what a write may target. A second member would fork the state-file predicate and the scratch-dir resolution that `workflow_state.rs` holds.

## What changes

### (1) The hook contract names the patch {design-bearing} {user-facing: codex-patch-path-residue's Deliverable, "read the patch's file headers for the written paths … designed with antigravity-hook-binding's write-tool reader as one `--hook` arm reader for a foreign write payload's path"}

`gate-sdk/SPEC.md` §The hook contract. **Not yet applied.**

The payload bullet's phrase "the *written path*" becomes "the *written path*, or a *patch* carried as the command, whose file headers name each path it writes".

A new paragraph after the payload bullet's list closes, before **The binding table, Claude Code.**:

> **A write call's targets have one reader**, `written_paths` in `native/src/hook/patch.rs`. Where the tool input carries a written path, that path is the call's one target. Otherwise, where the command's first non-blank line is `*** Begin Patch`, every `*** Add File: `, `*** Update File: `, `*** Delete File: ` and `*** Move to: ` line names a target, and an `*** Environment ID: ` line names none. Otherwise the call has none. A relative target is joined to the payload's working directory where it carries one, else to the tree the arm entered. The headers are the patch tool's grammar, which a harness speaking the shipped binding may carry in its command; a patch not opening with the begin line is no patch, and its targets are none.

### (2) The state-file and superseded-stage rules read every target {design-bearing} {user-facing: as delta 1}

**Not yet applied.**

**`lifecycle-kit/SPEC.md` §check-stage-evidence**, in the paragraph opening **The `workflow-state-guard` harness-integration arm**, the sentence "It refuses a write whose target **resolves** to the state file, naming `--enter-stage` as the sanctioned writer." becomes:

> It refuses a write any of whose targets **resolves** to the state file, the targets being the written path or a patch's file headers (gate-sdk/SPEC.md §The hook contract), and names `--enter-stage` as the sanctioned writer.

In the residual paragraph, "Otherwise the guard reaches agent `Write`/`Edit` tool calls and nothing else." becomes "Otherwise the guard reaches the agent's file-write tool calls, a patch tool's included, and nothing else." A patch applied through a shell command is a shell call, read by the shell guard, which reads no patch headers. That sentence joins the misses the paragraph lists.

**`lifecycle-kit/SPEC.md` §check-dispatch-entry**, the third rule's sentence "A target is inside it by the rule's own resolution, the nearest existing ancestor canonicalized, and a `..` component never is." becomes:

> A write passes only when it has a target and every target is inside it, by the rule's own resolution: the nearest existing ancestor canonicalized, and a `..` component never inside.

Its honest limits' "The rule reaches `Write` and `Edit` and nothing else." becomes "The rule reaches the file-write tools, a patch tool's included, and nothing else."

### (3) The reader and the rules in the crate {design-bearing}

All **Not yet applied.**

- **`native/src/hook/patch.rs`**, its first line a `spec:` binding to gate-sdk/SPEC.md §The hook contract. It holds:
  - the header spellings as `const`s;
  - `written_paths(payload: Option<&Value>) -> Vec<String>` as delta 1 states it, reading through `hook::field` and `binding.rs`'s `TOOL_INPUT`, `WRITTEN_PATH`, `COMMAND` and `WORKING_DIR`;
  - unit cases:
    - a written path alone;
    - a written path beside a patch command, where the written path wins;
    - one patch carrying each header, an `*** Environment ID: ` line and an `*** Update File: ` with a `*** Move to: `, both of the latter's paths returned;
    - a relative header joined to `cwd`, and to no `cwd`;
    - a command not opening with the begin line;
    - a begin line after blank lines;
    - CRLF line ends;
    - a header whose filename carries trailing blanks, kept as the grammar's `/(.+)/` keeps them.
- **`native/src/hook/mod.rs`** declares `pub mod patch;`, and `hook/mod.rs`'s owner test is unchanged, since `patch` is no member.
- **`native/src/hook/workflow_state.rs`**:
  - `run` blocks when any of `patch::written_paths(payload)` is the state file, through `same_file`;
  - `superseded_stage` returns 0 only when the list is non-empty and every member is `under` the scratch dir;
  - the block messages name the target that tripped them;
  - the module's two reads of `[TOOL_INPUT, WRITTEN_PATH]` are deleted, so no second reader remains.
- **`lifecycle-kit/gate-tests/stamp-before-write.test.sh`** gains a `patch_payload` builder and two cases:
  - a superseded caller's patch naming one target under the scratch dir and one outside it is blocked;
  - the same patch with both targets under the scratch dir passes.
- **The first rule's patch case** is held by a `workflow_state.rs` unit case: a patch whose `*** Update File: ` names the state file blocks. It runs where the arm's other unit cases do, since its test module already resolves the repository's own state file.

### (4) The plugin's Codex limit narrows {mechanical} {user-facing: as delta 1}

**Not yet applied.**

`plugin/SPEC.md` §The guards, the Codex bullet "Two of the rendering's blocks reach less there." becomes:

> - **The wakeup guard never fires there**, since no Codex tool matches `ScheduleWakeup|CronCreate`. Its patch tool matches `Write|Edit` and carries the patch as the command, and `workflow-state-guard` reads each path the patch's file headers name (gate-sdk/SPEC.md §The hook contract).

`plugin/SPEC.md` §Honest limits, the bullet "Codex runs no guard from this package, and once it discovers them, still no wakeup guard and no `workflow-state-guard` over a patch (§The guards)." becomes "Codex runs no guard from this package, and once it discovers them, still no wakeup guard (§The guards)."

**The live check, where it can run.** The Codex plugin's hooks are undiscovered, so the build wires `workflow-state-guard` as a project-level Codex hook in a scratch vendored repository, under a scratch `CODEX_HOME` served by the local model provider plugin/SPEC.md §The validation leg names. The provider answers with one patch tool call whose `*** Update File: ` names the state file. The guard blocks it, and the captured payload settles both inferred lines above. Where the scratch home cannot open that session, the build records the check as not run in its journal, and the crate cases stand as the oracle.

### (5) The release declaration and the mirror {mechanical}

**Not yet applied.** `.workflow/release-declarations.md`, under **Behavior changes**, gains:

> - **`workflow-state-guard`** — reads every path a write call targets: the written path, or each file header of a patch carried as the command, a relative one joined to the session's working directory. A patch naming the lifecycle state file is refused, and a superseded stage session's patch passes only when every path is under the scratch dir. Nothing to do.

The docs mirror regenerates: `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write`.

## Producers and consumers

- **`patch::written_paths`** (deltas 1 and 3). Its producer is any write call on the file-write matcher: Claude Code's `Write` and `Edit`, Codex's patch tool where its hooks are discovered, and Antigravity's write tools through the adapter (`SPEC-antigravity-binding.md`). Its consumers are `workflow_state::run` and `workflow_state::superseded_stage`. A probe found no other reader of the written path: `git grep -n "WRITTEN_PATH" native/src` names `binding.rs`'s constant and `workflow_state.rs`'s two reads.
- **Each field the reader takes has its reader.**
  - `tool_input.file_path` is read as today.
  - `tool_input.command` was already a shipped-binding key, read by the shell guard on the shell tools; on a write call it is read only by this reader.
  - `cwd` is newly read, here alone. Its red condition is none: an absent `cwd` falls back to the entered tree, which is today's resolution.
- **Roster-holding readers, each red condition named:**
  - `binding.rs`'s table test reds a spelling the table carries and the module does not, or the reverse. `COMMAND`, `WRITTEN_PATH` and `WORKING_DIR` are already held, and the patch headers live in `patch.rs` and in prose outside the table, so neither reads them.
  - `hook/mod.rs`'s owner test reads `HOOKS`, which this amendment does not change.
  - `check-amendment-queue` reds a `[spec:]` ref resolving to no file. The paired entry carries this file's name.
  - `check-docs-mirror-fresh` reds until the mirror regenerates (delta 5).
  - `check-release-declaration-parity` holds the next composed note to delta 5's bullet.
  - `check-harness-literal` reds a configured literal outside a valved line on its corpus. Delta 1's header spellings and delta 2's sentences carry none of `GATE_SDK_HARNESS_LITERALS`' entries, and delta 4's text sits in `plugin/`, outside the corpus.
- **A narrowing, so the red conditions are named (point 5).** Delta 2 narrows what the third rule admits: a write with any target outside the scratch dir. Its red condition is a block, monotone in that set. A call with one target, every Claude Code write among them, reads exactly as today.

## Existing sections updated

- `gate-sdk/SPEC.md` §The hook contract, its payload bullet and a new paragraph (delta 1).
- `lifecycle-kit/SPEC.md` §check-stage-evidence, the `workflow-state-guard` paragraph and its residual, and §check-dispatch-entry, the third rule and its honest limits (delta 2).
- `native/src/hook/patch.rs`, `native/src/hook/mod.rs`, `native/src/hook/workflow_state.rs` and `lifecycle-kit/gate-tests/stamp-before-write.test.sh` (delta 3).
- `plugin/SPEC.md` §The guards and §Honest limits (delta 4).
- `.workflow/release-declarations.md` and the docs mirror (delta 5).

Roster probe: `git grep -n "workflow-state-guard\|patch text\|over a patch"` over the tracked tree, less the docs mirror, released notes, the queue and the amendments, names the prose sites `plugin/SPEC.md` lines 89 and 133, `lifecycle-kit/SPEC.md` line 1193, `lifecycle-kit/README.md` line 60, `guard-kit/README.md` line 35, `guard-kit/SPEC.md` lines 255 and 885 and `delegation-kit/SPEC.md` line 162, beside wiring, code and tests. Deltas 2 and 4 rewrite the first three. The READMEs name the wiring and the state file, and neither claims the written path is the only target. guard-kit's line 255 says the guard reads its path-bearing field in-crate, which delta 3 keeps, and line 885 names the predicate `is_state_file`, which delta 3 keeps. delegation-kit's line 162 names the guard as a shipped instance, with no target claim. `docs/enforcement.md` line 210 is the enforcement map's generated row, whose matcher is unchanged.

## Retired spellings

- None — no delta retires a spelling; "over a patch" and "carries the patch text" are rewritten in `plugin/SPEC.md` alone, which delta 4 names.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the root (`ls SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
