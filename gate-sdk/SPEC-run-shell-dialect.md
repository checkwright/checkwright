# SPEC amendment: run-shell-dialect

Closes `action-run-shell-dialect-by-runner`. `check-action-run-shell` resolves an
absent `shell:` key to bash on a ground that is false for one runner class, so
the first Windows step that omits the key is linted as shell — the
false-positive engine that section's extractor rules exist to prevent, arriving
through the one door those rules leave open.

**The SPEC half of the defect is already repaired; this is the gate half.**
gate-sdk/SPEC.md §check-action-run-shell's dialect table already reads
`absent | -s bash — GitHub's documented default for a run: step on a Linux or
macOS runner. It is pwsh on a Windows runner … action-run-shell-dialect-by-runner
owns resolving it from runs-on`. The prose was corrected at the drain that filed
the entry, which left the gate visibly disagreeing with its own spec — the honest
state, and the one this amendment ends.

**This is a precondition of the target-roster widening, not a sibling lint
hazard, and the corpus census is what makes that concrete.** Every `runs-on:` in
the tree was enumerated at this stage: 40 occurrences, 39 of them plain literal
labels (38 `ubuntu-latest`, one `windows-latest`), zero arrays, zero
runner-group objects, and **exactly one** value a tree-local resolver cannot
read — `.github/workflows/publish.yml:81`'s `runs-on: ${{ matrix.runner }}`,
whose matrix is `include: ${{ fromJSON(needs.roster.outputs.legs) }}` (`:80`),
produced at runtime by the `roster` job's step from a hand-kept
`declare -A runner=( [x86_64-unknown-linux-gnu]=ubuntu-latest )` inside a **shell
body** (`:54-56`) keyed off `native/targets.list`. That one job is the one whose
runner label *becomes* `windows-latest` on the day the roster widens, and its
`run:` block names no `shell:` — as none of `publish.yml`'s five do. So the day
`x86_64-pc-windows-msvc` joins the roster, every bash body in that leg runs under
`pwsh` and the release breaks, while this gate lints those same bodies as bash
and reports clean.

**The fork this amendment was left to rule — what a resolver does with an
unreadable `runs-on` — is ruled against a measured blast radius rather than the
feared one.** The entry priced refusal as turning "any expression-valued
`runs-on` into a red on a tree that lints clean now". Measured, the price is
**one finding and one line of repair**, and the finding sits on the exact block
whose dialect is about to change under it. The ruling and its price are stated
together in delta 3.

**Measured, not asserted.** The gate was run on this tree at this stage through
its own config bridge and reports
`ACTION-RUN-SHELL: clean (18 run: block(s) linted at -S warning across 5
Actions-shaped file(s) of 11 walked; 6 file(s) skipped by the Actions-shape
predicate, 6 plain-scalar run: value(s) skipped, 0 block(s) skipped on a
non-shell dialect)`. After delta 5's repair the same eighteen blocks lint under
the same dialects and every count on that line is unchanged — the change buys a
guarantee, not a coverage move, and saying so is what stops a later reader
reading a stable tally as a no-op.

**What this amendment does not do.** It does not widen `native/targets.list`, add
a `windows-latest` entry to `publish.yml`'s runner map, or touch
`platform-support-ci-matrix`'s promotion condition. Those are that entry's and
`gate-binary-target-roster-widening`'s; this one makes the release survive the
day they land.

## What changes

### (1) The extractor learns the job partition, and dialect resolution moves to the job boundary

`native/src/gates/action_run_shell.rs`'s `Extractor` gains a job layer. Today it
has none at all — it is a flat column-indentation scan whose only structural
state is `stepcol`, and it carries no spelling of `jobs`, no job column, and no
`runs-on`. **{design-bearing}**

- A column-0 key `jobs` opens the job section; any other column-0 key closes it.
  This is §check-action-run-shell's Actions-shape predicate's own test, reused
  rather than re-spelled — a `jobs:` key at any indent is a foreign schema and
  the predicate already refuses to see it.
- The first indent under `jobs:` is the **job column**. A line at that column
  begins a job: it ends the previous job, and it resets the job's `runs-on` to
  *absent*.
- The first indent under a job id is the **job-key column**. A `runs-on:` key
  there captures the job's runner value; a value that is empty on the key line
  takes the following lines more indented than that column, so a block sequence
  and a mapping are captured whole rather than read as absent.
- A `runs:`-shaped **composite action** never enters the job section, so its
  steps carry no job and no `runs-on`. That is correct rather than a gap:
  GitHub's own schema makes `shell:` **required** on a composite `run:` step, so
  a composite step reaching delta 3's unresolved class is invalid Actions YAML
  and the finding is true. The shipped fixture `good/tree/action.yml:11` names
  `shell: bash` already and stays green.

**Resolution moves from the step boundary to the job boundary, and that is a
behavioural change worth its own sentence.** Today `flushstep` stamps the step's
`shell:` onto its blocks the moment the step ends, which is right because the
`shell:` sibling key may sit either side of the `run:` block it governs. A job's
`runs-on` has the same freedom against the whole `steps:` list — YAML admits it
after `steps:` — so the *inferred* half cannot be resolved at the step boundary
without reading a key that may not have arrived. `Item::Block` therefore carries
the step's **explicit** `shell:` as an optional value, and the job boundary (or
the file's end) stamps the job's `runs-on` class onto every block that has none.
The step-boundary flush is unchanged and keeps its own rule and its `# spec:`
comment; a second flush is added beside it, not in place of it.

**No knob, and the job layer stays inline in the module.** The extractor's
standing *stays inline until a second consumer* rule (§check-action-run-shell)
survives this delta intact and delta 2 records why.

### (2) `runs-on` is classified by a Windows test, never by a platform roster

The captured value resolves to one of three classes, and the classifier is a
new private helper in the same module. **{design-bearing}**

- **Labels.** A scalar value is one label. A sequence — flow (`[a, b]`) or block
  — is its members. A mapping is a runner group: its `labels:` members are the
  labels, and a mapping carrying `group:` with no `labels:` yields none.
- **Windows** if any label, lowercased and stripped of surrounding quotes, is
  exactly `windows` or begins `windows-`. Any label matching makes the job
  Windows: a mixed self-hosted selector may land on a Windows machine, and a
  dialect that is only *probably* bash is not one the gate may state.
- **Unreadable** if the captured text contains `${{` anywhere, or if the value
  yields no labels at all (the group-only mapping, an empty value, an absent
  `runs-on`).
- **Non-Windows** otherwise — and that is the *only* other answer. There is no
  Linux label set and no macOS label set, because bash is GitHub's default
  everywhere that is not Windows: enumerating the platforms that resolve to bash
  would be a maintained roster of runner labels drifting against a provider's
  release notes, which derivation-first refuses. The gate asserts one
  distinction because one distinction is what the dialect turns on.

**The honest limit, stated rather than papered over.** A self-hosted Windows
runner registered without a `windows` label reads as non-Windows and its
unshelled bodies are linted as bash. The label convention is the only
platform signal a tree-local reader has, and widening the match by guessing at
label vocabularies is what turns a stated assertion back into a heuristic. The
remedy in that tree is the same one line delta 3's finding asks for.

**This delta does not make the gate a consumer of §check-action-gh-repo's
job-partitioned walk, and the refusal is reasoned rather than assumed.** That
walk over-detects by design, carries no body reassembly and no refusal
semantics, and emits events shaped for repository-context arms; this extractor
must reassemble a block scalar byte-exactly and refuse four constructs loudly.
What the two now share is an *indentation convention* — find the job column,
find the job-key column — not a mechanism, and a helper holding fifteen lines of
convention for two differently-shaped state machines is a coupling with no
invariant to hold. The standing rule at §check-action-gh-repo ("the walk is this
gate's own … it is **not** the second consumer that section's standing rule
waits for") therefore still holds between *these two*; its sibling amendment
`SPEC-action-permissions.md` is where a real second consumer arrives, and the two
answers differ because what is shared differs.

### (3) The dialect table gains its second axis, and the gate gains one new failure class

gate-sdk/SPEC.md §check-action-run-shell's dialect table is replaced. The
`absent` row's prose apology — the row that names this amendment's own slug as
the owner of the repair — goes with it. **{design-bearing}**

| `shell:` | `runs-on` | resolution |
| --- | --- | --- |
| absent | non-Windows | `-s bash` — GitHub's documented default for a `run:` step everywhere but a Windows runner |
| absent | Windows | **finding** — the runner's default is `pwsh` and the step does not say so |
| absent | unreadable, or the step has no enclosing job | **finding** — the dialect cannot be stated, so it is not assumed |
| `bash` (with or without arguments) | any | `-s bash` |
| `sh` / `dash` / `ksh` | any | the matching ShellCheck dialect — linting a POSIX body as bash hides the portability findings that dialect exists to surface |
| anything else (`pwsh`, `python`, a custom `{0}` template) | any | the block is **skipped and counted** — the body is not shell, so there is no shell to lint |

**One rule, stated once, covering both new rows: a step's dialect must be
knowable, and where the gate cannot state it the step says it.** The finding line
names the file, the `run:` block's line and which of the two conditions fired;
the class takes its own `help:` line, per §Output contract's rule for a gate with
more than one failure class, and the remedy it names is one line — `shell: bash`,
or `shell: pwsh` where pwsh is what was meant.

**Why the Windows row is a finding and not a skip, which is the fork's real
question.** Skipping would make the inferred case agree with the explicit `pwsh`
row, and agreement was the entry's complaint — the resolver taking the opposite
branch from the one it takes on the same body spelled out. But agreement is not
the objective the entry actually named: it observed that nothing is red today
"and the reason is a habit rather than a mechanism", `gates.yml`'s
`install-smoke-windows` naming `shell: bash` on every one of its six steps with a
header comment saying why. A skip leaves that habit a habit and adds a silent
lint hole on exactly the platform the trajectory is buying. A finding converts
the header comment into the mechanism it describes, at a cost of one line on a
Windows step — and a Windows author who genuinely wants pwsh writes `shell:
pwsh` and is skipped-and-counted through the row that already existed.

**Why the unreadable row is a finding and not a refusal.** Exit 2 is for a
construct the gate cannot process; here the gate processed the file perfectly and
found a step whose dialect nothing in the tree states. That is a property of the
workflow, which is exit 1 — and unlike a refusal it names a remedy the author can
take. §Fail-closed contract is untouched: nothing is captured and read as clean.

**No valve, and the refusal is deliberate.** The `# graph:` manifest keeps
`valve=none`. Every finding this class raises is discharged by one `shell:` key,
which is strictly better than an exemption marker in the same place: the marker
would be one line that leaves the dialect unstated, the key is one line that
states it. A valve is worth minting where the remedy is unavailable, and here it
never is.

### (4) A `defaults:` block carrying a `run:` key is a loud refusal

A `defaults:` key at column 0 or at the job-key column, whose subtree carries a
`run:` key, is refused at exit 2 naming the construct and its line — joining the
folded scalar, the explicit indentation indicator, the anchor and the alias in
§check-action-run-shell's *refused* set. **{design-bearing}**

GitHub's `defaults.run.shell` overrides the runner default for every step beneath
it, at workflow or job level. A resolver that reads `runs-on` and not `defaults`
would answer bash for a job whose steps all run under pwsh, which is precisely
the wrong-dialect lint this amendment exists to end — reintroduced one level up.
Modelling it properly means a third inheritance layer with its own precedence
rules; refusing it costs nothing and can never become a false negative, which is
that section's own stated reason for preferring a loud refusal.

**The cost is measured at zero:** a tree-wide grep for `defaults:` in every
`.yml`/`.yaml` returns exactly one hit, `docs/_config.yml:15`, which is Jekyll's
and is not Actions-shaped. No workflow, no kit template and no fixture carries
one. The refusal fires only inside the Actions-shape subject, as every other
refusal in that set does.

### (5) `publish.yml`'s one unresolvable step names its shell

`.github/workflows/publish.yml`'s `build` job step *build the gate binary for
`${{ matrix.target }}`* (`:92-95`) gains `shell: bash` beside its existing `env:`
key. This is the tree's only finding under delta 3 and the whole of the in-tree
repair. **{mechanical}**

The other four `run:` blocks — `roster` (`:51`), `pack` (`:141`), `npm` (`:237`)
and `release` (`:279`) — sit in jobs whose `runs-on` is the literal
`ubuntu-latest`, so delta 3's first row resolves them and they are linted exactly
as today. **They are deliberately left unshelled**: adding `shell: bash` to them
would be decoration a gate does not ask for and does not hold, and the
enforcement-first reading is that the mechanism covers them already.

**`shell: bash` is the right value rather than the appeasing one.** On a Windows
runner `shell: bash` selects Git-for-Windows bash, which is what `gates.yml`'s
`install-smoke-windows` job already does for all six of its steps and what its
header states the reason for. So the line that clears the finding is the same
line that keeps the release working on the day the runner map gains a
`windows-latest` entry — the failure this entry was filed against.

### (6) The fixture pair gains the runner-dialect cases

`gate-sdk/gate-tests/check-action-run-shell/`'s `good/` and `bad/` trees gain the
cases the corpus census proved absent: every existing fixture that carries a
`runs-on:` at all carries the literal `ubuntu-latest`, no fixture is a
multi-runner workflow, and none exercises a Windows job or a non-literal
`runs-on`. **{design-bearing}**

`good/` gains a workflow whose jobs cover the rows that stay clean: a Windows job
naming `shell: bash` (linted as bash), a Windows job naming `shell: pwsh`
(skipped and counted), a non-Windows job with an absent `shell:` (the existing
behaviour, held), a job whose `runs-on` is a sequence of non-Windows labels, and
a job whose `runs-on` is a `${{ }}` expression but whose step names `shell:`
explicitly — the case that proves the finding is about the *step*, not the job.
`good/expect.txt`'s counts move with the added blocks.

`bad/` gains a second workflow carrying both new finding conditions: a Windows
job with an absent `shell:`, and a job with an expression-valued `runs-on` and an
absent `shell:`. The entry anticipated the first of these as the `bad/` case and
was right about it; it could not know the second existed until the fork was
ruled, and both live on the same side.

The refusal arm joins the **dynamic** suite beside the four refusals already
there (`gate-sdk/gate-tests/check-action-run-shell.test.sh`, which synthesises
its workflows): a workflow-level `defaults: run: shell:` block and a job-level
one, each asserted to exit 2 naming the construct. A refusal case cannot live in
the static `bad/` tree — a `bad/` case is asserted to emit a `help:` line, which
is exit 1 — and the existing refusals are already there for that reason.

## Producers and consumers

**The job partition and its `runs-on` capture (deltas 1 and 2)** is new state
inside one module and crosses no process or file boundary.

- *Producer:* `Extractor::feed`, on every line of every Actions-shaped file the
  walk yields, in the same pass that already extracts blocks. **Enabling config
  actually set:** none exists to set — the extractor takes no knob, the scan set
  is derived and the prune set is the shared one (§Shared cross-gate values), so
  the layer is live on this repo's first battery run after the build rather than
  gated behind a configuration.
- *Consumer:* the job-boundary flush added in delta 1, by direct call, which
  stamps the classified `runs-on` onto every `Item::Block` the job produced that
  carries no explicit `shell:`. Its own consumer is `scan`, which reads the
  resolved dialect exactly where it reads it today — to skip and count, to lint,
  or (new) to raise the delta 3 finding.
- *Named reader for every field:* `Item::Block`'s `shell` field changes from a
  resolved `String` to the step's **optional explicit** value; its reader is the
  job-boundary flush, at the job's close. The job's `runs-on` class is a single
  field on the extractor read at that same transition and nowhere else. No field
  is added that is written at one transition and read at none.

**The new finding class (delta 3)** produces an output line and an exit code.

- *Producer:* `scan`, at the point it today decides between lint and
  skip-and-count.
- *Consumers,* two, both existing and both named because neither is optional: the
  battery `gate-sdk/bin/run-gates.sh`, which reads the exit code; and
  `gate-sdk/bin/run-gate-tests.sh`, which reads the `help:` line as the `bad/`
  case's assertion (§Output contract's runtime half). The pre-commit hook is a
  third and reaches it through the generated projection, which delta 3 does not
  perturb: the gate's tier stays `precommit` and its `# graph:` couplings are
  unchanged, so `scripts/git-hooks/pre-commit` needs no regeneration.

**The refusal (delta 4)** produces an exit-2 message. Its producer is
`Extractor::feed`'s existing refusal path, its consumer is `print_refusal` and
then the battery, and it adds no state.

**Deltas 5 and 6 introduce no state, event or interface.** Delta 5 adds one YAML
key to a workflow whose readers are GitHub Actions and this gate. Delta 6 adds
fixture files, whose one reader is the fixture runner.

**Narrowing check (canon-kit/SPEC.md §The causal-completeness check, point 5).**
No delta narrows a corpus — no file is pruned, no glob tightened, no path
removed. The subject set is *unchanged*: the same walk, the same prune set, the
same Actions-shape predicate. What moves is the verdict on a subset of it, so the
readers below are enumerated by **red condition** rather than by subject, because
three of them are not monotone.

- `check-action-run-shell` itself — reds on a ShellCheck finding and, after delta
  3, on an unresolved dialect. **Not monotone**, and it is the reader this
  amendment must move in the same commit: delta 5 is what keeps the tree green,
  so deltas 1-4 and 5 are inseparable.
- `check-gate-substrate-parity` — reds when a member's two substrates disagree.
  **Cleared by inspection and by fact:** this member has no shell twin (a
  tree-wide search for a `check-action-run-shell*` script outside `gate-tests/`,
  `checks/` and `docs/` finds none), so there is no second substrate to diverge.
  The parity obligation that does bind is §The port-candidate criteria's
  criterion 4 live-tree arm, and it is discharged against the **pre-change
  binary** — recorded in the Definition of Done rather than assumed here.
- `check-gate-binary-fresh` — reds when the registered binary is older than the
  crate source. **Monotone here and cleared by construction:** delta 1 edits
  crate source, so the binary is stale until `bash gate-sdk/bin/build-native.sh`
  runs, the commit-time obligation CLAUDE.md §Housekeeping already states.
- `check-crate-arms` — reds when the crate's lint or test arms fail. **Not
  monotone**: it asserts arms *pass*, so a new private helper with an unused
  import or a match the linter reads as non-exhaustive reds it. It is the oracle
  for deltas 1 and 2 and runs in the battery.
- `check-gate-output` — reds when a member's source does not carry the canonical
  clean line. **Monotone under an addition** and cleared by inspection: the clean
  line is not touched. Delta 3 deliberately adds **no term** to it — a
  Windows-default block is now a finding rather than a skip, so no counter is
  needed for it, and the existing `block(s) skipped on a non-shell dialect` term
  keeps its exact meaning.
- `check-gate-fixture-coverage` — reds when a registered member has no
  `good/`+`bad/` pair. **Monotone** and already satisfied; delta 6 adds cases to
  an existing pair rather than a pair.
- `check-gate-assertions` / `check-assertion-strength` — red on a fixture whose
  assertion is weaker than its class allows. **Not monotone** over added
  fixtures: delta 6's new cases are held to them, which is why the new `good/`
  cases carry distinguishing bodies rather than empty ones.
- `check-action-pinning` and `check-action-gh-repo` — red on an unpinned `uses:`
  and on a job invoking `gh` with no repository context. **Both hold over delta
  6's new fixture files**, which live under `gate-tests/` and are pruned from
  every whole-tree walk, and over delta 5, which adds no `uses:` and no `gh`
  call.
- `check-graph` and `check-enforcement-fresh` — red when a gate's `# graph:`
  manifest or the generated enforcement projection is stale against the
  registry. **Not monotone** in general and **cleared by inspection here**: no
  coupling, tier or valve value changes, so
  `gate-sdk/checks/check-action-run-shell.gate`'s manifest line is untouched and
  neither projection moves.

## Existing sections updated

- `gate-sdk/SPEC.md` §check-action-run-shell — the **dialect table** is replaced
  by delta 3's two-axis form and the `absent` row's apology paragraph, which
  names this amendment's own slug as the owner of the repair, is deleted
  (delta 3).
- `gate-sdk/SPEC.md` §check-action-run-shell, *The extractor* — the rule list
  gains the job-partition rules and the job-boundary resolution, and the existing
  step-boundary sentence is amended to say it resolves the **explicit** half
  alone (deltas 1 and 2).
- `gate-sdk/SPEC.md` §check-action-run-shell, *The fidelity limit* — a
  `defaults:` subtree carrying `run:` joins the *refused* list with its reason,
  and the zero-instance measurement (delta 4).
- `gate-sdk/SPEC.md` §check-action-run-shell, *Tier / no new knob* — the
  paragraph gains the no-valve ruling and the reason the remedy makes one
  unnecessary (delta 3).
- `gate-sdk/SPEC.md` §check-action-gh-repo — the standing sentence that its walk
  is its own and §check-action-run-shell's extractor is not reusable is amended
  to say **which** pair it now holds between, since the sibling amendment
  answers the same question the other way for a different pair (delta 2). The
  sibling amendment's own delta 1 names this same paragraph, for the opposite
  reason — recording the second consumer this rewrite is actually about;
  whichever batch lands second reconciles one paragraph rather than two.
- `.github/workflows/gates.yml` — the `install-smoke-windows` job's header
  comment (`:162-166`) currently explains the habit by citing the gate's wrong
  resolution: "check-action-run-shell resolves an absent `shell:` to bash …
  right for every job above, wrong here." That sentence becomes false at delta
  3 and is rewritten to say the gate now requires the key on a Windows step
  (delta 3).
- `.github/workflows/publish.yml` — the `build` job's step gains `shell: bash`
  (delta 5). Its `roster` job's runner-map comment (`:44-48`) is re-read at merge
  and left alone unless delta 5 makes it read oddly; the map itself is out of
  this amendment's envelope.
- `gate-sdk/gate-tests/check-action-run-shell/good/expect.txt` and the `bad/`
  expectation — the counts and finding lines move with delta 6's cases
  (delta 6).
- `gate-sdk/gate-tests/check-action-run-shell.test.sh` — the dynamic suite gains
  delta 4's two refusal arms (deltas 4 and 6).
<!-- update-target-exempt: listed because a reader will expect it and it is deliberately unchanged — the descriptor's couplings, tier and valve are all unmoved, so no delta owns it -->
- `gate-sdk/checks/check-action-run-shell.gate` — **no change**: the `# graph:`
  manifest's couplings, `dir=one`, `valve=none` and `tier=precommit` all still
  hold, so neither `scripts/git-hooks/pre-commit` nor the enforcement projection
  is stale.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`), discharged **at the iteration** rather
      than at this commit, the sibling `SPEC-action-permissions.md` being in
      flight (canon-kit/SPEC.md §Merging an amendment, step 3).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles. Specifically: the `absent`-row apology and the
      `gates.yml` header sentence both cite the old resolution and both must go.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **The binary is rebuilt in the same commit** — `bash
      gate-sdk/bin/build-native.sh` beside the battery, neither discharging the
      other (CLAUDE.md §Housekeeping).
- [ ] **Criterion 4's live-tree arm is re-run, not cited** — the pre-change
      binary and the post-change binary are driven from the same cwd with the
      same argv over the live tree and both fixture cases, and every verdict is
      compared. The only permitted difference is the one delta 5 repairs; any
      other moved verdict is a regression in the job layer, which is the risk
      deltas 1 and 2 carry.
