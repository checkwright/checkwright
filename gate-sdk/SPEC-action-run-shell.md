# SPEC amendment: action-run-shell

Promotes the queue entry `workflow-run-block-lint`. It adds one gate-sdk gate,
`check-action-run-shell`, which extracts the shell out of GitHub Actions `run:`
blocks and lints it under ShellCheck at the gate family's `-S warning`.

**The gap it closes.** No oracle executes or lints workflow shell at all.
`check-shellcheck` builds its target list as `targets+=("$d"/*.sh)` over the
consumer gates dir and each kit's `lib/`, `bin/`, `checks/`, `templates/`, so
`.github/workflows/*.yml` is unreached **by construction** — the `run:` blocks
are shell that nothing lints, shellchecks, or executes outside a tag push. The
`v0.16.0` publish failure is the class's evidence: a careful by-eye review of
that exact line passed it and the defect reached a released tag.

**Provenance seam.** Nothing private crosses. GitHub Actions' `run:`/`shell:`
keys and YAML's block-scalar grammar are public vocabulary; the gate ships no
term list and adds no config knob.

## The operator ruling this amendment implements

The queue entry recorded its YAML-extraction question as unresolved and said
`/spec` surfaces but does not settle it. It has since been ruled by the
operator, and the ruling is recorded here rather than re-argued:

> **An awk extractor with a stated fidelity limit.** Extract `run: |` block
> scalars with awk keyed on block-scalar indentation, lint each under ShellCheck
> at the gate family's `-S warning`, and ship a fixture that *proves where the
> extractor gives up*. The dependency-light floor stays intact — no YAML parser,
> no language runtime, `PROBE_SET` unchanged.

Rejected with cause, so the boundary is not redrawn by the next reader: a real
YAML parser (the local floor grows a runtime, and the skip-vs-red question on a
bare box becomes a new decision nobody wanted); and narrowing to a
hazard-pattern check, which would dissolve this entry into `publish-spec-gate`
and leave the other blocks unlinted — the class that let a defect reach a tag.

**The fidelity limit is a deliverable, not a caveat.** §C states what the
extractor does not handle and §D pins each limit in the fixture, so a later
reader learns the boundary from the oracle rather than discovering it as a false
negative.

## What changes

Every delta carries its work class. **mechanical** — executing it demands only
oracle-running (a fixed battery, a substitution sweep, a regen command).
**design-bearing** — executing it demands generative or verificational judgment.

Every quantitative and behavioral claim below was measured against the current
tree during authoring, with a working prototype of the extractor; the counts and
the ShellCheck findings are observations, not projections.

### A. Placement and name

**A1. gate-sdk mechanism, not a consumer gate. {design-bearing}** It ships in
`gate-sdk/checks/`. The reasoning is `check-action-pinning`'s, continued, and it
turns on one fact: **two kits ship a workflow template carrying `run:` shell** —
`gate-sdk/templates/gates-workflow.yml` (one block) and
`site-kit/templates/site-health.yml` (two), the copy-outs consumers vendor. A
consumer gate under `scripts/` could reach both in *this* tree while leaving them
unlinted for everyone downstream, which is the same "fix the instance, ship the
drift source" shape `check-action-pinning` rejected. That the second template
belongs to a *different kit* is what makes the argument decisive rather than
convenient: no consumer gate and no site-kit-local gate covers both, while a
gate-sdk gate scanning by shape (`B2`) covers them with no roster to drift. Second, a `run:` block is shell in
anybody's workflow — unlike an `npm publish` spec, which is this repo's product
knowledge and stays a consumer gate (`SPEC-publish-spec-gate.md` §A). Third, the
"GitHub Actions is not universal" objection is a *reach* question, and gate-sdk
answers reach with counted inertness: a tree with no workflow file exits clean
with a zero count.

**A2. Named `check-action-run-shell`, not `check-workflow-*`. {mechanical}** In
this tree "workflow" already means the `.workflow/` directory —
`check-workflow-tiering` gates it and `GATE_SDK_WORKFLOW_DIR` configures it — so
a second `check-workflow-*` gate over `.github/workflows/` would collide on the
reader's only disambiguator. `action-` is the prefix `check-action-pinning`
established for this surface.

### B. The gate

**B1. Invariant. {design-bearing}** Every GitHub Actions `run:` **literal block
scalar** in a scanned YAML file is ShellCheck-clean at `-S warning` under the
dialect the step actually runs.

**B2. Scan set — derived, then narrowed to the gate's subject.
{design-bearing}** Two stages, and the second is a ruling rather than a
mechanism.

*Stage one, the walk.* A whole-tree walk over `*.yml` / `*.yaml` via
`gate_find`, which prunes through the shared `GATE_SDK_PRUNE_DIRS` — `gate-tests`
among them, so the `bad/` fixture cannot red the whole-tree run. The same
positional `[scan-root]` argument reaches a synthetic tree.

*Stage two, the Actions-shape predicate.* A walked file enters the gate's subject
only if it carries a **top-level `jobs:` key** (a workflow) or a **top-level
`runs:` key** (a composite action). Everything else is **skipped and counted**,
the same visible-cost mechanic §C2 uses for plain scalars. The predicate governs
**extraction as well as refusal** — a non-matching file is neither linted nor
refused.

Whole-tree reach was the authoring draft, borrowed from `check-action-pinning`,
and it does not survive the audit. `uses:` with a 40-hex ref is self-limiting
grammar; `run:` is an ordinary word serving as a YAML key in more than one CI
schema. Under whole-tree reach the gate would *lint* a `run: |` block found in a
foreign schema as though it were shell, which is a stream of silent false
positives against text that is not shell — strictly worse than the loud
`C1` refusal, and squarely what gate-sdk/SPEC.md §When a gate earns its place
forbids. Two further reasons the narrowing is right rather than merely safe: the
gate's own name asserts its subject, and a gate whose name and reach disagree
teaches every later reader the wrong boundary; and gate-sdk has no standing to
impose `C1`'s literal-form conformance rule on a consumer's non-Actions YAML —
inside a workflow the kit ships the template and owns the contract, outside one
it would be reddening a file the kit never claimed, whose likeliest remedy is
deleting the gate and losing the whole class.

*Re-derived at the audit stage against the live prune set, not against a bare
grep.* The walk yields **eight** files; the predicate admits **five** — the three
under `.github/workflows/` plus both vendored copy-out templates
(`gate-sdk/templates/gates-workflow.yml`, `site-kit/templates/site-health.yml`) —
and skips three (`.github/ISSUE_TEMPLATE/config.yml`,
`.github/ISSUE_TEMPLATE/gate-defect.yml`, `docs/_config.yml`). Those five carry
**eight** literal block scalars: `gates.yml` 1, `publish.yml` 2, `site-health.yml`
2, `gates-workflow.yml` 1, `site-kit/templates/site-health.yml` 2. **The
predicate loses no block and no plain scalar** — every one sits in a matching
file, verified by running the extractor over the skipped set and getting nothing.

Two earlier counts were low, each for the same reason: the queue entry's five
surveyed `.github/workflows/` only, and the authoring count of six reached
gate-sdk's template but not site-kit's. **The number is measurement, never
contract** — the sibling `release-tarball-delivery-channel` unit adds a ninth
block to `publish.yml` this same iteration — so gate-sdk/SPEC.md carries the
derivation and the predicate, never a count a later commit falsifies.

The `# graph:` manifest (`B8`) couples the **walked** surface rather than the
matching one: a file that *gains* a top-level `jobs:` key must retrigger the
gate, and a manifest naming only today's workflows would miss exactly that.

**B3. The extractor. {design-bearing}** One awk pass per file, keyed on
block-scalar indentation. Four rules, each of which a prototype proved necessary
by failing without it:

- **The key column is the column of the `run` token, not of the list dash.** A
  `- run: |` item's dash sits left of the key; taking the dash's column as the
  block's indentation floor makes every sibling key of that step (`env:`,
  `name:`) satisfy "more indented than the block header" and be swallowed into
  the shell body — and dedented by the body's indent, so `env:` arrives as `v:`.
  Measured: the naive form emitted `echo body-line` / `v:` / `FOO: bar` /
  `me: sibling keys…` as one shell fragment. That is not a missed block, it is a
  false-positive engine, and it is the single most important line in the
  extractor.
- **The body is every following line more indented than the key column**, blank
  lines included, dedented by the first body line's indentation. It ends at the
  first non-blank line at or left of the key column.
- **No block header is recognised while inside a block.** Measured: a body
  containing a heredoc whose text is literally `run: |` is kept as shell rather
  than double-extracted.
- **A comment line is never a header.** `#     run: bash …` in the shipped
  template's placeholder block is a commented example, not a step.

**B4. GitHub expressions — the placeholder is `${GHEXPR}`, measured.
{design-bearing}** `${{ … }}` is not shell syntax; left raw it is a parse
error. Measured on `echo ${{ github.ref_name }}` plus an `if [ … ]` line:
ShellCheck reports SC2296 (error) and SC1083 (warning) per expression.

The substituted token must not itself alter the parse, and the obvious choice
fails that test. Both candidates were measured at `-S warning`:

| substitution | result |
| --- | --- |
| bare word `GHEXPR` | manufactures **SC2050** ("this expression is constant") in `[ … ]` and **SC2194** ("this word is constant") in `case` |
| `${GHEXPR}` | **no substitution-caused finding** in any tested position |

Ruled: **`${{ … }}` is replaced by `${GHEXPR}`**, a braced parameter expansion.
It presents to ShellCheck as an opaque runtime value, which is exactly what a
GitHub expression is; a bare word presents as a literal constant and drags
ShellCheck's constant-expression analysis into firing on correct code. The
substitution is per line, and a line carrying an unbalanced `${{` is refused
loudly under `C1` rather than linted mangled.

**B5. Dialect — resolved, never assumed. {design-bearing}** Linting a block
under the wrong dialect manufactures false positives, so the gate resolves the
step's effective shell from its `shell:` sibling key rather than assuming one:

- **absent** → `-s bash`. GitHub's documented default for a `run:` step on every
  hosted runner is `bash -e {0}`. Measured: no `shell:` key exists anywhere in
  this tree, so all six blocks resolve to bash today.
- **`bash` (with or without arguments)** → `-s bash`.
- **`sh` / `dash` / `ksh`** → the matching ShellCheck dialect, which it supports
  natively. Linting a POSIX body as bash would hide exactly the portability
  findings that dialect exists to surface.
- **anything else** (`pwsh`, `powershell`, `python`, `cmd`, a custom `{0}`
  template) → the block is **skipped and counted** in the gate's output line.
  This is not a false-negative hole: the body is not shell, so there is no shell
  to lint. Counted so the skip is visible rather than silent.

**B6. Severity — `-S warning`, and why not lower. {design-bearing}** The gate
family's level, matching `check-shellcheck`, so one threshold governs all
ShellCheck lint in the tree.

Measured: all **eight** in-tree blocks (`B2`) are **clean at `-S warning`
today**, so the gate lands greenfield — it proves the boundary rather than
clearing a backlog. At `-S style` five findings appear, and **two** of them are
an artifact of extraction rather than a property of the code: **SC1091** ("not
following: … was not specified as input") fires on each of the two blocks that
`source gate-sdk/lib/gate.sh`, because an extracted fragment has no resolvable
source root. Both sit at `info`, below the threshold, so they do not bite — but
a future author lowering the threshold inherits false positives the gate
created. The remaining three are SC2016, a property of the workflows' own
single-quoted `printf` formats rather than of extraction. Recorded so that
decision is made knowingly.

**B7. The extractor stays inline. {mechanical}** It lives in the check script,
not in `gate-sdk/lib/`. A `lib/` helper earns its place at a second consumer and
there is none: the sibling `check-npm-publish-spec` deliberately needs no
extraction (`SPEC-publish-spec-gate.md` §A3), and coupling the two would
serialize a critical-path publish fix behind this gate.

**B8. Gate contracts. {mechanical}** `precommit` tier; a `# graph:` manifest
coupling the scanned YAML surface; a `# spec:` header binding this section's
successor; the output and fail-closed contracts per gate-sdk/SPEC.md §The gate
model, including the ShellCheck-absent exit 2 `check-shellcheck` already models.
Registration in `scripts/gates.list` and a row in `gate-sdk/README.md`'s roster
block, which `check-readme-roster` holds bidirectionally against
`gate-sdk/checks/`. **No new knob**: the scan set is derived, the prune set is
the shared one, and the severity is the family's literal.

### C. The fidelity limit

This is the section the ruling asks for. Three classes, and the distinctions are
the whole point: what the extractor **refuses loudly** can never become a false
negative, what is **out of reach** is a stated cost, and what is **out of
subject** is a boundary the kit declines to cross.

**C1. Refused loudly — exit 2, naming the construct. {design-bearing}** Each of
these is detectable, so the gate stops rather than guessing. **Every refusal here
fires only inside `B2`'s subject** — a construct in a file the Actions-shape
predicate skipped is not refused, because the gate never read it as shell:

- **A folded block scalar** (`run: >`, `run: >-`). Reassembling folded lines
  needs YAML's folding rules, and mis-folding manufactures findings. Refusing it
  also makes the literal form a conformance requirement: a multi-line `run:`
  body **in an Actions-shaped file** must be `|`, so it is linted. That
  requirement is governance where gate-sdk ships the template and owns the
  contract, which is exactly the reach `B2` confines it to.
- **An explicit block-scalar indentation indicator** (`run: |2`). The extractor
  derives the body indent from the first body line; an explicit indicator can
  contradict it.
- **A YAML anchor or alias** as the `run:` value (`run: *setup`). No anchor
  resolution is attempted.
- **An unbalanced `${{` on a body line**, per `B4`.

Measured with the prototype, and re-measured across the whole set at the audit
stage: `>`, `>-`, `|2`, and `*alias` were each detected and refused; `|-` and
`|+` (chomping indicators, ordinary spellings) are handled and extracted
normally, body bytes intact — silently skipping those would have been the worst
hole of the set, since an author reaches for `|-` by habit.

**C2. Out of reach, and out of subject — stated cost. {design-bearing}** Two
different things, kept apart because the remedies differ: an **ability** limit is
something the extractor cannot do, while a **subject** limit is something the
gate declines to claim.

*Out of subject (`B2`'s predicate).* **Other CI dialects that also spell a shell
step `run:`.** CircleCI's `.circleci/config.yml` is the concrete case —
`- run: |` there is genuinely shell, and the extractor would handle it correctly.
It is skipped anyway, because gate-sdk ships no CircleCI template and owns no
contract over that file; linting it would be an unrelated vendored gate reddening
a consumer's battery over a surface the kit never claimed. This is a **decision,
not an inability**, and the distinction matters to whoever revisits it: widening
the predicate is a governance question about what the kit claims, never an
engineering question about what the extractor can parse. A consumer-configurable
predicate is the honest generalization and is filed rather than built here.

*Out of ability.* The genuine limits:

- **Single-line plain-scalar `run:` values** (`run: bash gate-sdk/bin/run-gates.sh`).
  Not linted. A plain scalar's text is governed by YAML's plain-scalar rules —
  a space-preceded `#` opens a *YAML* comment, not a shell one — so recovering
  the shell honestly means parsing the scalar, which is the dependency the
  ruling declined. Re-derived at the audit stage over `B2`'s admitted set:
  **four** such lines (`gates.yml` 31/33/45, `gates-workflow.yml` 39), each a
  single command with no control flow, and none of them lost to the predicate.
  The class that produced the incident is multi-line blocks. The gate counts
  these in its output as skipped so the cost is visible on every run.
- **Reusable workflows.** A called workflow's blocks are linted in the file that
  *defines* them, since that file carries its own top-level `jobs:` key and so
  satisfies `B2`; what is out of reach is following a `uses:` call into another
  repository, which no tree-local gate can do. Composite actions are **not** in
  this class — `B2`'s `runs:` arm admits an `action.yml`, and `D1`'s fixture is
  where that arm is exercised, the tree carrying none today.
- **GitHub-expression injection.** `B4`'s substitution turns an interpolation
  into an opaque expansion, so the gate cannot see the injection hazard of an
  unquoted `${{ }}` — a textual substitution that happens before the shell ever
  runs, and a genuinely different and worse class than an unquoted variable.
  This is a **non-goal**, not an oversight: it belongs to a dedicated
  workflow-security linter. This repo's standing mitigation is unrelated and
  already in force — every workflow routes expressions through `env:`, which is
  why no in-tree `run:` body contains a `${{ }}` at all.
- **What this gate would not have caught.** Measured, and stated plainly because
  the entry sits beside the incident that motivated it: the `v0.16.0`
  `"$(ls dist/*.tgz)"` line is **clean at `-S warning`** — and, re-measured at
  the audit stage, clean at `-S style` too, so lowering `B6`'s threshold would
  not recover it either. The defect is npm's spec-resolution grammar, which
  ShellCheck has no theory of at any severity. `check-npm-publish-spec` is the
  gate that catches that defect; this one closes the *class* of unlinted
  blocks around it. Neither subsumes the other, and reading this gate as the
  incident's fix would be a false comfort.

### D. The fixture pair

**D1. `good/` pins the limit, not merely the pass. {design-bearing}** A
synthetic workflow tree carrying, at minimum: a `|` block that lints clean; a
`|-` block, so a regression that silently drops chomped blocks turns `good/` red;
a `- run: |` dash-inline block **followed by sibling `env:` and `name:` keys**,
which is `B3`'s swallow case and reds the moment the key-column rule regresses;
a body containing a heredoc whose text is `run: |`; a commented-out `run:` line;
a body carrying `${{ … }}` in both bare and `[ … ]` positions, which reds under
the `B4` bare-word substitution and passes under `${GHEXPR}`; a `shell: sh` step
whose body is POSIX-clean; and a `shell: pwsh` step, asserted **skipped and
counted** rather than linted.

It also carries the fixture that makes `B2`'s reach ruling durable rather than a
comment: **a non-Actions YAML — no top-level `jobs:` or `runs:` key — carrying a
`run: >` block**, the very construct `C1` refuses. It must be neither reddened
nor linted, only skipped and counted. A later author who restores whole-tree
reach turns `good/` red at exit 2, so the boundary is held by the oracle rather
than by prose that a refactor can quietly drop. A second file exercising the
predicate's **`runs:` arm** (a composite action whose `runs.steps[].run` block
lints clean) keeps that arm real rather than decorative: the tree carries no
composite action today, so the fixture is the only place it is ever executed.

**D2. `bad/` carries a genuine finding and each refusal. {design-bearing}** A
block with a real `-S warning` ShellCheck finding, plus one file per `C1`
refusal (`>`, `|2`, an alias, an unbalanced `${{`) asserting **exit 2** and the
construct name in the message — so the boundary is proved by the oracle. The
expectation files match on the gate's text, not on an exit code alone.

### E. The regen tail

**E1. {mechanical}** A new gate in a kit moves a fixed set of generated
projections, each naming its own regen command on a red: the pre-commit hook
(`gen-pre-commit.sh --write`), the graph artifact
(`check-graph.sh --emit > docs/check-graph.html`), the enforcement map
(`enforcement-map.sh --emit > docs/enforcement.md`), the on-site SPEC mirror
(`gen-docs-mirror.sh --write`, which carries `B8`'s SPEC section), the footprint,
and the value rollup that joins them. Executing this is running commands until
the battery is green.

## Producers and consumers

No new state, event, or message is introduced — this unit adds one gate script,
one registry row, one README roster row, and one fixture pair.

- **The extracted shell fragment (`B3`).** *Producer*: the awk pass, running
  inside `check-action-run-shell.sh` on every battery run — not a code path
  behind an unset config, since `B2`'s scan set is derived and the gate is
  registered unconditionally. *Consumer*: `shellcheck`, invoked once per
  fragment with the dialect `B5` resolved. The fragment is scratch: it lives in
  a `mktemp -d` the gate removes on exit, so no tracked or persistent state is
  produced and there is no artifact for a second reader to drift from.

- **The dialect resolution (`B5`).** *Producer*: the `shell:` sibling key in the
  step mapping, read by the same awk pass that finds the block. *Consumer*: the
  `-s <dialect>` argument on that fragment's ShellCheck invocation, at the
  transition from extraction to lint. The absent case has a named producer too —
  GitHub's documented runner default — which is why absence resolves to bash
  rather than to a skip.

- **The refusal verdict (`C1`).** *Producer*: the extractor, on detecting an
  unhandled construct. *Consumer*: the gate's exit status, which
  `gate-sdk/bin/run-gates.sh` and the generated pre-commit hook both read; exit 2
  is the fail-closed code the gate model already defines, so no new signalling is
  introduced. The construct name is read by the human fixing the workflow — the
  reader that makes a refusal actionable instead of merely blocking.

- **The skipped-and-counted tally (`B5`, `C2`).** *Producer*: the gate's own
  counters. *Consumer*: the gate's clean-run output line, which is the only
  place the cost of `C2` is visible on a green run. A count with no reader would
  be removed; this one is the mechanism that keeps the stated limit in front of
  the operator instead of buried in this amendment.

- **`check-action-run-shell`'s own wiring (`B8`).** *Producer*: the row in
  `scripts/gates.list`, which is what makes `run-gates.sh` resolve and run it.
  *Consumers*: the full battery, the generated pre-commit hook (through the
  `# graph:` manifest and `precommit` tier), and the CI backstop. The `# graph:`
  manifest carries a further reader: `check-graph`'s artifact and the enforcement
  map both project from it, which is why `E1` is a delta and not housekeeping.

- **The gate's own shipped script (`A1`).** *Producer*: the new file under
  `gate-sdk/checks/`. *Consumer*: `check-shellcheck`, which lints every
  `gate-sdk/checks/*.sh` — so the gate that lints workflow shell is itself
  covered by the linter it extends, closing the loop rather than exempting
  itself.

## Existing sections updated

- **gate-sdk/SPEC.md** — a `### check-action-run-shell` section under the
  per-component contracts, carrying `B1`'s invariant, `B2`'s derived scan set
  **and its Actions-shape predicate**, `B5`'s dialect table, `B6`'s severity and
  the SC1091 extraction artifact, and §C's limit in all three classes — refused,
  out of reach, and out of subject. The boundary belongs in the canonical spec,
  not only here: this amendment is deleted at merge and it must outlive it, and
  the out-of-subject class in particular is the one a later reader is most
  likely to mistake for an oversight and "fix". It carries `B2`'s **derivation
  and not `B2`'s count** — the block tally is a measurement this iteration's own
  sibling unit changes.
- **gate-sdk/SPEC.md §check-shellcheck** — owned by `B8`: one sentence naming
  its sibling, so the reader who asks "does anything lint my workflows?" is
  answered at the section whose target list makes the answer *no*. Today that
  section states the target derivation and stops.
- **gate-sdk/README.md** — the roster row for the new gate, per `B8`.
- **`.github/workflows/publish.yml`** — no edit is owed by *this* unit, but its
  two blocks come under lint; the `release-tarball-delivery-channel` unit adds a
  third block in the same iteration and it must land clean under this gate.
  **The job split and the `uses:` pins stay untouched** per that file's header.
- **docs/gate-sdk/SPEC.md, docs/enforcement.md, docs/check-graph.html,
  scripts/git-hooks/pre-commit, docs/footprint.md, docs/value.md** — generated
  projections, regenerated through their owning commands per `E1`.
- **This repo's release note for the shipping version** — a Tightened-gates
  bullet, per docs/install.md §The upgrade contract; authored at close.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain
      (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **The fidelity limit is pinned by the oracle, not only by prose** — every
      §C1 refusal has a `bad/` fixture asserting exit 2 and the construct name,
      and every §C2 out-of-reach class is named in gate-sdk/SPEC.md.
- [ ] **The reach ruling is pinned by the oracle** — `good/` carries a
      non-Actions YAML with a `run: >` block, and restoring whole-tree reach reds
      it at exit 2. Verified by making that reversion before shipping. The
      predicate's `runs:` arm has a `good/` fixture that fails if the arm is
      dropped.
- [ ] **The predicate loses no block** — the extractor run over the files §B2's
      predicate skips yields nothing, re-derived against the tree rather than
      inherited from this amendment.
- [ ] **The extractor's own trap is pinned** — `good/` carries the `- run: |`
      dash-inline block with sibling keys, and reverting the key-column rule to
      the dash's column reds it. Verified by making that reversion before shipping.
- [ ] **`PROBE_SET` is unchanged** — no YAML parser, no language runtime; the
      floor roster in `context-kit/lib/toolfloor.sh` is byte-identical.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
