# SPEC amendment: refusal-record

The turn-end liveness hook records a refusal without recording **which launch
record it refused on**, so a reader cannot tell the guard working — a real
sibling producer is running — from the guard wedged, where the record naming a
live pid is the waiting session's own. This amendment closes that by logging the
record set the decision was taken over, and by naming it in the refusal message.

This is the iteration's **operator-ruled exception to the port-only run**
(TRAJECTORY.md §PRIORITY DIRECTIVE), taken 2026-09-02 by the operator over the
lead's relay after the lead put all three threshold-reaching deferred entries up
as one batch. It rides **neither** port amendment: the host entry's own text
refuses folding a live `[design-pending]` fork into a port cut as non-port design
work, and that refusal binds its own promotion.

**The entry left its fork unsettled and this amendment settles it** — the two
shapes it names are *log `agent_id`, `agent_type` and the matched run key* (a
diagnostic for a later reader) and *name the matched record in the refusal
message* (a steer for the refused session). Neither is taken as stated. What is
taken is stated in delta (1), and the ground is three facts probed at this HEAD
rather than reasoned from the entry's premises.

## What changes

### (1) The fork is answered by neither shape as written, because two of its three fields do not exist to be logged

The entry argues that "the payload already carries the attribution and the hook
drops it" — that `keys=` lists `agent_id` and `agent_type`, "so logging those two
plus the matched record's run key would make every one of these rows readable"
{design-bearing}. Three probed facts falsify two thirds of that and re-aim the
third:

- **The matched run key is not available to the hook at all, and no surface says
  so.** `native/src/proc.rs:241-243` — `run_bounded` spawns the reader with
  `.stdout(Stdio::null()).stderr(Stdio::null())`, so the hook holds the reader's
  **exit code and nothing else**. `check-producer-liveness` set mode does print
  the matched record, its run key and its pid on stdout
  (evidence-kit/SPEC.md §The producer-liveness lock), and that output goes to
  `/dev/null`. Logging "the matched run key" is therefore not the one-table-edit
  the entry costs it at: it would need either a bounded **and capturing** `proc`
  helper — where the 10-second bound is itself load-bearing, keeping a hung
  *reader* from being read as a live *producer* — or the hook parsing records for
  itself, which delegation-kit/SPEC.md §The turn-end liveness hook forbids
  outright as a third copy of a grammar evidence-kit owns.
- **`agent_id` is observed per-firing rather than per-agent.** §The turn-end
  liveness hook's own `session` bullet already carries this, landed from the
  merged `SPEC-agent-id-doubt.md`: five firings in one session carried five
  *distinct* top-level `agent_id` values, none matching the stable identifier the
  same payloads' `background_tasks` array reported for the one live dispatched
  agent. The section states the consequence for an amendment in exactly this
  position — "a consumer proposing to log `agent_id` for attribution is proposing
  to settle that question, not to consume a settled answer" — and settling it
  means reading a raw payload, which the no-values privacy ruling holds
  **operator-class**. So the field is not takeable here even if it were wanted.
- **`session_id` is measured not to discriminate**, and `agent_type`'s stability
  is measured by nothing at all.

**What is left is the fact the entry was actually reaching for.** Its own
statement of the general form is "a refusal is correct when a real producer runs,
and a wedge when the record naming a live pid is the waiter's own". That
discrimination is about the **record**, not about an agent identity — and the
record set is already in the hook's hand and already thrown away.
`count_records` (`native/src/hook/stop_liveness.rs:188-197`) enumerates every
`*.run` entry under the run dir and keeps the count, discarding the basenames in
the same expression. So the attribution this entry wants is **one derivation the
hook already performs**, and the amendment takes it there rather than from the
payload.

**This amendment amends the OPEN-set paragraph and leaves the two omission
rulings standing.** §The turn-end liveness hook declares the record's field set
open "so per-session attribution can be added later" and names the open question
as "the agent id, the agent type, the matched run key"; two other paragraphs
refuse a **session-attribution** field and rule that the refusal message "carries
no session identity, because the hook has none to carry". Those are shape and
identity respectively, and they are reconcilable but not reconciled in the text —
a build session reading only the second would conclude this whole unit was
already refused. Stated in words so it is not left to be inferred: what lands is
a **record** field, not an identity field; the identity refusals are untouched
and this delta strengthens rather than narrows them, because it answers the open
question without minting the field they refuse.

### (2) `runs=` — the record set the decision was taken over, logged before `keys=`

The `FIELDS` table (`native/src/hook/stop_liveness.rs:18-26`) gains one member,
`runs`, **between `records` and `decision`** {design-bearing}. The line grammar
becomes:

```
<UTC ISO-8601>  event=<hook_event_name|->  session=<session_id|->  live=<yes|no>  verdict=<green|red|corrupt|unresolved|error|unavailable>  records=<n>  runs=<comma-separated run keys|->  decision=<refuse|allow>  keys=<comma-separated top-level payload keys>
```

Three placement facts, each load-bearing rather than stylistic:

- **Before `keys`**, because the section rules `keys` stays last: it is the one
  free-ish value, so a space-delimited parse never has to step over it, and a
  field added after it would sit past exactly that field.
- **Beside `records`**, because the two are the count and the names of one set,
  and the section's field table already reads `live`/`verdict`/`records` as a
  group. `decision` keeps its stated position before `keys`.
- **A field, not a widened `records`.** The count has a named reader at a named
  transition today; replacing it would move that reader for no gain.

Its value is the `*.run` basenames `count_records` walks, each with the `.run`
suffix stripped, **sorted and comma-joined** — the same join and the same sort
`keys` already uses, so the field mints no second convention and inherits that
field's stated limit that a value containing the separator is not escaped. It
passes through the existing `sanitize`, so whitespace maps to `_` and an empty
set renders `-`; `runs=-` beside `records=0` is therefore the ordinary green row
and needs no reader special case.

**What it settles, and its honest limit, both stated.** Every attested refusal in
all five recorded measurements reads `live=yes records=1 decision=refuse`, so at
`records=1` this field names the matched record **exactly** and the row becomes
readable in one glance: a run key naming the refused session's own stage is the
wedge, a run key naming a sibling producer is the guard working. At `records>1`
it names the **candidate set** rather than the match, because the hook has no
reading that distinguishes them, and that is declared here rather than left for a
reader to discover — a superset is strictly more than the nothing carried today,
and claiming it is the match would be the claim the discarded stdout forbids.

### (3) The refusal message's two record-bearing arms name the same set

`refusal()` (`native/src/hook/stop_liveness.rs:202-225`) keeps its **three-way**
branch — one arm per refusing verdict, never two, which the section rules — and
its `red` and `corrupt` arms gain the same run-key set in their finding
{design-bearing}. The `red` arm's finding names the record set that holds a live
producer; the `corrupt` arm's names the set one member of which does not parse.
The `unresolved` arm is **unchanged**: its record set is empty by construction
(the reader exited 2 over an empty set), so a field naming it would print `-` in
a sentence about a reader that could not run.

**This is the entry's fork (B), and it is bought by delta (2)'s mechanism at zero
additional cost** — which is the finding rather than a convenience. The two forks
are presented as a choice between serving a later reader and serving the refused
session; they differ only in *where one derived fact is rendered*, and the
derivation is the whole price. Taking one and declining the other would defer the
other's whole value to buy nothing back.

Both arms keep every existing sentence: the two lawful exits guard-kit requires
of a block message, and the `Run bash <liveness_cmd> <run_dir>` line. The message
moves from *run this to see the record set for yourself* to *this is the record,
and run this to see it for yourself*, which is what makes it a steer rather than
a notification.

**It carries no session identity, and that sentence in the section stays true.**
A run key is the name a session chose for a piece of work at launch, written into
a record the same session wrote; it is not an identifier the harness assigns and
it attributes nothing about who is running. The section's ruling that the hook
"has none to carry" is unamended.

### (4) `agent_id` and `agent_type` are refused, and `session=` is kept

Neither payload identity field is logged, on delta (1)'s grounds, and that
refusal is written into the section rather than left as an absence
{design-bearing} — the omission paragraph gains this unit's answer, so the next
reader meeting the same idea finds it costed rather than re-deriving it.

**`session=` stays.** The entry's third candidate shape drops it, on the ground
that it "serves neither but stops a field reading as attribution while carrying
none". That is refused: the field has a **named reader at a named transition** in
the section's field table — separating one top-level session's firings from
another's — and the section's governing rule is that no field is carried that the
table does not name a reader for, never that every field must discriminate every
question. Dropping it would delete a live reader to fix a misreading, and the
misreading is fixed instead by `runs=` carrying the discrimination that field
never claimed.

### (5) The field table is zipped positionally and truncates silently, so the arity assertion is the guard

`append_record` zips `FIELDS` against the `values` array
(`native/src/hook/stop_liveness.rs:102-110`, `:124`), and a length mismatch
**silently truncates** rather than panicking {mechanical}. So the `values` array
gains its member in the same edit as the table, and the two crate tests that pin
the arity are updated rather than merely surviving:

- `the_record_is_key_addressed_over_the_field_table` (`:489`) asserts
  `values.len() == FIELDS.len()` against a hardcoded seven-element array — it
  fails until the literal is extended, which is the guard working.
- `whitespace_in_a_payload_cannot_split_the_line` (`:436`) asserts exactly **8**
  whitespace-separated fields ("stamp + 7 keys") and becomes **9**.

A third case, `no_reader_at_all_is_unavailable_and_allows` (`:293`), lists every
field in its expectation and would pass unchanged with the new field absent from
its `want`; it gains the field, because a case that silently under-covers a field
is the shape this delta exists to stop. Two further cases assert refusal-message
substrings and move with delta (3).

**A new case is owed and is named here rather than left to the build:** a firing
whose run dir holds a record asserts that the row carries that record's key, and
a firing over an empty dir asserts `runs=-`. That is the firing/non-firing pair
this kit's testing section requires of every rule, transplanted.

### (6) The wired-arm test and the generated mirror move in the landing commit

`scripts/gate-tests/subagent-stop-reader.test.sh` fires the **real wired arm**
against the **real configured reader**, and its arm B stages a `pid=1 run=k`
record and asserts `verdict=red live=yes records=1 decision=refuse` plus the
refusal message's substrings {mechanical}. It gains `runs=k` on that arm and the
message assertion follows delta (3). Arm A (empty dir, green) gains `runs=-`.
That file is this change's only end-to-end oracle — the hook is a non-gate arm,
so it owes no `good/`+`bad/` pair and none exists.

The section edit stales `docs/delegation-kit/SPEC.md`, the generated on-site SPEC
mirror, which `check-docs-mirror-fresh` byte-gates; it is rostered with its
trigger and regen command in docs/site-architecture.md §Generated projections and
is discharged in the landing commit.

### (7) The host entry promotes, is re-priced on its lead line, and reaches Done

`subagent-liveness-log-unattributed-refusal` moves into `## New Features` with
`[design-pending]` swapped for `[spec: SPEC-refusal-record.md]` {mechanical}.
Two arithmetic facts, measured rather than estimated, so the promotion is not the
place they are discovered:

- **The lead line does not fit the tag as it stands.** It is 93 columns against
  `check-queue-wrap`'s resolved floor of 100 — `QUEUE_KIT_WRAP_BUDGET`, default
  `100`, not overridden in this repo's queue config — leaving 7 free where the
  tag costs `9 + len(basename)`. Dropping `[design-pending]`, which the promotion
  retires anyway, frees 17; the trailing em-dash clause is trimmed for the rest,
  and the prose it sheds flows onto the entry's second line.
- **The entry is at the cap and the promotion is nonetheless free.** Its extent
  is 52 lines and its counted size is 50 against `QUEUE_KIT_ENTRY_LINE_CAP` = 50
  — the count discounts **at most one line of each declaration grammar**, so one
  of its three `ruled:` lines and its one `recurrence:` line are discounted and
  the other two `ruled:` lines are counted — **zero headroom**. The cap binds
  deferred entries only: queue-kit rules the active sections uncapped, because an
  active entry's residency is one iteration by the drain rule. So the body edit
  is unpriced at promotion, and the entry **reaches `## Done`** at build rather
  than demoting, its deliverable being this change and not a corpus.

The promoted body sheds the design rulings this file now owns — the
`[design-pending]` fork paragraph and the three-declination history — rather than
keeping a second copy of them.

### (8) The coupled wedge entry is cited, not drained

`wait-record-self-deadlock` is the entry that holds the wedge this log could not
show: a validate session backgrounded a wait, wrote a record naming **its own**
pid, and that one record refused SubagentStop **21 times** in ten minutes on
2026-08-26, every row reading `live=yes verdict=red records=1 decision=refuse`
{mechanical}. Under delta (2) every one of those rows would have carried the run
key naming that session's own stage, which is the whole discrimination this unit
buys — stated here because it is the strongest available evidence that the field
is the right one, and because it is a *worked* case rather than an argument.

That entry is **not** drained, fixed or promoted here. The port-only run promotes
nothing, this unit is the single ruled exception to it, and the exception was
ruled over this entry alone. Its three candidate fixes stay its own.

## Producers and consumers

The amendment introduces **one new field** and changes **two message arms**. It
introduces no new state, no new event, no new interface, no new knob, and no new
file.

- **Producer of `runs=`** — `fire()` in `native/src/hook/stop_liveness.rs`, on
  **every** firing, allow and refuse alike, at the one existing `append_record`
  call. Its input is the `*.run` basename set `count_records` already walks under
  `${GATE_SDK_TMP_DIR}`; the derivation moves from *count the entries* to *keep
  the names and count them*, so the enabling configuration is the one already
  resolved — `GATE_SDK_TMP_DIR`, which the arm declares today and which
  `gate_knob_env` already resolves for it. No knob is added, so there is no
  configuration a deployment could fail to set.
- **Consumer of `runs=`, named at its transition** — the **close-stage triage at
  the close-surface drain** (lifecycle-kit's close template, step 4: run
  `--emit close-surfaces` and disposition every row). That is the same reader and
  the same transition the section's field table already names for `verdict=error`,
  `verdict=unresolved` and `decision`, and it is the only surface in the tree that
  reads these rows at all: no code anywhere parses a log row, the roster
  derivation emits the path and never the contents, and `check-close-surfaces`
  asserts declaration presence and shape only. The reader's question is the
  entry's own — working or wedged — and `runs=` beside `decision=refuse` is what
  answers it.
- **Consumer of the changed refusal message** — the **refused session**, at its
  own turn end, on stderr, as the hook's exit-2 block reason. It is a second
  consumer of one derivation rather than a second derivation.
- **Consumer of the arity** — the two crate tests at `:436` and `:489`, at every
  `cargo test`, which `check-crate-arms` runs at pre-commit; and
  `scripts/gate-tests/subagent-stop-reader.test.sh`, which drives the wired arm
  end to end.

**The field's honest limit on countability is the section's existing one and is
not widened.** The named close reader resolves the log path against the *writing*
session's cwd, so a worktree-isolated agent's firings are invisible to it — the
later reader `runs=` serves is the main-checkout close. That limit is stated in
the section already; this amendment inherits it rather than restating it as new.

**No corpus is narrowed by this amendment**, so §The causal-completeness check's
point 5 has no reader to enumerate: nothing is pruned, no glob tightens and no
file is dropped. The one file-set change is additive — a new crate test case —
and every gate whose verdict ranges over the crate source is monotone in it.

**Cross-component signal.** This amendment's own component is `delegation-kit`,
and its body's contract-surface tokens reach `evidence-kit` (the
`pid=<n> run=<key>` grammar and `check-producer-liveness`'s output, both cited
rather than re-implemented) and `lifecycle-kit` (the close-surface drain that
reads the field). Two **sibling** amendments land in `delegation-kit/` and
`gate-sdk/` this iteration in any case, so `check-stage-entry` assertion C fires
on the amendment-files-span-two-components arm and the **align stamp is demanded
at the build stage's entry**. Stated here so the build session is not the one
that learns it.

## Existing sections updated

- `delegation-kit/SPEC.md §The turn-end liveness hook`, the fenced line grammar —
  gains `runs=<comma-separated run keys|->` between `records` and `decision`
  (delta 2).
- `delegation-kit/SPEC.md §The turn-end liveness hook`, the field table — gains a
  `runs` bullet naming its reader at a named transition, which the section's own
  rule requires of every field and without which the field would be inadmissible;
  the `keys`-stays-last sentence gains the new field's placement as its second
  worked instance (delta 2).
- `delegation-kit/SPEC.md §The turn-end liveness hook`, the **field-set-is-OPEN**
  paragraph — its live open question is answered in part: the matched run key is
  recorded as **unavailable to the hook** with its cause, and the record set is
  recorded as what stands in for it. The paragraph stops describing the question
  as open in full (deltas 1 and 2).
- `delegation-kit/SPEC.md §The turn-end liveness hook`, the **two refused
  omissions** paragraph — gains this unit's costed refusal of `agent_id` and
  `agent_type` and its refusal of the drop-`session=` shape, so the omissions are
  answered rather than merely restated. The session-attribution refusal itself is
  unchanged (deltas 1 and 4).
- `delegation-kit/SPEC.md §The turn-end liveness hook`, the **refusal-message**
  contract — the `red` and `corrupt` arms carry the record set; the three-way
  branch and the two-lawful-exits requirement are unchanged; the "carries no
  session identity" sentence is restated as still true of a run key (delta 3).
- `delegation-kit/SPEC.md §Attribution was weighed and is not available` —
  unchanged in its ruling, and named here because a reader arriving at delta (2)
  from that paragraph must find the shape/identity distinction drawn rather than
  inferred (delta 1).
- `delegation-kit/SPEC.md §Testing` — the wired-arm test's arms A and B gain the
  new field, and the new firing/non-firing pair for `runs=` is stated as this
  rule's own transplanted fixture-pair discipline (deltas 5 and 6).
- `docs/delegation-kit/SPEC.md` — the generated on-site SPEC mirror, stale on any
  edit above; rostered with its trigger and regen command in
  `docs/site-architecture.md` §Generated projections (deltas 2, 3, 4 and 6).
- `TASK-QUEUE.md`, the `subagent-liveness-log-unattributed-refusal` entry —
  promoted into `## New Features` with `[design-pending]` swapped for this
  amendment's `[spec:]` ref, its lead line re-priced against
  `check-queue-wrap`'s floor, and its body shedding the design rulings this file
  now owns. It reaches `## Done` at build (delta 7).
- `TASK-QUEUE.md`, the `wait-record-self-deadlock` entry — deliberately
  **unwritten**: it is cited as this unit's worked evidence and its own three
  candidate fixes stay its own, the port-only run promoting nothing beyond this
  one ruled exception (delta 8).

<!-- update-target-exempt: the gate binary is rebuilt by every commit touching the crate under this repo's standing build obligation, so it is not a target any single delta claims -->
- The gate binary itself — rebuilt with `bash gate-sdk/bin/build-native.sh`
  beside the battery, neither discharging the other.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls delegation-kit/SPEC-*.md`), the none-remain half discharged
      at the **iteration** rather than at the commit, this iteration carrying a
      sibling `delegation-kit` amendment.
- [ ] **Removals propagated** — grepped every spec, skill, template, README and
      test for the log line's arity and for the refusal message's asserted
      substrings; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **The arity guard fired and was cleared** — the two hardcoded-arity crate
      tests were run *before* the literals were extended and observed to fail,
      so the silent-truncation hazard is proved caught rather than assumed.
- [ ] **The pair exists** — a firing over a populated run dir carries the run
      key and a firing over an empty one carries `runs=-`, both in the crate
      module and in the wired-arm test.
- [ ] **The identity refusal is written, not merely absent** — the section says
      why `agent_id` and `agent_type` are not logged and why `session=` stays,
      so the next reader meeting the idea finds it costed.
- [ ] **The mirror and the binary are fresh in the landing commit** —
      `check-docs-mirror-fresh` and `check-gate-binary-fresh` green.
