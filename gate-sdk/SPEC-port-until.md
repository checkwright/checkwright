# SPEC amendment: port-until

Mints `# port-until: <slug>` — the declarable spelling for a **temporarily
held** port, designed at §The port-candidate criteria and refused there on the
closed-roster rule for want of holders. This amendment supplies the holders and
lands the field, its four readers and its assertions.

It does **not** discharge `check-tree-terms`' criterion-4 hold, the other work
`cohort-held-members-port-prerequisites` owes. That entry's deliverable is a
corpus and this amendment delivers one increment of it, so its terminal move is
a demotion rather than a Done move (canon-kit/SPEC.md §Merging an amendment,
the entry-outlives-the-amendment branch).

## What changes

### 1. The field

`# port-until: <slug>` is a second optional header field on a gate's
**`<name>.sh` declaration spelling alone**, beside `# graph:`, `# install:`,
`# spec:` and `# no-port:`. It declares that the member is **still owed to the
port and not takeable now**, and names the live queue entry that owns the
blocker. *design-bearing.*

The payload is a **bare slug and nothing else** — no `<cause>` half. `# no-port:`
carries free text because permanence is a *ruling* whose only home is prose; a
temporary hold's home is a **queue entry**, which already carries the ground, the
cost and the disposition, so a cause half here would be a second place the same
ground lives and the one that rots when the entry is re-scoped. Both fields cost
their reader exactly one hop; the refusal is recorded so a later author does not
re-propose the half as an ergonomic. *design-bearing.*

No value vocabulary and no knob, on `# no-port:`'s own grounds (§The `# graph:`
manifest): presence is the verdict, and both readers already resolve the
declaration path through `gate_resolve` under `gate_kit_roots`. *mechanical.*

**Its domain is wider than the two classes the design named, and the widening is
ruled here rather than absorbed silently.** §The port-candidate criteria drafts the
field for a *class-(b)/(c)* hold, both of which are born-native exception classes —
that is, statements about a gate being **authored** in shell. The day-one roster is
not shaped that way: four holders are criterion-7 blockers (a program the payload
does not carry, which is class (c) read over an existing gate), and one is a
criterion-4 hold, which needs **design** rather than substrate and fits no lettered
class at all. So the field's domain is **any temporary hold with named work owed and
a live owning entry**, of which classes (b) and (c) are the born-native instances.
Narrowing it to the letters instead would leave the roster's clearest member —
`check-tree-terms`, the one the design paragraph itself points at — unable to
declare, which is the reading that fails on its own worked example. *design-bearing.*

**The seam.** The kit ships the field, its readers and its assertions — never a
roster of held members and never a slug. Slugs are consumer content by
construction: a consumer with no held member reads a field that never fires, and
one with five declares five, out of that consumer's own tree. This is `# no-port:`'s
seam ruling applied unchanged, and it is what keeps this project's port queue out
of a kit literal (CLAUDE.md §The provenance seam). *design-bearing.*

### 2. The arithmetic — the delta a reader is most likely to get wrong

§port-blockers today documents a derivation in prose: *scanned minus ported minus
permanent minus undecidable* is **still owed**. A fourth exclusion class silently
falsifies that sentence for every existing reader, because a held member is still
owed — it is temporarily untakeable, which is the whole difference between class
(b)/(c) and `# no-port:`. So the arm gains a **held** clause in its trailer and
**prints both numbers rather than leaving the reader to subtract**: *still owed*
(held members included) and *takeable at this cut* (held members excluded).
*design-bearing.*

A held member **leaves the partition** on `# no-port:`'s terms — the grouping
orders the corpus this arm can take now, and a member whose blocker is unlanded is
not in it. That is the whole of the value: a selector's candidate list stops
carrying members it must re-adjudicate by hand at every cut. *design-bearing.*

The failure direction is stated because it differs from `# no-port:`'s. An
**undeclared** hold is counted takeable — the status quo, and a selector's own
audit catches it. A **stale** declaration (the blocker landed, the slug moved to
Done) would under-count takeable members and hide real work; that is what the
liveness reader in delta 4 exists for, and it is why this field, unlike
`# no-port:`, cannot rest on a shape assertion alone. *design-bearing.*

### 3. The shape reader — `check-gate-substrate-parity` assertion G

Assertion G already owns *where a declaration field may live by spelling*. It
gains the field on identical terms plus one clause the pair of fields creates:

- **A `.gate` descriptor carries no `# port-until:` line** — a descriptor's
  existence is the dispatch declaration, so a ported member has no port question
  left to declare. Same failure mode as `# no-port:`'s: a port forward-copying the
  field with the lines that *are* copied verbatim. *mechanical* (the clause is a
  second instance of one that ships).
- **A `# port-until:` line on a shell declaration carries a non-empty slug, and a
  declaration carries at most one.** *mechanical.*
- **No declaration carries both `# no-port:` and `# port-until:`** — the new
  clause, and the one neither field owns alone: permanent and temporarily-held are
  contradictory verdicts about the same member, and a declaration asserting both
  makes the trailer's two exclusion counts overlap. *design-bearing.*

**Presence is still not asserted**, on assertion G's own recorded ground: a gate
deriving which members hold a blocker would have to parse SPEC argument text. The
error direction stays the safe one. *mechanical.*

### 4. The liveness reader — `check-gate-exemption-tasks`, not a sixth holder

The slug must resolve to a **live** queue entry or the field rots into an
under-count. The reader is **`check-gate-exemption-tasks`**, whose invariant
widens from *`# exception-list:` array elements* to *every temporary-disposition
annotation a gate declaration carries*. *design-bearing.*

The ground is enforcement-first plus the cost that gate's own SPEC section already
prices. It holds the live-section span, the bullet-lead-line predicate and the
queue-file coupling **today**, and its corpus is already the declaration set this
field lives in — the same `gate_check_dirs` walk over each check dir's `*.sh` and
`*.gate`, with `TASK-QUEUE.md` already in its `couples=`. Landing the liveness in
assertion G instead would make `check-gate-substrate-parity` a **sixth**
independent holder of the lead-line predicate that section prices at five and names
as a standing cost, and would add a queue-file coupling to a gate that
deliberately has none — it audits the dispatch seam. Zero new holders is the
ruling. *design-bearing.*

**The rival's argument is recorded, because it is a good one and it lost on cost
rather than on shape.** Today that gate reads *per-element trailing comments inside
`# exception-list:`-tagged arrays*; a top-of-file header field is a different
syntactic subject, and on shape alone assertion G — which already reads header
fields on this exact declaration set — is the closer fit. The widening is taken
anyway because the two annotations make **one claim about one queue**, so the
gate's invariant generalizes cleanly to *every temporary-disposition annotation a
gate declaration carries*, while the shape-fit rival buys a sixth parser of a
format five copies already disagree about silently. What build owes is the new
**trigger**: the gate fires today on the `# exception-list:` marker alone, so the
header-field arm is a second entry into the same walk rather than a widened regex.
*design-bearing.*

The split between the two readers is the one the fields already draw: **shape and
placement** are assertion G's (a field's spelling-domain is the dispatch seam's own
partition), **slug liveness** is the exemption gate's (`# until: <slug>` is the
same claim about the same queue). *design-bearing.*

**The `# until:` / `# port-until:` spelling collision is deliberate and is the
precedent being cited, not an accident to rename.** §The port-candidate criteria
chose `until` over `blocked` precisely because it is this tree's existing word for
a temporary disposition. The two annotations differ in subject (an array element
versus a whole member) and in prefix, and the widened gate reads them with one
liveness predicate — which is the saving. A reader who greps `until:` finds both,
which is correct: they are the same claim. *design-bearing.*

### 5. The comment-tier roster

`port-until:` joins canon-kit's blessed comment-prefix roster in **both** holders
— the prose roster in canon-kit/SPEC.md and `native/src/gates/comment_tier.rs` —
or `check-comment-tier` reds every declaration that adopts the field. This is what
makes the amendment cross-component. *mechanical.*

### 6. The holders

The field lands **with** its declarations, in the same unit. A field minted empty
is the reservation the closed-roster rule refuses, and shipping the readers a cut
before the holders would reproduce that refusal exactly. *design-bearing.*

**The roster was surveyed at spec rather than assumed, and it is non-empty.** Five
still-shell members are held on a named, temporary blocker: `check-tree-terms`
(criterion 4 — its corpus is the whole tracked tree, so every declaration path lies
inside what it scans as content), `check-action-run-shell` and `check-shellcheck`
(`shellcheck` on `PATH`), `check-gate-assertions` (`paste`), and
`check-docs-render-fidelity` (the renderer `SITE_KIT_RENDERER`'s first element
names — a dependency spelled nowhere in the gate's source, which is why site-kit's
own SPEC records it). Membership is **derived at build** from the criterion-7
report plus each gate's SPEC ground, never copied from this list. *design-bearing.*

**Two members are excluded by name, because the mechanical reading admits them and
the rule does not.** `check-reads-couples` and `check-gate-binary-fresh` report
`c7=?`, and the `?` is `port-blockers.sh`'s tokenizer failing to resolve a
command-position variable naming **the crate's own binary** — not a substrate gap,
and no SPEC prose states a hold, a class or a ground for either. §The
port-candidate criteria already rules that *a session that reads `?` as a hold has
let the scanner's limit become the rule's*; declaring them would write that mistake
into a machine-read field. *design-bearing.*

**Three of the five holders owe a ground sentence before they may declare, not
one — audited at align.** `check-shellcheck`'s blocker is identical to
`check-action-run-shell`'s and the tool reports both, but neither gate's own SPEC
section grounds its own hold: §check-shellcheck states only that a `.gate` member
is outside its corpus, and §check-action-run-shell states nothing about its own
hold at all — both causes live solely in §The port-candidate criteria's
criterion-7 worked-example prose, which is the central section the own-section
rule exists to keep causes out of. `check-gate-assertions`'s `paste` dependency is
grounded the same way — named in that same worked-example prose and nowhere in
§check-gate-assertions. §The port-candidate criteria requires the cause in **the
gate's own SPEC section**, so build writes it there for all three before their
declarations land; a declaration whose ground is inferred from a sibling, or from
the criteria section's shared prose, is the one shape the field must not
normalise. `check-docs-render-fidelity` is the one holder that already clears this
bar: site-kit/SPEC.md records its `SITE_KIT_RENDERER` dependency beside the knob
itself. *design-bearing.*

**The slug's honest limit: the owner is an umbrella, and the anti-rot is coarse.**
Only `check-tree-terms` has a specifically owning entry. The other four resolve to
`cohort-held-members-port-prerequisites`, which by its own text claims exactly this
class — *gates are held on shell by operator ruling, each owing a named prerequisite
nothing else tracks* — so it is a live, correct slug for each. What it does not buy
is per-blocker rot detection: when the umbrella closes, every declaration reds at
once, and a member whose **own** blocker lands earlier keeps a live-but-wrong
declaration until then. That is strictly better than today's hand re-adjudication at
every cut and it is not the design's ceiling. The condition under which per-blocker
entries earn their filing is stated rather than left implicit: **when a blocker
acquires its own designed answer** — a shellcheck-free lint rule, a `paste`
replacement, a renderer check that needs no ruby — that answer is a unit, and the
declaration re-points at it. Filing three such entries speculatively now would be
three costed filings against work nobody has scoped. *design-bearing.*

**The mint fails closed on the roster.** If build finds no holder with a live owning
entry, the field is not minted and the unit reports that rather than landing a
reserved name — the closed-roster rule is what this amendment exists to satisfy,
not a formality it may discharge by assertion. *design-bearing.*

### 7. Fixtures

`check-gate-substrate-parity`'s `good/`+`bad/` pair gains cases for each new
clause — a descriptor carrying the field, a bare line with no slug, two lines on
one declaration, and one declaration carrying both fields — beside the
`# no-port:` cases it already holds. `check-gate-exemption-tasks`' pair gains a
header-field case in each direction: a declaration whose `# port-until:` slug is
live, and one whose slug is Done-only. Both gates are shell and stay shell.
*mechanical.*

## Producers and consumers

**New interface: the `# port-until: <slug>` header field.**

- **Producer** — a porting or authoring session writing the line into a
  `<kit>/checks/<name>.sh` declaration. Its enabling config is **nothing**: no
  knob is minted, both readers resolve the declaration path through `gate_resolve`
  under `gate_kit_roots`, which every consumer already has. The producer is
  reachable in a vendored tree, which is what the seam ruling requires.
- **Consumer 1 — `gate-sdk/bin/port-blockers.sh`, `--group` arm**, at the same
  transition its `# no-port:` skip runs: after `$decl` resolves and before the
  member is placed in a group, beside the `permanent_excluded` increment. It reads
  the field's **presence** to drop the member from the partition, and increments a
  **held** counter the trailer `printf` prints as a new field beside `scanned`,
  `groups`, undecidable, `ported_excluded` and `permanent_excluded`.
- **Consumer 2 — `check-gate-substrate-parity`, assertion G**, inside the member
  loop over the declaration set assertion A already derives, where the three
  `# no-port:` clauses live as one `if`/`elif` block. It reads the field's
  **placement** (spelling domain), its **payload non-emptiness**, its
  **cardinality**, and its **co-occurrence** with `# no-port:`; the clean line gains
  its count beside the existing `# no-port:` fragment.
- **Consumer 3 — `check-gate-exemption-tasks`**, over its existing
  `gate_check_dirs` walk of each check dir's `*.sh` and `*.gate`. It reads the
  field's **slug** and resolves it through the `IS_LIVE` map its `awk` prelude
  already builds from the live-section span and the bullet-lead-line predicate.
  **The gate's trigger is what changes**: it opens today on the
  `# exception-list:` marker, so a declaration carrying only a header field is
  currently skipped and the arm must be reached independently of that marker.

- **Consumer 4 — `check-comment-tier`**, over the shared comment surface, at the
  transition where it decides whether a comment line is a directive. It reads the
  **prefix alone** and never the payload, which is why delta 5's roster entry is
  the whole of what it needs — and why omitting that entry would red every
  declaration the other three consumers depend on.

**Every field has a named reader**, and the field has exactly one payload:

| payload | reader | transition |
| --- | --- | --- |
| presence | `port-blockers --group` | partition placement |
| presence | assertion G | descriptor-spelling clause, both-fields clause |
| slug, non-empty | assertion G | payload-shape clause |
| slug, resolved | `check-gate-exemption-tasks` | live-slug resolution |
| prefix | `check-comment-tier` | directive classification |

**Existing integration prose updated** — §port-blockers' derivation sentence, which
this change falsifies if left alone. Named in *Existing sections updated* below.

**Nothing is narrowed.** The change adds an exclusion to one tool's partition and
adds clauses to two gates; no corpus is pruned, no glob tightened, no file
dropped. Point 5's red-condition enumeration is therefore not binding here — and
it is stated rather than skipped, because the *partition* shrinking reads like a
narrowing and is not one: the corpus every reader walks is unchanged, and the two
gates' violation sets can only grow.

The one verdict that does move is `port-blockers`' own trailer, which is
**advisory** — §port-blockers states that nothing parses either arm's output — so
no gate's red condition depends on it. That is what makes the arithmetic delta a
documentation obligation rather than a gate-breaking one, and it is exactly why
delta 2 treats the prose sentence as the thing at risk.

## Existing sections updated

- **gate-sdk/SPEC.md §The port-candidate criteria** — the refusal paragraph
  (`A declarable spelling for the held classes is designed and refused`) becomes
  the record of the mint: what was refused, on what ground, and what supplied the
  holders. The refusal is *retired*, not deleted — a later reader must be able to
  see that the field waited for its holders rather than being reserved. Owned by
  deltas 1 and 6.
- **gate-sdk/SPEC.md §The `# graph:` manifest** — the paragraph minting
  `# no-port:` on the `<name>.sh` spelling alone gains its sibling, and the closed
  field roster gains one entry with its named reader. Owned by delta 1.
- **gate-sdk/SPEC.md §port-blockers** — the *declared-permanent members leave on
  exactly those terms* paragraph and its derivation sentence. This is the update
  that must not be missed: the documented subtraction stops yielding *still owed*
  the moment a held member is excluded. Owned by delta 2.
- **gate-sdk/SPEC.md §check-gate-substrate-parity, assertion G** — the two clauses
  double to four on identical terms, plus the new co-occurrence clause the pair of
  fields creates, for five; and the presence-is-not-asserted paragraph is re-stated
  over both fields rather than one. Owned by delta 3.
- **gate-sdk/SPEC.md §check-gate-exemption-tasks** — the invariant statement, the
  scope sentence (`Scope is in-script exemption arrays only`) which this widening
  contradicts as written, and the clean-line contract, which now reports a third
  count. The five-holders cost paragraph is updated to record that this widening
  added **no** holder, which is the reason it landed here. Owned by delta 4.
- **canon-kit/SPEC.md §check-comment-tier** — the blessed-prefix roster. Owned by
  delta 5.
- **The gate's own SPEC section for each holder** — the class-(b)/(c) cause
  already required there by §The port-candidate criteria now has a machine-read
  companion, and the two must agree. Owned by delta 6.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
