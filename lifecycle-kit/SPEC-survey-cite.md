# SPEC amendment: survey-cite

Closes `survey-citation-outlives-its-record`. A permanent surface that points a
reader into `.workflow/survey-record.md` writes a citation that dies on schedule:
the record is boundary-truncated by its own contract (§The survey record), so the
pointer resolves to nothing at the next `enter-stage.sh` boundary reset. Two
firings are attested, both found by a scope session hitting its own reset.

## The finding that decides it

The queue entry framed three closes as a trade: inline (cheap, **loses the
two-command witness**), commit-pin, or grow a promotion path. Probing the
mechanism collapses the trade.

**Commit-pinning is wrong by construction, not merely awkward.** `bin/file-survey.sh`
stamps `rev` as **HEAD at filing time**, which precedes the commit that lands the
block (§The survey record — "It stamps `rev` and the date itself"). So
`git show <rev>:<record>` reads a blob that **does not contain the block being
cited**. A hand-authored pin is therefore not a pin to a stale tree — it is a pin
to the wrong object, and it fails silently by printing a record without the block.
The correct sha is the *landing* commit, which no field carries and which is
exactly the class of value §The survey record already rules an author gets wrong.

**Inlining loses nothing.** The "two-command witness" is not a property of the
record file — it is the four fields `corpus`, `oracle`, `rev`, `finding`, and the
witness protocol runs `git diff <rev>..HEAD -- <corpus>` plus a re-run of
`<oracle>` **from HEAD**. Every input is a short string. Copied into the citing
surface, the witness is *more* durable than in the record, because the citing
surface is not truncated. The recorded trade rested on treating the witness as
something only the record could hold; it is four fields.

So the ruling is the cheap close, and the mechanism work is making it cheap to do
right and impossible to do wrong.

**Ruled out: growing the record a promotion path** (close three). It re-opens the
per-iteration lifetime the record's whole design turns on, and it buys nothing
inlining does not, now that the witness is known to be portable.

## What changes

**(1) The rule — a permanent surface carries the finding, never a pointer into
per-iteration scratch.** [design-bearing] Add to lifecycle-kit/SPEC.md §The
survey record: a surface outside the boundary-truncated set never carries a
**retrieval pointer** into one. It inlines the finding together with the block's
four witness fields, which is what keeps the finding re-usable rather than merely
readable. Naming the record as a *subject* ("the survey record is per-iteration
scratch") is unaffected — the rule is about promising a reader retrievable
content, and the distinction is what delta (3) is calibrated against.

**(2) `bin/cite-survey.sh <heading-substring>` — the affordance that makes
inlining one command.** [design-bearing] Reads
`LIFECYCLE_KIT_SURVEY_RECORD_FILE`, selects the one block whose `##` heading
contains the substring, and writes an inline-ready markdown snippet carrying the
heading and all four fields to stdout. Refuses (exit 2) on no match or on an
ambiguous match rather than guessing, and refuses on a record with no blocks. It
follows `bin/file-survey.sh` exactly — repo-root cd, config-via-env, exit 2 on a
missing or empty argument — and is **advisory tooling, not a gate**, the same
disposition `file-survey.sh` carries: the surface's contract is the grammar, not
the writer.

It deliberately does **not** rewrite the citing surface. The author chooses where
the finding belongs and how much of the `finding` prose to carry; a tool that
spliced would need a marker block, which would make a hand-written citation a
second-class form of the thing the rule is trying to make ordinary.

**(3) `check-scratch-citation` — the enforcement.** [design-bearing] A new
lifecycle-kit gate, `tier=precommit`, `couples=` the permanent-surface globs plus
`kit:lib/stages.sh`. It reds when a file in
`LIFECYCLE_KIT_PERMANENT_SURFACE_GLOBS` carries a **retrieval pointer** to a path
in the boundary-truncated set.

The forbidden-target set is **derived, never maintained**: it is
`lifecycle_supersede_set()` (lifecycle-kit/lib/stages.sh:95), already the single
derivation behind the installer's `.gitattributes` block and
`check-merge-attrs`'s parity check. This gate is its **third reader**, so a
consumer adding a `LIFECYCLE_KIT_BOUNDARY_TRUNCATE` member gets citation
enforcement over it with no second roster to update.

**Retrieval-pointer position, stated as the gate's red condition** — a
supersede-set path that is either (a) a markdown link target, `[...](<path>)`, or
(b) the final token of a line whose text before it ends in a colon (the attested
form: *"Full finding and its two-command witness: `.workflow/survey-record.md`"*).
A bare path elsewhere in prose is a mention and is clean. Per-line escape hatch:
a `scratch-citation-exempt:` tag on the preceding line, the repo's established
opt-out shape, for a surface that must quote a dead citation verbatim — which
this amendment's own queue entry does.

**The scan joins a bullet's wrapped continuation lines before testing form (b),
or it misses its own worked example.** `TASK-QUEUE.md` wraps prose to
`QUEUE_KIT_WRAP_BUDGET`, so a colon and the path it introduces routinely land on
different physical lines — the attested instance does exactly this: this
amendment's own queue entry (`survey-citation-outlives-its-record`) ends one line
in `witness:` and opens the next with `` `.workflow/survey-record.md` ``, so the
path's line has no colon-terminated text before it *on that line*. A scanner
reading physical lines in isolation would clear this entry with no tag needed,
contradicting delta (3)'s own claim that it is where the exempt tag first lands.
The join is not new design: `canon-kit/checks/check-spec-pointer.sh`'s
`PROSE_EXTRACT` already solves this for a citation-liveness scan — *"the
blank-line paragraph join"* (:69), which concatenates a paragraph's lines
(`flush()`, :72-84) before matching and maps a hit back to its physical line via
per-line start offsets. `check-scratch-citation` adopts the same join over each
bullet's lines rather than re-deciding the shape. The `bad/` fixture case must
include a wrapped instance (mirroring the live one) alongside the single-line
`prose-profile` and markdown-link forms — the calibration protocol's own rule,
"a regex that reds any of the four is wrong, and the fixture is what says so,"
applies equally to a scan that is right on an unwrapped fixture and silently
blind on the wrapped case it was written for.

**Calibration is the fixture pair's job, and both cases already exist in the
tree.** Four live queue entries name the record path descriptively and must stay
clean — `survey-record-supersede-invisible`,
`probe-before-assertion-doctrine`, `survey-record-extension-tier-hybrid`, and
`survey-record-claim-reliability`. They are cited by slug rather than by line
number deliberately: an amendment read at build outlives the line numbers, as
this iteration's own promotion proved by moving every one of them.

Copy their lines into the `good/` case; the `bad/` case carries the attested
`prose-profile` form and a markdown-link form. **A regex that reds any of the
four is wrong, and the fixture is what says so** — that is the whole calibration
protocol, and it is why delta (3) specifies a red condition rather than a regex.

The fifth occurrence is the interesting one: this amendment's **own queue entry**
quotes the dead citation verbatim in order to describe it. That is precisely the
case the `scratch-citation-exempt:` tag exists for, and the entry is the first
place it lands — so the escape hatch ships with a real user rather than a
hypothetical one.

**(4) The knob.** [mechanical] `LIFECYCLE_KIT_PERMANENT_SURFACE_GLOBS` — glob
array of surfaces held to the rule. Kit default is the queue file alone
(`LIFECYCLE_KIT_QUEUE_FILE`): the one permanent surface lifecycle-kit itself owns
and where both attested firings landed, so the default is non-vacuous in every
consumer and over-reaches in none. This repo's consumer config widens it to the
governed spec set. Config-via-env per the `<KIT>_<KNOB>` convention; the array
follows `QUEUE_KIT_PROSE_SURFACE_GLOBS`'s shape exactly.

**Provenance seam.** The gate ships mechanism only. The surface roster is
consumer config, and the forbidden-target set is derived from the consumer's own
truncate configuration — no kit literal names any surface this repo happens to
have.

**(5) Registration and fixtures.** [mechanical] Register `check-scratch-citation`
in `scripts/gates.list`; ship the `good/`+`bad/` pair delta (3) specifies; add the
gate to lifecycle-kit's README roster; regenerate the pre-commit hook, the graph
artifact, the enforcement map, the footprint/value rollup and the docs mirror per
docs/site-architecture.md §Generated projections.

## Producers and consumers

**New interface: `lifecycle_supersede_set()`'s third reader.**
- *Producer* — unchanged; the function already exists and is already populated by
  `enter-stage.sh`'s truncate loop (lifecycle-kit/bin/enter-stage.sh:178) reading
  the same three built-ins plus `LIFECYCLE_KIT_BOUNDARY_TRUNCATE`. Enabling
  config: none needed — the two kit-owned built-ins are unconditional, which is
  why the gate is non-vacuous in a consumer that sets no knob.
- *Consumer* — `check-scratch-citation`, by sourcing `kit:lib/stages.sh` exactly
  as `check-merge-attrs` does.

**New interface: `bin/cite-survey.sh` stdout snippet.**
- *Producer* — a human or agent session about to write a permanent surface that
  wants a carried finding; invoked by hand, and named in lifecycle-kit's stage
  templates at the point the witness protocol is already described.
- *Consumer* — the author, who pastes it. There is no machine consumer, and that
  is stated rather than left implicit: this is an emitter, so its output format is
  read by a person and `check-scratch-citation` never parses it.
- *Fields* — the snippet carries exactly the four record fields plus the heading.
  Named reader for each: `corpus` and `rev` are read by the witness's
  `git diff <rev>..HEAD -- <corpus>`; `oracle` by the witness's re-run; `finding`
  by the consuming session; the heading by a reader locating the superseding block
  if one was later filed. No fifth field is added, so the emitter cannot outgrow
  §check-survey-record's closed four-key grammar.

**New state: the `scratch-citation-exempt:` tag.**
- *Producer* — an author writing a surface that must quote a dead citation.
- *Consumer* — `check-scratch-citation`'s line scanner, at the transition where it
  has matched a retrieval pointer and is deciding whether to record a finding.

**Red conditions of the readers this change touches** (§The causal-completeness
check, point 5 — this delta *adds* a surface rather than narrowing one, but the
gate's own verdict is stated as its red condition rather than its subject):
- `check-scratch-citation` reds on **finding one or more** retrieval pointers —
  monotone in the violation set, so widening the glob array can only add
  violations.
- `check-gate-fixture-coverage` reds on a `gates.list` member with **neither** a
  fixture pair **nor** a `# no-fixture:` opt-out — a zero-count red, satisfied
  here by delta (5) shipping the pair.
- `check-graph`, `check-enforcement-fresh`, `check-value-rollup-fresh` red on a
  stale projection — a new gate stales all of them, which is why delta (5) names
  the regeneration rather than leaving it to a red run.
- `check-readme-roster` reds on a gate absent from its kit README in either
  direction.

## Existing sections updated

- **lifecycle-kit/SPEC.md §The survey record** — owned by delta (1). Gains the
  no-retrieval-pointer rule, the recorded refutation of commit-pinning (the `rev`
  / landing-commit gap), and a pointer to `bin/cite-survey.sh` beside the existing
  `bin/file-survey.sh` affordance paragraph. The existing paragraph "**The
  witness — the re-use protocol**" is where the portability of the four fields is
  made explicit, since that is the prose a reader consults before citing.
- **lifecycle-kit/SPEC.md §Layout and configuration** — owned by delta (4). Adds
  `LIFECYCLE_KIT_PERMANENT_SURFACE_GLOBS` to the knob roster with its default.
- **lifecycle-kit/SPEC.md §Multi-operator semantics** — owned by delta (3). Its
  `lifecycle_supersede_set()` prose names the two readers that justify the
  derivation ("the installer's `.gitattributes` block and `check-merge-attrs`'s
  parity check"); the third reader is added there, or the sentence becomes a
  stale count.
- **lifecycle-kit/SPEC.md §check-scratch-citation** — new per-component contract
  section, owned by delta (3), carrying the red condition and the exempt tag.
- **lifecycle-kit/README.md** — gate roster, owned by delta (5).
- **`scripts/lifecycle-config.sh`** — this repo's consumer widening of the glob
  array, owned by delta (4).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
