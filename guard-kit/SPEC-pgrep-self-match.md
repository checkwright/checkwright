# SPEC amendment: pgrep-self-match

guard-kit rule 12 (`guard_rule_pgrep_self_match`) blocks a `pgrep -f`/`pkill -f`
only when its pattern literal occurs a second time in the same command. Its own
grounds say the harness's wrapper argv carries the literal, and the wrapper holds
the whole command text, so one occurrence is enough to self-match. The rule's
"a pattern occurring nowhere else in the command is a genuine query and passes"
contradicts those grounds. This amendment replaces the occurrence count with the
self-match predicate itself.

## What changes

### (1) Rule 12 fires when the pattern matches the command's own text

The occurrence count is replaced by the condition that decides a self-match.
**{design-bearing}** A walkable `pgrep`/`pkill` segment carrying `-f` is
blocked when its pattern, read as the ERE `pgrep` compiles it to, matches the
raw command text. The segment walk, the command-word read past
`until`/`while`/`if`/`!`, and the pattern walker (`_guard_pgrep_pattern`) are
unchanged. Three calibrations follow from what `pgrep` does with the options the
walker already accepts:

- `-i` / `--ignore-case` — the match against the command text is
  case-insensitive, as `pgrep`'s own is.
- `-x` / `--exact` and `-v` / `--inverse` — the segment **declines**. With `-x`
  the pattern must match the whole argv line, and the wrapper's line is longer
  than any pattern in it; with `-v` the predicate is inverted and a self-match
  argument no longer applies. Declining is the ruleset's conservative direction.
- A pattern that is not a valid ERE (the matcher refuses it) declines.

**Probed at authoring, 2026-09-21.** A lone foreground `pgrep -af
'zzq-probe-literal'` run through this harness exited 0 and printed its own
wrapper: `/bin/bash -c source <snapshot> … && eval 'pgrep -af
'"'"'zzq-probe-literal'"'"'; …' < /dev/null && pwd -P >| <cwd file>`. The
wrapper is not replaced by an `exec` of the last command, because the command is
not the last word of the `-c` string. So the single-foreground spelling the queue
entry asked about is not clean either. No spelling of a pattern that matches its
own text is clean under a wrapper that carries the command.

**What passes after the change, and why that is not a loosening of the
doctrine.** A pattern that does not match its own text passes. The bracket trick
(`pgrep -f '[r]un-smoke.sh'`) is that case: the regex needs `run-smoke.sh`, and
the text holds `[r]un-smoke.sh`. Today the rule blocks it only when the literal is
repeated, which is a false block, since the predicate cannot self-match. The
bracket trick stays refused as the *sanctioned* form on delegation-kit's grounds
(delegation-kit/SPEC.md §The delegation model). Those grounds say the guard is
the enforcement half *because* a correct form one character from an incorrect
one will be got wrong, so a guard that blocks exactly the incorrect form is the
rule doing its job.

**Replacement text for guard-kit/SPEC.md §The generic ruleset, rule 12** —
**Not yet applied.** It replaces the rule's text from "— a `pgrep`/`pkill -f`
whose" through "and a pattern occurring nowhere else in the command is a genuine
query and passes.":

> — a `pgrep`/`pkill -f` whose pattern matches the command's own text is
> **blocked**. `-f` matches against full argv, and the harness runs a command
> through a wrapper whose argv carries the whole command text, so such a pattern
> always finds at least the wrapper: `until ! pgrep -f '<script>'; do …; done` has
> a permanently-true condition and never exits, a one-shot query always answers
> *running*, and a `pkill -f` signals its own wrapper. It reds nothing: the work
> completes correctly and the only symptom is the foreground cap absorbing an
> unbounded loop, which reads from outside as a fixed cap-length wait. Fires on
> the conjunction no allowlist glob expresses: a segment whose **command word** is
> `pgrep` or `pkill` — read past a leading `until`/`while`/`if`/`!`, which do not
> change which binary runs — carrying `-f` (bare or bundled in a short cluster),
> whose pattern operand, read as the ERE `pgrep` compiles, matches the raw
> command text (case-insensitively under `-i`). A pattern that does not match its
> own text cannot self-match and passes; the bracket trick is that case, and it
> passes this rule while staying refused as the sanctioned form (below).

The rest of the rule's text is kept, with its closing conservative-direction
sentence rewritten to:

> Conservative by construction, in this ruleset's established directions: an
> expansion or substitution anywhere in the command declines outright (rule 6
> already blocks those shapes); a `pgrep` without `-f` matches process **names**
> rather than argv and is untouched; an unrecognized option, an option whose
> argument cannot be walked, a second bare operand, `-x`/`--exact`,
> `-v`/`--inverse`, and a pattern that is no valid ERE all decline. Each biases
> toward passing rather than toward a false block.

The block message in `guard-kit/lib/guard.sh` is rewritten on the same terms: it
says the harness's wrapper argv carries the whole command, pattern included, and
drops "this command's own argv … carries that same literal", which named the
repeat.

### (2) The rule-12 fixture rows follow the predicate

`guard-kit/guard-tests/cases.tsv`'s rule-12 block and its header comment are
rewritten to the new predicate. **{mechanical}** The header comment's
"a pattern occurring nowhere else are genuine queries and pass" becomes "a
pattern that does not match its own text passes". Rows:

- `fallthrough	pgrep -f run-smoke.sh` becomes `block` — the single-occurrence
  arm this unit exists for.
- New `block	while pgrep -f 'checkwright-gates --run-validate'; do sleep 5; done`
  — the attested waiter, verbatim.
- New `block	if pgrep -f worker; then echo up; fi` — the `if`-headed spelling.
- New `fallthrough	until ! pgrep -f '[r]un-smoke.sh'; do sleep 5; done` — the
  non-self-matching pattern passes rule 12. It then meets rule 19, which declines
  a `pgrep` condition under clause (c), so the row's verdict is `fallthrough`.
- New `fallthrough	pgrep -xf run-smoke.sh` — the `-x` decline.
- The existing `block` rows (174, 175) and the `pgrep` without `-f` row (177)
  are kept.

Rule 19's clause (c) row `fallthrough	while pgrep -f worker >/dev/null 2>&1; do
sleep 20; done; echo waited` would now stop at rule 12 as a `block`, so it would
pin another rule's verdict. That row is respelled without `-f`
(`while pgrep worker >/dev/null 2>&1; do sleep 20; done; echo waited`), which
rule 12 leaves alone and which still reaches clause (c)'s refusal.

## Producers and consumers

- **The predicate (delta 1).** *Producer:* `guard_rule_pgrep_self_match` in
  `guard-kit/lib/guard.sh`, reached on every Bash `PreToolUse` call of a consumer
  that wires a bash guard sourcing the library. This repo's `scripts/bash-guard.sh`
  does, and so does the shipped `guard-kit/templates/bash-guard.sh`. *Consumer:*
  the session whose call is blocked, which reads the block message. There is no
  compiled twin: `grep -rln "pgrep_self_match" --include=*.sh --include=*.rs .`
  returned `guard-kit/lib/guard.sh` alone, so
  `guard-kit/gate-tests/guard-lib-parity.test.sh`, which compares the five
  primitives guard-kit holds twice, is not a reader. No field is added.
- **Narrowing check (point 5).** Delta 1 widens the refusal set and narrows it in
  one shape, the repeated bracket-trick literal. The one reader of the verdict is
  the guard test lane over `cases.tsv`, which compares each row's expected
  verdict exactly, so it reds on every flipped row; delta 2 enumerates them.
- **The rows rule 12 can reach (point 6).** Probe:
  `grep -n "pgrep\|pkill" guard-kit/guard-tests/*.tsv` matched lines 174–177 and
  321 of `cases.tsv`, and nothing in `background-cases.tsv` or
  `escalation-cases.tsv`. Each one's value after delta 1:
  - 174 `bash run-smoke.sh & until ! pgrep -f run-smoke.sh; …` — `block`,
    unchanged (the pattern matches its own text).
  - 175 `pkill -f run-smoke.sh; bash run-smoke.sh` — `block`, unchanged.
  - 176 `pgrep -f run-smoke.sh` — flips to `block` (delta 2).
  - 177 `pgrep run-smoke.sh; bash run-smoke.sh` — `fallthrough`, unchanged (no `-f`).
  - 321 rule 19's clause (c) row — respelled (delta 2).
- **Surfaces that state the old predicate.** Probe:
  `grep -rn "nowhere else\|occurring \*\*elsewhere\*\*\|genuine query" --include=*.md --include=*.sh --include=*.tsv --include=*.rs .`
  matched guard-kit/SPEC.md (rule 12, two lines), `guard-kit/guard-tests/cases.tsv:173`,
  the generated `docs/guard-kit/SPEC.md` mirror, and the queue entry. The
  delegation-kit template's pattern-match paragraph
  (`delegation-kit/templates/agent-execution.md`, "never by pattern-matching the
  process table") states no occurrence count, so it stays true unchanged.

## Existing sections updated

- guard-kit/SPEC.md §The generic ruleset, rule 12 — the replacement text above (delta 1).
- `guard-kit/lib/guard.sh` — `guard_rule_pgrep_self_match`'s predicate and block message (delta 1).
- `guard-kit/guard-tests/cases.tsv` — the rule-12 block and rule 19's clause (c) row (delta 2).
- `.workflow/release-declarations.md` — a `## Behavior changes` bullet led by
  `**guard-kit rule 12**`: a `pgrep -f`/`pkill -f` whose pattern matches its own
  command text now blocks with one occurrence, and a repeated bracket-trick
  literal no longer blocks. A guard rule is no `gates.list` member, so it takes
  the Behavior-changes grammar rather than a Tightened-gates bullet, as the rule-17
  and rule-22 changes did (delta 1).
- `docs/guard-kit/SPEC.md` — the generated mirror, regenerated by its arm (all deltas).

## Retired spellings

- None — the rule keeps its name and function, and no knob, path or tag is renamed.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness
      check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a
      template, agent definition or shim carries no grounds; a delta places them
      (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the
      canonical-spec text it refines rather than appending to it; the merged spec
      reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls guard-kit/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in
      `## Retired spellings` above.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks.
