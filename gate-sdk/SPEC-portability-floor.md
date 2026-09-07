# SPEC amendment: portability-floor

**Nothing keeps a GNU-only shell construct off the adopter install path.** The one live instance
— `find -printf` in `installer/lib/init.sh` — sat there from the day the loop was written and was
found by a hand survey, then fixed. Nothing catches the next one, and the macOS leg that would
otherwise catch it is now binding, so the next one reds `master` instead of a contributor's
terminal. This amendment mints `check-portability-floor`: a born-native blocklist gate over the
files an adopter's machine actually executes, whose construct vocabulary is consumer config and
whose declared exceptions are marked at their sites.

**Two premises the queue entry carries are false against the tree, and the amendment is built on
the corrected ones rather than on the entry's.** Both were re-probed at HEAD, and the census
behind them is filed at `.workflow/survey-record.md` under *Which GNU-only shell constructs are
live on the adopter install path*.

- The entry says *"guard-kit already ships blocklist-roster mechanism"*. It does not. guard-kit
  has no `checks/` directory and registers no gate; `GUARD_KIT_RO_BINS` and
  `GUARD_KIT_APPEND_BINS` classify bash **command words** for the permission steering hook and
  never scan file content. The real precedent is gate-sdk's own `check-commit-msg` /
  `check-tree-terms` pair, which resolve one POSIX-ERE-per-line `.list` through the shared
  `gate_msg_pattern_files` helper — and gate-sdk/SPEC.md §check-tree-terms already rules that
  the second consumer of that shape reuses the resolver rather than re-deriving it.
- The entry names four corpus roots as *"the files `init` actually reaches"*. Under its own
  criterion the roster is short by one: `installer/lib/doctor.sh:9` sets `FLOOR` to the payload's
  `context-kit/lib/toolfloor.sh` and runs it on the adopter's machine, and that file carries a
  live `sort -V` at `:51`. A corpus drawn by directory rather than by reachability is the same
  mistake that let the `find -printf` sit unseen.

**What this amendment asserts, and what it deliberately does not.**

- **Asserted** — the four deltas below, all checkable by the battery and the gate's own fixture
  pair on this host.
- **Not asserted** — that the construct vocabulary is complete. A blocklist is a roster of what
  someone thought to name, and a gate over one buys *the next instance of a named construct*, not
  portability. The section says so in its own text rather than letting a green read as a proof.
- **Not asserted** — anything about the macOS leg's package set. That is
  `macos-leg-brew-set-vs-documented-requirements`, cut from this iteration on sizing; it shares a
  trigger with this unit and not a mechanism, and no delta here reaches it.

**The gate lands green, and saying so up front is the honest framing.** The census over the
corrected corpus returns six live sites and **every one is already declared** in
`docs/install.md` §Requirements: `sort -V` at `installer/lib/init.sh:70` and
`context-kit/lib/toolfloor.sh:51`, `realpath --relative-to` at
`gate-sdk/bin/gen-pre-commit.sh:33` and `gate-sdk/lib/gate.sh:563,575,616`. Nothing else in the
corpus matches any construct the census swept. So this gate finds no defect on the day it lands;
it is preventive, which is what the entry's own cost line describes and what enforcement-first
asks for on a unit that shipped its fix without its guard.

## What changes

### (1) `check-portability-floor` — a blocklist gate over the adopter install path

gate-sdk mints a born-native gate asserting that no file on the configured install-path corpus
matches a configured banned-construct pattern, except at a site carrying a declaration valve
{design-bearing}.

- **Descriptor** `gate-sdk/checks/check-portability-floor.gate`, on the shipped manifest grammar:
  it couples the pattern file, takes `dir=one`, `valve=none` and `tier=precommit`, and triggers on
  every commit, plus its `# install:` and `# spec:` lines. Registered in `scripts/gates.list` and
  in `native/src/gates/mod.rs`'s `REGISTRY` beside its module. It couples the **pattern file** and
  not the corpus, for the reason §Producers and consumers gives: corpus membership is a knob a
  consumer may legitimately empty, and a coupling glob matching nothing is a red.
- **Module** `native/src/gates/portability_floor.rs`. Born native, no shell form authored: a
  crate-carrying tree births a gate native by default and none of the exception classes at
  gate-sdk/SPEC.md §The port-candidate criteria applies — the corpus reaches no gate declaration
  path, so criterion 4 clears, exactly as it did for `check-tree-terms`.
- **Verdict.** Exit 1 on a matching site with no valve, reporting `path:lineno:line` and the
  pattern that matched. Exit 2 on the fail-closed set: a corpus member that is not readable, a
  pattern file that is not readable, and a pattern the ERE engine refuses at compile — the same
  compile-time refusal `check-tree-terms` takes, so a GNU escape (`\b`, `\s`, `\w`) in a
  portability blocklist is itself refused, which is the joke the gate has to survive rather than
  make.
- **Self-exemption.** Any file whose basename matches the pattern-file's own name is skipped, on
  `check-tree-terms`'s precedent: a roster of banned constructs necessarily spells them.

**Reuse rather than a new resolver.** The pattern-file resolution is `gate_msg_pattern_files`'
shape and the module reads the resolved arrays across the config bridge exactly as
`native/src/gates/tree_terms.rs` does. What is **not** shared is the roster itself: a separate
knob pair, because the two rosters answer different questions — one is a privacy leak-ban over
every tracked file, this is a portability floor over a named execution path — and folding a
portability pattern into `msg-patterns.list` would make a `master`-reddening leak and a BSD
incompatibility indistinguishable in one file, on one corpus, under one verdict.

### (2) The construct vocabulary and the corpus are both consumer config

Neither the banned constructs nor the corpus is a kit literal; both are knobs the kit defaults to
*disabled* and this repo supplies {design-bearing}.

- **`GATE_SDK_PORTABILITY_PATTERNS`** — whitespace-separated pattern files, defaulting to
  `<gates-dir>/portability-patterns.list`. gate-sdk ships
  `gate-sdk/templates/portability-patterns.list` as a copyable starting roster, on
  `templates/msg-patterns.list`'s model.
- **`GATE_SDK_PORTABILITY_PATHS`** — whitespace-separated pathspecs naming the install-path
  corpus. **Kit default: empty.**

**Absent config disables the assertion; it does not fail closed, and the divergence from the
sibling is deliberate.** `check-commit-msg`'s tracked pattern file is *required* and missing it is
exit 2. Here an absent pattern file or an empty corpus makes the gate assert nothing and exit
clean, naming the absence in its detail line — the `check-graph` / `graph-vocab.sh` degradation,
which is the pattern CLAUDE.md §The provenance seam names for exactly this case. The ground is
that the kit cannot know any consumer's install path, so a fail-closed default would red every
adopter's first commit on a roster only their project can write. **The honest limit, stated
because the degradation is the whole risk:** an adopter who deletes the roster silently disables
the gate, and no gate catches that. What bounds it is the detail line — a clean verdict that says
*no corpus is configured* is a different sentence from one that says *no violation was found*, and
a reader of a passing battery can tell them apart.

**This is the provenance-seam ruling for the unit, stated rather than implied.** A construct list
is a vocabulary; a kit literal spelling one publishes it, and a kit literal spelling *this*
project's install path publishes the layout too. So: **kit mechanism** is the scanner, the valve
grammar, the resolution and the two knobs; **consumer config** is the pattern roster and the
corpus roster; and **nothing is private rule content** here — this repo's roster is generic
portability knowledge and is tracked, with no `.local` sibling minted, because a construct name is
not an identity.

### (3) A declared construct is marked at its site, not remembered

A corpus line matching a banned pattern is clean when it carries
`# portability-declared: <reason>` on that line or the one above; the reason is mandatory
{design-bearing}.

This is the delta that decides what the gate is *for*, so the alternative it refuses is worth
naming. The cheap design is to leave `sort -V` and `realpath --relative-to` out of the blocklist
entirely — then nothing matches, nothing needs a valve, and the gate still catches `find -printf`.
It is refused because it buys the smaller half. Under it, a **new** use of `sort -V` in a file
that never had one passes silently, and `docs/install.md` §Requirements and the code drift apart
with nothing between them — which is the same two-hand-maintained-lists shape this iteration
declined to take on elsewhere. With the valve, the declared constructs stay *in* the blocklist,
every live site carries a marker naming the declaring surface, and adding a seventh site is a red
until its author has been to §Requirements. The declaration becomes a coupling instead of a
memory.

The window — the line or the one above — and the mandatory reason are the conventions the tree
already uses for `update-target-exempt` and `comment-tier-exempt`; nothing new is invented, which
is why the grammar is stated in one sentence and not designed here.

The six live sites take their markers in this delta, each naming `docs/install.md` §Requirements
and what the construct is doing there.

### (4) The gate's fan-out is regenerated and its roster memberships land with it

Every projection a new gate stales is regenerated in the landing unit {mechanical}.

`docs/site-architecture.md` §Generated projections and their freshness gates owns the roster and
each row prints its own command on red; the members a new gate touches are the on-site SPEC
mirror, `docs/enforcement.md`, `docs/value.md`'s rollup block, `docs/check-graph.html`,
gate-sdk's `README.md` gate-roster block, gate-sdk's `smoke/install.sh` expected-gate roster, the
`ported-gate-members` measured claim in `docs/install.md`, and — this gate being `tier=precommit`
— the generated pre-commit hook. **The two hook-and-binary regenerations are staging-ordered**:
both derive through `git ls-files`, so the unit stages its new files first and regenerates second.

The fixture pair is the gate's oracle, and each case carries a stated job. `good/` holds a corpus
member with a declared construct under its valve, a member with none, a pattern file with comments
and blanks, and the self-exempt roster itself. `bad/` holds an undeclared construct, a declared
construct whose valve carries an empty reason, and a valve one line too far away — so each arm has
an executable statement rather than a shared one. `check-portability-floor.test.sh` holds the
fail-closed exits and the two disabled-by-empty-config cleans, which a one-pair harness cannot
spell.

## Producers and consumers

**New interface: the two knobs (delta 2).**

- **Producer.** `gate-sdk/lib/gate.sh` resolves both, pre-declaring each to its default before the
  conditional read, and bridges the resolved values as `GATE_SDK_KNOB_*` — the identical path
  `GRAPH_LAYER_RULES` and `GATE_MSG_PATTERN_FILES` already take. **The enabling configuration is
  actually set**: `scripts/gate-sdk-config.sh` declares this repo's corpus roster and
  `scripts/portability-patterns.list` its vocabulary, both in the landing commit, so the producer
  is live in this tree and not only in fixtures.
- **Consumer.** `native/src/gates/portability_floor.rs`, reading the resolved arrays through
  `walk.rs`'s `knob_array`. The compiled member receives values and never paths, which is why
  neither knob is declared by the crate — the same reason `gate-sdk/lib/gate.sh:150` gives for
  `GATE_SDK_GRAPH_VOCAB`.
- **Every field has a named reader.** `GATE_SDK_PORTABILITY_PATTERNS` is read by the pattern
  compiler; `GATE_SDK_PORTABILITY_PATHS` by the corpus walk. There is no third field and no
  field that only the fixture reads.

**New state: the `portability-declared` valve (delta 3).**

- **Producer.** A comment written by hand at the site, by the author of the construct's use. Its
  enabling condition is nothing but the file being in the corpus.
- **Consumer.** The gate's per-line check, at the exempt-window test, before the violation is
  recorded. Its **reason field's reader is the human** reviewing the diff that adds a valve — and
  that is the field's whole point, so it is mandatory and an empty one is a violation rather than
  a pass. Stated because a field whose only reader is a person is the field this check exists to
  challenge, and the answer here is that a valve nobody has to justify is a valve everybody uses.

**Reader red conditions, enumerated because delta 2 can narrow a corpus.**
`GATE_SDK_PORTABILITY_PATHS` is a roster, and shrinking it is a narrowing — so the readers are
named by what makes them **red**, not by what they are about. `check-portability-floor` itself
reds on **finding a match**, which is monotone in the violation set: a narrower corpus can only
remove violations, and it can be cleared by inspection. The non-monotone reader is the one that
does not exist yet and is refused in the next paragraph. `check-graph`'s coupling assertion reds
on a `couples=` glob matching nothing, so pruning the pattern file out of the tree would red it —
which is why the descriptor couples the pattern file and not the corpus, whose membership is a
knob a consumer legitimately empties.

**A coverage-floor assertion is refused, and it is refused on the same point.** The tempting
second arm — *every corpus root resolves to at least one file* — reds on a **zero count** and is
therefore non-monotone: a consumer legitimately narrowing their install path to one file would go
red for having narrowed it. The gate asserts the decidable half and leaves roster completeness to
review, which is the same line `check-amendment-update-target` draws for its own roster.

**Existing integration prose describing the prior flow, updated rather than left to drift.**
`docs/install.md` §Requirements today *states* the GNU dependency in prose, as the surface a reader
consults. It becomes the surface a **gate** points at: the section gains the sentence that a
construct named there is what a `portability-declared` valve cites, so the declaration and the
enforcement are one fact with one owner rather than two lists that agree by habit. `CLAUDE.md`
§Conventions established in gate-sdk needs no edit — the gate takes `<KIT>_<KNOB>` with this
repo's layout as the defaults, which is the convention as written.

## Existing sections updated

- `gate-sdk/SPEC.md` — a new `### check-portability-floor` section carrying the invariant, the
  two knobs, the degradation-on-absent-config ruling with its honest limit, the valve grammar, the
  fail-closed set, the refused coverage-floor arm, and the born-native declaration (all deltas).
- `gate-sdk/SPEC.md` §Layout and configuration — the knob roster gains both knobs with their
  defaults (delta 2).
- `gate-sdk/SPEC.md` §check-tree-terms — the sentence naming the pattern-file mechanism's
  consumers, which is no longer a pair (deltas 1 and 2).
- `gate-sdk/README.md` — the `<!-- gate-roster:begin -->` block (delta 4).
- `docs/install.md` §Requirements — the declared-construct paragraphs at `:103-107` and
  `:179-186`, which gain the coupling sentence naming the valve that cites them (delta 3).
- `installer/lib/init.sh`, `context-kit/lib/toolfloor.sh`, `gate-sdk/lib/gate.sh`,
  `gate-sdk/bin/gen-pre-commit.sh` — the six declared sites, each gaining its valve (delta 3).
- `scripts/gates.list`, `scripts/gate-sdk-config.sh`, `scripts/portability-patterns.list`,
  `gate-sdk/checks/check-portability-floor.gate`, `native/src/gates/mod.rs` — registration and
  this repo's own config (deltas 1, 2 and 4).
<!-- update-target-exempt: the roster is generic over new gates and this one introduces no new projection, so an entry naming it would be the maintained-roster anti-pattern the section refuses -->
- `docs/site-architecture.md` §Generated projections and their freshness gates — **deliberately
  untouched**: it already carries the new-gate fan-out and this gate adds no projection class.
- `TASK-QUEUE.md` — `gnu-ism-on-adopter-install-path-ungated` promotes to New Features with this
  file's `[spec:]` ref, in the same commit as this file (all deltas).

## Definition of Done

- [ ] **Causal completeness** — both knobs have a resolving producer whose enabling config this
      repo actually sets, a named compiled consumer, and no field without a reader; the valve's
      human reader is named rather than assumed.
- [ ] **The seam holds** — no construct and no path is a kit literal; the shipped template is a
      copyable starting roster and the kit default is disabled.
- [ ] **Absent config is a stated degradation, not a silent one** — the clean verdict distinguishes
      *nothing configured* from *nothing found*, and the SPEC carries the honest limit.
- [ ] **Every declared site carries its valve** — all six, each naming `docs/install.md`
      §Requirements and what the construct does there, with a non-empty reason.
- [ ] **The gate is green on this tree and its fixtures prove each arm** — `good/`+`bad/` pair plus
      the `.test.sh` for the fail-closed exits and the disabled-config cleans.
- [ ] **The fan-out is regenerated, staged in the right order** — every freshness gate green
      without a hand edit to any generated file.
- [ ] **No completeness claim is made** — neither the SPEC section nor `docs/install.md` says or
      implies that a green run means the install path is portable.
- [ ] **Battery and crate green** — `bash gate-sdk/bin/run-gates.sh` plus
      `bash gate-sdk/bin/build-native.sh`, neither discharging the other, and gate-sdk's fixture
      suite clean.
- [ ] **Merged with no information lost** — each addition integrated into its proper canonical
      section, not appended.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component
      (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — grepped for the retired claim that the pattern-file mechanism has
      exactly two consumers; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps found during the work filed to the gap inbox.
