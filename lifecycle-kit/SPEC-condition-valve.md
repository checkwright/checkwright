# SPEC amendment: condition-valve

Queue entry: `ruling-staleness-header-reads-as-undeclared`. Rides unit set
`carried-record-reliability` under **operator direction, 2026-09-13, lead-relayed**.

## The question, restated with its premise corrected

`--emit ruling-staleness` prints one permanent row under `== undeclared conditions ==`:
the ruling record's own framing paragraph, which declares the record's contract and is no
ruling. A row that is always present trains its reader to skip the one section built to
surface a ruling somebody forgot to declare.

**The entry's mechanism is wrong, and the correction moves the design.** It says the probe
"partitions the file into paragraphs and demands a declaration of each". It does not. The
undeclared pass in `native/src/emit/ruling_staleness.rs` reports a paragraph only when the
paragraph matches the arm's forward-looking phrase list and carries no readable `discharge:`
line. Measured at this stage: the framing paragraph matches on `until that`, from *"each an
override of a business-as-usual instruction until that instruction is updated"*. A paragraph
with no such phrase already reports nothing.

So the row is not a boundary bug in a partition. It is the weak detector doing what the
section says it does, on a paragraph that *defines* a ruling. That definition is the steering
vocabulary's own ("overrides a business-as-usual instruction until the instruction is
updated", §The steering vocabulary). Any consumer whose record opens by saying what a ruling
is will meet the same row.

Three dispositions were weighed. One is taken.

- **Refused: the probe skips text above the first `##` heading.** This makes the record's
  heading structure the probe's business, which the section rules out when it says the arm
  "reads nothing else" than its two declarations and three knobs. It also fails in the unsafe
  direction. A consumer whose record puts a real conditioned ruling above its first heading
  gets that ruling skipped silently, where today it gets a false positive that costs one line.
  The undeclared closure exists so a hole reports instead of being inherited.
- **Refused: narrow the phrase list, or reword the header so it stops matching.** That is the
  narrow-the-matcher shape §The committed gap inbox refuses for the same reason: an arm you
  must phrase around to stay accurate is miscalibrated, and the next author does not know to
  phrase around it. `until that` is also a true signal. It is the ruling definition's own
  wording, so dropping it loses the rulings that state their condition in the vocabulary's
  terms.
- **Taken: the record says it on the paragraph.** A valve on the paragraph itself, with a
  mandatory reason, excuses that paragraph from the undeclared pass and from nothing else. The
  fact lands on the governed surface, the same way the record already declares its own
  close-surface row inline. The probe stays shape-agnostic, and the fail direction is the safe
  one: a paragraph nobody valved still reports.

## The seam

- **Kit mechanism:** the valve's spelling and its reading in the arm.
- **Consumer content:** each valve's reason text, and which paragraphs of a consumer's record take
  a valve. The record path stays the existing `LIFECYCLE_KIT_RULING_RECORD` knob.
- **This repo's own:** the `TRAJECTORY.md` valve line.
- **Private rule content:** none in reach.

## What changes

### (1) The undeclared pass honours a per-paragraph valve with a mandatory reason {design-bearing}

`lifecycle-kit/SPEC.md` §The ruling-staleness probe gains a paragraph directly after
**An undeclared condition is reported, not inherited.** **Not yet applied:**

> **A paragraph that states the record's contract takes a valve, and the valve excuses it from
> this pass alone.** A record that opens by defining what a ruling is will match the detector,
> because the definition carries the forward phrasing a ruling carries. That paragraph takes one
> line, anywhere inside it:
>
> ```
> <!-- undeclared-condition-exempt: <why this paragraph is no ruling> -->
> ```
>
> The **reason is mandatory**. A valve with no reason reports in the malformed band and does
> **not** excuse its paragraph, on the rule the band already states: a malformed declaration must
> not buy the skip it failed to justify. The valve reaches the undeclared pass only. It is no
> declaration of a ruling, so the discharge report, the citing report and the `ruling:` name
> lookup never read it. It is per **paragraph** because the paragraph is the pass's unit.
> Structural answers are refused. The probe does not skip a record's preamble by heading
> position, which would couple it to the record's shape and silently drop a real ruling placed
> there. The detector's phrases are not narrowed either, which is the evasion §The committed gap
> inbox refuses. The valve puts the fact on the record, where the author who wrote the paragraph
> can state it.

The valve line is found by the same reading the two declarations get: the fence skip, and a
leading-whitespace trim. So a valve quoted inside a fenced block, like the one above, is
quotation and excuses nothing.

### (2) The arm implements the valve and a malformed-valve row {design-bearing}

`native/src/emit/ruling_staleness.rs` changes in three places:

- `undeclared` drops a paragraph that carries a **well-formed** valve line, in the same filter
  that drops a paragraph carrying a readable `discharge:` line.
- `malformed` emits one row for a valve line whose reason is empty after trimming. The row names
  its site and says what the grammar wanted: `<!-- undeclared-condition-exempt: <reason> --> — a
  non-empty reason`. The paragraph still reports as undeclared.
- The module's `#[cfg(test)]` block gains three cases: a valved conditioned paragraph is
  excused; a reasonless valve reports malformed and leaves its paragraph a hole; and a valve
  inside a fence excuses nothing.

The valve spelling is kit mechanism, one literal beside `FORWARD_PHRASES`. It carries no
consumer vocabulary, and the reason text is the consumer's. The arm is a reporting arm and owes
no fixture pair (§The ruling-staleness probe's opening paragraph), so the unit tests are the
coverage, run by `check-crate-arms`. `lifecycle-kit/gate-tests/ruling-staleness.test.sh` covers
the bridge seam only and is unchanged.

### (3) This repo's ruling record valves its framing paragraph {mechanical}

`TRAJECTORY.md`'s opening paragraph ("This file carries two things and shrinks toward empty…")
gains one valve line as its first line. **Not yet applied:**

```
<!-- undeclared-condition-exempt: the record's contract, defining what a ruling is; it directs no work and carries no condition of its own -->
```

This is no ruling edit. The paragraph is the file's framing, and none of the record's three acts
reaches it. **Build may add this line: operator direction, 2026-09-13**, given in the lead
session and relayed to spec. The oracle for the delta is the arm itself: after the edit, `bash
gate-sdk/bin/run-gates.sh --emit ruling-staleness` prints `(none)` under `== undeclared
conditions ==` on this tree. Before the edit it prints the one `TRAJECTORY.md` row.

## Producers and consumers

- **The valve line** (new state on a consumer's ruling record). *Producer:* the session writing
  or maintaining the record. Here that is build for delta 3, and after merge any session that
  adds a framing paragraph. No enabling config is needed beyond `LIFECYCLE_KIT_RULING_RECORD`,
  which this repo sets (`scripts/lifecycle-config.sh`). *Consumer:* the `--emit ruling-staleness`
  arm's undeclared pass, which reads the line in process at report time.
- **Its one field, the reason.** *Reader:* the malformed pass, which checks it for emptiness at
  report time. A human reader of the record reads its text, the same as the other `-exempt:`
  valves' reasons.
- **The malformed-valve row** (new output row). *Producer:* the arm's `malformed` pass.
  *Consumer:* close's ruling-record repair step (`lifecycle-kit/templates/stages/close.md` step
  7), which reads the whole report and so already reads the malformed band. The template needs
  no edit, because the band's contract (one row per unreadable declaration, with its site and
  what the grammar wanted) already covers a valve. This repo's scope ritual
  (`.claude/commands/scope.md`) reads the discharge report alone, which this amendment leaves
  unchanged.
- **Red conditions (point 5).** The arm is a reporting arm and reds nothing. Its exit status
  carries no verdict. The delta narrows the undeclared report's corpus, and that report's only
  reader is the close step above, which acts per row. No reader asserts a count, a minimum or
  finding-none over it. `ruling-staleness.test.sh` greps for the `undeclared conditions` section
  *heading*, which the arm always prints, so that check stays green whether or not any row
  survives.

## Existing sections updated

- `lifecycle-kit/SPEC.md` §The ruling-staleness probe — the valve paragraph (delta 1), and the
  sentence opening **A malformed declaration reports as malformed**, which widens its "both
  declarations" to cover the valve's missing reason (delta 1).
- `native/src/emit/ruling_staleness.rs` — the undeclared filter, the malformed pass and the unit
  tests (delta 2).
- `TRAJECTORY.md` — the framing paragraph's valve line (delta 3).
<!-- update-target-exempt: generated mirror regenerated by its own freshness gate at commit, no delta edits it by hand -->
- `docs/lifecycle-kit/SPEC.md` — the site's generated mirror of the kit SPEC. It regenerates
  when delta 1 merges, and its freshness gate prints the regen command on red.

## Retired spellings

- None — no delta retires a spelling; the valve is an addition and the phrase list is unchanged.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Instruction surfaces: instruction only** — replacement text for a
      template, agent definition or shim carries no grounds; a delta places them
      (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls <component>/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in
      `## Retired spellings` above, and `check-amendment-retired-spelling` runs
      each declaration against the whole tracked tree, not against the specs
      alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **The arm reports clean on this tree** — `--emit ruling-staleness` prints `(none)` under
      undeclared conditions after delta 3.
