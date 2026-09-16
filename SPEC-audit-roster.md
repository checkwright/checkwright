# SPEC amendment: audit-roster

Pairs four queue entries: `audit-roster-row-carry-unruled` (lead),
`audit-roster-mechanism-has-no-kit-owner` (carrier), `audit-class-corpus-attestation`
and `audit-roster-last-stamp-author-unconstrained`. It spans lifecycle-kit,
doctrine-kit, the native crate and this repo's config, so it sits at the repo root.

**Every replacement text below is Not yet applied.** Build lands it.

## What changes

### (1) lifecycle-kit owns the audit roster: a new `## The audit roster` section {design-bearing}

Today no kit ships the roster. Doctrine rule 2 says a cadence is owed. This repo's
close binding is the only procedure that runs the review. Nothing grades the row
grammar. A new lifecycle-kit SPEC section, placed after `## The close-surface roster`,
owns the format, the carry rule, the stamp author, the corpus attestation and the
review step's contract. Which classes a tree runs stays consumer content: the rows
live in the consumer's file, and the kit ships no class.

Replacement text (Not yet applied), as the section body:

> **The audit roster** is the capture mechanism of doctrine-kit's Enforcement-first
> carve-out. A class that no check can decide cleanly stays a stated manual duty,
> and a duty with no named cadence is one no session performs. So the class joins
> a tracked roster that the close stage reviews, with event-keyed due-ness. The
> roster is hand-curated, not derived. Which classes escape a clean check is a
> judgment no tool enumerates, so Derivation-first's ladder lands on
> state-once-at-the-owner. The roster and its review step replace a gate; they
> are not one.
>
> **The surface.** `LIFECYCLE_KIT_AUDIT_ROSTER_FILE` names it, and the knob
> defaults empty. A tracked record file carries a `# contract:` pointer header,
> then one block per class, separated by blank lines:
>
> ```
> class: <class-slug>
> scope: <what to sweep, and each standing reading that changes how the next sweep runs>
> due: <the named event(s) that make the class due>
> last: <iteration> <stage>
> corpus: <the command the sweep ran> | surfaces: <path>[, <path>...]
> hits: <candidates triaged>
> declined: <what the sweep left unread or ruled outside its trigger, and why> | none
> ```
>
> A never-swept class carries `last: never` and stops there. Every other block
> carries all seven keys, in this order, one per line, with no stray line. Each
> field has a named reader:
> - `class` — the review, as the block's key.
> - `due` and `last` — the review, to judge due-ness.
> - `scope` — the sweeping session, to derive its corpus.
> - `corpus` — the next sweep, which re-runs it as a floor.
> - `hits` — the next sweep. A count that falls with no tree change to explain it
>   is a signal to read.
> - `declined` — the next sweep, which takes up what its predecessor set aside.
>
> **Nothing appends.** A sweep *replaces* the `last`, `corpus`, `hits` and
> `declined` lines. A reading that changes how the next sweep runs is folded into
> `scope` by re-phrasing it, never by appending. The sweep's narration, its
> findings and its cleared candidates go into the commit message that re-stamps
> the block, so `git log -p` over the roster recovers every earlier reading, and
> the file carries only the standing ones. Every line is bounded by
> `LIFECYCLE_KIT_AUDIT_ROSTER_LINE_CAP`, so a whole-file read stays affordable at
> the review.
>
> *Ruled out: one file per class.* It moves growth from one file to many and
> removes none of it, and a review still reads each accreted body whole. *Ruled
> out: capping an appending row and forcing the appending close to compress.* How
> much a sweep appends depends on the iteration's shape, such as a deletion or a
> ruling landing, so a per-close cap would truncate hardest in the iteration the
> roster exists for. The cap here bounds a standing line that is rewritten, never
> an append. *Compaction without a grammar is unsafe:* a pass that reads an accreted
> row as prose can drop a mandatory field. That is why the fields are separate lines
> and `check-audit-roster` grades them.
>
> **`corpus` attests what was read, not what the next sweep must read.** A stored
> probe goes stale whenever a class's instances are event-derived, such as the path
> components a deletion touched. So `scope` states how to derive the corpus, each
> sweep derives its own, and the predecessor's `corpus` is a floor that the sweep
> widens. A class with no single-command corpus stamps
> `surfaces:` and the list of surfaces it actually read. A capability claim's
> instances are an example: they are a tree set that no scanner infers. A sweep
> that read nothing has nothing to stamp there, so it cannot record a verdict.
> `declined` records any live-looking hit the sweep ruled outside its own trigger.
> A declined hit that is a live defect is still dispositioned, under the
> Gap-disposition rule.
>
> **`last` names its stage, and the stage is checked against the stamps.**
> `<stage>` is a member of `LIFECYCLE_KIT_STAGES`. The review belongs to the last
> configured stage, but any stage may stamp. A stage that just moved a class's
> population would otherwise leave the row stale. The review reads a `last` naming
> any other stage as a **pre-stamp**: the audit is still owed and has not run.
> Where `<iteration>` is the queue header's current iteration, the state file must
> carry an `<iteration> <stage>` stamp, so a row cannot claim a stage that never
> ran. *The honest limit:* the check proves the named stage was entered. It does
> not prove that session wrote the line, and it does not prove the audit was
> faithful.
>
> **A consumer that sets the knob owes the roster a `close-surface:` declaration,
> `advisory`** (§The close-surface roster). The roster is tracked, so without one it
> reaches no derived roster, which is the same position as the pre-flight valve's
> ledger. No forcing function exists, and the mode makes a skipped review visible.
> An empty knob leaves the review step skipped and `check-audit-roster` inert.

Every surface in this repo that argued the roster's grounds points here after the
merge (delta 6).

### (2) Two lifecycle-kit knobs {mechanical}

Add two rows to lifecycle-kit's knob roster and its static table.

- `LIFECYCLE_KIT_AUDIT_ROSTER_FILE`, **default empty**. The ground is the one
  `LIFECYCLE_KIT_RULING_RECORD` uses: a kit default filename would assert that every
  adopter keeps this artifact under this name. Empty makes the review step and the
  gate inert.
- `LIFECYCLE_KIT_AUDIT_ROSTER_LINE_CAP`, a positive integer in bytes, **proposed
  default `1500`**. It is a stated policy rather than a derived number, like
  `QUEUE_KIT_ENTRY_LINE_CAP`. The table validator refuses a non-positive value at exit 2.

This repo's `scripts/lifecycle-config.knobs` sets
`LIFECYCLE_KIT_AUDIT_ROSTER_FILE = .workflow/audit-roster.txt`, under a `# spec:`
line naming §The audit roster.

### (3) A close-template review step replaces the binding's sub-step {mechanical}

`lifecycle-kit/templates/stages/close.md` gains a step 8 after the ruling-record repair.
Steps 8 through 12 become 9 through 13. The close binding deletes its **Audit-roster review**
sub-step. It gains the advisory declaration in its place, and its
"brevity pass (step 10)" becomes "(step 11)".

Replacement text for the template (Not yet applied):

> 8. **Review the audit roster**, where `LIFECYCLE_KIT_AUDIT_ROSTER_FILE` names one
>    (lifecycle-kit/SPEC.md §The audit roster). For each class block, judge which
>    `due:` events fired since `last:`. A `last:` naming a stage other than this one
>    is a pre-stamp, so read it as unreviewed. Perform each due audit, or defer it as
>    a costed filing. For each audit performed:
>    - Derive the corpus from `scope:` and this iteration's range, taking the
>      predecessor's `corpus:` as a floor.
>    - Run it, and triage every hit.
>    - **Replace** `last:`, `corpus:`, `hits:` and `declined:`.
>    - Fold a reading that changes the next sweep into `scope:` by re-phrasing.
>
>    The sweep's narration and findings go in the commit message, never on the roster.

Replacement text for the binding, in place of the sub-step (Not yet applied):

> - **The audit roster is a roster row this binding declares.** It is tracked, and
>   its knob is this repo's config.
>
>   close-surface: .workflow/audit-roster.txt advisory

### (4) `check-audit-roster`, born native {design-bearing}

A new lifecycle-kit gate. It ships as a Rust module under `native/src/gates/`, a
`lifecycle-kit/checks/check-audit-roster.gate` descriptor
(install disposition on-surface, and a precommit-tier graph manifest coupling the
`knob:LIFECYCLE_KIT_AUDIT_ROSTER_FILE` token, the state file and the queue file,
where build calibrates the spelling against `check-scratch-citation`'s `knob:`
precedent), a `good/`+`bad/` pair, and a
`.test.sh` for the state-file arm. Its SPEC section is `### check-audit-roster`.

Invariant, over each block of the configured roster:
- **A.** The keys are present, in order, one per line, with no stray line:
  `last: never` closes a block after four keys, and every other `last:` closes it
  after seven. Every value is non-empty, and `declined` takes the literal `none`.
  `hits` is a decimal integer. `last` is `never` or `<iteration> <stage>`, with
  `<stage>` a member of `LIFECYCLE_KIT_STAGES`.
- **B.** No line exceeds `LIFECYCLE_KIT_AUDIT_ROSTER_LINE_CAP` bytes.
- **C.** `class` slugs are unique.
- **D.** A `last` whose `<iteration>` equals the queue header's iteration names a
  stage that the state file stamped for that iteration.

An empty knob, an absent file, or a file holding its header alone is clean and
counted inert. Bare mode drives the configured roster with all four assertions. An
explicit file argument drives it hermetically with A, B and C, because a fixture's
iteration names nothing in the host's state file. The clean line names which mode
ran, on `check-survey-record`'s precedent.

The four contracts:
- **Output:** `AUDIT-ROSTER: clean`, and a `help:` naming the grammar section.
- **Fail-closed:** exit 2 on an unreadable file or a failed parse.
- **Fixture-pair:** the bad case carries a dropped `due:` line (the attested
  compaction loss), an over-cap line, a duplicate class, a `last:` naming an
  unconfigured stage, and an empty `declined:`. The good case carries a `never`
  block, a `surfaces:` block and a command block.
- **Self-lint:** registration in `scripts/gates.list`.

It also joins the gate roster in `lifecycle-kit/README.md` and the gate list in
`lifecycle-kit/smoke/install.sh`, and takes a release declaration.

*Not gated:* whether a sweep read its corpus faithfully, and whether a `hits` count
is true. Both are session acts with no tracked residue.

### (5) Migrate this repo's roster to the grammar {design-bearing}

Rewrite `.workflow/audit-roster.txt`'s nine class rows into blocks. For each row:
- `scope` keeps the standing rulings and the readings that change how a sweep runs.
  Examples are the survey-record exclusion, the deferred-pool inclusion, the
  deletion-blast-radius oracle triaged on tense, and a derived probe set in place
  of a stored one.
- `due` is recovered whole. The prior accidental compaction lost it once.
- `last`, `corpus`, `hits` and `declined` are recomposed from the row's most recent
  sweep reading. A sweep that stamped no corpus takes `surfaces:` naming what its
  narration says it read. Where the narration names nothing, it takes the
  `last: never`-style truth that the corpus is unknown: `declined:` carries
  `corpus unrecorded before the grammar`.
- The narration goes to the migrating commit message, which names the parent
  commit whose blob holds the full prior text.

The header becomes the pointer form alone:
`# contract: lifecycle-kit/SPEC.md §The audit roster`. The migration runs under
`check-audit-roster` in bare mode, so a dropped field reds before it lands.

### (6) Doctrine and the other roster citations point at the kit section {mechanical}

**doctrine-kit/DOCTRINE.md rule 2.** The rule keeps the cadence obligation. The
grounds for hand-curation and the capture-mechanism argument leave, because they now
live in §The audit roster. Replacement for the passage from "A stated manual duty
carries a *named cadence*" to the end of the rule body (Not yet applied):

> A stated manual duty carries a *named cadence*, or it is a duty no session
> performs: the un-gateable class joins a tracked audit roster reviewed on a
> lifecycle hook, with event-keyed due-ness — a named observable event beats an
> iteration counter no surface tracks
> ([lifecycle-kit/SPEC.md](../lifecycle-kit/SPEC.md) §The audit roster owns the
> mechanism).

**Rule 3.** In its *Enforced by:* line, "the close-stage audit roster
(`.workflow/audit-roster.txt`)" becomes "the close-stage audit roster
(lifecycle-kit/SPEC.md §The audit roster)". A kit doctrine must not spell one
consumer's path.

**lifecycle-kit/SPEC.md §check-close-surfaces.** Its closing citation
"(doctrine-kit/DOCTRINE.md §Methodology-maintenance rules)" becomes
"(§The audit roster)".

## Producers and consumers

- **The roster block (delta 1).**
  - Producer: the close review step (delta 3). A non-close stage may also write a
    pre-stamp. It is reachable wherever the consumer sets
    `LIFECYCLE_KIT_AUDIT_ROSTER_FILE`, and this repo sets it (delta 2).
  - Consumers: the close review, at the next close, reads `class`, `due` and
    `last`. The sweeping session reads `scope`, `corpus`, `hits` and `declined`.
    `check-audit-roster` reads every field at commit time. The ruling-staleness
    citing report reads the file through `LIFECYCLE_KIT_RULING_CITERS`, and it
    quotes lines, which shorter lines serve better. `--emit close-surfaces` reads
    the binding's declaration.
- **Field readers** are named per field in delta 1's text. No field goes unread.
- **The stage token (delta 1).**
  - Producer: the stamping session.
  - Consumers: the close review, which reads a non-close stage as a pre-stamp, and
    assertion D, which checks the stamp exists. D reds when the named stage has no
    current-iteration stamp. It is silent on a past iteration's row, because the
    boundary truncates the state file.
- **The knobs (delta 2).**
  - `LIFECYCLE_KIT_AUDIT_ROSTER_FILE` is read by the close step and the gate. Empty
    makes both inert.
  - `LIFECYCLE_KIT_AUDIT_ROSTER_LINE_CAP` is read by the gate alone.
  - Roster-holding readers of the minted names: the static table and its validator,
    `check-knob-citation` and `check-kit-ref-liveness` over the SPEC, and
    `check-lifecycle-registration` if the marker block cites knobs. Build probes each
    with `git grep -n LIFECYCLE_KIT_SURVEY_RECORD_FILE` as the model row.
- **The gate (delta 4).**
  - Producer: the battery.
  - Consumer: the pre-commit hook, regenerated from the `# graph:` manifest.
  - Roster readers of a new gate name: `scripts/gates.list`, `native/src/gates/mod.rs`,
    `lifecycle-kit/README.md`, `lifecycle-kit/smoke/install.sh`,
    `scripts/git-hooks/pre-commit` (generated), and the release declarations surface.
    The probe is `git grep -n check-survey-record`, run 2026-09-16 over the tracked
    tree as the model gate.
- **Red conditions (point 5).** The migration (delta 5) narrows a corpus: it shrinks
  the roster's bytes. Its readers are the ruling-staleness citing report and
  `check-scratch-citation`, which read the roster as text.
  - The citing report reds nothing. It reports sites, so a dropped mention of a
    ruling name removes a quoted site and never flips a verdict.
  - `check-scratch-citation` reads the forbidden truncated set, and the roster is not
    in it.
  - No reader asserts a count or a floor over the roster. Probe:
    `git grep -n "audit-roster" -- '*.rs' '*.gate' scripts/` returned no gate reader
    on 2026-09-16.
- **Point 6.** The members are the nine class rows, enumerated by
  `cut -d' ' -f1 .workflow/audit-roster.txt`:
  - internal-identifier-restatement
  - workflow-surface-extension
  - capability-liveness-after-descope
  - stale-identifier-after-retirement
  - capability-pendency-after-landing
  - close-surface-actually-read
  - survey-engagement
  - ruling-record-retirement
  - unresolvable-walk-root

  Each one's satisfying value is its recomposed block. Where a row's narration
  records no corpus, the value is the `declined:` sentence in delta 5, so no member
  lacks one.

## Existing sections updated

Roster probe: `git grep -n -e "audit-roster" -e "audit roster"`, run 2026-09-16 over
the tracked tree and excluding `docs/` mirrors. It is a floor that build re-derives.

- `lifecycle-kit/SPEC.md` §The audit roster (new), §Layout and configuration (the
  two knob rows), and a new §check-audit-roster (deltas 1, 2 and 4).
- `lifecycle-kit/SPEC.md` §check-close-surfaces, the closing citation (delta 6).
- `lifecycle-kit/templates/stages/close.md`, the new step 8 and renumbering (delta 3).
- `.claude/commands/close.md`, the housekeeping binding: the sub-step is replaced by
  the declaration and the step reference moves (delta 3).
- `native/src/knobs/lifecycle_kit.rs`, the two table rows (delta 2).
- `scripts/lifecycle-config.knobs`, the roster path (delta 2).
- `native/src/gates/mod.rs`, `lifecycle-kit/checks/check-audit-roster.gate`,
  `lifecycle-kit/gate-tests/check-audit-roster/`, `scripts/gates.list`,
  `lifecycle-kit/README.md`, `lifecycle-kit/smoke/install.sh`,
  `scripts/git-hooks/pre-commit` (regenerated) and `.workflow/release-declarations.md`
  (delta 4).
- `.workflow/audit-roster.txt`, the header and nine blocks (deltas 1 and 5).
- `doctrine-kit/DOCTRINE.md`, rules 2 and 3 (delta 6).
- `docs/` generated mirrors of the lifecycle-kit SPEC and the doctrine (all deltas).
<!-- update-target-exempt: a subject mention, not a statement of the roster's mechanism; it stays true after every delta -->
- `gate-sdk/SPEC.md` §The workflow directory, "a cadenced review entry on the
  consumer's audit roster". It is left as is.
<!-- update-target-exempt: a subject mention of the residue's destination; it stays true after every delta -->
- `canon-kit/SPEC.md` §check-docs-cmd, "the close-stage audit roster's". It is left
  as is.

## Retired spellings

- `Audit-roster review` — the binding's sub-step heading, which the template step
  replaces (delta 3).
- `audit-roster v1` — the roster's version-marker header line, which the pointer
  header replaces (delta 5).

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The
      causal-completeness check holds for each new state, event, interface and
      obligation.
- [ ] **Instruction surfaces: instruction only** — the close-template step and the
      binding carry no grounds. Delta 1's section places them (the Content-tiering /
      SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the text it
      refines, and doctrine rule 2's removed grounds land in §The audit roster.
- [ ] **Amendment deleted** — this file is removed on merge, and none remain
      (`ls SPEC-*.md`).
- [ ] **Removals propagated** — both retired spellings are declared above, and
      `check-audit-roster` runs green in bare mode over the migrated roster.
- [ ] **Gaps filed** — cross-component gaps discovered during the work are filed as
      debt tasks.
