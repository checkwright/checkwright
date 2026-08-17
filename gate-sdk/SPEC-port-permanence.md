# SPEC amendment: port-permanence

A gate declares **whether it will ever port**, in one optional header line on its
shell declaration path, and `port-blockers.sh` stops counting the members that
never will.

This is arm **(3)** of `port-remainder-permanent-shell-inflation`, ruled by the
lead: *a declarable field on the descriptor makes both other arms derivable*.
Arm (1) — three hardcoded names in the tool — and arm (2) — a reported split
needing the same input — are refused by that ruling and not re-argued here. What
this file settles is everything the parenthetical `permanent|blocked|candidate`
left open: **where the field lives, how many values it has, what reads it, and
what holds it honest**.

**The defect has two consequences and the second is the one that costs.** The
inflation makes the port's progress metric asymptote at three instead of zero.
It also **mis-selects the cohort**: `--group`'s only non-singleton group pairs
`check-readme-roster` with `check-install-disposition`, whose permanence the tool
cannot see, so the size arm reads takeable when it is exhausted. That
adjudication is sanctioned (§The first cohort, and the rule that selects the next
— *an advisory group is a finding the selecting session adjudicates*) and is
re-bought at every one of the ~32 cuts still owed.

**Measured at authoring, from the oracle rather than from the entry's prose.**
`bash gate-sdk/bin/port-blockers.sh --group` at this rev reports
`104 member(s) scanned, 34 group(s) formed, 0 undecidable, 69 already ported and
excluded`. Three of the 35 remaining are permanently shell, so 32 are genuinely
owed, and exactly one of the three sits in a non-singleton group.

## What changes

### 1. A `# no-port:` field is minted, and its domain is the **shell** declaration path

`# no-port: <cause>`, one optional line in a gate's header block beside
`# graph:`, `# install:` and `# spec:`, declaring that the gate is never going to
the binary substrate and naming the ruling that makes it so — **design-bearing**.

**The field's domain is the `<name>.sh` spelling of the declaration path, and a
`.gate` descriptor never carries it.** A descriptor's *existence is the dispatch
declaration* (§The `# graph:` manifest) — a member that has one is ported, so it
has no port question left to declare, and a `# no-port:` line there would be a
field asserting the negation of the file it sits in. This is what keeps the
descriptor's **closed field roster** closed: the roster gains nothing, reserves
nothing, and the refusal that closed it is untouched.

**The spelling is `no-port:` rather than the entry's sketched `port:`, and the
ground is measured rather than aesthetic.** `check-comment-tier`'s colon
directives match as an **unanchored substring** — `body.contains(c)` in the gate's
own matcher — so a roster entry `port:` silently blesses every comment carrying
`support:`, `transport:`, `export:`, `import:` or `report:`. The collision class
is live rather than hypothetical: this tree already carries
`# E — anti-double-report: a relative target …`, which a `port:` entry would bless
today. The bare word `port` in the *anchored* word roster was weighed and is
worse here — it would bless any comment using the standalone word "port", in a
repository whose central work is porting gates. `no-port:` occurs inside no
English word, and it takes the shape of the descriptor's existing `# no-fixture:`
opt-out, which is the nearest precedent in the same header family.

**That refusal is answered by kind rather than skirted, because it applies to
`# reads:` and `# needs:` and a later reader will ask why not to this one.** Both
were refused on one ground — *nothing would hold it to the implementation, so it
is a self-declaration whose would-be reader could not verify what it read*. Both
are claims about **runtime behavior**: the implementation can contradict them,
which is why the crate's registry plus a unit test that runs the member is a
strictly better home. `# no-port:` is a claim about a **design ruling**. It has no
runtime referent at all — no execution could falsify it — so "hold it to the
implementation" names nothing that could be done for it in any location. What it
can be held to is the ruling it cites, and the exception criterion already
requires that ruling to exist in the gate's own SPEC section (§The port-candidate
criteria, *the cause is recorded in the gate's own SPEC section*). The field is a
**pointer to a recorded ruling**, and delta 6 holds the pointer's shape.

**The `<cause>` half is the whole payload, which is what makes this one field
rather than a field with a vocabulary.** The cause names the section that rules
the member permanent, so a reader who finds the line reaches the argument in one
hop rather than searching for it.

### 2. There is **no value vocabulary**, and absence is the other state

The field's presence *is* the verdict — *this member the port does not take* —
and **absence means still owed**, which is every other member —
**design-bearing**, and it is the delta the entry's `permanent|blocked|candidate`
parenthetical is narrowed by.

**`candidate` is refused because declaring the default is the cost the whole
shape was deferred on.** This SPEC already weighed a per-gate
`# substrate: shell — <cause>` header *asserted over every shell member* and
deferred it: *"it demands one retrospective declaration per unported member,
sixty against this tree at the flip, each deleted again as its member ports. It
becomes cheap when the residue is small, which is the condition to revisit it
under."* A `candidate` value reinstates exactly that — 32 declarations that each
say nothing, each deleted at its port. Default-by-absence buys the same
derivation for three lines.

**And the two shapes must not be confused, which is why the passage above is
cited rather than quietly reused, and why the field is not simply named
`# substrate:`.** That one enforces the **born-native default** — *may this gate
land in shell at all?* — over a corpus of newly authored gates. This one reports
the **port remainder** — *will this gate ever leave shell?* — over the corpus of
existing ones. Same shape, different subject, different corpus, different reader.
Borrowing its name would collapse a distinction this delta exists to draw.
**`# no-port:` does not discharge born-native enforcement**, and a later session
must not read a green `check-gate-substrate-parity` as having landed it; that
disposition stays filed where it already is.

**With no value there is no vocabulary to close**, which is a simplification the
`no-fixture:` precedent already demonstrates: an optional opt-out named for what
it opts out of carries a reason, not an enum. A *held* member is a different
declaration rather than a second value of this one — delta 3.

### 3. `until <slug>` is designed, refused for this unit, and filed — not left blank

The entry's `blocked` value has an honest spelling, a working precedent and a
named reader, and **none of that makes it landable here** — **design-bearing**,
because the refusal is the ruling and a later session that re-proposes it from
scratch will re-buy the design.

**The honest spelling is a second field, `# port-until: <slug>`.** `blocked`
collides with a word this tool already uses for something *derived* — the default
arm's criterion-7 verdict — and a declared field wearing a derived term is the
drift that follows. `until` is this tree's existing word for a temporary
disposition: `check-gate-exemption-tasks` pairs `# until: <live-slug>` with
`# permanent: <reason>` per exemption-list element, on exactly this
permanent-versus-held axis — two annotations rather than one annotation with two
values, which is the shape delta 2's no-vocabulary ruling makes this inherit.

**That precedent also kills the objection the entry raises.** Class (b) is
*temporary by construction* and class (c) *expires when the substrate lands*, so
a held declaration looks like it must rot into an under-count. It does not: the
precedent holds `until:`'s slug to a **live** queue entry, so when the blocker
lands and the slug moves to `## Done` the gate reds and the declaration must be
dropped. The value has a real reader too — §The port-candidate criteria records
that a class-(c) cause states *"a blocker and its owning entry that
`port-blockers.sh` can derive but not attribute"*, which is this value's job
description.

**It is refused anyway, on the closed-roster rule.** The descriptor's field
roster carries *no field that lacks a named reader, reserving nothing against a
future reader*, and this field would ship with **zero holders**: the members that
would declare `until` live on `cohort-held-members-port-prerequisites`' roster,
and retrofitting them is that entry's work rather than this one's. A value minted
empty is a reservation however good its design. **Filed costed**, per the
gap-disposition rule, against that entry — which is where the holders are.

### 4. `port-blockers.sh --group` excludes declared-permanent members and counts them

A member whose shell declaration carries `# no-port:` **leaves the
partition entirely and is counted in the trailing line**, on precisely the terms
an already-ported member does — **design-bearing**, and this is the delta that
makes arms (1) and (2) derivable at once.

The trailer gains one clause:

```
port-blockers --group: <n> member(s) scanned, <n> group(s) formed, <n> undecidable, <n> already ported and excluded, <n> permanently shell and excluded
```

**Arm (1) is the exclusion and arm (2) is the trailer, from one input.** The
remaining count a reader derives — scanned minus ported minus permanent minus
undecidable — is *still owed* rather than *still shell*, which is the number the
port track has wanted since it started. Nothing is hardcoded and nothing is
maintained: the tool learns of a new permanent member the day that member's
declaration says so.

**The exclusion reuses the ported-member branch rather than adding a second
one.** Both answer the same question — *is this member in the corpus this arm
exists to order?* — and §port-blockers already rules the divergence from the
default arm's `?` treatment for the ported case: *the grouping exists to order
the remaining corpus, and a ported member is not in it*. Neither is a permanent
one.

### 5. The default arm is **unchanged**, and that is a ruling rather than an omission

A permanent member keeps its criterion-7 row — **design-bearing**, because the
symmetry the shape of delta 4 invites is wrong here.

`check-crate-arms` prints `c7=cargo`, and that row **is** part of the evidence
for its own permanence (§The port-candidate criteria, criteria 4 and 7). Excluding
it would delete the finding that grounds the declaration, leaving a reader with a
verdict and no oracle behind it. The default arm answers *what external programs
does this rule require* — a true and useful fact about a rule whatever its port
future — while `--group` answers *what should the next cohort take*, which is the
only question permanence bears on.

### 6. `check-gate-substrate-parity` gains **assertion G — port-declaration placement**

Two clauses over the declaration set the gate already derives — **design-bearing**:

- **A `.gate` descriptor carries no `# no-port:` line.** Delta 1's placement rule,
  and the one failure the mechanism can actually produce: a port that lands a
  descriptor from a declaration carrying the field, forward-copying it with the
  `# graph:`/`# install:`/`# spec:` lines that *are* copied verbatim.
- **A `# no-port:` line on a shell declaration carries a non-empty cause, and a
  declaration carries at most one.** The same shape
  `check-install-disposition` already holds for `# install:` — *exactly one line
  where a gate declares one* — reached here with *at most*, since the field is
  optional.

**It folds into this gate rather than shipping as its own**, on the precedent
that section states for its own assertion E: *"the descriptor set it reads is the
one this gate already derives, and a separate gate would duplicate that
derivation to add nothing."* Assertion A resolves every member to exactly one
declaration and knows which spelling it got, which is the entire input. The
subject fits too — this gate holds *the dispatch seam honest*, and where a
declaration field may live by spelling is that seam's own partition, the
question assertion D answers for the implementation-versus-declaration axis.

**The gate is permanently shell (exception class (a)), so no substrate question
arises from widening it**, and this delta adds no member to the conservation
table: the corpus is the declaration set it already walks.

### 7. Presence is **not** asserted, and the asymmetry that makes that safe is stated

No assertion demands that a permanently-shell member declare — **design-bearing**,
because the omission looks like a hole and is not one.

Permanence is a ruling in prose, and a gate that derived which members hold one
would have to parse SPEC argument text. What makes the gap safe is the direction
of the error: an **undeclared** permanent member is counted as owed, which is
today's inflation for that one member and nothing worse, while the field can only
ever *shrink* the reported remainder. The mechanism therefore fails toward the
status quo rather than toward the under-count the entry's class-(b) paragraph
warns about. Enforcement-first is not waived — it ranks a gate above discipline
*where a gate is available*, and here the available gate is delta 6's shape
assertion, which is what ships.

### 8. `# no-port:` joins `check-comment-tier`'s built-in directive roster

**This is the cross-component half, and without it the three declarations of
delta 9 red on the commit that lands them** — **mechanical**.

The roster is the crate's `SHELL_COLON` table in canon-kit's `check-comment-tier`
module, plus its prose restatement in canon-kit/SPEC.md's machine-directive
invariant; the two are hand-held rather than derived (probed:
`scripts/enum-sets.sh` carries no comment-tier set), so **both move**. `# no-port:`
belongs in the **built-in** roster and not behind a `CANON_KIT_COMMENT_*`
consumer knob: that roster is *the directive names the kits themselves carry*,
and this one is a gate-sdk kit mechanism.

### 8a. The seam: mechanism ships, the verdicts do not

**The kit ships the field, the reader and the assertion; it ships **no** roster of
permanent members and no cause text** — **design-bearing**, because the shape most
likely to cross the seam here is the convenient one.

A kit literal naming *this* tree's three permanently-shell gates would publish one
project's work queue as everyone's mechanism, which is the defect §The first
cohort already guards against for the batch arm — *"it is stated generically — no
gate names, no member roster, no count of any tree's remaining corpus"* — and
which the `check-graph` rule-content split exists to prevent. The remainder is
**derived from declarations in the consumer's own tree** at every read, so a
consumer with no permanent member reads a field that never fires and a consumer
with ten declares ten.

**The `<cause>` is consumer content by construction.** It is free text pointing
at whatever surface that consumer records the ruling on; the kit constrains only
that it be non-empty. Nothing in gate-sdk parses a cause, matches it against a
vocabulary, or knows what a section reference looks like — which is what keeps
this field from becoming the coupling-vocabulary shape the seam forbids.

**And no knob is minted.** The field needs none: both readers already resolve the
declaration path through `gate_resolve` under `gate_kit_roots`, which is the
configuration this tool and this gate each resolve today, so the delta adds no
`<KIT>_<KNOB>` and no default to be unset anywhere.

### 9. The three permanent members declare

`check-install-disposition` and `check-gate-substrate-parity` cite the exception
class that rules them permanent; `check-crate-arms` cites criteria 4 and 7,
**not** that class, which its own SPEC section is emphatic about —
**mechanical**.

The roster is not written into this SPEC and is not written here as a durable
list: it is the members whose SPEC sections already carry a permanence ruling,
found by reading those sections, and after this unit it is derivable by grepping
the field. Three at this rev, and the count is a measurement rather than a
fixture.

### 10. `§The `# graph:` manifest`'s field-roster paragraph is corrected while it is open

It enumerates the descriptor's closed roster as `# graph:`, `# spec:` and
`# no-fixture:` — and **`# install:` is a fourth field on both spellings**, ruled
at §The install disposition (*"and in a `.gate` descriptor on the same terms"*),
recorded in the conservation table's own `check-install-disposition` row, and
present on every live descriptor — **mechanical**.

The paragraph is the exact prose delta 1 edits, and its closed-roster claim is
the argument delta 1 rests on, so a reader who checks that claim against the tree
must not find it false. Verified against `canon-kit/checks/check-comment-tier.gate`
rather than assumed.

### 11. The fixture pair is widened, and the binary is rebuilt before commit

`check-gate-substrate-parity`'s `good/`+`bad/` pair gains cases for both of
assertion G's clauses — a descriptor carrying `# no-port:`, and a shell declaration
carrying a bad value and an empty cause. `check-comment-tier`'s pair gains a
`# no-port:` line, since delta 8 changes a compiled table. Then
`bash gate-sdk/bin/build-native.sh` **and** `bash gate-sdk/bin/run-gates.sh`,
neither discharging the other — **mechanical**.

## Producers and consumers

### The `# no-port:` field

- **Producer:** a hand-authored header line on a gate's `<name>.sh` declaration,
  written by the session that lands the member's permanence ruling. **Enabling
  config actually emitted:** none — the field needs no knob. It is read off the
  declaration path both readers already resolve through `gate_resolve` under
  `gate_kit_roots`, the configuration the tool and the gate each resolve today.
  Not test-only: delta 9 lands three live declarations in this repo's own tree,
  and both readers run in the live battery.
- **Consumers, named with the mechanism, surveyed across the whole component
  set:**
  - `gate-sdk/bin/port-blockers.sh` `--group` arm, by header grep at the point it
    already tests the declaration's spelling — the member leaves the partition
    and increments the trailer's new count (delta 4).
  - `check-gate-substrate-parity` assertion G, by header grep over the
    declaration set assertion A derives — placement and shape (delta 6).
  - `check-comment-tier`, by directive-roster match, on every shell declaration
    and every `.gate` in the comment surface — it must **bless** the line or the
    declaration reds (delta 8). This is a consumer that gains no assertion and
    must still be changed, which is exactly the reader a component-local survey
    would have missed.
  - **`check-install-disposition` is surveyed and is not a consumer**: its parse
    is anchored to `^# install:` specifically, so a sibling header line is
    invisible to it. Named because the shape invites the assumption.
  - **`scripts/measured-claims.sh` is surveyed and is not a consumer**: it counts
    `ported-gate-members` by resolving each registry member and testing for
    `*.gate` itself, never by reading either arm's trailer. Delta 4 therefore
    moves no measured claim and reddens no marked sentence. Probed rather than
    inferred from the queue entry, whose parenthetical describes the trailer as
    that emitter's source.
  - **No other machine reader exists.** §port-blockers states both arms are
    *advisory, never parsed*; the trailer's new clause is read by the session
    cutting a cohort, at the cut.
- **Every field has a named reader.** The line has exactly two parts, and delta
  2 is what reduced it to two. The **field's presence** is read by
  `port-blockers --group` at the partition decision and by assertion G's
  placement clause; the **cause** is read by assertion G's non-empty clause and
  by the human at the cut, who follows it to the SPEC section that rules the
  member permanent. There is no third part: a value slot would have had no reader
  the presence does not already serve, and delta 3 refuses the one value that
  would have had no holder at all.

### Assertion G's two clauses

- **Producer:** `check-gate-substrate-parity.sh`, on every full-battery run and
  on every commit touching its coupled surfaces — the manifest it already has,
  unchanged, since the corpus is the declaration set it already walks.
- **Consumer:** the committing session, through the gate's finding lines and the
  pre-commit hook.
- **Red condition, named rather than its subject:** clause one reds on a `.gate`
  descriptor carrying a `# no-port:` line; clause two reds on a shell declaration
  whose `# no-port:` line has a value outside `{permanent}`, an empty cause, or more
  than one such line. A declaration with **no** `# no-port:` line is clean under
  both — delta 2's absence-is-a-state rule, and delta 7's deliberate
  non-assertion of presence.

### This delta set does **not** narrow a corpus, and that is checked rather than assumed

Causal-completeness point 5 binds on a delta that narrows a corpus — a prune, a
tighter glob, a dropped file. Nothing here drops a file, tightens a glob or
prunes a walk: three header lines are **added**, one gate gains an assertion, one
directive roster **grows** by an entry, and the `--group` arm's partition shrinks
by three members while its `scanned` count is unchanged. The one reader whose
verdict is non-monotone in an *added* line is `check-comment-tier`, and it is
delta 8's whole subject: an unblessed `# no-port:` line is a **new violation**, which
is why the roster entry and the declarations must land in one unit rather than in
sequence.

## Existing sections updated

- **gate-sdk/SPEC.md §The `# graph:` manifest** — delta 1's placement rule (the
  field's domain is the shell spelling; the descriptor's roster is untouched and
  why the `# reads:`/`# needs:` refusal does not reach it) and delta 10's
  correction of the roster enumeration to name `# install:`.
- **gate-sdk/SPEC.md §port-blockers** — delta 4's exclusion and trailer clause,
  and delta 5's ruling that the default arm is unchanged. The section's own
  standing caveat applies unedited: adding an arm once changed no byte of the
  default arm's output, and *that is a fact about that change, not a standing
  guarantee* — this delta moves the `--group` trailer deliberately.
- **gate-sdk/SPEC.md §check-gate-substrate-parity** — delta 6's assertion G, and
  the assertion count in its lead paragraph, which reads "Six assertions."
- **gate-sdk/SPEC.md §The port-candidate criteria** — delta 2's boundary against
  the `# substrate: shell — <cause>` shape weighed there, so the deferred
  born-native enforcement is not read as landed; and delta 3's refusal recorded
  where the class-(b)/(c) causes it would have carried are ruled.
- **canon-kit/SPEC.md §check-comment-tier** — delta 8's roster entry in the
  machine-directive invariant.
- **The three members' own SPEC sections** — §check-install-disposition,
  §check-gate-substrate-parity, §check-crate-arms — each already states the
  permanence ruling delta 9's cause cites, so none gains an argument; each is
  read to confirm the citation resolves before the header line is written.
- **`cohort-held-members-port-prerequisites`** — delta 3's filed value, costed
  against the roster that would hold it.

## Definition of Done

- [ ] **Causal completeness** — the field's two readers and one must-be-changed
      non-reader are named above with their mechanisms; both parts of the line
      have a named reader at a named transition; assertion G's red condition is
      named rather than its subject; the two surveyed non-consumers were probed,
      not inferred.
- [ ] **Merged with no information lost** — each delta lands in the canonical
      section named above, integrated rather than appended; delta 10's correction
      lands with delta 1 rather than as a separate note.
- [ ] **Amendment deleted** — this file removed on merge; `ls gate-sdk/SPEC-*.md`
      checked at the iteration horizon, since a sibling gate-sdk amendment is in
      flight for the same component this iteration.
- [ ] **Removals propagated** — nothing is retired by this unit; the grep that
      proves it is over `# no-port:` before the unit lands, confirming the spelling
      is unused in the tree (`# permanent:` is a *different* directive, on
      exemption-list elements, and is untouched). Stderr not silenced.
- [ ] **Gaps filed** — delta 3's `until <slug>` value filed costed through
      `bash lifecycle-kit/bin/file-gap.sh`; any build-time causal gap resolved
      that session rather than deferred.
- [ ] **Terminal move** — `port-remainder-permanent-shell-inflation`'s
      deliverable is complete in one unit, so its entry moves to `## Done` rather
      than demoting.
