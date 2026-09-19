# SPEC amendment: guard-lib-native-reads

guard-kit's hook library spawns `jq` for every payload read and every decision it
renders, so `jq` is on the floor of every adopter who selects guard-kit
(`jq:::guard-kit` in `native/src/toolfloor.rs:10`, docs/install.md's toolchain
block, installer/SPEC.md's refusal at `:59-65`). This is rung 4b of the
adopter-floor ladder. It moves those reads and renders into the gate binary, and
`lib/guard.sh` stays shell, because it is the API a consumer's own rules are
written against (guard-kit/SPEC.md §The guard framework, its `# no-port:` ground).

The seven `jq` sites and the one presence probe, read off `guard-kit/lib/guard.sh`:

| site | primitive | what it does |
|---|---|---|
| `:23` | `guard_input_field <path>` | one field of the cached payload, `null` printing nothing |
| `:37` | `guard_read_command` | `.tool_input.command // empty` |
| `:51` | `guard_read_path` | `.tool_input.file_path // empty` |
| `:63` | `guard_advise` | renders the advisory envelope |
| `:68-70` | `guard_allow` | renders the allow envelope |
| `:73-77` | `guard_rewrite` | renders the rewrite envelope |
| `:314`, `:324` | `_guard_allow_inners` | probes for `jq`, then reads `.permissions.allow[]?` from `GUARD_KIT_SETTINGS` |

`_guard_allow_inners` runs once per call site, from `:344`, `:1403`, `:1496`,
`:1876` and `:1907`, so one hook call can re-read the settings file several times.

**The library already needs the binary, which is what makes this rung cheap.** Its
sourcing tail sources `gate-sdk/lib/gate.sh` and reads every guard-kit knob from
the binary (`:2063-2077`). When the binary is unreachable, it advises that no rule
ran and exits (guard-kit/SPEC.md §The guard framework, *The load fails in one of
two ways*). So a hook that reaches any `jq` site has already spawned the binary on
that call. Moving the reads adds no dependency, only more calls to a program the
call already ran.

It is a root-level amendment because it spans guard-kit (the library and its
SPEC), the crate (`native/src/guard.rs` and the probe roster in
`native/src/toolfloor.rs`), the install page and the installer's doctor and
consumer smoke.

The tree does not already do this. `grep -n "jq" guard-kit/lib/guard.sh` finds the
eight sites above, and the crate has no arm that prints a payload field for a shell
caller. The `--hook` members read their payload in-process and print a decision
(`native/src/hook/mod.rs:100-148`), which a shell caller cannot use.

## The two questions the filing carried, settled

**Where the code lives: in the crate, beside guard-kit's other compiled half.**
`native/src/guard.rs` already holds the compiled twins of five library primitives
and answers `--guard-lib-parity` (guard-kit/SPEC.md §The guard framework, *Five of
these primitives are held twice*). The new reads and renders join it as a sibling
flag. They reuse two things the crate already has: the hook module's field
renderer and envelope serializer (`hook::field`, `hook::quote`,
`native/src/hook/mod.rs:64-116`), and the allowlist reader
`compare-settings-allow` already runs in-crate
(`native/src/emit/compare_settings_allow.rs:44`, `read_allow`). A second binary,
or code in any other kit, would be a second holder of a serializer the crate
already owns.

**The latency: one spawn per read, and it is cheaper than the `jq` spawn it
replaces.** Measured on a 14-core Linux host, over 50-call loops:

| call | per call |
|---|---|
| `jq -r .tool_input.command` on a small payload | 1.68 ms |
| the binary's `--version` | 0.60 ms |
| the binary's `--hook agent-dispatch-guard`, reading the same payload on stdin | 0.82 ms |
| the whole hook on an allowed `ls`, knobs loaded | 52.4 ms, 3 `jq` spawns inside it |

So each read moves from about 1.7 ms to about 0.8 ms, and the hook's budget is set
by its sourcing and knob load, not by these spawns. **One call per payload was
considered and refused.** `guard_input_field` takes whatever path a consumer's rule
passes at call time, so a single call would need the path set in advance, or would
have to flatten the whole payload into a shell encoding bash then re-parses. Both
cost more design than the under-1 ms they would save per field, and the second is a
second JSON encoding for this library to keep correct.

## What changes

**Batching.** Deltas 1 to 3 land in one commit: the library cannot call a flag
that does not exist, and the roster and the page must change together or
`check-install-toolchain` reds. Delta 4 rides the same commit, because the
consumer smoke's `jq`-less arm asserts the old refusal until it is inverted.

### (1) The binary reads and renders for the library {design-bearing}

**Not yet applied.** The binary gains `--guard-json <mode> [<arg>…]`, a top-level
flag beside `--guard-lib-parity`. It declares no knob, because the library passes
every value it needs as argv or stdin. Its modes:

- **`field <path>`** reads a payload on stdin and prints the value at `<path>`:
  a string bare, a number or boolean as JSON spells it, and nothing for `null`,
  an absent path, an object or an array. `<path>` is the crate's JSON path grammar
  (`native/src/json.rs` `Path`: `.name`, `[N]` and `["key"]` steps), which is jq's
  path-expression subset. A path outside that grammar prints nothing.
- **`field-or-empty <path>`** is the same, except `false` also prints nothing. That
  is `jq -r '<path> // empty'`, which the two headline readers use, and it is
  `hook::field`'s existing rendering.
- **`allow-entries <settings-file>`** prints each string member of the file's
  `permissions.allow`, one per line, and nothing when the file is absent or
  unreadable. It reads through `read_allow`, so a malformed list is read the way
  `compare-settings-allow` reads it.
- **`advise <msg>`**, **`allow <reason>`** and **`rewrite <cmd> <reason>`** print
  the envelope `guard_advise`, `guard_allow` and `guard_rewrite` print today. The
  keys and their order are unchanged, and every interpolated value is serialized,
  never quoted by hand.

Every mode exits 0 except a usage error, which exits 2. Output is LF-terminated on
every host.

**The library's contract does not change.** Each primitive keeps its name, its
arguments, its return status and what it prints. What changes is the program it
spawns:

- `guard_input_field` calls `field`.
- `guard_read_command` and `guard_read_path` call `field-or-empty`.
- The three renderers call their modes.
- `_guard_allow_inners` calls `allow-entries`, and caches the result in a global
  on its first call in a hook process. The settings file does not change within
  one call, and the five call sites stop re-reading it.

The binary's path is read once at the sourcing tail, where `gate_native_bin`
already runs.

**The fail-open posture is the library's own, carried over.**

- A read whose spawn fails, or which prints nothing, is the absent field it
  already was. The caller declines exactly as it did when `jq` failed.
- A render whose spawn fails leaves stdout empty and still exits 0. That is the
  harness's own permission path, the decline a `jq`-less render already produced.
- The unreachable-binary branch at the sourcing tail can no longer call
  `guard_advise`, because that renderer now needs the binary. It prints its
  advisory envelope as a fixed literal instead. The message no longer interpolates
  the vendor root, so it carries no character JSON must escape. A unit test in the
  guard suite parses the literal as JSON. This is the delivery constraint
  guard-kit/SPEC.md already states for a guard degrading because its renderer's
  program is absent, now applied to the binary.

**Honest limit: a consumer rule passing a jq *filter* reads nothing.**
`guard_input_field`'s argument was documented as a `<jq-path>` and forwarded to
`jq` whole, so a consumer copy could have passed something beyond a path (a pipe,
`//`, a function). After this delta such an argument is outside the path grammar
and reads as an absent field, which fails open. No rule in this tree does it: the
one caller outside the library, `guard-kit/gate-tests/guard-read-path.test.sh:61`,
passes `.tool_input.run_in_background`. The SPEC line renames the argument
`<path>` and names the grammar.

### (2) The Windows CR LF absorption retires {mechanical}

**Not yet applied.** guard-kit/SPEC.md §The hook on native Windows rules that
the library absorbs one substrate difference: a native Windows `jq` ends each line
with CR LF. `_guard_lf` turns CR LF back into LF for the three substitution
readers, and `_guard_allow_inners` strips a trailing CR per entry. The binary
writes LF on every host, so neither step has anything left to absorb. Both are
deleted: `_guard_lf` with its three calls, and the `${e%$'\r'}` strip.

The paragraph's accepted exception inverts. A command whose own text carries a
CR LF was read with LF, because the reader could not tell that CR from one `jq`
wrote. It is now read verbatim on every host.

**Point 5, the reader whose red condition moves.** `guard-read-path.test.sh`'s
CR LF cases (guard-kit/SPEC.md `:3481-3482` names the file as holding "the four
`jq` readers to their CR handling") assert that a JSON `\r\n` is read back as LF.
They are rewritten to assert the command is read verbatim, and to assert that no
reader's output carries a CR that was not in the payload.

### (3) `jq` leaves guard-kit's floor {mechanical}

**Not yet applied.** `jq:::guard-kit` in `PROBE_SET` becomes `jq:::contributor`.
`jq`'s remaining users are contributor-side: guard-kit's own smoke recipe
(`guard-kit/smoke/install.sh`, which merges settings with it) and the consumer
smoke harness (`installer/consumer-smoke/run-smoke.sh:71`). docs/install.md's
`jq` bullet reads `(@contributor)` and loses the adopter install lines.

`doctor` skips a contributor member outright (context-kit/SPEC.md §bin/env-probe,
*The owed-predicate*), so it stops naming `jq` in every profile.

`--run-guard-tests` refuses at exit 2 with `jq not found on PATH` because its
subject spawned `jq` (guard-kit/SPEC.md `:3245-3254`). Its subject no longer does,
so the refusal and its paragraph are deleted.

**Point 6, each member of the probe-set corpus.** `PROBE_SET` has six elements. Only
the `jq` element changes, and its satisfying value is `contributor`. The unit test
holding the audience set closed (`toolfloor.rs`,
`every_audience_value_is_closed_over_the_kit_roots`) accepts `contributor`.
`owed_names`' test (`toolfloor.rs:240-245`) expects `jq` in the owed set under a
guard-kit selection, so its expectation drops `jq`.

### (4) The consumer smoke's `jq`-less arm asserts the guard-kit install succeeds {mechanical}

**Not yet applied.** The arm (`installer/consumer-smoke/run-smoke.sh:1011-1080`)
asserts three things today:

- The verbs at the lattice minimum run clean with no `jq`.
- `init` at a profile carrying guard-kit is refused by doctor's floor verdict.
- `doctor` names `jq` unprobed with no install, and omits it inside an install that
  does not owe it.

The second becomes: `init` at that profile runs clean with no `jq`, and the guard
hook it installs answers a payload. The third becomes: `doctor` names `jq` nowhere.
The arm's header comment and installer/SPEC.md §The consumer smoke's sentence
describing it change with it.

**Point 5.** The arm reds when the refusal does not happen. After delta 3 the
refusal cannot happen, so the old assertion would red on every run, and inverting
it is what this delta is.

## Producers and consumers

- **`--guard-json` (delta 1).** Producer: the crate. Its one consumer is
  `lib/guard.sh`, through the seven primitives, and through them a consumer's rules.
  Point 2: the flag is top-level and declares no knob, so no arm-table row, knob
  roster or couples reader lists it. `check-crate-arms`' cargo tests cover the
  module.
- **Each mode's output field.** `field` and `field-or-empty` are read by one
  primitive each (point 4). `allow-entries` is read by `_guard_allow_inners`'
  `Bash(...)` filter, which is unchanged. The three envelopes are read by the
  harness, whose wire shape is unchanged.
- **The cached allowlist global.** Producer: `_guard_allow_inners`' first call.
  Consumers: its later calls in the same process.
- **The literal advisory.** Its reader is the harness, and the parse test holds it.
- **`jq:::contributor` (delta 3).** Readers: `doctor` (skips it), the env-probe arm
  (marks it contributor), `check-install-toolchain` (holds the page bullet to it).

## Existing sections updated

Rosters produced by `grep -n "jq" guard-kit/SPEC.md installer/SPEC.md docs/install.md`,
`grep -rn "jq" native/src/toolfloor.rs installer/consumer-smoke/run-smoke.sh`, and by
reading guard-kit/SPEC.md §The guard framework and §The hook on native Windows.

- `guard-kit/SPEC.md` §The guard framework: the primitive list (`:173`, the
  `<jq-path>` argument becomes `<path>` and names the grammar), and *A third
  posture* (`:523-526`), whose "guard_advise is itself jq-backed" now reads as the
  binary (delta 1).
- `guard-kit/SPEC.md` §The hook on native Windows: the floor sentence (`:646-647`,
  "Git for Windows' bash plus `jq`") drops `jq`, and the CR LF paragraph
  (`:652-663`) is replaced by one sentence saying the binary writes LF (deltas 2
  and 3).
- `guard-kit/SPEC.md`, the fail-open statements naming `jq` as a failure cause
  (`:780`, `:1531`, `:1716`, `:1903`): "no `jq`" becomes "the binary's read fails"
  (delta 1).
- `guard-kit/SPEC.md` `:2338-2342` and `:2722-2725`: the claims that the library
  "still shells to `jq`" and that "guard-kit's floor is unchanged". The library
  stops shelling to it. The smoke recipe still does, as a contributor floor
  (delta 3).
- `guard-kit/SPEC.md` `:3245-3254`, `--run-guard-tests`' `jq` precondition, is
  deleted, and `:3419-3420`, which contrasts a sibling arm with it, is re-phrased
  (delta 3). `:3481-3482` follows delta 2.
- `guard-kit/lib/guard.sh` (deltas 1 and 2), and `native/src/guard.rs` with
  `native/src/main.rs`'s top-level flag list (delta 1).
- `native/src/toolfloor.rs` and its tests, and docs/install.md's toolchain block
  (delta 3). context-kit/SPEC.md §bin/env-probe names no `jq` audience
  (`grep -n jq context-kit/SPEC.md`: `:531` says only that no `jq` floor is
  pinned, which stays true), so it needs no edit.
- `installer/SPEC.md` `:52-65` (the `jq`-less refusal) and `:1079` (doctor's
  sample line), rewritten because doctor no longer names `jq` (delta 3). The unit
  test asserting that line (`native/src/installer/doctor.rs:460`) asserts its
  absence instead.
- `installer/SPEC.md` §The consumer smoke and `run-smoke.sh`'s `jq`-less arm
  (delta 4).
- The `--run-guard-tests` arm in the crate (delta 3).
- `.workflow/release-declarations.md` §Behavior changes: one bullet that guard-kit
  no longer needs `jq`, and one that `guard_input_field` takes a path, not a
  filter, appended by the landing session (deltas 1 and 3).
<!-- update-target-exempt: generated mirrors of the kit SPECs, regenerated by their freshness gate's printed command, never hand-edited -->
- `docs/guard-kit/SPEC.md` and `docs/installer/SPEC.md`.

## Retired spellings

- `_guard_lf` — deleted with its three calls by delta 2.
- `jq:::guard-kit` — the roster element delta 3 re-audiences.

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The
      causal-completeness check holds for each new state, event, interface and
      obligation.
- [ ] **Merged with no information lost** — each addition re-phrases the
      canonical-spec text it refines rather than appending to it.
- [ ] **Amendment deleted** — this file removed on merge (`ls SPEC-*.md`).
- [ ] **Entry moved** — `floor-jq-guard-lib` moves to Done in the merge commit, a
      stage before the drain stage.
- [ ] **No jq in the library** — `grep -n "jq" guard-kit/lib/guard.sh` finds
      nothing, and the guard suite passes with `jq` masked off `PATH`.
- [ ] **Removals propagated** — `check-amendment-retired-spelling` runs the block
      above against the tracked tree.
- [ ] **Gaps filed** — any cross-component gap build discovers is resolved that
      session.
