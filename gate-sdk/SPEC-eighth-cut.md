# SPEC amendment: eighth-budget-batch

The **eighth budget batch**: `check-gate-assertions` and `check-tree-terms` port
to the binary substrate. Two members, **two batches of one member each** in every
sense that matters — no joint proof, no shared walk, no comparison spanning
both — which is the property §The first cohort, and the rule that selects the
next rules makes a budget batch safe, and which means dropping either mid-build
invalidates nothing about the other.

**Selection is read off the oracle, not off this file.** `bash
gate-sdk/bin/port-blockers.sh --group`, run at this stage on 2026-08-23, trails
*106 member(s) scanned, 2 group(s) formed, 0 undecidable, 98 already ported and
excluded, 3 permanently shell and excluded, 3 temporarily held and excluded; 5
still owed, 2 takeable at this cut*, in two groups of one. The size arm is
permanently exhausted, so the **budget arm** composes, and the takeable tier is
exactly these two. That is identical to the read `.workflow/survey-record.md`
carries for this iteration's scope, so this stage cites the survey behind a
re-run witness rather than re-buying it.

**Neither member is held.** Both had their `# port-until:` declarations retired
2026-08-22 with their prices relocated to their own SPEC sections:
§check-gate-assertions prices `paste -sd, -` as criterion 7 **class (ii)**, an
incidental comma join whose verdict is identical either side of the substitution,
and names the GNU-awk three-argument `match()` a capture-API re-expression rather
than a hold; §check-tree-terms prices its criterion-4 bind as **a fixture widening
before the port**.

**Taking both empties the takeable tier**, after which every unported member sits
behind `cohort-held-members-port-prerequisites`.

## What changes

### (1) `check-tree-terms` dispatches to the binary

`gate-sdk/checks/check-tree-terms.sh` is replaced by
`gate-sdk/checks/check-tree-terms.gate` dispatching to
`native/src/gates/tree_terms.rs`, the shell original deleted. **Design-bearing.**

The descriptor keeps the manifest verbatim —
`couples=scripts/msg-patterns.list dir=one valve=none tier=precommit trigger=*`
— and keeps `# install: zero-config`.

**The port owes no new bridge work on the pattern-file half**, and that is
measured rather than assumed: `GATE_MSG_PATTERN_FILES` and
`GATE_MSG_PATTERN_FILES_LOCAL` already cross the config bridge today, because
`check-commit-msg` is ported and reads them through `walk::knob_array`. The two
gates share one pattern source precisely so the two halves of the leak ban cannot
drift, and the port preserves that by reading the same resolved arrays. **No
pattern is baked into `native/src/`** — the roster is consumer config on the
`check-graph` / `scripts/graph-vocab.sh` pattern, and baking one would publish a
consumer's vocabulary, which is the provenance seam's whole subject.

The **filter/exec split** is preserved: a fork-free per-path filter, then **one**
content match over all surviving paths. A port that matched per file would be the
regression the shell's own `# spec:` comment forbids.

### (2) `check-tree-terms`' three unspecified behaviors are ruled, not inherited

Three properties the shell form has by accident of `grep` and that a port would
otherwise settle by whatever it happened to emit. **Design-bearing.**

- **A GNU escape in a pattern is refused, loudly, and the public example is
  corrected.** The crate's POSIX-ERE engine refuses `\b`, `\s`, `\w` and the rest
  of the GNU set **by name** at compile, which becomes exit 2. `docs/ddd.md`
  publishes a worked adoption example whose pattern file is `\bparcel\b` /
  `\bshipment\b` and invokes **this gate** on it, so that documented config
  fail-closes after the port. The example is corrected to POSIX ERE and
  §check-tree-terms states the refusal as designed behavior — both, because
  correcting only the doc leaves a consumer with `\b` in their own list meeting an
  exit 2 with no stated cause. The divergence is not this port's to invent: it
  already exists on the same pattern set through the ported `check-commit-msg`;
  what this delta does is stop it being undocumented.
- **A binary file yields a path-only record.** GNU grep prints
  `Binary file <path> matches` and suppresses the line. The compiled form emits a
  path-only record and **still reds**, rather than dumping lossily-decoded bytes:
  a binary's bytes can carry no newline for a megabyte and can carry control
  characters, either of which corrupts the `path:lineno:line` grammar the red
  output contract rests on — and the record's *purpose*, naming a line to edit,
  does not exist for a binary anyway. Dead on this tree (no tracked binaries),
  live in a consumer's, which is exactly why it is ruled here rather than
  discovered there.
- **The three fail-closed arms are discharged by the shared helper.** The
  pattern-file grep, the `git ls-files` call and the content match each fail-close
  through one shared helper, held by `check-gate-fail-closed` across the whole
  registry. They are recorded as proved **there** rather than per member — stated
  as a limit rather than banked as coverage, on §Fixture-pair discipline's terms.

### (3) `check-tree-terms`' fixture pair widens — **before** the port

Criterion 4's discharge, and the delta the price was quoted as. **Design-bearing.**

The gate walks `git ls-files` over the whole tracked tree, so **every** registry
member's declaration path lies inside the corpus it scans as content — criterion
4's predicate verbatim, reached through the **walk** and not the trigger field,
which is one literal non-declaration file and selects no declaration at all. The
member joins the couple-clears-walk-binds register.

Measured at this stage: the pair exercises **2 of 12** control arms — the clean
line and one red record. The widening, and the three-way split of it:

- **Into the `good/` tree, each case proving a skip and each staying green:** a
  file under a pruned directory carrying a banned shape; a `msg-patterns.list` and
  a `msg-patterns.local.list` carrying one, which proves the self-exemption is a
  **prefix** glob rather than an exact-name match; and a non-regular file.
- **Into `good/patterns.list`:** a blank line, and the two live pattern shapes the
  fixture omits — including the only **anchored** shape in the live set, which is
  the one whose ERE anchoring a port can silently get wrong.
- **Into `bad/`:** a second leaking file, and a file carrying two leaks on one
  line and two on separate lines — which pins multi-record **ordering**, the
  no-dedup rule, and the `path:lineno:line` shape together.
- **Into a new `check-tree-terms.test.sh`:** the non-repository fail-close, the
  missing-required-pattern-file fail-close — **and with it the entire env-knob
  resolution path**, which both existing cases short-circuit by passing a
  positional — and the empty-pattern-set *tree unchecked* clean.

**Two facts about the fixture corpus decide the widening's shape, and both were
probed rather than reasoned.** A case dir is **not** its own repository: `git
ls-files` inside one returns the **outer** repo's index scoped to that subdir,
printed relative to cwd. So (i) the non-repository arm is structurally
unreachable from any case dir, which is why it takes the unit-test arm; (ii) the
`gate-tests` prune never fires inside a case, because the emitted paths never
carry that segment — which is why the pair works at all and why it is inert under
a port; and (iii) **a widening file that is written but not `git add`ed is
invisible to the gate**, so an untracked case silently under-covers.

### (4) `check-tree-terms`' port carries one authoring rule, with a live precedent

`native/src/gates/tree_terms.rs` is a tracked file **inside the gate's own scanned
corpus**, so any banned shape spelled literally in its source or its unit tests
reds the gate against itself. **Design-bearing.**

The module **composes** such a shape at runtime rather than spelling it. This is
not a new invention: `native/src/gates/commit_msg.rs` already does exactly this,
with a `# spec:` comment naming this gate as the reason. And the constraint runs
the **opposite** way for the widening files of delta (3), which may spell banned
shapes freely because `gate-tests` is pruned. Getting those two backwards is the
plausible build-stage error, which is why both directions are stated.

**The module takes its test inputs from the fixture files, never from the
consumer's tracked pattern list.** `commit_msg.rs` compiles in two live pattern
strings; repeating that here would be a second instance of the class
`kit-spec-consumer-config-literal` files and no gate catches — the leaked string
is not itself a banned term, so this very gate cannot see it. Refused rather than
inherited.

### (5) `check-gate-assertions` dispatches to the binary

`gate-sdk/checks/check-gate-assertions.sh` is replaced by
`gate-sdk/checks/check-gate-assertions.gate` dispatching to
`native/src/gates/gate_assertions.rs`, the shell original deleted. **Design-bearing.**

The four arms port unchanged — count-word ↔ span consistency, heading resolution,
the zero-marker retrofit obligation, and marker/span set equality with its
`missing` and `extra` sub-branches — as do the three fail-closed exits and the
**exit-3 internal skip sentinel**, which is not a failure path: a `.gate`-declared
member with no crate manifest present is counted onto the clean line as *declared
out of reach*, and that segment is part of the output contract.

Criterion 7 discharges as priced: `paste -sd, -` becomes a comma join over the
sorted label set at each of its four sites, and the three-argument `match()`
becomes the target language's own capture API. Neither changes a verdict.

### (6) Four `check-gate-assertions` port hazards are pinned in the contract

Each is a place where the natural port silently diverges. **Design-bearing.**

- **Lowercasing must be ASCII and index-preserving.** The awk matches on a
  lowercased copy and then slices the **original-case** paragraph using the
  lowercased copy's indices, which is sound only because gawk's `tolower` is
  length-preserving and indexes by **character**. The target language's Unicode
  lowercase is **not** length-preserving, so the port lowercases ASCII-only or
  carries both offset spaces. This is the single sharpest silent-divergence
  hazard in the member.
- **The slice keeps one boundary character.** The paragraph regex consumes one
  boundary character on each side, and the slice deliberately starts one position
  early so the trailing boundary character survives. A port slicing from the
  match end diverges on any paragraph whose enumeration noun is followed
  immediately by a parenthesis.
- **The sort is byte order, and that is a stated narrowing.** The shell's
  `sort -u` and `comm` run under the **ambient locale** with no `LC_ALL=C` pin; the
  compiled form sorts by byte, which is C-locale order. The two can differ only on
  a mixed-case label span under a UTF-8 locale, and no live span is anything but
  `A`-`H` or `1`-`3`. Recorded as a deliberate narrowing of a latent divergence
  rather than left as an accident of substrate.
- **The two marker grammars differ on purpose.** The extraction grep accepts a
  multi-character label while the contract-span regex accepts exactly one, so a
  multi-character marker can only ever surface as an *extra marker* finding. The
  port reproduces both widths rather than unifying them.

### (7) Criterion 4 binds on `check-gate-assertions`, and the contingent immunity is ended deliberately

**Design-bearing**, and this is the delta the cut's spine was named for.

**The criterion has two spellings in one section and they give opposite verdicts
here**, which is the finding, not a reading error. Under *a registry member's
declaration path lies inside the corpus the gate scans as content*, it **binds
today, before the port, in every configuration**: eight of the nine live
enumerated contracts already resolve their markers out of `native/src/gates/*.rs`
and the ninth out of a shell gate, so the gate reads registry members'
declaration and implementation paths as content on every run, and there is no
consumer config in which the SPEC corpus stops naming gates. Under *the gate's
**own** declaration path*, it **clears** — because §check-gate-assertions carries
no count-word-plus-labelled-span, so the gate's own heading is filtered out and
its own bytes are never read.

**That second verdict is a contingent immunity, not a structural one, and the
contrast with the register's structural case is the point.**
`check-gate-fixture-coverage`'s immunity is a theorem: it reaches a declaration's
bytes only for a member with no fixture pair, and it must carry a pair to pass its
own rule. This member's immunity is **one sentence of prose away from ending** —
and the port is the likeliest author of that sentence, since every ported
sibling's SPEC section opens with an enumerated contract and the descriptor's
`# spec:` one-liner conventionally states one too. An immunity whose only guard is
a prohibition that **this gate itself would have to enforce** is circular.

**Ruling, in two parts.**

- **The bind is taken** under the registry-member predicate, which is already true
  and is therefore not a choice: the member joins `check-graph` and
  `check-gate-exemption-tasks` in the **no-clearing-configuration** row. This is
  also the direction §The port-candidate criteria itself takes when a verdict is
  uncertain — the conservative verdict costs a fixture widening and cannot be
  wrong in the harmful direction, while clearing wrongly ships the hole the
  criterion exists to point at.
- **The contingency is ended by making the member self-auditing.**
  §check-gate-assertions gains its **own** enumerated contract over the four arms
  of delta (5), and `gate_assertions.rs` gains the matching markers, so the gate
  reads its own module and the immunity stops being a prose accident. The
  precedent is `check-comment-tier`, which audits its own declaration and whose
  **fixture pair, not the live tree, is what proves those arms** — the same
  sentence this member now inherits.

**The ordering this forces is stated, because it is not free.**
`check-gate-substrate-parity` assertion A forbids a script and a descriptor
coexisting in one resolve dir, so the cross-substrate comparison necessarily runs
on the **pre-descriptor** tree. Therefore: the pair widens **first**; parity is
proved over the **pair**, the only corpus inert under the port; the live-tree arm
is retained and its verdict recorded as **no disagreement found on the
pre-descriptor tree**, never as parity proved; and the enumerated contract plus
the module markers land **with** the port, since neither can exist before the
module does.

**The shared-snapshot ordering constraint binds independently** and is not
discharged by any of the above. This member's live corpus moves whenever **any**
of the nine contracts' members ports, so the parity comparison is pinned to a
snapshot — criterion 4 protects the oracle from the member's own port, the
ordering protects every comparison from a sibling's, and they are independent
facts.

### (8) `check-gate-assertions`' fixture pair widens — before the port

Measured at this stage, the pair stands at roughly **2 of 8 arms**: one finding
arm of four and one resolution arm of four. **Design-bearing.**

**One structural cause explains most of the darkness.** Both cases pass a second
positional, which short-circuits resolution to the *scripts-dir plus `.sh`*
branch, so the registry walk, the descriptor-to-module redirection and the
no-crate skip are reached by no case at all — and the **`//` marker leader**,
which is the leader every live descriptor-declared member actually uses, is
covered by nothing. The consumer-facing configuration path is proved by no case
either.

The `args` file carries positionals only and cannot set a knob, so opening those
arms takes one of two instruments, and the choice is ruled rather than left to the
build: a bespoke `check-gate-assertions.test.sh` standing up a throwaway
mini-consumer, on the precedent of `check-graph`'s own tree test, which is the
instrument that carried five member declarations in **both** spellings. It is
preferred to a per-case config file because it reaches the no-crate arm — which
needs a tree with **no crate manifest**, a state a case dir inside this repository
cannot have.

What the widening must reach: the count↔span arm, the unresolvable-heading arm,
the zero-marker retrofit arm, the `extra marker(s)` sub-branch, the registry
resolution arm, the descriptor-to-module redirection, the no-crate skip and its
clean-line segment, the `//` leader **including its indented form**, and the
multi-SPEC kit-roots walk.

### (9) `check-gate-assertions`' `couples=` field is corrected

The manifest's declaration reach is **empty in practice**, and the port fixes it
while it is authoring the descriptor anyway. **Mechanical.**

Probed at this stage: `kit:*.sh` expands to `<kit-root>/*.sh` and **no kit root
holds a top-level `.sh`**; `scripts/*.sh` matches nothing, because every gate in
this consumer's gates dir is now a descriptor. So the only declaration path the
field reaches is `native/src/gates/*.rs`, while the **walk** reads
`gate-sdk/checks/check-gate-substrate-parity.sh` on every run — a shell gate whose
edit re-fires nothing. The descriptor's `couples=` gains `kit:checks/*.sh` and
`kit:checks/*.gate`: the first closes the missed content trigger, the second is a
**reverse trigger**, since creating or deleting a descriptor changes which file
the gate resolves and greps even though the descriptor's own bytes are never read.

No conservation row is added by this: the field already carries
`native/src/gates/*.rs`, so the member is already substrate-sensitive and already
carries its row.

### (10) Descriptors, registration, and the port's remainder accounting

Both descriptors carry their existing `# install:` disposition and their `# spec:`
header field; `scripts/gates.list` is untouched, both members being registered by
bare name already. **Mechanical.**

The generated pre-commit hook, the enforcement-map projections, the graph artifact
and the docs mirrors regenerate; each prints its own regen command on red and the
full fan-out is rostered at `docs/site-architecture.md` §Generated projections.
The commit-time obligation is the battery **plus** `bash
gate-sdk/bin/build-native.sh`, and neither discharges the other.

**Criterion 5's aggregate price is measured, never predicted from this line.**
Both members are `# install: zero-config`, so both are seeded into a freshly
initialised consumer's registry and both are therefore **lost** on a host with no
published artifact: the cut's own predicted growth is **two**, intersected with the
measured profile's kit set, and the standing residual is read off
`installer_smoke`'s binary-less leg against the post-cut registry, from a clean
checkout of the cut's own commit reached **by path**. The judgment the cut owes is
the standing one — **accept and declare**: an adopter on an uncovered platform
receives each omission declared in its own registry rather than as a broken
battery, restoring the class shell-side reinstates the duplication the port
deletes, and a binary-gated declaration is what the omit path already is.

The port-track remainder moves as follows, read off the oracle at the build cut
and never off this line: the **takeable** tier empties and the owed count falls by
two. `native-gate-port-remaining-corpus` **demotes** on the
entry-outlives-the-amendment branch rather than moving to `## Done`.

### (11) The interpreter floor's residue shrinks, and its two readers are updated

After this cut `check-gate-assertions` no longer holds the **gawk** floor and
`paste` leaves the battery's program set. **Mechanical.**

gate-sdk/SPEC.md states the residual gawk floor as this member's and
`check-action-run-shell`'s; after the port only the latter holds it, and
`docs/install.md` §Requirements is the public reader of that claim. Whether
`paste` leaves the set **entirely** is measured at the build cut by re-running
`bash gate-sdk/bin/port-blockers.sh`, not asserted here — the tool reports which
program a rule invokes, and the run is the oracle.

## Producers and consumers

The port introduces **no new knob, no new field and no new message**, which is the
shape of a clean port and is stated so the absence is not read as an omission.
What it does introduce is a **new state per member** — a descriptor-dispatched
verdict where a shell-dispatched one stood — plus one new contract and one new
marker set, and each is named below.

**Each ported gate's compiled verdict** (new producer for an existing state).
*Producer:* `gate_command`'s config bridge, resolving the owning kit's shell
library into the argv baked verbatim into the generated pre-commit hook, and
dispatching the binary — the live path every one of the 98 already-ported members
takes, so nothing new must be configured for either member to run.
*Consumers:* the committing session through the output contract; the generated
pre-commit hook for `check-tree-terms` (`tier=precommit`); the align stage for
`check-gate-assertions` (`tier=align-only`, which emits into no hook — the reason
criterion 3 names a real cost for that member); `run-gates.sh` and CI; and
`run-gate-tests.sh` through each fixture pair.
*Red conditions, and neither is monotone under the widening deltas (3) and (8)
perform:* `run-gate-tests.sh` reds on an `expect.txt` substring **not found** — a
zero-count red — so adding a case cannot be cleared by inspection and every added
case's expectation is derived by running it; and `check-gate-fixture-coverage`
reds on a registry member with **no resolvable pair**, a red condition the port
must keep satisfied at every intermediate commit, not only at the end.

**§check-gate-assertions' own enumerated contract and `gate_assertions.rs`'
markers** (new interface, delta 7).
*Producer:* the amendment's own edit to gate-sdk/SPEC.md, plus the `// assertion
<label>:` markers in the module.
*Consumer:* `check-gate-assertions` itself, at the resolution transition, reading
its own module — which is the self-audit the delta rules.
*Named reader of every label:* the marker/span set-equality arm, at the comparison,
and the count↔span arm, at the count. A label carried by the span with no marker,
or the reverse, is precisely what the gate reds on, so no label can exist without a
reader by construction.
*Standing hazard, named:* the contract and the markers are now **two copies of one
fact held by the gate itself**, which is the intended coupling and not a
duplication defect — the same relationship `check-comment-tier` already carries
for its own declaration.

**`check-tree-terms`' three ruled behaviors** (delta 2).
*Producer of the GNU-escape refusal:* the crate's ERE compiler, at pattern compile,
on every run for every consumer whose pattern file carries one. *Consumer:* the
committing session, as an exit-2 harness error naming the escape.
*Producer of the path-only binary record:* the compiled content match, at the
per-file decode. *Consumer:* the same session, through the red output contract.
*Red condition:* the binary record reds on **finding** a match — monotone in the
violation set, so it is safe to clear by inspection.

**Existing integration prose describing the prior flow**, surveyed across the
**whole** component set rather than a hand-picked subset, with stderr unsilenced
on every probe so a bad path reads as an error and never as *no reader*. Four
classes were searched: the two member names across every tracked SPEC, README,
smoke script, roster and doc; the conservation table; the criterion-7 worked
example; and the interpreter-floor claims. What it reaches is inventoried below.

## Existing sections updated

Each names the delta that owns it.

- **gate-sdk/SPEC.md §check-tree-terms** — gains the compiled-subcommand opening
  the ported siblings carry, the three ruled behaviors of delta (2), and the
  criterion-4 discharge recorded as **taken** rather than priced (deltas 1, 2, 3).
- **gate-sdk/SPEC.md §check-gate-assertions** — gains the compiled-subcommand
  opening; gains the **tier, `dir=`, `valve=` and `couples=` statement it does not
  carry today**, which the ported siblings' sections do; gains the four pinned
  port hazards (delta 6); gains the criterion-4 ruling, both spellings, the
  contingent-immunity finding and the self-audit disposition (delta 7); gains the
  fixture-widening record and the instrument choice (delta 8); gains the corrected
  `couples=` and its cause (delta 9); and — the delta that changes what the gate
  scans — gains **its own enumerated contract over the four arms** (deltas 5, 7).
- **gate-sdk/SPEC.md §The port-candidate criteria** — criterion 4's
  no-clearing-configuration row gains `check-gate-assertions` as its third member,
  and the register gains the **contingent immunity** as a shape distinct from
  `check-gate-fixture-coverage`'s structural one; the couple-clears-walk-binds
  register gains `check-tree-terms` (deltas 3, 7).
- **gate-sdk/SPEC.md §The first cohort, and the rule that selects the next** —
  gains the eighth budget batch's record: its members, the oracle read that
  selected them, the takeable tier emptying, and the criterion-5 judgment
  (deltas 10, 11). The dated takeable-tier paragraph is superseded by this cut and
  is rewritten rather than left to read as current.
- **gate-sdk/SPEC.md §Meta-gate conservation for the binary substrate** — the
  `check-gate-assertions` conservation row currently reads *retained, corpus
  extended to the gate's Rust module*; after the port it describes a **ported**
  member and is re-worded on the precedent its two neighbours already set
  (deltas 5, 7).
- **gate-sdk/SPEC.md, the criterion-7 worked example** — the `paste` /
  class-(ii) prose is re-read at merge: the example stays, because the criterion's
  classes outlive this member, but the tense of *the member is takeable and
  priced* does not survive the port (deltas 5, 11).
- **gate-sdk/SPEC.md, the interpreter-floor claim** and **docs/install.md
  §Requirements** — the residual gawk floor loses one of its two holders
  (delta 11).
- **docs/ddd.md** — the worked adoption example's pattern file is corrected from
  GNU escapes to POSIX ERE, and it is the **public** surface the port would
  otherwise falsify (delta 2).
- **gate-sdk/gate-tests/check-tree-terms/** and
  **gate-sdk/gate-tests/check-gate-assertions/** — the two widenings, each landing
  **before** its own port and each with every new file **tracked**, without which
  the tree-terms cases are invisible to the gate they are meant to cover
  (deltas 3, 8).
- **gate-sdk/README.md, gate-sdk/smoke/install.sh, docs/enforcement.md,
  scripts/git-hooks/pre-commit, docs/check-graph.html and the docs SPEC
  mirrors** — the generated and rostered projections both members appear in;
  regenerated by their own commands rather than hand-edited (delta 10).
- **installer/README.md §The gate binary** and **gate-sdk/SPEC.md §Consumer
  smoke** — only if the binary-less residual roster grows; the roster is
  **measured** at the build cut by `installer_smoke`'s binary-less leg from a clean
  checkout of the cut's own commit, never predicted here. Both members are
  `zero-config`, so the growth is expected to be two and the recorded number is
  the measured one (delta 10).
- **.workflow/gate-timing-baseline.txt and .workflow/validate-baseline.txt** —
  both members carry rows; re-measured rather than edited (delta 10).
- **CLAUDE.md §The provenance seam (never cross it)** — re-read at merge: the
  seam's worked example is `check-graph` / `scripts/graph-vocab.sh` and
  `check-tree-terms`' own three-tier pattern roster is a second instance of the
  same shape. No edit is planned, and this entry exists so the check is not
  skipped (deltas 1, 4).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`), discharged at the iteration rather
      than at the commit.
- [ ] **Removals propagated** — grepped every spec, README, smoke script, roster
      and doc for both shell paths, for the gawk floor's retired holder, and for
      `paste`; nothing dangles.
- [ ] **Widen first, port second** — each member's fixture widening lands, and is
      tracked, before its own descriptor; a port answering only the criterion's
      sentence ships the hole the sentence was pointing at.
- [ ] **Parity run recorded with its honest verdict** — proved over each fixture
      pair; the live-tree arm recorded as *no disagreement found on the
      pre-descriptor tree*, never as parity proved.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
