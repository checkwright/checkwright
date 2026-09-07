# SPEC amendment: stream-witness

**The Windows consumer smoke compares no manifest entry as a hash, because every `want` reaches
the comparison carrying a trailing carriage return — and after eighteen rounds nobody has seen a
single raw byte of the stream that `want` is read from.** This amendment buys that witness and
pins the decision rule it settles. **It does not choose the repair**, and §Why this ships an
instrument and not a fix states why that is a ruling rather than a shortfall.

**Ruled `lead, own-authority` 2026-09-07**, on this stage's escalation.

## Why this ships an instrument and not a fix

**The entry's own sequencing bar.** The finding states that one octet dump of a raw line off that
stream, on the runner, "must be bought BEFORE choosing between stripping the CR at the read and
changing how the stream is produced, because the two repairs differ in what else they cover". A
strip at the read covers exactly one read; a repair at the producer covers every reader of that
stream. Choosing between them without the dump is choosing which coverage to buy while blind to
which one is needed.

**A second ground, found at this stage and stronger than the first.** The entry's one remaining
inference is that `jq` emits CRLF into that stream on this runner. The report already carries a
control that bears on it and was never read as one: `wantalt` re-reads the same key with a
standalone `jq -r '.files[$p]'` through a **command substitution**, which strips trailing newlines
and would *preserve* a carriage return. In run `34142337941` it came back **clean — forty octets,
no `0d`, `printf '%q'` unquoted**. Same host, same run, same binary, same stdout-to-pipe. That is
evidence *against* the jq-CRLF inference and points instead at the process-substitution channel
the loop reads through, or at something after the read. Buying the fix now would be choosing a
repair against a hypothesis this run already weakens.

**A third ground, which is why this unit is sequenced at all.** `installer/SPEC-witness-fidelity.md`
establishes that the report and the verdict currently decompose one recorded string and print
different answers. Until that lands, **any octet dump this unit buys is printed by the very
instrument that is self-contradicting**. The witness is only worth the round it rides once the
report's fidelity is repaired, so this amendment's deltas land **after** that amendment's.

**The consequence, stated plainly:** this entry's terminal move is a **demotion**, not a Done —
the `[spec:]` tag drops and the entry returns to the deferred section under `[design-pending]`
with its open question narrowed from *where does the CR come from* to *which repair does the
observed dump select*. That is canon-kit/SPEC.md §Merging an amendment's route for an entry whose
deliverable is a corpus and whose amendment delivered one increment, and it is chosen here rather
than discovered at the merge.

## What changes

### (1) The manifest loop reads the raw line and splits it in the shell

`installer/consumer-smoke/run-smoke.sh`'s manifest arm reads with `IFS= read -r line` and derives
`path` and `want` by parameter expansion, so the line **as it came off the stream** is a variable
the report can print {design-bearing}.

Today (`:425`) the loop reads `IFS=$'\t' read -r path want`, and the split is the same operation
that would consume the evidence: there is no point at which the bytes the stream delivered exist
in a variable. After this delta there is, and it is the operand delta 2 dumps.

**The split is not merely equivalent, it is less normalizing, and that is a second reason to take
it.** Tab is an **IFS whitespace** character, so `IFS=$'\t' read` collapses runs of tabs and
strips trailing ones — probed at this stage rather than reasoned about: on the line `a\t\tb\t`,
`read` yields `path=a want=b` while the parameter-expansion split yields `path=a want=$'\tb\t'`.
For the two-field, single-tab line `jq -r '"\(.key)\t\(.value)"'` actually emits, the two agree
byte for byte — including on a CR-terminated line, where neither strips the CR. Where they differ
is on an anomalous line, and there the current read **normalizes the anomaly away** before
anything can observe it. That is the same anti-normalization rule §The consumer smoke already
holds for its renderings, reached at the read.

### (2) The raw line's octet dump reaches the report

Two raw lines are carried into `manifest_report` and printed through the existing `value_probe`:
the **first line of the stream**, whatever it is, and the **raw line of the first disagreeing
entry** {design-bearing}.

Both, because they answer different questions. The first line says whether the channel delivers
CRLF at all — it is the cheapest possible statement about the stream and is unconditional on any
comparison. The disagreeing entry's line says whether *this* entry's bytes are what the verdict
claims, which is the question the verdict is actually about; carrying only one of the two would
buy half the witness for the same round.

They ride as named operands rather than being re-read, for the reason `held_probe` exists at all:
a value re-read is a second observation and cannot testify about the first. `value_probe`'s octet
dump is produced through no construct that could normalize, which is the property this whole
delta depends on.

### (3) The decision rule is pinned, so the next round settles the repair with no design turn

`installer/README.md` §The consumer smoke states, ahead of the round, what each possible dump
selects {design-bearing}.

- **The raw line ends `0d 0a`, or ends `0d` after the newline is stripped** — the carriage return
  is in the stream, so the repair is at **production or at the channel**, and it covers every
  reader of that stream rather than this one read. The channel is then discriminated from the
  producer by the control already present: `wantalt` reads the same manifest through a command
  substitution, so a clean `wantalt` beside a CR-bearing raw line implicates the process
  substitution `< <(…)` rather than `jq`, and a CR-bearing `wantalt` implicates `jq`.
- **The raw line carries no `0d`, yet `want` does** — the carriage return is introduced at or
  after the split, so the repair is at **the read**, and its coverage is correctly narrow.
- **Neither carries a `0d`** — the verdict's decomposition is wrong rather than the value, which
  is `installer/SPEC-witness-fidelity.md`'s subject and not this one's, and this entry demotes
  with its question answered in the negative rather than left open.

Three outcomes, each naming its repair and its coverage. Whoever reads the next red round executes
against this list instead of re-deriving it.

### (4) The record carries the sequencing and the control that was never read as one

§The consumer smoke's round record gains the two facts this stage established {mechanical}: that
`wantalt`'s clean read in run `34142337941` is evidence against the jq-CRLF inference, and that
this witness is sequenced behind `SPEC-witness-fidelity.md` because until that lands the
instrument printing the dump is itself in question.

## Producers and consumers

**New state — the raw manifest line, and its two carried instances (deltas 1 and 2).**

- **Producer:** the manifest arm's read loop in `installer/consumer-smoke/run-smoke.sh`, which runs
  on every profile of every install-smoke leg with no enabling configuration whatsoever — it is
  the arm that already reads the manifest, and the variable is its own input. Reachability is not
  a question here: the loop runs on the four binding and non-binding legs alike, every run.
- **Consumer:** `manifest_report`, on the manifest arm's **failure branch only** (`:436-437`),
  which prints both instances through `value_probe`; and a human reading a red leg's log, at the
  transition delta 3's decision rule describes.
- **Fields and their named readers.** The **first stream line** is read by delta 3's first and
  third outcomes, which is where a reader decides whether the channel delivers CRLF at all. The
  **disagreeing entry's raw line** is read by all three outcomes, and it is the one compared
  against `want`'s already-printed decomposition to separate "the CR arrived in the stream" from
  "the CR was introduced at the split". Both are printed only on the branch that already prints
  six values per sample, so neither adds output to a green run.

**Existing integration prose updated:** §The consumer smoke's round record describes the prior
flow — eighteen rounds of value-side reading with no statement about the stream — and delta 4
updates it in place rather than appending a nineteenth entry to a list that no longer describes
what the harness observes.

**No corpus is narrowed.** Delta 1 changes how one line is split, not which lines are read; the
manifest entry set, the profile set and the leg set are all unchanged, so the causal-completeness
check's point 5 does not bind. Stated because delta 1 *looks* like a narrowing — it removes IFS
collapsing — and the honest reading is the opposite: it makes the split **less** lossy, so the
value set the comparison sees can only gain content, never lose it.

**Reader red conditions, enumerated anyway because delta 1 sits on the comparison's own input.**
`install-smoke` and `install-smoke-macos` are **binding** and red on a non-zero `run-smoke.sh`;
`install-smoke-windows` is `continue-on-error` and already red; `install-smoke-powershell` is
binding on its own arms. On every one of them the split's output for the line the producer
actually emits is byte-identical to today's, probed and not assumed, so no leg's verdict can move
on a tree whose manifest agrees. Delta 2 adds failure-branch-only output and delta 3 and 4 are
prose.

## Existing sections updated

- `installer/README.md` §The consumer smoke, the manifest arm's description — the read and its
  split, which delta 1 changes (delta 1).
- `installer/README.md` §The consumer smoke, the manifest report's six-value block — it gains two
  values and the section's truth table has a row for each, since the block's contract is that
  every value it prints has a named reader there (delta 2).
- `installer/README.md` §The consumer smoke, the round record — the decision rule, the sequencing
  behind `SPEC-witness-fidelity.md`, and `wantalt`'s clean read as a control that bears on the
  producer question (deltas 3 and 4).
- `installer/README.md` §The consumer smoke, its anti-normalization rule — extended to the **read**,
  which delta 1 shows was normalizing a class of anomaly out of existence before any probe could
  see it (delta 1).
- `installer/consumer-smoke/run-smoke.sh`'s `# spec:` pointer at `:429` — it describes the tuple the
  shape test records and the split that produces it, both of which delta 1 moves (delta 1).

<!-- update-target-exempt: installer/SPEC-witness-fidelity.md is a sibling amendment of the same iteration and the same file, and its deltas land first; it is named here as an ordering fact, and no delta of this amendment changes it -->
- `installer/SPEC-witness-fidelity.md` — ordering only.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls installer/SPEC-*.md`), which for this iteration is
      discharged by whichever batch merges the last of the two installer
      amendments, not by this one alone.
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Entry demoted, not Doned** — the `[spec:]` tag drops and the entry
      returns to the deferred section under `[design-pending]`, its open
      question narrowed to which repair the observed dump selects.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
