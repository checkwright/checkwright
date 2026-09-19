# SPEC amendment: door-binding

Rung 4a (`floor-bash-hooks-front-end`, merged into gate-sdk/SPEC.md
§gen-pre-commit, installer/SPEC.md §init and context-kit/SPEC.md §bin/env-probe)
re-pointed the **starter and prose** adopter surfaces at the gate binary and left
every other kit's shipped procedure naming `bash gate-sdk/bin/run-gates.sh`. This
amendment finishes that sweep.

**The door binding is not an open choice.** The entry offers three candidate
shapes — a knob, a derived path, a substituted literal at vendor time — and the
tree already settled the first: `gate_native_bin()` (`gate-sdk/lib/gate.sh:79-81`)
resolves `GATE_SDK_NATIVE_BIN` through `_gate_prebinary_knob` (env →
`<gates-dir>/gate-sdk-config.local.knobs` → the knob file → the default
`native/target/release/checkwright-gates<exe>`), the PowerShell twin resolves it
identically, and installer/SPEC.md §The install boundary already names it the one
owner: *"The install location has exactly one owner — `GATE_SDK_NATIVE_BIN` in
your `gate-sdk-config.knobs` — and that value is what the battery actually
dispatches to."* `init` already binds its own follow-up text to it
(`native/src/installer/init.rs:363` computes `artifact_dest`, `:754-756` returns
`./<artifact_dest>`).

So the open call is not *which* mechanism but **how each class of surface spells
the one mechanism**, and the classes differ in what can resolve at read time. The
probe (`git grep -n run-gates.sh`, 39 kit-shipped files) separates exactly three:

1. **Shipped shell code**, which can call `gate_native_bin` — `guard-kit/lib/guard.sh`'s
   `_guard_front_end` (`:581`), `context-kit/templates/session-context.sh:11`,
   `drift-kit/templates/kpi-deprecated-surface.sh:11` (which already resolves the
   front-end by walking kit roots, the nearest existing door search).
2. **Harness-wired JSON**, which can resolve nothing — `guard-kit/templates/settings-hooks.json:14,20`,
   `settings-allow.json:5,6`, and the `statusLine`/hook wiring
   `delegation-kit/README.md:51,53,62,69` tells an adopter to write. A harness
   spawns a literal argv; no knob is read on the way.
3. **Prose a session or a reader types** — all eleven kit READMEs, the six stage
   templates, `lead.md`, `agent-execution.md`, `economics.md`, `close-brevity.md`,
   `close-triage.md`, and seven `templates/<kit>-config.knobs` headers.

Class 3 has a landed precedent: `canon-kit/templates/canon-config.knobs:1` already
reads *"the gate binary at `GATE_SDK_NATIVE_BIN` prints the defaults with
`--emit knob-roster`"* — prose cites the knob, never a path literal, which is the
De-literalization rule applied to a door.

**The retirement the entry anticipates does not follow.** The entry expects that
once no adopter surface names the front-end, `run-gates.ps1`,
`--run-front-end-parity` and the fail-open set retire. They cannot, and the
reason is already in the tree rather than new here: gate-sdk/SPEC.md §run-gates
rules that the stub's copy of the fail-open set *"cannot be deleted, because it
is read exactly when the binary it would ask is absent"*. Class 2 is made of
exactly those arms (`FAIL_OPEN_ARMS='--hook --statusline'`), so classes 1 and 3
stop naming the front-end and class 2 keeps it by construction. Delta 5 adds only
the consequence that section does not yet draw, and the residue is filed below.

It is a root-level amendment because it spans five kits' templates
(context, delegation, drift, guard, lifecycle), eleven kit READMEs, gate-sdk's
`lib/` and `bin/`, `native/` (the install-time substitution) and `installer/`.

**The tree does not already do this.** `git grep -n "bash gate-sdk/bin/run-gates.sh"`
returns hits in all eleven kit READMEs, seven knob headers, six stage templates
and both guard-kit settings templates. `canon-kit/templates/canon-config.knobs:1`
is the one file already converted.

## What changes

**Batching.**

- Delta 1 lands alone: it adds the shell accessor every later delta spells.
- Deltas 2 and 3 land together — class 2's substitution and the gate that holds
  it, because the emitted settings and the assertion about them have to agree in
  one commit.
- Deltas 4 and 5 land together: the prose sweep and the limit it stops at.
- Delta 6 rides with 4 and 5, because it is what proves them.

### (1) One spelled accessor for the door, in the library and in the twin {design-bearing}

**Not yet applied.** `gate-sdk/lib/gate.sh` gains `gate_native_bin_spelled`, which
returns `gate_native_bin`'s value already prefixed `./` when it is relative and
carries no `/` leading segment a shell would take as a `PATH` lookup — the same
`./`-prefix rule `init.rs:754-756` applies, promoted from a formatter in the
installer to the library both the shell surfaces and the installer read, so the
two cannot disagree about how a door is spelled. `run-gates.ps1` gains the
PowerShell twin, held by the existing `--run-front-end-parity` comparison.

The three shipped shell surfaces of class 1 call it:

- **`guard-kit/lib/guard.sh`'s `_guard_front_end` (`:581`)** stops printing
  `"$root"gate-sdk/bin/run-gates.sh` and prints the spelled binary. Every
  `guard_block` message that interpolates it (`:615`, `:669`, `:685`, `:1676`)
  therefore names the binary. The function's *name* retires with the spelling
  (§Retired spellings): it no longer names a front end.
- **`context-kit/templates/session-context.sh:11`** drops `RUN_GATES=` and its
  three `bash "$RUN_GATES"` call sites (`:33`, `:35`, `:52`) exec the spelled
  binary. Its `:12` `bash -c 'source gate-sdk/lib/gate.sh; gate_native_bin'`
  subshell collapses into a direct source, because the script already runs under
  bash and the subshell only ever existed to survive a `set -e`.
- **`drift-kit/templates/kpi-deprecated-surface.sh:11`** drops its kit-root walk
  for the library accessor. It is the one class-1 site that already searched, and
  the search is exactly the duplication this delta removes.

**Why the library and not each caller.** `_guard_prebinary_*` and
`gate_native_bin` already live in the two libraries these three scripts source or
can source; a per-caller resolution would be the fourth copy of a precedence
order gate-sdk/SPEC.md §lib/gate.sh owns.

### (2) The harness-wired templates carry a substitution marker, resolved at install {design-bearing}

**Not yet applied.** A harness reads a JSON `command` string and spawns it; no
knob resolves on that path. So the template ships a marker and the install arm
resolves it, the way the generated hooks already bake their per-gate argv at
generation time rather than at run time (gate-sdk/SPEC.md §gen-pre-commit).

`guard-kit/templates/settings-hooks.json` and `settings-allow.json` spell the
door as `@GATE_SDK_NATIVE_BIN@`. The arms that write a consumer's settings —
`--install-hooks` and `init` — substitute the resolved, spelled value as they
copy. A template read by a human still names the knob rather than a path, so the
provenance seam holds: no kit file carries an install location.

**The allowlist entries are the reason this is not a knob read.** `settings-allow.json:5,6`
are `"Bash(bash gate-sdk/bin/run-gates.sh)"` and its `*` twin — a permission
*pattern* the harness matches a literal command against. A pattern naming a knob
would match nothing. So substitution is forced here, not preferred.

**`--hook` and `--statusline` keep naming the front end**, by delta 5's limit.
Their `command` values are the fail-open set, and the marker does not reach them.

### (3) A gate holds the substitution, both directions {design-bearing}

**Not yet applied.** `check-door-binding` (guard-kit, native substrate, with the
`good/`+`bad/` fixture pair the four contracts require) asserts, over the tracked
tree:

- **A.** No kit-shipped surface outside the fail-open set names
  `gate-sdk/bin/run-gates.sh` or `run-gates.ps1` as a command to run. The corpus
  is every kit root's `README.md`, `templates/`, `lib/` and `bin/`, minus
  `gate-tests/` and `smoke/`; the exempted set is the arms
  `check-front-end-fail-open` already declares, read from that gate rather than
  re-listed.
- **B.** Every `@GATE_SDK_NATIVE_BIN@` marker in a kit template is reached by an
  install arm that substitutes it — the reverse direction, so a marker added to a
  template no arm copies cannot sit unresolved in a consumer's settings.

**Red condition, per reader.** A names a file and the line that violates it and
exits 2; B names the unreached marker and exits 2. The clean line prints both
counts, so a corpus that silently shrank to nothing is visible on green — the
failure mode this iteration's sibling unit measured in `--emit enum-sets`.

**Why a gate and not the sweep alone.** Enforcement-first: the sweep without the
gate is one commit of correct text and no reason it stays correct, and the
already-measured evidence that it does not stay correct is rung 4a's own
`canon-config.knobs` conversion, which seven sibling headers did not follow.

### (4) Every prose surface names the binary through the knob {mechanical}

**Not yet applied.** Each class-3 site is rewritten to canon-kit's landed
spelling — the binary named by `GATE_SDK_NATIVE_BIN`, the arm named as the arm,
and no `bash` and no path literal:

- **Seven knob headers** — `context-`, `delegation-`, `drift-`, `evidence-`,
  `guard-`, `lifecycle-` and `queue-config.knobs` line 1, each taking
  `canon-config.knobs:1`'s wording verbatim modulo its own SPEC name (guard-kit
  keeps its extra *"when to set it"* clause).
- **Eleven kit READMEs** — every usage line and every inline instruction.
  `gate-sdk/README.md:17,23,50` describe the front end itself and are rewritten
  to describe what delta 5 leaves it doing, not deleted.
- **Six stage templates** (`lifecycle-kit/templates/stages/*.md`), `lead.md:17,23,37`,
  `upgrade.md:6`, `delegation-kit/templates/agent-execution.md:424,449,490`,
  `drift-kit/templates/economics.md:13,16`,
  `context-kit/templates/close-brevity.md:8,32`,
  `guard-kit/templates/close-triage.md:6,27`.
- **`gate-sdk/templates/gates-workflow.yml:19,44,49,54,60`** — a CI runner, which
  has a checkout and no install, so it resolves the knob's default through the
  library rather than taking a substitution marker.

This repo's own instantiations move with them, because it is a consumer of every
one of these templates: `scripts/delegation-config.knobs:11`,
`scripts/context-config.knobs:17`, `scripts/evidence-config.knobs`' nine command
values, `scripts/canon-config.knobs:78`, `scripts/lifecycle-config.knobs:14-22`'s
nine preflight rows, and `.claude/settings.json`'s allowlist and hook values.
`scripts/gate-sdk-config.knobs:16`'s `GATE_SDK_PORTABILITY_PATHS` **does not** —
it names `gate-sdk/bin/run-gates.sh` as a file whose shell portability is
checked, which is the file, not the door.

Mechanical because every site takes one settled wording and the gate of delta 3
is the oracle for whether the sweep is complete.

### (5) The front end's remaining duty is stated where it is declared {design-bearing}

**Not yet applied.** gate-sdk/SPEC.md §run-gates already owns the reason the
shell half cannot be deleted — *"it is read exactly when the binary it would ask
is absent"* — and this delta adds only what that reason implies once no adopter
surface names the front end, which the section does not yet say: **the stub's
caller set narrows, and narrowing it to zero is unreachable.** After deltas 1 to
4 it is reached by the fail-open arms (`--hook`, `--statusline`) that class 2
keeps, and by a contributor clone before the first build. Both are callers the
sweep does not touch, so the front end stops being an *adopter-facing door* and
becomes a *harness shim plus pre-build door* — a change of role, not of code.

Its `no-port:` declaration is re-phrased to name that role, because the
declaration's ground (it locates the binary, which the binary cannot do for
itself) is unchanged while the audience it serves is not.

The queue entry's sentence anticipating that `run-gates.ps1`,
`--run-front-end-parity` and the fail-open set retire with this sweep is
therefore answered rather than carried forward: they stay, on the ground
§run-gates already states.

`installer/SPEC.md` §The install boundary gains one sentence: the owner sentence
quoted above now also governs what an install-time substitution writes into a
consumer's settings, so the knob is the single owner for a *spawned* door as well
as for the battery's dispatch.

### (6) The consumer smoke executes a substituted door {design-bearing}

**Not yet applied.** `installer/consumer-smoke/run-smoke.sh` gains an arm that,
after `init` on a profile carrying guard-kit, reads the written settings, asserts
no `@GATE_SDK_NATIVE_BIN@` marker survives in them, and **executes** the
substituted hook `command` — proving the value the arm wrote is a path the host
can actually spawn, which a string comparison does not prove.

**Point 5.** The arm reds on a surviving marker, on a `command` whose first token
is not an executable file, and on a non-zero exit from the executed hook other
than the hook's own declared decline status.

**Inferred, cannot run before build:** that the Windows install-smoke leg's
harness settings accept a `./`-prefixed forward-slash path in a JSON `command`
value — no Windows host is reachable from this stage; the `install-smoke-windows`
leg exercises it once delta 2 lands.

## Producers and consumers

- **`gate_native_bin_spelled` (delta 1).** Producer: `gate-sdk/lib/gate.sh`, and
  its PowerShell twin in `run-gates.ps1`. Consumers: `guard-kit/lib/guard.sh`'s
  message constructor, `context-kit/templates/session-context.sh`,
  `drift-kit/templates/kpi-deprecated-surface.sh`, and `--run-front-end-parity`,
  which compares the two implementations by executing them. The enabling
  configuration is `GATE_SDK_NATIVE_BIN`, which `init` writes into every
  consumer's knob file today (`native/src/install.rs`) — deployed, not test-only.
- **The `@GATE_SDK_NATIVE_BIN@` marker (delta 2).** Producer: the two guard-kit
  settings templates. Consumers: `--install-hooks` and `init`, which substitute
  it; `check-door-binding` assertion B, which holds the producer and the
  consumers in lockstep; and the consumer smoke's arm (delta 6), which reads the
  written result. No field is unread: the marker's only value is its own name,
  and its reader is the substituting arm.
- **`check-door-binding` (delta 3).** Producer: its `.gate` descriptor in
  guard-kit's `checks/`, registered in `gates.list`. Consumers: the battery, the
  generated pre-commit hook through the gate's `# graph:` manifest, and the
  fixture pair. Its exempted-arm set is **read from** `check-front-end-fail-open`
  rather than re-listed, so the two gates cannot disagree about the fail-open set.
- **The rewritten prose (delta 4).** Readers: every adopter and every session that
  loads a stage template; `check-docs-cmd`, which resolves invoked paths in the
  governed doc set and today resolves `gate-sdk/bin/run-gates.sh` as a tracked
  file; `check-door-binding` assertion A.
- **§run-gates' stated residue (delta 5).** Readers: `check-front-end-fail-open`,
  whose ground it states; the port oracle (`--emit port-blockers`), which reads
  the stub's `no-port:` declaration against the section its `# spec:` field names;
  a later session asking whether the stub can retire.

**Point 6 — every member of the kit corpus has a satisfying value.** Delta 4's
corpus is every kit root. canon-kit's knob header is already converted and its
README carries two sites (`:63`, `:88`). doctrine-kit, evidence-kit, queue-kit and
site-kit carry README sites only and ship no template naming the door. gate-sdk
is the floor-holder: its README describes the front end rather than instructing a
reader to use it as a door, and delta 4 rewrites those three lines to match delta
5. No kit root is narrowed past.

## Existing sections updated

Rosters produced by `git grep -n "run-gates.sh"` and `git grep -n "run-gates.ps1"`
over the tracked tree (1049 and 34 hits, 263 files, bucketed by directory), by
`git grep -n '](SPEC.md'`-style reads of each kit README, and by reading
gate-sdk/SPEC.md §run-gates and §lib/gate.sh, installer/SPEC.md §The install
boundary and §init, and guard-kit/SPEC.md §The generic ruleset.

- `gate-sdk/SPEC.md` §lib/gate.sh: the accessor roster gains
  `gate_native_bin_spelled` and the `./`-prefix rule it owns (delta 1).
- `gate-sdk/SPEC.md` §run-gates: the stub's residue paragraph and its `no-port:`
  ground, restated as *harness shim plus pre-build door* rather than *adopter
  front end* (delta 5).
- `gate-sdk/lib/gate.sh`, `gate-sdk/bin/run-gates.ps1` (delta 1).
- `guard-kit/lib/guard.sh` (`_guard_front_end` and the four messages
  interpolating it), `context-kit/templates/session-context.sh`,
  `drift-kit/templates/kpi-deprecated-surface.sh` (delta 1).
- `guard-kit/templates/settings-hooks.json`, `guard-kit/templates/settings-allow.json`,
  and guard-kit/SPEC.md §compare-settings-allow, whose allowlist grammar now
  admits a substituted literal (delta 2).
- `native/src/install.rs` and `native/src/installer/init.rs`: the substituting
  copy, and installer/SPEC.md §The install boundary and §init (deltas 2 and 5).
- `scripts/gates.list`, a new `guard-kit/checks/check-door-binding.gate`, its
  native module and its `good/`+`bad/` fixture pair; guard-kit/README.md's gate
  roster block (delta 3).
- The seven `templates/<kit>-config.knobs` headers; the eleven kit READMEs;
  `lifecycle-kit/templates/stages/*.md`, `lead.md`, `upgrade.md`;
  `delegation-kit/templates/agent-execution.md`; `drift-kit/templates/economics.md`;
  `context-kit/templates/close-brevity.md`; `guard-kit/templates/close-triage.md`;
  `gate-sdk/templates/gates-workflow.yml` (delta 4).
- `scripts/delegation-config.knobs`, `scripts/context-config.knobs`,
  `scripts/evidence-config.knobs`, `scripts/canon-config.knobs`,
  `scripts/lifecycle-config.knobs`, `.claude/settings.json` — this repo's own
  instantiations of the templates above (delta 4).
- `CLAUDE.md`: every `bash gate-sdk/bin/run-gates.sh --emit …` capture line
  (knowledge-friction, gap capture, recurrence stamping, survey capture) and the
  battery line under §This repo is governed by its own kits (delta 4). Named
  because BRIEF.local.md §Substrate trajectory rules that a surviving script
  surface is a cost to argue down, *this repo's own always-loaded instruction
  files included*.
- `installer/consumer-smoke/run-smoke.sh` and installer/SPEC.md §The consumer
  smoke (delta 6).
- `.workflow/release-declarations.md` §Behavior changes, appended by the landing
  session: one bullet that a kit's settings templates now carry a substitution
  marker an install arm resolves (delta 2).
<!-- update-target-exempt: generated mirrors and projections, regenerated by their freshness gates' printed commands, never hand-edited -->
- `docs/gate-sdk/SPEC.md`, `docs/guard-kit/SPEC.md`, `docs/installer/SPEC.md`,
  every `docs/<kit>/README.md`, `docs/enforcement.md`, `scripts/CHECK-GRAPH.html`,
  `scripts/git-hooks/pre-commit`.

## Filed at this stage, not built here

On scope-gated intake, filed to the gap inbox with its cost rather than started:

- **The fail-open door's own retirement.** Delta 5 holds the front end for the
  harness-wired arms on a stated constraint. Whether a harness can be wired to a
  *binary* that fail-opens on its own absence — a wrapper the installer places, or
  a harness feature — is a separate design question this unit does not open.

## Retired spellings

<!-- retired-spelling-exempt: the spelling is retired only as an ADOPTER-FACING door; delta 5 rules that the fail-open arms and the pre-build contributor path keep naming it, and gate-sdk's own bin/, lib/, SPEC.md, gate-tests/ and smoke/ keep it as the file's own name — so the surviving surfaces are deliberate and are the set check-door-binding assertion A exempts, not sites the roster missed -->
- `bash gate-sdk/bin/run-gates.sh` — the adopter-facing spelling of the door;
  every kit README, template, knob header and stage procedure names the binary
  through `GATE_SDK_NATIVE_BIN` instead (deltas 1, 2 and 4).
- `_guard_front_end` — guard-kit's message constructor no longer names a front
  end; it prints the spelled binary (delta 1).

## Definition of Done

- [ ] **Causal completeness** — every point of canon-kit/SPEC.md §The
      causal-completeness check holds for each new state, event, interface and
      obligation.
- [ ] **Instruction surfaces: instruction only** — the rewritten templates,
      READMEs and knob headers carry no grounds; deltas 1 and 5 place them in
      gate-sdk/SPEC.md.
- [ ] **Merged with no information lost** — each addition re-phrases the
      canonical-spec text it refines rather than appending to it.
- [ ] **Amendment deleted** — this file removed on merge (`ls SPEC-*.md`).
- [ ] **Entry moved** — `binary-door-wide-sweep` moves to Done in the merge
      commit, a stage before the drain stage.
- [ ] **The sweep is gated, not just done** — `check-door-binding` is registered,
      carries its fixture pair, and reds on a reintroduced adopter-facing
      front-end spelling.
- [ ] **No unresolved marker reaches a consumer** — the consumer smoke's arm
      executes a substituted hook command on the `gates` job's Linux leg.
- [ ] **Removals propagated** — `check-amendment-retired-spelling` runs the block
      above against the tracked tree.
- [ ] **Gaps filed** — the fail-open door's own retirement is filed at spec; any
      cross-component gap build discovers is resolved that session.
