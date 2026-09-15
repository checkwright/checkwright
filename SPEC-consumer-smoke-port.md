# SPEC amendment: consumer-smoke-port

Queue entries: `run-consumer-smoke-port` and `consumer-smoke-library-port`, both riding unit set
`owed-port-tail` (operator direction, 2026-09-15, lead-relayed), and the rider
`gate-command-status-conflation-third-caller`, whose second surviving call site (`acct_probe`) this port
deletes. SPEC-git-hooks-port.md deletes the first, and the rider closes when both have landed.

**The order the two entries left to spec: together, in one unit.** `lib/consumer-smoke.sh` cannot be
deleted while `bin/run-consumer-smoke.sh` sources it. Porting the library first while keeping that shell
caller would mean an in-crate builder beside a library still holding the same mechanics, which is the
second holder §The port-candidate criteria's criterion 6 refuses. Porting the harness first would give
it the `bash -c` spawn road the three compiled arms already take, only for the next unit to delete that
road. That is one unit of throwaway wiring. The two files share one builder and one set of callers, so
they port together. The harness becomes an arm, the builder moves into the crate, every arm calls the
builder in process, and both files are deleted.

**What stays spawned, and it is not negotiable here.** The scratch battery, each kit's
`smoke/install.sh` and `smoke/violation.sh`, and the registration accounting's probes are all children
of the arm. §Consumer smoke rules the battery's spawn load-bearing: an in-process registry would run the
host's gates against the scratch tree. Delta 2 states the same ground for the probes.

## The seam

- **Kit mechanism:** the in-crate scratch-consumer builder, the `--run-consumer-smoke` arm, the three
  arms' in-process calls, and the reworded entry-point guard hint in every kit's `smoke/` recipes and
  their fixtures.
- **Consumer config:** unchanged. `GATE_SDK_KIT_DIRS` and `GATE_SDK_NATIVE_BIN` keep their meaning, and
  `TMPDIR` stays an environment read, not a knob.
- **This repo's own:** `scripts/evidence-config.knobs`' `consumer_smoke` suite command, and two
  permission allowlist entries (delta 5).
- **Private rule content:** none in reach.

## What changes

### (1) The scratch-consumer builder moves into the crate {design-bearing}

`native/src/emit/csmoke.rs` stops being a spawn seam onto the shell library and becomes the builder
itself. Its three functions are the library's three, with the library's contracts:

- `gate_descriptors(roots) -> usize` counts the `.gate` descriptors under each root's `checks/`. It is
  the one derivation of whether a kit set needs the binary, kept separate so a caller that must produce
  a binary asks the same question a step earlier (§Consumer smoke).
- `place_binary(consumer, host, roots) -> Result<(), PlaceError>`: when `gate_descriptors` is zero it
  does nothing. Otherwise it copies `<host>/<GATE_SDK_NATIVE_BIN>` to `<consumer>/<GATE_SDK_NATIVE_BIN>`,
  creating the parent. `PlaceError` names the two environment failures the shell form printed, an
  unnamed host and an absent or non-executable artifact (with the `build-native.sh` help line), and each
  caller renders it in its own verdict grammar.
- `vendor_and_install(host, roots, base, out) -> Result<Scratch, String>`, where `Scratch` carries the
  consumer directory and the installed-kit count. It:
  1. creates `consumer-smoke.XXXXXX` under `base`;
  2. runs `git init`, writes a `.gitignore` of `.tmp/` plus the binary path, and makes the seed commit;
  3. copies each root by name;
  4. calls `place_binary`;
  5. runs each kit's `smoke/install.sh` with `bash`, cwd at the consumer, and `GATE_SDK_ROOT` and
     `SMOKE_KIT_ROOT` exported;
  6. commits the installed baseline with `--no-verify`.

  An installer's non-zero exit is an `Err` naming the kit, and the caller maps it to its environment
  failure. `base` is the caller's (`TMPDIR`, else `/tmp`, as the shell form read it). `out` routes the
  installers' output. `--run-consumer-smoke` keeps it on stdout as the sourced harness did, and the
  three other arms keep it on stderr, where the retired stdout protocol had forced it. **No caller's
  output channel moves in this cut.** The protocol that forced stderr is gone, and moving a channel is a
  separate, observable change this port does not make.

The two shell variables the library communicated through, `SCRATCH` and `CSMOKE_INSTALLED`, become the
returned `Scratch`. The spawn wrapper `csmoke::spawn`, the `csmoke::SOURCE` prologue and the
read-`SCRATCH`-off-stdout protocol are deleted. Cleanup stays each caller's, as the library's contract
said. The builder returns the directory before any assertion runs, so a caller's `--keep` or teardown
sees it even when a later step fails.

### (2) The harness becomes `--run-consumer-smoke` {design-bearing}

A new `Arm::Run` row, `--run-consumer-smoke`, in `native/src/emit/run_consumer_smoke.rs`, taking
`[--keep] [kit-root...]`. Its knob roster is `GATE_SDK_KIT_DIRS` and `GATE_SDK_NATIVE_BIN`, the test
that put `--agents-md-smoke` and `--run-demo` in the table. It is a bare flag because its contract is an
exit status that an emitting arm would collapse (§The non-gate arm). The front-end passes it through
with no edit. The usage text `run-gates.sh --help` prints, which lives in `native/src/runner.rs` beside
`--upgrade-smoke`'s line, gains one line for it. Its operands are documented there and nowhere else.

**The contract is §Consumer smoke's unchanged: phases, order, exit codes 0/1/2, and every line of
output, byte for byte.** That includes the `CONSUMER-SMOKE: clean (…)` token, the
`CONSUMER-SMOKE: accounting — …` line, each `FAIL —` line with its help text, the `no violation
script` notice and the `--keep` retention line. Diagnostics keep their `run-consumer-smoke:` prefix,
which is the arm's own name without its dashes. The steps, in the script's order:

- **Operands.** `--keep` retains the scratch. Any other `-`-led word is `unknown option`, exit 2. A
  non-directory kit root is exit 2. With no roots, the set is `walk::kit_roots_abs()`, and gate-sdk is
  always ordered first.
- **Recipes.** A root without `smoke/install.sh` is exit 2 with the existing help line.
- **Build.** `csmoke::vendor_and_install` with the invoking checkout as host. Its `Err` is exit 2.
- **Battery.** `bash gate-sdk/bin/run-gates.sh` is spawned with cwd at the consumer and both streams
  merged, and the arm asserts exit 0 plus the `All N gates passed` token.
- **Accounting.** Registered members come from `registry::members` over the scratch
  `scripts/gates.list`. The universe is the `check-*.sh` and `check-*.gate` basenames under each vendored
  kit's `checks/`. Declarations are the `# smoke-unregistered:` lines of each vendored
  `smoke/install.sh`, with the separator trimming the script applies. Then come the probe (below), the
  measured wall-clock, the three stale-declaration shapes, and the contradicted `zero-config` report,
  which reads the first `# install:` token of the gate's declaration.
- **Restore.** `git reset -q --hard` and `git clean -qfd` in the consumer, before the violation phase
  and after each violation.
- **Violations.** Each kit's `smoke/violation.sh` is spawned with `bash` under the recipe environment.
  Its first stdout line names the expected gate, and the battery must go non-zero carrying
  `FAIL: <gate>`. The final green check follows.

**The probe resolves in one checks dir and spawns in the probed tree, so the rider's conflation cannot
recur.** `acct_probe(tree, checks_dir, gate)` becomes a function returning a `Probe` of `Exit(code)` or
`HarnessError(message)`:

- `registry::resolve` over the single `checks_dir`. **Not found** is `Exit(2)`, the "could not run"
  reading the corroboration table already gives an unresolvable dispatch.
- A **`.sh`** declaration is spawned by its path with cwd at `tree`, as the shell form executed it.
- A **`.gate`** declaration spawns `<tree>/<GATE_SDK_NATIVE_BIN> <gate>` with cwd at `tree`: the scratch
  consumer's placed binary on the scratch leg, and the invoking checkout's binary on the host leg. **An
  absent or non-executable binary there is `HarnessError`**, and the arm prints it naming the tree and
  the path and exits 2. The shell form read that case as a probe verdict of 2. On the host leg it could
  then combine with the scratch leg into an `unaccounted` exit 1 blaming the gate. On the scratch leg it
  could fake the surface-absent row and grant a permanent exemption, the false-exemption channel the
  corroborating probe exists to close.
- Both streams of a probe are discarded, as today.

**Both legs spawn, and neither runs in process.** The ground is the battery's, and it reaches the
probes: a gate reads its knob files relative to its working directory, once per process, so an
in-process call from the arm would read the invoking tree's knob state while pointed at the scratch
tree. On the host leg an in-process call would happen to pair correctly. Spawning there too keeps one
probe shape instead of two, a leg-specific shortcut being exactly where a later edit reintroduces the
mispairing.

### (3) The three compiled arms call the builder in process {mechanical}

- `native/src/emit/upgrade_smoke.rs`: `descriptors` calls `csmoke::gate_descriptors`, `vendor_and_install`
  calls the builder with its scratch base and FROM's binary tree as host, and `place_binary` calls
  `csmoke::place_binary` with TO's. Its private `bash` wrapper keeps serving its own remaining scripts
  (`regenerate` and the rest) over `proc::run_streamed` directly, since `csmoke::spawn` is gone.
- `native/src/emit/agents_md_smoke.rs`: `vendor` calls the builder, and its private `spawn` over
  `csmoke::spawn` is deleted. Its refusal that "the kit roots name no gate-sdk root, so the
  consumer-smoke library … cannot be found" keeps its check. Gate-sdk still leads the vendored set and
  holds `bin/run-gates.sh`. The reason clause names that front-end instead of the library.
- `native/src/emit/demo.rs`: `place_binary` calls `csmoke::place_binary`. Its own narrated scratch build
  is untouched, for the reason §Consumer smoke gives.

### (4) Both shell files are deleted {mechanical}

`gate-sdk/bin/run-consumer-smoke.sh` and `gate-sdk/lib/consumer-smoke.sh` are deleted. `lib/gate.sh`
loses no function here: `gate_kit_roots`, `gates_list_members` and `gate_command` each keep other
callers (a tracked-tree census the build re-takes).

### (5) Callers and the entry-point guard hint {mechanical}

- `scripts/evidence-config.knobs` — `EVIDENCE_KIT_RUN_consumer_smoke = bash gate-sdk/bin/run-gates.sh
  --run-consumer-smoke`. The suite name, its parser (the default) and its
  `.workflow/validate-baseline.txt` row are unchanged, because the verdict grammar is.
- Every `smoke/install.sh` and `smoke/violation.sh` that carries the entry-point guard, and every
  fixture copy of one: the guard's hint text becomes
  `: "${SMOKE_KIT_ROOT:?run via run-gates.sh --run-consumer-smoke}"`. `check-smoke-entry-guard` matches
  the `${SMOKE_KIT_ROOT:?` prefix alone (`native/src/gates/smoke_entry_guard.rs`), so no gate verdict
  moves. The hint is what a bare invocation prints, and it would otherwise name a deleted file.
- `.claude/settings.json` — `Bash(bash gate-sdk/bin/run-consumer-smoke.sh)` and its `*` sibling are
  stale. The build prepares their removal as a diff for the lead to relay, as CLAUDE.md §Housekeeping
  requires, and never applies it by hand. **Not yet applied.**

### (6) The SPEC sections that described the two shell files {design-bearing}

gate-sdk/SPEC.md §Consumer smoke. **Not yet applied:**

- The opening paragraphs name the harness as `run-gates.sh --run-consumer-smoke [--keep] [kit-root...]`,
  a non-gate `Arm::Run` and never a registered gate.
- **The scratch-consumer build itself is factored into `lib/consumer-smoke.sh`** becomes: the build is
  `native/src/emit/csmoke.rs`'s `vendor_and_install`, which returns the scratch directory and the
  installed count. The caller-count statements that follow (three callers of the builder: the harness,
  the AGENTS.md smoke, the upgrade suite; `place_binary` also called by the walkthrough) keep their
  subjects under the new name.
- **Deleted:** **Two of those three are compiled arms, and each reaches the library by spawning `bash`**,
  **The library's sourcer set is narrower than the builder's caller set** and **The library is owed its
  port**. Their subject, a shell library reached across a process boundary, no longer exists. Nothing in
  them survives, because the single-producer rule they applied is now satisfied by construction and
  criterion 6 already states it.
- The walkthrough paragraph's "reaches this library for `csmoke_place_binary` alone" and **It reaches
  `csmoke_place_binary` by spawn** become in-process calls to the builder's `place_binary`. That second
  paragraph's refusal of a crate-side placement beside the library is deleted, since the library's own
  port was the road it named.
- *The registration accounting* gains delta 2's probe rule (one checks dir, spawn in the probed tree,
  `HarnessError` as exit 2), and the spawned-battery ground is extended to the probes.
- `smoke/`'s per-kit contract quotes the guard with its new hint (delta 5).

gate-sdk/SPEC.md §The port disposition, the paragraph **The class membership above is closed and
enumerated, and the harness's own two files are not in it**. **Not yet applied:** it keeps the closed
class and states that the harness is the `--run-consumer-smoke` arm and the builder is in-crate. The
two constraints it lists as "what the ports must keep" move into *The registration accounting*, where
the arm now honours them.

gate-sdk/SPEC.md §run-consumer-smoke is rewritten to name the arm. §upgrade-smoke's "third caller of
`csmoke_vendor_and_install`" names the builder. §The non-gate arm's roster gains `--run-consumer-smoke`.
§Consumer payload, §The port-candidate criteria and every other gate-sdk/SPEC.md sentence naming either
file or a `csmoke_` function is rewritten to the arm or the builder. **Not yet applied.**

context-kit/SPEC.md §Testing, **The consumer-smoke helpers are called in the library that owns them**.
**Not yet applied:** the arm calls the in-crate builder. The paragraph keeps its one observable
consequence, that the installers' output reaches the arm's stderr, and states it as the arm's choice of
routing rather than a protocol's constraint. The earlier "not driven by `run-consumer-smoke.sh`" names
the arm.

### (7) Prose, projections and release declarations {mechanical}

- Rewritten to the arm or the builder: `README.md`'s command listing, `gate-sdk/README.md`'s tool roster,
  `guard-kit/SPEC.md`, `drift-kit/README.md`, `drift-kit/SPEC.md` and `canon-kit/SPEC.md`.
  `canon-kit/gate-tests/check-comment-tier/good/good.sh` and
  `gate-sdk/gate-tests/check-test-hermetic-smoke.test.sh` are updated where they spell the retired
  path.
- Regenerate the hooks, whose baked `tree-shell-owed` value moves as two owed files leave, then the SPEC
  and README mirrors and every projection whose freshness gate reds.
- `.workflow/release-declarations.md` bullets. **gate-sdk:** `bin/run-consumer-smoke.sh` and
  `lib/consumer-smoke.sh` are deleted, and the harness is `bash gate-sdk/bin/run-gates.sh
  --run-consumer-smoke`, with the same operands, output and exit codes, so a validate suite or script
  naming the old path must switch. A shell script sourcing the library must stop. **Every kit:** the
  `smoke/` guard's hint text changed, and nothing is required of an adopter.

## Producers and consumers

- **`--run-consumer-smoke` (new arm)** — producer: the binary through the front-end. Consumers: the
  `consumer_smoke` validate suite, which reads the exit status at every validate stage through the
  default parser (`scripts/evidence-config.knobs`), and a session running it by hand. Enabling config:
  the suite command delta 5 writes, which this repo's validate runs, so the producer is reachable in
  the deployed configuration.
- **`csmoke::Scratch` (new return value)** — fields `dir` and `installed`. `dir` is read by every caller
  as the consumer path for the steps after the build, and by each caller's teardown. `installed` is read
  by `--run-consumer-smoke` alone, on its clean line. The two other builder callers ignore it, as they
  ignored `CSMOKE_INSTALLED`. **So `installed` is populated for all three and read by one**, which
  point 4 permits: it is one field computed at one transition, the end of the build, and never
  populated elsewhere.
- **`csmoke::PlaceError` (new)** — variants `NoHost` and `ArtifactUnusable`, each read by its caller's
  environment-failure rendering: `--run-consumer-smoke` exit 2, `--upgrade-smoke` `FAIL(env)`,
  `--agents-md-smoke` refusal, and `--run-demo` `DEMO: FAIL(env)`.
- **`Probe::HarnessError` (new)** — producer: the probe on an absent tree binary. Consumer: the arm's
  accounting loop, which exits 2. `Probe::Exit` is read by the corroboration table as today.
- **Retired producers.** The library's `SCRATCH` and `CSMOKE_INSTALLED` globals, read by the harness and
  by the stdout protocol, both deleted. `csmoke::SOURCE` and `csmoke::spawn`, read by the three arms,
  which delta 3 moves.
- **No narrowing delta.** `port-blockers --tree`'s owed set shrinks. Its only red-bearing reader is
  `check-measured-claim` over a `measured:` literal, which reds on any disagreement and is re-stamped,
  so the change is caught rather than silent.

How the rosters here and below were derived, per SPEC-roster-rederivation.md's delta 2:
`git grep -c "run-consumer-smoke\|consumer-smoke\.sh\|csmoke_\|CSMOKE_"` over the tracked tree without
stderr suppression, a delegated read-only survey of the three arms' spawn seam, the suite
configuration and `check-smoke-entry-guard`'s match literal, and `check-amendment-retired-spelling`'s
survivor report. This is a floor, and the build re-derives it.

## Existing sections updated

- `native/src/emit/csmoke.rs` — the builder replaces the spawn seam (delta 1).
- `native/src/emit/run_consumer_smoke.rs` — the new arm (delta 2).
- `native/src/emit/mod.rs` — the arm-table row (delta 2).
- `native/src/runner.rs` — the front-end usage line (delta 2).
- `native/src/emit/upgrade_smoke.rs` — in-process builder calls (delta 3).
- `native/src/emit/agents_md_smoke.rs` — in-process builder call and its refusal text (delta 3).
- `native/src/emit/demo.rs` — in-process placement (delta 3).
- `native/src/gates/smoke_entry_guard.rs` — its comment, where it quotes the guard's hint (delta 5).
- `gate-sdk/bin/run-consumer-smoke.sh` — deleted (delta 4).
- `gate-sdk/lib/consumer-smoke.sh` — deleted (delta 4).
- `scripts/evidence-config.knobs` — the suite command (delta 5).
- `.claude/settings.json` — two stale allow entries, as a prepared diff only (delta 5).
- `gate-sdk/smoke/install.sh` — the guard hint (delta 5).
- `gate-sdk/smoke/violation.sh` — the guard hint (delta 5).
- `canon-kit/smoke/install.sh` — the guard hint (delta 5).
- `canon-kit/smoke/violation.sh` — the guard hint (delta 5).
- `context-kit/smoke/install.sh` — the guard hint (delta 5).
- `context-kit/smoke/violation.sh` — the guard hint (delta 5).
- `delegation-kit/smoke/install.sh` — the guard hint (delta 5).
- `delegation-kit/smoke/violation.sh` — the guard hint (delta 5).
- `doctrine-kit/smoke/install.sh` — the guard hint (delta 5).
- `doctrine-kit/smoke/violation.sh` — the guard hint (delta 5).
- `drift-kit/smoke/install.sh` — the guard hint (delta 5).
- `evidence-kit/smoke/install.sh` — the guard hint (delta 5).
- `evidence-kit/smoke/violation.sh` — the guard hint (delta 5).
- `guard-kit/smoke/install.sh` — the guard hint (delta 5).
- `lifecycle-kit/smoke/install.sh` — the guard hint (delta 5).
- `lifecycle-kit/smoke/violation.sh` — the guard hint (delta 5).
- `queue-kit/smoke/install.sh` — the guard hint (delta 5).
- `queue-kit/smoke/violation.sh` — the guard hint (delta 5).
- `site-kit/smoke/install.sh` — the guard hint (delta 5).
- `site-kit/smoke/violation.sh` — the guard hint (delta 5).
- `gate-sdk/gate-tests/check-assertion-strength/bad/smoke/install.sh` — the guard hint (delta 5).
- `gate-sdk/gate-tests/check-assertion-strength/good/smoke/install.sh` — the guard hint (delta 5).
- `gate-sdk/gate-tests/check-install-disposition/bad/alpha-kit/smoke/install.sh` — the guard hint
  (delta 5).
- `gate-sdk/gate-tests/check-install-disposition/good/alpha-kit/smoke/install.sh` — the guard hint
  (delta 5).
- `gate-sdk/gate-tests/check-smoke-entry-guard/bad/alpha-kit/smoke/install.sh` — the guard hint
  (delta 5).
- `gate-sdk/gate-tests/check-smoke-entry-guard/good/alpha-kit/smoke/install.sh` — the guard hint
  (delta 5).
- `gate-sdk/gate-tests/check-smoke-entry-guard/good/alpha-kit/smoke/violation.sh` — the guard hint
  (delta 5).
- `gate-sdk/gate-tests/check-smoke-entry-guard/good/beta-kit/smoke/install.sh` — the guard hint
  (delta 5).
- `gate-sdk/gate-tests/check-test-hermetic-smoke.test.sh` — the retired path it spells (delta 7).
- `canon-kit/gate-tests/check-comment-tier/good/good.sh` — the retired path it spells (delta 7).
- `gate-sdk/SPEC.md` — §Consumer smoke, §The port disposition, §run-consumer-smoke, §upgrade-smoke,
  §The non-gate arm and every other sentence naming either file (deltas 2 and 6).
- `context-kit/SPEC.md` — §Testing (delta 6).
- `README.md` — the command listing (delta 7).
- `gate-sdk/README.md` — the tool roster (delta 7).
- `guard-kit/SPEC.md` — the harness mention (delta 7).
- `drift-kit/README.md` — the harness mention (delta 7).
- `drift-kit/SPEC.md` — the harness mention (delta 7).
- `canon-kit/SPEC.md` — the harness mention (delta 7).
- `scripts/git-hooks/pre-commit` — regenerated (delta 7).
- `docs/gate-sdk/SPEC.md` — the SPEC mirror, regenerated (all deltas).
- `docs/gate-sdk/README.md` — the README mirror, regenerated (delta 7).
- `docs/context-kit/SPEC.md` — the SPEC mirror, regenerated (delta 6).
- `docs/guard-kit/SPEC.md` — the SPEC mirror, regenerated (delta 7).
- `docs/drift-kit/README.md` — the README mirror, regenerated (delta 7).
- `docs/drift-kit/SPEC.md` — the SPEC mirror, regenerated (delta 7).
- `docs/canon-kit/SPEC.md` — the SPEC mirror, regenerated (delta 7).
- `.workflow/release-declarations.md` — the bullets (delta 7).
- `docs/posts/2026-07-17-checkwright-v0-4-0.md` — a published release note, deliberately unchanged: it
  records what that release shipped (delta 4).

## Retired spellings

- `run-consumer-smoke.sh` — the deleted harness's filename, every path spelling included (delta 4).
- `consumer-smoke.sh` — the deleted library's filename. The spelling also matches inside the
  harness's, and both retire (delta 4).
- `csmoke_` — the library's function prefix, which covers `csmoke_vendor_and_install`,
  `csmoke_place_binary` and `csmoke_gate_descriptors` (deltas 1 and 4).
- `CSMOKE_INSTALLED` — the library's installed-count global (delta 1).

## Definition of Done

- [ ] **Causal completeness** — the arm, `Scratch`, `PlaceError` and `Probe` each have named readers, and
      every retired producer's readers moved.
- [ ] **Output parity** — before the deletion, the script and the arm run on this tree, and their
      stdout, minus the measured milliseconds and the scratch path, is diffed. It is empty, and the
      commit message records it.
- [ ] **The validate suite green** — `consumer_smoke` passes through the new command, and so do
      `upgrade`, `agents_md_smoke` and `demo`, the three arms delta 3 touched.
- [ ] **Owed count** — `bash gate-sdk/bin/run-gates.sh --emit port-blockers --tree` lists neither file.
- [ ] **Instruction surfaces: instruction only.**
- [ ] **Merged with no information lost** — §Consumer smoke reads whole without this file.
- [ ] **Amendment deleted** — this file removed on merge.
- [ ] **Roster re-derived** — per SPEC-roster-rederivation.md delta 1, with missed sites named in the
      commit.
- [ ] **Removals propagated** — `check-amendment-retired-spelling` green.
- [ ] **Settings diff prepared** — handed to the lead, never applied by hand.
- [ ] **Gaps filed** — through the gap inbox.
