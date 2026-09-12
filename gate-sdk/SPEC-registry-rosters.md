# SPEC amendment: registry-rosters

The binary carries several rosters — the crate's gate registry, the battery's
`scripts/gates.list`, the declared walk roots each member publishes through
`--reads`, and the non-gate `--emit` arm set — and **nothing joins any pair of
them**. This amendment makes four of those joins, and repairs the one place a
roster's value is computed for a caller that never receives it.

Queue entries: `bridged-knob-case-tmp-dir-override-inert` (deltas 1 and 2),
`reads-couples-skipped-set-has-no-census-oracle` (deltas 3 and 4),
`crate-registry-battery-roster-divergence-unasserted` (deltas 5, 6 and 7),
`unknown-emit-arm-diagnostic-misdirects` (deltas 8, 9 and 10). All four are
**operator directions of 2026-09-12**, revisable here; where authoring found a
premise that does not hold, the delta says so at the passage rather than
building around it.

**The seam is already ruled and is not re-opened** (scope, `e597f672`): pinning a
fixture case's scratch in every spelling a member may read is **kit mechanism**.
**One ground of that decision is corrected here and the decision itself stands.**
It was stated as though descriptive — that a case's sandbox location *is* the
harness's own fact, never a choice — and build measured that it is not: a case
config assigning `GATE_SDK_TMP_DIR` overrides the harness's pin at the live code
today (delta 2, with its reproduction). The decision is **prescriptive**. It
refuses to *mint* a per-gate opt-out; it never claimed none existed, so finding one
already live does not contradict it, and recording that one is not minting it.
Nothing becomes consumer
config; `GATE_SDK_TMP_DIR` already exists and its value stays the consumer's for
ordinary runs. Two shapes are foreclosed rather than ranked: computing the pin at
the **member** (a second producer on a single-producer bridge), and a **per-gate
opt-out knob** (a second source for one pin). Delta 2 exists because the SPEC
currently presents the second of those as sanctioned behaviour.

## What changes

### (1) The case scratch pin becomes an input to the config bridge, not an override of its output

`--run-gate-tests` supplies the pinned scratch to the shell that resolves a
case's argv, so the bridge computes it once and every spelling a member may read
carries it {design-bearing}.

**What is broken, traced rather than inferred.** `setup` computes the pin
correctly — `native/src/emit/run_gate_tests.rs:94` absolutizes
`GATE_SDK_TMP_DIR` against the invoker's root, which is the whole point of the
mechanism §run-gate-tests describes. `run_case` then pushes it into the per-case
child environment under the **bare** name
(`run_gate_tests.rs:333-334`). That reaches a `.sh` member, which sources
`lib/gate.sh` in its own process and reads the bare name — so the existing pin is
live, not dead code. It does **not** reach a `.gate` member: every crate-side
reader goes through `walk::knob_scalar`, which reads
`GATE_SDK_KNOB_<NAME>` and nothing else (`native/src/walk.rs:256-257`). The pin
covers one of the two spellings, which is exactly the entry's claim.

**Why pinning the prefixed name at the same point would not have worked, stated
because it is the obvious repair and it is wrong.** A `.gate` member's argv is
`env GATE_SDK_KNOB_GATE_SDK_TMP_DIR=<value> <binary> <name>`, so `env(1)` is
argv[0] and its own assignment is applied *after* the environment
`Command::env` built. The harness would be setting a variable the very next
process overwrites. The value has to be corrected **upstream of the bridge**, not
downstream of it.

**Where the repro-relative value actually enters.** `resolve_argv`
(`run_gate_tests.rs:418-437`) spawns `bash` with **no environment vector**, so
the pin never reaches the process that bakes the argv. `lib/gate.sh:48` —
`[[ -v GATE_SDK_TMP_DIR ]] || GATE_SDK_TMP_DIR=".tmp"` — therefore resolves the
deliberately repo-relative default, and `_gate_knob_emit` bakes `.tmp` into the
`env` prefix. Under the case's working directory that resolves inside the tracked
fixture corpus.

After this delta `resolve_argv` exports the pinned value into the resolving shell
**in the spelling that script already uses for the binary** — a positional the
script `export`s, beside `GATE_SDK_NATIVE_BIN` — rather than relying on ambient
inheritance. Three properties follow, and they are why this spelling was chosen
over rewriting the emitted argv element:

- **The bridge stays the single producer.** The harness supplies an *input*; the
  value crossing the seam is still the one `_gate_knob_emit` computed. Rewriting
  the emitted `GATE_SDK_KNOB_GATE_SDK_TMP_DIR=` element would have made the
  harness a second producer on a single-producer surface, which is the
  criterion-6 shape the seam ruling forecloses.
- **Both spellings are served by one pin.** The bare name is what the resolving
  shell reads and what a `.sh` member reads; the prefixed name is what the bridge
  derives from it. The pin at `run_case` stays exactly as it is — after this delta
  the two pins are the same value reaching the two substrates, rather than one
  live pin and one inert one.
- **Every future member is covered without a further edit.** The fix is at the
  knob's resolution, not at a member, so a gate ported tomorrow that declares
  `GATE_SDK_TMP_DIR` inherits it. Today exactly one registry member declares the
  knob — `check-crate-arms` (`native/src/gates/mod.rs:1580-1591`) — and an
  entry-shaped fix naming that member would have to be re-made at the second one.

**Per-case sandbox isolation is a deliberate non-target.** The pin resolves to the
invoker's root, so every case shares one scratch directory; giving each case its
own would additionally stop one case's cache greening another. That is a different
property from the one this unit closes (a gate depositing state inside the corpus
it is the oracle for), §run-gate-tests already commits to invoker-root
absolutization, and widening here would settle a question no entry asks. Recorded
so a later reader does not read the absence as an oversight.

### (2) Three passages about one seam are re-grounded, and the live override is recorded as a hole

One root defect — the consumer config seam's assignments landing ahead of the
resolutions that would preserve an inherited value — has produced three wrong
statements in §Layout and configuration and §run-gate-tests, and the section
contradicts itself between two of them {design-bearing}.

**Authoring's premise was itself half wrong, and build measured it.** Authoring
held that the arm reads the *invoker's* seam only, and concluded that the
case-config claim was false and should be withdrawn. The `cd`-order half is right;
the conclusion drawn from it is not. Literal reproduction of the resolution script
against two directories each carrying a `scripts/gate-sdk-config.sh`, each logging
its own `$PWD`, records **two** seam source events per resolution — the invoker's,
then the **case's**. The second comes from `gate_command`'s knob resolution, which
re-sources the owning kit's `lib/*.sh` in a subshell (`gate.sh:381-396`) that runs
after the `cd`. So a case config naming its own scratch does win, at the live code,
today.

The three passages and what each takes:

- **`§Layout and configuration`, the seam's own precedence claim.** It reads *"Env
  vars still win (the config file sets a default the invoking shell may
  override)"*. False: the library `source`s the file plainly, so a bare
  `NAME=value` assignment beats the environment. Measured — an exported
  `GATE_SDK_GRAPH_ARTIFACT` does not survive this repo's own seam, and all ten live
  consumer configs in this tree use bare assignment. **Corrected** to state that the
  precedence is the config file's choice and not the seam's, with the consequence a
  caller needs: a harness cannot make a knob authoritative by exporting one. This
  passage is outside the two authoring named, and it is **forced** rather than
  swept in: the §run-gate-tests hole below cites this section for the seam's
  precedence and cannot cite a section stating the opposite.
- **`§run-gate-tests`, the `cd`-order ground.** *"because it enters the case dir
  before the source"* — backwards, against the section's own quoted script and
  `run_gate_tests.rs:422`. The **conclusion is kept** (the arm does read the case's
  seam) and the ground is replaced with the second source that actually produces it.
- **`§run-gate-tests`, the case-config claim.** Its content is **true** and is
  kept; its **normative force is inverted**. The sentence read as a sanctioned
  behaviour — a case config naming its own scratch "still wins" — which is exactly
  the per-gate opt-out the seam decision forecloses. It is rewritten as a **known
  hole**: unsanctioned, unclosed, filed, and explicitly not an extension point.

**Silence was the option to refuse, and the refusal is the reason this delta is
shaped this way.** Withdrawing the sentence outright would have left the SPEC
refusing to mint an opt-out while saying nothing about the one already live, and the
natural reading of an undocumented live capability is that it is available — which
is how a latent behaviour becomes a relied-on contract by attrition. Recording it
with the opposite normative force satisfies the instruction not to resurrect the
withdrawn promise: the withdrawn sentence promised the override as intended, and
this records the same fact as unintended.

**Closing the hole is not taken here.** It is a change to how the seam treats
*every* knob, not this one, so it wants its own unit and is filed to the gap inbox
rather than absorbed. A case config's other knobs — which the live fixtures do use,
`GATE_SDK_NATIVE_CRATE` and `GATE_SDK_CARGO_TARGET_DIR` among them — are unaffected
either way.

**Applied.** All three passages landed.

### (3) `--emit reads-census`: the `?` population, measured once by an oracle

A new non-gate arm prints every registry member declaring an undecidable walk
root, with its `?` count and the roots it does declare {design-bearing}.

`check-reads-couples` skips a root declared `?` (`reads_couples.rs:483-486`), and
the population of those skips is hand-swept. The entry records three sweeps in one
stage landing on 26, then 27, then at least 29, each miss caused by another
spelling of one idiom. **Re-measured here by oracle: 115 registry members, of
which 36 declare at least one `?`, across 51 `?` root-lines in total.** The arm
prints that, and the count is derivable from its lines rather than transcribed —
the same rule §check-reads-couples' own `--reads` report states, for the same
reason.

**What the arm reads is registry data and nothing else.** Field 2 of `GateEntry`
(`native/src/gates/mod.rs:134-141`) is the declared-roots slice, and it is already
answered per member by `--reads`; the arm is the roster form of a question the
binary answers one member at a time. No new config, no new registry field, and no
source parsing.

**The entry's own cheapness claim does not hold for the rest of its deliverable,
and that is recorded here rather than absorbed.** The entry asks for the `?` set
"together with its walk's root expression and filter argument", calling the arm "a
projection over data that exists". For the population that is true. For the other
half it is not: every `?` row in the registry is literally `("?", "", "")` — the
filter and prune fields are **empty**, because a member that could name them would
not be declaring `?`. The root expression and the filter argument exist only in
Rust source. **Delta 4 is what makes them reachable** — by a declared locator
rather than by an extracted copy — and this delta is the half that is a projection
over what the registry already holds. The two ship together: this one alone leaves
the Deliverable's other half unserved.

**The entry's "39" is a third quantity again, and it is unauditable today.** 39 is
the *decidable* subset — the `?` roots that in fact resolve to a static literal,
as against the kit-literal-fallback branches §check-reads-couples rules
legitimately undecidable (that section's *second half is the boundary* passage;
this amendment first cited it by line number and the line number was already
wrong, landing in §check-graph — which is the case against citing a moving file by
line at all). Separating those needs
exactly the classifier delta 4 would retire. No command in the tree reproduces 39,
and this amendment does not assert it.

**Build's measurement, and a fourth quantity this entry nearly shipped wrong.**
The arm reproduces authoring's figures exactly — 36 members, 51 `?` root-lines,
confirmed twice over by two independent routes, the arm itself and a per-member
`--reads` sweep. But **three denominators are live over one population and they are
not interchangeable**, and an entry whose whole subject is counts that were wrong
every time must not add a fourth:

- **51** `?` **root-lines**, member-expanded — a shared root const counted once per
  member using it. 25 inline + 26 from the three consts (2×10, 1×3, 3×1).
- **36** **members** declaring at least one `?` — 22 declaring inline, 14 using a
  shared const, and the **overlap is exactly zero**: no member combines a const
  with an inline `?`, and none uses two consts.
- **31** textual **declaration sites** in the registry source — 25 inline plus 6
  inside the consts — spread over 28 source lines, because three members carry two
  inline `?` tuples on one line each (`check-close-surfaces`,
  `check-shim-restatement`, `check-gate-substrate-parity`). 28 + 3 = 31 is the
  self-consistency check.

A count of *sites* is not a count of *members*, and adding one to the other is the
category error that produces a phantom overlap. §check-reads-couples carries the
warning so the next assertion over this population states which denominator it
counts.

**Applied.** The arm, its bridged-table registration, its two crate unit tests, its
bespoke cross-oracle test, and both SPEC halves.

### (4) Every `?` declares its ground, and the ground names where the walk is

A `?` root gains a required ground and a resolvable source locator, so the
decidable/undecidable split is **declared** rather than re-derived by reading Rust
source {design-bearing}.

**Decision of the intent oracle (the scope session), 2026-09-12, lead-routed:
ship delta 3 and delta 4.** Recorded with its class, because a decision may be
revised by a later session where a direction or a consult ruling may not, and
writing it up as either would wrongly harden it.

The shape: field 2's tuple gains a fourth element, the **ground** a `?` is
declared on, and a member declaring `?` with an empty ground **fails to
compile** — the same construction that makes the roots themselves un-omittable.
`--reads` gains the ground on a `?` line; `--emit reads-census` groups by it. Two
ground values are live today and §check-reads-couples already names both classes:
`fallback`, for the kit-literal fallback branch of a runtime-selected corpus
helper, which that section rules legitimately undecidable and files under its own
open semantics question; and `dynamic`, for a root genuinely computed at runtime.
A third value is deliberately **not** offered — a `?` that is in fact a hardcoded
literal or a positional with a literal default has no ground, because it should
be declaring the root. Under this delta such a member does not compile, which is
what converts the entry's recurring hand sweep into a build-time impossibility.

**The ground carries a source locator, and that is the delta's own defence against
being satisfied mechanically.** Each `?` declares `<class>@<path>:<line>` — the
class above, and the location of the walk the `?` stands for. The requirement
exists because of a fact about the corpus this amendment measured: **all 51 `?`
root-lines are the identical spelling `("?", "", "")` with zero counterexamples**,
which is one default replicated fifty-one times, not fifty-one considered
declarations. Build authors all of them in one pass, and a ground written in bulk
to satisfy a compiler reads as considered when it was mechanical — the failure
class this tree already carries an open entry about on the audit roster's
pre-stamp.

A locator is what closes that, where a prose justification would not:

- **It cannot be written without performing the classification.** You cannot name
  the line of the walk without finding the walk, and finding the walk *is* the act
  of deciding which class it is. The evidence requirement and the classification
  become one act rather than two, the second of which is skippable.
- **It is machine-checkable, where honesty is not.** A crate-side assertion holds
  each locator to resolving, and to resolving on a line that calls a member of the
  root-entry roster §check-reads-couples keeps for its recursive-walk refusal —
  wherever that line lives. That is presence and placement, which a gate can
  decide — deliberately *not* whether the class is correct, which it cannot, and
  which is the shape §check-queue-entry-budget's own refusals name as the standing
  failure.
- **It is a pointer, not prose.** One token per site, no restatement, no
  fifty-one-paragraph tier to maintain. Content-tiering applied to a registry
  field.
- **It answers the Deliverable's other half better than an extractor would.** The
  entry asks for each `?` "together with its walk's root expression"; a locator
  *points at* the expression instead of transcribing it, so no copy exists to
  drift. Derivation-first, and it retires the static-analyser route a previous
  iteration already measured wrong three times running.

**A locator off its site's home module carries one `via` clause, and the grammar
requires it.** A site's *home module* is the module of its member's registry
dispatch function; a site inside a shared root const has none, the const being no
one member's. The ground is `<class>@<path>:<line>` where the walk line lies in the
home module, and `<class>@<path>:<line> via <symbol>` otherwise — `<symbol>` being
the first `<module>::<fn>` the home module calls on the way to the walk line, or
`const <NAME>` for a site inside that const. The clause is **forbidden** in-module,
so each situation has one spelling. The locator assertion reds on a clause missing
off-module or present in-module — its presence is a function of the locator path,
so it is derived rather than trusted — on a `<module>::<fn>` with no definition or
not referenced by the home module, and on a `const` site outside the named const's
body.

**Why required.** Placing the locator on the walk line ties it to a walk and no
longer to its member; the `via` clause restores that tie on the first hop rather
than the last, which keeps the placement rule exemption-free. For
`check-enforcement-fresh` and `check-value-rollup-fresh` it is the only record of
the call chain in the tree: their only fixture case steers the gate off the emitter
through a positional, so unit test A observes no walk for them. **Its honest
limit:** the assertion resolves the symbol and its first-hop reference, never the
chain from symbol to walk line, which would be a call-graph analysis — so for those
two members the clause records the chain rather than proving it.

**Why the grammar and not the Definition of Done alone.** A Definition of Done item
is checked once, at the landing commit, and leaves the next off-module `?` a later
port adds unconstrained. In the grammar, the locator assertion holds every future
site — the fix and the gate that catches it land in one unit.

**Why this is the nearest satisfiable form of the entry rather than a widening.**
The Deliverable as written cannot be satisfied at all — there is no root expression
or filter argument in the registry to enumerate, every `?` row being empty in both
fields. So delta 3 alone is not the cheap version of this unit along the same axis;
it is *further* from the Deliverable, and it leaves the entry live to be re-ranked
at the next boundary while the iteration pays the fixed cost anyway. The envelope is
"measured once by an oracle rather than re-swept per session", and a compiler is an
oracle.

**It also leaves the entry's own evidence verifiable.** The entry argues census
unreliability from 26/27/29 against 39. This authoring measured 51 and 36, and
nothing in the tree reproduces 39. Under delta 3 alone, an entry about census
unreliability would keep an uncensused headline number; the ground partition is
what finally produces a figure of the class 39 was meant to be.

**Measured at the source before it landed.** The batch that landed deltas 2 and 3
measured this delta before starting it, and stopped on what the measurement found
rather than half-landing it. The measurement stays recorded here, with the answers
to its two findings below it and the landing after those.

**The declaration unit is the source site, not the member** (lead-routed decision,
revisable here). The per-member rule was written against 51, and 51 is a count of
member-expanded root-lines; the authored population is the **31 textual declaration
sites** — 25 inline plus 6 inside the three shared consts. Demanding ten separate
grounds for one shared `MANIFEST_ROOTS` walk would have manufactured the
fake-consideration failure the locator exists to prevent: ten authors writing ten
grounds for one fact. The shared-const sites are already the `fallback` class
§check-reads-couples names, so this aligns the declaration unit with a
classification that section already carries — convergence, not widening.

**Measured cost, at the source.** The arity change is the dominant term and it is
larger than this section assumed: field 2's element type is spelled inline with no
alias, so a fourth element respells it in 6 declaration sites and rewrites **64**
tuple literals across 58 lines — not 51, because 33 ordinary rows with nothing to
say must still be rewritten to say nothing. Six destructuring sites widen with it
(`--reads`' emission loop, the gate's consumption loop, and four registry unit
tests). Call it 70 existing lines across three files, plus the ground emission on a
`?` line and a new locator-resolution assertion of the size unit test C already
carries. Authoring the 31 grounds is **not** uniform: 13 sites are a one-line
lookup because the declaring module holds exactly one walk; 15 require matching N
`?` slots against M ≠ N walks and deciding which walk each `?` stands for; 3 are
resolved below — one conversion out of the root set, and two off-module sites.
Around 26 files are opened. The direction the per-source-site unit buys is real —
31 authored grounds rather than 51, the consts' walks read once rather than per
use — but this is not a session's remainder.

**Both findings build recorded are answered, and neither answer is a ruling.**

- **Placement: a locator lands on a line that calls a root-entry roster member,
  wherever that line lives** — decision of the intent oracle (the scope session),
  2026-09-12, lead-routed, revisable here. The member-module clause is dropped with
  **no exemption list**. The roster is the one §check-reads-couples keeps for its
  recursive-walk refusal, cited rather than restated, so a walk entry point added
  there reaches this assertion with no second edit. Refused: "or a module it
  transitively calls", which makes placement near-vacuous, and an exemption list for
  the shared sites, which is the declared exemption this gate family refuses. What
  the dropped clause carried — the tie from a locator to its member — moves to the
  `via` clause above.
- **Conversion: a `?` standing for a read outside the analyzed class leaves the root
  set rather than taking a ground.** §check-reads-couples already rules that an
  empty root set "means the member performs no walk in the analyzed sense", and that
  `git ls-files` enumeration, single-file reads and a single-level listing each fall
  outside that class. `check-hook-exec-bit` enumerates through `git ls-files -s` and
  calls no root-entry roster member, so it converts to the empty root set, on the
  precedent `check-rule-citation` already takes. It is not the re-scan-over-`ls-files`
  evasion that section refuses: the member's subject is the index's mode bit, so the
  index is its correct source. A `?` of `check-close-surfaces` whose only read is a
  `list_dir` leaves that member's root set under the same rule.
  **`check-enforcement-fresh` and `check-value-rollup-fresh` do not convert, and the
  earlier finding misread them.** The positional with a literal default names the
  projection they compare against, not their read set: on the default branch both
  reach `find_with_prune` in `native/src/emit/enforcement_map.rs`, a recursive walk
  rooted at the `GATE_SDK_ENFORCE_SCAN_DIR` knob. Their `?` is correct, their ground
  is `dynamic`, and their locators are off-module sites. An empty root set for them
  would be a false no-walk claim unit test A cannot catch, their only case steering
  off that branch — the opt-out this gate family refuses, spelled as a declaration.

**The answer text carried `check-footprint-fresh` as a second precedent for the
empty root set, and build dropped it on a probe rather than landing it.** That
member's emitter calls `walk::glob_files` — a root-entry roster member the recorder
notes — at `native/src/emit/footprint.rs:31` and `:90`, and both of its fixture
cases pass the two positionals that steer it off the emitter. Its `&[]` is
therefore the same false no-walk claim the bullet above refuses for the enforcement
pair, not a precedent for `check-hook-exec-bit`'s. Correcting that member's own
declaration is outside this delta's sites and is escalated, not absorbed.

**Applied.** Every `?` declaration site carries a ground in the locator grammar, a
const assertion refuses a `?` with no classed ground at compile time, and the
locator assertion is a registry unit test with a seeded-bad sibling. Build ran the
conversion rule over **every** site rather than the three the answers named, and it
reached further, each by a rule this delta already states:

- **Two more conversions out of the root set, one a member and one a slot.**
  `check-shellcheck` leaves it by **mechanism**: `walk::glob_entries("<dir>/*.sh")`
  (`shellcheck.rs:63`) expands one directory level per pattern component — a
  `read_dir` per metacharacter component and no globstar (`walk.rs:811-835`) — so
  it is a bounded single-level expansion, outside the recursive class on the
  one-level pathname-expansion precedent §check-gate-exemption-tasks already takes.
  That the entry point is no root-entry roster member is **not** the ground: the
  roster is a floor, and a recursive walk behind an unrostered entry point would
  still owe a root. `check-docs-mirror-fresh` did **not** leave the root set — it
  keeps its declared root, `docs` filtered by the `SPEC.md,README.md,DOCTRINE.md`
  literal, for its own `find_with_prune` (`docs_mirror_fresh.rs:66`); what left is
  its **second** `?`, which stood for the mirror emitter's single-level `list_dir`
  (`docs_mirror.rs:192`).
- **One conversion to a declared root.** `check-docs-kit-parity` globs
  `*/index.md` under the directory of a positional whose default is the literal
  `docs/kits.md`, so it declares `docs` with the kit literal as its filter. Unit
  test C missed it because `dirname` is none of the root wrappers it enumerates —
  the floor that test's own honest limit names.
- **Calibrations inside the envelope, written into §check-reads-couples.** A locator
  path is **crate-relative** (`src/…`), so registry data carries no repo layout. The
  census gains a **fourth column** carrying the grounds, which makes the class
  partition a projection of the report rather than a second report. A `?` also
  refuses a filter or prune at compile time, which keeps `--reads`' second column
  unambiguous on a `?` line.
- **One classification was escalated, and it is settled by a third class.**
  `check-spec-embedded-source`'s `?` walk has a static root (a positional defaulting
  to `.`) and a filter projected out of `CANON_KIT_EMBED_LANGS`' packed elements —
  a filter the field cannot express, so the `?` stands, but the walk was neither
  class as first written. It first landed as `dynamic`, read as a walk whose *bound*
  is computed at run time; that reading separates nothing, since every knob filter's
  bound is computed at run time. A class names what would retire its `?`, so the
  site takes **`projection`**, retired when the filter grammar gains a form reaching
  a packed-field projection, and `dynamic` narrows to a root no literal names.
  §check-reads-couples admits `projection` as the one exception to the refusal of
  `?` on a statically resolvable root, bounded by its cause. Every `dynamic` site
  was then re-read against the narrowed definition: none but this one has a literal
  or literal-default root — each is a knob, a positional falling back to a knob, the
  kit roots, or a positional falling back to the git toplevel, which is not a
  literal default because a case's working directory is not its toplevel — so
  nineteen stay `dynamic` and none converts to a declared root.

The figures, each from its own oracle at landing: the locator assertion enumerates
**26** declaration sites — **17** in-module and **9** off-module; **19** `dynamic`,
**6** `fallback` and **1** `projection` — and `--emit reads-census` prints **32**
lines summing to **46** `?` root-lines.

### (5) `check-gate-substrate-parity` assertion I joins the crate registry to the battery registration

A ninth assertion compares the crate's dispatch roster with `scripts/gates.list`
in both directions, each residue member taking a recorded disposition
{design-bearing}.

Assertion B already equates the `.gate` descriptor set with the subcommand roster
(`gate-sdk/SPEC.md:12877-12879`), and the gate derives both in-process —
descriptors by globbing the resolve dirs, the roster from
`gates::names_with_owners()` (`gate_substrate_parity.rs:551-577`). **Neither path
reads `scripts/gates.list` at all**, which is the entry's premise and it holds at
the code. Re-measured at this authoring: 115 crate members against 113
registration lines, residue exactly `check-surface-duplication` and
`check-producer-liveness`, and the **reverse direction empty**.

Assertion I reds when the two rosters' symmetric difference is not exactly the
declared set of delta 6. Both directions are asserted, and the empty one is
asserted deliberately: a name in `gates.list` that the crate does not carry is a
battery that cannot run its own registration, which today no member exhibits and
which nothing would otherwise catch.

**It costs no new configuration.** `registry::list_path(gates_dir)`
(`native/src/registry.rs:55-57`) already derives the registration file's path from
the gates dir the gate already resolves, and the descriptor already carries
`scripts/gates.list` in its `couples=`, so the trigger is already correct and no
manifest edit is owed.

**Why the disposition is required rather than the divergence merely reported.** The
entry's measured cost is that "two members stay dispatchable and unrun with their
causes stated only in prose no reader joins". A bare report would leave the causes
where they are. Requiring a disposition is what moves the cause onto the surface
the assertion reads.

### (6) `gates.list` admits an `# unregistered: <name> — <reason>` declaration

The registration file gains one declaration form, so a deliberately unregistered
member records its cause in the consumer's own file {design-bearing}.

**The grammar is reused, not minted.** `# smoke-unregistered: <gate-name> —
<reason>` already exists for the same shape of fact — a member deliberately absent
from a roster, with a stated cause, both fields machine-read
(`gate-sdk/SPEC.md:8480`). This is that grammar on a second roster, and the
parallel is stated in the section so the two cannot drift into two dialects.

**The provenance seam decides where the cause lives, and it is the reason the
declaration is not a kit table.** Why *this* consumer does not register
`check-surface-duplication` (it declares no glossary, for the provenance-seam
reason) and does not register `check-producer-liveness` (its own validate battery
would red against its own lock) are **consumer facts about a consumer's tree**. A
kit SPEC holding them would publish one consumer's configuration as kit content,
and would rot the moment a second adopter registered either member. So the kit
owns the **grammar** and the assertion; the consumer's own `gates.list` owns the
**cause**. This is the `check-graph` / `graph-vocab.sh` pattern on a smaller
surface.

**No member set moves, checked rather than assumed.** Both readers of this file
filter comment lines already — `registry::members` (`native/src/registry.rs:12-18`)
and its shell twin `gates_list_members` (`gate-sdk/lib/gate.sh:216-218`) — so a
declaration line is invisible to every existing reader, and the 113-member count
is unchanged by adding them. The declaration is therefore additive to an existing
file format rather than a change to it.

### (7) The two live residue members take their declarations

`scripts/gates.list` gains one `# unregistered:` line per residue member, each
carrying the cause its prose site already states {mechanical}.

`check-surface-duplication`'s cause is `canon-kit/README.md:34` — "needs a glossary
(exits 2 without one)" — which is a **kit** statement and generic to any adopter,
so it stays where it is and the declaration states this tree's instance of it.
`check-producer-liveness`' cause is `scripts/lifecycle-config.sh:12`, whose live
clause is that "a registered liveness gate would red every validate run against
its own lock"; the pointer there is retained and the declaration carries the
consumer-side fact. Content tiering: the declaration is the machine-read surface
and owns the cause in one line; neither prose site gains a restatement, and
`scripts/lifecycle-config.sh:12` keeps its existing `# spec:` pointer unchanged.

### (8) An unknown `--emit` arm gets its own refusal, naming the arm roster and nothing else

`no_such_arm` replaces the gate-roster refusal on the emit path, printing the
`--emit` roster and citing the section that owns it {design-bearing}.

**Reproduced, not reasoned.** `normalize()` (`native/src/main.rs:283-289`) rewrites
`--emit <name>` to `--emit-<name>` with no validity check; `emit::lookup` misses;
control reaches `gates::lookup`, misses again, and lands in `no_such_gate`
(`native/src/main.rs:42-50`), which prints `gates::names_with_owners()` — the
crate's own 115-member registry, **not** `scripts/gates.list`'s 113 registration
lines; the two rosters delta 5 measures apart are conflated at this call site's
fallback, and the fix below is scoped to the roster it actually prints. It is
**byte-for-byte the same code path** a mistyped gate name takes — there is no
arm diagnostic in the binary at all. A reader who typed `--emit` is handed the one
roster that cannot contain what they meant.

After this delta the emit path carries its own miss. `normalize` already knows the
argv began with `--emit`, so the information needed to route the refusal is present
at the point of failure and no new state is introduced. `no_such_arm` prints the
live `--emit-` arm names off `BRIDGED_ARMS` the same way `names()` already does —
33 of them at this authoring, before this amendment's own delta 3
(`--emit-reads-census`) and, where it lands in the same batch,
`SPEC-compression-legibility.md` delta 2 (`--emit-entry-history`) each add one
more; the count is never transcribed, so neither addition is a further edit here
— cites `gate-sdk/SPEC.md §The non-gate arm` as the roster's owner, prints **no
gate roster**, and exits 2 unchanged.

**The exit code and the `help:` shape are unchanged**, so the output contract's
existing assertions are untouched: this delta changes which roster a `help:` line
carries, never whether there is one.

### (9) The output contract states that a refusal may cite a governing SPEC section

§Output contract gains one rule, which the tree already follows everywhere
{design-bearing}.

The entry frames delta 8 as deciding "whether a refusal may cite a SPEC section — a
coupling the gate-output contract does not currently carry". **That is right about
the contract text and wrong about the practice, and the correction is what makes
this delta cheap.** §Output contract (`gate-sdk/SPEC.md:946`) is silent on message
sourcing, while the coupling is already taken in live refusal paths across both
substrates: `native/src/gates/crate_arms.rs:21`, `guard-kit/lib/guard.sh:1578` and
`:1580`, `gate-sdk/bin/build-native.sh:23`, `:33` and `:107`, and
`gate-sdk/bin/run-consumer-smoke.sh:42`, `:111` and `:197`.

So the delta **states a rule the tree already keeps** rather than minting a
coupling: a `help:` line may name the governing SPEC section that owns the remedy,
and the citation is a **pointer, never a restatement** — the content-tiering rule
applied to refusal text, which is also what stops a message growing into a copy of
the section. The rule is stated because a silent convention is one a later reader
may read as an accident and remove.

### (10) The bridge's knob-probe refusal names the whole arm argv

`gate_knob_env`'s refusal prints the arm's full argv rather than its first token,
so the two-token `--emit <name>` form is not reported as `--emit` {mechanical}.

On the same mistyped-arm invocation a reader meets a *second* misleading line
before delta 8's: `gate_knob_env` (`gate-sdk/lib/gate.sh:400-409`) is called with
`$arm` = `--emit`, and its `printf` reports "the config bridge could not report
what **`--emit`** reads" — naming a token that is not the arm, at the one moment
the reader is trying to learn what the arm set is.

**This sits inside the entry's envelope rather than beside it**, and the reason is
stated because the opposite reading is available: the entry's subject is the
diagnostic **at the moment of miss**, and this line is the first thing that moment
prints. Fixing the second line and leaving the first would ship a corrected
diagnostic underneath an uncorrected one.

## Producers and consumers

- **The pinned bridge input** (delta 1).
  - *Producer:* `--run-gate-tests`' `resolve_argv`, exporting the value `setup`
    already computes at `run_gate_tests.rs:94`. Reachable in the deployed
    configuration: the arm runs on every kit fixture suite, which the commit-time
    battery invokes.
  - *Consumers:* `lib/gate.sh:48`'s resolution, which reads the bare name; then
    `_gate_knob_emit`, which derives `GATE_SDK_KNOB_GATE_SDK_TMP_DIR` from it; then
    `walk::knob_scalar` in the member. Named reader today:
    `check-crate-arms`' `cache_path` (`native/src/gates/crate_arms.rs:39-47`), at
    the source-stamp cache read and write.
  - *No new field:* the knob already exists and its value stays the consumer's for
    ordinary runs. What changes is which process sees the harness's pin.
- **`--emit reads-census`** (delta 3).
  - *Producer:* the crate's emit dispatch, over registry field 2 — data that
    already fails to compile when omitted.
  - *Consumer:* a session sizing the skipped population, and
    §check-reads-couples' cadence row, which currently names a population no
    command answers. Every emitted field has a named reader: the member name and
    the `?` count are read at the cadence review; the declared roots are read
    beside them to tell a wholly-undecidable member from a partly-declared one.
  - *No count line is emitted*, so no transcribed total can drift from the lines.
- **The `?` ground and its locator** (delta 4).
  - *Producer:* the registry entry itself, compile-enforced. Reached in the
    deployed configuration at the landing commit, since every live `?` declaration
    site takes one.
  - *Consumers:* `--reads`, on a `?` line; `--emit reads-census`, printing the
    grounds as a fourth column, from which the class partition is projected.
  - *Named reader of the refusal:* the compiler, at the member's registration —
    which is the point: a member with no ground cannot be added.
  - *Named reader of the locator:* a crate-side assertion, at the registry's
    unit-test pass, whose **red condition** is a locator that does not resolve or
    that resolves to a line calling no root-entry roster member, or whose `via`
    clause is missing off-module, present in-module, or names a symbol that is
    undefined or unreferenced by the home module. It does **not** red on
    a wrong class, which no gate can decide — stated so a later reader does not
    read the silence as coverage.
- **Assertion I** (delta 5).
  - *Producer:* `check-gate-substrate-parity`, at its existing roster derivation;
    the registration file is reached through `registry::list_path`, already in the
    crate.
  - *Consumer:* the battery, on every precommit run — the gate's existing tier.
  - *Red condition, named rather than its subject:* it reds when the symmetric
    difference of the two rosters is not exactly the declared set — so it reds on
    an **undeclared** residue member, on a **stale** declaration naming a member
    that is in fact registered, and on a `gates.list` name the crate does not
    carry. It does **not** red on the size of the residue, which is a consumer's
    business.
- **The `# unregistered:` declaration** (deltas 6 and 7).
  - *Producer:* the consumer, editing its own `gates.list`; delta 7 is the first
    deployed instance, so the producer is reached in this tree rather than only in
    fixtures.
  - *Consumer:* assertion I, at the comparison above. Both fields are read: the
    name joins the rosters, the reason is what the assertion requires to be
    non-empty.
  - *Every field has a named reader*, and there is no third field because there is
    no third reader.
- **`no_such_arm`** (deltas 8 and 10).
  - *Producer:* the crate's top-level dispatch, on the emit path `normalize`
    already identifies; and `gate_knob_env`, for the argv it names.
  - *Consumer:* the human or agent that mistyped the arm — the only consumer a
    diagnostic has, which is why its correctness is the whole deliverable.
  - *Reader of the roster it prints:* the same reader, at the next invocation.
- **Red conditions of the affected readers under a narrowing** (causal-completeness
  point 5). **One delta narrows a corpus and it is checked rather than argued.**
  Delta 1 moves where `check-crate-arms` resolves its cache, which narrows nothing
  it scans but does change a path it reads: its red condition is a failing `cargo`
  arm, never a cache hit or miss, so a relocated cache costs a re-run and cannot
  flip a verdict. Delta 6 adds **comment** lines to `gates.list`; both readers
  already filter them (`registry.rs:12-18`, `gate.sh:216-218`), so the member set
  is bit-identical and no member-count reader can move — this is the one place a
  "a narrower corpus only removes violations" reflex would have been wrong in the
  other direction, and it is why the two twins were both read. Deltas 3, 5, 8, 9
  and 10 are additive: a new arm, a new assertion, a new refusal branch, a stated
  rule, and a widened `printf` argument.
  - `check-gate-fixture-coverage` reds on a registered member with no `good/`+`bad/`
    pair. Delta 5 adds an assertion to an existing member rather than a member, so
    its derived set is unchanged; deltas 3 and 8 ship their own fixture pairs.
  - `check-gate-output` reds on a missing `: clean` or `help:` line. Delta 8 keeps
    both, and delta 9 states a rule about a `help:` line's *content*, which that
    gate does not read — recorded because the opposite assumption would make
    delta 9 look like a gate change.
  - `check-graph` reds when the generated hook diverges from the manifests. No
    `couples=` field changes, because the one the join needs is already declared.
  - `check-crate-arms` reds on a failing crate lint or test arm; every delta
    touching the crate lands its unit tests in the same commit.

## Existing sections updated

- `gate-sdk/SPEC.md` §run-gate-tests, the scratch-pin passage at 10653-10661
  (delta 1). It states which process the pin enters and why that is upstream of the
  bridge rather than beside it, and records that the bare and prefixed spellings
  are one pin reaching two substrates. **Applied.**
- `gate-sdk/SPEC.md` §Layout and configuration, the seam's precedence claim
  (delta 2). *"Env vars still win"* is corrected: precedence is the config file's
  choice, not the seam's, with the consequence that a harness cannot make a knob
  authoritative by exporting one. Cited by the §run-gate-tests hole below, which is
  why it is corrected in the same unit rather than filed. **Applied.**
- `gate-sdk/SPEC.md` §run-gate-tests, the `cd`-order ground (delta 2). The
  conclusion is kept and the ground replaced: the case's seam is read at
  `gate_command`'s knob-resolution re-source, which runs after the `cd`, not at the
  arm's own source, which runs before it. **Applied.**
- `gate-sdk/SPEC.md` §run-gate-tests, the case-config claim (delta 2). Kept as
  fact, inverted in force: recorded as a known hole, unsanctioned and filed, with
  an explicit statement that it is not an extension point. **Applied.**
- `gate-sdk/SPEC.md` §check-reads-couples (delta 3). The section gains the
  census arm as the answer to the population its cadence row asks for, and records
  the measured figures (36 members, 51 `?` roots) as of this authoring with the
  oracle that produces them, so a later reader re-runs rather than trusts. It also
  gains the **three-denominator warning** — that the member-expanded root-line
  total, the member count and the textual declaration-site count are different
  numbers over one population, and an assertion must say which it counts. That was
  not in the amendment as authored; build measured the three apart and the section
  carries it because conflating them is how the hand sweeps this arm replaces went
  wrong. **Applied.**
- `gate-sdk/SPEC.md` §check-reads-couples (delta 4). The three ground classes, each
  named by what retires its `?`, the refusal of a fourth, the bounded admission of
  `projection` against the refusal of `?` on a statically resolvable root, the
  locator's grammar with its `via` clause, the placement
  rule citing the root-entry roster, and the conversion rule's reach to a `?`
  standing for a read outside the analyzed class. **Applied**, with the census's
  fourth column, the `--reads` `?` line's ground column, and the three denominators'
  figures re-run at landing; the `--reads` roster row in §Meta-gate conservation
  for the binary substrate takes the ground in one clause.
- `native/src/gates/mod.rs`, the four-member registry comment above
  `check-hook-exec-bit` (delta 4). It grounds `?` in "a positional scan root the
  shell parser calls undecidable"; after `check-hook-exec-bit` converts it is
  **reworded, not deleted**, since it covers three other members. **Applied**, with
  the comments over the other converted members — `check-close-surfaces`,
  `check-shellcheck`, `check-docs-kit-parity` and `check-docs-mirror-fresh` —
  reworded in the same pass.
- `gate-sdk/gate-tests/reads-census.test.sh` (delta 4). Its `--reads` sweep counts
  a `?` line by its opening column rather than as a whole line, and it asserts the
  census's ground column against `--reads`. **Applied.**
- `gate-sdk/SPEC.md` §The non-gate arm (deltas 3 and 8). `--emit-reads-census`
  joins the roster; the section is named as the roster `no_such_arm` cites, which
  is what makes it a governed surface rather than documentation. **Delta 8's half
  applied**, in two paragraphs appended below the roster so the enumeration itself
  is untouched — it is the paragraph a sibling amendment's new arm edits, and a
  reworded enumeration would have turned two sequential edits into a conflict.
  **Delta 3's half applied**, as one clause inside the enumeration — the first edit
  to take the sequential shape delta 8 left that paragraph in. Delta 10's clause
  lands in the same passage.
- `gate-sdk/SPEC.md` §check-gate-substrate-parity (delta 5). Assertion I is added
  beside B with its both-directions statement, its red conditions, and the note
  that it reads the registration file through the existing path derivation so no
  knob is minted. **Applied**, with two calibrations the amendment left open and
  build ruled inside the envelope, both written into the section: the assertion
  takes assertion B's **scope clause** (unscoped it demands a declaration per
  unvendored member in any subset-vendoring consumer), and the **reverse arm** is
  scoped to registrations that dispatch to the binary (unscoped it reds every
  consumer's own shell gate). The entry's ground for the reverse arm — that
  nothing would otherwise catch it — does **not** hold, and the section says so at
  the passage: scoped correctly it reads the fault assertion B reads from the
  descriptor side, neither subsuming the other.
- `gate-sdk/SPEC.md` §Layout and configuration (deltas 6 and 7). The `gates.list`
  format gains the `# unregistered:` declaration, stated as reusing
  §Consumer smoke's grammar and pointing at it rather than restating it, and
  recording that both member readers already filter comments so no existing reader
  moves. **Applied.**
- `gate-sdk/SPEC.md` §Consumer smoke (delta 6). One clause: the
  `# smoke-unregistered:` grammar now has a sibling on a second roster, so a reader
  editing one knows the other exists. **Applied.**
- `gate-sdk/SPEC.md` §Output contract (delta 9). The refusal-citation rule, with
  its pointer-never-restatement limit. **Applied.**
- `scripts/gates.list` (delta 7) — the two declarations. **Applied.**
- `native/src/emit/run_gate_tests.rs`'s `# spec:` pointer at lines 91-93 (delta 1).
  It currently says the pin's scope "is the pair loop, never this process"; after
  delta 1 the pin also enters the resolving shell, which is a different and wider
  claim. **Applied**, and §run-gate-tests' own scope sentence corrected with it.
- `scripts/git-hooks/pre-commit` and `docs/check-graph.html` (deltas 5, 6, 7) — the
  generated projections. Regenerate with
  `bash gate-sdk/bin/gen-pre-commit.sh --write`, then
  `bash gate-sdk/bin/run-gates.sh --emit graph > docs/check-graph.html`.
- `docs/gate-sdk/SPEC.md` (all deltas) — the generated on-site mirror; regenerate
  with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write`.

**Deliberate non-updates, recorded so a reader does not go looking.**
`canon-kit/README.md:34` is unchanged: its statement is generic to any adopter and
is the kit's to make, so delta 7's declaration is this tree's instance of it rather
than a relocation. `scripts/lifecycle-config.sh:12` is unchanged for the same
reason on the other side — it owns why the gate is wired where it is, which is a
different fact from why this tree does not register it.

## Retired spellings

- `a case config naming its own scratch still wins` (delta 2) — the **spelling** is
  retired because it reads as a sanctioned behaviour, which the per-gate opt-out the
  seam decision forecloses is not. What it asserted is true and is **not** retired
  with it: the same fact is re-stated in §run-gate-tests as a known hole. No
  identifier is retired either; the spelling is a prose claim and the grep is over
  prose.

Delta 8 adds a **sibling** refusal for the emit path; the existing gate-path
refusal keeps its name and its behavior, so no identifier is retired there either.
Stated below the roster rather than as a bullet in it, because a bullet in that
roster declares a retirement and this is the opposite.

## Definition of Done

- [ ] **Causal completeness** — every new state, event and interface has a named,
      reachable producer and a named consumer; every new field has a named reader
      at a named transition.
- [ ] **Every `?` ground names its evidence, and the locator is checked by oracle** —
      every declaration site (the unit is the site, not the member and not the
      member-expanded root-line; §check-reads-couples names the three denominators)
      declares a ground in the locator grammar, its `via` clause included where the
      site is off its home module. The locator assertion holds every site to
      resolving on a line that calls a root-entry roster member, and reds on a
      seeded locator of each bad kind: an unresolved line, a line calling no roster
      member, a `via` missing off-module or present in-module, and a `via` symbol
      undefined or unreferenced by the home module. A ground with no locator does not
      compile, so the bulk pass cannot skip the step that makes it a classification.
      The in-module and off-module counts are **reported from the assertion's own
      enumeration** at the landing commit, never transcribed from this amendment —
      no pre-landing oracle reports the site denominator, which is why none is
      written here.
- [ ] **Every `dynamic` site is re-read against the narrowed definition** — a site
      whose root is in fact statically resolvable is re-grounded `projection` if its
      filter is beyond the filter field's reach, and otherwise converted to a
      declared root. Only `check-spec-embedded-source` is known to be the former; the
      rest were grounded against the wider reading and are not assumed to survive it.
- [ ] **The ground partition is reported with every class non-empty, or the
      landing states why.** Every site is the identical spelling today, so a pass
      returning every site in one class and none in the other is the bulk-authoring
      signature rather than a result — §check-reads-couples names every class as
      live, and an empty one is a finding to resolve before landing, never a figure
      to report. Every class is reachable on the corpus as it stands, measured:
      `COMMENT_SURFACE_ROOTS`' site is a kit-literal fallback branch,
      `check-memory-off`'s root set is computed from a knob's glob expansion with no
      literal anywhere, and `check-spec-embedded-source`'s third walk is a
      packed-knob projection, so the guard is satisfiable rather than aspirational.
- [ ] **A `?` for a decidable root or an out-of-class read does not survive the
      pass** — each site is checked against §check-reads-couples' two decidable
      shapes (a hardcoded literal root, a positional with a literal default) and its
      out-of-class reads, and each one found converts: **to a declared root** where
      the member walks from a static root, and **out of the root set** where the
      read is a `git ls-files` enumeration, a single-file read or a single-level
      listing (§check-reads-couples, the empty-root-set passage).
      `check-hook-exec-bit` converts to the empty root set; any `check-close-surfaces`
      `?` whose only read is a `list_dir` leaves its root set the same way.
      `check-enforcement-fresh` and `check-value-rollup-fresh` do **not** convert —
      their `?` stands for the emitter's recursive walk and takes an off-module
      ground. `--emit reads-census` is re-run after the conversions, and its line
      count and column-2 sum replace every figure this amendment carries.
- [ ] **The scratch fix is verified by the corpus, not by the diff** — delete
      `gate-sdk/gate-tests/check-crate-arms/good/.tmp/`, run the `gate_sdk` fixture
      suite, and confirm the directory does **not** reappear while the pinned
      location does. The pre-fix state is reproducible today and is the baseline.
- [ ] **Both spellings are exercised** — a `.gate` member and a `.sh` member each
      resolve the pinned scratch, so the fix is not silently one-sided in the other
      direction.
- [ ] **The census figures are the oracle's** — §check-reads-couples' recorded
      figures are re-run at the landing commit and corrected if the tree moved; a
      figure that disagrees with the arm is the arm's to win.
- [ ] **Assertion I reds on a seeded divergence** — a fixture pair in which a
      registered-but-unlisted member carries no declaration, and one in which a
      declaration names a member that is in fact registered.
- [ ] **No member set moved** — `scripts/gates.list` still yields 113 members from
      both readers after delta 7.
- [ ] **The diagnostic is read, not diffed** — `--emit <typo>` prints the arm
      roster, cites §The non-gate arm, and prints no gate name; and the knob-probe
      line above it names the whole arm argv.
- [ ] **Fixture pairs for every shipped assertion and arm** (gate-sdk/SPEC.md
      §Consumer smoke's kit-landing checklist).
- [ ] **Full battery green** (`bash gate-sdk/bin/run-gates.sh`),
      `bash gate-sdk/bin/build-native.sh`, and the gate-sdk, canon-kit and
      evidence-kit fixture suites.
- [ ] **Merged with no information lost** — each addition integrated into its proper
      canonical-spec section, not appended; the three stale seam passages (one in
      §Layout and configuration, two in §run-gate-tests) re-grounded in place rather
      than left beside their corrections, and the live override recorded as a hole
      rather than deleted or promoted to a capability.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`). No sibling amendment is in flight for
      gate-sdk, so the none-remain half is satisfiable at this amendment's own
      merge commit.
- [ ] **Queue transition at the merging build batch, not at close** — the drain
      stage's entry refuses a non-empty active queue, so the batch merging this
      amendment moves all four entries in the same commit.
- [ ] **Removals propagated** — `## Retired spellings` above holds, re-run by
      `check-amendment-retired-spelling`.
- [ ] **Gaps filed** — a cross-component gap found during the work is resolved that
      session, not deferred.
