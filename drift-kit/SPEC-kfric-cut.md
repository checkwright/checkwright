# SPEC amendment: kfric-cut

The port disposition of **`drift-kit/bin/kfric.sh` (60 lines), the one owed file
declaring §The knowledge-friction loop**, onto the binary substrate as a non-gate
arm. This is a stated-contract cut under the port-only run (TRAJECTORY.md
§PRIORITY DIRECTIVE), composed at scope and ruled by the **lead on its own
authority**, 2026-09-03, over the resume channel; it did not reach the operator,
and that is stated because a composition ruling recorded without its authority
reads at the post-port triage as more settled than it is.

**Measured at this HEAD rather than carried from scope's survey**: the port
oracle's `--tree` arm reads 104 files scanned, 64 declared `no-port`, 0
temporarily held, **40 owed**. This cut takes one of that column and **no ported
member is a gate** — the tool is advisory capture tooling that has never joined
`gates.list`, and §The knowledge-friction loop has no gate at all: probed rather
than assumed, `check-close-surfaces` asserts only that the log is *declared* on
the close-surface roster with the right mode and reclaim command, never its
content.

**This cut is the sibling of `lifecycle-kit/SPEC-gap-cut.md`**, landing in the
same iteration; three deltas below turn on that and say so where they do.

## What changes

### (1) This cut discharges the section's owed set, which is the material difference from its sibling

The composer ruling of 2026-08-28 selects a cut **by stated contract** — the owed
files behind one specification section, ported behind the one amendment that
section needs {design-bearing}. `bin/kfric.sh:2` declares
`drift-kit/SPEC.md §The knowledge-friction loop` in its own header ("the capture
affordance; stamps the grammar, no caller-side redirect"), and it is the only
**owed** file that does.

**Unlike the sibling cut, this one leaves the section's owed set empty**, and the
amendment states that rather than leaving a reader to infer it. Four surfaces
declare the section in a `# spec:` / `// spec:` header and their dispositions are
already fixed:

- `native/src/emit/kpi/knowledge_friction.rs:1` — the **reader**,
  `kpi-knowledge-friction`, already compiled. It is untouched by this cut: it
  reads the log's lines, not its writer.
- `drift-kit/smoke/install.sh:86` and `:102` — the reader's behavioural coverage,
  in a file **header-declared `no-port`**. Delta (5) narrows a different block of
  the same file.
- `drift-kit/bin/kfric.sh:2` — this cut.

So after this cut the section's contract is wholly in-crate apart from a
no-port-declared smoke, and no later cut is sequenced against it. That is a real
discharge and is stated as one, because the 2026-09-01 survey cut established the
opposite shape as the norm and a reader who learned the norm there would expect a
residue paragraph here.

### (2) `--emit-kfric` — the capture affordance, two positionals and one declared knob

`bin/kfric.sh` becomes an `Arm::Emit` member of `BRIDGED_ARMS` spelled
`--emit-kfric`, reachable through the shipped front-end as
`run-gates.sh --emit kfric [--] "<fact>" "<surface>"` with no front-end change
{design-bearing}. Its declared roster is **one** name,
`DRIFT_KIT_KNOWLEDGE_LOG`, already defaulted at `drift-kit/lib/drift.sh:48` and
therefore declarable without the bridge's does-not-define refusal. The family is
**forced, not chosen** by §The non-gate arm's test: the tool resolves a consumer
knob, and a hardcoded top-level flag would resolve the platform default while
silently ignoring every override.

Five behaviours are the contract and each survives explicitly:

- **Two positionals, each required non-empty**, in the fixed order
  fact / surface. Arity misuse is exit 2 with usage on stderr, exactly as today.
  The **order is load-bearing and the second field's direction is contested** —
  `kfric-second-field-direction-inverted` is live against it — so the port
  preserves the grammar verbatim and rules nothing about it (delta 7).
- **The free-text shape refusal survives the substrate, and it survives on both
  positionals.** A positional beginning with `-` that is not preceded by `--` is
  a refusal at exit 2, and `--` ends option processing. The section already
  states the ground — "two slots making arity safe in neither" — and the
  crate-side helper is `file_survey::positionals`
  (`native/src/emit/file_survey.rs:23`), already in tree with this exact refusal.
- **`-h` / `--help` does not port.** Usage for a bridged arm lives in
  `run-gates.sh`'s own help and in `drift-kit/README.md`. **This is the one
  observable that moves, and the crate has no branch for it today**: probed
  rather than assumed, `file_survey::positionals` has no help case, so `-h` falls
  to the generic unrecognized-option refusal, and `run-gates.sh`'s own `-h`
  branch fires only on its first argument — so `--emit kfric -h` is a **refusal**
  at exit 2 rather than usage at exit 0. Delta (5) re-homes the two smoke
  assertions that pin the old behaviour.
- **The appended line's grammar is byte-preserved**: `<date> <fact> ← <surface>`,
  the same line echoed to stdout as the arm's returned string, with the log's
  parent directory created if missing. `date` joins the arm's spawned-program
  set, the shape `--emit-file-survey` already carries.
- **The repo-root anchor moves with the tool.** The shell form `cd`s to the git
  toplevel before resolving the log path; the arm resolves it through
  `file_survey::anchored` (`native/src/emit/file_survey.rs:41`), falling back to
  the working directory outside a repository exactly as today.

### (3) A standalone config resolver is collapsed into the bridge, and the claim that motivates it is checked rather than repeated

`bin/kfric.sh:9-25` carries an **inline copy** of the `DRIFT_KIT_CONFIG_FILE`
resolution block that `drift-kit/lib/drift.sh:6-24` also holds — the tool does
not source the library, it re-implements its opening. The arm removes that second
copy outright: a bridged member's knobs are resolved by `gate_command`, which
sources `lib/drift.sh` itself, so the arm reads a resolved environment and
resolves nothing {mechanical}.

**The stronger reading of that duplication is refused here, having been probed.**
`drift-kit/lib/drift.sh:2` declares itself the config bridge's "**sole
resolver** for the `DRIFT_KIT_*` knobs", and it is tempting to read the inline
copy as already falsifying that sentence. It does not: the claim is scoped by its
own next clause to knobs resolved *through the bridge*, and a standalone `bin/`
tool's own resolution is outside that scope. So this delta removes a genuine
duplicate without repairing a defect, and the no-port declaration's ground is
unchanged in either direction — which is the honest statement and the one a later
cohort sweep of that class needs.

### (4) The class's argv-shape split gains its third instance, and the ordinal is merge-order-dependent

gate-sdk/SPEC.md §The non-gate arm rules the split for a capture member — the
shape refusal and `--` escape survive the port because the hazard belongs to the
argument, while `-h`/`--help` retires because usage belongs to the substrate —
and states it with `--emit-file-survey` as its single worked instance
{design-bearing}.

**The pair of amendments landing this iteration takes that from one worked
example to three instances**, and the count is written that way deliberately.
`lifecycle-kit/SPEC-gap-cut.md`'s delta (2) calls `--emit-file-gap` the class's
"second" such member, which is true of the state that amendment was written
against and **stale the moment this one merges first**. Build writes the
final-state sentence **once**, in whichever batch merges last, rather than an
ordinal each amendment asserted independently. Recorded as a delta because this
is exactly the shape `amendment-dod-sibling-dependence` describes and the
cheapest moment to catch it is before either merge.

### (5) The behavioural coverage narrows to the seam a crate test cannot see

`drift-kit/smoke/install.sh:590-616` is the **only** surface in the tree that
executes this tool: probed rather than assumed, no `gate-tests/*.test.sh` invokes
it, and the file's other kfric-adjacent blocks (`:78-158`) drive the already-ported
KPI **reader** and are untouched {design-bearing}.

That block asserts five things: `--help`/`-h` at exit 0 with usage on stdout; a
flag in **either** positional slot refused at exit 2; no stdout on the refusal
path; the log byte-unchanged after a help or refusal path; and the `--` escape
filing a fact beginning with a dash. It narrows to the shape
`queue-kit/gate-tests/queue-index.test.sh` took at its own port — it keeps the
**discriminating** cases, the either-slot flag refused at exit 2 with the log
byte-unchanged and the `--` escape, all driven through `run-gates.sh --emit`, and
hands the grammar cases to the ported module's own `#[cfg(test)]` tests where
`check-crate-arms` runs them. The two `-h`/`--help` cases **retire** with delta
(2)'s help arm rather than being rewritten, because the behaviour they pin is the
one that intentionally moved.

### (6) Every path-bearing surface moves in the deleting commit

The count is probed rather than assumed {mechanical}:

- `.claude/settings.json` — **one** grant names the deleted path,
  `Bash(bash drift-kit/bin/kfric.sh *)`. Removing a grant whose target a ruled
  port cut deletes is outside the 2026-08-22 bar under the operator's 2026-08-29
  settings-grant carve-out, and the removal lands in the same commit as the
  delete. **No replacement grant is needed**, probed rather than assumed:
  `Bash(bash gate-sdk/bin/run-gates.sh *)` is already granted. The unrelated
  `Bash(: > .workflow/knowledge-friction.log)` grant is the log's reclaim command
  and is **not** touched — named because it looks adjacent and is not.
  `check-settings-paths` is the oracle, and its red condition is a literal `.sh`
  grant that does not resolve, so the deletion without its grant line reds it.
- `CLAUDE.md`'s knowledge-friction-capture bullet, which carries the invocation
  form verbatim.
- `drift-kit/README.md`'s affordance line.
- `drift-kit/SPEC.md` §The knowledge-friction loop, step 1's affordance sentence.
- `lifecycle-kit/SPEC.md` §The committed gap inbox, whose affordance paragraph
  names "the `bin/kfric.sh` pattern" as `bin/file-gap.sh`'s precedent — **a
  sentence the sibling amendment also edits**, so the two cuts collide on one
  paragraph and the second batch to touch it must read what the first wrote.
- `lifecycle-kit/bin/file-gap.sh:2` cites "the kfric.sh pattern" in its own
  header. It is **not** on this cut's worklist: the sibling cut deletes that file
  outright, so the citation dies with its carrier. Named so a build session does
  not edit a file another batch is removing.
- `drift-kit/smoke/install.sh` (delta 5).
- The dated release post under `docs/posts/` is **out of corpus** and is not
  edited: version-pinned historical prose, the standing disposition for that
  surface.

### (7) Two live entries name this tool as their subject; their premises narrow and their verdicts do not

Each is corrected in place to name the arm, and **no verdict moves** {mechanical}:

- `kfric-second-field-direction-inverted` cites `drift-kit/bin/kfric.sh` **line 3,
  its `usage()`, and line 59** by exact location as the grammar's authority. All
  three are deleted; the grammar's holders become the arm's usage string and the
  ported emitter, and the entry's three candidate dispositions — validate at the
  affordance, rename or split the field, or fix the doctrine line — are as open
  against a compiled arm as against the script. This is the entry most damaged by
  a silent delete, because its argument rests on a line-numbered citation.
- `gap-capture-argv-prompt-friction` cites the `kfric.sh *` grant as evidence
  that the friction is not missing coverage, and already anticipates this cut by
  name: "the capture tool's port to a compiled arm moves where that shape is
  written and settles nothing here". That sentence stays true and is **not**
  edited into a claim of progress; only the grant citation moves.

Taking either inside a port cut is non-port design work the composer refuses.

### (8) The regeneration fan-out this cut stales

Deleting one owed `.sh` file moves `measured-claims.sh`'s `tree-shell-owed` key,
read off `--emit port-blockers --tree`'s trailer {mechanical}.
docs/site-architecture.md §Generated projections rules that a tree edit moving a
measured claim stales the generated `pre-commit` and `commit-msg` hooks — the
baked invocation carries `check-measured-claim`'s resolved values — and
`docs/check-graph.html` with them. The edits to `drift-kit/SPEC.md`,
`drift-kit/README.md`, `gate-sdk/SPEC.md` and `lifecycle-kit/SPEC.md`
additionally stale their on-site mirrors. All are rostered with their regen
commands in that section and are discharged in the landing commit; `check-graph`,
`check-docs-mirror-fresh` and `check-gate-binary-fresh` are the reds.

### (9) The criterion-5 residual is an affordance, not the loop

Criterion 5 asks what a consumer whose payload carries no artifact for its host
still has after the cut {design-bearing}. This cut has **no gate in it**, so the
binary-less leg's roster and its non-zero count do not move.

What such a consumer loses is one affordance, and **not** the loop. §The
knowledge-friction loop already rules the raw append "the fallback — the grammar,
not the writer, is the log's contract; both consumers below read lines, not
provenance", and both readers survive untouched: the close triage in
`drift-kit/templates/close-knowledge.md` and the compiled
`kpi-knowledge-friction`. So an artifact-less consumer still captures by hand,
still has the log triaged at close, still has it counted by the KPI and still has
its reclaim command on the close-surface roster.

**The residual has a sharper edge here than on the sibling cut, and it is stated
rather than smoothed.** The section's stated reason the affordance exists at all
is that the raw form is a shell redirect "that no allowlist glob suppresses
safely", so a permission prompt "never turns capture into deferred capture". For
a consumer with no artifact the affordance is gone and the fallback is exactly
the redirect that argument rules out — so the loss is the prompt-free property,
not merely a convenience. That is accepted, and it is the same shape every
binary-less-leg residual takes; it is written here because this section is the
one that argued the property was load-bearing.

## Producers and consumers

The amendment introduces **one interface** — a single bridged flag — and **no new
state, no new event, no new field, and no new knob**. The one knob named is
already shipped, already defaulted in `drift-kit/lib/drift.sh` (which stays
permanently shell as the bridge's sole `DRIFT_KIT_*` resolver, so no default
moves in this cut) and already read by the script being replaced.

- **Producer** — `native/src/emit/mod.rs`'s `BRIDGED_ARMS` table, one row
  (`"--emit-kfric"`, `Arm::Emit(kfric::emit)`, `kfric::KNOBS`), with no
  `run-gates.sh` change: the front-end composes `--emit-<name>` from its
  `--emit <name>` operand and passes the remaining argv, a leading `--` included,
  through untouched. The enabling config is the table row itself — `--knobs`
  publishes the roster and `gate_command` resolves it before the exec — so
  nothing must be configured per install.
- **Consumer** — a **session that caught itself re-deriving a fact no doc owns**,
  invoking it through the front-end at the moment of re-derivation; the channel
  is live everywhere the kit is vendored because the knob carries a shipped
  default, so the deployed configuration that must be set is none. Its stdout
  confirmation is read by that session at that transition.
- **Consumers of the appended line**, both already named in the section and
  neither new: **close's triage**, through
  `drift-kit/templates/close-knowledge.md` spliced into the consumer's close
  skill, at the close stage; and **`kpi-knowledge-friction`**
  (`native/src/emit/kpi/knowledge_friction.rs`), which counts non-blank lines at
  drift-report time and carries the three-state grammar. Both read lines and not
  provenance, which is why neither moves when the writer's substrate does.
- **Consumer of the declared roster** — `gate_command`, which resolves
  `DRIFT_KIT_KNOWLEDGE_LOG` by sourcing `drift-kit/lib/drift.sh` and refuses the
  whole environment for a knob it does not define. It is defined there today at
  line 48, which is why this cut moves no default — unlike the 2026-09-01 index
  cut, whose driver held its defaults inline.

**The arm has a caller that is not a test** — the CLAUDE.md bullet mandating
in-the-moment capture, invoked by any session — which is the third property
§The non-gate arm requires. The narrowed smoke of delta (5) is a *second* caller
rather than the qualifying one, so this cut leaves
`gate-test-in-tree-invoker-ruling` standing.

**One corpus is narrowed, and its readers' red conditions are enumerated rather
than their subjects** (canon-kit/SPEC.md §The causal-completeness check, point 5).
The narrowing is the deletion of one file from the tracked `*.sh` tree:

- `check-settings-paths` — reds on a literal repo-relative `.sh` grant that
  **does not resolve**, so its verdict is *not* monotone under this narrowing:
  removing the file **adds** a violation. Cleared by delta (6), in the same
  commit, not by inspection.
- `check-measured-claim` — `tree-shell-owed` moves by one, and by **two** across
  the iteration with the sibling cut. Whether a governed sentence pins it is
  checked by scanning the `measured:` markers rather than assumed; the derived
  consumers are the baked hook invocations, cleared by delta (8).
- `check-graph` / `check-docs-mirror-fresh` / `check-gate-binary-fresh` — red on a
  stale hook, artifact, mirror or binary, and non-monotone for the same
  baked-value reason. Cleared by delta (8).
- `check-close-surfaces` — named because a reader assumes a cut on the loop moves
  it, and it does not: its subject is the roster **declaration** in
  `drift-kit/SPEC.md`, which this cut does not touch, and neither the log's path
  nor its reclaim command moves.
- `check-gate-fixture-coverage` — likewise unmoved: no `.gate` descriptor and no
  fixture pair is owed or removed here, the member never having been a gate.
- `check-exec-bit`, `check-shellcheck`, `check-comment-tier`, `check-path-dialect`
  — monotone in the scanned `.sh` set; removing one file can only remove
  findings, cleared by inspection.

**Cross-component signal: this amendment's component set is two** — drift-kit and
gate-sdk (§The non-gate arm's class roster) — and it additionally edits a
paragraph in `lifecycle-kit/SPEC.md` that its sibling amendment also edits, so
`check-stage-entry` assertion C fires on both counts and the **align stamp is
demanded at the build stage's entry**. Stated here so the build session is not
the one that learns it.

## Existing sections updated

- `drift-kit/SPEC.md §The knowledge-friction loop`, step 1 — the affordance is
  restated as its arm: the invocation form, the two required positionals in their
  fixed order, the surviving argv-shape refusal on both, the retired help arm, the
  preserved line grammar, and the one declared knob. **The discharge is stated**:
  the section names its four declaring surfaces, that this cut takes the only
  owed one, and that its contract is thereafter wholly in-crate apart from a
  no-port-declared smoke (deltas 1 and 2).
- `drift-kit/SPEC.md §The knowledge-friction loop`, step 1's prompt-free
  paragraph — the ground survives the port and is now stated of the arm: the
  invocation is a static prefix under an existing grant, and the raw redirect
  stays the fallback the section already rules legal (deltas 2 and 9).
- `drift-kit/SPEC.md §lib/drift.sh` — a sentence that the arm resolves its knob
  through the bridge rather than through a second inline copy, with the honest
  note that the removed copy never falsified the sole-resolver claim, whose scope
  is bridge-resolved knobs (delta 3).
- `gate-sdk/SPEC.md §The non-gate arm` — the class roster gains `--emit-kfric`;
  the argv-shape/help split gains its instance count, written **once** for the
  final state of both cuts rather than as an ordinal each amendment asserted
  (delta 4).
- `lifecycle-kit/SPEC.md §The committed gap inbox` — its affordance paragraph
  names the arm rather than `bin/kfric.sh` as `bin/file-gap.sh`'s precedent; the
  sibling amendment edits the same paragraph, and the second batch reads what the
  first wrote (delta 6).
- `CLAUDE.md`'s knowledge-friction-capture bullet and `drift-kit/README.md`'s
  affordance line (delta 6).
- `TASK-QUEUE.md`, the `drift-kit-bin-port-residue` entry — minted and promoted
  in the same motion as this amendment, carrying its `[spec:]` ref; it **demotes**
  to the deferred section at build and never reaches `## Done`, its deliverable
  being a corpus (all deltas). No prior deferred position exists to return it to,
  so the entry states where it demotes rather than leaving build to choose.
- `TASK-QUEUE.md`, the `kfric-second-field-direction-inverted` and
  `gap-capture-argv-prompt-friction` entries — premises corrected to name the arm,
  verdicts unchanged (delta 7).
- The generated projections this cut stales — the on-site SPEC and README
  mirrors, the generated `pre-commit`/`commit-msg` hooks, `docs/check-graph.html`,
  and the gate binary itself. All are rostered with their triggers and regen
  commands in `docs/site-architecture.md` §Generated projections (all deltas).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls drift-kit/SPEC-*.md`), the none-remain half discharged at the
      iteration rather than at the commit.
- [ ] **Removals propagated** — grepped every spec, skill, template, README,
      smoke script, settings file and compiled remedy string for the deleted path;
      nothing dangles. `kfric-second-field-direction-inverted`'s line-numbered
      citation is checked by name, being the one that dangles silently.
- [ ] **The discharge is written, not implied** — §The knowledge-friction loop
      names its four declaring surfaces and says the owed set is empty after this
      cut; a reader who never saw this amendment can tell no later cut is
      sequenced against it.
- [ ] **The sibling's ordinal is reconciled once** — gate-sdk/SPEC.md §The
      non-gate arm's argv-shape split states the instance count for the final
      state of both cuts, written by whichever batch merges last, not twice.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **The oracle moved, and by the roster rather than by a number** — the
      `--tree` arm lists no `drift-kit/bin/kfric.sh` row, taken as a per-file
      roster diff and not as a trailer delta.
- [ ] **The regeneration fan-out is discharged in the landing commit** — the
      generated hooks, the graph artifact, the SPEC and README mirrors, and the
      gate binary.
