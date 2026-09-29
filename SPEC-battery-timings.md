# SPEC amendment: battery-timings

The runner writes `<tmp-dir>/gate-timings.txt` from whatever it ran. A `--only` or `--for` run therefore overwrites the battery's timing file with a subset, and `kpi-gate-runtime` reports that subset's `TOTAL` as the full battery's runtime. No reader can tell the two apart, because the file carries no mark. On the authoring host the file holds `check-producer-liveness 0` and `TOTAL 0`, so the next drift report would read a zero-millisecond battery.

This amendment makes the file mean what its reader says it means: only an unfiltered run of the configured registry writes it.

One queue entry pairs it: [battery-timing-file-overwritten-by-only-run](TASK-QUEUE.md#battery-timing-file-overwritten-by-only-run).

**The rulings.**

- **A filtered run leaves the file alone, rather than marking it.** The entry proposed a mark the KPI reads and refuses to sum. A mark still destroys the last full reading, so every `--only` run between two batteries would turn the report's row into an `n/a`. Leaving the file untouched keeps the last full battery, whose reading age the KPI already prints. It also needs no new line in the file's grammar and no new reader branch.
- **Three runs are filtered.** A `--only` selection and a `--for` selection each run a subset. A positional gates dir other than the configured one runs another registry's battery. The runner already computes that last case as `explicit`, for its steer diagnostic.
- **The seam.** Runner mechanism only; no knob, no consumer config.

## What changes

### (1) Only an unfiltered run writes the timings file {mechanical}

**Not yet applied.** gate-sdk/SPEC.md §run-gates, first paragraph. The parenthesis *(`<tmp-dir>/gate-timings.txt`, `<gate> <elapsed-ms>` per line + `TOTAL` — uncommitted by design: a measurement, not state)* becomes:

*(`<tmp-dir>/gate-timings.txt`, `<gate> <elapsed-ms>` per line + `TOTAL` — uncommitted by design: a measurement, not state. Only an unfiltered run of the configured registry writes it. A `--only` or `--for` selection, or a gates-dir positional naming another registry, leaves the file as the last full battery wrote it, so its one reader never sums a subset.)*

Implementation: `native/src/runner.rs` calls `write_timings` only when `parsed.only` and `parsed.paths` are empty and `explicit` is false. The usage text's closing line becomes *Per-gate timings of an unfiltered run land in $GATE_SDK_TMP_DIR/gate-timings.txt (default .tmp/).*

### (2) The KPI's reading is the last whole battery {mechanical}

**Not yet applied.** drift-kit/SPEC.md §Bundled KPIs, `kpi-gate-runtime`. After *full-battery runtime from the runner's timings file (`<tmp-dir>/gate-timings.txt`)*, add: *, which only an unfiltered run writes (gate-sdk/SPEC.md §run-gates), so a `--only` run between batteries leaves the reading, and its age, as the last battery set them*. The member's code is unchanged.

### (3) The fixtures {mechanical}

**Not yet applied.** The write condition becomes one predicate in `runner.rs`, and a unit test beside the parser's tests pins it over the four cases: a bare run writes; `--only`, `--for` and an explicit gates dir do not. gate-sdk/smoke/install.sh's `--only` block, which already runs `--only check-smoke-pass`, seeds the scratch consumer's timings file before that run and asserts it byte-identical after.

## Producers and consumers

Probes: `git grep -n 'gate-timings\|TIMINGS_FILE'` over the tracked tree, less the queue and the generated mirrors; `grep -n 'write_timings\|explicit' native/src/runner.rs`; `grep -n 'dispatch_one\|write_timings' native/src/emit/git_hook.rs`, which confirms the hook arm writes no timings file; `grep -n -- '--only' gate-sdk/smoke/install.sh`.

- **The timings file.** Producer: `runner::run`'s unfiltered path (delta 1). Its one parsing reader is `kpi-gate-runtime`, through `DRIFT_KIT_TIMINGS_FILE` (delta 2). The drift-kit smoke points that knob at an absent path, so it reads no runner output. No gate reads the file.
- **Narrowing.** The file's corpus narrows from every run to unfiltered runs, so each reader's red condition is named. `kpi-gate-runtime` has no red: its degrades are `n/a (no timings file — run the battery)` and `n/a (no TOTAL line)`. A tree that has only ever run `--only` now shows the first of these rather than a subset total, and that is the intended reading.

## Existing sections updated

Roster probe: the `git grep` above.

- `gate-sdk/SPEC.md` §run-gates (delta 1).
- `native/src/runner.rs` (deltas 1 and 3).
- `gate-sdk/bin/run-gates.sh` — its header comment *timings → $GATE_SDK_TMP_DIR/gate-timings.txt (default .tmp/); a measurement, never committed* gains *, written by an unfiltered run* (delta 1).
- `drift-kit/SPEC.md` §Bundled KPIs (delta 2).
- `gate-sdk/smoke/install.sh` (delta 3).
- `.workflow/surface-ceiling.txt` — the grown `gate-sdk/SPEC.md` and `drift-kit/SPEC.md` rows re-stamped with `bash gate-sdk/bin/run-gates.sh --emit always-loaded --ceiling`, which `check-surface-ratchet` demands with the growth (deltas 1 and 2).
- `docs/gate-sdk/SPEC.md` and `docs/drift-kit/SPEC.md`, regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` (all deltas).

## Retired spellings

- None — the deltas narrow when an existing file is written; no name, path or token is retired.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical text it refines rather than appending to it.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the root (`ls SPEC-*.md`).
- [ ] **Removals propagated** — `## Retired spellings` above holds, checked by `check-amendment-retired-spelling`.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks.
- [ ] **Witnessed on this tree** — after a full `--run`, a `--only check-producer-liveness` run leaves `.tmp/gate-timings.txt` byte-identical, and `--emit drift-report` reads the full battery's total. The entry moves to Done before the drain stage (`LIFECYCLE_KIT_DRAIN_STAGE`).
