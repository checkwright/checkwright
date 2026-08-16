# SPEC amendment: twelfth-cohort

The size arm's group 1 — `check-close-surfaces` and
`check-queue-prose-precondition` — ports to the binary substrate. The two share
no blocker; the arm selected them on shape (`libs=fail_closed globs=-`), and
what each buys is different, which is why this amendment rules two things rather
than one.

`check-close-surfaces` is not a 64-line gate. It spawns
`lifecycle-kit/bin/close-surfaces.sh` (94 lines), whose derivation the port must
reimplement, and that script has a **second caller** — close's own step 3. So
the port is the freshness-emitter shape exactly: the derivation becomes a
non-gate arm, the gate calls it in-process, and the shell script is deleted. In
taking it, the emit-arm class acquires its first member that is **not** a stored
projection, which §1 rules rather than leaves for the next port to discover.

`check-queue-prose-precondition` buys the last member of the ERE roster its own
port, and raises the question gate-sdk/SPEC.md §The POSIX ERE matcher explicitly
left for a member with a reader: its `awk` runs `gsub`, and the engine has no
substitution arm. §4 rules it **without** widening the engine's surface, and
records why.

## What changes

### 1. A non-gate emit arm emits a document; a document need not be a stored projection

`native/src/emit/mod.rs`'s three members are all generated projections, and its
header records their named reader as "the regen command in
docs/site-architecture.md §Generated projections, and the comparator calling
`emit()`". That reader clause is **narrowed to the instances that existed**, and
this delta widens it to what §The non-gate arm actually rules: an arm owes *a
named caller*, and a regen command is one shape of caller rather than the shape.
**Design-bearing.**

The close-surface roster is a live derivation over the working tree that nothing
stores — there is no `docs/close-surfaces.md`, no freshness comparator, and
there must not be: the roster's whole value is that it is recomputed at the
moment close reads it, so a capture surface added yesterday appears today. It is
still a **document-returning arm with two named callers**, which is the whole of
what the class requires.

**Rejected: a second top-level arm, `--close-surfaces`.** §The non-gate arm
rules that "an arm receives no configuration" and that a member needing some is
reached through a front-end that already sources the shell library. The
derivation reads four bridged knobs (§2), so a bare top-level arm would either
resolve nothing or mint the second entry point into the crate that the class
forbids. `run-gates.sh --emit` is the front-end that already exists and already
builds the bridged environment (`gate_knob_env "$EMIT_ARM"`,
`bin/run-gates.sh:35`), so riding it costs no new mechanism.

**The word "projection" in the front-end's operand is the only thing that
strains, and it is prose, not structure.** The arm spelling is derived —
`arm_name()` is `format!("--emit-{}", projection)` — so nothing keys off what
the operand is called; `run-gates: --emit needs a projection name` becomes
`--emit needs an arm name`. No mapping table is added and none is drifted.

### 2. `bin/close-surfaces.sh` ports to `native/src/emit/close_surfaces.rs`

The script is deleted and its derivation lands as
`("close-surfaces", close_surfaces::emit, &[...])` in `EMITTERS`, reached as
`bash gate-sdk/bin/run-gates.sh --emit close-surfaces [scan-root]`.
lifecycle-kit/SPEC.md §The close-surface roster is unchanged as a *contract* —
two sources unioned, the mode echoed verbatim, `(undeclared)` for an
undeclared capture member — and this port asserts nothing new about it.
**Design-bearing.**

Every library piece is already in the crate, which is what makes this a sizing
rather than a hold:

- the declaration surfaces — `walk::kit_roots_rel()` (bridged as
  `GATE_KIT_ROOTS_REL`) joined with `LIFECYCLE_KIT_ROSTER_BASENAME`, unioned
  with `walk::glob_files` over `LIFECYCLE_KIT_CLOSE_SURFACE_GLOBS`. `glob_files`
  is already `**`-capable and bash-faithful, which the consumer's
  `*/SPEC.md`-shaped globs need and the `shopt -s globstar` in the script is
  there for;
- the capture tier — `proc::run("git", ["-C", base, "check-ignore", "-q", "--", rel])`,
  with `native/src/gates/workflow_tiering.rs:110` the live precedent for that
  exact call and its three-way exit split;
- the extraction — a plain line loop. The `awk` program skips fenced blocks and
  matches a full-line `close-surface:` lead token; neither needs a regex, so
  this member reaches `ere.rs` nowhere.

**Three details that are behavior, not implementation, and are stated so build
reproduces them rather than rediscovering them.**

**(a) The base is computed, never `cd`'d into.** The shell branches: with a
scan-root argument it `cd`s there and treats it as the world; with none it jumps
to `git rev-parse --show-toplevel`. `std::env::set_current_dir` is
process-global and the crate anchors instead (`queue_slug_liveness.rs:72`,
`template_registry_parity.rs:57` are the precedents), so the arm resolves an
**effective base** — the argument, else the toplevel via `proc::run` as
`fresh.rs:23` already does — and every path it globs, ignores and prints is
relative to that base. The `./`-stripping the shell does in `_add_surface`
becomes the relativization.

**(b) The sort's tie-break is the whole row, and that is observable.** The shell
ends `LC_ALL=C sort -t'\t' -k1,1` **without `-s`**, so GNU sort's last-resort
whole-line comparison decides ties — and ties are reachable, because one path
may be declared on two surfaces, which the derivation deliberately does not
collapse (it collapses duplicate *surfaces*, not duplicate declarations). The
port sorts by `(path, whole row)` byte-wise. A path-only sort would leave tie
order at the sorter's discretion and churn the gate's error order for no edit.

**(c) An empty roster prints nothing and exits 0.** The shell's
`[[ ${#ROWS[@]} -eq 0 ]] && exit 0` guard, preserved: an empty document is a
resolved-empty roster, not an error, and the gate reads it as zero surfaces.

The affordance contract lifecycle-kit/SPEC.md §bin/close-surfaces.sh states —
repo-root resolution, config-via-env, exit 2 on a non-repo cwd, an unreadable
declaration surface, or an undecidable `git check-ignore` — carries over whole.
Exit 2 becomes the arm's `Err(String)`, which the front-end and the in-process
caller both surface; §Fail-closed contract is what forbids degrading any of the
three into a silently smaller roster.

### 3. `check-close-surfaces` ports and calls the arm in-process

`lifecycle-kit/checks/check-close-surfaces.sh` is replaced by
`lifecycle-kit/checks/check-close-surfaces.gate` dispatching to
`native/src/gates/close_surfaces.rs`, which calls `emit::close_surfaces::emit`
directly rather than spawning anything. Its three assertions and their
calibration are unchanged — lifecycle-kit/SPEC.md §check-close-surfaces stays
the contract. **Design-bearing.**

The in-process call is the point, not an optimization: it is what makes the
"derivation and gate can never disagree" guarantee structural instead of
conventional, the same dividend the freshness family banked. It also means the
descriptor acquires a source coupling, per §The non-gate arm's rule that a gate
reaching an arm in-process owes its descriptor every module it reaches
transitively. So `couples=` grows `native/src/gates/close_surfaces.rs` and
`native/src/emit/close_surfaces.rs` beside the surfaces it already names —
`context-kit/checks/check-footprint-fresh.gate` is the worked precedent for the
spelling. Omitting them leaves the gate green and un-triggered on the edit that
broke it.

**The three assertion markers move with the assertions.** `close_surfaces.rs`
carries `// assertion A:`, `// assertion B:` and `// assertion C:` comments
mirroring the shell member's, because `check-gate-assertions` set-matches them
against lifecycle-kit/SPEC.md §check-close-surfaces' "Three assertions… (A)(B)(C)"
span and reds on an empty marker set. Its grep takes either comment leader, so
the Rust spelling satisfies it unchanged — `native/src/gates/stage_entry.rs` is
the precedent. This is the one obligation of the port that is invisible in the
gate's own source and visible only in another kit's SPEC prose, which is why it
is a delta rather than a build detail. **Mechanical**, the marker set being
fixed by the contract.

`FORCED_RE` — `^forced=[A-Za-z0-9._/-]+\.md[[:space:]]+§[^[:space:]]` — is a
bash `=~` pattern baked into the member's own source, not consumer config, so it
hand-compiles as a byte scan and this member reaches `ere.rs` nowhere either.
Assertion B stays shape-only for the reason lifecycle-kit/SPEC.md already gives
(canon-kit's heading resolver is private to its own compiled member, so there is
still no resolver a second gate could call); the port does not change that and
must not be read as an opportunity to.

### 4. `check-queue-prose-precondition` ports, and `gsub` is a caller-side loop over `Ere::find`

`queue-kit/checks/check-queue-prose-precondition.sh` is replaced by
`queue-kit/checks/check-queue-prose-precondition.gate` dispatching to
`native/src/gates/queue_prose_precondition.rs`, shell script deleted. The
invariant is unchanged from queue-kit/SPEC.md §check-queue-prose-precondition.
**Design-bearing.**

Its consumer knob `QUEUE_KIT_PRECONDITION_REGEX` is applied as a **match test
and nothing else** (`b ~ trig`), so it is one of the eight members
gate-sdk/SPEC.md §The POSIX ERE matcher counted when it sized the owed engine as
a matcher; that sizing is confirmed by this port, not disturbed by it. What the
member also does is run **two `gsub`s over patterns baked literally into its own
awk source**:

```
gsub(/\[[^]]*\]/, " ", b)
gsub(/(once|when|after)[^.,;]*(landed|shipped|merged|resolved|completed|was [a-z]+ed)/, " ", b)
```

**The ruling: the substitution is a loop in the gate module over the existing
`Ere::find`, and `ere.rs`'s public surface stays exactly three items.**
`Ere::find` reports the **leftmost-longest** span, which *is* awk's `gsub` match
rule, so the loop is: find in the remaining tail, append prefix and the
replacement, advance past the match, and on an empty match advance one byte.
Neither pattern here can match empty, but the loop states the rule anyway
because a silent infinite loop is the failure mode it has.

**Rejected: adding `Ere::replace_all`.** §The POSIX ERE matcher fixes the
engine's surface at `compile`/`is_match`/`find` and rules that "adding one is a
design decision with its own reader rather than an omission to fill in". This
amendment has a reader — but it is **one** reader, in **one** member, and the
recorded ruling is what forecloses the addition until a second appears. The
promotion trigger is therefore stated rather than left to be re-argued: a
**second** ported member needing substitution promotes the loop into `ere.rs` as
that fourth item, with the differential oracle §The acceptance oracle already
stands up widened to `awk 'gsub(p,r){...}'`. Until then a twelve-line private
loop is cheaper than a public contract with one caller.

**Rejected: hand-compiling the second pattern into a bespoke scanner.**
gate-sdk/SPEC.md:1948 says of this member that its `gsub` "runs over a pattern
baked literally into awk source, which a port hand-compiles". That clause's
*claim* — that a substitution engine is outside this member's ERE hold, and the
substitution is the port's own cheap work — is confirmed here and is why this
delta is twelve lines. What it did not anticipate is that the engine would
already be paid when the member ported: hand-compiling
`(once|when|after)[^.,;]*(landed|shipped|merged|resolved|completed|was [a-z]+ed)`
means writing an alternation, a negated-class gap and a `was [a-z]+ed`
sub-matcher by hand, with leftmost-longest arbitration, in a gate module —
roughly forty lines of the exact code the engine exists to stop anyone writing,
next to a compiled matcher that already answers it. So the clause is **refined,
not reversed**: the port owns the substitution, and it spends the paid matcher
to do it. The first pattern, `\[[^]]*\]`, is a two-byte scan either way and goes
through the same loop for consistency rather than acquiring a second spelling.

**Rejected, and refuted by a live run rather than by argument: replacing the
substitution with span-exclusion.** The tempting reformulation is to skip the
rewriting and instead test whether the trigger's match span falls outside the
bracket and past-tense spans. It is **not equivalent**, because `gsub` replaces
with a space and a space can *bridge* text that was not adjacent before. Probed
at this rev, both cases going from no-match to match:

```
a gated[x]on b            ->  a gated on b
c waitingonce foo landedon d  ->  c waiting on d
```

The port therefore builds the rewritten string, exactly as awk does.

**`tolower` is ASCII, not Unicode, and this is a correctness point rather than a
style one.** The shell's `tolower(body)` runs under the C locale, and `ere.rs`
is documented byte-wise precisely because "`find`'s offsets are handed to
byte-indexed slicing, and a char-wise engine shifts every one of them on a
multi-byte glyph". Rust's `str::to_lowercase` is Unicode-aware and can change a
string's **byte length**, which would desynchronize the offsets this loop slices
with. The port uses `to_ascii_lowercase`. **Design-bearing.**

The remaining machinery is already in the crate: the active/section boundary
scan is `QUEUE_ACTIVE_RE` and `QUEUE_SECTION_RE` over `native/src/queue.rs`,
which the queue-kit cohort paid for, and the `[blocked-by:` / `[precondition-ok:`
detection is a substring test with no pattern in it.

### 5. Fixtures and gate-tests move with their members

Both ported members keep their `good/`+`bad/` fixture pairs and their existing
`gate-tests/` suites, which move rather than being re-authored — including
`lifecycle-kit/gate-tests/check-close-surfaces.test.sh`, whose real-git-repo
scenarios are the only coverage the capture-tier branch has.
`lifecycle-kit/bin/close-surfaces.sh` owes no fixture pair today and its
replacement owes none either, for the same reason from two directions:
lifecycle-kit/SPEC.md calls it advisory tooling, and §The non-gate arm rules
that an arm returning a document has no pass and no fail to fixture.
**Mechanical.**

The ERE-substitution loop earns unit coverage under `check-crate-arms` for the
three cases the fixture pair cannot reach: a subject with no match (returned
unchanged), consecutive matches (both replaced, no bytes lost between them), and
the empty-match advance. **Mechanical.**

### 6. The generated projections restale together, and the binary must be rebuilt

Two members change dispatch and one shell tool disappears, so the generated
pre-commit hook, the graph artifact, the enforcement map, the footprint and
value rollups and the docs mirror all restale, and
`bash gate-sdk/bin/build-native.sh` must run — which the battery does not
discharge. The roster and each regen command are docs/site-architecture.md
§Generated projections'; this amendment names the fan-out rather than restating
it. **Mechanical.**

### 7. The seam ruling

**Kit mechanism:** the widened emit-arm reader clause, the close-surface
derivation, both ported gates, and the substitution loop. All generic — no term
list, no vocabulary, no product constant crosses into a kit literal.

**Consumer config:** unchanged, and **no knob is minted by this amendment**. The
four knobs the arm reads (`GATE_KIT_ROOTS_REL`, `LIFECYCLE_KIT_ROSTER_BASENAME`,
`LIFECYCLE_KIT_CLOSE_SURFACE_GLOBS`, `GATE_SDK_WORKFLOW_DIR`) and the one
`check-queue-prose-precondition` reads (`QUEUE_KIT_PRECONDITION_REGEX`, plus
`QUEUE_KIT_QUEUE_FILE` and the section knobs behind `QUEUE_ACTIVE_RE`) all exist
and keep their kit defaults and their `scripts/*-config.sh` overrides. They move
from being read by a shell library to being resolved across the bridge, which is
a substrate change and not a config change.

**Private rule content:** none is involved, and the one place it could have
leaked is worth naming because the port walks right past it.
`QUEUE_KIT_PRECONDITION_REGEX`'s default is a **phrase set** — a vocabulary —
and it stays where it is, as a kit default a consumer overrides, transported
across the bridge and interpreted by the engine. The port does **not** bake it,
or any part of it, into the crate: `check-queue-prose-precondition` compiles
whatever the knob resolves to. The two `gsub` patterns are a different thing and
are correctly crate literals — they are the member's own source, fixed by its
contract rather than by what a consumer writes, which is the same line §The
POSIX ERE matcher draws when it says the sizing correction "binds the pattern
language, not the API surface".

## Producers and consumers

**New interface: the `--emit-close-surfaces` arm.**
- *Producer* — `native/src/emit/close_surfaces.rs::emit`, registered in
  `EMITTERS` and resolved by `emit::lookup` in `main` before the registry
  lookup, so it stays outside `--list` and §check-gate-substrate-parity
  assertion B's equality holds in both directions. Enabling config: the four
  knobs above, declared as the registry tuple's third element, which is the data
  `--knobs` prints and which `gate_knob_env` resolves — the same construction
  that makes an arm's knob declaration un-omittable.
- *Consumers, two, both named and both live on the day it lands.* First,
  `native/src/gates/close_surfaces.rs`, calling `emit()` **in-process** — no
  spawn, no serialization round trip. Second, close's step 3
  (`lifecycle-kit/templates/stages/close.md:110`), through the
  `bash gate-sdk/bin/run-gates.sh --emit close-surfaces` front-end, which
  resolves the bridged environment in front of the arm. Had it only the second,
  it would still satisfy §The non-gate arm; had it neither, the class's own rule
  deletes it.

**No new field on the row.** The emitted row keeps its four tab-separated
fields, each with the reader it already has: **path** is read by
`check-close-surfaces` at assertions A and C (the capture-tier test) and by
close's disposition sweep; **mode** at assertion B (present, and `advisory` or a
well-formed `forced=`) and by close, which routes a `forced=` row differently
from an `advisory` one; **reclaim** at assertion C (a capture-tier row names a
drain) and by close when it runs that drain; **owner** by assertion B's and C's
error text, which names the declaring surface so the fix has an address, and by
close when an `(undeclared)` row's owner reads `-`. Nothing is populated at a
transition where nothing reads it, and no fifth field is added.

**New crate interfaces: two gate modules and one emit module.**
- *Producer* — this amendment's build. *Consumer* — `gate_command`, which
  resolves `<dir>/<name>.gate` to the two-element binary argv, and
  `gates.list`, whose registration is unchanged: a member's name does not change
  with its substrate.

**New internal interface: the substitution loop.**
- *Producer* — `queue_prose_precondition.rs`, over an `Ere` compiled from a
  literal in its own source. *Consumer* — the same module's trigger test, at
  exactly one transition: after both rewrites, before `is_match` against the
  consumer regex. It is deliberately **not** public and deliberately **not** in
  `ere.rs` (§4), so it has no reader outside the module that owns it and the
  engine's three-item contract is untouched.

**Red conditions, because this delta narrows three corpora.** The build deletes
two `*/checks/*.sh` members and one `*/bin/*.sh` tool, so every reader over
those corpora sees a **narrowing**, and a narrowing may *add* violations where a
verdict is non-monotone (canon-kit/SPEC.md §The causal-completeness check, point
5). Enumerated by red condition rather than by subject:

- `check-gate-assertions` — **the reader this port most easily misses, and it is
  non-monotone twice over.** It couples a `### <gate>` section's enumerated
  assertion span to the resolved gate file's assertion markers, and reds both on
  a file carrying **zero** markers and on a marker set that is **not equal** to
  the contract span. lifecycle-kit/SPEC.md §check-close-surfaces enumerates
  "Three assertions… (A)(B)(C)", and today `check-close-surfaces.sh` carries
  `# assertion A:`, `# assertion B:` and `# assertion C:`. The marker grep is
  `(#|//)[[:space:]]*assertion[[:space:]]+[A-Za-z0-9]+:` — **the comment leader
  is the substrate's** — so `close_surfaces.rs` carries `// assertion A/B/C:`
  markers or the gate reds on an empty set. `native/src/gates/stage_entry.rs`
  is the worked precedent. `check-queue-prose-precondition` is **not** exposed:
  queue-kit/SPEC.md's section for it enumerates no assertion span, so its module
  owes no markers.
- `check-install-disposition` — **and not `check-install-claim`, which is a
  different gate reading a different declaration.** `check-install-claim`'s
  zero-count red is over `install-primary:` declarations in the governed-doc
  set, so it never sees a `checks/` file; the reader of the `# install:` header
  is `check-install-disposition`, whose corpus is `checks/check-*.sh` **and**
  `checks/check-*.gate`. Two red conditions: a file carrying other than exactly
  one `# install:` line (per-file, **monotone** under deletion), and a
  `zero-config` gate its kit's `smoke/install.sh` does not register
  (**non-monotone** — a registration is a zero-count assertion). So
  `# install: on-surface` and `# install: zero-config` are carried verbatim onto
  the two descriptors (`check-footprint-fresh.gate:2` is the worked precedent
  for the field on a `.gate`), and `check-queue-prose-precondition` must stay
  registered in `queue-kit/smoke/install.sh` — a registration keyed by gate
  **name**, so the extension change alone does not disturb it.
  `bin/close-surfaces.sh` carries **no** `# install:` line, so its deletion is
  the monotone direction — probed at this rev, and to be re-probed at build
  rather than taken from here, since a line added meanwhile flips it.
- `check-gate-substrate-parity` — assertion A reds on a directory carrying
  **both** `<name>.sh` and `<name>.gate`; assertion B reds on the descriptor set
  not **equalling** the binary's `--list` roster. **Non-monotone both ways**:
  every intermediate state of a staggered landing is red. Discharged by landing
  descriptor, module, `mod.rs` registry entry and shell deletion in **one commit
  per member**. Its assertion C is **ruled out, checked rather than assumed**:
  neither member's `couples=` matches a resolved gate-declaration path
  (`.workflow/*`, `kit:SPEC.md`, `.claude/commands/*.md`, `.gitignore` for one;
  `TASK-QUEUE.md` for the other), so neither is substrate-sensitive and neither
  owes a row in §Meta-gate conservation for the binary substrate. That is a
  difference from the ERE cohort, whose two sensitive members did owe one.
- `check-gate-fixture-coverage` — reds on a registered member with **no**
  fixture pair; a zero-count condition, **non-monotone**. Lookup is by gate
  **name** at `gate-tests/<name>/{good,bad}`, so it is extension-agnostic and
  discharged by leaving each pair where it is (§5).
- `check-reads-couples` — reds on a member walking a tracked path no `couples=`
  covers. **Non-monotone**: the ported `check-close-surfaces` declares real walk
  roots where the shell member declared none analyzable, so the coverage
  assertion runs for real for the first time on this member. Two readers must
  agree, not one: the commit-time gate, and the crate's own
  `every_registry_member_declares_the_roots_it_walks` unit test under
  `check-crate-arms`, which reads the `mod.rs` registry tuple's roots element.
  The derivation walks the kit roots, the close-surface globs and the workflow
  directory, so all three are declared. Build re-runs the analyzer rather than
  reasoning about it.
- `check-graph` — assertion A reds on a `.gate` without a well-formed
  `# graph:` line; assertions D and E red on the committed hook and graph
  artifact differing from a live re-emission. The hook is **derived from
  `couples=`**, so a descriptor missing a module leaves the gate registered,
  green, and never triggered on the edit that breaks it. That is a silent
  failure no reader reports, which is why §3 states the coupling as a delta
  rather than leaving it to the regen.
- `check-gate-output` — for a fixtured member it defers to the fixture run
  rather than grepping source, so the substrate swap is invisible to it. The
  obligation it carries forward is **behavioral** and rides the Rust: the clean
  line stays `CLOSE-SURFACES: clean (…)` / `QUEUE-PROSE-PRECONDITION: clean (…)`
  and the failure text keeps its `help:` line. The fixtures' `expect.txt` files
  are the oracle for that, and they do not move (§5).
- `check-spec-pointer` — its **prose-citation pass** over the manifest set
  resolves a `<path>.md §<heading>` citation, and deleting the script retires
  lifecycle-kit/SPEC.md's `### bin/close-surfaces.sh` **heading**, at which point
  any surviving citation to it resolves to nothing. **Non-monotone.** The
  section is renamed rather than deleted (§Existing sections updated) and every
  citation to it moves in the same commit. Its *comment-surface* pass, and
  `check-comment-tier`, are a separate matter: `CANON_KIT_COMMENT_SURFACE` is
  configured nowhere in this tree outside fixtures, so their live comment corpus
  here is empty. That is reported as read from the config rather than from a
  run, and is a **build-time probe**, not a licence to drop a `# spec:` line —
  every comment carried onto a descriptor is carried because it is a directive.
- `check-shellcheck`, `check-exec-bit`, `check-template-copy-parity` — corpora
  are `*/checks/*.sh` and `*/templates/*.sh`; each reds per finding on a file
  that is present. **Monotone under deletion**; clear by inspection.
- `check-crate-arms` and `check-gate-binary-fresh` — the crate grows three
  modules and the binary must be rebuilt; both red until `build-native.sh` runs.
  They red on staleness, which the rebuild clears.
- `check-footprint-fresh`, `check-value-rollup-fresh`, `check-enforcement-fresh`,
  `check-docs-mirror-fresh` — each reds on a byte difference between a tracked
  projection and a live re-emission. **Monotone in the sense that matters**: the
  regen clears them, and §6 names the fan-out.

This enumeration is authored from the gate sources at this rev and is **the
list build starts from, not the list build trusts**: the battery is the oracle,
and a reader that reds outside this set is a finding to resolve that session
under the causal-gap rule, never a deferred TODO.

## Existing sections updated

- **gate-sdk/SPEC.md §The non-gate arm** — owned by delta 1. The class's
  "owes a named reader instead" bullet currently exemplifies with
  `--source-stamp`, `--queue-parity` and `--declaration-parity`; it gains the
  ruling that a *caller* is the requirement and a regen command or a stored
  projection is one shape of it, so an arm emitting a document nothing stores
  is a member in good standing. `native/src/emit/mod.rs`'s own header comment
  carries the same narrowing and is corrected with it.
- **gate-sdk/SPEC.md §The first cohort, and the rule that selects the next** —
  owned by deltas 2–4. The twelfth cohort's record lands here: members, what the
  size arm selected on, the sizing correction the tool does not print (the pair
  is not two 64-line gates; it is ~222 shell lines, unevenly split), and the
  price. `check-queue-prose-precondition`'s hold entry — whose closing sentence
  is "The engine has landed, so what this member is held on is its own port" —
  becomes a **retired** hold naming this increment, keeping its grounds, since
  the section is canonical for every member's hold and its disposition.

  The same section carries a **debt this cohort discharges**: "**`check-close-surfaces`
  is out rather than held**: it sources no `lib/stages.sh` at all, so it is
  *unsized*, and a later selector owes it a sizing rather than inheriting a hold
  whose ground was never established." This increment is that later selector,
  and the sizing it owed is recorded with its **cause**, which is the reusable
  half: the member reads as unsized *because* its shell-level dependency set is
  measured on the gate file, and this gate's work is behind a **spawn** —
  `check-close-surfaces.sh` sources only `lib/gate.sh`, while the
  `bin/close-surfaces.sh` it spawns sources `lib/stages.sh` and carries the
  derivation. A spawned tool is invisible to every static sizing signal the
  selector has, `port-blockers.sh --group`'s line counts included. That is the
  general rule the section gains, stated once here rather than rediscovered per
  member; the remaining live instance is filed to the gap inbox rather than
  fixed here, since it is another member's entry.
- **gate-sdk/SPEC.md §The POSIX ERE matcher** — owned by delta 4. The
  three-item surface and the "no `replace`, no `replace_all`" ruling are
  **unchanged and are the reason the delta reads as it does**, so they are cited
  in the new text rather than softened. What the section gains is the recorded
  promotion trigger — the second member needing substitution is what promotes
  the loop into the engine — and a one-line correction to its own
  "which a port hand-compiles" clause, which stays true of the *substitution*
  and is refined on the *pattern*: a member ports against the engine that has
  since landed rather than around it.
- **lifecycle-kit/SPEC.md §bin/close-surfaces.sh** — owned by delta 2. The
  heading is **renamed**, not deleted: the derivation still exists and is still
  lifecycle-kit's contract, it is simply no longer a script. The affordance-
  contract sentence (repo-root `cd`, config-via-env, the three exit-2 causes) is
  restated for an arm — the `cd` becomes the computed base, the exit 2s become
  the arm's error return — and the invocation becomes the `--emit` front-end.
  The "advisory tooling, no fixture pair owed" sentence keeps its verdict and
  gains §The non-gate arm as its second, now-structural ground.
- **lifecycle-kit/SPEC.md §The close-surface roster** — owned by delta 2, for
  the one sentence naming the derivation as `bin/close-surfaces.sh`. Its
  substance — two sources unioned, and why the second source is the closure that
  makes the roster fail loudly — is untouched.
- **lifecycle-kit/SPEC.md §check-close-surfaces** — owned by delta 3, for the
  fixture-and-coverage sentence naming the shell script's pair, and for the
  in-process call replacing the spawn. Its note that the port to the binary
  substrate did **not** give canon-kit a callable heading resolver stays exactly
  as written and is now load-bearing for a second member. The "Three
  assertions… (A)(B)(C)" enumeration is **unchanged** and is what the ported
  module's markers must match (§3).
- **lifecycle-kit/SPEC.md §Testing, the `gate_run` paragraph** — owned by
  deltas 2 and 3, and the staleness this port most directly creates. It reads
  "Every gate-driving runner in this kit, including `check-close-surfaces` —
  **whose gate remains shell, unsized by any cohort** — resolves through
  `gate_run` rather than a held `checks/<name>.sh` path", with
  `check-stage-entry` named as "the worked payoff rather than an exception: its
  runner needed no edit when the gate ported, because it already named the
  gate." The port falsifies the parenthetical and, in the same motion, makes
  this member the **second** instance of the payoff — verified, not assumed:
  `gate-tests/check-close-surfaces.test.sh` dispatches
  `gate_run check-close-surfaces "$DIR/checks"` by name and needs no edit. So
  the clause is not merely deleted; the example is promoted from
  still-shell-illustration to second attestation, which is the stronger claim
  the paragraph was written to earn.
- **lifecycle-kit/gate-tests/check-close-surfaces.test.sh** — owned by delta 3.
  Its header comment reads "Behavioral test of checks/check-close-surfaces.sh";
  the file itself needs no change (the in-process call is what delta 3 rules),
  only that one path in the comment.
- **lifecycle-kit/templates/stages/close.md, step 3** — owned by deltas 1 and 2:
  `run bin/close-surfaces.sh` becomes
  `run bash gate-sdk/bin/run-gates.sh --emit close-surfaces`. Everything about
  what to do with the rows is unchanged.
- **queue-kit/SPEC.md §check-queue-prose-precondition** — owned by delta 4, for
  the fixture sentence and for any statement of substrate. The invariant, the
  opt-out tag's any-position reach, and the past-tense strip's purpose are
  unchanged.
- **CLAUDE.md** — owned by deltas 2 and 4, the two shell-path deletions,
  conditionally: only if it names either path directly, which it does not at
  this rev (grepped, zero hits) — the port changes no rule it states. Re-checked
  at build rather than assumed stale from this reading.
- **docs mirror of every file above** — owned by delta 6, regenerated not
  edited.

No wire-contract delta is embedded: the two fenced blocks above are this
member's own awk source quoted as the subject of a ruling, and the third is a
probe transcript.

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
