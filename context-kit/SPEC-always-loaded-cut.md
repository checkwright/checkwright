# SPEC amendment: always-loaded-cut

The port disposition of **`context-kit/bin/always-loaded.sh` (85 lines), the one owed file
declaring context-kit/SPEC.md §The always-loaded meter**: it ports to a bridged
`--emit-always-loaded` `Arm::Emit`, its three modes arriving as operands, and the hook-body
command it measures **stays spawned** because that knob is a consumer command seam. A
stated-contract cut under the port-first run (TRAJECTORY.md §PRIORITY DIRECTIVE), hosted on its
own per-cut feature entry and packaged by the lead as one of this iteration's two.

**The composer's precondition was run at this stage rather than inherited.**
`bash gate-sdk/bin/run-gates.sh --emit port-blockers --group` trails *0 group(s) formed … 0
takeable at this cut* — no takeable group, which is the budget arm's stated precondition
(gate-sdk/SPEC.md §The first cohort). The selection ground is the **owed column** of
`--emit port-blockers --tree` — *86 file(s) scanned, 67 declared no-port, 0 temporarily held,
19 owed* — where this file reads `owed lines=85`.

**This cut does not discharge context-kit's owed column**, and the section it selects already
says which members stay: `bin/run-index-tests.sh` (95) and `index-tests/toolfloor-cases.sh` (49)
are sequenced behind the installer's behind-invoke relocation by §Testing, and `lib/toolfloor.sh`
(58) behind the same relocation. Those three keep their `owed` reading and take no
`# port-until:`, exactly as §Testing's sequencing prose requires — a held file would leave the
owed column and the completion predicate admits no contributor-side subtraction. What this cut
removes from that column is the one member §Testing calls "correctly homed, behind nothing, and
takeable as a singleton whenever a cut wants it".

## What changes

### (1) The cut is §The always-loaded meter's one owed file, and it leaves three behind

`bin/always-loaded.sh` reads `owed lines=85` and is the only owed file whose `# spec:` pointer
binds context-kit/SPEC.md `## The always-loaded meter` {mechanical}. Its reach and its section
bound coincide, so no second section is rewritten by construction — with the two exceptions
every cut moves, gate-sdk/SPEC.md §The port disposition's owed-corpus prose and the roster
sentence in §The non-gate arm that delta 8 adds it to, plus the drift-kit consumer delta 5
names.

### (2) The arm is `--emit-always-loaded`, an `Arm::Emit`, and both halves are forced

The member ports as a **bridged-arm table** row —
`("--emit-always-loaded", Arm::Emit(always_loaded::emit), always_loaded::KNOBS)` in
`native/src/emit/mod.rs`'s `BRIDGED_ARMS`, with the body in
`native/src/emit/always_loaded.rs` {design-bearing}.

**Table membership is forced by the forced-family test, not chosen.** The tool reads four
consumer knobs — `CONTEXT_KIT_SURFACES`, `CONTEXT_KIT_HOOK_CMD`, `CONTEXT_KIT_BASELINE_FILE` and
`CONTEXT_KIT_GROWTH_PATHS` — and a hardcoded top-level flag receives none of them, which
§The non-gate arm calls the difference between working and appearing to. The declared roster is
those four and nothing else: `GATE_SDK_WORKFLOW_DIR` and `GATE_SDK_GATES_DIR` are **not**
declared, because `lib/context.sh` already rides them into `CONTEXT_KIT_BASELINE_FILE`'s and
`CONTEXT_KIT_HOOK_CMD`'s own resolved values, so declaring either would resolve one fact twice.

**The `--emit-` spelling is forced by the exit grammar, read in the direction
`--emit-stage-rules` establishes.** The member's whole failure grammar is already
`Arm::Emit`'s collapse: every mode returns 0, and the one non-zero path — `cannot enter repo
root` at `:7` — is exit 2. It declares no 1 and never has, so the collapse to {0, 2} discards
nothing it carried. `--emit-usage-trend` is the member admitted on exactly that ground, and the
prefix additionally carries the reachability: `bin/run-gates.sh --emit <name>` composes
`--emit-<name>`, so a differently-spelled arm would be reachable by no shipped front-end.

**The three modes are operands, never composed into the flag.** `--emit always-loaded`,
`--emit always-loaded --growth` and `--emit always-loaded --update-baseline` — the shape `--hook`
and `--wait-probe` already carry for their own subcommand words, so the arity question is a
solved registration rather than a new one: `--knobs` forwards the arm's own argv to the bridge.
Composing three flag spellings is refused for the reason that shape exists — three spellings
would be three rows publishing one knob roster, and the front-end's grammar would carry a second
copy of a decision the table already holds.

### (3) `CONTEXT_KIT_HOOK_CMD`'s default moves into `lib/context.sh` in this cut, never after

Today the knob's default is computed **inside the ported script**, at `:17-26`: a two-candidate
resolution over `${GATE_SDK_GATES_DIR:-scripts}/run-gates.sh` and the sibling kit's
`bin/run-gates.sh`, consumer-first, composing
`bash <front-end> --emit queue-index --collapse-deferred`, and leaving the knob **empty** when
neither candidate resolves {design-bearing}. `lib/context.sh` defines every other
`CONTEXT_KIT_*` default and does not define this one.

**That is not tidying, it is the bridge's refusal.** The bridge resolves a declared knob by
sourcing exactly one kit's library and reads it through `declare -p`; a default left beside the
compiled reader is sourced by nothing and resolves empty, which the reader takes as an unset
knob rather than as an error — the failure is silent (§The non-gate arm, *a default the deleted
shell driver held inline moves into the owning kit's library in the same cut*). The declaration
and the default are therefore one change.

**All three properties move with it and none is re-derived.** The candidate order stays
consumer-first; the `-f` existence test stays the resolution predicate; and the empty-on-
unresolvable behaviour stays, because it is what the meter's `hook=0` branch at `:36` reads and
what a consumer vendoring context-kit without a battery front-end depends on. `[[ -z
"${CONTEXT_KIT_HOOK_CMD+x}" ]]` — set-but-empty is a deliberate override and is preserved as
such — is the exact test that moves, not `[[ -v ]]` and not `:-`.

### (4) The hook body stays spawned, and calling `queue-index` in-process is refused

The meter measures the steady-state hook body by running whatever command
`CONTEXT_KIT_HOOK_CMD` names, through `bash -c`, and taking its line count {design-bearing}.
The compiled arm keeps that spawn.

**The ground is that the knob is a command seam, not an implementation detail.** The default
happens to name this project's own queue-index arm, and the natural port reads that as an
invitation to call `queue_index::emit` in-process and skip a process. It is not: the knob's
whole contract is that a consumer names *their* hook body, and §The non-gate arm keeps a
launcher external for exactly this reason wherever the knob is a command seam —
`DELEGATION_KIT_REFRESH_CMD` under `--usage-verdict`, and `curl` under `--usage-poll` on the
same reasoning. An in-process shortcut would measure the wrong thing for every consumer whose
hook body is not queue-kit's, silently and at exit 0.

**The witness already exists and is committed.** `context-kit/index-tests/` drives this member
with `CONTEXT_KIT_HOOK_CMD="cat $CORPUS/hook-sample.txt"` against the golden
`expected/always-loaded.txt`. That case is a consumer command that is not an arm at all, so a
port that special-cased the default would keep the golden green and break the seam the golden
exists to prove — which is why the spawn is stated as a ruling here rather than left to the
implementation.

**The spawned-program set is therefore `bash`, plus `git` for the `--growth` arm's
`git diff --numstat` and the `--update-baseline` arm's `rev-parse`, plus whatever program the
consumer's hook command names.** It joins the prose paragraph in §The non-gate arm that records
those sets, beside `--emit-env-probe` as the second member whose set a consumer can change,
since `--needs` answers about registry members only and a bridged arm is not one.

### (5) The measurement becomes a library function, and `kpi-always-loaded` reads its figures

`native/src/emit/kpi/always_loaded.rs:46-52` spawns `bash bin/always-loaded.sh` and parses the
rendered line back apart — `leading_total` reading `^[0-9]+l` and `since_delta` reading
`[+-][0-9]+ since`, which that module's own test comment calls "an undeclared cross-kit output
contract" {design-bearing}. The port ends that.

**The shape is `--emit-footprint`'s, already ruled in this kit.** §bin/footprint states that the
emission "is a **library function** the arm wraps rather than the arm itself, which is what lets
§check-footprint-fresh call it in-process and the value rollup consume its per-kit figures as
data rather than re-parsing the rendered page". The same split lands here: a function returning
the measurement — surface total, hook total, and the baseline row's total and commit where one
resolves — with the arm rendering the three modes' text over it and the KPI consuming the
figures. `leading_total`, `since_delta` and their `#[cfg(test)]` test are deleted with the
contract they existed to parse.

**The presence witness moves to the library, and that is the half a port gets wrong.** The KPI
answers `context-kit absent` today by failing to find `bin/always-loaded.sh` under the kit
roots. After the cut that file is gone from every tree, so the witness must move or the row
reports *absent* everywhere. It moves to `lib/context.sh` — the exact shape
`kpi/prompt_friction.rs` took when `scan-prompts` ported, whose comment states the rule: "the
presence witness is the library, never the surface the measurement reads". `lib/context.sh`
carries `# no-port:` on the sole-resolver ground, so no later cut can delete it out from under
the row.

**`--emit-drift-report`'s declared roster gains what the KPI now reads in-process, and the
natural reading misses this.** While the KPI spawned the meter, the child sourced
`lib/context.sh` and resolved its own knobs; an in-process read resolves nothing the report's
own row was not handed. `drift_report::KNOBS` already carries `GUARD_KIT_LOG`,
`GUARD_KIT_SETTINGS` and `GUARD_KIT_SETTINGS_LOCAL` for precisely this reason on the
prompt-friction row, and it gains `CONTEXT_KIT_SURFACES`, `CONTEXT_KIT_HOOK_CMD` and
`CONTEXT_KIT_BASELINE_FILE` on this one. `CONTEXT_KIT_GROWTH_PATHS` is **not** added: the KPI
reads the bare mode only, and a knob no reader on that path reads would be a declaration with no
named reader.

### (6) An unrecognized mode operand is a refusal, and it is the cut's one behavior change

The shell form matches its two modes with two independent `[[ ]]` tests at `:10-11` and has no
else-branch, so `always-loaded.sh --growht` — or any other misspelling, including `--help` —
falls through into the **bare** reading and exits 0 {design-bearing}. The ported arm refuses
instead: an operand that is not `--growth` or `--update-baseline` prints the usage block on
stderr and exits 2, the shape `scan_prompts.rs`'s `USAGE` constant already carries.

**This is a behavior change and is named as one rather than folded into the port.** It is the
only one this cut takes; everything else is held byte-identical by delta 7's golden.

**Its ground is the harm, not the argument-shape rule.** §The bin/-tool contract binds on a tool
whose positionals are **free text**, and this member's are a closed option set — validating
membership already validates shape — so that rule's trigger does not fire and this delta does
not claim it as a sixth instance. What the silent fallthrough costs is the harm that clause's
own reader instances are enumerated for, reached from a mode selector rather than a positional:
`--update-baseline` is a **close-stage act**, and a typo of it prints an ordinary-looking meter
line at exit 0 while writing no baseline at all. The session then commits a close whose baseline
was never rewritten, and the next iteration's brevity pass reacts to a delta measured from the
wrong commit. A refusal is what makes that state unreachable, and there is no third disposition
available to an operand dispatch.

**Its red condition is named because no committed case covers it today.** The index-test golden
drives the bare mode alone; the new case asserts the refusal's exit 2 and its usage-on-stderr,
and is owed in the same commit as the arm.

### (7) The named callers, re-pointed in the same commit, and the golden is the parity oracle

Five callers read this tool and every one moves to `bash gate-sdk/bin/run-gates.sh --emit
always-loaded […]` — verified this session over the tracked tree rather than assumed
{mechanical}:

- `context-kit/bin/run-index-tests.sh:79` — the golden case, already reaching every other tool
  through `$RUN_GATES`; its `env CONTEXT_KIT_CONFIG_FILE=$cfg` prefix survives untouched, that
  knob being a config-file selector the bridge reads *before* it sources the library rather than
  an argument arriving after resolution (§The non-gate arm's `LIFECYCLE_KIT_CONFIG_FILE` near
  miss). **`expected/always-loaded.txt` is held byte-for-byte and is this cut's parity oracle** —
  §Testing calls those goldens "unusually strong" because they were produced by the shell
  implementations the arms replaced, and this cut is cleared by that line matching and by
  nothing else.
- `context-kit/smoke/install.sh:61` — the `--update-baseline` assertion in the scratch consumer.
  The file itself stays permanently shell on §Consumer smoke's class ruling and is untouched
  beyond the one command.
- `native/src/emit/agents_md_smoke.rs:376` — the spawned
  `cd "$1" && exec bash context-kit/bin/always-loaded.sh` becomes the front-end form its sibling
  assertion three lines below already uses for the footprint emitter.
- `context-kit/templates/close-brevity.md:7` and `:29` — the brevity pass's two steps.
- `context-kit/README.md:57`, `:77`, `:78` — the quick-start baseline step and the two
  cheat-sheet lines.

**The two `.claude/settings.json` grants drop in the deleting commit**, in-cut and with no
out-of-band step, under `native-gate-port-remaining-corpus`' ruling (2) as widened 2026-09-05.
Both `Bash(bash context-kit/bin/always-loaded.sh)` and its `*`-suffixed twin are in
`check-settings-paths`' scope — that gate skips a `*` in the **command token** and this pattern
sits in the argument — so leaving either standing is a red rather than merely dead weight.

### (8) §The non-gate arm's roster gains the member, undated

The class roster gains `--emit-always-loaded` with its owning section named and **no
landing-date cohort label** {mechanical}, for the reason the sibling cuts state: those labels are
provenance, which `kit-spec-provenance-seam-sweep-remainder` retires, so a dated label would be
landed and removed inside one iteration.

## Producers and consumers

**New interface: the `--emit-always-loaded` bridged arm.**
*Producer* — `native/src/emit/mod.rs`'s `BRIDGED_ARMS` row, dispatched in `native/src/main.rs`
before the registry lookup; its enabling configuration is the bridged environment
`gate_knob_env` builds for it, which `bin/run-gates.sh` resolves and execs. Reachable with no
front-end edit — `--emit <name>` composes the flag and forwards every remaining argument.
*Consumers* — the five callers delta 7 enumerates, each reading **stdout**; and
`native/src/emit/kpi/always_loaded.rs`, reading the library function's figures in process at the
transition where `--emit-drift-report` collates its lead rows.

**New interface: the measurement function the arm wraps.**
*Producer* — `native/src/emit/always_loaded.rs`, called by the arm and by the KPI.
*Consumers and every field's named reader* — `surface` is read by the KPI's absent-check and by
the arm's `(surfaces N · hook N)` parenthetical; `hook` by the same parenthetical; `total` by the
KPI's `loaded <total>l` trend fragment and by the arm's lead figure; `base_total` and
`base_commit` by the arm's `+N since <sha>` suffix and by the KPI's delta fragment. No field is
added beyond those five, and each is one the rendered line already carried — which is the point
of the split: the figures stop being re-parsed out of prose and start being read where they are
produced.

**Existing interface whose producer moves: `CONTEXT_KIT_HOOK_CMD`'s default.**
*Producer* — was `always-loaded.sh:17-26`, becomes `context-kit/lib/context.sh`. *Consumer* —
the config bridge's `declare -p` read, at the transition where `gate_knob_env` resolves this
member's declared roster, and every reader that already resolved the knob through the library.
The enabling configuration is actually emitted: `scripts/context-config.sh` is this repo's seam
and sets neither of the two knobs the move touches, so the library default is the live value
here and the move is exercised on every invocation rather than in tests alone.

**Existing interface whose consumer set does not change: the baseline file.**
*Producer* — the `--update-baseline` mode, as today, writing the `# contract:` header and the
`<total> <surface> <commit>` data line, and preserving a consumer's trailing extra fields.
*Consumers* — the bare mode's delta, the KPI's trend fragment, and `context-kit/smoke/install.sh`
asserting the file appears. The trailing-extra-field tolerance is a named field with a named
reader outside this repo (§The always-loaded meter's fourth-field case) and is preserved
verbatim.

**This delta set narrows one corpus, and delta 6 is where point 5 binds.** Every other delta is
extent-preserving. Delta 6 narrows the set of accepted invocations, so each reader's **red
condition** is named rather than its subject: `check-settings-paths` reds on a **zero-resolution**
for a literal `.sh` grant, which is why delta 7's grant removal is same-commit rather than
tidy-up; `check-exec-bit` reds on an executable-bit mismatch over `*/bin/*.sh` and its corpus
loses a member, which is monotone and clears by inspection; the index-test runner reds on **any
diff from golden**, which is non-monotone in both directions and is therefore cleared only by
the golden running green over an unedited expectation file; and `check-gate-binary-fresh` reds on
a **stamp mismatch**, which is why the new crate source is staged before `build-native.sh` runs
and not after. No reader in the set reds on finding none, asserts an exact count, or holds a
coverage floor over this corpus.

## Existing sections updated

- **context-kit/SPEC.md §The always-loaded meter** — the mechanism restated for the arm: the
  bridged row and its four knobs, the three modes as operands, the spawned hook-body command with
  its command-seam ground, and the unrecognized-operand refusal (deltas 2, 4, 6).
- **context-kit/SPEC.md §lib/context.sh** — the section gains `CONTEXT_KIT_HOOK_CMD`'s default,
  its two-candidate resolution and its empty-on-unresolvable behaviour, which is where the knob's
  default now lives (delta 3).
- **context-kit/SPEC.md §Testing** — the sentence naming this member as the section's takeable
  singleton becomes the counterpart sentence: what stays owed to context-kit is the two sequenced
  members and `lib/toolfloor.sh`, and the golden's role as this cut's parity oracle is stated
  where the goldens are (deltas 1, 7).
- **context-kit/SPEC.md §bin/footprint** — its library-function-the-arm-wraps sentence gains its
  second instance in this kit, so the shape reads as the kit's rather than as that member's
  (delta 5).
- **drift-kit/SPEC.md §Bundled KPIs** — `kpi-always-loaded`'s row: the spawn and the two prose
  parsers are gone, the figures are read in process, and the presence witness is
  `lib/context.sh` (delta 5).
- **gate-sdk/SPEC.md §The non-gate arm** — the class roster gains `--emit-always-loaded`,
  undated, and the spawned-program prose gains this member's consumer-changeable set (deltas 4
  and 8).
- **gate-sdk/SPEC.md §The port disposition** — the owed-corpus prose every cut moves; context-kit
  keeps three owed members after this cut and the section says which (delta 1).
- **`context-kit/templates/close-brevity.md`** and **`context-kit/README.md`** — the invocation
  lines (delta 7).
- **`.claude/settings.json`** — the two grants naming the deleted path (delta 7).
- <!-- update-target-exempt: a generated projection with its own freshness gate and regen command, rostered in docs/site-architecture.md §Generated projections and their freshness gates; it stales on the line-count change alone and no delta owns its content --> `docs/footprint.md` and `docs/value.md`'s rollup block, and the `docs/` SPEC and README mirrors.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named, reachable producer
      and a named consumer; every new field has a named reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its proper
      canonical-spec section (not appended); the merged spec reads as one coherent document a
      reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component
      (`ls context-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec, config, template, doc and crate source for
      `always-loaded.sh`; nothing dangles.
- [ ] **The golden runs green over an unedited expectation file** — the index-test case is
      re-pointed at the front-end and `expected/always-loaded.txt` is not touched, which is the
      only form in which this cut's parity clearance is available.
- [ ] **The new crate source is staged before `bash gate-sdk/bin/build-native.sh` runs**, the
      tree `check-gate-binary-fresh` compares being `git ls-files`-derived.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a
      build-time causal gap is resolved that session, not deferred).
