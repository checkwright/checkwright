# SPEC amendment: placeholder-mark

The skeleton replaces each inert region with a placeholder, and today a placeholder is two plain letters: `SQ`, `DQ` or `HD`. Command text can spell those letters, so every rule that asks *does this carry a placeholder* by testing for the letters also fires on a real path or value that happens to hold them.

**Measured at authoring (2026-09-24, at `d3046960`)**, by piping payloads into the built member with `GUARD_KIT_LOG` pointed at scratch:

- `bash gate-sdk/bin/run-gates.sh --run > .tmp/ab.log 2>&1 & echo "pid=$! run=k" > .tmp/k.run; wait; rm -f .tmp/k.run` is granted by rule `bounded_wait`'s arm (B).
- The same command with the target `.tmp/DQ.log` falls through with no output. The arm read the literal letters as a quoted span and declined.

**The placeholder test sites**, from `git grep -n '"SQ"\|"DQ"\|"HD"\|b"SQ"\|b"DQ"\|b"HD' native/src` at `d3046960`:

- `native/src/guard/rules/liveness.rs:338`, rule `bounded_wait`'s arm (B), a launch redirect target carrying a placeholder;
- `native/src/guard/rules/liveness.rs:209`, the same arm matching the record argument's `DQ` in the skeleton;
- `native/src/guard/rules/spelling.rs:98`, rule `git_c_root`'s arm (d), a knob value carrying a placeholder;
- `native/src/guard/rules/grants.rs:70` and `native/src/guard/bash.rs:128`, a heredoc residue line that is exactly `HD`;
- `native/src/guard/bash.rs` `scan` (the three emit sites) and `dequoted` (the lockstep's two placeholder reads), the reader itself.

`native/src/emit/scan_prompts.rs` `quoted_view` also writes `SQ` and `DQ`, but nothing tests for them. It collapses a quoted span to one word so the ranker's allow match cannot split it. It is not a placeholder test, and this amendment leaves it alone.

**The ruling: a mark no command can carry, not a list of positions.** The queue entry named two sound fixes. One maps placeholders by position, with the skeleton handing back its spans. The other uses a token no command text can spell. This amendment takes the token, for three reasons:

- Every site above receives a substring that a split has already cut out of the skeleton. A span list would have to follow each one through the split that produced it, and every split in the reader loses offsets.
- With the token, each site keeps a one-line substring test.
- The PowerShell reader (`guard-powershell-tool-unguarded`, SPEC-powershell-reader.md) emits the same placeholders through the same constants, so it inherits the fix without adopting any machinery.

A token no command can spell requires the reader to refuse any command that does spell it, and delta 2 is that refusal.

## What changes

### (1) A placeholder is a NUL mark and two letters {design-bearing}

**Not yet applied.** Each skeleton placeholder is the byte NUL followed by the class letters: `\0SQ`, `\0DQ` and `\0HD`. The bash reader holds the three as named constants in `native/src/guard/reader.rs`, beside `View`, and `bash.rs` emits them in `scan`, reads them in `dequoted`'s lockstep, and compares its residue line against the `HD` one. Every rule site in the roster above tests the constant instead of the letters. So `liveness.rs:338` asks whether the target contains either quoted-span mark, `liveness.rs:209` matches the `DQ` mark, `spelling.rs:98` skips a value carrying a mark, and `grants.rs:70` compares against the `HD` mark. The mark contains no blank, so the skeleton keeps its token count, and "placeholder, never deletion" holds unchanged.

In guard-kit/SPEC.md §The reader and its views, the skeleton paragraph's "replaced by a placeholder token" becomes:

> replaced by a placeholder: the byte NUL followed by the class's two letters (`SQ`, `DQ` or `HD`). No command the member reads carries a NUL (§The shell guard), so a test for a placeholder cannot fire on command text, and a path or value that spells the letters is read as written. The mark never leaves the process: every printed form drops it (§The shell guard, the verdicts; §Consumer rules, `view`; §scan-prompts, the ranking key), so what a reader outside the member sees is the two letters it always saw.

### (2) A command carrying a control byte blocks {design-bearing}

**Not yet applied.** After the reader is selected and before the first rule runs, a command carrying a C0 control byte other than tab, line feed and carriage return is blocked. The block message names the byte in hex and gives the correction: write it as an escape the shell expands (`printf '\x<hh>'`), or remove it. No rule runs, and no fall-through line is written. `--guard-json view` prints nothing for such a command.

This one refusal covers both kinds of internal mark the reader writes into its views:

- the placeholder mark from delta 1;
- the separator sentinels the dequoted view already writes for a quoted span's blank, tab, `;`, `|` and `&` (bytes `0x01` to `0x05`), which a raw command carrying those bytes could forge today.

**Why a block and not the fail-open posture.** An unmodelled command defaults to the harness's own path (§The fail-open postures). That ground is the price of guessing: a fail-open answer costs nothing when the guard cannot read a command. Here the byte is never needed, because every shell spells it by escape. Passing would let one byte switch off every block rule, rule `git_mutation_under_producer` included. Blocking costs one re-issue and wedges nothing. Carriage return is allowed because a CR LF inside a command is read verbatim on every host (§The hook on native Windows).

In guard-kit/SPEC.md §The shell guard, **The call** gains after the sentence about how the rules read the command:

> A command carrying a C0 control byte other than tab, line feed or carriage return is blocked before any rule runs, with the byte named in hex and the escape that spells it. No shell needs the raw byte, and the reader's own marks are drawn from that range (§The reader and its views), so passing it would let the byte forge one.

In §The fail-open postures, "The refused knob read above is this member's one block that decides no rule, on the ground stated there" becomes "The refused knob read and the control-byte refusal (§The shell guard) are this member's two blocks that decide no rule, each on the ground stated there".

### (3) The printed forms drop the mark {mechanical}

**Not yet applied.** The mark is removed at the three places a skeleton-derived string leaves the process:

- the verdict, once, where `native/src/hook/shell_guard.rs` renders the engine's `Verdict`, for block, advise, allow and rewrite text alike;
- `--guard-json view`'s print (`native/src/guard/mod.rs` `json_view`);
- the `scan-prompts` ranking key (`native/src/emit/scan_prompts.rs` `ranking_key`), whose words are read off the skeleton.

Removing only the mark leaves the two letters, so every printed form is byte-identical to today's. In guard-kit/SPEC.md §Consumer rules, `view`'s bullet gains: "Each placeholder prints as its two letters."

### (4) The suites stop redrawing their sandbox {mechanical}

**Not yet applied.** Three suites redraw a `mktemp -d` name until it spells neither letter pair, because the defect otherwise reds them on a random draw. The loop and its comment go from each: `guard-kit/gate-tests/guard-config-knobs.test.sh`, `guard-kit/gate-tests/worktree-confinement.test.sh` and `guard-kit/gate-tests/consumer-rules.test.sh`. The fix gains rows that pin it:

- `guard-kit/guard-tests/cases.tsv`, under rule `bounded_wait`'s arm (B), gains a granted canonical launch whose launch target spells `DQ` (for example `.tmp/DQ.log`). Beside it stays a declining launch whose target is quoted, so its target is a real placeholder.
- `guard-config-knobs.test.sh` gains an arm (d) case whose sandbox path spells `SQ`, steered like the existing same-value case.
- `consumer-rules.test.sh` keeps its `view` rows unchanged, which pin the printed spelling. It gains a payload whose command carries a `0x01` byte: the member blocks it naming the byte, and `view` prints nothing.
- The unit tests in `bash.rs` and `scan_prompts.rs` that assert a skeleton or key text compare against the constants or the rendered form.

§Testing's decision-table paragraph needs no edit. Every existing row keeps its verdict, because no row spells the letters. That was checked with `grep -n 'SQ\|DQ\|HD' guard-kit/guard-tests/*.tsv` at `d3046960`, which found nothing.

## Producers and consumers

- **The placeholder mark.**
  - Producer: the bash reader's `scan`, on every skeleton view.
  - Consumers: the rule sites in the roster above, by substring test against the constants; `dequoted`'s lockstep; the residue split.
  - Named reader to keep in step: the PowerShell reader, which spells its placeholders through the same constants (see that unit's amendment).
- **The control-byte block.**
  - Producer: the member, on every payload whose tool selects a reader.
  - Consumer: the session, through the block message.
- **Roster-holding readers.** `check-guard-registration` holds names, order and declared views, and neither delta adds a rule or a view. The block is no rule, as the knob refusal is none.
- **Point 5.** No corpus narrows. Delta 1 turns declines into grants where a path spells the letters. That flips no existing row: the grep above shows no row spells them.
- **Point 6.** Not obliged.

## Existing sections updated

Roster from `grep -n "placeholder token\|one block that decides no rule\|prints the named view" guard-kit/SPEC.md` and the site grep above, run 2026-09-24.

- `guard-kit/SPEC.md` §The reader and its views, the skeleton paragraph (delta 1).
- `guard-kit/SPEC.md` §The shell guard, **The call**, and §The fail-open postures (delta 2).
- `guard-kit/SPEC.md` §Consumer rules, the `view` bullet (delta 3).
- `native/src/guard/reader.rs`, `native/src/guard/bash.rs`, `native/src/guard/rules/liveness.rs`, `native/src/guard/rules/spelling.rs`, `native/src/guard/rules/grants.rs` (delta 1).
- `native/src/hook/shell_guard.rs` (deltas 2 and 3).
- `native/src/guard/mod.rs`, `native/src/emit/scan_prompts.rs` (delta 3).
- `guard-kit/guard-tests/cases.tsv`, `guard-kit/gate-tests/guard-config-knobs.test.sh`, `guard-kit/gate-tests/worktree-confinement.test.sh`, `guard-kit/gate-tests/consumer-rules.test.sh` (delta 4).
- `.workflow/release-declarations.md`, one Behavior changes bullet (deltas 1 and 2): `**--hook shell-guard**` now grants a recorded launch whose target, and steers a knob echo whose value, spells `SQ` or `DQ`, where it declined; and it blocks a command carrying a raw control byte other than tab, line feed or carriage return, naming the escape that spells it.
<!-- update-target-exempt: generated mirror, regenerated by its freshness gate's printed command -->
- `docs/guard-kit/SPEC.md`.

## Retired spellings

- None — the printed spelling is kept by delta 3, and no name is renamed or deleted.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The causal-completeness check holds for the mark and the block.
- [ ] **Instruction surfaces: instruction only.** Not reached.
- [ ] **Merged with no information lost.** Each SPEC edit re-phrases the passage it refines.
- [ ] **Amendment deleted.** This file is removed on merge (`ls guard-kit/SPEC-*.md`).
- [ ] **Entry moved.** `guard-placeholder-letter-paths` moves to Done in the merge commit, at a stage before the drain stage.
- [ ] **Removals propagated.** Not reached.
- [ ] **Gaps filed.** Any cross-component gap found during the work is filed to the gap inbox.
