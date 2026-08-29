# SPEC amendment: the path-dialect contract

Rules `msys-path-dialect-boundary-unmodelled`. Ruled by the lead on 2026-08-29
(`lead 2026-08-29 own-authority`): the contract, **plus the one-function repair**
the diagnosis below locates. That is not a widening — the bound was always "the
contract plus dialect-correct root resolution for the sites the cut writes or
touches", and the defect turns out to sit *in* root resolution. A falsified
diagnosis does not shrink a unit to the shape of its error.

**Home.** gate-sdk, and the choice is argued rather than defaulted. No SPEC in
the tree owns cross-kit root resolution today; the only normative sentence
anywhere is gate-sdk/SPEC.md §Layout and configuration's *"paths are
repo-root-relative; every entry point cd's to `git rev-parse --show-toplevel`"*,
which states shape and mechanism and never **dialect**. gate-sdk also already
carries the tree's only platform reasoning — the `on_path`/`PATHEXT` work and its
cannot-exercise-locally doctrine — and already owns the single dialect-dispatching
helper (`gate_exe_suffix`, which switches on `uname -s` over
`MINGW*|MSYS*|CYGWIN*|Windows_NT` and is declared single-owner). The contract
belongs beside that reasoning, not in a new home.

## The diagnosis, corrected — the entry's is falsified

The entry records two candidate mechanisms (npm's MSYS bin shim; the toplevel
probe under Git-for-Windows) and says plainly that **neither** explains the
observed spelling, which carries a POSIX leading slash *and* backslashes, "so a
third step is unaccounted for". The third step is found, and it is not in shell.

Read off the current tree, the chain reproduces the observed string
character-for-character:

1. `native/src/walk.rs` — `cwd()` is `std::env::current_dir()`. The Windows CI
   leg's binary is `x86_64-pc-windows-msvc`, so this returns a **backslash**
   spelling, `D:\a\_temp\…`.
2. `abs_against` tests absoluteness with `starts_with('/')` — a **POSIX-only**
   predicate. A drive-lettered path answers *false*, so a root that is in fact
   absolute takes the *join-onto-cwd* arm.
3. `normalize_abs` splits on `'/'` **only**, so backslash segments are never
   split, and then composes its result as `format!("/{}", …)` — an
   **unconditional POSIX leading slash**.
4. `registry.rs` appends the `/checks` segment, and every gate resolves to
   nothing.

**The shell side behaved correctly and is exonerated.** `gate-sdk/lib/gate.sh`
deliberately derives its kit roots *relative* to `$PWD` precisely so no absolute
path is baked into the tracked hook. The Windows spelling entered on the **Rust**
side. Neither recorded candidate reproduces the leading slash; this does.

Delta 6 lands this correction on the entry itself, naming both superseded
candidates, so the record shows a premise killed by a probe rather than quietly
replaced.

## What changes

### (1) The path-dialect contract

A new section in gate-sdk/SPEC.md stating the three things nothing states today —
**which dialect a root is in, where the boundary is crossed, and who crosses it**
— so that a call site can be judged right or wrong at all. {design-bearing}

The contract's three clauses:

- **The declared dialect.** Every root variable in this tree is **POSIX-spelled**:
  forward separators, no drive letter. That is the invariant, and stating it is
  what converts every call site from unjudgeable to judgeable in one sentence.
- **The boundary and its crosser.** The dialect boundary is crossed exactly where
  a value enters from a **platform-native producer** — `git rev-parse
  --show-toplevel` under Git-for-Windows, `std::env::current_dir()` on a
  windows-msvc binary, and an npm bin shim's basedir. The **crosser** is the
  entry point that reads such a producer, and normalizing at that point is the
  crosser's obligation. A value already inside the tree is POSIX by the clause
  above and is never re-normalized — a second normalization is how a contract
  becomes a ritual.
- **The judging predicate, which is about consumption rather than about the
  producer.** This is the clause that makes the contract usable, and it is the
  one a reader gets wrong:

  > **A root consumed only by `cd` is dialect-tolerant. A root consumed by
  > string concatenation is dialect-exposed.**

  `cd` accepts either spelling on an MSYS host; `"$ROOT/sub"` does not. So the
  audit question at a call site is never "where did this root come from" but
  "what is done with it".

  **A `|| pwd` fallback confers nothing, and believing otherwise is the trap.**
  It fires only when `git` *fails*; on MSYS `git` **succeeds**, in the wrong
  dialect. The fallback is not a dialect guard and must not be read as one — which
  is why the `gate-tests/` bucket does **not** fall out by construction the way
  `msys-dialect-migration` allows it might: roughly a third of those sites
  concatenate.

### (2) Porting to Rust does not retire dialect exposure

A first-class clause of the contract, not a note. {design-bearing}

The intuitive inference — *shell is the dialect-fragile substrate, so a ported
file's dialect problem goes away* — is **false in this crate**, and this defect is
the proof: the port did not inherit the exposure, it **created** it.
`native/src/walk.rs` composes paths with `String`, `split('/')` and `format!`
rather than with `Path`/`PathBuf`, so it re-implements POSIX assumptions that
`std::path` would have handled. Rust is dialect-safe only where `Path` is
actually used.

This clause is stated here and **cited from the anchor's amendment**, because the
reader who most needs it is the build session porting the next kit — and left
implicit it is re-derived at full cost by whoever ports after them. It is also why
the corpus in `msys-dialect-migration` does not simply shrink as the port
advances: that entry's own cost line says the Rust half **grows**, and this clause
is the reason it grows rather than the reason it shrinks.

### (3) The repair in `native/src/walk.rs`

`abs_against` and `normalize_abs` become dialect-correct: absoluteness recognizes
a drive-lettered root, segment splitting recognizes both separators, and the
composed result carries the input's own root rather than an unconditional POSIX
slash. {design-bearing}

Scoped deliberately as a **pure-function** repair, which is what keeps it inside
the ruled envelope: the change is to how two functions map inputs to outputs, and
nothing about when they are called, what calls them, or what a root *means*
elsewhere moves. The migration of the other call sites remains
`msys-dialect-migration`'s and is untouched.

### (4) The holding mechanism, since no oracle can exercise this locally

The claim is held by a **pure function over injected inputs**, plus a labelled
control that fails if the assertion would pass vacuously. {design-bearing}

This reuses the `on_path` mechanism, and the entry's description of that
precedent is **wrong in a way that matters**: it says "a fixture pair plus a
labelled reasoned-from-shape arm", and the `on_path` unit shipped **no fixture
pair at all**. What it actually did, and what this delta copies:

- the unexercisable decision is factored into a **pure function of its inputs**
  with the platform-dependent part **injected** — `on_path` injects an existence
  predicate, so the decision is testable on any host;
- the "label" is a three-part construct rather than a marker token: a test name
  spelled as a **sentence** stating the claim, a `spec:` comment above it stating
  the **honest limit**, and an assertion that is a **source scan of the crate's
  own text** where no behavioural call could run;
- every reasoned-from-shape assertion is **paired with a control** that fails if
  the test would pass vacuously;
- the SPEC carries the honest limit in prose, so a green board never reads as a
  Windows run.

**There is no CI oracle and the amendment says so rather than implying one.** The
Windows leg runs the installer smoke only, `continue-on-error`, and dies before
the battery runs; the Ubuntu leg merely cross-compiles. So the arm must be local
and host-independent — which is exactly what the injected-predicate shape buys.

### (5) The four drift-kit `bin/` tools, dispositioned rather than swept

The envelope's named call sites, each judged by delta 1's predicate and recorded
with its disposition. {mechanical}

- `kfric.sh`, `overhead-meter.sh`, `stage-economics.sh` — each resolves one
  `REPO_ROOT` and consumes it **exactly once, on the next line, with `cd`**, never
  concatenating. **Dialect-tolerant already; no change is owed.** Recording that
  verdict *is* the deliverable here — an unjudged site and a judged-safe site look
  identical, and the contract's value is the difference.
- `drift-report.sh` — carries a **second** root, `KIT`, derived from
  `BASH_SOURCE` and consumed at line 11 as `"$KIT/../gate-sdk/lib/gate.sh"`. That
  is a **real concatenation site**, inside the envelope, and the only one among
  the four. It is also the one file the anchor ports, so see the note below.

**The interaction with the anchor, stated so no batch discovers it.** The anchor
ports `drift-report.sh` into a crate arm, which retires its shell `KIT` root
entirely. Whichever lands first, the other must not re-fix or re-break it: if the
port lands first this site vanishes and delta 5 records that it did; if this lands
first the repair is superseded by the port and the record says so. Delta 2 is why
the port is not automatically the safer of the two.

### (6) The entry's diagnosis corrected, in the same commit

`msys-path-dialect-boundary-unmodelled`'s body gains the located third step and
names both superseded candidate mechanisms. {mechanical} A second correction is
owed on the sibling: `msys-dialect-migration` glosses "eight roots in shell alone"
as a count of root *variables*, and it is a count of **top-level directories** —
the root-variable count is four, and the call-site census re-derives to 48 real
sites of 52 occurrences.

## Producers and consumers

**The new state is one contract clause set plus one corrected pure function.** No
message, no field, no file, no knob — so the field walk is short by construction.

- **Producer, named and reachable:** the crosser named in delta 1 — concretely,
  `walk.rs`'s `cwd()`/`abs_against` pair, reached on **every** invocation of the
  binary that resolves a root, which is every gate run. No enabling config gates
  it: there is no configuration in which the binary resolves roots and this path
  does not run. The contract's *other* producers (`gate.sh`'s root derivation, the
  installer's entry points) are named as crossers by delta 1 and migrated by
  `msys-dialect-migration`, not here.
- **Consumer, named, by what mechanism:** `native/src/registry.rs`, which appends
  the `checks` segment to each resolved root and enumerates descriptors from it —
  the exact consumer whose failure produced the observed all-gates-unresolved run.
  Downstream of it, every gate dispatched from that registry.
- **Second consumer, at a different transition:** the **build session porting the
  next kit**, reading delta 2 at the moment it assumes a port retires a dialect
  problem. Its mechanism is the citation the anchor's amendment carries; that
  citation is the delivery, and without it the clause has a reader with no path
  to it.
- **Third consumer:** `msys-dialect-migration`, which is unpickable until the
  contract exists (`[blocked-by:]`) and whose per-site judgment is delta 1's
  predicate. The contract is that entry's enabling input.

**Red conditions, named rather than subjects.** No delta narrows a corpus; delta 3
changes a shared pure function's outputs, which reaches every reader of a resolved
root, so each is enumerated by what makes it red:

- **The crate's own test suite** (`cargo test`, run through `check-crate-arms`) —
  reds on any assertion over `abs_against`/`normalize_abs` outputs. **Non-monotone
  and the sharpest reader here**: every existing test that pinned the *old*
  POSIX-only composition reds on the corrected one, and those reds are the repair
  working rather than a regression. Re-derived by running, never cleared by
  inspection.
- **`check-gate-binary-fresh`** — reds when the committed binary is older than the
  crate sources. Fires on delta 3 and is discharged only by
  `bash gate-sdk/bin/build-native.sh`; the battery does **not** discharge it.
- **The whole gate battery** — every gate resolves through the repaired path, so
  a mistake here reds broadly rather than narrowly. That is a property worth
  stating: this function has no small blast radius, which is the argument for the
  pure-function scoping in delta 3.
- **`check-docs-mirror-fresh`** — reds on a byte difference between
  gate-sdk/SPEC.md and its `docs/` mirror; fires on deltas 1, 2 and 4, clears on
  regeneration.
- **`check-comment-tier`** — reds on a non-directive comment; delta 4's `spec:`
  line and sentence-named test are directives. Monotone.
- **`check-queue-wrap`, `check-tag-lead-line`, `check-amendment-queue`** — red on
  the promotion's own shape. Monotone; cleared by the battery at commit.

## Existing sections updated

- gate-sdk/SPEC.md §Layout and configuration — its repo-root-relative sentence is
  the tree's only normative statement about roots and now points at the contract
  for the dialect half it does not carry (delta 1).
- gate-sdk/SPEC.md §Fail-closed contract and the `on_path` reasoning beside it —
  the cannot-exercise-locally doctrine gains this unit as its second instance, and
  its description of the holding mechanism is the one delta 4 reuses, so the two
  are stated once and cited rather than restated (delta 4).
- gate-sdk/SPEC.md — `gate_exe_suffix`'s single-owner declaration, which is the
  tree's existing dialect-dispatching helper and now sits under a contract that
  explains what it is dispatching on (delta 1).
- `native/src/walk.rs` and its tests (deltas 3, 4).
- TASK-QUEUE.md — `msys-path-dialect-boundary-unmodelled`'s diagnosis and
  `msys-dialect-migration`'s roots-versus-sites gloss (delta 6).
- `docs/gate-sdk/SPEC.md` — the generated mirror, stale the moment any delta lands
  (`all deltas`); regenerated by the command `check-docs-mirror-fresh` prints on
  red, rostered in docs/site-architecture.md §Generated projections.

<!-- update-target-exempt: the general per-site migration is msys-dialect-migration's whole deliverable and is out of this envelope by the lead's ruling, so it is owned by no delta here -->
- The ~44 remaining root call sites — **not migrated by this amendment**; they are
  `msys-dialect-migration`'s, which this contract unblocks.

## Definition of Done

- [ ] **Causal completeness** — the contract's crosser is named and reachable on
      every root resolution, and three consumers are named at three transitions,
      one of them the next kit's build session reading delta 2.
- [ ] **Merged with no information lost** — the contract lands as its own section
      beside gate-sdk's existing platform reasoning, not appended; delta 2 lands
      as a clause rather than a parenthetical.
- [ ] **Amendment deleted** — this file removed on merge; none remain for gate-sdk
      (`ls gate-sdk/SPEC-*.md`), discharged at the iteration where a sibling is in
      flight.
- [ ] **Removals propagated** — the entry's two superseded candidate mechanisms
      marked superseded rather than deleted, and the roots-versus-sites gloss
      corrected on the sibling entry.
- [ ] **The repair held by a control, not only by an assertion** — the
      reasoned-from-shape arm has its paired vacuity guard, and the SPEC carries
      the honest limit in prose so a green board is never read as a Windows run.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed to
      the gap inbox.
