# SPEC amendment: upgrade-smoke-cut

The port disposition of **gate-sdk's one member behind §upgrade-smoke** —
`bin/upgrade-smoke.sh` (239 lines) — off the shell substrate as a bridged
`Arm::Run` member of the gate binary. This is the second of the iteration's two
port cuts under the port-only run (TRAJECTORY.md §PRIORITY DIRECTIVE), composed
at scope and ruled option (a) by the **lead on its own authority**, 2026-09-02,
over the resume channel; it did not reach the operator, and that is stated
because a composition ruling recorded without its authority reads at the
post-port triage as more settled than it is.

**Measured at this HEAD rather than carried from the survey**: the port oracle's
`--tree` arm reads 115 files scanned, 64 declared `no-port`, 0 temporarily held,
**51 owed**. This cut takes one of that column and **unblocks a second**, which
is why it was selected: it is the only cut in the corpus that ends another owed
member's stated blocker.

**The member is not a gate** — §upgrade-smoke says so in its own words, "A
`bin/` tool, not a gate — no `good/`+`bad/` fixture pair is owed" — so no gate
roster and no binary-less residual roster moves, which is the whole of what
criterion 5's per-cohort measurement asks of a cut with no gate in it.

## What changes

### (1) The cut is a singleton behind one stated contract, and its leverage is the reason it was taken

The composer ruling of 2026-08-28 selects a cut **by stated contract** — the
owed files behind one specification section, ported behind the one amendment
that section needs {design-bearing}. `gate-sdk/bin/upgrade-smoke.sh` declares
`gate-sdk/SPEC.md §upgrade-smoke` in its own header, and **no other tracked
non-test `.sh` declares that section**, probed rather than inferred. A singleton
is a well-formed cut under that ruling exactly as a group of five is: the
ruling's unit is the section, not a member count.

**Its leverage over every other candidate was measured, not argued.** Of the 51
owed files, this is the only one whose deletion ends another owed member's
**stated** blocker — delta (4) is that discharge — and the unblock was probed
before it was claimed rather than taken from the grouping census.

### (2) `--upgrade-smoke` is a bridged `Arm::Run` member, and both halves of that are forced

The member's contract is its **exit status**: 2 for a broken tag or a broken
environment, 1 for an upgrade finding, 0 clean with one `UPGRADE-SMOKE: clean
(…)` line on stdout and every diagnostic on stderr {design-bearing}. gate-sdk's
own §The non-gate arm rules that "a member whose stated contract is its child's
exit status cannot take the `--emit-` spelling at all", because `Arm::Emit`
collapses every error to 2 and every success to 0 and the 1-versus-2 split is
this suite's whole verdict grammar. So the spelling is the operand-style
`--upgrade-smoke`, the shape `--lesson-sink` and the harness-integration arms
already carry, and `bin/run-gates.sh` gains a front-end arm beside `--usage-poll`
rather than reaching it through the `--emit <name>` composer.

**Table membership is forced by the forced-family test.** The member resolves
five knobs, so a hardcoded top-level flag "resolves platform defaults and
silently ignores every consumer override — which is not a calibration between
two workable shapes but the difference between working and appearing to". Its
declared roster is `GATE_SDK_UPGRADE_REPO`, `GATE_SDK_UPGRADE_FROM`,
`GATE_SDK_UPGRADE_TO`, `GATE_SDK_TMP_DIR` and `GATE_SDK_WORKFLOW_DIR` — each
read exactly once at the resolve step, as §upgrade-smoke already specifies.

**Its named caller needs no invention**: the `upgrade` validate suite, at every
validate stage, re-pointed from `bash gate-sdk/bin/upgrade-smoke.sh` to the
front-end arm. Its second caller is a **session** running the standing
pre-release assertion at the default `TO=HEAD`, which §The non-gate arm counts
exactly as a stage step.

**The argv question binds zero times, probed rather than assumed.** The script
takes **no positionals and no flags** — its only `$1`/`$2` are function-local —
so gate-sdk's distinguishing test between an input-corpus positional and one
redirecting resolved config has nothing to decide, nothing is deleted, and no
documented sentence about an argument comes due.

### (3) The consumer-smoke library survives, and the arm drives it by `bash`, which is criterion 6's duplication-*absent* road

`gate-sdk/lib/consumer-smoke.sh` is permanently `# no-port:` — its registration
accounting probes each unregistered gate through `gate_command`, "and a
crate-side arm doing the same would be the second producer criterion 6 refuses"
— and it keeps three other sourcers after this cut: `bin/run-consumer-smoke.sh`,
`demo/run-demo.sh` and `context-kit/smoke/agents-md.sh`. So its shell caller set
does **not** empty, and criterion 6's delete-the-original road is unavailable
{design-bearing}.

**The road taken is not the *unless* clause's machine-held-twin form but its
strongest form: the duplication is absent, because the arm creates none.** The
three helpers this member uses — `csmoke_gate_descriptors`,
`csmoke_vendor_and_install` and `csmoke_place_binary` — are **not
reimplemented**. The arm spawns `bash` and calls them in the library that owns
them. `bash` is on `GATE_SDK_PROGRAM_FLOOR`, so criterion 7 clears by the rule
gate-sdk already states: "a rule shelling out to `bash <emitter>` clears this
criterion, because `bash` is on the floor, however unported that emitter is",
which is the `check-graph`→`gen-pre-commit.sh` shape exactly.

**The seam problem and its in-file precedent.** `csmoke_vendor_and_install`
communicates by *setting the caller's shell variable* `SCRATCH`, which no
process boundary carries. The idiom that solves it is already in this very file
at line 152, where the tool reads the scratch consumer's own
`GATE_SDK_GRAPH_ARTIFACT` with
`bash -c 'source gate-sdk/lib/gate.sh; printf "%s" "$GATE_SDK_GRAPH_ARTIFACT"'`
— an unchanged library plus a one-line stdout protocol. The arm uses that shape:
each helper is invoked in a `bash -c` that sources the library, calls the
function, and prints the one value the caller needs. **The library's source is
not touched**, so there is no second producer and no second definition, and the
protocol is the function's own contract read out rather than a private one
minted beside it.

**The enumeration criterion 6 demands was done.** Its clause binds "on a helper
**set**, not on the one helper an amendment happened to name", and instructs a
port to enumerate the shell callers of **every** helper the ported member
touches. This member touches two libraries beyond `lib/gate.sh`:
`lib/consumer-smoke.sh`, whose caller set does not empty and which takes the
spawn road above; and `lib/declaration.sh`, whose caller set **does** empty —
delta (4).

### (4) `lib/declaration.sh`'s shell caller set empties, and four prose claims that say otherwise are repaired

The declaration resolve step moves **in-crate**: the arm reads the tightened-gates
declaration through `native/src/declaration.rs`, the crate holder that already
exists and that the standing `--declaration-parity` lane already holds equal to
the shell form {design-bearing}. It does not source `lib/declaration.sh` at all.

**That empties the library's non-test shell caller set, and the probe is the
claim rather than the argument.** A grep for sourcers of `gate-sdk/lib/declaration.sh`
over every tracked `.sh` returns exactly three files: `bin/upgrade-smoke.sh`,
`gate-tests/declaration-lib-parity.test.sh` and `gate-tests/lib-declaration.test.sh`.
The `*.test.sh` suffix is outside the `--tree` corpus by that arm's own rule, so
after this cut the set is empty of non-test callers.

**What that does and does not discharge.** §lib/declaration.sh rules the library
"owed to the port, not dispositioned by §The kit-library port disposition … its
own disposition is criterion 6's *unless* clause … and it is **temporary rather
than permanent** — the stated test is whether the shell caller set empties, and
it has not." After this cut it has. So the library becomes **takeable** by a
later cut under its own section — it does **not** port here, because it declares
a different stated contract and folding it in would fail the composer's own
words.

Four claims elsewhere become false and are repaired in this amendment rather
than left to a reader:

- `gate-sdk/SPEC.md §lib/declaration.sh` — "`bin/upgrade-smoke.sh` … is the
  shell form's **only** caller … The shell caller set therefore does not empty,
  so criterion 6's delete-the-shell-form outcome is unavailable."
- `gate-sdk/SPEC.md` — "`bin/upgrade-smoke.sh` **survives the port** as
  `lib/declaration.sh`'s only remaining caller, so neither the
  duplication-absent road nor the deleted-original road is available."
- `gate-sdk/SPEC.md` — "Its consumers are four and are named …
  `bin/upgrade-smoke.sh` is deliberately not among them, which is the criterion-6
  ruling and the reason the oracle exists."
- `gate-sdk/lib/consumer-smoke.sh`'s own `# no-port:` cause, which names
  `upgrade-smoke.sh` among the three sourcers that "keep sourcing it". **No gate
  catches this one**: `check-gate-substrate-parity` assertion G checks a
  disposition's presence and shape, never the truth of its prose. It is named
  first among the ungated repairs in delta (10) for that reason.

### (5) The three `GATE_SDK_UPGRADE_*` defaults move into `lib/gate.sh`, and two cannot move verbatim

`GATE_SDK_UPGRADE_REPO`, `GATE_SDK_UPGRADE_FROM` and `GATE_SDK_UPGRADE_TO` are
defaulted **inside the deleted script** and nowhere else — `_REPO` as the
enclosing repo's toplevel, `_FROM` as the newest `v*` tag, `_TO` as the literal
`HEAD`. All three move into `gate-sdk/lib/gate.sh` in the commit that deletes
the driver {design-bearing}.

**The rule is load-bearing rather than tidy**, and gate-sdk states the failure
mode: the bridge resolves a declared knob by sourcing exactly one kit's library
and **exits 2** on a knob that library does not define, so a default left beside
the compiled reader is sourced by nothing. Because delta (2) declares all three,
leaving any behind is not a silent degradation but a refusal of the whole arm on
every invocation.

- **`GATE_SDK_UPGRADE_TO` moves verbatim** as a guarded assignment defaulting to
  `HEAD`, so the documented default and the supplying site become the same
  string for the first time. `check-knob-default-coupling` reads that literal
  against §upgrade-smoke's stated default and is the oracle for it — and this is
  the one of the three that gate is able to see at all.
- **`GATE_SDK_UPGRADE_REPO` and `GATE_SDK_UPGRADE_FROM` cannot move verbatim**,
  each being a *derivation* over a git repository rather than a literal. Both
  default to **empty**, and empty means *derive it* rather than *no value*: the
  derivation belongs to the one reader of the knob, delta (2)'s arm, which
  resolves an empty `_REPO` to the enclosing repo's toplevel and an empty
  `_FROM` to the newest `v*` tag, keeping every existing exit-2 refusal
  unchanged. This is the shape `CONTEXT_KIT_PUB_LANGS` and
  `CONTEXT_KIT_MEMORY_DIRS` already carry in this tree and for the same reason —
  a repo-relative literal cannot express the value, and the reader is the only
  place that can. Transcribing a derivation into the library would be the second
  producer criterion 6 refuses.

**Two further oracles force the move and are named because neither is obvious.**
`check-docs-cmd` assertion B reds on a kit-prefixed ALL-CAPS token occurring in
**no tracked kit code**, and `native/` is not a kit root — so a Rust `KNOBS`
array does not satisfy it and the three knobs would go red the moment the shell
file that spells them is deleted. `check-kit-ref-liveness` reds on the same
resolver over a **wider** corpus, every tracked file, and `TASK-QUEUE.md`
already carries `GATE_SDK_UPGRADE_REPO` in an entry's prose. Landing the three
in `lib/gate.sh` clears both.

The two redundant re-defaults the script holds for `GATE_SDK_TMP_DIR` and
`GATE_SDK_WORKFLOW_DIR` agree with the library's and die harmlessly with the
file.

### (6) The worktree refusal is preserved verbatim, and criterion 2's oracle must reproduce it

`upgrade-smoke-refuses-inside-a-worktree` is a live `[design-pending]` entry
whose subject is **line 15 of the file this cut ports**: the repo predicate
`[[ -d "$REPO/.git" ]]`, which refuses in every linked git worktree, where
`.git` is a gitdir *pointer file* rather than a directory {design-bearing}.

**The port carries that predicate across unchanged.** A Rust reimplementation
would not naturally spell `is_dir()` — the idiomatic write is exactly the
honest predicate the entry names as one of its two candidates — so this is an
explicit instruction, not an omission to be inferred. The ground is the
composer's: settling a live `[design-pending]` fork inside a port cut is
non-port design work, and the doctrine is gate-sdk's own — a port proves parity
and does not fix the rules it ports. The entry's premise stands, its verdict is
untouched, and its fixture obligation still follows whichever answer eventually
wins. This is the same disposition `md-section-near-miss-match` took at the
previous port cut, and it is the second member of that class.

**Criterion 2's parity run must therefore *reproduce the refusal*, not route
around it.** Running the comparison from a linked worktree is the case that
proves the predicate ported; running it only from the main checkout proves the
happy path and leaves the ported predicate untested in the one place it is known
to matter. The entry itself records that this tree mandates worktree isolation
for a read-only dispatch, so the case is not hypothetical.

### (7) The battery-summary parse stays, and it stays for a structural reason rather than by omission

The suite reads the scratch consumer's battery through two literal patterns —
`All [0-9]+ gates passed` for a green run and `^[0-9]+ of [0-9]+ gates FAILED:`
for the red set — and those literals already have two holders,
`bin/run-gates.sh` and `native/src/runner.rs` {design-bearing}. The port makes
this a **third reader**, and the obvious improvement — call the runner
in-process instead of grepping stdout — is **unavailable by construction**: the
battery under test is the *scratch consumer's*, vendored at a different ref and
dispatched to that ref's own binary, which is the whole point of the
one-binary-per-ref pairing. The host binary cannot run it in process without
destroying the property the suite exists to assert.

So the parse survives, and it survives as a *recorded* cross-ref coupling rather
than as an accident: this member reads a summary line whose producer is a
different implementation at a different ref, and the two literals are the
contract between them. Stated because a later reader will reach for the
in-process call, and because the reason it is refused is not cost.

The membership loop's pipe-free spelling (`bin/upgrade-smoke.sh:226-230`) has a
`# spec:` comment explaining that an abandoned producer's SIGPIPE under
`pipefail` would flip the verdict. **The comment retires with the shell; the
rule does not** — the crate expresses set membership directly and the hazard
does not exist there, which is criterion 7's incidental-spelling class.

### (8) `tar` joins `cargo` on §upgrade-smoke's own suite-requirement sentence

The member spawns `git`, `bash`, `cargo`, `tar`, and a handful of floor
utilities. **`cargo` and `tar` are both off `GATE_SDK_PROGRAM_FLOOR`**
{design-bearing}. §upgrade-smoke already rules `cargo` out of criterion 7's
reach — "a requirement on this suite, not on an adopter", binding only for a ref
that dispatches to the binary, and reaching no consumer. `tar` is **unaddressed
anywhere today**, and it rides the same sentence for the same reason: it is used
to extract a `git archive` of a ref inside a scratch tree, it is not a gate
rule's invocation, and it reaches no adopter.

**The alternative was considered and declined.** Replacing `git archive | tar
-x` with a per-ref detached worktree for the kit trees would drop `tar`
entirely, and the machinery is already present since the binary build adds
worktrees per ref. It is refused here because the archive's committed-content-only
property is what the untagged-`TO` declaration arm reads against, and because
swapping an extraction mechanism the section's own `# spec:` comments describe
is a rewrite inside a port. Recorded as declined rather than unconsidered, so a
later reader does not re-open it as an oversight.

An arm's spawned programs are recorded **in prose and nowhere a machine reads**,
which is gate-sdk's stated position and the open gap
`bridged-arm-requirements-undeclared`. This member carries the heaviest program
set in the class, and it re-instances that gap by construction without widening
`--needs`.

### (9) Criterion 2's oracle is a constructed cross-substrate scenario, and criterion 5's residual is narrower than the class's

**Criterion 2.** The member has no fixture pair and owes none. Its discharge is
the `# no-fixture:` road: the criterion's actual demand is *the same cases, both
substrates, while both implementations exist*, so the port stands the scenario
up and runs both {mechanical}. Concretely: one FROM/TO pair driven through the
shell tool and the arm with both alive, comparing the exit status, the stdout
verdict line and the stderr diagnostics; plus the worktree case delta (6)
names; plus a deliberately red phase-B run, so the 1-versus-2 split is compared
rather than only the clean path. It is bought once, at port time, and it carries
that road's standing limit — it proves the two agree then and nothing
machine-held keeps them agreeing after, which is why the shell original is
**deleted** rather than left running beside the arm. The cost is real and is
stated: the suite already runs the whole battery twice in scratch and builds two
binaries, and the parity run doubles that once.

**Criterion 5, and its residual is unusually narrow.** A vendored consumer on a
host the artifact roster does not cover loses the upgrade smoke outright, where
today it has a shell script that runs anywhere. That is the loss, stated in its
own terms rather than reasoned away — and the class of hosts it reaches is
smaller than the criterion's usual one, because **this suite already requires
`cargo` on `PATH`** for any ref that dispatches to the binary. A host with the
toolchain the suite already demands can build the binary the arm needs. No
`gates.list` row is omitted and the binary-less residual roster does not move,
this cut carrying no gate. The loss is accepted on the ground that the suite is
a validate-stage and pre-release instrument in the kit-source repo rather than
an adopter-facing one.

### (10) Every surface the deletion stales moves in the landing commit, and four of them no gate sees

Probed rather than assumed {mechanical}:

- **`scripts/evidence-config.sh`**, `EVIDENCE_KIT_RUN_upgrade` — re-pointed at
  the front-end arm. The suite **name** stays `upgrade`, which is what keeps
  `.workflow/validate-baseline.txt`'s row valid.
- **`README.md`**'s battery-roster block — the invocation line. It is
  hand-maintained and ruled never generated, and `check-battery-roster` is
  **bidirectional**, so editing the config without editing the roster reds both
  directions at once.
- **`gate-sdk/SPEC.md §upgrade-smoke`** — the whole section is restated for the
  arm: the invocation form, the knob roster with two empty-means-derive
  defaults, the `bash`-driven consumer-smoke seam, the in-crate declaration
  resolve, and every stated honest limit kept. **The `### upgrade-smoke` heading
  is not renamed**: `docs/install.md` links its slug, and `check-md-refs` reds
  on a heading anchor that stops resolving. That is the one thing in this cut
  that a tidy-up would break silently.
- **The four ungated prose repairs of delta (4)** — the three `gate-sdk/SPEC.md`
  claims and `gate-sdk/lib/consumer-smoke.sh`'s `# no-port:` cause. No gate
  reads any of them for truth.
- **Two further `gate-sdk/SPEC.md` narratives** whose premise is this file's
  survival: the paragraph calling it "the third caller" of
  `csmoke_vendor_and_install`, and the `.workflow/` asymmetry paragraph whose
  whole ground is that this tool "resolves the directory through
  `GATE_SDK_WORKFLOW_DIR`" — a bridged arm can declare that knob, so that
  paragraph's open question changes shape rather than merely losing a citation.
- **`.claude/settings.json`** — **no edit owed, probed rather than assumed.** No
  grant names `upgrade-smoke.sh`, so `native-gate-port-remaining-corpus`'
  2026-08-29 settings-grant carve-out is exercised **zero times** by this cut;
  and the ported invocation is already covered by the existing
  `Bash(bash gate-sdk/bin/run-gates.sh *)` grant, so no grant is added either.
  That ruling demands the count be probed rather than assumed, and this is the
  probe with the answer it did not have to have.
- **`bin/run-gates.sh`'s own `--help` text**, which is where a bridged arm's
  usage lives — the class rules that a per-arm help flag would be a second home
  for one sentence.

### (11) The regeneration fan-out this cut stales

{mechanical} Each is rostered with its trigger and regen command in
docs/site-architecture.md §Generated projections and discharged in the landing
commit:

- **The generated `pre-commit` hook**, on two independent triggers: it bakes
  every `EVIDENCE_KIT_RUN_*` value, and it bakes `check-measured-claim`'s
  resolved values, whose `tree-shell-owed` key moves. `check-graph` is the red,
  and docs/site-architecture.md names this exact route — "any tree edit that
  *moves* a measured claim … stales the hooks from a file no manifest names
  either".
- **`docs/check-graph.html`**, asserted fresh with it.
- **`docs/enforcement.md`** and **`docs/value.md`**'s marker block, on the
  suite-command edit.
- **`docs/gate-sdk/SPEC.md`**, the generated on-site mirror, on the section
  rewrite (`check-docs-mirror-fresh`).

**`check-measured-claim` itself is clear**, and the asymmetry is stated because
it is the easiest finding to miss: `tree-shell-owed` moves, but **no tracked
`.md` binds that key with a `measured:` marker**, checked by scanning the
markers rather than assumed, so the movement lands on `check-graph` through the
baked hook value and nowhere else.

### (12) This amendment pairs to `kit-library-port-residue`, and the pairing is forced by arithmetic plus that entry's own text

The host is **`kit-library-port-residue`**, not the standing composer entry
{design-bearing}. Two facts force it and both are measured.

**First, the composer entry admits exactly one `[spec:]` tag and the sibling cut
takes it.** gate-sdk's §The port disposition says a cut "promotes the standing
composer entry `native-gate-port-remaining-corpus` itself, carrying that cut's
own `[spec:]` amendment ref" — a sentence written for one cut per iteration.
`[spec:]` and `[roadmap:]` are both lead-line-scoped and `check-queue-wrap`'s
floor resolves to 100, so that entry's fixed lead-line part is 66 columns and a
tag costs `9 + len(basename)`: one fits at a basename up to 25 characters, and a
second cannot at any naming, the shortest legal basename being `SPEC-a.md` at 9
for a cost of 18 against the 16 that remain. The sentence's insufficiency for a
two-cut iteration is **filed to the gap inbox** as its own finding rather than
absorbed here, because this is the third iteration to re-derive it and the
2026-09-01 ruling that resolved it landed in an amendment that merged and was
deleted.

**Second, this host is the right one rather than the leftover one, and the
ground is the entry's own words.** `kit-library-port-residue` says of
`declaration.sh`: "its stated test is whether the shell caller set empties, and
`bin/upgrade-smoke.sh` keeps it non-empty. **Temporary, not permanent.**" This
cut is exactly the event that sentence is waiting on, so the entry's text
changes *because of* this cut and not incidentally to it.

**What this host does not claim.** The cut delivers **no member** of that
entry's four-member residue — it discharges one member's stated blocker.
canon-kit's corpus branch speaks of an amendment delivering one *increment*, and
a blocker discharge is a narrower thing; it is recorded as such rather than
dressed as delivery, and the entry's member roster is corrected to say
`declaration.sh` is now unblocked and takeable rather than that it is done. The
previous iteration hosted its cut here on the stronger ground of delivering two
of six members, and the difference between the two groundings is stated so the
next reader does not read this as the same claim.

### (13) The demotion re-prices the host entry, and the correction is owed inside its headroom

`kit-library-port-residue` returns to the deferred section under
`[design-pending]` at build — its deliverable is a corpus and this is not even
one increment of it — and canon-kit/SPEC.md §Merging an amendment rules that a
demotion, unlike a Done move, lands the entry **back inside
`check-queue-entry-budget`'s per-entry cap** {mechanical}. The active sections
are uncapped, so the promotion costs nothing; the debt falls due at the
demotion.

The headroom is measured rather than estimated. The entry's extent is
`TASK-QUEUE.md` lines 70–117 = 48 lines, carrying **one** `ruled:` line and no
`recurrence:` line, so its counted size is 47 against
`QUEUE_KIT_ENTRY_LINE_CAP` = 50 — **three lines of headroom**. The `ruled:` line
this promotion would add is *not* free: the discount is at most one line per
grammar and the entry already spends its one. So the build has **two** lines to
spend on rewriting the `declaration.sh` paragraph from *blocked, temporary* to
*unblocked, takeable under its own section*; if the correction wants more, spent
narrative is compressed in place in the same commit that demotes the entry.
Stated here because neither owner states it alone — the cap is queue-kit's and
the demotion is canon-kit's — and because a build meeting it as a red is a batch
spent rediscovering an arithmetic this amendment already did.

## Producers and consumers

The amendment introduces **one interface** — the `--upgrade-smoke` flag and its
`bin/run-gates.sh` front-end arm — and **no new state, no new event, no new
field and no new knob**. Every knob it names is already shipped and already
read; what changes is which process reads it and, for three of them, which file
supplies the default.

- **Producer of the arm** — `native/src/emit/mod.rs`'s `BRIDGED_ARMS` table, one
  row carrying the flag spelling, `Arm::Run` and the five-knob roster. The
  enabling config is the table row itself: `--knobs` publishes the roster and
  `gate_knob_env` resolves it, so nothing must be configured per install, and
  the three defaults delta (5) moves into `lib/gate.sh` are what make that
  resolution succeed rather than refuse with exit 2.
- **Producer of the front-end reach** — `bin/run-gates.sh`'s new arm. Unlike the
  `--emit` family the front-end does **not** compose this flag from an operand;
  an `Arm::Run` member is reached by its own branch, the shape `--usage-poll`
  and `--lesson-sink` already carry.
- **Consumer of the verdict** — two, at two named transitions. The **`upgrade`
  validate suite** reads the exit status at every validate stage and writes it
  into the validate session's evidence file against
  `.workflow/validate-baseline.txt`'s `upgrade upgrade pass` row. A **session**
  reads it at the standing pre-release assertion, `TO` defaulting to `HEAD`, and
  at the release procedure's go/no-go. Neither is created here; both are
  re-pointed.
- **Consumer of the three moved defaults** — `gate_command`/`gate_knob_env`,
  which sources `gate-sdk/lib/gate.sh` to resolve them before it execs the
  binary and **refuses the whole environment** if the library does not define
  them. Named at that transition because it is the only one where their absence
  is visible, and where it is fatal rather than degrading.
- **Consumer of the surviving shell library** — the arm, by `bash` spawn, once
  per helper call: `csmoke_gate_descriptors` before each phase's build decision,
  `csmoke_vendor_and_install` once at the FROM baseline, `csmoke_place_binary`
  once at the phase-A swap. The library is unchanged, so it gains a caller of a
  new kind and loses only a shell one.
- **Consumer that stops existing** — `gate-sdk/lib/declaration.sh`'s non-test
  shell caller set, which empties (delta 4). Its *readers* do not disappear: the
  declaration is still read, in-crate, by `native/src/declaration.rs`, and the
  `--declaration-parity` lane still holds the two forms equal for as long as
  both exist.

**One corpus is narrowed — the tracked non-test `*.sh` tree loses one file — and
its readers' red conditions are enumerated rather than their subjects**
(canon-kit/SPEC.md §The causal-completeness check, point 5):

- **`check-docs-cmd` assertion A** — reds on a repo-relative `.sh` path in
  invocation position inside a fence in a governed doc that resolves to no
  tracked file. **Not monotone**: the deletion *adds* a violation, at
  `README.md`'s battery-roster line. Cleared by delta (10), in the same commit.
- **`check-docs-cmd` assertion B** and **`check-kit-ref-liveness` leg (b)** —
  each reds on a kit-prefixed ALL-CAPS token occurring in no tracked **kit**
  code, the second over every tracked file. **Not monotone**: deleting the one
  file that spells the three `GATE_SDK_UPGRADE_*` knobs adds violations, and a
  Rust `KNOBS` array does not clear them because `native/` is not a kit root.
  Cleared by delta (5)'s move into `lib/gate.sh`, never by inspection.
- **`check-battery-roster`** — reds bidirectionally on a configured suite whose
  normalized command matches no roster line, or a roster line matching no suite.
  Not monotone in either direction. Cleared by delta (10).
- **`check-graph`** — reds when a committed generated hook or the graph artifact
  differs byte-for-byte from its generator's output. Not monotone: the deletion
  moves the baked `tree-shell-owed` value. Cleared by delta (11).
- **`check-docs-mirror-fresh`** — reds on a `docs/` mirror not byte-equal to its
  kit source. Not monotone on the section rewrite. Cleared by delta (11).
- **`check-md-refs`** — reds on an internal markdown link resolving to no
  tracked file or heading slug. **Clear on the deletion** (the only link targets
  the SPEC anchor, not the script) and **red if the heading is renamed**, which
  delta (10) forbids.
- **`check-knob-default-coupling`** — reds on a literal knob default disagreeing
  across its sites or with its owning SPEC, and on a knob whose SPEC states no
  default at all. Delta (5) is what keeps it green: `_TO`'s literal must equal
  §upgrade-smoke's stated `HEAD`, and the two empty-means-derive knobs need
  their SPEC bullets rewritten to state the empty default and name the reader
  that expands it.
- **`check-measured-claim`** — reds when a `measured:` marker disagrees with its
  oracle, or names a key the oracle does not emit. `tree-shell-owed` moves; **no
  tracked marker binds it**, checked by scanning the markers rather than
  assumed.
- **`check-settings-paths`** — reds on a literal repo-relative `.sh` grant that
  does not resolve. Not monotone in general; **clear here**, probed (delta 10).
- **`check-exec-bit`, `check-shellcheck`, `check-comment-tier`,
  `check-path-dialect`, `check-tree-terms`, `check-gate-substrate-parity`,
  `check-gate-exemption-tasks`, `check-evidence-baseline`** — each verdict is
  **monotone** in the narrowing: removing one `.sh` can only remove findings,
  the shellcheck corpus is nowhere near empty, no `.gate` roster moves, the file
  declares no disposition field, and the baseline row survives because the suite
  name does not change. Cleared by inspection.
- **On the addition side**, `check-comment-tier` and `check-path-dialect` reach
  the new crate module from the commit it lands in. `check-path-dialect` is a
  **newly tightened** gate and this member manages absolute paths across four
  boundaries — the per-ref worktrees, the archive extraction, the cargo build
  and the scratch consumer's own battery invocation — which is the largest
  hidden cost in the port and is named here rather than met at build.

**Cross-component signal: this amendment's component set is three** — gate-sdk,
evidence-kit (the suite registry it re-points) and the consumer surfaces under
`scripts/` — and two **sibling** amendments land in `delegation-kit/` this
iteration. So `check-stage-entry` assertion C fires on the
amendment-files-span-two-components arm and the **align stamp is demanded at the
build stage's entry**. Stated here so the build session is not the one that
learns it.

## Existing sections updated

- `gate-sdk/SPEC.md §upgrade-smoke` — restated for the arm throughout: the
  invocation form and its front-end reach, the exit-status contract and why it
  forbids the `--emit-` spelling, the `bash`-driven consumer-smoke seam, the
  in-crate declaration resolve, the knob roster with two empty-means-derive
  defaults, `tar` joining `cargo` on the suite-requirement sentence, and the
  cross-ref battery-summary parse recorded as a coupling. Every stated honest
  limit survives, and the `### upgrade-smoke` **heading is unchanged** (deltas
  2, 3, 4, 5, 7, 8 and 10).
- `gate-sdk/SPEC.md §lib/declaration.sh` — the "only caller … the shell caller
  set therefore does not empty" paragraph becomes its own discharge: the set is
  empty, the disposition is no longer temporary-because-blocked, and the library
  is takeable under its own section by a later cut (delta 4).
- `gate-sdk/SPEC.md`, the two further claims that this file "survives the port"
  and that it "is deliberately not among" the declaration consumers — both
  false after this cut and both repaired (delta 4).
- `gate-sdk/SPEC.md`, the `csmoke_vendor_and_install` caller narrative and the
  `csmoke_place_binary` host-argument paragraph — the third caller is now a
  crate arm reaching the library by spawn (deltas 3 and 4).
- `gate-sdk/SPEC.md`, the `.workflow/` asymmetry paragraph — its premise was
  that this tool resolves the declaration directory through
  `GATE_SDK_WORKFLOW_DIR` as a shell reader; a bridged arm declares that knob,
  so the paragraph's open question changes shape (deltas 2 and 5).
- `gate-sdk/SPEC.md §The non-gate arm` — the class roster gains
  `--upgrade-smoke`; the spawned-program prose gains `cargo` and `tar` under
  this member, and the `Arm::Run`-cannot-take-`--emit-` ruling gains its second
  worked member after `--lesson-sink` (deltas 2 and 8).
- `gate-sdk/SPEC.md §lib/gate.sh` — the three `GATE_SDK_UPGRADE_*` defaults land
  here, two of them empty-means-derive, joining `CONTEXT_KIT_MEMORY_DIRS`'
  precedent class (delta 5).
- `gate-sdk/SPEC.md §The port-candidate criteria` — no ruling changes; named
  because criterion 5's own §upgrade-smoke citation is about this member and a
  reader checking the residual arrives from there (delta 9).
- `gate-sdk/lib/consumer-smoke.sh`'s `# no-port:` cause — the sourcer list loses
  `upgrade-smoke.sh` and gains the crate arm as a spawn-side caller; ungated, so
  it is an explicit target (delta 4).
- `gate-sdk/bin/run-gates.sh` and its `--help` text — the new front-end arm and
  its usage line, the class's stated home for a bridged arm's usage (deltas 2
  and 10).
- `scripts/evidence-config.sh` and `README.md`'s battery-roster block — the
  suite command and its hand-maintained mirror (delta 10).
- `docs/site-architecture.md` — no ruling changes; named because delta (11)'s
  fan-out is read off it (delta 11).
- `TASK-QUEUE.md`, the `kit-library-port-residue` entry — promoted into
  `## New Features` with `[design-pending]` swapped for this amendment's
  `[spec:]` ref, carrying the pairing's own `ruled:` line in the same commit as
  the ruling's content, and its `declaration.sh` paragraph corrected at build
  from *blocked, temporary* to *unblocked and takeable under its own section*.
  It **demotes** at build rather than reaching `## Done`, and the demotion
  re-prices it against a cap it clears by three lines (deltas 4, 12 and 13).

<!-- update-target-exempt: the composer entry takes no write from a cut by its own 2026-08-28 ruling — each closed cut's record lives in the contract section that cut selected, which is delta 1's section — and the sibling amendment's pairing is the only thing that touches its lead line this iteration -->
- `TASK-QUEUE.md`, `native-gate-port-remaining-corpus`'s body — deliberately
  unwritten.

<!-- update-target-exempt: a live design-pending entry this cut deliberately preserves rather than settles; delta 6 states the preservation and the entry takes no write -->
- `TASK-QUEUE.md`, the `upgrade-smoke-refuses-inside-a-worktree` entry —
  deliberately unwritten.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`), the none-remain half discharged at
      the **iteration** rather than at the commit.
- [ ] **Removals propagated** — grepped every spec, README, config file,
      settings file, library header and baseline for the deleted path and for
      the three knob names; nothing dangles, the four **ungated** prose claims
      delta (4) names included.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **The oracle moved, and by the roster rather than by a number** — the
      `--tree` arm lists no `gate-sdk/bin/upgrade-smoke.sh` row, taken as a
      per-file roster diff and not as a trailer delta.
- [ ] **The caller set is empty, proved by the probe rather than by the
      argument** — a grep for sourcers of `gate-sdk/lib/declaration.sh` over
      every tracked `.sh` returns `*.test.sh` files and nothing else.
- [ ] **Parity was executed with both substrates alive** — one FROM/TO pair, a
      deliberately red phase-B run, **and a run from inside a linked worktree**,
      compared on exit status, the stdout verdict line and the stderr
      diagnostics, *before* the shell file was deleted.
- [ ] **The worktree refusal still refuses** — the ported predicate reproduces
      `upgrade-smoke-refuses-inside-a-worktree`'s exit 2, and that entry is
      unchanged and unclosed.
- [ ] **The heading survives** — `### upgrade-smoke` is not renamed, and
      `check-md-refs` is green over `docs/install.md`'s link to its slug.
- [ ] **The regeneration fan-out is discharged in the landing commit** — the
      generated hook, the graph artifact, the enforcement map, the value rollup,
      the `docs/gate-sdk/` mirror and the gate binary.
- [ ] **The host entry demotes inside its cap** — `kit-library-port-residue`
      returns to `## Deferred` under `[design-pending]` with its
      `declaration.sh` paragraph corrected, and its counted size is at or under
      `QUEUE_KIT_ENTRY_LINE_CAP` **in the demoting commit**, spent narrative
      compressed in place if the correction needs more than the two lines delta
      (13) measures.
