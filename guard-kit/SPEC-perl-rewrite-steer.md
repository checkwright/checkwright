# SPEC amendment: perl-rewrite-steer

Queue entry: `in-place-rewrite-steer-reach`, lead unit of `guard-friction-reach` (operator direction,
2026-09-15, lead-relayed).

**The two calls the entry left open, ruled.**

- **`perl` in place: the steer fires on exactly one literal file operand.** A rewrite of one named
  file has a better form, the Edit tool. A sweep over several files is legitimate work that the Edit
  tool would charge for once per file. The measurement below shows sweeps are most of the class, so
  firing on every in-place `perl` (the `sed` arm's shape) would block mostly legitimate calls. Firing
  on none would leave the one-file rewrite unsteered.
- **`python3 -` bodies: no steer.** Delta 3 records the grounds in the rule. The short version: the
  discriminator (does the body write a file?) lives in the heredoc or `-c` body, which every rule's
  skeleton blanks. The committed `Bash(python3 -*)` grant already rests on rule 23's
  body-visibility ground.

**Measured before ruling.** The probe was a read-only sweep of this repository's harness session
transcripts (every Bash `tool_use` command, 2026-09-08 to 2026-09-15, about 27k calls). Heredoc bodies
and single-quoted spans were stripped before tokenizing.

- **`perl` with an in-place flag: 95 calls.** Exactly one literal operand: 23. Two or more literal
  operands: 41. An operand carrying `*`, `?` or `[`: 20. Driven by `xargs` or a pipe: 1. Not
  classifiable (a `|`-delimited substitution defeats the splitter): 10. The hook blocked 3 of the 95.
- **`python3 -c` / `python3 -` bodies: 743 calls.** 572 write one path, 50 write several paths or
  loop over files, and 121 write nothing.

The one-operand arm therefore reaches about a quarter of the `perl` class. The sweep share stays as
measured friction, which §The triage criterion calls an honest outcome.

## The seam

- **Kit mechanism:** one option-table row and one firing clause in rule 8's walker (`lib/guard.sh`),
  its decision-table rows, and the rule's SPEC text. `perl` is a universal interpreter name, and its
  switch grammar is shell-substrate knowledge. Neither is a class CLAUDE.md §The provenance seam names.
- **Consumer config:** none. No knob is added. `GUARD_KIT_SCRIPT_INTERPRETERS` already lists `perl`
  for rule 23 and is not read here: rule 8 is not an interpreter-roster rule, and a knob would let a
  consumer drop an option table the walker needs.
- **Private rule content:** none in reach.

## What changes

### (1) Rule 8's walker gains a `perl` row and the in-place arm fires on one literal operand {design-bearing}

`guard-kit/lib/guard.sh`: `_guard_program_operands` gains a `perl` table, and `guard_rule_sed_file`'s
segment loop also takes a segment led by `perl`. The function keeps its name. It is internal and
already carries the `awk` arm, so a rename would retire a spelling and buy nothing. The table,
conservative in the direction the `awk` row already takes:

- `-e` or `-E` as a whole word consumes the next word, which supplies the program. In a bundle
  (`-ne`, `-pe`), `e`/`E` takes the rest of the word as the program, or the next word when the rest is
  empty. Either way the bundle ends there.
- `i` anywhere in a bundle marks the in-place rewrite, and the rest of that word is its backup
  extension (`-i`, `-i.bak`, `-pi`, `-0pi`, `-0777pi`). The bundle ends there.
- `a c n p s t T u U w W X` take no argument, and the bundle continues. `0` and `l` take the digits
  that follow them in the same word.
- `--` ends options. Any other option word or bundle letter (`-I`, `-M`, `-m`, `-F`, `-x`, `-d`, `-D`,
  `-C`, `-V`, `-S`, `-f`, a long option) returns non-zero, so the caller declines rather than guess
  which word is the program.
- With no supplying option, the first bare word is the program file and every later word is an
  operand.

The `perl` arm blocks when **all** of these hold: the walker marked the rewrite in-place; exactly one
operand exists; and that operand is not `-` and carries none of `*`, `?`, `[`. The message steers to
the Edit tool, says a sweep over several files is untouched, and names `!<command>` as the escape, in
the `sed` message's shape. A pipe into the segment does not suppress it, as for `sed`: `-i` rewrites
its operand whatever arrives on stdin. The arm sits in rule 8's existing dispatch position, before
both auto-allow rules, so the placement argument rule 8 already states covers it.

### (2) Decision-table rows for the `perl` arm {mechanical}

`guard-kit/guard-tests/cases.tsv`, under rule 8's block. Its header comment gains the `perl` arm's
firing and non-firing shapes. Rows:

- `block`: `perl -pi -e 's/a/b/' notes.md`, `perl -0pi -e 's/a/b/' notes.md` and
  `perl -i.bak -pe 's/a/b/' notes.md`.
- `fallthrough`: `perl -pi -e 's/a/b/' a.md b.md` (two operands), `perl -pi -e 's/a/b/' src/*.rs` (a
  glob operand), `grep -rl foo src | xargs perl -pi -e s/a/b/` (the lead is `xargs`),
  `perl -pi -e 's/a/b/'` (no operand), `perl -I lib -pi -e 's/a/b/' notes.md` (an unmodelled option)
  and `perl -ne 'print if /x/' notes.md` (no in-place flag).

§Testing's decision-table obligation (a firing and a non-firing case per rule) is met by these rows.

### (3) Rule 8's SPEC text: the `perl` row, its bound, and the `python3` refusal {design-bearing}

guard-kit/SPEC.md §The generic ruleset, rule 8. **Not yet applied:**

- The rule's lead line becomes `` **`sed` or `awk` reading a file, or `sed` or `perl` rewriting
  one** ``. Its first sentence gains `perl -i` (any bundle carrying `i`, on exactly one file operand)
  steered to the Edit tool.
- The **One walker, not a parser per tool** option list gains a `perl` bullet stating delta 1's table
  in the list's register.
- A paragraph follows the `awk` arm's paragraphs:

> **The `perl` arm fires on one file operand, and the bound is the ruling.** The Edit tool is the
> better form for a rewrite of one named file. For a sweep, it is one call per file where a `perl`
> sweep is one reviewable call, which is the serialization cost §What the steering is buying names.
> So the arm fires only on an in-place rewrite with exactly one operand that is neither `-` nor a
> glob. Two or more operands, a glob operand, an `xargs` lead and a read all pass. The bound was
> measured before it was ruled: sweeps over named files or a glob were most of the in-place `perl`
> calls sampled, and the one-operand shape was about a quarter of them. **The `sed` arm is not bounded
> the same way:** it fires on every in-place rewrite whatever its operand count, so the two arms
> differ on a multi-file sweep.
>
> **`python3 -` bodies are not steered, on three grounds.** First, whether an inline body rewrites a
> file is decided by text inside the heredoc or the `-c` argument, which the skeleton every rule here
> declares blanks. A test reading it would be a lexer over a language no kit owns, and its false
> fires land on the computations that write nothing. Second, rule 23 already rules that a body
> carried in the command string is shown to the approver verbatim, so there is nothing for a guard to
> compensate for. A committed grant of the inline shape rests on exactly that ground. Third, the
> calls this shape ranked for were multi-line bodies that such a grant did not match (a triage reading,
> not a probe of the harness). The unmatched form is what cost the decision, not the write, and a steer
> to the Edit tool would leave that form as it is. **The
> honest limit:** most inline `python3` bodies sampled did write one path, so a later reading will
> find the shape still ranking. That is measured friction this ruleset declines to steer, not a
> missed row.

## Producers and consumers

- **The `perl` arm's block** — producer: `guard_rule_sed_file`, reached from `guard_generic_rules`
  through the consumer's copied `bash-guard.sh` on every `PreToolUse(Bash)` call. The hook is wired
  by this repo's committed settings, so the enabling config is deployed. Consumer: the agent session,
  through the harness's hook protocol (`guard_block`: stderr plus exit 2). A blocked call never
  reaches rule 27, so the friction log loses the one-operand `perl` rows. §scan-prompts' **a reading
  that drops after a grant widens** caveat applies to a steer the same way: fewer `perl` rows is not
  fewer rewrites.
- **The decision-table rows** — reader: `--run-guard-tests`, run by the battery.
- **No new knob, field, state or file.** No corpus is narrowed. The friction log loses rows, and its
  readers (`scan-prompts`, `kpi-prompt-friction`) assert a count's shape, never a floor. Their red
  conditions are unchanged, so point 5 has no red-condition subject.

Derivation of the rosters here and below: `git grep -n "guard_rule_sed_file\|_guard_program_operands"`
over the tracked tree, which found `lib/guard.sh` and guard-kit/SPEC.md rule 8 (plus its docs mirror).
`git grep -n "rule 8"` over `guard-kit/`, which found §scan-prompts' **`cat` is the only word this
bites today** paragraph. That paragraph names `sed` as absent from the log because rule 8 blocks it
upstream, a statement this amendment leaves true and extends to one-operand `perl` by delta 3's
caveat pointer. The build unit re-derives both rosters.

## Existing sections updated

- `guard-kit/lib/guard.sh` — `_guard_program_operands` and `guard_rule_sed_file` (delta 1).
- `guard-kit/guard-tests/cases.tsv` — rule 8's rows and header comment (delta 2).
- `guard-kit/SPEC.md` — §The generic ruleset, rule 8 (delta 3).
- `docs/guard-kit/SPEC.md` — the on-site SPEC mirror, regenerated with `--emit docs-mirror --write`
  (delta 3).
- `.workflow/release-declarations.md` — one guard-kit bullet: the generic ruleset now blocks a
  one-file in-place `perl` rewrite and steers it to the Edit tool, and a consumer needs to do nothing
  (delta 1).

## Retired spellings

- None — no delta retires a spelling. Rule 8's lead line is rewritten in place, and its old wording is
  not a name any other surface cites.

## Definition of Done

- [ ] **Causal completeness** — the arm has a deployed producer and a named consumer, and the decision
      table holds both its firing and non-firing shapes.
- [ ] **Instruction surfaces: instruction only** — the block message carries no grounds, and delta 3
      places them.
- [ ] **Merged with no information lost** — rule 8's text integrated, not appended.
- [ ] **Amendment deleted** — this file removed on merge.
- [ ] **Roster re-derived** — this amendment's rosters re-derived against the tree before the merge
      counts as complete, with any missed site landed and named in the commit.
- [ ] **Removals propagated** — `check-amendment-retired-spelling` green.
- [ ] **Gaps filed** — any cross-component gap found while landing filed through the gap inbox.
