# SPEC amendment: perl-rewrite-steer

Queue entry: `in-place-rewrite-steer-reach`, lead unit of `guard-friction-reach` (operator direction,
2026-09-15, lead-relayed). Reshaped at spec under a second direction: steer agents off interpreters
and utilities whose side effects cannot be read, onto predictable tools checkwright builds in Rust.
**It waits on `SPEC-rewrite-arm.md`** (entry `rewrite-arm`), because the arm that amendment mints is
this steer's target.

**The two calls the entry left open, ruled under that direction.**

- **Every in-place `perl`, like every in-place `sed`, steers to the rewrite arm.** The arm handles a
  sweep over many files in one reviewable call and a single-file rewrite equally, so the operand
  count stops being the bound. The earlier bound (fire on one operand only, since a sweep has no
  better form than N Edit calls) fell when a better form for the sweep was minted. It also closes the
  asymmetry that bound opened against the `sed` arm, which already fires on every in-place rewrite.
- **`python3 -` bodies: no steer yet, for want of a target.** Delta 3 records the grounds. Deciding
  whether a body writes a file needs text every rule's skeleton blanks. And the inline computations,
  unlike a rewrite, have no predictable tool to send them to until
  `inline-interpreter-substrate-census` names one.

**Measured before ruling.** The probe was a read-only sweep of this repository's harness session
transcripts (every Bash `tool_use` command, 2026-09-08 to 2026-09-15, about 27k calls). Heredoc bodies
and single-quoted spans were stripped before tokenizing.

- **`perl` with an in-place flag: 95 calls.** Exactly one literal operand: 23. Two or more: 41. An
  operand carrying `*`, `?` or `[`: 20. Driven by `xargs` or a pipe: 1. Not classifiable: 10.
- **`python3 -c` / `python3 -` bodies: 743 calls.** 572 write one path, 50 write several paths or
  loop over files, and 121 write nothing.

So the widened arm reaches every `perl`-led in-place call but the `xargs` one.

## The seam

- **Kit mechanism:** one option-table row and one firing clause in rule 8's walker, both in-place
  steers repointed at the rewrite arm, the decision-table rows, and rule 8's SPEC text. `perl` is a
  universal interpreter name, and its switch grammar is shell-substrate knowledge.
- **Consumer config:** none. `GUARD_KIT_SCRIPT_INTERPRETERS` lists `perl` for rule 23 and is not read
  here, because rule 8 is not an interpreter-roster rule.
- **Private rule content:** none in reach.

## What changes

### (1) Rule 8's walker gains a `perl` row, and both in-place arms steer to the rewrite arm {design-bearing}

`guard-kit/lib/guard.sh`: `_guard_program_operands` gains a `perl` table, and `guard_rule_sed_file`'s
segment loop also takes a segment led by `perl`. The function keeps its name. It is internal and
already carries the `awk` arm. The table is conservative in the direction the `awk` row already takes:

- `-e` or `-E` as a whole word consumes the next word, which supplies the program. In a bundle
  (`-ne`, `-pe`), `e`/`E` takes the rest of the word, or the next word when the rest is empty. Either
  way the bundle ends there.
- `i` anywhere in a bundle marks the in-place rewrite, and the rest of that word is its backup
  extension (`-i`, `-i.bak`, `-pi`, `-0pi`, `-0777pi`). The bundle ends there.
- `a c n p s t T u U w W X` take no argument, and the bundle continues. `0` and `l` take the digits
  that follow them.
- `--` ends options. Any other option word or bundle letter (`-I`, `-M`, `-m`, `-F`, `-x`, `-d`, `-D`,
  `-C`, `-V`, `-S`, `-f`, a long option) returns non-zero, so the caller declines.
- With no supplying option, the first bare word is the program file and every later word is an
  operand.

**The `perl` arm blocks when the walker marked the rewrite in place and at least one operand
exists.** No operand means `perl -pi` reading stdin, which rewrites no file, so that case passes.

**Both in-place messages steer to the rewrite arm.** The `sed` arm's message is replaced and the
`perl` arm takes the same one. The message:

- prints the command derived by `_guard_front_end`, as rule 23's does:
  `bash <front-end> --rewrite [--regex] [--expect <n>] [--] <find> <replace> <file>…`;
- says the arm replaces a literal (or, with `--regex`, a line-scoped POSIX ERE) with fixed text across
  every named file and prints each changed span;
- names the Edit tool for an edit that needs more than a fixed replacement (a capture group, a
  deletion keyed on context);
- keeps `!<command>` as the escape.

The arm sits in rule 8's dispatch position, before both auto-allow rules.

### (2) Decision-table rows {mechanical}

`guard-kit/guard-tests/cases.tsv`, under rule 8's block. Its header comment gains the `perl` arm's
firing and non-firing shapes.

- **`block`:**
  - `perl -pi -e 's/a/b/' notes.md`
  - `perl -0pi -e 's/a/b/' notes.md`
  - `perl -i.bak -pe 's/a/b/' notes.md`
  - `perl -pi -e 's/a/b/' a.md b.md`
  - `perl -pi -e 's/a/b/' src/*.rs`
- **`fallthrough`:**
  - `grep -rl foo src | xargs perl -pi -e s/a/b/` (the lead is `xargs`)
  - `perl -pi -e 's/a/b/'` (no operand)
  - `perl -I lib -pi -e 's/a/b/' notes.md` (an unmodelled option)
  - `perl -ne 'print if /x/' notes.md` (no in-place flag)
  - `bash gate-sdk/bin/run-gates.sh --rewrite --regex 'a|b' 'c' notes.md` (the steer target itself,
    with a quoted pipe, is blocked by no rule)

The existing `sed -i` row keeps its `block`. The guard test reads the output class, not the message
text, so the repointed message moves no row.

### (3) Rule 8's SPEC text: the `perl` row, the repointed steer, and the `python3` passage {design-bearing}

guard-kit/SPEC.md §The generic ruleset, rule 8. **Not yet applied:**

- The lead line becomes `` **`sed` or `awk` reading a file, or `sed` or `perl` rewriting one** ``.
- Its first sentence's `` `sed -i` (or any short bundle carrying `i`) to the Edit tool `` becomes
  `` `sed -i` or `perl -i` (any short bundle carrying `i`, on any file operand) to the `--rewrite` arm
  (§rewrite), with the Edit tool named for an edit a fixed replacement cannot express ``.
- The **One walker, not a parser per tool** option list gains a `perl` bullet stating delta 1's table.
- A paragraph follows the `awk` arm's paragraphs:

> **Both in-place arms steer to a tool whose effect is its command line.** An in-place `sed` or `perl`
> program can read, write elsewhere, or execute, and the guard cannot tell which from outside the
> program. `--rewrite` replaces fixed text in named files and does nothing else, so the steer trades a
> program for a statement of intent. The Edit tool stays named for a single edit needing more than a
> fixed replacement. A sweep over many files is one `--rewrite` call, so neither arm bounds its
> operand count. **What passes:** `perl -pi` with no file operand, which rewrites no file; an
> in-place call behind another command word (`xargs perl -pi`), which is its lead's rule's subject; and
> a `perl` option the walker does not model.
>
> **`python3 -` bodies are not steered, because no target exists yet.** Deciding whether an inline body
> rewrites a file needs text inside the heredoc or the `-c` argument, which the skeleton every rule
> here declares blanks. And unlike a rewrite, what the bodies compute has no predictable tool to be sent
> to. Rule 23's ruling still holds, that a body carried in the command string is shown to the approver
> verbatim, so the shape stays a reviewable call rather than a hazard. Most inline bodies sampled did
> write one path, so the shape will keep ranking. That is measured friction waiting on a target, not a
> missed row.

## Producers and consumers

- **The widened block** — producer: `guard_rule_sed_file`, reached from `guard_generic_rules` through
  the consumer's copied `bash-guard.sh` on every `PreToolUse(Bash)` call. The hook is deployed in this
  repo's committed settings. Consumer: the agent session, through `guard_block`. A blocked call never
  reaches rule 27, so `perl -pi` rows leave the friction log. §scan-prompts' **a reading that drops
  after a grant widens** caveat applies: fewer rows is not fewer rewrites.
- **The printed steer command** — consumer: the agent, which runs the `--rewrite` arm. Its producer
  exists only once `SPEC-rewrite-arm.md` lands, which is the build order this amendment states.
- **The decision-table rows** — reader: `--run-guard-tests`, run by the battery.
- **No new knob, field, state or file.** No corpus narrows, and the friction log's readers assert a
  count's shape, never a floor.

Derivation of the rosters here and below: `git grep -n "guard_rule_sed_file\|_guard_program_operands"`
over the tracked tree (`lib/guard.sh`, guard-kit/SPEC.md rule 8 and its docs mirror), and
`git grep -n "rule 8"` over `guard-kit/`, which found §scan-prompts' **`cat` is the only word this
bites today** paragraph. That paragraph says `sed` is absent from the log because rule 8 blocks it
upstream, and it stays true. The build unit re-derives both.

## Existing sections updated

- `guard-kit/lib/guard.sh` — `_guard_program_operands` and `guard_rule_sed_file`, both messages
  (delta 1).
- `guard-kit/guard-tests/cases.tsv` — rule 8's rows and header comment (delta 2).
- `guard-kit/SPEC.md` — §The generic ruleset, rule 8 (delta 3).
- `docs/guard-kit/SPEC.md` — the on-site SPEC mirror, regenerated with `--emit docs-mirror --write`
  (delta 3).
- `.workflow/release-declarations.md` — one guard-kit bullet: rule 8 now blocks every in-place `perl`
  as it blocks `sed -i`, and both steer to the `--rewrite` arm. A consumer needs to do nothing
  (delta 1).

## Retired spellings

- None — no delta retires a spelling. Rule 8's lead line and its steer message are rewritten in place,
  and neither wording is a name another surface cites.

## Definition of Done

- [ ] **Causal completeness** — the widened arm has a deployed producer and a named consumer, and its
      steer target has landed before it.
- [ ] **Instruction surfaces: instruction only** — the block message carries no grounds, and delta 3
      places them.
- [ ] **Merged with no information lost** — rule 8's text integrated, not appended.
- [ ] **Amendment deleted** — this file removed on merge.
- [ ] **Roster re-derived** — rosters re-derived against the tree before the merge counts as complete,
      with any missed site landed and named in the commit.
- [ ] **Removals propagated** — `check-amendment-retired-spelling` green.
- [ ] **Gaps filed** — any cross-component gap found while landing filed through the gap inbox.
