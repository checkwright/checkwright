# SPEC amendment: economics-fold

The stage-economics trend log keeps **one row per `(iteration, stage, model)` triple**, and a stage
that ran as several sessions on one model therefore persists **one session's draw as though it were
the stage's**, with nothing in the file marking that anything was dropped. This amendment closes
that by giving the stage rows the fold the fan-out rows already have: accumulate in memory across
the sessions of a row key, then emit **once** per key. The grammar does not change, the dedup key
does not change, and no field is added.

**The close is settled by the owner doc rather than chosen between three candidates.**
drift-kit/SPEC.md already rules this exact fold correct for the sibling row family — "One
`(iteration, stage)` can hold **several** anchors: a stage run as several sessions in one iteration
(a batch split) stamps once per session … Their subtrees **sum into the single fan-out row** for
that pair … the alternative is not a second row but a **lost** one: two appends under one
`<iteration> <stage> <model>` triple make the dedup key replace the first with the second, stranding
its transcripts attributed to a row the replacement erased." That paragraph diagnoses the stage
rows' defect precisely while fixing only the fan-out rows', and says nothing about the asymmetry.
Reading it as reaching both families is the spec-over-precedent answer, not a design choice among
three.

**Two premises on the host entry are false and the promotion corrects both**, because an amendment
built on either would be built on sand.

- **"The session id is written as a same-line field, never into the key."** It is not written to the
  log **at all**. The `who` discriminator — a session's short id, the lead's id for a supervision
  row, or `"<n> anchors"` for a folded fan-out row — appears only in the stdout report; the persisted
  line carries `<date> <iteration> <stage> <model> in= out= cr= cw= cost=` and no `who` term. So the
  candidate the entry prices as "widen the key with the session id" is not a re-keying of an existing
  column; it is **adding a column the grammar never had**, which is a materially worse trade than
  the entry states and is refused below on that corrected basis.
- **The loss needs no second run.** The entry reads as a race between two measurement runs. It is
  not: the run seeds its retained set from the existing log and writes it back whole, so two rows
  under one triple annihilate **inside a single invocation**. A first measurement of a split stage
  is already lossy.

**This amendment is one of five in an iteration spanning gate-sdk, lifecycle-kit and drift-kit**, so
`check-stage-entry` assertion C is armed on the amendment file count alone and the audit stage's
stamp will be demanded at build's entry.

## What changes

### (1) The stage pass accumulates across a row key and emits once, on the fan-out pass's own shape

Today the stage pass calls the row writer **once per session**, and lets the log's
replace-on-match arbitrate which survives {design-bearing}. The fan-out pass in the same module does
the opposite and is correct: it accumulates per row key in memory — keyed on the iteration, the
stage-or-role label and the model, joined by a control character rather than a space — apportions
first and folds second, then walks the accumulated keys emitting **exactly one row each**. The stage
pass takes that shape.

**Three properties of the fan-out fold are what make it correct, and all three transfer.** The
accumulation happens wholly before any write, so the writer is invoked once per row key and its
replace can never annihilate a sibling. The key is a tuple joined by a separator that cannot occur
in a field, so no two distinct keys can alias. And the discriminator column degrades honestly when
the fold is non-trivial, which delta 2 carries over.

**The replace-on-match is kept exactly as it is, and that is the constraint a naive fix breaks.**
The collector re-derives *every* historical row on every invocation — it unions the state file's
whole committed history with its live content, unbounded, with no watermark or cursor — and the
dedup key is the only thing making that union safe. §The trend log says so in terms: the key "is
also what makes the history ∪ live read safe with no added mechanism: a history arm re-derives rows
already logged, and re-derivation replaces a triple's line rather than double-counting it."
Converting the write to an accumulate is the naive sum the host entry already rules out, and it
would duplicate every historical row on every run. **The fold moves the summation earlier; it does
not move it into the log.**

### (2) The row's stdout discriminator degrades to a session count when the fold is non-trivial

The stdout report keeps one line per emitted row and its `who` column becomes `"<n> sessions"`
wherever a row key drew from more than one session, exactly as the fan-out row's already becomes
`"<n> anchors"` {design-bearing}. Where a key drew from one session the column is unchanged and
carries that session's id.

**This is the fold's honesty half and it is not optional.** §The fan-out row's own sentence — "The
stdout caveat names the contributing anchor count where it exceeds one, so a folded row is never
read as one session's" — is the reason the fan-out fold is legible rather than merely correct, and a
stage fold without it would replace a silent loss with a silent sum. The `/economics` narrative is
where that caveat is read, and the template already instructs its reader to repeat a run's caveats
so the reader can discount a row.

**No log field is added, and the no-field rule is what forbids it.** The count lives on stdout at
measurement time, beside the apportionment key and the collision notices, on the section's standing
rule that "a log field with no reader is a field removed". A persisted session count would have no
reader: the narrative reads stdout, the operator reads `cost`, no gate reads the log at all, and the
one prose reader that parses the stage column reads the column and not a count.

### (3) The per-session detail stays on stdout, and losing it from the log is the priced cost

After the fold, the log answers "what did this stage cost" and no longer answers "what did each
session of it cost" — which is a real loss and is the one the host entry names for this candidate
{design-bearing}. It is priced and accepted rather than waved past, on two grounds.

**The per-session detail is not lost, it is relocated to where it already is and already correct.**
The stdout report emits one line per session before the fold, with the session's own id, and no
replace ever touched it — which is why the defect survived from its filing to now: the one named
narrative reader reads stdout and has always seen the truth.

**And the log's own stated purpose is the trend, not the audit.** Its readers are a close-over-close
cost comparison, a deferred measurement rung that consumes rather than rebuilds it, and the tiering
watch delta 5 names. Every one of them reads a per-stage series. A per-session log would serve none
of them better and would break all of them the same way delta 4 refuses.

### (4) The two refused candidates, refused on their corrected premises

**Widening the key with the session id is refused** {design-bearing}. On the corrected premise above
it is a grammar change, not a re-keying: the published grammar block gains a column, every
downstream surface quoting that block moves, and five of the six fixture sets carry anchored or
space-delimited row assertions that break. It contradicts §The trend log's own statement that the
dedup key **is** the triple and §The fan-out row's that the key "stays the `<iteration> <stage>
<model>` triple". It fails the no-field-without-a-reader rule, no reader existing for the new column.
And it raises two questions the entry never reached and cannot answer cleanly: what session id a
**folded fan-out** row carries, where the discriminator is already a count and not an id, and what a
**supervision** row carries, where it is a lead's id apportioned across several iterations. Either
the column is nullable — a sentinel with no reader — or the fan-out fold is undone, which
contradicts the ruling this amendment is standing on.

**Making the row an explicit aggregate the writer recomputes is refused as either under-determined
or already delta 1** {design-bearing}. The entry's phrasing admits two readings and the corpus
settles neither. Read as "same row, different code path", it *is* delta 1 with more machinery and no
difference in the result. Read as "the row carries an explicit aggregate marker", it inherits the
grammar cost delta 4's first half just refused. **A third fact decides it against both readings**:
"every matching transcript" can only mean every *stamped* transcript, because stage membership is
known only through the stamp, and the transcript finder resolves at most one file per short session
id, silently preferring the newest on a collision. So the recompute buys no completeness delta 1
does not already have, and on a short-id collision it would report an aggregate that is silently
missing a member — a worse failure than today's, which is at least symmetric.

### (5) §The stage-economics meter's reader roster gains its fourth reader, which parses the column

The section's blast-radius bullet enumerates three readers and concludes that no reader parses the
stage column: this meter's own dedup, the harness's line-count assertion, and the `/economics`
narrative, which "reads it as prose rather than parsing the stage column" {design-bearing}. That
roster is stale. A **fourth** reader exists and it does parse the column: the consumer's lead
binding judges an align-tier decision on the cache-read field of the **bare `align` rows**,
explicitly never mixed with the `align+fanout` family. It is prose-tier and hand-read, which is why
the bullet's carve-out for the narrative does not cover it — the narrative does not discriminate on
the column and this reader does.

**Naming it is this delta's whole content, because the fold does not break it and the near-miss is
the point.** The stage token is unchanged, the bare-versus-suffixed discrimination is unchanged, and
one row per `(iteration, align, model)` still holds — so the reader's contract survives untouched.
What moves is its **conclusions**: a split stage's cache-read figure rises to its true value, so a
tier judged cheap on a truncated series may not be. That belongs on the entry that owns the tiering
judgment, not in this section, and delta 7 puts it there.

**The roster's own conclusion is corrected rather than merely extended.** "No reader parses the
stage column" becomes "no reader in *production code* parses it, and the one prose reader that does
is named" — because the original sentence is what would let a later widening be cleared by
inspection.

### (6) §The join's per-session-is-per-stage claim is struck, being the premise that hid all this

§The join states that because this repo runs one session per stage under the session-boundary
posture, "a session maps to exactly one stage, so per-session usage *is* per-stage usage", and
contemplates only the inverse failure — one session spanning several stages, which it handles
{design-bearing}. The claim is a bijection resting on an injection: the boundary posture bounds a
session to at most one stage; it does not bound a stage to at most one session, and the same SPEC
says elsewhere that a batch split stamps once per session.

**The sentence is replaced rather than qualified.** What survives is the true half — a session maps
to at most one stage, so a session's usage is attributable without apportionment — plus the
statement the fold now makes safe: a stage may hold several sessions, and their usage sums into the
stage's row. The join itself is unchanged: it still keys on the session and not the stamp, which is
the over-count defect that rule exists to forbid, and the fold happens after attribution rather than
instead of it.

### (7) The fixture that would witness this is one field away from existing, and gains it

The fan-out fixture set already stands up a **second same-stage session** — a second transcript
stamped into the same `(iteration, build)` pair — and deliberately authors it with **no assistant
usage**, so it exercises the anchor walk without emitting a second stage row {design-bearing}. That
is exactly why the collision has never reddened a fixture: give that transcript usage and the
existing assertion that the stage row carries the first session's draw alone reds today.

**So the regression witness is: give it usage, and re-point the assertion at the sum.** The stage
row's expected token figures become the sum over both sessions, and a new assertion requires the
stdout report to name the contributing session count. No new fixture set is minted and no other
set's assertions move — the flat set's whole-file line count and its re-measure idempotence check
are untouched, both being about duplication rather than loss, and both still hold.

**State why the existing assertion was blind, because it is the reusable half.** The flat set's
`-eq 1` is a **file-global** line count, monotone in duplication and completely blind to
replacement: a violation that *replaces* rather than duplicates leaves the count at one and the
assertion green. That is the structural reason this survived six fixture sets and a full port parity
run, and it is why the new assertion asserts a **value** rather than a count.

## Producers and consumers

**No new state, event or interface is introduced.** The row grammar, the dedup key, the knob roster,
the arm's family and its exit contract are all unchanged. What changes is the order of two existing
operations — sum, then write — and one existing stdout column's degradation rule. The
causal-completeness points are answered against the existing objects for that reason.

**The changed producer: the stage rows.** *Producer* — the arm's stage pass, at the transition where
it has resolved each stamped session's transcript and priced its usage; it now writes once per
`(iteration, stage, model)` key instead of once per session. Its enabling configuration is
unchanged and is actually emitted: the arm resolves its seven knobs through the bridged-arm table on
every front-end invocation, and this repo's config seam sets the metric dir the log lives under.
*Consumers* — unchanged in identity and in mechanism: the meter's own replace-on-append reading the
log back on the next run; the harness's install smoke asserting over its fixture logs; the
`/economics` narrative reading the arm's **stdout**; the operator reading `cost` close over close;
the lead binding reading the cache-read column of bare stage rows; and the deferred measurement rung
that consumes this log rather than rebuilding it.

**The changed field: the stdout report's discriminator column.** *Producer* — the same emission, at
the same transition. *Reader* — the `/economics` narrative, at the step where the template instructs
it to repeat a run's caveats so a reader can discount a row; and a session reading the report
directly. It is the only column whose meaning moves, and its new value is a count where the fold was
non-trivial and the existing session id otherwise.

**The reader whose verdict moves without its contract moving: the lead binding.** *Mechanism* — a
hand read of the cache-read field over bare stage rows. *Transition* — the tier judgment recorded on
the consumer's own tiering entry. Named here because "the reader is unbroken" and "the reader's
answer is unchanged" are different claims, and only the first is true.

**Narrowing check.** This delta set narrows no corpus. It removes no file, no field, no accepted
invocation and no fixture; the one thing that shrinks is the **number of rows** a split stage
persists, which was already one and stays one — what changes is that the one row is now the sum
rather than an arbitrary member. Point 5's monotonicity question therefore does not arise, and this
paragraph says so rather than leaving the absence to be read as an omission. The readers that *do*
red are named with their conditions: the harness's fan-out assertions red on a **value mismatch**
against expected token figures, which delta 7 moves deliberately; the flat set's line assertions red
on an **exact count**, which is unmoved; and `check-gate-binary-fresh` reds on a **stamp mismatch**,
so the crate source is staged before the binary is rebuilt.

## Existing sections updated

- **drift-kit/SPEC.md §The stage-economics meter — the trend log** — the fold: one row per
  `(iteration, stage, model)` is now a *sum over that key's sessions* rather than an assertion that
  only one session contributes, with the replace-on-append and its history-union ground restated as
  the thing that did not change (deltas 1 and 3).
- **drift-kit/SPEC.md §The stage-economics meter — the join** — the per-session-is-per-stage
  sentence, struck and replaced by the attributable-without-apportionment half plus the several-
  sessions-per-stage fact (delta 6).
- **drift-kit/SPEC.md §The stage-economics meter — the fan-out row's several-anchors bullet** — its
  fold generalizes from one row family to both, so the paragraph stops being the fan-out row's
  private rule and becomes the meter's, with the fan-out apportionment staying the fan-out's own
  (deltas 1 and 2).
- **drift-kit/SPEC.md §The stage-economics meter — the blast-radius bullet** — the reader roster
  gains its fourth member and its "no reader parses the stage column" conclusion is corrected to
  production code (delta 5).
- **drift-kit/SPEC.md §The stage-economics meter — the stdout-caveat prose** — the session-count
  degradation joins the anchor-count one as a second instance of the same rule (delta 2).
- **drift-kit/SPEC.md §Testing** — the fan-out fixture set's second same-stage session gains usage,
  and the section states why the flat set's line count could never have caught this (delta 7).
- **`drift-kit/smoke/install.sh`** — the fan-out fixture's transcript, the stage row's expected
  figures, and the new stdout session-count assertion (delta 7).
- **`native/src/emit/stage_economics.rs`** — the stage pass's accumulation and single emission, and
  the discriminator's degradation (deltas 1 and 2).
- <!-- update-target-exempt: a consumer's own binding shim, outside every kit's authorship and held by no delta; the fold leaves its contract intact and delta 5 states the one thing that moves, which is a conclusion rather than a surface --> the consumer lead binding's align-tier paragraph.
- <!-- update-target-exempt: generated projections with their own freshness gates and regen commands, rostered in docs/site-architecture.md §Generated projections and their freshness gates --> the on-site SPEC mirror.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named, reachable producer and
      a named consumer; every new field has a named reader at a named transition. (This amendment
      introduces none, and says so in §Producers and consumers rather than leaving it blank.)
- [ ] **The witness reds before it passes** — the fan-out fixture's second same-stage session is
      given usage and the suite is run **before** the fold lands, confirming the existing stage-row
      assertion reds; a fixture that never reddened is not a witness.
- [ ] **The history union is re-proved, not assumed** — the flat set's re-measure idempotence check
      is run after the fold, and a second consecutive run leaves the log byte-identical.
- [ ] **The prose reader is checked with the gate, not by reading** — the consumer shim's
      restatement gate is run rather than emulated, the new SPEC prose being about the same subject
      as a passage in a binding shim.
- [ ] **Merged with no information lost** — each addition integrated into its proper canonical-spec
      section (not appended); the merged spec reads as one coherent document a reader who never saw
      the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component
      (`ls drift-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec, template, doc and crate source for the struck
      per-session-is-per-stage claim and for the three-reader roster; nothing dangles.
- [ ] **The new crate source is staged before `bash gate-sdk/bin/build-native.sh` runs**, the tree
      `check-gate-binary-fresh` compares being `git ls-files`-derived.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a
      build-time causal gap is resolved that session, not deferred).
