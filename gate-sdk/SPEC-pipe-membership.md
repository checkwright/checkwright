# SPEC amendment: pipe-membership

§run-gates states the rule: a short-circuiting consumer may not be fed by a producer
inside the same gate. The membership idiom
`printf '%s\n' "${set[@]}" | grep -q …` under `set -o pipefail` reads a present
member as absent, because the abandoned writer's `SIGPIPE` becomes the pipeline's
status. The rule is stated, and it has still recurred. Five sites were fixed at once
after a sweep had counted two. **This amendment adds the gate, born native, and rules
the predicate that separates the idiom from a deliberate early-exit pipe without
flooding.**

**The predicate is keyed on the producer, and that is what keeps it narrow
(measured 2026-09-21).** 221 of 289 tracked shell files set `pipefail`. A
structural scan for any pipe into `grep -q`, `grep -m` or `head` in those files finds
31 such pipes, 15 of them in a consumed position. Each of the 15 is safe or benign.
Its producer writes at most one line (`find … -print -quit`, `head -1`), or reads a
small fixed input (a smoke block, a fixture log), or its exit status is never the
verdict. A structural gate would red all fifteen and teach the tree to ignore it. What
the attested sites shared is the **producer**: an in-shell write loop over a set,
issuing one `write(2)` per element. That is what keeps writing after the consumer
has exited, and it is why the onset is a roster size (between 400 and 800
single-token members) rather than a race on every run. A pipe whose producer is an
array expansion or a loop is the membership idiom, because membership is the only
reason to stream a set into a short-circuiting reader.
`git grep -n -E '(printf|echo)[^|]*\[[@*]\][^|]*\|\s*(grep|head)|done\s*\|\s*(grep|head)' -- '*.sh'`
returns no site, so the gate lands green. The five array-producer pipes the tree does
carry all feed a full consumer (`paste`, `sort`, `git hash-object`), which the
predicate leaves alone.

**Why a gate, when the corpus is empty and shrinking.** The sweep that counted two
while five existed is the evidence that review does not hold this. Shell is the
shrinking substrate, but 39 shell files are declared `no-port`
(`--emit port-blockers --tree`), smoke and test libraries among them, and those are
where the membership idiom lives. The cost of the defect is a false absence reported
as a real parity finding. That cost is paid by exactly the gates that report parity.

## What changes

### (1) `check-pipe-membership` holds the rule over the tracked shell tree {design-bearing}

**Not yet applied.** It is a new gate-sdk gate on the native substrate: a Rust module,
`gate-sdk/checks/check-pipe-membership.gate`, a `good/`+`bad/` pair, `tier=precommit`
and `# install: zero-config`.

- **Corpus.** `walk::tracked_shell_tree`, the corpus and prune set
  `check-path-dialect` already walks, with the same code-and-comment split.
- **A file is in scope** when its code sets `pipefail`: `set -o pipefail`, or a
  combined `set -…o pipefail` flag word, anywhere in the file.
- **Red** on a pipeline whose last stage is a short-circuiting reader and whose
  first stage is a set producer. A short-circuiting reader is `grep` carrying `-q`,
  `--quiet`, `--silent`, `-m`/`--max-count`, or a short flag cluster holding `q` or
  `m`, or `head`. A set producer is `printf` or `echo` with an array expansion
  (`[@]` or `[*]`) among its arguments, or a loop whose `done` feeds the pipe.
  Whether the status is consumed is not asked, since a membership pipe exists to be
  tested.
- **Finding text** names the file and line, and gives the remedy the rule already
  states: membership is a `for` loop over the set, or a `[[ ]]` test, never a
  pipeline.
- **No valve.** A deliberate early-exit pipe is outside the predicate by
  construction, because its producer is not a set. A site that needs a set streamed
  into a short-circuiting reader has the loop form available.
- **Honest limit, in the section.** A producer that is a command reading a large
  file or roster, such as `grep -Ev … "$roster" | grep -qxF …`, passes. The live
  instance of that shape, `installer/bin/checkwright.sh:113`, runs under `/bin/sh`
  with no `pipefail`, so it is out of the hazard's precondition.

The fixture pair: `bad/` carries a `printf` array pipe into `grep -q`, a `done |`
pipe into `head -1` and an `echo "${a[*]}" | grep -m1`, all under `set -euo
pipefail`, and asserts the count. `good/` carries the same array pipe with no
`pipefail`, an array pipe into `sort`, a `find -print -quit | grep -q .`, and a
`for`-loop membership.

### (2) §run-gates points at the gate, and the gate's section states the predicate {mechanical}

**Not yet applied.** In §run-gates, the paragraph **What the pool actually exposed was
a different class** ends with a sentence saying the rule is held by
`check-pipe-membership` (§check-pipe-membership). A new §check-pipe-membership,
beside §check-path-dialect, states delta 1's corpus, predicate, measured grounds and
honest limit.

### (3) The gate is registered and rostered {mechanical}

**Not yet applied.** `scripts/gates.list` registers it. `gate-sdk/smoke/install.sh`
registers it, as assertion B of `check-install-disposition` requires for a
`zero-config` member. gate-sdk/README.md's `gate-roster` block gains the line. The
crate's `REGISTRY` gains the row. The descriptor's `couples=` names the shell tree
exactly as `check-path-dialect.gate`'s does at the moment of landing, with
`trigger=*`, since the corpus is the whole tracked shell tree.

## Producers and consumers

- **The gate (deltas 1 and 3).** Producer: the battery, at `precommit`, and a
  consumer's `init`, via `zero-config`. Consumers, each a roster-holding reader:
  - `scripts/gates.list` and the generated hook;
  - `check-readme-roster`, satisfied by the README line;
  - `check-install-disposition`, whose assertion A takes the `# install:` line and
    whose assertion B takes the smoke registration;
  - `check-gate-fixture-coverage`, satisfied by the pair;
  - `check-gate-substrate-parity`, satisfied by the descriptor plus the `REGISTRY`
    row;
  - the consumer smoke's accounting, which finds it registered;
  - `docs/enforcement.md` and `docs/check-graph.html`, regenerated.
  **Release declaration:** build adds a Tightened-gates bullet for the new gate to
  `.workflow/release-declarations.md`.
- **Point 5.** Nothing narrows.
- **Point 6.** The gate obliges every in-scope pipe, and the probe above enumerates
  none. The corpus has no member to satisfy at landing.

## Existing sections updated

Rosters from the two `git grep` probes above, the pipefail-file count
(`git ls-files '*.sh'` filtered by a `pipefail` set line), and reading §run-gates.

- gate-sdk/SPEC.md §run-gates and a new §check-pipe-membership (delta 2).
- The new module, `gate-sdk/checks/check-pipe-membership.gate` and
  `gate-sdk/gate-tests/check-pipe-membership/` (delta 1).
- `scripts/gates.list`, `gate-sdk/smoke/install.sh`, gate-sdk/README.md,
  `native/src/gates/mod.rs` and `.workflow/release-declarations.md` (delta 3).
<!-- update-target-exempt: generated projections, regenerated by their freshness gates' printed commands -->
- `docs/gate-sdk/SPEC.md`, `docs/gate-sdk/README.md`, `docs/enforcement.md`,
  `docs/check-graph.html`, `scripts/git-hooks/pre-commit`.

## Retired spellings

- None — no name is removed.

## Definition of Done

- [ ] **Causal completeness.** Every point of canon-kit/SPEC.md §The
      causal-completeness check holds for the gate.
- [ ] **Instruction surfaces: instruction only.** Not reached.
- [ ] **Merged with no information lost.** §run-gates keeps the class's history, and
      the new section owns the predicate.
- [ ] **Amendment deleted.** This file is removed on merge (`ls gate-sdk/SPEC-*.md`).
- [ ] **Entry moved.** `pipeline-membership-idiom-latent` moves to Done in the merge
      commit, at a stage before the drain stage.
- [ ] **Fails closed.** `bad/` reds on all three producer shapes with the count
      asserted, and an unreadable tracked file is exit 2.
- [ ] **Removals propagated.** `check-amendment-retired-spelling` runs the block
      above.
- [ ] **Gaps filed.** Any cross-component gap build discovers is resolved that
      session.
