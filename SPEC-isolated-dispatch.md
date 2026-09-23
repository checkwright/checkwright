# SPEC amendment: isolated-dispatch

A read-only sweep has no lawful way to do the work it is dispatched for.

- **Isolation has no oracle.** `DELEGATION_KIT_READONLY_TYPES` names `audit-sweep`, so D2 forces it into a worktree. A worktree carries no build output, and isolation cost (4) forbids building one. Only the harness arms reach the main checkout's binary (`gate_harness_bin`, gate-sdk/SPEC.md §lib/gate.sh). Every verdict-bearing arm fails closed.
- **Isolation has no untracked corpus.** Cost (3) makes an untracked or gitignored corpus "not delegable at all" for a D2 type.
- **The escape is ungated.** A dispatcher who needs either therefore reaches for a write-capable type outside the D2 roster, unisolated, and says "read-only" in the prompt. This amendment's own authoring stage did exactly that: three research children dispatched as `general-purpose`, unisolated, with no `model` override, so they inherited the dispatcher's tier. That is the claim-by-sentence delegation-kit/templates/agent-execution.md refuses, and nothing enforces the refusal.

The unit's scope carries the directions recorded on its queue entry:

- make gates workable in worktrees, or state the limitation;
- a read-only dispatch lands on a type structurally unable to alter the work tree, keeps its resume journal as its one sanctioned write, and still reaches the oracle arms and the untracked corpus it needs;
- the model tier is chosen at dispatch, never inherited, with a stated default for the read-only type and no model-name list in the kit;
- no sandbox binding, the measured residue stated as an honest limit.

**The ruling, in four parts.**

1. **Worktrees gain the oracle.** Inside a linked worktree the front end links the main checkout's binary into the worktree's own knob path, and only when that binary's source stamp equals the worktree's crate source stamp. It never builds. Every reader, shell or crate, then finds one binary at one path. check-gate-binary-fresh is the fail-closed check on skew, the property delegation-kit/SPEC.md §The turn-end liveness hook reserved a main-checkout binary against. `check-crate-arms` reads the main checkout's green cache and refuses rather than builds on a miss.
2. **The untracked corpus is read in place.** A dispatcher names each untracked or gitignored path absolute into the main checkout, and the child reads it there through the harness's read tools. Cost (3)'s "not delegable at all" clause retires.
3. **Confinement is a chokepoint, not a sentence.**
   - **D4.** A new dispatch rule, armed by a consumer's `DELEGATION_KIT_MUTATING_TYPES`, blocks an unisolated dispatch of any type outside that roster.
   - **File-tool writes: the harness's check, kept rather than duplicated.** The harness already refuses an isolated child's file-tool write to any main-checkout path, measured below. A kit hook doing the same would be a second copy that could only disagree.
   - **Rule 27, and the journal.** A new guard-kit rule refuses a Bash command from a linked-worktree session that names a main-checkout path outside the main scratch dir. So an isolated child's one sanctioned write, its journal, goes by a shell append into that scratch dir.

   **Escalated pending design (2026-09-23).** Align's own read probe (above) found rule 27 as drafted over-refuses: Grep and Glob are absent from this harness build's toolset for an isolated child, so a search over the untracked corpus (as opposed to a read of one named path) has no route but Bash, and rule 27 blocks every Bash command naming a main-checkout path outright — a finding against this delta and rule 27's text below.

   **Operator direction, 2026-09-23, lead-relayed, widening rule 27 and this delta's envelope:** "Instead of fully blocking bash we could allow only certain read-only bash commands that we can advertise to sub-agent as available and the error message when running other commands could suggest the same." Concretely, rule 27's read half is no longer a blanket refusal:
   - it admits a declared set of read-only commands over main-checkout paths for an isolated child, and refuses everything else;
   - the admitted set is advertised to the child — the read-only agent type's definition and/or the dispatch protocol names it as available;
   - the refusal message names the admitted set as the lawful alternative (guard-kit already requires every block message to name one);
   - whether the set is kit-shipped or consumer-bound is checked against doctrine-kit's Policy-as-choice and De-literalization rules, and whichever the owning surfaces rule is what lands;
   - the write half of rule 27 is unchanged.

   **Addendum, 2026-09-23, lead-relayed, operator DIRECTION: the allowlist is stated interim.** The read-only Bash allowlist above is this unit's whole answer and stands as such — it is explicitly not the operator's longer track, which is side-effect-free tools of this project's own, replacing Bash for read-only agents, seeded from gate-sdk's `FENCE_SAFE_ARMS` set (gate-sdk/SPEC.md §The arm table: no network spawn, writes nowhere but its working tree and stdout). That track is filed by the lead as a Deferred entry, not built in this unit. The redesign owed here (above) states the allowlist as interim and names the future tools track as its planned successor, so the advertised set and the refusal message can later point at those tools instead. Nothing for the tools track is authored here.

   **Not authored here.** Choosing the admitted set's grammar, its owner (kit-shipped vs. consumer knob), and how it composes with D4/D5 and the journal-append path is design work outside align's tier on this dispatch — journaled and escalated for a judgment-tier re-dispatch rather than authored at this tier. The rule 27 spec text below (delta 4) and delta 3's summary above are therefore **unrevised and superseded pending that redesign**; do not treat them as the shipped shape of rule 27's read half.
4. **The tier is chosen.** A new dispatch rule, D5, is armed by a policy knob. It blocks a dispatch that carries no `model` and whose type has no tracked definition stating `model:`. A read-only type's stated default is its own definition's field.

**Honest limits, stated rather than left to be found**:

- **(i) A shell write the command does not name.** Rule 27 refuses a command that names a main-checkout path. A program the child runs can write any path it computes, and nothing confines that write:
  - **No `PreToolUse` rule sees it.** A rule reads only the command it is shown.
  - **The harness sandbox does not confine it.** The sandbox's write-root is the session's project directory, not the child's worktree: an isolated child's shell `touch` into the main checkout's root was allowed under a strict sandbox (below). A binding would bound such a write to the repository and no tighter, so none is made.
- **(ii) A mutating type chosen for read-only work.** A dispatcher can still pick a declared mutating type for read-only work. D4 makes that a named choice rather than a silent one, and no payload field carries intent (§The delegation model already refuses keying on prompt text).
- **(iii) Gitignored overlays.** A worktree verdict omits the main checkout's gitignored overlays — a `.local.knobs` file, a local pattern list — so it can be greener than the same verdict in the main checkout.
- **(iv) The file-tool refusal is the harness's.** It was measured in a session loading no project settings, so it is read as harness behavior. A harness revision can move it, and nothing in this tree would notice. That is the footing §The delegation model already records for the hook payload roster.

**Measured at authoring (2026-09-23).**

- **The worktree probes**, by the research dispatch in a scratch `git worktree add` it later removed:
  - The bare worktree's own front end exits 2, binary absent, on `--run`, `--only` and `--emit`.
  - The main binary invoked by relative path reads the worktree's tree and not its own location (gate-sdk/SPEC.md §Layout and configuration: the binary's path is no anchor). The battery then fails 5 of 131 with the knob unset, and 1 of 131 (`check-graph`) with the knob exported absolute, because the hook emitter bakes the knob's text. In both runs `check-crate-arms` missed its gitignored cache and ran cargo into the worktree, about 50 MB.
  - With the main binary linked at the worktree's default knob path and the crate-arms cache record copied, `bash gate-sdk/bin/run-gates.sh --run` passed 131 of 131 in 13s with no cargo run.
  - One appended line in the worktree's `native/src/main.rs` turned `check-gate-binary-fresh` red.
  - The main binary's `--source-stamp` equalled the worktree's `gate_native_source_stamp` on a clean main crate.
- **The confinement probe**, run by the lead with the operator's approval. It used a nested `claude -p` with no project settings, a strict sandbox (`enabled`, `failIfUnavailable`, `allowUnsandboxedCommands: false`) and one `isolation: worktree` child.
  - **Shell writes.**
    - A shell `touch` into the worktree, into the main checkout's root and into its scratch dir was each allowed.
    - A `touch` outside the repository was refused: "Read-only file system".
  - **File-tool writes.** A Write-tool write to the main checkout's root, and one to its scratch dir, was each refused by the harness: "This agent is isolated in the worktree …; Edit the worktree copy of this file instead of the shared-checkout path."
  - **The base.** The child's worktree base was `origin/master`, not the dispatcher's HEAD. That is isolation cost (1)'s default base, in a session loading no project `worktree.baseRef`.
- **Harness hooks run with the worktree as cwd.** The gate-sdk-blind-spots finding that every binary-backed hook fell open inside an isolated dispatch is that fact seen from its other side.
- **Measured (2026-09-23), by the lead, operator-approved** (`claude -p --setting-sources "" --permission-mode acceptEdits` dispatching one `isolation: worktree` `general-purpose` child to Read an untracked main-checkout file and use its Grep and Glob tools on a gitignored one): Read on a named main-checkout path is **allowed**. Grep and Glob are **absent from this harness build's toolset** — "tool not available in this session" — which is a missing-capability answer, not a path refusal. So a child that needs to *search* the untracked corpus rather than read one named path has no tool for it, and reaches for Bash — which guard rule 27 as drafted refuses outright for any command naming a main-checkout path. **Escalated pending design (2026-09-23)** — see below, after "Confinement is a chokepoint, not a sentence."

## What changes

### (1) The verdict binary inside a linked worktree {design-bearing}

**Not yet applied.**

- **The function.** `gate-sdk/lib/gate.sh` gains `gate_verdict_bin`. When `gate_native_bin_spelled` is executable, it returns that. Otherwise, inside a linked worktree whose common dir is `<main>/.git` (`gate_harness_bin`'s test), it links the main checkout's own resolution of the knob into the local door path and returns the door. It links only when all four hold:
  - that binary is executable;
  - the local door path is gitignored (`git check-ignore`);
  - where the worktree carries `GATE_SDK_NATIVE_CRATE`, the binary's `--source-stamp` equals `gate_native_source_stamp` over it;
  - where it carries none, the binary is the installed payload of the same checkout and no stamp is compared.

  The link is a symlink, falling back to a copy. Otherwise it returns the door unchanged. It never builds.
- **The caller.** `gate-sdk/bin/run-gates.sh` `exec_arm`'s verdict branch calls it where it called `gate_native_bin_spelled`.
- **The absent-binary message.** Inside a linked worktree the message names the refused condition (skew, a tracked door path, no main binary) and the remedy "run it in the main checkout". It never says "Build it", which would build in the worktree.
- **The PowerShell twin.** `gate-sdk/bin/run-gates.ps1` mirrors all three, under the front-end parity cases.
- **The tests.** `gate-sdk/gate-tests/run-gates-linked-worktree.test.sh` gains three cases: a matching stamp links and runs, skew refuses at 2 without linking, and nothing is built. Its assertion that `--emit` reports the binary absent flips to "absent only on skew". So does the matching case in `native/src/emit/front_end_parity.rs`.

In gate-sdk/SPEC.md §lib/gate.sh, after the `gate_harness_bin` bullet, add:

> - **`gate_verdict_bin` answers which binary a verdict-bearing arm runs.** The local door when executable; inside a linked worktree otherwise, the main checkout's own resolution of the knob, **linked into the local door path** — so every reader, the shell's and the crate's, finds one binary at the path the knob names — when that binary's source stamp equals the worktree's crate source stamp and the door path is gitignored; the door unchanged otherwise, so the caller fails closed. A tree carrying no crate source compares no stamp: its binary is the installed payload of the same checkout. It links and never builds. Where `gate_harness_bin` is unconditional because a hook enforces the parent's policy, this one is stamp-gated because a verdict is about the tree: a binary built from other source would answer for code the worktree does not hold, which is what `check-gate-binary-fresh` exists to catch and still catches.

In gate-sdk/SPEC.md §The harness-integration arm, "Every verdict-bearing path keeps resolving locally, failing closed, so the liveness hook's ruling that a main-checkout binary is never how the binary knob resolves (delegation-kit/SPEC.md §The turn-end liveness hook) stands for everything this branch does not reach." becomes:

> A verdict-bearing path takes the main checkout's binary only through `gate_verdict_bin`'s stamp-gated link (§lib/gate.sh), and fails closed otherwise.

### (2) check-crate-arms never builds in a linked worktree {design-bearing}

**Not yet applied.** In `native/src/gates/crate_arms.rs`, inside a linked worktree, a miss in the local cache is looked up in the main checkout's scratch dir under the same record name. The record is keyed on source stamp and toolchain, so sharing it is sound. A miss in both is exit 2, naming the rule, and cargo is never run. In gate-sdk/SPEC.md §check-crate-arms, beside the cache paragraph, add:

> **In a linked worktree the cache is read from the main checkout too, and a miss there is a refusal rather than a build**: a worktree is where a delegated read-only session runs, and a build is the mutation isolation exists to prevent (delegation-kit/templates/agent-execution.md, isolation cost (4)).

### (3) The isolation costs rewritten {design-bearing}

**Not yet applied.** In `delegation-kit/templates/agent-execution.md`:

- The bullet title's "Isolation charges four harness costs" becomes "Isolation charges five harness costs", the count its own list holds.
- Cost (3)'s sentences from "**A sweep whose corpus includes an untracked or gitignored surface is not delegable to an isolated agent**" through "`git check-ignore` decides it." become:

  > **A sweep whose corpus includes an untracked or gitignored surface names that surface absolute into the main checkout** in the prompt, and the child reads it there with the harness's read tools — never through a shell, which the confinement rule refuses to point at the main checkout. Classify the corpus before you dispatch rather than after you read the answer; `git check-ignore` decides it.

- Cost (4)'s heading sentence and body up to "**The dispatcher's half, on (3)'s pattern:**" become:

  > **(4) A gate dispatched to a compiled binary resolves inside an isolated worktree when the main checkout's binary matches the worktree's source, and never by a build.** The front end links that binary into the worktree after comparing source stamps (gate-sdk/SPEC.md §lib/gate.sh, `gate_verdict_bin`); a skewed or absent one leaves the gate unavailable, which inside isolation is **the expected reading, not a defect to repair**: do not read it as a verdict, and **do not build the binary**. Name the gate that could not run, say why, and return. A worktree verdict omits the main checkout's gitignored overlays — a `.local.knobs` file, a local pattern list — so a sweep whose conclusion turns on one names it absolute, as (3) does.

- The dispatcher's half up to the "**One exception**" sentence becomes:

  > **The dispatcher's half:** an oracle-running sweep is delegable into isolation; classify only whether its oracle reads a gitignored overlay.

- The "**One exception**" sentences stay.

In delegation-kit/SPEC.md §The delegation model, the D2-and-cost-(3) composition paragraph — the one opening "**D2 composes with isolation's untracked-blindness cost**" — is re-phrased. The *not delegable at all* consequence is replaced by the absolute-read route and its grounds: a read of a named main-checkout path is not a write, the harness refuses the child's file-tool writes there, and rule 27 refuses a shell command naming a main-checkout path outside the scratch dir.

In the same section, §The turn-end liveness hook's closing narrowing — "That is emphatically not true of gates in general … which is why it belonged in one consumer front end for one gate and never in how the binary knob resolves" — becomes:

> That is not true of gates in general — `check-gate-binary-fresh` exists precisely to compare a binary against the source in its own tree — which is why a verdict-bearing arm takes a main-checkout binary only through a stamp comparison that reproduces that gate's own question (gate-sdk/SPEC.md §lib/gate.sh, `gate_verdict_bin`), and never unconditionally.

### (4) Confinement at the chokepoints {design-bearing}

**Not yet applied.**

- **D4, default isolation.** In delegation-kit's `agent-dispatch-guard` (`native/src/hook/dispatch.rs`, beside D1 to D3), D4 runs after D2 and fires when all three hold:
  - `tool_input.isolation` is not `worktree`;
  - `DELEGATION_KIT_MUTATING_TYPES` is non-empty;
  - `tool_input.subagent_type` is not a member.

  The decision is block, and the message names the two lawful shapes: isolate the dispatch, or dispatch a declared mutating type. The knob is indexed and empty by default, so D4 is inert until configured, as D2 is.

  In delegation-kit/SPEC.md §The delegation model, the rule table gains:

  > | D4 | default isolation | `DELEGATION_KIT_MUTATING_TYPES` is non-empty, `tool_input.subagent_type` is not a member, and `tool_input.isolation` is not `worktree` | block |

  Beside it goes a paragraph stating the ground: D2 holds a named read-only type to isolation, and D4 closes the complement. A type nobody declared mutating is confined by default, so reaching past isolation takes a named choice.

- **The Bash rule — superseded pending redesign (see "Escalated pending design", above).** The text below blocks every Bash command naming a main-checkout path, read or write alike. Align's probe found that over-refuses reads: Grep and Glob are absent from an isolated child's toolset, so a Bash-only search over the untracked corpus has no lawful route under this text, and the operator has directed a widened read half (a declared, advertised, read-only command allowlist) that a judgment-tier session still owes. The write half — blocking everything outside the scratch-dir journal append — is unchanged by that direction.

  guard-kit's generic ruleset gains rule 27, `guard_rule_worktree_confinement`, appended so nothing renumbers. From a linked-worktree cwd, it blocks a command carrying a path token, `git -C` target included, that resolves into the main checkout outside the main scratch dir. The corrective names the read tools for a read and the scratch-dir journal for a write.

  In guard-kit/SPEC.md §The generic ruleset, append:

  > 27. **A main-checkout path from a linked worktree** (`guard_rule_worktree_confinement`) — a command run from a linked worktree whose path token resolves into the main checkout, outside the main checkout's scratch dir, is blocked, `git -C` included: an isolated session reaches the main checkout for its journal, which it appends there by shell because the harness refuses its file-tool writes to the main checkout, and for reads, which the harness's read tools do. Declares `sq dq hd`. **Honest limit:** a read and a write are one token here, so a shell read of the main checkout is refused with the write; and a program the session runs can write any path it computes, which no rule reading the command sees and — measured — no harness sandbox confines to the worktree (delegation-kit/SPEC.md §The delegation model).

- **Docs and tests.**
  - delegation-kit/SPEC.md §Layout and configuration lists `DELEGATION_KIT_MUTATING_TYPES`.
  - §Testing gains the payload cases in which D4 fires and falls through.
  - `guard-kit/guard-tests/cases.tsv` gains rule 27's rows, driven from a scratch linked worktree in `guard-kit/gate-tests/`, because the table's sandbox is not one. The rows cover a journal append into the main scratch dir falling through and a `touch` of the main root blocking.

### (5) The tier is chosen at dispatch {design-bearing}

**Not yet applied.** `agent-dispatch-guard` gains D5, armed by `DELEGATION_KIT_REQUIRE_TIER` (`on` or `off`, default `off`, the knob-file validator refusing any other value). When on, it blocks a dispatch whose `tool_input.model` is absent and whose `tool_input.subagent_type` has no definition under `DELEGATION_KIT_AGENT_DIR` stating `model:`. D1 owns `fork`.

The message:

- tells the dispatcher to name a model for this dispatch, or to dispatch a type whose definition states one;
- names the rule that selection is affirmative (agent-execution.md, **Match the dispatched model and effort to the unit's shape**);
- carries no model name, because the value is the consumer's.

The rule table gains:

> | D5 | chosen tier | `DELEGATION_KIT_REQUIRE_TIER` is `on`, `tool_input.model` is absent, and the dispatched type has no definition under `DELEGATION_KIT_AGENT_DIR` stating `model:` | block |

Beside it goes a paragraph: the read-only type's stated default is its own definition's `model:` field, which `check-agent-tier-explicit` already requires every tracked definition to carry. A built-in type with no definition names its tier at every dispatch. The kit ships no model name, because the harness roster churns.

### (6) The read-only sweep keeps a journal, by shell {mechanical}

**Not yet applied.**

- **The template.** In `delegation-kit/templates/agent-execution.md`'s resume-journal bullet:
  - "The agent `Write`s a running progress journal" becomes "The agent writes a running progress journal — an isolated agent by a shell append, since the harness refuses its file-tool writes to the main checkout —".
  - "So: for a **read-only fan-out** (audit, survey), the return value *is* the contract — don't rely on a journal. Reserve the **journal** for agents that **mutate files**, and for those grant the journal path explicitly before dispatch rather than assuming the write succeeds" becomes:

    > So: grant a journal path explicitly before dispatch to every agent whose run an interruption could cost — a mutating agent, and a read-only sweep alike — rather than assuming the write succeeds; for a read-only fan-out the return value stays the contract, and its journal is the record an interruption leaves.

  - "The child owes no journal, and the **parent** owes" becomes "The child journals as it goes, and the **parent** still owes".
  - In the next bullet, disposition (2)'s "which is the read-only carve-out above" becomes "which a short read-only fan-out still is".

- **The agent definition.** In `.claude/agents/audit-sweep.md`:
  - §Return contract's "You owe no resume journal: the journal mechanics are written for a mutating agent, and for a read-only fan-out the return value *is* the contract" becomes "Append each finding, as you confirm it, to the resume journal your dispatch grants, absolute into the main checkout's scratch dir, by a shell append: the harness refuses your file-tool writes outside your worktree. Your final message is still the whole contract, and the journal is what an interruption leaves".
  - The audit section's "inside a worktree a compiled gate is reported unavailable, never built" becomes "inside a worktree a compiled gate runs when the main checkout's binary matches your source, and is otherwise reported unavailable, never built".

- **The owning SPEC.** In delegation-kit/SPEC.md:
  - **The near-loss paragraph and §Resume journal's caveat paragraphs.** These are re-phrased. The carve-out is no longer "kept": the read-only child journals, and the parent's durability duty stands beside it unchanged, since the caveat's content — do not make recoverability depend on a child's write — is still true of the parent's end.
  - **The narrowing paragraph.** Its "A worktree-isolated agent dispatched with an absolute path in the **main checkout** wrote it successfully" becomes "A worktree-isolated agent reaches an absolute path in the **main checkout** by a shell write and not by a file-tool write, which the harness refuses (measured 2026-09-23)". The paragraph's fairness argument is unchanged, since a stage session's entry assertion reads journals written by unisolated sessions.

### (7) This repo's binding {mechanical}

**Not yet applied.**

- `scripts/delegation-config.knobs` binds `DELEGATION_KIT_MUTATING_TYPES[] = stage-session` and `DELEGATION_KIT_MUTATING_TYPES[] = edit-sweep`, and `DELEGATION_KIT_REQUIRE_TIER = on`.
- A new `.claude/agents/edit-sweep.md` is the declared mutating sweep type. Its definition states its `model:` and its commit discipline by pointer to agent-execution.md.
- No settings file changes, and no host dependency is added.

### (8) The isolation boundary, measured, is stated {mechanical}

**Not yet applied.** In delegation-kit/SPEC.md §The delegation model, beside D4, add:

> **What isolation confines, measured, and what it leaves.** An isolated child's file-tool writes into the main checkout are refused by the harness, its scratch dir included, so its journal travels by a shell append. Its shell writes are not refused: under a strict harness sandbox a child's `touch` into the main checkout's root succeeded, because the sandbox's write-root is the session's project directory rather than the worktree, and only a write outside the repository was refused. So the sandbox is not bound as a confinement — it would bound a child to the repository and no tighter. What confines the shell is guard-kit's rule 27, and only for a path the command names; a program the child runs can write any path it computes. That residue, and a dispatcher's knowing choice of a mutating type for read-only work (D4 makes the choice named, not impossible), are this design's honest limits. The file-tool refusal is harness behavior, measured in a session loading no project settings; a harness revision can move it, on the footing this section records for the hook payload.

## Producers and consumers

- **The linked binary** (delta 1).
  - Producer: `gate_verdict_bin`, at a verdict arm's first call in a linked worktree.
  - Consumers: every reader of the knob's path, shell or crate, and `check-gate-binary-fresh`, which re-verifies it.
  - The link lives in gitignored build output inside a worktree the harness deletes.
- **The shared cache record** (delta 2). Producer: the main checkout's green `check-crate-arms` run. Consumer: the worktree's run, read-only.
- **D4 and D5, and their knobs** (deltas 4 and 5).
  - Producer: `agent-dispatch-guard`.
  - Consumer: the dispatching session, through the block message.
  - Roster-holding readers: `check-knob-default-coupling` reads the Layout bullets, which state both defaults. `--emit knob-roster` is derived. `templates/delegation-config.knobs` is a one-line pointer.
- **Rule 27** (delta 4).
  - Producer: the harness's Bash `PreToolUse` call from a linked-worktree session.
  - Consumer: that session.
  - Roster-holding reader: `check-guard-registration` holds rule 27's number, function and dispatch position equal.
- **The journal grant** (delta 6). Producer: the dispatcher. Consumers: the read-only child, which appends by shell, and a cold reader after an interruption.
- **Point 5.** Delta 3 widens what an isolated child may read, and no corpus narrows.
- **Point 6.** The obliged corpus is the tracked agent definitions under D5, and `check-agent-tier-explicit` already holds each one to a stated `model:`.

## Existing sections updated

Roster from `grep -n "gate_harness_bin\|four harness costs\|not delegable\|owes no resume journal\|never in how the binary knob resolves\|Reserve the \*\*journal\*\*\|wrote it successfully\|agent \`Write\`s" -r gate-sdk delegation-kit guard-kit .claude/agents`, `git grep -n "worktree" -- '*/SPEC.md'` read for the flipped claims, and the research dispatch's surface list, run 2026-09-23.

- `gate-sdk/lib/gate.sh` and `gate-sdk/bin/run-gates.sh` (delta 1).
- `gate-sdk/bin/run-gates.ps1` (delta 1).
- `gate-sdk/gate-tests/run-gates-linked-worktree.test.sh` and `native/src/emit/front_end_parity.rs` (delta 1).
- `gate-sdk/SPEC.md` §lib/gate.sh and §The harness-integration arm (delta 1).
- `native/src/gates/crate_arms.rs` and `gate-sdk/SPEC.md` §check-crate-arms (delta 2).
- `delegation-kit/templates/agent-execution.md` (deltas 3 and 6).
- `delegation-kit/SPEC.md` §The delegation model and §The turn-end liveness hook (deltas 3, 4, 5 and 8).
- `delegation-kit/SPEC.md` §Resume journal, §Layout and configuration and §Testing (deltas 4, 5 and 6).
- `native/src/hook/dispatch.rs` and `native/src/knobs/delegation_kit.rs` (deltas 4 and 5).
- `guard-kit/lib/guard.sh`, `guard-kit/SPEC.md` §The generic ruleset and `guard-kit/guard-tests/cases.tsv` (delta 4).
- `.claude/agents/audit-sweep.md` (delta 6).
- `scripts/delegation-config.knobs` and `.claude/agents/edit-sweep.md` (delta 7).
- `.workflow/release-declarations.md` (all deltas):
  - **Behavior changes:** verdict arms run inside a linked worktree on a stamp-matched main-checkout binary; a read-only sweep journals by shell append.
  - **Tightened gates:** D4, D5 and rule 27, each inert until configured or outside a linked worktree.
<!-- update-target-exempt: generated mirrors, regenerated by their freshness gate's printed command -->
- `docs/gate-sdk/SPEC.md`, `docs/delegation-kit/SPEC.md` and `docs/guard-kit/SPEC.md`.

## Retired spellings

- None — no delta renames or deletes a spelling; the retired phrases are prose, rostered above.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The causal-completeness check holds for the link, D4, D5 and rule 27.
- [ ] **Instruction surfaces: instruction only.** Template and agent-definition text carries the act; grounds sit in the SPEC deltas.
- [ ] **Merged with no information lost.** Each SPEC edit re-phrases the passage it refines.
- [ ] **Amendment deleted.** This file is removed on merge (`ls SPEC-*.md`).
- [ ] **Entry moved.** `worktree-gate-execution` moves to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** Not reached.
- [ ] **Gaps filed.** The build files a gap bullet recording the two icebox entries' dispositions: `worktree-dispatch-rebuilds-the-gate-binary` is dissolved by deltas 1 and 2, and `worktree-isolated-dispatch-cannot-reach-the-main-checkout` keeps only its capture-log half.
