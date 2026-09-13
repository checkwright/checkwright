# SPEC amendment: claim-standing

Queue entries: `survey-record-claim-reliability` and `kfric-capture-unverified-assertion`,
decided together, with the rider `survey-edges-reader-arrives-after-truncation`. All three sit in
unit set `carried-record-reliability` under **operator direction, 2026-09-13, lead-relayed**.

## The question, and the one principle that answers it for three surfaces

This tree has three channels that carry a session's claim to a later reader: the committed gap
inbox, the knowledge-friction log and the survey record. Each has now carried a confident wrong
claim that a later session relied on. The gap inbox has already been ruled. Its capture carries
no claim-status grammar, and its drain re-verifies (lifecycle-kit/SPEC.md §The committed gap
inbox). The two entries ask whether the other two channels take that answer or a different one.
The inbox's refusal is recorded, so a different answer must argue against it rather than reopen
it.

**The principle: verification is paid by the party that holds the evidence, unless something
downstream is obliged to pay it.** The inbox's refusal rests on two premises, and both are about
the inbox:

1. *Capture must stay cheap.* The filer is mid-stage, has usually not read the queue, and a
   refused filing sends the finding back into session context.
2. *A drain re-verifies.* Every bullet reaches a mandatory reader that establishes its central
   claim before acting on it.

Where both premises hold, capture carries nothing and the drain pays. Where either fails, the
refusal's grounds do not reach the surface. Applying its conclusion there anyway is precedent,
not the rule.

- **The knowledge-friction log satisfies both premises.** Its drain re-verifies the fact and the
  ownership limb (drift-kit/SPEC.md §The knowledge-friction loop step 2), and its capture is
  built to be prompt-free. So it takes the inbox's answer: no capture-side field. What remains of
  the entry is a seam question, answered in delta 4.
- **The survey record satisfies neither.** A survey is *bought*, not captured. The session filing
  it has just run the oracle and holds the evidence, so placing a claim costs it almost nothing
  next to the survey. And nothing drains the record. Its readers are told the opposite: a clean
  witness licenses citing the finding *instead of* re-deriving it (§The survey record). Nobody
  downstream is obliged to pay, so the author pays. The record takes the claim-status split that
  the inbox refuses.

This is not a reversal of the inbox's refusal. Delta 3 states the distinction at both sections,
so that a reader who meets the survey record's split does not take it as licence to re-draft
the inbox's grammar.

## What the survey record's five attested errors were, sorted by what would have caught them

- **Two mechanism claims no command established.** One was the hook "never calls
  `gate_command`", false against the emitter at the time. The other was a premise called STALE
  without running the reproduction its entry named. Each fell to one command that nobody ran at
  filing time. Delta 1's split puts the author at the decision *did I run that?* for every claim
  a later stage will route on.
- **One derived figure transcribed by hand.** An `edges` sum was one short. The field takes a
  hand-typed argument for a value that `--emit queue-edges` prints deterministically in about a
  second. Delta 2 retires the field. That also answers the rider: the field's one named reader,
  the next boundary's ranking, runs after the boundary truncation has emptied the record, so no
  session has ever read the field where the SPEC says it is read.
- **Two judgments contested on the reading** (a cohort claim and a criterion verdict). No grammar
  catches these, and none is claimed. They are what the existing supersede-by-a-later-block rule
  is for.

## The seam

- **Kit mechanism:** the survey record's field grammar and citing rule, the `file-survey` and
  `cite-survey` arms, `check-survey-record`, the entry report's note, and the knowledge-friction
  seam rule with its drain limb.
- **Consumer config:** none new. `LIFECYCLE_KIT_SURVEY_RECORD_FILE` and `DRIFT_KIT_KNOWLEDGE_LOG`
  are unchanged.
- **This repo's own:** the `CLAUDE.md` capture bullet's operands.
- **Private rule content:** none in reach.

## What changes

### (1) The survey record's judgment half splits into `finding` and `inferred` {design-bearing}

`lifecycle-kit/SPEC.md` §The survey record's block grammar becomes, **Not yet applied**:

```
## <YYYY-MM-DD> <stage> — <the one-line question this survey answered>
- corpus: <git pathspec the survey covered>
- oracle: <the command whose verdict grounds it, or the literal `none`>
- rev: <full commit sha the survey was taken at>
- finding: <the judgment over what this survey's commands established, in prose>
- inferred: <each claim the survey reasoned to without running a command, or the literal `none`>
```

The paragraph that says each of the five fields earns its place changes its reader list to:
`corpus` and `rev` by the diff, `oracle` by the re-run, `finding` by the consuming session under
the witness, and `inferred` by the consuming session **before** its work turns on any claim
listed there. Two new paragraphs follow it. **Not yet applied:**

> **`inferred` is where a claim goes that nothing ran.** A claim belongs in `finding` only when a
> command the survey ran established it — the oracle or any other — and in `inferred` otherwise:
> a mechanism read off prose, a count reasoned to rather than printed, a premise judged from its
> description. A verdict on a premise whose owning surface names its own reproduction is
> established only by running that reproduction. Every block carries the field, with the literal
> `none` legal and an empty value refused, on the convention `oracle:` set. The obligation is what
> does the work. A block missing the field reds. An author who writes `none` has written a
> checkable statement. So what stays uncaught is an author who believed an unrun claim verified,
> and that class is strictly smaller than the one uncaught without the field. The same argument
> justifies the retired-spelling block (canon-kit/SPEC.md §The amendment lifecycle).
>
> **Why this record takes the split §The committed gap inbox refuses.** That refusal rests on
> capture being cheap for a mid-stage filer and on a drain re-verifying every bullet. Neither
> holds here. A survey is bought by a session holding the oracle's output, and nothing drains the
> record: its readers are licensed to cite instead of re-deriving. The refusal's grounds do not
> reach this surface, so its conclusion does not either.

The witness paragraph's outcome **Both hold → cite the record; do not re-buy the survey** gains,
**Not yet applied**: *"cite its `finding`. A claim in its `inferred` field is not carried by the
witness: re-establish it before your work turns on it, or carry it onward as inferred."* The
honest-limit paragraph on `oracle: none` stays as written. A block with no oracle is still a
note, and `inferred` is the same limit applied to one claim at a time.

### (2) The `edges` field retires, and a ranking's inbound sums are re-derived at their reader {design-bearing}

`lifecycle-kit/SPEC.md` §The survey record loses four paragraphs, **Not yet applied**: the one
opening **`edges` sits fourth**, the one opening **`edges` is obliged on every block**, the one
beginning *The sum itself is bought at scope*, and the affordance's paragraph opening **The arm
does not default the `edges` slot**. In their place:

> **No field carries a ranking's inbound sums, and that is a derivation rule, not an omission.**
> The sum is an oracle's output (queue-kit/SPEC.md §The queue-edges arm), printed
> deterministically in about a second, so the Derivation-first rule says it is re-derived and
> never transcribed. A transcribed sum was also unreadable where it was meant to be read. Its
> reader was the next boundary's ranking, and that boundary's own truncation empties this record
> before the ranking runs. The queue moves at close's drain and at scope's intake anyway, so a
> carried sum would be stale at its only reader. A ranking survey therefore names the
> queue-edges command in `oracle:`, where the witness re-runs it, and its `finding` cites the
> figures it ranked on without the record promising them to a later boundary.

The **Every field's git-object-shaped tokens are real** paragraph's field list becomes `corpus`,
`oracle`, `finding` or `inferred`. The witness paragraph's "the witness is five strings" list
becomes `corpus`, `oracle`, `rev`, `finding` and `inferred`. The field count stays five, so no
sentence counting it changes.

**The rider's disposition, and what it does not claim.** `survey-edges-reader-arrives-after-truncation`
framed two options: let a ranking block's `edges` half survive the boundary, or correct the SPEC's
named reader. This delta takes the second and follows it through under Derivation-first. Once
the named reader is corrected to the ranking itself, re-deriving the sum at that ranking, and no
field left carrying it, is the answer. That is a **lead decision, 2026-09-13**, taken on the
intent oracle's judgment that the retirement is in-intent. The rider's cost while deferred was
"every scope re-derives the inbound sums its predecessor bought". **That cost is accepted, not
removed.** Each scope still re-derives the sums. What changed is that the re-derivation is one
`--emit queue-edges` call of about a second, so carrying the sums is not worth a field, and a
transcribed copy was stale and once wrong.

### (3) The committed gap inbox's refusal states its reach {mechanical}

`lifecycle-kit/SPEC.md` §The committed gap inbox, the paragraph **Two capture-time shapes are
refused**, gains one closing sentence, **Not yet applied**: *"The refusal is this channel's and
reaches no surface where capture is not cheap or no drain re-verifies. The survey record is such
a surface, and it takes the split (§The survey record)."*

### (4) Knowledge-friction capture takes no claim-status field; an estimate is outside the seam {design-bearing}

`drift-kit/SPEC.md` §The knowledge-friction loop step 1, after the **Seam:** sentences, gains,
**Not yet applied**:

> **The `<surface>` field is the measured-versus-estimated distinction, so capture needs no
> other.** A kfric line records a fact *read off* a surface. Such a fact was observed by
> construction, and whether the observation holds is the drain's fact limb. A figure or mechanism
> that no surface printed — an estimate, a cost reasoned from a manifest, a behaviour inferred
> from a name — has no surface it was read off. It is not knowledge friction and is not captured
> here. A flag or a grammar would buy the same distinction with the capture rate this loop depends
> on, which is the ground §The committed gap inbox refuses its own capture-time shapes on, and
> both of that refusal's premises hold for this channel.

Step 2's remediation list gains a limb ahead of **Both hold**, **Not yet applied**:

> - **The surface field names nothing the fact could have been read off** — the line is an
>   estimate or an inference, outside this loop's seam. Drop it, and record in the close commit
>   what it claimed and, where the drain measured it, what the measurement said. If it already
>   reached a governed surface, that is work-shaped and routes to the committed gap channel, as
>   a false fact does.

`drift-kit/templates/close-knowledge.md` carries the same limb as instruction only.

### (5) The two affordances and the gate follow the grammar {design-bearing}

- `native/src/emit/file_survey.rs` — the positionals become `"<question>" "<corpus>" "<oracle>"
  "<finding>" "<inferred>"`, still five, still with no default on the last slot. The usage
  string, the stamped block and the unit tests move with it.
- `native/src/emit/cite_survey.rs` — `FIELDS` becomes `corpus, oracle, rev, finding, inferred`,
  and the unit-test record moves with it.
- `native/src/gates/survey_record.rs` (`check-survey-record`) — `WANT` becomes the same five keys
  in that order. The empty-value refusal moves from `edges` to `inferred`, with the help text
  naming `none` as the form for a survey that inferred nothing. The hex-token scan's field set
  becomes `corpus`, `oracle`, `finding`, `inferred`. Its `good/` and `bad/` fixture blocks are
  re-keyed. **Re-keying is a content rewrite, not a label swap**: the current `edges` slot's
  content (a comma-separated per-candidate sum, or `none`) has no home under `finding`, and the
  current `finding` slot's content (a prose judgment) has no home under `inferred`, so each
  fixture block's `finding:`/`inferred:` pair is re-authored — a judgment sentence under `finding`,
  `none` under `inferred` for every block, since every block in both fixtures already ran an
  oracle (`oracle: none` blocks stay notes and still write `inferred: none`, on the ground the
  field records what no *command* established, not what the block's grounds are). The `bad/`
  blocks currently exercise two `edges` shapes — an empty `edges` value and a missing `edges` key
  (`lifecycle-kit/gate-tests/check-survey-record/bad/record.md`'s 2026-01-06 and 2026-01-07
  blocks) — not three, and no block misses both `oracle` and `edges` together. Those two shapes
  move to `inferred`: an empty `inferred` value and a missing `inferred` key. The `oracle`-facing
  bad shapes (a missing `oracle` line, an empty `oracle` value, a short `rev`) are untouched. The
  `edges-token-unknown` case in `check-survey-record.test.sh` plants its token in `inferred`, and
  that file's descriptive comments naming `edges` (its `write_token_record` doc-comment's
  `[$5=edges value]` and the surrounding case comments) are reworded to `inferred` alongside it.
- `lifecycle-kit/SPEC.md` §check-survey-record — two prose spots beyond the key roster and the
  empty-value refusal retarget from `edges` to `inferred`: the paragraph opening **`edges` is
  asserted on the same two footings as `oracle`**, and the sentence *"Its input is every field but
  `rev`, `edges` included"*. The gate-model paragraph's fixture-shape sentence — *"two carrying
  `edges: none`... an **empty** `edges` and a block whose `edges` line is **missing**"* — retargets
  to `inferred` on the corrected shapes above (two `bad/` shapes, not three; the good fixture's two
  `edges: none` blocks become two `inferred: none` blocks).
- `.workflow/survey-record.md` — this repo's own live record carries three blocks filed this
  iteration, each with a live `- edges:` line. The commit landing this delta re-keys those blocks
  in the same motion (§Red conditions already states the obligation; this is that obligation's
  file).
- `lifecycle-kit/smoke/install.sh` — its `--emit file-survey` call (§check-survey-record's smoke
  coverage) currently passes `"none"` then a prose judgment as its fourth and fifth positionals,
  the old `edges`/`finding` order. Under the new order those arguments land under `finding` and
  `inferred` respectively, backwards from what they say — the fourth argument becomes the swapped
  pair: the prose judgment fourth, `"none"` fifth.
- `native/src/emit/enter_stage.rs` — the survey read trigger's note, *"run its witness (…) and
  cite it if both hold"*, becomes *"… and cite its finding if both hold — its inferred claims are
  not carried"*. The entry report is the one surface every stage session reads before it cites a
  survey.

### (6) Instruction surfaces follow {mechanical}

- `lifecycle-kit/templates/stages/scope.md` — the sentence saying **The sum lands in the survey
  record's `edges` field** becomes *"The ranking survey names the queue-edges command as its
  oracle"*.
- `CLAUDE.md` §Housekeeping, the survey-capture bullet — the invocation's last two operands
  become `"<finding>" "<inferred>"`.
- `lifecycle-kit/README.md` and the front-end's `--help`, wherever they print the file-survey
  usage.

### (7) The re-key is declared to vendoring adopters as a behavior change {mechanical}

Deltas 2 and 5 change two things a vendoring adopter meets. First, what `--emit file-survey`'s
fourth and fifth positionals mean: `"<edges>" "<finding>"` becomes `"<finding>" "<inferred>"`,
still five arguments, so an old-order call is accepted and files its fields under the wrong keys.
Second, which keys `check-survey-record` demands. That makes this a **lead decision,
2026-09-13**: the commit landing delta 5 appends a Behavior changes bullet to
`.workflow/release-declarations.md`, the release declaration surface
`gate-sdk/SPEC-release-declarations.md` mints. It is written in that surface's grammar, with a
bolded lead naming the arm. **Not yet applied:**

> - **`--emit file-survey`** (with `check-survey-record` and `--emit cite-survey`) — a survey
>   block's fields are now `corpus`, `oracle`, `rev`, `finding`, `inferred`. The `edges` field is
>   retired, and `inferred` holds each claim no command established, or `none`. The arm's fourth
>   and fifth positionals are now `"<finding>" "<inferred>"`. An old-order call still has five
>   arguments and is accepted, so update any instruction file or script that spells the old
>   order. A survey record your tree has not yet truncated at its next boundary re-keys its blocks
>   in the same motion, or `check-survey-record` reds.

**How it composes with that amendment's seed.** `gate-sdk/SPEC-release-declarations.md` delta 8
creates the surface. If this delta lands after delta 8, the bullet is appended in this delta's
commit. If it lands first, the seed carries the bullet, and delta 8's seed includes it.
`check-release-declaration-parity` then holds it through to the next note.

## Producers and consumers

- **`inferred:` (new field).** *Producer:* any session filing a survey through `--emit
  file-survey`, whose fifth positional it is, or through the sanctioned raw append. The arm is
  live wherever the kit is vendored, with no enabling config. *Consumer:* the citing session at
  whichever stage reads the record: spec, align, build, validate, close or scope, each already
  directed to the record by its template. It reads the field at the moment it would cite the
  block, and it is told to at `--enter-stage`'s read trigger (delta 5). *Gate reader:*
  `check-survey-record`, at commit, for presence, position and a non-empty value.
- **The re-key's Behavior changes bullet (delta 7).** *Producer:* the build commit landing delta
  5, or delta 8's seed in the sibling amendment. *Consumers:* close's composition of the next
  release note, and `check-release-declaration-parity` during that note's composition window.
- **`edges:` (retired field).** Its only named reader was the next boundary's ranking, which
  cannot read it (delta 2).
- **The out-of-seam kfric disposition (new drain limb).** *Producer:* close's knowledge-friction
  triage (`drift-kit/templates/close-knowledge.md`, spliced into the close skill). *Consumer:*
  the close commit message, which already records re-verification outcomes. No new field.
- **Red conditions (point 5).** `check-survey-record` reds on a block whose keys are not exactly
  the five in order, on an empty `oracle` or `inferred`, on a short or unreal `rev`, and on an
  unreal hex token in the scanned fields. The re-key changes which key it demands. It does not
  narrow a corpus, and the live record is truncated at every boundary, so no committed block needs
  migrating except any filed this iteration. The commit landing delta 5 re-keys those blocks in
  the same motion, or the gate reds that commit. `--emit file-survey` refuses on arity, which is
  unchanged at five. The knowledge-friction KPI counts non-blank lines, so a narrower seam lowers
  the count and reds nothing.

## Existing sections updated

- `lifecycle-kit/SPEC.md` — §The survey record's grammar fence, field-reader paragraph, `inferred`
  paragraphs and witness outcome (delta 1); its four `edges` paragraphs out, the derivation paragraph in,
  and the token-scan and witness field lists (delta 2); §The committed gap inbox's refusal reach
  sentence (delta 3); §check-survey-record's key roster, empty-value refusal, the "asserted on the
  same two footings" paragraph, the "every field but `rev`" sentence and the gate-model
  fixture-shape sentence (delta 5).
- `drift-kit/SPEC.md` — §The knowledge-friction loop's seam paragraph and drain limb (delta 4).
- `drift-kit/templates/close-knowledge.md` — the drain limb (delta 4).
- `native/src/emit/file_survey.rs` — positionals, usage, stamped block, unit tests (delta 5).
- `native/src/emit/cite_survey.rs` — `FIELDS` and the unit-test record (delta 5).
- `native/src/gates/survey_record.rs` — `WANT`, the empty-value refusal, the token-scan field set,
  the help text (delta 5).
- `lifecycle-kit/gate-tests/check-survey-record/good/record.md` — every block re-keyed (delta 5).
- `lifecycle-kit/gate-tests/check-survey-record/bad/record.md` — the missing-key and empty-value
  cases re-keyed to `inferred` (delta 5).
- `lifecycle-kit/gate-tests/check-survey-record.test.sh` — the `edges-token-unknown` case moves
  to a token in `inferred` (delta 5).
- `lifecycle-kit/gate-tests/survey-record-entry.test.sh` — any block it seeds re-keyed (delta 5).
- `lifecycle-kit/smoke/install.sh` — its `--emit file-survey` call's fourth and fifth positionals
  swapped to the new order (delta 5).
- `.workflow/survey-record.md` — this repo's own live blocks filed this iteration re-keyed in the
  same commit (delta 5).
- `native/src/emit/enter_stage.rs` — the read trigger's note (delta 5).
- `lifecycle-kit/templates/stages/scope.md` — the edges-field sentence (delta 6).
- `CLAUDE.md` — the survey-capture bullet's operands (delta 6).
- `lifecycle-kit/README.md` — the file-survey usage line (delta 6).
- `.workflow/release-declarations.md` — the Behavior changes bullet declaring the re-key (delta 7).
<!-- update-target-exempt: generated mirror, regenerated by its own freshness gate when its source changes; no delta edits it by hand -->
- `docs/lifecycle-kit/SPEC.md` — the site mirror of lifecycle-kit's SPEC.
<!-- update-target-exempt: generated mirror, regenerated by its own freshness gate when its source changes; no delta edits it by hand -->
- `docs/lifecycle-kit/README.md` — the site mirror of lifecycle-kit's README.
<!-- update-target-exempt: generated mirror, regenerated by its own freshness gate when its source changes; no delta edits it by hand -->
- `docs/drift-kit/SPEC.md` — the site mirror of drift-kit's SPEC.

## Retired spellings

- `- edges:` — the survey record's fourth key line, retired with its field (delta 2).
- `"<edges>"` — the file-survey arm's fourth positional in every usage spelling (deltas 2 and 5).

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
- [ ] **Release declaration** — delta 7's Behavior changes bullet is on
      `.workflow/release-declarations.md`, appended in delta 5's commit or carried by the seed.
