# SPEC amendment: the launch chokepoint

Closes `launch-chokepoint-liveness-record-write`. Rule 14 blocks a tracked-tree
mutation while a producer a session **recorded** is still live, and
delegation-kit/SPEC.md §The delegation model ships that claim with its bound: *a
session that backgrounds without recording is invisible to rule 14 and to the
entry preflight alike.* The launch itself passes a chokepoint — the same
`PreToolUse(Bash)` hook rule 14 fires in — and nothing fires there.

## The refusal this amendment discharges, and the probe that discharged it

delegation-kit/SPEC.md §The delegation model records the candidate as *designed,
refused, and filed*, on a stated empirical unknown: a shell `&` is in the command
text every rule already reads, but *"a harness's background-this parameter is
**tool input, not command text**, and whether it reaches the `PreToolUse` payload
is an empirical question about one field"* — and building only the `&` arm was
refused as **worse than not building it**, because every attested firing used the
harness form.

**The probe ran at this stage, 2026-08-22, and the field is there.** One
backgrounded Bash call and one foreground control were taken through a guard
temporarily teeing its stdin (the tee was reverted and never committed; the probe
needed no permission-surface and no hook-surface change, exactly as the entry
said). Result:

- `.tool_input.run_in_background` is a JSON **boolean `true`** on a backgrounded
  Bash call, and the key is **absent** on a foreground one.
- The `PreToolUse` payload's top-level keys, recorded so no session re-buys this:
  `agent_id`, `agent_type`, `cwd`, `effort`, `hook_event_name`,
  `permission_mode`, `prompt_id`, `session_id`, `tool_input`, `tool_name`,
  `tool_use_id`, `transcript_path`. `tool_input` on a Bash call carries
  `command`, `description`, and `run_in_background`.

Both backgrounding forms are therefore reachable, the refusal's sole ground is
gone, and this amendment builds **both arms in one rule**.

## What changes

### (1) `guard_read_input` — the payload becomes readable more than once

**Design-bearing**, and it is a framework change rather than a rule: today the
payload is unreadable past the first field. `guard_read_command` reads stdin with
`cat` and is called as `cmd="$(guard_read_command)"` — in a **command
substitution**, so any cache it set would die with the subshell, and stdin is
consumed. A rule needing a second field cannot get one.

The lib gains one primitive and two compatible redefinitions:

- `guard_read_input` — reads stdin **once** into the global `GUARD_INPUT` and
  returns non-zero if stdin yielded nothing. Called **directly** (never in a
  substitution) at the top of a consumer guard, before `guard_read_command`.
- `guard_read_command` and `guard_read_path` parse `GUARD_INPUT` when it is set
  and non-empty, and otherwise read stdin exactly as they do today.
- `guard_input_field <jq-path>` — prints the value at `<jq-path>` in
  `GUARD_INPUT`, or nothing when `GUARD_INPUT` is unset, empty, or the path is
  absent.

**The fallback is what makes this non-breaking, and it is the whole reason for
this shape.** Every existing consumer copy — this repo's four guards and any
vendored elsewhere — keeps working byte-identically without an edit: they never
set `GUARD_INPUT`, so both readers take the stdin path. A guard that wants a
second field opts in by adding one line. The alternative shape, making
`guard_read_input` mandatory, would break every consumer copy on upgrade for a
field only one rule needs.

### (2) Rule: a backgrounded launch that records no producer

**Design-bearing** — the rule, its two arms, its exemption set and its posture.

A **backgrounding call** is a Bash call that is either

- **harness form** — `.tool_input.run_in_background` is `true`; or
- **shell form** — the command's skeleton (`sq dq hd`) ends a statement with a
  bare `&` that is not `&&`.

A backgrounding call **advises** when it neither writes a liveness record nor
falls in an exemption below. The advisory names the record, its grammar, its
home, and the two things the record buys — rule 14's reach and the next
arrival's ability to tell whether the producer is still writing.

**The record-writing test.** The call writes a record when its skeleton carries a
redirect whose target is a path under a `GUARD_KIT_SCRATCH_DIRS` member ending in
`.run`. The test is on the **command text**, because at `PreToolUse` the child has
not started and no record can exist yet: the only thing observable at the launch
chokepoint is whether the launch is *going to* write one. That is the honest
predicate, and it is why the rule advises rather than refuses (below).

**The exemption set, and every member reuses machinery already in the lib.**

1. **The call writes a record** — the obligation is discharged inline.
2. **The call is a wait loop.** A backgrounded `until <cond>; do sleep N; done` is
   the sanctioned wait primitive rule 13 steers *toward*; it is a waiter, not a
   producer, and owes no record. Advising there would make the ruleset refuse the
   form its own corrective recommends. Detected by the `do … done` span walk rule
   13 already performs.
3. **The call is a read-only pipeline.** Every segment leads with a
   `GUARD_KIT_RO_BINS` member and every redirect target is `/dev/null` or an
   fd-dup — rule 16's own test. A child that writes nothing has nothing for a
   later commit to corrupt, so the record buys nothing.

Anything the rule cannot resolve **declines**, the direction every conservative
clause in this ruleset takes: an expansion or substitution anywhere in the
command (rule 6 blocks those shapes already), an unbalanced `do`/`done`, or an
absent `GUARD_INPUT` on a call carrying no shell `&` — in which case the harness
arm is simply unavailable and the rule is inert, which is the graceful
degradation the `guard_read_input` fallback in delta (1) buys.

### (3) The posture is **advise**, and the refusal of a block is reasoned, not hedged

**Design-bearing**, and it departs from the entry's own candidate wording
(*"refusing one that writes no record"*), so the grounds are stated rather than
assumed.

- **The guard cannot tell a producer from a trivial child.** A backgrounded
  `printf` and a backgrounded gate battery are the same shape at this chokepoint.
  A block's false-fire population is therefore every short-lived backgrounded
  call in the tree, and this ruleset's established direction is to bias toward
  passing. Rule 14 departs from that direction on an attested record of failure;
  this rule has no such record to depart on.
- **Both attested firings cost zero.** 2026-08-19 and 2026-08-21, the second
  self-disclosed: a build batch backgrounded its step-0 battery and wrote no
  record. *No orphan resulted, so the cost was zero this time.* The harm is latent,
  and a block is not warranted by a latent harm when the reminder is what was
  missing.
- **The harm already has a block.** Rule 14 refuses the mutation. This rule covers
  the **omission** that hides the harm, one step upstream; stacking a second
  refusal for one harm buys a second stop, not a second catch.
- **What an advisory actually fixes is what actually failed.** Both firings were a
  session that *knew* the rule and forgot it at the call. An `additionalContext`
  note at the moment of the call is exactly that reminder, and it costs one
  sentence rather than a stopped turn.

**The re-opening condition is named rather than left to judgment.** A firing of
the recording omission *after* this advisory ships is evidence the advisory does
not hold, and the block is then the next step with the attested record rule 14's
own departure required. That is filed as the disposition, not left as a hope.

**The honest limit, stated because the entry's title claims more than this
delivers.** An advisory does not refuse. A session determined to background
without recording still can, and rule 14's bound — *only for a session that
recorded* — is narrowed rather than closed. What closes at this chokepoint is the
silent case: the omission is now visible at the moment it is made, to the only
party that can fix it.

### (4) Placement: the rule inserts at 15 and the roster renumbers

**Mechanical**, with one judgment inside it — that the insertion is required
rather than tidy.

The rule joins rules 12, 13 and 14 as the fourth member of the wait-discipline
family, **before both auto-allow rules**, and appending it after them is wrong for
a concrete reason rather than an aesthetic one: `bash gate-sdk/bin/run-gates.sh`
is an allowlisted committed glob and a backgrounded read-only pipeline is granted
outright by rule 16, so a rule sitting after the auto-allows would never run on
two of the shapes it exists for. §The generic ruleset's numbered list is in
execution order and `guard_generic_rules` calls in the same order; keeping both
true means the new rule is **15** and the present 15–20 become 16–21, in the SPEC
list, in every cross-reference to a renumbered rule, and in the `# spec:` comment
lines that name a rule by number.

The renumbering is mechanical but it is not free, and it is taken rather than
avoided because the alternative — a list whose numbering no longer matches
execution order — makes every future placement argument unreadable.

### (5) `_guard_loop_span` is extracted so two rules share one keyword parse

**Mechanical.** Rule 13 carries the only shell-keyword parse in the ruleset —
tokenize the skeleton, walk `do`/`done` spans, track depth. Exemption 2 needs the
same walk. It is extracted to a helper both rules call rather than copied, so the
ruleset keeps one keyword-parsing dialect; a second copy is the drift the
content-tiering rule exists to prevent, in code rather than prose.

### (6) The decision table gains rows — and a second table, because it must

**Design-bearing**, because the finding is a real limit of the existing
instrument rather than a choice.

`guard-tests/cases.tsv`'s grammar is `<expected-decision><TAB><command>`. The
shell `&` arm is a command, so its firing and non-firing rows land there
unchanged. **The harness arm cannot be written in that grammar at all** — its
input is a tool parameter, not a command — so the harness form would ship
untested, which is exactly the coverage-in-appearance the entry's own refusal
paragraph warns about.

The kit already solved this shape once: `guard-tests/escalation-cases.tsv` drives
the same runner from a second table with a different column set, feeding a
`SendMessage`-shaped payload. This amendment takes that precedent — a third table,
`guard-tests/background-cases.tsv`, `<expected-decision><TAB><run_in_background><TAB><command>`,
fed through the template guard as a Bash payload carrying the flag.

Rows owed, each a firing/non-firing pair per the transplanted fixture-pair
discipline: harness-form background with no record → advise; harness-form
background writing a `.tmp/<key>.run` record → fallthrough; harness-form
background of a wait loop → fallthrough; harness-form background of a read-only
pipeline → the auto-allow it already earns; shell-`&` background with no record →
advise (in `cases.tsv`); shell-`&` background writing a record → fallthrough (in
`cases.tsv`); and a foreground call carrying neither → fallthrough, proving the
rule is inert off the backgrounding path.

**The table's red condition is not monotone, and this delta narrows nothing but
must still be re-derived.** guard-kit/SPEC.md §Testing rules that the table fails
on a verdict mismatch **in either direction**. Every existing row whose command
carries a trailing `&` — rule 16's non-firing cases are the population to check —
has its expected column **re-derived** against the new rule, never assumed still
correct. This is the build's obligation and is called out because "a new rule only
adds blocks" is the first thing a builder will reach for and it is false here: the
new rule emits an **advise**, which converts a `fallthrough` row into an `advise`
row wherever it fires.

### (7) The consumer guard opts in

**Mechanical.** `guard-kit/templates/bash-guard.sh` and this repo's
`scripts/bash-guard.sh` each gain one line — `guard_read_input || exit 0` before
the existing `cmd="$(guard_read_command)"` — which is what makes the harness arm
reachable in a shipped guard rather than only in the tests. Without it the rule
still runs and still covers the shell `&` arm; delta (1)'s fallback is what makes
that true.

## Producers and consumers

**`GUARD_INPUT`, the cached payload.**
*Producer:* `guard_read_input`, called directly by a consumer guard at the top of
its run — `scripts/bash-guard.sh` and the shipped template, per delta (7), so the
producer is on the live hook path and not test-only. Its enabling config is none;
the primitive needs no knob.
*Consumers:* `guard_read_command` and `guard_read_path` (as their preferred
source), and `guard_input_field`.
*Reader for the one new field:* `.tool_input.run_in_background` is read by the new
rule, through `guard_input_field`, at the backgrounding-form test — the single
transition where the rule decides whether a call is a launch. It is read nowhere
else and populated nowhere else. No other payload field is added to the read set:
`permission_mode`, `agent_id`, `agent_type` and `effort` are present in the
payload and are deliberately **not** read here — a field with no reader is not
shipped, and each of those belongs to an entry of its own.

**The advisory.**
*Producer:* the new rule, via `guard_advise`, which emits the existing
`additionalContext` shape — no new emitter, no new hook event.
*Consumer:* the model in the session making the call, at the `PreToolUse` return,
before the command runs. There is no log and no persistent state: an advisory is
transient, exactly as the escalation-guard's is, so nothing accrues for the
close-stage triage and no `close-surface:` declaration is owed.
*Red condition:* the rule has no verdict — it advises and returns. The
**decision table** is the reader with a red condition, and it is stated in delta
(6): it reds on a verdict mismatch in either direction, which is **not** monotone,
so no existing row is cleared by inspection.

**The record-writing test's corpus.**
*Producer:* `GUARD_KIT_SCRATCH_DIRS`, already declared and already read by
`_guard_live_run_records`. *Consumer:* the new rule's redirect-target test.
*Red condition:* none in the rule. The knob's other reader, rule 14, is unchanged
by this amendment — no member is added or removed — so its own monotonicity is not
disturbed.

**No corpus is narrowed by this amendment.** Causal-completeness point 5's
narrowing clause is inert: the ruleset gains a rule and loses none, no glob
tightens, and no file is pruned. The one non-monotone reader in the unit is the
decision table, and its re-derivation obligation is stated in delta (6) rather
than cleared.

## Existing sections updated

Each names the delta that owns it.

- **guard-kit/SPEC.md §The generic ruleset** — the new rule lands as **15**, with
  its two arms, its exemption set, its decline directions, its advise-not-block
  grounds and its re-opening condition (deltas 2, 3); rules 15–20 renumber to
  16–21 and every in-SPEC cross-reference to a renumbered rule moves with them
  (delta 4); rule 13's entry gains the note that its keyword walk is now shared
  (delta 5). Rule 14's own section gains the pointer that its stated bound — *only
  for a session that recorded* — is now narrowed at the launch chokepoint, and is
  **not** claimed closed (delta 3's honest limit).
- **guard-kit/SPEC.md §The guard framework (`lib/guard.sh`)** — the primitive
  roster gains `guard_read_input` and `guard_input_field`, and the sentence
  describing `guard_read_command`/`guard_read_path` gains the
  `GUARD_INPUT`-first-then-stdin resolution and the reason it is ordered that way
  (delta 1). This is the section a consumer reads to write a guard, so the
  opt-in line belongs in its description of a guard's shape (delta 7).
- **guard-kit/SPEC.md §Consumer rules** — the template's shape is described here;
  it gains the `guard_read_input` line (delta 7).
- **guard-kit/SPEC.md §Testing** — the third decision table, its columns, its
  runner reuse and the fixture-pair rows are specified beside the escalation
  table's paragraph, which is the precedent being extended (delta 6); the
  non-monotone re-derivation obligation is already stated there and is cited
  rather than restated (delta 6).
- **guard-kit/SPEC.md §Layout and configuration** — the layout block gains
  `guard-tests/background-cases.tsv` (delta 6). **No knob is added**, and that is
  recorded so the merge does not look for one.
- **guard-kit/templates/bash-guard.sh** and **scripts/bash-guard.sh** — the
  opt-in line (delta 7). The repo copy carries `copy-divergence:` comments for
  every place it differs from the template; this line is **not** a divergence and
  must land in both, so no such comment is owed.
- **delegation-kit/SPEC.md §The delegation model** — the paragraph beginning
  *"Enforcing the record's write at the launch chokepoint is designed, refused,
  and filed"* is the prior-flow prose this change falsifies. It is rewritten:
  the probe is settled, the field's name and shape are recorded, the refusal is
  discharged, and the rule is named — with the *building only the `&` arm is
  worse* judgment **kept**, because it is what made the two-arm rule the right
  build rather than a bigger one (deltas 2, 3). The adjacent paragraph shipping
  rule 14's bound — *a session that backgrounds without recording is invisible to
  rule 14 and to the entry preflight alike, exactly as it is today* — has its
  closing clause updated: the session is now **advised at the launch**, and still
  not refused (delta 3).
- **delegation-kit/templates/agent-execution.md** — the launch-record bullet
  states the record's grammar and home; it gains no new obligation, and is
  re-read at the merge to confirm the advisory's wording does not diverge from
  the rule the template already states (delta 2). Recorded so the merge checks it
  rather than assuming it.
- **The `docs/` kit-SPEC mirror** — `docs/guard-kit/SPEC.md` and
  `docs/delegation-kit/SPEC.md` are generated projections of the kit SPECs and go
  stale the moment either is edited. The regen command and its freshness gate are
  rostered at docs/site-architecture.md §Generated projections; the merge runs it
  in the same commit (all deltas).

## What the build owes beyond the deltas

- The probe's payload facts above are **this amendment's**, and they merge into
  guard-kit/SPEC.md §The guard framework as the statement of what a `PreToolUse`
  payload carries, so no later session re-buys the probe.
- A gap is **already filed**, 2026-08-22 at this stage, in the committed gap
  inbox: **nothing in the tree checks that a `hooks[].hooks[].command` path
  exists.** `check-settings-paths`'s subject is `permissions.allow[]` alone, so a
  hook registration naming a script that was renamed or deleted reds nowhere and
  fails silently at run time. Found while surveying the settings gates for the
  sibling amendment; it is not this unit's work and the build owes nothing
  further on it.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls guard-kit/SPEC-*.md`), discharged at the iteration rather
      than at the commit.
- [ ] **Removals propagated** — grepped every spec, template and comment for a
      rule number this renumbering moved, and for the *designed, refused, and
      filed* claim this amendment falsifies; nothing dangles.
- [ ] **Gaps filed** — the hook-path gap above is filed; cross-component gaps
      discovered during the work are resolved that session, not deferred.
