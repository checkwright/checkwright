# SPEC amendment: action-gh-repo-context

A workflow job that invokes `gh` while carrying neither a checkout nor a
repository-context environment cannot resolve a target repository, and nothing
catches it until a tag fires. This amendment lands the assertion that catches it.

The class is attested rather than hypothetical: it took down `v0.17.0`'s
`release` job on its first live run, which died in seconds on
`failed to run git: fatal: not a git repository` before its first API call, with
no Release created and no assets attached. The concrete instance is fixed; what
stays open is the class, and the failure mode is the worst-timed one available —
green everywhere, red only at the tag, on the release path itself.

**Why no existing gate reaches it.** §check-action-run-shell lints exactly this
block and passes it: the shell is valid, the variables are quoted, the control
flow is sound. The defect is **semantic** — an assumption about the runner's
filesystem that no syntactic linter can hold. The gate that exists is not the
gate this needed, and that distinction is the unit's whole point.

## What changes

### Delta 1 — a new gate, `check-action-gh-repo`, in gate-sdk {design-bearing}

Invariant: in every Actions-shaped YAML file, a **job** whose `run:` bodies
invoke `gh` establishes a repository context — the job contains a checkout step
ordered before its first such invocation, or `GH_REPO` is set at workflow, job,
or invoking-step level, or **every** detected `gh` invocation in the job carries
`--repo`.

**Placement — gate-sdk, not this repo's `scripts/`.** The deciding fact is that a
*kit* ships a workflow template carrying `gh` calls in its `run:` bodies:
site-kit's `templates/site-health.yml`, a copy-out consumers vendor. A consumer
gate placed here would cover this tree and leave every downstream vendor of that
template unchecked — the same fix-the-instance shape §check-action-pinning
rejected — and that the template belongs to a *different kit* is what makes the
argument decisive, since no consumer gate and no site-kit-local gate covers both.
Reach is bought the way the two sibling `action-` gates buy it, and a consumer
running no Actions pays a zero count, the counted inertness that makes this kit
mechanism rather than a consumer gate.

**Naming.** `check-action-gh-repo`, joining the `action-` family. Not
`check-workflow-*`, for §check-action-pinning's reason: this tree already spends
"workflow" on §The workflow directory and `GATE_SDK_WORKFLOW_DIR`, so a second
`check-workflow-*` gate would collide on the reader's only disambiguator.

### Delta 2 — the detector, and why the `--repo` arm is not the fiddly one {design-bearing}

The queue entry flagged "`--repo` on every call" as the arm to design carefully,
because a job mixing prefixed and unprefixed calls is a false negative waiting to
happen. The design that removes the worry: **the trigger and the arm share one
detector.** The gate must already answer "does this line invoke `gh`" to arm at
all, so the arm is evaluated per detected invocation and the predicate is
*universally quantified over the detected set* rather than satisfied by a
witness. One unprefixed call fails the arm by construction; there is no search
for a positive example to be fooled by, and the arm costs almost nothing beyond
the trigger that was already owed.

**The scan is this gate's own, and §check-action-run-shell's extractor is not
reusable here.** That extractor has no theory of `jobs:` at all — it tracks step
column and block scalars per *file*, emits records carrying a block number and a
start line but no job identity, extracts neither `uses:` nor `env:` lines, and
rewrites `${{ … }}` before emitting. This gate needs a job-partitioned walk
carrying all three of those. So it is **not** the second consumer that
§check-action-run-shell's standing rule waits for: that section's "a helper earns
its place at a second consumer and there is none", and the matching `# spec:`
comment at the extractor itself, stay true and are not edited by this unit.
Stated because the opposite reading is the natural one, and because
`release-note-lead-token-grammar` invoked the *same* standing rule earlier in the
same iteration and did satisfy it, landing gate-sdk/SPEC.md §lib/declaration.sh —
the two units meet that rule with opposite
answers, and only the difference in what is being extracted decides it.

The detector, over each `run:` body in the job:

- **Logical lines.** Backslash continuations are joined before matching, so a
  call split across lines — the shape `site-health.yml` already uses — is one
  unit and its `--repo` is found wherever on the call it sits.
- **Command position.** A `gh` token counts when it is a whole word at the start
  of a logical line or immediately after `|`, `||`, `&&`, `;`, `&`, `(`, a
  backtick, or `$(`. Whole-word matching keeps `ghost` and `gh-pages` out.
- **Comment lines.** A body line whose first non-blank character is `#` is not a
  command line — a leading `#` in shell is always a comment. A trailing `#`
  comment is left in place, which over-detects and is the safe direction.
- **The bias, stated rather than left implicit.** Every ambiguity resolves toward
  over-detection. A false positive costs a workflow author one `GH_REPO:` line
  or one exemption marker; a false negative is the release-path failure this gate
  exists for.

### Delta 3 — the checkout arm and its honest limit {design-bearing}

A step satisfies the checkout arm when its `uses:` ref before the `@` is
`actions/checkout` **and** its line precedes the job's first detected `gh`
invocation. A checkout ordered after the call establishes nothing, and the
ordering comparison is free once both line numbers are in hand.

Honest limit: any other means of establishing a git remote — a hand-rolled
`git clone`, a different checkout action — is outside the gate's theory and
takes the valve. That limit is stated rather than papered over with a looser
match, because a looser match is what turns a semantic assertion back into a
syntactic one.

### Delta 4 — the environment arm {design-bearing}

`GH_REPO` satisfies the arm when set at workflow-root `env:`, at
`jobs.<id>.env:`, or on the `env:` of the step whose body carries the invocation.
`gh` resolves its target repository from `GH_REPO` and from the git remote, so no
other variable joins the arm; a consumer relying on a different mechanism takes
the valve.

The arm is deliberately the one the fixed instance used, and it is the arm that
survives maintenance: a job-level `GH_REPO` cannot be undone by someone adding a
new `gh` call, whereas per-call `--repo` can.

**The step-level lookup is load-bearing, not completeness.** The fixed instance
sets `GH_REPO` on the invoking step's `env:`, not at workflow or job root — so
the one in-tree job this gate exists for passes on the *narrowest* of the three
sub-arms. A gate implementing only the workflow- and job-level lookups would red
`publish.yml` on day one and read as a false positive.

### Delta 5 — the valve {design-bearing}

`# gh-repo-exempt: <reason>` on the job or on the step, taking the kit's
established `# <thing>-exempt:` marker shape (`# fail-closed-exempt:`,
`# hermetic-exempt:`, `# assertion-strength-exempt:`).

The reason is required and non-empty — a bare marker is a red, since the valve's
whole value is the sentence saying which arm the author is standing outside of.
**That requirement is a deliberate tightening, not conformance**, and saying so
is the honest form: no sibling enforces it. `check-test-hermetic` and
`check-assertion-strength` match their marker with the colon and read no reason;
`check-gate-fail-closed` matches a bare `fail-closed-exempt` without even
requiring the colon. Nor is there a shared marker parser in `lib/gate.sh` — each
gate rolls its own, and this gate rolls a fourth. The tightening is worth its
inconsistency here because this valve stands a job outside a *release-path*
assertion, where an unexplained exemption is the failure mode the gate exists
for; whether the siblings should follow is not this unit's question.

### Delta 6 — scope, and the composite-action limit {design-bearing}

The gate's unit is a **job under `jobs:`**. A composite-action file
(`runs:`-shaped, carrying no `jobs:`) is skipped and counted: it has no job of
its own and inherits the calling job's repository context, so the assertion
belongs to the caller rather than to the action.

The scan set is derived exactly as §check-action-pinning's is — a `gate_find`
walk for `*.yml` and `*.yaml` from the scan root, with the shared prune set
keeping it out of `gate-tests/` so the `bad/` fixture cannot red the whole-tree
run. No roster, and **no new knob**: the scan set is derived and the prune set is
the shared one.

The Actions-shape predicate is §check-action-run-shell's, **split rather than
borrowed whole**. That gate asks one question, `^(jobs|runs):`, because it lints
either shape's `run:` bodies alike. This gate needs the two arms apart: a
`jobs:`-shaped file is the subject, a `runs:`-only file is the composite action
skipped and counted above, and a file matching neither is outside the scan.
Two lineages, named separately because they are separate — §check-action-pinning
contributes the scan set and carries no Actions-shape predicate of its own
(it walks every `*.yml`/`*.yaml` in the tree), while §check-action-run-shell
contributes the predicate and carries no bearing on the scan set.

### Delta 7 — the landing checklist {mechanical}

A `good/`+`bad/` fixture pair from `templates/check-skeleton.sh`; a `# graph:`
manifest coupling the same YAML surfaces §check-action-pinning couples, `dir=one`
(a one-way audit), `valve=none`, `tier=precommit`; registration in
`scripts/gates.list` in the `action-` neighbourhood (the list is curated order,
not alphabetical); a
§check-action-gh-repo section in gate-sdk/SPEC.md placed after
§check-action-run-shell; and the generated projections regenerated. The
projection fan-out is owned by docs/site-architecture.md §Generated projections
and their freshness gates together with CLAUDE.md's hook and graph regeneration
rules — each freshness gate names its own regen command on red, so the set is
recovered by running the battery rather than transcribed here.

**The gate lands green on this tree, and the align audit is where that was
established rather than discovered at build.** Exactly three of the tree's YAML
files invoke `gh` in a `run:` body — `.github/workflows/publish.yml` (step-level
`GH_REPO`), `.github/workflows/site-health.yml` and
`site-kit/templates/site-health.yml` (both checkout-before-first-call). All three
pass, on two different arms, so the gate ships with no repair worklist and the
two live arms each have a real in-tree witness beside their fixture one.

This iteration's release note carries a Tightened-gates bullet for the new gate,
spelled per docs/install.md §The upgrade contract's canonical spelling (a
backticked, unbolded bare gate name), now held by
`check-tightened-gates-grammar`. Note that a new gate does
not red the upgrade smoke's phase B — §upgrade-smoke states that an N+1 gate is
absent from the scratch consumer's `gates.list` because phase A never re-runs the
installer — so the bullet is owed to the human upgrader, not to the smoke.

### Delta 8 — the `bad/` fixture is the attested miss {mechanical}

The `bad/` fixture reproduces the `v0.17.0` release job as it shipped: a job with
`contents: write`, a download-artifact step, deliberately no checkout, `GH_TOKEN`
and `TAG` set, `GH_REPO` absent, and `gh release view` / `create` / `upload` in
the body. §When a gate earns its place says a higher-false-positive gate waits
for a real miss to attest it and that the miss *is* the `bad/` fixture; this one
is on record with its failure mode, so the fixture is the attestation rather than
an invention.

The `good/` fixture carries the same job with `GH_REPO` restored, plus a job
satisfying the checkout arm and one satisfying the `--repo` arm, so all three
arms have a passing witness and the mixed-call case has a failing one.

### Delta 9 — `publish.yml`'s binding comment is re-pointed {mechanical}

The release job's `GH_REPO` comment currently carries the coupling as standing
prose — an instruction to keep the variable whenever the no-checkout design
holds. With the gate live the coupling is machine-held, so the comment cites
§check-action-gh-repo instead of arguing for itself. This is the comment-tier
move: the pointer names where the why lives and couples the code to it, rather
than keeping the why in the comment.

## Producers and consumers

- **The gate's verdict** — produced by `check-action-gh-repo.sh` under the
  battery, fired at the pre-commit tier by the generated hook's trigger block and
  at the CI tier by `run-gates.sh`. Consumed by the committing author and by CI's
  required check. Its enabling registration is real rather than test-only: the
  `scripts/gates.list` line (delta 7) is what puts it in the battery, and
  `check-gate-fixture-coverage` reds without the fixture pair.
- **The `# graph:` manifest** — produced in the gate's header; consumed by
  `gen-pre-commit.sh` (which turns it into the hook's trigger block) and by
  `check-graph.sh` (which turns it into the graph artifact). Both are the
  generated readers every registered gate has, and both are freshness-gated, so
  the manifest has no unread field.
- **`GH_REPO`** — not a new interface. It is `gh`'s own documented input,
  produced by the workflow author and consumed by `gh` inside the runner. This
  gate reads it as *text in the workflow file*, never by executing `gh`, which is
  what keeps the assertion hermetic.
- **The `# gh-repo-exempt:` marker** — produced by a workflow author on a job or
  a step; consumed only by this gate, at its per-job verdict, and its reason
  field is read by the human reviewing the diff. This is the one genuinely new
  name on a governed surface, and it is what makes the unit feature-shaped rather
  than debt.

**Seam.** Pure mechanism, with nothing owed to consumer config. The gate reads
GitHub's own workflow schema and `gh`'s own documented variable; it carries no
consumer vocabulary, no rule content, and no new knob, and its scan set is
derived with the shared prune set. The one consumer-supplied input is the
exemption reason, which is authored in the consumer's own workflow file rather
than configured into the kit.

## Existing sections updated

- **gate-sdk/SPEC.md** — a new §check-action-gh-repo after §check-action-run-shell
  (delta 1-6); §check-action-run-shell's framing of itself as the sibling
  reaching `run:` bodies gains the note that a second sibling now reads the same
  surface for a semantic property ShellCheck is structurally blind to, so a
  reader arriving at the shell linter does not conclude the `run:` surface has
  one gate.
- **gate-sdk/SPEC.md §check-action-run-shell's extractor paragraph, and the
  `# spec:` comment on the extractor itself — deliberately unchanged.** Both say
  a `lib/` helper earns its place at a second consumer and there is none; delta 2
  establishes that this gate is not that consumer, so both stay true. Named here
  because the natural reading is the opposite one, and because the build session
  that landed `release-note-lead-token-grammar` earlier in this iteration edited
  that rule's *other* instance. Editing them on this unit's authority is the
  error; if a build session finds a design in which this gate does consume that
  extractor, that is an escalation rather than an edit.
- **`.github/workflows/publish.yml`** — the release job's `GH_REPO` comment
  (delta 9).
- **`scripts/gates.list`** and the generated projections the new gate moves
  (delta 7).

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
