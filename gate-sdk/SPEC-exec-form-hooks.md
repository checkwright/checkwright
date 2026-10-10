# SPEC amendment: exec-form-hooks

Every hook registration the kits ship is a shell-form `command` led by the word `bash`, so a native-Windows adopter whose profile carries context-kit or guard-kit owes Git for Windows' bash for the hook wiring alone. The harness also takes an **exec form**: `command` names an executable and a sibling `args` array carries its arguments, with no shell between. Operator-run on Windows 11 and run on Linux, harness 2.1.292: the exec form started the binary with its `args` and the project-directory placeholder substituted; a block refused the call and an advisory reached the session; a suffix-less path started the file carrying the executable suffix; and with the binary renamed away the call proceeded on a failed direct spawn. This amendment moves the shipped registrations to the exec form and has the `--hook` arm do what the front end did for it.

**It reverses a refusal §run-gates records** (operator direction, 2026-10-10, lead-relayed, not a ruling; the refusing sentence and the fresh-clone consequence were both put to the operator). That section refuses wiring a harness to the binary because an absent binary then declines through the harness's one notice where the front end prints its remedy on every call. The quiet state is accepted as the price of the Windows floor, and delta 4 rewrites the passage.

## What changes

### (1) The shipped registrations take the exec form

Every hook registration a kit ships or publishes names the gate binary as `command` and carries `--hook` and the member in `args` {design-bearing} {user-facing: the entry's Deliverable for the form; operator direction, 2026-10-10, lead-relayed, that it is the one shipped wiring; lead decision of that date that a template names the binary by the existing placeholder}.

The registration, as a kit template spells it. **Not yet applied.**

```json
{ "type": "command", "command": "${CLAUDE_PROJECT_DIR}/@GATE_SDK_NATIVE_BIN@", "args": ["--hook", "shell-guard"] }
```

- **The binary is spelled as the placeholder `@GATE_SDK_NATIVE_BIN@`**, the one guard-kit's allowlist template already carries and on its grounds (guard-kit/SPEC.md §The recommended allowlist): the knob's value is per consumer, and guard-kit/SPEC.md §check-door-binding's assertion B, which stays as written, bars a path to the binary from a kit template. The adopter resolves it at the merge to the knob's value as a path from the repository root, **with no executable suffix**, so one committed settings file serves a Windows and a POSIX clone alike. A value that is absolute replaces the whole `command`, the project-directory prefix with it.
- **An unresolved placeholder is a guard that is off**, where in an allow entry it only costs a prompt. `check-settings-paths` reds it where it is registered (delta 3), and each template's `//` note says to resolve it and how to print the value.
- **The `//` note names the harness's `onFailure` field** as the adopter's own choice for a hook that should refuse when its program cannot start.

The note states the field at the tier it was read at: the harness's published changelog, fetched at authoring, adds `onFailure: "block"` under 2.1.295, *a hook that can't start … blocks the action*. No session ran under it here, and the note says the adopter tries it before relying on it.
- **`shell` and an interpreter word leave the registration.** The exec form starts no shell, so the path needs no quoting and the matcher groups are untouched.

**The members and each one's value**, by `git grep -n -E -e '--hook [a-z<]|--statusline' -- ':!docs/*/SPEC.md' ':!TASK-QUEUE.md' ':!*/gate-tests/*' ':!scripts/gate-tests/*' ':!native/src/*'`:

- `guard-kit/templates/settings-hooks.json`, three hooks, and `context-kit/templates/settings-sessionstart.json`, one: the registration above with its own member.
- The README wiring steps of guard-kit, delegation-kit (the budget guard and the turn-end hook) and lifecycle-kit, and the SPEC sentences quoting a wiring (guard-kit/SPEC.md §The shell guard, context-kit/SPEC.md §The session-context hook, delegation-kit/SPEC.md §Layout and configuration, lifecycle-kit/SPEC.md §check-stage-evidence): the binary `GATE_SDK_NATIVE_BIN` names under `<project-dir>`, as `command`, with `--hook <member>` as `args`.
- **Narrowed past, each with no exec form to take.** The status line (delegation-kit's README step and this repo's own setting) stays a shell-form `command` through the front end: the harness documents no `args` for that setting. `plugin/hooks/hooks.json` stays shell form, since each of its commands must exit 0 in a repository that never installed the kits, which takes a shell test (delta 5).
- **The front end keeps the `--hook` arm.** A consumer's already-merged shell-form wiring, the plugin and the status line reach it, so `FAIL_OPEN_ARMS` and both stubs are unchanged.

### (2) The `--hook` arm resolves the tree it runs in

Started with no front end, the arm resolves the repository root itself and declines where there is none it may read {design-bearing} {user-facing: the entry's Deliverable — the refused-repository decline moves into the `--hook` arm; the root resolution is what that decline already needs and changes nothing for a call through the front end}.

- **The root.** Before any knob is read the arm asks git for the toplevel of its working directory, through the crate's one crosser, and runs the member there. Run on this tree: the arm started directly reads a tree's guard knob file from the root and misses it from a subdirectory, exiting 0 with no word, so without this step a session's `cd` would switch a consumer's configuration off.
- **The decline.** Where git refuses the repository, skips a nested one, or answers none, the arm exits 0 and writes the `systemMessage` envelope the front end writes for that case, its path-bearing diagnostic on stderr (§The harness-integration arm). The front end keeps its own copy for the callers that reach it, since there the stub runs no binary.
- **Through the front end nothing changes**: the arm is already at the toplevel, and resolving it again answers the same directory.
- **A linked worktree.** The placeholder names the root the session started in, so a session that entered a worktree, an isolated child among them, starts the main checkout's binary with its working directory in the worktree, and the arm reads the worktree's own tree. That is what `gate_harness_bin` arranges for a front-end call. Run on Linux, harness 2.1.292: in a linked worktree the placeholder named the main checkout.

**The honest limit, stated in §The harness-integration arm:** a session *launched* inside a linked worktree is not run. The harness reference defines the placeholder as the root the session started in, which there would name the worktree, where no binary is built, leaving its guards off behind the harness's one notice. No delta rests on it.
- **The gate-sdk root locator** takes its default on a direct spawn, as §Layout and configuration already states for a harness spawning the binary directly.

### (3) The registration grammar reads the binary

The registration parser takes the gate binary as a command token beside the two front-end names, and `check-settings-paths` resolves a suffix-less hook candidate {design-bearing}.

- **The parser** (§The harness-integration arm, *the registration grammar has one parser*): a command token whose file name is a front end's, or the binary's own — the crate's package name, with or without an executable suffix — then `--hook`, then the member. It is the name assertion B of guard-kit's `check-door-binding` already derives, so the two cannot drift. A consumer who renamed the binary gets no member check, a stated limit.
- **`check-settings-paths`** (context-kit/SPEC.md §check-settings-paths): a hook candidate resolves when its path is a file, or, where its file name carries no extension, when the path with the host's executable suffix is one. An unresolved placeholder is a candidate that resolves to nothing, and its finding's `help:` names the merge step. The fixture pair gains an exec-form binary registration in `good/`, and in `bad/` an unresolved placeholder and an exec-form `--hook` operand the binary does not carry, each moving the pinned counts.
- **The enforcement map** reads registrations through the same functions (§enforcement-map), so its rows keep their members and `docs/enforcement.md` is regenerated where a row's printed surface moves.

### (4) The sections that say a harness-wired arm keeps a shell door

The canonical text that makes the front end a harness's permanent door is rewritten to the narrower fact {design-bearing}.

Replacement for §run-gates' two paragraphs opening *The front-end serves a harness shim and a pre-build clone rather than an adopter door* and *A harness cannot be wired to a binary that fail-opens on its own absence, and the residency is permanent*. **Not yet applied.**

> **The front-end serves a harness value only a shell can run, and a pre-build clone, rather than an adopter door.** Every kit README, template, knob header and stage procedure names the binary `GATE_SDK_NATIVE_BIN` names, the hook registrations among them, in the harness's exec form (§The harness-integration arm). The stub keeps two harness callers: a setting the harness runs only through a shell, the status line, and a wiring that must test for the kits before it runs, the plugin's. The other caller is a contributor clone before its first build, which has no binary to name.
>
> **An exec-form registration declines through the harness when its binary is absent.** An absent program does not run, so nothing of this project's speaks for it: the harness reports a hook that could not start once and lets each call through. The front end's decline carried the remedy on every call, and that is given up for a wiring that needs no shell. A consumer who relocates the binary re-points the registration, which follows no knob, and `check-settings-paths` reds one that no longer resolves.

- **§The adopter constraints**, the script-door sentence: *A **harness-configuration value carrying a fail-open arm** keeps one by construction, since its status is read exactly when the binary that would answer is absent* becomes a **harness-configuration value the harness can run only through a shell**, with the status line and the plugin's rendering named as that class's members by their kind.
- **§The harness-integration arm**, the fail-open paragraph: the front end's exit 0 and envelope are stated for a call that reaches the arm through it, and the exec-form registration's two declines are stated beside them — the arm's own (delta 2) and the harness's, for an absent binary.
- **guard-kit/SPEC.md §check-door-binding**, *The exemption keys on the arm, never on the file*: the fail-open arms no longer name the front end *by construction*; the exemption stays corpus-wide for the shell-form registrations that remain. Assertion B is untouched, and the placeholder carries no path for it to read.
- **guard-kit/SPEC.md §The recommended allowlist** and `templates/settings-allow.json`'s `//` note: the front-end pair is granted for a pre-build clone and for a session running the front end, no longer because the kit's hook wiring spawns it.
- **guard-kit/SPEC.md §The shell guard** and **§The hook on native Windows**: the member is reached by a direct spawn; the anchor paragraph keeps the project-directory binding and drops the relative-spelling `127` case and the inferred bash-opens-the-path sentence, neither having a shell left to occur in; the hook's floor on Windows is the binary alone.
- **The no-bash host is a stated limit** (operator direction, 2026-10-10, lead-relayed), in §The hook on native Windows: the Windows runs were on a host carrying Git for Windows, where the harness still found its bash for its own shell tool, and a host with no bash at all is not run.
- **context-kit/SPEC.md §The session-context hook**: *through the front end* and *It runs where the front end put it* become the direct spawn and the toplevel the arm resolved.
- **context-kit/SPEC.md §bin/env-probe** is unchanged: its settings-template arm keys on a `bash` word, and no shipped template carries one after delta 1, so both kits leave the bash audience by derivation. `docs/requirements.md`'s toolchain row then reads as the derivation prints it.

### (5) The plugin's hooks are rendered from the template's members

`check-plugin-parity`'s rendering composes each plugin command from the template hook's member, where it prefixed the template's command string {design-bearing}.

- **The rendering** (plugin/SPEC.md §The guards): for each template hook, read through the registration parser, the plugin's command is the fail-open prefix followed by the shell-form front-end call of that member, `bash "${CLAUDE_PROJECT_DIR}/gate-sdk/bin/run-gates.sh" --hook <member>`. `plugin/hooks/hooks.json` therefore keeps the bytes it has, and the gate's fixture templates take the exec form.
- **A template hook that is no member registration** fails the rendering at exit 2, since the plugin has no command to give it.
- **The plugin still owes bash**, on every host, and plugin/SPEC.md §The guards keeps saying so.

### (6) The smokes run the wiring as the harness does

Each kit smoke that drives a merged registration spawns its `command` with its `args`, the placeholder resolved the way an adopter resolves it {mechanical}.

- `guard-kit/smoke/install.sh` selects the shell-guard group by `args`, and replaces its `bash -c "$wired"` run from a subdirectory with a direct spawn of the resolved `command` and `args` under the project-directory variable. That run is the executable oracle for delta 2's root step.
- `context-kit/smoke/install.sh` merges its template too, and resolves the placeholder at the merge: the kit's own `check-settings-paths` reads the merged file and reds an unresolved one.
- `context-kit/smoke/install.sh` and `delegation-kit/smoke/install.sh` drive their members through the front end, by `git grep -n -e '--hook' -- '*/smoke/*'`, and run no merged wiring string. Each keeps its call, the arm staying on the front end.
- The suites that assert a front-end `--hook` call, `gate-sdk/gate-tests/run-gates-linked-worktree.test.sh` and the front-end parity corpus, keep it: the arm stays on the front end.

### (7) This repo's own registrations

The six hook registrations in `.claude/settings.json` take the exec form, `command` naming `${CLAUDE_PROJECT_DIR}/native/target/release/checkwright-gates`; the status line is untouched {mechanical} {user-facing: the entry — the edit waits for the operator's confirmation of the prepared diff, owed when the diff exists and not yet given}.

- **Build prepares the diff and applies nothing.** A permission-settings edit is applied on the operator's behalf, and this one changes what guards every call in the tree.
- **The witness runs first**, on a matcher no session uses and a scratch copy of the binary: the bare binary exits 2, run on this tree, so a harness that dropped `args` would block every call a matcher takes.
- **Order.** Deltas 2 and 3 land before the diff is applied, or `check-settings-paths` stops checking the six members and a `cd` switches the guards' configuration off.

### (8) The harness floor

`docs/requirements.md` §Installing and running the shipped gates states the harness version the hook wiring needs, outside its two generated blocks, since the harness is neither a tool `doctor` probes nor an install prerequisite {mechanical} {user-facing: operator direction, 2026-10-10, lead-relayed, not a ruling — a minimum harness version stated on the requirements page, marked inferred until a witness runs}.

The floor is 2.1.139, read from the harness's published changelog, fetched at authoring: that version's entry adds *hook `args: string[]` field (exec form)*. The page states it at that tier, as the changelog's and not as run here: the entry's runs were on 2.1.292, this seat runs 2.1.296, and no build below the floor was run. What such a build does with a registration carrying `args` is undocumented, which is the hazard delta 7's witness guards against on this tree, and the page says a session on an older build must not merge the wiring.

### (9) The Windows assertions

The Windows leg holds what no local run reaches {mechanical}.

- The crate tests and the fixture pair of delta 3 run there, so a suffix-less candidate resolves against the file carrying the suffix on that host.
- A step starts the placed binary as `--hook shell-guard` from a subdirectory of a scratch repository carrying a malformed guard knob file, and reads the refusal, delta 2's root step on that host.
- **With `windows-running-hook-replace` in the same build batch** the assertions ride its mid-iteration push. **Without it** this unit owes that push itself, inside the budget by the operator's grant on that entry's Push need line.
- The linked-worktree placeholder on Windows and same-session arming stay unrun, and §The hook on native Windows says so beside the no-bash host.

## Producers and consumers

- **The exec-form registration.** Producer: the adopter's merge of a kit template into the settings file, and delta 7's edit here. Consumers: the harness, which spawns it; `check-settings-paths` and the enforcement map, through the one parser (delta 3); `check-plugin-parity`, which renders from the guard-kit template (delta 5); the tool-floor derivation, which reads a template's first command word and finds no `bash`.
- **Each field's reader.** `command`: the harness, and `check-settings-paths`' path resolution. `args`: the harness, and the parser's member read.
- **The arm's root resolution and decline.** Producer: the arm, on every direct spawn. Consumers: every hook member, which reads its knobs from the tree it now stands in; the session and the operator, through the envelope.
- **Rosters holding a registration's shape**, each named in a delta: the parser's front-end name set, `check-settings-paths`' fixture pair, `check-plugin-parity`'s rendering and fixture templates, `check-door-binding`'s fail-open exemption, the tool floor's template arm.
- **Narrowing (point 5).** Delta 1 removes the `bash` word from every shipped template. The tool-floor derivation is the reader with a non-monotone verdict: the bash member's audience shrinks, and `check-install-toolchain` reds until `docs/requirements.md`'s row is what the derivation prints. Its fixture templates carry their own hooks and are not narrowed. No reader asserts a count of shell-form registrations.
- **Every member's value (point 6).** Enumerated under delta 1 with the probe that produced the roster, the two members with no exec form narrowed past there.

## Existing sections updated

- `gate-sdk/SPEC.md` §run-gates — the two paragraphs (delta 4).
- `gate-sdk/SPEC.md` §The adopter constraints — the first script-door class (delta 4).
- `gate-sdk/SPEC.md` §The harness-integration arm — the fail-open paragraph, the arm's root step and decline, the parser's command token (deltas 2, 3 and 4).
- `guard-kit/SPEC.md` §The shell guard, §The hook on native Windows, §check-door-binding, §The recommended allowlist, §Testing — the wiring, the floor and its limits, the exemption's ground, the front-end grant's ground, the smoke's wiring run (deltas 1, 4, 6 and 9).
- `context-kit/SPEC.md` §The session-context hook, §check-settings-paths — the wiring and the suffix-less candidate (deltas 3 and 4).
- `delegation-kit/SPEC.md` §Layout and configuration, `lifecycle-kit/SPEC.md` §check-stage-evidence — the quoted wirings (delta 1).
- `plugin/SPEC.md` §The guards — the rendering (delta 5).
- `guard-kit/templates/settings-hooks.json`, `context-kit/templates/settings-sessionstart.json`, `guard-kit/templates/settings-allow.json` — the registrations and the notes (deltas 1 and 4).
- `guard-kit/README.md`, `delegation-kit/README.md`, `lifecycle-kit/README.md` — the wiring steps (delta 1).
- `native/src/hook/mod.rs`, `native/src/gates/settings_paths.rs`, `native/src/gates/plugin_parity.rs` — the arm, the parser, the candidate and the rendering (deltas 2, 3 and 5).
- `context-kit/gate-tests/check-settings-paths/`, `scripts/gate-tests/check-plugin-parity/` — the fixture pairs (deltas 3 and 5).
- `guard-kit/smoke/install.sh`, `context-kit/smoke/install.sh` — the placeholder's resolution at the merge and the wiring run (delta 6).
- `.claude/settings.json` — six registrations, on the operator's confirmation (delta 7).
- `docs/requirements.md` — the harness floor and the regenerated toolchain row (deltas 1 and 8).
- `docs/enforcement.md` — regenerated where a row moves (delta 3).
- `.github/workflows/` — the Windows leg's step (delta 9).
- `docs/gate-sdk/SPEC.md`, `docs/guard-kit/`, `docs/context-kit/`, `docs/delegation-kit/`, `docs/lifecycle-kit/` — generated mirrors, stale once any delta lands (all deltas).
- `.workflow/release-declarations.md` — a bullet: the templates changed form, an already-merged shell-form wiring keeps working, and re-merging takes the placeholder step and the harness floor (deltas 1 and 8).

## Retired spellings

- None — the shell-form registration stays a served spelling: the front end keeps the arm, and the plugin and the status line keep the form.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls gate-sdk/SPEC-*.md`), discharged at the iteration.
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Entry moved** — its oracle is a remote run, so the entry moves to Done when build's remote-oracle rule says, ahead of the drain stage, and never in its merge commit; delta 7 stays open on the entry until the operator confirms the diff.
