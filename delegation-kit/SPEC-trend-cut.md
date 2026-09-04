# SPEC amendment: trend-cut

The port disposition of **`delegation-kit/bin/usage-trend.sh` (120 lines), the one
owed file declaring §Trend reporter**, onto the binary substrate as a bridged
`Arm::Emit` member spelled `--emit-usage-trend`. This is a stated-contract cut under
the port-only run (TRAJECTORY.md §PRIORITY DIRECTIVE), composed at scope and packaged
by the **lead on its own authority**, 2026-09-04, over the resume channel; it did not
reach the operator, and that is stated because a packaging ruling recorded without
its authority reads later as more settled than it is.

**Measured at this HEAD rather than carried from scope's survey**: the port oracle's
`--tree` arm reads 98 files scanned, 64 declared `no-port`, 0 temporarily held,
**34 owed**. The selection ground is that owed column and never the registry
`--group` read (gate-sdk/SPEC.md §port-blockers). §Trend reporter already rules the
tool "advisory tooling, never a gate", so this member's whole port surface is the
non-gate class.

**This member's hidden cost is the fourth recorded axis — cost in another kit's
gate** (gate-sdk/SPEC.md §The first cohort, and the rule that selects the next) — and
this cut's contribution is that the axis is now **exhausted rather than paid**:
§check-assertion-strength's live reach is already zero, so the port no longer
*causes* the zero. What it still costs is the deletion of one option from a live
deferred entry's design space, and delta (8) is that cost stated rather than
discovered at build.

## What changes

### (1) The cut is the one owed file declaring §Trend reporter, and taking it leaves the kit one owed member

`bin/usage-trend.sh` is the **only** owed file declaring `## Trend reporter`
{design-bearing}. Its two siblings declare other sections and neither is available:
`bin/wait-probe.sh` left the tree at the 2026-09-04 cut that discharged
§bin/wait-probe, and `bin/usage-verdict.sh` left at the cut before it. So the
stated-contract composer's *one section, the one amendment* is satisfied by a clean
singleton.

**The remainder is stated so no reader reads a discharged section as a discharged
kit**: `lib/delegation.sh` stays permanently shell as the config bridge's sole
`DELEGATION_KIT_*` resolver (gate-sdk/SPEC.md §The kit-library port disposition) and
never enters the owed column. With this cut, delegation-kit's `bin/` column reads
zero.

**The section was already promised to this cut.** §usage-verdict records at the
moment of its own port that "`bin/usage-trend.sh` (§Trend reporter) declares its own
section and **stays owed**", and §Testing records the same fact from the test side —
"the trend runner above keeps a shell subject and spawns it, `bin/usage-trend.sh`
declaring its own section and belonging to the kit-`bin/` owed cohort still". This
cut discharges both sentences, and both are update targets below.

### (2) `--emit-usage-trend` is an `Arm::Emit`, and the exit contract is what settles the family — the opposite way round from its sibling

The reporter lands as `native/src/emit/usage_trend.rs` and registers as one
`BRIDGED_ARMS` row spelled `--emit-usage-trend`, reachable through the shipped
front-end as `bash gate-sdk/bin/run-gates.sh --emit usage-trend` {design-bearing}.

**The `Arm::Emit` family is an exact fit and the fit is asserted by the tool's own
declared contract, not inferred from its output.** `bin/usage-trend.sh:4` declares
`exit: 0 report emitted, 2 knob unset or history missing/unreadable (never 1 — it
renders no verdict; usage-verdict stays the sole pause authority)`. That is
`Arm::Emit`'s family verbatim: `native/src/main.rs:483-491` maps `Ok(doc)` onto
`exit(0)` after printing the document and `Err(e)` onto `exit(2)` after naming the
error on stderr, and the family can never return 1. The member also *is* a document
producer — a header line plus one block per account and segment — so both halves of
the family's shape hold at once.

**This is the reverse of the ruling its own kit's sibling took, and the contrast is
worth carrying because a reader who met `--usage-verdict` first will expect
`Arm::Run`.** `--usage-verdict` is an `Arm::Run` **because** its three-state exit
carries a 1 a hook grades, which an emitting arm collapses. Here the *absence* of a
1 is contract — §Trend reporter's own words, "never **1** (it renders no verdict; the
verdict stays the sole pause authority)" — so choosing `Arm::Run` would preserve
nothing and would cost the front-end reachability the `--emit` operand gives for
free. The two members sit one row apart in the same table under opposite rulings from
the same test, which is the clearest statement of what that test actually asks.

**And because spelling and grammar are one decision, `--emit-` is forced rather than
chosen** (gate-sdk/SPEC.md §The non-gate arm): `bin/run-gates.sh` composes
`--emit-<name>` from its `--emit <name>` operand, so a member spelled anything else
would be reachable by no shipped front-end. No new `case` arm is added to the
front-end — the `--emit` arm already dispatches it, which is a delta this cut does
**not** owe and which distinguishes it from every other cut in this iteration's
batch.

### (3) The `[history-file]` positional ports unchanged, and the class ruling already answers why

`usage-trend.sh:3` documents `[history-file]` as overriding the configured path "for
test injection", and `:11` reads it as `${1:-$DELEGATION_KIT_USAGE_HISTORY}`
{mechanical}.

This is the **second** kind under gate-sdk/SPEC.md §The non-gate arm's distinguishing
test — an argument the rule itself consumes, arriving as argv into the subcommand,
which "ports unchanged: the subcommand reads its own argv and may override a bridged
knob value with it perfectly well". It is not a selector for where configuration
comes from, so it does not arrive a process too late and it is not deleted. The
identical shape already crossed on this kit's own `--usage-verdict`, so the point is
cited rather than re-argued.

### (4) The argv-shape split binds, and the absorbed argument here is a path — the `--usage-verdict` shape exactly

`usage-trend.sh` handles no `-h`/`--help` and no `--` — probed, not read off the
usage comment {design-bearing}. A first argument of `--help` is assigned straight
into `HISTORY` at `:11`, found unreadable at `:18`, and reported as
`usage-trend: cannot read --help` at exit 2.

**That is the second reader instance's shape, reached in the same kit one section
away.** gate-sdk/SPEC.md §The bin/-tool contract records `--usage-verdict` as the
instance where "the shell tool assigned its first positional straight into the
snapshot path, so `usage-verdict.sh --help` read the flag as a file, found it
unreadable, and printed `cannot read --help … -> STALE` at exit 2". The difference
here is only in what the misread costs: this member renders no verdict, so an
absorbed flag produces a diagnostic rather than a wrong number, and the harm is the
**discoverability** half the contract also measures — a session hunting for the
tool's modes gets a file-not-found where usage belongs.

**So the split binds as the class already rules it**: the shape refusal (a positional
beginning with `-` that is not a recognized option is a refusal — usage on stderr,
exit 2) and the `--` escape **cross** to the arm; the `-h`/`--help` arm **retires to
the front-end**, where the class keeps usage and where `--emit usage-trend --help`
is therefore a refusal rather than an absorption. All three are additions the shell
form never had.

**Not blocked on `bin-tool-help-arm-absent-tree-wide`, and the ground is that
entry's own scope question.** That entry holds its corpus open because whether the
contract's behaviours bind a tool taking **no** positionals is unstated. This tool
takes one free-text positional, so it sits inside the contract's stated scope and the
open question does not reach it. This cut applies the split per member, as that
entry's own sentence anticipates, and rules nothing about the members whose
disposition waits on the scope answer. **A census note is owed at build** — four of
this iteration's cut members sit on that census — and the entry's derivation is not
reproducible as written: it states 20 paths / 17 shipped tools measured 2026-09-04,
while the obvious spelling of its own command returns 19 at this HEAD, three of them
fixtures, so 16 shipped. The pattern is unstated, so the difference is unattributable
rather than a proven staleness; build reproduces the census with the entry's own
pattern before moving the number, and files the unstated-pattern finding.

### (5) The declared knob roster is two names, and neither is minted

`DELEGATION_KIT_USAGE_HISTORY` and `DELEGATION_KIT_PAUSE_PCT_7D` {mechanical}. Both
are the two the shell form already reads, both are defined and defaulted in
`delegation-kit/lib/delegation.sh` (`:29` and `:24`, the second validated numeric at
`:79-80`), and neither is minted by this cut — §Trend reporter's position that the
reporter configures nothing of its own survives verbatim.

The forced-family test settles the registration with nothing left to calibrate: both
defaults live in a `no-port` library the bridge sources, so a hardcoded top-level
flag would resolve neither. `native/src/hook/verdict.rs:10-21` already declares both
names on a live arm and reads them through `walk::knob_scalar`
(`native/src/walk.rs:256-265`), which errors rather than defaulting because the crate
holds no default for a bridged knob — so this member copies a working declaration and
a working reader.

**One consumer fact is worth recording because it decides what the port must not
assume**: this repo overrides `DELEGATION_KIT_USAGE_HISTORY` to `.metric/usage-history.log`
(`scripts/delegation-config.sh:16`) and **does not** override
`DELEGATION_KIT_PAUSE_PCT_7D` at all — probed — so the headroom line this tree prints
runs on the kit default of 95. A port that baked the shipped default into the crate
would work here and break silently for a consumer that sets it.

### (6) The reader is ported; its producer is already compiled and is out of scope

The history log's producer is `--usage-verdict`, which appends one sample line after
every successfully parsed snapshot whatever the verdict
(`native/src/hook/verdict.rs:180-217`, called from each of its five verdict branches)
{design-bearing}. So this cut ports the **reader** of a log a compiled producer
already writes, and the seam's producer side is untouched.

**That asymmetry is what makes the wire shape a contract rather than an
implementation detail, and the port must read it as one.** The line is
space-separated `key=value` tokens with optional keys **omitted rather than emitted
empty**, owned by §The usage.txt contract. The shell reader's `awk` prologue at
`:26-37` is written to that: it splits on the first `=` per token, defaults a missing
`account` or `tier` to `-`, a missing `login_at` to `0`, and a missing `verdict`,
`tokens_in` or `tokens_out` to `-`. The compiled reader keeps every one of those
defaults, because each is the reader's half of the producer's omit-don't-empty rule
and dropping one would turn an absent optional key into a parse failure on a log the
producer is entitled to write.

**And the two-axis emission crosses unchanged**: one `5h` record per sample carrying
`pct`/`resets_at`/`updated_at`, plus one `7d` record when **both** `pct_7d` and
`resets_7d` ride, so a log without the weekly keys yields 5h segments alone rather
than an error.

### (7) The sort is a contract the port must reproduce, and its stability is proved rather than assumed

`usage-trend.sh:43` pipes the axis records through
`LC_ALL=C sort -t<TAB> -k2,2 -k1,1 -k3,3 -k4,4n -k5,5n -k6,6n` {design-bearing} —
account, then axis, then tier lexically, then `login_at`, `resets_at` and
`updated_at` numerically. That ordering is not incidental: the segmenter downstream
keys on the tuple `$1|$2|$3|$4|$5` and flushes on change, so a different order would
silently produce different segments rather than a differently-ordered report.

Three properties the compiled form owes, stated separately because a port could lose
any one:

- **Byte-order collation on the three lexical keys.** `LC_ALL=C` is what makes the
  order locale-independent, and Rust's `str` ordering is already byte-wise, so the
  port needs no locale handling — recorded as a *non-target* because the natural
  reach for a locale-aware comparator would be a silent divergence on a
  non-ASCII account name.
- **Numeric keys with GNU `sort -n`'s tolerance.** The three numeric fields are
  always populated by the `awk` stage, which defaults `login_at` to `0` upstream, so
  a malformed row does not reach the sort with an empty key. The compiled form states
  its own fallback for a non-numeric value rather than inheriting an unstated one.
- **Tie behaviour.** GNU `sort` is not stable without `-s`; Rust's `sort_by` is. Six
  keys leave only the four report-only fields untied, so a full-key tie is rare —
  which is exactly why this is a **prove-by-comparison** item under delta (10) and
  not a reasoned one.

**The statistics are contract too and are ported literally, not re-derived.** The
median-of-3 smoothing with endpoints keeping their own value; the monotonicity flag
that indicts *both* sides of a downward correction and excludes them from rate math
rather than averaging them in; the all-samples-suspect fallback to the raw span so a
segment still reports; the rate in percent per hour with `n/a (single reading)` where
the span is zero; the weekly headroom against the pause ceiling with its
rate-flat-or-negative branch; the first-PAUSE-onset annotation; and the per-account
grouping with `(unstamped)` for an account-less segment. Each has a golden assertion
already, which delta (9) makes the port's parity oracle.

### (8) The port forecloses one option of a live deferred entry, and the entry is owed a note

`assertion-strength-exit-header-reach` (TASK-QUEUE.md) records three candidate
widenings for a gate whose live reach is zero, and its option **(c)** is *give
`usage-trend.sh` an uppercase-token `# exit:` header* — one line against a
still-owed shell file, restoring a nonzero map {design-bearing}. This cut deletes
that file, so **(c) leaves the design space outright**, and what remains is (a) — a
gate-sdk unit of its own, "taken deliberately rather than inside a port cut" — (b),
already refused, and "not worth it".

**The note is landed on that entry in this cut's own commit rather than left for the
entry's own design pass to discover.** Its wording is factual and takes no position
on the value question that entry holds open: option (c) is foreclosed by this cut,
the reach was already zero before it, and the remaining options are unchanged.

**The mechanism is stated so a reader does not think the port broke the gate.** The
gate resolves a callee only through the own-kit-`bin/` convention — it requires
`{kitroot}/bin/{name}` to exist on disk — so a compiled arm is not a callee it can
reach, whatever the arm's exit contract. §check-assertion-strength already rules
that widening as a unit of its own and this cut does not take it. The gate's scan
roots are `smoke/` and `gate-tests/`, never `bin/`, so deleting a `bin/` member
changes what it can *resolve* and not what it *scans*.

**One live drift the cut found and does not fix.** Run at this HEAD the gate prints
`ASSERTION-STRENGTH: clean (107 script(s) scanned; 0 call(s) to a script with a
declared exit contract)`, where §check-assertion-strength and the deferred entry both
state **104**. The reach matches both exactly; only the scanned count has grown as
scripts joined some kit's `smoke/`+`gate-tests/`. That is a dated attestation drifting
against a live figure, and it is routed to the gap inbox rather than corrected inside
a port cut — a mid-session initiative is filed, not started.

### (9) The crate's own test is the parity oracle, it is pre-paid, and it changes shape rather than subject

`native/src/usage_tests.rs:552-595` already drives this exact tool as a test subject
{design-bearing}: it points `DELEGATION_KIT_USAGE_HISTORY` at the kit's static
fixture, runs the script, and asserts nine golden needles, two segment counts, and
both fail-closed arms — unset knob and missing history, each exit 2. The parity
oracle a port owes therefore exists before the port starts, which is this member's
one genuine economy and is why the column understates nothing here for once.

**What changes is the invocation, and the ruling on its shape is already written.**
`:561` resolves `kit("delegation-kit/bin/usage-trend.sh")` and `:563` spawns it
through the `subject()` helper (`:100-129`), which runs `bash <script> <args>` with
the whole `DELEGATION_KIT_` namespace stripped from the child. After the port the
call becomes an **in-process** call to the arm's own function, the shape
`verdict::verdict` already takes at `:502`. A spawned-front-end alternative is
**refused**, and not by this amendment: §Testing already rules it, "re-introducing
the `bash` spawn this port deleted, one process further out". The knob-poisoning
discipline stays — `knobenv::lock()`, `set`, and the paired `remove` — because the
strip is what makes the unset-knob arm meaningful rather than vacuous, and the
in-process form needs it more than the spawn did, not less.

**`:561` is the crate's last literal `kit(...)`-resolved shell-script invocation, and
that is measured rather than asserted.** `git grep 'kit("'` over `native/src`
returns three hits: `:262` and `:557` resolve **data** fixtures, and `:561` is the
one script. The four other `Command::new("bash")` sites in the crate spawn
operator-configured commands rather than an owned kit script. So this cut retires the
pattern outright, and §Testing gains that sentence.

**The two fail-closed arms are preserved as *the arm's own* statuses**, not as the
shell idiom that read them: an unset `DELEGATION_KIT_USAGE_HISTORY` and an unreadable
history are both `Err` into `Arm::Emit`'s `exit(2)`. The third outcome — a readable
history with no parseable samples — stays **exit 0** with the
`no parseable samples … (0 segments)` line, because an empty log is a reading and not
a refusal, and collapsing it into 2 would be the reading-versus-non-reading harm the
`bin/`-tool contract records.

### (10) Criterion 2's discharge is the both-substrates comparison, bought once, before the delete

This member ships no `good/`+`bad/` fixture pair and owes none — it is not a gate
{design-bearing}. §Testing already rules what stands in its place for this kit:
"criterion 2's discharge for a non-gate member is the both-substrates comparison
bought once at port time … before either shell file was deleted", with nothing
machine-held keeping the two agreeing afterwards, which is why the original is
deleted rather than left running beside the crate test.

The comparison this cut owes, stated as a procedure:

- the **whole report**, byte for byte, over `usage-tests/trend-history.log` — the
  header count line, every account heading including the `(unstamped)` form, every
  segment line with its two percentages, span, rate, sample count and suspect count,
  every token-delta line, every weekly-headroom line in both its rate-positive and
  rate-flat forms, and the first-PAUSE-onset line;
- the **same report over a second input that exercises what the fixture does not** —
  a full-key tie, so delta (7)'s stability question is answered by observation; a
  row with `pct_7d` present and `resets_7d` absent, so the both-keys rule is proved
  rather than assumed; and a log whose only lines are comments, so the zero-segment
  branch is compared;
- the **exit status** for each of the three outcomes: report emitted (0), unset knob
  (2), unreadable history (2), plus the zero-segment reading (0);
- the **three new behaviours** of delta (4), which exist in one substrate only and
  are therefore asserted rather than compared.

Nothing in the report is wall-clock dependent — the fixture's own header records that
the reporter measures within-segment deltas and never against *now* — so unlike some
prior cuts every field is compared for equality and none for relation.

### (11) Every path-bearing surface moves in the deleting commit, and here a gate does red

The roster is probed and split by what a gate catches {mechanical}.

**The gate-caught set is non-empty, which distinguishes this cut from its
evidence-kit siblings.** `check-docs-cmd`'s invoked-path assertion requires a fenced
invoked repo-relative `.sh` path to resolve, and `delegation-kit/README.md:83` is a
fenced `bash delegation-kit/bin/usage-trend.sh` invocation. It becomes
`bash gate-sdk/bin/run-gates.sh --emit usage-trend`. `docs/delegation-kit/README.md:88`
is its mirror and moves with it under `check-docs-mirror-fresh`.

**One further fenced site is a judgement rather than a certainty, and build settles it
by running the gate rather than by reading the grammar.**
`delegation-kit/SPEC.md:2837` lists `bin/usage-trend.sh` inside the §Layout and
configuration directory-tree fence, and whether the gate's bare-path form treats a
tree-listing line as an invocation was not established by observation. The line is
edited either way — the file is gone — so the only thing the uncertainty affects is
whether the edit is forced or voluntary.

**The silent set, fixed by hand because nothing reds:**
`delegation-kit/SPEC.md`'s prose mentions at `:2231`, `:2389`, `:2412`, `:2488` and
their `docs/` mirrors; `delegation-kit/README.md:89` (bare-name prose, "`usage-trend`
reads that log...") and its mirror `docs/delegation-kit/README.md:94`;
`drift-kit/SPEC.md:666,1064` and its mirror, which cite the reporter by role rather
than by path; `scripts/delegation-config.sh:14`, a `# spec:` comment naming the file;
`usage-tests/trend-history.log:1`, the fixture's own header comment; `TASK-QUEUE.md`'s
bodies, which the manifest set excludes; and `.workflow/survey-record.md`, a captured
finding no gate cross-checks. The frozen `docs/posts/2026-07-19-checkwright-v0-9-0.md`
is **not** edited — a dated post records what the tree was.

**`delegation-kit/README.md:101-104` and its mirror `docs/delegation-kit/README.md:106`
are re-spelled rather than merely re-pathed, and the finding is theirs regardless of
this cut.** They read "the `usage-verdict` decision table and the `usage-trend`
assertions... retired into the gate binary's crate test lane... and spawn the two
shell subjects (SPEC §Testing)". Probed against `native/src/usage_tests.rs`: the
verdict-table test (`:246`) reads a data fixture (`:262`) and spawns nothing, and
`:561` was already, before this cut, the crate's **only** remaining `kit(...)`-resolved
shell-script invocation (delta 9) — so "the two shell subjects" is inaccurate **today**,
not only after this port. After this cut the count is zero, per delta (9)'s own
sentence that this member retires the pattern outright. Rewritten to say so.

**`.claude/settings.json:48`** carries exactly one grant naming this path,
`Bash(bash delegation-kit/bin/usage-trend.sh)` — probed rather than assumed, as
`native-gate-port-remaining-corpus`'s ruling (2) requires. It is deleted in the same
commit as the file, inside that ruling's carve-out, and **no addition is owed**:
`.claude/settings.json:12` already grants `Bash(bash gate-sdk/bin/run-gates.sh *)`,
which reaches every bridged arm.

**`delegation-kit/SPEC.md §Trend reporter` itself** is rewritten as the arm's section
per deltas (2) through (8). The heading **stays**: five sections of this kit's own
SPEC cite it by name and deleting the heading would dangle all five, where deleting
only the script dangles none.

### (12) The regeneration fan-out this cut stales

Deleting one owed `.sh` moves `scripts/measured-claims.sh`'s `tree-shell-owed` key
(`:42`), whose resolved value is baked inline into the generated
`scripts/git-hooks/pre-commit` (`:369`) {mechanical}. That hook, `commit-msg` and
`docs/check-graph.html` regenerate in the landing commit; the SPEC and README edits
stale their `docs/` mirrors and the crate change stales the binary. `check-graph`,
`check-docs-mirror-fresh` and `check-gate-binary-fresh` are the reds, all discharged
in the landing commit
(docs/site-architecture.md §Generated projections and their freshness gates).

**Two keys that do not move, probed rather than assumed:** this member is not a
registered gate — it has no `.gate` descriptor and no `gates.list` membership — so
the registry-scoped loop at `measured-claims.sh:19-23` never sees it and both
`ported-gate-members` and `gate-substrates` are unaffected. **And no governed prose
binds `tree-shell-owed` behind a `measured:` marker** — probed across `CLAUDE.md`,
`README.md` and every kit SPEC and README — so `check-measured-claim` stays green
from the marker side and the staleness is the baked hook alone.

## Producers and consumers

**New interface: the `--emit-usage-trend` arm.**
*Producer* — `gate-sdk/bin/run-gates.sh`'s existing `--emit` arm, which composes
`--emit-usage-trend` from its operand, resolves the two declared knobs through
`gate_knob_env` and `exec`s the binary (`run-gates.sh:143-148`, `:114-128`). **No new
front-end `case` arm is added**, which is the one delta this cut does not owe. Its
enabling config is not test-only: `scripts/delegation-config.sh:16` points the history
knob at a live log this repo's own compiled `--usage-verdict` appends to on every
dispatch.
*Consumers* — a session running the reporter by hand, which is the tool's only
caller and always has been (§Trend reporter rules it advisory tooling with no
trigger and no tier), and `native/src/usage_tests.rs`'s trend test, which after
delta (9) calls the arm's function **in process**. Each reads the emitted document on
stdout; the test additionally reads the exit status at each of the three outcomes.

**Existing interface whose reader changes substrate: the usage-history line.**
*Producer* — `native/src/hook/verdict.rs:180-217`, already compiled, appending one
`key=value` line per parsed snapshot; its enabling config is
`DELEGATION_KIT_USAGE_HISTORY` being non-empty, which this repo sets.
*Consumer* — the ported reader, which parses the line token by token. **Every field
keeps a named reader at a named transition**, and they are enumerated because the
producer omits optional keys rather than emitting them empty: `updated_at` is read at
the segment's span and rate; `pct` at the smoothing and at the endpoints; `resets_at`
and `resets_7d` at the segment key; `login_at` and `account` and `tier` at the segment
key, with `account` additionally read at the grouping heading; `verdict` at the
first-PAUSE-onset annotation; `tokens_in` and `tokens_out` at the token-delta line,
read only when both are present at both endpoints; `pct_7d` at the weekly axis
record, emitted only when `resets_7d` rides with it.

**New state: none.** No new file, no new record format, no new field, and no new knob.

**Each reader's RED condition, not merely its subject** — binding because delta (11)
*narrows* a corpus (one `.sh` file and one settings grant leave the tree):

- `check-docs-cmd` — reds on a fenced invoked repo-relative `.sh` path that does not
  resolve. Its red condition is a **count of unresolvable paths**, so the narrowing
  can only *add* violations, which is precisely what it does here: the delete makes
  `delegation-kit/README.md:83` unresolvable and the gate reds until the line moves.
  Not clearable by inspection; discharged by the edit.
- `check-docs-mirror-fresh` — reds on a kit surface diverging from its `docs/` mirror.
  A **pairwise equality**, not monotone; both sides are edited and the gate re-run.
- `check-assertion-strength` — reds on a call to a script with a declared exit
  contract whose assertion is weaker than the contract. Its live count is **zero
  calls**, and its scan roots are `smoke/` and `gate-tests/` rather than `bin/`, so
  deleting a `bin/` member cannot add a violation. Safe to clear by inspection, and
  cleared — with the drift in its *scanned* count filed rather than fixed (delta 8).
- `check-comment-tier` — reds on a full-line comment that is neither directive nor
  exempt. Its corpus reaches `*.rs`, and this cut moves directives into it, so it is
  re-run rather than inspected.
- `check-crate-arms` — reds when the crate's lint or test arm fails. Delta (9)
  rewrites a test inside that arm, so it is re-run by construction.
- `check-measured-claim` — reds only on a bound `measured:` marker whose oracle value
  disagrees with it; no marker binds `tree-shell-owed` (delta (12)'s probe), so it has
  no claim to check here and stays green. `check-graph` is what catches the moved
  value, via the baked hook.
- `check-settings-pins` — reds on a settings key whose pinned value has drifted. Its
  subject is the pinned key set rather than the grant list, so removing a grant line
  does not reach it; re-run rather than reasoned about, because the file is edited.

## Existing sections updated

- **`delegation-kit/SPEC.md §Trend reporter`** — rewritten as the arm's section: the
  spelling, the `Arm::Emit` ruling and its contrast with `--usage-verdict`, the
  positional's survival, the argv-shape additions, the declared roster, the wire
  shape's omit-don't-empty reading, the sort contract, and the foreclosed option
  (deltas 2, 3, 4, 5, 6, 7, 8). The heading stays.
- **`delegation-kit/SPEC.md §usage-verdict`** — its sentence that
  `bin/usage-trend.sh` "declares its own section and **stays owed**" is discharged,
  and the log's one remaining in-verdict consumer sentence is left standing
  (delta 1).
- **`delegation-kit/SPEC.md §The usage.txt contract`** — the reader's half of the
  omit-don't-empty rule is stated where the wire shape is owned, so a later producer
  change meets it (delta 6).
- **`delegation-kit/SPEC.md §Layout and configuration`** — the layout tree loses the
  `bin/usage-trend.sh` line; the two knobs' roster entries gain the arm as a declared
  reader (deltas 5, 11).
- **`delegation-kit/SPEC.md §Testing`** — the trend runner's "keeps a shell subject
  and spawns it" sentence is replaced by the in-process form; the crate's last
  literal `kit(...)` shell-script invocation is recorded as retired; the
  both-substrates discharge gains this member (deltas 9, 10).
- **`delegation-kit/README.md`** — the fenced invocation at `:83` re-spelled, the
  bare-name mention at `:89` re-spelled, and `:101-104`'s "spawn the two shell
  subjects" sentence rewritten to state zero (delta 11).
- **`native/src/usage_tests.rs`** — `:561` and `:563` replaced by the in-process
  call; the knob-poisoning discipline retained; three cases added for delta (4)'s
  behaviours (delta 9).
- **`scripts/delegation-config.sh:14`** — the `# spec:` comment naming the script
  re-pointed (delta 11).
- **`.claude/settings.json`** — line 48 deleted in the deleting commit (delta 11).
- **`gate-sdk/SPEC.md §The non-gate arm`** — the `--emit-` roster gains
  `--emit-usage-trend`, recorded as the member whose family the *absence* of a 1
  settles, one table row from the member the presence of a 1 settled the other way
  (delta 2).
- **`gate-sdk/SPEC.md §The bin/-tool contract`** — the reader-instance roster gains
  this member, whose absorbed argument costs discoverability rather than a wrong
  number (delta 4).
- **`gate-sdk/SPEC.md §check-assertion-strength`** — the honest limit gains the
  sentence that option (c) is foreclosed, and the section's stated scanned count is
  left to the gap-inbox filing rather than corrected here (delta 8).
- **`TASK-QUEUE.md` `assertion-strength-exit-header-reach`** — the option-(c)
  foreclosure note landed on the entry in this cut's own commit (delta 8).
- **The generated projections** — `scripts/git-hooks/pre-commit`, `commit-msg`,
  `docs/check-graph.html`, the `docs/` SPEC and README mirrors, and the gate binary
  (delta 12).
<!-- update-target-exempt: a dated release post records what the tree was on its release day and is deliberately not edited by any delta -->
- **`docs/posts/2026-07-19-checkwright-v0-9-0.md`** — named so a later reader does
  not read its stale mentions as an omission.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every field of the usage-history line
      has a named reader at a named transition (delta 6 and the roster above).
- [ ] **Merged with no information lost** — each addition integrated into its proper
      canonical-spec section, not appended.
- [ ] **Amendment deleted** — this file removed on merge; none remain for
      delegation-kit, which this component can satisfy at its own commit because no
      sibling amendment is in flight for it.
- [ ] **Both-substrates comparison bought before the delete** — delta (10)'s
      procedure run in the deleting session, including the stability and
      partial-weekly-key inputs the shipped fixture does not exercise.
- [ ] **The foreclosure note landed** — `assertion-strength-exit-header-reach`
      carries the option-(c) note in the same commit as the delete (delta 8).
- [ ] **Removals propagated** — every surface in delta (11) edited, the settings
      grant deleted in the same commit as the file, and every spec grepped for names
      this change retired.
- [ ] **Gaps filed** — the drifted scanned-script count (delta 8) and the
      unreproducible help-arm census derivation (delta 4) filed to the gap inbox; a
      build-time causal gap is resolved that session, not deferred.
