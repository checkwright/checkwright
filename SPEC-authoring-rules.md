# SPEC amendment: authoring-rules

Queue entries, all three selected for `amendment-authoring-completeness` (operator direction,
2026-09-15, lead-relayed), paired to this one amendment because their edits meet on the same
passages:

- `causal-completeness-has-no-per-member-satisfiability-point` — deltas 1 and 4.
- `directive-minting-delta-roster-obligation` — deltas 1, 2 and 4.
- `spec-growth-restraint-unstated-at-authoring-surfaces` — deltas 3, 4 and 5, and the line budget
  every delta holds.

Sited at the repo root: the change spans canon-kit, lifecycle-kit and context-kit.

## The questions, and what this stage probed

**Per-member satisfiability.** canon-kit/SPEC.md §The causal-completeness check carries five
points, and none asks what value satisfies an obligation for each member of the corpus it ranges
over. The attested instance is `SPEC-reads-root-default.md` delta 2: five of 26 members had no knob
to declare. The entry leaves one design question open: a sixth point, or a clause on point 4.
**Ruled: a sixth point.** Point 4 is about fields on a message, a producer/consumer edge. This
obligation is about corpus members, and that is not a field.

**Directive minting.** The entry proposes a gate: red when a delta body mints a `<word>:` comment
token absent from `check-comment-tier`'s built-in roster. It calls this slice decidable without
reading the tree. **Probed, and it is not.** The corpus was every amendment deleted from this
repository's history, 480 deletions outside `templates/`. Each file's last version before deletion
had its `## What changes` body grepped for comment-leader colon tokens (`(#|//) ?<word>:`). The
result was compared with the roster literal in `native/src/gates/comment_tier.rs`, which has 13
entries.

- 215 amendments carry such a token, 29 distinct tokens in all. 15 are absent from the roster, and
  none of the 15 ever needed a roster row. Their live sites are gate-test fixtures, workflow YAML,
  `smoke/` scripts, `.gate` descriptors, trailing position, or a line inside another directive's
  window. `check-comment-tier` either does not classify those sites or has already blessed them.
- The obligation depends on a fact the amendment does not state: whether the name lands as a
  full-line comment on `CANON_KIT_COMMENT_SURFACE`. So the proposed gate would fire mostly on names
  that owed no row. That is the cry-wolf shape gate-sdk/SPEC.md §When a gate earns its place bars.
- **The guarantee is already delivered at the site.** `check-comment-tier` reds on the first
  full-line comment carrying an unrostered directive. The portability-floor instance was caught
  exactly that way, at build. What is missing is the prompt at authoring time, and a checklist
  clause supplies it.

The probe is filed in `.workflow/survey-record.md`.

**Ruled: no new gate.** Point 2 gains a clause naming the roster-holding reader, and
§check-amendment-update-target records the refused third slice beside the two it already refuses.
Nothing is added that a later reader must maintain.

**Growth restraint.** The direction: a SPEC is not expected to grow each iteration. A feature or an
ambiguity is handled by re-phrasing existing text for clarity and brevity, and text is added only
where it adds value. None of the surfaces that decide the next merge states this. **Ruled: land it
by re-phrasing, and hold this amendment to it.** No edited file ends longer than it began:

- the templates already sit under `check-surface-ratchet` ceilings;
- canon-kit/SPEC.md and context-kit/SPEC.md are held by this amendment's own Definition of Done.

`lifecycle-kit/templates/stages/build.md` is **not** edited. Its binding merges an amendment per
canon-kit/SPEC.md §Merging an amendment, so delta 3 already reaches it, and repeating the rule in the
template would be a second copy.

## The seam

Kit mechanism: canon-kit's checklist, merge rule and amendment template, lifecycle-kit's spec and
scope stage templates, and context-kit's brevity pass. No private rule content, no consumer config,
no knob, no gate. The checklist example names `check-comment-tier`, a canon-kit gate. A consumer's
directive extras stay in `CANON_KIT_COMMENT_MACHINE` / `_REASON`, and point 2's clause covers them
without naming them.

## What changes

Every delta holds this: **the file it edits does not end longer than it began.** Build checks it
with `wc -l` before and after, per file.

### (1) The causal-completeness check names roster readers and per-member values {design-bearing}

canon-kit/SPEC.md §The causal-completeness check is replaced whole, through the end of its
cross-component-gap paragraph. **Not yet applied.**

The replacement makes four changes:

- The intro drops its count (`passes five points`), which a sixth point would make stale.
- Point 2 gains the roster-reader clause.
- Point 6 is new.
- Points 1, 4 and 5 and the closing paragraph are tightened to pay for the additions.

Points keep their numbers, so three existing citations stay true: gate-sdk/SPEC.md's "three
non-monotone shapes … point 5 names", guard-kit/SPEC.md's "point 4", and the amendment template's
"point 5". The section must not come out longer than the one it replaces.

```text
### The causal-completeness check

Before an amendment is ready, every new state, event, interface and obligation
it introduces passes these points:

1. **Producer named and reachable** — what code path, call, or timer triggers
   it; a producer whose enabling config no deployed configuration sets is dead
   everywhere but unit tests.
2. **Consumer named** — what receives it, by what mechanism (stream, call,
   poll), and every roster-holding reader of the surface it lands on: such a
   reader reds on a name its roster lacks, so a minted name lists that roster
   as an update target (a comment directive meets `check-comment-tier`'s).
3. **Existing integration sections updated** — any spec section describing the
   prior flow is updated in the amendment itself.
4. **Every field has a named reader** — the consumer and transition reading each
   field of a new message; a field with no reader is removed, and one read at
   one transition is not populated at others.
5. **Each reader's red condition named, not merely its subject** — binding on a
   delta that *narrows* a corpus (a prune, a tighter glob, a dropped file). Only
   a verdict monotone in the violation set clears by inspection, and three
   ordinary shapes are not: a reader that reds on *finding none*, one asserting
   an exact count, one holding a minimum or coverage floor. "A narrower corpus
   can only remove violations" is false: pruning the file holding a
   declaration's sole instance flips `check-install-claim` red on a zero count.
6. **Every member's satisfying value named** — binding on a delta obliging each
   member of a corpus enumerable at authoring time: enumerate the members by a
   named probe and name each one's value; a member with none narrows the
   assertion here, because build can neither satisfy nor narrow it.

A cross-component causal gap surfacing at build is not a deferred TODO: stop,
resolve it that session, update the spec before resuming. lifecycle-kit's stage
templates carry these hooks; canon-kit owns the checklist and the promotion gate.
```

### (2) The refused directive slice is recorded beside the two refused arms {mechanical}

canon-kit/SPEC.md §check-amendment-update-target: the **Deliberately not asserted: roster
completeness** paragraph is replaced whole. The replacement adds the third refused arm and pays for
it by tightening the rest. It must not come out longer than the paragraph it replaces. **Not yet
applied.**

```text
**Deliberately not asserted: roster completeness.** Whether the roster names
every surface the change obliges a write to is a claim about the world, not the
file, and no scanner reaches it; arm B catches a target *listed and unowned*,
never one never listed. **One narrow slice of that half is mechanized**, by
§check-amendment-retired-spelling rather than a fourth arm here: retiring a
*literal* leaves the retired and replacing spellings in disjoint token spaces,
so a survivor scan reconciles against this roster. The residue is carried by
the align stage, which reads it against the tree, and by the build stage, which
re-derives the roster before the merge counts as complete
(lifecycle-kit/SPEC.md §templates/stages/) — which is why a roster names the
probe that produced it rather than claiming completeness
(`templates/SPEC-amendment.md`), a claim that invites the merge to skip that
re-derivation. The residue is a stale prose sentence, a semantic over-claim, a
cross-reference dangled by a deletion, and the renumber case, whose two
spellings share one token space. Three stronger arms were refused. **Every
delta cited by some target** is false — a delta adding a new section touches no
existing one. **Every path or `§` reference in a delta body rostered** cries
wolf, since a delta names many surfaces for context (gate-sdk/SPEC.md §When a
gate earns its place). **Every comment directive a delta mints rostered** cries
wolf too: a name obliges `check-comment-tier`'s roster only where it lands as a
full-line comment on the governed surface, which the amendment does not say,
and names minted for fixtures, workflow files, descriptors or a trailing
position owe no row. `check-comment-tier` reds at an obliged site regardless,
and §The causal-completeness check point 2 prompts the row at authoring.
```

### (3) The merge integrates by re-phrasing {mechanical}

canon-kit/SPEC.md §Merging an amendment: step 2 is replaced whole, keeping its line count. This is
the owner of the restraint rule, and every other surface points here. **Not yet applied.**

```text
2. Integrate by re-phrasing, never by appending: a spec is not expected to grow
   each merge, so an addition rewrites the instruction it refines — clearer,
   briefer, phrase-shaped — and adds text only where no rewrite carries it. The
   merged spec reads as one document to a reader who never saw the amendment.
   Design rationale relocates into the spec's prose; an embedded wire-delta
   becomes a citation to the contract file — the exemption is file-scoped, so
   once step 3 deletes it `check-spec-embedded-source` re-arms on a kept embed.
```

### (4) The authoring surfaces point at the checklist instead of restating it {mechanical}

Four instruction-surface edits. Each carries the instruction only. **Not yet applied.**

- `lifecycle-kit/templates/stages/spec.md`: the paragraph opening **Verify causal completeness before
  declaring an amendment ready** is replaced by the two paragraphs below. Today it restates points 1
  to 4 and omits 5, so a sixth point would widen the gap.

  ```text
  **Verify causal completeness before declaring an amendment ready** — every
  point of the checklist your canon-kit owns, for each new state, event,
  interface and obligation. Survey each reader across the **whole component
  set**, never a hand-picked subset, and never silence a probe's stderr — a
  `2>/dev/null` on a path grep reads a bad path as "no reader", the false
  negative that hides a cross-component reader.

  **Replacement text re-phrases; it never appends.** Rewrite the passage a
  delta refines, clearer and briefer, and add text only where no rewrite
  carries the change (canon-kit's amendment-merge rule).
  ```

- `lifecycle-kit/templates/stages/scope.md`: the default-roster authoring paragraph's parenthetical
  (`every new field's producer, consumer, and named reader, surveyed across the whole component
  set`) becomes `canon-kit's checklist, surveyed across the whole component set`. Rewrap the
  sentence so it is one line shorter.
- `canon-kit/templates/SPEC-amendment.md`: the `## Producers and consumers` comment is replaced by
  the text below, in the same ten lines.

  ```text
  <!-- The causal-completeness check (SPEC §The causal-completeness check), for
       every new state, event, interface and obligation:
       — Producer: what triggers it, and the enabling config some deployed
         configuration actually sets — not test-only.
       — Consumer: what receives it, by what mechanism — and any roster-holding
         reader of the surface a minted name lands on (point 2).
       — Every field has a named reader at a named transition, or is removed.
       — Narrowing a corpus? name each reader's RED CONDITION (point 5).
       — Obliging every member of an enumerable corpus? name each member's
         satisfying value, or narrow past a member with none (point 6). -->
  ```

- `canon-kit/templates/SPEC-amendment.md`, Definition of Done. Two items are re-phrased and keep
  their bold labels: canon-kit/SPEC.md's paragraph after §Merging an amendment names the checklist
  items by those labels.
  - **Causal completeness** becomes two lines:
    `every point of SPEC §The causal-completeness check holds for each new state, event, interface
    and obligation.`
  - **Merged with no information lost** keeps its three lines:
    `each addition re-phrases the canonical-spec text it refines rather than appending to it; the
    merged spec reads as one document a reader who never saw the amendment can use alone.`

### (5) The brevity pass reads growth against no-growth, not against net-additive {mechanical}

`net-additive by design` is the ground the brevity pass gives for reacting to the delta rather than
the level, and it contradicts the direction. It is re-phrased in both places that state it, and
neither place grows. **Not yet applied.**

- `context-kit/templates/close-brevity.md`, opening paragraph: the clause from `close is net-additive
  by design` to the end of the sentence becomes `a file is not expected to grow each iteration, so
  growth since the iteration baseline is the worklist.` The closing **Goal** sentence becomes
  `Goal: a governed prose file grows only where re-phrasing could not carry the change, and every
  session pays for context that is still true and still terse.`
- `context-kit/SPEC.md` §The always-loaded meter, the `--update-baseline` bullet: the parenthetical
  `(close is net-additive by design; only growth since the iteration started is actionable)` becomes
  `(a file is not expected to grow each iteration, so growth since the iteration started is the
  worklist)`.

## Producers and consumers

This amendment adds no state, event, field or interface. Its new content is checklist wording and
instruction text, so the points below are recorded against those instruction edges.

- **Point 6 and point 2's clause.** The producer is the authoring session applying the checklist.
  Its consumers are the causal-completeness hooks: the spec stage template (delta 4), the amendment
  template's `## Producers and consumers` comment (delta 4), and the align stage, which reads
  amendments against the tree. No gate reads the checklist text. A probe of `native/`, `*.gate`,
  `*.sh`, `*.knobs` and `scripts/` for the edited literals (`causal completeness`, `whole component
  set`, `Producers and consumers`, `no information lost`, `net-additive`, `passes five`) returned
  nothing.
- **The merge rule (delta 3).** The producer is the build session merging an amendment, which reaches
  §Merging an amendment through its binding. The consumer is the next reader of the merged spec.
  There are two mechanical floors, neither of which judges value: `check-surface-ratchet` for the
  ceilinged templates, and `--emit always-loaded --growth` at close for every governed prose file,
  SPECs included. Close reads the growth list through delta 5's re-phrased pass.
- **The refused slice (delta 2).** Its reader is the next session that proposes the directive gate.
  The obligation itself is gated at its site by `check-comment-tier`.
- **Point 5, applied to this amendment's own narrowings.**
  - Delta 4 removes a restatement from `spec.md` and `scope.md`. Every reader of those files is a
    session, and no gate asserts their wording (probe above). `check-surface-ratchet` reds only
    **above** a ceiling, which is monotone under shrinking, and `check-stage-skill-coverage` reads
    the resume-journal step, which is untouched.
  - Delta 1 drops the literal count, and no gate reads it.
  - This amendment's own `## Retired spellings` block reds while a declared spelling survives at an
    unrostered path. That is monotone in survivors, and every survivor at authoring is rostered
    below.
- **Point 6, applied to this amendment.** No delta obliges every member of a corpus, so the point
  does not bind. Delta 4's line budget is a claim about four named files, each measurable, not an
  obligation over an open corpus.

## Existing sections updated

Roster probes, run at this stage:
`git grep -n -i "causal.completeness\|five points\|point 5\|§The causal-completeness"` and
`git grep -n "passes five points\|net-additive by design\|Merged with no information lost\|Two stronger arms"`,
both over the tracked tree. A third,
`git grep -n "hand-picked subset\|whole component set"`, excluded `docs/` and `TASK-QUEUE.md`.

- `canon-kit/SPEC.md` — §The causal-completeness check (delta 1), §check-amendment-update-target's
  roster-completeness paragraph (delta 2), and §Merging an amendment step 2 (delta 3).
- `canon-kit/templates/SPEC-amendment.md` — the `## Producers and consumers` comment and two
  Definition-of-Done items (delta 4).
- `lifecycle-kit/templates/stages/spec.md` — the causal-completeness paragraph and the new
  re-phrasing paragraph (delta 4).
- `lifecycle-kit/templates/stages/scope.md` — the authoring how-to parenthetical (delta 4).
- `context-kit/templates/close-brevity.md` — the opening paragraph's ground and the Goal line
  (delta 5).
- `context-kit/SPEC.md` — §The always-loaded meter, the `--update-baseline` bullet (delta 5).
- `docs/canon-kit/SPEC.md` — the generated mirror, regenerated with
  `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` (deltas 1, 2 and 3).
- `docs/context-kit/SPEC.md` — the generated mirror, regenerated the same way (delta 5).
- `.workflow/release-declarations.md` — one bullet per shipped kit whose template or rule text moved:
  canon-kit, lifecycle-kit and context-kit (deltas 1, 3, 4 and 5).

## Retired spellings

- `passes five points` — the checklist's literal count, retired when point 6 lands (delta 1).
- `net-additive by design` — the brevity pass's ground, re-phrased against the no-growth direction
  (delta 5).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named, reachable producer and
      a named consumer; every new field has a named reader at a named transition.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition
      or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition integrated into its proper canonical-spec
      section (not appended); the merged spec reads as one coherent document a reader who never saw
      the amendment can use alone.
- [ ] **No edited file grew** — `wc -l` on each rostered non-generated file is no higher after the
      merge than at the commit before it, and the commit message states the per-file figures.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the repo root
      (`ls SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings`
      above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked
      tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a
      build-time causal gap is resolved that session, not deferred).
