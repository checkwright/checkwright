# SPEC amendment: criterion-five

**Criterion 5 of the port-candidate criteria stops pricing a port against an omit-and-declare install and prices what the installer now does.** A host the target roster carries no artifact for, and a POSIX host with no hasher, is refused at the bootstrap and receives no install (installer/SPEC.md §Hosts refused at the bootstrap, §The gate binary), and the installer writes no `# omitted:` record. Yet gate-sdk/SPEC.md §The port-candidate criteria still carries the retired omission branch: an instrument (the binary-less leg's omitted roster), a growth predicate, a standing quantity and a *standing judgment* naming a loss declared in the consumer's own `gates.list`. Exception class (b) and the criterion-5 bullet for a born-native gate rest on the same outcome, as do several cut records, the installer, guard-kit, canon-kit, doctrine-kit, lifecycle-kit and context-kit prose that cites it, the installer smoke's refusal message, and `check-install-platforms`' fourth arm, which reads it.

**Landing order.** Delta 5 lands before withheld-literals' delta 3, which edits the same gate module, registry row and fixture pair; SPEC-withheld-literals.md states the order.

**Measured at authoring (the tree already does what the corrected text says).**

- The bootstrap refuses an unrostered host, a broken pair and a hasher-less host: `installer/bin/checkwright.sh`'s `resolve_pair` and hasher branch, and installer/SPEC.md §Selection's table.
- `native/src/emit/installer_smoke/profiles.rs` (`placed_artifact`) fails any install whose registry carries a `# omitted:` line; `.workflow/validate-baseline.txt` holds a passing `installer_smoke artifact-less-refusal-leg` row and no held `fail` row for a cohort price.
- No shell gate declaration remains: `ls */checks/*.sh` finds nothing, and `bash gate-sdk/bin/run-gates.sh --emit port-blockers --tree` reports 0 temporarily held and no class-(b) holder. Every registered member dispatches to the binary, so a host with no binary has no battery of any substrate.
- `runner.rs` refuses an empty registry with `names no gates`. §run-gates lists that refusal among the arm's (*Every refusal is the arm's*), but its ground, that an empty registry never reads green whatever emptied it, is stated only inside a paragraph delta 1 replaces.
- TRAJECTORY.md's rulings and objectives sections are empty, so no recorded ruling is reversed: the *ruled* and *accept and declare* wording is SPEC prose in per-cut records.

## What changes

### (1) Criterion 5 restated against the bootstrap refusal {design-bearing}

gate-sdk/SPEC.md §The port-candidate criteria, criterion 5 body. **Not yet applied.** The first paragraph (*Its vendored form stays runnable*) stands. The paragraphs after it, identified by their lead-ins, are replaced: *The condition that satisfies this criterion is ruled*, *What the omission branch governs*, *It also bounds what the installer may relocate*, *The criterion is priced per member and paid per cohort*, *The residual is the omitted roster and its count*, *That instrument is per-cohort; the standing one is `check-install-platforms`' fourth arm*, *The measurement's order is fixed*, *The growth predicate*, *The residual is a standing quantity*, *The verdict is a price, not a screen*, *The standing judgment is accept and declare*, *An all-omitted install refuses*, and *The value arm is a different claim*. The replacement, in order:

> **The condition that satisfies this criterion is ruled.** The payload carries a prebuilt binary per declared target, built by the release and never from a working tree. The installer resolves the host to a target, verifies the matching artifact against a published digest and copies it; no selection ever builds. Both halves ship: the publish workflow's roster-derived build matrix emits a binary and a digest sidecar per declared target, `--pack-installer` verifies and places them, the Release publishes them (§Consumer payload), and `init` resolves, verifies and places. The criterion is satisfied **per target, not globally**: a host whose triple the roster carries no published artifact for is refused at the bootstrap (installer/SPEC.md §Hosts refused at the bootstrap), a complement stated rather than counted, since a count moves on a roster join and on every release.
>
> **An uncovered host receives no install, so no member is dispatched into an absent binary and none is omitted.** Every install step sits behind the invoke, so a host without a verified binary has no path an install could proceed through. The bootstrap refuses it, names the platform and writes nothing, and the consumer smoke's artifact-less refusal leg asserts that refusal (installer/SPEC.md §The consumer smoke). The registry's `# omitted:` line is a consumer's own reason-agnostic record (§run-gates), never the installer's.
>
> **It also bounds what the *installer* may relocate.** A host with no verified binary leaves `init` nothing to invoke, so an install step moved behind the invoke is a step such a host cannot run, which is why it is refused rather than half-installed. The rule is installer/SPEC.md §The install boundary's; this criterion is where a porting session meets it. The consumer smoke packs a real artifact on the main payload, so every profile it installs takes the placement branch.
>
> **The criterion prices the support roster, never a member or a cohort.** A host outside the roster loses the whole install whatever substrate a member is written in, so porting a member, or landing a born-native one, removes nothing such a host would have kept, and no per-cohort quantity exists to measure. The binary substrate's cost is the roster's complement, the hosts `native/targets.list` carries no artifact for: the project's support commitment, held at the commit that would break it by `check-install-platforms`' lockstep of the platform declaration, the roster and both host detectors (§Consumer payload), and shrunk as a target joins on a run that produced and exercised its artifact. No member is held back on shell for an uncovered host's sake, since that host has no battery for a shell member to keep alive.
>
> **What a cohort owes under this criterion is reachability on a covered host.** Each member registers as a dispatch to its descriptor, so the placed binary carries it and §check-gate-binary-fresh holds that binary fresh against the source. The cohort adds no install step outside the invoke, and the consumer smoke's per-profile battery, run on a payload carrying a real artifact, stays green.
>
> **The value arm is the covered-host claim.** It plants a real defect in adopter-authored prose and asserts that some profile below the maximum catches it, on a payload that carries an artifact (installer/SPEC.md §The consumer smoke). A cohort that changes what a covered host catches is held there, never by this criterion.

### (2) The born-native bullet and exception class (b) {design-bearing}

gate-sdk/SPEC.md §The port-candidate criteria. **Not yet applied.** Class (b)'s ground, a gate left out on an uncovered platform where it is the only reader, falls with the omission: a host with no artifact receives no install of any substrate, so a shell form reaches no host the descriptor does not. No member holds class (b) (delta 1's probe). The SPEC's own rule makes a class change an amendment, which this is.

The born-native list's criterion-5 bullet (*5 binds hardest, and it is the price.*) becomes:

> - **5 binds at the roster and no further.** A born-native `.gate` member dispatches to the shipped binary, so it runs on every host the installer serves and on none it refuses. A shell form would reach no host the descriptor does not, since a host with no artifact receives no install of any substrate, so the born-native default charges criterion 5 nothing per gate.

The paragraph *The exception criterion: two live classes…* becomes:

> **The exception criterion: one live class with a stated cause, and two retired.** Shell is taken only under the live class, (c), and the gate's own SPEC section states why. A further class is an amendment, not a judgment call.

Class (b)'s bullet is retired in (a)'s shape, keeping its letter:

> - **(b) RETIRED: the gate's subject is a platform the target roster does not cover.** The class held such a gate in shell because born-native would omit it on exactly the platforms it reads. A host the roster carries no artifact for receives no install of any substrate (installer/SPEC.md §Hosts refused at the bootstrap), so a shell form reaches no host the descriptor does not, and no member holds the class.

The ripple edits re-phrase:

- *The held classes take their own field…* becomes *The held class takes its own field, `# port-until: <slug>`*; *Classes (b) and (c) are temporary, so `# no-port:` must not carry them* becomes *Class (c) is temporary, so `# no-port:` must not carry it*; *its domain is wider than these two classes* becomes *wider than this class*.
- In the cause's readers list, *the session landing the missing substrate or target, for which a class-(b) or class-(c) cause is the list…* becomes *the session landing the missing substrate, for which a class-(c) cause is the list of gates that become portable with it*.
- In *What the classes deliberately exclude*, the sentence opening *Nor is* and the class-(b) sentence after it are replaced by one sentence, keeping the quoted-claim shape of the excluded causes before it:

  > Nor is *the gate would be lost on an uncovered platform*: no gate of any substrate reaches a host the roster does not cover.
- §The `# graph:` manifest's *`# port-until:` covers any temporary hold…*: *wider than the two born-native exception classes … Those classes describe* becomes *wider than the born-native exception class … That class describes*.

Build keeps every class letter as it is, so a cause citing (c) elsewhere keeps resolving.

### (3) The cut records stop citing the retired outcome {mechanical}

gate-sdk/SPEC.md. **Not yet applied.**

- §The fourth budget batch, *Width buys session overhead…*: *one criteria audit, one assertion-C re-run, one criterion-5 measurement and one amendment* becomes *one criteria audit, one assertion-C re-run and one amendment*.
- The `check-spec-embedded-source` cut's *Criterion 5 is ruled accept and declare against this cut's own subtraction.* becomes *Criterion 5 charges this cut nothing: a host outside the roster never receives the binary the guard moves behind, and a covered host runs the same rule.*
- The consumer-declared cohort's *A reader of a ported member no kit roster names…*: its last sentence becomes *A tree-green repair is not a consumer-green one, and the consumer smoke's per-profile battery, run on a freshly installed consumer, tells them apart (installer/SPEC.md §The consumer smoke).*
- Its *The aggregate price is zero, reasoned before it was measured.* becomes *The members assert over kit-authored files only, a vendored kit's own `smoke/`, `gate-tests/`, `templates/` and registry rows, so no class of adopter-authored content rests on them.*
- The `spec_canonical_specs` cohort's *Criterion 5 is accept and declare, with the rivals refused.* becomes *Criterion 5 charges this cohort nothing, and the shell-side rival is refused: a host outside the roster loses the Definition-of-Done singleton and the derivable-section budget with every other member, and restoring the class shell-side would reinstate the second `spec_canonical_specs` implementation the criterion-6 discharge removed.*
- *A binary-less measurement is scoped to the profile its leg installs.* and §The declaration cohort's *Criterion 5's structural ground for a consumer-declared member.* are cut: each scopes the retired instrument.

### (4) Surfaces that cite the omission branch {mechanical}

gate-sdk/SPEC.md. **Not yet applied.** Each re-phrases the sentence named; the rest of its passage stands.

- §The harness-integration arm's fail-open paragraph: *A tree with no binary for its platform (criterion 5's omit-and-declare branch) would refuse every guarded call.* becomes *A tree with no binary, one deleted or vendored by hand on a host the roster carries no artifact for, would refuse every guarded call.*
- §The first cohort's *A declaration is not a dispatch.*: *The sharpest case is every adopter on `init`'s omit path, whose members are dropped from the registry while the descriptors still vendor.* becomes *The sharpest case is a consumer registering fewer members than its vendored kits ship, so a descriptor sits in the tree that nothing dispatches to.*
- §Consumer payload's *Ruled elsewhere.*: *omit-and-declare where the target roster carries no artifact for the host* becomes *a host the target roster carries no artifact for refused at the bootstrap*.
- §Consumer payload's *The installer needs the roster…*: *This platform was never committed to is omit-and-declare, a supported outcome.* becomes *This platform was never committed to is a refusal with no adopter action.*, and the next sentence *This platform was committed to and the artifact is missing is a broken payload, also a refusal, with a remedy of its own.*
- §lib/gate.sh's `gate_native_targets` bullet: *the distinction §Consumer payload's omit-and-declare path turns on* becomes *the distinction §Consumer payload's selection table turns on*.
- §Consumer smoke, *This harness does not run the installer…*: *so no payload, no digest and no `# omitted:` record are in play* becomes *so no payload and no digest are in play*.
- The fail-closed dispatch paragraph (*The binary's path is the knob*): *a member arrives with its verified artifact or not at all, omitted from `gates.list` and recorded there (…criterion 5). So exit 2 is the backstop for a tree whose binary was deleted or replaced* becomes *a host receives its verified artifact or is refused before anything is written (…criterion 5). So exit 2 is the backstop for a tree whose binary was deleted or replaced, or one vendored by hand on a host the roster does not cover*.
- §run-gates, *A declared omission is what keeps that tripwire honest*: the sentences from *It is not the installer's, whose own selection outcome retires…* through *…publish an installer decision as a kit narrowing* become *The installer writes none (installer/SPEC.md §The gate binary).* The ground delta 1 removes is re-homed as the paragraph's last sentence: *A registry whose live set is empty, whatever emptied it, takes the arm's `names no gates` refusal at exit 2 (below), so an empty registry never reads green.*
- §run-gates, *The front-end's port disposition is the stub*: *A host with no verified artifact seeds a registry with no live member, which the arm refuses rather than passing vacuously.* becomes *A registry with no live member is refused rather than passing vacuously.*
- §lib/test-hermetic.sh's refused alternatives, the `--emit-test-hermetic` bullet: *A consumer vendoring the shell library on an uncovered platform would lose hermeticity silently* becomes *A tree vendoring the shell library with no gate binary would lose hermeticity silently*.
- §run-gate-tests, *A `*.test.sh` may also be a *scenario* runner*: *a consumer on an uncovered platform, vendoring the shell library with no artifact behind it* becomes *a tree vendoring the shell library with no binary behind it*, and *That is the reporting shape the omitted-member roster uses, so a reader can tell* becomes *So a reader can tell*.
- §with-foreign-shells, *It is an arm, not a recipe or a `scripts/` tool.*: *a no-port cause from the live exception classes* becomes *a no-port cause from a live exception class*.
- §upgrade-smoke, *Criteria 2 and 5, in this member's terms.*: from *Criterion 5's residual is narrow…* to the paragraph's end becomes *Criterion 5 costs it nothing: a consumer on a host the roster does not cover is refused at install and has no such suite to lose, and the suite runs in the kit-source repo, where `cargo`, already required for any ref that dispatches to the binary, builds the binary the arm needs. It is a validate-stage, pre-release instrument rather than an adopter-facing one.*
- §check-gate-binary-fresh, *What makes the binary load-bearing…*: *commented out by `init`'s `# omitted:` record* becomes *omitted by a consumer's own `# omitted:` record*, and *covers the omit path* becomes *covers the omitted case*; its *Too tight…* sentence's *an adopter on `init`'s omit path, whose re-run on a machine that has since gained a hasher must convert the member back* becomes *a consumer that omitted a member by its own record, whose re-registration must convert the member back*.
- The toolchain section's closing paragraph: *Criterion 5's omission (§The port-candidate criteria) is a vendor-time decision `init` makes before any gate runs* becomes *Criterion 5's refusal (§The port-candidate criteria) is a vendor-time decision the bootstrap makes before any gate runs*.

### (5) `check-install-platforms`' fourth arm is retired {design-bearing}

Arm D prints, per held platform, *`<triple>` omits N member(s)*, the count of registry members resolving to a `.gate`. It asserts nothing, so no red condition moves, but its label names the retired outcome: a held host loses the whole install, and no decision turns on the number. The gate is withheld to this repo's gates dir, so no adopter meets the change. **Not yet applied.**

- **Code.** `native/src/gates/install_platforms.rs`: remove `omitted_members`, `omitted_report` and their call sites, the clean-line clause naming the omitted count, the red-path line *omitted on each held platform*, and the header comment's per-platform count. The detector-count comment before `detector_report` grounds itself on *the same vacuous-pass ground arm D stands on*; it keeps its ground and drops the arm-D reference: *the count rides the clean line because a source scan whose extraction quietly stops matching reports an empty set as agreement, and a number is what makes that visible without an audit*. In `native/src/gates/mod.rs`, drop the arm-D `// spec:` comment and drop `GATE_SDK_GATES_DIR` and `GATE_SDK_KIT_DIRS` from the row's knob slice, since both are read only inside `omitted_members` (`grep -n 'knob_scalar\|kit_roots\|GATE_SDK' native/src/gates/install_platforms.rs`); `check-reads-couples` holds the slice to what the module reads.
- **Fixtures.** `scripts/gate-tests/check-install-platforms/{good,bad}`: remove the `scripts/` tree and `kitroot/checks/check-vendored.gate`, which only arm D reads; drop `good/expect.txt`'s `omits 2 member(s)` line and `bad/expect.txt`'s `omitted on each held platform:` line, and the `args` comments naming arm D's count.
- **docs/site-architecture.md** §Generated projections: cut from *It carries a fourth arm that asserts nothing and reports instead.* through the sentence on how the two relate, with its pointer to the aggregate cost's instrument; *The binding is four-way rather than three-way…* ends at the detectors' set equality.

### (6) Installer and guard-kit surfaces that name the retired outcome or the old leg {mechanical}

**Not yet applied.**

- installer/SPEC.md §Hosts refused at the bootstrap: *Both hosts once **proceeded**, omitting the compiled gates…* becomes *A refused host is not served a smaller battery: every registered member dispatches to the binary, so an artifact-less install would have no live member.*
- installer/SPEC.md §Selection's oracle paragraph: *the binary-less leg's* becomes *the artifact-less refusal leg's*; the same rename in §The consumer smoke's cost paragraph.
- installer/SPEC.md, *The registry's `# omitted:` class is gate-sdk's, not an install outcome.*: from *`init` once wrote it…* through *…publish an installer decision as a kit narrowing* becomes *`init` writes none: an unrostered host and a hasher-less host are refused before an install begins (§The install boundary), so an install that happens at all installs the whole starting roster.*
- `native/src/emit/installer_smoke/artifact.rs`, the placement-on-an-uncommitted-host failure message: *omit-and-declare retired with the relocation, so this platform is refused rather than served an install whose battery cannot run* becomes *this platform is refused at the bootstrap rather than served an install whose battery cannot run*. No test reads the message (`git grep -n "never committed to" -- native`).
- guard-kit/SPEC.md §scan-prompts, *What a consumer on an artifact-less host loses is one advisory ranking*: *artifact-less host* becomes *tree with no gate binary*, and *so no `gates.list` row is omitted (gate-sdk/SPEC.md §The port-candidate criteria, criterion 5)* becomes *so no `gates.list` row depends on it*.

### (7) Other kits' prose that prices the retired outcome {mechanical}

**Not yet applied.** Each re-phrases the sentence named; the rest of its paragraph stands.

- canon-kit/SPEC.md, §check-measured-claim's *Born native, and this is the first gate with no shell original.*: from *The accepted cost:* through *a paid price, not an open risk.* becomes *Criterion 5 charges it nothing: a host the release publishes no artifact for is refused at install and receives no gate of any substrate, so a shell form would reach no host this descriptor does not.*, and *with shell an exception under one of three stated classes* becomes *with shell an exception under a live exception class*.
- doctrine-kit/SPEC.md §stage-rules, the paragraph opening *Criterion 6 is discharged by the duplication being absent*: from *Criterion 5's residual is narrow and real* through *where a shell script used to give it to them.* becomes *Criterion 5 charges it nothing: a host the artifact roster does not cover is refused at install, so no consumer loses the craft-rule pointer block to a missing binary that a shell script would have kept.*, and the next sentence opens *In a tree whose binary is absent all the same, the block is advisory output…*.
- lifecycle-kit/SPEC.md §bin/install-lifecycle.sh, the paragraph opening *Advisory tooling, not a gate, so no fixture pair is owed.*: from *The platform residual bites at adoption* to the paragraph's end becomes *No platform residual bites at adoption: a host the artifact roster does not cover is refused at the bootstrap (installer/SPEC.md §Hosts refused at the bootstrap), so no vendored consumer is left holding the registration block and merge attributes with no writer, and `check-lifecycle-registration` and `check-merge-attrs`, compiled too, never face a host that cannot run them.*
- context-kit/SPEC.md §The session-context hook (template), *The availability guard reads the gate binary, and its ordering is contract.*: *on every host the artifact roster does not cover* becomes *in every tree with no gate binary*.
- CLAUDE.md's born-native line: *a cause from the live exception classes* becomes *a cause from a live exception class*.

### (8) The site mirrors follow {mechanical}

`docs/gate-sdk/SPEC.md`, `docs/installer/SPEC.md`, `docs/guard-kit/SPEC.md`, `docs/canon-kit/SPEC.md`, `docs/doctrine-kit/SPEC.md`, `docs/lifecycle-kit/SPEC.md` and `docs/context-kit/SPEC.md` are regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` in the commit landing deltas 1 to 7.

## Producers and consumers

Point 5 binds: a corpus narrows. Every reader of the retired outcome, found by the probes below, with its red condition:

- **The runner's omission line** (`native/src/runner.rs`) and **`doctor`'s omitted block** (`native/src/installer/doctor.rs`, `omitted_block`) read whatever `# omitted:` lines a consumer wrote, reason-agnostically. The installer is no longer a producer; neither red condition moves.
- **The smoke's no-record assertions** (`profiles.rs` `placed_artifact`, `init.rs`'s unit test) red on a `# omitted:` line after a placing install, which is the corrected text; unchanged.
- **The binary meta-gates' shared predicate** (`check-gate-binary-fresh`, `check-gate-substrate-parity` assertion F) never names `# omitted:`; `registry::members` strips comment lines. Only prose naming `init`'s omit path is stale (delta 4).
- **`check-install-platforms` arm D** reports and asserts nothing, so it has no red condition to move; delta 5 retires it. The gate's six asserting arms, A, B, C, E, F and G, are untouched.
- **A held validate `fail` row for an unpaid cohort price**, which delta 1's removed paragraphs name: `.workflow/validate-baseline.txt` holds none, and `grep -rn 'aggregate price\|unpaid' lifecycle-kit evidence-kit .workflow` finds no reader.
- **Point 6** is vacuous: no member is obliged. No state, knob, event or interface is added, and no delta is user-facing: every edit is SPEC prose, a withheld gate's report line or a publisher-side smoke message.

## Existing sections updated

Rosters by `git grep -n -I -i -E 'binary-less|artifact-free|accept and declare|criterion 5|criterion-5|# omitted:'` and `git grep -n -I -i -E 'omit-and-declare|omit path|uncovered|does not cover|live exception classes'` over the tracked tree, `grep -rn -i 'omit' native/src --include=*.rs`, and `grep -n 'arm D\|fourth arm'` over `native`, `docs`, `scripts` and `installer`.

- `gate-sdk/SPEC.md` — §The port-candidate criteria (deltas 1 and 2), §The `# graph:` manifest (delta 2), the cut records (delta 3), and the surfaces delta 4 lists (delta 4).
- `native/src/gates/install_platforms.rs` — arm D's functions and lines (delta 5).
- `native/src/gates/mod.rs` — its registry row (delta 5).
- `scripts/gate-tests/check-install-platforms/` — the fixture pair (delta 5).
- `docs/site-architecture.md` — §Generated projections' arm-D sentences (delta 5).
- `installer/SPEC.md` — four paragraphs (delta 6).
- `guard-kit/SPEC.md` — §scan-prompts' artifact-less paragraph (delta 6).
- `native/src/emit/installer_smoke/artifact.rs` — its uncommitted-host failure message (delta 6).
- `canon-kit/SPEC.md` — §check-measured-claim's born-native paragraph (delta 7).
- `doctrine-kit/SPEC.md` — §stage-rules' criterion paragraph (delta 7).
- `lifecycle-kit/SPEC.md` — §bin/install-lifecycle.sh's advisory-tooling paragraph (delta 7).
- `context-kit/SPEC.md` — §The session-context hook's availability-guard bullet (delta 7).
- `CLAUDE.md` — the born-native line (delta 7).
- `docs/gate-sdk/SPEC.md` — the regenerated mirror (delta 8).
- `docs/installer/SPEC.md` — the regenerated mirror (delta 8).
- `docs/guard-kit/SPEC.md` — the regenerated mirror (delta 8).
- `docs/canon-kit/SPEC.md` — the regenerated mirror (delta 8).
- `docs/doctrine-kit/SPEC.md` — the regenerated mirror (delta 8).
- `docs/lifecycle-kit/SPEC.md` — the regenerated mirror (delta 8).
- `docs/context-kit/SPEC.md` — the regenerated mirror (delta 8).
<!-- update-target-exempt: surfaces the probes reached and judged still true, so no delta edits them -->
- Cleared, no edit: §Consumer payload's *Criterion 5's delivery mechanism* bullet, *Building the binary per vendoring is refused*, the binary-less `--help` paragraph and the scenario runner's skip, whose host clause alone delta 4 re-phrases (an absent binary is still a state), installer/SPEC.md's no-build and artifact-free payload sentences, and `front_end_fail_open.rs`'s "binary-less session".
<!-- update-target-exempt: a published release note, a frozen record that names the retired outcome; listed so the retired-spelling reconciliation reads it as deliberate -->
- `docs/posts/2026-09-26-checkwright-v0-26-0.md` — no edit; the dated release notes under `docs/posts` are frozen records.

## Retired spellings

- `accept and declare` — the standing judgment and the two cut-record rulings (deltas 1 and 3).
- `omit-and-declare` — the install outcome the installer retired (deltas 1, 4, 6 and 7).
- `binary-less residual` — the retired per-cohort quantity (deltas 1 and 5).
- `binary-less leg` — the old name of the artifact-less refusal leg (deltas 1, 3, 5 and 6).
- `omitted roster` — the retired instrument's roster (deltas 1 and 5).

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each retired reader above.
- [ ] **Instruction surfaces: instruction only** — no template, agent definition or shim is touched; CLAUDE.md's one edit keeps its line an instruction.
- [ ] **Merged with no information lost** — each replacement re-phrases the passage it replaces, and the empty-registry refusal lands in §run-gates in the commit that removes the paragraph carrying it.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls gate-sdk/SPEC-*.md`), discharged at the iteration while sibling gate-sdk amendments are in flight.
- [ ] **Removals propagated** — `check-amendment-retired-spelling` green over the declarations above.
- [ ] **Gaps filed** — a residual surface found at build is resolved that session or filed.
- [ ] **Run green** — `bash gate-sdk/bin/build-native.sh`, the full battery and the `check-install-platforms` fixture pair in the merging batch.
- [ ] **The entry is done** — `criterion-five-omit-stale` moves to Done in the merging commit, before the drain stage.
