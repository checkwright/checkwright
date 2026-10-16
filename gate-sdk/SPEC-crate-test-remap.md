# SPEC amendment: crate-test-remap

Pairs `crate-test-unremaps-binary`. `cargo test --release` over the crate builds the bin target the integration tests need into the directory `bin/build-native.sh` builds into. It builds it without that script's `--remap-path-prefix` flags, so cargo uplifts an unremapped binary over the placed one, and the next `--installer-smoke` refuses it. Two spellings reach it: `check-crate-arms`' test arm and validate's `native_crate` suite.

**The probe the design rests on**, run with a scratch target dir under `.tmp/` on cargo 1.99 against this crate:

| Step | Units compiled | Placed binary | Home-path hits |
|---|---|---|---|
| `cargo build --release` under the remap flags | 7 | built | 0 |
| `cargo test --release --no-run` under the same flags | 1 | unchanged | 0 |
| the same test with no flags | 7 | rewritten | 3 |
| the remap build again | 0 | restored | 0 |

So cargo keys the compiled units by their flags, and a test arm carrying the build's flags reuses the build's cache and never rewrites the placed binary. That is the cheapest spelling: it compiles nothing the build already compiled, where a target dir of its own compiles the dependencies and the binary again.

## What changes

### (1) The build's RUSTFLAGS get one home in `lib/gate.sh` {design-bearing}

`bin/build-native.sh` composes its `RUSTFLAGS` inline: the caller's value, then one `--remap-path-prefix=<prefix>=/builder` for each of `CARGO_HOME` (default `$HOME/.cargo`) and `HOME`. A prefix that is empty or `/` contributes no flag, and one trailing `/` is stripped. That composition moves into a `lib/gate.sh` function, `gate_native_rustflags`, which prints the value. `build-native.sh` exports what it prints, and the composition is unchanged.

The home is shell because §build-native's `# no-port:` cause already forces it: the build runs before any binary exists. It is a library function rather than a second script, so the battery's reader below takes the value without re-deriving it.

Replacement for §build-native, the paragraph beginning **The released artifact carries no builder path**, its last three sentences. **Not yet applied.**

> The build therefore exports the `RUSTFLAGS` value `gate_native_rustflags` prints (§lib/gate.sh): the caller's own flags, then `--remap-path-prefix` for two prefixes. `CARGO_HOME` (defaulted to `$HOME/.cargo`) is the measured leak, and `HOME` covers a cargo home this tree cannot predict and any other path under the builder's home. A prefix that is empty or `/` contributes **no** flag, because remapping `/` would rewrite every absolute path in the binary. Every cargo call that can rewrite the placed binary takes that one value (§check-crate-arms).

A new §lib/gate.sh bullet, beside `gate_native_source_stamp`. **Not yet applied.**

> - `gate_native_rustflags` prints the `RUSTFLAGS` value the crate builds under, the caller's own followed by §build-native's remap flags. It is a pre-binary function, since the build reads it before any binary exists. §check-crate-arms reads it through a `bash` spawn rather than re-deriving it, so the value has one holder.

### (2) `check-crate-arms`' cargo arms build under that value {design-bearing}

The gate reads the value once per run from a `bash` spawn that sources `lib/gate.sh` under the `GATE_SDK_ROOT` locator, and sets it as `RUSTFLAGS` on both cargo spawns, the clippy arm and the test arm. The test arm then reuses the build's compiled units and leaves the placed binary as `build-native.sh` left it. The clippy arm takes the value too, so every cargo spawn of the gate builds as the build does, and no arm keeps a third flag set in the cache.

- **A spawn rather than a crate twin of the composition.** The crate consumes the library's answer and re-derives nothing, which is §run-gate-tests' discriminator for the `gate_command` spawn. A twin held equal by a cross-substrate test would be duplication held by a machine where it can be absent.
- **The gate is the publisher's own and runs only where the crate is**, so `bash` is the contributor floor it already stands on: `build-native.sh` is bash. A spawn that cannot start, or that exits non-zero, is exit 2 naming the function, since an arm that cannot learn the build's flags cannot say the test left the binary alone.
- **The gate declares `bash`.** It joins `cargo`, `rustc` and `git` among the programs the gate declares, and unit test A then observes it.
- **The cache key is unchanged.** The value is a function of the builder's home and the caller's flags, and neither moves a verdict.
- **The target-lint arm is the consumer's command and takes nothing** (§The knob file, command knobs).

Replacement for §check-crate-arms, the sentence after the two-line command block. **Not yet applied.**

> `$CRATE` is `gate_native_crate` (§Layout and configuration), `$TARGET_DIR` is `GATE_SDK_CARGO_TARGET_DIR`, and both spawns run under the `RUSTFLAGS` `gate_native_rustflags` prints (§lib/gate.sh). So the test arm reuses the cache `bin/build-native.sh` fills and never rewrites the binary it placed, and its read is a `bash` spawn that exits 2 on failure.

Replacement for §check-gate-binary-fresh, the paragraph **Running the crate's tests does not discharge the rebuild**, its second and third sentences. **Not yet applied.**

> A **plain** `cargo test --release` also rewrites that binary unremapped, because the integration tests need the bin target. This gate then reads the binary current, and the installer smoke's staging step refuses it at exit 2 on the builder path the banned-pattern roster matches (installer/SPEC.md §The consumer smoke). The test arm of §check-crate-arms builds under the build's own flags and leaves the binary alone, and validate's crate suite builds into a target dir of its own. So the obligation is `bash gate-sdk/bin/build-native.sh` (§build-native) as **its own named step**, after a plain test run where both run: a contributor routine stating a `cargo test` line has not stated it.

### (3) Validate's crate suite builds into a target dir of its own {mechanical} {user-facing: crate-test-unremaps-binary's Deliverable, the test arms on a target dir of their own}

A suite's run command is split and exec'd as a literal argv with no expansion (`scripts/evidence-config.knobs`, the comment above the run commands). So it cannot carry a value composed from the machine's home directory. It takes a target dir of its own instead, under the crate's ignored `target/`, which persists across iterations:

`EVIDENCE_KIT_RUN_native_crate = cargo test --release --manifest-path native/Cargo.toml --target-dir native/target/validate`

- **README.md's battery-roster register carries the same line.** `check-battery-roster` holds the register to each suite's run command as an exact string (evidence-kit/SPEC.md §check-battery-roster), so the two change in one commit.
- **The cost is one cold build per clone**: 7 units, measured at about 50 seconds by the probe's no-flags row.
- **The integration tests still spawn the placed binary.** They spawn it through `GATE_SDK_NATIVE_BIN`, which validate runs after a build, so the suite reads the same binary it did.
- **The CI crate-test legs pass `--target <triple>`**, so cargo places their artifacts under `target/<triple>/` and never over the placed host binary. They are unchanged. Probed: a no-flags `cargo test --release --no-run --target x86_64-unknown-linux-gnu` into the scratch dir above built under `<dir>/x86_64-unknown-linux-gnu/release/` and left the host-layout binary's digest unchanged.

## Producers and consumers

- **`gate_native_rustflags` (delta 1).**
  - Producer: `lib/gate.sh`. No enabling config: the inputs are `RUSTFLAGS`, `CARGO_HOME` and `HOME`, read as variables.
  - Consumers: `bin/build-native.sh`, which exports its output on every build, the CI legs' included, and `check-crate-arms`, which reads its output on every non-cached run in this tree (delta 2).
  - Roster-holding readers: §lib/gate.sh's function list, updated by delta 1. `run-gates.ps1`, the PowerShell front end, re-holds only the library functions it calls, and it calls no build. Probe: `grep -n "gate_native" gate-sdk/bin/run-gates.ps1`.
- **The arms' `RUSTFLAGS` (delta 2).**
  - Producer: the gate.
  - Consumer: cargo, through the environment of both spawns.
  - Roster-holding reader: the gate's declared program set, its `--needs` answer, which gains `bash`. Unit test A holds observed ⊆ declared (§The port-candidate criteria).
- **The suite's target dir (delta 3).**
  - Producer: the knob value.
  - Consumers: `--run-validate`, which execs it, and `check-battery-roster`, which compares it with the README line.
- **No field and no corpus narrows**, so points 4 to 6 add nothing.

## Existing sections updated

The roster below came from `git grep -n "cargo test" -- ':!*.rs'` and `grep -n "remap\|RUSTFLAGS" gate-sdk/bin/build-native.sh gate-sdk/lib/*.sh` over the tracked tree.

- `gate-sdk/SPEC.md` §build-native, the remap paragraph (delta 1).
- `gate-sdk/SPEC.md` §lib/gate.sh, the new function bullet (delta 1).
- `gate-sdk/SPEC.md` §check-crate-arms, the arms' paragraph and the declared-programs paragraph, which gains `bash` (delta 2).
- `gate-sdk/SPEC.md` §check-gate-binary-fresh, the crate-tests paragraph (delta 2).
- `gate-sdk/SPEC.md` §Layout and configuration, the `GATE_SDK_CARGO_TARGET_DIR` bullet. It says the warm cache `bin/build-native.sh` fills is the one the gate reuses, which delta 2 makes true. The bullet gains "under the build's own `RUSTFLAGS`" (delta 2).
- `docs/gate-sdk/SPEC.md`, the generated mirror, regenerated by the command its freshness gate prints (deltas 1 and 2).
- `gate-sdk/lib/gate.sh`, the new function (delta 1).
- `gate-sdk/bin/build-native.sh`, which reads the function (delta 1).
- `native/src/gates/crate_arms.rs`, the read, the two spawns, and the gate's `--needs` answer, which gains `bash` (delta 2).
- `scripts/evidence-config.knobs`, `EVIDENCE_KIT_RUN_native_crate` (delta 3).
- `README.md`, the battery-roster line (delta 3).
- <!-- update-target-exempt: the sentence says a plain cargo test rewrites the binary, which stays true -->`installer/SPEC.md` §The consumer smoke, the staging refusal's sentence on a plain `cargo test --release`. Unchanged.
- <!-- update-target-exempt: the line says cargo test does not discharge the build, which stays true -->`CONTRIBUTING.md`, the build-before-commit bullet. Unchanged.

## Retired spellings

- None — no delta of this amendment retires a name; the suite's run command gains an operand and keeps its name.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The causal-completeness check holds for each new state, event, interface and obligation.
- [ ] **Instruction surfaces: instruction only** — replacement text for a template, agent definition or shim carries no grounds; a delta places them (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition re-phrases the canonical-spec text it refines rather than appending to it.
- [ ] **The deliverable observed** — after a crate edit, `bash gate-sdk/bin/build-native.sh`, then a full `--run`, then the `native_crate` suite: the placed binary carries no home path, and `--installer-smoke` passes with no rebuild between.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — `## Retired spellings` above holds.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as debt tasks.
