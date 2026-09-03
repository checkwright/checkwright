# SPEC amendment: scan-prompts-cut

The port disposition of **`guard-kit/bin/scan-prompts.sh` (192 lines), the one
owed file declaring §scan-prompts**, onto the binary substrate as a non-gate arm.
This is a stated-contract cut under the port-only run (TRAJECTORY.md §PRIORITY
DIRECTIVE), composed at scope and ruled by the **lead on its own authority**,
2026-09-04, over the resume channel; it did not reach the operator, and that is
stated because a composition ruling recorded without its authority reads at the
post-port triage as more settled than it is.

**Measured at this HEAD rather than carried from scope's survey**: the port
oracle's `--tree` arm reads 102 files scanned, 64 declared `no-port`, 0
temporarily held, **38 owed**. This cut takes one of that column and **no ported
member is a gate** — guard-kit registers no gates at all (§Overview: "its runtime
surfaces are hooks and advisory `bin/` tools, so nothing joins `gates.list`"), so
this kit's whole port surface is the non-gate and harness-integration classes.

**The cut's argument is a contract retired, not lines moved.** `kpi-prompt-friction`
today shells out to this tool and parses its stdout across a process boundary;
`native/src/emit/kpi/prompt_friction.rs:7` labels that `<distinct>/<total>` string
"an undeclared cross-kit output contract this member parses and guard-kit's scanner
produces". The cut turns it into a typed in-crate call and the undeclared contract
ceases to exist. Delta (6) is that dividend, and it is why this section was
selected over the four other takeable ones the scope survey ranked.

**And the cut is larger than its `lines=` column, which is the sizing fact the
composing session did not have.** The tool composes three primitives out of
`guard-kit/lib/guard.sh`, a **permanently shell** library, so the port owes
compiled twins and the machine-held comparator criterion 6 demands for them
(deltas 3, 4, 5). The 192-line column does not see any of that — the `lines=`
rule at gate-sdk/SPEC.md:4331-4341 is exactly this: "a floor on a port's size and
never a ranking of it". Recorded here rather than at build, and reported to the
lead in the same session, because it moves what "fill the window"
(gate-sdk/SPEC.md:4343-4353) costs on this cut.

## What changes

### (1) The cut is the one owed file declaring this section, and the residue is stated rather than implied

The composer ruling of 2026-08-28 selects a cut **by stated contract** — the owed
files behind one specification section, ported behind the one amendment that
section needs {design-bearing}. `bin/scan-prompts.sh:2` declares
`guard-kit/SPEC.md §scan-prompts` in its own header ("rank recurring prompt
sources from the friction log"), and it is the **only** owed file that does —
verified by reading each owed guard-kit file's first `# spec:` header rather than
by grepping the section title, the distinction the 2026-09-01 survey cut recorded
for the next selector. The kit's three other owed files declare elsewhere:
`bin/compare-settings-allow.sh` §compare-settings-allow, `bin/scratch-run.sh`
§scratch-run, `bin/run-guard-tests.sh` §Testing.

**Taking this cut does not discharge the section**, and the reason is structural
rather than incidental. §scan-prompts' behaviour is composed out of
`lib/guard.sh`'s primitives, and that library is **header-declared `no-port`** on
two independent grounds this kit's own SPEC states (§The guard framework,
`lib/guard.sh`; guard-kit/SPEC.md:299-328): it is the config bridge's sole
resolver for the `GUARD_KIT_*` knobs, and it is the API a consumer's own shell
rules are composed from (§Consumer rules). Both grounds stay live after this cut.
**So this section's contract will never be wholly in-crate while that disposition
stands**, and delta (3) is what the cut does about it instead.

The section is also cited from `lib/guard.sh:253` and from guard-kit/SPEC.md:161-176,
where `guard_split_compound`'s roster names `scan-prompts' allowed()` as one of
its consumers. Delta (3) moves that sentence rather than leaving it false.

### (2) `--emit-scan-prompts` — the ranker as a bridged arm, one flag, one positional, three declared knobs

`bin/scan-prompts.sh` becomes an `Arm::Emit` member of `BRIDGED_ARMS` spelled
`--emit-scan-prompts`, reachable through the shipped front-end as
`run-gates.sh --emit scan-prompts [--count] [--] [<log>]` with no front-end change
{design-bearing}. Its declared roster is three names, every one already defined in
`guard-kit/lib/guard.sh` and therefore declarable without the bridge's
does-not-define refusal (gate-sdk/SPEC.md:76-77): `GUARD_KIT_LOG` (`:23`),
`GUARD_KIT_SETTINGS` (`:25`) and `GUARD_KIT_SETTINGS_LOCAL` (`:26`).

**The family is forced, not chosen**, on §The non-gate arm's own test
(gate-sdk/SPEC.md:2408-2414): the tool resolves three consumer knobs, and a
hardcoded top-level flag "resolves platform defaults and silently ignores every
consumer override". This is also the arrangement guard-kit/SPEC.md:308-315
already anticipated in prose — "**The route is a ported non-gate arm, not a
gate**" — so the cut takes a route its own kit's SPEC named before it existed.

**No knob default moves in this cut.** All three defaults live in `lib/guard.sh`,
which stays exactly where it is, so the hazard gate-sdk/SPEC.md:2490-2496 names —
a default left beside the compiled reader, sourced by nothing, resolving silently
empty — cannot arise here.

The contract survives explicitly, behaviour by behaviour:

- **The three-way split of the friction log** — committed-covered (silently
  granted, off every list), prompting (the headline, ranked), overlay-covered (a
  separate visibly-advisory section, never mixed into the headline). Unchanged.
- **Matching is per compound segment**, so a whole-string glob spanning a compound
  the harness would split and refuse does not read as allowed. This is the
  behaviour delta (3) has to reproduce and delta (4) has to hold equal.
- **The ranking key** — leading binary, plus subcommand for the thirteen
  multi-command binaries, plus the write-shape suffix, word and suffix both read
  off the **first** segment. Unchanged, including the fd-dup exclusion and the
  read-redirect exclusion.
- **`--count` emits `<patterns>/<occurrences>`**, overlay-covered excluded.
  Unchanged in spelling and semantics; what changes is who reads it (delta 6).
- **An explicit file argument overrides the log path, and the two compose in
  either order.** Ported unchanged — delta (10) states the test that says so.
- **Every advisory line, the triage-criterion block and the clear-the-log
  instruction are byte-preserved**, since their reader is a close-stage session
  following `templates/close-triage.md` and the two must keep agreeing.

`-h` / `--help` **does not port**. Usage for a bridged arm lives in
`run-gates.sh`'s own help and in `guard-kit/README.md`, the disposition
`--emit-queue-counts`, `--emit-file-survey`, `--emit-file-gap` and `--emit-kfric`
each took (gate-sdk/SPEC.md:2416-2434). The tool has no help arm today, so nothing
retires — but delta (9) is where that absence stops being free.

### (3) Three primitives get compiled twins under criterion 6's *unless* clause, and `lib/guard.sh` does not move

`bin/scan-prompts.sh` calls exactly three functions out of the permanently-shell
library, and the set is enumerated rather than named one at a time
{design-bearing}: `guard_split_compound` (called at `:83`; defined
`lib/guard.sh:254-256`), `guard_skeleton` (`:105`; `lib/guard.sh:116-251`) and
`_guard_redirect_pairs` (`:98`; `lib/guard.sh:806-808`). Enumerating the **set**
rather than the one helper the cut noticed first is evidence-kit's lesson at
gate-sdk/SPEC.md:3559-3574, where the *unless* clause was found to bind on
`ek_pid_alive` **and** `ek_lock_read` together.

**The governing rule is criterion 6's *unless* clause, not silence.** Criterion 6
reads "*Its corpus derivation is self-contained*, unless the duplication the port
creates is machine-held" (gate-sdk/SPEC.md:3492-3493), and the qualification is
"not the presence of a shared call but whether a machine notices when the two
sides diverge" (`:3505-3508`). The tree carries four live instances of exactly this
disposition — queue-kit's `queue_live_slugs` and section regexes, gate-sdk's
`gate_staged_matches`, evidence-kit's `ek_pid_alive`/`ek_lock_read`, context-kit's
`tool_floor_parse`/`tool_floor_check` — each a permanently-dual predicate held by a
parity harness in its owning kit's `gate-tests/`.

**The `no-port` grounds do not reach these three, and that is checked rather than
assumed.** Ground (1) is about **knob-value resolution**, and none of the three
resolves a knob — each is "a pure function of its arguments: no config read, no
subprocess, no global" (guard-kit/SPEC.md:159-160). Ground (2) is about the
**documented `guard_*` surface a consumer composes its own rules from**, and
guard-kit/SPEC.md:1344-1346 already carved this exact call site out of it:
`_guard_redirect_pairs` and its siblings are "`_`-prefixed internal helpers rather
than the documented `guard_*` surface, called from a `bin/` tool inside the same
kit — a kit-internal call that widens no consumer contract". So the file stays
`no-port` for both stated reasons and the three predicates are separately twinned,
exactly as `lib/toolfloor.sh` and `lib/queue.sh` stay `no-port` while carrying
compiled twins.

**The duplication is permanent, and that is what makes the disposition the right
one rather than a concession.** The shell caller set for these three never empties:
their live callers are `lib/guard.sh`'s own rules 8/12/14/15/17/18/19/20/22 and the
read-compound carve-out of rules 9/10, which are themselves functions in the same
permanently-shell file (guard-kit/SPEC.md:378-380). This is the queue-kit shape
(gate-sdk/SPEC.md:3532-3534), not the `lib/declaration.sh` shape, where the
disposition was "temporary rather than permanent, the stated test being whether the
shell caller set empties" (`:8555-8567`). Here it cannot.

**What the section's own guarantee becomes, stated because leaving it standing
would make it false.** guard-kit/SPEC.md:163-167 says `guard_split_compound` is
"the single implementation every consumer that reasons *per segment* shares
(rules 8/12/14/15/17/18/19/20/22, the read-compound carve-out of rules 9/10, and
scan-prompts' `allowed()`), so the harness's per-segment matching surface is
modelled in exactly one place and cannot drift between them." After this cut it is
modelled in **two** places and the drift is prevented by a comparator instead of by
uniqueness. This kit has already recorded that same transition once, for a
different mechanism, in its own words: the three ported hook arms "reproduc[e] the
same postures in their own compiled module ... which makes this library one of two
independent producers of that envelope shape rather than the sole one"
(guard-kit/SPEC.md:289-294). The precedent is guard-kit's own, not an analogy
borrowed from another kit — which matters because ruling (1) requires the ground be
"the porting kit's OWN governed surfaces ... never analogy".

### (4) `--guard-lib-parity` and its harness — the machine that notices the divergence

The *unless* clause is discharged by a new bridged `Arm::Run` member,
`--guard-lib-parity <mode> <arg>...`, and a new harness
`guard-kit/gate-tests/guard-lib-parity.test.sh` {design-bearing}. Three modes, one
per twinned predicate: `split`, `skeleton`, `redirect`. Shape and grammar are
`--toolfloor-parity <mode> <arg>...`'s, which is the closest precedent in two
respects — a permanently-shell holder, and a surviving shell caller outside the
battery.

The harness follows `queue-kit/gate-tests/queue-lib-parity.test.sh` exactly: it
feeds **one canned corpus** to both holders and compares their **classification**,
A against B directly with **no committed expected file**, because "a maintained
golden would be a third copy to drift, and the failure this exists to catch is one
side edited without the other". It is run by gate-sdk's `run-gate-tests.sh` like
every other `gate-tests/*.test.sh` and must exit 0.

**The arm's named caller is that harness, and unlike `--declaration-parity` it does
not retire.** gate-sdk/SPEC.md:2232-2243 rules that "a parity arm's caller is the
second holder, so the arm retires with it", and refuses an arm that "can only
skip". Here the second holder is permanently shell on two structural grounds, so
the arm is durable by construction — the property `--toolfloor-parity` already has
and the one `--declaration-parity` lost.

**The corpus is scoped to the branches the comparison is bought for**, which is
evidence-kit's own refinement of coverage-by-assertion (gate-sdk/SPEC.md:3568-3571)
and is stated here because no section rules where a 135-line normalizer's compared
corpus stops. The scope: the shapes a **friction-log line** can carry — single- and
double-quoted spans (including a backslash escape inside a double-quoted one),
a bare backslash escape, `<<<` here-strings, `<<`/`<<-` heredoc headers, every
statement separator in and out of quotes, write and read redirects with and without
a descriptor, and fd-dups. Delta (5) states why that scope is a property of the
input rather than a convenience.

### (5) The skeleton twin implements the reachable subset, and the unreachable half is unreachable by a stated property of the log

`guard_skeleton`'s largest branch is its heredoc-body machinery — the `$'\n'` case
at `lib/guard.sh:209-244` and the `pending`/`pending_q` queues that feed it
{design-bearing}. **That branch cannot fire on a friction-log line**, and the
reason is structural, not statistical: `guard_log_fallthrough`
(`lib/guard.sh:103-107`) writes `printf '%s' "$1" | tr '\n\t' '  ' | cut -c1-500`,
so **every logged line is newline-free by construction**, and the tool reads the
log with `while IFS= read -r line`. With no newline in the input the `$'\n'` case
is never reached, the pending queue is never drained, and no `HD` placeholder is
ever emitted — a `<<TERM` header is emitted verbatim as its own header text and
nothing follows it.

So `guard::skeleton` implements the contract for **newline-free input** and omits
the body machinery. Omitting it is the rule rather than an economy: a branch with
no reader is the same defect as a field with no reader (canon-kit/SPEC.md §The
causal-completeness check, point 4), and shipping 40 lines of Rust that nothing can
call would be dead weight the parity harness could not even exercise.

**The precondition is carried in the code, not only in this sentence.** The twin's
entry point takes one log line and the arm's own reader is what guarantees the
shape; a newline-bearing input is out of contract rather than silently
mis-normalized, so the omission cannot become a landmine for a later caller. The
parity harness's corpus is newline-free for the same reason, and that scope is
recorded beside the corpus rather than left to be re-derived.

### (6) The KPI's cross-kit output contract retires — the cut's dividend

`native/src/emit/kpi/prompt_friction.rs` today resolves the tool with
`sibling_tool(&ctx.kit_roots, "bin/scan-prompts.sh")` (`:22`), spawns
`bash <scanner> --count` (`:26`), and parses the stdout with `parse_count` (`:10`),
whose own `spec:` comment at `:7-9` calls `<distinct>/<total>` "an undeclared
cross-kit output contract this member parses and guard-kit's scanner produces; the
shape check is what keeps a changed spelling a visible `n/a` rather than a wrong
number" {design-bearing}.

After the cut the member calls the ported counter **in-crate** and receives two
integers. There is no process boundary, no stdout, no string, and therefore **no
undeclared contract and nothing for a shape check to defend**. `parse_count`, its
`#[cfg(test)]` test `the_cross_kit_count_is_read_only_in_its_declared_shape`, and
the `proc::run` spawn are deleted with it.

**Two of the member's four `n/a` degrades retire with the boundary**, and this is
the honest accounting rather than a claim that the row got better:
`n/a (scanner failed)` and `n/a (unreadable count)` both describe failure modes of
a spawn-and-parse that no longer happens. `n/a (no friction logged)` is unchanged —
it is a property of the log, not of the transport. `n/a (guard-kit absent)`
survives and delta (7) re-witnesses it. This is the shape drift-kit/SPEC.md:172-174
already records for `kpi-settings-local`, which "has one degrade fewer than its
shell original" for the same reason on a different dependency.

**`--count` keeps a named caller after its machine reader leaves**, which is worth
stating because a mode losing its only in-tree consumer is exactly what §The
non-gate arm calls dead weight. Its callers after the cut are a **session**
reaching it through the front-end and `gate-tests/scan-prompts.test.sh`; a session
front-end invocation "counts exactly as a stage step does"
(gate-sdk/SPEC.md:2264-2268), the disposition `--emit-queue-index`'s `extent` mode
already carries.

**The KPI's numerator does not step at this landing**, and that is checked rather
than assumed: the keying is unchanged (delta 2), so both `<patterns>` and
`<occurrences>` read exactly what they read before. §scan-prompts' standing
obligation to record a step with its pre-change reading is therefore discharged by
recording that there is none.

### (7) The presence witness moves from a `bin/` tool to the permanently-shell library

`n/a (guard-kit absent)` is today witnessed by the existence of
`bin/scan-prompts.sh` under some kit root {design-bearing}. The cut deletes that
file, and the compiled counter is in the binary whether or not guard-kit is
vendored — so without a new witness the row would report a number for a consumer
that has no friction log, no allowlist reader and no guard.

The witness becomes `lib/guard.sh`, resolved through the same `sibling_tool`
helper. It is the **right** witness rather than the nearest one: it is the file
whose presence *is* guard-kit being vendored (the `GUARD_KIT_LIB` indirection every
consumer copy sources), and it is permanently shell on two structural grounds, so
no later cut can delete it out from under this reader — which is precisely what
this cut is doing to the current witness.

**The bridge is a second, independent guard on the same fact and is named so the
redundancy is legible rather than accidental:** the arm declares three
`GUARD_KIT_*` knobs, and `gate_command` resolves them by sourcing
`guard-kit/lib/guard.sh`. A tree without guard-kit cannot resolve them at all. The
KPI's own witness is what keeps the *row* readable; the bridge is what keeps the
*arm* honest.

### (8) The settings read leaves `jq`, and the claim is bounded where it is true

`bin/scan-prompts.sh:29-33` reads both settings files through `jq`, and the read
ends `2>/dev/null || true` {design-bearing}. The compiled arm parses them with the
crate's own `json` module instead — the `kpi-settings-local` disposition
(drift-kit/SPEC.md:172-174), where "the compiled member parses the overlay itself,
so it carries no external-program dependency an absent `jq` could take".

**This closes a silent inflation, not merely a dependency.** With `jq` absent
today, `read_allow` returns nothing, `ALLOW` and `ALLOW_LOCAL` are empty, and
*every* logged command reads as prompting — the tool reports a large, plausible,
entirely wrong number at exit 0, and the KPI records it as a trend. That is the
same failure class as delta (9)'s and it goes the same way.

**"The cut retires `jq`" is false in every direction but this tool's**, and the
bound is stated in the model gate-sdk/SPEC.md:5030-5035 sets. `lib/guard.sh` shells
to `jq` at `:50`, `:63`, `:76`, `:87`, `:92`, `:98`, `:260` and `:269`;
`bin/compare-settings-allow.sh` at `:27-28`; `bin/run-guard-tests.sh` at `:14`,
`:65`, `:79`, `:81`; `smoke/install.sh` at `:12`. Criterion 7's report is over the
**battery**, which this member never joined, so the cut moves the battery's floor
by nothing at all. What it moves is one `bin/` tool's dependency, and saying more
than that would be the false claim.

### (9) The argv shape refusal crosses the port for a *reader*, and the silent zero it closes is attested

§The bin/-tool contract (gate-sdk/SPEC.md:756-767) rules three behaviours for a
`bin/` tool whose positional arguments are free text — `-h`/`--help` to stdout at
exit 0, a refusal at exit 2 for an unrecognized `-`-prefixed positional, and `--`
ending option processing. `bin/scan-prompts.sh`'s argv loop (`:14-21`) is
`--count`, empty, or `*) LOG="$1"` — **no help arm, no refusal branch**, and its
positional is a path, which is free text by the section's own definition (an
uninterpreted caller-supplied string, not one drawn from a known set)
{design-bearing}.

**The consequence is attested rather than argued.** Against the live log — 200
lines, true reading `13/36`:

- `--count --nonsense` returns **`0/0`** at exit 0.
- `--counts`, a one-character typo of the tool's own flag, reports
  **`PROMPT-FRICTION: clean`** at exit 0.

That is the identical failure mode §scan-prompts already condemns in its own words,
one door over: the single-argument parse it replaced "made `--count <path>` return a
real-looking number for the *default* log, which is the worst failure mode an
instrument has: silent, plausible, and reached for first by exactly the measurement
session that cannot afford it" (guard-kit/SPEC.md:1379-1383). That fix closed one
door and left this one open, and no gate reads the contract — §The bin/-tool
contract rules at gate-sdk/SPEC.md:795-797 that none does, which is how the
violation survived unremarked.

So the ported arm **refuses an unrecognized `-`-prefixed argument with usage on
stderr at exit 2**, and `--` ends option processing. This is conformance to a
contract that already binds this file, not a new behaviour the port invents: no
governed sentence asserts that an unknown flag is a log path, and the section's own
prose demands the opposite.

**It does extend the crossing clause, and that is recorded rather than slipped
through.** gate-sdk/SPEC.md:786-793 argues the shape half outlives a member's port
because "the refusal exists because free text reaches a *capture*", and names three
capture members as its instances. This is the first **reader** to take it. The
ground is available and is stronger here than the capture ground, not weaker: a
capture's defect writes a bad line into a surface a human later reads, while a
reader's writes a wrong number into a trend nothing reviews. Delta stated at
gate-sdk/SPEC.md §The bin/-tool contract, which is why this amendment's component
set is three.

The `-h`/`--help` half does **not** cross — it retires to the front-end with the
rest of the class, so `--emit scan-prompts --help` is a refusal.

### (10) The log positional ports unchanged, and the test that says so is named because the naive read deletes it

§The non-gate arm rules that "a gate argument that selects where configuration
comes from cannot survive a port" (gate-sdk/SPEC.md:2310-2323) {mechanical}. The
log positional overrides `GUARD_KIT_LOG`, a knob the bridge resolves, so the naive
read deletes it — and that read is the error the section names at `:2331-2334`:
"an argument is not unportable merely by being an argument."

Applied per the distinguishing test (`:2325-2330`): an argument is unportable when
it redirects something `gate_command` has **already resolved** and therefore
silently changes nothing. This one selects the rule's **input corpus** — which log
to analyse — and the rule itself consumes it, which is `check-gate-tamper`'s
`--fixture` and `check-amendment-queue`'s knob-backed queue-file positional, both
named in that section as ports-unchanged shapes. It ports unchanged, in both
orders with `--count`, which §scan-prompts pins in prose and
`gate-tests/scan-prompts.test.sh:100-111` pins as a test.

### (11) The behavioural oracle is re-homed, not narrowed

`guard-kit/gate-tests/scan-prompts.test.sh` (116 lines) is the only behavioural
oracle for this tool and it stays one {design-bearing}. It asserts the three-way
split, the overlay-off-headline rule, the compound-splitting fix, the silent
omission of an all-committed compound, `--count`'s true count, the clean-with-worklist
baseline, the four write-shape cases (create, append, fd-dup, first-segment
non-attribution), the `4/4` shape count, and the argument override in both orders.

Every one is re-homed onto the arm: the case set is re-pointed at
`run-gates.sh --emit scan-prompts` so the end-to-end path keeps a holder, and the
key-derivation cases additionally land as `#[cfg(test)]` tests in the ported
module where `check-crate-arms` runs them. **It is re-homed, not deleted** — it is
a `.test.sh` outside the port corpus by the suffix rule, and it is the only thing
that reads the tool's *output shape* rather than its primitives.

It gains two cases for delta (9): `--count --nonsense` is a refusal at exit 2 with
nothing on stdout, and `--` admits a dash-prefixed log path. The second is the
non-firing pair the first needs, since a refusal with no escape is a capability
loss rather than a fix.

**What this suite does not become is the parity oracle.** Delta (4)'s harness is a
separate file because it compares two *implementations* over a corpus, while this
one asserts one implementation's *output* — and §Testing's own reasoning at
guard-kit/SPEC.md:1738-1754 (that `bin/scan-prompts.sh` cannot substitute for the
decision table, on a structural ground) is the same distinction one level down.

### (12) Every path-bearing surface moves in the deleting commit

The count is probed rather than assumed {mechanical}:

- `.claude/settings.json` — **exactly two** grants name the deleted path, at `:34`
  and `:35`: `Bash(bash guard-kit/bin/scan-prompts.sh)` and its `*`-suffixed twin.
  **Both are in `check-settings-paths`' scope, probed rather than assumed**: the
  gate skips a candidate *containing* `*`, but that rule "is scoped to the *command
  token*, not the entry", and the twin's command token "is as literal — and as
  strandable — as the bare form's, so it stays in scope"
  (context-kit/SPEC.md:922-927). Both therefore red on the delete without their
  removal. Removing a grant whose target a ruled port cut deletes is outside the
  2026-08-22 bar under the operator's 2026-08-29 settings-grant carve-out, and the
  removal lands in the same commit as the delete. **No replacement grant is needed**,
  probed: `Bash(bash gate-sdk/bin/run-gates.sh *)` is already granted.
- `guard-kit/README.md:66-67` — the two-line invocation roster, both forms.
- `guard-kit/templates/close-triage.md:5` — the close step's step 1, which names
  the invocation verbatim. This template is **not spliced** into a consumer copy in
  this tree: `.claude/commands/close.md:21-22` routes to it *by path*, so
  `check-template-copy-parity` has no second side to hold here and the template is
  the only edit.
- `guard-kit/SPEC.md` — `:33` (§The friction loop step 2), `:1604` (the layout
  roster line), `:1748` (§Testing's structural-limit sentence), plus §scan-prompts
  itself and the §The guard framework roster at `:161-176`.
- `drift-kit/SPEC.md:242-249` — `kpi-prompt-friction`, whose text names
  `scan-prompts.sh --count` and asserts the `^[0-9]+/[0-9]+$` contract deltas (6)
  and (7) retire.
- `native/src/emit/kpi/prompt_friction.rs` — `:1`, `:7-9`, `:22`, `:26`, `:33`
  (delta 6).
- `guard-kit/gate-tests/scan-prompts.test.sh:8` — the `SCAN=` path (delta 11).
- The two `docs/` mirrors of the edited README and SPECs, which are generated and
  are delta (13)'s.
- **Not on this list, named so the omission is legible:** `guard-kit/lib/guard.sh`
  does not move (delta 3), and the `.workflow/prompt-friction.log` itself is
  untouched — it is a capture surface with its own reclaim path and nothing in this
  cut rewrites it.

### (13) The regeneration fan-out this cut stales

Deleting one owed `.sh` file moves `measured-claims.sh`'s `tree-shell-owed` key
(`scripts/measured-claims.sh:42`), read off `--emit port-blockers --tree`'s trailer
{mechanical}. docs/site-architecture.md §Generated projections rules that a tree
edit moving a measured claim stales the generated `pre-commit` and `commit-msg`
hooks — the baked invocation carries `check-measured-claim`'s resolved values — and
`docs/check-graph.html` with them. The edits to `guard-kit/SPEC.md`,
`guard-kit/README.md`, `drift-kit/SPEC.md` and `gate-sdk/SPEC.md` additionally stale
their on-site mirrors. All are rostered with their regen commands in that section
and are discharged in the landing commit; `check-graph`, `check-docs-mirror-fresh`
and `check-gate-binary-fresh` are the reds.

### (14) The criterion-5 residual is one advisory tool, and the loop it serves does not move

Criterion 5 asks what a consumer whose payload carries no artifact for its host
still has after the cut {design-bearing}. This cut has **no gate in it** — guard-kit
registers none — so the binary-less leg's omitted-member roster and its non-zero
count do not move at all.

What such a consumer loses is one advisory ranking. **The friction loop itself does
not move**: the guard still blocks, steers and logs (`templates/bash-guard.sh` and
the consumer copy are both permanently shell), the log still accrues, the triage
criterion is still the section's, `bin/compare-settings-allow.sh` still runs, and
the close step's other four numbered items are untouched. What is lost on an
artifact-less host is step 1 of five — the ranking — and the honest statement is
that such a consumer triages an unranked log by reading it, which is worse and is
not nothing. Stated in those terms rather than behind an "advisory tooling" label.

### (15) The queue entry is promoted at exactly zero headroom, and the demotion is priced now

`native-gate-port-remaining-corpus` is promoted by swapping `[design-pending]` for
`[spec: SPEC-scan-prompts-cut.md]` on its lead line {mechanical}. Both numbers are
measured, not estimated:

- The gate's own headroom line (`GATE_SDK_VERBOSE=1 run-gates.sh --only
  check-queue-entry-budget`) reads **`native-gate-port-remaining-corpus: 0 lines of
  headroom`** against the 50-line cap. The promotion is therefore **line-neutral by
  necessity**, and this cut's record lands in guard-kit/SPEC.md §scan-prompts — the
  entry's own rule that "each closed cut's record ... lives in the contract section
  that cut selected" — never on the entry.
- The lead line is 83 columns; less `[design-pending] ` it is a 66-column base. The
  **bare-basename** ref is 32 columns, for 99 against `QUEUE_KIT_WRAP_BUDGET=100` —
  one column to spare. The repo-relative spelling `guard-kit/SPEC-scan-prompts-cut.md`
  is 42 columns and would overflow at 109. canon-kit/SPEC.md:765-766 admits either
  form; only one fits, and it is chosen for that reason rather than by house style.

The entry **demotes** at build and never reaches `## Done`, which its own body
rules. The demotion re-prices it against `check-queue-entry-budget`, unlike a Done
move, so at zero headroom the demoting commit may add no line to it — which is the
constraint build meets, stated here so build does not discover it.

### (16) The provenance seam: three command literals cross into the binary, and each is examined rather than waved through

The tool carries three hardcoded command vocabularies, and a port moves them out
of a vendored kit file and into the shipped binary, so the seam is ruled here
rather than assumed to travel {design-bearing}. CLAUDE.md §The provenance seam is
a **privacy** boundary before it is a design one, and guard-kit is the kit that
states the strictest form of it — §compare-settings-allow's knob roster ships **no
default probes** because "every string naming a command is the consumer's
vocabulary, never the kit's" (guard-kit/SPEC.md:1652-1653).

- **`GIT_RO` and `DOCKER_RO`** (`bin/scan-prompts.sh:37-38`) model **the harness's
  own built-in read-only auto-allows**, which §scan-prompts already names as one of
  the three things the tool filters against. They are not any project's toolchain
  knowledge: guard-kit/SPEC.md:402-411 rules exactly this case — "A harness tool
  name is public, documented, and shared by every consumer of that harness, so it
  is not private rule content and the provenance seam ... does not reach it. What
  such a literal does cost is **portability**." That cost is unchanged by the
  substrate — a second harness with a different built-in set would force a
  configurable slot whether the set sits in bash or in Rust — so the port neither
  pays it nor pre-pays it, and mints no knob.
- **The thirteen multi-command binaries** the ranking key sub-keys on
  (`bin/scan-prompts.sh:113-116`) are **shell-substrate knowledge**, the class
  §The generic ruleset admits alongside harness behaviour (guard-kit/SPEC.md:378-380).
  They name no project's toolchain and no consumer's vocabulary; §scan-prompts
  already describes the set generically as "the common multi-command binaries", and
  the *measurement* of which words the write-shape suffix bites (`cat`, `awk`,
  `grep`, `python3`, `git`, `sed`, `echo`/`printf`) is prose in that section, not a
  kit literal, and stays there.
- **What is consumer config and stays consumer config** — the two settings files
  themselves. The arm reads a consumer's own `permissions.allow[]` through
  `GUARD_KIT_SETTINGS` and `GUARD_KIT_SETTINGS_LOCAL`; not one allowlist string
  crosses into the crate, and the arm ships no default allow entry of any kind.
  This is the `graph-vocab.sh` shape: the mechanism is the kit's, the vocabulary is
  the consumer's file.
- **What is this project's provenance and does not ship** — every dated ruling,
  authority and refused alternative in this amendment. They belong to TRAJECTORY.md
  and to git history; the merged §scan-prompts states the rules undated, which is
  the 2026-09-03 consult's own ruling.

**Nothing becomes new consumer config in this cut**, and that is the finding
rather than an omission: the three knobs already exist, the harness sets are
already kit-owned on a ruling that does not turn on substrate, and the only thing
the port could have minted — a knob for the built-in auto-allow set — would be
designing against a harness that does not exist.

## Producers and consumers

The amendment introduces **two interfaces** — two bridged flags — and **no new
state, no new event, no new field, and no new knob**. All three knobs named are
already shipped, already defaulted in `guard-kit/lib/guard.sh` (which stays
permanently shell as the bridge's sole `GUARD_KIT_*` resolver, so no default moves)
and already read by the script being replaced.

- **Producer, arm one** — `native/src/emit/mod.rs`'s `BRIDGED_ARMS` table, one row
  (`"--emit-scan-prompts"`, `Arm::Emit(scan_prompts::emit)`, `scan_prompts::KNOBS`),
  with no `run-gates.sh` change: the front-end composes `--emit-<name>` from its
  `--emit <name>` operand and passes the remaining argv, a leading `--` included,
  through untouched. The enabling config is the table row itself — `--knobs`
  publishes the roster and `gate_command` resolves it before the exec — so nothing
  must be configured per install.
- **Consumer, arm one** — a **close-stage session** running step 1 of
  `templates/close-triage.md`, at the close stage's tooling-friction triage. The
  channel is live wherever guard-kit is vendored because all three knobs carry
  shipped defaults. Its stdout ranking and its triage-criterion block are read by
  that session at that transition; the overlay-covered section by the same session
  at the same one, feeding step 4's promote-or-prune.
- **Producer, arm two** — the same table, one row
  (`"--guard-lib-parity"`, `Arm::Run(guard::parity)`, empty roster). Its roster is
  empty in the *reads-nothing* sense gate-sdk/SPEC.md:2443-2452 records for
  `--emit-md-section`: the modes take their corpus on argv and resolve no knob.
- **Consumer, arm two** — `guard-kit/gate-tests/guard-lib-parity.test.sh`, run by
  gate-sdk's `run-gate-tests.sh` at every battery run and at commit time. It is the
  second holder criterion 6's *unless* clause requires, and it does not retire,
  because the shell holder is permanent (delta 3).
- **Consumer of the counter, in-crate** — `native/src/emit/kpi/prompt_friction.rs`,
  which calls the ported counting function directly and receives two integers. This
  replaces a spawn and a stdout parse; the transition is the drift report's
  `kpi-prompt-friction` row, in both full and `--trend` modes.
- **Consumer of the declared roster** — `gate_command`, which resolves the three
  knobs by sourcing `guard-kit/lib/guard.sh` and refuses the whole environment for a
  knob it does not define. All three are defined there today at the lines delta (2)
  cites, which is why this cut moves no default.

**Both arms have a caller that is not the arm's own fixture**, which is §The
non-gate arm's third property. For `--emit-scan-prompts` it is the close-stage
session; for `--guard-lib-parity` it is the harness, which is a *second holder*
rather than a fixture of the arm — the distinction gate-sdk/SPEC.md:2228-2231 draws
for `--queue-parity`, `--evidence-lib-parity` and `--toolfloor-parity`.

**The arm's spawned-program set shrinks to nothing.** The shell tool spawns `jq`,
`sed` (inside `guard_split_compound` and `allowed`), `grep` (inside
`_guard_redirect_pairs`), `sort`, `wc` and `tr`; the compiled arm spawns none of
them. Recorded in prose because a `BRIDGED_ARMS` row carries no requirement element
and `--needs` answers about registry members only (gate-sdk/SPEC.md:2270-2276).

**One corpus is narrowed, and its readers' red conditions are enumerated rather
than their subjects** (canon-kit/SPEC.md §The causal-completeness check, point 5).
The narrowing is the deletion of one file from the tracked `*.sh` tree:

- `check-settings-paths` — reds on a literal repo-relative `.sh` grant that **does
  not resolve**, so its verdict is *not* monotone under this narrowing: removing the
  file **adds two** violations. Cleared by delta (12), in the same commit, not by
  inspection. Its clean line also reports a **checked count**, which drops by two;
  that count is pinned by the fixture pair against the fixture dir and by nothing
  against this tree, so no expectation moves.
- `check-measured-claim` — `tree-shell-owed` moves by one. Whether a governed
  sentence pins it is checked by scanning the `measured:` markers rather than
  assumed; the derived consumers are the baked hook invocations, cleared by delta
  (13).
- `check-graph` / `check-docs-mirror-fresh` / `check-gate-binary-fresh` — red on a
  stale hook, artifact, mirror or binary, and non-monotone for the same baked-value
  reason. Cleared by delta (13).
- `check-gate-fixture-coverage` — named because a reader assumes a deleted tool
  moves it, and it does not: no `.gate` descriptor and no fixture pair is owed or
  removed here, the member never having been a gate and guard-kit registering none.
- `check-template-copy-parity` — named because delta (12) edits a **template**, and
  the natural assumption is that a spliced copy must move with it. Probed: this tree
  routes to `close-triage.md` by path rather than splicing it, so the gate has no
  second side to hold and stays green in both directions.
- `check-exec-bit` — its corpus is the whole `*/bin/*.sh` set; removing one file can
  only remove findings. Monotone, cleared by inspection.
- `check-shellcheck`, `check-comment-tier`, `check-path-dialect` — monotone in the
  scanned `.sh` set for the same reason. Cleared by inspection.
- `check-crate-arms` — gains, rather than loses: two new modules and their
  `#[cfg(test)]` tests join the lint and test arms it runs.

**Cross-component signal: this amendment's component set is three** — guard-kit
(§scan-prompts, §The guard framework, §The friction loop, §Testing, §Layout and
configuration), drift-kit (§Bundled KPIs' `kpi-prompt-friction`) and gate-sdk
(§The non-gate arm's class roster and §The bin/-tool contract's crossing clause) —
so `check-stage-entry` assertion C fires and the **align stamp is demanded at the
build stage's entry**. Stated here so the build session is not the one that learns
it, and it is the reason this session recommends the audit stage next.

## Existing sections updated

- `guard-kit/SPEC.md §scan-prompts` — the tool is restated as its arm: the
  invocation form, the three declared knobs, the surviving argv-shape refusal and
  the `--` escape, the retired help arm, `--count`'s surviving spelling and its new
  caller set, and the log positional's unchanged override in both orders (deltas 2,
  9, 10). **The residue is stated in this section rather than implied**: the section
  names that its behaviour is composed from three `lib/guard.sh` primitives, that
  the library is permanently shell on two grounds, and that the section's contract
  will therefore not be wholly in-crate while that disposition stands (delta 1). The
  paragraph recording each keying step with its pre-change reading gains the
  sentence that this cut produces **no** step and why (delta 6). The
  silent-wrong-number paragraph at `:1379-1383` gains the second door this cut
  closed, since it is the same defect it already names (delta 9).
- `guard-kit/SPEC.md §The guard framework` — `guard_split_compound`'s roster entry
  loses the claim that the per-segment surface "cannot drift between them" and gains
  what replaced it: two implementations, a permanent shell holder, and a comparator
  that notices divergence. `guard_skeleton`'s entry gains the reachable-subset
  statement and the newline-free property that grounds it. `_guard_redirect_pairs`'
  entry gains its twin. All three cite criterion 6's *unless* clause and the parity
  harness by name (deltas 3, 4, 5).
- `guard-kit/SPEC.md §The friction loop` — step 2 names the arm rather than the
  script (delta 12).
- `guard-kit/SPEC.md §Testing` — the roster gains
  `gate-tests/guard-lib-parity.test.sh` with one line on what it compares and why it
  does not retire; the structural-limit sentence at `:1748` names the arm (deltas 4,
  11).
- `guard-kit/SPEC.md §Layout and configuration` — `bin/scan-prompts.sh` leaves the
  tree block, `gate-tests/guard-lib-parity.test.sh` joins it, and the knob roster's
  three entries gain no change beyond the reader they now also serve (deltas 2, 4).
- `guard-kit/README.md` — the two-line invocation roster becomes the arm's, and the
  arm's usage lands here, since §The non-gate arm puts a bridged arm's
  discoverability in the front-end's help and the owning kit's README (deltas 2, 12).
- `guard-kit/templates/close-triage.md` — step 1's invocation (delta 12).
- `drift-kit/SPEC.md §Bundled KPIs`, `kpi-prompt-friction` — the row stops naming a
  sibling kit's script and a stdout contract: the measurement is an in-crate call,
  two degrades retire, the surviving `guard-kit absent` degrade names its new
  witness, and the `^[0-9]+/[0-9]+$` assertion goes with the string it was about.
  The pointer to guard-kit/SPEC.md §scan-prompts as the key's owner stays (deltas 6,
  7).
- `gate-sdk/SPEC.md §The non-gate arm` — the class roster gains
  `--emit-scan-prompts` and `--guard-lib-parity`. The first is recorded as the
  class's first member whose *whole* configuration is a permanently-shell library's;
  the second joins the parity family beside `--toolfloor-parity` with the note that
  its holder cannot empty, which is the property `--declaration-parity` lacked
  (deltas 2, 4).
- `gate-sdk/SPEC.md §The bin/-tool contract` — the crossing clause gains its first
  **reader** instance and the ground that carries it there: a reader's silent-shape
  defect writes a wrong number into a trend, which is at least as bad as a capture's
  bad line, so the clause does not turn on capture (delta 9).
- `gate-sdk/SPEC.md §The port-candidate criteria`, criterion 6 — the *unless*
  clause's worked-instance roster gains guard-kit's three primitives, the fifth live
  instance and the first where **both** of the holder's `no-port` grounds are about
  something other than the twinned predicates (delta 3).
- `TASK-QUEUE.md`, the `native-gate-port-remaining-corpus` entry — promoted with
  `[design-pending]` swapped for this amendment's `[spec:]` ref; it **demotes** at
  build and never reaches `## Done`, which its own body already rules. At zero
  headroom the swap is line-neutral and the demoting commit may add no line
  (delta 15).
- The generated projections this cut stales — the on-site SPEC and README mirrors,
  the generated `pre-commit`/`commit-msg` hooks, `docs/check-graph.html`, and the
  gate binary itself. All are rostered with their triggers and regen commands in
  `docs/site-architecture.md` §Generated projections (all deltas).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named reader
      at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls guard-kit/SPEC-*.md`), the none-remain half discharged at the
      iteration rather than at the commit.
- [ ] **Removals propagated** — grepped every spec, skill, template, README, smoke
      script, gate-test, settings file and committed workflow surface for the
      deleted path; nothing dangles.
- [ ] **The residue is written, not implied** — §scan-prompts names the three
      primitives it is composed from and the permanence of the `lib/guard.sh` half;
      a reader who never saw this amendment can tell the section is not discharged.
- [ ] **The duplication is machine-held, not merely declared** — the parity harness
      exists, runs under `run-gate-tests.sh`, and **fails** when one side is edited
      alone; proved by a negative control in the landing session, not asserted.
- [ ] **The omitted skeleton branch is proved unreachable, not assumed** — the
      newline-free property of the log is asserted at the twin's entry point, and
      the parity corpus's scope is recorded beside it.
- [ ] **The behavioural oracle survives the port** — every scenario
      `scan-prompts.test.sh` asserts today still has a holder, plus delta (9)'s
      firing and non-firing pair.
- [ ] **The KPI reads the same number across the substrate change** — the full and
      `--trend` rows compared against a capture taken before the delete, on the same
      log; a step here would be a port defect, not a definitional one.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt
      tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **The oracle moved, and by the roster rather than by a number** — the `--tree`
      arm lists no `guard-kit/bin/scan-prompts.sh` row, taken as a per-file roster
      diff and not as a trailer delta.
- [ ] **The regeneration fan-out is discharged in the landing commit** — the
      generated hooks, the graph artifact, the SPEC and README mirrors, and the gate
      binary.
- [ ] **The demotion fits** — the entry returns to Deferred at build with no line
      added, against a measured zero headroom.
- [ ] **The seam held** — no allowlist string, no project toolchain word and no
      dated ruling of this project's crossed into the crate or into the merged
      section; the three harness/substrate literals travel on the ruling delta (16)
      cites and mint no knob.
