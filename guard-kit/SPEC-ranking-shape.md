# SPEC amendment: ranking-shape

Queue entry: `prompt-ranking-ungrantable-shape-class`. **Operator direction,
2026-09-12**, joining the entry to unit set `registry-roster-oracles` on its
recurrence count — three, the tree's highest — rather than on the set's surface.

The friction ranking sends every prompting row to a triage whose first
disposition is *allowlist it*, and some of those rows are rows **no allowlist
entry can ever match**. So each close re-triages a population one of the three
dispositions structurally cannot reach, and the entry's measurements put such a
row at the head of the ranking every time.

The entry declares its design question **answered** — partition the ranked output
by shape, as it already partitions overlay-covered rows — and leaves open only
where the partition lands. This amendment lands it, and narrows the predicate the
entry proposed, because the wider one contradicts the ranker's own model.

**State at build: all four deltas are applied; this file's deletion and the
entry's Done move are held** until the entry body's one record with no other
governed home — the unlanded `cat >> .tmp/*` allowlist grant, which a live entry
cites — has a disposition.

## What changes

### (1) The ranker computes an allowlist-reachability verdict per logged call

A call carrying a shape the permission matcher refuses outright is
**allowlist-unreachable**, decided from primitives `lib/guard.sh` and its compiled
twin already hold {design-bearing}.

Two shapes qualify, each on a ground the ruleset already states in its own refusal
text rather than on a new judgment:

- **An expansion, substitution or backtick.** Rule 6's ground is that "the
  harness's matcher refuses every expansion, so no allowlist entry can match the
  command" (`guard-kit/lib/guard.sh:432`), and the assignment and brace arms say
  the same for their glyphs (`:436`, `:459`, `:461`). No entry reaches these.
- **A write redirect.** Rule 16's ground is that a redirect "defeats the permission
  matcher, so it is always decided out of band"
  (`guard-kit/SPEC.md:1068-1069`). The detection reuses `guard::redirect_pairs`,
  the scan the ranking key's write-shape suffix already performs, with rule 17's
  own target test excluding an fd-dup. **No parser is minted.**

**Chaining is excluded, and the exclusion is the correction this delta makes to
its own entry.** The entry's structural ground reads "any chaining, redirect or
expansion breaks the match". Chaining does not: §The triage criterion states that
"the harness matches the allowlist **per segment** of a compound command" and
concludes that the chained banner and diagnostic tools "are therefore themselves
legitimate allowlist entries; allowlist them"
(`guard-kit/SPEC.md:96-102`). The ranker already agrees — `granted()` returns true
only if **every** segment is granted, which is a model in which a compound is
reachable. Including chaining would have made the partition contradict the
function computing it, and would have retired rows an allowlist genuinely can fix.

**The verdict is per logged call, and a ranking key aggregates calls**, so a key
can hold both kinds. A key is reported unreachable only when **every** call under
it is — the conservative direction on purpose: under-retiring leaves an
unactionable row on the actionable list, which is today's cost and is visible;
over-retiring hides friction a reader could have fixed, which is silent.

### (2) The prompting ranking partitions into two sections, and the denominator does not move

The headline count stays a true prompt count; the ranked rows below it split into
an actionable section and an allowlist-unreachable one {design-bearing}.

The output already carries this shape: `overlay_section`
(`native/src/emit/scan_prompts.rs:296-313`) is a separate, visibly-advisory
section below the headline. The unreachable partition is that same construction on
the other side of the headline, and the entry's "as it already partitions
overlay-covered rows" is a pointer to it.

**The two sections are not symmetric with the overlay one, and that is the whole
design decision.** An overlay-covered row is excluded from the headline because it
**did not prompt** — §scan-prompts defends that explicitly, "the count is a true
prompt count, not an upper bound" (`guard-kit/SPEC.md:1962`). An
allowlist-unreachable row **did** prompt, and will prompt again on every future
call of that shape. Moving it off the denominator would make the count stop
measuring the thing the KPI reads it for. So:

- `<n> prompting call(s) across <m> pattern(s)` is unchanged, and
  `kpi-prompt-friction`'s two integers (`native/src/emit/kpi/prompt_friction.rs`)
  are unchanged with it.
- The headline gains **one derived clause** naming how much of that total is
  allowlist-unreachable — the figure that tells a reader at a glance how much of
  the ranking the first disposition cannot touch.
- The ranked rows are printed in two labelled sections rather than one list.

**What the partition is *about* is narrower than "ungrantable", and the section
says so.** It reports which rows disposition **(a) allowlist** cannot reach. It
makes no claim about (b) guard rule or (c) habit change, both of which remain
available for every row — a guard rule reaches shapes no glob can express, which
is the disposition's stated reason for existing. The entry proposes a fourth
disposition, "structurally ungrantable, retired from the actionable set"; this is
that disposition scoped to the thing the ranker can actually decide. Claiming more
would put a judgment in an oracle's mouth, which is the shape §check-queue-entry-budget's
own refusals name as the standing failure.

### (3) The triage criterion states which dispositions a shape-unreachable row admits

§The triage criterion gains the reachability limit, so the close step reads the
partition as an instruction rather than as decoration {design-bearing}.

The criterion today lists three dispositions and a caution, and its closing note
already tells a reader that one unmatched segment takes a whole line off the match
path. It gains one paragraph: for a row the ranking marks allowlist-unreachable,
disposition (a) is **unavailable rather than unattractive**, and the row is
resolved by (b) or (c) or left standing as measured friction — a standing row
being an honest outcome, not an unfinished triage. §The close-stage triage step
inherits this through the criterion it already cites, so the close template needs
no edit of its own.

### (4) The advisory tier is stated for the new section

The unreachable section declares itself advisory on the same terms the log already
does {mechanical}.

The friction log is "a capture-tier surface with no forcing function" whose
close-surface row is already advisory (`guard-kit/SPEC.md:2420-2425`). The new
section changes nothing about that: it reds nothing, gates nothing, and the
close-surface roster line is unchanged. Stated so the partition is not read as a
new obligation on close.

## Producers and consumers

- **The allowlist-reachability verdict** (delta 1).
  - *Producer:* `scan_prompts::tally`, at the per-line loop where it already
    computes the ranking key and the grant test
    (`native/src/emit/scan_prompts.rs:238-272`). Reachable in the deployed
    configuration: the arm runs from the close triage step, and the log it reads is
    written by the live guard on every fall-through
    (`guard-kit/lib/guard.sh:105-109`).
  - *Consumers:* the render pass, which places the row in one of two sections; and
    the headline's derived clause.
  - *Inputs:* `guard::redirect_pairs` and `guard::skeleton`, both twinned. **Aged
    premise, corrected at build:** rules 6 and 7's predicates have no compiled
    twin, so the expansion test is a ranker-local scan (as `redirect_op` is), not a
    guard primitive — and rule 17's target test exempts `/dev/null` as well as an
    fd-dup, which the verdict follows. No new knob, no new kit literal, and
    nothing consumer-specific — the shapes are harness matcher facts, the same
    class §The generic ruleset already rules "public, documented, and shared by
    every consumer of that harness" and therefore outside the provenance seam.
- **The partitioned ranking** (delta 2).
  - *Producer:* `render`, at the point it calls `rank_section` today.
  - *Consumer:* the close-stage triage step, at the triage criterion; and a human
    reading the report.
  - *Every new field has a named reader:* the headline's unreachable clause is read
    at the same glance as the headline it sits on; the section label is read at the
    triage. There is no third field because there is no third reader — in
    particular **no per-row shape token is emitted**, since a reader who needs to
    know *which* shape made a row unreachable reads the row, whose command word the
    key already carries.
- **The `Tally` struct** gains one collection beside `prompting` and `overlay`,
  with `prompting` keeping its current meaning and its current arithmetic. Named
  because `count()` reads `prompting.len()` and `total`
  (`scan_prompts.rs:283-286`), and those two reads must not move.
- **Red conditions of the affected readers.** **No delta narrows a corpus** — the
  log is read whole, exactly as today, and every call still lands in exactly one
  bucket — so the point-5 narrowing hazard does not arise. The readers that could
  nonetheless flip, each by its red condition rather than its subject:
  - `kpi-prompt-friction` has no red condition; it emits `n/a` when guard-kit is
    absent or nothing is logged. Its two integers are unchanged by construction,
    which delta 2 makes an explicit requirement rather than a hope.
  - `check-comment-tier` reds on a comment that is not a directive. Every new
    `// spec:` pointer this unit adds binds one line, per canon-kit's
    one-line-binding rule.
  - `check-crate-arms` reds on a failing crate lint or test arm; the unit tests
    land with the code.
  - `guard-lib-parity.test.sh` holds `lib/guard.sh`'s twinned primitives to their
    compiled counterparts. **This unit adds no primitive**, which is why it reuses
    `redirect_pairs` rather than writing a redirect scan — a new primitive would
    owe a twin on both substrates and is exactly the cost this design avoids.

## Existing sections updated

- `guard-kit/SPEC.md` §scan-prompts, the three-way split at 1953-1965 (deltas 1
  and 2). The **prompting** bullet gains its two-section partition and the
  statement that the partition is over rows, never over the count; the reason it
  differs from the overlay bullet's exclusion is stated at the overlay bullet
  itself, which is where a reader meets the precedent. **Applied.**
- `guard-kit/SPEC.md` §scan-prompts (delta 1). A new passage owns the
  reachability predicate: its two shapes with the rules whose grounds they are,
  the **exclusion of chaining** with §The triage criterion's per-segment rule as
  its ground, and the every-call-under-a-key rule with the direction of its
  conservatism.
- `guard-kit/SPEC.md` §The triage criterion, lines 80-102 (delta 3). The
  reachability limit and what a row admits when disposition (a) cannot reach it.
  **Applied.**
- `guard-kit/SPEC.md` §The close-stage triage step (delta 4). One clause recording
  that the new section is advisory on the log's existing terms and that the
  close-surface row is unchanged — so the addition is not read as a new forcing
  function.
- `native/src/emit/scan_prompts.rs`'s header `// spec:` pointer (deltas 1 and 2).
  It currently says the fall-throughs "split three ways"; after this unit the
  prompting share itself splits, which is a different claim. **Applied.**
- `docs/guard-kit/SPEC.md` (all deltas) — the generated on-site mirror; regenerate
  with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write`.

**Deliberate non-updates.** `guard-kit/templates/close-triage.md` is unchanged: it
cites the triage criterion rather than restating it, so delta 3 reaches it through
the pointer it already carries. `lib/guard.sh` is unchanged: no rule's decision
moves, and the ranker is a reader of the log rather than a participant in the
decision.

## Retired spellings

- None — no delta removes a name. `prompting` keeps its spelling, its meaning and
  its arithmetic; `overlay` keeps its section; `ranking_key` keeps its grammar, and
  in particular the write-shape suffix is **read** by delta 1's predicate rather
  than replaced by it.

## Definition of Done

- [ ] **Causal completeness** — every new state and interface has a named,
      reachable producer and a named consumer; every new field has a named reader
      at a named transition.
- [ ] **The denominator is unmoved, proved by test** — a unit test asserts that
      `count()` returns the same pair before and after a corpus is partitioned, so
      a later edit cannot quietly move the KPI.
- [ ] **The instrument is cleared before the measurement.** The entry records
      `.workflow/prompt-friction.log` as contaminated with a prior iteration's
      calls, so no emitted figure denominates one window. Clear it
      (`: > .workflow/prompt-friction.log`) **before** reporting any figure this
      unit produces, and state the window any reported figure covers.
- [ ] **Chaining is verified excluded** — a test asserts a compound of two
      allowlistable segments is *not* marked unreachable, since that exclusion is
      the delta's own correction to the entry and is the easiest thing to lose in a
      later edit.
- [ ] **Mixed keys stay actionable** — a test with one key covering both a
      reachable and an unreachable call asserts the key lands in the actionable
      section.
- [ ] **No new guard primitive** — `guard-lib-parity.test.sh` passes unchanged, and
      no twin is owed.
- [ ] **Instruction surfaces: instruction only** — no grounds land in a template;
      the grounds are this amendment's and §scan-prompts'.
- [ ] **Full battery green** (`bash gate-sdk/bin/run-gates.sh`),
      `bash gate-sdk/bin/build-native.sh`, and the guard-kit and gate-sdk fixture
      suites.
- [ ] **Merged with no information lost** — each addition integrated into its proper
      canonical-spec section, not appended.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls guard-kit/SPEC-*.md`).
- [ ] **Removals propagated** — `## Retired spellings` above holds, re-run by
      `check-amendment-retired-spelling`.
- [ ] **Gaps filed** — a cross-component gap found during the work is resolved that
      session, not deferred.
