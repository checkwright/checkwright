# SPEC amendment: keyed-knob-bridge

The array-knob config bridge grows a **key channel**, and the two registered
gates held off the binary substrate on its absence are ported behind it. The
hold is one prerequisite shared by two members in two kits
(gate-sdk/SPEC.md §The first cohort), so the wire change and both ports are one
increment: a bridge arm with no member reading it would be an unread interface.

Authoring found a **second** missing channel of the same class — `--reads` cannot
express the name filter a ported walk needs, and the descriptor has no exemption
to fall back on. It is ruled in §5b, where the design has one spec-compatible arm
and the scope half is the lead's.

## What changes

### 1. The bridge acquires a keyed arm, and its shape is derived, not declared

`_gate_knob_value` (gate-sdk/lib/gate.sh) gains a third arm beside the scalar and
indexed-array cases: a knob whose bash declaration is **associative** serializes
its **key-value pairs**, sorted by key, one pair per tab-separated element, each
pair spelled `<key>=<value>`. **Design-bearing.**

```
GATE_SDK_KNOB_<NAME>=<key>=<value><TAB><key>=<value>…
```

`LIFECYCLE_KIT_PREDECESSOR` as this repo configures it therefore crosses as

```
GATE_SDK_KNOB_LIFECYCLE_KIT_PREDECESSOR=align=scope	build=scope	close=validate	spec=scope	validate=build
```

The **split is on the first `=`**, so a value may contain `=` freely and only the
key is constrained. That is the same rule `env` itself applies one level out — the
outer `env` splits the argv element's first `=` to recover the variable name, the
reader splits each element's first `=` to recover the key — so the grammar adds no
second parsing convention to the protocol, it repeats the one already in it.

**Which arm a knob takes is derived from `declare -p`, not declared.** The bridge
already sources the owning kit's `lib/*.sh` and confirms the knob is defined; the
same `declare -p` output carries the `declare -A` marker, so the producer can see
the shape without being told it. **Design-bearing.**

That asymmetry with the prefix arm is deliberate and is the rule a later arm
should be measured against: the `*` spelling in a member's declared knob roster
exists because there **is** no variable named `EVIDENCE_KIT_RUN_` to inspect — the
spelling carries resolution information nothing else holds. A keyed knob carries
none: the variable exists and answers the question itself. Adding a spelling there
would maintain a fact already derivable, which derivation-first forbids, and would
put the declaration and the declaration's subject in two places that can disagree.
`--knobs` output stays a flat list of names, and no `.gate` descriptor field is
added.

**Rejected: reusing the prefix-family wire.** Decomposing a map into
`GATE_SDK_KNOB_<NAME>_<key>=<value>` variables would reuse
`_gate_knob_prefix_values` and `walk.rs::knob_prefix` almost entirely, and it was
the first candidate. It fails on the readers. Both held members iterate the map's
**key set** — `check-stage-entry.sh:67` walks
`${!LIFECYCLE_KIT_PREDECESSOR[@]}`, `check-evidence-baseline.sh:79` walks
`${!EVIDENCE_KIT_SCENARIO_GLOBS[@]}` — and gate-sdk/SPEC.md §lib/gate.sh rules
that **a prefix is a resolution set, never a roster**, precisely so a stray
variable sharing the stem is not published as a member (`EVIDENCE_KIT_RUN_ID` is
the recorded instance). A map has no separate roster knob to fall back on, so
reusing the family wire would either violate that rule or reintroduce exactly the
collision it was written against. It also restricts keys to identifier-safe
characters for no gain. A single variable carrying its own keys has neither
problem: the `declare -A` *is* the roster.

**Rejected: an index-aligned sibling knob** (`…=<values>` plus `…__KEYS=<keys>`).
The install-transport vocabularies cross index-aligned today
(gate-sdk/SPEC.md §lib/gate.sh, `GATE_SDK_RESOLVING_KNOB`), so the precedent is
live — but there the two halves are one consumer command's single output, whereas
this would mint a second bridged name per map whose alignment nothing enforces and
whose halves can be resolved, baked, and read independently.

### 2. Sorting is load-bearing, not cosmetic

Bash associative arrays iterate in **hash order**. Probed at this rev:
`LIFECYCLE_KIT_PREDECESSOR` yields keys `validate close build spec align`, and
values-only `build validate scope scope scope`. Both held members are
`tier=precommit`, so their resolved argv is baked **verbatim into
`scripts/git-hooks/pre-commit`, which is tracked** — an unsorted emission would
make that generated file's content depend on bash's hash seed and churn the diff
for no change. Sorting by key is therefore the same obligation the prefix arm
already carries and for the same stated reason. **Design-bearing.**

### 3. A third element-shape refusal, and one silent fail-open closed

The two existing refusals — an element containing a **newline** (breaks the
line-per-element argv protocol) or a **tab** (breaks the serialization) — apply to
the key and the value of every pair, naming the offending **key** rather than the
knob, as the prefix arm names the offending family member. A third joins them: a
**key containing `=`**, which would make the pair unsplittable. Values are
unconstrained beyond the existing two. An empty map serializes to the empty string
and is a resolved-empty map, so *absent* and *empty* part company exactly as they
do for an array. **Design-bearing.**

**This closes a live fail-open the current spec describes but does not enforce.**
`_gate_knob_value` checks only that the knob is *declared* (gate.sh:144) and then
expands it through a nameref; an associative array does **not** refuse today — it
silently serializes its values, in hash order, losing the keys. So the documented
limit ("a `declare -A` knob is not portable across the bridge as it stands") is
prose-only: nothing stops a consumer from porting such a gate and receiving a
quietly wrong environment. The keyed arm removes the hazard by construction — the
shape is now *taken*, not fallen through — so no separate guard is added. Probed,
not reasoned: the mis-serialization above is a live run at this rev, not a reading
of the code.

### 4. The receiving half

`native/src/walk.rs` gains `knob_map(knob: &str) -> Result<Vec<(String, String)>, String>`,
the map counterpart of `knob_array`. An **absent** variable is an error, because
the crate holds no default for a bridged knob; an **empty** one is a resolved-empty
map; an element carrying no `=` is an error naming the knob and the element.
Pairs are returned in the sorted order the wire carries, so a reader's output
order does not depend on the environment's — the same guarantee `knob_prefix`
gives. **Mechanical**, the contract being fully authored above.

Lookup rides `knob_in_family`, which already takes a `&[(String, String)]` and a
name: a map and a resolved prefix family are the same shape once resolved, so no
second lookup helper is minted.

**The residue this leaves, stated rather than discovered later.** A crate reading
`knob_array` on a knob a consumer has since redeclared `declare -A` receives
`key=value` strings and cannot tell. The reverse direction *is* caught — `knob_map`
refuses an element with no `=`. Closing the remaining direction means transporting
the reader's expected shape back to the producer, which is the maintained
declaration §1 just declined to mint, and the hazard needs a consumer to change a
shipped knob's **grammar**, which is already a kit-SPEC-governed contract change
rather than a configuration edit. Filed to `.workflow/gap-inbox.md` as
`knob-shape-flip-undetected` with its cost and a candidate close, for close to
drain — not left as a flagged-and-skipped gap.

### 5. `check-stage-entry` ports to the binary substrate

`lifecycle-kit/checks/check-stage-entry.sh` is replaced by
`lifecycle-kit/checks/check-stage-entry.gate` dispatching to
`native/src/gates/stage_entry.rs`, with a registry entry declaring its knobs and
the shell script deleted. Its three assertions, their calibration, and assertion
C's honest limit are unchanged — lifecycle-kit/SPEC.md §check-stage-entry stays
the contract and this port asserts nothing new. **Design-bearing.**

Every library function it reaches is already in the crate:
`native/src/stages.rs` carries `stages`, `stage_known`, `header`, `header_iter`,
`current_stage` and `data_lines`; `walk.rs` carries the pruned walk `gate_find`
resolves to; and assertion C's `grep -oE` over an amendment's contract-surface
tokens rides `native/src/ere.rs`, whose engine was paid at the ERE cohort. The map
is the only piece that was missing, which is what made this a one-blocker hold
rather than a sizing.

### 5b. `--reads` grows a filter-knob channel, because the port has no other arm

Porting `check-stage-entry` uncovers a **second missing channel of the same
class**, which scope did not cost and which the amendment rules here rather than
leaving for build to discover. **Design-bearing.**

The shell member's two whole-tree scans carry `# reads-couples-exempt:` markers
(`check-stage-entry.sh:125,132`, reason: the re-fire is owned by the state-file
couple every stage entry stamps). gate-sdk/SPEC.md §check-reads-couples rules that
**there is deliberately no descriptor-level exemption** — "a port ends this
assertion by answering it, never by opting out of it" — so the port loses that
channel by design. And `--reads` prints one line per walk root **and nothing
else**, so the consumption path analyzes a declared root with an **empty** name
filter (`check-reads-couples.sh:172`, `cover_root "$root" 1 "" …`) where the shell
arm passes the extracted `-name` pattern (`:189`). An unfiltered `.` demands that
every tracked file match `couples=TASK-QUEUE.md,.workflow/WORKFLOW-STATE.txt`,
which no widening short of `*` satisfies — and widening a glob to pass is
foreclosed by that section too.

So `--reads` grows the channel the shell analyzer lacks, and **the filter is
carried as a knob name, not as a pattern**. A root line may take a tab-separated
second field naming the knob whose value is the `-name` pattern —
`<root><TAB><knob-name>` — which `check-reads-couples` resolves through the config
bridge it already sources, passing the resolved value where it currently passes
`""`. A bare root keeps its present meaning, unfiltered, so every existing
declaration is unchanged and the 40 members declaring `?` are untouched. The
registry's third element grows to carry the pair.

**A literal pattern in the registry is refused, and this is the load-bearing
detail.** `check-stage-entry`'s scans are `-name "$LIFECYCLE_KIT_ROSTER_BASENAME"`
and `-name "$LIFECYCLE_KIT_AMENDMENT_GLOB"` — *knob values*, not literals — which
is exactly why the shell analyzer extracts nothing for them: `name_pattern`
discards a pattern containing `$` (`check-reads-couples.sh:66`), so the shell
member's exemption is not laziness but the only channel it had. Spelling `SPEC.md`
into the crate's registry to make the field static would be a second spelling of a
knob's default, which de-literalization forbids and which
`knob-default-accessor-singularity` names as a class. Carrying the **name** keeps
the value single-sourced and reuses the resolution path the bridge already owns —
so this delta and deltas 1–3 are the same repair twice: **a wire the compiled
substrate cannot say what the shell substrate could.**

`check-stage-entry` then declares `.` twice, filtered by those two knob names, and
its `couples=` widens to cover `*/SPEC.md`, `SPEC-*.md` and `*/SPEC-*.md`. That
widening is **correct coupling, not a concession**: an amendment landing anywhere
genuinely changes what assertion C sees, which is the thing the shell member's
exemption was excusing rather than expressing. The port therefore **answers** the
assertion where the shell member was exempt from it — the strictly-better outcome
that section demands of a port. It also makes check-stage-entry the arm's **first
live instance**: gate-sdk/SPEC.md records that "no ported member reports a
resolvable walk root, so this arm has no live instance and its clean line reports
a counted zero", and this increment ends that.

**Two alternatives, both refused, recorded so a later port does not retry them.**
Declaring `?` is refused: `?` marks a root that cannot be *bounded statically*, and
every member carrying it does so because its root is an argument with a default,
whereas this one is a literal `.` — spelling it `?` would be the opt-out the
section forecloses, moved into the registry. Re-implementing the scan over
`git ls-files` to fall outside the analyzed class is refused for the same reason
with a behavioral cost on top: it is out of scope *because* it is not a walk, so
using it to evade the assertion is opting out spelled in code, and it silently
narrows assertion C to **tracked** amendments, where the shell member sees an
untracked one too.

**This delta's scope is the lead's to confirm; its design is not open.** What is
open is whether this increment carries the delta or splits — porting
`check-evidence-baseline` alone (unaffected: its glob expansion is not a `find` at
command position, and `queue_slug_liveness.rs:72` is the precedent for declaring
`?` when the globs come from a knob) and holding `check-stage-entry` on the newly
named `--reads` prerequisite, designed above so the next increment ports rather
than discovers.

The amendment is authored for **carrying it**, on two grounds the split does not
answer. First, splitting leaves the keyed arm with **no non-empty live instance in
this tree**: `EVIDENCE_KIT_SCENARIO_GLOBS` is empty here (delta 6), so the only map
that actually resolves at every pre-commit is `LIFECYCLE_KIT_PREDECESSOR`, and it
resolves only if `check-stage-entry` is the member reading it. A wire format whose
in-tree exercise is the empty case is a wire format whose first real consumer finds
the bugs. Second, the blocker-retiring override outranked the size arm's two-member
group 1 **on the two-member count**; retiring a blocker for one member is a weaker
case than the arm it displaced. Against that, the honest cost of carrying it: this
increment is larger than scope costed, by one wire-format change scope did not see.

### 6. `check-evidence-baseline` ports to the binary substrate

`evidence-kit/checks/check-evidence-baseline.sh` is replaced by
`evidence-kit/checks/check-evidence-baseline.gate` dispatching to
`native/src/gates/evidence_baseline.rs`, same shape, shell script deleted.
Assertions unchanged from evidence-kit/SPEC.md §check-evidence-baseline: baseline
grammar, blocking-slug liveness against the queue, and per-suite manifest↔disk set
equality. **Design-bearing.**

Its one shell library function, `ek_data_lines`, is a comment-and-blank filter that
lands in the crate beside the reader; the glob expansion `for f in $glob` rides
`walk.rs::glob_files`, and the queue's slug/section walk rides `native/src/queue.rs`,
which the queue-kit cohort already paid for.

**Its enabling config in this tree is fixture-only, and that is a real
causal-completeness caveat rather than a defect.** `scripts/evidence-config.sh`
sets neither `EVIDENCE_KIT_SCENARIO_GLOBS` nor `EVIDENCE_KIT_PERMANENT_SLUGS`, so
the kit default (`evidence-kit/lib/evidence.sh:44`, `declare -A …=()`) is what
crosses in the repo's own battery — the **empty-map** path. The non-empty map is
exercised only by `evidence-kit/gate-tests/check-evidence-baseline.test.sh:68`.
Consequence for build: a defect in the keyed wire would pass this repo's own
pre-commit run of that gate and be caught only by the fixture suite, so the fixture
is the load-bearing evidence for the non-empty arm and must run before the port is
called done. `LIFECYCLE_KIT_PREDECESSOR` has no such gap — `scripts/lifecycle-config.sh`
sets a five-key map that every commit resolves.

### 7. Fixture coverage for the new arm

`gate-sdk/gate-tests/lib-gate.test.sh` gains keyed-arm cases beside its existing
scalar, spaced-element and prefix-family ones: a map resolving to sorted pairs, an
empty map resolving to the empty string, and each of the three refusals — a key or
value carrying a tab, one carrying a newline, and a key carrying `=`. **Mechanical.**

Both ported members keep their `good/`+`bad/` fixture pair and their existing
`gate-tests/` suites, which move with the port rather than being re-authored.
**Mechanical.**

### 8. The generated projections restale together

The port changes two members' dispatch, so the generated pre-commit hook, the
graph artifact, the enforcement map, the footprint and value rollups and the docs
mirror all restale, and the binary itself must be rebuilt
(`bash gate-sdk/bin/build-native.sh`, which the battery does not discharge). The
roster and each regen command are docs/site-architecture.md §Generated
projections'; this amendment names the fan-out rather than restating it.
**Mechanical.**

### 9. The seam ruling (a ruling, not a delta — it carries no build work)

**Kit mechanism:** the keyed wire grammar, the shape derivation, the three
refusals, both halves of the transport, and `--reads`' filter field. All generic —
no term list, no vocabulary, no product constant. The filter *values*
check-stage-entry declares are its own kit's knobs
(`LIFECYCLE_KIT_ROSTER_BASENAME`, `LIFECYCLE_KIT_AMENDMENT_GLOB`), read through
the bridge rather than spelled as gate-sdk literals — the seam holds there too.
**Consumer config:** nothing new. **No knob is minted by this amendment.** The two
live maps' *contents* are already consumer config in `scripts/lifecycle-config.sh`
and `scripts/evidence-config.sh` and stay there; the bridge learns to carry a shape,
not a value.
**Private rule content:** none is involved. The maps carry stage names and file
globs, which are this consumer's configuration and already tracked in public files.

A third `declare -A` knob-shaped instance exists, `QUEUE_KIT_LESSON_SINKS`, which
ported with the limit untouched because no **gate** reads it — only
`queue-kit/bin/lesson-sink.sh`. It becomes portable when this lands and is named
here so a later selector does not re-derive its status, but it is **not** in this
increment: it has no held member to release.

## Producers and consumers

**New interface: the keyed wire element `<key>=<value>`.**
- *Producer* — `_gate_knob_value` in `gate-sdk/lib/gate.sh`, reached by
  `gate_knob_env_one` for every declared knob of every `.gate` member, on the
  argv `gate_command` emits. Enabling config: none — the arm is selected by the
  knob's own bash declaration, so it is live the moment a consumer declares a map
  and a member declares it. In this tree it fires on `LIFECYCLE_KIT_PREDECESSOR`
  at every pre-commit run and on `EVIDENCE_KIT_SCENARIO_GLOBS` (empty in-tree,
  non-empty in the fixture).
- *Consumer* — `walk.rs::knob_map`, called by `stage_entry.rs` and
  `evidence_baseline.rs`, reading the process environment `env` established in the
  argv. The generated pre-commit hook is the second delivery path for the same
  elements, baked verbatim.
- *Every field has a named reader.* The wire has exactly two fields per element.
  **Key** is read by `stage_entry.rs` at assertion A (`predecessor[entered_stage]`)
  and at assertion B's drain-successor scan (which iterates keys), and by
  `evidence_baseline.rs` at the per-suite coverage loop (which iterates keys as
  the suite roster). **Value** is read by `stage_entry.rs` at those same two
  transitions (the predecessor stage name, compared against the drain stage) and
  by `evidence_baseline.rs` as the glob it expands. Neither field is populated at a
  transition where it is not read, and no third field is added — a shape marker was
  considered in §1 and refused for want of a reader.

**New crate interface: `knob_map`.**
- *Producer* — the bridge above. *Consumer* — the two gate modules named.
- It has two readers on the day it lands, which is the bar `--knobs` enforces
  structurally elsewhere: the crate declares only knobs its own code reads.

**New field: `--reads`' optional filter-knob name (delta 5b).**
- *Producer* — `main.rs`'s `--reads` arm, printing the registry's declared
  (root, knob-name) pairs. Enabling config: none; it is registry data a member
  cannot compile without, the same construction that makes `--knobs`
  un-omittable.
- *Consumer* — `check-reads-couples.sh`'s consumption path, at the call site that
  currently passes `""`. It resolves the named knob through `gate_knob_env_one`,
  which it reaches by already sourcing `lib/gate.sh`, and forwards the **resolved
  value** to `cover_root` as the `git ls-files` filter. That is the field's
  **only** reader, at exactly one transition: the per-root coverage assertion for
  a `.gate` member. A bare root line stays bare and keeps its meaning, so the
  field is populated nowhere it is not read.
- *Resolution failure is fail-closed*, inheriting the bridge's own contract: a
  named knob the owning kit does not define is exit 2 naming it, never an empty
  filter silently widening the demand to the whole root.
- The field has a reader on the day it lands (`check-stage-entry`'s two declared
  roots); had it not, §1's own rule would delete it.

**New descriptors: two `.gate` files.**
- *Producer* — this amendment's build. *Consumer* — `gate_command`, which resolves
  `<dir>/<name>.gate` to the two-element binary argv, and `gates.list`, whose
  registration is unchanged (a member's name does not change with its substrate).

**Red conditions, because this delta narrows two corpora.** The build deletes two
shell scripts, so every reader whose corpus is `*/checks/*.sh` sees a **narrowing**,
and a narrowing may *add* violations wherever a verdict is non-monotone
(canon-kit/SPEC.md §The causal-completeness check, point 5). Enumerated by red
condition, not by subject:
- `check-gate-substrate-parity` — reds on a name carrying **both** spellings, and
  on a `.gate` whose subcommand the binary does not answer. **Non-monotone**: the
  port adds a `.gate` before the crate answers it, so the transient both-spellings
  and unknown-subcommand states are both red. Discharged by landing descriptor,
  module, registry entry and deletion in one commit per member.
- `check-gate-fixture-coverage` — reds on a registered member with **no** fixture
  pair; a **zero-count** red condition, non-monotone. Discharged by moving each
  pair with its member rather than re-authoring it.
- `check-exec-bit` — reds on a `checks/` file without the bit. Monotone under
  deletion; clear by inspection.
- `check-shellcheck` — corpus `*/checks/*.sh`; reds per finding. Monotone under
  deletion; clear by inspection.
- `check-install-claim` — the attested non-monotone case: its red condition is a
  **zero count**, so pruning a file holding a declaration's sole instance flips it
  green to red. Both deleted scripts carry `# install:` lines (`on-surface` and
  `zero-config`), which the replacing `.gate` descriptors must carry forward
  verbatim — the descriptor format takes them, and `check-stage-evidence.gate` is
  the worked precedent.
- `check-reads-couples` — reds on a member walking a tracked path no `couples=`
  covers, and its clean line reports a **counted zero** of analyzed roots today.
  Non-monotone twice over: the port removes two exempted shell walks (shrinking
  the exempt counter) while adding the arm's first analyzed root (moving the
  counted zero off zero), and the coverage assertion then runs for real. Discharged
  by delta 5b, which is what makes the ported member's declared root tractable at
  all; without 5b this reader is red and no widening or exemption clears it.
- `check-crate-arms` / `check-gate-binary-fresh` — the crate grows two modules and
  the binary must be rebuilt; both red until `build-native.sh` runs. Monotone in
  the sense that matters (they red on staleness, which the rebuild clears).

## Existing sections updated

- **gate-sdk/SPEC.md §lib/gate.sh, the array-knob config bridge** — owned by
  deltas 1–3. The bullet ruling that the bridge "carries scalars and indexed arrays
  and cannot carry an associative array" is replaced by the keyed grammar, its
  derived-shape rule, and the rejected alternatives; the three-refusal bullet grows
  its third; the paragraph contrasting a prefix family with "the keyed knob the
  bullet above holds off the substrate" is rewritten, since both shapes now cross
  and the contrast becomes *which* wire each takes and why, not whether one crosses.
  The sentence naming `QUEUE_KIT_LESSON_SINKS`, `EVIDENCE_KIT_SCENARIO_GLOBS` and
  `LIFECYCLE_KIT_PREDECESSOR` as the exercised limit is rewritten to record them as
  the keyed arm's instances.
- **gate-sdk/SPEC.md §The first cohort, and the rule that selects the next** —
  owned by deltas 5 and 6. The sixth cohort's hold record ("**`check-stage-entry`
  is held**… a prerequisite it now shares with `check-evidence-baseline`") states a
  hold that no longer exists; it becomes a **retired** hold naming the increment
  that paid it, keeping the grounds, since the section is canonical for every
  cohort's holds and their disposition. The blocker-retiring override's live-arm
  record gains this increment.
- **gate-sdk/SPEC.md §check-reads-couples** — owned by delta 5b. The `--reads`
  grammar ("one line per walk root and nothing else") grows the optional filter
  field; the sentence recording that "no ported member reports a resolvable walk
  root, so this arm has no live instance and its clean line reports a counted zero"
  is replaced by the live instance and what it covers; the no-descriptor-exemption
  ruling is **unchanged and is the reason this delta exists**, so it is cited in
  the new text rather than softened.
- **gate-sdk/SPEC.md §Layout and configuration** — owned by delta 1, the knob
  roster's `GATE_SDK_KNOB_<NAME>` entry, whose description of what the name carries
  now spans three shapes.
- **lifecycle-kit/SPEC.md §check-stage-entry** — owned by delta 5, for the fixture
  and coverage sentence naming the shell script's good/bad pair.
- **evidence-kit/SPEC.md §check-evidence-baseline** — owned by delta 6, same
  reason, plus §Layout and configuration's `EVIDENCE_KIT_SCENARIO_GLOBS` entry if
  it states a substrate constraint.
- **docs mirror of every file above** — owned by delta 8, regenerated not edited.

No wire-contract delta is embedded beyond the two fenced examples in §1, which are
illustrations of a grammar this amendment states in prose; at merge they become
part of the bridge section's own worked example rather than a cited contract file.

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
