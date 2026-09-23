# SPEC amendment: shell-guard

guard-kit's `PreToolUse` guard is a bash script. A consumer copies `templates/bash-guard.sh`, which sources a 2381-line bash library, `lib/guard.sh`, and the library holds the 27 generic rules, every primitive a consumer rule is written in, and the harness-protocol plumbing. The rules read bash grammar only. On native Windows the guard runs because Git for Windows' bash is on the host, and the harness's `PowerShell` tool is not guarded at all. guard-kit/SPEC.md §The hook on native Windows refuses both a PowerShell twin and a native hook front, and §The guard framework declares the library permanently shell because it is the API consumer rules are written in.

**Operator direction, 2026-09-23 (lead-relayed), confirmed 2026-09-24 as authority to rewrite both refusals and the permanent-shell ground — a direction, not a TRAJECTORY ruling:** make the bash guard a **shell guard** over the host's native shell, bash or sh on Linux and macOS and PowerShell on Windows. It is one tool that checks shell commands and blocks, warns or rewrites them, and its consumer-rule seam is not shell functions. The operator folded four guard-kit rows the port reshapes into this design (2026-09-24): rule 23's worktree scratch match, the unheld `Declares` correspondence, and both rule-number entries. The dated provenance above is amendment-only. None of it merges into a kit SPEC (CLAUDE.md §The provenance seam).

**The design in one paragraph.** The guard becomes a harness-integration member of the gate binary, `--hook shell-guard`, wired through the front end like every other hook member. It reads the payload once and picks a **shell reader** by the payload's `tool_name`. It runs the consumer's rule command first and then the generic ruleset, over views the reader computes. Rules are compiled, keyed by **name**, and declare the views they read in the rule table the dispatcher walks. A consumer rule stops being a shell function in a copied script. It becomes a **command** the member spawns, speaking the harness's own hook protocol, and the binary's `--guard-json` flag is its toolkit.

**Sliced in two; this amendment is slice 1.** Slice 1 is the engine, the bash reader, the whole generic ruleset ported with unchanged verdicts, the consumer command seam, retirement of the shell library and template, rule names, declared views, and rule 23's worktree match. Slice 2 is the PowerShell reader, the rules that apply to PowerShell commands, and the `Bash|PowerShell` matcher. It stays Deferred as [guard-powershell-tool-unguarded](TASK-QUEUE.md#guard-powershell-tool-unguarded), re-scoped at this commit onto slice 1's reader seam. **Bash first, on three grounds.** The guard's two decision tables (277 rows) and its bespoke suites are the only oracle for guard behaviour, and a port has to match them. A PowerShell-first engine would be designed against no oracle at all. Bash first also keeps one engine and one consumer seam live at every commit, where PowerShell first leaves two guard engines with two seams until the port. And every folded row rides the port. **The honest cost:** after slice 1 the `PowerShell` tool is still unguarded, exactly as today.

**Ruled out.**

- *A PowerShell twin of the bash guard.* The refusal in §The hook on native Windows stands in substance: no second script implementation. The second reader lives in the binary, beside the first.
- *Keeping `lib/guard.sh` as the consumer composition API beside a compiled ruleset.* That keeps two engines and the bash dependency the direction removes.
- *Consumer rules as a second hook registered beside the guard.* Order is the seam's whole reason (§Consumer rules): a consumer grant or rewrite refining a generic block has to run first and win. The order between two separately registered hooks is no contract this kit controls.
- *Consumer rules as a declarative pattern table in the knob file*, refused on gate-sdk/SPEC.md §The extensibility model's DSL ground. A command covers every pattern such a table could hold.
- *Consumer rules as loaded native code*, refused on the same section's plugin ground.
- *A tombstone `lib/guard.sh`* whose one job is to make a stale consumer copy loud. It would keep a shell surface in the kit to announce its own removal. The release declaration carries the migration instead, and the limit is stated at delta 9.
- *Widening `--scratch-run`'s root* so it takes a main-checkout scratch path from a linked worktree. That root is a fail-closed reach control (guard-kit/SPEC.md §scratch-run), and delta 7 moves rule 23's match instead.

## What changes

### (1) A rule is identified by its name, and no rule has a number

The generic ruleset's roster stops being a numbered list, and every citation of a rule names it. **{mechanical}**

- **The name.** A rule's name is its current function name minus the `guard_rule_` prefix: `cd_compound`, `git_c_root`, `scratch_redirect`, `abs_script`, `abs_prefix`, `expansion`, `brace_glyph`, `sed_file`, `find_glob`, `cat_file`, `git_grep`, `pgrep_self_match`, `bare_sleep`, `git_mutation_under_producer`, `background_no_record`, `truncate_scratch`, `append_scratch`, `ro_pipeline`, `bounded_wait`, `allowlist_chain`, `git_rewrite`, `rm_tracked`, `script_interpreter`, `grant_path_slot`, `emitter_write`, `shell_wrapper`, `worktree_confinement`, in today's dispatch order (probe: `grep -n 'guard_rule_' guard-kit/lib/guard.sh`, the `guard_generic_rules` body). No rule is renamed beyond the prefix, so the name is the one the SPEC item already carries. The Rust rule function takes the same identifier (delta 4).
- **The roster.** §The generic ruleset's items move under a subsection of their own, `### The rule roster`, whose body is the roster alone, as a **bulleted** list in dispatch order. An item opens `- **<title>** (`<name>`) — `. Nothing in the roster carries an ordinal, so a reader has no number to copy. The widening bullets and the paragraphs before the list stay in the parent section, and the fall-through and sleep paragraphs after it move there too.
- **The citation grammar.** A citation is the word `rule` or `rules` followed by one or more backticked names, joined by `, ` and/or ` and `: `` rule `ro_pipeline` ``, `` rules `abs_script` and `brace_glyph` ``, and a possessive `` rule `grant_path_slot`'s test ``. Outside guard-kit's own tree and the crate's guard module, a citation is qualified: `` guard-kit rule `git_mutation_under_producer` ``. **Arms keep their letters** (`` rule `bounded_wait`'s arm (B) ``), since an arm is named inside one item.
- **The sweep.** Every `rule N` / `rules N, M and K` / `Rule N's` citation is rewritten to the name form, over the probe `git grep -nE '[Rr]ules? [0-9]+'` and the roster below. Out-of-kit hits cite guard-kit where the context says so, and the probe also hits doctrine-kit and canon-kit rule numbers, which stay as they are. Each hit is classified from its line (survey 2026-09-24 spec, `.workflow/survey-record.md`). About 525 intra-kit tokens and about 40 out-of-kit guard citations move. Record surfaces are left as written: `.workflow/` files, dated posts under `docs/posts/`, and `TASK-QUEUE.md`'s Done slugs and provenance lines. A record answers what happened.
- **The prose replacing the ruling.** §The generic ruleset's sentence "An item's number, its function and its dispatch position therefore say one thing three times" and the item-grammar paragraph become: **Replacement text** — **Not yet applied**:

  > **A rule is identified by its name.** The roster below lists the rules in dispatch order, and each item carries its name, which is also the identifier of the rule's function in the crate's rule table. Nothing cites a rule by position: an insertion moves no citation, and a rename is the one edit that moves them, which `check-guard-registration`'s arm D reads.

### (2) `check-guard-registration` holds the roster to the compiled table, and every citation to the roster

The gate's subject moves from the shell library to the crate's rule table, and it gains the two relations the folded rows left ungated. **{design-bearing}**

- **Inputs.** The SPEC's §The rule roster, read as delta 1 shapes it. The rule table, read in process from the binary running the gate, since that binary is the one built from this tree (§check-gate-binary-fresh holds it fresh). The citation corpus below. The positional form becomes `check-guard-registration [spec-file [table-file]]`. A table file is a synthetic table, one line per rule in dispatch order, `<name><TAB><view>[; <view>…]`, for the fixture pair.
- **Four assertions, each a finding at exit 1:**
  - **A, the roster grammar.** Every top-level bullet of §The rule roster carries exactly one `(`<name>`)` token right after its bold title, and the names are distinct. A name is `[a-z][a-z0-9_]*`.
  - **B, the order.** The roster's names in order equal the table's names in dispatch order. A finding prints the diff (`<` roster, `>` table).
  - **C, the declarations.** Every item carries exactly one declaration clause (delta 4's grammar), and its views equal the table's views for that rule, as sets. A finding names the rule and both sides.
  - **D, the citations.** Every citation in delta 1's grammar resolves to a roster name. The corpus is guard-kit's tree and the crate's guard module, `native/src/guard/`, for the bare and the qualified form alike, plus every other tracked file outside the workflow directory (`GATE_SDK_WORKFLOW_DIR`) for the qualified form. Within guard-kit's tree and the guard module, no `rules? <digits>` token may survive (`[Rr]ules? [0-9]+`). A finding names the file, the line and the citation.
- **Fail-closed, exit 2:** an unreadable SPEC, table file or corpus file; an absent §The rule roster, or one with no item; a declaration clause the grammar cannot parse; a table-file line of any other shape; and a failed `git ls-files`.
- **The clean line** becomes `GUARD-REGISTRATION: clean (<n> rule(s): roster, table order, declared views and <m> citation(s) in lockstep)`.
- **Fixtures.** The good/bad pair under `scripts/gate-tests/check-guard-registration/` is rebuilt over a synthetic table file: good holds one roster in lockstep, and bad fires A through D. The `.test.sh` keeps the exit-2 shapes: roster absent, a malformed declaration, a malformed table line.
- **What it still does not hold, stated:** a *bare* out-of-kit citation (`rule `x`` without the qualifier) is not read. The qualifier is the discriminator that stops the gate from claiming another kit's rule vocabulary. Since no rule carries a number, a writer who drops the qualifier still names the rule, and a later rename is the only way such a citation can go stale.

### (3) The shell guard is a hook member of the gate binary

`--hook shell-guard` replaces `templates/bash-guard.sh` and `lib/guard.sh` as the `PreToolUse` guard. **{design-bearing}**

- **Home.** A `shell-guard` row in `native/src/hook/mod.rs`'s `HOOKS` table and a thin `native/src/hook/shell_guard.rs`. The engine, the readers and the rules live under `native/src/guard/`, which absorbs today's `native/src/guard.rs`. The row's knob slice is every `GUARD_KIT_*` knob a rule or the log reads, `GUARD_KIT_CONSUMER_RULES_CMD` (delta 8), `GATE_SDK_NATIVE_BIN` for the steer door, and `GATE_SDK_WORKFLOW_DIR`, which the two log defaults derive from.
- **Wiring.** `templates/settings-hooks.json`'s `Bash` matcher group runs `bash gate-sdk/bin/run-gates.sh --hook shell-guard`, the front-end door every hook member takes. So an absent binary fails open at the front end, and a linked worktree reaches the main checkout's binary (gate-sdk/SPEC.md §The harness-integration arm). The matcher stays `Bash` in slice 1.
- **The call.** Read stdin whole once. An absent, empty or unparseable payload exits 0 with nothing written. On any payload that parses, the consumer stage runs first (delta 8), whatever the tool, and a consumer decision ends the call. Then `tool_name` selects the reader. `Bash` selects the bash reader (delta 5). Any other tool name, or a payload with no `tool_input.command` string, exits 0 with nothing written and nothing logged, which is the slot slice 2 fills. The rules then run in table order, and the first rule that decides ends the call. A call nothing decided appends its fall-through line to `GUARD_KIT_LOG`, in the encoding §scan-prompts decodes, and exits 0.
- **Verdicts,** each the envelope the library printed, through the crate's hook-JSON serializer:
  - A **block** is exit 2 with `shell-guard: <message>` on stderr.
  - An **advise**, an **allow** and a **rewrite** are the envelopes `guard_advise`, `guard_allow` and `guard_rewrite` printed, keys in the same order.
- **Knobs** resolve in process from the static table and the consumer's knob file, with no spawn. **A refused knob read keeps the guard's block**, with the refusal's own text. That is the posture a set-but-missing or malformed guard knob file has always had, and it is kept rather than traded for the class's `hook::decline`, which writes its reason to stderr and exits 0. gate-sdk/SPEC.md §The harness-integration arm grounds the decline on one case: a malformed *unrelated* knob wedging every guarded call. That ground does not reach here. The refused file is guard-kit's own, and its repair tool, Edit, is a call this member never sees, so the block wedges nothing it cannot also clear. That section names this member as the exception, and gives this ground (delta 10).
- **Retired with the library, because nothing is left to answer them:**
  - the unreachable-binary advisory envelope — the front end owns that case now;
  - the library-ahead-of-binary skew answer — one artifact cannot skew against itself;
  - the Git Bash trailing-CR limit — the member reads the payload bytes itself, and no command substitution sits between the field and a rule.
- **The steer door.** Every steer that names the binary spells it as gate-sdk's command-spelling rule does for `gate_native_bin_spelled`: `GATE_SDK_NATIVE_BIN`'s resolved value, `./`-prefixed when relative. The rules that name the door today keep naming it: `sed_file`'s section extractor and `--rewrite`, and `script_interpreter`'s and `shell_wrapper`'s `--scratch-run`. The crate spells it in one function beside the installer's own (`native/src/installer/init.rs`, which prefixes the placed artifact the same way). A seam case holds that function to `gate_native_bin_spelled`'s output for a relative and an absolute value (delta 6), which is criterion 6's machine-held clause for the second holder.
- **Latency is measured, not claimed.** Build records the member's per-call time on a `git status` payload over ten calls, beside §Testing's 65 to 68 ms for the shell guard, and the merged SPEC states the measured figure.

### (4) The rule table carries each rule's declared views, and a rule reads only what it declares

The `Declares` correspondence becomes a property of the table, which is what makes it checkable. **{design-bearing}**

- **Rows.** The table is a static slice in dispatch order. Each row carries the rule's name, the shells it applies to (`Bash` for every rule in slice 1), its declared **views**, and its function.
- **Views.** A view is `raw`, the command as received, or a **skeleton** named by its inert classes, a subset of `sq dq hd hdq` written in that order. There are also two derived views: `dequoted`, which is rule `sed_file`'s content-kept view aligned with the `sq dq hd` skeleton, and `body`, the heredoc body extraction. A view is spelled as it is written in a declaration: `raw`, `sq dq hd`, `sq hdq`, `dequoted`.
- **Reads are declared by construction.** The engine hands a rule its views through a context, and asking the context for a view the running rule does not declare is an internal fault. The member then emits a loud advise naming the rule and the view, and does not decide the call. So a decision-table row that exercises the read turns red, as an `advise` against its expected verdict. The normalizer is not visible to the rules module, so a rule has no path to a view except its context.
- **A predicate runs under its own declaration.** Where a rule takes another rule's test as a predicate, as `abs_script` and `brace_glyph` take `grant_path_slot`'s and `worktree_confinement`'s, the engine evaluates that test under the other rule's declared views. So a caller declares only its own reads, and the item's prose names its predicates as it does today.
- **The SPEC's declaration grammar.** Each roster item carries exactly one clause: the word `Declares`, then one or more backticked views separated by `, ` or ` and `. For example, `` Declares `sq hdq`, `sq dq hdq` `` for `expansion`. The item's prose may still say which view serves which check, but outside the clause.
- **Every item declares.** A rule that reads the raw command declares `raw`. So §The guard framework's hand-held roster of raw-reading rules ("rules 9, 10, 11, …") is deleted. What it said is now each item's clause, and arm C holds that. The same goes for the one-value-per-rule objection §check-guard-registration raised: a declaration is a set of views, one per view the rule reads.
- **Each member's satisfying value** — the views each of the 27 rules reads, derived from `lib/guard.sh`'s bodies with helper calls followed and other rules' predicates left out — is the table below. A disagreement with today's SPEC text is marked, and the table is what the port matches.

  Derived at spec (2026-09-24) by reading `guard-kit/lib/guard.sh` rule by rule, following every `guard_skeleton`, `_guard_dequoted_view` and `body=` site through its helpers. A shared helper that is no rule's test, such as `_guard_rewrite_granted` or `_guard_ro_forms_clear`, counts as the caller's own read. The harness view is a reader operation over a `dequoted` segment and is not a view of its own.

  | Rule | Declares | Takes as a predicate | Today's SPEC item |
  |---|---|---|---|
  | `cd_compound` | `sq dq hd` | — | no clause |
  | `git_c_root` | `raw`, `sq dq hd` | — | `sq dq hd`; the `raw` read spells arm (d)'s corrective |
  | `scratch_redirect` | `sq dq hd` | — | no clause |
  | `abs_script` | `sq dq hd`, `dequoted` | `grant_path_slot`, `worktree_confinement` | no clause |
  | `abs_prefix` | `sq dq hd` | — | no clause |
  | `expansion` | `sq hdq`, `sq dq hdq` | — | agrees |
  | `brace_glyph` | `sq dq hd`, `dequoted` | `grant_path_slot`, `worktree_confinement` | `sq dq hd`; omits `dequoted` |
  | `sed_file` | `sq dq hd`, `dequoted`, `body` | — | no clause |
  | `find_glob` | `raw`, `sq dq hd` | — | no clause; on the raw roster |
  | `cat_file` | `raw`, `sq dq hd` | — | no clause; on the raw roster |
  | `git_grep` | `raw`, `sq dq hd` | — | no clause; on the raw roster |
  | `pgrep_self_match` | `raw` | — | no clause; on the raw roster |
  | `bare_sleep` | `raw`, `sq dq hd` | — | no clause; on the raw roster |
  | `git_mutation_under_producer` | `raw`, `sq dq hd` | — | no clause; on the raw roster |
  | `background_no_record` | `raw`, `sq dq hd`, `dequoted` | — | no clause; on the raw roster |
  | `truncate_scratch` | `sq dq hd` | `worktree_confinement` | no clause |
  | `append_scratch` | `hdq`, `sq dq hd` | `worktree_confinement` | agrees |
  | `ro_pipeline` | `raw`, `sq dq hd`, `dequoted` | `worktree_confinement` | `sq dq hd`; omits `raw` and `dequoted` |
  | `bounded_wait` | `raw`, `sq`, `sq dq hd`, `dequoted` | `worktree_confinement`; in arm (B), `grant_path_slot`, `rm_tracked`, `script_interpreter` | no clause; on the raw roster |
  | `allowlist_chain` | `sq dq hd` | — | agrees |
  | `git_rewrite` | `sq dq hd` | `worktree_confinement` | no clause |
  | `rm_tracked` | `raw`, `sq dq hd`, `dequoted` | — | no clause; on the raw roster |
  | `script_interpreter` | `raw`, `sq dq hd` | — | `sq dq hd`; omits `raw` |
  | `grant_path_slot` | `raw`, `sq hdq`, `sq dq hd`, `dequoted` | — | no clause. The raw roster lists it, but only its backtick test reads `raw`: its substitution test reads `sq hdq` |
  | `emitter_write` | `raw`, `hdq`, `sq dq hd` | — | no clause |
  | `shell_wrapper` | `raw`, `sq dq hd` | — | `sq dq hd`; omits `raw` |
  | `worktree_confinement` | `raw`, `sq hdq`, `sq dq hd`, `dequoted` | — | `sq dq hd`; omits `raw`, `sq hdq` and `dequoted`, and is missing from the raw roster although its tests split exactly as `grant_path_slot`'s do |

  Three `raw` reads are cheap pre-filters rather than tests: `emitter_write`'s `*>*`, `shell_wrapper`'s `*sh*-c*` and `grant_path_slot`'s `*/*`. A port that drops one of them drops `raw` from that row's declaration in the same commit. Arm C holds the result, and build records the drop in its journal. Every other value above is what the port declares.

- **Honest limit.** The fault check catches an undeclared read only where a table row exercises it. A declared view no path reads (over-declaration) is not caught.

### (5) The bash reader is the command model the rules read

The bash reader brings together today's compiled twins and the library's remaining shell parsing, as one module the engine calls. **{design-bearing}**

- **What it provides.** Every view of delta 4. The compound split, whose separator class stays as measured. The statement split, which runs `;`, `&&`, `||` and newline and carries heredoc residue, as rules `append_scratch` and `emitter_write` need. The two-level split `script_interpreter` runs, statements and then pipes. Redirect pairs, and the harness view.
- **The twins become the one holder.** `skeleton`, `split_compound`, `redirect_pairs`, `harness_view` and `allow_match` already exist in `native/src/guard.rs` and keep their specified behaviour. The `scan-prompts` ranker, `--emit-compare-settings-allow` and the rules now call the same functions. With the shell holder gone, the parity machine has nothing left to compare (delta 9).
- **The reader seam.** The engine sees the reader through one interface: views, segments, statements, redirect pairs and harness view. So slice 2's PowerShell reader is a second implementation of that interface, and it does not change the engine. The seam's methods are the ones slice 1's rules call, and none is added ahead of a caller.

### (6) Every generic rule is re-implemented with its verdicts unchanged, and the decision tables are the oracle

Each roster item is ported to a Rust function in `native/src/guard/`, its SPEC item the contract. **{design-bearing}**

- **Everything the tables pin stays the same.** That is each rule's firing and declining conditions, its corrective text, its order, and its reading of `GUARD_KIT_*` knobs, the settings allowlist, git, and the process table for rule `git_mutation_under_producer`'s liveness read. The payload fields read are the ones §The guard framework's payload paragraph records: `tool_input.command` and `tool_input.run_in_background`. The working directory is the member's own, as it was the hook's.
- **The oracle.** The guard's two decision tables, `guard-tests/cases.tsv` (271 rows) and `background-cases.tsv` (6 rows), and every bespoke suite below, pass against the member. The row count is `grep -cvE '^\s*(#|$)'`. `escalation-cases.tsv` is escalation-guard's, and it does not move. No expected column is edited except for the re-derivations named here. A row whose expected verdict would otherwise change is a finding. Build stops and resolves it in that session, and nothing is re-derived to fit. The re-derivations:
  - `guard-config-knobs.test.sh`: the missing- and malformed-knob-file cases keep their block (delta 3). The unreachable-binary and skew cases are deleted, and so is the wrapper binary that drove them. Rule cases that sourced the library drive the member with the sandbox knob file.
  - `guard-read-path.test.sh` is replaced by the `--guard-json` seam cases of delta 8. `guard_read_path` retires. Its one sanctioned consumer shape, a path-bearing guard, is now a compiled member's own concern: lifecycle-kit's `workflow-state-guard` reads its path in-crate.
  - `git-mutation-under-producer.test.sh` and `worktree-confinement.test.sh` drive `--hook shell-guard` in place of the template. So does gate-sdk's `run-gates-linked-worktree.test.sh`, where it drives the guard.
  - A new seam case holds the steer door to `gate_native_bin_spelled` (delta 3).
  - Rule `script_interpreter` gains the rows of delta 7.
- **Sequencing is build's.** The only constraint is that wiring does not flip (delta 9) before every row of both tables and every bespoke suite passes against the member. How the port reaches that is for build to decide, rule group by rule group or otherwise. Any transitional switch that points the runner at the member is removed in the flipping commit.

### (7) Rule `script_interpreter` reads the main checkout's scratch dirs from a linked worktree

Its match resolves `GUARD_KIT_SCRATCH_DIRS` the way rules `git_mutation_under_producer`, `background_no_record`, `bounded_wait`'s arm (B) and `worktree_confinement` already do, and its steer from a worktree names a runner call that works there. **{design-bearing}**

- **Measured at spec, 2026-09-24,** from a temporary linked worktree:
  - `bash gate-sdk/bin/run-gates.sh --scratch-run <main>/.tmp/x.sh` exits 2: *refusing … outside the scratch dir `<worktree>/.tmp`*, or *no scratch dir* when the worktree has none.
  - `--scratch-run .tmp/own.sh` runs.
  - The guard lets `bash <main>/.tmp/x.sh` through unsteered (exit 0), and blocks `bash .tmp/own.sh` by this rule.

  This settles the filed entry's inferred premise, and it is why the runner's root stays where it is.
- **The match.** From a linked worktree a body source fires when it sits under a scratch member of the main checkout **or** of the session's own worktree, each compared lexically with `..` folded. From the main checkout nothing changes.
- **The steer.** For a body under the main checkout's scratch dir, the steer from a worktree says to write the script under the worktree's own scratch dir and run it through the runner there. The runner refuses the main checkout's path from a worktree, and a steer to a call that refuses cannot be followed. A body under the worktree's own scratch dir keeps today's steer.
- **Rows.** `worktree-confinement.test.sh` gains the case `bash <main>/.tmp/x.sh` from the worktree, which blocks, and a check that its corrective names the worktree's own scratch dir. `bash .tmp/own.sh` from the worktree stays blocked with today's steer. The table's sandbox is no worktree, so no `cases.tsv` row moves.
- **The item's scratch-dir sentence** in §Layout and configuration's `GUARD_KIT_SCRATCH_DIRS` entry ("Rules 3 and 23 read the member as written too") becomes: rule `scratch_redirect` reads the member as written, and rule `script_interpreter` reads it both ways from a worktree. **Not yet applied.**

### (8) A consumer's rules are a command the guard runs first

§Consumer rules' placement contract is kept whole: a consumer rule runs ahead of the generic ruleset and can refine it. What changes is the medium, from shell functions in a copied script to a command. **{design-bearing}**

- **The knob.** `GUARD_KIT_CONSUMER_RULES_CMD` is a command knob (gate-sdk/SPEC.md §The knob file: an indexed argv, spawned directly, no shell), and its default is empty, meaning no consumer stage. It is a table row in `native/src/knobs/guard_kit.rs` and a commented-out line in `templates/guard-config.knobs`. **The seam:** the command and what it blocks are the consumer's own. The protocol is kit mechanism. The kit ships no rule and no consumer-rule template.
- **The protocol is the harness's own hook protocol.** A consumer rule command is written like any `PreToolUse` hook command:
  - The member spawns the argv once per call, before the generic ruleset. The payload bytes go to stdin as received, and the working directory and environment are inherited.
  - **Exit 2** blocks. The child's stderr is relayed verbatim and the member exits 2, and the generic ruleset does not run.
  - **Exit 0 with stdout** is the consumer's decision. The member relays stdout verbatim and exits 0 when it parses as a JSON object. The generic ruleset does not run, and no fall-through is logged.
  - **Exit 0 with empty stdout** is no decision, and the generic ruleset runs.
  - **A fault** is any other exit, a failed spawn, or stdout that does not parse. It is loud and never silent, and it never stops the generic ruleset. The member runs the ruleset and carries the fault on its own output: appended to a block's stderr, or as `additionalContext` on an allow or rewrite envelope, or as an advise naming the command and the fault when the ruleset decides nothing. A fault costs the consumer's rules for that call and nothing else.
- **What a consumer command gets from the kit is `--guard-json`**, kept and repurposed from the library's internal channel into the command's toolkit. Its read modes `field` and `field-or-empty` and its render modes `advise`, `allow` and `rewrite` stay as specified. `allow-entries` stays. It gains one mode:
  - **`view <view>`** reads a payload on stdin and prints the named view (delta 4's spelling, e.g. `view sq dq hd`) of its `tool_input.command`, through the reader its `tool_name` selects. It prints nothing for a tool no reader serves or a payload that does not parse. That is the quote- and heredoc-inert match surface §Writing a consumer rule tells a rule to match on, without a JSON parser or the library.
- **This repo's three rules move to `scripts/guard-rules.sh`**, a consumer command named by `scripts/guard-config.knobs`: the `--no-verify` block, the harness-scratchpad block and the `git clean -x` block. Each matches the `sq dq hd` view read through `--guard-json view`, as the copy matched `guard_skeleton … sq dq hd`, and blocks with its message unchanged. `scripts/bash-guard.sh` is deleted. The file is a consumer's own rule content, so it carries `# no-port:` on the provenance-seam ground its predecessor declared, and it is shellcheck-linted as a member of the consumer gates dir.
- **Order within the consumer stage is the consumer's.** The ordering guidance in §Writing a consumer rule — blocks before side-effecting rules, a steer before the broader rule — is kept as guidance for the command's author.
- **Honest limits.**
  - A consumer command that hangs holds the call until the harness's hook timeout. The member sets no timeout of its own.
  - A consumer grant is bounded by its author and not by the generic ruleset. That is the posture today, and rule `worktree_confinement`'s limits already state it.
  - A consumer's rules still have no verification lane, the gap §Consumer rules names. It is unchanged, and the command form does not close it.
- **Cases.** A bespoke `gate-tests/consumer-rules.test.sh` drives the member under a sandbox knob file naming a scratch command:
  - a block relayed with its stderr;
  - an allow envelope relayed while the generic ruleset would have blocked the same command;
  - an empty result falling through to a generic block;
  - a non-zero-non-2 exit running the ruleset with the fault on its output;
  - unparseable stdout, handled the same way;
  - the empty knob running no stage.

  `--guard-json`'s seam cases sit beside them, where `guard-read-path.test.sh`'s field cases move: verbatim bytes, `view` over a heredoc-bearing and a quoted command, and `view` printing nothing for a non-`Bash` payload.

### (9) Wiring flips and the shell surfaces retire, in one commit

Once delta 6's oracle is green against the member, one commit points the wiring at it and removes the shell guard. **{mechanical}**

- **Wiring.** `templates/settings-hooks.json`: the `Bash` group's command becomes `bash gate-sdk/bin/run-gates.sh --hook shell-guard`, and its `//` note is rewritten to match (no copied script, consumer rules through the knob, the `PowerShell` tool not yet guarded). **This repo's `.claude/settings.json` hook entry is a settings edit:** build prepares the one-line diff and hands it up, and it is applied on the operator's behalf per CLAUDE.md §Housekeeping. It is high-impact because it replaces a hook. The flipping commit carries the diff and does not apply it.
- **Deleted:** `guard-kit/lib/guard.sh`, `guard-kit/templates/bash-guard.sh`, `scripts/bash-guard.sh` (for `scripts/guard-rules.sh`, delta 8), `guard-kit/gate-tests/guard-lib-parity.test.sh`, and `guard-read-path.test.sh` (delta 6). Also deleted: the `--guard-lib-parity` flag and its `TOP_LEVEL_FLAGS` entry in `native/src/main.rs`, and the `GUARD_KIT_LIB` spelling everywhere. **Why `--guard-lib-parity` goes:** its second holder is gone, which is the `--declaration-parity` precedent gate-sdk/SPEC.md §The non-gate arm records.
- **`--run-guard-tests` spawns the member.** Each row runs `<running binary> --hook shell-guard`, directly and with no front end, with the payload on stdin and the working directory in the sandbox. The row's `GUARD_KIT_LOG` is overridden into the sandbox, as today. `GUARD_KIT_LIB` and the `bash -c` wrapper at `run_guard_tests.rs` go. The sandbox, table grammar and classification ladder are unchanged. The consumer's knob file is not in the sandbox, so no consumer stage runs, which is what the kit's table needs. **The arm stays an arm**, on §Testing's reach ground: anyone holding the binary can run the table, with no toolchain needed.
- **Smoke.** `guard-kit/smoke/install.sh` stops copying a guard. It merges the wiring, unions the allowlist, and drives the crafted payload through `bash gate-sdk/bin/run-gates.sh --hook shell-guard`, asserting the `cd_compound` block, and it keeps the allowlist-literal assertion over the member. `installer/consumer-smoke/run-smoke.sh`'s guard step drives the member the same way.
- **Everything else that names the shell guard** moves to the member or is deleted, per the roster below: the `overhead_meter.rs` hook-name literal becomes `shell-guard`, and so does the `docs/enforcement.md` row, regenerated. The `check-guard-registration.gate` `couples=` line drops `guard-kit/lib/guard.sh` and gains the guard module. The generated pre-commit hook and `docs/check-graph.html` are regenerated.
- **Migration, declared.** `.workflow/release-declarations.md` gets an entry for the next release naming the break and the steps: rewire the `Bash` hook to `--hook shell-guard`, move project rules into a command named by `GUARD_KIT_CONSUMER_RULES_CMD`, and delete the copied `bash-guard.sh`. **Honest limit:** a consumer that upgrades without rewiring keeps a copy whose first line tests for the deleted library and exits 0. Its guard fails open silently until it rewires, and the declaration is the only notice.

### (10) guard-kit's SPEC is rewritten around the member, and gate-sdk's guard-specific rulings follow

The prose merge, ruled here so build carries no design judgment into it. **{design-bearing}**

- **§The guard framework (`lib/guard.sh`)** becomes **§The shell guard**. It is the member's contract (delta 3), the reader seam (delta 5), the view model and its classes (delta 4), and the fail-open postures, which keep their names. The primitive roster is rewritten as the member's behaviour and the reader's views, with no shell function names. The normalizer's classes, the heredoc extent, the placeholder-not-deletion point, the harness view's wrapper walk, the allow-match model and its measurements, the splitter's measured separator class, and the payload and permission-mode records all move over unchanged in substance. The paragraphs on the five twins, on the permanent-shell ground and its *What reopens it*, on `GUARD_INPUT` and on the jq filter limit are deleted. The ground dissolved on the exact condition that paragraph named: §Consumer rules stopped composing consumer rules from shell primitives.
- **§Consumer rules** and **§Writing a consumer rule** state delta 8's seam. Both sides of the seam stop being permanent shell: the kit side is compiled, and the consumer side is whatever the consumer's command is written in.
- **§The hook on native Windows** is rewritten. The guard is a hook member reached through the front end, as every hook is. So the host needs the front end's shell and the binary, and nothing guard-specific. The Git Bash CR limit is deleted, since the member reads the payload itself. The two refusals are replaced by the design: the rules are compiled, and a second shell is a second reader in the binary (slice 2). The `PowerShell` honest limit stays, with its pointer changed from *separate work, not this kit's today* to the reader seam slice 2 fills. The oracle paragraph names the decision table run by the arm on the native-Windows leg, where it now needs no Git Bash for the subject.
- **§The generic ruleset**: the roster subsection (delta 1), a declaration on every item (delta 4), and rule `script_interpreter`'s match and steer (delta 7). Its door paragraph ("`lib/guard.sh` resolves `_guard_door` once at load…") becomes delta 3's steer-door sentence. The cross-kit-coupling ground it argued is gone with the library.
- **§Layout and configuration**: `lib/` and `templates/bash-guard.sh` leave the tree listing. `GUARD_KIT_LIB`'s entry is deleted. `GUARD_KIT_CONSUMER_RULES_CMD`'s entry is added. The listing names the member's module home and the new tests.
- **§Testing**: the runner spawns the member (delta 9). The *what is under test does not move* paragraph is rewritten, because the subject is now in-crate while the arm stays an arm on the reach ground. The paragraph that refused a crate-test lane keeps its discriminator: reach decides, now for a subject that is in-crate. The bespoke-lane paragraphs follow delta 6's retargets. The knob-load latency paragraph gives way to delta 3's measured figure.
- **§check-guard-registration**: delta 2's assertions, inputs and clean line. Its *out of its reach, by construction* paragraph is deleted, since arm C now holds that.
- **README**: the manual `cp … bash-guard.sh` step goes, the wiring snippet becomes the member, and consumer rules are pointed at the knob.
- **gate-sdk/SPEC.md**:
  - **§The adopter constraints**: the sentence "A script that guards a shell the harness already runs is written in that shell on every host…" is deleted. No kit script guards a shell any longer, and the interpreter bullet's two caller classes stand as they are.
  - **§The harness-integration arm**: the second-envelope-producer paragraph is deleted, since one producer is left. The knob-decline sentence names `shell-guard` as its one exception, on delta 3's ground. The fail-open ground paragraph's "`guard-kit/lib/guard.sh` already fails open…" is re-grounded on the member's own degraded path. `GUARD_KIT_LIB`'s clause in the member-table paragraph drops to the verdict-binary precedent it cites.
  - **§The non-gate arm**: `--guard-lib-parity`'s named-caller clause is deleted, and `--guard-json`'s caller becomes consumer rule commands (delta 8).
  - **§The port-candidate criteria**'s fifth-instance paragraph records the three primitives' twins ending as single holders.
  - **§lib/gate.sh**'s "the one static kit library that stays" sentence is deleted.
  - **§The harness-template port disposition** drops `templates/bash-guard.sh` and its copy from the declaring partition and from both grounds' worked members. The partition's remaining members are unchanged.
  - **§Output contract**'s list of refusals citing a section drops `lib/guard.sh`'s.
- **Elsewhere**, each a clause re-pointed at the member or deleted:
  - context-kit/SPEC.md §The session-context hook (template): "the `bash-guard.sh` pattern" is re-pointed, and the bash floor paragraph drops guard-kit's library from the 4.3 sourcers. guard-kit stays in env-probe's bash audience, because its hook wiring spawns the front end with `bash`, and the predicate derives that unaided.
  - installer/SPEC.md §Requirements and §The consumer smoke.
  - drift-kit/SPEC.md §Bundled KPIs.
  - canon-kit/SPEC.md §check-spec-pointer and §check-prose-enum's `guard_allow` worked example, which is re-worded over names the tree still carries.
  - doctrine-kit/SPEC.md §Out of scope and delegation-kit/SPEC.md's `guard_advise` mention.
- **The merged text carries no dated provenance**, only the undated rule and its engineering grounds (CLAUDE.md §The provenance seam).

## Producers and consumers

- **The `shell-guard` member (delta 3).**
  - Producer: the harness, on every `Bash` tool call, through the `PreToolUse` wiring. The enabling config is `templates/settings-hooks.json` for an adopter, and this repo's `.claude/settings.json` once the prepared edit is applied.
  - Consumer: the harness reads its exit status, stderr and envelope, which is the protocol unchanged.
  - Roster-holding readers of the minted name: the `HOOKS` table itself, and the unknown-member refusal, which derives from it. `check-front-end-fail-open` holds `--hook` as a family, so no line changes. `overhead_meter.rs`'s hook-name list gains `shell-guard` (delta 9). `docs/enforcement.md`'s guard row is regenerated from the class registry.
- **The rule table (deltas 2 and 4).**
  - Producer: the crate, at build.
  - Consumers: the engine's dispatch; `check-guard-registration` arms B and C, in process; the context's view-fault check.
  - Every field has a reader. `name` is read by dispatch messages and arms A to D. `shells` is read by the engine's per-reader filter. `views` is read by the context and arm C. The function is read by dispatch.
- **A rule name (delta 1).** Producer: the roster and the table, held equal by arm B. Consumers: citations, read by arm D.
  - Red conditions (point 5): arm D is monotone in unresolved citations. The retired-number sub-arm reds on any survivor, and the sweep empties the corpus of survivors before the gate lands.
  - Arm B asserts an exact equality, so the rename sweep and the table must land in one commit with the gate change, or B reds.
- **A declared view (delta 4).**
  - Producer: the row.
  - Consumers: the context, which faults on an undeclared read, and arm C.
  - Point 6: every member's value is named in delta 4's table, which was derived from the library bodies.
- **`GUARD_KIT_CONSUMER_RULES_CMD` (delta 8).**
  - Producer: a consumer's knob file. This repo sets it in `scripts/guard-config.knobs` to `bash` + `scripts/guard-rules.sh`, so a deployed configuration reaches the stage.
  - Consumer: the member's consumer stage, by spawn.
  - Roster-holding readers of the minted knob name, found with `grep -ln 'KIT\.rows\|knobs::' native/src/gates/*.rs` and the gates' headers:
    - the crate's knob table, and `--emit knob-roster` derived from it;
    - the `shell-guard` row's knob slice;
    - canon-kit's `kit_ref_liveness` check (`check-kit-ref-liveness`), which reds on a `GUARD_KIT_*` mention that no table knob resolves. So the row lands in or before the first commit that names the knob;
    - `check-knob-default-coupling`, over the §Layout entry's stated default.

    The probe is a grep, so build runs the battery on the landing commit to catch any reader it missed.
- **`--guard-json view` (delta 8).**
  - Producer: a consumer command's call.
  - Consumer: that command, by stdout.
  - `main.rs`'s usage string and the flag's mode table are its roster, and both gain the mode.
- **The fault channel (delta 8).** Producer: the consumer stage. Consumer: the session, through the member's block text, `additionalContext` or advise. No field is added to any log.
- **Retirements (delta 9), with each reader's red condition (point 5):**
  - `check-shellcheck` reads kit `lib/` and `templates/`, and a smaller corpus removes findings only.
  - `check-template-copy-parity` finds its pairs as `*/templates/*.sh` with a same-named gates-dir copy, and asserts no minimum count (probed: `native/src/gates/template_copy_parity.rs`, `rule()`). So deleting both files together drops one pair and reds nothing. `scripts/guard-rules.sh` has no template, so it is no pair.
  - `port-blockers --tree` counts two fewer `no-port` members and gains `scripts/guard-rules.sh`. A consumer-side completion predicate over its owed count moves by nothing, since no owed file is added.
  - `check-door-binding` sweeps kit `templates/`, `lib/` and `bin/`. The deleted template named no door, and the rewired `settings-hooks.json` names the front end inside the fail-open set (`--hook`), which the gate admits.
  - `check-gate-substrate-parity` read the two `# no-port:` headers and nothing counts them.
  - env-probe's bash audience: guard-kit stays in it, because `native/src/toolfloor.rs`'s `spawns_bash_in_settings` qualifies a settings template whose hook command's first word is `bash`, and the rewired command keeps that word. So `docs/install.md`'s derived floor line does not move.
- **The seam, ruled.**
  - **Kit mechanism:** the member, the engine, the readers, the 27 generic rules, the view model, the consumer-command protocol, `--guard-json`, the gate.
  - **Private rule content:** this repo's three consumer rules in `scripts/guard-rules.sh`, whose messages name this repo's own instructions.
  - **Consumer config:** `GUARD_KIT_CONSUMER_RULES_CMD`, next to the existing `GUARD_KIT_*` rosters.

## Existing sections updated

Roster probes, run at authoring over the retired-spelling gate's reconciliation corpus (`git ls-files` minus `TASK-QUEUE.md`, `TRAJECTORY.md`, `.workflow/*`, `*/gate-tests/*`): `git grep -l -E '<spelling>'` for each spelling in the block below, and `git grep -nE '[Rr]ules? [0-9]+'` for the citation sweep. Section attribution came from a heading-tracking pass over each file. Test files under `gate-tests/` are listed from `git grep -l` over the unfiltered tree.

- `guard-kit/SPEC.md` — §The friction loop; §The recommended allowlist; §The guard framework (`lib/guard.sh`) → §The shell guard; §Consumer rules; §The hook on native Windows; §The generic ruleset and its new §The rule roster; §Writing a consumer rule; §scratch-run's port-owed paragraph; §scan-prompts' primitive citations; §Layout and configuration; §Testing; §check-guard-registration (all deltas).
- `guard-kit/README.md` — the copy step, the wiring and the consumer-rule pointer (deltas 8 and 9).
- `guard-kit/lib/guard.sh` — deleted (delta 9).
- `guard-kit/templates/bash-guard.sh` — deleted (delta 9).
- `guard-kit/templates/settings-hooks.json` — the `Bash` group's command and the `//` note (delta 9).
- `guard-kit/templates/guard-config.knobs` — the consumer-command line, and its rule citations to names (deltas 1 and 8).
- `guard-kit/guard-tests/cases.tsv` — comment citations to names, and its `lib/guard.sh` mention (delta 1).
- `guard-kit/guard-tests/background-cases.tsv` — comment citations to names (delta 1).
- `guard-kit/gate-tests/guard-lib-parity.test.sh` — deleted (delta 9).
- `guard-kit/gate-tests/guard-read-path.test.sh` — deleted, its cases moved (deltas 6 and 8).
- `guard-kit/gate-tests/guard-config-knobs.test.sh` — drives the member; citations to names (deltas 1 and 6).
- `guard-kit/gate-tests/git-mutation-under-producer.test.sh` — drives the member; citations to names (deltas 1 and 6).
- `guard-kit/gate-tests/worktree-confinement.test.sh` — drives the member, with rule `script_interpreter`'s new cases; citations to names (deltas 1, 6 and 7).
- `guard-kit/gate-tests/scan-prompts.test.sh` — its log-writing step drives the member (delta 6).
- `guard-kit/gate-tests/consumer-rules.test.sh` — new (delta 8).
- `guard-kit/smoke/install.sh` — no copy step; it drives the member (delta 9).
- `native/src/guard.rs` — absorbed into `native/src/guard/` as the bash reader; `json_arm` gains `view` (deltas 3, 5 and 8).
- `native/src/hook/mod.rs` — the `shell-guard` row, and its `guard_block`/`guard_advise` comment citations re-pointed (delta 3).
- `native/src/main.rs` — `--guard-lib-parity` removed, `--guard-json` usage gains `view`, and its `guard_log_fallthrough`/`guard_allow_match`/`_guard_harness_view` comment citations re-pointed (deltas 8 and 9).
- `native/src/emit/run_guard_tests.rs` — spawns the member (delta 9).
- `native/src/emit/scan_prompts.rs` — calls the one holder, and its rule-number and `guard_skeleton`/`guard_log_fallthrough` citations move to names (deltas 1 and 5).
- `native/src/emit/wait_probe.rs` — rule citations to names (delta 1).
- `native/src/knobs/guard_kit.rs` — rule citations to names, and the `GUARD_KIT_CONSUMER_RULES_CMD` row (deltas 1 and 8).
- `native/src/emit/overhead_meter.rs` — the hook-name literal (delta 9).
- `native/src/emit/kpi/prompt_friction.rs` — its permanently-shell comment (delta 9).
- `native/src/gates/guard_registration.rs` — rewritten to delta 2.
- `scripts/check-guard-registration.gate` — `couples=` (deltas 2 and 9).
- `scripts/git-hooks/pre-commit` — regenerated after the `couples=` edit with the command `check-graph` prints on red; it and `docs/check-graph.html` form one projection set (docs/site-architecture.md §Generated projections and their freshness gates) (delta 9).
- `scripts/gate-tests/check-guard-registration/` — rebuilt fixture pair over a synthetic table file (delta 2).
- `scripts/gate-tests/check-guard-registration.test.sh` — the exit-2 shapes (delta 2).
- `scripts/bash-guard.sh` — deleted for `scripts/guard-rules.sh` (deltas 8 and 9).
- `scripts/guard-rules.sh` — new, this repo's consumer rule command (delta 8).
- `scripts/guard-config.knobs` — `GUARD_KIT_CONSUMER_RULES_CMD`, and its rule citation to a name (deltas 1 and 8).
- `.claude/settings.json` — the `Bash` hook command, a prepared diff applied on the operator's behalf (delta 9).
- `installer/consumer-smoke/run-smoke.sh` — the guard step (delta 9).
- `gate-sdk/gate-tests/run-gates-linked-worktree.test.sh` — drives the member (delta 6).
- `gate-sdk/SPEC.md` — §Output contract, §The non-gate arm, §The harness-integration arm, §The port-candidate criteria, §lib/gate.sh, §The adopter constraints, §The harness-template port disposition (delta 10).
- `context-kit/SPEC.md` — §The session-context hook (template) and the bash-floor paragraph (delta 10).
- `context-kit/templates/session-context.sh` — its header's "bash-guard pattern" (delta 10).
- `installer/SPEC.md` — §Requirements, §The consumer smoke (delta 10).
- `drift-kit/SPEC.md` — §Bundled KPIs (delta 10).
- `canon-kit/SPEC.md` — §check-spec-pointer, §check-prose-enum (delta 10).
- `doctrine-kit/SPEC.md` — §Out of scope (delta 10).
- `delegation-kit/SPEC.md` — every guard rule citation in §The delegation model, §Operative residency, §The turn-end liveness hook, §Attribution was weighed and is not available, §What `background_tasks` carries, §The probe is asymmetric, and no reading may treat it otherwise, and §bin/wait-probe, and the `guard_advise` mention (deltas 1 and 10).
- `delegation-kit/templates/agent-execution.md` — its guard rule citation (delta 1).
- `lifecycle-kit/SPEC.md` — §check-stage-evidence's guard rule citation (delta 1).
- `evidence-kit/SPEC.md` — §lifecycle-kit integration's guard rule citation (delta 1).
- `TASK-QUEUE.md` — live entries' guard rule citations, the two icebox one-liners citing a guard rule by number included (delta 1).
- `.workflow/release-declarations.md` — the next release's migration entry (delta 9).
- `docs/guard-kit/SPEC.md` — generated mirror, regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` (all deltas).
- `docs/guard-kit/README.md` — generated mirror, regenerated likewise (all deltas).
- `docs/gate-sdk/SPEC.md` — generated mirror, regenerated likewise (delta 10).
- `docs/context-kit/SPEC.md` — generated mirror, regenerated likewise (delta 10).
- `docs/installer/SPEC.md` — generated mirror, regenerated likewise (delta 10).
- `docs/drift-kit/SPEC.md` — generated mirror, regenerated likewise (delta 10).
- `docs/canon-kit/SPEC.md` — generated mirror, regenerated likewise (delta 10).
- `docs/doctrine-kit/SPEC.md` — generated mirror, regenerated likewise (delta 10).
- `docs/delegation-kit/SPEC.md` — generated mirror, regenerated likewise (deltas 1 and 10).
- `docs/lifecycle-kit/SPEC.md` — generated mirror, regenerated likewise (delta 1).
- `docs/evidence-kit/SPEC.md` — generated mirror, regenerated likewise (delta 1).
- `docs/enforcement.md` — regenerated with `bash gate-sdk/bin/run-gates.sh --emit enforcement-map > docs/enforcement.md` (delta 9).
- `docs/check-graph.html` — regenerated with `bash gate-sdk/bin/run-gates.sh --emit graph > docs/check-graph.html` (delta 9).
<!-- update-target-exempt: a dated release post is a record of what shipped, and keeps the spellings it published with -->
- `docs/posts/2026-08-14-checkwright-v0-23-0.md` — left as published.
<!-- update-target-exempt: a dated release post is a record of what shipped, and keeps the spellings it published with -->
- `docs/posts/2026-08-23-checkwright-v0-25-0.md` — left as published.

## Retired spellings

- `guard_rule_` — the rule-function prefix; a rule is its name (delta 1).
- `guard_generic_rules` — the shell dispatcher; the table is the dispatch (delta 3).
- `GUARD_KIT_LIB` — named the library the copied guard sourced (delta 9).
- `guard-lib-parity` — the parity flag and its test; one holder is left (delta 9).
- `lib/guard.sh` — the shell library (delta 9).
- `bash-guard` — the hook's name, its template and this repo's copy; the member is `shell-guard` (delta 9).
- `guard_read_input` — a library primitive; the member reads the payload itself (deltas 3 and 9).
- `guard_input_field` — a library primitive; a consumer command reads a field with `--guard-json field` (deltas 8 and 9).
- `guard_read_command` — a library primitive (deltas 3 and 9).
- `guard_read_path` — a library primitive with no remaining shell consumer (deltas 6 and 9).
- `guard_block` — a library primitive; a block is the member's exit 2 (deltas 3 and 9).
- `guard_advise` — a library primitive; a consumer renders with `--guard-json advise` (deltas 8 and 9).
- `guard_rewrite` — a library primitive (deltas 8 and 9).
- `guard_log_fallthrough` — a library primitive; the member appends the line (deltas 3 and 9).
- `guard_allow_match` — the shell holder of the allow-match core; `guard::allow_match` is the one holder (deltas 5 and 9).
- `guard_skeleton` — the shell holder of the normalizer (deltas 5 and 9).
- `guard_split_compound` — the shell holder of the splitter (deltas 5 and 9).
- `_guard_harness_view` — the shell holder of the harness view (deltas 5 and 9).
- `_guard_redirect_pairs` — the shell holder of the redirect scan (deltas 5 and 9).
- `GUARD_INPUT` — the library's cached-payload global (deltas 3 and 9).

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for the member, the rule table, the rule name, the declared view, the consumer command knob, `--guard-json view` and the fault channel.
- [ ] **Oracle green before the flip** — both decision tables and every bespoke suite pass against `--hook shell-guard` with no expected column edited beyond delta 6's named re-derivations, and the flipping commit is the one that deletes the shell guard.
- [ ] **Latency measured** — delta 3's ten-call figure is recorded in the merged §Testing.
- [ ] **Settings edit handed up** — the `.claude/settings.json` diff is prepared, stated with its grounds, and applied on the operator's behalf, never by the build session's hand.
- [ ] **Instruction surfaces: instruction only** — `templates/settings-hooks.json`'s note and `templates/guard-config.knobs`' line carry the instruction and no grounds (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each measured record in §The guard framework moves into §The shell guard, and the merged SPEC reads as one document. No dated provenance merges.
- [ ] **Amendment deleted** — this file removed on merge; no root amendment names guard-kit (`ls SPEC-*.md guard-kit/SPEC-*.md`).
- [ ] **Removals propagated** — every spelling above is declared, and `check-amendment-retired-spelling` runs each against the tracked tree.
- [ ] **Entries moved** — the five paired entries move to Done in the merge commit, which lands before the drain stage; `guard-powershell-tool-unguarded` stays Deferred as slice 2.
- [ ] **Gaps filed** — cross-component gaps found during the work are filed as debt tasks through the gap inbox.
