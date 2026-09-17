# SPEC amendment: differential-sweep

**Close keeps the instruction corpus swept by reading only what changed since the
last sweep.** One full pass of the instruction-surface clause (doctrine-kit's
Content-tiering / SSOT rule: an instruction surface carries the instruction, its
grounds go to the owning section) landed over the template and agent-definition
corpus. Its result decays from the next template edit onward. The deliverable is
the differential successor, plus the answer to the question the entry left open:
which surface owns the range base.

**The design is an audit-roster class, not a new close step.** The class the
sweep judges, what-vs-why, is un-gateable, and §The audit roster is the kit's
mechanism for exactly that: a manual duty with an event-keyed cadence that close
step 8 already reviews. A new numbered close step would be a second cadence
mechanism for one class. It would also put the class on a surface the class
itself governs. So the kit gains one generic derivation and the consumer gains
one roster block. Which classes a tree runs is consumer content (§The audit
roster), so the block is this repo's, and the kit ships no class.

**What stays as it is.** Close's step numbering and step 8's review procedure, the
roster grammar and `check-audit-roster`'s four assertions, the iteration-start
commit and every reader of it. No knob is added and no gate is added. The judgment
stays ungated. Only its worklist is derived.

## What changes

### (1) The previous-close commit is a named derivation beside the iteration-start commit

§The state machine defines the **previous-close commit**: the `<head>` of the
last stamp naming the last configured stage, in the state file as it stood at the
iteration-start commit {design-bearing}. **Not yet applied.**

- **The derivation.** Read the state file at the iteration-start commit (`git show
  <iteration-start>:<state-file>`, the path repo-relative) and take the `<head>` of
  its last data line whose `<stage>` is `LIFECYCLE_KIT_STAGES`' last member. At the
  iteration-start commit the boundary truncation has not yet run, so that blob still
  holds the previous iteration's stamps. Probed on this tree: the command yields
  `bea4dd49`, the previous iteration's close entry.
- **Why the iteration-start range is not enough.** The iteration-start commit is
  HEAD at the first stage's entry. Everything the previous close committed after its
  own audit review lies before that commit, and so does any interstitial commit.
  Neither lies in the iteration-start range. On this tree the previous close
  reviewed its roster at `58e9a8ed` and ran its brevity pass at `a024f14f`, the
  iteration-start commit itself, so `a024f14f..HEAD` excludes the brevity pass's
  own diff. `<previous-close>..HEAD` covers all of it.
- **The overlap is the price, and it is stated.** The range also re-covers the
  previous close's commits from its entry up to its audit review, which that review
  already read. A re-read of already-swept lines costs reading time. A hole costs an
  unswept edit, so the overlap is taken.
- **No previous-close commit** where there is no iteration-start commit, where the
  blob at it carries no last-stage stamp (a first iteration, or a tree adopting
  the kit mid-history), or where that `<head>` is `none` or does not resolve. A
  reader states what it does then. The class in delta 3 falls back to its full-pass
  base.
- **A derivation in prose, not an adapter.** Its only reader is a closing session
  deriving a corpus. A `stages::` member earns its place when two readers must agree
  (§The stage-machine adapters), and one reader does not meet that bar. A second
  mechanical reader would make it an adapter.

### (2) A range-derived audit class may base its range on the previous-close commit

§The audit roster's `corpus` paragraph re-phrases to name the two range bases,
and close step 8 derives a class's range from its `scope:` {mechanical}.
**Not yet applied.**

- **§The audit roster**, the `corpus` paragraph: a class whose `scope` derives
  its corpus from a commit range bases that range on the iteration-start commit by
  default. A class that must also see the previous close's own later commits and
  interstitial commits names the previous-close commit (§The state machine)
  instead.
- **Close step 8**, replacement text for its second sentence's first clause
  (instruction only):
  `Per audit: derive the corpus from \`scope:\` and the range it names (this iteration's range unless \`scope:\` names another base); the predecessor's \`corpus:\` is a floor;`
  The rest of the step is unchanged.

### (3) This repo's roster gains the instruction-surface-tier class

`.workflow/audit-roster.txt` gains one block, a never-swept class {mechanical}.
**Not yet applied.** Its text, each line under `LIFECYCLE_KIT_AUDIT_ROSTER_LINE_CAP`:

```
class: instruction-surface-tier
scope: an instruction surface carrying grounds or history rather than the instruction alone (doctrine-kit/DOCTRINE.md rule 1, Content-tiering / SSOT); un-gateable, since telling a ground from a scoping clause or a mechanism the reader needs is a reading. Corpus: git diff --stat <base>..HEAD -- '*/templates/*.md' '.claude/agents/*.md' ':!*/gate-tests/*', read hunk by hunk. <base> is the previous-close commit (lifecycle-kit/SPEC.md §The state machine); while last: is never, it is 63dce1be, the full pass's landing commit, whose message lists the compliant calls a sweep inherits rather than re-deciding. Judge added and rewritten lines only. A verbatim move keeps its earlier verdict. A ground goes to the section owning the mechanism in the sweep's own commit; history is deleted; a passage whose owner already carries the ground is deleted. queue-kit/templates/TASK-QUEUE.md is a grammar skeleton and outside the rule. Binding shims (.claude/commands/) are outside this class; check-shim-restatement holds their restatement half.
due: any change to the corpus since <base>
last: never
```

- **The glob excludes `gate-tests/`.** Probed: without it the pathspec matches
  a fixture (`gate-tests/…/good/demo-kit/templates/guide.md`). With it, `git
  ls-files` over the same pathspec lists the twenty files the full pass swept.
- **The first base is the full pass's landing commit, not the previous close.**
  Template edits landed between the full pass and the previous close. The
  previous-close base would leave them unswept, which is the decay the unit exists
  to stop.
- **The first sweep is this iteration's close**, at step 8, stamping `last:`,
  `corpus:`, `hits:` and `declined:` as the grammar requires.

## Producers and consumers

- **The previous-close commit (delta 1).** Producer: `--enter-stage`'s existing
  stamp writes, since every close stamp records its `<head>`, and git history keeps
  the pre-truncation blob. No config enables it beyond `LIFECYCLE_KIT_STATE_FILE`
  and `LIFECYCLE_KIT_STAGES`, which every deployment sets by default. Consumer: the
  closing session sweeping a class whose `scope` names it, at close step 8, by
  running the two-command derivation. No field is minted.
- **The `scope`-named range base (delta 2).** Producer: a roster block's `scope`
  line. Consumer: the close step 8 sweeping session, reading `scope` to derive its
  corpus, which is `scope`'s existing reader (§The audit roster). No new reader.
- **The class block (delta 3).** Producer: build, once, writing the block.
  Consumers: `check-audit-roster` (grammar, line cap, unique class; assertion D
  reads nothing for `last: never`), `--emit close-surfaces` through the roster's
  existing `advisory` declaration, and close step 8's review. Each field's reader
  is §The audit roster's. `due` is judged against `<base>..HEAD`, which is a
  non-empty diff over the corpus.
- **Point 5 (narrowing).** Nothing narrows. The glob exclusion scopes a new
  class's corpus and removes no reader's population.
- **Point 6 (members).** The obliged corpus is the class's twenty files, enumerated
  by `git ls-files -- '*/templates/*.md' '.claude/agents/*.md' ':!*/gate-tests/*'`.
  Each member's satisfying value is the full pass's verdict at `63dce1be`, compliant
  after its edits, so the first sweep reads only lines changed since then.

## Existing sections updated

- `lifecycle-kit/SPEC.md` §The state machine: the previous-close commit paragraph,
  after the iteration-start paragraph and phrased on its pattern (delta 1).
- `lifecycle-kit/SPEC.md` §The audit roster: the `corpus` paragraph names the two
  range bases (delta 2).
- `lifecycle-kit/templates/stages/close.md` step 8: the range clause (delta 2).
- `lifecycle-kit/SPEC.md` §templates/stages/: no paragraph. The class needs no
  close-template ground, since step 8's ground is §The audit roster's (delta 2).
- `.workflow/audit-roster.txt`: the `instruction-surface-tier` block (delta 3).
- `TASK-QUEUE.md`: `close-differential-instruction-sweep` moves to Done at merge,
  by the build session that merges this file, before the drain stage is entered
  (all deltas).
- `.workflow/release-declarations.md`: one bullet naming the previous-close
  derivation and the step 8 range clause, both reaching a vendoring consumer
  (deltas 1 and 2).
- <!-- update-target-exempt: generated projections, each rostered with its freshness gate and regen command in docs/site-architecture.md §Generated projections and their freshness gates --> The `docs/` mirror of the kit's SPEC.

Roster produced by: `grep -rn "iteration-start commit\|this iteration's range"`
over the tracked tree, excluding `docs/`, `TASK-QUEUE.md` and `.workflow/`. Its
other hits, in four kits and the crate, read the iteration-start commit,
which this amendment leaves unchanged. The roster is a floor that build re-derives.

## Retired spellings

- None — no delta of this amendment retires a spelling; delta 2 re-phrases a
  clause whose words are not a name any surface matches.

## Definition of Done

- [ ] **Causal completeness** — every point of the kit's causal-completeness
      check holds for the previous-close commit, the `scope`-named base and the
      class block.
- [ ] **Instruction surfaces: instruction only** — close step 8's replacement
      clause carries no grounds; delta 1 places them in §The state machine.
- [ ] **Merged with no information lost** — the refused new close step, the
      refused adapter and the overlap's price survive in the merged §The state
      machine and §The audit roster prose, undated and without this tree's probe
      shas, which stay in the amendment and the commit history.
- [ ] **Derivation probed at merge** — the delta 1 command, run on the merged
      tree, prints the previous iteration's close-entry head.
- [ ] **Roster block green** — `check-audit-roster` is clean over the new block.
- [ ] **Queue move placed before the drain stage** — the Done move lands in the
      session that merges this file, before the drain stage is entered.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls lifecycle-kit/SPEC-*.md`) once the iteration's last batch lands.
- [ ] **Gaps filed** — cross-component gaps found during the work go to the gap
      inbox.
