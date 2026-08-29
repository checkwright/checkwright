# SPEC amendment: survey-record inbound-edge sum

Rules `survey-edge-aggregation-residue`. The lead ruled the surviving alternative
on 2026-08-29: **fold the inbound-edge sum into the survey record as an obliged
field**, the field carrying the sum rather than an attestation that it was taken.
The entry's six dated observations, its alternation evidence and its narrowed
fork are not re-derived here — they are settled, and this amendment builds.

**The corrected carry, restated because it is what the design has to close.**
The entry's own cost line was corrected at that boundary: it is *not* that a
close restamps an audit it never performed. It is that **the sum is taken every
time and recorded nowhere a later stage can read it**, so each boundary re-buys
an aggregation the last one already paid for.

**Probed rather than assumed, and it moved the design.** This iteration's scope
*did* record its sums — inside `finding:`, as one clause of a roughly
seven-hundred-word prose line. So the recording is not absent; it is
**unaddressable**. Nothing asserts it is there, nothing finds it without reading
the whole blob, and the next boundary cannot tell "no sum was taken" from "the
sum is in there somewhere". That is the gap a named, gated field closes and a
prose convention does not.

**Also probed, and it removes a delta this amendment would otherwise have
carried.** The entry names a `survey-engagement` **audit class** as the field's
downstream reader. `grep -rn "survey-engagement"` over the tree hits
`TASK-QUEUE.md` and nothing else — no roster, no stage template, no gate, no
evidence suite carries the name, which the queue's own
`survey-engagement-residue-untracked` states independently ("nothing OBLIGES a
survey or its lead to leave durable engagement residue, so the
`survey-engagement` audit passes or fails on practice"). **There is no roster
line to update**, and a delta claiming to update one would have reached build as
an instruction to edit a file that does not exist. The field's readers are the
three named in §Producers and consumers, all of which exist today.

## What changes

### (1) A fifth block field, `edges:`, in the survey-record grammar

lifecycle-kit/SPEC.md §The survey record's block grammar takes a fifth key
between `rev` and `finding`; the field's obligation rule and its legal-value set
are the whole ruling, and both are argued below rather than picked.
{design-bearing}

```
## <YYYY-MM-DD> <stage> — <the one-line question this survey answered>
- corpus: <git pathspec the survey covered>
- oracle: <the command whose verdict grounds it, or the literal `none`>
- rev: <full commit sha the survey was taken at>
- edges: <the inbound-citation sum per candidate this survey ranked, or the
  literal `none` when it ranked none>
- finding: <the judgment, in prose>
```

**Position four, and it is not arbitrary.** `corpus`, `oracle` and `rev` are the
*witness* — the three strings the two-command re-use protocol consumes — and they
stay contiguous. `edges` and `finding` are the *judgment half*: an input to a
ranking and the ranking's reading. Appending `edges` after `finding` would put
the field a reader wants at a glance behind the longest line in the block.

**Obliged on every block, with `none` legal — and this is where the ruling's own
tension is resolved.** The ruling obliges the field, and separately preserves the
ceremony objection against a *declaration*. A field present only on ranking
blocks satisfies the second and abandons the first: an optional key reintroduces
exactly the "absent, or taken and dropped?" ambiguity the entry is about, and
`check-survey-record`'s grammar is rigidly positional precisely so that question
never has to be asked. A field obliging every block to write
`edges: n/a — not a ranking survey` satisfies the first and violates the second:
that string is a declaration and nothing else.

The close is to **reuse `oracle:`'s shipped precedent exactly**, argued and gated
in this same section already: the literal `none` is the honest form, an *empty*
value is the silent form and is refused. `edges: none` is one word, asserts
nothing about a pass having run, and is true of every survey that ranked no
candidates. The grammar stays rigid, the obligation stays mechanized, and no
block carries ceremony. Nothing new is minted — the second reader of a
convention this surface already ships.

**What the value carries when it is not `none`:** one inbound-citation sum per
candidate the survey ranked, in whatever spelling the author finds honest — the
form scope already writes, `<slug> <n>`, comma-separated, with any caveat the sum
needs. Free prose, deliberately.

### (2) `bin/file-survey.sh` takes a fifth positional

The affordance grows one slot, in field order: `[--] "<question>" "<corpus>"
"<oracle>" "<edges>" "<finding>"` — the arity refusal moves from four to five, the
emitted `printf` gains one `- edges: %s` line in position four, and the existing
every-positional flag-shape scan (gate-sdk/SPEC.md §The `bin/`-tool contract)
widens to the new slot by construction because it already iterates `"$@"`.
{mechanical} The usage line and the file's header comment move with it.

The tool does **not** gain a default. An omitted fifth argument is the four-arity
misuse the tool already refuses, which is the behavior wanted: a session that
forgot the field is told at filing time rather than at commit time.

### (3) `bin/cite-survey.sh` emits five fields

The citation affordance writes "the heading and all four fields"; it writes all
five, the block selector unchanged and the emitter's field set the same array the
block grammar defines. {mechanical}

### (4) `check-survey-record` asserts the fifth key, with its fixture pair and hermetic arms

The gate's block assertion becomes **five keys present, in order, one per line,
no sixth key and no stray line**; `edges` non-empty on the same footing as
`corpus` and `oracle`, with `none` legal and empty refused. The widened
git-object-token probe (§check-survey-record) covers `edges` too — it already
covers "the other three fields", and the phrase becomes "the other four", so a
fabricated sha pasted into a sum is caught by the arm that exists rather than by
a new one. The red condition, the `none`-legal arm and the fixture pair's
calibration are judgment, and the `bad/` fixture has to distinguish an *empty*
`edges` from a *missing* one, which are different findings with different
remedies. {design-bearing}

Substrate: the gate is ported, so this lands in `native/src/gates/survey_record.rs`
— the key array at its head, the fifth-key refusal message that names the grammar,
and the empty-value arm — under gate-sdk/SPEC.md §Porting a gate to the binary
substrate's rules for editing a ported gate. The `good/`+`bad/` fixture pair and
`gate-tests/check-survey-record.test.sh` move with it: the good record gains the
field on all three of its blocks including the `oracle: none` note and the valved
block, and the bad record gains an empty `edges` and a block whose `edges` line is
missing, beside the shapes it already carries.

### (5) The live record's existing blocks gain the field in the gate's own commit

This is a hard build-time coupling rather than tidying: the gate's red condition
is "not exactly five keys", so the commit that ships delta 4 reds
`.workflow/survey-record.md` unless the same commit migrates it. {mechanical} The record
is boundary-truncated, so this is a one-time cost that never recurs — but it is
owed *now*, and a batch that lands delta 4 alone cannot commit.

The migration is not uniform and must not be done by a blanket `none`: this
iteration's scope ranking block **has** its sums, inside `finding:`, and they lift
into the field. The other blocks ranked no candidates and take `none`.

### (6) scope's stage template names the field as where the sum lands

`lifecycle-kit/templates/stages/scope.md`'s aggregation paragraph tells a session
to aggregate a candidate's inbound edges before ranking it. It gains the
destination: the sum lands in the survey record's `edges` field, which is what
makes it readable at the next boundary instead of dying with the session. The
sentence has to add a destination without re-stating the grammar the SPEC owns
(content-tiering), and it is the surface where a scope session actually meets the
obligation. {design-bearing}

## Producers and consumers

**The new state is one field, `edges`, on the survey-record block.**

- **Producer, named and reachable:** `lifecycle-kit/bin/file-survey.sh` (delta 2),
  invoked by any stage session, plus the raw markdown append this surface keeps
  as a sanctioned fallback (§The survey record). The producer needs no enabling
  config: the record's path is `LIFECYCLE_KIT_SURVEY_RECORD_FILE`, already
  defaulted and already emitted by this repo's `scripts/lifecycle-config.sh`,
  and the field rides the same file. There is no configuration under which the
  producer exists and the field does not.

- **Consumer 1 — `check-survey-record`, at commit time** (delta 4), by the
  pre-commit hook and the battery. Mechanism: it parses each `## ` block and
  asserts the key set. This is the consumer that makes the field *obliged* rather
  than conventional, and it is the entry's whole ask.

- **Consumer 2 — the next boundary's scope session, at the ranking transition**
  (delta 6). Mechanism: it reads the record before buying a survey (the witness
  protocol, §The survey record) and reads `edges` to decide whether the
  aggregation still has to be bought. This is the reader the corrected cost line
  names, and the only one that recovers the re-bought aggregation.

- **Consumer 3 — `lifecycle-kit/bin/cite-survey.sh`, at the inlining transition**
  (delta 3). Mechanism: it selects one block by heading substring and writes every
  field to stdout as an inline-ready snippet. A field the citer dropped would be
  a field that never reaches the permanent surface a finding is inlined onto,
  which is the transition §The survey record calls the point of the tool.

**Every field has a named reader** — there is exactly one new field and it has the
three readers above, each at a named transition. No other field is added, and no
existing field's population changes.

**Red conditions, named rather than subjects.** Delta 4 **widens** a corpus (a
fourth key becomes a fifth), which is the safe direction; the check's point 5 is
still run, because the delta *also* changes what an existing reader reds on:

- `check-survey-record` — reds on a block whose key set is not exactly the five,
  in order; on an empty `corpus`, `oracle` or `edges`; on a `rev` that is not a
  full 40-hex sha naming a commit; and on a git-object-shaped token in any of the
  four non-`rev` fields naming no object. **Non-monotone by construction** — it
  reds on a key being *absent*, so widening the required set turns every existing
  clean block red. That is delta 5, and it is the reason delta 5 exists rather
  than a courtesy.
- `check-scratch-citation` — reds on a retrieval pointer, from a permanent
  surface, to a boundary-truncated path. Unchanged: this amendment adds no
  pointer and moves no path into or out of `lifecycle_supersede_set`. Monotone in
  its violation set and clear by inspection.
- `check-docs-mirror-fresh` — reds on a byte difference between a kit SPEC and its
  `docs/` mirror. Fires on all deltas that touch `lifecycle-kit/SPEC.md`, and
  clears on regeneration; see below.
- `check-comment-tier` — reds on a non-directive comment. Deltas 2 and 3 edit
  shell whose `spec:` lines are one-line bindings; a moved usage comment stays a
  directive.

## Existing sections updated

- lifecycle-kit/SPEC.md §The survey record — the fenced block grammar, the
  sentence reading "Four fields, and each earns its place by being read at a named
  transition" (it becomes five, and the new field's transition is named there),
  and the `bin/file-survey.sh` affordance paragraph's signature and arity
  sentence (deltas 1, 2). The `cite-survey.sh` paragraph's "all four fields"
  (delta 3). *Not* updated: "No field for how long this stays true" — `edges` is
  not a staleness field and that ruling is untouched.
- lifecycle-kit/SPEC.md §check-survey-record — the "all four keys present, in
  order … and no fifth key" invariant sentence, the widened-arm paragraph's "the
  other three fields", and the fixture-pair paragraph's enumeration of what each
  side carries (delta 4).
- lifecycle-kit/SPEC.md §Layout and configuration — the
  `LIFECYCLE_KIT_SURVEY_RECORD_FILE` knob paragraph describes what
  `bin/file-survey.sh` writes; the field count moves with it (delta 1).
- lifecycle-kit/templates/stages/scope.md — the inbound-edge aggregation
  paragraph gains the field as the destination (delta 6).
- CLAUDE.md §Housekeeping — the **Survey capture** bullet spells the tool's
  signature literally, and it is a four-argument spelling that becomes wrong the
  moment delta 2 lands (deltas 1, 2).
- `docs/lifecycle-kit/SPEC.md` and `docs/lifecycle-kit/README.md` — generated
  mirrors of the two surfaces above, stale the moment any delta lands
  (`all deltas`); regenerated by the command `check-docs-mirror-fresh` prints on
  red, and rostered with its trigger in docs/site-architecture.md §Generated
  projections.

## Definition of Done

- [ ] **Causal completeness** — the one new field has a named, reachable producer
      (`bin/file-survey.sh`) and three named consumers, each at a named
      transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper section of lifecycle-kit/SPEC.md, not appended; the `none`-legal
      argument lands beside `oracle:`'s, which is the precedent it reuses.
- [ ] **Amendment deleted** — this file removed on merge; none remain for
      lifecycle-kit (`ls lifecycle-kit/SPEC-*.md`), discharged at the iteration
      rather than at the commit where a sibling is in flight.
- [ ] **Removals propagated** — grepped every spec and every generated mirror for
      the four-field spelling (`"all four fields"`, `"no fifth key"`, the
      four-argument usage line); nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed to
      the gap inbox; a build-time causal gap is resolved that session.
