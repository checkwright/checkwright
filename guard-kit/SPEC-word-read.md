# SPEC amendment: word-read

Two rules read a command word without its quoting. Rule `rm_tracked` tests the words of the `sq dq hd` skeleton, where a quoted operand is a mark, so `rm 'README.md'`, `rm "README.md"` and `rm -r "docs"` pass while `rm README.md` and `rm docs` block; `Remove-Item 'README.md'` passes the same way under a PowerShell payload. Rule `find_exec` re-quotes words of the dequoted view, which keeps an escaping backslash, so `-exec cat a\ b '{}' \;` is steered to `cat 'a\ b'` and an unquoted `-name \*.md` to `-name '\*.md'`, each another argument. Each was run through the member on this tree's own files. This amendment gives the reader a word read that carries a word's quoting and has both rules take it.

## What changes

### (1) The reader answers a segment's words with their quoting

§The reader and its views gains the **word read**: for a segment of the compound split, each word with three faces {design-bearing}.

- **Its spelling**, the bytes the command spells it with, quotes and escapes included. A steer prints this face, so the shell reads the steered word as it read the original.
- **Its value**, what the shell passes: quote characters removed and each escape resolved to the character it protects.
- **Its live text**, the skeleton's word, where a quoted span is a mark and an escaped character is still behind its backslash. A character standing bare in it is one the shell still interprets.

The three are index-aligned, one entry per word, and a word the shell joins across an escaped blank is one entry. The read is a seam method both readers implement, added with its two callers (deltas 2 and 3). It is built from the `raw`, `sq dq hd` and `dequoted` views and mints no view name: a rule calling it declares those three, and the context faults on the read as it faults on an undeclared view. It answers nothing where the dequoted view cannot be aligned, and each caller states what it does then.

Addition after the sentence of §The reader and its views that closes the two derived views, *A reader pairing the skeleton's words with the view joins a skeleton word ending in an odd backslash run to the next.* That sentence stays: the skeleton's held word still pairs with the dequoted view for the rules that read it (`native/src/guard/text.rs`'s `held_words`, bound to the sentence, which `native/src/guard/rules/grants.rs` and `unredirected_words` in `native/src/guard/rules/mod.rs` call). **Not yet applied.**

> The dequoted view keeps an escaping backslash, so in it `a\ b` and a single-quoted `a\ b` are one text. A rule that needs a word as the shell passes it, or as a steer must print it, takes the **word read** instead: each word of a segment as its spelling, its value and its live text, index-aligned, with a word joined across an escaped blank as one entry.

- **Under PowerShell** a value undoes what that reader's dequoted view undoes, a doubled quote and a backtick escape, and the spelling is the command's own bytes.
- **The dequoted view is unchanged**, and so is every rule reading it. Stripping its backslashes, as rule `grant_path_slot` does for its own comparison, would break the single-quoted regex a steer most often carries, which is why the quoting travels with the word and is not recovered from the view.
- **A reader-owned word type and the skeleton's held word beside the dequoted one are the same contract**; which the crate spells is build's.

### (2) Rule `rm_tracked` tests each operand as the shell passes it

The deletion arm reads its operands through the word read, so a quoted or escaped operand reaches the tracked test {design-bearing} {user-facing: the entry's Deliverable — the tracked test takes a quoted operand literally and an unquoted one as the glob the shell expands, and the steer prints the shell's own word}.

- **A literal operand**, one whose live text carries none of `*`, `?` and `[` bare, is tested by its value under git's `:(literal)` pathspec magic.
- **A glob operand**, a word no quote or escape contributed to, is tested under `:(glob)`, where `*` stops at a `/` as the shell's does. The bare pathspec the rule passes today crosses directories: on this tree `*.rs` matches and `:(glob)*.rs` does not, there being no such file at the root.
- **A word mixing the two**, a quoted span or an escape beside a bare glob character, is skipped, on the shared decline.
- **A literal operand whose value carries `*`, `?` or `[`, or opens with `:`, is skipped too**, since the steer's `git rm` would read that value as a pattern and remove more than the `rm` named.
- **The steer prints the operand's spelling**, so `rm 'two words.md'` is steered to `git rm -q 'two words.md'`. The directory test reads the value.
- **Where the word read answers nothing** the arm reads the skeleton's words, as it does today, and a quoted operand there stays unread.
- **Under PowerShell** the deletion commands expand a wildcard in a quoted path as in a bare one, so every path word is tested as a glob by its value, `\` folded to `/` first on rule `worktree_confinement`'s fold, since git reads a pathspec's `\` as an escape. The parameter walk is unchanged and reads values.
- **The force arm is unchanged.**

The item's lead sentence gains the operand reading in place of *An `rm` statement naming a git-tracked path (`git ls-files --error-unmatch`)*. **Not yet applied.**

> An `rm` statement naming a git-tracked path is **blocked**: each operand is tested as the shell passes it (`git ls-files --error-unmatch`), a quoted or escaped one literally and a bare one as the glob the shell expands

**Honest limits**, stated in the item: git's glob lets `*` match a leading dot where the shell's does not; a PowerShell `-LiteralPath` value carrying a wildcard character is read as a pattern; a tracked file whose own name carries a glob character is not reached when quoted.

Every rule taking this rule's test as a predicate narrows with it: rule `bounded_wait`'s arm (B1) and rule `bounded_write`'s clause (e) withhold a grant from a call deleting a quoted tracked path.

### (3) Rule `find_exec` prints the words the command spelled

The steer joins the spellings of the predicate words and of the executed command's words, and re-quotes nothing {design-bearing} {user-facing: the entry's Deliverable — both steers print the shell's own word}.

- **Its tests read values**: the `-exec` word, the terminator, the `{}` placement and the roster test. The terminator is the word whose value is `;` or `+`, which replaces the backslash strip the rule applies to a dequoted word.
- **The declaration gains `raw`**, the word read's third view: the item's clause reads *Declares `raw`, `sq dq hd` and `dequoted`*, and the rule's table row with it.
- **It declines where the word read answers nothing**, as it declines today where the dequoted view does not align.

### (4) The rows and the cases

The decision tables and the module tests take the new verdicts and pin the printed words {mechanical}.

- **The sandbox's third precondition** gains a tracked `two words.md` beside `tracked.md`, and §Testing's list says so. Every row whose verdict a second tracked file moves is re-derived.
- **`guard-tests/cases.tsv`**, block: `rm 'tracked.md'`, `rm "tracked.md"`, `rm two\ words.md`, `rm 'two words.md'`. Not a block of this rule: `rm 'scratch.txt'` and `rm '*.md'`.
- **`guard-tests/powershell-cases.tsv`**, block: `Remove-Item 'tracked.md'` and `del "tracked.md"`.
- **Every existing row carrying a deletion with a quoted or escaped operand, or a bare glob operand, is re-derived**, the non-monotone table's rule (§Testing). Probe: `grep -nE "(rm|del|erase|ri|rd|rmdir|Remove-Item) .*['\"\\\\*?]" guard-kit/guard-tests/*.tsv`.
- **A verdict row cannot hold a steer's words**, since a wrong steer still blocks. The module's own tests hold them: for rule `find_exec`, an escaped blank (`a\ b`), a single-quoted regex backslash (`'.*\.md'`) and an escaped glob (`\*.md`) each printed as spelled; for rule `rm_tracked`, a quoted and a blank-bearing operand.
- **The verdict moves of delta 3 get rows**: `guard-tests/cases.tsv` takes `-exec cat '{}' '\;'` (falls through) and `-exec \cat '{}' \;` (steered), beside the existing escaped-terminator rows, which are re-derived.
- **Each reader's module tests** hold the word read's three faces over a quoted span, an escaped blank, a mixed word and a command the dequoted view refuses.

## Producers and consumers

- **The word read.** Producer: the bash and PowerShell readers, from the command the engine holds, on a rule's call through its context. Consumers: rule `rm_tracked`'s deletion arm (delta 2) and rule `find_exec` (delta 3), by call. Each face has a reader: the spelling both steers, the value both rules' tests, the live text delta 2's literal-or-glob split.
- **Rosters holding a view name.** None is minted, so the reader's view table, the `--guard-json` `view` mode and the declaration grammar are untouched. `check-guard-registration`'s assertion C holds delta 3's added `raw` in lockstep between the item and the table row.
- **Predicate callers of rule `rm_tracked`'s test**, by `grep -n rm_tracked_reach native/src/guard/rules/*.rs`: rule `bounded_wait`'s arm (B) and rule `bounded_write`. Each runs the test under this rule's declaration and narrows as delta 2 states.
- **Widening, not narrowing.** Delta 2 widens a block, and delta 3 changes printed text and, through its value-read tests, the verdict of a few exotic spellings: `-exec cat '{}' '\;'` is steered today (run through the member on this tree) and falls through after, since the shell passes `\;` literally, and `-exec \cat '{}' \;` falls through today (run) and is steered after. The readers that red on the change are the decision tables, in both directions, and delta 4 names their rows.

**Inferred, cannot run before build:** the two verdicts after the change — the value read they rest on does not exist until build.

## Existing sections updated

- `guard-kit/SPEC.md` §The reader and its views — the word read, added after the pairing sentence, which stays, and the seam enumeration naming it (delta 1).
- `guard-kit/SPEC.md` §The rule roster — the `rm_tracked` item's operand reading, steer and limits, including its closing sentence that the steer and the force arm are unchanged, which no longer holds of the PowerShell steer (delta 2); the `find_exec` item's declaration, terminator sentence and steer (delta 3).
- `guard-kit/SPEC.md` §Testing — the sandbox's third precondition (delta 4).
- `native/src/guard/reader.rs`, `native/src/guard/bash.rs`, `native/src/guard/powershell.rs`, `native/src/guard/engine.rs` — the seam method, its two implementations and the context's read (delta 1); `native/src/guard/text.rs` — `held_words` and its spec binding, kept (delta 1).
- `native/src/guard/rules/reach.rs` — the deletion arm and its steer (delta 2).
- `native/src/guard/rules/tools.rs`, `native/src/guard/rules/mod.rs` — the steer, the terminator test and the table row's views (delta 3).
- `native/src/emit/run_guard_tests.rs` — the sandbox's second tracked file (delta 4).
- `guard-kit/guard-tests/cases.tsv`, `guard-kit/guard-tests/powershell-cases.tsv` — the rows (delta 4).
- `docs/guard-kit/SPEC.md` — generated, stale once any delta lands (all deltas).
- `.workflow/release-declarations.md` — a bullet for the widened block and the changed steer text (deltas 2 and 3).

## Retired spellings

- None — no delta removes a name another surface carries; whether the steer's re-quoting helper survives delta 3 is build's.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls guard-kit/SPEC-*.md`), discharged at the iteration.
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Entry moved** — the queue entry moves to Done in the merging commit, ahead of the drain stage.
