# SPEC amendment: word-read-heredoc

Pairs `guard-word-read-heredoc`. The bash word read answers nothing on a call carrying a heredoc (§The reader and its views). It checks the dequoted view first, and that view refuses every heredoc: the body is not in the `sq dq hd` skeleton the view walks beside, and the skeleton keeps a quoted delimiter verbatim where the view expects a mark. So on a heredoc-bearing call every rule reading a word by value falls back to the skeleton, and a quoted or escaped word there goes unread. This amendment has the read follow the skeleton's own heredoc scan, so it answers there too. `--guard-json words` answers with it.

**What the tree does today**, run at spec on the built binary with payloads fed to `--hook shell-guard` through the scratch runner:

| command | verdict | rule |
|---|---|---|
| `cat >> ".tmp/j.md" <<'EOF'` … | **block** | `emitter_write`, "don't write the quoted target" |
| `cat >> .tmp/j.md 2>"/dev/null" <<'EOF'` … | **block** | `emitter_write`: the quoted `/dev/null` is a file no rule can name, so the call is blocked, not only refused the append grant |
| `cat >> "README.md" <<'EOF'` … | block | `emitter_write`, its steer naming the quoted target instead of `README.md` |
| `rm 'README.md' <<'EOF'` …, `git 'stash' <<'EOF'` …, `\bash .tmp/x.sh <<'EOF'` … | **fall through** | `rm_tracked`, `git_stash` and `script_interpreter` read the skeleton's words |
| `cat >> .tmp/j.md <<'EOF'` … | allow | `append_scratch`, on the unquoted target |
| `--guard-json words` over any of these | prints nothing | |

**The reach, measured on a prototype.** The probe took each `cases.tsv` row carrying no `#`, `<<` or `@NL@` and ending in no `&`, `|`, `;` or backslash, and appended ` <<'EOF'@NL@x@NL@EOF`. That is 568 rows, run through `--run-guard-tests` as a cases table. Today 203 of them come out off their row's verdict, 93 of them a block or an advise lost to a fall-through. The prototype was built from a scratch worktree into its own target dir and is not in the tree. Its word read takes the skeleton scan's heredoc extents and keeps the dequoted check on heredoc-free calls. Under it:

- **On the derived table** the mismatches fall from 203 to 146. 57 rows return to their verdict and none newly mismatches. The rows that come back sit under rules `rm_tracked`, `git_stash`, `git_c_root`, `script_interpreter`, `shell_wrapper`, `commit_only_paths`, `scratch_redirect`, `sed_file`, `bare_sleep`, `pgrep_self_match`, `allowlist_chain`, `git_rewrite`, `append_scratch` and `emitter_write`.
- **On the authored suite** `--run-guard-tests` fails 2 of 13156 cases, the kit tables, the variant lane and the consumer lane together: `cases.tsv` lines 511 and 512. Those two rows pin the limit this amendment lifts, and each moves from block to allow. The background tables, derived the same way, move nothing.
- **What stays off its verdict** is 40 block-to-fall-through rows, in the sections of `cat_file`, `find_glob`, `find_exec`, `git_grep`, `brace_glyph`, `grant_path_slot`, `expansion`, `sed_file` (its `python3 -c` rows) and `emitter_write` (its by-value lead). Most carry no quote of their own, so their decline on a heredoc-bearing call is a read this one does not reach. They are filed as a gap, out of this unit.

**Rule `ro_pipeline` does not depend on the order of this entry and `ro-pipeline-wrong-grants`.** That sibling moves the rule's forms check onto word-read values. Whether it lands first or after, rule `ro_pipeline` grants no heredoc-bearing call: a non-empty body's segment and the terminator's segment each lead with no roster binary, and the rule needs every segment to. On the prototype the derived table's 37 `ro_pipeline` allow rows each declined or blocked, and none was granted. The set record orders the sibling first, and neither order moves a verdict there.

## What changes

### (1) The bash word read answers on a heredoc-bearing call {design-bearing} {user-facing: guard-word-read-heredoc's Deliverable, "the word read answered on heredoc calls (the body a mark word, the delimiter live verbatim), a reader change reaching every by-value rule"}

Replacement for `guard-kit/SPEC.md` §The reader and its views, the paragraph opening **It is built from the `raw`, `sq dq hd` and `dequoted` views**. Its last sentence, from **The quoting travels with the word**, stays as written. **Not yet applied.**

> It is built from the `raw`, `sq dq hd` and `dequoted` views and mints no view name: a rule calling it declares those three, and the context faults on the read as it faults on an undeclared view. **A heredoc is read as the skeleton holds it**, off the skeleton's own heredoc scan. Its opener, `<<` through the delimiter, and its terminator line are bare characters on all three faces. So a quoted delimiter keeps its quotes in its value, and no rule reads a delimiter, the opener being a redirect. A non-empty body is one word, alone in its segment, whose value and live text are the `hd` mark and whose spelling is the body's bytes. A body with no terminator line runs to the end of the command, as the skeleton's extent does. The read answers nothing where the dequoted view cannot be aligned on a call carrying no heredoc, and on a heredoc-bearing one wherever that view's refusal does not come from the heredoc: an unterminated quoted span, or one carrying a newline. Nor does it answer where its walk and the skeleton's compound split part, as on a backslash before `&&`. Each caller states what it does then. `native/src/guard/bash.rs`'s tests hold that a heredoc adds only its own words: for every bash decision-table row a heredoc can be appended to, the read of the row with one appended is the row's read plus the opener's words, the body's segment and the terminator's.

`native/src/guard/bash.rs`, `word_read`:

- **The dequoted check stays on a call carrying no heredoc**, so there the read answers exactly where it answers today.
- **On a heredoc-bearing call** the read takes the extents and bodies from `scan`, the function `heredoc_extents` and the `body` view already read, never a second copy of the opener grammar.
  - An opener `heredoc_header` matches is fed as one bare piece per byte, a blank ending a word, so `<< 'EOF'` is two words as the skeleton's `held_words` gives them.
  - At an extent's start a non-empty body is one piece, spelling the body's bytes without its last newline, with `HD_MARK` as its value and its live text, and its segment ends after it. The terminator line is fed as bare pieces.
  - A quoted span carrying a newline refuses the read, the check the dequoted view makes on a heredoc-free call.
  - The skeleton-alignment check at the end stands unchanged, and it is the read's last refusal.
- **Its `spec:` comment** and **the `Reader` trait's in `native/src/guard/reader.rs`** read "`None` where the dequoted view cannot be aligned outside a heredoc, or where the walk and the compound split of the skeleton part".

No rule's code changes. Every by-value read goes through `Ctx::word_read`, so each one reaches a heredoc-bearing call as it reaches any other call: the command read, the git read, the redirect pairs' values (`valued_targets`, `statement_targets`, `nulled_by_value`), rule `rm_tracked`'s operands and rule `scratch_redirect`'s targets.

`guard-kit/SPEC.md` §The rule roster, rule `emitter_write`, **not yet applied:** the sentence "On a heredoc-bearing call, where the word read answers nothing, a quoted target is the skeleton's and its steer names it the quoted target, never the placeholder's letters." reads "Where the word read answers nothing, a quoted target is the skeleton's, and its steer names it the quoted target, never the placeholder's letters."

Tests:

- **`native/src/guard/bash.rs`'s unit tests.** The `None` assertion on `"cat <<EOF\nx\nEOF"` becomes an assertion of its three faces: `cat` and `<<EOF`, then the body word `x` / `HD_MARK` / `HD_MARK`, then `EOF`. New cases:
  - a quoted delimiter and a spaced one, `<< "EOF"` being two words;
  - an empty body, which adds no body segment;
  - two heredocs on one line;
  - a body with no terminator;
  - a quoted span carrying a newline beside a heredoc, which answers nothing.
- **The invariant test** reads `guard-kit/guard-tests/cases.tsv` from `CARGO_MANIFEST_DIR`, as `guard_ps_variants.rs`'s test reads the tables. It takes each row the probe above took, `@ROOT@` and `@CR@` substituted, and appends three heredocs in turn, each through its own call: `<<'EOF'` with the body `x`, `<<EOF` with a body carrying quotes and a backslash, and `<< "EOF"` with an empty body. It asserts that each read equals the row's read with the opener's words appended to its last segment, then the body's segment where the body is non-empty, then `[EOF]`, compared face by face. It also asserts that the row's read answers, so a row the read refuses cannot pass vacuously. The prototype ran this test over 1704 commands and all 1704 were equal.
- **`native/src/guard/mod.rs`'s emitter-steer test**: `cat >> "README.md" <<'EOF'\nx\nEOF` blocks with a message naming `'README.md'` and carrying no placeholder.

Rows in `guard-kit/guard-tests/cases.tsv`:

- **`append_scratch`.** Lines 509 to 512 become:

  ```
  # a heredoc-bearing call's targets are read by value too, whatever its delimiter's quoting
  allow	cat >> ".tmp/j.md" <<'EOF'@NL@x@NL@EOF
  allow	cat >> .tmp/j.md 2>"/dev/null" <<'EOF'@NL@x@NL@EOF
  allow	cat >> ".tmp/j.md" << "EOF"@NL@x@NL@EOF
  allow	cat >> ".tmp/j.md" <<'E"F'@NL@x@NL@E"F
  ```

- **`emitter_write`.** Line 1038's comment reads "# a heredoc-bearing call's quoted target is read by its value too, and the steer names it". After line 1039, add `fallthrough	echo x 2>"/dev/null" <<'EOF'@NL@x@NL@EOF`.
- **`script_interpreter`'s carriage-return rows.** Line 975's comment, "the word read and, under a heredoc, the fallback scan both take bash's blank", reads "the word read takes bash's blank, and so does the skeleton's scan on a call the read refuses, a quoted newline say". The comment is stale once row 977 is read by the word read, and the fallback scan keeps no row of its own. After line 977, add two heredoc-free rows, whose verdicts were run on the built binary and which the amendment moves not at all: `fallthrough	bash <@CR@.tmp/x.sh; echo 'a@NL@b'` and `block	bash < .tmp/x.sh; echo 'a@NL@b'`.
- **One row per rule the read now reaches**, each in its rule's section, every verdict run on the prototype:
  - `rm_tracked`: `block	rm 'tracked.md' <<'EOF'@NL@x@NL@EOF`;
  - `git_stash`: `block	git 'stash' <<'EOF'@NL@x@NL@EOF`;
  - `git_c_root`: `block	'git' -c core.pager=cat log --oneline -3 <<'EOF'@NL@x@NL@EOF`;
  - `script_interpreter`: `block	\bash .tmp/x.sh <<'EOF'@NL@x@NL@EOF`;
  - `scratch_redirect`: `block	some-runner 2>'op.err' <<'EOF'@NL@x@NL@EOF`;
  - `sed_file`: `block	sed -'n' '1,60p' TASK-QUEUE.md <<'EOF'@NL@x@NL@EOF`;
  - `git_rewrite`: `advise	git 'commit' --amend -F - <<'EOF'@NL@msg@NL@EOF`, and `advise	git commit -F - <<'MSG'@NL@git stash; rm tracked.md@NL@MSG`, whose body stays a mark that no rule reads.

The variant lane derives none of these, since each carries `<<`.

`scripts/guard-rules.sh`, the comment above `payload="$(cat)"`: "(a heredoc-bearing command)" reads "(a quoted span carrying a newline, say)". The consumer lane's two heredoc rows stay green: under the prototype the rule's `words` path read each body as one mark word, blocking `--no-verify` on the opener line and leaving `git clean -x` in the body unread. **Not yet applied.**

### (2) `--guard-json words` prints a heredoc's read, every mark as its two letters {mechanical} {user-facing: guard-word-read-heredoc's Deliverable, "a reader change reaching … `--guard-json words`"}

`native/src/guard/mod.rs`, `json_words`: each value passes through `reader::unmark` before the two encodings, as `json_view` unmarks its view. A body word's value then prints as `HD`, never a NUL byte. **Not yet applied.**

Replacement for `guard-kit/SPEC.md` §Consumer rules, the **`words`** bullet: its sentence "It prints nothing wherever `view` does and wherever the word read answers nothing, a heredoc or a here-string among them." reads "It prints nothing wherever `view` does and wherever the word read answers nothing, a PowerShell here-string among them. A heredoc prints as the read holds it: its opener's words on their segment's line, its body as a line reading `HD`, and its terminator as a line, each mark as its two letters, as `view` prints them." **Not yet applied.**

`guard-kit/SPEC.md` §Testing, the `--guard-json` seam cases sentence: "it prints nothing over a heredoc-bearing command and a `Read` payload" reads "it prints a heredoc-bearing command's body as `HD` and nothing over a `Read` payload". **Not yet applied.**

Tests:

- `native/src/guard/mod.rs`, `words_print_the_word_read_s_values_per_segment`: the `git commit -F - <<'MSG'\nx\nMSG` assertion reads `Some` of `git\tcommit\t-F\t-\t<<'MSG'`, `HD`, `MSG`.
- `guard-kit/gate-tests/consumer-rules.test.sh`, the `words-heredoc` case: the same command prints those three lines.

### (3) The release record says the limit is lifted {mechanical}

`.workflow/release-declarations.md`, `## Behavior changes`. The three bullets are unreleased, so each is rewritten in place rather than contradicted by a new one. **Not yet applied.**

- **The `script_interpreter` bullet**: "where the command read yields no words for the call, a heredoc-bearing one say," reads "where the command read yields no words for the call, a quoted span carrying a newline say,". Its example stays, `bash <` and a carriage return beside a heredoc still falling through, since `cases.tsv` line 977 pins the verdict and the read now answers on that call.

- **The `--guard-json` `words` bullet**: "It prints nothing where the read does not answer, a heredoc-bearing command among them" reads "It prints nothing where the read does not answer, a PowerShell here-string among them; a heredoc's body prints as the line `HD`".
- **The bullet for rules `ro_pipeline`, `background_no_record`, `bounded_wait`, `append_scratch` and `emitter_write`** loses its "Declared limit:" sentence. In its place: "A heredoc-bearing call is read the same way, so `cat >> ".tmp/j.md" <<'EOF'` is granted, and a quoted or escaped word beside a heredoc now reaches every rule that reads words by value: `rm 'tracked.md' <<'EOF'` blocks as `rm 'tracked.md'` does. A decision-table row of yours pinning a fall-through there is re-derived."

## Producers and consumers

- **The heredoc read (delta 1).**
  - Producer: `bash::word_read`, called through `Ctx::word_read` on every rule declaring the three views and by `json_words`.
  - Consumers: every rule able to call it, the 28 `TABLE` rows in `native/src/guard/rules/mod.rs` declaring `Raw`, `SqDqHd` and `Dequoted` (`grep -n "Dequoted" native/src/guard/rules/mod.rs`). Those are rules `cd_compound`, `git_c_root`, `scratch_redirect`, `abs_script`, `brace_glyph`, `sed_file`, `find_glob`, `find_exec`, `cat_file`, `git_grep`, `pgrep_self_match`, `bare_sleep`, `git_mutation_under_producer`, `background_no_record`, `commit_only_paths`, `git_stash`, `append_scratch`, `ro_pipeline`, `bounded_wait`, `bounded_write`, `allowlist_chain`, `git_rewrite`, `rm_tracked`, `script_interpreter`, `grant_path_slot`, `emitter_write`, `shell_wrapper` and `worktree_confinement`. They reach it directly or through the helpers `git grep -n "ctx.word_read" -- native/src/guard/rules` finds: `passed_read`, `command_read`, `git_read`, `valued_targets`, `nulled_by_value`, `statement_targets` and `ro_forms_read`. The PowerShell reader's word read is unchanged.
  - Each field has a reader. A body word's spelling has none, since no steer names a body, and it is kept so that every word carries the spelling face the read defines. Its value and its live text are the mark that the segment readers already meet on the skeleton's body segment. An opener's faces are what the command read's redirect drop reads off the live text.
  - Roster-holding readers: the `TABLE` rows' declarations, which do not change, and `check-guard-registration`, which holds them to the roster. No declaration moves, so no roster row moves.
- **`json_words`' unmark (delta 2).** Its consumer is a consumer rule command reading stdout. In this repo that is `scripts/guard-rules.sh`, whose `words` path now takes heredoc-bearing calls.
- **Point 5: the narrowed corpus.** The read narrows the set of calls where it answers nothing, and every caller's fallback reads the skeleton there. Two kinds of reader red on that set:
  - The decision table and the consumer lane red on a verdict that moves. On the prototype that was cases.tsv lines 511 and 512, rewritten by delta 1.
  - The `words-heredoc` seam case reds on a non-empty print, and delta 2 rewrites it.

  No reader asserts a count or a floor over calls the read refuses.
- **Point 6:** no delta obliges the members of a corpus.

## Existing sections updated

The roster came from `grep -n "answers nothing\|heredoc-bearing\|a heredoc or a here-string" guard-kit/SPEC.md` over guard-kit/SPEC.md, `git grep -n "heredoc" -- guard-kit/guard-tests/cases.tsv guard-kit/gate-tests/consumer-rules.test.sh scripts/guard-rules.sh .workflow/release-declarations.md`, and the callers' probe above.

- `guard-kit/SPEC.md` §The reader and its views, the word read's construction paragraph; §The rule roster, rule `emitter_write` (delta 1).
- `guard-kit/SPEC.md` §Consumer rules, the `words` bullet; §Testing, the seam cases (delta 2).
- `native/src/guard/bash.rs`, `word_read` and its tests; `native/src/guard/reader.rs`, the `Reader::word_read` comment (delta 1).
- `native/src/guard/mod.rs`: `json_words` and its test (delta 2), and the emitter-steer test (delta 1).
- `guard-kit/guard-tests/cases.tsv` (delta 1).
- `guard-kit/gate-tests/consumer-rules.test.sh` (delta 2).
- `scripts/guard-rules.sh`, the comment (delta 1).
- `.workflow/release-declarations.md` (delta 3).
- `docs/guard-kit/SPEC.md`, the generated mirror (all deltas).

## Retired spellings

- None — no delta retires a name. The word read, its helpers and the `words` mode keep their names, and the sentences rewritten are prose, not spellings.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls guard-kit/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **The reach re-measured** — the derived heredoc table re-run against the built binary, the 57 rows back on their verdict and none newly off it. A difference is explained before the merge.
- [ ] **Suites green** — `bash gate-sdk/bin/build-native.sh`, the crate's unit tests, `--run-guard-tests` (kit, variant and consumer lanes), `--run-gate-tests guard-kit/gate-tests guard-kit/checks` and the battery.
- [ ] **Done move** — the entry moves to Done in the commit that merges this amendment, before the drain stage, after validate's review has read the verdict moves where the set's second-vendor review is granted.
