# SPEC amendment: shape-witness

**The Windows install-smoke leg's manifest arm refuses a value as "not 40 lowercase hex" while
its own byte rendering of that same variable shows 40 lowercase hex, for every entry in the
profile.** Round 17 established that on the deciding entry all five printed values are one
identical 40-hex string that `printf '%q'` quotes nowhere, and the run still refuses with *the
`want` operand on `gate-sdk/README.md` is not 40 lowercase hex*, counting 493 of 493. The
observation is not about any operand. It is a **disagreement between two subsystems reading one
variable** — bash's `printf` on one side, the platform's `regcomp`/`regexec` behind `[[ =~ ]]` on
the other — and a report that renders through only one of them cannot adjudicate it. This
amendment makes the instrument print what each subsystem actually says, separately, so round 18
is terminal by construction rather than the eighteenth round of an open series.

**What this amendment asserts, and what it deliberately does not.**

- **Asserted** — the five deltas below, every one tree-verifiable and checkable at validate.
- **Not asserted** — a **green** Windows leg, the **content** of round 18's reading, and any
  claim about which subsystem is at fault. Asserting the third would be asserting the very thing
  the instrument is being built to find out. The shape is
  `observation-predicate-entry-cannot-drain-in-its-own-iteration`'s:
  *landed-but-unobservable, not unstarted*.

**The ruling this amendment makes, because the entry left it open.** The queue entry names three
candidate directions — render with `od -c` instead of `%q`; test length separately from character
class; compare against a value produced without the `jq`-into-`read` pipeline — and says each
edits a documented contract. **All three are taken**, because each alone leaves round 18
non-terminal: an octet dump without a decomposed predicate shows the bytes and not the verdict
that refused them; a decomposed predicate without an octet dump still reads its operand through
bash's own formatter; and neither separates a mangling introduced by the manifest pipeline from
one introduced anywhere else. Taken together they partition every hypothesis round 17 leaves
open, which is the property that makes the round terminal.

**The decider does not move, and that is the load-bearing refusal.** The composite
`[[ "$want" =~ ^[0-9a-f]{40}$ && "$got" =~ ^[0-9a-f]{40}$ ]]` stays the thing that selects the
exit code. The decomposed test delta 2 adds is an **observation**, never a verdict. Moving the
decider onto it now would green the leg on a *hypothesis* — that the ERE engine is the faulty
subsystem — before any round has shown which subsystem is faulty, and it would destroy the
disagreement that is the entire evidence. That is the same trap §The consumer smoke's
*What the arm must not do is normalize* already closes, reached from a new direction: normalizing
the operand and swapping in a matcher that happens to accept it are one move under two spellings.
If round 18 shows `len=40`, `class=ok` and `shape=fail` together, moving the decider becomes the
right repair and a documented host fact — and it is a **later** amendment's ruling, resting on an
observation this envelope does not assert.

## What changes

### (1) The report gains a rendering that is a byte rendering, beside the one that is not

Every value in the manifest report's per-path block is printed with an **octet dump** and an
explicit **length**, alongside the existing `printf '%q'` rendering rather than in place of it
{design-bearing}.

**The promise the section makes today is stronger than the renderer delivers, and round 17 is
what makes the gap visible.** `installer/README.md:1517-1518` says each value is "printed plain
and rendered **byte-exactly** beside it", and `:1565-1567` says the byte rendering "is not
decoration and not optional: a value carrying a trailing carriage return compares unequal and
*prints* equal, and no other line in the report can see it." Both sentences are written about
`printf '%q'`. `%q` is a **shell-quoting** renderer: it emits a string that, re-read by bash,
yields the same value. For the trailing-carriage-return class that motivated it, quoting and
byte-exactness coincide — `$'…\r'` — which is why the promise has held for five rounds without
anyone testing its edges. They are not the same property, and round 17 is the observation that
separates them: a rendering that quotes nothing is evidence that bash's formatter found nothing
worth quoting, and **not** evidence that the variable holds forty bytes drawn from `0-9a-f`.

**What lands.** `hash_probe` and `held_probe` each gain two lines beneath the existing `bytes`
line:

- `len` — `${#v}`, the length bash itself reports.
- `octets` — the value's bytes in hexadecimal, one pair per byte, produced by piping the value
  through `od` with no interpretation (`printf '%s' "$v" | od -An -tx1`, the newlines folded so
  the dump is one line). It must be produced **without** any construct that could itself
  normalize — no `echo`, whose escape handling is shell-dependent, and no re-quoting.

`%q` **stays**. It is the readable form, and every round from 12 through 17 is recorded in its
terms; deleting it would make the existing record unreproducible against a later run. Two
renderings whose disagreement is itself a finding is the point, and one of the two cannot lie
about bytes.

### (2) The shape predicate is decomposed, and each part's verdict prints beside the value it judged

The arm computes and prints, per value, the shape test's **independently observable parts** —
length-is-40, character-class-clean, and the composite `=~` verdict — using for the class test a
matcher that is not the ERE engine {design-bearing}.

**Today the predicate's verdict reaches the log only as a run-wide count and an operand name on
the very last line.** Nothing prints *which part* of the shape failed, and nothing prints the
predicate's verdict beside the value it judged. That is precisely why round 17 took a careful
human reading to state at all: the reader had to hold a rendering from the top of the report
against a refusal at the bottom and notice they could not both be true.

**What lands**, per value, one line beneath `octets`:

- `len40` — whether `${#v}` is exactly 40, from the length delta 1 already prints.
- `class` — whether the value contains any character outside `0-9a-f`, computed by
  `leftover="${v//[0-9a-f]/}"`: the residue after deleting every acceptable character is exactly
  the offending set. When it is non-empty the line names the residue and the **index of the first
  offending character**, so the reader gets a position and not only a set.
- `shape` — the composite `[[ "$v" =~ ^[0-9a-f]{40}$ ]]` verdict itself, printed as the pass/fail
  it is.

**The class test deliberately uses a different matcher from the composite, and that is the whole
mechanism.** `${v//[0-9a-f]/}` is a parameter expansion over a **glob bracket expression**;
`=~ ^[0-9a-f]{40}$` is an **ERE** evaluated by the platform's `regcomp`/`regexec`. They are two
independent implementations of "is this character acceptable". If the class test reports clean,
the length is 40, and the composite still fails, the report has printed a disagreement **between
two matchers inside the same shell on the same variable**, which is a statement about the host's
regex engine or its locale and about nothing else. Using `=~` for the decomposition would inherit
whatever the composite suffers and print an agreement that means nothing.

### (3) `want` gains a control produced without the manifest pipeline

The per-path block gains a sixth value, `wantalt`: `files[P]` read from the same lock through a
channel that uses neither the `jq`-into-`read` pipeline nor tab splitting {design-bearing}.

**`want` is the one value in the block with no independent producer, and the entry's own next
question is about exactly that.** `got`, `reread`, `own` and `raw` all come from `git
hash-object`, so the block already carries three controls for `got`. `want` arrives solely
through `jq -r '.files | to_entries[] | "\(.key)\t\(.value)"' | IFS=$'\t' read -r path want` —
a `jq` render, a tab-delimited line, a `read` split. The entry states the open question as *what
`read -r want` holds on MSYS bash versus what `%q` renders of it*, and the report as it stands
cannot answer it, because it holds no second reading of `want` to compare against.

**What lands.** `wantalt` is read with `jq -r --arg p "$path" '.files[$p]'` directly into a
command substitution — the same file, the same key, no line assembly, no tab, no `read`. It is a
**re-read**, labelled as one, and it takes the full probe treatment deltas 1 and 2 give every
value.

**The reading it buys is a clean partition.** `want != wantalt` places the defect in the
pipeline — the `jq` line render, the tab, or `read`'s own splitting — and the two octet dumps
name the byte. `want == wantalt`, both dumps identical, exonerates the pipeline entirely and
leaves the matcher as the only remaining subject. Neither outcome is asserted here.

**The block is six values, not five, and the section's own architecture sentence moves with it.**
`:1519-1521` reads "**Two of the five are the operands the failing comparison used, printed out
of the variables it read; the other three are re-reads**". It becomes two held and four re-read,
and the split's rationale is unchanged — a held value and a re-read may never wear one label.

**The count's own echo inside the tool moves with it, not only the doc's.** `run-smoke.sh:212`
prints `read the five values below against the truth table in installer/README.md §The consumer
smoke` immediately before the per-sample loop this delta extends; the literal becomes `six` in
this same delta, so the tool's own prompt and the truth table's own count cannot drift apart from
each other the moment either lands.

### (4) The truth table gains the rows that read the *instrument*, and the byte-rendering promise is corrected

`installer/README.md` §The consumer smoke's truth table gains three rows whose subject is the
harness rather than the operand, and the byte-rendering paragraph is corrected to say what `%q`
delivers {design-bearing}.

**The table has no row for round 17's observation, and its nearest row is falsified by it.** The
last row today reads *any of the five is not 40 lowercase hex → the value is not a hash — a stray
byte, a truncation or a refusal; the byte rendering shows which*. Round 17 has the shape test
failing and the byte rendering showing nothing, so the row's own remedy does not apply to the case
the arm is actually in. The table's rows all read the operand; the instrument was never a subject
it could name.

The three rows that land:

| observation | reading |
| --- | --- |
| `shape` fails while `len40` and `class` are both clean | two matchers in one shell disagree about one variable — the ERE engine or its locale is the subject, not the value; the operand is a hash and the harness refused it anyway |
| `%q` renders bare while `octets` shows a byte outside `0-9a-f` | `printf '%q'` is not a byte rendering on this host, so **every** prior round's "bare rendering" reading is weakened to what quoting alone establishes; re-read rounds 12-17 against the octet dump before citing them |
| `want != wantalt` | the manifest pipeline mangles between the lock and the comparison — the `jq` line render, the tab, or `read`'s splitting; the two octet dumps name the byte and the position |

**The promise is corrected rather than deleted.** `:1565-1567` keeps its point — the rendering is
not optional, and the carriage-return case is still exactly why — and gains the honest bound:
`%q` renders a value bash can re-read to the same bytes, which is byte-faithful for a stray
control character and is **not** in general a byte rendering; the octet dump is the line that
carries the byte claim, and the two are printed together so a disagreement between them is
visible rather than assumed away. `:1517-1518`'s "rendered byte-exactly" moves onto the octet
dump, which is the line that can carry it.

**One existing row makes the identical assumption and is corrected in place, not left as a fourth
survivor of the same falsified promise.** `:1561`'s `got != reread` row reads "`got`'s byte
rendering names the stray byte" — the same claim on `%q`'s bare output that round 17 falsifies for
the shape row two lines below it, and the new `%q`-renders-bare row above corrects generally. It
is rewritten to name `octets` rather than the bare rendering, the same reading the new `want !=
wantalt` row already gives its own pipeline-mangling case: `got != reread` → the two octet dumps
name the byte and its position. Grepped for every other row and prose sentence naming `%q`'s
output a byte rendering or byte-exact so no further survivor of the same claim is left uncorrected
by this delta.

### (5) The refusal states what it knows, rather than asserting the operand's shape

The `blocked` line's text is rewritten to report the shape test's **verdict** and the decomposed
observation beside it, instead of declaring the operand malformed {design-bearing}.

**The refusal currently makes a claim round 17 shows may be false.** It reads *the `want` operand
on `gate-sdk/README.md` is not 40 lowercase hex, and 493 of 493 disagreeing entries fail that
test*. The first clause is a statement about the value; what the arm actually observed is that
its shape test refused the value. Where the decomposition reports `len40` clean and `class`
clean, the first clause is simply untrue, and it is the line §The consumer smoke:1584-1587
promises is *readable from the last line alone, by a reader who never scrolls up*. A false
statement is what that reader takes away.

**What lands.** The refusal names: the path; which of `want` and `got` the shape test refused;
how many disagreements it refused; and — new — the decomposed verdict for the refused operand, so
the last line itself distinguishes *the operand is not a hash* from *this harness's matcher
refused a hash*. The exit code is unchanged and stays `2` in both cases: a matcher that refuses
valid input is a harness precondition every bit as much as a mangled operand is, and neither is a
finding about the consumer's tree. §The consumer smoke's exit-class paragraph gains that second
ground explicitly, because its current wording justifies the 2 on the operand alone.

## Producers and consumers

**New values: `len`, `octets`, `len40`, `class`, `shape` (deltas 1 and 2).**

- **Producer.** `installer/consumer-smoke/run-smoke.sh`'s `hash_probe` and `held_probe`, both
  called from `manifest_report()`'s per-sample block, which runs on the failure branch only —
  `manifest_report` is invoked at `:410-411` under `[[ "$mismatch" -eq 0 ]] ||`. **No enabling
  configuration exists or is minted**, and that is deliberate under the standing rule the prior
  amendment cited: *a report enabled by a configuration no CI job sets is the dead-producer
  shape, and the one host that needs this one is the host nobody is standing at.* So the values
  are produced wherever a disagreement is, on every leg, with nothing to switch on.
- **Consumer.** A human reading the failing job's log, at the transition the landed
  `smoke-comparison` amendment's delta 4 already established and this amendment does not change:
  **close**, reading `install-smoke-windows`'s log **job-keyed** off the run its own push
  produced (`gh run view <id> --log`, a free read of a finished run), never inferring the leg
  from the workflow's conclusion — the job is `continue-on-error: true`, so the workflow concludes
  `success` while the job concludes `failure`.
- **Every new field has a named reader.** `len` and `octets` are read by delta 4's second and
  third new truth-table rows. `len40` and `class` are read by delta 4's first row and by delta
  5's refusal line. `shape` is read by delta 4's first row. No field is added that no row reads.

**New value: `wantalt` (delta 3).**

- **Producer.** A `jq -r --arg p "$path" '.files[$p]'` command substitution inside
  `manifest_report()`'s per-sample loop, over the same `$LOCK` the arm already reads, on the same
  failure-only path. Its enabling configuration is likewise none.
- **Consumer.** The same human at the same transition, through delta 4's third row.
- **Reader named.** `wantalt` is read by exactly one row, and by the `want == wantalt` /
  `want != wantalt` partition delta 3 states. If a build finds no reading that consumes it, the
  field is removed rather than kept.

**Readers of the arm's verdict, each with its red condition.**

- **`install-smoke-windows`** (`.github/workflows/gates.yml:202`) — `continue-on-error: true`, so
  its red condition does not reach master's verdict. Untouched by every delta.
- **`install-smoke`** (Linux, `:88`) and **`install-smoke-macos`** (`:434`) — **binding**. Their
  red condition is a non-zero exit from `run-smoke.sh`. Every delta above adds code on the
  **failure branch only**, which on a green leg never executes, so neither leg's verdict can move
  on a tree where the manifest agrees. This is the assertion a build must actually check rather
  than assume, and it is why no delta touches the comparison, the loop, or any arm header.
- **The `installer_smoke` evidence suite** (`scripts/evidence-config.sh`, parser
  `parse-smoke-log`) — its red condition is derived from **the log's arm headers alone**, and it
  never reads the process status, so exit 1 and exit 2 are indistinguishable to it
  (§The consumer smoke:1593-1602). No delta adds, removes or renames an arm header, so its
  verdict is unchanged by construction. Stated rather than assumed, because delta 5 edits a
  refusal line and the natural reading is that a suite consumes it.

**No delta narrows a corpus.** Every delta adds an observation, a row or a clause; none prunes a
file, tightens a glob or drops a member. So the causal-completeness check's point 5 — a reader
whose verdict is non-monotone under a narrowing — has no narrowing to bind to here. Recorded
rather than left silent, so a later reader knows the point was checked and cleared rather than
overlooked.

**No knob is minted**, so no `<KIT>_<KNOB>` roster moves and no config seam changes. The
provenance seam is not approached: every delta is inside this repo's own acceptance harness and
its owner doc, `installer/consumer-smoke/run-smoke.sh` riding no payload and reaching no adopter.

## Existing sections updated

- `installer/README.md` §The consumer smoke — the *Five values per sampled path* block's
  introduction and its bullet list, which becomes six values with two held and four re-read
  (deltas 1, 2 and 3); the truth table, which gains three instrument-reading rows, whose existing
  shape row is qualified by the first of them, and whose existing `got != reread` row at `:1561`
  is corrected onto the octet dump alongside it, the same survivor-of-the-falsified-promise the
  shape row was (delta 4); the byte-rendering paragraph at `:1565-1567` and the "rendered
  byte-exactly" clause at `:1517-1518`, corrected onto the octet dump (delta 4); the
  malformed-operand exit-class paragraph at `:1569-1591`, which gains the second ground for the 2
  and the refusal's new text (delta 5).
- `installer/consumer-smoke/run-smoke.sh` — `hash_probe` at `:167-176` and `held_probe` at
  `:179-183`, which gain the length, octet and decomposition lines (deltas 1 and 2);
  `manifest_report()`'s per-sample loop at `:257-265`, which gains `wantalt`, and its own
  five-values count literal at `:212`, which becomes six (delta 3);
  `malformed_operands()` at `:197-203` and the `blocked` call at `:413-414`, whose text and
  decomposed report change (delta 5); and each touched function's own `# spec:` header sentence,
  which states what that function promises and must move with it (all deltas).
- `installer/README.md` §The consumer smoke's *What the native Windows leg has measured so far* —
  the round record gains the statement of what round 18 must show to be terminal, and round 17's
  block gains the consequence delta 4 draws from it, namely that `%q`'s bare renderings in rounds
  12 through 17 establish quoting rather than bytes (deltas 1 and 4).
<!-- update-target-exempt: the drop condition is a green observation this envelope explicitly does not assert, so touching it would assert what the envelope records as unasserted -->
- `.github/workflows/gates.yml` — **deliberately untouched**. The `continue-on-error` line's own
  comment conditions its removal on the leg being first observed green, and green is exactly what
  this amendment does not assert.
- `TASK-QUEUE.md` — `manifest-shape-predicate-and-rendering-disagree` promotes to New Features
  with this file's `[spec:]` ref, in the same commit as this file (all deltas).

## Definition of Done

- [ ] **Causal completeness** — every new value has a named, reachable producer on the failure
      branch, a named consumer, and a named reader at a named transition; no field is kept that
      no truth-table row or refusal line reads.
- [ ] **The byte claim sits on a line that can carry it** — the octet dump is produced through no
      construct that can normalize, `%q` is retained beside it, and no surface still calls `%q` a
      byte rendering.
- [ ] **The decomposition uses a different matcher from the composite** — the class test is a
      glob-bracket parameter expansion, never `=~`, so a disagreement between the two is
      observable rather than absorbed.
- [ ] **The decider did not move** — the composite `=~` still selects the exit code; no delta
      greens the leg by accepting what the shape test refused, and the anti-normalization rule is
      not circumvented by a change of matcher.
- [ ] **The refusal says only what was observed** — the last line distinguishes *the operand is
      not a hash* from *this harness's matcher refused a hash*, and asserts neither where the
      decomposition is silent.
- [ ] **The binding legs cannot move** — every added construct is on the failure branch, no arm
      header is added, removed or renamed, and the Linux and macOS install-smoke legs are
      verified green on a tree whose manifest agrees.
- [ ] **Nothing green is asserted** — no delta, and no line of the merged section, claims the
      Windows leg passes, predicts round 18's content, or names a faulty subsystem.
- [ ] **Battery and suite green** — `bash gate-sdk/bin/run-gates.sh` plus
      `bash gate-sdk/bin/build-native.sh`, neither discharging the other, and the
      `installer_smoke` suite clean on this host — which does not exercise the failure-path
      report, a limit §The consumer smoke states rather than gates.
- [ ] **Merged with no information lost** — each addition integrated into its proper canonical
      section, not appended.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component
      (`ls installer/SPEC-*.md`), discharged at the iteration rather than at the commit.
- [ ] **Removals propagated** — grepped for the retired "five values" count and for every surface
      calling `%q` a byte rendering; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps found during the work filed to the gap inbox.
