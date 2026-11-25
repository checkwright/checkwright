# SPEC amendment: tree-free-widening

Pairs `tree-free-hook-roster-widening`. Every `HOOKS` member is weighed against the tree-free test, which asks whether each input its rule reads can resolve absolute. Four more members pass: `escalation-guard`, `wakeup-guard`, `agent-budget-guard` and `agent-dispatch-guard`. Each gets its path checks and its own fault route under the tree-free mark, and `init --user` accepts it. The other three members record why they read a repository. A before-tool-call registration needs a matcher, which no table records today, so the table gains a matcher column for `init --user` to print.

**What the tree does today**, each bullet with the probe that settled it:

- The tree-free column is true for `subagent-stop-liveness` alone, and a unit test asserts that set (`grep -n 'true,$\|only_the_turn_end_liveness_member_is_tree_free' native/src/hook/mod.rs`).
- `escalation-guard` reads only its payload and declares no knob. `wakeup-guard` reads one knob, `GUARD_KIT_WAKEUP_LOG`, whose default derives under `${GATE_SDK_WORKFLOW_DIR:-.workflow}` and is relative. It appends to that file only when the knob resolves, and denies either way (`grep -n 'knob_scalar\|block' native/src/hook/wakeup.rs native/src/hook/escalation.rs`; guard-kit/SPEC.md §Layout and configuration).
- `agent-budget-guard` runs the verdict rule in process. That rule reads the usage, credential and account paths, whose defaults derive from the home directory and the harness config home. It also reads the history path, empty by default, and a refresh command it spawns. A knob it cannot read is a `STALE` verdict, which the guard advises and never blocks on (`grep -n 'fn config\|-> STALE' native/src/hook/verdict.rs`; `grep -n 'fn paths' -A20 native/src/hook/usage.rs`).
- `agent-dispatch-guard` reads name rosters, D5's switch, the agent dir (default `.claude/agents`, relative) and D6's binding. A knob it cannot read leaves the rule it serves unenforced with a note, and the guard spawns nothing (`grep -n 'went unenforced\|AGENT_DIR\|door_command' native/src/hook/dispatch.rs`).
- Three members read the repository itself:
  - `shell-guard` asks git for the main checkout root and reads the project's settings file (`grep -n 'main_checkout_root\|GUARD_KIT_SETTINGS' native/src/guard/host.rs`).
  - `session-context` re-runs the binary's own arms and consumer commands over the tree (`grep -n 'proc::run' native/src/hook/session_context.rs`).
  - `workflow-state-guard` judges the repository's stamp file (`grep -n 'LIFECYCLE_KIT_STATE_FILE' native/src/hook/workflow_state.rs`).
- The registration `init --user` prints carries the event key and no matcher (`grep -n 'fn wiring_fragment' -A12 native/src/hook/binding.rs`). A before-tool-call entry with no matcher fires on every tool call.
- A `--knobs` file the arm cannot use declines, so the call is allowed, before the member runs (`grep -n 'check_hook_file' -A2 native/src/hook/mod.rs`).
- The knob file `init --user` writes carries `GATE_SDK_TMP_DIR` and `DELEGATION_KIT_STOP_LOG` for any member (`grep -n 'fn knob_text' -A8 native/src/installer/user.rs`).
- `walk::host_absolute` is the one absolute-path test under the mark, and the crate's Windows leg already holds it (gate-sdk/SPEC.md §The crate's crosser, the paragraph on a value the host resolves as given).

## What changes

### (1) Four members widened, each with its path checks and its fault route {design-bearing} {user-facing: an adopter outside any repository gets the escalation, wakeup, budget and dispatch guards, and `--knobs` and `init --user` accept them}

The column is set for `escalation-guard`, `wakeup-guard`, `agent-budget-guard` and `agent-dispatch-guard`. Under the mark each holds its path-valued knobs to `hook::tree_free_path`. A relative value, a default included, is a knob fault, and each member takes the fault through the route its rule already has for a knob it cannot read. A uniform decline would allow the very call `wakeup-guard` exists to deny.

**Not yet applied** — gate-sdk/SPEC.md §The harness-integration arm, the fifth-column paragraph's sentence "It is set for `subagent-stop-liveness` alone." becomes:

> It is set for `subagent-stop-liveness`, `escalation-guard`, `wakeup-guard`, `agent-budget-guard` and `agent-dispatch-guard`, each owning section stating what it reads under the mark. A member whose column is false records why beside its row's comment: `shell-guard` asks git for the main checkout and reads the project's settings, `session-context` runs the binary's arms over the tree, and `workflow-state-guard` judges the repository's stamp file.

**Not yet applied** — the bullet opening "**Under the mark, every path the member reads is absolute**" re-phrases its decline clause to:

> a relative value, a default included, is a knob fault the member takes through its own route, never a silent read: a fail-open member declines through `hook::decline` naming the knob and its value, or leaves the one rule the knob serves unenforced with a note, as its owning section states. `wakeup-guard` keeps its denial and loses its record.

The grounds sentence stays as it is. The bullet opening "**No repository, a tree-free member.**" re-phrases "gets no turn-end guard at all" to "gets none of these guards at all".

**Not yet applied** — the sub-bullet opening "**A file the arm cannot use declines**" becomes:

> - **A file the arm cannot use is a knob fault carried to the member**, checked whole before the member runs. The causes are a relative path, a missing or unreadable file, a malformed line, a name no kit owns, and one its owning kit does not declare. A fail-open member declines through `hook::decline` at exit 0, naming `--knobs`, the file and, where one is at fault, the line. `wakeup-guard` denies, its record skipped and its denial naming the fault. **Grounds:** an exit 2 on a turn-end event refuses every turn end of every session the user runs, the harm the arm exists to prevent. A wakeup denial is the rule itself, so a decline there would trade the rule for a notice.

The members' cases, each **Not yet applied** in its owning SPEC:

- **`escalation-guard`** (guard-kit/SPEC.md §escalation-guard, appended to its second paragraph): "It reads its payload alone, so it runs with no repository as it runs in one (gate-sdk/SPEC.md §The harness-integration arm), and a `--knobs` file it cannot use declines with the advise envelope."
- **`wakeup-guard`** (guard-kit/SPEC.md §wakeup-guard, a paragraph after its attempt-line paragraph): "**It runs with no repository** (gate-sdk/SPEC.md §The harness-integration arm), where `GUARD_KIT_WAKEUP_LOG` must be absolute. A relative value, the default included, or a `--knobs` file the arm cannot use costs the record and never the rule. The call is denied, and the denial gains one sentence naming the knob or the file and the fault, so a lost record is never silent." guard-kit/SPEC.md §Layout and configuration's `GUARD_KIT_WAKEUP_LOG` bullet gains "absolute when run with no repository (§wakeup-guard)".
- **`agent-budget-guard`** (delegation-kit/SPEC.md §Layout and configuration, the budget guard's wiring paragraph, appended): "It runs with no repository. There `DELEGATION_KIT_USAGE_FILE`, `DELEGATION_KIT_CRED_FILE`, `DELEGATION_KIT_ACCOUNT_CONFIG` and a non-empty `DELEGATION_KIT_USAGE_HISTORY` must be absolute, and so must `DELEGATION_KIT_REFRESH_CMD`'s first element where it carries a separator. Their derived defaults are absolute wherever the home directory is set. A relative value is the `STALE` verdict naming the knob, which advises and never blocks (§usage-verdict). The `PAUSE` corrective names the `--knobs` file as where a threshold is raised, since there is no project settings file."
- **`agent-dispatch-guard`** (delegation-kit/SPEC.md §Layout and configuration, the dispatch guard's wiring paragraph, appended): "It runs with no repository. With D5 switched on, `DELEGATION_KIT_AGENT_DIR` must then be absolute, and a relative one, the default included, leaves D5 unenforced with its note. The other rules read names, not paths. **Honest limit:** a user-scope guard looks a type up in the one agent dir its knob names, never in the project the call fires in, and D6's refusal names the knob-values door as the consumer's tree spells it."

`subagent-stop-liveness` keeps its checks and its decline. delegation-kit/SPEC.md §The turn-end liveness hook's sentence "It is its table's one tree-free member" re-phrases to "It is one of its table's tree-free members".

- **Why these four.** Each reads its payload, knobs that can be set absolute, and at most a spawned program whose head can be held absolute, which is the column's own test. None asks git anything.
- **Why per-member fault routes, never the arm's one decline.** The arm cannot tell a fail-closed rule from a fail-open one, and each member already states what a knob it cannot read costs. Routing through that keeps one statement of each member's posture.
- **Why `agent-dispatch-guard`'s agent dir is not written for the user.** D5 is off by default, so the default install never reads the dir. Resolving the harness's user agent dir at install time would add a host-specific derivation of the config home, which `init --user` does not hold today. An adopter switching D5 on sets the dir absolute in the knob file, and the hook's page says so.

### (2) The matcher column and the per-member user install {design-bearing} {user-facing: `init --user` prints a registration scoped to the member's tools and writes the member's own knob lines}

**Not yet applied** — gate-sdk/SPEC.md §The harness-integration arm, a paragraph after the sixth-column paragraph:

> **A row's seventh column, `matcher`, names the tools a before-tool-call member registers on**, spelled from the binding's tool constants (§The hook contract), and is empty for an event that takes no matcher. `init --user` prints it in the registration, so an entry fires on the member's tools alone. A unit test holds every before-tool-call row to a non-empty matcher. Another holds guard-kit's `templates/settings-hooks.json` to carry each member it registers under that member's matcher. A wake guard with no matcher would deny every tool call.

`native/src/hook/binding.rs` gains constants for the dispatch, message and wake tools that §The hook contract's binding table already names. `binding::wiring_fragment` takes the matcher and writes the group's matcher key when it is non-empty.

**Not yet applied** — installer/SPEC.md §The user install:

- The JSON-fragment bullet of **The report** gains "and, for a before-tool-call member, the group's matcher from the member's row".
- The `launch records:` bullet becomes "a line per state file the member reads or writes: `launch records:` for `subagent-stop-liveness`, naming the scratch dir and the knob that moves it, and `attempt log:` for `wakeup-guard`, naming its log".
- **The knob file** paragraph's first sentence becomes: "**The knob file** carries `GATE_SDK_TMP_DIR = <state>/tmp`, and for a member keeping a log, the log's knob at `<state>/<member>.log`: `DELEGATION_KIT_STOP_LOG` for `subagent-stop-liveness`, `GUARD_KIT_WAKEUP_LOG` for `wakeup-guard`." The sentence after it gains "A unit test fires each tree-free member under the knob file written for it and holds that no knob fault is reported".

- **Why a column, never a template read.** The installer ships no kit template, and the matcher is a fact of the member as the event is. The template stays the project-scope wiring, held to the column by the test.

### (3) The pages, the fixtures and the tests {mechanical}

- **Hook pages.** `docs/hooks/<member>.md` for each of the four, in the shape of `docs/hooks/subagent-stop-liveness.md`: what it refuses or advises, what it reads, how it exits, and its generated knob block. `docs/install.md`'s "One session hook alone" bullet lists the five pages. §check-hook-knobs-fresh already owes a page per tree-free member.
- **The gate's good fixture.** `gate-sdk/gate-tests/check-hook-knobs-fresh/good/hooks/` gains a fresh page per newly tree-free member, and its `args` comment re-phrases "the one tree-free member" to "every tree-free member". Without it the good side reds.
- **Unit tests in `native/src/hook/mod.rs`.**
  - `only_the_turn_end_liveness_member_is_tree_free` becomes a test that the five members are tree-free and the three are not.
  - `every_tree_free_row_carries_a_binding_event` holds each row to an event the binding names, the claim the sixth-column paragraph already states, rather than to the child-turn-end event.
  - The two matcher tests of delta 2 are added.
- **Member unit tests.**
  - `wakeup-guard` denies under a relative log and under a carried file fault, and names the fault.
  - The verdict rule's config reads `STALE` for a relative path knob under the mark.
  - `agent-dispatch-guard` notes D5 unenforced for a relative agent dir with D5 on.
- **Integration.** `native/tests/hook_tree_free.rs` fires each of the four with no repository on absolute knobs and once on a relative default, holding each fault route. `native/src/installer/user.rs`'s tests cover a before-tool-call member's printed matcher and its knob lines.

## Producers and consumers

- **The widened column.** Producer: the four rows. Consumers: the arm's no-repository dispatch, the `--knobs` refusal, the registration parser's `Malformed` case, `init --user`'s operand set, `update --user`'s re-check and §check-hook-knobs-fresh's page roster. All of them read the table and derive, so none lists the set.
- **The carried file fault.** Producer: `under_knobs`, on a file `check_hook_file` refuses. Consumer: each tree-free member, at its first act, through its own route.
- **The per-member path checks.** Producer: each member's knob reads under the mark. Consumers: the decline, the `STALE` line, the dispatch note and the wake denial, each read by the operator or the model as its member's channel already is.
- **The matcher column.** Producer: the rows, from binding constants. Consumers: `init --user`'s settings block, and the two unit tests. `check-settings-paths`, the enforcement map and `check-plugin-parity` read names and owners and ignore it.
- **The per-member knob text.** Producer: `init --user`. Consumers: the member's knob reads under `--knobs`, and `user.lock`'s hash claim, which is already per member.
- **Point 2, roster-holding readers.** The column's prose rosters are gate-sdk/SPEC.md's fifth-column sentence and delegation-kit/SPEC.md's "one tree-free member" sentence, both re-phrased by delta 1. `docs/install.md`'s bullet and the hook pages dir are re-phrased by delta 3, and the good fixture's `args` comment by delta 3. `native/tests/hook_tree_free.rs`' `MEMBER` constant stays the turn-end member, and its refusal case keeps `shell-guard`, which stays tree-bound. The matcher lands on `binding.rs`, which the binding-table test holds.
- **Point 5, narrowing.** None narrows. Two widenings are behavior changes the release declares. `--knobs` on the four was a grammar refusal at exit 2 and is now accepted. A registration of the four that fired outside a repository took the no-tree notice and allowed the call, and the member now enforces there.
- **Point 6, every member.** All eight rows are weighed: five tree-free with their cases, and three tree-bound, each reason recorded in delta 1.

## Existing sections updated

Roster probe: `git grep -n -i 'tree-free' -- ':!docs/llms-full.txt' ':!*/SPEC-*.md' ':!TASK-QUEUE.md' ':!docs/*/SPEC.md'`, `git grep -n 'wiring_fragment\|fn knob_text\|launch records:'` and `git grep -n '"matcher"' -- '*.json'`, over the tracked tree.

- `gate-sdk/SPEC.md` §The harness-integration arm — the fifth-column sentence, the mark's decline clause, the file-fault sub-bullet and the no-repository bullet (delta 1), and the seventh-column paragraph (delta 2).
- `guard-kit/SPEC.md` §wakeup-guard, §escalation-guard and the `GUARD_KIT_WAKEUP_LOG` bullet (delta 1).
- `delegation-kit/SPEC.md` §The turn-end liveness hook's sentence, and §Layout and configuration's budget-guard and dispatch-guard wiring paragraphs (delta 1).
- `installer/SPEC.md` §The user install — the report's fragment and state-file bullets, and the knob-file paragraph (delta 2).
- `native/src/hook/mod.rs` — the four rows' column, the seventh column, the tree-bound reasons in the row comments, the carried fault in `under_knobs`, and the table tests (deltas 1 to 3).
- `native/src/hook/wakeup.rs`, `escalation.rs`, `budget.rs`, `verdict.rs` and `dispatch.rs` — each member's path checks and fault route (delta 1).
- `native/src/hook/binding.rs` — the tool constants and the fragment's matcher (delta 2).
- `native/src/installer/user.rs` — the per-member knob text, the matcher in the settings block and the state-file lines (delta 2).
- `native/tests/hook_tree_free.rs` — the four members' cases (delta 3).
- `docs/hooks/` — the new pages `escalation-guard.md`, `wakeup-guard.md`, `agent-budget-guard.md` and `agent-dispatch-guard.md` (delta 3).
- `docs/install.md` — the one-hook bullet (delta 3).
- `gate-sdk/gate-tests/check-hook-knobs-fresh/good/` — the four pages and the `args` comment (delta 3).
- `.workflow/release-declarations.md` — *Behavior changes* gains the widening and the user install's matcher (deltas 1 and 2).
- `docs/gate-sdk/SPEC.md` and `docs/guard-kit/SPEC.md` — generated mirrors (all deltas).
- `docs/delegation-kit/SPEC.md` — generated mirror (all deltas).
- `docs/llms-full.txt`, `docs/requirements.md`'s install-size block and `.workflow/surface-ceiling.txt` — generated, each regenerated by the command its freshness gate prints on red (all deltas).

## Retired spellings

- `one tree-free member` — delegation-kit/SPEC.md's "its table's one tree-free member" and the good fixture's `args` comment, re-phrased by deltas 1 and 3. gate-sdk/SPEC.md's sentence naming `subagent-stop-liveness` as the column's lone member is re-phrased by delta 1.
- `only_the_turn_end_liveness_member_is_tree_free` — the unit test's name, retired by delta 3.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for the widened column, the carried fault, each member's path checks, the matcher column and the per-member knob text.
- [ ] **Instruction surfaces: instruction only** — each hook page states what its member does and links its owning section, with no grounds.
- [ ] **Merged with no information lost** — each passage is re-phrased in place, not appended to.
- [ ] **Amendment deleted** — this file removed on merge (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — `## Retired spellings` holds.
- [ ] **Run once** — `init --user wakeup-guard` in a scratch dir outside any repository, and its printed entry fired with a wake payload. It exits 2 and the state dir's log records the attempt, and the printed group carries the wake matcher.
- [ ] **Push need read** — the build adds no host-specific path handling. Every new check rides `walk::host_absolute` and the installer's existing spelling, so no mid-iteration push is owed and the closing push reads the Windows legs. A build that adds a host-specific path derivation reopens this item.
- [ ] **Entry moved** — `tree-free-hook-roster-widening` moves to Done before the drain stage, in the commit that lands the merge.
- [ ] **Gaps filed** — the verdict's usage-file dirname split on `/` alone, which misreads a backslash-only Windows usage path, filed as a debt task, and any other cross-component gap discovered during the work filed as one.
