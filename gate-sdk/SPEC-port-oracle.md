# SPEC amendment: port-oracle-corpus-narrower-than-the-directive

The port's completion claim becomes measurable over the corpus the directive
actually bounds. `gate-sdk/bin/port-blockers.sh` gains a third arm that walks the
tracked shell tree rather than the gate registry, `# no-port:` and
`# port-until:` widen from a gate declaration path to any tracked script, and the
directive's own completion predicate becomes a number a session reads off a run
instead of a judgment a session takes by hand.

**The gap, measured at this stage.** TRAJECTORY.md §The closed rulings
(2026-08-23) states the predicate: every remaining non-test `.sh` outside the
battery runner and the install bootstrap "either carries a stated `no-port` cause
or is deleted". The oracle that reports the port's remainder walks
`scripts/gates.list`, so it scans **106 gate members** and can speak for none of
the rest. `bash gate-sdk/bin/port-blockers.sh --group` reads *6 still owed, 6
takeable at this cut*, and `shell-gate-tail-port` takes all six — after which the
oracle reads **zero takeable** over a corpus that is not the one the ruling names.
Every session that reads a zero there reads it as a finished port.

**The remainder is measured, decomposed and reconciled, not asserted.**
`git ls-files '*.sh'` is **404 files / 27,650 lines**. Minus `*.test.sh`:
**17,949**. Minus `gate-tests/` paths as well: **16,923** — the two figures the
entry carries, re-run at this stage and **confirmed exact**. Subtracting the two
buckets a port entry does own — the six registered shell gates (1,148 lines) and
the two kit-shipped gates this tree never registers (181 lines) — leaves:

**152 files, 15,594 lines of non-test, non-fixture, non-gate shell that no queue
entry owns and no oracle counts.** Decomposed: each kit's `bin/` 42 files/5,883
lines; each kit's `lib/` 18/3,397; `*/smoke/` 21/1,962; `installer/` 11/2,074;
`scripts/` 25/968; `*/templates/` 20/613; `drift-kit/kpis/` 13/552; `demo/` 1/96;
`context-kit/index-tests/` 1/49. `native/` and `.claude/` hold none. The
decomposition reconciles to the census exactly on both axes, which is what makes
it a partition rather than a sample.

**The entry's two shapes are both refused as stated, and the synthesis is why.**
Widening the existing walk "to a whole-tree scan with a per-file disposition"
would put a criterion-column report over a corpus that has no criteria — a plain
script carries no fixture pair, no tier and no `couples=`, so `c2=`, `c3=`,
`c7=` and the `couples=` cross-check are all unanswerable for it, and an arm that
printed them empty would invite exactly the guess §port-blockers removed three
columns to prevent. Giving the non-gate corpus "a second roster" reintroduces the
maintained list derivation-first refuses and that §port-blockers already declined
a freshness gate over. What both shapes are reaching for is available from
neither: **one tool, one place to read completion, and one arm per question** —
which is the shape adding `--group` already established, and which delta 1 takes.

**No exclusion knob is minted, and that is the sharper half of the design.** The
obvious spelling for "outside the battery and the bootstrap" is a pair of path
knobs, and it is refused: a knob defaulted to this tree's battery and bootstrap
paths is a kit literal carrying one project's layout, the defect §The install
disposition already names, and it would need editing every time a file moved. The
directive's own predicate supplies the mechanism instead — the bootstrap is
**permanently shell by a closed ruling**, so it *declares*, and the battery
runner is simply **owed** until `battery-runner-port` lands. Delta 2 is that
substitution, and it is what makes the arm's owed count reaching zero *be* the
completion predicate rather than approximate it.

**What this unit is not.** It ports nothing. It takes no disposition on any of
the 152 files — deciding which are deleted, which port and which declare is work
those files' owning entries hold, and an amendment that ruled 152 dispositions
would be a maintained roster wearing a design document's clothes. And it does
**not** widen `ported-gate-members`, which delta 4 rules explicitly.

## What changes

### (1) `--tree`: a third exclusive arm, over the tracked shell corpus

`bash gate-sdk/bin/port-blockers.sh --tree` reports the disposition of every
tracked non-test shell file, with a trailer whose owed count is the directive's
completion predicate. **Design-bearing.**

**Exclusive, like `--group`, and the existing arms move by not one byte.** The
default arm answers criterion 7 over the registry and `--group` answers criterion
6 over the still-shell members; neither's corpus, columns or trailer changes.
That is the precedent §port-blockers records for the second arm's addition,
stated there as a fact about that change rather than a standing guarantee — so
this delta re-establishes it by running the two arms across the commit and
diffing, rather than by citing it.

**The corpus is derived and its exclusions are rules, not lists.** It is
`git ls-files` over tracked `*.sh`, minus the `*.test.sh` suffix — which the
directive itself names by writing *non-test* — minus the shared prune-dir set
`GATE_SDK_PRUNE_DIRS` and `GATE_SDK_PRUNE_EXTRA_DIRS` resolve, which is what
removes `gate-tests/` fixture content without naming it. Both are honoured, on
§check-reads-couples' ground that a substrate honouring one of an additive pair
scans a different tree than the shell for any consumer who set the other.
Enumeration rather than a walk is correct here and is the one place it is: the
subject is *tracked* files, because an untracked script is not part of what the
project ships and cannot carry a reviewable declaration.

**One row per file, and the columns are the ones a plain script can answer.**
`<path><TAB><disposition><TAB>lines=<n>`, where `<disposition>` is `owed`,
`no-port`, or `port-until:<slug>`. `lines=` is `wc -l` over the same path the row
is read from, on §port-blockers' own admission of that column: exact rather than
a guess, and read down a list as a cost. **No criterion column is emitted**, and
the ground is the one that removed criteria 4, 5 and 6 from `--group` — a column
whose only honest reader would have to disregard it is not emitted. A registered
gate's row carries its disposition on the same terms as any other file; the arm
does not partition gates from scripts, because the directive does not.

**The trailer is the completion predicate.**

```
port-blockers --tree: <n> file(s) scanned, <n> declared no-port, <n> temporarily
held, <n> owed
```

Owed reaching **zero** is the ruling's sentence made decidable: every remaining
non-test `.sh` then either carries a stated cause or is gone. The *held* count is
separated from *no-port* for the reason §port-blockers separated *still owed*
from *takeable at this cut* — a temporary hold is not a permanent disposition,
and folding the two silently falsifies the subtraction a reader would do. There
is no fourth count, because there is no fourth disposition: absence of a
declaration is `owed`, which is the field's established default-by-absence
(§The `# graph:` manifest) and is what keeps an undeclared file **over**-counted
as work rather than lost.

**Its consumer is a human session and nothing parses it**, exactly like the two
arms beside it: the reader is a session asking whether the port is done, and the
transition is that question. No freshness gate accompanies it, on §port-blockers'
own enforcement-first ground — a gate would compare the derivation against a
stored expectation, which is the maintained roster returning by the back door and
wrong for every consumer whose tree differs.

### (2) `# no-port:` and `# port-until:` widen from a gate declaration to any tracked script

The two fields keep their grammar, their readers and their refusals, and gain a
corpus. **Design-bearing.**

**Nothing is migrated, because nothing declares today.** Both fields are live
mechanism with **zero declarations tree-wide** — the 2026-08-23 ruling that no
gate is permanently shell invalidated every cause that existed, and
`--group`'s trailer reads `0 permanently shell and excluded, 0 temporarily held
and excluded` in corroboration. So the widening lands on an empty set and cannot
break a holder.

**The domain rule is already stated wider than the fields' current corpus.**
§The `# graph:` manifest rules `# port-until:`'s domain as "any temporary hold
with named work owed and a live owning entry", and refuses to narrow it to the
born-native exception letters on the ground that the narrower reading "would
leave the criteria section's own worked example unable to declare". This delta is
that sentence applied to the other axis — the *corpus* rather than the *cause
class* — and `# no-port:` widens with it, because a permanent shell disposition
is exactly what the bootstrap has and exactly what the directive's predicate asks
a script to state.

**Three things are explicitly unchanged, so the widening adds no second
grammar.** The payloads: `# no-port:` carries free text because permanence is a
ruling whose home is prose, and `# port-until:` carries a bare slug because the
slug reaches what a queue reader needs. The mutual exclusion: at most one of the
pair on a file. And the closed-roster rule: a `.gate` descriptor still carries
**neither**, its existence being the dispatch declaration, which
§check-gate-substrate-parity assertion G still reds on.

**Assertion G's corpus does not widen with the field, and stating that is the
delta's most easily missed half.** G's subject is the *gate registry* — one
declaration per member, no contradiction between the pair, a non-empty cause —
and assertion H opens the section a declaration's own `# spec:` names. A plain
script has no registry membership and no `# spec:` pointer, so neither assertion
has anything to say about one, and widening their walk would red every script
that declared. The reader of a script's declaration is `--tree` alone. The
liveness half is delta 3's.

**The one existing surface a header on a plain script crosses is
`check-comment-tier`.** Its corpus is governed source and it demands every
full-line comment be a recognised directive or be deleted, which is exactly right
here: a `# no-port:` line whose class that gate did not know would red, and a
blanket exemption would be the "blessing a restatement" defect. So the field
joins its recognised directive classes, and its **mandatory non-empty payload**
is what makes it a directive rather than a restatement — the same test
`# comment-tier-exempt: <reason>` already passes.

### (3) A held script's slug is held live, by the reader that already holds a gate's

`check-gate-exemption-tasks`' `# port-until:` liveness arm widens to the same
corpus `--tree` walks. **Design-bearing.**

**Without this the widening ships its own worst failure direction.**
§port-blockers rules the asymmetry: an **undeclared** hold is counted owed, which
is the status quo and which a reader's own audit catches; a **stale** declaration,
whose blocker landed and whose slug moved to Done, under-counts the owed set and
hides real work. That is the direction no shape assertion covers, which is why a
gate declaration's slug is held to a live queue entry rather than to a shape. A
script's slug earns the same reader or the widened field ships an under-count
with nothing watching it.

**It is the same assertion over a wider walk, not a new one.** The gate already
resolves a slug against the live queue and reds when it moves to Done; what
changes is which files it collects slugs from. The precedent it is built on —
`# until: <live-slug>` paired with `# permanent: <reason>`, two annotations on one
axis — is that gate's own, so the widening inherits the anti-rot argument rather
than restating it.

**A `# no-port:` cause gets no such reader, and the asymmetry is deliberate.** A
cause is free text pointing at whatever surface records the ruling; nothing here
parses a cause, matches it against a vocabulary or knows what a section reference
looks like, which is the seam rule §The `# graph:` manifest fixes for both fields.
What holds a cause honest is review at the diff, and for a *gate* it is assertion
H — an assertion a plain script cannot satisfy, having no `# spec:` pointer to
open. Naming that gap here rather than closing it: the cheap closure would demand
a `# spec:` pointer on every declaring script, which mints a second obligation to
buy an assertion the free-text field was chosen not to need.

### (4) The tree-wide count is a new measured claim; `ported-gate-members` is not widened

`scripts/measured-claims.sh` gains one key for the `--tree` arm's owed count, and
the existing key keeps its meaning exactly. **Design-bearing**, and it is the
delta that prevents a public sentence from silently changing what it asserts.

**`ported-gate-members` answers a narrower question than the directive's and must
keep answering it.** Its oracle is `gates_list_members` over `scripts/gates.list`;
it reads **100** today and it means *how much of the gate battery dispatches to
the binary*. That value is bound by a `<!-- measured: -->` marker at
**docs/install.md:195**, under the public sentence "100 gates in the battery
dispatch to the compiled binary today", and held by `check-measured-claim`.
Redefining the key's oracle to the tree would leave that marker green, that
sentence unchanged, and its meaning replaced — the one failure mode a gated claim
cannot catch, because arm A compares the marker to the oracle and arm C compares
the marker to the prose, and both stay satisfied while the referent moves.

**So the arm's count gets its own key.** One new line in
`scripts/measured-claims.sh` emitting the `--tree` owed count, available to bind
a claim about the *tree's* port state wherever one is later written. It is minted
**with** its reader or not at all, on the closed-roster rule the descriptor
fields are held to: this amendment's own §What changes is that reader, and a key
emitted against a claim nobody has written would be a reservation.

**The vocabulary is the consumer's, which is the seam this delta holds.**
`CANON_KIT_MEASURED_CLAIMS_CMD` points at a consumer's emitter; the key names and
their meanings are this tree's config to define. gate-sdk ships the **arm** and
its trailer; it ships no claim key, no key name and no roster of what a consumer
should measure — a kit literal naming one project's claim vocabulary would
publish that project's measurement program as everyone's mechanism.

### (5) The oracle's two standing blind spots are recorded where the directive can see them

Two ways the gate arms miss a real requirement are stated at §port-blockers as
limits of the derivation rather than left to be re-found. **Design-bearing.**

**Both were found by this iteration's sibling unit rather than reasoned.**
`check-producer-liveness` requires `ps`, and the oracle can report neither: the
member is absent from `gates.list`, and the spawn sits in **a shared library**
(`ek_pid_alive`, `evidence-kit/lib/evidence.sh:117-122`) rather than in the gate's
own declaration text, which the command-position scan does not follow across.

**The second blind spot is the one that generalises**, and it is the reason this
delta exists rather than being a footnote on the first. Unregistered members are a
known corpus limit the `--tree` arm now covers. A **library-mediated** requirement
is different in kind: it is invisible for a **registered, in-corpus** member too,
so the default arm can report `clean` for a gate that genuinely requires an
off-floor program. That is a false negative of exactly the shape §port-blockers
already records for its repaired tokenizer — a member the roster reported clean
and now reports blocked — and recording it is what keeps a future reader from
trusting a `clean` more than the derivation earns.

**It is recorded and not repaired here.** Following a call into a kit library and
resolving its command positions is a scanner widening with its own cost and its
own false-positive surface, and this unit's deliverable is the *corpus* and the
*measurement*. The honest statement is the one §port-blockers already makes of
its undecidable count: the report's bound is stated, so a reader knows what share
of the corpus it cannot speak for. Filing the widening as work is the gap
disposition, not this amendment's ruling.

## Producers and consumers

**The `--tree` arm (new interface, delta 1).**
*Producer:* `gate-sdk/bin/port-blockers.sh`, at the argument parse, resolving
`GATE_SDK_PRUNE_DIRS` and `GATE_SDK_PRUNE_EXTRA_DIRS` through the same bridge the
tool's other arms already resolve `gate_sdk_gates_dir` and `gate_kit_roots`
through — so the arm introduces **no knob** and no default to be unset anywhere.
*Consumer:* a human session at exactly one transition — asking whether the port's
completion predicate holds — and `scripts/measured-claims.sh` at the emitter's own
run (delta 4). No gate parses it.
*Every row field has a named reader:* `<path>` by the session, at the row it acts
on; `<disposition>` by the trailer's three counters; `lines=` by the session
sizing a remaining unit, the reader §port-blockers already fixed for that column.

**The `--tree` trailer's owed count (new state, delta 1).**
*Producer:* the arm, derived from the row set at the end of the walk.
*Consumer:* `scripts/measured-claims.sh`'s new key (delta 4), and through it any
prose that later binds a `<!-- measured: -->` marker to it, at
`check-measured-claim`'s arm A. Its enabling configuration is
`CANON_KIT_MEASURED_CLAIMS_CMD`, which this tree already sets and which every
existing claim already flows through.

**`# no-port:` / `# port-until:` on a non-gate script (new state, delta 2).**
*Producer:* a session authoring the declaration, in the file's own header. The
enabling path is nothing but the file being tracked — there is no registration
step, which is the whole reason the field can reach a corpus that owns no
descriptor.
*Consumers, three, each at a named transition:* the `--tree` arm, at the row's
disposition column; `check-gate-exemption-tasks`, at the slug-liveness assertion,
for `# port-until:` only (delta 3); and `check-comment-tier`, at the
directive-class test, which is what keeps the line from reading as a restatement
(delta 2).
*Consumers it deliberately does not gain:* `check-gate-substrate-parity`
assertions G and H, whose subject is the gate registry, and `--group`'s two
exclusion arms, whose walk is `gates.list`. Both are stated as non-readers in
delta 2 because their existing code would otherwise be assumed to widen with the
field.

**The new measured-claim key (new field, delta 4).**
*Producer:* `scripts/measured-claims.sh`, one line, on every invocation of the
emitter.
*Consumer:* `check-measured-claim` arm B's vocabulary check on every run, which is
what makes the key non-vacuous the moment it exists; and arm A for any prose that
binds it. A key with no reader would be a reservation, which is why it is minted
in this amendment and not ahead of it.

**Red conditions named, because delta 2 widens a corpus and delta 3 widens a
walk — and one of them narrows something.** `check-comment-tier`'s red condition
is *a full-line comment on a governed source that is not a recognised directive*
— **not** monotone in the obvious direction, because delta 2 adds comment lines
rather than removing them, so every new declaration is a potential new violation
until the class is recognised. That is why the class registration is inside
delta 2 rather than deferred. `check-gate-exemption-tasks`' red condition is *a
declared slug that does not resolve to a live queue entry*, which is monotone in
the declaration set — widening the walk can only add findings, never remove one,
so no existing verdict can flip green-to-red by inspection failure. And the arm
addition **narrows nothing**: the default and `--group` arms keep their corpora
byte for byte, which delta 1 proves by diffing rather than asserting, precisely
because "a wider tool cannot change a narrower arm" is the same false comfort
§The causal-completeness check point 5 names from the other direction.

## Existing sections updated

- **gate-sdk/SPEC.md §port-blockers** — the third arm: its corpus derivation, its
  row and trailer grammar, the exclusivity restated as re-established rather than
  inherited, the refusal of a criterion column over a corpus with no criteria, and
  the two recorded blind spots (deltas 1, 5).
- **gate-sdk/SPEC.md §The `# graph:` manifest** — both fields' domain widens from
  a gate declaration path to any tracked script, with the payloads, the mutual
  exclusion, the descriptor's continued refusal of both, and the explicit
  statement that assertions G and H do **not** widen with them (delta 2).
- **gate-sdk/SPEC.md §check-gate-substrate-parity** — assertions G and H are
  stated as registry-scoped, which they already are and which the widened field
  makes worth saying (delta 2).
- **gate-sdk/SPEC.md §check-gate-exemption-tasks** — the `# port-until:` liveness
  arm's corpus widens to the tracked shell tree, and the asymmetry with
  `# no-port:` (no liveness reader, no `# spec:` pointer to open) is recorded
  (delta 3).
- **gate-sdk/SPEC.md §The port-candidate criteria** — the seven criteria are
  stated as bearing on *gates*, and the directive's predicate as bearing on the
  tree, so a reader does not apply a criterion to a script that has no fixture
  pair to carry (deltas 1, 2).
- **gate-sdk/README.md** — the `port-blockers` description enumerates the arms and
  gains `--tree` (delta 1).
- **canon-kit/SPEC.md §check-comment-tier** — `# no-port:` and `# port-until:`
  join the recognised directive classes, on the mandatory-payload test
  `# comment-tier-exempt:` already passes (delta 2).
- **canon-kit/SPEC.md §check-measured-claim** — nothing in the gate changes; the
  section gains the statement that a key's *meaning* is consumer-owned and that
  redefining an existing key's oracle is invisible to all three arms, which is the
  hazard delta 4 is taken against (delta 4).
- **CLAUDE.md §Delivery doctrine or §This repo is governed by its own kits** —
  wherever a session is told how to read the port's remainder, the tree-wide arm
  is the one that answers completion and the registry arms answer the battery
  (deltas 1, 4).
- **TRAJECTORY.md §The closed rulings**, on the 2026-08-23 completion predicate —
  the predicate gains its oracle: the sentence stays the ruling and stops being
  the only place the condition is evaluable (deltas 1, 4).
<!-- update-target-exempt: a no-change confirmation, owned by no delta by construction -->
- **docs/install.md §Versioning**, at the `ported-gate-members=100` marker — the
  claim, its key, its oracle and its sentence are **unchanged**, and this target
  is listed so the build confirms that rather than assuming it while delta 4
  reasons about the key it binds.
<!-- update-target-exempt: a no-change confirmation the ruling produces, owned by no delta -->
- **`scripts/measured-claims.sh`'s existing two claims** — `ported-gate-members`
  and `gate-substrates` keep their oracles and their meanings; listed for the same
  confirm-rather-than-assume reason, the new key sitting beside them.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`), discharged at the iteration rather
      than at the commit, a sibling amendment being in flight for gate-sdk.
- [ ] **Removals propagated** — grepped every spec for names this change retired
      (the gate-declaration-path scoping of `# no-port:` and `# port-until:`, and
      any prose reading the registry arms' remainder as the port's completion);
      nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred); the library-mediated-requirement scanner widening delta 5
      records is filed rather than flagged-and-skipped.
