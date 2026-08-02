# SPEC amendment: native-gate-dispatch-seam

Queue entry: **`native-gate-dispatch-seam`**. Slice 1 of **`native-gate-binary-port`**.

The port's slice 1 is not "port some gates". It is the seam that lets a gate's
**implementation** move to a compiled subcommand while its **declaration** stays
where every existing reader already looks — plus the contract that stops the move
from silently deleting assertions.

That second half is the larger half, and it is why this slice exists at all.
A gate today is one shell file carrying five different things: the rule, the
`# graph:` manifest, the output-contract strings, its `# spec:` and `# assertion`
directives, and the greppable evidence that its reads stay inside its declared
couples. **Nineteen** meta-gates read that file (§Meta-gate conservation for the
binary substrate tabulates all nineteen; the figure is derived from the manifests,
never hand-counted, and this sentence is checked against that table rather than the
other way around). Delete the file and nineteen assertions stop applying — and most
of them stop by finding nothing and printing `clean`, which is a false green, not a
red, because their own file-selection is a literal `*.sh` glob with no `.gate` arm.
The iteration immediately before this one existed to eliminate exactly that class.
A port that reintroduces it at scale, one gate at a time, would be the most
expensive way this repo has yet found to lose coverage.

## What changes

### 1. Resolution splits: a declaration path and an invocation argv {design-bearing}

`gate_resolve` (gate-sdk/lib/gate.sh) is today the single name→path mechanism, and
it hard-codes `<dir>/<name>.sh` behind a literal `-f` test. Its callers use the
result for two unrelated jobs: **reading the gate as text** (the manifest readers,
`check-gate-fixture-coverage`'s `# no-fixture:` probe, `enforcement-map.sh`'s
`tier=` extractor) and **executing it** (`run-gates.sh`, `run-gate-tests.sh`, the
`run_gate` lines `gen-pre-commit.sh` emits).

Those two jobs stop having the same answer, so they become two functions:

- **`gate_resolve`** keeps its name and its signature and keeps returning a
  *declaration path* — the file whose text carries the gate's manifest and
  directives. It gains one arm: within each search dir, `<name>.sh` first, then
  `<name>.gate`. Every text reader is then **unchanged**, because
  `gate_manifest_field` greps `^# graph: ` out of whatever path it is handed and
  has never required the file to be shell.
- **`gate_command`** is new: same name and dir list in, an *argv* out — either the
  one-element `<dir>/<name>.sh`, or the two-element `<binary> <name>`. Its callers
  are exactly the four execution sites above.

Search order is preserved verbatim, which is what keeps registry-plus-shadowing
semantics intact: dirs are tried consumer-first, and `.sh` beats `.gate` **within a
dir**, so a consumer shadowing a kit's ported gate with its own shell script still
wins. A dir carrying both spellings for one name is ambiguous dispatch and is red
(delta 4, assertion A) rather than silently resolved by ordering.

**The binary's path is a knob, not a literal**: `GATE_SDK_NATIVE_BIN`, default
`native/target/release/checkwright-gates`, following the kit convention that every
knob is `<KIT>_<KNOB>` with this repo's layout as the default
(§Layout and configuration). An **absent or non-executable** binary when a
registry member dispatches to it is a harness error — **exit 2, never a skip and
never a pass**. This is the fail-closed contract (§Fail-closed contract) applied to
dispatch: the failure mode a skip would create is a battery that silently stops
running a gate whenever a build is missing, which is the worst available outcome.

### 2. The `.gate` descriptor — the manifest's home for a ported gate {design-bearing}

A gate whose implementation is a subcommand keeps a **non-executable** declaration
file `<dir>/<name>.gate` carrying the lines its readers need: the `# graph:`
manifest (required), its `# spec:` pointer, and an optional `# no-fixture:`
opt-out. Nothing else. It is data, and it is never sourced or run.

**Its existence is the dispatch declaration, and the subcommand name is the gate
name.** There is no dispatch field, no second registry, and no mapping table —
because any of those would be a name that could drift from the thing it names.
Derivation-first: the one fact is the file's presence, and the subcommand is
derived from the name that already identifies the gate everywhere else.

Three alternatives were weighed and refused, each on a verified constraint rather
than on taste:

- **The binary emits its own manifest** (`checkwright-gates manifest <name>`, read
  by `gen-pre-commit.sh`). Refused: `installer/lib/init.sh` runs
  `gen-pre-commit.sh --write` **in the consumer tree**, so hook generation is a
  consumer-side operation. A manifest obtainable only by executing the binary makes
  slice 1 depend on **`gate-payload-disclosure-ruling`** — what a consumer
  receives — which is a deferred companion this slice must not block on. Keeping
  the manifest as text is what makes slice 1 payload-neutral **by construction**.
- **A shell stub carrying the manifest and exec'ing the binary.** Refused, and this
  is the important refusal: a `.sh` stub stays inside the `*.sh` corpora of
  `check-shellcheck`, `check-comment-tier`, `check-gate-output` and the rest, which
  would then scan a three-line file and report `clean`. That is coverage that reads
  as real and asserts nothing — the precise defect class the preceding iteration
  was spent eliminating. A `.gate` descriptor is **honestly outside** those corpora
  instead of trivially passing inside them, and delta 4 is what makes that honesty
  auditable.
- **The manifest moves into `gates.list`.** Refused: it relocates all 94 entries'
  manifests for one slice's benefit, and it ends the co-location of a rule with its
  declared couples — the property `check-reads-couples` exists to police.

### 3. The binary: one multi-call Rust crate at `native/` {design-bearing}

A new tracked root directory `native/` holds a Rust workspace producing one
multi-call binary, one subcommand per ported gate — the busybox shape the parent
entry has carried since filing. It carries no `checks/` and no `smoke/`, which is
the predicate that makes a root directory a kit, so it is correctly not one and
`gate_kit_roots` will not pick it up.

**Rust is settled, not chosen here.** The decision and the alternative it refused
are **`native-gate-language-ruling`**'s to record; this amendment consumes the
ruling and re-opens nothing.

Build output (`native/target/`) is gitignored. Slice 1 commits **no compiled
artifact** — whether this repo runs built artifacts at all is
**`native-gate-dogfood-ruling`**, deliberately not settled here (delta 8).

### 4. Meta-gate conservation — the contract that makes the port honest {design-bearing}

Every registry member whose expanded `couples=` covers a gate-declaration path is
**substrate-sensitive**: porting a gate silently changes what it asserts. That set
is **derived from the manifests, never maintained as a roster** — the same
derivation-first rule that forbids a hand-kept count. Each member of it takes
exactly one recorded disposition, and gate-sdk/SPEC.md gains the section that holds
them (§Existing sections updated):

| Meta-gate | Disposition for a `.gate`-dispatched member |
|---|---|
| `check-shellcheck` | **Retired with cause** — no shell exists to lint. `cargo clippy` at deny-warnings is the substrate equivalent and runs in CI, not as a gate. |
| `check-gate-output` | **Ported and strengthened for the fixtured corpus; source-grep retained for the one member outside it.** Its source-grep for `: clean`/`help:` was always a proxy for behavior; for the 93 of 94 registry members carrying a fixture pair, the assertion moves into `run-gate-tests.sh`: a `good/` case's output must match the clean-line grammar, and a `bad/` case's must carry a `help:` line — applies to **shell gates too**, the port pays a dividend here rather than relaxing. The remaining member, `check-task-conservation` (`# no-fixture:` per queue-kit/SPEC.md §check-task-conservation — a HEAD-vs-worktree diff has no static-fixture representation), has no `good/`/`bad/` case for a fixture-runner assertion to ever reach, so its `: clean`/`help:` lines keep the source-grep as their only oracle. Fully retiring the static check in favor of the runtime one would silently zero out its output-contract coverage — the exact vacuity class this table exists to close. |
| `check-gate-fail-closed` | **Retired with cause** — the defect (branching on a captured value's emptiness when the subprocess died) is unrepresentable once a fallible call returns a `Result` that cannot be ignored. This is a real substrate win and is stated as one. |
| `check-reads-couples` | **Retained, and must fail closed.** Its shell parser finds no walks in a binary gate and would print `clean` — the single worst vacuity in this table. Until the binary-side equivalent exists it **refuses** (exit 2) on a member resolving to a `.gate`, rather than passing. |
| `check-gate-assertions` | **Retained, corpus extended** to the gate's Rust module; the `# assertion` marker is matched on its token, independent of the comment leader. |
| `check-gate-exemption-tasks` | **Retained, corpus extended** the same way. |
| `check-comment-tier` | **Retained, corpus extended** to the Rust module and the `.gate` descriptor. The descriptor's own lines are directives by construction. Mechanism: `canon-kit/lib/spec.sh`'s `spec_comment_surface_with_templates`, the shared corpus primitive this gate calls, gains a `*.gate` arm beside its hard-coded `*.sh` `gate_find` (see the `check-spec-pointer` row below — the two gates share this one function, so the widening is made once, not twice). |
| `check-spec-pointer` | **Retained, and its corpus depends on the same widening as `check-comment-tier`'s** — not "unchanged" in mechanism, only in assertion logic. It calls the *same* shared `spec_comment_surface` (not `_with_templates`) in `canon-kit/lib/spec.sh`, today also a hard-coded `*.sh` `gate_find` with no `.gate` arm; absent that one shared fix, a ported gate's `# spec:` line would silently stop being checked, which is exactly the vacuity class this table exists to close. Once the shared primitive gains the `.gate` arm, check-spec-pointer's own probe logic needs no further change — the descriptor already carries the `# spec:` line. |
| `check-readme-roster` | **Retained, glob widened** to `*.sh` + `*.gate`. Without this a ported gate silently drops out of its kit README's roster in both directions. |
| `check-exec-bit` | **Retained, extended**: a `.gate` descriptor must be **non**-executable. Stated as an assertion so "not executable" cannot read as "not covered". |
| `check-todo-task-liveness`, `check-deprecation-task` | **Retained, corpus extended** to the Rust module and the `.gate` descriptor, the same shape as `check-comment-tier`: both walk `canon-kit/lib/spec.sh`'s `spec_comment_surface` (all tracked `*.sh`, unconditionally) hunting `TODO(task:)`/deprecation markers, so a marker left in a ported gate's Rust source would otherwise silently stop being tracked. |
| `check-knob-default-coupling` | **Retained, corpus extended** to the Rust module for any knob-default literal it declares — its own `gate_find` walks every `*.sh` under each kit root independently of the shared `spec.sh` primitives above, so this is a second, separate widening. |
| `check-gate-tamper` | **Retained, two assertions extended separately**: its `is_gate_file()` glob roster (`DELEGATION_KIT_GATE_FILES`) gains the `.gate` spelling, or a future gate's tamper-isolation exemption goes unchecked; its `extract_exemptions()` parser, which reads a shell `# exception-list:` array literal, has no Rust-source equivalent yet and is out of scope for this slice's one ported gate (`check-spec-fence-balance` carries no exemption list) — flagged here so a later port that does isn't the session that discovers the gap. |
| `check-graph`, `check-kit-enum`, `check-gate-fixture-coverage`, `check-enforcement-fresh`, `check-value-rollup-fresh` | **Survive unchanged** — all five read the declaration path as text (directly, or through `enforcement-map.sh`/`footprint.sh`, which do), which the descriptor still is. |

Gates whose corpus is kit directories, templates, smoke scripts, or hooks
(`check-kit-registration`, `check-template-copy-parity`,
`check-template-registry-parity`, `check-smoke-entry-guard`, `check-hook-exec-bit`,
`check-test-hermetic`, `check-assertion-strength`) are not substrate-sensitive by
the derivation above and are untouched.

**The new gate: `check-gate-substrate-parity`.** Three assertions:

- **(A) Declaration uniqueness** — each `gates.list` member resolves to exactly one
  declaration; a dir carrying both `<name>.sh` and `<name>.gate` is ambiguous
  dispatch and is red.
- **(B) Subcommand parity, both directions** — the set of `.gate` descriptors across
  the resolve dirs equals the binary's reported subcommand roster. A descriptor
  naming no subcommand is a gate that cannot run; a subcommand with no descriptor is
  a gate nothing declares.
- **(C) Disposition coverage** — every derived substrate-sensitive member carries a
  disposition line in the conservation section. **This is the anti-vacuity
  assertion**: a new meta-gate over gate source, added later by a session that never
  read this amendment, reds until its disposition is recorded.

It ships the `good/`+`bad/` pair every registered gate owes, and it stays a shell
gate — a gate that audits the port is not a gate the port may consume.

### 5. The fixture pair executes against whatever the member dispatches to {mechanical}

`run-gate-tests.sh` resolves execution through `gate_command`, so a case runs
`<binary> <name>` where today it runs the script. Its `-x` guard moves onto the
resolved argv's first element. Everything else is unchanged, including the `cd`
into the case dir that is the pair's whole hermeticity story (§run-gate-tests).

This is what makes the parity oracle **executed** rather than merely present, which
`check-gate-fixture-coverage` alone has never asserted. The ordering the parent
entry restores is binding: a ported gate's pair is green **before** the script it
replaces is deleted, never after.

Mechanical: the semantics are delta 1's; this is the call-site change plus running
the fixture battery as the oracle.

### 6. The toolchain floor gains a Rust pin {mechanical}

`context-kit/lib/toolfloor.sh`'s `PROBE_SET` gains the Rust toolchain member the
build needs. It is a **contributor/build** floor, not a runtime one: git remains the
sole runtime dependency, shelled out rather than embedded
(**`native-gate-vendoring-model`** owns that constraint).

Mechanical, and its oracle is `check-install-toolchain`, which holds
`docs/install.md` §Requirements in whole-element parity with `PROBE_SET` in both
directions — so the docs edit is not optional and not discretionary.

### 7. The first ported gate: `check-spec-fence-balance` {design-bearing}

One gate ports in slice 1. Not zero — a seam no member uses is a seam nothing
proves, and this repo has just spent an iteration on assertions that could not
fail. Not a cohort — the disposition table above is the risk surface, and it wants
one instance to be wrong about cheaply.

The selection criterion, stated so build may re-select if the tree has moved:
**registered; carries a fixture pair; `tier=precommit`; and not itself
substrate-sensitive.** The last is not circularity-avoidance pedantry — porting a
gate that audits gate sources would make the parity proof self-referential.
`tier=precommit` is the load-bearing one: it puts the gate in the generated hook,
so a green `check-graph` after the port is an end-to-end proof that the manifest
survived the substrate change, which is the entire question this slice was blocked
on.

`canon-kit/checks/check-spec-fence-balance.sh` satisfies all four (registered at
`scripts/gates.list`; `good/`+`bad/` present; `tier=precommit`; corpus is canonical
specs, not gate sources), and fence balancing is a hand-rolled-parser instance —
one of the four defect classes the parent's correctness ground names.

Its shell script is deleted only once its pair passes against the subcommand.

### 8. What slice 1 does not settle, and why that is a ruling {design-bearing}

Stated because a deferral nobody wrote down is how the four grounds this queue
repaired today got lost in the first place.

- **The dogfood question** (**`native-gate-dogfood-ruling`**) and **the consumer
  payload** (**`gate-payload-disclosure-ruling`**) are one lever seen from two
  ends: what this repo runs, and what a consumer receives. Slice 1 defers **both**,
  and delta 2 is what earns the right to — keeping the manifest as text makes the
  seam work identically whichever way those rule.
- **Vendoring and extensibility** (**`native-gate-vendoring-model`**) is untouched:
  slice 1 ships no artifact and changes nothing about how a kit installs.
- **Opacity is not claimed.** This repo builds from source and the Rust source sits
  readable in-tree, so slice 1's benefit is the seam and the conservation contract,
  not the parent's headline ground. Claiming otherwise here would be the
  "landing the port then relaxing" failure the parent entry warns against, inverted.

## Producers and consumers

**The `.gate` descriptor (new interface).**
Producer: a kit author, at the moment a gate ports — one file committed beside the
script it replaces. Reachable in the real configuration, not test-only: slice 1
produces exactly one (delta 7), and `check-gate-substrate-parity` assertion B reds
if it is ever absent while its subcommand exists. No configuration enables it; its
presence *is* the configuration.
Consumers, each named with the mechanism: `gate_resolve` (path resolution, by
`-f` test) and through it every text reader — `gate_manifest_field`
(`grep -m1 '^# graph: '`), `gen-pre-commit.sh` (`couples`/`trigger`/`mode`/`gen`/
`tier` for the hook projection), `check-graph` (its own inline manifest parse and
the HTML projection), `check-reads-couples`, `check-kit-enum` (`couples=` only),
`enforcement-map.sh` (its `sed` extractor, `tier=` only), `run-gates.sh --for` (the
staged-path matcher), and `check-gate-fixture-coverage` (the `# no-fixture:` grep).
Every one of those reads the file as text and none requires it to be shell — which
is why this delta is a change of *what is at the path* and not of any reader.

**Every field of the descriptor has a named reader**, which is the whole reason it
carries so little: `# graph:` is read by the seven readers above; `# spec:` by
`check-spec-pointer`; `# no-fixture:` by `check-gate-fixture-coverage`. There is no
dispatch field, no subcommand field and no version field — each was considered and
removed for want of a reader, and the subcommand is derived from the gate name
instead (delta 2).

**The invocation argv (new internal state).**
Producer: `gate_command`, once per gate per battery run and once per fixture case.
Consumers: `run-gates.sh` (executes it), `run-gate-tests.sh` (executes it inside
the case dir), and `gen-pre-commit.sh` — which does not execute it but *emits* it,
making the generated hook a persisted consumer. That persistence is why the binary
path is a knob with a stable relative default: a machine-specific absolute path
baked into a tracked generated hook would make `check-graph`'s byte-freshness
comparison machine-dependent, and the hook is byte-compared on every clone.

**`GATE_SDK_NATIVE_BIN` (new knob).**
Producer: `gate-sdk/lib/gate.sh`'s default assignment, overridable per consumer
exactly as every other kit knob. Reader: `gate_command`, at resolution. Its
default is this repo's layout, per the config-via-env convention.

**The conservation section (new contract obligation on every kit).**
Producer: gate-sdk/SPEC.md's new §Meta-gate conservation, as the statement a gate
author writes against. Consumer: `check-gate-substrate-parity` assertion C, which
derives the substrate-sensitive set from the manifests and reds on any member the
section does not name — so the obligation is machine-held, not remembered. This is
the delta that makes slice 1 cross-component in *contract* as well as in code.

**Whole-component-set reader survey.** The surfaces that read a gate's declaration
or execute it are: `gate-sdk/lib/gate.sh`, `gate-sdk/bin/` (`run-gates.sh`,
`run-gate-tests.sh`, `gen-pre-commit.sh`, `enforcement-map.sh`,
`install-hooks.sh`), `gate-sdk/checks/` (the meta-gates tabulated in delta 4),
`canon-kit/lib/spec.sh` (`spec_comment_surface` and `spec_comment_surface_with_templates`,
the shared corpus primitive `check-spec-pointer` and `check-comment-tier` both call —
named explicitly because delta 4's table gives these two gates different-looking
dispositions that resolve to one shared fix), `canon-kit/checks/`
(`check-comment-tier`, `check-spec-pointer`, `check-todo-task-liveness`,
`check-deprecation-task`, `check-knob-default-coupling`), `delegation-kit/checks/`
(`check-gate-tamper`), the generated
`scripts/git-hooks/pre-commit` and `commit-msg`, `scripts/gates.list`,
`installer/lib/init.sh` (which regenerates the hook consumer-side), and the
`gates` CI workflow. Build re-runs this survey against its own HEAD before
implementing — the roster above is dated the moment it is written — and runs every
path probe with **stderr unsilenced**: a `2>/dev/null` on a mistyped path reads a
live reader as absent, which on this change would mean shipping a seam that
strands a reader nobody noticed.

## Existing sections updated

- **gate-sdk/SPEC.md §lib/gate.sh** — `gate_resolve`'s contract gains the `.gate`
  arm and the within-dir precedence rule, and states that it returns a
  *declaration* path. `gate_command` is specified beside it as the invocation
  resolver, with the fail-closed exit-2 rule for an absent binary (delta 1).
- **gate-sdk/SPEC.md §The `# graph:` manifest** — the manifest's home is stated as
  "the gate's declaration path", not "the gate script", and the `.gate` descriptor
  is named as the second spelling of that path. The grammar itself is unchanged
  (delta 2).
- **gate-sdk/SPEC.md §Layout and configuration** — `GATE_SDK_NATIVE_BIN` joins the
  knob roster with its default; the name-resolution paragraph gains the `.gate`
  spelling so the registry contract reads correctly in one place (deltas 1, 3).
- **gate-sdk/SPEC.md §Meta-gate conservation for the binary substrate** — new
  section, holding delta 4's disposition table and the derivation of the
  substrate-sensitive set. This is the section assertion C reads.
- **gate-sdk/SPEC.md §check-gate-substrate-parity** — new per-component contract
  section, assertions A/B/C, matching the `# assertion` markers
  `check-gate-assertions` will demand (delta 4).
- **gate-sdk/SPEC.md §Output contract** — the clean/`help:` grammar becomes a
  *runtime* assertion of the fixture runner rather than a source-grep proxy for
  the 93 fixtured members; it cites §run-gate-tests for the mechanism instead of
  restating it, and states the one `# no-fixture:` member's carve-out (the static
  source-grep is its only oracle and stays) so the contract's coverage is total
  across all 94, not 93-of-94 (delta 4).
- **gate-sdk/SPEC.md §run-gate-tests** — the invocation resolves through
  `gate_command`; the new output-grammar assertions on `good/` and `bad/` cases are
  specified here, since this section already owns the runner's contract
  (deltas 4, 5).
- **gate-sdk/SPEC.md §check-reads-couples** — the refuse-rather-than-pass rule for a
  member resolving to a `.gate`, stated as a fail-closed arm with its cause
  (delta 4).
- **gate-sdk/SPEC.md §check-readme-roster, §check-exec-bit** — the widened glob and
  the non-executable-descriptor assertion respectively (delta 4).
- **gate-sdk/SPEC.md §check-shellcheck, §check-gate-fail-closed** — each gains the
  one-line statement that a `.gate`-dispatched member is out of its corpus **with
  cause**, citing the conservation section rather than restating the reasoning
  (delta 4, one owner per fact).
- **canon-kit/SPEC.md §check-comment-tier** — the governed-source corpus reaches the
  Rust module; the comment-leader-independent marker match is stated where the
  directive grammar already lives (delta 4).
- **canon-kit/SPEC.md §lib/spec.sh** — `spec_comment_surface` and
  `spec_comment_surface_with_templates` gain the `.gate` arm beside their hard-coded
  `*.sh` `gate_find`, stated once at the shared primitive rather than twice at its
  two callers (`check-spec-pointer`, `check-comment-tier`); `check-todo-task-liveness`
  and `check-deprecation-task` inherit the same widening through the same call
  (delta 4).
- **context-kit/SPEC.md §bin/env-probe** — the new `PROBE_SET` member and, beside
  it, the construct that forces the floor, which that section requires of every
  member (delta 6).
- **docs/install.md §Requirements** — the toolchain bullet, held in whole-element
  parity with `PROBE_SET` by `check-install-toolchain` in both directions (delta 6).
- **CLAUDE.md §Housekeeping** — one line for `native/`: what it holds, that it is
  not a kit (no `checks/`, no `smoke/`), and that its build output is gitignored
  and never committed. `scripts/root-allowlist.list` gains `native/` in the same
  commit, which is `check-root-tiering`'s intentional-new-surface valve (delta 3).
- **README.md §This repo, governed** — the per-kit fixture battery gains the native
  crate's own test command, so a contributor's pre-commit routine covers the new
  substrate rather than only the shell one (delta 3).
- **`.github/workflows/gates.yml`** — a toolchain-setup step and a release build
  before the battery, since a registry member now dispatches to a build artifact.
  The action is pinned by SHA under `check-action-pinning` like every other
  (deltas 1, 3).
- **docs/site-architecture.md §Generated projections** — `check-gate-substrate-parity`
  is a new gate, which stales the full new-gate fan-out that section already
  rosters (SPEC mirror, enforcement map, footprint, value rollup, `check-graph`,
  and the generated hooks for a `tier=precommit` gate). No new projection is added;
  build regenerates from the rostered commands rather than re-deriving them (delta 4).

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`).
- [ ] **Removals propagated** — grepped every spec for names this change
      retired; nothing dangles.
- [ ] **Gaps filed** — cross-component gaps discovered during the work filed as
      debt tasks (a build-time causal gap is resolved that session, not
      deferred).
