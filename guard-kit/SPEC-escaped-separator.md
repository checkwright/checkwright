# SPEC amendment: escaped-separator

**The bash reader's compound and statement splits stop cutting at an escaped `;`, and rule `find_exec` reads the `\;` it now keeps.** Today `split_on` in `native/src/guard/bash.rs` matches `;` bytewise, so `find … -exec cmd '{}' \;` arrives as a segment ending in a lone `\` plus an empty segment, and `grep foo a\;b` arrives as two statements. Rule `find_exec` patches its own case (`native/src/guard/rules/tools.rs`, the lone-backslash branch in `find_exec`), and every other reader of the split sees a boundary the shell does not have.

**Measured at authoring.** These are the harness's reading, which is what the split models (guard-kit/SPEC.md §The reader and its views). The method is §The reader and its views' own: Claude Code 2.1.287, `claude -p --safe-mode --setting-sources '' --permission-mode dontAsk --settings <file>` with one grant, told to run one command verbatim, read off `permission_denials`, and the command run confirmed from the `stream-json` tool input.

- The grant `Bash(touch -c .tmp/hm/a:*)` alone **refused** `touch -c .tmp/hm/a ; touch -c .tmp/hm/b`. That is the control: the matcher splits at a bare `;`.
- The same grant **granted** `touch -c .tmp/hm/a \; .tmp/hm/b`. Had the matcher split there, ` .tmp/hm/b` would be an ungranted segment. So the harness does not split at an escaped `;`.
- Probes spelled with `echo` decide nothing: the harness allows `echo` with no grant, so every `echo` variant ran.
- The escaped `|` and a backslash before a newline were not measured. The model rewrote the escaped-pipe probe without its backslash on one run, and a confirming re-run was refused by the session's permission classifier. So both stay split, as today.

## What changes

### (1) The split keeps an escaped `;` inside its word

In `native/src/guard/bash.rs`, `split_on` skips a `;` that follows an odd run of backslashes {design-bearing} {user-facing: bash-reader-escaped-separator's deliverable, the split skipping an escaped separator}. The `;` and its backslash stay in the open segment as two literal bytes, as bash reads them. An even run is a literal backslash followed by a real break. The run is counted in the view the split is fed, which is a skeleton, so a backslash inside a quoted span is already a placeholder and never counts.

Both `split_compound` and `statements` call `split_on`, so the compound split and the statement split change together. `|`, `||`, `&&`, `|&` and newline keep their bytewise match. A separator inside a word is still cut there, so the escape test covers `;` alone.

The module's own tests pin the change on:

- `a \; b` (one segment);
- `a \\; b` (two segments);
- `a \\\; b` (one segment);
- `a '\;' b`, whose skeleton carries no backslash (one segment, unchanged).

**Which verdicts move.** A call carrying an unquoted `\;` reaches every reader of the split as one fewer segment. The readers, by `grep -rn "ctx.segments\|ctx.statements\|split_compound\|statements(" native/src` over the crate: the engine (`native/src/guard/engine.rs`), the rules in `native/src/guard/rules/` (`grants.rs`, `liveness.rs`, `reach.rs`, `spelling.rs`, `tools.rs`, `mod.rs`), and the `scan-prompts` ranker (`native/src/emit/scan_prompts.rs`). No reader is rewritten; each reads the corrected segments through the one holder. The decision table is the oracle for which verdicts move: build runs `--run-guard-tests` and gives every moved row a cause before it lands. A row that moved for no reason is a defect the change exposed, and it is fixed in this unit, never re-expected.

### (2) Rule `find_exec` reads the kept `\;`

In `native/src/guard/rules/tools.rs`, `find_exec` drops its lone-backslash branch {mechanical}. That branch read a segment ending in `\` as the `;` form when only an empty segment followed it. After delta 1 the dequoted segment ends in the word `\;`. The rule's terminator test already strips backslashes before it compares a word with `;` or `+`, so the `\;` form is the `';'` form with no further code. The skeleton and dequoted splits stay equal in count, which the rule already checks, because both views keep the backslash and the `;` outside quotes. So the rule needs no new code.

### (3) Decision-table cases for both

`guard-kit/guard-tests/cases.tsv` {mechanical}. Under rule `find_exec`, the existing rows `block find src -type f -exec grep -q foo '{}' \;` and `fallthrough find src -type f -exec grep -q foo '{}' \; -print` keep their verdicts. The second now falls through because a predicate follows the terminator inside one segment, not because the split leaves one behind. The section gains:

- `fallthrough	find src -type f -exec grep -q foo '{}' \\;` — an even backslash run: a literal backslash operand, a real break, and no terminator.

Under rule `ro_pipeline` the section gains:

- `allow	grep foo a\;b` — one read-only segment naming the file `a;b`. Before delta 1 this was two segments, the second `b`, off the roster, and the shell guard falls through on it today (run at authoring, through `--hook shell-guard` on a payload file).
- `fallthrough	grep foo a\\;b` — a literal backslash, a real break, then `b`. It falls through today and keeps doing so.

**Inferred, cannot run before build:** that rule `ro_pipeline` grants a segment whose word carries `\;`, so the first row's expected verdict is `allow` — the split that hands it one segment is delta 1's, which does not exist until build lands it. If the rule declines the word, the row's expected verdict is `fallthrough` and its comment names the decline.

### (4) §The reader and its views and §The rule roster state the escape

guard-kit/SPEC.md {mechanical}. **Not yet applied.**

In §The reader and its views, the compound-split paragraph's last sentence becomes:

> It is fed a skeleton view, so a separator inside a quoted argument is never mistaken for a statement break, and neither is an **escaped `;`**: a `;` after an odd run of backslashes stays in its word, as the shell reads it, so `find … -exec cmd '{}' \;` is one segment ending in the word `\;`. Every other separator matches bytewise, a backslash before it included.

In the next paragraph, *The class is the harness's, measured*, after the sentence about the `&` refusals, add:

> The escaped `;` was measured too: `Bash(touch -c a:*)` alone granted `touch -c a \; b` and refused `touch -c a ; touch -c b`. The escaped `|` and the backslash before a newline are unmeasured, so the split still cuts at both. That is the conservative direction: an extra segment can only withhold a grant.

In §The rule roster, rule `find_exec`'s first sub-bullet becomes:

> - **The terminator is the word `;` or `+` after dequoting**, so `\;`, `';'` and `";"` are one spelling, and the compound split keeps `\;` whole (§The reader and its views).

### (5) The site mirror follows

`docs/guard-kit/SPEC.md` is regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` in the commit landing delta 4 {mechanical}.

## Producers and consumers

- **The corrected segments.** Producer: `split_on`, reached through `split_compound` and `statements`, on every bash call the shell guard and the `scan-prompts` ranker read (`--emit-compare-settings-allow` calls neither, by `grep -n "split_compound\|segments" native/src/emit/compare_settings_allow.rs`). Enabling configuration: none, because the reader is unconditional wherever the shell-guard member is wired. This repo wires it in `.claude/settings.json`, and so does every adopter installing guard-kit's settings template. Consumers: the readers rostered under delta 1, each by direct call.
- **Readers whose verdict moves.** A command with an unquoted `\;` loses a boundary. A grant rule (`ro_pipeline`, `find_glob`, `cat_file`'s batch, `allowlist_chain`) may now grant a call it fell through on, and only where the shell runs one command, which is the model's premise. A block rule keyed on a lone segment, such as `find_exec`, now fires where the shell has one command. A rule that tests for a `;` with its own pattern rather than through the split still reads `\;` as a separator. By `grep -rn -F '[;&|' native/src/guard/rules/` that is rule `cd_compound`'s bash test alone, which blocks `cd x \; y` as a compound. A block is the conservative direction, so the rule is left as it is.
- **No new state, knob, event or interface.** The PowerShell reader is unchanged: its escape is the backtick, and it already splits nothing at an escaped separator (§The reader and its views).

## Existing sections updated

Roster produced by `grep -n -F '\;' guard-kit/SPEC.md guard-kit/guard-tests/cases.tsv` and `grep -rn "split_on\|lone\|== Some(&\"\\\\\\\\\")" native/src/guard`, over the tracked tree.

- `native/src/guard/bash.rs`: `split_on` and its tests (delta 1).
- `native/src/guard/rules/tools.rs`: `find_exec`'s lone-backslash branch (delta 2).
- `guard-kit/guard-tests/cases.tsv`: the `find_exec` and `ro_pipeline` sections (delta 3).
- `guard-kit/SPEC.md`: §The reader and its views and §The rule roster, rule `find_exec` (delta 4).
- `docs/guard-kit/SPEC.md`: the regenerated mirror (delta 5).

## Retired spellings

- None — the change narrows where the split cuts; it retires no name.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls guard-kit/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
