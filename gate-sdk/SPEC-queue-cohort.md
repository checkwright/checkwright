# SPEC amendment: queue-cohort

The first **real** gate cohort of the native port: **queue-kit, taken as a whole
kit** — all ten of its registered gates — ruled by the operator 2026-08-11 under
TRAJECTORY.md §PRIORITY DIRECTIVE. This amendment designs that port. It does not
restate the porting procedure (§Porting a gate to the binary substrate), the
conservation contract (§Meta-gate conservation for the binary substrate), or the
directive's grounds.

## The cohort, and why by-kit does not contradict the ordering rule

§The first cohort, and the rule that selects the next sets the ordering axis as
*"the largest set of criteria-clearing gates sharing one corpus derivation"* and
warns in the same paragraph that *"selecting by kit, by profile, or by whatever
is easiest next all re-import work this cohort only paid once."* The operator's
ruling selects by kit. **The two select the same set here**, and recording why is
what keeps the ordering rule usable rather than quietly overridden: queue-kit's
gates are the kit whose corpus derivation *is* the queue file.

Read off the ten `# graph:` manifests: **eight couple `TASK-QUEUE.md` alone**;
`check-roadmap-fresh` couples `TASK-QUEUE.md,ROADMAP.md,scripts/queue-config.sh`
and carries an explicit `trigger=` naming the same three; `check-queue-slug-liveness`
couples `TASK-QUEUE.md,scripts/*.sh`, where the glob is a **reverse trigger** —
the conservation table already records that shape for this family (*"names
`scripts/*.sh` in `couples=` only so that a script change re-runs it"*), and
`check-queue-slug-liveness` is named in that very row. All ten are
`dir=one valve=none tier=precommit`, so **criterion 3 is clear across the
cohort** and a green `check-graph` after the port is end-to-end proof the
manifest survived the substrate change.

**Criterion 7 is clear for nine of the ten, and the tenth is sequenced later for
it.** This paragraph previously read that *"across all ten gates the only
external program invoked is `git`"* and cited that as a real reason to take this
cohort. **That claim was false, and it was load-bearing.**
Probed at build: `check-roadmap-fresh` invokes **`bash` on a kit shell script** —
`GEN="$KIT/bin/roadmap.sh"`, `emitted="$(bash "$GEN" --emit)"`
(queue-kit/checks/check-roadmap-fresh.sh:97-99). `bin/roadmap.sh` is one of the
five `bin/` consumers of `lib/queue.sh` that this port does not touch (delta 2),
so nothing in this cohort ports the emitter the gate shells out to.

**Its fixture pair cannot catch this**, which is why the claim survived to build:
`good/args` and `bad/args` both pass `projection.txt emit.txt`, and the args
file's own comment records the intent — *"Two args steer assertion A off the
live emitter onto pre-baked fixture files."* A ported member's pair would
therefore go **green over an arm with no implementation**, the same vacuity
shape delta (7) exists to close, arriving through a different door.

**Operator-ruled 2026-08-11: `check-roadmap-fresh` is held for a later cohort.**
The whole-kit ruling was made on the premise that all ten members cleared the
criteria; that premise is false for exactly this one.

**The ground is sequencing, not exclusion — recorded precisely because an earlier
draft of this paragraph got it wrong.** That draft said criteria 6 and 7 are "the
governed selection rule" and that the member "fails selection". §The
port-candidate criteria denies exactly that in its opening sentence, and
criterion 7's own worked example — `check-action-run-shell` and `shellcheck` —
names such a member *the largest piece of port work rather than a permitted
exclusion*. A criterion a member fails **orders the work**; the 2026-08-09
directive ports the whole corpus **over time**, so holding a member for a later
cohort is the sequencing those criteria prescribe. The outcome the operator ruled
is unchanged and correct; only the ground it was recorded under was wrong, and a
later reader must not cite this hold as precedent for an eligibility screen.

So: the member is **held with its port work named and owed**, its `.sh` stays
unmodified, and the design it owes — shell-out to `bin/roadmap.sh`, a Rust
reimplementation of the emission format, or collapsing `bin/roadmap.sh` itself
onto the binary — is filed for the cohort that takes it rather than answered
here.

What remains true, and is still a real reason to take this cohort: across the
other nine the only external program is **`git`** — `git rev-parse` and
`git show` in `check-task-conservation` — the one sanctioned exception,
*"because it is the floor"*. No `jq`, no `ruby`, no `shellcheck` anywhere in the
kit.

**A second member is held on the same reading, for a different mechanism.**
`check-queue-prose-precondition` does not merely *transport*
`QUEUE_KIT_PRECONDITION_REGEX` across the bridge — it **interprets** it
(`check-queue-prose-precondition.sh:21,27`, `b ~ trig`), and the knob is consumer
config carrying an arbitrary POSIX ERE. Its awk also runs two `gsub`es with
alternation, groups and negated classes. The crate **vendors nothing** — that is
asserted, not assumed, by `walk.rs`'s
`the_crate_vendors_no_walker_because_it_vendors_nothing` — so porting it means
hand-writing an ERE engine plus awk `gsub` semantics. Sizing a subset to this
repo's one regex is foreclosed by the glob commitment §The port-candidate
criteria already makes for exactly this shape: the config surface permits what
this consumer happens not to write, and a narrow reader would silently mis-scan
the first consumer who writes it. **Operator-ruled 2026-08-11**, on the
sequencing ground above rather than an eligibility one: held for a later cohort,
`.sh` unmodified, with the ERE engine named and owed as a prerequisite that
cohort inherits — the same shape criterion 7 gives `shellcheck`.

The corrected shape is **eight ported of ten**.

The kit boundary and the corpus boundary therefore coincide, so the by-kit ruling
buys the by-corpus economy rather than trading it away. A later selector must not
read this as licence to take a kit whose gates share nothing.

## The two independent nine-of-ten splits

The figure carried into this iteration — **9 of queue-kit's 10 gates are
walk-free, only `check-queue-slug-liveness` walks** — is correct and is the
*walk* axis. Probed first-hand at HEAD, there is a **second, orthogonal
nine-of-ten**, and it lands on a **different member**:

| axis | the nine | the one |
|---|---|---|
| corpus shape (criterion 6) | walk-free, fixed-path reads | `check-queue-slug-liveness` walks `QUEUE_KIT_PROSE_SURFACE_GLOBS` (`native/src/gates/queue_slug_liveness.rs` since the port; the shell original read it at `check-queue-slug-liveness.sh:21`) |
| fixture pair (criterion 2) | `good/`+`bad/` under `queue-kit/gate-tests/` | `check-task-conservation` — the kit's sole `# no-fixture:` member (queue-kit/checks/check-task-conservation.sh:4) |

Evidence: `find queue-kit/gate-tests -maxdepth 2 -type d -name good -o -name bad`
returns pairs for exactly nine names, and `grep -rn '^# no-fixture:'
queue-kit/checks/` returns exactly one line. All ten are registered in
`scripts/gates.list` (lines 55-64), so criterion 1 is clear across the cohort.

**The consequence is the cohort's real shape: six cheap, two designed, two
held.** Six members clear every criterion the existing substrate answers and
port on the bridge alone — **once delta (12) repairs the harness the bridge is
proved through**. Two carry a named engineering problem this cohort answers:
`check-queue-slug-liveness` needs delta (5)'s basename-glob matcher, and
`check-task-conservation` needs delta (6)'s constructed parity scenario and delta
(7)'s `check-gate-output` repair. Two more — `check-roadmap-fresh` and
`check-queue-prose-precondition` — carry one each that it does not, and are held
for a later cohort by the 2026-08-11 operator rulings above, each with its port
work named and owed. All four problems are *different*, and none is discharged
by another's work. Reading "9 of 10" on one axis as the cohort's difficulty
is the mis-sizing this section exists to prevent — and both of this section's
nine-of-ten splits were drawn before the criterion-7 and ERE probes, so **neither
axis predicted either member that was actually held**. A third and a fourth axis
existed and nobody had looked down them; that, rather than the arithmetic, is
this section's transferable lesson.

## What changes

### The harness prerequisite

**(12) The fixture runner must resolve a bridged member's knob values inside the
case dir.** [design-bearing] Found at build, and it **blocks every member of the
cohort**, so it lands before any of them. It is a prerequisite of exactly delta
(7)'s shape — a meta-gate defect invisible from the cohort's own gates — and is
recorded here for the same reason.

`run-gate-tests.sh` computed `gate_command` **before** the `cd` into the case
dir, so `_gate_knob_value` sourced the owning kit's library with cwd = the
*invoker's* root and `queue-kit/lib/queue.sh:13` resolved
`scripts/queue-config.sh` against it. A shell gate, invoked after the `cd`, reads
the **case's own** config. The two substrates therefore read different config for
the same case — not one oracle over two substrates, but two oracles.

**Measured at HEAD before the repair**, bridge value vs. what the shell gate saw:

| knob | bridge (invoker's root) | shell gate (case dir) |
|---|---|---|
| `QUEUE_KIT_REQUIRED_SECTIONS` | 7, incl. `Icebox` | 6 |
| `QUEUE_KIT_HORIZONS` | `now next later` | `soon someday` |
| `QUEUE_KIT_TRACKS` | `adoption reliability ecosystem commercial` | `alpha beta` |
| `QUEUE_KIT_ICEBOX_SECTION` | `Icebox` | *(unset)* |
| `QUEUE_KIT_LESSON_TAGS` | `essay` | *(unset)* |

Two members failed their **good** case outright: `check-queue-sections` (the
injected 7-section set reports `missing: ## Icebox` against a fixture carrying
exactly the six defaults) and `check-roadmap-fresh` (the injected horizons report
`unknown horizon 'soon'` against a fixture tagged `[roadmap: soon/alpha]`).

**Why this is incompleteness rather than a contradiction to escalate.**
§run-gate-tests already draws this exact line for the *executable* — absolutized
at the invoker's root precisely because the `cd` would otherwise resolve it
against the case dir — and already names *"silently inherits this repo's consumer
config"* as the failure mode its hermeticity clause exists to prevent. The repair
extends a distinction the section already makes to the one thing it never
contemplated: a bridged member inside a case dir. Rejected alternatives: stripping
the per-case configs contradicts the hermeticity clause outright, and having the
crate read the config file itself re-imports the duplication criterion 6 forbids.

**The no-op for members declaring no knob is demonstrated, not asserted.** The
per-kit fixture battery (README.md §This repo, governed), all eleven rows, before
and after: **101 pairs / 65 unit tests, every row `rc=0`, output byte-identical**.
The first cohort's own knob is checked at the value level rather than only the
count level, since identical counts could mask a moved value:
`GATE_PRUNE_DIRS` resolves to `target|.git|node_modules|.tmp|gate-tests|worktrees`
from the invoker's root and from a case dir alike.

### The cheap six

**(1) Port the six criteria-clearing members.** [design-bearing]
`check-queue-entry-budget`, `check-queue-hygiene`,
`check-queue-sections`, `check-queue-wrap`,
`check-tag-lead-line`, `check-task-names` — each becomes a module under
`native/src/gates/` and a 4-tuple in `REGISTRY`, its `.sh` deleted and its
`.gate` descriptor landed in one motion. Design-bearing rather than mechanical:
each rule is re-implemented, and byte-identical parity over its fixture pair is
the acceptance oracle, not a formality.

**(2) The shared queue library ports once — and it carries more than helpers.**
[design-bearing] `queue-kit/lib/queue.sh` is 193 lines and the cohort's members
source it. Three distinct things live in it, and only the first is what "shared
library" suggests:

- **Four helpers** — `queue_alt` (:68), `queue_live_slugs` (:91),
  `queue_done_slugs` (:102), `queue_roadmap_entries` (:116). Re-implemented
  **once**, as a Rust module the cohort's gates share — the "ported once and
  proved N times" economy §The first cohort names. Porting them per-gate is the
  failure mode to refuse.

  **Only two of the four are actually in scope**, and holding
  `check-roadmap-fresh` on shell is what settled the fourth. `queue_alt` is
  lib-internal (it composes the derived regexes and no gate calls it), so it ports
  as an implementation detail of the globals rather than as a helper.
  `queue_roadmap_entries` has exactly two consumers — `bin/roadmap.sh` and
  `check-roadmap-fresh` — and **both stay shell**, so it is not ported at all.
  That leaves `queue_live_slugs` (`check-queue-slug-liveness`,
  `check-task-conservation`) and `queue_done_slugs` (`check-task-conservation`
  alone, so it arrives with that member rather than with this batch).
- **Eight derived globals** — `QUEUE_ACTIVE_RE` (:71), `QUEUE_DEFERRED_RE`
  (:73), `QUEUE_ICEBOX_RE` (:76), `QUEUE_DONE_RE` (:78), `QUEUE_TASK_SECTIONS`
  (:80), `QUEUE_TASK_RE` (:84), `QUEUE_SECTION_RE` (:86), `QUEUE_LESSONS_RE`
  (:89) — computed from the knobs at source time and consumed **directly** by the
  gates' awk programs. These are **not knobs**, so the bridge does not carry
  them; the Rust side derives them from the bridged knob values by the same
  rules. Naming them matters because a port that bridges only the knobs and then
  looks for the regexes will not find them declared anywhere — they are the
  cohort's largest piece of shared surface after the helpers, and the piece most
  easily missed by a per-gate reading.

  `QUEUE_LESSONS_RE` is worth one line of its own: it hard-codes the literal
  `^## Lessons Learned`, where its siblings derive from a knob. The port
  reproduces it as-is; noticing the asymmetry and "fixing" it into a knob would
  be a behavior change smuggled in under a substrate change.
- **A fail-closed config validation** (:164-192) that `exit 2`s with *"queue-kit:
  malformed queue config — the gates cannot run"*, covering emptiness, integer
  shape, an icebox/deferred name collision, and a **cross-knob** rule that
  `QUEUE_KIT_HORIZONS` and `QUEUE_KIT_TRACKS` are configured as a pair.

**The validation survives the port at no cost, and the reason is worth
recording.** `_gate_knob_value` resolves each declared knob by **sourcing the
owning kit's `lib/*.sh` in a subshell** (gate-sdk/lib/gate.sh:104-113), so
`queue.sh` is sourced at dispatch and its validation block runs there. Its
`exit 2` propagates out of the subshell, through `_gate_knob_value`'s non-zero
return, into `gate_command`'s `|| exit 2` — the §Fail-closed contract, preserved
without re-implementing a line of it. This is criterion 6's *"the duplication is
not machine-held, it is absent"* argument extending from knob **defaults** to
knob **validation**, which is what makes a thirteen-knob cohort tractable at all.

**Its honest limit, stated rather than discovered.** That only holds for a member
declaring **at least one** knob; a member declaring none never causes the library
to be sourced and would silently skip validation. Every one of the ten reads at
least `QUEUE_KIT_QUEUE_FILE`, so the limit is not live in this cohort — but it is
a property of the bridge rather than of this cohort, and the next port meets it
again.

**`lib/queue.sh` is not retired, and one of its four helpers keeps two live
implementations after this port.** [design-bearing] The shell library is not
exclusive to the cohort: `bin/queue-index.sh`, `bin/queue-counts.sh`,
`bin/queue-edges.sh`, `bin/roadmap.sh` and `bin/lesson-sink.sh` all `source` it
(queue-kit/SPEC.md §Per-component contracts documents each as a consumer), and
none of the five is a `gates.list` member, so none is in this cohort and none
moves to Rust. `queue-kit/lib/queue.sh` therefore stays on disk, unmodified, as
their library — this delta adds a Rust module beside it, it does not replace it.

One helper is consequently **dual-consumed**: `queue_live_slugs` (shared today by
`bin/queue-edges.sh:25`, `check-queue-slug-liveness` and
`check-task-conservation`, per queue-kit/SPEC.md §lib/queue.sh — *"[bin/queue-
edges.sh] sources this library for the section regexes and `queue_live_slugs` it
does share"*). Once `check-queue-slug-liveness` and `check-task-conservation`
port, the section's stated guarantee is what stops being true for the ported
half: the gate calls the Rust reimplementation, the `bin/` script keeps calling
the shell original, and the two are edited independently from here on.

**`queue_roadmap_entries` was the second instance and no longer is.** It is
shared by `bin/roadmap.sh:32` and `check-roadmap-fresh` under that section's
guarantee that *"the emitter and the gate can never disagree about what an entry
claims"* — and holding `check-roadmap-fresh` on shell (criterion 7, above) leaves
both consumers on the one adapter. The guarantee survives this port untouched,
which is a real reduction in the debt delta (2) files rather than a deferral of
it. The eight derived
globals (:71-89) are the same shape at a wider fan-out — every one is also read
directly by at least one `bin/` script's `awk` invocation
(`queue-counts.sh:25`, `queue-edges.sh:32`, `queue-index.sh:53,93-96`), so the
Rust side's re-derivation duplicates section-matching rules a `bin/` script still
runs in shell.

**This is criterion 6's own qualification, not a new rule.** *"Unless the
duplication the port creates is machine-held"* (§The port-candidate criteria) is
written for exactly this shape, and unlike a bridged knob — discharged by
construction because the binary holds no default to drift — a re-derived regex
or a re-implemented helper is logic, not a value, and nothing this delta builds
makes the two sides' agreement machine-held. Delta (9)'s one-time parity proof
(both implementations compared on the fixture pair, the live tree and an edge
tree) closes the gap **at port time**; it does not survive the next edit to
either side, because the shell original is not deleted for these two helpers and
the eight globals — it keeps running, for a caller outside this cohort.
**Not closed here**, and the reason is scope rather than oversight: an ongoing
cross-implementation check is new scope this cohort does not need in order to
clear criterion 6 for the *gates it ports* — their own parity is proved and
their `.sh` is deleted, which is what the criterion actually asks for a ported
member. Filed as debt rather than built (`.workflow/gap-inbox.md`), because the
risk it names is real and outlives this session without being this cohort's to
close.

**(3) Knob bridge, no new substrate — and the count is larger than carried.**
[design-bearing] Probed at HEAD (`grep -ohE 'QUEUE_KIT_[A-Z_]+' queue-kit/checks/*.sh`),
the cohort's gates name **thirteen** knobs, not the eight the scope survey lists:

- **Seven scalars** — `QUEUE_KIT_QUEUE_FILE`, `QUEUE_KIT_DEFERRED_SECTION`,
  `QUEUE_KIT_WRAP_BUDGET`, `QUEUE_KIT_ENTRY_LINE_CAP`,
  `QUEUE_KIT_PRECONDITION_REGEX`, `QUEUE_KIT_ROADMAP_FILE`,
  `QUEUE_KIT_ROADMAP_MARKER`.
- **Six arrays** — `QUEUE_KIT_REQUIRED_SECTIONS`, `QUEUE_KIT_PROSE_LEADS`,
  `QUEUE_KIT_PROSE_SURFACE_GLOBS`, `QUEUE_KIT_LESSON_TAGS`,
  `QUEUE_KIT_HORIZONS`, `QUEUE_KIT_TRACKS`.

The survey omits `ROADMAP_FILE`, `ROADMAP_MARKER`, `PROSE_SURFACE_GLOBS`,
`HORIZONS` and `TRACKS` — four of the five belong to `check-roadmap-fresh`,
which the survey reached late as one of the two members it added to the census's
original seven. Sourcing the library reaches five more
(`QUEUE_KIT_ACTIVE_SECTIONS`, `QUEUE_KIT_DONE_SECTION`,
`QUEUE_KIT_ICEBOX_SECTION`, `QUEUE_KIT_ICEBOX_AGE_DAYS`,
`QUEUE_KIT_ATTEND_CAP`), which the validation above reads whether or not a member
declares them.

Every one of the thirteen is carried by the existing config bridge — the
substrate that landed last iteration and ported zero gates by ruling, and
discharging criterion 6 for these arrays is exactly what it was built for. Each
member declares its knob reads in its registry tuple's fourth element, so a knob
the bridge was never asked to carry cannot be read.

**One knob shape the bridge cannot carry, found here and clear here.**
`QUEUE_KIT_LESSON_SINKS` is a `declare -A` associative array (queue-kit/lib/queue.sh:56).
The bridge's wire format is **tab-joined elements with no key channel**
(§lib/gate.sh), so an associative array has no serialization in it. **No cohort
member reads it** — it is `bin/lesson-sink.sh`'s — so this cohort is unaffected.
It is recorded because it is a property of the bridge that the next kit's port
will meet, and meeting it at implementation time rather than at design time is
the expensive order.

**(4) `gates.list` is unchanged.** [mechanical] A port swaps a member's
declaration spelling, never its name, so the registry is untouched — which is
what makes the `check-graph` proof above a proof of the *manifest* rather than of
a rewritten registration.

### `check-queue-slug-liveness` — the walker

**(5) Basename-glob list matching over a bridged glob array.** [design-bearing]
The member walks `QUEUE_KIT_PROSE_SURFACE_GLOBS` (this repo's config sets
`docs/*.md` and `*.local.md`). `walk.rs` today filters by extension; this needs
list matching of a **basename glob**. Two commitments are inherited rather than
re-decided: the matcher is **`**`-capable (globstar semantics)**, already
committed under criterion 6 against the `shopt -s globstar` the shell side
enables; and the bridge transports the glob **strings** and interprets nothing,
so the matcher lives on the reader's side.

This is one of the two shared mechanisms §The first cohort records **Cohort A**
(the canon-kit `spec_manifest_files` callers) as still owing. Building it here
means Cohort A's remaining debt after this iteration is the other one alone — a
Rust `gate_kit_roots` for kit-root pruning. Recorded so the next selector reads
Cohort A's cost correctly rather than re-deriving it.

The member's walk root is declared in its registry tuple's third element and held
to executed behavior by crate unit test A. The declaration is a concrete root or
`?`, never a guess.

### `check-task-conservation` — the fixture-less member

**(6) Parity is proved by a constructed scenario, not a fixture pair.**
[design-bearing] Its `# no-fixture:` reason is structural and stays true after the
port: the rule diffs HEAD against the worktree, and a committed fixture has
HEAD == worktree, so the loss case has no static representation. Criterion 2's
demand — *"parity between substrates is proved by running both against the same
cases, never asserted"* — is therefore met by a **scratch-clone scenario**: a
throwaway repo in which an entry is deleted in the worktree without committing,
both implementations run against it, and their bytes compared. That still
satisfies the criterion's actual requirement (same cases, both substrates, while
both implementations exist) without pretending a fixture pair is available.

**(7) The conservation table's `check-gate-output` row must change, and it is a
prerequisite rather than a follow-up.** [design-bearing] This is the port's
sharpest consequence and it is invisible from the cohort's own gates.

`check-gate-output` resolves each `gates.list` member's declaration path and
branches on `# no-fixture:` (gate-sdk/checks/check-gate-output.sh:37): a fixtured
member is asserted on real output by the runner, and a `# no-fixture:` member is
**source-grepped** for its `: clean` and `help:` lines (:41, :44). The
conservation table records this precisely — *"source-grep retained for the one
member outside it… The remaining member, `check-task-conservation`… has no case
for a runtime assertion to reach, so the source-grep stays its only oracle"*
(§Meta-gate conservation for the binary substrate). **That one member is a cohort
member.** After the port its resolved declaration path is the `.gate` descriptor,
and a descriptor carries the `# graph:` manifest, its `# spec:` pointer and an
optional `# no-fixture:` opt-out — *"Nothing else"* (§The `.gate` descriptor). So:

- Keep `# no-fixture:` on the descriptor → the source-grep arm runs against a
  file that **cannot** contain an `echo`/`printf` line → `check-gate-output` goes
  **red**.
- Drop `# no-fixture:` → the member falls to the runtime arm, where **no fixture
  case exists for the runner to assert over** → its output-contract coverage
  silently becomes **zero**. That is the exact vacuity the conservation table
  exists to close, arriving through the table's own remaining exception.

Both branches are wrong, so the port owes a design. The repair: for a
`.gate`-dispatched `# no-fixture:` member, `check-gate-output`'s source-grep
corpus follows the rule to where it now lives — the **implementation module** —
resolved by the crate's existing name convention, which is specified once rather
than left as a second registry that could drift. This is the same "corpus
extended to the Rust module" shape the table already uses for
`check-comment-tier`, `check-gate-assertions` and `check-todo-task-liveness`; what
is new is that this reader needs the mapping **per member** rather than a widened
glob, and that is the piece to specify.

Sequencing follows: `check-task-conservation` ports **last within the cohort**,
after delta (7) lands, on the same principle §The first cohort applies to
criterion-4 and criterion-7 gates — *"Neither is ported and patched later; both
are designed, then ported."*

### Conservation and parity obligations across the cohort

**(8) Disposition coverage is re-derived, not assumed.** [design-bearing]
`check-gate-substrate-parity` assertion C derives the substrate-sensitive set at
runtime — a member whose expanded `couples=` covers a registry member's
declaration path — and reds any member without a disposition line. Eight new
descriptors change which members that derivation selects, so the table is
re-checked against the derivation after the port rather than assumed complete.
**The re-derivation is not deferrable to the cohort's end**: the selection
changes at the *first* commit that lands descriptors, so each batch re-checks the
table for the members it lands. Only the `check-gate-output` row (delta 7) waits
on `check-task-conservation`.
Two rows are known to be in scope already: `check-gate-output` by delta (7), and
`check-queue-slug-liveness`, which is itself a table row (*"Survive unchanged —
reverse triggers"*) **and** a cohort member — its row's reasoning stands, since
what it scans is the governed-doc set rather than a gate's content, but the row
now describes a `.gate`-declared gate and must read correctly as one.

**(9) Parity is proved while both implementations exist.** [mechanical] Assertion
A forbids a `<name>.sh` and a `<name>.gate` in one resolve dir, so the comparison
runs with the shell gate still in place and the descriptor staged elsewhere; the
descriptor lands and the script is deleted in one motion. Per member: the fixture
pair (or delta (6)'s scenario), the live tree, and an edge tree.

**(10) The commit-time obligations this repo already carries.** [mechanical]
`bash gate-sdk/bin/build-native.sh` before each commit, and the generated
projections regenerated per docs/site-architecture.md §Generated projections. Eight
descriptors and eight deleted scripts move the footprint and value rollup, the
graph artifact, the enforcement map and the docs mirror.

**(11) Two deferred entries become closure candidates as this lands.**
[mechanical] `gate-battery-parallel-execution` and `gate-battery-result-cache`
each say in their own text that the port subsumes them. They are dispositioned at
close, not built.

## Producers and consumers

**New interface: eight `.gate` descriptors under `queue-kit/checks/`.**
- *Producer* — this port; the descriptor's existence **is** the dispatch
  declaration, with no dispatch field and no mapping table (§The `.gate`
  descriptor).
- *Consumers* — `gate_resolve` (declaration path, for every manifest reader) and
  `gate_command` (invocation argv), gate-sdk/lib/gate.sh. Enabling config is
  already emitted everywhere it must be: `GATE_SDK_NATIVE_BIN` carries a kit
  default, so no consumer must set anything for dispatch to resolve.
- *Every field has a named reader* — `# graph:` by `check-graph`,
  `gen-pre-commit.sh`, `enforcement-map.sh` and `footprint.sh`; `# spec:` by
  `check-spec-pointer`; `# install:` by `check-install-disposition`;
  `# no-fixture:` (on `check-task-conservation`'s descriptor only) by
  `check-gate-fixture-coverage` **and**, per delta (7), by `check-gate-output`.
  No field is added to the closed roster.

**New interface: eight `REGISTRY` tuples in `native/src/gates/mod.rs`.**
- *Producer* — the crate, at compile time; a member added without its declared
  read roots and knob reads fails to compile.
- *Consumers* — `lookup` (dispatch), `roots` (`--reads`, consumed by
  `check-reads-couples`), `knobs` (`--knobs`, consumed by `gate_command`'s bridge
  loop at gate-sdk/lib/gate.sh:156-167), `names` (`--list`, consumed by
  `check-gate-substrate-parity` assertion B).
- *Field readers at named transitions* — the **knob-reads** element is read by
  `gate_command` at argv construction, where each name becomes one
  `GATE_SDK_KNOB_<NAME>=<tab-joined>` element; the **read-roots** element is read
  by `--reads` and held to behavior by unit test A at `cargo test`.

**New interface: the shared queue-library Rust module (delta 2).**
- *Producer* — the cohort's own port.
- *Consumer* — each cohort member that calls a queue helper; there is no
  external caller and no `--` arm exposing it, so it adds nothing to the binary's
  flag surface and cannot leak into `--list` as a phantom subcommand.

**New capability: basename-glob list matching in `walk.rs` (delta 5).**
- *Producer* — `check-queue-slug-liveness`'s corpus derivation, driven by the
  bridged `QUEUE_KIT_PROSE_SURFACE_GLOBS`.
- *Consumer* — the walk recorder, which must observe the roots this walk visits or
  unit test A's assertion over the member is vacuous. Unit test B's rule binds:
  the matching lives inside the crate's single sanctioned walk implementation, not
  in the gate module, or the walk becomes invisible to the recorder.

**Red conditions of the readers this change touches** (§The causal-completeness
check, point 5 — the port **narrows** a corpus by deleting eight `.sh` files, and
"a narrower corpus can only remove violations" is the first argument this delta
reaches for and is false):
- `check-gate-output` — red on a **zero count** of a matching `: clean` / `help:`
  line. Non-monotone: deleting `check-task-conservation.sh` *adds* a violation.
  This is delta (7), and it is the attested instance of exactly the shape point 5
  names.
- `check-gate-fixture-coverage` — red on a member with **neither** a pair **nor**
  a `# no-fixture:` opt-out: another zero-count reader, cleared only if the
  descriptor carries the opt-out forward.
- `check-gate-substrate-parity` assertion B — red on a descriptor with no
  subcommand **and** on a subcommand with no descriptor; both directions, so a
  half-landed member reds either way.
- `check-gate-substrate-parity` assertion C — red on a substrate-sensitive member
  with **no disposition line**: a zero-count reader over a set the port *widens*.
- `check-reads-couples` — red on a walk outside the declared couples, and its
  shell parser finds nothing in a binary gate; the `--reads` consumption is what
  keeps it from printing `clean` vacuously over the ported members.
- `check-readme-roster` — red in **both** directions, so queue-kit's README must
  move eight of its ten `.sh` names to `.gate` names rather than gaining them;
  `check-roadmap-fresh` and `check-queue-prose-precondition` keep their `.sh`
  spelling.
- `check-shellcheck`, `check-comment-tier`, `check-spec-pointer`,
  `check-todo-task-liveness`, `check-deprecation-task` — each loses eight shell
  files from its corpus and gains eight descriptors plus eight Rust modules; the
  comment-surface arms for `*.gate` and `*.rs` already exist, so these are
  monotone here and clearable by inspection.
- `check-gate-binary-fresh` — red on a stale binary once a member dispatches;
  already armed, and eight more dispatches do not change its predicate.
- `check-docs-cmd` — red on a doc fencing a path that no longer runs: real signal
  after eight `.sh` deletions, and the reason the docs mirror is in delta (10).

## Existing sections updated

- **gate-sdk/SPEC.md §The first cohort, and the rule that selects the next** —
  owned by delta (1), which executes the by-kit selection this section's
  reconciliation explains, and delta (5), which discharges one of Cohort A's two
  outstanding shared mechanisms. Records that the by-kit ruling and the
  shared-corpus ordering rule select the same set here, and why that is not a
  precedent for taking any kit, and updates the section's statement of Cohort A's
  outstanding shared mechanisms accordingly.
- **gate-sdk/SPEC.md §run-gate-tests** — owned by delta (12). The section's
  invoker's-root/case-dir split, drawn today for the dispatch executable alone,
  is extended to a bridged member's knob values, which resolve in the case dir;
  the hermeticity clause below it is what that extension serves rather than
  contradicts.
- **gate-sdk/SPEC.md §Meta-gate conservation for the binary substrate** — owned
  by deltas (7) and (8). The `check-gate-output` row is rewritten: its "one
  member outside the fixtured corpus" is a ported member, and the row states the
  implementation-module corpus rather than a source-grep over a descriptor. The
  `check-queue-slug-liveness` row is re-read as describing a `.gate`-declared
  gate.
- **gate-sdk/SPEC.md §The port-candidate criteria** — owned by delta (6).
  Criterion 2 gains the constructed-scenario form as the parity oracle
  for a `# no-fixture:` member, which criterion 2 as written does not contemplate.
- **gate-sdk/SPEC.md §check-gate-output** — owned by delta (7): the per-member
  corpus resolution for a `.gate`-dispatched `# no-fixture:` member.
- **queue-kit/SPEC.md** — owned by deltas (1), (5) and (6): every per-gate
  contract section whose prose says the gate is a shell script or cites its
  `.sh` path; and **§check-task-conservation**, which owns the `# no-fixture:`
  reasoning delta (6) extends to parity.
- **queue-kit/SPEC.md §lib/queue.sh** — owned by delta (2). Its "one-adapter,
  can't disagree" guarantee for `queue_live_slugs` (shared with
  `bin/queue-edges.sh`) no longer holds once `check-queue-slug-liveness` and
  `check-task-conservation` read the Rust reimplementation instead; the section
  records the split and points at the filed gap rather than asserting the old
  guarantee unchanged. **The guarantee for `queue_roadmap_entries` survives
  intact** — holding `check-roadmap-fresh` on shell leaves `bin/roadmap.sh` and
  that gate as its only two consumers, both still calling the one adapter, so the
  emitter and the gate still cannot disagree about what an entry claims.
- **queue-kit/README.md** — owned by deltas (1), (5) and (6): gate roster, eight
  of ten names `.sh` → `.gate`.
- **native/src/gates/mod.rs** — owned by deltas (1), (5) and (6): the registry,
  and the module-per-gate comment whose "one module per ported gate" claim now
  spans ten members.

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
