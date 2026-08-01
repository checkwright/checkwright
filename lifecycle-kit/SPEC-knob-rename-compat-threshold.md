# SPEC amendment: knob-rename-compat-threshold

The knob-rename compat precedent (lifecycle-kit/SPEC.md, §Layout and
configuration, immediately above the `LIFECYCLE_KIT_*` roster) reads today:

> Knob-rename compat precedent: before the first release tag a knob rename is
> compat-free — no read-the-old-name shim, no deprecation window — because no
> external consumer can have vendored the kit yet (the first tag is a
> launch-comms prerequisite). From the first tag onward a rename owes the
> queue-bound deprecation mechanism and a tightened-gates/release-note
> declaration (the deprecation-lifecycle and upgrade-path rungs).

Its stated premise and its stated threshold have come apart. The premise — *no
external consumer can have vendored the kit yet* — still holds: no external
install has ever been observed. The threshold — *the first release tag* — was a
proxy for that premise on the parenthetical assumption that the first tag rides a
launch announcement. **Twenty-one tags have shipped and no launch announcement has
happened**, so the proxy reads as long crossed while the thing it proxied for is
untouched. The clause misfires on the next real knob rename, demanding a
deprecation window that protects nobody.

## The diagnosis the entry did not have, and what it changes

The entry framed the fix as swapping one observable for another. The survey behind
this amendment found something better: **the clause carries one threshold for two
obligations of different natures, and that conflation is the actual defect.**

- A **declaration** — the tightened-gates/release-note entry — is owed to anyone
  who *received* the artifact. A tag distributes; the tag is therefore the correct
  threshold for this half, and it has been correct all along.
- A **compat shim and deprecation window** are owed to a *promise of stability*.
  Nobody is owed a migration path off a name the project never promised to keep.
  A tag is a distribution event, not a promise, so it was never the right
  threshold for this half.

Splitting them dissolves the misfire without weakening anything, and it resolves a
**live contradiction between two governed surfaces** that this amendment is the
first to name. `docs/install.md` §Versioning's pre-1.0 qualifier states that
*"while the line is 0.x, breaking changes other than decommissions may ride
minors … each still declared in the note."* The current clause says a rename from
the first tag onward owes a deprecation window. Both are governed prose, both are
live, and on the shim/window half they disagree outright — the versioning policy
permits what the compat precedent forbids. On the declaration half they already
agree, which is the tell that the split is the real seam rather than a convenient
one.

## What changes

### Delta 1 — the threshold splits, and each half gets the observable that fits it *{design-bearing}*

The clause is rewritten to carry two thresholds:

**Declaration — owed from the first release tag, unconditionally.** Unchanged in
substance and now stated as its own rule. A tagged release is a distribution, and
whoever vendored it reads the note to learn what moved. The existing pointer to
the tightened-gates/release-note mechanism stands.

**Compat shim and deprecation window — owed from the project's declared
general-availability posture onward, not from any tag.** While a project's own
declared stability posture is pre-general-availability, a knob rename is
compat-free: no read-the-old-name shim, no deprecation window. The declaration is
still owed, so a consumer who vendored early is never surprised — they are told,
they are simply not carried.

**Why a declared posture rather than an adoption count.** The obvious alternative
observable is "the first observed external install", which tracks the entry's
stated premise most literally. It is rejected: it is unfalsifiable from the tree
(a project cannot prove nobody vendored it) and it makes the obligation turn on a
fact outside the project's control and possibly outside its knowledge. A declared
posture is in-tree, singular, and *the project's own act* — which is the honest
framing, because the obligation was always normative rather than empirical. You
owe a migration path because you promised stability, not because you counted
users. A project that has promised nothing owes nothing, and owes it the instant
it promises, whether or not anyone has installed.

That reframing is also why the tag was a bad proxy from the beginning rather than
one that merely aged: it measured distribution where the obligation tracks
promise, so it would have drifted under any release history, fast or slow.

**Fail-safe direction, stated so it cannot be read the permissive way.** A
consumer whose project declares no stability posture at all is treated as
**past** the threshold — the compat obligation applies. Silence must not read as
"still pre-GA", because that is the reading under which a project that never got
round to declaring anything grants itself a permanent exemption. The window is
opened by an explicit pre-GA declaration and by nothing else.

### Delta 2 — the false parenthetical is deleted, not softened *{design-bearing}*

The clause's parenthetical *"(the first tag is a launch-comms prerequisite)"* is
removed rather than qualified. It is the sentence that made the tag look like a
proxy for launch, it is false as written, and the rewritten clause no longer needs
a launch relationship of any kind — the two thresholds are a tag and a
declaration, both directly observable, with launch nowhere in the chain.

**This is one of two instances of the same false premise, and the other is a
sibling unit's.** `docs/install.md` §Versioning states *"The first tag rides the
launch announcement."* That instance is owned by this iteration's
`preview-release-cadence` amendment, which replaces it with the release-channel
declaration. Named here so the pair lands one reading rather than two: the tags
are pre-GA iteration artifacts and the announcement is a separate, later event.
**Stated as a sequencing rule rather than a mutual precondition**, because
"neither lands while the other's instance stands" is a deadlock no order satisfies:
the two corrections land in the **same commit**, or `preview-release-cadence`'s
first. What must not happen is this clause landing while `docs/install.md`'s
instance still stands — which is the same direction the ordering below requires for
an independent reason.

**Ordering, because it is load-bearing.** Delta 1's compat threshold names a
declared general-availability posture. In *this* repo that declaration is the
`Release channel:` line `preview-release-cadence` introduces in
`docs/install.md` §Versioning. The kit clause names the criterion generically and
never names that page — see §The seam — but the criterion is unsatisfiable here
until the declaration exists. **The channel declaration lands before or with this
clause, never after.**

### Delta 3 — the knob scoping is settled, with its reason, and stays *{design-bearing}*

The entry's second open question: does the corrected clause generalize beyond
knobs? It is knob-scoped today, sits above the `LIFECYCLE_KIT_*` roster, and is
cited nowhere else — a scoping that reads accidental.

**Ruling: it stays knob-scoped, and the scoping is substantive rather than
accidental.** Three reasons, recorded in the clause so the next reader does not
re-litigate it:

1. **The mechanism it points at is knob-shaped.** The deprecation path it invokes
   is the queue-bound deprecation markers and the release note's `Renamed knobs`
   section, whose `old → new` / `old → ∅` grammar is specific to config names. A
   rule reaching gate names or file conventions would point at a mechanism that
   does not accept them.
2. **No class is left without a home**, which is the load-bearing check — and it is
   stated at the strength the tree actually supports. A non-knob rename — a gate
   name, a file or directory convention — is **structurally accommodated** by the
   release note's `Behavior changes` section under `docs/install.md` §The upgrade
   contract, whose bullet lead is defined as *"the changed surface's name (the
   script, knob, template, or file), bolded"* — a definition that already admits
   every one of those classes. What the page does **not** carry is a sentence
   explicitly *routing* non-knob renames there, and no release note to date
   instantiates one, so this is a sound structural inference rather than an
   established convention. Recording the difference matters: the narrow scoping
   leaves no rename without a section shaped for it, which is all this reason
   claims, but the routing is inferred and the missing sentence is filed as a gap
   rather than asserted away.
3. **Widening it would restate a neighbour.** A general "any governed name" rule
   is doctrine-tier, and that placement call — new rule versus clause on an
   existing one — is the jurisdiction the deferred `rule-reach-before-merits` unit
   holds. (That entry states the general placement question rather than naming this
   knob-widening case among its instances, so this is its jurisdiction applied, not
   a literal citation.) Minting the wider rule here would settle a doctrine
   question from inside a kit SPEC and duplicate whatever that unit lands.

**No gap is filed for the widening itself, and the absence is deliberate**: reason 2
establishes every rename class has a section shaped for it, so the widening costs
nothing to leave unbuilt, and what remains is a placement question already filed
under another slug rather than a hole in coverage. **One narrower gap is owed and
is filed**: `docs/install.md` §The upgrade contract never states that a gate-name
or file/convention rename routes to `Behavior changes`, which reason 2 infers from
the section's own lead-token definition. Reason 2 stands on that inference; the
sentence that would make it explicit is a real absence and is filed rather than
flagged-and-skipped.

### Delta 4 — the restatement sweep *{mechanical}*

Grep the tree for restatements of the first-tag threshold and the
launch-prerequisite claim outside the two instances Deltas 1–2 own, and correct or
delete what turns up. Purely mechanical: the ruling is fixed, the sweep executes
it. This is the iteration's **only** sweep for this premise —
`preview-release-cadence` cites it rather than running a second pass.

**The sweep run at this spec turned up exactly one further target, and it is a
generated projection rather than prose.** `docs/lifecycle-kit/SPEC.md` mirrors the
clause, so it carries the old paragraph until the mirror is regenerated. The remedy
is a `scripts/gen-docs-mirror.sh` regen (fan-out and command:
`docs/site-architecture.md` §Generated projections), **never a hand-edit** — editing
a generated page is the defect the roster exists to prevent. Everything else the
grep surfaces is a false positive: a v0.1.0 post describing that release as "the
first tagged release", an unrelated harvest tag reusing the term, and the queue
entry's own quoted framing of the defect being fixed.

## Producers and consumers

This amendment changes a **rule**, not a mechanism. It introduces no state, no
event, no interface, no file, no knob, and no field. The causal-completeness pass
is therefore run on the rule's readers, which is where a prose ruling can still
go causally wrong — by having no one who acts on it, or by naming an input nothing
produces.

**The compat threshold** (changed rule, Delta 1).
*Producer of the input it reads:* the consumer's own declared stability posture —
a tracked, in-tree declaration the project maintains. In this repo that producer
is the `Release channel:` line in `docs/install.md` §Versioning, written by hand
into a tracked page (that page is not on `scripts/core-files.list`; tracked-ness is
what the claim needs and all it needs) and held against the publish workflow by
`check-release-channel-parity` (`preview-release-cadence`, Delta 1). So the input
is not merely named: it is produced by a surface that exists in every clone and is
gated against drift. That is the check this amendment most needed to pass, because
a threshold reading a declaration nobody maintains is the prose equivalent of a
producer whose enabling config no deployment sets.
*Consumer of the rule:* the build-stage session performing a knob rename, which
reads the clause to decide whether it owes a shim and a deprecation marker. This
is a human-and-agent-read rule with no gate reader — see §Enforcement below, where
that is ruled rather than left implicit.

**The declaration threshold** (unchanged rule, restated separately, Delta 1).
*Producer:* the tag, cut at the close stage's release-disposition step.
*Consumers:* the build stage appending to `.workflow/tightened-gates.txt`, and the
close stage composing the release note from it — both already exist and are
already the clause's readers today. This half's causal chain is untouched by this
amendment and is verified here only to confirm the split severs nothing.

**Enforcement, ruled explicitly.** This unit ships **no gate**, and that is a
ruling rather than an omission. The predicate a gate would need is *"this commit
renames a knob"*, which is not decidable from a diff without a knob-identity model
no kit has; a gate approximating it would fire on additions and removals and would
be valved into silence.

What this amendment removes instead is a **contradiction between two governed
surfaces** — lifecycle-kit's clause and `docs/install.md` §Versioning's pre-1.0
qualifier no longer disagree about whether a breaking rename may ride a minor.

**Stated precisely, because the neighbouring doctrine clause is easy to over-claim
here.** Enforcement-first ranks *removing the duplication* above gating it, and a
contradiction is not a duplication: the stronger move would be to delete one surface
and leave a single owner. That move is **unavailable, and its unavailability is
structural rather than a shortfall of effort**. §The seam below rules that the kit
states a criterion and never an instance, while the consumer states the instance —
so a kit clause and a consumer page saying compatible things about stability is the
correct kit/consumer split, not a duplication awaiting collapse. There is no single
surface both readers could share. Reconciling the two is therefore the most the
ordering makes available here, and the residual is a rule with no gate reader,
recorded rather than papered over.

## Existing sections updated

- **`lifecycle-kit/SPEC.md` §Layout and configuration, the knob-rename compat
  precedent paragraph** — rewritten to two thresholds (Delta 1), the false
  parenthetical deleted (Delta 2), the knob scoping stated with its reason
  (Delta 3). This is the amendment's whole surface within the kit.
- **`docs/install.md` §Versioning** — not edited by this amendment; its
  first-tag sentence is `preview-release-cadence`'s to remove, and its pre-1.0
  qualifier is the surface this clause is being made consistent *with*. Listed
  here because a reader of either amendment must see the coupling, and an update
  target no delta claims is exactly the orphan the amendment template warns about
  — this one is claimed, by the sibling.
- **No other kit SPEC changes**, and no consumer-side gate or config changes.

## The seam

Ruled explicitly, per CLAUDE.md §The provenance seam.

**The clause states a criterion, never an instance.** lifecycle-kit says the
compat window closes at the project's declared general-availability posture. It
does **not** name `docs/install.md`, the `Release channel:` line, the token
`preview`, or this repo's version line — every one of those is this project's
release posture, and a kit literal carrying them would ship one project's channel
vocabulary as everyone's. A consumer reads the criterion and points it at whatever
declaration it maintains.

**No knob is introduced, deliberately.** The tempting move is a
`LIFECYCLE_KIT_GA_DECLARATION`-shaped knob letting the kit *read* the consumer's
posture. It is rejected: nothing in the kit would consume the value, because the
clause's reader is a human or agent performing a rename and not a gate. A knob no
mechanism reads is a governed name added for symmetry — the field-with-no-reader
defect, wearing config's clothes. The config-via-env convention has nothing to
bind here, and that is the correct outcome rather than a shortfall.

**Nothing moves the other way either.** No private rule content is being pushed
down into the kit, and no kit mechanism is being pulled up into the consumer. The
amendment's entire footprint is one paragraph of generic prose in a kit SPEC.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition. (This amendment adds none; the pass is run on
      the rule's readers and the declaration its threshold consumes.)
- [ ] **Merged with no information lost** — the rewritten clause lands in
      §Layout and configuration in place of the old paragraph, not beside it; the
      merged spec reads as one coherent document a reader who never saw the
      amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for
      lifecycle-kit (`ls lifecycle-kit/SPEC-*.md`).
- [ ] **Removals propagated** — the launch-prerequisite parenthetical is gone,
      Delta 4's tree-wide sweep for restatements has run, and the sibling
      instance in `docs/install.md` §Versioning is confirmed removed by
      `preview-release-cadence` so the tree carries one reading of the first tag,
      not two.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks. (The knob-versus-general scoping is settled in Delta 3, not
      filed; §Delta 3 records why no filing is owed.)
