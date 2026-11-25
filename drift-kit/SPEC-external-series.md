# SPEC amendment: external-series

Pairs `external-kpi-series`. A series an outside tool holds, such as a velocity, a lesson count or a queue age, joins the drift report as a data file, `kpi-<name>.series`, dropped in a KPI dir and named in the registry like any plugin. The binary reads it instead of running it, validates it, and renders its latest point with a mark naming its source and its age. The kit ships the form and its reader. The series and the tool that writes it are consumer content.

**What the tree does today**, each bullet with the probe that settled it:

- A registry name resolves, within a KPI dir, to `<name>.sh` and then `<name>.ps1`, and to nothing else (`grep -n 'format!("{}/{}' native/src/emit/drift_report.rs`).
- A row is a label and a value, rendered with no field telling a consumer plugin's row from a built-in's (`grep -n -A5 'fn render_rows' native/src/emit/drift_report.rs`).
- An outside series reaches the report today only through a plugin someone writes to read it, a `.sh` that a native-Windows host does not start or a `.ps1` that needs `GATE_SDK_POWERSHELL` set (drift-kit/SPEC.md §The extensibility contract).
- The bundled members read a date in the operator's civil zone through one in-process reader (`grep -n 'fn civil_days\|fn today_iso' native/src/emit/kpi/mod.rs`).

## What changes

### (1) The series form: a data member, its grammar and its validation {design-bearing} {user-facing: the entry's deliverable — the input form an outside series takes and its validation}

**Not yet applied** — a new section `## The external series form` in drift-kit/SPEC.md, after §The KPI plugin contract:

> ## The external series form
>
> A series an outside tool holds joins the report as **data, never code**: a file `kpi-<name>.series`, resolved through the registry like a plugin (§The extensibility contract) and read by the binary. A tool writing it needs no shell, no PowerShell host and no knowledge of the plugin environment, so one file works on every host.
>
> The file is UTF-8 text, one line per entry, `#` comments and blank lines ignored. Header lines come first, each `<field>: <value>`:
>
> - `label:` — required, the row's label.
> - `section:` — required, `lead` or `lag`, the honesty label the outside tool claims for its series.
> - `source:` — required, the name of the tool or record the series comes from, shown on the row.
> - `unit:` — optional, printed after each value.
> - `trend:` — optional, a key of letters, digits and `-`. When present the member volunteers the `--trend` fragment `<key> <latest>`.
>
> Then one point per line, `<YYYY-MM-DD><TAB><value>`, the value an optional `-`, digits, and an optional `.` with digits. At least one point is owed, and the dates are strictly increasing.
>
> ```
> label: velocity
> section: lead
> source: sprint-board export
> unit: pts
> trend: vel
> 2026-11-01	21
> 2026-11-15	18.5
> ```
>
> **Validation fails visible, never closed.** A file breaking the grammar emits one row in the lead section, labelled with the member's name, valued `n/a (series line <n>: <defect>)`. The defects are a missing required field, an unknown or repeated field, a `section:` other than `lead` or `lag`, a `trend:` key outside its alphabet, a header after a point, a point that is not a date, a tab and a value, dates not strictly increasing, and no point. The report still exits 0. A malformed series is refused whole and never partly rendered, since a row built from the points that parsed would show a trend the source does not hold.
>
> **Where the file lives.** The KPI dirs are the consumer's gates dir by default, which is tracked. A series the outside tool rewrites on its own schedule belongs in a dir the consumer gitignores, named in `DRIFT_KIT_KPI_DIRS`, so the tool's writes never dirty the tree.

**Not yet applied** — §The extensibility contract's tier 1 reads "for a file named `<name>.sh`, then `<name>.ps1`, then `<name>.series`", tier 2 likewise. The shadowing sentence names `kpi-task-split.series` beside the other two spellings, since a series shadows a built-in as a plugin does, and the paragraph after the list gains, after "within one dir the `.sh` file wins": "and a `.ps1` file wins over a `.series` file". §The KPI plugin contract's opening sentence re-phrases to "A plugin is `kpi-<name>.sh` or `kpi-<name>.ps1`; a series file is a data member read under §The external series form, never invoked."

- **Why a file in the KPI dir, never a second registry.** The extensibility contract is drift-kit's promise to adopters: drop a member in a KPI dir and name it in `kpis.list`. A series joining through the same resolution keeps one registry, one place to prune a row, and shadowing as it already works. A separate knob listing series files would be a second registry the report reads in a second order.
- **Why a line format, never JSON.** The plugin row and the price table are line-and-tab already (§The KPI plugin contract, §The stage-economics meter input 3). Any tool can write lines without a serializer, and a named-field header is versioned by its names: a field the reader does not know is a refusal, so a tool written for a later grammar fails visibly rather than misread.
- **Why the outside tool declares its section.** Whether a series leads or lags is a claim about how it is measured, which only the tool that measures it knows. The report prints the claim and its source together, so a reader can weigh it.

### (2) The row and its provenance mark {design-bearing} {user-facing: the entry's deliverable — how the report marks an outside series' provenance}

**Not yet applied** — appended to the new §The external series form:

> **The row.** A valid series emits one row in its declared section: its label, and the value `<latest>[ <unit>] (<±delta> since <previous date>; external: <source>, as of <latest date>, <age>d old)`. The delta is printed with the larger fraction precision of its two operands. A one-point series has no delta, so its value opens `<latest>[ <unit>] (external: …)`. The age counts whole civil days from the latest date to today, in the operator's civil zone (§Bundled KPIs).
>
> **The mark is the provenance.** `external: <source>` tells a reader the report did not compute the number and names who did, and `as of <date>, <age>d old` is the reading-age caveat §The KPI plugin contract owes a past measurement. A source that stopped writing shows as a growing age, never as a stale value read as current.

- **Why the latest point and one delta.** The report reads trend, not level (the opening of this SPEC), and the trend across sessions is what the session-start line carries. One delta is the series' own slope at its latest point. A fuller chart is the statistics view's to add if a consumer needs one.

### (3) The README sentence, the smoke and the unit tests {mechanical}

**Not yet applied** — drift-kit/README.md's plugin paragraph gains, after "on native Windows, write it in PowerShell.":

> A series an outside tool keeps joins as data instead: a `kpi-<name>.series` file in a KPI dir, named in the registry, its grammar in [drift-kit/SPEC.md §The external series form](SPEC.md#the-external-series-form).

**Not yet applied** — drift-kit/SPEC.md §Testing gains a bullet:

> - **The external series form** runs in the smoke through the consumer KPI dir the extension-point assertions already configure. A valid two-point series renders its row in its declared section with the delta, the `external:` mark and the age, and its `trend:` key's fragment. A one-point series renders no delta. A malformed series renders its `n/a (series line <n>: …)` row in the lead section and the report exits 0. A `.ps1` beside a `.series` of the same name wins the resolution, and a `.sh` beside it wins too. The grammar's defects, each by class, and the delta's precision are unit tests of the series module.

## Producers and consumers

- **The series file.** Producer: an outside tool, or a consumer exporting from one, writing into a KPI dir. This repo deploys none: its trend sources are in-tree members. The smoke's fixture series is the exercised producer, and `drift-kit/SPEC.md §Testing` names it. Consumer: the resolver in `native/src/emit/drift_report.rs`, through a new `Resolved` variant, and a new reader module under `native/src/emit/kpi/`.
- **`label:`, `section:`, `source:`, `unit:`.** Consumer: the row renderer, the series' section choosing which labelled block the row joins. `source:` is the provenance mark's only input.
- **`trend:`.** Consumer: the `--trend` collation, which joins the fragment into the session-start line through `CONTEXT_KIT_DRIFT_REPORT` as any member's.
- **The points.** Consumer: the latest-value, delta and age derivation of delta 2.
- **The defect row.** Consumer: the report reader, who learns the series is broken from the row rather than its absence.
- **Point 2, roster-holding readers.** The new extension lands on the resolution order, whose rosters are §The extensibility contract, the README's plugin paragraph and `drift_report.rs`' resolution unit test, each named above. No knob is added, so no knob roster moves. `templates/kpis.list` names bundled members only, and a series is never bundled, so `check-template-registry-parity` is untouched.
- **Point 5, narrowing.** None. A dir holding a `.sh` or a `.ps1` resolves as before, since both outrank the new form.
- **Point 6, every member.** Every series file a registry name resolves to is read, and every one emits exactly one row: a valid one its value, a broken one its defect.

## Existing sections updated

Roster probe: `git grep -n 'ps1' -- drift-kit/ native/src/emit/drift_report.rs` and `git grep -n 'kpi-<name>' -- '*.md' ':!docs/*'` over the tracked tree.

- drift-kit/SPEC.md — the new §The external series form (deltas 1 and 2), §The extensibility contract's tier lines and in-dir order, and §The KPI plugin contract's opening sentence (delta 1), and §Testing's bullet (delta 3).
- drift-kit/README.md — the plugin paragraph's sentence (delta 3).
- native/src/emit/drift_report.rs — the `Resolved::Series` variant, `in_dir`'s third probe and its resolution unit case (delta 1).
- native/src/emit/kpi/series.rs — new module: the grammar, the defects, the row and the delta, with their unit tests (deltas 1 and 2). native/src/emit/kpi/mod.rs gains its `mod` line.
- drift-kit/smoke/install.sh — the series assertions (delta 3).
- .workflow/release-declarations.md — *Behavior changes* gains the series form (deltas 1 and 2).
- docs/drift-kit/SPEC.md, docs/drift-kit/README.md, docs/llms-full.txt, docs/requirements.md's install-size block and .workflow/surface-ceiling.txt — generated, each regenerated by the command its freshness gate prints on red (all deltas).

## Retired spellings

- None — the form adds a resolution step and a section, and retires no name.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for the file form, its five fields, the points, the row and the defect row.
- [ ] **Instruction surfaces: instruction only** — the README sentence states the form and points at its section, with no grounds.
- [ ] **Merged with no information lost** — the new section lands whole, and the extensibility and plugin-contract passages are re-phrased, not appended to.
- [ ] **Amendment deleted** — this file removed on merge (`ls drift-kit/SPEC-*.md`).
- [ ] **Removals propagated** — `## Retired spellings` holds.
- [ ] **Entry moved** — `external-kpi-series` moves to Done before the drain stage, in the commit that lands the merge.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks.
