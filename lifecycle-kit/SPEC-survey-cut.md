# SPEC amendment: survey-cut

The port disposition of **lifecycle-kit's two `bin/` affordances behind §The
survey record** — `bin/file-survey.sh` (69 lines) and `bin/cite-survey.sh` (80),
149 lines — onto the binary substrate as non-gate arms. This is one of the
iteration's two port cuts under the port-only run (TRAJECTORY.md §PRIORITY
DIRECTIVE), composed at scope and ruled option B by the **lead on its own
authority**, 2026-09-01, over the resume channel; it did not reach the operator,
and that is stated because a composition ruling recorded without its authority
reads at the post-port triage as more settled than it is.

**Measured at this HEAD rather than carried from the survey**: the port oracle's
`--tree` arm reads 122 files scanned, 64 declared `no-port`, 0 temporarily held,
**58 owed**. This cut takes two of that column and **no ported member is a
gate** — both are advisory tooling that has never joined `gates.list`, the
assertion half of this section having ported already as
`native/src/gates/survey_record.rs`.

## What changes

### (1) The cut is two of this section's four implementations, and half the residue is permanent

`native-gate-port-remaining-corpus`'s composer ruling of 2026-08-28 selects a cut
**by stated contract** — the owed files behind one specification section, ported
behind the one amendment that section needs {design-bearing}. Both members
declare `lifecycle-kit/SPEC.md §The survey record` in their own `# spec:`
headers: `file-survey.sh:2` "the capture affordance; stamps the block grammar",
`cite-survey.sh:2` "the citation affordance: emits one record block as an
inline-ready snippet".

**Taking this cut does not discharge the section, and the amendment states that
rather than implying it.** Four shell files implement §The survey record; the cut
is two of them, and neither of the other two is reachable by this cut, because
the composer selects by **stated contract** and each declares a different section
in its own header:

- `bin/enter-stage.sh:610` implements the section's **read trigger** — the entry
  report that prints the record's headings and never its findings — and its
  boundary truncation at `:558`, where the record is a **kit built-in** member of
  the reset rather than a `LIFECYCLE_KIT_BOUNDARY_TRUNCATE` entry. Its header at
  line 2 declares §bin/enter-stage.sh. The file is owed at 617 lines and ports in
  a **different** cut.
- `lib/stages.sh:71` implements `LIFECYCLE_KIT_PERMANENT_SURFACE_GLOBS`, the
  no-retrieval-pointer surface list. Its header at line 2 declares §lib/stages.sh,
  and that file is **header-declared `no-port`** under the 2026-08-30 kit-library
  class ruling, as the config bridge's sole resolver for the `LIFECYCLE_KIT_*`
  knobs. That is structural rather than a sizing judgment, so **this section's
  contract will never be wholly in-crate while that class ruling stands.**

**The general shape, which outlives this cut:** a stated-contract cut ports the
files that **declare** its section, not every file that **implements** it, and
the two sets come apart wherever a shared entry point or a config library carries
one clause of another section's contract. A later cut selector should expect this
rather than treat it as this section's peculiarity.

**The census behind that finding is carried rather than re-bought, and its
witness is inlined here rather than pointed at** — the record it sits in is
boundary-truncated scratch, so a pointer into it resolves to nothing one
iteration after this is written, which is this very section's own rule. Question:
*does the lifecycle-kit §The survey record cut port every implementation of that
section's contract?* — corpus `lifecycle-kit/**/*.sh x the literal 'The survey
record'`; oracle `grep -rn 'The survey record' --include=*.sh .` together with
`bash gate-sdk/bin/run-gates.sh --emit port-blockers --tree`; rev
`6f9fdcd276f66a4ada1bec66af94e46a166ed57c`; edges `none`; finding: **no** — the
cut leaves the two-file residue above and one half of it is permanent. Raised as
a claim by the lead, **run** at scope rather than relayed, and re-witnessed at
this stage's entry: the corpus diff since that rev is clean, the grep set is
unchanged at four implementations plus two gate-tests, and the oracle still reads
58 owed. Two gate-tests also cite the section and are outside the port corpus by
the `*.test.sh` suffix rule.

### (2) `--emit-file-survey` — the capture affordance, five positionals and a stamp

`bin/file-survey.sh` becomes an `Arm::Emit` member of `BRIDGED_ARMS` spelled
`--emit-file-survey`, reachable through the shipped front-end as
`run-gates.sh --emit file-survey [--] "<question>" "<corpus>" "<oracle>" "<edges>" "<finding>"`
with no front-end change {design-bearing}. Its declared reads are
`LIFECYCLE_KIT_SURVEY_RECORD_FILE` and `LIFECYCLE_KIT_STATE_FILE`. The family is
**forced, not chosen** by §The non-gate arm's test: the tool resolves consumer
knobs, and a hardcoded top-level flag would resolve platform defaults while
silently ignoring every override.

**The `--emit-` spelling is taken for a member that appends rather than renders,
and the precedent is in the class already.** §The non-gate arm records that
`--emit-queue-index extent` "answers two integers and emits none, so the
`--emit-` spelling reads as a stretch — it is precedented and taken rather than
renaming the family", and that the table is keyed by the arm's own flag
spelling with `--emit-` demoted to a per-arm spelling. `EmitFn`'s returned string
is documented as "the document, or **the action a write performed**", which is
exactly this member's stdout line; the write-in-place emitters
(`--emit-docs-mirror --write` and its siblings) are the shape it joins.

Six behaviours are the contract and each survives explicitly:

- **Five positionals, each required non-empty**, in the fixed order
  question / corpus / oracle / edges / finding. The `edges` slot **takes no
  default**: an omitted fifth argument is arity misuse the arm refuses, so a
  session that forgot the field is told at filing time by the producer rather
  than at commit time by the gate.
- **The free-text shape refusal survives the substrate, and it survives on every
  positional.** A positional beginning with `-` that is not preceded by `--` is
  a refusal at exit 2, and `--` ends option processing. The hazard the rule
  exists for — a flag captured into a committed surface at exit 0, attested three
  times — is a property of free text reaching a capture tool, not of the tool
  being a `bin/` script, so it does not retire with the substrate. Five slots
  make arity no protection at all, which is why the scan is over every
  positional and not the first.
- **`-h` / `--help` does not port.** Usage for a bridged arm lives in
  `run-gates.sh`'s own help and in `lifecycle-kit/README.md`, which is where the
  class already keeps it, and a per-arm help flag would be a second home for one
  sentence — the disposition `--emit-queue-counts` took at its own port. The
  substantive half of the `bin/`-tool contract is preserved by the bullet above:
  `--emit file-survey --help` is a **refusal**, never a capture.
- **`rev` is machine-stamped and a tree with no HEAD is a refusal.** The arm
  runs `git rev-parse HEAD` through `proc::run`, the crate's one spawn site,
  requires a 40-hex result, and exits 2 with today's message otherwise. `git`
  joins the arm's spawned-program set. This is the load-bearing decision in the
  tool: `rev` is the field the whole re-use protocol turns on and the one an
  author gets wrong.
- **The stage is derived from the cursor, never asked for.** The arm reads
  `LIFECYCLE_KIT_STATE_FILE` and calls the crate's existing
  `stages::current_stage`, stamping the never-named `—` when the cursor is
  absent. That knob is declared for this reason and no other.
- **The contract header is seeded when the record does not exist**, byte-identical
  to today's, so a fresh consumer's first filing produces a record the boundary
  truncation can reduce to exactly that line.

The stdout confirmation line and the two stderr advisories — the `oracle: none`
note-not-a-survey warning and the witness-command hint — are byte-preserved,
stdout as the arm's returned string and stderr written directly.

**The repo-root anchor moves with the tool.** The shell form `cd`s to the git
toplevel before resolving the record path, so a relative
`LIFECYCLE_KIT_SURVEY_RECORD_FILE` means the same file from any subdirectory;
the arm resolves it against `walk::toplevel` for the same reason, falling back to
the working directory exactly as today.

### (3) `--emit-cite-survey` — the citation affordance, refusals unchanged

`bin/cite-survey.sh` becomes `--emit-cite-survey`, an `Arm::Emit` taking one
positional off the arm's own argv slice {mechanical}. Its declared read is
`LIFECYCLE_KIT_SURVEY_RECORD_FILE` alone — it derives no stage and stamps no rev,
so delta (2)'s second knob is deliberately not on its roster.

Its rule is small and fully specified by the section, so what the port owes is
parity rather than design: select the one block whose `## ` heading **contains**
the substring; print the heading reformatted as `**Carried survey — <heading>**`
followed by the five `- <key>: <value>` lines in record order; refuse at exit 2
on no record, on no match, and on an ambiguous match. All three refusals are exit
2 today and stay 2 under `Arm::Emit`, so **no observable moves** in this member.
The no-match refusal keeps printing the record's headings and the ambiguous one
keeps printing every match, because the author asked for one finding and a
silently-chosen sibling would be pasted onto a permanent surface as if it were
the one they read. The `--` escape and the leading-`-` refusal survive on
delta (2)'s ground, and `-h`/`--help` retires on delta (2)'s.

### (4) Two compiled gates prescribe a shell path in their remedy text

`native/src/gates/survey_record.rs:266` prints, on the red path,
`File blocks with 'bash lifecycle-kit/bin/file-survey.sh …', which stamps the rev
itself`; `native/src/gates/scratch_citation.rs:222` prints
`bash lifecycle-kit/bin/cite-survey.sh "<heading-substring>" emits the block's
heading and …`. Both are user-facing remedy text naming a path this cut deletes,
and both move in the same commit {mechanical}. Neither pair's `expect.txt` pins
the string today — probed, not assumed — so the change is a source edit; the
build re-runs both fixture pairs rather than relying on that probe.

These two literals are the reason a port of an advisory tool can strand a
*gate*: the gate stays green while telling its reader to run a command that no
longer exists, which no oracle catches.

### (5) The behavioural coverage narrows to the seam a crate test cannot see

`lifecycle-kit/smoke/install.sh` is the **only** surface in the tree that
exercises either tool's argv contract, and it does so at three sites: line 70
seeds a real survey block, and lines 244-273 drive `--help`, `-h`, a `--list`
refusal and — for `file-survey.sh` alone — a flag in the **fifth** slot,
asserting exit 2 and asserting that neither the help nor the refusal path wrote
to the record. No `gate-tests/*.test.sh` invokes either tool: both existing
suites hand-write record fixtures and drive the gate and `enter-stage.sh`
instead.

Each site narrows to the shape `queue-kit/gate-tests/queue-index.test.sh` took at
its own port {design-bearing}: it holds "the seam a crate unit test cannot see —
that the battery runner's `--emit` front-end resolves the arm at all, and that a
set consumer knob actually reaches the rendering through the shell bridge", while
the rendering itself is pinned in the ported module's own `#[cfg(test)]` tests
where `check-crate-arms` runs them. So the smoke keeps its **discriminating**
cases — the fifth-slot flag refused at exit 2 and the record untouched on a
refusal path, both driven through `run-gates.sh --emit` — and hands the grammar
cases to the module. The `-h`/`--help` cases for these two members retire with
delta (2)'s help arm; the loop's other members, `file-gap.sh` and
`enter-stage.sh`, keep theirs unchanged, so the loop narrows rather than
disappears.

### (6) Two live entries name `file-survey.sh` as their subject; their premises narrow and their verdicts do not

`gap-capture-argv-prompt-friction` and `survey-oracle-liveness-unasserted` each
propose a behaviour change to this tool and each states its premise by path
{mechanical}. Both are corrected in place to name the arm, and **neither verdict
moves**: a `--from <path>` arm and an oracle-liveness assertion are as open
against the compiled member as against the script. Recorded as a delta rather
than left for a later scope to rediscover, because an entry whose stated premise
a landed cut falsified reads as settled work. Taking either inside a port cut is
non-port design work the composer refuses.

### (7) Every path-bearing surface moves in the deleting commit

The count is probed rather than assumed {mechanical}:

- `.claude/settings.json` — **one** grant names a deleted path,
  `Bash(bash lifecycle-kit/bin/file-survey.sh *)`. Removing a grant whose target
  a ruled port cut deletes is outside the 2026-08-22 bar under the operator's
  2026-08-29 settings-grant carve-out, and the removal lands in the same commit
  as the delete. **No replacement grant is needed**, probed rather than assumed:
  `Bash(bash gate-sdk/bin/run-gates.sh *)` is already granted.
  `check-settings-paths` is the oracle, and its red condition is a literal `.sh`
  grant that does not resolve, so the deletion without its grant line reds it.
- **A finding the census bought, recorded because the port discharges it**:
  `cite-survey.sh` has **no** grant at all, so every invocation of the affordance
  this section calls "one command" has cost an out-of-band permission decision
  since it landed. The port resolves it as a side effect — the replacement rides
  the existing `run-gates.sh` grant — and it is stated so the discharge is
  legible rather than accidental.
- `CLAUDE.md`'s survey-capture bullet, which carries the invocation form
  verbatim.
- `lifecycle-kit/README.md`'s gate-roster line, which names `bin/file-survey.sh`
  as what makes `check-survey-record` non-inert.
- `lifecycle-kit/smoke/install.sh` (delta 5).

### (8) The regeneration fan-out this cut stales

Deleting two owed `.sh` files moves `measured-claims.sh`'s `tree-shell-owed`
key, read off `--emit port-blockers --tree`'s trailer {mechanical}.
docs/site-architecture.md §Generated projections rules that a tree edit moving a
measured claim stales the generated `pre-commit` and `commit-msg` hooks — the
baked invocation carries `check-measured-claim`'s resolved values, "from a file
no manifest names either" — and `docs/check-graph.html` with them. The merge into
`lifecycle-kit/SPEC.md` and the `README.md` edit additionally stale the on-site
mirrors. All are rostered with their regen commands in that section and are
discharged in the landing commit; `check-graph` and `check-docs-mirror-fresh` are
the reds.

### (9) The criterion-5 residual is an affordance, not the surface

Criterion 5 asks what a consumer whose payload carries no artifact for its host
still has after the cut {design-bearing}. This cut has **no gate in it**, so the
binary-less leg's roster and its non-zero count do not move.

What such a consumer loses is the two affordances — and **not** the surface they
serve, which is the material difference from this iteration's sibling cut. §The
survey record already rules the raw append "the sanctioned fallback", the grammar
being the surface's contract rather than the writer; `check-survey-record` is
already compiled and stays the assertion; and the section's read trigger and
boundary truncation live in `bin/enter-stage.sh`, which delta (1) shows stays
shell in this cut. So an artifact-less consumer still files surveys by hand,
still has them asserted, still has them printed at every stage entry and still
has them truncated at the boundary. The residual is one convenience and one
witness hint. That is accepted and stated in those terms rather than as a bare
"advisory tooling" label.

## Producers and consumers

The amendment introduces **two interfaces** — one bridged flag each — and **no
new state, no new event, no new field, and no new knob**. Both knobs named are
already shipped, already defaulted in `lifecycle-kit/lib/stages.sh` (which stays
permanently shell as the bridge's sole `LIFECYCLE_KIT_*` resolver, so no default
moves in this cut) and already read by the two scripts being replaced.

- **Producer** — `native/src/emit/mod.rs`'s `BRIDGED_ARMS` table, one row per
  arm, with no `run-gates.sh` change: the front-end composes `--emit-<name>` from
  its `--emit <name>` operand and passes the remaining argv, including a leading
  `--`, through untouched. The enabling config is the table row itself —
  `--knobs` publishes each member's roster and `gate_command` resolves it before
  the exec — so nothing must be configured per install.
- **Consumer of `--emit-file-survey`** — a **session that bought a survey**,
  invoking it through the front-end at the moment the finding is bought; the
  channel is live everywhere the kit is vendored because both knobs carry shipped
  defaults, so the deployed configuration that must be set is none. Its stdout
  confirmation is read by that session at that transition; its stderr witness
  hint by the same session at the same one. The **block it appends** is read by
  three parties already named in the section: `bin/enter-stage.sh` at every stage
  entry (headings only), `check-survey-record` at commit time, and the next stage
  session at its pre-dispatch witness.
- **Consumer of `--emit-cite-survey`** — a **session about to carry a finding
  onto a permanent surface**, invoking it through the front-end and pasting its
  stdout; and, at one remove, **`check-scratch-citation`**, whose red-path remedy
  is the instruction to run it (delta 4). That gate is a caller of the *text*
  rather than of the arm, and it is named because delta (4) is otherwise a
  string edit with no stated reader.
- **Consumer of the declared rosters** — `gate_command`, which resolves each
  arm's knobs by sourcing `lifecycle-kit/lib/stages.sh` and refuses the whole
  environment for a knob it does not define. Both knobs are defined there today,
  which is why this cut moves no default and why that is worth saying: the
  sibling cut's delta (6) is not a general obligation of a tool port, it is the
  consequence of a driver that held its own defaults inline.

**Both arms have a caller that is not a test**, enumerated above, which is the
third property §The non-gate arm requires; the smoke of delta (5) is a *second*
caller rather than the qualifying one, so this cut leaves
`gate-test-in-tree-invoker-ruling` standing.

**One corpus is narrowed, and its readers' red conditions are enumerated rather
than their subjects** (§The causal-completeness check, point 5). The narrowing is
the deletion of two files from the tracked `*.sh` tree:

- `check-settings-paths` — reds on a literal repo-relative `.sh` grant that
  **does not resolve**, so its verdict is *not* monotone under this narrowing:
  removing the file **adds** a violation. Cleared by delta (7), in the same
  commit, not by inspection.
- `check-measured-claim` — `tree-shell-owed` moves by two. No governed sentence
  pins it, checked by scanning the `measured:` markers rather than assumed; the
  derived consumers are the baked hook invocations, cleared by delta (8).
- `check-graph` / `check-docs-mirror-fresh` — red on a stale hook, artifact or
  mirror, and non-monotone for the same baked-value reason. Cleared by delta (8).
- `check-exec-bit`, `check-shellcheck`, `check-comment-tier`, `check-path-dialect`
  — monotone in the scanned `.sh` set; removing two files can only remove
  findings, cleared by inspection.
- `check-survey-record` — unaffected by the narrowing in either direction: its
  corpus is the record file, not the tree, and its inert-on-absent-record arm is
  untouched. Named because it is the gate a reader assumes this cut moves.
- `check-scratch-citation` — likewise unaffected in verdict; only its remedy text
  changes (delta 4).

**Cross-component signal: this amendment's component set is two** —
lifecycle-kit and gate-sdk (§The non-gate arm's class roster) — and a sibling
amendment lands in `context-kit/` this iteration, so `check-stage-entry`
assertion C fires on both counts and the **align stamp is demanded at the build
stage's entry**. Stated here so the build session is not the one that learns it.

## Existing sections updated

- `lifecycle-kit/SPEC.md §The survey record` — both affordances are restated as
  their arms: the invocation forms, the surviving argv-shape refusal and the
  retired help arm, the machine-stamped `rev` and derived stage, the seeded
  contract header, and the three citation refusals. **The residue is stated in
  this section rather than implied**: the section names its four implementations,
  which two this cut takes, that `bin/enter-stage.sh` ports in another cut, and
  that `lib/stages.sh` is permanently shell so the section's contract will not be
  wholly in-crate while the kit-library class ruling stands (deltas 1, 2 and 3).
- `lifecycle-kit/SPEC.md §The survey record`, the producers-and-consumers
  paragraph — the producer is the arm; the three readers of a filed block are
  unchanged and stay stated (delta 2).
- `lifecycle-kit/SPEC.md §bin/enter-stage.sh` — its read-trigger and
  boundary-truncation paragraphs gain the sentence that they are the section's
  surviving shell half, so a later cut selector meets the fact where it works
  rather than in the other section (delta 1).
- `lifecycle-kit/SPEC.md §check-scratch-citation` — its remedy names the arm
  (delta 4).
- `gate-sdk/SPEC.md §The non-gate arm` — the class roster gains the two members;
  the `--emit-file-survey` row carries the sentence the class does not yet hold,
  that a member taking **free-text argv into a capture** keeps its shape refusal
  and `--` escape across the port while its `-h`/`--help` arm retires to the
  front-end, because the hazard belongs to the argument and the help belongs to
  the substrate (deltas 2 and 3).
- `gate-sdk/SPEC.md §The bin/-tool contract` — a pointer that the shape half of
  the contract outlives a member's port even though the contract's own corpus is
  `*/bin/*.sh`; without it the rule reads as retiring with the file (delta 2).
- `CLAUDE.md`'s survey-capture bullet and `lifecycle-kit/README.md`'s gate-roster
  line (delta 7).
- `TASK-QUEUE.md`, the `native-gate-port-remaining-corpus` entry — promoted with
  `[design-pending]` swapped for this amendment's `[spec:]` ref; it **demotes** at
  build and never reaches `## Done`, which its own body already rules (all
  deltas).
- `TASK-QUEUE.md`, the `gap-capture-argv-prompt-friction` and
  `survey-oracle-liveness-unasserted` entries — premises corrected to name the
  arm, verdicts unchanged (delta 6).
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
      at the iteration rather than at the commit, this iteration carrying a
      sibling amendment.
- [ ] **Removals propagated** — grepped every spec, skill, template, README,
      smoke script, settings file and **compiled gate remedy string** for the two
      deleted paths; nothing dangles.
- [ ] **The residue is written, not implied** — §The survey record names all four
      implementations and the permanence of the `lib/stages.sh` half; a reader who
      never saw this amendment can tell the section is not discharged.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **The oracle moved, and by the roster rather than by a number** — the
      `--tree` arm lists no `lifecycle-kit/bin/file-survey.sh` and no
      `lifecycle-kit/bin/cite-survey.sh` row, taken as a per-file roster diff and
      not as a trailer delta.
- [ ] **The regeneration fan-out is discharged in the landing commit** — the
      generated hooks, the graph artifact, the SPEC and README mirrors, and the
      gate binary.
