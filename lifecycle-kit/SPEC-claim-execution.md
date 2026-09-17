# SPEC amendment: claim-execution

**An amendment reaches build carrying claims nobody ran, and the stage that wrote
them is the cheapest place to run them.** Two filed failures share that shape.
First, spec-stage amendments assert tree facts that a one-command probe refutes,
and a later stage catches each one. Second, a survey can mark a claim
inferred-not-executed, and nothing obliges any later stage to execute it. In the
attested instance, a survey's `inferred` field named the arm to run, the amendment
restated the claim as fact, align returned zero defects, and build measured it
false. Align already had a rule to check a claim about a program against that
program. The rule was a prompt, and the prompt missed.

**Two deliverables, one mechanism.** An **authoring-exit pass** in the `spec`
template runs each delta's own checks before the amendment is committed. An
**inferred marker** on the passage carries every load-bearing claim the pass could
not run. `check-stage-entry` refuses the build entry while an unrun marker remains.
The pass is bounded by the amendment's own text, not by the whole tree. The marker
is what turns the survey record's disclosure into something a stage must consume.

**What stays as it is.** The survey record's grammar and `check-survey-record`.
Assertions A, B and C of `check-stage-entry`, and every knob. Canon-kit's amendment
template and its gates: the marker is this kit's authoring-template spelling, as
the work-class tag is. Align's trigger set.

## What changes

### (1) The spec template gains an authoring-exit pass over each delta's own claims

`lifecycle-kit/templates/stages/spec.md` gains a paragraph, placed after the
causal-completeness paragraph, that runs each delta's predicate and its
load-bearing premises before the amendment is committed {design-bearing}.
**Not yet applied.** Replacement text (instruction only):

> **Before committing an amendment, run each delta's own claims.** Run the
> command that tells whether the tree already does what the delta instructs; a
> delta the tree already satisfies is rewritten or deleted, never shipped. Then,
> for each premise the delta's ruling or reach rests on — what a program does, what
> a file holds, a count, which readers a surface has — run the one command that
> settles it and correct the delta to what it returned. That includes every claim a
> survey block you rely on lists under `inferred`. A premise no command settles
> before build takes the inferred marker on its passage. An illustration the delta
> does not rest on is exempt. The marker grammar and why the pass is bounded this
> way: lifecycle-kit/SPEC.md §templates/stages/.

**The three classes, and what the pass reaches in each.** The filed recurrence
separates them, and their costs differ by an order of magnitude.

- **A spent delta** is an instruction whose predicate the authoring commit already
  satisfied. The pass catches it outright, since it is the one class fully visible
  from the commit that wrote it. Its hazard is inverted: a build session trusting
  it hunts a discrepancy that does not exist, or "corrects" a correct figure.
- **A premise** is load-bearing: a ruling's ground, or a contract sentence cited
  to a section. The pass reaches every premise the author recognizes as one, by
  running it or by marking it. What stays uncaught is a premise the author
  believed verified. That class is strictly smaller than the one uncaught without
  the pass, which is the survey record's `inferred` argument applied here.
- **An illustration** moves no oracle row, and its delta's ruling stands on its
  own derivation. It is exempt, because probing it buys nothing a reader acts on.

**Refused: probing every tree fact the amendment asserts.** That is open-ended,
and most asserted facts are illustrations. The pass is bounded by the deltas'
predicates and the premises their rulings name.

### (2) The inferred marker: the passage-level carry of an unrun claim

An amendment passage turning on a claim nobody ran carries one of two markers,
each opening its own line {design-bearing}. **Not yet applied.**

- `**Inferred, not run:** <claim> — \`<command>\`` — the claim and the command
  that would settle it. The command is mandatory, since the marker exists to
  hand a later stage something to run.
- `**Inferred, cannot run before build:** <claim> — <reason>` — a claim whose
  subject does not exist until build lands something, with the reason. The reason
  is mandatory.

**Grammar.** A marker is recognized only at the start of a line, after optional
indentation and an optional `- ` or `> ` lead. It is not recognized inside a
fenced block. A mention of the spelling in running prose is therefore not a
marker, because it sits mid-line or in backticks. The spellings are **kit
constants, not config**: they belong to this kit's authoring template, on
canon-kit's precedent that a kit-shipped template's own headings are constants.

**Why on the passage and not only in the survey record.** The survey record is
boundary-truncated and holds a claim per block, not per passage. An amendment
restating a claim loses the record's `inferred` flag at the restatement, which is
the attested path. The marker sits where the next reader reads, which is the rule
the "Not yet applied" marker already follows.

### (3) Align runs every marker; build runs the cannot-run ones first

Align's claim-check paragraph and build's run-the-system paragraph each gain the
marker's consumption {mechanical}. **Not yet applied.**

- **`align.md`**, the paragraph opening "**A claim about an existing program is
  checked against that program**", re-phrased (instruction only):

  > **A claim about an existing program is checked against that program** — a
  > modelled grammar, a check a gate applies, a refusal an arm makes: one
  > invocation, or the implementing line where none is cheap. If the named program
  > lacks the behaviour, find the one that holds it. Run every
  > `**Inferred, not run:**` marker's command, correct the passage to what it
  > returned and delete the marker, or rewrite it to the cannot-run form with its
  > reason.

- **`build.md`**, the "**Run the system; don't reason about it**" paragraph, one
  sentence after its second one (instruction only):

  > An amendment passage marked `**Inferred, cannot run before build:**` is run as
  > soon as its subject exists, before the delta resting on it lands, and the
  > merge carries what it returned.

**Why both, when delta 4 already blocks.** Align is trigger-gated and can be
skipped. Delta 4 is what holds the obligation when it is. Align's sentence names
the stage that meets a marker first when align does run, so the refusal is not
the first time the marker is noticed.

### (4) check-stage-entry assertion D refuses the build entry on an unrun marker

`check-stage-entry` gains assertion **D, inferred-claim residue**. At entry to
`LIFECYCLE_KIT_AUDIT_ENTRY_STAGE`, any on-disk amendment carrying an
`**Inferred, not run:**` marker, or a cannot-run marker with an empty reason, is a
refusal {design-bearing}. **Not yet applied.**

- **Corpus.** Assertion C's amendment walk and nothing else: the gate-sdk prune
  set, `templates/` paths excluded, basenames matching
  `LIFECYCLE_KIT_AMENDMENT_GLOB`. D reuses C's file set, so no new knob and no new
  `--reads` root is needed. D runs whether or not an audit stamp exists, because
  the audit stamp proves the audit ran and not that the markers were consumed.
- **Red.** Exit 1, one error line per marker, `<file>:<line>: <marker line>`. The
  help line names the remedy: run the command, correct the passage and delete the
  marker, or rewrite it to the cannot-run form with a reason. The fix is made at
  the stage the refused entry leaves the cursor at. A refused `--enter-stage`
  writes nothing, so that stage is the authoring or the audit stage.
- **Clean.** No marker, or only cannot-run markers with a reason. The clean detail
  adds `N cannot-run claim(s) carried` when N > 0, which is the count build's
  delta 3 sentence consumes.
- **Inert** where `LIFECYCLE_KIT_AUDIT_ENTRY_STAGE` is empty, meaning no audit
  stage is configured. **The honest limit:** such a roster gets the template
  obligations without the backstop. A dedicated knob for the implementing stage
  would add a name for one assertion, which the knob roster refuses without an
  attested consumer.
- **Not an assertion-C sibling for the authoring stage.** §check-stage-entry
  refuses that sibling because an authoring stage's *output* is already verified
  by the pairing rule. D asserts something the pairing rule never reads, a
  property of the amendment's content, so the refusal's ground does not reach it.
- **Why a block and not an annotation.** An annotation already existed, the
  survey record's `inferred` field, and the attested failure is that nothing
  consumed it. A marker that blocks nothing reproduces that failure one surface
  later.
- **Not gated, and stated so:** whether the authoring-exit pass ran, and whether
  an unmarked claim is true. The first leaves no tracked residue. The second is the
  premise the author believed verified, which delta 1's argument concedes.

**Fixtures.** `gate-tests/check-stage-entry.test.sh` gains four sandbox scenarios:
a not-run marker at audit-entry-stage entry (red, and it names the file and line),
a cannot-run marker with a reason (clean, and the count appears), a cannot-run
marker with an empty reason (red), and a not-run marker inside a fence, in running
prose mid-line, and in a `templates/` stub (clean). The good/bad pair covers
assertion A and stays as it is.

### (5) The survey record's carry-onward sentence names the marker

§The survey record's witness paragraph, the sentence "A claim in its `inferred`
field is not carried by the witness: re-establish it before your work turns on
it, or carry it onward as inferred", is re-phrased so that carrying onward into an
amendment means the passage's inferred marker (§templates/stages/)
{mechanical}. **Not yet applied.**

## Producers and consumers

- **The authoring-exit pass (delta 1).** Producer: the `spec` stage session, and
  `scope` where a default roster authors in scope, since scope's authoring step
  points at `spec.md`'s how-to (§templates/stages/). Its outputs are a corrected
  amendment and zero or more markers. Consumer of its outputs: deltas 3 and 4.
- **The not-run marker (delta 2).** Producer: the authoring session at
  authoring exit. Consumers: the align session (delta 3), and assertion D at
  audit-entry-stage entry (delta 4), by line scan. Fields: `<claim>` is read by
  align and build as the text to correct; `<command>` is read by align as the
  thing to run. D reads the marker's presence alone and prints the whole line, so
  a reader of the refusal gets both fields.
- **The cannot-run marker (delta 2).** Producer: the authoring or align session.
  Consumers: build (delta 3), which runs the claim once its subject exists; D,
  which reads the reason's non-emptiness and counts the markers into its clean
  detail. `<reason>` is read by D (empty or not) and by build (why it waited).
- **Assertion D (delta 4).** Producer of a verdict only. Consumers: `--enter-stage`
  at the refusal, the committing session through the pre-commit battery while the
  cursor sits at the audit-entry stage, and the lead's `--enter-stage --simulate`
  pre-completion read. Its enabling config is `LIFECYCLE_KIT_AUDIT_ENTRY_STAGE`,
  which is derived as `build` wherever the default `align` audit stage is set,
  this repo included.
- **Roster-holding readers of the minted names.** The marker spellings land in
  amendment bodies, which no prose or knob gate scans (canon-kit's §The amendment
  lifecycle puts amendments outside the governed manifest), and in the `spec`,
  `align` and `build` templates, which `check-shim-restatement`'s dedup corpus
  reads for n-gram overlap with bindings. No binding repeats them. D's new
  `// spec:` comments bind to §check-stage-entry and meet `check-comment-tier`'s
  existing directive roster.
- **Point 5 (narrowing).** Nothing narrows. D adds a red condition over an
  existing corpus.
- **Point 6 (members).** At authoring time the obliged corpus is the on-disk
  amendments, enumerated by `find . -name 'SPEC-*.md' -not -path '*/templates/*'`
  under the prune set: this file and `lifecycle-kit/SPEC-differential-sweep.md`,
  plus fixture copies under `gate-tests/`, which the prune set removes. Neither live
  amendment carries a marker line, so each one's satisfying value is "no marker".
  This file names the spellings only mid-line, in backticks or behind `> `
  followed by prose. **Build verifies that the `> ` lead does not make delta 3's
  quoted template lines read as markers**: the quoted sentences open with prose,
  not with the marker, so they are not markers under delta 2's grammar.

## Existing sections updated

- `lifecycle-kit/templates/stages/spec.md`: the authoring-exit paragraph (delta 1).
- `lifecycle-kit/templates/stages/align.md`: the claim-check paragraph (delta 3).
- `lifecycle-kit/templates/stages/build.md`: the run-the-system paragraph
  (delta 3).
- `lifecycle-kit/SPEC.md` §templates/stages/: the `spec.md` paragraph gains the
  pass's grounds, the three classes and the refused whole-tree probe, and the
  marker grammar with its passage-not-record ground (deltas 1 and 2). The `align`
  claim-check paragraph gains the marker-consumption ground and why both align
  and D carry it (delta 3).
- `lifecycle-kit/SPEC.md` §check-stage-entry: "three assertions" becomes four,
  with D's corpus, red, clean, inert and honest-limit clauses and why it is not the
  refused authoring-stage sibling; the test paragraph gains the four scenarios
  (delta 4).
- `lifecycle-kit/SPEC.md` §The survey record: the carry-onward sentence
  (delta 5).
- `native/src/gates/stage_entry.rs`: assertion D over `audit_signal`'s file set,
  whose walk is factored so C and D share it, plus the help line, the clean detail
  and a unit test for the line-start and fence rules (delta 4).
- `lifecycle-kit/gate-tests/check-stage-entry.test.sh`: the four scenarios
  (delta 4).
- `lifecycle-kit/checks/check-stage-entry.gate`: the `# spec:` description line
  names the inferred-claim residue (delta 4).
- `.workflow/release-declarations.md`: one bullet, since a tightened gate reaches
  a vendoring consumer, plus one for the three template obligations (deltas 1, 3
  and 4).
- `TASK-QUEUE.md`: `spec-authoring-self-check-pass` and
  `survey-inferred-claim-has-no-execution-obligation` move to Done at merge, by the
  build session that merges this file, before the drain stage is entered
  (all deltas).
- <!-- update-target-exempt: generated projections, each rostered with its freshness gate and regen command in docs/site-architecture.md §Generated projections and their freshness gates --> The `docs/` mirror of the kit's SPEC, `docs/enforcement.md`, and the generated pre-commit hook.

Roster produced by: `grep -rn "check-stage-entry"` over `lifecycle-kit/` and
`native/src/`, and `grep -n "inferred"` over `lifecycle-kit/SPEC.md` and
`lifecycle-kit/templates/`. It is a floor that build re-derives.

## Retired spellings

- None — no delta retires a spelling; "three assertions" is re-phrased prose, not
  a name any surface matches.

## Definition of Done

- [ ] **Causal completeness** — every point of the kit's causal-completeness
      check holds for the pass, both markers and assertion D.
- [ ] **Instruction surfaces: instruction only** — the spec, align and build
      replacement text carries no grounds; deltas 1 to 4 place them in
      §templates/stages/ and §check-stage-entry.
- [ ] **Merged with no information lost** — the three classes, the refused
      whole-tree probe, the block-not-annotate ground and D's honest limit survive
      in the merged prose.
- [ ] **Attested reproduction** — a sandboxed amendment carrying a not-run marker
      refuses `--enter-stage build`, and the same amendment with the marker
      rewritten to the cannot-run form enters.
- [ ] **This file is marker-clean** — assertion D is clean over the live tree at
      the first build entry, which proves delta 2's grammar does not read this
      amendment's own mentions as markers.
- [ ] **Queue moves placed before the drain stage** — both Done moves land in the
      session that merges this file, before the drain stage is entered.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls lifecycle-kit/SPEC-*.md`) once the iteration's last batch lands.
- [ ] **Gaps filed** — cross-component gaps found during the work go to the gap
      inbox.
