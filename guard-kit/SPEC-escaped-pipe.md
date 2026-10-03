# SPEC amendment: escaped-pipe

**The bash reader's compound, statement and pipe splits stop cutting at an escaped `|` and at a backslash before a newline, as they already stop at an escaped `;`.** Today `split_on` in `native/src/guard/bash.rs` exempts only `;` from its bytewise match (the `escaped` filter), and the reader's `pipes` cuts at every `|` and newline. So `grep foo a \| b` arrives as the segments `grep foo a \` and ` b`, and `grep -c x a.md \`, newline, `b.md` as two statements, where the shell and the harness both read one command.

**Measured at authoring.** The method is §The reader and its views' own: Claude Code 2.1.288, `claude -p --safe-mode --setting-sources '' --permission-mode dontAsk --settings <file>` with the one grant `Bash(touch -c .tmp/hm/a:*)`, told to run one command verbatim, read off `permission_denials`, with the command run confirmed from the `stream-json` tool input on every probe.

- **Controls, refused:** `touch -c .tmp/hm/a | touch -c .tmp/hm/b`, and the same two commands on two lines. The matcher splits at a bare `|` and at a newline.
- **Granted:** `touch -c .tmp/hm/a \| .tmp/hm/b`, and `touch -c .tmp/hm/a \`, newline, `.tmp/hm/b`. Had the matcher split at either, the second segment would be ungranted. So it splits at neither.
- **Even runs, refused:** `touch -c .tmp/hm/a \\| touch -c .tmp/hm/b`, and `touch -c .tmp/hm/a \\`, newline, `touch -c .tmp/hm/b`. A literal backslash is followed by a real break.
- **Refused:** `touch -c .tmp/hm/a \|| touch -c .tmp/hm/b`. The escaped `|` is a word byte, and the `|` after it is a real pipe.

So the matcher's rule for `|` and newline is the one already measured for `;`: an odd backslash run escapes the byte, and an even run does not.

## What changes

### (1) The splits read an escaped `|` and a continued line as the shell does {design-bearing} {user-facing: bash-reader-escaped-pipe's deliverable, the splits matched to the harness's measured reading}

In `native/src/guard/bash.rs` the escape test that `split_on` applies to `;` applies to every separator whose first byte is `;`, `|` or a newline: `;`, `|`, `||`, `|&` and newline. A separator so escaped is no cut. Its first byte stays in the open segment beside its backslash, as two literal bytes, and the next byte is read fresh, so `\||` is a literal `|` then a pipe and `\|&` a literal `|` then a lone `&`, which is no separator. `&&` keeps its bytewise match, since no probe has measured `\&&`. That is the conservative direction, because an extra segment can only withhold a grant. The reader's `pipes` takes the same test for its `|` and newline cuts. The test counts the backslash run in the view the split is fed, so in a skeleton a backslash inside a quoted span is a placeholder and never counts.

Two things do not move. **The residue split's line cut stays physical.** `residue_statements` cuts at every newline before it reads heredoc residue. A continued emitter write there is two statements, so rule `emitter_write` reads it as a compound, the conservative direction. **The dequoted view still carries a backslash out of a quoted span unchanged**, so `'a\'|b` cuts twice on the skeleton and once on the dequoted view. This mismatch already exists for `;`. Every reader that grants off a dequoted split withholds where the two splits differ in count: the declared-forms reader `ro_forms_clear` and rule `grant_path_slot`'s `slot_reach`, which `rewrite_granted` calls. The probe was `grep -n "ctx.dequoted" native/src/guard/rules/*.rs` and a read of each hit. Delta 2 pins one row for it.

The module's own tests, replacing the pin `split_compound("a \\| b") == ["a \\", " b"]`:

- `a \| b`, one segment; `a \\| b`, two; `a \|| b`, two (`a \|` and ` b`);
- `a \`, newline, `b`, one; `a \\`, newline, `b`, two;
- `a '\|' b`, one, whose skeleton carries no backslash;
- `pipes` over `a \| b`, one member; over `a | b`, two.

**Which verdicts move.** The readers of the splits are the engine, the rules in `native/src/guard/rules/`, and the `scan-prompts` ranker, by `grep -rn "split_compound\|\.segments(\|\.statements(\|\.pipes(\|residue_statements" native/src`. None is rewritten. A call carrying an unquoted `\|` or a backslash-newline reaches each of them as one fewer segment. A grant may now take a call it fell through on, and only where the shell runs one command. Rules that test for a separator with their own pattern still read the byte as a break. Rule `ro_pipeline`'s chain test names neither a lone `|` nor a newline, so it is not one of them. Rule `bounded_wait`'s arm (B) shape test, which refuses a launch carrying `;`, `|` or a newline, and rule `cd_compound`'s bash test are. Both stay as they are, since both decline or block. The decision table is the oracle: build runs `--run-guard-tests` and gives every moved row a cause before it lands. A row that moved for no cause is a defect the change exposed, fixed in this unit and never re-expected.

### (2) Decision-table cases {mechanical}

`guard-kit/guard-tests/cases.tsv`, rule `ro_pipeline`'s section. Its comment over the escaped-`;` rows gains the `|` and newline rows below it:

- `allow	grep foo a \| b`: one read-only segment naming the file `|`. It falls through today (run at authoring, through `--hook shell-guard` on a payload).
- `fallthrough	grep foo a \\| md5sum`: a literal backslash, a real pipe, then a segment off the roster.
- `allow	grep -c foo tracked.md \@NL@scratch.txt`: one statement over two lines. It falls through today.
- `fallthrough	grep -c foo tracked.md \\@NL@md5sum scratch.txt`: a literal backslash, a real break, then a statement off the roster. A newline compound of two roster reads is granted today, so the second statement must be off the roster for the row to test the break.
- `fallthrough	grep foo 'a\' | sort -o out.txt`: the dequoted view's quoted backslash reads as an escape there and not on the skeleton, so the declared-forms reader withholds on the count mismatch, and `sort -o` is never granted.

Rule `append_scratch`'s section gains:

- `allow	printf x \@NL@>> .tmp/j.md`: one statement, as the shell reads it.
- `fallthrough	cat >> .tmp/j.md <<'EOF' \@NL@; touch x@NL@EOF`: the continued opener line puts `; touch x` on the command line, ahead of the body, and the rule's residue test refuses it. It falls through today.

**Inferred, cannot run before build:** the three `allow` rows' verdicts after delta 1, and the continued-opener row's staying `fallthrough` — the splits that decide them are delta 1's, which build lands.

A row whose verdict differs takes its cause into its comment under delta 1's oracle rule.

### (3) §The reader and its views states the escape {mechanical}

guard-kit/SPEC.md. **Not yet applied.**

The compound-split paragraph's last two sentences become:

> Nor is an **escaped separator**: a `;`, `|` or newline after an odd run of backslashes stays in its word, as the shell reads it, so `find … -exec cmd '{}' \;` is one segment ending in the word `\;` and a backslash before a newline continues the line. `&&` matches bytewise, a backslash before it included, and the statement and pipe splits share the escape test.

In *The class is the harness's, measured*, the paragraph opening *The escaped `;` was measured too* becomes:

> The escapes were measured too: `Bash(touch -c a:*)` alone granted `touch -c a \; b`, `touch -c a \| b` and `touch -c a \`, newline, `b`, and refused each separator bare and after an even backslash run. The escaped `&&` is unmeasured, so the split still cuts there, the conservative direction, since an extra segment can only withhold a grant. A rule testing for a separator with its own pattern, such as rule `cd_compound`'s bash test, still reads an escaped one as a break, in that same direction.

In the statement-split paragraph, after *the two-level split rule `script_interpreter` runs*, add:

> The residue split cuts at every newline, a continued one included, so a continued statement carries its residue no further than its own line.

### (4) The site mirror follows {mechanical}

`docs/guard-kit/SPEC.md` is regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` in the commit landing delta 3.

## Producers and consumers

- **The corrected segments, statements and pipe members.** Producer: `split_on` through `split_compound` and `statements`, and the reader's `pipes`, on every bash call the shell guard and the `scan-prompts` ranker read. Enabling configuration: none, since the reader runs wherever the shell-guard member is wired. This repo wires it in `.claude/settings.json`, as does every adopter merging guard-kit's settings template. Consumers: the readers delta 1 lists, each by direct call through the reader seam.
- **The scan-prompts ranking key** for a call carrying an escaped `|` or a continued line now counts one segment where it counted two. Its reader is the close-stage triage, through `--emit scan-prompts`. No field is added.
- **No new state, knob, event or interface.** The PowerShell reader is unchanged: its escape is the backtick, and it already splits nothing at an escaped separator or a backtick-continued line (§The reader and its views).

## Existing sections updated

Roster by `grep -n "escaped\|bytewise\|\\\\;" guard-kit/SPEC.md guard-kit/guard-tests/cases.tsv` and `grep -n "escaped\|fn pipes\|split_on" native/src/guard/bash.rs`, over the tracked tree.

- `native/src/guard/bash.rs`: `split_on`, `escaped`, `pipes` and their tests (delta 1).
- `guard-kit/guard-tests/cases.tsv`: rules `ro_pipeline`'s and `append_scratch`'s sections (delta 2).
- `guard-kit/SPEC.md`: §The reader and its views (delta 3).
- `docs/guard-kit/SPEC.md`: the regenerated mirror (delta 4).

## Retired spellings

- None — the change narrows where the splits cut; it retires no name.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls guard-kit/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
