# SPEC amendment: verdict-witness

**The manifest arm's exit-2 verdict is earned by an entry the report cannot print, and the arm's
own owner doc already records that as a finding rather than a hypothesis.** The verdict is set by
a run-wide boolean; the report samples by position and by path; the two share no state, so the
row that decided the run is reachable only by coincidence. This amendment makes the verdict carry
its own witness.

**What this amendment asserts, and what it deliberately does not.** The unit is the instrument's
verdict-to-report coupling and nothing else.

- **Asserted** — the four deltas below, every one decidable against `run-smoke.sh` and
  `installer/README.md` on this host, with no Windows round needed to check any of them.
- **Not asserted** — anything about *why* rounds 12 to 15 disagree. §The consumer smoke rules the
  diagnostic series closed on the pairing, and this amendment neither confirms nor reopens it.
  What the repair buys for that question is stated once, as a consequence, and claimed no
  further: the next round's report will contain the verdict's own row, which is the one thing the
  fifth row's reading cannot get today.
- **Not asserted** — a green Windows leg. `gates.yml:207` keeps `continue-on-error: true`, whose
  drop condition is a green observation this envelope does not make.

**The wider option the entry left open is refused, and the refusal is the design.** The entry
offers (b): ask whether a run-wide flag is the right instrument at all when the report is
per-entry. Refused on two grounds the tree settles. Refusing at the first malformed operand is
already refused by the section itself — *"refusing early would trade the report for the verdict
and lose the more valuable half"* (`installer/README.md:1474-1476`) — and a per-entry verdict has
nowhere to go: an arm reports through one process exit code, and its evidence-side reader cannot
even see the code (see §Producers and consumers). What (b) is right about is kept and is the
whole of what lands: **a run-wide flag that records only that some entry tripped it is the
defect, and the repair is to make it record which.** That is (a) done as a witness rather than as
a third sample slot.

## What changes

### (1) The malformed flag becomes a witness, not a boolean

`installer/consumer-smoke/run-smoke.sh`'s manifest arm records **which** entry first failed the
operand shape test and **how many** did, in place of the bare `malformed=1` it sets today
{design-bearing}.

`:319` initializes `malformed=""` and `:327` sets it to `1` from inside the failure branch,
testing both operands. Nothing anywhere records the entry that set it, and `manifest_report`'s
sample set is chosen at `:208` (`bad[0]`, by position) and `:210-230` (the artifact row, by
path). The two are disjoint state, which is why the row earning the verdict is unprintable by
construction rather than by bad luck.

Two variables replace the one:

- **`malformed_first`** — the *whole tuple* of the first entry whose `want` or `got` failed
  `^[0-9a-f]{40}$`, in the same `<path><TAB><want><TAB><got>` spelling `bad_hash` already carries,
  so the report parses it with the code it already has and no second grammar is minted.
- **`malformed_n`** — how many disagreeing entries failed that test.

The count is not decoration and it is the field a reader acts on first. **One** malformed entry
out of hundreds is a statement about that path; **all** of them is a statement about the capture
step every entry runs through. Those two readings send a reader to different places, and today
the arm distinguishes them not at all.

`malformed_first` is the first rather than every one, on the sampler's own standing ground: the
report is a bounded sample by design, and a report that prints hundreds of rows is a report
nobody reads. The count is what says whether the one printed row is representative.

### (2) The report's sample set always contains the verdict's own row

`manifest_report()` takes the malformed witness as a named operand and prints it as a third
sample, deduplicated against the two it already chooses {design-bearing}.

The sample set becomes `bad[0]` ∪ the artifact row ∪ `malformed_first`, in that order, each
appended only when it is not already present. Where the malformed witness coincides with a sample
already chosen, the arm prints the coincidence rather than silently collapsing it — the shape
`:225-227` already uses for the artifact row, extended to the third member because the reader's
question is the same one ("is this the row the verdict is about?") and a silent collapse answers
it wrongly.

The report's header gains one line stating the count and the sampling rule, so a reader who sees
one malformed row knows whether the run held one or four hundred.

**The sampler's other two members are kept.** This is the half of the entry's option (a) that is
load-bearing: the first disagreeing path and the artifact row answer different questions from the
verdict's row, and the artifact row in particular is the filter-free control the section builds
its whole content argument on (`installer/README.md:1511`). A sampler narrowed to the verdict's
row would buy the verdict's legibility with the control.

### (3) The refusal names its operand instead of describing its class

The exit-2 message routed through `blocked` at `:334` states the path, the offending operand and
the count, in place of the class description it prints today {design-bearing}.

Today it reads *"a manifest comparison operand is not 40 lowercase hex — the report above renders
it byte-exactly"*, and on round 15 that sentence was true while the report above rendered two
other entries. A refusal that points at a report not containing its subject is worse than a bare
one, because it tells the reader the answer is somewhere it is not. The repaired message names
the path from `malformed_first`, says which of `want` and `got` failed the shape test (or both),
and carries `malformed_n`, so the verdict is readable from the refusal line alone even by a
reader who never scrolls up.

The exit code itself is unchanged, and so is its ground: a malformed operand is this harness's
precondition and takes 2, a genuine disagreement is a finding about the consumer and takes 1
(`installer/README.md:1463-1472`). This delta repairs what the refusal *says*, never what it
*is*.

### (4) §The consumer smoke states the coupling as a rule, not as a round-15 anecdote

`installer/README.md` §The consumer smoke records the repair, and states the general rule the
defect is an instance of {design-bearing}.

The section already carries the finding, at `:1594-1603`, as a fact about round 15 — *"the entry
that earned the exit-2 verdict is, by construction, one the report cannot print… The next repair
owns that, not another round."* This delta is that repair, so the passage stops being an open
finding and becomes the statement of what the arm now does. What lands beside it is the rule,
because the arm is not the only place the shape can arise:

> **A run-wide verdict states which entry earned it, or it is not readable.** Where an arm reports
> a bounded sample and decides on an unbounded scan, the deciding entry joins the sample. A flag
> recording only *that* something tripped it hands the reader a verdict and withholds its subject.

Three consequences of the repair land with it. The sampling rule at `:1395` (*"It samples at most
two paths, and neither is chosen by arrival"*) becomes at most three and names the third. The
after-the-report paragraph at `:1463-1479` keeps its ordering argument unchanged and gains the
sentence that the report the refusal points at now contains the refusal's subject — which is what
that argument assumed all along and did not hold. And the truth table's shape row at `:1457`
gains the count's reading, since the row is the one the arm acts on and the count is now part of
what it acts with.

The round record at `:1534-1625` is **not** rewritten. Rounds 12 to 15 measured what they
measured, and round 15's finding is what motivated this unit; the record gains one sentence
saying which repair the finding was drained into, and nothing about those rounds' readings
changes.

## Producers and consumers

**New state: `malformed_first`, the witness tuple (deltas 1, 2, 3).**

- **Producer.** The shape test at `run-smoke.sh:327`, on the failure branch inside the manifest
  loop, assigning on its first firing only. It is produced exactly where the flag it replaces is
  produced, so no new code path and no enabling configuration is minted — the arm is failure-path
  code with no knob, which is deliberate and is the section's own standing rule (*a report
  enabled by a configuration no CI job sets is the dead-producer shape*).
- **Consumers, two, at two named transitions.** `manifest_report()` receives it as an operand and
  prints it as a sample (delta 2), read by a human reading the failing job's log; and the
  `blocked` call at `:334` reads its path field into the refusal message (delta 3), read by the
  same human and by anyone reading the job's last line alone.
- **Every field has a named reader.** The tuple's three fields are the ones `bad_hash` already
  defines and `manifest_report:234` already parses — `path` is read by the sample header and by
  the refusal message, `want` and `got` by `held_probe` at `:236-237`. No field is added to the
  tuple, so none is unread.

**New state: `malformed_n`, the count (deltas 1, 3, 4).**

- **Producer.** The same failure branch, incremented on each entry failing the shape test.
- **Consumer.** The report's header line and the refusal message; its reader is the human, at the
  transition delta 4's own record names — close, reading `install-smoke-windows`'s log off the run
  its own push produced.
- **Its red condition, and why the field is not merely informational.** Nothing reds on the count;
  it discriminates two readings of one red. That is a field whose reader is a person rather than a
  gate, which is admitted here because the whole report is that kind of artifact — and it is
  stated rather than assumed, because a field with no *machine* reader is exactly the field the
  causal-completeness check exists to challenge.

**The verdict's evidence-side reader, and the honest limit that has to be stated here.**
`scripts/evidence-config.sh:22` gives the `installer_smoke` suite the parser
`bash gate-sdk/bin/run-gates.sh --emit parse-smoke-log …`, and the custom-parser arm at
`native/src/evidence.rs:114-122` never references the process status it is handed. So the
evidence row is derived from the log's arm headers alone, and **exit 1 and exit 2 are
indistinguishable to it**. The distinction is real for CI's job verdict and for a human reading
the log, and it is invisible to `--run-validate`'s evidence. This amendment changes nothing about
that and asserts nothing that depends on it — it is recorded because the natural reading of delta
3 is that a suite consumes the verdict class, and that reading is false.

**Existing integration prose describing the prior flow, updated rather than left to drift.**
The sampling rule at `installer/README.md:1395`, the after-the-report paragraph at `:1463-1479`
and the truth table's shape row at `:1457` each describe the arm as it stands; delta 4 rewrites
each in place. The `# spec:` comments at `run-smoke.sh:324`, `:329` and `:332` narrate the same
three decisions from the code side and move with them.

**No knob is minted by any delta**, so no `<KIT>_<KNOB>` roster moves and no config seam changes.
The provenance seam is not approached: every delta is inside this repo's own acceptance harness
and its owner doc, `run-smoke.sh` being declared `no-port` precisely because it rides no payload
and reaches no adopter.

**Coherence with the sibling amendment on the same two surfaces.**
`SPEC-printed-followup.md` adds an assertion to `assert_install()` in the same file and a
sub-heading to the same section. The two touch disjoint regions — this one `:319-336` and
`manifest_report()`, that one the block immediately after `:280` — and they share one rule, which
both state and neither invents: a finding about the consumer's payload exits 1 through `fail`, a
failure of the harness's own construction exits 2 through `blocked`. Whichever lands first, the
other rebases on it rather than on this text.

## Existing sections updated

- `installer/README.md` §The consumer smoke — the sampling rule at `:1395` (deltas 2 and 4); the
  after-the-report paragraph at `:1463-1479` (deltas 3 and 4); the truth table's shape row at
  `:1457` (deltas 1 and 4); the round record at `:1534-1625`, which gains the drain sentence and
  loses nothing (delta 4); and the new run-wide-verdict rule (delta 4).
- `installer/consumer-smoke/run-smoke.sh` — the manifest arm at `:319-336`, `manifest_report()`
  at `:197-268`, and the three `# spec:` comments at `:324`, `:329` and `:332` (all deltas).
<!-- update-target-exempt: the drop condition is a green observation this amendment expressly declines to make -->
- `.github/workflows/gates.yml` — **deliberately untouched**; the `continue-on-error` line's own
  comment conditions its removal on the run the leg is first observed green, and green is what
  this envelope does not assert.
- `TASK-QUEUE.md` — `consumer-smoke-manifest-verdict-outruns-its-report` promotes to New Features
  with this file's `[spec:]` ref, in the same commit as this file (all deltas).

## Definition of Done

- [ ] **Causal completeness** — the witness tuple and the count each have a named producer on an
      already-live path, named consumers at named transitions, and no unread field.
- [ ] **The verdict's row is in the report** — no run can exit 2 on an entry the report did not
      print, and a coincidence with an existing sample is stated rather than collapsed.
- [ ] **The refusal names its operand** — path, which operand failed, and the count, readable from
      the last line without scrolling.
- [ ] **The sampler kept its other two members** — the first disagreeing path and the artifact
      control are both still printed.
- [ ] **The rule is stated, not just the instance** — §The consumer smoke carries the run-wide
      verdict rule, and round 15's open finding reads as drained rather than outstanding.
- [ ] **Nothing about the diagnosis moved** — no delta, and no merged sentence, reopens or
      re-rules why rounds 12 to 15 disagree, and none claims the leg green.
- [ ] **The evidence-side limit is on the page** — the merged section says the suite's parser
      cannot distinguish exit 1 from exit 2, so no later reader infers that it can.
- [ ] **Battery and suite green** — `bash gate-sdk/bin/run-gates.sh` plus
      `bash gate-sdk/bin/build-native.sh`, neither discharging the other, and the
      `installer_smoke` suite clean on this host — which does not exercise the failure-path report,
      a limit §The consumer smoke already owns and states rather than gates.
- [ ] **Merged with no information lost** — each addition integrated into its proper canonical
      section, not appended.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component
      (`ls installer/SPEC-*.md`), discharged at the iteration since a sibling amendment is in
      flight for the same component.
- [ ] **Removals propagated** — grepped for the retired "at most two paths" claim and for the old
      class-description refusal string; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps found during the work filed to the gap inbox.
