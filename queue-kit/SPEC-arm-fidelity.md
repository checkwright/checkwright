# SPEC amendment: arm-fidelity

Four reporting arms of queue-kit report less than they hold. The retired block
tells a reader that a citation points at disposed work when the name it points
at is live mechanism the tree still carries; the eviction worklist spends its
whole cause budget on a fixed prefix and shows three characters of the grounds;
`entry-history` refuses the one entry whose history a reader most needs — the
one that has left the queue; and `queue-counts` reports section sizes with no
way to ask how the pool partitions by a tag. One amendment because the four
share one adapter layer and two of the deltas move the same derivation.

## What changes

### (1) The retired block separates a retired *entry* from a name still live in the tree

§The queue-edges arm's retired block is derived from the queue file's own
history: a slug that held a top-level entry lead line in some earlier revision
and holds none now. That test is correct and stays. What it does not answer is
whether the *name* is still live somewhere other than the queue — and where it
is, the citations of it are current prose about a living thing rather than
pointers at disposed work. The arm marks that row. **design-bearing.**

A retired target is marked **name-live** when the tracked tree holds a file
whose basename, less its final extension, is exactly the slug. The retired block
renders it as

```
<slug> (<N> inbound, retired — name live at <path>)
```

where `<path>` is the first such file in `git ls-files` order, and a second and
further match append ` (+<M>)` inside the parenthesis. An unmarked row renders
byte-for-byte as it does today.

**The input is the tree listing, and that is the whole seam ruling this delta
makes.** §The tag algebra already refused reading gate-sdk's `check-root-tiering`
allowlist for a set the tree yields, on the ground that it would couple a
queue-kit reader to a gate-sdk consumer file. The same ground reaches
`scripts/gates.list`: the registry is a curated roster, the tree is not, and
**the two give the same answer here** — measured at rev `b2bcd457` over 792
distinct tracked-file stems against the 56 retired targets, the stem test yields
exactly one hit, `check-spec-pointer`, which is also the only hit scope's sweep
found against `gates.list` and against every `*.gate` in the tree. So this
amendment lands **no gate-sdk clause**: no registry read, no `.gate` literal, no
gate vocabulary anywhere in queue-kit. The predicate names no kit and no
extension — a knob file, a script, a template and a gate descriptor all satisfy
it identically.

**The test under-claims and never invents, which is the posture the block
already has.** A live name that is not a tracked file's own stem — a gate
registered with no descriptor, a knob, a section heading — is not marked, and
the row reads exactly as it does today. That degradation is the same shape as
the section's declared history degradations, and it is why the mark is an
addition to a reader's evidence rather than a filter on the block: nothing is
dropped, so a missed mark costs the read it costs now.

**No verdict, no filtering and no gate.** §The queue-edges arm's no-red posture
is untouched — the block gains a column of evidence, not a classification, and a
marked row is still a row a reader reads.

Measured at rev `b2bcd457`: 56 retired targets over 88 retired citing rows
(101 live). The single marked target carries 15 of those 88 — 17 percent of the
finding set close is sent to act on, at the block's largest target.

### (2) The ineligibility cause prints its class whole and caps only its variable tail

§The queue-index arm caps every `--icebox-candidates` cause at one fixed
character count. The cap is applied to the whole cause string, so a class whose
fixed prefix is nearly as wide as the cap shows almost none of its variable
part. Two changes, one rule: the `[standing]` prefix loses a token the `✗` mark
and the class token already say, and **the cap binds each cause's variable tail,
never its fixed class prefix**. **mechanical.**

The cause corpus is closed and every member is named here with its split:

| cause | fixed prefix (prints whole) | variable tail (capped) |
| --- | --- | --- |
| roadmap tag | `[roadmap] tag — not icebox-eligible` | none |
| standing declaration | `[standing] <date> — ` | `<grounds>`, or `(ungrounded)` |
| dated recurrence | `[recurrence] dated re-filing — live trigger` | none |
| named live slug | `[trigger] names live slug ` | `<slug>` |

`<date>` is the declaration's date or `(undated)`. Two of the four have no
variable part at all and so can never be cut; the other two print their class
and then up to `CAUSE_CAP` characters of the one field a reader came for.

`CAUSE_CAP` stays a module constant at 48 rather than becoming a knob.
`QUEUE_KIT_ATTEND_CAP` is a knob because it budgets tokens on the
**always-loaded** session-context brief; this worklist is read once at close,
budgets nothing standing, and a consumer wanting more grounds wants the line
itself. A knob here would have no reader but `check-knob-citation`.

Measured at rev `b2bcd457`: `--icebox-candidates` renders five standing rows,
every one of them cut to three characters and an ellipsis (`wit…`, `evi…`,
`liv…`, `liv…`, `ado…`) against a 44-character prefix and a 48-character cap.
After this delta each shows 48 characters of grounds behind a 21-character
prefix.

### (3) The retired-set derivation moves into the shared queue adapter

`retired_set` lives in `native/src/emit/queue_edges.rs`, where
§The queue-edges arm sited it under the module's own rule: a derivation with one
reader stays in its arm. Delta 4 gives it a second reader, and
§The shared queue adapters states the consequence for exactly this case — the
lead-line grammar is shared "by that same rule rather than by convention", and a
second reader is what promotes a derivation into that roster. The derivation
moves to `native/src/queue.rs` unchanged, behaviour for behaviour, including
every declared degradation. **mechanical.**

Nothing else moves with it. The body-citation scan stays in the arm — it still
has one reader — and delta 1's name-live test is a *rendering* input with one
reader, so it is authored in the arm rather than beside the derivation. The
module's rule is applied in both directions in one delta, which is the point of
stating it here.

### (4) `entry-history` addresses a departed entry, bounded by the retired set

`--emit entry-history <slug>` walks the queue file's commits newest-first and
stops at the first carrying no entry for the slug. An entry that has *left* the
queue has no entry at the newest commit, so the walk ends before it starts and
the arm refuses with "the arm reads a live entry" — exactly when a reader most
needs what the disposition commit took with it. **design-bearing.**

**The addressable domain becomes the live slugs plus the retired ones**, which
is the domain §The queue-edges arm already widened `--inbound` to, for the same
reason and with the same consequence for silence. That domain is also the
**bound** the entry's `[design-pending]` marker asked for: a back-search for a
departed entry's last live revision is unbounded only when the slug may never
have existed, and retired-set membership settles that from one history pass
before any blob is read.

- A slug that is **neither live nor retired** refuses at exit 2 with no history
  walk at all — the harness-error class the arm's two-valued exit contract
  already carries, and never a finding.
- A **live** slug behaves exactly as it does today.
- A **retired** slug: the walk searches back from the newest commit to the
  newest one carrying a counted entry for the slug. The commit *after* that one
  in log order is the entry's **departure commit**. The fall report then runs
  from the last live commit back to the filing commit, unchanged.

The header names the departure, because that is the one fact a departed-entry
report has that a live one does not:

```
ENTRY-HISTORY: <slug> — departed at <commit> (<subject>); <N> commit(s) walked
back to its filing commit <commit>
```

**The departure is not a fall row, and the distinction is the existing
absence/zero rule.** §check-queue-entry-budget already rules that a bare slug
under the done section "is not an entry the cap measures and reads as absence" —
absence is not a count of zero, so the disposition cannot be reported as a
decrease from N to 0 without inventing a measurement. It is a header fact. The
row grammar keeps its four fields, there is still no verdict column, and the
arm still issues no verdict: whether a disposition was a Done move, a demotion,
an eviction or a discard is read off the commit by the reader who opens it.

**Two honest limits, both inherited one scale up.** A fall is attributed to a
slug, so an entry renamed mid-history already read as filed at its rename; under
this delta it also reads as *departed* at its rename, and the same holds for an
entry that spent a stretch outside the task sections. And the retired set is
whatever history the clone holds, so a shallow clone or a rewritten history
addresses fewer departed slugs and never more — the refusal under-claims, in the
direction §The queue-edges arm already declares.

No new arm, no new flag and no new spelling: the domain widens under the
existing argv tail.

### (5) `queue-counts` partitions by a named tag

`--emit queue-counts` emits one `<section-name><TAB><count>` line per task
section. A session wanting the pool's share carrying a tag — every live entry's
`[design-pending]`, the deferred pool's `[cost:]` census — counts lead lines by
hand. The arm gains `--by <tag>`. **design-bearing.**

```
--emit queue-counts [--by <tag>] [queue-file]
  default:      one "<section><TAB><count>" line per task section
  --by <tag>:   one "<section>/<value><TAB><count>" line per partition
```

**One output grammar, which is the test the refusal actually states.**
§The queue-counts arm's "why a second arm rather than a fourth mode" refusal
turns on one arm carrying two output grammars, and §The queue-index arm's
no-fourth-mode refusal turns on the same test. This flag changes the
**partition key** and nothing else: every line is `<key><TAB><count>`, the key
compounding as `<section>/<value>` on the precedent `[roadmap: <horizon>/<track>]`
sets in this kit for a pair whose halves are never independently meaningful. The
job — the size of each set — is one job under both keys.

The value rule, so an absent input appears rather than vanishing (§The
queue-index arm's own rule for `(undated)` and `(unclassed)`):

- a **field** tag (`[<name>: <value>]`) yields its first value on the lead line;
- a **bare** tag (`[<name>]`) yields the tag's own name;
- an entry carrying neither yields `(none)`.

Sections keep their configured order; values sort by first appearance in queue
order, the order every reader of this file walks. `<tag>` is taken as written
and matched against the lead line — the arm enumerates no tag name, so §The tag
algebra stays the one owner of the vocabulary and an unknown tag honestly
reports every entry under `(none)`.

**The in-process caller is unaffected, by construction.** delegation-kit's
statusline arm calls `emit(&[])` with no argv (`native/src/hook/statusline.rs`),
which is the default partition, byte-identical to today. The flag is parsed in
the arm's argv and the default rendering is untouched, so
delegation-kit/SPEC.md §The statusline arm's description of what comes back
stays true of everything that arm receives.

## Producers and consumers

**Producer (all five deltas)** — a session invoking the arm through
`bash gate-sdk/bin/run-gates.sh --emit <arm>`, plus delegation-kit's statusline
arm calling `queue_counts::emit` in process at each statusline fire. No delta
adds a trigger, a schedule, a timer or an invocation point; every one changes
what an existing invocation prints. No delta's behaviour is reachable only under
a configuration no deployed tree sets: deltas 1, 3 and 4 need a git work tree
(the degradation is declared), and deltas 2 and 5 need nothing.

**No name is minted on a rostered surface.** No delta adds an arm, so
gate-sdk/SPEC.md §The non-gate arm's `--emit-` roster is unchanged and is not an
update target; `--by` is a flag inside an existing arm's argv tail, the
mechanism §The queue-index arm's three modes already use. No delta adds a gate,
so `scripts/gates.list`, the `# graph:` manifests and the generated pre-commit
hook are unchanged. No delta adds a knob, so no knob roster moves — delta 2
records the refusal to mint one.

**Consumers, by delta.** The probe behind this roster and the one below is
`grep -rln "queue-counts\|queue_counts\|queue-edges\|queue_edges\|entry-history\|entry_history\|icebox-candidates\|CAUSE_CAP" --include=*.md --include=*.rs --include=*.sh --include=*.list --include=*.gate --include=*.knobs .`
over the tracked tree at rev `b2bcd457`, less the generated `docs/` mirror. It
is a floor the merging session re-derives.

- **Delta 1** — the retired block's two named readers, both stage templates that
  run the arm and read its output: scope at its ranking survey
  (`lifecycle-kit/templates/stages/scope.md`) and close at its gap-inbox drain
  (`lifecycle-kit/templates/stages/close.md`). Both hold instructions about how
  to read a retired row, and scope's holds the rule this delta mechanizes, so
  both are update targets. The `name live at <path>` field's reader is the
  session reading the row: the path is opened, or its existence alone settles
  that the citation is current.
- **Delta 2** — the close session reading the eviction worklist
  (`.claude/commands/close.md` §Backlog eviction routes it there, and
  `context-kit/SPEC.md` routes the tier's eviction work to the same command).
  Checked: neither describes the cause rendering, so neither is an update
  target. Each tail field's reader is that session, deciding whether the row's
  standing declaration still holds.
- **Delta 3** — two readers, which is the whole ground for the move:
  `native/src/emit/queue_edges.rs` and, after delta 4,
  `native/src/emit/entry_history.rs`. No prose surface names the function.
- **Delta 4** — the session assertion A's cap has just blocked, routed there by
  the failure's own help text (`native/src/gates/queue_entry_budget.rs`), and
  the drain that ranks the deferred pool. Checked: the help text names the
  command and not its domain, so it is not an update target. The new header
  fields' readers are that session — the departure commit is opened, its subject
  decides whether opening it is worth it.
- **Delta 5** — delegation-kit's statusline arm
  (`native/src/hook/statusline.rs`, contract at delegation-kit/SPEC.md §The
  statusline arm), which receives the unchanged default; and a session invoking
  the arm at the command `queue-kit/README.md` documents. Each partition row's
  reader is a scope or close session sizing the pool by a tag.

**No delta narrows a corpus** (causal-completeness point 5): deltas 1 and 2 add
a field to a row that still prints, delta 3 moves a derivation without changing
its result, delta 4 widens an addressable domain, and delta 5 adds a flag whose
default is byte-identical. No reader of any changed surface reds on finding
none, on an exact count, or on a coverage floor — none of these arms is a gate,
and none of them has a red condition at all.

**Delta 2's corpus is enumerable and every member is named** (point 6): the four
ineligibility causes, each with its fixed prefix and its variable tail, in the
table under that delta. No member is left without a satisfying value.

## Existing sections updated

Replacement text below is a **proposal** — build lands it. Each passage is
marked **Not yet applied**. Every bullet opens with its **backticked path**, the
convention canon-kit/SPEC.md §check-amendment-retired-spelling reads a roster by,
so a path appears as a leading token even where one file takes several bullets.

- `queue-kit/SPEC.md` — §The queue-edges arm (deltas 1 and 3) — the retired
  block's rendering gains the name-live mark and its derivation clause; the
  section's sentence siting the retired set in this arm is rewritten to site it
  in the shared adapter, keeping the body-citation scan where it is.

  **Not yet applied.** The paragraph opening "**The retired set is derived from
  the file's own history, so nothing is maintained.**" is rewritten to:

  > **The retired set is derived from the file's own history, so nothing is
  > maintained**, and it lives in the shared adapter because two arms read it —
  > this one and `entry-history` (§check-queue-entry-budget), on
  > §The shared queue adapters' two-reader rule rather than by convention. One
  > `git log -p --format= -- <queue-file>` pass at start-up, its added, removed
  > and context lines matched against the lead-line grammar that module owns —
  > the same one the live reader applies, through the same shared adapter, never
  > a second spelling.

  **Not yet applied.** After the paragraph ruling that retired targets sort
  alphabetically, this is added:

  > **A retired slug and a retired name are different questions, and the block
  > answers both.** The history test says an *entry* is gone; it says nothing
  > about the name, and where the name shipped as live mechanism the citations
  > of it are current prose rather than pointers at disposed work. A target
  > whose slug is the basename, less its final extension, of a file the tracked
  > tree carries renders as `<slug> (<N> inbound, retired — name live at
  > <path>)`, `<path>` being the first such file in tracked order with a further
  > match noted as `(+<M>)`. **The input is the tree listing and not a curated
  > roster**, on the ground §The tag algebra already recorded for the
  > `[surface:]` value set: a registry of names is a consumer file this kit
  > would couple to for a set the tree yields. The test **under-claims** — a
  > live name that is no file's own stem is unmarked, and that row reads as it
  > did — which is the same direction the two history degradations above
  > declare. It adds evidence and filters nothing: the no-red posture and the
  > report-don't-rule division of labour are untouched.

- `queue-kit/SPEC.md` — §The queue-index arm (delta 2) — the cause cap sentence
  is rewritten to bind the variable tail, and the `[standing]` rendering loses
  its redundant token.

  **Not yet applied.** In the `--icebox-candidates` paragraph, "a
  `not-icebox-eligible:` declaration (§The tag algebra), printed as `[standing]
  not-icebox-eligible <date> — <grounds>`, a line with no date as `(undated)`
  and one with no grounds as `(ungrounded)`" becomes "a `not-icebox-eligible:`
  declaration (§The tag algebra), printed as `[standing] <date> — <grounds>` —
  the declaration's own lead token is dropped because the `✗` mark and the
  `[standing]` class token both already say it, and the four class tokens read
  uniformly as a result; a line with no date renders `(undated)` and one with no
  grounds `(ungrounded)`". The sentence "Every cause prints under one fixed
  character cap, so a long grounds clause shows its head, and the self-naming
  line is one anchored grep away." becomes:

  > Every cause prints its **class prefix whole** and caps only its **variable
  > tail**, so a long grounds clause or a long slug shows its head while the
  > class that decides the row is never the thing cut. Two of the four causes
  > have no variable part and can never be cut at all. The cap is a module
  > constant rather than a knob: `QUEUE_KIT_ATTEND_CAP` budgets tokens on the
  > always-loaded brief, and this worklist is read once at close and budgets
  > nothing standing. A tail that is cut is one anchored grep on the row's own
  > slug away from the line that wrote it.

- `queue-kit/SPEC.md` — §The shared queue adapters (delta 3) — the paragraph
  siting the body-citation scan outside this module gains the retired-set
  derivation as a member admitted by the same rule.

  **Not yet applied.** The paragraph opening "The in-body citation scan (§The
  tag algebra) is deliberately **not** here" keeps its subject and gains:

  > The **retired-set derivation** is the same rule's other verdict. It began in
  > the queue-edges arm with one reader and joined this module when
  > `entry-history` became a second (§check-queue-entry-budget) — a history walk
  > carrying its own spelling of what an entry is would let two arms disagree
  > about which slugs ever existed, which is the drift the lead-line grammar is
  > already here to remove. It takes `git` with it: this module's readers
  > degrade to an empty retired set exactly as §The queue-edges arm declares,
  > and no other adapter here gains a program requirement.

- `queue-kit/SPEC.md` — §check-queue-entry-budget (delta 4) — the
  `--emit entry-history <slug>` clauses gain the departed-entry domain, its
  bound, the departure header and the widened rename limit.

  **Not yet applied.** In "**It issues no verdict...**", the exit-contract
  sentence's parenthesis "(an absent slug, or a slug the newest commit carries
  no entry for)" becomes "(an absent slug, or a slug that is neither live nor
  retired — refused before any history is read)". The paragraph "**The walk is
  bounded, and the bound is stated.**" gains:

  > A slug the newest commit carries no entry for is **not** an error: it is a
  > **departed** entry, and the arm reports it. The addressable domain is the
  > live slugs plus the retired ones — the set §The queue-edges arm derives and
  > §The shared queue adapters now holds — and that membership *is* the bound a
  > back-search would otherwise lack, settled from one history pass before a
  > blob is read. For a departed slug the walk searches back to the newest
  > commit carrying a counted entry; the commit after that one in log order is
  > the **departure commit**, named with its subject in the report's header, and
  > the fall rows then run from the last live commit back to the filing commit
  > unchanged. The departure is a header fact rather than a fall row, because
  > absence is not a count of zero — the same rule that already reads a bare
  > done-section slug as absence — and reporting it as a decrease would invent a
  > measurement. The row grammar keeps its four fields and still carries no
  > verdict column: which disposition a departure was is read off the commit.

  The "**Two honest limits**" paragraph's first limit is rewritten to: "A fall
  is attributed to a **slug**, so an entry renamed mid-history reads as filed at
  its rename and as departed at it — as does one that spent a stretch outside
  the task sections. A shallow or rewritten history addresses fewer departed
  slugs and never more, so the refusal under-claims in the direction
  §The queue-edges arm already declares for its own derivation."

- `queue-kit/SPEC.md` — §The queue-counts arm (delta 5) — the "no flags, no
  modes, one output grammar" sentence is rewritten so the invariant it protects
  (one grammar) survives and the description it also made (no flags) does not.

  **Not yet applied.** The synopsis line becomes
  `run-gates.sh --emit queue-counts [--by <tag>] [<queue-file>]`, and "One job:
  the size of each **task section** ... and nothing else — no flags, no modes,
  one output grammar." becomes:

  > One job: **the size of each set**, for a caller that wants the shape of the
  > queue rather than its contents. It emits one `<key><TAB><count>` line and
  > nothing else — **one output grammar**, which is the property the refusal
  > below protects and which `--by` keeps. The key is the task-section name, in
  > configured order; under `--by <tag>` it is `<section>/<value>`, the
  > slash-joined compound `[roadmap: <horizon>/<track>]` already uses in this
  > kit for a pair whose halves are never independently meaningful. A field tag
  > yields its first lead-line value, a bare tag yields its own name, and an
  > entry carrying neither yields `(none)` — an absent input appears rather than
  > vanishing, as §The queue-index arm's `(undated)` and `(unclassed)` do.
  > Values order by first appearance in queue order. The arm enumerates no tag
  > name, so §The tag algebra stays the vocabulary's one owner and an unknown
  > tag honestly reports every entry under `(none)`.

  The "**Why a second arm rather than a fourth mode**" paragraph gains a closing
  sentence: "`--by` is not that fold: it changes the partition key, leaving one
  job and one grammar, and the in-process caller
  (delegation-kit/SPEC.md §The statusline arm) receives the unchanged default."

- `lifecycle-kit/templates/stages/scope.md` (delta 1) — the ranking-survey
  paragraph instructs the reader to tell a retired entry from a retired name "by
  resolving the name in the tree rather than in the queue". The arm now does
  that resolution, so the instruction becomes a restatement and is rewritten to
  route to the mark and to state what the mark does not reach.

  **Not yet applied.** "**One class in that block is not a disposed premise, and
  discounting it is the opposite error:** where a retired slug's *name* shipped
  as live mechanism — a check, a knob, a script the tree now carries — its
  inbound edges cite that mechanism and are current, however large the block
  says the target is. Tell the two apart by resolving the name in the tree
  rather than in the queue." becomes:

  > **One class in that block is not a disposed premise, and discounting it is
  > the opposite error:** where a retired slug's *name* shipped as live
  > mechanism, its inbound edges cite that mechanism and are current, however
  > large the block says the target is. The arm marks the row it can prove —
  > `name live at <path>` — so read the mark rather than re-deriving it. The
  > mark reaches a name that is a tracked file's own stem and no further, so an
  > unmarked target with many inbound edges is still worth one resolution in the
  > tree before it is read as disposed.

- `lifecycle-kit/templates/stages/close.md` (delta 1) — the gap-inbox drain
  step sends close to correct every retired citation inline; a marked row needs
  no correction, and the step says so.

  **Not yet applied.** In the retired-block step, after "It is a finding, never a
  violation: the correction is inline on the citing entry, in this session, the
  way every attested instance was fixed.", this is added:

  > A row the arm marks `name live at <path>` is **not** in that finding set:
  > its citations point at mechanism the tree still carries and are correct as
  > written. Skip it, and spend the step on the rows that are left.

- `queue-kit/README.md` (deltas 1, 4 and 5) — the command roster's
  `--emit queue-counts` line carries the output grammar in a trailing comment
  and gains `[--by <tag>]`; the one-line arm descriptions gain the retired
  block's mark and `entry-history`'s departed-entry domain. Not transcribed here:
  the file is a command roster, and the delta text above is what it points at.

- `docs/queue-kit/SPEC.md` (all deltas) — the generated public mirror of
  `queue-kit/SPEC.md`, regenerated rather than edited; its trigger, regen command
  and freshness gate are rostered in docs/site-architecture.md §Generated
  projections and their freshness gates.

- `docs/queue-kit/README.md` (all deltas) — the generated public mirror of
  `queue-kit/README.md`, regenerated on the same roster entry.

- `native/src/emit/queue_edges.rs` (deltas 1 and 3) — the name-live mark and the
  rendering that carries it, and the call site left behind by the derivation's
  move.

- `native/src/emit/queue_index.rs` (delta 2) — `ineligibility` returns a cause
  split into its fixed prefix and its variable tail, and only the tail reaches
  `cap_chars`; `standing` drops the declaration's lead token from the row.

- `native/src/queue.rs` (delta 3) — the retired-set derivation lands here, with
  the degradations it carries.

- `native/src/emit/entry_history.rs` (delta 4) — the widened domain, the
  back-search to the last live commit, the departure header, and the refusal that
  now fires before any history is read.

- `native/src/emit/queue_counts.rs` (delta 5) — the `--by` flag in the arm's
  argv, the compound key, and the value rule; `emit(&[])` stays byte-identical.

- `queue-kit/gate-tests/queue-edges.test.sh` (deltas 1 and 3) — a marked and an
  unmarked retired target in the seeded sandbox.

- `queue-kit/gate-tests/queue-index.test.sh` (delta 2) — a standing cause whose
  grounds survive the cap, and the shortened prefix.

- `queue-kit/gate-tests/entry-history.test.sh` (delta 4) — a departed slug
  reported with its departure commit, and a slug in neither set refused without a
  walk.

- `queue-kit/gate-tests/queue-counts.test.sh` (delta 5) — a `--by` partition over
  a field tag, a bare tag and an entry carrying neither, beside the unchanged
  default.

## Retired spellings

- `[standing] not-icebox-eligible <date> — <grounds>` — the eviction worklist's
  standing-cause rendering; the declaration's own lead token is dropped from the
  printed row, which becomes `[standing] <date> — <grounds>` (delta 2). The
  declaration's grammar in the queue body is untouched — `not-icebox-eligible:`
  is still what an entry writes and what a reader greps for; only the row's echo
  of it retires.
- `the arm reads a live entry` — `entry-history`'s refusal message for a slug
  the newest commit carries no entry for; the condition it named is no longer a
  refusal, and the message retires with it (delta 4).

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The
      causal-completeness check holds for each new state, event, interface and
      obligation: the name-live mark, the tail-capped cause, the promoted
      derivation, the departed-entry domain and header, and the `--by`
      partition.
- [ ] **Instruction surfaces: instruction only** — the two stage-template
      replacements carry instruction and no grounds; the grounds are in deltas 1
      and in queue-kit/SPEC.md.
- [ ] **Merged with no information lost** — each replacement above re-phrases
      the passage it refines rather than appending to it; the merged SPEC reads
      as one document to a reader who never saw this amendment.
- [ ] **Amendment deleted** — this file removed on merge, with the four queue
      entries moved in the same commit; `ls queue-kit/SPEC-*.md` empty at the
      iteration.
- [ ] **Removals propagated** — the three declared spellings above re-run
      against the whole tracked tree by `check-amendment-retired-spelling`.
- [ ] **Release declaration appended** — the behaviour changes to four shipped
      arms declared under `## Behavior changes` in `.workflow/release-declarations.md`
      by the session landing them (gate-sdk/SPEC.md §upgrade-smoke).
- [ ] **Battery and fixtures green** — `bash gate-sdk/bin/run-gates.sh`,
      `bash gate-sdk/bin/build-native.sh`, and queue-kit's fixture suite; neither
      discharges the other.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed
      through the gap inbox (a build-time causal gap is resolved that session).
