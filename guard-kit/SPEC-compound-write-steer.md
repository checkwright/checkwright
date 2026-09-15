# SPEC amendment: compound-write-steer

Queue entry: `file-authoring-act-ungoverned`, the write-residue unit of `guard-friction-reach`
(operator direction, 2026-09-15, lead-relayed).

**What the entry's residue is now, re-measured against the tree rather than read off the entry.** The
entry names a write outside the gitignored scratch set: a heredoc to a tracked file, a commit-message
file, a scratch script under another root. It records the Write-tool steer as refused for now,
because a steer firing on writes rule 17 had just granted would argue with its neighbour. Rule 25
landed after that record. Its arm (b) is that steer, bounded to targets git does not ignore, which
rule 17 never grants, so the refusal's ground does not reach it. The spec is ground truth over the
entry's history. What rule 25 leaves falling through is the **compounded** emitter write to a target
git does not ignore:

- arm (a) needs the statement to satisfy rule 17 alone, and a tracked target fails rule 17's (b);
- arm (b) needs the statement to be the whole command.

`guard-kit/guard-tests/cases.tsv` pins that fall-through as a row: `printf x >> tracked.md; git status`.

**Measured before ruling.** The probe was a read-only sweep of this repository's harness session
transcripts (every Bash `tool_use` command). Heredoc bodies and single-quoted spans were stripped, and
the commands were split into statements on `;`, `&&`, `||` and newlines. It counted statements led by
`cat`, `printf` or `echo` that write through `>`/`>>` to a target outside `.tmp/`, the `.workflow`
logs and `/dev/`.

- In the roughly 2.3k calls after rule 25 landed: **13 compounded writes to repo-relative targets**
  (3 blocked by other rules) and 9 lone writes (5 blocked by arm (b)).
- Over the week before: 141 compounded writes to repo-relative targets and 9 to targets outside the
  repository.

**The ruling.** Rule 25 gains arm (c), the composition of arms (a) and (b): a compounded emitter write
to a target git does not ignore steers to authoring that file with the Write or Edit tool as its own
call, and the rest as a separate call. With it, every emitter write statement meets exactly one of
five outcomes: rule 17's grant, arm (a), arm (b), arm (c), or a stated decline. The residue is closed
by partition, not by a new class of rule.

## The seam

- **Kit mechanism:** one arm in `guard_rule_emitter_write` (`lib/guard.sh`), its decision-table rows,
  and rule 25's SPEC text. The destination test is rule 17's, and the emitter roster is
  `GUARD_KIT_APPEND_BINS`, already a knob.
- **Consumer config:** none added.
- **Private rule content:** none in reach. The steer names no capture arm and no surface, for rule
  25's own stated reason.

## What changes

### (1) Rule 25 gains arm (c), a compounded write to a target git does not ignore {design-bearing}

`guard-kit/lib/guard.sh`, `guard_rule_emitter_write`'s multi-statement branch. Today a statement whose
`_guard_emitter_write` holds is skipped (`continue 2`) as soon as one target fails `git check-ignore`.
It becomes a three-way reading of that statement, in statement order, blocking on the first statement
that qualifies:

- every target is gitignored: arm (a), unchanged;
- a target sits under `/dev/`: that statement declines, as arm (b) declines on a device;
- otherwise: **arm (c)** blocks. The message names the lead word and the non-ignored target(s). It
  steers to writing that file with the Write or Edit tool as its own call and issuing the rest of the
  command as a separate one: git does not ignore the target, so no rule grants the write, and
  compounding it takes the whole call off the match path. It keeps `!<command>` as the escape.

A statement whose targets are mixed (one ignored, one not) is arm (c), since arm (a)'s premise (the
write alone would be granted) is false for it. Every decline rule 25 already states still runs first
and still holds: `_guard_emitter_unmodelled` over the whole command, a backgrounded command, and a
line whose heredoc openers sit in more than one statement. So arm (c) never reads a statement arm (a)
could not.

### (2) Decision-table rows for arm (c) {mechanical}

`guard-kit/guard-tests/cases.tsv`, rule 25's block:

- The row `fallthrough	printf x >> tracked.md; git status` becomes
  `block	printf x >> tracked.md; git status`.
- The header comment's clause `a compounded write rule 17 would refuse alone does not fire` is
  replaced by arm (c)'s firing shape, plus the one compounded write that still passes: a device
  target.
- Rows added: `block` `mkdir -p docs && cat > docs/x.md <<'EOF'@NL@line@NL@EOF`,
  `block` `git status && echo x > /tmp/x.txt` (a target outside the repository fails
  `git check-ignore` exactly as arm (b) already reads it), and `fallthrough`
  `git status; echo x > /dev/stderr`. The backgrounded row stays `fallthrough`.

### (3) Rule 25's SPEC text: arm (c), its partition, and the arms' measurements {design-bearing}

guard-kit/SPEC.md §The generic ruleset, rule 25. **Not yet applied:**

- A third bullet after arm (b):

  > - **(c) Compounded, to a target git does not ignore.** The command holds more than one statement,
  >   and one of them is a write both arms above leave: not every target is gitignored, so arm (a)
  >   does not fire, and it is not the whole command, so arm (b) does not. The steer is to write that
  >   file with the Write or Edit tool as its own call and issue the rest separately. It is the
  >   composition of (a) and (b), and with it every emitter write statement meets rule 17's grant, one
  >   of these three arms, or one of the declines below.

- The declines paragraph's `and, for arm (b), a target under /dev/` becomes
  `and, for arms (b) and (c), a target under /dev/`.
- The sentence `Arm (b) has no measured instance and is kept as the READ steer's mirror (rule 10).` is
  replaced by:

  > Arm (b) fired on lone writes to tracked targets once it shipped, and arm (c) was cut from the
  > compounded shape that still fell through beside it: writes led by `cat`, `printf` or `echo` to a
  > repo-relative target outside the scratch set, chained to another statement.

### (4) §scan-prompts' aged reading of what still keys as an emitter write {mechanical}

guard-kit/SPEC.md §scan-prompts, the **`cat` is the only word this bites today** paragraph. Its
sentence `What still keys as echo > or cat > is a write rule 17 declines: a tracked target, an emitter
off the roster, a live substitution, a second statement.` predates rule 25. A tracked target and a
second statement now block before the log. **Not yet applied:**

> What still keys as `echo >` or `cat >` is a write both rule 17 and rule 25 decline: a shape rule
> 17's clause (d) leaves unmodelled, a backgrounded launch, a line whose heredoc residue cannot be
> attributed, or a device target.

The paragraph's measurement and its closing sentence stand. That sentence says the axis is general
and currently near-single-instance, and it stays true.

## Producers and consumers

- **Arm (c)'s block** — producer: `guard_rule_emitter_write`, reached from `guard_generic_rules`
  through the consumer's copied `bash-guard.sh` on every `PreToolUse(Bash)` call. The hook is
  deployed in this repo's committed settings. Consumer: the agent session, through `guard_block`
  (stderr plus exit 2). A blocked call never reaches rule 27, so these rows leave the friction log.
  §scan-prompts' **a reading that drops after a grant widens** caveat is what a later triage reads.
  The drop is the steer landing, not fewer writes.
- **The decision-table rows** — reader: `--run-guard-tests`, run by the battery. Delta 2 flips one
  pinned row. That row is the whole holder of the prior boundary, so the flip is the assertion of this
  change and not a regression.
- **No new knob, field, state or file.** No corpus is narrowed. The friction log loses rows, and its
  readers assert a count's shape, never a floor, so point 5 has no red-condition subject.

Derivation of the rosters here and below: `git grep -n "rule 25\|guard_rule_emitter_write"` over the
tracked tree, and a read of guard-kit/SPEC.md §scan-prompts for prose describing which emitter writes
reach the log. The build unit re-derives both.

## Existing sections updated

- `guard-kit/lib/guard.sh` — `guard_rule_emitter_write`'s multi-statement branch (delta 1).
- `guard-kit/guard-tests/cases.tsv` — rule 25's rows and header comment (delta 2).
- `guard-kit/SPEC.md` — §The generic ruleset, rule 25 (delta 3), and §scan-prompts, the **`cat` is the
  only word this bites today** paragraph (delta 4).
- `docs/guard-kit/SPEC.md` — the on-site SPEC mirror, regenerated with `--emit docs-mirror --write`
  (deltas 3 and 4).
- `.workflow/release-declarations.md` — one guard-kit bullet: the generic ruleset now blocks a
  compounded `cat`/`printf`/`echo` write to a target git does not ignore and steers it to the Write or
  Edit tool, and a consumer needs to do nothing (delta 1).

## Retired spellings

- None — no delta retires a spelling. The rewritten sentences in deltas 3 and 4 are not names any other
  surface cites.

## Definition of Done

- [ ] **Causal completeness** — arm (c) has a deployed producer and a named consumer, and the decision
      table holds its firing shapes and the device decline.
- [ ] **Instruction surfaces: instruction only** — the block message carries no grounds, and delta 3
      places them.
- [ ] **Merged with no information lost** — rule 25 and §scan-prompts integrated, not appended.
- [ ] **Amendment deleted** — this file removed on merge.
- [ ] **Roster re-derived** — rosters re-derived against the tree before the merge counts as complete,
      with any missed site landed and named in the commit.
- [ ] **Removals propagated** — `check-amendment-retired-spelling` green.
- [ ] **Gaps filed** — any cross-component gap found while landing filed through the gap inbox.
