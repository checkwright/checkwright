# SPEC amendment: action-permissions

Closes `workflow-permissions-scope-oracle`. No gate parses a workflow
`permissions:` block, so every token scope a workflow needs is landed on reading
alone. This amendment mints `check-action-permissions` — a gate far narrower
than a workflow-security linter — and, as its second consumer, promotes
§check-action-gh-repo's job-partitioned walk into a shared crate module.

**The failure mode is concrete and this repo supplied it.** A `permissions:`
block is an **allowlist**; an undeclared scope makes the read come back as an
HTTP 404, and a 404 on a read is indistinguishable from an absent resource — the
site-health release-body arm needs `contents: read` and would have reported "no
such Release" without it. On a public repository the omission stays invisible
until a private-repo consumer copies the workflow, which is exactly the shape a
kit ships into.

**Two live in-tree instances, probed at this stage rather than argued from the
class.** `.github/workflows/gates.yml`'s own `gates` job — the battery every
push depends on — checks out at `:20` and declares no `permissions:` block, and
that file carries no workflow-level block either, so nothing is inherited. And
`gate-sdk/templates/gates-workflow.yml`, the **kit template** consumers vendor
into their own `.github/workflows/`, has no `permissions:` block anywhere while
its one job checks out. The second instance is what makes this kit mechanism
rather than a consumer gate: a consumer gate could fix one tree and leave every
downstream vendor of that template unchecked — §check-action-run-shell's and
§check-action-gh-repo's deciding fact, reached here by the same route.

**The count, stated with its scope because a relayed one did not survive
checking.** A tree-wide grep for a `permissions:` **key** (comment lines
excluded) finds **nine** in `.github/workflows/` — one workflow-level in
`site-health.yml`, one workflow-level plus five job-level in `publish.yml`, two
job-level in `gates.yml` — **ten** including `site-kit/templates/site-health.yml`,
and **fourteen** including the four in `check-action-gh-repo`'s fixtures. A
relayed "eight" could be reconstructed only by silently dropping the two
`permissions: {}` blocks, and those are the most relevant ones here:
`publish.yml:32`'s empty workflow-level allowlist is this repo's explicit
read-nothing floor and is what makes its five job-level blocks load-bearing.

**Precedent, stated precisely.** gate-sdk/SPEC.md §check-action-run-shell
declares **GitHub-expression injection** a non-goal and defers it to "a dedicated
workflow-security linter". That is supporting precedent for keeping this gate
narrow — one property, mechanically decidable, tree-local, hermetic — and not a
standing exclusion of the whole workflow-security category.

**The name is `check-action-permissions`, and the naming is already ruled.**
§check-action-pinning refuses `check-workflow-*` for a gate over
`.github/workflows/`: this tree spends "workflow" on §The workflow directory and
`GATE_SDK_WORKFLOW_DIR`, so a second `check-workflow-*` would collide on the
reader's only disambiguator — and `check-workflow-tiering`, which governs the
local `.workflow/` state directory, is that collision already sitting in the
registry. The queue slug keeps its own spelling; a slug is not a gate name.

## What changes

### (1) The job-partitioned walk becomes a shared crate module

`native/src/gates/action_gh_repo.rs`'s `Walk` — its `Ev` event enum, its
`EnvScope`, and the indentation helpers `ind`, `keyof`, `substr_from`,
`dash_prefix_len`, `is_block_scalar`, `marker_reason` and the `gh` command-position
scanner — moves to a new shared module `native/src/actions.rs`, beside `walk.rs`,
`queue.rs`, `spec.rs` and `declaration.rs`. `check-action-gh-repo` and the new
`check-action-permissions` both consume it. **{design-bearing}**

**This is the second consumer that section's standing rule was waiting for, and
the rule is honoured rather than overridden.** §check-action-gh-repo records that
its walk is its own and that §check-action-run-shell's extractor is not reusable,
closing with "a helper earns its place at a second consumer and there is none".
There is now one, and it wants the walk **whole**: job identity and line, step
boundaries, `uses: actions/checkout` recognition with its ordering line, the
`gh` detector with its command-position roster and logical-line joining, the
workflow/job/step `env:` scope ladder, and the indentation-bound valve marker.
Duplicating three hundred lines of state machine to get them is the parallel copy
the content-tiering rule names as the defect.

Two changes make one walk serve two gates:

- **The valve marker is a parameter.** The walk takes the marker token as an
  argument, so `check-action-gh-repo` passes `gh-repo-exempt` and
  `check-action-permissions` passes `action-permissions-exempt`. The
  indentation-binding rule, the required non-empty reason and the bare-marker
  failure class are the walk's and are shared; only the spelling is the caller's.
- **The event stream gains members it did not carry**, listed in delta 2. An
  existing consumer ignores an event it has no arm for, which is why widening the
  stream is additive rather than a change to `check-action-gh-repo`.

**`check-action-run-shell` is deliberately *not* made a consumer, and its sibling
amendment `SPEC-run-shell-dialect.md` states the reasoning from its side.** In
short: that extractor must reassemble a block scalar byte-exactly and refuse four
constructs loudly, where this walk over-detects by design and refuses nothing.
What the two share after that amendment is an *indentation convention*, not a
mechanism, and a helper holding fifteen lines of convention for two
differently-shaped state machines has no invariant to hold. The two amendments
answer the same question opposite ways because what is shared differs, and both
say so where a reader will look.

**The verdict-invariance obligation is what this delta costs.**
`check-action-gh-repo` must produce byte-identical output — findings, counts,
clean line, exit code — on the live tree and on both fixture cases, before and
after the move. This is §The port-candidate criteria's criterion 4 arm applied to
a refactor rather than a port, and it is the whole risk deltas 1 and 2 carry.

### (2) The walk emits the permissions blocks and the token reference

`native/src/actions.rs`'s event stream gains three members. **{design-bearing}**

- **`WorkflowPerms(scopes, line)`** — a column-0 `permissions:` key and the scope
  keys in its subtree.
- **`JobPerms(scopes, line)`** — a `permissions:` key at the job-key column and
  the scope keys in its subtree, attributed to the job in scope.
- **`Token(line)`** — a `secrets.GITHUB_TOKEN` or `github.token` reference
  anywhere in a job's extent.

**Scopes are read as a set of key names with their values, and three value shapes
are recognised**, because GitHub admits all three and a gate that read only the
mapping form would answer wrongly on the other two: a mapping of
`<scope>: read|write|none`; the scalar shorthands `read-all` and `write-all`,
which grant every scope at that level; and the empty mapping `{}` — or a
`permissions:` key with an empty subtree — which grants **nothing**. No other
value is a permissions block the gate understands, and one it does not is a
refusal (delta 4).

**`Token` is a textual match, and its over-detection is the safe direction** —
§check-action-gh-repo's stated bias, inherited with the walk. A job that mentions
the token in a comment arms the gate and its author writes one `permissions:`
block or one valve; a job that consumes it silently is the failure this gate
exists for. `secrets.NPM_TOKEN` and every other secret are **not** matched: they
are not the GitHub token and carry no `permissions:` scope. `publish.yml`'s `npm`
job is the in-tree case that proves the distinction matters — it references
`secrets.NPM_TOKEN` and nothing else, and is correctly inert.

### (3) The invariant: a token-consuming job declares what it takes

`check-action-permissions` is registered. In every Actions-shaped YAML file, a
**job** that consumes the GitHub token has its scopes **declared** rather than
inherited from an invisible repository default. **{design-bearing}**

**The trigger — a job is armed by any one of three, and the set is the same
detector the walk already runs for its sibling:**

1. a step whose `uses:` ref before the `@` is `actions/checkout`;
2. a `gh` invocation in one of the job's `run:` bodies;
3. a `secrets.GITHUB_TOKEN` or `github.token` reference in the job's extent.

**The scopes in scope for a job** are its own `JobPerms` where it has one, and
otherwise the file's `WorkflowPerms`. A job-level block **replaces** the
workflow-level one rather than adding to it — GitHub's own rule — so a job
declaring `id-token: write` alone inherits no `contents:` from a workflow-level
block that names it. **Modelling the inheritance is load-bearing, not
completeness:** `.github/workflows/site-health.yml`'s `probe` job declares no
block of its own and takes `contents: read` from workflow level, so a gate
implementing only the job-level lookup would red a correct live file on day one
and read as a false positive — the same shape §check-action-gh-repo's step-level
`GH_REPO` lookup has, and named here for the same reason.

**Two arms, disjoined by what the trigger proves:**

- **Arm A — `contents:`, where the consumption is exact.** A job armed by trigger
  1 must have `contents:` at `read` or `write` in scope (`write` satisfies
  `read`; `read-all` and `write-all` satisfy both). A checkout fetches repository
  contents, so the scope is not a guess and the gate may name it.
- **Arm B — a declaration, where the scope depends on the call.** A job armed by
  triggers 2 or 3 and not by 1 must have a **non-empty** set of scopes in scope.
  Which scope a given `gh` call consumes depends on the subcommand, and a
  verb-to-scope map is a **vocabulary** — unbounded, provider-versioned, and the
  kind of rule content a kit must not ship as a literal (CLAUDE.md §The
  provenance seam). So this arm asserts that the job *says* what it takes, which
  is the whole difference between a reviewed allowlist and a repository default
  nobody in the tree can see.

A job armed by 1 takes arm A, which is the stronger; the arms are not mixed
within a job. A job armed by none is **inert and counted**, the counted inertness
that makes this kit mechanism: a consumer running no GitHub Actions, or none that
touches the token, pays nothing for it.

**`permissions: {}` fails both arms, and that is the gate working rather than a
harshness to soften.** An empty allowlist grants nothing, so a job that checks
out under it has no `contents:` and fails at runtime — on a private repository
immediately, and on a public one only once the content stops being anonymously
readable. That silence is the entry's core observation, and the `bad/` fixture
opens on it.

**Composite actions are skipped and counted.** §check-action-gh-repo's split
predicate is consumed unchanged: a top-level `jobs:` key makes a file the
subject; a `runs:`-shaped composite action has no job and no `permissions:` of
its own, inheriting the calling job's, so the assertion belongs to the caller.

**A reusable-workflow call (`jobs.<id>.uses:`) is inert**, having no `steps:` for
any trigger to fire in. Its `permissions:` are passed to the called workflow,
whose own jobs are held by this gate in the file that defines them — the same
boundary §check-action-run-shell draws for a called workflow's `run:` bodies.
Stated because the silence would otherwise read as an oversight.

**Output.** Clean is one line naming what was checked — armed jobs, inert jobs,
Actions-shaped files, walked files and files skipped by the predicate. A finding
names the file, the job id, its line and which arm failed; the two arms are two
failure classes and take **two `help:` lines** (§Output contract), one naming
`contents: read` and one naming the declaration. The bare-marker class inherited
with the walk keeps its own third.

**The valve.** `# action-permissions-exempt: <reason>`, the kit's established
marker shape, reason **required and non-empty**, binding by indentation exactly
as its sibling's does: at or left of the job-id column it precedes a job, at the
step-list dash column it precedes a step, inside a step's keys it binds that
step. A job-bound marker skips the job; a step-bound one drops that step's
evidence from the trigger set and leaves the job's other evidence held. A valve
is minted here — unlike in `SPEC-run-shell-dialect.md`, where it is refused —
because the remedy is **not** always available: the `gh` detector over-detects by
design, and a job reaching the token through a third-party action's own
machinery is outside this gate's theory.

**Tier `precommit`; no new knob.** The scan set is derived — `walk::find_files`
for `*.yml`/`*.yaml` from the scan root, the shared prune set — and the asserted
scope name is GitHub's own published vocabulary, not rule content a consumer
could own differently. A knob here would be a scope-roster surface with no second
consumer and nothing private to externalize, which is the test §Layout and
configuration's config-via-env convention sets rather than a licence to add one.

### (4) The refusals, and the fidelity limits that ship with them

Exit 2, naming the construct, and only inside the Actions-shape subject: a
`permissions:` value that is neither a mapping, nor `read-all`/`write-all`, nor
empty — a `${{ }}` interpolation, an anchor or an alias. The gate cannot resolve
what such a block grants and does not guess. **{design-bearing}**

*Out of ability, stated as cost rather than papered over.* A **third-party action
that consumes the token implicitly** — passed through its own `with:` inputs or
read from the environment — is not detected; the trigger set is checkout, `gh`
and an explicit token reference, and widening it by guessing at action names
would be a maintained roster of someone else's software. A **`curl` to
`api.github.com`** is likewise undetected: it is a bare URL in a shell body with
no token binding the gate can see. Both take the valve, and both are the honest
limit of a tree-local reader.

*Not asserted, deliberately.* That a declared scope is **not wider than
necessary**. Over-declaration is a real hazard and it is a different gate: it
needs the verb-to-scope map arm B refuses to ship, and asserting a minimum is
mechanically decidable where asserting a maximum is not. Named so a later reader
does not read the silence as an omission.

### (5) The two in-tree instances are repaired

`.github/workflows/gates.yml`'s `gates` job gains `permissions:` with
`contents: read`, matching the two install-smoke jobs beside it (`:85-86`,
`:177-178`). `gate-sdk/templates/gates-workflow.yml`'s `gates` job gains the same
block. These are the whole of the tree's findings under delta 3.
**{mechanical}**

`contents: read` is exact for both: each job checks out and neither invokes `gh`,
references the token, or writes anything through the API. Verified per job at
this stage across all three live workflows and both kit templates — every other
armed job already declares a satisfying block, whether its own
(`publish.yml`'s five, `gates.yml`'s two) or an inherited workflow-level one
(`site-health.yml`'s `probe`, `site-kit/templates/site-health.yml`'s).

**The template and its filled instance are a hand-mirrored pair with nothing
holding them together, and this delta widens that exposure by one block rather
than creating it.** `check-template-copy-parity` globs `*/templates/*.sh`, so
this YAML pair is uncovered; the gap is owned by the deferred
`template-copy-parity-yaml-widening`, cited here rather than re-diagnosed, and
this delta is a second instance for whoever promotes it.

### (6) The landing set

The full kit-landing and new-gate fan-out, assembled here so the build reads it
rather than discovering it one red gate at a time. **{mechanical}**

- `gate-sdk/checks/check-action-permissions.gate` — the descriptor, whose
  `# graph:` line is the `action-*` cohort's verbatim (`check-action-pinning`,
  `check-action-run-shell` and `check-action-gh-repo` all carry the identical
  `couples=` value), with `dir=one`, `tier=precommit` and `valve=` naming delta
  3's marker rather than `none`.
- `scripts/gates.list` — one row, beside its cohort at `:41-43`.
- `native/src/gates/action_permissions.rs` — the module, named by the crate's own
  gate-name-to-module derivation.
- `native/src/gates/mod.rs` — a `pub mod` line and a `REGISTRY` entry in the
  cohort's shape (`&[("?", "")]` scan root, `&["GATE_PRUNE_DIRS"]`, owning kit
  `gate-sdk`, no declared external program). `native/src/main.rs` needs no edit:
  its dispatch resolves any registered name through `gates::lookup`.
- `gate-sdk/gate-tests/check-action-permissions/{good,bad}/` — the fixture pair.
  The `bad/` tree opens on the attested miss itself: a `gates`-shaped job that
  checks out under no block at all, plus a job under a workflow-level
  `permissions: {}`, plus a job invoking `gh` under an empty declaration, plus a
  bare valve marker. The `good/` tree carries a job satisfied by its own block, a
  job satisfied by inheritance, a job satisfied by `read-all`, a job whose
  job-level block *replaces* a workflow-level one and still satisfies, an inert
  job that neither checks out nor touches the token, a composite action, and both
  valve bindings.
- `gate-sdk/README.md`'s `<!-- gate-roster:begin -->` block — the gate joins it.
- The generated projections a new gate stales, each with its regen command in
  docs/site-architecture.md §Generated projections: the on-site SPEC mirror, the
  enforcement map (`docs/enforcement.md`), `docs/value.md`'s rollup block,
  `docs/check-graph.html`, and — this being a `precommit`-tier gate — the
  generated hooks. **Two of these are staging-ordered**: the generated hooks and
  the gate binary both derive through `git ls-files`, so this unit stages its new
  files first and regenerates second.

## Producers and consumers

**`native/src/actions.rs` (delta 1)** is a new module, the only new crate-level
interface here.

- *Producer:* nothing produces it; it is code. Its **walk** produces the event
  stream, from `walk_file(text, marker)` called once per Actions-shaped file by
  each consuming gate. **Enabling config actually set:** none — the module takes
  no knob, its callers' scan sets are derived and the prune set is the shared
  one, so the module is live on the first battery run after the build rather than
  reachable only from tests.
- *Consumers,* two, each by direct call and each named: `action_gh_repo::run`,
  which consumes the stream it consumes today and ignores delta 2's new members;
  and `action_permissions::run`, which consumes `Job`, `Step`, `Checkout`, `Gh`,
  the two `*Perms` members, `Token`, the two exempt members and `BareMarker`.
- *Named reader for every field:* the marker-token parameter's reader is
  `Walk::marker`, at every comment line. `WorkflowPerms`/`JobPerms` carry a scope
  set and a line: the scope set is read by delta 3's two arms at the job's close,
  the line by the finding formatter when an arm fails. `Token` carries a line,
  read by the trigger evaluation at the job's close and by no formatter — the
  finding names the **job**, not the evidence line, so **the `Token` line number
  has no reader and is dropped**; the event is a bare marker that the job is
  armed. Recorded rather than silently omitted, because carrying it would be a
  field populated at one transition and read at none.
- *Reader that moves rather than arrives:* the `Command`-absent roster test
  (§Fail-closed contract) asserts its subject over every module under
  `native/src/gates/`. `native/src/actions.rs` leaves that corpus. It spawns
  nothing today and must not start — the property is preserved by the crate's
  single-spawn-site rule in `proc.rs`, which is where a spawn would have to be
  added, and this is the reader most likely to be missed by a move.

**`check-action-permissions` (deltas 3 to 6)** produces a verdict and no in-tree
state.

- *Producer:* the gate binary's subcommand, invoked by the registry through
  `gate_command`.
- *Consumers,* three, all existing: `gate-sdk/bin/run-gates.sh` reads the exit
  code; `gate-sdk/bin/run-gate-tests.sh` reads the clean line and the `help:`
  lines as the fixture pair's assertions (§Output contract's runtime half); and
  `scripts/git-hooks/pre-commit` reaches it through the generated projection,
  which is why delta 6 regenerates the hooks rather than leaving them stale.
- *Named reader for the descriptor's every field:* `couples=` is read by
  `check-graph` and `check-reads-couples`; `tier=` by `gen-pre-commit.sh` and the
  enforcement map; `valve=` by `check-gate-exemption-tasks`; `dir=` by
  `check-graph`. The `# install:` line is read by `check-install-disposition`,
  and the `# spec:` line by `check-spec-pointer` — which resolves the heading, so
  §check-action-permissions must exist in `gate-sdk/SPEC.md` in the same commit
  as the descriptor.

**Narrowing check (canon-kit/SPEC.md §The causal-completeness check, point 5).**
No delta narrows a corpus: no file is pruned, no glob tightened, no tracked path
removed, and the new gate's scan set is the cohort's existing derived one. The
readers that move are enumerated by **red condition** anyway, because several are
not monotone under an addition:

- `check-action-gh-repo` — reds on a job invoking `gh` with no repository
  context. **Not monotone under delta 1**, which rewrites how it parses: any
  verdict change is a regression, which is why verdict-invariance is asserted in
  the delta and re-run in the Definition of Done rather than reasoned about.
- `check-action-permissions` itself — reds on delta 3's two arms and on a bare
  valve marker. **Not monotone**, and delta 5 is what keeps the tree green, so
  deltas 3 and 5 are inseparable in one commit.
- `check-crate-arms` — reds when the crate's lint or test arms fail. **Not
  monotone**: a module move plus a new module is exactly the shape that produces
  an unused import, a dead-code warning on an event member one consumer ignores,
  or a non-exhaustive match. It is the oracle for deltas 1 and 2.
- `check-gate-binary-fresh` — reds when the binary is older than the crate
  source. **Monotone and cleared by construction:** `bash
  gate-sdk/bin/build-native.sh` in the same commit, the standing obligation.
- `check-gate-output`, `check-gate-fail-closed`, `check-gate-fixture-coverage`,
  `check-gate-assertions`, `check-assertion-strength`, `check-install-disposition`
  — each reds when a **registered** member fails its contract. **All non-monotone
  under an addition**, because the new registry row is a new obligation for every
  one of them: the clean line, the refusal posture, the fixture pair, and the
  `# install:` disposition all have to land with the row, not after it. This is
  the set a new-gate unit most often discovers one red at a time.
- `check-graph`, `check-reads-couples`, `check-enforcement-fresh`,
  `check-readme-roster`, `check-battery-roster` — red when a projection or roster
  is stale against the registry. **Not monotone under an addition** and the
  reason delta 6 exists as a delta rather than a checklist line.
- `check-spec-pointer` — reds when a `# spec:` pointer resolves to no heading.
  **Not monotone**: the descriptor's pointer dangles until
  §check-action-permissions exists, which is why the SPEC section is an update
  target below rather than a follow-up.
- `check-comment-tier` and `check-rule-citation` — red on a comment restating
  rather than citing. **Not monotone** over the moved `# spec:` comments in delta
  1: each one cites §check-action-gh-repo today and several now bind to shared
  code, so they are re-pointed at the section that owns the moved rule rather
  than carried across unchanged.
- `check-template-copy-parity` — reds when a registered template/copy pair
  diverges undeclared. **Cleared by inspection, and the clearance is the
  finding:** its glob is `*/templates/*.sh`, so the YAML pair delta 5 edits on
  both sides is outside its corpus entirely. That is `template-copy-parity-
  yaml-widening`'s subject, not this amendment's.

## Existing sections updated

- `gate-sdk/SPEC.md` — a **new** `### check-action-permissions` section carrying
  the invariant, the trigger set, the two arms, the inheritance rule, the valve,
  the refusals and the stated limits, placed with its `action-` cohort after
  §check-action-gh-repo (deltas 3 and 4).
- `gate-sdk/SPEC.md` §check-action-gh-repo — the paragraph beginning "The walk is
  this gate's own" is rewritten: the walk moves to `native/src/actions.rs` under
  its second consumer, the marker spelling becomes a parameter, and the sentence
  that no second consumer exists is retired. Its companion claim — that
  §check-action-run-shell's extractor is not reusable — **stays true and stays**,
  now scoped to the pair it holds between (delta 1). Its sibling amendment
  `SPEC-run-shell-dialect.md` names the same paragraph from its own delta 2, for
  the opposite reason — recording that §check-action-run-shell is *not* the
  second consumer this rewrite is about; whichever batch lands second reconciles
  one paragraph rather than two.
- `gate-sdk/SPEC.md` §check-action-gh-repo — the detector, the checkout arm, the
  `env:` ladder and the valve's indentation binding are now **shared mechanism**
  described in one place; that section keeps them as its own rule and cites the
  module, and the new section cites that description rather than restating it
  (deltas 1 and 2).
- `gate-sdk/SPEC.md` §check-action-run-shell — the standing "a helper earns its
  place at a second consumer and there is none" sentence is amended to say which
  pair it still holds between, now that a second consumer has arrived for the
  *other* walk (delta 1). This is this amendment's edit alone: the sibling
  amendment's own delta 1 states this sentence "survives this delta intact",
  so no reconciliation is owed here — unlike the §check-action-gh-repo paragraph
  above, which both amendments do edit.
- `gate-sdk/SPEC.md` §Fail-closed contract — the `Command`-absent roster test's
  stated corpus is "every module under `native/src/gates/`", and delta 1 moves
  code out of it; the paragraph gains the one-line statement that
  `native/src/actions.rs` is shared gate mechanism outside that corpus and why the
  property still holds (delta 1).
- `site-kit/SPEC.md` — the release-body arm's paragraph motivating
  `site-health.yml`'s `permissions:` declaration currently closes "No gate
  parses a `permissions:` block (workflow-security linting is an explicit
  non-goal, gate-sdk/SPEC.md §check-action-run-shell), so this one is held by
  review rather than by an oracle" — the exact in-tree instance this amendment's
  own opening cites. That sentence is retired: `check-action-permissions` is the
  oracle it says does not exist, and it is a **cross-kit** citation this
  amendment must repair since site-kit does not own gate-sdk's roster (delta 3).
- `gate-sdk/README.md` — the `<!-- gate-roster:begin -->` block gains the row
  (delta 6).
- `scripts/gates.list` — the registration row (delta 6).
- `.github/workflows/gates.yml` and `gate-sdk/templates/gates-workflow.yml` — the
  `gates` job gains its `permissions:` block in both (delta 5).
- `docs/enforcement.md`, `docs/value.md`'s rollup block, `docs/check-graph.html`,
  the on-site SPEC mirror and `scripts/git-hooks/pre-commit` — regenerated, never
  hand-edited, each by the command docs/site-architecture.md §Generated
  projections states (delta 6).
<!-- update-target-exempt: named because a reader will look for it and it is deliberately untouched — the site page states the fan-out this amendment consumes, and consuming a roster does not change it -->
- `docs/site-architecture.md` §Generated projections — **no change**: delta 6
  reads its new-gate fan-out and adds no projection of its own.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`), discharged **at the iteration** rather
      than at this commit, the sibling `SPEC-run-shell-dialect.md` being in
      flight (canon-kit/SPEC.md §Merging an amendment, step 3).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles. Specifically: every citation of the walk as
      `check-action-gh-repo`'s own, the "there is none" clause, and — found by
      the same grep discipline turned cross-kit — `site-kit/SPEC.md`'s "No gate
      parses a `permissions:` block" sentence.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **`check-action-gh-repo`'s verdicts are proved unchanged, not reasoned
      unchanged** — the pre-move and post-move binaries are driven from the same
      cwd with the same argv over the live tree and both fixture cases, and the
      output is compared byte for byte including exit codes. This is delta 1's
      whole risk and the one assertion no gate makes for it.
- [ ] **The binary is rebuilt in the same commit** — `bash
      gate-sdk/bin/build-native.sh` beside the battery, neither discharging the
      other (CLAUDE.md §Housekeeping).
- [ ] **The new-gate fan-out is regenerated in staging order** — the new files
      are staged before the hooks and the binary are regenerated, both deriving
      through `git ls-files` (docs/site-architecture.md §Generated projections).
