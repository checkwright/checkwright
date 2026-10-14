# SPEC amendment: consumer-word-read

A consumer rule command reads the command through `--guard-json view` alone, and no view gives it a word as the shell passes it. A skeleton view replaces a quoted word with a mark, so a rule matching one misses every quoted or escaped re-spelling of a word it keys on. The dequoted view is one string, so a rule matching it cannot tell a word from part of a quoted value. This repo's three rules in `scripts/guard-rules.sh` match the `sq dq hd` view, and `scripts/guard-rules-variant-limits.tsv` declares the 19 pairs of rule and re-spelling they miss. Run at this amendment's authoring with the table emptied (`GUARD_KIT_CONSUMER_VARIANT_LIMITS` pointed at a comment-only file), `--run-guard-tests` exits 1 on 50 derived re-spellings, every one in the consumer lane, among them `git clean '-X' -n`, `git clean \-X -n` and `git cl'ean' -X -n`.

The engine already holds the read a rule needs: the word read (§The reader and its views) gives each segment's words with their values. This amendment hands its values to a consumer command as a `--guard-json` mode, and has this repo's three rules read it.

## What changes

### (1) `--guard-json words` prints the word read's values per segment

A new `--guard-json` mode, `words`, reads a payload on stdin and prints the values of the word read over the command `view` reads, one line per segment {mechanical} {user-facing: the entry's Deliverable — a `--guard-json` verb printing the word read's values per segment}.

- **The read.** The command is `tool_input.command` with its trailing newlines stripped, read through the reader its `tool_name` selects, as `view` reads it. Each segment of the word read is one line, in order. A segment's words print as their values, joined by a tab, and a segment with no word prints as an empty line. Within a value a backslash prints as `\\` and a tab as `\t`, the fall-through line's encoding (§The shell guard), so a tab on the line always parts two words. No value carries a newline, since the word read answers nothing on a newline inside quotes.
- **Nothing printed.** `words` prints nothing wherever `view` prints nothing: a tool no reader serves, a payload that does not parse, an empty command, a command carrying a control byte. It also prints nothing wherever the word read answers nothing, a heredoc-bearing bash command and a PowerShell here-string among them (`native/src/guard/bash.rs` and `native/src/guard/powershell.rs` pin both in their word-read tests). A caller reads the empty output as *no word read* and falls back to `view`.
- **No operand.** `words` takes none, and an operand is a usage error at exit 2. A face other than the value is not printed: the spelling and the live text have no consumer caller, and §The reader and its views adds no method ahead of a caller.
- **The crate.** `json_arm` in `native/src/guard/mod.rs` gains the mode and its usage line names it. A `json_words` beside `json_view` reads the payload, refuses the control byte as `json_view` does, and renders `word_read`'s answer. Its unit test holds a quoted, an escaped and a split word to their values, two segments to two lines, a quoted tab to `\t`, a PowerShell backtick-escaped word to its value, and a heredoc-bearing command to no output.
- **The seam cases**, in `guard-kit/gate-tests/consumer-rules.test.sh` beside `view`'s: `words` over a quoted, an escaped and a split word printing each value, over two segments printing two lines, over a quoted tab printing `\t`, over a PowerShell payload's backtick-escaped word printing its value, over a heredoc-bearing command and a `Read` payload printing nothing, and refusing an operand at exit 2.
- **The marks, as align asked** (lead decision, 2026-10-14, a direction not a ruling): delta 1's `{user-facing}` mark settles the spelling `words`, the line format and the encoding, and delta 2 stays unmarked as user-facing; both marks stand as written.
- **The release declaration** (`.workflow/release-declarations.md`, §Behavior changes) gains one bullet: **guard-kit `--guard-json`** — a new mode, `words`, prints the values of the payload command's words, one line per segment with its words tab-separated, so a consumer rule can compare a quoted or escaped word by the value the shell passes. It prints nothing where the read does not answer, a heredoc-bearing command among them. Nothing to do; to have a rule see through a quoted word, read `words` and fall back to `view sq dq hd` where it prints nothing.

Replacement for §Consumer rules' `view` bullet, as two bullets. **Not yet applied.**

> - **`view <view>`** reads a payload on stdin and prints the named view of its `tool_input.command` through the reader its `tool_name` selects, the command read as the member reads it, with its trailing newlines stripped. It takes the view spellings the reader carries — `raw`, `sq`, `hdq`, `sq hdq`, `sq dq hd`, `sq dq hdq` and `dequoted` — as one operand or as separate words; `body` and any other spelling are a usage error. It prints nothing for a tool no reader serves, a payload that does not parse, an empty command, or a command the member blocks for a control byte (§The shell guard). Each placeholder prints as its two letters. That is the quote- and heredoc-inert match surface §Writing a consumer rule tells a rule to match on, without a JSON parser of its own.
> - **`words`** reads the same command and prints its word read's values (§The reader and its views), one line per segment: each word's value, the words joined by a tab and a segment with no word an empty line, with a backslash and a tab in a value printed as `\\` and `\t`, the fall-through line's encoding (§The shell guard). It takes no operand. It prints nothing wherever `view` does and wherever the word read answers nothing, a heredoc or a here-string among them. A rule comparing a word whole reads it here: a skeleton view replaces a quoted word with a mark, and the dequoted view is one string, in which a quoted value naming a flag reads as the flag.

Replacement for §Writing a consumer rule's third discipline bullet. **Not yet applied.**

> - a substring rule sees the entire tool input, heredoc bodies included, so prose that merely *names* a guarded command, such as a `git commit -F -` message describing one, false-blocks. Compare words whole, off `--guard-json words`, which reads a quoted or escaped word by the value the shell passes; where it prints nothing, match the skeleton view `--guard-json view` prints, naming the classes inert for the rule (`sq dq hd` for any rule that does not itself test for an expansion). Keep longer prose in a quoted `-m` span or a `-F <file>` the hook never reads.

Replacement for §Testing's `--guard-json` seam sentence, the first sentence of its paragraph. **Not yet applied.**

> `--guard-json`'s seam cases sit beside them: a field read verbatim, `view` over a heredoc-bearing and a quoted command, `view` over a PowerShell payload printing its skeleton, and `view` printing nothing for a payload of a tool no reader serves (`Read`). `words` prints a quoted, an escaped and a split word as its value, two segments as two lines, a quoted tab encoded, and a PowerShell backtick-escaped word as its value; it prints nothing over a heredoc-bearing command and a `Read` payload, and refuses an operand.

Replacement for the clause of `gate-sdk/SPEC.md` §The non-gate arm, in the bullet opening *It owes a named caller instead*, that names what the consumer command reads through the arm: a word value is neither a field nor a view. **Not yet applied.**

> `--guard-json` by a consumer's rule command, which reads payload fields, views and word values and renders envelopes through it (guard-kit/SPEC.md §Consumer rules).

### (2) This repo's three rules compare word values

`scripts/guard-rules.sh` reads `--guard-json words` and matches each rule per segment on whole word values, falling back to its present `sq dq hd` match where the mode prints nothing, and `scripts/guard-rules-variant-limits.tsv` loses all 19 rows {mechanical}.

- **The read.** The script reads `words` once. A failed read is the fault it prints today for a failed view read, at exit 1. Where the output is empty it reads the `sq dq hd` view and runs today's three matches unchanged. Otherwise it splits each line at its tabs into the segment's words, and a rule fires where any segment satisfies it:
  - **`hook_bypass`**: a word `git` followed at once by a word `commit`, and after them a word `--no-verify` anywhere, or `-n` as the word right after `commit`, today's two spellings read as whole words;
  - **`session_scratchpad`**: a word whose value contains `/tmp/claude-`;
  - **`git_clean_ignored`**: a word `git` followed at once by a word `clean`, and after them a word matching `^-[A-Za-z]*[xX]`, today's pattern anchored at the word's start.
- **The messages and the block's exit are unchanged**, and so is the file's `no-port` cause: the predicates are this project's rule content.
- **What moves, each in the block direction or out of a false block:**
  - every quoted, escaped or split spelling of a word the three rules key on now blocks, which is the 19 rows' deletion;
  - a rule no longer fires across a separator: `git clean -fd; ls -x` and `git commit -m x; echo --no-verify` fall through, where today's whole-command match blocks both;
  - a quoted word carrying `/tmp/claude-` now fires the consumer's `session_scratchpad`, a quoted message naming the path included. This repo's commit messages carry no local path (CLAUDE.md), and a heredoc message, which the fallback reads as today, keeps its body inert;
  - a `-m` message whose whole value is `--no-verify` blocks, since a word compared whole cannot tell an option from an option's argument. It costs one re-issue, in the direction a guard errs in.
- **The limits table** keeps its header and no row, and `GUARD_KIT_CONSUMER_VARIANT_LIMITS` stays bound to it, so the consumer lane still derives every re-spelling and now holds each to its row with no limit. A comment-only table is a readable one: the authoring run above exited 1, not the unreadable table's 2.
- **`scripts/guard-rules-cases.tsv`** gains, each in its rule's section:
  - `block	git commit --no-verify -F - <<'MSG'@NL@x@NL@MSG`, holding the fallback live, since a heredoc-bearing command gets no word read;
  - `fallthrough	git commit -m x; echo --no-verify` and `fallthrough	git clean -fd; ls -x`, holding the per-segment read;
  - `block	git commit -m 'see /tmp/claude-0'`, holding the quoted-value reach.
- **The script's `# spec:` comment on the view** is rewritten to the read it now takes: word values per segment, the `sq dq hd` view where none is read.
- **The order inside the unit:** delta 1 lands first, since the script calls the mode. The rule edit and the 19 rows' deletion are one commit, because the lane reds on a limit row under which nothing fails and, without the rows, on every re-spelling a rule still misses.

The bash word read answers on all 50 derived re-spellings, and each of delta 2's predicates holds on their values: a unit probe over the 50 failing commands, run against the reader's `word_read`.

**Inferred, cannot run before build:** that the lane is green with no limit row — the mode the script reads does not exist until delta 1 lands, and the probe reads the reader and no script; `--run-guard-tests` after delta 2 settles it, and a re-spelling still failing is a predicate to fix in that commit, never a limit row restored.

## Producers and consumers

- **The `words` output.** Producer: `json_arm`'s `words` mode, on a consumer command's call, which `GUARD_KIT_CONSUMER_RULES_CMD` names and which this repo's knob file sets (`scripts/guard-config.knobs`), so the producer is reached on every guarded call here. Consumer: `scripts/guard-rules.sh`, by command substitution, once per call. Each field has its reader: the line is a segment, read by the per-segment predicates; the tab parts words, read by the split; a value is compared by the three rules. The empty output is read by the fallback.
- **The roster-holding readers of the mode set**, by `git grep -ln 'guard-json'` over the tracked tree: `guard-kit/SPEC.md` (§Consumer rules, §Writing a consumer rule and §Testing, delta 1), `gate-sdk/SPEC.md` (§The non-gate arm, delta 1), `native/src/guard/mod.rs` (the usage line, delta 1), `guard-kit/gate-tests/consumer-rules.test.sh` (its closing summary line, with the seam cases) and the generated `docs/guard-kit/SPEC.md` and `docs/gate-sdk/SPEC.md`. `TOP_LEVEL_FLAGS` in `native/src/main.rs` rosters `--guard-json` itself and no mode, and `guard-kit/README.md` names the flag alone.
- **The consumer lane's red conditions under delta 2's narrowing of the limits table to no row** (canon-kit/SPEC.md §The causal-completeness check, point 5), from §Testing: a re-spelling no limit admits reds, which the lift must leave none of; a limit row under which nothing fails reds, which leaves the rows nowhere to stay; an unreadable table is exit 2, and a comment-only one was read. No count or floor reads the table.
- **Each of the 19 rows' satisfying value** (point 6), by `grep -v '^#' scripts/guard-rules-variant-limits.tsv`: every transform the rows name, `sq`, `dq`, `esc` and `part` at a `lead` or an `arg` site, quotes or escapes a word's head, and the word read's value removes the quotes and resolves the escape. Each derived word therefore has the value of the word it re-spells, which the predicates of delta 2 read whole. That the read answers on each derived command is the marker above.
- **Knobs.** None is minted and none gains a reader. `GUARD_KIT_CONSUMER_VARIANT_LIMITS` keeps its value.
- **The seam.** The mode is kit mechanism: it prints what the engine already reads and names no project's vocabulary. The three predicates are this repo's rule content and stay in its command. The empty limits table is this repo's binding of an existing knob.

## Existing sections updated

- `guard-kit/SPEC.md` §Consumer rules — the `view` bullet becomes two (delta 1).
- `guard-kit/SPEC.md` §Writing a consumer rule — the third discipline bullet (delta 1).
- `guard-kit/SPEC.md` §Testing — the `--guard-json` seam sentence (delta 1).
- `gate-sdk/SPEC.md` §The non-gate arm — the caller clause of the bullet *It owes a named caller instead* (delta 1).
- `native/src/guard/mod.rs` — the mode, its usage line and its unit test (delta 1).
- `guard-kit/gate-tests/consumer-rules.test.sh` — the seam cases and the closing summary line (delta 1).
- `.workflow/release-declarations.md` — the §Behavior changes bullet (delta 1).
- `scripts/guard-rules.sh` — the read, the three predicates and the `# spec:` comment (delta 2).
- `scripts/guard-rules-variant-limits.tsv` — the 19 rows deleted (delta 2).
- `scripts/guard-rules-cases.tsv` — the four rows (delta 2).
- `docs/guard-kit/SPEC.md` and `docs/gate-sdk/SPEC.md` — generated, stale once the delta lands (all deltas).

## Retired spellings

- None — the amendment adds a mode and changes this repo's rule content, and retires no spelling.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls guard-kit/SPEC-*.md`), discharged at the iteration.
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Entry moved** — the queue entry moves to Done in the merging commit, ahead of the drain stage.
