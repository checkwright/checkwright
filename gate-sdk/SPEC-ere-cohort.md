# SPEC amendment: ere-cohort

The fifth gate cohort of the native port: **the three canon-kit members the
`spec_manifest_files` cohort held on an ERE engine** — `check-install-claim`,
`check-payload-claim`, `check-manifest-temporal` (§The canon-kit
`spec_manifest_files` cohort, the 2026-08-12 operator hold). The cohort was ruled
by the operator 2026-08-13; a five-member clean-group alternative and an
eight-member superset were both offered and declined, so the cohort does not
widen. This amendment designs that port and the engine it pays for. It does not
restate the porting procedure (§Porting a gate to the binary substrate), the
conservation contract (§Meta-gate conservation for the binary substrate), the
criteria roster (§The port-candidate criteria), or the directive's grounds
(TRAJECTORY.md §PRIORITY DIRECTIVE — the port track's sequence).

**The economy is the whole reason this cohort exists.** Three members is the
smallest cohort since the first, and on member count alone the selection rule
would pick elsewhere. What it buys is the engine: a POSIX ERE matcher is owed by
**nine** members across four kits
(`cohort-held-members-port-prerequisites`), and it is the largest single piece of
work the port has named. Paying it against three members retires the blocker for
all nine, which is why a cohort of three outranks a cohort of five here.

## Three premises this cohort is sized on, each probed rather than inherited

### (i) Criterion 4 does not bind on any of the three, and the verdict was taken by running the derivation

§The port-candidate criteria, criterion 4 rules that the verdict "is taken by
running the derivation at cohort-cut time, never read off this paragraph". Run at
this cohort's cut, over 103 registered members and their 103 resolved declaration
paths, assertion C's derivation
(`gate-sdk/checks/check-gate-substrate-parity.sh:146-167`) reports:

| Member | Assertion C | Via | Criterion 4 |
|---|---|---|---|
| `check-install-claim` | **substrate-sensitive** | `couples=` glob `scripts/*.sh` covering `scripts/check-docs-kit-parity.sh` | **does not bind** |
| `check-payload-claim` | **substrate-sensitive** | the same couple | **does not bind** |
| `check-manifest-temporal` | not selected | — | **does not bind** |

Both selections are the **reverse-trigger** over-selection criterion 4 already
names: the `scripts/*.sh` couple exists so that an edit to this repo's transport
and disclosure emitters re-runs the gate that consumes them, and neither gate
reads a declaration path as content. What all three actually scan is
`spec_manifest_files` — the governed markdown set — inside which no gate
declaration lies. Both sensitive members already carry a conservation row
(§Meta-gate conservation, the reverse-trigger row); `check-manifest-temporal`
earns none and must not be given one.

**Criteria 1, 2, 3, 6 and 7 clear, verified rather than assumed.** All three are
registered in `scripts/gates.list`; all three carry a `good/`+`bad/` pair under
`canon-kit/gate-tests/` with no `# no-fixture:` member among them; all three land
in the generated `scripts/git-hooks/pre-commit`; `bash
gate-sdk/bin/port-blockers.sh` names none of the three, so no external program is
invoked; and their corpus derivation is `spec_manifest_files`, **already
compiled** in `native/src/spec.rs` by the canon-kit cohort, which is what makes
this cohort's only new substrate the engine.

### (ii) The engine is materially smaller than the queue's sizing, and the correction is the sizing

`cohort-held-members-port-prerequisites` sizes the union as *"`gsub` is
`check-queue-prose-precondition`'s alone and `check-deprecation-task` needs span
extraction"*, and §The first cohort states the queue-kit hold as *"porting it
means hand-writing an ERE engine plus `gsub` semantics"*. The first half of each
is wrong, and it is wrong in the direction that costs the most: it puts a
substitution engine inside the owed work.

The survey record's three ERE blocks
(`.workflow/survey-record.md`, rev `116f0d86`) establish the correct union, and
its witness was re-run at this cohort's cut rather than the survey re-bought: a
`git diff --stat` of the nine-file corpus since that rev is **empty**, and the
recorded oracle re-run gives the recorded verdict unchanged. **Eight of the nine
apply their consumer pattern only as a match test**; `check-deprecation-task`
alone extracts a span from one
(`canon-kit/checks/check-deprecation-task.sh:56` — `match(m, markerre)` then
`substr(m, RSTART, RLENGTH)`). **Every `gsub`/`sub` in the whole set runs over a
pattern baked literally into awk source** —
`check-queue-prose-precondition.sh:25-26`, `check-manifest-temporal.sh:54,61`,
`check-install-claim.sh:109`, `check-spec-derivable-section.sh:47`,
`check-brevity.sh:75-76` — which a port hand-compiles, exactly the ground that
correctly screened `check-comment-tier` out of the roster.

**The owed engine is therefore: a POSIX ERE matcher with leftmost-longest span
reporting, and no substitution engine or capture-group replacement.**

### (iii) "Sizing is foreclosed" binds the pattern language, not the API surface — and the two are one word apart

§The first cohort and §The canon-kit `spec_manifest_files` cohort both rule that
sizing the engine "to this consumer's patterns is **foreclosed**", on criterion
6's globs argument: the config surface permits what this consumer happens not to
write, and a narrow reader silently mis-scans the first consumer who writes one.
That ruling binds the **grammar** the engine accepts, and premise (ii) does not
touch it — the engine accepts any POSIX ERE, including intervals, bracket
expressions and character classes this tree's vocabularies never use.

What premise (ii) sizes is the **API**: what the nine gates *do* with a pattern
once it matches. That is fixed by the gates' own source, not by what a consumer
may write, so no future consumer can turn a match test into a substitution. The
two axes are independent, and reading the foreclosure across both is the error
this section exists to prevent — it would buy a `gsub` implementation with no
caller in the corpus that justifies the engine, which is a field with no reader
in the sense §The causal-completeness check point 4 forbids.

## The provenance seam, ruled

This cohort is the one place in the iteration where the seam actually bites, so
the three-way split is stated rather than left to the implementation.

- **Kit mechanism** — `native/src/ere.rs` in full. It is a general POSIX ERE
  engine and carries **no vocabulary whatever**: not a transport id, not a
  disclosure class, not a temporal marker. Premise (iii)'s language/API split is
  what keeps it that way — an engine sized to a grammar rather than to a corpus
  cannot encode one project's terms, and the pressure to shrink it toward this
  tree's five live patterns is exactly the pressure that would.
- **Consumer config, unchanged** — the transport vocabulary, the disclosure
  vocabulary and the temporal marker set. Two arrive through consumer *commands*
  (`CANON_KIT_INSTALL_TRANSPORTS_CMD`, `CANON_KIT_PAYLOAD_CLAIMS_CMD`), which is
  the `check-graph` / `scripts/graph-vocab.sh` pattern the seam rule names; the
  third is a consumer array over a generic-English kit default. **Delta (6) moves
  none of them across the seam** — it changes the *transport* (a bridged resolved
  value instead of an in-gate subprocess) and not the *ownership*. A port that
  compiled this repo's five patterns into the crate to avoid writing an engine
  would publish one project's distribution model in a public kit, which is the
  privacy boundary before it is a design one.
- **Kit literal, and legitimately so** — the `install-primary:` and
  `payload-discloses:` declaration regexes, the heading and fence patterns, the
  inline-code stripper. These are *grammar the kit owns*, carry no project term,
  and are the patterns delta (1)'s boundary rule leaves hand-compiled.

The tell that the seam is held: `native/src/ere.rs` and the three ported modules
should be readable by an adopter with no knowledge of this repo's install
transports, and a `grep` for any transport or disclosure id across `native/src/`
returns nothing after the port. That is the build-time check this ruling reduces to.

## What changes

### The engine

**(1) `native/src/ere.rs` — a POSIX ERE matcher, the crate's one pattern
mechanism for consumer-supplied regexes.** [design-bearing] The crate vendors
nothing, asserted by `native/src/walk.rs:544-570`'s own test, which fails the
build on a non-empty `[dependencies]`; the engine is hand-written. The accepted
grammar is **POSIX ERE in full**: alternation, concatenation, `*` `+` `?`,
intervals `{n}` `{n,}` `{n,m}`, grouping, `.`, anchors `^` `$`, bracket
expressions with ranges, negation and the POSIX character classes
(`[:alpha:]` and the rest), and backslash escaping of every special. Its public
surface is three items and no more:

- `Ere::compile(pattern) -> Result<Ere, EreError>`
- `Ere::is_match(&self, hay: &str) -> bool`
- `Ere::find(&self, hay: &str) -> Option<(usize, usize)>` — the **leftmost-longest**
  span, byte offsets, the `RSTART`/`RLENGTH` pair awk reports

There is no `replace`, no `replace_all`, and no capture-group accessor. Adding
one later is a design decision with its own reader, not an omission to be filled
in by whoever next needs it.

**(2) Leftmost-longest is the semantics, not an implementation detail, and the
distinction has a live consequence.** [design-bearing] POSIX ERE alternation is
leftmost-**longest**; a backtracking matcher gives leftmost-**first**. The two
agree on every `is_match` answer and disagree on spans, so a boolean-only engine
can be built on the wrong semantics and never show it — until `check-deprecation-task`
ports and extracts the wrong marker from `(deprecated|deprecated-since)`. The
engine is built leftmost-longest from the start, and delta (4) is what proves it.

**(3) An unsupported construct is a fail-closed refusal, never a silent
mis-parse.** [design-bearing] awk on the shell side is GNU awk, whose ERE dialect
carries extensions POSIX does not: `\y` `\<` `\>` `\B` `\w` `\s` and the
backreference forms. A hand-written POSIX engine that treats `\y` as a literal
`y` would scan a consumer's corpus and report a **clean** verdict off a pattern
that never matched what it meant — the worst failure this port can ship, because
it is invisible on both substrates' exit codes. `compile` therefore returns
`EreError` for any escape or construct outside the POSIX ERE grammar, each ported
member turns that into **exit 2** naming the offending pattern *and the knob it
came from*, and the message says the pattern uses a GNU extension the compiled
substrate does not implement. This is the §Fail-closed contract applied to a
parser rather than to a subprocess. The three live vocabularies use no extension
— verified by running this repo's emitters — so the refusal is a guard for
consumers, not a change to this tree's behavior.

**(4) The engine's acceptance oracle is a differential run against the shell's
own awk, not a unit-test suite the author writes both sides of.** [design-bearing]
A hand-written regex engine is the one component where authoring the tests and the
implementation from the same understanding proves nothing. The port stands up a
throwaway corpus and, for a generated pattern-and-subject cross product, compares
`Ere::is_match` against `awk '$0 ~ p'` and `Ere::find` against
`awk 'match($0,p){print RSTART, RLENGTH}'`, byte for byte. The generator covers
the constructs this tree's vocabularies never exercise — intervals, nested
alternation under a quantifier, negated bracket ranges, anchors inside groups,
the character classes — because those are exactly the branches no fixture pair
and no live-tree run reaches. This is criterion 2's constructed-scenario form
applied to a *mechanism* rather than to a member, the same move the canon-kit
cohort made for the default walk, and for the same recorded reason: the live tree
is the oracle a session trusts most and it proves nothing about a branch it does
not execute.

**(5) `find`'s production reader lands with the engine, in this cohort.**
[design-bearing] Point 1 of §The causal-completeness check refuses a producer
"dead everywhere but unit tests", and the span API's obvious reader —
`check-deprecation-task` — is not in this cohort. It gets one here:
`check-install-claim`'s section scanner takes a heading's text by
`match($0, /^#{2,6}[[:space:]]+/)` and `substr($0, RSTART + RLENGTH)`
(`canon-kit/checks/check-install-claim.sh:107-108`), which is a span read. The
ported member routes that pattern through `Ere::find` rather than hand-writing a
second scanner, so the span path is exercised by every invocation of a
precommit-tier gate from the day it lands.

**The rest of the baked literals stay hand-written, and this is the boundary.**
`check-manifest-temporal.sh:54`'s `sub(/^#{1,6}[[:space:]]+/, "", h)` and `:61`'s
`gsub(/`[^`]*`/, "", scan)` are substitutions over kit literals; they port as a
prefix strip and an inline-code stripper written directly, which is why delta (1)
owes no `replace`. The rule the cohort applies: **a pattern the kit owns is hand-compiled;
a pattern a consumer supplies goes through the engine.** Delta (5) is the one
deliberate exception, taken so the span path has a live reader rather than a test-only one.

### The bridged vocabularies

**(6) The two emitter-backed vocabularies become index-aligned bridgeable
arrays.** [mechanical] `check-install-claim` and `check-payload-claim` obtain
their patterns by running a consumer-configured shell command through
`spec_claim_vocabulary` (`canon-kit/lib/spec.sh:465-481`). Transliterating that
puts a `bash` spawn inside the binary, against TRAJECTORY.md §The objectives'
sixth. The mechanism to extend already exists and needs no design: the
`CANON_KIT_ENUM_SET_NAMES` / `CANON_KIT_ENUM_SET_MEMBERS` pair
(`canon-kit/lib/spec.sh:527-545`) — index-aligned arrays populated only when
`GATE_SDK_RESOLVING_KNOB` names one of them, so the subprocess runs once per gate
rather than on every source. Four new knobs follow it exactly:
`CANON_KIT_INSTALL_TRANSPORT_IDS` / `CANON_KIT_INSTALL_TRANSPORT_PATTERNS` and
`CANON_KIT_PAYLOAD_CLAIM_IDS` / `CANON_KIT_PAYLOAD_CLAIM_PATTERNS`. Labelled
mechanical because the shape, its gating and its wire constraint are all settled;
only the four names are new.

**The bridge's tab refusal is already discharged upstream, which is worth
recording rather than re-checking at build.** `_gate_knob_value`
(`gate-sdk/lib/gate.sh:139-150`) refuses any element containing a tab or a
newline. A consumer ERE legitimately *may* contain a literal tab — but
`spec_claim_vocabulary:474` already refuses a vocabulary line with an extra tab,
and the line-oriented read forecloses a newline, so no value reaching these four
knobs can violate the bridge. The constraint is a pre-existing upstream one, not
a new bound this port imposes on consumers.

### The three members

**(7) Port `check-manifest-temporal`.** [design-bearing] The narrowest of the
three: no emitter, no vocabulary bridge. Its knob reads are
`CANON_KIT_TEMPORAL_MARKERS`, `CANON_KIT_TEMPORAL_EXEMPT_SECTIONS`,
`CANON_KIT_TEMPORAL_EXEMPT_PATHS`, plus the `spec_manifest_files` set the
canon-kit cohort already declares. Two fidelity points a transliteration loses:
the marker test runs against a **lowercased** line
(`check-manifest-temporal.sh:62-64`), so the engine is applied to the folded
subject and the pattern is not folded; and `CANON_KIT_TEMPORAL_MARKERS_EXTRA`
unions onto the base array inside the library (`canon-kit/lib/spec.sh:137`)
**before** the bridge reads it, so the port declares the base knob only and must
not add a second `_EXTRA` knob.

**(8) Port `check-install-claim`.** [design-bearing] Delta (5) is inside this
one. Beyond it: `CANON_KIT_INSTALL_SECTION_RE` is a consumer ERE applied as a
match test against a heading (`:110`), each transport pattern is a match test
against a line (`:118`), and the `install-primary:` declaration regex is a kit
literal that hand-compiles. The two fail-closed exits are reproduced exactly and
are not the engine's: an unconfigured vocabulary or section regex is a **clean
skip** (`:22-25`), while a declared primary outside the configured vocabulary is
**exit 2** (`:78-82`). Assertion B's earliest-match-wins-per-section state
machine, its fence handling and its skip of the declaration line as evidence for
itself all port unchanged.

**(9) Port `check-payload-claim`.** [design-bearing] Structurally the simplest
consumer of the engine — membership over the whole document rather than position
inside a section (`:105-113`) — and it lands last of the three, so the engine and
the vocabulary bridge are both proved by two members before it. Same clean-skip
and exit-2 split as delta (8).

### Standing obligations

**(10) The conservation table's reverse-trigger row is corrected.** [mechanical]
§Meta-gate conservation's row for `check-docs-cmd`, `check-install-claim`,
`check-payload-claim`, `check-queue-slug-liveness` closes with *"`check-install-claim`
and `check-payload-claim` are still shell, so their half of the row still
describes scripts"*. After this port every member of that row is `.gate`-declared;
the sentence is rewritten to say so, and the reverse-trigger reasoning it rests on
is unaffected. `check-manifest-temporal` gains **no** row — premise (i).

**(11) The cohort's criterion-5 price is measured, and its measurement has a
dependency inside this same iteration.** [mechanical] Criterion 5 is priced per
member and **paid per cohort**: the amendment records the binary-less residual
against the post-cohort registry. That residual's oracle is
`installer/SPEC-value-arm.md`'s own subject — the omitted roster and its
completeness assertion, not `installer_smoke`'s value-arm verdict, which that
amendment corrects §The port-candidate criteria, criterion 5 to stop calling the
oracle. The instrument that prints the roster, the named binary-less `prose` leg,
does not exist until that amendment lands; before it, every profile is the
accidental no-artifact case the amendment retires, which is indistinguishable
from a deliberate measurement. That arm is currently held `fail` on the baseline
against `port-criterion-aggregate-cost-blindness`, which rides this same
iteration. **This cohort's price is therefore measured after that entry's
amendment lands**, not before, or the measurement reads a residual that is not
this cohort's own. Recorded as an ordering constraint on the build batches rather
than left for a batch to discover.

**(12) `cohort-held-members-port-prerequisites` is compressed by answering, not
by dropping.** [mechanical] That entry stands at 49 lines against a 50-line cap,
and its longest block is the ERE-engine roster this cohort **pays off**. The
engine landing *answers* those grounds, which is the compression assertion A
sanctions: the block collapses from the nine-member roster and its sizing argument
to a statement that the engine landed and which members still owe only their own
port. The write is net-negative in lines, so it needs no relief and no
authorization. Naming it here is what keeps a build session from reading a
49-line entry as a wall.

**(13) The commit-time obligations this repo already carries.** [mechanical]
`bash gate-sdk/bin/build-native.sh` before each commit, the full battery, and the
regenerated projections per docs/site-architecture.md §Generated projections —
three descriptors and three deleted scripts move the graph artifact, the
enforcement map, the footprint and value rollups and the docs mirror.

## Producers and consumers

**New interface: `native/src/ere.rs`'s three-item API (deltas 1-3).**
- *Producer* — `Ere::compile`, called by each ported member once per pattern at
  invocation setup, before its first corpus line is read.
- *Consumers* — `is_match` by all three members (delta 7's marker loop, delta 8's
  section and transport tests, delta 9's class loop); `find` by delta (5)'s
  heading extractor in `check-install-claim`, at the transition where a scanned
  line is classified as a heading.
- *Enabling config actually emitted* — none is needed for the engine itself; the
  patterns it compiles arrive as ordinary bridged knob values, and
  `GATE_SDK_NATIVE_BIN` carries a kit default so dispatch resolves with no
  consumer setting anything.
- *Every field has a named reader* — the API is three items and each has a reader
  named above. `EreError` is read at exactly one transition: the member's setup,
  which converts it to an exit-2 message. There is no `replace`, so there is no
  field without a reader — premise (iii) is what keeps one from being added.

**New interface: four bridged knobs (delta 6).**
- *Producer* — canon-kit's shell library, running the consumer's
  `CANON_KIT_INSTALL_TRANSPORTS_CMD` / `CANON_KIT_PAYLOAD_CLAIMS_CMD` at
  knob-resolution time, gated on `GATE_SDK_RESOLVING_KNOB` naming one of the four.
- *Consumer* — `check-install-claim` and `check-payload-claim`, reading resolved
  `GATE_SDK_KNOB_*` values; no `bash` spawn on the binary side.
- *Named reader at a named transition* — read once per invocation, when the member
  builds its id/pattern vocabulary, before the corpus walk begins. The **id**
  array's reader is the declared-primary membership test (`:78` / `:79`); the
  **pattern** array's reader is the per-line match test (`:118` / `:108`). Both
  arrays are read at both transitions, so neither is a field populated where it is
  not read.
- *Enabling config actually emitted* — this repo sets both commands
  (`scripts/canon-config.sh:59,67`), so both producers are live here rather than
  test-only; a consumer setting neither takes each gate's documented clean skip.

**New interface: three `.gate` descriptors under `canon-kit/checks/`, and three
`REGISTRY` tuples in `native/src/gates/mod.rs`.**
- *Producer* — this port; the descriptor's existence **is** the dispatch
  declaration (§The `.gate` descriptor). A registry member added without its
  declared walk roots, knob reads and owning kit fails to compile.
- *Consumers* — `gate_resolve` and `gate_command` (`gate-sdk/lib/gate.sh`) for
  dispatch; `--reads` by `check-reads-couples`; `--knobs` by `gate_command`'s
  bridge loop; `--list` by `check-gate-substrate-parity` assertion B.
- *Every field has a named reader* — `# graph:` by `check-graph`,
  `gen-pre-commit.sh`, `enforcement-map.sh` and `footprint.sh`; `# spec:` by
  `check-spec-pointer`; `# install:` by `check-install-disposition`. No field is
  added to the closed roster, and no member takes `# no-fixture:`.

**Red conditions of the readers this change touches.** §The causal-completeness
check point 5 binds: the port **narrows** a corpus by deleting three `.sh` files,
and "a narrower corpus can only remove violations" is false and is the first
argument this delta reaches for.

- `check-install-claim` — **its own red condition is the attested non-monotone
  case** (canon-kit/SPEC.md §The causal-completeness check names it): it reds on a
  **zero count** of `install-primary:` declarations. Deleting three governed files
  would flip it red. The port deletes gate *scripts*, which are not in
  `spec_manifest_files`, so its corpus is unchanged — stated because this gate is
  both a cohort member and the worked instance of the trap.
- `check-gate-output` — red on a **zero count** of a `: clean` / `help:` line.
  Non-monotone. All three are fixtured, so each falls to the runtime arm.
- `check-gate-fixture-coverage` — red on a member with **neither** a pair **nor**
  an opt-out: a zero-count reader, cleared because every pair moves with its member.
- `check-gate-substrate-parity` — assertion B reds on a descriptor with no
  subcommand and on a subcommand with no descriptor, so a half-landed member reds
  either way; assertion A reds on a `<name>.sh` and `<name>.gate` in one resolve
  dir, which fixes the landing order in delta (14) below. Assertion C reds on a
  substrate-sensitive member with **no disposition line** — a zero-count reader over
  a set this port does **not** widen (premise (i)), so delta (10) is a correction
  rather than an addition.
- `check-reads-couples` — red on a walk outside the declared couples; its shell
  parser finds nothing in a binary gate, so the `--reads` declaration is what keeps
  it from printing clean vacuously over three members.
- `check-readme-roster` — red in **both** directions, so `canon-kit/README.md:41,45,46`
  move from `.sh` names to `.gate` names rather than gaining them.
- `check-comment-tier`, `check-spec-pointer`, `check-todo-task-liveness`,
  `check-deprecation-task` — each loses three shell files and gains three
  descriptors plus one or more Rust modules. The `*.gate` and `*.rs` arms already
  exist on `spec_comment_surface` (canon-kit/SPEC.md §lib/spec.sh), so these are
  monotone here and clearable by inspection.
- `check-shellcheck` — loses three files; monotone. The substrate equivalent is
  `cargo clippy`, already at commit time through `check-crate-arms`.
- `check-docs-cmd` — reds on a doc fencing a path that no longer runs: real signal
  after three `.sh` deletions, and the reason the docs mirror is in delta (13).
- `check-knob-default-coupling` — reds on two literal sites for one knob carrying
  **different** literals. Delta (6) adds four knobs with exactly one literal site
  each (the library's `declare -p … || …=()` guard), so its verdict is unchanged.
  Stated because four new knobs invite the wrong edit.
- `check-crate-arms` — reds on a failing `cargo test`/`clippy` arm. Delta (4)'s
  differential harness runs under it, so a divergence between the engine and awk is
  a **commit-time** red rather than a validate-time one.
- `check-gate-binary-fresh` — red on a stale binary once a member dispatches;
  already armed, and three more dispatches do not change its predicate.

**(14) Parity is proved while both implementations exist.** [mechanical] Assertion
A forbids a `<name>.sh` and a `<name>.gate` in one resolve dir, so each comparison
runs with the shell gate still in place and the descriptor staged elsewhere; the
descriptor lands and the script is deleted in one motion. Per member: the fixture
pair, the live tree, and — for the engine rather than the member — delta (4)'s
differential corpus.

## Existing sections updated

- **gate-sdk/SPEC.md §The canon-kit `spec_manifest_files` cohort** — owned by
  premises (ii) and (iii) and deltas (1)-(9). Its three-member hold is discharged;
  the paragraph records the engine as paid rather than owed, and its sizing
  sentence gains premise (iii)'s language/API distinction so the foreclosure is not
  read across both axes again.
- **gate-sdk/SPEC.md §The first cohort, and the rule that selects the next** —
  owned by premise (ii): the `check-queue-prose-precondition` hold's *"an ERE
  engine plus `gsub` semantics"* is corrected to the engine alone, and the
  paragraph records that the engine has landed so the hold is now that member's own
  port. Also owned by the cohort-economy paragraph above: a three-member cohort
  that retires a nine-member blocker is a worked exception to the largest-set
  selection rule, recorded beside the `check-roadmap-fresh` one.
- **gate-sdk/SPEC.md §The port-candidate criteria** — owned by premise (i)
  (criterion 4's live worked instance gains a second, machine-derived case: a
  member the derivation selects through a consumer-emitter reverse trigger) and by
  delta (11) (criterion 5's per-cohort payment gains the ordering constraint when
  the measuring suite is itself under repair in the same iteration).
- **gate-sdk/SPEC.md §Meta-gate conservation for the binary substrate** — owned by
  delta (10).
- **gate-sdk/SPEC.md §lib/gate.sh** — owned by delta (6): the knob bridge gains the
  two claim vocabularies as resolved values, and the note that their tab constraint
  is discharged upstream by `spec_claim_vocabulary` rather than at the bridge.
- **canon-kit/SPEC.md §lib/spec.sh** — owned by deltas (1) and (6): the section
  states which substrate each of the two corpus primitives runs on, and gains the
  four bridgeable vocabulary arrays beside the two it already documents.
- **canon-kit/SPEC.md §check-install-claim, §check-payload-claim,
  §check-manifest-temporal** — owned by deltas (7)-(9): each section's prose naming
  the gate as a shell script or citing its `.sh` path. §check-install-claim
  additionally owns delta (5)'s ruling that its heading extraction is the span
  API's production reader.
- **canon-kit/SPEC.md §Layout and configuration** — owned by delta (6): four new
  knobs join the roster with their defaults, beside the enum-set pair.
- **canon-kit/README.md** — owned by deltas (7)-(9): three roster names, `.sh` →
  `.gate` (lines 41, 45, 46).
- **native/src/gates/mod.rs** — owned by deltas (7)-(9): the registry gains the
  three members' tuples, and the module-per-gate comment whose member count
  changes.
- **TASK-QUEUE.md `cohort-held-members-port-prerequisites`** — owned by delta (12).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`), discharged at the iteration rather than
      at the commit while sibling amendments are in flight.
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
