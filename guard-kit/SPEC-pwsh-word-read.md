# SPEC amendment: pwsh-word-read

Pairs `grant-pwsh-word-read`. Three reads still sit on the skeleton, and this amendment moves each onto the word read (§The reader and its views):

- **The bash grants' redirect targets.** A quoted `/dev/null` there is a placeholder, so it is a file written.
- **Rule `ro_pipeline`'s command word.** A quoted or escaped member there is no roster binary.
- **The PowerShell branches' command words, redirect targets and body paths.** A backtick or a quote there is unread.

The PowerShell word read already exists (§The reader and its views, the PowerShell reader's word read bullet), so no reader is added. The rules' PowerShell branches stop reading the skeleton where it is built.

**What the tree does today**, run at spec on the built binary with payloads fed to `--hook shell-guard` through the scratch runner:

| command | verdict | rule |
|---|---|---|
| `grep -rn foo src 2>/dev/null` | allow | `ro_pipeline` |
| `grep -rn foo src 2>"/dev/null"` | falls through | `ro_pipeline` declines |
| `until [ -f .tmp/x ]; do sleep 5; done 2>"/dev/null"` | falls through | `bounded_wait` declines |
| `'grep' -rn foo src \| head`, `"grep" foo f`, `\grep foo f` | fall through | `ro_pipeline` declines |
| the backgrounded `grep -rn foo src 2>"/dev/null"` | **block** | `background_no_record`: exemption (3) not met |
| `echo x 2>"/dev/null"`, `printf x 2>"/dev/null"` | **block** | `emitter_write`, "don't write 'DQ' through a redirect" |
| `cat >> ".tmp/j.md" <<'EOF'` … | **block** | `emitter_write`, the same false block on an ignored target |
| `echo x > "README.md"` | block | `emitter_write`, the right verdict naming `DQ` and not the file |
| `printf x >> .tmp/j.md 2>"/dev/null"`, `rm -rf .tmp/x 2>"/dev/null"` | allow | `bounded_write`, which already reads its targets by value |

`--run-guard-tests` was green at spec: 781 cases, 11884 derived rows, 10 limit rows. The nine PowerShell limit rows are lines 4, 5 and 7 to 13 of `guard-tests/variant-limits.tsv`. The lane reds a limit row under which nothing fails, so each is live.

**The `emitter_write` narrowing is in scope by a decision of 2026-10-18, relayed by the lead** (intent oracle, not an operator ruling). The rule steers a redirect to a **tracked** target (§The rule roster, *The write ladder*), and a quoted `/dev/null` is not one, so its block there is a defect against the stated rule. The helper that fixes `append_scratch` is the one `emitter_write` reads, so the two move together. Rows keep every redirect to a tracked target blocked, and the change lands before the validate stage's granted second-vendor review.

**Each lift lands in its own commit, with the limit rows it discharges deleted in that commit.** The lane reds a limit row that admits nothing and a failure no row admits, so a commit deleting too few or too many is red. Every delta widens a grant or narrows a block, so all of them land in a build the validate stage's second-vendor review reads.

## What changes

### (1) A redirect target is read by its value in the bash grants and in rule `emitter_write` {design-bearing} {user-facing: grant-pwsh-word-read's Deliverable, "rule `abs_script`'s by-value read … given to each grant", and the 2026-10-18 decision on `emitter_write` above}

Replacement for `guard-kit/SPEC.md` §The reader and its views, the paragraph opening **The redirect pairs**. Its last two sentences, from **The scan's blanks are bash's**, stay as written. **Not yet applied.**

> **The redirect pairs** answer each output redirect as its operator and target, and a rule reads what a call writes off them. `/dev/null` and an fd-dup are the **inert targets**, and any other is a file written. Under bash `>&` is an operator, glued to its word or apart. An fd number, with or without a closing `-`, or a bare `-` makes it an fd-dup (`2>& 1`, `>&-`). Any other word is a file, the write `> <file>` is (`>& out`, `>&2x`), so no rule grants or exempts the command as writing nothing. **A target is read by its value** where the word read answers and the skeleton's targets and the read's values pair one for one, in order, so `2>"/dev/null"` is the device and `> "README.md"` names `README.md`. Where they do not pair, the skeleton's target stands and a quoted one is a file no rule can name.

`native/src/guard/rules/`:

- `redirect_values` and `nulled_by_value` move from `grants.rs` to `mod.rs` as `pub(super)`. A new helper there, `valued_targets(ctx, c) -> Result<Vec<String>, Fault>`, pairs `redirect_targets` over the skeleton with `redirect_values` over `ctx.word_read(c)`. It returns the values where both answer and their counts agree, and otherwise the skeleton's targets unchanged.
- **Rule `ro_pipeline`** tests `valued_targets` for inertness in place of its `redirect_targets` loop.
- **Rule `background_no_record`'s exemption (3)** (`is_ro_background` in `liveness.rs`) does the same.
- **Rule `bounded_wait`'s arm (A)** reads the condition's and the tail's targets through `valued_targets`, called once over the call at the rule's entry. The per-segment `wait_redirects_inert` stays the reading for rule `wait_no_producer`, which declares no `dequoted` view.
- **Arm (B)** compares the launch's targets by value: the launch is already a `Cmd`, so `ctx.word_read` takes it. Its decline on a quote mark in a launch target holds only where that read answers nothing.
- **`emitter_targets`**, shared by rules `append_scratch` and `emitter_write`, reads each statement's targets by value. The statement's targets are the call's `valued_targets` taken in statement order. A target whose value is inert is skipped. Any other target is tested by its value with `git check-ignore`, and a value still carrying a quote character declines. So a quoted `/dev/null` is no write, a quoted ignored target is granted, and a quoted tracked target is blocked with its name in the steer, never the placeholder's letters.
- **Rule `append_scratch` declares `raw`, `hdq`, `sq dq hd` and `dequoted`.** The `TABLE` row gains `Raw` and `Dequoted`, which `Ctx::word_read` requires.

`guard-kit/SPEC.md` §The rule roster, **not yet applied**:

- **`append_scratch`:**
  - The declaration reads "Declares `raw`, `hdq`, `sq dq hd` and `dequoted`."
  - Clause (b)'s second sentence reads: "A target is read by its value (§The reader and its views, the redirect pairs), and `/dev/null` and an fd-dup (`&1`, `&2`) are exempt, the targets rule `ro_pipeline` already treats as no file."
  - Clause (d)'s "or a redirect target that survives normalization carrying a quote" reads "or a redirect target whose value still carries a quote".
- **`ro_pipeline`:** "and every redirect target is `/dev/null` or an fd-dup" gains ", each read by its value".
- **`background_no_record` exemption (3)** and **`bounded_wait` clause (c)** read the same, by citation of the redirect pairs.
- **`bounded_wait` arm (B2):** "on rule `append_scratch`'s exact `git check-ignore --quiet --` predicate and its inert-target carve-out" gains ", each target read by its value".
- **`emitter_write`**, after the sentence ending "which reads the skeleton's lead alone":

  > Its targets are read by their value, as rule `append_scratch` reads them, so a quoted `/dev/null` is no write and a quoted tracked target is arm (b)'s, named as its value.

- **§The reader and its views**, the paragraph **Rule `append_scratch` reads no raw command**, opens instead: "**Rule `append_scratch`'s decline reads no raw command**: its `raw` and `dequoted` views feed the word read of its targets alone." The rest of the paragraph stands.

Authored rows, in `guard-kit/guard-tests/cases.tsv` under each rule's section:

- `ro_pipeline`: allow `grep -rn foo src 2>"/dev/null"`, allow `grep -rn foo src 2>'/dev/null'`, fallthrough `grep -rn foo src 2>"out.txt"`.
- `bounded_wait`: allow `until [ -f .tmp/x ]; do sleep 5; done 2>"/dev/null"`.
- `append_scratch`:
  - allow `cat >> ".tmp/j.md" <<'EOF'@NL@x@NL@EOF`;
  - allow `cat >> .tmp/j.md 2>"/dev/null" <<'EOF'@NL@x@NL@EOF`.
- `emitter_write`:
  - fallthrough `echo x 2>"/dev/null"`;
  - block `echo x > "README.md"`;
  - block `cat >> "README.md" <<'EOF'@NL@x@NL@EOF`;
  - block `printf x >> 'README.md'`.

  Each block row's message names `README.md`, held by the crate test below.
- `background-cases.tsv`, `background_no_record`: allow `true` `grep -rn foo src 2>"/dev/null"`.

A unit test in `native/src/guard/rules/mod.rs` holds `valued_targets` on a bare, a quoted, an escaped and a split-apart `/dev/null`, on an fd-dup, and on a count mismatch, which falls back. A test in `grants.rs` holds that `emitter_write`'s steer for `echo x > "README.md"` names `README.md` and carries no placeholder.

### (2) Rule `ro_pipeline` reads its command word by value {design-bearing} {user-facing: grant-pwsh-word-read's Deliverable, "rule `ro_pipeline`'s command word read off the word read with authored rows as its oracle"}

`native/src/guard/rules/grants.rs` `ro_pipeline`:

- **Each pipe member's command word is its value.** It is the first word of the member's `passed_read` words, the members mapped to the compound split's segments by `pipe_members`, as `tool_members` in `tools.rs` maps them for the `sed` and `awk` readers.
- **The roster test compares the whole value.** It is never the last path component, so `'/bin/cat'` is no roster binary, as `/bin/cat` is none today.
- **The `xargs` test reads the same value.**
- **`passed_read`, never `command_read`.** The matcher's wrappers stay unread past, since the harness view is never a grant view (§The reader and its views).
- **A member whose skeleton head is a shell keyword declines**, so `! grep -q x f` keeps today's verdict.
- **Where the word read answers nothing or the two splits part**, the skeleton's head word is read, as today.

`guard-kit/SPEC.md` §The rule roster, **not yet applied**:

- `ro_pipeline`'s "Granted silently when every pipe segment leads with a roster binary" reads "Granted silently when every pipe segment's command word, read by its value off the word read and compared whole, is a roster binary".
- The list of refusals loses "a leftover quote after normalization" and gains "a command word the word read cannot give".
- **One reader**'s "so a quoted or escaped member (`"sort" -o <file>`) is read and can only withhold" reads "so a quoted or escaped member is read, granted where its forms clear and withheld where they do not (`"sort" -o <file>`)".
- §The generic ruleset's shared direction **A rule reads a word as its item says** gains rule `ro_pipeline` among the rules reading the command word by value, "through the word read, compared whole and not past a wrapper".

Authored rows, `cases.tsv` `ro_pipeline` section:

- allow `'grep' -rn foo src | head`;
- allow `"grep" foo f`;
- allow `\grep foo f`;
- allow `g'rep' foo f`;
- fallthrough `'/bin/cat' f`;
- fallthrough `'sort' -o out.txt f`;
- fallthrough `'xargs' rm`;
- fallthrough `! grep -q x f`.

The bash variant lane already derives `sq-lead`, `dq-lead`, `esc-lead` and `part-lead` for `ro_pipeline`'s allow rows, and its relation admits a grant, so the lift turns their fall-throughs to allow and reds nothing. No limit row names `ro_pipeline`.

**The safety argument (`'/bin/cat' f` stays blocked by rule `cat_file` ahead of the band):**

- A quoted command word suppresses alias expansion, so its value is the program run.
- The word read is aligned with the skeleton and answers nothing where it cannot be.
- `"so\rt"` has the value `so\rt`, not `sort`, and declines.
- The declared-forms reader runs on the same segment and can only withhold.

### (3) The PowerShell branches read command words, redirect targets and body paths by value {design-bearing} {user-facing: grant-pwsh-word-read's Deliverable, "the PowerShell reader's own word read, each lift deleting its limit rows"}

Each rule below already declares `raw`, `sq dq hd` and `dequoted`, so no `TABLE` row and no roster declaration changes. Where the word read answers nothing, each reads the skeleton as today. Each lift is its own commit and deletes its rows from `guard-kit/guard-tests/variant-limits.tsv`:

| rule | the read lifted (`native/src/guard/rules/`) | rows deleted |
|---|---|---|
| `cd_compound` | a segment's first word by value, case-insensitive (`spelling.rs`) | `cd_compound ps-tick-lead` |
| `rm_tracked` | the deletion command word by value before the `PS_DELETE` lookup (`reach.rs`); the operands already read by value | `rm_tracked ps-tick-lead` |
| `background_no_record` | `ps_detaches`' head word by value (`liveness.rs`) | `background_no_record ps-tick-lead` |
| `scratch_redirect` | the PowerShell target by value through `redirect_values`, with `\` folded to `/` before `scratch_name` (`spelling.rs`) | `scratch_redirect ps-dq-arg`, `ps-sq-arg`, `ps-tick-arg` |
| `script_interpreter` | `interpreter_reach`'s lifted tokens under PowerShell too, the two `!ps` guards dropped, with `powershell_form` taking those tokens rather than a skeleton segment (`reach.rs`) | `script_interpreter ps-dq-arg`, `ps-sq-arg`, `ps-tick-lead` |

**`redirect_values` takes the reader's escape character**: `\` under bash and the backtick under PowerShell, as `passed_words` already does. Its bash behaviour and its unit test are unchanged, and the test gains a PowerShell case for each of a bare, a quoted and a backticked target.

**A steer prints the spelling face**, as §The reader and its views' word read already states, so `rm_tracked`'s PowerShell corrective names the command as the segment spelled it.

**Honest limit, kept.** PowerShell's stop-parsing token `--%` is not modelled: the words after it are read with quoting PowerShell does not apply there. No rule this delta lifts reads a command word after it. Its arguments are read as the reader's other honest limits read an unmodelled construct.

`guard-kit/SPEC.md`, **not yet applied**:

- **§The generic ruleset's shared direction A rule reads a word as its item says:** its second sentence gains "Under PowerShell rules `cd_compound`, `rm_tracked`, `background_no_record`, `scratch_redirect` and `script_interpreter` read their command words, redirect targets and body paths through the word read." The sentence opening "Any other rule reads the skeleton's word" stands.
- **`cd_compound`:** "when one of them is the first word of a segment" reads "when one of them is the value of a segment's first word".
- **`scratch_redirect`:** the sentence "Under bash a target is read by its value too, off rule `abs_script`'s by-value redirect read …" loses "Under bash", and its example gains `` 2> op.e`rr `` under PowerShell.
- **`background_no_record`'s PowerShell reading, the backgrounding forms:** "a segment of the reader's compound split whose first word is" reads "whose first word's value is".
- **`rm_tracked`'s Under PowerShell bullet:** opens "the deletion commands, read by their value, are".
- **`script_interpreter`:** the paragraph **Under Bash each pipe member's words come through the command read** opens "**Under either reader** each pipe member's words come through the word read, past a wrapper under bash". It gains the PowerShell examples ``pw`sh -File .tmp/x.ps1`` and `& '.tmp/x.ps1'`.

### (4) The PowerShell re-spellings are parsed by a PowerShell host {design-bearing}

The entry's inferred claim, that a PowerShell host reads each derived `ps-` row as its base row's command, is settled by a host, not by this tree's own lexer.

- **A new arm, `--emit-guard-ps-variants`,** prints every derived PowerShell row of the kit's two PowerShell tables as `<transform> <TAB> <base> <TAB> <variant>`, one per line, from the same `guard_ps_variants::derive` the lane calls. It reads the tables' default paths, exits 0, and exits 2 on an unreadable table. It is an `ARMS` row in `native/src/emit/mod.rs` with an `Arm::Run` member, its `spec:` binding naming guard-kit/SPEC.md §Testing.
- **A step on `crate-tests-windows`' x64 leg**, `guard PowerShell re-spellings under pwsh's parser`, runs the arm. A `pwsh` script inline in the step parses each base and variant with `[System.Management.Automation.Language.Parser]::ParseInput`. It reds the step on any line where the two parse with errors, or where their `CommandAst` element lists differ in count, kind or static value. A `StringConstantExpressionAst` compares by `.Value`, and a redirection by operator and by its target's static value. It prints the number of lines compared, and reds on zero.

**Inferred, cannot run before build:** pwsh reads each derived PowerShell row as its base row's command — no PowerShell host is on the build machine, and the step that runs one lands with this delta and runs at the entry's mid-iteration push.

The step's first green run is the evidence, and the build that reads it deletes the marker above.

`guard-kit/SPEC.md` §Testing, the PowerShell deriver's **Its honest limits**: "No shell runs a derived row, so the equivalence is the construction's" reads "No shell runs a derived bash row, so its equivalence is the construction's; a PowerShell host parses each derived PowerShell row against its base on the x64 Windows leg (`--emit-guard-ps-variants`)". **Not yet applied.**

### (5) The release record names the verdict moves {mechanical}

`.workflow/release-declarations.md`, `## Behavior changes`, one bullet per delta that lands. **Not yet applied.**

- **Delta 1:** a quoted `/dev/null` or a quoted gitignored target no longer falls through or blocks in rules `ro_pipeline`, `background_no_record`, `bounded_wait`, `append_scratch` and `emitter_write`, and `emitter_write`'s steer names a quoted tracked target by its value.
- **Delta 2:** rule `ro_pipeline` grants a quoted or escaped roster binary.
- **Delta 3:** five rules read PowerShell command words, targets and body paths by value, so a backtick or a quote there no longer evades rules `rm_tracked` and `script_interpreter`.

## Producers and consumers

- **`valued_targets` (delta 1).**
  - Producer: rule code at a grant's or `emitter_targets`' entry, once per call, from `ctx.word_read` and the skeleton.
  - Consumers: rules `ro_pipeline`, `background_no_record`, `bounded_wait`, `append_scratch` and `emitter_write`.
  - Readers holding a roster: the `TABLE` row's declarations, which `Ctx::word_read` checks at run time. `check-guard-registration` assertion C holds each row equal to its SPEC roster item's "Declares" clause, so delta 1's `append_scratch` declaration edit lands in both in one commit.
- **`redirect_values`' escape parameter (delta 3).** Its callers, found with `git grep -n "redirect_values" -- native/src` at spec:
  - `grants.rs` (`nulled_by_value` and its test);
  - `spelling.rs:167` (`scratch_redirect`).

  Each passes the running reader's escape, which `passed_words` already takes.
- **`--emit-guard-ps-variants` (delta 4).**
  - Producer: the arm, run by the new workflow step only.
  - Consumer: the step's `pwsh` script, by a pipe.
  - Each field has a reader:
    - `<transform>` is printed on a mismatch, so the reader can tell which deriver to fix;
    - `<base>` and `<variant>` are parsed and compared.
  - Roster-holding readers:
    - `ARMS`, the arm table, which the binary's no-subcommand roster prints;
    - the crate test holding every `ARMS` row to a `spec:` binding;
    - `check-crate-arms`' clippy and test arms.

  This repo has no projection of the arm roster, by `git grep -n "emit-queue-edges" -- docs scripts`: the hits are prose mentions and no rostered list.
- **Point 5, a narrowed corpus.**
  - Delta 3 deletes nine rows of `variant-limits.tsv`. Its reader, `--run-guard-tests`' variant lane, reds a failure no limit row admits (`Ledger`). Its red condition is a `ps-` re-spelling that still fails after the commit claiming to discharge it, so each lift commit runs the lane before it lands.
  - Delta 1 narrows rule `emitter_write`'s block set. Its readers are the decision table and the variant lane's write-added transforms, whose relation admits a narrowed block only where an authored row asserts it. The four `emitter_write` rows above are that assertion.
- **Point 6, every member's value.** The nine limit rows are enumerated by `grep -n "ps-" guard-kit/guard-tests/variant-limits.tsv`, and delta 3's table names each one's discharging rule. Line 6, `fallthrough pgrep_self_match part-arg`, is a bash row and stays.

## Existing sections updated

The roster came from the guard-kit SPEC passages the survey's source read cited, re-read at spec. That is `grep -n "redirect pairs\|reads no raw\|leads with a roster\|exemption (3)\|first word is\|Under Bash each pipe member\|No shell runs a derived row" guard-kit/SPEC.md` over guard-kit/SPEC.md, plus `git grep -n "redirect_values\|redirect_targets\|inert_target" -- native/src/guard` over the crate.

- `guard-kit/SPEC.md` §The reader and its views: the redirect-pairs paragraph and the `append_scratch` raw paragraph (delta 1).
- `guard-kit/SPEC.md` §The generic ruleset's shared directions (deltas 2 and 3).
- `guard-kit/SPEC.md` §The rule roster: items `append_scratch`, `ro_pipeline`, `background_no_record`, `bounded_wait`, `emitter_write` (delta 1); `ro_pipeline` (delta 2); `cd_compound`, `scratch_redirect`, `background_no_record`, `rm_tracked`, `script_interpreter` (delta 3).
- `guard-kit/SPEC.md` §Testing, the PowerShell deriver's honest limits (delta 4).
- `native/src/guard/rules/mod.rs`: the `TABLE` row for `append_scratch`, `valued_targets`, and the moved `redirect_values` and `nulled_by_value` (deltas 1 and 3).
- `native/src/guard/rules/grants.rs`: `emitter_targets`, `append_scratch`, `ro_pipeline` (deltas 1 and 2).
- `native/src/guard/rules/liveness.rs`: `is_ro_background`, `bounded_wait`, `ps_detaches` (deltas 1 and 3).
- `native/src/guard/rules/spelling.rs`: `cd_compound`, `scratch_redirect` (delta 3).
- `native/src/guard/rules/reach.rs`: `rm_tracked`, `interpreter_reach`, `powershell_form` (delta 3).
- `native/src/emit/mod.rs` and a new `native/src/emit/guard_ps_variants_arm.rs`, or the arm's body beside `run_guard_tests.rs` (delta 4).
- `.github/workflows/gates.yml`, `crate-tests-windows` (delta 4).
- `guard-kit/guard-tests/cases.tsv` and `background-cases.tsv` (deltas 1 and 2); `variant-limits.tsv` (delta 3).
- `.workflow/release-declarations.md` (delta 5).
- `docs/guard-kit/SPEC.md`, the generated mirror (all deltas).

## Retired spellings

- None — no delta retires a name. The helpers that move keep their names, and the limit rows deleted are data, not spellings.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls guard-kit/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Suites green per lift commit** — `bash gate-sdk/bin/build-native.sh`, the crate's unit tests, `--run-guard-tests`, `--run-gate-tests guard-kit/gate-tests guard-kit/checks` and the battery, each lift in its own commit with its limit rows deleted there.
- [ ] **The Windows leg read** — the mid-iteration push after delta 3 lands, its `crate-tests-windows` x64 run watched green, the new step's compared count read and the inferred marker of delta 4 deleted.
- [ ] **Done move** — the entry moves to Done in the commit that merges this amendment, which is no later than the stage before the drain stage and after the Windows leg's run is read.
