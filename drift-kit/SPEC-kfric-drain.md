# SPEC amendment: kfric-drain

**The knowledge-friction drain remediates a captured claim without ever checking it, and its
sibling channel forbids exactly that.** `drift-kit/templates/close-knowledge.md` tells close to
turn each log line into a doc-owner edit; `lifecycle-kit/SPEC.md` §The committed gap inbox tells
close that *"the drain re-verifies; capture does not"*, on an argument that transfers word for
word: capture is deliberately cheap, so nothing upstream established the claim. Two channels, one
property, one guard — and only one of them has it.

**This amendment is one axis of a two-axis entry, and the other axis stays shut.** The
capture-side question — whether the affordance should carry a measured-versus-estimated
distinction — needs a design ruling nobody has made and is **expressly out of scope**, taken in
part 2026-09-07 `lead, own-authority` relayed in this iteration's dispatch. No delta here adds a
field, a flag or a convention to the capture arm. The entry is **not spent** by this amendment and
its terminal move is a demotion, not a Done move (see §Definition of Done).

**What this amendment asserts, and what it deliberately does not.**

- **Asserted** — the four deltas below, all prose on two drift-kit surfaces, checkable by reading
  them.
- **Not asserted** — that the obligation is enforced. Neither channel's re-verification is gated:
  `check-gap-inbox-neutrality` asserts a bullet's *shape* and no gate on either side asserts that a
  re-verification ran. Parity with the sibling therefore does **not** include a gate, and saying so
  is part of the deliverable — otherwise a later reader takes the missing gate for an omission
  rather than for the sibling's own posture.
- **Not asserted** — any efficacy figure. The three attested instances are graded honestly in
  §Producers and consumers rather than counted.

## What changes

### (1) The kfric drain re-verifies before it remediates

`drift-kit/SPEC.md` §The knowledge-friction loop's triage step gains the obligation: per log
entry, name the claim the remediation turns on, name the command that establishes it, run that
command, and remediate against what it returned {design-bearing}.

The section's step 2 today reads *walk the log; for each entry, the remediation is a doc-owner
edit*, and nothing between the capture and the canonical surface asks whether the captured
sentence is true. The obligation lands as the sibling's, **cited and not restated**: the ground
lives at `lifecycle-kit/SPEC.md` §The committed gap inbox, whose *"nothing upstream established
it"* argument is about a frictionless capture channel and is not specific to gaps. Restating the
argument here would be the parallel-copy defect on the one surface this kit's own tiering rules
are strictest about; what this section owns is the obligation's **shape for this channel**, which
delta 2 is.

**Why the drain and not the capture.** The same asymmetry the sibling records: the whole value of
`--emit kfric` is that stamping costs less than deferring, so a check at capture time is paid on
every capture and buys accuracy with the capture rate the loop depends on. The drain is paid once
per entry, by a session that is already reading the entry to decide where the fact goes.

### (2) A capture carries two claims, and the ownership clause is the one that routes the fix

The obligation covers both limbs of a kfric line — the **fact** and the **surface** — and the
disposition is keyed on which limb fell {design-bearing}.

This is the delta that is not merely the sibling's rule copied across, and it comes from the
channel's own grammar. A kfric line is `<date> <fact re-derived> ← <surface it was read from>`,
and that second field is an implicit **ownership claim**: *this fact has no owner, or its owner is
not the surface I looked at*. A gap-inbox bullet carries no such clause, so the sibling's rule has
no reason to name one — but here it is the field the remediation reads. The attested case is
2026-09-06: the fact was **true** and the ownership clause — *"no gate-sdk/SPEC.md section states
it"* — was **false**, the section having stated it since `121e76cb`, two days earlier, landed by
this same triage loop. A drain checking only the fact passes that entry and mints a second home
for a fact that already had one, which is the shape the star topology exists to refuse.

The dispositions, keyed on the re-verification's outcome:

- **Both limbs hold** — the existing remediation, unchanged: give the fact a home under the tier
  contract.
- **The fact is false** — no doc-owner edit. The entry is dropped with the correction recorded, and
  if the false fact already reached a governed surface, that is a work-shaped finding and routes to
  the gap inbox rather than being fixed silently inside a triage step.
- **The fact holds, the ownership clause is false** — the fact has an owner and the session read
  the wrong surface, so the remediation is the section's **second** existing shape: a pointer from
  where the session looked to where the owner is. Never a second home. This branch already exists
  in the step's text; what this delta adds is that a re-verification is what *selects* it, rather
  than the filer's own clause selecting it unchecked.
- **No cheap command settles either limb** — the entry is dispositioned *as a claim*: say so, and
  let whatever it produces carry the unverified premise openly. Taken verbatim in spirit from the
  sibling, because the alternative — a drain blocked on an unsettleable claim — converts a cheap
  loop into a stalling one.

### (3) The template carries the mechanized sub-step

`drift-kit/templates/close-knowledge.md` gains a re-verify sub-step inside step 2, mirroring
`lifecycle-kit/templates/stages/close.md`'s {mechanical}.

The split is the sibling's exactly and this amendment adopts it rather than inventing one: the
**SPEC states the rule and its grounds**, the **template states the procedure** a spliced close
skill executes, and the template cites the SPEC for why the check sits at the drain and not at
filing time. The sub-step lands ahead of the two remediation shapes, since it is what chooses
between them.

### (4) The outcome is recorded where the disposition is recorded

The drain records, in the close commit that clears the log, which entries were re-verified and
what fell {design-bearing}.

Without this the obligation is unobservable: a triage that silently re-verified and a triage that
silently did not leave identical trees, because the log is cleared either way and the doc-owner
edit looks the same. The record rides the commit message beside the dispositions close already
states there — the sibling's own mechanism, and the reason it is a commit message rather than a
new file is that the log is gitignored scratch cleared in the same motion, so a durable record
placed *in* the channel would be erased by the channel's own reclaim.

**The honest limit, stated in the section rather than left to be discovered.** This is an
authoring obligation attested by a commit message, not a gate. Nothing mechanizes *a command was
run*, and the alternative — a session attesting to its own diligence in a parsed field — is worth
less than the rule. That is the same limit `lifecycle-kit/SPEC.md` states for its own channel, and
it is the reason parity here means prose parity.

## Producers and consumers

**No new state, event or interface is created by any delta.** Every delta constrains an existing
procedure, so the causal-completeness points are answered against surfaces that already exist —
and that is stated plainly rather than left as a silence, because an amendment whose §What changes
is four deltas long and whose producers section is empty reads like an omission.

- **Producer of the claim being checked.** `--emit kfric`
  (`native/src/emit/kfric.rs`, bridged at `native/src/emit/mod.rs`), writing
  `<date> <fact> ← <surface>` to `.workflow/knowledge-friction.log`
  (`DRIFT_KIT_KNOWLEDGE_LOG`). Unchanged by every delta. Its enabling configuration is set in this
  tree — CLAUDE.md §Housekeeping instructs every session to stamp — so the producer is live and
  not test-only.
- **Consumer of the obligation.** The close stage, through the spliced
  `close-knowledge.md`. In this repo the splice is a **binding**, not kit mechanism:
  `.claude/commands/close.md`'s housekeeping block routes the knowledge-friction row of
  `--emit close-surfaces` at drift-kit's template. That indirection is why delta 3 puts the
  procedure in the template — a consumer who routes the row elsewhere gets the SPEC's rule and
  writes their own procedure, which is the seam working rather than a hole.
- **Consumer of the record (delta 4).** The next session reading close's commit for what the
  iteration's triage did, and the operator reading the health triad's commentary. Its **red
  condition is none** — no gate reads it, deliberately, per delta 4's own limit.
- **Every field has a named reader.** No field is added to the kfric line, to the log's grammar,
  or to any emitted record. The two fields the obligation newly *reads* — the fact and the
  surface — are the two the grammar has always carried; what changes is that the drain reads the
  second as a claim rather than as a locator.

**Existing integration prose describing the prior flow, updated rather than left to drift.**
`drift-kit/SPEC.md` §The knowledge-friction loop's step 2 and
`drift-kit/templates/close-knowledge.md`'s step 2 both describe the drain as remediation-only;
deltas 1 to 4 rewrite each in place. The update-target roster is narrow and that is a verified
finding rather than an assumption: a sweep for `kfric`, `knowledge-friction` and `close-knowledge`
across tracked files returns those two surfaces as the only ones stating the drain's procedure.
`CLAUDE.md` §Housekeeping already only points at drift-kit/SPEC.md and needs no edit — which is
the star topology paying out, and is why it appears here as a checked non-target rather than not
at all.

**Cross-kit direction, checked rather than assumed.** There is no code dependency either way —
`drift-kit/lib/drift.sh` sources only its own config and no lifecycle-kit lib. The coupling is
prose citation, and it already runs in both directions: each SPEC cites the other's section to
draw the boundary between the two channels. So drift-kit citing
`lifecycle-kit/SPEC.md` §The committed gap inbox for the transferring argument adds no new
dependency and inverts none.

**The provenance seam.** Nothing here is rule content: the obligation is generic mechanism over a
channel drift-kit already ships, no vocabulary is minted, no knob is added, and no consumer term
list is touched. The one consumer-specific fact in play — that this repo splices the template
through `.claude/commands/close.md` — stays in this repo's binding and out of both kits, which is
where it already is.

## Existing sections updated

- `drift-kit/SPEC.md` §The knowledge-friction loop — the triage step, which gains the obligation
  (delta 1), the two-limb shape with its disposition set (delta 2), the record-in-the-commit
  requirement and its honest limit (delta 4), and a pointer to
  `lifecycle-kit/SPEC.md` §The committed gap inbox for the ground it does not restate (delta 1).
- `drift-kit/templates/close-knowledge.md` — step 2, which gains the re-verify sub-step ahead of
  the two remediation shapes and the citation back to the SPEC for why the check sits at the drain
  (delta 3).
<!-- update-target-exempt: the sibling section is cited, not changed, and a back-reference from it would invert the tier the citation exists to respect -->
- `lifecycle-kit/SPEC.md` §The committed gap inbox — **deliberately untouched**. Its rule and its
  grounds are correct as they stand and this amendment cites them; editing it to announce a second
  consumer would put a fact about drift-kit inside lifecycle-kit's tier.
<!-- update-target-exempt: the always-loaded bullet states the capture affordance and points at the owner, so a drain-side change reaches it through the pointer and not through an edit -->
- `CLAUDE.md` §Housekeeping — **deliberately untouched**; its knowledge-friction bullet is
  capture-side and already points at the owning section rather than restating the drain.
- `TASK-QUEUE.md` — `kfric-capture-unverified-assertion` promotes to New Features with this file's
  `[spec:]` ref, in the same commit as this file (all deltas).

## Definition of Done

- [ ] **Causal completeness** — the amendment creates no new state, event or interface, and says
      so; the two fields the obligation newly reads are the grammar's existing two, each with a
      named reader at a named transition.
- [ ] **The ground is cited, never restated** — drift-kit's section points at
      `lifecycle-kit/SPEC.md` §The committed gap inbox for the *why* and owns only the shape for
      this channel.
- [ ] **Both limbs are covered** — the fact and the ownership clause, with a stated disposition for
      each of the four outcomes, including the unsettleable one.
- [ ] **The template mechanizes what the SPEC rules** — the sub-step sits ahead of the two
      remediation shapes and cites the SPEC for its placement.
- [ ] **The obligation is observable** — the close commit that clears the log states what was
      re-verified and what fell.
- [ ] **The limit is on the page** — the merged section says no gate enforces this on either
      channel, so the absence is read as the sibling's posture rather than as this unit's debt.
- [ ] **The capture side is untouched** — no delta, and no merged sentence, adds a field, a flag or
      a convention to `--emit kfric` or to the log's grammar.
- [ ] **Terminal move is a DEMOTION, not a Done move** — the entry's deliverable is two axes and
      this amendment lands one, so at merge the `[spec:]` tag drops, `[design-pending]` returns,
      and the entry goes back to the **position in Deferred it was promoted from**, recovered from
      the promoting commit's own diff (canon-kit/SPEC.md §Merging an amendment). A Done move would
      assert a finished deliverable that is not finished.
- [ ] **The demoted entry is compressed back inside its cap in the same commit** — measured, not
      estimated: its extent is 51 lines less one `recurrence:` discount, so it re-enters Deferred
      exactly at `QUEUE_KIT_ENTRY_LINE_CAP` with no room for the drain-axis record. Run
      `--emit queue-index --extent` and `check-queue-entry-budget` rather than counting by hand.
- [ ] **Battery green** — `bash gate-sdk/bin/run-gates.sh` plus
      `bash gate-sdk/bin/build-native.sh`, neither discharging the other, and drift-kit's fixture
      suite clean.
- [ ] **Merged with no information lost** — each addition integrated into its proper canonical
      section, not appended.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component
      (`ls drift-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped for the retired claim that the drain's only obligation is a
      doc-owner edit; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps found during the work filed to the gap inbox.
