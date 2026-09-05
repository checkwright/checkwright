# SPEC amendment: settings-allow-cut

The port disposition of **`guard-kit/bin/compare-settings-allow.sh` (115 lines), the one owed
file declaring guard-kit/SPEC.md §compare-settings-allow**: it ports to a bridged
`--emit-compare-settings-allow` `Arm::Emit`, and the match core it is built on stops being an
inline expression in one crate module and becomes a named primitive held to its shell twin. A
stated-contract cut under the port-first run (TRAJECTORY.md §PRIORITY DIRECTIVE), hosted on its
own per-cut feature entry and packaged by the lead as one of this iteration's two.

**The composer's precondition was run at this stage rather than inherited.**
`bash gate-sdk/bin/run-gates.sh --emit port-blockers --group` trails *0 group(s) formed … 0
takeable at this cut* — no takeable group, which is the budget arm's stated precondition
(gate-sdk/SPEC.md §The first cohort). The selection ground is the **owed column** of
`--emit port-blockers --tree` — *86 file(s) scanned, 67 declared no-port, 0 temporarily held,
19 owed* — where this file reads `owed lines=115`.

**This cut empties guard-kit's owed column outright, for the second time and the last.** The
kit's §Testing already recorded that its own cut "empties this section's owed set and not the
kit's", and named what stayed: "`bin/compare-settings-allow.sh` (§compare-settings-allow) — a
different section, correctly homed, and takeable as a singleton on its own rather than behind
anything this cut moved". That is this file. `lib/guard.sh` and `templates/bash-guard.sh` carry
`# no-port:` on the two independent grounds §The guard framework states — the sole-resolver
ground and the extension-point ground — and this amendment reopens neither. After this cut
guard-kit has no owed file at all.

## What changes

### (1) The cut is §compare-settings-allow's one owed file, and it empties the kit

`bin/compare-settings-allow.sh` reads `owed lines=115` and is the only owed file whose `# spec:`
pointer binds guard-kit/SPEC.md `## compare-settings-allow` {mechanical}. Its reach and its
section bound coincide, so no second section is rewritten by construction — with the exceptions
every cut moves, gate-sdk/SPEC.md §The port disposition's owed-corpus prose and the roster
sentence in §The non-gate arm that delta 7 adds it to, plus the two guard-kit sections deltas 3
and 6 name.

### (2) The arm is `--emit-compare-settings-allow`, an `Arm::Emit`, and both halves are forced

The member ports as a **bridged-arm table** row —
`("--emit-compare-settings-allow", Arm::Emit(compare_settings_allow::emit),
compare_settings_allow::KNOBS)` in `native/src/emit/mod.rs`'s `BRIDGED_ARMS`, with the body in
`native/src/emit/compare_settings_allow.rs` {design-bearing}.

**Table membership is forced by the forced-family test at its sharpest**, the shape
`--emit-scan-prompts` already holds in this kit: all four declared knobs —
`GUARD_KIT_SETTINGS`, `GUARD_KIT_SETTINGS_LOCAL`, `GUARD_KIT_BREADTH_PROBES` and
`GUARD_KIT_BREADTH_DECLARED` — are defined and defaulted in `guard-kit/lib/guard.sh`, a
`no-port` file the bridge sources to resolve them. A hardcoded top-level flag would resolve not
a stale default but **no input path at all**, and the arm would be unable to name the two files
it compares. The same fact makes the member unresolvable in a tree that does not vendor
guard-kit, which is the property `--emit-scan-prompts` already carries and is not a defect.

**Two of the four are array knobs and neither needs new mechanism** — probed rather than
assumed. `GUARD_KIT_BREADTH_PROBES` is an indexed array and `GUARD_KIT_BREADTH_DECLARED` an
associative one; `gate-sdk/lib/gate.sh`'s emitter derives the scalar/keyed arm from `declare -p`
and the crate reads them through `walk::knob_array` and `walk::knob_map`. The keyed knob is the
one worth naming: the bridge's family arm deliberately takes the scalar arm's associative branch
because `${map[*]}` yields values with keys destroyed, and this member's whole declaration
contract is the **key**, so a value-only crossing would silence nothing and report nothing while
passing every element-shape check.

**The `--emit-` spelling is forced by the exit grammar, read in the direction
`--emit-stage-rules` establishes.** The member's failure grammar is already `Arm::Emit`'s
collapse: every report path returns 0 — including the no-overlay path, the empty-probe-set path
and every path that finds candidates, this being an advisory that never renders a verdict — and
the one non-zero path is the `*)` usage refusal at `:17`, exit 2. It declares no 1 and never
has. **`--count` does not make it `Arm::Run`**: that mode answers two integers on stdout, which
is `--emit-queue-index extent`'s reading of the family — a member may be a *query* tool rather
than a generator — and its status carries no distinction the collapse would lose.

### (3) The match core becomes a named primitive and gains a parity mode

`guard_allow_match` (`lib/guard.sh:109-113`) is one line —
`[[ "$s" == ${glob//:\*/\*} ]]` — and it is the whole of what this member computes: the
redundancy question calls it, the breadth question calls it with the arguments swapped, and
§compare-settings-allow states in terms that this is why "there is no second matching
implementation and the `:*` normalization is shared" {design-bearing}.

**A compiled twin of it already exists, unheld, and this cut is what makes its divergence
consequential.** `native/src/emit/scan_prompts.rs`'s `granted_by` spells the same predicate
inline as `walk::glob_match(&p.replace(":*", "*"), c)`. The port does not create the second
holder; it makes a second compiled member depend on it, at which point an edit to one side and
not the other silently changes this member's prune and narrowing verdicts while rule 20's stay
right. So the predicate is **extracted** to `native/src/guard.rs` as `allow_match(s, glob)`
beside the three primitives already there, `scan_prompts::granted_by` is re-pointed at it, and
the new arm calls it — one compiled holder, not two.

**`--guard-lib-parity` gains a fourth mode, `allow-match`, and criterion 6's *unless* clause is
what admits the duplication at all.** The shell holder cannot empty: rule 20 calls
`guard_allow_match` from inside the same permanently-shell file, so this is the shape
§The guard framework already describes for the splitter, the normalizer and the redirect scan —
"the member whose second holder cannot empty even in principle". `gate-tests/guard-lib-parity.test.sh`
gains the fourth comparison over a canned corpus of (string, glob) pairs, A against B directly
with no committed expected file, on that harness's own stated ground that a maintained golden
would be a third copy to drift. The corpus is scoped to the shapes a **permission rule** can
carry rather than to arbitrary globs: the harness `:*` idiom in both positions, a bare trailing
`*`, an interior `*`, a `?`, a bracket class, and a literal with no metacharacter — the last
because a rule string is compared as a pattern and a consumer's literal must not acquire one.

**Extracting it is not scope creep and the test is stated so a later reader can check.** The
predicate is this member's entire computation; a cut that ported it by writing a second inline
copy would ship the member and the divergence in one commit, which is the duplication criterion
6 refuses reached by the back door. Enforcement-first's own clause — removing the duplication
outranks gating it — is what makes the extraction the primary move and the parity mode the
residue that holds what the extraction cannot remove.

### (4) `jq` leaves this member entirely, and the test's precondition goes with it

The shell form reads both allow lists with `jq -r '.permissions.allow[]?'` at `:27-28`
{design-bearing}. The port reads them with `serde_json`, which the crate already depends on, so
no dependency is added and this member spawns no external program at all — its spawned-program
set is **empty**, which is a first for the class and is why it is stated rather than left to be
inferred from a module with no `proc::` call.

**The scope of that claim is this member, not the kit.** `lib/guard.sh:269` still runs
`jq -r '.permissions.allow[]?'` for rule 20's own allowlist read, so guard-kit's floor is
unchanged and no `# no-port:` ground moves. What is genuinely removed is the last `jq` spawn on
*this* path.

**The consequence the sibling cut's rule inverts:** `gate-tests/compare-settings-allow.test.sh:10`
refuses at exit 2 when `jq` is absent, and that precondition exists only because the subject
spawned it — the sandbox JSON is written with `printf`, never `jq`. The `--run-guard-tests` cut
kept its own `jq` refusal because its subject still spawned one; this cut's subject stops, so
the precondition is **dropped** rather than carried. Carrying it would refuse a runnable suite on
a machine that no longer needs the tool, which is the mirror image of the false dividend that
rule was written against.

**The read must not degrade an unreadable allowlist into an empty one.** The shell form's
`2>/dev/null || true` reads a malformed settings file as zero entries and prints
*no redundant local entries*, a clean line the document does not support. The compiled read
distinguishes them: an unparseable or unreadable **local overlay** takes the existing
no-overlay path (the header and `no <path> — nothing to compare`, `0 0` under `--count`), and an
unparseable **committed** file is a refusal with the path named, exit 2 — the shape
`scan_prompts.rs` already states for its own allowlist read, "an unreadable allowlist from
reading as an empty one on a machine that merely lacks a tool". This is the cut's one deliberate
behaviour change beyond delta 5's, and it is named as one.

### (5) The `*)` refusal crosses the port; the missing help arm lands at the refusal, not the front-end

The shell form's `case` at `:14-18` already refuses an unrecognized first operand with usage on
stderr at exit 2, and it has no `-h`/`--help` arm at all — so `compare-settings-allow.sh --help`
prints usage on stderr at exit 2 today {design-bearing}. The arm keeps the refusal verbatim,
with the usage spelling updated to the front-end form
(`usage: --emit compare-settings-allow [--count]`).

**Where the usage lives is settled by the family rather than open.** An `--emit-` member gets no
front-end `case` arm — `--emit <name>` composes its flag, which is also why its spelling is
forced — and therefore no named line and no paragraph in `bin/run-gates.sh --help`. So "the
`-h`/`--help` arm retires to the front-end" is unavailable here, and the narrower reading
§The bin/-tool contract settled at its fifth reader instance binds: usage lives at the member's
own shape refusal. The `USAGE` constant `scan_prompts.rs:19` already carries is the shape.

**This member is not a reader instance of that contract and does not claim to be.** Its one
operand is drawn from a closed set, so validating membership already validates shape and the
free-text trigger never fires. It is named here only because the two readings are one sentence
apart and the next porting session will meet them together.

### (6) The bespoke test is re-pointed whole, and the split the sibling cuts took is refused

`gate-tests/compare-settings-allow.test.sh` drives the tool through `GUARD_KIT_CONFIG_FILE`
pointed at a sandbox config, with `GUARD_KIT_SETTINGS` and `GUARD_KIT_SETTINGS_LOCAL` set in the
environment beside it, so "the consumer's own probe array cannot leak into the fixture"
{design-bearing}. All three survive the port untouched: a config-file selector is read by the
bridge *before* it sources the owning kit's library rather than arriving after resolution, which
is §The non-gate arm's `LIFECYCLE_KIT_CONFIG_FILE` near miss exactly. Only `$CMP` changes, to
`bash gate-sdk/bin/run-gates.sh --emit compare-settings-allow`.

**The unit/seam split `--scratch-run` and the kfric port took is refused here, on the split's own
discriminator.** That split moved pure functions over inputs into `#[cfg(test)]` tests and kept
the cases that need the front end, the bridge and a real child process at the seam. Every case
in this suite is the second kind: each asserts the *report* rendered from four knobs a sandbox
config supplies, and the two that look like unit cases are not — the exactness case asserts that
the declaration lookup never became a glob match, which is only observable as an entry staying in
the narrowing section, and `--count`'s breadth number excluding a declared entry is a property of
the partition as rendered. Nothing is left over to move, so the split would produce an empty half.
The one predicate that *is* a pure function moves instead to delta 3's parity mode, where its
second holder is what makes it worth pinning.

### (7) §The non-gate arm's roster gains the member, undated

The class roster gains `--emit-compare-settings-allow` with its owning section named and **no
landing-date cohort label** {mechanical}, for the reason the sibling cuts state: those labels are
provenance, which `kit-spec-provenance-seam-sweep-remainder` retires, so a dated label would be
landed and removed inside one iteration. The spawned-program prose gains this member as the
class's first with an **empty** set, stated beside the worst cases it already names so a reader
sizing the class meets both ends of it.

### (8) The named callers, re-pointed in the same commit

Four callers name this tool and every one moves to
`bash gate-sdk/bin/run-gates.sh --emit compare-settings-allow` — verified this session over the
tracked tree rather than assumed {mechanical}:

- `guard-kit/templates/close-triage.md:27` — the close-stage triage step, which is the member's
  one **scheduled** reader and the mechanism §compare-settings-allow names in place of a gate.
- `guard-kit/gate-tests/compare-settings-allow.test.sh:8` — delta 6's harness.
- `guard-kit/README.md:68` — the cheat-sheet line.
- `guard-kit/SPEC.md:37` and `:1873` — the prose invocations, plus the `bin/` roster line at
  `:1902` which loses the file.

**The `.claude/settings.json` grant drops in the deleting commit**, in-cut and with no
out-of-band step, under `native-gate-port-remaining-corpus`' ruling (2) as widened 2026-09-05.
`Bash(bash guard-kit/bin/compare-settings-allow.sh)` is a literal `.sh` command token in
`check-settings-paths`' scope, so leaving it standing is a red rather than merely dead weight.
There is no `*`-suffixed twin for this member, unlike the sibling cut's — the entry is bare,
which is checked rather than assumed.

## Producers and consumers

**New interface: the `--emit-compare-settings-allow` bridged arm.**
*Producer* — `native/src/emit/mod.rs`'s `BRIDGED_ARMS` row, dispatched in `native/src/main.rs`
before the registry lookup; its enabling configuration is the bridged environment
`gate_knob_env` builds for it, which `bin/run-gates.sh` resolves and execs. Reachable with no
front-end edit — `--emit <name>` composes the flag and forwards every remaining argument. The
enabling configuration is actually emitted in a deployed tree: `scripts/guard-config.sh` sets
both breadth knobs here, and the kit ships neither, so both the empty-default path and the
populated path are live rather than test-only.
*Consumers* — the four callers delta 8 enumerates. The scheduled one is the close-stage triage
step, reading **stdout**; the suite reads stdout and the exit status; a session running it by
hand is the third.

**New interface: `guard::allow_match` and the `--guard-lib-parity allow-match` mode.**
*Producer* — `native/src/guard.rs`, called by `scan_prompts::granted_by` and by the new arm;
the parity mode's producer is `native/src/main.rs`'s `--guard-lib-parity` dispatch, one mode per
twinned predicate.
*Consumer* — `gate-tests/guard-lib-parity.test.sh`, at the transition where the fixture runner
executes it, comparing the mode's classification against `guard_allow_match`'s over one canned
corpus. That harness's caller cannot empty, because the shell holder is permanently shell and
rule 20 calls it from the same file — which is what distinguishes this parity mode from
`--declaration-parity`, retired when its second holder went.

**Existing interface whose fields keep their readers: the two report sets and the declaration
map.** `redundant` is read by the prune section's rendering and by `--count`'s first integer;
`too_broad` by the narrowing section and by `--count`'s second; `declared` by the declared-intended
section alone, and deliberately **not** by `--count`, because a declared entry is not
outstanding and the count's one purpose is how much is. `GUARD_KIT_BREADTH_DECLARED`'s value half
is read at the declared section's `<glob> ⊇ <probe> — <reason>` row and nowhere else, which is
the named reader that keeps it from being a silencer. No field is added.

**This delta set narrows no corpus.** The two allow lists, the probe set and the declaration map
are unchanged in extent; the four sections' emission conditions — an empty probe set omitting the
whole breadth section, an all-declared over-broad set printing no narrowing section and no false
clean line, an empty declaration map omitting only the declared subsection — are reproduced
condition for condition. Point 5 of the causal-completeness check therefore does not bind on the
corpus, but the readers whose verdicts are **not** monotone are named anyway because this cut is
cleared against them: the bespoke suite reds on any assertion mismatch **in either direction**,
so a green run over an unedited case set is the clearance and nothing else is;
`check-settings-paths` reds on a **zero-resolution** for a literal `.sh` grant, which is why
delta 8's grant removal is same-commit; `check-gate-binary-fresh` reds on a **stamp mismatch**,
which is why the new crate sources are staged before `build-native.sh` runs and not after; and
`check-exec-bit`'s `*/bin/*.sh` corpus loses a member, which is monotone and clears by
inspection.

## Existing sections updated

- **guard-kit/SPEC.md §compare-settings-allow** — the mechanism restated for the arm: the bridged
  row and its four knobs including the two array shapes, the `--emit-` family choice with
  `--count` as a query mode rather than a verdict, the operand refusal and where its usage lives,
  and the unreadable-versus-absent distinction the compiled read draws (deltas 2, 4, 5).
- **guard-kit/SPEC.md §The guard framework (`lib/guard.sh`)** — the twinned-primitive set becomes
  four, with `guard_allow_match`'s compiled counterpart named and its cannot-empty ground stated
  (delta 3).
- **guard-kit/SPEC.md §Testing** — the parity harness line gains the fourth primitive and its
  canned corpus; the bespoke suite's line records that it drives the arm through the front-end and
  that its `jq` precondition is gone; and the sentence naming
  `bin/compare-settings-allow.sh` as what stays owed to guard-kit becomes the sentence recording
  that nothing does (deltas 1, 3, 4, 6).
- **guard-kit/SPEC.md §Layout and configuration** — the `bin/` roster loses the file (delta 8).
- **guard-kit/SPEC.md §The close-stage triage step** — the invocation it names (delta 8).
- **gate-sdk/SPEC.md §The non-gate arm** — the class roster gains
  `--emit-compare-settings-allow`, undated, and the spawned-program prose gains this member as
  the class's first empty set (deltas 4 and 7).
- **gate-sdk/SPEC.md §The port disposition** — the owed-corpus prose every cut moves; guard-kit's
  owed column is empty after this cut and the section says so (delta 1).
- **`guard-kit/templates/close-triage.md`** and **`guard-kit/README.md`** — the invocation lines
  (delta 8).
- **`.claude/settings.json`** — the grant naming the deleted path (delta 8).
- <!-- update-target-exempt: a generated projection with its own freshness gate and regen command, rostered in docs/site-architecture.md §Generated projections and their freshness gates; it stales on the line-count change alone and no delta owns its content --> `docs/footprint.md` and `docs/value.md`'s rollup block, and the `docs/` SPEC and README mirrors.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named, reachable producer
      and a named consumer; every new field has a named reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its proper
      canonical-spec section (not appended); the merged spec reads as one coherent document a
      reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component
      (`ls guard-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec, config, template, doc and crate source for
      `compare-settings-allow.sh`; nothing dangles.
- [ ] **One compiled holder of the match core** — `scan_prompts::granted_by` calls
      `guard::allow_match` and spells the predicate nowhere itself; the fourth parity mode runs
      green over its canned corpus.
- [ ] **The bespoke suite runs green over an unedited case set**, which is the only form in which
      this cut's clearance is available, its assertions failing in either direction.
- [ ] **The new crate sources are staged before `bash gate-sdk/bin/build-native.sh` runs**, the
      tree `check-gate-binary-fresh` compares being `git ls-files`-derived.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a
      build-time causal gap is resolved that session, not deferred).
