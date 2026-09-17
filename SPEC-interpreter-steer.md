# SPEC amendment: interpreter-steer

Queue entries: `inline-interpreter-substrate-census` and `scan-prompts-heredoc-grant-split`, the
unit set of `interpreter-steer-census`. **Deltas 1 to 3 serve the second entry and build first**:
delta 4's census figures are read off the ranker that delta 3 corrects.

A root-level amendment, because it spans two components:

- guard-kit owns the friction log, the ruleset, §scan-prompts and the shell holders of the twinned
  primitives.
- `native/` holds the compiled twins and the ranker.

**What the census found, and why it names no new tool.** The census classified the inline interpreter
bodies in a week of this project's session transcripts (the survey record's
2026-09-17 spec block carries the corpus, filters and witness). Every inline body was `python3`: 821
heredocs and 50 `-c` calls, with no inline `node`, `ruby` or `perl -e`. The cluster that recurs
across sessions is one idiom: read a file, assert a literal occurs once, replace it with a literal,
write the file back (280 calls over 26 sessions, plus 157 heading-anchored splices of the same shape).
Its predictable target already exists. `--rewrite` replaces a literal across named files, and
`--expect` is the count assertion. No rule sends python there, because rule 8's in-place arms read
`sed` and `perl` only. The rest has no predictable tool to be sent to: computed replacement text
(f-strings, index splices, per-file pairs), and tallies and probes that only read. So the census's
answer is **one steer to an existing arm**, not a new arm. The unported remainder stays measured
friction, and the ranker has to measure it correctly first.

**What the probe found about the harness.** The heredoc entry was design-pending because either
reading of a heredoc was a guess. It is now measured. The harness's documented separator list includes
newlines, and its documentation says a here-doc is a redirect target that is not checked. A direct
probe ran a headless harness session under a lone `Bash(python3 -*)` grant in the mode that denies
anything unmatched. The results:

- **Ran:** a multi-line heredoc whose body carries `;` and `|`, and a body line that reads like an
  ungranted command.
- **Denied:** an ungranted command on the line after the terminator, a pipe on the opener line into an
  ungranted command, an unquoted-delimiter body carrying `$HOME`, and a bare `perl -e` control.

So a heredoc body is data. Text after its terminator is shell. An unquoted body is still live for
expansion. The ranker today splits the body on `;` and `|` and reads the result as prompting, and the
owed-port-tail report that multi-line heredocs prompt despite the grant did not reproduce. The fix
needs the line structure that the friction log currently flattens away.

## The seam

- **Kit mechanism:** the log encoding, the twins' heredoc arm, the ranker's heredoc reading and rule
  8's python arm. None of them names a project, a path or a vocabulary.
- **Kit literals about the harness and the language, minting no knob:** the harness's
  10,000-character analysis bound, and python's file-write and computed-text spellings. Both pass §The
  generic ruleset's test, since neither admits anything a project owns. Their cost is a harness or
  language revision, not privacy. Each is stated as an honest limit below, beside the wrapper set's.
- **Consumer config:** none added. The python arm is rule content a consumer shadows, like every rule.
- **Private rule content:** none in reach.

## What changes

### (1) The friction log records a call losslessly, up to the harness's analysis bound

`guard_log_fallthrough` appends the command encoded rather than flattened, and cut at the harness's
analysis bound rather than at 500 characters {design-bearing}.
**Not yet applied.**

- **The encoding.** Each `\` is written as `\\`, each newline as `\n` and each tab as `\t`. Nothing
  else changes. One call is still one line, and the line decodes back to the command.
- **The cut.** The line carries the command's first 10,001 characters. The harness asks about every
  command longer than 10,000 characters, whatever the allowlist says. So a decoded line longer than
  10,000 characters is enough for the ranker to know the call was over the bound (delta 3). A shorter
  command is logged whole. That keeps the census's population whole: the median inline python body is
  844 characters, and 3 of 8,112 Bash calls exceeded the bound.
- **No spawn.** The encoding and the cut are shell parameter expansions. The `tr` and `cut` pipeline
  this replaces goes, so logging a fall-through spawns no program. The append stays best-effort and
  never affects the decision.
- **The honest limits.** The bound is a harness literal, and a harness revision that moves it is drift
  nothing here detects. The cut counts characters the way the hook's shell locale counts them. Under a
  byte locale, a multi-byte command near the bound may read under it. A line logged before a consumer
  upgrades is flattened, and it decodes wrongly where it carries a backslash. The log is cleared at
  every close, so that lasts one iteration.

**Replacement text, guard-kit/SPEC.md §The guard framework**, the `guard_log_fallthrough` bullet
(**Not yet applied**):

> - `guard_log_fallthrough` — append the command to the friction log, one line per call: `\`, newline
>   and tab encoded as `\\`, `\n` and `\t`, cut to the harness's analysis bound plus one character
>   (§scan-prompts reads both); best-effort, never affects the decision.

### (2) The compiled skeleton and splitter twins take newline-bearing input

`guard::skeleton` gains the heredoc-body arm its shell holder has, and `guard::split_compound` treats a
newline as a segment boundary, so a decoded log line is in both twins' contract {mechanical}.
**Not yet applied.**

- **Skeleton.** The twin ports the shell holder's opener scan and pending-terminator queue. An opener
  is `<<` or `<<-`, optional blanks, then a double-quoted, single-quoted or bare-identifier delimiter,
  and a here-string `<<<` is not one. At each newline, each pending body runs to the first line whose
  leading whitespace, stripped, equals its delimiter. A non-empty body becomes the `HD` placeholder
  line when `hd` is requested, or when `hdq` is requested and the delimiter was quoted. The terminator
  line stays live. An unterminated body runs to the end of the input. `NewlineInInput` retires and the
  function returns its view directly.
- **Splitter.** A newline ends a segment, as the shell holder's line-per-segment output already
  implies.
- **The oracle is the parity arm, not a reading of the shell.** `--guard-lib-parity`'s corpus gains
  newline-bearing shapes, and both holders print each command and view in delta 1's encoding, so a
  record stays one line. The shapes:
  - a single-quoted, a double-quoted and a bare delimiter;
  - `<<-` with a tab-indented terminator;
  - two openers on one line;
  - a separator inside a body;
  - a command after the terminator;
  - an unterminated body;
  - an empty body.

  The `newline-out-of-contract` assertion retires with the refusal it asserted. The `hd-inert`
  assertion stays, since it is still true of a newline-free command.

**Replacement text, guard-kit/SPEC.md §The guard framework** (**Not yet applied**). In the
`guard_skeleton` bullet, the passage from "Its **compiled twin implements the reachable subset
only**" to "with nothing following it" becomes:

> Its compiled twin carries the whole machinery, the heredoc arm included, because a friction-log line
> decodes to the multi-line command it recorded (§scan-prompts).

In the `guard_split_compound` bullet, the sentence opening "Its separator class carries **no
newline**" stays, with this appended: "and its compiled twin, which returns segments rather than lines,
ends one at a newline for the same reason."

### (3) scan-prompts reads a heredoc as the harness does

The ranker decodes each log line and reads it through the heredoc-aware views, so a heredoc body is
data to the grant test, the command after its terminator is shell, an unquoted body is live for the
expansion test, and a call over the analysis bound is allowlist-unreachable {design-bearing}.
**Not yet applied.**

- **Decode first.** `\\`, `\n` and `\t` decode to a backslash, a newline and a tab. Any other backslash
  pair, and a trailing lone backslash, stay literal. Every reading below takes the decoded command.
- **The grant test.** A heredoc body and its terminator line are dropped from the segment set. The
  extent comes from the twin skeleton's `hd` arm, and the quoted-span collapse runs over what remains,
  so an apostrophe inside a python body cannot pair with a quote outside it. Every remaining segment
  must be granted, as today. So `python3 - <<'EOF'` with any body reads granted under
  `Bash(python3 -*)`, while the same call with `perl -e 1` after the terminator does not.
- **The reachability verdict, three shapes.** Expansion reads the `sq hdq` view: a quoted-delimiter
  body is inert and an unquoted one is live, which is the harness's `$HOME` denial. The write redirect
  reads the `sq dq hd` view. `before_heredoc` retires, because the scans no longer stop at an opener.
  The third shape is new. **A decoded line longer than the harness's analysis bound is
  allowlist-unreachable**, on the harness's own documented refusal, and so it is never granted.
- **The key** reads the first segment of the decoded `sq dq hd` skeleton, so a heredoc-bearing call
  keys exactly as it keys today (`python3 -`, `cat >`).
- **A definitional step, recorded at landing.** The grant test moves heredoc-bearing calls from
  prompting to granted, and the third shape can move an over-bound call the other way. The build
  records `<patterns>/<occurrences>` for one log immediately before and after, in the §scan-prompts
  paragraph family that records such steps. The KPI is not rebased.

**Replacement text, guard-kit/SPEC.md §scan-prompts** (**Not yet applied**). In the reachability
paragraph, "and two shapes qualify" becomes "and three shapes qualify". The sentence set from "**A
heredoc body is not read**" to "the scan stops there." becomes:

> **A heredoc body is read the way the harness reads it**, which was probed rather than assumed: the
> body is data, the text after its terminator line is shell, and an unquoted-delimiter body still
> expands. So the expansion test blanks only a quoted-delimiter body, the redirect test blanks every
> body, and the log line is decoded first (§The guard framework, `guard_log_fallthrough`). The third
> shape is a call longer than the harness's analysis bound, which the harness asks about whatever the
> allowlist says. A decoded line longer than that bound is one.

After the "**The grant test reads the verdict**" paragraph, a new paragraph:

> **The grant test drops each heredoc body and its terminator line from the segments it matches.** A
> body is not a command, and a split inside it would read `;` and `|` in a python body as separators.
> The honest limit is that the analysis bound is the harness's literal, so a revision that moves it is
> drift this arm cannot detect.

### (4) Rule 8 steers an inline python literal rewrite to `--rewrite`

Rule 8 gains a python arm: a segment leading with `python3` or `python` whose program body the command
string carries is **blocked** when that body is a literal rewrite. The steer is to `--rewrite`, and to
the Edit tool {design-bearing}.
**Not yet applied.**

- **The body source.** Two sources are read:
  - **the `-c` argument**, through `_guard_program_operands`, which gains a `python` row;
  - **the body of the heredoc feeding the segment's stdin**, through a new internal helper,
    `_guard_heredoc_body <cmd> <k>`. It prints the body of the command's `<k>`th heredoc opener, with
    the extent guard_skeleton's own heredoc arm uses.

  A body from anywhere else is rule 23's subject or no rule's, and the arm declines on it. That covers
  an operand path, a `<` redirect and a pipe.
- **The `python` row of `_guard_program_operands`.** `-c` consumes the next word and supplies the
  program. `-` names stdin. `-u`, `-B`, `-E`, `-I`, `-s`, `-S`, `-O` and `-q` take no argument, while
  `-W` and `-X` consume one. `-m` and any other option decline.
- **A literal rewrite**, all three conditions on the body text:
  - **(i) it writes a file**, through `open(` with a string-literal mode containing `w` or `a`, or
    through `.write_text(`;
  - **(ii) it carries `.replace(`**;
  - **(iii) it carries no computed-text construct:** an `f"`/`f'` prefix, `.format(`, a `%` after a
    closing quote, `re.`, any `import`, `.index(`, `.find(`, `.join(`, `.split(`, a subscript holding
    `:`, `input(`, `sys.`, or, in an unquoted-delimiter body, a `$`.

  The literal pairs may sit inline, in a helper the body defines, or in a loop over a literal list. All
  three are the recurring idiom, and none computes its text.
- **The steer** names three routes: `--rewrite` with `--expect <n>` for the count assertion, one call
  per pair, `\n` escapes carrying a multi-line literal on one line; the Edit tool for a multi-line
  literal; and `!<command>` where the program is intended as written.
- **What passes:**
  - a body that only reads;
  - a body with any construct on list (iii);
  - a body writing through `shutil`, `os` or `pathlib` without `.write_text(`;
  - a `python` behind another command word;
  - an interpreter other than `python` and `python3`, since the census found none inline.
- **Placement:** inside rule 8, so ahead of both auto-allow rules. A committed `Bash(python3 -*)` grant
  is not argued with, on the awk arm's ground: a block is not overridden by an allow, and the grant
  stays load-bearing for every body the arm declines.
- **The honest limit.** The arm reads python's spelling, not its semantics. A literal rewrite spelled
  through a construct on list (iii) passes, and a computed one hiding behind none of them is blocked.
  Its steer names `!<command>` for that case. The lean is toward passing, as rule 8's other arms lean.

**Replacement text, guard-kit/SPEC.md §The generic ruleset, rule 8** (**Not yet applied**). The lead
becomes "**`sed` or `awk` reading a file, or `sed`, `perl` or an inline `python` body rewriting one**".
The paragraph "**`python3 -` bodies are not steered, because no target exists yet.**" is replaced by:

> **The `python` arm blocks an inline body that is a literal rewrite**: it writes a file through `open(`
> with a `w` or `a` mode or through `.write_text(`, it calls `.replace(`, and it carries no
> computed-text construct (an f-string, `.format(`, `%` formatting, `re.`, an `import`, `.index(`,
> `.find(`, `.join(`, `.split(`, a slice, `input(`, `sys.`, or a `$` in an unquoted-delimiter body). The
> body is the `-c` argument (`_guard_program_operands`' `python` row: `-c` supplies the program, `-W`
> and `-X` consume one word, `-u -B -E -I -s -S -O -q` none, anything else declines) or the heredoc
> feeding the segment's stdin (`_guard_heredoc_body`). It steers to `--rewrite` (`--expect` for a count
> assertion) or the Edit tool, and names `!<command>`. **Why this shape and no wider one:** the inline
> bodies that recur across sessions are a literal read, replace and write-back, which `--rewrite`
> performs with an effect its command line states. Computed replacements, tallies and probes have no
> such tool, so they pass and stay measured friction. The arm reads spelling, not semantics, and leans
> toward passing.

### (5) Tests {mechanical}

- **`guard-kit/guard-tests/cases.tsv`**, rule 8 rows, heredoc rows using `@NL@`.
  - **Block:** a quoted-delimiter heredoc that reads, asserts `count(...)==1`, replaces and writes
    back; the same idiom through a `def sub(p,o,n)` helper; a `python3 -c` one-line rewrite.
  - **Fallthrough:** an f-string replacement, a `re.sub`, an `import json` body, a pure-read body, an
    unquoted-delimiter body carrying `$`, and `python3 -m`.
  - **Rule 23's own row** `python3 - < .tmp/x.py` keeps its verdict.
- **`guard-kit/gate-tests/guard-lib-parity.test.sh`:** delta 2's newline-bearing shapes for `skeleton`
  and `split`, with the encoded record. The `newline-out-of-contract` assertion is removed.
- **`guard-kit/gate-tests/scan-prompts.test.sh`**, end-to-end through the front end, with logs written
  in delta 1's encoding under a `Bash(python3 -*)` grant:
  - a multi-line body carrying `;` reads granted;
  - an ungranted command after the terminator reads prompting;
  - an unquoted body carrying `$HOME` reads prompting and unreachable;
  - a 10,001-character line reads prompting and unreachable.
- **In-crate, `scan_prompts.rs`:** the decode, including a trailing lone backslash, and the grant
  test's body drop. `guard.rs` replaces its `NewlineInInput` test with heredoc cases.
- **The guard's own log write:** a guard-tests or gate-tests case runs a heredoc-bearing fall-through
  through the template guard. It asserts the logged line decodes to the command.

### (6) Rosters, mirrors and the release declaration {mechanical}

- `docs/guard-kit/SPEC.md`, regenerated with `--emit docs-mirror --write`.
- `.workflow/release-declarations.md`, one guard-kit bullet. The friction log's line format changes,
  and a line logged before the upgrade may mis-decode until the next close clears the log. Rule 8 now
  blocks an inline python literal rewrite, which a committed `Bash(python3 -*)` grant previously let
  through. No action is required.
- `guard-kit/SPEC.md` §Testing, where it describes the parity corpus's newline-free scope.

## Producers and consumers

- **The encoded log line** — producer: `guard_log_fallthrough`, called by the consumer's copy of
  `templates/bash-guard.sh` on every fall-through. This repo's `scripts/bash-guard.sh` enables it.
  Consumers: `scan_prompts::tally` and, through `scan_prompts::count`, `kpi-prompt-friction`. The
  encoding's reader is the decode. The cut's reader is the over-bound verdict.
- **The heredoc-aware twin views** — producer: `guard::skeleton` and `guard::split_compound`.
  Consumers: the ranker's grant test, verdict and key, and `--guard-lib-parity`'s comparison against
  the shell holders.
- **The third unreachable shape** — reader: `unreachable_section` and the headline's `<u>` clause. It
  moves no call off the headline, since an unreachable call is prompting by definition.
- **`_guard_heredoc_body`** — caller: rule 8's python arm. It is an internal `_`-prefixed helper with
  one shell caller. So it carries no compiled twin and adds no parity mode.
- **The python row of `_guard_program_operands`** — caller: rule 8's python arm.
- **The rule 8 block** — consumer: the agent, through the harness's block message. The steer targets
  `--rewrite` and Edit exist, and the front-end grant reaching `--rewrite` is in this repo's committed
  settings and in `templates/settings-allow.json`.
- **Point 5, readers of a narrowed corpus.** Delta 4 narrows the log: a blocked call never logs. The
  census row `python3 -` shrinks for a reason other than fewer rewrites, which §scan-prompts' grant
  boundary paragraph already warns a trend reader about. `kpi-prompt-friction` asserts
  `^[0-9]+/[0-9]+$` and holds no floor, so a smaller corpus cannot red it. Delta 2 removes a
  parity-test assertion. That test's red condition is any classification mismatch or a failed
  assertion, never a count floor.
- **Point 2, roster-holding readers of the minted names.** `_guard_heredoc_body` lands in
  `lib/guard.sh`, whose helpers `check-comment-tier` reads for a directive comment. The build gives it
  a `# spec:` line, as every helper beside it has. No gate holds a roster of `_guard_*` helpers. Probe:
  `git grep -n "_guard_program_operands"` over the tracked tree, for where a helper of this class is
  rostered.

Roster derivations, re-derived by the build:

- `git grep -n "prompt-friction\|GUARD_KIT_LOG\|guard_log_fallthrough"` over the tracked tree, for the
  log's producers and readers.
- `git grep -n "guard::skeleton\|guard::split_compound\|NewlineInInput"` over `native/src`, for the
  twins' callers.
- `git grep -n "before_heredoc\|newline-flattened\|newline-out-of-contract"` over the tracked tree, for
  the retired spellings.

## Existing sections updated

- `guard-kit/SPEC.md` — §The guard framework: the `guard_log_fallthrough` bullet (delta 1), and the
  `guard_skeleton` and `guard_split_compound` bullets (delta 2).
- `guard-kit/SPEC.md` — §scan-prompts: the reachability paragraph, the grant-test paragraph and the
  definitional-step record (delta 3).
- `guard-kit/SPEC.md` — §The generic ruleset's rule 8 (delta 4) and §Testing (delta 6).
- `guard-kit/lib/guard.sh` — `guard_log_fallthrough` (delta 1); `_guard_heredoc_body`, the python row
  of `_guard_program_operands` and rule 8's python arm (delta 4).
- `native/src/guard.rs` — `skeleton`, `split_compound`, their tests and the retired error type
  (delta 2).
- `native/src/main.rs` — `--guard-lib-parity`'s skeleton mode, which stops refusing a newline-bearing
  command and prints encoded records (delta 2).
- `native/src/emit/scan_prompts.rs` — the decode, the grant test, the verdict, the key and the retired
  helper (delta 3).
- `guard-kit/gate-tests/guard-lib-parity.test.sh` — the corpus, the encoded record and the retired
  assertion (deltas 2 and 5).
- `guard-kit/gate-tests/scan-prompts.test.sh` and `guard-kit/guard-tests/cases.tsv` (delta 5).
- `docs/guard-kit/SPEC.md` — the on-site SPEC mirror (all deltas).
- `.workflow/release-declarations.md` — the guard-kit bullet (delta 6).

## Retired spellings

- `before_heredoc` — the ranker's stop-at-the-opener helper, retired when its scans read the
  heredoc-aware views (delta 3).
- `NewlineInInput` — the twin skeleton's refusal of a newline-bearing command (delta 2).
- `newline-flattened` — the `guard_log_fallthrough` bullet's description of the log line (delta 1).
- `newline-out-of-contract` — the parity test's assertion on the refusal (delta 2).

## Definition of Done

- [ ] **Causal completeness** — the encoded line, the twin views, the third unreachable shape,
      `_guard_heredoc_body` and the python arm each have a named producer and named consumers.
- [ ] **Order held** — deltas 1 to 3 land before delta 4, and the definitional step's before-and-after
      reading is recorded in §scan-prompts at delta 3's landing.
- [ ] **Instruction surfaces: instruction only** — rule 8's block message and the steer carry no
      grounds; delta 4 places them.
- [ ] **Merged with no information lost** — each SPEC passage re-phrased, not appended.
- [ ] **Entries moved before the drain stage** — both queue entries move to Done at a stage before
      `LIFECYCLE_KIT_DRAIN_STAGE`, with the merge that deletes this file.
- [ ] **Amendment deleted** — this file removed on merge, and none remain at the root for this unit.
- [ ] **Roster re-derived** — the probes above re-run against the tree before the merge counts as
      complete.
- [ ] **Removals propagated** — `check-amendment-retired-spelling` green.
- [ ] **Gaps filed** — any cross-component gap found while landing filed through the gap inbox.
