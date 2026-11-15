# SPEC amendment: test-control-binding

Pairs `test-control-decl-binding`. A control line binds its witness to the run it witnessed (`commit=`, `cmd=`) and to the declaration's suite and scenario, but not to the declaration's assignments. So a control whose `VAR=value` is edited after its red witness, re-pointed at another switch or at none that matters, still satisfies `check-test-control` (C) at close. Per the entry's fix shape, this amendment **binds the declaration rather than stating the limit**: a digest of the control's assignments rides the control line, and (C) holds it equal to the digest of the declaration as it stands.

**What the tree does today**, read from `native/src/gates/test_control.rs`, `native/src/evidence.rs`, `native/src/emit/run_validate.rs` and `native/src/gates/evidence_manifest.rs`:

- **(C) never reads the assignments.** `close_witness` matches a control line by `control=` and its suite field, then checks `scenario=`, `witnessed=red`, the date window, and `commit=` and `cmd=` against the suite line it follows. `Control.env` is read by the spine's spawn alone.
- **The command digest covers the suite's run and parser words alone** (`command_digest`), so a control's assignments reach no digest on any line.
- **The control line is exactly eight fields.** `evidence_manifest.rs`' `control_line` reds any other count as malformed, and `ControlLine::render` writes eight.
- **The control line has no released writer.** The last release tag carries the v1 manifest (`git show v0.36.0:evidence-kit/SPEC.md` names `evidence-manifest v1`, and test controls landed after it), so no adopter tree holds a v2 manifest, let alone an eight-field control line.

## What changes

### (1) The assignments digest rides the control line {mechanical} {user-facing: operator direction 2026-11-15, lead-relayed (not a ruling), on the paired entry: a digest of the control's assignments rides the control line}

The control line gains a ninth field, `assign=<assignments-digest>`, appended after `cmd=` so the eight existing fields keep their positions. **Not yet applied.**

- **The assignments digest** is the SHA-256, 64 lowercase hex characters, of the declaration's assignments in declared order, each `<VAR>=<value>` followed by a newline. One function in `native/src/evidence.rs` computes it from a parsed `Control`, so the spine and `check-test-control` (C) cannot hash two inputs. The parser has already word-split the declaration, so whitespace between assignments does not reach the digest, and any edit to an assignment, its order included, changes it.
- **The writer.** `--run-validate` writes `assign=` on each control line from the declaration it ran the control under.
- **The grammar.** `check-evidence-manifest` (B) holds the control line at nine fields, `assign=` a 64-hex digest, each bad field its own finding as for the other eight.
- **No migration.** The control line and the v2 header ship in one release, so the field changes the grammar in place: no header move, no rewrite at the fold. A working tree built from an unreleased revision that holds an eight-field line reds (B) until its suite's next run supersedes it (§Evidence manifest's supersession rule), and the boundary truncation clears it at the next iteration in any case.

Replacement text for §Evidence manifest's control-line bullet. **Not yet applied:**

> - **A control line is nine fields:** `<run-key> <suite> control=<name> scenario=<scenario> witnessed=<red|green|absent> <date> commit=<hex|-> cmd=<command-digest> assign=<assignments-digest>`, one per test control the run witnessed (§Test controls). `commit=` and `cmd=` repeat the values on the suite line it follows and `assign=` digests the assignments the control ran under, so the line says which run of which command it witnessed under which declaration. It sits directly after its suite's line, in `EVIDENCE_KIT_CONTROLS` order, and a control line with no suite line of its key and suite above it is malformed.

Replacement text for §Test controls' witness paragraph. **Not yet applied:**

> **The witness is the scenario's observed status**, the last observation winning: `fail` is `red`, `pass` is `green`, and an `ignore` or an unobserved scenario is `absent`. The control line (§Evidence manifest) records it after the suite's line, with the **assignments digest**: the SHA-256 of the declaration's assignments in declared order, each `<VAR>=<value>` newline-terminated, one function in `native/src/evidence.rs` serving the spine and `check-test-control` (C).

### (2) (C) holds the witness to the current declaration {mechanical} {user-facing: operator direction 2026-11-15, lead-relayed (not a ruling), on the paired entry: (C) holds the control line's digest equal to the current declaration's}

`check-test-control` (C) adds one condition: the witnessing control line's `assign=` equals the assignments digest of the control's declaration as the gate reads it now. A mismatch is a finding naming the control, `close entry: control '<name>' was witnessed under other assignments than its declaration's — re-run --run-validate`, and leaves the other (C) findings for that control standing. So an assignment edited after its witness reds the close entry until a run witnesses the edited declaration. **Not yet applied.**

Replacement text for §check-test-control's (C) bullet. **Not yet applied:**

> - **(C) close-entry witness** — a `close` cursor requires, for every declared control, a current-key control line with `witnessed=red` whose `scenario=` and `assign=` are the declaration's and whose `commit=` and `cmd=` equal the line of the suite it follows. Its date window is §check-evidence-manifest (A)'s. An empty cursor disarms (C), as it disarms that gate's (A).

## Producers and consumers

- **The assignments digest** (new interface) — producer: the one digest function in `native/src/evidence.rs`, called by `--run-validate` per control it runs, under the consumer's `EVIDENCE_KIT_CONTROLS` declaration, the config that arms the whole feature. Consumers: the control line's writer, and `check-test-control` (C), which recomputes it from the declaration parsed by the shared parser.
- **The `assign=` field** (new field on an existing line) — producer: `--run-validate`'s control-line writer. Readers: `check-evidence-manifest` (B) for its shape at every commit, and `check-test-control` (C) for its value at the close-entry transition. No other reader exists: the trajectory extractor skips the line by its third field and the field spells no `fail=`, the fold reads a line's key and suite alone, and `--import-gate-run` writes no control line (probe: `git grep -n "control="` and `git grep -n -i "control line"` over the tracked tree).
- **The new (C) finding** — producer: `check-test-control` at a close cursor; consumer: the committing session through the output contract, on the pre-commit hook, `run-gates.sh` and CI.
- **Point 5 (narrowing).** No corpus narrows. (B) of `check-evidence-manifest` widens the control-line shape and reds an eight-field line, the case delta 1's no-migration bullet answers. (C) adds a red condition, `assign=` unequal, and keeps its others.
- **Point 6 (every member).** The members obliged are the declared controls of a tree. This repo declares none (`git grep -n "EVIDENCE_KIT_CONTROL" -- '*.knobs'` hits only the template and the `check-test-control` fixtures). The fixtures' one control, `nullguard` (`unit t_null BREAK_NULL=1`), takes the digest of `BREAK_NULL=1` and a newline in both `good/` and `bad/`.

## Existing sections updated

Roster from `git grep -n "control="`, `git grep -n -i "control line"` and `git grep -n -e "eight-field control line" -e "control line is eight fields" -e "control line's eight fields"` over the tracked tree.

- `evidence-kit/SPEC.md` §Evidence manifest's control-line bullet, §Test controls' witness paragraph (delta 1); §check-evidence-manifest's (B) bullet, *eight-field control line* re-phrased to the nine-field one (delta 1); §check-test-control's (C) bullet and its test-coverage sentence, which gains the other-assignments case (delta 2); §Producers and consumers' control-line bullet, `assign=` against the current declaration's digest beside the fields (C) already reads, and its control-declaration bullet, which gains the spine's digest (deltas 1 and 2).
- `docs/evidence-kit/SPEC.md` — the generated mirror, regenerated by `--emit docs-mirror --write` (all deltas).
- `native/src/evidence.rs` — the digest function, `ControlLine`'s `assign` field and its render, and a unit case pinning the digest of a two-assignment declaration (delta 1).
- `native/src/emit/run_validate.rs` — the writer fills `assign` from the control it ran (delta 1). Its fold tests' literal control lines read key and suite alone and need no change.
- `native/src/gates/evidence_manifest.rs` — `control_line` at nine fields, `CONTROL_SHAPE`, the `assign=` check, the comments naming eight fields, and the unit case's well-formed line plus a bad-`assign=` variant (delta 1).
- `evidence-kit/gate-tests/check-evidence-manifest.test.sh` — `CTLLINE` gains `assign=`, and an eight-field control line reds as malformed (delta 1).
- `evidence-kit/gate-tests/test-control-run.test.sh` — the recorded control line carries `assign=` with the declaration's digest (delta 1).
- `native/src/gates/test_control.rs` — (C)'s `assign=` check against the declaration's digest, and the unit case's control line gains the field (delta 2).
- `evidence-kit/gate-tests/check-test-control/good/validate-evidence.txt` — its control line gains `assign=` with `nullguard`'s digest (deltas 1 and 2).
- `evidence-kit/gate-tests/check-test-control/bad/validate-evidence.txt` — its control line gains the same digest, so `bad/`'s `expect.txt` is unchanged (deltas 1 and 2).
- `evidence-kit/gate-tests/check-test-control.test.sh` — case 8's witness gains the field, plus a `close-other-assignments` case: the knob file declares `nullguard` with another value, and (C) reds naming it (delta 2).
- `.workflow/release-declarations.md` — the *New and tightened gates* `check-test-control` bullet: at a close cursor it also reds a control witnessed under assignments its declaration no longer carries; and the *Behavior changes* test-controls bullet: the control line records the assignments it ran under (deltas 1 and 2).

## Retired spellings

- `eight-field control line` — the control line is nine fields (delta 1).
- `control line is eight fields` — §Evidence manifest's bullet re-phrased to nine (delta 1).
- `control line's eight fields` — `evidence_manifest.rs`' grammar comments (delta 1).

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for the digest, the field and the finding.
- [ ] **Instruction surfaces: instruction only** — no template, agent definition or shim is touched.
- [ ] **Merged with no information lost** — each replacement re-phrases the passage it refines; the merged spec reads as one document.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls evidence-kit/SPEC-*.md`).
- [ ] **Removals propagated** — the three spellings above survive nowhere `check-amendment-retired-spelling` reds.
- [ ] **Entry moved** — `test-control-decl-binding` to Done with `--queue done` in the build batch's merge commit, before the drain stage; its oracle is local (the evidence-kit fixture suite and the battery), so no remote run gates the move.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks.
