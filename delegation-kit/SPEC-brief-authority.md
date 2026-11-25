# SPEC amendment: brief-authority

Pairs `brief-authority-lint`. A new delegation-kit gate, `check-brief-authority`, reads an agent brief before it is dispatched. It reds a line granting an authority the consumer forbids, such as push, post or install, and a brief missing a section the consumer requires, such as the quoted request. The kit ships the mechanism and a starter rule list a consumer copies. Its forbidden verbs and required sections are consumer config, off by default.

**What the tree does today**, each bullet with the probe that settled it:

- No gate, hook or arm reads a brief's text against authority or section rules (`git grep -n -i 'brief' -- '*/SPEC.md' native/src/gates native/src/hook`, read over the hits: briefs are named as a dispatch's input and never as a gate's corpus).
- `agent-dispatch-guard` reads a dispatch's `subagent_type`, `isolation` and `model` and the caller's `agent_id`, and never the prompt (`grep -n 'tool_input' native/src/hook/dispatch.rs`).
- The crate matches consumer patterns through its own POSIX ERE engine, and a malformed pattern is exit 2. `check-commit-msg` and `check-tree-terms` read pattern-list files named by whitespace-list knobs, declare `# armed-by: content` over them, and clean-skip a no-argument run (`grep -n 'armed-by' gate-sdk/checks/check-commit-msg.gate`; gate-sdk/SPEC.md §check-commit-msg).
- delegation-kit ships two gates, `check-gate-tamper` and `check-agent-tier-explicit` (`ls delegation-kit/checks`).

## What changes

### (1) The gate: a brief file read against consumer rules {design-bearing} {user-facing: the entry's deliverable — where a brief is read, the config grammar and its honest limit}

**Not yet applied** — a new section `## check-brief-authority` in delegation-kit/SPEC.md, after §check-agent-tier-explicit:

> ## check-brief-authority
>
> `checks/check-brief-authority.gate` (`precommit`, binary-dispatched). Invariant: a brief grants no authority the consumer forbids and carries every section it requires. **A brief is read as a file**, named as the gate's operands: `checkwright-gates check-brief-authority <brief>…`. A dispatcher runs it on the brief it is about to hand an agent: a foreign run's prompt file, an orchestrator's brief, or a lead's dispatch prompt written out first. A no-operand run, the battery's, is a clean skip, since no brief is a tracked surface. An operand that is not a readable file is exit 2.
>
> **The rules come from `DELEGATION_KIT_BRIEF_RULES`**, a whitespace list of rule files, each line one directive and one POSIX ERE, `#` comments and blanks ignored:
>
> - `forbid <ERE>` — a brief line matching it grants a forbidden authority, unless the line also matches an `unless` rule.
> - `unless <ERE>` — a line matching it states a prohibition, not a grant. It excuses that line from every `forbid` rule.
> - `require <ERE>` — some line of the brief must match it.
>
> A line with another directive, no pattern, or a malformed pattern is exit 2, naming the file, the line and the defect. Matching runs through the crate's POSIX ERE engine (gate-sdk/SPEC.md §The POSIX ERE matcher). A named file that does not exist is exit 2. An empty knob arms nothing, and the descriptor's `# armed-by: content DELEGATION_KIT_BRIEF_RULES` lets `doctor` name that state (gate-sdk/SPEC.md §The install disposition).
>
> **The starter list is a choice, never the default.** `templates/brief-rules.list` forbids the harness-generic grants, a push, a forge post and a package install, and excuses lines phrased as a prohibition. Its `require` lines are commented out, since a required section names the consumer's brief format. A consumer copies it into its gates dir, edits it and names the copy in the knob.
>
> A finding names the brief, the line and the rule: `<brief>:<n>: grants forbidden authority (forbid <ERE>)`, or `<brief>: lacks a required section (require <ERE>)`, each with a `help:` line. The clean line is `BRIEF-AUTHORITY: clean (<n> brief(s); <f> forbid, <u> unless, <r> require rule(s))`.
>
> **Honest limits.**
>
> - **Lexical and line-scoped.** A grant phrased in words no pattern names passes, and an `unless` match excuses its whole line, so one line that forbids one act and grants another passes. Keep one instruction per line in a brief the gate reads.
> - **A brief is not a permission.** The gate reads what the brief says. What the agent can do is set by the harness's permission system and the dispatch's isolation, so a green brief does not confine anything.
> - **A dispatch payload is not read.** A brief passed inline to the harness's `Agent` tool reaches the agent without this gate unless the dispatcher writes it out and runs the gate first.

- **Why a file, and not the dispatch payload, first.** A file is the one form every dispatcher holds: a foreign run's prompt file, an outside orchestrator's brief, a prompt written out before an inline dispatch. A payload reader is a hook rule on `agent-dispatch-guard`. It needs its own decision class, advise or deny, and its own resolution of the rule files where a hook may run with no repository. It reuses this section's grammar and matcher unchanged, so it lands as a follow-on without reshaping anything here.
- **Why three directives.** `forbid` alone cannot tell "push the branch" from "never push", and a negation-aware parser is a language model's job, not a lint's. An `unless` pattern the consumer writes is the honest lexical approximation, and its line scope is a stated limit. `require` is the "quotes the request" check expressed as config, since the request's form, a blockquote or a heading, is the consumer's brief format.
- **Why one knob of files, with no tracked-and-local pair.** The knob file already has a gitignored `.local` overlay (gate-sdk/SPEC.md §The knob file), which can name a private rule file. A second knob would be a second spelling of that overlay.

### (2) The knob, the descriptor, the starter list and the pair {mechanical}

**Not yet applied** — delegation-kit/SPEC.md §Layout and configuration gains, after `DELEGATION_KIT_AGENT_DIR`'s D5 bullet:

> - `DELEGATION_KIT_BRIEF_RULES` — the rule files `check-brief-authority` reads (§check-brief-authority): a whitespace list of paths, repo-root-relative or absolute, expanded by nothing; default empty, which arms no rule.

The layout tree gains `checks/check-brief-authority.gate`, `gate-tests/check-brief-authority/{good,bad}/` and `templates/brief-rules.list  # starter rule list a consumer copies; read by nothing until named`. §Testing gains:

> - **`check-brief-authority`**, over fixture briefs passed as operands, with a case knob file naming a fixture rule file. The `good/` side holds a brief whose only push line is a prohibition the `unless` rule excuses, and which carries the required section. The `bad/` side holds a granted push and a granted install on separate lines, and a brief missing the required section. The rule grammar's exit-2 refusals (an unknown directive, an empty pattern, a malformed pattern, a named file that does not exist) and the no-operand clean skip are its module's unit tests.

The descriptor:

> ```
> # graph: couples=knob:DELEGATION_KIT_BRIEF_RULES dir=one valve=none tier=precommit
> # install: on-surface
> # armed-by: content DELEGATION_KIT_BRIEF_RULES
> # spec: delegation-kit/SPEC.md §check-brief-authority — a brief grants no forbidden authority and carries every required section
> ```

The README's gate roster block gains `check-brief-authority   # a brief names no forbidden grant and every required section`, and its consumer smoke registers it beside the other two.

## Producers and consumers

- **The brief operands.** Producer: a dispatcher, before a dispatch, through the direct form: a foreign run's prompt file, or an outside orchestrator's brief. The no-operand battery run reads no brief, by design. This repo wires no standing caller: its stage dispatches are inline payloads, the honest limit delta 1 states, and its foreign-run prompt files are written per run. The DoD's run item exercises the form once over one. Consumer: the new module `native/src/gates/brief_authority.rs`.
- **`DELEGATION_KIT_BRIEF_RULES`.** Producer: a consumer's knob file. This repo registers the gate and leaves the knob empty, so its battery run clean-skips, as `check-rule-id-coverage`'s does on a tree declaring no family. Consumers: the gate, and `doctor`'s armed-by read through the descriptor.
- **The starter list.** Producer: the kit. Consumer: a consumer copying it. No reader opens it in place, the same disposition as gate-sdk's `templates/msg-patterns.list`.
- **The findings and the clean line.** Consumer: the dispatcher deciding whether to send the brief. The battery's SARIF reader sees only the clean skip.
- **Point 2, roster-holding readers.** The gate's name lands on `native/src/gates/mod.rs`'s registry, `scripts/gates.list`, the kit README's gate-roster block, `delegation-kit/smoke/install.sh` and the generated rosters (`docs/enforcement.md`, `docs/check-graph.html`, `docs/llms-full.txt`). The knob lands on `native/src/knobs/delegation_kit.rs` and `--emit knob-roster`.
- **Point 5, narrowing.** None. The gate adds a corpus and narrows no existing one.
- **Point 6, every member.** Every operand brief is read against every rule. A brief obliged by a `require` rule it cannot satisfy is the finding the rule exists for.

## Existing sections updated

Roster probe: `git show --stat 0bb012643`, the last commit landing a new kit gate, whose file set names the sites every new gate lands on, and `git grep -n 'check-agent-tier-explicit' -- delegation-kit/ scripts/ native/src/gates/mod.rs` over the tracked tree.

- delegation-kit/SPEC.md — the new §check-brief-authority (delta 1), and §Layout and configuration's knob bullet and tree lines and §Testing's pair bullet (delta 2).
- delegation-kit/README.md — the gate-roster block line (delta 2).
- delegation-kit/checks/check-brief-authority.gate — new descriptor (delta 2).
- delegation-kit/gate-tests/check-brief-authority/good/ and bad/ — new pair, each with its args, its expect.txt, its fixture briefs and a fixture rule file named by a case knob file (delta 2).
- delegation-kit/templates/brief-rules.list — the starter list (delta 1).
- delegation-kit/smoke/install.sh — the gate joins the registration heredoc (delta 2).
- native/src/gates/brief_authority.rs — new module with the grammar's unit tests. native/src/gates/mod.rs gains its `pub mod` and registry row (delta 1).
- native/src/knobs/delegation_kit.rs — the whitespace-list row (delta 2).
- scripts/gates.list — registered under the dogfooding rule, clean-skipped here (delta 2).
- .workflow/release-declarations.md — *Knob changes* gains the knob and *Behavior changes* the gate (deltas 1 and 2).
- Generated projections the new member moves: docs/enforcement.md, docs/check-graph.html, docs/llms-full.txt, docs/delegation-kit/SPEC.md, docs/delegation-kit/README.md, docs/value.md's per-kit blocking-gate count, docs/requirements.md's install-size block and .workflow/surface-ceiling.txt. Each is regenerated by the command its freshness gate prints on red (docs/generated-projections.md) (all deltas).

## Retired spellings

- None — the gate, its knob and its starter list are additions.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for the gate, the knob, the three directives and the starter list.
- [ ] **Instruction surfaces: instruction only** — the starter list's comments name each rule's intent and carry no grounds.
- [ ] **Merged with no information lost** — the new section lands whole and the layout and testing additions re-phrase the passages they extend.
- [ ] **Amendment deleted** — this file removed on merge (`ls delegation-kit/SPEC-*.md`).
- [ ] **Removals propagated** — `## Retired spellings` holds.
- [ ] **Run once** — the direct form run over a written-out foreign-run prompt file with the starter list copied and named, its verdict read: a clean prompt is clean, and the same prompt with a push grant added reds naming the line.
- [ ] **Entry moved** — `brief-authority-lint` moves to Done before the drain stage, in the commit that lands the merge.
- [ ] **Gaps filed** — the dispatch-payload reader (delta 1's first ground) filed as a costed Deferred follow-on, and any other cross-component gap discovered during the work filed as a debt task.
