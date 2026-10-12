# SPEC amendment: variant-lane

The decision tables pin the spellings their author wrote. A spelling the shell reads as the same command as a row, with a word quoted or escaped, a redirect glued or parted, or `command` ahead of the command word, is asserted nowhere until a reader outside the session writes it. This amendment gives `--run-guard-tests` a lane that derives those spellings from the tables' own rows and holds each to its row.

A prototype run at authoring sized it. An out-of-crate deriver over this tree's two bash tables, each derived table fed to the arm as a positional, produced one derived row per site and transform and failed the relation below under the sections of twenty-one rules. Nearly every failure is one shape: a rule reads its command word, option or operand off the skeleton, so the re-spelling falls through. The prototype's deriver is not the crate's, so its roster is **inferred** for the lane this amendment builds, and delta 5 has the build run the real one.

## What changes

### (1) The variant lane and its relation

`--run-guard-tests` derives re-spellings of the two bash tables' rows after the kit's tables and ahead of the consumer lane, and fails on a derived row the guard reads more permissively than its base row {design-bearing} {user-facing: the entry's Deliverable — rows derived from each table row by a declared set of shell-equivalent re-spellings, each held to the row unless a limit is declared; the arm's summary line gains the lane's counts and its exit 1 a variant finding}.

- **The subject is the member, unchanged.** A derived row is fed as its base row is: the same payload, the same sandbox, the base row's backgrounding flag where its table carries one, and the two substitutions applied to the derived cell.
- **The relation is one-sided: the guard is at least as strict on a re-spelling as on its row.** By the base row's verdict:
  - `block` holds on `block`;
  - `advise` holds on `advise` or `block`;
  - `fallthrough` holds on `fallthrough` or `block`;
  - `allow` and `rewrite` hold on `allow`, `rewrite`, `fallthrough` or `block`.

  Any other class, an `advise` its base does not carry or the ladder's `exit<rc>` and `unknown`, fails, and no limit admits it: an `advise` out of nowhere is the view fault's class.
- **Why not the row's own verdict.** A grant declines on a spelling it does not model, the ruleset's stated direction, so equality would red every grant row under every quote transform for a decline that costs one permission decision. A steer that blocks a re-spelling of a sanctioned form costs one re-issue in the plain spelling. The two classes the lane exists for are the block a re-spelling evades and the grant no row holds, and the relation is exactly those.
- **A derived row equal to an authored row of the same table and flag is skipped**: the authored row decides it.
- **A failing derived row prints** its transform, its base row's section, the derived command, the base verdict and the verdict it got, in the kit lane's mismatch form. The summary line adds the lane's derived-row count and its limit-row count.
- **The limits table is a fifth positional**, defaulting to `guard-tests/variant-limits.tsv` under the kit root as the four tables default. A call naming its own tables names its own limits.
- **No knob is minted.** The lane reads the kit's tables against the kit's rules, so it is the kit's self-test and no consumer calibration.

Replacement for §Testing's opening paragraph's last sentence, *The runner has two lanes…*. **Not yet applied.**

> The runner has three lanes: the kit's tables against the member, the variant lane, and the consumer lane, each below.

New §Testing passage, placed after *The sandbox's five preconditions* and its allowlist note. **Not yet applied.**

> **The variant lane** holds each bash row to the spellings the shell reads as the same command. For every eligible row of `cases.tsv` and `background-cases.tsv` it derives one re-spelling per site and transform, feeds each through the member as the row itself is fed, and fails where the guard reads the re-spelling more permissively than the row:
>
> - a `block` row's re-spelling blocks;
> - an `advise` row's advises or blocks;
> - a `fallthrough` row's falls through or blocks;
> - an `allow` or `rewrite` row's takes any of the four.
>
> Any other class fails, an `advise` its row does not carry among them, and no limit admits it. The relation is one-sided because a grant declines on a spelling it does not model (§The generic ruleset, the shared directions): equality would red every grant row for a decline that costs one permission decision. A re-spelling equal to an authored row of the same table and flag is skipped.

### (2) The deriver: eligible rows, sites and transforms

The deriver is a lexer of its own over blanks and one character class, and it respells only what it can respell without a shell {design-bearing}.

- **It shares no function with the bash reader.** A transform derived through the reader under test inherits that reader's misreads, and the two then agree on them.
- **An eligible row** is one whose command cell, before substitution, carries none of: a quote, a backslash, `$`, a backtick, `#`, a parenthesis, a brace, `[[`, `<<`, or `@NL@`. Any other row is underived, since each of those makes a blank-split token something other than a word.
- **A token** is a maximal run of non-blank characters. **A site** is a token's plain head: its longest leading run of letters, digits and `_ . / : % + , -`, where all of these hold:
  - the head is at least two characters and is no reserved word (`if`, `then`, `else`, `elif`, `fi`, `for`, `while`, `until`, `do`, `done`, `case`, `esac`, `in`, `function`, `select`, `time`, `coproc`);
  - the rest of the token is empty, or opens with `;`, `|`, `&`, `<` or `>`;
  - the head is not all digits where the rest opens with `<` or `>`, since it is then a file descriptor;
  - the token does not follow `for`, `select`, `case` or `function`, whose next word is a name the shell does not dequote.

  A word carrying `@`, the substitution tokens' character, is no site, so no verdict turns on how a host spells its temporary directory.
- **A site is a `lead` or an `arg`.** It is a `lead` where its token opens a statement: the row's first token, or the token after one that is or ends in `;`, `&&`, `||`, `|`, `|&` or `&`, or after `if`, `then`, `else`, `elif`, `while`, `until`, `do`, `!` or `time`. Every other site is an `arg`.
- **The quote transforms**, each derived once per site and named by its form and the site's kind (`sq-lead`, `sq-arg`, and so on):
  - `sq`, the head single-quoted;
  - `dq`, the head double-quoted;
  - `esc`, a backslash ahead of the head's first character;
  - `part`, the head's second half single-quoted, cut at half its length rounded down.
- **The spacing transforms**, each derived once per occurrence, a redirect operator being `<`, `>` or `>>` with an optional descriptor ahead of it and no `&` on either side:
  - `redirect-tight`, the blanks between a redirect operator and its target removed;
  - `redirect-apart`, one blank put between a redirect operator and a target glued to it;
  - `dup-apart`, one blank put between `>&` and the descriptor glued to it;
  - `word-glued`, the blanks between a word and the redirect operator after it removed, where the word does not end in a digit;
  - `separator-tight`, the blanks on both sides of `;`, `&&`, `||`, `|` or `|&` removed.
- **`wrap-command`**, derived once per row: `command` and a blank ahead of the row, where the row's first token is a `lead` site.
- **Why `command` and no other wrapper.** It is the one wrapper that is both the shell's own spelling of the same command and one the permission matcher strips (§The reader and its views, the harness view), so a grant for the bare command reaches through it with no decision. `env`, `sudo` and their kin run a second program and are a program reach, no re-spelling.
- **What the set leaves to review.** Two of the recorded finding classes are no re-spelling and are derived by nothing here: a decoration, an inert redirect appended or a banner put ahead, which changes the command and moves a verdict by design (`cat <file> 2>/dev/null` is a pipeline the band grants); and a keyword standing where no command stands, which is a rename. Their rows are authored.
- **The deriver's own tests** pin each transform and each exclusion on a literal, in the module's `#[cfg(test)]` tests.

New §Testing passage, following delta 1's. **Not yet applied.**

> **The deriver is a lexer of its own**, over blanks and one character class, sharing no function with the bash reader: a transform derived through the reader under test would inherit its misreads. A row is eligible where its cell, before substitution, carries no quote, backslash, `$`, backtick, `#`, parenthesis, brace, `[[`, `<<` or `@NL@`. A **site** is a blank-split token's plain head, its leading run of letters, digits and `_ . / : % + , -`, at least two characters long, no reserved word and no name after `for`, `select`, `case` or `function`, with nothing after it in the token but a separator or a redirect, and never a file descriptor. A site is a `lead` where its token opens a statement and an `arg` elsewhere. The transforms:
>
> - per site, named with the site's kind: `sq` and `dq` quote the head, `esc` puts a backslash ahead of it, and `part` single-quotes its second half;
> - per occurrence: `redirect-tight` and `redirect-apart` remove or add the blank between a redirect operator and its target, `dup-apart` parts `>&` from its descriptor, `word-glued` glues a word to the redirect operator after it, and `separator-tight` removes the blanks around a statement separator or a pipe;
> - per row: `wrap-command` leads the row with `command`, the one wrapper that is both the shell's spelling of the same command and one the matcher strips.
>
> **Its honest limits.** No shell runs a derived row, so the equivalence is the construction's and the deriver's own tests pin it. A row carrying a quote or an escape is underived. A decoration and a keyword standing as a word change the command and are no re-spelling, so their rows are authored. The PowerShell tables and the consumer lane are underived.

### (3) The limits table

A derived row that fails the relation is green only under a limit row naming its base row's rule section and its transform, and a limit row nothing needs is itself a failure {design-bearing}.

- **`guard-tests/variant-limits.tsv`**, in the table grammar: `<floor> <TAB> <rule> <TAB> <transform>`.
  - `<floor>` is `fallthrough` or `allow`, the most permissive verdict the limit admits. Under `fallthrough` a failing re-spelling may fall through. Under `allow` it may also be granted, `rewrite` included.
  - `<rule>` is a rule name that opens a section in either bash table.
  - `<transform>` is one of delta 2's names.
- **A section** opens at a comment line beginning `# rule` and a backticked rule name, and runs to the next such line. Sections of one name are one section, across both tables. A row ahead of the first section belongs to none, so its derived rows take no limit.
- **Every field has a reader.** The lane reads `<rule>` and `<transform>` to find the limit for a failing derived row, and `<floor>` to decide whether the verdict it got is admitted.
- **The table is held from both sides**, each at exit 1:
  - a failing derived row with no limit row, or one whose verdict is below its limit's floor;
  - a limit row under which no derived row fails;
  - a limit row whose floor is `allow` where no failing derived row under it is granted, which is tightened to `fallthrough`.
- **A table the lane cannot read is exit 2**, the harness-precondition code: a missing file, a floor or a transform off its set, or two rows naming one rule and transform. A rule naming no section is the stale case above.
- **Why the floor.** A limit that admits a grant is a wider statement than one that admits a pass, and a `fallthrough` row must not come to cover a re-spelling that a later edit starts granting.
- **Why a limit is as wide as a section and no narrower.** The rule is the unit a contract is written in: an item says its words are read by value, or it does not. A limit per derived row would be a second table several times the size of the first, generated and unreasoned. **Its honest limit:** under a limit row, a second row of the same section may start failing the same transform unseen.
- **Refused: a marker on the base row.** The decision cell is the table grammar's first field, and a second meaning in it would be read by every table's reader.

New §Testing passage, following delta 2's. **Not yet applied.**

> **A limit is declared, and held from both sides.** `guard-tests/variant-limits.tsv` (`<floor> <TAB> <rule> <TAB> <transform>`) names the re-spellings a rule's read does not reach. A section of a bash table opens at a comment line beginning `# rule` and a backticked rule name and runs to the next, and a failing re-spelling is green only under a limit row naming its row's section and its transform. The floor is the most permissive verdict the limit admits: `fallthrough`, or `allow` where the re-spelling is granted. The lane fails on a re-spelling no limit admits, on a limit row under which nothing fails, and on an `allow` floor nothing reaches, so lifting a rule's read deletes its rows in the same commit. A table it cannot read, a floor or transform off its set or a pair named twice is exit 2. **Its honest limit:** a limit is as wide as its section, so a second row of that section may start failing the same transform unseen.

### (4) The ruleset states what a rule's read reaches

§The generic ruleset's shared directions gain the direction the limits table is the roster of {design-bearing}.

- **It is the statement a review finding of this shape is answered by.** A re-spelling a rule does not read is a declared limit with a row, or a defect with none, and the limits table tells which.
- **The pass is not uniformly loud, and the text says where.** A `command`-led spelling is one the matcher strips, so a grant for the bare command reaches it with no decision. Whether the matcher matches a quoted or escaped command word against a grant is unmeasured, so the text claims nothing for it.

New bullet under *The shared directions*, after *A decline biases toward passing*. **Not yet applied.**

> - **A rule reads a word as its item says.** An item that reads words by their value says so. Any other rule reads the skeleton's word, so a command word, an option or an operand spelled with a quote or an escape, glued to a redirect, or led by `command` is a construct its walk does not model, and it declines. Each pair of rule and re-spelling that stands so is a row of the variant lane's limits table (§Testing), which holds the roster, and lifting a rule's read deletes its rows. The decline is silent under `command`, which the matcher strips (§The reader and its views), so a grant for the bare command reaches the call. A quoted or escaped command word is unmeasured at the matcher.

### (5) The first limits table

The build runs the lane over the tables as they stand and writes one limit row per failing pair of section and transform, at the floor the failing rows reach {mechanical}.

- **It is written before any sibling unit lands**, the lane being built first, so its rows include the pairs those units lift.
  - Where `git-rules-word-read-siblings` lands in this iteration, its commit deletes the limit rows its lifted read makes stale, under rules `git_grep`, `git_rewrite`, `git_stash` and `commit_only_paths` and wherever else the lane then names one. Where it does not, those rows stay.
  - Where `git-stash-glued-redirect-value` lands, its commit deletes rule `git_stash`'s `dup-apart` row and the quote rows its repaired cut clears. Where it does not, they stay.
- **A grant-reaching pair takes the `allow` floor**, and the prototype met two: a quoted or escaped option under rule `cat_file`'s section, which the band then grants as a read, and an escaped `-f` under rule `sed_file`'s, which grants an `awk` program file. The second is filed in the gap inbox as a defect. Where its fix lands in this iteration the row is never written, and where it does not the row is written at `allow`.
- **No rule's read is lifted by this delta.** A failing pair is declared, not fixed, whatever its rule.
- **The prototype's roster is the estimate, not the oracle**: it failed under rules `abs_prefix`, `abs_script`, `allowlist_chain`, `background_no_record`, `bare_sleep`, `cat_file`, `cd_compound`, `commit_only_paths`, `emitter_write`, `find_glob`, `git_c_root`, `git_grep`, `git_rewrite`, `git_stash`, `grant_path_slot`, `pgrep_self_match`, `rm_tracked`, `ro_pipeline`, `scratch_redirect`, `script_interpreter` and `sed_file`, and under no section for `separator-tight`, `redirect-tight` and `redirect-apart`. It respelled `@`-bearing words, which delta 2 excludes, so the crate's roster is narrower there.

### (6) The surfaces that name the arm

The arm's readers and describers take the third lane {mechanical}.

- `guard-kit/gate-tests/consumer-rules.test.sh` calls the arm with positional tables. It passes an empty limits table as the fifth, and its expected summary lines take the lane's counts.
- `guard-kit/README.md`'s comment on the arm names the variant lane between the two it names.
- `.workflow/release-declarations.md` gains a Behavior-changes bullet: the arm derives re-spellings of the kit's bash rows, a call naming its own tables names a limits table as the fifth, and nothing is owed by a consumer running the arm with no operands.

## Producers and consumers

- **The lane.** Producer: `--run-guard-tests`, on every run, with no enabling knob, wherever the arm runs: the consumer's battery line, `EVIDENCE_KIT_RUN_guard_tests` in `scripts/evidence-config.knobs`, and both `gates.yml` steps. Consumer: whoever reads the arm's exit and its mismatch lines, which is the build session and the evidence-kit suite, the suite reading the exit alone.
- **The arm's three-valued exit is unchanged in meaning.** Readers of it, by `git grep -n -e 'run-guard-tests' -e 'run_guard_tests'` over the tracked tree: `native/src/emit/mod.rs`'s arm-table row, `.github/workflows/gates.yml`, `scripts/evidence-config.knobs`, `README.md`, `guard-kit/README.md`, `gate-sdk/SPEC.md` §The non-gate arm's split sentence, `guard-kit/gate-tests/consumer-rules.test.sh`, `guard-kit/templates/guard-config.knobs`, `native/src/gates/guard_registration.rs` and `.workflow/release-declarations.md`. Only the seam test passes positional tables, and delta 6 gives it the fifth.
- **The spawned programs are unchanged**: the member and `git`, as gate-sdk/SPEC.md §The non-gate arm records, so that sentence stands.
- **The limits table.** Producer: a build session, by hand, off the lane's own failure lines. Consumer: the lane, each field at the read delta 3 names. Readers of the directory, by `git grep -n 'guard-tests'` over the tracked tree: the arm, `native/src/hook/escalation.rs` (its own table alone) and `native/src/gates/guard_registration.rs`, which reads every tracked file for rule citations and meets none in a table of bare names.
- **The section grammar** reads comment lines every table reader skips. The opening lines, by `grep -n '^# rule `' guard-kit/guard-tests/cases.tsv guard-kit/guard-tests/background-cases.tsv`: thirty-seven, each a rule name the roster carries.
- **The run's price.** The prototype's derived rows added about fourteen seconds to the arm on this tree, one spawn per row. The arm is not a pre-commit gate.
- **Knobs.** None is minted, and the arm's declared roster is unchanged.

## Existing sections updated

- `guard-kit/SPEC.md` §Testing — the opening paragraph's lane count, and the three new passages (deltas 1, 2 and 3).
- `guard-kit/SPEC.md` §The generic ruleset — the shared directions (delta 4).
- `native/src/emit/run_guard_tests.rs` — the lane, the deriver and the limits read, in that file or a module beside it (deltas 1, 2 and 3).
- `guard-kit/guard-tests/variant-limits.tsv` — new (deltas 3 and 5).
- `guard-kit/gate-tests/consumer-rules.test.sh` — the fifth positional and the summary lines (delta 6).
- `guard-kit/README.md` — the arm's comment (delta 6).
- `.workflow/release-declarations.md` — the Behavior-changes bullet (delta 6).
- `docs/guard-kit/SPEC.md`, `docs/guard-kit/README.md` — generated, stale once any delta lands (all deltas).

## Retired spellings

- None — the amendment adds a lane and a table and retires no spelling.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls guard-kit/SPEC-*.md`), discharged at the iteration.
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Entry moved** — the queue entry moves to Done in the merging commit, ahead of the drain stage.
