# SPEC amendment: dollar-quote-read

Pairs `word-read-dollar-quote-grants`. Two hotfixes made the bash word read answer nothing on a call carrying an ANSI-C quoted word (`$'…'`) or a locale-translated one (`$"…"`), closing the wrong grants that taking the `$` as a literal had opened (§The reader and its views). Every rule reading a word by value then falls back to the skeleton on such a call. So the read-only calls rule `ro_pipeline` granted by value fall through to a prompt, and a block keyed on a value, such as rule `rm_tracked`'s or rule `git_stash`'s, misses its spelling. This amendment has the read resolve an ANSI-C word as bash does, keeps the locale-translated refusal declared, and widens `--guard-json words`' encoding to the newline a resolved value can now carry.

**What the tree does today**, run at spec on the built binary with payloads fed to `--hook shell-guard` through the scratch runner (`TASK-QUEUE.md` standing for a tracked file):

| command | verdict |
|---|---|
| `grep $'a\tb' TASK-QUEUE.md`, `head -5 $'README.md'`, `ls $'docs'`, `echo $'a\nb' \| wc -l` | fall through, where the single-quoted spellings allow |
| `rm $'TASK-QUEUE.md'`, `git $'stash'` | fall through, where `rm 'TASK-QUEUE.md'` and `git 'stash'` block |
| `printf x > .tmp/$'x'`, `cat >> .tmp/$'x'` | **block**, `emitter_write`'s "don't write the quoted target" |
| `cat >> ".tmp/"$'\056\056/README.md'` | block, the steer naming "the quoted target" |
| `grep $"a" TASK-QUEUE.md` | fall through |
| `--guard-json words` over any of these | prints nothing |

**What bash does**, probed on bash 5.3 under `LC_ALL=C` and `C.UTF-8`, each spelling's bytes read through `od`:

- The C escapes resolve: `\t`, `\n` and `\r`, the controls `\a`, `\b`, `\e`, `\E`, `\f` and `\v`, and `\\`, `\'`, `\"` and `\?` as themselves.
- `\` then one to three octal digits is that byte modulo 256 (`\101` is `A`, `\0101` is a backspace then `1`, `\777` is the byte `ff`). `\x` takes one or two hex digits and gives a byte, and `\x`, `\u` or `\U` with no digit stays as written.
- `\u` takes up to four hex digits and `\U` up to eight. Below `80` the character is ASCII whatever the locale. At or above it bash encodes the character in the locale's charset, so under `C` the escape stays as written, its hex upper-cased, and under `C.UTF-8` `\U000000e9` is the two bytes `c3 a9`.
- `\c` then a character is that character's control, its case ignored, so `\ca` is `\cA`, the byte `01`; `\c?` is `7f`, `\c\\` takes both backslashes, and a closing `\c` stays as written.
- An escape bash does not know keeps its backslash: `\q` is `\q`, `\8` is `\8`.
- A resolved NUL ends the span: `$'a\0b'c` is `ac`, as with `\x00`, the four-digit `\u` NUL and `\c@`.
- `\'` is an escaped quote, so `$'it\'s'` is `it's`. The skeleton closes its single-quoted span at that quote, a skeleton limit out of this unit and filed as a gap.

**The reach, measured on a prototype** built from a scratch worktree into its own target dir and not in the tree. Its word read resolves an ANSI-C span as delta 1 states, and its `held_values` and `json_words` change as deltas 1 and 2 state.

- **On the authored suite** `--run-guard-tests` fails 1 of 13490 cases, the kit tables, the variant lane and the consumer lane together: `printf x > .tmp/$'x'`, which pins the refusal this amendment lifts and moves from block to allow. The crate's guard tests fail one test, the refusal test delta 1 rewrites. The heredoc invariant passes with ANSI-C rows taken in.
- **On a derived table** of 116 `cases.tsv` rows, each with every single-quoted span re-spelled `$'…'`, taking the rows with no backslash, double quote, `$`, `#`, heredoc or `@NL@`: 54 come out off their row's verdict on the tree binary (34 block to fall through, 18 allow to fall through, one allow to block and one rewrite to block), and 2 on the prototype. Those two are `find .tmp/old -name $'*.txt' -delete`, allow to fall through, and an absolute repo script with `2>$'/dev/null'`, rewrite to block. Rule `grant_path_slot`'s test and rule `bounded_write` decline on any `$` live in the `sq hdq` view, and the `$` of an ANSI-C word is live there. Each move is the conservative direction, and the pair is filed as a gap, out of this unit.
- **On the value face**, over the 111 of those rows carrying no `$` of their own, the read of each re-spelled row equals the row's read word for word.
- **The 25 decision-table rows delta 1 keeps, moves or adds** are each green on the prototype, and 8 of them fail on the tree binary: the moved row, and the five grants and two blocks it restores.

**The two refusal sets are the lead's decision (2026-10-19, lead decision from the governed surfaces, not an operator direction and not a ruling).** The locale-translated refusal is kept, and the ANSI-C read refuses the three outcomes delta 1 names. Grounds: the release bar in `tarball-attestation-observed` admits a fall-through as a declared limit and no wrong grant, and every refusal here answers nothing, so its call falls through or blocks and is never granted on a value the read did not give. **Reopen condition:** a refused spelling found to grant rather than fall through is outside this envelope, and is escalated, never landed.

**No act here depends on a sibling's landing.** `heredoc-call-rule-decline` lands first, by the set record's order, and it changes per-rule operator tests, not the word read, so it moves none of the rows or tests below. Rows are named by their text, never their line, since that entry adds rows to the same table.

## What changes

### (1) The bash word read resolves an ANSI-C quoted word {design-bearing} {user-facing: word-read-dollar-quote-grants' Deliverable, "ANSI-C escapes (octal, hex, `\u`, `\c`, the C escapes) resolved into the value face, restoring those grants"; the two refusal sets, the lead's decision of 2026-10-19 above}

Replacement for `guard-kit/SPEC.md` §The reader and its views, the paragraph opening **It is built from the `raw`, `sq dq hd` and `dequoted` views**. Its last sentence, from **The quoting travels with the word**, stays as written. **Not yet applied.**

> It is built from the `raw`, `sq dq hd` and `dequoted` views and mints no view name: a rule calling it declares those three, and the context faults on the read as it faults on an undeclared view. The read answers nothing where the dequoted view cannot be aligned on a call carrying no heredoc, and on a heredoc-bearing one wherever that view's refusal does not come from the heredoc: an unterminated quoted span, or one carrying a newline. Nor does it answer where its walk and the skeleton's compound split part, as on a backslash before `&&`. Each caller states what it does then.
>
> **An ANSI-C quoted word is read as bash resolves it.** A single-quoted span behind an odd run of bare `$`, the first unescaped, is one piece: `$$` is the shell's pid, so an even run is a literal. Its spelling is the span with its `$`, and its live text is that `$` and the `sq` mark, as the skeleton holds them. Its value is the span's content with each escape resolved: the C escapes, one to three octal digits modulo 256, `\x` with one or two hex digits, `\u` and `\U` with up to four and eight, and `\c` with the character it controls, `\c?` being DEL. An escape bash does not know keeps its backslash, as does `\x`, `\u` or `\U` with no digit.
>
> **The read answers nothing on an ANSI-C span whose value it cannot give exactly**:
>
> - where an escape resolves to a C0 control other than tab, line feed and carriage return, NUL included. A command cannot carry one raw (§The shell guard), and in a value it would read as a mark or as one of the dequoted view's sentinels;
> - where an escape resolves above ASCII. Bash encodes a `\u` or `\U` there in the locale's charset, which the guard cannot see, and an octal or hex escape there is a byte that is no character;
> - where the span's content ends in an odd run of backslashes. Bash reads that quote as escaped and runs the span on, where the skeleton closes it.
>
> **Each refusal is a declared limit, and it costs one permission decision**: a read-only call so spelled (`grep $'\e[1m' f`) reaches the harness's decision rather than a grant, and a write so targeted blocks as a write to a quoted target. A rule whose verdict reads no refused word still decides, so `printf $"x" >> .tmp/j.md` is granted on its unquoted target.
>
> **A locale-translated word is not read.** On a double-quoted span behind an odd run of bare `$` the read answers nothing. Bash translates the span through a message catalog named by `TEXTDOMAIN`, `TEXTDOMAINDIR` and the locale, which the guard cannot see and an earlier line of the same call can set. Taking the span's double-quoted value would hand a grant a value the catalog may replace, and taking the `$` as a literal hands it the wrong one: `".tmp/"$"../x"` is `.tmp/../x` where no catalog translates it. It is a declared limit at the same cost, one permission decision: `grep $"a" f` reaches the harness's decision.

The same section's heredoc paragraph: "carrying no ANSI-C quoted or locale-translated word" reads "carrying no locale-translated word, nor an ANSI-C quoted one the read refuses". **Not yet applied.**

`native/src/guard/bash.rs`, `word_read`:

- **A `$` opening an ANSI-C span**, the last of an odd run as `dollar_quote` reads it, takes the span through its closing quote as one piece: the bytes from the `$` through that quote as its spelling, `$` and `SQ_MARK` as its live text, and the resolved content as its value. A raw newline in the span refuses the read, as a quoted span carrying one does.
- **The resolver is a function of its own**, taking the span's content and answering its value, or `None` on each refusal above. Its `spec:` comment names this section. The single-quote arm's `dollar_quote` refusal then serves no caller, so it is removed. The double-quote arm's stays.
- **`dollar_quote`'s `spec:` comment** reads "a `$`-quoted word: a quote behind an odd run of bare `$`, less an escaped first `$`, since `$$` is the shell's pid".

`native/src/guard/rules/tools.rs`, `held_values`: a newline in a value is held as the blank's sentinel `01`, as a blank is (a tab takes `02`), so the declared-forms reader takes a value carrying one as one word. `past_wrappers` already holds it so. **Not yet applied.**

The value readers were surveyed with `git grep -n "\.value" -- native/src/guard/rules`. Most of them compare a value whole. Four re-join or re-split values:

- `past_wrappers`, which holds a blank, a tab and a newline;
- `held_values`, which this delta extends;
- `segment_tool` in `native/src/guard/rules/tools.rs`, which passes each value through `unsentinel` in `native/src/guard/text.rs`, mapping the bytes `01` to `05` back to a blank, a tab, `;`, `|` and `&`. No value carrying one reaches it once the read refuses C0 controls;
- `allowlist_chain`'s space join, which compares against the allow patterns the harness matches on the raw command.

Tests in `native/src/guard/bash.rs`:

- **`the_word_read_answers_nothing_on_a_dollar_quoted_word`** keeps its `$"…"` assertions and its `\$'x'`, `$$'x'` and `"$"'x'` faces. Its ANSI-C `None` assertions become faces: `".tmp/"$'\056\056/README.md'` has the value `.tmp/../README.md` and the live text `DQ_MARK` then `$` and `SQ_MARK`, beside a heredoc too. `$$$'x'` has the value `$$x`, the pid's `$$` read as written, `\$$'x'` has `$x`, and `\\$'x'` has `\x`.
- **A resolver table test** holds each escape to the bytes the bash probe above gave, one assertion per row:
  - each C escape, `\'` included;
  - octal at one, two and three digits, `\0101` refusing as a backspace, and `\777` refusing as the byte `ff`;
  - `\x41`, the one-digit `\x4` refusing as a control, `\x414` as `A4`, and `\xg` as written;
  - `\u` with one and four digits below `80`, the eight-digit `\U000000e9` refusing, and `\u41g` as `Ag`;
  - `\cA`, `\ca`, `\cI` as tab, `\c?` as DEL, and `\c\\x` refusing as the control `1c`, its two backslashes taken;
  - `\q`, `\8`, a closing `\c`, and a bare `\x`, `\u` and `\U`, each as written;
  - `\0`, `\x00` and `\c@` refusing;
  - a content ending in one backslash refusing, and in two read as one `\`.
- **The ANSI-C invariant test** reads `guard-kit/guard-tests/cases.tsv` from `CARGO_MANIFEST_DIR`, as the heredoc invariant test does. It takes each row carrying a single-quoted span and no backslash, double quote, `$`, `#`, `<<` or `@NL@`, with balanced quotes, `@ROOT@` and `@CR@` substituted. It puts a `$` ahead of each span's opening quote, and asserts that the row's read answers and that the re-spelled row's read equals it on the value face, word for word. The prototype ran it over 111 rows and all 111 were equal.
- **The heredoc invariant test** stops skipping a row for carrying `$'`. It skips a row carrying `$"`, and a row carrying `$'` whose own read answers nothing, so the refusal rows below are not taken as rows the read must answer.

Rows in `guard-kit/guard-tests/cases.tsv`, each run on the prototype:

- **`append_scratch`.** The comment opening "an ANSI-C quoted word leaves the word read answering nothing" reads "an ANSI-C quoted word is read by its resolved value, so a target spelling one names the file it resolves to (`$'\056\056'` is `..`, out of the scratch dir) and rule `emitter_write` blocks, beside a heredoc too; a span the read cannot resolve exactly leaves the target unread". Under it:
  - `block	cat >> ".tmp/"$'\056\056/tracked.md'` and its heredoc-bearing twin stay;
  - `block	printf x > .tmp/$'x'` becomes `allow	printf x > .tmp/$'x'`;
  - `allow	printf $'x\n' >> .tmp/j.md` stays;
  - add `block	printf x > .tmp/$'\e'`.

  The `$"…"` comment and its four rows stay as written.
- **`ro_pipeline`.** The comment opening "an ANSI-C quoted or locale-translated word is unread" reads "an ANSI-C quoted word is read by its value, so a write form spelled with one withholds the grant as the plain spelling does, and a read-only call so spelled is granted; a locale-translated word is unread, so a write form spelled with one withholds too". Its rows `fallthrough	sort $'-o' tracked.md scratch.txt` and `fallthrough	sort $"-o" tracked.md scratch.txt` stay. Add:

  ```
  fallthrough	sort $'-\157' tracked.md scratch.txt
  allow	grep $'a\tb' tracked.md
  allow	head -5 $'tracked.md'
  allow	echo $'a\nb' | wc -l
  allow	sort $'a\n-o' tracked.md
  allow	grep $'\q' tracked.md
  # a span the read cannot resolve exactly leaves the call unread: a control, a byte above ASCII,
  # a NUL, an escaped quote
  fallthrough	grep $'\cA' tracked.md
  fallthrough	grep $'\U000000e9' tracked.md
  fallthrough	grep $'\x80' tracked.md
  fallthrough	grep $'a\0b' tracked.md
  fallthrough	grep $'it\'s' tracked.md
  fallthrough	grep $"a" tracked.md
  ```

- **`rm_tracked`**: `block	rm $'tracked.md'`.
- **`git_stash`**: `block	git $'st\x61sh'`.

The variant lane derives none of these, since each carries a `$`.

### (2) `--guard-json words` encodes a newline in a value {mechanical} {user-facing: word-read-dollar-quote-grants' Deliverable, "ANSI-C escapes … resolved into the value face"}

`native/src/guard/mod.rs`, `json_words`: a newline in a value prints as `\n`, after the backslash's and the tab's encodings, so a segment stays one line. **Not yet applied.**

`guard-kit/SPEC.md` §Consumer rules, the **`words`** bullet: "a backslash and a tab in a value print as `\\` and `\t`, the fall-through line's encoding (§The shell guard)" reads "a backslash, a tab and a newline in a value print as `\\`, `\t` and `\n`, the fall-through line's encoding (§The shell guard)". **Not yet applied.**

`guard-kit/SPEC.md` §Testing, the `--guard-json` seam cases: "a quoted tab encoded" reads "a quoted tab and an ANSI-C newline encoded, an ANSI-C escape resolved". **Not yet applied.**

Tests:

- `native/src/guard/mod.rs`, `words_print_the_word_read_s_values_per_segment`: `echo $'a\nb'` prints `echo`, a tab, then `a\nb`.
- `guard-kit/gate-tests/consumer-rules.test.sh`, beside `words-tab`: a `words-newline` case for `echo $'a\nb'`, and a `words-ansi-c` case printing `git $'st\x61sh'` as `git`, a tab, then `stash`.

### (3) The release record says the ANSI-C refusal is lifted {mechanical}

`.workflow/release-declarations.md`, `## Behavior changes`, the unreleased bullet for rules `ro_pipeline`, `background_no_record`, `bounded_wait`, `append_scratch` and `emitter_write`. It is rewritten in place rather than contradicted by a new one. **Not yet applied.**

- Its sentences from "A call carrying an ANSI-C quoted word" through "A read-only call so spelled (`grep $'a\tb' f`, `grep $"a" f`) reaches the harness's decision." become: "An ANSI-C quoted word (`$'…'`) is read by the value bash resolves, so `cat >> ".tmp/"$'\056\056/README.md'` blocks as the write to `README.md` it is, and a read-only call so spelled (`grep $'a\tb' f`) is granted. A span whose value hangs on a control byte, the locale or an escaped quote is not read. A locale-translated word (`$"…"`) is not read either, since its catalog is out of the guard's sight, so `cat >> ".tmp/"$"../README.md"` blocks, `sort $"-o" README.md x` is not granted, and `grep $"a" f` reaches the harness's decision."
- The unreleased `--guard-json` bullet, its `words` mode's: after "a heredoc's body prints as the line `HD`", it reads "and a newline in a value prints as `\n`".

## Producers and consumers

- **The ANSI-C read (delta 1).**
  - Producer: `bash::word_read`, called through `Ctx::word_read` on every rule declaring `Raw`, `SqDqHd` and `Dequoted`, and by `json_words`.
  - Consumers: the `TABLE` rows in `native/src/guard/rules/mod.rs` declaring the three views (`grep -n "Dequoted" native/src/guard/rules/mod.rs`), directly or through the helpers `git grep -n "ctx.word_read" -- native/src/guard/rules` finds. No declaration moves, so `check-guard-registration`'s roster does not move.
  - Each face has a reader. The value is what every by-value rule compares. The spelling is what a steer prints, as rule `rm_tracked`'s `git rm -q $'tracked.md'` does. The live text, `$` then the `sq` mark, is what the alignment check compares against the skeleton's held word and what the command read's redirect drop reads.
  - New value states: a value may now carry a newline, DEL, or a C escape's printable result. The re-joining readers are rostered under delta 1, and `held_values` is the one that did not already hold a newline.
- **`json_words`' newline encoding (delta 2).** Its consumer is a consumer rule command reading stdout, in this repo `scripts/guard-rules.sh`, which splits lines on a newline and words on a tab, so an encoded newline keeps a segment on its line.
- **Point 5: the narrowed corpus.** The read narrows the set of calls where it answers nothing. Two kinds of reader red on that set:
  - The decision table and the consumer lane red on a verdict that moves. On the prototype that was the row `printf x > .tmp/$'x'`, rewritten by delta 1.
  - The heredoc invariant test reds on a row it skips that the read answers, and on a row it takes that the read refuses. Delta 1 rewrites its filter.

  No reader asserts a count or a floor over calls the read refuses.
- **Point 6:** no delta obliges the members of a corpus.

## Existing sections updated

The roster came from `git grep -n "ANSI-C\|locale-translated\|dollar_quote" -- ':!TASK-QUEUE.md'` over the tracked tree, `grep -n "words" scripts/guard-rules.sh guard-kit/gate-tests/consumer-rules.test.sh`, `grep -n "guard-json" guard-kit/SPEC.md`, and the value readers' probe above.

- `guard-kit/SPEC.md` §The reader and its views, the word read's construction paragraph and the heredoc paragraph (delta 1).
- `guard-kit/SPEC.md` §Consumer rules, the `words` bullet; §Testing, the seam cases (delta 2).
- `native/src/guard/bash.rs`: `word_read`, the resolver, `dollar_quote`'s comment and the tests (delta 1).
- `native/src/guard/rules/tools.rs`, `held_values` (delta 1).
- `native/src/guard/mod.rs`, `json_words` and its test (delta 2).
- `guard-kit/guard-tests/cases.tsv` (delta 1).
- `guard-kit/gate-tests/consumer-rules.test.sh` (delta 2).
- `.workflow/release-declarations.md` (delta 3).
- `docs/guard-kit/SPEC.md`, the generated mirror (all deltas).

## Retired spellings

- None — no delta retires a name. The word read, `dollar_quote`, `held_values` and the `words` mode keep their names, and the sentences rewritten are prose, not spellings.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls guard-kit/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **The reach re-measured** — the derived ANSI-C table re-run against the built binary: 114 of its 116 rows on their verdict, and the two named above off it in the conservative direction. A difference is explained before the merge.
- [ ] **Suites green** — `bash gate-sdk/bin/build-native.sh`, the crate's unit tests, `--run-guard-tests` (kit, variant and consumer lanes), `--run-gate-tests guard-kit/gate-tests guard-kit/checks` and the battery.
- [ ] **Done move** — the entry moves to Done in the commit that merges this amendment, before the drain stage, after validate's review has read the verdict moves where the set's second-vendor review is granted.
