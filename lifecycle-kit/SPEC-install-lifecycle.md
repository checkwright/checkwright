# SPEC amendment: install-lifecycle

The port disposition of **lifecycle-kit's one member behind
§bin/install-lifecycle.sh** — `bin/install-lifecycle.sh` (36 lines) — off the
shell substrate as a bridged `Arm::Run` member of the gate binary. The third and
last cut of `declaration-install-and-stage-helper-cuts`, under the port-only run
(TRAJECTORY.md §PRIORITY DIRECTIVE). The iteration's unit set was composed at
scope as five cuts and ruled by the **operator on 2026-09-03 over the lead
relay**; two of the five dropped at this stage on findings recorded in their own
sections — `doctrine-kit/SPEC.md` §install-doctrine (sequenced behind the
installer's behind-invoke relocation) and `lifecycle-kit/SPEC.md`
§bin/session-id.sh (no host entry existed) — leaving three.

**Measured at this HEAD rather than carried from the survey**: the port oracle's
`--tree` arm reads 112 files scanned, 64 declared `no-port`, 0 temporarily held,
**48 owed**, `lifecycle-kit/bin/install-lifecycle.sh owed lines=36` among them.

**The member is not a gate** — §bin/install-lifecycle.sh says so in its own
words, "Advisory tooling, not a gate: no fixture pair is owed" — so no
`gates.list` row, no `.gate` descriptor and no binary-less residual roster moves.

## What changes

### (1) The cut is a singleton behind one stated contract, and it takes the whole section

`lifecycle-kit/bin/install-lifecycle.sh` declares
`lifecycle-kit/SPEC.md §bin/install-lifecycle.sh` in its own line-2 `# spec:`
header, and **no other tracked non-test `.sh` declares that section**
{design-bearing}. Two files carry that section name in a `# spec:` *comment* —
`lib/stages.sh`'s two renderer comments — but the file's own header declares
`§lib/stages.sh`, and that library is `no-port`. The declaring set behind this
section is therefore one file and the cut takes all of it; the 2026-09-03 *outer
bound, never a minimum* ruling has no sequenced member to leave behind.

### (2) `--install-lifecycle` is a bridged `Arm::Run` member, and the `--install` family is refused with cause

The member's contract is an **action with an exit status** — it mutates two files
and writes one git config key, printing narration on stdout {design-bearing}. The
`Arm` split is a return shape: `Arm::Emit` renders a document, `Arm::Run` returns
an exit code. A mutating installer that emits no document is `Arm::Run`, so the
spelling is the operand-style `--install-lifecycle`, the shape `--upgrade-smoke`
and `--lesson-sink` already carry, and `bin/run-gates.sh` gains a front-end arm
beside them rather than reaching it through the `--emit <name>` composer.

**The obvious alternative is an op of the existing `--install <op>` family, and
it is refused on that family's own stated terms.** `installer/README.md` §The
install boundary rules that arm "deliberately **not** a bridged one … **every**
value the arm needs arrives as argv, and the arm reads no kit config and no knob",
because its caller is the bootstrap, which may not be assumed to be a POSIX shell.
This member's whole job is to render blocks **derived from resolved kit config** —
eight knobs across two renderers — so an unbridged op would have to take all eight
on argv from a caller that has no way to resolve them. Recorded as refused rather
than unconsidered, because the name collision makes it the first place a reader
looks.

**Table membership is forced by the forced-family test.** The declared roster is
the union of what the two renderers read, taken from the two gates that already
declare it rather than re-derived: `LIFECYCLE_KIT_AGENT_FILE`,
`LIFECYCLE_KIT_STAGES` and `LIFECYCLE_KIT_QUEUE_FILE` from
`check-lifecycle-registration`, and `LIFECYCLE_KIT_STATE_FILE`,
`LIFECYCLE_KIT_LESSON_EVIDENCE_FILE`, `LIFECYCLE_KIT_SURVEY_RECORD_FILE`,
`LIFECYCLE_KIT_BOUNDARY_TRUNCATE` and `LIFECYCLE_KIT_GAP_INBOX_FILE` from
`check-merge-attrs` — **eight**. A hardcoded top-level flag would resolve platform
defaults and silently ignore every consumer override, which is not a calibration
between two workable shapes but the difference between working and appearing to.

### (3) The `[agent-file]` positional PORTS, and the test that decides it was run rather than assumed

§The non-gate arm requires a porting session to run its distinguishing test
**before** sizing a member, "since it is an interface removal on a governed
surface rather than an implementation detail" {design-bearing}. It was run and its
answer is *ports unchanged*.

The test: an argument is **unportable** when it redirects something
`gate_command` has *already resolved* from the tree's own config before the exec —
it arrives a process too late and would silently change nothing. An argument **the
rule itself consumes**, arriving as argv into the subcommand, ports unchanged.
`install-lifecycle.sh [agent-file]` is the second kind: the path is not a
*selector for where configuration comes from*, it **is the file the rule writes
into**, read by the arm's own argv and overriding the bridged default exactly as
`check-root-tiering` kept both its positionals through its port. Nothing is
deleted and no documented sentence about an argument comes due.

**The near miss is stated because it is one line away and it is an env var rather
than a positional.** `smoke/install.sh:334` runs the installer under
`LIFECYCLE_KIT_CONFIG_FILE=lock-stages.sh` — a genuine config-file selector, and
the unportable shape. It survives untouched, and for a reason worth having in
writing: it is not an *argument* at all. `gate_knob_env` resolves the eight knobs
by sourcing `lib/stages.sh` in a subshell that inherits the caller's environment,
and that library is what reads `LIFECYCLE_KIT_CONFIG_FILE` — so the redirection
happens *inside* the resolution rather than arriving after it. The smoke case
keeps working through the bridge unchanged, and the build proves that rather than
reasoning it.

### (4) Both renderers already have compiled counterparts, so criterion 6 is discharged by collapsing a duplication rather than by creating one

This is the cut's strongest property and the reason it is cheap {design-bearing}.

- **The registration block.** `native/src/stages.rs` already carries
  `pub fn registration_block()`, declared against this very section in its own
  `// spec:` header, because `check-lifecycle-registration` must derive the block
  it certifies. The arm calls it. Nothing new is written and no second producer
  appears.
- **The marker insert/replace.** `native/src/marker.rs` is `lib/inject.sh`'s
  compiled half and says so in its own header — "one reader shared by every block
  consumer, and a writer that replaces a block in place. The shell library keeps
  its own copy for its remaining shell callers; this is the compiled counterpart,
  not its retirement." The arm calls it. The dual existence is **already
  sanctioned by §lib/inject.sh itself**, so this cut adds no duplication there
  either.
- **The merge-attribute block is the one that improves, and it improves in the
  direction criterion 6 prefers.** Today the *writer* is shell
  (`lifecycle_merge_attrs_block`) and the *asserter* is crate
  (`native/src/gates/merge_attrs.rs`, composing `stages::supersede_set()` and
  `stages::union_set()`), so the two derive the same lines through two
  implementations held together by nothing but `smoke/install.sh`. The port lands
  **one** renderer in the crate that both the arm and the gate read, which turns
  a live cross-substrate duplication into an absent one. §lib/stages.sh states the
  writer/asserter split as a design property — "the installer emits this,
  check-merge-attrs verifies it" — and that property survives; what stops being
  true is that the two halves are written twice.

**The enumeration criterion 6 demands was done.** Its clause binds "on a helper
**set**, not on the one helper an amendment happened to name". This member sources
two libraries. `lib/stages.sh` is permanently `# no-port:` as the config bridge's
sole resolver for the `LIFECYCLE_KIT_*` knobs, so its caller set is irrelevant and
the criterion is discharged for it **by construction** — one place computes the
value and the crate holds no default. `gate-sdk/lib/inject.sh` is delta (5).

### (5) `lib/inject.sh` loses a sourcer and the leverage does NOT cash, which is stated rather than implied

`gate-sdk/SPEC.md` §lib/inject.sh names three shell sourcers —
`context-kit/bin/env-probe.sh`, `lifecycle-kit/bin/install-lifecycle.sh` and
`doctrine-kit/bin/install-doctrine.sh` — and this cut removes one {mechanical}.
`kit-library-port-residue` carries the same roster and says `inject.sh` "moves
behind them".

**Two remain, so `inject.sh` does not become takeable and this cut claims no
unblock.** That is a change from the shape scope proposed: the five-cut set would
have taken two of the three sourcers and left one, and the ruling that dropped
`install-doctrine.sh` leaves two. `env-probe.sh` sits behind the Windows leg and
`install-doctrine.sh` behind the installer's behind-invoke relocation, both
recorded in their own sections. The rosters in §lib/inject.sh and on
`kit-library-port-residue` are corrected from three to two; **neither may be
rewritten to read *unblocked*,** which is the error a reader who remembers the
five-cut plan would make.

### (6) Three crate gate modules print an invocation this cut deletes, and no gate reads them

`native/src/gates/lifecycle_registration.rs` prints
`bash lifecycle-kit/bin/install-lifecycle.sh` as remediation text at **two** sites,
and `native/src/gates/merge_attrs.rs` at one {design-bearing}. These are the
strings a session actually follows when either gate reds, so a stale one is not
cosmetic — it sends the reader to a path that no longer exists, at the moment they
are trying to fix something.

**No gate catches this.** `check-docs-cmd` assertion A scans the governed *doc*
set, not Rust string literals; `check-gate-substrate-parity` checks a
disposition's presence and shape, never the truth of a message. They are named
first among the ungated repairs for that reason, and they are the reason this
delta exists separately rather than riding delta (8)'s list.

### (7) The three-step contract ports whole, and two of its behaviors are preserved because a rewrite would not spell them

The section specifies three steps and each keeps its stated behavior
{design-bearing}: the registration-block injection, the merge-attribute
injection, and the driver-config registration. Two carry asymmetries an idiomatic
Rust write would smooth away, so they are explicit instructions rather than
omissions to be inferred:

- **The agent file is never minted; `.gitattributes` always is.** A missing agent
  file is **exit 2** — "the installer edits an always-loaded file, it does not
  mint one" — while `.gitattributes` is legitimately created when absent, "it is
  not an always-loaded file the consumer authored". Two adjacent file writes with
  opposite absent-file dispositions is precisely the shape a port unifies by
  accident.
- **A non-repo cwd degrades to a printed skip, never a hard failure.** The
  driver-config step is in the `install-hooks.sh` per-clone opt-in class, and the
  recorded honest limit — the `.gitattributes` attribute stays inert until a clone
  registers the driver — depends on that step being allowed to fail soft. The skip
  message goes to **stderr** and the two action lines to **stdout**, and that
  split is preserved: the arm is machine-drivable and a finding on stdout is a
  finding in a caller's data stream.

A begin marker without its end stays exit 2 in both files, by `marker.rs`'s own
reader rather than by a second refusal path.

### (8) Every surface the deletion stales moves in the landing commit, probed rather than assumed

{mechanical}

- **`lifecycle-kit/README.md`**, two sites — a numbered install step and a
  command-roster line, both `bash lifecycle-kit/bin/install-lifecycle.sh` in
  invocation position. `check-docs-cmd` **assertion A** reds on these the moment
  the file goes, so they are forced rather than optional.
- **`lifecycle-kit/SPEC.md`**, seven further sites beyond §bin/install-lifecycle.sh
  itself: §Layout and configuration's knob bullet, §Multi-operator semantics,
  the `install-hooks.sh` opt-in-class paragraph, §check-merge-attrs' writer
  attribution, §lib/stages.sh's two renderer paragraphs, and §lib/stages.sh's
  `set -e` caller sentence, which names the installer by path for a different
  reason (the probe-loader's abort-on-source contract, not the writer role) —
  each naming `bin/install-lifecycle.sh`. The *role* survives and the *name*
  changes; each is re-pointed at the arm.
- **`gate-sdk/SPEC.md` §lib/inject.sh**, the three-sourcer roster (delta 5).
- **`lifecycle-kit/lib/stages.sh`**, the two renderer `# spec:` comments naming
  the installer as the emitter — ungated for truth, and edited.
- **`lifecycle-kit/smoke/install.sh`**, five invocation sites plus the `il_run`
  helper — re-pointed at the front-end arm. The smoke is permanently `# no-port:`
  and stays so; it is the member's **only** caller in the tree, which is why no
  validate suite registry and no `EVIDENCE_KIT_RUN_*` value moves.
- **`installer/README.md`** — prose about what the installer writes, not an
  invocation; checked and left, since the sentence stays true of the arm.
- **`docs/posts/2026-07-17-checkwright-v0-5-0.md`** — a release post carrying the
  old invocation. **Historical record: not rewritten.** A post is outside
  `check-docs-cmd`'s governed doc set (its coupling is `*SPEC*.md`, `*README.md`,
  `CLAUDE.md`, `scripts/*.sh`, `kit:*.sh`), so nothing reds — and rewriting a
  dated release note to describe a later tree would be false. **Verified in the
  landing commit rather than asserted here**, because the whole disposition turns
  on that corpus boundary.
- **`.claude/settings.json`** — **no edit owed, probed rather than assumed.** No
  grant names `install-lifecycle.sh`, so the 2026-08-29 settings-grant carve-out is
  exercised **zero times** by this cut, and the ported invocation is already
  covered by the existing `run-gates.sh` grant. That ruling demands the count be
  probed; this is the probe.
- **No `.gate` descriptor is touched** — `check-lifecycle-registration` and
  `check-merge-attrs` couple `CLAUDE.md`/`.gitattributes` and `lib/stages.sh`,
  neither of which this cut deletes, checked rather than assumed.

### (9) Criterion 2's oracle is a constructed scenario, and criterion 5's residual is the sharpest in the iteration

**Criterion 2.** The member has no fixture pair and owes none. Its discharge is the
`# no-fixture:` road — *the same cases, both substrates, while both
implementations exist* {mechanical}. The scenario is the smoke's own, run twice:
a fresh agent file (block appended), a re-run (byte-identical, idempotent), a
staled block (gate reds), a missing agent file (exit 2), a begin marker without
its end (exit 2), a fresh scratch repo (`.gitattributes` minted, driver
registered), a re-run (idempotent), a **non-repo cwd** (skip on stderr, exit 0),
and the `LIFECYCLE_KIT_CONFIG_FILE=lock-stages.sh` case of delta (3) — compared on
exit status, both output streams and the **bytes of both written files**. Bought
once, at port time, with that road's standing limit, which is why the shell
original is **deleted** rather than left beside the arm.

**Criterion 5.** A vendored consumer on a host the artifact roster does not cover
loses the ability to install or refresh its own registration block and merge
attributes — and unlike this iteration's other residuals, this one bites at
**adoption** rather than during use, because the block is what a consumer writes
on day one. It is accepted on the class's stated accept-and-declare terms and on
one narrowing fact: `check-lifecycle-registration` and `check-merge-attrs` are
themselves compiled members, so a host with no artifact does not run the gates
that would demand the block either — it loses the writer and the asserter
together rather than being held to a standard it cannot meet. The loss is real, it
shrinks as targets are published, and it is not repaired by this cut.

### (10) The regeneration fan-out this cut stales

{mechanical} Each is rostered with its trigger and regen command in
`docs/site-architecture.md` §Generated projections and discharged in the landing
commit:

- **The generated `pre-commit` and `commit-msg` hooks** — the baked
  `check-measured-claim` values carry `tree-shell-owed`, which this deletion moves
  (`bash gate-sdk/bin/gen-pre-commit.sh --write`).
- **`docs/check-graph.html`**, asserted fresh with them by `check-graph`.
- **`docs/lifecycle-kit/SPEC.md`, `docs/lifecycle-kit/README.md`,
  `docs/gate-sdk/SPEC.md`**, the generated on-site mirrors
  (`bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write`).
- **The gate binary**, `bash gate-sdk/bin/build-native.sh`, held current by
  `check-gate-binary-fresh`. Stage the deletion first and regenerate second, both
  deriving through `git ls-files`.
- **`docs/footprint.md`** — checked against context-kit/SPEC.md §bin/footprint in
  the landing commit rather than assumed either way. This cut edits no kit
  `templates/` markdown, which is the trigger a `bin/` deletion would otherwise
  not reach.

### (11) The host is `kit-library-port-residue`, beside the iteration's first cut, and the lead line is measured

The host is the same entry cut A promoted {design-bearing}. §Porting a gate to the
binary substrate rules that "an entry already waiting on this cut's subject is the
better host rather than the leftover one", and this entry's `inject.sh` paragraph
names `lifecycle-kit/bin/install-lifecycle.sh` in as many words as one of the
three sourcers `inject.sh` moves behind — so the entry's text changes *because of*
this cut, which is the ground the rule states.

**Two `[spec:]` tags on one lead line, and that is arithmetic run rather than a
convention broken.** The same section's per-entry arithmetic, corrected at this
stage after its flat form misled this session, resolves here as: fixed part 30
columns (`- **kit-library-port-residue**`, carrying no permanent tag), a tag
costing `9 + len(basename)`, `check-queue-wrap`'s floor at 100. With cut A's
`SPEC-declaration-cut.md` at 32 columns, the budget for this one is **29
characters of basename**; `SPEC-install-lifecycle.md` is 25, landing the line on
**96**. The entry's trailing prose reflows onto the continuation lines, which is
what a tags-only lead line costs — the entry renders as a bare slug in the
parsing tools' output.

**What this host claims is an increment, not a blocker discharge.** Cut A
delivered `declaration.sh` out of this entry's member roster; this cut delivers no
member of it — `install-lifecycle.sh` is not one of the four, it is a *blocker* of
one — and it does not discharge that blocker either, because two of the three
remain (delta 5). So the entry's `inject.sh` paragraph is corrected from three
sourcers to two and nothing else about it moves. Recorded in those terms because
the two hostings this entry carries this iteration make different claims, and a
later reader must not average them.

**The demotion arithmetic, done here rather than met as a red — and the answer is
zero headroom, measured after the promotion rather than predicted before it.** The
entry demotes to `## Deferred` under `[design-pending]` at build
(canon-kit/SPEC.md §Merging an amendment) and lands back inside
`check-queue-entry-budget`'s per-entry cap. Measured **at this promotion, on the
tree as it now stands**: **51 raw lines**, three `ruled:` lines against a discount
of at most one line per grammar, counted **50** against
`QUEUE_KIT_ENTRY_LINE_CAP` = 50 — **exactly at the cap, with no headroom at
all**. The reflow this delta's lead line required is what spent the last line:
moving 33 columns of prose off the lead line did not fit in the two body lines it
rejoined and took a third.

**So the build inherits a cap it is already touching, and the two edits it owes
are both subtractive** — `declaration.sh` leaves the member roster (cut A's
delta) and *three shell sourcers* becomes *two* (delta 5) — so the demoting commit
should **gain** lines rather than spend them. That is the arithmetic's whole
margin, and it is not assumed: the build asserts the counted size in the demoting
commit, and where the two subtractions do not cover an edit it wants, it
compresses spent narrative in place in that same commit. Recorded at zero rather
than rounded up to one, because a build that reads *one line of headroom* here and
finds none has been handed a red this stage could have measured.

## Producers and consumers

The amendment introduces **one interface** — the `--install-lifecycle` flag and
its `bin/run-gates.sh` front-end arm — and **no new state, no new event, no new
field and no new knob.** Every knob it names is already shipped, already read, and
already declared by a gate; what changes is which process reads it.

- **Producer of the arm** — `native/src/emit/mod.rs`'s `BRIDGED_ARMS` table, one
  row carrying the flag spelling, `Arm::Run` and the eight-knob roster. The
  enabling config is the table row itself: `--knobs` publishes the roster and
  `gate_knob_env` resolves it by sourcing `lifecycle-kit/lib/stages.sh`. **No
  default moves**, and that is checkable rather than asserted: all eight are
  already defined in that library because two compiled gates already declare
  them, which is what makes the resolution succeed rather than refuse with exit 2.
- **Producer of the front-end reach** — `bin/run-gates.sh`'s new arm. An
  `Arm::Run` member is reached by its own branch, the shape `--upgrade-smoke` and
  `--lesson-sink` already carry, not by the `--emit <name>` composer.
- **Producer of the written blocks** — `crate::stages::registration_block()` and a
  merge-attribute renderer composed in-crate from `stages::supersede_set()` and
  `stages::union_set()`, both writing through `crate::marker`'s in-place block
  writer. The first two already exist; the third is the collapse of delta (4).
- **Consumer of the registration block** — `check-lifecycle-registration`, at
  every pre-commit and every battery run, reading `CLAUDE.md` and comparing
  against the **same** renderer the arm wrote it with. That shared-renderer
  property is stated in §lib/stages.sh today and is what the port preserves; what
  changes is that writer and asserter now share it in one substrate.
- **Consumer of the merge attributes** — `check-merge-attrs`, at the same
  transitions, over `.gitattributes`, and **git itself** at every merge of a
  supersede-set or union-set path, which is the consumer that makes the step worth
  having at all.
- **Consumer of the driver registration** — `git merge` in the clone where it ran,
  resolving `merge.iteration-scoped` to keep-ours. Per-clone and opt-in; unchanged.
- **Consumer of the verdict** — `lifecycle-kit/smoke/install.sh`, at every
  consumer-smoke run and every validate stage, over exit status and the bytes of
  both written files. It is the member's only caller and it is re-pointed, not
  created.
- **Consumer of the moved count** — `check-measured-claim`'s oracle, transitively
  through the `tree-shell-owed` key and the resolved values the generated hooks
  bake, which is why delta (10)'s regeneration is an update target.
- **Consumer that stops existing** — `gate-sdk/lib/inject.sh`'s shell sourcer set
  loses one of three. Its *readers* do not disappear and the library is not
  unblocked: two sourcers remain (delta 5).

**One corpus is narrowed — the tracked non-test `*.sh` tree loses one file — so
each reader's RED CONDITION is enumerated rather than its subject**
(canon-kit/SPEC.md §The causal-completeness check, point 5). "A narrower corpus
can only remove violations" is false and is the first argument a narrowing delta
reaches for.

- **`check-docs-cmd` assertion A** — reds on a repo-relative `.sh` path in
  invocation position inside a fence in a governed doc that resolves to no tracked
  file. **Not monotone**: the deletion *adds* two violations, both in
  `lifecycle-kit/README.md`. Cleared by delta (8), in the same commit.
- **`check-docs-cmd` assertion B** and **`check-kit-ref-liveness` leg (b)** — each
  reds on a kit-prefixed ALL-CAPS token occurring in no tracked kit *code*, the
  second over every tracked file. **Clear, and the reason is structural rather
  than lucky**: all eight declared knobs live in `lib/stages.sh`, which this cut
  does not touch, so the trap that forced §upgrade-smoke to move three defaults
  into `lib/gate.sh` cannot arise. Checked rather than assumed, because it is the
  single most expensive thing to discover at build.
- **`check-knob-default-coupling`** — reds on a literal knob default disagreeing
  across its sites or with its owning SPEC, and on a knob whose SPEC states no
  default. **Clear**: no default moves and no knob's owning section changes its
  stated value.
- **`check-graph`** — reds when a committed generated hook or the graph artifact
  differs byte-for-byte from its generator's output. **Not monotone**: the
  deletion moves the baked `tree-shell-owed` value. Cleared by delta (10).
- **`check-docs-mirror-fresh`** — reds on a `docs/` mirror not byte-equal to its
  kit source. **Not monotone** on the SPEC and README rewrites. Cleared by delta
  (10).
- **`check-gate-binary-fresh`** — reds when the committed binary's source stamp
  disagrees with the crate's. **Not monotone**: the crate changes. Cleared by the
  rebuild in delta (10).
- **`check-spec-pointer`** — reds on a `# spec:` header naming a section that does
  not exist. **Clear**: the deleted file takes its own pointer with it, and
  `lib/stages.sh`'s two comments naming this section keep a section that survives,
  restated.
- **`check-settings-paths`** — reds on a literal repo-relative `.sh` grant that
  does not resolve. **Not monotone in general — a port is the event this gate
  exists for — but clear here**, probed in delta (8). The trap rides with it: the
  generated hook matches staged `ACMR` paths, so a *deleted* `.sh` never fires the
  trigger and only a whole-tree battery run would catch a stranded grant.
- **`check-measured-claim`** — reds when a `measured:` marker disagrees with its
  oracle or names a key the oracle does not emit. `tree-shell-owed` moves; whether
  a tracked marker binds it is **checked by scanning the markers** in the landing
  commit rather than assumed.
- **`check-smoke-entry-guard`** — reds on a `smoke/` script reachable by a bare
  invocation. **Monotone**: the smoke's guard is untouched and only its invocation
  targets change. Cleared by inspection.
- **`check-shellcheck`, `check-exec-bit`, `check-comment-tier`,
  `check-path-dialect`, `check-tree-terms`, `check-gate-substrate-parity`,
  `check-gate-exemption-tasks`, `check-evidence-baseline`, `check-test-hermetic`**
  — each verdict is **monotone** in the narrowing: removing one `.sh` can only
  remove findings, the shellcheck corpus is nowhere near empty, no `.gate` roster
  moves, the file declares no disposition field, and no validate suite's name
  changes. Cleared by inspection.
- **The reader nothing catches, and it is delta (6)** — the three remediation
  strings in two crate gate modules. No gate reads a Rust string literal for
  truth, and these are the strings a session follows when the gate reds.
- **On the addition side**, `check-comment-tier` and `check-path-dialect` reach
  the new crate code from the commit it lands in. `check-path-dialect` matters
  more here than the member's size suggests: the arm writes two files at
  consumer-relative paths and mints one, which is exactly the boundary that gate
  is tightened over.

**Cross-component signal: this amendment's component set is two** — lifecycle-kit
and gate-sdk (§lib/inject.sh's roster and the front-end arm). Together with its
two sibling amendments this iteration, `check-stage-entry` assertion C fires on
the amendment-files-span-two-components arm and the **align stamp is demanded at
the build stage's entry**. Stated here so the build session is not the one that
learns it.

## Existing sections updated

- `lifecycle-kit/SPEC.md §bin/install-lifecycle.sh` — restated for the arm: the
  invocation form and its front-end reach, the eight-knob declared roster, the
  `[agent-file]` positional and the test that kept it, the three steps with both
  preserved asymmetries (the agent file never minted, `.gitattributes` always,
  the non-repo skip on stderr), and the shared-renderer property now holding in
  one substrate. The `### bin/install-lifecycle.sh` **heading is unchanged**, six
  in-SPEC citations resolving against it (deltas 1, 2, 3, 4, 7 and 9).
- `lifecycle-kit/SPEC.md §lib/stages.sh` — the two renderer paragraphs whose
  writer is this installer, and the `set -e` caller sentence that names it by
  path; the merge-attribute renderer's writer/asserter split survives while its
  two implementations collapse to one (deltas 4 and 8).
- `lifecycle-kit/SPEC.md §Multi-operator semantics`, `§check-merge-attrs`,
  `§Layout and configuration` and the `install-hooks.sh` opt-in-class paragraph —
  each names `bin/install-lifecycle.sh` as the writer; the role survives and the
  name changes (delta 8).
- `lifecycle-kit/README.md` — the numbered install step and the command-roster
  line, forced by `check-docs-cmd` assertion A rather than optional (delta 8).
- `lifecycle-kit/lib/stages.sh` — the two renderer `# spec:` comments naming the
  installer as the emitter; ungated for truth, so an explicit target (delta 8).
- `lifecycle-kit/smoke/install.sh` — five invocation sites and the `il_run`
  helper re-pointed at the arm; the harness stays `# no-port:` and its
  `LIFECYCLE_KIT_CONFIG_FILE` case is proved through the bridge (deltas 3, 8 and 9).
- `gate-sdk/SPEC.md §lib/inject.sh` — the sourcer roster goes from three to two,
  and the library stays owed and unblocked-by-nothing (delta 5).
- `gate-sdk/SPEC.md §The non-gate arm` — the class roster gains
  `--install-lifecycle`; the `--install <op>` refusal is recorded as a worked
  instance of a member declining that family, and the argv distinguishing
  test gains a *ports unchanged* answer beside its two deletions (deltas 2 and 3).
- `gate-sdk/bin/run-gates.sh` and its `--help` text — the new front-end arm and
  its usage line, the class's stated home for a bridged arm's usage (deltas 2
  and 8).
- `native/src/gates/lifecycle_registration.rs` and
  `native/src/gates/merge_attrs.rs` — three remediation strings naming the deleted
  path. Ungated, and the first thing a reader follows on a red (delta 6).
- `docs/site-architecture.md` — no ruling changes; named because delta (10)'s
  fan-out is read off it (delta 10).
- `TASK-QUEUE.md`, the `kit-library-port-residue` entry — gains this amendment's
  `[spec:]` ref **beside** cut A's, at 96 columns with its trailing prose
  reflowed; at build its `inject.sh` paragraph goes from three sourcers to two.
  It **demotes** at build rather than reaching `## Done`, at zero headroom
  (deltas 5 and 11).

<!-- update-target-exempt: the composer entry takes no body write from a cut by its own 2026-08-28 ruling — each closed cut's record lives in the contract section that cut selected, which is delta 1's section; its lead line already carries the sibling cut's ref -->
- `TASK-QUEUE.md`, `native-gate-port-remaining-corpus`'s body — deliberately
  unwritten.

<!-- update-target-exempt: two members dropped from this iteration's unit set by operator ruling, each recorded in its own governing section at this stage; neither is this cut's subject and neither takes a write from it -->
- `doctrine-kit/SPEC.md §install-doctrine` and `lifecycle-kit/SPEC.md
  §bin/session-id.sh` — already written, deliberately untouched here.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named reader
      at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls lifecycle-kit/SPEC-*.md`), the none-remain half discharged at
      the **iteration** rather than at the commit, this iteration carrying sibling
      amendments.
- [ ] **Removals propagated** — grepped every spec, README, library header, smoke,
      settings file, crate source and doc for the deleted path; nothing dangles,
      the three **ungated** crate remediation strings of delta (6) included.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **The oracle moved by the roster, not by the trailer** — the `--tree` arm
      lists no `lifecycle-kit/bin/install-lifecycle.sh` row, taken as a per-file
      roster diff.
- [ ] **Parity was executed with both substrates alive** — the nine-case scenario
      of delta (9), compared on exit status, both output streams **and the bytes of
      both written files**, *before* the shell file was deleted.
- [ ] **The two asymmetries still hold** — a missing agent file is exit 2 and
      `.gitattributes` is still minted when absent; a non-repo cwd still exits 0
      with its skip **on stderr**.
- [ ] **The config-file case still redirects** — the smoke's
      `LIFECYCLE_KIT_CONFIG_FILE=lock-stages.sh` run resolves through the bridge
      and the installed block reflects the locked stage set, proved by running it.
- [ ] **`inject.sh` is corrected, not promoted** — §lib/inject.sh and
      `kit-library-port-residue` both read *two* sourcers, and neither reads
      *unblocked*.
- [ ] **The release post is untouched and that was verified** —
      `docs/posts/2026-07-17-checkwright-v0-5-0.md` still carries the historical
      invocation, and `check-docs-cmd` is green over it because a post is outside
      its governed doc set.
- [ ] **The regeneration fan-out is discharged in the landing commit** — the two
      generated hooks, the graph artifact, the three `docs/` mirrors and the gate
      binary, staged before regenerated; `docs/footprint.md`'s trigger checked
      against its owning section.
- [ ] **The host entry demotes inside its cap** — `kit-library-port-residue`
      returns to `## Deferred` under `[design-pending]` carrying **neither**
      `[spec:]` ref, with `declaration.sh` delivered out of its member roster and
      its `inject.sh` roster at two, and its counted size at or under
      `QUEUE_KIT_ENTRY_LINE_CAP` **in the demoting commit**.
