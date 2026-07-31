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

Measured at authoring: the literal appears **161×** across **23 files**; **98×**
in `TASK-QUEUE.md` (58 lead-line tags — one per deferred entry — and 40 in entry
prose, mostly the recurring `**Why [needs-spec]:**` field idiom); the rest across
two gate bodies, three kit SPECs, two READMEs, two templates, one smoke fixture,
five gate-test inputs, the generated docs mirror, and one hand-authored site
page. No variant spelling exists. No `expect.txt` in the tree contains it.

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
cross-check *total*: every entry in the pool carries it, so any entry in an
active section carrying it is a botched promotion, with no exempt population to
hide in. Narrowing buys honesty the rename already delivers and pays for it in
coverage.

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
literal prefix match — and removes the metacharacter-escaping hazard as a side
effect. `enum-sets.sh` retargets its derivation from the `arr["<tag>"]` rows to
this table. That derivation gets **more** robust, not less: the current grep can
partially parse (some rows renamed, some not) and silently emit a short set,
whereas a single `split()` line either matches or does not, and the existing
`[[ ${#alltags[@]} -gt 0 ]] || exit 2` guard fails closed on the latter.

### 5. `check-amendment-queue`: token swap, guard clauses verbatim — *mechanical*

Thirteen occurrences in `canon-kit/checks/check-amendment-queue.sh` — three awk
regexes, two classifier labels, two `case` labels, four operator-facing rejection
or help strings. Every assertion, negation, and section scope is unchanged; only
the literal moves. Assertion (a)'s prose arm keeps its guidance verbatim apart
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
token swap in the body's `**Why …:**` field idiom, the sibling's cost-field
triage, and — where the sibling's judgment evicts the entry — the compression to
a one-line icebox entry, which discards the body the token swap would otherwise
have edited. **Eviction therefore settles before body edits**, or the pass edits
prose it is about to delete.

Ordering with the sibling amendment is a build-batch concern, not a contract:
whichever lands first, the other's sweep must run over the *result*, and a second
full-file rewrite is the failure this pairing exists to prevent.

### 7. Occurrences that name the old token *as* the old token do not migrate — *design-bearing*

A mechanical sweep gets this wrong, so it is a stated rule with a named
population: **a mention whose subject is the historical token is preserved
verbatim; only live-grammar occurrences migrate.** Renaming these would falsify
a record rather than update a grammar.

- `canon-kit/SPEC.md:560` — a frozen drift attestation that "names the set as it
  stood when the drift occurred", already carrying a `prose-enum-exempt` escape.
  Left exactly as it is.
- This unit's own queue entry, including its `[needs-spec]` appears **98×`
  measurement, which is a dated observation of the pre-rename state and is the
  rename's own provenance.

Every other occurrence is live grammar and migrates. The three existing
`prose-enum-exempt` escapes (`canon-kit/SPEC.md:325`, `:560`,
`queue-kit/SPEC.md:530`) are carried through unchanged in placement — the
migration must not silently drop an escape while rewriting the line under it.

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
- `lifecycle-kit/bin/enter-stage.sh:140` — the gap-drain help string.
- `docs/queue-kit/index.md:15` — hand-authored, **not** generated and not
  content-gated; it survives only because it enumerates all five tags and is in
  `CANON_KIT_MANIFEST_FILES`, so `check-prose-enum` reaches it. Named explicitly
  because the docs-mirror regeneration does not touch it.
- The generated docs mirror (17 occurrences across `docs/{canon,queue,lifecycle}-kit/`)
  is **regenerated, never hand-edited** — `check-docs-mirror-fresh` byte-compares
  it and fails loudly.

### 10. Release-note declaration — *mechanical*

The token is a governed grammar name a vendored consumer's queue carries, so the
release note declares it under **Renamed knobs** as `[needs-spec] → [design-pending]`,
per the note structure docs/install.md owns. This is a breaking-but-not-
decommissioning change; §Ruled out records why it takes no deprecation window,
and that ruling is the escalated one.

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
design-pending tag>` and now names the token), §check-tag-lead-line (the governed
tag enumeration at :435-436, the bare-prose "masks a needs-spec state" at :429,
and a sentence stating the class table as the single source both the matcher and
`enum-sets.sh` read), §check-queue-slug-liveness-adjacent prose at :314, and the
`prose-enum-exempt` block at :530-531.

canon-kit/SPEC.md — §The amendment lifecycle (the **Design-pending** state
bullet at :59-61 and the bidirectional-rule paragraph at :72),
§check-amendment-queue (assertions (a) and (b) at :398-402), the content-tiering
marker roster at :326-327 (the token, keeping its `prose-enum-exempt` escape),
and :187, :211, :214. **:560-561 is deliberately not updated** (delta 7).

lifecycle-kit/SPEC.md — :494 (close's gap-drain disposition) and :1041 (the
deferred-filing model).

README.md:89 and queue-kit/README.md:5 — the enumerating kit-roster rows.

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
- **A deprecation window or compat shim for the old token.** *This ruling is
  escalated to the lead and is not settled here.* The mechanical facts: the tree
  has **no** alias, shim, or dual-resolution convention; the one deprecation gate
  (`check-deprecation-task`) binds `task: <slug>` markers on a *source-comment*
  surface via `CANON_KIT_DEPRECATION_MARKERS`, a roster this repo does not set —
  so it structurally does not reach a queue-tag literal, and honoring it would
  mean inventing the roster, the marker surface, and the dual-parse together.
  Against that, lifecycle-kit/SPEC.md:239-244 rules that "from the first tag
  onward a rename owes the queue-bound deprecation mechanism and a
  tightened-gates/release-note declaration", and the line is at v0.17.0. The
  clause's own stated rationale, however, is "because no external consumer can
  have vendored the kit yet (the first tag is a launch-comms prerequisite)" —
  a premise the tag was a *proxy* for and which has drifted from it, since 17
  tags have shipped with no launch and no observed external install. Resolving
  that proxy is an envelope question neither queue entry settles.
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
