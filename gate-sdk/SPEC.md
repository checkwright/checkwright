# gate-sdk — a self-testing lint framework for prose/spec/config surfaces

Machine-gated consistency for the surfaces conventional linters ignore:
markdown specs, glossaries, task queues, config projections, diagrams — any
text whose drift is mechanically decidable. A **gate** is a small shell script
that checks one invariant across one or more governed surfaces and blocks the
commit (or the merge) when they disagree.

The kit carries the generic mechanism only; a consumer's rule content (term
lists, coupling vocabularies, glossary bodies) stays in the consumer repo.

## Layout and configuration

The kit is vendored (or submoduled) into a consumer repo, conventionally at
`gate-sdk/`. The consumer owns a **gates directory** (default `scripts/`,
override with `GATE_SDK_GATES_DIR`) holding:

- `gates.list` — the registry: one gate name per line (`#` comments and blank
  lines ignored). A listed name resolves to `<gates-dir>/<name>.sh` first, then
  `<gates-dir>/<name>.gate`, then each vendored kit's `checks/<name>.sh` and
  `checks/<name>.gate` — `.sh` beating `.gate` **within** a dir, dirs tried
  consumer-first. So any kit's shipped gates are
  registered by name alone, and a consumer can shadow one by dropping a
  same-named file in its own gates dir — including shadowing a ported gate with
  its own shell script. The `.gate` spelling is the non-executable declaration of
  a gate whose implementation is a binary subcommand (§The `# graph:` manifest);
  resolution returns it as a declaration path, while execution resolves
  separately through `gate_command` (§lib/gate.sh). The kit set defaults to gate-sdk plus
  every sibling directory holding a `checks/` **or** a `smoke/` (a vendored
  Checkwright kit — a gateless kit is discovered by its `smoke/` alone);
  override with `GATE_SDK_KIT_DIRS` (space-separated kit roots).
- the consumer's own `check-*.sh` gates (copy-edits of
  `templates/check-skeleton.sh`).
- `gate-tests/` — the consumer's fixture tree (see §run-gate-tests).
- `git-hooks/` — the generated `pre-commit` (see §gen-pre-commit) and any
  hand-written hooks.
- `gate-sdk-config.sh` — optional persistent config: a sourced shell file that
  sets any `GATE_SDK_*` layout knob so the override outlives the shell that set
  it (see the loader paragraph below).
- `graph-vocab.sh` — optional rule content for `check-graph` (see there).
- `graph-theme.sh` — optional consumer theme for `check-graph`'s emitted
  artifact (see there).
- `core-files.list` — optional manifest for `check-core-files`: the
  repo-relative paths that must stay present and tracked (see there).
- `identity.conf` — optional manifest for `check-identity`: the git identity
  (committer email, remote host) this clone must resolve to (see there).
- `root-allowlist.list` — optional manifest for `check-root-tiering`: the
  tracked top-level entries permitted at the repo root (see there).
- `msg-patterns.list` — tracked banned-pattern list for `check-commit-msg` /
  `check-tree-terms` (generic patterns; copy `templates/msg-patterns.list`);
  `msg-patterns.local.list` — its gitignored companion for private terms, which
  must never be tracked (tracking the banned terms would itself be the leak).

Environment overrides, all optional: `GATE_SDK_GATES_DIR` (default `scripts`),
`GATE_SDK_TESTS_DIR` (default `<gates-dir>/gate-tests`), `GATE_SDK_HOOKS_DIR`
(default `<gates-dir>/git-hooks`), `GATE_SDK_WORKFLOW_DIR` (default
`.workflow`; the directory's two-tier membership rule, header form, and
extension rule are §The workflow directory), `GATE_SDK_GRAPH_ARTIFACT` (default
`<gates-dir>/CHECK-GRAPH.html`; the emitted coupling-graph artifact's path,
read by `check-graph` assertion E — set it to republish the artifact elsewhere,
e.g. a served docs page), `GATE_SDK_TMP_DIR` (default `.tmp`), `GATE_SDK_VERBOSE`
(default unset = quiet green; any non-empty value restores the full per-gate
banner roll on `run-gates.sh` and the generated hooks — see §run-gates),
`GATE_SDK_QUEUE_FILE` (default `TASK-QUEUE.md`), `GATE_SDK_AGENT_FILE` (default
`CLAUDE.md`; the always-loaded agent file a consumer's harness reads — set it to
`AGENTS.md` under an agent-file harness, and `check-root-tiering`'s built-in
allowlist accepts that file at root), `GATE_SDK_GRAPH_THEME` (default
`<gates-dir>/graph-theme.sh`; the optional consumer theme file `check-graph`
sources to inline host-site tokens/chrome into the emitted artifact — see
there), `GATE_SDK_GRAPH_EXTERNAL_REFS` (default empty; space-separated URL
prefixes the `check-graph` external-ref assertion sanctions beyond the
kit-seeded mermaid import — a consumer whose theme chrome links absolute URLs
lists their prefixes here — see §check-graph), `GATE_SDK_CORE_FILES_FILE` (default
`<gates-dir>/core-files.list`), `GATE_SDK_IDENTITY_FILE` (default
`<gates-dir>/identity.conf`), `GATE_SDK_PRUNE_DIRS` (default
`target .git node_modules .tmp gate-tests`), `GATE_SDK_GRAPH_VOCAB` (default
`<gates-dir>/graph-vocab.sh`), `GATE_SDK_KIT_DIRS` (default: gate-sdk + its
siblings holding a `checks/` or a `smoke/`), `GATE_SDK_ROOT` (default: the
vendored `gate-sdk/` resolved beside the sourcing script — the root a
consumer-copied gate sources `lib/gate.sh` from and the anchor kit roots
relativize against), `GATE_SDK_ROOT_ALLOWLIST` (default
`<gates-dir>/root-allowlist.list`), `GATE_SDK_REGISTRY_DOC` (default `README.md`)
and `GATE_SDK_RUNNER_DOC` (default `README.md`) for `check-kit-registration`,
`GATE_SDK_MSG_PATTERN_FILES` (default
`<gates-dir>/msg-patterns.list`; space-separated, each tracked and required —
fail-closed when missing), `GATE_SDK_MSG_PATTERN_FILES_LOCAL` (default
`<gates-dir>/msg-patterns.local.list`; gitignored, skipped when absent so a
fresh clone without the operator's private list still commits),
`GATE_SDK_COMMIT_TYPES` (default
`feat fix refactor perf docs test build ci chore style`; the shared
commit-type roster — see §check-commit-subject), `GATE_SDK_EXEC_GLOBS`
(default `*/checks/*.sh */kpis/*.sh */bin/*.sh` plus the computed
`<gates-dir>/check-*.sh` and `<gates-dir>/kpi-*.sh`; the path globs whose
tracked `*.sh` members `check-exec-bit` holds to index mode `100755` — see
there), `GATE_SDK_EXEC_PRUNE` (default `gate-tests fixtures templates smoke`;
the path segments whose subtrees `check-exec-bit` exempts — see there), `GATE_SDK_ENFORCE_SCAN_DIR` (default `.`; the enforcement map's
monitor-marker scan root — see §enforcement-map), and
`GATE_SDK_LINT_EXTRA_DIRS` (default empty; space-separated directories whose
direct `*.sh` members join `check-shellcheck`'s derived scan set — the seam for
a shipped script that sits under no kit root — see §check-shellcheck), and
`GATE_SDK_NATIVE_BIN` (default `native/target/release/checkwright-gates`; the
multi-call binary `gate_command` dispatches a `.gate`-declared member to — see
§lib/gate.sh). Its default is a **stable relative path** deliberately: the
generated pre-commit hook persists the emitted argv, and a machine-specific
absolute path baked into a tracked hook would make `check-graph`'s byte-freshness
comparison machine-dependent. And `GATE_SDK_NATIVE_SRC` (default `native/src`;
the implementation tree §check-gate-substrate-parity assertion D holds free of
manifest-class annotation — a **path, not a language**, so the knob assumes
nothing about what implements a ported gate). Paths are
repo-root-relative; every entry point `cd`s to `git rev-parse --show-toplevel`
before resolving them.

`lib/gate.sh` auto-sources the consumer config seam on load, so every gate (all
source the library) sees the same knob resolution: `GATE_SDK_CONFIG_FILE` when
set — and a set-but-missing path exits 2 rather than silently running on
defaults (an operator typo must not pass as clean) — else
`<gates-dir>/gate-sdk-config.sh`, sourced only when the file exists — a
zero-config consumer is unaffected. Env vars still win (the config file sets a
default the invoking shell may override), but the file is how an override
*persists*: an env-only knob dies with the shell that exported it, so a
consumer that must relocate a layout knob for every session sets it here. The
one knob the file cannot set is `GATE_SDK_GATES_DIR`, which locates the file
itself — it stays env-or-default (a config file cannot name its own directory).

## The workflow directory

`GATE_SDK_WORKFLOW_DIR` (default `.workflow`) is where every kit writes its
committed projections and its local capture. gate-sdk owns the knob, so it owns
the directory's contract; the kits that write into it own their individual
files.

**Two tiers, partitioned by tracking.** Every member is either a **checked
projection** (tracked, committed, gate-read) or **local capture** (gitignored,
advisory, drained by a named reclaim path). A member that is neither tracked nor
ignored is the drift state `check-workflow-tiering` refuses: an uncommitted file
no reviewer sees and no `.gitignore` line accounts for.

**The header requirement follows tracking, and it must.** Local capture's
reclaim path is whole-file truncation (`: > <file>`, the shape every capture
log's close-stage step uses), which erases a header on every drain. A header
requirement on that tier would fight the tier's own reclaim mechanism — either
the header is re-seeded on every clear, adding a writer to a surface whose whole
point is that any appender may write it, or it decays to a rule violated by
correct operation. So: **checked projections carry the header; local capture
carries none.** That asymmetry is the substantive content of the
tracked-vs-gitignored axis, not a convenience.

**It follows that a tracked member which drains, drains header-preservingly.**
The tier admits accumulation buffers as well as one-shot projections — a
surface any session appends to and one session empties at a boundary — and for
those the reclaim path cannot be local capture's `: > <file>`, which erases the
header and reds this gate on the very commit that drains. Truncate to the
header instead. Every draining member of this tier already does:
`WORKFLOW-STATE.txt` at the iteration boundary, `gap-inbox.md` at close,
`tightened-gates.txt` at the tag.

**The header form.** A checked projection's first line is `# contract: `
followed by one of two ruled payloads:

- **Pointer form** — `<owner-path> §<section>`, optionally followed by an
  em-dash tail (` — ` then a line grammar or a gloss). The path is a tracked
  file and the section resolves;
  `check-spec-pointer` already owns that resolution, and the em-dash tail is
  already stripped before heading matching, so a grammar line may ride the same
  header as its pointer. This is the default form.
- **Version-marker form** — `<format-name> v<N>` (`^[a-z0-9-]+ v[0-9]+$`), used
  only where a gate parses the header itself as a wire-format version. The
  owning SPEC states that it does — evidence-kit/SPEC.md §Evidence manifest is
  the one such statement in this tree.

Requiring *some* header would gate nothing — the tracked side already satisfies
it — so the requirement is the **prefix with a ruled payload**. Both payload
forms are machine-recognizable, which is what keeps the rule a rule rather than
a description.

**The coupling-graph artifact is not a member of this tier.** A rendered HTML
document opens with a doctype, so it can carry neither payload form and stay a
valid document; and it is tracked and freshness-gated, so local capture is not
open to it either. It fits no row of a partition this section calls total, which
makes its placement here a defect in the emitting default rather than a case the
partition widens for — `check-workflow-tiering` reddens on it, correctly, in any
consumer that registers the gate under kit defaults. Its home is the **gates
dir**, `<gates-dir>/CHECK-GRAPH.html`, on three grounds: `gen-pre-commit.sh`'s
hook is already a generated projection gate-sdk owns and already lives there, so
the two artifacts sit under one owner instead of straddling two directories; the
gates dir is a root entry every consumer allowlists anyway, so no adopter pays a
new root surface (§check-root-tiering); and it presumes no `docs/`, which a
published-site path would. A consumer wanting it on a site repoints
`GATE_SDK_GRAPH_ARTIFACT` (§check-graph), which is what that knob is for.

**The extension rule keys on writer and reader, not on tier.** These are two
independent axes, and conflating them is why the convention resisted statement:
the directory holds tracked `.md` beside gitignored `.md`, so no extension
tracks tracking.

- `.txt` — a **record file with a stated line grammar** that a gate or `bin/`
  affordance parses field-wise.
- `.md` — a **prose surface a human reads and dispositions**, machine-read only
  for emptiness or for a bullet count.
- `.log` — an **append-only capture stream** written by tooling at the moment of
  an event, triaged in bulk and cleared wholesale; no per-line grammar contract,
  which is exactly why nothing parses it field-wise.

A new file's extension is therefore determined, not chosen: ask which of the
three describes its writer and its reader. The extension rule is deliberately
**not** gated — deciding whether a file is a record, a prose surface, or a
capture stream is the judgment the rule exists to guide, and mechanizing it
would mean inferring a writer's intent from a file's bytes. Under the
enforcement-first carve-out it takes the other disposition available to an
un-gateable class: a cadenced review entry on the consumer's audit roster, due
at each workflow-directory addition.

The tracked tier is also the surface canon-kit's comment gates read — the
governed comment/pointer surface admits a workflow member iff it is tracked,
whatever its extension, and blesses only `contract:`/`see` there
(canon-kit/SPEC.md §check-comment-tier).

## The gate model

A gate family imposes test-grade rigor on prose and config surfaces, but a gate
whose own correctness is unverified silently stops enforcing — production use
attested two *self-broken* gates (a false-green on a crashed `awk`; two scanner
crashes) before the four contracts below were adopted.
The family tests, lints, and constrains itself by the same standard it holds
the governed tree to.

### Output contract

A gate signals success with exactly one line `^<NAME>: clean (<parenthetical>)$`,
where `<NAME>` is the gate's stable upper-token id and the parenthetical states
what was checked. One canonical success token across the whole family — a gate
that "passes silently" is indistinguishable from one that did nothing. Failure
is one line per finding (location + what is wrong) then a `help:` remedy line
naming the concrete action — the model is the Rust compiler's `help:`, not a
restatement of the violation. A gate with multiple distinct failure classes
gives each its own `help:` line. Exit codes: **0** clean, **1** violation,
**2** harness/usage error.

Enforced **at runtime, on real output**, by the fixture runner: a `good/` case
must emit the canonical clean line and a `bad/` case a `help:` line
(§run-gate-tests owns the mechanism). That is the oracle for every fixtured
member, on either substrate — a source grep was only ever a proxy for the
behavior, and it reads nothing at all once a gate's rule is a compiled
subcommand.

`check-gate-output` keeps the **static** half for the one class the runtime
assertion cannot reach: a member carrying `# no-fixture:`, which has no
`good/`/`bad/` case for the runner to execute (§check-gate-fixture-coverage).
For those the source grep is the only oracle, so it stays. Retiring it outright
in favour of the runtime check would silently zero that member's
output-contract coverage — total coverage across the registry is the point, and
94-of-95 dressed as complete is the failure this split avoids.

### Fail-closed contract

A gate that captures a subprocess's stdout and branches on the captured value's
**emptiness** false-greens when that subprocess crashes: the output is empty,
so the gate prints `clean` — but the check never ran. The fix is to key on the
subprocess **exit status**. The wrapper is centralized in `lib/gate.sh`; source
it and call `fail_closed` right after a capture:

```bash
out="$(awk '…' "$FILE")"; st=$?
fail_closed "$st" check-foo awk     # non-zero status -> exit 2, never a false clean
```

Wrap an `awk`/`jq` capture that reads a **file** and feeds the verdict. Leave
alone — and a sweep must NOT blanket-wrap — `grep`'s exit-1-on-no-match
(expected, not an error), an already-status-checked capture, and in-memory
transforms (`<<<` here-strings, trivial field selectors that cannot fail on
present input). Enforced by `check-gate-fail-closed`; a genuinely-safe capture
opts out with `# fail-closed-exempt: <reason>` on the comment block immediately
above it. The helper itself is tested directly by
`gate-tests/lib-gate.test.sh` (a per-gate input fixture cannot prove it — a
well-formed `awk` cannot be crashed on present input).

### Fixture-pair discipline

When a gate is written or edited, it ships with — or updates — its
`good/`+`bad/` fixture pair under `<tests-dir>/<gate>/`, run by
`run-gate-tests.sh`. A gate that prints `clean` on broken input is invisible to
every static check; the only proof it fails closed and that its error text is
right is a known-bad input (the `bad/` case asserting exit 1 + its `expect.txt`
line(s) — every one of them, §run-gate-tests owning the semantics), paired with
a `good/` case asserting acceptance. Coverage is
enforced by `check-gate-fixture-coverage`: every registry member carries either
a pair or a `# no-fixture: <reason>` header annotation — the honest, reviewable
escape for whole-tree scanners whose state has no static-fixture representation
(e.g. a HEAD-vs-worktree diff). A fixture-*capable* gate carrying the valve as
a stopgap is filed as debt and fixtured, never given a dishonest "infeasible"
reason.

### Self-lint

Every script in the family — the consumer's gates and the kit's own `lib/`,
`bin/`, `checks/`, `templates/` — passes ShellCheck at `-S warning`, enforced
by `check-shellcheck`. Template stubs are lint-governed (they are runnable
shell a consumer copies out) — and `check-comment-tier` governs their comments
too: a copied-out template's `spec:` line resolves against the vendored kit
path, so its comments are directives like any source, thinned to the
`# graph:`/`# spec:` lines and the placeholder scaffolding the consumer fills
in (canon-kit/SPEC.md §check-comment-tier). Only `check-spec-pointer` skips a
template, its `spec:` line being a placeholder unresolvable by design
(§check-spec-pointer) — so the template↔copy relationship itself is governed
separately, by §check-template-copy-parity, whose assertion A is the only
mechanism that resolves a template's target at all. A template's post-copy
`source` paths are legitimate. A
false positive is silenced inline with `# shellcheck disable=SCxxxx` plus a
justifying comment, never a blanket `.shellcheckrc`.

### Shared cross-gate values

A value or small lookup needed by ≥2 gates lives in `lib/gate.sh` as the single
source — reference it, never re-literal it per gate. This is deliberately *not*
a base library: it shares **values and walk adapters**, never gate structure
(structure is copied from the skeleton, so it stays per-gate and legible), and
its failure mode is loud — a missing or malformed `lib/gate.sh` aborts every
sourcing gate at the `source` line, the fail-closed direction.

### Calibration lessons (paid for, now design rules)

- **Wrap-aware matching.** When a gate's signal token can hard-wrap on a
  ~80-col prose surface, match over the joined logical unit, not the physical
  line, and prove it with a wrapped-case `bad/` fixture.
- **Scope a sentinel allowance to its phase.** A gate that admits a bootstrap
  value (`—`, `TODO`, `0`) with a blanket allowance leaks it past the one phase
  it was meant for; bind the allowance to the phase.
- **Exclude the fixture tree from whole-tree discovery.** A gate that discovers
  inputs by a shape its own fixtures also carry descends into the fixture tree
  and false-reds — and a *different* gate's fixtures can break yours. Use
  `lib/gate.sh`'s prune adapters: a `find` walk → `gate_find <root> <expr…>`
  (parenthesize a multi-term expression — `gate_find` appends `-print`, and an
  unparenthesized `-o` chain binds it to the last term only); a `grep -r` walk
  → splice `"${GATE_GREP_EXCLUDES[@]}"`; a paths-then-filter walk →
  `gate_path_pruned "$f"`.
- **Couple per surface, not per file.** When one artifact holds N
  representations of the same model, each needs its own coupling edge or an
  explicit ungated marker; "this file is heavily gated" silently reads as "all
  of it is gated."
- **Fixtures probe the boundary, not the live tree.** A gate passing on the
  real corpus is not evidence it is correct — write the pair to drive the
  inputs production doesn't (last element, empty set). Last-line idiom:
  `while IFS= read -r e || [[ -n "$e" ]]`.

### When a gate earns its place

A gate is a standing cost (maintenance, false-positive friction, reading load),
so the family is sized deliberately. When a consistency property on a driftable
governed surface has a **cheap, low-false-positive, mechanically-decidable**
check, build the gate **proactively — before any attested miss**; discipline is
not an acceptable substitute for a check this class can make. A gate that is
expensive or higher-FP waits for a real miss to attest it — and the miss *is*
the `bad/` fixture. Cheap + low-FP is necessary, not sufficient: the gate must
check a real drift axis, never a trivially-true proxy (heading *presence* while
the content drifts) that manufactures false confidence. The dual rule minimizes
the standing set *subject to the guarantee being preserved*: a gate is
removable only when its guarantee is delivered another way or was never
attested. What stays human is the irreducibly semantic judgment alone — *is
this prose still true?*

## Enforcement tiers

Three concentric tiers, each an outer backstop for the one inside it:

- **pre-commit** (opt-in, per-clone, fast) — the generated triggered subset,
  catching drift in the single commit that perturbs it. Bypassable by design
  (`git commit --no-verify` is a valve, not a hole, once an outer tier exists).
- **pre-push / full battery** (opt-in, per-clone, fuller) — `run-gates.sh`
  whole-tree before the work leaves the machine.
- **CI** (server-side, authoritative) — `run-gates.sh` + `run-gate-tests.sh`
  on every push, with branch protection making a merge conditional on them.
  The copy-out is `templates/gates-workflow.yml` (see there); only this tier is
  a guarantee, and it stops bypass but not workflow self-edit — the tamper-proof
  verifier is the deferred hosted-attestation rung. The inner tiers are latency
  optimizations.
  `run-gate-tests.sh` runs as its own step, not folded into the battery —
  `check-gate-fixture-coverage` asserts fixtures *exist* but never *executes*
  them; the execution is the gate-authority backstop, kept separate so a
  fixture-logic failure is attributed to the gate.

## The `# graph:` manifest

Every registered gate's **declaration path** carries a one-line coupling
manifest in its header. The declaration path has two spellings: the gate's own
`<name>.sh`, or — where the implementation is a compiled subcommand — the
`<name>.gate` descriptor beside it (§lib/gate.sh resolves between them). The
manifest grammar below is identical in both; every reader greps it out of the
declaration path as text and none requires that path to be shell.

**The `.gate` descriptor.** A gate whose implementation is a binary subcommand
keeps a **non-executable** declaration file `<dir>/<name>.gate` carrying only
the lines its readers need: the `# graph:` manifest (required), its `# spec:`
pointer, and an optional `# no-fixture:` opt-out. Nothing else. It is data — it
is never sourced and never run.

**Its existence is the dispatch declaration, and the subcommand name is the gate
name.** There is no dispatch field, no second registry and no mapping table,
because each would be a name that could drift from the thing it names. The one
fact is the file's presence; the subcommand is derived from the name that
already identifies the gate everywhere else. Every field it carries has a named
reader — `# graph:` by the manifest readers below, `# spec:` by
canon-kit's `check-spec-pointer`, `# no-fixture:` by
§check-gate-fixture-coverage — and the descriptor carries no field that lacks
one, reserving nothing against a future reader.

**The descriptor is a durable surface, not port scaffolding.** Its reason does
not expire with the port that introduced it: `installer/lib/init.sh` runs
`gen-pre-commit.sh --write` in the *consumer* tree, so the manifest must stay
readable **with no build and no execution**, and that constraint only
strengthens as more gates port.

**Annotations partition by reader, and the partition is a rule.**

- **Manifest-class** — the `# graph:` manifest, and anything else whose reader
  must work without a build — lives in the descriptor **only**. It is never
  emitted into, nor hand-written in, the implementation. Held by
  §check-gate-substrate-parity assertion D.
- **Locality-class** — `# spec:` with its one-line binding, and the
  comment-tier directives — binds to a *line of implementation*, so it **stays
  in the implementation**: moving it to a companion file destroys the binding
  that gives it meaning. The corpus primitive
  `canon-kit/lib/spec.sh` reaches the implementation for exactly this reason
  (canon-kit/SPEC.md §lib/spec.sh), so those assertions stay live on a ported
  gate instead of going dark.

**The layer above the descriptor is substrate-blind.** Because the manifest sits
outside the implementation, the graph, hook, and meta-gate layers read a gate's
declaration without knowing what implements it — a gate could be implemented in
any language behind a descriptor, and nothing in this format or in the
resolution path assumes otherwise. Slice 1 neither builds nor claims that
generality; it only declines to foreclose it where a neutral choice cost the
same. The same property is what keeps the deferred consumer-payload question
tractable: a descriptor discloses a gate's *shape* without its *predicate*.

Two shapes were refused, each on a constraint rather than on taste. **The binary
emitting its own manifest** would make hook generation depend on executing the
binary, but `installer/lib/init.sh` runs `gen-pre-commit.sh --write` in the
*consumer* tree; keeping the manifest as tracked text is what makes the seam
payload-neutral by construction. **A shell stub carrying the manifest and
exec'ing the binary** would stay inside the `*.sh` corpora of `check-shellcheck`,
`check-comment-tier`, `check-gate-output` and the rest, which would then scan a
three-line file and report `clean` — coverage that reads as real and asserts
nothing. A `.gate` descriptor is *honestly outside* those corpora instead of
trivially passing inside them, and §Meta-gate conservation for the binary
substrate is what makes that honesty auditable.

The manifest grammar:

```
# graph: couples=<globs> dir=bi|one valve=none|PROPOSED tier=precommit|align-only|commit-msg [mode=staged|whole-tree] [trigger=<globs>] [gen=manual]
```

- `couples=` — the surfaces the gate binds (comma-separated globs). One token
  is special: `kit:<glob>`, which the shared manifest reader in `lib/gate.sh`
  (`gate_expand_couples_var`) expands to `<kit-root>/<glob>` —
  repo-root-relative — for every `gate_kit_roots` member at read time; the same
  reader feeds
  `gen-pre-commit` (hook emission), `check-graph` (freshness + the HTML
  projection), and `run-gates --for` (path-scoped selection §run-gates), so
  emitter, checker, and selector cannot desync on the kit set. A
  whole-tree gate writes `kit:*.sh` in place of a per-kit hand list; non-kit
  couples (`scripts/*.sh`, `.workflow/*.txt`, `scripts/gates.list`) stay
  literal. Expansion over-approximates by design — a kit is coupled even where
  the gate's subject is narrower, so an extra trigger runs a green gate while a
  missing one would skip a red — and `check-kit-enum` gates the residual
  hand-lists derivation cannot reach. The corollary is a hard authoring rule:
  `couples=` must *cover* every path the gate reads at runtime, never a subset.
  Globs never cross `/` (and `kit:<glob>` expands one level only), so a gate
  that walks a directory recursively (`find` / `gate_find`) must couple that
  recursion — a `<dir>/<sub>/*.ext` sibling glob or a wider one — and never
  lean on the tree holding only the shape the couple enumerates; an
  under-covering couple silently skips the gate on the very edit it should
  catch. `check-graph` verifies couples→hook parity, not reads⊆couples;
  `check-reads-couples` (§check-reads-couples) mechanizes the reads⊆couples half
  for the statically resolvable walks — so the author's duty narrows to the
  undecidable remainder that gate skips-and-counts (check-shim-restatement's
  stage-template subdirectory was exactly this bug).
- `dir=` — `bi` for a coupling bijection (both sides must agree), `one` for a
  one-way audit.
- `valve=` — `PROPOSED` marks a cycle valve: a coupling where a leading
  (design) surface may run ahead of a lagging (code) surface via a
  queue-tracked marker; `none` means the sides must agree now.
- `tier=` — `precommit` gates emit a trigger block in the generated
  `pre-commit` hook; `align-only` gates run only in the full battery;
  `commit-msg` gates emit an unconditional invocation into the generated
  `commit-msg` hook (the message-file surface, not a tracked path — see
  §gen-pre-commit). Default to `precommit`; the discriminator between the first
  two is not cost but whether the invariant is **restorable within the single
  commit that perturbs it** — a settled-corpus audit would false-red on
  work-in-progress and belongs to the full battery. A `commit-msg` gate's
  `couples=` names its config files (the regeneration triggers), never a tree
  path, and it no-ops cleanly under a no-argument full-battery run — the
  prospective message exists only at commit time.
- `mode=staged` — the hook passes the staged subset of the trigger globs as
  positional args; default (`whole-tree`) emits a bare invocation.
- `trigger=` — hook guard globs when they diverge from `couples=`; `trigger=*`
  emits an unconditional invocation.
- `gen=manual` — the gate's hook block is bespoke and round-trips verbatim
  between `# >>> manual: <gate>` / `# <<< manual: <gate>` sentinels.

## Meta-gate conservation for the binary substrate

A gate's shell file carries five different things at once: the rule, the
`# graph:` manifest, the output-contract strings, its `# spec:`/`# assertion`
directives, and the greppable evidence that its reads stay inside its declared
couples. Porting the rule to a compiled subcommand removes the file the other
four are read from — so a port that records nothing **silently ends** the
assertions those readers make, and most of them end by finding nothing and
printing `clean`. A false green, not a red. This section is the contract that
stops that, and `check-gate-substrate-parity` assertion C is what makes it
machine-held rather than remembered.

**The substrate-sensitive set is derived, never maintained.** A registry member
is substrate-sensitive when its expanded `couples=` covers the **declaration
path of a registry member** — the derivation `check-gate-substrate-parity`
performs at runtime, so no count or roster here can rot. (The test is against
registry members' declaration paths specifically, not against every `*.sh`
under a resolve dir: `scripts/queue-config.sh` sits in the gates dir and is not
a gate, and matching it would over-report.) Every derived member takes exactly
one recorded disposition below, and a member the section does not name is red.

| Meta-gate | Disposition for a `.gate`-dispatched member |
|---|---|
| `check-shellcheck` | **Retired with cause** — no shell exists to lint. `cargo clippy` at deny-warnings is the substrate equivalent and runs in CI, not as a gate. |
| `check-gate-output` | **Ported and strengthened for the fixtured corpus; source-grep retained for the one member outside it.** The source-grep for `: clean`/`help:` was always a proxy for behavior; for the fixtured members the assertion now runs in `run-gate-tests.sh` (§run-gate-tests) against the case's real output, on **shell gates too**. The remaining member, `check-task-conservation` (`# no-fixture:` per queue-kit/SPEC.md §check-task-conservation — a HEAD-vs-worktree diff has no static-fixture representation), has no case for a runtime assertion to reach, so the source-grep stays its only oracle. Retiring the static half outright would zero out that member's output-contract coverage — the exact vacuity this table exists to close. |
| `check-gate-fail-closed` | **Retired with cause** — the defect (branching on a captured value's emptiness when the subprocess died) is unrepresentable once a fallible call returns a `Result` that cannot be ignored. A real substrate win, stated as one. |
| `check-reads-couples` | **Retained, and must fail closed.** Its shell parser finds no walks in a binary gate and would print `clean` — the single worst vacuity available here. Until a binary-side equivalent exists it **refuses** (exit 2) on a member resolving to a `.gate`, rather than passing. |
| `check-gate-assertions` | **Retained, corpus extended** to the gate's Rust module; the `# assertion` marker matches on its token, independent of the comment leader. |
| `check-gate-exemption-tasks` | **Retained, corpus extended** the same way. |
| `check-comment-tier` | **Retained, corpus extended** to the implementation module and the `.gate` descriptor, whose own lines are directives by construction. Mechanism: `canon-kit/lib/spec.sh`'s `spec_comment_surface_with_templates` gains `*.gate` **and `*.rs`** arms — the shared primitive, widened once (see the `check-spec-pointer` row). The implementation arm is the load-bearing one: locality-class directives stay in the implementation by the reader partition (§The `# graph:` manifest), so without it they would go dark exactly where they still apply. |
| `check-spec-pointer` | **Retained, and its corpus depends on the same widening** — not "unchanged" in mechanism, only in assertion logic. It calls the *same* shared `spec_comment_surface` in `canon-kit/lib/spec.sh`; absent that one shared fix a ported gate's `# spec:` line would silently stop being checked in both places it can live — the descriptor and the implementation. Once the primitive gains the `.gate` and `*.rs` arms its own probe logic needs no change. |
| `check-readme-roster` | **Retained, glob widened** to `*.sh` + `*.gate`. Without it a ported gate silently drops out of its kit README's roster in both directions. |
| `check-exec-bit` | **Retained, extended**: a `.gate` descriptor must be **non**-executable. Stated as an assertion so "not executable" cannot read as "not covered". |
| `check-todo-task-liveness`, `check-deprecation-task` | **Retained, corpus extended** to the Rust module and the descriptor, the same shape as `check-comment-tier`: both walk `spec_comment_surface` hunting `TODO(task:)`/deprecation markers, so a marker left in a ported gate's Rust source would otherwise stop being tracked. |
| `check-knob-default-coupling` | **Retained unchanged, and deliberately *not* corpus-extended** — the extension the shape of this table invites would be vacuous. Its two default idioms are shell (`${KNOB:-v}`, the guarded assignment) and its knob prefixes derive from `gate_kit_roots` members; `native/` is not a kit root and a Rust `const` matches neither idiom, so pointing it at `*.rs` would scan files whose grammar it cannot parse and add zero assertions while reading as coverage. The duplication it therefore cannot reach — the crate's prune-dir default against `lib/gate.sh`'s — is closed by an **executed** assertion instead: a unit test in the crate reads the shell library and compares the two literals, failing on drift. Recorded this way because a disposition that names a mechanism which cannot fire is the same defect as no disposition. |
| `check-gate-tamper` | **Retained, extended**: its `is_gate_file()` glob roster gains the `.gate` spelling, or a ported gate's edit escapes the isolation rule. Two known limits, recorded so a later port is not the session that discovers them: its `extract_exemptions()` parser reads a shell `# exception-list:` array literal and has no Rust-source equivalent, and its **meta-layer path roster does not contain `native/`**, so a commit editing a gate's Rust implementation alongside its descriptor is refused. Neither binds slice 1 — the one ported gate carries no exemption list, and its Rust module lands in a commit separate from its descriptor. |
| `check-graph`, `check-kit-enum`, `check-gate-fixture-coverage`, `check-enforcement-fresh`, `check-value-rollup-fresh` | **Survive unchanged** — all five read the declaration path as text (directly, or through `enforcement-map.sh`/`footprint.sh`, which do), which the descriptor still is. |
| `check-gate-substrate-parity` | **Retained by construction** — it is substrate-sensitive by the same derivation it performs, and it reads declaration paths both as text and as a *set*, which is precisely what it exists to see. It stays a shell gate (§check-gate-substrate-parity), so the auditor never depends on the substrate it audits. Its own row is written out rather than left to the section's prose mention: assertion C is satisfied by any occurrence of a member's name in this section, and a gate passing its own assertion by being *discussed* is a coincidence, not a disposition. |
| `check-docs-cmd`, `check-install-claim`, `check-prose-enum`, `check-queue-slug-liveness` | **Survive unchanged — reverse triggers.** Each names `scripts/*.sh`/`kit:*.sh` in `couples=` only so that a script change re-runs it; the corpus each actually scans is the governed-doc set, and none reads a gate script's *content* as its assertion target. `check-docs-cmd` is worth naming: it will correctly — not vacuously — red on a doc still fencing a deleted `.sh` path after a port. That is real signal. |
| `check-spec-embedded-source` | **Survives unchanged — reverse trigger of the same shape.** Its `couples=` extension list (`*.rs`, `*.sh`, `*.toml`, …) is the roster of **languages it recognizes inside fenced blocks**, not a reference to gate declarations; its scanned corpus is the canonical specs and amendments. It already carries `*.rs`, so a ported gate's Rust module is inside its trigger set with no widening. |
| `check-template-copy-parity`, `check-template-registry-parity` | **Survive unchanged** — their corpus is kit templates and the template registry, not gate declarations; a gate's substrate does not reach either. |

Gates whose corpus is kit directories, smoke scripts or hooks
(`check-kit-registration`, `check-smoke-entry-guard`, `check-hook-exec-bit`,
`check-test-hermetic`, `check-assertion-strength`) are not substrate-sensitive
by the derivation above and are untouched.

## What the dispatch seam does not settle

Recorded because a deferral nobody wrote down is indistinguishable from a
question nobody asked — and this entry has already lost worked arguments to
compression once.

**Dogfooding is settled in one direction and open in the other.** A registered
member dispatching to a compiled subcommand at `tier=precommit` puts
`gate_command` on the pre-commit path, and `gate_command` is fail-closed on an
absent binary. So this repo **does** build and run the binary from source at
commit time, and `cargo` is a hard commit-time requirement — the toolchain floor
records it (context-kit/SPEC.md §bin/env-probe). Stating that plainly matters,
because the earlier framing that the seam "defers whether this repo runs built
artifacts at all" is not what the seam does: porting one gate decides it. What
remains genuinely open is the *other* end — whether this repo, or a consumer,
ever runs a **prebuilt or released** artifact rather than one built from source.
No compiled artifact is committed, and nothing here assumes one ever will be.

**The consumer payload is untouched and deliberately reachable either way.**
Keeping the manifest as tracked text is what earns that: hook generation runs
consumer-side, so the seam works identically whichever way the payload question
rules. Vendoring and the extensibility model are likewise unchanged — this slice
ships no artifact and changes nothing about how a kit installs.

**Opacity is not claimed.** This repo builds from source and the implementation
sits readable in-tree, so the benefit delivered here is the seam and the
conservation contract, not the headline ground the port is sometimes argued on.
Claiming otherwise would be the "land it then relax" failure inverted.

**The language-agnostic reading is visible and not built.** The layer above the
descriptor is substrate-blind (§The `# graph:` manifest), which makes a
gate-authoring SDK — the descriptor as the neutral surface of a gate, its
predicate in any language — a coherent next thing rather than a wish. Slice 1
declines to build or widen to it, and declines to foreclose it; the initiative
is carried as its own queue entry so the framing outlives the session that saw
it.

**One thing this slice did not defer.** The manifest-class/implementation
SSOT split is *enforced*, not trusted to habit
(§check-gate-substrate-parity assertion D) — noted here because the natural
outcome for a rule stated only in prose is that the first tired session breaks
it.

## Consumer smoke

The fixture suites prove each gate in isolation on contrived case dirs, and a
consumer repo's battery runs under that consumer's own config overrides. Two
things go untested there: that a *fresh* consumer reaches green by following
the kit READMEs, and that the **kit defaults** hold on a vendored-kit tree
under zero config. The DoD-mode defect (`canon-kit-vendored-spec-dod-scope`)
shipped through exactly that gap. `bin/run-consumer-smoke.sh` closes it,
mechanizing what was a hand-repeated validate-stage prose ritual with no
committed evidence.

The harness (`run-consumer-smoke.sh [--keep] [kit-root...]`, a `bin/` tool,
never a registered gate — it builds a repo and runs the battery repeatedly, so
it is pre-commit-unfit by runtime budget): builds a scratch consumer in a fresh
temp dir (`git init`, seed commit), vendors each kit root by copy (default:
`gate_kit_roots`), and runs each kit's `smoke/install.sh` — gate-sdk first,
then argument order. It then commits the installed baseline and asserts the
full battery is green under **zero consumer config** (the positive green token
`All N gates passed` — the defaults-on-a-vendored-tree assertion no fixture
suite makes). It then runs the registration accounting (below) over the union of
the vendored kits' gates. Per kit shipping `smoke/violation.sh` it fires one crafted
violation, re-runs the battery, asserts a non-zero exit **and** a `FAIL:`
line naming the expected gate, then restores the tree (`git reset --hard &&
git clean -fd` — a hard reset, not `git checkout`, so a violation that staged
its shape is unstaged too: an index-reading gate like `check-gate-tamper` sees
only the index) before the next kit; it asserts green once more after the last
restore. Exit codes follow the gate convention (0 all hold, 1 an assertion
failed, 2 usage/environment); the success token is `CONSUMER-SMOKE: clean
(<n> kits installed, <m> violations fired, <r> gates registered, <s>
self-declared, <h> hand-declared)`. `--keep` retains the temp dir and
prints its path (the temp-dir write's named reclaim path).

The scratch-consumer build itself — temp dir, seed commit, vendor-by-copy, the
`smoke/install.sh` loop, the installed-baseline commit — is factored into
`lib/consumer-smoke.sh` (`csmoke_vendor_and_install`, which sets `SCRATCH` and
`CSMOKE_INSTALLED`), so a second harness that needs the same green baseline
before it diverges shares the mechanics rather than copying them. The caller
owns its cleanup trap and every assertion after the baseline commit.
context-kit's `smoke/agents-md.sh` is that second caller: it builds the same
baseline, then converts the consumer to a nondefault agent file (`AGENTS.md`)
and asserts the agent-file knobs carry it — an assertion `run-consumer-smoke.sh`
cannot make, since it fixes the kit defaults under zero config
(context-kit/SPEC.md §Testing). `bin/upgrade-smoke.sh` is the third caller: it
builds the same FROM baseline, then diverges into the two-phase upgrade proof
(§upgrade-smoke).

**The `smoke/` per-kit contract.** Every vendored kit ships a `smoke/`
directory — shipping it joins fixtures + README + SPEC in the kit-landing
checklist; a kit root lacking `smoke/` is an environment error (exit 2). Every
`smoke/` script that mutates the invoking tree — `install.sh` and
`violation.sh` both do — opens with the entry-point guard
`: "${SMOKE_KIT_ROOT:?run via run-consumer-smoke.sh}"` before its first mutating
command, so a bare invocation (outside the harness that exports
`SMOKE_KIT_ROOT`) refuses instead of writing into the caller's repo;
`check-smoke-entry-guard` (§check-smoke-entry-guard) holds the guard's presence
across the roster. The
README item of that checklist carries the register-the-gates block in
`<!-- gate-roster:begin -->` / `<!-- gate-roster:end -->` markers, held in
name-set parity with the kit's shipped `checks/` by `check-readme-roster`
(§check-readme-roster) — a kit that ships checks registers them. A new gate MAY
also gain a `pass` row in the held-constant validate baseline —
evidence-kit/SPEC.md §Baseline manifest tolerates its absence (a classification
cost only, zero enforcement loss). A new kit's
bespoke `gate-tests/*.test.sh` sources `lib/test-hermetic.sh`
(§lib/test-hermetic.sh) — one clause of the same checklist, `check-test-hermetic`
enforcing it.

- `smoke/install.sh` (required) — run with cwd = scratch-consumer root and env
  `SMOKE_KIT_ROOT` = the vendored copy of the installing kit. The executable
  form of that kit's README install steps: register its gates in
  `scripts/gates.list`, establish the minimal governed surface its gates need
  to be green, and regenerate the hook + graph artifacts. *Which* gates it
  registers is not the author's discretion — *The registration accounting*
  below rules on every omission. It may assume gate-sdk
  is already installed (it runs first), nothing else. A non-zero exit aborts the
  harness with exit 2 (a broken installer is an environment failure, not a gate
  finding).
- `smoke/violation.sh` (optional) — same cwd/env contract and entry-point guard
  (above); mutates the scratch
  tree to introduce exactly one violation the harness restore (`git reset --hard`
  + `git clean`) reverses (edit a tracked file, add an untracked one, or stage a
  shape), and prints the expected gate name as its first stdout line (the
  harness's red-phase assertion reads it). A kit without one contributes install
  coverage only; the harness prints a notice per such kit so the gap is visible
  in the evidence. The file
  is rightly absent only where no battery-reddening violation is craftable —
  a kit that registers no gates has nothing to redden.

**The registration accounting.** A gate earns a scratch-battery slot when it
**reads a surface the install writes** — the rule that keeps the smoke proving a
vendored kit installs and runs, instead of drifting toward a second copy of the
host battery. A gate whose subject no install writes would either pass vacuously
or fail-closed on an absent projection, and neither outcome is coverage. The rule
binds every kit, and it is **evaluated, not transcribed**: on every run the
harness accounts for each shipped-but-unregistered gate, so an omission is either
derived-justified or declared, never silent. An unregistered gate is how a live
contract violation survives — the harness cannot redden on a gate nothing runs.

The accounting is one pass in `run-consumer-smoke.sh`, between the green-battery
assertion and the violation phase, over the **union**: every vendored kit's
`checks/` basenames against the scratch consumer's `scripts/gates.list`. The tree
is already built and green at that point, so the pass adds no install and no
second tree. Each unregistered gate is run in the scratch consumer and its exit
code read against §Output contract's three meanings — the authority here, and the
only one (`check-gate-fail-closed` is a static lint over `awk`/`jq` captures and
asserts nothing about a gate's behaviour on an absent surface).

**One reading is not enough, and the second probe is not redundant.** Exit 2 is
usage/environment failure *generally* — a missing binary, a malformed config, a
non-repo cwd, an empty roster, or an absent subject surface — and only the last
is a justified omission. Because the exit-2 verdict grants a **permanent**
exemption owing no written reason, ever, it is the one verdict nobody will
re-examine; granting it off that ambiguous signal would turn the never-reviewed
row into a false-exemption channel, silently and permanently justifying a gate
that is merely broken here. So it is **corroborated**: the same gate is re-run in
the invoking repo, where the kit's own surfaces exist. An absent-surface gate
exits 2 only in the scratch consumer; an environmentally-broken one exits 2 in
both. Removing the second probe as belt-and-braces re-opens the channel.

| probe (scratch consumer) | probe (invoking repo) | verdict |
|---|---|---|
| exit 2 | exit 0 or 1 | **Justified omission**, self-declaring: the surface it reads is genuinely absent. No written reason owed, now or ever. |
| exit 2 | exit 2 | **Not exempt** — broken or environment-dependent, not surface-absent. Declare or red. |
| exit 0 | — | Green here and registering costs nothing: an unexplained omission. Red unless declared. |
| exit 1 | — | Green nowhere: the gate finds real violations and nothing runs it. **The hiding shape.** Red unless declared. |

Only the exit-2 rows reach the corroborating probe.

**Probe first, reasons second.** The exemption is *derived* — recomputed from the
tree every run, so it cannot go stale, be forgotten, or be copied wrong. That is
its worth, and its worth is not coverage: on a real sweep the derivation justifies
only a minority of the unregistered gates and the rest owe a human disposition.
What that minority buys is the row that matters most — the permanent exemption
owing no written reason ever, the one nobody re-examines — taken out of human hands
rather than left to a line written once. The written reason is the valve for the
rest, never the mechanism, and an implementation that collects reasons for every
unregistered gate and consults the probe afterwards has inverted this contract
and is wrong however green it runs. The probe set is self-limiting: a gate that
probes green gets registered, which moves it out of the probe set and into the
battery, where it was going to run anyway — most of that remainder dispositions
exactly that way, which is why the hand-declared count stays small even though the
derivation's share of the sweep is a minority. The live figures are deliberately
not restated here: the accounting line prints registered / self-declared /
hand-declared / unaccounted on every run, and it is the only authority for them.

**Not derived from the README roster.** The kit's `<!-- gate-roster:begin -->`
block already carries full `checks/` parity (`check-readme-roster`) and
per-gate annotations naming each gate's subject surface, so deriving
`smoke/install.sh`'s registration from it would unify the roster and its
reasons on paper. Declined on a boundary, not on merit: it would turn
`smoke/install.sh` from an executable install recipe into a derivation over a
doc, which is what `kit-owned-install-recipe` (open, design-pending) asks
whether a kit's install-time roster should look like at all — deciding that
shape from inside this contract would pre-empt it sideways.

**The declaration valve.** Where a kit author judges an exit-0 or exit-1 omission
legitimate — a vacuous pass that is not real coverage is the honest case — that
kit's `smoke/install.sh` carries `# smoke-unregistered: <gate-name> — <reason>`
beside its registration block: kit-local, sitting where a reader looking at
registration already is, and readable off the vendored copy the harness has in
hand. Both fields are read, the name to match and the reason into the harness's
report. Three shapes of stale valve are findings, all caught at the same
transition because a declaration surface's failure mode is going stale: a
declaration naming a gate that *is* registered; one naming a gate that kit does
not ship; and one naming a gate the probe already exempts — that last is the
anti-inversion clause enforced, since a reason written where none is owed is a
reason nobody reads and the beginning of a maintained roster. A declaration line
missing either field is likewise a finding.

**The counts are permanent, and the cost is reported rather than cached.** The
clean line carries three: gates registered, self-declared (the corroborated
exit-2 set), and hand-declared. The hand-declared number is the one that says
whether the derivation is decaying into a maintained roster, so it is printed
every run rather than left to an audit. The phase's own wall-clock and verdict
split ride a second line,
`CONSUMER-SMOKE: accounting — <n> unregistered gate(s) probed in <ms>ms (<s> self-declared, <h> hand-declared, <u> unaccounted)`,
printed
on a red run as well as a green one, since a cost measured only when everything
already passes is a cost nobody sees while fixing. If the added wall-clock is
material, the sanctioned response is to report it — never to sample, and never to
cache a verdict across runs, a cached exemption being a maintained exemption
wearing a derivation's clothes.

**Never at pre-commit.** `run-consumer-smoke.sh` is a `bin/` tool, never a
registered gate, so the accounting costs the pre-commit battery exactly nothing.
The two sibling callers of `csmoke_vendor_and_install` do not run it: they build
the same baseline for a different assertion, and charging them for a verdict they
do not consume would be a cost with no reader.

**CI tool provisioning.** A gate whose oracle shells out beyond the
checkout+bash baseline (a renderer, a language runtime) is provisioned in the CI
backstop (`gates.yml`), not just locally — a green pre-commit on a dev box that
carries the tool is not a green CI, and the divergence surfaces only after push.
The provisioning step rides the consumer's filled workflow instance, never the
generic template (§templates/gates-workflow.yml), since the dependency joins the
toolchain only for a consumer that registers the gate.

**Starter-template conformance.** A kit that ships a starter template (in
`templates/`) ships it battery-clean: the template must pass the **full
battery** — every vendored kit's gates — when copied verbatim into a
combined-kit consumer, not merely the shipping kit's own gates. Kits compose,
and the first combined tree is where a per-kit-clean template still reddens a
foreign kit's gate. The obligation is mechanical, not ritual: where a kit ships
such a template, its `smoke/install.sh` installs it **verbatim** (no fill-in)
as the governed surface, so a template regression against any kit reddens the
harness instead of waiting for a hand-run validate proof. That "mechanical" claim
is only as good as the scratch battery's coverage, which is why *The registration
accounting* above is its enabling mechanism: a template installed verbatim but
linted by a gate nothing registers is ritual wearing mechanism's clothes, and
that is precisely how a starter queue shipped reddening its own kit's gate with
no harness able to see it. A template that
composes with a downstream kit's contract carries that kit's inert scaffold —
queue-kit's starter queue ships lifecycle-kit's iteration header so the
verbatim copy clears the stage gates too, and a single-kit adopter deletes it.

Producers and consumers: `smoke/` content is produced by the kit author at
kit-landing time and consumed by the harness's install and violation phases;
the expected-gate name (violation.sh line 1) is read by the red-phase
assertion; `SMOKE_KIT_ROOT` is produced by the harness per invocation and read
by the scripts to copy from their own kit; the harness verdict is consumed by
the validate-stage ritual (which gates on the success token) and is the natural
CI entry point (wiring CI is out of scope here).

The **accounting phase** is produced by `run-consumer-smoke.sh` on every
invocation — no enabling knob, since the harness *is* a consumer's
`consumer_smoke` validate suite, so the producer is reachable in the real
configuration and not only under test. It reads the vendored kits' `checks/`
directories, the scratch `scripts/gates.list`, and the vendored
`smoke/install.sh` declarations, all present in the scratch tree once
`csmoke_vendor_and_install` has run; its findings are consumed by the same
harness verdict, adding to an existing channel rather than opening one. Its
**three counts** are read on the clean line by the operator at the validate
transition, and the hand-declared count specifically by whoever owns the
iteration, as the number whose growth says the derivation is decaying into a
maintained roster. The **gate exit contract** (§Output contract) gains a second
reader in that phase, beside `run-gates.sh` — no gate is modified and none
acquires an obligation it did not already carry; what the phase adds is the
disambiguation of exit 2, which the contract deliberately leaves general.

## Per-component contracts

### lib/gate.sh

The family's single sourced library — values + adapters, never gate structure.
It gives a gate author the fail-closed guard `fail_closed`, the walk adapters
(`gate_find` / `GATE_GREP_EXCLUDES` / `gate_path_pruned` over the dirs
`GATE_SDK_PRUNE_DIRS` names),
the registry helpers that resolve a check consumer-first across kit dirs
(`gate_resolve`, `gate_kit_roots` / `gate_kit_roots_rel`, `gate_check_dirs` —
the multi-kit resolution path other kits' gates ride), and the `# graph:`
manifest readers (`gate_expand_couples` and its siblings §The `# graph:`
manifest). How each derives its result lives in the source; the invariants a
reader needs outlive the refactor that renames a helper:

- The commit-surface value adapters (`gate_msg_pattern_files`,
  `gate_commit_types`) are the single home of the banned-pattern set and the
  commit-type roster, each documented at its gate (§check-commit-msg,
  §check-commit-subject).
- `gate_self_repo_prefix` is the single identity derivation `check-md-refs`'
  resolver (canon-kit/SPEC.md §check-md-refs) and the reference-link producers
  share, so an emitted link and the pass that validates it cannot derive
  divergent identities; it ships no repo name — the provenance seam holds.
- `gate_fixture_suites` is the single source both the CI workflow
  (`.github/workflows/gates.yml`) and evidence-kit's validate config loop over,
  so adding a kit enrols its fixtures with no hand-list to drift.
- `gate_kit_roots_rel` emits the roots repo-root-relative — the anchor the
  couples globs share.

**Resolution splits into a declaration path and an invocation argv.** A gate's
implementation may live in a compiled subcommand while its declaration stays a
tracked text file, so the two jobs the one resolver used to serve stop having the
same answer:

- `gate_resolve` returns the **declaration path** — the file whose text carries
  the `# graph:` manifest and the `# spec:`/`# assertion` directives. Within each
  search dir it tries `<name>.sh` first, then `<name>.gate` (§The `# graph:`
  manifest names the descriptor). Dir order stays consumer-first and `.sh` beats
  `.gate` **within a dir**, which is what preserves registry-plus-shadowing: a
  consumer shadowing a kit's ported gate with its own shell script still wins.
  Every text reader is unchanged by the second spelling, because
  `gate_manifest_field` greps the manifest out of whatever path it is handed and
  has never required the file to be shell. A dir carrying both spellings for one
  name is ambiguous dispatch and is red (§check-gate-substrate-parity assertion
  A), never resolved by ordering.
- `gate_command` returns the **invocation argv**, one element per line: the
  one-element `<dir>/<name>.sh`, or the two-element `<binary> <name>`. Its
  callers are the execution sites — §run-gates, §run-gate-tests, and
  §gen-pre-commit, which does not execute the argv but *emits* it into the hook.

The binary's path is the knob `GATE_SDK_NATIVE_BIN` (§Layout and configuration),
never a literal. An **absent or non-executable** binary when a registry member
dispatches to it is a harness error — **exit 2, never a skip and never a pass**.
This is §Fail-closed contract applied to dispatch: the failure a skip would
create is a battery that silently stops running a gate whenever a build is
missing, which is the worst available outcome.

`fail_closed` must be passed *only* a status that genuinely means the check
could not execute (an awk/jq/parser crash) — never `grep`'s exit 1, which is
the expected "no match"; the caller draws that line at the capture site.

### lib/inject.sh

The marker-bounded insert/replace every kit's agent-file injector shares —
one function, `inject_marker_block <file> <begin> <end>`, taking the inner
block content on stdin. It writes `<begin>` + the piped content + `<end>` into
the target: replacing the span between an existing marker pair (inclusive) in
place, or appending a fresh block when the markers are absent, so a re-run
never duplicates. A begin marker without its end is a malformed target — it
refuses (exit 2) rather than guess the bounds; a missing target file is exit
2. On success it echoes the action taken (`appended`|`replaced`) for the
caller to report. Block-content *generation* stays with the caller (the
lifecycle roster, the doctrine digest) — this helper owns only the placement
mechanism, so a second injector adds no second copy of the awk replace logic:
`doctrine-kit/bin/install-doctrine.sh` and
`lifecycle-kit/bin/install-lifecycle.sh` both ride it. A sourced library, not
a gate: exercised end-to-end wherever an installer that rides it runs
(doctrine-kit and lifecycle-kit `smoke/install.sh`).

### lib/declaration.sh

The tightened-gates declaration grammar — **two container arms over one token
predicate**, sourced by three callers. The token predicate is a bare gate name
(`DECL_TOKEN_RE`); the container is the only thing that differs between the arms,
which is what keeps two surfaces from re-opening the same defect from opposite
directions.

- **The markdown arm** — a note's `## <section>` bullet lead tokens
  (`decl_section_bullets` for the container alone, `decl_section_tokens` for the
  verdict). A bullet's
  lead token is the backticked, unbolded bare gate name directly after the bullet
  marker; a bullet shaped any other way yields *no* token rather than a stripped
  one, which is what makes the bolded and bold-and-backticked spellings visible
  instead of silent.
- **The record arm** — a declaration file's data lines, one bare gate name each
  (`decl_record_tokens`). That surface is deliberately markup-free, so the
  spelling question does not arise on it at all. A missing file is the empty set
  rather than an error: a tree that has never declared one is not thereby
  malformed.

The status a caller branches on: **0** resolved, with the declared set on stdout
(empty for an explicit `None`); **1** unparsed while not `None`, with the
offending lines on stdout — which is empty when the section held no bullet at
all; **2** the named section is absent. The record arm has no `None` body and so
no status 2.

The markdown arm reports the **trichotomy** the grammar defines: an explicit
`None` body (the resolved empty set), a non-empty token list, or
unparsed-and-not-`None`. The third arm is the one that earns the helper. A
non-`none` section yielding zero tokens must not become "no allowed reds": on a
green battery that passes silently over a note naming several gates, and on a red
one it fails *loudly with a false message*, accusing the note of an omission it
did not make. The assertion is not leniently disarmed there — it is severed from
the artifact it claims to read. So the helper refuses, and the refusal is what
closes the class permanently: no future markup variant can disarm the assertion,
only red it.

Callers, all four named: `bin/upgrade-smoke.sh` at its declaration-resolve step
uses both arms (§upgrade-smoke); this repo's `check-tightened-gates-grammar`
uses the markdown arm's verdict at each note it walks; `scripts/check-tightened-gates-note-parity.sh`
also uses both arms, comparing a note's `Tightened gates` section
(`decl_section_tokens`) against its `DECL_FILE` argument's record set
(`decl_record_tokens`); and `scripts/check-release-bump.sh` uses the markdown
arm's *container* alone, counting bullets across the note's declaration-bearing
sections. That last caller is why the container and the token predicate are
separable rather than one pass: Behavior-changes lead tokens are legitimately
prose phrases, so the bump derivation needs the bullets without the token
predicate. Before this helper the container was stated three times and two of
the statements already disagreed on whether a bullet marker could be indented,
so the section a bump was derived from and the section an allowed-red set was
parsed from were not guaranteed to be the same section. A sourced library, not
a gate, so it owes no `good/`+`bad/` pair; its runtime lock-in is
`gate-tests/lib-declaration.test.sh`. The record arm is also exercised through
`scripts/gate-tests/check-tightened-gates-note-parity`'s own `good/`+`bad/`
pair, whose `tightened-gates.txt` fixture drives `decl_record_tokens` via the
gate's `DECL_FILE` argument. The direct unit test keeps its place — it is
still the arm's runtime lock-in — standing on its own rather than on an
absence that is not there.

The helper carries no section name and no gate name of its own — both are the
caller's arguments, and it takes no configuration. That is where the seam falls:
the parsing is kit mechanism, the parsed content is the consumer's.

### lib/test-hermetic.sh

The bespoke-test hermeticity bootstrap, sourced as the first act of every
`gate-tests/*.test.sh` (§run-gate-tests, enforced by §check-test-hermetic). It
derives the kit roster from its own location — the `gate-sdk` and `*-kit`
subdirectories of gate-sdk's parent, a name-glob that needs no config to
bootstrap and reaches the kits whose loader lives in `bin/` (context-kit,
drift-kit) as well as the `lib/` loaders — and for each exports
`<KIT>_CONFIG_FILE` (name uppercased, `-`→`_`) pointing at one shared empty file
at `${TMPDIR:-/tmp}/gate-sdk-hermetic-empty.sh`, created idempotently with `: >`:
fixed path, always empty, so no trap (a test's own `trap … EXIT` stays
unclobbered) and no growth. Every kit loader shares one strict shape — fail
closed (exit 2) on a set-but-missing `<KIT>_CONFIG_FILE` — and the shared
*existing* empty file is a no-op source for it. Knob-free by
design: a config-pinning tool cannot itself be configured by the surface it
pins. A test that must exercise real config overrides after the source (a later
assignment, or an `env -u <KIT>_CONFIG_FILE` prefix so the loader falls back to
its cwd-relative default) — ordering wins, no opt-in flag needed.

### run-gates

Aggregate runner: executes every `gates.list` member in one shot, timing each
(`<tmp-dir>/gate-timings.txt`, `<gate> <elapsed-ms>` per line + `TOTAL` —
uncommitted by design: a measurement, not state). A member that resolves
nowhere is a failure, not a skip. Exit 0 only when every member passed.

The output contract is **quiet green, loud red**. A passing gate prints
nothing: its captured output is discarded and the run ends with the summary
line alone, whose executed-gate count (`All N gates passed.`) is the
roster-collapse tripwire — a battery that silently shrank shows a smaller N. A
failing or erroring member prints its `===== <name> =====` banner and its
captured output verbatim, always — the red path is the feedback channel and
never quiets. `GATE_SDK_VERBOSE` (any non-empty value) restores the full banner
roll, the on-demand reading for the vacuous-pass tripwire (a "0 files scanned"
clean line is visible only in the gate's own banner). Env over flag by the kit
convention: one mechanism serves the interactive run, the generated hooks, and
any CI wrapper without an argv contract change. Gates themselves are untouched —
each still prints its single clean line per the output contract (§Output
contract); the runner captures it.

`run-gates.sh --for <path> [<path>...]` is the path-scoped selector, the
agent-callable half of the oracle-first rule: it resolves the registry exactly
as a bare run, then runs only the members whose *effective trigger* (`trigger=`
else `couples=`, expanded through `gate_expand_couples_var`) glob-matches at least
one given repo-relative path. Registry order and per-gate output are unchanged;
a bare `run-gates.sh` keeps its behavior. The loop this buys — edit → run
coupled gates → read the verdict+help — is strictly cheaper than reading gate
source to predict a verdict; the producer is a mid-edit session or a delegated
agent's gate-driven worklist, needing no new config (every gate already carries
the `# graph:` manifest the selection reads).

The match is **one matcher, defined as identical to the generated hook's
staged-path matching**: the glob step is `gate_staged_matches` in `lib/gate.sh`,
whose body `gen-pre-commit` emits verbatim as the hook's `staged_matches` (the
self-contained hook and the selector share one source, held in sync by
check-graph's freshness assertion §check-graph), and both draw the `# graph:`
fields through `gate_manifest_field` + `gate_expand_couples_var` — the single reader
§The `# graph:` manifest names. Two hook behaviors the selector reproduces
beyond that matcher: a `trigger=*` gate is selected for every path, and a
`mode=staged` gate — whose hook branch matches by git pathspec (exact path or
subtree prefix), a second mechanism — is selected exactly when that branch would
run it and receives its matching paths as positional args, as the staged branch
does. A divergence between what the hook would run for a staged path and what
`--for` runs for the same path is a bug against this contract. When no gate
couples to a given path the selector prints an explicit `no registered gate
couples to <path>` note and exits 0 — an ungoverned path is a fact, not a
failure; the selector is a `bin/` tool, never a registered gate, but its own
plumbing stays fail-closed (an unreadable registry or unresolvable member exits
non-zero).

### run-gate-tests

Golden-fixture runner. Each `<tests-dir>/<gate>/` holds `good/` + `bad/` case
dirs; the runner `cd`s into the case dir and invokes the gate with the args in
the case's `args` file (`#` lines stripped). `good/` must exit 0 (and, when
`good/expect.txt` exists, satisfy it); `bad/` must exit 1 and satisfy
`bad/expect.txt` — a rejection expectation is required, so the *right*
finding fired.

**The case runs whatever the member dispatches to.** The invocation resolves
through `gate_command` (§lib/gate.sh), so a case runs `<binary> <name>` for a
`.gate`-declared member exactly where it runs the script for a shell one, and
the executable guard applies to the resolved argv's first element. This is what
makes the fixture pair an **executed** parity oracle across both substrates
rather than a shell-only one — `check-gate-fixture-coverage` asserts a pair
*exists* and never runs it, so without this the pair would be an unrun
assertion the moment a gate ported. The ordering is binding: a ported gate's
pair passes against the subcommand **before** the script it replaces is
deleted, never after. argv[0] is absolutized before the `cd`, because the binary
knob's default is deliberately a repo-relative path.

**The output contract is asserted here, at runtime** (§Output contract). A
`good/` case must emit the canonical `^<NAME>: clean (<parenthetical>)$` line
and a `bad/` case must emit a `help:` remedy line — on top of exit code and
`expect.txt`. This applies to **every** fixtured member on either substrate, so
it is a strengthening for shell gates too, not merely a replacement for a source
grep that a ported gate leaves nothing to read. An `expect.txt` is a **conjunction**: every non-blank line must
appear literally in the case's combined output, and the case fails when any one
of them does not. So **a case pinning two findings writes two lines**, which is
what the plural form is for — the semantics both files share, `good/`'s being
optional to supply and identical once supplied. Matching stays
**order-independent** (requiring the printed order would couple every fixture to
a report ordering several gates do not fix and none contracts, and a fixture
pinning two findings cares that both fired, not which printed first), and a
blank or whitespace-only line asserts nothing — it is a separator, not a pin, a
line `grep -F` would match against any output at all. A failing case names
**every** missing line rather than the first, so one re-run resolves the case
instead of revealing one absent pin per run. Exit 2 from a gate marks the
fixture malformed (harness error, distinct from logic failure). Fixture-pair hermeticity is the `cd` into the case
dir: a gate resolving its `<KIT>_CONFIG_FILE` under the cwd finds only the case's
own files (and a fixture may ship its own cwd-relative config deliberately).
`<tests-dir>/*.test.sh` unit tests run after the pairs; each must exit 0 — and
each runs with the **invoker's** cwd (repo root in this repo's battery), so
absent a pin a gate it drives silently inherits this repo's consumer config. The
hermeticity contract is therefore explicit: every bespoke test sources
`lib/test-hermetic.sh` (§lib/test-hermetic.sh) as its first act — pinning every
kit's `<KIT>_CONFIG_FILE` to one shared empty file so the gate runs on kit
defaults, not the consumer's posture — with `check-test-hermetic`
(§check-test-hermetic) enforcing the pairing. A case that deliberately exercises
config overrides *after* the source (a later assignment, or an
`env -u <KIT>_CONFIG_FILE` / per-invocation prefix) by ordering. A tests dir may
hold fixture pairs, bespoke unit tests, or **both**, and the runner exits 2 only
when it holds **neither** — the emptiness case, where the invocation named
nothing to test at all. So a **gateless** kit ships bespoke tests with no fixture
pair to give: it registers no gate to build a pair *around*, yet its `lib/` and
`bin/` still need testing, and the discovery rule already recognizes that kind of
kit (`gate_kit_roots` keys on `checks/` **or** `smoke/` — §lib/gate.sh). A
unit-test-only dir prints `0 pairs` beside its unit count, so the shape reads as
deliberate rather than silent. This widens what a *tests dir* may contain, never
what a *gate* may omit: a registered gate still owes its `good/`+`bad/` pair
(§check-gate-fixture-coverage). Rehoming such a test in the owning kit's own runner is
ruled out — `check-test-hermetic`'s assertion A enumerates
`<kit-root>/gate-tests/*.test.sh` only, so a test outside that directory silently
escapes the bootstrap-or-marker obligation that is the whole reason this lane
lives here. The runner is a
test layer parallel to the gates, never a `gates.list` member — so, like
§upgrade-smoke's tool, it owes no `good/`+`bad/` pair of its own and a pair
would sit outside `check-gate-fixture-coverage`'s registry authority set,
audited by nothing. Its own coverage is the bespoke
`gate-tests/run-gate-tests.test.sh`, which drives it over scratch fixture trees
to pin the expect-line conjunction above; the inner invocation is bounded by
handing it a tests dir holding fixture dirs and **no** `*.test.sh`, so it runs
pairs and returns.

### run-consumer-smoke

The scratch-consumer install+violation harness (§Consumer smoke): vendors the
kit roots into a fresh temp repo, drives each `smoke/install.sh`, asserts the
full battery is green under zero config, runs the registration accounting over
every shipped-but-unregistered gate, then fires each `smoke/violation.sh`
and asserts the battery reddens at the named gate before restoring. A `bin/`
tool, never a `gates.list` member — it is pre-commit-unfit by runtime budget
and is the proof that the kit defaults hold on a vendored-kit tree.

### upgrade-smoke

The two-phase upgrade proof, `bin/upgrade-smoke.sh` — the third caller of
`csmoke_vendor_and_install` (§Consumer smoke), reusing the same green baseline
before it diverges. Where run-consumer-smoke proves a *single* release's
defaults hold, this proves the *transition* between two: it vendors every kit at
a **FROM** ref into a scratch consumer, installs and asserts the baseline is
green (a red FROM baseline is a broken tag — exit 2, not an upgrade finding),
then replaces the vendored kit directories wholesale at a **TO** ref and
regenerates the generated artifacts — the contract's consumer phase-A steps
(docs/install.md §The upgrade contract). It asserts **determinism** (the scratch
consumer's `git status` shows changes only under the kit roots and the two
regenerated artifacts — the pre-commit hook and the graph) and then, over the
phase-B battery, that the **red set is a subset of TO's tightened-gates
declaration**. A new N+1 gate
is *not* in this consumer's `gates.list` (the phase-A sync never re-runs the
installer, so it does not run in phase B); the smoke asserts the declaration's
sufficiency for the gates that *do* run, and the upgrade skill
(lifecycle-kit/SPEC.md §templates/upgrade.md) is the executor that registers the
new ones. A
red gate absent from the declaration, or a declaration that does not parse while
reds exist, is a fail (exit 1); usage/environment failure is exit 2 (the gate exit
convention). A malformed declaration is a contract violation rather than a broken
environment, which is why it takes exit 1 and not 2.
A `bin/` tool, not a gate — no `good/`+`bad/` fixture pair is owed;
the `upgrade` validate suite running it (scripts/evidence-config.sh) is its
evidence, at ~2× run-consumer-smoke's cost since it runs the battery twice in
scratch (accepted as validate-stage cost, never pre-commit).

**What it does not cover.** The transition it proves is the *vendored kit
directories* moving FROM→TO — phase A replaces them wholesale, in tree. It never
re-runs an installer, so a consumer's **cross-version init path** is outside its
reach entirely: a green `upgrade` suite is evidence about kit contents, not about
whatever activation surface a consumer ships to deliver them. State it here
rather than leaving it inferable from the phase-A step list, since the suite's
name invites the wider reading.

**The declaration resolves on two arms, both over §lib/declaration.sh's one
token predicate.** A **tagged TO** resolves its version from the `v*` tag
pointing at it and its declaration from the `docs/posts/` note whose front-matter
`release:` names that version — the Tightened-gates section's bullet lead tokens,
whose grammar docs/install.md owns. An **untagged TO** — the `HEAD` default, and
so every run of the standing pre-release assertion — reads
`<workflow-dir>/tightened-gates.txt` out of TO's tree instead. It is *this* arm
that makes the assertion satisfiable by an iteration that tightens something: the
old rule resolved no version, so no note, so an empty declared set, so a red set
that had to be empty — which no tightening iteration can be until the moment it
is tagged. The empty-declaration rule survives as the narrow case (an empty
declared set still forces an empty red set), not as the universal one.

**The tightened-gates declaration surface**, `<workflow-dir>/tightened-gates.txt`,
is a tracked checked projection (§The workflow directory) whose path derives from
`GATE_SDK_WORKFLOW_DIR` — no knob of its own, since a knob naming this file would
add a way to configure the assertion away without adding a way to satisfy it
honestly. It always exists, header-only when the declared set is empty, so
"absent" is never a state a reader must interpret. Its data lines are one bare
gate name each and nothing else: no markup, no prose, no ordering significance —
a rationale column would be a field the smoke never reads and the note's bullet
prose already owns. Only gates that ship **inside a kit** are declared, because
those are the only ones a consumer's vendored tree runs; a gate living solely in
the consumer's own gates directory cannot appear in a vendored tree and is not
part of any release's allowed-red set. Being tracked is load-bearing rather than
incidental: the smoke reads the file out of a `git archive` of TO, which carries
tracked content only.

Its **producer** is the build stage that lands or tightens a gate, appending the
name in the same unit (lifecycle-kit/templates/stages/build.md). Build is the
only stage that knows what it tightened at the moment it tightens it, so the
declaration is written from knowledge rather than reconstructed from a red — and
an assertion that discovers its allowed-red set from the gate it was meant to
check is its own trigger. A gate that lands **new** is appended too: docs/install.md
defines the note's section as one bullet per gate that landed new or got
stricter, so a surface holding only strictly-tightened gates would make the
composition lossy. It costs the assertion nothing — containment is red ⊆
declared, so a declared gate that never reds is inert.

It **accumulates**, and that shape is chosen rather than inherited. Tightened
gates is a *release*-level aggregate, not an iteration-level one: several
internal iterations batching into one external release is a shape this repo
wants, and under it a build stage authoring note prose directly would write into
an artifact that does not exist yet and whose version it cannot know. Appending
to a buffer composed once at the release boundary is correct under batching and
degenerates gracefully to the one-iteration-one-release case. So an iteration
closing on `release none` or a deferral carries its declarations forward, which
is exactly what the next release's note must inherit. RELEASING.md §The procedure
composes the note's Tightened-gates section from it at step 1 and drains it —
truncating to the header, never clearing the file — at the tag in step 4.

Knobs — config-via-env in the `<KIT>_<KNOB>` shape, defaults this repo's layout,
each read exactly once at the resolve step:

- `GATE_SDK_UPGRADE_REPO` — the kit-source git repository (default: the
  enclosing repo's toplevel). A consumer points it at their checkwright clone;
  the smoke never touches the network.
- `GATE_SDK_UPGRADE_FROM` — the FROM ref (default: the source repo's newest
  `v*` tag; none resolvable is exit 2, not a skip).
- `GATE_SDK_UPGRADE_TO` — the TO ref (default: `HEAD`).
- Scratch base is the existing `GATE_SDK_TMP_DIR` knob; the extracted trees and
  the consumer are `mktemp`-created under it and trap-removed.

Producers and consumers: the smoke's verdict (exit code + assertion output) is
produced by the `upgrade` suite each validate stage, or by a consumer invoking
the script pre-upgrade; it is consumed by the validate session's evidence file
(this repo) or the operator's go/no-go on a consumer tree. The
`GATE_SDK_UPGRADE_*` knobs are produced by the invoking environment (defaults
emitted by the script itself, so the zero-config run works here) and read only
at the resolve step. The declaration path is derived from `GATE_SDK_WORKFLOW_DIR`
at the same step. The tightened-gates declaration is produced by the build stage
that lands or tightens a gate, appending to that surface, and composed
by close into the note at the release boundary; it is consumed at three named
transitions — here at the resolve step on either arm (the allowed-red-set parse),
by close when it composes and drains, and by the upgrade skill reading the note
as the consumer's registration checklist. That third reader is unaffected by the
two-arm resolution: it reads the *note*, which is unchanged as an artifact, and
only the note's Tightened-gates section changed its source. The grammar's owner
is docs/install.md §The upgrade contract; its implementation is
§lib/declaration.sh, and this repo holds the corpus to it with
`check-tightened-gates-grammar`. The surface has a second consumer-side reader:
`check-tightened-gates-note-parity` compares it against the note composed from it
while that note is still untagged, so the compose-then-drain flow specified here
is held equal at its one comparable moment rather than by review.

### gen-pre-commit

Emits the generated git hooks from the per-gate `# graph:` manifests. A
`tier=precommit` gate becomes one trigger block in `<hooks-dir>/pre-commit`,
shaped by `trigger=`/`mode=`; a `gen=manual` region round-trips from the
current hook. A `tier=commit-msg` gate becomes one unconditional invocation in
a second hook, `<hooks-dir>/commit-msg`, passing that hook's `$1` (the
prospective-message file git supplies) through to the gate — the message is
rejected before the commit exists, whereas a history scan would find a leak
only after push, when the remedy is a destructive rewrite (that CI/history
backstop stays with the deferred hosted-attestation rung). `--emit` prints the
pre-commit hook to stdout and `--emit-commit-msg` the commit-msg hook
(`check-graph` compares against each); `--write` rewrites `pre-commit` always
and `commit-msg` only when a `tier=commit-msg` gate is registered. Adding a
gate to either hook is manifest-only — there is no second hand-wiring step to
drift. The emission is deterministic (no timestamps) so the committed hooks are
byte-stable. Both emitted hooks carry the quiet-green wrapper in their
generated header (§run-gates owns the contract): each gate invocation's output
is captured and reprinted only when that invocation fails (then the uniform
failure report as before) or when `GATE_SDK_VERBOSE` is set, and a fully green
hook run prints one summary line carrying its executed-invocation count. The
wrapper lives in the emitter's heredoc, so the freshness assertion carries any
change into the committed hooks. A `trigger=`/`couples=` `kit:<glob>` token is emitted *expanded*
(via `gate_expand_couples_var`), so adding a kit later reddens `check-graph`
(committed hook ≠ `--emit`) until regeneration — the freshness gate keeps the
static hooks honest across a kit-set change.

### install-hooks

One-time per-clone opt-in: sets `core.hooksPath → <hooks-dir>` (and
`blame.ignoreRevsFile` when `.git-blame-ignore-revs` exists). The wiring is
hooks-dir granular, so it enables every generated hook (`pre-commit` and, when
present, `commit-msg`) with no per-hook step. Refuses to point at a nonexistent
hooks dir — generate the hook first. Then, as the
apply-and-verify rung, runs `check-identity` once immediately after enabling
`core.hooksPath` (resolved through the registry, so a consumer shadow wins):
the fresh clone learns of a wrong-identity or wrong-remote mapping before its
first commit — the moment the push-identity half is cheapest to fix — and the
gate's exit status surfaces through this script's.

### check-shellcheck

Invariant: every `*.sh` directly under the consumer gates dir, each
vendored kit's `lib/`, `bin/`, `checks/`, and `templates/`, and each directory
named in `GATE_SDK_LINT_EXTRA_DIRS` passes ShellCheck at `-S warning` (the
self-lint contract). A missing `shellcheck` binary is exit 2 — a gate that
cannot run is not clean.

A `.gate`-dispatched member is outside this corpus **with cause** — there is no
shell to lint, and `cargo clippy` at deny-warnings is the substrate equivalent
(§Meta-gate conservation for the binary substrate, which owns the reasoning).

That derivation is also the answer to "does anything lint my workflows?", and on
its own the answer is no: `.github/workflows/*.yml` sits under no kit root and
under no gates dir, so it is unreached here by construction. §check-action-run-shell
is the sibling that reaches it, extracting the shell out of `run:` block scalars
and linting each at this gate's severity.

The knob **appends to** that derived set and never replaces it, so a consumer
that sets nothing keeps the shipped coverage exactly and a consumer that sets it
can only widen. It exists because the kit-root predicate (§lib/gate.sh) is what
puts a directory in scope, so a shipped script under no kit root — a consumer's
own tooling, a runnable walkthrough, an installer — is lintable but not linted
until its directory is named. Positional arguments remain a full scope override
(the hermetic fixture affordance), not an addition.

**The knob widens what the gate scans, not when the hook fires it.** The
generated hook's trigger comes from this gate's `# graph:` couples
(`scripts/*.sh,kit:*.sh`), and `kit:` expands to kit roots — a set a consumer
knob has no way to join. The kit family itself is fully covered and never was
the gap: the hook matches with `[[ "$f" == $pat ]]`, where `*` spans `/`, so
`<kit>/*.sh` already matches every script in every subdirectory of that kit.
What is genuinely uncovered is a directory the knob adds and nothing else
names: an edit there is linted by the full battery and by CI, but not by the
hook at commit time, so the finding arrives one tier later than for every other
script in the tree. Closing it is not a matter of widening the glob — the
trigger derives from a manifest a *kit* ships, and a kit manifest that named
consumer paths would carry consumer content across the provenance seam, which
is the thing kit literals are forbidden to do. The honest fix is a
consumer-expandable token in §The `# graph:` manifest, and that is a contract
change tracked on the queue rather than an adjustment available here.

### check-gate-output

Invariant: every `gates.list` member's source contains both a `: clean`
success emission and a `help:` remedy line (the static half of the output
contract). Presence is checked, not correctness — whether the clean line
actually fires is the `good/`-fixture job; whether the remedy text is accurate
is human review.

### check-gate-fail-closed

Invariant: every `awk`/`jq` command-substitution capture in a `check-*.sh`
gate handles its subprocess exit status — `fail_closed`, an inline
`|| { … }` guard, a captured `=$?`, or an explicit `# fail-closed-exempt:`
opt-out. Only `awk`/`jq` captures are checked: `grep`'s exit 1 on no-match is
expected, here-string captures read data already in memory, and arithmetic
`$((…))` is never matched. A parser wrapped inside a shell function is not
visible to this static scan; the opt-out covers residual false positives.

A `.gate`-dispatched member is outside this corpus **with cause**: the defect —
branching on a captured value's emptiness when the subprocess died — is
unrepresentable once a fallible call returns a `Result` that cannot be ignored
(§Meta-gate conservation for the binary substrate). A real substrate win, not a
gap.

### check-gate-fixture-coverage

Invariant: every `gates.list` member either ships a `{good,bad}/` fixture pair
(searched across the consumer tests dir, then each vendored kit's
`gate-tests/`) or carries a `# no-fixture: <reason>` header annotation. The authority set is the
registry — the gates that gate the tree — not every `check-*.sh` file. A
half-built pair is a defect regardless of any opt-out. The gate cannot
mechanically distinguish "infeasible" from "stopgap"; honesty is upheld by the
reason text.

### check-gate-assertions

Invariant: every `### <gate>` subsection in the family SPEC whose contract
enumerates its assertions (a count-word followed by an enumeration noun and a
labeled span) is coupled to a matching `# assertion <label>:` marker set in
the gate's code — the marker label set equals the contract's label span, and
its size equals the count-word. This catches the prose-vs-code drift an
internal count⟺span check cannot: a contract can be internally consistent
while the code grew a sixth assertion.

Discovery is first-paragraph-scoped, requires the enumeration noun
(`assertion(s)`|`axes`|`axis`|`checks`) adjacent to the count-word
(`two`…`nine`), and requires the first following parenthetical to be a
single-char `(X)` label — four filters that exclude sibling-gate mentions,
follow-on sentences, hierarchical axis/sub-rule contracts, and count-words
with non-enumeration nouns. With no spec argument the gate scans
`<gates-dir>/SPEC.md` when present plus each vendored kit's own `SPEC.md`;
each matched heading resolves to its gate source through the registry path. Honest residual: the marker catches editing one
side without the other, but not adding an assertion while forgetting *both*
its marker and the contract. A first paragraph that embeds the literal pattern
in example prose self-matches — the failure is loud (a false positive forcing
a reword), never a silent miss, so it is accepted. Requires GNU awk.

### check-gate-substrate-parity

Holds the dispatch seam honest: a gate's implementation may move to a compiled
subcommand, but not by quietly deleting the declaration other gates read or the
record of what that move costs. Usage
`check-gate-substrate-parity.sh [gates-dir] [conservation-doc]`; the two-arg form
steers the fixture pair onto hermetic copies of each surface. Three assertions.

- **assertion A — declaration uniqueness.** Each `gates.list` member resolves to
  exactly one declaration. A dir carrying both `<name>.sh` and `<name>.gate` is
  ambiguous dispatch and is red, rather than being silently settled by
  `gate_resolve`'s within-dir precedence: that precedence exists so a consumer
  can *shadow* a kit's gate, and using it to paper over a half-finished port
  would hide the state a port passes through.
- **assertion B — subcommand parity, both directions.** The set of `.gate`
  descriptors across the resolve dirs equals the binary's reported subcommand
  roster (`--list`). A descriptor naming no subcommand is a gate that cannot
  run; a subcommand with no descriptor is a gate nothing declares. With
  descriptors present and the binary absent or non-executable the gate exits 2,
  never 0 — the §Fail-closed contract, since "cannot verify" and "verified
  equal" must not share an exit code.
- **assertion C — disposition coverage.** Every substrate-sensitive member
  carries a disposition line in §Meta-gate conservation for the binary
  substrate. The set is **derived at runtime** — a member whose expanded
  `couples=` covers the declaration path of a registry member — so it cannot
  drift from a maintained roster. **This is the anti-vacuity assertion**: a new
  meta-gate over gate source, added later by a session that never read the
  conservation section, reds until its disposition is recorded.
- **assertion D — one writable home for the manifest.** No file under the
  implementation tree (`GATE_SDK_NATIVE_SRC`, default `native/src`) carries a
  manifest-class annotation. The comment leader is matched as `#`, `//` or
  `/*`, so the assertion is **language-agnostic**: it holds for whatever
  language sits behind a descriptor, not for Rust alone. This enforces the
  reader partition below, and it is a live assertion rather than a convention —
  two writable sources of one truth drift silently, and this is the truth every
  build-free reader depends on.

It stays a **shell** gate: a gate that audits the port is not a gate the port
may consume, or assertion B would be checking a roster through the very binary
whose roster is in question.

### check-test-hermetic

Two assertions guard a run against ambient state: (A) every bespoke
`gate-tests/*.test.sh` under the configured kit dirs either sources
`lib/test-hermetic.sh` or carries a `# hermetic-exempt: <reason>` marker line;
and (B) a credential-managing smoke script — one that assigns a `*_CRED_FILE` —
pins every own-kit bin call to a `*_CRED_FILE` path on the invocation line, or
carries the same `# hermetic-exempt:` valve.

Assertion A's exposure: `run-gate-tests` runs fixture pairs `cd`'d into a case
dir (hermetic by construction) but runs the bespoke unit tests with the
invoker's cwd, so a gate a unit test drives silently inherits the consumer's
`<KIT>_CONFIG_FILE` resolution — a test can green under the consumer's posture
rather than kit defaults (attested: 689cd9c). The bootstrap is the fix (one
shared pin, not one per test §lib/test-hermetic.sh); the marker is the valve for
a test that establishes hermeticity otherwise (constructing its own cwd).

Assertion B's exposure: a smoke script drives its kit's bins under the real
`$HOME`, so a bin that resolves its credential file from the ambient `~/.claude`
(the `<KIT>_CRED_FILE` default) reads live login state — a call whose verdict
then turns on the wall-clock age of the operator's credential, non-deterministic
by construction (the delegation-kit `usage-verdict` login-window reroute is the
attested case). The fix pins the cred at an absent path, so no ambient auth
event leaks in; the trigger is the script's own `*_CRED_FILE` assignment (a
smoke script that manages no credentials is never held to the rule), and the
scan keys on the `$SMOKE_KIT_ROOT/bin/` own-kit-bin convention, so no kit's
credential-consuming bin roster is spelled out in gate code.

With no argument the gate runs both — each `gate_kit_roots` kit's `gate-tests/`
(A) and `smoke/` (B); a positional arg scans the named gate-tests dir(s) (A, the
mode the fixture pair drives); `--smoke [dir...]` scans the named smoke dir(s)
(B). Tier `precommit`; the `# graph:` couples `kit:gate-tests/*.test.sh`,
`kit:smoke/install.sh`, and `kit:smoke/violation.sh` (`dir=one`, a one-way audit
over the test and smoke trees).

### check-assertion-strength

Invariant: **an assertion is at least as strong as its own failure message.** In
a smoke or gate-test script, a guard that discriminates only zero from non-zero
must not carry a failure message naming a verdict token that the invoked
script's declared exit contract binds to one specific non-zero exit code.

The exposure is not a false green today but a **masked regression**: the
attested case is delegation-kit's smoke guarding a `usage-verdict` call with a
bare `if`, which accepts PAUSE (1) and STALE (2) alike under a message asserting
specifically that the call "did not PAUSE". A STALE regression then reports
itself under the wrong name — worse than an honest silence, because it sends the
reader to the wrong place.

**The declared exit contract.** A script may declare its exit codes in its
header comment block. Both live declarations sit inside a wider `# usage:` block
and are written `#   exit: …` with leading whitespace, so the parser keys on
`^#[[:space:]]*exit:`, not a bare `# exit:` prefix. The grammar: each uppercase
token binds to the nearest preceding integer on the line, yielding a token→code
map. Tokens admit internal hyphens, so `RESET-OK` reads as one token rather than
two. A token bound to more than one code, or to code 0, is **not discriminable
and is skipped** — that rule is load-bearing, not housekeeping: a truthiness
guard discriminates code 0 exactly, so a message naming a code-0 token claims no
more than the guard established, and the rule is what keeps the gate off the
honest `did not verdict OK` neighbour sitting a few lines from its own fixture.

This is the same seam move `check-test-hermetic` makes with the own-kit-bin
convention: the verdict vocabulary (`PAUSE`, `STALE`, and any other kit's
tokens) is derived from the callee's own declaration and **never appears as a
literal in gate code**. A gate shipping a vocabulary would publish it; a gate
deriving one cannot.

**Reach is opt-in and the gate never widens it.** A callee declaring no `# exit:`
header is simply out of reach — the gate demands the header of no one, so the
unit imposes no new obligation on every script in the tree. Stated honestly, the
day-one reach is thin: of the two live declarations, `usage-trend.sh` names its
codes in prose with no uppercase token and so yields an empty map, leaving
`usage-verdict.sh`'s `PAUSE`→1 and `STALE`→2 as the whole live vocabulary
(`OK` and `RESET-OK` bind to 0 and are skipped). One declaring script, two usable
tokens; the success line reports the call count so the reach stays visible rather
than implied.

**Detection.** For each guard invoking a declaring script through the own-kit-bin
convention (`bin/<name>.sh` resolved against the scanned dir's kit root) whose
discrimination is truthiness only — the call sits in an `if` or chains into
`||`/`&&` — the gate reads the guard body's failure text within a bounded window,
truncated at the next such call so guards cannot excuse each other, and reds when
that text names a discriminable token while the guard compares no status to that
token's code.

**Valve.** A `# assertion-strength-exempt: <reason>` marker, for a guard that
establishes the outcome by other means. It is an **inline per-site directive**,
sited on the guard it excuses — its own line, its immediately preceding comment
block, or its body — and its discipline is the adjacent `<reason>`, exactly as
`# fail-closed-exempt:` and the `# hermetic-exempt:` valve it is modelled on.
It is deliberately **not** queue-linked: §check-gate-exemption-tasks scopes
itself to `# exception-list:`-tagged arrays and rules this class out as local and
self-evident via its adjacent comment.

**Honest limit.** The gate reads guard shape, not semantics. An assertion
weakened without a token-naming message is out of reach, as is any callee
declaring no exit contract; a message naming a token bound to several codes is
skipped rather than guessed. What it catches is precisely the attested shape — a
message more specific than the guard behind it. Like its siblings, a false
positive is loud (a forced reword or an explicit exemption), never a silent miss.

With no argument the gate scans each `gate_kit_roots` kit's `smoke/` and
`gate-tests/`; positional args scan the named dir(s), the mode the fixture pair
drives. Tier `precommit`, matching its sibling; the `# graph:` couples
`kit:smoke/*.sh`, `kit:gate-tests/*.test.sh`, and `kit:bin/*.sh` (`dir=one`, a
one-way audit — an edit to a scanned script *or* to a declaring header re-fires
it). Configuration adds **no new knob**: the scan roots come from the existing
kit-roots derivation, as `check-test-hermetic`'s do.

### check-gate-exemption-tasks

Invariant: every element of an `# exception-list:`-tagged array in a
`check-*.sh` gate carries exactly one of two disposition annotations —
`# until: <slug>` (temporary; must resolve to a live task in the queue file's
New Features / Technical Debt / Deferred sections — *live* meaning the slug on a
**bullet lead line** within that span, one per entry, never every bold token in
it) or `# permanent: <reason>`
(structural out-of-scope). An element with neither, a `# until:` slug that is
Done-only or missing, or elements sharing the array's opening `=(` line are
violations. Scope is in-script exemption arrays only; inline per-site
directives (`# fail-closed-exempt:`, `# no-fixture:`) stay out — they are
local and self-evident via their adjacent comment.

**The live-section span is positional, and a consumer's optional queue tiers
ride on that.** The scan opens on the first of those headings and closes on the
done or lessons heading, with **no reset on an unknown heading**, so any
section a consumer places *between* the deferred and done sections is swept
into the live set. queue-kit's optional icebox tier (queue-kit/SPEC.md §The
icebox tier) states its position as a contract for exactly this reason: an
exemption backed by an iceboxed task keeps resolving, because an iceboxed task
is unbuilt. gate-sdk cannot depend on queue-kit for the section set, so the
coupling is carried by both SPECs rather than by code, and **nothing enforces
section order** — an icebox placed after the done section silently drops those
slugs from the live set. The same-shaped scan in drift-kit's `kpi-deferred-age`
*does* reset on an unknown heading and so excludes the tier; same shape,
opposite behavior, both wanted.

**The live slug of an entry is the bold lead-in of its bullet lead line**, one
per entry — the line predicate
`^[[:space:]]*-[[:space:]]+\*\*[a-z0-9][a-z0-9-]*\*\*`, from which the first
bold token is the slug. Reading every bold token on every line of the span
instead is a fail-open, and the loud kind of one: entry prose bolds ordinary
words, so any bolded lowercase word joins the live set, an `# until:` resolves
against no task at all, and the gate's clean line still claims every element
declares until-with-live-task. The whitespace tolerance is the format rather
than a tolerance — an indented bold lead-in is a sub-task, and sub-task slugs
share the one global namespace an `# until:` may name (queue-kit/SPEC.md §The
queue format), so a column-0 anchor would drop that class and fail *closed* at
the other end. A holder that wants entries only is entitled to that narrower
anchor; the level is the reader's choice, the predicate is not.

The seam is the one the section set above already takes: **gate-sdk cannot
depend on queue-kit for the lead-line format, so the coupling is carried by both
SPECs rather than by code.** Sourcing `lib/queue.sh` from gate-sdk is refused on
layering — gate-sdk is the substrate every kit vendors, and a queue format must
not become a precondition for running any gate; a consumer vendoring gate-sdk
and no queue-kit still gets a working exemption gate. Re-implement and cite from
both ends is not a new ruling: queue-kit/SPEC.md §The queue format already
states it for drift-kit, with a cycle rather than a layering inversion as the
reason. No knob: the lead-line shape is not a consumer's posture but the one
format the `# until:` contract is written against, and a consumer free to
redefine it could redefine it back into the fail-open this closes.

**The honest cost, stated with its size: eight independent holders now carry
that predicate and no gate enforces their agreement** — `queue_live_slugs` and
`queue_roadmap_entries` (queue-kit/lib/queue.sh), `spec_queue_slugs`
(canon-kit/lib/spec.sh), the inline scans in queue-kit's `check-task-names` and
`check-queue-entry-budget`, `bin/queue-index.sh` (which carries it twice, in the
`--extent` and `--icebox-candidates` walks), drift-kit's `kpi-queue-net-delta`
`pool()`, and this gate. They are named because a grep for the *function* names
finds two of them and the rest are inline scans no naming convention surfaces —
which is how the count was twice under-stated before it was surveyed. The risk
and the cost do not sit in the same place: a **set builder** with a wrong
predicate fails silently, in wrong membership, which is this gate's own defect
class, while a **per-bullet extractor** fails loudly and locally — a missing
index row, an extent measured wrong. So a format change costs all eight edits
and endangers only the few that build sets. Accepted on the same ground as the
section-set residue above: a cross-kit code dependency would cost more than the
divergence risk. Whether eight hand-coupled parsers earn a shared derivation, a
conformance test, or a gate is a real question and a **different unit**; this
section neither answers it nor forecloses it.

Clean-line contract: the line reports the exemption-array count **and** the
derived live-slug count. Both are §run-gates' vacuous-pass tripwire applied to
this gate's two sets, and they read in opposite directions — an empty array set
means the gate ranged over nothing, while an absurdly *large* slug set is the
fail-open above, silent by construction because every `# until:` then resolves.
A number on the line is what makes the second readable without an audit, and its
drift is the signal that the predicate has come loose from a reformatted queue.

### check-graph

Invariant: the `# graph:` manifest on every `gates.list` member is well-formed
and consistent, and the pre-commit hook is the faithful generated projection of
the manifests. The manifest grammar a gate author writes against: four required
keys (`couples`/`dir`/`valve`/`tier`) plus the optional `mode`/`trigger`/`gen`,
each `couples=`/`trigger=` token a syntactically valid glob (or a `kit:<glob>`
couples/trigger form that expands before the vocabulary and parity checks), and
surfaces drawn from the declared vocabulary when one exists. The `valve=` value
follows the cycle rule: a `dir=bi` gate spanning a declared-leading and a
declared-lagging surface carries `valve=PROPOSED`; a leading-only gate may carry
either; a gate with no leading surface carries `valve=none`. From that grammar
the check derives its guarantees — each `couples=` surface is covered by the
gate's `trigger=` globs (trigger defaulting to couples), so editing a coupled
surface always fires the gate; and the committed pre-commit hook, the commit-msg
hook (when any gate is `tier=commit-msg`), and the coupling-graph projection at
`GATE_SDK_GRAPH_ARTIFACT` (default `<gates-dir>/CHECK-GRAPH.html`) each equal
their generator's `--emit` output, down to every emitted asset href resolving
under the artifact's own directory. The remedy lines print the resolved artifact
path, so a consumer that republishes it (this repo serves `docs/check-graph.html`)
is always offered its own regenerate command; how each guarantee is asserted
lives in the check. A `# graph:` manifest embedded in a `SPEC-*.md` amendment
body is held to the glob grammar but not to the vocabulary or hook-parity — the
gate it describes is unbuilt, so its coupled surface may itself be design-ahead;
parity re-fires through the normal registry path once the gate lands.

Dual-couple manifest: the artifact path is a knob, but check-graph's own
`# graph:` manifest is kit-shipped static text a consumer never edits, so it
cannot read the knob. It lists **both** artifact homes as couples — the
gates-dir default and `docs/check-graph.html` — so the generated hook
re-fires on whichever a consumer publishes to. For a default consumer the docs
path is simply an inert trigger pattern that never stages; couples↔trigger
parity holds because the hook derives from the same manifest.

Rule content is config, not code: `<gates-dir>/graph-vocab.sh` may declare
`GRAPH_VOCAB` (the legal surface tokens; empty/absent disables the vocabulary
check), `GRAPH_LEADING`/`GRAPH_LAGGING` (the assertion-C sets; absent
disables cycle-valve classification, leaving the no-leading `valve=none` rule),
and `GRAPH_LAYERS` + `graph_surface_layer()` (the projection's subgraph
grouping; absent renders one layer). The `--amend-only [dir]` mode runs only
(G) over a given directory, letting the fixture pair exercise it hermetically.
Coverage ruling: a full `couples ⊇ find-globs` parity check over arbitrary
shell is undecidable — neither cheap nor low-FP — so check-graph does not carry
it, and the couples⊆trigger guarantee already ensures editing a *coupled*
surface fires the gate. The
statically resolvable slice of that parity is carried by its sibling
`check-reads-couples` (§check-reads-couples); the undecidable remainder stays
the author's duty under §The `# graph:` manifest.

Theme seam (`emit_graph`): the emitted HTML artifact bypasses the consumer's
site generator, so it renders foreign beside the rest of a docs host unless the
host theme is inlined — and the theme is consumer-specific, so the emitter must
not hardcode it. `GATE_SDK_GRAPH_THEME` (default `<gates-dir>/graph-theme.sh`,
mirroring `GATE_SDK_GRAPH_VOCAB`) names a consumer file sourced when present at
every emission. It may define override functions, the `graph_surface_layer`
pattern: `graph_theme_css` emits the `<style>` element's body (replacing the
kit's default stylesheet), `graph_theme_header` emits a fragment directly after
`<body>` (site chrome above the kit header), and `graph_theme_footer` emits a
fragment directly before `</body>`. An absent file or an undefined function
falls back to the kit default, so a themeless consumer's output stays
byte-identical. Determinism: the freshness assertion's in-memory emission and the `--emit` a
consumer redirects into the artifact resolve the same theme path, so the
byte-compare holds; the artifact stays generated-only, a styling change landing
in the theme file (or the emitter), never a hand-edit.
Self-containment is unchanged: injected content is inline, and a theme emitting
a relative asset href must resolve under the artifact dir or the asset-href
assertion is red — the existing gate already polices the link-the-site-stylesheet
shortcut into inlining. Its complement is the **external-ref assertion**, over
the same in-memory emission: every absolute (`://`-carrying) `href`/`src`
attribute value and every ESM `import` specifier must prefix-match the allowed
set, or the gate reds naming the URL. The set is seeded with `emit_graph`'s own
pinned-major mermaid ESM import from the jsdelivr CDN — the one sanctioned
external reference the kit itself emits (the diagram renderer is client-side, and
inlining a megabyte-scale library into a byte-compared artifact is the worse
trade), always allowed so a consumer cannot lock the kit's emission out. A
consumer's chrome may need more — this repo's theme links its docs host and its
source repo — so `GATE_SDK_GRAPH_EXTERNAL_REFS` (space-separated URL prefixes,
default empty) adds consumer-sanctioned prefixes: rule content the kit must not
hardcode, the `graph-vocab` seam. The assertion runs whole-tree and hermetically
via `--refs-only` (the fixture's `check-graph-refs.test.sh` drives it, as
`--amend-only` drives assertion G). Honest limit, now narrowed: the scan covers
`href`/`src` values and import specifiers only — CSS `url()` and an inline-script
`fetch()` still pass unseen (review-caught), and `xmlns=` namespace values are
neither `href` nor `src` and are out of scope. Dark mode is the theme owner's disposition: the kit default keeps its
light+dark scheme, and because the emitted mermaid init keys on
`prefers-color-scheme`, a theme's chrome must honor that query too or it clashes
with a dark-rendered graph on the same page. This repo's `scripts/graph-theme.sh`
supplies the docs-host tokens, header, and footer (both schemes), so
`docs/check-graph.html` reads as the same site.

Render cap (`GATE_SDK_GRAPH_MAX_EDGES`, default `100000`): Mermaid refuses to
render a flowchart whose edge count *exceeds* `maxEdges` (its own default is
`500`), painting an error graphic in place of the diagram — so a graph that
outgrows the cap is a published page that never draws, invisible to the
byte-compare freshness assertion because the HTML is still well-formed. The
emitter writes the knob's value into the page's `mermaid.initialize` call, and
the **render-cap assertion** re-derives the cap and the edge count from the same
in-memory emission and reds when `edges > cap`. Modelling Mermaid's boundary
exactly (`edges == cap` renders; `edges > cap` throws) keeps the gate honest
against the renderer rather than a guessed margin. The default is effectively
unlimited for a trusted, manifest-bounded graph; the knob exists so a consumer
whose graph legitimately dwarfs it, or who wants the guard to bite sooner, sets
its own ceiling — the emitted page and the assertion always read the one value,
so they cannot disagree. A missing `maxEdges` in the emission falls back to
Mermaid's own `500` default, catching a regression that drops the init key
entirely. The assertion runs whole-tree and hermetically via `--cap-only` (the
fixture's `check-graph-cap.test.sh` drives it, as `--refs-only` drives assertion
H).

### check-reads-couples

Invariant: for every registered gate, every **statically resolvable recursive
walk** in its source has its tracked read-set covered by the gate's expanded
`couples=` — the reads⊆couples half `check-graph` leaves to the author
(§The `# graph:` manifest).

**A member resolving to a `.gate` makes this gate refuse: exit 2, never a
pass.** Its walk parser reads shell, so a binary gate yields zero walks and the
gate would print `clean` — the single worst vacuity available at the substrate
seam, because the absence of findings is indistinguishable from an absence of
coverage. Refusing is the §Fail-closed contract applied to a corpus the scanner
cannot see, and it stands until a binary-side equivalent exists
(§Meta-gate conservation for the binary substrate).

This is check-graph's coverage sibling: check-graph
proves editing a *coupled* surface fires the gate; check-reads-couples proves
the couples name every surface a resolvable walk *reads*, so no recursion hides
a surface the couple never listed.

The tractable class is stated as the invariant, because deciding what arbitrary
bash reads is undecidable and a gate must not over-claim. A **walk** is a
`gate_find` or `find` at command position; a walk is *resolvable* when its first
directory argument is one of three shapes — a quoted literal repo-relative path,
`"$KIT"[/sub]` (the analyzed gate's own kit dir, from the source's
`checks/`-parent), or `"$REPO_ROOT"[/sub]`. For each resolvable root the gate
enumerates the **tracked** files under it (`git ls-files`, filtered by a literal
`-name '<pat>'` primary when one is extractable from the same invocation, else
unfiltered; `gate_find` walks additionally drop the pruned dirs) and asserts
every one matches at least one expanded couple under the manifest's own glob
semantics — segments never cross `/`, so path and glob must share a segment
count (a shallow one-level couple misses a file one level down, the
check-shim-restatement bug). A walk whose root does not resolve is **skipped and
counted** in the clean line: the gate claims only the resolvable class and says
how much it left undecided. Only tracked files need coverage (couples exist to
fire the hook on a tracked-path commit; a walk over `.tmp/` or generated state
has no commit to couple to) and only walks are analyzed — single-file reads and
`git ls-files` enumeration are out of scope.

Over-demand is absorbed one of two ways, never by weakening the glob semantics
to pass a near-miss: add the covering sibling glob (the correct fix), or mark
the deliberate uncoupled walk `# reads-couples-exempt: <reason>` on the walk's
own line or the line directly above (the `comment-tier-exempt` precedent — local
to the walk it excuses, auditable in place; a trailing marker excuses only its
own line). The marker's sole reader is this gate; the skip counter's sole reader
is the clean-line parenthetical, its honesty label for the undecidable
remainder. Manifest `tier=precommit trigger=*`: the unconditional trigger is
load-bearing, not laziness — the invariant breaks two ways, a gate edit that
changes a walk *or* a new subdirectory grown under a walked root, and no couple
glob can name a directory that does not exist yet. Its own `couples=` names what
it reads as content (every `checks/` dir plus `gates.list`); the tracked-file
enumeration is `git ls-files` metadata, not a content read, so it needs no
couple. The hermetic fixture affordance: positional gate-source arguments make
the gate analyze the given source(s) with `git ls-files` anchored to the case
dir, instead of walking the real `gates.list`.

### enforcement-map

`bin/enforcement-map.sh --emit` writes `docs/enforcement.md`: a kit-first map of
every check surface — kit, governed surface, enforcement class — derived from
the class registries so it cannot drift from what actually runs. It is
check-graph's sibling in shape (an emitter whose output a freshness gate
byte-compares), advisory by construction: it never joins `gates.list`, and a
*healthy* run exits 0 whatever registries are absent — while a misconfigured run
(a set-but-missing registry knob) exits 2, fail-closed. Every knob check runs
before the first stdout byte in both modes, so a misconfigured
`--emit > docs/enforcement.md` regen leaves an empty projection and a nonzero
exit, never a plausible partial page that byte-matches itself on the next
freshness check. Bare it prints a human header before the page; `--emit` prints
the page alone, for the committed projection.

Each enforcement class reads one registry, every registry defaulting to this
repo's layout through the owning kit's knob: **blocking gates** from
`gates.list` plus each gate's `# graph:` `tier=` field, the owning kit taken
from the same name-resolution walk the runner uses (a consumer-dir gate groups
as the consumer's); **advisory KPIs** from the drift-kit `kpis.list` registry
(`DRIFT_KIT_KPIS_FILE`); **guards** and **session warnings** from the
`PreToolUse` / `SessionStart` command hooks in the tracked harness settings file
(`CONTEXT_KIT_SETTINGS_FILE`, parsed with `jq`); **validate suites** from
evidence-kit's suite config (`EVIDENCE_KIT_CONFIG_FILE`); and **monitors** — the
one class with no parseable registry — from a line-start
`# enforce: class=monitor <free-text>` marker a non-gate surface declares itself
with, greped under `GATE_SDK_ENFORCE_SCAN_DIR` (this repo's first carrier is the
site-health workflow — deployment truth, not tree truth). A marker is **dormant
in a template or fixture** — an inert copy-source — and **activates only where a
consumer copies the file into a live path**: the walk therefore prunes
`templates/` (a `grep -v`, the sibling-finder idiom) atop the `gate-tests`
exclusion `GATE_GREP_EXCLUDES` already carries, so site-kit's shipped
`templates/site-health.yml` marker projects no row while this repo's own copy of
it under `.github/workflows/` does. The `# enforce:`
grammar is the reusable name a future uncovered class adopts rather than growing
a bespoke registry.

Cross-registry reads are a reporting surface (the drift-report precedent), and
the emitter distinguishes two absences per class, converging on the strict
set-but-missing rule the kit config loaders share (§lib/test-hermetic.sh): a
registry *not adopted* — its knob unset with the default path absent — degrades,
leaving its section absent, so a gates-only consumer still gets its gate map; a
registry *adopted but broken* — its knob explicitly set to a missing path —
refuses (exit 2, a stderr line naming the knob). The settings knob additionally
checks parseability when set: set and `jq`-unparseable refuses likewise (a typo
inside an explicitly adopted file is the same laundering vector as a typo in its
path), while the default-path file unparseable keeps degrading — a stray
`.claude/` must not break a zero-config consumer — and `jq` *absence* keeps
degrading in both modes, a toolchain gap owned by the install requirements /
env-probe parity, not a config typo. The emitted
page — not this SPEC — owns the enforcement-class **taxonomy** prose (what each
class means and how hard it binds); the emitter's preamble heredoc is its single
source, and this section documents the emitter contract and cites the page for
the taxonomy.

The page is a docs-site artifact, so it carries the site's link topology. The
**taxonomy is a bulleted roster**, each class citing its *mechanism owner* — the
kit SPEC section that defines the class — through the reference-link grammar
(canon-kit/SPEC.md §The reference-link grammar): a self-repo blob link built from
`gate_self_repo_prefix` on the `CANON_KIT_DOCS_BLOB_REF` ref (§lib/gate.sh), so
the citation pins to the same ref `check-md-refs`' self-repo pass validates. The
citation degrades to the bare `<path> §<section>` text when the identity is
unknown (no origin) or the owning kit is unvendored (its SPEC untracked), so a
gates-only consumer keeps a resolvable page. The **kit column links each kit's
docs page** (`<kit>/index.md`, relative to the page under the docs root); the
`(consumer)` group owns no kit page and stays plain text.

### check-enforcement-fresh

Invariant: `docs/enforcement.md` byte-matches `enforcement-map.sh --emit` — the
check-graph / trajectory-freshness byte-compare pattern. Bare, it runs the
emitter and compares the committed page; given two arguments
(`projection-file emit-file`) it compares pre-baked files, letting the fixture
pair exercise it hermetically off the live registries. Fail-closed: a missing
projection, a missing emitter, or a non-zero emit is a red (exit 2), never a
false clean. Its `# graph:` manifest couples every class registry — the gate
sources (so a `tier=` edit re-fires), `kpis.list`, the settings file, and the
monitor-carrier workflows — beside the artifact itself, so any registry change
re-runs the freshness compare. The corrective its help text names is the
regeneration command, reachable because the gate rides the generated pre-commit
hook (the check-graph contract exactly).

### check-kit-enum

Invariant: for every `gates.list` member, a `couples=` set that literally names
two or more `gate_kit_roots` members under a common glob suffix must name
*every* kit root having tracked files matching that suffix. `kit:<glob>` deletes
the drift axis by derivation (the reader expands it to all roots); this gate is
the residual meta-check for the hand-list a future gate author writes anyway —
the fix its help text names is the token, not a longer list. Completeness is
measured against tracked files (`git ls-files` at the repo root), so a suffix no
kit carries forces nothing and the over-approximating token stays a superset of
what the gate requires. A lone named root is not a hand list (no completeness
obligation); the check engages at two. Fail-closed: an unreadable manifest, an
unresolvable registered gate, or a non-repo cwd is a red, not a skip. A member
with no `# graph:` line is `check-graph`'s finding, not this gate's.

### check-kit-registration

Invariant: every kit root `gate_kit_roots` enumerates is registered in the
consumer's human-facing docs — closing the prose-registry gap `check-kit-enum`
leaves open (that gate guards gate-coupling hand-lists, so a landed kit can
silently fall out of the registry docs). Two assertions: (A) **registry row** —
the registry doc carries, for each root, a markdown link *into* the kit root:
any target under the repo-root-relative prefix `<kit>/`, whether the bare
directory itself or a page beneath it such as `<kit>/index.md`. The prefix is
the assertion because the two registries address a kit differently — a source
tree browses to the directory, a rendered docs site must name the page — and
both are the same registration. A landed kit missing from the public kit table
is red; (B) **fixture-runner
line** — every kit root with
tracked `gate-tests/` files (`git ls-files <kit>/gate-tests/`) has a line in the
runner doc naming `<kit>/gate-tests` (the documented fixture-runner
invocation), so a kit whose fixtures never entered the battery is red. A kit
shipping no tracked `gate-tests/` files (drift-kit) owes nothing under B — the
carve-out is derived from the tree per run, not a standing list, so a kit that
grows fixtures loses it the moment they are tracked.

B is deliberately redundant with evidence-kit's `check-battery-roster`
(evidence-kit/SPEC.md §check-battery-roster), whose assertion (A) holds the same
runner doc against the configured suite roster and, where that config derives
the fixture suites from the `gate-tests/` roots, supersets B. The redundancy is
kept on a dependency direction: **a gate-sdk gate may not require evidence-kit
config**. The line is between reading it opportunistically and asserting over
it — §enforcement-map reads that config where a consumer has one and emits no
suite rows where it does not, which an assertion cannot do, since a gate with
nothing to compare against is either fail-closed or a false clean. So B is the
arm that survives a gate-sdk-only adoption — the more common shape — and
retiring it would trade a duplicate finding for a coverage hole. One omission
named differently from each side (a kit root here, a suite there) is a duplicate
finding, not a contradiction.

Config, the standard kit shape: `GATE_SDK_REGISTRY_DOC` (default `README.md`)
is A's doc, `GATE_SDK_RUNNER_DOC` (default `README.md`) is B's; both resolve
relative to the git toplevel, and an explicit positional argument
(`[registry-doc [runner-doc]]`) overrides. Fail-closed: a configured doc that
does not exist is a misconfiguration (exit 2, like `check-kit-enum`'s missing
registry), as is a non-repo cwd or empty roster — never a false clean. A
consumer keeping no prose registry opts out by not registering the gate in its
`gates.list`; there is no empty-knob valve. This gate retires close's manual
"does the kit table still reflect the kit set?" staleness check, narrowing that
step to the un-gated remainder (row descriptions, per-kit READMEs).

### check-readme-roster

Invariant: every kit README's register-the-gates block holds name-set parity
with the kit's shipped `checks/` basenames, both directions — over **both**
declaration spellings, `*.sh` and `*.gate`, since a ported gate is still a gate
the kit ships and would otherwise drop out of its README roster in both
directions at once. This is the
`check-install-toolchain` fork applied a second time: the roster names are
derivable (the `checks/` script basenames, extension stripped), the per-gate
annotation clauses beside them are hand prose, so a gate asserts parity over a
hand-maintained list rather than an emitter generating it — the list stays a
human-read register, the gate holds it honest.

Marker vocabulary: each kit README wraps its register block in
`<!-- gate-roster:begin -->` / `<!-- gate-roster:end -->` markers (the
`docs/install.md` `toolchain:begin` shape). Inside the markers, a line's first
`check-`-prefixed token is a roster name; everything after it — annotation
prose, `#` clauses — the gate never reads. Calibration: marker lines may carry
leading indentation, because a README nests the fenced block inside an install
list item; the roster scan trims surrounding whitespace before matching the
marker and the names.

Two assertions, per kit, over the roster set versus the `checks/` basename set:
(A) every shipped check appears in the README roster — a gate shipped but never
registered is red; (B) every roster name resolves to a shipped check — a roster
name naming no `checks/` script is red. Each finding names the kit and the
name.

Sweep: kit roots come from `gate_kit_roots` (the `GATE_SDK_KIT_DIRS` knob —
§Layout and configuration), the `check-kit-enum` / `check-kit-registration`
sweep shape. A kit root without a `checks/` dir is skipped-and-counted; a kit
root with `checks/` but no marker block in its README is red — the kit-landing
checklist (§Consumer smoke) has a kit that ships checks registering them.
Config reuses `GATE_SDK_KIT_DIRS`; no new knob. Positional form
`check-readme-roster.sh [root]` resolves relative kit roots against a fixture
tree (the case dir's `gate-sdk-config.sh` names the fixture kits), the sibling
meta-gates' hermetic-fixture shape; bare, it sweeps against the git toplevel.
Fail-closed: a non-repo cwd with no root argument, an empty roster, or an
unreadable README marker scan is exit 2, never a false clean.

### check-smoke-entry-guard

Invariant: for every kit root in the roster, every `smoke/install.sh` and
`smoke/violation.sh` present contains the entry-point guard expansion
`${SMOKE_KIT_ROOT:?` (§Consumer smoke's contract clause) — a mutating smoke
script run bare, outside the harness that exports `SMOKE_KIT_ROOT`, must abort
rather than write into the caller's repo. A missing guard is a finding naming
the file (the fix worklist). `install.sh`'s shipped guard is the precedent this
gate promotes from convention to contract; `violation.sh` joins it.

Honest limit: the gate asserts the guard's **presence**, not its **position** —
a guard placed below a mutating line still passes, so ordering (the guard
before the first mutation) is review's, not the gate's. Presence is the
mechanically decidable half; position would need a mutation model the gate does
not carry.

Sweep: kit roots come from `gate_kit_roots` (the `GATE_SDK_KIT_DIRS` knob —
§Layout and configuration), the sibling roster meta-gates' shape; a kit root
without a `smoke/` dir is skipped, and `violation.sh` is checked only where it
exists (it is optional — §Consumer smoke). Config reuses `GATE_SDK_KIT_DIRS`;
no new knob. Positional form `check-smoke-entry-guard.sh [root]` resolves
relative kit roots against a fixture tree (the case dir's `gate-sdk-config.sh`
names the fixture kits); bare, it sweeps against the git toplevel. Fail-closed:
a non-repo cwd with no root argument, an empty roster, or an unreadable smoke
script is exit 2, never a false clean. The `# graph:` couples the mutating
smoke scripts (`kit:smoke/install.sh,kit:smoke/violation.sh`, `dir=one`,
`tier=precommit`), so editing one re-fires the gate.

### check-core-files

Invariant: every path in the consumer's `core-files.list` manifest exists in
the worktree **and** is tracked (`git ls-files --error-unmatch`). Red on a
missing or untracked listed path — one existence-plus-tracked test catches a
plain `rm`, a `git rm`, and a listed-but-never-added path alike, with no
`--diff-filter` timing window that only sees the loss at some later stage.

The manifest is optional consumer config (the `graph-vocab.sh` pattern): the
path knob is `GATE_SDK_CORE_FILES_FILE` (default
`<gates-dir>/core-files.list`), registry-style — one repo-relative path per
line **or** a `kit:<path>` token, `#` comments and blanks ignored. An absent
manifest is clean with a note;
an empty or comment-only manifest is clean; a present-but-unreadable manifest
is fail-closed (exit 2). Calibration: the intentional-removal valve is the
manifest itself — retiring a surface means deleting its line in the same commit
that removes the file, a diff-visible edit that needs no exemption tag, so the
gate is re-scoped in the open, never weakened to pass.

A `kit:<path>` line derives one `<kit-root>/<path>` entry per `gate_kit_roots`
member through `gate_expand_couples_var` — the same expansion, the same root
set, and the same spelling `# graph:` `couples=` fields already use (§check-graph
owns them; nothing about them is re-specified here). What is new is only that
`check-core-files` is a second reader of it. A manifest with no `kit:` line
behaves exactly as before, so no consumer's manifest changes meaning.

**One restriction, and it is fail-closed.** The expansion is unconditional: it
emits `<root>/<path>` per root with no existence test. In a `couples=` field the
result is matched as a glob against tracked files, so a wildcard is meaningful
there. This reader instead *requires* each expanded path to exist and be tracked,
and "every kit has at least one file matching `checks/*.sh`" is a different
invariant from "this path exists". A `kit:` token carrying a wildcard is
therefore refused with exit 2 and a message naming the limit — one expansion, two
readers, one stated restriction, never divergent semantics.

Because a derived line makes a **new kit** change the checked set without any
manifest edit, the gate's `# graph:` couples the manifest *and* the derived
basenames (`kit:SPEC.md,kit:README.md`, `tier=precommit`): staging a twelfth
kit's spec or README on the commit that creates it re-fires the gate, so the
guarantee holds at the perturbation rather than one tier out at the whole-tree
battery. That battery stays the backstop for a
pure-deletion commit the `ACMR` pre-commit filter would skip.

**What a consumer derives, and what it hand-lists.** The manifest derives a
surface that is uniform across kit roots and hand-lists everything else, and
uniformity is decidable by running the expansion rather than by taste: a basename
every kit root carries expands to a set that wholly exists and is therefore
derivable, while a single-kit deliverable would expand to one existing path and
N-1 missing ones — proving it is a hand line rather than a derivation someone
failed to spot. Surfaces with no kit-root shape at all (a queue, repo-root
chrome, workflows, an installer entry point) stay an honest hand list.

**A derived line is worth taking even where another gate looks like it covers
the same surface, unless that coverage is direct.** The discriminator is whether
the other gate asserts the surface's existence *as its subject* or merely trips
over its absence while checking something else. Incidental coverage keys on a
property that can change for unrelated reasons, and it vanishes the moment that
property does — which is exactly the failure a core-file pin exists to prevent.

### check-identity

Invariant: every expectation in the `identity.conf` manifest matches this
clone's local git identity — a verification backstop for the fresh-clone gap
where an agent commits or pushes under the wrong identity and fails silently
(misattribution is unpurgeable without a SHA-breaking history rewrite; the
wrong-SSH-key symptom is a misleading "Repository not found"). Multi-identity
setups — a work and a personal account on one machine — make this the common
case for the integrator audience. **Scope fence:** the identity *mapping* stays
git's job (`includeIf`, `core.sshCommand`); this gate only asserts the mapping
actually applied here.

Two expectation kinds, both local reads (cheap, no network, no false positives
from a settled corpus):

- `email <expected>` — matches `git config user.email` exactly.
- `remote-host <remote> <host>` — matches `<host>` against the host part of
  `git remote get-url <remote>` by exact string. An SSH host alias is matched
  as the alias — that *is* the identity selector in multi-identity setups, so a
  scp-like `git@alias:path` compares as `alias`, and a `scheme://[user@]host/…`
  URL as `host`. A configured remote that does not exist in this clone is red.

The manifest is optional consumer config (the `graph-vocab.sh` pattern): the
path knob is `GATE_SDK_IDENTITY_FILE` (default `<gates-dir>/identity.conf`),
line-based `key value…` with `#` comments and blanks ignored. An absent, empty,
or comment-only manifest is clean with a note; a mismatch (or a manifest-named
remote that is absent) is a violation (exit 1); a malformed line — an unknown
key or wrong field count — is fail-closed (exit 2), never a false clean on an
uninterpretable manifest. A live run under CI (the vendor-neutral `CI` env var)
is clean-skipped ahead of the manifest reads: the server-side battery is not a
committing clone, so there is no local identity to misattribute a commit or
push with, and the CI runner's unset `user.email` is expected, not a violation
(fixture mode is unaffected — it exercises the comparison deterministically).
Enforcement is dual: the `# graph:` couples the
manifest at `tier=precommit` (a `git config` change to the mapping is not
diff-visible, so the whole-tree `run-gates.sh` battery is the real backstop for
the commit-identity half), and `install-hooks.sh` runs the gate once at opt-in
to cover the push-identity half (no pre-push hook is added — gate-sdk generates
only the pre-commit hook, and the setup rung plus the precommit tier already
cover the surface). A `--fixture <dir>` mode injects the clone's actual identity
(`git-config-email`, `git-remotes`) so the fixture pair is deterministic without
touching real git config.

### check-hook-exec-bit

Invariant: every tracked file in the hooks dir (`GATE_SDK_HOOKS_DIR`, default
`<gates-dir>/git-hooks`; override with the gate's first argument) carries git
*index* mode `100755`. The index is the checked surface because it is what a
fresh clone receives: git silently skips a non-executable hook, so a
`pre-commit` committed at mode `100644` disables the entire gate battery for
every clone — a catastrophe-class, invisible failure — and `install-hooks.sh`'s
per-clone `chmod` cannot repair a wrong committed mode. One `git ls-files -s`
reads the mode a clone would get, sidestepping the worktree bit entirely. A
non-repo cwd is fail-closed (exit 2); a hooks dir with no tracked files, or an
absent hooks dir, is clean (nothing committed to skip). The `# graph:` couples
the hooks dir at `tier=precommit`, and the whole-tree `run-gates.sh` battery is
the backstop for a mode-only change no `ACMR` content filter would surface.

### check-exec-bit

Invariant: every tracked `*.sh` path matching an exec-glob carries git *index*
mode `100755`, **and every tracked `*.gate` descriptor carries `100644`**. The
second is stated as an assertion rather than left implicit so that "a descriptor
is not executable" cannot read as "a descriptor is not covered": the descriptor
is data — a manifest and directives, never sourced and never run — and an
executable one invites a reader to run a file carrying no interpreter line. The
first class is by-path-invoked kit scripts — gate-sdk's runner
(`run-gates.sh`), drift-kit's collator (`drift-report.sh`), and lifecycle-kit's
entry preflight all invoke kit scripts **by path**, and a shebang'd `bin/` tool
is by-convention path-invocable — so a script committed `100644` degrades
silently in a fresh clone: a KPI plugin to `n/a (plugin failed)`, a
runner-invoked preflight to a skipped check. The index is the checked surface
because it is the mode a clone receives, and a `Write`-tool-authored script
acquires `100644` there regardless of worktree state; one `git ls-files -s`
reads it, sidestepping the worktree bit.

The subject class is two knobs (both join §Layout and configuration's roster).
`GATE_SDK_EXEC_GLOBS` is the space-separated glob set (globs match with `*`
spanning `/`); its default `*/checks/*.sh */kpis/*.sh */bin/*.sh` plus the
computed `<gates-dir>/check-*.sh` and `<gates-dir>/kpi-*.sh` covers the per-kit
exec dirs and the consumer gates dir (consumer gates and KPI plugins resolve
from the gates dir first, so they are by-path targets too).
`GATE_SDK_EXEC_PRUNE` (default `gate-tests fixtures templates smoke`) exempts
subtrees by path segment: fixture trees deliberately carry glob-matching paths,
and `templates/` members are copied content sourced at their destination, never
invoked in place. `lib/` needs no prune — sourced libraries match no default
glob.

Sibling, not overlap, with `check-hook-exec-bit`: that gate asserts the same
index-mode invariant over the hooks dir, a disjoint target class (hook files
are not `*.sh`, and no default glob reaches the hooks dir), so both gates
stand. The honest limit is shared: the gate reads the index, so a mode broken
only in an uncommitted worktree file is invisible until staged, and the
whole-tree `run-gates.sh` battery is the backstop for a mode-only change no
`ACMR` content filter would surface. A non-repo cwd is fail-closed (exit 2).

Argument mode (fixture capability): `check-exec-bit.sh [ls-files-dump]` lints a
canned `git ls-files -s` dump instead of running `git ls-files -s` from the
repo root, so a fixture is hermetic against the host repo's index (the
check-merge-attrs precedent). The `good/`+`bad/` pair runs on the argument
path; the bespoke `gate-tests/check-exec-bit.test.sh` builds a temp git repo
and re-stages a KPI at `100644` then `100755`, exercising the live
`git ls-files` path. Tier `precommit`.

### check-root-tiering

Invariant: every tracked top-level entry of the scanned tree (`git ls-files`
first-segment set; scan root is the optional second argument, default `.`) is
covered by an allowlist entry — an exact name or a glob (`SPEC-*.md`). The repo
root is the orientation surface a reader lands on; agent-authored repos
accumulate root scratch by reflex, so the allowlist makes a genuinely new root
surface a deliberate config edit rather than a silent drop, and keeps workflow
machinery under the configured workflow/gates dirs. A non-repo cwd, or an
unreadable allowlist, is fail-closed (exit 2).

The allowlist is optional consumer config (the `graph-vocab.sh` pattern): the
path knob is `GATE_SDK_ROOT_ALLOWLIST` (default `<gates-dir>/root-allowlist.list`),
registry-style — one entry per line, `#` comments and blanks ignored. An absent
file falls back to the built-in minimal orientation set (`README.md`, `LICENSE`,
the configured queue file, the configured agent file (`GATE_SDK_AGENT_FILE`,
default `CLAUDE.md` — the always-loaded agent file a consumer's harness reads,
`AGENTS.md` under an agent-file harness), `.gitignore`, plus the `SPEC-*.md`
amendment glob — a root-component amendment is a legitimate transient root
surface for any canon-kit consumer, so a gate that reddened every authoring stage
would only train bypassing). The intentional-new-surface valve is the manifest
itself: adding a root entry means adding its allowlist line in the same commit,
a diff-visible edit needing no exemption tag. The `# graph:` couples the
manifest at `tier=precommit`; the whole-tree `run-gates.sh` battery is the
backstop for a pure-addition commit outside the trigger's staged view.

### check-workflow-tiering

Two assertions, over `GATE_SDK_WORKFLOW_DIR`: (A) **partition totality** — every
member is tracked or ignored, never neither; (B) **header presence and form** —
every tracked member's first line is `# contract: ` with a payload matching the
pointer form or the version-marker form. Both are §The workflow directory's
rules; this section is the mechanism, that one the argument.

It deliberately does **not** re-resolve the pointer's path and heading:
`check-spec-pointer` already does, and a second resolver is the parallel copy
canon-kit's own tiering rule bans. The division is presence-and-shape here,
resolution there — the same split `check-comment-tier` and `check-spec-pointer`
already hold between them.

No new knob: `GATE_SDK_WORKFLOW_DIR` is the whole configuration surface.
Fail-closed (exit 2) on a non-repo scan root, on a workflow directory the knob
names but the tree lacks, and on an unreadable member. A directory member is
partition-checked and header-exempt — a header belongs to a file. Tier
`precommit`; the `# graph:` manifest couples the workflow dir and `.gitignore`.

### check-action-pinning

Invariant: every `uses:` ref in a scanned YAML file is immutable — a full 40-hex
commit SHA, or a repo-local `./` path to an in-repo composite action, which the
checkout already pins. A tag or branch ref (`@v5`, `@main`) reds: whoever owns
the tag can repoint it, so the code a run executes stops being the code that was
reviewed. The ref is the first whitespace-delimited token after a key-position
`uses:`, so a trailing `# v1.2.3` comment falls away and a commented-out step —
the copy-paste seed a template ships — is read like a live one.

Not `check-workflow-*`. This tree already spends "workflow" on §The workflow
directory (and `GATE_SDK_WORKFLOW_DIR`), so a second `check-workflow-*` gate,
over `.github/workflows/` instead, would collide on the reader's only
disambiguator.

The scan set is **derived, not rostered**: a `gate_find` walk for `*.yml` /
`*.yaml` from the scan root (the optional positional argument, default `.`, the
`check-root-tiering` form). Not a `git ls-files` enumeration — a tracked-files
filter would put the fixture pair's synthetic tree out of reach, since it is
untracked by construction. The walk covers `.github/workflows/` and the shipped
copy-outs (`templates/gates-workflow.yml`, site-kit's `templates/site-health.yml`)
with no roster to drift, and the shared prune set keeps it out of `gate-tests/`,
so the `bad/` fixture cannot red the whole-tree run. **No new knob** — the scan
set is derived and the prune set is the shared one (§Shared cross-gate values). A
tree holding no YAML exits clean on a zero count, the counted inertness that makes
this kit mechanism rather than a consumer gate: a consumer running no GitHub
Actions pays nothing for it.

What it deliberately does not assert is the trailing version comment. The comment
is not required, and where one is present the gate does not check that it names
the tag the SHA resolves to — that needs a network call, which breaks the
hermetic-gate contract, and demanding an unverifiable comment would be exactly
the trivially-true proxy §When a gate earns its place bars. The comment stays a
convention for review. The currency limit the pin does *not* buy is
§templates/gates-workflow.yml's.

Tier `precommit`; the `# graph:` couples the scanned YAML surfaces, `dir=one` —
a one-way audit.

### check-action-run-shell

Invariant: every GitHub Actions `run:` **literal block scalar** in a scanned YAML
file is ShellCheck-clean at `-S warning` under the dialect the step actually
runs. It closes the class §check-shellcheck's target derivation leaves open — a
workflow's `run:` body is shell that nothing lints, shellchecks, or executes
outside a push or a tag. It is not the only gate on that surface:
§check-action-gh-repo reads the same `run:` bodies for a **semantic** property
ShellCheck is structurally blind to, so a reader arriving here should not
conclude the `run:` surface has one gate.

Kit mechanism rather than a consumer gate, and the deciding fact is that kits
ship workflow templates carrying `run:` shell — §templates/gates-workflow.yml and
site-kit's `templates/site-health.yml`, the copy-outs consumers vendor. A
consumer gate could reach both in one tree while leaving them unlinted for every
downstream vendor, the same fix-the-instance shape §check-action-pinning
rejected; that the second template belongs to a *different kit* is what makes the
argument decisive, since no consumer gate and no site-kit-local gate covers both.
Named `action-`, not `check-workflow-*`, for §check-action-pinning's reason: this
tree already spends "workflow" on §The workflow directory.

**Scan set — derived, then narrowed to the subject.** Stage one is a `gate_find`
walk for `*.yml` / `*.yaml` from the scan root (the optional positional argument,
default `.`), pruned by the shared set, so `gate-tests/` is out and the `bad/`
fixture cannot red the whole-tree run. Stage two is the **Actions-shape
predicate**: a walked file enters the subject only if it carries a top-level
`jobs:` key (a workflow) or a top-level `runs:` key (a composite action).
Everything else is **skipped and counted**. The predicate governs *extraction as
well as refusal* — a file it skips is neither linted nor refused.

Whole-tree reach is the wrong boundary here even though §check-action-pinning
takes it. `uses:` with a 40-hex ref is self-limiting grammar; `run:` is an
ordinary word serving as a YAML key in more than one CI schema, so whole-tree
reach would lint foreign-schema text as shell — a stream of silent false
positives, strictly worse than a loud refusal and squarely what §When a gate
earns its place forbids. Two further reasons the narrowing is right rather than
merely safe: a gate whose name and reach disagree teaches every later reader the
wrong boundary; and gate-sdk has no standing to impose the literal-form
conformance rule below on a consumer's non-Actions YAML, where it ships no
template and owns no contract.

The `# graph:` manifest couples the **walked** surface rather than the matching
one — a file that *gains* a top-level `jobs:` key must retrigger the gate, and a
manifest naming only today's workflows would miss exactly that. The block tally
is a measurement, never a contract: this section carries the derivation so a
later commit adding a step cannot falsify it.

**The extractor** is one awk pass per file, keyed on block-scalar indentation,
and stays inline in the check script rather than moving to `lib/` — a helper
earns its place at a second consumer and there is none. Its rules, each of which
a prototype proved necessary by failing without it:

- **The key column is the column of the key token, never the list dash.** A
  `- run: |` item's dash sits left of the key; taking the dash's column as the
  body's indentation floor makes every sibling key of that step (`env:`, `name:`)
  satisfy "more indented than the block header" and be swallowed into the shell
  body, dedented by the body's indent, so `env:` arrives as `v:`. That is not a
  missed block but a false-positive engine, and it is the single most important
  line in the extractor.
- **The body is every following line more indented than the key column**, blank
  lines included, dedented by the first body line's indentation, ending at the
  first non-blank line at or left of the key column.
- **No block header is recognised while inside a block**, so a body containing a
  heredoc whose text is literally `run: |` stays shell instead of being
  double-extracted.
- **A comment line is never a header** — the commented example a shipped template
  carries is not a step.

**GitHub expressions.** `${{ … }}` is not shell syntax; left raw it is a parse
error. It is replaced per line by `${GHEXPR}`, a braced parameter expansion,
which presents to ShellCheck as the opaque runtime value a GitHub expression
actually is. A bare word does not work and the difference is measured, not
stylistic: a literal constant drags ShellCheck's constant-expression analysis
into firing on correct code, manufacturing SC2050 inside `[ … ]` and SC2194
inside `case`. The braced form causes no finding in any tested position.

**Dialect — resolved, never assumed.** Linting a block under the wrong dialect
manufactures false positives, so the step's effective shell comes from its
`shell:` sibling key, which may sit either side of the `run:` block:

| `shell:` | resolution |
| --- | --- |
| absent | `-s bash` — GitHub's documented default for a `run:` step on every hosted runner |
| `bash` (with or without arguments) | `-s bash` |
| `sh` / `dash` / `ksh` | the matching ShellCheck dialect — linting a POSIX body as bash hides the portability findings that dialect exists to surface |
| anything else (`pwsh`, `python`, a custom `{0}` template) | the block is **skipped and counted** — the body is not shell, so there is no shell to lint |

**Severity is `-S warning`**, the gate family's level, so one threshold governs
all ShellCheck lint in the tree. A future author lowering it inherits false
positives this gate creates rather than finds: **SC1091** ("not following: … was
not specified as input") fires on any block that sources a library, because an
extracted fragment has no resolvable source root. It sits at `info`, below the
threshold, so it does not bite today — recorded so that decision is made
knowingly.

**The fidelity limit.** What the extractor **refuses loudly** can never become a
false negative; what is **out of reach** is a stated cost; what is **out of
subject** is a boundary the kit declines to cross. The distinctions carry the
remedies, so they are kept apart.

*Refused — exit 2, naming the construct.* Each is detectable, so the gate stops
rather than guessing, and every refusal fires **only inside the Actions-shape
subject**: a folded block scalar (`run: >`, `run: >-`), because reassembling
folded lines needs YAML's folding rules and mis-folding manufactures findings;
an explicit block-scalar indentation indicator (`run: |2`), which can contradict
the indent derived from the first body line; a YAML anchor or alias as the `run:`
value, since no anchor resolution is attempted; and an unbalanced `${{` on a body
line. Refusing the folded form makes the literal form a conformance requirement
for a multi-line `run:` body *in an Actions-shaped file* — governance where
gate-sdk ships the template and owns the contract. The chomping indicators `|-`
and `|+` are ordinary spellings, handled and extracted with the body bytes
intact; silently skipping those would be the worst hole of the set, since an
author reaches for `|-` by habit.

*Out of subject.* **Other CI dialects that also spell a shell step `run:`.**
CircleCI's `.circleci/config.yml` is the concrete case — `- run: |` there is
genuinely shell and the extractor would handle it correctly. It is skipped
anyway, because gate-sdk ships no CircleCI template and owns no contract over
that file. This is a **decision, not an inability**, and the distinction matters
to whoever revisits it: widening the predicate is a governance question about
what the kit claims, never an engineering question about what the extractor can
parse.

*Out of ability.* **Single-line plain-scalar `run:` values** are not linted: a
plain scalar's text is governed by YAML's plain-scalar rules — a space-preceded
`#` opens a *YAML* comment, not a shell one — so recovering the shell honestly
means parsing the scalar, which is the dependency this gate declines. They are
counted in the output so the cost is visible on every run, and the class that
produces incidents is multi-line blocks. **Following a `uses:` call into another
repository** is beyond any tree-local gate; a called workflow's own blocks are
linted in the file that defines them, which carries its own top-level `jobs:`
key. **GitHub-expression injection** is a non-goal rather than an oversight: the
substitution above turns an interpolation into an opaque expansion, so the gate
cannot see the injection hazard of an unquoted `${{ }}` — a textual substitution
happening before the shell ever runs, and a different and worse class belonging
to a dedicated workflow-security linter.

Tier `precommit`; **no new knob** — the scan set is derived, the prune set is the
shared one, and the severity is the family's literal. A missing `shellcheck`
binary is exit 2, as §check-shellcheck models. A tree holding no YAML exits clean
on a zero count, the counted inertness that makes this kit mechanism: a consumer
running no GitHub Actions pays nothing for it.

### check-action-gh-repo

Invariant: in every Actions-shaped YAML file, a **job** whose `run:` bodies
invoke `gh` establishes a repository context — the job contains a checkout step
ordered before its first such invocation, or `GH_REPO` is set at workflow, job,
or invoking-step level, or every detected invocation in the job carries
`--repo`. A job satisfying none of them cannot resolve a target repository, and
nothing else catches it until a tag fires.

The class is attested rather than hypothetical: it took down `v0.17.0`'s
`release` job on its first live run, which died in seconds on `failed to run
git: fatal: not a git repository` before its first API call, with no Release
created and no assets attached. The failure mode is the worst-timed one
available — green everywhere, red only at the tag, on the release path itself.
§check-action-run-shell lints exactly that block and passes it: the shell is
valid, the variables are quoted, the control flow is sound. The defect is
**semantic**, an assumption about the runner's filesystem no syntactic linter can
hold.

Kit mechanism rather than a consumer gate, and the deciding fact is
§check-action-run-shell's: a *kit* ships a workflow template carrying `gh` calls
in its `run:` bodies — site-kit's `templates/site-health.yml`, a copy-out
consumers vendor. A consumer gate would cover one tree and leave every downstream
vendor of that template unchecked, and that the template belongs to a *different
kit* is what makes the argument decisive. Named `action-`, not
`check-workflow-*`, for §check-action-pinning's reason.

**Scan set and the split predicate.** The walk is §check-action-pinning's —
`gate_find` for `*.yml` / `*.yaml` from the scan root, shared prune set, no
roster and **no new knob**. §check-action-run-shell's Actions-shape predicate is
then split rather than borrowed whole, because this gate needs its two arms
apart: a top-level `jobs:` key makes a file the subject; a `runs:`-shaped
composite action is **skipped and counted**, having no job of its own and
inheriting the calling job's repository context, so the assertion belongs to the
caller; a file matching neither is outside the scan. Two lineages, named
separately because they are separate — §check-action-pinning contributes the scan
set and carries no Actions-shape predicate, §check-action-run-shell contributes
the predicate and no bearing on the scan set.

**The detector — the trigger and the `--repo` arm are one.** The gate must
already answer "does this line invoke `gh`" to arm at all, so the arm is
evaluated per detected invocation and is *universally quantified over the
detected set* rather than satisfied by a witness. One unprefixed call fails the
arm by construction; there is no search for a positive example to be fooled by,
and a job mixing prefixed and unprefixed calls — the false negative the arm was
designed against — reds. Over each `run:` body in the job:

- **Logical lines.** Backslash continuations are joined before matching, so a
  call split across lines is one unit and its `--repo` is found wherever on the
  call it sits. A detected call's `--repo` is looked for only within that call's
  own extent, so a second call on the same logical line cannot lend the first one
  its flag.
- **Command position.** A `gh` token counts only where a shell would read it as a
  command: opening a logical line, or following a command separator, an opener, or
  a command-introducing keyword. Whole-word matching keeps `ghost` and `gh-pages`
  out. The check owns the roster of separators, openers, and keywords, and
  **that roster is a calibration the in-tree corpus forced**, not decoration:
  `publish.yml`'s release job opens with `if ! gh release view`, and without the
  negation among the openers the gate would take a later call as the job's *first*
  invocation and run the checkout arm's ordering comparison against the wrong
  line.
- **Comment lines.** A logical line whose first non-blank character is `#` is not
  a command line — a leading `#` in shell is always a comment. A trailing `#`
  comment is left in place, which over-detects and is the safe direction.
- **Scalar forms.** A literal block scalar is scanned line-wise; a folded one is
  scanned the same way even though folding would join its lines, and a plain
  single-line scalar is scanned as one logical line. All three over-detect rather
  than skip, which is why this gate refuses none of the forms
  §check-action-run-shell refuses — it reads for one token, not for shell it must
  reassemble faithfully.
- **The bias, stated rather than left implicit.** Every ambiguity resolves toward
  over-detection. A false positive costs a workflow author one `GH_REPO:` line or
  one exemption marker; a false negative is the release-path failure this gate
  exists for.

**The checkout arm and its honest limit.** A step satisfies it when its `uses:`
ref before the `@` is `actions/checkout` **and** its line precedes the job's
first detected invocation; a checkout ordered after the call establishes nothing,
and the ordering comparison is free once both line numbers are in hand. Any other
means of establishing a git remote — a hand-rolled `git clone`, a different
checkout action — is outside the gate's theory and takes the valve. That limit is
stated rather than papered over with a looser match, because a looser match is
what turns a semantic assertion back into a syntactic one.

**The environment arm.** `GH_REPO` satisfies it when set at workflow-root `env:`,
at `jobs.<id>.env:`, or on the `env:` of the step whose body carries the
invocation. `gh` resolves its target repository from `GH_REPO` and from the git
remote, so no other variable joins the arm; a consumer relying on a different
mechanism takes the valve. The arm is the one the fixed instance used and the one
that survives maintenance: a job-level `GH_REPO` cannot be undone by someone
adding a new `gh` call, whereas per-call `--repo` can. **The step-level lookup is
load-bearing, not completeness** — the fixed instance sets `GH_REPO` on the
invoking step's `env:`, not at workflow or job root, so the one in-tree job this
gate exists for passes on the *narrowest* of the three lookups, and a gate
implementing only the outer two would red `publish.yml` on day one and read as a
false positive.

**The arms are disjoined per job, and each is quantified over the whole detected
set.** A job passes on the checkout arm, or because every detected invocation has
`GH_REPO` in scope, or because every one carries `--repo` — never by pairing a
`GH_REPO` step with a `--repo` step. Mixing them reds, which is a false positive
one `env:` line or one marker resolves, and the alternative — a per-invocation
disjunction — would let a job satisfy the invariant in a way no reader of it
could state.

**The valve.** `# gh-repo-exempt: <reason>`, taking the kit's established
`# <thing>-exempt:` marker shape. It binds by its own indentation: at or left of
the job-id column it precedes a job, at the step-list dash column it precedes a
step, and inside a step's keys it binds that step — a job-bound marker skips the
job, a step-bound one drops that step's invocations from the detected set and
leaves the job's others held to the arms. The reason is **required and
non-empty**; a bare marker is its own failure class with its own `help:` line,
since the valve's whole value is the sentence saying which arm the author is
standing outside of. That requirement is a deliberate tightening, not
conformance, and saying so is the honest form: no sibling enforces it —
`check-test-hermetic` and `check-assertion-strength` match their marker with the
colon and read no reason, `check-gate-fail-closed` matches a bare
`fail-closed-exempt` without even requiring the colon, and there is no shared
marker parser in `lib/gate.sh`. The tightening is worth its inconsistency here
because this valve stands a job outside a *release-path* assertion, where an
unexplained exemption is the failure mode the gate exists for; whether the
siblings should follow is not this section's question.

**The walk is this gate's own, and §check-action-run-shell's extractor is not
reusable here.** That extractor has no theory of `jobs:` at all: it partitions by
*file*, and what it emits carries no job identity and none of the `uses:` or
`env:` lines the arms read. This gate needs a job-partitioned walk carrying all
three, so it is **not** the second consumer that section's standing
rule waits for: its "a helper earns its place at a second consumer and there is
none", and the matching `# spec:` comment at the extractor itself, stay true.
Stated because the opposite reading is the natural one, and because
§lib/declaration.sh met the *same* standing rule with the opposite answer — only
the difference in what is being extracted decides it.

Tier `precommit`; the `# graph:` couples the surfaces §check-action-pinning
couples, `dir=one` — a one-way audit. A tree holding no YAML, or no job invoking
`gh`, exits clean on a zero count, the counted inertness that makes this kit
mechanism rather than a consumer gate. The `bad/` fixture opens on the attested
miss itself — the `v0.17.0` release job as it shipped — and carries further
reject jobs for the arms that one does not exercise, so §When a gate earns its
place's demand that a higher-false-positive
gate wait for a real miss is met by a fixture that *is* the miss rather than an
invention of one.

### check-commit-msg

Invariant: the prospective commit message (the `commit-msg` hook's `$1`) matches
no banned pattern. This is the message half of the CLAUDE.md ban on leaked
local paths / private repo/project/account terms — the surface the `pre-commit`
hook never sees, since the message does not exist until commit time. Enforcement
is a generated `commit-msg` hook (`tier=commit-msg`), which rejects the message
before the commit exists rather than a history scan finding the leak after push,
when the only remedy is a destructive rewrite. Patterns come from
`gate_msg_pattern_files` (lib/gate.sh): explicit positional pattern-file args
win; otherwise `GATE_SDK_MSG_PATTERN_FILES` (tracked, must exist — fail-closed)
plus `GATE_SDK_MSG_PATTERN_FILES_LOCAL` (gitignored, skipped when absent, so a
fresh clone without the operator's private list still commits). Each file is
`grep -E` one-pattern-per-line, `#` comments and blanks ignored; the kit ships
`templates/msg-patterns.list` with the generic defaults (absolute home paths,
the Claude Code promo URL — the marketing link that ends a generated PR body,
never the `Co-Authored-By` trailer — and the session-reference class: the
session-share URL host and a trailer-shaped `Key: ` line carrying a full UUID,
the shape of a harness-injected session-id trailer. Both are generic mechanism —
a public host and a shape, no private term — so they ship tracked, not local. The
UUID half is anchored to a trailer-shaped line so the shared pattern set (also
read by check-tree-terms §check-tree-terms) matches an injected footer, never the
synthetic session UUIDs the tracked `smoke/` trees legitimately carry). A
no-argument run (the whole-tree battery)
is a clean skip: the message is not a tracked surface and the history-scan
backstop is deferred to the hosted-attestation rung. A missing message-file
argument-with-value, or a missing required tracked pattern file, is fail-closed
(exit 2). The `# graph:` couples the pattern file (the regeneration trigger),
not a tree path — the gate is emitted into the commit-msg hook, not the
pre-commit hook. Subject *shape* is the sibling check-commit-subject's job:
this gate stays the leak guard, that one the parse guarantee.

### check-commit-subject

Invariant: the prospective commit message's subject line (the `commit-msg`
hook's `$1`, first line) parses as `<type>(<scope>)?!?: <summary>` with
`<type>` drawn from the shared roster and `<scope>` a `[a-z0-9./-]+` token, or
matches a git-generated carve-out — `Merge `, `Revert ` and the `fixup! ` /
`squash! ` autosquash forms. A subject that does not parse is an unread write
to a governed projection, not a style nit: trajectory.sh's feat/debt column
classifies commit subjects, and the closed-row freeze leans on docs/chore
filings sitting outside that harvest — both properties held by convention
until this gate made every subject carry a roster token, turning a mistyped
prefix into a blocked commit rather than a silently drifted evidence row. It
rides the generated `commit-msg` hook (`tier=commit-msg`) beside
check-commit-msg, each an independent assertion with its own fixtures: the leak
guard checks banned patterns, this one subject shape.

The roster is `gate_commit_types` (lib/gate.sh), reading `GATE_SDK_COMMIT_TYPES`
(default `feat fix refactor perf docs test build ci chore style`). The
one-vocabulary/two-readers tension — this gate and the evidence classifiers
both key off commit types — is ruled *share the roster, keep the mappings*: the
roster's single home is lib/gate.sh; drift-kit's kpi-task-split and
trajectory.sh keep their own class mapping (feat vs fix+refactor), a
classification over roster tokens rather than a second roster. Edge behavior
matches check-commit-msg: a no-argument run (the whole-tree battery) is a clean
skip — the message is not a tracked surface; a missing message-file
argument-with-value is fail-closed (exit 2). The `# graph:` couples the
roster's config home (lib/gate.sh, the regeneration trigger), not a tree path —
the gate is emitted into the commit-msg hook.

### check-tree-terms

Invariant: no tracked file matches the same banned-pattern set — the
tracked-files half of the leak ban, sharing one pattern source with
check-commit-msg (`gate_msg_pattern_files`) so the two halves cannot drift
apart. It runs over `git ls-files` (scan root the optional first argument,
default `.`; pattern-file overrides follow), at `tier=precommit` with
`trigger=*` so it fires on every commit — any tracked file can carry a leak,
not only an edited pattern list. Two skips keep it from flagging its own
scaffolding: the shared prune dirs (so a fixture tree under `gate-tests/` is out
of scope), and any file whose basename begins `msg-patterns` — a pattern list or
its template contains, by construction, what it bans, and tracking a private
list is caught by keeping it gitignored, not by this gate. A non-repo cwd, or a
missing required tracked pattern file, is fail-closed (exit 2). When the pattern
set is empty the tree is unchecked (clean) — the fail-closed obligation is on a
missing file, not an empty one.

### check-template-copy-parity

Invariant: a kit template and its vendored consumer copy agree on their
**declared contract surface**. No mechanism kept the two in step before this
gate — `scripts/agent-budget-guard.sh` and its template were both hand-edited
with nothing verifying the edits matched.

**Scope derives from layout, never a roster.** The pairing is
`<kit>/templates/<name>.sh` ↔ `<gates-dir>/<name>.sh` (scan root the optional
first argument, default the git toplevel). Two exclusions, both derivable:
`*-config.sh` is out of scope by name suffix — a config template is a starting
point the consumer customizes, so equality would be the defect — and a template
with **no** same-named file under the gates dir is silently skipped, not failed.
An unpaired template was never vendored out and has no copy to be in parity
with; running a template in place (this repo wires two from the template path
itself) is a legitimate adoption mode, so failing closed there would red a tree
for six files that are working as designed. The same principle applies one axis
over in `check-template-registry-parity` (§check-template-registry-parity),
which derives *its* population from layout too — so a reader arriving at either
meets both. They do not overlap: this gate compares a template to a **consumer
copy** of itself, that one compares a template to the **kit directory it
registers**.

**Byte parity is ruled out, on measurement**, and so is containment in either
direction. Every executable pair diverges deliberately: the consumer copies add
this repo's own steering content. *copy ⊆ template* is therefore false on
purpose; *template ⊆ copy* catches only the reverse of the direction that rots.
Both alternatives are recorded here so no later pass re-derives them.

**The declared surface** is four classes of *declaration*, namespaced so a
collision across classes cannot mask one:

- `func:` — function declarations, both `name()` and `function name` spellings.
- `case:` — each `case`-arm's **exit token**: the first command word of the arm
  body.
- `lib:` — command-position calls to `_`-bearing identifiers the file does not
  itself declare (the sourced-lib API).
- `knob:` — uppercase `_`-bearing names read via the defaulted-env idiom
  `${NAME:-…}` / `${NAME:=…}`.

The `case:` class reads the arm's **action, never its pattern**, and that is a
seam requirement rather than a convenience: a consumer's arm patterns are its own
rule vocabulary, which a kit gate must never read (CLAUDE.md §The provenance
seam). The gate asserts that a consumer's divergent rule lines are *declared*,
never what they say. `func:` is inert on every pair in this tree (thin hook
scripts source a kit lib and delegate, declaring no functions); the surface's
whole bite comes from the other three classes, which are non-empty on every file.

**The three assertions.**

- **A — same resolved `spec:` target.** Both copies' first `spec:` line resolves
  to the same `<file> §<section>`, compared on the target with trailing prose
  stripped: a pair may gloss one target two ways deliberately, and comparing
  whole lines would red a sanctioned divergence. A inherits a second property —
  `check-spec-pointer` skips templates by design (§Self-lint), so a template's
  target has never been resolved by anything, and A is the only mechanism that
  surfaces a dangling template pointer.
- **B — the template's declared surface is present in the copy.** Catches a
  template-side change never propagated *and* a copy-side **removal**. B is the
  only arm that sees a removal, which is why it is not droppable: the
  `session-context` pair is a mutual rewrite, not a pure addition.
- **C — the copy declares what it adds.** A surface token in the copy but not the
  template is drift unless a `# copy-divergence: <reason>` line in that copy
  **names** the token. Naming, not blanket per-file coverage: a blanket marker
  would let C fire once and never again.

**The `# copy-divergence: <reason>` marker** is source, like the `# graph:` and
`# spec:` headers it sits beside — hand-authored by whoever adds something the
template does not declare, read by assertion C, and a directive rather than a
restatement under `check-comment-tier` (it changes gate behavior). The reason is
required and non-empty, the `[drain-exempt:]` precedent where the reason *is* the
audit trail.

**Why this is the low-false-positive shape:** the gate reads declarations, not
prose or logic, so a consumer rewording a message, reordering rules, or adding
steering text triggers nothing. It fires only when a declared contract element
appears on one side unexplained.

`dir=bi` — parity is symmetric, either side going stale is the defect, which is
precisely what B and C split between them. The fixture pair **synthesizes** the
one-sided edit rather than capturing one: the tree's own attesting divergence is
already repaired, so `good/` proves green-on-a-sanctioned-`spec:`-prose-difference
(alongside an unpaired template and a divergent `*-config.sh` pair, proving both
scope exclusions) and `bad/` proves all three assertions red on a hand-edited
copy. Tier `precommit`.

### check-template-registry-parity

Invariant: a kit's shipped `.list` registry template names exactly the artifacts
of the sibling directory it registers. Two assertions, per registry: (A) every
shipped artifact is registered — a bundled plugin absent from the template is
red, the finding this gate landed on; and (B) every registry line resolves to a
shipped artifact — a line naming no file is red, so a retired plugin cannot
leave a line that installs a broken registry into every consumer. Each finding
names the kit, the template, and the member.

**A kit template that registers artifacts the kit itself ships names every one
of them.** The consumer's *copy* is the consumer's to prune — drift-kit's
`templates/kpis.list` header already says so — but the shipped template is the
kit's claim about what it bundles, and the kit's SPEC roster, its README, and
its smoke's coverage assertions are all stated over that set. A deliberate
starter subset is refused because nothing distinguishes it from an omission: it
is exactly what a dropped line looks like, which is how drift-kit's missing
`kpi-queue-net-delta` line survived a shipped registry, a smoke that copies it,
and a SPEC that named the KPI.

**Scope derives from layout, never a roster** — §check-template-copy-parity's
rule, applied one axis over. A template enters the population when
`<kit>/templates/<name>.list` has a sibling **directory** `<kit>/<name>/`
holding the artifacts the list registers; a `.list` with no such sibling is
skipped-and-counted, not failed, the same silent-skip exclusion its sibling gate
gives an unpaired template. Shipped members are the basenames of that
directory's `*.sh` files, extension stripped, read from `git ls-files` so an
untracked scratch file forces no registry line; registry members are the
non-comment, non-blank lines — the `gates.list` grammar `gates_list_members`
reads, the same grammar `drift-kit/bin/drift-report.sh` resolves its registry
through, so the gate calls a name registered exactly when the consumer's
resolver would.

**The population predicate carries the provenance seam, and that is why it is
structural.** The flat form of the rule — *a kit's `templates/` registry must be
the full bundled set* — is unsafe as stated: applied to
`drift-kit/templates/price-table.tsv` it would force a kit literal enumerating a
model roster, which is precisely the seam (CLAUDE.md §The provenance seam). Two
kinds of template are out of population, both by construction and in this order:
one that is not a `.list` at all never reaches the sibling test (`price-table.tsv`
exits there, on its extension); one that is a `.list` whose rows are consumer
rule content the kit stubs generically — `gate-sdk/templates/msg-patterns.list`,
whose rows are `grep -E` patterns — has no sibling directory of kit artifacts,
because having no kit-shipped artifacts to register *is* the same fact as having
no sibling directory of them. This is deliberately **not a per-file exception
list**, and that is the whole point of the shape: an exception list re-arms on
the next template of this kind, since someone must remember to extend it, and
the failure mode of forgetting is a kit literal publishing a private vocabulary.

A second structural consequence, stated so it is not read as an omission:
`drift-kit/templates/kpi-deprecated-surface.sh` is an example plugin shipped *as
a template* for a consumer to adapt. It is not in `drift-kit/kpis/`, so it is
not a bundled artifact and is not required in the registry — the shipped set is
the sibling directory's contents and nothing else.

Sweep: kit roots come from `gate_kit_roots` (the `GATE_SDK_KIT_DIRS` knob —
§Layout and configuration), the sibling roster meta-gates' shape; config adds
**no new knob**. Positional form `check-template-registry-parity.sh [root]`
resolves relative kit roots against a fixture tree (the case dir's
`gate-sdk-config.sh` names the fixture kits); bare, it sweeps against the git
toplevel. Fail-closed: a non-repo cwd with no root argument, an empty kit
roster, an unreadable template or sibling directory, or a failed `git ls-files`
is exit 2, never a false clean. `dir=bi` — either side going stale is the
defect, which is what (A) and (B) split between them — tier `precommit`. The
fixture pair synthesizes both sides at once: `good/` proves green on a registry
in parity while carrying both structural exclusions (a `.list` with no sibling,
and a non-`.list` template), and `bad/` proves a registry that is one-sided in
each direction at the same time.

### templates/check-skeleton.sh

The copy-paste reference skeleton — the canonical "how to write a gate"
template (structure + fail-closed + output contract). A new gate is a
copy-edit of it, shipping with its fixture pair. It is a template, never a
registry member; structure is copied, not imported, so it stays per-gate and
legible.

### templates/gates-workflow.yml

The CI backstop template — the server-side outer tier of §Enforcement tiers,
copied out to a consumer's `.github/workflows/gates.yml`. It closes the two
gaps the local hook cannot by construction: a `--no-verify` commit, and a clone
that never ran `install-hooks.sh`. Trigger is push + pull_request on the
consumer's default branch (a fill-in — the template ships `main` and says so);
step one is `run-gates.sh` (the full battery); step two is a fail-closed
placeholder the consumer replaces with its own fixture/guard-test runners (an
unfilled copy reddens CI, the fill-me signal). Checkout + bash only — no
caching, no matrix, no third-party actions — so the workflow surface an agent
could tamper with stays minimal and reviewable. The one action that survives that
rule is pinned to a full commit SHA with the tag kept as a trailing comment, and
§check-action-pinning holds it there: ruling *which* actions may appear leaves
open *which code* a named tag resolves to, and a tag its owner can repoint is the
same tamper axis one hop further out.

The pin buys immutability, not currency — and the two are easy to confuse. No
update bot refreshes a pinned SHA (this repo deliberately runs none), so a pin
stays exactly as old as the day it was written; refreshing one is a manual act at
release time. A pinned ref is therefore not a maintained ref, and a consumer who
reads it as one has the wrong picture of what the pin is worth. This repo's own
`.github/workflows/gates.yml` is the filled example; both files register in
`core-files.list` so their silent deletion is red, and the instance needs no
freshness gate — a workflow invoking a retired script reddens in CI on its next
run, the drift signal working as designed.

Scope boundary, stated in the template header rather than overclaimed:
consumer-owned CI stops *bypass*, but cannot stop an agent editing the workflow
itself in the same change. A tamper-proof verifier (verifier neutrality) is the
deferred hosted-attestation-service rung. CI is not a smoke surface — it runs
the real battery, so it is not installed by any kit's `smoke/` (§Consumer
smoke); the branch-protection recipe that makes the check a required merge gate
is a GitHub setting, so it lands as install-page docs, not committable
mechanism.
