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
That determination is itself the audit `.workflow/audit-roster.txt`'s
`workflow-surface-extension` class calls for at any workflow-directory addition,
a class standing at `last: never`; close stamps the roster, and this paragraph is
the audit it stamps rather than a fresh one close must re-derive.

Its first line is the `# contract: ` pointer header the tracked tier requires,
pointing at **gate-sdk/SPEC.md §upgrade-smoke** with the line grammar on the
em-dash tail. That target is named rather than left to build: this amendment
merges into three homes, `check-spec-pointer` resolves whichever one is picked,
so the choice is a coin-flip unless ruled. §upgrade-smoke is where deltas 2 and 4
land the two-arm resolution, which makes it the section stating the file's
contract — the same rule every tracked member here already follows
(`release-sweep-evidence.txt` → RELEASING.md §The procedure, `validate-baseline.txt`
→ evidence-kit/SPEC.md §Baseline manifest, `gap-inbox.md` → lifecycle-kit/SPEC.md
§The committed gap inbox).

Data lines are **one bare gate name each**, and nothing else: no markup, no
prose, no ordering significance. The token predicate that decides what a bare
gate name is belongs to the helper `release-note-lead-token-grammar` landed,
gate-sdk/SPEC.md §lib/declaration.sh, not to this delta — see delta 2.

The file always exists, header-only when the declared set is empty, so "absent"
is never a state the reader has to interpret. **Draining it therefore truncates
to the header, never the whole file.** The directory's local-capture tier reclaims
by whole-file truncation (`: > <file>`), and that idiom applied here erases the
header and reds `check-workflow-tiering` on the release commit itself. Both
tracked members that drain — `WORKFLOW-STATE.txt` at the iteration boundary,
`gap-inbox.md` at close — drain header-preservingly, and so does this one.

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

**The token predicate is not this unit's to implement.**
`release-note-lead-token-grammar` landed one helper (gate-sdk/SPEC.md
§lib/declaration.sh) carrying two
*container* arms — a markdown section's bullet lead tokens, and this file's data
lines — over one token predicate, and it names this surface as its second arm.
So this delta supplies the container (a record file's data lines) and consumes
that helper's verdict; it does not state a second token predicate of its own.
Landing an inline parse here would file forward the exact defect that unit
exists to close, one surface over — three statements of one grammar where the
complaint was two.

**Build ordering, because the two units collide on two surfaces.** Both amend
gate-sdk/SPEC.md §upgrade-smoke — this unit the untagged-`TO` sentence and the
producers-and-consumers paragraph (delta 4), that unit the clause naming the note
"parsed for the bullet lead tokens docs/install.md owns" — and both amend
`bin/upgrade-smoke.sh` at the same declaration-resolve step. Land the helper
first and this delta against it; landing this one first means writing a parse
that is then replaced.

### Delta 3 — the producer: build appends, close composes and drains {design-bearing}

**Build appends.** The build stage that tightens a gate appends its name to the
declaration surface in the same unit that lands the tightening. Build is the only
stage that knows what it tightened at the moment it tightens it, so the
declaration is written from knowledge rather than reconstructed from a red — and
validate discovering the set from the gate that was supposed to check it is
exactly the shape that makes an assertion its own trigger.
`lifecycle-kit/templates/stages/build.md` gains that step.

**A gate that *lands new* is appended too, and the file's name does not narrow
that.** docs/install.md §The upgrade contract defines the note's section as "one
bullet per gate that **landed new or got stricter**" — so if close composes that
section from this surface, a surface holding only strictly-tightened gates makes
the composition lossy, and the new gate's bullet would have to come from
somewhere else, which is the recall this delta exists to remove. It costs the
smoke nothing: the assertion is red ⊆ declared, so a declared gate that never
reds is inert, and §upgrade-smoke already states that an N+1 gate cannot red
phase B because the phase-A sync never re-runs the installer. **This binds this
iteration** — `SPEC-action-gh-repo-context.md` lands `check-action-gh-repo` as a
new gate and its delta 7 owes that note bullet, so the very first use of this
surface is a landed-new gate rather than a tightened one.

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
design was shaped against rather than a side effect of it.

**The shape is attested, and the nearest precedent is the closest thing this
repo has to this file.** `.workflow/gap-inbox.md` is tracked, opens with a
`# contract: ` header, is appended by any mid-iteration session that does not
load close's triage doc, and is drained by close — every property this surface
needs, already running. `.workflow/WORKFLOW-STATE.txt` is the second: appended by
every stage session, truncated at the iteration boundary, tracked, headed. Those
are the two to copy. `.workflow/release-disposition.txt` and the lesson-evidence
file also run a write-here-read-there shape but are close-written single-writer
records, so they model the tracking and not the accumulation.

That is also the answer to the tier question this file raises on its face — a
tracked accumulating buffer is not a third tier. §The workflow directory defines
the tracked tier as *tracked, committed, gate-read*, this surface is all three,
and two members of that tier are already accumulation buffers rather than
projections. CLAUDE.md §Housekeeping's two-tier sentence needs no change.

**Honest limit, stated because it is not gated — and it merges into
RELEASING.md §The procedure step 1, not out of existence.** The composed note and
the declaration surface can disagree: close transcribes one into the other and
drains by hand, and nothing asserts the note's Tightened-gates set equals the
surface it was composed from. Review holds that agreement. It is mechanizable,
and it is filed rather than built here because closing it is a unit rather than a
line.

Naming the merge home is the substance, not the bookkeeping. This limit binds a
*close* session at the moment it composes, so it has to be readable where close
is already reading; stated only here it would be deleted with this amendment, and
the filed gap would preserve the work item while losing the warning. The gap
inbox is a work queue, not a surface anyone reads while writing a note.

### Delta 4 — the two statements of the old contract are revised {design-bearing}

gate-sdk/SPEC.md §upgrade-smoke's untagged-`TO` sentence and its
producers-and-consumers paragraph, and the script's `# spec:` line at the
declaration-resolve step. Both currently assert the empty-declaration rule as the
contract; both state the two-arm resolution instead. Leaving either is the defect
this unit was filed for, one surface over.

**The producers-and-consumers paragraph names a third reader that survives the
rewrite.** Besides the smoke, it names the upgrade skill (lifecycle-kit/SPEC.md
§templates/upgrade.md) as a consumer of the declaration, reading it as the
consumer's registration checklist. That reader is genuinely unaffected — it reads
the *note*, and the note is unchanged as an artifact — but the rewrite changes
the declaration's producer from "the release session" to build-appends-and-close-composes,
and dropping the third reader while doing so is the removals-propagated failure
this amendment's own Definition of Done bars.

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
  producers-and-consumers paragraph, third reader preserved (delta 4); the
  two-arm resolution and the exit-1 convention for a malformed declaration
  (delta 2). This is also the section the declaration surface's `# contract: `
  header points at (delta 1), and the section
  `release-note-lead-token-grammar` amended earlier in the same iteration.
- **gate-sdk/SPEC.md §The workflow directory** — the new tracked member joins the
  checked-projection tier (delta 1).
- **docs/install.md §The upgrade contract** — the Tightened-gates paragraph
  (delta 5).
- **RELEASING.md §The procedure** — step 1's Tightened-gates section composed
  from the declaration surface, carrying delta 3's honest limit on note/surface
  agreement at the point close composes; step 4's drain at the tag, stated as a
  truncation to the header rather than a clear (deltas 1, 3). Step 3's sentence
  on `.workflow/release-sweep-evidence.txt` is the model for how a tracked
  workflow member is introduced in this runbook — one step away, same shape.
  The runbook's residency is unchanged, and that is deliberate.
- **`.workflow/audit-roster.txt` — deliberately not edited here.** Delta 1 is a
  workflow-directory addition, which makes the `workflow-surface-extension` class
  due; that roster's `last:` field is close's stamp to write after its review, not
  an amendment edit. Named so build neither stamps it early nor treats delta 1's
  extension argument as unrecorded.
- **`lifecycle-kit/templates/stages/build.md`** — the append step (delta 3).
- **`gate-sdk/bin/upgrade-smoke.sh`** — the `# spec:` line at the
  declaration-resolve step (delta 4), alongside the resolution itself (delta 2).
  `release-note-lead-token-grammar` edited the same step earlier in the same
  iteration; delta 2 owns the ordering, and that unit landed first as it required.
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
