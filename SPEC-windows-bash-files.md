# SPEC amendment: the kit-shipped bash files

Pairs `windows-kit-bash-files`. Sited at the repository root because it changes gate-sdk's, guard-kit's, context-kit's, drift-kit's and lifecycle-kit's contracts together.

**Directions this amendment executes** (operator direction 2026-10-06, lead-relayed, none a ruling):

- **The wiring.** Rewire the shipped harness hook wiring to the exec form naming the binary directly, now. The option was chosen with its costs stated: it retires the front end's fail-open stub for that wiring, it contradicts gate-sdk/SPEC.md §The adopter constraints' sentence that a fail-open harness value keeps a script door permanently, it rests on harness facts nobody has run, and it needs a minimum harness version. The amendment owes a way for those facts to be witnessed before the wiring is relied on; delta 1 is that way.
- **`session-context.sh`.** Port it to a `--hook session-context` member, its `[EDIT ME]` gaps moving to context-kit config.
- **`kpi-deprecated-surface.sh`.** Port it to a built-in drift-kit member, reversing the file's recorded `# no-port:` declaration.
- **The consult-inbox line.** Given at align, on the audit's question: mint a consult-count arm in this entry and register it as a brief command, so the line this repository's copy printed stays in the brief. It widens the entry by one lifecycle-kit arm; delta 5 carries it.

**What the survey established, which the entry's own re-verification did not.** The bash row's kit list is derived (context-kit/SPEC.md §bin/env-probe). guard-kit is in it through its settings template, whose three hook commands lead with a `bash` word, and through no tracked bash file. context-kit is in it twice, by its template's shebang and by its settings template. So a kit leaves the row when its wiring stops spawning bash, and porting a file alone moves only drift-kit.

## What changes

### (1) A witness of the exec-form hook, before any wiring rests on it

The build's first act on this amendment is to run the harness facts deltas 2 and 3 rest on, on a unix host and on a native Windows host, and to record what each printed. {design-bearing}

**The facts, each one observation.**

- **(a)** A `type: command` hook carrying an `args` array is started with no shell, and a leading project-directory placeholder in `command` is substituted by the harness.
- **(b)** A member's block, exit 2 with its text on stderr, still refuses the tool call, and an advise envelope still reaches the session.
- **(c)** With the named binary renamed away, the harness reports a hook that could not start and lets the call proceed.
- **(d)** In a session whose working directory is a linked worktree, the placeholder still names the directory holding the binary.
- **(e)** On Windows, a `command` path written with no executable suffix starts the file carrying one.
- **(f)** On Windows, (a) through (c) hold with no bash on `PATH`.

**Inferred, cannot run before build:** each of (a) through (f) — they were relayed from a documentation lookup read through a summarizer, and their subject is a registration no tracked settings file carries until this delta's diff is applied.

**How it is run.** The session prepares a diff adding exec-form registrations of existing members beside the standing shell-form ones, and the operator applies it: the settings file is operator-owned (guard-kit/SPEC.md §compare-settings-allow), and a `hooks[]` edit arms in the running session. The unix half runs in that session. The Windows half needs a native Windows host running the harness; no workflow leg runs a harness, so it is an operator-observed run. The witness names the harness version it ran on, which becomes the stated minimum of delta 2.

**The registrations, settled at build.** The binary given no argument exits 2, so a harness that dropped `args` would block every call the matcher takes. The witness therefore rides two matchers no session here uses, in the untracked local settings overlay, each naming a scratch copy of the binary with no executable suffix:

- `agent-budget-guard` on `NotebookEdit`, which advises whatever the payload, its `args` trailing `;`, `touch` and a marker path: a marker that appears means a shell read them.
- `wakeup-guard` on `EnterWorktree`, which blocks whatever the payload and logs the attempt.

Fact (c) renames the copy, so no standing hook is disturbed. Fact (d) repeats the first call from a session in a linked worktree, where the copy does not exist. The overlay's `hooks` key is removed once both halves are recorded.

**Who supplies the Windows half: nobody this iteration** (operator direction 2026-10-07, lead-relayed, not a ruling). The Windows half stays unrun, so the gate below takes its second branch as written: deltas 2 and 3 do not land, and guard-kit and context-kit stay in the bash row. The overlay block was applied by the lead on an operator grant of 2026-10-07, and the lead removes it once the unix half is recorded.

**What lands this batch** (lead decision 2026-10-07): deltas 4, 5, 6 and 8 with the member wired through the front end, delta 7 in its narrower result, then the iteration's push. The unix half is run and recorded all the same, as evidence the deferred rewire will need. The queue disposition of deltas 2 and 3 goes back to the lead as an escalation, and no queue entry is written for them before it is ruled.

**The gate it sets.**

- **Every fact holds on both hosts:** deltas 2 and 3 land, and delta 4's wiring takes the exec form.
- **A fact fails, or the Windows half has not been run when the batch is cut:** deltas 2 and 3 do not land. Delta 4's member is wired through the front end, as every hook is today. Delta 7 lands in its narrower result. The build escalates the queue disposition of deltas 2 and 3 to the lead, stating which fact failed or was not run.

### (2) Shipped hook wiring takes the exec form and names the binary

Every hook registration a kit ships or publishes for an adopter to paste becomes `command` naming the gate binary under the project-directory placeholder, with `args` carrying `--hook` and the member. {design-bearing} {user-facing: operator direction 2026-10-06, lead-relayed — rewire the shipped templates to the exec form naming the binary directly}

- **The path is the installed binary's default place**, the gates directory's default under the placeholder, written with no executable suffix. A template's note tells an adopter with another layout to retarget it, as the session-start template's note does.
- **One committed wiring serves every host**, as now. guard-kit/SPEC.md §The hook on native Windows is rewritten to what delta 1 observed: the hook's floor is the gate binary and the harness, on every host, and the harness version witnessed is the minimum.
- **A host with no Git Bash** is claimed only if fact (f) held there; otherwise that section keeps the host unclaimed and says which fact was not observed.
- **The registration grammar gains the binary as a command token.** The one parser beside the member table reads `--hook <member>` after a command token that is the front end, as now, or the path `GATE_SDK_NATIVE_BIN` names, with the placeholder stripped and the host's executable suffix optional. `check-settings-paths` and the enforcement map read the result unchanged, and each declares `GATE_SDK_NATIVE_BIN` on its own knob roster, the parser now resolving it.
- **`check-settings-paths` resolves a suffix-less hook candidate** against the path as written and against it with the host's executable suffix, since one committed path names a file whose name differs by host.
- **The plugin's hook wiring keeps the shell form.** It is installed outside any one repository, so its command first looks for a repository that carries the front end and exits 0 where there is none; an exec form has no such branch and would report a failed start on every tool call in a repository that never installed the kits. plugin/SPEC.md §The guards' rendering is restated over the exec-form template: from each entry it takes the event, the matcher and the member its `args` name, and writes the shell-form command, the lookup prefix then the front end with `--hook` and that member. `check-plugin-parity` assertion C still compares the plugin file to that rendering as JSON, so the plugin file's bytes do not move; the rendering function and its unit test do.
- **This repository's own settings file** takes the same rewrite as a prepared diff the operator applies.

### (3) The cannot-run branches move behind the binary, and the script-door class narrows

The fail-open behaviour the front end gave shipped wiring is restated as what the harness and the binary do between them. {design-bearing} {user-facing: operator direction 2026-10-06, lead-relayed — retiring the fail-open stub for shipped wiring, and reversing the adopter-constraints sentence that such a value keeps a script door permanently}

- **An absent binary is the harness's failed start**, fact (c): the call proceeds and the harness shows its own notice. The fixed `systemMessage` envelope naming the remedy is no longer written for exec-form wiring, since nothing runs to write it.
- **A repository git refuses or skips** is seen inside the binary. The `--hook` arm declines through `hook::decline` with the text the front end's envelope carried, naming the two `git status` remedies.
- **The linked-worktree dispatch needs no arm**: the placeholder names the session's project directory, fact (d), which is the checkout holding the binary.
- **The front end keeps its fail-open set unchanged**, `--hook` included, because the plugin's shell-form wiring and the status line still reach it. `check-front-end-fail-open` and the crate's declaration are untouched.
- **gate-sdk/SPEC.md §The adopter constraints**: the first permanent script-door class narrows from *a harness-configuration value carrying a fail-open arm* to *a harness-configuration value the harness can only run through a shell*, which today is the plugin's lookup and the status line. guard-kit/SPEC.md §check-door-binding's exemption sentence is restated to the same class.

### (4) `session-context` becomes a hook member

The session brief is assembled by a `--hook session-context` member context-kit owns, and the shell template, its `<gates-dir>/` copy and their parity pair are deleted. {design-bearing} {user-facing: operator direction 2026-10-06, lead-relayed — port to a `--hook session-context` arm; step 8's narrowing below is a lead decision of 2026-10-06, the operator having delegated the call, revisable at a later scope or spec}

- **The section keeps its subject and drops its qualifier**: context-kit/SPEC.md §The session-context hook (template) becomes §The session-context hook. A citation spelled without the qualifier resolves as before. The qualified spelling and the anchor derived from it have two readers: gate-sdk/SPEC.md §The harness-template port disposition, which delta 8 rewrites, and the enforcement map's owner table, which takes the new anchor.

- **The ten steps, their order, their guards and their printed strings are kept**, with the cursor-lag rule and the session-role signal. The member never exits non-zero and an unparseable payload reads as an absent role signal.
- **Its stdout is the brief**, the text the session-start integration point reads; it writes no envelope. A knob that cannot resolve is the one exception: the member takes `hook::decline`, whose stdout is the `systemMessage` envelope a non-`PreToolUse` event gets, and prints no brief.
- **The binary guards vanish**: every step that tested for the binary before printing now runs inside it, so the *absent rather than empty* ordering rule has no subject and is deleted from the section.
- **Steps that spawned a sibling arm call it in process or re-exec this binary**, on §run-gates' rule for a member. Steps 5 and 6 read the tree with the crate's own walk, and the session-role read parses the payload in process, so the member spawns no `find`, `grep`, `sed`, `head` or `cat`; `git` and a configured command are its only children.
- **Step 8 no longer spawns a shell.** `CONTEXT_KIT_STAGE_RULES` is split on blanks, the stage appended as the last argument. A value whose first word begins `--` is run as arguments to this binary; any other is started as a program. A consumer value leading with an interpreter still runs, as that consumer's choice. **Honest limit:** the split honours no quoting, no pipeline and no expansion, so a value that relied on the shell is rewritten as one command or as a program the consumer keeps. doctrine-kit/SPEC.md §stage-rules' surfacing-seam paragraph, which states the seam as a command `bash` runs, is restated to this rule.
- **`templates/settings-sessionstart.json`** registers the member, in the form delta 1's gate selects.
- **This repository's copy** is deleted with the template, and its settings registration is rewritten by a prepared diff the operator applies.
- installer/SPEC.md §The update notice loses its honest limit on a consumer copy made before the update-notice step existed, a member having no copy to lag.

### (5) The marked gaps become context-kit config

Each layout judgment the template marked `[EDIT ME]` is a knob of context-kit's table, defaulting to what the template did. {design-bearing} {user-facing: operator direction 2026-10-06, lead-relayed — the `[EDIT ME]` gaps moving to context-kit config}

The members were enumerated by `grep -n 'EDIT ME' context-kit/templates/session-context.sh` and by `diff` of the template against this repository's copy. Each one's satisfying value:

| what the adopter edited | where it goes |
|---|---|
| the library path and the probe path | nothing: the member is the binary |
| which directories are components, in step 2 | `CONTEXT_KIT_BRIEF_COMPONENT_MARK`, a directory name, default `src`; empty turns the step off |
| which stages draw which nudge, in step 4 | `CONTEXT_KIT_BRIEF_NUDGES`, keyed by stage, each value a tracked text file printed verbatim; default empty |
| the index footer, in step 7 | `CONTEXT_KIT_BRIEF_FOOTER_FILE`, a tracked text file printed verbatim; default empty, which prints the built-in footer naming the three index arms, its public-surface line spelled with the configured component mark and omitted when that mark is empty |
| a step the consumer added | `CONTEXT_KIT_BRIEF_COMMANDS`, indexed, each element run on step 8's rule after the drift line, its stdout printed with a blank line after it whatever status it exits, since a verdict arm reports through its status and its line is the point; silent when it prints nothing or cannot be started |

- **Three names the template read from the environment gain table rows** with their existing defaults: `CONTEXT_KIT_DRIFT_REPORT`, `CONTEXT_KIT_STAGE_RULES` and `CONTEXT_KIT_SESSION_ROLE_FILE`. A compiled member refuses a name its table does not declare. drift-kit's two statements that the trend knob's default lives in the template or the hook copy, in its README's wiring step and in drift-kit/SPEC.md §The report skeleton, are restated to the knob.
- **The delegation nudge ships as `templates/nudge-delegation.md`**, the exemplar a consumer copies and keys to its stages. The kit default prints no nudge.
- **This repository's copy held five values beyond its template**, and each moves to its context-kit knob file in the deleting commit:
  - the drift arm name and the stage-rules command, as those two knobs, the command written in its `--emit` form;
  - the reworded delegation nudge, as a nudge file keyed to the three stages it printed on;
  - the budget line, as a brief command running `--usage-verdict`, printed as that arm prints it and without the copy's label;
  - the consult-inbox count, as a brief command running the arm below (operator direction 2026-10-06, lead-relayed, not a ruling: mint the arm in this entry so the line stays in the brief). No value leaves.
- **A brief command takes no stage argument.** It is split and dispatched as step 8 splits and dispatches, and nothing is appended: an arm reading positionals, `--usage-verdict` among them, would take an appended stage as an operand.
- **lifecycle-kit gains `--emit consult-count`**, the one arm that reads the consult inbox for a count. {user-facing: operator direction 2026-10-06, lead-relayed — mint the consult-count arm now, registered as a brief command}
  - It reads `LIFECYCLE_KIT_CONSULT_INBOX_FILE` and declares that knob alone. It counts the bullets the inbox grammar defines (lifecycle-kit/SPEC.md §The consult inbox) and takes the oldest from the first bullet's date.
  - With one bullet or more it prints one line, `Consult inbox: <n> item(s) owed to /consult, oldest <date>.`, `undated` standing for a first bullet carrying no date. That is the line this repository's copy printed.
  - With no inbox file or no bullet it prints nothing. Both exit 0, so a brief command stays silent on an empty inbox by its own rule.
  - It takes no argument, and one is a usage refusal at exit 2. It writes nothing.
  - lifecycle-kit/SPEC.md §The consult inbox states it beside the filing affordance, and the kit README's command list gains its line.

### (6) `kpi-deprecated-surface` becomes a bundled member

The deprecated-surface trend is a built-in member of the drift report, and its shell template is deleted. {design-bearing} {user-facing: operator direction 2026-10-06, lead-relayed — port to a built-in drift-kit member, reversing the file's `# no-port:` declaration}

- **It reads `CANON_KIT_DEPRECATION_MARKERS` in process and counts marker lines over the comment surface canon-kit's `check-deprecation-task` scans**, through the crate function that gate calls (`spec::comment_surface`), so the trend and the gate cannot count different files.
- **Its rows and its trend fragment are the template's**, with the template's `n/a` row for an unset marker roster.
- **An unset surface roster takes whatever that shared function answers**, which is canon-kit's own default for the knob. The template's separate fallback, every shell file, is dropped, so the member mints no surface literal of its own.
- **The counted surface is therefore the gate's and not the template's glob expansion**: the shared function prunes what the gate prunes, so a tree's count can step at the port with no marker moving.
- **The template's `n/a (knob read failed)` row has no subject**, the knobs being read in process.
- **It joins `templates/kpis.list`**, which names every bundled member, so an adopter with no marker roster reads one `n/a` row until they prune it from their copy. The kit smoke's per-member row assertion gains its row.
- **This repository registers it in its own `kpis.list`**, because its release-sweep binding ran the template by path and no arm runs one member alone. The binding's inventory command becomes the drift report, read at its deprecated-surface row. The binding is hand-authored, so it is edited, not regenerated.
- **The reversal's ground.** The declaration held that porting would publish consumer rule content. The file holds none: the marker spellings and the surface arrive through two knobs, which is the consumer-config pattern gate-sdk/SPEC.md §The provenance seam names, and the one literal the file carried, its shell-file fallback, is dropped in favour of the default canon-kit already owns.
- drift-kit/SPEC.md §Out of scope loses its deprecated-surface paragraph to §Bundled KPIs, and §Layout and configuration loses the template from its listing and the sentence holding it out of the registry. canon-kit/SPEC.md §check-deprecation-task and gate-sdk/SPEC.md §The non-gate arm's `knob-values` paragraph each name the template as an example plugin and are restated to the member. A consumer file of the same name in a KPI dir still shadows the member, by §The extensibility contract.

### (7) The bash audience and its published row follow the derivation

The derivation is unchanged; what it reads has changed, and every surface stating its result is brought to it. {mechanical} {user-facing: operator direction 2026-10-05 on the entry — the toolchain row narrowed to what still owes bash, guard-kit's place in it included}

- **Where deltas 2 and 3 landed**, no kit root ships a bash surface under the two kit-root arms. docs/requirements.md's bash row names the two conditions that remain, a runnable fence in the adopter's docs and a registered shell gate, and its Why cell drops the kit-shipped file. The Git for Windows prerequisite row stops offering bash for a kit.
- **Where they did not**, context-kit and guard-kit stay, each by its settings template, and drift-kit leaves. The row names those two, and its Why cell says each ships hook wiring the harness runs with bash.
- The crate's unit test over the authoring tree's derived audience is restated to the result that landed.
- context-kit/SPEC.md §bin/env-probe's `bash` member loses the sentence naming the session template as a library reader, and installer/SPEC.md §Requirements' three bash bullets are restated to the result.
- **Two of those surfaces are shared with the sibling amendment, sentence by sentence.** In §Requirements' *What runs without `bash`* bullet this delta owns the closing sentence, on guard-kit's hook, and the sibling owns the sentences on the git hooks. In §bin/env-probe this delta owns the library-reader sentence and the sibling owns the sentence on the git hooks staying POSIX sh. Neither rewrites the other's sentence, so the two land in either order.
- The Git for Windows row's sh clause is the sibling entry `native-executable-git-hooks`'s. If that entry's deltas stand in the same batch the row is rewritten once, to both results; if they do not, this delta changes the bash clause alone.

### (8) The port dispositions are restated

gate-sdk/SPEC.md §The harness-template port disposition is rewritten to the declaring set that remains. {mechanical}

- `gate-sdk/templates/check-skeleton.sh` is the one declaring member, on the extension-point ground.
- The second ground keeps its statement and loses both instances: the session-context copy is deleted with its template, and `kpi-deprecated-surface.sh` is ported.
- The section's *what reopens it* paragraph records that the extension-point face dissolved for `session-context.sh` as it said it would, its gaps having left the template.
- §check-template-registry-parity's sentence excusing the example plugin from the registry is deleted, the member now being named there.

## Producers and consumers

- **The witness (delta 1).** Producer: the build session and, for the Windows half, the operator. Consumer: the build's own gate on deltas 2 and 3, and guard-kit/SPEC.md §The hook on native Windows, which states the result.
- **An exec-form registration (delta 2).** Producer: the kit templates, merged into the settings file by an adopter or a kit smoke. Consumers: the harness, by spawn; the registration parser, called by `check-settings-paths` and the enforcement map; `check-plugin-parity`, rendering the guard-kit template's event, matcher and member into the plugin's shell form.
- **The refused-repository decline (delta 3).** Producer: the `--hook` arm, on git's own answer. Consumer: the session or the operator through the envelope `hook::decline` chooses.
- **The `session-context` member (delta 4).** Producer: the harness's session-start event, enabled by the registration the session-start template ships. Consumer: the session, reading stdout. Roster-holding readers of the minted member name: the crate's member table and its owner test, and through it `check-settings-paths`, the enforcement map and the generated enforcement page.
- **Each minted knob (delta 5)** is read by the member at the one step its row names. Roster-holding readers: context-kit's table and its arm-knob test, context-kit/SPEC.md §Layout and configuration, and `check-knob-default-coupling`.
- **The `consult-count` arm (delta 5).** Producer: the `session-context` member, enabled by the brief-command element this repository's context-kit knob file sets; a session may also run it by hand. Consumer: the session, reading the brief. Its one field, the printed line, is read there. Roster-holding readers of the minted arm name: the crate's arm table with its arm-knob test, lifecycle-kit/SPEC.md §The consult inbox, and the kit README's command list.
- **`templates/nudge-delegation.md` (delta 5).** Producer: the kit. Consumer: an adopter, by copy; the member reads only the path a consumer's knob names.
- **The built-in KPI (delta 6).** Producer: the drift report's registry walk, enabled by the line in `templates/kpis.list`. Consumer: the report's reader, and the brief's drift line. Roster-holding readers of the member name: `templates/kpis.list` with `check-template-registry-parity`, drift-kit/SPEC.md §Bundled KPIs, and the kit smoke's per-member row assertion. This repository's reader is its release-sweep binding, through its own `kpis.list`.
- **Narrowed corpora, each reader's red condition.**
  - `check-template-copy-parity` skips a template with no copy and reds on a pair that diverges, so deleting both sides of the one pair removes a subject and no floor (delta 4).
  - `port-blockers --tree` counts owed files and subtracts declared ones, so three declared files leaving lowers both terms alike (deltas 4 and 6).
  - The bash audience has a reader that treats an empty result as undecided rather than not owed (`toolfloor::owed`). Where deltas 2 and 3 landed, the two kit-root arms answer nothing in the authoring tree, and `doctor` then renders the member unprobed for a selection reaching neither anchor arm. Delta 7 states that reading on the requirements page (delta 7).
  - `check-install-toolchain` holds the requirements page to the derivation, so the row and the derivation move in one commit (delta 7).

## Existing sections updated

Produced by `git grep -l -F` over the tracked tree for the two spellings under §Retired spellings and for the bash-led hook command, minus the paths the retired-spelling exclusion knob holds out, plus the sections the deltas name. The align audit added the surfaces a delta changes that carry neither spelling, found by `git grep -n -F session-context` and `git grep -n -F deprecated-surface` over the tracked tree and by reading the program each delta names. The build re-derives it.

- `guard-kit/SPEC.md` — §The hook on native Windows (deltas 1 and 2); §check-door-binding's exemption sentence (delta 3); the smoke paragraph's wiring (delta 2).
- `guard-kit/templates/settings-hooks.json` — the registrations, and the note's sentence on Git for Windows' bash (delta 2).
- `plugin/SPEC.md` — §The guards' rendering and §check-plugin-parity's assertion C (delta 2).
- `native/src/gates/plugin_parity.rs` — the rendering function and its unit test (delta 2).
- `native/src/gates/settings_paths.rs` — the suffix-less candidate (delta 2).
- `native/src/gates/mod.rs` — the knob roster on `check-settings-paths`' row (delta 2).
- `guard-kit/README.md` (delta 2)
- `guard-kit/smoke/install.sh` (delta 2)
- `gate-sdk/SPEC.md` — §The adopter constraints' interpreter-surface bullet and §The harness-integration arm's fail-open paragraphs (delta 3); §run-gates' statement of who reaches the stub (delta 3); §The harness-integration arm's registration grammar (delta 2); §The non-gate arm's `knob-values` paragraph, which names the example plugin as a reader (delta 6); §The harness-template port disposition and §check-template-registry-parity (delta 8).
- `context-kit/SPEC.md` — §The session-context hook (template) rewritten as the member's section under the unqualified heading (delta 4); §Layout and configuration's knob roster (delta 5); §check-settings-paths' front-end paragraph and suffix rule (delta 2); §bin/env-probe (delta 7).
- `context-kit/README.md` — the wiring step, which told an adopter to copy and edit a script (deltas 4 and 5).
- `context-kit/smoke/install.sh` — installs and runs the member in place of the copied script (delta 4).
- `context-kit/templates/settings-sessionstart.json` (delta 4)
- `context-kit/templates/session-context.sh` — deleted (delta 4).
- `scripts/session-context.sh` — deleted (delta 4).
- `context-kit/templates/nudge-delegation.md` — new (delta 5).
- `scripts/context-config.knobs` — this repository's five moved values (delta 5).
- `lifecycle-kit/SPEC.md` — §The consult inbox gains the count arm (delta 5); each published hook registration (delta 2).
- `lifecycle-kit/README.md` — the command list gains the arm (delta 5); its hook registration (delta 2).
- `native/src/emit/mod.rs` — the arm table's `consult-count` row, beside its new module (delta 5); the enforcement map arm's knob roster (delta 2).
- `native/src/knobs/context_kit.rs` — the minted rows and the three names gaining one (delta 5).
- `doctrine-kit/SPEC.md` — §stage-rules' surfacing-seam paragraph (deltas 4 and 5).
- `drift-kit/SPEC.md` — §Out of scope, §Bundled KPIs, the layout listing and the registry paragraph (delta 6); §The report skeleton's clause that the trend knob is wired in the template (delta 5).
- `drift-kit/README.md` — the wiring step's clause on a default in the hook copy (delta 5).
- `drift-kit/templates/kpi-deprecated-surface.sh` — deleted (delta 6).
- `drift-kit/templates/kpis.list` (delta 6)
- `drift-kit/smoke/install.sh` — the per-member row assertion (delta 6).
- `native/src/emit/kpi/mod.rs` — the member's registration, beside its new module (delta 6).
- `scripts/kpis.list` — this repository registers the member (delta 6).
- `canon-kit/SPEC.md` — §check-deprecation-task's sentence naming the example (delta 6).
- `delegation-kit/SPEC.md` — each published hook registration (delta 2).
- `delegation-kit/README.md` (delta 2)
- `delegation-kit/smoke/install.sh` (delta 2)
- `installer/SPEC.md` — §The update notice's honest limit on a consumer copy of the template (delta 4); §Requirements' bash bullets (delta 7).
- `docs/requirements.md` — the bash row and the Git for Windows row (delta 7).
- `native/src/hook/mod.rs` — the member row and the registration parser (deltas 2 and 4).
- `native/src/emit/enforcement_map.rs` — the registration reader (delta 2); the owner table's anchor for the session-context section (delta 4).
- `native/src/emit/run_guard_tests.rs` (delta 2)
- `native/src/gates/door_binding.rs` (delta 3)
- `native/src/toolfloor.rs` (delta 7)
- `plugin/hooks/hooks.json` — kept in the shell form; named so the build confirms it (delta 2).
- `.claude/settings.json` — rewritten by diffs the operator applies (deltas 1, 2 and 4).
- `.claude/commands/lead.md` — its cited hook registration (delta 2).
- `.claude/commands/release-sweep.md` — the hand-authored `inventory-command` binding, which runs the deleted template by path (delta 6).
- `docs/enforcement.md` — regenerated (deltas 2 and 4).
- `docs/context-kit/README.md` — mirror, regenerated (deltas 4 and 5).
- `docs/context-kit/SPEC.md` — mirror, regenerated (deltas 2, 4, 5 and 7).
- `docs/drift-kit/SPEC.md` — mirror, regenerated (deltas 5 and 6).
- `docs/drift-kit/README.md` — mirror, regenerated (delta 5).
- `docs/doctrine-kit/SPEC.md` — mirror, regenerated (deltas 4 and 5).
- `docs/canon-kit/SPEC.md` — mirror, regenerated (delta 6).
- `docs/installer/SPEC.md` — mirror, regenerated (deltas 4 and 7).
- `docs/plugin/SPEC.md` — mirror, regenerated (delta 2).
- `docs/gate-sdk/SPEC.md` — mirror, regenerated (deltas 3, 6 and 8).
- `docs/guard-kit/SPEC.md` — mirror, regenerated (deltas 1, 2 and 3).
- `docs/guard-kit/README.md` — mirror, regenerated (delta 2).
- `docs/delegation-kit/SPEC.md` — mirror, regenerated (delta 2).
- `docs/delegation-kit/README.md` — mirror, regenerated (delta 2).
- `docs/lifecycle-kit/SPEC.md` — mirror, regenerated (deltas 2 and 5).
- `docs/lifecycle-kit/README.md` — mirror, regenerated (deltas 2 and 5).
- `docs/posts/2026-09-26-checkwright-v0-26-0.md` — a dated release post, left standing as history (deltas 4 and 6).
- `docs/posts/2026-09-28-checkwright-v0-27-0.md` — a dated release post, left standing as history (delta 4).

## Retired spellings

- `session-context.sh` — the template and its copy are deleted (delta 4)
- `kpi-deprecated-surface.sh` — the template is deleted (delta 6)

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **The witness before the wiring** — delta 1's six facts are recorded for both hosts, or the unrun half is named, before a commit carrying delta 2 or 3 is made.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component.
- [ ] **Entry moved** — to Done before the drain stage, once the remote run is read, as the build stage's remote-oracle rule says, never in the merge commit; where delta 1's gate held deltas 2 and 3 back, on the disposition the lead gives.
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
