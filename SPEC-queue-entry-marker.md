# SPEC amendment: queue-entry-marker

**A queue entry cannot say which of its premises nobody ran.** An entry filed at
close-stage speed and an entry filed at full evidence read the same to the session
that later builds on them. The re-verification that catches a false premise all
happens downstream. It worked five times out of five across one iteration, but only
by re-buying every premise, because nothing marked the ones worth probing. The
amendment side of this already landed. An amendment passage now carries the inferred
marker (lifecycle-kit/SPEC.md §templates/stages/), and `check-stage-entry` assertion D
refuses the build entry while a not-run marker remains. A queue entry has neither the
grammar nor a consumer.

**Shape, decided by the intent oracle (scope's reading, lead-relayed 2026-09-17;
revisable, not operator direction):** reuse the inferred marker in queue-entry
bodies, and widen assertion D to the active sections. Two alternatives were weighed
and not chosen. A template convention with no gate repeats the entry's own point
(7): an annotation nobody consumes goes unused. Declining drops a unit the operator
put in the set.

**The half this does not cover, and why it needs nothing new.** The entry's older
subject was that nothing signals an entry was *compressed*. That half is answered by
the `entry-history` arm (queue-kit/SPEC.md §check-queue-entry-budget), which reports
the commits where one entry's counted extent fell, with no verdict. This amendment
adds nothing there.

**What stays as it is.** The marker's two spellings and its line grammar. Assertions
A to C and every knob. The gap inbox's refusal of a capture-time grammar
(lifecycle-kit/SPEC.md §The committed gap inbox): a bullet stays two fields, and the
marker is written only where a deferred entry is written, which is where that
refusal's ground stops ("reaches no surface where capture is not cheap or no drain
re-verifies").

## What changes

### (1) The marker reaches queue-entry bodies

§The committed gap inbox's filing paragraph and §templates/stages/'s marker paragraph
extend the inferred marker to queue entries. Whoever writes a deferred entry marks
each load-bearing claim it did not run {design-bearing}. **Not yet applied.**

- **Where.** In a queue entry's body, on a continuation line of its own, indented,
  with **no `- ` lead**. With the lead, the line would parse as a sub-task whose bold
  lead-in is not a slug. Probed 2026-09-17 on a scratch queue: `check-task-names` reds
  on it. The marker's own grammar (line start, optional indentation, never in a
  fence) is unchanged. The no-lead rule is a queue-format consequence, stated where
  the queue form is described.
- **Who writes it.** Every session that writes a new deferred entry is already bound
  by §The committed gap inbox's "Filing looks for an owner first": a drain's or
  intake's →promote, close's own filings, and a finding validate files as a task. A
  claim its disposition turns on that the session did not run takes
  `**Inferred, not run:** <claim> — <command>`, or the cannot-run form with its
  reason. This replaces the drain step's "let the promoted entry carry the unverified
  premise openly" with a spelling that has a consumer.
- **Not at capture.** A gap-inbox bullet gains nothing. The marker is written when
  the drain turns the bullet into an entry, so the inbox's refused-grammar ground
  stands untouched.
- **Deferred and icebox entries are not read by any gate.** In the pool the marker
  is the carried signal. An eviction to the icebox drops the body and the marker with
  it. The icebox tier's mandatory recovery before any ruling
  (queue-kit/SPEC.md §The icebox tier) brings the marker back with the body.
- **Against the entry cap.** A marker line counts toward
  `check-queue-entry-budget`'s assertion A like any body line. The discount covers
  only declaration grammars the queue format defines (`recurrence:`,
  `not-icebox-eligible:`), which are fixed-shape record lines. A marker is a filing
  premise of free width, which is content. Counting it also points the right way: an
  entry near its cap pays a line for an unrun premise, and running the premise at
  filing costs nothing against the cap.

### (2) The stages that promote an entry run its markers first

The promoting stage runs an entry's not-run markers before it rules on or promotes
the entry. For debt that stage is scope. For a feature it is the authoring stage that
pairs the entry {mechanical}. **Not yet applied.** Replacement text, instruction
only:

- **`lifecycle-kit/templates/stages/scope.md`**, the paragraph opening "A premise
  inherited from a queued task", re-phrased:

  > A premise inherited from a queued task ("clean/mechanical", "already filed",
  > "dead code") is a dated hypothesis — re-verify it against the current tree
  > before building on it, whether or not this stage goes on to author. On every
  > entry you promote as debt, run each `**Inferred, not run:**` marker's command,
  > correct the entry to what it returned and delete the marker, or rewrite it to
  > the cannot-run form with its reason.

- **`lifecycle-kit/templates/stages/spec.md`**, the authoring-exit paragraph's
  sentence "That includes every claim a survey block you rely on lists under
  `inferred`." becomes: "That includes every claim a survey block you rely on lists
  under `inferred`, and every inferred marker on the entry you pair — run it and
  delete it there, as on a passage."
- **`lifecycle-kit/templates/stages/align.md`**, the claim-check paragraph's "Run
  every `**Inferred, not run:**` marker's command" becomes "Run every
  `**Inferred, not run:**` marker's command, in an amendment or an active queue
  entry".
- **`lifecycle-kit/templates/stages/close.md`**, the drain's re-verification step:
  "let the promoted entry carry the unverified premise openly" becomes "carry it on
  the promoted entry as the inferred marker (lifecycle-kit/SPEC.md
  §templates/stages/)".
- **`lifecycle-kit/templates/stages/build.md`**, the run-the-system sentence: "An
  amendment passage marked" becomes "An amendment passage or active queue entry
  marked".

**Why scope owes debt and not only D.** D fires at the audit-entry stage. A feature
meets the authoring and audit stages first. A debt entry promoted straight into an
active section meets no stage between scope and D, so without the scope sentence
the first reader of its marker is the refusal. That is the failure §check-stage-entry
already names for amendments when it says both align and D carry the obligation.

### (3) Assertion D widens to the active queue sections

§check-stage-entry's (D) clause and the "Assertion D reads C's corpus and nothing
else" paragraph are re-phrased. D reads the configured active queue sections as well
as the amendment tree {design-bearing}. **Not yet applied.**

- **Corpus.** C's amendment walk, unchanged. **Plus** every line inside a top-level
  entry of a section named by `LIFECYCLE_KIT_ACTIVE_SECTIONS`, in
  `LIFECYCLE_KIT_QUEUE_FILE`. These are the file and sections assertion B already
  reads. The line grammar and the fence rule are the ones D applies to an amendment.
- **The corpus property, restated rather than silently widened.** The sentence "reads
  C's corpus and nothing else … so it needs no knob and no `--reads` root of its own"
  becomes: **D reads C's amendment walk and B's queue read, and nothing else.** Both
  are already this gate's reads, through `LIFECYCLE_KIT_AMENDMENT_GLOB` and through
  `LIFECYCLE_KIT_QUEUE_FILE` with `LIFECYCLE_KIT_ACTIVE_SECTIONS`. So D still needs no
  knob and no `--reads` root of its own. The queue read is a named file rather than a
  walk, which gate-sdk/SPEC.md §check-reads-couples rules outside the walk class. The
  descriptor's `couples=` already carries `TASK-QUEUE.md`.
- **Red.** A queue marker prints `<queue file>:<line>: <marker line>`, in the shape an
  amendment marker prints. The help line names the remedy for both: run the command,
  correct the passage or entry and delete the marker, or rewrite it to the cannot-run
  form with a reason.
- **Clean.** The `N cannot-run claim(s) carried` count sums both corpora.
- **Not read:** the deferred section, the icebox and the done section, per delta 1.
- **Honest limit, added:** a queue file whose active-section names are misconfigured
  reads no entries for D, exactly as for B, and nothing extra is asserted about it.
- **`native/src/gates/stage_entry.rs`.** D's scan takes the queue text B already
  loads, walks the active sections' entry extents, and applies the existing
  `inferred_marker` and fence toggle. Unit tests cover a marker in an active entry, a
  marker in a deferred entry (unread), and a fenced marker in an active entry
  (unread).
- **`lifecycle-kit/gate-tests/check-stage-entry.test.sh`** gains two sandbox
  scenarios. A not-run marker on an active debt entry at audit-entry-stage entry is
  red and names the queue line. The same marker on a deferred entry is clean.

### (4) queue-kit states the body line and its cost

§check-queue-entry-budget's calibration gains one sentence. §The queue format's
deferred-body paragraph names the marker line beside the four recurring fields
{mechanical}. **Not yet applied.**

- **§The queue format**, after the four bold-lead-in fields: "A body may also carry
  lifecycle-kit's inferred marker on a line of its own, with no `- ` lead
  (lifecycle-kit/SPEC.md §templates/stages/); queue-kit reads it nowhere."
- **§check-queue-entry-budget calibration:** "A marker line is a counted body line —
  it is content rather than a declaration grammar this format defines."

## Producers and consumers

- **A queue-entry marker (delta 1).** Producer: every session writing a new deferred
  entry, as rostered by §The committed gap inbox's filing paragraph. Consumers: scope
  at a debt promotion and the authoring stage at a feature pairing (delta 2), align
  over the active sections, build for a cannot-run marker, and D (delta 3), by line
  scan. Fields: `<claim>` is read by the running stage as the text to correct, and
  `<command>` as the thing to run. D reads presence (and a cannot-run reason's
  non-emptiness) and prints the whole line.
- **D's widened read (delta 3).** Producer of a verdict only. Its enabling config is
  `LIFECYCLE_KIT_AUDIT_ENTRY_STAGE` (derived `build` in this repo), plus B's queue
  knobs, set in every deployed configuration that runs B. Consumers:
  `--enter-stage` at the refusal, the pre-commit battery while the cursor sits at the
  audit-entry stage, and the lead's `--enter-stage --simulate` read.
- **Roster-holding readers.** No name is minted. The marker spellings already sit in
  the `spec`, `align` and `build` templates. `check-shim-restatement` reads those
  templates, and no binding repeats them. New `// spec:` comments in
  `stage_entry.rs` bind to §check-stage-entry.
- **Point 5 (narrowing).** Nothing narrows. D gains a red condition over a corpus
  this gate already reads.
- **Point 6 (members).** The newly obliged corpus is the lines of active-section
  entries in `TASK-QUEUE.md`. Probed with
  `grep -nE '^\s*(- |> )?\*\*Inferred, (not run|cannot run before build):\*\*' TASK-QUEUE.md`
  on 2026-09-17: no match anywhere in the file. So every current entry's satisfying
  value is "no marker". This iteration's paired features carry none.

## Existing sections updated

- `lifecycle-kit/SPEC.md` §The committed gap inbox: the filing paragraph gains the
  marker obligation and the not-at-capture boundary (delta 1).
- `lifecycle-kit/SPEC.md` §templates/stages/: the marker paragraph gains the queue
  site, the no-lead rule, and the unread pool (delta 1). The spec, scope and align
  paragraphs gain the promoting-stage consumption and why scope owes debt (delta 2).
- `lifecycle-kit/SPEC.md` §check-stage-entry: the (D) clause, the corpus-property
  paragraph, red, clean and honest limit (delta 3).
- `lifecycle-kit/templates/stages/scope.md`: the inherited-premise paragraph
  (delta 2).
- `lifecycle-kit/templates/stages/spec.md`: the authoring-exit sentence (delta 2).
- `lifecycle-kit/templates/stages/align.md`: the claim-check sentence (delta 2).
- `lifecycle-kit/templates/stages/close.md`: the drain's re-verification sentence
  (delta 2).
- `lifecycle-kit/templates/stages/build.md`: the run-the-system sentence (delta 2).
- `native/src/gates/stage_entry.rs`: D's queue read, help line, clean count and unit
  tests (delta 3).
- `lifecycle-kit/gate-tests/check-stage-entry.test.sh`: two scenarios (delta 3).
- `lifecycle-kit/checks/check-stage-entry.gate`: the `# spec:` description names the
  queue half of the inferred-claim residue (delta 3).
- `queue-kit/SPEC.md` §The queue format and §check-queue-entry-budget (delta 4).
- `.workflow/release-declarations.md`: one bullet for the widened assertion and one
  for the five template sentences (deltas 2 and 3).
- `TASK-QUEUE.md`: `queue-entry-evidence-tier` moves to Done at merge, by the build
  session that merges this file, before the drain stage is entered (all deltas).
- `docs/lifecycle-kit/SPEC.md`: generated mirror, regenerated (all deltas).
- `docs/queue-kit/SPEC.md`: generated mirror, regenerated (all deltas).
- <!-- update-target-exempt: generated projections, each rostered with its freshness gate and regen command in docs/site-architecture.md §Generated projections and their freshness gates --> `docs/enforcement.md` and the generated pre-commit hook.

Roster produced by `grep -n "Inferred"` over `lifecycle-kit/templates/stages/`,
`grep -n "inferred"` over `lifecycle-kit/SPEC.md`, and `grep -n "assertion D"` over
`lifecycle-kit/` and `native/src/gates/stage_entry.rs`. It is a floor that build
re-derives.

**Merge coordination.** The sibling unit `design-pending-tag-restates-its-own-section`
edits `close.md`'s →promote sentence and `scope.md`'s exit-condition example, next to
two of this file's edits. Whichever batch lands second rebases onto the first.

## Retired spellings

- None — no delta retires a spelling; the drain sentence and the corpus-property
  sentence are re-phrased prose, not names any surface matches.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit's causal-completeness check
      holds for the queue-entry marker and D's widened read.
- [ ] **Instruction surfaces: instruction only** — the five template edits carry no
      grounds; deltas 1 to 3 place them in lifecycle-kit/SPEC.md.
- [ ] **Merged with no information lost** — the not-at-capture boundary, the
      counted-line ground, why scope owes debt, and the restated corpus property
      survive in the merged prose.
- [ ] **Attested reproduction** — a sandbox with a not-run marker on an active debt
      entry refuses `--enter-stage build`. The same marker moved to a deferred entry
      enters.
- [ ] **Queue move placed before the drain stage** — the Done move lands in the
      session that merges this file, before the drain stage is entered.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the repo
      root (`ls SPEC-*.md`) once the iteration's last batch lands.
- [ ] **Gaps filed** — cross-component gaps found during the work go to the gap inbox.
