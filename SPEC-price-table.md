# SPEC amendment: price-table

The stage-economics price table fails silently in two ways, and this amendment closes both.

**An unpriced model id reds nothing.** `kpi-price-table-age` reads the table's two dating headers and never a row, so a table dated current reads healthy while a model that is actually running has no row, and the meter prices that model's cells `n/a` as specified. The authoring probe found a live case. Over the last seven days of this repo's transcripts, 95 assistant turns ran on `claude-haiku-4-5-20251001`. The table carries `claude-haiku-4-5`, an id no transcript in the sessions dir has ever named, and 14 logged rows price at `cost=n/a` for it.

**The table has no time axis.** It carries one row per model id, so every token is priced at the row's current rate whatever date its session ran. After a table edit, a re-run reprices every row whose transcript survives, while a row whose transcript aged out keeps its first rate, and one log then mixes two price regimes unmarked.

Two queue entries pair it: [price-table-roster-coverage-oracle](TASK-QUEUE.md#price-table-roster-coverage-oracle) and [price-table-effective-dating](TASK-QUEUE.md#price-table-effective-dating). One amendment serves both because a coverage check has to know which row is in force, and that is the effective-dating grammar's to say.

**The rulings.**

- **One arm, two halves, read at the lead's first step.** `--price-coverage` reports the model ids in recent transcripts that the table does not price, and, where a consumer opts in, whether the published pricing page's price section changed since the table was transcribed. The lead template's first step runs it, and an unpriced id is the harness roster churn the template's *re-judge every assignment* rule turns on. Its reader is a session, so it is advisory and exits 0 like the meters.
- **Not a KPI row.** `kpi-price-table-age` stays header-only. A coverage row would scan hundreds of megabytes of transcripts inside every drift report for a reading the lead already takes before the first dispatch.
- **The page probe is off by default, and the fetch is the consumer's own command.** It is network at session start, so it ships off (Policy-as-choice). The fetch is `DRIFT_KIT_PRICE_PAGE_CMD`, a command knob the consumer fills, so no URL, no provider and no fetch program enters the kit, and no program joins the binary's roster or an adopter's floor. The section is taken by a heading line the consumer names. The authoring probe found the provider's pricing page served as markdown, with the model table under one heading. The hash is `git hash-object`, the floor program, rather than a new dependency.
- **Effective-from is an optional sixth column.** A five-column table reads exactly as before, and a price change becomes a second row for the same id with a later date. The meter prices a log row at the row in force on that log row's stamp date, which the trend log already derives from stamps alone. A re-run then reprices nothing whose rate has not changed, and a new rate reaches only the sessions on or after its date.
- **A zero-token cell prices at zero.** A turn the harness writes under a placeholder id carries zero usage. At present its cell degrades to `n/a` and raises the incomplete-pricing caveat on every run, which is 98 logged rows here. No tokens cost nothing at any rate, and the coverage arm counts only ids with usage, so the meter and the arm must agree on what an unpriced id is.
- **Fast mode stays the stated undercount.** The speed field is not a date axis, so effective dating does not carry it; the consumer table's known-undercount paragraph stands, reworded for the new grammar.
- **The seam.** The arm, its knobs, the column and the selection rule are kit mechanism. The table's rows, the fetch command, the section heading and the stored hash are consumer config. No kit literal names a model id, a provider or a URL.

## What changes

### (1) The price-coverage arm: the roster half {design-bearing}

**Not yet applied.** drift-kit/SPEC.md gains a section after §The stage-economics meter:

*## The price-coverage arm*

*`--price-coverage` answers, before a session tiers anything, whether the price table can price the models the harness is running. A new id is also the signal that the harness's model roster moved, which is why its first reader is the lead's first step (lifecycle-kit/templates/lead.md).*

*It reads every transcript under `DRIFT_KIT_SESSIONS_DIR` whose modification time falls within the last `DRIFT_KIT_PRICE_COVERAGE_DAYS` days, through the shared derivation's two-tier scan (§The overhead meter). It sums usage per model id with the meter's own reader, so the arm and the meter agree on what a model id is. An id counts only when some turn under it carried usage. An id is **priced** when `DRIFT_KIT_PRICE_TABLE` holds a row for it in force today (§The stage-economics meter, input 3).*

*It prints one head line, `price-coverage: <m> model id(s) with usage in <n> transcript(s) of the last <d>d`. It then prints either `  all priced` or `  unpriced: <id>[, <id>…] — no row in force today in <table>`, the ids in first-seen order. An absent table reads every id unpriced and says `no price table`. An empty window prints the head line with zeros. It is advisory and exits 0, and exits 2 on any operand. It is an `Arm::Run` table member because it declares consumer knobs and runs the second half below. Its knob roster is `DRIFT_KIT_PRICE_TABLE`, `DRIFT_KIT_SESSIONS_DIR`, `DRIFT_KIT_PRICE_COVERAGE_DAYS`, `DRIFT_KIT_PRICE_PAGE_CMD` and `DRIFT_KIT_PRICE_PAGE_SECTION`.*

***Named caller and transition**: the lead, at its first step, before its first dispatch is tiered; and any session judging a `cost=n/a` cell. **Honest limit:** the window is by modification time, so an id last used before it reads as absent rather than priced, and "in force today" misses an id whose only row starts after sessions that already ran on it, a table the meter will price `n/a` for those sessions.*

Implementation: a new module `native/src/emit/price_coverage.rs`, its `ARMS` row, and `usage_by_model` and `Prices` reused from `stage_economics.rs` rather than copied.

### (2) The price-coverage arm: the page half {design-bearing}

**Not yet applied.** The section delta 1 adds continues:

*The second half runs only where `DRIFT_KIT_PRICE_PAGE_CMD` is non-empty, and is off by default (network at session start). That command knob's argv is spawned directly, with no shell, and its stdout is the pricing page. The **section** is the page's lines from the first line equal to `DRIFT_KIT_PRICE_PAGE_SECTION` up to the line before the next heading of the same or a higher level (as many or fewer leading `#`), with CRLF read as LF. Its hash is `git hash-object --stdin` over those bytes. The stored hash is the table's `# price-page-hash:` header, read like the dating headers and transcribed with `priced-as-of:` whenever the rows are re-verified.*

*It prints one line, `price-page:` followed by one of:*

- *`off`*
- *`unchanged since priced-as-of <date>`*
- *`CHANGED since priced-as-of <date> — re-verify the rows, then record: # price-page-hash: <hash>`*
- *`n/a (no price-page-hash: header) — record: # price-page-hash: <hash>`*
- *`n/a (no section heading configured)`*
- *`n/a (section heading not found)`*
- *`n/a (fetch failed: <status>)`*

*A changed hash says something in the section moved: a wording edit, a new model, a price. It never says which, so the answer is always a re-read. A restructured page reads as changed or as not found, which is the same instruction.*

The fetch spawns through `Program::consumer` with the knob as its ground (gate-sdk/SPEC.md §The program roster).

### (3) Effective-from dating {design-bearing}

**Not yet applied.** drift-kit/SPEC.md §The stage-economics meter, input 3. After *The kit ships `templates/price-table.tsv` with placeholder rows and the column schema; the consumer copies it and fills their roster.*, add:

*A row's optional sixth column, `effective_from`, is an ISO day. A model may carry several rows, one per effective date, and an empty or absent column is an open start. The row **in force** on a day is the one with the latest `effective_from` on or before it, an open start being earliest. A day before a model's every row has no row in force. A second row repeating a model and date replaces the first, as a repeated id did. A row whose sixth column is not an ISO day prices nothing, and the run counts such rows in one stdout line. A cost cell is priced at the row in force on the log row's own date, the stamp date The trend log derives (the run's date where no dated stamp names the row). So a re-run reprices no row whose rate did not change, and a rate added for a date reaches only the rows dated on or after it.*

The sentence *Where a row is time-boxed, `prices-valid-through:` **owns** that date and the file's own prose cites the header rather than restating it* becomes: *A scheduled change whose next rate is published is written as a row dated for the change. `prices-valid-through:` owns the date the table is vouched for through: a cliff whose next rate is unpublished, or a re-check horizon. The file's prose cites the header rather than restating it.*

Implementation: `Prices` keeps `model → [(Option<day>, [f64; 4])]`, and `cell` takes the pricing date. `emit_row` passes `r.dates.row(iter, stage, supervision, suffix)`, else `r.today`, which covers stage, supervision and fan-out rows alike.

### (4) A zero-token cell prices at zero {mechanical}

**Not yet applied.** drift-kit/SPEC.md §The stage-economics meter, **Degradation**. The sentence *A price table that is absent, or that has no row for a model the transcripts name, degrades that model's cost cell to `n/a`* becomes *A price table that is absent, or that has no row in force for a model on its row's date, degrades that model's cost cell to `n/a`, unless its four token counts are all zero, which prices `0.0000` at any rate.* `Prices::cell` returns `0.0000` for all-zero tokens before the lookup. A logged `n/a` row for such a model re-prices to zero on its next re-measure, and a row whose transcript aged out keeps its line.

### (5) The lead reads the arm at its first step {mechanical}

**Not yet applied.** lifecycle-kit/templates/lead.md, after the paragraph ending *never overwrite the file.*:

*Then read the model roster: run `--price-coverage` on the gate binary `GATE_SDK_NATIVE_BIN` names (drift-kit/SPEC.md §The price-coverage arm; without drift-kit, skip this). An id it names unpriced is a harness roster churn. Re-judge every tier assignment before your first dispatch (§Economics), and file the missing row with `--emit file-gap`. File a `CHANGED` price page the same way.*

lifecycle-kit/SPEC.md §templates/lead.md, the paragraph opening *The lead's first step writes the session-role marker and opens its journal.*, becomes: *The lead's first step writes the session-role marker, opens its journal and reads the model roster.* It gains a closing sentence: *The roster read is drift-kit's `--price-coverage` arm, a cross-kit citation rather than a dependency: an id the consumer's price table has never priced is the roster churn §Economics' re-judge rule turns on, read before the first dispatch is tiered.*

### (6) The network-spawner roster {mechanical}

**Not yet applied.** gate-sdk/SPEC.md §The non-gate arm, the fence-safe paragraph. *The crate's only network spawners are `--usage-poll` (`curl`), `--pack-installer` (`npm`) and `--with-foreign-shells` (`docker`)* becomes *The crate's network spawners are `--usage-poll` (`curl`), `--pack-installer` (`npm`), `--with-foreign-shells` (`docker`) and `--price-coverage` (a consumer's `DRIFT_KIT_PRICE_PAGE_CMD`, where set).* The **Changeable by a consumer** bullet gains: *`--price-coverage` spawns `git hash-object`, and whatever `DRIFT_KIT_PRICE_PAGE_CMD` names only where that knob is non-empty (drift-kit/SPEC.md §The price-coverage arm).* `native/src/emit/mod.rs`'s `no_network_spawning_arm_is_fence_safe` test adds `--price-coverage` to `NETWORK_ARMS`. It is a non-`--emit` `Arm::Run`, so it is outside the fence-safe set already, and the test holds it there.

### (7) The knobs, the template and this repo's table {mechanical}

**Not yet applied.**

- **The knobs.** drift-kit/SPEC.md §Layout and configuration gains three rows, and `native/src/knobs/drift_kit.rs` the matching three:
  - *`DRIFT_KIT_PRICE_COVERAGE_DAYS` — non-negative integer, default `7`; the modification-time window `--price-coverage` reads transcripts over, `0` reading every transcript.*
  - *`DRIFT_KIT_PRICE_PAGE_CMD` — a command knob, one `DRIFT_KIT_PRICE_PAGE_CMD[] = word` line per argv element, spawned with no shell, default empty (the page half off); its stdout is the pricing page.*
  - *`DRIFT_KIT_PRICE_PAGE_SECTION` — the heading line whose section is hashed, default empty.*
- **The kit template.** drift-kit/templates/price-table.tsv: the line *Tab-separated. One row per model id (the id as it appears in the transcript's message.model).* becomes *Tab-separated. One row per model id and effective date (the id as it appears in the transcript's message.model; `effective_from` optional, an ISO day, empty for an open start).* The schema comment line gains `\teffective_from`. The template also gains, under the dating headers, *`# price-page-hash:` — optional; written by `--price-coverage` when `DRIFT_KIT_PRICE_PAGE_CMD` is set.* The placeholder rows stay five-column.
- **This repo's table.** scripts/price-table.tsv: *One row per model id as it appears in a transcript's message.model.* becomes *One row per model id and effective date (`effective_from`, empty for an open start), the id as it appears in a transcript's message.model.* The schema line gains `\teffective_from`. The `claude-haiku-4-5` row's id becomes `claude-haiku-4-5-20251001`, the id the harness writes; its four prices are unchanged. In the KNOWN UNDERCOUNT paragraph, *There is one row per model id by construction, so this is a stated limit rather than a second row.* becomes *Rows are keyed by model id and date, and speed is neither, so this is a stated limit rather than a second row.* No row gains a date: the 5.5-generation cache-read rate has no published start date. `# price-page-hash:` is recorded from the arm's first run.
- **This repo's opt-in.** scripts/drift-config.knobs gains `DRIFT_KIT_PRICE_PAGE_CMD[] = curl`, `-fsSL` and `https://platform.claude.com/docs/en/about-claude/pricing.md`, and `DRIFT_KIT_PRICE_PAGE_SECTION = ## Model pricing`, under one `# spec:` line citing §The price-coverage arm.

### (8) The fixtures {mechanical}

**Not yet applied.** drift-kit/smoke/install.sh gains a price-coverage block over its own throwaway sessions dir and table. It asserts that an id with usage and no row reads `unpriced`, that a zero-usage id is not counted, and that a row dated after today leaves its id unpriced while an open-start row prices it. It asserts `price-page: off` with the command knob empty. With the knob set to a `bash -c` argv printing a fixture page, it asserts `unchanged` against the fixture's stored hash and `CHANGED` after one price in the section is edited. It also asserts `section heading not found`. Its stage-economics block gains a two-row fixture model, one open start and one dated today, whose logged `cost` is the dated rate's. It also gains a zero-usage model whose cell reads `0.0000` and raises no incomplete-pricing caveat. drift-kit/SPEC.md §Testing gains a sentence naming each. `stage_economics.rs`'s unit test for the parser adds the column, the selection and the malformed-date count.

## Producers and consumers

Probes: `git grep -n 'PRICE_TABLE\|price-table\|price_table\|Prices::'` over the tracked tree, less the queue, `docs/posts/` and the generated mirrors; `usage_by_model` and `emit_row` read in `native/src/emit/stage_economics.rs`; the seven-day window counted with `find <sessions-dir> -name '*.jsonl' -mtime -7` (500 transcripts, 694 MB on the authoring host) and its ids read with `jq -r 'select(.type=="assistant") | .message.model'`; `curl -sSL` of the pricing page's markdown form; `grep -n 'open-lead-journal\|first step' lifecycle-kit/templates/lead.md lifecycle-kit/SPEC.md`.

- **The coverage reading** (delta 1). Producer: `--price-coverage`, run by the lead's first step (delta 5). Consumer: the lead's tier assignment and its gap filing. Each field has a reader. The head line's counts size the window for the reader. The unpriced ids name the row to add, and their presence is the churn signal.
- **The page verdict** (delta 2). Producer: the same arm, only where `DRIFT_KIT_PRICE_PAGE_CMD` is set, which this repo's knob file sets (delta 7). Consumer: the same session, re-verifying the rows. The printed hash is read by the session that records it into the table header, and the header by the arm's next run.
- **`# price-page-hash:`** (delta 2). Producer: a session transcribing the arm's printed value. Consumer: the arm's page half. `kpi-price-table-age` reads only `priced-as-of:` and `prices-valid-through:` by name, so a third header changes neither of its rows.
- **`effective_from`** (delta 3). Producer: the consumer's table. Consumers: `Prices::parse` for the meter and for the arm, one parser. The `/economics` narrative and the lead binding read `cost`, whose meaning is unchanged: the priced spend of the row.
- **The zero-token rule** (delta 4). It changes what the meter's incomplete-pricing caveat counts, and that caveat's reader is the `/economics` narrative. `smoke/install.sh`'s missing-row assertion prices a fixture model with non-zero tokens, so its `n/a` and caveat assertions stand.
- **Roster-holding readers of the new names.** The arm's `ARMS` row and its `KNOBS`, held by the arm-knob test. The drift-kit knob table, which `--emit knob-roster` and the knob parity gates read (delta 7). `NETWORK_ARMS` (delta 6). The `Program::consumer` ground scan, which requires the ground to name a knob, as `DRIFT_KIT_PRICE_PAGE_CMD` does.

## Existing sections updated

Roster probe: the `git grep` above, plus `grep -n 'network spawners\|Changeable by a consumer' gate-sdk/SPEC.md`.

- `drift-kit/SPEC.md` — a new §The price-coverage arm (deltas 1 and 2); §The stage-economics meter, input 3 and **Degradation** (deltas 3 and 4); §Bundled KPIs, `kpi-price-table-age`'s sentence *the KPI reads the table's header and never its rows* gains *— an unpriced id is `--price-coverage`'s (§The price-coverage arm)* (delta 1); §Layout and configuration (delta 7); §Testing (delta 8).
- `native/src/emit/price_coverage.rs` (new), `native/src/emit/mod.rs` (deltas 1, 2 and 6).
- `native/src/emit/stage_economics.rs` (deltas 3, 4 and 8).
- `native/src/knobs/drift_kit.rs` (delta 7).
- `drift-kit/templates/price-table.tsv`, `scripts/price-table.tsv`, `scripts/drift-config.knobs` (delta 7).
- `drift-kit/README.md` — the usage block gains `"$gates" --price-coverage  # which running model ids the price table cannot price; with a page command set, whether the pricing page moved` (delta 1).
- `lifecycle-kit/templates/lead.md`, `lifecycle-kit/SPEC.md` §templates/lead.md (delta 5).
- `gate-sdk/SPEC.md` §The non-gate arm (delta 6).
- `drift-kit/smoke/install.sh` (delta 8).
- `.workflow/surface-ceiling.txt` — the grown `drift-kit/SPEC.md`, `lifecycle-kit/SPEC.md`, `lifecycle-kit/templates/lead.md` and `gate-sdk/SPEC.md` rows re-stamped with `bash gate-sdk/bin/run-gates.sh --emit always-loaded --ceiling`, which `check-surface-ratchet` demands with the growth (all deltas).
- `docs/drift-kit/`, `docs/lifecycle-kit/` and `docs/gate-sdk/` SPEC and README mirrors, regenerated with `bash gate-sdk/bin/run-gates.sh --emit docs-mirror --write` (all deltas).

## Retired spellings

- None — the deltas add an arm, three knobs, a column and a header. Delta 7's corrected row id is consumer data, and its old spelling is a prefix of the new one, so no name, knob or path is retired.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical text it refines rather than appending to it.
- [ ] **Amendment deleted** — this file removed on merge; none remain at the root (`ls SPEC-*.md`).
- [ ] **Removals propagated** — `## Retired spellings` above holds, checked by `check-amendment-retired-spelling`.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks.
- [ ] **Witnessed on this tree** — `--price-coverage` run before delta 7's row fix names `claude-haiku-4-5-20251001` unpriced, and after it reads `all priced`; its page half prints `unchanged` once the hash is recorded; its wall-clock over the default window on this host is stated in the build's report. Both entries move to Done before the drain stage (`LIFECYCLE_KIT_DRAIN_STAGE`).
