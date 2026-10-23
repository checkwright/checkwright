# SPEC amendment: antigravity-binding

Pairs `antigravity-hook-binding`. The plugin's guards reach Claude Code alone; Antigravity installs the package with its hooks skipped (plugin/SPEC.md §The guards). This amendment adds a second binding to gate-sdk/SPEC.md §The hook contract, Antigravity's, reached through an adapter on the `--hook` arm. It also renders the plugin's guards into the hook file Antigravity reads. The shell guard and `workflow-state-guard` run unchanged behind the adapter. `SPEC-patch-paths.md` is designed with it: the adapter hands a write tool's path to the one written-path reader that amendment gives `workflow-state-guard`, so either can land first.

**What was run at spec**, with `agy` 1.3.4. None of it touched the user's agy configuration: the plugin was a workspace-local copy under a scratch repository's `.agents/plugins/`, and the installs used a scratch `HOME`.

- **The CLI's built-in documentation** is embedded in the binary and was read from it: the hooks page (`hooks.json`, events, the input and output contracts) and the plugins page.
- **Installs under a scratch `HOME`.**
  - `agy plugin install` of a directory carrying a root `plugin.json` and a root `hooks.json` prints `hooks : 1 processed`. It copies the whole directory, a script beside the hook file included, to `<HOME>/.gemini/config/plugins/<name>/`, and `agy plugin list` shows `Components: 1 hook`.
  - A copy of `plugin/` with a root `hooks.json` validates with `hooks 1` and installs with `13 skills, 1 hook`. `agy plugin validate` warns `hooks/hooks.json: never loaded: plugin hooks are read only from hooks.json at the plugin root`. It also notes that a manifest declaring the Agent Plugins `$schema` loads as an Agent Plugins plugin while the CLI's `enable-agent-plugins-spec` flag is on, and as a native plugin otherwise.
- **Four headless sessions** (`agy -p`, model `gemini-3.8-flash-low`, signed in by the operator). Each ran in a scratch git repository whose `.agents/plugins/probe/` held a `SessionStart` hook and a `PreToolUse` hook matching `*`. The hook logged its payload, working directory and environment, and answered by the payload's content.
  - Both events fire. The hook runs with its working directory, and `PWD`, set to the plugin's directory. No variable names the project directory; `ANTIGRAVITY_CONVERSATION_ID` is the one agy-specific variable.
  - The payload carries **no event name**. Its common fields are `conversationId`, `workspacePaths` (one entry, the launch directory), `transcriptPath`, `artifactDirectoryPath` and `modelName`, and a tool call adds `stepIdx` and `toolCall`.
  - `toolCall.name` is `run_command` for a shell call, with `args.CommandLine`, `args.WaitMsBeforeAsync` and `args.toolSummary`. It is `write_file` for a file write, with `args.Path` and `args.Content`, and `edit_file` for an edit, with `args.Path`, `args.Target` and `args.Replacement`.
  - **Empty stdout at exit 0 is no decision.** The call proceeds to the harness's own permission check: a `write_file` ran under `--mode accept-edits`.
  - **`{}` denies**, with an empty reason, and so does any object without a recognized decision. `{"decision":"deny","reason":"<r>"}` denies, and the model reads `tool call denied by pre-tool hook: <r>`.
  - **A non-zero exit fails the call.** At exit 2 the model read `JSON hook "<name>" failed: command failed: exit status 2, stderr: <text>`, and the tool did not run.
  - `{"decision":"allow"}` on a `run_command` did not lift headless mode's own auto-deny of the call.
- **Not run**:
  - A hook from a plugin installed into a signed-in `HOME`'s `.gemini/config/plugins/`. That would write the user's own agy configuration. The workspace copy and the install path are loaded by the same runtime plugin discovery (the plugins page), so the installed path's firing is inferred from the workspace path's.
  - Native Windows. The hooks page says a command runs under `cmd /c` there.
  - The `write_to_file` and `replace_file_content` tools, which the binary's own tool examples spell with `args.TargetFile`.

**The mechanism, and the shapes refused.**

- **An adapter self-spawns the shipped member.** The `--hook` arm, given `--binding antigravity`, translates the payload into the shipped binding's shape, runs the named member as a child of the same binary, and translates the child's verdict into Antigravity's decision. The members print their envelopes at many sites, so translating in process would put a binding test at each one. The child keeps every member, envelope and spelling of the shipped binding as its single copy, and the adapter is a pure pair of translations a unit test reaches without any member. It costs one process per guarded call.
- **A grant is withheld.** Every grant and rewrite the shell guard issues rests on a model of the shipped binding's permission matcher (guard-kit/SPEC.md §The generic ruleset). guard-kit already withholds every grant from the PowerShell tool for the same reason: "A missing grant costs a permission decision, while a wrong one would bless a call". So under the adapter a grant is no decision.
- **The plugin's command reads the payload inline, and no launcher script ships.** The command is one line in the rendered hook file, as the Claude Code rendering's prefix is. A launcher file was refused at harness-reach-pass as interpreter surface. The inline prefix reads one field with `sed`, finds the repository with `git`, and hands the whole payload to the front end, whose binary does every JSON read that decides anything.
- **The command is a POSIX `sh` line that `cmd` reads as a comment.** It opens with `rem=1;`. `sh` reads that as an assignment, while `cmd` splits a command word at `=` and runs `rem`, which ignores the rest of the line and exits 0 with nothing on stdout. A native Windows host therefore takes no decision, and runs no guard rather than failing every matched call. That is inferred, and delta 5's Windows test is its oracle.

## What changes

### (1) The hook contract ships a second binding {design-bearing} {user-facing: antigravity-hook-binding's body, "a binding needs payload and envelope translation in the `--hook` arm plus project discovery from `workspacePaths` without a launcher script", within the set record's option A, operator direction 2026-10-23}

All in `gate-sdk/SPEC.md` §The hook contract. **Not yet applied.**

Replacement for the opening paragraph's last two sentences, from "The kit ships one binding, Claude Code's.":

> The kit ships two bindings. Claude Code's is the one every kit template and registration speaks, and a harness speaking it reaches every member unchanged. Antigravity's reaches the before-tool-call members through an adapter on the arm (§The harness-integration arm). A harness speaking a third protocol is unwired until a binding of its own is built (plugin/SPEC.md §The guards).

New paragraphs after **Honest limits.**, the last paragraph before §The port-candidate criteria:

> **The binding table, Antigravity.** The adapter's binding: no kit template speaks it, and the plugin's rendering is its one wiring (plugin/SPEC.md §The guards).
>
> - Events: before-tool-call `PreToolUse`, which the payload does not name. <!-- harness-binding: the Antigravity binding table -->
> - Tools: shell `run_command`; file-write `write_file` and `edit_file`, and `write_to_file` and `replace_file_content`. <!-- harness-binding: the Antigravity binding table -->
> - Common payload: `workspacePaths`, the launch directory first; also `conversationId`, `transcriptPath`, `artifactDirectoryPath`, `modelName` and `stepIdx`, which no member reads. <!-- harness-binding: the Antigravity binding table -->
> - Tool call: `toolCall`, carrying `name` and `args`; `args` carries `CommandLine`, and the written path as `Path` or `TargetFile`. <!-- harness-binding: the Antigravity binding table -->
> - Envelopes: block `{"decision":"deny","reason":…}`. No decision is an empty stdout, and every other object, `{}` included, blocks. A non-zero status fails the call. <!-- harness-binding: the Antigravity binding table -->
> - Variables: none. The hook runs in its hook file's directory. <!-- harness-binding: the Antigravity binding table -->
> - Wiring: a `hooks.json` keyed by hook name at the plugin root; `command`, run by `sh -c`, or `cmd /c` on Windows; `timeout`, default 30 seconds. <!-- harness-binding: the Antigravity binding table -->
>
> **Its measured record.** The rows were read off the CLI's embedded hooks documentation and off recorded firings of a plugin's hooks in a workspace-local copy. `write_to_file`, `replace_file_content` and `TargetFile` are read off the binary's own tool examples and have not fired. The table has no transcript, permission-rule, child or turn-end row: the adapter serves before-tool-call members alone, and no member it serves reads those roles.
>
> **Its one copy** is `native/src/hook/antigravity.rs`, which also holds both translations. Its unit test holds every row and the module to one set of spellings, as `binding.rs`'s holds the shipped table.

### (2) The arm takes a binding {design-bearing} {user-facing: as delta 1, for the `--binding` token an adopter's hook command carries}

New paragraph in `gate-sdk/SPEC.md` §The harness-integration arm, after the paragraph opening **`--hook <member>` takes the member name as argv**. **Not yet applied.**

> **`--hook <member> --binding <name>` reaches a member through a foreign binding's adapter.** `<name>` comes from the crate's closed set, `antigravity` alone, and without the token the arm speaks the shipped binding. The adapter reads the payload whole, translates it into the shipped binding's shape, and runs `--hook <member>` as a child of the same binary, through the crosser, on the translated payload. It answers in its own binding's envelope:
>
> - **The payload.** The event is before-tool-call, and the working directory is the first workspace path. A shell call becomes the bash shell tool carrying the command, and a write call becomes the file-write tool carrying the written path. A payload without a tool call is handed on as no payload, the member's degraded input.
> - **The verdict.** A child's block, exit 2, is a deny whose reason is the child's stderr. Every other outcome is no decision: a grant, a rewrite, an advise, a notice, a decline and a failure to spawn. The child's stdout is never forwarded.
> - **A grant is withheld**, because each rests on a model of the shipped binding's permission matcher, the ground on which guard-kit grants nothing on the PowerShell tool (guard-kit/SPEC.md §The generic ruleset).
> - **The adapter always exits 0**, since a foreign binding may fail a call on any other status. An unknown `<name>` is still exit 2 with the set named, on the shape of the unknown-member refusal.
>
> Its honest limit is that an advise reaches nobody: the binding has no channel that leaves the call to proceed and reaches the model.

### (3) The adapter module {design-bearing}

All **Not yet applied.**

- **`native/src/hook/antigravity.rs`**, its first line a `spec:` binding to gate-sdk/SPEC.md §The hook contract:
  - It holds delta 1's spellings as `pub const`s, with a test holding them to the new table in the shape of `binding.rs`'s. The split points are the table's own heading and **Its measured record.**, and the test also holds that `binding.rs`'s split still finds exactly its own rows.
  - It holds `translate_payload(&Value) -> Option<Value>`, which builds the shipped-binding payload from `binding.rs`'s own constants and never spells a shipped key again.
  - It holds `translate_verdict(status, stderr) -> String`, which returns the deny envelope or the empty string. The deny envelope's braces stay a literal with key order `decision`, `reason`, as `binding.rs`'s envelopes do, and the reason is serialized by `hook::quote`.
  - It holds `run(member, payload_bytes) -> i32`, which spawns `std::env::current_exe()` with `--hook <member>` through `proc::run`, its stdin the translated payload. It prints the translated verdict and returns 0.
- **`native/src/hook/mod.rs`**: `run()` reads `--binding <name>` after the member, before it reads stdin. It dispatches to `antigravity::run` for `antigravity`, and refuses any other name at exit 2. Tree entry stays where it is, so the child resolves the tree the parent entered.
- **The unit cases**, in `antigravity.rs`:
  - every observed payload shape at spec translated, the three tool calls (`run_command`, `write_file`, `edit_file`) and a payload with no `toolCall`;
  - a `TargetFile` path;
  - an unknown tool name handed on with no command and no path;
  - each verdict class: exit 2 with stderr, exit 0 with each shipped envelope, a non-zero non-2 status;
  - the deny envelope's byte order and the escaping of a quote in the reason.
- **The `registration` parser is unchanged**: it reads `--hook` and the token after it, so a trailing `--binding` names the same member. The case `a_registration_names_its_member_through_each_command_spelling` gains a spelling ending in `--binding antigravity`.

### (4) The plugin renders its guards for Antigravity {design-bearing} {user-facing: as delta 1, for what installing the plugin in Antigravity does}

**Not yet applied.**

**`plugin/SPEC.md` §The guards.** Replacement for the paragraph opening **Antigravity is not wired.**:

> **Antigravity reads `plugin/hooks.json`**, a second rendering of the same template in its own binding (gate-sdk/SPEC.md §The hook contract, the Antigravity table). It reads hooks only from a `hooks.json` at the plugin root and never from `hooks/hooks.json`, and no other harness reads the root file. The rendering:
>
> - is one hook named for the package, carrying one `PreToolUse` group per template group whose tools have an Antigravity spelling. Its matcher is those spellings joined by `|`: the shell group becomes `run_command`, and the file-write group becomes the four file-write tools. A group with no such tool, the wake group, is left out, and so is the PowerShell tool;
> - composes each `command` as `rem=1; ` followed by the Antigravity prefix, then the member, then the suffix.
>
> The prefix reads the payload once and takes the first `workspacePaths` entry with `sed`. It asks git for that path's toplevel and tests for the vendored front end and for `bash`, and it exits 0 with nothing on stdout where any of the three fails. Otherwise it enters the toplevel and hands the payload to the front end as `--hook <member> --binding antigravity`. The suffix prints the front end's stdout only where it opens with the deny envelope, and exits 0 whatever happened. So the front end's own operator notice, which Antigravity would read as a deny, never reaches it, and the prefix never fails a call.
>
> The hook runs in the plugin's own directory, so it finds the repository from the payload rather than from the working directory. On native Windows Antigravity runs the command under `cmd`, which reads the leading `rem` as a comment and exits 0 with nothing on stdout. There the package installs and runs no guard.

**`plugin/SPEC.md` §The harnesses**, the Antigravity row's hooks cell becomes: "run: `hooks.json` at the plugin root, its own rendering (§The guards), in a workspace-local copy; read: an installed copy and native Windows".

**`plugin/SPEC.md` §check-plugin-parity**, assertion (C) is replaced:

> - **(C) The hooks match the template.** `plugin/hooks/hooks.json` equals §The guards' rendering of `guard-kit/templates/settings-hooks.json`, and `plugin/hooks.json` equals its Antigravity rendering, each compared as JSON. So a block the template gains reds until both files carry it, or until the Antigravity rendering leaves it out by rule.

**`plugin/SPEC.md` §The validation leg**, the Antigravity bullet becomes:

> - **Antigravity.** `agy plugin validate plugin` and `agy plugin install ./plugin`, `HOME` pointed at an absolute scratch directory, since `agy` refuses a relative one, then `agy plugin list`, which counts the skills and the hook. Where `agy` is signed in, a vendored scratch repository holding a copy of the package under `.agents/plugins/checkwright/` is opened with `agy -p`. There the shell guard denies a `cat` of a tracked file, `workflow-state-guard` denies a `write_file` to the state file, and both stay silent in an empty repository. A scratch `HOME` carries no login, so the session runs in the signed-in home with the package workspace-local, and writes nothing to that home's configuration.

**`plugin/SPEC.md` §Honest limits**: the bullet "Antigravity, Cursor and Muse Code run no guard from this package." becomes "Cursor and Muse Code run no guard from this package, and Antigravity none on native Windows." Two bullets are added:

> - Under Antigravity the guards grant nothing and advise nothing: a grant becomes no decision, since its safety rests on the shipped binding's matcher, and an advise has no channel (gate-sdk/SPEC.md §The harness-integration arm). A workspace path whose JSON spelling carries an escaped quote or backslash defeats the prefix's `sed` read and leaves that session unguarded.
> - Whether an installed copy's hooks fire is inferred from the workspace copy's, which share Antigravity's plugin loader. While the CLI's Agent Plugins flag is on, a package declaring the Agent Plugins `$schema` may load without its hooks, since that standard excludes hooks.

**`plugin/README.md`**, line 27's second sentence becomes: "Codex registers the skills and no guard; Antigravity runs the shell and state-file guards, except on native Windows."

**The crate**:
- `native/src/gates/plugin_parity.rs` gains the Antigravity rendering: the prefix and suffix as `fn`s beside `fail_open()`, and the tool-to-spelling map read from `antigravity.rs`. Assertion (C) compares the second file and reds it with the expected rendering.
- `plugin/hooks.json` is generated from the rendering, never hand-written.
- The fixture pair `scripts/gate-tests/check-plugin-parity/` gains `plugin/hooks.json` in `good/`. In `bad/` the file carries a stale matcher, and `expect.txt` gains its line.
- The gate's positional form is unchanged, since it reads `plugin-dir`.
- `scripts/check-plugin-parity.gate`'s `couples=` already holds `plugin/`.

### (5) The Windows posture is measured where a Windows host runs {design-bearing}

**Not yet applied.** `native/src/gates/plugin_parity.rs` gains a `#[cfg(windows)]` unit test. It composes the Antigravity command for one member and spawns `cmd /c` on it, the command line composed as Go's `syscall.EscapeArg` composes one argument. It writes a `run_command` payload to stdin and asserts exit 0 and an empty stdout. `crate-tests-windows` runs it at the closing push.

**Both acts, on its verdict there:**
- **Green**: the posture stands as delta 4 states it.
- **Red**: the closing session withdraws `plugin/hooks.json` and assertion (C)'s second half in its hotfix push. It restores "Antigravity, Cursor and Muse Code run no guard from this package." and files the polyglot as a gap, because a rendering Windows `cmd` fails would fail every matched call there.

**Inferred, cannot run before build:** that `agy` composes `cmd /c` with Go's default argument escaping, since the hooks page names `cmd /c` and nothing more — no Windows host runs here, and the test that measures it is one build lands.

### (6) The harness-literal roster carries the second binding {mechanical}

**Not yet applied.** `scripts/gate-sdk-config.knobs`' `GATE_SDK_HARNESS_LITERALS` gains `toolCall CommandLine workspacePaths conversationId run_command`. Delta 1's rows carry the `harness-binding:` valve. The build runs `check-harness-literal` and moves any other red site's prose to the role.

### (7) The release declaration and the mirrors {mechanical}

**Not yet applied.** `.workflow/release-declarations.md`, under **Behavior changes**, gains:

> - **`--hook <member> --binding antigravity`** — the gate binary's hook arm reaches its before-tool-call members from Antigravity through an adapter, and the plugin package now carries `hooks.json` at its root, so installing it in Antigravity runs the shell and state-file guards. Nothing to do; to keep Antigravity unguarded, set `"enabled": false` on the `checkwright` hook in the installed copy's `hooks.json`.

The docs mirror regenerates: `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write`.

## Producers and consumers

- **`plugin/hooks.json`** (delta 4). Its producer is the parity gate's rendering, written by the build. `agy plugin install` copies it with the package, and its consumer is Antigravity's runtime plugin loader. At spec, `claude plugin validate` printed the same two warnings and the same pass over a copy of `plugin/` with the root file as over `plugin/` itself. Codex's documentation names `hooks/hooks.json` alone, and delta 4's validation leg runs the Codex install over the changed package, so what Codex makes of the root file is read there.
- **`--binding antigravity`** (deltas 2 and 3). Its producer is the rendered command, its one caller. Its consumer is the arm's dispatch in `hook/mod.rs`. Readers of a registration's tokens, each run through the one parser:
  - `check-settings-paths` reads a settings file, which never carries the rendering. Its red condition is unchanged.
  - the enforcement map reads the template, which carries no binding token;
  - `check-plugin-parity`'s rendering is the producer.
- **The translated payload** (delta 3). Its producer is `translate_payload` and its consumer the child member. Each field has its reader:
  - `hook_event_name` is read by `hook::decline_envelope`;
  - `tool_name` and `tool_input.command` by the shell guard;
  - `tool_input.file_path` by `workflow-state-guard`, through `SPEC-patch-paths.md`'s reader where that has landed;
  - No `session_id` or `transcript_path` is produced: their readers, the wakeup guard, the session-context member and the delegation-kit members, are members the rendering does not reach.
  - `cwd` by `SPEC-patch-paths.md`'s relative-path join, and by nothing before it lands. That stays harmless, since a member ignores an unread key.
  - No `agent_id` or `agent_type` is produced, so `workflow-state-guard`'s second and third rules exit 0 at their first test. Antigravity's subagents carry no child marker this binding reads.
- **The deny envelope** (delta 3). Its producer is `translate_verdict`, and its consumer Antigravity, which shows the reason to the model.
- **Roster-holding readers, each red condition named:**
  - `binding.rs`'s table test reds a row it does not hold. It splits the shipped table between its heading and **The measured record.**, and the new table sits after both, so the split is unchanged; delta 3's test holds that.
  - `check-harness-literal` reds a configured literal outside a valved line. Delta 6 adds the new literals and delta 1 valves the table rows. Any other site the build's run reds is reworded to the role.
  - `check-plugin-parity` (C) reds a stale `plugin/hooks.json`. It is generated in the same commit.
  - `check-md-refs` reds a dead anchor. Delta 4 adds none.
  - `check-docs-mirror-fresh` reds until the mirror regenerates (delta 7).
  - `check-release-declaration-parity` holds the next composed note to delta 7's bullet.
  - `check-amendment-queue` reds a `[spec:]` ref resolving to no file. The paired entry carries this file's name.
- **Members, each one's satisfying value** (point 6), enumerated from `guard-kit/templates/settings-hooks.json`'s groups by `jq '.hooks.PreToolUse[].matcher'`:
  - `Bash|PowerShell` renders `run_command`, which carries `shell-guard`;
  - `ScheduleWakeup|CronCreate` has no Antigravity tool and is left out;
  - `Write|Edit` renders the four file-write tools, which carry `workflow-state-guard`.

## Existing sections updated

- `gate-sdk/SPEC.md` §The hook contract, its opening paragraph and a new table after **Honest limits.** (delta 1).
- `gate-sdk/SPEC.md` §The harness-integration arm (delta 2).
- `native/src/hook/antigravity.rs` and `native/src/hook/mod.rs` (delta 3).
- `plugin/SPEC.md` §The harnesses, §The guards, §check-plugin-parity, §The validation leg and §Honest limits, and `plugin/README.md` (delta 4).
- `native/src/gates/plugin_parity.rs`, `plugin/hooks.json` and `scripts/gate-tests/check-plugin-parity/` (deltas 4 and 5).
- `scripts/gate-sdk-config.knobs` (delta 6).
- `.workflow/release-declarations.md` and the docs mirror (delta 7).

Roster probe: `git grep -n -i "antigravity"` over the tracked tree, the queue and `.workflow/` aside, names `plugin/SPEC.md` and `plugin/README.md` alone, every site of which delta 4 rewrites; the docs mirror copies them.

## Retired spellings

- None — no delta retires a spelling; "not wired" leaves the Antigravity row, and the Codex and Cursor rows keep the phrase.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the root (`ls SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **The live leg** — delta 4's signed-in session run, or, where `agy` is not signed in, the Antigravity hooks cell left at read and that stated.
