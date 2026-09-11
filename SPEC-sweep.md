# SPEC amendment: instruction-sweep

**No delta is applied; every deletion and relocation below is Not yet applied.**

This amendment serves `instruction-surface-sweep`: one full pass across the instruction corpus,
applying the instruction-surface clause that SPEC-instruction-tier.md delta 1 places in the doctrine's
Content-tiering rule. The entry is blocked by `instruction-motivation-owner` because that clause is
the predicate this sweep applies, so that amendment lands first.

It is a root-level amendment: its deltas span lifecycle-kit, delegation-kit, canon-kit and this
repo's `.claude/agents/`.

## The grounds this design rests on

This section is amendment-only rationale and does not merge. Every claim in it was probed on
2026-09-12 at the commit that stamped this stage, 05e38ca1, and every line range below is at that
commit. Build re-locates each passage by its content, because SPEC-instruction-tier.md shifts
`build.md` and `SPEC-amendment.md` before this sweep reaches them.

**The corpus is derived, and one member falls outside the rule.**
`git ls-files "*/templates/*.md" ".claude/agents/*.md"` minus the `gate-tests` fixture copies returns
20 files, 2920 lines.

- `queue-kit/templates/TASK-QUEUE.md` is a queue skeleton a project copies once. Its comments state
  section and row grammar, not instructions to a session acting under them, so the clause does not
  reach it.
- `canon-kit/templates/SPEC-amendment.md` is a skeleton too, but its comments instruct the authoring
  session at the moment it writes, so the clause reaches it.

**The worklist is a filed survey, cited behind its witness.** The census is the 2026-09-12 spec block
in `.workflow/survey-record.md`, with the witness that block prints. It classed about 2605 lines as
instruction, 81 as history, and about 229 as grounds. About 114 of the grounds lines are already
carried by their owner and 113 are not. Spec spot-checked the load-bearing calls against the tree
and corrected two:

- The tightened-gates grounds in `build.md` belong to gate-sdk/SPEC.md §upgrade-smoke.
- The reversal-trap sentence in `stage-session.md` is already carried by lifecycle-kit/SPEC.md
  §templates/lead.md, so it is delete-only, not relocation-owed.

Before deleting any passage as already carried, build re-verifies the call against its owner at that
passage.

**An attested failure an instruction answers is grounds, not history.** This is delta 1's bound in
SPEC-instruction-tier.md. The census classed the pin-path grammar case in `align.md` as history, but
it is the ground of the check it sits under. SPEC-align-claims.md replaces that paragraph and moves
the case to §templates/stages/, so this sweep leaves that paragraph to that amendment.

**Compliant calls go in the landing commit's message, not on a surface.** The entry asks that
compliant calls be stated so the next differential sweep inherits a baseline. A per-file ruling is
history about the tree, and the operator direction on the entry makes version control the home of
history. The one reader that would consume such a record is the filed differential pass,
`close-differential-instruction-sweep`, which derives its worklist from `git diff` over a range. That
same range yields the landing commit's message through `git log`, so no new surface is owed.

**`lead.md` and `agent-execution.md` go first.** The lead contract loads both whole, and together
they hold 73 of the 81 history lines and about 161 of the grounds lines. The entry's direction line
records that ordering.

**Why a sweep edit keeps what it keeps.**

- **Bold lead-ins.** `check-rule-citation` resolves delegation-kit/SPEC.md's citations of
  agent-execution rules against that template's bullet lead-ins. Hook help texts and other SPECs also
  cite rules by lead-in.
- **Named slots.** `check-skill-binding` holds template and binding in slot parity.
- **The stamp first step and the journal last step.** `check-stage-skill-coverage` requires both.
- **The wording of every rule a `.claude/agents/` definition restates**, and the template's propagate
  header. Under §Operative residency, a changed rule propagates to its carriers, and carriers are
  found by grepping the rule's phrasing, never from a roster.

**Relocated grounds land undated.** The provenance seam forbids a dated stamp or a session reference
in a kit SPEC. A relocated failure keeps its mechanism and loses its date, its session and its
tallies. The canon-kit manifest gates judge the landed text.

**Relocation re-phrases; it does not append.** Operator direction, 2026-09-12: SPECs are not
expected to grow each iteration; re-phrase existing text for clarity and brevity, add only what
adds value, and prefer brief phrase-shaped wording. A relocated ground therefore lands as the
shortest phrase that keeps its instruction justified, folded into the owner's existing sentence
on that mechanism where one exists. A ground its instruction stands without is deleted, not
relocated. The landing commit states each destination SPEC's net line change.

**Stage-template grounds land as paragraphs, not new headings.** lifecycle-kit/SPEC.md
§templates/stages/ already carries per-template contract as paragraphs led "The `close` template
carries…". New subsections would add headings for pointers to resolve, and no reader needs one.

## What changes

### (1) `lead.md` loses its history and its grounds

In `lifecycle-kit/templates/lead.md`, every history passage is deleted, grounds its owner already
carries are deleted, and uncarried grounds relocate into lifecycle-kit/SPEC.md §templates/lead.md
{design-bearing}. §templates/lead.md is already a dense compressed reading of this template, so
relocated grounds integrate at the clause they justify rather than accreting at the section's foot.
The census calls, each re-verified at the passage:

- **Delete — history:**
  - L82–87 and L144–146: the validate session still writing after its evidence read green.
  - L419–422: a completed oracle's rows re-run and lost.
  - L452–455: a relayed tool-use count discrediting a survey.
- **Delete — grounds the owner carries:** L71–76, L146–149, L223–238, L258–264, L280–282,
  L286–293, L310–319 and L372–378, plus the §Economics rationale for cost, split and compaction,
  except the two passages below.
- **Relocate:**
  - L162–167: the asymmetry behind relay-never-assert.
  - L402–406: who a misnamed journal costs.
  - L438–443: a checkable duration or count standing in for the property it measures.
  - L510–513: a backdated stamp falsifying the trail.
  - L561–565: the producer/consumer edge a batch cut must not split.
  - L575–594: the justification for tiering each batch to its work class.

### (2) `agent-execution.md` loses its history and its one uncarried ground

In `delegation-kit/templates/agent-execution.md`, history is deleted, carried grounds are deleted,
and the uncarried ground relocates into delegation-kit/SPEC.md §The delegation model
{design-bearing}. Where a sweep edit changes the wording of a rule that `.claude/agents/stage-session.md`
or `.claude/agents/audit-sweep.md` restates, the carrier is updated in the same commit.

- **Delete — history:** L149–152, L159–161, L216–220, L236–240, L287–293, L307–310, L342–348 and
  L395–398.
- **Delete — grounds the owner carries:** L220–222, L302–306 and L474–477.
- **Relocate:** L195–198, the harness lock-pid trap stated beside the reap step.

### (3) The stage templates' grounds move into §templates/stages/

In each stage template, uncarried grounds relocate into lifecycle-kit/SPEC.md §templates/stages/ as
brief phrases folded into an existing paragraph on that template, or a short paragraph led
"The `<stage>` template …" where none exists, and carried grounds are deleted {design-bearing}. The
paragraph and ritual line SPEC-instruction-tier.md delta 3 lands are left as landed.

| Template | Relocate into §templates/stages/ | Delete |
| --- | --- | --- |
| `align.md` | L47–51 (the surviving surface wins: a correction written into an amendment dies with it at merge); L94–97 (authoring a producer asserts the consumer's read side) | L90–92, history. L71–75 is SPEC-align-claims.md delta 1's to replace. |
| `build.md` | L42–45 (why one fresh session per task) | L87–90, carried by gate-sdk/SPEC.md §upgrade-smoke |
| `close.md` | L260–268 (the push-identity step's grounds) | — |
| `scope.md` | L102–106 (the recurrence-override failure analysis); L208–210 (why a split sums its siblings' weight) | — |
| `spec.md` | L72–93 (the inline work-class label: why inline and not rostered, why a demand and not a model name, why spec holds the judgment) | — |
| `validate.md` | L67–73 (why a repair is committed before the suite resumes) | — |

### (4) The boundary skills and the stage-session definition shed their grounds

One passage relocates, and two are deleted because their owner already carries them
{design-bearing}. Each deletion is re-verified against its owner first:

- `lifecycle-kit/templates/consult.md` L10–12 — the paragraph opening "A consultation that ends with
  its rulings still in the transcript has not exited" — **relocates** into lifecycle-kit/SPEC.md
  §templates/consult.md, beside that section's landing-contract paragraph. The section states the
  landing contract but not the failure it prevents, so the census's delete-only call does not hold.
- `lifecycle-kit/templates/release-sweep.md` L7–11 — the paragraph opening "Between majors,
  `check-deprecation-task` (canon-kit) already holds" — against lifecycle-kit/SPEC.md
  §templates/release-sweep.md, which states the same between-majors and boundary split.
- `.claude/agents/stage-session.md` L29–33 — the sentence opening "The trap it exists for is not
  sloppy reasoning" — against lifecycle-kit/SPEC.md §templates/lead.md's clause that a session holding
  contrary evidence reads the surface carrying a ruling as stale rather than as closed. The imperative
  beside it stays.

### (5) The amendment skeleton's placement ground moves to §The spec model

`canon-kit/templates/SPEC-amendment.md` L12–15, from "Say this out rather than leaving it to
precedent" to the end of that sentence, relocates into canon-kit/SPEC.md §The spec model, beside its
sentence on root-level amendments {design-bearing}.

### (6) The generated projections are regenerated

`docs/lifecycle-kit/SPEC.md`, `docs/delegation-kit/SPEC.md`, `docs/canon-kit/SPEC.md`,
`docs/footprint.md` and `docs/value.md` are regenerated, each with the command its freshness gate
prints {mechanical}.

## The compliant calls the landing commit records

- **Whole files:** `.claude/agents/audit-sweep.md`, `context-kit/templates/close-brevity.md`,
  `delegation-kit/templates/dispatch-checklists.md`, `drift-kit/templates/close-knowledge.md`,
  `drift-kit/templates/economics.md`, `guard-kit/templates/close-triage.md` and
  `lifecycle-kit/templates/upgrade.md`.
- **Borderline passages, kept.** Each is a one-clause because serving one imperative, a scoping
  condition, or mechanism the reader needs in order to follow the rule:
  - `lead.md` L172–180, L300–308 and L457–466.
  - `agent-execution.md` L139–142, points (1) to (3) of the isolation-costs bullet in L169–214, and
    L326–336.
  - `build.md` L48–56 and `close.md` L246–253.
- **Outside the rule:** `queue-kit/templates/TASK-QUEUE.md`.

## Producers and consumers

- **No new state, event, field or interface.** The sweep removes content from instruction surfaces
  and places grounds in existing SPEC sections.
- **Narrowing (point 5).** The kit templates are a governed corpus this sweep narrows, so each
  reader's red condition is named.
  - `check-rule-citation` reds when a SPEC citation resolves to no bullet lead-in. That is not
    monotone under narrowing, because removing a lead-in adds a violation. Every lead-in is
    therefore kept byte-identical (delta 2).
  - `check-skill-binding` reds on a slot mismatch; no slot is touched (deltas 1, 3 and 4).
  - `check-stage-skill-coverage` reds when an executed stage surface lacks its stamp or journal
    step; neither is touched (delta 3).
  - `check-shim-restatement` reds on a span a binding shim copies from the template corpus. That red
    is monotone in removed text.
  - `check-footprint-fresh` and `check-value-rollup-fresh` red on a stale byte-compare of pages that
    count template lines, and are regenerated (delta 6).
  - drift-kit's overhead meter matches the string `lifecycle-kit/templates/stages` inside transcript
    lines and reads no template file, so it is unaffected.
  - A quotation of a deleted passage elsewhere — a docs page, a shim, a queue entry — is a stale
    claim no gate reads. Build greps each deleted passage's distinctive phrase across the tracked
    tree before deleting it (all deltas).
- **The destination SPECs grow.** Relocated text meets the canon-kit manifest gates: links, section
  pointers, bare counts, temporal markers and knob citations. Build runs the battery rather than
  predicting their verdicts.
- **The always-loaded ratchet** (context-kit/SPEC-surface-ratchet.md) stamps its first ceilings after
  this sweep lands, so the ceilings hold the swept sizes.

## Existing sections updated

- `lifecycle-kit/SPEC.md` §templates/lead.md — relocated lead grounds (delta 1).
- `delegation-kit/SPEC.md` §The delegation model — the lock-pid trap (delta 2).
- `lifecycle-kit/SPEC.md` §templates/stages/ — relocated stage grounds (delta 3).
- `lifecycle-kit/SPEC.md` §templates/consult.md — the failure the landing contract prevents
  (delta 4).
- `canon-kit/SPEC.md` §The spec model — the placement ground (delta 5).
- `.claude/agents/stage-session.md` and `.claude/agents/audit-sweep.md` — any restated rule whose
  wording delta 2 changes (delta 2).
- `docs/lifecycle-kit/SPEC.md`, `docs/delegation-kit/SPEC.md`, `docs/canon-kit/SPEC.md`,
  `docs/footprint.md` and `docs/value.md` — generated (delta 6).

## Retired spellings

- None — every bold lead-in and named slot is kept byte-identical, and a deleted passage retires no
  name another surface cites; the per-passage grep in §Producers and consumers enforces the second
  half.

## Definition of Done

- [ ] **Causal completeness** — no new state or field; every reader of the narrowed corpus named with
      its red condition.
- [ ] **Instruction surfaces carry instructions** — every corpus file read against the clause; each
      violation deleted or relocated to a named section; the compliant calls above recorded in the
      landing commit's message.
- [ ] **Merged with no information lost** — every relocated ground reads whole in its section,
      undated; every deleted history passage remains reachable through `git log -p` on its file.
- [ ] **Merged by re-phrasing** — each relocated ground is a brief phrase folded into existing
      owner prose where one covers it; the landing commit states each destination SPEC's net
      line change (operator direction, 2026-09-12).
- [ ] **Amendment deleted** — this file removed on merge; no root-level `SPEC-sweep.md`
      remains.
- [ ] **Removals propagated** — `check-amendment-retired-spelling` green; restated rules propagated to
      their carriers.
- [ ] **Gaps filed** — any gap discovered during the work filed through the gap inbox.
