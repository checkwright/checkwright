# SPEC amendment: upgrade-smoke-note-resolution

The standing pre-release assertion — `bin/upgrade-smoke.sh` at its `TO=HEAD`
default — is satisfiable today only by an iteration that tightens nothing. This
amendment gives an untagged `TO` a declaration to be contained by, so the
assertion proves containment rather than emptiness.

The premise this rests on, and the reason the unit is feature-shaped: the present
behavior is **specified, not accidental**. The script's `# spec:` line at the
declaration-resolve step and gate-sdk/SPEC.md §upgrade-smoke both state that an
unreleased `TO` resolves no version, so no note, so the red set must be empty. A
fix cannot converge the script onto a contract that already says otherwise; it
revises the contract, which is what earns the amendment.

**A script-only fix is a no-op, not a relocation.** Both resolution rules the
queue entry floated fail in the normal flow, and for the same reason: at the
moment validate runs, no artifact on disk names the gate this iteration
tightened. Reading the highest note above the baseline finds the *previous*
release's Tightened-gates set, which cannot contain a gate tightened since;
keying off the derived bump looks for a note that does not exist yet. The refusal
fires either way. The entry's framing was misled by `pre-adoption-grammar-break`,
where the note *was* on disk at validate — but only under a one-off ruling to
author it early, not because any contract required it. That one-off is itself the
evidence that the missing piece is a **producer**, and that assigning it belongs
in the contract rather than in a session's judgment.

## What changes

### Delta 1 — a declaration surface, written during the iteration {design-bearing}

The gap is that the allowed-red set has no home between the moment a gate is
tightened and the moment a release note is composed. Add one:
`<workflow-dir>/tightened-gates.txt`, a **tracked checked projection** of the
workflow directory (gate-sdk/SPEC.md §The workflow directory).

The extension is determined rather than chosen — a record file with a stated line
grammar that a tool parses field-wise is `.txt` by that section's extension rule.
Its first line is the `# contract: ` pointer header the tracked tier requires,
naming this amendment's merged home and the line grammar. Data lines are **one
bare gate name each**, and nothing else: no markup, no prose, no ordering
significance.

The file always exists, header-only when the declared set is empty, so "absent"
is never a state the reader has to interpret.

**No new knob.** The path derives from the existing `GATE_SDK_WORKFLOW_DIR`, the
way `GATE_SDK_GRAPH_ARTIFACT` derives its default. A knob naming this file would
add a way to configure the assertion away without adding a way to satisfy it
honestly.

### Delta 2 — the smoke resolves an untagged TO from the declaration surface {design-bearing}

The `TO`-tagged arm is unchanged: the version is the `v*` tag pointing at `TO`,
and the note is the `TO`-tree post whose `release:` key names it.

The untagged arm stops asking a tag what version `TO` is and reads the
declaration surface out of the `TO` tree instead. The declared set is the file's
data lines; an empty set means the red set must be empty, which is today's rule
kept as the narrow case rather than the universal one.

A data line that is not a bare gate-name token is a **fail, exit 1** — not exit
2. gate-sdk/SPEC.md §upgrade-smoke already states the convention this follows: a
missing or unparseable declaration while reds exist is a fail, and
usage/environment failure is exit 2. A malformed declaration is a contract
violation, not a broken environment.

### Delta 3 — the producer: build appends, close composes and drains {design-bearing}

**Build appends.** The build stage that tightens a gate appends its name to the
declaration surface in the same unit that lands the tightening. Build is the only
stage that knows what it tightened at the moment it tightens it, so the
declaration is written from knowledge rather than reconstructed from a red — and
validate discovering the set from the gate that was supposed to check it is
exactly the shape that makes an assertion its own trigger.
`lifecycle-kit/templates/stages/build.md` gains that step.

**Close composes and drains.** RELEASING.md §The procedure step 1 already authors
the note at close; its Tightened-gates section is now *composed from* the
declaration surface rather than recalled. The surface is drained when the tag is
pushed (step 4) and only then — an iteration closing on `release none` or a
deferral carries its declarations forward, which is exactly what the next
release's note must inherit.

**Why the producer appends rather than authors.** Tightened gates is a
**release-level aggregate**, not an iteration-level one. Several internal
iterations batching into one external release is a shape this repo already wants,
and under it a build stage that authored note prose directly would be writing
into an artifact that does not exist yet and whose version it cannot know. A
build stage that appends a name to an accumulating surface, composed once at the
release boundary, is correct under batching and degenerates gracefully to the
one-iteration-one-release case. The two shapes are indistinguishable today and
diverge exactly when batching arrives, which is why the aggregate shape is chosen
now rather than migrated to later.

**This deliberately does not widen the release runbook's load-trigger
residency.** Build writes a `.workflow/` record; it does not load RELEASING.md,
which stays resident only at close's release step. CLAUDE.md §Housekeeping's
statement to that effect stays true, and keeping it true is a constraint this
design was shaped against rather than a side effect of it. The repo already runs
this producer/consumer shape twice — `.workflow/release-disposition.txt` and the
lesson-evidence file — so the pattern is attested rather than invented here.

**Honest limit, stated because it is not gated.** The composed note and the
declaration surface can disagree: close transcribes one into the other and drains
by hand, and nothing asserts the note's Tightened-gates set equals the surface it
was composed from. Review holds that agreement. It is mechanizable, and it is
filed rather than built here because closing it is a unit rather than a line.

### Delta 4 — the two statements of the old contract are revised {design-bearing}

gate-sdk/SPEC.md §upgrade-smoke's untagged-`TO` sentence and its
producers-and-consumers paragraph, and the script's `# spec:` line at the
declaration-resolve step. Both currently assert the empty-declaration rule as the
contract; both state the two-arm resolution instead. Leaving either is the defect
this unit was filed for, one surface over.

### Delta 5 — docs/install.md states that a declaration precedes its release {design-bearing}

§The upgrade contract describes the note as the thing a release ships. Under
delta 2 a consumer running the smoke against their checkwright clone at an
untagged `TO` reads the declaration surface, so an allowed-red set is owed from
the moment a gate tightens rather than from the tag. One sentence, in the
Tightened-gates paragraph that already names the section as the mechanical
allowed-red set.

### Delta 6 — the `upgrade` validate baseline row is promoted {mechanical}

`.workflow/validate-baseline.txt` holds `upgrade` at `fail` against this slug.
The hold ends when this lands, and the row goes to `pass` with the slug dropped.

Two traps the queue entry recorded, both binding here. **Do not promote the row
on a post-tag green**: immediately after a tag, `TO` is momentarily a tagged
`HEAD` and the smoke runs clean for that reason alone; the next commit restores
the defect. **Do not read a green as proof the mechanism works**: this iteration
may tighten no existing gate, in which case the declared set and the red set are
both empty and the untagged arm is never exercised against a non-empty set. That
exercise belongs to the argument above and to the smoke's own assertion messages,
not to a green run.

## Producers and consumers

- **The declaration surface** — produced by the build stage of an iteration that
  tightens a gate (delta 3), appending one gate name per line;
  `lifecycle-kit/templates/stages/build.md` is the step that makes the producer
  reachable rather than aspirational. Consumed at two named transitions: by
  `bin/upgrade-smoke.sh` at its declaration-resolve step on the untagged arm, and
  by close's release step (RELEASING.md step 1) when it composes the note's
  Tightened-gates section. Drained by close at the tag (step 4).
- **Each data line** — a gate name, read by both consumers above. There is no
  second field, deliberately: a rationale column would be a field the smoke never
  reads and the note's bullet prose already owns.
- **The header line** — produced by whoever creates the file; consumed by
  `check-workflow-tiering` (the tracked tier's header requirement) and by
  `check-spec-pointer` (its pointer payload must resolve).
- **The composed note** — produced by close, consumed by the smoke's tagged arm
  and by the human upgrader reading the site. Unchanged as an artifact; only its
  Tightened-gates section's *source* changes.
- **No new knob.** The knob roster in §upgrade-smoke (`GATE_SDK_UPGRADE_REPO` /
  `_FROM` / `_TO`, and `GATE_SDK_TMP_DIR` for scratch) is unchanged, and the
  declaration path derives from `GATE_SDK_WORKFLOW_DIR` (delta 1).

**Seam.** The mechanism is generic — a tracked record file of names, read by a
tool that already reads the consumer's tree. The gate names in it are consumer
content, supplied by the consumer's own build sessions, and the path derives from
a knob the consumer already sets. Nothing private crosses, and no kit literal
carries a consumer's roster.

## Existing sections updated

- **gate-sdk/SPEC.md §upgrade-smoke** — the untagged-`TO` sentence and the
  producers-and-consumers paragraph (delta 4); the two-arm resolution and the
  exit-1 convention for a malformed declaration (delta 2).
- **gate-sdk/SPEC.md §The workflow directory** — the new tracked member joins the
  checked-projection tier (delta 1).
- **docs/install.md §The upgrade contract** — the Tightened-gates paragraph
  (delta 5).
- **RELEASING.md §The procedure** — step 1's Tightened-gates section composed
  from the declaration surface, and step 4's drain at the tag (delta 3). The
  runbook's residency is unchanged, and that is deliberate.
- **`lifecycle-kit/templates/stages/build.md`** — the append step (delta 3).
- **`gate-sdk/bin/upgrade-smoke.sh`** — the `# spec:` line at the
  declaration-resolve step (delta 4), alongside the resolution itself (delta 2).
- **`.workflow/validate-baseline.txt`** — the `upgrade` row (delta 6).
- **CLAUDE.md §Housekeeping — deliberately unchanged.** Its statement that the
  release runbook is resident only at close's release step stays true under delta
  3. Named here so a build session does not reach for it on the assumption that
  moving the producer must move the runbook; if a build session finds a design in
  which it must, that is an escalation rather than an edit.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
