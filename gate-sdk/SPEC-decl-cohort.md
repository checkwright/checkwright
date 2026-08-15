# SPEC amendment: decl-cohort

The tenth port cohort — **three members, and what it buys is the release-note
declaration family off the shell substrate.**

`check-release-bump`, `check-tightened-gates-grammar` and
`check-tightened-gates-note-parity` share one corpus derivation: all three draw
only from `gate-sdk/lib/declaration.sh` and all three walk the release-note
directory with the identical `nullglob` idiom. All three are **consumer-declared**,
so this cohort is also `consumer-gate-port-disposition`'s **first tranche, 3 of
13** — and the first-mover design that tranche owns is authored beside this file
in `SPEC-consumer-first-mover.md` rather than here, because every later tranche
inherits it and this cohort merely happens to be the first to need it.

**Operator-ruled 2026-08-15 at scope**, under the selection rule's documented
blocker-retiring override (§The first cohort, and the rule that selects the next),
never by member count: `--group`'s **size arm is exhausted a second consecutive
time**. The scoping session's run reported the largest *takeable* derivation group
at one member, and the tool's exact-set key **split this cohort's shared
derivation into three singletons** — which the selecting session adjudicated into
one cohort, the adjudication §The first cohort explicitly reserves to it (*"a group
whose members' corpora visibly diverge is a finding the selecting session
adjudicates, never a cohort the tool cut"*, read here in its mirror image: a
derivation the key split, rejoined). The union of the three keys is one 59-line
library and one walk, and `check-release-bump.sh` states the sharing in its own
`# spec:` line.

**Two rulings are carried forward from scope so this amendment does not re-derive
them, and a build session does not re-open them.**

- **Criterion 6:** `gate-sdk/bin/upgrade-smoke.sh` keeps calling
  `lib/declaration.sh`, so the shell caller set does **not** empty and the library
  stays **dual** — the `spec_manifest_files` disposition, never the seventh
  cohort's delete-the-shell-form outcome. What the dual disposition then owes is
  delta 5, and it is owed *because* of this ruling rather than in spite of it.
- **The provenance seam, ruled clean:** the library parameterises its section name
  and its token grammar is generic, so grammar crosses into the crate and
  vocabulary does not; the four section names the gates pass it are **published**
  (docs/install.md §The upgrade contract), not private rule content. Delta 2
  states where the resulting line falls inside the crate.

## What changes

### 1. The cohort's selection evidence is recorded, from a run at the cut

The build session re-runs `bash gate-sdk/bin/port-blockers.sh --group` at cohort
cut and records **the members, the group keys they were split across, the
undecidable count, the largest takeable group's size, and the union key the
selecting session adjudicated** in the cohort's spec section. **[mechanical]**

The last item is this cohort's addition to the recording contract and it is the
one that makes the section legible: an override cohort whose members the tool
reported as three separate singletons will otherwise read, to a later selector, as
three coincidences. The figures in this amendment's preamble are the *scoping*
session's dated run and are the ground for selecting the cohort, not the evidence
the section records — the tool's output is not stable across tree changes, and the
undecidable count is the bound on any claim about size.

### 2. The declaration grammar lands in the crate as a bounded module, and the shell library stays

`native/src/declaration.rs` lands as the cohort's engine, on the `ere.rs` /
`json.rs` precedent: a module whose public surface is bounded and named here, so a
later port extends it deliberately rather than by accident. **[design-bearing]**

- **`section_bullets(text: &str, section: &str) -> Option<Vec<&str>>`** — the
  container arm alone. `None` is the *section absent* case, which the shell
  reports as a status rather than a value.
- **`section_tokens(text: &str, section: &str) -> SectionVerdict`** — the markdown
  arm's verdict, where `SectionVerdict` is `Absent`, `ExplicitNone`,
  `Tokens(Vec<String>)`, `Unparsed(Vec<String>)`. **The trichotomy becomes a type
  rather than a status code, and that is the one deliberate improvement in this
  delta.** The shell reports it as status 2 / status 0 with empty stdout / status 0
  with tokens / status 1 with the offending lines, and every caller re-derives the
  resolved-empty case by testing stdout emptiness *after* branching on status —
  the shape that made the silently-empty declaration possible in the first place
  (§lib/declaration.sh). A caller that cannot reach `Tokens` without matching
  `ExplicitNone` cannot make that mistake. The shell keeps its status protocol
  unchanged; this is the compiled form expressing the same trichotomy in the
  substrate's own terms, not a semantic change, and delta 5 is what holds the two
  to that claim.
- **`record_tokens(text: &str) -> Result<Vec<String>, Vec<String>>`** — the record
  arm; `Err` carries the malformed lines. The shell's *missing file is the empty
  set* rule stays at the **caller**, so the module is pure over text and has no
  filesystem reach of its own.

The module carries **no section name and no gate name**: both are arguments,
exactly as in the shell library, which is where §lib/declaration.sh already puts
the seam. The four published section names live in the three gate modules that
pass them, which is this consumer's rule content sitting in this consumer's rules.
The check this reduces to: a grep across `native/src/declaration.rs` for
`Tightened gates`, `Renamed knobs`, `Behavior changes` and `In brief` returns
nothing.

There is no writer, no renderer and no section-discovery API; adding one is a
design decision with its own reader rather than an omission to fill in.

### 3. The version comparator is defined over a stated grammar, and a token outside it is a loud refusal

`check-release-bump` orders versions with `sort -V` in four places — the
`<version>\t<file>` note rows, and three two-element maxima over bare versions.
The crate implements ordering over a **stated grammar** instead:
`<major>.<minor>.<patch>`, each a run of ASCII digits, with the caller stripping a
leading `v` as it does today; comparison is field-wise numeric, and the row form
ties on the path's byte order. A token outside that grammar is **exit 2** naming
the token, the file or disposition line it came from, and the grammar.
**[design-bearing]**

**Operator-ruled 2026-08-15 at spec: ships as authored — option (a), the
refusal stands, and the ordering ruling stays deferred to the session that
first cuts a prerelease.**

**The ground is that a faithful port and a correct one are different programs
here, which is the fact that forecloses the two obvious dispositions.** Probed
this session: `sort -V` orders `1.0.0` **before** `1.0.0-rc1`; semver orders it
**after**. So reproducing `sort -V` would bake a prerelease ordering into a gate
whose entire subject is a semver line (docs/install.md §Versioning: *"one semver
line, applied as git tags"*), and implementing semver instead would not be a port
at all — it would be a rule change smuggled through a substrate change. Refusing
is the only disposition that neither invents a rule nor ships a known-wrong one.
It is §The POSIX ERE matcher's refusal shape and the JSON cohort's pins-grammar
shape, applied to an ordering rather than to a parse.

**What the refusal does not foreclose, stated because it looks like a closure.**
`scripts/pack-installer.sh` admits a `-`/`+` suffix in its version regex, and
docs/install.md §Versioning names that as *nothing forecloses the change*. This
delta leaves the prerelease path exactly as open as it was: it converts the first
note that uses one from a **silent mis-order** into a **loud red at the gate**,
and the ordering ruling belongs to the session that actually cuts one, which will
have the case in front of it. That obligation is filed as a gap rather than left
implicit, and docs/install.md §Versioning gains the sentence naming where the
ruling is owed.

**Measured against this tree:** all 23 live `release:` keys are bare `vX.Y.Z` and
the disposition file carries no data line, so the refusal is a guard for the
future rather than a change to this repo today — the standing the ERE refusals and
the pins narrowing both took.

The `sort -u` inside the disposition collector is **not** version ordering — it is
a byte sort with dedupe over whole lines, and it ports as one. Named because
`sort -V` and `sort -u` sitting four lines apart in the same function is exactly
how a port fuses them.

### 4. The three rules port, with every subprocess read through the crate's one spawn site

The rules move to `native/src/gates/release_bump.rs`,
`native/src/gates/tightened_gates_grammar.rs` and
`native/src/gates/tightened_gates_note_parity.rs`. **[design-bearing]**

Two of them shell out, and both calls go through `proc::run` — a gate module
cannot construct a `Command` at all, and a unit test holds that
(§Fail-closed contract):

- **`git rev-parse -q --verify refs/tags/<tag>`**, the *under composition*
  predicate in both `check-release-bump` and `check-tightened-gates-note-parity`.
  Its non-zero status is the *no such tag* branch, which is a **verdict, not a
  failure** — so it is read through `code()` rather than `stdout()`, the accessor
  §Fail-closed contract added for exactly the child that grades its own outcome by
  its exit code.
- **`git log --reverse --format='%H' -p -U0 -- <disposition-file>`**, whose added
  lines the shell extracts with a fixed `sed -E` pattern. The pattern is the
  **gate's own**, so it is hand-compiled rather than routed through `ere.rs` —
  §The POSIX ERE matcher's boundary: a pattern the kit owns is hand-compiled, a
  pattern a consumer supplies goes through the engine.

**One conflation is preserved deliberately rather than repaired, because a port
proves parity and does not fix rules.** The shell silences that `git log`'s stderr
and returns success regardless, so a tree where git cannot answer yields *no
historical dispositions* and the deferral floor is derived from the live file
alone. The compiled form reproduces it, and the reasoning is recorded here rather
than left as an accident of translation: the branch is unreachable in the live
tree (which always has a repository) and in the fixture pair (which supplies the
file), so there is no case that would prove a change correct. A session wanting
the distinction files an entry against the merged spec.

**The three declare no knobs, and the `.workflow/` asymmetry that exposes is
preserved rather than repaired — because repairing it needs an answer this
tranche does not have.** Both `.workflow/` defaults are **hardcoded literals** in
the gates (`.workflow/release-disposition.txt`, `.workflow/tightened-gates.txt`),
while `bin/upgrade-smoke.sh`, a reader of the *same* declaration file, resolves
the directory through `GATE_SDK_WORKFLOW_DIR`. Honouring the knob in the compiled
form would make it a **declared** knob, which sends `gate_command` to
`_gate_knob_value`, which sources *the owning kit's* library — and a
consumer-declared member has no owning kit. That is the config-bridge question
`consumer-gate-port-disposition` will have to answer for the first
knob-declaring member of its tranche; it is **named here and deliberately not
answered**, because answering it inside a port whose members need no knob would be
designing against no case. Keeping the literals keeps the port a port, keeps
`gate_command` on its zero-knob path (no `env` prefix, `_gate_knob_value` never
reached), and leaves the asymmetry exactly as visible as it is today. It is filed
to the gap inbox so the debt is costed rather than implicit.

Everything else is direct code against delta 2's module: the front-matter
`release:` extraction, the fixed-section presence assertions, the patch-only
predicate over the major/minor fields, the deferral discharge and floor, and the
two-direction set comparison the note-parity gate does with `comm`.

### 5. `--declaration-parity`, the standing oracle criterion 6's dual disposition demands

The binary gains a **top-level flag arm** — never a subcommand — reporting
`declaration.rs`'s classification of one input, one record per line:
`--declaration-parity section <file> <section>` and
`--declaration-parity record <file>`. Its consumer is a new
`gate-sdk/gate-tests/declaration-lib-parity.test.sh`, which feeds **one canned
corpus** to both holders and compares their classification **byte for byte**.
**[design-bearing]**

**A flag rather than a subcommand, and the reason is structural**:
§check-gate-substrate-parity assertion B reds a subcommand no descriptor
dispatches to, and nothing dispatches here — so an introspection arm joins
`--list`, `--reads`, `--knobs` and `--source-stamp` at the binary level, the
four siblings that paragraph names. That is the assertion's own recorded
consequence, stated generically rather than by flag spelling — the paragraph
describes the shape and cites queue-kit/SPEC.md §lib/queue.sh as "the first
such arm's consumer" without naming `--queue-parity` itself, which is
gate-sdk/SPEC.md never carrying one project's flag vocabulary past the
generic pattern. `--queue-parity` (`native/src/main.rs`, real and dispatching
today) is the actual precedent this arm is built on line for line, even
though the roster prose it joins names it by shape, not by spelling.

**Why it is owed at all, which is the half a build session skips.** Criterion 6
admits a duplication only where it is *machine-held*, and criterion 2's port-time
byte-identity proof is not that — it proves the two agreed once and expires at the
next edit to either side, which is precisely the failure the clause names. The
shell caller set does not empty here (`bin/upgrade-smoke.sh` survives the port, as
scope ruled), so neither the duplication-absent road nor the deleted-original road
is available. What remains is queue-kit's `lib/queue.sh` disposition, taken here
for the same reason and by the same mechanism (queue-kit/SPEC.md §lib/queue.sh).

**The corpus is the trichotomy, not a sample of this repo's notes.** It carries
every arm both holders can reach: a section that is absent, one whose body is an
explicit `None`, one resolving to tokens, one that is neither — including the
empty-bullet case where the shell's status 1 comes with *empty* stdout — and the
record arm's clean and malformed inputs. Comparing classification rather than
internal representation is queue-kit's ground and it holds unchanged here.

`gate-tests/lib-declaration.test.sh` keeps its place **unchanged**. It is the
shell arm's own runtime lock-in and it answers a different question; the two are
siblings, and folding the parity comparison into it would leave the shell arm
without a test that fails when only the shell is wrong.

### 6. Criterion 5 is priced by measurement, with the acceptability judgment fixed in advance

The cohort records the **binary-less residual** — the roster a consumer whose
payload carries no artifact for its host loses, and its count — measured with
`installer_smoke`'s binary-less leg, after this cohort's own commit, from a clean
checkout reached **by path**. **[design-bearing]**

The prediction, and it is structural rather than empirical this time: all three
members are **consumer-declared**, so they sit in no kit's `checks/`, `init` can
never seed them, and no adopter has ever had them — a residual counts what a
consumer *loses*, and this cohort takes nothing from any consumer. Growth of zero
is therefore predicted on stronger grounds than the ninth cohort's `on-surface`
argument. It stays a **prediction the measurement rules on, never a discharge**;
criterion 5 is explicit that N members each individually runnable is not one. The
judgment, fixed now so a build session facing a number has a decision procedure:
**zero → the cohort lands on that finding**, with the consumer-declared ground
restated rather than banked; **non-zero → the cohort lands only with one of
criterion 5's three designed answers named in its spec section**, and a non-zero
here is a finding about the *measurement* as much as about the cohort, since the
structural argument predicts it cannot happen.

### 7. The narrowing's readers, enumerated by red condition

Three declaration paths change from `<gates-dir>/<name>.sh` to
`<gates-dir>/<name>.gate`, and three shell files leave the tree. Canon-kit's
causal-completeness point 5 binds, so every reader is enumerated by **what makes
it red**, never by what it is about — the reds-on-empty and inverse-verdict shapes
are the ones a "a narrower corpus can only remove violations" reading misses.
**[design-bearing]**

- **`check-gate-substrate-parity` assertion A** — reds on a descriptor and a
  script **coexisting** in one resolve dir. The narrowing *satisfies* it and a
  half-done one violates it, which is the inverse of the reflex; it is also why
  parity must be proved **while both implementations still exist**.
- **`check-gate-substrate-parity` assertion B** — reds when the descriptor set and
  the `--list` roster disagree. All three subcommands declare the consumer
  sentinel as owner and are in scope in this tree
  (`SPEC-consumer-first-mover.md` deltas 1–2), so all three must land their
  descriptor with their registry entry.
- **`check-gate-substrate-parity` assertion C** — reds on a derived
  substrate-sensitive member with no disposition row. **Measured this session by
  replaying assertion C's own loop against both the current and the post-port
  declaration paths: zero transitions in either direction.** Roughly nineteen
  members reach these three today through a `scripts/*.sh` glob, and every one of
  them also matches other surviving `.sh` declaration paths, so none loses
  sensitivity; four members carry a bare `*.gate` token that already matches
  dozens of existing descriptors, so none gains it. `check-gate-binary-fresh`
  couples `kit:checks/*.gate` specifically and does **not** reach a
  gates-directory descriptor. The finding is a **measurement, not a discharge** —
  the set is derived at runtime and is re-derived at the cut, because a manifest
  edit between now and then moves it. Only the newly-selected direction can fail:
  a row the derivation stops selecting is not red.
- **`check-gate-substrate-parity` assertion E** — reds on an implementation
  sibling under a **kit root**; the consumer's gates directory is outside its
  reach by design (`SPEC-consumer-first-mover.md` delta 5), so this narrowing adds
  nothing here and the crate-root half is untouched.
- **`check-shellcheck`** — reds on a ShellCheck finding and, separately, **exits 2
  on an empty target set**. A reds-on-finding-none condition, not monotone. The
  gates directory retains many `check-*.sh`, so the set stays non-empty —
  re-checked at build rather than inherited, since the ground is a count.
- **`check-gate-fixture-coverage`** — reds on a registered member with no
  `good/`+`bad/` pair. All three pairs are **retained**; deleting the script and
  orphaning its pair are one action apart.
- **`check-gate-output`** — for a fixtured member the assertion runs at fixture
  time against real output, so the port must keep both a `: clean` emission and a
  `help:` line in the compiled form, with the `println!`/`eprintln!` alternation
  rather than the shell's.
- **`check-exec-bit`** — its `.gate` arm is **unconditional and separate from
  `GATE_SDK_EXEC_GLOBS`**: any tracked path ending `.gate` outside a pruned
  segment must be index mode `100644`. So it reaches a gates-directory descriptor
  already, with no widening, and the red condition is the inverse of the usual one
  — a descriptor committed **executable**. The `.sh` globs it does carry include
  the computed `<gates-dir>/check-*.sh`, whose set simply shrinks by three.
- **`check-reads-couples`** — reds when a member's observed walk roots are not a
  subset of its declared ones, answered for a `.gate` member by the binary's
  `--reads`, with **no descriptor-level exemption**. Each of the three walks
  exactly one directory, and that directory is **relocated by the gate's own
  positional argument** — the fixture cases pass `posts`, the live tree defaults
  to `docs/posts` — so the honest declaration is a single **`?`**, not the
  default path. Declaring `docs/posts` would red unit test A the moment a fixture
  case ran, and `?` at arity one is exactly the shape the roster already uses
  throughout (no registry member declares a concrete root today). The two `git`
  calls are subprocess reads: `walk.rs` has no hook for them and neither the
  shell extractor nor `--reads` recognises a subprocess as a walk, so they enter
  no roster in **either** substrate. That is inherited scope, not a gap this port
  opens — stated because the reflex is to file one.
- **`check-comment-tier`, `check-spec-pointer`, `check-todo-task-liveness`,
  `check-deprecation-task`** — each reds over the comment surface the corpus
  follows to `native/src/gates/*.rs` and the three descriptors, whose own lines
  are directives by construction.
- **`check-readme-roster`** — **structurally out of reach, measured.** Its corpus
  is `gate_kit_roots` only, comparing each kit README's roster block against that
  kit's `checks/*.sh` + `checks/*.gate`; the consumer's gates directory is not a
  kit root and the root README is not a kit README, so no roster names these three
  before or after. Neither direction can fire. Recorded as a verdict because
  "extend the roster" is the reflex, and because the *general* fact it exposes —
  that no consumer-declared gate has a roster reader at all — is a standing
  coverage question filed to the gap inbox rather than answered inside this
  cohort's envelope.
- **`check-install-disposition`** — **structurally out of reach, measured.** It
  sweeps `gate_kit_roots` only, globbing `<kit>/checks/check-*.sh` and
  `check-*.gate`; the gates directory is never swept, so it imposes no `# install:`
  requirement on a consumer-declared gate in either substrate. None of the three
  carries the header today, and the descriptors owe none. This is the section's
  own recorded disposition — *substrate-blind by construction; a port moves
  nothing here* — holding as written.
- **`check-graph`, `check-enforcement-fresh`, `check-value-rollup-fresh`,
  `check-footprint-fresh`, `check-docs-mirror-fresh`, the generated pre-commit
  hook** — each reds on staleness against a changed declaration set. Every ported
  member changes the manifests those projections derive from, so all are stale by
  construction and are regenerated with the cohort; each prints its own regen
  command on red.
- **`check-measured-claim`** — reds when a marked literal disagrees with its
  oracle. `docs/install.md` carries `<!-- measured: ported-gate-members=44 -->`
  over a sentence stating the same number, and the emitter counts registry members
  resolving to a `.gate` — so the marker disagrees the moment the descriptors
  land and the literal moves to 47. **The generated pre-commit hook bakes the same
  value** into its config-bridge argv, so the hook regen is owed with the doc
  edit, not after it.
- **`check-settings-paths`** — reds on a committed permission allow entry naming a
  `.sh` path that does not resolve. **Measured: no entry names any of the three.**
  The two grants that could have — `Bash(bash scripts/check-*.sh)` and its
  argument form — carry a `*` in the command token and are skipped by the
  predicate, so this cohort strands nothing and needs no operator settings edit.
  Re-verified at build, because the file is operator-owned and may have moved. One
  non-red consequence, recorded so it is not mistaken for coverage: those wildcard
  grants silently stop covering three gates that no longer run as scripts.
- **`check-docs-cmd`** — reds on a governed doc naming a bare `<dir>/…/<name>.sh`
  path in backticks or a fence that does not resolve to a tracked file. **Measured:
  two live hits, both in `gate-sdk/SPEC.md` §lib/declaration.sh's caller roster**,
  which names `scripts/check-tightened-gates-note-parity.sh` and
  `scripts/check-release-bump.sh` by path. Both red after the deletion, and the
  fix is the caller-roster rewrite this amendment already owes — the row
  §Meta-gate conservation writes for this gate predicts exactly this and calls it
  real signal. Every other mention across the governed set is a bare gate **name**
  in prose and is substrate-neutral. The docs mirror regenerates once the source
  is fixed.
- **`check-crate-arms`** — reds on a failing crate lint or test arm, which is
  where delta 5's parity arm and the widened owner unit test surface at commit
  time.

### 8. What the cohort does not take, and why

**[design-bearing]**

- **`check-installer-no-deps`** — ruled **past this iteration at scope**, never
  rescinded. It remains the named cheapest single-gate first mover for a later
  cohort, with its `jq` reader already paid. A session reading its exclusion as a
  new hold would be re-litigating cohort sizing that is closed.
- **The other ten consumer-declared members** — sequencing, not exclusion, on the
  standing rule that every held member has port work owed rather than a waiver.
  Three of them are held on an unported emitter and one on an undetermined
  criterion 7; those holds and what each owes belong to
  `cohort-held-members-port-prerequisites` and `consumer-gate-port-disposition`,
  not here.

### 9. The terminal queue move is a demotion, not a Done move

On completion `native-gate-port-remaining-corpus` drops its `[spec:]` tag and
returns to the deferred section under `[design-pending]`, with `[roadmap:
now/reliability]` intact. **[mechanical]**

The entry's deliverable is the whole corpus and this amendment delivers one
increment; a Done move would assert a finished port with 57 members unported and
would silently drop the item from the public roadmap projection, which reads
`[roadmap:]` tags off live entries. Stated because neither half has a gate behind
it. The entry has taken this demotion at each of the last seven cohorts.

### 10. What the cohort inherits unchanged, cited rather than restated

The per-member procedure is §Porting a gate to the binary substrate; the payload
rule is §Consumer payload; the criteria are §The port-candidate criteria.
**[mechanical]** Two standing obligations that bite per cohort, named so the build
session does not rediscover them: parity is proved **while both implementations
still exist**, since assertion A forbids a descriptor and a script coexisting in
one resolve dir; and the shell original is **deleted** for a ported member rather
than left running beside it.

## Producers and consumers

**This amendment introduces two new interfaces — `declaration.rs`'s public surface
and the `--declaration-parity` arm — and no new state, event, message or field.**
A port re-implements existing rules on the compiled substrate; the descriptors,
the dispatch seam, the manifest format and the parity harness exist and are
unchanged. The one genuinely new *declaration* this cohort makes, the owner
sentinel, belongs to `SPEC-consumer-first-mover.md` and its producers and
consumers are named there.

- **`native/src/declaration.rs`'s public surface** (delta 2). **Producer:** the
  crate, compiled into the binary; no enabling configuration exists to be missing.
  **Consumers: exactly four, all named** — the three gate modules of delta 4, plus
  the `--declaration-parity` arm of delta 5. No fifth consumer exists at landing,
  and the bounded surface is what keeps a later one deliberate. `bin/upgrade-smoke.sh`
  is deliberately **not** among them: it keeps calling the shell library, which is
  the criterion-6 ruling and the reason delta 5 exists.
- **`SectionVerdict`'s four variants** (delta 2). **Every variant has a named
  reader at a named transition.** `Absent` — read by `check-release-bump` at its
  fixed-section presence assertions (exit 2, *the floor cannot be derived*) and by
  `check-tightened-gates-grammar` at its status-2 arm; `ExplicitNone` — read by
  the grammar gate's `none` counter, which its clean line prints, and by
  note-parity as the resolved empty set it compares; `Tokens` — read by all three,
  as a bullet count in the bump gate and as a token set in the other two;
  `Unparsed` — read by the grammar gate's finding list and by note-parity's exit 2.
  A variant no gate reads would be removed, and none is.
- **The version comparator and its refusal** (delta 3). **Producer:** a `release:`
  front-matter key in a note, or a disposition data line; this repo's producers are
  23 committed notes and a disposition file with no data line. **Consumer:**
  `check-release-bump`'s ordering, and — on the refusal path — the operator reading
  exit 2, which names the token *and its source file*, because a refusal whose text
  does not name where the token came from sends its reader to the wrong file.
- **The `--declaration-parity` arm** (delta 5). **Producer:** the binary's
  top-level dispatch, outside the `--list` roster by construction. **Consumer:**
  `gate-sdk/gate-tests/declaration-lib-parity.test.sh`, run by
  `run-gate-tests.sh` in the per-kit fixture-runner battery — a real, scheduled
  reader, not a harness invoked by hand. **The enabling condition is emitted
  everywhere it must be**: the test resolves the binary through
  `GATE_SDK_NATIVE_BIN`, the knob every other binary reader uses, so an installed
  artifact answers identically to one `cargo` produced.
- **Each ported member's `.gate` descriptor.** **Producer:** the build session,
  writing it as the `.sh` is deleted. **Consumer:** `gate_resolve` at dispatch,
  reaching `gate_command` and the subcommand. All three declare **no knobs**, so
  `gate_command` emits no `env` prefix and `_gate_knob_value` is never reached —
  which is what keeps this tranche clear of the question a knob-declaring
  consumer member would raise, namely which library the config bridge would source
  for a member no kit owns. That question is **named and not answered**: it is not
  this tranche's, and the first consumer-declared member that declares a knob owes
  it.
- **The recorded selection evidence and the measured residual** (deltas 1, 6).
  **Producer:** the build session, into the cohort's spec section. **Consumer:**
  the session cutting the eleventh cohort, which reads the undecidable count to
  know how blind the size claim was, the adjudicated union key to know that three
  reported singletons were one derivation, and the residual to know which value
  class is already thin.

**Every new field has a named reader.** The modules add no message and no record
type beyond `SectionVerdict` above; each gate's finding lines are read by the
operator at exit 1 and by its fixture pair's `expect.txt`, and each clean line's
counts — the bump gate's note count and In-brief presence state, the grammar
gate's note/none/token triple, the note-parity gate's token count — are read by
the `good/` expectations, which is what catches a predicate that vacuously matched
nothing.

## Existing sections updated

- **gate-sdk/SPEC.md §The first cohort, and the rule that selects the next** —
  owned by deltas 1, 3, 6, 8. It gains a cohort subsection, **§The declaration
  cohort**, named for what the cohort buys on §The POSIX ERE matcher's precedent
  rather than for a group key. It records the members, the shared derivation the
  tool split and the session rejoined, the second consecutive size-arm exhaustion
  and the override it selected under, the undecidable count, the version
  grammar's refusal, the parity evidence, and the measured residual with its
  acceptability ruling.
- **gate-sdk/SPEC.md §lib/declaration.sh** — owned by deltas 2 and 5. Its
  four-caller roster is rewritten: two of the three consumer gates now call the
  crate's module, `check-release-bump`'s container-only call moves with them, and
  `bin/upgrade-smoke.sh` remains the shell library's live caller — which is what
  makes the dual disposition true rather than asserted. The section gains delta
  5's standing-oracle paragraph and the pointer to the parity test, and its
  closing seam sentence is extended to say where the seam falls on the compiled
  side.
- **gate-sdk/SPEC.md §Meta-gate conservation for the binary substrate** — owned by
  delta 7. Assertion C's derivation is re-run at the cut and **any member it newly
  selects owes a row**; a row it stops selecting is not removed on that ground
  alone.
- **gate-sdk/SPEC.md §check-gate-substrate-parity** — owned by delta 5, one
  clause: `--declaration-parity` joins the named roster of top-level flag arms
  outside the subcommand equality, for the reason that paragraph already gives its
  four siblings.
- **gate-sdk/SPEC.md §run-gate-tests** — owned by delta 5, only if the new
  `*.test.sh` needs a mention beyond the existing any-`*.test.sh` rule; read the
  section before adding, since a restatement is the defect rather than the fix.
- **docs/install.md §Versioning** — owned by delta 3: one sentence naming that the
  bump gate refuses a version outside the `<major>.<minor>.<patch>` triple, so the
  prerelease path its own text leaves open owes an ordering ruling at the session
  that cuts one.
- **docs/install.md, the `ported-gate-members` marker** — owned by delta 7: the
  marked literal and the sentence stating it move from 44 to 47, and the generated
  pre-commit hook regenerates with them, since it bakes the same value.
- **The three gates' own spec homes** — owned by deltas 2, 3, 4: each gate's
  canonical section gains the sentence that its implementation is a compiled
  subcommand, on the shape §check-action-gh-repo already carries, including what
  the port retired from its requirement set.
- **`TASK-QUEUE.md`'s `release-note-section-set-derivation` entry** — owned by
  delta 7. It cites two of the three by `.sh` path, and **TASK-QUEUE.md is outside
  `CANON_KIT_MANIFEST_FILES`** (probed), so `check-docs-cmd` cannot see it: this is
  a stale-citation edit with **no gate behind it**, which is exactly why it is
  named here rather than left to a red.
- **The docs mirror** — owned by delta 7; it regenerates with the cohort. No
  README roster is owed in either direction, for the reason delta 7 measures.
- **`.workflow/validate-baseline.txt`** — owned by delta 6, if the residual
  measurement leaves an unpaid price; read the file for whether a held `fail` row
  stands rather than assuming one does.

## Definition of Done

- [ ] **Causal completeness** — `declaration.rs` has four named consumers and no
      fifth; every `SectionVerdict` variant has a named reader at a named
      transition; the parity arm's consumer is a scheduled test rather than a
      hand-run harness; every narrowed corpus's reader has its **red condition**
      enumerated, with the reds-on-empty (`check-shellcheck`) and inverse
      (assertion A, the descriptor exec-bit) cases named rather than cleared by
      the "a narrower corpus can only remove violations" reflex.
- [ ] **Selection evidence recorded** — members, the split keys and the
      adjudicated union key, undecidable count, and the largest-takeable-group
      size, from a `--group` run at the cut.
- [ ] **Parity proved while both implementations existed** — per member, over the
      fixture pair and the live tree, before the `.sh` is deleted.
- [ ] **Criterion 6's dual disposition is machine-held, not asserted** —
      `--declaration-parity` landed, the parity test in the fixture-runner
      battery, and a deliberate one-sided edit proved to red it.
- [ ] **The version refusal is loud and sourced** — a token outside the triple
      exits 2 naming the token, its source file and the grammar; a fixture case
      pins that arm; docs/install.md §Versioning carries the sentence and the gap
      is filed.
- [ ] **The preserved conflation is recorded, not repaired** — the silenced
      `git log` branch reproduced, with its unreachability in both live and
      fixture arms written down.
- [ ] **Criterion 5 priced** — residual measured with the binary-less leg after
      this cohort's commit, from a clean checkout by path; acceptability ruled per
      delta 6, and a non-zero treated as a finding about the measurement.
- [ ] **Provenance seam held** — no section name and no gate name in
      `native/src/declaration.rs`.
- [ ] **Merged with no information lost** — §The declaration cohort integrated
      into §The first cohort, and the rule that selects the next; §lib/declaration.sh's
      caller roster rewritten in place rather than appended to.
- [ ] **Amendment deleted** — this file removed on merge; none remain for gate-sdk
      (`ls gate-sdk/SPEC-*.md`), discharged at the iteration rather than at the
      commit while `SPEC-consumer-first-mover.md` is in flight.
- [ ] **Removals propagated** — grepped every spec, doc and README for the three
      `.sh` paths; nothing dangles.
- [ ] **Terminal move is a demotion** — `native-gate-port-remaining-corpus`
      returns to deferred under `[design-pending]` with `[roadmap:]` intact.
- [ ] **Gaps filed** — cross-component gaps found during the work filed through
      the gap inbox.
