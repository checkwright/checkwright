# SPEC amendment: baseline-claims

**Governed prose quotes what the validate baseline holds, and nothing checks it against the file.** A sentence that names the baseline file and quotes one of its `<suite> <scenario> <status>` rows is a transcribed machine value. It goes stale the day the row is promoted or retired. gate-sdk/SPEC.md once claimed a held `fail` row after the row had been earned out to `pass`, and a reader pricing a criterion from the prose read a pointer to a mechanism that no longer existed.

**The mechanism already exists, so the unit adds an oracle and a class, not a gate.** canon-kit/SPEC.md §Content tiering names two gates as the remedy for a transcribed value: `check-measured-claim` binds a marked literal to an oracle the gate re-runs, and `check-unmarked-claim` pressures a declared class of claim into carrying a marker. The baseline row fits both. The value may be a status rather than a cardinal, which arm C exempts, so arms A and B do the work. The missing piece is the oracle. No kit arm prints a baseline row in the `<key>`⇥`<value>` shape the measured-claims adapter reads, so this unit adds that arm to evidence-kit, which owns the file. It also declares this repository's class. A new canon-kit gate reading evidence-kit's file was refused: it would be a second mechanism for the class those two gates already hold, and a canon-kit member reading another kit's knob.

**The past-tense ruling.** A sentence deliberately recording a *retired* row falls in the class, because a class matches what a sentence asserts and not its tense. That sentence cannot carry a marker: the oracle would contradict it (arm A) or no longer emit its key (arm B). So it takes `check-unmarked-claim`'s valve with its reason, `<!-- unmarked-claim-exempt: records the retired <row> -->`. That valve is the convention. A tense predicate is refused because tense is not decidable, which is §check-amendment-queue arm (e)'s ground for its own refusal.

**Measured at authoring.**

- `.workflow/validate-baseline.txt` holds three-field rows, `<suite> <scenario> <status>`, with an optional blocking slug. The file's `# contract:` header and evidence-kit/SPEC.md §Baseline manifest both say so.
- Its suites and scenarios carry underscores (`gate_sdk`, `installer_smoke`), which the adapter's slug-shaped key refuses (`is_slug` in `native/src/spec.rs`).
- The one quoted row in governed prose is gate-sdk/SPEC.md §upgrade-smoke's "`upgrade upgrade pass` row", and it is true today. `git grep -n 'validate-baseline'` over the tracked tree finds no sentence recording a retired row.
- This repository's `CANON_KIT_MEASURED_SURFACE_GLOBS` covers the manifest set, the binding shims and TASK-QUEUE.md, so the class reaches every surface a row is quoted on.

## What changes

### (1) evidence-kit's baseline-claims arm

`native/src/emit/baseline_claims.rs`, registered in `native/src/emit/mod.rs` as `--emit-baseline-claims` {design-bearing}.

It is a non-gate arm (gate-sdk/SPEC.md §The non-gate arm) declaring `EVIDENCE_KIT_BASELINE_FILE` and nothing from canon-kit. That keeps the nested-arm rule of canon-kit/SPEC.md §The shared spec adapters: a command knob's arm never declares the knob that names it.

For each data line of the baseline (`evidence::data_lines`), it prints `<key>`⇥`<status>`. The key is `baseline-<suite>--<scenario>`, each part lowercased with every character outside `[a-z0-9]` turned to `-`, which is always slug-shaped. So `installer_smoke build pass` becomes `baseline-installer-smoke--build`⇥`pass`. A blocking slug and `reproduces-at=` are not printed, because a status is what a prose claim quotes. Lines follow the file's order.

**Exit 2** comes from:

- an unreadable baseline file;
- a data line with fewer than three fields;
- two rows mapping to one key, with both named, since the adapter would refuse the repeat anyway and a collision named at its source is the useful message.

Unit tests cover the mapping, header and comment skipping, the collision and a short row.

### (2) evidence-kit/SPEC.md states the arm and the binding

A new section after §bin/diff-baseline.sh {mechanical}. **Not yet applied.**

> ### The baseline-claims arm
>
> `--emit baseline-claims` prints each baseline row as a `measured:` oracle line, `baseline-<suite>--<scenario>`⇥`<status>`, each part lowercased with every character outside `[a-z0-9]` turned to a hyphen so the key is slug-shaped. A consumer whose governed prose quotes a row names the arm in `CANON_KIT_MEASURED_CLAIMS_CMD`, or appends its output to its own emitter, and binds each quoting sentence with `<!-- measured: <key>=<status> -->`. canon-kit's `check-measured-claim` then reds the sentence the day the row moves (canon-kit/SPEC.md §check-measured-claim). A claim class under `check-unmarked-claim` makes the marker owed rather than voluntary. A sentence recording a *retired* row cannot carry a marker, since the oracle contradicts it or no longer emits its key, so it takes that gate's `unmarked-claim-exempt:` valve naming the retired row. The status is the whole value: a blocking slug is the row's bookkeeping and not what prose quotes. Two rows mapping to one key, an unreadable file or a row short of three fields exit 2. The arm reads the baseline alone and declares no canon-kit knob, so naming it in a canon-kit command knob cannot recurse.

§Baseline manifest's paragraph "**A row is a claim about one scenario …**" gains a closing sentence: "Prose quoting a row is a second copy of that claim, bound to the row through §The baseline-claims arm."

### (3) This repository wires the oracle and declares the class

`scripts/measured-claims.sh` {mechanical}: after its three `printf` lines, it appends the arm's output, `bash gate-sdk/bin/run-gates.sh --emit baseline-claims`, failing the script when the arm fails. The `# no-port:` cause stands, since the script still carries this repository's own keys.

`scripts/claim-classes.sh` {mechanical}: a second class, `baseline-row`, whose ERE matches a normalized paragraph naming `validate-baseline.txt` and quoting a backticked `<suite> <scenario> <status>` triple in either order:

```
validate-baseline\.txt[^.]*`[a-z0-9_-]+ [a-z0-9_.-]+ (pass|fail|ignore)[ `]|`[a-z0-9_-]+ [a-z0-9_.-]+ (pass|fail|ignore)[ `][^.]*validate-baseline\.txt
```

Its `comment-tier-exempt:` line says the class is this repository's because the file name is this repository's layout, and that a sentence recording a retired row takes the valve.

### (4) The one live quotation leaves the kit SPEC

gate-sdk/SPEC.md §upgrade-smoke, "**Its callers are two.**" {mechanical}. **Not yet applied.** The clause "against `.workflow/validate-baseline.txt`'s `upgrade upgrade pass` row" becomes "against the `upgrade` suite's baseline row (evidence-kit/SPEC.md §Baseline manifest)".

A row of this repository's baseline is this repository's content, which gate-sdk/SPEC.md §The provenance seam keeps out of a kit SPEC. Rewriting the claim out of the class is also `check-unmarked-claim`'s first remedy. Build runs the gate before the rewrite and sees this sentence red under delta 3's class, which is the class's live proof. After the rewrite it is clean.

### (5) The release declaration

`.workflow/release-declarations.md` {mechanical}, under Behavior changes:

> - **evidence-kit `--emit baseline-claims`** — a new arm prints each validate-baseline row as a `measured:` oracle line, `baseline-<suite>--<scenario>`⇥`<status>`. Nothing to do; to hold prose that quotes a baseline row, name it in `CANON_KIT_MEASURED_CLAIMS_CMD` (or append its output to your emitter) and mark each quoting sentence.

## Producers and consumers

- **The oracle lines.**
  - Producer: the arm, spawned once per process by the `measured_claims` adapter through this repository's `scripts/measured-claims.sh`.
  - Enabling config: `CANON_KIT_MEASURED_CLAIMS_CMD`, which this repository already sets.
  - Consumers: `check-measured-claim`'s arms A and B, which read the key and the status at each marker. Every field the arm prints has that reader, and the slug and the flip token are not printed because nothing reads them.
- **The class.**
  - Producer: `scripts/claim-classes.sh` through `CANON_KIT_CLAIM_CLASSES_CMD`.
  - Consumer: `check-unmarked-claim`'s arm A over `CANON_KIT_MEASURED_SURFACE_GLOBS`.
  - A paragraph yields at most one finding, at the first class in roster order, so `gate-substrates` keeps its precedence.
- **Roster-holding readers of the new arm name.**
  - The arm table in `native/src/emit/mod.rs` is the one roster reachability reads (gate-sdk/SPEC.md §The non-gate arm).
  - The front ends reach any table member through the generic `--emit <name>` spelling, so neither front end changes.
- **Readers of the baseline file.** Unchanged: the arm only reads, and §Baseline manifest's "tooling never writes it" holds.

A scratch probe applied delta 3's ERE to the lowercased tracked markdown less fixtures and mirrors. It matched gate-sdk/SPEC.md's sentence and, outside the measured surface, one line of the pendency-contradiction amendment. `check-unmarked-claim` is the oracle once delta 3 registers the class.

## Existing sections updated

- `native/src/emit/baseline_claims.rs` and `native/src/emit/mod.rs`: the arm (delta 1).
- `evidence-kit/SPEC.md`: the new §The baseline-claims arm and §Baseline manifest (delta 2).
- `scripts/measured-claims.sh` and `scripts/claim-classes.sh` (delta 3).
- `gate-sdk/SPEC.md` §upgrade-smoke (delta 4).
- `docs/evidence-kit/SPEC.md` and `docs/gate-sdk/SPEC.md`: the generated mirrors, regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` (deltas 2 and 4).
- `.workflow/prose-bound-ceiling.txt`: the `evidence-kit/SPEC.md` and `gate-sdk/SPEC.md` rows, re-stamped to the counts `check-prose-bounds` prints if deltas 2 and 4 move them (deltas 2 and 4).
- `.workflow/release-declarations.md` (delta 5).

The roster came from `git grep -n 'validate-baseline'` over the tracked tree, `grep -n 'printf' scripts/measured-claims.sh scripts/claim-classes.sh`, and `grep -n '"--diff-baseline"' native/src/emit/mod.rs` for the arm table.

## Retired spellings

- None — the amendment adds an arm and a class and rewrites one clause, renaming nothing.

## Definition of Done

- [ ] **Causal completeness**: every point of canon-kit/SPEC.md §The causal-completeness check holds for the arm, its lines and the class.
- [ ] **Instruction surfaces: instruction only**: the scripts' comment lines state the class and its valve, with the grounds in evidence-kit/SPEC.md.
- [ ] **Merged with no information lost**: the new section carries the past-tense ruling and the refused new gate.
- [ ] **Amendment deleted**: this file is removed on merge, and `ls SPEC-*.md` at the root lists no file of this amendment.
- [ ] **Removals propagated**: nothing retired.
- [ ] **Gaps filed**: a cross-component gap found during the work is filed with `--emit file-gap`.
- [ ] **The entry moves**: `baseline-row-prose-coupling-gate` moves to Done in the landing commit, before the drain stage.
