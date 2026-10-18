# SPEC amendment: hook-contract

Pairs `hook-contract-harness-neutral`. gate-sdk/SPEC.md §The adopter constraints declares the hook members "the exception by construction". Their sections state the shipped Claude Code binding's event names, matchers, payload keys and variables as the contract itself, so an adopter on another harness cannot tell the kit's contract from one vendor's spelling of it. This amendment does three things:

- **It states the contract once, in roles.** A new section of gate-sdk/SPEC.md does this, beside §The harness-integration arm, whose channels it already owns.
- **It spells the shipped binding in one table there.** The kit SPECs then name the roles and cite it.
- **It widens this repository's `GATE_SDK_HARNESS_LITERALS`** so `check-harness-literal` reds a spelling that leaves the table. That is the catcher landing with the fix.

**Reach: gate-sdk, guard-kit, delegation-kit, context-kit, lifecycle-kit, drift-kit, evidence-kit and this repo's knob file.** Delta 3 rewrites passages in six kits' SPECs and READMEs, so the amendment is cross-component and the audit stage follows spec. The set record orders this unit first. `plugin-harness-reach`'s guard wiring is stated against the table, and its amendment (plugin/SPEC-harness-reach.md) carries both acts for the order.

**The table's scope, decided at spec within the entry's corpus.**

- **The parts it covers**, each a class the entry names:
  - the hook protocol: events, matchers and tool names, payload fields, decision envelopes, exit status;
  - the session and config-home variables (whose owner stays lifecycle-kit/SPEC.md §bin/session-id.sh, the resolver);
  - the dispatch tool's parameters;
  - the transcript vocabulary of drift-kit's two transcript-reading meters, the stage-economics meter's fan-out walk and the manual-operation meter.
- **What it leaves out:** the settings file's permission-rule grammar (`Bash(…)` patterns, the matcher's wrapper strip). That is a different protocol, guard-kit's to state, and it is filed as a gap.

**The worklist is the oracle's, not a hand count.** The queue entry recorded 93 lines, and its grep is not reproducible. Run at spec over the gate's configured corpus, with delta 4's roster set through the environment, `check-harness-literal` reds **129** lines:

| file | lines |
|---|---|
| delegation-kit/SPEC.md | 57 |
| guard-kit/SPEC.md | 39 |
| lifecycle-kit/SPEC.md | 7 |
| context-kit/SPEC.md | 7 |
| gate-sdk/SPEC.md | 6 |
| drift-kit/SPEC.md | 6 |
| guard-kit/README.md | 2 |
| delegation-kit/README.md | 2 |
| lifecycle-kit/README.md | 1 |
| evidence-kit/SPEC.md | 1 |
| delegation-kit/templates/agent-execution.md | 1 |

The command:

```text
GATE_SDK_HARNESS_LITERALS="<delta 4's roster>" ./native/target/release/checkwright-gates --only check-harness-literal
```

**Two facts the kit prose omits, found reading the crate at spec**: the members read `tool_input.file_path` (`workflow-state-guard`), `tool_input.to` and `tool_input.message` (`escalation-guard`), and `agent_transcript_path` (`subagent-stop-liveness`), which no kit SPEC names as a read. The table names them.

## What changes

### (1) gate-sdk states the hook contract and its binding table {design-bearing}

A new `gate-sdk/SPEC.md` subsection, `#### The hook contract`, immediately after `#### The harness-integration arm`. **Not yet applied.** Each binding line carries the valve shown on its first line, so the section is the one declared site `check-harness-literal` admits.

> #### The hook contract
>
> Every hook member speaks one contract, stated here in **roles**. A harness reaches it through a **binding**, its spelling of each role. Kit prose names the role and cites this section, and the binding table below is the one place a spelling is written. The kit ships one binding, Claude Code's. A harness speaking it reaches every member unchanged; a harness speaking another protocol is unwired until a binding of its own is built (plugin/SPEC.md §The harnesses).
>
> - **The events.** *Before-tool-call* fires ahead of a tool call, scoped by the wiring's matcher to the tools a member serves. *Session-start* fires once per session, and never for a dispatched child. *Child-turn-end* fires when a dispatched child ends its turn. The main session's turn end is deliberately unwired (delegation-kit/SPEC.md §The turn-end liveness hook). A member is named by argv, never by its event (§The harness-integration arm).
> - **The tools a matcher names.** The *shell tools*, one per shell reader; the *dispatch tool*; the *message tool*; the *wake tools*; the *file-write tools*; the *search tools* (`GUARD_KIT_SEARCH_TOOLS`).
> - **The payload.** Every event carries the *event name*, the *session id*, the *transcript path* and the *working directory*. A call inside a dispatched child also carries the *child id*, whose presence and never whose value marks the nesting, and the *child type*. Before-tool-call adds the *tool name* and the *tool input*: the *command*, the *background flag*, the *written path*, the *message target* and *body*, and the dispatch tool's *dispatch parameters* (type, isolation, model, description, prompt). Child-turn-end adds the *loop guard*, the *live tasks* (each an id, a kind, a status, a description, and a child type or a command), the *child transcript* and the *last message*. Session-start adds the *model*.
> - **The decisions.** *Block*: exit 2, the reason on stderr, which the harness feeds to the model. *No decision*: exit 0 with nothing on stdout. *Advise*: exit 0 with a *context envelope* the model reads, on before-tool-call alone, since on child-turn-end it keeps the child running. *Grant*: exit 0 with an *allow envelope* and a reason. *Rewrite*: a grant carrying the changed command. *Operator notice*: exit 0 with the *notice field*, which every event accepts. Any other non-zero status is a non-blocking error after which the call proceeds, so an absent member disables its policy rather than refusing (guard-kit/SPEC.md §The shell guard). No member asks the user.
> - **The variables.** The *project directory*, written `<project-dir>` in kit prose; the *session-id variable*, which carries the root session's id in a child; the *child-session marker*, which is set at the top level too; the *config home*, written `<config-home>`; and the *plugin root*.
> - **The wiring.** The *exec form*, a program and an argument array started with no shell (§The harness-integration arm), and the optional *start-failure policy*.
> - **The transcript.** The *sessions directory* under `<config-home>`, a session's transcript there, each child's transcript and *meta record* beside it (the *parent id*, the *spawn depth*, the *fork flag*, the *child type*), and the *tool-call block* a meter counts.
>
> **The binding table, Claude Code.**
>
> - Events: before-tool-call `PreToolUse`, session-start `SessionStart`, child-turn-end `SubagentStop`, the unwired main turn end `Stop`. <!-- harness-binding: the shipped binding table -->
> - Tools: shells `Bash` and `PowerShell`; dispatch `Agent`; message `SendMessage`; wake `ScheduleWakeup` and `CronCreate`; file-write `Write` and `Edit`; search `Glob` and `Grep`. <!-- harness-binding: the shipped binding table -->
> - Common payload: `hook_event_name`, `session_id`, `transcript_path`, `cwd`, and inside a child `agent_id` and `agent_type`; also `permission_mode` and `prompt_id`, and on a tool call `effort` and `tool_use_id`, which no member reads. <!-- harness-binding: the shipped binding table -->
> - Tool call: `tool_name`; `tool_input`, carrying `command`, `run_in_background` (JSON `true` when set, absent when not), `file_path`, `to` and `message`, and for `Agent` `subagent_type`, `isolation`, `model`, `description` and `prompt`. <!-- harness-binding: the shipped binding table -->
> - Child turn end: `stop_hook_active`; `background_tasks`, each carrying `id`, `type` (`subagent` or `shell`), `status` and `description`, plus `agent_type` or `command`; `agent_transcript_path`; `last_assistant_message`; also `session_crons`, which no member reads. <!-- harness-binding: the shipped binding table -->
> - Session start: `model`. <!-- harness-binding: the shipped binding table -->
> - Envelopes: context `{"hookSpecificOutput":{"hookEventName":"PreToolUse","additionalContext":…}}`; allow `permissionDecision` `"allow"` with `permissionDecisionReason`; rewrite `updatedInput`; notice `systemMessage`; a turn end also refuses on `decision: "block"`. <!-- harness-binding: the shipped binding table -->
> - Variables: `CLAUDE_PROJECT_DIR`, `CLAUDE_CODE_SESSION_ID`, `CLAUDE_CODE_CHILD_SESSION`, `CLAUDE_CONFIG_DIR` (default `~/.claude`), `CLAUDE_PLUGIN_ROOT`. <!-- harness-binding: the shipped binding table -->
> - Wiring: `command` with `args`; `onFailure`. <!-- harness-binding: the shipped binding table -->
> - Transcript: `<config-home>/projects/<cwd-slug>/<session-id>.jsonl`, children under `<session-id>/subagents/` with a `.meta.json` carrying `parentAgentId`, `spawnDepth`, `isFork` and `agentType`; a call is a `tool_use` block on an assistant line. <!-- harness-binding: the shipped binding table -->
>
> **The measured record.** The payload rows above were read off the harness's published hook contract and off recorded firings, fetched rather than recalled. A harness revision reshaping the payload is drift no gate here detects, so only re-reading the contract catches it.
>
> **Honest limits.** The crate spells each member's payload paths and its envelopes in the member's own module, so no gate holds this table to the code: a renamed key reds a member's fixture, not this section. The table names the payloads the members read, not every field a harness sends.

### (2) The adopter constraint cites the contract {design-bearing}

Replacement for `gate-sdk/SPEC.md` §The adopter constraints, the bullet **The master harness is a binding, never an assumption.** **Not yet applied.**

> - **The master harness is a binding, never an assumption.** Kit prose names a harness surface generically wherever config owns it: the settings file and the agent-skill directory, each a knob. Kit prose names a hook member's protocol, the project directory, the session and config-home variables, the dispatch tool's parameters and the transcript's shape by their roles in §The hook contract, whose binding table is the one site spelling Claude Code's binding. §check-harness-literal holds the listed spellings to that table and to the adapter sites that read them. Grounds: an adopter on another harness must tell a contract of the kit from one vendor's spelling of it.

### (3) The kit prose names roles and cites the contract {design-bearing}

Every line the widened gate reds (delta 4) takes one of three dispositions. The amendment fixes the rule, and the gate's output is the exhaustive worklist:

- **(a) A contract sentence**, one that states what a member does in the shipped binding's spelling, is rewritten to the role and cites gate-sdk/SPEC.md §The hook contract once per section. The rewrite re-phrases, never appends.
- **(b) A measured record of the shipped payload** is merged into §The hook contract's binding table and measured record, and its site keeps the role-level conclusion it argues to. There are three such records: guard-kit/SPEC.md §The reader and its views' payload record; delegation-kit/SPEC.md §The delegation model's chokepoint roster and its honest limit; and delegation-kit/SPEC.md §The turn-end liveness hook's live-children shape.
- **(c) An adapter site** keeps the literal under a `harness-binding: <reason>` valve, naming why. Such a site reads or writes the spelling itself:
  - the session-id resolver;
  - the settings-path and memory-off readers;
  - a fixture, table-column or log grammar;
  - a wiring instruction an adopter pastes;
  - a template addressed to the harness's own agent.

The sections, from the oracle's output grouped by heading, with the disposition each section's lines mostly take. Build reads each line against the rule:

- **gate-sdk/SPEC.md:** §The harness-integration arm (the argv paragraph, the decline paragraphs), (a); §run-gates, the stub's notice envelope, (a); §enforcement-map's session-start warning, (a).
- **guard-kit/SPEC.md:**
  - the intro, §The friction loop and §The generic ruleset's shared directions, (a);
  - §The shell guard: the protocol and verdict paragraphs and the call's step 3, (a); its valved project-directory sentence, already (c), now citing the table for the spelling;
  - §The reader and its views' payload record, (b), with the live-mode paragraphs following it keeping their argument in roles;
  - §Consumer rules, (a), with the `view` mode's payload reads as (c);
  - §The hook on native Windows, (a);
  - §The rule roster: the chokepoint and grant-envelope wording, (a); `background_no_record`'s harness form, (a);
  - §Writing a consumer rule, (a);
  - §rewrite's §The rewrite arm's security posture, (a);
  - §wakeup-guard and §escalation-guard, (a), with the attempt-log line's fields as (c);
  - §Layout and configuration's file roster comment and fixture columns, (c);
  - §Testing's in-crate payload and classification-ladder descriptions and its fixture tables, (c);
  - guard-kit/README.md's opening line, (a), and its §Install wiring line, (c).
- **delegation-kit/SPEC.md:**
  - §The delegation model with its §Operative residency, §The probe is asymmetric… and §Attribution was weighed… subsections, (a), with the chokepoint roster (b) and the two env-route sentences as (c);
  - §The turn-end liveness hook, (a) throughout, with the log line's grammar (c), and its §What `background_tasks` carries subsection's live-children shape (b);
  - §usage-verdict's §The usage.txt contract, the `<config-home>` line, (c);
  - §model-verdict's session-start payload, (a);
  - §Layout and configuration's config home and wiring prose, (c);
  - §Testing's fixture columns, (c);
  - delegation-kit/README.md's two §Install wiring steps, (c);
  - delegation-kit/templates/agent-execution.md line 8, (c).
- **context-kit/SPEC.md:** §The session-context hook, (a), with its env-route sentences (c); §check-memory-off and §Layout and configuration's config home, (c).
- **lifecycle-kit/SPEC.md:** §bin/session-id.sh, (c) as the resolver; §The rename mode, §check-stage-evidence and §check-dispatch-entry, (a); lifecycle-kit/README.md's §Install wiring line, (c).
- **drift-kit/SPEC.md:** §The stage-economics meter's fan-out walk and §The manual-operation meter, (a); §Testing's fan-out fixture description, (c).
- **evidence-kit/SPEC.md:** §lifecycle-kit integration, (a).

**The project-directory sentence moves with it.** guard-kit/SPEC.md §The shell guard's valved paragraph, "Kit prose writes that variable `<project-dir>` … Claude Code's is `CLAUDE_PROJECT_DIR`", keeps its argument. Its spelling clause reads "the binding table spells it (gate-sdk/SPEC.md §The hook contract)", and its valve stays for the sentence that names the wiring templates.

### (4) This repository lists the binding's spellings {mechanical}

`scripts/gate-sdk-config.knobs`, `GATE_SDK_HARNESS_LITERALS`, from its one member to the roster below. **Not yet applied.** Each member is a spelling distinctive enough that a byte match finds no kit-neutral word; `Bash`, `Agent`, `Write`, `Edit`, `session_id` and `agent_id` are left out as words prose uses otherwise.

```text
CLAUDE_PROJECT_DIR CLAUDE_CODE_SESSION_ID CLAUDE_CODE_CHILD_SESSION CLAUDE_CONFIG_DIR CLAUDE_PLUGIN_ROOT
PreToolUse PostToolUse SessionStart SessionEnd SubagentStop UserPromptSubmit PreCompact
hookSpecificOutput hookEventName additionalContext permissionDecision updatedInput systemMessage
hook_event_name tool_input tool_name transcript_path stop_hook_active background_tasks agent_transcript_path last_assistant_message
run_in_background subagent_type
parentAgentId spawnDepth isFork tool_use
```

**It lands in the commit that clears the last red line**, so no commit carries the wider roster red. Until then the same roster set in the environment is the sweep's worklist (the command above).

### (5) check-harness-literal's closing sentence names the roster, not one variable {mechanical}

`gate-sdk/SPEC.md` §check-harness-literal, the last sentence of its final paragraph. "This repo's tracked gate-sdk knob file sets both, and each site on its corpus that reads the variable or declares the binding carries the valve." reads: "This repo's tracked gate-sdk knob file sets both, the literals being the binding table's distinctive spellings (§The hook contract). The table and each adapter site reading a listed spelling carry the valve." **Not yet applied.**

## Producers and consumers

- **§The hook contract (delta 1).**
  - Producer: this amendment's merge.
  - Consumers:
    - the kit SPECs' citations (delta 3);
    - plugin/SPEC.md §The guards and §The harnesses, whose per-harness reach is stated against the table;
    - an adopter writing a binding of their own.
  - Its one roster-holding reader is `check-harness-literal`, which holds its listed spellings to the valved lines.
  - The roles are prose. No field of a new message is minted, so point 4 binds only the table's own rows. Each row names a field some member reads, per the survey's crate read, or is marked "which no member reads". Those marked rows stay because the measured records (delta 3 (b)) carry them now.
- **The widened roster (delta 4).**
  - Producer: this repo's knob file, a deployed configuration.
  - Consumer: `check-harness-literal`, at `precommit` with `trigger=*` and on the battery.
  - Its red condition is a listed spelling on an unvalved corpus line, monotone in the violation set, so the verdict tightens by inspection. The kit's default stays empty (§Layout and configuration), so no adopter meets it.
- **Point 5, a narrowed corpus.** Delta 3 removes literals from prose, a narrowing of what the gate finds. Its red condition is a finding, never an absence, so removal clears it. No reader asserts a count or a floor over these spellings: `git grep -n "harness-binding" -- native/src` names the gate's own module and the comment-tier directive roster, neither counting sites.
- **Point 6.** Delta 3 obliges every line the oracle lists. Each line's disposition is one of the three above, and a line none fits is a gap the build resolves that session.

## Existing sections updated

The roster came from the oracle command above over `GATE_SDK_HARNESS_LITERAL_PATHS`' corpus at revision `c0fe5fbd8`, grouped by the heading above each line.

- `gate-sdk/SPEC.md` §The harness-integration arm, the new §The hook contract after it (delta 1), and its literal-bearing paragraphs (delta 3).
- `gate-sdk/SPEC.md` §The adopter constraints (delta 2); §run-gates, §enforcement-map (delta 3); §check-harness-literal (delta 5).
- `guard-kit/SPEC.md` and `guard-kit/README.md`, the sections named in delta 3 (delta 3).
- `delegation-kit/SPEC.md`, `delegation-kit/README.md` and `delegation-kit/templates/agent-execution.md` (delta 3).
- `context-kit/SPEC.md` (delta 3).
- `lifecycle-kit/SPEC.md` and `lifecycle-kit/README.md` (delta 3).
- `drift-kit/SPEC.md` (delta 3).
- `evidence-kit/SPEC.md` (delta 3).
- `scripts/gate-sdk-config.knobs` (delta 4).
- The generated mirrors under `docs/` for every SPEC and README above, and `docs/llms-full.txt` (all deltas).

## Retired spellings

- None — no name is retired; every spelling stays, relocated behind the binding table or valved at its adapter site.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **The oracle clean** — `check-harness-literal` green with delta 4's roster committed, and every kit suite the edited SPECs reach green.
- [ ] **Done move** — the entry moves to Done in the commit that merges this amendment, no later than the stage before the drain stage.
