# SPEC amendment: parser-arms-cut

The port disposition of **`scripts/parse-gates-log.sh` (15 lines)** and
**`scripts/parse-installer-smoke-log.sh` (51 lines)** — the two owed files behind
`evidence-kit/SPEC.md §Layout and configuration`, this repo's two
`EVIDENCE_KIT_PARSER_<suite>` adapters — off the shell substrate as two bridged
`Arm::Emit` members of the gate binary. Cut A of
`parser-and-enum-adapter-cuts-with-graph-hotfix`, under the port-only run
(TRAJECTORY.md §PRIORITY DIRECTIVE); the unit set and this cut's host were ruled by the
**lead on 2026-09-03 on its own authority**.

**Measured at this HEAD rather than carried from the scope survey**: the port oracle's
`--tree` arm reads 107 files scanned, 64 declared `no-port`, 0 temporarily held,
**43 owed**, with `scripts/parse-gates-log.sh owed lines=15` and
`scripts/parse-installer-smoke-log.sh owed lines=51` among them.

**The cut takes the whole of its section's owed set, so the 2026-09-03 outer-bound ruling
is not exercised.** `§Layout and configuration` also specifies `evidence-config.sh` and
the template it comes from, and both are **permanently shell** with the `# no-port:` cause
of §The config-seam port disposition already on them — they are not owed and are not
subtracted by this cut. What is owed behind that section is exactly these two files.

**Why these two are owed at all**, since the instinct is that a knob's value is the
consumer's business: `native-gate-port-remaining-corpus`' 2026-09-03 ruling (4) names both
of them by name — *a file this repo names as a knob's **value** … is what the seam
resolves, not the seam: no adopter edits it, and its whole documented purpose is to run*.
The same ruling refuses in as many words the two readings a session would reach for
instead, ruling (1) as an exemption for consumer-side plugins and a `# no-port:` citing
the seam. Neither is available and neither is re-argued here.

**Neither member is a gate**: each maps a captured log to `<scenario> <status>` lines and
returns a status, so no `gates.list` row, no `.gate` descriptor and no `good/`+`bad/`
fixture pair.

## What changes

### (1) The mechanism moves into the binary and the knob's **value** re-points; the seam does not narrow

`EVIDENCE_KIT_PARSER_<suite>` keeps its contract exactly — *a parser adapter name or a
consumer command mapping a captured log to `<scenario> <pass|fail|ignore>` lines* — and
this repo's two values move to `bash gate-sdk/bin/run-gates.sh --emit parse-gates-log`
and `bash gate-sdk/bin/run-gates.sh --emit parse-smoke-log installer/consumer-smoke/run-smoke.sh`
{design-bearing}. `ek_parse`'s dispatch is **not edited**: its consumer-command branch
already runs any command on the log, so the re-point is the whole of the change on the kit
side, and the extension point an adopter may point anywhere is untouched.

### (2) The **named-adapter** route is refused, on two independent grounds, and the host entry's question stays open

The obvious alternative is to make the gates parser a third built-in adapter beside
`exit-code` and `libtest` — `EVIDENCE_KIT_PARSER_gates=run-gates`, the awk landing in
`ek_parse` — and this cut's own host entry names that shape as a candidate answer
{design-bearing}. It is refused twice over.

**First: it moves nothing into the binary.** `evidence-kit/lib/evidence.sh` is
**permanently shell** under §The kit-library port disposition, and its header says so.
Absorbing an owed script into a `# no-port:` library discharges the `--tree` **count**
while leaving the mechanism on the substrate the port exists to leave. The count is the
port's predicate; the binary is its objective, and a move that satisfies the first by
defeating the second is the one shape a completion predicate cannot catch.

**Second: naming the convention is a yield, and the run forbids one.** *Whether the
front-end shape should be a named convention rather than each consumer's invention* is
`kit-knob-consumer-adapter-convention`'s own open deliverable, and answering it inside a
port cut is exactly the yield TRAJECTORY.md's port-only run refuses with a single
exception this cut is not. So the entry **demotes** rather than closing.

**What the cut does deliver against that entry is worth stating precisely, because it is
not nothing.** After it, an adopter wanting per-gate scenarios writes a knob value naming
a payload arm instead of authoring a script — the *mechanism* stops being each consumer's
invention even though the *name* does not yet exist. The entry's question narrows to the
naming, and its own text already frames it that way.

### (3) `--emit-parse-gates-log` is an `Arm::Emit` member with an **empty** declared roster

The member reads the log path off its own argv and resolves no kit knob {design-bearing},
so its bridged-arm row declares nothing. That is the `--emit-md-section` kind of empty
roster — *happens to read nothing* — and not the `--emit-session-id` kind, which **may
not** declare anything because its names are defined in no kit library; the pair is named
in §The non-gate arm and a reader should be able to tell which this is. Table membership
is still what makes the arm reachable at all, since `bin/run-gates.sh --emit <name>`
composes the flag and `exec_arm` finds a member only in that table.

Its grammar is unchanged and is **gate-sdk's**, not evidence-kit's: the two-space-indented
`PASS:` / `FAIL:` tail lines and the gate name on each, and nothing else.
§check-graph and §run-gates both already pin that grammar against this reader by name, and
the cut mints no fourth tail.

### (4) `--emit-parse-smoke-log` takes the driver path as **argv**, and that is where the provenance seam is held

The smoke driver `installer/consumer-smoke/run-smoke.sh` is this product's file, not a
kit's, so its path may not become a crate literal {design-bearing}. It arrives as the
arm's first positional and the crate holds no default for it: an invocation without it is
exit 2 naming the missing operand, the same fail-closed shape the shell form takes when
the driver is absent. The arm's declared roster is empty for the same reason delta (3)'s
is — everything it reads arrives on argv.

**What ships as kit mechanism is the derivation, and it is genuinely generic**: an arm
header is a **top-level** `printf '<literal>\n'` with no redirect, and the arm's name is
the literal up to its parenthetical. That rule is about how a shell driver announces
itself, not about what this driver announces, and it is already stated as contract at
§Layout and configuration and in the driver's own text.

### (5) The one consumer literal is replaced by a **derivation**, because it can be neither argv nor a crate constant

The shell form carries `INSTALLER-SMOKE: clean` twice — once as the completion marker it
scans the log for, once as the `INSTALLER-SMOKE*` prefix it skips when deriving the arm
roster {design-bearing}. Neither spelling survives the port, and the reason it cannot
simply move to argv is delta (6)'s: the literal contains spaces.

**The replacement, and it is exact rather than approximate: a driver's *last* top-level
header is its completion announcement, and the ones before it are its arms.** Probed
against this driver rather than assumed — its last top-level `printf` is the clean line,
and the name that derivation yields from it is exactly `INSTALLER-SMOKE: clean`, which is
what the `INSTALLER-SMOKE*` skip was a loose spelling of all along. So the derived roster
is unchanged, the completion test is unchanged, and the two consumer literals collapse
into a positional rule about the driver's own text.

**The hazard the new rule creates is named because it has no oracle**: a header printed
*after* the completion line would silently become the marker and demote the real one to an
arm. The driver's text says where its headers are a parsed contract; this clause is added
there, beside the clean line, rather than only in the kit's spec.

**Two generic skips are preserved verbatim** and are not consumer content: a header whose
literal is empty, and one that is entirely a format specifier — a `printf '%s\n'` names no
stable scenario. The zero-headers fail-closed arm is preserved and **tightened by one**:
fewer than two headers cannot yield an arm and a marker, so it is the same refusal reached
one case earlier, with `run-validate`'s produced-no-result guard behind it exactly as
today.

### (6) The argv contract is forced by word-splitting, and the log arrives **last**

`ek_parse`'s consumer branch is `$parser "$log"` — deliberately unquoted, with the SC2086
disable saying so in the library {design-bearing}. Two consequences bind the arm's
interface and neither is a preference. **No argument may contain a space**, which is what
puts delta (5)'s marker out of reach of argv and forces the derivation. And **the log path
is appended after everything the knob value spells**, so the arm's positionals are
`[<driver>] <log>` in that order, with the log last. §lib/evidence.sh's *a consumer parser
command receives the log path alone — no suite name, no exit status* is unchanged and is
what makes the log the only trailing operand.

### (7) The attribution's honest limit travels unchanged, and closing it inside a port is refused

§Layout and configuration records that per-arm attribution leans on the smoke being
fail-fast: an arm is judged failed when the log reaches its header and neither a later
header nor the run's own clean line follows, so a smoke that gained a *non-fatal* failure
path would read that arm as passing {design-bearing}. The port carries that limit
verbatim. Closing a live honest limit inside a port is the move this track has refused
repeatedly — the worktree predicate, §stage-rules' empty output on an unknown stage,
§bin/env-probe's `uncomparable` arm — and the suite's verdict is right either way; only
the blame would be wrong.

### (8) The bespoke test is **re-pointed at the configured knob value**, and gains the second suite

`scripts/gate-tests/parse-gates-log.test.sh` drives a file this cut deletes, so it moves
either way; what it moves *to* is a decision {design-bearing}. It is re-pointed at the
value `ek_parser_for gates` resolves out of `scripts/evidence-config.sh` — not at a
hardcoded arm invocation — and a second case does the same for `installer_smoke`.

**The ground is this cut's own host entry.** `kit-knob-consumer-adapter-convention` was
filed because a knob's configured value pointed at a path a port had deleted, and the
degradation was silent for 77 firings; its repair was proved by a negative control
(`scripts/gate-tests/subagent-stop-reader.test.sh`) that fails against the dead default.
A test that hardcodes the arm asserts the Rust works; a test that runs **the value the
consumer actually configured** asserts the seam works, and the seam is where the attested
failure was. The `installer_smoke` half additionally covers the new failure mode this cut
creates — a knob value whose leading positional is missing or wrong.

This is the one place the cut **adds** assertion rather than relocating it, and it is
named as such so the addition is visible rather than smuggled. The grammar-level
assertions — per-gate lines off a verbose log, both `FAIL` tails, a no-tail log yielding
no output, a missing log failing closed — move into the crate's own `#[cfg(test)]` module
beside the arms, which is where the tree already keeps a compiled member's unit coverage.

### (9) Exactly one permission grant is deleted and none is added

`.claude/settings.json` carries `Bash(bash scripts/parse-gates-log.sh *)` and it is the
only line in the file naming either parser {mechanical}. It is removed **in the same
commit as the delete**, the window the 2026-08-29 settings-grant carve-out exists to
close, and the count that carve-out demands be probed rather than assumed is **one**. No
grant is added: `Bash(bash gate-sdk/bin/run-gates.sh)` and its `*` form are already
committed, which is what keeps this inside the carve-out rather than against the
2026-08-22 bar.

### (10) The prose citations re-point, and **no gate forces them**

Governed prose names these files in five places and each re-points {mechanical}:
`evidence-kit/SPEC.md §Layout and configuration`'s `EVIDENCE_KIT_PARSER_<suite>` bullet
(twice, once per adapter), `gate-sdk/SPEC.md §run-gates`' signal-exit-code paragraph,
`gate-sdk/SPEC.md §check-graph`'s widening-is-monotone paragraph, and the `// spec:`
comment in `native/src/proc.rs` that names this reader for the same tail grammar. The
`docs/` mirrors follow as a regenerated projection.

**The hazard is that a stale citation ships green.** `check-docs-cmd` assertion A resolves
invoked `.sh` paths **inside a fence only** — its inline-backtick arm scans for *knobs*,
not paths — and every one of these citations is inline backticks. `prose-filename-citation-liveness`
owns the general gap and is not re-filed. The consequence is a Definition-of-Done item
rather than a gate, and the enumeration above is what makes the sweep a check-list.
Assertion B is cleared **by naming the in-corpus holder**: the `EVIDENCE_KIT_PARSER_`
family stem is extended by `evidence-kit/lib/evidence.sh`'s own
`local var="EVIDENCE_KIT_PARSER_$1"`, which is inside that assertion's kit-roots corpus —
`scripts/evidence-config.sh` is **not**, because `scripts/` is no kit root, so the config
file's own mentions would not have satisfied it.

### (11) The validate baseline is the port's oracle, and a faithful port leaves it **unedited**

`.workflow/validate-baseline.txt` holds 75 `gates` rows and 13 `installer_smoke` rows, each
keyed by a scenario name these two parsers produce {mechanical}. So the port's success
condition is that the file needs **no edit** — a changed scenario name, a changed
hyphenation, a dropped or added row all red through `run-validate` and
§check-evidence-baseline's directional rule. No golden is minted for the purpose, because
the baseline already is one and a second copy would be the third holder evidence-kit's own
*no committed golden* rule refuses.

**No parity arm is minted either, and the reason is a ruling rather than an economy.**
§The non-gate arm rules that a parity arm's caller is the second holder, so the arm retires
with it — `--declaration-parity` left that roster in the cut that deleted the shell form it
compared against. Both shell holders here are deleted by this cut, so a parity arm could
only ever skip. What stands in its place is a **one-time** comparison in the deleting
session, before the delete.

### (12) The owed count moves **43 → 41** on this cut alone, and the host entry demotes

Both files leave the owed column by **deletion**, not by declaration {mechanical}: no
`# no-port:` and no `# port-until:` is written anywhere by this cut, so nothing is
subtracted from the 2026-08-28 completion predicate. The sibling enum-sets cut takes one
more file off the same total independently; neither amendment may claim the other's, and
the iteration's arithmetic is 43 → 40 only when both land.
`kit-knob-consumer-adapter-convention` **demotes** back to the deferred section under
`[design-pending]`, returning to the position this promotion took it from, because its
deliverable — the convention — is delta (2)'s explicit non-deliverable. It is at **49 of a
50-line cap** after this promotion, so the demotion's own note is priced against that cap
and compresses in the same commit rather than after it.

## Producers and consumers

The cut introduces **no new state, event or field**. It introduces two new **interfaces**,
the two arms, and relocates two existing ones — programs that map a log to
`<scenario> <status>` lines — from consumer scripts into the binary. The emitted grammar,
the scenario names, both derivations and every exit status are unchanged, so the checklist
runs over the relocation and over the deletion.

- **Producers.** `--emit-parse-gates-log` and `--emit-parse-smoke-log`, each dispatched in
  `main` ahead of the registry lookup and absent from `--list`, each reached through
  `bin/run-gates.sh --emit <name>`. Neither has an enabling configuration to set: both
  declare an empty knob roster and read everything from argv, so point 1 of the
  causal-completeness check is satisfied without a knob for a consumer to forget.
- **Consumer, and it is one function for both.** `ek_parse` in
  `evidence-kit/lib/evidence.sh`, on its consumer-command branch, reached from **two**
  spine callers — `bin/run-validate.sh` when it captures a suite's log, and
  `bin/diff-baseline.sh` when it re-reads one. Both inherit the re-point with no edit of
  their own, which is the property §lib/evidence.sh already claims for the dispatch living
  inside `ek_parse`.
- **The scenario lines' reader is the baseline comparison**, at the transition where
  `run-validate` diffs a run's scenarios against `.workflow/validate-baseline.txt`. That is
  the transition delta (11) makes the port's oracle.
- **The driver path has exactly one reader**, `--emit-parse-smoke-log`'s roster derivation,
  and it is read once per invocation before the log is opened. A driver that does not
  resolve is exit 2 before any line of the log is judged.
- **No new field.** Both fields of each emitted line keep the readers they have.

**Every reader's RED condition, because this delta narrows a corpus.**
canon-kit/SPEC.md §The causal-completeness check point 5 binds — a reader is clearable by
inspection only where its verdict is monotone in the violation set, and reds-on-finding-none,
exact-count and coverage-floor shapes are not.

- **`check-evidence-baseline` is the non-monotone reader that matters.** §Baseline manifest's
  directional rule reds a baselined `pass` scenario that is red **or absent**, so a
  *narrower* scenario set adds violations rather than removing them — the shape point 5
  names by construction. It cannot be cleared by inspection and delta (11) does not try:
  the port is run and the baseline is proved to need no edit.
- **`check-evidence-manifest`** — reds when the manifest's scenario set and the configured
  globs disagree, and reads the lifecycle surfaces for its close-entry and stamp-coupling
  assertions. `EVIDENCE_KIT_SCENARIO_GLOBS` is not configured for either suite here, so
  the set-equality arm is disarmed for both; stated rather than assumed, because arming it
  would have made this cut's scenario-name question a second gate's business.
- **`check-battery-roster`** — holds `EVIDENCE_KIT_RUNNER_DOC`'s battery-roster block
  against the **suite** roster. Untouched: no suite is added or removed, and only two
  suites' parser *values* change.
- **`check-docs-cmd` assertion A** — reds on a **fenced** invoked `.sh` path that does not
  resolve; monotone, and **cleared vacuously** because none of these citations is fenced.
  That vacuity is delta (10)'s hazard, not a discharge.
- **`check-docs-cmd` assertion B** — a **zero-count** reader per kit-prefixed knob name,
  one of the three non-monotone shapes by name. Cleared by inspection only because the
  in-corpus holder is named: `evidence-kit/lib/evidence.sh:88`.
- **`check-settings-paths`** — reds on an allow entry naming a path that no longer
  resolves; it reds *because* of the cut, which is what forces delta (9) rather than
  leaving it to authorial memory.
- **`check-gate-fixture-coverage`** — reds on a registered gate with no `good/`+`bad/`
  pair. Untouched: neither arm is a gate, which is what §The non-gate arm's second property
  buys.
- **`check-gate-substrate-parity` assertion B** — an **equality** between the `.gate`
  descriptor set and `--list`'s roster, which a non-gate arm stays outside by construction.
- **The `scripts` fixture suite** — `gate_fixture_suites` rosters `scripts/gate-tests`
  because the gates dir joins the kit roots in that walk, so delta (8)'s test runs in it
  and a test left driving a deleted path fails there rather than nowhere. `scripts/` is
  **not** a kit root, so this file is outside `check-prose-enum`'s `<kit>-gate-test`
  families and its rename or re-point moves no enum set.
- **`check-shellcheck`, `check-comment-tier`, `check-exec-bit`** — monotone in the removing
  direction; deleting two files removes findings, and the crate modules' `// spec:` headers
  carry the deleted scripts' comment bindings rather than dropping them.
- **The port oracle `--emit port-blockers`** — reports rather than reds; `--tree` owed
  moves 43 → 41 on this cut alone.
- The build re-runs the full battery rather than resting on this enumeration, which is the
  enumeration's purpose: it says where to look when one goes red.

## Existing sections updated

- `evidence-kit/SPEC.md §Layout and configuration` — the `EVIDENCE_KIT_PARSER_<suite>`
  bullet names the two arms rather than the two scripts; the `run-gates` tail grammar and
  the per-arm derivation are restated for the compiled holders; the completion-marker rule
  replaces the consumer literal, with the hazard that a header printed after it would take
  its place; and the fail-fast attribution's honest limit is carried verbatim. The knob's
  own contract is unchanged, and saying so is what stops a reader taking the re-point for a
  narrowing (deltas 1, 3, 4, 5 and 7).
- `evidence-kit/SPEC.md §lib/evidence.sh` — `ek_parse`'s consumer-command branch is
  unchanged and gains the two properties this cut depends on and no other surface states:
  the value word-splits so no argument may carry a space, and the log path is appended
  last. The *permanently shell* declaration and its ground are untouched, and the
  named-adapter refusal is recorded here because this is the file that would have grown the
  third adapter (deltas 2 and 6).
- `gate-sdk/SPEC.md §The non-gate arm` — the `--emit-` family roster gains
  `--emit-parse-gates-log` and `--emit-parse-smoke-log` as 2026-09-03 members; the
  empty-roster paragraph gains two more members of the *happens to read nothing* kind, with
  the contrast against `--emit-session-id`'s *may not declare anything* kept explicit; and
  the parity-arm paragraph gains a second instance of an arm **not** minted because both
  holders are deleted (deltas 3, 4 and 11).
- `gate-sdk/SPEC.md §run-gates` — the signal-exit-code paragraph names the arm rather than
  the script; the claim that the tail grammar keeps one shape is unchanged and remains
  true, which is the point of naming it (delta 10).
- `gate-sdk/SPEC.md §check-graph` — the widening-is-monotone paragraph names the arm
  rather than the script; nothing about check-graph changes (delta 10).
- `native/src/proc.rs` — the `// spec:` comment naming this reader for the tail grammar
  re-points to the arm (delta 10).
- `scripts/evidence-config.sh` — two knob values re-pointed, in the commit that deletes
  their targets (delta 1).
- `scripts/gate-tests/parse-gates-log.test.sh` — re-pointed at the configured knob value
  and extended to a second case for `installer_smoke`; the grammar assertions move into the
  crate's own test module (delta 8).
- `installer/consumer-smoke/run-smoke.sh` — the text that already says its printed headers
  are a parsed contract gains the completion-marker clause, beside the clean line it
  governs (delta 5).
- `.claude/settings.json` — one allow entry deleted, none added, in the commit that deletes
  its target (delta 9).
- `TASK-QUEUE.md`, the `kit-knob-consumer-adapter-convention` entry — gains this
  amendment's `[spec:]` ref, the lead's 2026-09-03 own-authority host ruling, and a
  re-wrapped lead-line description; it **demotes** at build rather than reaching `## Done`,
  at 49 of a 50-line cap, so the demotion compresses in the same commit (deltas 2 and 12).

<!-- update-target-exempt: the composer entry takes no body write from a cut by its own 2026-08-28 ruling, and this cut hosts elsewhere; its lead line already carries the sibling cut's ref -->
- `TASK-QUEUE.md`, `native-gate-port-remaining-corpus`'s body — deliberately unwritten.

<!-- update-target-exempt: the config seam's two files are permanently shell with their cause already declared, and this cut neither ports them nor changes what they are; naming them here is what stops a reader taking two adapters' port for the seam's -->
- `gate-sdk/SPEC.md §The config-seam port disposition` — deliberately untouched.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls evidence-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
- [ ] **Each shell holder and its arm agree line-for-line before the delete**, on a
      captured log of the suite it parses, in the session that deletes it — after the
      delete one holder cannot be compared with itself, and delta (11) is why no parity arm
      stands in for this.
- [ ] **`.workflow/validate-baseline.txt` is UNEDITED** at the end of the cut, and that is
      read off `git status` rather than inferred from a green run.
- [ ] **The derived arm roster is re-counted against the driver** — thirteen arms and one
      completion marker — and the marker proved to be the driver's last top-level header
      rather than assumed to be.
- [ ] **The tightened fail-closed arm exercised**: a driver yielding fewer than two headers
      is exit 2, and `run-validate`'s produced-no-result guard reports it.
- [ ] **The re-pointed test runs the CONFIGURED value**, not a hardcoded arm invocation,
      and is proved to fail against a deliberately broken knob value — the negative control
      the host entry's own repair was proved by.
- [ ] **The grant count re-probed at the deleting commit** and the one allow entry removed
      in it, per the 2026-08-29 carve-out's own terms.
- [ ] **The five prose citations swept by the list in delta (10)**, because no gate forces
      any of them and a missed one ships green.
- [ ] **The `--tree` owed count re-read to confirm 43 → 41 on this cut**, with no
      `# no-port:` and no `# port-until:` written anywhere by it.
