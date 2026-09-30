# SPEC amendment: foreign-run

**The first slice of cross-vendor delegation: a foreign-CLI executor for the class this protocol already pre-authorizes, a read-only audit or a mechanical sweep.** The operator holds working subscriptions to other vendors' coding agents and wants read-heavy delegation routed to them for budget headroom. Every dispatch today rides the master harness's own `Agent` tool, so all delegated spend lands on one vendor's window. This slice is not full stage dispatch. The escalation resume model, a transport-neutral handoff for a stage, and the N-keyed budget oracle stay the entry's later slices, and the entry is demoted to carry them when this one lands.

**The seam ruling is on record in the entry: generic mechanism only.** The executor, its knobs and its return contract are kit mechanism. The adapter for each vendor is consumer config: the program and its argv, including the vendor's non-interactive mode, its output format, its sandbox mode and its model. A kit literal naming a vendor, a vendor CLI or a vendor's event schema crosses the provenance seam (gate-sdk/SPEC.md §The provenance seam). An adapter speaks its vendor's machine plane and never its interactive screen, on the entry's recorded ground.

**The attribution policy is the operator's direction, relayed by the lead and not a ruling: foreign harnesses' commit trailers off, and the method stated once in the README.** This design holds it by construction and changes no harness setting. The foreign agent works in a scratch clone whose commits never reach this repository. Its output returns as a report and a patch, and it lands in the dispatching session's own commit. Any change to a vendor harness's own trailer setting stays the operator's to confirm, and none is made here.

**Measured at authoring.**

- The upgrade smoke builds a ref's binary in a shared local clone and never in a linked worktree (gate-sdk/SPEC.md §upgrade-smoke). Rust runs no destructor on a signal, so a killed run would strand a worktree registration, which the iteration-boundary entry refuses. A clone registers nothing. The executor takes the same shape for the same reason.
- A clone's refs are its own. A commit made inside it is unreachable from this repository's refs until something fetches it, so "never take a foreign commit" is the default state rather than a check that could be skipped.
- The master harness's per-dispatch guards fire on its `Agent` tool (delegation-kit/SPEC.md §The delegation model). An executor reached through a shell call passes neither `agent-budget-guard` nor `agent-dispatch-guard`, so its limits are stated below rather than inherited.

**Four alternatives were weighed and refused.**

- **Letting the foreign agent commit on the shared tree.** It would race the shared git index, which is this protocol's first failure surface, and every vendor's own commit trailer would add a contributor per vendor. That is the attribution the operator directed off.
- **A linked worktree.** Refused on the upgrade smoke's ground above.
- **Parsing each vendor's event stream in the kit to extract the final message.** It is a vendor schema in a kit literal. The adapter selects the vendor mode whose standard output is the report, and the kit reads bytes.
- **Gating the executor behind the budget guard.** The guard reads the master harness's window. A foreign run spends a different vendor's window, which is the point of the slice. A foreign vendor's budget oracle is the entry's N-keyed seam and a later slice.

## What changes

### (1) `DELEGATION_KIT_FOREIGN_ADAPTERS` and `DELEGATION_KIT_FOREIGN_TIMEOUT`

delegation-kit gains two knobs {mechanical}. **Not yet applied.** Both rows join `native/src/knobs/delegation_kit.rs`, and both bullets join §Layout and configuration after `DELEGATION_KIT_STATUSLINE_INBOXES`':

> - `DELEGATION_KIT_FOREIGN_ADAPTERS` — the foreign adapters `--foreign-run` spawns (§The foreign-vendor run): indexed, each element `<adapter>=<word>`, an adapter's argv being its words in element order. Default empty, which configures no adapter. The table validator refuses an element with no `=`, an adapter name outside `[a-z0-9-]` or an empty word. A word containing `@PROMPT_FILE@` has the prompt file's absolute path substituted. The kit ships no adapter, since every one names a vendor's program.
> - `DELEGATION_KIT_FOREIGN_TIMEOUT` — the wall-clock bound on one foreign run, in seconds; default `1800`, validated a positive integer by the table validator. A run outliving it is killed and reads as failed.

`templates/delegation-config.knobs` gains a commented example under a `spec:` line citing §The foreign-vendor run: `# DELEGATION_KIT_FOREIGN_ADAPTERS[] = <adapter>=<program>` and `# DELEGATION_KIT_FOREIGN_ADAPTERS[] = <adapter>=@PROMPT_FILE@`, with no vendor named.

### (2) `--foreign-run` runs one foreign unit in a scratch clone

delegation-kit gains the compiled arm `bash gate-sdk/bin/run-gates.sh --foreign-run <adapter> <prompt-file> [--mode audit|sweep] [--key <key>]`, and delegation-kit/SPEC.md gains `## The foreign-vendor run` before `## Layout and configuration` {design-bearing}. **Not yet applied.**

> ## The foreign-vendor run
>
> A read-only audit or a mechanical sweep, the mechanical tier class (§The tier binding), may run on another vendor's coding agent instead of a dispatch. `bash gate-sdk/bin/run-gates.sh --foreign-run <adapter> <prompt-file> [--mode audit|sweep] [--key <key>]` runs one such unit. The mode defaults to `audit`. The key defaults to the prompt file's basename without its extension, and names the run's directory, `<GATE_SDK_TMP_DIR>/foreign/<key>/`.
>
> 1. **The tree.** A shared, no-checkout clone of the toplevel, checked out detached at the committed `HEAD`, under the run's directory as `tree/`. A clone rather than a linked worktree, on §upgrade-smoke's ground (gate-sdk/SPEC.md): a killed run leaves scratch and no registration. The foreign agent sees committed state only, as an isolated child does (§The delegation model, isolation's untracked-blindness cost).
> 2. **The spawn.** The adapter's argv from `DELEGATION_KIT_FOREIGN_ADAPTERS`, spawned directly with no shell, with its working directory the clone and its standard input the prompt file. Standard output goes to `report.txt` and standard error to `stderr.txt`, both in the run's directory, bounded by `DELEGATION_KIT_FOREIGN_TIMEOUT`.
> 3. **The shape.** After the agent exits, a clone whose `HEAD` moved is refused: the foreign agent committed, and its commits are never taken. In `audit` mode, a clone whose status is not clean is refused: an audit wrote. In `sweep` mode, the clone's whole change, untracked files included, is written as `change.patch` in the run's directory.
> 4. **The cleanup.** The clone is removed, except on a refusal, where it stays for inspection and the line names it.
>
> **The verdict line** is `foreign-run: adapter=<adapter> mode=<mode> key=<key> exit=<status> report=<path> patch=<path|none> -> <VERDICT>`. The exits:
>
> - **0 `OK`** — the adapter exited 0 and the shape held.
> - **1 `REFUSED (<why>)`** — the shape broke: `committed`, or `audit wrote`.
> - **2 `FAILED (<why>)`** — an unknown or empty adapter, an unreadable prompt file, a clone or spawn failure, a timeout, or a non-zero adapter exit. The report is kept, and the line names the adapter's status.
>
> **What returns, and how it lands.**
>
> - `report.txt` is the unit's return value, and a durable one: it outlives the calling session, so a successor finds it.
> - A sweep's `change.patch` is applied by the dispatching session, which verifies it as it verifies any agent commit (§Verify after every agent commit) and commits it under its own attribution. So a foreign vendor's work carries no trailer of its own, and the repository's contributor list does not grow per vendor. A consumer's README states that method once.
> - The adapter's argv carries its own vendor's model choice, an alias or a pinned id, on §The tier binding's follow-or-pin reading.
>
> **Honest limits.**
>
> - **Confinement.** The clone is the agent's working directory, not a sandbox. A foreign program can write any path it can reach, so an adapter selects its vendor's read-only or workspace-bounded sandbox mode, and that mode is the only confinement there is. This is the residue §The delegation model records for a program an isolated child runs.
> - **No guard fires.** The executor is reached through a shell call, so the budget guard and the dispatch guard do not fire on it, and nothing budgets a foreign vendor's window.
> - **No resume.** A run is not resumable, and its escalation channel is its report.
> - **The report's own claims.** A report's claims carry the tier a dispatcher gives any child's report: checked before they are built on.
>
> It is an `Arm::Run` row that spawns a consumer's program which may reach the network, so it takes a bare-flag spelling and stays out of the fence-safe set (gate-sdk/SPEC.md §The non-gate arm). Its declared knobs are `DELEGATION_KIT_FOREIGN_ADAPTERS`, `DELEGATION_KIT_FOREIGN_TIMEOUT` and `GATE_SDK_TMP_DIR`.

Registration:

- the arm row joins `native/src/emit/mod.rs`'s table;
- `--foreign-run` joins the crate's network-spawner list there, which a unit test holds disjoint from the fence-safe set;
- its usage line joins `native/src/runner.rs`'s help;
- gate-sdk/SPEC.md §The non-gate arm's spawner rosters gain it: the network-spawner sentence of the fence-safe paragraph, and the *changeable by a consumer* bullet ("`--foreign-run` spawns `git` for the clone and whatever `DELEGATION_KIT_FOREIGN_ADAPTERS` names").

### (3) The executor's crate tests

A crate test module drives the arm's library function against a throwaway repository, with a firing and a non-firing case per exit {design-bearing}. **Not yet applied.**

- **`OK`, both modes.** An audit whose adapter prints and writes nothing returns its output as the report. A sweep whose adapter writes a tracked and an untracked file yields a patch that applies cleanly to the source repository.
- **`REFUSED`.** An adapter that commits is refused as `committed` in both modes. An audit whose adapter writes is refused as `audit wrote`. The clone is kept on each.
- **`FAILED`.** An unknown adapter; an unreadable prompt file; a non-zero adapter exit, with its report kept; and a timeout under a one-second bound.
- **The rest.** `@PROMPT_FILE@` substitution, standard input carrying the prompt, and the clone removed on every non-refusal exit.

The stub adapters use programs the crate's tests already spawn, `git` first among them: `git config --file <name>` writes a file and `git commit --allow-empty` moves `HEAD`. The build picks a program for the timeout row that outlives its bound on every CI host. §Testing gains a paragraph naming the module and these rows.

### (4) The protocol template carries the dispatcher's half

delegation-kit/templates/agent-execution.md gains a bullet after **Match the dispatched model and effort to the unit's shape** {mechanical}. **Not yet applied.**

> - **A foreign-vendor run is mechanical work returned through files.** Where your consumer configures a foreign adapter, a read-only audit or a mechanical sweep may run on another vendor's coding agent through `--foreign-run` instead of a dispatch (delegation-kit/SPEC.md §The foreign-vendor run). Write the prompt to a file in the scratch dir. The report file is the return; a sweep's patch is yours to apply, verify and commit; the agent's own commits are never taken. It sees committed `HEAD` only, so a unit over uncommitted or gitignored content stays on your harness. No guard fires on it and it cannot be resumed, so size it to finish in one run.

That bullet's lead-in is new, and §The foreign-vendor run cites no template rule by name, so `check-rule-citation` has nothing new to resolve.

### (5) The README states the attribution method once

README.md's contributor paragraph ("Most commits here carry a `Co-Authored-By` trailer…") gains one sentence after its first {mechanical}. **Not yet applied.**

> Work delegated to another vendor's coding agent carries no trailer of its own: it returns as a report or a patch and lands in the delegating session's commit, so the contributor list does not grow per vendor.

### (6) The tier binding names the foreign route

§The tier binding gains the sentence "The mechanical class may also run on another vendor's agent through a configured adapter (§The foreign-vendor run)." {mechanical} **Not yet applied.** Its act depends on tier-model-binding's landing:

- **If tier-model-binding delta 1 has landed, in this unit's batch or earlier,** the sentence joins §The tier binding's opening paragraph, and §The foreign-vendor run cites §The tier binding as written above.
- **If it has not,** this delta waits. §The foreign-vendor run's two citations of §The tier binding become "the mechanical tier class this protocol pre-authorizes" and "the follow-or-pin reading its tier binding takes", with no section pointer. This delta, and restoring the pointers, then become tier-model-binding's batch's last act. A dangling section pointer would red the spec-pointer gates in the meantime.

## Producers and consumers

- **`DELEGATION_KIT_FOREIGN_ADAPTERS` and `DELEGATION_KIT_FOREIGN_TIMEOUT`.**
  - Producer: a consumer's knob file. This repository sets neither in this slice: an adapter names the operator's own vendor programs, which a private overlay (`delegation-config.local.knobs`) carries where the operator configures one.
  - Consumers: the arm, and the roster-holding readers of a knob name: `native/src/knobs/delegation_kit.rs`'s table, `--emit knob-roster`, `check-knob-citation`, `check-knob-default-coupling` (the timeout's `1800` and the adapters' "default empty") and the template's example lines.
- **The arm.** Producer: a dispatching session choosing the foreign route, per the template bullet. Its roster-holding readers: the arm table, the network-spawner list and its disjointness test in `native/src/emit/mod.rs`, the help text in `native/src/runner.rs`, and gate-sdk/SPEC.md §The non-gate arm's spawner rosters.
- **The run's files.**
  - `report.txt` is read by the dispatching session as the unit's return.
  - `change.patch` is read by the dispatching session, which applies it.
  - `stderr.txt` is read by a person diagnosing a failed run.
  - The kept clone is read by a person inspecting a refusal, and by the iteration boundary's scratch reset, which removes it with the scratch dir.
- **The verdict line's fields.** The `report` and `patch` paths are what the dispatcher opens next, and `exit` with the verdict is what it routes on.
- **Enabling config.** No tracked configuration can set an adapter, since every adapter names a vendor's program. The deployed configuration is therefore the operator's private overlay, `scripts/delegation-config.local.knobs`, which `.gitignore` already covers. One vendor CLI is installed on the authoring machine (`which` over the candidate names, not recorded here on the provenance seam). The build writes one adapter there once the operator confirms it (Definition of Done, *a live run*), so the producer is reachable outside the crate tests.

## Existing sections updated

- `delegation-kit/SPEC.md`: §The foreign-vendor run, new (delta 2); §Layout and configuration (delta 1); §Testing (delta 3); §The tier binding, once merged (delta 6).
- `native/src/knobs/delegation_kit.rs` (delta 1).
- `native/src/emit/mod.rs`, `native/src/runner.rs` and a new executor module (delta 2).
- The executor module's tests (delta 3).
- `gate-sdk/SPEC.md` §The non-gate arm, the spawner rosters (delta 2).
- `delegation-kit/templates/delegation-config.knobs` (delta 1).
- `delegation-kit/templates/agent-execution.md` (delta 4). tier-model-binding's tier-reading edit changes the neighbouring bullet.
- `README.md` (delta 5).
- `docs/delegation-kit/SPEC.md`, `docs/delegation-kit/README.md` and `docs/gate-sdk/SPEC.md`: the generated mirror (all deltas).

The roster came from `git grep -n 'price-coverage'` and `git grep -n 'with-foreign-shells'` for the registration sites of a network-spawning `Arm::Run`, `git grep -n 'DELEGATION_KIT_STATUSLINE_INBOXES'` for a delegation-kit indexed knob's readers, and `grep -n -i 'co-authored\|trailer' README.md CLAUDE.md` for the attribution statement.

## Retired spellings

- None — the amendment adds an arm and two knobs and renames nothing.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for the knobs, the arm, the run's files and the template bullet.
- [ ] **Instruction surfaces: instruction only** — the template bullet carries the route and its limits as instructions; the grounds sit in §The foreign-vendor run.
- [ ] **Merged with no information lost** — §The foreign-vendor run carries the refused alternatives as its grounds and the honest limits beside the mechanism.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls delegation-kit/SPEC-*.md`).
- [ ] **Removals propagated** — nothing retired.
- [ ] **No harness setting changed** — no settings file in the tree and no vendor configuration was edited for the attribution policy.
- [ ] **A live run** — with the operator's confirmation of the adapter's argv (its sandbox mode included) and of the vendor spend, the private overlay configures one adapter. One read-only audit then runs through `--foreign-run` to `OK`, and its verdict line is quoted in the landing commit's message with the adapter name elided.
- [ ] **Gaps filed** — a cross-component gap found during the work filed with `--emit file-gap`.
- [ ] **The entry is demoted, not done** — its deliverable is the corpus of slices, so the landing commit returns `heterogeneous-agent-delegation` to the deferred section with `--queue demote`, a stage before the drain stage. It drops the `[spec:]` tag, records this slice as landed and names the next (canon-kit/SPEC.md §Merging an amendment, step 4). The same commit rewords the entry's body to fit queue-kit's per-entry cap.
