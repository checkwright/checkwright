# SPEC amendment: powershell-reader

The harness has a second shell tool, `PowerShell`. guard-kit's hook matches `Bash` alone, so none of this tool's calls are steered or logged. guard-kit/SPEC.md §The hook on native Windows records this as an honest limit and refuses to widen the matcher until a PowerShell reader fills the reader seam. This amendment builds that reader. It decides which generic rules apply to PowerShell commands, widens the matcher, and retires the limit.

**What the harness documents about the tool.** Read 2026-09-24 from the Claude Code docs pages `tools-reference`, `hooks`, `permissions` and `env-vars`. Each item is documented, not measured here.

- **The payload.** `tool_name` is `PowerShell`, and `tool_input` carries the Bash tool's fields: `command`, `description`, `timeout` and `run_in_background`.
- **When the tool is on.** On Windows it is on by default, and wherever it is enabled the harness routes shell commands through it as the primary shell. On Windows without Git Bash, the Bash tool is not registered at all. On Linux, macOS and WSL it is opt-in (`CLAUDE_CODE_USE_POWERSHELL_TOOL=1`, with `pwsh` on `PATH`). The harness runs `pwsh.exe` where present and `powershell.exe` 5.1 otherwise.
- **Permission rules.** They are spelled `PowerShell(…)` and take the Bash rule shape. The permission matcher parses the PowerShell syntax tree, canonicalizes common aliases, and matches without regard to case. It splits a compound on `|`, `;` and, on PowerShell 7, on `&&` and `||`, and every subcommand must match.
- **Undocumented.** Nothing says whether the matcher strips wrappers, or whether it refuses a command carrying an expansion.
- **Working directory.** It persists between calls exactly as the Bash tool's does.
- **Hook wiring.** The hooks reference tells a shell-inspecting hook to match `Bash|PowerShell`.

So on the host this gap names, an agent's shell commands go through the PowerShell tool, and the guard sees none of them.

**The ruling: which rules apply to PowerShell.** A generic rule applies to PowerShell commands when two things hold:

- its subject is a fact about something other than the shell: git's own argv, the tracked tree, the liveness records, the checkout boundary, or the harness's persisted working directory;
- the only PowerShell vocabulary its test needs is a roster of cmdlet names and their built-in aliases. That is shell-substrate knowledge, which §The generic ruleset's clause already admits.

Seven of the table's 27 rules pass that test. They take the PowerShell reader's segments, quoting and words, and their test is otherwise the one they run on Bash: `cd_compound`, `scratch_redirect`, `git_grep`, `git_mutation_under_producer`, `git_rewrite`, `rm_tracked` and `worktree_confinement`.

Every other rule stays Bash-only, and each class has its own ground:

- **The grants and rewrites** (`abs_script`, `brace_glyph`, `truncate_scratch`, `append_scratch`, `ro_pipeline`, `bounded_wait`). Each grant's safety argument is a model of the Bash permission matcher and of bash's redirect and emitter grammar. The PowerShell matcher reads a syntax tree and canonicalizes aliases, and nothing here has measured it. So no call the PowerShell tool makes is granted by the guard. A missing grant costs a permission decision, while a wrong one would bless a call.
- **The steers whose ground is the Bash matcher's own behavior** (`git_c_root`, `abs_prefix`, `expansion`, `allowlist_chain`, `emitter_write`, `grant_path_slot`). Each steers toward the spelling a `Bash(…)` entry grants, or away from one the Bash matcher refuses. Neither premise is documented for the PowerShell matcher, and the kit recommends no `PowerShell(…)` entry.
- **The tool and discipline steers that read a bash utility's vocabulary** (`sed_file`, `find_glob`, `cat_file`, `pgrep_self_match`, `bare_sleep`, `shell_wrapper`). A PowerShell counterpart would read cmdlet spellings that no measurement has ranked. The friction log this amendment opens to PowerShell calls is what earns one, through the close-stage triage (§The triage criterion). This is §The generic ruleset's own *measure first, then grant* order, applied to steers.
- **Two harm rules whose PowerShell form needs mechanism this unit does not have.** Each is filed as a costed gap (delta 7).
  - `background_no_record`'s corrective writes a record whose PID the liveness predicate must answer. On native Windows that predicate asks Git Bash's `kill -0`, then `ps` (`native/src/evidence.rs` `signal_zero`, its non-unix arm), while a PowerShell session names a Windows process id. **Inferred, cannot run on this host:** whether either answers such an id. So a PowerShell corrective could name a record no reader can judge.
  - `script_interpreter` steers to the `--scratch-run` runner, which runs a bash body only, so a PowerShell script body has no runner to be steered to.

## What changes

### (1) The PowerShell reader {design-bearing}

**Not yet applied.** `native/src/guard/powershell.rs` implements the reader seam (§The reader and its views) for PowerShell's grammar, beside `native/src/guard/bash.rs`, and adds no method to the seam. What it models is quoting, comments, escapes and separators. Everything else, as with the bash reader, is treated as live, which errs toward matching.

- **The four inert classes, mapped.** `sq` is a single-quoted span `'…'`, inside which a doubled `''` is one literal quote. `dq` is an expandable span `"…"`, inside which a doubled `""` and a backtick-escaped character (`` `" ``, ` `` `` ` ``) do not end the span. `hd` is a whole here-string literal, verbatim `@'…'@` or expandable `@"…"@`: the opener ends its line, and the closing `'@` or `"@` begins a line, optionally after blanks. `hdq` is the verbatim subset, the one PowerShell guarantees does not expand.
- **A here-string becomes one placeholder, opener to terminator.** A bash heredoc is a redirect whose body follows on later lines. A PowerShell here-string is an ordinary string value inside its statement: `@'` … `'@ | git commit -F -` is one statement. Replacing the whole literal keeps that statement on one line of the skeleton, so the segment and statement splits carry no heredoc residue.
- **Comments.** A `#` that begins a token (at a line start, after a blank, or after `;`, `|` or `(`) opens a comment to the end of its line. `<#` opens a block comment that runs to `#>` and may span lines. Every skeleton view replaces a comment with one blank, which keeps the tokens on either side apart. The `raw` view keeps it.
- **Escapes.** Outside a single-quoted span, a backtick escapes the character after it. So an escaped quote opens no span, an escaped `;` or `|` separates nothing, and a backtick before a newline continues the line rather than ending a segment.
- **The splits.** The compound split cuts on `||`, `&&`, `;`, `|` and newline, taking the two-character separators whole. It is the harness's documented PowerShell split, applied to a skeleton the way the bash reader applies its own. The statement split cuts on the same set without `|`. The pipe split cuts on `|`. A trailing `&`, PowerShell 7's background-job operator, is not a separator, just as bash's is not.
- **Residue.** `residue_statements` returns the statement split unchanged, and `heredoc_terms` returns nothing, because a here-string leaves no residue.
- **Redirect pairs.** An operator `[1-6*]?>>?`, a blank run, then `&[1-6]` or a target word. PowerShell reserves `<`, so it has no read redirect.
- **The dequoted view.** The same lockstep over the `sq dq hd` skeleton the bash reader runs. It unescapes a single-quoted span's `''` and an expandable span's `""` and backtick escapes, and it refuses to align (`None`) on a here-string, an unterminated span, or a newline inside quotes.
- **`body`.** The `k`th here-string's content.
- **The harness view.** The identity. No wrapper strip is documented for the PowerShell matcher, and no rule that applies to PowerShell reads this view.

**The placeholders.** The reader spells its placeholders through the same constants `bash.rs` uses, so it keeps whichever placeholder contract the bash reader has. Which one that is depends on the sibling unit `guard-placeholder-letter-paths` (merged into guard-kit/SPEC.md §The reader and its views):

- **Landed first, or ahead of this unit in the same batch,** which is the lead's stated order: the NUL-led marks, with the control-byte refusal covering PowerShell commands as well.
- **Not landed:** the bare letters, carrying that unit's defect into this reader until it lands. Nothing in this unit needs re-editing when it does.

**The reader's stated bounds:**

- The typographic quotes PowerShell also accepts (`‘ ’ ‚ ‛` and `“ ” „`) are not modelled, so their spans read live.
- An expandable span containing `$(` ends at the first unescaped `"`, even inside the subexpression, as the bash reader's double-quoted span does. Every applied rule declines on a live `$` first (delta 3), so a mis-scan of that span decides nothing.
- A command inside a script block, a subexpression or a parenthesized pipeline is read only where a separator happens to begin its segment.

`native/src/guard/powershell.rs`'s own `#[cfg(test)]` tests pin every bullet above, and where the two grammars differ they sit beside `bash.rs`'s tests.

In guard-kit/SPEC.md §The reader and its views, the opening paragraph's "The bash reader is `native/src/guard/bash.rs`" gains its sibling: "and the PowerShell reader is `native/src/guard/powershell.rs`". A new paragraph after the skeleton's two bounds states the mapping in the words of the bullets above, and states the three bounds as that reader's honest limit.

### (2) The member selects it and logs its fall-throughs {design-bearing}

**Not yet applied.** `crate::guard::reader_for` answers `PowerShell` with the PowerShell reader and a new `Shell::PowerShell`. The engine's walk (`engine::decide`) then runs only the table rows naming that shell.

A PowerShell fall-through is written to the same friction log, as `PowerShell`, a raw tab, and then the command encoded exactly as a Bash line is encoded. The encoding writes every tab as `\t`, so no Bash line carries a raw tab. That makes a raw tab an unambiguous marker, and it leaves every existing Bash line and the whole Bash log grammar unchanged.

In guard-kit/SPEC.md §The shell guard:

- **The call**'s "`Bash` selects the bash reader" becomes "`Bash` selects the bash reader and `PowerShell` the PowerShell reader". Its "which is the slot a second reader fills (§The hook on native Windows)" is deleted.
- **The fall-through line** gains: "A call the PowerShell reader read is written as `PowerShell`, a tab, then the encoded command. The encoding leaves no raw tab in a line, so the tab marks the tool and every Bash line reads as it always did (§scan-prompts)."
- The payload paragraph's "On a Bash call `tool_input` carries" becomes "On a Bash or PowerShell call `tool_input` carries". The field list gains `timeout`, which the harness documents for both tools and no rule reads.
- The opening paragraph's "`templates/settings-hooks.json`'s `Bash` matcher group" becomes "`templates/settings-hooks.json`'s `Bash|PowerShell` matcher group". §The friction loop's "wired as the `PreToolUse(Bash)` hook" becomes "wired as the `PreToolUse(Bash|PowerShell)` hook".

### (3) Each rule names its shells, and seven name PowerShell {design-bearing}

**Not yet applied.** §The generic ruleset's item grammar gains an optional **shells clause**: the word `Shells`, then one or more backticked shell names, `bash` or `powershell`, joined by commas or the word `and`. An item carries at most one. An item with none applies to `bash` alone, so the twenty Bash-only items need no edit.

The seven applied items gain "Shells `bash` and `powershell`." after their declaration clause, and their rows in `native/src/guard/rules/mod.rs` name both shells. Each item also gains what its PowerShell reading adds, and nothing else changes in its test:

- **`cd_compound`.** Under PowerShell the location commands are `cd`, `chdir`, `sl`, `Set-Location`, `pushd`, `Push-Location`, `popd` and `Pop-Location`, matched without regard to case. The rule fires when one of them is the first word of a segment of the PowerShell reader's compound split and that split yields more than one non-empty segment. So an escaped separator, which the split does not cut at, makes no compound.
- **`scratch_redirect`.** Under PowerShell the operator class adds `*` and the digits `1` to `6`. A `\` in the target is a path separator, so `logs\x.log` is not a bare name.
- **`git_grep`, `git_mutation_under_producer` and `git_rewrite`.** No PowerShell-specific text. git's argv is the same under both tools, and so are the record and its home.
- **`rm_tracked`.**
  - The deletion commands are `rm`, `del`, `erase`, `ri`, `rd`, `rmdir` and `Remove-Item`, matched without regard to case. A comma-joined word is several paths, and a `-Name:value` word carries its value.
  - A word following one of these value-taking parameters is not a path: `-Filter`, `-Include`, `-Exclude`, `-Stream`, `-Credential`, and the common parameters `-ErrorAction`, `-ErrorVariable`, `-WarningAction`, `-WarningVariable`, `-InformationAction`, `-InformationVariable`, `-OutVariable`, `-OutBuffer`, `-PipelineVariable` and `-ProgressAction`. PowerShell takes an unambiguous prefix of a parameter name, and so does this walk.
  - Any other `-` word is skipped, and a word naming `-WhatIf` by any prefix from `-wh` up declines the segment, because it deletes nothing.
  - The steer and the `git rm` force arm are unchanged, since both read git.
- **`worktree_confinement`.**
  - Under PowerShell a path word's `\` is a separator. The comparison with the two roots folds `\` to `/`, and a drive-letter root is rooted, by `gate_path_rooted`'s rule.
  - The admitted read is not offered, because no declared-forms reader exists for PowerShell commands. So every main-checkout path word outside a scratch dir blocks, and the corrective names the harness's read tool and the scratch journal and omits the admitted read.
  - Its honest limits gain one: the comparison is case-sensitive, so a path word that differs from the root only in case, which NTFS resolves to the same directory, escapes it.

**The expansion decline, read through the reader.** Each applied rule already declines on an expansion or a substitution anywhere in the command, and says rule `expansion` blocks those shapes first. Under PowerShell no earlier rule blocks them, so the decline stands on its own:

- A `$` outside a single-quoted span and a verbatim here-string declines, read on the `sq hdq` view.
- A backtick does not decline, because it is PowerShell's escape character.
- An applied rule reaching a bash-only helper on the raw command gets the reader's answer instead. The rule reads the running shell from its context, which carries the shell `engine::decide` was called with.

`check-guard-registration`'s assertion C gains the second set: an item's shells, `bash` where it carries no shells clause, must equal the table's shells for that rule, as sets. A finding names the rule and both sides. The synthetic table-file line becomes `<name><TAB><view>[; <view>…][<TAB><shell>[, <shell>…]]`, and an absent third field reads as `bash`. Its fixture pair gains a two-shell item in `good/` and a shells mismatch in `bad/`. A shells clause the grammar cannot parse, or a shell name the crate does not carry, is exit 2, as a malformed declaration is.

### (4) scan-prompts reads the PowerShell lines apart {design-bearing}

**Not yet applied.** `--emit scan-prompts` reads a tab-marked line as a PowerShell call. Such a line is decoded as a Bash line is. It is kept out of the headline, the ranking, the overlay and unreachable sections and `--count`, because the grant test matches `Bash(…)` entries only.

After the existing sections the report prints one advisory section: `<p> PowerShell fall-through(s), not ranked against the allowlist`, then one row per key with its call count. The key is the first segment's first word as the PowerShell reader splits it, lowercased because PowerShell command names ignore case. The section is absent when the log holds no such line.

`kpi-prompt-friction` reads `--count`, which moves no Bash line, so the series takes no step. §scan-prompts records this as its steps are recorded: a reading before and after the landing on one log, and the same two integers.

In guard-kit/SPEC.md §scan-prompts, the three-way split's paragraph gains: "A PowerShell line (§The shell guard, the fall-through line) is none of the three. It is counted in its own advisory section, keyed by its command word, because the grant test reads `Bash(…)` entries alone and would call every such line prompting."

### (5) The wiring widens and the limit retires {mechanical}

**Not yet applied.**

- **`guard-kit/templates/settings-hooks.json`.** The shell guard's matcher becomes `Bash|PowerShell`. Its `//` comment's closing clause about the unguarded `PowerShell` tool becomes: "the shell-guard reads both shell tools; a consumer rule command receives both, and reads `tool_name` to tell them apart."
- **§The hook on native Windows.** "**The rules are compiled, and a second shell is a second reader.**" is re-phrased in the present tense: the PowerShell reader is that second reader, and the rules each row's shells clause names are the ones its commands meet. The "**The honest limit: the harness's `PowerShell` tool is not guarded.**" paragraph is replaced by one that states what a PowerShell call meets: the seven applied rules, no grant, and a fall-through line in the friction log. It ends with the two filed gaps (delta 7) as that paragraph's honest limit. The ruling's grounds above move into §The generic ruleset's item-grammar paragraph as the shells clause's rule.
- **§Writing a consumer rule.** Gains: "Under the `Bash|PowerShell` matcher a consumer command receives both tools' payloads. A rule written for one shell's grammar reads `tool_name` first and says nothing on the other."
- **`guard-kit/README.md` step 2.** "the `shell-guard` on `PreToolUse(Bash)`" becomes "the `shell-guard` on `PreToolUse(Bash|PowerShell)`".
- **`guard-kit/templates/close-triage.md` step 2.** Gains one sentence: "A row in the PowerShell section takes the same criterion. Its allowlist entry is spelled `PowerShell(…)`, and its guard-rule disposition is a PowerShell reading of the generic rule it matches."
- **`guard-kit/smoke/install.sh`.** After the Bash payload it drives a `PowerShell` payload through the wired member, `Set-Location deploy; Get-ChildItem`, and asserts rule `cd_compound`'s block. So the merged wiring and the new reader are self-verifying in the scratch consumer.

This repository's own `.claude/settings.json` keeps its `Bash` matcher, because the tool is opt-in on the development host and off there. Widening it is a settings edit applied on the operator's behalf (guard-kit/SPEC.md §compare-settings-allow), and this unit does not owe it.

### (6) The decision table and the seam suites {mechanical}

**Not yet applied.** A fourth table, `guard-kit/guard-tests/powershell-cases.tsv`, takes `cases.tsv`'s grammar and substitutions. `--run-guard-tests` feeds every row as a `PowerShell` payload, so the table runs wherever the arm does, the binding native-Windows install-smoke leg included, and the member needs no `pwsh` to decide. Its rows:

- **Each applied rule firing and not firing:**
  - `Set-Location deploy; Get-ChildItem` blocks; `cd deploy` does not.
  - `git status > out.log` blocks; `git status > .tmp/out.log` and `git status 2>&1` do not.
  - `git grep foo` blocks; `git grep foo HEAD` does not.
  - `git commit --amend --no-edit` advises; `git commit -m 'x'` falls through past the dead record.
  - `Remove-Item tracked.md`, `rm tracked.md` and `git rm -f tracked.md` block. `del scratch.txt`, `Remove-Item -Path tracked.md -WhatIf` and `git rm -q tracked.md` do not.
- **The reader's grammar**, each row a command an applied rule would block if the reader misread it:
  - a single-quoted span carrying `''` and `; cd x`;
  - an expandable span carrying `` `" `` and `; cd x`;
  - a verbatim here-string whose body holds `cd x; rm tracked.md`, piped into `git commit -F -`, which advises on rule `git_rewrite` and nothing else;
  - `# cd x; ls` and `<# cd x #> git status`;
  - `` git status `; cd x ``;
  - `cd x && git status` and `cd x || git status`, which block.
- **The Bash-only rules staying inert:** `cat tracked.md`, `echo x >> .tmp/j.md`, `Get-ChildItem | Select-Object -First 3`, `sleep 5`, `bash -c 'ls'` and `$x = 1; git status` each fall through.

§Testing gains a paragraph for the table, on the third table's precedent: its input differs from `cases.tsv`'s in the payload's `tool_name` and nothing else, and the member it drives is the same.

The seam suites each gain PowerShell cases:

- `git-mutation-under-producer.test.sh`: a PowerShell `git commit` blocked under a live record.
- `worktree-confinement.test.sh`: a PowerShell `Set-Content` into the main checkout blocked, a backslash-spelled one blocked, and a scratch-dir journal append falling through.
- `consumer-rules.test.sh`: `view` over a PowerShell payload printing its skeleton. The existing non-shell `view` case keeps its tool, `Read`.
- `scan-prompts.test.sh`: a log mixing both tools. It asserts the Bash headline and `--count` equal to the Bash-only log's, and the PowerShell section keyed and counted.

`native/src/emit/run_guard_tests.rs` reads the fourth table beside the first, and it stays on disk for the first table's reason.

### (7) The two gaps are filed {mechanical}

**Applied at spec**, in the commit that added this file: two costed gaps went to `.workflow/gap-inbox.md` through `--emit file-gap`.

- **A PowerShell-backgrounded producer writes no record.** Rule `git_mutation_under_producer` cannot see such a producer. The PowerShell form of rule `background_no_record` waits on a liveness predicate known to answer a Windows process id on native Windows, which is unmeasured today.
- **A PowerShell script run off a scratch-dir body is unsteered:** `& .tmp\x.ps1`, `.\.tmp\x.ps1`, `pwsh -File .tmp/x.ps1`. No runner echoes a PowerShell body.

Rule `grant_path_slot` does not bound a `PowerShell(…)` grant. That is not filed. It is one instance of the matcher premise the steers class above rests on, and the friction log's PowerShell section is where a recurring grant would surface.

## Producers and consumers

- **The PowerShell reader.**
  - Producer: `reader_for`, on every payload whose `tool_name` is `PowerShell`.
  - Consumers: the seven applied rules through the context; `--guard-json view`; the `scan-prompts` PowerShell key.
  - Enabling config: the `Bash|PowerShell` matcher, which `templates/settings-hooks.json` ships and a consumer merges. The harness sets the tool on by default on Windows.
- **The tab-marked log line.**
  - Producer: the member's fall-through on a PowerShell call.
  - Consumer: `--emit scan-prompts`, by the raw-tab test.
  - Every field has a reader. `PowerShell` selects the section, and the encoded command yields the key.
- **The shells clause.**
  - Producer: the SPEC roster item.
  - Consumers: `check-guard-registration` assertion C, against the table's shells; the engine's walk, through the table.
- **Roster-holding readers.**
  - `check-guard-registration` holds items, order, views and now shells. It adds no rule, so assertion B's order is unchanged.
  - The `--run-guard-tests` arm's table list gains `powershell-cases.tsv`.
  - `check-kit-registration` reads guard-kit's `gate-tests/` runner line, which is unchanged, because no new suite file is added.
- **Point 5.** No corpus narrows. The friction log's Bash lines are unchanged, and `--count` reads them alone.
- **Point 6.** The seven applied rules are enumerated by the ruling above. Each rule's satisfying value is its PowerShell clause in delta 3 and its rows in delta 6.

## Existing sections updated

Roster from `git grep -n "PreToolUse(Bash)\|Bash\` matcher\|honest limit: the harness\|second shell is a second reader\|slot a second reader\|On a Bash call\|item grammar the gate reads\|B, the order\|C, the declarations" -- guard-kit` and `grep -n "PowerShell\|tool_name" guard-kit/gate-tests/*.sh guard-kit/smoke/install.sh`, run 2026-09-24.

- `guard-kit/SPEC.md` §The reader and its views (delta 1).
- `guard-kit/SPEC.md` §The shell guard, **The call**, the fall-through line, the payload paragraph and the opening paragraph; §The friction loop's call-time item (delta 2).
- `guard-kit/SPEC.md` §The generic ruleset's item grammar, the seven applied items, and §check-guard-registration assertion C and its inputs (delta 3).
- `guard-kit/SPEC.md` §scan-prompts (delta 4).
- `guard-kit/SPEC.md` §The hook on native Windows and §Writing a consumer rule (delta 5).
- `guard-kit/SPEC.md` §Layout and configuration's tree and §Testing (delta 6).
- `native/src/guard/powershell.rs`, `native/src/guard/mod.rs` (delta 1).
- `native/src/guard/engine.rs`, `native/src/hook/shell_guard.rs`, `native/src/guard/host.rs` (delta 2).
- `native/src/guard/rules/mod.rs`, `native/src/guard/rules/spelling.rs`, `native/src/guard/rules/tools.rs`, `native/src/guard/rules/reach.rs`, `native/src/guard/rules/liveness.rs`, `native/src/gates/guard_registration.rs`, `scripts/gate-tests/check-guard-registration/`, `scripts/gate-tests/check-guard-registration.test.sh` (delta 3).
- `native/src/emit/scan_prompts.rs` (delta 4).
- `guard-kit/templates/settings-hooks.json`, `guard-kit/templates/close-triage.md`, `guard-kit/README.md`, `guard-kit/smoke/install.sh` (delta 5).
- `guard-kit/guard-tests/powershell-cases.tsv`, `native/src/emit/run_guard_tests.rs`, `guard-kit/gate-tests/git-mutation-under-producer.test.sh`, `guard-kit/gate-tests/worktree-confinement.test.sh`, `guard-kit/gate-tests/consumer-rules.test.sh`, `guard-kit/gate-tests/scan-prompts.test.sh` (delta 6).
- `.workflow/gap-inbox.md` (delta 7), written at spec.
- `.workflow/release-declarations.md`, three Behavior changes bullets (deltas 2, 4 and 5):
  - `**guard-kit/templates/settings-hooks.json**`: the shell guard's matcher is `Bash|PowerShell`. Re-merge it, and a PowerShell call then meets seven generic rules, takes no guard grant, and is logged.
  - `**GUARD_KIT_CONSUMER_RULES_CMD**`: under the widened matcher your consumer command also receives `PowerShell` payloads. Read `tool_name` in any rule written for bash grammar.
  - `**--emit scan-prompts**`: PowerShell fall-throughs print in their own advisory section, off the headline and `--count`.
<!-- update-target-exempt: generated mirror, regenerated by its freshness gate's printed command -->
- `docs/guard-kit/SPEC.md` and `docs/guard-kit/README.md`.

## Retired spellings

- None — no name is renamed or deleted. The retired honest-limit paragraph is prose and carries no governed name.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The causal-completeness check holds for the reader, the log line and the shells clause.
- [ ] **Instruction surfaces: instruction only.** The `settings-hooks.json` comment and the close-triage sentence carry no grounds.
- [ ] **Merged with no information lost.** Each SPEC edit re-phrases the passage it refines, and the ruling's grounds land in the item-grammar paragraph.
- [ ] **Amendment deleted.** This file is removed on merge (`ls guard-kit/SPEC-*.md`), the sibling placeholder-mark amendment that cited it by filename having merged already.
- [ ] **Entry moved.** `guard-powershell-tool-unguarded` moves to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** Not reached.
- [x] **Gaps filed at spec.** The two gaps of delta 7.
- [ ] **Gaps filed at build.** Any cross-component gap found during the work.
