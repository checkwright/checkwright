# SPEC amendment: side-effect-free-read-arms

**A read an agent spells with a utility that can also write or execute has a write path the guard judges per call.** `sed` carries `w` and `e` commands, `awk` carries `system()` and `print >`, and `find` carries `-exec` and `-delete`. The guard already steers some of these reads to a tool with no such path: `sed -i` goes to `--rewrite`, a `sed` or `awk` line range on a file goes to the Read tool, and an `awk` heading range on a markdown file goes to `--emit md-section` (guard-kit rule `sed_file`). The rest reach the harness's out-of-band decision. Some of those have a replacement the guard already grants, and some are side-effect-free as written, which no rule reads. An isolated child's read set trusts the roster's declarations, which is guard-kit rule `worktree_confinement`'s stated honest limit: "Only a tool whose effect is its command line would close that."

**The shape is an operator direction, relayed by the lead, not a /consult ruling.** It was first answered through AskUserQuestion on 2026-09-30. The operator clarified it in the lead session the same day, because the first answer had been read as excluding a custom tool.

- **Motivation.** A read spelled with a side-effect-capable utility falls through to the harness's out-of-band decision. In auto mode that decision is the classifier, which gives occasional false positives. This unit takes those calls off that path.
- **Standard utilities first.** Models are trained on the standard utility spellings, and a custom tool costs tokens to learn. The ladder has three rungs, taken in order:
  1. **A standard spelling with no side effect exists**: steer to it (deltas 2 and 3).
  2. **The utility's own program has no side effect**: admit it with a static program check. Where the check cannot decide, it declines, and the call stays where it is today (delta 7).
  3. **Only a residue with neither**: a custom read arm of the gate binary, authored only where the residue is real, and named (deltas 1 and 4).
- **Block only a form that can write or execute, and only where a rung-1 replacement exists.** A form with no replacement, and a program the check cannot clear, keep today's disposition.
- **Declare a read-only arm set in the crate, hold it with a crate test, and have rule `worktree_confinement` admit it.**
- **Add no allowlist entry.** The recommended `Bash(@GATE_SDK_NATIVE_BIN@ *)` grant already reaches every arm (guard-kit/SPEC.md §The recommended allowlist). An isolated child gains the confinement rule's admission, which no settings entry can express. Rung 2 admits through rule `ro_pipeline`'s grant, which is not a settings entry either.
- **True writes stay out of this unit.** `find -delete` and `tee` to a file are filed as a gap of their own.
- **No forced steer where rung 2 admits the standard spelling.** Operator direction, lead-relayed 2026-09-30 from the lead session, not a /consult ruling: rule `sed_file`'s arm (ii) blocks an `awk` heading range on a markdown file and steers it to `--emit md-section`, although that program has no side effect and delta 7's check clears it. The arm is dropped, so the read passes to the step-2 program check. `--emit md-section` stays available and advertised as an option, no longer forced (delta 8).

**The entry's seed was corrected at spec.** `FENCE_SAFE_ARMS` (`native/src/emit/mod.rs`) is not a read-only set. gate-sdk/SPEC.md §The non-gate arm admits an arm there that "writes nowhere but its working tree and stdout". So `--run`, `--run-gate-tests`, every `--emit-` arm (the `--write` modes included) and every gate are members. The read-only set is a new, narrower declaration beside it, and a subset of it.

**Measured at authoring, on this checkout.**

- **Demand.** `.workflow/prompt-friction.log` held 313 fall-through lines, mostly compounds. A segment was led by `awk` 16 times, by `sed` 3, and by `find`, `tee` or `xargs` never. The `awk` programs were counts, `NR` line ranges and markdown heading reads, and one piped `sed -n '40,200p'`. Re-read at align, the log held 473 lines. Thirty carry an `awk`- or `sed`-led segment. Twenty-three of those are compounds or carry a redirect, which rule `ro_pipeline` refuses on its own statement-separator and redirect clauses. Six are standalone pipelines of roster members with `awk` or `sed`, and one of the six is the piped range print delta 2 steers.
- **The shell guard's verdicts today** (payloads fed to `--hook shell-guard` with `GUARD_KIT_LOG` pointed at scratch):
  - `grep -n door guard-kit/SPEC.md | sed -n '5,10p'` fell through, and its replacement `grep -n door guard-kit/SPEC.md | head -n 10 | tail -n +5` was auto-allowed.
  - `find guard-kit -name '*.md' -exec grep -l door '{}' +` and its `\;` twin fell through. Their replacement `find guard-kit -name '*.md' -print0 | xargs -0 grep -l door` was auto-allowed.
  - The file-operand `sed -n '5,10p' <file>` and `awk 'NR>=5 && NR<=10' <file>` are already blocked to the Read tool. So is every `sed` with a file operand, `sed 's/a/b/' <file>` included, so rung 2 reaches `sed` only on a stream.
  - Measured at align: `grep -n x <file> | sed 's/a/b/'`, `awk '{s+=$1} END {print s}' <file>`, `grep -c x <file> | awk '{s+=$1} END {print s}'`, `awk '{print $1}' <file> | sort | uniq -c`, `awk 'NR==3||NR==13' <file> | cut -c1-120` and `awk '/^#/ {print NR": "$0}' <file>` fall through, and so does `grep x <file> | awk '{print > "out.txt"}'`.
  - `find … -delete` falls through and has no read replacement.
- **Arm modules.** A scan of each candidate module's rule text (cut at its test module, comment lines dropped) for the write and spawn APIs found none in `md_section.rs` or `md_index.rs`, and one `proc::run` of `git ls-files` in `enum_sets.rs`. `md_index.rs` reaches the `walk` module, whose `make_scratch` writes, through helpers it does not call. Read at align, `md_index.rs`'s `emit` calls `walk::toplevel_opt`, which runs `git rev-parse --show-toplevel` through `proc::run`. That is a program spawn the module scan cannot see, the crate test's honest limit.

**What stays as it is.** `GUARD_KIT_RO_FORMS` and rule `ro_pipeline`'s declared-forms test, which stays the admitted read's roster test for shell utilities. `FENCE_SAFE_ARMS` and `check-fence-run`. The recommended allowlist. Rule `worktree_confinement`'s program-bearing exclusion, so an isolated child's admitted read takes no `sed` or `awk` whatever the roster holds. Every write-or-execute form with no read replacement: `find -delete`, `tee` to a file, and a `sed` or `awk` program the check does not clear, such as an `awk` program calling `system()`.

## What changes

### (1) The crate declares a read-only arm set

`native/src/emit/mod.rs` gains `READ_ONLY_ARMS: &[&str] = &["--emit-md-section"]` beside `FENCE_SAFE_ARMS`, and a crate test holds it {design-bearing}. **Not yet applied.** gate-sdk/SPEC.md §The non-gate arm gains, after the fence-safe paragraph:

> **The read-only arm set is declared beside it and is a subset of it.** `READ_ONLY_ARMS` names the arms a read may be steered to and an isolated child may run against the main checkout (guard-kit/SPEC.md §The rule roster, rules `sed_file` and `worktree_confinement`). An arm belongs when its effect is its command line: it writes nothing but stdout and stderr, spawns no program, and reaches no network. The set holds reads, not every arm that happens to meet the contract. A crate unit test holds three things: each member names an arm-table row, each is fence-safe, and each member's module rule text carries no filesystem write, removal, rename, copy, permission or link call and no program spawn. **Honest limit:** the scan reads the member's own module, so a write or a spawn reached through a helper in another module passes it. An arm therefore joins only after its call graph has been read once, and that reading is recorded where the member is added.

The one seed member is `--emit md-section`, the rung-3 residue for an isolated child. The admitted read excludes `awk` whatever the roster holds (delta 7), so a heading-range read has no rung-2 path in a linked worktree, and without delta 4 it has no path at all there. In the main session an `awk` heading range passes rung 2 (delta 8), and the arm stays advertised as the option it is. Its call graph, read at align: `emit` calls `read_text` (`std::fs::read`) and the `section` module's line helpers, and the binary's dispatcher spawns nothing before the arm. That reading is the record the paragraph asks for, and it lands in the commit that adds the member.

`--emit md-index` was proposed as a second seed and stays out. It resolves its path column's root through `walk::toplevel_opt`, which spawns `git`. `--emit enum-sets` spawns `git` from its own module and stays out for the same reason. A heading listing is no residue: `grep -n '^#' <file>` is its rung-1 spelling, and in the main session an `awk` heading program is admitted by delta 7.

**No new arm.** The measured demand held no read that has neither a rung-1 spelling nor a program delta 7 clears.

### (2) Rule `sed_file` steers a piped range print to `head` and `tail`

Rule `sed_file` gains a stream arm. It blocks a segment fed by a pipe and led by `sed -n` whose whole program is a range print (`A,Bp` or `Ap`). It also blocks one led by `awk` whose program is arm (i)'s `NR`-comparison pattern, with no action or the print-all action. The steer is `head -n <B> | tail -n +<A>` (`head -n <A> | tail -n 1` for a single line), which rule `ro_pipeline` grants {design-bearing}. **Not yet applied.** In guard-kit/SPEC.md §The rule roster, rule `sed_file`'s item changes in four places:

- **The title**, "`sed` or `awk` reading a file, or `sed`, `perl` or an inline `python` body rewriting one", becomes "`sed` or `awk` reading a file or a stream's line range, or `sed`, `perl` or an inline `python` body rewriting one". Its comment line in `guard-kit/guard-tests/cases.tsv` follows.
- **The pipe sentence**, "A `sed` fed by a pipe is a text filter with no tool equivalent and is untouched, so the discriminator is the operand, not the binary", becomes: "A `sed` fed by a pipe is a text filter and is untouched, except a bare range print, whose equivalent is `head` and `tail`; so the discriminator is the operand and the program's shape, not the binary."
- **The `awk` arm's lead**, "**The `awk` arm fires on two read shapes and nothing else.** Both require exactly one file operand and no pipe into the segment, meaning the `awk` heads its pipeline.", becomes: "**The `awk` arm fires on three read shapes and nothing else.** Arms (i) and (ii) require exactly one file operand and no pipe into the segment, meaning the `awk` heads its pipeline. Arm (iii) requires a pipe into the segment and no operand." The arm list gains:

  > - **(iii) A piped line-range read.** The (i) pattern with the segment fed by a pipe and no file operand. The steer names `head -n <B> | tail -n +<A>`, the read-only roster's spelling of the same lines, which rule `ro_pipeline` grants where it grants the pipeline's other segments.

- **The pass list**, "An action, a program file, a non-`.md` range, a second operand, a `-` stdin operand, or a stream all pass.", becomes "An action, a program file, a non-`.md` range, a second operand, a `-` stdin operand, or a stream outside arm (iii) all pass." The paragraph's "the steer fires on the two read shapes ahead of any grant" becomes "the steer fires on the three read shapes ahead of any grant".

**The ground is the replacement, not the frequency.** A range print has an exact equivalent in two roster members with no write or execute form. A substitution, a transform or a filter has none, and passes to delta 7's check. The arm reads the program on the dequoted view and declines where that view does, as arms (i) and (ii) do. It is placed where rule `sed_file` already sits, ahead of the auto-allow band, so rung 1 meets a read before rung 2's grant can.

### (3) Rule `find_exec` steers a roster command under `-exec` to `xargs`

A new generic rule is placed directly after rule `find_glob` {design-bearing}. **Not yet applied.** Its roster item:

> - **A read-only command run by `find -exec`** (`find_exec`) — Declares `sq dq hd` and `dequoted`. A segment led by `find` whose only execute form is one `-exec … '{}' +` or `-exec … '{}' \;` is **blocked** when the command it runs is one rule `ro_pipeline`'s segment test counts read-only: a `GUARD_KIT_RO_BINS` member in none of its declared write and execute forms, clearing that rule's program discriminator where the member carries one. The terminator is the word `;` or `+` after dequoting, so `\;`, `';'` and `";"` are one spelling. The steer is `find <predicates> -print0 | xargs -0 <command>` (`xargs -0 -n 1` for the `;` form), which rule `ro_pipeline` grants through its `xargs` discriminator. `-exec` is one of `find`'s declared execute forms, so the read falls through today although its pipeline spelling is granted. The replacement runs the same command over the same files, and its effect is readable from its command line. **Declines** on any other execute or write form in the segment (`-execdir`, `-ok`, `-delete`, `-fprint`), on an executed command the segment test withholds, on more than one `-exec`, and on an unquoted `{}`, which rule `brace_glyph` blocks first. Placed before the auto-allow band on rule `find_glob`'s reasoning, and independent of `GUARD_KIT_SEARCH_TOOLS`, since its steer names no harness tool.

The crate half is a row in `native/src/guard/rules/mod.rs`'s table after `find_glob`, and a rule function beside it. Its declared views are whatever the item says (`check-guard-registration` arms B and C).

### (4) Rule `worktree_confinement` admits the read-only arm set

The admitted read gains a second form: a segment whose command word is the gate binary or the front end, and whose arm is a `READ_ONLY_ARMS` member {design-bearing}. **Not yet applied.** The binary counts as the steer door's spelling of `GATE_SDK_NATIVE_BIN`, or that value resolved against the main checkout. The front end counts as `gate-sdk/bin/run-gates.sh` under `bash`. The arm's `--emit <name>` spelling is normalized to `--emit-<name>` as the binary normalizes it. In guard-kit/SPEC.md, the rule's first admitted-read bullet becomes: "Every segment passes rule `ro_pipeline`'s roster test … or runs a `READ_ONLY_ARMS` member on the gate binary or the front end (gate-sdk/SPEC.md §The non-gate arm)." The honest-limit bullet "The admitted read is exactly as safe as the roster's declarations … Only a tool whose effect is its command line would close that." becomes: "A roster-led read is exactly as safe as the roster's declarations, which is rule `ro_pipeline`'s own limit: a member gaining a write option is a hole until it is declared. A read-only arm closes that for the reads it replaces." The corrective's advertisement list gains the arm set, interpolated from the crate's declaration like the roster, and printed with the steer door. So §The shell guard's steer-door paragraph, "The steers that name it are rule `sed_file`'s section extractor and `--rewrite`, and the `--scratch-run` runner rules `script_interpreter` and `shell_wrapper` name", gains rule `worktree_confinement`'s advertised arms. The PowerShell exclusion stays: the admitted read is still not offered there.

### (5) delegation-kit's isolated-child read set names the arms

Three surfaces name the arm set beside the pipeline {mechanical}. **Not yet applied.**

- **delegation-kit/SPEC.md** says rule `worktree_confinement` "admits a read-only pipeline over the main checkout, in the one form unable to write there". It gains the arm set.
- **Isolation cost (3) in delegation-kit/templates/agent-execution.md** reads "searches under it only with a read-only pipeline of the shell guard's read-only roster" and then "its refusal names the roster". These become "searches under it only with a read-only pipeline or a read-only arm of the gate binary, the forms the shell guard's refusal names" and "its refusal names both".
- **`.claude/agents/audit-sweep.md`** is that bullet's operative-residency copy (delegation-kit/SPEC.md §Operative residency). It reads "search only with a read-only pipeline of the bash guard's read-only roster" and then "the refusal names the roster's members". These become "search only with a read-only pipeline or a read-only arm of the gate binary, the forms the bash guard's refusal names" and "the refusal names both".

The re-phrasing points at the refusal rather than the roster, because after delta 7 the roster holds `sed` and `awk`, which the admitted read excludes.

### (6) The queue entry's seed wording is corrected

In the promoting commit, the entry's "the seed is gate-sdk's fence-safe arm set, `FENCE_SAFE_ARMS` (gate-sdk/SPEC.md, arms that reach no network and write nowhere)" is corrected. It now says that the fence-safe set admits working-tree writes, and that the seed is a read-only subset declared beside it {mechanical}. Applied with this file. At align, the entry's deliverable ("…and the allowlist entries") was re-stated to the ladder and to no allowlist entry, and the direction's motivation was recorded on it. Applied.

### (7) Rule `ro_pipeline` admits a `sed` or `awk` program the static check clears

`sed` and `awk` join the default `GUARD_KIT_RO_BINS` roster, and each carries a program discriminator, as `xargs` carries one {design-bearing}. **Not yet applied.** A `sed` or `awk` segment counts read-only only when two things hold. `program_operands`, rule `sed_file`'s walker, must yield every program text from the command line. And the static program check must clear each one. The discriminator rides the one declared-forms reader, so every rule applying that reader applies it:

- rule `ro_pipeline`'s grant;
- rule `background_no_record`'s exemption (3);
- rule `bounded_wait`'s clauses (c) and (d);
- rule `find_exec`'s executed-command test;
- the `xargs` discriminator, for a `sed` or `awk` that `xargs` runs.

Rule `worktree_confinement`'s program-bearing exclusion stays, so the admitted read does not take them. That is the conservative direction for the one defence against an isolated child's write into the main checkout, and a static program check is a new reader there. The corrective's interpolated roster omits the program-bearing tools, since the admitted read excludes them whatever the roster holds.

The check reads the program on rule `sed_file`'s dequoted view and declines wherever that view does. It also declines on anything it does not model. Declining withholds the segment, which leaves the call where it is today.

- **`sed`.** Declaration `-i --in-place`. A program file (`-f`, `--file`) withholds, since its text is not on the command line. Each script, from `-e`, `--expression` or the first bare word, is walked command by command. A command is an optional address or address range (a line number, `$`, `/re/` or `\cREc` with its `I` and `M` flags, `first~step`, `addr,+N` or `addr,~N`), an optional `!`, then the command letter. The check withholds on the `w` and `W` commands, on the `e` command, and on an `s` command whose flags carry `w` or `e`. It walks the arguments of `s` and `y` by their delimiter, and the text of `a`, `i` and `c` to the end of the line. A `{` block is walked as a sequence. A command letter outside `s y p P d D n N g G h H x b t T : = l q Q z F r R a i c { } #` withholds. `r` and `R` read a file and pass.
- **`awk`.** Declaration `none`. `program_operands`' `awk` row declines on every option but `-F`, `-v` and `--`, so gawk's source-loading and extension options (`-e`, `-E`, `-i`, `-l`) never reach the check, and a program file (`-f`) withholds. The program is tokenized with string literals, regex literals and comments skipped, and a `/` is a divide after an operand and a regex elsewhere, by awk's lexical rule. The check withholds on:
  - a call to `system`;
  - a `print` or `printf` statement carrying `>`, `>>` or `|` outside parentheses;
  - a `|` feeding `getline`, which runs a command;
  - a `|&`;
  - an `@` directive;
  - an unterminated string or regex.

  A plain `getline` and `getline < file` read, and pass. So does a `>` or `<` outside a `print` statement, which is a comparison.

**The honest limit.** The check reads spelling, not semantics, as rule `sed_file`'s `python` arm does. The spellings it reads are POSIX's and GNU's. An implementation that runs a command or writes a file through a construct the check reads as inert is a hole until the check models it. A consumer that will not carry that limit drops `sed` and `awk` from its `GUARD_KIT_RO_BINS`, which restores today's disposition. That is the knob's existing policy-as-choice, and it mints no new name.

**Placement.** Rule `ro_pipeline` keeps its position. Rule `sed_file`'s steers stay ahead of it, so a read with a rung-1 replacement meets the steer first, and a file-operand `sed` meets the Read steer as it does today. So rung 2 reaches two things: a stream `sed` whose program is not a bare range print, and an `awk` program, on a stream or a file, that arms (i) and (ii) do not claim, delta 8 having dropped the heading-range arm.

In guard-kit/SPEC.md §The rule roster, rule `ro_pipeline`'s item gains a discriminator paragraph after the `xargs` one. It states the two members, the check above, its declining direction and its honest limit. The kit's declared-forms table gains `sed`: `-i --in-place` and `awk`: `none`. Rule `sed_file`'s placement sentences change. Today they read "a consumer that widens `GUARD_KIT_RO_BINS` with `sed` would otherwise have rule `ro_pipeline` silently grant an in-place rewrite. `awk` has no honest place on that roster, since a program can print to a file or call `system()`, and the placement guards a consumer who adds it anyway." They become: "a read with a steer meets it ahead of rule `ro_pipeline`'s grant, and the in-place arms block whatever the roster declares. `sed` and `awk` sit on that roster only behind its program discriminator, since a program can print to a file or call `system()`." §Layout and configuration's `GUARD_KIT_RO_BINS` bullet reads "plus `xargs`, whose membership is qualified by rule `ro_pipeline`'s discriminator". That becomes "plus `xargs`, `sed` and `awk`, whose membership is qualified by a discriminator of rule `ro_pipeline`'s". The same two members join the crate default in `native/src/knobs/guard_kit.rs`, the kit's forms table in `native/src/guard/rules/mod.rs`, and this repository's explicit roster in `scripts/guard-config.knobs`.

### (8) Rule `sed_file` drops its heading-range arm

The operator direction above {design-bearing}. **Not yet applied.** Rule `sed_file`'s arm (ii), an `awk` `/re1/,/re2/` range on a `.md` operand steered to `--emit md-section`, is removed from the rule and its roster item. The heading read reaches delta 7's check like any other `awk` program the steer arms do not claim, and it is admitted there. Applied together with delta 2:

- delta 2's arm (iii) is numbered (ii), so the `awk` arm fires on two read shapes: (i) the line-range read with one file operand and no pipe into the segment, (ii) the piped line-range read with no operand;
- delta 2's "three read shapes" in the lead and in "the steer fires on the … read shapes ahead of any grant" read "two";
- the pass list reads "An action, a program file, a regex range, a second operand, a `-` stdin operand, or a stream outside arm (ii) all pass.";
- "Three shapes are refused as steer inputs — a transform, a single-regex filter, and a range on a non-markdown file" becomes "A transform, a single-regex filter and a regex range are refused as steer inputs", and the frequency sentence before it stays, since the shape still decides between the Read tool and the `head` and `tail` spelling.

**The option stays advertised.** The session-context hook already lists `--emit md-section` among the arms, and the `sed` file-operand Read steer already names the section extractor for a markdown section. Arm (i)'s Read steer names it too, through the steer door, as the option for a markdown section. So §The shell guard's steer-door paragraph and §The recommended allowlist keep naming rule `sed_file`'s section extractor as a steer that names the binary. Rule `worktree_confinement`'s corrective advertises it to an isolated child (delta 4).

## Producers and consumers

- **`READ_ONLY_ARMS`.** Producer: the crate declaration. Consumers: rule `worktree_confinement`'s admission and corrective (delta 4), and the crate test (delta 1). Rule `sed_file`'s markdown steer already names `--emit md-section` through the steer door and does not read the set.
- **The stream arm and rule `find_exec`.** Producer: the shell guard's rule table, on every `Bash` payload in every consumer wiring the guard. Consumer: the agent, through the block. Readers of the new rule name: `check-guard-registration` (arms A to D), the decision table, and the generated `docs/guard-kit/SPEC.md` mirror.
- **The program discriminator.** Producer: the declared-forms reader, on every `Bash` payload whose segment leads with `sed` or `awk`, or whose `xargs` or `find -exec` runs one, where `GUARD_KIT_RO_BINS` holds the member. It does so by default, and in this repository through the explicit roster delta 7 extends. Consumers: the five rule sites delta 7 lists. The one input it reads beyond the command is the roster knob.
- **The admission.** Producer: rule `worktree_confinement`, selected by `GUARD_KIT_WORKTREE_READS` at `read-only`, the default, so every consumer that dispatches an isolated child reaches it. Consumer: the isolated child, which runs the arm.
- **Decision-table rows** (`guard-kit/guard-tests/cases.tsv`), each rule's firing and non-firing pair:
  - Firing, `block`: `grep -n x tracked.md | sed -n '5,10p'`, `grep -n x tracked.md | awk 'NR>=5 && NR<=10'` and `find . -name '*.md' -exec grep -l x '{}' +`.
  - Firing, `allow`: `grep -n x tracked.md | sed 's/a/b/'`, `grep -c x tracked.md | awk '{s+=$1} END {print s}'` and `awk '{print "a > b | c"}' tracked.md`. The last carries the redirect glyphs inside a string, which the tokenizer skips.
  - Non-firing: `grep -n x tracked.md | head -n 10 | tail -n +5` and `find . -name '*.md' -print0 | xargs -0 grep -l x`, which are allowed. Falling through: `find . -name '*.tsv' -delete`, `grep x tracked.md | awk '{print > "out.txt"}'`, `grep x tracked.md | sed 's/a/b/w out.txt'`, `awk 'BEGIN {system("ls")}'`, `grep x tracked.md | awk '{"date" | getline d; print d}'` and `grep x tracked.md | sed -f prog.sed`.
  - Each existing row carrying a piped `sed -n`, a piped `awk`, a `find -exec`, or a `sed`- or `awk`-led segment is re-derived under §Testing's non-monotone rule. Read at align, rows `find . -exec grep foo '{}' +` and `find src -type f -exec grep -q foo '{}' ';'` move from `fallthrough` to `block`. Rows `awk '$1' data.txt`, `awk '{print $1}'`, `sed -E 's/a/b/'`, `grep foo file | sed -e s/a/b/`, `awk 'NR>=10 && NR<=40 {print $2}' notes.txt`, `awk '/needle/' notes.txt`, `awk '/^## Layout/,/^## Testing/' notes.txt`, `awk 'NR<=5' a.txt b.txt`, `grep -n foo notes.txt | awk 'NR<=5' notes.txt` and `grep foo file | awk '{print}'` move from `fallthrough` to `allow`. Row `awk '/^## Layout/,/^## Testing/' SPEC.md` moves from `block` to `allow` (delta 8). Rows `awk -f prog.awk notes.txt` and `awk -W posix 'NR<=5' notes.txt` stay `fallthrough`. The build's run is the oracle for each.
- **The admission's cases** join `guard-kit/gate-tests/worktree-confinement.test.sh`, the harness for rows a table cannot vary. From a scratch linked worktree, `<door> --emit md-section <main>/x.md "H"` is admitted, and `<door> --emit md-unwrap --write <main>/x.md` is still blocked. That suite's existing case already covers the program-bearing exclusion: a roster adding `awk` still has `awk` over a main-checkout path blocked. It stays as the case holding delta 7's exclusion, and a case holds the corrective's roster to omitting `sed` and `awk`.

## Existing sections updated

- `gate-sdk/SPEC.md` — §The non-gate arm, the read-only set's paragraph (delta 1).
- `native/src/emit/mod.rs` — the declaration and its test (delta 1).
- `guard-kit/SPEC.md`:
  - §The shell guard, the steer-door paragraph (delta 4).
  - §The rule roster: rule `sed_file`'s title, pipe sentence, `awk` arm lead, arm (iii) and pass list (delta 2), arm (ii) and the steer-input sentence (delta 8), and its placement sentences (delta 7). The rule `find_exec` item (delta 3). Rule `ro_pipeline`'s discriminator paragraph and forms table (delta 7). Rule `worktree_confinement`'s admitted read, corrective and honest limit (delta 4).
  - §Layout and configuration's `GUARD_KIT_RO_BINS` bullet (delta 7).
  - §Testing's `worktree-confinement.test.sh` paragraph, which gains the admission's cases and the corrective's omission (deltas 4 and 7).
- `native/src/guard/rules/` — the stream arm, the new rule, the admission, the discriminator, the corrective and the dropped heading-range arm (deltas 2, 3, 4, 7 and 8).
- `native/src/knobs/guard_kit.rs` and `scripts/guard-config.knobs` — the roster's two members (delta 7).
- `guard-kit/guard-tests/cases.tsv` and `guard-kit/gate-tests/worktree-confinement.test.sh` — the cases (deltas 2, 3, 4 and 7).
- `delegation-kit/SPEC.md`, `delegation-kit/templates/agent-execution.md` and `.claude/agents/audit-sweep.md` — the isolated-child read set (delta 5).
- `TASK-QUEUE.md` — the entry's seed wording and deliverable (delta 6).
- `docs/gate-sdk/SPEC.md`, `docs/guard-kit/SPEC.md` and `docs/delegation-kit/SPEC.md` — the generated mirrors (all deltas).

The roster came from `git grep -n 'worktree_confinement\|FENCE_SAFE\|read-only roster\|admitted read'` over the tracked tree, and `git grep -n 'sed_file\|find_glob'` for the readers of a neighbouring rule name. At align it was re-derived with `git grep -n 'GUARD_KIT_RO_BINS\|no honest place\|reading a file, or'` for delta 7's surfaces, and delegation-kit/templates/agent-execution.md's operative-residency pointer for delta 5's third surface.

## Retired spellings

- None — no delta retires a spelling. The re-phrased sentences in rules `sed_file` and `worktree_confinement` are quoted by no other surface, and `cases.tsv`'s comment line is re-phrased with the title.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for the set, the stream arm, rule `find_exec`, the admission and the program discriminator.
- [ ] **Instruction surfaces: instruction only** — the template clause and the agent-definition copy carry no grounds.
- [ ] **Merged with no information lost** — the ladder and its grounds land in guard-kit/SPEC.md as rule grounds, undated and unattributed (the provenance seam). Those are the grounds for blocking only the replaced forms, for admitting a cleared program, and for adding no allowlist entry.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the repo root (`ls SPEC-*.md`).
- [ ] **Removals propagated** — nothing retired.
- [ ] **Gaps filed** — a cross-component gap found during the work filed with `--emit file-gap`.
- [ ] **The entry moves** — `side-effect-free-read-arms` moves to Done in the landing commit, a stage before the drain stage.
