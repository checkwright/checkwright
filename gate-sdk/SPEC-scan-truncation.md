# SPEC amendment: scan-truncation

`bin/port-blockers.sh`'s tokenizer stops scanning a declaration part-way through
and reports the remainder as absent rather than as undecidable. Both arms
under-report silently: the criterion-7 roster has been reporting *clean* where it
had simply stopped reading, and the `--group` arm derives a truncated key from
the same scan. This amendment repairs the two causes and retires the honest-limit
paragraph that recorded them.

**Why this is a unit and not an in-flight edit.** The repair changes the default
arm's output. `port-corpus-grouping-census-unbought` landed on a proved claim
that adding the `--group` arm left that output byte-identical, and a fix folded
into a session that was implementing something else would have falsified the
claim in the same breath as making it — so the repair is scoped, priced and
proved on its own.

**The guard that exists and does not hold, stated because the next reader will
find it and conclude the finding is false.** The heredoc branch *does* carry a
here-string guard: it declines to treat `<<` as a heredoc introducer when the
character after it is a third `<`. That guard is bypassed rather than absent. The
generic redirect branch below it consumes the first `<` on its own, and the scan
re-enters at the second `<`, where the two characters ahead are the third `<` and
the first character of the operand — so the guard's condition is satisfied, the
heredoc branch fires, and the operand is captured as a delimiter that never
recurs. Reading the guard and stopping there yields the wrong verdict; the defect
lives one step past it.

## What changes

### 1. A here-string is consumed as one operator, ahead of the heredoc branch

`<<<` is recognised and consumed whole — all three characters, as a redirection
operator that ends the current word and leaves the parser out of command
position — **before** the heredoc branch can be reached at the second `<`.
**[design-bearing]**

Placing it ahead of the heredoc branch rather than widening that branch's guard
is what makes the fix total: the bypass exists because the operator was reachable
in two passes, and an operator consumed whole cannot be re-entered part-way.

### 2. A `)` closing a pushed substitution frame pops it, ahead of the
case-pattern reading

Where the parser holds a pushed frame — the state a `$(`, a backquote, a `<(` or
a subshell paren opens — a `)` **pops that frame** and is not read as the close
of a `case` pattern. **[design-bearing]**

The bug this fixes is independent of delta 1 and is reached through
`[[ … ]]`: the in-case predicate is true whenever the double-bracket state is
set, so a `)` closing a command substitution *inside* a conditional was read as a
case-pattern terminator, the frame was never popped, and the restored quoting
state was lost — corrupting every line after it. Ordering the pop first is
correct rather than merely convenient: a pushed frame is unambiguous evidence
that the `)` closes it, while the case-pattern reading is a guess the parser only
needs where no frame is open.

### 3. The byte-stability claim is re-scoped, and the honest-limit paragraph is
retired

§port-blockers currently states that the default arm's output is byte-unchanged,
and separately carries a paragraph recording the truncation as a measured limit
both arms live with. **[mechanical]**

The claim is re-scoped rather than deleted: what was proved is that *adding the
`--group` arm* changed no byte of the default arm's output, and that remains
true and worth stating. What it must not be read as is a standing guarantee that
the arm's output never moves — this amendment moves it deliberately. The limit
paragraph is **removed**, because the limit is gone; leaving it would leave a
spec claiming a defect the tree no longer has.

### 4. The blocker the repair surfaces is recorded where criterion 7 keeps them

The repaired scan reports `paste` for `check-gate-assertions`, a program the
payload does not carry. **[design-bearing]** It is recorded beside criterion 7's
existing worked example rather than left in a commit message, because it is
exactly what the roster exists to surface and it was invisible for as long as the
truncation stood.

## Producers and consumers

**This amendment introduces no new state, event, field or interface.** It changes
what an existing report says, and both arms' producers and consumers are
unchanged: the producer is `bash gate-sdk/bin/port-blockers.sh` invoked by hand
from the repo root, and the consumer is the human session sequencing a port
cohort. No knob is added, so there is no new default to be unset anywhere.

**This delta widens what both arms report, and the widening is real.** Every
declaration the scan previously abandoned part-way is now read to the end, so the
reports gain rows they never had. The red conditions of the readers, not their
subjects:

- **Neither arm is a gate**, so neither has a red condition of its own: nothing
  machine-parses either output, no `# graph:` manifest names them, and no
  freshness gate holds either — the standing ruling in §port-blockers, unchanged
  here. The failure mode a widened report can cause is therefore a *session*
  mis-sequencing a cohort, which is what the widening removes rather than adds.
- **The undecidable count rises, and that is the repair working.** Lines the scan
  never reached carry command-position expansions it cannot resolve, and each is
  now reported `?` rather than passed over. A reader must not read the higher
  count as a regression: the pre-repair count was low because the scan was blind,
  not because the corpus was clean.
- **The criterion-7 roster gains a genuine blocker** (delta 4), which is a
  *widening* of the work the port owes and never a narrowing. No member loses a
  row it previously carried except the false positives delta 2 removes — words
  the corrupted quoting state emitted from inside an `awk` program, which were
  never external-program requirements at all.

**The already-ported corpus was checked, and it is clean.** Each already-ported
member's pre-port shell source was recovered from history and re-scanned with the
repaired tokenizer; **no** member reports an external-program requirement the
payload does not carry. The check is recorded because the truncation stood
through every cohort delivered so far, so "did the false green ship a blocker
into the crate?" is the question a reader will ask, and the answer is no. The
born-native member has no pre-port source and is correctly absent from the check.

## Existing sections updated

- **gate-sdk/SPEC.md §port-blockers** — owned by deltas 1, 2 and 3. The
  measured-limit paragraph is removed and the exclusivity sentence's
  byte-stability claim is re-scoped to what was actually proved. The derivation
  inputs, the undecidable ruling and the no-freshness-gate ruling are unchanged.
- **gate-sdk/SPEC.md §The port-candidate criteria, criterion 7** — owned by delta
  4. The criterion's derived roster gains the `check-gate-assertions` finding
  beside its existing worked example, with the note that it was invisible while
  the scan truncated.

## Definition of Done

- [ ] **Causal completeness** — no new state, event, field or interface; the
      unchanged producer and consumer are named, and the widened report's readers
      carry their red conditions, including the rising undecidable count read as
      the repair working rather than as a regression.
- [ ] **Repair proved by run** — no still-shell member leaves the tokenizer in a
      broken state at EOF, and the default arm's delta against the pre-repair
      scanner is enumerated rather than asserted.
- [ ] **Already-ported corpus checked** — each ported member's pre-port source
      re-scanned; a blocker found there is filed as its own unit, never fixed
      here.
- [ ] **Merged with no information lost** — deltas integrated into
      §port-blockers and criterion 7; the merged spec reads as one document.
- [ ] **Amendment deleted** — this file removed on merge; the none-remain half is
      discharged at the iteration while `SPEC-eighth-cohort.md` is in flight.
- [ ] **Removals propagated** — grepped every spec for the retired limit claim.
- [ ] **Gaps filed** — cross-component gaps found during the work filed as debt
      tasks.
