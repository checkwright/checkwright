# SPEC amendment: gap-cut

The port disposition of **`lifecycle-kit/bin/file-gap.sh` (100 lines), the one
owed file declaring §The committed gap inbox**, onto the binary substrate as a
non-gate arm. This is a stated-contract cut under the port-only run
(TRAJECTORY.md §PRIORITY DIRECTIVE), composed at scope and ruled by the **lead on
its own authority**, 2026-09-03, over the resume channel; it did not reach the
operator, and that is stated because a composition ruling recorded without its
authority reads at the post-port triage as more settled than it is.

**Measured at this HEAD rather than carried from scope's survey**: the port
oracle's `--tree` arm reads 104 files scanned, 64 declared `no-port`, 0
temporarily held, **40 owed**. This cut takes one of that column and **no ported
member is a gate** — the tool is advisory capture tooling that has never joined
`gates.list`; the assertion half of this section is already compiled as
`native/src/gates/gap_inbox_neutrality.rs`.

## What changes

### (1) The cut is one of this section's three implementations, and the residue is stated rather than implied

The composer ruling of 2026-08-28 selects a cut **by stated contract** — the owed
files behind one specification section, ported behind the one amendment that
section needs {design-bearing}. `bin/file-gap.sh:2` declares
`lifecycle-kit/SPEC.md §The committed gap inbox` in its own header ("the capture
affordance; stamps the bullet grammar, no caller-side redirect"), and it is the
**only owed file that does** — verified by grepping first-`# spec:` headers over
the owed set rather than by grepping the section title, which is the distinction
the 2026-09-01 survey cut recorded for the next selector.

**Taking this cut does not discharge the section.** Three shell files implement
it and this cut takes one:

- `bin/enter-stage.sh:435` implements the section's **iteration-boundary
  gap-inbox check** (one detector, two dispositions) and `:440` its
  close-skipped/post-close discriminator, the "one cursor read shared with
  `bin/file-gap.sh`". Its own header at line 2 declares §bin/enter-stage.sh, so
  the composer does not reach it here. The file is owed at 624 lines and ports in
  a **different** cut.
- `lib/stages.sh:131` implements `LIFECYCLE_KIT_UNION_SURFACES`' gap-inbox
  membership — the union-merge set §Multi-operator semantics owns. That file is
  **header-declared `no-port`** under the 2026-08-30 kit-library class ruling as
  the config bridge's sole `LIFECYCLE_KIT_*` resolver, so **this section's
  contract will never be wholly in-crate while that ruling stands**. Its already-
  ported twin `native/src/stages.rs:27` carries the same constant on the crate
  side and does not move in this cut.

### (2) `--emit-file-gap` — the capture affordance, one positional and five declared knobs

`bin/file-gap.sh` becomes an `Arm::Emit` member of `BRIDGED_ARMS` spelled
`--emit-file-gap`, reachable through the shipped front-end as
`run-gates.sh --emit file-gap [--] "<gap prose>"` with no front-end change
{design-bearing}. Its declared roster is five names, every one already defined in
`lifecycle-kit/lib/stages.sh` and therefore declarable without the bridge's
does-not-define refusal: `LIFECYCLE_KIT_GAP_INBOX_FILE` (`:54`),
`LIFECYCLE_KIT_QUEUE_FILE` (`:51`), `LIFECYCLE_KIT_STATE_FILE` (`:52`),
`LIFECYCLE_KIT_STAGES` (`:21`, an **array** knob the bridge resolves through
`knob_array`) and `LIFECYCLE_KIT_FIRST_STAGE` (`:27`). The family is **forced,
not chosen** by §The non-gate arm's test: the tool resolves consumer knobs, and a
hardcoded top-level flag would resolve platform defaults while silently ignoring
every override.

**No knob default moves in this cut**, which is worth saying because the
2026-09-01 index cut had to move two: that cut's driver held its defaults inline,
this one sources `lib/stages.sh` at line 10 and holds none.

Seven behaviours are the contract and each survives explicitly:

- **One positional, required non-empty.** Arity misuse is exit 2 with usage on
  stderr, exactly as today.
- **The free-text shape refusal survives the substrate, and it survives on the
  one positional as it does on `--emit-file-survey`'s five.** A positional
  beginning with `-` that is not preceded by `--` is a refusal at exit 2, and
  `--` ends option processing. gate-sdk/SPEC.md §The non-gate arm already rules
  this split for a capture member: the hazard belongs to the **argument**, not to
  the tool being a `bin/` script, so it does not retire with the file. The
  crate-side helper is `file_survey::positionals`, already in tree at
  `native/src/emit/file_survey.rs:23` with exactly this refusal text shape.
- **`-h` / `--help` does not port.** Usage for a bridged arm lives in
  `run-gates.sh`'s own help and in `lifecycle-kit/README.md`, the disposition
  `--emit-queue-counts` took at its own port and `--emit-file-survey` took at
  its. `--emit file-gap --help` is therefore a **refusal**, never a capture,
  which preserves the substantive half of §The bin/-tool contract while the
  discoverability half moves to where the class keeps it. This is the one
  observable that moves in this member, and delta (6) re-homes the two smoke
  assertions that pin it today.
- **The contract header is seeded when the inbox does not exist**,
  byte-identical to the string at `bin/file-gap.sh:46`, so a fresh consumer's
  first filing produces an inbox close's drain can truncate back to exactly that
  line. **The header this repo's live inbox carries is not that string** — probed
  rather than assumed: `.workflow/gap-inbox.md:1` additionally spells the bullet
  grammar and ends `(filed via lifecycle-kit/bin/file-gap.sh)`. That divergence
  predates the cut, it is a committed surface no tool rewrites, and delta (8)
  moves its dead path.
- **One bullet shape for every filing.** `- <YYYY-MM-DD> — <prose>` appended,
  the same line echoed to stdout as the arm's returned string. `date` joins the
  arm's spawned-program set, the shape `--emit-queue-index` and
  `--emit-file-survey` already carry.
- **The repo-root anchor moves with the tool.** The shell form `cd`s to the git
  toplevel before resolving the inbox path; the arm resolves it through
  `file_survey::anchored` (`native/src/emit/file_survey.rs:41`), falling back to
  the working directory outside a repository exactly as today.
- **Three stderr advisories are byte-preserved and stay on stderr**, the arm's
  returned string carrying stdout alone: the live-slug prompt (delta 3), the
  closing-stage WARNING, and the blocks-the-next-entry notice.

### (3) The live-slug matcher ports as lifecycle-kit's own predicate, and the collapse onto `queue::live_slugs` is refused here and recorded

`bin/file-gap.sh:48-77` resolves the filed prose against the live slug set with an
inline `awk` that is deliberately **lifecycle-kit's own** rather than queue-kit's
`queue_live_slugs`. The section states the ground at `SPEC.md:792-798`: reaching
for it "would close a cross-kit cycle", and queue-kit/SPEC.md §The queue format
carries that as the general rule — a kit that cannot depend on queue-kit
re-implements the predicate and both ends cite the owner section.

**The port reproduces the predicate rather than collapsing it, and the reason is
that collapsing it is a behaviour change no port may take on its own authority**
{design-bearing}. The crate already carries `native/src/queue.rs:230`
`pub fn live_slugs(text: &str, sec: &Sections) -> Vec<String>`, the compiled form
of queue-kit's own predicate, consumed by four modules today. Its section scope
is composed from `QUEUE_KIT_ACTIVE_SECTIONS` + `QUEUE_KIT_DEFERRED_SECTION` +
`QUEUE_KIT_ICEBOX_SECTION`; this section's predicate is **grammar-scoped
instead** — every column-0 entry bullet outside the fixed-spelling
`## Lessons Learned` section — and the section is explicit that it "needs no
section knob of its own because both exclusions fall out of grammar the kit
already reads". Those are not the same corpus: a consumer whose icebox is
unconfigured, or whose active-section roster differs from its heading set, gets a
different live set from each. So the collapse is a verdict change on a real
consumer and not a refactor.

Two further behaviours the reproduction must carry, both pinned by the existing
gate-test and neither present in `queue::live_slugs`: **longest match wins**, and
the match is **word-bounded** on `[a-z0-9-]` in both directions, so a
hyphen-embedded near-miss raises nothing.

**The residue is recorded because the stated ground weakens under this
substrate.** "Would close a cross-kit cycle" is a claim about a **shell** source
dependency — lifecycle-kit's `bin/` sourcing queue-kit's `lib/`. Inside one
binary both predicates are already compiled together and no vendoring decision
separates them, so the anti-cycle premise no longer describes the arrangement it
was written against. Whether the refusal survives on an independent ownership
ground, or retires with its premise, is **not settled by any surface in this
tree** and is not settled here: a cut that answered it would be widening its
asserted deliverable past the composer's own words. It is filed to the gap inbox
in this session rather than left for a later reader to re-derive.

### (4) The closing-stage predicate gains a second holder, across the substrate seam

`bin/file-gap.sh:93-94` calls `lifecycle_current_stage` and
`lifecycle_closing_stage_reached`, and `lib/stages.sh:109` states why the second
exists: "Hoisted because two tools must agree by construction rather than by
lookalike — the filer warned at capture that no stage is left to drain a bullet
is warned by the same test that later admits it at the boundary"
{design-bearing}.

The arm composes the same predicate in-crate from `stages::current_stage`
(`native/src/stages.rs:101`, already ported and already used by
`--emit-file-survey`) and the last element of the `LIFECYCLE_KIT_STAGES` array,
which is exactly `lib/stages.sh:111`'s test. **So the hoisting's guarantee
survives inside each substrate and is lost across them**: until
`bin/enter-stage.sh` ports, the boundary check and the capture warning agree by
lookalike rather than by construction. The amendment states this as a cost of the
cut rather than discovering it at build, and it dissolves — without further work
— on the cut that takes `enter-stage.sh`, which delta (1) already sequences.

### (5) One compiled gate prescribes a shell path in its remedy text

`native/src/gates/gap_inbox_neutrality.rs:158` prints, on the red path,
`File with 'bash lifecycle-kit/bin/file-gap.sh "<gap prose>"', which stamps the
one legal bullet shape`. That is user-facing remedy text naming a path this cut
deletes, and it moves in the same commit {mechanical}. Whether the fixture pair's
`expect.txt` pins the string is **probed at build, not assumed here**; the build
re-runs the pair rather than relying on the probe.

This literal is the reason a port of an advisory tool can strand a *gate*: the
gate stays green while telling its reader to run a command that no longer exists,
which no oracle catches.

### (6) The behavioural coverage narrows to the seam a crate test cannot see, and one suite is re-homed rather than narrowed

Two surfaces exercise this tool and they are not the same kind of thing
{design-bearing}.

`lifecycle-kit/gate-tests/file-gap-recurrence.test.sh` (122 lines) is the **only**
behavioural oracle for the matcher of delta (3). It drives the tool against a
sandboxed queue fixture and asserts nine scenarios: the exact bullet shape, that
no `recurrence of` verdict is ever interposed, that a live **deferred** slug
raises the advisory naming the **longest** match, that an **icebox** slug counts
as live, that a **done** slug and a **Lessons**-shaped slug and an indented
sub-task each raise nothing, that a hyphen-embedded near-miss raises nothing,
that a denying prose is still asked, and — the invariant with no other holder —
that **the queue file is never modified** by any filing. Every one of these is
re-homed onto the arm: the grammar cases into the ported module's own
`#[cfg(test)]` tests where `check-crate-arms` runs them, and the suite itself
re-pointed at `run-gates.sh --emit file-gap` so the no-queue-write invariant keeps
an end-to-end holder. **It is re-homed, not deleted**: it is a `.test.sh` outside
the port corpus by the suffix rule, and deleting the only oracle for a predicate
this cut is committing to reproduce would leave delta (3) unwitnessed.

`lifecycle-kit/smoke/install.sh:240-293` is the argv-contract smoke. It narrows to
the shape `queue-kit/gate-tests/queue-index.test.sh` took at its own port: it
keeps the **discriminating** cases — a leading-`-` refused at exit 2 with the
inbox byte-unchanged, and the `--` escape filing a dash-prefixed prose — driven
through `run-gates.sh --emit`, and hands the rest to the module. Its two
`-h`/`--help` cases for **this member** retire with delta (2)'s help arm; the
loop's other member, `enter-stage.sh`, keeps its own unchanged, so the loop
narrows rather than disappears.

### (7) Three live entries name this tool as their subject; their premises narrow and their verdicts do not

Each is corrected in place to name the arm, and **no verdict moves** {mechanical}:

- `gap-inbox-commit-ownership` — proposes the tool commit its own bullet. As open
  against a compiled arm as against the script.
- `recurrence-resolver-literal-match-only` — names "`bin/file-gap.sh`'s live-slug
  scan" as the mechanism at issue. Delta (3) reproduces that scan unchanged, so
  the entry's subject survives with a new spelling.
- `gap-capture-argv-prompt-friction` — measures this tool's prompting cost and
  already anticipates this cut by name: "the capture tool's port to a compiled
  arm moves where that shape is written and settles nothing here". That sentence
  stays true and is **not** edited into a claim of progress. Its one stale fact is
  the grant citation, which delta (8) moves.

Recorded as a delta rather than left for a later scope to rediscover, because an
entry whose stated premise a landed cut falsified reads as settled work. Taking
any of the three inside a port cut is non-port design work the composer refuses.

### (8) Every path-bearing surface moves in the deleting commit

The count is probed rather than assumed {mechanical}:

- `.claude/settings.json` — **one** grant names the deleted path,
  `Bash(bash lifecycle-kit/bin/file-gap.sh *)`. Removing a grant whose target a
  ruled port cut deletes is outside the 2026-08-22 bar under the operator's
  2026-08-29 settings-grant carve-out, and the removal lands in the same commit
  as the delete. **No replacement grant is needed**, probed rather than assumed:
  `Bash(bash gate-sdk/bin/run-gates.sh *)` is already granted.
  `check-settings-paths` is the oracle, and its red condition is a literal `.sh`
  grant that does not resolve, so the deletion without its grant line reds it.
- `CLAUDE.md`'s gap-capture bullet, which carries the invocation form verbatim.
- `.claude/commands/consult.md` and `lifecycle-kit/templates/lead.md`, which name
  `bin/file-gap.sh` as the mid-iteration filing channel;
  `lifecycle-kit/templates/stages/close.md`, which names it as the drain's filer.
  The two templates are the sources; the commands file is this consumer's spliced
  copy and `check-template-copy-parity` is its oracle.
- `lifecycle-kit/README.md`'s tool-roster line.
- `lifecycle-kit/SPEC.md` — eight prose citations of the path inside this
  section and its neighbours.
- `gate-sdk/SPEC.md` §The bin/-tool contract, whose worked example is
  `bash lifecycle-kit/bin/file-gap.sh -- "--list is captured at exit 0"`.
- `.workflow/gap-inbox.md:1` — the live contract header ends `(filed via
  lifecycle-kit/bin/file-gap.sh)`. It is a committed surface, no tool rewrites
  it after seeding, and its trailing path is dead after the cut.
- `lifecycle-kit/smoke/install.sh` (delta 6) and
  `lifecycle-kit/gate-tests/file-gap-recurrence.test.sh` (delta 6).
- `drift-kit/bin/kfric.sh` is **not** on this list and is named so the omission is
  legible: the dependency runs the other way — `bin/file-gap.sh:2` cites *kfric*
  as its pattern, not the reverse.
- The two dated release posts under `docs/posts/` are **out of corpus** and are
  not edited: they are version-pinned historical prose, the standing disposition
  for that surface.

### (9) The regeneration fan-out this cut stales

Deleting one owed `.sh` file moves `measured-claims.sh`'s `tree-shell-owed` key,
read off `--emit port-blockers --tree`'s trailer {mechanical}.
docs/site-architecture.md §Generated projections rules that a tree edit moving a
measured claim stales the generated `pre-commit` and `commit-msg` hooks — the
baked invocation carries `check-measured-claim`'s resolved values — and
`docs/check-graph.html` with them. The edits to `lifecycle-kit/SPEC.md`,
`gate-sdk/SPEC.md` and `lifecycle-kit/README.md` additionally stale their on-site
mirrors. All are rostered with their regen commands in that section and are
discharged in the landing commit; `check-graph`, `check-docs-mirror-fresh` and
`check-gate-binary-fresh` are the reds.

### (10) The criterion-5 residual is an affordance, not the surface

Criterion 5 asks what a consumer whose payload carries no artifact for its host
still has after the cut {design-bearing}. This cut has **no gate in it**, so the
binary-less leg's roster and its non-zero count do not move.

What such a consumer loses is one affordance, and **not** the surface it serves.
§The committed gap inbox already rules the raw append "a legal fallback, the
grammar being the surface's contract, not the writer"; `check-gap-inbox-neutrality`
is already compiled and stays the assertion; the boundary check and the union-merge
membership live in `bin/enter-stage.sh` and `lib/stages.sh`, which delta (1) shows
stay shell in this cut. So an artifact-less consumer still files gaps by hand,
still has them held to the neutrality bound, still has them refuse an iteration
boundary, and still has them union-merged. What it loses is the dated stamp, the
live-slug prompt and the two cursor advisories — three prompts, no contract. That
is accepted and stated in those terms rather than as a bare "advisory tooling"
label.

## Producers and consumers

The amendment introduces **one interface** — a single bridged flag — and **no new
state, no new event, no new field, and no new knob**. All five knobs named are
already shipped, already defaulted in `lifecycle-kit/lib/stages.sh` (which stays
permanently shell as the bridge's sole `LIFECYCLE_KIT_*` resolver, so no default
moves in this cut) and already read by the script being replaced.

- **Producer** — `native/src/emit/mod.rs`'s `BRIDGED_ARMS` table, one row
  (`"--emit-file-gap"`, `Arm::Emit(file_gap::emit)`, `file_gap::KNOBS`), with no
  `run-gates.sh` change: the front-end composes `--emit-<name>` from its
  `--emit <name>` operand and passes the remaining argv, a leading `--` included,
  through untouched. The enabling config is the table row itself — `--knobs`
  publishes the roster and `gate_command` resolves it before the exec — so
  nothing must be configured per install.
- **Consumer** — a **session that surfaced a work-shaped finding mid-iteration**,
  invoking it through the front-end at the moment the finding is in mind; the
  channel is live everywhere the kit is vendored because all five knobs carry
  shipped defaults, so the deployed configuration that must be set is none. Its
  stdout bullet is read by that session at that transition; its three stderr
  advisories by the same session at the same one.
- **Consumers of the appended bullet**, all three already named in the section
  and none of them new: `bin/enter-stage.sh`'s iteration-boundary check at every
  first-stage entry, `check-gap-inbox-neutrality` at commit time, and **close's
  drain**, the one party the live-slug advisory is written for.
- **Consumer of the declared roster** — `gate_command`, which resolves the five
  knobs by sourcing `lifecycle-kit/lib/stages.sh` and refuses the whole
  environment for a knob it does not define. All five are defined there today at
  the lines delta (2) cites, which is why this cut moves no default.

**The arm has a caller that is not a test** — the CLAUDE.md bullet mandating
in-the-moment capture, invoked by any session — which is the third property
§The non-gate arm requires. The re-homed gate-test of delta (6) is a *second*
caller rather than the qualifying one, so this cut leaves
`gate-test-in-tree-invoker-ruling` standing.

**One corpus is narrowed, and its readers' red conditions are enumerated rather
than their subjects** (canon-kit/SPEC.md §The causal-completeness check, point 5).
The narrowing is the deletion of one file from the tracked `*.sh` tree:

- `check-settings-paths` — reds on a literal repo-relative `.sh` grant that
  **does not resolve**, so its verdict is *not* monotone under this narrowing:
  removing the file **adds** a violation. Cleared by delta (8), in the same
  commit, not by inspection.
- `check-measured-claim` — `tree-shell-owed` moves by one. Whether a governed
  sentence pins it is checked by scanning the `measured:` markers rather than
  assumed; the derived consumers are the baked hook invocations, cleared by
  delta (9).
- `check-graph` / `check-docs-mirror-fresh` / `check-gate-binary-fresh` — red on a
  stale hook, artifact, mirror or binary, and non-monotone for the same
  baked-value reason. Cleared by delta (9).
- `check-template-copy-parity` — reds when a spliced consumer copy diverges from
  its kit template, so editing the template without the copy **adds** a violation.
  Not monotone; cleared by delta (8) editing both sides in one commit.
- `check-gate-fixture-coverage` — named because a reader assumes a deleted tool
  moves it, and it does not: no `.gate` descriptor and no fixture pair is owed or
  removed here, the member never having been a gate.
- `check-gap-inbox-neutrality` — unaffected in verdict in either direction: its
  corpus is the inbox file, not the tree. Only its remedy text changes (delta 5).
- `check-exec-bit`, `check-shellcheck`, `check-comment-tier`, `check-path-dialect`
  — monotone in the scanned `.sh` set; removing one file can only remove
  findings, cleared by inspection.

**Cross-component signal: this amendment's component set is two** —
lifecycle-kit and gate-sdk (§The non-gate arm's class roster and §The bin/-tool
contract's worked example) — so `check-stage-entry` assertion C fires and the
**align stamp is demanded at the build stage's entry**. Stated here so the build
session is not the one that learns it.

## Existing sections updated

- `lifecycle-kit/SPEC.md §The committed gap inbox` — the affordance is restated as
  its arm: the invocation form, the surviving argv-shape refusal and the retired
  help arm, the seeded contract header, the one bullet shape, the three stderr
  advisories, and the five declared knobs. **The residue is stated in this section
  rather than implied**: the section names its three implementations, which one
  this cut takes, that `bin/enter-stage.sh` ports in another cut, and that
  `lib/stages.sh` is permanently shell so the section's contract will not be
  wholly in-crate while the kit-library class ruling stands (deltas 1 and 2).
- `lifecycle-kit/SPEC.md §The committed gap inbox`, the live-slug paragraph — the
  predicate is reproduced rather than collapsed, with the reason (a different
  corpus, grammar-scoped against knob-scoped) and the honest note that the
  cross-kit-cycle premise describes a shell arrangement the substrate no longer
  has (delta 3).
- `lifecycle-kit/SPEC.md §lib/stages.sh` — the closing-stage predicate's hoisting
  paragraph gains the sentence that its by-construction guarantee now holds
  inside each substrate and not across them, until `enter-stage.sh` ports
  (delta 4).
- `lifecycle-kit/SPEC.md §bin/enter-stage.sh` — its gap-inbox boundary-check and
  discriminator paragraphs gain the sentence that they are this section's
  surviving shell half, so a later cut selector meets the fact where it works
  rather than in the other section (delta 1).
- `gate-sdk/SPEC.md §The non-gate arm` — the class roster gains
  `--emit-file-gap`; the row carries the fact that this is the class's **second**
  free-text capture member, so the argv-shape/help split the section states for
  `--emit-file-survey` is now a rule with two instances rather than one worked
  example (deltas 2 and 6).
- `gate-sdk/SPEC.md §The bin/-tool contract` — its worked example names the arm,
  and the existing pointer that the shape half outlives a member's port gains
  this second instance (delta 2).
- `CLAUDE.md`'s gap-capture bullet, `lifecycle-kit/README.md`'s tool-roster line,
  `lifecycle-kit/templates/lead.md`, `lifecycle-kit/templates/stages/close.md`,
  `.claude/commands/consult.md`, and `.workflow/gap-inbox.md`'s contract
  header (delta 8).
- `TASK-QUEUE.md`, the `native-gate-port-remaining-corpus` entry — promoted with
  `[design-pending]` swapped for this amendment's `[spec:]` ref; it **demotes** at
  build and never reaches `## Done`, which its own body already rules (all
  deltas). The demotion re-prices it against `check-queue-entry-budget`, unlike a
  Done move, so any roster this amendment leaves on the entry is priced against
  the 50-line cap in the demoting commit.
- `TASK-QUEUE.md`, the `gap-inbox-commit-ownership`,
  `recurrence-resolver-literal-match-only` and `gap-capture-argv-prompt-friction`
  entries — premises corrected to name the arm, verdicts unchanged (delta 7).
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
      component (`ls lifecycle-kit/SPEC-*.md`), the none-remain half discharged
      at the iteration rather than at the commit.
- [ ] **Removals propagated** — grepped every spec, skill, template, README,
      smoke script, gate-test, settings file, committed workflow surface and
      **compiled gate remedy string** for the deleted path; nothing dangles.
- [ ] **The residue is written, not implied** — §The committed gap inbox names all
      three implementations and the permanence of the `lib/stages.sh` half; a
      reader who never saw this amendment can tell the section is not discharged.
- [ ] **The matcher's oracle survives the port** — every one of the nine scenarios
      `file-gap-recurrence.test.sh` asserts today still has a holder, the
      no-queue-write invariant end-to-end and the grammar cases in-module.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred); delta (3)'s open collapse question is filed in the spec session
      itself rather than at build.
- [ ] **The oracle moved, and by the roster rather than by a number** — the
      `--tree` arm lists no `lifecycle-kit/bin/file-gap.sh` row, taken as a
      per-file roster diff and not as a trailer delta.
- [ ] **The regeneration fan-out is discharged in the landing commit** — the
      generated hooks, the graph artifact, the SPEC and README mirrors, and the
      gate binary.
