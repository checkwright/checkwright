# SPEC amendment: escaped-blank

**The bash dequoted view holds a backslash-escaped blank or tab as one word byte, as it already holds a quoted span's blanks, and the readers pairing the skeleton's words with the view's read the pair as one word.** Today `dequoted` in `native/src/guard/bash.rs` copies a backslash and its next byte unchanged, so the view of `rm -rf .tmp/x\ y` carries a real blank and the shared word split cuts where the shell does not. Under a `Bash(rm -rf .tmp/*)` grant, rule `grant_path_slot` reads `y` as a second operand outside the slot and blocks, though the shell passes one word, `.tmp/x y`, inside it.

**Run at authoring**, `bash gate-sdk/bin/run-gates.sh --hook shell-guard` on a payload under this repo's grant `Bash(rm -rf .tmp/*)`: `rm -rf .tmp/x\ y`, `rm -rf .tmp/x\ y scratch.txt` and `rm -rf .tmp/x\ y; make build` each block naming `y` as a second operand outside the slot. The tree does not satisfy the fix.

**The safety ground is the shell's reading, not the harness's.** The shell hands the program one word for `x\ y`, and every grant this unit moves is decided on the guard's own reading of the operands. The allow match the grants also run reads the segment's restored text, which delta 1 leaves unchanged, so no harness-side claim is added.

## What changes

### (1) The dequoted view holds an escaped blank as one word byte {design-bearing} {user-facing: grant-slot-escaped-blank's deliverable, the dequoted view holding an escaped blank so the false block on a slot operand ends}

`native/src/guard/bash.rs`, `dequoted`'s backslash arm. It still copies the pair, but a blank or tab after the backslash becomes its sentinel: `\ ` is the backslash then `0x01`, and a backslash then a tab is the backslash then `0x02`, the bytes a quoted span's blanks already take, so `unsentinel` restores them. The backslash stays in the view, and every reader that strips backslashes does so before it restores sentinels. The arm consumes the pair, so `x\\ y` (an even run) is a literal backslash then a real break. A backslash before a newline and the escaped separators `\;`, `\|` and `\&` are untouched, since the splits' escape test owns them (§The reader and its views). The skeleton is unchanged, so it and the view now differ in word count where an escaped blank occurs; delta 2 closes that. The function's `spec:` pointer moves to §The reader and its views, where delta 4 states the view.

The module's tests: `dequoted("rm x\\ y")` holds `0x01` after the backslash; `dequoted("rm x\\\\ y")` is unchanged; a backslash-tab holds `0x02`; `dequoted("rm 'x\\ y'")` is as today.

**Which verdicts move, and which way.** The view's readers, by `grep -rn "dequoted(" native/src/guard` (no reader outside `native/src/guard`): `rules/grants.rs` (`slot_reach` behind rule `grant_path_slot` and its predicate uses, `rewrite_granted`, and rule `bounded_write`), `rules/mod.rs` (the declared-forms reader), `rules/liveness.rs` (rule `bounded_wait`'s recorded launch), `rules/tools.rs` (rule `sed_file`'s awk, stream and python arms, rule `find_exec`) and `rules/reach.rs` (rule `commit_only_paths`, rule `rm_tracked`'s `git rm` arm, rule `worktree_confinement`'s path words). An unquoted `\ ` now reads as one word where it read as two:

- **Grants more, as the shell reads it:** rule `grant_path_slot`'s false block ends, and its predicate turns clean behind rules `abs_script`, `brace_glyph`, `bounded_wait` arm (B) and `bounded_write`; rule `bounded_write` grants `touch .tmp/a\ b`, one bounded target; the declared-forms reader no longer reads an option-shaped fragment after an escaped blank as an option, since the shell passes it inside a word.
- **Blocks more, as the shell reads it:** rule `commit_only_paths` reads `git commit -m fix\ it` as `-m` consuming `fix it`, a pathless commit, as `-m 'fix it'` is; rule `sed_file`'s awk read arm (`awk_read`, which steers only a program with exactly one operand) reads `awk 'NR==2' my\ file` as one file and steers it to the Read tool, where it read two operands and passed; and its python arm reads an unquoted `-c` program carrying an escaped blank whole, where it read the fragment before the blank.
- **Unmoved:** rule `rm_tracked`'s `rm` arm and rule `sed_file`'s main arm read the skeleton; rule `sed_file`'s stream arm passes any segment with an operand, so one operand or two decide the same; rule `find_exec` still blocks, its steer text aside; rule `worktree_confinement` roots a path word by its head, which an escaped blank inside the word cannot forge.

The decision table is the oracle: build runs `--run-guard-tests` and gives every moved row a cause before it lands. A row that moved for no cause is a defect the change exposed, fixed in this unit and never re-expected.

### (2) The word-aligning readers join a skeleton word ended by an escaped blank {design-bearing} {user-facing: grant-slot-escaped-blank's deliverable, the reader following the held blank so an escaped-blank operand is aligned rather than withheld}

`write_words` and `unredirected_words` (`native/src/guard/rules/`) walk the skeleton's words and the view's by index and refuse on a count mismatch. After delta 1 that refusal would make rules `bounded_write` and `bounded_wait`'s recorded launch withhold a command they grant today, such as `touch .tmp/a\ .tmp/b`. So a helper beside `words` in `native/src/guard/text.rs` joins a skeleton word ending in an odd backslash run to the next word when the byte between them is a blank or tab, the pair delta 1 holds as one, and both readers call it. The declared-forms reader's positional counter stays on the plain skeleton words, an over-count that withholds and never grants.

The helper's tests: `a\ b c` is two words; `a\\ b` is two; `a\ ` at the end is one; a backslash before a newline joins nothing. `write_words` over `touch .tmp/a\ b` returns the one target `.tmp/a b`.

### (3) Decision-table cases {mechanical}

`guard-kit/guard-tests/cases.tsv`, under delta 1's oracle rule.

- Rule `bounded_write`'s section, beside `allow	rm -rf .tmp/x .tmp/y`: `allow	rm -rf .tmp/x\ y`, one operand inside the slot, which blocks today; and `allow	touch .tmp/a\ b`, one bounded target.
- Rule `grant_path_slot`'s section, beside `block	rm -rf .tmp/x scratch.txt`: `block	rm -rf .tmp/x\ y scratch.txt`, the escaped-blank operand inside the slot and a real second operand outside it, its cause moving from `y` to `scratch.txt`; and `block	rm -rf .tmp/x\\ y`, an even run then a real blank.
- Rule `commit_only_paths`'s section: `block	git add tracked.md && git commit -m fix\ it`, a pathless commit.

**Inferred, cannot run before build:** the two `allow` rows' and the `commit_only_paths` row's verdicts — deltas 1 and 2, which decide them, land in build.

### (4) §The reader and its views states the held blank {mechanical}

guard-kit/SPEC.md. **Not yet applied.**

In §The reader and its views, the **A view** paragraph's second sentence becomes:

> There are two derived views: `body`, a heredoc body extraction, and `dequoted`, the content-kept view walked in lockstep with the `sq dq hd` skeleton, which drops a quoted span's quote characters and holds its blanks, tabs, `;`, `|` and `&` as sentinel bytes no word split cuts at; a backslash-escaped blank or tab outside a span is held the same way, so `x\ y` is one word and `x\\ y` two, and a reader pairing the skeleton's words with it joins a skeleton word ending in an odd backslash run to the next.

In §The generic ruleset's rule roster, two sentences gain the escaped blank: rule `sed_file`'s *It removes the quote characters and holds each quoted span's blanks and statement separators as sentinels* becomes *… and statement separators, and each backslash-escaped blank, as sentinels*; and rule `grant_path_slot`'s step (1) *so a quoted `>` is an operand and a quoted blank stays inside its word* becomes *… and a quoted or backslash-escaped blank stays inside its word*. The act depends on the sibling debt unit `guard-kit-ruleset-brevity`, which rewrites that roster and is ordered after this unit:

- **Brevity not yet landed:** edit both sentences in place.
- **Brevity landed first:** find each sentence by its fact (the dequoted view's sentinels in rule `sed_file`'s entry; a quoted blank staying inside its word in rule `grant_path_slot`'s step (1)) and make the same one-clause change; where brevity cut the restatement in favour of a pointer to §The reader and its views, make no roster edit.

### (5) The site mirror follows {mechanical}

`docs/guard-kit/SPEC.md` is regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` in the commit landing delta 4.

### (6) The release declaration {mechanical}

`.workflow/release-declarations.md`, under Behavior changes, in the commit landing delta 1. **Not yet applied.**

> - **guard-kit shell guard, the bash reader's dequoted view** — a blank or tab after an odd run of backslashes stays inside its word, as the shell reads it, so a slot operand spelled `x\ y` is one operand and rule `grant_path_slot` no longer blocks it, rule `bounded_write` grants a bounded target so spelled, rule `commit_only_paths` reads `git commit -m fix\ it` as a pathless commit, and rule `sed_file` steers an `awk` line-range read of a file so spelled to the Read tool and reads an inline python program so spelled whole. Nothing to do.

## Producers and consumers

- **The held escaped blank.** Producer: `dequoted`, on every bash command whose view a rule declares. Enabling configuration: none, since the reader runs wherever the shell-guard member is wired (this repo's `.claude/settings.json`, and every adopter merging guard-kit's settings template). Consumers: the readers delta 1 lists, each through the engine's context. Red condition per reader (point 5): one comparing a word to text now meets `0x01` inside a word where it met two words; each either restores sentinels before comparing (`unsentinel`, or the sentinel replacement in `slot_segment` and `slot_capture`) or tests only a word's head or flag shape, which a held blank inside a word cannot forge.
- **The joined skeleton word.** Producer: delta 2's helper. Consumers: `write_words` and `unredirected_words`, and through the latter `slot_segment` and `recorded_launch`. No field is added.
- **Unchanged:** the PowerShell reader, whose escape is the backtick and whose dequoted view has its own contract. No state, knob, event or interface is added.

## Existing sections updated

Roster by `grep -rn "dequoted(" native/src/guard`, `grep -rn "words(" native/src/guard` and `grep -n "dequoted\|sentinel" guard-kit/SPEC.md` over the tracked tree.

- `native/src/guard/bash.rs` — `dequoted`, its `spec:` pointer and its tests (delta 1).
- `native/src/guard/text.rs` and `native/src/guard/rules/` — the joining helper and its two callers (delta 2).
- `guard-kit/guard-tests/cases.tsv` — rules `bounded_write`'s, `grant_path_slot`'s and `commit_only_paths`'s sections (delta 3).
- `guard-kit/SPEC.md` — §The reader and its views, and rules `sed_file`'s and `grant_path_slot`'s roster sentences (delta 4).
- `docs/guard-kit/SPEC.md` — the regenerated mirror (delta 5).
- `.workflow/release-declarations.md` — one Behavior changes bullet (delta 6).

## Retired spellings

- None — the change adds a held byte class to the dequoted view; it retires no name.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — no template or agent definition is touched.
- [ ] **Merged with no information lost** — §The reader and its views states the dequoted view whole, and the roster sentences agree with it.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls guard-kit/SPEC-*.md`), discharged at the iteration while a sibling guard-kit amendment is in flight.
- [ ] **Removals propagated** — `check-amendment-retired-spelling` green over the declaration above.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks.
- [ ] **Run green** — `bash gate-sdk/bin/build-native.sh`, `--run-guard-tests`, the full battery and guard-kit's fixture suite in the merging batch.
- [ ] **The entry is done** — `grant-slot-escaped-blank` moves to Done in the merging commit, before the drain stage and before `guard-kit-ruleset-brevity` is applied.
