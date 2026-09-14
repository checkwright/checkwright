# SPEC amendment: lead-journal-open

Queue entry: `lead-journal-advisory-fires-on-the-live-lead`. It is ranked into
`config-bridge-floor` at the recurrence threshold under **operator direction, 2026-09-14,
lead-relayed**. Nothing in this amendment carries a further direction: the choice among the
entry's candidate discriminators is this stage's, and the grounds are below.

## The defect, re-verified at this stage

The boundary entry's undisposed-journal advisory (lifecycle-kit/SPEC.md §bin/enter-stage.sh,
*Preserving is not draining*) tests exactly one thing: whether the lead journal's last non-empty
line is `DISPOSED` (`native/src/emit/enter_stage.rs`, `disposed`). A split-posture lead opens its
journal — posture, grant, dispatch log — before it dispatches scope, so at every lead-dispatched
boundary the file is the **live** lead's and never ends in the mark. The advisory prints it as
*the prior lead's journal*, at the one moment the entering session reads the report. It fired at
this iteration's own scope entry as well, which is the second recurrence date on the entry.

It is worse than noise. The journal is one basename, so a live lead opening its journal at a
path an undisposed prior journal still holds either overwrites it, losing the findings before
any advisory runs, or appends to it, after which the last line is the live lead's and the prior
content can never be told apart. **Under a lead, the advisory cannot report the case it exists
for.**

## The discriminator ruling

The entry named three candidates. Each is refused on a ground probed at this stage, and the
chosen shape is a variant of the first.

- **The lead writes its journal only after scope stamps — refused.** A grant lives in the lead's
  resume journal (lifecycle-kit/SPEC.md §The steering vocabulary). The lead holds the grant
  before it dispatches scope, so deferring the write leaves the grant with no durable home for
  the whole scope stage.
- **A session marker — refused.** The lead already writes `lead <id>` to context-kit's
  session-role marker, and the marker survives the boundary. The marker's staleness test is an
  id match against *the reading session's own* id, which works for the lead's own hook and for
  nobody else. At the boundary the reader is the scope session, so a marker left by a lead that
  ended without disposing reads exactly like a live one. That fails open on the case the advisory
  exists for.
- **An opening line naming the iteration — refused as stated.** The boundary reset writes the
  unnamed placeholder, so a lead opening before scope sees either the prior iteration's name or
  the placeholder. Two consecutive iterations that never got a name collide on the placeholder.
- **Chosen: an opening heading keyed on the stage cursor.** The cursor is the last stamp in the
  state file (lifecycle-kit/SPEC.md §The state machine). A journal opened after that stamp was
  opened for the iteration about to open, and the stamp is unique by construction, since it
  carries a session id and a head. The key is **fields 2 through the last** of the stamp line,
  never the whole line, because `--rename` rewrites column 1 of every stamp. That is the same
  witness `--rename` already asserts (`fields_two_to_last`).

**Why the heading is written by the entry tool and not by hand.** A hand-written key means the
lead reading the state file and copying a line out of it. The lead template forbids that read
(*The lead never hand-derives prior-stage completeness*), and a copied key drifts by a single
space. The writer and the reader of the key must agree byte for byte, which is the ground the
stage journal's `JOURNAL_MARK` is spelled once for. So the opener is an operand of the tool that
already owns the lead journal's knob, its disposition mark and the advisory. It is not a new
arm: a new arm would be a second module holding one key derivation, and it would need its own
bridged roster.

## What changes

### (1) `--enter-stage [--simulate] --open-lead-journal` — the opener

lifecycle-kit/SPEC.md §bin/enter-stage.sh gains a third grammar form beside `<stage>` and
`--rename <name>`, the lead's journal opener, and the paragraph below lands after the `--rename`
paragraphs {design-bearing}. **Not yet applied:**

> **`--open-lead-journal` — the lead's journal opener.** `[--simulate] --open-lead-journal`
> appends one heading to the lead journal (`<scratch>/$LIFECYCLE_KIT_LEAD_JOURNAL_FILE`),
> creating the file and its directory when absent:
>
> `## lead-journal opened after <key>`
>
> `<key>` is fields 2 through the last of the state file's last stamp, or the literal `none` when
> the file carries no stamp. The heading opens a **segment**, which runs to the next such heading
> or to the end of the file. Content before the first heading is a segment as well, so a journal
> written before this opener existed reads as one segment with no key.
>
> - **Append, never overwrite**, on the stage-journal opener's precedent. An undisposed prior
>   segment is the thing the boundary advisory reports, and an overwrite would delete it before
>   any reader ran.
> - **Disposed segments are dropped.** Before appending, the opener removes every segment whose
>   last non-empty line is `DISPOSED`. It keeps every other segment, in order. A disposed segment
>   has been discharged by the lead that wrote it, so no reader remains for it. Dropping it keeps
>   the protected file from becoming the accumulator the invariant warns about.
> - **Idempotent.** When the file's last segment already opens with the heading for the current
>   key, the opener reports and exits 0 without writing. That covers a resumed lead re-running
>   its first step. **Honest limit:** a second lead opening at the same cursor, after the first
>   ended without disposing, continues the first lead's segment. The opener cannot tell a resumed
>   lead from a replacing one. Either way the segment belongs to the iteration about to open, and
>   that is the property the advisory reads.
> - **Not stage motion.** No stamp is appended, no pre-flight runs, the queue is untouched, and
>   the command writes nothing tracked. `--simulate --open-lead-journal` relays what it would
>   write and drop, prefixed `enter-stage (simulate):`, and writes nothing.
> - **Exit contract:** 0 opened, or a reported no-op; 2 a usage or configuration error. Surplus
>   arguments after the operand are refused by this tool's existing surplus rule. It has no 1,
>   because it refuses nothing a caller could clear.

The grammar sentence at the head of the section becomes
`--enter-stage [--simulate] <stage>`, `--enter-stage [--simulate] --rename <name>` or
`--enter-stage [--simulate] --open-lead-journal`. **Not yet applied.**

The key derivation lives in one function that the opener and the advisory (delta 2) both call.
The heading lead is one constant beside `DISPOSITION_MARK`, spelled once for the reason
`JOURNAL_MARK` is.

### (2) The boundary advisory reads segments, and exempts only the live one

The *Preserving is not draining* paragraph of lifecycle-kit/SPEC.md §bin/enter-stage.sh changes
its test from the file's last line to per-segment disposition, with a live-segment exemption
{design-bearing}. **Not yet applied** — the paragraph's second and third sentences become:

> When the lead journal exists, the boundary entry splits it into segments at its
> `## lead-journal opened after <key>` headings (§`--open-lead-journal`) and compares each key
> with the **boundary key**: fields 2 through the last of the state file's last stamp *before
> this entry wrote*, or `none`. The **last** segment is **live** when its heading's key equals
> the boundary key. That segment was opened after the stamp this boundary closes over, so it
> belongs to the lead dispatching this very entry. Every other segment is **prior**. Each prior
> segment whose last non-empty line is not `DISPOSED` is reported: the advisory prints that
> segment's `## ` headings, its own opening heading first, and **proceeds**. A journal with no
> opening heading is a single prior segment, which is exactly the old whole-file test.

The boundary key is read before the reset truncates the state file, because the truncate removes
the stamp that the key names. The rest of the paragraph stands: the refusal is declined, and the
honest limit remains. The limit gains one sentence. **Not yet applied:**

> The live exemption trusts the heading. A lead that opens its journal by hand, or not at all,
> gets the old whole-file test and the false advisory with it, and a hand-written heading with
> a wrong key reads as prior. That is the safe direction.

### (3) The lead template opens its journal through the opener

`lifecycle-kit/templates/lead.md` changes in three places, all instruction only
{mechanical}. **Not yet applied:**

- **First step** — after the session-role marker block, one more command and one sentence:
  `bash gate-sdk/bin/run-gates.sh --enter-stage --open-lead-journal` — "Open your resume journal
  with it before you write anything to it, and append under the heading it writes; never
  overwrite the file."
- **§Economics, *Write the lead journal at every stage completion*** — "append to the lead's
  **own** resume journal" gains "under the heading your first step opened".
- **§Closing an iteration, *Dispose of your journal before you report*** — unchanged in
  substance. `DISPOSED` stays the last line; the next opener drops the segment it closes.

The template states no grounds. The grounds are delta 2's and §templates/lead.md's.

### (4) §templates/lead.md stops saying the lead invokes no `--enter-stage`

lifecycle-kit/SPEC.md §templates/lead.md says the template "invokes no `--enter-stage`". That was
already imprecise, since the template runs `--enter-stage --simulate` before every dispatch, and
this amendment adds a second non-stamping form. The clause becomes "stamps nothing through
`--enter-stage` — it runs only the non-stamping `--simulate` and `--open-lead-journal` forms"
{mechanical}. The paragraph naming the lead's first step, *Both of the lead's scratch artifacts
outlive the iteration*, gains the opener beside the session-role marker. **Not yet applied.**

### (5) The front-end usage text and the release declaration

The `--enter-stage` paragraph of the usage text in `native/src/runner.rs` names
`--open-lead-journal` beside `--simulate` and `--rename` {mechanical}. A **Behavior changes**
bullet lands in `.workflow/release-declarations.md`, saying three things. `--enter-stage` gains
`--open-lead-journal`. The boundary's undisposed-journal advisory now exempts a journal the live
lead opened with it. A consumer that copied `templates/lead.md` out should re-take the first
step, or the advisory keeps firing on its live lead. **Not yet applied.**

### (6) Hermetic cases

`lifecycle-kit/gate-tests/boundary-scratch-wipe.test.sh` keeps its undisposed and disposed cases
and gains four more {mechanical}:

- a journal whose only segment is live stays silent at the boundary;
- a prior undisposed segment followed by a live one is reported, and only the prior segment's
  headings print;
- a segment whose key names an older stamp is reported;
- a heading-less journal behaves as before.

A new `lifecycle-kit/gate-tests/lead-journal-open.test.sh` covers the opener: creation, append
after an undisposed segment, dropping a disposed segment, the idempotent no-op,
`--simulate --open-lead-journal` writing nothing, the surplus refusal, and the `none` key on a
stampless state file. The unit tests in `enter_stage.rs` pin the key derivation against a
renamed state file. Advisory tooling like `--simulate`, so no fixture pair is owed, on
§bin/enter-stage.sh's `--rename` precedent.

## Producers and consumers

- **The opening heading** (new state). *Producer:* `--enter-stage --open-lead-journal`, run by
  the lead template's first step (delta 3). This repo reaches it through `.claude/commands/lead.md`,
  which binds the template. *Consumers:* the boundary advisory, in process, at the first-stage
  entry (delta 2), and the opener itself for idempotence and segment dropping (delta 1).
- **The heading's one field, `<key>`.** *Reader:* the advisory's live test at the boundary
  transition, and the opener's idempotence test at open. The heading's text is also a `## `
  heading, so the advisory prints it as the first line of a reported segment. That is how the
  entering session learns *which* lead's segment is undisposed. No lead id is carried: no reader
  would branch on it, which is the ground the disposition mark carries no author on.
- **The boundary key** (new internal value). *Producer:* the entry tool, read off the state file
  before the reset truncates it. *Consumer:* the advisory's live test only.
- **Segments** (a new reading of an existing file). *Producer:* the lead, by appending under a
  heading. *Consumers:* the advisory and the opener. Nothing else reads the lead journal. A
  whole-tree grep for `lead-journal` and `LEAD_JOURNAL` at this stage finds only the knob's
  definition and validation (`lifecycle-kit/lib/stages.sh`), the boundary wipe's keep entry
  and the advisory (`enter_stage.rs`), the wipe fixture, and prose.
- **Red conditions (point 5).** The advisory narrows what it reports, since a live segment
  leaves the set. It is an advisory, exit 0 either way, so no verdict flips. The fixture
  `boundary-scratch-wipe.test.sh` reds on the *absence* of the advisory for its undisposed case,
  which is not monotone. That case writes a heading-less journal, which delta 2 keeps prior, so
  it stays red-on-regression. Delta 6 adds the live case beside it rather than editing it.
  `check-stage-skill-coverage` reads only the skills dir, and captures only a lowercase token
  after `--enter-stage`, so `--open-lead-journal` is never read as a stage name.
- **Existing integration prose.** The boundary report's *prior lead's journal* wording becomes
  true, because it now prints only for prior segments. The *Preserving is not draining*
  paragraph, §templates/lead.md and the template are updated here (deltas 2 to 4).

## Existing sections updated

- `lifecycle-kit/SPEC.md` — §bin/enter-stage.sh: the grammar sentence and the new
  `--open-lead-journal` paragraph (delta 1); the *Preserving is not draining* paragraph and its
  honest limit (delta 2).
- `lifecycle-kit/SPEC.md` — §templates/lead.md: the *invokes no `--enter-stage`* clause and the
  scratch-artifacts paragraph (delta 4).
- `lifecycle-kit/templates/lead.md` — first step, §Economics journal bullet, §Closing (delta 3).
- `native/src/emit/enter_stage.rs` — the operand dispatch, the opener, the shared key function,
  the heading constant, the segment-reading advisory and its unit tests (deltas 1, 2 and 6).
- `native/src/runner.rs` — the usage text's `--enter-stage` paragraph (delta 5).
- `lifecycle-kit/gate-tests/boundary-scratch-wipe.test.sh` — the live and prior-segment cases
  (delta 6).
- `lifecycle-kit/gate-tests/lead-journal-open.test.sh` — new (delta 6).
- `.workflow/release-declarations.md` — Behavior changes (delta 5).
- `docs/lifecycle-kit/SPEC.md` — generated mirror, regenerated (all deltas).

## Retired spellings

- None — no delta retires a name. The `invokes no --enter-stage` clause is reworded rather than
  retired, and the advisory's message keeps its wording.

## Definition of Done

- [ ] **Causal completeness** — the heading has a reachable producer in the template's first
      step and two named readers. The boundary key has one.
- [ ] **Instruction surfaces: instruction only** — the template's first step gains a command
      and one sentence, and no grounds.
- [ ] **Merged with no information lost** — the discriminator ruling's three refused candidates
      move into §bin/enter-stage.sh beside the advisory they bound.
- [ ] **Amendment deleted** — this file removed on merge, and no `lifecycle-kit/SPEC-*.md`
      remains at the iteration.
- [ ] **Removals propagated** — `check-amendment-retired-spelling` green.
- [ ] **Gaps filed** — none expected. A build-time causal gap is resolved that session.
