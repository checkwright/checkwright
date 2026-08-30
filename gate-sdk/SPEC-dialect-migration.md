# SPEC amendment: path-dialect migration

Amends gate-sdk/SPEC.md §The path-dialect contract. That section states how a
root call site is **judged**; this amendment states how the tree is **migrated**
onto it, corrects three of its clauses that the migration's own census falsified,
and lands the gate that keeps the claim true while the port continues.

Scope boundary, so it is not re-argued: `msys-path-dialect-boundary-unmodelled`
landed the contract and the four drift-kit `bin/` sites its own cut touched. This
amendment owns everything else — the crossing idiom per substrate, the corpus,
and the enforcement.

## What changes

### (1) The declared dialect becomes per-substrate

The **declared dialect** clause is replaced. It reads today "Every root variable
in this tree is POSIX-spelled — forward separators, no drive letter." That names
**one** dialect and the tree provably holds two, so this is a correction of a
false clause rather than a relaxation of a true one. {design-bearing}

The crate's own crosser does not deliver the clause and must not. `normalize_abs`
keeps `path_root(abs)`, so `C:\repo` becomes `C:/repo` — separators normalized,
drive letter preserved — and §The crate's crosser already says so in as many words
("the composed result carries the **input's own** root"). Teaching the crate to
strip the drive is refused, and the refusal is the substantive content here: a
`*-windows-msvc` binary reaches the filesystem through `std::path`, which cannot
resolve `/c/repo`. There the drive letter is not residue, it is the only absolute
spelling that works. An MSYS bash process wants the opposite — its own `getcwd(3)`
answers `/c/repo`, and every sibling path such a shell derives carries that
spelling, so a root spelled `C:/repo` compares unequal to a path spelled `/c/repo`
while naming one directory. Each substrate's correct dialect is the other's
defect, and no single spelling is available to declare.

The clause becomes: **a root is spelled in its own substrate's dialect — forward
separators, absolute by that substrate's rules — and is converted into it once, at
the producer.** MSYS bash holds `/c/repo`; a `windows-msvc` binary holds `C:/repo`.
What survives unchanged is the part that carries the weight: no value inside the
tree is in a foreign dialect, and no value is normalized twice.

Why the split leaks nothing is stated with it, because it is the first thing a
reader will doubt: **roots cross the shell/crate boundary relative, never
absolute.** `walk::kit_roots_abs` re-absolutises each bridged root against the
crate's own cwd, and its existing note — "a bridged root crosses spelled relative
to the invoking directory" — is this same rule already written from the crate's
side. No value ever arrives in a substrate whose dialect it was not spelled in, so
the per-substrate declaration costs nothing at the seam. The one place the two
spellings would meet is named as a non-target in delta 4.

### (2) The shell crossing idiom is named, and `pwd -P` is named as the half that crosses

§The boundary, and who crosses it blesses `cd … && pwd` as POSIX by construction.
That sentence is **true only of `pwd -P`**, and as written it licenses a no-op
migration — so the correction is the delta, not a citation to it.
{design-bearing}

bash's `cd` with an **absolute** argument sets `PWD` from the argument itself and
does not call `getcwd`. So `cd 'C:/repo'; pwd` prints `C:/repo` straight back,
unconverted, and a migration written that way changes nothing while looking
exactly like the fix. `pwd -P` calls `getcwd(3)`, which under the MSYS runtime
answers in the MSYS spelling. The crossing lives in `-P`.

The idiom, in the general cwd-preserving form most sites take:

```sh
ROOT="$( { cd "$(git rev-parse --show-toplevel 2>/dev/null)" && pwd -P; } 2>/dev/null )"
```

and, where the entry point means to *be* at the root, the two-line form that keeps
each site's existing refusal arm verbatim:

```sh
cd "$(git rev-parse --show-toplevel 2>/dev/null)" || { …refuse exactly as today…; }
ROOT="$(pwd -P)"
```

Two properties are asserted with it rather than left to a reader. `pwd -P` also
resolves symlinks — a behavior change accepted rather than overlooked, and a no-op
in practice because `git rev-parse --show-toplevel` already answers a physical
path. And the `|| pwd` hedge several sites carry survives inside the substitution
untouched: it is a missing-repository guard, the contract already rules that it
confers nothing against dialect, and the idiom neither needs it gone nor is
weakened by it.

**No shared shell normalizer is introduced, and the refusal is load-bearing.** The
census found none in any kit's `lib/`, so one would be new rather than a second
copy — but the three families in the corpus source three different libraries and
two resolve their root before sourcing anything at all, so a shared helper buys a
new cross-kit dependency to save a one-line idiom, against CLAUDE.md's provenance
seam and for no error class the gate in delta 5 does not already catch. The idiom
needs no name, and two sites in this tree already write it —
`scripts/producer-liveness-reader.sh` and `scripts/pack-installer.sh` — neither as
a dialect measure, which is the evidence that it is the shape a shell author
reaches for unprompted.

### (3) `native/src/walk.rs` becomes the crate's sole platform-native producer

The crate already declares `walk.rs` the single owner of absoluteness. This
extends that ownership one step back, to the **producers**: `std::env::current_dir()`,
the `git rev-parse` spawn and `std::fs::canonicalize()` live in `walk.rs` and
nowhere else in the crate. {design-bearing}

- `walk::cwd()` becomes **public and normalizing**, returning `normalize_abs` of
  `std::env::current_dir()`. It is private today with exactly one caller,
  `kit_roots_abs`, which normalizes downstream through `abs_against` — so there is
  no live hole, and promoting it to the crate's producer is precisely what moves
  the obligation to the source.
- `walk::toplevel()` is **new**: the `git rev-parse --show-toplevel` spawn with its
  answer passed through `normalize_abs` on the way out. Git-for-Windows is a native
  Windows binary and answers in Windows spelling even to a Rust process, so this
  producer crosses for the same reason `cwd()` does.
- `fresh::toplevel()` keeps its name, its error text and both its callers, and
  becomes a caller of `walk::toplevel()` rather than a second spawn. Its emitter-anchor
  semantics are unchanged; only the producer moves.
- **`stage_evidence.rs`' `norm()` is retired onto `walk::normalize_abs`.** It is a
  second implementation of the crate's normalizer that splits on `'/'` only, so it
  cannot repair a backslash-spelled root — a normalizer that silently does nothing
  is worse than none, and the crate's single-owner rule is exactly what forbids it.
  This is the one delta-3 change that is a deletion rather than a move.

Every other crate site binding a producer migrates onto `walk::cwd()` /
`walk::toplevel()`. The crate's declared dialect stays drive-preserving under
delta 1, so `normalize_abs` itself needs no change and this delta adds no new
normalization rule anywhere.

**Corrected at build, three defects the sweep itself surfaced** (the fourth,
fifth and sixth this amendment has carried; ruled in-envelope, `lead 2026-08-30
own-authority` for the batch boundary that exposed them).

1. **The producer count is 25, not the 27 §The census records** — and 23, not
   "nine plus fourteen", once `fresh.rs`'s own spawn is counted where it belongs.
   Outside `walk.rs` at `14b7994b`: 9 `std::env::current_dir()` (7 shipping, 2 in
   `gates/mod.rs` under `#[cfg(test)]`), 14 `--show-toplevel` spawns **including**
   `fresh.rs`'s, and 2 `std::fs::canonicalize()`. The 2-off is `main.rs`'s
   `.expect()` and `assert!()` *message strings*, which name the producer in prose
   — a grep counts them and a producer census must not. Same error class as
   delta 5's comment-stripping rule, one file earlier.
2. **`std::fs::canonicalize()` had no named producer.** Delta 3's rule names three
   forms but §Producers and consumers gives functions for two. `walk::canonicalize()`
   is the third, and it is the one producer that **does not convert**: on Windows
   that call answers in the extended-length spelling `\\?\C:\repo`, which
   `path_root` reads as separator-rooted and `normalize_abs` would mangle to
   `/?/C:/repo`. Both callers compare its output only against its own, so the
   asymmetry is unobservable; the verdict and the open question are recorded in
   §The crate's crosser and the gap is filed.
3. **Two spawn sites take `-C <dir>`**, which no delta anticipated:
   `stage_evidence.rs` compares a `-C` answer against a bare one — a root-to-root
   string comparison, the exact shape delta 1 warns of — and `main.rs`'s test
   anchors on `CARGO_MANIFEST_DIR`. `walk::toplevel_in(dir)` crosses both sides.
   `walk::toplevel_opt()` is the same producer with its two refusals kept apart,
   for the five callers that report a dead `git` differently from a directory
   outside a work tree.

The `--git-dir` and `--git-common-dir` occurrences are **not** delta-3 producers
and were probed rather than assumed: all 20 in the crate read `stdout().is_some()`
and discard the value, so each is a repository-presence probe that yields no root.
They remain in delta 5's scan roster, which is stated here because that gate will
meet them.

### (4) The corpus migrates onto the two idioms — every producer, not only the exposed ones

Every producer occurrence in the census below crosses, **including the sites the
contract's consumption predicate judges dialect-tolerant**. This overrides the
three "dialect-tolerant; no change owed" dispositions §Worked dispositions records
for drift-kit's `kfric.sh`, `overhead-meter.sh` and `stage-economics.sh`, and the
override is the deliberate content of this delta. {mechanical}

**The census, re-derived at this amendment's rev.** The producer set is four forms,
not the one the queue entry's figure counts: `git rev-parse --show-toplevel` /
`--git-dir` / `--git-common-dir`, `std::env::current_dir()`,
`std::fs::canonicalize()`, and `env!("CARGO_MANIFEST_DIR")`.

- **Shell — 33 occurrences across 32 files.** 16 dialect-tolerant, 17
  dialect-exposed, and **all 17 uncrossed**: not one exposed shell site converts at
  its point of production today.
- **Rust — 27 occurrences across 22 files**, outside `walk.rs`. 13 are
  dialect-exposed and uncrossed. A further 8 `env!("CARGO_MANIFEST_DIR")` sites
  clear by construction under delta 5's `Path`-wrap rule and are not counted here.
- **60 occurrences across 54 files; 30 live violations.**

The queue entry's `show-toplevel` grep figure and this one are different measures
over different corpora, and neither supersedes the other — this one is the
migration's extent, that one is a single producer's grep count.

**Why every producer and not only the 30.** The consumption predicate is an
**audit** tool and a good one: it lets a reader judge a site that already exists.
As a **migration target** it costs a recorded verdict per site, because
§Worked dispositions is right that a judged-safe site and an unjudged one look
identical — which is 60 verdicts in prose, a roster that goes stale on every ported
file. Crossing at every producer retires the question instead: with no uncrossed
value anywhere, consumption never has to be judged, no roster has to be
maintained, and delta 5's gate reads one line at each producer rather than tracing
a variable through a file and out through its callees.

The census produced that argument's decisive evidence — **consumption is not in
fact always local.** `installer/lib/common/lock.sh`'s `lock_path()` concatenates
whatever root it is handed and is the *sole* exposure path for
`installer/lib/update.sh`, whose only use of its root is to pass it there; the same
shape recurs in the crate at `scan_root`'s three callers and `fresh::toplevel`'s
two. So the judging predicate's claim that "consumption is local and decidable by
reading one line" is corrected to what is true: consumption is local **within a
file**, and a root handed to a shared helper is judged at the helper. Under
producer discipline that correction costs nothing, because no site is judged by
consumption at all.

Three mechanical constraints on the sweep, each of which will bite a session that
does not know it. Template/copy pairs move together or `check-template-copy-parity`
reds — `context-kit/templates/session-context.sh` and `scripts/session-context.sh`
are such a pair in this corpus. The **six** shell sites that consume one root by
*both* `cd` and concatenation take the two-line form, never the subshell form,
because the `cd` is load-bearing at those sites. And every site's variable keeps
its name and every use keeps its text: this delta changes how a value is computed
and nothing about what is done with it, which is what makes it mechanical.

**One non-target, stated so the silence is not read as an omission.**
`check-memory-off` derives the harness's per-project directory by folding a root's
`/` and `.` to `-` (`memory_off.rs`, and its two `tr '/.' '-'` shell twins in the
`session-context.sh` pair). That derivation compares a *this-tree* root against a
*harness-owned* directory name, so it is the one place delta 1's two spellings
would meet — and which spelling the harness uses on Windows is a fact no surface
here owns and no command on a Linux host can probe. The rule's owner is
context-kit, not gate-sdk. Those three sites therefore take a recorded `spec:`
verdict naming the open question, the migration normalizes their producer like any
other, and the gap is filed rather than guessed. This is not a regression: the fold
already recognizes nothing in a backslash-spelled root today.

### (5) `check-path-dialect`, born native, is the record that replaces the roster

A new gate — a Rust module plus a `.gate` descriptor and a `good/`+`bad/` fixture
pair, registered in `scripts/gates.list`. Without it this migration decays inside
its own iteration: the contract already states that every newly ported file adds a
site to the remainder, and the port is still running. {design-bearing}

**Predicate.** Over tracked shell sources (`walk::tracked_shell_tree`, already the
corpus two other arms use) and `native/src/**/*.rs`, every occurrence of a
producer in delta 4's four-form roster is red unless one of these holds:

- **shell, crossing position** — the substitution is the direct argument of a `cd`
  whose result is read back with `pwd -P`;
- **Rust, inside `native/src/walk.rs`** — the declared crosser's own body;
- **Rust, `Path`-typed** — the occurrence is the direct argument of `Path::new(`
  or `PathBuf::from(`, so the value never becomes a string and `std::path` carries
  the dialect. This is §Porting to Rust does not retire dialect exposure read
  positively, and it is what clears the eight `env!("CARGO_MANIFEST_DIR")` sites
  with no edit;
- **a recorded verdict** — an adjacent `spec:` comment citing
  `§The path-dialect contract`, for a site that deliberately does not cross.

A call to `walk::cwd` or `walk::toplevel` is not an occurrence of a producer at
all; it is named here only so a reader does not hunt for an exemption that is not
needed.

**The exemption needs no new comment grammar.** It reuses canon-kit's `spec:`
one-line binding, which already carries a mandatory cited section and is already
gate-read on both sides; `walk.rs` writes exactly this form at three sites today. A
dedicated tag class was considered and refused — it would put the verdict in a
second grammar for no error class the existing one does not catch, and would make
this a two-component amendment for a naming convenience.

**A producer named inside a comment is not an occurrence.** This is a correctness
requirement, not a nicety, though the nearest examples in this tree are not live
counterexamples: `gate-sdk/gate-tests/check-graph-tree.test.sh` and
`gate-sdk/gate-tests/check-template-copy-parity.test.sh` each name
`git rev-parse --show-toplevel` in a header comment explaining what their fixture
pair cannot reach, and `walk::tracked_shell_tree` already excludes both — by its
`*.test.sh` filename filter and by `gate-tests` sitting in the default
`GATE_PRUNE_DIRS` (`gate-sdk/lib/gate.sh`) — so neither reaches the scanner
today. The exclusion is corpus configuration, not a property of comments, and a
producer named in prose in any file the corpus *does* reach would false-positive
with no such rule. The gate strips comments before scanning on that ground, and
the `bad/` fixture carries a commented producer as a case that must stay green.

**Fail-closed, and the honest limit.** The gate's verdict is a syntactic property
of the source, fully exercisable on any host, so its fixture pair is an ordinary
one and no injected input is owed *for the gate*. What stays unexercisable is the
premise underneath — that `getcwd(3)` answers in the MSYS spelling and `git
rev-parse` does not. That is the same injected premise §How the claim is held
already declares, and this gate **inherits it rather than discharging it**: a green
`check-path-dialect` asserts that every producer crosses, never that the crossing
works.

### (6) §Worked dispositions keeps its teaching and drops its roster

The section's per-site enumeration is retired in favour of the at-site record
delta 5 makes mandatory, and its closing paragraph — "The tree's remaining root
call sites are **not** migrated onto this contract yet" — becomes false and is
replaced by the gate's name and what a green verdict does and does not assert.
{design-bearing}

What is kept is what the section is for: the *exposed-but-satisfied* verdict, the
one a reader most needs a worked instance of. The retired `drift-report.sh`/`KIT`
example loses nothing by being joined, and is joined, by a live one the census
found — `scripts/producer-liveness-reader.sh`, which crosses `git rev-parse
--git-common-dir` inline with `pwd -P` and then concatenates the result. Same
verdict, on a site that still exists, over a producer the section never named.

## Producers and consumers

**`walk::cwd()`, public and normalizing — delta 3.** Producer: any crate path
needing the process's directory; the function *is* the boundary crossing, wrapping
`std::env::current_dir()` in `normalize_abs`. Consumers: `walk::kit_roots_abs`
(unchanged caller) and every crate site migrated off a raw `current_dir()` in
delta 3's sweep — nine occurrences, seven of them shipping. It sits on the default
path of every gate that resolves a kit root, i.e. the whole battery, so it is
reachable in deployed configuration and not only under test. No new field.

**`walk::toplevel()` — delta 3.** Producer: the `git rev-parse --show-toplevel`
spawn, the crate's only one after this amendment. Consumers: `fresh::toplevel()`
and the fourteen crate occurrences migrated off their own spawns. Its `Err` arm is
the existing "not a git repository" refusal relocated rather than invented, and its
reader is each caller's existing error path, so no caller gains a failure mode it
does not already handle. No new field.

**The shell crossing idiom — delta 2.** Producer: `git rev-parse` under a `cd`,
read back by `pwd -P`. Consumer: the binding variable at each site, unchanged in
name and unchanged in every use. It introduces **no interface** — it is a different
way of computing a value the site already had, which is why delta 4 is mechanical
and why nothing downstream of any site needs updating.

**`check-path-dialect` — delta 5.** Producer: registration in `scripts/gates.list`
plus `gate-sdk/checks/check-path-dialect.gate`, which is what puts it on the battery
and, through its `tier=` declaration and `graph:` manifest, into the generated
`scripts/git-hooks/pre-commit`. Its enabling config is the descriptor itself —
zero-config like every gate here, so there is no knob a deployed configuration
could fail to set and no test-only reachability. Consumers: `bin/run-gates.sh` at
battery time, the generated pre-commit hook at commit time, and the `gates`
workflow at push. **Red condition, named rather than described:** a producer
occurrence, outside a comment, that is neither in crossing position nor `Path`-typed
nor carrying a `spec:` citation to §The path-dialect contract. Its verdict is
**monotone in the violation set** — it reds on finding a violation, never on
finding none, asserts no count, and holds no coverage floor.

**The `spec:` citation as a recorded verdict — delta 5.** Producer: an author who
writes it at a non-crossing site. Consumers, two of them: `check-path-dialect`'s
scanner, which reads it at scan time to clear the site, and canon-kit's
`check-comment-tier`, which is **unchanged** — the citation is a real `spec:`
binding to a real section, exactly what that gate already requires, so this
amendment adds no case it does not already handle. Field readers: the cited section
name is the only field and both consumers read it, the gate to clear the site and a
human reader to reach the rule.

**Readers under delta 6's narrowing.** Delta 6 removes prose from
`gate-sdk/SPEC.md`, so each reader's red condition is named rather than its subject.
`check-md-refs` reds on a reference resolving to nothing; `check-spec-pointer` reds
on a pointer whose target section is absent. Both are cleared by one assertion made
once here rather than re-derived per reader: **this amendment renames no heading**,
so every `§`-citation in `walk.rs` and in §Layout and configuration keeps its
target. `check-surface-duplication` reds on a second copy of a gated fact, and a
removal can only reduce copies — monotone. `check-docs-mirror-fresh` reds on a byte
difference rather than on a count, and is cleared by the regeneration named below.
`check-comment-tier` reds on a `spec:` comment whose cited section does not resolve
— same heading-freeze argument, and delta 5 *adds* citations to a section that
exists rather than removing any.

## Existing sections updated

- §The path-dialect contract, **The declared dialect** — replaced by the
  per-substrate clause, with the relative-transport rule stated alongside it
  (delta 1).
- §The path-dialect contract, **The boundary, and who crosses it** — the
  `cd … && pwd` sentence corrected to `pwd -P`, with the reason bash's logical `cd`
  does not convert (delta 2).
- §The path-dialect contract, **The judging predicate** — "consumption is local and
  decidable by reading one line" corrected to hold within a file, with the
  shared-helper case named (delta 4).
- §The crate's crosser — gains `walk::cwd()`'s promotion, `walk::toplevel()`, and
  the producer-monopoly rule; records `norm()`'s retirement so a later reader knows
  a second normalizer existed and why it went (delta 3).
- §How the claim is held, with no oracle that can run it — gains the sentence that
  `check-path-dialect` asserts crossing and never the crossing's effect, so a green
  board is still not a Windows run (delta 5).
- §Worked dispositions — roster retired, teaching example kept and joined by
  `producer-liveness-reader.sh`, closing paragraph replaced (deltas 4 and 6).
- §Per-component contracts — a **new** `check-path-dialect` subsection carrying the
  predicate, the four clearance rules, the comment-stripping requirement and the
  fixture pair (delta 5).
- `scripts/gates.list` and `gate-sdk/checks/check-path-dialect.gate` — registration
  and manifest; the `graph:` manifest edit regenerates `scripts/git-hooks/pre-commit`
  (delta 5).
<!-- update-target-exempt: a generated projection mirrors whatever the deltas land, so naming one delta would go stale the moment a second touches the same source -->
- The generated projections a SPEC edit and a new gate stale — the on-site mirror
  `docs/gate-sdk/SPEC.md`, and the enforcement and value rollups a gate count moves.
  Each freshness gate prints its own regen command on red; the mirror's is
  `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write`.

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
