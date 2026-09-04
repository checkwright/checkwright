# SPEC amendment: diff-cut

The port disposition of **`evidence-kit/bin/diff-baseline.sh` (53 lines), the one
owed file declaring §bin/diff-baseline.sh**, onto the binary substrate as a bridged
`Arm::Run` member spelled `--diff-baseline`. This is a stated-contract cut under the
port-only run (TRAJECTORY.md §PRIORITY DIRECTIVE), composed at scope and packaged by
the **lead on its own authority**, 2026-09-04, over the resume channel; it did not
reach the operator, and that is stated because a packaging ruling recorded without
its authority reads later as more settled than it is.

**Measured at this HEAD rather than carried from scope's survey**: the port oracle's
`--tree` arm reads 98 files scanned, 64 declared `no-port`, 0 temporarily held,
**34 owed**. The selection ground is that owed column and never the registry
`--group` read (gate-sdk/SPEC.md §port-blockers). This cut takes one file of that
column, and the ported member is not a gate — §bin/diff-baseline.sh opens by ruling
the tool "The situational runtime diff, not a precommit gate".

**This cut consumes rather than pays.** Three of the twins it needs —
`ek_parser_for`, `ek_parse` and `ek_diff` — are paid into `native/src/evidence.rs`
by the sibling cut on §bin/run-validate.sh. That is a **between-cuts economy and not
a section pairing**: the two are separate `###` headings, neither amendment claims a
shared proof, and the parent `## Per-component contracts` is deliberately not read
here as a cut boundary — whether a `##` parent may bound a cut is the open question
`cut-boundary-section-legality-unruled` holds, which a scheduled consult will reach.
The dependency is an **ordering**, not a joint proof: this cut lands after its
sibling, and if it were taken alone it would pay the three twins itself at the
sibling's price. What it may not do is land while the sibling has not, and then be
dropped — because the sibling landing alone leaves a live double implementation of
the parser and the diff that only this cut retires. So of the iteration's four cuts
this is the **worst** one to drop, which is stated here because the drop-first
permission sits on a different cut and a reader should not generalize it.

## What changes

### (1) The cut is the one owed file declaring §bin/diff-baseline.sh, and it discharges the kit's owed column

`bin/diff-baseline.sh` is the **only** owed file declaring `### bin/diff-baseline.sh`
{design-bearing}. With the sibling cut landed, this one takes evidence-kit's owed
column to **zero**: `lib/evidence.sh` is permanently shell as the config bridge's
sole `EVIDENCE_KIT_*` resolver (gate-sdk/SPEC.md §The kit-library port disposition)
and never enters the column, `scripts/evidence-config.sh` carries its own `no-port:`
cause as the seeded-copy side of the config seam, and the kit's one gate is already
compiled.

**And the discharge is the kit's, not the section's alone** — that is worth stating
because it is the first kit in this corpus to reach zero owed on a `bin/` column, and
a later composer reading the section as discharged would still have to re-derive
whether anything else in the kit is owed.

Three `# spec:` directives sit on the file — `:2` (the whole tool contract), `:24`
(the optional-status disambiguation) and `:29` (the exit-code refusal) — and all
three bind the one titular section, so this cut's directive reach and its section
bound coincide, unlike its sibling's.

### (2) `--diff-baseline` is a bridged `Arm::Run`, and the three-state exit forces the family

The tool lands as `native/src/emit/diff_baseline.rs` and registers as one
`BRIDGED_ARMS` row, with its own `case` arm in `gate-sdk/bin/run-gates.sh`
{design-bearing}.

**It cannot be an `--emit-` member.** `native/src/main.rs:483-491` maps `Arm::Emit`
onto `exit(0)`/`exit(2)`, a family that can never return **1** — and 1 is this
tool's verdict, "NEW failures against the baseline". Exit 2 is its refusal: bad
argument shape, an unreadable log, or an `exit-code` suite named without its status.
Collapsing the two would make *a real regression* indistinguishable from *the tool
was called wrong*, on a tool whose one caller is a CI leg that reads nothing but the
status. `Arm::Run` (`native/src/main.rs:493`) passes the arm's own `i32` through
verbatim, and `exec_arm` ends in a true `exec` (`gate-sdk/bin/run-gates.sh:127`), so
the status survives the front-end.

**The spelling is its own rather than `--emit-`.** `bin/run-gates.sh` composes
`--emit-<name>` from its `--emit <name>` operand, so spelling and grammar are one
decision (gate-sdk/SPEC.md §The non-gate arm). This member does emit findings on
stdout, so the family is not excluded by the document test — it is excluded by the
exit contract above, and stating that explicitly matters because the natural read of
"it prints a report" is that `--emit-` fits.

### (3) The declared knob roster

The arm declares five names {design-bearing}: `EVIDENCE_KIT_BASELINE_FILE`,
`EVIDENCE_KIT_SKIP_FILE`, `EVIDENCE_KIT_TMP_DIR`, `EVIDENCE_KIT_PARSER` and
`EVIDENCE_KIT_PARSER_*`.

The forced-family test settles the registration: all of them are defined and
defaulted in `evidence-kit/lib/evidence.sh` (`:34-48`), a `no-port` file the bridge
sources to resolve them, so a hardcoded top-level flag would resolve platform
defaults and ignore every consumer override. `EVIDENCE_KIT_PARSER_*` is a **prefix
family** and the family arm is the sibling cut's — `_gate_knob_prefix_emit`
(`gate-sdk/lib/gate.sh:318-350`) resolving inside the owning kit's already-sourced
subshell, received by `walk::knob_prefix` / `walk::knob_in_family`
(`native/src/walk.rs:220-237`). Whichever of the two cuts lands first mints that
declaration; this amendment states the roster it needs rather than assuming the
order.

**It declares no suite roster and needs none**: this tool's suites arrive on argv,
one per argument group, which is exactly what distinguishes it from the spine.

### (4) It consumes three compiled twins and pays none, and the shell forms retire here

`ek_parser_for`, `ek_parse` and `ek_diff` are called at `:30`, `:40` and `:41`
{design-bearing}. Their compiled twins land in `native/src/evidence.rs` with the
sibling cut; this cut calls them.

**What this cut adds is the retirement.** The caller census is closed and probed:
those three functions have exactly two production callers, this file and
`bin/run-validate.sh`, and every other hit in the tree is `lib/evidence.sh`'s own
dispatch or one of the library's gate-tests sourcing it directly to test the
adapter. So once both cuts land, no production shell caller remains — and the shell
forms come out of `lib/evidence.sh` in **this** cut's commit, not the sibling's.
That is the whole content of the between-cuts economy: the sibling creates a
duplication and this cut ends it.

**The library's gate-tests are the reason the removal is not automatic and must be
decided here.** `evidence-kit/gate-tests/evidence-lib.test.sh` and
`scripts/gate-tests/evidence-parser-values.test.sh` source the library and exercise
`ek_parse`, `ek_parser_for` and `ek_diff` directly. Deleting the shell functions
deletes what those suites assert, so this cut **re-points them at the compiled
twins** through the front-end rather than deleting the coverage — the same
disposition §The non-gate arm rules for a parity arm whose second holder is gone:
what stands in place of a comparison that can only skip is the comparison a session
runs before the delete, and after it the assertions belong to the surviving
implementation.

### (5) The argv-shape contract binds here, the port ADDS all three behaviours, and this is the class's fourth reader instance

`diff-baseline.sh` takes **free-text positionals** — `<suite> <logfile> [<status>]`,
repeated — and today it validates **arity only** {design-bearing}. Probed rather
than read off the usage text: there is no `-h`/`--help` branch anywhere in the file,
and no `--` escape. A first argument of `--help` is absorbed as a **suite name**,
carried into `ek_parser_for "--help"`, and the run proceeds against whatever the
second argument is.

**That absorption is the harm the contract exists against, reached from a fourth
direction.** gate-sdk/SPEC.md §The bin/-tool contract records three reader instances
— `--emit-scan-prompts` (a wrong number into a durable trend), `--usage-verdict` (a
non-reading dressed as a reading at a graded decision point), `--wait-probe` (an
absorbed *exit code*). Here the absorbed argument becomes a **suite name that
matches no baseline row**, and §Baseline manifest's fail-closed rule keys on `fail`
alone — so an observed set with no baseline rows and no observed `fail` produces
**no findings and exit 0**. A typo'd invocation therefore prints
`diff-baseline: clean` and the CI leg that reads its status goes green. That is the
same shape as the second instance and it is worse in one respect: the reader is not
a session that might notice, it is a workflow step whose only output is a check mark.

**So the split binds and the port works forward rather than preserving a shape.**
The shape refusal (a positional beginning with `-` that is not a recognized option
is a refusal — usage on stderr, exit 2) and the `--` escape **cross** to the arm.
The `-h`/`--help` arm **retires to the front-end**, where the class already keeps
usage. All three behaviours are additions the shell form never had, which is the
clause working forward, exactly as `--usage-verdict`'s port did.

**This member is not blocked on `bin-tool-help-arm-absent-tree-wide`, and the ground
is that entry's own scope question.** That entry holds the corpus open because the
contract's three behaviours are stated under a **free-text-positional** rule and
whether they bind a tool taking **no** positionals is unstated. This tool takes free
text, so it sits squarely inside the stated scope and the open question does not
reach it. This cut therefore applies the split per member, as that entry's own
sentence anticipates, and rules **nothing** about the members whose disposition
waits on the scope answer.

**A census note is owed at build and its derivation is not reproducible from the
entry as written.** Four of this iteration's cut members sit on that census, so it
moves by four. But the entry states 20 paths / 17 shipped tools measured on
2026-09-04, and the obvious spelling of its own derivation —
`git ls-files '*/bin/*.sh' | xargs grep -L -- '--help'` — returns **19** at this
HEAD, three of them gate fixtures, so **16** shipped. The entry does not state its
grep pattern, so the difference is unattributable rather than a proven staleness.
Build reproduces the census with the entry's own pattern before moving the number,
and files the unstated-pattern finding rather than guessing.

### (6) The optional-status disambiguation and the exit-code refusal cross verbatim

Both are contract and both survive {design-bearing}.

- **The third token is unambiguous rather than heuristic.** A suite name suffixes
  `EVIDENCE_KIT_RUN_<suite>`, so it is a shell identifier and can never be all
  digits, and a status can never be anything else. The compiled arm keeps the same
  total disambiguation — an all-digit token in the third slot is the status,
  anything else opens the next group.
- **An `exit-code` suite named without a status is refused at exit 2, never run.**
  §bin/diff-baseline.sh's ground crosses unchanged: assuming success there would
  make the tool report pass for every log it is ever handed, clearing reds it
  structurally cannot observe. The refusal's three-line help text — naming the
  argument that is missing and what its absence costs — is part of the contract and
  not decoration, because the caller who hits it is the one who cannot see the
  problem from the output.

**The interaction between deltas (5) and (6) is worth one sentence, because the
naive ordering breaks the disambiguation.** The shape refusal is applied to a
positional **before** the group parser consumes it, so `-x` in a suite slot is
refused rather than resolved as a parser name; but the all-digit test stays where it
is, inside the group, because a status is not free text and a leading `-` cannot
appear in it. Getting that order wrong would make a negative-looking token refuse
where the shell form accepted a well-formed group.

### (7) The skip channel's reader crosses; its producer is a consumer harness and stays outside the tree

`ek_diff` reads `EVIDENCE_KIT_SKIP_FILE` and demotes an observed `pass` to `skip`
before the pass/fail branch, so a self-skipped scenario cannot masquerade as a pass
{design-bearing}. That reader is the sibling cut's twin and this cut consumes it.

**What this amendment adds is the honest limit on the producer side, probed rather
than assumed.** No tracked file **writes** the skip file: `evidence-kit/SPEC.md`
rules it "produced by a **consumer harness** that self-skips a scenario", so the
producer is structurally outside this tree and the port neither gains nor loses one.

**And one stated property does not hold in this tree, which is a finding this cut
files rather than fixes.** The SPEC describes the skip file as truncated per run at
the scope boundary via `LIFECYCLE_KIT_BOUNDARY_TRUNCATE`; this repo's own
`scripts/lifecycle-config.sh` sets that knob to three paths and
`.workflow/validate-skips.txt` is **not** among them. The mechanism is real and
already compiled (`native/src/emit/enter_stage.rs` reads the knob live); it is this
consumer's membership that omits the file. That is a consumer-config gap, not a port
obligation, and this cut routes it to the gap inbox rather than widening its own
scope to fix it — a mid-session initiative is filed, not started.

### (8) The one functional caller is a CI leg, and NO local gate reaches it

`.github/workflows/gates.yml:134` runs
`bash evidence-kit/bin/diff-baseline.sh installer_smoke "$log" "$rc"` as the verdict
of the install-smoke job {design-bearing}. It is the tool's **only** functional
caller in the tracked tree — no smoke, no demo, no kit template, no `scripts/*.sh`
invokes it — and it is the sharpest fact in this cut.

**Workflow YAML is outside every governed-doc glob**, so a delete that misses this
line leaves the whole local battery green and reds only on the remote `gates` run.
That is a **deferred red** bought at the worst possible price: this repo budgets one
to two pushes per iteration and the failure surfaces on the one watched push, after
the commits are made.

**The leg's own comment reads like a blocker and is not one — probed, because the
opposite reading would have refused this cut outright.** `gates.yml:119-120` says the
step "Stays a bash `run:` on the runner's preinstalled toolchain (no setup-node, no
setup-rust) to keep the checkout+bash tamper floor the steps above state." The floor
that comment protects is *no `setup-*` action*, not *no binary*: the same job builds
the crate at `gates.yml:39-40` (`bash gate-sdk/bin/build-native.sh`, also on the
runner's preinstalled cargo and also deliberately without `setup-rust`), so the
binary exists by the time this step runs and a bridged arm is reachable there. The
re-pointed line is `bash gate-sdk/bin/run-gates.sh --diff-baseline installer_smoke
"$log" "$rc"`, which changes nothing about the step's toolchain posture.

**What the leg asserts must not change, and the amendment states it because the
re-point is one line and the property is three paragraphs.** That leg is green
against a **recorded baseline**, never green simpliciter: the suite exits non-zero
today at its binary-less arm on a ruled outcome held in
`.workflow/validate-baseline.txt`, so the suite's status is captured rather than
obeyed and the verdict comes from this tool. The per-arm parser makes each printed
arm its own baseline scenario, and arms behind the ruled failure are recorded at
`ignore`. A port that changed the diff's line shapes or its status would silently
re-point what that leg means. Delta (10)'s comparison covers exactly that.

### (9) The behavioral test re-points, and it is this member's whole parity surface

`evidence-kit/gate-tests/diff-baseline-status.test.sh` drives the tool by literal
path — `BIN="$DIR/bin/diff-baseline.sh"` at `:17`, invoked at `:58`, `:88`, `:93`,
`:96` — and it is the member's entire behavioral coverage {design-bearing}. It is a
declared `evidence-kit-gate-test` enum member, so its own name is read by
`check-prose-enum` through the generated hook.

It is re-pointed at the front-end spelling in the deleting commit, and it **gains**
the three cases delta (5) adds: `--help` as the first argument (usage on stdout,
exit 0, from the front-end), a leading-dash positional in a suite slot (refusal,
stderr, exit 2), and `--` followed by a suite name that begins with a dash (taken as
free text). Probed rather than assumed: the existing test covers none of the three,
so the gap is untested as well as unimplemented, and adding the cases is what makes
the addition assertable rather than merely written.

### (10) Criterion 2's discharge is the both-substrates comparison, bought once, before the delete

This member ships no `good/`+`bad/` fixture pair and owes none — it is not a gate
{design-bearing}. In one session, with both implementations present and pointed at
the same baseline and skip file, run both over the same captured logs and compare:

- every printed line, byte for byte, over a corpus that produces **both** shapes —
  a `new-failure` from a baselined `pass` gone red, a `new-failure` from an observed
  `fail` with no baseline row at all, and a `recovery` from a baselined `fail`
  running green — plus the trailing `clean (<n> unpromoted recovery finding(s))` and
  `NEW failures` summary lines and their recovery counts;
- the **exit status** for each of: clean (0), any new failure (1), fewer than two
  arguments (2), an unreadable log (2), and an `exit-code` suite with no status (2);
- the **skip demotion**, by running a scenario that is observed `pass` and named in
  the skip file, asserting it does not clear a baselined `fail`;
- the **three new behaviours** of delta (5), which exist in one substrate only and
  are therefore asserted rather than compared.

The comparison is bought **before** the delete, in the session that deletes. A
comparison a session runs before a delete is evidence; an arm that can only skip
after it is not (gate-sdk/SPEC.md §The non-gate arm).

### (11) Every path-bearing surface moves in the deleting commit, and the settings file carries nothing

The roster is probed and split by what a gate catches {mechanical}.

**The gate-caught set is empty.** `check-docs-cmd`'s invoked-path assertion fires
only inside a triple-backtick fence, and every doc mention of this tool is
inline-backtick prose: `README.md:98`, `evidence-kit/README.md:15`,
`evidence-kit/SPEC.md`'s own section and its cross-references, and their `docs/`
mirrors. `check-tracking-claim` is silent for the same reason it is silent for the
sibling — none of the mentions carries its predicate grammar. So nothing static reds
on the delete.

**The two surfaces that red are executions**: the CI leg (delta 8) and the
behavioral test (delta 9). Neither is a pre-commit gate.

**The silent set, fixed by hand:** `README.md:98`; `evidence-kit/README.md:15`;
`evidence-kit/SPEC.md`'s prose cross-references at `:145`, `:186`, `:288`, `:302`,
`:1057`, `:1060`; `evidence-kit/lib/evidence.sh:138`, whose `# spec:` directive on
`ek_diff` binds this section by name; and the `TASK-QUEUE.md` bodies, which the
manifest set excludes. The frozen `docs/posts/*` release notes are **not** edited.

**`.claude/settings.json` names this path nowhere** — probed, explicit none. So this
cut triggers no permission-settings edit at all, does not meet the 2026-08-22 wall,
and needs no replacement grant: line 12's `Bash(bash gate-sdk/bin/run-gates.sh *)`
already reaches every bridged arm. Named because its sibling cut *does* carry a
grant removal and a reader would expect a matching delta here.

**`evidence-kit/SPEC.md §bin/diff-baseline.sh` itself** is rewritten as the arm's
section per deltas (2) through (7). The heading **stays**: `ek_diff`'s own directive
and two sibling sections cite it, and deleting the heading would dangle those where
deleting only the script dangles nothing.

### (12) The regeneration fan-out this cut stales

Deleting one owed `.sh` moves `scripts/measured-claims.sh`'s `tree-shell-owed` key
(`:42`), whose resolved value is baked inline into the generated
`scripts/git-hooks/pre-commit` (`:369`) {mechanical}. That hook, `commit-msg` and
`docs/check-graph.html` regenerate in the landing commit; the SPEC and README edits
stale their on-site mirrors and the crate change stales the binary. `check-graph`,
`check-docs-mirror-fresh` and `check-gate-binary-fresh` are the reds, all discharged
in the landing commit
(docs/site-architecture.md §Generated projections and their freshness gates).

**Two keys that do not move, probed rather than assumed:** `ported-gate-members` and
`gate-substrates` do **not** move — this cut registers no gate and adds no `.gate`
descriptor. **And no `measured:` marker binds `tree-shell-owed` in any tracked
`.md`** — probed; the live markers are `ported-gate-members=108` at
`docs/install.md:212` and `gate-substrates=native` at four sites — so
`check-measured-claim` stays green from the marker side and the staleness is the
baked hook alone.

## Producers and consumers

**New interface: the `--diff-baseline` arm.**
*Producer* — `gate-sdk/bin/run-gates.sh`'s new `case` arm, resolving the declared
roster through `gate_knob_env` and `exec`ing the binary (`run-gates.sh:114-128`). Its
enabling config is not test-only: this repo's `scripts/evidence-config.sh` sets the
baseline, skip and parser knobs the arm reads, and the CI job that calls it runs on
every push.
*Consumer* — `.github/workflows/gates.yml`'s install-smoke step, which reads the
arm's **exit status** as the leg's verdict, and a session invoking it by hand. The
mechanism is the process status, not a document.

**New behaviour: the argv-shape refusal, the `--` escape and the front-end help arm.**
*Producer* — the arm's own argument parser for the first two; `bin/run-gates.sh`'s
usage for the third.
*Consumers* — a session mistyping an invocation, and
`evidence-kit/gate-tests/diff-baseline-status.test.sh`, which gains one case per
behaviour (delta 9). Each behaviour has a named reader at a named transition: the
refusal is read by the caller's status check at invocation; the escape is read by the
group parser at the first positional; the help arm is read by the front-end before
dispatch.

**Retired interface: the shell `ek_parser_for` / `ek_parse` / `ek_diff`.**
*Reader after retirement* — the compiled twins in `native/src/evidence.rs`, called by
the two arms. The library's gate-tests are re-pointed rather than deleted (delta 4),
so the assertions keep a named subject.

**New state: none.** No new file, no new record format, no new field.

**Each reader's RED condition, not merely its subject** — binding because this cut
*narrows* a corpus (one `.sh` file, three library functions):

- `check-evidence-baseline` — reds on a configured suite with **no baseline row**, a
  **coverage floor** and therefore not monotone under a narrowing. Cleared by
  argument, not inspection: this cut changes no member of `EVIDENCE_KIT_SUITES`.
- `check-prose-enum` — reds when a declared enum's members do not match the surface,
  a **set equality** and therefore not monotone. The behavioral test keeps its name
  and its membership, so the set is unchanged; re-run rather than reasoned about.
- `check-docs-cmd` — reds on an unresolvable fenced invoked path; the count is zero
  for this file today, so the narrowing can only leave it zero.
- `check-comment-tier` — corpus reaches `*.rs`; three directives move into it, so it
  is re-run rather than inspected.
- `check-measured-claim` — reds only on a bound `measured:` marker whose oracle value
  disagrees with it; no marker binds `tree-shell-owed` (delta (12)'s probe), so it has
  no claim to check here and stays green. `check-graph` is what catches the moved
  value, via the baked hook.
- `check-evidence-lib-parity` / `--evidence-lib-parity` — its red condition is a
  **disagreement between two holders**, so removing one holder does not red it, it
  **empties** it. §The non-gate arm rules that shape: a parity arm's caller is its
  second holder and the arm retires with it. This cut therefore checks whether that
  arm's subject still has two holders after the shell functions go, and retires the
  arm in this commit if it does not — an arm that can only skip is the unreachable
  code the class refuses.

## Existing sections updated

- **`evidence-kit/SPEC.md §bin/diff-baseline.sh`** — rewritten as the arm's section:
  the spelling, the family choice, the three-state exit, the declared roster, the
  argv-shape additions, and the two contracts that cross verbatim
  (deltas 2, 3, 5, 6, 7). The heading stays.
- **`evidence-kit/SPEC.md §lib/evidence.sh`** — the three adapters' shell forms
  retire and the sentence naming them moves to the compiled twins; the library's
  remaining roster is restated (delta 4).
- **`evidence-kit/SPEC.md §Layout and configuration`** — the parser family's
  declaration and the skip file's producer limit are stated where the knobs are
  defined (deltas 3, 7).
- **`evidence-kit/SPEC.md §Baseline manifest`** — the fail-closed rule's reading is
  unchanged, but the sentence that a mistyped suite yields no findings at exit 0 is
  added as the ground for the shape refusal (delta 5).
- **`evidence-kit/README.md`** — the tool's mention re-spelled to the front-end form
  (delta 11).
- **`.github/workflows/gates.yml`** — the install-smoke verdict line re-spelled, and
  its surrounding comment left otherwise intact (delta 8).
- **`evidence-kit/gate-tests/diff-baseline-status.test.sh`** — re-pointed and
  extended by three cases (delta 9).
- **`evidence-kit/gate-tests/evidence-lib.test.sh` and
  `scripts/gate-tests/evidence-parser-values.test.sh`** — re-pointed at the compiled
  twins (delta 4).
- **`gate-sdk/SPEC.md §The non-gate arm`** — the class roster gains
  `--diff-baseline`, recorded as the argv-shape split's **fourth** reader instance
  and the first whose absorbed argument yields a clean verdict rather than a wrong
  one (deltas 2, 5).
- **`gate-sdk/SPEC.md §The bin/-tool contract`** — the reader-instance roster gains
  this member with its own distinguishing ground: the absorbed argument produces
  *no findings and exit 0*, read by a CI check mark rather than by a session
  (delta 5).
- **`gate-sdk/bin/run-gates.sh`** — the new `case` arm and its usage line (delta 2).
- **The generated projections** — `scripts/git-hooks/pre-commit`, `commit-msg`,
  `docs/check-graph.html`, the `docs/` SPEC and README mirrors, and the gate binary
  (delta 12).
<!-- update-target-exempt: a census note on another entry is build's filing obligation under delta (5), not a spec surface this amendment edits; the entry's own derivation is unreproducible as written and is routed to the gap inbox rather than corrected here -->
- **`TASK-QUEUE.md` `bin-tool-help-arm-absent-tree-wide`** — named as owed a census
  note at build, not edited here.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; each of the three new behaviours has
      a named reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its proper
      canonical-spec section, not appended.
- [ ] **Amendment deleted** — this file removed on merge; with the sibling cut also
      merged, **none remain for this component**, which is the batch that can
      satisfy the none-remain half.
- [ ] **Both-substrates comparison bought before the delete** — delta (10)'s
      procedure run in the deleting session.
- [ ] **The CI leg re-pointed and its assertion unchanged** — `gates.yml`'s verdict
      line moved to the front-end spelling with the baseline-diff semantics intact
      (delta 8).
- [ ] **Removals propagated** — every surface in delta (11) edited; the shell
      adapters removed from `lib/evidence.sh` and their gate-tests re-pointed; every
      spec grepped for names this change retired.
- [ ] **Gaps filed** — the skip-file truncation-membership gap (delta 7) and the
      unreproducible census derivation (delta 5) filed to the gap inbox; a
      build-time causal gap is resolved that session, not deferred.
