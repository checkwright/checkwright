# SPEC amendment: pins-smoke

Queue entry: `settings-pins-live-suite-coverage`. It leads unit set `evidence-population-fidelity`
under **operator direction, 2026-09-14, lead-relayed**.

## The question, and the premise re-verified

`check-settings-pins` is registered in the consumer smoke's scratch battery, and no suite ever
writes a pins file there. So every battery the smoke runs reaches only the gate's absent-pins
branch, which is a clean skip at exit 0. A vendoring defect that broke the gate against a real
settings file would ship green. The fixture pair does cover the pass and violation branches, but
it runs the gate on a contrived case dir through `--fixture`. It never runs the vendored
descriptor through `gate_command` against a settings file an install wrote, at the knob's
default path.

**Re-verified at this stage, against the tree:**

- `git grep` for `settings-pins.conf` or `CONTEXT_KIT_SETTINGS_PINS` finds them in context-kit's
  library, SPEC, README, gate descriptors and unit tests, in the crate, in the generated hook, in
  the queue and in the docs mirrors. It finds nothing under any `smoke/`, `installer/` or upgrade
  suite. No recipe writes a pins file.
- **The gate's absent-pins clean line already names the skip**
  (`SETTINGS-PINS: clean (no pins file at … — optional consumer config absent)`), so an adopter
  reading their own battery can tell a skip from a pass. The live gap is the smoke's, not the
  gate's reporting. It is also why the smoke must assert the pass branch's own token rather than
  exit 0, because both branches exit 0.
- **A pins file cannot be left standing in the scratch consumer.** `guard-kit/smoke/install.sh`
  *overwrites* `.claude/settings.json`, where context-kit's recipe merges into an existing one.
  The harness installs context-kit before guard-kit, so a pin authored in context-kit's leg would
  name a key the final settings file no longer carries. The gate would then fail closed at exit 2
  on the green-battery assertion. This is the ratchet's own ground (context-kit/SPEC.md §Testing):
  a stamp taken against a half-installed consumer is stale before the battery reads it. The
  overwrite is filed as a gap in its own right. This amendment only works around it.
- **One violation per kit.** `run-consumer-smoke.sh` fires exactly one `smoke/violation.sh` per kit
  (gate-sdk/SPEC.md §Consumer smoke), and context-kit's is `check-brevity`'s. A second bite cannot
  ride that slot without widening the harness contract.

## Which profile owns it

Three homes were weighed. One is taken.

- **Refused: a second violation slot per kit.** That widens `run-consumer-smoke.sh`'s contract and
  its success token for every kit to serve one member, and the pass branch still would not run.
- **Refused: the installer smoke.** `installer/consumer-smoke/run-smoke.sh` answers *does the
  installer deliver the kits*, and gate-sdk/SPEC.md §Consumer smoke rules that the two harnesses
  must not blur. `init` seeds no pins file, and it should not, because a pin is consumer config.
- **Taken: context-kit's own `smoke/install.sh`, exercised in-install and disarmed before the
  installed-baseline commit.** This is the ratchet block's shape in the same file. The member is
  dispatched through `gate_command`, the way a consumer's own battery reaches it, with zero
  consumer config. It sees the settings file this recipe just wrote, at the knob's default path.

## The seam

- **Kit mechanism:** the in-install exercise and its three wanted states.
- **Consumer config:** the pins file. The recipe **derives** its one pin from the settings file the
  install itself wrote, so it asserts nothing about which harness key is worth pinning. It never
  names this repo's own pins (the auto-memory keys), which would re-couple the kit to this repo's
  pin set and cross the provenance seam.
- **Private rule content:** none in reach.

## What changes

### (1) The install recipe drives the pins gate through its pass, violation and skip dispositions {mechanical}

In `context-kit/smoke/install.sh`, after the ratchet block, the recipe exercises
`check-settings-pins` through the file's existing `kit_gate` helper. The steps, in order:

1. **Derive the pin.** Take the first top-level key of `.claude/settings.json` whose name matches
   the path grammar's `ident` and whose value is not `null`. A `null` value is the gate's absent
   branch, so it cannot express a pass. Write one line, `.<key> = <value>`, to
   `scripts/settings-pins.conf`, where `<value>` is `jq -c` of that key's value. That path is the
   `CONTEXT_KIT_SETTINGS_PINS` default the scratch consumer resolves under zero config. If no key
   qualifies, the recipe fails at exit 1 and names the settings file.
2. **Pass.** Keep a byte copy of the settings file, then run the gate. It must exit 0, **and** its
   stdout must carry `1 pin(s) hold`. The absent-pins skip also exits 0, so the exit code alone
   would pass on a knob-resolution defect that never found the file.
3. **Violation.** Rewrite the pinned key's value to `{"consumer-smoke-violated": <original>}`, a
   value that cannot equal the original. Run the gate again. It must exit 1, and its stdout must
   name the pinned path.
4. **Disarm.** Restore the settings file from the byte copy and delete the pins file. Run the gate
   a third time. It must exit 0 with `no pins file` on stdout. That proves the tree the recipe hands
   on is the one it received.

Each mismatch fails the recipe at exit 1 with `context-kit/smoke/install.sh: check-settings-pins
<state>: want <expectation>, got <observation>`, which is the ratchet block's message shape. The
block sits under a `# spec:` line citing context-kit/SPEC.md §Testing. Its one-line reason is that
guard-kit's later install overwrites the settings file, so a pin left standing would desync.

The recipe ends in the tree it had before this block. So the installed-baseline commit, the
green-battery assertion and each violation phase are byte-identical to today's, for all three
callers of `csmoke_vendor_and_install` (`run-consumer-smoke.sh`, the AGENTS.md smoke and the
upgrade suite's TO leg).

### (2) context-kit/SPEC.md states the smoke's pins coverage {mechanical}

**Not yet applied.** In context-kit/SPEC.md §Testing, append to the paragraph beginning
`` `smoke/install.sh` copies the templates into the scratch consumer ``, after the sentence ending
`no \`# smoke-unregistered:\` reason is owed.`:

> It then drives `check-settings-pins` through its pass, violation and skip dispositions the same
> way — through `gate_command`, at the pins knob's default path — on a pin **derived** from the
> first `ident`-named, non-null key of the settings file the install just wrote, so the recipe
> asserts no particular key is pinnable. The pass is asserted on the clean line's pin count and
> not on exit 0, which the absent-pins skip shares. It restores the settings file and deletes the
> pins file before handing on, because a later co-vendored install overwrites the settings file
> and would leave a standing pin naming an absent key.

**Not yet applied.** In context-kit/SPEC.md §check-settings-pins, the sentence
`Ships a \`good/\`+\`bad/\` fixture pair and registers in the consumer's \`gates.list\` (this repo's
included).` becomes:

> Ships a `good/`+`bad/` fixture pair, registers in the consumer's `gates.list` (this repo's
> included), and is driven end to end against an installed settings file by the consumer smoke
> (§Testing).

## Producers and consumers

- **The derived pins file** (scratch consumer, transient). Its producer is step 1 of delta 1,
  reached on every run of `context-kit/smoke/install.sh`, and that recipe runs in every
  `csmoke_vendor_and_install` call that vendors context-kit. The enabling config is the
  `CONTEXT_KIT_SETTINGS_PINS` default, which the scratch consumer resolves with no config written,
  so nothing test-only enables it. Its consumer is `check-settings-pins`, dispatched by
  `kit_gate` through `gate_command` and reading the knob's resolved path. It has one field, the
  pin line, and its reader is the gate's evaluator at each of the three dispatches in delta 1. Its
  deleter is step 4, before the installed-baseline commit.
- **The three wanted states.** Their producer is the gate's exit status and stdout. Their consumer
  is the recipe's comparison. The recipe's red condition is any of: step 2 not exit 0 or no
  `1 pin(s) hold`; step 3 not exit 1 or the pinned path not on stdout; step 4 not exit 0 or no
  `no pins file`; no qualifying key in step 1. The recipe's exit status is read by
  `csmoke_vendor_and_install`'s install loop. That loop reaches the evidence-kit validate suites
  `consumer_smoke`, `agents_md_smoke` and `upgrade`, each read by exit code.
- **Narrowing?** None. The block adds assertions and changes no corpus a reader scans. After
  step 4 the scratch tree is byte-identical to today's, so the harness's registration accounting,
  green-battery assertion and violation phases see unchanged input.

## Existing sections updated

- `context-kit/smoke/install.sh` — the new pins block after the ratchet block (delta 1).
- `context-kit/SPEC.md` — §Testing's `smoke/install.sh` paragraph and §check-settings-pins' fixture
  sentence (delta 2).
- `docs/context-kit/SPEC.md` — generated mirror, regenerated (delta 2).

## Retired spellings

- None — no delta of this amendment retires a spelling; both add assertions and prose beside the
  existing text.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Instruction surfaces: instruction only** — replacement text for a
      template, agent definition or shim carries no grounds; a delta places them
      (the Content-tiering / SSOT rule).
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls context-kit/SPEC-*.md`).
- [ ] **Removals propagated** — every name this change retired is declared in
      `## Retired spellings` above, and `check-amendment-retired-spelling` runs
      each declaration against the whole tracked tree, not against the specs
      alone.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
