# SPEC amendment: needs-spec-tag-rename

The design-pending tag's *token* asserts something its own definition does not.
`spec` is feature-bound everywhere in the system — `[spec:]` pairs with an
authored amendment, the `/spec` stage authors amendments, and
`check-amendment-queue` makes `[spec:]` feature-only — but `[needs-spec]` is
mandatory on **every** entry in a design-pending section, and a deferred **debt**
entry promotes with the tag *deleted, never converted*. For that fraction the
token names an artifact that will never exist. This renames the token to
`[design-pending]` — the spelling its own owner doc already uses — preserving
every guard clause verbatim, and closes the one silent-failure path a token
rename opens.

Measured at authoring (baseline `7657141`): the literal appears **161×** across
**23 files**; **98×** in `TASK-QUEUE.md` (58 lead-line tags — one per deferred
entry — and 40 in entry prose, mostly a recurring `Why …` field idiom in several
spellings, delta 6); the rest across two gate bodies, three kit SPECs, two
READMEs, two templates, one smoke fixture, five gate-test inputs, the generated
docs mirror, and one hand-authored site page. No variant spelling of the token
itself exists. No `expect.txt` in the tree contains it — verified across all 172.

Re-measured at align: **166× across 25 files**, 90 in `TASK-QUEUE.md` (56 lead
lines). The census is stale, not wrong — the drift is this iteration's own two
promotions out of Deferred plus the two sibling amendment files, which is
precisely the population delta 7 governs.

## Seam ruling

**Kit mechanism:** the token spelling, in both kits. queue-kit/SPEC.md §The tag
algebra opens with the rule that settles this — tags are "square-bracket literals
with **fixed spelling (mechanism, not config** — the one exception is the
consumer-named harvest vocabulary)", and canon-kit/SPEC.md:327 names this marker
among the spellings that "are mechanism, not config". **No knob is minted.** A
configurable design-pending token would contradict the algebra's own opening
clause and buy nothing: unlike the harvest tags, this token carries no consumer
vocabulary — it names a lifecycle state the kit itself defines.

**Consumer config:** none added. The sections the tag spans are already
configurable (`CANON_KIT_DEFERRED_SECTION`, and batch 1's
`CANON_KIT_ICEBOX_SECTION`); the tag rides them and needs no knob of its own.

**Private rule content:** none. `design-pending` is generic delivery-process
English and is already the public definition on the owner doc.

## What changes

### 1. The token — `[needs-spec]` → `[design-pending]` — *design-bearing*

The replacement is the **owner doc's own definition, adopted verbatim**:
queue-kit/SPEC.md:66 already defines the tag as a "design-pending marker", and
canon-kit/SPEC.md:59 already names the state it marks **Design-pending**. The
rename therefore closes a token-versus-definition contradiction rather than
minting a third name, and it costs zero definition churn.

**Why `[design-pending]` over `[triage-pending]` / `[unscoped]`** — the deciding
argument is that **the defect is artifact-implication, not stage-implication**.
`needs-spec` names an *artifact* (a spec, an amendment) that provably never comes
into existence for the debt fraction; that is a falsifiable claim the queue makes
58 times and breaks on every debt promotion. `design-pending` names a *state*,
and "design" in this system explicitly spans both authoring stages — the `spec`
stage template calls itself "the generative half of design", which presupposes
scope's triage is the other half. Every entry in a design-pending section is in
that state, debt included: what is pending for a deferred debt item is the ruling
on how to converge it, which is exactly what scope supplies. No entry has to be
mentally corrected for, which was the whole complaint.

`[triage-pending]` is marginally more literal — scope's triage is the single
event that resolves every entry — and is **ruled out on second-order cost**: it
would force renaming the definition at queue-kit/SPEC.md:66, the state name at
canon-kit/SPEC.md:59, and the "design-pending section set" vocabulary the sibling
`deferred-queue-carry-cost` amendment lands this same iteration. That recreates
the token-versus-definition mismatch this unit exists to close, one iteration
after closing it. `[unscoped]` is rejected for reading as a queue-hygiene defect
("this entry lacks scope") rather than a lifecycle state.

### 2. The tag keeps its section-wide meaning — *design-bearing*

The queue entry left open "whether to narrow the tag's *meaning* to match a
renamed token or keep it section-wide under a truer name". **Keep it
section-wide.** Narrowing it to feature-only — mirroring `[spec:]` — would drop
the forbidden-in-active guard for every debt entry and break
mandatory-on-every-entry, and those two clauses together are what make the
cross-check reach every *promotion*: every entry in the pool carries it, so any
entry in an active section carrying it is a botched promotion. Narrowing buys
honesty the rename already delivers and pays for it in coverage.

**The check is not total, and the amendment does not claim it is.** Corrected at
align, because the overstatement was load-bearing prose rather than colour:
`check-amendment-queue` classifies any heading that is not a feature, active, or
deferred section as `other` and skips every arm, so **`## Done` is an exempt
population** — an entry moved Deferred → Done still carrying the tag reds
nowhere, and `check-tag-lead-line` misses it too, since `QUEUE_TASK_RE` spans
active plus deferred only. In practice a Done entry is a bare slug line
(`queue_done_slugs` matches nothing else), so the tag is dropped by that grammar
rather than by a gate. The guard is total over the promotion moves it exists to
catch — Deferred → an active section — and silent on the disposition move. That
is a pre-existing shape the rename neither creates nor closes; it is stated so
the decision to keep the tag section-wide rests on the coverage it actually has.

This is also what keeps the unit coherent with its sibling: batch 1 generalized
canon-kit's **section set** (deferred plus icebox) rather than its tag set, and
the tag stays exactly co-extensive with that set. The migration scope is
therefore stated once, precisely: **every entry in a design-pending section** —
not "every Deferred entry".

### 3. The tag is a checksum on the promotion move — *design-bearing*

Recording the reasoning, because the rename invites an objection that looks fatal
and is not. `[design-pending]` on every entry of a *design-pending section* reads
like the two-sources defect batch 1 cited when it refused to mint an `[icebox]`
tag ("section membership is the state; a tag restating its own section is the
two-sources defect").

It is not the same shape. **The tag is a checksum on a state transition, not a
second source of state.** Its redundancy is the mechanism: a promotion moves an
entry across a section boundary *and* must swap its tag, so
forbidden-in-active catches a move that dropped the swap, and `[spec:]`-in-
design-pending catches a swap that dropped the move. `[icebox]` stays refused
under exactly this test — Deferred → Icebox is not a promotion, spans no
pending/ready boundary, and so a third tag value would add a state name without
adding a caught error class.

The rename changes the checksum's *spelling* and nothing about its algebra. This
is not a decommission.

### 4. `check-tag-lead-line`: derive the class key from the matched literal — *design-bearing*

This is the enforcement half of the rename, and it closes a real hole rather than
a hypothetical one. `classes()` currently states each tag twice on one line — as
a match regex and as the `arr[]` key:

```awk
if (line ~ /\[needs-spec\]/) arr["needs-spec"] = 1
```

and `scripts/enum-sets.sh:8-9` **machine-derives this repo's `queue-task-tag`
enum vocabulary by grepping those `arr["<tag>"]` keys out of the gate**, feeding
`check-prose-enum` over every manifest paragraph that enumerates the tags. The
two halves can desync, and the two directions are not symmetric:

- Rename the **regex** only → the enum set stays stale, prose migrates,
  `check-prose-enum` reds. Caught, loudly.
- Rename the **key** only → the enum set and all prose agree, everything is
  green, and the gate's regex still matches a token no queue entry carries. The
  lead-line guard for the design-pending tag is **silently dead**. Nothing
  catches this.

Per enforcement-first — *removing the duplication outranks gating it* — the fix
is derivation, not a new assertion. `classes()` takes a single class table whose
entries carry the tag name and its bracket terminator (`]` for a bare tag, `:`
for a field tag), and both the match and the key come off that one token:

```awk
BEGIN { ncls = split("blocked-by: spec: design-pending] attend] drain-exempt: roadmap:", cls, " ") }
function classes(line, arr,   i, nm, term) {
    delete arr
    for (i = 1; i <= ncls; i++) {
        term = substr(cls[i], length(cls[i]))
        nm   = substr(cls[i], 1, length(cls[i]) - 1)
        if (index(line, "[" nm term)) arr[nm] = 1
    }
    for (i = 1; i <= nlt; i++)
        if (index(line, "[" lt[i] "]")) arr[lt[i]] = 1
}
```

`index()` on a literal replaces the regexes exactly — every current pattern is a
literal prefix match, verified arm by arm at align — and removes the
metacharacter-escaping hazard as a side effect. `enum-sets.sh` retargets its
derivation from the `arr["<tag>"]` rows to this table; the class table is the
only quoted-literal `split()` in the gate (the other takes an awk variable), so
it is unambiguously greppable. Two honest notes on that retarget:

- **The extraction trades one escaping hazard for another.** The current
  derivation is a single capture group; the retarget must split on space and
  strip a trailing `:` or `]`, and `]` is a bracket-expression metacharacter
  needing care in the class. Net still favourable, but not free.
- **The robustness argument holds in one direction, not both.** A missed table
  yields zero tags and the existing `[[ ${#alltags[@]} -gt 0 ]] || exit 2` guard
  fails closed — that is real. But the claim that the *current* grep "silently
  emits a short set" on a partial rename is wrong for the rename described:
  `arr["design-pending"]` still satisfies `[a-z][a-z-]*`, so a half-done key
  rename emits a full set with a wrong member, not a short one. The set only
  shortens if a rename leaves the charset. The two-direction desync above is the
  real defect; this is not a second one.

`enum-sets.sh:7`'s own `# spec:` line names ``the arr["<tag>"] rows
check-tag-lead-line keys on`` as the derivation surface. It is a one-line-binding
`spec:` comment under `check-comment-tier` and is rewritten with the retarget —
leaving it would leave the file citing a surface it no longer reads.

### 5. `check-amendment-queue`: token swap, guard clauses verbatim — *mechanical*

Thirteen occurrences in `canon-kit/checks/check-amendment-queue.sh` — three awk
regexes, two classifier labels, two `case` labels, four operator-facing rejection
or help strings, and **the two header comments at `:23` and `:26`** describing
assertions (a) and (b), which are under `check-comment-tier` governance and were
missing from this itemisation (it summed to eleven). Every assertion, negation,
and section scope is unchanged; only the literal moves. Assertion (a)'s prose arm keeps its guidance verbatim apart
from the token ("say \"needs design\" in prose"), because the advice was never
about the token's spelling.

The rejection strings are safe to rewrite: **no `expect.txt` in the tree matches
on this token** (verified tree-wide), including
`check-amendment-queue/bad/expect.txt`, which deliberately matches only
`misfiled in an active non-feature section`. That expect line must stay as it is
— it is what makes the bad fixture survive the rename, and "improving" it to name
the new token would couple a fixture to a literal for no gain.

### 6. The one-pass queue migration, composed with the sibling sweep — *design-bearing*

Both this unit's token migration and `deferred-queue-carry-cost`'s part (d)
triage sweep rewrite every entry in the design-pending pool. They are paired into
one iteration precisely so the queue is rewritten **once**, and this amendment
states the composition rather than leaving build to discover it:

**The sweeps compose into a single pass, entry by entry.** For each entry in a
design-pending section, one visit performs: the token swap on the lead line, the
token swap in the body's `Why …` field idiom, the sibling's cost-field triage,
and the sibling's **disposition** — evict to the icebox, rule wontfix, or keep.

**Disposition settles first, then body edits.** Both dispositions destroy prose
the token swap would otherwise have edited, so visiting them in the other order
edits text it is about to delete:

- **Evicted** → compressed to a one-line icebox entry; the body goes, the lead
  line is rewritten from scratch and simply *carries* the new token.
- **Ruled wontfix** → the entry is reduced to a bare `- <slug>` line under
  `## Done` (the only shape `queue_done_slugs` matches; anything richer reds
  `check-task-conservation` as a lost task). The lead line and its tag are
  **deleted, not swapped** — this population must not be visited by the token
  migration at all. The original composition omitted this disposition and named
  only eviction; it is the case where "swap the token" is affirmatively wrong.
- **Kept** → the full lead-line-plus-body swap, and the cost field triaged.

**The body idiom is not one spelling, and a sweep keyed on one misses most of
it.** `grep 'Why \[needs-spec\]'` returns **zero** hits — the token is backticked
inside the bold run. Live variants at authoring: ``**Why `[needs-spec]`:**``
(22), ``**Open design (why `[needs-spec]`, not a build unit):**``,
``**Open design (why `[needs-spec]`):**``, ``**Why `[needs-spec]` and not
closed:**``, ``**Design question (why [needs-spec], not a build unit):**`` and
``**Design question (why [needs-spec], and why the value is the open part):**``
— the last two *unbackticked*. The migration keys on the token, backticked or
bare, never on the surrounding idiom.

Ordering with the sibling amendment is a build-batch concern, not a contract:
whichever lands first, the other's sweep must run over the *result*, and a second
full-file rewrite is the failure this pairing exists to prevent. **This extends
to `canon-kit/SPEC.md §check-amendment-queue`, which both units rewrite** — the
sibling widens assertion (b) to a two-section read while this unit renames the
token that assertion keys on, so the second to land edits the merged text, not
the line numbers cited here.

### 7. Occurrences that name the old token *as* the old token do not migrate — *design-bearing*

A mechanical sweep gets this wrong, so it is a stated rule with a named
population: **a mention whose subject is the historical token is preserved
verbatim; only live-grammar occurrences migrate.** Renaming these would falsify
a record rather than update a grammar.

- `canon-kit/SPEC.md:560` — a frozen drift attestation that "names the set as it
  stood when the drift occurred", already carrying a `prose-enum-exempt` escape.
  Left exactly as it is.
- This unit's own queue entry, including its "`[needs-spec]` appears **98×**"
  measurement, which is a dated observation of the pre-rename state and is the
  rename's own provenance.
- **The unit slug `needs-spec-tag-rename` itself** — a fourth population,
  neither live tag grammar nor a frozen token attestation. It survives at
  `TASK-QUEUE.md:52`, `:60`, `:135` and
  `lifecycle-kit/SPEC-stages-taxonomy.md:266`. The slug is **not** renamed:
  `TASK-QUEUE.md:60` is a live lead line under `check-queue-slug-liveness` and
  `check-task-conservation`, so renaming it would owe a Done-move and a
  `[blocked-by:]` sweep for no gain. The DoD's "removals propagated" grep is
  read against this exclusion, which is why the population is named here rather
  than discovered when the grep comes back non-empty.

Every other occurrence is live grammar and migrates. The three existing
`prose-enum-exempt` escapes (`canon-kit/SPEC.md:325`, `:560`,
`queue-kit/SPEC.md:530`) are carried through unchanged in placement — the
migration must not silently drop an escape while rewriting the line under it.
**"Unchanged in placement" means the covered lines must not grow**: the escape's
reach is two lines (the comment's own line and the next), and
`needs-spec` → `design-pending` lengthens every occurrence by three characters,
so a reflow that pushes a tag onto a third line silently un-covers it. Preserving
the comment while letting the line wrap is the failure this clause forbids.

### 8. Fixtures: the uncovered class gets covered — *design-bearing*

The design-pending class has **no fixture exercising it** in
`check-tag-lead-line`'s pair today — the bad case trips on `[drain-exempt]` and
`[attend]`. That absence is what would let delta 4's silent direction ship
unnoticed under the old duplicated form, so the pair gains a case placing
`[design-pending]` on a continuation line whose lead line lacks it, with the
matching `expect.txt` rejection substring. This is owed by the fixture-pair
contract independently of the rename; the rename is what surfaced it.

Five gate-test input files carry the token as realistic queue input and migrate
mechanically (`check-amendment-queue/good/`, `check-todo-task-liveness/good/`,
`check-deprecation-task/good/`, `check-roadmap-fresh/{good,bad}/`), as does
`drift-kit/smoke/install.sh`'s inline queue heredoc — which is under no parity
gate and fails only at smoke-run time, so it is called out by name rather than
left to a grep.

### 9. Templates, projections, and registration — *mechanical*

- `queue-kit/templates/TASK-QUEUE.md:37` — the starter template's example
  deferred entry. Under `check-template-copy-parity`; the tag stays trailing on
  the lead line, which keeps `check-tag-lead-line` satisfied.
- `lifecycle-kit/templates/skills/close.md:22,:65` — the gap-drain prose. The
  `.claude/commands/close.md` shim does **not** carry the token, so no shim edit.
  **Path dependency:** the sibling `templates-stages-taxonomy-realignment` unit
  moves this file to `lifecycle-kit/templates/stages/close.md` in the same
  iteration. If that unit lands first the file is at the new path; a `git mv`
  carries this edit along if it lands second. Either order works, neither is
  assumed — the build batch resolves the path at edit time rather than trusting
  the spelling here.
- `queue-kit/SPEC-queue-carry-cost.md` — the sibling amendment's own body carries
  the token where it restates `check-amendment-queue` assertion (b). That is live
  grammar destined to merge into a canonical spec, not a historical mention, so
  it migrates with everything else. Align abstracted it at its source; it is
  named here because a tree-wide sweep that skips `SPEC-*.md` on the assumption
  that amendments are scratch would miss it, and `check-kit-ref-liveness` valves
  that path class.
- `lifecycle-kit/bin/enter-stage.sh:140` — the gap-drain help string.
- `docs/queue-kit/index.md:15` — hand-authored, **not** generated and not
  content-gated; it survives only because it enumerates all five tags and is in
  `CANON_KIT_MANIFEST_FILES`, so `check-prose-enum` reaches it. Named explicitly
  because the docs-mirror regeneration does not touch it.
- The generated docs mirror (17 occurrences across `docs/{canon,queue,lifecycle}-kit/`)
  is **regenerated, never hand-edited** — `check-docs-mirror-fresh` byte-compares
  it and fails loudly.

### 10. Release-note declaration — the only compat obligation — *mechanical*

The token is a governed grammar name a vendored consumer's queue carries. It
declares in **two** sections of the one shared note, because the two sections
answer different questions and this rename owes both an answer.

**Behavior changes** — `[needs-spec] → [design-pending]`, the bullet a human
upgrader reconciles by rewriting their queue's tags. **Not Renamed knobs.**
Ruled at align, and derivable rather than judged: docs/install.md scopes that
section to config twice — `:416-419` lists the residue class as "knob renames in
**your own config**", and `:421-423` maps "**own-config** knob renames and
removals → Renamed knobs". A queue tag lives in the consumer's `TASK-QUEUE.md`,
which is content, not own-config, so it lands in the "behavior your tree depends
on" class the third section takes. The section is named "Renamed *knobs*", not
"Renamed things"; the sibling path-move unit reaches the same section by the
same reading.

**Tightened gates** — `check-amendment-queue`, because it **reds on a clean
upgrade**. Verified at align at the read site rather than assumed: a consumer
whose Deferred entries still carry `[needs-spec]` trips the `deferred-open` arm
(`:50-52`) on **every** such entry once the gate matches the new token, and the
gate exits 1 (`:111`). That is precisely the allowed-red set those lead tokens
are read as, mechanically, by a consumer's tooling — so the bullet is owed *in
addition to* the Behavior-changes one, never instead of it. A rename that can
turn a gate red owes both, and omitting the mechanical half is the failure that
matters, because that is the half a machine acts on.

**One gate changes and must *not* be listed: `check-tag-lead-line`.** Its class
table stops recognizing `[needs-spec]`, so on an unmigrated consumer queue the
lead-line guard silently **stops firing** rather than reddening. A stop-firing
gate is not an allowed-red and would corrupt the set if listed — but it is a real
consumer hazard, and it is exactly the silent direction delta 4's derivation
closes on the kit side. The Behavior-changes bullet names it, since nothing
mechanical will.

With no deprecation window owed (§Ruled out), **the note is the entire compat
obligation this rename carries** — which raises its weight rather than lowering
it. It is the only artifact that will tell a consumer their queue's tags must be
rewritten, and only the Tightened-gates bullet will tell their tooling which
gate may go red saying so: the migration happens in the consumer's tree, which
`upgrade-smoke`'s phase A never touches.

The release class is a **minor**. docs/install.md:349-351's pre-1.0 qualifier
rides a non-decommission break on a minor while the line is 0.x, and :341-345
reserves major for a decommission — a release that *removes* a deprecated
surface, which this is not. :338 files even a knob rename *carrying* a
deprecation path under Minor, so there is no reading on which a rename with no
window earns more. `check-release-bump` independently floors the note off a
patch: it ORs the three section counts (`:97`), so the floor holds on the
Tightened-gates and Behavior-changes bullets above and never depended on the
Renamed-knobs placement.

**Renamed knobs reads "None." for this release, with a routing clause, not a
denial.** docs/install.md:412-415 sanctions a trailing clause exactly where it
rules out a near-miss a reader would otherwise mis-classify — and a note
shipping two renames whose Renamed-knobs section reads "None." is that
near-miss. But :415 forbids a clause that only restates the heading's own
negation, so it must *route*: name that this release's renames are
consumer-content and copied-out-template residue and point at Behavior changes.
"None. — no own-config knob was renamed" is the deletable kind.

## Producers and consumers

- **The `[design-pending]` token.** Producers: any session filing into a
  design-pending section (scope's triage, close's gap drain via
  `lifecycle-kit/bin/file-gap.sh`, an operator-directed filing), plus the starter
  template's example entry. Consumers: `check-amendment-queue` assertions (a) and
  (b); `check-tag-lead-line`'s class table; `bin/queue-index.sh`'s tag rendering;
  and every reading session.
- **`check-tag-lead-line`'s class table.** Producer: the gate body, one literal
  per class. Consumers: `classes()` itself (both the match and the `arr[]` key),
  and `scripts/enum-sets.sh` → the `queue-task-tag` enum set → `check-prose-enum`
  → the four enumerating manifest paragraphs (`README.md:89`,
  `queue-kit/README.md:5`, `queue-kit/SPEC.md:435-436`, `docs/queue-kit/index.md:15`).
- **The renamed rejection strings.** Producer: `check-amendment-queue`. Consumer:
  the committing session. No `expect.txt` reads them, which is what makes the
  rewrite safe.

No new state, field, or interface is introduced — the unit renames one existing
literal and removes one duplication. The causal-completeness obligation is
therefore discharged by naming, for the one changed interface (the class table),
its producer and both its readers, which delta 4 does.

## Existing sections updated

queue-kit/SPEC.md — §The tag algebra (the tag bullet at :66 and the body-field
idiom at :31-32, which already writes the field abstractly as `Why <the
design-pending tag>` and now names the token — deltas 1 and 6),
§check-tag-lead-line (the governed tag enumeration at :435-436, the bare-prose
"masks a needs-spec state" at :429, and a sentence stating the class table as the
single source both the matcher and `enum-sets.sh` read — delta 4),
§check-queue-slug-liveness-adjacent prose at :314 (delta 1), and the
`prose-enum-exempt` block at :530-531 (delta 7).

canon-kit/SPEC.md — §The amendment lifecycle (the **Design-pending** state
bullet at :59-61 and the bidirectional-rule paragraph at :72 — deltas 1 and 2),
§check-amendment-queue (assertions (a) and (b) at :398-402, plus the `## Done`
coverage limit — deltas 2 and 5), the content-tiering marker roster at :326-327
(the token, keeping its `prose-enum-exempt` escape — delta 7), and :187, :211,
:214 (delta 1). **:560-561 is deliberately not updated** (delta 7). Line numbers
are pre-merge: the sibling unit rewrites assertion (b) in this same section, so
the second unit to land re-locates them (delta 6).

lifecycle-kit/SPEC.md — :494 (close's gap-drain disposition) and :1041 (the
deferred-filing model) — delta 1.

README.md:89 and queue-kit/README.md:5 — the enumerating kit-roster rows
(delta 1; both also name `[precondition-ok:]`, which is outside the class table
and the enum set and does not move).

`scripts/enum-sets.sh` — the derivation at :8-9 and its `# spec:` line at :7
(delta 4).

`queue-kit/templates/TASK-QUEUE.md`, `lifecycle-kit/templates/skills/close.md`,
`lifecycle-kit/bin/enter-stage.sh`, `drift-kit/smoke/install.sh`, the five
gate-test inputs, and `docs/queue-kit/index.md` — per delta 9.

**Deliberately not updated: `CLAUDE.md`.** It names the gap-inbox and
scope-gated-intake mechanisms without spelling the tag, so the rename does not
reach it — and adding a line naming the token would be resident weight for every
session that never files.

## Ruled out

- **A configurable token (`QUEUE_KIT_DESIGN_PENDING_TAG`).** The tag algebra's
  opening clause makes fixed spelling the rule and names its single exception
  (the consumer-named harvest vocabulary, which exists because sink names are
  private rule content). This token names a kit-defined lifecycle state; a knob
  would let two consumers spell one mechanism two ways and would put a second
  source under `enum-sets.sh`'s derivation for nothing.
- **Narrowing the tag to feature-only.** Delta 2 — it costs the totality that
  makes forbidden-in-active a complete check.
- **`[triage-pending]` and `[unscoped]`.** Delta 1.
- **A deprecation window or compat shim for the old token.** Ruled 2026-07-31 at
  spec: **no window is owed, because no clause reaches this rename.**
  lifecycle-kit/SPEC.md:239-244 is the nearest-looking rule and it does not
  apply — it is headed *"Knob-rename compat precedent"*, sits inside §Layout and
  configuration immediately above the `LIFECYCLE_KIT_*` roster it governs, and is
  cited nowhere else in the tree. Its second sentence's bare "a rename" is loose
  drafting inside a knob-scoped clause, not a wider grant. This unit renames a
  queue-tag token, not a knob. **This is a scoping finding, not an exception
  taken** — nothing is waived, and a later reader should not re-litigate it as
  one.

  The mechanical facts are recorded because they would have blocked compliance
  even had the clause reached: the tree has **no** alias, shim, or
  dual-resolution convention anywhere, and the one deprecation gate
  (`check-deprecation-task`) binds `task: <slug>` markers on a *source-comment*
  surface via `CANON_KIT_DEPRECATION_MARKERS` — a roster this repo does not set,
  on which the gate no-ops. It structurally does not reach a queue-tag literal.
  A window would have meant inventing the roster, the marker surface, and the
  dual-parse together, mid-iteration.

  **Filed, not fixed here:** that clause's "before the first release tag"
  threshold is a proxy for "nobody has vendored this" which has come loose from
  its premise — 17 tags have shipped with no launch and no observed external
  install — and it will misfire on the next real *knob* rename. Amending a
  governing clause is a new initiative and goes through scope-gated intake, so
  it rides the gap inbox for close to triage rather than this unit.
- **Retitling the entry's `Why [needs-spec]:` body field to a new fixed name.**
  The four deferred body fields are explicitly ungated conventions that
  queue-kit/SPEC.md already writes abstractly; the sweep swaps the token inside
  the idiom and mints no new governed field name.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls canon-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles. The one-and-only surviving `needs-spec` mentions
      are delta 7's frozen attestations.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **Migration landed with the derivation** — the queue, both gates, the
      fixtures, and `enum-sets.sh` are green at the same commit; no `--no-verify`.
