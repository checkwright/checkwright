# SPEC amendment: graph-port-and-config-seam

Ports `check-graph` to the binary substrate and discharges the operator ruling
of 2026-08-20 (TRAJECTORY.md §The closed rulings) that the graph gate's consumer
config becomes a **data-only contract** and the sourced-function seam retires.

**This is a breaking change to a shipped consumer seam, and it is stated rather
than presented as a clean port.** It narrows asserted behavior for every consumer
carrying a `graph-theme.sh`: after this amendment such a file is not read, and
the theme it holds is not emitted. It also re-cuts the seam CLAUDE.md §The
provenance seam names as the provenance doctrine's own worked example. What the
ruling binds the replacement to, and what this amendment holds to, is that the
doctrine stays true through the cut — the vocabulary and the theme stay the
consumer's, and only their **form** moves from executable to declarative.

## What changes

### (1) `check-graph` dispatches to the binary; its emitter becomes a non-gate arm

`gate-sdk/checks/check-graph.sh` is replaced by `gate-sdk/checks/check-graph.gate`
dispatching to `native/src/gates/graph.rs`, the shell original deleted, and
`emit_graph` moves to `native/src/emit/graph.rs` registered in `emit::EMITTERS`
as the projection `graph` (arm `--emit-graph`). **Design-bearing.**

The gate carries assertions A, B, C, E, F, G, H and I — manifest
well-formedness, `couples ⊆ trigger` parity, the cycle-valve rule, graph-artifact
freshness, emitted asset hrefs, amendment-body manifests, the external-ref
allowlist, and the render cap. Assertion D — hook parity — is treated by delta (7).

The `--amend-only`, `--refs-only` and `--cap-only` modes port unchanged: each
selects the *rule's own input corpus* rather than redirecting config the bridge
has already resolved, which is gate-sdk/SPEC.md §The non-gate arm's distinguishing
test, so none of the three is a deletion.

### (2) The theme seam becomes a directory of verbatim part files

`GATE_SDK_GRAPH_THEME` and the three `graph_theme_*` functions are **retired**.
In their place `GATE_SDK_GRAPH_THEME_DIR` (default `<gates-dir>/graph-theme/`)
names a directory of at most three optional part files, each inlined **byte
verbatim** at the injection point the retired function fed. **Design-bearing.**

| part file | injection point | absent |
|---|---|---|
| `theme.css` | the `<style>` element's body, replacing the kit default stylesheet | kit default stylesheet |
| `header.html` | directly after `<body>`, above the kit header | nothing emitted |
| `footer.html` | directly before `</body>` | nothing emitted |

Fallback semantics are unchanged from the retired seam: an absent directory or an
absent part falls back exactly as an undefined function did, so a themeless
consumer's output stays byte-identical. The kit neither adds nor strips a
trailing newline — a part file's own bytes are what appear — which is what makes
the migration a move of the heredoc body rather than a re-authoring of it.

**Why a directory of files rather than bridged values.** The config bridge
(gate-sdk/SPEC.md §lib/gate.sh) refuses any element containing a newline, exit 2,
because the newline would break the line-per-element argv protocol. A stylesheet
and two HTML fragments are newline-bearing by construction, so theme *content*
cannot ride the bridge at all. Only the **path** crosses, relative, and the
binary reads the files itself. This is the general rule the amendment contributes
to §lib/gate.sh, because delta (3) takes the other branch of it: **values cross
the bridge; documents cross as a path.**

**Why not the two alternatives the ruling already refused**, restated here only
so the port does not re-reach for them: a shell shim kept for theme emission adds
a shell-only install step the interpreter policy forbids outright, and shipping
the artifact unthemed regresses a shipped capability.

**The retired seam fails loudly rather than silently.** The gate refuses, exit 2,
naming the migration, when `GATE_SDK_GRAPH_THEME` is set in its environment or a
file exists at `<gates-dir>/graph-theme.sh`. The tripwire is permanent kit weight
and it is worth it: a themed consumer that silently loses its theme produces a
regenerated artifact the byte-compare cannot distinguish from a legitimate theme
edit, so the failure would be invisible in a green battery — the exact shape
gate-sdk/SPEC.md §Fail-closed contract exists to refuse.

### (3) The vocabulary keeps its shell file and crosses the bridge; the layer hook becomes data

`graph_surface_layer()` is retired. The consumer's vocabulary file keeps its
name, its location knob (`GATE_SDK_GRAPH_VOCAB`) and its four existing globals,
and gains two: **design-bearing.**

- `GRAPH_LAYER_RULES` — an **ordered** array of `<path-prefix>:<layer-id>`
  elements, split at the **last** `:` (a layer id is `[A-Za-z0-9_]+`, a prefix
  may contain `:`), **first match wins**.
- `GRAPH_LAYER_DEFAULT` — the layer id for a surface no rule matches. Kit
  default `surfaces`, which is the value the retired hook's absent-function
  branch returned.

`lib/gate.sh` sources the vocabulary file at top level — the shape it already
uses for `GATE_SDK_MSG_PATTERN_FILES` — and guarantees all six globals are
defined, so the bridge's does-not-define refusal cannot fire on any of them and
an *absent* vocabulary file resolves to empty arrays that disable their checks
exactly as today. `GATE_SDK_GRAPH_VOCAB` is therefore **not** a knob the crate
declares: the crate receives the resolved values, never the path.

**The rule is a prefix test, not a glob, and that is a deliberate narrowing.**
The retired hook was a bash `case`, whose `*` crosses `/`; the crate carries a
component-wise matcher and a slash-spanning one side by side and nothing says
which a port reaches for (`couples-glob-semantics-unowned`). A prefix test has no
glob semantics to own, expresses every rule the live consumer's hook expressed,
and closes this surface's exposure rather than adding a fourth reader of an
unowned question. This repo's `scripts/graph-vocab.sh` migrates to seven prefix
rules plus `GRAPH_LAYER_DEFAULT=k_shared`, reproducing the hook exactly.

### (4) The graph knobs get bridge-visible defaults in `lib/gate.sh`

`GATE_SDK_GRAPH_ARTIFACT`, `GATE_SDK_GRAPH_THEME_DIR` and
`GATE_SDK_GRAPH_MAX_EDGES` move from inline `${KNOB:-…}` defaults at their use
sites to guarded top-level assignments in `gate-sdk/lib/gate.sh`, riding
`GATE_SDK_GATES_DIR`'s own resolved value so the pair stays one value by
construction. **Mechanical.**

`GATE_SDK_GRAPH_EXTERNAL_REFS` is a whitespace-separated scalar feeding an array,
which §lib/gate.sh rules is the one case a resolved global earns a spelling of
its own, so it resolves to the array `GATE_GRAPH_EXTERNAL_REFS` beside
`GATE_PRUNE_DIRS`. Every one of these values stays **relative**: the resolved
argv is baked verbatim into the tracked pre-commit hook, and an absolute value
would commit one machine's checkout path to a public file.

### (5) `upgrade-smoke.sh` reads the resolved artifact knob

`gate-sdk/bin/upgrade-smoke.sh`'s hard-coded `scripts/CHECK-GRAPH.html` becomes
the resolved `GATE_SDK_GRAPH_ARTIFACT` that delta (4) puts in the library the
smoke already sources. **Mechanical.**

This discharges `upgrade-smoke-graph-artifact-literal` through neither of the two
dispositions that entry could see: it neither duplicates the default expression
nor mints an arm on a gate for one caller. Delta (4) is required by the bridge
independently, and it leaves exactly one place the path is computed — which is
what the entry wanted.

### (6) Assertion B's `couples ⊆ trigger` semantics are stated, then ported verbatim

`check-graph` assertion B is the **third** reader of `couples=` and it invokes no
glob matcher: it tests exact-token subset membership against `trigger=` through a
four-branch predicate — a `*` trigger covers everything; an exact string match
covers; a *literal* couple (one carrying no `*` or `?`) covered by the trigger
read as a bash pattern covers; and a `*.<ext>` trigger covers a couple with that
suffix. Those four branches are stated in gate-sdk/SPEC.md §check-graph by this
amendment and reproduced **verbatim** in the port. **Design-bearing.**

The port must not reach for either crate matcher. Neither is this predicate:
the component-wise matcher requires equal segment counts and the slash-spanning
one is a bare bash-pattern test, and substituting either flips verdicts on the
live registry. The globstar commitment at gate-sdk/SPEC.md §The port-candidate
criteria (criterion 6) governs *a Rust glob matcher over a bridged knob* and
does not reach a predicate that matches no glob — stated because it is the first
ruling a porting session will find and it is the wrong one for this reader.

This closes the **port's** exposure to `couples-glob-semantics-unowned` and
closes that entry's undocumented-third-semantics half. It does **not** close the
entry: whether `couples=` has one semantics with stated exceptions or a
per-reader meaning that must be declared per reader is untouched, and that
question is the entry's remaining deliverable.

### (7) The hook emitter stays shell, with a stated cause; assertion D keeps its spawn

`gate-sdk/bin/gen-pre-commit.sh` **does not port**, and check-graph's assertion D
keeps spawning it for `--emit` and `--emit-commit-msg`. **Design-bearing.**

The cause is structural rather than a sizing judgment. The generated hook bakes
the **resolved** invocation argv verbatim — `env GATE_SDK_KNOB_…=… <binary>
<name>` — and resolving a knob means sourcing the owning kit's `lib/*.sh` in a
subshell and reading `declare -p`. gate-sdk/SPEC.md §lib/gate.sh rules there is
exactly **one** place a knob's value is computed, the kit's shell library; a
crate-side hook emitter would have to be the second, which criterion 6 refuses.
The generator is therefore not an unported emitter awaiting a cohort — it is the
config bridge's own producer, and it stays where the bridge is.

Criterion 7 clears the spawn explicitly: *a rule shelling out to `bash <emitter>`
clears this criterion, because `bash` is on the floor, however unported that
emitter is.*

**The alternative was costed and refused, and it is recorded so the next reader
does not re-derive it.** Assertion D could move to a **new shell gate** owning
hook parity alone, under born-native exception class (a) — a gate asserting that
the persisted dispatch matches its generator is auditing the dispatch relation,
and a compiled form would compute both sides of that comparison through the very
binary under test. That reading is sound and the split is available. It is
refused here because it mints a gate name, a descriptor, a fixture pair and a
SPEC section to relocate an assertion criterion 7 already sanctions in place, and
because this iteration already carries a breaking seam change and a second unit.
The residue it leaves is real and is filed rather than absorbed: the ported
`check-graph` becomes the crate's **first** gate to spawn a program other than
git, which runs against the direction of TRAJECTORY.md §The objectives 1, 2 and
6. The filed entry carries the split as the designed-but-unbought answer
(`graph-port-bash-spawn-residue`).

**Ratified by the operator, 2026-08-21.** This refusal was ruled at this stage;
the operator has since ratified the cost and the refused alternative on the
same grounds, which stands as a stronger record than the stage's own ruling
alone.

### (8) The fixture corpus widens *before* the port — criterion 4's discharge

**Criterion 4 binds on `check-graph`, and this amendment rules it rather than
leaving it excluded-before-reached.** The gate resolves every `gates.list`
member's declaration path and reads its bytes for the `# graph:` line, so a
registry member's declaration path lies inside the corpus the gate scans as
content, which is criterion 4's predicate exactly. It binds under every
configuration — there is no consumer config in which the registry is not the
corpus — so this joins `check-gate-exemption-tasks` as a member whose verdict
flips on nothing. **Design-bearing.**

Criterion 4's discharge is the fixture corpus, and its condition is that the pair
carry **every arm of the derivation being ported**. Measured at this stage on
2026-08-21, the pair carries **one** arm of nine: `good/args` and `bad/args` are
both `--amend-only`, so only assertion G is fixtured, and every other assertion
rests on a live tree that is green because it is clean. Widening is therefore the
port's largest single piece of work, and it happens **first** — a port answering
only the criterion's sentence ships the hole the sentence was pointing at.

The widening splits by assertion shape:

- **The `good/`+`bad/` pair carries assertion G, and only G.** The pair runs one
  argv per case, so it selects exactly one of the gate's modes, and only
  `--amend-only` reaches an assertion without also reaching the two the pair
  cannot satisfy. **Corrected at build, 2026-08-21, against a run rather than a
  reading**: a case dir *is* a mini-consumer root as this delta said, and A, B and
  C do run clean in one — but a `good/` case must exit 0 and the full path always
  reaches D and E, which red there on two counts a fixture may not answer.
  `gen-pre-commit.sh` cds to `git rev-parse --show-toplevel` **before** resolving
  the gates dir, so `--emit` invoked from a case dir emits *this repo's* hook;
  satisfying D would take a committed byte-copy of the real generated hook inside
  `gate-tests/`, and satisfying E a committed `CHECK-GRAPH.html` beside it —
  both the second-copy-of-a-generated-file this delta already refuses below. The
  alternative, a fourth mode selecting the registry assertions, is refused: it
  adds consumer-visible surface and a fourth arm to delta (1)'s port, to buy a
  corpus a driver already has. So G's widening is the pair's whole job — every
  branch of `validate_amend_manifest` and of the extractor that feeds it.
- **Every other assertion — A, B, C, D, E, F, H, I — takes a constructed
  scenario in a `gate-tests/*.test.sh` driver.** A committed expected hook or
  artifact inside `gate-tests/` is a second copy of a generated file that rots, so
  these generate and compare inside a throwaway tree instead. That tree is
  criterion 4's discharge on the same ground the pair is: it is written by a file
  under `gate-tests/`, which every live-tree walk prunes, so no port can add or
  remove a file in the corpus the comparison runs over. **Corrected against the
  fixture tree, 2026-08-21 at align: only H and I already have hermetic drivers**
  (`check-graph-refs.test.sh`, `check-graph-cap.test.sh` respectively) —
  neither exercises assertion F (`check-graph.sh:600-605`, every emitted local
  `href`/`src` resolves under the artifact dir), which has none today. Assertion
  F needs its own driver for the same reason H's does: the kit's own default
  emission is self-contained by construction — it emits no local `href`/`src` at
  all — so the failure path (a theme injecting a relative href to a file that is
  not there) and its clearing counterpart are untested until a theme supplies one.
  Assertion D needs a driver because `gen-pre-commit.sh` anchors itself at
  `git rev-parse --show-toplevel` and so cannot be driven from a non-repository
  case dir at all; assertion E needs one for the artifact byte-compare.
- **The driver is `check-graph-tree.test.sh`, delivered at build 2026-08-21**,
  and it is one tree rather than one per assertion because A through F share a
  single mini-consumer: a `mktemp`'d git repo carrying `scripts/gates.list`, five
  member declarations in **both spellings**, and `scripts/graph-vocab.sh`. The
  `.gate` member is `tier=align-only`, which keeps it out of both generated hooks
  and so out of `gate_command`'s reach — that is what lets the sandbox exercise
  the descriptor branch of assertion A's resolution with **no binary present**,
  rather than making a shell-substrate fixture depend on a built crate. The
  baseline case is the parity oracle for the branch set: it greens only if all
  four of delta (6)'s coverage branches and all three cycle-valve branches clear,
  so a branch that stops firing reds rather than passing vacuously.
- **The three existing drivers reach the gate by script path and must be moved to
  `gate_run`, found at build.** `check-graph-refs.test.sh`,
  `check-graph-cap.test.sh` and `check-graph-theme.test.sh` each invoke
  `"$GATE"` = `checks/check-graph.sh` directly. gate-sdk/SPEC.md §run-gate-tests
  rules that a bespoke test names a gate and a checks dir, never a script path,
  **precisely so a member's port leaves its behavioral tests untouched** — so as
  written all three stop testing anything the moment the shell file is deleted.
  Re-pointing them onto `gate_run` is part of the port, not a cleanup, and it is
  the same edit in all three files.
- **`check-graph-theme.test.sh` is re-pointed** from the retired sourced-function
  seam onto the part-file contract, keeping its four existing claims: injected
  content lands, an absent theme falls back with no leakage, both paths are
  byte-deterministic, and the kit body survives theming. It gains a fifth: the
  retired-seam tripwire of delta (2) refuses.
- **`check-graph-refs.test.sh` is not theme-neutral and needs the same
  re-pointing, found at this stage.** It injects its un-allowlisted chrome link
  through `graph_theme_footer()` sourced via `GATE_SDK_GRAPH_THEME` (lines
  23–28) — the same retired seam, not merely a similar one — so under the port
  it would hit delta (2)'s tripwire (exit 2, naming the migration) on every one
  of its three cases instead of exercising assertion H at all. Its fixture must
  move to a `GATE_SDK_GRAPH_THEME_DIR/footer.html` part file carrying the same
  link, alongside the theme test's migration rather than as separate work.
- **`check-graph-tree.test.sh` joins that migration, for assertion F only.**
  Because the kit emits no local `href`/`src` of its own, F's failure path is
  reachable through the theme and nowhere else, so its two cases inject a
  `graph_theme_header()` and are written against the seam that exists when they
  land. Post-port the injection becomes a `GATE_SDK_GRAPH_THEME_DIR/header.html`
  part file. The three re-pointings are one migration over three files, and the
  widening lands **first** by this delta's own sequencing — a driver written
  against a contract the tree does not yet carry is a driver that runs nothing.

**The live-tree parity arm is demoted from proof to smoke**, per criterion 4's
own rule for a member whose assertion target is gate source: assertion A forbids
a descriptor and a script coexisting in one resolve dir, so the cross-substrate
comparison necessarily runs on the pre-descriptor tree, which the port then
changes. Its verdict is recorded as **no disagreement found on the pre-descriptor
tree**, never as parity proved.

### (9) §check-graph's runtime paragraph is corrected against measurement

The paragraph at gate-sdk/SPEC.md §check-graph claiming *the port makes those
calls in-process, the dividend the freshness family already banked* is **false
under delta (7)** and is rewritten rather than left standing. **Design-bearing.**

Measured at this stage on 2026-08-21 against this tree: the full gate runs
7591 ms, of which `gen-pre-commit.sh --emit` is 5685 ms and
`--emit-commit-msg` 213 ms — so 78% of the member's runtime is the two spawns
that do not port, and the 5.7 s is the config bridge resolving argv for every
registered member. What the port does bank is the graph emission and the
per-member manifest read, measured at 1462 ms for `--emit` alone. The corrected
paragraph states the split and drops the second independent argument for this
member's port; the operator ruling of 2026-08-09 is the argument that stands, and
it needs no other.

The bridge's own resolution cost is a finding this member surfaced and does not
own — no queue entry covers it, and `gate-battery-parallel-execution` and
`gate-battery-result-cache` are about the battery rather than the bridge. It is
filed rather than carried.

### (10) Descriptor, registration and the port's remainder accounting

`check-graph.gate` keeps the current `# graph:` manifest's `couples=`, `dir=`,
`valve=` and `tier=` and gains the crate modules the gate reaches in-process —
`native/src/emit/graph.rs` among them — per §The non-gate arm's transitive-coupling
rule, so an edit to the emitter re-fires its comparator. **Mechanical.**

The generated pre-commit hook, `scripts/CHECK-GRAPH.html`, `docs/check-graph.html`
and the enforcement-map projections regenerate; `docs/site-architecture.md`
§Generated projections carries the graph artifact's regen command and updates
with it, since the command's spelling moves from the gate's `--emit` to the
emit-arm front-end.

The port-track remainder moves as follows, and the numbers are read off
`bash gate-sdk/bin/port-blockers.sh --group` at the build cut rather than from
this line: the **takeable** tier empties, and the count of members still owed
falls by one. `native-gate-port-remaining-corpus` demotes on the
entry-outlives-the-amendment branch rather than moving to `## Done`.

## Producers and consumers

**`GATE_SDK_GRAPH_THEME_DIR`** (new knob, scalar, relative path).
*Producer:* the guarded top-level assignment in `gate-sdk/lib/gate.sh`, resolved
by `gate_command`'s config bridge into the argv of every `.gate` member and every
emit arm declaring it — so it is emitted by the live dispatch path, not only
under test. *Consumers:* `native/src/emit/graph.rs`, which reads the three part
files at each emission, and through it `native/src/gates/graph.rs` assertion E.
*Reader of each part:* `theme.css` at the `<style>` body, `header.html` after
`<body>`, `footer.html` before `</body>` — the three transitions delta (2)
tabulates. A part file with no injection point is not defined; the roster is
closed at three.

**`GRAPH_LAYER_RULES`, `GRAPH_LAYER_DEFAULT`** (new consumer-file globals).
*Producer:* the consumer's vocabulary file, sourced by `gate-sdk/lib/gate.sh` at
top level and defaulted there so the bridge's `declare -p` always finds them.
*Consumer:* `native/src/emit/graph.rs`'s layer lookup, at the transition where
the projection groups surfaces into subgraphs — the one call site the retired
`graph_surface_layer` had. *Red condition:* neither is a corpus narrowing and
neither has a red condition of its own; a surface matching no rule takes the
default, and a default naming a layer `GRAPH_LAYERS` does not declare drops that
surface from the projection exactly as the retired hook did.

**`GATE_GRAPH_EXTERNAL_REFS`** (new resolved global).
*Producer:* `gate-sdk/lib/gate.sh`, resolving the consumer's
`GATE_SDK_GRAPH_EXTERNAL_REFS` whitespace scalar into an array. *Consumer:*
`native/src/gates/graph.rs` assertion H, at the external-ref scan over the
in-memory emission. *Red condition:* the assertion reds on **finding** a
reference matching no allowed prefix — monotone in the violation set, so a
narrowing of the emitted reference set cannot add a violation, and it is safe to
clear by inspection.

**The `--emit-graph` arm.** *Named callers, per §The non-gate arm's
owes-a-named-caller rule:* `native/src/gates/graph.rs` reaching it in-process for
assertions E, F, H and I; `gate-sdk/bin/run-gates.sh --emit graph` as the regen
command the artifact's own header prints and `docs/site-architecture.md` rosters;
and `gate-sdk/bin/upgrade-smoke.sh`'s regen step. The arm owes no descriptor, no
registration and no fixture pair.

**The retired-seam tripwire.** *Producer:* `native/src/gates/graph.rs`, at gate
start, reading its own environment for `GATE_SDK_GRAPH_THEME` and testing for a
file at `<gates-dir>/graph-theme.sh`. *Consumer:* the battery, as an exit-2
harness error. *Red condition:* it reds on **finding** either — monotone.

**Existing prose describing the prior flow.** The four `declare -F` dispatch
sites (`check-graph.sh:37,78,82,87`) are the whole of the retired mechanism and
they live in one file; no other file in the tree sources a consumer file to
dispatch one of *these* override functions. The bare `declare -F <name>` idiom
itself is not unique to this mechanism and is not retired: it is this tree's
named convention for probing an optional shell hook
(`gate-sdk/SPEC.md §The port-candidate criteria`), and two other sites use it
for unrelated hooks — `gate-sdk/bin/port-blockers.sh:375` and
`delegation-kit/lib/delegation.sh:49` (`gate_kit_roots_rel`) — re-verified at
this stage, narrowed from an unbounded "no other file uses the pattern" that a
literal-string grep for `declare -F` disproves. The prose describing the
retired mechanism is inventoried in the next section.

## Existing sections updated

Each names the delta that owns it.

- **gate-sdk/SPEC.md §check-graph** — the *Theme seam* paragraph is rewritten
  onto the part-file contract, including its determinism and self-containment
  clauses, which hold unchanged and must be re-stated against files rather than
  functions (delta 2). The *Rule content is config* paragraph drops
  `graph_surface_layer()` and states `GRAPH_LAYER_RULES` /
  `GRAPH_LAYER_DEFAULT` (delta 3). Assertion B's four coverage branches are
  stated (delta 6). The *Port sizing* paragraph is corrected: the 929-line figure
  counted the generator, which delta (7) rules does not port, so the ported
  surface is `check-graph.sh`'s 632 lines and the generator's 297 stay shell with
  the cause stated (deltas 7, 9). The runtime paragraph is rewritten against the
  measurement (delta 9). The criterion-4 verdict and the fixture-widening
  condition land here (delta 8).
- **gate-sdk/SPEC.md §lib/gate.sh** — the new guarded defaults and the resolved
  `GATE_GRAPH_EXTERNAL_REFS` join the knob roster (delta 4); the vocabulary-file
  sourcing joins the consumer-config-file paragraph (delta 3); and the
  *values cross the bridge; documents cross as a path* rule lands beside the
  newline refusal it derives from (delta 2).
- **gate-sdk/SPEC.md §gen-pre-commit** — gains the stated cause for staying
  shell, on criterion 6's one-place-a-knob-is-computed ground (delta 7).
- **gate-sdk/SPEC.md §The port-candidate criteria** — criterion 4's register of
  worked instances gains `check-graph` as a second member with no clearing
  configuration; criterion 6's globstar commitment gains the clause that it
  governs a glob matcher and not a predicate that matches no glob (deltas 6, 8).
- **gate-sdk/SPEC.md §upgrade-smoke** — the regen step's path source (delta 5).
- **gate-sdk/SPEC.md §Consumer smoke** and **installer/README.md §The gate
  binary** — only if the binary-less residual roster grows; the roster is
  measured at the build cut by `installer_smoke`'s binary-less leg from a clean
  checkout of the port's own commit, never predicted here. `check-graph` is
  `# install: zero-config`, so it *is* seeded into a freshly initialised
  consumer's registry and the growth is expected to be one; the recorded number
  is the measured one (criterion 5).
- **docs/site-architecture.md §Generated projections** — the graph artifact's
  regen command (delta 10).
- **CLAUDE.md §The provenance seam (never cross it)** — the worked example it
  names is `check-graph` / `scripts/graph-vocab.sh`, and the sentence stays true
  through the cut; it is re-read at the merge and edited only if the cut made it
  false. No edit is planned, and this entry exists so the check is not skipped.
- **scripts/graph-vocab.sh, scripts/graph-theme.sh** — this consumer's own
  migration (deltas 2, 3). `graph-theme.sh` is deleted and its three heredoc
  bodies move verbatim into `scripts/graph-theme/{theme.css,header.html,footer.html}`.
  **Acceptance:** with the *shell* gate still in place, `--emit` before and after
  the theme migration is byte-identical, which is what proves the retired seam's
  behavior was preserved rather than re-authored. The regen-command change of
  delta (10) is taken afterwards, since it changes the artifact's header comment.

## Definition of Done

- [ ] **Causal completeness** — every new state/event/interface has a named,
      reachable producer and a named consumer; every new field has a named
      reader at a named transition.
- [ ] **Merged with no information lost** — each addition integrated into its
      proper canonical-spec section (not appended); the merged spec reads as one
      coherent document a reader who never saw the amendment can use alone.
- [ ] **Amendment deleted** — this file removed on merge; none remain for the
      component (`ls gate-sdk/SPEC-*.md`), discharged at the iteration rather
      than at the commit.
- [ ] **Removals propagated** — grepped every spec for `graph_theme_css`,
      `graph_theme_header`, `graph_theme_footer`, `graph_surface_layer` and
      `GATE_SDK_GRAPH_THEME`; nothing dangles.
- [ ] **Gaps filed** — the crate's first non-git spawn and the config bridge's
      resolution cost are filed as deferred entries; cross-component gaps
      discovered during the work are resolved that session, not deferred.
