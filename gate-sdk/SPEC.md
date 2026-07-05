# gate-sdk — a self-testing lint framework for prose/spec/config surfaces

Machine-gated consistency for the surfaces conventional linters ignore:
markdown specs, glossaries, task queues, config projections, diagrams — any
text whose drift is mechanically decidable. A **gate** is a small shell script
that checks one invariant across one or more governed surfaces and blocks the
commit (or the merge) when they disagree.

Extracted from the governance meta-layer of a private production platform. The
kit carries the generic mechanism only; a consumer's rule content (term lists,
coupling vocabularies, glossary bodies) stays in the consumer repo.

## Layout and configuration

The kit is vendored (or submoduled) into a consumer repo, conventionally at
`gate-sdk/`. The consumer owns a **gates directory** (default `scripts/`,
override with `GATE_SDK_GATES_DIR`) holding:

- `gates.list` — the registry: one gate name per line (`#` comments and blank
  lines ignored). A listed name resolves to `<gates-dir>/<name>.sh` first, then
  each vendored kit's `checks/<name>.sh` — so any kit's shipped gates are
  registered by name alone, and a consumer can shadow one by dropping a
  same-named file in its own gates dir. The kit set defaults to gate-sdk plus
  every sibling directory holding a `checks/` (a vendored Checkwright kit);
  override with `GATE_SDK_KIT_DIRS` (space-separated kit roots).
- the consumer's own `check-*.sh` gates (copy-edits of
  `templates/check-skeleton.sh`).
- `gate-tests/` — the consumer's fixture tree (see §run-gate-tests).
- `git-hooks/` — the generated `pre-commit` (see §gen-pre-commit) and any
  hand-written hooks.
- `graph-vocab.sh` — optional rule content for `check-graph` (see there).

Environment overrides, all optional: `GATE_SDK_GATES_DIR` (default `scripts`),
`GATE_SDK_TESTS_DIR` (default `<gates-dir>/gate-tests`), `GATE_SDK_HOOKS_DIR`
(default `<gates-dir>/git-hooks`), `GATE_SDK_WORKFLOW_DIR` (default
`.workflow`), `GATE_SDK_TMP_DIR` (default `.tmp`), `GATE_SDK_QUEUE_FILE`
(default `TASK-QUEUE.md`), `GATE_SDK_PRUNE_DIRS` (default
`target .git node_modules .tmp gate-tests`), `GATE_SDK_GRAPH_VOCAB` (default
`<gates-dir>/graph-vocab.sh`), `GATE_SDK_KIT_DIRS` (default: gate-sdk + its
siblings holding a `checks/`). Paths are repo-root-relative; every entry point
`cd`s to `git rev-parse --show-toplevel` before resolving them.

## The gate model

A gate family imposes test-grade rigor on prose and config surfaces, but a gate
whose own correctness is unverified silently stops enforcing — the platform
this kit was extracted from shipped two *self-broken* gates (a false-green on a
crashed `awk`; two scanner crashes) before adopting the four contracts below.
The family tests, lints, and constrains itself by the same standard it holds
the governed tree to.

### Output contract

A gate signals success with exactly one line `^<NAME>: clean (<parenthetical>)$`,
where `<NAME>` is the gate's stable upper-token id and the parenthetical states
what was checked. One canonical success token across the whole family — a gate
that "passes silently" is indistinguishable from one that did nothing. Failure
is one line per finding (location + what is wrong) then a `help:` remedy line
naming the concrete action — the model is the Rust compiler's `help:`, not a
restatement of the violation. A gate with multiple distinct failure classes
gives each its own `help:` line. Exit codes: **0** clean, **1** violation,
**2** harness/usage error.

Enforced by `check-gate-output` (the static half — it asserts each member's
source carries both a `: clean` emission and a `help:` emission); the `good/`
fixture is the behavioral half that catches a clean line that never executes.

### Fail-closed contract

A gate that captures a subprocess's stdout and branches on the captured value's
**emptiness** false-greens when that subprocess crashes: the output is empty,
so the gate prints `clean` — but the check never ran. The fix is to key on the
subprocess **exit status**. The wrapper is centralized in `lib/gate.sh`; source
it and call `fail_closed` right after a capture:

```bash
out="$(awk '…' "$FILE")"; st=$?
fail_closed "$st" check-foo awk     # non-zero status -> exit 2, never a false clean
```

Wrap an `awk`/`jq` capture that reads a **file** and feeds the verdict. Leave
alone — and a sweep must NOT blanket-wrap — `grep`'s exit-1-on-no-match
(expected, not an error), an already-status-checked capture, and in-memory
transforms (`<<<` here-strings, trivial field selectors that cannot fail on
present input). Enforced by `check-gate-fail-closed`; a genuinely-safe capture
opts out with `# fail-closed-exempt: <reason>` on the comment block immediately
above it. The helper itself is tested directly by
`gate-tests/lib-gate.test.sh` (a per-gate input fixture cannot prove it — a
well-formed `awk` cannot be crashed on present input).

### Fixture-pair discipline

When a gate is written or edited, it ships with — or updates — its
`good/`+`bad/` fixture pair under `<tests-dir>/<gate>/`, run by
`run-gate-tests.sh`. A gate that prints `clean` on broken input is invisible to
every static check; the only proof it fails closed and that its error text is
right is a known-bad input (the `bad/` case asserting exit 1 + an `expect.txt`
substring), paired with a `good/` case asserting acceptance. Coverage is
enforced by `check-gate-fixture-coverage`: every registry member carries either
a pair or a `# no-fixture: <reason>` header annotation — the honest, reviewable
escape for whole-tree scanners whose state has no static-fixture representation
(e.g. a HEAD-vs-worktree diff). A fixture-*capable* gate carrying the valve as
a stopgap is filed as debt and fixtured, never given a dishonest "infeasible"
reason.

### Self-lint

Every script in the family — the consumer's gates and the kit's own `lib/`,
`bin/`, `checks/` — passes ShellCheck at `-S warning`, enforced by
`check-shellcheck`. A false positive is silenced inline with
`# shellcheck disable=SCxxxx` plus a justifying comment, never a blanket
`.shellcheckrc`.

### Shared cross-gate values

A value or small lookup needed by ≥2 gates lives in `lib/gate.sh` as the single
source — reference it, never re-literal it per gate. This is deliberately *not*
a base library: it shares **values and walk adapters**, never gate structure
(structure is copied from the skeleton, so it stays per-gate and legible), and
its failure mode is loud — a missing or malformed `lib/gate.sh` aborts every
sourcing gate at the `source` line, the fail-closed direction.

### Calibration lessons (paid for, now design rules)

- **Wrap-aware matching.** When a gate's signal token can hard-wrap on a
  ~80-col prose surface, match over the joined logical unit, not the physical
  line, and prove it with a wrapped-case `bad/` fixture.
- **Scope a sentinel allowance to its phase.** A gate that admits a bootstrap
  value (`—`, `TODO`, `0`) with a blanket allowance leaks it past the one phase
  it was meant for; bind the allowance to the phase.
- **Exclude the fixture tree from whole-tree discovery.** A gate that discovers
  inputs by a shape its own fixtures also carry descends into the fixture tree
  and false-reds — and a *different* gate's fixtures can break yours. Use
  `lib/gate.sh`'s prune adapters: a `find` walk → `gate_find <root> <expr…>`
  (parenthesize a multi-term expression — `gate_find` appends `-print`, and an
  unparenthesized `-o` chain binds it to the last term only); a `grep -r` walk
  → splice `"${GATE_GREP_EXCLUDES[@]}"`; a paths-then-filter walk →
  `gate_path_pruned "$f"`.
- **Couple per surface, not per file.** When one artifact holds N
  representations of the same model, each needs its own coupling edge or an
  explicit ungated marker; "this file is heavily gated" silently reads as "all
  of it is gated."
- **Fixtures probe the boundary, not the live tree.** A gate passing on the
  real corpus is not evidence it is correct — write the pair to drive the
  inputs production doesn't (last element, empty set). Last-line idiom:
  `while IFS= read -r e || [[ -n "$e" ]]`.

### When a gate earns its place

A gate is a standing cost (maintenance, false-positive friction, reading load),
so the family is sized deliberately. When a consistency property on a driftable
governed surface has a **cheap, low-false-positive, mechanically-decidable**
check, build the gate **proactively — before any attested miss**; discipline is
not an acceptable substitute for a check this class can make. A gate that is
expensive or higher-FP waits for a real miss to attest it — and the miss *is*
the `bad/` fixture. Cheap + low-FP is necessary, not sufficient: the gate must
check a real drift axis, never a trivially-true proxy (heading *presence* while
the content drifts) that manufactures false confidence. The dual rule minimizes
the standing set *subject to the guarantee being preserved*: a gate is
removable only when its guarantee is delivered another way or was never
attested. What stays human is the irreducibly semantic judgment alone — *is
this prose still true?*

## Enforcement tiers

Three concentric tiers, each an outer backstop for the one inside it:

- **pre-commit** (opt-in, per-clone, fast) — the generated triggered subset,
  catching drift in the single commit that perturbs it. Bypassable by design
  (`git commit --no-verify` is a valve, not a hole, once an outer tier exists).
- **pre-push / full battery** (opt-in, per-clone, fuller) — `run-gates.sh`
  whole-tree before the work leaves the machine.
- **CI** (server-side, authoritative) — `run-gates.sh` + `run-gate-tests.sh`
  on every push, with branch protection making a merge conditional on them.
  Only this tier is a guarantee; the inner tiers are latency optimizations.
  `run-gate-tests.sh` runs as its own step, not folded into the battery —
  `check-gate-fixture-coverage` asserts fixtures *exist* but never *executes*
  them; the execution is the gate-authority backstop, kept separate so a
  fixture-logic failure is attributed to the gate.

## The `# graph:` manifest

Every registered gate's header carries a one-line coupling manifest:

```
# graph: couples=<globs> dir=bi|one valve=none|PROPOSED tier=precommit|align-only [mode=staged|whole-tree] [trigger=<globs>] [gen=manual]
```

- `couples=` — the surfaces the gate binds (comma-separated globs).
- `dir=` — `bi` for a coupling bijection (both sides must agree), `one` for a
  one-way audit.
- `valve=` — `PROPOSED` marks a cycle valve: a coupling where a leading
  (design) surface may run ahead of a lagging (code) surface via a
  queue-tracked marker; `none` means the sides must agree now.
- `tier=` — `precommit` gates emit a trigger block in the generated hook;
  `align-only` gates run only in the full battery. Default to `precommit`; the
  discriminator is not cost but whether the invariant is **restorable within
  the single commit that perturbs it** — a settled-corpus audit would false-red
  on work-in-progress and belongs to the full battery.
- `mode=staged` — the hook passes the staged subset of the trigger globs as
  positional args; default (`whole-tree`) emits a bare invocation.
- `trigger=` — hook guard globs when they diverge from `couples=`; `trigger=*`
  emits an unconditional invocation.
- `gen=manual` — the gate's hook block is bespoke and round-trips verbatim
  between `# >>> manual: <gate>` / `# <<< manual: <gate>` sentinels.

## Consumer smoke

The fixture suites prove each gate in isolation on contrived case dirs, and a
consumer repo's battery runs under that consumer's own config overrides. Two
things go untested there: that a *fresh* consumer reaches green by following
the kit READMEs, and that the **platform defaults** hold on a vendored-kit tree
under zero config. The DoD-mode defect (`spec-kit-vendored-spec-dod-scope`)
shipped through exactly that gap. `bin/run-consumer-smoke.sh` closes it,
mechanizing what was a hand-repeated validate-stage prose ritual with no
committed evidence.

The harness (`run-consumer-smoke.sh [--keep] [kit-root...]`, a `bin/` tool,
never a registered gate — it builds a repo and runs the battery repeatedly, so
it is pre-commit-unfit by runtime budget): builds a scratch consumer in a fresh
temp dir (`git init`, seed commit), vendors each kit root by copy (default:
`gate_kit_roots`), and runs each kit's `smoke/install.sh` — gate-sdk first,
then argument order. It then commits the installed baseline and asserts the
full battery is green under **zero consumer config** (the positive green token
`All N gates passed` — the defaults-on-a-vendored-tree assertion no fixture
suite makes). Per kit shipping `smoke/violation.sh` it fires one crafted
violation, re-runs the battery, asserts a non-zero exit **and** a `FAIL:`
line naming the expected gate, then restores the tree (`git checkout . &&
git clean -fd`) before the next kit; it asserts green once more after the last
restore. Exit codes follow the gate convention (0 all hold, 1 an assertion
failed, 2 usage/environment); the success token is `CONSUMER-SMOKE: clean
(<n> kits installed, <m> violations fired)`. `--keep` retains the temp dir and
prints its path (the temp-dir write's named reclaim path).

**The `smoke/` per-kit contract.** Every vendored kit ships a `smoke/`
directory — shipping it joins fixtures + README + SPEC in the kit-landing
checklist; a kit root lacking `smoke/` is an environment error (exit 2).

- `smoke/install.sh` (required) — run with cwd = scratch-consumer root and env
  `SMOKE_KIT_ROOT` = the vendored copy of the installing kit. The executable
  form of that kit's README install steps: register its gates in
  `scripts/gates.list`, establish the minimal governed surface its gates need
  to be green, and regenerate the hook + graph artifacts. It may assume gate-sdk
  is already installed (it runs first), nothing else. A non-zero exit aborts the
  harness with exit 2 (a broken installer is an environment failure, not a gate
  finding).
- `smoke/violation.sh` (optional) — same cwd/env contract; mutates the scratch
  tree to introduce exactly one violation restorable by `git checkout` +
  `git clean` (edit a tracked file or add an untracked one), and prints the
  expected gate name as its first stdout line (the harness's red-phase assertion
  reads it). A kit without one contributes install coverage only; the harness
  prints a notice per such kit so the gap is visible in the evidence. The file
  is rightly absent only where no battery-reddening violation is craftable —
  a kit that registers no gates has nothing to redden.

Producers and consumers: `smoke/` content is produced by the kit author at
kit-landing time and consumed by the harness's install and violation phases;
the expected-gate name (violation.sh line 1) is read by the red-phase
assertion; `SMOKE_KIT_ROOT` is produced by the harness per invocation and read
by the scripts to copy from their own kit; the harness verdict is consumed by
the validate-stage ritual (which gates on the success token) and is the natural
CI entry point (wiring CI is out of scope here).

## Per-component contracts

### lib/gate.sh

The family's single sourced library — values + adapters, never gate structure.
Owns `fail_closed`, `GATE_PRUNE_DIRS` + the `gate_find` /
`GATE_GREP_EXCLUDES` / `gate_path_pruned` walk adapters, and the registry
helpers `gate_sdk_root`, `gate_sdk_gates_dir`, `gates_list_members`,
`gate_resolve`, `gate_kit_roots`, `gate_check_dirs` (the multi-kit resolution
path other kits' gates ride). `fail_closed` must be passed *only* a status that genuinely
means the check could not execute (an awk/jq/parser crash) — never `grep`'s
exit 1, which is the expected "no match"; the caller draws that line at the
capture site.

### run-gates

Aggregate runner: executes every `gates.list` member in one shot, timing each
(`<tmp-dir>/gate-timings.txt`, `<gate> <elapsed-ms>` per line + `TOTAL` —
uncommitted by design: a measurement, not state). A member that resolves
nowhere is a failure, not a skip. Exit 0 only when every member passed.

### run-gate-tests

Golden-fixture runner. Each `<tests-dir>/<gate>/` holds `good/` + `bad/` case
dirs; the runner `cd`s into the case dir and invokes the gate (resolved against
the consumer gates dir, then each vendored kit's `checks/`) with the args in the case's
`args` file (`#` lines stripped). `good/` must exit 0 (and, when
`good/expect.txt` exists, print its substring); `bad/` must exit 1 and print
`bad/expect.txt`'s substring — a rejection substring is required, so the *right*
finding fired. Exit 2 from a gate marks the fixture malformed (harness error,
distinct from logic failure). `<tests-dir>/*.test.sh` unit tests run after the
pairs; each must exit 0. The runner is a test layer parallel to the gates,
never a `gates.list` member.

### run-consumer-smoke

The scratch-consumer install+violation harness (§Consumer smoke): vendors the
kit roots into a fresh temp repo, drives each `smoke/install.sh`, asserts the
full battery is green under zero config, then fires each `smoke/violation.sh`
and asserts the battery reddens at the named gate before restoring. A `bin/`
tool, never a `gates.list` member — it is pre-commit-unfit by runtime budget
and is the proof that the platform defaults hold on a vendored-kit tree.

### gen-pre-commit

Emits `<hooks-dir>/pre-commit` from the per-gate `# graph:` manifests: a
`tier=precommit` gate becomes one trigger block shaped by `trigger=`/`mode=`;
a `gen=manual` region round-trips from the current hook. `--emit` prints to
stdout (`check-graph` compares against it); `--write` rewrites the hook.
Adding a gate to the hook is manifest-only — there is no second hand-wiring
step to drift. The emission is deterministic (no timestamps) so the committed
hook is byte-stable.

### install-hooks

One-time per-clone opt-in: sets `core.hooksPath → <hooks-dir>` (and
`blame.ignoreRevsFile` when `.git-blame-ignore-revs` exists). Refuses to point
at a nonexistent hooks dir — generate the hook first.

### check-shellcheck

Invariant: every `*.sh` directly under the consumer gates dir and each
vendored kit's `lib/`, `bin/`, and `checks/` passes ShellCheck at `-S warning`
(the self-lint contract). A missing `shellcheck` binary is exit 2 — a gate that cannot run is
not clean.

### check-gate-output

Invariant: every `gates.list` member's source contains both a `: clean`
success emission and a `help:` remedy line (the static half of the output
contract). Presence is checked, not correctness — whether the clean line
actually fires is the `good/`-fixture job; whether the remedy text is accurate
is human review.

### check-gate-fail-closed

Invariant: every `awk`/`jq` command-substitution capture in a `check-*.sh`
gate handles its subprocess exit status — `fail_closed`, an inline
`|| { … }` guard, a captured `=$?`, or an explicit `# fail-closed-exempt:`
opt-out. Only `awk`/`jq` captures are checked: `grep`'s exit 1 on no-match is
expected, here-string captures read data already in memory, and arithmetic
`$((…))` is never matched. A parser wrapped inside a shell function is not
visible to this static scan; the opt-out covers residual false positives.

### check-gate-fixture-coverage

Invariant: every `gates.list` member either ships a `{good,bad}/` fixture pair
(searched across the consumer tests dir, then each vendored kit's
`gate-tests/`) or carries a `# no-fixture: <reason>` header annotation. The authority set is the
registry — the gates that gate the tree — not every `check-*.sh` file. A
half-built pair is a defect regardless of any opt-out. The gate cannot
mechanically distinguish "infeasible" from "stopgap"; honesty is upheld by the
reason text.

### check-gate-assertions

Invariant: every `### <gate>` subsection in the family SPEC whose contract
enumerates its assertions (a count-word followed by an enumeration noun and a
labeled span) is coupled to a matching `# assertion <label>:` marker set in
the gate's code — the marker label set equals the contract's label span, and
its size equals the count-word. This catches the prose-vs-code drift an
internal count⟺span check cannot: a contract can be internally consistent
while the code grew a sixth assertion.

Discovery is first-paragraph-scoped, requires the enumeration noun
(`assertion(s)`|`axes`|`axis`|`checks`) adjacent to the count-word
(`two`…`nine`), and requires the first following parenthetical to be a
single-char `(X)` label — four filters that exclude sibling-gate mentions,
follow-on sentences, hierarchical axis/sub-rule contracts, and count-words
with non-enumeration nouns. With no spec argument the gate scans
`<gates-dir>/SPEC.md` when present plus each vendored kit's own `SPEC.md`;
each matched heading resolves to its gate source through the registry path. Honest residual: the marker catches editing one
side without the other, but not adding an assertion while forgetting *both*
its marker and the contract. A first paragraph that embeds the literal pattern
in example prose self-matches — the failure is loud (a false positive forcing
a reword), never a silent miss, so it is accepted. Requires GNU awk.

### check-gate-exemption-tasks

Invariant: every element of an `# exception-list:`-tagged array in a
`check-*.sh` gate carries exactly one of two disposition annotations —
`# until: <slug>` (temporary; must resolve to a live task in the queue file's
New Features / Technical Debt / Deferred sections) or `# permanent: <reason>`
(structural out-of-scope). An element with neither, a `# until:` slug that is
Done-only or missing, or elements sharing the array's opening `=(` line are
violations. Scope is in-script exemption arrays only; inline per-site
directives (`# fail-closed-exempt:`, `# no-fixture:`) stay out — they are
local and self-evident via their adjacent comment.

### check-graph

Invariant: the `# graph:` manifest on every `gates.list` member is well-formed
and consistent, and the pre-commit hook is the faithful generated projection
of the manifests. Seven assertions: (A) every member carries a well-formed
`# graph:` line — the four required keys (`couples`/`dir`/`valve`/`tier`) plus
the optional `mode`/`trigger`/`gen` — with surfaces in the declared vocabulary
when one exists; (B) couples⊆trigger parity — each `couples=` surface is
covered by the gate's `trigger=` globs (trigger defaulting to couples), so
editing a coupled surface always fires the gate; (C) cycle-valve consistency —
a `dir=bi` gate spanning a declared-leading and a declared-lagging surface
must carry `valve=PROPOSED`; one with a leading surface but no lagging surface
may carry either valve; one with no leading surface must carry `valve=none`;
(D) hook artifact freshness — the committed pre-commit equals
`gen-pre-commit.sh --emit`; (E) the committed `CHECK-GRAPH.html` projection
matches `--emit` output; (F) every emitted asset href resolves under the
artifact's own directory — a path wrong in both generator and artifact that
(E) cannot detect; (G) every `# graph:` manifest embedded in a `SPEC-*.md`
amendment body is well-formed, with each `couples=`/`trigger=` token a
syntactically valid glob. Unlike (A), an amendment manifest's surfaces are not
required to be in the vocabulary (the coupled surface may itself be
design-ahead), and hook parity is not applied — the gate it describes is
unbuilt; parity re-fires through the normal registry path once it lands.

Rule content is config, not code: `<gates-dir>/graph-vocab.sh` may declare
`GRAPH_VOCAB` (the legal surface tokens; empty/absent disables the vocabulary
check), `GRAPH_LEADING`/`GRAPH_LAGGING` (the assertion-C sets; absent
disables cycle-valve classification, leaving the no-leading `valve=none` rule),
and `GRAPH_LAYERS` + `graph_surface_layer()` (the projection's subgraph
grouping; absent renders one layer). The `--amend-only [dir]` mode runs only
(G) over a given directory, letting the fixture pair exercise it hermetically.
Coverage ruling inherited from the platform: a `couples ⊇ find-globs` parity
check — verifying a gate's declared couples cover its real read-set — is *not*
carried; it would require parsing arbitrary shell, neither cheap nor low-FP,
and (B) already guarantees editing a coupled surface fires the gate.

### templates/check-skeleton.sh

The copy-paste reference skeleton — the canonical "how to write a gate"
template (structure + fail-closed + output contract). A new gate is a
copy-edit of it, shipping with its fixture pair. It is a template, never a
registry member; structure is copied, not imported, so it stays per-gate and
legible.
