# SPEC amendment: kpi-powershell

The drift report starts a consumer's tier-1 or tier-2 KPI plugin, `<dir>/<name>.sh`, as a program with no interpreter word (`native/src/emit/drift_report.rs`, `invoke`). Windows starts no `.sh`, so on a native-Windows host the spawn fails and the row reads its plugin-failed value. §The extensibility contract's promise, that a consumer adds a KPI by dropping a plugin in its own KPI dir, is unreachable there.

The entry left the dispatch shape open and said `custom-gate-substrates`' ruling bears on it. That ruling (gate-sdk/SPEC-gate-substrates.md) admits a PowerShell script as a gate's implementation, started by the host `GATE_SDK_POWERSHELL` names. This amendment gives a plugin the same class through the same helper: a tier dir may hold `<name>.ps1`, started by that host. One knob then arms PowerShell for the consumer's gates and plugins alike, and a native-Windows consumer meets the promise with no bash.

**Refused: starting a `.sh` plugin through `bash` on Windows.** It would need Git for Windows' bash for a plugin a Windows consumer would rather write in PowerShell, and it would split the plugin start in two per platform. The execute-bit rule depends on starting the file directly, and on Windows that rule does not exist. A `.sh` plugin keeps its start unchanged on every host.

## What changes

### (1) A tier dir resolves a PowerShell plugin {design-bearing} {user-facing: windows-kpi-plugin-start's Deliverable, "a plugin dispatch a native-Windows consumer can meet", operator direction lead-relayed when it was filed}

Replacement for §The extensibility contract's tier list and the paragraph after it. **Not yet applied.**

> 1. each entry of `DRIFT_KIT_KPI_DIRS` — the adopter's own dirs, default the consumer gates dir — for a file named `<name>.sh`, then `<name>.ps1`;
> 2. each vendored kit's `kpis/<name>.sh`, then `kpis/<name>.ps1`;
> 3. the **built-in members the binary carries** (§Bundled KPIs).
>
> So a consumer file named `kpi-task-split.sh` or `kpi-task-split.ps1` **shadows** the built-in of that name, and within one dir the `.sh` file wins. The port moved the kit's own members into tier 3 and touched nothing about how a consumer's are found.
>
> A tier-1 or tier-2 `.sh` hit is **executed directly** (`"$path"`, never `bash "$path"`), so the execute bit still governs and a non-executable plugin degrades to the fail-visible row rather than running anyway. A `.ps1` hit is a **PowerShell plugin**, started by the host `GATE_SDK_POWERSHELL` names with gate-sdk's PowerShell target arguments (gate-sdk/SPEC.md §The gate-authoring surface), so the knob, not an execute bit, is what arms it. With the knob empty its row reads `n/a (PowerShell plugin, GATE_SDK_POWERSHELL is empty)`, which names the knob that arms it. A native-Windows host starts no `.sh`, so there a `.sh` plugin's row is its fail-visible read, and a PowerShell plugin is the form that runs. A consumer plugin reads the exported environment of §The KPI plugin contract and a built-in reads the same resolved values in process, so a consumer plugin is no second-class reader of the contract.

Replacement for §The KPI plugin contract's first sentence. **Not yet applied.**

> A plugin is `kpi-<name>.sh` or `kpi-<name>.ps1`, resolved through the registry (§The extensibility contract). A `.sh` plugin is invoked directly and must carry the execute bit, or the report degrades that row to its fail-visible read; a `.ps1` plugin is started by the PowerShell host. Two modes:

The modes, the exported environment and the scratch rule are unchanged, and a PowerShell plugin receives the same environment.

Replacement for `drift-kit/README.md`'s KPI paragraph, its first two sentences. **Not yet applied.**

> A KPI plugin is `kpi-<name>.sh`, or `kpi-<name>.ps1` started by the PowerShell host `GATE_SDK_POWERSHELL` names, resolved through `kpis.list` against your KPI dirs, then each vendored kit's `kpis/`, then the binary's built-in members — three tiers, consumer-first. Add your own by dropping a plugin in your gates dir and naming it in the registry; on native Windows, write it in PowerShell. Shadow a bundled one with a same-named file.

### (2) The plugin start calls gate-sdk's PowerShell helper {mechanical}

- `resolve` in `native/src/emit/drift_report.rs` tries `<name>.sh` then `<name>.ps1` per tier-1 dir and per tier-2 root, and `Resolved::Plugin` carries which it found.
- `invoke` starts a `.sh` plugin as today. It starts a `.ps1` plugin with the program and arguments `native/src/substrate.rs` returns for a PowerShell target, the plugin's own `--trend` after them, through the same `proc::run_with_env` and environment. An empty host yields the row of delta 1 without a spawn.
- The arm's `KNOBS` gains `GATE_SDK_POWERSHELL`.
- **This delta's act depends on gate-sdk/SPEC-gate-substrates.md delta 2.** Where that delta has landed first or lands in the same batch, this delta calls `substrate.rs` as it stands. Where this delta lands first, it creates `native/src/substrate.rs` with the PowerShell target class's extension test, argument list, host resolution and the `GATE_SDK_POWERSHELL` row exactly as that delta specifies, and that delta then calls the module rather than creating it.

### (3) The crate cases {mechanical}

In `drift_report.rs`' test module:

- resolution finds a tier-1 `.ps1` where no `.sh` sits beside it, prefers a `.sh` beside it, finds a tier-2 `.ps1`, and a tier-1 `.ps1` shadows a built-in;
- with `GATE_SDK_POWERSHELL` empty a `.ps1` plugin's row is delta 1's text and nothing is spawned;
- **a live case**: a `.ps1` plugin printing one lead row and one `--trend` fragment, run in both modes under each host that resolves. It asserts the row, the fragment and the plugin's view of one exported `DRIFT_KIT_*` value. On a Windows build it runs under `powershell` unconditionally, since every Windows host carries it, so the Windows legs bind it. A local run under gate-sdk's `--with-foreign-shells` reaches the `pwsh` arm.

### (4) The release declaration {mechanical}

`.workflow/release-declarations.md` §Behavior changes gains one bullet: **drift-kit KPI plugins** — a KPI dir may hold `kpi-<name>.ps1`, started by the PowerShell host `GATE_SDK_POWERSHELL` names, so a consumer KPI reports on native Windows. Nothing to do; a `.sh` plugin is unchanged, and wins over a `.ps1` of the same name in one dir.

## Producers and consumers

- **A PowerShell plugin.** Producer: the consumer, a file in a KPI dir named in `kpis.list`. Enabling config: `GATE_SDK_POWERSHELL` set, in the consumer's knob file or local overlay; this repo registers no consumer plugin, so the live producers are delta 3's cases. Consumer: the drift report's `invoke`, by spawn, at every report and `--trend` read, and its output by the row parser, unchanged.
- **The host-off row.** Producer: `invoke`, on a `.ps1` hit with the knob empty. Consumer: the report's reader, the same reader as the plugin-failed row. drift-kit/smoke/install.sh asserts no plugin-failed row in the bundled solo run. It reads the literal `plugin failed`, which this row does not contain, and the bundled run resolves no `.ps1`, so its verdict cannot move.
- **Roster-holding readers.** `drift_report.rs`' `KNOBS` (delta 2), the source `check-reads-couples` and the knob-file derivation read the arm's declarations from. The tier list is prose in §The extensibility contract and `drift-kit/README.md` (delta 1). `templates/kpis.list` names members, never files, so it gains nothing.
- **Knobs.** None is minted here; `GATE_SDK_POWERSHELL` is gate-sdk's, read across kits as §The non-gate arm permits.
- **The seam.** The tier rule and the start are kit mechanism; the plugin and the host are the consumer's.

## Existing sections updated

- `drift-kit/SPEC.md` §The extensibility contract — the tier list and the start paragraph (delta 1).
- `drift-kit/SPEC.md` §The KPI plugin contract — the first sentence (delta 1).
- `drift-kit/README.md` — the KPI paragraph (delta 1).
- `native/src/emit/drift_report.rs` — `resolve`, `invoke`, `KNOBS` and the cases (deltas 2 and 3).
- `native/src/substrate.rs` — created here only where gate-sdk/SPEC-gate-substrates.md delta 2 has not landed (delta 2).
- `.workflow/release-declarations.md` — the §Behavior changes bullet (delta 4).
- `docs/drift-kit/SPEC.md` and `docs/drift-kit/README.md` — generated mirrors, stale once a delta lands (all deltas).

## Retired spellings

- None — the amendment widens the tier rule by one extension and retires no spelling.

## Definition of Done

- [ ] **Causal completeness** — every point of SPEC §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it; the merged spec reads as one document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls drift-kit/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in `## Retired spellings` above, and `check-amendment-retired-spelling` runs each declaration against the whole tracked tree, not against the specs alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks (a build-time causal gap is resolved that session, not deferred).
- [ ] **Entry moved** — `windows-kpi-plugin-start` moves to Done with `--queue done` once its remote oracle is read: the mid-iteration push's `crate-tests-windows` legs green with delta 3's Windows case. It moves ahead of the drain stage.
