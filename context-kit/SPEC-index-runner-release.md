# SPEC amendment: index-runner-release

**`context-kit/SPEC.md` §Testing holds three files owed where only two are coupled, and this
narrows the hold to the two that are.** The section's sequencing sentence names
`bin/run-index-tests.sh` and `index-tests/toolfloor-cases.sh` as "sequenced together with" each
other behind `lib/toolfloor.sh`'s installer relocation, and its closing count says "three files,
and no fourth". The coupling is real for one of the two and absent for the other, so the hold
over-declares by exactly one member and that member is a 107-line singleton.

**Why it is an amendment and not a reading.** The section *names* the member and closes its own
reach, so releasing it against the section's words is spec-over-precedent violated at the point
the doctrine is about. The 2026-09-06 lead declined to take it as a build reading on exactly that
ground, recorded the argument persuasive, and left the amendment route open; this is that route.

**The unit is port-critical, so the release and the cut ride one amendment.** Delta 3 specifies
the cut because build sessions never author specs (canon-kit/SPEC.md §The amendment lifecycle) —
a release that left the port disposition unwritten would hand build a spec-authoring job wearing
a build job's name. Deltas 1 and 2 stand alone and leave the tree consistent; delta 3 depends on
them.

## What changes

### (1) §Testing's sequencing narrows to the two members that genuinely couple

The sentence at `context-kit/SPEC.md` §Testing — "it declares §Testing, and it is sequenced
together with `index-tests/toolfloor-cases.sh` … That sequencing reaches exactly those two
members" — is rewritten so the sequencing reaches `index-tests/toolfloor-cases.sh` and
`lib/toolfloor.sh` alone, and `bin/run-index-tests.sh` is released from it {design-bearing}.

**The discriminator is the section's own, not a new one.** §Testing already worked this exact
case for the AGENTS.md smoke and stated the test: a member that **sources neither the runner nor
the library** is unblocked on its own ground, because a stated cause reaches only the members it
names (gate-sdk/SPEC.md §Porting a gate to the binary substrate). Applied to the two members the
sentence currently binds together, the test separates them:

- `index-tests/toolfloor-cases.sh:8` is `source "$LIB/toolfloor.sh"` — a **content coupling**.
  The case table is a projection of the library's own verdict set, so the two move together and
  the sequencing is correct for this member.
- `bin/run-index-tests.sh` sources nothing at all. Its only contact with the sequenced library is
  `:53`, `check toolfloor "$EXPECTED/toolfloor.txt" bash "$KIT/index-tests/toolfloor-cases.sh"` —
  a `bash` subprocess spawn whose stdout is diffed against a golden, **structurally identical to
  the five spawns above it at `:49-52`** that already reach ported arms through the front-end.
  A spawn is a process boundary, and a member that spawns a still-shell sibling is coupled to it
  no more tightly than to the compiled arms it already spawns.

**What the current sentence actually asserts, and why it is a co-location claim.** The two files
share a section and a subject; nothing in the sentence's own words establishes that the runner
reads, sources, or is read by `lib/toolfloor.sh`. "Sequenced together with" is doing the work of
a coupling claim on the strength of adjacency, which is the over-declaration this delta removes.
The rewritten sentence states the coupling **and its mechanism** (`source`), so a later reader
can re-run the test rather than inherit the verdict.

**The honest limit, stated in the section rather than left to a reader.** Releasing the runner
does not release `index-tests/toolfloor-cases.sh`, and cutting the runner leaves that file still
owed and still spawned by the compiled arm. The section says so, so a later cut does not read
delta 3's landing as having emptied the pair.

### (2) The section's owed-set count corrects to the two that remain

`context-kit/SPEC.md` §Testing's closing sentence — "What stays owed to context-kit after both
cuts is the two sequenced members named above and `lib/toolfloor.sh` behind the same installer
relocation — three files, and no fourth" — corrects to name `index-tests/toolfloor-cases.sh` and
`lib/toolfloor.sh` as what stays owed *behind the installer relocation*, with
`bin/run-index-tests.sh` named as released and takeable {mechanical}.

The count is derivable and so is not restated as a bare number the section then has to maintain:
the sentence names the two members and cites `bash gate-sdk/bin/run-gates.sh --emit
port-blockers --tree` as the roster's oracle, which is the derivation-first form the same section
already uses for sizes. The "no fourth" clause's purpose — closing the section's reach so a later
session cannot quietly add a member — is preserved and reattached to the narrowed set.

**Measured at this spec, HEAD `18ce331c`:** `--emit port-blockers --tree` reads 83 scanned, 67
no-port, 0 temporarily held, 16 owed. `context-kit/bin/run-index-tests.sh owed lines=107`.
Note the oracle already reports the file **owed rather than held** — it carries no `# no-port:`
and no hold declaration — so the hold lives in §Testing's prose and nowhere else, and deltas 1
and 2 are the whole release mechanism. Nothing else in the tree has to move for delta 3 to be
takeable.

### (3) `bin/run-index-tests.sh` cuts to the binary as a non-gate `Arm::Run`, spelled `--run-index-tests`

The released 107-line singleton lands as a bridged non-gate arm on the three properties
gate-sdk/SPEC.md §The non-gate arm specifies, and its disposition is written into §Testing beside
the AGENTS.md smoke's {design-bearing}.

**The spelling is `--run-index-tests`, a bare flag, and the variant decides it.** The runner's
contract is a **verdict** — 0 with the clean summary line, 1 with the failing-check report, 2 on
a harness error (no golden, tool exited non-zero) — which the `index_tests` validate suite reads.
That makes it `Arm::Run`, and §The non-gate arm's correlation rule fixes an `Arm::Run` at a bare
flag: `--run-gate-tests`, `--run-guard-tests`, `--run-demo` and `--agents-md-smoke` are the
precedent set. The `--emit-` prefix is load-bearing for the emit family alone and is not
available here. No front-end edit is owed: `bin/run-gates.sh`'s argv case falls through on any
leading `-` token and execs the arm name.

**`--update` survives as argv.** It is an argument the rule itself consumes rather than a
selector for where configuration comes from, so it stays argv under §The non-gate arm's
distinguishing test — the `--keep` precedent on `--agents-md-smoke`. The file's `usage:` line
moves to the arm's own refusal text.

**The arm keeps spawning the front-end, and that is the property the port must not lose.** The
five index checks reach their arms through `bash gate-sdk/bin/run-gates.sh --emit <name>` rather
than through the binary, because the front-end is what resolves the bridged environment two of
them declare; §Testing already states this and it survives the port unchanged. The compiled arm
therefore spawns `bash` and `git`-free subprocesses per check, which §The non-gate arm's prose
roster of spawned programs records for the arm (that roster is prose, since `--needs` answers
about registry members and a bridged arm is not one).

**The goldens are the port's acceptance oracle and are held byte-for-byte.** They were produced
by the shell implementations the arms replaced, so they are a cross-substrate comparison over a
committed corpus rather than an assertion of parity — §Testing says so and the port does not
weaken it. The cut asserts byte-identity the way §port-blockers' own port did: capture the shell
form's stdout and exit status at the commit before deletion, across the bare arm and `--update`,
and diff both including exit codes. **No expectation file is edited by this cut.** An edited
golden would convert the parity oracle into an assertion about the new implementation, which is
the one way this port can pass while destroying the thing it ported.

**Three sub-behaviours a port most easily loses, named because each is a check the goldens do not
hold:**

- The **consumer-shadowing case** (`:56-68`) mktemps a `CONTEXT_KIT_PUB_LANG_DIR` whose `rust.sh`
  emits a marker row, and its golden records the shadow's output rather than the built-in
  grammar's. It is the extractor seam's end-to-end proof, so the arm must still write a scratch
  extractor to disk and pass `CONTEXT_KIT_CONFIG_FILE` into a spawned front-end — resolving that
  knob for the arm itself would hand it this repo's posture instead of the one it is constructing,
  the same rule §Testing states for the AGENTS.md smoke's six-knob set.
- The **refusal case** (`:81-91`) asserts exit 2, the usage block on **stderr**, and **nothing on
  stdout** for `--emit always-loaded --growht`. No golden holds it; the assertion is the exit
  status and the stream. It survives as an assertion in the arm, not as an expectation file.
- The **`norm()` path rewrite** (`:19`) rewrites `^[^ ]*/corpus/` to `corpus/` so the goldens are
  machine-independent. Losing it makes every golden hold an absolute path and the suite passes
  only on the machine that last ran `--update`.

**Scratch lifecycle is the arm's own control flow rather than a trap.** The shell form arms one
`trap … EXIT` at `:71` that cleans two mktemp'd files and one mktemp'd dir — and it is armed
*after* the shadow dir and config are already created and used, so a failure between `:56` and
`:71` leaks them today. The arm cleans every scratch path on every exit path, which repairs that
window rather than reproducing it; the `--agents-md-smoke` port took the same shape and its
reasoning is §Testing's.

## Producers and consumers

**New interface: the `--run-index-tests` arm (delta 3).**

- **Producer.** `gate-sdk/bin/run-gates.sh`'s leading-`-` argv fall-through execs the binary with
  the arm name; the arm is a `BRIDGED_ARMS` row resolved before the registry lookup, absent from
  `--list` (§check-gate-substrate-parity assertion B equates the descriptor set with exactly what
  `--list` prints, so an arm inside that roster would red the gate). No enabling config is minted:
  the arm's declared knob roster is whatever the bridge must resolve for it, and every
  `CONTEXT_KIT_` knob the checks need is written into a spawned child's environment instead —
  never resolved for the arm — for the reason §Testing already states for `--agents-md-smoke`.
- **Consumer.** `scripts/evidence-config.sh:30`,
  `EVIDENCE_KIT_RUN_index_tests='bash context-kit/bin/run-index-tests.sh'`, becomes
  `'bash gate-sdk/bin/run-gates.sh --run-index-tests'`. That is the arm's **named caller** and the
  transition where its verdict is read: every validate stage, through `--run-validate`. The suite
  name `index_tests` does not change, so `EVIDENCE_KIT_SUITES` is untouched.
- **Named reader of the verdict, with its red condition.** The `index_tests` suite reds the
  validate stage on a non-zero exit. That is a monotone verdict — a clean run says the goldens
  match and the meter refuses an unrecognized mode — so it is clearable by inspection.

**Existing state whose reader set this cut changes: the file path
`context-kit/bin/run-index-tests.sh`.** Surveyed across the whole component set with an
unsilenced tree-wide grep at HEAD `18ce331c` — not a hand-picked subset, and no `2>/dev/null` on
the probe. Every reader, with its disposition:

| reader | kind | disposition |
| --- | --- | --- |
| `scripts/evidence-config.sh:30` | live caller | rewritten to the arm (above) |
| `README.md:135` | governed roster, gated by `check-battery-roster` via `EVIDENCE_KIT_RUNNER_DOC` | rewritten to the arm; the gate is its red condition |
| `context-kit/README.md:87` | kit runner line | rewritten to the arm |
| `context-kit/SPEC.md:1127` | §Layout and configuration's tree listing | row deleted with the file |
| `context-kit/SPEC.md:1371` | §Testing's own prose naming the driver | rewritten to the arm |
| `gate-sdk/SPEC.md:15138` | §check-exec-bit's rationale, naming this file as an instance of a script that invokes kit scripts **by path** | see below — this is the cross-component target |
| `.claude/settings.json:22-23` | two literal Bash grants, gated by `check-settings-paths` | removed; a grant naming a deleted path is the `--scratch-run` precedent, a port that **removes** a grant rather than relocating one |
| `scripts/git-hooks/pre-commit` (3 baked copies) | **generated** projection of the knob set | regenerated, never hand-edited |
| `docs/context-kit/README.md`, `docs/context-kit/SPEC.md:1132,1376`, `docs/gate-sdk/SPEC.md:15143` | **generated** docs mirrors | regenerated |
| `TASK-QUEUE.md:146,150`, `.workflow/survey-record.md:15` | queue and survey records of this very finding | left as the dated record they are |

**The `gate-sdk/SPEC.md:15138` citation is the one cross-component consequence and it does not
simply delete.** §check-exec-bit names three instances of a script that invokes kit scripts *by
path*, and uses them to justify checking the git **index** mode of tracked `*.sh`. The compiled
arm still invokes `gate-sdk/bin/run-gates.sh` by path, so the *class* survives intact — what dies
is the file that was the example. The citation is re-pointed at a live path-invoker rather than
dropped, because dropping it would shrink the gate's stated ground to two instances on a cut that
did not narrow the ground at all.

**No new field, message or config knob is introduced by any delta**, so there is no field
without a named reader. Deltas 1 and 2 introduce no state at all: they are contract prose whose
consumer is the next session composing a port cut, and whose mechanical reader is the
`--emit port-blockers --tree` oracle, which already reports the member `owed`.

## Existing sections updated

- `context-kit/SPEC.md` §Testing — the sequencing sentence at :1382-1385 and its reach clause at
  :1386 (deltas 1 and 2); the closing owed-set sentence at :1394-1397 (delta 2); the driver
  sentence at :1371 and the runner's port disposition beside the AGENTS.md smoke's (delta 3).
- `context-kit/SPEC.md` §Layout and configuration — the `bin/run-index-tests.sh` row at :1127
  (delta 3).
- `context-kit/README.md` — the runner line at :87 (delta 3).
- `README.md` §This repo, governed — the per-kit fixture runner line at :135, which
  `check-battery-roster` holds against `EVIDENCE_KIT_SUITES` (delta 3).
- `scripts/evidence-config.sh` — `EVIDENCE_KIT_RUN_index_tests` at :30 (delta 3).
- `gate-sdk/SPEC.md` §The non-gate arm — the arm roster gains `--run-index-tests` with its
  section pointer, and the prose spawned-program record gains this arm's `bash` (delta 3).
- `gate-sdk/SPEC.md` §check-exec-bit — the by-path-invoker instance at :15138 (delta 3).
- `.claude/settings.json` — the two literal grants at :22-23, held by `check-settings-paths`
  (delta 3).
- `scripts/git-hooks/pre-commit` and the `docs/` mirrors — **generated**; regenerated from their
  own triggers, never hand-edited (docs/site-architecture.md §Generated projections and their
  freshness gates owns the fan-out and each freshness gate prints its own command on red)
  (delta 3).
- `TASK-QUEUE.md` — `context-kit-testing-hold-is-colocation-not-coupling` promotes to New
  Features with this file's `[spec:]` ref, in the same commit as this file (all deltas).

## Definition of Done

- [ ] **Causal completeness** — the arm has a named producer (the front-end's fall-through) and a
      named consumer (`EVIDENCE_KIT_RUN_index_tests`, read at every validate stage); no new field
      is introduced, so none lacks a reader.
- [ ] **The release is real, not described** — `bash gate-sdk/bin/run-gates.sh --emit
      port-blockers --tree` no longer has §Testing's prose holding `bin/run-index-tests.sh`, and
      the owed count falls by one when delta 3 lands.
- [ ] **Goldens byte-unchanged** — no file under `context-kit/index-tests/expected/` is edited by
      this cut; the shell form's stdout and exit status are captured at the commit before its
      deletion, across the bare arm and `--update`, and diffed byte-for-byte against the compiled
      arm's.
- [ ] **The three unheld sub-behaviours survive** — the consumer-shadowing case, the exit-2
      refusal case with its stream split, and `norm()`'s path rewrite each assert in the arm.
- [ ] **Every reader in the table above is dispositioned**, the generated ones by regeneration and
      the battery green (`bash gate-sdk/bin/run-gates.sh` plus
      `bash gate-sdk/bin/build-native.sh`, neither discharging the other).
- [ ] **Merged with no information lost** — each addition integrated into its proper canonical
      section, not appended.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component
      (`ls context-kit/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for `run-index-tests.sh`; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps found during the work filed to the gap inbox.
