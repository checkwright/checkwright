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
2. **The untracked corpus is read in place.** A dispatcher names each untracked or gitignored path absolute into the main checkout. The child reads a named file there through the harness's read tool, and searches under it with a read-only pipeline, the one shell form rule 27 admits over a main-checkout path. Cost (3)'s "not delegable at all" clause retires.
3. **Confinement is a chokepoint, not a sentence.**
   - **D4.** A new dispatch rule, armed by a consumer's `DELEGATION_KIT_MUTATING_TYPES`, blocks an unisolated dispatch of any type outside that roster.
   - **File-tool writes: the harness's check, kept rather than duplicated.** The harness already refuses an isolated child's file-tool write to any main-checkout path, measured below. A kit hook doing the same would be a second copy that could only disagree.
   - **Rule 27, the journal, and the admitted read.** A new guard-kit rule refuses a Bash command from a linked-worktree session that names a main-checkout path outside the main scratch dir, unless the command is a read in the one form unable to write there. So an isolated child's one sanctioned write, its journal, goes by a shell append into that scratch dir, and its search of an untracked corpus goes by a read-only pipeline.

   **Escalated pending design (2026-09-23), resolved at align's judgment-tier re-dispatch the same day.** Align's own read probe (above) found rule 27 as drafted over-refuses: Grep and Glob are absent from this harness build's toolset for an isolated child, so a search over the untracked corpus (as opposed to a read of one named path) has no route but Bash, and rule 27 blocked every Bash command naming a main-checkout path outright. The design below and delta 4's rule-27 text carry the resolution.

   **Operator direction, 2026-09-23, lead-relayed, widening rule 27 and this delta's envelope:** "Instead of fully blocking bash we could allow only certain read-only bash commands that we can advertise to sub-agent as available and the error message when running other commands could suggest the same." Concretely, rule 27's read half is no longer a blanket refusal:
   - it admits a declared set of read-only commands over main-checkout paths for an isolated child, and refuses everything else;
   - the admitted set is advertised to the child — the read-only agent type's definition and/or the dispatch protocol names it as available;
   - the refusal message names the admitted set as the lawful alternative (guard-kit already requires every block message to name one);
   - whether the set is kit-shipped or consumer-bound is checked against doctrine-kit's Policy-as-choice and De-literalization rules, and whichever the owning surfaces rule is what lands;
   - the write half of rule 27 is unchanged.

   **Addendum, 2026-09-23, lead-relayed, operator DIRECTION: the allowlist is stated interim.** The read-only Bash allowlist above is this unit's whole answer and stands as such — it is explicitly not the operator's longer track, which is side-effect-free tools of this project's own, replacing Bash for read-only agents, seeded from gate-sdk's `FENCE_SAFE_ARMS` set (gate-sdk/SPEC.md §The arm table: no network spawn, writes nowhere but its working tree and stdout). That track is filed by the lead as a Deferred entry, not built in this unit. The redesign owed here (above) states the allowlist as interim and names the future tools track as its planned successor, so the advertised set and the refusal message can later point at those tools instead. Nothing for the tools track is authored here.

   **Operator direction, 2026-09-23, lead-relayed: a dual-use utility is never admitted on its name.** "Shells like Bash can offer utilities that have side effects, e.g. we expect a utility to be used for read-only operations while it can also write." So the admitted form must be unable to write where rule 27 refuses: `sed -i`, `find -delete`/`-exec`, `awk`'s `system()` and a redirect or `tee` are each the same utility a read uses.

   **The design, authored at the re-dispatch.** Delta 4 carries the rule text; the rulings it rests on:
   - **One definition of read-only, not a second roster.** The admitted read is rule 18's read-only test, the one the ruleset already grants on. Every segment leads with a `GUARD_KIT_RO_BINS` member invoked in none of its declared write and execute forms, and an undeclared member is withheld, so admission reads the invocation and never the name alone. Two clauses are rule 27's own:
     - A redirect target may also lie in the session's own worktree or under a main-checkout scratch dir, the two places rule 27 does not refuse a write.
     - No segment may lead with a program-bearing tool, meaning rule 8's walker rows and every `GUARD_KIT_SCRIPT_INTERPRETERS` member. Such a tool's write form lives in its program text, so no declaration of it can be true, and a consumer who rosters `awk` or `sed` still gets no admission.

     `sed`, `awk`, `tee` and the interpreters are off the default roster already. Measured, rule 18 already grants these reads from a linked worktree ahead of the appended rule: a `grep -rn`, a `find -name`, an `ls` and a piped `head` over main-checkout paths were each granted, and `find -delete` fell through.
   - **Kit-shipped against consumer-bound, ruled on the owning surfaces.** The predicate is kit grammar and not policy, since another value breaks the confinement rather than a taste (doctrine-kit Policy-as-choice's "not policy" class). That covers the path-word resolution, the scratch and own-worktree exemptions, rule 18's declaration grammar and the program-bearing exclusion. *Which* utilities are admitted is a calibration, and it is already consumer-bound: `GUARD_KIT_RO_BINS` and `GUARD_KIT_RO_FORMS` carry the kit's defaults and the consumer's value. Minting a second roster for rule 27 is refused on De-literalization's one-owner rule, because "read-only" would then have two owners that drift. *Whether* shell reads of the main checkout are admitted at all is a calibration whose `off` leaves the write half standing, so Policy-as-choice owes it a selector. That is the new `GUARD_KIT_WORKTREE_READS`, `read-only` or `off`, defaulting to `read-only` on the measured ground that a harness build may carry no search tool, and with none `off` leaves a search of the untracked corpus with no route.
   - **The advertisement is derived, never copied.** The refusal message is the one carrier of the members. It interpolates the loaded `GUARD_KIT_RO_BINS` at the call, so it names the live roster and holds no literal. The read-only type's definition and the protocol template name the *class* — a read-only pipeline of the guard's read-only roster — and say the refusal names the members, so neither restates a roster value. Because the set is the roster's content, a later side-effect-free replacement for a dual-use utility reaches every carrier by entering the roster, with no message edit.
   - **Composition with D4 and D5.** D4 sends every undeclared type into a worktree, and rule 27's admitted read is what keeps that default workable for a sweep over an untracked corpus. Without it, D4 would push such a sweep into a worktree whose shell cannot search the corpus, rebuilding the pressure toward the named mutating-type escape D4 exists to make visible. Rule 27 keys on the session's working tree, never on `agent_type` or any dispatch field. So it confines a mutating type dispatched isolated exactly as it confines a read-only one, and it shares no knob with D4's roster. D5 is orthogonal: rule 27 reads no `model`. The one surface the two meet on is the read-only type's definition, which D5 obliges to state `model:` and which now also names the admitted class. A built-in type with no definition gets both from its dispatch, its tier from `model` and its route from the protocol's cost (3).
   - **Composition with the journal and with the earlier rules, measured.** Two defects in the drafted text surfaced, and the design closes both.
     - **The journal was not reachable.** From a linked worktree, the journal append was blocked by rule 25 arm (b). `git check-ignore` answers a main-checkout path as "outside repository", so rule 17 declined, and rule 25 steered the child to the Write tool the harness refuses it. The drafted "journal append falls through" row was false against the tree. The ignored-target test rules 16, 17, 19 (B2) and 25 share now asks the checkout holding the target.
     - **A tail rule is reached only by what nothing earlier ended.** Every grant, rewrite and advise exits 0 and ends the hook, so a rule placed last never sees what an earlier non-block decision took. Rule 27 is appended as drafted, so nothing renumbers. Every rule whose decision is not a block takes its test as a predicate, on rule 24's pattern: rules 4, 7, 16, 17, 18 and 19 and rule 21's advise.

     With both in place the journal append is granted by rule 17 as a main-checkout session's is. A gitignored main-checkout target outside the scratch dir, such as a private brief, is withheld by rule 17 and refused by rule 27.
   - **Interim, with a named successor.** The allowlist is this unit's whole answer and stands as such. Its successor is the side-effect-free tools track in the addendum above, filed as a gap on 2026-09-23 and not built here. That track closes the honest limit the roster cannot close: a member gaining a write option in a later release is a hole until its declaration is widened.
4. **The tier is chosen.** A new dispatch rule, D5, is armed by a policy knob. It blocks a dispatch that carries no `model` and whose type has no tracked definition stating `model:`. A read-only type's stated default is its own definition's field.

**Honest limits, stated rather than left to be found**:

- **(i) A shell write the command does not name.** Rule 27 refuses a command that names a main-checkout path. A program the child runs can write any path it computes, and nothing confines that write:
  - **No `PreToolUse` rule sees it.** A rule reads only the command it is shown.
  - **The harness sandbox does not confine it.** The sandbox's write-root is the session's project directory, not the child's worktree: an isolated child's shell `touch` into the main checkout's root was allowed under a strict sandbox (below). A binding would bound such a write to the repository and no tighter, so none is made.
- **(ii) A mutating type chosen for read-only work.** A dispatcher can still pick a declared mutating type for read-only work. D4 makes that a named choice rather than a silent one, and no payload field carries intent (§The delegation model already refuses keying on prompt text).
- **(iii) Gitignored overlays.** A worktree verdict omits the main checkout's gitignored overlays — a `.local.knobs` file, a local pattern list — so it can be greener than the same verdict in the main checkout.
- **(iv) The file-tool refusal is the harness's.** It was measured in a session loading no project settings, so it is read as harness behavior. A harness revision can move it, and nothing in this tree would notice. That is the footing §The delegation model already records for the hook payload roster.
- **(v) The admitted read is as safe as the roster's declarations.** This is rule 18's own limit, inherited. A roster member that gains a write option in a later release is a hole until its declaration is widened, and a platform spelling the option differently is a hole until that spelling is declared too. The program-bearing exclusion closes the class whose writes live in program text. Only a tool whose effect is its command line closes the rest, and that is the successor track's, not this unit's.
- **(vi) The members reach the child at its first refusal.** The definition and the protocol name the class. The roster's members arrive in the refusal, so a child that guesses outside the class spends one call learning them.
- **(vii) A consumer rule is not bounded by rule 27.** Consumer rules dispatch ahead of the generic ruleset (guard-kit/SPEC.md §Consumer rules), so a consumer grant of a main-checkout write from a linked worktree is its author's to bound.
- **(viii) The harness's search tools are unmeasured here.** Where `GUARD_KIT_SEARCH_TOOLS` names Glob or Grep, rules 9 and 11 steer a listing-only `find` and a working-tree `git grep` to those tools ahead of rule 27. Whether they reach a main-checkout path from an isolated child is not measured: the measured build carried neither tool.

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
- **Measured (2026-09-23), by the lead, operator-approved** (`claude -p --setting-sources "" --permission-mode acceptEdits` dispatching one `isolation: worktree` `general-purpose` child to Read an untracked main-checkout file and use its Grep and Glob tools on a gitignored one): Read on a named main-checkout path is **allowed**. Grep and Glob are **absent from this harness build's toolset** — "tool not available in this session" — which is a missing-capability answer, not a path refusal. So a child that needs to *search* the untracked corpus rather than read one named path has no tool for it, and reaches for Bash — which guard rule 27 as first drafted refused outright for any command naming a main-checkout path. Resolved by the admitted read — see "Confinement is a chokepoint, not a sentence", above, and delta 4.
- **The guard from a linked worktree**, by align's judgment-tier re-dispatch (2026-09-23). The committed guard was driven with Bash payloads from the working tree of a scratch `git worktree add --detach`, which it later removed, with rule 27 not yet built:
  - **The journal append was blocked.** `printf … >> <main>/.tmp/<j>` and the `cat >> … <<'EOF'` form were each blocked by rule 25 arm (b). `git check-ignore` from the worktree exits 128 on a main-checkout path ("outside repository"), and git's text leaks into the block message.
  - **Read-only pipelines were granted by rule 18.** `grep -rn`, `grep … | head`, `find … -name` and `ls` over main-checkout paths were each granted.
  - **Writes fell through, with one exception.** `find … -delete`, `touch <main>/<x>` and `git -C <main> commit --amend` each fell through, since rule 21 does not fire through `-C`. `sed -i` met rule 8.
  - **A record in the main checkout was refused.** A recorded launch writing its record to `<main>/.tmp/<key>.run` was blocked by rule 15, whose corrective names the worktree's scratch dir. That is a pre-existing gap outside this unit, filed to the gap inbox.

## What changes

### (1) The verdict binary inside a linked worktree {design-bearing}

**Applied.** The shared linked-worktree test is factored into `_gate_main_checkout_bin`; a declined link prints a second line naming the refused condition, which the front end's message carries. The parity corpus flips its `--emit` case to the unignored-door refusal and adds a linking case with a worktree `.gitignore`.

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

**Applied.** In `native/src/gates/crate_arms.rs`, inside a linked worktree, a miss in the local cache is looked up in the main checkout's scratch dir under the same record name. The record is keyed on source stamp and toolchain, so sharing it is sound. A miss in both is exit 2, naming the rule, and cargo is never run. In gate-sdk/SPEC.md §check-crate-arms, beside the cache paragraph, add:

> **In a linked worktree the cache is read from the main checkout too, and a miss there is a refusal rather than a build**: a worktree is where a delegated read-only session runs, and a build is the mutation isolation exists to prevent (delegation-kit/templates/agent-execution.md, isolation cost (4)).

### (3) The isolation costs rewritten {design-bearing}

**Not yet applied.** In `delegation-kit/templates/agent-execution.md`:

- The bullet title's "Isolation charges four harness costs" becomes "Isolation charges five harness costs", the count its own list holds.
- Cost (3)'s sentences from "**A sweep whose corpus includes an untracked or gitignored surface is not delegable to an isolated agent**" through "`git check-ignore` decides it." become:

  > **A sweep whose corpus includes an untracked or gitignored surface names that surface absolute into the main checkout** in the prompt. The child reads a named file there with the harness's read tool, and searches under it only with a read-only pipeline of the shell guard's read-only roster. Under guard-kit, its rule *A main-checkout path from a linked worktree* admits that form and refuses every other shell command naming the main checkout outside its scratch dir, and its refusal names the roster. Tell the child so in the prompt when the type's definition does not. Classify the corpus before you dispatch rather than after you read the answer; `git check-ignore` decides it.

- Cost (4)'s heading sentence and body up to "**The dispatcher's half, on (3)'s pattern:**" become:

  > **(4) A gate dispatched to a compiled binary resolves inside an isolated worktree when the main checkout's binary matches the worktree's source, and never by a build.** The front end links that binary into the worktree after comparing source stamps (gate-sdk/SPEC.md §lib/gate.sh, `gate_verdict_bin`); a skewed or absent one leaves the gate unavailable, which inside isolation is **the expected reading, not a defect to repair**: do not read it as a verdict, and **do not build the binary**. Name the gate that could not run, say why, and return. A worktree verdict omits the main checkout's gitignored overlays — a `.local.knobs` file, a local pattern list — so a sweep whose conclusion turns on one names it absolute, as (3) does.

- The dispatcher's half up to the "**One exception**" sentence becomes:

  > **The dispatcher's half:** an oracle-running sweep is delegable into isolation; classify only whether its oracle reads a gitignored overlay.

- The "**One exception**" sentences stay.

In delegation-kit/SPEC.md §The delegation model, the D2-and-cost-(3) composition paragraph — the one opening "**D2 composes with isolation's untracked-blindness cost**" — is re-phrased. The *not delegable at all* consequence is replaced by the absolute-read route and its grounds: a read of a named main-checkout path is not a write, and the harness refuses the child's file-tool writes there. Rule 27 admits a read-only pipeline over a main-checkout path and refuses every other shell command naming one outside the scratch dir. D4 sends an undeclared type into isolation, and the admitted read is what leaves that default workable for a sweep over an untracked corpus when the harness build carries no search tool.

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

- **The Bash rule.** guard-kit's generic ruleset gains rule 27, `guard_rule_worktree_confinement`, appended so nothing renumbers. From a linked-worktree session it blocks a command whose path word resolves into the main checkout outside the main scratch dir, unless the command is the admitted read. The rulings behind it are "Confinement is a chokepoint, not a sentence", above.
  - **In `guard-kit/lib/guard.sh`:**
    - the rule, dispatched last;
    - a predicate call to its test in each non-block exit of rules 4, 7, 16, 17, 18, 19 and 21;
    - the ignored-target test of rules 16, 17, 19 (B2) and 25 asking the checkout that holds the target. Build factors the five inline `git check-ignore` calls into one helper, and arm (b)'s call gains the `2>/dev/null` arm (c)'s already carries;
    - `GUARD_KIT_WORKTREE_READS` added to the knob-load name list.
  - **In `native/src/knobs/guard_kit.rs`:** the `GUARD_KIT_WORKTREE_READS` row, scalar, default `read-only`, with the kit's validator refusing any value but `read-only` and `off`, on canon-kit's enumerated-scalar precedent.

  In guard-kit/SPEC.md §The generic ruleset, append:

  > 27. **A main-checkout path from a linked worktree** (`guard_rule_worktree_confinement`) — from a session whose working tree is a linked worktree (the test `gate_harness_bin` applies, gate-sdk/SPEC.md §lib/gate.sh), a command carrying a **path word** that resolves into the main checkout is **blocked**, unless every such word lies under a `GUARD_KIT_SCRATCH_DIRS` member of the main checkout or the command is the **admitted read**. An isolated session reaches the main checkout for two things. One is its journal, which it appends under the scratch dir by shell, because the harness refuses its file-tool writes there. The other is reads: the harness's read tool takes a named file, and a search, which a harness build may carry no tool for, takes the admitted read.
  >
  > **The path word.** A path word is a word of rule 8's dequoted view, read as the skeleton's words where that view cannot be aligned, or the value of a `--name=value` word. Redirect targets and a `git -C` target are included. It resolves lexically against the working directory, with `..` components folded. It resolves into the main checkout when it lies under the main checkout's root and not under the session's own worktree root, so a sibling worktree nested there counts as the main checkout's. The roots are resolved once per hook process.
  >
  > **The admitted read** is the one shell form unable to write where this rule refuses. It must hold three things:
  > - Every segment passes rule 18's roster test: it leads with a `GUARD_KIT_RO_BINS` member invoked in none of its declared write and execute forms, with the literal `echo`/`printf` banner tolerated.
  > - No segment leads with a program-bearing tool, meaning rule 8's walker rows and every `GUARD_KIT_SCRIPT_INTERPRETERS` member. Such a tool's write form lives in its program text rather than its options, so no declaration of it can be true, whatever the roster holds.
  > - Every redirect target is inert, inside the session's own worktree, or under a main-checkout scratch dir.
  >
  > So a dual-use utility is never admitted on its name. The admission reads the invocation's forms, and an undeclared member is withheld. `GUARD_KIT_WORKTREE_READS` selects the admission. `read-only`, the default, admits it, and `off` admits none, which leaves the harness's read tool as the one route to a main-checkout file.
  >
  > **The corrective is the advertisement.** It names the offending word and what it reaches, then the lawful alternatives:
  > - the harness's read tool, for a named file;
  > - under `read-only`, the admitted read, with its roster interpolated from the loaded knob at the call, so the message never carries a copy;
  > - a history read in the session's own worktree, which shares the main checkout's refs and objects;
  > - the journal, appended by shell under the main checkout's scratch dir.
  >
  > **No rule ends the hook on a call this rule refuses.** Every rule whose decision is not a block takes this rule's test as a predicate, on rule 24's pattern: the grants and rewrites of rules 4, 7, 16, 17, 18 and 19, and rule 21's advise. The reason is that a rule placed last is reached only by the calls every earlier non-block decision declined. **The ignored-target test rules 16, 17, 19 (B2) and 25 share asks the checkout that holds the target.** From a linked worktree git answers a main-checkout path as outside the repository, which would refuse the very journal append the scratch exemption exists for.
  >
  > Declares `sq dq hd`. **Declines** where this ruleset's block rules decline: on an expansion or a substitution, which rule 6 blocks first, and on a backtick span.
  >
  > **Honest limits.**
  > - The test is lexical, so a symlink reaches past it.
  > - A program the session runs can write any path it computes. No rule reading the command sees that write, and, measured, no harness sandbox confines it to the worktree (delegation-kit/SPEC.md §The delegation model).
  > - The admitted read is exactly as safe as the roster's declarations, which is rule 18's own limit: a member gaining a write option is a hole until it is declared. Only a tool whose effect is its command line would close that.
  > - The roster reaches the session at its first refusal, not before.
  > - A consumer rule dispatches ahead of the generic ruleset, so a consumer grant is bounded by its author rather than by this rule.

  The same section's cross-references move with it:
  - Rule 18's **"One reader, four sites"** gains rule 27's admission as its fifth site, and §Layout and configuration's `GUARD_KIT_RO_FORMS` reader list gains rule 27.
  - Each of rules 4, 7, 16, 17, 18, 19 and 21 gains a one-clause pointer to rule 27's predicate.
  - Rule 16's `git check-ignore` sentence, which rules 17, 19 (B2) and 25 cite, gains the owning-checkout clause.

- **The advertisement, on the consumer's read-only type.** This is this repo's binding of the class, beside delta 6's edits to `.claude/agents/audit-sweep.md`.
  - The audit section's "A target you cannot read inside a worktree because it is untracked or gitignored is the same shape" becomes "A target your dispatch did not name absolute into the main checkout, and that you cannot read inside a worktree because it is untracked or gitignored, is the same shape".
  - A bullet follows it: "**A path your dispatch names absolute into the main checkout** you read with the Read tool, and search only with a read-only pipeline of the bash guard's read-only roster. Any other shell command naming the main checkout outside its scratch dir is refused (guard-kit/SPEC.md §The generic ruleset, rule *A main-checkout path from a linked worktree*), and the refusal names the roster's members."
  - The bullet names the class and the rule by title and holds no member, so it does not restate the knob's value.

- **Docs and tests.**
  - delegation-kit/SPEC.md §Layout and configuration lists `DELEGATION_KIT_MUTATING_TYPES`.
  - §Testing gains the payload cases in which D4 fires and falls through.
  - guard-kit/SPEC.md §Layout and configuration lists `GUARD_KIT_WORKTREE_READS`, with its two values and its default's ground. The layout tree gains the new gate-test.
  - `guard-kit/templates/guard-config.knobs` gains the commented `# GUARD_KIT_WORKTREE_READS = off` line under a `spec:` pointer to §The generic ruleset, as its `GUARD_KIT_SEARCH_TOOLS` line does, so the `off` alternative is visible where a consumer binds.
  - **Rule 27's rows run in a new bespoke `guard-kit/gate-tests/worktree-confinement.test.sh`.** It drives the template guard from a scratch linked worktree it adds and removes. That is §Testing's lane on a sixth structural ground, the session's working tree, which the decision table's sandbox cannot vary. Its rows:
    - **Allowed:** a journal append into the main scratch dir, bare and by heredoc; a `grep -rn`, a `grep … | head` and a `find … -name` over main-checkout paths; a read redirected into the session's own worktree.
    - **Blocked:** `find … -delete` and `find … -fprint` over the main checkout; `touch` of the main root; a relative `../`-traversal into it; a `<main>/.tmp/../` fold; a gitignored main-checkout target outside the scratch dir written by `printf >>`, which rule 17 withholds on the predicate; `git -C <main> commit --amend`; and a read redirected to a main-checkout path.
    - **Fell through:** a `touch` inside the own worktree.
  - `gate-tests/guard-config-knobs.test.sh` gains the selector's cases:
    - under `off`, the `grep` over a main-checkout path blocks and the journal append is still granted;
    - a consumer roster adding `awk` with a `none` declaration still has `awk` over a main-checkout path blocked by the program-bearing exclusion.
  - `guard-tests/cases.tsv` gains one non-firing row, `touch ../outside` falling through: from the table's unlinked sandbox, a path outside the root is not rule 27's.

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

> **What isolation confines, measured, and what it leaves.** An isolated child's file-tool writes into the main checkout are refused by the harness, its scratch dir included, so its journal travels by a shell append. Its shell writes are not refused: under a strict harness sandbox a child's `touch` into the main checkout's root succeeded, because the sandbox's write-root is the session's project directory rather than the worktree, and only a write outside the repository was refused. So the sandbox is not bound as a confinement — it would bound a child to the repository and no tighter. What confines the shell is guard-kit's rule 27, and only for a path the command names. It admits a read-only pipeline over the main checkout, in the one form unable to write there, and refuses every other command naming it outside the scratch dir. A program the child runs can write any path it computes. That residue, and a dispatcher's knowing choice of a mutating type for read-only work (D4 makes the choice named, not impossible), are this design's honest limits. The file-tool refusal is harness behavior, measured in a session loading no project settings; a harness revision can move it, on the footing this section records for the hook payload.

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
  - Predicate consumers: the non-block exits of rules 4, 7, 16, 17, 18, 19 and 21, each taking rule 27's test before it ends the hook.
- **`GUARD_KIT_WORKTREE_READS`** (delta 4).
  - Producer: the consumer's knob file, or the kit's default.
  - Consumer: rule 27's admission, reached through the guard's knob load.
  - Roster-holding readers: `check-knob-default-coupling` reads the Layout bullet stating the default. `--emit knob-roster` is derived.
- **The admitted roster in the refusal** (delta 4). Producer: `GUARD_KIT_RO_BINS`, as loaded at the call. Consumer: the refused session. No surface carries a copy of it.
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
- `guard-kit/SPEC.md` §Layout and configuration and §Testing, `native/src/knobs/guard_kit.rs`, `guard-kit/templates/guard-config.knobs`, `guard-kit/gate-tests/worktree-confinement.test.sh` (new) and `guard-kit/gate-tests/guard-config-knobs.test.sh` (delta 4).
- `.claude/agents/audit-sweep.md` (deltas 4 and 6).
- `scripts/delegation-config.knobs` and `.claude/agents/edit-sweep.md` (delta 7).
- `.workflow/release-declarations.md` (all deltas):
  - **Behavior changes:** verdict arms run inside a linked worktree on a stamp-matched main-checkout binary; a read-only sweep journals by shell append; from a linked worktree the guard's ignored-target test asks the checkout holding the target, and shell reads of the main checkout are admitted only as a read-only pipeline, selectable `off` through `GUARD_KIT_WORKTREE_READS`.
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
