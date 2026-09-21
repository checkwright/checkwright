# SPEC amendment: account-noun-plural

The account-identification pattern §check-commit-msg ships matches a singular
account noun only. A trailing `s` is a letter, and the noun's boundary class
excludes letters. A sentence naming two backticked, handle-shaped tokens after
*"the accounts were"* passes both readers, while the same token after *"the
account is"* reds. The guard is therefore weakest against the phrasing
that names several identities, which is exactly the correlation case its own
rationale gives for existing. This amendment picks one of the three candidate
moves the paired entry left open and records why. Sited in `gate-sdk/` because
the pattern, its template and both of its readers are that SPEC's.

## What changes

### (1) The noun alternation admits the plural, on both readers

The shipped pattern's noun group becomes `(accounts?|logins?|usernames?|handles?)`
in both orders of the shape. It lands in `gate-sdk/templates/msg-patterns.list`
and in this repo's tracked `scripts/msg-patterns.list`, the two copies of the one
pattern. Both readers take it, `check-commit-msg` on the message and
`check-tree-terms` on the tree. **{design-bearing}**

**This is move one of three, and here is why the other two lose.**

- **A plural arm scoped to the message reader only** leaves the tree blind to the
  plural, and the tree is the surface where a leak persists. It also
  forks one pattern source into two, and the shared source is what §check-tree-terms
  relies on to keep the halves from drifting apart. A message-only plural would
  buy the message reader's looser over-refusal economics at the cost of that
  invariant.
- **Leaving the gap and recording it** ships a guard whose stated rationale is
  inverted at the phrasing that matters most. A recorded hole of that shape is a
  finding, not a limit.
- **The cost the singular-only choice was made to avoid has shrunk.** Measured
  at this stage, with the plural arm and delta 2's shape, the pattern matches
  **one** tracked sentence. The entry's filing recorded three rewordings. The
  one left is delta 3's.

### (2) A handle-shaped token carries at least one letter or digit

Today's shape admits a token made of `_`, `.` and `-` alone. The token
alternative becomes
`[a-z]*[0-9A-Z][A-Za-z0-9_.-]*|[a-z]+[_.-][A-Za-z0-9_.-]*|[_.-]+[A-Za-z0-9][A-Za-z0-9_.-]*|[a-z]{6,}`.
It admits every token the current shape admits except one with no letter or
digit. POSIX ERE has no lookahead, so the requirement is spelled by splitting on
the first non-lowercase character. **{design-bearing}**

**Why this is part of the unit.** With the plural arm alone, guard-kit/SPEC.md's
*"handles a `..` traversal"* reds: the verb `handles` sits beside a token that is
all punctuation. No forge handle is punctuation only, so the narrowing loses no
coverage, and it removes that false positive rather than rewording it. For the
singular it changes nothing on the tracked tree. The probe under Producers and
consumers shows the singular's hits are unchanged.

### (3) The one remaining tracked hit is reworded

gate-sdk/SPEC.md §The workflow directory: *"no .gitignore line accounts for"*
becomes *"no `.gitignore` line covers"*. Its docs mirror is regenerated.
**{mechanical}**

This is the cost §check-commit-msg's tree-exact calibration weighs. It is paid
once, on a sentence that is not a quotation, because the alternative is a guard
that is blind at the correlation case. The calibration paragraph records the
weighing (delta 4). It is not a precedent for rewording prose to satisfy the
heuristic in general.

### (4) §check-commit-msg states the plural and the measured cost

The shape paragraph names the plural, and the calibration paragraph records that
the plural was weighed against tree-exactness. It cost one rewording, and a
verb reading of *accounts* or *handles* next to a handle-shaped token is the
over-refusal it accepts. The topology limit stays unchanged. **{mechanical}**

### (5) Fixtures and unit tests pin the correlation sentence

`check-commit-msg`'s and `check-tree-terms`'s `bad/` sides gain the plural,
two-handle sentence the entry names as the case that matters. It takes its
tokens from the synthetic series the pair already uses, never a real one. The
`good/` sides gain a verb `handles` next to a punctuation-only token, and a
short all-lowercase CLI name. The crate's unit tests of the shipped pattern
shapes gain the same rows. **{mechanical}**

### (6) The template change is declared under the both-sections rule

The landing commit appends Tightened-gates bullets for `check-commit-msg` and
`check-tree-terms` and a Behavior-changes bullet on
`gate-sdk/templates/msg-patterns.list`. That is gate-sdk/SPEC.md §upgrade-smoke's
both-sections rule, and §check-release-change-declared's class-T arm reds this
commit otherwise. The remedy the
bullets carry for an adopter whose tree reds is to reword the sentence or edit
their own copy of the list, which `init` then keeps. **{mechanical}**

## Producers and consumers

**The pattern.** Producer: the two list files, read through
`GATE_SDK_MSG_PATTERN_FILES`, which defaults to `<gates-dir>/msg-patterns.list`
and which this repo deploys. Consumers: `check-commit-msg` in the generated
`commit-msg` hook and `check-tree-terms` at pre-commit and in the battery, both
through the crate's POSIX ERE engine. So the pattern must parse in that engine,
not only in `grep -E`. Delta 5's unit tests are that oracle.

**The adopter population.** `init` claims the template. An adopter who never
edited it takes the new pattern on upgrade, and a sentence in their tree can
red. That is why delta 6 is Tightened-gates-shaped and not only a behavior note.

**Point 5.** Delta 2 narrows the set of tokens the shape admits, so each
reader's red condition is named. Both readers red on **finding** a match, which
is monotone, and neither asserts a count or a floor. The narrowing can therefore
only remove matches, and the matches it removes are punctuation-only tokens.
The probe is `.tmp/`-scratch, run through `--scratch-run`: it `git grep -nE`s
the candidate over the tracked tree minus `msg-patterns*` and `*/gate-tests/*`,
greps it over `git log --all` messages, and runs it over five sample lines. It
returned `gate-sdk/SPEC.md`:1052 and its mirror as the only tracked hits, and 1
history hit, which today's singular pattern also matches. The samples hit on
the singular, on the plural-before-token form and on *"two logins: `a`, `b`"*,
and pass on *"handles a `..`"* and *"handles `gh`"*.

**Point 6.** Delta 3 obliges a corpus, the tracked hits, and the probe
enumerated it at one member whose value is delta 3's rewording.

## Existing sections updated

- `gate-sdk/SPEC.md` §check-commit-msg, the shape and calibration paragraphs.
  (deltas 1, 2 and 4)
- `gate-sdk/SPEC.md` §The workflow directory, the one sentence. (delta 3)
- `gate-sdk/templates/msg-patterns.list` and `scripts/msg-patterns.list`, the
  pattern line and its comment. (deltas 1 and 2)
- `gate-sdk/gate-tests/check-commit-msg/` and `gate-sdk/gate-tests/check-tree-terms/`,
  plus `native/src/gates/commit_msg.rs` and `native/src/gates/tree_terms.rs`
  unit tests. (delta 5)
- `.workflow/release-declarations.md`, the three bullets. (delta 6)
- `docs/gate-sdk/SPEC.md`, the generated mirror. (all deltas)

## Retired spellings

- None — the pattern line is replaced in place and no name, knob, path or token
  is displaced; the reworded sentence (delta 3) is prose, not a spelling.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness
      check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — the list files' comment
      carries the rule, not its grounds; §check-commit-msg carries the grounds.
- [ ] **Merged with no information lost** — §check-commit-msg re-phrased, not
      appended.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — `check-amendment-retired-spelling` green.
- [ ] **Gaps filed** — any cross-component gap discovered during the work is filed.
