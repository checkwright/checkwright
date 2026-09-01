# SPEC amendment: index-cut

The port disposition of **context-kit's five `bin/`- and `lib/`-level members
behind §Index-first reading** — `bin/md-index.sh` (85 lines),
`bin/md-section.sh` (63), `bin/pub-index.sh` (79), `lib/pub-lang/rust.sh` (26)
and `lib/pub-lang/ts.sh` (32), 285 lines — onto the binary substrate as non-gate
arms. This is one of the iteration's two port cuts under the port-only run
(TRAJECTORY.md §PRIORITY DIRECTIVE), composed at scope and ruled option B by the
**lead on its own authority**, 2026-09-01, over the resume channel; it did not
reach the operator, and that is stated because a composition ruling recorded
without its authority reads at the post-port triage as more settled than it is.

**Measured at this HEAD rather than carried from the survey**: the port oracle's
`--tree` arm reads 122 files scanned, 64 declared `no-port`, 0 temporarily held,
**58 owed**. This cut takes five of that column. **No ported member is a gate** —
all five are advisory tooling that has never joined `gates.list` — so no gate
roster and no binary-less residual roster moves, which is the whole of what
criterion 5's per-cohort measurement asks of a cut with no gate in it.

## What changes

### (1) The five members are one cut, and the ground is their own headers

`native-gate-port-remaining-corpus`'s composer ruling of 2026-08-28 selects a cut
**by stated contract** — the owed files behind one specification section, ported
behind the one amendment that section needs — and refuses size- and kit-ordered
composers on the ground that a cut assembled for convenience *averages* grounds
its members do not share {design-bearing}. These five share one ground and it is
not their kit: **each declares `context-kit/SPEC.md §Index-first reading` in its
own `# spec:` header**, so membership is read off the files rather than inferred
from their directory. Header text, verbatim in part: `md-index.sh:2` "compact
heading hierarchy with first sentence per section"; `md-section.sh:2` "print one
Markdown section by heading"; `pub-index.sh:2` "compact public API surface, a
dispatcher over per-language extractors (ships rust, ts)";
`lib/pub-lang/rust.sh:2` "the Rust public-item extractor"; `lib/pub-lang/ts.sh:2`
"the TypeScript public-surface extractor".

**The census that found this group is carried rather than re-bought, and its
witness is inlined here rather than pointed at.** The record it sits in is
boundary-truncated scratch, so a pointer into it resolves to nothing one
iteration after this is written (lifecycle-kit/SPEC.md §The survey record).
Question: *which owed shell files group behind one stated-contract section, and
which of those groups is takeable now?* — corpus `tracked non-test *.sh (the
port-blockers --tree corpus) x every kit SPEC.md section`; oracle
`bash gate-sdk/bin/run-gates.sh --emit port-blockers --tree`; rev
`9845f2c350a93a29ed055138da5826537783a4e1`; edges
`native-gate-port-remaining-corpus 7`; finding: of 58 owed files, only five
groups of 2+ sit behind one stated-contract section, and this is the cleanest —
`pub-index.sh` is a dispatcher over a consumer-first registry, so the seam
survives and only the two bundled extractors move in-crate. The witness was
re-run at this stage's entry: the corpus diff since that rev is clean and the
oracle still reads 58 owed.

**What is not the ground.** That the five happen to be one kit's is how the
*candidate* was found; the cut is well-formed because one section rules all five
and one amendment is what that section needs, which would still hold if a sixth
member sat in another kit.

### (2) `--emit-md-index` — the markdown structural index, byte-preserved

`bin/md-index.sh` becomes an `Arm::Emit` member of `BRIDGED_ARMS` spelled
`--emit-md-index`, reachable through the shipped front-end as
`run-gates.sh --emit md-index [paths…]` with no front-end change
{design-bearing}. Its declared read is `CONTEXT_KIT_PRUNE_DIRS` and nothing
else. The family is **forced, not chosen**: the tool resolves a consumer knob,
and gate-sdk/SPEC.md §The non-gate arm rules that a configured tool ported as a
hardcoded top-level flag "resolves platform defaults and silently ignores every
consumer override — which is not a calibration between two workable shapes but
the difference between working and appearing to".

Four observable properties are the contract and each survives explicitly,
because each is a place a reimplementation would quietly differ and each is
pinned by an existing golden (delta 7):

- **Per-file block shape** — `<repo-relative path>  (<N>L)` where `N` is the
  `wc -l` newline count, then one indented row per heading
  `<indent><hashes> <heading>:<lineno>` with `  — <first sentence>` appended
  when one is found, then a blank line.
- **The first-sentence rule** — the first non-blank line after the heading that
  is not inside a fence and is not itself a heading or a `---` rule; markdown
  link syntax reduced to its text, `*`, `_` and backticks stripped, cut at the
  first `.`, `!` or `?`, else at 120 characters.
- **Traversal and order** — `*.md` under the given paths, `CONTEXT_KIT_PRUNE_DIRS`
  matched on the **leaf basename**, results in byte order of the absolute path.
- **The empty case** — `No Markdown files found in <targets>`, where the
  targets are the ones given and default to the repository toplevel, so the
  default form of the message carries an absolute path exactly as it does today.

The optional `[paths…]` positional ports unchanged: it is an **input-corpus
positional** selecting what the rule analyses rather than redirecting resolved
config, which §The non-gate arm's distinguishing test admits by name and which
`check-prose-tells`, `check-spec-embedded-source` and `check-template-copy-parity`
already carry through their own ports. Nothing is deleted, so no documented
sentence about an argument comes due.

### (3) `--emit-md-section` — one section by heading, and the matcher is new code

`bin/md-section.sh` becomes `--emit-md-section`, an `Arm::Emit` taking
`<file> <heading>` off the arm's own argv slice {design-bearing}. Its declared
knob roster is **empty**: the tool resolves no knob today and the port must not
mint a read it does not make. An empty roster is nonetheless a **bridged-arm
table row rather than a hardcoded flag**, because table membership is what makes
the arm reachable at all — `run-gates.sh --emit <name>` composes `--emit-<name>`
and `exec_arm` resolves it through `gate_knob_env`, which finds a member only in
that table. This is the first member whose row exists for reachability rather
than for configuration, and it is recorded here so the next reader does not read
the empty slice as an omission.

**The crate's `section.rs` is the walk and not the matcher, and conflating them
is the error this delta exists to close.** `section::sections` bounds a section
by heading level exactly as this tool does, and that half is reused. Its
*match*, `line.starts_with(name)`, is case-sensitive, `§`-blind and fence-blind,
because its five existing callers hand it a literal `## <name>`. This tool's
stated contract is none of those: the match is **case-insensitive**, tolerates a
**leading `§`** so a spec citation pastes directly, compares the heading's
**text** (hashes and surrounding whitespace stripped) rather than a line prefix,
and **skips fenced blocks** so a heading inside one is not mistaken for
structure. So the matcher is new code beside the existing walk, and `section.rs`
is not widened — widening it would move five other gates' verdicts to save one
function.

**One observable moves.** A query matching no heading exits **1** today, with
`md-section: no heading matched: <query>` on stderr and nothing on stdout; it
will exit **2**, because `EmitFn` returns a `Result` and the dispatcher maps
every error arm to 2. That is §The queue-index arm's `--extent` finding
inherited rather than retaken: no in-tree caller reads the 1, the golden runner
compares stdout on a matching query, and the two session-facing callers are a
brief line and an interactive invocation. The file-not-found and missing-argument
refusals are exit 2 today and are unchanged.

**The behavior `md-section-near-miss-match` describes is preserved deliberately.**
That icebox entry records "empty on a near miss; correct on an exact query" as a
defect of this tool's matching. The port carries the exact-heading rule across
unchanged, so the entry's premise stands and its verdict is untouched; fixing it
inside a port cut would be non-port design work the composer refuses.

### (4) `--emit-pub-index` — the dispatcher moves, the extractor seam does not

`bin/pub-index.sh` becomes `--emit-pub-index`, an `Arm::Emit` taking
`[paths…]` {design-bearing}. Its declared reads are `CONTEXT_KIT_PRUNE_DIRS`,
`CONTEXT_KIT_PUB_LANGS` and `CONTEXT_KIT_PUB_LANG_DIR`.

**This is composer ruling (1) worked: the cut narrows the port, never the
extension point.** Everything a consumer can plug into survives with its
resolution order, its execution and its env contract intact:

- **Resolution stays consumer-first.** For each enabled language, a file at
  `<CONTEXT_KIT_PUB_LANG_DIR>/<lang>.sh` is used if it exists; otherwise the
  crate's built-in extractor for that language is used; otherwise the arm
  refuses at exit 2 naming the language and the consumer directory. Same
  precedence as today, with the kit's `lib/pub-lang/` leg replaced by the
  built-in roster rather than removed — so a consumer's `rust.sh` still shadows
  the shipped Rust grammar.
- **A consumer extractor is still a sourced bash file defining exactly two
  names.** `PUB_LANG_GLOBS` and `pub_lang_extract <file>` are unchanged
  contract. The arm runs a consumer extractor through **two `bash` spawns per
  language**: one sources the file and prints `PUB_LANG_GLOBS`, one sources it
  and calls `pub_lang_extract` over the file list the arm walked. Two rather
  than one because the globs are needed *before* the walk, and one per contract
  name rather than a private protocol so the seam a consumer writes against does
  not change shape. `bash` joins the arm's spawned-program set, which §The
  non-gate arm records in prose because a `BRIDGED_ARMS` row carries no
  requirement element; `--emit-port-blockers` and `--lesson-sink` already spawn
  it, and it is on `GATE_SDK_PROGRAM_FLOOR`.
- **The dispatcher's own three jobs stay the dispatcher's** — traversal under
  the prune set, the kind-then-name sort (`LC_ALL=C`, key 1 then key 2), and the
  row formatting `  %-8s %s :%s` under a `<rel>  (<count>)` block header.

### (5) The two bundled extractors become in-crate implementations, not arms

`lib/pub-lang/rust.sh` and `lib/pub-lang/ts.sh` move **in-crate as built-in
extractors behind the dispatcher**, not as members of the bridged-arm table
{design-bearing}. They are not tools: they have no caller but the dispatcher and
no output contract but the unsorted `kind name lineno` rows it consumes, so
giving each a flag would mint two spellings with one caller between them and
would put a second entry point into the emission path — the thing §The non-gate
arm forbids.

Their grammars are re-expressed against the crate's POSIX ERE matcher and stay
**grep-grade**: the Rust extractor takes `pub` / `pub(...)` items of the nine
declared kinds; the TypeScript extractor takes `export`-declared
`function`/`class`/`interface`/`type`/`enum`/`const`/`let`/`var`, `const enum`
folded to `enum`, and `export default` named or falling back to the literal
`default`, over `*.ts` and `*.tsx`. Re-exports and multi-line declarations
remain stated honest limits, not parsed — the port moves the wrapper, never the
rule. This is criterion 7's **incidental spelling** class: `grep -nE` piped into
`awk` is assembly the target language expresses directly, and the verdict is
identical either side of the substitution, which delta (7)'s goldens are what
prove rather than assert.

This delta discharges two of `kit-library-port-residue`'s six members. That
entry's own body already says these two "wait on the resolver that finds them,
`context-kit/bin/pub-index.sh`, which is itself owed" — delta (4) is that
resolver, so the wait ends here rather than by a separate ruling.

### (6) Two knob defaults move into `lib/context.sh`, and this is not tidying

`CONTEXT_KIT_PUB_LANG_DIR` and `CONTEXT_KIT_PUB_LANGS` are defaulted **inside
`bin/pub-index.sh` today** — the first as the inline fallback
`${CONTEXT_KIT_PUB_LANG_DIR:-${GATE_SDK_GATES_DIR:-scripts}/pub-lang}`
(`pub-index.sh:17`), the second as a run-time derivation over the shipped
`lib/pub-lang/` roster (`pub-index.sh:26-33`). Neither is defined in
`lib/context.sh`. Both move there in the commit that deletes the driver
{design-bearing}.

**The rule is gate-sdk's and it is load-bearing rather than tidy.** §The non-gate
arm: "A default the deleted shell driver held inline moves into the owning kit's
library in the same cut that deletes the driver, never after" — the bridge
resolves a declared knob by sourcing exactly one kit's library and **exits 2 on
a knob that library does not define**, so a default left beside the compiled
reader is sourced by nothing. Because delta (4) declares both knobs, leaving
either behind is not a silent degradation but a refusal of the whole arm.

- `CONTEXT_KIT_PUB_LANG_DIR` moves **verbatim** as a guarded assignment, so the
  documented default and the supplying site become the same string for the first
  time. §Layout and configuration already states this default in prose; today
  §lib/context.sh's claim to be "the one home of ... every knob default above"
  is false of this knob, and this delta makes it true.
- `CONTEXT_KIT_PUB_LANGS` cannot move verbatim, because its default is a
  derivation over a directory this cut deletes. It defaults to **empty**, and
  empty means *derive it* rather than *no languages*: the derivation belongs to
  the one reader of the knob, delta (4)'s arm, which expands it to the built-in
  extractor roster — the same shape `CONTEXT_KIT_MEMORY_DIRS` already carries in
  this library and this SPEC section, for the same reason (a repo-relative
  literal cannot express the value, and the reader is the only place that can).
  Transcribing the built-in roster into shell is refused outright: it would be
  the maintained list derivation-first forbids and the second producer criterion
  6 refuses.

**The honest limit this collapses, stated rather than absorbed.** Today the tool
distinguishes *unset* from *set to the empty array*, and an explicitly empty
array yields no output. After this delta the two are one case and both derive.
Nothing in tree sets it empty, no shipped sentence promises that spelling as a
way to disable the tool, and the alternative — a sentinel value meaning "none" —
would mint vocabulary for a use nobody has.

### (7) The parity oracle is bought and already committed

`bin/run-index-tests.sh` runs each tool over `index-tests/` and asserts **exact
output** against five goldens; it registers as the evidence-kit `index_tests`
validate suite. It is re-pointed at the three arms and its goldens stay
**byte-identical** {mechanical}. That is this cut's criterion-2 discharge and it
is unusually strong: the goldens were produced by the shell implementations, so
holding them byte-for-byte is a cross-substrate comparison over a committed
corpus rather than an assertion of parity.

Two of the five cases carry more than the grammar. `pub-index-shadow` points
`CONTEXT_KIT_PUB_LANG_DIR` at a scratch dir whose `rust.sh` emits a marker row
and asserts the shadow's output rather than the shipped grammar's — which is
delta (4)'s seam, proved end to end including the `bash` spawn. `pub-index-ts`
is the second extractor's whole coverage.

**The runner itself stays shell and is not this cut's to take.** It sits in the
`context-kit §Testing` group, which the composing census found **blocked as a
whole section**: `index-tests/toolfloor-cases.sh` exercises `lib/toolfloor.sh`'s
floor predicate, and that library is sequenced behind the installer's
behind-invoke relocation (`powershell-installer-surface`). A cut is selected by
stated contract, and this runner declares §Testing rather than §Index-first
reading.

### (8) The session-context hook's availability guard changes shape

`scripts/session-context.sh:38` guards its public-surface block on
`-f "$CTX_BIN/pub-index.sh"`, and `context-kit/templates/session-context.sh:41`
carries the same guard in the shipped template a consumer copies. Deleting the
script deletes the guard's subject, and the replacement guard is a **design
decision rather than a substitution** {design-bearing}: it becomes a check that
the gate binary is present and executable, taken **before** the block prints its
header.

The ordering is the point. `exec_arm` exits 2 with a diagnostic when the binary
is absent, and the existing call site swallows both channels
(`2>/dev/null || true`), so a naive substitution would print
`Public API surface of those components…` followed by nothing on every host the
artifact roster does not cover. A guard that reads the binary first degrades the
way the `-f` guard degrades today: the block is absent, not empty.

### (9) The criterion-5 residual, measured as a roster rather than reasoned

Criterion 5 asks what a consumer whose payload carries no artifact for its host
still has after the cut, and it is answered per cohort rather than per member
{design-bearing}. This cut has **no gate in it**, so the binary-less leg's gate
roster and its non-zero count do not move at all, and the instrument that prints
that roster has nothing to say about it.

What such a consumer loses is the three advisory tools outright — there is no
`gates.list` row to omit and declare, because these never had one. The residual
is therefore stated in its own terms: on a host the artifact roster covers,
nothing changes; on a host it does not, the index-first affordance is absent and
the session-context hook's public-surface block silently does not print (delta
8). That is accepted, and the ground is that these are advisory reading aids
whose absence blocks no commit, no gate and no stage — the same class of loss
the footprint emitter's "advisory bare mode did not survive the port" already
took in this kit.

### (10) Every path-bearing surface moves in the deleting commit

The count is probed rather than assumed {mechanical}:

- `.claude/settings.json` — three grants name the three deleted `bin/` paths
  (`md-index.sh *`, `md-section.sh *`, `pub-index.sh *`). Removing a grant whose
  target a ruled port cut deletes is **outside** the 2026-08-22 bar under the
  operator's 2026-08-29 settings-grant carve-out, and the removal lands in the
  same commit as the delete — the window that carve-out exists to close. **No
  replacement grant is needed**, probed rather than assumed:
  `Bash(bash gate-sdk/bin/run-gates.sh *)` is already granted, so the three
  lines are a pure narrowing. `check-settings-paths` is the oracle, and its red
  condition is a literal `.sh` grant that does not resolve — a deletion without
  its grant line reds it, which is why the two are one commit.
- `scripts/session-context.sh` — the live hook: the `pub-index.sh` execution and
  its guard (delta 8), and the three brief lines that *tell a session* to run
  the tools.
- `context-kit/templates/session-context.sh` — the shipped template carrying the
  same four sites; a consumer's copy is theirs to update and the template is
  what a fresh install seeds.
- `context-kit/bin/run-index-tests.sh` — five `check` invocations plus the
  shadow case's scratch config (delta 7).
- `scripts/context-config.sh` — the comment naming the two tools as the
  consumers of `CONTEXT_KIT_PRUNE_DIRS`.
- `README.md`'s kit-map row, `context-kit/README.md`'s three usage lines, and
  `context-kit/SPEC.md`'s layout tree.

### (11) The regeneration fan-out this cut stales

Deleting five owed `.sh` files moves `measured-claims.sh`'s `tree-shell-owed`
key, which is read off `--emit port-blockers --tree`'s trailer {mechanical}.
docs/site-architecture.md §Generated projections rules that a tree edit moving a
measured claim **stales the generated `pre-commit` and `commit-msg` hooks**,
because the baked invocation carries `check-measured-claim`'s resolved values —
"from a file no manifest names either" — and `docs/check-graph.html` with them.
The merge into `context-kit/SPEC.md` additionally stales the on-site SPEC
mirror. All three are rostered with their regen commands in that section and are
discharged in the landing commit; `check-graph` and `check-docs-mirror-fresh`
are the reds.

### (12) This amendment pairs to `kit-library-port-residue`, and the pairing is a ruling with a recorded authority

The host entry is **`kit-library-port-residue`**, not the standing composer
entry, and that is ruled rather than chosen: **lead, own authority, 2026-09-01,
over the resume channel. It did not reach the operator**, and both facts are
recorded because a pairing recorded without its authority reads at the post-port
triage as more settled than it is {design-bearing}.

**The forcing fact is arithmetic, and it is stated so the next two-amendment
iteration meets it before it authors.** `[spec:]` and `[roadmap:]` are both
lead-line-scoped (queue-kit/SPEC.md §check-tag-lead-line) and
`check-queue-wrap`'s floor resolves to 100 here, so
`native-gate-port-remaining-corpus`'s lead line — 83 columns today, of which
`- **native-gate-port-remaining-corpus** ` and ` [roadmap: now/reliability]` are
fixed — leaves at most 16 columns of basename across two `[spec: X]` tags, each
costing `8 + len(X)`. The shortest legal amendment basename is `SPEC-a.md` at 9,
so the shortest possible pair is 18. **No basename fits, at any naming**, and
the constraint is not `check-amendment-queue`, which takes every non-overlapping
`[spec:]` occurrence on a line without complaint.

**The alternative was refused rather than merely not chosen.** One amendment
spanning both cuts is the shape `native-gate-port-remaining-corpus`'s own
2026-08-30 ruling (operator, lead-relay) refuses — "three cohorts as ONE cut,
which fails this ruling's own words (*one* section, *the one* amendment)". Two
stated-contract sections take two amendments, so the two-amendment form is what
the composer requires and not a preference being preserved at a cost.

**The host is sanctioned by canon-kit rather than invented here.** §Merging an
amendment's corpus branch rules that where an entry's deliverable is a corpus and
the amendment delivers one **increment**, the entry promotes per increment and
its terminal move is a demotion "so the next increment re-promotes with a fresh
amendment". Delta 5 delivers two of this entry's six members, so that is the
applicable branch by construction, and the host is the entry whose stated blocker
this cut removes rather than a convenient one.

**This buys no non-port work, which the port-only run requires.** The amendment,
its deltas and its five files are all this cut's; the host is itself a port
entry, and no member of its residue outside delta (5) is touched.

### (13) The demotion re-prices the host entry, and the compression is owed in the demoting commit

`kit-library-port-residue` returns to the deferred section under
`[design-pending]` at build, and canon-kit/SPEC.md §Merging an amendment rules
that a demotion — unlike a Done move — lands the entry **back inside
`check-queue-entry-budget`'s per-entry cap** {mechanical}. So delta 5's roster
correction is priced against that cap and lands in the same commit that demotes
the entry, never after.

The headroom is measured rather than estimated. The entry's extent is
`TASK-QUEUE.md` lines 161-204 = 44 lines, carrying no `ruled:` line today, so its
**counted** size is 44 against `QUEUE_KIT_ENTRY_LINE_CAP` = 50 — **six lines of
headroom**. The `ruled:` line delta 12 adds is discounted by the count (at most
one line of each declaration grammar), so it costs nothing against the cap. The
build therefore has six lines to spend on stating that the two `pub-lang/`
extractors are discharged and that four members remain with their own
sequencing; if the correction wants more, spent narrative in the entry is
compressed in place in the same commit. Stated here because neither owner states
it alone — the cap is queue-kit's and the demotion is canon-kit's — and because
a build meeting it as a red is a batch spent rediscovering an arithmetic this
amendment already did.

## Producers and consumers

The amendment introduces **three interfaces** — one bridged flag each — and **no
new state, no new event, no new field, and no new knob**. Every knob named is
already shipped and already read; what changes is which process reads it and,
for two of them, which file supplies the default.

- **Producer** — `native/src/emit/mod.rs`'s `BRIDGED_ARMS` table, one row per
  arm, with no `run-gates.sh` change at all: the front-end composes
  `--emit-<name>` from its `--emit <name>` operand, which §The non-gate arm calls
  load-bearing rather than house style. The enabling config is the table row
  itself — `--knobs` publishes each member's roster and `gate_command` resolves
  it — so nothing must be configured per install, and the two defaults delta (6)
  moves into `lib/context.sh` are what make that resolution succeed rather than
  refuse.
- **Consumer of `--emit-md-index`** — a **session**, at the brief the
  session-context hook prints at every session start and at any interactive
  invocation; and the **`index_tests` validate suite**, which compares its stdout
  to a committed golden at every validate run. A session reaching a mode through
  the front-end counts exactly as a stage step does, which §The non-gate arm
  states, and the suite is a second caller that is not a test of the arm's
  existence but a consumer of its output.
- **Consumer of `--emit-md-section`** — the same two, at the same two
  transitions.
- **Consumer of `--emit-pub-index`** — three. The **session-context hook** calls
  it once per changed component directory at every session start
  (`scripts/session-context.sh`, and the template a consumer installs); a
  **session** at the brief's invocation; and the **`index_tests` suite** at
  validate, over two goldens plus the shadow case.
- **Consumer of the two built-in extractors** (delta 5) — the dispatcher alone,
  in process, once per enabled language per run. They are reachable by no other
  path, which is why they are not arms.
- **Consumer of the moved defaults** (delta 6) — `gate_command`, which sources
  `context-kit/lib/context.sh` to resolve `CONTEXT_KIT_PUB_LANG_DIR` and
  `CONTEXT_KIT_PUB_LANGS` before it execs the binary, and refuses the whole
  environment if the library does not define them. Named at that transition
  because it is the only one where their absence is visible, and where it is
  fatal rather than degrading.

**Every arm has a caller that is not a test**, enumerated above rather than
asserted, which is the third property §The non-gate arm requires. `--emit-md-index`
and `--emit-md-section` also have a golden-suite caller, and that is a
*second* caller rather than the qualifying one, so this cut leaves
`gate-test-in-tree-invoker-ruling` standing exactly as the queue-kit cut did.

**One corpus is narrowed, and its readers' red conditions are enumerated rather
than their subjects** (§The causal-completeness check, point 5). The narrowing
is the deletion of five files from the tracked `*.sh` tree:

- `check-settings-paths` — reds on a literal repo-relative `.sh` grant in
  `permissions.allow[]` that **does not resolve**. Its verdict is *not* monotone
  under this narrowing: removing a file **adds** a violation. Cleared by delta
  (10), in the same commit, not by inspection.
- `check-measured-claim` — reds when a governed sentence's `measured:` marker
  disagrees with the recomputed value. `tree-shell-owed` moves 58 → 53. No
  governed sentence pins it today, checked by scanning the markers rather than
  assumed; the *derived* consumers of the value are the baked hook invocations,
  cleared by delta (11)'s regen.
- `check-graph` / `check-docs-mirror-fresh` — red on a stale generated hook,
  graph artifact or SPEC mirror. Non-monotone for the same reason: the narrowing
  moves a baked value. Cleared by delta (11).
- `check-exec-bit`, `check-shellcheck`, `check-comment-tier`, `check-path-dialect`
  — each reds on a property of a scanned `.sh` file. All four verdicts are
  monotone in the file set, so removing five files can only remove findings, and
  they are cleared by inspection.
- `check-knob-default-coupling` — reds on a literal kit-knob default that
  disagrees across its sites or with its SPEC. Both knobs delta (6) moves are
  *skipped-and-counted* today (a computed default; an array default), so the
  gate's finding set does not move; the checked count does, which the gate
  reports and no fixture pins.
- `check-kit-ref-liveness` / `check-md-refs` — red on a dangling reference. The
  five paths appear in prose as inline code spans rather than markdown links, so
  neither gate's corpus reaches them; delta (10) repoints the prose regardless.

**Cross-component signal: this amendment's component set is three** —
context-kit, gate-sdk (§The non-gate arm's class roster) and the consumer
surfaces under `scripts/` — and a sibling amendment lands in `lifecycle-kit/`
this iteration, so `check-stage-entry` assertion C fires on both counts and the
**align stamp is demanded at the build stage's entry**. Stated here so the build
session is not the one that learns it.

## Existing sections updated

- `context-kit/SPEC.md §Index-first reading` — the three tools are restated as
  their arms: the invocation form, the byte-preserved output contracts, the
  `§`-tolerant case-insensitive fence-aware match rule and its new exit status,
  and the extractor seam's surviving resolution order with its `bash`-spawn
  execution. The section keeps every stated honest limit (deltas 2, 3, 4 and 5).
- `context-kit/SPEC.md §Layout and configuration` — the layout tree loses three
  `bin/` entries and the `lib/pub-lang/` pair; the `CONTEXT_KIT_PUB_LANGS` bullet
  is rewritten for the empty-means-derive default and the `CONTEXT_KIT_PUB_LANG_DIR`
  bullet now describes a default the library actually supplies; the paragraph
  sequencing `lib/pub-lang/` behind the resolver is retired, discharged (deltas
  5 and 6).
- `context-kit/SPEC.md §lib/context.sh` — the two moved defaults land here, and
  the "one home of every knob default above" sentence becomes true of them; the
  `CONTEXT_KIT_MEMORY_DIRS` empty-means-derive precedent gains its second member
  (delta 6).
- `context-kit/SPEC.md §Testing` — the runner's five cases now drive the arms;
  the shadow case's role as the seam's end-to-end proof is stated rather than
  implied (delta 7).
- `context-kit/SPEC.md §The session-context hook` — the public-surface block's
  guard and its degradation (delta 8).
- `gate-sdk/SPEC.md §The non-gate arm` — the class roster gains the three
  members; the `--emit-md-section` row carries the sentence the class does not
  yet hold, that a member may join the bridged-arm table with an **empty**
  declared roster because table membership is what makes the front-end's
  `--emit <name>` grammar reach it at all (deltas 2, 3 and 4).
- `gate-sdk/SPEC.md §The non-gate arm`, the spawned-program paragraph — `bash`
  gains a third arm (delta 4).
- `context-kit/README.md`'s three usage lines and `README.md`'s kit-map row
  (delta 10).
- `docs/site-architecture.md` — no ruling changes; named because delta (11)'s
  fan-out is read off it and a reader checking that the fan-out was honoured
  starts there (delta 11).
- `TASK-QUEUE.md`, the `kit-library-port-residue` entry — promoted into
  `## New Features` with `[design-pending]` swapped for this amendment's
  `[spec:]` ref, carrying the pairing ruling's own line
  `ruled: kit-library-port-residue lead 2026-09-01 own-authority` in the same
  commit as the ruling's content, and its member roster corrected at build: the
  two `pub-lang/` extractors are discharged and the four remaining members keep
  their own sequencing. It **demotes** at build rather than reaching `## Done`,
  its deliverable being a corpus of which this is one increment, and the
  demotion re-prices it against the per-entry cap (deltas 1, 5, 12 and 13).
- The generated projections this cut stales — the on-site SPEC mirrors, the
  generated `pre-commit`/`commit-msg` hooks, `docs/check-graph.html`, and the
  gate binary itself. All are rostered with their triggers and regen commands in
  `docs/site-architecture.md` §Generated projections (all deltas).

<!-- update-target-exempt: the composer entry takes no write from a cut by its own 2026-08-28 ruling — each closed cut's record lives in the contract section that cut selected, which is delta 1's section -->
- `TASK-QUEUE.md`, `native-gate-port-remaining-corpus`'s body — deliberately
  unwritten beyond what the sibling amendment's pairing already does to it.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls context-kit/SPEC-*.md`), the none-remain half discharged at
      the iteration rather than at the commit, this iteration carrying a sibling
      amendment.
- [ ] **Removals propagated** — grepped every spec, skill, template, README,
      config comment and settings file for the five deleted paths; nothing
      dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **The oracle moved, and by the roster rather than by a number** — the
      `--tree` arm lists no `context-kit/bin/md-*` or `context-kit/bin/pub-index.sh`
      row and no `context-kit/lib/pub-lang/` row at all, taken as a per-file
      roster diff and not as a trailer delta.
- [ ] **The goldens are byte-identical** — all five `index-tests` expectations
      unchanged, driven through the arms; the shadow case still records the
      shadow's output.
- [ ] **The regeneration fan-out is discharged in the landing commit** — the
      generated hooks, the graph artifact, the SPEC mirrors and the gate binary.
- [ ] **The host entry demotes inside its cap** — `kit-library-port-residue`
      returns to `## Deferred` under `[design-pending]` with its roster
      corrected, and its counted size is at or under
      `QUEUE_KIT_ENTRY_LINE_CAP` **in the demoting commit**, spent narrative
      compressed in place if the correction needs more than the six lines of
      headroom delta 13 measures.
