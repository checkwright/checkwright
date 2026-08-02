# SPEC amendment: fixture-expect-conjunction

Queue entry: **`gate-fixture-expect-conjunction`**.

`run-gate-tests.sh` matches a case's `expect.txt` against the gate's output with
`grep -qF -- "$expect"`. `grep -F` splits a multi-line pattern into **separate
fixed strings and matches any of them**, so a two-line expect is a disjunction:
the case passes when *either* line appears. An author who wrote two lines to pin
two findings pinned one, and nothing anywhere says otherwise —
gate-sdk/SPEC.md §run-gate-tests says the case must "print its substring",
singular, and never states the limit.

This is the fixture-corpus member of the vacuous-green set, and it is the worst
shape in it: the other units make a *gate's* green mean less than it claims, this
one makes the **fixture pair** — the thing that proves a gate works at all —
assert less than its author wrote down. A pair that under-asserts is worse than
an absent one, because it reads as coverage.

## What changes

### 1. A multi-line expect is a conjunction {design-bearing}

`run_case`'s single `grep -qF` becomes a per-line match: every **non-blank** line
of `expect.txt` must appear literally in the case's combined output, and the case
fails when any one of them does not. Blank lines are separators, not assertions —
a blank line matches everything under `grep -F`, so treating it as an assertion
would assert nothing while looking like a fourth pin.

The semantics deliberately stay **order-independent** and **substring-per-line**.
Each line keeps exactly the meaning a one-line expect has today; the change is
only that N lines now mean N of them rather than one of them. Requiring the lines
to appear *in order* was weighed and refused: it would couple every fixture to a
gate's report ordering, which several gates do not fix and none contracts, and it
would buy nothing — a fixture pinning two findings cares that both fired, not
which printed first.

The rule applies to `good/expect.txt` identically. The asymmetry today is that
`good/expect.txt` is optional and `bad/expect.txt` is required; that asymmetry is
unchanged, and there is no reason for the two to *interpret* their contents
differently once both exist.

### 2. The failure report names every missing line, not the first {design-bearing}

Today the diagnostic prints `want: $expect` — the whole file, against output that
did match part of it. Under a conjunction that is actively misleading, because
the reader cannot tell which line failed.

The runner collects **all** missing lines and prints them. Not the first one: the
defect this unit closes is silent under-assertion, and a diagnostic that reveals
one missing pin per run rebuilds a smaller version of the same problem — an
author fixes line 2, re-runs, and learns line 3 was also missing. The cost is a
loop over at most a handful of lines on a path that only executes on failure.

### 3. The semantics are stated in the contract {mechanical}

gate-sdk/SPEC.md §run-gate-tests currently says `good/` must "print its
substring" and `bad/` must print "`bad/expect.txt`'s substring" — singular, in a
sentence that has no room for the multi-line case. It gains the conjunction rule,
the blank-line rule, the order-independence, and — the part that makes this a
contract rather than a note — the statement that **a case pinning two findings
writes two lines**. That sentence is what turns a runner behavior into something
every kit's fixture authors can rely on, which is why this unit is a feature and
not a bug fix.

Mechanical because delta 1 fixes the semantics; this delta transcribes them into
the section that already owns the runner's contract.

### 4. Coverage: a bespoke unit test, not a fixture pair {design-bearing}

The queue entry asks for "the runner's own fixture pair". That shape is wrong and
the correction is a ruling, not a quibble: `run-gate-tests.sh` is a `bin/` tool,
never a `gates.list` member, so it owes no `good/`+`bad/` pair — the same standing
§upgrade-smoke states for its own tool ("a `bin/` tool, not a gate — no
`good/`+`bad/` fixture pair is owed"). A pair would also be circular in a way
that hides failures: `check-gate-fixture-coverage`'s authority set is the
registry, and the runner is not in it, so a pair here would be an unowned
directory the coverage gate never audits.

The coverage lands as **`gate-sdk/gate-tests/run-gate-tests.test.sh`**, on the
precedent of `run-for-path.test.sh` — the same shape for the sibling `bin/` tool:
build a hermetic scratch tree, invoke the tool against it, assert on its output
and exit code. It sources `lib/test-hermetic.sh` as its first act, which
`check-test-hermetic` assertion A requires of every bespoke test.

**The self-invocation is bounded and worth naming**, because a test of the runner
is run *by* the runner. The inner invocation is passed an explicit scratch
`TESTS_DIR` holding fixture directories and **no `*.test.sh`**, so it runs pairs
and returns; there is no second level. Build must not let the scratch tree
acquire a `.test.sh`.

**The cases, and the one that carries the whole unit:**

| Case | Old semantics | New semantics |
|---|---|---|
| `bad/expect.txt` with two lines, gate prints **only the first** | **passes** (disjunction) | **fails**, naming line 2 |
| two lines, gate prints both | passes | passes |
| two lines, gate prints **only the second** | passes | fails, naming line 1 |
| one line, present | passes | passes |
| blank line among two real lines | — | passes; the blank asserts nothing |
| multi-line `good/expect.txt`, one line absent | passes | fails |

Row 1 is the unit. It is the only case that changes verdict in the direction the
defect lives, and a test suite that omits it proves nothing — this whole
iteration exists because assertions that cannot fail were shipped as if they
could. Build writes row 1 first, watches it **fail against the unpatched
runner**, and only then applies delta 1. Row 3 exists because the disjunction is
symmetric and a fix that only checks the first line would pass row 1 and still be
wrong.

### 5. The existing corpus is re-verified, not assumed {mechanical}

Measured at scope: **3 of 187** tracked `expect.txt` files are multi-line —
`check-spec-pointer` (canon-kit), `check-queue-entry-budget` and
`check-tag-lead-line` (queue-kit), all `bad/` cases, 8 lines between them (2 + 4
+ 2). All eight were probed against a conjunction reading and all currently hold,
so **the tightening is expected to red nothing**.

Expected, not assumed. Build re-runs the full fixture battery after delta 1 and
treats any red as a real finding: a line that no longer holds is a fixture that
has been asserting nothing, which is this unit's whole thesis arriving as
evidence. The corpus also grows — build re-derives the multi-line set at its own
HEAD rather than trusting these three names.

Mechanical: the oracle is the battery. Run it, read it, stop when green.

## Producers and consumers

**The expect-line set (existing state, changed interpretation).**
Producer: a fixture author writing `<case>/expect.txt`. Reachable in the real
configuration by construction — every registered gate owes a pair
(§check-gate-fixture-coverage), each pair's `bad/` case owes an `expect.txt`, and
187 of them exist today. No configuration enables or disables this; there is no
knob and none is added, which keeps the runner's zero-knob posture intact.
Consumer: `run_case` in `gate-sdk/bin/run-gate-tests.sh` — the file's **only**
reader in the tree. That single-reader property is what bounds this change: the
interpretation of `expect.txt` changes in exactly one function, and no other
component parses those files.

**The per-line verdict (new internal state).**
Producer: the per-line loop in `run_case`, once per non-blank line, on the
failure path only. Reader, named because a field without one is removed: the
`FAIL:` report in the same function, which prints the missing set — and through
it the operator at the `run-gate-tests` transition and the CI tier, which
§Enforcement tiers specifies runs `run-gate-tests.sh` as its own step precisely
so a fixture-logic failure is attributed to the gate rather than to the battery.
That attribution is what makes delta 2's per-line naming reach a reader who is
not the author: on CI, the named missing line *is* the whole report. It is
consumed at the point it is produced and never stored, so it opens no channel and
no artifact goes stale on it.

**The conjunction contract (new interface obligation on every kit).**
Producer: gate-sdk/SPEC.md §run-gate-tests, as the statement fixture authors
write against. Consumer: every kit's fixture authors — this is the delta that
makes the unit cross-kit in *contract* even though it is single-component in
*code*. Every kit's `gate-tests/` corpus is governed by the new reading from the
moment it lands, and the obligation it creates is one a kit can only violate by
writing an expect line that does not hold, which the runner then reds on. There
is no migration and no per-kit action: the contract tightens, the corpus already
satisfies it, and the enforcement is the runner itself.

**Whole-component-set reader survey.** `run-gate-tests.sh` is invoked by this
repo's per-kit fixture battery (README.md §This repo, governed), by the CI
`gates` workflow as its own step, and by the evidence-kit validate suite that
runs the fixture corpus; the fixture trees it reads live at
`<kit>/gate-tests/*/{good,bad}/`. Its contract prose lives at gate-sdk/SPEC.md
§run-gate-tests and §Fixture pairs, mirrored to `docs/gate-sdk/SPEC.md` by the
docs projection. Build re-runs this survey against the tree before implementing,
with **no `2>/dev/null` on any path probe** — a silenced stderr on a mistyped
path reads a live reader as absent.

## Existing sections updated

- **gate-sdk/SPEC.md §run-gate-tests** — the `good/`/`bad/` sentence's singular
  "substring" becomes the conjunction contract: every non-blank line of an
  `expect.txt` must appear, order-independent, blank lines asserting nothing, and
  a case pinning two findings writes two lines (deltas 1, 3). The failure-report
  behavior — all missing lines named — joins it (delta 2). The bespoke unit test
  is named beside the other `bin/`-tool tests, with the no-fixture-pair-owed
  reason (delta 4).
- **gate-sdk/SPEC.md §Fixture-pair discipline** — the parenthetical describing a
  `bad/` case as "asserting exit 1 + an `expect.txt`" gains the plural reading,
  so the two sections that describe the same obligation agree. It cites
  §run-gate-tests for the semantics rather than restating them — one owner per
  fact.
- **gate-sdk/README.md** — if its fixture-authoring guidance restates the
  single-substring reading, it cites the SPEC section instead of restating the
  new one. De-literalization: one owner for the rule.
- **docs/site-architecture.md §Generated projections** — no new gate, no
  `# graph:` manifest change, so the pre-commit hook and the graph artifact are
  untouched. A new `gate-tests/*.test.sh` changes the `gate-sdk-gate-test`
  derived enum-set family (canon-kit/SPEC.md §check-prose-enum derives it from
  tracked top-level `gate-tests/*.test.sh` basenames) — derived, so it enrols
  with no edit, but build should expect the set to grow by one and not read that
  as drift. The docs mirror of gate-sdk/SPEC.md restales and is regenerated from
  its rostered command.

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
